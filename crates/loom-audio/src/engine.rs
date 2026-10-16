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
//! playhead in the UI) and old instruments, note lists and songs that need
//! freeing (through a second ring buffer, so the memory is freed on the
//! UI thread).
//!
//! The audio thread keeps its own copy of what it needs to play: the
//! notes of every channel in every pattern, the length of every pattern,
//! and the song (the clips of the playlist). In pattern mode it loops the
//! selected pattern; in song mode it plays the clips.

use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig};
use loom_core::{
    Clip, DEFAULT_VELOCITY, MAX_CHANNELS, MAX_PATTERNS, Note, NoteEvent, STEP_NOTE, STEPS_PER_BAR,
    STEPS_PER_BEAT, Sequencer, Tempo,
};

use crate::dsp::Smoothed;
use crate::instrument::Instrument;

/// How many commands can wait in the queue before `send` starts failing.
/// The audio thread empties the queue every few milliseconds, so this is
/// far more than the UI will ever send in that time.
const COMMAND_QUEUE_SIZE: usize = 1024;

/// How many old instruments and note lists can wait to be freed by the UI
/// thread. Removing a pattern throws away one list per channel, and
/// loading a project throws away every channel, so this is generous.
pub(crate) const TRASH_QUEUE_SIZE: usize = 1024;

/// The value of the playhead while stopped.
pub(crate) const NO_STEP: usize = usize::MAX;

/// How long volume changes take to settle, in seconds.
const VOLUME_SMOOTHING: f32 = 0.01;

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

