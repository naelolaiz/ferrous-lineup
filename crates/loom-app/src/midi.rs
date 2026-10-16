//! MIDI keyboards and synths, through the `midir` crate.
//!
//! midir talks to each system's own MIDI layer: ALSA on Linux, Core MIDI
//! on macOS and Windows Multimedia on Windows. It gives the same few types
//! everywhere: `MidiInput` and `MidiOutput` to list ports, and a
//! connection to one port that stays open for as long as it is kept.
//!
//! Three threads share the work, and none of them waits for another:
//!
//! - midir runs a thread of its own that calls [`on_message`] for every
//!   message from the keyboard. Notes go straight to the audio thread
//!   through a [`LiveInput`], and a copy goes to the UI, which shows the
//!   keys held down and records them.
//! - The audio thread plays the notes, and MIDI Out channels turn notes
//!   into messages, which it leaves in a [`MidiOutput`] queue.
//! - A sending thread, started here, empties that queue into the output
//!   port. Sending can take a while (it is a system call), so the audio
//!   thread must not do it.

use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::Duration;

use eframe::egui;
use loom_audio::{LiveInput, MidiOutput};
use loom_core::MidiMessage;
use loom_core::midi::all_notes_off;
use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutputConnection};

/// The name Ferrous Loom shows to other MIDI programs.
const CLIENT_NAME: &str = "Ferrous Loom";

/// How long the sending thread sleeps before it looks for messages again.
/// The queue has no way to wake a thread that waits on it, so the thread
/// looks often: a thousand times a second costs almost nothing.
const SEND_INTERVAL: Duration = Duration::from_millis(1);

/// A key that went down or up, and where playback was at that moment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Played {
    pub message: MidiMessage,
    /// In steps of the loop (see `Sequencer::position`), or `None` when
    /// stopped.
    pub position: Option<f64>,
}

/// A MIDI port that can be picked from the menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    /// What midir finds the port by. It stays the same while the device
    /// is plugged in, even when other devices come and go.
    id: String,
    pub name: String,
}

/// Locks a mutex, even if another thread panicked while holding it.
///
/// A panic while a lock is held "poisons" it, and `lock` then returns an
/// error, because the data inside may be half changed. The data here (a
/// queue end, a connection) is never left half changed, so it is safe to
/// carry on with it.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Everything the input callback needs. It is moved to midir's thread
/// when a port is connected.
struct InputLink {
    /// The way to the audio thread, if audio is running. It sits behind a
    /// lock because it must survive a connection: midir does not give the
    /// data back if connecting fails. Only one connection exists at a time,
    /// so the lock is never waited on.
    live: Option<Arc<Mutex<LiveInput>>>,
    to_ui: mpsc::Sender<Played>,
    ctx: egui::Context,
}

/// Called by midir, on its own thread, for every message from the keyboard.
fn on_message(_microseconds: u64, bytes: &[u8], link: &mut InputLink) {
    let Some(message) = MidiMessage::parse(bytes) else {
        return;
    };
    let mut position = None;
    if let Some(live) = &link.live {
        let mut live = lock(live);
        live.send(message);
        position = live.position();
    }
    // `send` fails only if the UI is gone, and then nobody needs the copy.
    let _ = link.to_ui.send(Played { message, position });
    // egui only redraws when something happens. This is something, and
    // `request_repaint` is one of the few egui calls meant for other threads.
    link.ctx.request_repaint();
}

/// The MIDI ports, the open connections, and the keys that came in.
pub struct Midi {
    /// Why MIDI cannot be used, if it cannot (no MIDI system, say).
    unavailable: Option<String>,
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    input: Option<(Port, MidiInputConnection<InputLink>)>,
    output_port: Option<Port>,
    /// The output connection, shared with the sending thread.
    output: Arc<Mutex<Option<MidiOutputConnection>>>,
    live: Option<Arc<Mutex<LiveInput>>>,
    to_ui: mpsc::Sender<Played>,
    from_keyboard: mpsc::Receiver<Played>,
    ctx: egui::Context,
}

impl Midi {
    /// Sets up MIDI. `live` and `messages` are the engine's ways in and
    /// out, if audio is running.
    pub fn new(ctx: &egui::Context, live: Option<LiveInput>, messages: Option<MidiOutput>) -> Self {
        let (to_ui, from_keyboard) = mpsc::channel();
        let output = Arc::new(Mutex::new(None));
        // Opening a client is the test: it fails when the system has no MIDI.
        let unavailable = MidiInput::new(CLIENT_NAME).err().map(|err| err.to_string());
        if unavailable.is_none()
            && let Some(messages) = messages
        {
            start_sending(messages, Arc::clone(&output));
        }
        let mut midi = Midi {
            unavailable,
            inputs: Vec::new(),
            outputs: Vec::new(),
            input: None,
            output_port: None,
            output,
            live: live.map(|live| Arc::new(Mutex::new(live))),
            to_ui,
            from_keyboard,
            ctx: ctx.clone(),
        };
        midi.refresh_ports();
        midi
    }

