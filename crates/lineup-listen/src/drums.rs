//! Hearing the drums of a recording: when the kick, the snare and the
//! hi-hat are hit, and how hard.
//!
//! A drum kit plays several sounds at once (a kick and a hi-hat on the
//! same beat is the rule, not the exception), and often under a band
//! playing sounds of its own. So the drums are not found by finding the
//! moments something starts and then asking what it was: the sound is
//! taken apart into the drums that make it, moment by moment, and each
//! drum's hits are found in its own part. It works in five stages.
//!
//! **Bands.** A spectrogram (see [`crate::spectrum`]) every 10 ms, its
//! frequencies gathered into bands a sixth of an octave wide, from 30 Hz
//! to 16 kHz: a kick is a few bands at the bottom, a hi-hat a few at the
//! top, a snare some of both. Each band holds the average strength of
//! its frequencies, so noise, which is as strong at every frequency, is
//! as strong in every band.
//!
//! **Drums, not notes.** A held note is a line along time in the bands,
//! a drum hit a line across them. Fitzgerald's median filtering (2010)
//! keeps of each value the share that looks like a hit, so the bass and
//! the chords of a band mostly drop out before the drums are looked for.
//!
//! **Taking the sound apart.** *Non-negative matrix factorization* (NMF)
//! writes the bands `V` as the product `W·H` of two smaller tables, with
//! no negative numbers in either: each column of `W` is a *template*,
//! the spectrum of one sound, and each row of `H` that sound's
//! *activation*, how strongly it sounds in each frame. Starting from a
//! guess, the two are improved in turn by Lee and Seung's multiplicative
//! updates (2001), each step bringing `W·H` closer to `V` (closer in
//! Kullback-Leibler divergence, which weighs a quiet band as much as a
//! loud one). Four templates start as the spectra of a kick, a snare, a
//! hi-hat and a cymbal; two more start as anything at all, and learn
//! whatever else sounds: what is left of the bass, the chords, a voice.
//! That is *partially fixed* NMF (Wu and Lerch, 2015): with somewhere
//! else to go, the band's notes stay out of the drums. The drum
//! templates are let go gradually, so that they end up as the drums of
//! this very recording but each stays its drum (*semi-adaptive* NMF,
//! Dittmar and Gärtner, 2014).
//!
//! **Hits.** A drum is hit where its activation jumps: where the jump
//! peaks within 30 ms either side, and stands above the average around
//! it by a share of the drum's biggest (the peak picking of Böck, Krebs
//! and Schedl, 2012). A kick and a snare ring on and are found where
//! their activation peaks instead, which leaves out the small jumps of a
//! bass note or a strummed chord, then placed back where it rose most.
//! The drum must also make a fair share of the sound in its own bands at
//! that moment: a peak where another drum makes most of it is that drum
//! leaking into this one's template. And a snare's wires must rattle:
//! something above 1.5 kHz must start with it.
//!
//! **Open or closed.** A hi-hat that rings on is open, and one that stops
//! at once closed.
//!
//! The cymbal template is there to take the wash of the cymbals away
//! from the hi-hat, but its hits are not reported, nor are toms looked
//! for: measured on MDB Drums (see the crate's README), most of the
//! cymbal hits and nearly all of the tom hits it heard were snare wires,
//! hi-hats, guitars and kicks, and the tom templates took hits away from
//! the kick and the snare.

use lineup_core::Drum;
pub use lineup_core::take::DrumHit;

use crate::spectrum::{Spectrogram, median_of};

/// How often the drums are looked at: every 10 ms.
const HOP_SECONDS: f64 = 0.01;

/// How long a frame lasts, at least: 40 ms, rounded up to a power of two
/// (2048 samples at 44.1 or 48 kHz). Long enough for the bands of a
/// kick, short enough to tell a flam from one hit.
const WINDOW_SECONDS: f64 = 0.04;

/// The frequencies the bands cover, in Hz.
const LOWEST_HZ: f32 = 30.0;
const HIGHEST_HZ: f32 = 16_000.0;

/// How many bands an octave is cut into.
const BANDS_PER_OCTAVE: f32 = 6.0;

/// How many frames and bands the medians that tell hits from notes look
/// at: 310 ms, longer than a drum rings but shorter than a held note,
/// and an octave and a half.
const STEADY_FRAMES: usize = 31;
const SPREAD_BANDS: usize = 9;

