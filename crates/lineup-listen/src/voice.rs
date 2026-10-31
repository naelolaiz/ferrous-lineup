//! Hearing a tune that someone sings or whistles: its pitch moment by
//! moment, then the notes it makes, each with its start, its end, its
//! pitch and how loud it was sung.
//!
//! It works in two stages.
//!
//! **The pitch.** A sung or whistled note is a sound that repeats: the
//! same wave shape comes back every *period*, 1/440 of a second for A4.
//! The YIN method (de Cheveigné and Kawahara, 2002) finds the period by
//! comparing a short frame of the sound with itself shifted by every lag
//! `τ` from short to long: where the shifted copy lines up with the
//! original, their *difference* is close to zero, and that lag is the
//! period. Each frame can line up at more than one lag (twice the period
//! lines up too, an octave down), so every dip is kept as a *candidate*
//! with a probability, as in probabilistic YIN (pYIN, Mauch and Dixon,
//! 2014). Then a *Viterbi* search picks one candidate per frame, or
//! "no pitch", for the whole take at once: the path whose pitch moves
//! least while staying on likely candidates. A wrong octave in one frame
//! is outvoted by its neighbours.
//!
//! **The notes.** The pitch line of a voice is never flat: it slides into
//! notes, wobbles with vibrato, and drops out on consonants. The line is
//! cut into notes where the voice stops, where it moves to another pitch
//! and stays there, and where it dips in loudness and comes back (a note
//! sung again on the same pitch, "la, la"). Each note's pitch is the
//! middle of where it settled, leaving out the slide into it; its
//! loudness is the loudest moment of it.
//!
//! Pitches are fractional MIDI note numbers: 69.0 is A4 (440 Hz), 69.5 a
//! quarter tone above it. Rounding to a note of the song, and to the
//! song's grid, is left to the caller, which knows the key and the tempo
//! (see [`lineup_core::take`]).

use std::sync::Arc;

pub use lineup_core::take::SungNote;
use realfft::num_complex::Complex;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};

/// The lowest pitch looked for: about B1, below a bass singer's range.
const LOWEST_HZ: f32 = 60.0;

/// The highest pitch looked for: about E7, above a high whistle.
const HIGHEST_HZ: f32 = 2600.0;

/// How often the pitch is measured: every 10 ms.
pub const HOP_SECONDS: f64 = 0.01;

/// How long the stretch of sound compared with itself is: 25 ms, long
/// enough for one and a half periods of the lowest pitch.
const WINDOW_SECONDS: f64 = 0.025;

/// The analysis runs at a rate of at least this much: a microphone's
/// 48 kHz is divided by 3 and 44.1 kHz by 2, which keeps every pitch up
/// to [`HIGHEST_HZ`] and its first overtones, for a third or half of the
/// work.
const ANALYSIS_RATE: u32 = 16_000;

/// A frame quieter than this, in dB below full scale, is silence.
const SILENCE_DB: f32 = -60.0;

/// A frame this many dB quieter than the loud part of the take is taken
/// as background (breath, the room, a backing track heard through
/// headphones) rather than the voice.
const BACKGROUND_DB: f32 = 30.0;

/// The pitch at one moment of the take.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PitchFrame {
    /// The middle of the frame, in seconds from the start of the take.
    pub time: f64,
    /// The pitch heard, as a fractional MIDI note number, or `None` where
    /// there is no voice (silence, a breath, a consonant).
    pub pitch: Option<f32>,
    /// How loud the frame is, in dB below full scale (0 dB is the
    /// loudest a recording can be).
    pub loudness: f32,
}

/// The pitch of a take, frame by frame, [`HOP_SECONDS`] apart.
pub fn pitch_track(samples: &[f32], sample_rate: u32) -> Vec<PitchFrame> {
    let (signal, rate) = prepare(samples, sample_rate);
    let mut yin = Yin::new(rate);
    let hop = (f64::from(rate) * HOP_SECONDS).round() as usize;
    let mut frames = Vec::new();
    let mut candidates = Vec::new();
    let mut start = 0;
    while start + yin.span <= signal.len() {
        let frame = &signal[start..start + yin.span];
        let analysed = yin.analyse(frame);
        frames.push((
            (start + yin.window / 2) as f64 / f64::from(rate),
            analysed.loudness,
        ));
        candidates.push(analysed.candidates);
        start += hop;
    }
    let loud = background_level(frames.iter().map(|&(_, loudness)| loudness));
    let path = viterbi(&candidates, &frames, loud);
    frames
        .iter()
        .zip(path)
        .map(|(&(time, loudness), pitch)| PitchFrame {
            time,
            pitch,
            loudness,
        })
        .collect()
}

