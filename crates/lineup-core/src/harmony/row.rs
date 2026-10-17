//! Twelve-tone rows: Arnold Schoenberg's way of writing music without a key.
//!
//! In a key, some notes are home and others pull towards them. Around
//! 1921 Schoenberg looked for a way to write music where no note is
//! home: the **twelve-tone technique** (in Spanish, *dodecafonismo*).
//! The composer puts all twelve notes in an order, the **row** (or
//! *series*), and the whole piece is made of that order:
//!
//! - **P**, the prime form: the row as written.
//! - **I**, the inversion: every interval turned upside down. Where the
//!   row goes up a minor third, the inversion goes down one.
//! - **R**, the retrograde: the row backwards.
//! - **RI**, the retrograde inversion: the inversion backwards.
//!
//! Each of the four can start on any of the twelve notes, which makes 48
//! forms. They are numbered by the note they start on (C is 0, C# is 1,
//! and so on): `P4` is the row starting on E, and `I0` its inversion
//! starting on C. A retrograde is numbered like the form it reverses, so
//! `R4` is `P4` backwards and ends on E. (Some books number the forms
//! from the row's first note instead, so the row itself is always `P0`;
//! the forms are the same, only their numbers differ.)
//!
//! The **matrix** shows all 48 forms at once: the row along the top, its
//! inversion down the left side, and every square filled in so that each
//! line read left to right is a P form, right to left an R form, top to
//! bottom an I form and bottom to top an RI form.
//!
//! **Serialism** takes the idea further: after the pitches, the
//! durations, the dynamics and the octaves can each follow a series too.
//! See [`crate::serial`] for writing rows into a pattern that way.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::PitchSet;
use crate::{Note, NoteName, PitchClass, Spelling};

/// A twelve-tone row: the twelve pitch classes, each once, in order.
///
/// Stored as semitones above C (0 to 11). A project file writes those
/// numbers in a line, such as `"0 1 3 9 2 11 4 10 7 8 5 6"`, and reading
/// it checks that the twelve are all different, so a `ToneRow` is always
/// a real row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ToneRow([u8; 12]);

/// The four ways of reading a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RowForm {
    Prime,
    Inversion,
    Retrograde,
    RetrogradeInversion,
}

impl RowForm {
    pub const ALL: [RowForm; 4] = [
        RowForm::Prime,
        RowForm::Inversion,
        RowForm::Retrograde,
        RowForm::RetrogradeInversion,
    ];

    /// The letters of its label: P, I, R or RI.
    pub fn letters(self) -> &'static str {
        match self {
            RowForm::Prime => "P",
            RowForm::Inversion => "I",
            RowForm::Retrograde => "R",
            RowForm::RetrogradeInversion => "RI",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            RowForm::Prime => "prime",
            RowForm::Inversion => "inversion",
            RowForm::Retrograde => "retrograde",
            RowForm::RetrogradeInversion => "retrograde inversion",
        }
    }

    fn is_inverted(self) -> bool {
        matches!(self, RowForm::Inversion | RowForm::RetrogradeInversion)
    }

    fn is_backwards(self) -> bool {
        matches!(self, RowForm::Retrograde | RowForm::RetrogradeInversion)
    }
}

/// One of the 48 forms of a row, such as `P4` or `RI11`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowLabel {
    pub form: RowForm,
    /// The pitch class the form is numbered by, 0 (C) to 11 (B): where
    /// a P or I form starts, and where an R or RI form ends.
    pub number: u8,
}

impl RowLabel {
    pub fn new(form: RowForm, number: u8) -> Self {
        RowLabel {
            form,
            number: number % 12,
        }
    }
}

impl fmt::Display for RowLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.form.letters(), self.number)
    }
}

impl TryFrom<String> for ToneRow {
    type Error = String;

    fn try_from(text: String) -> Result<Self, String> {
        ToneRow::parse(&text)
    }
}

impl From<ToneRow> for String {
    fn from(row: ToneRow) -> Self {
        let numbers: Vec<String> = row.0.iter().map(u8::to_string).collect();
        numbers.join(" ")
    }
}

