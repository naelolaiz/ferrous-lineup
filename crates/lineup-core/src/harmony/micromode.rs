//! Micromodes: Francisco Kröpfl's twelve colours of three notes.
//!
//! The Argentine composer Francisco Kröpfl (1931-2021) taught
//! composition with a method of his own (his piano piece *Música '66*
//! was written to try it out), which his students passed on: build the
//! music from groups of three different notes, and let one group give a
//! passage its colour. He called these groups **micromodes**
//! (*micromodos*).
//!
//! - **Three notes, any order.** A micromode is three different pitch
//!   classes. The order does not matter, nor the octave, nor how a note
//!   is spelled (C# and Db are the same note).
//! - **Moved or upside down, still the same.** Like a pitch-class set,
//!   a micromode stays itself when it is transposed or inverted. So any
//!   three notes belong to exactly one of only **twelve** micromodes,
//!   which are Allen Forte's twelve trichord classes, 3-1 to 3-12, in
//!   the same order.
//! - **Three families.** The *minor* micromodes (Menor 1 to 5) have a
//!   minor second, a semitone. The *major* ones (Mayor 1 to 4) have a
//!   major second and no semitone. The last three have two thirds or
//!   more and are named after the chords they are: the diminished, the
//!   minor and major, and the augmented triad.
//! - **Named by their prime form.** Written in their most compact form
//!   from C (do), the first nine start with a minor second (do-do#) or a
//!   major second (do-re), which gives the family its name; the number
//!   counts up as the third note rises.
//!
//! Kröpfl never published the theory himself; it is known from his
//! students and from analyses of their music: Pablo Cetta, "La tercera
//! es la vencida de Francisco Kröpfl y Oscar Steimberg", *Cuadernos de
//! Análisis y Debate sobre Músicas Latinoamericanas Contemporáneas* 1
//! (2018), with the table of the twelve; Pablo Jaureguiberry, "Tres
//! amores de Diana Rud: otras particularidades de los micromodos de
//! Francisco Kröpfl" (2016), with the three families; Alejandro
//! Martínez, "Algunos aspectos de los micromodos de Francisco Kröpfl y
//! su contextualización histórica", *Actas de la XXI Conferencia de la
//! AAM* (2015); Marcela Pavia, "La estructura de la música atonal: dos
//! enfoques", *CodeXXI* 1 (1998), which compares Kröpfl with Forte.
//!
//! The Dutch composer Peter Schat came to the same twelve groups on his
//! own and called them the twelve hours of his *Tone Clock* (1993):
//! hour I is Menor 1, hour XII the augmented triad.
//!
//! In this module a micromode is a [`Micromode`], found with
//! [`Micromode::of`]. A larger set is made of many groups of three, and
//! [`micromode_content`] counts them: how much of each colour it holds.

use super::{Chord, ChordQuality, PitchSet, SetClass};
use crate::pitch::PitchClass;

/// The three families of micromodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MicromodeFamily {
    /// With at least one minor second: Menor 1 to 5.
    Minor,
    /// With at least one major second and no semitone: Mayor 1 to 4.
    Major,
    /// With two thirds or more: the diminished triad, the minor and
    /// major triad (acorde menor-mayor) and the augmented triad.
    Chord,
}

impl MicromodeFamily {
    pub const ALL: [MicromodeFamily; 3] = [
        MicromodeFamily::Minor,
        MicromodeFamily::Major,
        MicromodeFamily::Chord,
    ];

    /// The family's name, Kröpfl's in brackets.
    pub fn name(self) -> &'static str {
        match self {
            MicromodeFamily::Minor => "Minor (menores)",
            MicromodeFamily::Major => "Major (mayores)",
            MicromodeFamily::Chord => "Chords (acordes)",
        }
    }

    /// What every micromode of the family has.
    pub fn rule(self) -> &'static str {
        match self {
            MicromodeFamily::Minor => "Each has a minor second: a semitone.",
            MicromodeFamily::Major => "Each has a major second, and no semitone.",
            MicromodeFamily::Chord => "Each has two thirds or more: they are chords.",
        }
    }

    /// The family of a group of three notes, worked out from its
    /// intervals with the rules above (the way Jaureguiberry states
    /// them), not from the table. The tests check the table with it.
    #[cfg(test)]
    fn of_intervals(vector: [u8; 6]) -> MicromodeFamily {
        if vector[0] > 0 {
            MicromodeFamily::Minor
        } else if vector[1] > 0 {
            MicromodeFamily::Major
        } else {
            MicromodeFamily::Chord
        }
    }
}