/// The pitch of the last moment of `samples`, for a live display while
/// recording: the pitch of the most likely candidate of one frame, or
/// `None` for silence or no clear pitch.
pub fn pitch_now(samples: &[f32], sample_rate: u32) -> Option<f32> {
    let (signal, rate) = prepare(samples, sample_rate);
    let mut yin = Yin::new(rate);
    let frame = signal.get(signal.len().checked_sub(yin.span)?..)?;
    let analysed = yin.analyse(frame);
    if analysed.loudness < SILENCE_DB + 15.0 {
        return None;
    }
    analysed
        .candidates
        .iter()
        .filter(|candidate| candidate.probability > 0.5)
        .max_by(|a, b| a.probability.total_cmp(&b.probability))
        .map(|candidate| candidate.pitch)
}

/// The notes of a pitch track.
pub fn notes(track: &[PitchFrame]) -> Vec<SungNote> {
    let mut notes = Vec::new();
    for run in voiced_runs(track) {
        let pitches = smoothed(&run.iter().map(|frame| frame.pitch).collect::<Vec<_>>());
        let loudness: Vec<f32> = run.iter().map(|frame| frame.loudness).collect();
        let mut cuts = pitch_cuts(&pitches);
        cuts.extend(loudness_cuts(&loudness));
        cuts.extend(attack_cuts(&loudness));
        cuts.sort_unstable();
        cuts.dedup();
        let mut pieces = Vec::new();
        let mut from = 0;
        for cut in cuts.into_iter().chain([run.len()]) {
            if cut > from {
                pieces.push(Piece {
                    note: note_of(&run[from..cut], &pitches[from..cut]),
                    // Whether the voice dipped in loudness before it: a
                    // note sung again, which stays a note of its own.
                    sung_again: from > 0 && sung_again(&loudness, from),
                });
                from = cut;
            }
        }
        notes.extend(tidy(pieces));
    }
    notes
}

/// How far the take is from concert pitch: the average distance, in
/// semitones from -0.5 to 0.5, between each note's pitch and the nearest
/// note of the piano, longer notes counting more. A whistle a little
/// sharp throughout gives a small positive number; subtracting it from
/// every pitch rounds each to the note meant.
pub fn tuning(notes: &[SungNote]) -> f32 {
    // The distances go round in a circle: +0.45 and -0.45 are close
    // (0.1 apart, across the half-way point), not 0.9 apart. So each is
    // turned into an angle, the angles are averaged as arrows, and the
    // average arrow is turned back.
    let (mut x, mut y) = (0.0_f64, 0.0_f64);
    for note in notes {
        let angle = f64::from(note.pitch - note.pitch.round()) * std::f64::consts::TAU;
        let weight = note.seconds().min(1.0);
        x += angle.cos() * weight;
        y += angle.sin() * weight;
    }
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    (y.atan2(x) / std::f64::consts::TAU) as f32
}

/// `samples` ready to analyse, and its rate: high-passed to take away a
/// microphone's hum and offset, and at a lower rate (see
/// [`ANALYSIS_RATE`]).
fn prepare(samples: &[f32], sample_rate: u32) -> (Vec<f32>, u32) {
    let factor = (sample_rate / ANALYSIS_RATE).max(1) as usize;
    let mut signal = high_passed(samples, sample_rate);
    if factor > 1 {
        signal = decimated(&signal, factor);
    }
    (signal, sample_rate / factor as u32)
}

/// `samples` without what is below about 40 Hz: a microphone's offset
/// from zero and the rumble of the room. A two-pole filter run forwards.
fn high_passed(samples: &[f32], sample_rate: u32) -> Vec<f32> {
    // A Butterworth high-pass "biquad", the classic recipe from Robert
    // Bristow-Johnson's audio EQ cookbook.
    let w = std::f32::consts::TAU * 40.0 / sample_rate as f32;
    let alpha = w.sin() / std::f32::consts::SQRT_2;
    let cos = w.cos();
    let a0 = 1.0 + alpha;
    let b0 = (1.0 + cos) / 2.0 / a0;
    let b1 = -(1.0 + cos) / a0;
    let b2 = b0;
    let a1 = -2.0 * cos / a0;
    let a2 = (1.0 - alpha) / a0;
    let (mut x1, mut x2, mut y1, mut y2) = (0.0, 0.0, 0.0, 0.0);
    samples
        .iter()
        .map(|&x| {
            let y = b0 * x + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
            (x2, x1, y2, y1) = (x1, x, y1, y);
            y
        })
        .collect()
}

