//! Scales and keys: which notes belong together.

use serde::{Deserialize, Serialize};

use super::{Chord, ChordQuality, PitchSet};
use crate::{Note, PitchClass, Spelling};

/// The kinds of scale the piano roll can highlight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScaleKind {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    MajorPentatonic,
    MinorPentatonic,
    Blues,
    Chromatic,
}

impl ScaleKind {
    /// All kinds, in the order the menu lists them.
    pub const ALL: [ScaleKind; 13] = [
        ScaleKind::Major,
        ScaleKind::NaturalMinor,
        ScaleKind::HarmonicMinor,
        ScaleKind::MelodicMinor,
        ScaleKind::Dorian,
        ScaleKind::Phrygian,
        ScaleKind::Lydian,
        ScaleKind::Mixolydian,
        ScaleKind::Locrian,
        ScaleKind::MajorPentatonic,
        ScaleKind::MinorPentatonic,
        ScaleKind::Blues,
        ScaleKind::Chromatic,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ScaleKind::Major => "major",
            ScaleKind::NaturalMinor => "minor",
            ScaleKind::HarmonicMinor => "harmonic minor",
            ScaleKind::MelodicMinor => "melodic minor",
            ScaleKind::Dorian => "Dorian",
            ScaleKind::Phrygian => "Phrygian",
            ScaleKind::Lydian => "Lydian",
            ScaleKind::Mixolydian => "Mixolydian",
            ScaleKind::Locrian => "Locrian",
            ScaleKind::MajorPentatonic => "major pentatonic",
            ScaleKind::MinorPentatonic => "minor pentatonic",
            ScaleKind::Blues => "blues",
            ScaleKind::Chromatic => "chromatic",
        }
    }

    /// The notes of the scale as semitones above its root.
    ///
    /// The major scale is the pattern of the white keys from C:
    /// whole, whole, half, whole, whole, whole, half step.
    pub fn intervals(self) -> &'static [u8] {
        match self {
            ScaleKind::Major => &[0, 2, 4, 5, 7, 9, 11],
            ScaleKind::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
            ScaleKind::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            ScaleKind::MelodicMinor => &[0, 2, 3, 5, 7, 9, 11],
            ScaleKind::Dorian => &[0, 2, 3, 5, 7, 9, 10],
            ScaleKind::Phrygian => &[0, 1, 3, 5, 7, 8, 10],
            ScaleKind::Lydian => &[0, 2, 4, 6, 7, 9, 11],
            ScaleKind::Mixolydian => &[0, 2, 4, 5, 7, 9, 10],
            ScaleKind::Locrian => &[0, 1, 3, 5, 6, 8, 10],
            ScaleKind::MajorPentatonic => &[0, 2, 4, 7, 9],
            ScaleKind::MinorPentatonic => &[0, 3, 5, 7, 10],
            ScaleKind::Blues => &[0, 3, 5, 6, 7, 10],
            ScaleKind::Chromatic => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        }
    }

    /// How far above the root the major key with the same key signature
    /// starts. A minor has no sharps or flats, like C major, which is 3
    /// semitones higher; D Dorian also uses only white keys, and C is 10
    /// semitones above D.
    fn relative_major(self) -> u8 {
        match self {
            ScaleKind::Major | ScaleKind::MajorPentatonic | ScaleKind::Chromatic => 0,
            ScaleKind::NaturalMinor
            | ScaleKind::HarmonicMinor
            | ScaleKind::MelodicMinor
            | ScaleKind::MinorPentatonic
            | ScaleKind::Blues => 3,
            ScaleKind::Dorian => 10,
            ScaleKind::Phrygian => 8,
            ScaleKind::Lydian => 7,
            ScaleKind::Mixolydian => 5,
            ScaleKind::Locrian => 1,
        }
    }
}

/// A scale starting on a particular note: a key, like "A minor".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scale {
    pub root: PitchClass,
    pub kind: ScaleKind,
}

impl Scale {
    pub fn new(root: PitchClass, kind: ScaleKind) -> Self {
        Scale { root, kind }
    }

    /// The pitch classes of the scale, from the root upwards.
    pub fn pitch_classes(self) -> impl Iterator<Item = PitchClass> {
        self.kind
            .intervals()
            .iter()
            .map(move |&interval| self.root.transpose(interval as i32))
    }

    pub fn pitch_set(self) -> PitchSet {
        self.pitch_classes().collect()
    }

    pub fn contains(self, pitch_class: PitchClass) -> bool {
        self.pitch_set().contains(pitch_class)
    }

