//! Chords written as Roman numerals, the way musicians write progressions
//! that work in any key: "I V vi IV", "ii7 V7 Imaj7", "i bVII bVI V".
//!
//! The numeral says which note of the key the chord is built on, and its
//! case says whether the chord is major (upper case) or minor (lower
//! case). What follows says the rest: "7", "maj7", "°", "sus4"...
//!
//! Numerals count along the **major** scale of the tonic, whatever the
//! key: III is always four semitones above the tonic, and a flat in front
//! lowers it by one. So the chords of a minor key are written i, ii°,
//! bIII, iv, v, bVI, bVII. This is how most pop and jazz books write
//! them, and it means one table of progressions works for every mode.

use crate::harmony::{Chord, ChordQuality};
use crate::pitch::{PitchClass, Spelling};

/// A chord relative to the tonic of a key: so many semitones above it,
/// with a quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RomanChord {
    /// Semitones above the tonic, 0 to 11.
    pub semitones: u8,
    pub quality: ChordQuality,
    /// How to spell the chord's root, if the numeral says: a flat in
    /// front ("bII7") means it is written with flats, Db7 in C, not C#7.
    /// `None` means as the key is written.
    pub spelling: Option<Spelling>,
}

impl RomanChord {
    /// The chord in the key whose tonic is `tonic`.
    pub fn in_key(self, tonic: PitchClass) -> Chord {
        Chord::new(tonic.transpose(self.semitones as i32), self.quality)
    }
}

/// The seven numerals, and how far above the tonic each one is in the
/// major scale. Longer numerals come first, so "III" is not read as "I"
/// followed by "II".
const NUMERALS: [(&str, u8); 7] = [
    ("III", 4),
    ("VII", 11),
    ("II", 2),
    ("IV", 5),
    ("VI", 9),
    ("I", 0),
    ("V", 7),
];

/// The qualities a suffix can give, for an upper-case (major) numeral
/// and a lower-case (minor) one. `None` means that suffix does not go
/// with that case.
const SUFFIXES: [(&str, Option<ChordQuality>, Option<ChordQuality>); 20] = {
    use ChordQuality::*;
    [
        ("", Some(Major), Some(Minor)),
        ("7", Some(Dominant7), Some(Minor7)),
        ("maj7", Some(Major7), Some(MinorMajor7)),
        ("6", Some(Major6), Some(Minor6)),
        ("69", Some(Six9), None),
        ("9", Some(Dominant9), Some(Minor9)),
        ("maj9", Some(Major9), None),
        ("11", Some(Dominant11), Some(Minor11)),
        ("13", Some(Dominant13), None),
        ("maj13", Some(Major13), None),
        ("add9", Some(Add9), Some(MinorAdd9)),
        ("sus2", Some(Sus2), None),
        ("sus4", Some(Sus4), None),
        ("7sus4", Some(Dominant7Sus4), None),
        ("7b9", Some(Dominant7Flat9), None),
        ("7#9", Some(Dominant7Sharp9), None),
        ("+", Some(Augmented), None),
        ("°", None, Some(Diminished)),
        ("°7", None, Some(Diminished7)),
        ("ø7", None, Some(HalfDiminished7)),
    ]
};

/// Reads one chord, such as "bVII", "ii7" or "V7b9". Returns `None` if
/// it is not a chord this module knows how to write.
pub fn parse(text: &str) -> Option<RomanChord> {
    // Flats and sharps in front move the chord down or up a semitone.
    let mut shift: i32 = 0;
    let mut rest = text;
    loop {
        if let Some(after) = rest.strip_prefix('b') {
            shift -= 1;
            rest = after;
        } else if let Some(after) = rest.strip_prefix('#') {
            shift += 1;
            rest = after;
        } else {
            break;
        }
    }
    // The numeral, in either case. `get(..len)` gives `None` instead of
    // panicking when `rest` is shorter than the numeral.
    let (numeral, semitones) = NUMERALS.iter().find(|(numeral, _)| {
        rest.get(..numeral.len())
            .is_some_and(|start| start.eq_ignore_ascii_case(numeral))
    })?;
    let written = &rest[..numeral.len()];
    let upper = written == *numeral;
    if !upper && written != numeral.to_lowercase() {
        return None; // mixed case, like "Iv"
    }
    let suffix = &rest[numeral.len()..];
    let (_, major, minor) = SUFFIXES.iter().find(|(text, ..)| *text == suffix)?;
    let quality = if upper { *major } else { *minor }?;
    let semitones = (*semitones as i32 + shift).rem_euclid(12) as u8;
    let spelling = match shift {
        0 => None,
        down if down < 0 => Some(Spelling::Flats),
        _ => Some(Spelling::Sharps),
    };
    Some(RomanChord {
        semitones,
        quality,
        spelling,
    })
}

/// Reads a progression: chords separated by spaces. Returns `None` if
/// any of them cannot be read.
pub fn parse_progression(text: &str) -> Option<Vec<RomanChord>> {
    text.split_whitespace().map(parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ChordQuality::*;

    fn chord(text: &str) -> (u8, ChordQuality) {
        let chord = parse(text).unwrap_or_else(|| panic!("{text} did not parse"));
        (chord.semitones, chord.quality)
    }

    #[test]
    fn case_says_major_or_minor() {
        assert_eq!(chord("I"), (0, Major));
        assert_eq!(chord("vi"), (9, Minor));
        assert_eq!(chord("IV"), (5, Major));
        assert_eq!(chord("iii"), (4, Minor));
        assert_eq!(chord("vii°"), (11, Diminished));
    }

    #[test]
    fn flats_and_sharps_move_the_root() {
        assert_eq!(chord("bVII"), (10, Major));
        assert_eq!(chord("bIII"), (3, Major));
        assert_eq!(chord("#iv°"), (6, Diminished));
        assert_eq!(chord("bII7"), (1, Dominant7));
    }

    #[test]
    fn suffixes_depend_on_the_case() {
        assert_eq!(chord("V7"), (7, Dominant7));
        assert_eq!(chord("ii7"), (2, Minor7));
        assert_eq!(chord("Imaj7"), (0, Major7));
        assert_eq!(chord("iiø7"), (2, HalfDiminished7));
        assert_eq!(chord("V7b9"), (7, Dominant7Flat9));
        assert_eq!(chord("I69"), (0, Six9));
        assert_eq!(chord("vi9"), (9, Minor9));
    }

    #[test]
    fn nonsense_is_refused() {
        for text in ["", "X", "Iv", "Imaj", "ii°°", "vmaj9", "I7x", "bb"] {
            assert_eq!(parse(text), None, "{text}");
        }
    }

    #[test]
    fn a_progression_in_a_key() {
        let chords = parse_progression("i bVII bVI V").unwrap();
        let names: Vec<String> = chords
            .iter()
            .map(|c| c.in_key(PitchClass::A).name(crate::Spelling::Sharps))
            .collect();
        assert_eq!(names, ["Am", "G", "F", "E"]);
        assert_eq!(parse_progression("I V nope"), None);
    }
}
