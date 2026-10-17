//! Chord symbols, the way lead sheets write them: "Am7", "F#m7b5",
//! "Bb13", "G/B".
//!
//! A chord symbol has up to three parts:
//!
//! - the **root**, a letter with maybe a sharp or a flat: the "F#" of
//!   "F#m7b5";
//! - the **quality**, what kind of chord it is: the "m7b5";
//! - a **bass note** after a slash, when the lowest note is not the
//!   root: "G/B" is G major with B in the bass.
//!
//! Musicians write the same quality in several ways. A minor seventh is
//! "m7", "min7" or "-7"; a major seventh "maj7", "M7" or "Δ7". All of
//! them are read here, and written back in one way, the one
//! [`ChordQuality::symbol`] uses.
//!
//! A slash chord whose bass note is in the chord is an **inversion**:
//! C/E is C major with its third in the bass. When the bass note is not
//! in the chord, as in F/G, it is a chord over a bass note, a sound of
//! its own: F/G is often used instead of G11.

use std::fmt;
use std::str::FromStr;

use super::{Chord, ChordQuality};
use crate::pitch::{NoteName, PitchClass};

/// A chord as a lead sheet writes it: a root, a quality and maybe a
/// bass note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChordSymbol {
    /// The root, spelled as written: Db7 keeps its Db, it is not C#7.
    pub root: NoteName,
    pub quality: ChordQuality,
    /// The note under the chord, after the slash: the B of "G/B". `None`
    /// when the root is in the bass, as usual.
    pub bass: Option<NoteName>,
}

/// The ways of writing each quality that are read, after taking out
/// spaces and brackets: "m(maj7)" is read as "mmaj7" and "7(b9)" as
/// "7b9". Upper and lower case matter: "M7" is a major seventh and "m7"
/// a minor seventh.
const SPELLINGS: [(ChordQuality, &[&str]); 30] = {
    use ChordQuality::*;
    [
        (Major, &["", "maj", "M"]),
        (Minor, &["m", "min", "mi", "-"]),
        (Diminished, &["dim", "°", "o"]),
        (Augmented, &["aug", "+", "+5"]),
        (Sus2, &["sus2"]),
        (Sus4, &["sus4", "sus"]),
        (Major6, &["6", "maj6", "M6"]),
        (Minor6, &["m6", "min6", "mi6", "-6"]),
        (Dominant7, &["7", "dom7"]),
        (Major7, &["maj7", "M7", "ma7", "Δ", "Δ7"]),
        (Minor7, &["m7", "min7", "mi7", "-7"]),
        (
            MinorMajor7,
            &["mmaj7", "mM7", "minmaj7", "-maj7", "mΔ7", "-Δ7"],
        ),
        (
            HalfDiminished7,
            &["m7b5", "min7b5", "mi7b5", "-7b5", "ø", "ø7", "Ø", "Ø7"],
        ),
        (Diminished7, &["dim7", "°7", "o7"]),
        (
            AugmentedMajor7,
            &["maj7#5", "M7#5", "maj7+5", "+maj7", "Δ#5", "Δ7#5"],
        ),
        (Dominant7Sus4, &["7sus4", "7sus"]),
        (Dominant7Flat5, &["7b5", "7-5"]),
        (Augmented7, &["7#5", "7+5", "7+", "+7", "aug7"]),
        (Add9, &["add9"]),
        (MinorAdd9, &["madd9", "minadd9", "-add9"]),
        (Dominant9, &["9"]),
        (Major9, &["maj9", "M9", "Δ9"]),
        (Minor9, &["m9", "min9", "mi9", "-9"]),
        (Six9, &["6/9", "69"]),
        (Dominant7Flat9, &["7b9", "7-9"]),
        (Dominant7Sharp9, &["7#9", "7+9"]),
        (Dominant11, &["11"]),
        (Minor11, &["m11", "min11", "mi11", "-11"]),
        (Dominant13, &["13"]),
        (Major13, &["maj13", "M13", "Δ13"]),
    ]
};

