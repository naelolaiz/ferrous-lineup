//! Folklore: the dances and songs of South America.

use super::*;

/// The slow theme rests on A minor and its relative major, C.
const CONDOR_THEME_CHORDS: &str = "i | bIII | bIII | bIII | i | i";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/DanielA.abc
// (the Wikifonia lead sheet of the 1933 E. B. Marks piano edition, in D minor,
// with Robles's Andante, D.C. al Coda, and the Allegro fuga; cross-checked
// against John Chambers's ABC, https://trillian.mit.edu/~jc/music/abc/song/El_Condor_Pasa-Am-34-0.abc)
pub(super) const EL_CONDOR_PASA: Song = Song {
    name: "El cóndor pasa",
    origin: "Daniel Alomía Robles, 1913",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Huayno",
    band: ANDEAN,
    style: "Huayno",
    parts: &[
        Part::new("Pickup", "-:14 E4:2", "i"),
        Part::new(
            "Theme",
            "A4:2 G#4 A4 B4 C5 B4 C5 D5 | E5:12 G5:2 G5 | E5:12 A5:2 G5 | E5:12 D5:2 C5 | \
             A4:12 C5:4 | A4:12 -:2 E4:2",
            CONDOR_THEME_CHORDS,
        ),
        // The second time, the pickup leaps up an octave into the second strain.
        Part::new(
            "Theme into B",
            "A4:2 G#4 A4 B4 C5 B4 C5 D5 | E5:12 G5:2 G5 | E5:12 A5:2 G5 | E5:12 D5:2 C5 | \
             A4:12 C5:4 | A4:12 -:2 E5:2",
            CONDOR_THEME_CHORDS,
        ),
        Part::new(
            "B",
            "A5:6 G#5:2 A5:3 G#5:1 A5:3 G#5:1 | A5:12 C6:2 A5 | G5:12 A5:2 G5 | E5:12 -:2 E5 | \
             A5:6 G#5:2 A5:3 G#5:1 A5:3 G#5:1 | A5:12 C6:2 A5 | G5:12 A5:2 G5 | E5:12 D5:2 C5 | \
             A4:12 C5:4 | A4:12 E5:2 D5:1 C5 | A4:12 -:4",
            "bVI | bVI | bIII | bIII | bVI | bVI | bIII | bIII | i | i | i",
        )
        // The Andante ends under a fermata.
        .with_tempo(&[fermata(160, 12)]),
        Part::new(
            "Fuga A",
            "E5:1 A5 E5 A5 E5:2 D5 C5:1 A4:2 A4:5 | A4:1 B4 C5 D5 E5:2 G5 E5:8 | \
             E5:1 A5 E5 A5 E5:2 D5 C5:1 A4:2 G4:1 A4:4 | A4:1 B4 C5 D5 E5 D5 C5 B4 A4:8",
            "i | bIII | i | i",
        )
        // "Allegro": the fuga is half as fast again as the Andante.
        .with_tempo(&[tempo(0, 120)]),
        Part::new(
            "Fuga B",
            "E5:1 G5 E5 G5 D5:2 D5:1 G5 E5:8 | E5:1 G5 E5 G5 D5:1 G5:3 E5:4 E5 | \
             E5:1 A5 E5 A5 E5:2 D5 C5:1 A4:2 G4:1 A4:4 | A4:1 B4 C5 D5 E5 D5 C5 B4 A4:8",
            "bVII | bVII i | i | i",
        ),
        Part::new(
            "Fuga C",
            "C5:2 G4:1 C5 E5 G5 E5 D5 C5:8 | C5:2 G4:1 C5 E5 G5 E5 D5 C5:8 | \
             A5:1 A5:2 G5:1 A5 C6:2 A5:1 G5:2 E5:1 G5:5 | C5:2 A4:1 C5 E5 D5 C5 B4 A4:8 | \
             A4:3 B4:1 C5 B4 C5 D5 E5 A5 E5 A5 E5 D5 C5 B4 | A4:1 B4 C5 D5 E5 D5 C5 B4 A4:8",
            "bIII | bIII | i bIII | i | i | i",
        ),
        // Robles writes the last flourish as one long bar under fermatas; here it
        // is two bars, with the final note held.
        Part::new(
            "Fuga end",
            "A5:4 G5:1 A5 G5 E5 G5 E5 D5 E5 D5 C5 D5 C5 | A4:2 G4 A4:12",
            "i | i",
        )
        // The fermata on the flourish's first note.
        .with_tempo(&[fermata(0, 4)]),
    ],
    form: &[
        "Pickup",
        "Theme",
        "Theme into B",
        "B",
        "Fuga A",
        "Fuga A",
        "Fuga B",
        "Fuga B",
        "Fuga C",
        "Fuga C",
        "Fuga end",
    ],
    drums_from: 4,
    about: "From Robles's zarzuela of the same name, Lima, 1913. The slow theme rests on A \
            minor and its relative major, C; the second strain rises to F and climbs to the \
            high C before it comes home. Then the fuga, the fast dance that ends the piece, \
            runs up and down the same scale in sixteenths, and the drums come in with it. \
            The 1933 piano edition is in D minor; it sits in A minor here.",
};

