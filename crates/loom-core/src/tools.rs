//! Tools that change a group of notes at once, for the piano roll.
//!
//! Each tool is a plain function: it takes some notes and returns new
//! ones, and changes nothing itself. The piano roll hands the result to
//! [`Pattern::replace_notes`](crate::Pattern::replace_notes), through
//! the app's `edit`, so every tool can be undone like any other change.
//!
//! Tools that could push a note out of the MIDI range return an
//! `Option`, and `None` means "this would not work, so do nothing": a
//! chord is never transposed with one of its notes missing.

use crate::{Note, NoteEvent, Scale};

/// The notes grouped by the step they start on, each group lowest
/// first, the groups in time order. A group of two or more notes is a
/// chord; a group of one is a single note.
pub fn chords(notes: &[NoteEvent]) -> Vec<Vec<NoteEvent>> {
    let mut sorted = notes.to_vec();
    sorted.sort_by_key(|note| (note.start, note.pitch));
    // `chunk_by` cuts the sorted list wherever the start changes.
    sorted
        .chunk_by(|a, b| a.start == b.start)
        .map(<[NoteEvent]>::to_vec)
        .collect()
}

/// Moves every note up (or down, if negative) by some semitones.
pub fn transpose(notes: &[NoteEvent], semitones: i32) -> Option<Vec<NoteEvent>> {
    // Collecting an iterator of `Option`s into `Option<Vec<_>>` gives
    // `None` as soon as one item is `None`, and the whole `Vec` otherwise.
    notes
        .iter()
        .map(|note| {
            Some(NoteEvent {
                pitch: note.pitch.transpose(semitones)?,
                ..*note
            })
        })
        .collect()
}

/// Moves every note later (or earlier, if negative) by some steps.
/// `None` if a note would start before the pattern or at its end.
pub fn shift(notes: &[NoteEvent], steps: isize, pattern_length: usize) -> Option<Vec<NoteEvent>> {
    notes
        .iter()
        .map(|note| {
            let start = note.start.checked_add_signed(steps)?;
            (start < pattern_length).then_some(NoteEvent { start, ..*note })
        })
        .collect()
}

/// The nearest note of the scale above `note` (or below, if `up` is
/// false), not counting `note` itself.
pub fn step_in_key(note: Note, scale: Scale, up: bool) -> Option<Note> {
    let direction = if up { 1 } else { -1 };
    (1..=12).find_map(|distance| {
        note.transpose(direction * distance)
            .filter(|candidate| scale.contains(candidate.pitch_class()))
    })
}

/// Moves every note some notes of the scale up (or down, if negative):
/// in C major, one step up turns C, E, G into D, F, A. The chord stays in
/// the key, so its kind changes with its root: C major becomes D minor.
pub fn transpose_in_key(notes: &[NoteEvent], scale: Scale, steps: i32) -> Option<Vec<NoteEvent>> {
    notes
        .iter()
        .map(|note| {
            let mut pitch = note.pitch;
            for _ in 0..steps.unsigned_abs() {
                pitch = step_in_key(pitch, scale, steps > 0)?;
            }
            Some(NoteEvent { pitch, ..*note })
        })
        .collect()
}

/// The note of the scale nearest to `note`: `note` itself if it is in the
/// scale. When one above and one below are as near, the lower one wins.
pub fn nearest_in_key(note: Note, scale: Scale) -> Note {
    if scale.contains(note.pitch_class()) {
        return note;
    }
    (1..=6)
        .find_map(|distance| {
            [-distance, distance].into_iter().find_map(|semitones| {
                note.transpose(semitones)
                    .filter(|candidate| scale.contains(candidate.pitch_class()))
            })
        })
        .unwrap_or(note)
}

/// Moves every note that is not in the scale to the nearest one that is.
pub fn snap_to_scale(notes: &[NoteEvent], scale: Scale) -> Vec<NoteEvent> {
    notes
        .iter()
        .map(|note| NoteEvent {
            pitch: nearest_in_key(note.pitch, scale),
            ..*note
        })
        .collect()
}

