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

use crate::harmony::{Degree, PitchSet, RowLabel};
use crate::pitch::PitchClass;
use crate::{Note, NoteEvent, Scale, ScaleKind};

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

/// Moves every note onto the grid: its start and its end to the nearest
/// step, as a quantize does. A note that would then start at
/// the end of the pattern starts on its last step instead.
///
/// Notes recorded by hand or imported from a MIDI file are often a
/// little off the grid, which is how people play; quantizing makes them
/// as exact as a drum machine.
pub fn quantize(notes: &[NoteEvent], pattern_length: usize) -> Vec<NoteEvent> {
    let last = pattern_length.saturating_sub(1);
    notes
        .iter()
        .map(|note| {
            let quantized = note.quantized();
            let start = quantized.start.min(last);
            NoteEvent {
                start,
                length: quantized.end() - start,
                ..quantized
            }
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

/// Where `note` goes when the song changes key from `from` to `to`, the
/// way a musician would play it in the new key.
///
/// A new root moves every note by the same number of semitones, up or
/// down, whichever is nearer (C to G goes down a fourth, not up a fifth).
/// A new mode then moves each note of the old scale to the same degree
/// of the new one: from C major to C minor, E, A and B become Eb, Ab and
/// Bb, and C, D, F and G stay. Notes outside the old scale (a borrowed
/// chord, or a part in a key of its own) only take the new root, so
/// they keep their distance from it. Between scales of different sizes,
/// such as major and pentatonic, there are no matching degrees, so a
/// note of the old scale that is not in the new one moves to the nearest
/// one that is. A note that would leave the MIDI range stays put.
///
/// Between kinds that have nothing in common, such as a key and a
/// twelve-tone row, the note stays where it is (see
/// [`ScaleKind::notes_follow`](crate::ScaleKind::notes_follow)). From
/// one row to another, a note takes the same place in the new row that
/// it had in the old, both counted from the key note, as
/// [`change_row`] moves it when the form the note is part of is not
/// known.
///
/// A pitch-class set that is a key's own scale (C major switched to
/// "pitch-class set" is the set 7-35 on C) moves into a key as that key
/// would: from it to C minor, E, A and B go down as from C major.
pub fn change_key(note: Note, from: Scale, to: Scale) -> Note {
    let from = from_key(from, to);
    if !from.kind.notes_follow(to.kind) {
        return note;
    }
    if from.tone_row().is_some() && to.tone_row().is_some() {
        return change_row(note, None, from, to);
    }
    let up = from.root.semitones_up_to(to.root) as i32;
    let shift = if up > 6 { up - 12 } else { up };
    let Some(moved) = note.transpose(shift) else {
        return note;
    };
    // The chromatic scale holds every note, so it has no degrees to
    // match with a key's: into it or out of it, notes only take the new
    // root. (Out of it, every note would otherwise be "of the old scale"
    // and be pulled into the new key, losing every sharp and flat.)
    if from.kind == ScaleKind::Chromatic || to.kind == ScaleKind::Chromatic {
        return moved;
    }
    // The old scale moved to the new root, so `moved` can be compared
    // with it degree by degree.
    let old = Scale::new(to.root, from.kind);
    let Some(degree) = old.degree_of(moved.pitch_class()) else {
        return moved;
    };
    let (old_steps, new_steps) = (from.kind.intervals(), to.kind.intervals());
    if old_steps.len() == new_steps.len() {
        let offset = new_steps[degree] as i32 - old_steps[degree] as i32;
        moved.transpose(offset).unwrap_or(moved)
    } else {
        nearest_in_key(moved, to)
    }
}

/// Where `note` goes when the song changes key from `from` to `to`
/// while it plays `form`, the harmony it is part of: a form of the old
/// key, when the key is a set or a micromode.
///
/// Changing the set the key is keeps every form's degree, as changing
/// a key's mode keeps every chord's (see [`Degree`]): from Menor 3 on C
/// to Menor 5 on C, T2 (D D# F#) becomes T2 of Menor 5 (D D# G#). Each
/// note takes the same place in the new form that it had in the old,
/// counted from the key's note the way the form was made: in Menor 3 on
/// C, C C# E, the third note is E, and in Menor 5 on C, C C# F#, it is
/// F#, so every form's third note goes up two semitones (down, in a form
/// upside down). The note moves the shortest way to its new pitch class.
/// A new set with fewer notes has no place for the last ones, which go
/// to the nearest note of the new form.
///
/// Without a form (no harmony is known, or the note is not part of it,
/// or the key is not a set), the note moves as [`change_key`] moves it.
pub fn change_key_in(note: Note, form: Option<PitchSet>, from: Scale, to: Scale) -> Note {
    let old = from_key(from, to);
    let (ScaleKind::Set(_), ScaleKind::Set(_)) = (old.kind, to.kind) else {
        return change_key(note, from, to);
    };
    let Some((degree, place)) = form.and_then(|form| {
        let degree = Degree::of(old, form)?;
        Some((degree, place_in(degree, old, note.pitch_class())?))
    }) else {
        return change_key(note, from, to);
    };
    let target = match note_at(degree, to, place) {
        Some(target) => target,
        None => {
            let Some(new_form) = degree.form(to) else {
                return change_key(note, from, to);
            };
            let moved = note
                .pitch_class()
                .transpose(nearest_shift(old.root, to.root));
            nearest_in_set(moved, new_form)
        }
    };
    let shift = nearest_shift(note.pitch_class(), target);
    note.transpose(shift)
        .or_else(|| note.transpose(shift - 12 * shift.signum()))
        .unwrap_or(note)
}

/// The shortest move from `from` to `to`, in semitones: up to six
/// down, or up to six up (a tritone goes up).
fn nearest_shift(from: PitchClass, to: PitchClass) -> i32 {
    let up = from.semitones_up_to(to) as i32;
    if up > 6 { up - 12 } else { up }
}

/// Which note of the form at `degree` in `key` `pitch_class` is,
/// counting the notes as the key's own form counts them, up from its
/// note: the form's first note is the key note moved (or mirrored and
/// moved), and so on. `None` if it is not in the form.
fn place_in(degree: Degree, key: Scale, pitch_class: PitchClass) -> Option<usize> {
    (0..key.kind.intervals().len()).find(|&place| note_at(degree, key, place) == Some(pitch_class))
}

/// The note in place `place` of the form at `degree` in `key` (see
/// [`place_in`]), or `None` if the key's form has fewer notes.
fn note_at(degree: Degree, key: Scale, place: usize) -> Option<PitchClass> {
    let interval = i32::from(*key.kind.intervals().get(place)?);
    let from_key_note = if degree.inverted { -interval } else { interval };
    Some(key.root.transpose(from_key_note + i32::from(degree.up)))
}

/// The note of `set` nearest to `pitch_class`, the lower one when two
/// are as near; `pitch_class` itself if the set is empty.
fn nearest_in_set(pitch_class: PitchClass, set: PitchSet) -> PitchClass {
    (0..=6)
        .flat_map(|distance| [-distance, distance])
        .map(|semitones| pitch_class.transpose(semitones))
        .find(|&candidate| set.contains(candidate))
        .unwrap_or(pitch_class)
}

/// The key a change to `to` starts from: `from`, or, going from a
/// pitch-class set into a key, the key the set's notes are (see
/// [`Scale::as_key`]). Between two sets nothing changes, so a set picked
/// in the set browser still moves the song into the next set picked.
pub fn from_key(from: Scale, to: Scale) -> Scale {
    if to.kind.is_tonal() {
        from.as_key()
    } else {
        from
    }
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

/// The notes backwards in time, as in a crab canon: the last note comes
/// first and the first last, each as long as it was, with the rests
/// between them backwards too. They stay in the stretch they took, from
/// the first note's start to the last one's end. A note that started a
/// little late (nudged) ends that much early instead.
///
/// A form of a twelve-tone row played backwards is its retrograde, in
/// its rhythm backwards: P0 becomes R0.
pub fn backwards(notes: &[NoteEvent]) -> Vec<NoteEvent> {
    let (Some(first), Some(last)) = (
        notes.iter().map(|note| note.start).min(),
        notes.iter().map(NoteEvent::end).max(),
    ) else {
        return Vec::new();
    };
    notes
        .iter()
        .map(|note| NoteEvent {
            start: first + last - note.end(),
            nudge: note.trim,
            trim: note.nudge,
            ..*note
        })
        .collect()
}

/// The pitches in the opposite order, in the same rhythm: the first
/// note's place takes the last note's pitch, the second the one before
/// the last, and so on, while every place keeps its length and its
/// loudness. A chord moves as a chord, into the place of the chord (or
/// note) at the other end.
///
/// This is how a twelve-tone composer turns a line into its retrograde
/// and keeps its rhythm: P0 in some rhythm becomes R0 in that rhythm.
pub fn pitches_backwards(notes: &[NoteEvent]) -> Vec<NoteEvent> {
    let groups = chords(notes);
    groups
        .iter()
        .zip(groups.iter().rev())
        .flat_map(|(place, pitches)| {
            pitches.iter().enumerate().map(move |(i, note)| NoteEvent {
                pitch: note.pitch,
                ..place[i.min(place.len() - 1)]
            })
        })
        .collect()
}

/// The note the notes are turned upside down around: the first one, or
/// the lowest of the first chord.
fn first_note(notes: &[NoteEvent]) -> Option<Note> {
    chords(notes).first().map(|chord| chord[0].pitch)
}

/// The notes upside down: every interval from the first note turned the
/// other way, so a line that went up a major third from it goes down a
/// major third, and a major chord becomes a minor one. The first note
/// (the lowest, when the notes begin with a chord) stays where it is.
///
/// A form of a twelve-tone row upside down is its inversion on the same
/// first note: P0 becomes I0. `None` if a note would leave the MIDI
/// range.
pub fn upside_down(notes: &[NoteEvent]) -> Option<Vec<NoteEvent>> {
    let Some(axis) = first_note(notes) else {
        return Some(Vec::new());
    };
    notes
        .iter()
        .map(|note| {
            let down = 2 * (i32::from(axis.midi()) - i32::from(note.pitch.midi()));
            Some(NoteEvent {
                pitch: note.pitch.transpose(down)?,
                ..*note
            })
        })
        .collect()
}

/// The notes upside down in the key, as a fugue can answer its subject
/// upside down and still in the key: every note as many notes of the
/// key below the first note as it was above it. In C major, around C,
/// E (two notes of the key up) becomes A (two down) and G becomes F,
/// so the intervals keep their number but not always their size: the
/// major third C-E comes back as the minor third C-A below. A note
/// outside the key is a note of the key raised, and comes back lowered:
/// D# becomes Bb. The first note stays where it is; when it is outside
/// the key, the notes turn around the note of the key nearest it.
///
/// In a key where every note is in the key, as a twelve-tone row, this
/// is the same as [`upside_down`]. `None` if a note would leave the MIDI
/// range.
pub fn upside_down_in_key(notes: &[NoteEvent], scale: Scale) -> Option<Vec<NoteEvent>> {
    let Some(first) = first_note(notes) else {
        return Some(Vec::new());
    };
    let axis = nearest_in_key(first, scale);
    notes
        .iter()
        .map(|note| {
            // The note of the key at or under it, and how much higher it is.
            let base = if scale.contains(note.pitch.pitch_class()) {
                note.pitch
            } else {
                step_in_key(note.pitch, scale, false)?
            };
            let raised = i32::from(note.pitch.midi()) - i32::from(base.midi());
            let mut steps = 0i32;
            let mut walk = axis;
            while walk != base {
                walk = step_in_key(walk, scale, base > walk)?;
                steps += if base > axis { 1 } else { -1 };
            }
            let mut mirrored = axis;
            for _ in 0..steps.unsigned_abs() {
                mirrored = step_in_key(mirrored, scale, steps < 0)?;
            }
            Some(NoteEvent {
                pitch: mirrored.transpose(-raised)?,
                ..*note
            })
        })
        .collect()
}

/// Where `note` goes when the key changes from one twelve-tone row,
/// `from`, to another, `to`, while it is part of the form `label` of the
/// old row: to the same place in the same form of the new row. The
/// fifth note of RI7 becomes the fifth note of RI7, so the plan of forms
/// and the place of every note in its form stay, and only the row
/// changes. The forms count from the key note, so a new key note takes
/// them along: from a row on C to one on D, RI7 becomes RI9. The note
/// moves the shortest way to its new pitch class.
///
/// Without a form, the note is taken as part of the key's own form, the
/// row from the key note. Between keys that are not both rows, the note
/// moves as [`change_key`] moves it.
pub fn change_row(note: Note, label: Option<RowLabel>, from: Scale, to: Scale) -> Note {
    let (Some(old), Some(new)) = (from.tone_row(), to.tone_row()) else {
        return change_key(note, from, to);
    };
    let label = label.unwrap_or(old.label());
    let up = from.root.semitones_up_to(to.root);
    let place = old.form(label).position(note.pitch_class());
    let moved = RowLabel::new(label.form, label.number + up);
    let target = new.form(moved).pitch_classes()[place];
    let shift = nearest_shift(note.pitch_class(), target);
    note.transpose(shift)
        .or_else(|| note.transpose(shift - 12 * shift.signum()))
        .unwrap_or(note)
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
    use crate::{PitchClass, PitchSet};

    #[test]
    fn quantizing_moves_notes_to_the_nearest_steps() {
        let c4 = Note::C4;
        // Eighth-note triplets: ticks 0, 32 and 64, each 32 ticks long.
        let triplets: Vec<NoteEvent> = (0..3).map(|n| NoteEvent::exact(c4, n * 32, 32)).collect();
        let quantized = quantize(&triplets, 16);
        let places: Vec<(usize, usize)> = quantized.iter().map(|n| (n.start, n.length)).collect();
        // 32 ticks is 1 1/3 steps, so they land on steps 0, 1 and 3.
        assert_eq!(places, [(0, 1), (1, 2), (3, 1)]);
        assert!(quantized.iter().all(NoteEvent::is_on_grid));
        // A note very late in the last step stays in the pattern.
        let late = NoteEvent::exact(c4, 15 * 24 + 20, 4);
        assert_eq!(quantize(&[late], 16)[0].start, 15);
    }

    fn note(midi: u8, start: usize, length: usize) -> NoteEvent {
        NoteEvent::new(Note::new(midi), start, length)
    }

    fn pitches(notes: &[NoteEvent]) -> Vec<u8> {
        notes.iter().map(|note| note.pitch.midi()).collect()
    }

    const C_MAJOR: Scale = Scale::new(PitchClass::C, ScaleKind::Major);

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
    fn a_new_key_moves_notes_the_nearer_way() {
        use PitchClass::*;
        let key = |root, kind| Scale::new(root, kind);
        let moved = |midi, from, to| change_key(Note::new(midi), from, to).midi();
        // C to D goes up a tone; C to G goes down a fourth.
        assert_eq!(moved(60, C_MAJOR, key(D, ScaleKind::Major)), 62);
        assert_eq!(moved(64, C_MAJOR, key(G, ScaleKind::Major)), 59);
        // C major to C minor: E, A and B go down; C, D, F and G stay.
        let minor = key(C, ScaleKind::NaturalMinor);
        let scale: Vec<u8> = [60, 62, 64, 65, 67, 69, 71]
            .iter()
            .map(|&m| moved(m, C_MAJOR, minor))
            .collect();
        assert_eq!(scale, [60, 62, 63, 65, 67, 68, 70]);
        // And back again.
        assert_eq!(moved(63, minor, C_MAJOR), 64);
        // A note outside the scale only takes the new root.
        assert_eq!(moved(66, C_MAJOR, minor), 66);
        assert_eq!(moved(66, C_MAJOR, key(D, ScaleKind::NaturalMinor)), 68);
        // Key and mode at once: E in C major is the third, so it is F in
        // D minor.
        assert_eq!(moved(64, C_MAJOR, key(D, ScaleKind::NaturalMinor)), 65);
        // Major to pentatonic: F and B are not in it, so they move to the
        // nearest note that is.
        let pentatonic = key(C, ScaleKind::MajorPentatonic);
        assert_eq!(moved(65, C_MAJOR, pentatonic), 64);
        assert_eq!(moved(71, C_MAJOR, pentatonic), 72);
        // A note that would leave the MIDI range stays put.
        assert_eq!(moved(127, C_MAJOR, key(D, ScaleKind::Major)), 127);
        // To the chromatic scale and back, a D7 (D F# A C) stays a D7:
        // it has no degrees, so nothing is pulled into the key.
        let chromatic = key(C, ScaleKind::Chromatic);
        for midi in [62, 66, 69, 72] {
            let there = moved(midi, C_MAJOR, chromatic);
            assert_eq!(moved(there, chromatic, C_MAJOR), midi);
        }
        assert_eq!(moved(66, chromatic, key(D, ScaleKind::Major)), 68);
    }

    #[test]
    fn a_key_shown_as_a_set_moves_on_as_that_key() {
        use PitchClass::*;
        let moved = |midi, from, to| change_key(Note::new(midi), from, to).midi();
        let as_set = Scale::set(C, C_MAJOR.pitch_set());
        let minor = Scale::new(C, ScaleKind::NaturalMinor);
        // C major to the set 7-35 on C: the same notes, nothing moves.
        for m in [60, 62, 64, 65, 67, 69, 71] {
            assert_eq!(moved(m, C_MAJOR, as_set), m);
        }
        // From that set to C minor: as from C major, E A B go down.
        let scale: Vec<u8> = [60, 62, 64, 65, 67, 69, 71]
            .iter()
            .map(|&m| moved(m, as_set, minor))
            .collect();
        assert_eq!(scale, [60, 62, 63, 65, 67, 68, 70]);
        // And to G major, as from C major: down a fourth.
        assert_eq!(moved(64, as_set, Scale::new(G, ScaleKind::Major)), 59);
        // A set that is no key still leaves the notes where they are.
        let diminished = Scale::set(C, PitchSet::from_semitones(&[0, 3, 6, 9]));
        assert_eq!(moved(63, diminished, minor), 63);
    }

    #[test]
    fn a_new_set_keeps_the_degree_of_each_form() {
        use PitchClass::*;
        let set = |root, semitones: &[u8]| Scale::set(root, PitchSet::from_semitones(semitones));
        // Menor 3 on C is C C# E; Menor 5 on C is C C# F#.
        let (menor_3, menor_5) = (set(C, &[0, 1, 4]), set(C, &[0, 1, 6]));
        let moved = |midis: &[u8], from: Scale, to: Scale| -> Vec<u8> {
            let form: PitchSet = midis.iter().map(|&m| Note::new(m).pitch_class()).collect();
            midis
                .iter()
                .map(|&m| change_key_in(Note::new(m), Some(form), from, to).midi())
                .collect()
        };
        // Home, T2 (D D# F#) and T0I (C B Ab, upside down around C):
        // each third note moves, up or down as the form goes.
        assert_eq!(moved(&[60, 61, 64], menor_3, menor_5), [60, 61, 66]);
        assert_eq!(moved(&[62, 63, 66], menor_3, menor_5), [62, 63, 68]);
        assert_eq!(moved(&[60, 59, 56], menor_3, menor_5), [60, 59, 54]);
        // And back: the same forms of Menor 3 again.
        assert_eq!(moved(&[62, 63, 68], menor_5, menor_3), [62, 63, 66]);
        // Another key note as well: Menor 5 on D is D D# G#, and its T2
        // is E F A#.
        assert_eq!(
            moved(&[62, 63, 66], menor_3, set(D, &[2, 3, 8])),
            [64, 65, 70]
        );
        // To a set of four notes, C E G Bb (C7 as a set): each note takes
        // the same place, so T2 of Menor 3 becomes T2 of the set, D F# A.
        assert_eq!(
            moved(&[62, 63, 66], menor_3, set(C, &[0, 4, 7, 10])),
            [62, 66, 69]
        );
        // From it to Menor 3, the fourth note has no place: Bb goes to the
        // nearest note of the new form, C.
        assert_eq!(
            moved(&[60, 64, 67, 70], set(C, &[0, 4, 7, 10]), menor_3),
            [60, 61, 64, 72]
        );
        // Without a form, or with a note outside it, as `change_key`.
        assert_eq!(
            change_key_in(Note::new(62), None, menor_3, menor_5).midi(),
            62
        );
        let home = menor_3.pitch_set();
        assert_eq!(
            change_key_in(Note::new(66), Some(home), menor_3, menor_5).midi(),
            change_key(Note::new(66), menor_3, menor_5).midi()
        );
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
    fn backwards_turns_the_notes_round_in_time() {
        let tune = [note(60, 0, 2), note(64, 2, 1), note(67, 4, 4)];
        let mut crab = backwards(&tune);
        crab.sort_by_key(|note| note.start);
        // G first and for as long, the rest after it, then E, then C.
        assert_eq!(crab, [note(67, 0, 4), note(64, 5, 1), note(60, 6, 2)]);
        // A note played a little late ends that much early instead.
        let late = NoteEvent {
            nudge: 3,
            trim: 5,
            ..note(60, 0, 2)
        };
        let turned = backwards(&[late, note(62, 2, 2)]);
        assert_eq!(
            (turned[0].start, turned[0].nudge, turned[0].trim),
            (2, 5, 3)
        );
        assert!(backwards(&[]).is_empty());
    }

    #[test]
    fn pitches_backwards_keep_the_rhythm() {
        let tune = [
            NoteEvent {
                velocity: 30,
                ..note(60, 0, 2)
            },
            note(64, 2, 1),
            note(67, 4, 4),
        ];
        let turned = pitches_backwards(&tune);
        assert_eq!(pitches(&turned), [67, 64, 60]);
        let rhythm: Vec<(usize, usize, u8)> = turned
            .iter()
            .map(|note| (note.start, note.length, note.velocity))
            .collect();
        assert_eq!(rhythm, [(0, 2, 30), (2, 1, 100), (4, 4, 100)]);
        // A chord and a note change places; the chord takes the note's.
        let chord_then_note = [
            note(60, 0, 4),
            note(64, 0, 4),
            note(67, 0, 4),
            note(62, 4, 2),
        ];
        let turned = pitches_backwards(&chord_then_note);
        assert_eq!(turned[0], note(62, 0, 4));
        assert_eq!(
            &turned[1..],
            [note(60, 4, 2), note(64, 4, 2), note(67, 4, 2)]
        );
    }

    #[test]
    fn upside_down_turns_every_interval_the_other_way() {
        let tune = [note(60, 0, 1), note(64, 1, 1), note(67, 2, 1)];
        assert_eq!(pitches(&upside_down(&tune).unwrap()), [60, 56, 53]);
        // C major upside down is F minor: C, Ab and F.
        let chord = [note(60, 0, 4), note(64, 0, 4), note(67, 0, 4)];
        assert_eq!(pitches(&upside_down(&chord).unwrap()), [60, 56, 53]);
        // P0 of a row upside down is I0.
        let lineup = crate::harmony::RowPreset::named("Lineup row").unwrap().row;
        let p0: Vec<NoteEvent> = lineup
            .semitones()
            .iter()
            .enumerate()
            .map(|(step, &n)| note(60 + n, step, 1))
            .collect();
        let turned: Vec<u8> = pitches(&upside_down(&p0).unwrap())
            .iter()
            .map(|&midi| midi % 12)
            .collect();
        let i0 = lineup.form(RowLabel::new(crate::harmony::RowForm::Inversion, 0));
        assert_eq!(turned, i0.semitones());
        assert_eq!(upside_down(&[note(10, 0, 1), note(126, 1, 1)]), None);
    }

    #[test]
    fn upside_down_in_the_key_counts_the_notes_of_the_key() {
        let tune: Vec<NoteEvent> = [60, 64, 67, 63, 59, 72]
            .iter()
            .enumerate()
            .map(|(step, &midi)| note(midi, step, 1))
            .collect();
        let turned = upside_down_in_key(&tune, C_MAJOR).unwrap();
        // E becomes A, G becomes F, D# (D raised) becomes Bb (B lowered),
        // B becomes D and the C an octave up the C an octave down.
        assert_eq!(pitches(&turned), [60, 57, 53, 58, 62, 48]);
        // In a row, every note is in the key: the same as upside down.
        let row = Scale::row(crate::harmony::ToneRow::CHROMATIC);
        assert_eq!(upside_down_in_key(&tune, row), upside_down(&tune));
        assert_eq!(pitches(&upside_down_in_key(&[], C_MAJOR).unwrap()), []);
    }

    #[test]
    fn a_new_row_keeps_each_note_in_its_form_and_place() {
        use crate::harmony::{RowForm, RowPreset};
        let row = |name: &str| RowPreset::named(name).unwrap().row;
        let lineup = Scale::row(row("Lineup row"));
        let berg_on_c = Scale::row(row("Berg's Lyric Suite").starting_on(PitchClass::C));
        let berg_on_d = Scale::row(row("Berg's Lyric Suite").starting_on(PitchClass::D));
        let p0 = RowLabel::new(RowForm::Prime, 0);
        let e4 = Note::new(64);
        // E is the sixth note of the Lineup row's P0, and A the sixth of
        // Berg's on C: E goes up a fourth to A.
        assert_eq!(change_row(e4, Some(p0), lineup, berg_on_c).midi(), 69);
        // On D, P0 is P2, whose sixth note is B: E goes down to B.
        assert_eq!(change_row(e4, Some(p0), lineup, berg_on_d).midi(), 59);
        // In RI5, the note keeps its place in RI5.
        let ri5 = RowLabel::new(RowForm::RetrogradeInversion, 5);
        let old = row("Lineup row").form(ri5);
        let new = row("Berg's Lyric Suite")
            .starting_on(PitchClass::C)
            .form(ri5);
        for (from, to) in old.pitch_classes().into_iter().zip(new.pitch_classes()) {
            let moved = change_row(
                Note::new(60 + from.semitone()),
                Some(ri5),
                lineup,
                berg_on_c,
            );
            assert_eq!(moved.pitch_class(), to);
        }
        // Without a form, a note takes its place in the key's own form.
        assert_eq!(change_row(e4, None, lineup, berg_on_c).midi(), 69);
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