// Source: https://raw.githubusercontent.com/baobach/abc_search/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmbQQ3V7BEHLkjDvy16ozcfijwzH9FFppgH3LJcv6aThg4.abc
// (a piano arrangement in ABC, "Tiempo de Joropo", one flat with C sharps written in,
// turning to D major for the chorus; the tune is the piano's top voice)
pub(super) const ALMA_LLANERA: Song = Song {
    name: "Alma llanera",
    origin: "Pedro Elías Gutiérrez, 1914",
    feel: "Joropo",
    genre: Genre::Folklore,
    tonic: D,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 168.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Joropo",
    band: LLANERO,
    style: "Joropo",
    parts: &[
        Part::new(
            "Intro",
            "D4:2 E4 F4 G4 A4 Bb4 | A4:4 A5:8 | D4:2 E4 F4 G4 A4 Bb4 | A4:4 A5:8",
            "i | V7 | i | V7",
        ),
        Part::new(
            "Verse",
            "D5:2 F5:4 E5:4 D5:2 | C#5:2 E5:4 D5:4 C#5:2 | D5:4 A4:8 | _:12 | \
             D5:2 F5:4 E5:4 D5:2 | C#5:2 E5:4 D5:4 C#5:2 | D5:4 A4:8 | _:12 | \
             D5:2 F5:4 E5:4 D5:2 | C5:2 D5:4 C5:4 Bb4:2 | A4:12 | G4:2 G4:4 A4:4 Bb4:2 | \
             A4:4 D5:4 A4:4 | A4:2 G4:2 E4:4 F4:4 | D4:12 | -:12",
            "i | V7 | i | V7 | i | V7 | i | V7 | i | bVII | bIII | iiø7 | i | V7 | i | V7",
        ),
        Part::new(
            "Chorus",
            "A4:12 | _:12 | _:8 A4:4 | D5:6 C#5:2 E5 D5 | F#5:6 E5:2 D5 C#5 | E5:6 D5:2 C#5 B4 | C#5:12 | _:12 | -:4 B4:4 B4 | D5:6 D5:2 D5 D5 | C#5:4 A4:8 | _:12 | -:4 B4:4 B4 | D5:6 D5:2 D5 D5",
            "I | V7 | I | ii7 | I | IV | V7 | V7 | IV | IV | V7 | V7 | IV | IV",
        )
        .in_key(D, Major),
        Part::new(
            "Chorus end",
            "C#5:4 A4:8 | _:12 | -:4 B4:4 B4 | D5:6 D5:2 D5 D5 | A4:4 A4 A4:2 A4 | B4:4 G4 G4:2 G4 | A4:4 F#4 -:4 | -:4 G4:4 E4 | A4:12 | _:12 | _:12 | A4:4 G4 E4 | D4:4 -:8",
            "V7 | V7 | IV | IV | V7 | IV | I | ii7 | I | I | V7 | V7 | I",
        )
        .in_key(D, Major),
    ],
    form: &["Intro", "Verse", "Verse", "Chorus", "Chorus end", "Verse"],
    drums_from: 1,
    about: "A joropo from a zarzuela staged in Caracas in 1914, sung in Venezuela as a second \
            anthem. The verse, \"Yo nací en esta ribera\", swings between D minor and A7 and \
            passes through C and F, the relative major, on its way down; the chorus, \"Amo, \
            lloro, canto, sueño\", turns to D major on three chords. The 3/4 arpeggio stands \
            in for the harp and the cuatro.",
};

