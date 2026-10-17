//! The instruments a channel can play: a sampler, the Loom Osc synth,
//! and a MIDI Out that has an external synth play the notes. A sampler
//! or a synth can also be turned into a MIDI Out when every channel is
//! sent to the MIDI port (see [`Instrument::to_midi`]).
//!
//! The sampler and the synth are *polyphonic*: they have a fixed number
//! of voices, and each note that starts takes one. That is what lets the
//! synth play chords and lets a drum hit ring on while the next one
//! starts.

use std::sync::Arc;

use loom_core::{Channel, ChannelKind, MidiMessage, Note, STEP_NOTE, SampleSource};

use crate::drums;
use crate::dsp::LowPass;
use crate::sample::{SampleCache, SampleData, SampleError};
use crate::timbre::{Timbre, Wave};

/// How many notes the Loom Osc can play at once.
pub const SYNTH_VOICES: usize = 16;
/// Sounds longer than this, in seconds, stop when playback stops.
const LONG_SAMPLE_SECONDS: u64 = 10;

/// How fast a stopped long sound fades out: its level is multiplied by
/// this every frame, which takes it to silence in about 15 ms.
const FADE_FACTOR: f32 = 0.99;

/// How many hits a sampler can play at once.
pub const SAMPLER_VOICES: usize = 8;

/// A gate that never runs out: the note is held until [`Instrument::note_off`].
/// Notes played on a keyboard use it, because nobody knows how long a key
/// will be held when it goes down.
pub const HELD: u32 = u32::MAX;

/// Where in the song a note the sequencer started comes from.
///
/// Every note of the song is a note of some pattern, started on some step
/// of the song. Each voice remembers that, so the audio thread can tell
/// later whether the song still plays it: when its track is muted or its
/// clip is taken away while it sounds, the note should stop (see
/// [`Instrument::release_unless`]). Keys played on a keyboard and
/// previews have no origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Origin {
    /// The pattern the note belongs to.
    pub pattern: usize,
    /// The step of the pattern the note starts on.
    pub pattern_step: usize,
    /// The step of the song (of the pattern, in pattern mode) it started on.
    pub song_step: usize,
}

/// The sound-making part of a channel.
///
/// An enum instead of a trait object (`Box<dyn Instrument>`): there are
/// only three kinds, and the audio thread can `match` on them without any
/// pointer indirection.
#[derive(Debug, Clone)]
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    MidiOut(MidiOut),
}

