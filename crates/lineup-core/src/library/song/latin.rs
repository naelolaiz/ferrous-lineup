//! Latin: bossa nova, samba, choro, maxixe, son, bolero and the Mexican ranchera.

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

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/ErnestoNazarethCavaquinho.abc
// (the Wikifonia lead sheet, H. Laukens's arrangement in G, written in 2/2 at twice the
// note values; checked note for note against the banjo lead sheet in
// https://github.com/music4classicalguitar/music4classicalguitar.github.io/blob/master/docs/scores/Irish%20banjo/Tablature/Folk/Ernesto%20Nazareth/Ernesto%20Nazareth%20-%20Apanhei%20te%2C%20cavaquinho.mscz
// and the chords against the iReal Pro chart in https://github.com/smashub/choco,
// partitions/ireal-pro/choco/forum/jams-converted/ireal-pro-forum_4160.jams)
// Where the two sheets differ, the tune follows the banjo sheet, whose notes fit the iReal
// chords: in the second strain the first sixteenth of Wikifonia bars 25 and 33 is G, not F
// sharp, over B7 (iReal B9(b13)), and that of bar 30 is C, not D sharp (iReal B7b9). The chart
// follows the iReal chart where the Wikifonia sheet differs: C, not A minor, in bars 10 and 18;
// D7 for C diminished in the trio's bars 42 and 50; F sharp half-diminished, not F sharp 7, in
// bar 37; B7 for B in bar 30.
// PD: Ernesto Nazareth died in 1934; published in 1914.
pub(super) const APANHEI_TE_CAVAQUINHO: Song = Song {
    name: "Apanhei-te, cavaquinho",
    origin: "Ernesto Nazareth, 1914",
    feel: "Choro",
    genre: Genre::Latin,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::Habanera,
    bass: Bass::Quarters,
    rhythm: "Choro",
    band: FLUTE_AND_GUITAR,
    style: "Choro",
    parts: &[
        Part::new("Pickup", "-:13 G5:1 F#5 E5", "V7"),
        Part::new(
            "Intro",
            "E5:1 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 G4 F#4 E4 Eb4 D4:2 -:3 G5:1 F#5 E5",
            "V7 I | V7",
        ),
        Part::new(
            "A",
            "E5:1 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 F#5 E5 D5 C5 A#4 B4 E5 D5 - G5 F#5 E5 | \
             E5 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 D#5 E5 F5 F#5 G5:4 -:1 G5 F#5 E5",
            "V7 I | III7 vi | IV #iv°7 I I | V7 I | V7 I | III7 vi | IV #iv°7 I I | V7 I",
        ),
        Part::new(
            "A into B",
            "E5:1 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 F#5 E5 D5 C5 A#4 B4 E5 D5 - G5 F#5 E5 | \
             E5 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 D#5 E5 F5 F#5 G5:4 -:1 B4 C5 B4",
            "V7 I | III7 vi | IV #iv°7 I I | V7 I | V7 I | III7 vi | IV #iv°7 I I | V7 I",
        ),
        Part::new(
            "B",
            "A#4:1 B4 E5 F#5 G5 B4 G5 B4 G5 A#4 C#5 F#5:2 A#4:1 F#5 A#4 | \
             G5 A4 B4 F#5:2 A4:1 F#5 A4 F#5 G4 B4 E5 - B4 C5 B4 | \
             A#4 B4 E5 F#5 G5 B4 G5 B4 G5 B4 F#5 B4 E5 B4 D5 B4 | \
             D5 E4 F#4 C#5:2 E4:1 C#5 E4 C5 D#4 F#4 B4 - B4 C5 B4 | \
             A#4 B4 E5 F#5 G5 B4 G5 B4 G5 A#4 C#5 F#5:2 A#4:1 F#5 A#4 | \
             G5 A4 B4 F#5:2 A4:1 F#5 A4 F5 G#4 B4 E5 - E5 D#5 E5 | \
             F5 A4 E5 A4 D5 A4 C5 A4 C5 E4 G4 B4 - B4 A#4 B4 | \
             C5 B4 A#4 B4 E5 D#5 G5 F#5 E5:4 -:1 B4 C5 B4",
            "i II7 | V7 i | i v | II7 V7 | i II7 | V7 I7 | iv i | iiø7 V7 i i",
        )
        .in_key(E, HarmonicMinor),
        Part::new(
            "B into A",
            "A#4:1 B4 E5 F#5 G5 B4 G5 B4 G5 A#4 C#5 F#5:2 A#4:1 F#5 A#4 | \
             G5 A4 B4 F#5:2 A4:1 F#5 A4 F#5 G4 B4 E5 - B4 C5 B4 | \
             A#4 B4 E5 F#5 G5 B4 G5 B4 G5 B4 F#5 B4 E5 B4 D5 B4 | \
             D5 E4 F#4 C#5:2 E4:1 C#5 E4 C5 D#4 F#4 B4 - B4 C5 B4 | \
             A#4 B4 E5 F#5 G5 B4 G5 B4 G5 A#4 C#5 F#5:2 A#4:1 F#5 A#4 | \
             G5 A4 B4 F#5:2 A4:1 F#5 A4 F5 G#4 B4 E5 - E5 D#5 E5 | \
             F5 A4 E5 A4 D5 A4 C5 A4 C5 E4 G4 B4 - B4 A#4 B4 | \
             C5 B4 A#4 B4 E5 D#5 G5 F#5 E5:4 -:1 G5 F#5 E5",
            "i II7 | V7 i | i v | II7 V7 | i II7 | V7 I7 | iv i | iiø7 V7 i i",
        )
        .in_key(E, HarmonicMinor),
        Part::new(
            "A into C",
            "E5:1 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 F#5 E5 D5 C5 A#4 B4 E5 D5 - G5 F#5 E5 | \
             E5 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 D#5 E5 F5 F#5 G5:4 -:1 G5 F#5 G5",
            "V7 I | III7 vi | IV #iv°7 I I | V7 I | V7 I | III7 vi | IV #iv°7 I I | V7 I",
        ),
        Part::new(
            "C",
            "A5:1 G5 E5 C5 A4 G4 E4 C4 A4:2 -:1 A4 -:2 A4 | \
             -:1 G4:2 A4:1 B4 C5:2 D5:1 E5:2 -:1 E5 -:2 E5 | \
             -:1 E5 D#5 E5 F5 E5 D#5 E5 C5 E5 B4 E5 A4 E5 G4 E5 | \
             - D5 C#5 D5 E5 D5 C#5 D5 E5 D5 B4 G5 - G5 F#5 G5 | \
             A5 G5 E5 C5 A4 G4 E4 C4 A4:2 -:1 A4 -:2 A4 | \
             -:1 G4:2 A4:1 B4 C5:2 D5:1 E5:2 -:1 E5 -:2 E5 | \
             -:1 D#5:2 E5:1 F5 E5 D#5 E5 A5:2 -:1 A5 -:2 A5 | \
             -:1 G5 F5 D5 B4 G4 F4 D4 C4:2 -:3 G5:1 F#5 G5",
            "I II7 | V7 I | III7 vi | II7 V7 | I II7 | V7 III7 | VI7 II7 | V7 I",
        )
        .in_key(C, Major),
        Part::new(
            "C into A",
            "A5:1 G5 E5 C5 A4 G4 E4 C4 A4:2 -:1 A4 -:2 A4 | \
             -:1 G4:2 A4:1 B4 C5:2 D5:1 E5:2 -:1 E5 -:2 E5 | \
             -:1 E5 D#5 E5 F5 E5 D#5 E5 C5 E5 B4 E5 A4 E5 G4 E5 | \
             - D5 C#5 D5 E5 D5 C#5 D5 E5 D5 B4 G5 - G5 F#5 G5 | \
             A5 G5 E5 C5 A4 G4 E4 C4 A4:2 -:1 A4 -:2 A4 | \
             -:1 G4:2 A4:1 B4 C5:2 D5:1 E5:2 -:1 E5 -:2 E5 | \
             -:1 D#5:2 E5:1 F5 E5 D#5 E5 A5:2 -:1 A5 -:2 A5 | \
             -:1 G5 F5 D5 B4 G4 F4 D4 C4:2 -:3 G5:1 F#5 E5",
            "I II7 | V7 I | III7 vi | II7 V7 | I II7 | V7 III7 | VI7 II7 | V7 I",
        )
        .in_key(C, Major),
        Part::new(
            "A end",
            "E5:1 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 F#5 E5 D5 C5 A#4 B4 E5 D5 - G5 F#5 E5 | \
             E5 D5 C#5 D5 F#5 E5 D5 C5 C5 B4 A#4 B4 E5 D5 C5 B4 | \
             B4 A4 G#4 A4 D5 C5 B4 A4 A4 G4 F#4 G4 C5 B4 A4 G4 | \
             F#4 E4 D#4 E4 A4 G4 F#4 E4 E4 D4 C#4 D4 E4 F#4 G4 A4 | \
             B4 C5 C#5 D5 D#5 E5 F5 F#5 G5:2 - G5 -",
            "V7 I | III7 vi | IV #iv°7 I I | V7 I | V7 I | III7 vi | IV #iv°7 I I | V7 I",
        ),
    ],
    form: &[
        "Pickup", "Intro", "A", "A into B", "B", "B into A", "A into C", "C", "C into A", "A end",
    ],
    drums_from: 1,
    about: "Nazareth's polka of 1914: the first strain runs down G major in sixteenths, each \
            beat leaning on the semitone below it, over D7, G, B7 and E minor; the second turns \
            to E minor and climbs F#7 and B7 arpeggios; the trio, in C major, answers its runs \
            with short chords in a 3 + 3 + 2 rhythm. Each strain is two bars of 2/4 to a bar \
            here, in the rondo A A B B A C C A.",
};

