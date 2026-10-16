//! Notes, pitch classes and their names.

use std::fmt;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(u8);

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
}