/// How many templates learn what is not the drums. With more, they learn
/// the drums too: with six, a third of the snares of drums played alone
/// went to them.
const FREE_TEMPLATES: usize = 2;

/// How many times the templates and activations are improved.
const ITERATIONS: usize = 50;

/// How fast the drum templates are let go of their starting spectra: at
/// step `i` of `n`, the share `(1 - i/n)^ADAPTING` of each is its
/// starting spectrum (Dittmar and Gärtner's β). The higher, the sooner
/// they follow the recording.
const ADAPTING: f32 = 4.0;

/// The peak picking, in frames: a peak is the highest point within
/// [`PEAK_FRAMES`] either side, higher than the average of the
/// [`BEFORE_FRAMES`] before it to the [`AFTER_FRAMES`] after it by the
/// drum's threshold times its biggest, and at least [`APART_FRAMES`]
/// after the last hit of the same drum.
const PEAK_FRAMES: usize = 3;
const BEFORE_FRAMES: usize = 10;
const AFTER_FRAMES: usize = 7;
const APART_FRAMES: usize = 3;

/// The least share of the sound in a drum's own bands that it must make
/// at a peak to be a hit of it.
const LEAST_SHARE: f32 = 0.25;

/// A snare's wires rattle: at a snare hit, what sounds above
/// [`WIRES_HZ`] grows to at least [`WIRES_RISE`] times what it was just
/// before. A kick's thud has a snare's body in it as its pitch falls,
/// but not the rattle.
const WIRES_HZ: f32 = 1500.0;
const WIRES_RISE: f32 = 1.5;

/// A hi-hat still this strong, against its hit, 80 ms on is open.
const OPEN_RING: f32 = 0.4;
/// When an open hi-hat's ring is measured, in frames after its hit.
const RING_FRAMES: usize = 8;

/// A sound the templates tell apart, and how its hits are found.
struct Sound {
    /// The drum it is, or `None` for one whose hits are not reported.
    drum: Option<Drum>,
    /// Where its spectrum is strong to start with: peaks of `(centre in
    /// Hz, width in octaves, height)`, each a bell curve on a scale of
    /// octaves.
    peaks: &'static [(f32, f32, f32)],
    /// Whether it rings on, so its hits are found where its activation
    /// peaks rather than where it jumps.
    rings: bool,
    /// How far above the average around it a peak must stand, against
    /// the drum's biggest.
    threshold: f32,
}

const SOUNDS: [Sound; 4] = [
    // A thud between 40 and 100 Hz, and the click of the beater. Its
    // threshold is high: a bass note shows in its bands too.
    Sound {
        drum: Some(Drum::Kick),
        peaks: &[(60.0, 0.5, 1.0), (2500.0, 1.5, 0.04)],
        rings: true,
        threshold: 0.25,
    },
    // The drum's body around 200 Hz, and the rattle of its wires.
    Sound {
        drum: Some(Drum::Snare),
        peaks: &[(200.0, 0.45, 1.0), (3000.0, 1.4, 0.25)],
        rings: true,
        threshold: 0.1,
    },
    // Bright noise.
    Sound {
        drum: Some(Drum::HiHat),
        peaks: &[(9000.0, 0.7, 1.0)],
        rings: false,
        threshold: 0.1,
    },
    // Noise across the whole of the top half.
    Sound {
        drum: None,
        peaks: &[(5000.0, 1.4, 1.0)],
        rings: false,
        threshold: 0.1,
    },
];