const PALOMITAS_A_CHORDS: &str = "I | vi | vi I | V I";
const PALOMITAS_B_CHORDS: &str = "vi | vi | vi I | V I";

// Source: https://raw.githubusercontent.com/NischalRana29/DCP_Assignment_C24392253/92b3d6334af726832dd44495e54494fa79c2a69e/abc_books/1/hnwaynu0.abc
// (Henrik Norbeck's wayñu collection, hnwaynu0.abc from norbeck.nu/abc, tune 20,
// "Dos Palomitas", no composer, with its instrumental variations)
pub(super) const DOS_PALOMITAS: Song = Song {
    name: "Dos palomitas",
    origin: "Traditional huayno, Andes",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Huayno",
    band: ANDEAN,
    style: "Huayno",
    parts: &[
        Part::new(
            "A",
            "G4:1 G4:2 F#4:1 G4:2 A4 B4:1 B4:2 A4:1 B4:2 B4 | E5:4 D5 B4:8 | \
             B4:1 E5:2 D5:1 B4:2 A4 G4:1 G4:2 F#4:1 G4:2 A4 | B4:4 A4 G4:8",
            PALOMITAS_A_CHORDS,
        ),
        Part::new(
            "B",
            "E5:4 D5 B4:8 | E5:4 D5 B4:8 | \
             B4:1 E5:2 D5:1 B4:2 A4 G4:1 G4:2 F#4:1 G4:2 A4 | B4:4 A4 G4:8",
            PALOMITAS_B_CHORDS,
        ),
        Part::new(
            "A variation",
            "G4:1 G4:2 F#4:1 G4:2 A4 B4:1 B4:2 A4:1 B4:2 B4 | E5:2 E5:1 G5:2 D5:1 E5:2 B4:8 | \
             B4:1 E5:2 D5:1 B4:2 A4 G4:1 G4:2 F#4:1 G4:2 A4 | B4:2 B4:1 D5:2 G4:1 A4:2 G4:8",
            PALOMITAS_A_CHORDS,
        ),
        Part::new(
            "B variation",
            "E5:4 D5 B4:8 | E5:4 D5 B4:8 | \
             B4:1 E5:2 D5:1 B4:2 A4 G4:1 G4:2 F#4:1 G4:2 A4 | B4:2 B4:1 D5:2 G4:1 A4:2 G4:8",
            PALOMITAS_B_CHORDS,
        ),
    ],
    form: &[
        "A",
        "A",
        "B",
        "B",
        "A variation",
        "A variation",
        "B variation",
        "B variation",
    ],
    drums_from: 0,
    about: "A huayno, the dance of the Andes, in 2/4: two of its bars to each bar here. The \
            tune uses only G, A, B, D, E and F sharp, leans on E minor and ends every phrase \
            on G, the two-sided tonality of Andean song. The second half is the instrumental \
            variation that answers the sung verse.",
};

// Original tunes, one for each folklore dance that has no public-domain
// melody with a written source this project could check. Each follows its
// dance's harmony, metre and phrase shapes.