// Source: https://raw.githubusercontent.com/baobach/abc_search/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmaZw5S1V4ZLCS2FPLuw3GP7L9dEP3Bp1oZzcje6bBFaeT.abc
// (the piano score of the Acervo Digital Chiquinha Gonzaga, 2011, edited by Douglas
// Passoni with chord symbols by Alexandre Dias, in F; the tune is the top of the right
// hand, and the trio's offbeat chords under it are left to the comping). Here in G, a tone
// higher and an octave lower than the score, so that the tune, which spans nearly three
// octaves, lies between E3 and D6.
// PD: Chiquinha Gonzaga died in 1935; published in 1877.
pub(super) const ATRAENTE: Song = Song {
    name: "Atraente",
    origin: "Chiquinha Gonzaga, 1877",
    feel: "Polca",
    genre: Genre::Latin,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Habanera,
    bass: Bass::Quarters,
    rhythm: "Polca",
    band: FLUTE_AND_GUITAR,
    style: "Polca",
    parts: &[
        // The repeated octaves over F7, then the run up to the first strain under a fermata.
        // The introduction has five bars of 2/4: a bar of rest comes first.
        Part::new(
            "Intro",
            "-:8 D5:2 D4 D4 D5 | D5 D4 D4 D5 D5 D5 D5 D5 | D5 D5 D5 D5 D5:1 C4 D4 F#4 B4 A4 F#4 D4",
            "V7 | V7 | V7",
        )
        .with_tempo(&[fermata(44, 1)]),
        Part::new(
            "A",
            "C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | B3:3 A4:1 G4:4 -:1 A#3 B3 D4 A4 G4 D4 Bb3 | \
             A3:2 D4:1 F#4 A4 D5 F#5 A5 B5 A5 A5 G#5 G#5 G5 G5 F#5 | \
             F#5 D5 D5 C5 C5 A4 A4 F#4 F#4 C4 D4 F#4 B4 A4 F#4 D4 | \
             C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | \
             B3:3 A4:1 G4:4 -:1 D#4 E4 A4 D5 C5 Eb4 C5 | \
             C5 B4 B4 D4 D4:4 -:1 C#4 C#4 B4 B4 A4 A4 F#4 | \
             G4 B3 G4 D4 B4 G4 D5 B4 G5 C4 D4 F#4 B4 A4 F#4 D4",
            "V7 | I I I I I I I i | V II7 | V7 | V7 | I I I I ii ii ii iv°7 | I I II7 V7 | I",
        )
        .with_tempo(&[fermata(60, 1), fermata(124, 1)]),
        Part::new(
            "A into B",
            "C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | B3:3 A4:1 G4:4 -:1 A#3 B3 D4 A4 G4 D4 Bb3 | \
             A3:2 D4:1 F#4 A4 D5 F#5 A5 B5 A5 A5 G#5 G#5 G5 G5 F#5 | \
             F#5 D5 D5 C5 C5 A4 A4 F#4 F#4 C4 D4 F#4 B4 A4 F#4 D4 | \
             C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | \
             B3:3 A4:1 G4:4 -:1 D#4 E4 A4 D5 C5 Eb4 C5 | \
             C5 B4 B4 D4 D4:4 -:1 C#4 C#4 B4 B4 A4 A4 F#4 | G4 B3 G4 D4 B4 G4 D5 B4 G5:2 B4 A#4 B4",
            "V7 | I I I I I I I i | V II7 | V7 | V7 | I I I I ii ii ii iv°7 | I I II7 V7 | I",
        )
        .with_tempo(&[fermata(60, 1)]),
        // The second strain ends with the introduction again, its five bars of 2/4
        // padded with a bar of rest before the run, under its fermata.
        Part::new(
            "B",
            "C5:1 D#4 F#4 C5 C5 D#4 F#4 C5 C5 D#4 F#4 C5 C5 D#4 F#4 C5 | \
             C5 E4 G4 B4 B4 E4 G4 B4 B4 E4 G4 B4 B4 Bb4 Bb4 A4 | \
             A4 D5 D5 F#5 F#5 A5 A5 B5 B5 A5 A5 G5 G5 E5 E5 C#5 | \
             E5 D5 D5 A4 A4 F#4 F#4 D4 D4:2 B4 A#4 B4 | C5 C6 C5 C6 C5 C5 B4 F#4 | \
             B4 B4 B4 B4 B4 B4 Bb4 Bb4 | A4 A4 D5 F#5 B5 A5 G5 C#5 | D5 D4 D4 D5 D5 D4 D4 D5 | \
             D5 D6 D5 D6 D5 D6 D5 D6 | -:8 D5:1 C4 D4 F#4 B4 A4 F#4 D4",
            "III7 | vi vi vi vi vi vi vi biii°7 | V II7 | V | III7 | vi vi vi biii°7 | V II7 | V7 | \
             V7 | V7",
        )
        .with_tempo(&[fermata(156, 1)]),
        Part::new(
            "A into trio",
            "C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | B3:3 A4:1 G4:4 -:1 A#3 B3 D4 A4 G4 D4 Bb3 | \
             A3:2 D4:1 F#4 A4 D5 F#5 A5 B5 A5 A5 G#5 G#5 G5 G5 F#5 | \
             F#5 D5 D5 C5 C5 A4 A4 F#4 F#4 C4 D4 F#4 B4 A4 F#4 D4 | \
             C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | \
             B3:3 A4:1 G4:4 -:1 D#4 E4 A4 D5 C5 Eb4 C5 | \
             C5 B4 B4 D4 D4:4 -:1 C#4 C#4 B4 B4 A4 A4 F#4 | G4 B3 G4 D4 B4 G4 D5 B4 G5:2 G4 F#4 F4",
            "V7 | I I I I I I I i | V II7 | V7 | V7 | I I I I ii ii ii iv°7 | I I II7 V7 | I",
        )
        .with_tempo(&[fermata(60, 1)]),
        Part::new(
            "Trio",
            "E4:2 - G4 - A4:1 A3 D4 F4 A4 A3 Ab4 Ab3 | G4:2 - F4 - E4:1 E3 G3 C4 E4 G4 C5 E5 | \
             G5:2 - A5 - B5:1 B4 D5 G5 B5 D5 B5 D5 | \
             D6 C6 A5 F#5 D5 C5 A4 F#4 G4 G5 F5 D5 B4 G4 F4 D4 | \
             E4:2 - G4 - A4:1 A3 D4 F4 A4 A3 Ab4 Ab3 | G4:2 - F4 - E4:1 E3 G3 C4 E4 G4 C5 E5 | \
             G5:2 - A5 C#5:1 G5 G5 F5 F5 A4 E5:2 D5 | D5:1 C5 C5 E4 A4 G4 G4 B3 C4:4 C5",
            "I IV | V7 I | I i V V | II7 II7 V V7 | I ii | V7 I | I bvii°7 ii iv°7 | I V7 I I",
        )
        .in_key(C, Major),
        Part::new(
            "A end",
            "C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | B3:3 A4:1 G4:4 -:1 A#3 B3 D4 A4 G4 D4 Bb3 | \
             A3:2 D4:1 F#4 A4 D5 F#5 A5 B5 A5 A5 G#5 G#5 G5 G5 F#5 | \
             F#5 D5 D5 C5 C5 A4 A4 F#4 F#4 C4 D4 F#4 B4 A4 F#4 D4 | \
             C4:3 B4:1 A4:4 -:1 C4 D4 F#4 B4 A4 F#4 C4 | \
             B3:3 A4:1 G4:4 -:1 D#4 E4 A4 D5 C5 Eb4 C5 | \
             C5 B4 B4 D4 D4:4 -:1 C#4 C#4 B4 B4 A4 A4 F#4 | G4 B3 G4 D4 B4 G4 D5 B4 G5:2 -:6",
            "V7 | I I I I I I I i | V II7 | V7 | V7 | I I I I ii ii ii iv°7 | I I II7 V7 | I",
        )
        .with_tempo(&[fermata(60, 1)]),
    ],
    form: &["Intro", "A", "A into B", "B", "A into trio", "Trio", "Trio", "Intro", "A end"],
    drums_from: 1,
    about: "A polka by Chiquinha Gonzaga, published in 1877. The first strain waits two bars on \
            D7 before G arrives, climbs an arpeggio to the high B and slides down in semitones \
            over A7 and D7; the second leans on E minor through its own dominant, B7; the trio \
            moves to C, the subdominant, in leaping octaves. Each bar here holds two bars of \
            2/4; the score is in F, a tone lower and an octave higher.",
};