/// The drums hit in `samples`, at `sample_rate`, in the order they were
/// hit.
pub fn hits(samples: &[f32], sample_rate: u32) -> Vec<DrumHit> {
    let rate = f64::from(sample_rate.max(1));
    let window = ((rate * WINDOW_SECONDS) as usize).next_power_of_two();
    let hop = (rate * HOP_SECONDS).round() as usize;
    let spectrogram = Spectrogram::new(samples, sample_rate, window, hop, HIGHEST_HZ);
    let bands = Bands::new(spectrogram.bin_hz, spectrogram.bins);
    let frames = spectrogram.frames;
    if frames == 0 || bands.count() == 0 {
        return Vec::new();
    }
    let heard = bands.of(&spectrogram);
    let v = percussive(&heard, bands.count(), frames);
    let starts: Vec<Vec<f32>> = SOUNDS
        .iter()
        .map(|sound| bands.template(sound.peaks))
        .collect();
    let wire_bands: Vec<usize> = (0..bands.count())
        .filter(|&band| bands.centres[band] >= WIRES_HZ)
        .collect();
    let parts = Factorization::new(&v, bands.count(), frames, &starts);

    let mut found: Vec<DrumHit> = Vec::new();
    for (index, sound) in SOUNDS.iter().enumerate() {
        let Some(drum) = sound.drum else {
            continue;
        };
        let activation = parts.activation(index);
        let jumps: Vec<f32> = (0..frames).map(|frame| jump(activation, frame)).collect();
        let curve = if sound.rings { activation } else { &jumps };
        // Where each hit starts, and where it is loudest.
        let mut kept: Vec<(usize, usize)> = Vec::new();
        for peak in peaks(curve, sound.threshold) {
            let (top, start) = if sound.rings {
                let start = (peak.saturating_sub(PEAK_FRAMES)..=peak)
                    .max_by(|&a, &b| jumps[a].total_cmp(&jumps[b]))
                    .unwrap_or(peak);
                (peak, start)
            } else {
                let top = (peak..(peak + PEAK_FRAMES).min(frames))
                    .max_by(|&a, &b| activation[a].total_cmp(&activation[b]))
                    .unwrap_or(peak);
                (top, peak)
            };
            let too_soon = kept
                .last()
                .is_some_and(|&(last, _)| start < last + APART_FRAMES);
            let rattles = drum != Drum::Snare || rises(&v, frames, &wire_bands, start);
            if !too_soon && rattles && parts.share(&v, index, top) >= LEAST_SHARE {
                kept.push((start, top));
            }
        }
        for (hit, &(start, top)) in kept.iter().enumerate() {
            let next = kept.get(hit + 1).map(|&(next, _)| next);
            let drum = if drum == Drum::HiHat && rings(&heard, frames, &starts[index], top, next) {
                Drum::OpenHat
            } else {
                drum
            };
            found.push(DrumHit {
                time: spectrogram.time(start),
                drum,
                loudness: 20.0 * activation[top].max(1e-9).log10(),
            });
        }
    }
    found.sort_by(|a, b| a.time.total_cmp(&b.time));
    found
}

/// Whether what sounds in `bands` of `v` (`frames` frames each) rises
/// at a hit starting on `start`: whether it is [`WIRES_RISE`] times as
/// strong in the frames from there as at its quietest just before.
fn rises(v: &[f32], frames: usize, bands: &[usize], start: usize) -> bool {
    let at = |frame: usize| -> f32 { bands.iter().map(|&band| v[band * frames + frame]).sum() };
    let before = (start.saturating_sub(BEFORE_FRAMES)..start.saturating_sub(1).max(1))
        .map(at)
        .fold(f32::INFINITY, f32::min);
    let before = if before.is_finite() { before } else { 0.0 };
    let hit = (start..(start + PEAK_FRAMES).min(frames))
        .map(at)
        .fold(0.0, f32::max);
    hit >= WIRES_RISE * before.max(TINY)
}

/// Whether the hi-hat hit loudest on `top` rings on: whether what sounds
/// in its bands of `heard` (the bands before the drums were told from
/// the notes, `frames` frames each, weighed by its `template`), above
/// what sounded there just before the hit, is still [`OPEN_RING`] as
/// strong [`RING_FRAMES`] later. Not when it is hit again (on `next`)
/// before then.
fn rings(heard: &[f32], frames: usize, template: &[f32], top: usize, next: Option<usize>) -> bool {
    let later = top + RING_FRAMES;
    if later >= frames || next.is_some_and(|next| next <= later) {
        return false;
    }
    let strength = |frame: usize| -> f32 {
        template
            .iter()
            .enumerate()
            .map(|(band, weight)| weight * heard[band * frames + frame])
            .sum()
    };
    let before = (top.saturating_sub(BEFORE_FRAMES)..top.saturating_sub(2).max(1))
        .map(strength)
        .fold(f32::INFINITY, f32::min);
    let before = if before.is_finite() { before } else { 0.0 };
    let peak = strength(top) - before;
    peak > 0.0 && strength(later) - before >= OPEN_RING * peak
}

/// How much `activation` rises into `frame`.
fn jump(activation: &[f32], frame: usize) -> f32 {
    match frame.checked_sub(1) {
        Some(before) => (activation[frame] - activation[before]).max(0.0),
        None => activation[frame],
    }
}

