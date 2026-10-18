//! The soloist: an improvised line over the chord sheet. Experimental,
//! and off until it is switched on.
//!
//! The solo follows the rule jazz teachers give for a first improvised
//! line: **chord tones on the beat, other notes between the beats**. On
//! each beat (each quarter note, or each dotted quarter in 6/8) the
//! soloist plays a note of the harmony sounding there, the one nearest
//! to where the line is going. Between two beats it fills in from the
//! chord's scale (see [`chord_scale`]): a **passing** note a step between
//! them, a **neighbour** a step away and back, or a **chromatic
//! approach** a half step from the next one. It plays in phrases of two
//! bars and breathes between them.
//!
//! [`note_roles`] reads any line the same way, the soloist's or one you
//! edited, and says what each note is, for the piano roll to show.

use crate::general_midi as gm;
use crate::harmony::{Chord, PitchSet, Scale, chord_scale, chord_scale_name};
use crate::library::Genre;
use crate::library::comping::{BassPart, Played};
use crate::meter::Bar;
use crate::pattern::NoteEvent;
use crate::pitch::{Note, PitchClass};

/// What a soloist plays over: one harmony of the sheet, from `start` to
/// `end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Over {
    pub start: usize,
    pub end: usize,
    /// The chord, or `None` for a pitch-class set, a micromode or one
    /// trichord of a row.
    pub chord: Option<Chord>,
    /// The notes of the harmony.
    pub tones: PitchSet,
    /// The notes the soloist plays over it: for a chord, its scale in the
    /// key ([`chord_scale`]); for a set, its own notes.
    pub scale: PitchSet,
}

impl Over {
    /// What to play over `played`, a harmony the band plays, in `key`.
    /// `None` for no chord.
    pub fn of(played: &Played, key: Scale) -> Option<Over> {
        let (chord, tones, scale) = match played.bass {
            BassPart::Chord { chord, .. } => {
                (Some(chord), chord.pitch_set(), chord_scale(chord, key))
            }
            BassPart::Pedal(_) => {
                let tones: PitchSet = played
                    .voicing
                    .iter()
                    .map(|note| note.pitch_class())
                    .collect();
                (None, tones, tones)
            }
            BassPart::Rest => return None,
        };
        Some(Over {
            start: played.start,
            end: played.end,
            chord,
            tones,
            scale,
        })
    }

    /// The name of its scale, as "G Mixolydian", if it has a usual one.
    pub fn scale_name(&self, key: Scale) -> Option<String> {
        let chord = self.chord?;
        let name = chord_scale_name(self.scale, chord.root)?;
        Some(format!("{} {name}", key.spell_root(chord)))
    }
}

/// The harmony sounding at `step`.
fn over_at(overs: &[Over], step: usize) -> Option<&Over> {
    overs
        .iter()
        .find(|over| (over.start..over.end).contains(&step))
}

/// What a note of a solo line is, over the harmony where it starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteRole {
    /// A note of the harmony. For a chord, which one, counted from the
    /// root as musicians count (1, 3, 5, 7, 9...), as
    /// [`Chord::tone_of`] says.
    ChordTone(Option<u8>),
    /// A note of the scale, a step from the notes before and after it,
    /// going the same way: E F G.
    Passing,
    /// A note of the scale a step from the notes before and after it,
    /// which are the same note: E F E.
    Neighbour,
    /// A note from outside the scale between two notes a whole step
    /// apart, a half step from each: D D# E.
    ChromaticPassing,
    /// A note from outside the scale a half step from the next one,
    /// leading into it: F# G.
    Approach,
    /// Another note of the scale, reached or left by a leap.
    ScaleTone,
    /// Another note from outside the scale.
    Outside,
}

impl NoteRole {
    /// Whether it is a note of the harmony.
    pub fn is_chord_tone(self) -> bool {
        matches!(self, NoteRole::ChordTone(_))
    }

    /// Whether it is from outside the scale.
    pub fn is_chromatic(self) -> bool {
        matches!(
            self,
            NoteRole::ChromaticPassing | NoteRole::Approach | NoteRole::Outside
        )
    }

