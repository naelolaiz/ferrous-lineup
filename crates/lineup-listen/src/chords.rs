//! Hearing the chords of a recording: which chord sounds, from when to
//! when, and which of its notes is lowest.
//!
//! The method is Mauch and Dixon's (2010), as their Chordino does it, in
//! five stages.
//!
//! **A spectrum of notes.** The recording is brought down to about
//! 11 kHz, as chords need nothing higher, and a spectrogram (see
//! [`crate::spectrum`]) with frames 370 ms long, every 46 ms, measures
//! its frequencies finely enough to tell the notes of the bass apart.
//! Its frequencies are then gathered onto a scale of notes: three values
//! to a semitone, from A0 up to G♯7.
//!
//! **Tuning.** An instrument tuned a little flat puts every note off the
//! middle of its semitone by the same amount, and the three values of
//! each semitone show by how much: averaged over the whole recording,
//! the pattern of three peaks where its middle one is, or a little to
//! one side. Its phase, as a Fourier transform at one cycle every three
//! values measures it, is how far off. The notes are then gathered again,
//! around the semitones as that instrument is tuned.
//!
//! **Whitening.** Of each value only what stands above the spectrum
//! around it (an octave of it) is kept, against how much that spectrum
//! varies (its square root: in part only): the peaks of notes stay, the
//! broad hump of a drum or of noise goes.
//!
//! **Notes, not harmonics.** A note sounds at its pitch and at twice,
//! three times, four times it, each harmonic weaker: an A2 sounds an E4
//! too. So the spectrum is explained as notes, each note as its first
//! twenty harmonics, each 0.6 as strong as the one before: the strengths
//! of the notes that add up closest to the spectrum, none of them below
//! zero (*non-negative least squares*: Mauch and Dixon's "approximate
//! transcription"). From those notes, two *chromagrams*, how strong each
//! of the twelve pitch classes is: among the low notes, the bass, and
//! among the higher ones, the chord.
//!
//! **Chords.** Every chord (twelve roots, thirteen kinds, with any of
//! its notes in the bass) and "no chord" gets a score, moment by moment:
//! how close its notes are to the chord chromagram, and its bass note to
//! the bass one, minus a little for the less common kinds and for a
//! note other than the root in the bass. A hidden Markov model then
//! finds the likeliest chords from start to end, where each change costs
//! as much as several moments of a better fit, so a chord is not changed
//! for a passing note (the Viterbi algorithm).

use lineup_core::PitchClass;
use lineup_core::harmony::{Chord, ChordQuality};
pub use lineup_core::take::HeardChord;

use crate::spectrum::{Spectrogram, at_rate};

/// The recording is brought down to at least this rate.
const LOWEST_RATE: u32 = 11_025;

/// How long a frame lasts, about: 370 ms, rounded to the nearest power
/// of two of samples (4096 at 11 025 Hz). A semitone apart at the bottom
/// of a guitar is 5 Hz apart, and this tells apart frequencies 2.7 Hz
/// apart.
const WINDOW_SECONDS: f64 = 0.37;
/// How many frames start within one frame's length: one every 46 ms.
const HOPS_PER_WINDOW: usize = 8;

/// The notes looked for: from A0, as MIDI note numbers, so many
/// semitones up, with three values to each.
const LOWEST_NOTE: f32 = 21.0;
const NOTES: usize = 84;
const BINS_PER_NOTE: usize = 3;
const BINS: usize = NOTES * BINS_PER_NOTE;

/// How many values of the note spectrum whitening looks at around each:
/// an octave.
const WHITENING_BINS: usize = 36;
/// What stands above the spectrum around is divided by how much that
/// spectrum varies to this power: halfway between keeping the loud notes
/// loud (0) and making every note as strong as any other (1).
const WHITENING_POWER: f64 = 0.5;

/// How a note is modelled: its harmonics, each this much as strong as
/// the one before.
const HARMONICS: usize = 20;
const HARMONIC_DECAY: f32 = 0.6;

/// How many times each note's strength is improved, at most.
const SWEEPS: usize = 20;

/// A moment quieter than this, against the loud moments of the
/// recording, has no chord: 40 dB down.
const QUIET: f32 = 0.01;

/// Where the bass and the chord chromagrams listen: a bell over the
/// notes centred on `_CENTRE`, reaching `_REACH` semitones either side.
const BASS_CENTRE: f32 = 40.0;
const BASS_REACH: f32 = 18.0;
const TREBLE_CENTRE: f32 = 64.0;
const TREBLE_REACH: f32 = 30.0;

