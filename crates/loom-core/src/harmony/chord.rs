//! Chords: what they are made of, what they are called, and how to place them.

use super::{PitchSet, Scale};
use crate::{Note, PitchClass, Spelling};

/// The "shape" of a chord, independent of its root: major, minor seventh...
///
/// Each quality is a list of intervals above the root, in semitones. A
/// major triad is root, major third (4 semitones) and perfect fifth (7):
/// C major is C, E, G and D major is D, F#, A.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,
    Major6,
    Minor6,
    Dominant7,
    Major7,
    Minor7,
    MinorMajor7,
    HalfDiminished7,
    Diminished7,
    AugmentedMajor7,
    Dominant7Sus4,
    Add9,
    MinorAdd9,
    Dominant9,
    Major9,
    Minor9,
    Six9,
    Dominant7Flat9,
    Dominant7Sharp9,
    Dominant11,
    Minor11,
    Dominant13,
    Major13,
}

impl ChordQuality {
    /// All qualities. When several fit the same notes, the first one
    /// listed wins, so the simpler, more common chords come first.
    pub const ALL: [ChordQuality; 28] = [
        ChordQuality::Major,
        ChordQuality::Minor,
        ChordQuality::Diminished,
        ChordQuality::Augmented,
        ChordQuality::Sus2,
        ChordQuality::Sus4,
        ChordQuality::Dominant7,
        ChordQuality::Major7,
        ChordQuality::Minor7,
        ChordQuality::HalfDiminished7,
        ChordQuality::Diminished7,
        ChordQuality::Major6,
        ChordQuality::Minor6,
        ChordQuality::MinorMajor7,
        ChordQuality::AugmentedMajor7,
        ChordQuality::Dominant7Sus4,
        ChordQuality::Add9,
        ChordQuality::MinorAdd9,
        ChordQuality::Dominant9,
        ChordQuality::Major9,
        ChordQuality::Minor9,
        ChordQuality::Six9,
        ChordQuality::Dominant7Flat9,
        ChordQuality::Dominant7Sharp9,
        ChordQuality::Dominant11,
        ChordQuality::Minor11,
        ChordQuality::Dominant13,
        ChordQuality::Major13,
    ];