    /// A few words for it: "the 3rd", "passing note".
    pub fn short(self) -> String {
        match self {
            NoteRole::ChordTone(Some(degree)) => degree_name(degree),
            NoteRole::ChordTone(None) => "a note of the harmony".to_string(),
            NoteRole::Passing => "passing note".to_string(),
            NoteRole::Neighbour => "neighbour note".to_string(),
            NoteRole::ChromaticPassing => "chromatic passing note".to_string(),
            NoteRole::Approach => "chromatic approach".to_string(),
            NoteRole::ScaleTone => "scale note".to_string(),
            NoteRole::Outside => "outside the scale".to_string(),
        }
    }

    /// One or two characters to write on the note: "3", "p", "n".
    pub fn mark(self) -> String {
        match self {
            NoteRole::ChordTone(Some(1)) => "R".to_string(),
            NoteRole::ChordTone(Some(degree)) => degree.to_string(),
            NoteRole::ChordTone(None) => "•".to_string(),
            NoteRole::Passing => "p".to_string(),
            NoteRole::Neighbour => "n".to_string(),
            NoteRole::ChromaticPassing => "cp".to_string(),
            NoteRole::Approach => "a".to_string(),
            NoteRole::ScaleTone => "s".to_string(),
            NoteRole::Outside => "x".to_string(),
        }
    }

    /// A sentence about the note `name` over `over`, in `key`: "E4, the
    /// 3rd of C7: a chord tone." Other notes end with the scale of the
    /// chord: "Over G7 the scale is G Mixolydian: G A B C D E F."
    pub fn describe(self, name: &str, over: &Over, key: Scale) -> String {
        let what = match self {
            NoteRole::ChordTone(degree) => {
                let chord = over.chord.map(|chord| key.chord_name(chord));
                return match (degree, chord) {
                    (Some(degree), Some(chord)) => {
                        format!(
                            "{name}, the {} of {chord}: a chord tone.",
                            degree_name(degree)
                        )
                    }
                    _ => format!("{name}: a note of the harmony."),
                };
            }
            NoteRole::Passing => "a passing note, a step between the notes either side",
            NoteRole::Neighbour => "a neighbour note, a step from the note before and back",
            NoteRole::ChromaticPassing => {
                "a chromatic passing note, a half step from the notes either side"
            }
            NoteRole::Approach => "a chromatic approach, a half step from the next note",
            NoteRole::ScaleTone => "a note of the scale",
            NoteRole::Outside => "a note from outside the scale",
        };
        match (over.chord, over.scale_name(key)) {
            (Some(chord), Some(scale)) => format!(
                "{name}: {what}. Over {} the scale is {scale}: {}.",
                key.chord_name(chord),
                spell_set(over.scale, over.chord, key)
            ),
            _ => format!("{name}: {what}."),
        }
    }
}

/// "the root", "3rd", "♭9"...: which note of a chord a degree is.
fn degree_name(degree: u8) -> String {
    match degree {
        1 => "root".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{degree}th"),
    }
}

