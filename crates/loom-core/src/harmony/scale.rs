//! Scales and keys: which notes belong together.

use serde::{Deserialize, Serialize};

use super::{Chord, ChordQuality, PitchSet, ToneRow};
use crate::{Note, NoteName, PitchClass, Spelling};

/// How many letters above the tonic a note outside the scale is written,
/// by how many semitones above the tonic it is. Notes a semitone or
/// three, eight or ten semitones up are flats (b2, b3, b6, b7), six is
/// a sharp fourth (#4), and nine and eleven are the major sixth and
/// seventh: in A minor the raised notes are F# and G#, a sixth and a
/// seventh above A.
const CHROMATIC_LETTERS: [u8; 12] = [0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6];

/// The kinds of scale the piano roll can highlight.
///
/// Besides the scales of tonal music there are two kinds for music
/// without a key: a twelve-tone row and a pitch-class set. They carry
/// their notes with them, as semitones above the root.
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
    /// A twelve-tone row, stored starting on C. The scale's root is the
    /// note it starts on, so `E` with the row of Schoenberg's Suite is
    /// its form `P4`. Its "degrees" are the places in the row, 1 to 12.
    Row(ToneRow),
    /// A pitch-class set used as a scale: its notes as semitones above
    /// the root, the root among them. Any group of notes can be a scale
    /// this way, from three notes to eleven.
    Set(PitchSet),
}