/// A message from the UI thread to the audio thread.
///
/// Channel and pattern numbers are positions in the UI's lists, counted
/// from 0, and must match them exactly: the UI sends `InsertChannel`,
/// `RemoveChannel`, `InsertPattern` and `RemovePattern` whenever its own
/// lists change.
#[derive(Debug)]
pub enum Command {
    /// Start playing from the beginning.
    Play,
    /// Stop playback. Sounds already playing ring out.
    Stop,
    /// Switch between looping a pattern and playing the song. If playing,
    /// starts again from the beginning.
    SetMode(PlayMode),
    /// Choose the pattern that pattern mode loops.
    SelectPattern(usize),
    SetTempo(Tempo),
    SetMasterVolume(f32),
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
    /// Replace the song: the clips to play (only those on tracks that are
    /// not muted) and the song's length in steps.
    SetSong {
        clips: Vec<Clip>,
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
    /// Stop, and remove every channel, pattern and clip, before a new
    /// project is sent.
    Clear,
}

/// Something the audio thread has finished with. It is sent back to the
/// UI thread, because freeing memory is not allowed on the audio thread.
pub(crate) enum Garbage {
    Instrument(Instrument),
    Notes(Vec<NoteEvent>),
    Channel(ChannelState),
    Song(Vec<Clip>),
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
                Garbage::Channel(channel) => drop(channel),
                Garbage::Song(clips) => drop(clips),
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
pub(crate) struct ChannelState {
    instrument: Instrument,
    notes: ChannelNotes,
    volume: Smoothed,
    muted: bool,
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
    /// The clips of the song, and its length in steps.
    clips: Vec<Clip>,
    song_length: usize,
    mode: PlayMode,
    /// The pattern that pattern mode loops.
    pattern: usize,
    sequencer: Sequencer,
    playing: bool,
    master_volume: Smoothed,
    commands: rtrb::Consumer<Command>,
    trash: rtrb::Producer<Garbage>,
    playhead: Arc<AtomicUsize>,
}

impl AudioThread {
    pub(crate) fn new(
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
            pattern_lengths: Vec::with_capacity(MAX_PATTERNS),
            clips: Vec::new(),
            song_length: 0,
            mode: PlayMode::Pattern,
            pattern: 0,
            sequencer: Sequencer::new(Tempo::default(), sample_rate, STEPS_PER_BAR),
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
            Command::Play => {
                self.sequencer.reset();
                self.playing = true;
            }
            Command::Stop => self.stop(),
            Command::SetMode(mode) => {
                self.mode = mode;
                self.update_loop_length();
                self.sequencer.reset();
            }
            Command::SelectPattern(pattern) => {
                self.pattern = pattern;
                self.update_loop_length();
            }
            Command::SetTempo(tempo) => self.sequencer.set_tempo(tempo),
            Command::SetMasterVolume(volume) => self.master_volume.set(volume),
            Command::InsertPattern { index, length } => {
                if self.pattern_lengths.len() < MAX_PATTERNS {
                    let index = index.min(self.pattern_lengths.len());
                    self.pattern_lengths.insert(index, length);
                    for channel in &mut self.channels {
                        channel.notes.insert_empty(index);
                    }
                    self.update_loop_length();
                }
            }
            Command::RemovePattern(pattern) => {
                if pattern < self.pattern_lengths.len() {
                    self.pattern_lengths.remove(pattern);
                    for index in 0..self.channels.len() {
                        if let Some(notes) = self.channels[index].notes.remove(pattern) {
                            self.throw_away(Garbage::Notes(notes));
                        }
                    }
                    self.update_loop_length();
                }
            }
            Command::SetPatternLength { pattern, length } => {
                if let Some(slot) = self.pattern_lengths.get_mut(pattern) {
                    *slot = length;
                    self.update_loop_length();
                }
            }
            Command::SetSong { clips, length } => {
                let old = std::mem::replace(&mut self.clips, clips);
                self.throw_away(Garbage::Song(old));
                self.song_length = length;
                self.update_loop_length();
            }
            Command::InsertChannel {
                index,
                instrument,
                notes,
                volume,
                muted,
            } => {
                let channel = ChannelState {
                    instrument,
                    notes,
                    volume: Smoothed::new(volume, VOLUME_SMOOTHING, self.sample_rate),
                    muted,
                };
                if self.channels.len() < MAX_CHANNELS {
                    // `insert` moves the channels after `index` along by
                    // one, inside the room reserved up front.
                    self.channels
                        .insert(index.min(self.channels.len()), channel);
                } else {
                    self.throw_away(Garbage::Channel(channel));
                }
            }
            Command::RemoveChannel(index) => {
                if index < self.channels.len() {
                    let removed = self.channels.remove(index);
                    self.throw_away(Garbage::Channel(removed));
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
            Command::SetMuted { channel, muted } => {
                if let Some(state) = self.channels.get_mut(channel) {
                    state.muted = muted;
                }
            }
            Command::Preview(channel) => self.preview(channel, STEP_NOTE),
            Command::PreviewNote { channel, pitch } => self.preview(channel, pitch),
            Command::Clear => {
                self.stop();
                // `pop` takes the last channel out of the list; `while let`
                // keeps going until there are none left.
                while let Some(channel) = self.channels.pop() {
                    self.throw_away(Garbage::Channel(channel));
                }
                // `clear` empties the list but keeps its memory: nothing is freed.
                self.pattern_lengths.clear();
                let old = std::mem::take(&mut self.clips);
                self.throw_away(Garbage::Song(old));
                self.song_length = 0;
                self.pattern = 0;
                self.update_loop_length();
            }
        }
    }

    /// Puts every volume at its target at once. Smoothing avoids clicks
    /// while sound is playing; before an export starts, it would only
    /// make the first moment too loud or too quiet.
    pub(crate) fn settle_volumes(&mut self) {
        self.master_volume.settle();
        for channel in &mut self.channels {
            channel.volume.settle();
        }
    }

    fn stop(&mut self) {
        self.playing = false;
        self.playhead.store(NO_STEP, Ordering::Relaxed);
        for channel in &mut self.channels {
            channel.instrument.release_all();
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
    /// mode, it is a step of the song, and each clip playing then is asked
    /// for the notes at its own step: a clip that started on song step 32
    /// plays its pattern's step 2 on song step 34.
    fn start_notes(&mut self, step: usize) {
        let timing = Timing {
            step_frames: self.sequencer.step_frames(),
            sample_rate: self.sample_rate,
        };
        match self.mode {
            PlayMode::Pattern => {
                let length = self.pattern_length(self.pattern);
                start_pattern_notes(&mut self.channels, self.pattern, step, length, timing);
            }
            PlayMode::Song => {
                // `self.clips` is read while `self.channels` is changed: two
                // different fields, so the borrows do not overlap.
                for clip in &self.clips {
                    let length = self.pattern_lengths.get(clip.pattern).copied().unwrap_or(0);
                    if (clip.start..clip.start + length).contains(&step) {
                        let local_step = step - clip.start;
                        start_pattern_notes(
                            &mut self.channels,
                            clip.pattern,
                            local_step,
                            length,
                            timing,
                        );
                    }
                }
            }
        }
    }

    /// Applies waiting commands, then fills `output` with samples.
    pub(crate) fn process<T>(&mut self, output: &mut [T])
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

/// How long a step is, and the sample rate the instruments run at.
#[derive(Debug, Clone, Copy)]
struct Timing {
    step_frames: f64,
    sample_rate: u32,
}

/// Starts the notes of one pattern that begin on its `step`.
///
/// This is a plain function, not a method, so that it borrows only the
/// channels. Its caller can keep reading other fields of the audio thread
/// (such as the clips) while the channels are being changed.
fn start_pattern_notes(
    channels: &mut [ChannelState],
    pattern: usize,
    step: usize,
    length: usize,
    timing: Timing,
) {
    for channel in channels {
        if channel.muted {
            continue;
        }
        // `channel.notes` is borrowed to read while `channel.instrument`
        // is borrowed to change. That is allowed: they are different
        // fields of the struct, so the borrows do not overlap.
        for note in channel
            .notes
            .get(pattern)
            .iter()
            .filter(|note| note.start == step)
        {
            // A note that runs past the end of the pattern is cut there.
            let steps_held = note.length.min(length.saturating_sub(step));
            let gate = (steps_held as f64 * timing.step_frames) as u32;
            channel
                .instrument
                .note_on(note.pitch, note.velocity, gate, timing.sample_rate);
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

    /// An audio thread with one empty pattern, 16 steps long, at 120 BPM.
    fn harness() -> Harness {
        let (commands, command_consumer) = rtrb::RingBuffer::new(64);
        let (trash_producer, trash) = rtrb::RingBuffer::new(64);
        let playhead = Arc::new(AtomicUsize::new(NO_STEP));
        let audio = AudioThread::new(RATE, 1, command_consumer, trash_producer, playhead.clone());
        let mut h = Harness {
            audio,
            commands,
            trash,
            playhead,
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
                Garbage::Song(_) => {}
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
    fn preview_plays_without_the_sequencer() {
        let mut h = harness();
        add(&mut h, &[], false);
        h.commands.push(Command::Preview(0)).unwrap();
        assert!(render(&mut h, 10)[0] != 0.0);
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
        let clips = vec![Clip::new(1, 0, 0), Clip::new(0, 0, 16)];
        send(&mut h, Command::SetSong { clips, length: 32 });
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

    #[test]
    fn clips_can_overlap() {
        let mut h = two_patterns();
        // Both patterns at once: steps 0 and 2.
        let clips = vec![Clip::new(0, 0, 0), Clip::new(1, 1, 0)];
        send(&mut h, Command::SetSong { clips, length: 16 });
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
                clips: vec![Clip::new(0, 0, 0)],
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
                Garbage::Instrument(_) | Garbage::Notes(_) => "other",
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
}