/// Every `factor`-th sample of `samples`, after taking out what the lower
/// rate could not hold (it would fold back as false pitches): a low-pass
/// filter made of a windowed sinc.
fn decimated(samples: &[f32], factor: usize) -> Vec<f32> {
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

/// A possible pitch for one frame.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Candidate {
    /// A fractional MIDI note number.
    pitch: f32,
    /// How likely it is the frame's pitch, from 0 to 1. The candidates of
    /// a frame add up to at most 1; the rest is the chance that it has
    /// no pitch.
    probability: f32,
}

/// What one frame shows.
struct Analysed {
    candidates: Vec<Candidate>,
    loudness: f32,
}

/// The YIN pitch finder for one rate, with its FFTs planned once.
struct Yin {
    rate: u32,
    /// The stretch compared with itself, in samples.
    window: usize,
    /// The shortest and longest lags looked at.
    shortest: usize,
    longest: usize,
    /// How many samples a frame needs: the window and the longest lag.
    span: usize,
    forward: Arc<dyn RealToComplex<f32>>,
    inverse: Arc<dyn ComplexToReal<f32>>,
    /// The threshold weights of pYIN (see [`threshold_weights`]).
    weights: Vec<(f32, f32)>,
}

impl Yin {
    fn new(rate: u32) -> Self {
        let window = (f64::from(rate) * WINDOW_SECONDS).round() as usize;
        let longest = (rate as f32 / LOWEST_HZ).ceil() as usize;
        let shortest = ((rate as f32 / HIGHEST_HZ).floor() as usize).max(2);
        let span = window + longest + 1;
        let mut planner = RealFftPlanner::<f32>::new();
        let size = span.next_power_of_two();
        Yin {
            rate,
            window,
            shortest,
            longest,
            span,
            forward: planner.plan_fft_forward(size),
            inverse: planner.plan_fft_inverse(size),
            weights: threshold_weights(),
        }
    }

    /// The pitch candidates and the loudness of `frame`, which holds
    /// [`Yin::span`] samples.
    fn analyse(&mut self, frame: &[f32]) -> Analysed {
        let window = self.window;
        let energy: f32 = frame[..window].iter().map(|s| s * s).sum();
        let loudness = 10.0 * (energy / window as f32 + 1e-12).log10();
        if loudness < SILENCE_DB {
            return Analysed {
                candidates: Vec::new(),
                loudness,
            };
        }
        let difference = self.normalized_difference(frame);
        Analysed {
            candidates: self.candidates(&difference),
            loudness,
        }
    }

    /// YIN's *cumulative mean normalized difference* for every lag up to
    /// the longest: near 0 where the frame lines up with itself shifted
    /// by that lag, near 1 (or above) where it does not.
    fn normalized_difference(&self, frame: &[f32]) -> Vec<f32> {
        let window = self.window;
        let size = self.forward.len();
        // The difference at lag τ is the sum over the window of
        // (x[j] - x[j + τ])², which works out as
        //     energy of x[0..W] + energy of x[τ..τ + W] - 2 · r(τ),
        // where r(τ), the sum of x[j] · x[j + τ], is a correlation. An FFT
        // works out all the correlations at once: multiply one spectrum
        // by the other's mirror image and transform back.
        let mut first = self.forward.make_input_vec();
        first[..window].copy_from_slice(&frame[..window]);
        let mut whole = self.forward.make_input_vec();
        whole[..frame.len()].copy_from_slice(frame);
        let mut a = self.forward.make_output_vec();
        let mut b = self.forward.make_output_vec();
        let ok = self.forward.process(&mut first, &mut a).is_ok()
            && self.forward.process(&mut whole, &mut b).is_ok();
        let mut product: Vec<Complex<f32>> = a.iter().zip(&b).map(|(a, b)| a.conj() * b).collect();
        // The inverse transform needs the first and last values real.
        if let Some(first) = product.first_mut() {
            first.im = 0.0;
        }
        if let Some(last) = product.last_mut() {
            last.im = 0.0;
        }
        let mut correlation = self.inverse.make_output_vec();
        if !ok
            || self
                .inverse
                .process(&mut product, &mut correlation)
                .is_err()
        {
            return vec![1.0; self.longest + 1];
        }
        let scale = 1.0 / size as f32;

        // The energy of each shifted window, from running sums of squares.
        let mut squares = Vec::with_capacity(frame.len() + 1);
        squares.push(0.0_f64);
        for &sample in frame {
            let last = squares[squares.len() - 1];
            squares.push(last + f64::from(sample * sample));
        }
        let energy_at = |lag: usize| (squares[lag + window] - squares[lag]) as f32;
        let first_energy = energy_at(0);

        let mut normalized = vec![1.0_f32; self.longest + 1];
        let mut running = 0.0_f32;
        for lag in 1..=self.longest {
            let difference =
                (first_energy + energy_at(lag) - 2.0 * correlation[lag] * scale).max(0.0);
            running += difference;
            normalized[lag] = if running > 0.0 {
                difference * lag as f32 / running
            } else {
                1.0
            };
        }
        normalized
    }

