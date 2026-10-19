//! Latin: bossa nova, samba, choro, maxixe, marchinha, son, bolero and the Mexican ranchera.

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

// Source: https://github.com/soprodecarnaval/cadern.in (cadernin_2026 flute book, page 15, "Abre
// alas", marchinha no. 1, in G minor, 2/4, with chords Gm D7 Cm: A with a repeat, B, D.C.),
// written a fourth higher, in C minor.
// Second source: https://chiquinhagonzaga.com/acervo/partituras/o-abre-alas_canto-e-piano_cifra.pdf
// (the Chiquinha Gonzaga archive's voice and piano score, in F minor, read through WebFetch: chords
// Fm C7 / Fm C7 Fm Bbm Fm C7, first and second endings, the second marked Fine). Only its chords
// and form could be read, not its notes; the form here ends on the last bar of B, as its Fine does.
// The 2/4 bars are paired.
// Date: written in 1899 for the Rio carnival group (cordão) Rosa de Ouro, https://www.cnnbrasil.com.br/
// pop/carnaval/carnaval-qual-foi-a-primeira-marchinha-da-historia/
// PD: Chiquinha Gonzaga, the only author, died in 1935
pub(super) const O_ABRE_ALAS: Song = Song {
    name: "Ó abre alas",
    origin: "Chiquinha Gonzaga, 1899",
    feel: "Marchinha",
    genre: Genre::Latin,
    tonic: C,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Offbeat,
    bass: Bass::RootFifth,
    rhythm: "Marchinha",
    band: BRASS_BAND,
    style: "Marchinha",
    parts: &[
        Part::new("Pickup", "-:8 -:2 Eb5 Eb5 D5", "i"),
        Part::new(
            "A",
            "D5:2 C5:4 Eb5 D5:2 C5 B4 | C5:8 -:2 Eb5 Eb5 D5",
            "i V7 | i",
        ),
        Part::new(
            "A into B",
            "D5:2 C5:4 Eb5 D5:2 C5 B4 | C5:8 -:2 C5 C5 Bb4",
            "i V7 | i",
        ),
        Part::new(
            "B",
            "Bb4:2 Ab4:4 C5 C5:2 Bb4 Ab4 | G4:8 -:2 G4 B4 D5 | G5:4 D5:2 G5:4 F5:2 Eb5 D5",
            "iv | i | V7",
        ),
        Part::new("B back", "C5:8 -:2 Eb5 Eb5 D5", "i"),
        Part::new("B end", "C5:8 -", "i"),
    ],
    form: &[
        "Pickup", "A", "A into B", "B", "B back", "A", "A into B", "B", "B end",
    ],
    drums_from: 1,
    about: "Counted as the first carnival march, written for the Rio carnival group Rosa de \
            Ouro: C minor with only three chords, C minor, G7 and F minor. Every phrase starts \
            after a rest, three quick notes running into the bar line, the shout of a crowd on \
            the march. The march is in 2/4, two bars to a bar here.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/
// wikifonia-dataset/original-abc/NiloMendezAdolfoUtreraGreenEyes.abc (Wikifonia lead sheet "Green Eyes",
// Nilo Mendez, in E flat with chord symbols and first and second endings), written a tone higher, in F.
// Second source: the same dataset's NiloMenencoAquellosOjosVerdes.abc ("Aquellos Ojos Verdes", in F):
// the same pitches bar for bar, in plainer rhythm (no anticipations) and with plainer chords
// (F, C7, Gm, D7, G7, Bb). Slash chords are written as the chord itself; the Bb/C of bar 10 is
// written ii7 (G minor 7, which the F sheet has there); bar 24's "Fm C7" (Gm D7 here) is ii, as the
// F sheet has it. The first ending's stacked "Eb Eb6 Edim7" (F F6 F#dim7 here) is spread as
// I for two beats, I6, #i°7 (the positions are a guess; the source prints all three on beat 1).
// Published: c1929 (The Morgan Library, https://www.themorgan.org/music-manuscripts-and-printed-music/213977)
// PD: first published in 1929
pub(super) const AQUELLOS_OJOS_VERDES: Song = Song {
    name: "Aquellos ojos verdes",
    origin: "Nilo Menéndez, 1929",
    feel: "Bolero",
    genre: Genre::Latin,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Bolero",
    band: SONG_AND_GUITAR,
    style: "Bolero",
    parts: &[
        Part::new("Pickup", "-:6 C4:2 F4 G4 A4 C5", "I"),
        Part::new(
            "A",
            "E5:6 F5:10 | _:4 -:2 E5 F5 E5 D5 C5 | D5:6 E5:10 | _:4 -:2 C4 F4 G4 A4 Bb4 | \
             C5:6 D5:10 | _:4 -:2 C5 D5 C5 Bb4 A4 | C5:16 | _:4 -:2 C4 G4 A4 Bb4 C5",
            "Imaj7 I | I | I6 Imaj7 | Imaj7 | I I6 | #i°7 | V7 | V7",
        ),
        Part::new(
            "A2",
            "E5:6 F5:10 | _:4 -:2 E5 F5 E5 D5 C5 | D5:6 E5:10 | _:4 -:2 D5 E5 D5 C5 B4 | \
             Bb4:6 A4:10 | _:2 G4:4 A4:2 B4:4 C5:2 D5 | E5:16 | _:4 -:2 C4 F4 G4 A4 C5",
            "V7 ii7 | ii7 | V7 | V7 | VI+ VI7 | II7 | V7 | V7",
        ),
        Part::new(
            "B",
            "E5:6 F5:10 | _:4 -:2 E5 F5 E5 D5 C5 | D5:6 E5:10 | _:4 -:2 C4 F4 G4 A4 Bb4 | \
             C5:6 D5:10 | _:4 -:2 C5 D5 C5 Bb4 A4 | D5:16 | _:4 -:2 D4 G4 A4 Bb4 D5",
            "Imaj7 I | I | I6 Imaj7 | Imaj7 | #i°7 VI7 | VI7 | ii | ii",
        ),
        Part::new(
            "C",
            "G5:6 A5:10 | _:6 G5:2 A5 G5 F5 D5 | F5:6 C5:10 | -:6 C5:2 D5 C5 Bb4 A4 | D5:6 E5 F5:4 | \
             G5:8 F5:2 E5 D5 E5",
            "IV6 IVmaj7 | IVmaj7 IVmaj7 IVmaj7 #iv°7 | I v6 | VI7 | II7 | V7",
        ),
        Part::new(
            "First ending",
            "F5:16 | -:6 C4:2 F4 G4 A4 C5",
            "I I I6 #i°7 | V7",
        ),
        Part::new("Second ending", "F5:16 | _:12 -:4", "I | I"),
    ],
    form: &[
        "Pickup",
        "A",
        "A2",
        "B",
        "C",
        "First ending",
        "A",
        "A2",
        "B",
        "C",
        "Second ending",
    ],
    drums_from: 1,
    about: "A Cuban bolero in F major. Most phrases run up from low C to a long note that \
            arrives half a beat early, the bolero's anticipation. The harmony is rich for 1929: \
            major sevenths and sixths on F, a diminished chord on F sharp sliding into the \
            dominant, and secondary dominants, D7 to G minor and G7 to C7, before the last \
            phrase comes home.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/
// wikifonia-dataset/original-abc/LorenzoBarcelataS.abc (Wikifonia lead sheet "Maria Elena", Lorenzo
// Barcelata, in C with chord symbols, one repeat with first and second endings), written a fourth
// higher, in F. Single source. "Ebdim" and "Cdim" are written as triads, biii° and i°; bar 24's
// "Am7 Ab7" on one whole note is split half and half; bar 32's "C Cdim" likewise.
// PD: Lorenzo Barcelata, the only author, died in 1943 (https://en.wikipedia.org/wiki/Lorenzo_Barcelata)
pub(super) const MARIA_ELENA: Song = Song {
    name: "María Elena",
    origin: "Lorenzo Barcelata, Mexico",
    feel: "Bolero",
    genre: Genre::Latin,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Bolero",
    band: SONG_AND_GUITAR,
    style: "Bolero",
    parts: &[
        Part::new("Pickup", "-:6 F5:2 E5 F5 E5 D5", "I"),
        Part::new(
            "A",
            "F5:12 A4:4 | C5:10 E5:2 E5 Eb5 | D5:16 | _:6 G5:2 F#5 G5 F#5 F5 | E5:12 G4:4 | \
             C5:10 E5:2 C5 E5 | A4:16 | _:12 C5:4 | A5:10 F5:2 G5 E5 | F5:10 F4:2 A4 C5 | \
             E5:12 D5:4 | Bb4:12 C5:4 | G5:10 E5:2 F5 D5 | E5:10 G4:2 A4 Bb4 | E5:12 D5:4 | \
             A4:6 F5:2 E5 F5 E5 D5",
            "I | iii7 | ii | ii | V7 | V7 | I | I | I | I | biii° | ii7 | ii7 | V7 | I | \
             I",
        ),
        Part::new(
            "A2",
            "F5:12 A4:4 | C5:10 E5:2 E5 Eb5 | D5:16 | _:6 G5:2 F#5 G5 F#5 F5 | E5:12 A4:4 | \
             C#5:8 A4 | F5:16 | _:12 F5:4 | G5:10 F5:2 G5 F5 | G5:10 F5:2 G5 G#5 | A5:16 | \
             _:10 A4:2 C5 E5 | D5:10 B4:2 D5 G5 | E5:10 C5:2 E5 A5",
            "I | iii7 | ii | ii | viiø7 | III7 | vi7 bVI7 | v7 v7 v7 I7 | IVmaj7 | iv6 | \
             iii7 | vi7 | II9 | V13",
        ),
        Part::new("First ending", "F5:16 | _:6 F5:2 E5 F5 E5 D5", "I i° | V7"),
        Part::new("Second ending", "F5:16", "I"),
    ],
    form: &[
        "Pickup",
        "A",
        "A2",
        "First ending",
        "A",
        "A2",
        "Second ending",
    ],
    drums_from: 1,
    about: "A Mexican bolero in F major. The first half stays close to home, I, iii7, ii and V7, \
            with a passing diminished chord; the second half travels: E half-diminished and A7 \
            lead to D minor, then B flat major turns to B flat minor (iv6, borrowed from F \
            minor) and G9 and C13 walk back round the circle of fifths.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/
// wikifonia-dataset/original-abc/MariaGreverMagicIsTheMoonlight.abc (Wikifonia lead sheet "Magic Is The
// Moonlight", the English version of "Te quiero dijiste", María Grever, in F, 2/2, with chord symbols and
// first and second endings), written a fourth higher, in B flat. Single source. Chords written at the same spot are read in order
// (bar 24: C7, Gm, Gm7 is written ii7; bar 32: F, Bdim, split half and half; bar 34: F, Eb9);
// bar 19's C+ on the last eighth is put on
// beat 4.
// Published: Southern Music, New York, 1930 (https://imslp.org/wiki/Te_Quiero_Dijiste_(Grever,_Mar%C3%ADa))
// PD: first published in 1930; also, María Grever, the only author, died in 1951
pub(super) const TE_QUIERO_DIJISTE: Song = Song {
    name: "Te quiero dijiste",
    origin: "María Grever, 1930",
    feel: "Bolero",
    genre: Genre::Latin,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 84.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Bolero",
    band: SONG_AND_GUITAR,
    style: "Bolero",
    parts: &[
        Part::new("Pickup", "-:8 F4:2 G4 Bb4 C5", "I"),
        Part::new(
            "A",
            "D5:8 D5 | _ F5:2 D5 C5 Bb4 | D5:8 D5 | _ F5:2 D5 C5 Bb4 | C5:8 C5 | _ C5:2 Bb4 A4 C5 | \
             Bb4:16 | _:8 F4:2 G4 Bb4 C5 | D5:8 D5 | _ F5:2 D5 C5 Bb4 | D5:16 | _:8 F5:2 D5 C5 Bb4 | \
             C5:8 C5 | _ C5:2 Bb4 A4 C5 | Bb4:16 | _:8 Bb4:2 C5 Eb5 F5",
            "I I6 | I I6 | I I6 | I I6 | V7 V9 | V7 V9 | I | I I6 | I I6 | I I6 | I | I I6 | \
             V7 V9 | V7 V9 | I | I9",
        ),
        Part::new(
            "B",
            "G5:8 G5 | _ A5:2 G5 Bb5 A5 | A5:8 F5 | _ G5:2 F5 A5 G5 | G5:8 Eb5 | _ D5:2 C5 Bb4 D5 | \
             C5:16 | _:8 F4:2 G4 Bb4 C5 | D5:8 D5 | _ F5:2 D5 C5 Bb4 | D5:8 D5 | _ F5:2 D5 C5 Bb4 | \
             C5:8 C5 | _ C5:2 Bb4 A4 C5",
            "IV | IV IV ii V+ | Imaj7 I | I I I #iv° | V9 V7 | V7 II7 | ii7 | V7 | I I6 | \
             I I6 | I I6 | I I6 | V7 V9 | V7",
        ),
        Part::new(
            "First ending",
            "Bb4:16 | _:4 - F4:2 G4 Bb4 C5",
            "I #iv° | V9",
        ),
        Part::new("Second ending", "Bb4:16 | _:12 -:4", "I bVII9 | I"),
    ],
    form: &[
        "Pickup",
        "A",
        "B",
        "First ending",
        "A",
        "B",
        "Second ending",
    ],
    drums_from: 1,
    about: "\"Muñequita linda\": a Mexican song in B flat major whose first half rocks between B \
            flat and F7, the sixth and ninth of each chord adding the colour. The second half \
            goes up to E flat, the IV chord, with B flat 9 as its dominant, and back through a \
            diminished chord, C7 and F7.",
};

// Source: https://github.com/soprodecarnaval/cadern.in (cadernin_2026 flute book, page 51 of the
// PDF, "Zé Pereira", marchinha no. 19, "Domínio público", in B flat, 2/4, with chords B flat and
// F7: A with a segno and first and second endings, B, C with first and second endings, D.S.),
// read from the page rendered as an image at 400 dpi. No second
// written source was found. The eighth-note triplets of bars 4 and 8 (the book counts the
// pickup as bar 1) are written as an eighth and two sixteenths. The 2/4 bars are paired.
// The score's D.S. has no Fine and no coda (the book loops it, as the street band does): the
// form plays the whole song twice, repeats included, and stops on C's first ending, the held
// B flat, rather than on the second ending's upbeat F.
// PD: traditional. The tune is the French song "Les pompiers de Nanterre" (Antonin Louis,
// 1868; he died in 1915, data.bnf.fr), sung in Rio's carnival as Zé Pereira since the actor
// Francisco Correia Vasques's show "Zé Pereira carnavalesco" (1869).
pub(super) const ZE_PEREIRA: Song = Song {
    name: "Zé Pereira",
    origin: "Traditional, Brazil (Rio de Janeiro carnival)",
    feel: "Marchinha",
    genre: Genre::Latin,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Offbeat,
    bass: Bass::RootFifth,
    rhythm: "Marchinha",
    band: BRASS_BAND,
    style: "Marchinha",
    parts: &[
        Part::new("Pickup", "-:15 F4:1", "I"),
        Part::new(
            "A",
            "Bb4:3 F4:1 Bb4:3 F4:1 Bb4:3 F4:1 D4:4 | F4:2 F4:1 F4 Eb4:2 F4:1 Eb4 D4:4 -:3 F4:1 | \
             Bb4:3 F4:1 Bb4:3 F4:1 Bb4:3 F4:1 D4:4",
            "I | V7 I | I",
        ),
        Part::new(
            "A end 1",
            "D5:2 D5:1 D5 C5:2 D5:1 C5 Bb4:4 -:3 F4:1",
            "V7 I",
        ),
        Part::new("A end 2", "D5:2 D5:1 D5 C5:2 D5:1 C5 Bb4:4 -:4", "V7 I"),
        Part::new(
            "B",
            "F4:2 Bb4 Bb4 Bb4 Bb4:4 F4 | F4:2 D5 D5 D5 D5:4 Bb4 | Bb4:2 F5 F5 F5 F5 D5 Bb4 D5 | \
             Eb5:4 D5 C5:8 | F4:2 Bb4 Bb4 Bb4 Bb4:4 F4 | F4:2 D5 D5 D5 D5:4 - | \
             Bb4:2 F5 F5 F5 F5 D5 Bb4 D5 | Eb5:2 C5 A4 F4 Bb4:8",
            "I | I | I | I V7 | I | I | I | V7 I",
        ),
        Part::new(
            "C",
            "D5:2 Bb4:1 D5 F5:4 D5:2 Bb4:1 D5 F5:4 | C5:2 F5 C5 F5 D5:3 C5:1 Bb4:4 | \
             D5:2 Bb4:1 D5 F5:4 D5:2 Bb4:1 D5 F5:4",
            "I | V7 I | I",
        ),
        Part::new("C end 1", "F4:2 F4 G4 A4 Bb4:8", "V7 I"),
        Part::new("C end 2", "F4:2 F4 G4 A4 Bb4:6 -:1 F4:1", "V7 I"),
    ],
    form: &[
        "Pickup", "A", "A end 1", "A", "A end 2", "B", "C", "C end 1", "C", "C end 2", "A",
        "A end 1", "A", "A end 2", "B", "C", "C end 1",
    ],
    drums_from: 1,
    about: "The song of Zé Pereira, the bass drummer who leads Rio's street carnival, sung to the \
            tune of a French music-hall song of 1868. Two chords carry it all, B flat and F7: \
            the first strain marches in dotted rhythms, the second climbs the B flat chord, \
            F, B flat, D, F, and the third bounces between D and F before F7 brings it home. \
            The march is in 2/4, two bars to a bar here.",
};

// Source: https://github.com/OpenScore/Lieder, scores/Gonzaga,_Chiquinha/_/Feijoada_do_Brasil/
// lc6611543.mxl (OpenScore Lieder's transcription of the first edition, Vieira Machado & Cia.,
// Rio de Janeiro, 1909, IMSLP: voice and piano, in C, 2/4, "Moderato", D.C. at the end).
// The tune is the voice; in the ten bars of the piano's introduction, where the voice rests, it
// is the top of the right hand (in bars 9 and 10 the repeated G of its chords). The chords are
// read from the piano. The voice's first bar, a run up from E with the piano silent, is given
// G, the chord the introduction ends on. The 2/4 bars are paired. The fermata over the
// introduction's last rest (bar 10) is kept.
// The D.C. has no Fine: the form plays the song twice and ends on the last bar.
// PD: published in 1909; Chiquinha Gonzaga died in 1935.
pub(super) const FEIJOADA_DO_BRASIL: Song = Song {
    name: "Feijoada do Brasil",
    origin: "Chiquinha Gonzaga, 1909",
    feel: "Canção",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 84.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Maxixe",
    band: Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS),
    style: "Maxixe",
    parts: &[
        Part::new(
            "Intro",
            "G5:2 E5 G5 E5 G5:1 F5:2 D5:1 B4 G4:2 F4:1 | E4:2 G4 C5 A4 D5:6 -:2 | \
             G5 E5 G5 E5 G5:1 F5:2 D5:1 B4 G4:2 F4:1 | E4:2 G4 C5 A4 D5:6 -:2 | \
             G4:1 G4:2 G4:1 G4 G4:2 G4:1 G4 G4:2 G4:1 G4:2 -",
            "I V7 | I V7 | I V7 | I V7 | V7 I i V",
        )
        .with_tempo(&[fermata(78, 2)]),
        Part::new(
            "Verse",
            "-:2 E4:1 F4 G4 A4 B4 C5 D5:4 D5 | -:1 A4:2 B4:1 D5 C5 A4 B4 G4:8 | \
             -:1 G4:2 G4:1 C5:2 A4:1 A4 F4:8 | -:1 D4:2 D4:1 B4:2 G4 E4:8 | \
             -:2 E4:1 F4 G4 A4 B4 C5 D5:4 D5 | -:1 A4:2 B4:1 D5 C5 A4 B4 G4:8 | \
             -:1 G4:2 G4:1 C5:2 C5:1 C5 B4:8 | -:1 D4:2 D4:1 A4:2 B4 G4:8 | \
             _:2 G4 F5 C#5 E5:4 D5 | -:2 G4 E5 B4 D5:4 C5 | \
             -:2 G4 C5 A4 B4:1 B4:2 F4:1 G4:2 A4 | B4 -:1 D4 F4:2 B4 G4:8",
            "V II7 | V7 I | I V7 | V7 I | I II7 | V7 I | I V | II7 V | V V7 | V7 I | I V7 | V7 I",
        ),
        Part::new(
            "Refrain",
            "-:2 A4:1 A4 A4:2 A4:1 A4 D5:8 | -:1 G4:2 G4:1 G4:2 G4 C5:8 | \
             _:2 -:1 A4 E5:2 D5 G4:1 G4:2 G4:1 D5:2 C5 | F4:1 - F4 F4 B4:2 B4 G4:8 | \
             _:6 -:4 A4:1 A4 A4:2 A4:1 A4 | D5:8 -:1 G4:2 G4:1 G4:2 G4 | \
             C5:8 _:2 -:1 A4 E5:2 D5 | G4:1 G4:2 G4:1 D5:2 C5 F4:1 - G4 G4 B4:2 E5 | C5:8 -",
            "VI7 II7 | V7 I | ii ii° I I | V7 I | I VI7 | II7 V7 | I I ii ii° | I V7 | I",
        ),
    ],
    form: &["Intro", "Verse", "Refrain", "Intro", "Verse", "Refrain"],
    drums_from: 1,
    about: "A song by Chiquinha Gonzaga, published in 1909, in C major over the habanera rhythm \
            of the piano's left hand. The verse moves between C and G7 and twice puts D7 over a \
            C in the bass on its way to G; the refrain opens on A7 and D7, a chain of secondary \
            dominants, and turns D minor into D diminished, ii to ii°, before the cadence. Each \
            bar here holds two bars of 2/4.",
};

// Source: https://github.com/OpenScore/Lieder, scores/Gonzaga,_Chiquinha/_/A_mulatinha/
// lc6608395.mxl (OpenScore Lieder's transcription of the first edition, "Canção brasileira",
// Manoel Antonio Gomes Guimarães, Rio de Janeiro, ca. 1901, IMSLP: voice and piano, in C, 2/4,
// with three stanzas, the second and third printed as text, "Mulatinha, etc.").
// The tune is the voice; in the eight bars of the piano's introduction, and in bars 38 and 39
// where the voice rests, it is the top of the right hand. In bar 9 the piano's high C is kept
// and its two chords above C6 are left to the band, before the voice's first note. The chords
// are read from the piano. The refrain's last bar (41) is G7 for its first beat and C for its
// second. The fermatas of bars 17, 26 and 34 are kept.
// The introduction has nine bars of 2/4: a bar of rest comes first, so that the voice's entry
// falls on a bar line. The 2/4 bars are paired.
// PD: published about 1901; Chiquinha Gonzaga died in 1935 and the poet, José do Patrocínio
// Filho, in 1929.
pub(super) const A_MULATINHA: Song = Song {
    name: "A mulatinha",
    origin: "Chiquinha Gonzaga, 1901",
    feel: "Canção brasileira",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Maxixe",
    band: Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS),
    style: "Maxixe",
    parts: &[
        Part::new(
            "Intro",
            "-:9 C5:2 B4:1 D5 C5 A4 C5 | G4:2 G4 B4 B4 G5 G5 B5 B5 | E4 E4 A4 A4 E5 E5 A5 A5 | \
             G5:1 F5:2 A4:1 E5:2 D5 D5:1 C5:2 G4:1 D5:2 C5 | \
             B4:1 C5 D5 E5 F5 G5 A5 B5 C6:2 -:4 E4:2",
            "I | V7 | I | ii II7 I I | V7 V7 V7 V7 I V I I",
        ),
        Part::new(
            "Verse",
            "F4:1 F4:2 D4:1 E4:2 F4 G4 G4 - G4 | A4:1 A4:2 E5:1 D5:2 B4 A4 G4 - E4 | \
             F4:1 F4:2 D4:1 E4:2 F4 G4 G4 - G4:1 G4 | B4:2 B4:1 - A4:2 A4 D5:4 _:2 -:1 E4 | \
             F4 F4:2 D4:1 E4:2 F4 G4 G4 - G4 | A4:1 A4:2 E5:1 D5:2 B4 A4 G4 - G4 | \
             C5:1 C5:2 G4:1 A4:2 Bb4 C5:4 -:1 B4:2 A4:1 | G4:2 G4 G4 G4 D5:6 -:2",
            "V7 I | II7 V7 I I | V7 I | V II7 V7 V7 | V7 I | II7 V7 I I | I7 IV | V7",
        )
        .with_tempo(&[fermata(62, 1)]),
        Part::new(
            "Refrain",
            "C5:3 B4:1 D5 C5 A4 C5 G4:2 G4 -:4 | -:1 B4:2 A4:1 C5 B4 A#4 B4 E4:2 E4 -:4 | \
             -:1 A4:3 -:2 A4 B3:4 G4 | -:1 G4:2 G4:1 E5:2 D5 C5:4 G4:2 - | \
             C5:3 B4:1 D5 C5 A4 C5 G4:2 G4 -:4 | -:3 B4:1 C5 C5:2 B4:1 E4:4 G4 | \
             -:1 C4 D4 E4 F4 G4 A4 B4 B4:2 A4 D5:4",
            "I V7 | V7 I | I V7 | V7 I | I V7 | V7 I | I ii",
        )
        .with_tempo(&[fermata(0, 3), fermata(5, 1), fermata(64, 3), fermata(68, 1)]),
        Part::new(
            "Refrain end",
            "-:1 B4 B4 B4 B4 G4:2 E5:1 D5:4 C5:2 -",
            "V7 V7 V7 I",
        ),
        Part::new(
            "Refrain to verse",
            "-:1 B4 B4 B4 B4 G4:2 E5:1 D5:4 C5:2 E4",
            "V7 V7 V7 I",
        ),
    ],
    form: &[
        "Intro",
        "Verse",
        "Refrain",
        "Refrain to verse",
        "Verse",
        "Refrain",
        "Refrain to verse",
        "Verse",
        "Refrain",
        "Refrain end",
    ],
    drums_from: 1,
    about: "A Brazilian song of 1901 by Chiquinha Gonzaga, sung three times through. It is in C \
            major: the verse rocks between G7 and C, reaches G through its own dominant, D7, \
            and near its end turns C into C7 to land on F, I7 to IV; the refrain, \"Mulatinha \
            sem tardança\", keeps to C and G7, touching D minor before its last G7 to C. Each \
            bar here holds two bars of 2/4.",
};

// Source: https://github.com/OpenScore/Lieder, scores/Gonzaga,_Chiquinha/_/Roda_Yôyô/
// lc6611820.mxl (OpenScore Lieder's transcription of the first edition, "Canção Brazileira",
// Manoel Antonio Gomes Guimarães, Rio de Janeiro, plate 4575, ca. 1900,
// words by Ernesto de Souza, dedicated to Sophia Camps; IMSLP: voice and piano, in F, 2/4,
// one stanza). The tune is the voice, as written (the piano's right hand doubles it an octave
// higher). In the fourteen bars of the piano's introduction it is the top of the right hand;
// in bars 39 (second beat) to 43 (first beat), where the voice rests, it is the right hand's
// top line an octave lower, in the voice's octave. The chords are read from the piano. In
// bar 37 the C major chord has F in the bass for its first sixteenth; it is written V. The
// fermatas of bars 23, 31, 39 and 47 are kept.
// The 2/4 bars are paired; the voice's entry (bar 15) begins a paired bar, and a bar of rest
// completes the last one.
// PD: published about 1900; Chiquinha Gonzaga died in 1935 and the poet, Ernesto de Souza, in
// 1928.
pub(super) const RODA_YOYO: Song = Song {
    name: "Roda Yôyô",
    origin: "Chiquinha Gonzaga, c. 1900",
    feel: "Canção brasileira",
    genre: Genre::Latin,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Maxixe",
    band: Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS),
    style: "Maxixe",
    parts: &[
        Part::new(
            "Intro",
            "-:2 C4:2 C4:2 C4:1 -:1 -:1 A4:2 G4:1 F4:1 E4:2 D4:1 | \
             C4:2 C4:2 C4:2 C4:2 -:1 E4:2 D4:1 C4:1 Bb3:2 E3:1 | \
             F3:2 C4:2 C4:2 C4:1 -:1 -:1 A4:2 G4:1 F4:1 E4:2 D4:1 | \
             C4:2 C4:2 C4:2 C4:2 -:1 E4:2 D4:1 C4:1 Bb3:2 E3:1 | \
             F3:2 C4:2 D4:2 D4:1 -:1 C4:1 C4:2 C4:1 C4:2 C4:1 -:1 | \
             F4:2 C4:2 D4:2 D4:1 -:1 C4:1 C4:2 C4:1 C4:2 Bb3:1 -:1 | \
             F4:1 F4:2 F4:1 F4:2 C4:2 F4:1 F4:2 F4:1 F4:2 C4:2",
            "I | V7 | I | V7 | I I6 V7 V7 | I I6 V7 V7 | I I I V7 I I I V7",
        ),
        Part::new(
            "Verse",
            "-:3 C4:1 D4:2 -:1 C4:1 F4:2 -:3 F4:1 E4:2 | \
             D4:1 D4:2 -:3 D5:2 C5:1 Bb4:1 G4:1 E4:1 D4:2 Db4:2 | \
             C4:2 C4:2 -:2 A3:2 C4:1 C4:2 C4:1 C4:1 Bb3:2 A3:1 | \
             C4:2 C4:1 -:6 C4:2 D4:1 E4:1 F4:1 G4:1 A4:1 | \
             Bb4:2 A4:1 -:2 C4:1 D4:1 C4:1 F4:2 -:3 F4:2 E4:1 | \
             D4:2 D4:1 -:2 D5:2 D5:1 C5:1 Bb4:1 G4:1 E4:1 D4:2 Db4:2 | \
             C4:2 C4:2 -:1 F4:2 F4:1 C5:4 -:1 C5:1 C5:1 C5:1 | \
             B4:2 B4:2 -:5 F4:2 G4:1 A4:1 C5:1 A4:1 F4:1",
            "I I I V7 I I I I | ii V7 | I | V7 | iv I I V7 I I I I | IV V7 | I I7 | #iv° iv I I",
        )
        .with_tempo(&[fermata(64, 2)]),
        Part::new(
            "Refrain",
            "G4:1 F4:1 -:1 C4:1 C4:1 D4:2 C4:1 A4:1 A4:1 -:1 C4:1 C4:1 D4:2 C4:1 | \
             Bb4:1 Bb4:1 -:2 Bb4:1 A4:2 G4:1 D5:2 -:1 C5:1 Bb4:2 E4:2 | \
             D4:1 C4:1 -:1 C4:1 C4:1 D4:2 C4:1 A4:1 A4:1 -:1 C4:1 C4:1 D4:2 C4:1 | \
             G4:1 G4:2 -:1 G4:1 G4:2 G4:1 G4:2 G4:2 G4:2 G4:2 | \
             G4:2 G4:1 -:1 C4:1 D4:2 C4:1 A4:1 A4:2 C4:1 C4:1 D4:2 C4:1 | \
             Bb4:1 Bb4:2 -:1 Bb4:1 A4:2 G4:1 D5:3 C5:1 Bb4:2 E4:2 | \
             D4:1 C4:1 -:2 F4:1 F4:2 F4:1 F4:2 -:2 F4:1 F4:2 F4:1 | \
             F4:2 -:2 C5:1 A4:2 F4:1 D4:1 -:1 E4:1 F4:1 G4:2 A4:2 | \
             G4:4 F4:2 -:2 -:8",
            "V7 I I I I I I I | V7 | I | V II7 | V7 I I I | V7 | I I7 IV iv | I I IV V7 | V7 I I I",
        )
        .with_tempo(&[fermata(0, 1), fermata(64, 2), fermata(128, 4)]),
    ],
    form: &["Intro", "Verse", "Refrain"],
    drums_from: 1,
    about: "A Brazilian song by Chiquinha Gonzaga, published about 1900, in F major over the \
            habanera rhythm of the piano's left hand. The verse colours F with B flat minor, iv, \
            a passing D flat in the tune, F7 over E flat and a B diminished chord before its \
            cadence; the refrain, \"Que eu sou de fogo, que eu sou de brasa\", goes between F \
            and C7, reaches C through G7, and ends with B flat and B flat minor before the last \
            cadence. Each bar here holds two bars of 2/4.",
};

