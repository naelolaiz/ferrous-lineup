//! Sound generators.

use std::f32::consts::TAU;

/// How long volume changes take to settle, in seconds. Changing the volume
/// instantly would make an audible click, so we glide to the new value.
const GAIN_SMOOTHING_SECONDS: f32 = 0.005;

/// A sine-wave test tone that can be switched on and off without clicks.
///
/// This is the first instrument of Ferrous Loom. It proves that audio
/// reaches the speakers, and it is the starting point for the synths of
/// later steps.
#[derive(Debug, Clone)]
pub struct TestTone {
    sample_rate: f32,
    /// Position in the waveform, from 0.0 to 1.0 (one full cycle).
    phase: f32,
    frequency: f32,
    volume: f32,
    on: bool,
    /// The gain actually applied right now. It moves toward the target
    /// gain (`volume` when on, `0.0` when off) a little on every sample.
    current_gain: f32,
    /// How far `current_gain` moves toward the target per sample (0..1).
    smoothing: f32,
}

impl TestTone {
    /// Creates a silent tone at 440 Hz for the given sample rate.
    pub fn new(sample_rate: u32) -> Self {
        let sample_rate = sample_rate as f32;
        TestTone {
            sample_rate,
            phase: 0.0,
            frequency: 440.0,
            volume: 0.5,
            on: false,
            current_gain: 0.0,
            smoothing: 1.0 - (-1.0 / (GAIN_SMOOTHING_SECONDS * sample_rate)).exp(),
        }
    }

    /// Switches the tone on or off.
    pub fn set_on(&mut self, on: bool) {
        self.on = on;
    }

    /// Sets the pitch in hertz.
    pub fn set_frequency(&mut self, frequency: f32) {
        // Above half the sample rate (the Nyquist frequency) a digital
        // sine cannot be represented, so we keep it below that.
        self.frequency = frequency.clamp(1.0, self.sample_rate * 0.45);
    }

    /// Sets the volume, from 0.0 (silent) to 1.0 (full scale).
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Computes the next sample, between -1.0 and 1.0.
    pub fn next_sample(&mut self) -> f32 {
        let target = if self.on { self.volume } else { 0.0 };
        self.current_gain += (target - self.current_gain) * self.smoothing;

        let sample = (self.phase * TAU).sin() * self.current_gain;

        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }

    /// Fills an interleaved buffer: every frame gets the same sample on
    /// all `channels` (so a stereo buffer looks like L R L R ...).
    pub fn render(&mut self, buffer: &mut [f32], channels: usize) {
        for frame in buffer.chunks_mut(channels) {
            let sample = self.next_sample();
            frame.fill(sample);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 48_000;

    #[test]
    fn silent_when_off() {
        let mut tone = TestTone::new(RATE);
        let mut buffer = vec![1.0; 512];
        tone.render(&mut buffer, 2);
        assert!(buffer.iter().all(|&s| s == 0.0));
    }

    #[test]
    fn fades_in_without_a_click() {
        let mut tone = TestTone::new(RATE);
        tone.set_volume(1.0);
        tone.set_on(true);
        let first = tone.next_sample().abs();
        assert!(first < 0.01, "first sample jumped to {first}");
    }

    #[test]
    fn stays_within_volume() {
        let mut tone = TestTone::new(RATE);
        tone.set_volume(0.25);
        tone.set_on(true);
        let mut buffer = vec![0.0; RATE as usize];
        tone.render(&mut buffer, 1);
        let peak = buffer.iter().fold(0.0_f32, |max, s| max.max(s.abs()));
        assert!(peak > 0.24 && peak <= 0.25, "peak was {peak}");
    }

    #[test]
    fn plays_the_requested_frequency() {
        let mut tone = TestTone::new(RATE);
        tone.set_frequency(100.0);
        tone.set_on(true);
        let mut buffer = vec![0.0; RATE as usize];
        tone.render(&mut buffer, 1);

        // A sine at 100 Hz goes from negative to positive 100 times a second.
        let rising_crossings = buffer
            .windows(2)
            .filter(|w| w[0] < 0.0 && w[1] >= 0.0)
            .count();
        assert!(
            (99..=101).contains(&rising_crossings),
            "got {rising_crossings}"
        );
    }

    #[test]
    fn all_channels_get_the_same_sample() {
        let mut tone = TestTone::new(RATE);
        tone.set_on(true);
        let mut buffer = vec![0.0; 64];
        tone.render(&mut buffer, 2);
        for frame in buffer.chunks(2) {
            assert_eq!(frame[0], frame[1]);
        }
    }
}
