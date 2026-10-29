//! Pitch-class sets: a name for every group of notes, chord or not.
//!
//! Chord names work for the chords of tonal music: C major, Am7. Much
//! music of the 20th century uses groups of notes that have no chord
//! name, such as C, C# and F#. Pitch-class set theory, worked out by
//! Milton Babbitt, Allen Forte and John Rahn among others, names every
//! group of notes, by asking what stays the same when you move it.
//!
//! - **Pitch classes as numbers.** C is 0, C# is 1, up to B, 11.
//!   Octaves and spellings do not matter: every C is 0.
//! - **Transposition** (`T3`) moves every note up some semitones:
//!   C E G becomes Eb G Bb. It is the same chord, a minor third higher.
//! - **Inversion** (`I`) turns every interval upside down: C E G
//!   (up 4, up 3) becomes C Ab F (down 4, down 3), which is F minor.
//!   Upside down, a major triad is a minor triad.
//! - **Normal form** writes a set in its most compact order, so two
//!   orderings of the same notes look the same.
//! - **Prime form** is the most compact form of all the transpositions
//!   and inversions of a set, starting on 0. All major and minor triads
//!   share the prime form [0,3,7]. They are one **set class**.
//! - **Forte numbers** name the 224 set classes: 3-11 is the class of
//!   triads (three notes, number 11 in Allen Forte's list).
//! - The **interval vector** counts the intervals a set contains: how
//!   many semitones, whole tones, minor thirds, major thirds, fourths
//!   and tritones. It is what a set sounds like, in six numbers.
//!
//! This module finds these with the bits of [`PitchSet`]: a set is a
//! 12-bit number, transposing rotates the bits, and the most compact
//! form turns out to be simply the smallest number.

use super::PitchSet;
use crate::PitchClass;

impl PitchSet {
    /// The inversion around C (`I0`): every pitch class `n` becomes
    /// `12 - n`. C E G becomes C Ab F.
    pub fn invert(self) -> PitchSet {
        self.iter()
            .map(|pitch_class| PitchClass::from_semitone(12 - pitch_class.semitone()))
            .collect()
    }

    /// The pitch classes not in the set.
    pub fn complement(self) -> PitchSet {
        PitchSet::from_bits(!self.bits())
    }

    /// The set in **normal form**: its pitch classes in the order that
    /// packs them into the smallest span, starting on the note that
    /// does so.
    ///
    /// Written in a circle, a set can start on any of its notes. Normal
    /// form starts on the one that leaves the smallest gap from first to
    /// last note. On a tie, it compares the gap from the first note to
    /// the one before last, and so on (John Rahn's rule). Seen as
    /// numbers, that is the starting note that makes the set, moved down
    /// to start on C, the smallest number: comparing two numbers looks
    /// at their highest bits first, which are their last notes.
    pub fn normal_form(self) -> Vec<PitchClass> {
        let Some(start) = self
            .iter()
            .min_by_key(|start| self.relative_to(*start).bits())
        else {
            return Vec::new();
        };
        self.relative_to(start)
            .iter()
            .map(|pitch_class| pitch_class.transpose(start.semitone() as i32))
            .collect()
    }

    /// The **prime form**: the most compact of the set's transpositions
    /// and inversions, as a set starting on C. Sets with the same prime
    /// form belong to the same set class, and sound alike.
    ///
    /// It is the smallest number among the 24 sets you get by moving the
    /// set and its inversion to each of the 12 starting notes.
    pub fn prime_form(self) -> PitchSet {
        let inverted = self.invert();
        (0..12)
            .flat_map(|n| [self.transpose(n), inverted.transpose(n)])
            .min_by_key(|set| set.bits())
            .unwrap_or(self)
    }