// Source: https://github.com/OpenScore/Lieder, scores/Gonzaga,_Chiquinha/_/Dona_Adelaide/
// lc6610193.mxl (OpenScore Lieder's transcription of the first edition, "Cançoneta", Manoel
// Antônio Guimarães, Rio de Janeiro, plate 4651, 1900 per IMSLP; words by
// José do Patrocínio Filho; IMSLP: voice and piano, 2/4, the verse in D major and the refrain
// in A major). The tune is the voice, as written; in the two bars of introduction it is the top
// of the piano's right hand until the voice enters. The chords are read from the piano: the
// verse's bass falls by semitones from B to F sharp, under E minor, A sharp diminished, D,
// E7 and A7, and bar 7 is G sharp diminished.
// The score writes out both stanzas: bars 1 to 18, then bars 19 to 36, the same introduction
// and harmony with small changes of rhythm for the second stanza's words ("Intro 2",
// "Verse 2", "Refrain 2"). In bar 35 the voice has three sixteenth-note triplets in the last
// eighth (G sharp, F sharp, E, after a rest under a fermata); they are written as two
// sixteenths, G sharp and E, leaving out the passing F sharp. Bar 35's E sharp is written F.
// The fermatas of bars 2, 10, 18, 20, 28, 35 and 36 are kept.
// PD: published 1900; Chiquinha Gonzaga died in 1935 and the poet, José do Patrocínio Filho,
// in 1929.
pub(super) const DONA_ADELAIDE: Song = Song {
    name: "Dona Adelaide",
    origin: "Chiquinha Gonzaga, 1900",
    feel: "Cançoneta",
    genre: Genre::Latin,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Maxixe",
    band: Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS),
    style: "Maxixe",
    parts: &[
        Part::new("Intro", "A4:3 A5:1 A5:2 -:2 A4:2 -:1 D4:1 D4:2 C#4:2", "V7")
            .with_tempo(&[fermata(8, 2)]),
        Part::new(
            "Verse",
            "B3:1 B3:1 A#3:1 B3:1 B4:2 A#4:2 A4:2 A4:2 -:1 D4:1 E4:1 F#4:1 | \
             F#4:1 E4:1 D#4:1 E4:1 F#4:2 E4:2 A3:2 A3:2 -:1 D4:1 C#4:1 D4:1 | \
             F4:2 F4:1 D4:1 D4:1 D4:1 C#4:1 D4:1 B4:2 A4:2 -:1 D4:1 E4:1 F#4:1 | \
             A4:1 G4:1 G4:1 B3:1 B3:2 F#4:2 E4:4 D4:2 -:2",
            "ii #v°7 I I | II7 V7 I I | #iv°7 I | ii V V7 I",
        )
        .with_tempo(&[fermata(56, 4)]),
        Part::new(
            "Refrain",
            "F#4:2 B3:1 C#4:1 D4:1 C#4:1 D4:1 B3:1 F#4:2 F4:2 E4:1 E4:1 -:1 E4:1 | \
             E4:1 -:1 E4:1 F#4:1 G#4:1 A4:1 B4:1 B4:1 B4:2 A4:2 E4:1 E4:1 -:1 E4:1 | \
             F#4:1 E4:1 E4:1 B3:1 C#4:1 D4:1 E4:1 B3:1 F#4:2 F4:2 E4:1 E4:1 -:1 E4:1 | \
             F#4:1 F#4:1 E4:1 F#4:1 G#4:1 G#4:1 F#4:1 E4:1 B4:4 A4:2 -:2",
            "V7 I | V7 I | V7 I | IV V V7 I",
        )
        .in_key(A, Major)
        .with_tempo(&[fermata(56, 4)]),
        Part::new("Intro 2", "A4:3 A5:1 A5:2 -:2 A4:2 -:2 D4:2 C#4:2", "V7")
            .with_tempo(&[fermata(8, 2)]),
        Part::new(
            "Verse 2",
            "B3:2 A#3:1 B3:1 B4:1 B4:1 B4:1 A#4:1 A4:2 A4:2 -:1 D4:1 E4:1 F#4:1 | \
             F#4:1 E4:1 -:1 E4:1 F#4:1 F#4:1 E4:1 E4:1 A3:2 A3:2 -:1 D4:1 C#4:1 D4:1 | \
             F4:2 F4:1 D4:1 D4:1 D4:1 C#4:1 D4:1 B4:2 A4:2 -:1 D4:1 E4:1 F#4:1 | \
             A4:1 G4:1 G4:1 B3:1 B3:3 F#4:1 E4:4 D4:2 -:2",
            "ii #v°7 I I | II7 V7 I I | #iv°7 I | ii V V7 I",
        )
        .with_tempo(&[fermata(56, 4)]),
        Part::new(
            "Refrain 2",
            "D4:1 -:1 B3:1 C#4:1 D4:1 C#4:1 D4:1 B3:1 F#4:2 F4:2 E4:1 E4:1 -:1 E4:1 | \
             E4:1 -:1 E4:1 F#4:1 G#4:1 A4:1 B4:1 B4:1 B4:2 A4:2 E4:2 -:1 E4:1 | \
             F#4:1 E4:1 E4:1 B3:1 C#4:1 D4:1 E4:1 B3:1 F#4:2 F4:2 E4:1 -:2 E4:1 | \
             F#4:1 F#4:1 F4:1 F#4:1 G#4:1 -:1 G#4:1 E4:1 B4:4 A4:2 -:2",
            "V7 I | V7 I | V7 I | IV V V7 I",
        )
        .in_key(A, Major)
        .with_tempo(&[fermata(53, 1), fermata(56, 4)]),
    ],
    form: &[
        "Intro",
        "Verse",
        "Refrain",
        "Intro 2",
        "Verse 2",
        "Refrain 2",
    ],
    drums_from: 1,
    about: "A cançoneta of 1900 by Chiquinha Gonzaga. The verse, in D major, walks its bass down \
            by semitones from B to F sharp under E minor, A sharp diminished, D, E7 and A7, then \
            G sharp diminished, before a ii-V-I; the refrain moves to A major and rocks between \
            E7 and A, with an F natural slipping down to E in the tune. Both stanzas are sung, \
            each from D major to A major. Each bar here holds two bars of 2/4.",
};