/// One of the twelve micromodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Micromode {
    /// From 1 to 12: its place in the table, which is also its Forte
    /// number (3-`number`) and its hour on Peter Schat's Tone Clock.
    pub number: u8,
    pub family: MicromodeFamily,
    /// Kröpfl's name, in Spanish, as his students use it: "Menor 3".
    pub name: &'static str,
    /// The name in English.
    pub english: &'static str,
    /// The prime form, from C.
    pub prime: PitchSet,
    /// The prime form in solfège, as in Cetta's table: "do - do# - mi".
    pub solfege: &'static str,
    /// Where you hear it: a chord or scale it belongs to, or music that
    /// is built from it.
    pub about: &'static str,
}

impl Micromode {
    /// All twelve, in the order of the table.
    pub fn all() -> &'static [Micromode; 12] {
        &MICROMODES
    }

    /// The micromode `number`, from 1 to 12.
    pub fn number(number: u8) -> Option<&'static Micromode> {
        MICROMODES.iter().find(|m| m.number == number)
    }

    /// The micromode of a group of three notes, or `None` if `set` does
    /// not have exactly three.
    pub fn of(set: PitchSet) -> Option<&'static Micromode> {
        if set.len() != 3 {
            return None;
        }
        let prime = set.prime_form();
        MICROMODES.iter().find(|m| m.prime == prime)
    }

    /// The micromode as a key on `root`: its prime form moved to start
    /// on `root`, packed, as the Micromodes window and the styles place
    /// it. Menor 3 on D is D D# F#.
    pub fn key(&self, root: PitchClass) -> super::Scale {
        let first = self
            .prime
            .normal_form()
            .first()
            .copied()
            .unwrap_or(PitchClass::C);
        super::Scale::set(root, self.prime.transpose(first.semitones_up_to(root)))
    }

    /// The micromode with this name, in Spanish or English, any case:
    /// "menor 3", "Minor 3", "acorde aumentado".
    pub fn named(name: &str) -> Option<&'static Micromode> {
        let name = name.trim().to_lowercase();
        MICROMODES
            .iter()
            .find(|m| m.name.to_lowercase() == name || m.english.to_lowercase() == name)
    }

    /// Its pitch-class set class: Forte's 3-`number`.
    pub fn set_class(&self) -> &'static SetClass {
        self.prime.set_class()
    }

    /// "Menor 3 (minor 3)".
    pub fn full_name(&self) -> String {
        // Its name is Spanish already: in Spanish, it needs no other.
        match crate::i18n::Language::current() {
            crate::i18n::Language::English => {
                format!("{} ({})", self.name, self.english.to_lowercase())
            }
            crate::i18n::Language::Spanish => self.name.to_string(),
        }
    }

    /// Its hour on Peter Schat's Tone Clock, in Roman numerals: "III".
    pub fn hour(&self) -> &'static str {
        const HOURS: [&str; 12] = [
            "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII",
        ];
        HOURS[usize::from(self.number - 1)]
    }

    /// The three intervals between its notes, in words, smallest first:
    /// "a minor second, a minor third and a major third". An interval
    /// and its inversion count as one, as in the interval vector: a
    /// fifth is a fourth upside down.
    pub fn intervals(&self) -> String {
        const NAMES: [(&str, &str); 6] = [
            ("minor second", "minor seconds"),
            ("major second", "major seconds"),
            ("minor third", "minor thirds"),
            ("major third", "major thirds"),
            ("perfect fourth or fifth", "perfect fourths or fifths"),
            ("tritone", "tritones"),
        ];
        let vector = self.prime.interval_vector();
        let mut parts = Vec::new();
        for (class, &count) in vector.iter().enumerate() {
            let (one, many) = NAMES[class];
            // Whole phrases, as a Spanish number goes with its noun.
            match count {
                0 => {}
                1 => parts.push(crate::i18n::tr_string(format!("a {one}"))),
                2 => parts.push(crate::i18n::tr_string(format!("two {many}"))),
                _ => parts.push(crate::i18n::tr_string(format!("three {many}"))),
            }
        }
        crate::i18n::list(&parts)
    }

    /// Every group of three notes that is this micromode: its prime form
    /// and inversion on each of the twelve notes, without repeats. Most
    /// have 24; those that are the same upside down have 12, and the
    /// augmented triad only 4. Ordered by their lowest note, then their
    /// bits.
    pub fn members(&self) -> Vec<PitchSet> {
        let inverted = self.prime.invert();
        let mut members: Vec<PitchSet> = (0..12)
            .flat_map(|n| [self.prime.transpose(n), inverted.transpose(n)])
            .collect();
        members.sort_by_key(|set| {
            let first = set.normal_form().first().map_or(0, |p| p.semitone());
            (first, set.bits())
        });
        members.dedup();
        members
    }

    /// The triad a chord micromode is, written with `form`'s notes:
    /// the diminished triad, the minor or major triad, or the augmented
    /// triad (on the first note of its normal form, as it has no root of
    /// its own). `None` for the minor and major micromodes, and for a
    /// `form` that is not this micromode.
    pub fn chord(&self, form: PitchSet) -> Option<Chord> {
        if self.family != MicromodeFamily::Chord || Micromode::of(form) != Some(self) {
            return None;
        }
        form.normal_form().into_iter().find_map(|root| {
            ChordQuality::identify(form.relative_to(root)).map(|q| Chord::new(root, q))
        })
    }
}

