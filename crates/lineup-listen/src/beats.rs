//! Finding the beats: when each one falls, and so the tempo.
//!
//! This is the beat tracker of D. P. W. Ellis, "Beat Tracking by Dynamic
//! Programming" (Journal of New Music Research, 2007), with the settings
//! the librosa library documents as its defaults, so results can be
//! checked against it. It has three stages:
//!
//! 1. **Onset strength.** How much *new* sound starts in each short
//!    frame. A spectrogram on the *mel scale* (bands spaced the way the
//!    ear hears, wide at the top and narrow at the bottom) is taken, in
//!    decibels, and for each frame the increases of every band since the
//!    previous frame are added up. Decreases do not count: a note dying
//!    away is not an onset. The result is one number per frame, the
//!    *onset envelope*, which peaks on every drum hit and note start.
//! 2. **Tempo.** The envelope is compared with itself shifted by every
//!    possible beat length (its *autocorrelation*). At the right shift,
//!    peaks land on peaks and the correlation is high. Shifts near 120
//!    BPM get a head start, because trackers otherwise pick half or
//!    double the tempo as readily as the tempo itself.
//! 3. **Beats.** Of all the ways to place beats, find the one where the
//!    beats land on strong onsets and are spaced close to the beat
//!    length. Trying every placement is impossible, but *dynamic
//!    programming* makes it cheap: the best score for a beat at frame
//!    `i` is its onset strength plus the best score of a beat one beat
//!    length earlier, give or take, minus a penalty for the "give or
//!    take". Each frame remembers which earlier beat it chose; the
//!    strongest final score is traced back to the start.

use std::f64::consts::PI;

use realfft::RealFftPlanner;

/// The spectrogram's frame and hop, in samples. At 44.1 kHz a frame is
/// 46 ms and a new one starts every 12 ms.
const FRAME: usize = 2048;
const HOP: usize = 512;
/// How many mel bands the spectrogram has.
const MEL_BANDS: usize = 128;
/// The tempos considered, and the one tempos are drawn towards.
const SLOWEST_BPM: f64 = 30.0;
const FASTEST_BPM: f64 = 320.0;
const LIKELY_BPM: f64 = 120.0;
/// How wide the pull towards [`LIKELY_BPM`] is, in octaves.
const LIKELY_SPREAD_OCTAVES: f64 = 1.0;
/// How much uneven beat spacing costs, against onset strength.
const TIGHTNESS: f64 = 100.0;

/// The beats of a recording.
#[derive(Debug, Clone, PartialEq)]
pub struct Beats {
    /// The tempo the autocorrelation picked, in beats per minute.
    pub bpm: f64,
    /// When each beat falls, in seconds from the start.
    pub times: Vec<f64>,
}

/// Finds the beats of `samples` (mono) recorded at `sample_rate`.
pub fn track(samples: &[f32], sample_rate: u32) -> Beats {
    let frame_rate = f64::from(sample_rate) / HOP as f64;
    let envelope = onset_envelope(samples, sample_rate);
    let bpm = tempo(&envelope, frame_rate);
    let times = beats(&envelope, frame_rate, bpm)
        .into_iter()
        .map(|frame| frame as f64 / frame_rate)
        .collect();
    Beats { bpm, times }
}

/// The frequency of a mel value and back. The mel scale is a formula fit
/// to how far apart listeners judge pitches to be: about linear below
/// 1 kHz and logarithmic above.
fn mel_of(hz: f64) -> f64 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

fn hz_of(mel: f64) -> f64 {
    700.0 * (10_f64.powf(mel / 2595.0) - 1.0)
}

/// Triangular filters, one per mel band: `filters[band]` is a list of
/// `(fft bin, weight)`. Each band runs from the centre of the band below
/// to the centre of the band above, peaking at its own centre, and is
/// scaled by its width so that wide, high bands do not count for more.
fn mel_filters(sample_rate: u32) -> Vec<Vec<(usize, f64)>> {
    let bins = FRAME / 2 + 1;
    let top_mel = mel_of(f64::from(sample_rate) / 2.0);
    let edges_hz: Vec<f64> = (0..MEL_BANDS + 2)
        .map(|i| hz_of(top_mel * i as f64 / (MEL_BANDS + 1) as f64))
        .collect();
    let bin_of = |hz: f64| hz * FRAME as f64 / f64::from(sample_rate);
    (0..MEL_BANDS)
        .map(|band| {
            let (low, centre, high) = (
                bin_of(edges_hz[band]),
                bin_of(edges_hz[band + 1]),
                bin_of(edges_hz[band + 2]),
            );
            let scale = 2.0 / (edges_hz[band + 2] - edges_hz[band]).max(1e-12);
            (0..bins)
                .filter_map(|bin| {
                    let k = bin as f64;
                    let weight = if k >= low && k <= centre && centre > low {
                        (k - low) / (centre - low)
                    } else if k > centre && k <= high && high > centre {
                        (high - k) / (high - centre)
                    } else {
                        return None;
                    };
                    (weight > 0.0).then_some((bin, weight * scale))
                })
                .collect()
        })
        .collect()
}