pub(super) const ZAMBA_DEL_TELAR: Song = Song {
    name: "Zamba del telar",
    origin: "Written for Ferrous Loom",
    feel: "Zamba",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: SIX_EIGHT,
    tempo: 80.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Zamba",
    band: GUITARS,
    style: "Zamba",
    parts: &[
        Part::new(
            "Intro",
            "A5:6 G#5:2 A5 B5 | C6:4 A5 F5 | E5:6 D5:2 C5 B4 | A4:6 -:6",
            "i | iv | V7 | i",
        ),
        Part::new(
            "Verse",
            "E5:6 C5:2 B4 A4 | D5:4 F5 A5 | G#5:6 F5:2 E5 D5 | E5:2 D5 C5:8 | \
             G5:6 E5:2 F5 G5 | F5:4 D5 B4 | E5:2 F5 E5 D5 C5 B4 | A4:12",
            "i | iv | V7 | i | bIII | bVII | V7 | i",
        ),
        Part::new(
            "Estribillo",
            "G5:6 E5:2 G5 C6 | B5:4 A5 G5 | F5:6 A5:2 G5 F5 | E5:4 D5 B4 | \
             C5:6 E5:2 D5 C5 | D5:4 F5 A5 | G#5:2 A5 B5 G#5 E5 D5 | A5:12",
            "bIII | bVII | iv | V7 | i | iv | V7 | i",
        ),
    ],
    form: &[
        "Intro",
        "Verse",
        "Verse",
        "Estribillo",
        "Intro",
        "Verse",
        "Estribillo",
    ],
    drums_from: 0,
    about: "A slow zamba in 6/8 for a dance with handkerchiefs. Listen for the bars that \
            move in three quarter notes instead of two dotted ones, such as the D, F, A \
            over the iv chord: the zamba keeps crossing from 6/8 into 3/4. The estribillo \
            starts on the relative major, C, and finds its way back to A minor.",
};

pub(super) const CHACARERA_DEL_PATIO: Song = Song {
    name: "Chacarera del patio",
    origin: "Written for Ferrous Loom",
    feel: "Chacarera",
    genre: Genre::Folklore,
    tonic: E,
    mode: HarmonicMinor,
    time: SIX_EIGHT,
    tempo: 132.0,
    comping: Comping::Sway,
    bass: Bass::RootFifth,
    rhythm: "Chacarera",
    band: FIDDLE_AND_GUITAR,
    style: "Chacarera",
    parts: &[
        Part::new(
            "Intro",
            "B5:2 G5 E5 B5 G5 E5 | A5:2 F#5 D#5 A5 F#5 D#5 | \
             G5:2 E5 B4 G5 E5 B4 | F#5:4 D#5 B4",
            "i | V7 | i | V7",
        ),
        Part::new(
            "Verse",
            "E5:6 G5:2 F#5 E5 | D#5:4 F#5 A5 | G5:6 B5:2 A5 G5 | F#5:6 -:2 B4 B4 | \
             C5:4 E5 A5 | G5:6 E5:2 F#5 G5 | F#5:2 E5 D#5 E5 F#5 A5 | G5:4 F#5 E5",
            "i | V7 | i | V7 | iv | i | V7 | i",
        ),
        Part::new(
            "Estribillo",
            "A5:6 C6:2 B5 A5 | G5:4 B5 E5 | F#5:2 G5 A5 F#5 D#5 B4 | E5:6 -:6",
            "iv | i | V7 | i",
        ),
    ],
    form: &[
        "Intro",
        "Verse",
        "Estribillo",
        "Intro",
        "Verse",
        "Estribillo",
    ],
    drums_from: 0,
    about: "The chacarera of Santiago del Estero, quick and in E minor. The bombo plays \
            6/8 on its head and 3/4 on its rim at the same time, and the tune does the \
            same: most bars go in two dotted beats, but the V7 bar of each verse and its \
            last bar move in three quarter notes.",
};

