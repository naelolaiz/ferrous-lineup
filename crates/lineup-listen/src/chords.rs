//! Naming the chord that sounds during each beat.
//!
//! There are two steps, the classic way of doing this without machine
//! learning:
//!
//! 1. **Templates.** Each chord we look for is a set of pitch classes: C
//!    major is C, E and G. The beat's chroma is compared with the
//!    *template* of every chord (1 for its notes, 0 for the others), and
//!    the closer they are, the higher the chord scores. A bass note on
//!    the chord's root adds to the score, and so does belonging to the
//!    key, since most chords of a song do.
//! 2. **Smoothing.** Taken beat by beat, the best-scoring chord jumps
//!    around: a passing melody note can make C look like Am for one beat.
//!    Real chords last, so changing chord costs a penalty, and the
//!    *Viterbi algorithm* finds the sequence with the best total score:
//!    for each beat and each chord, it keeps the best way to arrive at
//!    that chord, and at the end walks back from the best last chord.

use lineup_core::{Chord, ChordQuality, PitchClass, Scale};

use crate::chroma::{Chroma, Frame};

/// The chords we look for. Richer chords (ninths, sus...) are hard to
/// tell apart in a recording; these cover most songs.
const QUALITIES: [ChordQuality; 6] = [
    ChordQuality::Major,
    ChordQuality::Minor,
    ChordQuality::Dominant7,
    ChordQuality::Minor7,
    ChordQuality::Major7,
    ChordQuality::Diminished,
];

/// How much a bass note on the root adds to a chord's score.
const BASS_WEIGHT: f32 = 0.15;
/// How much a chord of the key adds.
const KEY_WEIGHT: f32 = 0.08;
/// How much a four-note chord loses, so that a plain triad wins when the
/// seventh is only faint.
const SEVENTH_COST: f32 = 0.03;
/// What changing chord costs.
const CHANGE_COST: f32 = 0.25;
/// Below this score, the beat is heard as having no chord (silence,
/// drums only, speech...).
const NO_CHORD_SCORE: f32 = 0.25;

/// A chord we can recognize, or none.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Candidate {
    chord: Option<Chord>,
    template: Chroma,
}

fn candidates() -> Vec<Candidate> {
    let mut list = vec![Candidate {
        chord: None,
        template: [0.0; 12],
    }];
    for root in PitchClass::ALL {
        for quality in QUALITIES {
            let chord = Chord::new(root, quality);
            let mut template = [0.0; 12];
            for pitch_class in chord.pitch_set().iter() {
                template[usize::from(pitch_class.semitone())] = 1.0;
            }
            list.push(Candidate {
                chord: Some(chord),
                template,
            });
        }
    }
    list
}

/// `values` moved to have an average of 0 and a length of 1, so that
/// comparing two of them with a dot product gives their *correlation*:
/// 1 if they go up and down together, 0 if unrelated.
fn normalized(values: &Chroma) -> Chroma {
    let mean = values.iter().sum::<f32>() / 12.0;
    let mut centered = values.map(|v| v - mean);
    let length = centered.iter().map(|v| v * v).sum::<f32>().sqrt();
    if length > 1e-9 {
        for v in &mut centered {
            *v /= length;
        }
    }
    centered
}

fn dot(a: &Chroma, b: &Chroma) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

/// How well each candidate fits one beat.
fn scores(beat: &Frame, candidates: &[Candidate], scale: Option<Scale>) -> Vec<f32> {
    let treble = normalized(&beat.treble);
    let bass = normalized(&beat.bass);
    let bass_peak = bass.iter().copied().fold(f32::MIN, f32::max).max(1e-9);
    candidates
        .iter()
        .map(|candidate| {
            let Some(chord) = candidate.chord else {
                return NO_CHORD_SCORE;
            };
            let mut score = dot(&treble, &normalized(&candidate.template));
            score += BASS_WEIGHT * bass[usize::from(chord.root.semitone())] / bass_peak;
            if scale.is_some_and(|scale| chord.pitch_set().is_subset_of(scale.pitch_set())) {
                score += KEY_WEIGHT;
            }
            if chord.quality.intervals().len() > 3 {
                score -= SEVENTH_COST;
            }
            score
        })
        .collect()
}

/// The chord of each beat, given the average chroma of each beat and the
/// key, if known.
pub fn chords_per_beat(beats: &[Frame], scale: Option<Scale>) -> Vec<Option<Chord>> {
    if beats.is_empty() {
        return Vec::new();
    }
    let candidates = candidates();
    let count = candidates.len();

    // best[c]: the best total score of a sequence ending on candidate c.
    // from[b][c]: which candidate came before c at beat b, on that path.
    let mut best = scores(&beats[0], &candidates, scale);
    let mut from = vec![vec![0_usize; count]; beats.len()];
    for (b, beat) in beats.iter().enumerate().skip(1) {
        let here = scores(beat, &candidates, scale);
        // Changing chord costs the same from anywhere, so the best way to
        // change into any chord is from the overall best one.
        let (leader, leader_score) = best
            .iter()
            .copied()
            .enumerate()
            .fold((0, f32::MIN), |a, (i, s)| if s > a.1 { (i, s) } else { a });
        let mut next = vec![0.0; count];
        for c in 0..count {
            let (previous, total) = if best[c] >= leader_score - CHANGE_COST {
                (c, best[c])
            } else {
                (leader, leader_score - CHANGE_COST)
            };
            from[b][c] = previous;
            next[c] = total + here[c];
        }
        best = next;
    }

    // Walk back from the best last chord.
    let mut current = best
        .iter()
        .copied()
        .enumerate()
        .fold((0, f32::MIN), |a, (i, s)| if s > a.1 { (i, s) } else { a })
        .0;
    let mut path = vec![0; beats.len()];
    for b in (0..beats.len()).rev() {
        path[b] = current;
        current = from[b][current];
    }
    path.into_iter().map(|c| candidates[c].chord).collect()
}