/// The notes of `scale` from the chord's root up, spelled as the key
/// spells them, the chord's own as the chord does.
fn spell_set(scale: PitchSet, chord: Option<Chord>, key: Scale) -> String {
    let root = chord.map_or(PitchClass::C, |chord| chord.root);
    let spelled = chord
        .map(|chord| key.chord_notes(chord))
        .unwrap_or_default();
    let mut semitones = scale.relative_to(root).semitones();
    semitones.sort_unstable();
    semitones
        .iter()
        .map(|&up| {
            let pitch_class = root.transpose(i32::from(up));
            spelled
                .iter()
                .find(|name| name.pitch_class() == pitch_class)
                .copied()
                .unwrap_or_else(|| key.spell(pitch_class))
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// How far apart two notes of a line may be and still be a step: a half
/// or a whole step, or an augmented second (three half steps) between
/// two neighbouring notes of `scale`, as in harmonic minor.
fn is_step(from: u8, to: u8, scale: PitchSet) -> bool {
    match from.abs_diff(to) {
        1 | 2 => true,
        3 => {
            let (low, high) = (from.min(to), from.max(to));
            !(low + 1..high).any(|pitch| scale.contains(Note::new(pitch).pitch_class()))
        }
        _ => false,
    }
}

/// Notes of a line closer than this, in steps, follow each other: an
/// eighth-note rest does not break it.
const JOINED: usize = 2;

/// What each note of a line is, over the harmonies `overs`. The notes
/// must be in order and one at a time, as a solo is; `None` for a note
/// where no harmony sounds.
pub fn note_roles(notes: &[NoteEvent], overs: &[Over]) -> Vec<Option<NoteRole>> {
    let joined = |before: &NoteEvent, after: &NoteEvent| {
        before.start < after.start && before.start + before.length + JOINED >= after.start
    };
    (0..notes.len())
        .map(|index| {
            let note = &notes[index];
            let over = over_at(overs, note.start)?;
            let pitch = note.pitch.midi();
            let pitch_class = note.pitch.pitch_class();
            if over.tones.contains(pitch_class) {
                let degree = over.chord.and_then(|chord| chord.tone_of(pitch_class));
                return Some(NoteRole::ChordTone(degree));
            }
            let before = index
                .checked_sub(1)
                .map(|index| &notes[index])
                .filter(|before| joined(before, note))
                .map(|before| before.pitch.midi());
            let after = notes
                .get(index + 1)
                .filter(|after| joined(note, after))
                .map(|after| after.pitch.midi());
            let role = if over.scale.contains(pitch_class) {
                match (before, after) {
                    (Some(before), Some(after))
                        if before == after && is_step(before, pitch, over.scale) =>
                    {
                        NoteRole::Neighbour
                    }
                    (Some(before), Some(after))
                        if (before < pitch) == (pitch < after)
                            && before != pitch
                            && pitch != after
                            && is_step(before, pitch, over.scale)
                            && is_step(pitch, after, over.scale) =>
                    {
                        NoteRole::Passing
                    }
                    _ => NoteRole::ScaleTone,
                }
            } else {
                match (before, after) {
                    (Some(before), Some(after))
                        if before.abs_diff(pitch) == 1
                            && pitch.abs_diff(after) == 1
                            && before != after =>
                    {
                        NoteRole::ChromaticPassing
                    }
                    (_, Some(after)) if pitch.abs_diff(after) == 1 => NoteRole::Approach,
                    _ => NoteRole::Outside,
                }
            };
            Some(role)
        })
        .collect()
}

/// Who plays the solos in a style of each genre: a General MIDI
/// instrument, and the notes it plays them in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Soloist {
    pub program: u8,
    pub low: u8,
    pub high: u8,
}

impl Soloist {
    /// The soloist of the styles of `genre`: a tenor saxophone for jazz,
    /// a harmonica for the blues, a bandoneon for tango...
    pub fn of(genre: Genre) -> Soloist {
        let (program, low, high) = match genre {
            Genre::Jazz => (gm::TENOR_SAX, 50, 77),
            Genre::Pop | Genre::Soul => (gm::ALTO_SAX, 55, 79),
            Genre::Blues => (gm::HARMONICA, 60, 84),
            Genre::Rock => (gm::OVERDRIVEN_GUITAR, 57, 84),
            Genre::Latin | Genre::Reggae => (gm::TRUMPET, 58, 81),
            Genre::Tango => (gm::TANGO_ACCORDION, 55, 84),
            Genre::Folklore => (gm::PAN_FLUTE, 64, 88),
            Genre::Flamenco => (gm::NYLON_GUITAR, 52, 81),
            Genre::Folk => (gm::FIDDLE, 55, 88),
            Genre::Classical => (gm::FLUTE, 65, 89),
            Genre::HipHop => (gm::VIBRAPHONE, 60, 84),
            Genre::Electronic => (gm::SAW_LEAD, 60, 84),
        };
        Soloist { program, low, high }
    }
}

/// A pulse of the solo: a beat, or a quarter note of a longer beat (a
/// half note of 2/2 has two). The soloist plays a chord tone on each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pulse {
    start: usize,
    length: usize,
}

impl Pulse {
    fn end(self) -> usize {
        self.start + self.length
    }
}

/// The pulses of `bars`.
fn pulses(bars: &[Bar]) -> Vec<Pulse> {
    let mut pulses = Vec::new();
    for bar in bars {
        let starts: Vec<usize> = bar.beat_starts().collect();
        for (index, &start) in starts.iter().enumerate() {
            let end = starts.get(index + 1).copied().unwrap_or(bar.end());
            let length = end - start;
            if length > 4 && length.is_multiple_of(4) {
                pulses.extend(
                    (start..end)
                        .step_by(4)
                        .map(|start| Pulse { start, length: 4 }),
                );
            } else {
                pulses.push(Pulse { start, length });
            }
        }
    }
    pulses
}

/// A source of numbers that look random, always the same from the same
/// seed (the "SplitMix64" generator).
struct Dice(u64);

impl Dice {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut x = self.0;
        x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^ (x >> 31)
    }

    /// True `percent` times in a hundred.
    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }

    /// A number from 0 up to `count`, not included.
    fn below(&mut self, count: usize) -> usize {
        (self.next() % count.max(1) as u64) as usize
    }
}

