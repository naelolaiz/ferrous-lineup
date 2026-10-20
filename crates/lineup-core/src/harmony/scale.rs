//! Scales and keys: which notes belong together.

use std::cell::Cell;

use serde::{Deserialize, Serialize};

use super::{Chord, ChordQuality, PitchSet, ToneRow};
use crate::{Note, NoteName, PitchClass, Spelling};

/// How Roman numerals count the degrees of a minor key. Both ways are
/// correct, and books use one or the other; major keys are written the
/// same way by both.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MinorNumerals {
    /// From the major scale on the same tonic: i, ii°, bIII, iv, v, bVI,
    /// bVII. A flat marks each degree that is a semitone below the major
    /// scale's, so one table of progressions works in every mode. Most
    /// pop and jazz books write numerals this way.
    #[default]
    FromMajor,
    /// From the minor scale itself: i, ii°, III, iv, v, VI, VII, as
    /// classical harmony books write them. The sixth and seventh
    /// degrees have two forms in minor, and both are written without an
    /// accidental: in A minor, G major is VII and the leading-tone chord
    /// G#dim is vii°, F major is VI and F#dim (of melodic minor) vi°.
    FromMinor,
}

thread_local! {
    /// How the numerals written on this thread count a minor key's
    /// degrees. The window's thread sets it from the settings each frame;
    /// each thread has its own, so tests running side by side never see
    /// each other's.
    static MINOR_NUMERALS: Cell<MinorNumerals> = const { Cell::new(MinorNumerals::FromMajor) };
}

impl MinorNumerals {
    /// How numerals are written on this thread.
    pub fn current() -> MinorNumerals {
        MINOR_NUMERALS.with(Cell::get)
    }

    /// Writes numerals this way on this thread from now on.
    pub fn set(self) {
        MINOR_NUMERALS.with(|numerals| numerals.set(self));
    }