/// `note` moved up by octaves until it is above `ceiling`.
fn above(note: Note, ceiling: Note) -> Option<Note> {
    let mut note = note;
    while note <= ceiling {
        note = note.transpose(12)?;
    }
    Some(note)
}

/// `note` moved down by octaves until it is below `floor`.
fn below(note: Note, floor: Note) -> Option<Note> {
    let mut note = note;
    while note >= floor {
        note = note.transpose(-12)?;
    }
    Some(note)
}

/// Turns a chord over `times` times: each time its lowest note moves up,
/// above the highest. C, E, G once becomes E, G, C (the first inversion),
/// twice G, C, E (the second). Same notes, a different one in the bass.
pub fn inversion(pitches: &[Note], times: usize) -> Vec<Note> {
    let mut notes = pitches.to_vec();
    notes.sort();
    for _ in 0..times % notes.len().max(1) {
        let (Some(&lowest), Some(&highest)) = (notes.first(), notes.last()) else {
            break;
        };
        let Some(moved) = above(lowest, highest) else {
            break; // no room above: leave the chord as it is
        };
        notes.remove(0);
        notes.push(moved);
    }
    notes
}

/// Inverts every chord once: up moves each chord's lowest note above its
/// highest, down moves the highest below the lowest. Single notes stay.
pub fn invert(notes: &[NoteEvent], up: bool) -> Option<Vec<NoteEvent>> {
    let mut result = Vec::with_capacity(notes.len());
    for mut chord in chords(notes) {
        if let (Some(&lowest), Some(&highest)) = (chord.first(), chord.last())
            && chord.len() >= 2
        {
            if up {
                chord[0].pitch = above(lowest.pitch, highest.pitch)?;
            } else {
                let last = chord.len() - 1;
                chord[last].pitch = below(highest.pitch, lowest.pitch)?;
            }
            chord.sort_by_key(|note| note.pitch);
        }
        result.extend(chord);
    }
    Some(result)
}

/// The order an arpeggio plays a chord's notes in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArpeggioOrder {
    #[default]
    Up,
    Down,
    UpDown,
}

impl ArpeggioOrder {
    pub const ALL: [ArpeggioOrder; 3] = [
        ArpeggioOrder::Up,
        ArpeggioOrder::Down,
        ArpeggioOrder::UpDown,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ArpeggioOrder::Up => "up",
            ArpeggioOrder::Down => "down",
            ArpeggioOrder::UpDown => "up and down",
        }
    }

    /// Which note of a chord of `count` notes to play next, counted from
    /// the lowest, forever: up is 0, 1, 2, 0, 1, 2... and up and down is
    /// 0, 1, 2, 1, 0, 1, 2, 1... (the top and bottom notes are not repeated).
    fn indices(self, count: usize) -> impl Iterator<Item = usize> {
        let once: Vec<usize> = match self {
            ArpeggioOrder::Up => (0..count).collect(),
            ArpeggioOrder::Down => (0..count).rev().collect(),
            ArpeggioOrder::UpDown => (0..count)
                .chain((1..count.saturating_sub(1)).rev())
                .collect(),
        };
        // `cycle` repeats an iterator forever. It is lazy, so "forever"
        // costs nothing: the caller takes only as many as it needs.
        once.into_iter().cycle()
    }
}