/// How many times each micromode is found in `set`: one count for every
/// group of three of its notes, in the order of the table (index 0 is
/// Menor 1). A set of `n` notes has `n(n-1)(n-2)/6` groups of three.
///
/// This is how the micromodes link to larger sets: a dominant seventh
/// chord (C E G Bb) holds a major triad (C E G), a diminished triad
/// (E G Bb), Mayor 3 (C E Bb, the chord without its fifth) and Mayor 2
/// (C G Bb).
pub fn micromode_content(set: PitchSet) -> [u16; 12] {
    let mut counts = [0; 12];
    for trio in trichords(set) {
        if let Some(micromode) = Micromode::of(trio) {
            counts[usize::from(micromode.number - 1)] += 1;
        }
    }
    counts
}

/// Every group of three notes of `set`.
pub fn trichords(set: PitchSet) -> Vec<PitchSet> {
    let notes: Vec<PitchClass> = set.iter().collect();
    let mut trios = Vec::new();
    for a in 0..notes.len() {
        for b in a + 1..notes.len() {
            for c in b + 1..notes.len() {
                trios.push([notes[a], notes[b], notes[c]].into_iter().collect());
            }
        }
    }
    trios
}

/// The micromodes in `set`, most common first (then in the order of the
/// table), with how many of each: for C C# E F, `[(Menor 3, 2), (Menor
/// 4, 2)]`.
pub fn micromodes_in(set: PitchSet) -> Vec<(&'static Micromode, u16)> {
    let counts = micromode_content(set);
    let mut found: Vec<(&'static Micromode, u16)> = MICROMODES
        .iter()
        .zip(counts)
        .filter(|(_, count)| *count > 0)
        .collect();
    found.sort_by_key(|(m, count)| (std::cmp::Reverse(*count), m.number));
    found
}

const fn prime(bits: u16) -> PitchSet {
    PitchSet::from_bits(bits)
}

/// The twelve, as in Pablo Cetta's table (Ejemplo 1, 2018), with Forte's
/// numbers.
static MICROMODES: [Micromode; 12] = [
    Micromode {
        number: 1,
        family: MicromodeFamily::Minor,
        name: "Menor 1",
        english: "Minor 1",
        prime: prime(0b0000_0000_0111),
        solfege: "do - do# - re",
        about: "Three notes a semitone apart: the smallest cluster, a chromatic scale in \
                miniature.",
    },
    Micromode {
        number: 2,
        family: MicromodeFamily::Minor,
        name: "Menor 2",
        english: "Minor 2",
        prime: prime(0b0000_0000_1011),
        solfege: "do - do# - re#",
        about: "A semitone and a whole tone: the first three notes of the Phrygian mode \
                (E F G), and three notes of the octatonic scale.",
    },
    Micromode {
        number: 3,
        family: MicromodeFamily::Minor,
        name: "Menor 3",
        english: "Minor 3",
        prime: prime(0b0000_0001_0011),
        solfege: "do - do# - mi",
        about: "A favourite of Kröpfl's students, and the colour of the accompaniment of the \
                witch's song in his opera La tercera es la vencida (2011). Schoenberg's Piano Piece Op. 11 No. 1 \
                (1909) opens with it (B G# G), and all four groups of three in the row of \
                Webern's Concerto Op. 24 (1934) are Menor 3.",
    },
    Micromode {
        number: 4,
        family: MicromodeFamily::Minor,
        name: "Menor 4",
        english: "Minor 4",
        prime: prime(0b0000_0010_0011),
        solfege: "do - do# - fa",
        about: "A major seventh chord without its fifth (C E B): the semitone of a major \
                seventh, folded into one octave.",
    },
    Micromode {
        number: 5,
        family: MicromodeFamily::Minor,
        name: "Menor 5",
        english: "Minor 5",
        prime: prime(0b0000_0100_0011),
        solfege: "do - do# - fa#",
        about: "A semitone, a fourth and a tritone: the \"Viennese trichord\" of Schoenberg, \
                Berg and Webern. Kröpfl's Música '66 for piano uses it, beside Menor 3.",
    },
    Micromode {
        number: 6,
        family: MicromodeFamily::Major,
        name: "Mayor 1",
        english: "Major 1",
        prime: prime(0b0000_0001_0101),
        solfege: "do - re - mi",
        about: "Two whole tones: do re mi, and three notes of the whole-tone scale of \
                Debussy.",
    },
    Micromode {
        number: 7,
        family: MicromodeFamily::Major,
        name: "Mayor 2",
        english: "Major 2",
        prime: prime(0b0000_0010_0101),
        solfege: "do - re - fa",
        about: "Three notes of the pentatonic scale (C D F, D E G), and a minor seventh chord \
                without its fifth (C Eb Bb).",
    },
    Micromode {
        number: 8,
        family: MicromodeFamily::Major,
        name: "Mayor 3",
        english: "Major 3",
        prime: prime(0b0000_0100_0101),
        solfege: "do - re - fa#",
        about: "A dominant seventh chord without its fifth (C E Bb), with the tritone that \
                pulls it home. Also three notes of the whole-tone scale.",
    },
    Micromode {
        number: 9,
        family: MicromodeFamily::Major,
        name: "Mayor 4",
        english: "Major 4",
        prime: prime(0b0000_1000_0101),
        solfege: "do - re - sol",
        about: "A whole tone and two fourths: the sus chord (C F G, C D G) and chords stacked \
                in fourths (D G C).",
    },
    Micromode {
        number: 10,
        family: MicromodeFamily::Chord,
        name: "Acorde disminuido",
        english: "Diminished triad",
        prime: prime(0b0000_0100_1001),
        solfege: "do - mib - solb",
        about: "The diminished triad: two minor thirds that fill a tritone.",
    },
    Micromode {
        number: 11,
        family: MicromodeFamily::Chord,
        name: "Acorde menor-mayor",
        english: "Minor and major triad",
        prime: prime(0b0000_1000_1001),
        solfege: "do - mib - sol",
        about: "The minor and the major triad are one micromode: the prime form is minor \
                (C Eb G), and turned upside down it is major (C A F, F major). Its name, \
                menor-mayor, says both. The chords of tonal music.",
    },
    Micromode {
        number: 12,
        family: MicromodeFamily::Chord,
        name: "Acorde aumentado",
        english: "Augmented triad",
        prime: prime(0b0001_0001_0001),
        solfege: "do - mi - sol#",
        about: "The augmented triad: three major thirds that fill the octave. There are only \
                four different ones.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use PitchClass::*;

    fn set(semitones: &[u8]) -> PitchSet {
        PitchSet::from_semitones(semitones)
    }

    #[test]
    fn the_twelve_are_fortes_twelve_trichords_in_order() {
        let trichords: Vec<&SetClass> = SetClass::all()
            .iter()
            .filter(|class| class.cardinality == 3)
            .collect();
        assert_eq!(trichords.len(), 12);
        for (micromode, class) in MICROMODES.iter().zip(trichords) {
            assert_eq!(micromode.prime, class.prime, "{}", micromode.name);
            assert_eq!(
                micromode.set_class().name(),
                format!("3-{}", micromode.number)
            );
        }
    }

    #[test]
    fn any_three_notes_are_one_micromode() {
        let mut count = [0; 12];
        for bits in 0..4096u16 {
            let set = PitchSet::from_bits(bits);
            match Micromode::of(set) {
                Some(m) => {
                    assert_eq!(set.len(), 3);
                    count[usize::from(m.number - 1)] += 1;
                }
                None => assert_ne!(set.len(), 3),
            }
        }
        // 220 ways to choose three of twelve notes.
        assert_eq!(count.iter().sum::<usize>(), 220);
        for m in &MICROMODES {
            assert_eq!(
                m.members().len(),
                count[usize::from(m.number - 1)],
                "{}",
                m.name
            );
        }
        let sizes: Vec<usize> = MICROMODES.iter().map(|m| m.members().len()).collect();
        assert_eq!(sizes, [12, 24, 24, 24, 24, 12, 24, 24, 12, 12, 24, 4]);
    }

    /// The families follow from the intervals, by the rules of
    /// Jaureguiberry (2016), and agree with the names of Cetta's table.
    #[test]
    fn families_follow_from_the_intervals() {
        for m in &MICROMODES {
            let family = MicromodeFamily::of_intervals(m.prime.interval_vector());
            assert_eq!(family, m.family, "{}", m.name);
            let counts = MICROMODES.iter().filter(|o| o.family == m.family).count();
            match m.family {
                MicromodeFamily::Minor => {
                    assert!(m.name.starts_with("Menor"));
                    assert_eq!(counts, 5);
                }
                MicromodeFamily::Major => {
                    assert!(m.name.starts_with("Mayor"));
                    assert_eq!(counts, 4);
                }
                MicromodeFamily::Chord => {
                    assert!(m.name.starts_with("Acorde"));
                    assert_eq!(counts, 3);
                    // "At least two thirds each."
                    let vector = m.prime.interval_vector();
                    assert!(vector[2] + vector[3] >= 2, "{}", m.name);
                }
            }
        }
    }

    /// The prime forms are written from C in solfège, as in the table,
    /// and the first nine start do-do# (minor) or do-re (major).
    #[test]
    fn solfege_names_the_prime_form() {
        let syllable = |name: &str| -> u8 {
            match name {
                "do" => 0,
                "do#" => 1,
                "re" => 2,
                "re#" | "mib" => 3,
                "mi" => 4,
                "fa" => 5,
                "fa#" | "solb" => 6,
                "sol" => 7,
                "sol#" => 8,
                other => panic!("{other}"),
            }
        };
        for m in &MICROMODES {
            let notes: Vec<u8> = m.solfege.split(" - ").map(syllable).collect();
            assert_eq!(set(&notes), m.prime, "{}", m.name);
            let second = notes[1];
            match m.family {
                MicromodeFamily::Minor => assert_eq!(second, 1),
                MicromodeFamily::Major => assert_eq!(second, 2),
                MicromodeFamily::Chord => assert!(second >= 3),
            }
        }
    }

    #[test]
    fn examples_from_the_sources() {
        // Cetta: E F Ab and F D Gb are both Menor 3, like its prime form
        // do do# mi.
        let menor_3 = Micromode::named("menor 3").unwrap();
        assert_eq!(
            Micromode::of([E, F, GSharp].into_iter().collect()),
            Some(menor_3)
        );
        assert_eq!(
            Micromode::of([F, D, FSharp].into_iter().collect()),
            Some(menor_3)
        );
        assert_eq!(
            menor_3.intervals(),
            "a minor second, a minor third and a major third"
        );
        // Schoenberg, Op. 11 No. 1: B G# G.
        assert_eq!(
            Micromode::of([B, GSharp, G].into_iter().collect()),
            Some(menor_3)
        );
        // Webern, Concerto Op. 24: B Bb D | Eb G F# | Ab E F | C C# A.
        let row = [B, ASharp, D, DSharp, G, FSharp, GSharp, E, F, C, CSharp, A];
        for trio in row.chunks(3) {
            let group: PitchSet = trio.iter().copied().collect();
            assert_eq!(Micromode::of(group), Some(menor_3), "{trio:?}");
        }
        // Jaureguiberry: a set of four notes made of two Menor 2 and two
        // Menor 3, as C C# D# E.
        let four = set(&[0, 1, 3, 4]);
        let menor_2 = Micromode::named("Minor 2").unwrap();
        assert_eq!(micromodes_in(four), [(menor_2, 2), (menor_3, 2)]);
        // Cetta, Música '66: a set 4-9 holds four of Menor 5.
        let four_nine = SetClass::all()
            .iter()
            .find(|c| c.name() == "4-9")
            .unwrap()
            .prime;
        let menor_5 = Micromode::number(5).unwrap();
        assert_eq!(micromodes_in(four_nine), [(menor_5, 4)]);
    }

    #[test]
    fn larger_sets_are_made_of_micromodes() {
        // C E G Bb: C E G major, E G Bb diminished, C E Bb and C G Bb.
        let dominant = set(&[0, 4, 7, 10]);
        let names: Vec<(&str, u16)> = micromodes_in(dominant)
            .iter()
            .map(|(m, n)| (m.name, *n))
            .collect();
        assert_eq!(
            names,
            [
                ("Mayor 2", 1),
                ("Mayor 3", 1),
                ("Acorde disminuido", 1),
                ("Acorde menor-mayor", 1)
            ]
        );
        // The whole-tone scale has no semitone and no odd interval: only
        // Mayor 1, Mayor 3 and the augmented triad.
        let whole_tone = set(&[0, 2, 4, 6, 8, 10]);
        let content = micromode_content(whole_tone);
        assert_eq!(content.iter().sum::<u16>(), 20);
        assert_eq!(content, [0, 0, 0, 0, 0, 6, 0, 12, 0, 0, 0, 2]);
        // All twelve notes hold every member of every micromode once.
        let all = micromode_content(PitchSet::from_bits(0xfff));
        let sizes: Vec<u16> = MICROMODES
            .iter()
            .map(|m| m.members().len() as u16)
            .collect();
        assert_eq!(all.to_vec(), sizes);
        // Fewer than three notes hold none.
        assert_eq!(micromode_content(set(&[0, 7])), [0; 12]);
    }

    #[test]
    fn the_chord_micromodes_are_triads() {
        let dim = Micromode::number(10).unwrap();
        let chord = dim.chord(set(&[11, 2, 5])).unwrap();
        assert_eq!((chord.root, chord.quality), (B, ChordQuality::Diminished));
        let triads = Micromode::number(11).unwrap();
        let a_minor = triads.chord(set(&[9, 0, 4])).unwrap();
        assert_eq!((a_minor.root, a_minor.quality), (A, ChordQuality::Minor));
        let c_major = triads.chord(set(&[0, 4, 7])).unwrap();
        assert_eq!((c_major.root, c_major.quality), (C, ChordQuality::Major));
        let aug = Micromode::number(12).unwrap();
        assert_eq!(
            aug.chord(set(&[0, 4, 8])).unwrap().quality,
            ChordQuality::Augmented
        );
        // Not a chord micromode, or not this micromode.
        assert_eq!(Micromode::number(3).unwrap().chord(set(&[0, 1, 4])), None);
        assert_eq!(dim.chord(set(&[0, 4, 7])), None);
    }

    #[test]
    fn a_micromode_as_a_key_starts_on_its_note() {
        let menor_3 = Micromode::named("Menor 3").unwrap();
        let key = menor_3.key(PitchClass::D);
        assert_eq!(key.root, PitchClass::D);
        assert_eq!(key.pitch_set(), set(&[2, 3, 6]));
        assert_eq!(crate::i18n::key_name(key), "Menor 3 on D");
        // Every micromode, on every note, is itself as a key.
        for micromode in Micromode::all() {
            for root in PitchClass::ALL {
                let key = micromode.key(root);
                assert_eq!(Micromode::of(key.pitch_set()), Some(micromode));
                assert_eq!(key.root, root);
            }
        }
    }

    #[test]
    fn hours_and_names() {
        let m = Micromode::named("Acorde aumentado").unwrap();
        assert_eq!(m.hour(), "XII");
        assert_eq!(Micromode::number(3).unwrap().hour(), "III");
        assert_eq!(
            Micromode::number(9).unwrap().full_name(),
            "Mayor 4 (major 4)"
        );
        assert_eq!(
            Micromode::number(9).unwrap().intervals(),
            "a major second and two perfect fourths or fifths"
        );
        assert_eq!(
            Micromode::number(12).unwrap().intervals(),
            "three major thirds"
        );
        assert_eq!(Micromode::of(set(&[0, 4])), None);
    }
}