    /// The notes of the chord as semitones above the root, lowest first.
    pub fn intervals(self) -> &'static [u8] {
        use ChordQuality::*;
        match self {
            Major => &[0, 4, 7],
            Minor => &[0, 3, 7],
            Diminished => &[0, 3, 6],
            Augmented => &[0, 4, 8],
            Sus2 => &[0, 2, 7],
            Sus4 => &[0, 5, 7],
            Major6 => &[0, 4, 7, 9],
            Minor6 => &[0, 3, 7, 9],
            Dominant7 => &[0, 4, 7, 10],
            Major7 => &[0, 4, 7, 11],
            Minor7 => &[0, 3, 7, 10],
            MinorMajor7 => &[0, 3, 7, 11],
            HalfDiminished7 => &[0, 3, 6, 10],
            Diminished7 => &[0, 3, 6, 9],
            AugmentedMajor7 => &[0, 4, 8, 11],
            Dominant7Sus4 => &[0, 5, 7, 10],
            Add9 => &[0, 4, 7, 14],
            MinorAdd9 => &[0, 3, 7, 14],
            Dominant9 => &[0, 4, 7, 10, 14],
            Major9 => &[0, 4, 7, 11, 14],
            Minor9 => &[0, 3, 7, 10, 14],
            Six9 => &[0, 4, 7, 9, 14],
            Dominant7Flat9 => &[0, 4, 7, 10, 13],
            Dominant7Sharp9 => &[0, 4, 7, 10, 15],
            // The eleventh is the fourth an octave up (17 = 12 + 5), and
            // the thirteenth the sixth an octave up (21 = 12 + 9).
            Dominant11 => &[0, 4, 7, 10, 14, 17],
            Minor11 => &[0, 3, 7, 10, 14, 17],
            // Players leave the eleventh out of thirteenth chords, because
            // it clashes with the third; so does Ferrous Loom.
            Dominant13 => &[0, 4, 7, 10, 14, 21],
            Major13 => &[0, 4, 7, 11, 14, 21],
        }
    }

    /// The symbol written after the root in a chord name: the "m7" of "Am7".
    pub fn symbol(self) -> &'static str {
        use ChordQuality::*;
        match self {
            Major => "",
            Minor => "m",
            Diminished => "dim",
            Augmented => "aug",
            Sus2 => "sus2",
            Sus4 => "sus4",
            Major6 => "6",
            Minor6 => "m6",
            Dominant7 => "7",
            Major7 => "maj7",
            Minor7 => "m7",
            MinorMajor7 => "m(maj7)",
            HalfDiminished7 => "m7b5",
            Diminished7 => "dim7",
            AugmentedMajor7 => "maj7#5",
            Dominant7Sus4 => "7sus4",
            Add9 => "add9",
            MinorAdd9 => "m(add9)",
            Dominant9 => "9",
            Major9 => "maj9",
            Minor9 => "m9",
            Six9 => "6/9",
            Dominant7Flat9 => "7b9",
            Dominant7Sharp9 => "7#9",
            Dominant11 => "11",
            Minor11 => "m11",
            Dominant13 => "13",
            Major13 => "maj13",
        }
    }

    /// The name in words, such as "minor seventh".
    pub fn name(self) -> &'static str {
        use ChordQuality::*;
        match self {
            Major => "major",
            Minor => "minor",
            Diminished => "diminished",
            Augmented => "augmented",
            Sus2 => "suspended second",
            Sus4 => "suspended fourth",
            Major6 => "major sixth",
            Minor6 => "minor sixth",
            Dominant7 => "dominant seventh",
            Major7 => "major seventh",
            Minor7 => "minor seventh",
            MinorMajor7 => "minor major seventh",
            HalfDiminished7 => "half-diminished seventh",
            Diminished7 => "diminished seventh",
            AugmentedMajor7 => "augmented major seventh",
            Dominant7Sus4 => "dominant seventh suspended fourth",
            Add9 => "added ninth",
            MinorAdd9 => "minor added ninth",
            Dominant9 => "dominant ninth",
            Major9 => "major ninth",
            Minor9 => "minor ninth",
            Six9 => "six-nine",
            Dominant7Flat9 => "dominant seventh flat ninth",
            Dominant7Sharp9 => "dominant seventh sharp ninth",
            Dominant11 => "dominant eleventh",
            Minor11 => "minor eleventh",
            Dominant13 => "dominant thirteenth",
            Major13 => "major thirteenth",
        }
    }

    /// Whether the chord has a minor third and no major third. Roman
    /// numerals for these chords are written in lower case (ii, vi).
    pub fn is_minor(self) -> bool {
        use ChordQuality::*;
        matches!(
            self,
            Minor
                | Diminished
                | Minor6
                | Minor7
                | MinorMajor7
                | HalfDiminished7
                | Diminished7
                | MinorAdd9
                | Minor9
                | Minor11
        )
    }

    /// What follows a Roman numeral: the "7" of "V7" or the "°" of "vii°".
    pub fn numeral_suffix(self) -> &'static str {
        use ChordQuality::*;
        match self {
            Major | Minor => "",
            Diminished => "°",
            Augmented => "+",
            Diminished7 => "°7",
            HalfDiminished7 => "ø7",
            AugmentedMajor7 => "+maj7",
            Minor6 => "6",
            Minor7 => "7",
            MinorMajor7 => "maj7",
            MinorAdd9 => "add9",
            Minor9 => "9",
            Minor11 => "11",
            // Every other quality keeps its usual symbol, as in "IVmaj7".
            other => other.symbol(),
        }
    }

    /// The pitch classes of this chord when its root is C.
    pub fn pitch_set(self) -> PitchSet {
        PitchSet::from_semitones(self.intervals())
    }

    /// The quality whose notes, seen from the root, are exactly `relative`.
    pub fn identify(relative: PitchSet) -> Option<ChordQuality> {
        Self::ALL
            .into_iter()
            .find(|quality| quality.pitch_set() == relative)
    }
}