// Source: https://raw.githubusercontent.com/baobach/abc_search/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmYH1DJQdxPZabqsHWojoQRXzqj3NEvm4XWa5YiNfgtMHW.abc
// (abc_search: a piano score in G sharp minor, 2/4, "Con dolcezza", quarter = 70; the tune is the
// top of the right hand, the chords are read from the left hand)
// Form, dedication and date checked against the Musica Brasilis edition,
// https://musicabrasilis.com/media/en_carioca.pdf, and IMSLP, https://imslp.org/wiki/Carioca_(Nazareth,_Ernesto)
// (first edition Sampaio, Araújo & Cia., Rio de Janeiro, 1913).
// Here in E minor, a major third lower, with the tune on the violin of a salon trio. The
// score's runs climb to B6 and E7; to keep the tune between G3 and C6 it is taken an octave
// lower in places (bar numbers count the pickup as bar 1): in the second strain from the leap
// in bar 23 to the first note of bar 26, and from bar 39 to bar 42 the same; in the trio from
// the pickup at the end of bar 57 to the second note of bar 61, and from the pickup at the end
// of bar 65 to the first note of each ending (bars 69 and 70; the last eighth of the second
// ending stays up, as written).
// Bar 34 is a bar of rest for the tune, whose run goes on in the left hand, and the F sharp at
// the end of bar 49 (here D), written across the staves, is left to the bass.
// PD: published in 1913; Ernesto Nazareth died in 1934.
pub(super) const CARIOCA: Song = Song {
    name: "Carioca",
    origin: "Ernesto Nazareth, 1913",
    feel: "Tango brasileiro",
    genre: Genre::Latin,
    tonic: E,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 70.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Maxixe",
    band: SALON,
    style: "Maxixe",
    parts: &[
        Part::new("Pickup", "-:14 B3:2", ""),
        // "A" also follows the bridge: back to the tempo after its rit.
        Part::new(
            "A",
            "E4:1 F#4:2 G4 A4 B4:1 C5:3 B4:1 -:2 B4 | \
             E5:1 F#5:2 G5:1 G5 F#5:2 E5:1 C6:3 B5:1 -:2 B3 | \
             E4:1 F#4:2 G#4 A4 B4:1 D5:3 C5:1 - C5:2 B4:1 | \
             Bb4:2 G5 F#5 E5 D#5:1 C6 B5 A5 F#5 D#5 B4 B3 | E4 F#4:2 G4 A4 B4:1 C5:3 B4:1 -:2 B4 | \
             E5:1 F#5:2 G5:1 G5 F#5:2 E5:1 C6:3 B5:1 - B4:2 A4:1 | \
             G#4 F5:2 E5:1 D5 C5:2 B4:1 E5 A4:2 G4:1 C5 F4:2 E4:1 | \
             G4 F4:2 E4:1 D#4 B4:2 D#4:1 E4:4 -:2 B3",
            "i V7 | i V7 | I7 iv | bVI7 V7 | i V7 | i V7 | I7 I7 iv bVI7 | bII V7 i V",
        )
        .with_tempo(&[tempo(0, 70)]),
        // The second ending, which is also the Fine.
        Part::new(
            "A into B",
            "E4:1 F#4:2 G4 A4 B4:1 C5:3 B4:1 -:2 B4 | \
             E5:1 F#5:2 G5:1 G5 F#5:2 E5:1 C6:3 B5:1 -:2 B3 | \
             E4:1 F#4:2 G#4 A4 B4:1 D5:3 C5:1 - C5:2 B4:1 | \
             Bb4:2 G5 F#5 E5 D#5:1 C6 B5 A5 F#5 D#5 B4 B3 | E4 F#4:2 G4 A4 B4:1 C5:3 B4:1 -:2 B4 | \
             E5:1 F#5:2 G5:1 G5 F#5:2 E5:1 C6:3 B5:1 - B4:2 A4:1 | \
             G#4 F5:2 E5:1 D5 C5:2 B4:1 E5 A4:2 G4:1 C5 F4:2 E4:1 | \
             G4 F4:2 E4:1 D#4 B4:2 D#4:1 E4:2 - E5 -",
            "i V7 | i V7 | I7 iv | bVI7 V7 | i V7 | i V7 | I7 I7 iv bVI7 | bII V7 i i",
        ),
        Part::new(
            "B",
            "E5:2 - D5:4 -:1 Eb5:2 C5:1 A4 F#4 E4 C4 | \
             B3 C4:2 C#4:1 D4 E4:2 F#4:1 G4 A4:2 A#4:1 B4 E5:2 D5:1 | \
             C#5 D5:2 D#5:1 E5 B4:2 A4:1 F#4 G4:2 G#4:1 A4 G5:2 E5:1 | \
             C#5 E5 D5 A#4 C5 B4 F#4 A4 G4 C#5 E5 D5 B4 G4 E4 D4 | \
             E4:2 - D4:4 -:1 B4:2 A4:1 F#4 D#4 C4 A3 | \
             G3 A3:2 A#3:1 B3 C#4:2 D#4:1 E4 F#4:2 G4:1 A4 B4:2 C#5:1 | \
             D5 E5:2 E#5:1 F#5 B5:2 A5:1 G5 B4:2 D5:1 C#5 F#5:2 E5:1 | \
             D5 F#5 D5 F#4 D4 F#4 D4 -:9 | E5:2 - D5:4 -:1 Eb5:2 C5:1 A4 F#4 E4 C4 | \
             B3 C4:2 C#4:1 D4 E4:2 F#4:1 G4 A4:2 A#4:1 B4 E5:2 D5:1 | \
             C#5 D5:2 D#5:1 E5 B4:2 A4:1 F#4 G4:2 G#4:1 A4 G5:2 E5:1 | \
             C#5 E5 D5 A#4 C5 B4 F#4 A4 G4 C#5 E5 D5 B4 C5 D5 E5 | \
             F5:2 - E5:4 -:1 D5:2 B4:1 G#4 F4 D4 B3 | \
             A3 B3:2 C4:1 E4 A4:2 B4:1 C5 E5:2 A5:1 G5 F#5:2 E5:1 | \
             G5 F#5:2 E5:1 D5 C5:2 B4:1 E5 D5:2 C5:1 B4 A4:2 D4:1 | \
             G4 B3:2 B3:1 B3 A3:2 -:5 G4:4",
            "V7 V7b9 | I I | II7 V7 | I I | V7 V7b9 | vi vi | V V vi II7 | V V | V7 V7b9 | I I | \
             II7 V7 | I I | VI7 VI7b9 | ii ii ii #iv°7 | I vi7 ii V7 | I V7b9 I I",
        )
        .in_key(G, Major),
        // Second ending, then D.S. to the first strain, which goes to the coda.
        Part::new(
            "B into A",
            "E5:2 - D5:4 -:1 Eb5:2 C5:1 A4 F#4 E4 C4 | \
             B3 C4:2 C#4:1 D4 E4:2 F#4:1 G4 A4:2 A#4:1 B4 E5:2 D5:1 | \
             C#5 D5:2 D#5:1 E5 B4:2 A4:1 F#4 G4:2 G#4:1 A4 G5:2 E5:1 | \
             C#5 E5 D5 A#4 C5 B4 F#4 A4 G4 C#5 E5 D5 B4 G4 E4 D4 | \
             E4:2 - D4:4 -:1 B4:2 A4:1 F#4 D#4 C4 A3 | \
             G3 A3:2 A#3:1 B3 C#4:2 D#4:1 E4 F#4:2 G4:1 A4 B4:2 C#5:1 | \
             D5 E5:2 E#5:1 F#5 B5:2 A5:1 G5 B4:2 D5:1 C#5 F#5:2 E5:1 | \
             D5 F#5 D5 F#4 D4 F#4 D4 -:9 | E5:2 - D5:4 -:1 Eb5:2 C5:1 A4 F#4 E4 C4 | \
             B3 C4:2 C#4:1 D4 E4:2 F#4:1 G4 A4:2 A#4:1 B4 E5:2 D5:1 | \
             C#5 D5:2 D#5:1 E5 B4:2 A4:1 F#4 G4:2 G#4:1 A4 G5:2 E5:1 | \
             C#5 E5 D5 A#4 C5 B4 F#4 A4 G4 C#5 E5 D5 B4 C5 D5 E5 | \
             F5:2 - E5:4 -:1 D5:2 B4:1 G#4 F4 D4 B3 | \
             A3 B3:2 C4:1 E4 A4:2 B4:1 C5 E5:2 A5:1 G5 F#5:2 E5:1 | \
             G5 F#5:2 E5:1 D5 C5:2 B4:1 E5 D5:2 C5:1 B4 A4:2 D4:1 | \
             G4 B3:2 B3:1 B3 A3:2 -:5 B4:2 B3",
            "V7 V7b9 | I I | II7 V7 | I I | V7 V7b9 | vi vi | V V vi II7 | V V | V7 V7b9 | I I | \
             II7 V7 | I I | VI7 VI7b9 | ii ii ii #iv°7 | I vi7 ii V7 | I V7b9 I III7",
        )
        .in_key(G, Major),
        // The coda bar, with the pickup into the trio.
        Part::new(
            "A into trio",
            "E4:1 F#4:2 G4 A4 B4:1 C5:3 B4:1 -:2 B4 | \
             E5:1 F#5:2 G5:1 G5 F#5:2 E5:1 C6:3 B5:1 -:2 B3 | \
             E4:1 F#4:2 G#4 A4 B4:1 D5:3 C5:1 - C5:2 B4:1 | \
             Bb4:2 G5 F#5 E5 D#5:1 C6 B5 A5 F#5 D#5 B4 B3 | E4 F#4:2 G4 A4 B4:1 C5:3 B4:1 -:2 B4 | \
             E5:1 F#5:2 G5:1 G5 F#5:2 E5:1 C6:3 B5:1 - B4:2 A4:1 | \
             G#4 F5:2 E5:1 D5 C5:2 B4:1 E5 A4:2 G4:1 C5 F4:2 E4:1 | \
             G4 F4:2 E4:1 D#4 B4:2 D#4:1 E4:4 E5:1 D4:2 E4:1",
            "i V7 | i V7 | I7 iv | bVI7 V7 | i V7 | i V7 | I7 I7 iv bVI7 | bII V7 i i",
        ),
        Part::new(
            "Trio",
            "F4:1 G4 A4 B4 C5 D5 E5 F5 F#5 A5 G5 E5 C5 A4 G4 E4 | \
             D4 F#4 B4 A4 G4 B4 E5 D5 C5 E5 A5 G5 - D4:2 E4:1 | \
             F4 G4 A4 B4 C5 D5 E5 F5 E5 D5 C5 B4 A4 G4 F#4 E4 | \
             D4 D#4 F#4 E4 B3 D4 C4 A3 G3:2 G4 -:1 D4:2 E4:1 | \
             F4 G4 A4 B4 C5 D5 E5 F5 F#5 A5 G5 E5 C5 A4 G4 E4 | \
             D4 F#4 B4 A4 G4 B4 E5 D5 C5 E5 A5 G5 - C4:2 D4:1 | \
             E4 F4 G4 A4 Bb4 C5 D5 E5 F5 G5 A5 B5 C6 B5 C6 A5 | \
             G5 A5 G5 E5 D5 E5 D5 G4 C5:4 -:1 D4:2 E4:1",
            "V7 I | II7 V7 I I | V7 V7 #v°7 #v°7 vi vi vi #i°7 | V II7 V V | V7 I | II7 V7 I I | \
             I I7 IV #iv°7 | I V7 I I",
        )
        .in_key(C, Major),
        Part::new(
            "Trio end",
            "F4:1 G4 A4 B4 C5 D5 E5 F5 F#5 A5 G5 E5 C5 A4 G4 E4 | \
             D4 F#4 B4 A4 G4 B4 E5 D5 C5 E5 A5 G5 - D4:2 E4:1 | \
             F4 G4 A4 B4 C5 D5 E5 F5 E5 D5 C5 B4 A4 G4 F#4 E4 | \
             D4 D#4 F#4 E4 B3 D4 C4 A3 G3:2 G4 -:1 D4:2 E4:1 | \
             F4 G4 A4 B4 C5 D5 E5 F5 F#5 A5 G5 E5 C5 A4 G4 E4 | \
             D4 F#4 B4 A4 G4 B4 E5 D5 C5 E5 A5 G5 - C4:2 D4:1 | \
             E4 F4 G4 A4 Bb4 C5 D5 E5 F5 G5 A5 B5 C6 B5 C6 A5 | \
             G5 A5 G5 E5 D5 E5 D5 G4 C5:4 -:2 C6",
            "V7 I | II7 V7 I I | V7 V7 #v°7 #v°7 vi vi vi #i°7 | V II7 V V | V7 I | II7 V7 I I | \
             I I7 IV #iv°7 | I V7 I I",
        )
        .in_key(C, Major),
        // Over chromatic runs in the bass; rit. in its last two bars, then D.S. al Fine.
        Part::new(
            "Bridge",
            "B5:3 B5:1 -:2 A5 G5:3 G5:1 -:2 G5 | F#5:3 F#5:1 E5:2 E5 D#5:3 D#5:1 -:2 C5 | \
             B4:3 B4:1 -:2 D#5 E5:3 E5:1 -:2 E5 | G5:3 G5:1 E5:2 E5 D#5:1 B3 B4 B4 B5:2 B3",
            "V V7 i i | II7 V | V V7 i #iv°7 | i i bVI iv6 V V V V",
        )
        .with_tempo(&[glide(48, 56, 16)]),
    ],
    form: &[
        "Pickup",
        "A",
        "A into B",
        "B",
        "B into A",
        "A into trio",
        "Trio",
        "Trio end",
        "Bridge",
        "A",
        "A into B",
    ],
    drums_from: 1,
    about: "Nazareth's tango brasileiro of 1913, here in E minor (the score is in G sharp \
            minor). The first strain answers E minor with B7 and cadences through the Neapolitan \
            F major chord; the second, marked enérgico, moves to G major over the habanera bass \
            and climbs in chromatic steps; the trio, brilhante, goes to C major in running \
            sixteenths, and a bridge over chromatic bass runs leads back to the first strain. \
            Each bar here holds two bars of 2/4.",
};

