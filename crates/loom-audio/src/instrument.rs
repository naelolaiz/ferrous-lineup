//! The instruments a channel can play: a sampler, the Loom Osc synth,
//! and a MIDI Out that has an external synth play the notes.
//!
//! The sampler and the synth are *polyphonic*: they have a fixed number
//! of voices, and each note that starts takes one. That is what lets the
//! synth play chords and lets a drum hit ring on while the next one
//! starts.

use std::sync::Arc;

use loom_core::{ChannelKind, MidiMessage, Note, STEP_NOTE, SampleSource};

use crate::drums;
use crate::dsp::LowPass;
use crate::sample::{SampleCache, SampleData, SampleError};

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
    /// file that was loaded before comes from `samples`.
    pub fn for_channel(
        kind: &ChannelKind,
        sample_rate: u32,
        samples: &mut SampleCache,
    ) -> Result<Self, SampleError> {
        Ok(match kind {
            ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
                Instrument::Sampler(Sampler::new(Arc::new(drums::render(*drum, sample_rate))))
            }
            ChannelKind::Sampler(SampleSource::File(path)) => {
                Instrument::Sampler(Sampler::new(samples.get(path)?))
            }
            ChannelKind::Synth => Instrument::Synth(Synth::new(sample_rate)),
            ChannelKind::MidiOut { channel } => Instrument::MidiOut(MidiOut::new(*channel)),
        })
    }

    /// Starts playing a note.
    ///
    /// `gate` is how many frames the note is held for, or [`HELD`]. The
    /// synth starts fading out when it ends, and the MIDI Out sends a
    /// note-off; the sampler ignores it and always plays its sound to the
    /// end, like a drum machine.
    pub fn note_on(&mut self, pitch: Note, velocity: u8, gate: u32, output_rate: u32) {
        match self {
            Instrument::Sampler(sampler) => sampler.note_on(pitch, velocity, output_rate),
            Instrument::Synth(synth) => synth.note_on(pitch, velocity, gate),
            Instrument::MidiOut(midi_out) => midi_out.note_on(pitch, velocity, gate),
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

    /// The next output sample: the sum of every voice that is playing.
    pub fn next_sample(&mut self) -> f32 {
        match self {
            Instrument::Sampler(sampler) => sampler.next_sample(),
            Instrument::Synth(synth) => synth.next_sample(),
            Instrument::MidiOut(midi_out) => midi_out.next_sample(),
        }
    }

    /// Hands the MIDI messages made since the last call to `send`, which
    /// gets each one in order. Only a MIDI Out has any. The audio thread
    /// calls this once per block.
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

/// What every kind of voice can tell the voice allocator.
trait Voice {
    /// Whether the voice is silent and free to play a new note.
    fn is_idle(&self) -> bool;
    /// When the voice's note started, as a count of notes started so far.
    fn started(&self) -> u64;
}

/// Chooses the voice for a new note: a silent one if there is one,
/// otherwise the one that started longest ago ("voice stealing").
///
/// The function is generic, so the same code serves both instruments: the
/// compiler makes one copy for `SamplerVoice` and one for `SynthVoice`.
fn pick_voice<V: Voice>(voices: &mut [V]) -> &mut V {
    let index = voices
        .iter()
        .position(Voice::is_idle)
        .or_else(|| {
            voices
                .iter()
                .enumerate()
                .min_by_key(|(_, voice)| voice.started())
                .map(|(index, _)| index)
        })
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
}

impl Voice for SamplerVoice {
    fn is_idle(&self) -> bool {
        self.position.is_none()
    }

    fn started(&self) -> u64 {
        self.started
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

    pub fn next_sample(&mut self) -> f32 {
        let frames = self.sample.frames();
        self.voices
            .iter_mut()
            .map(|voice| voice.next_sample(frames))
            .sum()
    }
}

/// How long a Loom Osc note takes to reach full volume, in seconds.
const SYNTH_ATTACK: f32 = 0.004;
/// How quickly the volume falls from the peak to the sustain level (time constant).
const SYNTH_DECAY: f32 = 0.3;
/// The volume a held note settles at, after the decay.
const SYNTH_SUSTAIN: f32 = 0.45;
/// How quickly a note fades after it is released (time constant).
const SYNTH_RELEASE: f32 = 0.08;
/// Each voice is turned down so that a four-note chord is not too loud.
const SYNTH_VOICE_GAIN: f32 = 0.35;
/// Below this volume (about -80 dB) a releasing voice counts as silent.
const SILENT: f32 = 0.000_1;

/// The Loom Osc: sawtooth oscillators with a soft filter and an
/// attack-decay-sustain-release envelope, one per voice.
#[derive(Debug, Clone)]
pub struct Synth {
    sample_rate: f32,
    voices: Box<[SynthVoice; SYNTH_VOICES]>,
    envelope: EnvelopeRates,
    notes_started: u64,
}

/// The envelope's speeds, worked out once per sample rate.
#[derive(Debug, Clone, Copy)]
struct EnvelopeRates {
    /// How much the volume grows per frame during the attack.
    attack_step: f32,
    /// Each frame of the decay keeps this fraction of the distance to the sustain level.
    decay_factor: f32,
    /// Each frame of the release keeps this fraction of the volume.
    release_factor: f32,
}

impl EnvelopeRates {
    fn new(sample_rate: f32) -> Self {
        EnvelopeRates {
            attack_step: 1.0 / (SYNTH_ATTACK * sample_rate),
            decay_factor: (-1.0 / (SYNTH_DECAY * sample_rate)).exp(),
            release_factor: (-1.0 / (SYNTH_RELEASE * sample_rate)).exp(),
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
}

impl Voice for SynthVoice {
    fn is_idle(&self) -> bool {
        self.stage == Stage::Idle
    }

    fn started(&self) -> u64 {
        self.started
    }
}

impl SynthVoice {
    fn new(sample_rate: u32) -> Self {
        SynthVoice {
            pitch: Note::C4,
            frequency: 0.0,
            phase: 0.0,
            filter: LowPass::new(2_500.0, sample_rate),
            gain: 0.0,
            gate: 0,
            stage: Stage::Idle,
            level: 0.0,
            started: 0,
        }
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
                self.level = SYNTH_SUSTAIN + (self.level - SYNTH_SUSTAIN) * envelope.decay_factor;
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
        if matches!(self.stage, Stage::Attack | Stage::Decay) && self.gate != HELD {
            match self.gate.checked_sub(1) {
                Some(left) => self.gate = left,
                None => self.stage = Stage::Release,
            }
        }

        // A sawtooth ramps from -1 to 1 once per cycle.
        let saw = self.phase * 2.0 - 1.0;
        self.phase = (self.phase + self.frequency / sample_rate).fract();
        self.filter.process(saw) * self.level * self.gain
    }
}

impl Synth {
    pub fn new(sample_rate: u32) -> Self {
        Synth {
            sample_rate: sample_rate as f32,
            // `from_fn` builds an array by calling the closure once per element.
            voices: Box::new(std::array::from_fn(|_| SynthVoice::new(sample_rate))),
            envelope: EnvelopeRates::new(sample_rate as f32),
            notes_started: 0,
        }
    }

    pub fn note_on(&mut self, pitch: Note, velocity: u8, gate: u32) {
        self.notes_started += 1;
        let voice = pick_voice(&mut self.voices[..]);
        voice.pitch = pitch;
        voice.frequency = pitch.frequency();
        voice.gain = velocity_gain(velocity) * SYNTH_VOICE_GAIN;
        voice.gate = gate;
        voice.stage = Stage::Attack;
        voice.started = self.notes_started;
        // The level, phase and filter carry on from where they were, so a
        // stolen voice does not jump (and click).
    }

    /// Releases the held notes of `pitch`. Notes of the same pitch that
    /// the sequencer started keep their own gate.
    pub fn note_off(&mut self, pitch: Note) {
        for voice in self.voices.iter_mut() {
            if voice.pitch == pitch
                && voice.gate == HELD
                && matches!(voice.stage, Stage::Attack | Stage::Decay)
            {
                voice.stage = Stage::Release;
            }
        }
    }

    pub fn release_all(&mut self) {
        for voice in self.voices.iter_mut() {
            if matches!(voice.stage, Stage::Attack | Stage::Decay) {
                voice.stage = Stage::Release;
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
    /// Frames played so far: the clock that says when a note-off is due.
    now: u64,
    /// Boxed so an `Instrument` stays small, as the voices of the others are.
    state: Box<MidiOutState>,
}

#[derive(Debug, Clone)]
struct MidiOutState {
    /// For each of the 128 MIDI notes, whether it is sounding and until when.
    notes: [Sounding; 128],
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
            now: 0,
            state: Box::new(MidiOutState {
                notes: [Sounding::Off; 128],
                outbox: Vec::with_capacity(OUTBOX_SIZE),
            }),
        }
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
    }

    pub fn note_off(&mut self, pitch: Note) {
        let slot = usize::from(pitch.midi());
        if self.state.notes[slot] == Sounding::Held {
            self.post(self.note_off_message(pitch));
            self.state.notes[slot] = Sounding::Off;
        }
    }

    /// Ends every note that is sounding, held or not.
    pub fn release_all(&mut self) {
        for index in 0..128 {
            if self.state.notes[index] != Sounding::Off {
                self.post(self.note_off_message(Note::new(index as u8)));
                self.state.notes[index] = Sounding::Off;
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
        Instrument::for_channel(&ChannelKind::Synth, RATE, &mut SampleCache::new()).unwrap()
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
        let quiet_from = gate as usize + (10.0 * SYNTH_RELEASE * RATE as f32) as usize;
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
        assert!(Instrument::for_channel(&kind, RATE, &mut SampleCache::new()).is_err());
    }
}