/// The onset envelope: one value per frame, at `sample_rate / HOP` frames
/// per second, with frame `t` centred on sample `t * HOP`.
fn onset_envelope(samples: &[f32], sample_rate: u32) -> Vec<f64> {
    // Half a frame of silence on each side, so the first frame is centred
    // on the first sample.
    let pad = FRAME / 2;
    let mut padded = vec![0.0_f64; pad + samples.len() + pad];
    for (slot, &sample) in padded[pad..].iter_mut().zip(samples) {
        *slot = f64::from(sample);
    }
    let frames = (padded.len() - FRAME) / HOP + 1;

    let window: Vec<f64> = (0..FRAME)
        .map(|i| 0.5 - 0.5 * (2.0 * PI * i as f64 / (FRAME - 1) as f64).cos())
        .collect();
    let filters = mel_filters(sample_rate);
    let fft = RealFftPlanner::<f64>::new().plan_fft_forward(FRAME);
    let mut input = fft.make_input_vec();
    let mut spectrum = fft.make_output_vec();
    let mut scratch = fft.make_scratch_vec();

    let mut previous = vec![f64::NAN; MEL_BANDS];
    let mut envelope = Vec::with_capacity(frames);
    for frame in 0..frames {
        let start = frame * HOP;
        for ((slot, &sample), &gain) in input.iter_mut().zip(&padded[start..]).zip(&window) {
            *slot = sample * gain;
        }
        if fft
            .process_with_scratch(&mut input, &mut spectrum, &mut scratch)
            .is_err()
        {
            break;
        }
        // The energy of each band, in decibels, and how much louder it
        // got since the last frame. The first frame has nothing before it.
        let mut flux = 0.0;
        for (band, filter) in filters.iter().enumerate() {
            let energy: f64 = filter
                .iter()
                .map(|&(bin, weight)| weight * spectrum[bin].norm_sqr())
                .sum();
            let db = 10.0 * energy.max(1e-10).log10();
            if previous[band].is_finite() {
                flux += (db - previous[band]).max(0.0);
            }
            previous[band] = db;
        }
        envelope.push(if frame == 0 {
            0.0
        } else {
            flux / MEL_BANDS as f64
        });
    }
    envelope
}

/// The tempo, from the autocorrelation of the envelope.
fn tempo(envelope: &[f64], frame_rate: f64) -> f64 {
    if envelope.len() < 2 {
        return LIKELY_BPM;
    }
    let shortest = ((frame_rate * 60.0 / FASTEST_BPM).floor() as usize).max(1);
    let longest = ((frame_rate * 60.0 / SLOWEST_BPM).ceil() as usize).min(envelope.len() - 1);
    if longest <= shortest {
        return LIKELY_BPM;
    }
    let mean = envelope.iter().sum::<f64>() / envelope.len() as f64;
    let correlation = |lag: usize| {
        envelope[..envelope.len() - lag]
            .iter()
            .zip(&envelope[lag..])
            .map(|(a, b)| (a - mean) * (b - mean))
            .sum::<f64>()
    };
    let mut best = (shortest, f64::NEG_INFINITY);
    for lag in shortest..=longest {
        let bpm = frame_rate * 60.0 / lag as f64;
        let octaves = (bpm / LIKELY_BPM).log2();
        let pull = (-0.5 * (octaves / LIKELY_SPREAD_OCTAVES).powi(2)).exp();
        let score = correlation(lag) * pull;
        if score > best.1 {
            best = (lag, score);
        }
    }
    frame_rate * 60.0 / best.0 as f64
}

/// The envelope smoothed with a small Gaussian window, a quarter of a
/// beat wide, so that a beat is scored by the onset near it rather than
/// by the exact frame.
fn smoothed(envelope: &[f64], period: usize) -> Vec<f64> {
    let radius = (period / 4).max(1) as isize;
    let spread = radius as f64 / 2.0;
    let kernel: Vec<f64> = (-radius..=radius)
        .map(|i| (-0.5 * (i as f64 / spread).powi(2)).exp())
        .collect();
    let total: f64 = kernel.iter().sum();
    (0..envelope.len() as isize)
        .map(|i| {
            kernel
                .iter()
                .enumerate()
                .filter_map(|(k, &weight)| {
                    let j = i + k as isize - radius;
                    envelope.get(usize::try_from(j).ok()?).map(|v| v * weight)
                })
                .sum::<f64>()
                / total
        })
        .collect()
}

