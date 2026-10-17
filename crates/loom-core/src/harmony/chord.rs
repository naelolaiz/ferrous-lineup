//! Chords: what they are made of, what they are called, and how to place them.

use super::{PitchSet, Scale};
use crate::{Note, NoteName, PitchClass, Spelling};

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
    Dominant7Flat5,
    Augmented7,
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

/// The three kinds of triad: major, minor and diminished. See
/// [`ChordQuality::family`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordFamily {
    Major,
    Minor,
    Diminished,
}

impl ChordQuality {
    /// All qualities. When several fit the same notes, the first one
    /// listed wins, so the simpler, more common chords come first.
    pub const ALL: [ChordQuality; 30] = [
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
        ChordQuality::Dominant7Flat5,
        ChordQuality::Augmented7,
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
            Dominant7Flat5 => &[0, 4, 6, 10],
            Augmented7 => &[0, 4, 8, 10],
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

    /// Which note of the chord each interval is, counted in letters from
    /// the root as musicians count them: 1 is the root, 3 the third, 5
    /// the fifth, 7 the seventh, 9 the ninth... in the order of
    /// [`ChordQuality::intervals`].
    ///
    /// This is what spells a chord. The third of A is two letters up,
    /// so it is C#, never Db. The sharp ninth of E7#9 is a ninth, one
    /// letter above the root: F##, not G. The seventh of a diminished
    /// seventh chord is a seventh even though it sounds like a sixth:
    /// Bbb in C dim7, not A.
    pub fn tones(self) -> &'static [u8] {
        use ChordQuality::*;
        match self {
            Major | Minor | Diminished | Augmented => &[1, 3, 5],
            Sus2 => &[1, 2, 5],
            Sus4 => &[1, 4, 5],
            Major6 | Minor6 => &[1, 3, 5, 6],
            Dominant7 | Major7 | Minor7 | MinorMajor7 | HalfDiminished7 | Diminished7
            | AugmentedMajor7 | Dominant7Flat5 | Augmented7 => &[1, 3, 5, 7],
            Dominant7Sus4 => &[1, 4, 5, 7],
            Add9 | MinorAdd9 => &[1, 3, 5, 9],
            Dominant9 | Major9 | Minor9 | Dominant7Flat9 | Dominant7Sharp9 => &[1, 3, 5, 7, 9],
            Six9 => &[1, 3, 5, 6, 9],
            Dominant11 | Minor11 => &[1, 3, 5, 7, 9, 11],
            Dominant13 | Major13 => &[1, 3, 5, 7, 9, 13],
        }
    }