/// The frames where `curve` peaks: the highest within [`PEAK_FRAMES`]
/// either side, and above the average around it by `threshold` times
/// its highest.
fn peaks(curve: &[f32], threshold: f32) -> Vec<usize> {
    let highest = curve.iter().copied().fold(0.0, f32::max);
    if highest <= 0.0 {
        return Vec::new();
    }
    let frames = curve.len();
    let mut peaks: Vec<usize> = Vec::new();
    for (frame, &value) in curve.iter().enumerate() {
        let around = frame.saturating_sub(PEAK_FRAMES)..(frame + PEAK_FRAMES + 1).min(frames);
        // The first frame of a flat top is its peak.
        let top = curve[around].iter().all(|&other| other <= value)
            && (frame == 0 || curve[frame - 1] < value);
        if !top {
            continue;
        }
        let near = frame.saturating_sub(BEFORE_FRAMES)..(frame + AFTER_FRAMES + 1).min(frames);
        let average = curve[near.clone()].iter().sum::<f32>() / near.len() as f32;
        if value >= average + threshold * highest {
            peaks.push(frame);
        }
    }
    peaks
}

/// The part of `v` (`bands` bands of `frames` frames) that is drums
/// rather than notes: Fitzgerald's median filtering. A value is a note's
/// as much as the median along time around it, and a hit's as much as
/// the median across the bands, and is shared out between the two by
/// the squares of those medians.
fn percussive(v: &[f32], bands: usize, frames: usize) -> Vec<f32> {
    let mut out = vec![0.0; v.len()];
    let mut window: Vec<f32> = Vec::with_capacity(STEADY_FRAMES.max(SPREAD_BANDS));
    for band in 0..bands {
        let row = &v[band * frames..(band + 1) * frames];
        for frame in 0..frames {
            window.clear();
            let half = STEADY_FRAMES / 2;
            window.extend(&row[frame.saturating_sub(half)..(frame + half + 1).min(frames)]);
            let steady = median_of(&mut window);
            window.clear();
            let half = SPREAD_BANDS / 2;
            window.extend(
                (band.saturating_sub(half)..(band + half + 1).min(bands))
                    .map(|other| v[other * frames + frame]),
            );
            let spread = median_of(&mut window);
            let (steady, spread) = (steady * steady, spread * spread);
            out[band * frames + frame] = row[frame] * spread / (spread + steady + TINY);
        }
    }
    out
}

/// The bands the spectrogram is gathered into: the bins of each.
struct Bands {
    bins: Vec<std::ops::Range<usize>>,
    /// The middle of each, in Hz, on a scale of octaves.
    centres: Vec<f32>,
}

impl Bands {
    fn new(bin_hz: f32, bins: usize) -> Bands {
        let step = 2.0_f32.powf(1.0 / BANDS_PER_OCTAVE);
        let mut edges: Vec<usize> = Vec::new();
        let mut hz = LOWEST_HZ;
        while hz < HIGHEST_HZ {
            let edge = ((hz / bin_hz).round() as usize).max(1);
            if edge >= bins {
                break;
            }
            // Low down, several bands fall in one bin: they are one band.
            if edges.last() != Some(&edge) {
                edges.push(edge);
            }
            hz *= step;
        }
        edges.push(bins);
        let ranges: Vec<std::ops::Range<usize>> = edges
            .windows(2)
            .map(|pair| pair[0]..pair[1])
            .filter(|range| !range.is_empty())
            .collect();
        let centres = ranges
            .iter()
            .map(|range| {
                let low = (range.start as f32 - 0.5).max(0.5) * bin_hz;
                let high = (range.end as f32 - 0.5) * bin_hz;
                (low * high).sqrt()
            })
            .collect();
        Bands {
            bins: ranges,
            centres,
        }
    }

    fn count(&self) -> usize {
        self.bins.len()
    }

    /// The spectrogram in bands: band after band, each with a value for
    /// every frame.
    fn of(&self, spectrogram: &Spectrogram) -> Vec<f32> {
        let frames = spectrogram.frames;
        let mut v = vec![0.0; self.count() * frames];
        for frame in 0..frames {
            let row = spectrogram.frame(frame);
            for (band, range) in self.bins.iter().enumerate() {
                let sum: f32 = row[range.clone()].iter().sum();
                v[band * frames + frame] = sum / range.len() as f32;
            }
        }
        v
    }

