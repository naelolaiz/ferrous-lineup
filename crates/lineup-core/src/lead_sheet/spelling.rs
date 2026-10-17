//! Writing pitches on the staff: which letter each note gets, where it
//! sits, and which accidentals must be printed.

use std::collections::HashMap;

use crate::harmony::{ChordSymbol, Scale};
use crate::pitch::{Letter, Note, NoteName};

/// The clef the tune is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Clef {
    #[default]
    Treble,
    Bass,
}

impl Clef {
    /// The note on the bottom line, as a count of letters from C0
    /// ([`staff_step`]): E4 in the treble clef, G2 in the bass clef.
    pub fn bottom_line(self) -> i32 {
        match self {
            Clef::Treble => 4 * 7 + 2,
            Clef::Bass => 2 * 7 + 4,
        }
    }

    /// The clef that suits a tune whose middle note is `median`: the
    /// bass clef for tunes centred below G3, where the treble clef would
    /// need many leger lines.
    pub fn for_median(median: Note) -> Clef {
        if median.midi() < 55 {
            Clef::Bass
        } else {
            Clef::Treble
        }
    }
}

/// How many letters a written note is above C0: C4 is 28, D4 29. Two
/// notes on the same line or space of the staff have the same number.
///
/// The octave is the written letter's: B#3 sounds like C4 but sits on
/// the B line of octave 3, and Cb5 sounds like B4 but sits on C5's space.
pub fn staff_step(note: Note, name: NoteName) -> i32 {
    let natural = i32::from(note.midi()) - i32::from(name.accidental);
    let octave = natural.div_euclid(12) - 1;
    octave * 7 + name.letter as i32
}

/// The letters, in order, that a key signature puts sharps on, and then
/// flats on.
const SHARP_ORDER: [Letter; 7] = [
    Letter::F,
    Letter::C,
    Letter::G,
    Letter::D,
    Letter::A,
    Letter::E,
    Letter::B,
];
const FLAT_ORDER: [Letter; 7] = [
    Letter::B,
    Letter::E,
    Letter::A,
    Letter::D,
    Letter::G,
    Letter::C,
    Letter::F,
];

/// A key signature: how many sharps (positive) or flats (negative).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeySignature(pub i8);

impl KeySignature {
    /// The signature `key` is written with; none for a row, a set or a
    /// chromatic scale.
    pub fn of(key: Scale) -> KeySignature {
        KeySignature(key.key_signature().map_or(0, |(count, _)| count))
    }

    /// The accidental the signature gives `letter`.
    pub fn on(self, letter: Letter) -> i8 {
        let count = usize::from(self.0.unsigned_abs());
        if self.0 > 0 && SHARP_ORDER[..count].contains(&letter) {
            1
        } else if self.0 < 0 && FLAT_ORDER[..count].contains(&letter) {
            -1
        } else {
            0
        }
    }

    /// The signature's accidentals in the order they are written, with
    /// where each sits on the staff in `clef` ([`staff_step`]).
    ///
    /// The sharps zigzag from the top line down (F5 C5 G5 D5 A4 E5 B4 in
    /// the treble clef) and the flats from the middle line (B4 E5 A4 D5
    /// G4 C5 F4); the bass clef has the same shapes two octaves lower.
    pub fn accidentals(self, clef: Clef) -> Vec<(i8, i32)> {
        const SHARPS: [i32; 7] = [38, 35, 39, 36, 33, 37, 34];
        const FLATS: [i32; 7] = [34, 37, 33, 36, 32, 35, 31];
        let shift = match clef {
            Clef::Treble => 0,
            Clef::Bass => 14,
        };
        let count = usize::from(self.0.unsigned_abs());
        let (sign, steps) = if self.0 > 0 { (1, SHARPS) } else { (-1, FLATS) };
        steps[..count]
            .iter()
            .map(|&step| (sign, step - shift))
            .collect()
    }

