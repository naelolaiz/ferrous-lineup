//! Latin: cumbia, bossa nova, samba, choro, son and the Mexican ranchera.

use super::*;

pub(super) const LA_CUCARACHA: Song = Song {
    name: "La Cucaracha",
    origin: "Mexican folk song, traditional",
    feel: "Cumbia",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Offbeat,
    bass: Bass::RootFifth,
    rhythm: "Cumbia",
    band: CUMBIA,
    style: "Cumbia",
    parts: &[
        Part::new("Pickup", "-:10 G4:2 G4 G4", "I"),
        Part::new(
            "Song",
            "C5:6 E5:4 G4:2 G4 G4 | C5:6 E5:10 | -:2 C5:4 C5:2 B4 B4 A4 A4 | G4:8 -:2 G4:2 G4 G4 | \
             B4:6 D5:4 G4:2 G4 G4 | B4:6 D5:10 | -:2 G5:4 A5:2 G5 F5 E5 D5 | C5:8 -:2 G4:2 G4 G4",
            "I | I | I | V7 | V7 | V7 | V7 | I",
        ),
    ],
    form: &["Pickup", "Song", "Song", "Song", "Song"],
    drums_from: 1,
    about: "The whole song needs two chords, the home chord and V7, here as Colombian \
            cumbia: chords on the offbeats, bass on the beats and a scraper in eighths. \
            \"La cu-ca\" is a pickup; \"ra\" lands on the beat.",
};

pub(super) const RIO_AFTERNOON: Song = Song {
    name: "Rio Afternoon",
    origin: "Written for Ferrous Lineup",
    feel: "Bossa nova",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 128.0,
    comping: Comping::Bossa,
    bass: Bass::Bossa,
    rhythm: "Bossa nova",
    band: FLUTE_AND_GUITAR,
    style: "Bossa nova",
    parts: &[Part::new(
        "Tune",
        "E5:6 D5:2 E5:8 | B4:4 C5 E5 G5 | F#5:6 E5:2 D5:8 | C5:4 D5 F#5 A5 | \
         A5:6 G5:2 F5:8 | F5:6 Eb5:2 B4:8 | C5:4 E5 G5 B5 | B5:8 Ab5:8",
        "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | bII7",
    )],
    form: &["Tune", "Tune", "Tune"],
    drums_from: 0,
    about: "Bossa nova chords: D7 (II7) borrowed from another key, and D flat 7 (bII7) \
            standing in for G7. The tune leans on each chord's colour notes: F sharp over \
            D7, E flat and B over D flat 7.",
};

pub(super) const CARNIVAL_MORNING: Song = Song {
    name: "Carnival Morning",
    origin: "Written for Ferrous Lineup",
    feel: "Samba",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::Bossa,
    bass: Bass::Bossa,
    rhythm: "Samba",
    band: SAMBA,
    style: "Samba",
    parts: &[
        Part::new(
            "A",
            "-:2 G5:2 E5 G5 C6:4 G5 | -:2 A5:2 C#6 A5 E5:4 G5 | F5:2 A5 D6:4 C6:2 A5 F5:4 | \
             G5:2 F5 D5 B4 G4:8",
            "Imaj7 | VI7 | ii7 | V7",
        ),
        Part::new(
            "B",
            "E5:2 E5 G5 E5 C6 B5 G5:4 | C#6:2 C#6 A5 G5 E5:4 C#5 | D5:2 F5 A5 C6 A5:4 F5 | \
             D5:2 B4 G4 B4 C5:8",
            "Imaj7 | VI7 | ii7 | V7",
        ),
    ],
    form: &["A", "A", "B", "A"],
    drums_from: 0,
    about: "Samba is bossa nova's loud parent: the same jazz chords, with a whole \
            percussion section. A7 (VI7) is a secondary dominant that leads to D minor.",
};

const TICO_A_CHORDS: &str = "i V7 | V7 i | iv i | V7 i";

const TICO_B_CHORDS: &str = "I V7 | V7 I | I V7 | V7 I | I V7 | V7 I | \
                             IV #iv°7 I VI7 | ii7 V7 I I";

