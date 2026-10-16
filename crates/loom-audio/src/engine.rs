//! Opens the sound card and runs the audio thread.
//!
//! The audio thread is special: the sound card asks it for a new block of
//! samples every few milliseconds, and if it is late you hear a click or a
//! dropout. So the code that runs there must never wait: no locks, no file
//! or network access, no memory allocation or freeing, no printing.
//!
//! The UI therefore never touches the audio state directly. It sends
//! [`Command`] messages through a lock-free ring buffer (`rtrb`), and the
//! audio thread applies them at the start of each block. Two things travel
//! the other way: the current step (through an atomic integer, for the
//! playhead in the UI) and old instruments and note lists that need
//! freeing (through a second ring buffer, so the memory is freed on the
//! UI thread).

use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig};
use loom_core::{
    DEFAULT_VELOCITY, MAX_CHANNELS, Note, NoteEvent, STEP_NOTE, STEPS_PER_BEAT, Sequencer, Tempo,
};

use crate::dsp::Smoothed;
use crate::instrument::Instrument;

/// How many commands can wait in the queue before `send` starts failing.
/// The audio thread empties the queue every few milliseconds, so this is
/// far more than the UI will ever send in that time.
const COMMAND_QUEUE_SIZE: usize = 1024;

/// How many old instruments and note lists can wait to be freed by the UI thread.
const TRASH_QUEUE_SIZE: usize = 256;

/// The value of the playhead while stopped.
const NO_STEP: usize = usize::MAX;

/// How long volume changes take to settle, in seconds.
const VOLUME_SMOOTHING: f32 = 0.01;

/// A message from the UI thread to the audio thread.
///
/// Channel numbers are positions in the rack, counted from 0, and must
/// match the UI's list exactly: the UI sends `AddChannel` and
/// `RemoveChannel` whenever its own list changes.
#[derive(Debug)]
pub enum Command {
    /// Start the pattern from step 0.
    Play,
    /// Stop playback. Sounds already playing ring out.
    Stop,
    SetTempo(Tempo),
    SetPatternLength(usize),
    SetMasterVolume(f32),
    /// Add a channel at the bottom of the rack.
    AddChannel {
        instrument: Instrument,
        notes: Vec<NoteEvent>,
        volume: f32,
        muted: bool,
    },
    RemoveChannel(usize),
    /// Replace a channel's instrument, for example after loading a sample.
    SetInstrument {
        channel: usize,
        instrument: Instrument,
    },
    /// Replace all the notes of a channel. The UI sends a fresh copy of
    /// the list whenever a step or a note changes.
    SetNotes {
        channel: usize,
        notes: Vec<NoteEvent>,
    },
    SetVolume {
        channel: usize,
        volume: f32,
    },
    SetMuted {
        channel: usize,
        muted: bool,
    },
    /// Play a channel once right now, as when clicking its name.
    Preview(usize),
    /// Play one note on a channel right now, as when clicking a piano key.
    PreviewNote {
        channel: usize,
        pitch: Note,
    },
}

/// Something the audio thread has finished with. It is sent back to the
/// UI thread, because freeing memory is not allowed on the audio thread.
enum Garbage {
    Instrument(Instrument),
    Notes(Vec<NoteEvent>),
}

/// What the engine found when it opened the sound card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineInfo {
    pub device_name: String,
    pub sample_rate: u32,
    pub channels: u16,
}

/// Everything that can go wrong while starting the engine.
#[derive(Debug)]
pub enum EngineError {
    /// The system has no default output device (no sound card, or none enabled).
    NoOutputDevice,
    /// The device would not tell us its preferred settings.
    Config(cpal::Error),
    /// The device uses a sample format this engine does not handle.
    UnsupportedFormat(SampleFormat),
    /// The audio stream could not be created.
    BuildStream(cpal::Error),
    /// The audio stream was created but would not start.
    Play(cpal::Error),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::NoOutputDevice => write!(f, "no audio output device found"),
            EngineError::Config(err) => write!(f, "could not read the device settings: {err}"),
            EngineError::UnsupportedFormat(format) => {
                write!(f, "unsupported sample format: {format}")
            }
            EngineError::BuildStream(err) => write!(f, "could not open the audio stream: {err}"),
            EngineError::Play(err) => write!(f, "could not start the audio stream: {err}"),
        }
    }
}

impl std::error::Error for EngineError {}