    /// What must be cancelled with naturals when the key changes from
    /// this signature to `next`: the old sharps or flats the new
    /// signature does not have, in the order they were written. Going
    /// from sharps to flats or back, or to no accidentals at all, every
    /// old one is cancelled.
    pub fn cancelled_by(self, next: KeySignature) -> Vec<Letter> {
        let old: Vec<Letter> = match self.0 {
            0 => Vec::new(),
            n if n > 0 => SHARP_ORDER[..n as usize].to_vec(),
            n => FLAT_ORDER[..n.unsigned_abs() as usize].to_vec(),
        };
        old.into_iter()
            .filter(|&letter| next.on(letter) != self.on(letter))
            .collect()
    }

    /// Where the naturals go, in `clef`, that cancel this signature's
    /// accidentals when the key changes to `next`: on the lines and
    /// spaces the old sharps or flats were on.
    pub fn cancellations(self, next: KeySignature, clef: Clef) -> Vec<i32> {
        let cancelled = self.cancelled_by(next);
        let order = if self.0 > 0 { SHARP_ORDER } else { FLAT_ORDER };
        order
            .iter()
            .zip(self.accidentals(clef))
            .filter(|(letter, _)| cancelled.contains(letter))
            .map(|(_, (_, step))| step)
            .collect()
    }
}

/// How a note of the tune is written in `key`, given the chord sounding
/// with it and the note that comes next.
///
/// - A note of the key is written as the key writes it (in A major, C#).
/// - A note of the chord is written as the chord's own note: over E7 in
///   C major, the G# of the chord, not Ab.
/// - Any other note that moves by a semitone to the next note is written
///   the way the line goes: raised going up (F# to G), lowered going
///   down (Bb to A), on the letter of the note next to it.
/// - Otherwise, as the key writes notes outside it (see [`Scale::spell`]).
///
/// Outside the key, a white key of the piano is written as one (B, not
/// Cb), unless the chord has it otherwise.
pub fn spell(key: Scale, pitch: Note, chord: Option<ChordSymbol>, next: Option<Note>) -> NoteName {
    let pitch_class = pitch.pitch_class();
    if key.has_letter_for(pitch_class) {
        return key.spell(pitch_class);
    }
    if let Some(symbol) = chord {
        let names = symbol.chord().spell(symbol.root);
        let bass = symbol.bass.into_iter();
        if let Some(name) = names
            .into_iter()
            .chain(bass)
            .find(|name| name.pitch_class() == pitch_class && name.accidental.abs() <= 1)
        {
            return name;
        }
    }
    if key.kind.is_tonal()
        && let Some(next) = next
    {
        let step = i32::from(next.midi()) - i32::from(pitch.midi());
        let neighbour = match step {
            1 => Some(key.spell(next.pitch_class()).letter.up(-1)),
            -1 => Some(key.spell(next.pitch_class()).letter.up(1)),
            _ => None,
        };
        if let Some(letter) = neighbour {
            let name = NoteName::with_letter(pitch_class, letter);
            if name.accidental.abs() <= 1 && !is_odd(name) {
                return name;
            }
        }
    }
    let name = key.spell(pitch_class);
    if is_odd(name) {
        NoteName::chromatic(pitch_class)
    } else {
        name
    }
}

/// Whether `name` writes a white key of the piano with a sharp or a
/// flat: Cb, Fb, E# or B#. Keys with many sharps or flats have them, but
/// as a passing note C – Cb – Bb reads worse than C – B – Bb.
fn is_odd(name: NoteName) -> bool {
    name.accidental != 0 && matches!(name.pitch_class().semitone(), 0 | 2 | 4 | 5 | 7 | 9 | 11)
}

/// The accidentals in force in a bar: the key signature's, changed by
/// any accidental printed earlier in the bar on the same line or space.
#[derive(Debug, Clone)]
pub struct BarAccidentals {
    signature: KeySignature,
    printed: HashMap<i32, i8>,
}

