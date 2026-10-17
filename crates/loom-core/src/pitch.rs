//! Notes, pitch classes and their names.

use std::fmt;

use serde::{Deserialize, Serialize};

/// How to name the black keys: as sharps (C#) or as flats (Db).
///
/// The same key has two names, and which one is right depends on the
/// key you are in: F major has a B flat, never an A sharp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Spelling {
    #[default]
    Sharps,
    Flats,
}

/// One of the twelve notes of the octave, ignoring which octave it is in.
///
/// Every C on the piano has pitch class C, whether it is the lowest key
/// or the highest. Harmony mostly works with pitch classes: a C major
/// chord is "C, E and G", in any octave and any order.
///
/// The variants are listed from C upwards, so `PitchClass::D as u8` is 2,
/// the number of semitones above C.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PitchClass {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl PitchClass {
    /// All twelve, from C upwards.
    pub const ALL: [PitchClass; 12] = [
        PitchClass::C,
        PitchClass::CSharp,
        PitchClass::D,
        PitchClass::DSharp,
        PitchClass::E,
        PitchClass::F,
        PitchClass::FSharp,
        PitchClass::G,
        PitchClass::GSharp,
        PitchClass::A,
        PitchClass::ASharp,
        PitchClass::B,
    ];

    /// The pitch class `semitone` semitones above C (wrapping every 12).
    pub fn from_semitone(semitone: u8) -> Self {
        Self::ALL[(semitone % 12) as usize]
    }

    /// Semitones above C, from 0 to 11.
    pub fn semitone(self) -> u8 {
        self as u8
    }

    /// Moves up (or down, if negative) by some semitones, wrapping around the octave.
    pub fn transpose(self, semitones: i32) -> Self {
        let wrapped = (self.semitone() as i32 + semitones).rem_euclid(12);
        Self::from_semitone(wrapped as u8)
    }

    /// How many semitones you go up from `self` to reach `other`, from 0 to 11.
    pub fn semitones_up_to(self, other: PitchClass) -> u8 {
        (other.semitone() + 12 - self.semitone()) % 12
    }

    /// Whether this is one of the five black keys of the piano.
    pub fn is_black_key(self) -> bool {
        use PitchClass::*;
        matches!(self, CSharp | DSharp | FSharp | GSharp | ASharp)
    }

    /// The name, such as `"C"`, `"F#"` or `"Gb"`.
    pub fn name(self, spelling: Spelling) -> &'static str {
        use PitchClass::*;
        use Spelling::*;
        match (self, spelling) {
            (C, _) => "C",
            (CSharp, Sharps) => "C#",
            (CSharp, Flats) => "Db",
            (D, _) => "D",
            (DSharp, Sharps) => "D#",
            (DSharp, Flats) => "Eb",
            (E, _) => "E",
            (F, _) => "F",
            (FSharp, Sharps) => "F#",
            (FSharp, Flats) => "Gb",
            (G, _) => "G",
            (GSharp, Sharps) => "G#",
            (GSharp, Flats) => "Ab",
            (A, _) => "A",
            (ASharp, Sharps) => "A#",
            (ASharp, Flats) => "Bb",
            (B, _) => "B",
        }
    }
}

impl fmt::Display for PitchClass {
    /// Writes the sharp name. Use [`PitchClass::name`] to choose flats.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name(Spelling::Sharps))
    }
}

/// A musical note, stored as a MIDI note number (0..=127).
///
/// MIDI note 60 is middle C (C4) and MIDI note 69 is A4, the 440 Hz
/// tuning note. Wrapping the number in its own type (a "newtype") means
/// the compiler will not let us mix up a note with, say, a velocity.
///
/// In a project file a note is just its MIDI number. Reading one goes
/// through `From<u8>`, which calls [`Note::new`], so a file cannot
/// create a note above 127.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub struct Note(u8);

impl From<u8> for Note {
    fn from(midi: u8) -> Self {
        Note::new(midi)
    }
}

impl From<Note> for u8 {
    fn from(note: Note) -> Self {
        note.0
    }
}

