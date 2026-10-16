//! Notes and their frequencies.

use std::fmt;

/// The names of the twelve pitch classes, starting at C.
const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

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

    /// The pitch class, from 0 (C) to 11 (B), ignoring the octave.
    pub fn pitch_class(self) -> u8 {
        self.0 % 12
    }

    /// The octave number, using the convention where middle C is C4.
    pub fn octave(self) -> i8 {
        (self.0 / 12) as i8 - 1
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
        let name = NOTE_NAMES[self.pitch_class() as usize];
        write!(f, "{}{}", name, self.octave())
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
        assert_eq!(Note::MIN.to_string(), "C-1");
        assert_eq!(Note::MAX.to_string(), "G9");
    }

    #[test]
    fn new_clamps_out_of_range_numbers() {
        assert_eq!(Note::new(200), Note::MAX);
    }
}
