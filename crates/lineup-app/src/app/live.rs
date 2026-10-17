//! Playing live: the MIDI menu, the typing keyboard, the keys held down,
//! and recording them.
//!
//! Keys come from two places. A MIDI keyboard's come through [`Midi`],
//! already played by the audio thread on their way here. The typing
//! keyboard's are read from egui's key events, and sent to the audio
//! thread as [`Command::Live`]. From then on both are handled alike, by
//! [`LineupApp::key_played`].

use eframe::egui::{self, Key};
use lineup_audio::{Command, PlayMode};
use lineup_core::harmony::analyze;
use lineup_core::record::recorded_note;
use lineup_core::{DEFAULT_VELOCITY, MidiMessage, Note, Pattern, Scale, tools};

use super::follow::Follow;
use super::{Audio, LineupApp};
use crate::midi::{self, Midi, Played, Port, Route};
use crate::theme;

/// The typing keyboard, laid out like a piano on the letter keys, as in
/// other studios. Z X C V B N M are the white keys from C4, and S D G H J
/// in the row above are the black keys between them. Q W E R T Y U I O P
/// are the white keys from C5, with the number keys above them as the
/// black keys.
const TYPING_KEYS: &[(Key, u8)] = &[
    (Key::Z, 60),
    (Key::S, 61),
    (Key::X, 62),
    (Key::D, 63),
    (Key::C, 64),
    (Key::V, 65),
    (Key::G, 66),
    (Key::B, 67),
    (Key::H, 68),
    (Key::N, 69),
    (Key::J, 70),
    (Key::M, 71),
    (Key::Q, 72),
    (Key::Num2, 73),
    (Key::W, 74),
    (Key::Num3, 75),
    (Key::E, 76),
    (Key::R, 77),
    (Key::Num5, 78),
    (Key::T, 79),
    (Key::Num6, 80),
    (Key::Y, 81),
    (Key::Num7, 82),
    (Key::U, 83),
    (Key::I, 84),
    (Key::Num9, 85),
    (Key::O, 86),
    (Key::Num0, 87),
    (Key::P, 88),
];

/// The note a key of the typing keyboard plays, if it plays one.
fn typing_note(key: Key) -> Option<Note> {
    TYPING_KEYS
        .iter()
        .find(|(typing_key, _)| *typing_key == key)
        .map(|&(_, midi)| Note::new(midi))
}

/// A key that is down.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Held {
    pitch: Note,
    velocity: u8,
    /// The channel it plays, and will be recorded into.
    channel: usize,
    /// Where playback was when it went down, if it is being recorded.
    recorded_from: Option<f64>,
}

/// What the app remembers about playing live.
pub(super) struct Live {
    pub(super) midi: Midi,
    /// Whether the letter keys play notes.
    typing: bool,
    /// The typing keys that are down, and the note each one plays.
    typing_down: Vec<(Key, Note)>,
    /// Every key that is down, from either keyboard, in the order they
    /// went down.
    held: Vec<Held>,
    /// Whether keys are recorded while the pattern plays.
    recording: bool,
    /// The live channel last sent to the audio thread.
    channel_sent: Option<usize>,
    /// Whether the MIDI menu was open last frame, to look for new ports
    /// when it opens.
    menu_was_open: bool,
    /// Chord follow: the accompaniment plays the chord the keys hold.
    pub(super) follow: Follow,
    /// Whether every channel plays on the MIDI output, not only the MIDI
    /// Out channels.
    send_all: bool,
    /// The route each channel of the audio thread was last given, or
    /// `None` when that is not known (some channels changed) and they all
    /// have to be sent again.
    routes_sent: Option<Vec<Option<Route>>>,
}

impl Live {
    pub(super) fn new(midi: Midi) -> Self {
        Live {
            midi,
            typing: false,
            typing_down: Vec::new(),
            held: Vec::new(),
            recording: false,
            channel_sent: None,
            menu_was_open: false,
            follow: Follow::default(),
            send_all: true,
            routes_sent: Some(Vec::new()),
        }
    }

    /// Whether the letter and number keys play notes.
    pub(super) fn typing(&self) -> bool {
        self.typing
    }

    /// The pitches held down right now.
    pub(super) fn held_pitches(&self) -> Vec<Note> {
        self.held.iter().map(|held| held.pitch).collect()
    }