/// The frames the beats fall on, by dynamic programming.
fn beats(envelope: &[f64], frame_rate: f64, bpm: f64) -> Vec<usize> {
    if envelope.is_empty() || bpm <= 0.0 {
        return Vec::new();
    }
    let period = ((frame_rate * 60.0 / bpm).round() as usize).max(1);
    let strength = smoothed(envelope, period);

    // score[i]: the best total for a chain of beats ending at frame i.
    // from[i]: the beat before it in that chain, if there is one.
    let mut score = vec![0.0; envelope.len()];
    let mut from: Vec<Option<usize>> = vec![None; envelope.len()];
    score[0] = strength[0];
    for i in 1..envelope.len() {
        // The previous beat is between half a beat and two beats back.
        let earliest = i.saturating_sub(period * 2);
        let latest = i.saturating_sub((period / 2).max(1)).min(i - 1);
        let mut best: Option<(usize, f64)> = None;
        for (tau, &earlier) in score.iter().enumerate().take(latest + 1).skip(earliest) {
            let spacing = (i - tau) as f64 / period as f64;
            let candidate = earlier - TIGHTNESS * spacing.ln().powi(2);
            if best.is_none_or(|(_, b)| candidate > b) {
                best = Some((tau, candidate));
            }
        }
        match best {
            // A chain worth continuing: positive, so silence does not
            // chain beats that are not there.
            Some((tau, total)) if total > 0.0 => {
                score[i] = strength[i] + total;
                from[i] = Some(tau);
            }
            _ => score[i] = strength[i],
        }
    }

    // Walk back from the best last beat.
    let mut at = (0..envelope.len())
        .max_by(|&a, &b| score[a].total_cmp(&score[b]))
        .unwrap_or(0);
    let mut frames = vec![at];
    while let Some(previous) = from[at] {
        frames.push(previous);
        at = previous;
    }
    frames.reverse();
    frames
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Twenty seconds of short clicks, `bpm` of them a minute.
    fn clicks(bpm: f64, sample_rate: u32) -> Vec<f32> {
        let count = sample_rate as usize * 20;
        let mut samples = vec![0.0_f32; count];
        let period = 60.0 / bpm * f64::from(sample_rate);
        let mut at = 0.0;
        while (at as usize) < count {
            for i in 0..(sample_rate as usize / 100) {
                if let Some(sample) = samples.get_mut(at as usize + i) {
                    *sample += (-(i as f32) / 100.0).exp();
                }
            }
            at += period;
        }
        samples
    }

    #[test]
    fn clicks_are_found_at_their_tempo() {
        let result = track(&clicks(120.0, 22_050), 22_050);
        // The autocorrelation only knows whole frames: at 43 frames a
        // second, the lags either side of 120 BPM are 117 and 123.
        assert!((result.bpm - 120.0).abs() < 3.0, "got {} BPM", result.bpm);
        assert!(
            result.times.len() >= 35,
            "found {} beats",
            result.times.len()
        );
        // Every beat is within a couple of frames (23 ms each) of a click:
        // the beat period is rounded to whole frames, so the chain of
        // beats drifts a little before the penalty pulls it back.
        for time in &result.times {
            let nearest = (time * 2.0).round() / 2.0;
            assert!((time - nearest).abs() < 0.06, "beat at {time} s");
        }
    }

    #[test]
    fn silence_has_no_beats_to_speak_of() {
        // With nothing to correlate, some tempo wins by default, but no
        // chain of beats scores above zero: at most one lone frame.
        let result = track(&vec![0.0; 44_100], 44_100);
        assert!(result.times.len() <= 1, "found {:?}", result.times);
    }

    #[test]
    fn mel_filters_cover_the_spectrum_once() {
        let filters = mel_filters(22_050);
        assert_eq!(filters.len(), MEL_BANDS);
        // Every band has some bins, and bands overlap by half.
        assert!(filters.iter().all(|filter| !filter.is_empty()));
        let (last_of_first, _) = filters[0][filters[0].len() - 1];
        let (first_of_second, _) = filters[1][0];
        assert!(first_of_second <= last_of_first);
    }
}