    /// Runs `f` with numerals written this way, then goes back to the way
    /// they were, even if `f` panics.
    pub fn with<T>(self, f: impl FnOnce() -> T) -> T {
        struct Restore(MinorNumerals);
        impl Drop for Restore {
            fn drop(&mut self) {
                self.0.set();
            }
        }
        let _restore = Restore(MinorNumerals::current());
        self.set();
        f()
    }
}

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
    pub(crate) fn relative_major(self) -> u8 {
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
            // F# major and Gb major are the same keys of the piano, and
            // the tonic's name decides between them. Their modes are
            // named with sharps (G# Dorian, B Lydian), except F Locrian,
            // whose notes are those of Gb major: F Gb Ab Bb Cb Db Eb.
            FSharp if self.root == F => Spelling::Flats,
            _ => Spelling::Sharps,
        }
    }

    /// The key signature this key is written with: how many sharps (or
    /// flats, counted as negative), and whether the key is minor. The
    /// modes take the signature of the major key they come from, so D
    /// Dorian has none, like C major. A chromatic scale, a row or a set
    /// has no key signature.
    ///
    /// The signature follows the names the app gives keys (see
    /// [`Scale::spelling`]): F# major has six sharps, Db major five flats.
    pub fn key_signature(self) -> Option<(i8, bool)> {
        if !self.kind.is_tonal() || self.kind == ScaleKind::Chromatic {
            return None;
        }
        let major = self.root.transpose(self.kind.relative_major() as i32);
        // Going round the circle of fifths: each fifth up adds a sharp.
        let fifths = (0..12i8)
            .find(|&n| PitchClass::C.transpose(7 * i32::from(n)) == major)
            .unwrap_or(0);
        let sharps = match self.spelling() {
            Spelling::Flats => fifths - 12,
            Spelling::Sharps => fifths,
        };
        let minor = self.kind.relative_major() == 3;
        Some((sharps, minor))
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
    /// so its notes take their usual names (see [`NoteName::chromatic`]),
    /// except a set that is a scale, which is spelled as that scale.
    pub fn spell(self, pitch_class: PitchClass) -> NoteName {
        if let Some(scale) = self.set_as_scale() {
            return scale.spell(pitch_class);
        }
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

    /// The key this scale names: itself, or for a pitch-class set whose
    /// notes are exactly a scale on its root, that scale. Switching C
    /// major to "pitch-class set" gives the set 7-35 on C, the same notes
    /// under another name, so a change from it to C minor must move the
    /// song as from C major (see [`crate::tools::change_key`]).
    pub fn as_key(self) -> Scale {
        self.set_as_scale().unwrap_or(self)
    }

    /// The scale a pitch-class set is, if its notes are exactly those of
    /// one of the scales of tonal music on its root: the set 7-35 on C is
    /// C major, and on D it is D Dorian. Such a set is spelled as that
    /// scale, so E major as a set keeps its G#, C# and D# rather than
    /// the usual Ab and Eb.
    fn set_as_scale(self) -> Option<Scale> {
        let ScaleKind::Set(set) = self.kind else {
            return None;
        };
        let semitones = set.semitones();
        ScaleKind::ALL
            .into_iter()
            .filter(|&kind| kind != ScaleKind::Chromatic)
            .find(|kind| kind.tonal_intervals() == semitones.as_slice())
            .map(|kind| Scale::new(self.root, kind))
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
        if let Some(scale) = self.set_as_scale() {
            return scale.spell_root(chord);
        }
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
    /// is one Ferrous Lineup knows. On some degrees the ninth is only a
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
    ///
    /// Minor keys can be written from their own scale instead (III, VI,
    /// VII), as [`MinorNumerals::current`] says.
    pub fn roman_numeral(self, chord: Chord) -> Option<String> {
        self.numeral_on(self.letters_above_tonic(chord.root), chord)
    }

    /// The Roman numeral of `chord` written with the root `root`. The
    /// letter says which degree the chord stands on, so in C major
    /// C#°7, a raised tonic, is "#i°7", and Db7 is "bII7"; from the sound
    /// alone ([`Scale::roman_numeral`]) both would be on the second
    /// degree.
    pub fn spelled_numeral(self, root: NoteName, chord: Chord) -> Option<String> {
        self.numeral_on(self.tonic_name().letter.steps_up_to(root.letter), chord)
    }

    /// [`Scale::spelled_numeral`] with the figures of an inversion, as
    /// [`Scale::figured_numeral`] gives them.
    pub fn spelled_figured_numeral(
        self,
        root: NoteName,
        chord: Chord,
        bass: PitchClass,
    ) -> Option<String> {
        let numeral = self.spelled_numeral(root, chord)?;
        Some(with_figures(&numeral, chord.inversion_figures(bass)))
    }

    /// The Roman numeral of `chord` on the degree `degree` (0 for the
    /// tonic), with or without accidentals: bVII for Bb in C major.
    pub(crate) fn numeral_on_degree(self, degree: usize, chord: Chord) -> Option<String> {
        self.numeral_on(u8::try_from(degree).ok().filter(|&d| d < 7)?, chord)
    }

    /// The Roman numeral of `chord` on the degree `letters` letters above
    /// the tonic.
    fn numeral_on(self, letters: u8, chord: Chord) -> Option<String> {
        const NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
        if !self.is_heptatonic() {
            return None;
        }
        let letters = usize::from(letters);
        let unaltered =
            self.unaltered_degree(letters, chord.quality, MinorNumerals::current()) as i8;
        let semitones = self.root.semitones_up_to(chord.root) as i8;
        // How far the root is from the note the numeral names without an
        // accidental, from -6 to 5.
        let shift = (semitones - unaltered + 18).rem_euclid(12) - 6;
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

    /// Whether this is a minor key, natural, harmonic or melodic: the
    /// keys whose numerals [`MinorNumerals`] counts in two ways. The
    /// minor modes, such as Dorian, are counted from the major scale.
    pub fn is_minor_key(self) -> bool {
        use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
        matches!(self.kind, NaturalMinor | HarmonicMinor | MelodicMinor)
    }

    /// The note a numeral on the degree `letters` letters above the tonic
    /// names when it has no sharp or flat, in semitones above the tonic,
    /// for a chord of `quality`: the major scale's note, or in a minor
    /// key written [`MinorNumerals::FromMinor`], the minor scale's.
    ///
    /// The sixth and seventh degrees of a minor key have two forms, as
    /// in A minor F and F#, G and G#. A numeral without an accidental
    /// names the form whose chord belongs to the key: VII is G major,
    /// from natural minor, but vii° is G#dim, from harmonic minor, since
    /// G B Db is no chord of A minor. When neither form makes a chord of
    /// the key, it names the natural one.
    pub(crate) fn unaltered_degree(
        self,
        letters: usize,
        quality: ChordQuality,
        numerals: MinorNumerals,
    ) -> u8 {
        use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
        if numerals == MinorNumerals::FromMajor || !self.is_minor_key() {
            return ScaleKind::Major.tonal_intervals()[letters];
        }
        let natural = NaturalMinor.tonal_intervals()[letters];
        if letters < 5 {
            return natural;
        }
        // Whether the chord on `semitones` is in one of these scales,
        // counted from C, since only the distance from the tonic counts.
        let of_key = |semitones: u8, kinds: &[ScaleKind]| {
            let chord = Chord::new(PitchClass::C.transpose(i32::from(semitones)), quality);
            kinds
                .iter()
                .any(|&kind| Scale::new(PitchClass::C, kind).is_diatonic(chord))
        };
        if !of_key(natural, &[NaturalMinor]) && of_key(natural + 1, &[HarmonicMinor, MelodicMinor])
        {
            natural + 1
        } else {
            natural
        }
    }

    /// The Roman numeral of `chord` with `bass` lowest, with the figures
    /// of figured bass for an inversion: C/E in C major is "I⁶", G7/F is
    /// "V⁴₂". The figures stand in for a seventh chord's 7 ("V⁶₅", not
    /// "V7⁶₅"). In root position it is [`Scale::roman_numeral`].
    pub fn figured_numeral(self, chord: Chord, bass: PitchClass) -> Option<String> {
        let numeral = self.roman_numeral(chord)?;
        Some(with_figures(&numeral, chord.inversion_figures(bass)))
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

/// A numeral with inversion figures put in place of its 7: "V7" and
/// "⁶₅" make "V⁶₅", "vii°7" and "⁴₂" make "vii°⁴₂", "I" and "⁶" make
/// "I⁶". Used for secondary dominants too: "V7/vi" written as "V⁶₅/vi".
pub fn with_figures(numeral: &str, figures: &str) -> String {
    if figures.is_empty() {
        return numeral.to_string();
    }
    // A secondary dominant's figures go on its own numeral, before the
    // slash: V⁶₅/vi.
    let (own, target) = match numeral.split_once('/') {
        Some((own, target)) => (own, format!("/{target}")),
        None => (numeral, String::new()),
    };
    let own = own.strip_suffix('7').unwrap_or(own);
    format!("{own}{figures}{target}")
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
    fn a_set_that_is_a_scale_is_spelled_as_the_scale() {
        let names = |scale: Scale| -> Vec<String> {
            scale
                .pitch_classes()
                .map(|p| scale.spell(p).to_string())
                .collect()
        };
        for (root, kind) in [
            (PitchClass::E, ScaleKind::Major),
            (PitchClass::CSharp, ScaleKind::Major),
            (PitchClass::DSharp, ScaleKind::NaturalMinor),
            (PitchClass::A, ScaleKind::HarmonicMinor),
            (PitchClass::GSharp, ScaleKind::Major),
            (PitchClass::F, ScaleKind::Major),
            (PitchClass::D, ScaleKind::Dorian),
            (PitchClass::E, ScaleKind::MajorPentatonic),
        ] {
            let key = Scale::new(root, kind);
            let set = Scale::set(root, key.pitch_set());
            assert_eq!(names(set), names(key), "{}", key.name());
            assert_eq!(set.spell(root), key.spell(root));
        }
        assert_eq!(
            names(Scale::set(
                PitchClass::E,
                Scale::new(PitchClass::E, ScaleKind::Major).pitch_set()
            )),
            ["E", "F#", "G#", "A", "B", "C#", "D#"]
        );
        // The same seven notes on D are D Dorian.
        let on_d = Scale::set(PitchClass::D, Scale::default().pitch_set());
        assert_eq!(names(on_d), ["D", "E", "F", "G", "A", "B", "C"]);
        // A set that is no scale keeps the usual names.
        let diminished = Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 3, 6, 9]));
        assert_eq!(names(diminished), ["C", "Eb", "F#", "A"]);
    }

    #[test]
    fn a_set_that_is_a_scale_names_that_key() {
        let c_major = Scale::new(C, ScaleKind::Major);
        assert_eq!(Scale::set(C, c_major.pitch_set()).as_key(), c_major);
        let d_dorian = Scale::new(PitchClass::D, ScaleKind::Dorian);
        assert_eq!(
            Scale::set(PitchClass::D, c_major.pitch_set()).as_key(),
            d_dorian
        );
        let diminished = Scale::set(C, PitchSet::from_semitones(&[0, 3, 6, 9]));
        assert_eq!(diminished.as_key(), diminished);
        assert_eq!(c_major.as_key(), c_major);
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
    fn written_chords_are_numbered_by_their_letter() {
        let numeral = |key: Scale, text: &str| {
            let symbol: super::super::ChordSymbol = text.parse().unwrap();
            key.spelled_numeral(symbol.root, symbol.chord()).unwrap()
        };
        let c_major = Scale::new(C, ScaleKind::Major);
        // The same keys of the piano, two degrees: a raised tonic leading
        // to ii, and a Neapolitan-like flat second.
        assert_eq!(numeral(c_major, "C#dim7"), "#i°7");
        assert_eq!(numeral(c_major, "Db7"), "bII7");
        assert_eq!(numeral(c_major, "D#dim7"), "#ii°7");
        assert_eq!(numeral(c_major, "Ebmaj7"), "bIIImaj7");
        assert_eq!(numeral(c_major, "G#dim"), "#v°");
        assert_eq!(numeral(c_major, "Gb7"), "bV7");
        assert_eq!(numeral(c_major, "F#m7b5"), "#ivø7");
        // The key's own chords, double sharps included.
        assert_eq!(numeral(c_major, "G7"), "V7");
        let g_sharp_minor = Scale::new(GSharp, ScaleKind::HarmonicMinor);
        assert_eq!(numeral(g_sharp_minor, "F##dim7"), "vii°7");
        assert_eq!(numeral(g_sharp_minor, "D#7"), "V7");
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
    fn minor_keys_can_be_numbered_from_their_own_scale() {
        let numerals = |root, kind, size: fn(Scale, usize) -> Option<Chord>| {
            let scale = Scale::new(root, kind);
            (0..7)
                .map(|degree| scale.roman_numeral(size(scale, degree).unwrap()).unwrap())
                .collect::<Vec<_>>()
                .join(" ")
        };
        MinorNumerals::FromMinor.with(|| {
            assert_eq!(
                numerals(A, ScaleKind::NaturalMinor, Scale::triad),
                "i ii° III iv v VI VII"
            );
            assert_eq!(
                numerals(A, ScaleKind::HarmonicMinor, Scale::triad),
                "i ii° III+ iv V VI vii°"
            );
            assert_eq!(
                numerals(A, ScaleKind::MelodicMinor, Scale::triad),
                "i ii III+ IV V vi° vii°"
            );
            assert_eq!(
                numerals(A, ScaleKind::NaturalMinor, Scale::seventh),
                "i7 iiø7 IIImaj7 iv7 v7 VImaj7 VII7"
            );
            assert_eq!(
                numerals(A, ScaleKind::HarmonicMinor, Scale::seventh),
                "imaj7 iiø7 III+maj7 iv7 V7 VImaj7 vii°7"
            );
            // Major keys and the modes are written as before.
            assert_eq!(
                numerals(C, ScaleKind::Major, Scale::triad),
                "I ii iii IV V vi vii°"
            );
            assert_eq!(
                numerals(E, ScaleKind::Phrygian, Scale::triad),
                "i bII bIII iv v° bVI bvii"
            );
            let a_minor = Scale::new(A, ScaleKind::NaturalMinor);
            let numeral = |root, quality| a_minor.roman_numeral(Chord::new(root, quality)).unwrap();
            // A flat lowers the root from the form the bare numeral names:
            // Bb is the Neapolitan bII, and G B Db is a G#dim lowered.
            assert_eq!(numeral(ASharp, ChordQuality::Major), "bII");
            assert_eq!(numeral(G, ChordQuality::Diminished), "bvii°");
            // Raised, in neither form of the key: a sharp.
            assert_eq!(numeral(GSharp, ChordQuality::Major), "#VII");
            assert_eq!(numeral(FSharp, ChordQuality::Minor), "#vi");
            assert_eq!(numeral(CSharp, ChordQuality::Major), "#III");
            // The dominant of III is VII7 written as a secondary
            // dominant too: V7/III.
            assert_eq!(numeral(G, ChordQuality::Dominant7), "VII7");
        });
        // Back as it was afterwards.
        assert_eq!(
            numerals(A, ScaleKind::NaturalMinor, Scale::triad),
            "i ii° bIII iv v bVI bVII"
        );
    }

    #[test]
    fn numerals_from_the_minor_scale_read_back_as_the_same_chords() {
        use crate::library::numeral;
        use ChordQuality::*;
        let qualities = [
            Major,
            Minor,
            Diminished,
            Augmented,
            Dominant7,
            Major7,
            Minor7,
            Diminished7,
            HalfDiminished7,
        ];
        for numerals in [MinorNumerals::FromMajor, MinorNumerals::FromMinor] {
            numerals.with(|| {
                for kind in [
                    ScaleKind::NaturalMinor,
                    ScaleKind::HarmonicMinor,
                    ScaleKind::MelodicMinor,
                    ScaleKind::Major,
                ] {
                    for tonic in PitchClass::ALL {
                        let key = Scale::new(tonic, kind);
                        for up in 0..12 {
                            for quality in qualities {
                                let chord = Chord::new(tonic.transpose(up), quality);
                                let written = key.roman_numeral(chord).unwrap();
                                let read = numeral::parse_in(&written, key, numerals)
                                    .unwrap_or_else(|| panic!("{written} does not read"));
                                assert_eq!(
                                    read.in_key(tonic),
                                    chord,
                                    "{written} in {key:?}, {numerals:?}"
                                );
                            }
                        }
                    }
                }
            });
        }
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

    #[test]
    fn every_mode_is_spelled_as_its_key_signature_says() {
        // The sharps or flats a key's notes are written with are those of
        // its signature, so a staff needs no accidental for them.
        let modes = [
            ScaleKind::Major,
            ScaleKind::Dorian,
            ScaleKind::Phrygian,
            ScaleKind::Lydian,
            ScaleKind::Mixolydian,
            ScaleKind::NaturalMinor,
            ScaleKind::Locrian,
        ];
        for kind in modes {
            for root in PitchClass::ALL {
                let scale = Scale::new(root, kind);
                let (signature, _) = scale.key_signature().unwrap();
                let written: i8 = scale
                    .pitch_classes()
                    .map(|note| scale.spell(note).accidental)
                    .sum();
                assert_eq!(written, signature, "{}", scale.name());
            }
        }
    }

    #[test]
    fn signatures_of_all_keys() {
        let signatures = |kind| -> Vec<Option<(i8, bool)>> {
            (0..12)
                .map(|n| Scale::new(PitchClass::C.transpose(n), kind).key_signature())
                .collect()
        };
        // Sharps count up, flats count down, round the circle of fifths:
        // C, Db, D, Eb, E, F, F#, G, Ab, A, Bb, B.
        let major = [0, -5, 2, -3, 4, -1, 6, 1, -4, 3, -2, 5];
        assert_eq!(
            signatures(ScaleKind::Major),
            major.map(|sharps| Some((sharps, false)))
        );
        // A minor key has the signature of its relative major, 3
        // semitones up: C minor has the three flats of Eb major. The
        // harmonic and melodic minors are written with the same one.
        let minor = [-3, 4, -1, 6, 1, -4, 3, -2, 5, 0, -5, 2];
        for kind in [
            ScaleKind::NaturalMinor,
            ScaleKind::HarmonicMinor,
            ScaleKind::MelodicMinor,
        ] {
            assert_eq!(
                signatures(kind),
                minor.map(|sharps| Some((sharps, true))),
                "{kind:?}"
            );
        }
        // F Locrian has the notes, and the six flats, of Gb major.
        assert_eq!(
            Scale::new(PitchClass::F, ScaleKind::Locrian).key_signature(),
            Some((-6, false))
        );
        // D Dorian uses the white keys, like C major.
        assert_eq!(
            Scale::new(PitchClass::D, ScaleKind::Dorian).key_signature(),
            Some((0, false))
        );
        assert_eq!(
            Scale::new(PitchClass::C, ScaleKind::Chromatic).key_signature(),
            None
        );
    }
}