    /// Forgets which channel the audio thread plays, so it is sent again:
    /// after the audio thread was cleared, for a new project.
    pub(super) fn resend_channel(&mut self) {
        self.channel_sent = None;
    }

    /// Notes that the audio thread's channels changed: they were all made
    /// again, without routes, or some moved. `channels` is how many there
    /// are now.
    pub(super) fn channels_changed(&mut self, channels: usize) {
        let any_routed = match &self.routes_sent {
            Some(routes) => routes.iter().any(Option::is_some),
            None => true,
        };
        // Channels with no route need nothing sent again: new ones come
        // without one.
        self.routes_sent = if any_routed {
            None
        } else {
            Some(vec![None; channels])
        };
    }
}

impl LineupApp {
    /// Reads both keyboards, once per frame.
    pub(super) fn live_input(&mut self, ctx: &egui::Context) {
        // Keys play the channel the piano roll shows.
        let channel = self.piano_roll.channel();
        if self.live.channel_sent != Some(channel) {
            self.send(Command::SetLiveChannel(channel));
            self.live.channel_sent = Some(channel);
        }
        self.live.midi.snap_to(self.snap_scale());
        self.send_routes();
        self.typing_keyboard(ctx);
        for played in self.live.midi.played() {
            self.key_played(played);
        }
        self.follow_chords();
    }

    /// The key that notes played on the keyboards snap to, if the piano
    /// roll's "Snap notes you play" is on: the key where you are, after
    /// any key change.
    fn snap_scale(&self) -> Option<Scale> {
        self.piano_roll.snap_played.then(|| self.key_here())
    }

    /// Where playback is, in steps with a fraction, or `None` when stopped.
    fn position(&self) -> Option<f64> {
        match &self.audio {
            Audio::Running(engine) if self.playing => engine.position(),
            _ => None,
        }
    }

    /// Whether keys played now are recorded.
    pub(super) fn is_recording(&self) -> bool {
        self.live.recording && self.playing && self.mode == PlayMode::Pattern
    }