    /// A template with `peaks`, its highest value 1.
    fn template(&self, peaks: &[(f32, f32, f32)]) -> Vec<f32> {
        let mut template: Vec<f32> = self
            .centres
            .iter()
            .map(|&hz| {
                let shape: f32 = peaks
                    .iter()
                    .map(|&(centre, width, height)| {
                        let octaves = (hz / centre).log2() / width;
                        height * (-0.5 * octaves * octaves).exp()
                    })
                    .sum();
                // A little of everything, so a template can grow anywhere.
                shape + 1e-3
            })
            .collect();
        let top = template.iter().copied().fold(0.0, f32::max);
        for value in &mut template {
            *value /= top;
        }
        template
    }
}

/// The spectrogram taken apart: `count` templates of `bands` values,
/// and how strongly each sounds in each of `frames` frames.
struct Factorization {
    bands: usize,
    frames: usize,
    count: usize,
    /// The templates, band by band: `w[band * count + template]`, each
    /// template's highest value 1.
    w: Vec<f32>,
    /// The activations, template by template: `h[template * frames +
    /// frame]`.
    h: Vec<f32>,
}

/// Small enough to add to a value to keep from dividing by zero.
const TINY: f32 = 1e-12;

impl Factorization {
    /// Takes `v` (`bands` bands of `frames` frames) apart into the
    /// templates `starts`, let go of gradually, and [`FREE_TEMPLATES`]
    /// more.
    fn new(v: &[f32], bands: usize, frames: usize, starts: &[Vec<f32>]) -> Factorization {
        let count = starts.len() + FREE_TEMPLATES;
        let mut w = vec![0.0; bands * count];
        // The free templates start as anything: a fixed sequence of
        // pseudo-random numbers, so the result is always the same.
        let mut seed: u32 = 0x9E37_79B9;
        for band in 0..bands {
            for template in 0..count {
                w[band * count + template] = match starts.get(template) {
                    Some(start) => start[band],
                    None => {
                        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                        0.5 + (seed >> 8) as f32 / (1u32 << 24) as f32
                    }
                };
            }
        }
        // Every template starts with an even share of each frame.
        let mut h = vec![0.0; count * frames];
        for frame in 0..frames {
            let total: f32 = (0..bands).map(|band| v[band * frames + frame]).sum();
            for template in 0..count {
                let weight: f32 = (0..bands).map(|band| w[band * count + template]).sum();
                h[template * frames + frame] = total / weight.max(TINY) / count as f32;
            }
        }
        let mut parts = Factorization {
            bands,
            frames,
            count,
            w,
            h,
        };
        let mut ratio = vec![0.0; bands * frames];
        let mut sums = vec![0.0; frames];
        for iteration in 0..ITERATIONS {
            parts.ratio(v, &mut ratio);
            parts.update_activations(&ratio, &mut sums);
            parts.ratio(v, &mut ratio);
            parts.update_templates(&ratio);
            let held = (1.0 - (iteration + 1) as f32 / ITERATIONS as f32).powf(ADAPTING);
            parts.hold(starts, held);
        }
        parts
    }

    /// `ratio` becomes `V / (W·H)`, value by value.
    fn ratio(&self, v: &[f32], ratio: &mut [f32]) {
        let frames = self.frames;
        for band in 0..self.bands {
            let row = &mut ratio[band * frames..(band + 1) * frames];
            row.fill(0.0);
            for template in 0..self.count {
                let weight = self.w[band * self.count + template];
                let activation = &self.h[template * frames..(template + 1) * frames];
                for (value, &a) in row.iter_mut().zip(activation) {
                    *value += weight * a;
                }
            }
            for (value, &target) in row.iter_mut().zip(&v[band * frames..(band + 1) * frames]) {
                *value = target / (*value + TINY);
            }
        }
    }

    /// One multiplicative update of the activations: `H ← H ⊙ (Wᵀ R) /
    /// (Wᵀ 1)`.
    fn update_activations(&mut self, ratio: &[f32], sums: &mut [f32]) {
        let frames = self.frames;
        for template in 0..self.count {
            sums.fill(0.0);
            let mut weight_sum = 0.0;
            for band in 0..self.bands {
                let weight = self.w[band * self.count + template];
                weight_sum += weight;
                for (sum, &r) in sums
                    .iter_mut()
                    .zip(&ratio[band * frames..(band + 1) * frames])
                {
                    *sum += weight * r;
                }
            }
            let row = &mut self.h[template * frames..(template + 1) * frames];
            for (a, &sum) in row.iter_mut().zip(sums.iter()) {
                *a *= sum / (weight_sum + TINY);
            }
        }
    }