/// How much the bass counts in a chord's score, against its other notes:
/// a little, as a guitar's lowest string is often not the chord's bass.
const BASS_WEIGHT: f32 = 0.2;
/// A note other than the root in the bass costs this much of a score:
/// half of what the bass can add, so that only a bass clearly on another
/// note of the chord makes it inverted.
const INVERSION: f32 = 0.1;
/// What "no chord" loses against the score of a flat chromagram.
const NO_CHORD: f32 = 0.4;
/// How much a better score weighs against a change of chord: a score is
/// worth `SHARPNESS` times itself, and a change costs `SWITCH`, as much
/// as a chord a tenth better for three frames (140 ms).
const SHARPNESS: f32 = 10.0;
const SWITCH: f32 = 30.0;

/// The kinds of chord heard, and what each costs: the common ones
/// nothing, the rarer ones a little, so they win only when their notes
/// are clearly there.
const QUALITIES: [(ChordQuality, f32); 13] = [
    (ChordQuality::Major, 0.0),
    (ChordQuality::Minor, 0.0),
    (ChordQuality::Dominant7, 0.05),
    (ChordQuality::Major7, 0.08),
    (ChordQuality::Minor7, 0.08),
    (ChordQuality::Major6, 0.1),
    (ChordQuality::Minor6, 0.12),
    (ChordQuality::HalfDiminished7, 0.1),
    (ChordQuality::Diminished, 0.12),
    (ChordQuality::Diminished7, 0.12),
    (ChordQuality::Augmented, 0.15),
    (ChordQuality::Sus4, 0.12),
    (ChordQuality::Sus2, 0.12),
];

/// The chords of `samples`, at `sample_rate`, one after the other from
/// the start of the recording to its end.
pub fn heard(samples: &[f32], sample_rate: u32) -> Vec<HeardChord> {
    if samples.is_empty() || sample_rate == 0 {
        return Vec::new();
    }
    let seconds = samples.len() as f64 / f64::from(sample_rate);
    let (samples, rate) = at_rate(samples, sample_rate, LOWEST_RATE);
    let window = nearest_power_of_two(f64::from(rate) * WINDOW_SECONDS);
    let hop = window / HOPS_PER_WINDOW;
    let highest = 440.0 * 2.0_f32.powf((LOWEST_NOTE + NOTES as f32 + 1.0 - 69.0) / 12.0);
    let spectrogram = Spectrogram::new(&samples, rate, window, hop, highest);
    let frames = spectrogram.frames;
    if frames == 0 {
        return Vec::new();
    }

    let untuned = NoteBins::new(spectrogram.bin_hz, spectrogram.bins, 0.0);
    let tuning = tuning_of(&untuned.of(&spectrogram));
    let notes = NoteBins::new(spectrogram.bin_hz, spectrogram.bins, tuning).of(&spectrogram);

    let loudness: Vec<f32> = notes.chunks(BINS).map(|frame| frame.iter().sum()).collect();
    let loud = {
        let mut sorted = loudness.clone();
        sorted.sort_by(f32::total_cmp);
        sorted[(sorted.len() * 19) / 20]
    };
    let quiet = QUIET * loud;

    let dictionary = Dictionary::new();
    let states = states();
    let mut scores = vec![0.0; frames * states.len()];
    let mut strengths = vec![0.0; NOTES];
    let mut work = Work::default();
    let mut whitened = vec![0.0; BINS];
    for frame in 0..frames {
        let row = &mut scores[frame * states.len()..(frame + 1) * states.len()];
        if loudness[frame] <= quiet || loud <= 0.0 {
            for (score, state) in row.iter_mut().zip(&states) {
                *score = if state.chord.is_none() { 1.0 } else { 0.0 };
            }
            continue;
        }
        whiten(&notes[frame * BINS..(frame + 1) * BINS], &mut whitened);
        dictionary.solve(&whitened, &mut strengths, &mut work);
        let (bass, treble) = chromagrams(&strengths);
        score(&states, &bass, &treble, row);
    }
    for score in &mut scores {
        *score *= SHARPNESS;
    }
    let path = decode(&scores, states.len(), frames, SWITCH);

    let mut heard: Vec<HeardChord> = Vec::new();
    for (frame, &index) in path.iter().enumerate() {
        let state = &states[index];
        let time = ((frame as f64 - 0.5) * spectrogram.hop_seconds).clamp(0.0, seconds);
        let bass = state
            .chord
            .and_then(|chord| (chord.root != state.bass).then_some(state.bass));
        match heard.last_mut() {
            Some(last) if last.chord == state.chord && last.bass == bass => {}
            _ => {
                if let Some(last) = heard.last_mut() {
                    last.end = time;
                }
                heard.push(HeardChord {
                    start: if heard.is_empty() { 0.0 } else { time },
                    end: seconds,
                    chord: state.chord,
                    bass,
                });
            }
        }
    }
    heard
}