    /// Which note of the scale this is, counting the root as 0, or `None`
    /// if it is not in the scale. (Musicians count from 1: degree 0 here
    /// is "the first degree".)
    pub fn degree_of(self, pitch_class: PitchClass) -> Option<usize> {
        self.pitch_classes().position(|p| p == pitch_class)
    }

    /// Whether the scale has seven notes, like major, minor and the modes.
    /// Chords in key and Roman numerals only make sense for these.
    pub fn is_heptatonic(self) -> bool {
        self.kind.intervals().len() == 7
    }

    /// Whether this key is written with flats or with sharps.
    ///
    /// Keys whose major key signature has flats (F, Bb, Eb, Ab, Db) use
    /// flats; all others use sharps. This is a simplification: harmonic
    /// minor keys, for example, mix both in real sheet music.
    pub fn spelling(self) -> Spelling {
        use PitchClass::*;
        match self.root.transpose(self.kind.relative_major() as i32) {
            F | ASharp | DSharp | GSharp | CSharp => Spelling::Flats,
            _ => Spelling::Sharps,
        }
    }

    /// The name of the key, such as "Bb major" or "F# minor".
    pub fn name(self) -> String {
        format!("{} {}", self.root.name(self.spelling()), self.kind.name())
    }

    /// Stacks `count` notes of the scale a third apart (every other note),
    /// starting on `note`: C, E, G in C major. Returns `None` if `note` is
    /// not in the scale or the scale does not have seven notes.
    pub fn stack_thirds(self, note: Note, count: usize) -> Option<Vec<Note>> {
        if !self.is_heptatonic() {
            return None;
        }
        let intervals = self.kind.intervals();
        let degree = self.degree_of(note.pitch_class())?;
        let notes = (0..count)
            .map(|i| {
                let index = degree + 2 * i;
                // Going past the seventh note wraps into the next octave.
                let octave = (index / 7) as i32;
                let above_root = intervals[index % 7] as i32 + 12 * octave;
                above_root - intervals[degree] as i32
            })
            .filter_map(|semitones| note.transpose(semitones))
            .collect();
        Some(notes)
    }

    /// The triad (three-note chord) built on a degree of the scale.
    pub fn triad(self, degree: usize) -> Option<Chord> {
        self.chord_on(degree, 3)
    }

    /// The seventh chord (four notes) built on a degree of the scale.
    pub fn seventh(self, degree: usize) -> Option<Chord> {
        self.chord_on(degree, 4)
    }

    fn chord_on(self, degree: usize, count: usize) -> Option<Chord> {
        let root = self.pitch_classes().nth(degree)?;
        let notes = self.stack_thirds(Note::new(60 + root.semitone()), count)?;
        let pitch_set: PitchSet = notes.iter().map(|note| note.pitch_class()).collect();
        let quality = ChordQuality::identify(pitch_set.relative_to(root))?;
        Some(Chord::new(root, quality))
    }

    /// The Roman numeral of a chord in this key, such as "vi" for A minor
    /// in C major or "V7" for G7. Upper case means a major third, lower
    /// case a minor third. Returns `None` if the chord's root is not in
    /// the scale or the scale does not have seven notes.
    pub fn roman_numeral(self, chord: Chord) -> Option<String> {
        const NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
        if !self.is_heptatonic() {
            return None;
        }
        let degree = self.degree_of(chord.root)?;
        let numeral = if chord.quality.is_minor() {
            NUMERALS[degree].to_lowercase()
        } else {
            NUMERALS[degree].to_string()
        };
        Some(format!("{numeral}{}", chord.quality.numeral_suffix()))
    }

    /// Whether every note of the chord is in the scale.
    pub fn is_diatonic(self, chord: Chord) -> bool {
        chord.pitch_set().is_subset_of(self.pitch_set())
    }
}

impl Default for Scale {
    /// C major: all white keys.
    fn default() -> Self {
        Scale::new(PitchClass::C, ScaleKind::Major)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PitchClass::*;

    fn names(scale: Scale) -> Vec<&'static str> {
        let spelling = scale.spelling();
        scale.pitch_classes().map(|p| p.name(spelling)).collect()
    }

    #[test]
    fn c_major_is_the_white_keys() {
        let scale = Scale::default();
        assert_eq!(names(scale), ["C", "D", "E", "F", "G", "A", "B"]);
        assert!(
            PitchClass::ALL
                .iter()
                .all(|p| scale.contains(*p) != p.is_black_key())
        );
    }