/// Turns every chord into an arpeggio: its notes one after another, each
/// `rate` steps long, for as long as the chord lasted, but never into the
/// next chord or note. Single notes stay as they are.
pub fn arpeggiate(notes: &[NoteEvent], order: ArpeggioOrder, rate: usize) -> Vec<NoteEvent> {
    let rate = rate.max(1);
    let chords = chords(notes);
    let mut result = Vec::new();
    for (index, chord) in chords.iter().enumerate() {
        if chord.len() < 2 {
            result.extend(chord);
            continue;
        }
        let start = chord[0].start;
        let mut end = chord.iter().map(NoteEvent::end).max().unwrap_or(start + 1);
        if let Some(next) = chords.get(index + 1) {
            end = end.min(next[0].start);
        }
        // `step_by` takes every `rate`-th step; `zip` pairs each of those
        // steps with the next note of the order, and stops when the steps
        // run out (the order never does).
        for (step, pick) in (start..end).step_by(rate).zip(order.indices(chord.len())) {
            result.push(NoteEvent {
                start: step,
                length: rate.min(end - step),
                ..chord[pick]
            });
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PitchClass, ScaleKind};

    fn note(midi: u8, start: usize, length: usize) -> NoteEvent {
        NoteEvent::new(Note::new(midi), start, length)
    }

    fn pitches(notes: &[NoteEvent]) -> Vec<u8> {
        notes.iter().map(|note| note.pitch.midi()).collect()
    }

    const C_MAJOR: Scale = Scale {
        root: PitchClass::C,
        kind: ScaleKind::Major,
    };

    /// C major, then F major a bar later.
    fn two_chords() -> Vec<NoteEvent> {
        vec![
            note(67, 0, 16),
            note(60, 0, 16),
            note(64, 0, 16),
            note(65, 16, 16),
            note(69, 16, 16),
            note(72, 16, 16),
        ]
    }

    #[test]
    fn notes_are_grouped_into_chords() {
        let groups = chords(&two_chords());
        assert_eq!(groups.len(), 2);
        assert_eq!(pitches(&groups[0]), [60, 64, 67], "lowest first");
        assert_eq!(pitches(&groups[1]), [65, 69, 72]);
    }

    #[test]
    fn transposing_keeps_everything_but_the_pitch() {
        let notes = [note(60, 3, 2)];
        assert_eq!(transpose(&notes, 7), Some(vec![note(67, 3, 2)]));
        assert_eq!(transpose(&notes, -12), Some(vec![note(48, 3, 2)]));
        // One note too high spoils the whole transposition.
        assert_eq!(transpose(&[note(60, 0, 1), note(120, 0, 1)], 12), None);
    }

    #[test]
    fn shifting_stays_inside_the_pattern() {
        let notes = [note(60, 4, 1), note(64, 8, 1)];
        assert_eq!(
            shift(&notes, 2, 16),
            Some(vec![note(60, 6, 1), note(64, 10, 1)])
        );
        assert_eq!(shift(&notes, -5, 16), None, "before the start");
        assert_eq!(shift(&notes, 8, 16), None, "past the end");
    }

    #[test]
    fn moving_in_key_changes_the_chord() {
        // C major one step up in C major is D minor: D, F, A.
        let c_major = [note(60, 0, 4), note(64, 0, 4), note(67, 0, 4)];
        let up = transpose_in_key(&c_major, C_MAJOR, 1).unwrap();
        assert_eq!(pitches(&up), [62, 65, 69]);
        // Two steps down from C, E, G: A, C, E (A minor).
        let down = transpose_in_key(&c_major, C_MAJOR, -2).unwrap();
        assert_eq!(pitches(&down), [57, 60, 64]);
        // A note outside the key moves to the next one inside it.
        let c_sharp = [note(61, 0, 1)];
        assert_eq!(
            pitches(&transpose_in_key(&c_sharp, C_MAJOR, 1).unwrap()),
            [62]
        );
    }

    #[test]
    fn snapping_moves_only_notes_outside_the_key() {
        let notes = [
            note(60, 0, 1),
            note(61, 1, 1),
            note(66, 2, 1),
            note(70, 3, 1),
        ];
        // C stays. C#, F# and Bb are each one semitone from two notes of
        // the key, so the lower one wins: C, F and A.
        assert_eq!(pitches(&snap_to_scale(&notes, C_MAJOR)), [60, 60, 65, 69]);
        // In A minor pentatonic (A C D E G), F is two semitones from G and
        // one from E: it goes to E.
        let pentatonic = Scale::new(PitchClass::A, ScaleKind::MinorPentatonic);
        assert_eq!(pitches(&snap_to_scale(&[note(65, 0, 1)], pentatonic)), [64]);
    }

    #[test]
    fn inversions_put_another_note_in_the_bass() {
        let c_major = [Note::new(60), Note::new(64), Note::new(67)];
        let midi = |notes: Vec<Note>| notes.iter().map(|note| note.midi()).collect::<Vec<_>>();
        assert_eq!(midi(inversion(&c_major, 0)), [60, 64, 67]);
        assert_eq!(
            midi(inversion(&c_major, 1)),
            [64, 67, 72],
            "first inversion: E in the bass"
        );
        assert_eq!(
            midi(inversion(&c_major, 2)),
            [67, 72, 76],
            "second inversion: G in the bass"
        );
        assert_eq!(
            midi(inversion(&c_major, 3)),
            [60, 64, 67],
            "three notes, three inversions"
        );
        assert!(inversion(&[], 1).is_empty());
    }

    #[test]
    fn inverting_chords_up_and_down() {
        let up = invert(&two_chords(), true).unwrap();
        let groups = chords(&up);
        assert_eq!(pitches(&groups[0]), [64, 67, 72]);
        assert_eq!(pitches(&groups[1]), [69, 72, 77]);

        let down = invert(&two_chords(), false).unwrap();
        assert_eq!(pitches(&chords(&down)[0]), [55, 60, 64], "G goes below C");

        // An octave on top: the lowest C goes above the high C.
        let with_octave = [
            note(60, 0, 4),
            note(64, 0, 4),
            note(67, 0, 4),
            note(72, 0, 4),
        ];
        assert_eq!(
            pitches(&invert(&with_octave, true).unwrap()),
            [64, 67, 72, 84]
        );
        // Single notes are left alone.
        assert_eq!(invert(&[note(60, 0, 4)], true), Some(vec![note(60, 0, 4)]));
    }

    #[test]
    fn arpeggios_follow_their_order() {
        let c_major = [note(60, 0, 8), note(64, 0, 8), note(67, 0, 8)];
        let up = arpeggiate(&c_major, ArpeggioOrder::Up, 1);
        assert_eq!(pitches(&up), [60, 64, 67, 60, 64, 67, 60, 64]);
        assert!(
            up.iter()
                .enumerate()
                .all(|(step, note)| note.start == step && note.length == 1)
        );

        let down = arpeggiate(&c_major, ArpeggioOrder::Down, 2);
        assert_eq!(pitches(&down), [67, 64, 60, 67]);
        assert_eq!(down[3].start, 6);

        let up_down = arpeggiate(&c_major, ArpeggioOrder::UpDown, 1);
        assert_eq!(pitches(&up_down), [60, 64, 67, 64, 60, 64, 67, 64]);
    }

    #[test]
    fn an_arpeggio_stops_at_the_next_chord() {
        let mut notes = vec![note(60, 0, 16), note(64, 0, 16)];
        notes.push(note(72, 3, 1)); // a single note on step 3
        let arpeggio = arpeggiate(&notes, ArpeggioOrder::Up, 2);
        // Steps 0 and 2, the second one cut to 1 step; then the single note.
        assert_eq!(
            arpeggio,
            vec![note(60, 0, 2), note(64, 2, 1), note(72, 3, 1)]
        );
    }

    #[test]
    fn arpeggios_keep_each_note_velocity() {
        let soft = NoteEvent {
            velocity: 40,
            ..note(60, 0, 4)
        };
        let loud = NoteEvent {
            velocity: 120,
            ..note(67, 0, 4)
        };
        let arpeggio = arpeggiate(&[soft, loud], ArpeggioOrder::Up, 1);
        let velocities: Vec<u8> = arpeggio.iter().map(|note| note.velocity).collect();
        assert_eq!(velocities, [40, 120, 40, 120]);
    }
}