    /// The dips of the normalized difference, each a candidate pitch with
    /// the probability pYIN gives it: for each of a range of thresholds,
    /// the first dip under the threshold (the shortest lag, the highest
    /// pitch: twice the period also dips, but later) gets the threshold's
    /// weight. A frame where no dip goes under a threshold gives its
    /// deepest dip a hundredth of the weight.
    fn candidates(&self, difference: &[f32]) -> Vec<Candidate> {
        // Each dip: its lag, refined between samples, and its depth.
        let mut dips: Vec<(f32, f32)> = Vec::new();
        for lag in self.shortest.max(2)..self.longest {
            let (before, here, after) = (difference[lag - 1], difference[lag], difference[lag + 1]);
            if here < before && here <= after && here < 1.0 {
                // A parabola through the three points has its lowest
                // point between samples: the period, more precisely.
                let curve = before - 2.0 * here + after;
                let shift = if curve > 0.0 {
                    (0.5 * (before - after) / curve).clamp(-0.5, 0.5)
                } else {
                    0.0
                };
                let depth = here - 0.25 * (before - after) * shift;
                dips.push((lag as f32 + shift, depth.max(0.0)));
            }
        }
        if dips.is_empty() {
            return Vec::new();
        }
        let mut probabilities = vec![0.0_f32; dips.len()];
        let deepest = dips
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.1.total_cmp(&b.1.1))
            .map_or(0, |(index, _)| index);
        for &(threshold, weight) in &self.weights {
            match dips.iter().position(|&(_, depth)| depth < threshold) {
                Some(first) => probabilities[first] += weight,
                None => probabilities[deepest] += weight * 0.01,
            }
        }
        dips.iter()
            .zip(probabilities)
            .filter(|&(_, probability)| probability > 0.001)
            .map(|(&(lag, _), probability)| Candidate {
                pitch: midi_of(self.rate as f32 / lag),
                probability,
            })
            .collect()
    }
}

/// The thresholds pYIN tries, from 0.01 to 1, each with how much it
/// counts: a beta distribution with its mean at 0.1, so thresholds around
/// 0.1, which YIN's authors found best, count most.
fn threshold_weights() -> Vec<(f32, f32)> {
    let (alpha, beta) = (2.0_f32, 18.0_f32);
    let raw: Vec<(f32, f32)> = (1..=100)
        .map(|i| {
            let x = i as f32 / 100.0;
            (x, x.powf(alpha - 1.0) * (1.0 - x).max(0.0).powf(beta - 1.0))
        })
        .collect();
    let total: f32 = raw.iter().map(|&(_, weight)| weight).sum();
    raw.into_iter()
        .map(|(threshold, weight)| (threshold, weight / total))
        .collect()
}

/// The MIDI note number of `hz`, with its fraction.
pub fn midi_of(hz: f32) -> f32 {
    69.0 + 12.0 * (hz / 440.0).log2()
}

/// How loud the loud part of the take is, in dB: the level that only a
/// tenth of the frames go over. The voice is the loudest thing in a take,
/// so frames far below this are background (see [`BACKGROUND_DB`]).
fn background_level(loudness: impl Iterator<Item = f32>) -> f32 {
    let mut levels: Vec<f32> = loudness.filter(|&db| db > SILENCE_DB).collect();
    if levels.is_empty() {
        return SILENCE_DB;
    }
    levels.sort_by(f32::total_cmp);
    levels[(levels.len() * 9 / 10).min(levels.len() - 1)]
}

