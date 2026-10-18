//! Latin: bossa nova, samba, choro, son, bolero and the Mexican ranchera.

use super::*;

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

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/GonzaloRoigQuieremeMuchoYours.abc
// (the Wikifonia lead sheet in C, 4/4 at a quarter = 140, with its chords; here up a fifth,
// in G, and at half the note values, two of its bars to one bar here, at a quarter = 70).
// Checked against the second Wikifonia lead sheet, "Yours", in F,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/GonzaloRoigAlbertGamseJackSherrYours.abc
// which has the same notes, except that in bar 14 its third note is a flat (F natural
// here) where the first has F sharp, bar 16 has a slightly different rhythm, it ties the
// F sharp of bar 6 over the rest before "E E E", and it rests in the second half of several
// long notes that the first sheet holds. Its chords are different and in places do not
// fit its own tune, so the first sheet's chords are used. Its last line has a minor iv
// (C minor here) where "Yours" has IV.
// PD: first published in 1930 or earlier: written and first sung in Havana in 1911, published
// by Viuda de Carrera, Havana, in 1921 (Wikipedia, "Yours (Quiéreme Mucho)", with its
// sources), and recorded by Tito Schipa for Victor on 12 March 1923 (Victor 929, matrix
// B-27598, Discography of American Historical Recordings). Gonzalo Roig died in 1970, so
// it rests on that publication; the English words of "Yours" (1931) are not used.
pub(super) const QUIEREME_MUCHO: Song = Song {
    name: "Quiéreme mucho",
    origin: "Gonzalo Roig, Cuba, 1911",
    feel: "Bolero",
    genre: Genre::Latin,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 70.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Bolero",
    band: SONG_AND_GUITAR,
    style: "Bolero",
    parts: &[
        Part::new(
            "Verse",
            "B4:4 C5:2 D5:2 E5:4 D5:2 C#5:2 | E5:4 D5:4 _:8 | B4:4 C5:2 D5:2 G5:4 F#5:2 D5:2 | \
             C5:8 _:8 | E5:4 D#5:2 E5:2 G5:4 F#5:2 E5:2 | G5:4 F#5:4 -:2 E5:2 E5:2 E5:2 | \
             E5:4 F#4:4 _:4 E5:4 | D5:8 _:8",
            "I | I | I | ii | ii | V7 | V7 | I",
        ),
        Part::new(
            "Verse 2",
            "B4:4 C5:2 D5:2 E5:4 D5:2 C#5:2 | E5:4 D5:4 _:8 | B4:4 C5:2 D5:2 E5:4 D5:2 C5:2 | \
             E5:8 _:8 | E5:4 D#5:2 E5:2 G5:4 F#5:2 E5:2 | \
             E5:2 D5:1 F#5:1 _:1 E5:1 D5:2 C5:4 B4:2 D5:2 | C5:8 -:2 C5:2 G4:2 A4:2 | \
             B4:3 C5:1 B4:4 _:2 -:2 A4:4",
            "I | I | VI7 | ii | IV | I I ii VI7 | ii iv | I I I V7",
        ),
        Part::new("End", "G4:16", "I"),
    ],
    form: &["Verse", "Verse 2", "End", "Verse", "Verse 2", "End"],
    drums_from: 0,
    about: "A Cuban love song, sung slowly. The tune climbs B, C, D to E and leans on the notes \
            just below, C sharp and D sharp. In the second half E7 leads to A minor, and a C \
            minor chord, borrowed from the minor key, darkens the last line before it comes home.",
};

const SIBONEY_INTRO_TUNE: &str =
    "Ab5:1 G5:2 F5:1 _:1 Eb5:1 Db5:2 C5:1 G4:2 Bb4:1 _:1 Ab4:1 G4:2 | F4:4 -:4 -:4 F4:2 Ab4:2";
const SIBONEY_A_TUNE: &str =
    "C5:8 _:4 F5:2 Db5:2 | C5:3 Ab4:1 _:1 F5:1 Db5:2 C5:3 Ab4:1 _:1 Bb4:1 G4:2";
const SIBONEY_A_CHORDS: &str = "i i i V7b9 | i V7b9 i V7b9";
const SIBONEY_REFRAIN_TUNE: &str = "A5:8 _:4 G5:2 A5:2 | \
     G5:3 F5:1 _:2 E5:2 G5:1 F5:2 E5:1 _:1 D5:1 A4:2 | Bb4:1 C5:2 E5:1 _:4 _:2 -:2 Bb4:2 C5:2 | \
     G5:8 _:4 F5:2 G5:2 | F5:3 E5:1 _:2 D5:2 _:1 C5:2 D5:1 _:1 C5:1 Bb4:2 | \
     A4:8 _:4 A4:2 C5:2 | A5:8 _:3 Bb5:1 _:2 A5:2";
const SIBONEY_REFRAIN_2_TUNE: &str = "G5:3 F5:1 _:2 E5:2 G5:1 F5:2 E5:1 _:1 D5:1 A4:2 | \
     Bb4:1 C5:2 E5:1 _:4 _:4 Bb4:2 C5:2 | G5:8 _:4 F5:2 G5:2 | \
     F5:3 E5:1 _:2 D5:2 _:1 C5:2 D5:1 _:1 C5:1 Bb4:2 | A4:8 _:2 -:2 F5:2 Db5:2 | \
     C5:3 Ab4:1 _:1 F5:1 Db5:2 C5:3 Ab4:1 _:1 Bb4:1 G4:2";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/ErnestoLecuonaSiboney.abc