/// The power of two nearest `value`, on a scale of octaves.
fn nearest_power_of_two(value: f64) -> usize {
    1 << value.max(2.0).log2().round() as u32
}

/// The frequency of MIDI note `note`, in Hz, with A4 at 440 Hz.
fn hz(note: f32) -> f32 {
    440.0 * 2.0_f32.powf((note - 69.0) / 12.0)
}

/// How a spectrogram's frequencies are gathered onto the scale of notes,
/// with each semitone `tuning` semitones off its usual place.
struct NoteBins {
    /// For each value of the note spectrum: the frequency bins that fall
    /// within it, and the place of its middle among the bins.
    bins: Vec<(std::ops::Range<usize>, f32)>,
}

impl NoteBins {
    fn new(bin_hz: f32, bins: usize, tuning: f32) -> NoteBins {
        let third = 1.0 / BINS_PER_NOTE as f32;
        let bins = (0..BINS)
            .map(|bin| {
                let note = LOWEST_NOTE + (bin as f32 - 1.0) * third + tuning;
                let low = (hz(note - third / 2.0) / bin_hz).ceil() as usize;
                let high = (hz(note + third / 2.0) / bin_hz).floor() as usize + 1;
                let middle = hz(note) / bin_hz;
                (low.min(bins)..high.min(bins), middle.min((bins - 1) as f32))
            })
            .collect();
        NoteBins { bins }
    }

    /// The note spectrum of every frame of `spectrogram`, frame after
    /// frame: each value the strongest of its frequencies, or where it
    /// has none of its own, read between the two nearest.
    fn of(&self, spectrogram: &Spectrogram) -> Vec<f32> {
        let mut out = vec![0.0; spectrogram.frames * BINS];
        for frame in 0..spectrogram.frames {
            let row = spectrogram.frame(frame);
            for (value, (range, middle)) in out[frame * BINS..(frame + 1) * BINS]
                .iter_mut()
                .zip(&self.bins)
            {
                let below = middle.floor() as usize;
                let above = (below + 1).min(row.len() - 1);
                let share = middle - below as f32;
                let between = (1.0 - share) * row[below] + share * row[above];
                *value = row[range.clone()].iter().copied().fold(between, f32::max);
            }
        }
        out
    }
}

/// How far, in semitones, the notes of `notes` (a note spectrum, frame
/// after frame) are off their usual place: from -0.5 to 0.5.
fn tuning_of(notes: &[f32]) -> f32 {
    let (mut cos, mut sin) = (0.0_f64, 0.0_f64);
    for frame in notes.chunks(BINS) {
        for (bin, &value) in frame.iter().enumerate() {
            // The middle value of each semitone is the second of its three.
            let angle = std::f64::consts::TAU * (bin as f64 - 1.0) / BINS_PER_NOTE as f64;
            cos += f64::from(value) * angle.cos();
            sin += f64::from(value) * angle.sin();
        }
    }
    if cos == 0.0 && sin == 0.0 {
        return 0.0;
    }
    (sin.atan2(cos) / std::f64::consts::TAU) as f32
}

/// `notes` (one frame of the note spectrum), whitened into `out`: less
/// the average of the octave around each value, over how much that
/// octave varies, and nothing where that is below zero.
fn whiten(notes: &[f32], out: &mut [f32]) {
    let half = WHITENING_BINS / 2;
    let mut sums = vec![0.0_f64; BINS + 1];
    let mut squares = vec![0.0_f64; BINS + 1];
    for (bin, &value) in notes.iter().enumerate() {
        sums[bin + 1] = sums[bin] + f64::from(value);
        squares[bin + 1] = squares[bin] + f64::from(value) * f64::from(value);
    }
    for (bin, value) in out.iter_mut().enumerate() {
        let range = bin.saturating_sub(half)..(bin + half + 1).min(BINS);
        let count = range.len() as f64;
        let mean = (sums[range.end] - sums[range.start]) / count;
        let spread = ((squares[range.end] - squares[range.start]) / count - mean * mean)
            .max(0.0)
            .sqrt();
        let above = f64::from(notes[bin]) - mean;
        *value = if above > 0.0 && spread > 1e-12 {
            (above / spread.powf(WHITENING_POWER)) as f32
        } else {
            0.0
        };
    }
}