/// A running connection to the sound card.
///
/// Sound keeps playing for as long as this value is alive. When it is
/// dropped, the stream is closed and the audio thread stops.
pub struct AudioEngine {
    // The stream is never read, but it must be kept: dropping it stops audio.
    _stream: cpal::Stream,
    commands: rtrb::Producer<Command>,
    trash: rtrb::Consumer<Garbage>,
    playhead: Arc<AtomicUsize>,
    info: EngineInfo,
}

impl AudioEngine {
    /// Opens the default output device and starts the audio thread.
    pub fn start() -> Result<Self, EngineError> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(EngineError::NoOutputDevice)?;
        let supported = device
            .default_output_config()
            .map_err(EngineError::Config)?;

        let sample_format = supported.sample_format();
        let config: StreamConfig = supported.into();
        let info = EngineInfo {
            device_name: device.to_string(),
            sample_rate: config.sample_rate,
            channels: config.channels,
        };

        let (command_producer, command_consumer) = rtrb::RingBuffer::new(COMMAND_QUEUE_SIZE);
        let (trash_producer, trash_consumer) = rtrb::RingBuffer::new(TRASH_QUEUE_SIZE);
        let playhead = Arc::new(AtomicUsize::new(NO_STEP));

        let audio_thread = AudioThread::new(
            config.sample_rate,
            config.channels as usize,
            command_consumer,
            trash_producer,
            Arc::clone(&playhead),
        );

        // The sound card decides the sample type. We pick the matching
        // version of the generic `build_stream` function for each one.
        let stream = match sample_format {
            SampleFormat::F32 => build_stream::<f32>(&device, config, audio_thread),
            SampleFormat::F64 => build_stream::<f64>(&device, config, audio_thread),
            SampleFormat::I16 => build_stream::<i16>(&device, config, audio_thread),
            SampleFormat::I32 => build_stream::<i32>(&device, config, audio_thread),
            SampleFormat::U16 => build_stream::<u16>(&device, config, audio_thread),
            SampleFormat::U8 => build_stream::<u8>(&device, config, audio_thread),
            other => return Err(EngineError::UnsupportedFormat(other)),
        }?;
        stream.play().map_err(EngineError::Play)?;

        Ok(AudioEngine {
            _stream: stream,
            commands: command_producer,
            trash: trash_consumer,
            playhead,
            info,
        })
    }

    /// Sends a command to the audio thread.
    ///
    /// Returns `false` if the queue was full and the command was dropped.
    pub fn send(&mut self, command: Command) -> bool {
        self.commands.push(command).is_ok()
    }

    /// The step being played right now, or `None` when stopped.
    pub fn playhead(&self) -> Option<usize> {
        match self.playhead.load(Ordering::Relaxed) {
            NO_STEP => None,
            step => Some(step),
        }
    }

    /// Frees the instruments and note lists the audio thread has finished
    /// with. Call this regularly from the UI thread, for example once per frame.
    pub fn collect_garbage(&mut self) {
        while let Ok(garbage) = self.trash.pop() {
            // Dropping a value frees its memory. Here, on the UI thread,
            // that is allowed to take as long as it needs.
            match garbage {
                Garbage::Instrument(instrument) => drop(instrument),
                Garbage::Notes(notes) => drop(notes),
            }
        }
    }

    /// Details about the open device.
    pub fn info(&self) -> &EngineInfo {
        &self.info
    }
}

/// Builds an output stream for one sample type `T`.
///
/// `T: SizedSample + FromSample<f32>` means: any sample type cpal supports
/// that can be converted from our `f32` samples.
fn build_stream<T>(
    device: &cpal::Device,
    config: StreamConfig,
    mut audio_thread: AudioThread,
) -> Result<cpal::Stream, EngineError>
where
    T: SizedSample + FromSample<f32>,
{
    // This closure is the audio callback. `move` gives it ownership of
    // `audio_thread`, and cpal runs it on the audio thread.
    let data_callback = move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
        audio_thread.process(output);
    };

    // Called by cpal if something goes wrong while the stream runs, for
    // example when the device is unplugged. This one is allowed to print.
    let error_callback = |err: cpal::Error| eprintln!("audio stream error: {err}");

    device
        .build_output_stream(config, data_callback, error_callback, None)
        .map_err(EngineError::BuildStream)
}

/// One row of the channel rack, as the audio thread sees it.
struct ChannelState {
    instrument: Instrument,
    notes: Vec<NoteEvent>,
    volume: Smoothed,
    muted: bool,
}