    /// One multiplicative update of the templates: `W ← W ⊙ (R Hᵀ) /
    /// (1 Hᵀ)`.
    fn update_templates(&mut self, ratio: &[f32]) {
        let frames = self.frames;
        for template in 0..self.count {
            let activation = &self.h[template * frames..(template + 1) * frames];
            let total: f32 = activation.iter().sum();
            for band in 0..self.bands {
                let r = &ratio[band * frames..(band + 1) * frames];
                let sum: f32 = r.iter().zip(activation).map(|(r, a)| r * a).sum();
                self.w[band * self.count + template] *= sum / (total + TINY);
            }
        }
    }

    /// Brings each template's highest value back to 1, its activation
    /// making up for it, and mixes the share `held` of their starting
    /// spectra back into the templates in `starts`.
    fn hold(&mut self, starts: &[Vec<f32>], held: f32) {
        let frames = self.frames;
        for template in 0..self.count {
            let top = (0..self.bands)
                .map(|band| self.w[band * self.count + template])
                .fold(0.0, f32::max);
            if top <= TINY {
                continue;
            }
            for band in 0..self.bands {
                let value = &mut self.w[band * self.count + template];
                *value /= top;
                if let Some(start) = starts.get(template) {
                    *value = held * start[band] + (1.0 - held) * *value;
                }
            }
            for a in &mut self.h[template * frames..(template + 1) * frames] {
                *a *= top;
            }
        }
    }

    fn activation(&self, template: usize) -> &[f32] {
        &self.h[template * self.frames..(template + 1) * self.frames]
    }