const TICO_C_CHORDS: &str = "I | I V7 | V7 | V7 I | I | VI7 ii | ii #iv°7 iii7 VI7 | ii7 V7 I I";

// Source: https://github.com/oriolcolomefont/The-Sound-of-AI-workshop/blob/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/ZequinhaAbreuTicoticoSamba.abc
// (a Wikifonia lead sheet with chords, in 2/4), checked against Henrik Norbeck's
// ABC, which also gives the repeats of the rondo:
// https://github.com/syntithenai/abc2book/blob/master/abcresources/norbeck/abc_tune_norbeck_2893.abc
pub(super) const TICO_TICO: Song = Song {
    name: "Tico-Tico no Fubá",
    origin: "Zequinha de Abreu, 1917",
    feel: "Choro",
    genre: Genre::Latin,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Choro",
    band: FLUTE_AND_GUITAR,
    style: "Choro",
    parts: &[
        Part::new("Pickup", "-:13 E4:1 D#4 E4", "i"),
        Part::new(
            "A",
            "F4:1 E4:2 A4:1 -:1 E4:1 D#4 E4 F4:1 E4:2 G#4:1 -:1 E4:1 D#4 E4 | \
             F4:1 E4 D5 B4 G#4 E4 D4 Db4 C4:4 -:1 A4:1 Ab4 G4 | \
             F4:1 A4:2 D5:1 -:1 C5:1 A4 F4 E4:1 A4:2 C5:1 -:1 C5:1 B4 A#4 | \
             B4:1 B3 D#4 F#4 A4 C5 B4 A4 G#4:4 -:1 E4:1 D#4 E4",
            "i V7 | V7 i | iv i | II7 V",
        ),
        Part::new(
            "A into B",
            "F4:1 E4:2 A4:1 -:1 E4:1 D#4 E4 F4:1 E4:2 G#4:1 -:1 E4:1 D#4 E4 | \
             F4:1 E4 D5 B4 G#4 E4 D4 Db4 C4:4 -:1 A4:1 Ab4 G4 | \
             F4:1 A4:2 D5:1 -:1 C5:1 A4 F4 E4:1 A4:2 C5:1 -:1 C5:1 B4 A#4 | \
             B4:1 E4 G#4 B4 E5 D5 C5 B4 A4:4 -:1 B4:1 A4 Ab4",
            TICO_A_CHORDS,
        ),
        Part::new(
            "A into C",
            "F4:1 E4:2 A4:1 -:1 E4:1 D#4 E4 F4:1 E4:2 G#4:1 -:1 E4:1 D#4 E4 | \
             F4:1 E4 D5 B4 G#4 E4 D4 Db4 C4:4 -:1 A4:1 Ab4 G4 | \
             F4:1 A4:2 D5:1 -:1 C5:1 A4 F4 E4:1 A4:2 C5:1 -:1 C5:1 B4 A#4 | \
             B4:1 E4 G#4 B4 E5 D5 C5 B4 A4:4 -:1 A4:1 C#5 E5",
            TICO_A_CHORDS,
        ),
        Part::new(
            "A end",
            "F4:1 E4:2 A4:1 -:1 E4:1 D#4 E4 F4:1 E4:2 G#4:1 -:1 E4:1 D#4 E4 | \
             F4:1 E4 D5 B4 G#4 E4 D4 Db4 C4:4 -:1 A4:1 Ab4 G4 | \
             F4:1 A4:2 D5:1 -:1 C5:1 A4 F4 E4:1 A4:2 C5:1 -:1 C5:1 B4 A#4 | \
             B4:1 E4 G#4 B4 E5 D5 C5 B4 A4:4 -:4",
            TICO_A_CHORDS,
        ),
        Part::new(
            "B",
            "G4:1 C4 E4 G4 C4 E4 G4 G#4 A4:2 F4:2 -:1 B4:1 A4 Ab4 | \
             G4:1 B3 D4 F4 B3 D4 F4 G4 A4:2 E4:2 -:1 C5:1 C5 C5 | \
             C5:1 B4 B4 B4:2 A4:1 A4 A4 A4:1 F4 F4 F4 -:1 B4:1 B4 B4 | \
             B4:1 G4 G4 B4 A4 F4 F4 A4 G4:4 -:1 B4:1 A4 Ab4 | \
             G4:1 C4 E4 G4 C4 E4 G4 G#4 A4:2 F4:2 -:1 B4:1 A4 Ab4 | \
             G4:1 B3 D4 F4 B3 D4 F4 G4 A4:2 E4:2 -:1 C5:1 B4 Bb4 | \
             A4:1 G#4 A4 B4 D5 C5 B4 C5 E5 G4 C5 E5 G5 Gb5 F5 E5 | \
             D5:1 C5 B4 A4 G4 F4 E4 D4 C4:4 -:1 B4:1 A4 Ab4",
            TICO_B_CHORDS,
        )
        .in_key(C, Major),
        Part::new(
            "B into A",
            "G4:1 C4 E4 G4 C4 E4 G4 G#4 A4:2 F4:2 -:1 B4:1 A4 Ab4 | \
             G4:1 B3 D4 F4 B3 D4 F4 G4 A4:2 E4:2 -:1 C5:1 C5 C5 | \
             C5:1 B4 B4 B4:2 A4:1 A4 A4 A4:1 F4 F4 F4 -:1 B4:1 B4 B4 | \
             B4:1 G4 G4 B4 A4 F4 F4 A4 G4:4 -:1 B4:1 A4 Ab4 | \
             G4:1 C4 E4 G4 C4 E4 G4 G#4 A4:2 F4:2 -:1 B4:1 A4 Ab4 | \
             G4:1 B3 D4 F4 B3 D4 F4 G4 A4:2 E4:2 -:1 C5:1 B4 Bb4 | \
             A4:1 G#4 A4 B4 D5 C5 B4 C5 E5 G4 C5 E5 G5 Gb5 F5 E5 | \
             D5:1 C5 B4 A4 G4 F4 E4 D4 C4:4 -:1 E4:1 D#4 E4",
            TICO_B_CHORDS,
        )
        .in_key(C, Major),
        Part::new(
            "C",
            "A5:1 A4 C#5 G#5:2 A4:1 C#5 F#5 F#5:1 A4 C#5 E5 -:1 A4:1 C#5 F#5 | \
             F#5:1 A4 C#5 E5:2 A4:1 C#5 F#5 F#5:1 B4 D5 E5 -:1 D5:1 E5 F#5 | \
             A5:1 D5 E5 G#5:2 D5:1 E5 F#5 F#5:1 B4 D5 E5 -:1 D5:1 E5 F#5 | \
             A5:1 D5 E5 G#5:2 D5:1 E5 F#5 F#5:1 A4 C#5 E5 -:1 A4:1 C#5 E5 | \
             A5:1 A4 C#5 G#5:2 A4:1 C#5 F#5 F#5:1 A4 C#5 E5 -:1 A4:1 C#5 E5 | \
             F#5:1 E5 C#5 A#4 F#5 E5 C#5 A#4 B4 A#4 B4 C#5 D5:2 -:2 | \
             D4:1 C#4 D4 E4 F#4 G#4 A4 B4 C#5 D5 D#5 E5 F#5 E5 D5 C#5 | \
             B4:1 A4 G#4 F#4 E4 D4 C#4 B3 A3:2 -:3 A4:1 C#5 E5",
            TICO_C_CHORDS,
        )
        .in_key(A, Major),
        Part::new(
            "C into A",
            "A5:1 A4 C#5 G#5:2 A4:1 C#5 F#5 F#5:1 A4 C#5 E5 -:1 A4:1 C#5 F#5 | \
             F#5:1 A4 C#5 E5:2 A4:1 C#5 F#5 F#5:1 B4 D5 E5 -:1 D5:1 E5 F#5 | \
             A5:1 D5 E5 G#5:2 D5:1 E5 F#5 F#5:1 B4 D5 E5 -:1 D5:1 E5 F#5 | \
             A5:1 D5 E5 G#5:2 D5:1 E5 F#5 F#5:1 A4 C#5 E5 -:1 A4:1 C#5 E5 | \
             A5:1 A4 C#5 G#5:2 A4:1 C#5 F#5 F#5:1 A4 C#5 E5 -:1 A4:1 C#5 E5 | \
             F#5:1 E5 C#5 A#4 F#5 E5 C#5 A#4 B4 A#4 B4 C#5 D5:2 -:2 | \
             D4:1 C#4 D4 E4 F#4 G#4 A4 B4 C#5 D5 D#5 E5 F#5 E5 D5 C#5 | \
             B4:1 A4 G#4 F#4 E4 D4 C#4 B3 A3:2 -:3 E4:1 D#4 E4",
            TICO_C_CHORDS,
        )
        .in_key(A, Major),
    ],
    form: &[
        "Pickup", "A", "A into B", "B", "B into A", "A into C", "C", "C into A", "A end",
    ],
    drums_from: 1,
    about: "A sparrow (the tico-tico) pecking at the cornmeal (the fubá). Choro is the \
            instrumental music of Rio de Janeiro around 1900, here as flute, guitar and bass. \
            This one is a rondo, A A B B A C C A: A in A minor, B in C major, its relative \
            major, and C in A major, its parallel major. Each phrase of A starts with the same \
            three notes, E, D sharp, E. Choro is in 2/4; each bar here holds two.",
};