/// Picks one candidate per frame, or none, for the whole take: the
/// cheapest path, where each frame costs how unlikely its choice is, and
/// each move from one frame to the next costs how far the pitch jumps, or
/// a fixed price to start or stop the voice.
fn viterbi(candidates: &[Vec<Candidate>], frames: &[(f64, f32)], loud: f32) -> Vec<Option<f32>> {
    /// The price of starting or stopping the voice: about the cost of a
    /// choice with a one-in-eight chance.
    const SWITCH: f32 = 2.0;
    /// The price of a jump, for each semitone past the first third of one.
    const JUMP: f32 = 0.6;

    // For each frame, the cost of each candidate (and last, of no pitch)
    // as the end of the best path so far, and which state of the frame
    // before that path came from.
    let mut costs: Vec<Vec<f32>> = Vec::with_capacity(candidates.len());
    let mut from: Vec<Vec<usize>> = Vec::with_capacity(candidates.len());
    for (t, frame) in candidates.iter().enumerate() {
        let loudness = frames[t].1;
        // A quiet frame is unlikely to be the voice, whatever its pitch.
        let quiet = (loud - BACKGROUND_DB + 10.0 - loudness).max(0.0) * 0.3;
        let voiced: f32 = frame.iter().map(|c| c.probability).sum();
        let mut emission: Vec<f32> = frame
            .iter()
            .map(|c| -c.probability.max(1e-6).ln() + quiet)
            .collect();
        emission.push(-(1.0 - voiced).max(0.02).ln());
        let unvoiced = frame.len();

        let Some(previous) = costs.last() else {
            from.push(vec![0; emission.len()]);
            costs.push(emission);
            continue;
        };
        let before = &candidates[t - 1];
        let before_unvoiced = before.len();
        let mut row = Vec::with_capacity(emission.len());
        let mut came = Vec::with_capacity(emission.len());
        for (state, &cost) in emission.iter().enumerate() {
            let mut best = (f32::INFINITY, 0);
            for (old, &old_cost) in previous.iter().enumerate() {
                let step = match (state == unvoiced, old == before_unvoiced) {
                    (true, true) => 0.0,
                    (true, false) | (false, true) => SWITCH,
                    (false, false) => {
                        let jump = (frame[state].pitch - before[old].pitch).abs();
                        JUMP * (jump - 0.33).max(0.0)
                    }
                };
                let total = old_cost + step;
                if total < best.0 {
                    best = (total, old);
                }
            }
            row.push(best.0 + cost);
            came.push(best.1);
        }
        costs.push(row);
        from.push(came);
    }

    // Back from the cheapest end, frame by frame.
    let mut path = vec![None; candidates.len()];
    let Some(last) = costs.last() else {
        return path;
    };
    let mut state = last
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(b.1))
        .map_or(0, |(index, _)| index);
    for t in (0..candidates.len()).rev() {
        path[t] = candidates[t].get(state).map(|c| c.pitch);
        state = from[t][state];
    }
    path
}

/// A stretch of frames where the voice sounds without stopping (gaps of
/// a frame or two, where the pitch finder lost it for a moment, are left
/// in, with their pitch `None`).
fn voiced_runs(track: &[PitchFrame]) -> Vec<&[PitchFrame]> {
    /// The longest gap kept inside a run: 20 ms.
    const GAP: usize = 2;
    let mut runs = Vec::new();
    // The first and the last voiced frame of the run going on.
    let mut run: Option<(usize, usize)> = None;
    for (index, frame) in track.iter().enumerate() {
        if frame.pitch.is_none() {
            continue;
        }
        run = match run {
            Some((start, last)) if index - last > GAP + 1 => {
                runs.push(&track[start..=last]);
                Some((index, index))
            }
            Some((start, _)) => Some((start, index)),
            None => Some((index, index)),
        };
    }
    if let Some((start, last)) = run {
        runs.push(&track[start..=last]);
    }
    runs
}

/// The pitches of a run with each replaced by the middle value of the
/// five around it (a *median filter*): a single wrong frame is taken out,
/// while a real change of note, which lasts, stays. Gaps take the pitch
/// before them.
fn smoothed(pitches: &[Option<f32>]) -> Vec<f32> {
    let mut filled = Vec::with_capacity(pitches.len());
    let mut last = pitches.iter().flatten().next().copied().unwrap_or(60.0);
    for pitch in pitches {
        if let Some(pitch) = pitch {
            last = *pitch;
        }
        filled.push(last);
    }
    (0..filled.len())
        .map(|i| {
            let from = i.saturating_sub(2);
            let to = (i + 3).min(filled.len());
            median(&filled[from..to])
        })
        .collect()
}

fn median(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f32::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        sorted[middle]
    } else {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    }
}