    /// The **interval vector**: how many of each interval the set holds,
    /// counting every pair of notes once. The six numbers count minor
    /// seconds, major seconds, minor thirds, major thirds, perfect
    /// fourths and tritones. An interval and its inversion count as the
    /// same one (a fifth is a fourth upside down), so six are enough.
    ///
    /// C major (C E G) has a minor third, a major third and a fourth
    /// (the fifth C to G): `[0, 0, 1, 1, 1, 0]`.
    pub fn interval_vector(self) -> [u8; 6] {
        let notes = self.semitones();
        let mut vector = [0; 6];
        for (i, low) in notes.iter().enumerate() {
            for high in &notes[i + 1..] {
                let interval = high - low;
                let class = interval.min(12 - interval);
                vector[class as usize - 1] += 1;
            }
        }
        vector
    }

    /// How to turn `self` into `other` by moving it: `Some((n, false))`
    /// if `other` is `self` transposed up `n` semitones (`Tn`), or
    /// `Some((n, true))` if it is `self` inverted, then transposed
    /// (`TnI`). Transposition is tried first; the smallest `n` wins.
    pub fn moved_to(self, other: PitchSet) -> Option<(u8, bool)> {
        let inverted = self.invert();
        (0..12)
            .find(|&n| self.transpose(n) == other)
            .map(|n| (n, false))
            .or_else(|| {
                (0..12)
                    .find(|&n| inverted.transpose(n) == other)
                    .map(|n| (n, true))
            })
    }

    /// The set class this set belongs to.
    pub fn set_class(self) -> &'static SetClass {
        SetClass::of(self)
    }
}

/// One of the 224 set classes: every group of pitch classes belongs to
/// exactly one, together with all its transpositions and inversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetClass {
    /// How many notes: 3 for trichords, 4 for tetrachords...
    pub cardinality: u8,
    /// Its number in Allen Forte's list for that many notes.
    pub number: u8,
    /// Whether it is "Z-related" to another class: both have the same
    /// interval vector, but neither can be moved onto the other. Such
    /// sets have the same intervals, yet are different sets.
    pub z: bool,
    /// The prime form (John Rahn's, as most textbooks now use; for six
    /// classes Forte's own book packs them a little differently).
    pub prime: PitchSet,
}

impl SetClass {
    const fn new(cardinality: u8, number: u8, z: bool, bits: u16) -> SetClass {
        SetClass {
            cardinality,
            number,
            z,
            prime: PitchSet::from_bits(bits),
        }
    }

    /// Every set class, by size and then by Forte number. The empty set
    /// comes first.
    pub fn all() -> &'static [SetClass] {
        &SET_CLASSES
    }

    /// The class of `set`.
    pub fn of(set: PitchSet) -> &'static SetClass {
        let prime = set.prime_form();
        SET_CLASSES
            .iter()
            .find(|class| class.prime == prime)
            .expect("every prime form is in the table")
    }

    /// The Forte name, such as "3-11" or "4-Z15".
    pub fn name(&self) -> String {
        let z = if self.z { "Z" } else { "" };
        format!("{}-{z}{}", self.cardinality, self.number)
    }

    /// The prime form as Forte wrote it: `[0,3,7]`, with `T` and `E`
    /// for 10 and 11 so every pitch class is one character.
    pub fn prime_text(&self) -> String {
        format!("[{}]", digits(self.prime))
    }

    /// The interval vector, written `<001110>`, or with spaces when a
    /// count has two digits, as in `<12 12 12 12 12 6>` for all twelve
    /// notes.
    pub fn vector_text(&self) -> String {
        let vector = self.prime.interval_vector();
        let counts: Vec<String> = vector.iter().map(u8::to_string).collect();
        let gap = if vector.iter().any(|&count| count > 9) {
            " "
        } else {
            ""
        };
        format!("<{}>", counts.join(gap))
    }

    /// A well-known name for the class, if it has one, such as "major or
    /// minor triad" or "whole-tone scale".
    pub fn nickname(&self) -> Option<&'static str> {
        NICKNAMES
            .iter()
            .find(|(example, _)| PitchSet::from_semitones(example).prime_form() == self.prime)
            .map(|(_, name)| *name)
    }

    /// The class with the same interval vector, for a Z-related class.
    pub fn z_partner(&self) -> Option<&'static SetClass> {
        let vector = self.prime.interval_vector();
        SET_CLASSES.iter().find(|other| {
            other.prime != self.prime
                && other.cardinality == self.cardinality
                && other.prime.interval_vector() == vector
        })
    }

    /// Whether inverting the set gives a transposition of it, as for the
    /// diminished triad. Such a class has 12 or fewer members; the others
    /// have twice as many, half of them the inversions of the others.
    pub fn is_symmetric(&self) -> bool {
        self.prime
            .moved_to(self.prime.invert())
            .is_some_and(|(_, inverted)| !inverted)
    }

    /// A line describing the class: "3-11 [037] <001110> major or minor
    /// triad".
    pub fn describe(&self) -> String {
        let mut text = format!(
            "{} {} {}",
            self.name(),
            self.prime_text(),
            self.vector_text()
        );
        if let Some(nickname) = self.nickname() {
            text += &format!(" {}", crate::i18n::tr(nickname));
        }
        text
    }
}