impl Note {
    /// The lowest MIDI note.
    pub const MIN: Note = Note(0);
    /// The highest MIDI note.
    pub const MAX: Note = Note(127);
    /// Middle C.
    pub const C4: Note = Note(60);
    /// The A above middle C, tuned to 440 Hz.
    pub const A4: Note = Note(69);

    /// Creates a note from a MIDI number, clamping it to 0..=127.
    pub fn new(midi: u8) -> Self {
        Note(midi.min(Self::MAX.0))
    }

    /// The MIDI note number.
    pub fn midi(self) -> u8 {
        self.0
    }

    /// The pitch class (C, C#, D, ...), ignoring the octave.
    pub fn pitch_class(self) -> PitchClass {
        PitchClass::from_semitone(self.0)
    }

    /// The octave number, using the convention where middle C is C4.
    pub fn octave(self) -> i8 {
        (self.0 / 12) as i8 - 1
    }

    /// The note some semitones higher (or lower, if negative), or `None`
    /// if that would fall outside the MIDI range.
    pub fn transpose(self, semitones: i32) -> Option<Note> {
        let midi = self.0 as i32 + semitones;
        (0..=Self::MAX.0 as i32)
            .contains(&midi)
            .then_some(Note(midi as u8))
    }

    /// The name and octave, such as `"Db4"` or `"C#4"`.
    pub fn name(self, spelling: Spelling) -> String {
        format!("{}{}", self.pitch_class().name(spelling), self.octave())
    }

    /// The note written as `name` (which must name its pitch class),
    /// with the octave of its letter: middle C written as B# is "B#3",
    /// since B#3 is the B of the octave below raised a semitone, and the
    /// B below middle C written as Cb is "Cb4".
    pub fn name_as(self, name: NoteName) -> String {
        let natural =
            i32::from(self.0) - i32::from(name.letter.semitone()) - i32::from(name.accidental);
        format!("{name}{}", natural.div_euclid(12) - 1)
    }

    /// The frequency in hertz, in twelve-tone equal temperament with A4 = 440 Hz.
    ///
    /// Each semitone multiplies the frequency by the twelfth root of two,
    /// so twelve semitones (one octave) exactly double it.
    pub fn frequency(self) -> f32 {
        let semitones_from_a4 = self.0 as f32 - Self::A4.0 as f32;
        440.0 * 2.0_f32.powf(semitones_from_a4 / 12.0)
    }
}

impl fmt::Display for Note {
    /// Writes the note as a name and octave, such as `C#4`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class(), self.octave())
    }
}

/// One of the seven letters that name notes, from C to B: the white keys.
///
/// Every scale of seven notes uses each letter once. That is why the
/// seventh note of F# major is called E#, not F: F# already has the
/// letter F.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Letter {
    /// All seven, from C upwards.
    pub const ALL: [Letter; 7] = [
        Letter::C,
        Letter::D,
        Letter::E,
        Letter::F,
        Letter::G,
        Letter::A,
        Letter::B,
    ];

    /// The semitones above C of the white key with this letter: 0 for C,
    /// 2 for D, 4 for E, 5 for F...
    pub fn semitone(self) -> u8 {
        [0, 2, 4, 5, 7, 9, 11][self as usize]
    }

    /// The letter `steps` letters higher (or lower, if negative), going
    /// round from B back to C: two letters above A is C.
    pub fn up(self, steps: i32) -> Letter {
        Self::ALL[(self as i32 + steps).rem_euclid(7) as usize]
    }

    /// How many letters `other` is above `self`, from 0 to 6: from A to
    /// C is 2 (A, B, C).
    pub fn steps_up_to(self, other: Letter) -> u8 {
        (other as u8 + 7 - self as u8) % 7
    }

    pub fn as_char(self) -> char {
        ['C', 'D', 'E', 'F', 'G', 'A', 'B'][self as usize]
    }
}