/// Where the pitch of a run moves to another note and stays there: the
/// frames that start a new note. A move counts when the pitch stays more
/// than three quarters of a semitone away from where the note settled for
/// 70 ms; vibrato, which comes back sooner, does not. (A sixteenth note
/// at 140 beats a minute lasts 107 ms.)
fn pitch_cuts(pitches: &[f32]) -> Vec<usize> {
    const AWAY: f32 = 0.75;
    const STAYS: usize = 7;
    let mut cuts = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < pitches.len() {
        // Where the note settled: the middle of its pitches so far,
        // leaving out its first frames, which slide into it.
        let settled_from = start + ((index - start) / 4).min(5);
        let settled = median(&pitches[settled_from..=index.max(settled_from)]);
        let leaves = |at: usize| (pitches[at] - settled).abs() > AWAY;
        if index > start + 2 && index + STAYS <= pitches.len() && (index..index + STAYS).all(leaves)
        {
            // Every frame of the move goes the same way: up, or down.
            let up = pitches[index] > settled;
            if (index..index + STAYS).all(|at| (pitches[at] > settled) == up) {
                cuts.push(index);
                start = index;
            }
        }
        index += 1;
    }
    cuts
}

/// Where the loudness of a run dips and comes back up, as when a note is
/// sung again on the same pitch: the quietest frame of each dip at least
/// 5 dB deep on both sides within 100 ms.
fn loudness_cuts(loudness: &[f32]) -> Vec<usize> {
    let mut cuts: Vec<usize> = Vec::new();
    for i in (1..loudness.len().saturating_sub(1)).filter(|&i| dip_at(loudness, i)) {
        // Two bottoms close together are one dip: the deeper counts.
        match cuts.last_mut() {
            Some(last) if i - *last <= 6 => {
                if loudness[i] < loudness[*last] {
                    *last = i;
                }
            }
            _ => cuts.push(i),
        }
    }
    cuts
}

/// Whether frame `i` is the bottom of a dip in loudness (see
/// [`loudness_cuts`]).
fn dip_at(loudness: &[f32], i: usize) -> bool {
    const DEPTH: f32 = 5.0;
    const REACH: usize = 10;
    let here = loudness[i];
    if !(here < loudness[i - 1] && here <= loudness[i + 1]) {
        return false;
    }
    let before = loudness[i.saturating_sub(REACH)..i]
        .iter()
        .copied()
        .fold(f32::MIN, f32::max);
    let after = loudness[i + 1..(i + 1 + REACH).min(loudness.len())]
        .iter()
        .copied()
        .fold(f32::MIN, f32::max);
    before - here >= DEPTH && after - here >= DEPTH
}

/// Where a note is struck again without the voice stopping, as a
/// whistle's tongue or an instrument does: the loudness jumps up by 6 dB
/// or more within 30 ms. The first frame of each jump.
fn attack_cuts(loudness: &[f32]) -> Vec<usize> {
    let mut cuts: Vec<usize> = Vec::new();
    for i in (ATTACK_FRAMES..loudness.len().saturating_sub(ATTACK_FRAMES))
        .filter(|&i| attack_at(loudness, i))
    {
        // One jump, however many frames it takes.
        if cuts.last().is_none_or(|&last| i - last > 6) {
            cuts.push(i);
        }
    }
    cuts
}

/// How many frames an attack takes, at most: 30 ms.
const ATTACK_FRAMES: usize = 3;

/// Whether the loudness jumps up from frame `i` (see [`attack_cuts`]).
fn attack_at(loudness: &[f32], i: usize) -> bool {
    const RISE: f32 = 6.0;
    let here = loudness[i];
    let before = loudness[i - 1];
    let after = loudness[i + 1..=i + ATTACK_FRAMES]
        .iter()
        .copied()
        .fold(f32::MIN, f32::max);
    here <= before && after - here >= RISE
}

/// Whether the note cut off at frame `at` from the one before it was sung
/// (or struck) again: the loudness dips just before it, or jumps up
/// there.
fn sung_again(loudness: &[f32], at: usize) -> bool {
    let near = at.saturating_sub(2)..(at + 1).min(loudness.len());
    near.clone()
        .any(|i| i > 0 && i + 1 < loudness.len() && dip_at(loudness, i))
        || near.into_iter().any(|i| {
            i >= ATTACK_FRAMES && i + ATTACK_FRAMES < loudness.len() && attack_at(loudness, i)
        })
}

/// The note sung over `frames`, whose smoothed pitches are `pitches`.
fn note_of(frames: &[PitchFrame], pitches: &[f32]) -> SungNote {
    let count = frames.len();
    // The middle of the note, leaving out the slide into it and the
    // fall at its end.
    let (from, to) = if count >= 5 {
        (count / 4, count - count / 10)
    } else {
        (0, count)
    };
    let heard: Vec<f32> = (from..to)
        .filter(|&i| frames[i].pitch.is_some())
        .map(|i| pitches[i])
        .collect();
    let pitch = if heard.is_empty() {
        median(pitches)
    } else {
        median(&heard)
    };
    let half_hop = HOP_SECONDS / 2.0;
    SungNote {
        start: frames[0].time - half_hop,
        end: frames[count - 1].time + half_hop,
        pitch,
        loudness: frames
            .iter()
            .map(|frame| frame.loudness)
            .fold(f32::MIN, f32::max),
    }
}