    /// Reads the letter keys while the typing keyboard is on.
    fn typing_keyboard(&mut self, ctx: &egui::Context) {
        // A key that is let go while the window is not listening (another
        // window has the keyboard, or a text field does) never sends its
        // "up" here. So all keys are let go as soon as that happens, or
        // their notes would play for ever.
        let focused = ctx.input(|input| input.focused);
        if !self.live.typing || !focused || ctx.text_edit_focused() {
            self.release_typing_keys();
            return;
        }
        let keys: Vec<(Key, bool)> = ctx.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Key {
                        key,
                        physical_key,
                        pressed,
                        repeat: false,
                        modifiers,
                    } => {
                        // The *physical* key, where it is on the keyboard,
                        // so the piano keeps its shape on any layout:
                        // on a French keyboard, W is where Z is in English.
                        let key = physical_key.unwrap_or(*key);
                        // Ctrl+Z is undo, not a note. Key releases always
                        // count, or a note could stay on.
                        let shortcut = modifiers.command || modifiers.alt;
                        (!*pressed || !shortcut).then_some((key, *pressed))
                    }
                    _ => None,
                })
                .collect()
        });
        for (key, pressed) in keys {
            let Some(key_note) = typing_note(key) else {
                continue;
            };
            let pitch = match self.snap_scale() {
                Some(scale) => tools::nearest_in_key(key_note, scale),
                None => key_note,
            };
            let down = self.live.typing_down.iter().position(|&(k, _)| k == key);
            match (pressed, down) {
                (true, None) => {
                    self.live.typing_down.push((key, pitch));
                    self.play_typed(MidiMessage::NoteOn {
                        channel: 0,
                        pitch,
                        velocity: DEFAULT_VELOCITY,
                    });
                }
                (false, Some(index)) => {
                    // The note it started, which may not be `pitch` if
                    // the key changed while it was held.
                    let (_, pitch) = self.live.typing_down.remove(index);
                    self.play_typed(MidiMessage::NoteOff { channel: 0, pitch });
                }
                _ => {}
            }
        }
    }

    /// Plays a key of the typing keyboard, or a note of a harmony key.
    pub(super) fn play_typed(&mut self, message: MidiMessage) {
        self.send(Command::Live(message));
        let position = self.position();
        self.key_played(Played { message, position });
    }

    fn release_typing_keys(&mut self) {
        // `take` leaves an empty list behind, so the loop can call `&mut self` methods.
        for (_, pitch) in std::mem::take(&mut self.live.typing_down) {
            self.play_typed(MidiMessage::NoteOff { channel: 0, pitch });
        }
    }

    /// Lets go of every key, as when the MIDI keyboard is disconnected.
    fn release_all_keys(&mut self) {
        self.live.typing_down.clear();
        for held in std::mem::take(&mut self.live.held) {
            self.send(Command::Live(MidiMessage::NoteOff {
                channel: 0,
                pitch: held.pitch,
            }));
        }
    }

    /// Keeps track of the keys that are down, and records them.
    fn key_played(&mut self, played: Played) {
        match played.message {
            MidiMessage::NoteOn {
                pitch, velocity, ..
            } => {
                self.live.held.retain(|held| held.pitch != pitch);
                self.live.follow.key_down();
                let recorded_from = played.position.filter(|_| self.is_recording());
                self.live.held.push(Held {
                    pitch,
                    velocity,
                    channel: self.piano_roll.channel(),
                    recorded_from,
                });
            }
            MidiMessage::NoteOff { pitch, .. } => {
                let Some(index) = self.live.held.iter().position(|held| held.pitch == pitch) else {
                    return;
                };
                let held = self.live.held.remove(index);
                if let (Some(from), Some(until)) = (held.recorded_from, played.position) {
                    self.record(held, from, until);
                }
            }
            MidiMessage::ProgramChange { .. } => {}
        }
    }

    /// Adds a key that was held from position `from` until `until` to the
    /// selected pattern, with both ends moved to the nearest step.
    fn record(&mut self, held: Held, from: f64, until: f64) {
        let Some(length) = self.project.pattern(self.pattern).map(Pattern::length) else {
            return;
        };
        let note = recorded_note(held.pitch, held.velocity, from, until, length);
        self.edit_pattern("record notes", |pattern| {
            pattern.add_note(held.channel, note);
        });
    }

    /// Records the keys still down, as if they came up now. Called before
    /// playback stops, so the take ends where the music did.
    pub(super) fn finish_recording(&mut self) {
        let until = self.position();
        for index in 0..self.live.held.len() {
            let held = self.live.held[index];
            if let (Some(from), Some(until)) = (held.recorded_from, until) {
                self.record(held, from, until);
            }
            self.live.held[index].recorded_from = None;
        }
    }

    /// Arms or disarms recording. Recording works in pattern mode, so
    /// arming it switches there.
    pub(super) fn toggle_recording(&mut self) {
        if self.live.recording {
            self.finish_recording();
            self.live.recording = false;
        } else {
            self.live.recording = true;
            self.set_mode(PlayMode::Pattern);
        }
    }

    /// The Rec button, next to Play. It turns red while recording is armed.
    pub(super) fn record_button(&mut self, ui: &mut egui::Ui) {
        let button = theme::light_button(ui, "⏺", self.live.recording, Some(theme::RED));
        let hover = "Record what you play on a MIDI or typing keyboard into the channel the \
                     piano roll shows, while the pattern plays. Switches to pattern mode.";
        if theme::hint(button, hover).clicked() {
            self.toggle_recording();
        }
    }

    /// Gives each channel of the audio thread its route to the MIDI
    /// output, when "Send every channel" is on and a port is connected,
    /// and takes them away again when not. Only the channels whose route
    /// changed get a new instrument.
    fn send_routes(&mut self) {
        let channels = self.project.channels();
        let wanted = if self.live.send_all && self.live.midi.output().is_some() {
            midi::routes(channels)
        } else {
            vec![None; channels.len()]
        };
        if self.live.routes_sent.as_ref() == Some(&wanted) {
            return;
        }
        let sent = self.live.routes_sent.take();
        let sample_rate = self.sample_rate();
        for (channel, route) in wanted.iter().enumerate() {
            if sent.as_ref().and_then(|sent| sent.get(channel)) == Some(route) {
                continue;
            }
            let state = self.project.channels()[channel].clone();
            let mut instrument = self.build_instrument(&state, sample_rate);
            if let Some((midi_channel, pitch)) = *route {
                instrument = instrument.to_midi(midi_channel, pitch, state.program);
            }
            self.send(Command::SetInstrument {
                channel,
                instrument,
            });
        }
        self.live.routes_sent = Some(wanted);
    }

    /// The MIDI menu, next to the Edit menu.
    pub(super) fn midi_menu(&mut self, ui: &mut egui::Ui) {
        let response = ui.menu_button("MIDI", |ui| {
            if !self.live.menu_was_open {
                self.live.midi.refresh_ports();
            }
            match self.live.midi.unavailable() {
                Some(reason) => {
                    ui.colored_label(
                        ui.visuals().warn_fg_color,
                        format!("MIDI unavailable: {reason}"),
                    )
                    .on_hover_text(
                        "On Linux, MIDI needs the ALSA sequencer (the snd-seq kernel module, \
                         which makes /dev/snd/seq).",
                    );
                }
                None => self.port_choices(ui),
            }
            ui.separator();
            ui.checkbox(&mut self.live.typing, "Typing keyboard to piano")
                .on_hover_text(
                    "Play notes with the letter keys: Z X C V B N M from C4, with S D G H J \
                     as the black keys, and Q to P from C5, with the numbers above them.",
                );
            ui.checkbox(&mut self.live.follow.on, "Accompaniment follows chords")
                .on_hover_text(
                    "While the music plays, the chord you hold on a MIDI or typing keyboard \
                     becomes the chord of the accompaniment: the chords and bass line keep \
                     their rhythm and move to your chord from their next note, as on an \
                     arranger keyboard. One key plays its major chord, two keys a minor third \
                     apart its minor chord. The last chord stays until you play another, and \
                     the written notes come back when playback stops. The song itself does \
                     not change.",
                );
        });
        self.live.menu_was_open = response.inner.is_some();
    }

    /// The input and output ports, to pick from.
    fn port_choices(&mut self, ui: &mut egui::Ui) {
        ui.label("Input: a keyboard to play");
        let current = self.live.midi.input().cloned();
        if let Some(choice) = pick_port(ui, self.live.midi.inputs(), current.as_ref()) {
            self.release_all_keys();
            if let Err(err) = self.live.midi.connect_input(choice.as_ref()) {
                self.message = Some(format!("Could not connect: {err}"));
            }
            ui.close();
        }
        ui.separator();
        ui.label("Output: an external synth to play");
        let current = self.live.midi.output().cloned();
        if let Some(choice) = pick_port(ui, self.live.midi.outputs(), current.as_ref()) {
            if let Err(err) = self.live.midi.connect_output(choice.as_ref()) {
                self.message = Some(format!("Could not connect: {err}"));
            }
            ui.close();
        }
        ui.checkbox(&mut self.live.send_all, "Send every channel")
            .on_hover_text(
                "On: every channel plays on the output instead of the speakers, on the MIDI channels File > \
                 Export MIDI uses: drums on channel 10 as a General MIDI drum set, the others \
                 on 1, 2, 3 and so on. Off: only MIDI Out channels play there, as in FL Studio.",
            );
        if !self.live.midi.can_send() {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "Nothing is sent while audio is unavailable",
            );
        }
    }

    /// The keys held down and the chord they make, for the status bar.
    pub(super) fn held_chord(&self) -> Option<String> {
        if self.live.held.is_empty() {
            return None;
        }
        let harmony = analyze(self.live.held_pitches());
        let key = self.key_here();
        let described = harmony.describe(key);
        let text = match self.live.follow.chord() {
            Some(chord) => format!(
                "🎹 {} · accompaniment in {}",
                described.trim_end_matches('.'),
                key.chord_name(chord)
            ),
            None => format!("🎹 {described}"),
        };
        Some(text)
    }
}

/// "None" and one entry per port. Returns the one clicked, if any:
/// `Some(None)` for "None".
fn pick_port(ui: &mut egui::Ui, ports: &[Port], current: Option<&Port>) -> Option<Option<Port>> {
    let mut choice = None;
    if ui.selectable_label(current.is_none(), "None").clicked() {
        choice = Some(None);
    }
    if ports.is_empty() {
        ui.weak("No ports found");
    }
    for port in ports {
        if ui
            .selectable_label(current == Some(port), &port.name)
            .clicked()
        {
            choice = Some(Some(port.clone()));
        }
    }
    choice
}