/// The name of a note as musicians write it: a letter, and how many
/// sharps (above zero) or flats (below zero) it has.
///
/// The same key of the piano has several names, and which one is right
/// depends on the music around it. The black key between F and G is F#
/// in D major (F# is its third note) but Gb in Db major. Even a white key
/// can take another name: in F# major the seventh note is E#, the key
/// usually called F, and in Gb major the fourth note is Cb, the key
/// usually called B. Now and then a note needs two sharps (F## in G#
/// harmonic minor) or two flats (the Bbb of a C diminished seventh).
///
/// [`PitchClass`] is the key; `NoteName` is what it is called.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteName {
    pub letter: Letter,
    /// Sharps if above zero, flats if below: 1 is "#", -2 is "bb".
    pub accidental: i8,
}

impl NoteName {
    pub const fn new(letter: Letter, accidental: i8) -> Self {
        NoteName { letter, accidental }
    }

    /// The name of `pitch_class` written with `letter`: B with the letter
    /// C is Cb, and F with the letter E is E#.
    pub fn with_letter(pitch_class: PitchClass, letter: Letter) -> Self {
        let up = (pitch_class.semitone() + 12 - letter.semitone()) % 12;
        // Up to five semitones up is sharps; six or more is flats below.
        let accidental = if up <= 5 { up as i8 } else { up as i8 - 12 };
        NoteName::new(letter, accidental)
    }

    /// The everyday name, with sharps or flats for the black keys, as
    /// [`PitchClass::name`] writes it.
    pub fn plain(pitch_class: PitchClass, spelling: Spelling) -> Self {
        let letter = match pitch_class.name(spelling).chars().next() {
            Some('D') => Letter::D,
            Some('E') => Letter::E,
            Some('F') => Letter::F,
            Some('G') => Letter::G,
            Some('A') => Letter::A,
            Some('B') => Letter::B,
            _ => Letter::C,
        };
        NoteName::with_letter(pitch_class, letter)
    }

    /// The name most used when there is no key to follow, as in
    /// twelve-tone music: C C# D Eb E F F# G Ab A Bb B.
    pub fn chromatic(pitch_class: PitchClass) -> Self {
        let spelling = match pitch_class {
            PitchClass::CSharp | PitchClass::FSharp => Spelling::Sharps,
            _ => Spelling::Flats,
        };
        NoteName::plain(pitch_class, spelling)
    }

    /// The key of the piano it names.
    pub fn pitch_class(self) -> PitchClass {
        PitchClass::from_semitone(self.letter.semitone()).transpose(i32::from(self.accidental))
    }

    /// The note `semitones` semitones and `letters` letters above this
    /// one. A major third is four semitones and two letters: above C it
    /// is E, above A it is C#, and above Eb it is G.
    pub fn above(self, semitones: u8, letters: u8) -> NoteName {
        let pitch_class = self.pitch_class().transpose(i32::from(semitones));
        NoteName::with_letter(pitch_class, self.letter.up(i32::from(letters)))
    }

    /// The same key with one sharp or flat at most: Ebb becomes D and
    /// F## becomes G. For a chord outside the key, whose strict name
    /// would need two accidentals, the simpler name is easier to read.
    pub fn simplified(self) -> NoteName {
        if self.accidental.abs() < 2 {
            return self;
        }
        let spelling = if self.accidental > 0 {
            Spelling::Sharps
        } else {
            Spelling::Flats
        };
        NoteName::plain(self.pitch_class(), spelling)
    }
}