    /// The kind of triad the chord is built on: major, minor or
    /// diminished. Sus chords and augmented chords, which have no minor
    /// third, count as major. Keys use it to match a chord to their own
    /// chords: G7 is the major chord on G, and Bm7b5 the diminished one
    /// on B.
    pub fn family(self) -> ChordFamily {
        use ChordQuality::*;
        match self {
            Diminished | HalfDiminished7 | Diminished7 => ChordFamily::Diminished,
            quality if quality.is_minor() => ChordFamily::Minor,
            _ => ChordFamily::Major,
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
            Dominant7Flat5 => "7b5",
            Augmented7 => "7#5",
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
            Dominant7Flat5 => "dominant seventh flat fifth",
            Augmented7 => "augmented seventh",
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

    /// Whether this is a dominant seventh chord, maybe with more notes
    /// or an altered fifth or ninth: the chords that pull to the chord a
    /// fifth below, as G7, G7b9 and G7#5 all pull to C.
    pub fn is_dominant(self) -> bool {
        use ChordQuality::*;
        matches!(
            self,
            Dominant7
                | Dominant9
                | Dominant7Flat9
                | Dominant7Sharp9
                | Dominant7Flat5
                | Augmented7
                | Dominant11
                | Dominant13
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
            Augmented7 => "+7",
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

    /// The chord symbol with its root written as `root`: "C#m7" or
    /// "Dbm7". `root` should name the chord's root.
    pub fn name_from(self, root: NoteName) -> String {
        format!("{root}{}", self.quality.symbol())
    }

    /// The chord's notes, spelled up from its root's name in the order of
    /// [`ChordQuality::intervals`]: A major from A is A C# E, and B
    /// diminished seventh is B D F Ab.
    pub fn spell(self, root: NoteName) -> Vec<NoteName> {
        self.quality
            .intervals()
            .iter()
            .zip(self.quality.tones())
            .map(|(&interval, &tone)| root.above(interval % 12, tone - 1))
            .collect()
    }

    /// How a note of the chord is written when the chord's root is
    /// `root`: its third, fifth or seventh. `None` if it is not in the
    /// chord.
    pub fn spell_note(self, root: NoteName, pitch_class: PitchClass) -> Option<NoteName> {
        self.spell(root)
            .into_iter()
            .find(|name| name.pitch_class() == pitch_class)
    }

    /// A name for the root when no key says how to write it: the sharp
    /// or the flat name of the root, whichever spells the whole chord
    /// with fewer sharps and flats. C# major is C# E# G#, Db major is
    /// Db F Ab: it is written Db. On a tie, the usual name of the black
    /// key wins (see [`NoteName::chromatic`]).
    pub fn root_name(self) -> NoteName {
        let accidentals = |root: NoteName| -> u32 {
            self.spell(root)
                .iter()
                .map(|name| u32::from(name.accidental.unsigned_abs()))
                .sum()
        };
        let usual = NoteName::chromatic(self.root);
        [Spelling::Sharps, Spelling::Flats]
            .into_iter()
            .map(|spelling| NoteName::plain(self.root, spelling))
            .fold(usual, |best, other| {
                if accidentals(other) < accidentals(best) {
                    other
                } else {
                    best
                }
            })
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

    #[test]
    fn chords_are_spelled_by_their_letters() {
        let spelled = |root: NoteName, quality| {
            let names: Vec<String> = Chord::new(root.pitch_class(), quality)
                .spell(root)
                .iter()
                .map(|n| n.to_string())
                .collect();
            names.join(" ")
        };
        let name = |letter, accidental| NoteName::new(letter, accidental);
        use crate::Letter;
        assert_eq!(spelled(name(Letter::A, 0), ChordQuality::Major), "A C# E");
        assert_eq!(
            spelled(name(Letter::E, -1), ChordQuality::Minor),
            "Eb Gb Bb"
        );
        assert_eq!(
            spelled(name(Letter::C, 0), ChordQuality::Diminished7),
            "C Eb Gb Bbb"
        );
        assert_eq!(
            spelled(name(Letter::B, 0), ChordQuality::Diminished7),
            "B D F Ab"
        );
        assert_eq!(
            spelled(name(Letter::E, 0), ChordQuality::Dominant7Sharp9),
            "E G# B D F##"
        );
        assert_eq!(
            spelled(name(Letter::C, 0), ChordQuality::Augmented),
            "C E G#"
        );
        assert_eq!(spelled(name(Letter::F, 1), ChordQuality::Major), "F# A# C#");
        assert_eq!(
            spelled(name(Letter::C, 0), ChordQuality::Dominant13),
            "C E G Bb D A"
        );
        // Every quality has a tone number for each interval.
        for quality in ChordQuality::ALL {
            assert_eq!(
                quality.tones().len(),
                quality.intervals().len(),
                "{quality:?}"
            );
        }
    }

    #[test]
    fn roots_without_a_key() {
        let root = |pc, quality| Chord::new(pc, quality).root_name().to_string();
        assert_eq!(root(CSharp, ChordQuality::Major), "Db");
        assert_eq!(root(GSharp, ChordQuality::Minor), "G#");
        assert_eq!(root(DSharp, ChordQuality::Minor), "Eb");
        assert_eq!(root(FSharp, ChordQuality::Major), "F#");
        assert_eq!(root(C, ChordQuality::Major), "C");
    }

    #[test]
    fn families() {
        assert_eq!(ChordQuality::Dominant7.family(), ChordFamily::Major);
        assert_eq!(ChordQuality::Augmented.family(), ChordFamily::Major);
        assert_eq!(ChordQuality::Minor7.family(), ChordFamily::Minor);
        assert_eq!(
            ChordQuality::HalfDiminished7.family(),
            ChordFamily::Diminished
        );
    }
}