// (the Wikifonia lead sheet in C minor and C major, 2/4, with its chords, some of them
// written as text: "G7-9" is G7(b9), V7b9 here; "Dm7-5" is iiø7; Ebdim is biii°; here up
// a fourth, in F minor and F major, two of its bars to one bar here). Its form is followed:
// pickup, introduction, A twice with first and second endings, B, the refrain in the
// major, two linking bars, D.S. (taken here without the repeat of A) and the coda. In bar
// 3 of B the sheet ties a B natural to a B flat; the B natural of the same bar two bars
// later is used (E here). Checked against the Wikifonia sheet with Dolly Morse's English
// words, in C, 2/2 (twice the note values), which has only the refrain,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/ErnestoLecuonaEnglishLyricsbyDollyMorseSiboney.abc
// the same notes, but it holds the last note of each of the refrain's first three lines
// two bars longer (doubtful: which is Lecuona's is not settled here), its third line starts
// E D E (A G A here) where this sheet has E F E (A B flat A), it shortens the note before
// "fascinating", and it has other chords (G dim, Dm7) in places. The introduction, A and B
// have only the one source.
// PD: first published in 1930 or earlier: written and first sung in 1927 (Wikipedia, "Siboney
// (song)"; IMSLP gives 1927 as its first publication), and published by Leo Feist, New York,
// in 1929 with Dolly Morse's English words (IMSLP scan, plate 7274-4, "revised edition";
// York University and Western Michigan University library records, both dated 1929).
// Ernesto Lecuona died in 1963, so it rests on that publication.
pub(super) const SIBONEY: Song = Song {
    name: "Siboney",
    origin: "Ernesto Lecuona, Cuba, 1927",
    feel: "Canción",
    genre: Genre::Latin,
    tonic: F,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Bolero",
    band: Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS),
    style: "Bolero",
    parts: &[
        Part::new("Pickup", "-:12 F5:2 G5:2", "i"),
        Part::new("Intro", SIBONEY_INTRO_TUNE, "i V7 | i"),
        Part::new("A", SIBONEY_A_TUNE, SIBONEY_A_CHORDS),
        Part::new("A end 1", "Ab4:8 _:4 F4:2 Ab4:2", "i"),
        Part::new("A end 2", "F4:8 _:4 C5:2 E5:2", "i"),
        Part::new(
            "B",
            "G5:8 -:4 F5:2 G5:2 | F5:3 E5:1 _:2 C5:2 -:1 Db5:2 Eb5:1 _:1 F5:1 G5:2 | \
             F5:3 E5:1 _:2 C5:2 -:1 Bb4:2 C5:1 _:1 Db5:1 Eb5:2 | C5:8 _:2 -:2 F4:2 Ab4:2",
            "V iiø7 | V7 iiø7 | V7 iiø7 | V7",
        ),
        Part::new(
            "B2",
            "C5:8 _:4 F5:2 Db5:2 | C5:3 Ab4:1 _:1 F5:1 Db5:2 C5:3 Ab4:1 _:1 Bb4:1 G4:2 | \
             F4:8 _:4 A4:2 C5:2",
            "i i i V7b9 | i V7b9 i V7b9 | I",
        ),
        // "Siboney, yo te quiero, yo me muero por tu amor".
        Part::new(
            "Refrain",
            SIBONEY_REFRAIN_TUNE,
            "I | I I iii7 biii° | V7 | V7 | V7 | I | I",
        )
        .in_key(F, Major),
        Part::new(
            "Refrain 2",
            SIBONEY_REFRAIN_2_TUNE,
            "I I iii7 biii° | V7 | V7 | V7 | I I I V7b9 | i V7b9 i V7b9",
        )
        .in_key(F, Major),
        Part::new("Link", "Ab4:8 _:2 -:2 F5:2 G5:2", "i"),
        Part::new(
            "Coda",
            "F4:4 F5:2 G5:2 Ab5:1 G5:2 F5:1 _:1 Eb5:1 Db5:2 | \
             C5:2 G4:1 Bb4:1 _:1 Ab4:1 G4:2 F4:2 -:2 C5:2 -:2 | F5:2 -:6 -:8",
            "i | V7 V7 I V7 | I",
        ),
    ],
    form: &[
        "Pickup",
        "Intro",
        "A",
        "A end 1",
        "A",
        "A end 2",
        "B",
        "B2",
        "Refrain",
        "Refrain 2",
        "Link",
        "Intro",
        "A",
        "A end 2",
        "B",
        "B2",
        "Refrain",
        "Refrain 2",
        "Coda",
    ],
    drums_from: 1,
    about: "In F minor: C7 with a flat ninth answers each phrase of the first strain, and \
            the second leans on E natural, the harmonic minor's leading note, over C and C7, \
            before coming to rest on a bright F major chord. The refrain stays in the major \
            until its last line borrows D flat and A flat from the minor again, leading back \
            to the start.",
};