impl fmt::Display for NoteName {
    /// Writes the letter and its sharps or flats: "C", "F#", "Bb", "F##".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.accidental > 0 { "#" } else { "b" };
        write!(
            f,
            "{}{}",
            self.letter.as_char(),
            sign.repeat(self.accidental.unsigned_abs() as usize)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4_is_440_hz() {
        assert_eq!(Note::A4.frequency(), 440.0);
    }

    #[test]
    fn middle_c_frequency() {
        let freq = Note::C4.frequency();
        assert!((freq - 261.626).abs() < 0.01, "got {freq}");
    }

    #[test]
    fn an_octave_doubles_the_frequency() {
        let low = Note::new(57).frequency();
        let high = Note::new(69).frequency();
        assert!((high / low - 2.0).abs() < 1e-5);
    }

    #[test]
    fn names_and_octaves() {
        assert_eq!(Note::C4.to_string(), "C4");
        assert_eq!(Note::A4.to_string(), "A4");
        assert_eq!(Note::new(61).to_string(), "C#4");
        assert_eq!(Note::new(61).name(Spelling::Flats), "Db4");
        assert_eq!(Note::MIN.to_string(), "C-1");
        assert_eq!(Note::MAX.to_string(), "G9");
    }

    #[test]
    fn new_clamps_out_of_range_numbers() {
        assert_eq!(Note::new(200), Note::MAX);
    }

    #[test]
    fn transposing_notes() {
        assert_eq!(Note::C4.transpose(7), Some(Note::new(67)));
        assert_eq!(Note::C4.transpose(-12), Some(Note::new(48)));
        assert_eq!(Note::MAX.transpose(1), None);
        assert_eq!(Note::MIN.transpose(-1), None);
    }

    #[test]
    fn pitch_classes_wrap_around_the_octave() {
        assert_eq!(Note::A4.pitch_class(), PitchClass::A);
        assert_eq!(PitchClass::B.transpose(1), PitchClass::C);
        assert_eq!(PitchClass::C.transpose(-1), PitchClass::B);
        assert_eq!(PitchClass::A.semitones_up_to(PitchClass::C), 3);
        assert_eq!(PitchClass::C.semitones_up_to(PitchClass::A), 9);
        assert!(PitchClass::FSharp.is_black_key());
        assert!(!PitchClass::E.is_black_key());
    }

    #[test]
    fn every_pitch_class_has_a_number_matching_its_position() {
        for (i, pc) in PitchClass::ALL.iter().enumerate() {
            assert_eq!(pc.semitone() as usize, i);
            assert_eq!(PitchClass::from_semitone(i as u8), *pc);
        }
    }

    #[test]
    fn letters_go_round() {
        assert_eq!(Letter::A.up(2), Letter::C);
        assert_eq!(Letter::C.up(-1), Letter::B);
        assert_eq!(Letter::A.steps_up_to(Letter::C), 2);
        assert_eq!(Letter::F.semitone(), 5);
    }

    #[test]
    fn note_names_with_any_letter() {
        use PitchClass::*;
        let name = |pc, letter| NoteName::with_letter(pc, letter).to_string();
        assert_eq!(name(F, Letter::E), "E#");
        assert_eq!(name(B, Letter::C), "Cb");
        assert_eq!(name(G, Letter::F), "F##");
        assert_eq!(name(A, Letter::B), "Bbb");
        assert_eq!(name(FSharp, Letter::G), "Gb");
        assert_eq!(NoteName::with_letter(F, Letter::E).pitch_class(), F);
        assert_eq!(NoteName::plain(ASharp, Spelling::Flats).to_string(), "Bb");
        let chromatic: Vec<String> = PitchClass::ALL
            .iter()
            .map(|&pc| NoteName::chromatic(pc).to_string())
            .collect();
        assert_eq!(chromatic.join(" "), "C C# D Eb E F F# G Ab A Bb B");
    }

    #[test]
    fn a_spelled_note_keeps_the_octave_of_its_letter() {
        let c4 = Note::new(60);
        assert_eq!(c4.name_as(NoteName::new(Letter::C, 0)), "C4");
        assert_eq!(c4.name_as(NoteName::new(Letter::B, 1)), "B#3");
        assert_eq!(Note::new(59).name_as(NoteName::new(Letter::C, -1)), "Cb4");
        assert_eq!(Note::new(61).name_as(NoteName::new(Letter::D, -1)), "Db4");
    }

    #[test]
    fn intervals_are_counted_in_letters_too() {
        let a = NoteName::new(Letter::A, 0);
        // A major third above A is C#, not Db.
        assert_eq!(a.above(4, 2).to_string(), "C#");
        let b = NoteName::new(Letter::B, 0);
        // The diminished seventh above B is Ab; above C it is Bbb.
        assert_eq!(b.above(9, 6).to_string(), "Ab");
        assert_eq!(NoteName::new(Letter::C, 0).above(9, 6).to_string(), "Bbb");
        assert_eq!(NoteName::new(Letter::E, -2).simplified().to_string(), "D");
        assert_eq!(NoteName::new(Letter::F, 2).simplified().to_string(), "G");
    }
}
