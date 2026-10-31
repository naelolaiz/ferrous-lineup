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
//! playhead in the UI, and another counts the times the loop came round)
//! and old instruments, note lists and songs that need freeing (through
//! a second ring buffer, so the memory is freed on the UI thread).
//!
//! The audio thread keeps its own copy of what it needs to play: the
//! notes of every channel in every pattern, the length of every pattern,
//! and the song (the clips of the playlist). In pattern mode it loops the
//! selected pattern; in song mode it plays the clips.
//!
//! Since step 7 there are two more ways in and out, for MIDI:
//!
//! - Keys played on a MIDI keyboard arrive through a third ring buffer,
//!   [`LiveInput`], which the thread that receives them from the keyboard
//!   fills. They go straight to the audio thread, not through the UI, so
//!   a key sounds within one block of being pressed.
//! - MIDI Out channels make MIDI messages instead of sound. They leave
//!   through a fourth ring buffer, [`MidiOutput`], for a thread that
//!   sends them to the MIDI port, because sending can wait.
//!
//! And a fifth, while a trace is on: a [`Tracer`] writes down every note
//! the thread starts and ends, for another thread to read (see
//! [`crate::trace`]).
//!
//! The mixer's meters read how loud each channel and the master play
//! from shared atomics, [`Levels`], which the audio thread raises and the
//! UI takes once a frame (see [`crate::levels`]).
//!
//! And while a take is recorded, a [`SongClock`] says, block by block,
//! when the sound of the block will reach the speakers and where the song
//! is then, so the microphone's sound can be placed in the song (see
//! [`crate::input`]).

use std::collections::VecDeque;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig};
use lineup_core::i18n::{tr, trf};
use lineup_core::sequencer::frames_per_step;
use lineup_core::{
    Click, DEFAULT_VELOCITY, MAX_CHANNELS, MAX_PATTERNS, MidiMessage, Note, NoteEvent, Play,
    STEP_NOTE, STEPS_PER_BAR, STEPS_PER_BEAT, Sequencer, TICKS_PER_STEP, Tempo,
};

use crate::dsp::Smoothed;
use crate::effects::{ChannelEffects, DelayLine};
use crate::instrument::{HELD, Instrument, Origin};
use crate::levels::Levels;
use crate::metronome::{ClickVoice, click_at, count_in_bar};
use crate::phrase::Phrase;
use crate::trace::Tracer;

/// How many commands can wait in the queue before `send` starts failing.
/// The audio thread empties the queue every few milliseconds. A full
/// project sends one command per pattern and a few per channel, so this
/// holds a whole large project; `send_wait` covers what is left.
const COMMAND_QUEUE_SIZE: usize = 4096;

/// How many times [`AudioEngine::send_wait`] tries to hand a command to a
/// full queue before giving up. The audio thread drains it every block,
/// so this only runs out when the stream has stopped.
const SEND_WAIT_ROUNDS: usize = 1_000;

/// How many old instruments and note lists can wait to be freed by the UI
/// thread. Removing a pattern throws away one list per channel, and
/// loading a project throws away every channel, so this is generous.
pub(crate) const TRASH_QUEUE_SIZE: usize = 1024;

/// How many values can wait on the audio thread while the trash queue is
/// full. The UI empties it every frame, so this only happens in a burst
/// between two frames; the waiting values are never freed on the audio
/// thread unless this fills too.
const TRASH_BACKLOG: usize = 256;

/// How many MIDI messages can wait for room in the output queue. A lost
/// note off leaves a note stuck on the external synth, so these are kept
/// rather than dropped. The sending thread empties the queue every
/// millisecond, far faster than a block makes messages.
const MIDI_BACKLOG: usize = 256;

/// How many keys from a MIDI keyboard can wait for the next block. Ten
/// fingers and a sustain-happy pianist are nowhere near it.
const LIVE_QUEUE_SIZE: usize = 256;

/// How many MIDI messages can wait for the thread that sends them.
const MIDI_OUT_QUEUE_SIZE: usize = 1024;

/// How many blocks' worth of [`Heard`] can wait for the recorder: a few
/// seconds of them, far more than the recorder lets wait.
const HEARD_QUEUE_SIZE: usize = 1024;

/// The value of the playhead while stopped.
pub(crate) const NO_STEP: usize = usize::MAX;

/// The value of the playback position while stopped. The position is an
/// `f64` kept in an `AtomicU64` (there is no atomic float type), as its
/// 64 bits; these bits are a "not a number" that a position never is.
pub(crate) const NO_POSITION: u64 = u64::MAX;

/// How long volume changes take to settle, in seconds.
const VOLUME_SMOOTHING: f32 = 0.01;

/// How many notes can wait to start late, with loose timing on. Each
/// waits a few hundredths of a second at most, so even a dense song has
/// only a few dozen waiting at once.
const PENDING_NOTES: usize = 512;

/// What the play button plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayMode {
    /// Loop the selected pattern, to work on it.
    #[default]
    Pattern,
    /// Play the playlist from the start, and loop at the end of the song.
    Song,
}

/// The notes one channel plays in each pattern: list `p` holds its notes
/// in pattern `p`.
///
/// When the UI adds a pattern, the audio thread adds an empty list to
/// every channel. `Vec::new()` does not allocate (an empty `Vec` has no
/// memory of its own yet), but inserting it into the outer `Vec` would,
/// if the outer `Vec` were full. So `ChannelNotes::new`, which runs on the
/// UI thread, reserves room for [`MAX_PATTERNS`] lists up front. The
/// field is private, so there is no other way to make one: the type
/// itself guarantees the room is there.
#[derive(Debug, Default)]
pub struct ChannelNotes(Vec<Vec<NoteEvent>>);

impl ChannelNotes {
    /// One list per pattern, in pattern order.
    pub fn new(lists: impl IntoIterator<Item = Vec<NoteEvent>>) -> Self {
        let mut patterns = Vec::with_capacity(MAX_PATTERNS);
        patterns.extend(lists.into_iter().take(MAX_PATTERNS));
        ChannelNotes(patterns)
    }

    /// The notes in one pattern (none if there is no such pattern).
    fn get(&self, pattern: usize) -> &[NoteEvent] {
        self.0.get(pattern).map_or(&[], Vec::as_slice)
    }

    /// Adds an empty list for a new pattern at position `pattern`, without
    /// allocating. The lists from there on move up one place.
    fn insert_empty(&mut self, pattern: usize) {
        if self.0.len() < self.0.capacity() {
            self.0.insert(pattern.min(self.0.len()), Vec::new());
        }
    }

    /// Puts `notes` in place of a pattern's list and returns the old list,
    /// or gives `notes` back if there is no such pattern. Either way, the
    /// caller gets a list to throw away.
    fn replace(&mut self, pattern: usize, notes: Vec<NoteEvent>) -> Vec<NoteEvent> {
        match self.0.get_mut(pattern) {
            Some(list) => std::mem::replace(list, notes),
            None => notes,
        }
    }

    /// Takes out a pattern's list; the lists after it move down one place.
    fn remove(&mut self, pattern: usize) -> Option<Vec<NoteEvent>> {
        (pattern < self.0.len()).then(|| self.0.remove(pattern))
    }
}

/// The tempos for the next lap (see [`Command::SetLapTempos`]).
#[derive(Debug)]
pub struct LapTempos {
    /// In place of the [`Command::SetTempo`] tempo.
    pub tempo: Tempo,
    /// In place of the [`Command::SetSongTempos`] list.
    pub song: Vec<Tempo>,
}

/// A message from the UI thread to the audio thread.
///
/// Channel and pattern numbers are positions in the UI's lists, counted
/// from 0, and must match them exactly: the UI sends `InsertChannel`,
/// `RemoveChannel`, `InsertPattern` and `RemovePattern` whenever its own
/// lists change.
#[derive(Debug)]
pub enum Command {
    /// Start playing from the start position: the beginning, or where
    /// the last [`Command::Seek`] put it. With the count-in on, a bar of
    /// clicks comes first.
    Play,
    /// Move the playhead to this step (of the pattern in pattern mode, of
    /// the song in song mode). While stopped, it is where `Play` starts;
    /// while playing, playback jumps there.
    Seek(usize),
    /// Stop playback. Sounds already playing ring out.
    Stop,
    /// Switch between looping a pattern and playing the song. If playing,
    /// starts again from the beginning, and the start position goes back
    /// to the beginning too.
    SetMode(PlayMode),
    /// Choose the pattern that pattern mode loops.
    SelectPattern(usize),
    /// Change the tempo. Notes that are sounding keep their length in
    /// steps: they end sooner or later, on the step they should. With
    /// [`Command::SetSongTempos`], this is the tempo of pattern mode only.
    SetTempo(Tempo),
    /// The tempo of every step of the song, for a song whose tempo
    /// changes: song step `n` plays at `tempos[n]`. An empty list (the
    /// usual case) plays the whole song at the [`Command::SetTempo`]
    /// tempo. Sent together with [`Command::SetSong`].
    SetSongTempos(Vec<Tempo>),
    /// Tempos to play from the next time the loop comes round (see
    /// [`AudioEngine::laps`]) on, in place of those of
    /// [`Command::SetTempo`] and [`Command::SetSongTempos`]. They take
    /// over on the loop's first step, once; `None` takes back tempos
    /// asked for before. The practice speed trainer uses it, so that
    /// each new speed starts exactly with the loop.
    SetLapTempos(Option<LapTempos>),
    /// How far off the grid notes may start, in milliseconds: each one
    /// starts up to this much late. 0 plays every note on its step.
    SetLooseTiming(u8),
    SetMasterVolume(f32),
    SetReverb(lineup_core::ReverbSettings),
    /// Add an empty pattern at position `index`; the patterns from there
    /// on move up one place. An index past the end adds it at the end.
    InsertPattern {
        index: usize,
        length: usize,
    },
    RemovePattern(usize),
    SetPatternLength {
        pattern: usize,
        length: usize,
    },
    /// Replace the song: the pieces of patterns to play, with every
    /// repeat already unrolled (see [`lineup_core::Arrangement`]), and the
    /// song's length in steps. While the song plays, notes that the new
    /// song no longer plays (their clip was taken away, or its track was
    /// muted) stop.
    SetSong {
        plays: Vec<Play>,
        length: usize,
    },
    /// Add a channel at position `index`, with its notes in every pattern.
    /// The channels from there on move down one row. An index past the
    /// end adds it at the bottom.
    InsertChannel {
        index: usize,
        instrument: Instrument,
        notes: ChannelNotes,
        volume: f32,
        muted: bool,
    },
    RemoveChannel(usize),
    /// Replace a channel's instrument, for example after loading a sample.
    SetInstrument {
        channel: usize,
        instrument: Instrument,
    },
    /// Replace all the notes of a channel in one pattern. The UI sends a
    /// fresh copy of the list whenever a step or a note changes.
    SetNotes {
        pattern: usize,
        channel: usize,
        notes: Vec<NoteEvent>,
    },
    SetVolume {
        channel: usize,
        volume: f32,
    },
    /// Move a channel between the speakers (see [`lineup_core::Channel::pan`]).
    SetPan {
        channel: usize,
        pan: f32,
    },
    /// Send more or less of a channel to the reverb (see
    /// [`lineup_core::Channel::reverb_send`]).
    SetReverbSend {
        channel: usize,
        send: f32,
    },
    /// Change a channel's equalizer, compressor and delay (see
    /// [`lineup_core::Channel::effects`]). A delay needs a `line` the
    /// first time it is on; one the channel does not need comes back to
    /// be freed.
    SetEffects {
        channel: usize,
        effects: lineup_core::Effects,
        line: Option<DelayLine>,
    },
    /// Mute or unmute a channel. Muting it also stops the notes it is
    /// playing, but not keys held on a keyboard: those play on a muted
    /// channel too.
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
    /// A key went down or up on the typing keyboard. Keys from a MIDI
    /// keyboard take the faster way, through [`LiveInput`].
    Live(MidiMessage),
    /// Choose the channel that keys play. `None` releases live keys and
    /// ignores new key presses until another channel is selected.
    SetLiveChannel(Option<usize>),
    /// What the metronome plays on each step of the song, in song steps
    /// (see [`lineup_core::Project::played_clicks`]). Sent with
    /// [`Command::SetSong`].
    SetSongClicks(Vec<Click>),
    /// What the metronome plays on each step of the selected pattern.
    SetPatternClicks(Vec<Click>),
    /// Turn the metronome on or off: a click on every beat while playing.
    SetMetronome(bool),
    /// Turn the count-in on or off: a bar of clicks before playback
    /// starts, so you come in on time.
    SetCountIn(bool),
    /// Stop, and remove every channel, pattern and clip, before a new
    /// project is sent.
    Clear,
    /// Play a phrase over whatever else is playing, in place of the
    /// phrase playing before, if any: an ear-training question.
    PlayPhrase(Box<Phrase>),
    /// Let go of the phrase playing, and let it ring out.
    StopPhrase,
    /// Start writing a trace of what is played with this tracer, in
    /// place of the one writing before, if any; or stop (`None`).
    SetTrace(Option<Box<Tracer>>),
    /// Play a pattern over the song as it plays, or stop it (`None`): to
    /// try a pattern out with the song. It starts on the song's next bar
    /// line and loops in time with it. Another pattern takes over on the
    /// next bar line too, and after a jump (a seek, or the song going back
    /// round to its start) it starts again from its top on the next one.
    /// Only song mode plays it; [`Command::Stop`] and
    /// [`Command::SetMode`] end it.
    SetOverlay(Option<usize>),
}

/// Something the audio thread has finished with. It is sent back to the
/// UI thread, because freeing memory is not allowed on the audio thread.
// A channel is much bigger than the rest, but it cannot come in a box:
// the audio thread builds channels, and must not allocate.
#[allow(clippy::large_enum_variant)]
pub(crate) enum Garbage {
    Instrument(Instrument),
    Notes(Vec<NoteEvent>),
    Channel(ChannelState),
    Song(Vec<Play>),
    Tempos(Vec<Tempo>),
    Clicks(Vec<Click>),
    Phrase(Box<Phrase>),
    Trace(Box<Tracer>),
    DelayLine(DelayLine),
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
            EngineError::NoOutputDevice => f.write_str(tr("no audio output device found")),
            EngineError::Config(err) => {
                f.write_str(&trf("could not read the device settings: {}", &[err]))
            }
            EngineError::UnsupportedFormat(format) => {
                f.write_str(&trf("unsupported sample format: {}", &[format]))
            }
            EngineError::BuildStream(err) => {
                f.write_str(&trf("could not open the audio stream: {}", &[err]))
            }
            EngineError::Play(err) => {
                f.write_str(&trf("could not start the audio stream: {}", &[err]))
            }
        }
    }
}

impl std::error::Error for EngineError {}

/// Reads a playback position stored as the bits of an `f64`.
pub(crate) fn load_position(position: &AtomicU64) -> Option<f64> {
    match position.load(Ordering::Relaxed) {
        NO_POSITION => None,
        bits => Some(f64::from_bits(bits)),
    }
}

/// The pattern played over the song and the step of it, kept in one
/// `AtomicU64` so they are always read together: the pattern in the high
/// 32 bits, the step in the low ones, or [`NO_POSITION`] for none.
fn store_overlay(overlay: &AtomicU64, playing: Option<(usize, usize)>) {
    let bits = playing.map_or(NO_POSITION, |(pattern, step)| {
        ((pattern as u64) << 32) | step as u64
    });
    overlay.store(bits, Ordering::Relaxed);
}

/// Reads what [`store_overlay`] stored.
fn load_overlay(overlay: &AtomicU64) -> Option<(usize, usize)> {
    match overlay.load(Ordering::Relaxed) {
        NO_POSITION => None,
        bits => Some(((bits >> 32) as usize, (bits & u64::from(u32::MAX)) as usize)),
    }
}