impl Instrument {
    /// Builds the instrument for a channel of the project. Drums are
    /// synthesized and files are loaded here, on the calling thread; a
    /// file that was loaded before comes from `samples`. A synth takes the
    /// sound of the channel's General MIDI program, and a MIDI Out sends
    /// the program to the external synth.
    pub fn for_channel(
        channel: &Channel,
        sample_rate: u32,
        samples: &mut SampleCache,
    ) -> Result<Self, SampleError> {
        Ok(match &channel.kind {
            ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
                Instrument::Sampler(Sampler::new(Arc::new(drums::render(*drum, sample_rate))))
            }
            ChannelKind::Sampler(SampleSource::File(path)) => {
                Instrument::Sampler(Sampler::new(samples.get(path)?))
            }
            ChannelKind::Synth => Instrument::Synth(Synth::with_timbre(
                sample_rate,
                Timbre::for_program(channel.program),
            )),
            ChannelKind::MidiOut { channel: number } => {
                Instrument::MidiOut(MidiOut::new(*number).with_program(channel.program))
            }
        })
    }

    /// A MIDI Out on `channel` (0 to 15) in place of this instrument, so
    /// its notes play on the external synth instead of the speakers.
    /// With a `pitch`, every note goes out as that note: a drum is one
    /// note of a General MIDI drum set, whatever note its steps are on.
    /// With a `program`, the synth is told to play that General MIDI
    /// instrument. A MIDI Out already sends its notes, so it is given back
    /// as it is.
    pub fn to_midi(self, channel: u8, pitch: Option<Note>, program: Option<u8>) -> Self {
        match self {
            Instrument::MidiOut(_) => self,
            _ => Instrument::MidiOut(
                MidiOut::new(channel)
                    .playing_as(pitch)
                    .with_program(program),
            ),
        }
    }

    /// Starts playing a note.
    ///
    /// `gate` is how many frames the note is held for, or [`HELD`]. The
    /// synth starts fading out when it ends, and the MIDI Out sends a
    /// note-off; the sampler ignores it and always plays its sound to the
    /// end, like a drum machine.
    pub fn note_on(&mut self, pitch: Note, velocity: u8, gate: u32, output_rate: u32) {
        self.start(pitch, velocity, gate, output_rate, None);
    }

    /// Starts a note of the song: like [`Instrument::note_on`], but the
    /// note remembers where in the song it comes from.
    pub fn note_on_from(
        &mut self,
        origin: Origin,
        pitch: Note,
        velocity: u8,
        gate: u32,
        output_rate: u32,
    ) {
        self.start(pitch, velocity, gate, output_rate, Some(origin));
    }

    fn start(
        &mut self,
        pitch: Note,
        velocity: u8,
        gate: u32,
        output_rate: u32,
        origin: Option<Origin>,
    ) {
        match self {
            Instrument::Sampler(sampler) => sampler.start(pitch, velocity, output_rate, origin),
            Instrument::Synth(synth) => synth.start(pitch, velocity, gate, origin),
            Instrument::MidiOut(midi_out) => midi_out.start(pitch, velocity, gate, origin),
        }
    }

    /// Lets go of a note that was started with the [`HELD`] gate, as when
    /// a key comes up.
    pub fn note_off(&mut self, pitch: Note) {
        match self {
            Instrument::Sampler(_) => {} // samples always play to the end
            Instrument::Synth(synth) => synth.note_off(pitch),
            Instrument::MidiOut(midi_out) => midi_out.note_off(pitch),
        }
    }

    /// Lets go of every held note, as when playback stops.
    pub fn release_all(&mut self) {
        match self {
            Instrument::Sampler(sampler) => sampler.release_all(),
            Instrument::Synth(synth) => synth.release_all(),
            Instrument::MidiOut(midi_out) => midi_out.release_all(),
        }
    }

    /// Lets go of the notes of the song that `keep` turns down, given where
    /// each one came from. Keys held on a keyboard and previews did not
    /// come from the song, so they are left alone.
    ///
    /// The audio thread calls this when a channel is muted, and when the
    /// song changes while notes sound, so that a note whose clip was taken
    /// away or whose track was muted stops. A sampler fades such a sound
    /// out quickly, however short it is.
    pub fn release_unless(&mut self, keep: impl Fn(Origin) -> bool) {
        match self {
            Instrument::Sampler(sampler) => sampler.release_unless(keep),
            Instrument::Synth(synth) => synth.release_unless(keep),
            Instrument::MidiOut(midi_out) => midi_out.release_unless(keep),
        }
    }

    /// Makes the notes being held end on time after a change of tempo, so
    /// that each still lasts as many steps as it should.
    ///
    /// A note's length was turned into frames when it started. The step
    /// that is playing keeps its length in frames (see
    /// [`loom_core::Sequencer::set_tempo`]), so the frames up to the next
    /// step, `next_step`, stay as they are; every frame after that is
    /// multiplied by `stretch`, the new length of a step divided by the
    /// old one. The sampler plays its sounds to the end, so it has nothing
    /// to change.
    pub fn tempo_changed(&mut self, next_step: f64, stretch: f64) {
        match self {
            Instrument::Sampler(_) => {}
            Instrument::Synth(synth) => synth.tempo_changed(next_step, stretch),
            Instrument::MidiOut(midi_out) => midi_out.tempo_changed(next_step, stretch),
        }
    }

    /// The next output sample: the sum of every voice that is playing.
    pub fn next_sample(&mut self) -> f32 {
        match self {
            Instrument::Sampler(sampler) => sampler.next_sample(),
            Instrument::Synth(synth) => synth.next_sample(),
            Instrument::MidiOut(midi_out) => midi_out.next_sample(),
        }
    }

    /// Hands the MIDI messages made since the last call to `send`, which
    /// gets each one in order. Only a MIDI Out has any. The audio thread calls this once per block.
    pub fn send_midi(&mut self, send: impl FnMut(MidiMessage)) {
        if let Instrument::MidiOut(midi_out) = self {
            midi_out.flush(send);
        }
    }

    /// How many voices are making sound right now. For a MIDI Out, how
    /// many notes the external synth has been told to play.
    pub fn voices_playing(&self) -> usize {
        match self {
            Instrument::Sampler(sampler) => sampler.voices.iter().filter(|v| !v.is_idle()).count(),
            Instrument::Synth(synth) => synth.voices.iter().filter(|v| !v.is_idle()).count(),
            Instrument::MidiOut(midi_out) => midi_out.notes_sounding(),
        }
    }
}

/// Turns a MIDI velocity (1..=127) into a volume from 0.0 to 1.0.
fn velocity_gain(velocity: u8) -> f32 {
    velocity.min(127) as f32 / 127.0
}

/// How many frames a note has left after a change of tempo, if it had
/// `left` at the old tempo: the frames up to the next step (`next_step`)
/// stay as they are, and the rest are multiplied by `stretch`. Rounded to
/// the nearest frame.
fn stretched(left: f64, next_step: f64, stretch: f64) -> f64 {
    if left <= next_step {
        left
    } else {
        (next_step + (left - next_step) * stretch).round()
    }
}

/// What every kind of voice can tell the voice allocator.
trait Voice {
    /// Whether the voice is silent and free to play a new note.
    fn is_idle(&self) -> bool;
    /// When the voice's note started, as a count of notes started so far.
    fn started(&self) -> u64;
    /// Whether the voice's note has ended and it is only fading out.
    fn is_releasing(&self) -> bool;
}

/// Chooses the voice for a new note: a silent one if there is one,
/// otherwise one that is only fading out, otherwise the one that started
/// longest ago ("voice stealing"). Among the voices fading out, too, the
/// oldest goes first.
///
/// A fading voice is still busy for a while (the Loom Osc's release takes
/// most of a second to reach silence), but cutting its tail is far less
/// noticeable than cutting a note that is still held, such as a long bass
/// note under quick chords.
///
/// The function is generic, so the same code serves both instruments: the
/// compiler makes one copy for `SamplerVoice` and one for `SynthVoice`.
fn pick_voice<V: Voice>(voices: &mut [V]) -> &mut V {
    let oldest = |releasing: bool| {
        voices
            .iter()
            .enumerate()
            .filter(|(_, voice)| !releasing || voice.is_releasing())
            .min_by_key(|(_, voice)| voice.started())
            .map(|(index, _)| index)
    };
    let index = voices
        .iter()
        .position(Voice::is_idle)
        .or_else(|| oldest(true))
        .or_else(|| oldest(false))
        .unwrap_or(0);
    &mut voices[index]
}