// Source: https://github.com/oriolcolomefont/The-Sound-of-AI-workshop/blob/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/MoisesSimonsL.abc
// (a Wikifonia lead sheet with chords of the 1930 American edition, "The
// Peanut Vendor", in 2/2; the tune is the Cuban one). A bar of 2/2 is a bar
// of 4/4 here. The score's G on the last beat of a D7 bar is the bass's
// anticipation of the next bar, which the son's bass plays anyway.
pub(super) const EL_MANISERO: Song = Song {
    name: "El manisero",
    origin: "Moisés Simons, 1928",
    feel: "Son",
    genre: Genre::Latin,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 192.0,
    comping: Comping::Montuno,
    bass: Bass::Tumbao,
    rhythm: "Son",
    band: SON,
    style: "Son",
    parts: &[
        Part::new(
            "Intro",
            "-:4 G4 D4 E4 | -:4 F#4:2 E4:4 E4:2 D4:4 | -:4 G4 D4 E4 | -:4 F#4:2 E4:4 E4:2 D4:4",
            "I | V7 | I | V7",
        ),
        Part::new(
            "A",
            "-:4 B4 D5:8 | -:4 F#4:2 E4:4 E4:2 D4:4 | -:4 B4 D5:8 | -:4 F#4:2 E4:4 E4:2 D4:4 | \
             -:4 B4:2 C5 D5 E5 D5 C#5 | D5:4 D5:2 C5:4 C5:2 B4:4 | -:4 G4:2 A4 B4 C5 D5 B4 | \
             D5:4 A4:2 C5:4 B4:2 G4:4",
            "I | V7 | I | V7 | I | V7 | I | V7",
        ),
        Part::new(
            "B",
            "-:4 G4 D4 E4 | -:4 F#4:2 E4:4 E4:2 D4:4 | -:4 G4 D4 E4 | -:4 F#4:2 E4:4 E4:2 D4:4 | \
             -:4 D4 E4 G4:2 A4 | _:2 A4:4 C5 B4:2 G4:4 | -:4 D4 E4 G4:2 A4 | \
             _:2 A4:4 C5 B4:2 D5:4 | _:4 G4 D4 E4 | -:4 F#4:2 E4:4 E4:2 D4:4",
            "I | V7 | I | V7 | I | ii7 V7 | I | ii7 V7 | I | V7",
        ),
        Part::new(
            "C",
            "-:2 D5 E5 F#5 G5:4 E5:2 F#5 | _:4 D5:2 E5:4 C5:2 D5:4 | -:2 B4 C5 D5 E5:4 C5:2 D5 | \
             _:4 B4:2 C5:4 A4:2 B4:4 | -:2 G4:4 A4:2 B4:4 G4:2 A4 | _:4 F#4:2 E4:4 E4:2 D4:4 | \
             _:4 G4 D4 E4 | -:4 F#4:2 E4:4 E4:2 D4:4 | -:4 G4 D4 E4 | -:2 C5:4 D5:2 E5 D5 C5 D5 | \
             _:4 G4 D4 E4 | -:2 C5:4 D5:2 E5 F#5 E5 D5 | _:2 B4:4 C5:2 D5 E5 D5 C#5 | \
             D5:4 D5:2 C5:4 C5:2 B4:4 | -:2 G4:4 A4:2 B4 C5 D5 B4 | D5:2 A4:4 C5 B4:2 G4:4",
            "I | V7 | I | V7 | I | V7 | I | V7 | I | V7 | I | V7 | I | V7 | I I I #i°7 | ii7 V7",
        ),
    ],
    form: &["Intro", "A", "B", "C", "Intro", "A", "B", "C"],
    drums_from: 0,
    about: "The peanut seller's street cry (a pregón) made into a Cuban son: \"Maní!\" \
            (peanuts) over G and D7, a bar each, again and again. Don Azpiazu's Havana Casino \
            Orchestra recorded it in New York in 1930, and it was the first Cuban hit around \
            the world. The clave takes two bars, the bar of two hits first, as the tune does. \
            The bass plays no note on the first beat: it plays on the \"and\" of 2 and on 4, \
            where it already plays the root of the next chord.",
};