/// How hard the soloist plays a note on the beat, and one between; the
/// melody's shaping (see [`crate::library::interpret`]) adds the rest.
const ON_THE_BEAT: u8 = 92;
const BETWEEN: u8 = 82;

/// An improvised solo over `overs` along `bars`, from step `from` up to
/// `to`, in the `soloist`'s range. The same `seed` plays the same solo.
pub(super) fn improvise(
    bars: &[Bar],
    overs: &[Over],
    from: usize,
    to: usize,
    soloist: Soloist,
    seed: u64,
) -> Vec<NoteEvent> {
    let mut dice = Dice(seed);
    let bars: Vec<Bar> = bars
        .iter()
        .filter(|bar| bar.start >= from && bar.end() <= to)
        .copied()
        .collect();
    let mut line = Line {
        soloist,
        up: dice.chance(50),
        last: None,
        notes: Vec::new(),
    };
    for phrase in bars.chunks(2) {
        line.phrase(&pulses(phrase), overs, &mut dice);
    }
    line.notes
}

/// A solo as it is being played.
struct Line {
    soloist: Soloist,
    /// Whether the line is going up.
    up: bool,
    /// The last note played.
    last: Option<u8>,
    notes: Vec<NoteEvent>,
}

impl Line {
    /// Plays a phrase over `pulses`: it may start a beat late or with a
    /// pickup, and leaves a beat or two of rest at the end.
    fn phrase(&mut self, pulses: &[Pulse], overs: &[Over], dice: &mut Dice) {
        let count = pulses.len();
        let late = usize::from(count >= 4 && dice.chance(35));
        let rest = match count {
            6.. => 1 + dice.below(2),
            3.. => 1,
            _ => 0,
        };
        let Some(window) = pulses.get(late..count - rest) else {
            return;
        };
        // First the chord tones on the beats, then what goes between.
        let targets: Vec<Option<u8>> = window
            .iter()
            .map(|pulse| {
                let over = over_at(overs, pulse.start)?;
                let target = self.target(over.tones, dice);
                self.last = Some(target);
                Some(target)
            })
            .collect();
        let pickup = dice.chance(25);
        for (index, pulse) in window.iter().enumerate() {
            let Some(target) = targets[index] else {
                continue;
            };
            let next = window
                .get(index + 1)
                .filter(|next| next.start == pulse.end())
                .and(targets.get(index + 1).copied().flatten());
            let slots = (pulse.length / 2).max(1);
            // What goes between is from the scale of the harmony of the
            // beat; if another comes in during the beat, the note is held.
            let over = over_at(overs, pulse.start);
            let one_harmony = over == over_at(overs, pulse.end() - 1);
            let fill = match (next, over) {
                (Some(next), Some(over)) if one_harmony && dice.chance(70) => {
                    fill(target, next, slots - 1, over.scale, self.soloist, dice)
                }
                _ => Vec::new(),
            };
            // A phrase may start with a note just before its first beat.
            if index == 0
                && pickup
                && pulse.start >= 2
                && let Some(over) = over_at(overs, pulse.start - 2)
            {
                let approach = approach(target, over.scale, dice);
                self.play(approach, pulse.start - 2, 2, BETWEEN);
            }
            let held = pulse.length - 2 * fill.len();
            self.play(target, pulse.start, held, ON_THE_BEAT);
            for (slot, &pitch) in fill.iter().enumerate() {
                self.play(pitch, pulse.start + held + 2 * slot, 2, BETWEEN);
            }
        }
    }