/// The way in for keys played on a MIDI keyboard.
///
/// It is the producer end of a ring buffer whose consumer is the audio
/// thread. There can be only one producer, so this is a value to move to
/// the one thread that receives the keyboard's messages, not a method on
/// [`AudioEngine`].
pub struct LiveInput {
    notes: rtrb::Producer<MidiMessage>,
    position: Arc<AtomicU64>,
}

impl LiveInput {
    /// Plays (or lets go of) a key on the live channel. Returns `false`
    /// if the queue was full and the key was lost.
    pub fn send(&mut self, message: MidiMessage) -> bool {
        self.notes.push(message).is_ok()
    }

    /// Where playback is, in steps (see [`Sequencer::position`]), or
    /// `None` when stopped. Recording uses it to place the key.
    pub fn position(&self) -> Option<f64> {
        load_position(&self.position)
    }
}

/// When the sound of one block reaches the speakers, and where the song
/// is then.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Heard {
    /// When the block's first sample comes out of the speakers: when the
    /// audio thread made it, plus how long the sound card says it takes
    /// to get there.
    pub at: Instant,
    /// The position of the song (or of the pattern) at that sample, in
    /// steps with a fraction (see [`Sequencer::position`]).
    pub position: f64,
    /// How many steps go by in a second at the tempo playing.
    pub steps_per_second: f64,
}

/// What the song is doing as it is heard, block by block, while asked
/// (see [`AudioEngine::take_song_clock`]). A take recorded along with the
/// song is placed in it with this.
pub struct SongClock {
    heard: rtrb::Consumer<Heard>,
    wanted: Arc<AtomicBool>,
}

impl SongClock {
    /// Starts or stops writing down what is heard. Stopped, the audio
    /// thread writes nothing, so nothing waits in the queue.
    pub fn listen(&mut self, on: bool) {
        if on {
            while self.heard.pop().is_ok() {}
        }
        self.wanted.store(on, Ordering::Relaxed);
    }

    /// The oldest block heard that has not been taken yet.
    pub fn pop(&mut self) -> Option<Heard> {
        self.heard.pop().ok()
    }
}

/// The way out for the messages that MIDI Out channels make, to be moved
/// to the thread that sends them to a MIDI port.
pub struct MidiOutput(rtrb::Consumer<MidiMessage>);

impl MidiOutput {
    /// The oldest message waiting, if any.
    pub fn pop(&mut self) -> Option<MidiMessage> {
        self.0.pop().ok()
    }

    /// Whether the audio thread is gone (the engine was dropped), so no
    /// more messages will come.
    pub fn is_abandoned(&self) -> bool {
        self.0.is_abandoned()
    }
}

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
    position: Arc<AtomicU64>,
    overlay: Arc<AtomicU64>,
    laps: Arc<AtomicUsize>,
    levels: Arc<Levels>,
    live_input: Option<LiveInput>,
    midi_output: Option<MidiOutput>,
    song_clock: Option<SongClock>,
    /// The last problem the stream reported, for the UI to show, and
    /// whether it has stopped working.
    problems: Arc<StreamProblems>,
    info: EngineInfo,
}

/// What the stream's error callback tells the rest of the program.
#[derive(Default)]
struct StreamProblems {
    /// The last error, taken by the window.
    error: Mutex<Option<String>>,
    /// Set when the stream reports an error, so a command sent to a dead
    /// stream fails at once instead of waiting for room that never comes.
    broken: AtomicBool,
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
        let (live_producer, live_consumer) = rtrb::RingBuffer::new(LIVE_QUEUE_SIZE);
        let (midi_producer, midi_consumer) = rtrb::RingBuffer::new(MIDI_OUT_QUEUE_SIZE);
        let (heard_producer, heard_consumer) = rtrb::RingBuffer::new(HEARD_QUEUE_SIZE);
        let playhead = Arc::new(AtomicUsize::new(NO_STEP));
        let position = Arc::new(AtomicU64::new(NO_POSITION));

        let mut audio_thread = AudioThread::new(
            config.sample_rate,
            config.channels as usize,
            command_consumer,
            trash_producer,
            Arc::clone(&playhead),
            Arc::clone(&position),
        );
        audio_thread.live_input = Some(live_consumer);
        audio_thread.midi_out = Some(midi_producer);
        let wanted = Arc::new(AtomicBool::new(false));
        audio_thread.heard = Some(HeardOut {
            queue: heard_producer,
            wanted: Arc::clone(&wanted),
            at: None,
        });
        let song_clock = SongClock {
            heard: heard_consumer,
            wanted,
        };
        let overlay = Arc::clone(&audio_thread.overlay_playhead);
        let laps = Arc::clone(&audio_thread.laps);
        let levels = Arc::clone(audio_thread.levels());
        let live_input = LiveInput {
            notes: live_producer,
            position: Arc::clone(&position),
        };

        // The sound card decides the sample type. We pick the matching
        // version of the generic `build_stream` function for each one.
        let problems = Arc::new(StreamProblems::default());
        let stream = match sample_format {
            SampleFormat::F32 => {
                build_stream::<f32>(&device, config, audio_thread, Arc::clone(&problems))
            }
            SampleFormat::F64 => {
                build_stream::<f64>(&device, config, audio_thread, Arc::clone(&problems))
            }
            SampleFormat::I16 => {
                build_stream::<i16>(&device, config, audio_thread, Arc::clone(&problems))
            }
            SampleFormat::I32 => {
                build_stream::<i32>(&device, config, audio_thread, Arc::clone(&problems))
            }
            SampleFormat::U16 => {
                build_stream::<u16>(&device, config, audio_thread, Arc::clone(&problems))
            }
            SampleFormat::U8 => {
                build_stream::<u8>(&device, config, audio_thread, Arc::clone(&problems))
            }
            other => return Err(EngineError::UnsupportedFormat(other)),
        }?;
        stream.play().map_err(EngineError::Play)?;