const CIELITO_REFRAIN: &str = "E5:12 | D5:8 C5:4 | A4:12 | _:12 | D5:12 | D5:8 C5:2 C5:2 | \
                               E5:4 C5:8 | _:8 G4:4 | A4:8 G4:4 | A4:4 A4:4 G4:2 G4:2 | \
                               F5:4 F5:4 D5:4 | _:4 B4:4 G4:4 | A4:4 A4:4 G4:4 | _:4 F4:4 G4:4 | \
                               E4:2 D4:2 C4:8 | _:12";

const CIELITO_REFRAIN_CHORDS: &str =
    "I | I | IV | IV | ii | V7 | I | I | I | I | IV | V7 | V7 | V7 | I | I";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201262323.abc
// (John Chambers' ABC of 2015 with the Spanish words), checked against the
// Wikifonia lead sheet, whose chord before G7 in the refrain, D minor, is
// used here: https://github.com/oriolcolomefont/The-Sound-of-AI-workshop/blob/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/MexicanfolksongCielitoLindo.abc
pub(super) const CIELITO_LINDO: Song = Song {
    name: "Cielito lindo",
    origin: "Quirino Mendoza y Cortés, 1882",
    feel: "Ranchera",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 150.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Ranchera",
    band: MARIACHI,
    style: "Ranchera",
    parts: &[
        Part::new(
            "Verse",
            "C5:4 C5:4 A4:4 | _:4 B4:4 G4:4 | C5:4 C5:4 A4:4 | _:4 B4:4 G4:4 | \
             C5:4 C5:4 A4:4 | _:4 B4:4 G4:4 | F4:4 D4:8 | _:12 | \
             B4:4 B4:4 B4:4 | B4:4 A4:4 G4:4 | F4:4 D4:4 D4:4 | _:4 E4:4 F4:4 | \
             G4:4 G4:4 G4:4 | _:4 G4:6 F4:2 | E4:2 D4:2 C4:8 | _:12",
            "I | V7 | I | V7 | I | I | ii | ii | V | V | V7 | V7 | V7 | V7 | I | I",
        ),
        Part::new("Refrain", CIELITO_REFRAIN, CIELITO_REFRAIN_CHORDS),
        Part::new("Last refrain", CIELITO_REFRAIN, CIELITO_REFRAIN_CHORDS)
            .with_tempo(&[glide(144, 116, 24), fermata(172, 20)]),
    ],
    form: &[
        "Verse",
        "Refrain",
        "Refrain",
        "Verse",
        "Refrain",
        "Last refrain",
    ],
    drums_from: 0,
    about: "\"De la Sierra Morena, cielito lindo...\": written in 1882 and sung all over \
            Mexico as if it were a folk song, here as a mariachi waltz. The verse rocks \
            between C and G7 and sinks to D minor; the refrain, \"Ay, ay, ay, ay, canta y no \
            llores\" (sing and don't cry), starts on a long high E. The last time the \
            mariachi slows down and holds the final note.",
};
