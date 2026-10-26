//! How each part moves from one chord to the next.
//!
//! The voice-leading check ([`super::voice_leading`]) splits the tracks
//! into parts, the lines a reader of a score follows: a singer, an
//! instrument, one line of a piano's chords. This module follows those
//! parts through every change, wherever a new note starts in any of
//! them, and says what each one does there:
//!
//! - it **holds** its note, or strikes the same note again;
//! - it moves by **step**, a second (one or two semitones), the
//!   smoothest move, which part writing likes best;
//! - it moves by **skip**, a third;
//! - or it **leaps**, a fourth or more.
//!
//! Two parts move against each other in one of four ways, the *motion*
//! between them: in **parallel** (the same way, keeping the interval
//! between them), **similar** (the same way, by different intervals),
//! **contrary** (opposite ways) or **oblique** (one moves, the other
//! holds). Part writing keeps the outer parts, the top line and the
//! bass, in contrary or oblique motion as much as it can: that keeps
//! them two lines and not one.

use super::voice_leading::Voice;
use super::{Scale, spelled_interval_name};
use crate::{Note, NoteEvent, TICKS_PER_STEP};

/// A rest this long or longer ends a part's line, as the voice-leading
/// check says: a quarter note, in ticks.
const LINE_BREAK: usize = 4 * TICKS_PER_STEP;

/// How big a move is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// The same note.
    Held,
    /// A second: one or two semitones.
    Step,
    /// A third: three or four semitones.
    Skip,
    /// A fourth or more.
    Leap,
}

/// How two parts move against each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    /// Both hold.
    Static,
    /// One moves and the other holds.
    Oblique,
    /// Opposite ways.
    Contrary,
    /// The same way, by different intervals.
    Similar,
    /// The same way, keeping the interval between them.
    Parallel,
}

impl Motion {
    pub fn name(self) -> &'static str {
        match self {
            Motion::Static => "no motion",
            Motion::Oblique => "oblique motion",
            Motion::Contrary => "contrary motion",
            Motion::Similar => "similar motion",
            Motion::Parallel => "parallel motion",
        }
    }
}

/// One part's move at a change: from the note it had to the note it has.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    /// Which part, by its place in the check's list of parts.
    pub voice: usize,
    pub from: NoteEvent,
    pub to: NoteEvent,
}

impl Move {
    /// How far it moves, in semitones: up is positive.
    pub fn semitones(&self) -> i32 {
        i32::from(self.to.pitch.midi()) - i32::from(self.from.pitch.midi())
    }

    pub fn size(&self) -> Size {
        match self.semitones().unsigned_abs() {
            0 => Size::Held,
            1 | 2 => Size::Step,
            3 | 4 => Size::Skip,
            _ => Size::Leap,
        }
    }

    /// The move in a few letters: "E5→F5", "C4 held".
    pub fn short(&self, key: Scale) -> String {
        let (from, to) = (key.note_name(self.from.pitch), key.note_name(self.to.pitch));
        match self.size() {
            Size::Held => format!("{from} held"),
            _ => format!("{from}→{to}"),
        }
    }

    /// The move in words: "from E5 up a minor second to F5, a step".
    pub fn describe(&self, key: Scale) -> String {
        let (from, to) = (key.note_name(self.from.pitch), key.note_name(self.to.pitch));
        if self.size() == Size::Held {
            return match self.from == self.to {
                true => format!("holds {from}"),
                false => format!("strikes {from} again"),
            };
        }
        let way = if self.semitones() > 0 { "up" } else { "down" };
        let kind = match self.size() {
            Size::Step => "a step",
            Size::Skip => "a skip",
            _ => "a leap",
        };
        format!(
            "{way} {} from {from} to {to}, {kind}",
            interval(key, self.from.pitch, self.to.pitch)
        )
    }
}

/// The interval between two notes, as `key` spells them, with an
/// article: "a minor second", "an augmented second", "an octave", "a
/// perfect fifth and an octave".
fn interval(key: Scale, a: Note, b: Note) -> String {
    let (low, high) = if a.midi() <= b.midi() { (a, b) } else { (b, a) };
    let span = high.midi() - low.midi();
    let name = match span {
        12 => "octave".to_string(),
        24 => "two octaves".to_string(),
        _ => spelled_interval_name(key.spell(low.pitch_class()), key.spell(high.pitch_class())),
    };
    let article = match name.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    };
    let more = match span {
        13..=23 => " and an octave",
        25.. => " and two octaves",
        _ => "",
    };
    format!("{article} {name}{more}")
}

