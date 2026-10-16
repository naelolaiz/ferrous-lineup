//! Chroma: how strongly each of the twelve pitch classes sounds, moment
//! by moment.
//!
//! A *spectrogram* cuts the recording into short overlapping frames and
//! measures, with a Fourier transform (FFT), how loud each frequency is
//! in each frame. Every frequency belongs to a note (440 Hz is A4, and
//! each semitone up multiplies the frequency by 2^(1/12)), and every note
//! to a pitch class. Adding up the energy of all the frequencies of each
//! pitch class gives the frame's *chroma*: twelve numbers, from C to B.
//! A C major chord shows up as large numbers at C, E and G, in whatever
//! octave and on whatever instrument it is played.
//!
//! Two chromas are kept per frame: the *treble* one (notes from C3 up to
//! C7), where the chord is, and the *bass* one (E1 to C4), which says
//! which note is lowest and so, most of the time, the chord's root.

use std::sync::Arc;

use realfft::{RealFftPlanner, RealToComplex};

use crate::decode::Recording;

/// Twelve values, from C up to B.
pub type Chroma = [f32; 12];

/// How many samples one frame holds, at 44.1 or 48 kHz. Low notes need
/// long frames: the FFT tells frequencies apart only when they are more
/// than `sample_rate / FRAME` apart, here about 5 Hz, which is less than
/// a semitone down to about E2.
const FRAME: usize = 8192;

/// How far one frame starts after the previous one: about 46 ms.
const HOP: usize = 2048;

/// The frequency ranges of the two chromas, in Hz.
const TREBLE: (f32, f32) = (130.0, 2100.0);
const BASS: (f32, f32) = (40.0, 260.0);

/// The chromas of one frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    /// When the middle of the frame sounds, in seconds.
    pub time: f64,
    pub treble: Chroma,
    pub bass: Chroma,
}

/// Every frame of the recording.
pub fn frames(recording: &Recording) -> Vec<Frame> {
    let rate = recording.sample_rate as f32;
    let samples = &recording.samples;
    let mut planner = RealFftPlanner::<f32>::new();
    let fft: Arc<dyn RealToComplex<f32>> = planner.plan_fft_forward(FRAME);
    let mut input = fft.make_input_vec();
    let mut spectrum = fft.make_output_vec();
    let mut scratch = fft.make_scratch_vec();

    // A Hann window fades each frame in and out, so the FFT does not hear
    // the cut at its edges as a click full of every frequency.
    let window: Vec<f32> = (0..FRAME)
        .map(|i| 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / FRAME as f32).cos())
        .collect();

    // Which pitch class each FFT bin adds to, worked out once. Bin `k`
    // is the frequency `k * rate / FRAME`.
    let pitch_class_of = |bin: usize| {
        let frequency = bin as f32 * rate / FRAME as f32;
        let midi = 69.0 + 12.0 * (frequency / 440.0).log2();
        (midi.round() as i32).rem_euclid(12) as usize
    };
    let bins = |(low, high): (f32, f32)| {
        let first = (low * FRAME as f32 / rate).ceil() as usize;
        let last = ((high * FRAME as f32 / rate).floor() as usize).min(FRAME / 2);
        (first.max(1)..=last)
            .map(|bin| (bin, pitch_class_of(bin)))
            .collect::<Vec<_>>()
    };
    let treble_bins = bins(TREBLE);
    let bass_bins = bins(BASS);
    // High notes are further apart in Hz, so they get more bins than low
    // ones: each pitch class is divided by its number of bins, or B would
    // always look louder than C.
    let counts = |bins: &[(usize, usize)]| {
        let mut counts = [0.0_f32; 12];
        for &(_, pitch_class) in bins {
            counts[pitch_class] += 1.0;
        }
        counts.map(|count| count.max(1.0))
    };
    let treble_counts = counts(&treble_bins);
    let bass_counts = counts(&bass_bins);

    let mut frames = Vec::new();
    let mut start = 0;
    while start + FRAME <= samples.len() {
        for ((slot, &sample), &gain) in input.iter_mut().zip(&samples[start..]).zip(&window) {
            *slot = sample * gain;
        }
        if fft
            .process_with_scratch(&mut input, &mut spectrum, &mut scratch)
            .is_err()
        {
            break;
        }
        // Loudness is heard on a roughly logarithmic scale, so a quiet
        // note an octave away should still count: log(1 + x) squeezes
        // loud and soft together.
        let level = |bin: usize| (1.0 + 100.0 * spectrum[bin].norm()).ln();
        let mut treble = [0.0; 12];
        for &(bin, pitch_class) in &treble_bins {
            treble[pitch_class] += level(bin);
        }
        let mut bass = [0.0; 12];
        for &(bin, pitch_class) in &bass_bins {
            bass[pitch_class] += level(bin);
        }
        for i in 0..12 {
            treble[i] /= treble_counts[i];
            bass[i] /= bass_counts[i];
        }
        frames.push(Frame {
            time: (start + FRAME / 2) as f64 / f64::from(recording.sample_rate),
            treble,
            bass,
        });
        start += HOP;
    }
    frames
}

/// The average chroma of the frames between `from` and `to` seconds.
pub fn average(frames: &[Frame], from: f64, to: f64) -> Frame {
    let mut sum = Frame {
        time: (from + to) / 2.0,
        treble: [0.0; 12],
        bass: [0.0; 12],
    };
    let mut count = 0;
    for frame in frames.iter().filter(|f| (from..to).contains(&f.time)) {
        for i in 0..12 {
            sum.treble[i] += frame.treble[i];
            sum.bass[i] += frame.bass[i];
        }
        count += 1;
    }
    if count > 0 {
        for i in 0..12 {
            sum.treble[i] /= count as f32;
            sum.bass[i] /= count as f32;
        }
    }
    sum
}