/// Plays a sample from start to end for each note.
#[derive(Debug, Clone)]
pub struct Sampler {
    sample: Arc<SampleData>,
    /// Boxed so an `Instrument` stays small: commands carrying one are
    /// copied into the command queue, and a pointer is cheaper to copy
    /// than eight voices. The box is created here, on the UI thread.
    voices: Box<[SamplerVoice; SAMPLER_VOICES]>,
    notes_started: u64,
}

#[derive(Debug, Clone, Default)]
struct SamplerVoice {
    /// Where we are in the sample, in (fractional) frames, or `None` when silent.
    position: Option<f64>,
    /// How many sample frames to advance per output frame. It is 1.0 when
    /// the sample and the sound card use the same rate and the note is
    /// [`STEP_NOTE`]; an octave higher it is 2.0, an octave lower 0.5.
    increment: f64,
    gain: f32,
    started: u64,
    /// While stopping: how loud the voice still is, falling to silence.
    fading: Option<f32>,
    /// Where in the song its note comes from, if it does.
    origin: Option<Origin>,
}

impl Voice for SamplerVoice {
    fn is_idle(&self) -> bool {
        self.position.is_none()
    }

    fn started(&self) -> u64 {
        self.started
    }

    fn is_releasing(&self) -> bool {
        self.fading.is_some()
    }
}

impl SamplerVoice {
    fn next_sample(&mut self, frames: &[f32]) -> f32 {
        let Some(position) = self.position else {
            return 0.0;
        };
        let index = position as usize;
        if index + 1 >= frames.len() {
            self.position = None;
            return frames.get(index).copied().unwrap_or(0.0) * self.gain;
        }
        // Linear interpolation between the two nearest frames.
        let fraction = (position - index as f64) as f32;
        let sample = frames[index] + (frames[index + 1] - frames[index]) * fraction;
        self.position = Some(position + self.increment);
        if let Some(level) = self.fading {
            let level = level * FADE_FACTOR;
            if level < 0.001 {
                self.position = None;
            }
            self.fading = Some(level);
            return sample * self.gain * level;
        }
        sample * self.gain
    }
}

impl Sampler {
    pub fn new(sample: Arc<SampleData>) -> Self {
        Sampler {
            sample,
            voices: Box::default(),
            notes_started: 0,
        }
    }

    pub fn note_on(&mut self, pitch: Note, velocity: u8, output_rate: u32) {
        self.start(pitch, velocity, output_rate, None);
    }

    fn start(&mut self, pitch: Note, velocity: u8, output_rate: u32, origin: Option<Origin>) {
        // Every semitone up plays the sample 2^(1/12) times faster.
        let semitones = pitch.midi() as f64 - STEP_NOTE.midi() as f64;
        let pitch_ratio = 2.0_f64.powf(semitones / 12.0);
        let rate_ratio = self.sample.sample_rate() as f64 / output_rate as f64;

        self.notes_started += 1;
        *pick_voice(&mut self.voices[..]) = SamplerVoice {
            position: Some(0.0),
            increment: rate_ratio * pitch_ratio,
            gain: velocity_gain(velocity),
            started: self.notes_started,
            fading: None,
            origin,
        };
    }

    /// Fades out the voices playing a long sound, such as a whole song
    /// imported as a recording, so that it stops with the transport.
    /// Short sounds (drum hits) still ring out.
    pub fn release_all(&mut self) {
        let long = self.sample.frames().len() as u64
            > u64::from(self.sample.sample_rate()) * LONG_SAMPLE_SECONDS;
        if long {
            for voice in self.voices.iter_mut().filter(|voice| !voice.is_idle()) {
                voice.fading.get_or_insert(1.0);
            }
        }
    }

    /// Fades out the sounds of the song that `keep` turns down, long or
    /// short (see [`Instrument::release_unless`]).
    pub fn release_unless(&mut self, keep: impl Fn(Origin) -> bool) {
        for voice in self.voices.iter_mut().filter(|voice| !voice.is_idle()) {
            if voice.origin.is_some_and(|origin| !keep(origin)) {
                voice.fading.get_or_insert(1.0);
            }
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let frames = self.sample.frames();
        self.voices
            .iter_mut()
            .map(|voice| voice.next_sample(frames))
            .sum()
    }
}

/// Each voice is turned down so that a four-note chord is not too loud.
const SYNTH_VOICE_GAIN: f32 = 0.35;
/// Below this volume (about -80 dB) a releasing voice counts as silent.
const SILENT: f32 = 0.000_1;

/// The Loom Osc: oscillators with a soft filter and an
/// attack-decay-sustain-release envelope, one per voice. Its [`Timbre`]
/// says which wave, filter and envelope.
#[derive(Debug, Clone)]
pub struct Synth {
    sample_rate: f32,
    voices: Box<[SynthVoice; SYNTH_VOICES]>,
    /// Boxed, like the voices, so an `Instrument` stays small.
    envelope: Box<EnvelopeRates>,
    notes_started: u64,
}

/// The envelope's speeds, worked out once per sample rate, and the rest
/// of the timbre.
#[derive(Debug, Clone, Copy)]
struct EnvelopeRates {
    /// How much the volume grows per frame during the attack.
    attack_step: f32,
    /// Each frame of the decay keeps this fraction of the distance to the sustain level.
    decay_factor: f32,
    /// Each frame of the release keeps this fraction of the volume.
    release_factor: f32,
    /// The volume a held note settles at, after the decay.
    sustain: f32,
    wave: Wave,
    /// How loud this timbre is, compared with the Loom Osc's own sound.
    gain: f32,
}

impl EnvelopeRates {
    fn new(sample_rate: f32, timbre: &Timbre) -> Self {
        EnvelopeRates {
            attack_step: 1.0 / (timbre.attack * sample_rate),
            decay_factor: (-1.0 / (timbre.decay * sample_rate)).exp(),
            release_factor: (-1.0 / (timbre.release * sample_rate)).exp(),
            sustain: timbre.sustain,
            wave: timbre.wave,
            gain: timbre.gain,
        }
    }
}

/// Where a voice's volume envelope is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Idle,
    /// Rising to full volume.
    Attack,
    /// Falling towards the sustain level, and staying there while held.
    Decay,
    /// Fading to silence after the note ended.
    Release,
}