impl TryFrom<Vec<u8>> for ToneRow {
    type Error = String;

    fn try_from(semitones: Vec<u8>) -> Result<Self, String> {
        let array: [u8; 12] = semitones
            .try_into()
            .map_err(|v: Vec<u8>| format!("a row has 12 notes, not {}", v.len()))?;
        ToneRow::new(array).ok_or_else(|| "a row has each of the 12 notes once".to_string())
    }
}

impl ToneRow {
    /// The row of these pitch classes (semitones above C), or `None`
    /// unless each of the twelve appears exactly once.
    pub fn new(semitones: [u8; 12]) -> Option<Self> {
        let set: PitchSet = PitchSet::from_semitones(&semitones);
        (set.len() == 12 && semitones.iter().all(|&n| n < 12)).then_some(ToneRow(semitones))
    }

    /// The chromatic scale from C, the simplest row there is (and a dull one).
    pub const CHROMATIC: ToneRow = ToneRow([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    /// The pitch classes, in order, as semitones above C.
    pub fn semitones(self) -> [u8; 12] {
        self.0
    }

    pub fn pitch_classes(self) -> [PitchClass; 12] {
        self.0.map(PitchClass::from_semitone)
    }

    /// The note the row starts on.
    pub fn first(self) -> PitchClass {
        PitchClass::from_semitone(self.0[0])
    }

    /// Where in the row `pitch_class` comes, from 0 to 11.
    pub fn position(self, pitch_class: PitchClass) -> usize {
        self.0
            .iter()
            .position(|&n| n == pitch_class.semitone())
            .unwrap_or(0)
    }

    /// The same row moved to start on `start`: the `P` form numbered by
    /// `start`.
    pub fn starting_on(self, start: PitchClass) -> ToneRow {
        self.form(RowLabel::new(RowForm::Prime, start.semitone()))
    }

    /// The row's own label, `P` and the note it starts on.
    pub fn label(self) -> RowLabel {
        RowLabel::new(RowForm::Prime, self.0[0])
    }

    /// The form called `label`: see the module documentation.
    pub fn form(self, label: RowLabel) -> ToneRow {
        let first = self.0[0];
        let n = label.number;
        let mut notes = self.0.map(|note| {
            let up = (note + 12 - first) % 12;
            if label.form.is_inverted() {
                (n + 12 - up) % 12
            } else {
                (n + up) % 12
            }
        });
        if label.form.is_backwards() {
            notes.reverse();
        }
        ToneRow(notes)
    }

    /// Which form of this row `other` is, if it is one. Prime forms are
    /// found first, so a row that is its own retrograde a tritone up (as
    /// Webern liked) is named as the prime form it also is.
    pub fn label_of(self, other: ToneRow) -> Option<RowLabel> {
        RowForm::ALL.into_iter().find_map(|form| {
            (0..12)
                .map(|n| RowLabel::new(form, n))
                .find(|&label| self.form(label) == other)
        })
    }

    /// The 12×12 matrix, as semitones above C. Row `i` read left to
    /// right is a P form; column `j` read top to bottom is an I form.
    /// The first line is the row itself and the first column is its
    /// inversion starting on the same note.
    pub fn matrix(self) -> [[u8; 12]; 12] {
        let inversion = self.form(RowLabel::new(RowForm::Inversion, self.0[0]));
        inversion
            .0
            .map(|start| self.starting_on(PitchClass::from_semitone(start)).0)
    }

    /// The intervals from each note to the next, in semitones going up
    /// (0 to 11): eleven of them.
    pub fn intervals(self) -> [u8; 11] {
        std::array::from_fn(|i| (self.0[i + 1] + 12 - self.0[i]) % 12)
    }

    /// Whether the eleven intervals from note to note are all different:
    /// an **all-interval row**, like the one of Berg's Lyric Suite.
    pub fn is_all_interval(self) -> bool {
        let intervals = self.intervals();
        intervals
            .iter()
            .enumerate()
            .all(|(i, a)| !intervals[i + 1..].contains(a))
    }

    /// The row cut into groups of `size` notes, such as four trichords
    /// (size 3) or two hexachords (size 6). Composers often play each
    /// group as a chord. A last group shorter than `size` is left out.
    pub fn segments(self, size: usize) -> Vec<PitchSet> {
        self.0
            .chunks_exact(size.clamp(1, 12))
            .map(PitchSet::from_semitones)
            .collect()
    }

    /// The forms whose first six notes are exactly the six notes the
    /// row's first half leaves out. Played together with the row, such a
    /// form completes all twelve notes in each half: **hexachordal
    /// combinatoriality**, which Schoenberg used to build his later works.
    /// The retrograde, which always does this, is left out.
    pub fn combinatorial_forms(self) -> Vec<RowLabel> {
        let missing = PitchSet::from_semitones(&self.0[..6]).complement();
        [
            RowForm::Prime,
            RowForm::Inversion,
            RowForm::RetrogradeInversion,
        ]
        .into_iter()
        .flat_map(|form| (0..12).map(move |n| RowLabel::new(form, n)))
        .filter(|&label| PitchSet::from_semitones(&self.form(label).0[..6]) == missing)
        .collect()
    }

    /// The note names, such as "E F G Db Gb Eb Ab D B C A Bb", with the
    /// black keys all sharps or all flats.
    pub fn names(self, spelling: Spelling) -> String {
        self.pitch_classes().map(|p| p.name(spelling)).join(" ")
    }

    /// The note names as twelve-tone music usually writes them, with no
    /// key to follow: "E F G C# F# Eb Ab D B C A Bb". The black keys are
    /// C#, Eb, F#, Ab and Bb (see [`NoteName::chromatic`]).
    pub fn note_names(self) -> String {
        self.pitch_classes()
            .map(|p| NoteName::chromatic(p).to_string())
            .join(" ")
    }

    /// The first twelve different pitch classes among `notes`, in order,
    /// as a row: how to draw a row in the piano roll. Notes that repeat
    /// a pitch class already in the row are skipped.
    pub fn from_notes(notes: impl IntoIterator<Item = Note>) -> Result<ToneRow, String> {
        let mut seen = PitchSet::EMPTY;
        let mut order = Vec::with_capacity(12);
        for note in notes {
            let pitch_class = note.pitch_class();
            if !seen.contains(pitch_class) {
                seen.insert(pitch_class);
                order.push(pitch_class.semitone());
            }
        }
        if order.len() < 12 {
            let missing: Vec<String> = seen
                .complement()
                .iter()
                .map(|p| NoteName::chromatic(p).to_string())
                .collect();
            return Err(format!(
                "A row needs all twelve notes; these have {}. Missing: {}.",
                order.len(),
                missing.join(" ")
            ));
        }
        ToneRow::try_from(order[..12].to_vec())
    }

    /// Reads a row typed as note names ("E F G Db Gb Eb Ab D B C A Bb")
    /// or as numbers, with C as 0 ("4 5 7 1 6 3 8 2 11 0 9 10", or the
    /// compact "45716382E09T", with T and E for ten and eleven). Spaces,
    /// commas and dashes all separate notes.
    pub fn parse(text: &str) -> Result<ToneRow, String> {
        let separators = |c: char| c.is_whitespace() || ",;|-–—".contains(c);
        let tokens: Vec<&str> = text.split(separators).filter(|t| !t.is_empty()).collect();
        let numbers = tokens
            .first()
            .is_some_and(|t| t.starts_with(|c: char| c.is_ascii_digit()));
        // Fewer than twelve groups of digits that hold twelve characters
        // in all are compact: one character a note.
        let characters: usize = tokens.iter().map(|t| t.chars().count()).sum();
        let compact = numbers && tokens.len() < 12 && characters == 12;
        let notes: Vec<u8> = if compact {
            tokens
                .iter()
                .flat_map(|t| t.chars())
                .map(|c| parse_number(&c.to_string()))
                .collect::<Result<_, _>>()?
        } else if numbers {
            tokens
                .iter()
                .map(|t| parse_number(t))
                .collect::<Result<_, _>>()?
        } else {
            tokens
                .iter()
                .map(|t| parse_name(t))
                .collect::<Result<_, _>>()?
        };
        if notes.len() != 12 {
            return Err(format!(
                "A row has twelve notes, and this has {}.",
                notes.len()
            ));
        }
        let mut seen = PitchSet::EMPTY;
        for &note in &notes {
            let pitch_class = PitchClass::from_semitone(note);
            if seen.contains(pitch_class) {
                let missing: Vec<String> = PitchSet::from_semitones(&notes)
                    .complement()
                    .iter()
                    .map(|p| NoteName::chromatic(p).to_string())
                    .collect();
                return Err(format!(
                    "{} comes twice, and {} is missing: each note once.",
                    NoteName::chromatic(pitch_class),
                    missing.join(" ")
                ));
            }
            seen.insert(pitch_class);
        }
        ToneRow::try_from(notes)
    }

    /// A row in a random order, from `seed`: the same seed gives the
    /// same row.
    pub fn random(seed: u64) -> ToneRow {
        let mut random = Random(seed);
        let mut notes = ToneRow::CHROMATIC.0;
        // The Fisher-Yates shuffle: each place in turn gets one of the
        // notes not placed yet, each equally likely.
        for i in (1..12).rev() {
            notes.swap(i, random.below(i + 1));
        }
        ToneRow(notes)
    }

    /// A random all-interval row, from `seed`. There are 3,856 of them
    /// starting on C, out of 39,916,800 rows, so picking rows at random
    /// would rarely find one. Instead the row is built note by note,
    /// each time trying the intervals not used yet in a random order,
    /// and going back a note when none fits ("backtracking").
    pub fn random_all_interval(seed: u64) -> ToneRow {
        let mut random = Random(seed);
        let start = random.below(12) as u8;
        let mut notes = vec![start];
        all_interval_from(&mut notes, &mut random)
            .and_then(|notes| ToneRow::try_from(notes).ok())
            .unwrap_or(ToneRow::CHROMATIC)
    }
}

/// Completes an all-interval row from `notes`, or `None` if it cannot be.
fn all_interval_from(notes: &mut Vec<u8>, random: &mut Random) -> Option<Vec<u8>> {
    if notes.len() == 12 {
        return Some(notes.clone());
    }
    let used: Vec<u8> = notes.windows(2).map(|w| (w[1] + 12 - w[0]) % 12).collect();
    let mut intervals: Vec<u8> = (1..12).filter(|i| !used.contains(i)).collect();
    for i in (1..intervals.len()).rev() {
        intervals.swap(i, random.below(i + 1));
    }
    let last = *notes.last()?;
    for interval in intervals {
        let next = (last + interval) % 12;
        if notes.contains(&next) {
            continue;
        }
        notes.push(next);
        if let Some(row) = all_interval_from(notes, random) {
            return Some(row);
        }
        notes.pop();
    }
    None
}

/// A small random number generator ("SplitMix64"): good enough to
/// shuffle notes, and it needs no library.
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A number from 0 to `n - 1`.
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// A number from 0 to 11, or T/A for ten and E/B for eleven.
fn parse_number(token: &str) -> Result<u8, String> {
    match token {
        "t" | "T" | "a" | "A" => Ok(10),
        "e" | "E" | "b" | "B" => Ok(11),
        _ => match token.parse::<u8>() {
            Ok(n) if n < 12 => Ok(n),
            _ => Err(format!(
                "\"{token}\" is not a note number: use 0 (C) to 11 (B), or T and E."
            )),
        },
    }
}

/// A note name such as "C", "F#", "Bb" or "Ebb", in any case.
fn parse_name(token: &str) -> Result<u8, String> {
    let mut chars = token.chars();
    let letter = chars.next().map(|c| c.to_ascii_uppercase());
    let natural: i32 = match letter {
        Some('C') => 0,
        Some('D') => 2,
        Some('E') => 4,
        Some('F') => 5,
        Some('G') => 7,
        Some('A') => 9,
        Some('B') => 11,
        _ => {
            return Err(format!(
                "\"{token}\" is not a note: use names like C, F# or Bb."
            ));
        }
    };
    let mut semitone = natural;
    for accidental in chars {
        semitone += match accidental {
            '#' | '♯' | 's' => 1,
            'b' | '♭' => -1,
            _ => {
                return Err(format!(
                    "\"{token}\" is not a note: use names like C, F# or Bb."
                ));
            }
        };
    }
    Ok(semitone.rem_euclid(12) as u8)
}

/// A row to start from, with where it comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowPreset {
    pub name: &'static str,
    /// The composer and work, or "Written for Ferrous Lineup".
    pub source: &'static str,
    pub about: &'static str,
    pub row: ToneRow,
}

/// Rows to try: famous ones from works in the public domain, and some
/// written for Ferrous Lineup to show off what rows can do.
///
/// Three of the famous works were published between 1925 and 1929, so in
/// the United States they are in the public domain (95 years after
/// publication), and their composers died in 1951, 1935 and 1945, so they
/// are in the public domain where copyright lasts 70 years after the
/// author's death, too. Webern's Concerto was published in 1948; it is in
/// the public domain where copyright lasts 70 years after the author's
/// death (Webern died in 1945). Only the rows are used here: twelve
/// notes each.
pub const ROW_PRESETS: [RowPreset; 8] = [
    RowPreset {
        name: "Schoenberg's Suite",
        source: "Arnold Schoenberg, Suite for Piano, Op. 25 (1925)",
        about: "One of the first pieces built entirely from one row. Its last four notes, \
                B C A Bb, spell BACH backwards: in German, B is Bb and H is B, so BACH is \
                Bb A C B, and the row ends H C A B.",
        row: ToneRow([4, 5, 7, 1, 6, 3, 8, 2, 11, 0, 9, 10]),
    },
    RowPreset {
        name: "Berg's Lyric Suite",
        source: "Alban Berg, Lyric Suite (1927), first movement",
        about: "An all-interval row: the eleven steps from note to note are all different \
                sizes, with the tritone in the middle.",
        row: ToneRow([5, 4, 0, 9, 7, 2, 8, 1, 3, 6, 10, 11]),
    },
    RowPreset {
        name: "Webern's Symphony",
        source: "Anton Webern, Symphony, Op. 21 (1929)",
        about: "A palindrome: read backwards it is the same row a tritone higher (R9 is P3). \
                The second half mirrors the first.",
        row: ToneRow([9, 6, 7, 8, 4, 5, 11, 10, 2, 1, 0, 3]),
    },
    RowPreset {
        name: "Webern's Concerto",
        source: "Anton Webern, Concerto for nine instruments, Op. 24 (1934)",
        about: "Four groups of three, all the same micromode, Menor 3 (a semitone and two \
                thirds): B Bb D, Eb G F#, Ab E F, C C# A. The second group is the first \
                backwards and upside down (RI), the third backwards (R), the fourth upside \
                down (I).",
        row: ToneRow([11, 10, 2, 3, 7, 6, 8, 4, 5, 0, 1, 9]),
    },
    RowPreset {
        name: "Lineup row",
        source: "Written for Ferrous Lineup",
        about: "Hexachordally combinatorial: its first six notes and the first six of I3 \
                make all twelve, so the two forms can be played together without a note \
                repeating before all twelve have sounded.",
        row: ToneRow([0, 2, 6, 8, 5, 4, 9, 1, 11, 3, 7, 10]),
    },
    RowPreset {
        name: "Four triads",
        source: "Written for Ferrous Lineup",
        about: "Four triads in a row (C major, D major, Bb minor, G# minor) that still use \
                every note once: twelve-tone music can sound almost tonal, as in Berg.",
        row: ToneRow([0, 4, 7, 2, 6, 9, 10, 1, 5, 11, 3, 8]),
    },
    RowPreset {
        name: "Mirror row",
        source: "Written for Ferrous Lineup",
        about: "Like Webern's, a palindrome: backwards it is the same row a tritone higher.",
        row: ToneRow([0, 4, 3, 7, 8, 11, 5, 2, 1, 9, 10, 6]),
    },
    RowPreset {
        name: "All the intervals",
        source: "Written for Ferrous Lineup",
        about: "An all-interval row: every step from one note to the next is a different size.",
        row: ToneRow([0, 2, 5, 4, 11, 9, 3, 7, 8, 1, 10, 6]),
    },
];

impl RowPreset {
    pub fn named(name: &str) -> Option<&'static RowPreset> {
        ROW_PRESETS.iter().find(|preset| preset.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PitchClass::*;

    fn row(text: &str) -> ToneRow {
        ToneRow::parse(text).unwrap()
    }

    #[test]
    fn the_four_forms() {
        let schoenberg = RowPreset::named("Schoenberg's Suite").unwrap().row;
        assert_eq!(schoenberg.label().to_string(), "P4");
        assert_eq!(
            schoenberg.names(Spelling::Flats),
            "E F G Db Gb Eb Ab D B C A Bb"
        );
        let label = |form, n| RowLabel::new(form, n);
        // The inversion starting on E: every step turned upside down.
        assert_eq!(
            schoenberg
                .form(label(RowForm::Inversion, 4))
                .names(Spelling::Flats),
            "E Eb Db G D F C Gb A Ab B Bb"
        );
        // R4 is P4 backwards, and ends on E.
        assert_eq!(
            schoenberg
                .form(label(RowForm::Retrograde, 4))
                .names(Spelling::Flats),
            "Bb A C B D Ab Eb Gb Db G F E"
        );
        assert_eq!(
            schoenberg
                .form(label(RowForm::RetrogradeInversion, 4))
                .names(Spelling::Flats),
            "Bb B Ab A Gb C F D G Db Eb E"
        );
        // P0 is the row moved down to start on C.
        assert_eq!(
            schoenberg.form(label(RowForm::Prime, 0)).semitones(),
            [0, 1, 3, 9, 2, 11, 4, 10, 7, 8, 5, 6]
        );
    }

    #[test]
    fn every_form_is_found_again() {
        let lineup = RowPreset::named("Lineup row").unwrap().row;
        for form in RowForm::ALL {
            for n in 0..12 {
                let label = RowLabel::new(form, n);
                assert_eq!(lineup.label_of(lineup.form(label)), Some(label));
            }
        }
        assert_eq!(lineup.label_of(ToneRow::CHROMATIC), None);
    }

    #[test]
    fn the_matrix_holds_every_form() {
        let berg = RowPreset::named("Berg's Lyric Suite").unwrap().row;
        let matrix = berg.matrix();
        assert_eq!(matrix[0], berg.semitones());
        for (i, line) in matrix.iter().enumerate() {
            // Each line is a P form, and each column an I form.
            let p = ToneRow::new(*line).unwrap();
            assert_eq!(berg.label_of(p).unwrap().form, RowForm::Prime);
            let column: [u8; 12] = std::array::from_fn(|j| matrix[j][i]);
            let label = berg.label_of(ToneRow::new(column).unwrap()).unwrap();
            assert_eq!(label.form, RowForm::Inversion);
        }
        // The diagonal of a matrix built this way is the row's first note.
        assert!((0..12).all(|i| matrix[i][i] == berg.semitones()[0]));
    }

    #[test]
    fn webern_is_a_palindrome() {
        let webern = RowPreset::named("Webern's Symphony").unwrap().row;
        let r9 = webern.form(RowLabel::new(RowForm::Retrograde, 9));
        assert_eq!(webern.label_of(r9).unwrap().to_string(), "P3");
        let mirror = RowPreset::named("Mirror row").unwrap().row;
        let backwards = mirror.form(RowLabel::new(RowForm::Retrograde, 0));
        assert_eq!(mirror.label_of(backwards).unwrap().to_string(), "P6");
    }

    #[test]
    fn all_interval_rows() {
        assert!(
            RowPreset::named("Berg's Lyric Suite")
                .unwrap()
                .row
                .is_all_interval()
        );
        assert!(
            RowPreset::named("All the intervals")
                .unwrap()
                .row
                .is_all_interval()
        );
        assert!(!ToneRow::CHROMATIC.is_all_interval());
        for seed in 0..50 {
            let row = ToneRow::random_all_interval(seed);
            assert!(row.is_all_interval(), "seed {seed}: {row:?}");
        }
    }

    #[test]
    fn combinatoriality() {
        let lineup = RowPreset::named("Lineup row").unwrap().row;
        let forms: Vec<String> = lineup
            .combinatorial_forms()
            .iter()
            .map(|l| l.to_string())
            .collect();
        assert_eq!(forms, ["I3"]);
        // The chromatic hexachord is combinatorial with all four forms.
        assert!(ToneRow::CHROMATIC.combinatorial_forms().len() >= 3);
    }

    #[test]
    fn segments() {
        let triads = RowPreset::named("Four triads").unwrap().row;
        let names: Vec<String> = triads
            .segments(3)
            .iter()
            .map(|set| set.set_class().name())
            .collect();
        assert_eq!(names, ["3-11", "3-11", "3-11", "3-11"]);
        assert_eq!(triads.segments(6).len(), 2);
        assert_eq!(triads.segments(5).len(), 2);
    }

    #[test]
    fn reading_rows() {
        let by_name = row("E F G Db Gb Eb Ab D B C A Bb");
        assert_eq!(row("4 5 7 1 6 3 8 2 11 0 9 10"), by_name);
        assert_eq!(row("4,5,7,1,6,3,8,2,E,0,9,T"), by_name);
        assert_eq!(row("457163 82E09T"), by_name, "two compact halves");
        assert_eq!(row("e-f-g-c#-f#-d#-g#-d-b-c-a-a#"), by_name);
        assert!(ToneRow::parse("C D E").unwrap_err().contains("this has 3"));
        let twice = ToneRow::parse("C C D D# E F F# G G# A A# B").unwrap_err();
        assert!(twice.contains("C comes twice"), "{twice}");
        assert!(twice.contains("C#"), "{twice}");
        let schoenberg = RowPreset::named("Schoenberg's Suite").unwrap().row;
        assert_eq!(schoenberg.note_names(), "E F G C# F# Eb Ab D B C A Bb");
        assert!(ToneRow::parse("C H D").is_err());
    }

    #[test]
    fn drawn_rows() {
        let notes = [60, 62, 60, 61, 75, 64, 65, 66, 67, 68, 69, 70, 71, 72].map(Note::new);
        let row = ToneRow::from_notes(notes).unwrap();
        assert_eq!(row.semitones(), [0, 2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let short = ToneRow::from_notes([Note::C4]).unwrap_err();
        assert!(short.contains("Missing"), "{short}");
    }

    #[test]
    fn random_rows_are_rows() {
        for seed in 0..100 {
            assert!(ToneRow::new(ToneRow::random(seed).semitones()).is_some());
        }
        assert_ne!(ToneRow::random(1), ToneRow::random(2));
        assert_eq!(ToneRow::random(7), ToneRow::random(7));
    }

    #[test]
    fn rows_in_files() {
        let text = ron::to_string(&ToneRow::CHROMATIC).unwrap();
        assert_eq!(text, "\"0 1 2 3 4 5 6 7 8 9 10 11\"");
        assert!(ron::from_str::<ToneRow>("\"0 0 2 3 4 5 6 7 8 9 10 11\"").is_err());
        assert!(ron::from_str::<ToneRow>("\"0 1\"").is_err());
        assert_eq!(ToneRow::CHROMATIC.starting_on(E).first(), E);
        assert_eq!(ToneRow::CHROMATIC.position(E), 4);
    }

    #[test]
    fn presets_are_what_they_say() {
        let triads = RowPreset::named("Four triads").unwrap().row;
        assert_eq!(triads.note_names(), "C E G D F# A Bb C# F B Eb Ab");
        for preset in ROW_PRESETS {
            assert!(
                ToneRow::new(preset.row.semitones()).is_some(),
                "{}",
                preset.name
            );
        }
    }
}