        Ok(AudioEngine {
            _stream: stream,
            commands: command_producer,
            trash: trash_consumer,
            playhead,
            position,
            overlay,
            laps,
            levels,
            live_input: Some(live_input),
            midi_output: Some(MidiOutput(midi_consumer)),
            song_clock: Some(song_clock),
            problems,
            info,
        })
    }

    /// Whether the stream has reported an error and stopped working.
    pub fn is_broken(&self) -> bool {
        self.problems.broken.load(Ordering::Relaxed)
    }

    /// Takes the last problem the stream reported, if any. The window
    /// shows it, so an unplugged interface is not silent in both senses.
    pub fn take_error(&self) -> Option<String> {
        // No error has ever been reported: do not lock on every frame.
        if !self.problems.broken.load(Ordering::Relaxed) {
            return None;
        }
        self.problems
            .error
            .lock()
            .ok()
            .and_then(|mut error| error.take())
    }

    /// Sends a command to the audio thread.
    ///
    /// Returns `false` if the queue was full and the command was dropped.
    pub fn send(&mut self, command: Command) -> bool {
        self.commands.push(command).is_ok()
    }

    /// Sends a command, waiting if the queue is full.
    ///
    /// The audio thread empties the queue at the start of every block, so
    /// a full queue means the UI got ahead of it: waiting a moment is
    /// better than losing the command. Returns `false` if there is still
    /// no room after a short wait, as when the device has stopped.
    pub fn send_wait(&mut self, mut command: Command) -> bool {
        for _ in 0..SEND_WAIT_ROUNDS {
            match self.commands.push(command) {
                Ok(()) => return true,
                Err(rtrb::PushError::Full(returned)) => command = returned,
            }
            // A stream that has reported an error will not empty the
            // queue, so waiting for room would only freeze the window.
            if self.problems.broken.load(Ordering::Relaxed) {
                return false;
            }
            std::thread::yield_now();
        }
        false
    }

    /// How loud each channel and the master have been since the last
    /// look, for the mixer's meters.
    pub fn levels(&self) -> &Levels {
        &self.levels
    }

    /// The step being played right now, or `None` when stopped.
    pub fn playhead(&self) -> Option<usize> {
        match self.playhead.load(Ordering::Relaxed) {
            NO_STEP => None,
            step => Some(step),
        }
    }

    /// Where playback is, in steps with a fraction, or `None` when stopped.
    pub fn position(&self) -> Option<f64> {
        load_position(&self.position)
    }

    /// The pattern playing over the song (see [`Command::SetOverlay`]) and
    /// the step of it being played, or `None` while none is.
    pub fn overlay(&self) -> Option<(usize, usize)> {
        load_overlay(&self.overlay)
    }

    /// How many times playback has come round to the start of what it
    /// loops (the selected pattern, the song or the part of it set to
    /// loop) by playing on past its end, since the audio thread started.
    /// A jump back to the start does not count.
    pub fn laps(&self) -> usize {
        self.laps.load(Ordering::Relaxed)
    }

    /// The way in for a MIDI keyboard. It can be taken once; after that
    /// this returns `None`.
    pub fn take_live_input(&mut self) -> Option<LiveInput> {
        self.live_input.take()
    }

    /// The way out for MIDI Out channels. It can be taken once.
    pub fn take_midi_output(&mut self) -> Option<MidiOutput> {
        self.midi_output.take()
    }

    /// What the song is doing as it is heard, for recording a take. It
    /// can be taken once at a time: give it back with
    /// [`AudioEngine::give_back_song_clock`] when the take is over.
    pub fn take_song_clock(&mut self) -> Option<SongClock> {
        self.song_clock.take()
    }

    /// Gives back the song clock taken with
    /// [`AudioEngine::take_song_clock`]. A clock this engine still holds
    /// stays: the one given back came from an engine before it.
    pub fn give_back_song_clock(&mut self, mut clock: SongClock) {
        clock.listen(false);
        if self.song_clock.is_none() {
            self.song_clock = Some(clock);
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
                Garbage::Channel(channel) => drop(channel),
                Garbage::Song(clips) => drop(clips),
                Garbage::Tempos(tempos) => drop(tempos),
                Garbage::Clicks(clicks) => drop(clicks),
                Garbage::Phrase(phrase) => drop(phrase),
                Garbage::Trace(trace) => drop(trace),
                Garbage::DelayLine(line) => drop(line),
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
    problems: Arc<StreamProblems>,
) -> Result<cpal::Stream, EngineError>
where
    T: SizedSample + FromSample<f32>,
{
    // This closure is the audio callback. `move` gives it ownership of
    // `audio_thread`, and cpal runs it on the audio thread.
    let data_callback = move |output: &mut [T], info: &cpal::OutputCallbackInfo| {
        // How long the sound card says this block takes to reach the
        // speakers: the time from now until it is played.
        let stamp = info.timestamp();
        let delay = stamp.playback.duration_since(stamp.callback);
        audio_thread.heard_after(delay);
        audio_thread.process(output);
    };

    // Called by cpal if something goes wrong while the stream runs, for
    // example when the device is unplugged. It does not run on the audio
    // callback, so it may lock and print; it keeps the newest problem for
    // the window to show, and prints it for a run without one.
    let error_callback = move |err: cpal::Error| {
        problems.broken.store(true, Ordering::Relaxed);
        if let Ok(mut slot) = problems.error.lock() {
            *slot = Some(err.to_string());
        }
        eprintln!("audio stream error: {err}");
    };

    device
        .build_output_stream(config, data_callback, error_callback, None)
        .map_err(EngineError::BuildStream)
}

/// One row of the channel rack, as the audio thread sees it.
pub(crate) struct ChannelState {
    instrument: Instrument,
    notes: ChannelNotes,
    volume: Smoothed,
    muted: bool,
    pan: Smoothed,
    reverb_send: Smoothed,
    effects: ChannelEffects,
    /// The loudest sample since the end of the last block, for the meters.
    peak: f32,
}

/// Everything the audio callback owns.
///
/// Keeping this in a struct (instead of loose variables inside the
/// closure) lets the tests below run it without a sound card, and lets
/// [`Renderer`](crate::render::Renderer) play a song into a file.
pub(crate) struct AudioThread {
    sample_rate: u32,
    output_channels: usize,
    /// Reserved with room for `MAX_CHANNELS`, so `push` never allocates.
    channels: Vec<ChannelState>,
    /// The length of each pattern, in steps. Reserved with room for `MAX_PATTERNS`.
    pattern_lengths: Vec<usize>,
    /// The pieces of patterns the song plays, and its length in steps.
    plays: Vec<Play>,
    song_length: usize,
    /// The tempo of each step of the song, or none if it never changes.
    song_tempos: Vec<Tempo>,
    /// The tempo the user set: pattern mode's, and the song's when
    /// `song_tempos` is empty.
    tempo: Tempo,
    /// The tempo the sequencer is running at right now.
    tempo_now: Tempo,
    /// How many frames late a note may start, with loose timing.
    loose_frames: f64,
    /// Notes waiting to start late. Reserved with room for
    /// [`PENDING_NOTES`], so `push` never allocates.
    pending: Vec<Pending>,
    /// Frames played since the engine started: the clock pending notes
    /// wait on.
    now: u64,
    mode: PlayMode,
    /// The pattern that pattern mode loops.
    pattern: usize,
    sequencer: Sequencer,
    playing: bool,
    /// Where `Play` starts, set by `Seek`.
    start: usize,
    master_volume: Smoothed,
    reverb: crate::reverb::Reverb,
    commands: rtrb::Consumer<Command>,
    trash: rtrb::Producer<Garbage>,
    /// Values that found the trash queue full. Reserved up front, so
    /// keeping them never allocates on the audio thread.
    trash_backlog: VecDeque<Garbage>,
    playhead: Arc<AtomicUsize>,
    /// The sequencer's position, stored after every block.
    position: Arc<AtomicU64>,
    /// Keys from a MIDI keyboard. `None` when there is no keyboard to
    /// listen to, as when exporting a WAV file.
    live_input: Option<rtrb::Consumer<MidiMessage>>,
    /// Where MIDI Out channels send their messages, if anywhere.
    midi_out: Option<rtrb::Producer<MidiMessage>>,
    /// Messages that found the MIDI Out queue full in an earlier block,
    /// oldest first. Reserved up front, so keeping them never allocates.
    midi_backlog: VecDeque<MidiMessage>,
    /// The channel that keys play.
    live_channel: Option<usize>,
    /// For each of the 128 notes whose key is down, the channel it went
    /// down on. The key must come up on the same channel, even if
    /// another one has been selected in between, or the note would never end.
    live_keys: [Option<usize>; 128],
    /// What the metronome plays on each step of the song, and of the
    /// selected pattern.
    song_clicks: Vec<Click>,
    pattern_clicks: Vec<Click>,
    metronome: bool,
    count_in: bool,
    /// The count-in under way, if `Play` came with the count-in on.
    counting: Option<CountIn>,
    /// The count-in's own clock, so it runs at the tempo where playback
    /// will start without moving the playhead.
    counter: Sequencer,
    click: ClickVoice,
    /// A phrase playing apart from the song, if any.
    phrase: Option<Box<Phrase>>,
    /// The pattern played over the song, if any.
    overlay: Option<Overlay>,
    /// What it is playing, for the UI (see [`store_overlay`]).
    overlay_playhead: Arc<AtomicU64>,
    /// The sequencer's laps (see [`AudioEngine::laps`]), stored on every
    /// step.
    laps: Arc<AtomicUsize>,
    /// The tempos to play from the next lap on, if asked (see
    /// [`Command::SetLapTempos`]).
    lap_tempos: Option<LapTempos>,
    /// Where every note played is written down, while a trace is on.
    trace: Option<Box<Tracer>>,
    /// How loud each channel and the master were, for the mixer's meters.
    levels: Arc<Levels>,
    /// Where what is heard goes while a take is recorded. `None` when no
    /// one will be listening, as when exporting a WAV file.
    heard: Option<HeardOut>,
}

/// The audio thread's end of a [`SongClock`].
struct HeardOut {
    queue: rtrb::Producer<Heard>,
    wanted: Arc<AtomicBool>,
    /// When the block about to be made will be heard.
    at: Option<Instant>,
}

/// A pattern played over the song (see [`Command::SetOverlay`]).
#[derive(Debug, Clone, Copy, Default)]
struct Overlay {
    /// The pattern playing, and the step of it the next song step plays.
    /// `None` until the bar line it starts on.
    playing: Option<(usize, usize)>,
    /// The pattern to play from the next bar line on, if it changes.
    coming: Option<usize>,
    /// The song step played last, to notice a jump.
    last: Option<usize>,
}

/// A count-in under way: the bar it counts (where it starts in the
/// loop's clicks, and how many steps it has), how many steps it lasts in
/// all, and how many have gone by.
#[derive(Debug, Clone, Copy)]
struct CountIn {
    bar: usize,
    length: usize,
    steps: usize,
    counted: usize,
}

impl AudioThread {
    pub(crate) fn new(
        sample_rate: u32,
        output_channels: usize,
        commands: rtrb::Consumer<Command>,
        trash: rtrb::Producer<Garbage>,
        playhead: Arc<AtomicUsize>,
        position: Arc<AtomicU64>,
    ) -> Self {
        AudioThread {
            sample_rate,
            output_channels,
            channels: Vec::with_capacity(MAX_CHANNELS),
            pattern_lengths: Vec::with_capacity(MAX_PATTERNS),
            plays: Vec::new(),
            song_length: 0,
            song_tempos: Vec::new(),
            tempo: Tempo::default(),
            tempo_now: Tempo::default(),
            loose_frames: 0.0,
            pending: Vec::with_capacity(PENDING_NOTES),
            now: 0,
            mode: PlayMode::Pattern,
            pattern: 0,
            sequencer: Sequencer::new(Tempo::default(), sample_rate, STEPS_PER_BAR),
            playing: false,
            start: 0,
            master_volume: Smoothed::new(0.8, VOLUME_SMOOTHING, sample_rate),
            reverb: crate::reverb::Reverb::new(sample_rate),
            commands,
            trash,
            trash_backlog: VecDeque::with_capacity(TRASH_BACKLOG),
            playhead,
            position,
            live_input: None,
            midi_out: None,
            midi_backlog: VecDeque::with_capacity(MIDI_BACKLOG),
            live_channel: Some(0),
            live_keys: [None; 128],
            song_clicks: Vec::new(),
            pattern_clicks: Vec::new(),
            metronome: false,
            count_in: false,
            counting: None,
            counter: Sequencer::new(Tempo::default(), sample_rate, STEPS_PER_BAR),
            click: ClickVoice::new(sample_rate),
            phrase: None,
            overlay: None,
            overlay_playhead: Arc::new(AtomicU64::new(NO_POSITION)),
            laps: Arc::new(AtomicUsize::new(0)),
            lap_tempos: None,
            trace: None,
            levels: Arc::default(),
            heard: None,
        }
    }

    /// Says that the next block will reach the speakers `delay` from now.
    pub(crate) fn heard_after(&mut self, delay: Duration) {
        if let Some(heard) = &mut self.heard
            && heard.wanted.load(Ordering::Relaxed)
        {
            heard.at = Some(Instant::now() + delay);
        }
    }

    /// Writes down where the song is as the block about to be made is
    /// heard, if a take is recording and the song is playing (not
    /// counting in, when the song does not move yet).
    fn write_heard(&mut self) {
        let Some(heard) = &mut self.heard else {
            return;
        };
        let Some(at) = heard.at.take() else {
            return;
        };
        if !self.playing || self.counting.is_some() || !heard.wanted.load(Ordering::Relaxed) {
            return;
        }
        let steps_per_second = f64::from(self.sample_rate) / self.sequencer.step_frames().max(1.0);
        // A full queue means no one is reading: the block is let go.
        let _ = heard.queue.push(Heard {
            at,
            position: self.sequencer.position(),
            steps_per_second,
        });
    }

    /// Where it keeps how loud each channel and the master have been.
    pub(crate) fn levels(&self) -> &Arc<Levels> {
        &self.levels
    }

    /// Hands something to the UI thread to be freed there.
    fn throw_away(&mut self, garbage: Garbage) {
        // The queue is emptied every frame, so a full one is a burst
        // between two frames: keep the value until there is room.
        let garbage = match self.trash.push(garbage) {
            Ok(()) => return,
            Err(rtrb::PushError::Full(garbage)) => garbage,
        };
        if self.trash_backlog.len() < TRASH_BACKLOG {
            self.trash_backlog.push_back(garbage);
        }
        // A full backlog too means the UI has not collected for a long
        // time; the value is freed here as a last resort.
    }

    /// Moves values that waited for room in the trash queue into it,
    /// oldest first, for the UI to free.
    fn flush_trash(&mut self) {
        while let Some(garbage) = self.trash_backlog.pop_front() {
            match self.trash.push(garbage) {
                Ok(()) => {}
                Err(rtrb::PushError::Full(garbage)) => {
                    self.trash_backlog.push_front(garbage);
                    break;
                }
            }
        }
    }

    /// Moves MIDI messages that waited for room into the output queue,
    /// oldest first. Called before a block makes new messages.
    fn flush_midi_backlog(&mut self) {
        let Some(out) = &mut self.midi_out else {
            return;
        };
        while let Some(&message) = self.midi_backlog.front() {
            if out.push(message).is_err() {
                break;
            }
            self.midi_backlog.pop_front();
        }
    }

    /// The length of a pattern, in steps (0 if there is no such pattern).
    fn pattern_length(&self, pattern: usize) -> usize {
        self.pattern_lengths.get(pattern).copied().unwrap_or(0)
    }

    /// Tells the sequencer how long the loop is: the selected pattern in
    /// pattern mode, or the whole song. An empty song loops one silent bar.
    fn update_loop_length(&mut self) {
        let length = match self.mode {
            PlayMode::Pattern => self.pattern_length(self.pattern),
            PlayMode::Song => self.song_length,
        };
        let length = if length == 0 { STEPS_PER_BAR } else { length };
        self.sequencer.set_length(length);
    }

    /// How many frames a preview note is held for: one beat.
    fn preview_gate(&self) -> u32 {
        (self.sequencer.step_frames() * STEPS_PER_BEAT as f64) as u32
    }

    pub(crate) fn apply(&mut self, command: Command) {
        match command {
            Command::Play => self.begin_play(),
            Command::Seek(step) => self.seek(step),
            Command::Stop => {
                self.end_overlay();
                self.stop();
            }
            Command::SetMode(mode) => self.set_mode(mode),
            Command::SelectPattern(pattern) => {
                self.pattern = pattern;
                self.update_loop_length();
            }
            Command::SetTempo(tempo) => {
                self.tempo = tempo;
                // A song whose tempo changes sets it step by step.
                if self.mode == PlayMode::Pattern || self.song_tempos.is_empty() {
                    self.set_tempo(tempo);
                }
            }
            Command::SetSongTempos(tempos) => {
                let old = std::mem::replace(&mut self.song_tempos, tempos);
                self.throw_away(Garbage::Tempos(old));
            }
            Command::SetLapTempos(tempos) => {
                if let Some(old) = std::mem::replace(&mut self.lap_tempos, tempos) {
                    self.throw_away(Garbage::Tempos(old.song));
                }
            }
            Command::SetLooseTiming(ms) => {
                self.loose_frames = f64::from(ms) / 1000.0 * f64::from(self.sample_rate);
            }
            Command::SetMasterVolume(volume) => self.master_volume.set(volume),
            Command::SetReverb(settings) => self.reverb.configure(settings),
            Command::InsertPattern { index, length } => self.insert_pattern(index, length),
            Command::RemovePattern(pattern) => self.remove_pattern(pattern),
            Command::SetPatternLength { pattern, length } => {
                if let Some(slot) = self.pattern_lengths.get_mut(pattern) {
                    *slot = length;
                    self.update_loop_length();
                }
            }
            Command::SetSong { plays, length } => self.set_song(plays, length),
            Command::InsertChannel {
                index,
                instrument,
                notes,
                volume,
                muted,
            } => self.insert_channel(index, instrument, notes, volume, muted),
            Command::RemoveChannel(index) => self.remove_channel(index),
            Command::SetInstrument {
                channel,
                instrument,
            } => self.set_instrument(channel, instrument),
            Command::SetNotes {
                pattern,
                channel,
                notes,
            } => {
                let old = match self.channels.get_mut(channel) {
                    Some(state) => state.notes.replace(pattern, notes),
                    None => notes,
                };
                self.throw_away(Garbage::Notes(old));
            }
            Command::SetVolume { channel, volume } => {
                if let Some(state) = self.channels.get_mut(channel) {
                    state.volume.set(volume);
                }
            }
            Command::SetPan { channel, pan } => {
                if let Some(state) = self.channels.get_mut(channel) {
                    state.pan.set(pan);
                }
            }
            Command::SetReverbSend { channel, send } => {
                if let Some(state) = self.channels.get_mut(channel) {
                    state.reverb_send.set(send);
                }
            }
            Command::SetEffects {
                channel,
                effects,
                line,
            } => {
                let tempo = self.tempo_now;
                let spare = match self.channels.get_mut(channel) {
                    Some(state) => state.effects.set(effects, line, tempo),
                    None => line,
                };
                if let Some(spare) = spare {
                    self.throw_away(Garbage::DelayLine(spare));
                }
            }
            Command::SetMuted { channel, muted } => self.set_muted(channel, muted),
            Command::Preview(channel) => self.preview(channel, STEP_NOTE),
            Command::PreviewNote { channel, pitch } => self.preview(channel, pitch),
            Command::Live(message) => self.play_live(message),
            Command::SetLiveChannel(channel) => {
                if channel.is_none() {
                    self.release_live_keys();
                }
                self.live_channel = channel;
            }
            Command::SetSongClicks(clicks) => {
                let old = std::mem::replace(&mut self.song_clicks, clicks);
                self.throw_away(Garbage::Clicks(old));
            }
            Command::SetPatternClicks(clicks) => {
                let old = std::mem::replace(&mut self.pattern_clicks, clicks);
                self.throw_away(Garbage::Clicks(old));
            }
            Command::SetMetronome(on) => self.metronome = on,
            Command::SetCountIn(on) => self.count_in = on,
            Command::Clear => self.clear(),
            Command::PlayPhrase(phrase) => {
                if let Some(old) = self.phrase.replace(phrase) {
                    self.throw_away(Garbage::Phrase(old));
                }
            }
            Command::StopPhrase => {
                if let Some(phrase) = &mut self.phrase {
                    phrase.stop();
                }
            }
            Command::SetOverlay(Some(pattern)) => self.set_overlay(pattern),
            Command::SetOverlay(None) => self.end_overlay(),
            Command::SetTrace(trace) => self.set_trace(trace),
        }
    }

    /// Moves playback to `step`: counted in again if a count-in is
    /// running, and on from there at once if playing.
    fn seek(&mut self, step: usize) {
        self.start = step;
        self.pending.clear();
        if self.counting.is_some() {
            // Counted again, into the bar playback now starts in.
            self.begin_play();
        }
        if self.playing {
            // Notes that were playing would otherwise hang on
            // until the end they will now never reach.
            for channel in &mut self.channels {
                channel.instrument.release_all();
            }
            if let Some(trace) = &mut self.trace {
                trace.jumped(self.now, step);
            }
            self.sequencer.start_at(step);
            self.resume_long_sounds(step);
        }
    }

    /// Starts the long sounds of the song that began before `step` and
    /// still sound there, partway through, as they would be had playback
    /// started earlier (see [`Instrument::resume`]): a recording plays
    /// from wherever playback starts, jumps to, or a loop region begins.
    /// A note on a step before the loop region, in the part of a clip
    /// the region leaves out, counts too, its steps as long as the
    /// region's first.
    fn resume_long_sounds(&mut self, step: usize) {
        if self.mode != PlayMode::Song {
            return;
        }
        let timing = (&self.song_tempos[..], self.tempo, self.sample_rate);
        for play in self.plays.iter().filter(|play| play.start <= step) {
            let length = self.pattern_lengths.get(play.pattern).copied().unwrap_or(0);
            if length == 0 {
                continue;
            }
            for (index, channel) in self.channels.iter_mut().enumerate() {
                if channel.muted {
                    continue;
                }
                for note in channel.notes.get(play.pattern) {
                    // Every time the piece played the note before
                    // `step`, counted from the piece's start, which is
                    // below zero for a time in the part of the clip the
                    // piece skips.
                    let first = play.start as isize - play.skip as isize + note.start as isize;
                    let times = (first..play.end().min(step) as isize).step_by(length);
                    for at in times.filter(|&at| at < step as isize) {
                        let origin = Origin {
                            pattern: play.pattern,
                            pattern_step: note.start,
                            song_step: at.max(0) as usize,
                            over_song: false,
                        };
                        let elapsed = frames_between(timing, at, step);
                        let started = channel.instrument.resume(
                            origin,
                            note.pitch,
                            note.velocity,
                            self.sample_rate,
                            elapsed,
                        );
                        if started && let Some(trace) = self.trace.as_deref_mut() {
                            trace.note_on(
                                self.now,
                                index,
                                origin,
                                note.pitch,
                                note.velocity,
                                0,
                                0,
                                false,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Plays the selected pattern or the song from now on, from the top.
    fn set_mode(&mut self, mode: PlayMode) {
        self.end_overlay();
        self.mode = mode;
        self.pending.clear();
        self.update_loop_length();
        self.start = 0;
        self.sequencer.reset();
        // As after a seek: the notes of the old loop would hang
        // on over the new one until ends they will never reach.
        if self.playing {
            for channel in &mut self.channels {
                channel.instrument.release_all();
            }
            // The steps from now on are another loop's.
            if let Some(trace) = &mut self.trace {
                trace.cut(self.now, None, |_| false);
            }
            self.trace_play(0);
        }
        if self.counting.is_some() {
            self.begin_play();
        }
    }

    /// Adds an empty pattern of `length` steps at `index`, moving the
    /// patterns from there on up by one, if there is room for it.
    fn insert_pattern(&mut self, index: usize, length: usize) {
        if self.pattern_lengths.len() < MAX_PATTERNS {
            let index = index.min(self.pattern_lengths.len());
            self.pattern_lengths.insert(index, length);
            for channel in &mut self.channels {
                channel.notes.insert_empty(index);
            }
            self.update_loop_length();
            self.overlay_moved(|pattern| Some(pattern + usize::from(pattern >= index)));
        }
    }

    /// Takes pattern `pattern` away, with its notes in every channel,
    /// moving the patterns after it down by one.
    fn remove_pattern(&mut self, pattern: usize) {
        if pattern < self.pattern_lengths.len() {
            self.pattern_lengths.remove(pattern);
            for index in 0..self.channels.len() {
                if let Some(notes) = self.channels[index].notes.remove(pattern) {
                    self.throw_away(Garbage::Notes(notes));
                }
            }
            self.update_loop_length();
            let removed = pattern;
            self.overlay_moved(|pattern| match pattern.cmp(&removed) {
                std::cmp::Ordering::Less => Some(pattern),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(pattern - 1),
            });
        }
    }

    /// Takes the song's new clips, `plays`, and its length in steps.
    fn set_song(&mut self, plays: Vec<Play>, length: usize) {
        let old = std::mem::replace(&mut self.plays, plays);
        self.throw_away(Garbage::Song(old));
        self.song_length = length;
        self.update_loop_length();
        if self.playing && self.mode == PlayMode::Song {
            self.release_notes_left_out();
        }
    }

    /// Adds a channel at position `index`, if there is room for it.
    fn insert_channel(
        &mut self,
        index: usize,
        instrument: Instrument,
        notes: ChannelNotes,
        volume: f32,
        muted: bool,
    ) {
        let channel = ChannelState {
            instrument,
            notes,
            volume: Smoothed::new(volume, VOLUME_SMOOTHING, self.sample_rate),
            muted,
            // In the middle, all of it in the reverb, until a
            // `SetPan` or `SetReverbSend` says otherwise.
            pan: Smoothed::new(0.0, VOLUME_SMOOTHING, self.sample_rate),
            reverb_send: Smoothed::new(1.0, VOLUME_SMOOTHING, self.sample_rate),
            // No effects, until a `SetEffects` says otherwise.
            effects: ChannelEffects::new(self.sample_rate),
            peak: 0.0,
        };
        if self.channels.len() < MAX_CHANNELS {
            // The channels are about to move, and `live_keys`
            // and the notes waiting to start know them by position.
            self.release_live_keys();
            self.pending.clear();
            // `insert` moves the channels after `index` along by
            // one, inside the room reserved up front.
            let index = index.min(self.channels.len());
            self.channels.insert(index, channel);
            if let Some(trace) = &mut self.trace {
                trace.channel_inserted(index);
            }
        } else {
            self.throw_away(Garbage::Channel(channel));
        }
    }

    /// Takes channel `index` away, silencing what it was playing.
    fn remove_channel(&mut self, index: usize) {
        if index < self.channels.len() {
            self.release_live_keys();
            self.pending.clear();
            let mut removed = self.channels.remove(index);
            silence(
                &mut removed.instrument,
                &mut self.midi_out,
                &mut self.midi_backlog,
            );
            if let Some(trace) = &mut self.trace {
                trace.channel_removed(self.now, index);
            }
            self.throw_away(Garbage::Channel(removed));
        }
    }

    /// Gives `channel` a new instrument, silencing the old one.
    fn set_instrument(&mut self, channel: usize, instrument: Instrument) {
        match self.channels.get_mut(channel) {
            Some(state) => {
                let mut old = std::mem::replace(&mut state.instrument, instrument);
                silence(&mut old, &mut self.midi_out, &mut self.midi_backlog);
                if let Some(trace) = &mut self.trace {
                    trace.cut(self.now, Some(channel), |_| false);
                }
                self.throw_away(Garbage::Instrument(old));
            }
            None => self.throw_away(Garbage::Instrument(instrument)),
        }
    }

    /// Mutes or unmutes `channel`.
    fn set_muted(&mut self, channel: usize, muted: bool) {
        if let Some(state) = self.channels.get_mut(channel) {
            state.muted = muted;
            if muted {
                // Every note of the song on this channel stops,
                // and those about to start never do.
                state.instrument.release_unless(|_| false);
                self.pending.retain(|note| note.channel != channel);
                if let Some(trace) = &mut self.trace {
                    trace.cut(self.now, Some(channel), |_| false);
                }
            }
        }
    }

    /// Stops and throws everything away, for another project.
    fn clear(&mut self) {
        self.end_overlay();
        self.stop();
        self.reverb.clear();
        self.start = 0;
        // `pop` takes the last channel out of the list; `while let`
        // keeps going until there are none left.
        while let Some(mut channel) = self.channels.pop() {
            silence(
                &mut channel.instrument,
                &mut self.midi_out,
                &mut self.midi_backlog,
            );
            self.throw_away(Garbage::Channel(channel));
        }
        // `clear` empties the list but keeps its memory: nothing is freed.
        self.pattern_lengths.clear();
        let old = std::mem::take(&mut self.plays);
        self.throw_away(Garbage::Song(old));
        let old = std::mem::take(&mut self.song_tempos);
        self.throw_away(Garbage::Tempos(old));
        let old = std::mem::take(&mut self.song_clicks);
        self.throw_away(Garbage::Clicks(old));
        self.song_length = 0;
        self.pattern = 0;
        self.update_loop_length();
    }

    /// Plays `pattern` over the song from its next bar line, while the
    /// song plays (see [`Command::SetOverlay`]).
    fn set_overlay(&mut self, pattern: usize) {
        if self.mode == PlayMode::Song {
            let overlay = self.overlay.get_or_insert_default();
            let playing = overlay.playing.map(|(playing, _)| playing);
            overlay.coming = (playing != Some(pattern)).then_some(pattern);
        }
    }

    /// Starts or stops writing down what plays.
    fn set_trace(&mut self, trace: Option<Box<Tracer>>) {
        if let Some(old) = std::mem::replace(&mut self.trace, trace) {
            self.throw_away(Garbage::Trace(old));
        }
        // A trace started while playing starts where playback is.
        if self.playing {
            self.trace_play(self.sequencer.position() as usize);
        }
    }

    /// Writes down that playback starts on `step` of the loop, if a
    /// trace is on.
    fn trace_play(&mut self, step: usize) {
        if let Some(trace) = &mut self.trace {
            let song = self.mode == PlayMode::Song;
            trace.played(self.now, step, song, self.pattern);
        }
    }

    /// Stops the pattern played over the song, and the notes it started.
    fn end_overlay(&mut self) {
        if self.overlay.take().is_none() {
            return;
        }
        store_overlay(&self.overlay_playhead, None);
        self.pending.retain(|note| !note.origin.over_song);
        for channel in &mut self.channels {
            channel
                .instrument
                .release_unless(|origin| !origin.over_song);
        }
        if let Some(trace) = &mut self.trace {
            trace.cut(self.now, None, |origin| !origin.over_song);
        }
    }

    /// Keeps the pattern played over the song on the same pattern when
    /// patterns are added or removed before it: `moved` gives a pattern's
    /// new number, or `None` for the one removed, which stops it.
    fn overlay_moved(&mut self, moved: impl Fn(usize) -> Option<usize>) {
        let Some(overlay) = &mut self.overlay else {
            return;
        };
        overlay.playing = overlay
            .playing
            .and_then(|(pattern, step)| Some((moved(pattern)?, step)));
        overlay.coming = overlay.coming.and_then(&moved);
        if overlay.playing.is_none() && overlay.coming.is_none() {
            self.end_overlay();
        }
    }

    /// Moves the pattern played over the song on to song step `step`, and
    /// gives the pattern and the step of it to play there, if any.
    fn overlay_step(&mut self, step: usize) -> Option<(usize, usize)> {
        let bar_line = click_at(&self.song_clicks, step) == Click::Bar;
        let overlay = self.overlay.as_mut()?;
        // After a jump it waits for a bar line, to start again from its
        // top in time with the bars.
        if overlay.last.is_some_and(|last| step != last + 1)
            && let Some((pattern, _)) = overlay.playing.take()
        {
            overlay.coming.get_or_insert(pattern);
        }
        overlay.last = Some(step);
        if bar_line && let Some(pattern) = overlay.coming.take() {
            overlay.playing = Some((pattern, 0));
        }
        let (pattern, at) = overlay.playing?;
        let length = self.pattern_lengths.get(pattern).copied().unwrap_or(0);
        if length == 0 {
            return None;
        }
        let at = at % length;
        overlay.playing = Some((pattern, at + 1));
        Some((pattern, at))
    }

    /// Puts every volume at its target at once. Smoothing avoids clicks
    /// while sound is playing; before an export starts, it would only
    /// make the first moment too loud or too quiet.
    pub(crate) fn settle_volumes(&mut self) {
        self.master_volume.settle();
        self.reverb.settle();
        for channel in &mut self.channels {
            channel.volume.settle();
            channel.pan.settle();
            channel.reverb_send.settle();
        }
    }

    /// Changes the tempo, and stretches the notes that are sounding to
    /// match, so each still ends on the step it should.
    fn set_tempo(&mut self, tempo: Tempo) {
        self.tempo_now = tempo;
        let before = self.sequencer.step_frames();
        let next_step = self.sequencer.frames_to_next_step();
        self.sequencer.set_tempo(tempo);
        let stretch = self.sequencer.step_frames() / before;
        for channel in &mut self.channels {
            channel.instrument.tempo_changed(next_step, stretch);
            channel.effects.set_tempo(tempo);
        }
        if let Some(trace) = &mut self.trace {
            trace.tempo_changed(self.now, next_step, stretch);
        }
    }

    /// Stops the notes that the new song, just received, no longer plays.
    /// A note is still played if a piece of the new song plays its pattern
    /// on the song step where the note started, at the same step of the
    /// pattern: then nothing changed under it. Otherwise its clip was moved
    /// or taken away, or its track was muted.
    fn release_notes_left_out(&mut self) {
        // The fields are borrowed one by one, so the closure can read the
        // song while the channels are changed.
        let plays = &self.plays;
        let lengths = &self.pattern_lengths;
        let still_played = |origin: Origin| {
            if origin.over_song {
                return true;
            }
            let length = lengths.get(origin.pattern).copied().unwrap_or(0);
            plays.iter().any(|play| {
                play.pattern == origin.pattern
                    && play.pattern_step(origin.song_step, length) == Some(origin.pattern_step)
            })
        };
        for channel in &mut self.channels {
            channel.instrument.release_unless(still_played);
        }
        if let Some(trace) = &mut self.trace {
            trace.cut(self.now, None, still_played);
        }
    }

    /// The tempo step `step` of the loop plays at: the song's own for
    /// that step, if its tempo changes, or else the tempo the user set.
    fn tempo_of_step(&self, step: usize) -> Tempo {
        match self.mode {
            PlayMode::Song => self
                .song_tempos
                .get(step)
                .or(self.song_tempos.last())
                .copied()
                .unwrap_or(self.tempo),
            PlayMode::Pattern => self.tempo,
        }
    }

    /// Starts playing from `start`, or, with the count-in on, starts
    /// counting in first: the whole bar `start` is in, then on up to
    /// `start`, so the beats run on into the music without a jump. From
    /// beat 3 of a 4/4 bar that is "1 2 3 4 1 2", and the music starts on 3.
    fn begin_play(&mut self) {
        if !self.count_in {
            self.sequencer.start_at(self.start);
            self.playing = true;
            self.trace_play(self.start);
            self.resume_long_sounds(self.start);
            return;
        }
        if self.playing {
            self.stop();
        }
        let (bar, length) = count_in_bar(self.loop_clicks(), self.start);
        let length = length.max(1);
        let into_the_bar = self.start.saturating_sub(bar) % length;
        self.counter.set_tempo(self.tempo_of_step(self.start));
        self.counter.reset();
        self.counting = Some(CountIn {
            bar,
            length,
            steps: length + into_the_bar,
            counted: 0,
        });
    }

    /// What the metronome plays on each step of the loop.
    fn loop_clicks(&self) -> &[Click] {
        match self.mode {
            PlayMode::Pattern => &self.pattern_clicks,
            PlayMode::Song => &self.song_clicks,
        }
    }

    /// Moves the count-in on by one frame, clicking when a step of it
    /// starts. When the count is over, playback starts, on this same
    /// frame.
    fn count_in_frame(&mut self, mut count: CountIn) {
        if self.counter.tick().is_none() {
            return;
        }
        if count.counted == count.steps {
            self.counting = None;
            self.sequencer.start_at(self.start);
            self.playing = true;
            self.trace_play(self.start);
            self.resume_long_sounds(self.start);
            return;
        }
        let step = count.bar + count.counted % count.length;
        self.click.strike(click_at(self.loop_clicks(), step));
        count.counted += 1;
        self.counting = Some(count);
    }

    fn stop(&mut self) {
        if self.playing
            && let Some(trace) = &mut self.trace
        {
            trace.stopped(self.now);
        }
        self.playing = false;
        self.counting = None;
        self.pending.clear();
        self.playhead.store(NO_STEP, Ordering::Relaxed);
        self.position.store(NO_POSITION, Ordering::Relaxed);
        // This lets go of the keys that are down too.
        for channel in &mut self.channels {
            channel.instrument.release_all();
        }
        self.live_keys = [None; 128];
    }

    /// Plays a key from a keyboard (typing or MIDI) on the live channel.
    ///
    /// Live notes play like a preview: on a muted channel too, and
    /// whether or not the sequencer is playing.
    fn play_live(&mut self, message: MidiMessage) {
        match message {
            MidiMessage::NoteOn {
                pitch, velocity, ..
            } => {
                // A key that is already down (on both keyboards at once,
                // say) comes up first.
                self.release_live(pitch);
                if let Some(channel) = self.live_channel
                    && let Some(state) = self.channels.get_mut(channel)
                {
                    state
                        .instrument
                        .note_on(pitch, velocity, HELD, self.sample_rate);
                    self.live_keys[usize::from(pitch.midi())] = Some(channel);
                }
            }
            MidiMessage::NoteOff { pitch, .. } => self.release_live(pitch),
            MidiMessage::ProgramChange { .. } => {}
        }
    }

    /// Lets go of a key, on the channel it went down on.
    fn release_live(&mut self, pitch: Note) {
        if let Some(channel) = self.live_keys[usize::from(pitch.midi())].take()
            && let Some(state) = self.channels.get_mut(channel)
        {
            state.instrument.note_off(pitch);
        }
    }

    /// Lets go of every key that is down.
    fn release_live_keys(&mut self) {
        for midi in 0..=127 {
            self.release_live(Note::new(midi));
        }
    }

    /// Passes on the messages MIDI Out channels made during this block.
    fn send_midi(&mut self) {
        // Three different fields, borrowed apart so the closure can use
        // two of them while the channels are changed.
        let midi_out = &mut self.midi_out;
        let backlog = &mut self.midi_backlog;
        for channel in &mut self.channels {
            channel
                .instrument
                .send_midi(|message| queue_midi(midi_out, backlog, message));
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

    /// Starts every note that begins on `step` of the loop.
    ///
    /// In pattern mode, `step` is a step of the selected pattern. In song
    /// mode, it is a step of the song, and each piece playing then is asked
    /// for the notes at its own step: a piece that started on song step 32
    /// plays its pattern's step 2 on song step 34, and step 2 again on
    /// song step 50 if it repeats a 16-step pattern. The pattern played
    /// over the song, if any, plays its own next step too.
    fn start_notes(&mut self, step: usize) {
        let timing = Timing {
            step_frames: self.sequencer.step_frames(),
            sample_rate: self.sample_rate,
            loose_frames: self.loose_frames,
            now: self.now,
        };
        match self.mode {
            PlayMode::Pattern => {
                let length = self.pattern_length(self.pattern);
                start_pattern_notes(
                    &mut self.channels,
                    Origin {
                        pattern: self.pattern,
                        pattern_step: step,
                        song_step: step,
                        over_song: false,
                    },
                    length - step.min(length),
                    timing,
                    &mut self.pending,
                    self.trace.as_deref_mut(),
                );
            }
            PlayMode::Song => {
                // `self.plays` is read while `self.channels` is changed: two
                // different fields, so the borrows do not overlap.
                for play in &self.plays {
                    let length = self.pattern_lengths.get(play.pattern).copied().unwrap_or(0);
                    if let Some(local_step) = play.pattern_step(step, length) {
                        // Notes stop at the end of the pattern, or of the
                        // piece, whichever comes first.
                        let room = (length - local_step).min(play.end() - step);
                        start_pattern_notes(
                            &mut self.channels,
                            Origin {
                                pattern: play.pattern,
                                pattern_step: local_step,
                                song_step: step,
                                over_song: false,
                            },
                            room,
                            timing,
                            &mut self.pending,
                            self.trace.as_deref_mut(),
                        );
                    }
                }
                let overlay = self.overlay_step(step);
                store_overlay(&self.overlay_playhead, overlay);
                if let Some((pattern, at)) = overlay {
                    let length = self.pattern_length(pattern);
                    start_pattern_notes(
                        &mut self.channels,
                        Origin {
                            pattern,
                            pattern_step: at,
                            song_step: step,
                            over_song: true,
                        },
                        length - at,
                        timing,
                        &mut self.pending,
                        self.trace.as_deref_mut(),
                    );
                }
            }
        }
    }

    /// Applies waiting commands and keys, then fills `output` with samples.
    pub(crate) fn process<T>(&mut self, output: &mut [T])
    where
        T: SizedSample + FromSample<f32>,
    {
        self.flush_trash();
        self.flush_midi_backlog();
        while let Ok(command) = self.commands.pop() {
            self.apply(command);
        }
        // `as_mut` turns `&mut Option<Consumer>` into `Option<&mut Consumer>`,
        // and `and_then` asks it for a key only if there is a consumer.
        while let Some(message) = self.live_input.as_mut().and_then(|keys| keys.pop().ok()) {
            self.play_live(message);
        }
        self.write_heard();

        let mut master_peak = [0.0f32; 2];
        for frame in output.chunks_mut(self.output_channels) {
            if let Some(trace) = &mut self.trace {
                trace.frame(self.now);
            }
            if let Some(count) = self.counting {
                self.count_in_frame(count);
            }
            if self.playing {
                let lap = self.sequencer.due().is_some() && self.sequencer.comes_round();
                // A step about to start plays at its own tempo, which is
                // set before it starts, so it lasts as long as it should.
                if let Some(step) = self.sequencer.due() {
                    if self.sequencer.comes_round()
                        && let Some(tempos) = self.lap_tempos.take()
                    {
                        self.tempo = tempos.tempo;
                        let old = std::mem::replace(&mut self.song_tempos, tempos.song);
                        self.throw_away(Garbage::Tempos(old));
                    }
                    let tempo = self.tempo_of_step(step);
                    if tempo != self.tempo_now {
                        self.set_tempo(tempo);
                    }
                }
                if let Some(step) = self.sequencer.tick() {
                    self.playhead.store(step, Ordering::Relaxed);
                    self.laps.store(self.sequencer.laps(), Ordering::Relaxed);
                    if let Some(trace) = &mut self.trace {
                        trace.step(self.now, step, self.tempo_now);
                    }
                    // Round again, a recording starts again with the loop.
                    if lap && self.mode == PlayMode::Song {
                        for channel in &mut self.channels {
                            channel.instrument.release_long();
                        }
                    }
                    self.start_notes(step);
                    if lap {
                        self.resume_long_sounds(step);
                    }
                    if self.metronome {
                        self.click.strike(click_at(self.loop_clicks(), step));
                    }
                }
            }
            if !self.pending.is_empty() {
                self.start_pending();
            }
            self.now += 1;

            // Each channel goes through its own effects first. Then it
            // goes to the speakers as its pan says, and as
            // much of it as its send says to the reverb, which plays only
            // the room: what goes in is taken out of the dry sound in
            // the reverb's wet / dry mix, so a channel with all of it in
            // sounds as the whole song did before channels had sends,
            // and one with none plays dry, as with the reverb off.
            let wet = self.reverb.wet();
            let mut dry = [0.0; 2];
            let mut send = 0.0;
            for channel in &mut self.channels {
                let volume = channel.volume.next_value();
                let sample = channel.effects.process(channel.instrument.next_sample()) * volume;
                channel.peak = channel.peak.max(sample.abs());
                let amount = channel.reverb_send.next_value();
                let gains = lineup_core::Channel::pan_gains(channel.pan.next_value());
                let kept = sample * (1.0 - wet * amount);
                dry[0] += kept * gains[0];
                dry[1] += kept * gains[1];
                send += sample * amount;
            }
            if let Some(phrase) = &mut self.phrase {
                let sample = phrase.next_sample();
                dry = dry.map(|side| side + sample * (1.0 - wet));
                send += sample;
            }
            // The metronome's clicks are added after the reverb: the room
            // is the music's, and a click with a tail is harder to follow.
            let click = self.click.next_sample();
            let stereo = self
                .reverb
                .next(dry, send, wet)
                .map(|sample| sample + click);
            let volume = self.master_volume.next_value();
            let stereo = stereo.map(|sample| sample * volume);
            master_peak = [0, 1].map(|side| master_peak[side].max(stereo[side].abs()));
            // `tanh` gently rounds off peaks above 1.0 instead of clipping them hard.
            let stereo = stereo.map(f32::tanh);
            if frame.len() == 1 {
                frame[0] = T::from_sample((stereo[0] + stereo[1]) * 0.5);
            } else {
                for (index, sample) in frame.iter_mut().enumerate() {
                    *sample = T::from_sample(stereo[index % 2]);
                }
            }
        }

        for (index, channel) in self.channels.iter_mut().enumerate() {
            self.levels
                .raise_channel(index, std::mem::take(&mut channel.peak));
        }
        self.levels.raise_master(master_peak);

        if self.phrase.as_ref().is_some_and(|phrase| phrase.is_over())
            && let Some(over) = self.phrase.take()
        {
            self.throw_away(Garbage::Phrase(over));
        }
        self.send_midi();
        if self.playing {
            let position = self.sequencer.position().to_bits();
            self.position.store(position, Ordering::Relaxed);
        }
    }
}

impl AudioThread {
    /// Starts the notes whose wait is over.
    fn start_pending(&mut self) {
        let mut index = 0;
        while index < self.pending.len() {
            let note = self.pending[index];
            if note.at <= self.now {
                // `swap_remove` puts the last note in this one's place
                // instead of moving them all down: the order of the
                // waiting notes does not matter.
                self.pending.swap_remove(index);
                if let Some(channel) = self.channels.get_mut(note.channel) {
                    channel.instrument.note_on_from(
                        note.origin,
                        note.pitch,
                        note.velocity,
                        note.gate,
                        self.sample_rate,
                    );
                    if let Some(trace) = &mut self.trace {
                        trace.note_on(
                            self.now,
                            note.channel,
                            note.origin,
                            note.pitch,
                            note.velocity,
                            note.gate,
                            note.late,
                            one_per_pitch(&channel.instrument),
                        );
                    }
                }
            } else {
                index += 1;
            }
        }
    }
}

/// A note of the song waiting to start, a little late, with loose timing.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Pending {
    channel: usize,
    origin: Origin,
    pitch: Note,
    velocity: u8,
    gate: u32,
    /// The frame it starts on, counted on [`AudioThread::now`]'s clock.
    at: u64,
    /// How many frames after its step that is, for a trace.
    late: u32,
}

/// A number from 0 to 1 that looks random but is always the same for
/// the same `seed`: the "SplitMix64" mixer, which scrambles the bits of
/// a number so that seeds next to each other give unrelated results.
///
/// Loose timing uses it instead of a real random number so a song plays
/// the same way every time, and a WAV export sounds like playback.
fn scatter(seed: u64) -> f64 {
    let mut x = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    // The top 53 bits, as many as an `f64` holds exactly.
    (x >> 11) as f64 / (1u64 << 53) as f64
}

/// How many frames late a note starts, with loose timing of up to
/// `loose_frames`. A player is late or early by the same amount for all
/// the notes of a chord, give or take a little, so most of the delay is
/// the same for every note a channel starts on a step, and a fifth of
/// it differs from note to note.
fn delay(loose_frames: f64, channel: usize, song_step: usize, pitch: Note) -> f64 {
    let onset = ((song_step as u64) << 8) | channel as u64;
    let together = scatter(onset);
    let apart = scatter(onset ^ (u64::from(pitch.midi()) << 40));
    loose_frames * (0.8 * together + 0.2 * apart)
}

/// How many frames the steps of the loop from `from` up to `to` last, at
/// `timing`'s tempos: the song's, one a step and the last going on, or
/// else its single tempo, at its sample rate. A step before the loop
/// (below zero) lasts as long as the loop's first.
fn frames_between(timing: (&[Tempo], Tempo, u32), from: isize, to: usize) -> f64 {
    let (tempos, tempo, sample_rate) = timing;
    let tempo_at = |step: usize| tempos.get(step).or(tempos.last()).copied().unwrap_or(tempo);
    (from..to as isize)
        .map(|step| frames_per_step(tempo_at(step.max(0) as usize), sample_rate))
        .sum()
}

/// Whether an instrument plays one of each note at a time, so that a note
/// starting again ends the one sounding: a MIDI Out, and a SoundFont,
/// which plays its notes the same way.
fn one_per_pitch(instrument: &Instrument) -> bool {
    matches!(
        instrument,
        Instrument::MidiOut(_) | Instrument::SoundFont(_)
    )
}

/// Ends the notes an instrument has playing on an external synth, before
/// the instrument is thrown away. A synth only stops a note when it is
/// told to, so without this the notes would hang there for ever.
///
/// A plain function, so the caller can pass a field of the audio thread
/// while it holds an instrument taken out of another one.
fn silence(
    instrument: &mut Instrument,
    midi_out: &mut Option<rtrb::Producer<MidiMessage>>,
    backlog: &mut VecDeque<MidiMessage>,
) {
    instrument.release_all();
    instrument.send_midi(|message| queue_midi(midi_out, backlog, message));
}

/// Adds `message` to the MIDI Out queue, or keeps it in `backlog` for the
/// next block if the queue is full. A message lost here can be a note off
/// that never stops a note on the external synth.
fn queue_midi(
    midi_out: &mut Option<rtrb::Producer<MidiMessage>>,
    backlog: &mut VecDeque<MidiMessage>,
    mut message: MidiMessage,
) {
    let Some(out) = midi_out else {
        return;
    };
    // Messages already waiting go first: a note on sent past a waiting
    // note off for the same key would be cut off by it.
    if backlog.is_empty() {
        match out.push(message) {
            Ok(()) => return,
            Err(rtrb::PushError::Full(returned)) => message = returned,
        }
    }
    if backlog.len() < MIDI_BACKLOG {
        backlog.push_back(message);
    }
    // A full backlog too means the port has been stuck for hundreds of
    // messages, and there is nowhere left to keep this one.
}

/// How long a step is, the sample rate the instruments run at, how
/// loose the timing is, and the frame the step starts on.
#[derive(Debug, Clone, Copy)]
struct Timing {
    step_frames: f64,
    sample_rate: u32,
    loose_frames: f64,
    now: u64,
}

/// Starts the notes of one pattern that begin on one of its steps, as
/// `origin` says: which pattern, which step of it, and on which step of
/// the song. A note is held for `room` steps at most: up to the end of the
/// pattern, or of the piece of the song that plays it.
///
/// With loose timing, a note that should start a little late goes to
/// `pending` instead, to start when its time comes.
///
/// Each note that starts is written down in `trace`, if a trace is on.
///
/// This is a plain function, not a method, so that it borrows only the
/// channels. Its caller can keep reading other fields of the audio thread
/// (such as the clips) while the channels are being changed.
fn start_pattern_notes(
    channels: &mut [ChannelState],
    origin: Origin,
    room: usize,
    timing: Timing,
    pending: &mut Vec<Pending>,
    mut trace: Option<&mut Tracer>,
) {
    for (index, channel) in channels.iter_mut().enumerate() {
        if channel.muted {
            continue;
        }
        // `channel.notes` is borrowed to read while `channel.instrument`
        // is borrowed to change. That is allowed: they are different
        // fields of the struct, so the borrows do not overlap.
        for note in channel
            .notes
            .get(origin.pattern)
            .iter()
            .filter(|note| note.start == origin.pattern_step)
        {
            // A note that runs past the end of the pattern is cut there,
            // on the grid; otherwise it ends `trim` ticks before its last
            // step does. A note `nudge` ticks off the grid starts that
            // much late, as does each note with loose timing.
            let tick = timing.step_frames / TICKS_PER_STEP as f64;
            let held = if note.length <= room {
                note.length as f64 * timing.step_frames - f64::from(note.nudge + note.trim) * tick
            } else {
                room as f64 * timing.step_frames - f64::from(note.nudge) * tick
            };
            let gate = held as u32;
            let late = (f64::from(note.nudge) * tick
                + delay(timing.loose_frames, index, origin.song_step, note.pitch))
                as u64;
            // A full list never grows (that would allocate): the note
            // then starts on time.
            if late > 0 && pending.len() < pending.capacity() {
                pending.push(Pending {
                    channel: index,
                    origin,
                    pitch: note.pitch,
                    velocity: note.velocity,
                    gate,
                    at: timing.now + late,
                    late: late.min(u64::from(u32::MAX)) as u32,
                });
                continue;
            }
            channel.instrument.note_on_from(
                origin,
                note.pitch,
                note.velocity,
                gate,
                timing.sample_rate,
            );
            if let Some(trace) = trace.as_deref_mut() {
                trace.note_on(
                    timing.now,
                    index,
                    origin,
                    note.pitch,
                    note.velocity,
                    gate,
                    0,
                    one_per_pitch(&channel.instrument),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instrument::{MidiOut, Sampler, Synth};
    use crate::sample::SampleData;
    use lineup_core::{Channel, Clip, Project, Section};

    const RATE: u32 = 48_000;

    struct Harness {
        audio: AudioThread,
        commands: rtrb::Producer<Command>,
        trash: rtrb::Consumer<Garbage>,
        playhead: Arc<AtomicUsize>,
        position: Arc<AtomicU64>,
    }

    /// An audio thread with one empty pattern, 16 steps long, at 120 BPM.
    fn harness() -> Harness {
        let (commands, command_consumer) = rtrb::RingBuffer::new(64);
        let (trash_producer, trash) = rtrb::RingBuffer::new(64);
        let playhead = Arc::new(AtomicUsize::new(NO_STEP));
        let position = Arc::new(AtomicU64::new(NO_POSITION));
        let audio = AudioThread::new(
            RATE,
            1,
            command_consumer,
            trash_producer,
            playhead.clone(),
            position.clone(),
        );
        let mut h = Harness {
            audio,
            commands,
            trash,
            playhead,
            position,
        };
        send(
            &mut h,
            Command::InsertPattern {
                index: 0,
                length: 16,
            },
        );
        send(&mut h, Command::SetTempo(Tempo::new(120.0)));
        h
    }

    fn send(h: &mut Harness, command: Command) {
        h.commands.push(command).unwrap();
    }

    /// At 120 BPM and 48 kHz, one step lasts 6000 frames.
    const STEP: usize = 6_000;

    /// An index past the end, to insert a channel at the bottom.
    const AT_END: usize = usize::MAX;

    /// A sampler whose sound is a constant 0.5 for 10 frames.
    fn click() -> Instrument {
        Instrument::Sampler(Sampler::new(Arc::new(SampleData::new(vec![0.5; 10], RATE))))
    }

    fn steps(on: &[usize]) -> Vec<NoteEvent> {
        on.iter().map(|&step| NoteEvent::step(step)).collect()
    }

    /// Adds a click channel with the given steps on in the first pattern.
    fn add(h: &mut Harness, on: &[usize], muted: bool) {
        send(
            h,
            Command::InsertChannel {
                index: AT_END,
                instrument: click(),
                notes: ChannelNotes::new([steps(on)]),
                volume: 1.0,
                muted,
            },
        );
    }

    /// Adds a synth channel playing the given notes in the first pattern.
    fn add_synth(h: &mut Harness, notes: &[NoteEvent]) {
        send(
            h,
            Command::InsertChannel {
                index: AT_END,
                instrument: Instrument::Synth(Synth::new(RATE)),
                notes: ChannelNotes::new([notes.to_vec()]),
                volume: 1.0,
                muted: false,
            },
        );
    }

    /// The frames, in a block, where a new sound starts after silence.
    fn onsets(out: &[f32]) -> Vec<usize> {
        (0..out.len())
            .filter(|&i| out[i] != 0.0 && (i == 0 || out[i - 1] == 0.0))
            .collect()
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
        send(&mut h, Command::Play);

        // At 120 BPM a step lasts 6000 frames: hits at frame 0 and 6000.
        let out = render(&mut h, 7_000);
        assert!(out[0] != 0.0, "step 0 should sound at once");
        assert!(out[100..6_000].iter().all(|&s| s == 0.0));
        assert!(out[6_000] != 0.0, "step 1 should sound at frame 6000");
        assert_eq!(h.playhead.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn play_starts_where_seek_put_it() {
        let mut h = harness();
        add(&mut h, &[0, 2], false);
        send(&mut h, Command::Seek(2));
        send(&mut h, Command::Play);
        let out = render(&mut h, 100);
        assert!(out[0] != 0.0, "step 2 should sound at once");
        assert_eq!(h.playhead.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn laps_count_the_times_the_loop_came_round() {
        let mut h = harness();
        send(&mut h, Command::Play);
        render(&mut h, 16 * STEP);
        assert_eq!(h.audio.laps.load(Ordering::Relaxed), 0);
        render(&mut h, 1);
        assert_eq!(h.audio.laps.load(Ordering::Relaxed), 1);
        // A jump back to the start is not a lap.
        render(&mut h, 8 * STEP);
        send(&mut h, Command::Seek(0));
        render(&mut h, 12 * STEP);
        assert_eq!(h.audio.laps.load(Ordering::Relaxed), 1);
        render(&mut h, 4 * STEP + 1);
        assert_eq!(h.audio.laps.load(Ordering::Relaxed), 2);
    }

    fn lap_tempos(bpm: f32) -> Command {
        Command::SetLapTempos(Some(LapTempos {
            tempo: Tempo::new(bpm),
            song: Vec::new(),
        }))
    }

    #[test]
    fn lap_tempos_start_on_the_loops_first_step() {
        let mut h = harness();
        send(&mut h, Command::Play);
        send(&mut h, lap_tempos(240.0));
        // The pass playing keeps its tempo: 16 steps of 6000 frames.
        render(&mut h, 16 * STEP);
        assert_eq!(h.playhead.load(Ordering::Relaxed), 15);
        // The next starts at twice the speed, on its very first step.
        render(&mut h, 1);
        assert_eq!(h.audio.tempo_now, Tempo::new(240.0));
        render(&mut h, STEP / 2);
        assert_eq!(h.playhead.load(Ordering::Relaxed), 1);
        // It happens once.
        render(&mut h, 15 * STEP / 2);
        assert_eq!(h.audio.laps.load(Ordering::Relaxed), 2);
        assert_eq!(h.audio.tempo_now, Tempo::new(240.0));
        // Tempos taken back are not played; the lists left go to the UI
        // thread to be freed.
        send(&mut h, lap_tempos(60.0));
        send(&mut h, Command::SetLapTempos(None));
        render(&mut h, 16 * STEP / 2);
        assert_eq!(h.audio.tempo_now, Tempo::new(240.0));
        let mut freed = 0;
        while let Ok(garbage) = h.trash.pop() {
            assert!(matches!(garbage, Garbage::Tempos(_)));
            freed += 1;
        }
        assert_eq!(freed, 2, "the song's list before, and the list taken back");
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
        send(&mut h, Command::Play);
        render(&mut h, 100);
        send(
            &mut h,
            Command::SetNotes {
                pattern: 0,
                channel: 0,
                notes: steps(&[1]),
            },
        );
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
        send(
            &mut h,
            Command::SetNotes {
                pattern: 0,
                channel: 0,
                notes: steps(&[0]),
            },
        );
        send(&mut h, Command::RemoveChannel(0));
        render(&mut h, 10);
        let (mut instruments, mut note_lists, mut channels) = (0, 0, 0);
        while let Ok(garbage) = h.trash.pop() {
            match garbage {
                Garbage::Instrument(_) => instruments += 1,
                Garbage::Notes(_) => note_lists += 1,
                Garbage::Channel(_) => channels += 1,
                Garbage::Song(_)
                | Garbage::Tempos(_)
                | Garbage::Clicks(_)
                | Garbage::Phrase(_)
                | Garbage::Trace(_)
                | Garbage::DelayLine(_) => {}
            }
        }
        assert_eq!(instruments, 1, "the replaced instrument");
        assert_eq!(note_lists, 1, "the replaced notes");
        assert_eq!(
            channels, 1,
            "the removed channel, with its instrument and notes"
        );
    }

    #[test]
    fn notes_are_held_for_their_length() {
        let mut h = harness();
        add_synth(&mut h, &[NoteEvent::new(Note::A4, 0, 2)]);
        send(&mut h, Command::Play);
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
        add_synth(&mut h, &chord);
        send(&mut h, Command::Play);
        render(&mut h, 10);
        assert_eq!(h.audio.channels[0].instrument.voices_playing(), 3);
    }

    #[test]
    fn stopping_releases_held_notes() {
        let mut h = harness();
        add_synth(&mut h, &[NoteEvent::new(Note::A4, 0, 16)]);
        send(&mut h, Command::Play);
        render(&mut h, 1_000);
        h.commands.push(Command::Stop).unwrap();
        render(&mut h, 48_000);
        assert_eq!(h.audio.channels[0].instrument.voices_playing(), 0);
    }

    #[test]
    fn switching_between_pattern_and_song_releases_held_notes() {
        let mut h = harness();
        add_synth(&mut h, &[NoteEvent::new(Note::A4, 0, 16)]);
        send(&mut h, Command::Play);
        render(&mut h, 1_000);
        assert!(h.audio.channels[0].instrument.voices_playing() > 0);
        // The song is empty: nothing plays in it.
        h.commands.push(Command::SetMode(PlayMode::Song)).unwrap();
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

    fn phrase(start: f64) -> Box<Phrase> {
        let note = crate::PhraseNote {
            instrument: 0,
            start,
            length: 0.05,
            pitch: Note::C4,
            velocity: 100,
        };
        Box::new(Phrase::new(vec![click()], &[note], RATE))
    }

    #[test]
    fn a_phrase_plays_without_the_song_and_is_thrown_away_after() {
        let mut h = harness();
        send(&mut h, Command::PlayPhrase(phrase(0.01)));
        let out = render(&mut h, 1000);
        assert_eq!(onsets(&out), [480]);
        // It rings out for two seconds, then goes back to be freed.
        render(&mut h, 2 * RATE as usize);
        assert!(h.trash.pop().is_err());
        render(&mut h, RATE as usize / 10);
        assert!(matches!(h.trash.pop(), Ok(Garbage::Phrase(_))));
        assert!(h.audio.phrase.is_none());
    }

    #[test]
    fn a_new_phrase_takes_the_place_of_the_last() {
        let mut h = harness();
        send(&mut h, Command::PlayPhrase(phrase(0.5)));
        send(&mut h, Command::PlayPhrase(phrase(0.01)));
        let out = render(&mut h, RATE as usize);
        assert_eq!(onsets(&out), [480]);
        assert!(matches!(h.trash.pop(), Ok(Garbage::Phrase(_))));
    }

    #[test]
    fn a_stopped_phrase_plays_nothing_more() {
        let mut h = harness();
        send(&mut h, Command::PlayPhrase(phrase(0.5)));
        render(&mut h, 100);
        send(&mut h, Command::StopPhrase);
        assert!(onsets(&render(&mut h, RATE as usize)).is_empty());
    }

    /// Two patterns: the click plays step 0 of pattern 0 and step 2 of pattern 1.
    fn two_patterns() -> Harness {
        let mut h = harness();
        send(
            &mut h,
            Command::InsertPattern {
                index: 1,
                length: 16,
            },
        );
        send(
            &mut h,
            Command::InsertChannel {
                index: 0,
                instrument: click(),
                notes: ChannelNotes::new([steps(&[0]), steps(&[2])]),
                volume: 1.0,
                muted: false,
            },
        );
        h
    }

    #[test]
    fn pattern_mode_plays_the_selected_pattern() {
        let mut h = two_patterns();
        send(&mut h, Command::SelectPattern(1));
        send(&mut h, Command::Play);
        let out = render(&mut h, 16 * STEP);
        assert_eq!(onsets(&out), vec![2 * STEP]);
    }

    #[test]
    fn song_mode_plays_the_clips() {
        let mut h = two_patterns();
        // Pattern 1 at bar 1, pattern 0 at bar 2: a two-bar song.
        let plays = vec![whole(1, 0), whole(0, 16)];
        send(&mut h, Command::SetSong { plays, length: 32 });
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        let out = render(&mut h, 40 * STEP);
        // Step 2 of the first clip, step 0 of the second, then the song
        // loops back to the start: step 2 of the first clip again.
        assert_eq!(onsets(&out), vec![2 * STEP, 16 * STEP, 34 * STEP]);
        assert_eq!(
            h.playhead.load(Ordering::Relaxed),
            7,
            "step 39 is step 7 of the second time round"
        );
    }

    /// Two patterns (see [`two_patterns`]) and a song of three silent
    /// bars, playing: what plays is the pattern played over it.
    fn silent_song() -> Harness {
        let mut h = two_patterns();
        send(
            &mut h,
            Command::SetSong {
                plays: Vec::new(),
                length: 48,
            },
        );
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        h
    }

    /// The onsets of `frames` more frames, counted from the song's start
    /// when `before` frames have played already.
    fn onsets_from(h: &mut Harness, before: usize, frames: usize) -> Vec<usize> {
        onsets(&render(h, frames))
            .into_iter()
            .map(|frame| before + frame)
            .collect()
    }

    #[test]
    fn a_pattern_over_the_song_starts_on_the_next_bar_and_loops() {
        let mut h = silent_song();
        assert!(onsets(&render(&mut h, 3 * STEP)).is_empty());
        send(&mut h, Command::SetOverlay(Some(1)));
        // Pattern 1 plays its step 2 from bar 2 on: song steps 18 and 34.
        assert_eq!(
            onsets_from(&mut h, 3 * STEP, 17 * STEP),
            vec![18 * STEP],
            "nothing before the bar line"
        );
        assert_eq!(load_overlay(&h.audio.overlay_playhead), Some((1, 3)));
        assert_eq!(onsets_from(&mut h, 20 * STEP, 16 * STEP), vec![34 * STEP]);
        send(&mut h, Command::SetOverlay(None));
        assert!(onsets(&render(&mut h, 16 * STEP)).is_empty());
        assert_eq!(load_overlay(&h.audio.overlay_playhead), None);
    }

    /// The metronome's clicks for `steps` steps of 3/4: bars of 12 steps.
    fn three_four(steps: usize) -> Vec<Click> {
        (0..steps)
            .map(|step| match step % 12 {
                0 => Click::Bar,
                beat if beat % 4 == 0 => Click::Beat,
                _ => Click::None,
            })
            .collect()
    }

    #[test]
    fn a_pattern_over_the_song_follows_its_bars() {
        let mut h = silent_song();
        send(&mut h, Command::SetSongClicks(three_four(48)));
        render(&mut h, STEP);
        send(&mut h, Command::SetOverlay(Some(1)));
        assert_eq!(onsets_from(&mut h, STEP, 20 * STEP), vec![14 * STEP]);
    }

    #[test]
    fn another_pattern_over_the_song_waits_for_the_bar_line() {
        let mut h = silent_song();
        send(&mut h, Command::SetOverlay(Some(1)));
        render(&mut h, 20 * STEP);
        send(&mut h, Command::SetOverlay(Some(0)));
        // Pattern 1 goes on to the end of bar 2 (its step 2 is already
        // past), then pattern 0 plays its step 0 on bar 3.
        assert_eq!(onsets_from(&mut h, 20 * STEP, 20 * STEP), vec![32 * STEP]);
        // Asking for the pattern already playing changes nothing.
        send(&mut h, Command::SetOverlay(Some(0)));
        assert!(onsets(&render(&mut h, 8 * STEP)).is_empty());
        assert_eq!(onsets_from(&mut h, 48 * STEP, STEP), vec![48 * STEP]);
    }

    #[test]
    fn a_pattern_over_the_song_starts_again_after_a_jump() {
        let mut h = silent_song();
        send(&mut h, Command::SetOverlay(Some(1)));
        render(&mut h, 20 * STEP);
        // Back to step 5: it waits for bar 2 to start from its top.
        send(&mut h, Command::Seek(5));
        assert_eq!(onsets_from(&mut h, 5 * STEP, 16 * STEP), vec![18 * STEP]);
    }

    #[test]
    fn a_pattern_over_the_song_starts_again_when_the_song_does() {
        let mut h = silent_song();
        send(&mut h, Command::SetSongClicks(three_four(48)));
        render(&mut h, STEP);
        send(&mut h, Command::SetOverlay(Some(1)));
        // From bar 2 (step 12) its step 2 plays on steps 14, 30 and 46.
        // Then the song goes back round from step 47 to step 0, where it
        // starts again from its top instead of going on from its step 4.
        assert_eq!(
            onsets_from(&mut h, STEP, 51 * STEP),
            vec![14 * STEP, 30 * STEP, 46 * STEP, 50 * STEP]
        );
    }

    #[test]
    fn a_pattern_over_the_song_keeps_to_its_pattern_as_patterns_move() {
        let mut h = silent_song();
        send(&mut h, Command::SetOverlay(Some(1)));
        render(&mut h, 17 * STEP);
        // A pattern added before it: pattern 1 becomes pattern 2.
        send(
            &mut h,
            Command::InsertPattern {
                index: 0,
                length: 16,
            },
        );
        assert_eq!(onsets_from(&mut h, 17 * STEP, 2 * STEP), vec![18 * STEP]);
        assert_eq!(load_overlay(&h.audio.overlay_playhead), Some((2, 2)));
        // Removing it ends it.
        send(&mut h, Command::RemovePattern(2));
        assert!(onsets(&render(&mut h, 20 * STEP)).is_empty());
        assert_eq!(load_overlay(&h.audio.overlay_playhead), None);
    }

    #[test]
    fn a_pattern_over_the_song_sounds_as_its_clips_there_would() {
        // A bass note in each bar of the song, and a melody in pattern 1,
        // with loose timing on and a last note cut at the pattern's end.
        let song = |over_song: bool| {
            let mut h = harness();
            send(
                &mut h,
                Command::InsertPattern {
                    index: 1,
                    length: 16,
                },
            );
            send(&mut h, Command::SetLooseTiming(20));
            let melody = [(60, 2, 3), (64, 6, 2), (67, 12, 6)]
                .map(|(midi, start, length)| NoteEvent::new(Note::new(midi), start, length));
            send(
                &mut h,
                Command::InsertChannel {
                    index: AT_END,
                    instrument: Instrument::Synth(Synth::new(RATE)),
                    notes: ChannelNotes::new([
                        vec![NoteEvent::new(Note::new(40), 0, 8)],
                        melody.to_vec(),
                    ]),
                    volume: 1.0,
                    muted: false,
                },
            );
            let mut plays = vec![whole(0, 0), whole(0, 16), whole(0, 32)];
            if !over_song {
                plays.extend([whole(1, 16), whole(1, 32)]);
            }
            send(&mut h, Command::SetSong { plays, length: 48 });
            send(&mut h, Command::SetMode(PlayMode::Song));
            send(&mut h, Command::Play);
            let mut out = render(&mut h, 3 * STEP);
            if over_song {
                send(&mut h, Command::SetOverlay(Some(1)));
            }
            out.extend(render(&mut h, 45 * STEP));
            out
        };
        let clips = song(false);
        assert!(clips[16 * STEP..].iter().any(|&sample| sample != 0.0));
        assert!(song(true) == clips, "the same samples, from bar 2 on");
    }

    #[test]
    fn only_a_playing_song_plays_a_pattern_over_it() {
        let mut h = two_patterns();
        send(&mut h, Command::SetOverlay(Some(1)));
        send(&mut h, Command::Play);
        // Pattern mode loops pattern 0 alone.
        assert_eq!(onsets(&render(&mut h, 20 * STEP)), vec![0, 16 * STEP]);

        let mut h = silent_song();
        send(&mut h, Command::SetOverlay(Some(1)));
        render(&mut h, 20 * STEP);
        send(&mut h, Command::Stop);
        send(&mut h, Command::Play);
        assert!(
            onsets(&render(&mut h, 40 * STEP)).is_empty(),
            "Stop ends it"
        );
    }

    #[test]
    fn a_song_plays_each_step_at_its_own_tempo() {
        let mut h = harness();
        add(&mut h, &[0, 1, 2, 3], false);
        send(
            &mut h,
            Command::SetSong {
                plays: vec![whole(0, 0)],
                length: 16,
            },
        );
        // Steps 0 and 1 at 120 BPM, then 60 BPM: twice as long.
        let tempos = [120.0, 120.0, 60.0, 60.0]
            .into_iter()
            .chain(std::iter::repeat_n(120.0, 12))
            .map(Tempo::new)
            .collect();
        send(&mut h, Command::SetSongTempos(tempos));
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        let out = render(&mut h, 6 * STEP);
        assert_eq!(onsets(&out), vec![0, STEP, 2 * STEP, 4 * STEP]);
    }

    #[test]
    fn pattern_mode_keeps_the_tempo_that_was_set() {
        let mut h = harness();
        add(&mut h, &[0, 1, 2], false);
        send(&mut h, Command::SetSongTempos(vec![Tempo::new(30.0); 16]));
        send(&mut h, Command::Play);
        let out = render(&mut h, 3 * STEP);
        assert_eq!(onsets(&out), vec![0, STEP, 2 * STEP]);
    }

    #[test]
    fn loose_timing_starts_notes_a_little_late() {
        let loose = |ms: u8| {
            let mut h = harness();
            add(&mut h, &[0, 4, 8, 12], false);
            send(&mut h, Command::SetLooseTiming(ms));
            send(&mut h, Command::Play);
            onsets(&render(&mut h, 16 * STEP))
        };
        assert_eq!(loose(0), vec![0, 4 * STEP, 8 * STEP, 12 * STEP]);
        let late = loose(20);
        // 20 ms at 48 kHz is 960 frames.
        assert_eq!(late.len(), 4);
        for (onset, on_grid) in late.iter().zip([0, 4 * STEP, 8 * STEP, 12 * STEP]) {
            assert!((on_grid..on_grid + 960).contains(onset), "{late:?}");
        }
        assert_ne!(late[1] - 4 * STEP, late[2] - 8 * STEP, "{late:?}");
        // The same every time.
        assert_eq!(loose(20), late);
    }

    #[test]
    fn notes_between_steps_start_on_their_tick() {
        // Eighth-note triplets: three notes a beat, every 32 ticks.
        let mut h = harness();
        let triplets: Vec<NoteEvent> = (0..3)
            .map(|n| NoteEvent::exact(STEP_NOTE, n * 32, 8))
            .collect();
        send(
            &mut h,
            Command::InsertChannel {
                index: AT_END,
                instrument: click(),
                notes: ChannelNotes::new([triplets]),
                volume: 1.0,
                muted: false,
            },
        );
        send(&mut h, Command::Play);
        // A tick is a 24th of a step: 250 frames.
        let tick = STEP / TICKS_PER_STEP;
        assert_eq!(onsets(&render(&mut h, 4 * STEP)), [0, 32 * tick, 64 * tick]);
    }

    #[test]
    fn a_trimmed_note_is_let_go_early() {
        // From a quarter of a step in, held for half a step, or to the
        // end of its second step.
        let tick = STEP / TICKS_PER_STEP;
        let loudness = |note: NoteEvent| {
            let mut h = harness();
            add_synth(&mut h, &[note]);
            send(&mut h, Command::Play);
            let out = render(&mut h, 2 * STEP);
            assert!(
                out[..6 * tick].iter().all(|&s| s == 0.0),
                "silent until it starts"
            );
            // How loud it is just before the longer one is let go.
            out[29 * tick..30 * tick]
                .iter()
                .fold(0.0f32, |m, s| m.max(s.abs()))
        };
        let short = NoteEvent::exact(Note::A4, 6, 12);
        let long = NoteEvent::exact(Note::A4, 6, 24);
        assert_eq!((short.length, long.length), (1, 2));
        // Let go 3000 frames earlier, it has faded to under half.
        assert!(loudness(short) < 0.6 * loudness(long));
    }

    /// A song of one 256-step pattern whose only note, on its first
    /// step, plays a recording: 12 seconds (96 steps) of a sound that
    /// rises steadily from 0 to 1, so where in it playback is can be
    /// read from its level. The song plays `plays` of it, in song mode.
    fn recording_song(plays: Vec<Play>, length: usize) -> Harness {
        let mut h = harness();
        send(
            &mut h,
            Command::InsertPattern {
                index: 1,
                length: 256,
            },
        );
        let frames = 12 * RATE as usize;
        let ramp = (0..frames).map(|i| i as f32 / frames as f32).collect();
        send(
            &mut h,
            Command::InsertChannel {
                index: AT_END,
                instrument: Instrument::Sampler(Sampler::new(Arc::new(SampleData::new(
                    ramp, RATE,
                )))),
                notes: ChannelNotes::new([Vec::new(), steps(&[0])]),
                volume: 1.0,
                muted: false,
            },
        );
        // Dry, so the level is the recording's own.
        send(
            &mut h,
            Command::SetReverbSend {
                channel: 0,
                send: 0.0,
            },
        );
        send(&mut h, Command::SetSong { plays, length });
        send(&mut h, Command::SetMode(PlayMode::Song));
        render(&mut h, STEP);
        h
    }

    #[test]
    fn a_recording_plays_from_wherever_playback_starts() {
        let piece = Play {
            pattern: 1,
            start: 0,
            skip: 0,
            length: 256,
        };
        // Played from the start, as far as step 64 and a little more.
        let mut h = recording_song(vec![piece], 256);
        send(&mut h, Command::Play);
        let whole = render(&mut h, 64 * STEP + 1_000);
        // Played from step 32, it sounds at once, as it did there.
        let mut h = recording_song(vec![piece], 256);
        send(&mut h, Command::Seek(32));
        send(&mut h, Command::Play);
        let out = render(&mut h, 1_000);
        assert!(out[0] > 0.0, "the recording sounds at once");
        assert_eq!(out, whole[32 * STEP..32 * STEP + 1_000]);
        // A jump while playing goes on from the new place, once the
        // sound from before has faded.
        send(&mut h, Command::Seek(64));
        let out = render(&mut h, 1_000);
        assert!((out[999] - whole[64 * STEP + 999]).abs() < 0.001);
    }

    #[test]
    fn a_recording_loops_with_a_loop_region_inside_it() {
        // The loop region is steps 32 to 48 of the song: one piece of the
        // recording's clip, 32 steps into it.
        let piece = Play {
            pattern: 1,
            start: 0,
            skip: 32,
            length: 16,
        };
        let mut h = recording_song(vec![piece], 16);
        send(&mut h, Command::Play);
        let out = render(&mut h, 17 * STEP);
        let first = out[2_000];
        assert!(first > 0.2, "{first}: the recording from 4 s on");
        // Round again, it starts again from the region's start, not on
        // from where the last lap ended.
        let again = out[16 * STEP + 2_000];
        assert!((again - first).abs() < 0.001, "{first} then {again}");
    }

    /// All 16 steps of a pattern, once, from song step `start`.
    fn whole(pattern: usize, start: usize) -> Play {
        Play {
            pattern,
            start,
            skip: 0,
            length: 16,
        }
    }

    #[test]
    fn repeated_pieces_loop_their_pattern() {
        let mut h = two_patterns();
        // Pattern 1 (a note on step 2) three times, then the last half of
        // it once more, as when a section starts halfway through a clip.
        let plays = vec![
            Play {
                length: 48,
                ..whole(1, 0)
            },
            Play {
                pattern: 1,
                start: 48,
                skip: 8,
                length: 8,
            },
        ];
        send(&mut h, Command::SetSong { plays, length: 56 });
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        let out = render(&mut h, 56 * STEP);
        // The half piece starts after step 2, so it adds nothing.
        assert_eq!(onsets(&out), vec![2 * STEP, 18 * STEP, 34 * STEP]);
    }

    #[test]
    fn clips_can_overlap() {
        let mut h = two_patterns();
        // Both patterns at once: steps 0 and 2.
        let plays = vec![whole(0, 0), whole(1, 0)];
        send(&mut h, Command::SetSong { plays, length: 16 });
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        let out = render(&mut h, 16 * STEP);
        assert_eq!(onsets(&out), vec![0, 2 * STEP]);
    }

    #[test]
    fn removing_a_pattern_moves_the_next_one_down() {
        let mut h = two_patterns();
        send(&mut h, Command::RemovePattern(0));
        send(&mut h, Command::Play);
        // Pattern 1 is now pattern 0, and the selected pattern.
        let out = render(&mut h, 16 * STEP);
        assert_eq!(onsets(&out), vec![2 * STEP]);
        assert!(matches!(h.trash.pop(), Ok(Garbage::Notes(_))));
    }

    #[test]
    fn new_patterns_start_empty_in_every_channel() {
        let mut h = two_patterns();
        send(
            &mut h,
            Command::InsertPattern {
                index: 2,
                length: 16,
            },
        );
        send(&mut h, Command::SelectPattern(2));
        send(&mut h, Command::Play);
        render(&mut h, 10);
        assert_eq!(h.audio.channels[0].notes.0.len(), 3);
        assert!(onsets(&render(&mut h, 16 * STEP)).is_empty());
    }

    #[test]
    fn inserting_a_pattern_moves_the_later_ones_up() {
        let mut h = two_patterns();
        // A new, empty pattern 0: the old pattern 0 (a hit on step 0) is
        // now pattern 1, and the old pattern 1 (step 2) is pattern 2.
        send(
            &mut h,
            Command::InsertPattern {
                index: 0,
                length: 16,
            },
        );
        send(&mut h, Command::SelectPattern(2));
        send(&mut h, Command::Play);
        assert_eq!(onsets(&render(&mut h, 16 * STEP)), vec![2 * STEP]);
        assert_eq!(h.audio.pattern_lengths, vec![16, 16, 16]);
        assert!(h.audio.channels[0].notes.get(0).is_empty());
    }

    #[test]
    fn channels_can_be_inserted_between_others() {
        let mut h = harness();
        add(&mut h, &[0], false);
        add(&mut h, &[2], false);
        send(
            &mut h,
            Command::InsertChannel {
                index: 1,
                instrument: click(),
                notes: ChannelNotes::new([steps(&[1])]),
                volume: 1.0,
                muted: false,
            },
        );
        render(&mut h, 1);
        let first_steps: Vec<usize> = h
            .audio
            .channels
            .iter()
            .map(|channel| channel.notes.get(0)[0].start)
            .collect();
        assert_eq!(first_steps, vec![0, 1, 2]);
    }

    #[test]
    fn channel_notes_have_room_for_every_pattern() {
        let notes = ChannelNotes::new([Vec::new()]);
        assert!(notes.0.capacity() >= MAX_PATTERNS);
    }

    #[test]
    fn clear_throws_everything_away() {
        let mut h = two_patterns();
        send(
            &mut h,
            Command::SetSong {
                plays: vec![whole(0, 0)],
                length: 16,
            },
        );
        send(&mut h, Command::Play);
        send(&mut h, Command::Clear);
        render(&mut h, 10);
        assert!(h.audio.channels.is_empty());
        assert!(h.audio.pattern_lengths.is_empty());
        assert!(!h.audio.playing);
        let mut kinds = Vec::new();
        while let Ok(garbage) = h.trash.pop() {
            kinds.push(match garbage {
                Garbage::Channel(_) => "channel",
                Garbage::Song(_) => "song",
                Garbage::Tempos(_) | Garbage::Clicks(_) => continue,
                Garbage::Instrument(_)
                | Garbage::Notes(_)
                | Garbage::Phrase(_)
                | Garbage::Trace(_)
                | Garbage::DelayLine(_) => "other",
            });
        }
        // The empty song from the start, the real one, then the channel and the song.
        assert_eq!(kinds, ["song", "channel", "song"]);
    }

    #[test]
    fn commands_stay_small() {
        // Every slot of the command queue is as big as the biggest
        // command. The instruments keep their voices in a `Box`, so a
        // command holds a pointer to them instead of all the voices.
        // (72 bytes since `InsertChannel` got its `index`, in step 5.)
        let size = std::mem::size_of::<Command>();
        assert!(size <= 80, "a command takes {size} bytes");
    }

    fn key_down(pitch: Note) -> MidiMessage {
        MidiMessage::NoteOn {
            channel: 0,
            pitch,
            velocity: 100,
        }
    }

    fn key_up(pitch: Note) -> MidiMessage {
        MidiMessage::NoteOff { channel: 0, pitch }
    }

    fn voices(h: &Harness, channel: usize) -> usize {
        h.audio.channels[channel].instrument.voices_playing()
    }

    #[test]
    fn keys_play_the_live_channel_until_they_come_up() {
        let mut h = harness();
        add_synth(&mut h, &[]);
        add_synth(&mut h, &[]);
        send(&mut h, Command::SetLiveChannel(Some(1)));
        send(&mut h, Command::Live(key_down(Note::A4)));
        // Held far longer than a preview (one beat, 24 000 frames).
        render(&mut h, 48_000);
        assert_eq!((voices(&h, 0), voices(&h, 1)), (0, 1));
        send(&mut h, Command::Live(key_up(Note::A4)));
        render(&mut h, 48_000);
        assert_eq!(voices(&h, 1), 0);
    }

    #[test]
    fn deselecting_a_channel_releases_keys_and_disables_live_input() {
        let mut h = harness();
        add_synth(&mut h, &[]);
        send(&mut h, Command::Live(key_down(Note::C4)));
        render(&mut h, 100);
        assert_eq!(voices(&h, 0), 1);
        send(&mut h, Command::SetLiveChannel(None));
        send(&mut h, Command::Live(key_down(Note::A4)));
        render(&mut h, 48_000);
        assert_eq!(voices(&h, 0), 0);
        send(&mut h, Command::SetLiveChannel(Some(0)));
        send(&mut h, Command::Live(key_down(Note::A4)));
        render(&mut h, 100);
        assert_eq!(voices(&h, 0), 1);
    }

    #[test]
    fn a_key_comes_up_on_the_channel_it_went_down_on() {
        let mut h = harness();
        add_synth(&mut h, &[]);
        add_synth(&mut h, &[]);
        send(&mut h, Command::Live(key_down(Note::C4)));
        send(&mut h, Command::SetLiveChannel(Some(1)));
        send(&mut h, Command::Live(key_up(Note::C4)));
        render(&mut h, 48_000);
        assert_eq!(voices(&h, 0), 0, "the note on channel 0 has ended");
    }

    #[test]
    fn a_key_up_leaves_the_sequencers_notes_alone() {
        let mut h = harness();
        add_synth(&mut h, &[NoteEvent::new(Note::C4, 0, 16)]);
        send(&mut h, Command::Play);
        send(&mut h, Command::Live(key_down(Note::C4)));
        render(&mut h, 100);
        assert_eq!(voices(&h, 0), 2);
        send(&mut h, Command::Live(key_up(Note::C4)));
        render(&mut h, 48_000);
        assert_eq!(voices(&h, 0), 1, "the pattern's C4 is still held");
    }

    #[test]
    fn keys_from_a_midi_keyboard_skip_the_command_queue() {
        let mut h = harness();
        add_synth(&mut h, &[]);
        let (mut keys, consumer) = rtrb::RingBuffer::new(8);
        h.audio.live_input = Some(consumer);
        keys.push(key_down(Note::C4)).unwrap();
        keys.push(key_down(Note::A4)).unwrap();
        render(&mut h, 10);
        assert_eq!(voices(&h, 0), 2);
    }

    #[test]
    fn removing_channels_lets_go_of_their_keys() {
        let mut h = harness();
        add_synth(&mut h, &[]);
        add_synth(&mut h, &[]);
        send(&mut h, Command::SetLiveChannel(Some(1)));
        send(&mut h, Command::Live(key_down(Note::C4)));
        // Channel 1 is now channel 0. Its key must not stay down.
        send(&mut h, Command::RemoveChannel(0));
        render(&mut h, 48_000);
        assert_eq!(voices(&h, 0), 0);
        assert!(h.audio.live_keys.iter().all(Option::is_none));
    }

    #[test]
    fn the_position_is_shared_while_playing() {
        let mut h = harness();
        render(&mut h, 100);
        assert_eq!(load_position(&h.position), None);
        send(&mut h, Command::Play);
        render(&mut h, STEP * 5 / 2);
        assert_eq!(load_position(&h.position), Some(2.5));
        send(&mut h, Command::Stop);
        render(&mut h, 1);
        assert_eq!(load_position(&h.position), None);
    }

    /// Adds a MIDI Out channel on MIDI channel 3, and a queue to read what it sends.
    fn add_midi_out(h: &mut Harness, notes: &[NoteEvent]) -> rtrb::Consumer<MidiMessage> {
        let (producer, consumer) = rtrb::RingBuffer::new(64);
        h.audio.midi_out = Some(producer);
        send(
            h,
            Command::InsertChannel {
                index: AT_END,
                instrument: Instrument::MidiOut(MidiOut::new(3)),
                notes: ChannelNotes::new([notes.to_vec()]),
                volume: 1.0,
                muted: false,
            },
        );
        consumer
    }

    fn received(out: &mut rtrb::Consumer<MidiMessage>) -> Vec<MidiMessage> {
        std::iter::from_fn(|| out.pop().ok()).collect()
    }

    #[test]
    fn midi_out_channels_send_their_notes() {
        let mut h = harness();
        let mut out = add_midi_out(&mut h, &[NoteEvent::new(Note::A4, 1, 2)]);
        send(&mut h, Command::Play);
        let silence = render(&mut h, STEP + 10);
        assert!(silence.iter().all(|&s| s == 0.0), "no sound of its own");
        let on = MidiMessage::NoteOn {
            channel: 3,
            pitch: Note::A4,
            velocity: DEFAULT_VELOCITY,
        };
        assert_eq!(received(&mut out), [on]);
        // Two steps later, the note-off.
        render(&mut h, 2 * STEP);
        let off = MidiMessage::NoteOff {
            channel: 3,
            pitch: Note::A4,
        };
        assert_eq!(received(&mut out), [off]);
    }

    fn note_off(pitch: Note) -> MidiMessage {
        MidiMessage::NoteOff { channel: 3, pitch }
    }

    #[test]
    fn a_note_across_a_section_played_once_is_not_cut() {
        // A note from step 28 to step 36 of a four-bar clip, under a
        // section over the first two bars that is played once.
        let mut project = Project::empty();
        project.add_channel(Channel::midi_out(3));
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(64);
        pattern.add_note(0, NoteEvent::new(Note::A4, 28, 8));
        project.add_clip(Clip::new(0, 0, 0));
        project.add_section(Section::new("Intro", 0, 32));
        let song = project.arrangement();

        let mut h = harness();
        let mut out = add_midi_out(&mut h, project.patterns()[0].notes(0));
        send(
            &mut h,
            Command::SetPatternLength {
                pattern: 0,
                length: 64,
            },
        );
        send(
            &mut h,
            Command::SetSong {
                plays: song.plays,
                length: song.length,
            },
        );
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        // Past the end of the section, the note is still held...
        render(&mut h, 36 * STEP - 10);
        assert_eq!(received(&mut out).len(), 1, "only the note-on");
        // ...and it ends on step 36.
        render(&mut h, 20);
        assert_eq!(received(&mut out), [note_off(Note::A4)]);
    }

    #[test]
    fn muting_a_channel_stops_its_notes_but_not_keys_held_down() {
        let mut h = harness();
        add_synth(&mut h, &[NoteEvent::new(Note::A4, 0, 16)]);
        send(&mut h, Command::Play);
        send(&mut h, Command::Live(key_down(Note::C4)));
        render(&mut h, 100);
        assert_eq!(voices(&h, 0), 2);
        send(
            &mut h,
            Command::SetMuted {
                channel: 0,
                muted: true,
            },
        );
        // Long before the 16 steps of the A4 are over, it has faded out.
        render(&mut h, 48_000);
        assert_eq!(voices(&h, 0), 1, "only the key held down still sounds");
    }

    #[test]
    fn notes_stop_when_the_song_no_longer_plays_them() {
        let mut h = harness();
        let mut out = add_midi_out(&mut h, &[NoteEvent::new(Note::A4, 0, 16)]);
        let song = |plays: Vec<Play>| Command::SetSong { plays, length: 48 };
        send(&mut h, song(vec![whole(0, 0), whole(0, 16)]));
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        render(&mut h, STEP);
        assert_eq!(received(&mut out).len(), 1, "the note-on");
        // The other clip moves: the note plays on.
        send(&mut h, song(vec![whole(0, 0), whole(0, 32)]));
        render(&mut h, STEP);
        assert_eq!(received(&mut out), []);
        // The note's own clip goes (its track was muted, say): it stops.
        send(&mut h, song(vec![whole(0, 32)]));
        render(&mut h, 10);
        assert_eq!(received(&mut out), [note_off(Note::A4)]);
    }

    #[test]
    fn a_recording_stops_when_its_track_is_muted() {
        // A two-second sound started by one step, as a take is.
        let mut h = harness();
        let recording = Sampler::new(Arc::new(SampleData::new(vec![0.5; 2 * 48_000], RATE)));
        send(
            &mut h,
            Command::InsertChannel {
                index: AT_END,
                instrument: Instrument::Sampler(recording),
                notes: ChannelNotes::new([steps(&[0])]),
                volume: 1.0,
                muted: false,
            },
        );
        let plays = vec![whole(0, 0)];
        send(&mut h, Command::SetSong { plays, length: 16 });
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::Play);
        assert!(render(&mut h, 2 * STEP).iter().all(|&s| s != 0.0));
        send(
            &mut h,
            Command::SetSong {
                plays: Vec::new(),
                length: 16,
            },
        );
        let out = render(&mut h, 2_000);
        assert_eq!(out[1_999], 0.0, "faded out");
    }

    #[test]
    fn a_new_tempo_moves_the_end_of_the_notes_sounding() {
        let mut h = harness();
        let mut out = add_midi_out(&mut h, &[NoteEvent::new(Note::A4, 0, 8)]);
        send(&mut h, Command::Play);
        render(&mut h, STEP + STEP / 2); // halfway through step 1
        received(&mut out);
        // Half as fast. The rest of step 1 still takes the time it had
        // left; each of steps 2 to 7 then takes twice as long.
        send(&mut h, Command::SetTempo(Tempo::new(60.0)));
        let to_step_8 = STEP / 2 + 6 * 2 * STEP;
        render(&mut h, to_step_8 - 10);
        assert_eq!(received(&mut out), [], "still held");
        render(&mut h, 20);
        assert_eq!(received(&mut out), [note_off(Note::A4)]);
        assert_eq!(h.playhead.load(Ordering::Relaxed), 8, "on step 8");
    }

    #[test]
    fn removing_a_midi_out_ends_its_notes() {
        let mut h = harness();
        let mut out = add_midi_out(&mut h, &[]);
        send(&mut h, Command::Live(key_down(Note::C4)));
        render(&mut h, 10);
        assert_eq!(received(&mut out).len(), 1);
        send(&mut h, Command::RemoveChannel(0));
        render(&mut h, 10);
        let off = MidiMessage::NoteOff {
            channel: 3,
            pitch: Note::C4,
        };
        assert_eq!(received(&mut out), [off]);
    }

    #[test]
    fn a_midi_message_waits_for_room_instead_of_being_dropped() {
        let (producer, mut consumer) = rtrb::RingBuffer::new(1);
        let mut out = Some(producer);
        let mut backlog = VecDeque::new();
        let on = MidiMessage::NoteOn {
            channel: 3,
            pitch: Note::A4,
            velocity: DEFAULT_VELOCITY,
        };
        let off = note_off(Note::A4);
        queue_midi(&mut out, &mut backlog, on);
        queue_midi(&mut out, &mut backlog, off);
        assert_eq!(backlog, VecDeque::from([off]), "the note off waits");
        assert_eq!(consumer.pop().unwrap(), on);

        // The next block has room, so the waiting message goes out.
        let mut h = harness();
        h.audio.midi_out = out;
        h.audio.midi_backlog = backlog;
        h.audio.flush_midi_backlog();
        assert!(h.audio.midi_backlog.is_empty());
        assert_eq!(consumer.pop().unwrap(), off);
    }

    #[test]
    fn a_new_midi_message_does_not_pass_one_that_waits() {
        let (producer, mut consumer) = rtrb::RingBuffer::new(1);
        let mut out = Some(producer);
        let mut backlog = VecDeque::new();
        let on = |velocity| MidiMessage::NoteOn {
            channel: 3,
            pitch: Note::A4,
            velocity,
        };
        let off = note_off(Note::A4);
        queue_midi(&mut out, &mut backlog, on(DEFAULT_VELOCITY));
        queue_midi(&mut out, &mut backlog, off);
        assert_eq!(consumer.pop().unwrap(), on(DEFAULT_VELOCITY));

        // The port took the first note, so there is room again, but the
        // note off is still waiting: the key's next note on goes after it.
        queue_midi(&mut out, &mut backlog, on(64));
        assert_eq!(backlog, VecDeque::from([off, on(64)]));
        assert!(
            consumer.pop().is_err(),
            "nothing went ahead of the note off"
        );
    }

    #[test]
    fn trash_waits_for_the_ui_instead_of_being_freed_on_the_audio_thread() {
        let mut h = harness();
        let (producer, mut consumer) = rtrb::RingBuffer::new(1);
        h.audio.trash = producer;
        h.audio.throw_away(Garbage::Song(Vec::new()));
        h.audio.throw_away(Garbage::Song(Vec::new()));
        assert_eq!(h.audio.trash_backlog.len(), 1, "the second value waits");

        // The UI collects, and the next block hands over what waited.
        assert!(consumer.pop().is_ok());
        h.audio.flush_trash();
        assert!(h.audio.trash_backlog.is_empty());
        assert!(consumer.pop().is_ok(), "the waiting value reached the UI");
    }

    #[test]
    fn converts_to_integer_samples() {
        let mut h = harness();
        add(&mut h, &[0], false);
        send(&mut h, Command::SetMasterVolume(1.0));
        send(&mut h, Command::Play);
        let mut buffer = vec![0_i16; 10];
        h.audio.process(&mut buffer);
        // About 10 000: half volume, times the default velocity, times
        // the master volume, which is still sliding up from 0.8.
        assert!(buffer[0] > 5_000, "got {}", buffer[0]);
    }

    /// The clicks of a bar of 4/4: the bar on step 0, beats on 4, 8, 12.
    fn four_four() -> Vec<Click> {
        (0..16)
            .map(|step| match step {
                0 => Click::Bar,
                step if step % 4 == 0 => Click::Beat,
                _ => Click::None,
            })
            .collect()
    }

    #[test]
    fn the_count_in_comes_before_the_music() {
        let mut h = harness();
        add(&mut h, &[0], false);
        send(&mut h, Command::SetPatternClicks(four_four()));
        send(&mut h, Command::SetCountIn(true));
        send(&mut h, Command::Play);
        let counted = render(&mut h, 16 * STEP);
        // A click's first sample is sin(0) = 0, so it is heard one frame in.
        assert_eq!(
            onsets(&counted),
            [1, 4 * STEP + 1, 8 * STEP + 1, 12 * STEP + 1]
        );
        assert_eq!(
            h.playhead.load(Ordering::Relaxed),
            NO_STEP,
            "the playhead waits"
        );
        let out = render(&mut h, STEP);
        assert_eq!(onsets(&out), [0], "the music starts right after the bar");
        assert_eq!(h.playhead.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn the_count_in_runs_on_into_a_bar_started_in_the_middle() {
        let mut h = harness();
        add(&mut h, &[6], false);
        send(&mut h, Command::SetPatternClicks(four_four()));
        send(&mut h, Command::SetCountIn(true));
        send(&mut h, Command::Seek(6));
        send(&mut h, Command::Play);
        // "1 2 3 4 1 2", then the music from step 6, halfway into beat 2.
        let out = render(&mut h, 23 * STEP);
        let clicks: Vec<usize> = [0, 4, 8, 12, 16, 20].map(|step| step * STEP + 1).into();
        assert_eq!(onsets(&out), [clicks, vec![22 * STEP]].concat());
    }

    #[test]
    fn stopping_ends_the_count_in() {
        let mut h = harness();
        add(&mut h, &[0], false);
        send(&mut h, Command::SetCountIn(true));
        send(&mut h, Command::Play);
        render(&mut h, 2 * STEP);
        send(&mut h, Command::Stop);
        let out = render(&mut h, 20 * STEP);
        assert!(out[STEP..].iter().all(|&s| s == 0.0));
    }

    #[test]
    fn the_metronome_clicks_every_beat() {
        let mut h = harness();
        send(&mut h, Command::SetPatternClicks(four_four()));
        send(&mut h, Command::SetMetronome(true));
        send(&mut h, Command::Play);
        let out = render(&mut h, 16 * STEP);
        assert_eq!(onsets(&out), [1, 4 * STEP + 1, 8 * STEP + 1, 12 * STEP + 1]);
        let loudest = |from: usize| {
            out[from..from + 100]
                .iter()
                .fold(0.0_f32, |a, s| a.max(s.abs()))
        };
        assert!(
            loudest(0) > loudest(4 * STEP),
            "the first beat of the bar is louder"
        );
        // Off again: nothing more.
        send(&mut h, Command::SetMetronome(false));
        assert!(render(&mut h, 16 * STEP).iter().all(|&s| s == 0.0));
    }

    #[test]
    fn the_metronome_is_heard_dry_through_a_full_reverb() {
        let clicks = |reverb: bool| {
            let mut h = harness();
            send(
                &mut h,
                Command::SetReverb(lineup_core::ReverbSettings {
                    enabled: reverb,
                    mix: 1.0,
                    decay: 2.0,
                    damping: 0.0,
                    pre_delay_ms: 50.0,
                }),
            );
            send(&mut h, Command::SetPatternClicks(four_four()));
            send(&mut h, Command::SetMetronome(true));
            send(&mut h, Command::Play);
            render(&mut h, 8 * STEP)
        };
        // All wet, the reverb would delay the click and give it a tail.
        assert_eq!(clicks(true), clicks(false));
        assert!(clicks(true).iter().any(|&s| s != 0.0));
    }

    #[test]
    fn the_metronome_follows_the_song_in_song_mode() {
        let mut h = harness();
        let plays = vec![whole(0, 0)];
        send(&mut h, Command::SetSong { plays, length: 12 });
        // A bar of 3/4.
        let clicks = (0..12)
            .map(|step| match step {
                0 => Click::Bar,
                step if step % 4 == 0 => Click::Beat,
                _ => Click::None,
            })
            .collect();
        send(&mut h, Command::SetSongClicks(clicks));
        send(&mut h, Command::SetPatternClicks(four_four()));
        send(&mut h, Command::SetMode(PlayMode::Song));
        send(&mut h, Command::SetMetronome(true));
        send(&mut h, Command::Play);
        let out = render(&mut h, 16 * STEP);
        // The bar comes round again after three beats.
        assert_eq!(onsets(&out), [1, 4 * STEP + 1, 8 * STEP + 1, 12 * STEP + 1]);
        let loudest = |from: usize| {
            out[from..from + 100]
                .iter()
                .fold(0.0_f32, |a, s| a.max(s.abs()))
        };
        assert!(
            loudest(12 * STEP) > loudest(8 * STEP),
            "step 12 is a new bar"
        );
    }
}