/// The quality written as `text`, in any of its [`SPELLINGS`].
fn read_quality(text: &str) -> Option<ChordQuality> {
    let plain: String = text
        .chars()
        .filter(|c| !matches!(c, '(' | ')' | ' '))
        .map(|c| match c {
            '♭' => 'b',
            '♯' => '#',
            other => other,
        })
        .collect();
    SPELLINGS
        .iter()
        .find(|(_, spellings)| spellings.contains(&plain.as_str()))
        .map(|(quality, _)| *quality)
}

impl ChordSymbol {
    pub fn new(root: NoteName, quality: ChordQuality) -> Self {
        ChordSymbol {
            root,
            quality,
            bass: None,
        }
    }

    /// The same chord with `bass` under it. A bass note that is the root
    /// itself is no slash chord: C/C is C.
    pub fn over(self, bass: NoteName) -> Self {
        let bass = (bass.pitch_class() != self.root.pitch_class()).then_some(bass);
        ChordSymbol { bass, ..self }
    }

    /// Reads a chord symbol, such as "Am7", "F#m7b5", "Bb13", "C6/9" or
    /// "G/B". The root is a capital letter; a lower-case "b" after it is
    /// a flat.
    ///
    /// A slash is a bass note only when a note name follows it: in
    /// "C6/9" the slash belongs to the six-nine chord, and in "C6/9/E"
    /// the last one puts E in the bass.
    pub fn parse(text: &str) -> Result<ChordSymbol, String> {
        let text = text.trim();
        let Some((root, rest)) = NoteName::read(text) else {
            return Err(format!(
                "\"{text}\" does not start with a note: chords start with a letter from A to G, \
                 like C, F#m or Bb7."
            ));
        };
        let (quality_text, bass) = match rest.rsplit_once('/') {
            Some((before, after)) => match NoteName::parse(after.trim()) {
                Some(bass) => (before, Some(bass)),
                None => (rest, None),
            },
            None => (rest, None),
        };
        let Some(quality) = read_quality(quality_text) else {
            return Err(format!(
                "\"{text}\": \"{quality_text}\" is not a kind of chord Ferrous Loom knows, \
                 such as m, 7, maj7, m7b5, sus4 or 13."
            ));
        };
        let chord = ChordSymbol::new(root, quality);
        Ok(match bass {
            Some(bass) => chord.over(bass),
            None => chord,
        })
    }

    /// The chord it names, to play or to look up in a key.
    pub fn chord(self) -> Chord {
        Chord::new(self.root.pitch_class(), self.quality)
    }

    /// The lowest note's pitch class: the bass note after the slash, or
    /// else the root.
    pub fn bass_pitch_class(self) -> PitchClass {
        self.bass.unwrap_or(self.root).pitch_class()
    }

    /// The chord's notes, spelled up from its root: F#m7b5 is F# A C E,
    /// and Db7 is Db F Ab Cb, since a seventh above D is a C.
    pub fn notes(self) -> Vec<NoteName> {
        self.chord().spell(self.root)
    }

    /// Which note of the chord is in the bass, counted from the root as
    /// [`ChordQuality::tones`] counts: 3 for C/E (the first inversion),
    /// 5 for C/G (the second), 7 for G7/F (the third). `None` when the
    /// root is in the bass, or when the bass note is not in the chord, as
    /// in F/G.
    pub fn bass_in_chord(self) -> Option<u8> {
        let bass = self.bass?.pitch_class();
        self.notes()
            .iter()
            .zip(self.quality.tones())
            .find(|(name, _)| name.pitch_class() == bass)
            .map(|(_, &tone)| tone)
    }