/// The notes, as their harmonics sound on the note spectrum.
struct Dictionary {
    /// Each note's spectrum: the values it sounds in, and how strongly,
    /// its sum of squares 1.
    notes: Vec<Vec<(usize, f32)>>,
    /// How much each note's spectrum overlaps each other's: `gram[a *
    /// NOTES + b]`.
    gram: Vec<f32>,
}

/// What [`Dictionary::solve`] works in, kept from frame to frame.
#[derive(Default)]
struct Work {
    /// How much the spectrum sounds like each note.
    likeness: Vec<f32>,
    /// How the error changes with each note's strength.
    slope: Vec<f32>,
}

impl Dictionary {
    fn new() -> Dictionary {
        let notes: Vec<Vec<(usize, f32)>> = (0..NOTES)
            .map(|note| {
                let mut dense = vec![0.0_f32; BINS];
                let first = (BINS_PER_NOTE * note + 1) as f32;
                for harmonic in 1..=HARMONICS {
                    let at = first + 12.0 * BINS_PER_NOTE as f32 * (harmonic as f32).log2();
                    if at > BINS as f32 {
                        break;
                    }
                    let strength = HARMONIC_DECAY.powi(harmonic as i32 - 1);
                    // Each harmonic spreads over a value and a half either side.
                    let low = (at - 1.5).ceil().max(0.0) as usize;
                    let high = ((at + 1.5).floor() as usize).min(BINS - 1);
                    for (bin, value) in dense.iter_mut().enumerate().take(high + 1).skip(low) {
                        let away = (bin as f32 - at) / 1.5;
                        *value += strength * (0.5 + 0.5 * (std::f32::consts::PI * away).cos());
                    }
                }
                let size = dense.iter().map(|v| v * v).sum::<f32>().sqrt();
                dense
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| **v > 0.0)
                    .map(|(bin, v)| (bin, v / size))
                    .collect()
            })
            .collect();
        let mut gram = vec![0.0; NOTES * NOTES];
        let mut dense = vec![0.0; BINS];
        for a in 0..NOTES {
            dense.fill(0.0);
            for &(bin, value) in &notes[a] {
                dense[bin] = value;
            }
            for b in 0..NOTES {
                gram[a * NOTES + b] = notes[b].iter().map(|&(bin, v)| v * dense[bin]).sum();
            }
        }
        Dictionary { notes, gram }
    }

    /// The strengths of the notes that add up closest to `spectrum`,
    /// none below zero, into `strengths`, which starts from the last
    /// frame's: one note at a time, each set to its best with the others
    /// as they are, again and again (coordinate descent).
    fn solve(&self, spectrum: &[f32], strengths: &mut [f32], work: &mut Work) {
        work.likeness.clear();
        work.likeness.extend(
            self.notes
                .iter()
                .map(|note| note.iter().map(|&(bin, v)| v * spectrum[bin]).sum::<f32>()),
        );
        work.slope.clear();
        work.slope.extend((0..NOTES).map(|a| {
            let row = &self.gram[a * NOTES..(a + 1) * NOTES];
            row.iter()
                .zip(strengths.iter())
                .map(|(g, x)| g * x)
                .sum::<f32>()
                - work.likeness[a]
        }));
        let sweeps = SWEEPS;
        for _ in 0..sweeps {
            let mut biggest = 0.0_f32;
            let mut moved = 0.0_f32;
            for (note, strength) in strengths[..NOTES].iter_mut().enumerate() {
                let own = self.gram[note * NOTES + note];
                let best = (*strength - work.slope[note] / own).max(0.0);
                let change = best - *strength;
                if change != 0.0 {
                    *strength = best;
                    let row = &self.gram[note * NOTES..(note + 1) * NOTES];
                    for (slope, g) in work.slope.iter_mut().zip(row) {
                        *slope += change * g;
                    }
                    moved = moved.max(change.abs());
                }
                biggest = biggest.max(best);
            }
            if moved <= 1e-3 * biggest {
                break;
            }
        }
    }
}

