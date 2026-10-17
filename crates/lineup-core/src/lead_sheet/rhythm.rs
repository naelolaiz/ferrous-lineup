//! Writing durations down: which note values a bar's notes and rests are
//! written with, and where ties join them.
//!
//! A note four sixteenths long is a quarter note wherever it is, but
//! written music also shows *where the beats are*. A quarter note that
//! starts halfway through beat 2 of 4/4 hides beat 3, the middle of the
//! bar, so it is written as two eighths tied together:
//!
//! ```text
//! 4/4:  ♩ ♪ ♩ ♪ ♩       is written      ♩ ♪ ♪‿♪ ♪ ♩
//! ```
//!
//! The rules here are the usual ones of engraving (as in Elaine Gould's
//! *Behind Bars*): a note may hold over beats only if it starts on a
//! beat, never over the middle of a bar of four beats unless it starts at
//! the bar line, and in compound or irregular meters (6/8, 2+2+3/8) only
//! from beat to beat. One syncopation every reader knows is let through:
//! "♪ ♩ ♪" inside one half of the bar, a quarter from the middle of one
//! beat to the middle of the next. Rests are stricter still, so that
//! every beat can be counted.

use crate::meter::TimeSignature;

/// The plain note values, in sixteenths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Sixteenth = 1,
    Eighth = 2,
    Quarter = 4,
    Half = 8,
    Whole = 16,
}

/// A written note value: a plain value, perhaps dotted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value {
    pub base: Base,
    pub dotted: bool,
}

impl Value {
    /// The value `steps` sixteenths long, if one note can show it: 1, 2,
    /// 3 (a dotted eighth), 4, 6, 8, 12, 16 or 24 (a dotted whole note,
    /// a whole bar of 12/8).
    pub fn of(steps: usize) -> Option<Value> {
        let plain = |base| {
            Some(Value {
                base,
                dotted: false,
            })
        };
        let dotted = |base| Some(Value { base, dotted: true });
        match steps {
            1 => plain(Base::Sixteenth),
            2 => plain(Base::Eighth),
            3 => dotted(Base::Eighth),
            4 => plain(Base::Quarter),
            6 => dotted(Base::Quarter),
            8 => plain(Base::Half),
            12 => dotted(Base::Half),
            16 => plain(Base::Whole),
            24 => dotted(Base::Whole),
            _ => None,
        }
    }

    /// How many flags or beams it has: one for an eighth, two for a
    /// sixteenth, none for longer values.
    pub fn beams(self) -> usize {
        match self.base {
            Base::Sixteenth => 2,
            Base::Eighth => 1,
            _ => 0,
        }
    }
}

/// How strong a place in a bar is, counted in sixteenths from its start:
///
/// - 4 for the bar line;
/// - 3 for the middle of a bar of four even beats (beat 3 of 4/4, the
///   third dotted quarter of 12/8);
/// - 2 for any other beat;
/// - 1 for the halves of a beat (the "and" of 4/4), or the thirds of a
///   dotted beat (each eighth of 6/8);
/// - 0 for anything smaller.
pub fn strength(time: TimeSignature, offset: usize) -> u8 {
    if offset == 0 {
        return 4;
    }
    let starts = time.beat_starts();
    if starts.contains(&offset) {
        let lengths = beat_lengths(time);
        let even = lengths.windows(2).all(|pair| pair[0] == pair[1]);
        return if starts.len() == 4 && even && offset == starts[2] {
            3
        } else {
            2
        };
    }
    let beat = time.beat_at(offset);
    let part = if beat.length > 3 && beat.length.is_multiple_of(3) {
        beat.length / 3
    } else {
        beat.length / 2
    };
    if part > 0 && (offset - beat.start).is_multiple_of(part) {
        1
    } else {
        0
    }
}