/// A stretch of a run between two cuts: a note, or part of one.
#[derive(Debug, Clone, Copy)]
struct Piece {
    note: SungNote,
    /// Whether the voice dipped in loudness just before it.
    sung_again: bool,
}

/// The notes of a run's pieces. Pieces too short to be notes sung on
/// purpose are joined to their neighbours:
///
/// - a quick slide into a note (a *scoop*), less than 120 ms a little
///   below or above it, belongs to that note, which starts where the
///   slide does; so does less than 120 ms an octave away from it, as the
///   pitch finder can hear the start of a note an octave off;
/// - anything else under 60 ms (a wobble) goes into the note before it,
///   or the one after it at the start of a run;
/// - two notes in a row on the same pitch become one, unless the second
///   was sung again.
fn tidy(pieces: Vec<Piece>) -> Vec<SungNote> {
    /// The longest scoop.
    const SCOOP: f64 = 0.12;
    /// The furthest a scoop starts from its note, in semitones.
    const SCOOP_RANGE: f32 = 1.3;
    /// The shortest note kept on its own: 60 ms.
    const SHORTEST: f64 = 0.06;

    // Scoops first, so the note they lead into gets their start.
    let mut joined: Vec<Piece> = Vec::with_capacity(pieces.len());
    let mut scoop: Option<Piece> = None;
    for mut piece in pieces {
        if let Some(before) = scoop.take() {
            let apart = (before.note.pitch - piece.note.pitch).abs();
            if !piece.sung_again && (apart < SCOOP_RANGE || (apart - 12.0).abs() < SCOOP_RANGE) {
                piece.note.start = before.note.start;
                piece.note.loudness = piece.note.loudness.max(before.note.loudness);
                piece.sung_again = before.sung_again;
            } else {
                joined.push(before);
            }
        }
        if piece.note.seconds() < SCOOP - 1e-9 {
            scoop = Some(piece);
        } else {
            joined.push(piece);
        }
    }
    joined.extend(scoop);

    let mut kept: Vec<Piece> = Vec::new();
    let mut waiting: Option<SungNote> = None;
    for mut piece in joined {
        if let Some(short) = waiting.take() {
            piece.note.start = short.start;
        }
        if piece.note.seconds() < SHORTEST - 1e-9 {
            match kept.last_mut() {
                Some(last) => last.note.end = piece.note.end,
                None => waiting = Some(piece.note),
            }
            continue;
        }
        match kept.last_mut() {
            Some(last) if !piece.sung_again && (last.note.pitch - piece.note.pitch).abs() < 0.5 => {
                // One note: its pitch the longer one's.
                if piece.note.seconds() > last.note.seconds() {
                    last.note.pitch = piece.note.pitch;
                }
                last.note.end = piece.note.end;
                last.note.loudness = last.note.loudness.max(piece.note.loudness);
            }
            _ => kept.push(piece),
        }
    }
    if let Some(short) = waiting
        && short.seconds() >= 0.03
    {
        kept.push(Piece {
            note: short,
            sung_again: false,
        });
    }
    kept.into_iter().map(|piece| piece.note).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 44_100;

    fn hz(midi: f32) -> f32 {
        440.0 * 2.0_f32.powf((midi - 69.0) / 12.0)
    }

    /// A tone at `midi`, `seconds` long, with harmonics like a voice's:
    /// each overtone softer, and a little vibrato.
    fn voice(midi: f32, seconds: f32, loudness: f32) -> Vec<f32> {
        let frames = (seconds * RATE as f32) as usize;
        let mut phase = 0.0_f32;
        (0..frames)
            .map(|i| {
                let t = i as f32 / RATE as f32;
                let vibrato = 0.25 * (std::f32::consts::TAU * 5.5 * t).sin();
                phase += hz(midi + vibrato) / RATE as f32;
                // Fade in and out over 15 ms.
                let edge = (t / 0.015).min((seconds - t) / 0.015).clamp(0.0, 1.0);
                let wave: f32 = (1..=6)
                    .map(|h| (std::f32::consts::TAU * phase * h as f32).sin() / h as f32)
                    .sum();
                wave * loudness * edge
            })
            .collect()
    }

    /// A whistle: a pure tone, no overtones.
    fn whistle(midi: f32, seconds: f32) -> Vec<f32> {
        let frames = (seconds * RATE as f32) as usize;
        (0..frames)
            .map(|i| {
                let t = i as f32 / RATE as f32;
                let edge = (t / 0.01).min((seconds - t) / 0.01).clamp(0.0, 1.0);
                (std::f32::consts::TAU * hz(midi) * t).sin() * 0.3 * edge
            })
            .collect()
    }

    fn silence(seconds: f32) -> Vec<f32> {
        vec![0.0; (seconds * RATE as f32) as usize]
    }

    fn sung(samples: &[f32]) -> Vec<SungNote> {
        notes(&pitch_track(samples, RATE))
    }

    #[test]
    fn a_held_note_is_one_note_at_its_pitch() {
        for midi in [43.0, 57.0, 69.0, 81.0] {
            let mut take = silence(0.2);
            take.extend(voice(midi, 0.8, 0.3));
            take.extend(silence(0.2));
            let notes = sung(&take);
            assert_eq!(notes.len(), 1, "{midi}: {notes:?}");
            let note = notes[0];
            assert!((note.pitch - midi).abs() < 0.2, "{midi}: {note:?}");
            assert!((note.start - 0.2).abs() < 0.03, "{note:?}");
            assert!((note.end - 1.0).abs() < 0.04, "{note:?}");
        }
    }

    #[test]
    fn a_whistled_tune_gives_its_notes() {
        // C6 D6 E6, whistled with short breaths between.
        let mut take = Vec::new();
        for midi in [84.0, 86.0, 88.0] {
            take.extend(whistle(midi, 0.3));
            take.extend(silence(0.05));
        }
        let notes = sung(&take);
        let pitches: Vec<f32> = notes.iter().map(|n| n.pitch.round()).collect();
        assert_eq!(pitches, [84.0, 86.0, 88.0], "{notes:?}");
    }

    #[test]
    fn a_legato_line_is_cut_where_the_pitch_moves() {
        // G3 A3 B3 sung without a break.
        let mut take = Vec::new();
        for midi in [55.0, 57.0, 59.0] {
            take.extend(voice(midi, 0.4, 0.3));
        }
        let notes = sung(&take);
        let pitches: Vec<f32> = notes.iter().map(|n| n.pitch.round()).collect();
        assert_eq!(pitches, [55.0, 57.0, 59.0], "{notes:?}");
        assert!((notes[1].start - 0.4).abs() < 0.06, "{notes:?}");
    }

    #[test]
    fn a_note_sung_again_is_two_notes() {
        // "La, la" on A3: the voice dips between, but does not stop.
        let mut take = voice(57.0, 0.4, 0.3);
        let quieter: Vec<f32> = voice(57.0, 0.06, 0.3).iter().map(|s| s * 0.1).collect();
        take.extend(quieter);
        take.extend(voice(57.0, 0.4, 0.3));
        let notes = sung(&take);
        assert_eq!(notes.len(), 2, "{notes:?}");
    }

    #[test]
    fn loud_notes_are_louder() {
        let mut take = voice(60.0, 0.4, 0.05);
        take.extend(silence(0.1));
        take.extend(voice(62.0, 0.4, 0.4));
        let notes = sung(&take);
        assert_eq!(notes.len(), 2, "{notes:?}");
        assert!(notes[1].loudness > notes[0].loudness + 12.0, "{notes:?}");
    }

    #[test]
    fn silence_and_noise_have_no_notes() {
        assert!(sung(&silence(1.0)).is_empty());
        // White noise from a simple generator: breath, or a fan.
        let mut seed = 1_u32;
        let noise: Vec<f32> = (0..RATE)
            .map(|_| {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (seed >> 8) as f32 / (1 << 24) as f32 * 0.2 - 0.1
            })
            .collect();
        let notes = sung(&noise);
        assert!(notes.len() <= 1, "{notes:?}");
    }

    #[test]
    fn a_take_sung_sharp_is_heard_as_sharp() {
        let mut take = Vec::new();
        for midi in [60.3, 62.3, 64.3, 65.3] {
            take.extend(voice(midi, 0.3, 0.3));
            take.extend(silence(0.05));
        }
        let notes = sung(&take);
        assert_eq!(notes.len(), 4, "{notes:?}");
        let tuning = tuning(&notes);
        assert!((tuning - 0.3).abs() < 0.1, "{tuning}");
    }

    #[test]
    fn the_pitch_of_the_last_moment_is_heard_live() {
        let take = voice(64.0, 0.3, 0.3);
        let pitch = pitch_now(&take, RATE).unwrap();
        assert!((pitch - 64.0).abs() < 0.4, "{pitch}");
        assert_eq!(pitch_now(&silence(0.3), RATE), None);
    }
}