/// The pitch classes of `set` as digits, with T and E for 10 and 11.
pub fn digits(set: PitchSet) -> String {
    set.iter().map(|p| pc_digit(p.semitone())).collect()
}

/// A pitch class as one character: 0 to 9, then T (ten) and E (eleven).
pub fn pc_digit(pitch_class: u8) -> char {
    match pitch_class {
        10 => 'T',
        11 => 'E',
        n => char::from(b'0' + n),
    }
}

/// Well-known set classes, each given by one of its sets, in semitones
/// above C.
const NICKNAMES: &[(&[u8], &str)] = &[
    (&[0], "a single note"),
    (&[0, 1], "minor second"),
    (&[0, 2], "major second"),
    (&[0, 3], "minor third"),
    (&[0, 4], "major third"),
    (&[0, 5], "perfect fourth or fifth"),
    (&[0, 6], "tritone"),
    (&[0, 1, 2], "chromatic cluster"),
    (&[0, 1, 6], "Viennese trichord"),
    (&[0, 2, 4], "whole-tone trichord"),
    (&[0, 2, 7], "sus chord"),
    (&[0, 3, 6], "diminished triad"),
    (&[0, 4, 7], "major or minor triad"),
    (&[0, 4, 8], "augmented triad"),
    (&[0, 4, 10], "dominant seventh without its fifth"),
    (&[0, 1, 2, 3], "chromatic cluster"),
    (&[0, 1, 4, 6], "all-interval tetrachord"),
    (&[0, 1, 3, 7], "all-interval tetrachord"),
    (&[0, 1, 6, 7], "two tritones"),
    (&[0, 2, 4, 6], "whole-tone tetrachord"),
    (&[0, 2, 4, 7], "add-nine chord"),
    (&[0, 2, 5, 7], "stacked fourths"),
    (&[0, 2, 6, 8], "French sixth"),
    (&[0, 3, 6, 9], "diminished seventh chord"),
    (&[0, 3, 7, 10], "minor seventh chord"),
    (&[0, 3, 7, 11], "minor-major seventh chord"),
    (&[0, 4, 7, 10], "dominant or half-diminished seventh chord"),
    (&[0, 4, 7, 11], "major seventh chord"),
    (&[0, 4, 8, 10], "augmented seventh chord"),
    (&[0, 2, 4, 7, 9], "pentatonic scale"),
    (&[0, 2, 4, 6, 8], "whole-tone pentachord"),
    (&[0, 4, 7, 10, 14], "dominant ninth chord"),
    (&[0, 4, 7, 11, 14], "major or minor ninth chord"),
    (&[0, 2, 4, 6, 8, 10], "whole-tone scale"),
    (&[0, 1, 4, 5, 8, 9], "hexatonic scale"),
    (&[0, 1, 2, 3, 4, 5], "chromatic hexachord"),
    (&[0, 2, 4, 5, 7, 9], "diatonic hexachord"),
    (&[0, 2, 4, 6, 9, 10], "mystic chord"),
    (&[0, 1, 3, 6, 7, 9], "Petrushka chord"),
    (
        &[0, 2, 4, 5, 7, 9, 11],
        "diatonic scale: major, minor and the modes",
    ),
    (&[0, 2, 3, 5, 7, 9, 11], "melodic minor scale"),
    (&[0, 2, 3, 5, 7, 8, 11], "harmonic minor scale"),
    (&[0, 1, 4, 5, 7, 8, 11], "double harmonic scale"),
    (&[0, 1, 3, 4, 6, 7, 9, 10], "octatonic (diminished) scale"),
    (&[0, 1, 2, 4, 5, 6, 8, 9, 10], "Messiaen's third mode"),
    (
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        "all twelve notes: the aggregate",
    ),
];