/// How two moves, `upper` above `lower`, go against each other in
/// `key`. Parallel motion keeps the interval's number, as `key` spells
/// the notes: a major sixth to a minor sixth is parallel sixths, and a
/// fifth to a twelfth parallel fifths.
pub fn motion(upper: &Move, lower: &Move, key: Scale) -> Motion {
    let (a, b) = (upper.semitones(), lower.semitones());
    match (a.signum(), b.signum()) {
        (0, 0) => Motion::Static,
        (0, _) | (_, 0) => Motion::Oblique,
        (x, y) if x != y => Motion::Contrary,
        _ => {
            let number = |upper: Note, lower: Note| {
                let (upper, lower) = (
                    key.spell(upper.pitch_class()),
                    key.spell(lower.pitch_class()),
                );
                lower.letter.steps_up_to(upper.letter)
            };
            if number(upper.from.pitch, lower.from.pitch) == number(upper.to.pitch, lower.to.pitch)
            {
                Motion::Parallel
            } else {
                Motion::Similar
            }
        }
    }
}

/// What every part does at one change, where a new note starts in at
/// least one part and at least one part moves.
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    /// The step the new notes start on.
    pub step: usize,
    /// The moves of the parts that sound on both sides of the change,
    /// highest part first.
    pub moves: Vec<Move>,
}

impl Change {
    /// The motion between the top part and the bass in `key`, when both
    /// move or hold here and they are two different parts.
    pub fn outer(&self, key: Scale) -> Option<Motion> {
        let (top, bass) = (self.moves.first()?, self.moves.last()?);
        (top.voice != bass.voice).then(|| motion(top, bass, key))
    }

    /// The moves on one line, highest first, each named by `name` (or by
    /// nothing, where it gives an empty name): "Soprano E5→F5 · Bass
    /// C3→F2; outer parts in contrary motion".
    pub fn summary(&self, key: Scale, name: impl Fn(usize) -> String) -> String {
        let moves: Vec<String> = self
            .moves
            .iter()
            .map(|part| match name(part.voice) {
                name if name.is_empty() => part.short(key),
                name => format!("{name} {}", part.short(key)),
            })
            .collect();
        let mut text = moves.join(" · ");
        if let Some(outer) = self.outer(key) {
            text.push_str(&format!("; outer parts in {}", outer.name()));
        }
        text
    }

    /// The moves in sentences, highest first, with the motion between
    /// each pair of parts next to each other and between the outer ones.
    pub fn describe(&self, key: Scale, name: impl Fn(usize) -> String) -> String {
        let mut lines: Vec<String> = self
            .moves
            .iter()
            .map(|part| format!("{}: {}.", name(part.voice), part.describe(key)))
            .collect();
        for pair in self.moves.windows(2) {
            lines.push(format!(
                "{} and {}: {}.",
                name(pair[0].voice),
                name(pair[1].voice),
                motion(&pair[0], &pair[1], key).name()
            ));
        }
        if self.moves.len() > 2
            && let Some(outer) = self.outer(key)
        {
            lines.push(format!("Outer parts: {}.", outer.name()));
        }
        lines.join("\n")
    }
}

/// The note `voice` is on just before tick `tick`: the last one that
/// started before it, unless the part has rested a beat or more since.
fn before(voice: &Voice, tick: usize) -> Option<NoteEvent> {
    let index = voice
        .notes
        .partition_point(|note| note.start_tick() < tick)
        .checked_sub(1)?;
    let note = voice.notes[index];
    (tick < note.end_tick() + LINE_BREAK).then_some(note)
}

/// The note `voice` is on at tick `tick`: one starting there, or one
/// still sounding.
fn at(voice: &Voice, tick: usize) -> Option<NoteEvent> {
    let index = voice
        .notes
        .partition_point(|note| note.start_tick() <= tick)
        .checked_sub(1)?;
    let note = voice.notes[index];
    (note.start_tick() == tick || note.end_tick() > tick).then_some(note)
}

/// Every change in `voices` (the parts of a voice-leading check, highest
/// first): each step where a new note starts in a part that had one
/// before, with the move of each part sounding on both sides of it.
/// Changes where every part holds, as when a chord is struck again, are
/// left out.
pub fn changes(voices: &[Voice]) -> Vec<Change> {
    let mut ticks: Vec<usize> = voices
        .iter()
        .flat_map(|voice| voice.notes.iter().map(NoteEvent::start_tick))
        .collect();
    ticks.sort_unstable();
    ticks.dedup();
    let mut found = Vec::new();
    for tick in ticks {
        let moves: Vec<Move> = voices
            .iter()
            .enumerate()
            .filter_map(|(index, voice)| {
                let to = at(voice, tick)?;
                // A part that starts a note here moves from the note it
                // had before; one still holding a note holds it.
                let from = match to.start_tick() == tick {
                    true => before(voice, tick)?,
                    false => to,
                };
                Some(Move {
                    voice: index,
                    from,
                    to,
                })
            })
            .collect();
        if moves.iter().any(|part| part.size() != Size::Held) {
            found.push(Change {
                step: tick / TICKS_PER_STEP,
                moves,
            });
        }
    }
    found
}