    /// The chord tone (a note of `tones`) the line goes to next: the
    /// nearest one in the way it is going, now and then the one after,
    /// turning back at the edges of the soloist's range and now and then
    /// on its own.
    fn target(&mut self, tones: PitchSet, dice: &mut Dice) -> u8 {
        let Soloist { low, high, .. } = self.soloist;
        let candidates: Vec<u8> = (low..=high)
            .filter(|&pitch| tones.contains(Note::new(pitch).pitch_class()))
            .collect();
        let Some(last) = self.last else {
            let middle = (low + high) / 2 + dice.below(7) as u8 - 3;
            return nearest(&candidates, middle).unwrap_or(middle);
        };
        if last + 3 >= high {
            self.up = false;
        } else if last <= low + 3 {
            self.up = true;
        } else if dice.chance(30) {
            self.up = !self.up;
        }
        for _ in 0..2 {
            let ahead: Vec<u8> = match self.up {
                true => candidates.iter().copied().filter(|&p| p > last).collect(),
                false => candidates
                    .iter()
                    .rev()
                    .copied()
                    .filter(|&p| p < last)
                    .collect(),
            };
            if let Some(&first) = ahead.first() {
                let leap = ahead.len() > 1 && dice.chance(20);
                return if leap { ahead[1] } else { first };
            }
            self.up = !self.up;
        }
        last
    }

    fn play(&mut self, pitch: u8, start: usize, length: usize, velocity: u8) {
        self.notes.push(NoteEvent {
            velocity,
            ..NoteEvent::new(Note::new(pitch), start, length.max(1))
        });
    }
}

/// The note of `candidates` nearest to `pitch`.
fn nearest(candidates: &[u8], pitch: u8) -> Option<u8> {
    candidates
        .iter()
        .copied()
        .min_by_key(|candidate| candidate.abs_diff(pitch))
}

/// Up to `slots` notes from `scale` to go from the chord tone `from` to
/// the chord tone `to` in between two beats: passing notes a step apart
/// when they fit, a neighbour note when the two are the same, a
/// chromatic passing note between two notes a whole step apart, or else
/// a note just before `to`.
fn fill(
    from: u8,
    to: u8,
    slots: usize,
    scale: PitchSet,
    soloist: Soloist,
    dice: &mut Dice,
) -> Vec<u8> {
    if slots == 0 {
        return Vec::new();
    }
    let in_scale = |pitch: u8| scale.contains(Note::new(pitch).pitch_class());
    if from == to {
        let above = (from + 1..=from + 3).find(|&pitch| in_scale(pitch));
        let below = (from.saturating_sub(3)..from)
            .rev()
            .find(|&pitch| in_scale(pitch));
        let upper = dice.chance(50) && above.is_some_and(|pitch| pitch <= soloist.high);
        let neighbour = match upper {
            true => above,
            false => below.or(above),
        }
        .filter(|&pitch| is_step(from, pitch, scale));
        return match (neighbour, slots) {
            (None, _) => Vec::new(),
            (Some(pitch), 1) => vec![pitch],
            (Some(pitch), _) => vec![pitch, from],
        };
    }
    let between: Vec<u8> = match from < to {
        true => (from + 1..to).filter(|&pitch| in_scale(pitch)).collect(),
        false => (to + 1..from)
            .rev()
            .filter(|&pitch| in_scale(pitch))
            .collect(),
    };
    let steps = std::iter::once(from)
        .chain(between.iter().copied())
        .chain(std::iter::once(to))
        .collect::<Vec<u8>>();
    let stepwise = steps
        .windows(2)
        .all(|pair| is_step(pair[0], pair[1], scale));
    if !between.is_empty() && between.len() <= slots && stepwise {
        return between;
    }
    match from.abs_diff(to) {
        // A whole step with no note of the scale between: chromatic.
        2 if between.is_empty() => vec![(from + to) / 2],
        0..=3 => Vec::new(),
        // Too far to walk: leap to a note next to `to`, from the scale
        // or a half step away.
        _ => {
            let side = |pitch: u8| match from < to {
                true => pitch - 1,
                false => pitch + 1,
            };
            let mut notes: Vec<u8> = between.iter().rev().take(slots).rev().copied().collect();
            if dice.chance(50) {
                notes.pop();
                notes.push(side(to));
            }
            notes
        }
    }
}