/// Every set class, in the order of Allen Forte's list (*The Structure of
/// Atonal Music*, 1973). The numbers were generated from the table in
/// music21 (BSD licence) and are checked by the tests below: each entry
/// is the prime form of a different class, and every one of the 4096
/// sets belongs to one of them.
const SET_CLASSES: [SetClass; 224] = [
    SetClass::new(0, 1, false, 0),
    SetClass::new(1, 1, false, 0b000000000001),
    SetClass::new(2, 1, false, 0b000000000011),
    SetClass::new(2, 2, false, 0b000000000101),
    SetClass::new(2, 3, false, 0b000000001001),
    SetClass::new(2, 4, false, 0b000000010001),
    SetClass::new(2, 5, false, 0b000000100001),
    SetClass::new(2, 6, false, 0b000001000001),
    SetClass::new(3, 1, false, 0b000000000111),
    SetClass::new(3, 2, false, 0b000000001011),
    SetClass::new(3, 3, false, 0b000000010011),
    SetClass::new(3, 4, false, 0b000000100011),
    SetClass::new(3, 5, false, 0b000001000011),
    SetClass::new(3, 6, false, 0b000000010101),
    SetClass::new(3, 7, false, 0b000000100101),
    SetClass::new(3, 8, false, 0b000001000101),
    SetClass::new(3, 9, false, 0b000010000101),
    SetClass::new(3, 10, false, 0b000001001001),
    SetClass::new(3, 11, false, 0b000010001001),
    SetClass::new(3, 12, false, 0b000100010001),
    SetClass::new(4, 1, false, 0b000000001111),
    SetClass::new(4, 2, false, 0b000000010111),
    SetClass::new(4, 3, false, 0b000000011011),
    SetClass::new(4, 4, false, 0b000000100111),
    SetClass::new(4, 5, false, 0b000001000111),
    SetClass::new(4, 6, false, 0b000010000111),
    SetClass::new(4, 7, false, 0b000000110011),
    SetClass::new(4, 8, false, 0b000001100011),
    SetClass::new(4, 9, false, 0b000011000011),
    SetClass::new(4, 10, false, 0b000000101101),
    SetClass::new(4, 11, false, 0b000000101011),
    SetClass::new(4, 12, false, 0b000001001101),
    SetClass::new(4, 13, false, 0b000001001011),
    SetClass::new(4, 14, false, 0b000010001101),
    SetClass::new(4, 15, true, 0b000001010011),
    SetClass::new(4, 16, false, 0b000010100011),
    SetClass::new(4, 17, false, 0b000010011001),
    SetClass::new(4, 18, false, 0b000010010011),
    SetClass::new(4, 19, false, 0b000100010011),
    SetClass::new(4, 20, false, 0b000100100011),
    SetClass::new(4, 21, false, 0b000001010101),
    SetClass::new(4, 22, false, 0b000010010101),
    SetClass::new(4, 23, false, 0b000010100101),
    SetClass::new(4, 24, false, 0b000100010101),
    SetClass::new(4, 25, false, 0b000101000101),
    SetClass::new(4, 26, false, 0b000100101001),
    SetClass::new(4, 27, false, 0b000100100101),
    SetClass::new(4, 28, false, 0b001001001001),
    SetClass::new(4, 29, true, 0b000010001011),
    SetClass::new(5, 1, false, 0b000000011111),
    SetClass::new(5, 2, false, 0b000000101111),
    SetClass::new(5, 3, false, 0b000000110111),
    SetClass::new(5, 4, false, 0b000001001111),
    SetClass::new(5, 5, false, 0b000010001111),
    SetClass::new(5, 6, false, 0b000001100111),
    SetClass::new(5, 7, false, 0b000011000111),
    SetClass::new(5, 8, false, 0b000001011101),
    SetClass::new(5, 9, false, 0b000001010111),
    SetClass::new(5, 10, false, 0b000001011011),
    SetClass::new(5, 11, false, 0b000010011101),
    SetClass::new(5, 12, true, 0b000001101011),
    SetClass::new(5, 13, false, 0b000100010111),
    SetClass::new(5, 14, false, 0b000010100111),
    SetClass::new(5, 15, false, 0b000101000111),
    SetClass::new(5, 16, false, 0b000010011011),
    SetClass::new(5, 17, true, 0b000100011011),
    SetClass::new(5, 18, true, 0b000010110011),
    SetClass::new(5, 19, false, 0b000011001011),
    SetClass::new(5, 20, false, 0b000101100011),
    SetClass::new(5, 21, false, 0b000100110011),
    SetClass::new(5, 22, false, 0b000110010011),
    SetClass::new(5, 23, false, 0b000010101101),
    SetClass::new(5, 24, false, 0b000010101011),
    SetClass::new(5, 25, false, 0b000100101101),
    SetClass::new(5, 26, false, 0b000100110101),
    SetClass::new(5, 27, false, 0b000100101011),
    SetClass::new(5, 28, false, 0b000101001101),
    SetClass::new(5, 29, false, 0b000101001011),
    SetClass::new(5, 30, false, 0b000101010011),
    SetClass::new(5, 31, false, 0b001001001011),
    SetClass::new(5, 32, false, 0b001001010011),
    SetClass::new(5, 33, false, 0b000101010101),
    SetClass::new(5, 34, false, 0b001001010101),
    SetClass::new(5, 35, false, 0b001010010101),
    SetClass::new(5, 36, true, 0b000010010111),
    SetClass::new(5, 37, true, 0b000100111001),
    SetClass::new(5, 38, true, 0b000100100111),
    SetClass::new(6, 1, false, 0b000000111111),
    SetClass::new(6, 2, false, 0b000001011111),
    SetClass::new(6, 3, true, 0b000001101111),
    SetClass::new(6, 4, true, 0b000001110111),
    SetClass::new(6, 5, false, 0b000011001111),
    SetClass::new(6, 6, true, 0b000011100111),
    SetClass::new(6, 7, false, 0b000111000111),
    SetClass::new(6, 8, false, 0b000010111101),
    SetClass::new(6, 9, false, 0b000010101111),
    SetClass::new(6, 10, true, 0b000010111011),
    SetClass::new(6, 11, true, 0b000010110111),
    SetClass::new(6, 12, true, 0b000011010111),
    SetClass::new(6, 13, true, 0b000011011011),
    SetClass::new(6, 14, false, 0b000100111011),
    SetClass::new(6, 15, false, 0b000100110111),
    SetClass::new(6, 16, false, 0b000101110011),
    SetClass::new(6, 17, true, 0b000110010111),
    SetClass::new(6, 18, false, 0b000110100111),
    SetClass::new(6, 19, true, 0b000110011011),
    SetClass::new(6, 20, false, 0b001100110011),
    SetClass::new(6, 21, false, 0b000101011101),
    SetClass::new(6, 22, false, 0b000101010111),
    SetClass::new(6, 23, true, 0b000101101101),
    SetClass::new(6, 24, true, 0b000101011011),
    SetClass::new(6, 25, true, 0b000101101011),
    SetClass::new(6, 26, true, 0b000110101011),
    SetClass::new(6, 27, false, 0b001001011011),
    SetClass::new(6, 28, true, 0b001001101011),
    SetClass::new(6, 29, true, 0b001011001101),
    SetClass::new(6, 30, false, 0b001011001011),
    SetClass::new(6, 31, false, 0b001010110011),
    SetClass::new(6, 32, false, 0b001010110101),
    SetClass::new(6, 33, false, 0b001010101101),
    SetClass::new(6, 34, false, 0b001010101011),
    SetClass::new(6, 35, false, 0b010101010101),
    SetClass::new(6, 36, true, 0b000010011111),
    SetClass::new(6, 37, true, 0b000100011111),
    SetClass::new(6, 38, true, 0b000110001111),
    SetClass::new(6, 39, true, 0b000100111101),
    SetClass::new(6, 40, true, 0b000100101111),
    SetClass::new(6, 41, true, 0b000101001111),
    SetClass::new(6, 42, true, 0b001001001111),
    SetClass::new(6, 43, true, 0b000101100111),
    SetClass::new(6, 44, true, 0b001001100111),
    SetClass::new(6, 45, true, 0b001001011101),
    SetClass::new(6, 46, true, 0b001001010111),
    SetClass::new(6, 47, true, 0b001010010111),
    SetClass::new(6, 48, true, 0b001010100111),
    SetClass::new(6, 49, true, 0b001010011011),
    SetClass::new(6, 50, true, 0b001011010011),
    SetClass::new(7, 1, false, 0b000001111111),
    SetClass::new(7, 2, false, 0b000010111111),
    SetClass::new(7, 3, false, 0b000100111111),
    SetClass::new(7, 4, false, 0b000011011111),
    SetClass::new(7, 5, false, 0b000011101111),
    SetClass::new(7, 6, false, 0b000110011111),
    SetClass::new(7, 7, false, 0b000111001111),
    SetClass::new(7, 8, false, 0b000101111101),
    SetClass::new(7, 9, false, 0b000101011111),
    SetClass::new(7, 10, false, 0b001001011111),
    SetClass::new(7, 11, false, 0b000101111011),
    SetClass::new(7, 12, true, 0b001010011111),
    SetClass::new(7, 13, false, 0b000101110111),
    SetClass::new(7, 14, false, 0b000110101111),
    SetClass::new(7, 15, false, 0b000111010111),
    SetClass::new(7, 16, false, 0b001001101111),
    SetClass::new(7, 17, true, 0b001001110111),
    SetClass::new(7, 18, true, 0b001011110011),
    SetClass::new(7, 19, false, 0b001011001111),
    SetClass::new(7, 20, false, 0b001011100111),
    SetClass::new(7, 21, false, 0b001100110111),
    SetClass::new(7, 22, false, 0b001101100111),
    SetClass::new(7, 23, false, 0b001010111101),
    SetClass::new(7, 24, false, 0b001010101111),
    SetClass::new(7, 25, false, 0b001011011101),
    SetClass::new(7, 26, false, 0b001010111011),
    SetClass::new(7, 27, false, 0b001010110111),
    SetClass::new(7, 28, false, 0b001011101011),
    SetClass::new(7, 29, false, 0b001011010111),
    SetClass::new(7, 30, false, 0b001101010111),
    SetClass::new(7, 31, false, 0b001011011011),
    SetClass::new(7, 32, false, 0b001101011011),
    SetClass::new(7, 33, false, 0b010101010111),
    SetClass::new(7, 34, false, 0b010101011011),
    SetClass::new(7, 35, false, 0b010101101011),
    SetClass::new(7, 36, true, 0b000101101111),
    SetClass::new(7, 37, true, 0b000110111011),
    SetClass::new(7, 38, true, 0b000110110111),
    SetClass::new(8, 1, false, 0b000011111111),
    SetClass::new(8, 2, false, 0b000101111111),
    SetClass::new(8, 3, false, 0b001001111111),
    SetClass::new(8, 4, false, 0b000110111111),
    SetClass::new(8, 5, false, 0b000111011111),
    SetClass::new(8, 6, false, 0b000111101111),
    SetClass::new(8, 7, false, 0b001100111111),
    SetClass::new(8, 8, false, 0b001110011111),
    SetClass::new(8, 9, false, 0b001111001111),
    SetClass::new(8, 10, false, 0b001011111101),
    SetClass::new(8, 11, false, 0b001010111111),
    SetClass::new(8, 12, false, 0b001011111011),
    SetClass::new(8, 13, false, 0b001011011111),
    SetClass::new(8, 14, false, 0b001011110111),
    SetClass::new(8, 15, true, 0b001101011111),
    SetClass::new(8, 16, false, 0b001110101111),
    SetClass::new(8, 17, false, 0b001101111011),
    SetClass::new(8, 18, false, 0b001101101111),
    SetClass::new(8, 19, false, 0b001101110111),
    SetClass::new(8, 20, false, 0b001110110111),
    SetClass::new(8, 21, false, 0b010101011111),
    SetClass::new(8, 22, false, 0b010101101111),
    SetClass::new(8, 23, false, 0b010110101111),
    SetClass::new(8, 24, false, 0b010101110111),
    SetClass::new(8, 25, false, 0b010111010111),
    SetClass::new(8, 26, false, 0b010110111011),
    SetClass::new(8, 27, false, 0b010110110111),
    SetClass::new(8, 28, false, 0b011011011011),
    SetClass::new(8, 29, true, 0b001011101111),
    SetClass::new(9, 1, false, 0b000111111111),
    SetClass::new(9, 2, false, 0b001011111111),
    SetClass::new(9, 3, false, 0b001101111111),
    SetClass::new(9, 4, false, 0b001110111111),
    SetClass::new(9, 5, false, 0b001111011111),
    SetClass::new(9, 6, false, 0b010101111111),
    SetClass::new(9, 7, false, 0b010110111111),
    SetClass::new(9, 8, false, 0b010111011111),
    SetClass::new(9, 9, false, 0b010111101111),
    SetClass::new(9, 10, false, 0b011011011111),
    SetClass::new(9, 11, false, 0b011011101111),
    SetClass::new(9, 12, false, 0b011101110111),
    SetClass::new(10, 1, false, 0b001111111111),
    SetClass::new(10, 2, false, 0b010111111111),
    SetClass::new(10, 3, false, 0b011011111111),
    SetClass::new(10, 4, false, 0b011101111111),
    SetClass::new(10, 5, false, 0b011110111111),
    SetClass::new(10, 6, false, 0b011111011111),
    SetClass::new(11, 1, false, 0b011111111111),
    SetClass::new(12, 1, false, 0b111111111111),
];