/// The change at `step` or the last one before it, if there is one
/// within `reach` steps.
pub fn change_at(changes: &[Change], step: usize, reach: usize) -> Option<&Change> {
    let index = changes
        .partition_point(|change| change.step <= step)
        .checked_sub(1)?;
    let change = &changes[index];
    (step - change.step < reach).then_some(change)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PitchClass;
    use crate::harmony::ScaleKind;
    use crate::harmony::voice_leading::voices_of;

    fn note(midi: u8, start: usize, length: usize) -> NoteEvent {
        NoteEvent::new(Note::new(midi), start, length)
    }

    fn c_major() -> Scale {
        Scale::new(PitchClass::C, ScaleKind::Major)
    }

    /// C major (C3 G3 E4 C5) to F major (F2 A3 F4 C5) to G7 (G2 B3 F4 D5),
    /// a beat each, on one track in four parts.
    fn chorale() -> Vec<Voice> {
        let mut notes = Vec::new();
        for (beat, chord) in [[48, 55, 64, 72], [41, 57, 65, 72], [43, 59, 65, 74]]
            .iter()
            .enumerate()
        {
            notes.extend(chord.iter().map(|&midi| note(midi, beat * 4, 4)));
        }
        voices_of(0, &notes)
    }

    #[test]
    fn every_part_moves_at_each_chord_change() {
        let voices = chorale();
        assert_eq!(voices.len(), 4);
        let found = changes(&voices);
        assert_eq!(
            found.iter().map(|change| change.step).collect::<Vec<_>>(),
            [4, 8]
        );
        let name = |voice: usize| ["S", "A", "T", "B"][voice].to_string();
        let key = c_major();
        assert_eq!(
            found[0].summary(key, name),
            "S C5 held · A E4→F4 · T G3→A3 · B C3→F2; outer parts in oblique motion"
        );
        let sizes: Vec<Size> = found[0].moves.iter().map(Move::size).collect();
        assert_eq!(sizes, [Size::Held, Size::Step, Size::Step, Size::Leap]);
        let about = found[0].describe(key, name);
        assert!(about.contains("S: strikes C5 again."), "{about}");
        assert!(
            about.contains("A: up a minor second from E4 to F4, a step."),
            "{about}"
        );
        assert!(
            about.contains("B: down a perfect fifth from C3 to F2, a leap."),
            "{about}"
        );
        assert!(about.contains("A and T: parallel motion."), "{about}");
        assert!(about.contains("T and B: contrary motion."), "{about}");
        // F to G7: the soprano and the bass both rise a step, a fifth
        // (and two octaves) apart: parallel fifths.
        assert_eq!(found[1].outer(key), Some(Motion::Parallel));
    }

    #[test]
    fn a_held_note_moves_obliquely_against_a_moving_one() {
        // A melody over a whole note.
        let voices = [
            Voice {
                channel: 0,
                rank: 0,
                of: 1,
                notes: vec![note(72, 0, 4), note(74, 4, 4), note(76, 8, 4)],
            },
            Voice {
                channel: 1,
                rank: 0,
                of: 1,
                notes: vec![note(48, 0, 12)],
            },
        ];
        let found = changes(&voices);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].outer(c_major()), Some(Motion::Oblique));
        assert_eq!(found[0].moves[1].size(), Size::Held);
        let about = found[0].describe(c_major(), |voice| format!("P{voice}"));
        assert!(about.contains("P1: holds C3."), "{about}");
        assert!(
            about.contains("P0: up a major second from C5 to D5, a step."),
            "{about}"
        );
        assert_eq!(change_at(&found, 6, 8).map(|change| change.step), Some(4));
        assert_eq!(change_at(&found, 2, 8), None);
    }

    #[test]
    fn parts_a_fifth_apart_moving_the_same_way_move_in_parallel() {
        let up = |from: u8, to: u8| Move {
            voice: 0,
            from: note(from, 0, 4),
            to: note(to, 4, 4),
        };
        let key = c_major();
        assert_eq!(motion(&up(67, 69), &up(60, 62), key), Motion::Parallel);
        // A fifth to a twelfth is still parallel fifths.
        assert_eq!(motion(&up(67, 81), &up(60, 62), key), Motion::Parallel);
        assert_eq!(motion(&up(67, 72), &up(60, 62), key), Motion::Similar);
        assert_eq!(motion(&up(67, 65), &up(60, 62), key), Motion::Contrary);
        // A major sixth to a minor sixth: parallel sixths.
        assert_eq!(motion(&up(64, 65), &up(55, 57), key), Motion::Parallel);
        assert_eq!(interval(key, Note::new(60), Note::new(72)), "an octave");
        assert_eq!(
            interval(key, Note::new(60), Note::new(79)),
            "a perfect fifth and an octave"
        );
        // G# in A minor harmonic is an augmented second above F.
        let a = Scale::new(PitchClass::A, ScaleKind::HarmonicMinor);
        assert_eq!(
            interval(a, Note::new(65), Note::new(68)),
            "an augmented second"
        );
    }
}
