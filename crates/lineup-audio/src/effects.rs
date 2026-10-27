//! A channel's effects as the audio thread plays them: the equalizer's
//! three filters, the compressor and the delay (see
//! [`lineup_core::effects`] for what each one does to the sound).
//!
//! Each works on one sample at a time and never allocates. The delay
//! needs a stretch of memory to keep the sound it plays again: that is a
//! [`DelayLine`], made on the UI thread and sent with the settings that
//! turn the delay on.

use lineup_core::{Compressor, Delay, Effects, Equalizer, Tempo};

use crate::dsp::Smoothed;

/// How long a change to the delay's mix takes to settle, in seconds: as
/// quick as a fader, without a click.
const MIX_SMOOTHING: f32 = 0.01;

/// The memory a delay keeps: [`Delay::MOST_SECONDS`] of sound.
pub struct DelayLine(Box<[f32]>);

impl DelayLine {
    /// A silent line for a delay at `sample_rate`.
    pub fn new(sample_rate: u32) -> Self {
        let frames = (Delay::MOST_SECONDS * sample_rate as f32).ceil() as usize + 1;
        DelayLine(vec![0.0; frames].into_boxed_slice())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Debug for DelayLine {
    /// Its length, not the thousands of samples in it.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DelayLine({} frames)", self.0.len())
    }
}

/// One channel's effects.
pub(crate) struct ChannelEffects {
    sample_rate: u32,
    settings: Effects,
    low: Biquad,
    mid: Biquad,
    high: Biquad,
    compressor: Follower,
    line: Option<DelayLine>,
    /// Where the next sample goes in the line.
    written: usize,
    /// How many frames back the echo is read.
    delay_frames: usize,
    mix: Smoothed,
}

impl ChannelEffects {
    /// No effects: the sound goes through as it is.
    pub(crate) fn new(sample_rate: u32) -> Self {
        ChannelEffects {
            sample_rate,
            settings: Effects::default(),
            low: Biquad::default(),
            mid: Biquad::default(),
            high: Biquad::default(),
            compressor: Follower::new(sample_rate),
            line: None,
            written: 0,
            delay_frames: 1,
            mix: Smoothed::new(0.0, MIX_SMOOTHING, sample_rate),
        }
    }

    /// Takes new settings, at `tempo`. If the delay is on and has no line
    /// yet, it takes `line`; a line it does not take is given back, for
    /// the UI thread to free.
    pub(crate) fn set(
        &mut self,
        settings: Effects,
        line: Option<DelayLine>,
        tempo: Tempo,
    ) -> Option<DelayLine> {
        let rate = self.sample_rate as f32;
        let eq = settings.eq;
        if eq != self.settings.eq {
            self.low
                .set(Shape::LowShelf, Equalizer::LOW_HZ, eq.low, rate);
            self.mid.set(Shape::Peak, Equalizer::MID_HZ, eq.mid, rate);
            self.high
                .set(Shape::HighShelf, Equalizer::HIGH_HZ, eq.high, rate);
        }
        let mut spare = line;
        if settings.delay.is_on() {
            if self.line.is_none() {
                self.line = spare.take();
            }
            if !self.settings.delay.is_on()
                && let Some(line) = &mut self.line
            {
                // Echoes from the last time it was on would come back.
                line.0.fill(0.0);
            }
        }
        self.mix.set(match &self.line {
            Some(_) => settings.delay.mix,
            None => 0.0,
        });
        self.settings = settings;
        self.set_tempo(tempo);
        spare
    }

    /// Times the delay's echoes to `tempo`.
    pub(crate) fn set_tempo(&mut self, tempo: Tempo) {
        let frames = (self.settings.delay.seconds(tempo) * self.sample_rate as f32).round();
        let longest = self.line.as_ref().map_or(1, |line| line.len() - 1);
        self.delay_frames = (frames as usize).clamp(1, longest.max(1));
    }

    /// Whether anything it does changes the sound.
    fn is_on(&self) -> bool {
        !self.settings.eq.is_flat()
            || self.settings.compressor.is_on()
            || self.line.is_some() && (self.settings.delay.is_on() || self.mix.value() > 0.0)
    }

    /// The next sample, from `input`: through the EQ, the compressor and
    /// the delay, in that order.
    pub(crate) fn process(&mut self, input: f32) -> f32 {
        if !self.is_on() {
            return input;
        }
        let mut sample = input;
        let eq = self.settings.eq;
        if !eq.is_flat() {
            sample = self
                .high
                .process(self.mid.process(self.low.process(sample)));
        }
        let compressor = self.settings.compressor;
        if compressor.is_on() {
            sample *= self.compressor.gain(sample, &compressor);
        }
        let mix = self.mix.next_value();
        if let Some(line) = &mut self.line {
            let frames = line.len();
            let read = (self.written + frames - self.delay_frames) % frames;
            let echo = line.0[read];
            line.0[self.written] = sample + echo * self.settings.delay.feedback;
            self.written = (self.written + 1) % frames;
            sample += echo * mix;
        }
        sample
    }
}

/// How loud a channel is getting, and the gain the compressor sets for
/// it: a level that rises quickly with the peaks of the wave (its
/// attack) and falls slowly between them (its release), so the gain does
/// not flutter with every wave.
struct Follower {
    attack: f32,
    release: f32,
    /// The level, from 0.0 to 1.0 (full scale) and over.
    level: f32,
}

/// The quietest level the compressor reads, in decibels: under its
/// lowest threshold, so it does nothing there.
const FLOOR: f32 = -100.0;

impl Follower {
    fn new(sample_rate: u32) -> Self {
        let coefficient = |seconds: f32| 1.0 - (-1.0 / (seconds * sample_rate as f32)).exp();
        Follower {
            attack: coefficient(Compressor::ATTACK_SECONDS),
            release: coefficient(Compressor::RELEASE_SECONDS),
            level: 0.0,
        }
    }

