//! Guessing the meter (3 or 4 beats to the bar) and where bars start.
//!
//! Chords mostly change at the start of a bar. So for each way of
//! grouping the beats (in threes or fours, starting on any of them) we
//! count how many chord changes land on the first beat of a group, and
//! keep the grouping where that happens far more often than chance.

use lineup_core::Chord;

/// How much more likely 4/4 is than 3/4 before listening, as a head start
/// in the score: most popular music is in four.
const FOUR_PRIOR: f64 = 0.05;

/// `(beats per bar, first downbeat)` for chords given one per beat.
pub fn guess(chords: &[Option<Chord>]) -> (usize, usize) {
    let changes: Vec<usize> = (1..chords.len())
        .filter(|&i| chords[i] != chords[i - 1] && chords[i].is_some())
        .collect();
    if changes.len() < 4 {
        return (4, 0);
    }
    let mut best = (4, 0, f64::MIN);
    for beats in [4, 3] {
        for phase in 0..beats {
            let on_phase = changes.iter().filter(|&&i| i % beats == phase).count();
            let mut score = on_phase as f64 / changes.len() as f64 - 1.0 / beats as f64;
            if beats == 4 {
                score += FOUR_PRIOR;
            }
            if score > best.2 {
                best = (beats, phase, score);
            }
        }
    }
    (best.0, best.1)
}