#[derive(Debug, Clone)]
struct SynthVoice {
    /// The note being played, so `note_off` can find its voice.
    pitch: Note,
    frequency: f32,
    phase: f32,
    filter: LowPass,
    gain: f32,
    /// Frames left until the note is released, or [`HELD`].
    gate: u32,
    stage: Stage,
    level: f32,
    started: u64,
    /// Where in the song its note comes from, if it does.
    origin: Option<Origin>,
}

impl Voice for SynthVoice {
    fn is_idle(&self) -> bool {
        self.stage == Stage::Idle
    }

    fn started(&self) -> u64 {
        self.started
    }

    fn is_releasing(&self) -> bool {
        self.stage == Stage::Release
    }
}

impl SynthVoice {
    fn new(sample_rate: u32, cutoff: f32) -> Self {
        SynthVoice {
            pitch: Note::C4,
            frequency: 0.0,
            phase: 0.0,
            filter: LowPass::new(cutoff, sample_rate),
            gain: 0.0,
            gate: 0,
            stage: Stage::Idle,
            level: 0.0,
            started: 0,
            origin: None,
        }
    }

    /// Whether the note is still held: started, and not let go of yet.
    fn is_held(&self) -> bool {
        matches!(self.stage, Stage::Attack | Stage::Decay)
    }

    fn next_sample(&mut self, envelope: &EnvelopeRates, sample_rate: f32) -> f32 {
        match self.stage {
            Stage::Idle => return 0.0,
            Stage::Attack => {
                self.level += envelope.attack_step;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                let sustain = envelope.sustain;
                self.level = sustain + (self.level - sustain) * envelope.decay_factor;
            }
            Stage::Release => {
                self.level *= envelope.release_factor;
                if self.level < SILENT {
                    self.stage = Stage::Idle;
                    return 0.0;
                }
            }
        }

        // Count down the gate while the note is held, then let go. A
        // `HELD` gate is not counted: only `note_off` ends it.
        if self.is_held() && self.gate != HELD {
            match self.gate.checked_sub(1) {
                Some(left) => self.gate = left,
                None => self.stage = Stage::Release,
            }
        }

        let value = envelope.wave.at(self.phase);
        self.phase = (self.phase + self.frequency / sample_rate).fract();
        self.filter.process(value) * self.level * self.gain
    }
}

impl Synth {
    /// A Loom Osc with its own sound.
    pub fn new(sample_rate: u32) -> Self {
        Synth::with_timbre(sample_rate, Timbre::LOOM_OSC)
    }

    /// A Loom Osc with the sound `timbre`.
    pub fn with_timbre(sample_rate: u32, timbre: Timbre) -> Self {
        Synth {
            sample_rate: sample_rate as f32,
            // `from_fn` builds an array by calling the closure once per element.
            voices: Box::new(std::array::from_fn(|_| {
                SynthVoice::new(sample_rate, timbre.cutoff)
            })),
            envelope: Box::new(EnvelopeRates::new(sample_rate as f32, &timbre)),
            notes_started: 0,
        }
    }

    pub fn note_on(&mut self, pitch: Note, velocity: u8, gate: u32) {
        self.start(pitch, velocity, gate, None);
    }

    fn start(&mut self, pitch: Note, velocity: u8, gate: u32, origin: Option<Origin>) {
        self.notes_started += 1;
        let voice = pick_voice(&mut self.voices[..]);
        voice.pitch = pitch;
        voice.frequency = pitch.frequency();
        voice.gain = velocity_gain(velocity) * SYNTH_VOICE_GAIN * self.envelope.gain;
        voice.gate = gate;
        voice.stage = Stage::Attack;
        voice.started = self.notes_started;
        voice.origin = origin;
        // The level, phase and filter carry on from where they were, so a
        // stolen voice does not jump (and click).
    }

    /// Releases the held notes of `pitch`. Notes of the same pitch that
    /// the sequencer started keep their own gate.
    pub fn note_off(&mut self, pitch: Note) {
        for voice in self.voices.iter_mut() {
            if voice.pitch == pitch && voice.gate == HELD && voice.is_held() {
                voice.stage = Stage::Release;
            }
        }
    }

    pub fn release_all(&mut self) {
        for voice in self.voices.iter_mut() {
            if voice.is_held() {
                voice.stage = Stage::Release;
            }
        }
    }

    /// Releases the held notes of the song that `keep` turns down (see
    /// [`Instrument::release_unless`]).
    pub fn release_unless(&mut self, keep: impl Fn(Origin) -> bool) {
        for voice in self.voices.iter_mut() {
            if voice.is_held() && voice.origin.is_some_and(|origin| !keep(origin)) {
                voice.stage = Stage::Release;
            }
        }
    }