/// A note just before `to`: a half step below, now and then a step of
/// `scale` above.
fn approach(to: u8, scale: PitchSet, dice: &mut Dice) -> u8 {
    let above = (to + 1..=to + 2).find(|&pitch| scale.contains(Note::new(pitch).pitch_class()));
    match above {
        Some(above) if dice.chance(30) => above,
        _ => to - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::{ChordSymbol, ScaleKind};
    use crate::meter::{Meter, TimeSignature};

    const C_MAJOR: Scale = Scale {
        root: PitchClass::C,
        kind: ScaleKind::Major,
    };

    /// One harmony a bar of `time`, "N.C." for none.
    fn chart(names: &[&str], time: TimeSignature, key: Scale) -> (Vec<Bar>, Vec<Over>) {
        let meter = Meter::steady(time);
        let bars: Vec<Bar> = meter.bars().take(names.len()).collect();
        let overs = bars
            .iter()
            .zip(names)
            .filter(|(_, name)| **name != "N.C.")
            .map(|(bar, name)| {
                let chord = ChordSymbol::parse(name).unwrap().chord();
                Over {
                    start: bar.start,
                    end: bar.end(),
                    chord: Some(chord),
                    tones: chord.pitch_set(),
                    scale: chord_scale(chord, key),
                }
            })
            .collect();
        (bars, overs)
    }

    fn note(pitch: u8, start: usize, length: usize) -> NoteEvent {
        NoteEvent::new(Note::new(pitch), start, length)
    }

    #[test]
    fn roles_name_what_each_note_does() {
        let (_, overs) = chart(&["C"], TimeSignature::COMMON, C_MAJOR);
        // E F G A G, then D# E, then F# alone.
        let line = [
            note(64, 0, 2),
            note(65, 2, 2),
            note(67, 4, 2),
            note(69, 6, 2),
            note(67, 8, 1),
            note(63, 9, 1),
            note(64, 10, 2),
            note(66, 15, 1),
        ];
        let roles: Vec<NoteRole> = note_roles(&line, &overs).into_iter().flatten().collect();
        assert_eq!(
            roles,
            [
                NoteRole::ChordTone(Some(3)),
                NoteRole::Passing,
                NoteRole::ChordTone(Some(5)),
                NoteRole::Neighbour,
                NoteRole::ChordTone(Some(5)),
                NoteRole::Approach,
                NoteRole::ChordTone(Some(3)),
                NoteRole::Outside,
            ]
        );
        let chromatic = note_roles(&[note(62, 0, 1), note(63, 1, 1), note(64, 2, 1)], &overs);
        assert_eq!(chromatic[1], Some(NoteRole::ChromaticPassing));
    }

    #[test]
    fn a_role_is_described_with_its_chord_and_scale() {
        let (_, overs) = chart(&["G7"], TimeSignature::COMMON, C_MAJOR);
        let over = &overs[0];
        assert_eq!(
            NoteRole::ChordTone(Some(3)).describe("B4", over, C_MAJOR),
            "B4, the 3rd of G7: a chord tone."
        );
        assert_eq!(
            NoteRole::Passing.describe("A4", over, C_MAJOR),
            "A4: a passing note, a step between the notes either side. Over G7 the scale is \
             G Mixolydian: G A B C D E F."
        );
    }

    /// A solo the tests play: its bars, its harmonies, its notes and who
    /// played it.
    type Solo = (Vec<Bar>, Vec<Over>, Vec<NoteEvent>, Soloist);

    /// Every solo the tests play: these charts with many seeds.
    fn solos() -> Vec<Solo> {
        let a_minor = Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        let charts = [
            (
                vec!["C", "Am", "F", "G7", "C", "Am", "Dm7", "G7"],
                TimeSignature::COMMON,
                C_MAJOR,
            ),
            (
                vec!["Dm7", "G7", "Cmaj7", "A7", "Dm7", "G7", "C6", "C6"],
                TimeSignature::COMMON,
                C_MAJOR,
            ),
            (
                vec!["Am", "Dm", "E7", "Am", "F", "E7", "Am", "N.C."],
                TimeSignature::new(3, 4),
                a_minor,
            ),
            (
                vec!["C", "F", "G7", "C", "Fm", "C", "G7", "C"],
                TimeSignature::new(6, 8),
                C_MAJOR,
            ),
            (
                vec!["C7", "F7", "C7", "C7", "F7", "F7", "C7", "G7"],
                TimeSignature::new(2, 2),
                C_MAJOR,
            ),
        ];
        let mut solos = Vec::new();
        for (names, time, key) in charts {
            let (bars, overs) = chart(&names, time, key);
            let end = bars.last().unwrap().end();
            for (genre, seed) in [(Genre::Jazz, 1), (Genre::Classical, 2), (Genre::Blues, 3)] {
                for more in 0..8 {
                    let soloist = Soloist::of(genre);
                    let seed = seed * 100 + more;
                    let notes = improvise(&bars, &overs, 0, end, soloist, seed);
                    solos.push((bars.clone(), overs.clone(), notes, soloist));
                }
            }
        }
        solos
    }

    #[test]
    fn chord_tones_on_the_beats_and_nothing_unexplained_between() {
        for (bars, overs, notes, _) in solos() {
            assert!(notes.len() > 8, "a solo plays");
            let beats: Vec<usize> = pulses(&bars).iter().map(|pulse| pulse.start).collect();
            for (note, role) in notes.iter().zip(note_roles(&notes, &overs)) {
                let role = role.expect("a note over a harmony");
                assert_ne!(role, NoteRole::Outside, "{note:?} in {notes:?}");
                if beats.contains(&note.start) {
                    assert!(role.is_chord_tone(), "{note:?} on the beat is {role:?}");
                }
            }
        }
    }

    #[test]
    fn a_solo_is_one_line_in_range_that_breathes() {
        for (bars, overs, notes, soloist) in solos() {
            for pair in notes.windows(2) {
                assert!(pair[0].start + pair[0].length <= pair[1].start, "{pair:?}");
            }
            for note in &notes {
                let pitch = note.pitch.midi();
                assert!((soloist.low - 2..=soloist.high + 2).contains(&pitch));
                // Silent where there is no chord.
                assert!(over_at(&overs, note.start).is_some());
            }
            // Every two bars, a rest of an eighth note or more.
            for phrase in bars.chunks(2) {
                let (start, end) = (phrase[0].start, phrase[phrase.len() - 1].end());
                let mut sounding = vec![false; end - start];
                for note in notes
                    .iter()
                    .filter(|note| (start..end).contains(&note.start))
                {
                    for step in note.start..(note.start + note.length).min(end) {
                        sounding[step - start] = true;
                    }
                }
                let longest = sounding
                    .split(|&on| on)
                    .map(<[bool]>::len)
                    .max()
                    .unwrap_or(0);
                assert!(longest >= 2, "a breath in {phrase:?}");
            }
        }
    }

    #[test]
    fn the_same_seed_plays_the_same_solo() {
        let (bars, overs) = chart(&["C", "Am", "F", "G7"], TimeSignature::COMMON, C_MAJOR);
        let soloist = Soloist::of(Genre::Jazz);
        let one = improvise(&bars, &overs, 0, 64, soloist, 7);
        assert_eq!(one, improvise(&bars, &overs, 0, 64, soloist, 7));
        assert_ne!(one, improvise(&bars, &overs, 0, 64, soloist, 8));
    }
}