#[cfg(test)]
mod tests {
    use super::*;
    use PitchClass::*;

    fn set(semitones: &[u8]) -> PitchSet {
        PitchSet::from_semitones(semitones)
    }

    #[test]
    fn every_set_belongs_to_exactly_one_class() {
        let mut members = vec![0; SET_CLASSES.len()];
        for bits in 0..4096u16 {
            let class = SetClass::of(PitchSet::from_bits(bits));
            let index = SET_CLASSES.iter().position(|c| c == class).unwrap();
            members[index] += 1;
            assert_eq!(class.cardinality as usize, PitchSet::from_bits(bits).len());
        }
        // Each class has as many members as it has distinct
        // transpositions and inversions, and none is empty.
        assert!(members.iter().all(|&count| count > 0));
        assert_eq!(members.iter().sum::<usize>(), 4096);
    }

    #[test]
    fn the_table_holds_prime_forms_in_order() {
        for class in SET_CLASSES {
            assert_eq!(class.prime.prime_form(), class.prime, "{}", class.name());
        }
        for pair in SET_CLASSES.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            assert!(
                (a.cardinality, a.number) < (b.cardinality, b.number),
                "{} then {}",
                a.name(),
                b.name()
            );
        }
        // How many classes there are of each size: one empty set, 1, 6,
        // 12, 29, 38, 50 hexachords, then the complements in mirror.
        let counts: Vec<usize> = (0..=12)
            .map(|n| SET_CLASSES.iter().filter(|c| c.cardinality == n).count())
            .collect();
        assert_eq!(counts, [1, 1, 6, 12, 29, 38, 50, 38, 29, 12, 6, 1, 1]);
    }

    #[test]
    fn well_known_classes() {
        let triad = set(&[0, 4, 7]).set_class();
        assert_eq!(triad.name(), "3-11");
        assert_eq!(triad.prime_text(), "[037]");
        assert_eq!(triad.vector_text(), "<001110>");
        let all = PitchSet::from_bits(0xfff).set_class();
        assert_eq!(all.vector_text(), "<12 12 12 12 12 6>");
        assert_eq!(triad.nickname(), Some("major or minor triad"));
        // A minor triad is the same class: an inverted major triad.
        assert_eq!(set(&[9, 0, 4]).set_class(), triad);

        assert_eq!(set(&[0, 3, 6, 9]).set_class().name(), "4-28");
        assert_eq!(set(&[0, 1, 4, 6]).set_class().name(), "4-Z15");
        assert_eq!(set(&[0, 1, 3, 7]).set_class().name(), "4-Z29");
        assert_eq!(set(&[0, 2, 4, 5, 7, 9, 11]).set_class().name(), "7-35");
        assert_eq!(set(&[0, 2, 4, 6, 8, 10]).set_class().name(), "6-35");
        assert_eq!(set(&[0, 1, 3, 4, 6, 7, 9, 10]).set_class().name(), "8-28");
        assert_eq!(set(&[0, 1, 6]).set_class().name(), "3-5");
        assert_eq!(set(&[0, 2, 4, 7, 9]).set_class().name(), "5-35");
    }

    #[test]
    fn rahn_and_forte_differ_for_six_classes() {
        // Forte packs 5-20 as [01378]; Rahn, and this module, as [01568].
        assert_eq!(set(&[0, 1, 3, 7, 8]).prime_form(), set(&[0, 1, 5, 6, 8]));
        assert_eq!(set(&[0, 1, 5, 6, 8]).set_class().name(), "5-20");
    }

    #[test]
    fn z_related_classes_share_their_intervals() {
        let a = set(&[0, 1, 4, 6]).set_class();
        let b = a.z_partner().unwrap();
        assert_eq!(b.name(), "4-Z29");
        assert_eq!(a.vector_text(), "<111111>");
        assert_eq!(b.vector_text(), "<111111>");
        for class in SET_CLASSES {
            assert_eq!(class.z, class.z_partner().is_some(), "{}", class.name());
        }
    }

    #[test]
    fn normal_form_packs_the_notes_tightly() {
        // E G C in normal form is C E G.
        let c_major: PitchSet = [E, G, C].into_iter().collect();
        assert_eq!(c_major.normal_form(), [C, E, G]);
        // B D F G# looks the same from each of its notes, so the start
        // with the lowest number wins: D.
        let diminished: PitchSet = [B, D, F, GSharp].into_iter().collect();
        assert_eq!(diminished.normal_form(), [D, F, GSharp, B]);
        // A C E: the gap from A up to E is 7, smaller than any other start.
        let a_minor: PitchSet = [A, C, E].into_iter().collect();
        assert_eq!(a_minor.normal_form(), [A, C, E]);
        // Across the octave: B C D starts on B.
        let wrapping: PitchSet = [B, C, D].into_iter().collect();
        assert_eq!(wrapping.normal_form(), [B, C, D]);
    }

    #[test]
    fn transposed_or_inverted() {
        let c_major = set(&[0, 4, 7]);
        let e_major = set(&[4, 8, 11]);
        let a_minor = set(&[9, 0, 4]);
        assert_eq!(c_major.moved_to(e_major), Some((4, false)));
        // C E G inverted around C is C Ab F (F minor); up 4 is A minor.
        assert_eq!(c_major.invert(), set(&[0, 8, 5]));
        assert_eq!(c_major.moved_to(a_minor), Some((4, true)));
        assert_eq!(c_major.moved_to(set(&[0, 1, 2])), None);
    }

    #[test]
    fn complements() {
        let diatonic = set(&[0, 2, 4, 5, 7, 9, 11]);
        assert_eq!(diatonic.complement(), set(&[1, 3, 6, 8, 10]));
        assert_eq!(
            diatonic.complement().set_class().nickname(),
            Some("pentatonic scale")
        );
        assert!(set(&[0, 3, 6]).set_class().is_symmetric());
        assert!(!set(&[0, 1, 3]).set_class().is_symmetric());
    }

    #[test]
    fn every_nickname_names_one_class() {
        for (i, (a, _)) in NICKNAMES.iter().enumerate() {
            for (b, _) in &NICKNAMES[i + 1..] {
                assert_ne!(set(a).prime_form(), set(b).prime_form(), "{a:?} and {b:?}");
            }
        }
    }
}