/// The bass and the chord chromagrams of the notes' `strengths`.
fn chromagrams(strengths: &[f32]) -> ([f32; 12], [f32; 12]) {
    let bell = |note: f32, centre: f32, reach: f32| {
        let away = (note - centre) / reach;
        if away.abs() < 1.0 {
            0.5 + 0.5 * (std::f32::consts::PI * away).cos()
        } else {
            0.0
        }
    };
    let (mut bass, mut treble) = ([0.0; 12], [0.0; 12]);
    for (index, &strength) in strengths.iter().enumerate() {
        let note = LOWEST_NOTE + index as f32;
        let class = (LOWEST_NOTE as usize + index) % 12;
        bass[class] += strength * bell(note, BASS_CENTRE, BASS_REACH);
        treble[class] += strength * bell(note, TREBLE_CENTRE, TREBLE_REACH);
    }
    (bass, treble)
}

/// One state of the hidden Markov model: a chord with a note in the
/// bass, or no chord.
#[derive(Debug, Clone, Copy)]
struct State {
    chord: Option<Chord>,
    bass: PitchClass,
    /// The chord's pitch classes, a bit for each.
    notes: u16,
    /// What its kind and its bass cost.
    cost: f32,
}

/// Every chord, root position and inverted, and no chord. Two that have
/// the same notes over the same bass (C6 and Am7 over C; the four roots
/// of a diminished seventh) are one: the cheaper.
fn states() -> Vec<State> {
    let mut states: Vec<State> = Vec::new();
    for &(quality, cost) in &QUALITIES {
        for root in PitchClass::ALL {
            let chord = Chord::new(root, quality);
            let notes = quality.intervals().iter().fold(0u16, |bits, interval| {
                bits | 1 << ((root.semitone() + interval) % 12)
            });
            for &interval in quality.intervals() {
                let bass = root.transpose(i32::from(interval));
                let cost = if interval == 0 {
                    cost
                } else {
                    cost + INVERSION
                };
                let state = State {
                    chord: Some(chord),
                    bass,
                    notes,
                    cost,
                };
                match states
                    .iter_mut()
                    .find(|other| other.notes == notes && other.bass == bass)
                {
                    Some(other) if other.cost > cost => *other = state,
                    Some(_) => {}
                    None => states.push(state),
                }
            }
        }
    }
    states.push(State {
        chord: None,
        bass: PitchClass::ALL[0],
        notes: 0,
        cost: 0.0,
    });
    states
}

/// Each state's score for one frame, into `scores`: how close the
/// chord's notes are to `treble` and its bass to `bass` (the cosine of
/// the angle between them, as vectors), less its cost.
fn score(states: &[State], bass: &[f32; 12], treble: &[f32; 12], scores: &mut [f32]) {
    let size = |chroma: &[f32; 12]| chroma.iter().map(|v| v * v).sum::<f32>().sqrt().max(1e-12);
    let (bass_size, treble_size) = (size(bass), size(treble));
    let flat_treble = treble.iter().sum::<f32>() / treble_size / 12.0_f32.sqrt();
    let flat_bass = bass.iter().sum::<f32>() / bass_size / 12.0_f32.sqrt();
    for (score, state) in scores.iter_mut().zip(states) {
        *score = if state.chord.is_none() {
            flat_treble + BASS_WEIGHT * flat_bass - NO_CHORD
        } else {
            let count = state.notes.count_ones() as f32;
            let together: f32 = (0..12)
                .filter(|class| state.notes & (1 << class) != 0)
                .map(|class| treble[class])
                .sum();
            let lowest = bass[state.bass.semitone() as usize];
            together / treble_size / count.sqrt() + BASS_WEIGHT * lowest / bass_size - state.cost
        };
    }
}