    /// The gain to play `sample` at: turned down as `compressor` says for
    /// the level so far, and up by its make-up gain.
    fn gain(&mut self, sample: f32, compressor: &Compressor) -> f32 {
        let peak = sample.abs();
        let speed = if peak > self.level {
            self.attack
        } else {
            self.release
        };
        self.level += (peak - self.level) * speed;
        let level = (20.0 * self.level.log10()).max(FLOOR);
        let decibels = compressor.gain - compressor.reduction(level);
        10f32.powf(decibels / 20.0)
    }
}

/// The shapes of the equalizer's bands.
#[derive(Clone, Copy)]
enum Shape {
    /// Everything below the frequency up or down.
    LowShelf,
    /// A bell around the frequency.
    Peak,
    /// Everything above the frequency up or down.
    HighShelf,
}

/// A second-order filter, a "biquad", with the coefficients of Robert
/// Bristow-Johnson's "Audio EQ Cookbook": the filter most equalizers are
/// made of. Each sample out is a mix of the last three in and the last
/// two out.
#[derive(Clone, Copy)]
struct Biquad {
    b: [f32; 3],
    a: [f32; 2],
    inputs: [f32; 2],
    outputs: [f32; 2],
}

impl Default for Biquad {
    /// A filter that changes nothing.
    fn default() -> Self {
        Biquad {
            b: [1.0, 0.0, 0.0],
            a: [0.0, 0.0],
            inputs: [0.0; 2],
            outputs: [0.0; 2],
        }
    }
}

impl Biquad {
    /// Shapes it as `shape` at `hz`, turned up or down by `decibels`, at
    /// `sample_rate`. It keeps the samples it has, so it changes without
    /// a click.
    fn set(&mut self, shape: Shape, hz: f32, decibels: f32, sample_rate: f32) {
        let gain = 10f32.powf(decibels / 40.0);
        let omega = std::f32::consts::TAU * hz / sample_rate;
        let (sin, cos) = omega.sin_cos();
        // A slope of 1 for the shelves, and a Q of 1 for the bell: about
        // an octave and a half wide.
        let alpha = match shape {
            Shape::Peak => sin / 2.0,
            Shape::LowShelf | Shape::HighShelf => sin / 2.0 * std::f32::consts::SQRT_2,
        };
        let root = 2.0 * gain.sqrt() * alpha;
        let (b, a) = match shape {
            Shape::Peak => (
                [1.0 + alpha * gain, -2.0 * cos, 1.0 - alpha * gain],
                [1.0 + alpha / gain, -2.0 * cos, 1.0 - alpha / gain],
            ),
            Shape::LowShelf => (
                [
                    gain * ((gain + 1.0) - (gain - 1.0) * cos + root),
                    2.0 * gain * ((gain - 1.0) - (gain + 1.0) * cos),
                    gain * ((gain + 1.0) - (gain - 1.0) * cos - root),
                ],
                [
                    (gain + 1.0) + (gain - 1.0) * cos + root,
                    -2.0 * ((gain - 1.0) + (gain + 1.0) * cos),
                    (gain + 1.0) + (gain - 1.0) * cos - root,
                ],
            ),
            Shape::HighShelf => (
                [
                    gain * ((gain + 1.0) + (gain - 1.0) * cos + root),
                    -2.0 * gain * ((gain - 1.0) + (gain + 1.0) * cos),
                    gain * ((gain + 1.0) + (gain - 1.0) * cos - root),
                ],
                [
                    (gain + 1.0) - (gain - 1.0) * cos + root,
                    2.0 * ((gain - 1.0) - (gain + 1.0) * cos),
                    (gain + 1.0) - (gain - 1.0) * cos - root,
                ],
            ),
        };
        self.b = b.map(|b| b / a[0]);
        self.a = [a[1] / a[0], a[2] / a[0]];
    }