    /// Stretches the gates of the held notes after a change of tempo (see
    /// [`Instrument::tempo_changed`]).
    pub fn tempo_changed(&mut self, next_step: f64, stretch: f64) {
        for voice in self.voices.iter_mut() {
            if voice.is_held() && voice.gate != HELD {
                let left = stretched(f64::from(voice.gate), next_step, stretch);
                // Never as long as `HELD`, which would hold it for ever.
                voice.gate = left.min(f64::from(HELD - 1)) as u32;
            }
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut sum = 0.0;
        for voice in self.voices.iter_mut() {
            sum += voice.next_sample(&self.envelope, self.sample_rate);
        }
        sum
    }
}

/// How many MIDI messages a MIDI Out can make in one block before the
/// audio thread collects them. A block is a few milliseconds; even a
/// sixteen-note chord starting while sixteen others end is only 32.
const OUTBOX_SIZE: usize = 256;

/// Plays nothing itself: it turns notes into MIDI messages for an external
/// synth (a keyboard, a hardware sound module, or another program).
///
/// The audio thread may not talk to the MIDI system, which can take any
/// amount of time. So the messages wait in an *outbox*, and once per
/// block the audio thread passes them on to a queue that another thread
/// empties into the MIDI port.
#[derive(Debug, Clone)]
pub struct MidiOut {
    /// The MIDI channel, 0 to 15.
    channel: u8,
    /// The note every note goes out as, for a drum.
    pitch: Option<Note>,
    /// The General MIDI program the synth should play, if any.
    program: Option<u8>,
    /// Whether the program change still has to be sent: before the first
    /// note, and again after every stop, in case the synth's sound was
    /// changed on the synth itself in between.
    program_due: bool,
    /// Frames played so far: the clock that says when a note-off is due.
    now: u64,
    /// Boxed so an `Instrument` stays small, as the voices of the others are.
    state: Box<MidiOutState>,
}

#[derive(Debug, Clone)]
struct MidiOutState {
    /// For each of the 128 MIDI notes, whether it is sounding and until when.
    notes: [Sounding; 128],
    /// For each of the 128 MIDI notes, where in the song the note
    /// sounding comes from, if it does.
    origins: [Option<Origin>; 128],
    /// Messages not yet collected, with room for [`OUTBOX_SIZE`] reserved
    /// up front, so pushing never allocates.
    outbox: Vec<MidiMessage>,
}

/// Whether a note of a MIDI Out is playing on the external synth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sounding {
    Off,
    /// Until this frame, when its note-off is due.
    Until(u64),
    /// Until `note_off`.
    Held,
}

impl MidiOut {
    /// A MIDI Out on `channel`, 0 to 15. Called on the UI thread: this is
    /// where the outbox's memory is reserved.
    pub fn new(channel: u8) -> Self {
        MidiOut {
            channel: channel.min(15),
            pitch: None,
            program: None,
            program_due: false,
            now: 0,
            state: Box::new(MidiOutState {
                notes: [Sounding::Off; 128],
                origins: [None; 128],
                outbox: Vec::with_capacity(OUTBOX_SIZE),
            }),
        }
    }

    /// The same MIDI Out, sending every note as `pitch` if there is one.
    pub fn playing_as(mut self, pitch: Option<Note>) -> Self {
        self.pitch = pitch;
        self
    }

    /// The same MIDI Out, having the synth play General MIDI program
    /// `program`, if there is one.
    pub fn with_program(mut self, program: Option<u8>) -> Self {
        self.program = program.map(|program| program.min(127));
        self.program_due = self.program.is_some();
        self
    }

    /// Adds a message to the outbox. If the outbox is full (it never
    /// should be), the message is lost rather than growing the `Vec`.
    fn post(&mut self, message: MidiMessage) {
        let outbox = &mut self.state.outbox;
        if outbox.len() < outbox.capacity() {
            outbox.push(message);
        }
    }

    fn note_off_message(&self, pitch: Note) -> MidiMessage {
        MidiMessage::NoteOff {
            channel: self.channel,
            pitch,
        }
    }

    pub fn note_on(&mut self, pitch: Note, velocity: u8, gate: u32) {
        self.start(pitch, velocity, gate, None);
    }

    fn start(&mut self, pitch: Note, velocity: u8, gate: u32, origin: Option<Origin>) {
        if self.program_due
            && let Some(program) = self.program
        {
            self.post(MidiMessage::ProgramChange {
                channel: self.channel,
                program,
            });
            self.program_due = false;
        }
        let pitch = self.pitch.unwrap_or(pitch);
        let slot = usize::from(pitch.midi());
        // MIDI has one of each note per channel. If this one is already
        // sounding, end it first, or the synth would hear "on, on, off"
        // and the new note would stop at the old one's note-off.
        if self.state.notes[slot] != Sounding::Off {
            self.post(self.note_off_message(pitch));
        }
        self.post(MidiMessage::NoteOn {
            channel: self.channel,
            pitch,
            velocity,
        });
        self.state.notes[slot] = if gate == HELD {
            Sounding::Held
        } else {
            Sounding::Until(self.now + u64::from(gate))
        };
        self.state.origins[slot] = origin;
    }

    pub fn note_off(&mut self, pitch: Note) {
        let pitch = self.pitch.unwrap_or(pitch);
        let slot = usize::from(pitch.midi());
        if self.state.notes[slot] == Sounding::Held {
            self.post(self.note_off_message(pitch));
            self.state.notes[slot] = Sounding::Off;
        }
    }

    /// Ends every note that is sounding, held or not.
    pub fn release_all(&mut self) {
        self.program_due = self.program.is_some();
        for index in 0..128 {
            if self.state.notes[index] != Sounding::Off {
                self.post(self.note_off_message(Note::new(index as u8)));
                self.state.notes[index] = Sounding::Off;
            }
        }
    }