fn beat_lengths(time: TimeSignature) -> Vec<usize> {
    let starts = time.beat_starts();
    let bar = time.steps_per_bar();
    starts
        .iter()
        .zip(starts.iter().skip(1).copied().chain([bar]))
        .map(|(from, to)| to - from)
        .collect()
}

/// Whether the beats are plain note values all the same length (2/4,
/// 3/4, 4/4, 2/2, 3/8...), so a note held over a beat may end halfway
/// into the next one: a dotted quarter on beat 1 of 4/4.
fn is_simple(time: TimeSignature) -> bool {
    let lengths = beat_lengths(time);
    lengths.windows(2).all(|pair| pair[0] == pair[1])
        && lengths
            .first()
            .is_some_and(|length| length.is_power_of_two())
}

/// The strongest place strictly between `from` and `to`.
fn strongest_inside(time: TimeSignature, from: usize, to: usize) -> u8 {
    (from + 1..to)
        .map(|offset| strength(time, offset))
        .max()
        .unwrap_or(0)
}

/// Whether one note can be written from `from` to `to` (sixteenths from
/// the start of a bar `bar` long) without hiding a beat.
fn note_fits(time: TimeSignature, bar: usize, from: usize, to: usize) -> bool {
    if Value::of(to - from).is_none() {
        return false;
    }
    let inside = strongest_inside(time, from, to);
    if inside < 2 {
        return true;
    }
    let start = strength(time, from);
    // A syncopation: a note one beat long from the middle of a beat to
    // the middle of the next, "♪ ♩ ♪", in simple time and inside one half
    // of a bar of four.
    if start == 1 && inside == 2 && is_simple(time) && strength(time, to) == 1 {
        return to - from == time.beat_at(from).length;
    }
    if start < 2 {
        return false;
    }
    // From a beat, a note may hold over the next beats, but not over the
    // middle of a bar of four unless it starts at the bar line.
    let limit = if start >= 3 { start } else { 3 };
    if inside >= limit {
        return false;
    }
    is_simple(time) || to == bar || strength(time, to) >= 2
}

/// Whether one rest can be written from `from` to `to`. A rest may cover
/// beats only from the bar line or the middle of a bar of four beats, as
/// a half rest does on beat 1 or 3 of 4/4, and only as an undotted
/// value; in 3/4 two beats of rest are two quarter rests. A dotted rest
/// inside a beat must start on the beat.
fn rest_fits(time: TimeSignature, bar: usize, from: usize, to: usize) -> bool {
    let Some(value) = Value::of(to - from) else {
        return false;
    };
    let inside = strongest_inside(time, from, to);
    let start = strength(time, from);
    if inside >= 2 {
        let beats = time.beat_starts().len();
        let from_strong = start >= 3 && (from > 0 || beats.is_multiple_of(2));
        return from_strong
            && !value.dotted
            && inside < start
            && (is_simple(time) || to == bar || strength(time, to) >= 2);
    }
    !value.dotted || start >= 2
}

/// Splits `from..to` into pieces that `fits` accepts, cutting at the
/// strongest place inside first. Among equally strong places, the cut
/// that leaves the longest single value first wins: a whole bar of 5/4
/// is a whole note and a quarter, not a quarter and a whole note.
fn split(
    from: usize,
    to: usize,
    fits: &dyn Fn(usize, usize) -> bool,
    strength_at: &dyn Fn(usize) -> u8,
) -> Vec<(usize, usize)> {
    if to <= from {
        return Vec::new();
    }
    if fits(from, to) {
        return vec![(from, to)];
    }
    let strongest = (from + 1..to).map(strength_at).max().unwrap_or(0);
    let places: Vec<usize> = (from + 1..to)
        .filter(|&offset| strength_at(offset) == strongest)
        .collect();
    let cut = places
        .iter()
        .copied()
        .filter(|&offset| Value::of(offset - from).is_some())
        .max()
        .or_else(|| places.first().copied())
        .unwrap_or(from + 1);
    let mut pieces = split(from, cut, fits, strength_at);
    pieces.extend(split(cut, to, fits, strength_at));
    pieces
}

