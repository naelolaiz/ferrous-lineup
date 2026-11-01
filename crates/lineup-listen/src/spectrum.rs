//! What the drums and the chords are heard from: how loud each
//! frequency is, moment by moment (a *spectrogram*), and a recording
//! brought down to a lower rate first when the high frequencies are not
//! needed.
//!
//! The recording is cut into short overlapping frames, each faded in and
//! out by a Hann window so the cut at its edges is not heard as a click
//! full of every frequency, and a Fourier transform (an FFT, from
//! [realfft](https://crates.io/crates/realfft)) measures how strong each
//! frequency is in each frame. Frame `i` is centred on sample `i * hop`:
//! the recording is padded with silence by half a frame at each end, so
//! the first frame hears its start.

use realfft::RealFftPlanner;

/// How loud each frequency is in each frame of a recording.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Spectrogram {
    /// The magnitudes, frame after frame: `bins` values per frame.
    pub magnitudes: Vec<f32>,
    pub frames: usize,
    pub bins: usize,
    /// How far apart two bins are, in Hz.
    pub bin_hz: f32,
    /// How far apart two frames are, in seconds.
    pub hop_seconds: f64,
}

impl Spectrogram {
    /// The spectrogram of `samples` at `sample_rate`, with frames of
    /// `window` samples, `hop` samples apart, keeping the bins up to
    /// `highest_hz`.
    pub fn new(
        samples: &[f32],
        sample_rate: u32,
        window: usize,
        hop: usize,
        highest_hz: f32,
    ) -> Spectrogram {
        let rate = sample_rate.max(1) as f32;
        let bin_hz = rate / window as f32;
        let bins = ((highest_hz / bin_hz).floor() as usize + 1).min(window / 2 + 1);
        let hop = hop.max(1);
        let frames = if samples.is_empty() {
            0
        } else {
            samples.len().div_ceil(hop)
        };
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(window);
        let mut input = fft.make_input_vec();
        let mut output = fft.make_output_vec();
        let mut scratch = fft.make_scratch_vec();
        let hann: Vec<f32> = (0..window)
            .map(|i| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / window as f32).cos())
            .collect();
        let half = window / 2;
        let mut magnitudes = vec![0.0; frames * bins];
        for frame in 0..frames {
            let centre = frame * hop;
            for (i, slot) in input.iter_mut().enumerate() {
                let at = (centre + i).checked_sub(half);
                *slot = at
                    .and_then(|at| samples.get(at))
                    .map_or(0.0, |s| s * hann[i]);
            }
            if fft
                .process_with_scratch(&mut input, &mut output, &mut scratch)
                .is_err()
            {
                break;
            }
            let row = &mut magnitudes[frame * bins..(frame + 1) * bins];
            for (value, bin) in row.iter_mut().zip(&output) {
                *value = bin.norm();
            }
        }
        Spectrogram {
            magnitudes,
            frames,
            bins,
            bin_hz,
            hop_seconds: hop as f64 / f64::from(sample_rate.max(1)),
        }
    }

    /// The magnitudes of frame `frame`.
    pub fn frame(&self, frame: usize) -> &[f32] {
        &self.magnitudes[frame * self.bins..(frame + 1) * self.bins]
    }

    /// The time frame `frame` is centred on, in seconds.
    pub fn time(&self, frame: usize) -> f64 {
        frame as f64 * self.hop_seconds
    }
}

/// `samples` at `sample_rate`, at a rate of at least `lowest` (or as it
/// is, if that is already its rate or higher): every so many samples,
/// after a low-pass filter. Returns the samples and their rate.
pub(crate) fn at_rate(samples: &[f32], sample_rate: u32, lowest: u32) -> (Vec<f32>, u32) {
    let factor = (sample_rate / lowest.max(1)).max(1) as usize;
    if factor == 1 {
        return (samples.to_vec(), sample_rate);
    }
    (decimated(samples, factor), sample_rate / factor as u32)
}

/// Every `factor`-th sample of `samples`, after taking out what the lower
/// rate could not hold (it would fold back as false pitches): a low-pass
/// filter made of a windowed sinc.
pub(crate) fn decimated(samples: &[f32], factor: usize) -> Vec<f32> {
    let half = 8 * factor;
    let cutoff = 0.9 / factor as f32; // of the old Nyquist frequency
    let taps: Vec<f32> = (0..=2 * half)
        .map(|i| {
            let n = i as f32 - half as f32;
            let sinc = if n == 0.0 {
                cutoff
            } else {
                (std::f32::consts::PI * cutoff * n).sin() / (std::f32::consts::PI * n)
            };
            let hann = 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / (2 * half) as f32).cos();
            sinc * hann
        })
        .collect();
    (0..samples.len() / factor)
        .map(|out| {
            let centre = out * factor;
            taps.iter()
                .enumerate()
                .map(|(i, tap)| {
                    let at = (centre + i).checked_sub(half);
                    at.and_then(|at| samples.get(at)).map_or(0.0, |s| s * tap)
                })
                .sum()
        })
        .collect()
}

/// The median of `values`, which it reorders: the middle one, or `0.0`
/// for none.
pub(crate) fn median_of(values: &mut [f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let middle = values.len() / 2;
    *values.select_nth_unstable_by(middle, f32::total_cmp).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tone_shows_in_the_bin_of_its_frequency_from_the_first_frame() {
        let rate = 8000;
        let tone: Vec<f32> = (0..rate)
            .map(|i| (std::f32::consts::TAU * 1000.0 * i as f32 / rate as f32).sin())
            .collect();
        let spectrogram = Spectrogram::new(&tone, rate, 512, 80, 4000.0);
        assert_eq!(spectrogram.frames, 100);
        assert!((spectrogram.time(50) - 0.5).abs() < 1e-9);
        for frame in [0, 50] {
            let row = spectrogram.frame(frame);
            let loudest = (0..row.len()).max_by(|&a, &b| row[a].total_cmp(&row[b]));
            assert_eq!(loudest, Some(64), "frame {frame}");
        }
    }

    #[test]
    fn a_lower_rate_keeps_what_it_can_hold() {
        let rate = 48_000;
        let tone: Vec<f32> = (0..rate)
            .map(|i| (std::f32::consts::TAU * 440.0 * i as f32 / rate as f32).sin())
            .collect();
        let (lower, new_rate) = at_rate(&tone, rate, 11_025);
        assert_eq!(new_rate, 12_000);
        assert_eq!(lower.len(), 12_000);
        let loudness = (lower[1000..11_000].iter().map(|s| s * s).sum::<f32>() / 10_000.0).sqrt();
        assert!((loudness - std::f32::consts::FRAC_1_SQRT_2).abs() < 0.02);
    }

    #[test]
    fn the_median_is_the_middle_value() {
        assert_eq!(median_of(&mut [3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median_of(&mut []), 0.0);
    }
}