// Source: https://raw.githubusercontent.com/rodrigoborgesdeoliveira/sheet-music-and-chords/8980988549406f50fd736a0510ddf439082d3a3d/joao-pernambuco/sons-de-carrilhoes/sons-de-carrilhoes.abc
// (a guitar solo score in D, 2/4, quarter = 70; the tune is the top voice, written an
// octave above the guitar's sound as guitar music is, and the chords are read from the bass).
// Here a tone lower, in C, at the written pitch: at the guitar's sounding pitch the tune would
// lie between A2 and E5, too low for the lead, and in D at the written pitch it climbs to E6;
// in C it lies between G3 and D6.
// PD: João Pernambuco died in 1947.
pub(super) const SONS_DE_CARRILHOES: Song = Song {
    name: "Sons de carrilhões",
    origin: "João Pernambuco",
    feel: "Choro",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 70.0,
    comping: Comping::Habanera,
    bass: Bass::Quarters,
    rhythm: "Choro",
    band: GUITARS,
    style: "Choro",
    parts: &[
        Part::new("Pickup", "-:13 C6:1 C5 E5", "I"),
        Part::new(
            "A",
            "B5:5 A5:1 G5 F5 A4:5 A4:1 B4 C5 | E5:5 D5:1 G5 D5 A4:5 G5:1 E5 C5 | \
             A4 G4 E4 C4 E4:2 Eb4 D4:1 A4:2 A4 A4:1 B4 C5 | E5:5 D5:1 G5 E5 A4:5 C6:1 C5 E5 | \
             B5:5 A5:1 G5 F5 A4:5 A4:1 B4 C5 | E5:5 D5:1 G5 E5 A4:5 G4:1 E4 C4 | \
             C#4 E4 G4 Bb4 C#5 E5 G5 A5 G5 F5 D5 A4 F4 D4 A3 Ab3 | \
             G3 G4 E4 C4 A4:2 B4 C5:4 -:1 C6 C5 E5",
            "Imaj7 ii | V7 I6 | I ii | V7 I6 | Imaj7 ii7 | V7 I6 | VI7b9 ii | I I ii V7 I I I I",
        ),
        // The second ending, which is also the Fine.
        Part::new(
            "A end",
            "B5:5 A5:1 G5 F5 A4:5 A4:1 B4 C5 | E5:5 D5:1 G5 D5 A4:5 G5:1 E5 C5 | \
             A4 G4 E4 C4 E4:2 Eb4 D4:1 A4:2 A4 A4:1 B4 C5 | E5:5 D5:1 G5 E5 A4:5 C6:1 C5 E5 | \
             B5:5 A5:1 G5 F5 A4:5 A4:1 B4 C5 | E5:5 D5:1 G5 E5 A4:5 G4:1 E4 C4 | \
             C#4 E4 G4 Bb4 C#5 E5 G5 A5 G5 F5 D5 A4 F4 D4 A3 Ab3 | G3 G4 E4 C4 A4:2 B4 C5:4 C6",
            "Imaj7 ii | V7 I6 | I ii | V7 I6 | Imaj7 ii7 | V7 I6 | VI7b9 ii | I I ii V7 I I I I",
        ),
        Part::new(
            "B",
            "C5:1 C5:2 C5:1 C#5:2 C#5 E5:5 Bb5:1 E5 Bb5 | Bb5 Bb5:2 Bb5:1 Bb5:2 Bb5 C6:5 A5:1 D6 C6 | \
             A5 F5 C5 A4 F4 A4 C5 C6 Bb5 A5 G5 F#5 F5 E5 D5 C5 | E5:5 D5:1 G5 E5 D5:4 C5 | \
             C5:1 C5:2 C5:1 C#5:2 C#5 E5:5 Bb5:1 E5 Bb5 | Bb5:2 Bb5:1 Bb5 Bb5:2 Bb5 C6:5 A5:1 D6 C6 | \
             A5 F#5 D5 C5 A4 F#4 D4 C4 Bb3 D4 G4 Bb4 Db4 F4 Ab4 Db5 | C4 F4 A4 C5 D5:2 C5 F5:4 -",
            "I I+ V7 V7 | V7 I | I I | ii7 V7 | I I+ V7 V7 | V7 I | VI7 VI7 ii bVI | I V7 I I",
        )
        .in_key(F, Major),
        // Second ending, then D.S. to the first strain, which ends at the Fine.
        Part::new(
            "B into A",
            "C5:1 C5:2 C5:1 C#5:2 C#5 E5:5 Bb5:1 E5 Bb5 | Bb5 Bb5:2 Bb5:1 Bb5:2 Bb5 C6:5 A5:1 D6 C6 | \
             A5 F5 C5 A4 F4 A4 C5 C6 Bb5 A5 G5 F#5 F5 E5 D5 C5 | E5:5 D5:1 G5 E5 D5:4 C5 | \
             C5:1 C5:2 C5:1 C#5:2 C#5 E5:5 Bb5:1 E5 Bb5 | Bb5:2 Bb5:1 Bb5 Bb5:2 Bb5 C6:5 A5:1 D6 C6 | \
             A5 F#5 D5 C5 A4 F#4 D4 C4 Bb3 D4 G4 Bb4 Db4 F4 Ab4 Db5 | \
             C4 F4 A4 C5 D5:2 C5 F5:4 -:1 C6 C5 E5",
            "I I+ V7 V7 | V7 I | I I | ii7 V7 | I I+ V7 V7 | V7 I | VI7 VI7 ii bVI | I V7 I I",
        )
        .in_key(F, Major),
    ],
    form: &["Pickup", "A", "A end", "B", "B into A", "A end"],
    drums_from: 1,
    about: "A choro for the guitar by João Pernambuco; its title means sounds of bells. The \
            first strain, in C, rings out in octave leaps from the high C over C major seventh, \
            D minor and G7; the second moves to F, the subdominant, through an augmented F chord \
            and C7, and turns through D7, G minor and D flat before its cadence. Each bar here \
            holds two bars of 2/4; the score is in D, a tone higher.",
};