    /// The same chord moved by the distance from the key `from` to the
    /// key `to`, counted in letters as well as semitones, the way a
    /// musician transposes a chart. From E to D every letter goes down
    /// one: F#m7b5 B7 Em becomes Em7b5 A7 Dm.
    ///
    /// Chord symbols never have two sharps or flats on one note, so a
    /// root or bass that would need them takes its simpler name: D#°7 in
    /// E minor becomes G°7 in G# minor, where a score would write F##°7.
    pub fn transposed(self, from: NoteName, to: NoteName) -> ChordSymbol {
        let semitones = from.pitch_class().semitones_up_to(to.pitch_class());
        let letters = from.letter.steps_up_to(to.letter);
        let shift = |name: NoteName| name.above(semitones, letters).simplified();
        ChordSymbol {
            root: shift(self.root),
            quality: self.quality,
            bass: self.bass.map(shift),
        }
    }
}

impl fmt::Display for ChordSymbol {
    /// Writes the symbol in one way for each quality: "Am7", "Bbmaj7",
    /// "F#m7b5", "C6/9", "G/B".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root, self.quality.symbol())?;
        match self.bass {
            Some(bass) => write!(f, "/{bass}"),
            None => Ok(()),
        }
    }
}

impl FromStr for ChordSymbol {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ChordSymbol::parse(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pitch::{Letter, Spelling};
    use ChordQuality::*;

    fn read(text: &str) -> ChordSymbol {
        ChordSymbol::parse(text).unwrap_or_else(|error| panic!("{error}"))
    }

    fn names(notes: &[NoteName]) -> String {
        notes
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn reads_the_chords_of_a_lead_sheet() {
        let cases = [
            ("C", "C", Major),
            ("Am7", "A", Minor7),
            ("F#m7b5", "F#", HalfDiminished7),
            ("Bb13", "Bb", Dominant13),
            ("Ebmaj7", "Eb", Major7),
            ("Dsus4", "D", Sus4),
            ("Dsus", "D", Sus4),
            ("Gsus2", "G", Sus2),
            ("Cadd9", "C", Add9),
            ("Em(add9)", "E", MinorAdd9),
            ("Dbdim7", "Db", Diminished7),
            ("C°7", "C", Diminished7),
            ("Bø", "B", HalfDiminished7),
            ("C-7", "C", Minor7),
            ("CΔ7", "C", Major7),
            ("CM7", "C", Major7),
            ("C7(b9)", "C", Dominant7Flat9),
            ("A7#9", "A", Dominant7Sharp9),
            ("Gm6", "G", Minor6),
            ("C6/9", "C", Six9),
            ("C69", "C", Six9),
            ("Cm(maj7)", "C", MinorMajor7),
            ("C+", "C", Augmented),
            ("Caug", "C", Augmented),
            ("C7#5", "C", Augmented7),
            ("C+7", "C", Augmented7),
            ("C7b5", "C", Dominant7Flat5),
            ("F11", "F", Dominant11),
            ("Dm11", "D", Minor11),
            ("Fmaj9", "F", Major9),
            ("C9", "C", Dominant9),
            ("Cmaj13", "C", Major13),
            ("C7sus4", "C", Dominant7Sus4),
            ("E♭m7", "Eb", Minor7),
            (" G7 ", "G", Dominant7),
        ];
        for (text, root, quality) in cases {
            let chord = read(text);
            assert_eq!(chord.root.to_string(), root, "{text}");
            assert_eq!(chord.quality, quality, "{text}");
            assert_eq!(chord.bass, None, "{text}");
        }
    }

    #[test]
    fn a_slash_puts_a_note_in_the_bass() {
        let g_over_b = read("G/B");
        assert_eq!(
            (g_over_b.root.to_string(), g_over_b.quality),
            ("G".into(), Major)
        );
        assert_eq!(g_over_b.bass, Some(NoteName::new(Letter::B, 0)));
        assert_eq!(read("Am7/G").bass, NoteName::parse("G"));
        assert_eq!(read("D/F#").bass, NoteName::parse("F#"));
        let six_nine = read("C6/9/E");
        assert_eq!(six_nine.quality, Six9);
        assert_eq!(six_nine.bass, NoteName::parse("E"));
        assert_eq!(read("C6/9").bass, None, "the slash of a six-nine chord");
        assert_eq!(
            read("C/C"),
            read("C"),
            "the root in the bass is no slash chord"
        );
    }

    #[test]
    fn nonsense_is_refused() {
        for text in [
            "", "H7", "c", "am7", "Cxyz", "C/H", "C#5", "C/", "/E", "Bbb7", "Cmaj8", "C7/9",
        ] {
            assert!(ChordSymbol::parse(text).is_err(), "{text:?} was read");
        }
    }

    #[test]
    fn every_quality_is_written_and_read_back() {
        for quality in ChordQuality::ALL {
            for root in PitchClass::ALL {
                for spelling in [Spelling::Sharps, Spelling::Flats] {
                    let chord = ChordSymbol::new(NoteName::plain(root, spelling), quality);
                    let text = chord.to_string();
                    assert_eq!(read(&text), chord, "{text}");
                    let slash = chord.over(NoteName::new(Letter::E, -1));
                    if slash.bass.is_some() {
                        assert_eq!(read(&slash.to_string()), slash, "{slash}");
                    }
                }
            }
        }
    }

    #[test]
    fn spelled_up_from_the_root() {
        assert_eq!(names(&read("F#m7b5").notes()), "F# A C E");
        assert_eq!(names(&read("Bb13").notes()), "Bb D F Ab C G");
        assert_eq!(names(&read("Db7").notes()), "Db F Ab Cb");
        assert_eq!(names(&read("C#7").notes()), "C# E# G# B");
        assert_eq!(read("Db7").chord(), read("C#7").chord(), "the same keys");
    }

    #[test]
    fn inversions_and_chords_over_a_bass() {
        assert_eq!(read("C/E").bass_in_chord(), Some(3));
        assert_eq!(read("C/G").bass_in_chord(), Some(5));
        assert_eq!(read("G7/F").bass_in_chord(), Some(7));
        assert_eq!(read("F/G").bass_in_chord(), None, "G is not in F major");
        assert_eq!(read("C").bass_in_chord(), None);
        assert_eq!(read("F/G").bass_pitch_class(), PitchClass::G);
        assert_eq!(read("Am7").bass_pitch_class(), PitchClass::A);
    }

    #[test]
    fn transposing_moves_the_letters_too() {
        let e = NoteName::new(Letter::E, 0);
        let chart = ["F#m7b5", "B7", "Em"].map(read);
        let moved = |to: &str| -> Vec<String> {
            let to = NoteName::parse(to).unwrap();
            chart
                .iter()
                .map(|c| c.transposed(e, to).to_string())
                .collect()
        };
        assert_eq!(moved("D"), ["Em7b5", "A7", "Dm"]);
        assert_eq!(moved("C"), ["Dm7b5", "G7", "Cm"]);
        assert_eq!(moved("Bb"), ["Cm7b5", "F7", "Bbm"]);
        assert_eq!(moved("G#"), ["A#m7b5", "D#7", "G#m"]);
        assert_eq!(moved("Eb"), ["Fm7b5", "Bb7", "Ebm"]);
        // In every key, ii is one letter above the tonic and V four.
        for tonic in PitchClass::ALL {
            for spelling in [Spelling::Sharps, Spelling::Flats] {
                let to = NoteName::plain(tonic, spelling);
                let [two, five, one] = chart.map(|c| c.transposed(e, to));
                assert_eq!(one.root, to);
                assert_eq!(two.root.letter, to.letter.up(1), "ii in {to}");
                assert_eq!(five.root.letter, to.letter.up(4), "V in {to}");
                assert_eq!(two.root.pitch_class(), tonic.transpose(2));
                assert_eq!(five.root.pitch_class(), tonic.transpose(7));
            }
        }
    }

    #[test]
    fn transposing_keeps_the_bass_and_avoids_double_sharps() {
        let c = NoteName::new(Letter::C, 0);
        let to_d = read("G/B").transposed(c, NoteName::new(Letter::D, 0));
        assert_eq!(to_d.to_string(), "A/C#");
        let e = NoteName::new(Letter::E, 0);
        let g_sharp = NoteName::new(Letter::G, 1);
        assert_eq!(read("D#dim7").transposed(e, g_sharp).to_string(), "Gdim7");
    }
}