/// The likeliest path through `states` states over `frames` frames,
/// given each one's `scores` (frame after frame, as log-likelihoods),
/// where moving from one state to another costs `switch`.
fn decode(scores: &[f32], states: usize, frames: usize, switch: f32) -> Vec<usize> {
    let mut best: Vec<f32> = scores[..states].to_vec();
    let mut jumped = vec![false; frames * states];
    let mut from = vec![0usize; frames];
    for frame in 1..frames {
        let (leader, top) = best
            .iter()
            .copied()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap_or((0, 0.0));
        from[frame] = leader;
        let row = &scores[frame * states..(frame + 1) * states];
        for (state, (value, &score)) in best.iter_mut().zip(row).enumerate() {
            let jump = top - switch;
            if jump > *value {
                *value = jump;
                jumped[frame * states + state] = true;
            }
            // Kept near zero, so that adding never loses precision.
            *value += score - top;
        }
    }
    let mut state = best
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map_or(0, |(state, _)| state);
    let mut path = vec![0; frames];
    for frame in (0..frames).rev() {
        path[frame] = state;
        if frame > 0 && jumped[frame * states + state] {
            state = from[frame];
        }
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 44_100;

    /// A note as an instrument plays it: its first eight harmonics, each
    /// weaker, at `cents` from the usual tuning.
    fn note(midi: f32, cents: f32, t: f32) -> f32 {
        let pitch = hz(midi + cents / 100.0);
        (1..=8)
            .map(|harmonic| {
                let h = harmonic as f32;
                (std::f32::consts::TAU * pitch * h * t).sin() / h
            })
            .sum::<f32>()
    }

    /// Chords one after the other, each `(seconds, notes)`.
    fn play(chords: &[(f32, &[f32])], cents: f32) -> Vec<f32> {
        let mut out = Vec::new();
        for &(seconds, notes) in chords {
            let length = (seconds * RATE as f32) as usize;
            out.extend((0..length).map(|i| {
                let t = i as f32 / RATE as f32;
                0.1 * notes.iter().map(|&midi| note(midi, cents, t)).sum::<f32>()
            }));
        }
        out
    }

    fn chord_at(heard: &[HeardChord], time: f64) -> Option<(Option<Chord>, Option<PitchClass>)> {
        heard
            .iter()
            .find(|chord| chord.start <= time && time < chord.end)
            .map(|chord| (chord.chord, chord.bass))
    }

    fn c(semitone: u8) -> PitchClass {
        PitchClass::from_semitone(semitone)
    }

    /// C, A minor, F, G7: each with its root in the bass.
    const PROGRESSION: [(f32, &[f32]); 4] = [
        (2.0, &[48.0, 60.0, 64.0, 67.0]),
        (2.0, &[45.0, 57.0, 60.0, 64.0]),
        (2.0, &[41.0, 57.0, 60.0, 65.0]),
        (2.0, &[43.0, 59.0, 62.0, 65.0, 67.0]),
    ];

    fn assert_progression(heard: &[HeardChord]) {
        let wanted = [
            Chord::new(c(0), ChordQuality::Major),
            Chord::new(c(9), ChordQuality::Minor),
            Chord::new(c(5), ChordQuality::Major),
            Chord::new(c(7), ChordQuality::Dominant7),
        ];
        for (index, chord) in wanted.iter().enumerate() {
            let time = 2.0 * index as f64 + 1.0;
            assert_eq!(
                chord_at(heard, time),
                Some((Some(*chord), None)),
                "at {time} s: {heard:?}"
            );
        }
        assert_eq!(heard.first().map(|chord| chord.start), Some(0.0));
        assert_eq!(heard.last().map(|chord| chord.end), Some(8.0));
    }

    #[test]
    fn a_progression_is_heard_chord_by_chord() {
        assert_progression(&heard(&play(&PROGRESSION, 0.0), RATE));
    }

    #[test]
    fn an_instrument_tuned_flat_plays_the_same_chords() {
        assert_progression(&heard(&play(&PROGRESSION, -35.0), RATE));
    }

    #[test]
    fn a_chord_over_its_third_is_heard_inverted() {
        let heard = heard(&play(&[(3.0, &[52.0, 60.0, 64.0, 67.0])], 0.0), RATE);
        assert_eq!(
            chord_at(&heard, 1.5),
            Some((Some(Chord::new(c(0), ChordQuality::Major)), Some(c(4)))),
            "{heard:?}"
        );
    }

    #[test]
    fn silence_has_no_chord() {
        let heard = heard(&vec![0.0; 2 * RATE as usize], RATE);
        assert!(heard.iter().all(|chord| chord.chord.is_none()), "{heard:?}");
        assert!(super::heard(&[], RATE).is_empty());
    }

    #[test]
    fn the_tuning_is_heard() {
        let samples = play(&PROGRESSION, 30.0);
        let (samples, rate) = at_rate(&samples, RATE, LOWEST_RATE);
        let window = nearest_power_of_two(f64::from(rate) * WINDOW_SECONDS);
        let spectrogram = Spectrogram::new(&samples, rate, window, window / 8, 5000.0);
        let bins = NoteBins::new(spectrogram.bin_hz, spectrogram.bins, 0.0);
        let tuning = tuning_of(&bins.of(&spectrogram));
        assert!((tuning - 0.3).abs() < 0.08, "{tuning}");
    }
}