/// Everything the audio callback owns.
///
/// Keeping this in a struct (instead of loose variables inside the
/// closure) lets the tests below run it without a sound card.
struct AudioThread {
    sample_rate: u32,
    output_channels: usize,
    /// Reserved with room for `MAX_CHANNELS`, so `push` never allocates.
    channels: Vec<ChannelState>,
    sequencer: Sequencer,
    playing: bool,
    master_volume: Smoothed,
    commands: rtrb::Consumer<Command>,
    trash: rtrb::Producer<Garbage>,
    playhead: Arc<AtomicUsize>,
}

impl AudioThread {
    fn new(
        sample_rate: u32,
        output_channels: usize,
        commands: rtrb::Consumer<Command>,
        trash: rtrb::Producer<Garbage>,
        playhead: Arc<AtomicUsize>,
    ) -> Self {
        AudioThread {
            sample_rate,
            output_channels,
            channels: Vec::with_capacity(MAX_CHANNELS),
            sequencer: Sequencer::new(Tempo::default(), sample_rate, 16),
            playing: false,
            master_volume: Smoothed::new(0.8, VOLUME_SMOOTHING, sample_rate),
            commands,
            trash,
            playhead,
        }
    }

    /// Hands something to the UI thread to be freed there.
    fn throw_away(&mut self, garbage: Garbage) {
        // If the trash queue is full, it is freed here after all. That
        // should never happen, because the UI empties it every frame.
        let _ = self.trash.push(garbage);
    }

    /// How many frames a preview note is held for: one beat.
    fn preview_gate(&self) -> u32 {
        (self.sequencer.step_frames() * STEPS_PER_BEAT as f64) as u32
    }

    fn apply(&mut self, command: Command) {
        match command {
            Command::Play => {
                self.sequencer.reset();
                self.playing = true;
            }
            Command::Stop => {
                self.playing = false;
                self.playhead.store(NO_STEP, Ordering::Relaxed);
                for channel in &mut self.channels {
                    channel.instrument.release_all();
                }
            }
            Command::SetTempo(tempo) => self.sequencer.set_tempo(tempo),
            Command::SetPatternLength(length) => self.sequencer.set_length(length),
            Command::SetMasterVolume(volume) => self.master_volume.set(volume),
            Command::AddChannel {
                instrument,
                notes,
                volume,
                muted,
            } => {
                if self.channels.len() < MAX_CHANNELS {
                    self.channels.push(ChannelState {
                        instrument,
                        notes,
                        volume: Smoothed::new(volume, VOLUME_SMOOTHING, self.sample_rate),
                        muted,
                    });
                } else {
                    self.throw_away(Garbage::Instrument(instrument));
                    self.throw_away(Garbage::Notes(notes));
                }
            }
            Command::RemoveChannel(index) => {
                if index < self.channels.len() {
                    let removed = self.channels.remove(index);
                    self.throw_away(Garbage::Instrument(removed.instrument));
                    self.throw_away(Garbage::Notes(removed.notes));
                }
            }
            Command::SetInstrument {
                channel,
                instrument,
            } => match self.channels.get_mut(channel) {
                Some(state) => {
                    let old = std::mem::replace(&mut state.instrument, instrument);
                    self.throw_away(Garbage::Instrument(old));
                }
                None => self.throw_away(Garbage::Instrument(instrument)),
            },
            Command::SetNotes { channel, notes } => match self.channels.get_mut(channel) {
                Some(state) => {
                    let old = std::mem::replace(&mut state.notes, notes);
                    self.throw_away(Garbage::Notes(old));
                }
                None => self.throw_away(Garbage::Notes(notes)),
            },
            Command::SetVolume { channel, volume } => {
                if let Some(state) = self.channels.get_mut(channel) {
                    state.volume.set(volume);
                }
            }
            Command::SetMuted { channel, muted } => {
                if let Some(state) = self.channels.get_mut(channel) {
                    state.muted = muted;
                }
            }
            Command::Preview(channel) => self.preview(channel, STEP_NOTE),
            Command::PreviewNote { channel, pitch } => self.preview(channel, pitch),
        }
    }

    fn preview(&mut self, channel: usize, pitch: Note) {
        let gate = self.preview_gate();
        if let Some(state) = self.channels.get_mut(channel) {
            state
                .instrument
                .note_on(pitch, DEFAULT_VELOCITY, gate, self.sample_rate);
        }
    }