    /// Ends the notes of the song that `keep` turns down (see
    /// [`Instrument::release_unless`]). A key held down is `Held`, with no
    /// origin, so it stays.
    pub fn release_unless(&mut self, keep: impl Fn(Origin) -> bool) {
        for index in 0..128 {
            let from_song =
                self.state.origins[index].filter(|_| self.state.notes[index] != Sounding::Off);
            if from_song.is_some_and(|origin| !keep(origin)) {
                self.post(self.note_off_message(Note::new(index as u8)));
                self.state.notes[index] = Sounding::Off;
            }
        }
    }

    /// Moves the note-offs that are due after a change of tempo (see
    /// [`Instrument::tempo_changed`]).
    pub fn tempo_changed(&mut self, next_step: f64, stretch: f64) {
        for sounding in self.state.notes.iter_mut() {
            if let Sounding::Until(end) = sounding {
                let left = end.saturating_sub(self.now) as f64;
                *end = self.now + stretched(left, next_step, stretch) as u64;
            }
        }
    }

    /// A MIDI Out makes no sound of its own, but it counts the frames.
    pub fn next_sample(&mut self) -> f32 {
        self.now += 1;
        0.0
    }

    /// Posts the note-offs that are due, then hands over the outbox.
    ///
    /// Note-offs are checked once per block, not on every frame, so they
    /// can be up to one block late: a few milliseconds, which nobody
    /// hears at the end of a note.
    pub fn flush(&mut self, mut send: impl FnMut(MidiMessage)) {
        for index in 0..128 {
            if let Sounding::Until(end) = self.state.notes[index]
                && end <= self.now
            {
                self.post(self.note_off_message(Note::new(index as u8)));
                self.state.notes[index] = Sounding::Off;
            }
        }
        // `drain(..)` empties the `Vec` but keeps its memory for next time.
        for message in self.state.outbox.drain(..) {
            send(message);
        }
    }