    /// How much of the sound in the bands of `template` it makes on
    /// `frame`, from 0 to 1: each band counts as much as the template is
    /// strong in it.
    fn share(&self, v: &[f32], template: usize, frame: usize) -> f32 {
        let activation = self.h[template * self.frames + frame];
        let (mut made, mut heard) = (0.0, 0.0);
        for band in 0..self.bands {
            let weight = self.w[band * self.count + template];
            made += weight * weight * activation;
            heard += weight * v[band * self.frames + frame];
        }
        (made / (heard + TINY)).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 44_100;

    /// A pseudo-random number from -1 to 1.
    fn noise(seed: &mut u32) -> f32 {
        *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (*seed >> 8) as f32 / (1u32 << 23) as f32 - 1.0
    }

    /// The sound of `drum` hit once, as a drum machine makes it.
    fn hit(drum: Drum, seed: &mut u32) -> Vec<f32> {
        let rate = RATE as f32;
        let seconds = match drum {
            Drum::Kick => 0.35,
            Drum::Snare => 0.25,
            Drum::OpenHat => 0.4,
            Drum::Crash | Drum::Ride => 1.2,
            _ => 0.06,
        };
        let length = (rate * seconds) as usize;
        let mut phase = 0.0_f32;
        let (mut low, mut previous) = (0.0_f32, 0.0_f32);
        (0..length)
            .map(|i| {
                let t = i as f32 / rate;
                let mut tone = |hz: f32, decay: f32| {
                    phase += std::f32::consts::TAU * hz / rate;
                    phase.sin() * (-t / decay).exp()
                };
                let white = noise(seed);
                // A first difference makes the noise brighter.
                let bright = white - previous;
                previous = white;
                low += 0.05 * (white - low);
                match drum {
                    Drum::Kick => tone(50.0 + 90.0 * (-t / 0.03).exp(), 0.12),
                    Drum::Snare => {
                        0.6 * tone(190.0, 0.05) + 0.5 * (white - low) * (-t / 0.07).exp()
                    }
                    // Flatter and lower than a hi-hat.
                    Drum::Crash | Drum::Ride => 0.3 * (white - low) * (-t / 0.5).exp(),
                    Drum::OpenHat => 0.4 * bright * (-t / 0.15).exp(),
                    _ => 0.4 * bright * (-t / 0.015).exp(),
                }
            })
            .collect()
    }

    /// A beat of `drums` hit at `times` (in seconds), with the band's
    /// `bed` under it.
    fn beat(hits: &[(f64, Drum)], seconds: f64, bed: impl Fn(f32) -> f32) -> Vec<f32> {
        let mut seed = 7;
        let mut out: Vec<f32> = (0..(seconds * f64::from(RATE)) as usize)
            .map(|i| bed(i as f32 / RATE as f32))
            .collect();
        for &(time, drum) in hits {
            let start = (time * f64::from(RATE)) as usize;
            for (i, sample) in hit(drum, &mut seed).into_iter().enumerate() {
                if let Some(slot) = out.get_mut(start + i) {
                    *slot += sample;
                }
            }
        }
        out
    }

    /// How many of `wanted` (time, drum) were heard within 50 ms, and how
    /// many hits were heard in all.
    fn found(heard: &[DrumHit], wanted: &[(f64, Drum)]) -> usize {
        wanted
            .iter()
            .filter(|&&(time, drum)| {
                heard
                    .iter()
                    .any(|hit| hit.drum == drum && (hit.time - time).abs() <= 0.05)
            })
            .count()
    }

    /// A rock beat at 120 BPM: kick on 1 and 3, snare on 2 and 4, closed
    /// hi-hat on every eighth note, for four bars.
    fn rock_beat() -> Vec<(f64, Drum)> {
        let mut hits = Vec::new();
        for eighth in 0..32 {
            let time = 0.2 + eighth as f64 * 0.25;
            hits.push((time, Drum::HiHat));
            match eighth % 8 {
                0 | 4 => hits.push((time, Drum::Kick)),
                2 | 6 => hits.push((time, Drum::Snare)),
                _ => {}
            }
        }
        hits
    }

    #[test]
    fn a_rock_beat_is_heard_drum_by_drum() {
        let wanted = rock_beat();
        let heard = hits(&beat(&wanted, 8.6, |_| 0.0), RATE);
        let right = found(&heard, &wanted);
        assert!(
            right * 10 >= wanted.len() * 9,
            "{right} of {} found; heard {heard:?}",
            wanted.len()
        );
        // This kick's pitch falls from 140 Hz, through a snare's body, as
        // the hi-hat's rattle starts: it can be heard as a snare too.
        assert!(
            heard.len() <= wanted.len() + wanted.len() / 5,
            "{} heard for {}: {heard:?}",
            heard.len(),
            wanted.len()
        );
    }

    #[test]
    fn a_bass_and_chords_under_the_drums_are_not_heard_as_drums() {
        let wanted = rock_beat();
        // A bass on A1 and a held A minor chord.
        let bed = |t: f32| {
            let tone = |hz: f32| (std::f32::consts::TAU * hz * t).sin();
            0.3 * tone(55.0) + 0.1 * (tone(220.0) + tone(261.6) + tone(329.6))
        };
        let heard = hits(&beat(&wanted, 8.6, bed), RATE);
        let right = found(&heard, &wanted);
        assert!(
            right * 10 >= wanted.len() * 8,
            "{right} of {} found; heard {heard:?}",
            wanted.len()
        );
        assert!(
            heard.len() <= wanted.len() + wanted.len() / 5,
            "{} heard for {}",
            heard.len(),
            wanted.len()
        );
    }

    #[test]
    fn a_crash_over_the_beat_adds_no_hits() {
        let alone = hits(&beat(&rock_beat(), 8.6, |_| 0.0), RATE);
        let mut wanted = rock_beat();
        wanted.push((0.2, Drum::Crash));
        wanted.push((4.2, Drum::Crash));
        let heard = hits(&beat(&wanted, 8.6, |_| 0.0), RATE);
        assert!(
            heard.len() <= alone.len() + 1,
            "{} heard with the crashes, {} without: {heard:?}",
            heard.len(),
            alone.len()
        );
        let right = found(&heard, &rock_beat());
        assert!(
            right * 10 >= rock_beat().len() * 9,
            "{right} found: {heard:?}"
        );
    }

    #[test]
    fn a_hi_hat_left_to_ring_is_open() {
        let wanted = [
            (0.2, Drum::HiHat),
            (0.7, Drum::OpenHat),
            (1.4, Drum::HiHat),
            (1.9, Drum::OpenHat),
        ];
        let heard = hits(&beat(&wanted, 2.6, |_| 0.0), RATE);
        assert_eq!(found(&heard, &wanted), wanted.len(), "heard {heard:?}");
    }

    #[test]
    fn silence_has_no_drums() {
        assert!(hits(&vec![0.0; RATE as usize], RATE).is_empty());
        assert!(hits(&[], RATE).is_empty());
    }
}