pub(super) const GATO_DEL_ARROYO: Song = Song {
    name: "Gato del arroyo",
    origin: "Written for Ferrous Loom",
    feel: "Gato",
    genre: Genre::Folklore,
    tonic: G,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 126.0,
    comping: Comping::Waltz,
    bass: Bass::RootFifth,
    rhythm: "Gato",
    band: FIDDLE_AND_GUITAR,
    style: "Gato",
    parts: &[
        Part::new(
            "A",
            "D5:2 G5 B5 A5 G5 D5 | C5:2 F#5 A5 G5 F#5 C5 | D5:4 F#5 A5 | G5:6 -:2 D5 E5 | \
             B5:2 B5 B5 A5 G5 D5 | C6:2 B5 A5 F#5 E5 D5 | A5:4 F#5 D5 | G5:6 -:6",
            "I | V7 | V7 | I | I | V7 | V7 | I",
        ),
        Part::new(
            "B",
            "E5:2 G5 C6 B5 A5 G5 | B5:4 G5 D5 | A5:2 B5 C6 A5 F#5 D5 | G5:6 -:6",
            "IV | I | V7 | I",
        ),
    ],
    form: &["A", "A", "B", "B", "A"],
    drums_from: 0,
    about: "The gato is the quickest of the old dances of the Pampas, and nearly all of \
            it is two chords, I and V7. The eighth notes run in 6/8, but the guitar plays \
            on beats 2 and 3 of a 3/4 bar, so the two metres rub against each other.",
};

pub(super) const CHAMAME_DEL_ESTERO: Song = Song {
    name: "Chamamé del estero",
    origin: "Written for Ferrous Loom",
    feel: "Chamamé",
    genre: Genre::Folklore,
    tonic: A,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 116.0,
    comping: Comping::Sway,
    bass: Bass::Waltz,
    rhythm: "Chamamé",
    band: ACCORDION_BAND,
    style: "Chamamé",
    parts: &[
        Part::new(
            "A",
            "E5:3 C#5:1 E5:2 A5:4 G#5:2 | F#5:4 E5:2 D5:4 B4:2 | \
             D5:3 B4:1 D5:2 G#5:4 F#5:2 | E5:6 C#5:6 | \
             F#5:3 D5:1 F#5:2 A5:4 B5:2 | A5:4 E5:2 C#5:4 E5:2 | \
             D5:2 E5 F#5 G#5:4 B4:2 | A5:6 -:6",
            "I | V7 | V7 | I | IV | I | V7 | I",
        ),
        Part::new(
            "B",
            "A5:4 F#5:2 D5:4 F#5:2 | E5:3 A5:1 C#6:2 B5:4 A5:2 | \
             G#5:3 B5:1 G#5:2 E5:4 D5:2 | C#5:6 E5:6 | \
             F#5:2 E5 D5 F#5 A5 F#5 | E5:2 C#5 A4 C#5 E5 A5 | \
             G#5:4 F#5:2 E5:4 D5:2 | A4:6 -:6",
            "IV | I | V7 | I | IV | I | V7 | I",
        ),
    ],
    form: &["A", "A", "B", "A"],
    drums_from: 0,
    about: "From Corrientes, played on the accordion. The tune leans on a dotted eighth, \
            a sixteenth and an eighth, and the bars that end in a quarter and an eighth \
            pull it into 3/4 while the bass keeps 6/8. Three chords, I, IV and V7, in A.",
};

pub(super) const CUECA_DE_LA_CALETA: Song = Song {
    name: "Cueca de la caleta",
    origin: "Written for Ferrous Loom",
    feel: "Cueca",
    genre: Genre::Folklore,
    tonic: D,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 120.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Cueca",
    band: ACCORDION_BAND,
    style: "Cueca",
    parts: &[
        Part::new(
            "Verse",
            "A4:2 D5 F#5 A5:4 F#5:2 | G5:4 E5 C#5 | E5:2 G5 E5 C#5:4 A4:2 | D5:4 F#5 A5 | \
             B5:2 A5 F#5 A5:4 D5:2 | G5:2 B5 G5 D5:4 B4:2 | C#5:2 E5 G5 E5 C#5 A4 | D5:6 -:6",
            "I | V7 | V7 | I | I | IV | V7 | I",
        ),
        Part::new("Remate", "E5:4 G5 C#5 | D5:6 -:6", "V7 | I"),
    ],
    form: &["Verse", "Verse", "Verse", "Remate"],
    drums_from: 0,
    about: "Chile's national dance, in D major with hand claps between the beats. Its \
            phrases end in quarter notes that cross into 3/4, and a short remate of two \
            bars, V7 then I, closes the dance.",
};