    fn notes_sounding(&self) -> usize {
        self.state
            .notes
            .iter()
            .filter(|&&sounding| sounding != Sounding::Off)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 48_000;

    fn render(instrument: &mut Instrument, frames: usize) -> Vec<f32> {
        (0..frames).map(|_| instrument.next_sample()).collect()
    }

    fn synth() -> Instrument {
        Instrument::for_channel(&Channel::synth(), RATE, &mut SampleCache::new()).unwrap()
    }

    /// A channel of kind `kind`, with no program.
    fn channel(kind: ChannelKind) -> Channel {
        Channel {
            kind,
            ..Channel::synth()
        }
    }

    /// Counts how often the signal changes sign: about twice per cycle.
    fn sign_changes(samples: &[f32]) -> usize {
        samples
            .windows(2)
            .filter(|pair| (pair[0] < 0.0) != (pair[1] < 0.0))
            .count()
    }

    #[test]
    fn silent_until_a_note_starts() {
        let mut synth = synth();
        assert!(render(&mut synth, 1_000).iter().all(|&s| s == 0.0));
        synth.note_on(Note::A4, 100, RATE, RATE);
        assert!(render(&mut synth, 1_000).iter().any(|&s| s != 0.0));
    }

    #[test]
    fn synth_plays_the_right_pitch() {
        let mut synth = synth();
        synth.note_on(Note::A4, 100, RATE, RATE);
        let one_second = render(&mut synth, RATE as usize);
        let changes = sign_changes(&one_second);
        // A4 is 440 Hz: about 880 sign changes in one second.
        assert!((860..=900).contains(&changes), "got {changes}");
    }

    #[test]
    fn synth_stops_after_the_gate_and_release() {
        let mut synth = synth();
        let gate = RATE / 10; // held for 0.1 s
        synth.note_on(Note::C4, 100, gate, RATE);
        let out = render(&mut synth, RATE as usize);
        assert!(out[..gate as usize].iter().any(|&s| s != 0.0));
        // After the release (10 time constants is plenty) it is silent.
        let quiet_from = gate as usize + (10.0 * Timbre::LOOM_OSC.release * RATE as f32) as usize;
        assert!(out[quiet_from..].iter().all(|&s| s == 0.0));
        assert_eq!(synth.voices_playing(), 0);
    }

    #[test]
    fn release_all_fades_held_notes() {
        let mut synth = synth();
        synth.note_on(Note::C4, 100, u32::MAX, RATE);
        render(&mut synth, 1_000);
        synth.release_all();
        render(&mut synth, RATE as usize);
        assert_eq!(synth.voices_playing(), 0);
    }

    #[test]
    fn synth_plays_chords() {
        let mut synth = synth();
        for midi in [60, 64, 67] {
            synth.note_on(Note::new(midi), 100, RATE, RATE);
        }
        assert_eq!(synth.voices_playing(), 3);
    }

    #[test]
    fn voices_are_stolen_when_all_are_busy() {
        let mut synth = synth();
        for midi in 40..40 + SYNTH_VOICES as u8 + 4 {
            synth.note_on(Note::new(midi), 100, RATE, RATE);
        }
        assert_eq!(synth.voices_playing(), SYNTH_VOICES);
    }

    #[test]
    fn voices_that_are_fading_out_are_stolen_first() {
        let mut synth = Synth::new(RATE);
        // A long bass note, then fifteen short notes: all sixteen voices.
        synth.note_on(Note::new(36), 100, 10 * RATE);
        for midi in 60..75 {
            synth.note_on(Note::new(midi), 100, 100);
        }
        // The short notes have ended, but are still fading out.
        (0..1_000).for_each(|_| _ = synth.next_sample());
        assert!(synth.voices.iter().all(|voice| !voice.is_idle()));
        // A new note takes one of them, not the bass that is still held.
        synth.note_on(Note::new(80), 100, RATE);
        let held = |midi| {
            synth
                .voices
                .iter()
                .any(|voice| voice.pitch == Note::new(midi) && voice.is_held())
        };
        assert!(held(36), "the bass is still held");
        assert!(held(80));
    }

    #[test]
    fn a_new_tempo_stretches_the_gates() {
        let mut synth = Synth::new(RATE);
        synth.note_on(Note::C4, 100, 1_000);
        synth.note_on(Note::new(64), 100, 50);
        synth.note_on(Note::new(67), 100, HELD);
        // Twice as fast, 100 frames before the next step: the C4 keeps
        // those 100 frames, then the other 900 take half as long.
        synth.tempo_changed(100.0, 0.5);
        let gates: Vec<u32> = synth.voices[..3].iter().map(|voice| voice.gate).collect();
        assert_eq!(
            gates,
            [550, 50, HELD],
            "a note ending before the step keeps its gate"
        );
    }

    #[test]
    fn a_new_tempo_moves_midi_note_offs() {
        let mut midi_out = MidiOut::new(0);
        midi_out.note_on(Note::C4, 100, 1_000);
        sent(&mut midi_out);
        (0..100).for_each(|_| _ = midi_out.next_sample());
        // Half as fast, with the next step 300 frames away: the 900
        // frames left become 300 + 600 * 2.
        midi_out.tempo_changed(300.0, 2.0);
        (0..1_499).for_each(|_| _ = midi_out.next_sample());
        assert_eq!(sent(&mut midi_out), [], "one frame early");
        midi_out.next_sample();
        assert_eq!(sent(&mut midi_out).len(), 1);
    }

    #[test]
    fn release_unless_lets_go_of_the_song_and_not_the_keys() {
        let origin = |pattern| Origin {
            pattern,
            pattern_step: 0,
            song_step: 0,
        };
        let mut midi_out = Instrument::MidiOut(MidiOut::new(0));
        midi_out.note_on_from(origin(0), Note::C4, 100, 1_000, RATE);
        midi_out.note_on_from(origin(1), Note::new(64), 100, 1_000, RATE);
        midi_out.note_on(Note::new(67), 100, HELD, RATE);
        sent_by(&mut midi_out);
        // Only pattern 1 is still played.
        midi_out.release_unless(|origin| origin.pattern == 1);
        let off = MidiMessage::NoteOff {
            channel: 0,
            pitch: Note::C4,
        };
        assert_eq!(sent_by(&mut midi_out), [off]);
        assert_eq!(midi_out.voices_playing(), 2, "E4 and the key held down");
    }

    #[test]
    fn sampler_plays_once_then_stops() {
        let sample = Arc::new(SampleData::new(vec![0.5; 100], RATE));
        let mut sampler = Instrument::Sampler(Sampler::new(sample));
        sampler.note_on(STEP_NOTE, 127, 0, RATE);
        let out = render(&mut sampler, 200);
        assert!(out[..99].iter().all(|&s| s == 0.5));
        assert!(out[101..].iter().all(|&s| s == 0.0));
    }

    #[test]
    fn sampler_resamples_to_the_output_rate() {
        // One second recorded at 24 kHz must last one second at 48 kHz.
        let sample = Arc::new(SampleData::new(vec![0.5; 24_000], 24_000));
        let mut sampler = Instrument::Sampler(Sampler::new(sample));
        sampler.note_on(STEP_NOTE, 100, 0, RATE);
        let playing = render(&mut sampler, 50_000)
            .iter()
            .filter(|&&s| s != 0.0)
            .count();
        assert!(
            (47_990..=48_000).contains(&playing),
            "played {playing} frames"
        );
    }

    #[test]
    fn sampler_plays_higher_notes_faster() {
        // An octave up, a one-second sample lasts half a second.
        let sample = Arc::new(SampleData::new(vec![0.5; 48_000], RATE));
        let mut sampler = Instrument::Sampler(Sampler::new(sample));
        let octave_up = STEP_NOTE.transpose(12).unwrap();
        sampler.note_on(octave_up, 100, 0, RATE);
        let playing = render(&mut sampler, 48_000)
            .iter()
            .filter(|&&s| s != 0.0)
            .count();
        assert!(
            (23_990..=24_000).contains(&playing),
            "played {playing} frames"
        );
    }

    #[test]
    fn sampler_hits_can_overlap() {
        let sample = Arc::new(SampleData::new(vec![0.25; 1_000], RATE));
        let mut sampler = Instrument::Sampler(Sampler::new(sample));
        sampler.note_on(STEP_NOTE, 127, 0, RATE);
        render(&mut sampler, 100);
        sampler.note_on(STEP_NOTE, 127, 0, RATE);
        assert_eq!(sampler.voices_playing(), 2);
        assert_eq!(render(&mut sampler, 1)[0], 0.5, "both hits add up");
    }

    #[test]
    fn held_notes_last_until_note_off() {
        let mut synth = synth();
        synth.note_on(Note::C4, 100, HELD, RATE);
        synth.note_on(Note::new(64), 100, HELD, RATE);
        render(&mut synth, 2 * RATE as usize);
        assert_eq!(synth.voices_playing(), 2);
        synth.note_off(Note::C4);
        render(&mut synth, RATE as usize);
        assert_eq!(synth.voices_playing(), 1, "E4 (64) is still held");
    }

    /// Everything a MIDI Out has made so far.
    fn sent(midi_out: &mut MidiOut) -> Vec<MidiMessage> {
        let mut messages = Vec::new();
        midi_out.flush(|message| messages.push(message));
        messages
    }

    #[test]
    fn midi_out_ends_notes_after_their_gate() {
        let mut midi_out = MidiOut::new(2);
        midi_out.note_on(Note::C4, 90, 100);
        let on = MidiMessage::NoteOn {
            channel: 2,
            pitch: Note::C4,
            velocity: 90,
        };
        assert_eq!(sent(&mut midi_out), [on]);
        (0..99).for_each(|_| _ = midi_out.next_sample());
        assert_eq!(sent(&mut midi_out), [], "one frame early");
        midi_out.next_sample();
        let off = MidiMessage::NoteOff {
            channel: 2,
            pitch: Note::C4,
        };
        assert_eq!(sent(&mut midi_out), [off]);
    }

    #[test]
    fn midi_out_ends_a_repeated_note_before_starting_it_again() {
        let mut midi_out = MidiOut::new(0);
        midi_out.note_on(Note::C4, 100, HELD);
        midi_out.note_on(Note::C4, 100, HELD);
        let kinds: Vec<bool> = sent(&mut midi_out)
            .iter()
            .map(|message| matches!(message, MidiMessage::NoteOn { .. }))
            .collect();
        assert_eq!(kinds, [true, false, true]);
        midi_out.release_all();
        assert_eq!(sent(&mut midi_out).len(), 1);
        assert_eq!(midi_out.notes_sounding(), 0);
    }

    #[test]
    fn missing_file_reports_an_error() {
        let kind = ChannelKind::Sampler(SampleSource::File("/no/such.wav".into()));
        assert!(Instrument::for_channel(&channel(kind), RATE, &mut SampleCache::new()).is_err());
    }

    /// Everything an instrument has sent through MIDI so far.
    fn sent_by(instrument: &mut Instrument) -> Vec<MidiMessage> {
        let mut messages = Vec::new();
        instrument.send_midi(|message| messages.push(message));
        messages
    }

    #[test]
    fn a_synth_sent_to_midi_is_silent_and_sends_its_notes() {
        let mut echoed = synth().to_midi(4, None, None);
        echoed.note_on(Note::new(64), 90, 100, RATE);
        assert!(render(&mut echoed, 1_000).iter().all(|&s| s == 0.0));
        let on = MidiMessage::NoteOn {
            channel: 4,
            pitch: Note::new(64),
            velocity: 90,
        };
        let off = MidiMessage::NoteOff {
            channel: 4,
            pitch: Note::new(64),
        };
        assert_eq!(sent_by(&mut echoed), [on, off], "the gate was 100 frames");
    }

    #[test]
    fn a_drum_sent_to_midi_sends_its_own_note_and_lets_go_of_held_keys() {
        let kind = ChannelKind::Sampler(SampleSource::Builtin(loom_core::Drum::Kick));
        let kick = Instrument::for_channel(&channel(kind), RATE, &mut SampleCache::new()).unwrap();
        let mut echoed = kick.to_midi(9, Some(Note::new(36)), None);
        // A key held on the typing keyboard: the step note, C4.
        echoed.note_on(STEP_NOTE, 100, HELD, RATE);
        echoed.note_off(STEP_NOTE);
        let pitches: Vec<(bool, u8, u8)> = sent_by(&mut echoed)
            .into_iter()
            .map(|message| match message {
                MidiMessage::NoteOn { channel, pitch, .. } => (true, channel, pitch.midi()),
                MidiMessage::NoteOff { channel, pitch } => (false, channel, pitch.midi()),
                MidiMessage::ProgramChange { .. } => panic!("a drum has no program"),
            })
            .collect();
        assert_eq!(pitches, [(true, 9, 36), (false, 9, 36)]);
    }

    #[test]
    fn sending_a_midi_out_to_midi_changes_nothing() {
        let midi_out = Instrument::MidiOut(MidiOut::new(2));
        assert!(matches!(
            midi_out.to_midi(5, None, None),
            Instrument::MidiOut(_)
        ));
    }

    #[test]
    fn the_program_goes_out_before_the_first_note_and_after_every_stop() {
        let violin = Some(loom_core::general_midi::VIOLIN);
        let mut echoed = synth().to_midi(4, None, violin);
        let change = MidiMessage::ProgramChange {
            channel: 4,
            program: 40,
        };
        let starts = |echoed: &mut Instrument| {
            echoed.note_on(Note::new(64), 90, HELD, RATE);
            echoed.note_on(Note::new(67), 90, HELD, RATE);
            let sent = sent_by(echoed);
            sent.iter().filter(|message| **message == change).count()
        };
        assert_eq!(sent_by(&mut echoed), [], "nothing before a note");
        assert_eq!(starts(&mut echoed), 1);
        assert_eq!(starts(&mut echoed), 0, "once is enough");
        echoed.release_all();
        assert_eq!(starts(&mut echoed), 1, "again after a stop");
    }

    #[test]
    fn a_synth_with_a_program_sounds_like_it() {
        let mut piano = Channel::synth();
        piano.program = Some(loom_core::general_midi::ACOUSTIC_GRAND_PIANO);
        let mut piano = Instrument::for_channel(&piano, RATE, &mut SampleCache::new()).unwrap();
        let mut osc = synth();
        // A note held for four seconds: the piano dies away, the Loom Osc
        // keeps sounding.
        for instrument in [&mut piano, &mut osc] {
            instrument.note_on(Note::A4, 100, HELD, RATE);
        }
        let loudness = |instrument: &mut Instrument| {
            let late = render(instrument, 4 * RATE as usize);
            late[late.len() - 4_800..]
                .iter()
                .map(|s| s.abs())
                .fold(0.0, f32::max)
        };
        assert!(loudness(&mut piano) < 0.1 * loudness(&mut osc));
    }
}