    fn process(&mut self, input: f32) -> f32 {
        let [b0, b1, b2] = self.b;
        let [a1, a2] = self.a;
        let output = b0 * input + b1 * self.inputs[0] + b2 * self.inputs[1]
            - a1 * self.outputs[0]
            - a2 * self.outputs[1];
        // Far too quiet to hear: dropped, so the numbers never get so
        // small (denormals) that the processor slows down on them.
        let output = if output.abs() < 1e-20 { 0.0 } else { output };
        self.inputs = [input, self.inputs[0]];
        self.outputs = [output, self.outputs[0]];
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 48_000;

    /// How loud a sine wave at `hz` comes out of `effects`, next to how
    /// loud it went in, in decibels.
    fn response(effects: &mut ChannelEffects, hz: f32) -> f32 {
        let wave = |frame: usize| (std::f32::consts::TAU * hz * frame as f32 / RATE as f32).sin();
        // A second to settle, then a second to measure.
        let (mut peak_in, mut peak_out) = (0.0f32, 0.0f32);
        for frame in 0..2 * RATE as usize {
            let input = wave(frame);
            let output = effects.process(input);
            if frame >= RATE as usize {
                peak_in = peak_in.max(input.abs());
                peak_out = peak_out.max(output.abs());
            }
        }
        20.0 * (peak_out / peak_in).log10()
    }

    fn with(settings: Effects) -> ChannelEffects {
        let mut effects = ChannelEffects::new(RATE);
        let line = settings.delay.is_on().then(|| DelayLine::new(RATE));
        effects.set(settings, line, Tempo::new(120.0));
        effects
    }

    #[test]
    fn plain_effects_leave_the_sound_as_it_is() {
        let mut effects = ChannelEffects::new(RATE);
        for sample in [0.0, 0.5, -0.25, 1.0] {
            assert_eq!(effects.process(sample), sample);
        }
    }

    #[test]
    fn the_equalizer_turns_each_band_up_or_down() {
        let mut settings = Effects::default();
        settings.eq.low = 6.0;
        let mut effects = with(settings);
        assert!((response(&mut effects, 50.0) - 6.0).abs() < 0.5);
        assert!(response(&mut effects, 5_000.0).abs() < 0.5);

        let mut settings = Effects::default();
        settings.eq.mid = -6.0;
        let mut effects = with(settings);
        assert!((response(&mut effects, 1_000.0) + 6.0).abs() < 0.5);
        assert!(response(&mut effects, 60.0).abs() < 0.5);

        let mut settings = Effects::default();
        settings.eq.high = 9.0;
        let mut effects = with(settings);
        assert!((response(&mut effects, 15_000.0) - 9.0).abs() < 0.8);
        assert!(response(&mut effects, 100.0).abs() < 0.5);
    }

    #[test]
    fn the_compressor_turns_loud_sounds_down() {
        let mut effects = with(Effects {
            compressor: Compressor {
                threshold: -20.0,
                ratio: 4.0,
                gain: 0.0,
            },
            ..Effects::default()
        });
        // A wave at full scale is 20 dB over: it comes out 15 dB lower.
        let loud = response(&mut effects, 220.0);
        assert!((loud + 15.0).abs() < 1.5, "{loud}");
        // A quiet one, under the threshold, goes through.
        let mut quiet = 0.0f32;
        for frame in 0..RATE as usize {
            let input = 0.05 * (std::f32::consts::TAU * 220.0 * frame as f32 / RATE as f32).sin();
            quiet = quiet.max(effects.process(input).abs());
        }
        assert!((quiet - 0.05).abs() < 0.005, "{quiet}");
    }

    #[test]
    fn the_delay_echoes_in_time_with_the_song() {
        let mut effects = with(Effects {
            delay: Delay {
                steps: 2,
                feedback: 0.5,
                mix: 1.0,
            },
            ..Effects::default()
        });
        // Let the mix settle, in silence.
        for _ in 0..RATE {
            effects.process(0.0);
        }
        // An eighth note at 120 BPM is a quarter of a second.
        let eighth = RATE as usize / 4;
        let out: Vec<f32> = (0..3 * eighth + 1)
            .map(|frame| effects.process(if frame == 0 { 1.0 } else { 0.0 }))
            .collect();
        assert_eq!(out[0], 1.0);
        assert!((out[eighth] - 1.0).abs() < 1e-3, "{}", out[eighth]);
        assert!((out[2 * eighth] - 0.5).abs() < 1e-3);
        assert!((out[3 * eighth] - 0.25).abs() < 1e-3);
        assert!(out[eighth / 2].abs() < 1e-6);
    }

    #[test]
    fn a_delay_line_comes_once_and_goes_back_when_not_needed() {
        let mut effects = ChannelEffects::new(RATE);
        let mut settings = Effects::default();
        settings.delay.mix = 0.5;
        let tempo = Tempo::new(100.0);
        assert!(
            effects
                .set(settings, Some(DelayLine::new(RATE)), tempo)
                .is_none()
        );
        // It has one: another is given back.
        assert!(
            effects
                .set(settings, Some(DelayLine::new(RATE)), tempo)
                .is_some()
        );
        // Turned off and on again, it keeps the one it has.
        settings.delay.mix = 0.0;
        assert!(effects.set(settings, None, tempo).is_none());
        settings.delay.mix = 0.5;
        assert!(effects.set(settings, None, tempo).is_none());
        assert!(effects.line.is_some());
    }
}
