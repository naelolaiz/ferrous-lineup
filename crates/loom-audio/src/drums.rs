//! The built-in drum kit, synthesized from scratch.
//!
//! Each function renders one hit into a buffer of samples. This happens
//! once, on the UI thread, when a drum channel is created; after that the
//! drum is played back like any other sample. So these functions are free
//! to allocate, unlike the code on the audio thread.

use std::f32::consts::TAU;

use loom_core::Drum;

use crate::dsp::{HighPass, Noise};
use crate::sample::SampleData;

/// Renders a built-in drum at the given sample rate.
pub fn render(drum: Drum, sample_rate: u32) -> SampleData {
    let frames = match drum {
        Drum::Kick => kick(sample_rate),
        Drum::Snare => snare(sample_rate),
        Drum::Clap => clap(sample_rate),
        Drum::HiHat => hi_hat(sample_rate),
    };
    SampleData::new(normalize(frames, 0.9), sample_rate)
}

/// Calls `f(t)` for every sample time `t` (in seconds) of a sound that
/// lasts `seconds`, and collects the results.
fn generate(seconds: f32, sample_rate: u32, f: impl FnMut(f32) -> f32) -> Vec<f32> {
    let frames = (seconds * sample_rate as f32) as usize;
    (0..frames)
        .map(|i| i as f32 / sample_rate as f32)
        .map(f)
        .collect()
}

/// An exponential decay: 1.0 at `t = 0`, falling to about 37% after `tau` seconds.
fn decay(t: f32, tau: f32) -> f32 {
    (-t / tau).exp()
}

/// A sine whose pitch falls quickly from 150 Hz to 45 Hz: the "boom".
fn kick(sample_rate: u32) -> Vec<f32> {
    let mut phase = 0.0;
    generate(0.5, sample_rate, |t| {
        let frequency = 45.0 + 105.0 * decay(t, 0.04);
        phase += frequency / sample_rate as f32;
        (phase * TAU).sin() * decay(t, 0.15)
    })
}

/// A short tone for the drum body plus filtered noise for the wires.
fn snare(sample_rate: u32) -> Vec<f32> {
    let mut noise = Noise::new(7);
    let mut filter = HighPass::new(1_500.0, sample_rate);
    generate(0.3, sample_rate, |t| {
        let body = (t * 185.0 * TAU).sin() * decay(t, 0.05) * 0.5;
        let wires = filter.process(noise.next_sample()) * decay(t, 0.08);
        body + wires
    })
}

/// Three quick bursts of noise and a tail, like several hands clapping.
fn clap(sample_rate: u32) -> Vec<f32> {
    let mut noise = Noise::new(99);
    let mut filter = HighPass::new(900.0, sample_rate);
    generate(0.35, sample_rate, |t| {
        let bursts = [0.0, 0.011, 0.022]
            .iter()
            .filter(|&&start| t >= start)
            .map(|&start| decay(t - start, 0.006))
            .sum::<f32>();
        let tail = if t >= 0.022 {
            decay(t - 0.022, 0.09) * 0.6
        } else {
            0.0
        };
        filter.process(noise.next_sample()) * (bursts + tail)
    })
}

/// Very short, bright noise.
fn hi_hat(sample_rate: u32) -> Vec<f32> {
    let mut noise = Noise::new(1234);
    let mut filter = HighPass::new(7_000.0, sample_rate);
    generate(0.12, sample_rate, |t| {
        filter.process(noise.next_sample()) * decay(t, 0.025)
    })
}

/// Scales `frames` so that the loudest sample reaches `peak`.
fn normalize(mut frames: Vec<f32>, peak: f32) -> Vec<f32> {
    let loudest = frames.iter().fold(0.0_f32, |max, s| max.max(s.abs()));
    if loudest > 0.0 {
        let gain = peak / loudest;
        for sample in &mut frames {
            *sample *= gain;
        }
    }
    frames
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_drum_is_audible_and_in_range() {
        for drum in Drum::ALL {
            let sample = render(drum, 48_000);
            let peak = sample.frames().iter().fold(0.0_f32, |m, s| m.max(s.abs()));
            assert!((peak - 0.9).abs() < 1e-4, "{drum:?} peak was {peak}");
            assert!(sample.frames().len() > 1_000, "{drum:?} is too short");
        }
    }

    #[test]
    fn every_drum_fades_out() {
        for drum in Drum::ALL {
            let sample = render(drum, 48_000);
            let frames = sample.frames();
            let tail = &frames[frames.len() - 100..];
            let tail_peak = tail.iter().fold(0.0_f32, |m, s| m.max(s.abs()));
            assert!(tail_peak < 0.05, "{drum:?} ends abruptly at {tail_peak}");
        }
    }
}