impl BarAccidentals {
    pub fn new(signature: KeySignature) -> Self {
        BarAccidentals {
            signature,
            printed: HashMap::new(),
        }
    }

    /// The accidental to print before `name` written at `step`, if one
    /// is needed, and from then on in the bar it is in force there.
    ///
    /// A note held over the bar line with a tie keeps its accidental
    /// without printing it again (`tied_in`), but the accidental does not
    /// carry on to the next notes of the new bar: they are compared with
    /// the key signature as usual.
    pub fn print(&mut self, step: i32, name: NoteName, tied_in: bool) -> Option<i8> {
        if tied_in {
            return None;
        }
        let in_force = self
            .printed
            .get(&step)
            .copied()
            .unwrap_or_else(|| self.signature.on(name.letter));
        if in_force == name.accidental {
            None
        } else {
            self.printed.insert(step, name.accidental);
            Some(name.accidental)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::{ChordQuality, ScaleKind};
    use crate::pitch::PitchClass;

    fn key(root: PitchClass, kind: ScaleKind) -> Scale {
        Scale::new(root, kind)
    }

    fn written(key: Scale, midi: u8, chord: Option<&str>, next: Option<u8>) -> String {
        let symbol = chord.map(|text| ChordSymbol::parse(text).unwrap());
        let name = spell(key, Note::new(midi), symbol, next.map(Note::new));
        name.to_string()
    }

    #[test]
    fn notes_of_the_key_are_written_as_the_key_writes_them() {
        let a_major = key(PitchClass::A, ScaleKind::Major);
        assert_eq!(written(a_major, 61, None, None), "C#");
        let f_sharp_major = key(PitchClass::FSharp, ScaleKind::Major);
        assert_eq!(written(f_sharp_major, 65, None, None), "E#");
        let e_flat = key(PitchClass::DSharp, ScaleKind::Major);
        assert_eq!(written(e_flat, 68, None, None), "Ab");
    }

    #[test]
    fn a_note_of_the_chord_is_written_as_the_chord_has_it() {
        let c_major = key(PitchClass::C, ScaleKind::Major);
        // G# over E7 (the V of A minor), not Ab.
        assert_eq!(written(c_major, 68, Some("E7"), None), "G#");
        // The same key is Ab over Fm (the iv of C minor).
        assert_eq!(written(c_major, 68, Some("Fm"), None), "Ab");
        // Bb over C7 (on its way to F).
        assert_eq!(written(c_major, 70, Some("C7"), None), "Bb");
        // Eb over Ab (borrowed from C minor).
        assert_eq!(written(c_major, 63, Some("Ab"), None), "Eb");
    }

    #[test]
    fn a_chromatic_step_is_written_the_way_the_line_goes() {
        let c_major = key(PitchClass::C, ScaleKind::Major);
        // F# going up to G; Gb going down to F.
        assert_eq!(written(c_major, 66, None, Some(67)), "F#");
        assert_eq!(written(c_major, 66, None, Some(65)), "Gb");
        // C# up to D, Db down to C.
        assert_eq!(written(c_major, 61, None, Some(62)), "C#");
        assert_eq!(written(c_major, 61, None, Some(60)), "Db");
        // A leap: as the key writes it, a flat third.
        assert_eq!(written(c_major, 63, None, Some(70)), "Eb");
    }

    #[test]
    fn white_keys_keep_their_own_names() {
        let f_major = key(PitchClass::F, ScaleKind::Major);
        // C5, B4, Bb4 going down: B natural, not C flat.
        assert_eq!(written(f_major, 71, Some("F"), Some(70)), "B");
        assert_eq!(written(f_major, 71, None, None), "B");
        // In F# major E# is a note of the key.
        assert_eq!(
            written(key(PitchClass::FSharp, ScaleKind::Major), 65, None, None),
            "E#"
        );
    }

    #[test]
    fn minor_keys_raise_their_leading_tone() {
        let a_minor = key(PitchClass::A, ScaleKind::NaturalMinor);
        assert_eq!(written(a_minor, 68, None, Some(69)), "G#");
        assert_eq!(written(a_minor, 68, None, None), "G#");
        let d_minor = key(PitchClass::D, ScaleKind::NaturalMinor);
        assert_eq!(written(d_minor, 61, None, None), "C#");
    }

    #[test]
    fn notes_sit_on_their_written_letter() {
        let c4 = Note::new(60);
        assert_eq!(staff_step(c4, NoteName::parse("C").unwrap()), 28);
        // B#3 sounds as C4 but sits on the B of octave 3.
        assert_eq!(staff_step(c4, NoteName::parse("B#").unwrap()), 27);
        // Cb5 sounds as B4 but sits on C5.
        assert_eq!(
            staff_step(Note::new(71), NoteName::parse("Cb").unwrap()),
            35
        );
    }

    #[test]
    fn key_signatures_are_drawn_in_their_usual_places() {
        let three_sharps = KeySignature::of(key(PitchClass::A, ScaleKind::Major));
        assert_eq!(three_sharps, KeySignature(3));
        assert_eq!(
            three_sharps.accidentals(Clef::Treble),
            [(1, 38), (1, 35), (1, 39)]
        );
        let two_flats = KeySignature::of(key(PitchClass::G, ScaleKind::NaturalMinor));
        assert_eq!(two_flats.accidentals(Clef::Treble), [(-1, 34), (-1, 37)]);
        // In the bass clef, F#3 on the fourth line.
        assert_eq!(three_sharps.accidentals(Clef::Bass)[0], (1, 24));
        assert_eq!(three_sharps.on(Letter::G), 1);
        assert_eq!(three_sharps.on(Letter::D), 0);
    }

    #[test]
    fn a_change_of_key_cancels_what_the_new_key_does_not_have() {
        let d_major = KeySignature(2);
        assert_eq!(d_major.cancelled_by(KeySignature(1)), [Letter::C]);
        assert_eq!(
            d_major.cancelled_by(KeySignature(0)),
            [Letter::F, Letter::C]
        );
        assert_eq!(
            d_major.cancelled_by(KeySignature(-1)),
            [Letter::F, Letter::C]
        );
        assert!(d_major.cancelled_by(KeySignature(4)).is_empty());
        // D major to G major: a natural on the C# space.
        assert_eq!(d_major.cancellations(KeySignature(1), Clef::Treble), [35]);
        assert_eq!(
            KeySignature(-2).cancellations(KeySignature(0), Clef::Bass),
            [34 - 14, 37 - 14]
        );
    }

    #[test]
    fn accidentals_last_until_the_bar_line() {
        let mut bar = BarAccidentals::new(KeySignature(1));
        let name = |text| NoteName::parse(text).unwrap();
        // F# is in the key: nothing to print.
        assert_eq!(bar.print(31, name("F#"), false), None);
        // F natural needs a natural, and the next F in the bar does not.
        assert_eq!(bar.print(31, name("F"), false), Some(0));
        assert_eq!(bar.print(31, name("F"), false), None);
        // ...but the F an octave up is on another line.
        assert_eq!(bar.print(38, name("F"), false), Some(0));
        // F# again needs its sharp back.
        assert_eq!(bar.print(31, name("F#"), false), Some(1));
        // A note tied in from the last bar prints nothing.
        let mut next = BarAccidentals::new(KeySignature(1));
        assert_eq!(next.print(30, name("Eb"), true), None);
        assert_eq!(next.print(30, name("Eb"), false), Some(-1));
    }

    #[test]
    fn chords_used_in_tests_exist() {
        assert_eq!(
            ChordSymbol::parse("E7").unwrap().quality,
            ChordQuality::Dominant7
        );
    }
}