pub(super) const MARINERA_DEL_CALLEJON: Song = Song {
    name: "Marinera del callejón",
    origin: "Written for Ferrous Loom",
    feel: "Marinera",
    genre: Genre::Folklore,
    tonic: D,
    mode: HarmonicMinor,
    time: SIX_EIGHT,
    tempo: 116.0,
    comping: Comping::Arp3,
    bass: Bass::Held,
    rhythm: "Marinera",
    band: GUITARS,
    style: "Marinera",
    parts: &[
        Part::new(
            "Verse",
            "A4:2 D5 F5 A5:4 F5:2 | G5:2 Bb5 G5 D5:4 Bb4:2 | C#5:2 E5 G5 A5:4 G5:2 | \
             F5:4 E5 D5 | C5:2 F5 A5 C6:4 A5:2 | G5:2 E5 C5 E5:4 G5:2 | \
             A5:2 G5 F5 E5 C#5 E5 | D5:6 -:6",
            "i | iv | V7 | i | bIII | bVII | V7 | i",
        ),
        Part::new(
            "Fuga",
            "D5:2 F5 A5 D5 F5 A5 | E5:2 G5 A5 C#5 E5 A5 | G5:2 E5 C#5 A4 C#5 E5 | D5:6 -:6",
            "i | V7 | V7 | i",
        ),
    ],
    form: &["Verse", "Verse", "Fuga", "Fuga"],
    drums_from: 0,
    about: "Peru's courtship dance for guitar and cajón, in D minor. The verse visits the \
            relative major, F, and its neighbour C before V7 brings it home; then a quick \
            fuga in running eighths ends the dance, as a marinera does.",
};