impl ScaleKind {
    /// The scales of tonal music, in the order the menu lists them. The
    /// menus add the twelve-tone row and the pitch-class set after them.
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
            ScaleKind::Row(_) => "twelve-tone row",
            ScaleKind::Set(_) => "pitch-class set",
        }
    }

    /// Whether this is a scale of tonal music, with a home note that the
    /// others lean on, rather than a row or a set.
    pub fn is_tonal(self) -> bool {
        !matches!(self, ScaleKind::Row(_) | ScaleKind::Set(_))
    }

    /// Whether the notes of a song should move when its key changes from
    /// this kind to `other`. Between the scales of tonal music they do,
    /// as between two transpositions of the same row or two sets. Between
    /// a key and a row, two different rows, or a key and a set, there is
    /// nothing to move by: only what the piano roll highlights changes.
    pub fn notes_follow(self, other: ScaleKind) -> bool {
        match (self, other) {
            (ScaleKind::Row(a), ScaleKind::Row(b)) => a == b,
            (ScaleKind::Set(_), ScaleKind::Set(_)) => true,
            (a, b) => a.is_tonal() && b.is_tonal(),
        }
    }

    /// The notes of the scale as semitones above its root: for a row, in
    /// the row's order.
    ///
    /// The major scale is the pattern of the white keys from C:
    /// whole, whole, half, whole, whole, whole, half step.
    pub fn intervals(self) -> Vec<u8> {
        match self {
            ScaleKind::Row(row) => row.semitones().to_vec(),
            ScaleKind::Set(set) => set.semitones(),
            kind => kind.tonal_intervals().to_vec(),
        }
    }

    /// The notes of a tonal scale as semitones above its root.
    fn tonal_intervals(self) -> &'static [u8] {
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
            ScaleKind::Chromatic | ScaleKind::Row(_) | ScaleKind::Set(_) => {
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            }
        }
    }

    /// How many letters above the tonic each note of a tonal scale is
    /// written, in the order of its intervals. A seven-note scale uses
    /// each letter once. The pentatonic scales leave two letters out,
    /// and the blues scale writes its "blue note" as a flat fifth: A C D
    /// Eb E G.
    fn letter_steps(self) -> &'static [u8] {
        match self {
            ScaleKind::MajorPentatonic => &[0, 1, 2, 4, 5],
            ScaleKind::MinorPentatonic => &[0, 2, 3, 4, 6],
            ScaleKind::Blues => &[0, 2, 3, 4, 4, 6],
            ScaleKind::Chromatic | ScaleKind::Row(_) | ScaleKind::Set(_) => &CHROMATIC_LETTERS,
            _ => &[0, 1, 2, 3, 4, 5, 6],
        }
    }

    /// How far above the root the major key with the same key signature
    /// starts. A minor has no sharps or flats, like C major, which is 3
    /// semitones higher; D Dorian also uses only white keys, and C is 10
    /// semitones above D.
    fn relative_major(self) -> u8 {
        match self {
            ScaleKind::Major
            | ScaleKind::MajorPentatonic
            | ScaleKind::Chromatic
            | ScaleKind::Row(_)
            | ScaleKind::Set(_) => 0,
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

    /// A twelve-tone row as a scale: rooted on the note it starts on.
    pub fn row(row: ToneRow) -> Self {
        Scale::new(row.first(), ScaleKind::Row(row.starting_on(PitchClass::C)))
    }

    /// A pitch-class set as a scale on `root`. The root is added to the
    /// set if it is not in it.
    pub fn set(root: PitchClass, set: PitchSet) -> Self {
        let mut relative = set.relative_to(root);
        relative.insert(PitchClass::C);
        Scale::new(root, ScaleKind::Set(relative))
    }

    /// The row this scale plays, starting on its root, if it is a row.
    pub fn tone_row(self) -> Option<ToneRow> {
        match self.kind {
            ScaleKind::Row(row) => Some(row.starting_on(self.root)),
            _ => None,
        }
    }

    /// The pitch classes of the scale, from the root upwards: for a row,
    /// in the row's order.
    pub fn pitch_classes(self) -> impl Iterator<Item = PitchClass> {
        self.kind
            .intervals()
            .into_iter()
            .map(move |interval| self.root.transpose(interval as i32))
    }

    pub fn pitch_set(self) -> PitchSet {
        self.pitch_classes().collect()
    }

    pub fn contains(self, pitch_class: PitchClass) -> bool {
        match self.kind {
            ScaleKind::Row(_) => true,
            ScaleKind::Set(set) => {
                set.contains(pitch_class.transpose(-(self.root.semitone() as i32)))
            }
            _ => self.pitch_set().contains(pitch_class),
        }
    }

    /// Which note of the scale this is, counting the root as 0, or `None`
    /// if it is not in the scale. (Musicians count from 1: degree 0 here
    /// is "the first degree".)
    pub fn degree_of(self, pitch_class: PitchClass) -> Option<usize> {
        self.pitch_classes().position(|p| p == pitch_class)
    }

    /// Whether the scale has seven notes, like major, minor and the modes.
    /// Chords in key and Roman numerals only make sense for these, so a
    /// set of seven notes does not count.
    pub fn is_heptatonic(self) -> bool {
        self.kind.is_tonal() && self.kind.tonal_intervals().len() == 7
    }

    /// Whether this key's signature has flats or sharps, which is how
    /// its tonic is written: Bb major, not A# major.
    ///
    /// Keys whose major key signature has flats (F, Bb, Eb, Ab, Db) use
    /// flats; all others use sharps. Naming every note with just sharps
    /// or just flats is not enough, though: D harmonic minor has a Bb
    /// and a C#, and F# major has an E#. [`Scale::spell`] names each
    /// note properly.
    pub fn spelling(self) -> Spelling {
        use PitchClass::*;
        match self.root.transpose(self.kind.relative_major() as i32) {
            F | ASharp | DSharp | GSharp | CSharp => Spelling::Flats,
            _ => Spelling::Sharps,
        }
    }

    /// How a note is written in this key.
    ///
    /// A note of a seven-note scale takes the letter of its place in the
    /// scale, so each letter comes once: F# major is F# G# A# B C# D# E#,
    /// D harmonic minor is D E F G A Bb C#, and F Locrian is F Gb Ab Bb
    /// Cb Db Eb. The pentatonic and blues scales follow the major or
    /// minor scale they come from, with the blues note as a flat fifth.
    ///
    /// A note outside the scale is written from the tonic as musicians
    /// most often meet it: a flat second, third, sixth or seventh, a
    /// sharp fourth, and the major sixth and seventh, which in a minor
    /// key are the raised notes of melodic and harmonic minor (C# in D
    /// minor). If that would need two sharps or flats, the plain name
    /// is used instead.
    ///
    /// A twelve-tone row or a pitch-class set has no letters of its own,
    /// so its notes take their usual names (see [`NoteName::chromatic`]).
    pub fn spell(self, pitch_class: PitchClass) -> NoteName {
        if !self.kind.is_tonal() {
            return NoteName::chromatic(pitch_class);
        }
        let name = NoteName::with_letter(
            pitch_class,
            self.tonic_name()
                .letter
                .up(i32::from(self.letters_above_tonic(pitch_class))),
        );
        if self.has_letter_for(pitch_class) {
            name
        } else {
            name.simplified()
        }
    }

    /// A note's name and octave, spelled in this key: in C# harmonic
    /// minor, the note a semitone above B3 is "B#3", not "C4". See
    /// [`Note::name_as`].
    pub fn note_name(self, note: Note) -> String {
        note.name_as(self.spell(note.pitch_class()))
    }

    /// The name of the tonic: the root, with sharps or flats as the key
    /// signature has them (see [`Scale::spelling`]).
    fn tonic_name(self) -> NoteName {
        NoteName::plain(self.root, self.spelling())
    }

    /// Whether `pitch_class` is a note the scale gives a letter of its
    /// own to.
    pub(crate) fn has_letter_for(self, pitch_class: PitchClass) -> bool {
        self.kind != ScaleKind::Chromatic && self.kind.is_tonal() && self.contains(pitch_class)
    }

    /// How many letters above the tonic `pitch_class` is written: its
    /// place in the scale, or for a note outside it, see
    /// [`CHROMATIC_LETTERS`].
    fn letters_above_tonic(self, pitch_class: PitchClass) -> u8 {
        let semitones = self.root.semitones_up_to(pitch_class);
        if self.has_letter_for(pitch_class)
            && let Some(place) = self
                .kind
                .tonal_intervals()
                .iter()
                .position(|&interval| interval == semitones)
        {
            return self.kind.letter_steps()[place];
        }
        CHROMATIC_LETTERS[semitones as usize]
    }

    /// How the root of `chord` is written in this key.
    ///
    /// A root in the scale is written as the scale writes it. A root
    /// outside it is written from the tonic, as [`Scale::spell`] does
    /// (Eb major in C major, the chord borrowed from C minor), unless
    /// that would spell the chord with two sharps or flats on one note:
    /// then the enharmonic name with fewer is used. In Db major, the
    /// chord a semitone up is written D major rather than Ebb major.
    ///
    /// Without a key (a row or a set) the root is written as
    /// [`Chord::root_name`] chooses.
    pub fn spell_root(self, chord: Chord) -> NoteName {
        if !self.kind.is_tonal() {
            return chord.root_name();
        }
        let strict = NoteName::with_letter(
            chord.root,
            self.tonic_name()
                .letter
                .up(i32::from(self.letters_above_tonic(chord.root))),
        );
        if self.has_letter_for(chord.root) {
            return strict;
        }
        let doubled = |root: NoteName| chord.spell(root).iter().any(|n| n.accidental.abs() >= 2);
        if !doubled(strict) {
            return strict;
        }
        let other = chord.root_name();
        if doubled(other) {
            strict.simplified()
        } else {
            other
        }
    }

    /// The chord's symbol, spelled in this key: the chord on the third
    /// note of A major is "C#m", and on the fourth of Db major "Gb".
    pub fn chord_name(self, chord: Chord) -> String {
        chord.name_from(self.spell_root(chord))
    }

    /// The chord's notes, spelled in this key, from the root up: in D
    /// minor, A major is A C# E.
    pub fn chord_notes(self, chord: Chord) -> Vec<NoteName> {
        chord.spell(self.spell_root(chord))
    }

    /// The name of the key, such as "Bb major" or "F# minor". A row is
    /// named by its form, "row P4 (on E)", and a set by its set class,
    /// "set 4-28 on C".
    pub fn name(self) -> String {
        let root = self.spell(self.root);
        match self.kind {
            ScaleKind::Row(_) => format!("row P{} (on {root})", self.root.semitone()),
            ScaleKind::Set(set) => format!("set {} on {root}", set.set_class().name()),
            kind => format!("{root} {}", kind.name()),
        }
    }

    /// Stacks `count` notes of the scale a third apart (every other note),
    /// starting on `note`: C, E, G in C major. Returns `None` if `note` is
    /// not in the scale or the scale does not have seven notes.
    pub fn stack_thirds(self, note: Note, count: usize) -> Option<Vec<Note>> {
        if !self.is_heptatonic() {
            return None;
        }
        let intervals = self.kind.tonal_intervals();
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

    /// The ninth chord (five notes) built on a degree of the scale, if it
    /// is one Ferrous Loom knows. On some degrees the ninth is only a
    /// semitone above the root an octave up (E G B D F in C major), which
    /// clashes, and there is no such chord.
    pub fn ninth(self, degree: usize) -> Option<Chord> {
        self.chord_on(degree, 5)
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
    /// case a minor third. Returns `None` if the scale does not have
    /// seven notes.
    ///
    /// Numerals count along the **major** scale of the tonic, as the
    /// library's progressions do: a flat in front means the root is a
    /// semitone lower than in the major scale. So the chords of A minor
    /// are i, ii°, bIII, iv, v, bVI and bVII (C is a minor third above
    /// A, where A major has C#), E Phrygian has its bII, and C Lydian
    /// its #iv°. A chord from outside the key is numbered the same way:
    /// Bb major in C major is bVII.
    pub fn roman_numeral(self, chord: Chord) -> Option<String> {
        const NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
        if !self.is_heptatonic() {
            return None;
        }
        let letters = usize::from(self.letters_above_tonic(chord.root));
        let in_major = ScaleKind::Major.tonal_intervals()[letters] as i8;
        let semitones = self.root.semitones_up_to(chord.root) as i8;
        // How far the root is from the major scale's note, from -6 to 5.
        let shift = (semitones - in_major + 18).rem_euclid(12) - 6;
        let accidentals = if shift < 0 { "b" } else { "#" }.repeat(shift.unsigned_abs() as usize);
        let numeral = if chord.quality.is_minor() {
            NUMERALS[letters].to_lowercase()
        } else {
            NUMERALS[letters].to_string()
        };
        Some(format!(
            "{accidentals}{numeral}{}",
            chord.quality.numeral_suffix()
        ))
    }

    /// Whether every note of the chord is in the scale.
    pub fn is_diatonic(self, chord: Chord) -> bool {
        chord.pitch_set().is_subset_of(self.pitch_set())
    }

    /// If `chord` is a **secondary dominant** in this key, the chord it
    /// is the dominant of. A major or dominant seventh chord from outside
    /// the key, a fifth above one of the key's major or minor chords
    /// (not the tonic), pulls to that chord as V pulls to I: in C major,
    /// D major is the V of G ("V/V") and E7 the V7 of Am ("V7/vi").
    ///
    /// In minor keys the key's chords are those of natural minor.
    pub fn secondary_dominant_target(self, chord: Chord) -> Option<Chord> {
        use ChordQuality::*;
        let dominant = chord.quality == Major || chord.quality.is_dominant();
        if !dominant || !self.is_heptatonic() || self.is_chord_of_key(chord) {
            return None;
        }
        let reference = match self.kind {
            ScaleKind::HarmonicMinor | ScaleKind::MelodicMinor => {
                Scale::new(self.root, ScaleKind::NaturalMinor)
            }
            _ => self,
        };
        let degree = reference.degree_of(chord.root.transpose(5))?;
        let target = reference.triad(degree)?;
        (degree != 0 && matches!(target.quality, Major | Minor)).then_some(target)
    }

    /// The key with the same tonic and the other mode, if this is a major
    /// or a minor key: C minor for C major, A major for A minor. Chords
    /// taken from it are **borrowed** chords ("modal mixture"), like the
    /// Fm and the Bb of C major.
    pub fn parallel(self) -> Option<Scale> {
        let kind = match self.kind {
            ScaleKind::Major => ScaleKind::NaturalMinor,
            ScaleKind::NaturalMinor | ScaleKind::HarmonicMinor | ScaleKind::MelodicMinor => {
                ScaleKind::Major
            }
            _ => return None,
        };
        Some(Scale::new(self.root, kind))
    }

    /// The parallel key `chord` is borrowed from, if it is not in this
    /// key but is in that one. A major key borrows from both the
    /// natural and the harmonic minor: Bb and Bdim7 in C major are both
    /// borrowed from C minor.
    pub fn borrowed_from(self, chord: Chord) -> Option<Scale> {
        let parallel = self.parallel()?;
        let harmonic = Scale::new(self.root, ScaleKind::HarmonicMinor);
        let in_parallel = parallel.is_diatonic(chord)
            || (self.kind == ScaleKind::Major && harmonic.is_diatonic(chord));
        (!self.is_chord_of_key(chord) && in_parallel).then_some(parallel)
    }

    /// Whether the chord is one of the chords of this key: in the
    /// scale, or, in a minor key, the dominant or the leading-tone chord
    /// with the raised seventh of harmonic minor. E major and E7 belong
    /// to A minor, and so does G#dim7: most music in minor keys raises G
    /// to G# for them, so that the leading tone pulls up to A.
    pub fn is_chord_of_key(self, chord: Chord) -> bool {
        self.is_diatonic(chord) || self.uses_raised_seventh(chord)
    }

    /// Whether, in a natural minor key, `chord` is the V or vii° of
    /// harmonic minor: built on the fifth or the raised seventh, with
    /// the raised seventh in it.
    pub fn uses_raised_seventh(self, chord: Chord) -> bool {
        let harmonic = Scale::new(self.root, ScaleKind::HarmonicMinor);
        self.kind == ScaleKind::NaturalMinor
            && !self.is_diatonic(chord)
            && harmonic.is_diatonic(chord)
            && matches!(self.root.semitones_up_to(chord.root), 7 | 11)
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

    fn names(scale: Scale) -> Vec<String> {
        scale
            .pitch_classes()
            .map(|p| scale.spell(p).to_string())
            .collect()
    }

    fn line(scale: Scale) -> String {
        names(scale).join(" ")
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
    fn rows_and_sets_as_scales() {
        let row = crate::harmony::ROW_PRESETS[0].row;
        let scale = Scale::row(row);
        assert_eq!(scale.root, E);
        assert_eq!(scale.tone_row(), Some(row));
        assert_eq!(scale.name(), "row P4 (on E)");
        // Every note is in a row; its degrees are the places in the row.
        assert!(PitchClass::ALL.iter().all(|p| scale.contains(*p)));
        assert_eq!(scale.degree_of(G), Some(2));
        assert!(!scale.is_heptatonic());
        assert_eq!(scale.triad(0), None);

        let diminished = Scale::set(D, PitchSet::from_semitones(&[2, 5, 8, 11]));
        assert_eq!(diminished.name(), "set 4-28 on D");
        assert!(diminished.contains(F) && !diminished.contains(E));
        // A set has no letters of its own: its notes take their usual names.
        assert_eq!(names(diminished), ["D", "F", "Ab", "B"]);
        // The root joins the set if it was not in it.
        let added = Scale::set(C, PitchSet::from_semitones(&[4, 7]));
        assert!(added.contains(C));
        // A seven-note set has no chords of the key.
        let seven = Scale::set(C, Scale::default().pitch_set());
        assert!(!seven.is_heptatonic());
    }

    #[test]
    fn notes_follow_only_between_related_kinds() {
        let row = ScaleKind::Row(ToneRow::CHROMATIC);
        let set = ScaleKind::Set(PitchSet::from_semitones(&[0, 3, 6, 9]));
        assert!(ScaleKind::Major.notes_follow(ScaleKind::Dorian));
        assert!(row.notes_follow(row));
        assert!(!row.notes_follow(ScaleKind::Row(ToneRow::random(3))));
        assert!(!ScaleKind::Major.notes_follow(row));
        assert!(!set.notes_follow(ScaleKind::Major));
        assert!(set.notes_follow(set));
    }

    #[test]
    fn rows_and_sets_in_files() {
        let scale = Scale::set(C, PitchSet::from_semitones(&[0, 3, 6, 9]));
        let text = ron::to_string(&scale).unwrap();
        assert_eq!(text, "(root:C,kind:Set(\"0 3 6 9\"))");
        assert_eq!(ron::from_str::<Scale>(&text).unwrap(), scale);
        let row = Scale::row(ToneRow::CHROMATIC);
        let text = ron::to_string(&row).unwrap();
        assert_eq!(ron::from_str::<Scale>(&text).unwrap(), row);
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
        // Bb major is outside C major, a whole step below home: bVII.
        assert_eq!(
            scale
                .roman_numeral(Chord::new(ASharp, ChordQuality::Major))
                .unwrap(),
            "bVII"
        );
    }

    #[test]
    fn every_letter_once_in_seven_note_scales() {
        assert_eq!(
            line(Scale::new(FSharp, ScaleKind::Major)),
            "F# G# A# B C# D# E#"
        );
        assert_eq!(
            line(Scale::new(DSharp, ScaleKind::NaturalMinor)),
            "D# E# F# G# A# B C#"
        );
        assert_eq!(
            line(Scale::new(F, ScaleKind::Locrian)),
            "F Gb Ab Bb Cb Db Eb"
        );
        assert_eq!(
            line(Scale::new(GSharp, ScaleKind::Dorian)),
            "G# A# B C# D# E# F#"
        );
        assert_eq!(
            line(Scale::new(B, ScaleKind::Lydian)),
            "B C# D# E# F# G# A#"
        );
        for kind in ScaleKind::ALL {
            for root in PitchClass::ALL {
                let scale = Scale::new(root, kind);
                if !scale.is_heptatonic() {
                    continue;
                }
                let mut letters: Vec<_> = scale
                    .pitch_classes()
                    .map(|p| scale.spell(p).letter)
                    .collect();
                letters.sort();
                letters.dedup();
                assert_eq!(letters.len(), 7, "{}", scale.name());
                // Each name still names the right key.
                assert!(
                    scale
                        .pitch_classes()
                        .all(|p| scale.spell(p).pitch_class() == p)
                );
            }
        }
    }

    #[test]
    fn minor_keys_raise_their_sixth_and_seventh_with_sharps() {
        assert_eq!(
            line(Scale::new(D, ScaleKind::HarmonicMinor)),
            "D E F G A Bb C#"
        );
        assert_eq!(
            line(Scale::new(G, ScaleKind::MelodicMinor)),
            "G A Bb C D E F#"
        );
        assert_eq!(
            line(Scale::new(GSharp, ScaleKind::HarmonicMinor)),
            "G# A# B C# D# E F##"
        );
        // Outside the scale, too: C# in D natural minor, G# in A minor.
        let d_minor = Scale::new(D, ScaleKind::NaturalMinor);
        assert_eq!(d_minor.spell(CSharp).to_string(), "C#");
        assert_eq!(d_minor.spell(B).to_string(), "B");
    }

    #[test]
    fn notes_outside_the_key_are_written_from_the_tonic() {
        let c = Scale::default();
        let outside: Vec<String> = [CSharp, DSharp, FSharp, GSharp, ASharp]
            .iter()
            .map(|&p| c.spell(p).to_string())
            .collect();
        assert_eq!(outside, ["Db", "Eb", "F#", "Ab", "Bb"]);
        // No double flats outside the scale: in Db major, D is D.
        assert_eq!(
            Scale::new(CSharp, ScaleKind::Major).spell(D).to_string(),
            "D"
        );
        assert_eq!(
            line(Scale::new(C, ScaleKind::Chromatic)),
            "C Db D Eb E F F# G Ab A Bb B"
        );
    }

    #[test]
    fn pentatonic_and_blues_scales_follow_major_and_minor() {
        assert_eq!(line(Scale::new(A, ScaleKind::Blues)), "A C D Eb E G");
        assert_eq!(line(Scale::new(E, ScaleKind::Blues)), "E G A Bb B D");
        assert_eq!(
            line(Scale::new(FSharp, ScaleKind::MajorPentatonic)),
            "F# G# A# C# D#"
        );
        assert_eq!(
            line(Scale::new(DSharp, ScaleKind::MinorPentatonic)),
            "D# F# G# A# C#"
        );
    }

    #[test]
    fn notes_are_named_in_the_key_with_their_octave() {
        // C# harmonic minor raises its seventh, B, to B#.
        let c_sharp = Scale::new(PitchClass::CSharp, ScaleKind::HarmonicMinor);
        assert_eq!(c_sharp.note_name(Note::new(60)), "B#3");
        assert_eq!(c_sharp.note_name(Note::new(61)), "C#4");
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        assert_eq!(c.note_name(Note::new(61)), "Db4");
        assert_eq!(c.note_name(Note::new(66)), "F#4");
    }

    #[test]
    fn chords_are_spelled_in_the_key() {
        let notes = |scale: Scale, chord| {
            let names: Vec<String> = scale
                .chord_notes(chord)
                .iter()
                .map(|n| n.to_string())
                .collect();
            names.join(" ")
        };
        let d_minor = Scale::new(D, ScaleKind::NaturalMinor);
        assert_eq!(notes(d_minor, Chord::new(A, ChordQuality::Major)), "A C# E");
        assert_eq!(
            notes(d_minor, Chord::new(CSharp, ChordQuality::Diminished7)),
            "C# E G Bb"
        );
        let f_sharp = Scale::new(FSharp, ScaleKind::Major);
        assert_eq!(
            f_sharp.chord_name(Chord::new(F, ChordQuality::Diminished)),
            "E#dim"
        );
        assert_eq!(
            notes(f_sharp, Chord::new(CSharp, ChordQuality::Major)),
            "C# E# G#"
        );
        let c = Scale::default();
        assert_eq!(c.chord_name(Chord::new(DSharp, ChordQuality::Major)), "Eb");
        assert_eq!(c.chord_name(Chord::new(GSharp, ChordQuality::Major)), "Ab");
        assert_eq!(notes(c, Chord::new(E, ChordQuality::Major)), "E G# B");
        // In Db major the chord a semitone up is D major, not Ebb major.
        let d_flat = Scale::new(CSharp, ScaleKind::Major);
        assert_eq!(d_flat.chord_name(Chord::new(D, ChordQuality::Major)), "D");
        assert_eq!(
            d_flat
                .roman_numeral(Chord::new(D, ChordQuality::Major))
                .unwrap(),
            "bII"
        );
        // A set spells a chord as simply as it can.
        let set = Scale::set(C, PitchSet::from_semitones(&[0, 1, 5, 8]));
        assert_eq!(
            set.chord_name(Chord::new(CSharp, ChordQuality::Major)),
            "Db"
        );
    }

    #[test]
    fn numerals_count_along_the_major_scale() {
        let numerals = |root, kind| {
            let scale = Scale::new(root, kind);
            (0..7)
                .map(|degree| scale.roman_numeral(scale.triad(degree).unwrap()).unwrap())
                .collect::<Vec<_>>()
                .join(" ")
        };
        assert_eq!(
            numerals(A, ScaleKind::NaturalMinor),
            "i ii° bIII iv v bVI bVII"
        );
        assert_eq!(
            numerals(A, ScaleKind::HarmonicMinor),
            "i ii° bIII+ iv V bVI vii°"
        );
        assert_eq!(
            numerals(A, ScaleKind::MelodicMinor),
            "i ii bIII+ IV V vi° vii°"
        );
        assert_eq!(
            numerals(E, ScaleKind::Phrygian),
            "i bII bIII iv v° bVI bvii"
        );
        assert_eq!(numerals(C, ScaleKind::Lydian), "I II iii #iv° V vi vii");
        assert_eq!(numerals(G, ScaleKind::Mixolydian), "I ii iii° IV v vi bVII");
        assert_eq!(
            numerals(B, ScaleKind::Locrian),
            "i° bII biii iv bV bVI bvii"
        );
        assert_eq!(numerals(D, ScaleKind::Dorian), "i ii bIII IV v vi° bVII");
    }

    #[test]
    fn minor_keys_own_the_raised_dominant() {
        let a_minor = Scale::new(A, ScaleKind::NaturalMinor);
        assert!(a_minor.is_chord_of_key(Chord::new(E, ChordQuality::Major)));
        assert!(a_minor.is_chord_of_key(Chord::new(E, ChordQuality::Dominant7)));
        assert!(a_minor.is_chord_of_key(Chord::new(GSharp, ChordQuality::Diminished7)));
        assert!(!a_minor.is_chord_of_key(Chord::new(D, ChordQuality::Major)));
        assert!(!Scale::default().is_chord_of_key(Chord::new(E, ChordQuality::Major)));
    }
}