// Under copyright (Jobim d. 1994, de Moraes d. 1980); added at the owner's request.
// Source: two lead sheets in F, transposed up a minor sixth to D flat, the key of the
// Getz/Gilberto recording (recorded 1963, released 1964). First chorus, with the rhythm of the Portuguese words:
// Ulf Bro's ABC lead sheet, "The Girl From Ipanema (Garota De Ipanema)", 2002, from John
// Chambers' tune finder, in the abc2book collection:
// https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201280611.abc
// (the same sheet is X:78 in abcresources/jc/abc_tune_jc_201254999.abc there). Second chorus,
// with the rhythm of the English words and the record's ending tag: the Wikifonia lead sheet,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/AntonioCarlosJobimViniciusdeMoraesTheGirlFromIpanema.abc
// The two sheets give the same pitches and the same chord roots bar for bar. The second bar
// of the bridge is F G flat F in both Wikifonia and the tune's own sequence; Ulf's ABC writes
// F flat there, which is a slip. Key and tempo of the recording: jazzguitar.be ("The Girl
// From Ipanema – Original Guitar Chords") and Hooktheory give D flat; the tempo, 128, is
// Hooktheory's figure, an estimate (karaoke-version.com says about 130).
pub(super) const GAROTA_DE_IPANEMA: Song = Song {
    name: "Garota de Ipanema",
    origin: "Antônio Carlos Jobim and Vinícius de Moraes, 1962",
    feel: "Bossa nova",
    genre: Genre::Latin,
    tonic: CSharp,
    mode: Major,
    time: COMMON,
    // An estimate of the Getz/Gilberto recording's tempo.
    tempo: 128.0,
    comping: Comping::Bossa,
    bass: Bass::Bossa,
    rhythm: "Bossa nova",
    band: BOSSA,
    style: "Bossa nova",
    parts: &[
        // First chorus: the rhythm of the Portuguese words, "Olha que coisa mais linda",
        // as João Gilberto sings them on the record.
        Part::new(
            "A1",
            "Eb5:4 Eb5:2 C5:2 C5:4 C5:2 Bb4:2 | Eb5:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 Eb5:2 | \
             _:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 Eb5:2 | _:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 Db5:2 | \
             _:4 Db5:2 Bb4:2 Bb4:2 Bb4:2 Ab4:2 C5:2 | _:4 C5:2 Ab4:2 Ab4:2 Ab4:2 Gb4:2 Ab4:2 | \
             _:16 | -:16",
            "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | bII7",
        ),
        Part::new(
            "A2",
            "Eb5:4 Eb5:2 C5:2 C5:4 C5:2 Bb4:2 | Eb5:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 Eb5:2 | \
             _:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 Eb5:2 | _:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 Db5:2 | \
             _:4 Db5:2 Bb4:2 Bb4:2 Bb4:2 Ab4:2 C5:2 | _:4 C5:2 Ab4:2 Ab4:2 Ab4:2 Gb4:2 Ab4:2 | \
             _:16 | -:16",
            "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | Imaj7",
        ),
        // The bridge goes up a half step, to D major, for eight bars. The quarter-note
        // triplets are written 3 + 3 + 2 steps. The F at the end is the anticipation of the
        // next bar, which the score ties over; it is struck again on the bar line, where
        // the key goes back to D flat.
        Part::new(
            "Bridge 1",
            "C#5:16 | _:3 D5:3 C#5:2 B4:3 C#5:3 B4:2 | A4:6 B4:2 _:8 | _:12 -:2 E5:2 | \
             _:16 | _:3 F5:3 E5:2 D5:3 E5:3 D5:2 | C5:6 D5:2 _:8 | _:12 -:2 F5:2",
            "Imaj7 | Imaj7 | IV7 | IV7 | i7 | i7 | bVI7 | bVI7",
        )
        .in_key(D, Major),
        Part::new(
            "Bridge 2",
            "F5:16 | _:3 Gb5:3 F5:2 Eb5:3 F5:3 Eb5:2 | Db5:6 Eb5:2 _:8 | _:8 -:3 F5:3 Gb5:2 | \
             Ab5:3 Ab4:3 Bb4:2 C5:3 Db5:3 Eb5:2 | E5:6 F5:6 -:4 | \
             Gb5:3 Gb4:3 Ab4:2 Bb4:3 C5:3 Db5:2 | D5:6 Eb5:6 -:4",
            "ii7 | ii7 | bVII7 | bVII7 | iii7 | VI7 | ii7 | V7",
        ),
        // The last A of the first chorus. Ulf's sheet ends on its seventh bar; the D7 of the
        // eighth, leading into the second chorus, is the first ending of the Wikifonia sheet.
        Part::new(
            "A3",
            "Eb5:6 C5:2 C5:2 C5:2 Bb4:2 Eb5:2 | _:4 Eb5:2 C5:2 _:2 C5:2 Bb4:2 Eb5:2 | \
             _:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 Eb5:2 | _:4 Eb5:2 C5:2 C5:2 C5:2 Bb4:2 F5:2 | \
             _:4 F5:2 Db5:2 Db5:2 Db5:2 Bb4:2 Ab5:2 | _:4 Ab5:2 C5:2 C5:3 C5:3 Bb4:2 | \
             C5:16 | _:4 -:12",
            "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | bII7",
        ),
        // Second chorus: the rhythm of the English words, "Tall and tan and young and
        // lovely", as Astrud Gilberto sings them on the record.
        Part::new(
            "A1 English",
            "Eb5:6 C5:2 C5:4 Bb4:2 Eb5:2 | _:4 C5:2 C5:2 _:2 C5:2 Bb4:2 Eb5:2 | \
             _:4 C5:4 C5:4 Bb4:2 Eb5:2 | _:2 Eb5:2 C5:2 C5:2 _:2 C5:2 Bb4:2 Db5:2 | \
             _:2 Bb4:4 Bb4:2 _:2 Bb4:2 Ab4:2 C5:2 | _:2 Ab4:4 Ab4:2 _:2 Ab4:2 Gb4:4 | \
             -:4 Ab4:12 | _:8 -:8",
            "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | bII9",
        ),
        Part::new(
            "A2 English",
            "Eb5:6 C5:2 C5:4 Bb4:2 Eb5:2 | _:4 C5:2 C5:2 _:2 C5:2 Bb4:4 | \
             Eb5:4 C5:4 C5:4 Bb4:2 Eb5:2 | _:4 C5:2 C5:2 _:2 C5:2 Bb4:2 Db5:2 | \
             _:2 Bb4:4 Bb4:2 Bb4:2 Bb4:2 Ab4:2 C5:2 | _:2 Ab4:4 Ab4:2 _:2 Ab4:2 Gb4:4 | \
             -:4 Ab4:12 | _:8 -:8",
            "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | Imaj7",
        ),
        Part::new(
            "Bridge 1 English",
            "C#5:16 | _:3 D5:3 C#5:2 B4:3 C#5:3 B4:2 | A4:6 B4:2 _:8 | _:12 -:2 E5:2 | \
             _:16 | _:3 F5:3 E5:2 D5:3 E5:3 D5:2 | C5:6 D5:2 _:8 | _:12 -:2 F5:2",
            "Imaj7 | Imaj7 | IV9 | IV9 | i7 | i7 | bVI9 | bVI9",
        )
        .in_key(D, Major),
        Part::new(
            "Bridge 2 English",
            "F5:16 | _:3 Gb5:3 F5:2 Eb5:3 F5:3 Eb5:2 | Db5:6 Eb5:2 _:8 | _:8 -:3 F5:3 Gb5:2 | \
             Ab5:3 Ab4:3 Bb4:2 C5:3 Db5:3 Eb5:2 | E5:12 F5:4 | \
             Gb5:3 Gb4:3 Ab4:2 Bb4:3 C5:3 Db5:2 | D5:12 -:4",
            "ii7 | ii7 | bVII9 | bVII9 | iii7 | VI7b9 | ii7 | V7b9",
        ),
        // The last A, and the record's ending: "but she doesn't see", then twice more,
        // "she just doesn't see", "no, she doesn't see", rocking between D flat major 7
        // and D7, and a held last note.
        Part::new(
            "A3 end",
            "Eb5:6 C5:2 C5:4 Bb4:2 Eb5:2 | _:4 C5:2 C5:2 _:2 C5:2 Bb4:2 Eb5:2 | \
             _:4 C5:4 C5:4 Bb4:2 Eb5:2 | _:2 Eb5:2 C5:2 C5:2 _:2 C5:2 Bb4:2 F5:2 | \
             _:6 Db5:2 Db5:2 Db5:2 Bb4:2 Ab5:2 | Ab5:6 C5:2 C5:3 C5:3 Bb4:2 | C5:16 | \
             -:6 C5:2 C5:3 C5:3 Bb4:2 | C5:16 | -:6 C5:2 C5:3 C5:3 Bb4:2 | C5:16 | _:16",
            "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | Imaj7 bII7 bII7 bII7 | Imaj7 | \
             bII7 | Imaj7 bII7 | Imaj7",
        )
        .with_tempo(&[fermata(176, 16)]),
    ],
    form: &[
        "A1",
        "A2",
        "Bridge 1",
        "Bridge 2",
        "A3",
        "A1 English",
        "A2 English",
        "Bridge 1 English",
        "Bridge 2 English",
        "A3 end",
    ],
    drums_from: 0,
    about: "The second chord is the surprise: E flat 7 (II7, the dominant of the dominant) where \
            E flat minor 7 (ii7) would be expected, and the ii7 that follows goes home through \
            D7 (bII7), standing in for A flat 7. The bridge jumps up a half step to D major (I, \
            then G7 as IV7), turns D minor (i7), and B flat 7, D major's bVI7 but D flat's VI7, \
            leads to E flat minor and a chain of chords home: B7 (bVII7), F minor 7 (iii7), \
            B flat 7 (VI7), E flat minor 7 (ii7), A flat 7 (V7).",
};