    /// Starts the notes of every channel that begin on `step`.
    fn start_notes(&mut self, step: usize) {
        let step_frames = self.sequencer.step_frames();
        let length = self.sequencer.length();
        for channel in &mut self.channels {
            if channel.muted {
                continue;
            }
            // `channel.notes` is borrowed to read while `channel.instrument`
            // is borrowed to change. That is allowed: they are different
            // fields of the struct, so the borrows do not overlap.
            for note in channel.notes.iter().filter(|note| note.start == step) {
                // A note that runs past the end of the pattern is cut there.
                let steps_held = note.length.min(length.saturating_sub(step));
                let gate = (steps_held as f64 * step_frames) as u32;
                channel
                    .instrument
                    .note_on(note.pitch, note.velocity, gate, self.sample_rate);
            }
        }
    }

    /// Applies waiting commands, then fills `output` with samples.
    fn process<T>(&mut self, output: &mut [T])
    where
        T: SizedSample + FromSample<f32>,
    {
        while let Ok(command) = self.commands.pop() {
            self.apply(command);
        }

        for frame in output.chunks_mut(self.output_channels) {
            if self.playing
                && let Some(step) = self.sequencer.tick()
            {
                self.playhead.store(step, Ordering::Relaxed);
                self.start_notes(step);
            }

            let mut mix = 0.0;
            for channel in &mut self.channels {
                let volume = channel.volume.next_value();
                mix += channel.instrument.next_sample() * volume;
            }
            // `tanh` gently rounds off peaks above 1.0 instead of clipping them hard.
            let sample = (mix * self.master_volume.next_value()).tanh();
            frame.fill(T::from_sample(sample));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instrument::{Sampler, Synth};
    use crate::sample::SampleData;

    const RATE: u32 = 48_000;

    struct Harness {
        audio: AudioThread,
        commands: rtrb::Producer<Command>,
        trash: rtrb::Consumer<Garbage>,
        playhead: Arc<AtomicUsize>,
    }

    fn harness() -> Harness {
        let (commands, command_consumer) = rtrb::RingBuffer::new(64);
        let (trash_producer, trash) = rtrb::RingBuffer::new(64);
        let playhead = Arc::new(AtomicUsize::new(NO_STEP));
        let audio = AudioThread::new(RATE, 1, command_consumer, trash_producer, playhead.clone());
        Harness {
            audio,
            commands,
            trash,
            playhead,
        }
    }

    /// A sampler whose sound is a constant 0.5 for 10 frames.
    fn click() -> Instrument {
        Instrument::Sampler(Sampler::new(Arc::new(SampleData::new(vec![0.5; 10], RATE))))
    }

    fn steps(on: &[usize]) -> Vec<NoteEvent> {
        on.iter().map(|&step| NoteEvent::step(step)).collect()
    }

    fn add(h: &mut Harness, on: &[usize], muted: bool) {
        h.commands
            .push(Command::AddChannel {
                instrument: click(),
                notes: steps(on),
                volume: 1.0,
                muted,
            })
            .unwrap();
    }

    fn render(h: &mut Harness, frames: usize) -> Vec<f32> {
        let mut buffer = vec![0.0_f32; frames];
        h.audio.process(&mut buffer);
        buffer
    }

    #[test]
    fn silent_while_stopped() {
        let mut h = harness();
        add(&mut h, &[0], false);
        assert!(render(&mut h, 1_000).iter().all(|&s| s == 0.0));
        assert_eq!(h.playhead.load(Ordering::Relaxed), NO_STEP);
    }

    #[test]
    fn plays_steps_on_time() {
        let mut h = harness();
        add(&mut h, &[0, 1], false);
        h.commands
            .push(Command::SetTempo(Tempo::new(120.0)))
            .unwrap();
        h.commands.push(Command::Play).unwrap();

        // At 120 BPM a step lasts 6000 frames: hits at frame 0 and 6000.
        let out = render(&mut h, 7_000);
        assert!(out[0] != 0.0, "step 0 should sound at once");
        assert!(out[100..6_000].iter().all(|&s| s == 0.0));
        assert!(out[6_000] != 0.0, "step 1 should sound at frame 6000");
        assert_eq!(h.playhead.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn muted_channels_are_silent() {
        let mut h = harness();
        add(&mut h, &[0], true);
        h.commands.push(Command::Play).unwrap();
        assert!(render(&mut h, 1_000).iter().all(|&s| s == 0.0));
    }

    #[test]
    fn steps_can_be_changed_while_playing() {
        let mut h = harness();
        add(&mut h, &[], false);
        h.commands
            .push(Command::SetTempo(Tempo::new(120.0)))
            .unwrap();
        h.commands.push(Command::Play).unwrap();
        render(&mut h, 100);
        h.commands
            .push(Command::SetNotes {
                channel: 0,
                notes: steps(&[1]),
            })
            .unwrap();
        let out = render(&mut h, 6_000);
        assert!(out[5_900] != 0.0, "the new step should play at frame 6000");
    }

    #[test]
    fn replaced_things_go_to_the_trash() {
        let mut h = harness();
        add(&mut h, &[], false);
        h.commands
            .push(Command::SetInstrument {
                channel: 0,
                instrument: click(),
            })
            .unwrap();
        h.commands
            .push(Command::SetNotes {
                channel: 0,
                notes: steps(&[0]),
            })
            .unwrap();
        h.commands.push(Command::RemoveChannel(0)).unwrap();
        render(&mut h, 10);
        let (mut instruments, mut note_lists) = (0, 0);
        while let Ok(garbage) = h.trash.pop() {
            match garbage {
                Garbage::Instrument(_) => instruments += 1,
                Garbage::Notes(_) => note_lists += 1,
            }
        }
        assert_eq!(instruments, 2, "the replaced and the removed instrument");
        assert_eq!(note_lists, 2, "the replaced and the removed notes");
    }

    #[test]
    fn notes_are_held_for_their_length() {
        let mut h = harness();
        h.commands
            .push(Command::AddChannel {
                instrument: Instrument::Synth(Synth::new(RATE)),
                notes: vec![NoteEvent::new(Note::A4, 0, 2)],
                volume: 1.0,
                muted: false,
            })
            .unwrap();
        h.commands
            .push(Command::SetTempo(Tempo::new(120.0)))
            .unwrap();
        h.commands.push(Command::Play).unwrap();
        // Two steps at 120 BPM are 12 000 frames. The note is still held
        // just before that, and fades out (to silence) not long after.
        render(&mut h, 11_900);
        assert_eq!(h.audio.channels[0].instrument.voices_playing(), 1);
        render(&mut h, 48_000);
        assert_eq!(h.audio.channels[0].instrument.voices_playing(), 0);
    }

    #[test]
    fn chords_play_all_their_notes() {
        let mut h = harness();
        let chord = [60, 64, 67].map(|midi| NoteEvent::new(Note::new(midi), 0, 4));
        h.commands
            .push(Command::AddChannel {
                instrument: Instrument::Synth(Synth::new(RATE)),
                notes: chord.to_vec(),
                volume: 1.0,
                muted: false,
            })
            .unwrap();
        h.commands.push(Command::Play).unwrap();
        render(&mut h, 10);
        assert_eq!(h.audio.channels[0].instrument.voices_playing(), 3);
    }

    #[test]
    fn stopping_releases_held_notes() {
        let mut h = harness();
        h.commands
            .push(Command::AddChannel {
                instrument: Instrument::Synth(Synth::new(RATE)),
                notes: vec![NoteEvent::new(Note::A4, 0, 16)],
                volume: 1.0,
                muted: false,
            })
            .unwrap();
        h.commands.push(Command::Play).unwrap();
        render(&mut h, 1_000);
        h.commands.push(Command::Stop).unwrap();
        render(&mut h, 48_000);
        assert_eq!(h.audio.channels[0].instrument.voices_playing(), 0);
    }

    #[test]
    fn preview_plays_without_the_sequencer() {
        let mut h = harness();
        add(&mut h, &[], false);
        h.commands.push(Command::Preview(0)).unwrap();
        assert!(render(&mut h, 10)[0] != 0.0);
    }

    #[test]
    fn commands_stay_small() {
        // Every slot of the command queue is as big as the biggest
        // command. The instruments keep their voices in a `Box`, so a
        // command holds a pointer to them instead of all the voices.
        let size = std::mem::size_of::<Command>();
        assert!(size <= 64, "a command takes {size} bytes");
    }

    #[test]
    fn converts_to_integer_samples() {
        let (mut commands, consumer) = rtrb::RingBuffer::new(8);
        let (trash, _trash_consumer) = rtrb::RingBuffer::new(8);
        let playhead = Arc::new(AtomicUsize::new(NO_STEP));
        let mut audio = AudioThread::new(RATE, 1, consumer, trash, playhead);
        commands
            .push(Command::AddChannel {
                instrument: click(),
                notes: steps(&[0]),
                volume: 1.0,
                muted: false,
            })
            .unwrap();
        commands.push(Command::SetMasterVolume(1.0)).unwrap();
        commands.push(Command::Play).unwrap();
        let mut buffer = vec![0_i16; 10];
        audio.process(&mut buffer);
        // About 10 000: half volume, times the default velocity, times
        // the master volume, which is still sliding up from 0.8.
        assert!(buffer[0] > 5_000, "got {}", buffer[0]);
    }
}
