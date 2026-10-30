//! The built-in drum kit, synthesized from scratch.
//!
//! Each function renders one hit into a buffer of samples. This happens
//! once, on the UI thread, when a drum channel is created; after that the
//! drum is played back like any other sample. So these functions are free
//! to allocate, unlike the code on the audio thread.

use std::f32::consts::TAU;

use lineup_core::Drum;

use crate::dsp::{HighPass, LowPass, Noise};
use crate::sample::SampleData;

/// Renders a built-in drum at the given sample rate.
pub fn render(drum: Drum, sample_rate: u32) -> SampleData {
    let frames = match drum {
        Drum::Kick => kick(sample_rate),
        Drum::Snare => snare(sample_rate),
        Drum::Clap => clap(sample_rate),
        Drum::HiHat => hi_hat(sample_rate),
        Drum::OpenHat => open_hat(sample_rate),
        Drum::Rim => rim(sample_rate),
        Drum::LowTom => tom(sample_rate, 98.0, 0.24),
        Drum::HighTom => tom(sample_rate, 165.0, 0.18),
        Drum::Ride => ride(sample_rate),
        Drum::Crash => crash(sample_rate),
        Drum::Cowbell => cowbell(sample_rate),
        Drum::Shaker => shaker(sample_rate),
        Drum::Conga => conga(sample_rate),
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

/// The six frequencies of the metal sound in a famous 1980s drum machine:
/// square waves at pitches that are not multiples of each other, so
/// together they ring like metal instead of forming a chord.
const METAL: [f32; 6] = [205.3, 304.4, 369.6, 522.7, 540.0, 800.0];

/// A square wave: +1 for the first half of each cycle, -1 for the second.
fn square(t: f32, frequency: f32) -> f32 {
    if (t * frequency).fract() < 0.5 {
        1.0
    } else {
        -1.0
    }
}

/// The six square waves of [`METAL`], each `ratio` times higher, added up.
fn metal(t: f32, ratio: f32) -> f32 {
    METAL.iter().map(|&f| square(t, f * ratio)).sum::<f32>() / METAL.len() as f32
}

/// A hi-hat left open: metal and noise, ringing for a while.
fn open_hat(sample_rate: u32) -> Vec<f32> {
    let mut noise = Noise::new(4321);
    let mut filter = HighPass::new(6_500.0, sample_rate);
    generate(0.7, sample_rate, |t| {
        let sound = metal(t, 1.0) * 0.6 + noise.next_sample() * 0.5;
        filter.process(sound) * decay(t, 0.14)
    })
}

/// A stick hitting the rim of the snare: a short, hollow click.
fn rim(sample_rate: u32) -> Vec<f32> {
    let mut noise = Noise::new(77);
    let mut filter = HighPass::new(2_000.0, sample_rate);
    generate(0.15, sample_rate, |t| {
        let click = (t * 1_750.0 * TAU).sin() * decay(t, 0.012);
        let body = (t * 480.0 * TAU).sin() * decay(t, 0.02) * 0.6;
        let snap = filter.process(noise.next_sample()) * decay(t, 0.004) * 0.5;
        click + body + snap
    })
}

/// A tom: like the kick, a sine that falls in pitch, but higher and
/// rounder. `pitch` is where it settles, in hertz; `ring` how long it
/// rings, in seconds.
fn tom(sample_rate: u32, pitch: f32, ring: f32) -> Vec<f32> {
    let mut noise = Noise::new(55);
    let mut filter = HighPass::new(1_000.0, sample_rate);
    let mut phase = 0.0;
    generate(ring * 4.0, sample_rate, |t| {
        let frequency = pitch * (1.0 + 0.6 * decay(t, 0.03));
        phase += frequency / sample_rate as f32;
        let skin = (phase * TAU).sin() * decay(t, ring);
        let stick = filter.process(noise.next_sample()) * decay(t, 0.006) * 0.4;
        skin + stick
    })
}

/// A ride cymbal: a clear "ping" over metal that rings for a long time.
fn ride(sample_rate: u32) -> Vec<f32> {
    let mut filter = HighPass::new(3_000.0, sample_rate);
    let mut noise = Noise::new(808);
    generate(2.2, sample_rate, |t| {
        let wash = filter.process(metal(t, 2.3) + noise.next_sample() * 0.2) * decay(t, 0.45);
        let ping = (t * 3_150.0 * TAU).sin() * decay(t, 0.25) * 0.35;
        wash + ping
    })
}

/// A crash cymbal: a loud burst of noise and metal that fades slowly.
fn crash(sample_rate: u32) -> Vec<f32> {
    let mut filter = HighPass::new(2_500.0, sample_rate);
    let mut noise = Noise::new(909);
    generate(2.6, sample_rate, |t| {
        let sound = noise.next_sample() * 0.7 + metal(t, 1.7) * 0.5;
        let envelope = decay(t, 0.5) * 0.8 + decay(t, 0.05) * 0.2;
        filter.process(sound) * envelope
    })
}

/// A cowbell: two square waves a little less than a fifth apart, with a
/// sharp attack and a short ring.
fn cowbell(sample_rate: u32) -> Vec<f32> {
    let mut low_cut = HighPass::new(400.0, sample_rate);
    let mut high_cut = LowPass::new(3_000.0, sample_rate);
    generate(0.45, sample_rate, |t| {
        let tone = square(t, 540.0) + square(t, 800.0);
        let envelope = decay(t, 0.015) * 0.5 + decay(t, 0.09) * 0.5;
        high_cut.process(low_cut.process(tone)) * envelope
    })
}

/// A shaker: very bright noise that swells for a moment, then stops.
fn shaker(sample_rate: u32) -> Vec<f32> {
    let mut noise = Noise::new(31);
    let mut filter = HighPass::new(5_000.0, sample_rate);
    generate(0.2, sample_rate, |t| {
        let swell = (t / 0.015).min(1.0);
        let fade = decay((t - 0.015).max(0.0), 0.035);
        filter.process(noise.next_sample()) * swell * fade
    })
}

/// A conga struck open: a short, singing tone that drops a little in
/// pitch, and the slap of the hand.
fn conga(sample_rate: u32) -> Vec<f32> {
    let mut noise = Noise::new(63);
    let mut filter = HighPass::new(1_500.0, sample_rate);
    let mut phase = 0.0;
    generate(0.5, sample_rate, |t| {
        let frequency = 320.0 * (1.0 + 0.25 * decay(t, 0.015));
        phase += frequency / sample_rate as f32;
        let skin = (phase * TAU).sin() * decay(t, 0.11);
        let slap = filter.process(noise.next_sample()) * decay(t, 0.004) * 0.3;
        skin + slap
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