    #[test]
    fn keys_are_spelled_with_sharps_or_flats() {
        assert_eq!(
            names(Scale::new(D, ScaleKind::Major)),
            ["D", "E", "F#", "G", "A", "B", "C#"]
        );
        assert_eq!(
            names(Scale::new(F, ScaleKind::Major)),
            ["F", "G", "A", "Bb", "C", "D", "E"]
        );
        assert_eq!(
            names(Scale::new(C, ScaleKind::NaturalMinor)),
            ["C", "D", "Eb", "F", "G", "Ab", "Bb"]
        );
        assert_eq!(Scale::new(ASharp, ScaleKind::Major).name(), "Bb major");
        assert_eq!(
            Scale::new(FSharp, ScaleKind::NaturalMinor).name(),
            "F# minor"
        );
    }

    #[test]
    fn modes_share_notes_with_their_relative_major() {
        let c_major = Scale::default().pitch_set();
        assert_eq!(Scale::new(D, ScaleKind::Dorian).pitch_set(), c_major);
        assert_eq!(Scale::new(G, ScaleKind::Mixolydian).pitch_set(), c_major);
        assert_eq!(Scale::new(A, ScaleKind::NaturalMinor).pitch_set(), c_major);
    }

    #[test]
    fn degrees() {
        let scale = Scale::new(A, ScaleKind::NaturalMinor);
        assert_eq!(scale.degree_of(A), Some(0));
        assert_eq!(scale.degree_of(E), Some(4));
        assert_eq!(scale.degree_of(FSharp), None);
    }

    #[test]
    fn the_chords_of_c_major() {
        let scale = Scale::default();
        let triads: Vec<String> = (0..7)
            .map(|degree| scale.triad(degree).unwrap().name(Spelling::Sharps))
            .collect();
        assert_eq!(triads, ["C", "Dm", "Em", "F", "G", "Am", "Bdim"]);

        let numerals: Vec<String> = (0..7)
            .map(|degree| scale.roman_numeral(scale.triad(degree).unwrap()).unwrap())
            .collect();
        assert_eq!(numerals, ["I", "ii", "iii", "IV", "V", "vi", "vii°"]);

        let sevenths: Vec<String> = (0..7)
            .map(|degree| scale.seventh(degree).unwrap().name(Spelling::Sharps))
            .collect();
        assert_eq!(
            sevenths,
            ["Cmaj7", "Dm7", "Em7", "Fmaj7", "G7", "Am7", "Bm7b5"]
        );
        assert_eq!(
            scale.roman_numeral(scale.seventh(4).unwrap()).unwrap(),
            "V7"
        );
        assert_eq!(
            scale.roman_numeral(scale.seventh(6).unwrap()).unwrap(),
            "viiø7"
        );
    }

    #[test]
    fn harmonic_minor_has_a_major_five() {
        let scale = Scale::new(A, ScaleKind::HarmonicMinor);
        let five = scale.triad(4).unwrap();
        assert_eq!(five, Chord::new(E, ChordQuality::Major));
        assert_eq!(scale.roman_numeral(five).unwrap(), "V");
        assert_eq!(scale.seventh(6).unwrap().quality, ChordQuality::Diminished7);
    }

    #[test]
    fn every_seven_note_scale_has_named_chords() {
        // If a scale built a chord `ChordQuality` does not know, the
        // chords-in-key strip would have a hole in it.
        for kind in ScaleKind::ALL {
            let scale = Scale::new(C, kind);
            if !scale.is_heptatonic() {
                assert!(scale.triad(0).is_none());
                continue;
            }
            for degree in 0..7 {
                assert!(scale.triad(degree).is_some(), "{kind:?} triad {degree}");
                assert!(scale.seventh(degree).is_some(), "{kind:?} seventh {degree}");
            }
        }
    }

    #[test]
    fn stacking_thirds_wraps_into_the_next_octave() {
        let scale = Scale::default();
        let b3 = Note::new(59);
        let notes: Vec<u8> = scale
            .stack_thirds(b3, 4)
            .unwrap()
            .iter()
            .map(|n| n.midi())
            .collect();
        // B3, D4, F4, A4
        assert_eq!(notes, vec![59, 62, 65, 69]);
        assert!(
            scale.stack_thirds(Note::new(61), 3).is_none(),
            "C# is not in C major"
        );
    }

    #[test]
    fn diatonic_or_borrowed() {
        let scale = Scale::default();
        assert!(scale.is_diatonic(Chord::new(A, ChordQuality::Minor)));
        assert!(!scale.is_diatonic(Chord::new(E, ChordQuality::Major)));
        assert_eq!(
            scale
                .roman_numeral(Chord::new(E, ChordQuality::Major))
                .unwrap(),
            "III"
        );
        assert_eq!(
            scale.roman_numeral(Chord::new(ASharp, ChordQuality::Major)),
            None
        );
    }
}