/// The pieces a note from `from` to `to` is written as, tied together.
pub fn note_pieces(time: TimeSignature, bar: usize, from: usize, to: usize) -> Vec<(usize, usize)> {
    split(from, to, &|a, b| note_fits(time, bar, a, b), &|offset| {
        strength(time, offset)
    })
}

/// The rests that fill `from..to`.
pub fn rest_pieces(time: TimeSignature, bar: usize, from: usize, to: usize) -> Vec<(usize, usize)> {
    split(from, to, &|a, b| rest_fits(time, bar, a, b), &|offset| {
        strength(time, offset)
    })
}

/// Which notes are beamed together: the start of each group, as places
/// in the bar. Eighths and sixteenths are beamed beat by beat, or by
/// the whole bar when the beats are themselves eighths (3/8).
pub fn beam_groups(time: TimeSignature) -> Vec<usize> {
    let starts = time.beat_starts();
    if beat_lengths(time).iter().all(|&length| length < 4) {
        vec![0]
    } else {
        starts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOUR_FOUR: TimeSignature = TimeSignature::new(4, 4);
    const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
    const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);

    fn notes(time: TimeSignature, from: usize, to: usize) -> Vec<usize> {
        note_pieces(time, time.steps_per_bar(), from, to)
            .into_iter()
            .map(|(a, b)| b - a)
            .collect()
    }

    fn rests(time: TimeSignature, from: usize, to: usize) -> Vec<usize> {
        rest_pieces(time, time.steps_per_bar(), from, to)
            .into_iter()
            .map(|(a, b)| b - a)
            .collect()
    }

    #[test]
    fn places_have_their_strength() {
        let four: Vec<u8> = (0..16).map(|i| strength(FOUR_FOUR, i)).collect();
        assert_eq!(four, [4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0]);
        let six: Vec<u8> = (0..12).map(|i| strength(SIX_EIGHT, i)).collect();
        assert_eq!(six, [4, 0, 1, 0, 1, 0, 2, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn notes_on_the_beat_hold_over_beats() {
        assert_eq!(notes(FOUR_FOUR, 0, 16), [16]);
        assert_eq!(notes(FOUR_FOUR, 0, 12), [12]);
        assert_eq!(notes(FOUR_FOUR, 8, 16), [8]);
        // A dotted quarter on beat 1 or 3.
        assert_eq!(notes(FOUR_FOUR, 0, 6), [6]);
        assert_eq!(notes(FOUR_FOUR, 8, 14), [6]);
        // In 3/4, a half note on beat 2.
        assert_eq!(notes(THREE_FOUR, 4, 12), [8]);
        assert_eq!(notes(THREE_FOUR, 0, 12), [12]);
    }

    #[test]
    fn the_middle_of_a_bar_of_four_is_shown() {
        // A half note on beat 2 of 4/4 is two tied quarters.
        assert_eq!(notes(FOUR_FOUR, 4, 12), [4, 4]);
        assert_eq!(notes(FOUR_FOUR, 4, 16), [4, 8]);
        assert_eq!(notes(FOUR_FOUR, 4, 10), [4, 2]);
    }

    #[test]
    fn a_note_off_the_beat_does_not_hide_the_next_beat() {
        assert_eq!(notes(FOUR_FOUR, 2, 8), [2, 4]);
        assert_eq!(notes(FOUR_FOUR, 2, 5), [2, 1]);
        assert_eq!(notes(FOUR_FOUR, 1, 16), [3, 4, 8]);
        // Inside a beat anything goes: sixteenth, eighth, sixteenth.
        assert_eq!(notes(FOUR_FOUR, 1, 3), [2]);
        assert_eq!(notes(FOUR_FOUR, 1, 4), [3]);
    }

    #[test]
    fn simple_syncopations_are_not_tied() {
        // ♪ ♩ ♪ in each half of 4/4, but not across its middle.
        assert_eq!(notes(FOUR_FOUR, 2, 6), [4]);
        assert_eq!(notes(FOUR_FOUR, 10, 14), [4]);
        assert_eq!(notes(FOUR_FOUR, 6, 10), [2, 2]);
        // ♪ ♩ ♩ ♪ in 3/4.
        assert_eq!(notes(THREE_FOUR, 2, 6), [4]);
        assert_eq!(notes(THREE_FOUR, 6, 10), [4]);
        // Not in 6/8, where beats are dotted.
        assert_eq!(notes(SIX_EIGHT, 4, 8), [2, 2]);
    }

    #[test]
    fn compound_beats_are_kept_whole() {
        assert_eq!(notes(SIX_EIGHT, 0, 6), [6]);
        assert_eq!(notes(SIX_EIGHT, 0, 12), [12]);
        // A quarter and an eighth inside a beat.
        assert_eq!(notes(SIX_EIGHT, 0, 4), [4]);
        assert_eq!(notes(SIX_EIGHT, 2, 6), [4]);
        // Four eighths from the start: a dotted quarter tied to an eighth.
        assert_eq!(notes(SIX_EIGHT, 0, 8), [6, 2]);
        let seven = TimeSignature::grouped(&[2, 2, 3], 8).unwrap();
        assert_eq!(notes(seven, 0, 8), [8]);
        assert_eq!(notes(seven, 8, 14), [6]);
        assert_eq!(notes(seven, 4, 14), [4, 6]);
    }

    #[test]
    fn uneven_bars_are_split_at_beats() {
        let five = TimeSignature::new(5, 4);
        assert_eq!(notes(five, 0, 20), [16, 4]);
    }

    #[test]
    fn rests_show_every_beat() {
        assert_eq!(rests(FOUR_FOUR, 0, 8), [8]);
        assert_eq!(rests(FOUR_FOUR, 0, 12), [8, 4]);
        assert_eq!(rests(FOUR_FOUR, 4, 12), [4, 4]);
        assert_eq!(rests(FOUR_FOUR, 0, 6), [4, 2]);
        assert_eq!(rests(THREE_FOUR, 4, 12), [4, 4]);
        assert_eq!(rests(THREE_FOUR, 0, 8), [4, 4]);
        assert_eq!(rests(FOUR_FOUR, 1, 4), [1, 2]);
        assert_eq!(rests(FOUR_FOUR, 0, 3), [3]);
        assert_eq!(rests(SIX_EIGHT, 0, 6), [6]);
        assert_eq!(rests(SIX_EIGHT, 6, 12), [6]);
    }

    #[test]
    fn eighths_are_beamed_by_the_beat() {
        assert_eq!(beam_groups(FOUR_FOUR), [0, 4, 8, 12]);
        assert_eq!(beam_groups(SIX_EIGHT), [0, 6]);
        assert_eq!(beam_groups(TimeSignature::new(3, 8)), [0]);
    }

    #[test]
    fn every_piece_is_a_written_value() {
        for time in [FOUR_FOUR, THREE_FOUR, SIX_EIGHT, TimeSignature::new(5, 8)] {
            let bar = time.steps_per_bar();
            for from in 0..bar {
                for to in from + 1..=bar {
                    let pieces = note_pieces(time, bar, from, to);
                    assert_eq!(pieces.first().unwrap().0, from);
                    assert_eq!(pieces.last().unwrap().1, to);
                    for pair in pieces.windows(2) {
                        assert_eq!(pair[0].1, pair[1].0);
                    }
                    for (a, b) in pieces.into_iter().chain(rest_pieces(time, bar, from, to)) {
                        assert!(Value::of(b - a).is_some(), "{a}..{b} in {time:?}");
                    }
                }
            }
        }
    }
}