pub(super) const BAMBUCO_DEL_PARAMO: Song = Song {
    name: "Bambuco del páramo",
    origin: "Written for Ferrous Loom",
    feel: "Bambuco",
    genre: Genre::Folklore,
    tonic: E,
    mode: HarmonicMinor,
    time: SIX_EIGHT,
    tempo: 104.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Bambuco",
    band: FLUTE_AND_GUITAR,
    style: "Bambuco",
    parts: &[Part::new(
        "Verse",
        "-:2 B4 E5 G5:4 F#5:2 | -:2 A5 F#5 D#5:4 B4:2 | -:2 E5 G5 B5:4 A5:2 | -:2 G5 E5 C5:6 | \
         -:2 D5 G5 B5:4 G5:2 | -:2 A5 F#5 D#5 F#5 A5 | G5:4 F#5 E5 | E5:6 -:6",
        "i | V7 | i | bVI | bIII | V7 | i | i",
    )],
    form: &["Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "Colombia's Andean song, in E minor. Almost every phrase starts an eighth note \
            after the downbeat, the bambuco's trademark, and the chords wander to C and G, \
            bVI and bIII, before V7 brings them home.",
};

pub(super) const MALAMBO_DEL_FOGON: Song = Song {
    name: "Malambo del fogón",
    origin: "Written for Ferrous Loom",
    feel: "Malambo",
    genre: Genre::Folklore,
    tonic: A,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 144.0,
    comping: Comping::Waltz,
    bass: Bass::Quarters,
    rhythm: "Malambo",
    band: GUITARS,
    style: "Malambo",
    parts: &[Part::new(
        "Mudanza",
        "A4:2 C#5 E5 A5 E5 C#5 | B4:2 D5 E5 G#5 E5 D5 | C#5:2 E5 A5 C#6 A5 E5 | \
         D5:2 E5 G#5 B5 G#5 E5 | A5:4 E5 C#5 | B4:4 E5 G#5 | A5:2 G#5 A5 E5 C#5 A4 | \
         B4:2 D5 G#4 A4:6",
        "I | V7 | I | V7 | I | V7 | I | V7 I",
    )],
    form: &["Mudanza", "Mudanza", "Mudanza"],
    drums_from: 0,
    about: "The gaucho's dance of stamping boots, where the guitar only keeps time: I and \
            V7, over and over, in running eighths. Bars 5 and 6 switch to three quarter \
            notes, the 3/4 the dancer's feet answer.",
};

pub(super) const VIDALA_DE_LA_QUEBRADA: Song = Song {
    name: "Vidala de la quebrada",
    origin: "Written for Ferrous Loom",
    feel: "Vidala",
    genre: Genre::Folklore,
    tonic: A,
    mode: NaturalMinor,
    time: THREE_FOUR,
    tempo: 66.0,
    comping: Comping::Sway,
    bass: Bass::Held,
    rhythm: "Vidala",
    band: SONG_AND_GUITAR,
    style: "Vidala",
    parts: &[Part::new(
        "Copla",
        "E5:8 C5:4 | A4:8 C5:4 | D5:8 B4:4 | E5:4 G5 E5 | E5:8 A5:4 | A5:4 F5 D5 | \
         D5:4 B4 G4 | A4:12",
        "i | i | bVII | bIII | i | iv | bVII | i",
    )],
    form: &["Copla", "Copla", "Copla"],
    drums_from: 0,
    about: "A slow song of the Andean north-west of Argentina, sung over a hand drum. The \
            tune keeps to the notes of the A minor chord, A, C and E, with a few steps \
            between, and the chords never use a dominant: G and C, bVII and bIII, lead \
            home instead.",
};

pub(super) const CARNAVALITO_DE_LA_QUEBRADA: Song = Song {
    name: "Carnavalito de la quebrada",
    origin: "Written for Ferrous Loom",
    feel: "Carnavalito",
    genre: Genre::Folklore,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Carnavalito",
    band: ANDEAN,
    style: "Carnavalito",
    parts: &[Part::new(
        "A",
        "A5:2 A5:1 A5 G5:2 E5 A5:2 A5:1 A5 G5:2 E5 | D5:2 D5:1 E5 G5:2 E5 D5:4 B4:4 | \
         E5:2 E5:1 G5 E5:2 C5 E5:2 G5:1 E5 D5:2 C5 | B4:2 B4:1 C5 D5:2 E5 E5:8 | \
         C6:2 C6:1 C6 A5:2 G5 A5:2 A5:1 G5 E5:2 D5 | D5:2 D5:1 E5 G5:2 A5 G5:2 E5 D5:4 | \
         E5:2 E5:1 D5 B4:2 D5 E5:2 D5:1 C5 B4:4 | A4:8 -:8",
        "i | bVII | bIII | V7 | i | bVII | V7 | i",
    )],
    form: &["A", "A", "A", "A"],
    drums_from: 0,
    about: "The carnival dance of the Quebrada de Humahuaca, for quena, charango and \
            bombo. It is a quick 2/4, written here two bars to a bar, and its tune uses \
            only five notes, A, C, D, E and G: the minor pentatonic scale of the Andes, \
            in an eighth and two sixteenths again and again.",
};

pub(super) const CANDOMBE_DEL_BARRIO: Song = Song {
    name: "Candombe del barrio",
    origin: "Written for Ferrous Loom",
    feel: "Candombe",
    genre: Genre::Folklore,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Offbeat,
    bass: Bass::Tresillo,
    rhythm: "Candombe",
    band: CANDOMBE,
    style: "Candombe",
    parts: &[Part::new(
        "A",
        "-:2 G4:2 C5:3 E5:3 G5:4 E5:2 | F5:3 A5:3 F5:2 C5:4 -:4 | \
         -:2 D5:2 F5:3 G5:3 B5:4 G5:2 | E5:3 G5:3 E5:2 C5:8 | \
         -:2 C5:2 E5:3 G5:3 C6:4 B5:2 | A5:3 F5:3 C5:2 A5:4 G5:4 | \
         F5:3 D5:3 B4:2 D5:4 F5:4 | E5:4 D5 C5:8",
        "I | IV | V7 | I | I | IV | V7 | I",
    )],
    form: &["A", "A", "A"],
    drums_from: 0,
    about: "Montevideo's candombe, across the river from Buenos Aires. The tune moves in \
            3 + 3 + 2, two dotted eighths and an eighth, the same clave the stick plays on \
            the drum's wooden shell, over three plain chords in C major.",
};
