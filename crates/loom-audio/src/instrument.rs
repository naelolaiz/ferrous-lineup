//! The instruments a channel can play: a sampler and the Loom Osc synth.

use std::sync::Arc;

use loom_core::{ChannelKind, Note, SampleSource};

use crate::drums;
use crate::dsp::LowPass;
use crate::sample::{SampleData, SampleError};

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
    /// synthesized and files are loaded here, on the calling thread.
    pub fn for_channel(kind: &ChannelKind, sample_rate: u32) -> Result<Self, SampleError> {
        Ok(match kind {
            ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
                Instrument::Sampler(Sampler::new(Arc::new(drums::render(*drum, sample_rate))))
            }
            ChannelKind::Sampler(SampleSource::File(path)) => {
                Instrument::Sampler(Sampler::new(Arc::new(SampleData::load_wav(path)?)))
            }
            ChannelKind::Synth { note } => Instrument::Synth(Synth::new(*note, sample_rate)),
        })
    }

    /// Starts the sound from the beginning.
    pub fn trigger(&mut self, output_rate: u32) {
        match self {
            Instrument::Sampler(sampler) => sampler.trigger(output_rate),
            Instrument::Synth(synth) => synth.trigger(),
        }
    }

    /// The next output sample (silence once the sound has finished).
    pub fn next_sample(&mut self) -> f32 {
        match self {
            Instrument::Sampler(sampler) => sampler.next_sample(),
            Instrument::Synth(synth) => synth.next_sample(),
        }
    }
}

/// Plays a sample from start to end each time it is triggered.
#[derive(Debug, Clone)]
pub struct Sampler {
    sample: Arc<SampleData>,
    /// Where we are in the sample, in (fractional) frames, or `None` when silent.
    position: Option<f64>,
    /// How many sample frames to advance per output frame. It is 1.0 when
    /// the sample and the sound card use the same rate; a 44.1 kHz sample
    /// on a 48 kHz card advances by 0.91875 per frame.
    increment: f64,
}

impl Sampler {
    pub fn new(sample: Arc<SampleData>) -> Self {
        Sampler {
            sample,
            position: None,
            increment: 1.0,
        }
    }

    pub fn trigger(&mut self, output_rate: u32) {
        self.increment = self.sample.sample_rate() as f64 / output_rate as f64;
        self.position = Some(0.0);
    }

    pub fn next_sample(&mut self) -> f32 {
        let Some(position) = self.position else {
            return 0.0;
        };
        let frames = self.sample.frames();
        let index = position as usize;
        if index + 1 >= frames.len() {
            self.position = None;
            return frames.get(index).copied().unwrap_or(0.0);
        }
        // Linear interpolation between the two nearest frames.
        let fraction = (position - index as f64) as f32;
        let sample = frames[index] + (frames[index + 1] - frames[index]) * fraction;
        self.position = Some(position + self.increment);
        sample
    }
}

/// How long the Loom Osc takes to reach full volume, in seconds.
const SYNTH_ATTACK: f32 = 0.003;
/// How quickly the Loom Osc fades after the attack, in seconds (time constant).
const SYNTH_DECAY: f32 = 0.25;

/// The Loom Osc: a sawtooth oscillator with a soft filter and a
/// pluck-shaped volume envelope (fast attack, exponential decay).
#[derive(Debug, Clone)]
pub struct Synth {
    sample_rate: f32,
    frequency: f32,
    phase: f32,
    /// Seconds since the last trigger, or `None` when silent.
    time: Option<f32>,
    filter: LowPass,
}

impl Synth {
    pub fn new(note: Note, sample_rate: u32) -> Self {
        Synth {
            sample_rate: sample_rate as f32,
            frequency: note.frequency(),
            phase: 0.0,
            time: None,
            filter: LowPass::new(2_500.0, sample_rate),
        }
    }

    pub fn trigger(&mut self) {
        self.time = Some(0.0);
        self.phase = 0.0;
        self.filter.reset();
    }

    pub fn next_sample(&mut self) -> f32 {
        let Some(time) = self.time else {
            return 0.0;
        };
        let envelope = if time < SYNTH_ATTACK {
            time / SYNTH_ATTACK
        } else {
            (-(time - SYNTH_ATTACK) / SYNTH_DECAY).exp()
        };
        // After about 7 time constants the sound is below -60 dB: stop.
        if time > SYNTH_ATTACK + 7.0 * SYNTH_DECAY {
            self.time = None;
            return 0.0;
        }

        // A sawtooth ramps from -1 to 1 once per cycle.
        let saw = self.phase * 2.0 - 1.0;
        self.phase = (self.phase + self.frequency / self.sample_rate).fract();
        self.time = Some(time + 1.0 / self.sample_rate);

        self.filter.process(saw) * envelope
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(instrument: &mut Instrument, frames: usize) -> Vec<f32> {
        (0..frames).map(|_| instrument.next_sample()).collect()
    }

    #[test]
    fn silent_until_triggered() {
        let mut synth =
            Instrument::for_channel(&ChannelKind::Synth { note: Note::A4 }, 48_000).unwrap();
        assert!(render(&mut synth, 1_000).iter().all(|&s| s == 0.0));
        synth.trigger(48_000);
        assert!(render(&mut synth, 1_000).iter().any(|&s| s != 0.0));
    }

    #[test]
    fn sampler_plays_once_then_stops() {
        let sample = Arc::new(SampleData::new(vec![0.5; 100], 48_000));
        let mut sampler = Instrument::Sampler(Sampler::new(sample));
        sampler.trigger(48_000);
        let out = render(&mut sampler, 200);
        assert!(out[..99].iter().all(|&s| s == 0.5));
        assert!(out[101..].iter().all(|&s| s == 0.0));
    }

    #[test]
    fn sampler_resamples_to_the_output_rate() {
        // One second recorded at 24 kHz must last one second at 48 kHz.
        let sample = Arc::new(SampleData::new(vec![0.5; 24_000], 24_000));
        let mut sampler = Instrument::Sampler(Sampler::new(sample));
        sampler.trigger(48_000);
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
    fn synth_fades_out_and_stops() {
        let mut synth = Instrument::Synth(Synth::new(Note::C4, 48_000));
        synth.trigger(48_000);
        let out = render(&mut synth, 48_000 * 3);
        assert!(out[48_000 * 2..].iter().all(|&s| s == 0.0));
    }

    #[test]
    fn missing_file_reports_an_error() {
        let kind = ChannelKind::Sampler(SampleSource::File("/no/such.wav".into()));
        assert!(Instrument::for_channel(&kind, 48_000).is_err());
    }
}
