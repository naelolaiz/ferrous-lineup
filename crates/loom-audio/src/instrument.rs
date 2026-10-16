//! The instruments a channel can play: a sampler and the Loom Osc synth.
//!
//! Both are *polyphonic*: they have a fixed number of voices, and each
//! note that starts takes one. That is what lets the synth play chords
//! and lets a drum hit ring on while the next one starts.

use std::sync::Arc;

use loom_core::{ChannelKind, Note, STEP_NOTE, SampleSource};

use crate::drums;
use crate::dsp::LowPass;
use crate::sample::{SampleCache, SampleData, SampleError};

/// How many notes the Loom Osc can play at once.
pub const SYNTH_VOICES: usize = 16;
/// How many hits a sampler can play at once.
pub const SAMPLER_VOICES: usize = 8;

/// The sound-making part of a channel.
///
/// An enum instead of a trait object (`Box<dyn Instrument>`): there are
/// only two kinds, the audio thread can `match` on them without any
/// pointer indirection, and creating one never needs a `Box`.
#[derive(Debug, Clone)]
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
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
        })
    }

    /// Starts playing a note.
    ///
    /// `gate` is how many frames the note is held for. The synth starts
    /// fading out when it ends; the sampler ignores it and always plays
    /// its sound to the end, like a drum machine.
    pub fn note_on(&mut self, pitch: Note, velocity: u8, gate: u32, output_rate: u32) {
        match self {
            Instrument::Sampler(sampler) => sampler.note_on(pitch, velocity, output_rate),
            Instrument::Synth(synth) => synth.note_on(pitch, velocity, gate),
        }
    }

    /// Lets go of every held note, as when playback stops.
    pub fn release_all(&mut self) {
        match self {
            Instrument::Sampler(_) => {} // samples always play to the end
            Instrument::Synth(synth) => synth.release_all(),
        }
    }

    /// The next output sample: the sum of every voice that is playing.
    pub fn next_sample(&mut self) -> f32 {
        match self {
            Instrument::Sampler(sampler) => sampler.next_sample(),
            Instrument::Synth(synth) => synth.next_sample(),
        }
    }

    /// How many voices are making sound right now.
    pub fn voices_playing(&self) -> usize {
        match self {
            Instrument::Sampler(sampler) => sampler.voices.iter().filter(|v| !v.is_idle()).count(),
            Instrument::Synth(synth) => synth.voices.iter().filter(|v| !v.is_idle()).count(),
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
        };
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
    frequency: f32,
    phase: f32,
    filter: LowPass,
    gain: f32,
    /// Frames left until the note is released.
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

        // Count down the gate while the note is held, then let go.
        if matches!(self.stage, Stage::Attack | Stage::Decay) {
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
        voice.frequency = pitch.frequency();
        voice.gain = velocity_gain(velocity) * SYNTH_VOICE_GAIN;
        voice.gate = gate;
        voice.stage = Stage::Attack;
        voice.started = self.notes_started;
        // The level, phase and filter carry on from where they were, so a
        // stolen voice does not jump (and click).
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
    fn missing_file_reports_an_error() {
        let kind = ChannelKind::Sampler(SampleSource::File("/no/such.wav".into()));
        assert!(Instrument::for_channel(&kind, RATE, &mut SampleCache::new()).is_err());
    }
}