    /// Why MIDI cannot be used, or `None` if it can.
    pub fn unavailable(&self) -> Option<&str> {
        self.unavailable.as_deref()
    }

    /// Looks for ports again: a keyboard may have been plugged in.
    pub fn refresh_ports(&mut self) {
        if self.unavailable.is_some() {
            return;
        }
        if let Ok(midi_in) = MidiInput::new(CLIENT_NAME) {
            self.inputs = midi_in
                .ports()
                .iter()
                .filter_map(|port| {
                    let name = midi_in.port_name(port).ok()?;
                    Some(Port {
                        id: port.id(),
                        name,
                    })
                })
                .collect();
        }
        if let Ok(midi_out) = midir::MidiOutput::new(CLIENT_NAME) {
            self.outputs = midi_out
                .ports()
                .iter()
                .filter_map(|port| {
                    let name = midi_out.port_name(port).ok()?;
                    Some(Port {
                        id: port.id(),
                        name,
                    })
                })
                .collect();
        }
    }

    pub fn inputs(&self) -> &[Port] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[Port] {
        &self.outputs
    }

    /// The input port that is connected, if any.
    pub fn input(&self) -> Option<&Port> {
        self.input.as_ref().map(|(port, _)| port)
    }

    /// The output port that is connected, if any.
    pub fn output(&self) -> Option<&Port> {
        self.output_port.as_ref()
    }

    /// Listens to `port` instead of the port before, or to nothing if `None`.
    pub fn connect_input(&mut self, port: Option<&Port>) -> Result<(), String> {
        // Closing gives back the `MidiInput` and the link, which are no
        // longer needed: the link only holds copies.
        if let Some((_, connection)) = self.input.take() {
            drop(connection.close());
        }
        let Some(port) = port else {
            return Ok(());
        };
        let mut midi_in = MidiInput::new(CLIENT_NAME).map_err(|err| err.to_string())?;
        // Clock ticks, "I am still here" messages and system exclusive
        // data are not notes. Leaving them out saves waking up for them.
        midi_in.ignore(Ignore::All);
        let found = midi_in
            .find_port_by_id(&port.id)
            .ok_or_else(|| format!("{} is not connected any more", port.name))?;
        let link = InputLink {
            live: self.live.clone(),
            to_ui: self.to_ui.clone(),
            ctx: self.ctx.clone(),
        };
        let connection = midi_in
            .connect(&found, "input", on_message, link)
            .map_err(|err| err.to_string())?;
        self.input = Some((port.clone(), connection));
        Ok(())
    }

    /// Sends MIDI Out channels to `port` instead of the port before, or to
    /// nowhere if `None`.
    pub fn connect_output(&mut self, port: Option<&Port>) -> Result<(), String> {
        let new = match port {
            None => None,
            Some(port) => {
                let midi_out =
                    midir::MidiOutput::new(CLIENT_NAME).map_err(|err| err.to_string())?;
                let found = midi_out
                    .find_port_by_id(&port.id)
                    .ok_or_else(|| format!("{} is not connected any more", port.name))?;
                Some(
                    midi_out
                        .connect(&found, "output")
                        .map_err(|err| err.to_string())?,
                )
            }
        };
        // Swap the connections while holding the lock, so the sending
        // thread never sends half to one and half to the other.
        let old = std::mem::replace(&mut *lock(&self.output), new);
        if let Some(mut old) = old {
            // Notes that were playing there would hang: tell them all to stop.
            for channel in 0..16 {
                let _ = old.send(&all_notes_off(channel));
            }
            // `close` gives back the `MidiOutput`, which is not needed.
            old.close();
        }
        self.output_port = port.cloned();
        Ok(())
    }

    /// The keys that came in from the MIDI keyboard since the last call.
    pub fn played(&self) -> Vec<Played> {
        self.from_keyboard.try_iter().collect()
    }
}

/// Starts the thread that sends MIDI Out messages to the output port.
fn start_sending(mut messages: MidiOutput, output: Arc<Mutex<Option<MidiOutputConnection>>>) {
    let sender = move || {
        while !messages.is_abandoned() {
            thread::sleep(SEND_INTERVAL);
            let mut output = lock(&output);
            while let Some(message) = messages.pop() {
                // With no port connected, the messages are dropped. A send
                // that fails (a cable pulled out) loses only that message.
                if let Some(connection) = output.as_mut() {
                    let _ = connection.send(&message.bytes());
                }
            }
        }
    };
    // If the thread cannot start, MIDI Out stays silent; nothing else breaks.
    let _ = thread::Builder::new()
        .name("midi-out".to_string())
        .spawn(sender);
}