/// A chord: a root and a quality, like A minor seventh ("Am7").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord {
    pub root: PitchClass,
    pub quality: ChordQuality,
}

impl Chord {
    pub fn new(root: PitchClass, quality: ChordQuality) -> Self {
        Chord { root, quality }
    }

    /// The pitch classes of the chord.
    pub fn pitch_set(self) -> PitchSet {
        self.quality.pitch_set().transpose(self.root.semitone())
    }

    /// The chord's notes, stacked upwards from `root_note` (which should
    /// have the chord's root as its pitch class). Notes that would be
    /// above the highest MIDI note are left out.
    pub fn notes_from(self, root_note: Note) -> Vec<Note> {
        self.quality
            .intervals()
            .iter()
            .filter_map(|&interval| root_note.transpose(interval as i32))
            .collect()
    }

    /// The chord symbol, such as "F#m7" or "Bbmaj7".
    pub fn name(self, spelling: Spelling) -> String {
        format!("{}{}", self.root.name(spelling), self.quality.symbol())
    }
}

/// What a click in the piano roll places: one note or a whole chord.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ChordStamp {
    /// Just the note you clicked.
    #[default]
    Single,
    /// The three-note chord of the scale that starts on the clicked note.
    ScaleTriad,
    /// The four-note (seventh) chord of the scale that starts on the clicked note.
    ScaleSeventh,
    /// A chord of a fixed quality, starting on the clicked note.
    Fixed(ChordQuality),
    /// A pitch-class set, as semitones above the clicked note: any group
    /// of notes, picked in the piano roll's set browser.
    Set(PitchSet),
}

impl ChordStamp {
    /// Every stamp, in the order the piano roll menu lists them.
    pub fn all() -> impl Iterator<Item = ChordStamp> {
        [
            ChordStamp::Single,
            ChordStamp::ScaleTriad,
            ChordStamp::ScaleSeventh,
        ]
        .into_iter()
        .chain(ChordQuality::ALL.into_iter().map(ChordStamp::Fixed))
    }

    /// A name for menus.
    pub fn label(self) -> String {
        match self {
            ChordStamp::Single => "Single note".to_string(),
            ChordStamp::ScaleTriad => "Chord in key (triad)".to_string(),
            ChordStamp::ScaleSeventh => "Chord in key (seventh)".to_string(),
            ChordStamp::Set(set) => {
                format!("Set {} [{}]", set.set_class().name(), super::digits(set))
            }
            ChordStamp::Fixed(quality) => {
                let name = quality.name();
                // Capitalize the first letter: "minor" becomes "Minor".
                let mut letters = name.chars();
                match letters.next() {
                    Some(first) => first.to_uppercase().chain(letters).collect(),
                    None => String::new(),
                }
            }
        }
    }

    /// The notes to place when the user clicks on `note`.
    ///
    /// The scale stamps only work on notes of the scale, and only for
    /// seven-note scales; anywhere else they place the single note.
    pub fn notes_at(self, note: Note, scale: Scale) -> Vec<Note> {
        match self {
            ChordStamp::Single => vec![note],
            ChordStamp::ScaleTriad => scale.stack_thirds(note, 3).unwrap_or_else(|| vec![note]),
            ChordStamp::ScaleSeventh => scale.stack_thirds(note, 4).unwrap_or_else(|| vec![note]),
            ChordStamp::Fixed(quality) => Chord::new(note.pitch_class(), quality).notes_from(note),
            ChordStamp::Set(set) => set
                .semitones()
                .into_iter()
                .filter_map(|up| note.transpose(i32::from(up)))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ScaleKind;
    use PitchClass::*;

    #[test]
    fn every_quality_is_listed_once() {
        for quality in ChordQuality::ALL {
            let count = ChordQuality::ALL.iter().filter(|&&q| q == quality).count();
            assert_eq!(count, 1, "{quality:?}");
        }
    }

    #[test]
    fn no_two_qualities_have_the_same_notes() {
        // Otherwise `identify` could never find the second one.
        for (i, a) in ChordQuality::ALL.iter().enumerate() {
            for b in &ChordQuality::ALL[i + 1..] {
                assert_ne!(a.pitch_set(), b.pitch_set(), "{a:?} and {b:?}");
            }
        }
    }

    #[test]
    fn chord_names() {
        assert_eq!(
            Chord::new(A, ChordQuality::Minor7).name(Spelling::Sharps),
            "Am7"
        );
        assert_eq!(
            Chord::new(ASharp, ChordQuality::Major).name(Spelling::Flats),
            "Bb"
        );
        assert_eq!(
            Chord::new(FSharp, ChordQuality::Diminished).name(Spelling::Sharps),
            "F#dim"
        );
    }

    #[test]
    fn chord_pitch_sets() {
        let d_major: PitchSet = [D, FSharp, A].into_iter().collect();
        assert_eq!(Chord::new(D, ChordQuality::Major).pitch_set(), d_major);
        let b_dim: PitchSet = [B, D, F].into_iter().collect();
        assert_eq!(Chord::new(B, ChordQuality::Diminished).pitch_set(), b_dim);
    }

    #[test]
    fn identify_finds_the_quality() {
        let relative = PitchSet::from_semitones(&[0, 3, 7, 10]);
        assert_eq!(ChordQuality::identify(relative), Some(ChordQuality::Minor7));
        // Ninths wrap around the octave: 14 semitones is a 2 in a pitch set.
        let add9 = PitchSet::from_semitones(&[0, 2, 4, 7]);
        assert_eq!(ChordQuality::identify(add9), Some(ChordQuality::Add9));
        assert_eq!(
            ChordQuality::identify(PitchSet::from_semitones(&[0, 1, 2])),
            None
        );
    }

    #[test]
    fn notes_from_a_root() {
        let notes = Chord::new(C, ChordQuality::Major7).notes_from(Note::C4);
        let midi: Vec<u8> = notes.iter().map(|note| note.midi()).collect();
        assert_eq!(midi, vec![60, 64, 67, 71]);
    }

    #[test]
    fn stamps() {
        let c_major = Scale::new(C, ScaleKind::Major);
        let midi = |notes: Vec<Note>| notes.iter().map(|n| n.midi()).collect::<Vec<_>>();
        let d4 = Note::new(62);

        assert_eq!(midi(ChordStamp::Single.notes_at(d4, c_major)), vec![62]);
        // D in C major gives D minor: D, F, A.
        assert_eq!(
            midi(ChordStamp::ScaleTriad.notes_at(d4, c_major)),
            vec![62, 65, 69]
        );
        // ... and D minor seventh: D, F, A, C.
        assert_eq!(
            midi(ChordStamp::ScaleSeventh.notes_at(d4, c_major)),
            vec![62, 65, 69, 72]
        );
        // D# is not in C major, so the scale stamps fall back to one note.
        assert_eq!(
            midi(ChordStamp::ScaleTriad.notes_at(Note::new(63), c_major)),
            vec![63]
        );
        // A fixed stamp ignores the scale.
        let fixed = ChordStamp::Fixed(ChordQuality::Major).notes_at(d4, c_major);
        assert_eq!(midi(fixed), vec![62, 66, 69]);
        // A set is placed above the clicked note, like a chord.
        let viennese = ChordStamp::Set(PitchSet::from_semitones(&[0, 1, 6]));
        assert_eq!(midi(viennese.notes_at(d4, c_major)), vec![62, 63, 68]);
        assert_eq!(viennese.label(), "Set 3-5 [016]");
    }

    #[test]
    fn stamp_labels() {
        assert_eq!(
            ChordStamp::Fixed(ChordQuality::Minor7).label(),
            "Minor seventh"
        );
        assert_eq!(ChordStamp::all().count(), 3 + ChordQuality::ALL.len());
    }
}
