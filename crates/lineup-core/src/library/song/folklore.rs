//! Folklore: the dances and songs of Latin America, from Mexico to Argentina.

use super::*;

/// The slow theme rests on A minor and its relative major, C.
const CONDOR_THEME_CHORDS: &str = "i | bIII | bIII | bIII | i | i";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/DanielA.abc
// (the Wikifonia lead sheet of the 1933 E. B. Marks piano edition, in D minor,
// with Robles's Andante, D.C. al Coda, and the Allegro fuga; cross-checked
// against John Chambers's ABC, https://trillian.mit.edu/~jc/music/abc/song/El_Condor_Pasa-Am-34-0.abc,
// which has the Andante only. The lead sheet has no chords; the charts follow
// the tune. Where it slurs two notes of the same pitch, they are one note here,
// as in Chambers's ABC. After the D.C. the Andante's repeat is not taken again.)
// PD: Robles died in 1942 (https://en.wikipedia.org/wiki/El_C%C3%B3ndor_Pasa_(song)).
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
        // Its last bar is where "To Coda" stands.
        Part::new(
            "B",
            "A5:6 G#5:2 A5:3 G#5:1 A5:3 G#5:1 | A5:12 C6:2 A5 | G5:12 A5:2 G5 | E5:12 -:2 E5 | \
             A5:6 G#5:2 A5:3 G#5:1 A5:3 G#5:1 | A5:12 C6:2 A5 | G5:12 A5:2 G5 | E5:12 D5:2 C5 | \
             A4:12 C5:4 | A4:12 E5:2 D5:1 C5",
            "bVI | bVI | bIII | bIII | bVI | bVI | bIII | bIII | i | i",
        ),
        // The bar marked "D.C.", with the pickup back to the theme.
        Part::new("D.C.", "A4:12 -:2 E4:2", "i"),
        Part::new(
            "Coda",
            "A4:12 C5:4 | A4:12 E5:2 D5:1 C5 | A4:12 C5:4 | A4:12 -:4",
            "i | i | i | i",
        )
        // The Andante ends under a fermata.
        .with_tempo(&[fermata(48, 12)]),
        // "Allegro": two bars that the lead sheet leaves to the accompaniment,
        // half as fast again as the Andante.
        Part::new("Fuga intro", "-:16 | -:16", "i | i").with_tempo(&[tempo(0, 120)]),
        Part::new(
            "Fuga A",
            "E5:1 A5 E5 A5 E5:2 D5 C5:1 A4:2 A4:5 | A4:1 B4 C5 D5 E5:2 G5 E5:8 | \
             E5:1 A5 E5 A5 E5:2 D5 C5:1 A4:2 G4:1 A4:4 | A4:1 B4 C5 D5 E5 D5 C5 B4 A4:8",
            "i | bIII | i | i",
        ),
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
    // The fuga plays A twice, then B and C twice over.
    form: &[
        "Pickup",
        "Theme",
        "Theme into B",
        "B",
        "D.C.",
        "Theme into B",
        "B",
        "Coda",
        "Fuga intro",
        "Fuga A",
        "Fuga A",
        "Fuga B",
        "Fuga C",
        "Fuga B",
        "Fuga C",
        "Fuga end",
    ],
    drums_from: 8,
    about: "From Robles's zarzuela of the same name, Lima, 1913. The slow theme rests on A \
            minor and its relative major, C; the second strain rises to F and climbs to the \
            high C before it comes home, the whole Andante is played again, and a short coda \
            ends it under a fermata. Then the fuga, the fast dance that ends the piece, runs \
            up and down the same scale in sixteenths, and the drums come in with it. The 1933 \
            piano edition is in D minor; it sits in A minor here.",
};

// Source: https://raw.githubusercontent.com/baobach/abc_search/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmbQQ3V7BEHLkjDvy16ozcfijwzH9FFppgH3LJcv6aThg4.abc
// (a piano arrangement in ABC, "Tiempo de Joropo", one flat with C sharps written in,
// turning to D major for the chorus; the tune is the piano's top voice and the
// chords are read from the left hand. The low run of the introduction is an
// octave up here, for the range. Where the right hand only accompanies, in the
// interludes and before the chorus, the melody rests and the band plays the
// chords. The very last note, D6, is the score's.)
// PD: Gutiérrez died in 1954 (https://en.wikipedia.org/wiki/Pedro_El%C3%ADas_Guti%C3%A9rrez)
// and the lyricist, Rafael Bolívar Coronado, in 1924
// (https://onlinebooks.library.upenn.edu/webbin/who/Bol%26iacute%3bvar%20Coronado%2c%20Rafael%2c%201884%2d1924).
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
        // The left hand alone: D minor and A7.
        Part::new("Vamp", "-:12 | -:12", "i | V7"),
        Part::new(
            "Intro low",
            "D4:2 E4 F4 G4 A4 Bb4 | A4:4 C#5:4 A4:2 C#5:2 | \
             D4:2 E4 F4 G4 A4 Bb4 | A4:4 C#5:4 A4:2 C#5:2",
            "i | V7 | i | V7",
        ),
        Part::new(
            "Intro",
            "D4:2 E4 F4 G4 A4 Bb4 | A4:4 A5:8 | D4:2 E4 F4 G4 A4 Bb4 | A4:4 A5:8",
            "i | V7 | i | V7",
        ),
        // With the first ending, which goes back to the vamp.
        Part::new(
            "Verse",
            "D5:2 F5:4 E5:4 D5:2 | C#5:2 E5:4 D5:4 C#5:2 | D5:4 A4:8 | _:12 | \
             D5:2 F5:4 E5:4 D5:2 | C#5:2 E5:4 D5:4 C#5:2 | D5:4 A4:8 | _:12 | \
             D5:2 F5:4 E5:4 D5:2 | C5:2 D5:4 C5:4 Bb4:2 | A4:12 | G4:2 G4:4 A4:4 Bb4:2 | \
             A4:4 D5:4 A4:4 | A4:2 G4:2 E4:4 F4:4 | D4:12 | A3:8 -:4",
            "i | V7 | i | V7 | i | V7 | i | V7 | i | bVII | bIII | ii° | i | V7 | i | V7",
        ),
        // With the second ending, where the interlude starts.
        Part::new(
            "Verse end",
            "D5:2 F5:4 E5:4 D5:2 | C#5:2 E5:4 D5:4 C#5:2 | D5:4 A4:8 | _:12 | \
             D5:2 F5:4 E5:4 D5:2 | C#5:2 E5:4 D5:4 C#5:2 | D5:4 A4:8 | _:12 | \
             D5:2 F5:4 E5:4 D5:2 | C5:2 D5:4 C5:4 Bb4:2 | A4:12 | G4:2 G4:4 A4:4 Bb4:2 | \
             A4:4 D5:4 A4:4 | A4:2 G4:2 E4:4 F4:4 | -:12",
            "i | V7 | i | V7 | i | V7 | i | V7 | i | bVII | bIII | ii° | i | V7 | i",
        ),
        Part::new(
            "Interlude",
            "-:12 | -:12 | -:12 | -:12 | -:12 | -:12 | -:12",
            "ii° | V7 | V7 | i | ii° | V7 | V7",
        ),
        // Played twice; the E flat chord is the Neapolitan, bII.
        Part::new(
            "Interlude 2",
            "-:4 A5:4 A4:2 A5:2 | -:4 A5:4 A4:2 A5:2 | -:4 A5:4 A4:2 A5:2 | \
             -:4 Bb5:4 Bb4:2 Bb5:2 | -:4 A5:4 A4:2 A5:2 | -:2 C#5:2 E5:2 G5:2 F5:2 E5:2",
            "i | V7 | i | bII | V7 | V7",
        ),
        Part::new(
            "Interlude 3",
            "-:4 A4:4 F4:2 A4:2 | -:4 Bb4:4 E4:2 Bb4:2 | -:4 A4:4 E4:2 A4:2 | \
             -:2 C#4:2 D4:2 G4:2 F4:2 E4:2 | D4:4 A4:4 F4:2 A4:2 | -:4 A4:4 F4:2 A4:2",
            "i | iv6 | V7 | V7 | i | i",
        ),
        // The second minor strain, played twice.
        Part::new(
            "Verse 2",
            "F5:2 F5 -:2 F5 E5 D5 | E5:4 E5:6 -:2 | E5:2 E5 -:2 E5 D5 C#5 | D5:4 A4:6 -:2 | \
             A4:2 A4 -:2 A4 C5:3 C5:1 | Bb4:4 E4:8 | G4:2 G4 -:2 G4 A4:3 A4:1 | F4:4 D4:8",
            "i | V7 | V7 | i | I7 | iv6 | V7 | i",
        ),
        Part::new(
            "Verse 2 end",
            "A4:2 A4 -:2 A4 G4 F4 | A4:4 E4:8 | G4:2 G4 -:2 G4 F4 E4 | D4:12 | \
             A4:2 A4 -:2 A4 G4 F4 | A4:4 E4:8 | G4:2 G4 -:2 G4 F4 E4",
            "i | V7 | V7 | i | i | V7 | V7",
        ),
        Part::new(
            "Chorus intro",
            "-:12 | -:12 | -:12 | -:12 | -:12 | -:12 | -:12 | -:12",
            "I | IV6 | V7 | V7 | I | IV6 | V7 | V7",
        )
        .in_key(D, Major),
        Part::new(
            "Chorus",
            "A4:12 | _:12 | _:8 A4:4 | D5:6 C#5:2 E5 D5 | F#5:6 E5:2 D5 C#5 | E5:6 D5:2 C#5 B4 | C#5:12 | _:12 | -:4 B4:4 B4 | D5:6 D5:2 D5 D5 | C#5:4 A4:8 | _:12 | -:4 B4:4 B4 | D5:6 D5:2 D5 D5",
            "I | V7 | I | ii7 | I | IV | V7 | V7 | IV | IV | V7 | V7 | IV | IV",
        )
        .in_key(D, Major),
        // With the first ending, which goes back to the chords before the chorus.
        Part::new(
            "Chorus end",
            "C#5:4 A4:8 | _:12 | -:4 B4:4 B4 | D5:6 D5:2 D5 D5 | A4:4 A4 A4:2 A4 | B4:4 G4 G4:2 G4 | A4:4 F#4 -:4 | -:4 G4:4 E4 | A4:12 | _:12 | _:12 | _:4 G4 E4 | D4:4 -:8",
            "V7 | V7 | IV | IV | V7 | IV | I | ii7 | I | I | V7 | V7 | I",
        )
        .in_key(D, Major),
        Part::new(
            "Chorus vamp",
            "-:12 | -:12 | -:12 | -:12 | -:12 | -:12 | -:12",
            "IV6 | V7 | V7 | I | IV6 | V7 | V7",
        )
        .in_key(D, Major),
        // With the second ending and the pickup into the coda.
        Part::new(
            "Chorus last",
            "C#5:4 A4:8 | _:12 | -:4 B4:4 B4 | D5:6 D5:2 D5 D5 | A4:4 A4 A4:2 A4 | B4:4 G4 G4:2 G4 | A4:4 F#4 -:4 | -:4 G4:4 E4 | A4:12 | _:12 | _:12 | _:4 G4 E4 | D4:4 -:8 | A4:4 A4 -:4",
            "V7 | V7 | IV | IV | V7 | IV | I | ii7 | I | I | V7 | V7 | I | I",
        )
        .in_key(D, Major),
        Part::new(
            "Coda",
            "B4:4 B4 -:4 | C#5:4 C#5 -:4 | A4:4 A4 -:4 | F#5:2 D5 E5 C#5 D5 B4 | G4:12 | \
             E5:2 C#5 D5 B4 C#5 A4 | F#4:12 | -:4 A4:4 F#4:2 A4:2 | -:4 B4:4 G4:2 B4:2 | \
             -:4 C#5:4 A4:2 C#5:2 | -:4 A4:4 F#4:2 A4:2 | F#5:2 D5 E5 C#5 D5 B4 | C#5:4 A4 A4 | \
             B4:4 A4:2 G4 F#4 E4 | D4:4 -:8",
            "IV | V7 | I | I | V7 | V7 | I | I | IV6 | V7 | I | I | V7 | V7 | I",
        )
        .in_key(D, Major),
        // The second strain's last phrase again, now in D major, and the final chords.
        Part::new(
            "Coda end",
            "A4:2 A4 A4 A4 G4 F#4 | A4:4 E4:8 | G4:2 G4 G4 G4 F#4 E4 | D4:4 A5:4 A4:2 A5:2 | \
             A4:2 A4 A4 A4 G4 F#4 | A4:4 E4:8 | G4:2 A4 B4 C#5 D5 E5 | F#5:4 F#5:2 F#5:2 F#5:4 | \
             F#5:4 F#5 F#5 | F#5:4 -:8 | E5:4 D5 A4 | -:4 D5:2 D5:2 D5:4 | D5:4 D5 D5 | \
             D5:4 -:8 | A5:4 -:8 | D6:4 -:8",
            "I | V7 | V7 | I | I | V7 | V7 | I | I | I | V7 | I | I | I | V | I",
        )
        .in_key(D, Major),
    ],
    form: &[
        "Vamp",
        "Vamp",
        "Intro low",
        "Intro",
        "Verse",
        "Vamp",
        "Intro low",
        "Intro",
        "Verse end",
        "Interlude",
        "Interlude 2",
        "Interlude 2",
        "Interlude 3",
        "Verse 2",
        "Verse 2",
        "Verse 2 end",
        "Chorus intro",
        "Chorus",
        "Chorus end",
        "Chorus vamp",
        "Chorus",
        "Chorus last",
        "Coda",
        "Coda end",
    ],
    drums_from: 4,
    about: "A joropo from a zarzuela staged in Caracas in 1914, sung in Venezuela as a second \
            anthem. The first strain swings between D minor and A7 and passes through C and F, \
            the relative major, on its way down; after the interludes a second minor \
            strain turns through D7 to G minor, and the chorus, \"Amo, lloro, canto, sueño\", \
            turns to D major, where the coda brings back the second strain's last phrase. The \
            3/4 arpeggio stands in for the harp and the cuatro.",
};

const PALOMITAS_A_CHORDS: &str = "I | vi | vi I | V I";
const PALOMITAS_B_CHORDS: &str = "vi | vi | vi I | V I";

// Source: https://raw.githubusercontent.com/NischalRana29/DCP_Assignment_C24392253/92b3d6334af726832dd44495e54494fa79c2a69e/abc_books/1/hnwaynu0.abc
// (Henrik Norbeck's wayñu collection, hnwaynu0.abc from norbeck.nu/abc, tune 20,
// "Dos Palomitas", no composer, with its instrumental variations; A and B checked
// against Jack Campin's ABC in F, the same notes a tone lower:
// https://github.com/syntithenai/abc2book/blob/master/abcresources/misc/abc_tune_misc_0.abc, X:262.
// Neither has chords; the charts follow the tune.)
// PD: traditional (neither Norbeck nor Campin names a composer).
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
            tune uses only G, A, B, D, E and F sharp, leans on E minor and ends each strain \
            on G, the two-sided tonality of Andean song. The second half is the instrumental \
            variation that answers the sung verse.",
};

// Original tunes, one for each folklore dance that has no public-domain
// melody with a written source this project could check. Each follows its
// dance's harmony, metre and phrase shapes.

pub(super) const ZAMBA_DEL_TELAR: Song = Song {
    name: "Zamba del telar",
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
    origin: "Written for Ferrous Lineup",
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
            3 + 3 + 2, two dotted eighths and an eighth, the tresillo that the milonga and \
            the tango lean on too, over three plain chords in C major.",
};

/// The verse: E minor, then D7 into G, the relative major, and A minor and
/// B7 back home.
const PAPEL_A_CHORDS: &str = "i i bVII7 bIII | iv iv V7 i | i i bVII7 bIII | iv iv V7 i";

/// The refrain, "Ay, palomita", in 3/4.
const PAPEL_B_CHORDS: &str = "i bVII7 bIII | bIII bVII7 bIII | bIII iv bIII | V7 i i";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/norbeck/abc_tune_norbeck_2990.abc
// (Henrik Norbeck's wayñu collection, hnwaynu0.abc from norbeck.nu/abc, tune 26,
// "Papel de plata", D: Inti-Illimani, in E minor; the verse in 2/4, the refrain in 3/4)
// Chords: https://acordes.lacuerda.net/intiillimani/papel_de_plata.shtml (Inti-Illimani's
// version, in E minor: Em D7 G / Am Am/G B7 Em for the verse, Em D7 G / D7 G / Am G /
// B7 Em for the refrain), placed on the syllables the chord sheet puts them on; Am/G is
// written Am.
// PD: traditional (credited "Traditional" on Inti-Illimani's "Canto de pueblos andinos";
// Norbeck names no composer)
pub(super) const PAPEL_DE_PLATA: Song = Song {
    name: "Papel de plata",
    origin: "Traditional huayno, Andes",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: E,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Huayno",
    band: ANDEAN,
    style: "Huayno",
    parts: &[
        Part::new(
            "Verse",
            "E4:1 B4:2 B4 B4 A4:1 B4:2 B4 D5:4 | F#4:1 A4:2 A4 G4 E4:1 D4:2 D4 E4:4 | \
             E4:1 B4:2 B4 B4 A4:1 B4:2 B4 D5:4 | A4:1 A4:2 A4 G4 E4:1 G4:2 G4 E4:4",
            PAPEL_A_CHORDS,
        ),
        Part::new(
            "Refrain",
            "B4:3 A4:2 G4 E4:1 D4:4 | G4:2 A4:1 B4:2 B4 A4:1 B4:4 | \
             A4:2 A4:1 A4:2 G4 E4:1 D4:4 | G4:2 A4 G4 G4 E4:4",
            PAPEL_B_CHORDS,
        )
        .in_time(THREE_FOUR)
        .played(Comping::Waltz, Bass::Waltz, "Vidala"),
    ],
    form: &["Verse", "Verse", "Refrain", "Refrain"],
    drums_from: 0,
    about: "A huayno in E minor, \"Papel de plata quisiera\": E minor, then D7 resolving to G, \
            the relative major, and A minor and B7 to come home. The tune keeps its D natural \
            even over B7, the two-sided tonality of Andean song. The verse is in 2/4, two bars \
            to a bar here; the refrain, \"Ay, palomita\", turns to 3/4.",
};

const QUEBRADA_A_CHORDS: &str = "i | bIII i";
const QUEBRADA_B_CHORDS: &str = "bVII i | bIII i";

/// The second strain, the second time: its last bar turns back up, and the
/// last phrase is played twice more.
const QUEBRADA_B_END_CHORDS: &str = "bVII i | bIII i | bIII i | bIII i";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/norbeck/abc_tune_norbeck_2979.abc
// (Henrik Norbeck's wayñu collection, tune 15, "La Quebrada de Humahuaca", O: Argentina,
// in E minor, 2/4, both strains repeated, the second with first and second endings)
// Chords: the source has none; these are the plain chords of the style (i, bIII, bVII),
// chosen to fit the melody.
// PD: traditional (Inti-Illimani's "Canto de pueblos andinos" credits "Quebrada de
// Humahuaca" as traditional; Norbeck names no composer)
pub(super) const LA_QUEBRADA_DE_HUMAHUACA: Song = Song {
    name: "La Quebrada de Humahuaca",
    origin: "Traditional huayno, northern Argentina",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: E,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Huayno",
    band: ANDEAN,
    style: "Huayno",
    parts: &[
        Part::new(
            "A",
            "E5:1 E5:2 E5:1 E5 F#5:2 G5:1 G5:1 F#5:2 E5:1 E5:2 D5 | \
             B4:1 B4:2 B4:1 A4:2 G4 E4:3 E4 E4:2",
            QUEBRADA_A_CHORDS,
        ),
        Part::new(
            "B",
            "D5:1 D5:2 D5:1 E5 B4:2 A4:1 G4:1 E4:2 D5:1 D5:2 E5 | \
             B4:1 B4:2 B4:1 A4:2 G4 E4:3 E4 E4:2",
            QUEBRADA_B_CHORDS,
        ),
        Part::new(
            "B end",
            "D5:1 D5:2 D5:1 E5 B4:2 A4:1 G4:1 E4:2 D5:1 D5:2 E5 | \
             B4:1 B4:2 B4:1 A4:2 G4 E4:3 D5:1 D5:2 E5 | \
             B4:1 B4:2 B4:1 A4:2 G4 E4:3 D5:1 D5:2 E5 | \
             B4:1 B4:2 B4:1 A4:2 G4 E4:3 E4 E4:2",
            QUEBRADA_B_END_CHORDS,
        ),
    ],
    form: &["A", "A", "B", "B end"],
    drums_from: 0,
    about: "A huayno named after the valley of Jujuy, in E minor and 2/4, two bars to a bar \
            here. Every phrase falls to the same cadence, B, A, G and three Es in the long, \
            long, short of the huayno, and the second strain starts a step lower, on D, the \
            flat seventh, with no D sharp anywhere.",
};

const HUAYTA_A_CHORDS: &str = "I | V7 I";
const HUAYTA_B_CHORDS: &str = "I | vi I";
const HUAYTA_C_CHORDS: &str = "V7 I | V7 I";

/// The third strain again, then the fourth twice, each bar swinging from D
/// to A7 on a tied A.
const HUAYTA_C_INTO_D_CHORDS: &str = "V7 I | V7 I | I V7 I V7 | V7 I | I V7 I V7 | V7 I";

/// The fifth strain ends on F#7 and B minor, the relative minor.
const HUAYTA_E_CHORDS: &str = "I | ii7 V7 | I | III7 vi";
const HUAYTA_F_CHORDS: &str = "I | I | I | I vi";

const HUAYTA_C_TUNE: &str = "E4:1 E4:2 F#4:1 G4 F#4:2 E4:1 D4:2 F#4 A4:4 | \
                             E4:1 E4:2 F#4:1 G4 F#4:2 E4:1 D4:2 D4 D4:4";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/norbeck/abc_tune_norbeck_2992.abc
// (Henrik Norbeck's wayñu collection, tune 28, "Huayta Huaytucha", D: Los Incas, in D,
// 2/4, six strains with their repeats, endings and pickups)
// Chords: the source has none; these are the plain chords of the style, chosen to fit
// the melody (the F#7 of the fifth strain is spelled out by its tune, C#, A#, F#, E).
// PD: traditional (Norbeck names no composer)
pub(super) const HUAYTA_HUAYTUCHA: Song = Song {
    name: "Huayta huaytucha",
    origin: "Traditional huayno, Andes",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Huayno",
    band: ANDEAN,
    style: "Huayno",
    parts: &[
        Part::new(
            "A",
            "D4:1 F#4:2 D4:1 F#4 D4:2 F#4:1 A4:2 A4 A4:4 | \
             E4:1 E4:2 F#4:1 G4 F#4:2 E4:1 D4:2 D4 D4:4",
            HUAYTA_A_CHORDS,
        ),
        Part::new(
            "B",
            "A4:1 A4:2 F#4:1 A4:2 B4:1 C#5 D5:1 D5:2 B4:1 D5:4 | \
             B4:1 B4:2 C#5:1 D5 C#5:2 B4:1 A4:1 A4:2 F#4:1 A4:4",
            HUAYTA_B_CHORDS,
        ),
        Part::new("C", HUAYTA_C_TUNE, HUAYTA_C_CHORDS),
        // The second ending leads into the fourth strain on an A tied over the
        // bar line, and so does every bar of it.
        Part::new(
            "C into D",
            "E4:1 E4:2 F#4:1 G4 F#4:2 E4:1 D4:2 F#4 A4:4 | \
             E4:1 E4:2 F#4:1 G4 F#4:2 E4:1 D4:2 D4 D4:3 A4:1 | \
             _:1 F#4:2 D4:1 E4:3 A4:1 _:1 F#4:2 D4:1 E4:4 | \
             E4:1 E4:2 F#4:1 G4 F#4:2 E4:1 D4:2 D4 D4:3 A4:1 | \
             _:1 F#4:2 D4:1 E4:3 A4:1 _:1 F#4:2 D4:1 E4:4 | \
             E4:1 E4:2 F#4:1 G4 F#4:2 E4:1 D5:2 D5 D5 D5:1 D5",
            HUAYTA_C_INTO_D_CHORDS,
        ),
        Part::new(
            "E",
            "D5:4 B4:2 D5:1 D5 D5:4 B4:2 C#5:1 D5 | E5:3 D5:2 B4:1 D5:2 A4:4 A4:2 A4 | \
             A4:4 F#4:2 A4:1 A4 A4:4 F#4:2 B4:1 B4 | C#5:2 A#4:1 F#4:2 F#4:1 E4:2 D4:4 D5:2 D5",
            HUAYTA_E_CHORDS,
        ),
        Part::new(
            "E end",
            "D5:4 B4:2 D5:1 D5 D5:4 B4:2 C#5:1 D5 | E5:3 D5:2 B4:1 D5:2 A4:4 A4:2 A4 | \
             A4:4 F#4:2 A4:1 A4 A4:4 F#4:2 B4:1 B4 | C#5:2 A#4:1 F#4:2 F#4:1 E4:2 D4:8",
            HUAYTA_E_CHORDS,
        ),
        Part::new(
            "F",
            "F#5:3 E5:1 D5 A4 D5 E5 F#5:3 E5:1 D5 A4 D5 E5 | \
             F#5:2 A5:1 F#5:2 A5:1 F#5 E5 F#5:8 | \
             F#5:3 E5:1 D5 A4 D5 E5 F#5:3 E5:1 D5 A4 D5 E5 | \
             F#5:2 A5:1 F#5:2 E5:1 D5:2 B4:8",
            HUAYTA_F_CHORDS,
        ),
    ],
    form: &["A", "A", "B", "B", "C", "C into D", "E", "E end", "F", "F"],
    drums_from: 0,
    about: "A huayno in D major in six short strains, 2/4 written two bars to a bar. The first \
            ones swing between D and A7; the fourth ties an A over the bar line into its \
            downbeat, a syncopation the huayno loves; the fifth bends to F sharp 7 and \
            lands on B minor, the relative minor, and the last strain, high on F sharp \
            and A, ends there too.",
};

const KICHARI_A_CHORDS: &str = "i | bIII";
const KICHARI_B_CHORDS: &str = "bIII i | bIII";

/// The third strain twice: five bars of 2/4 each time, so the second time
/// starts in the middle of a bar here.
const KICHARI_C_CHORDS: &str = "i | i bVII | bIII i | i | bVII bIII";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/norbeck/abc_tune_norbeck_2994.abc
// (Henrik Norbeck's wayñu collection, tune 30, "Kicharirillaway" / "Ábreme la puerta
// vidita", O: Bolivia, D: Los Pachecos, Sumaq Raymi, Los Incas; D minor, 2/4, three
// strains, each repeated)
// Chords: the source has none; these are the plain chords of the style (i, bIII, bVII),
// chosen to fit the melody.
// PD: traditional (Norbeck names no composer)
pub(super) const KICHARIRILLAWAY: Song = Song {
    name: "Kicharirillaway",
    origin: "Traditional huayno, Bolivia",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: D,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Huayno",
    band: ANDEAN,
    style: "Huayno",
    parts: &[
        Part::new(
            "A",
            "D4:1 E4 F4 G4 A4 G4:2 A4:1 D5:6 C5:2 | A4:2 A4:1 A4:2 F4:1 G4:2 F4:8",
            KICHARI_A_CHORDS,
        ),
        Part::new(
            "B",
            "F5:6 E5:2 D5:2 C5:1 D5:3 C5:2 | A4:2 A4:1 A4:2 F4:1 G4:2 F4:8",
            KICHARI_B_CHORDS,
        ),
        // The second ending ends with the pickup, C and D, into the third strain.
        Part::new(
            "B into C",
            "F5:6 E5:2 D5:2 C5:1 D5:3 C5:2 | A4:2 A4:1 A4:2 F4:1 G4:2 F4:4 C5:2 D5",
            KICHARI_B_CHORDS,
        ),
        Part::new(
            "C",
            "A4:3 C5:2 C5:1 D5:2 A4:4 D4:1 E4 F4 G4 | A4:3 C5:2 C5:1 D5:2 A4:3 A4:1 G4:2 G4 | \
             F4:4 C5:2 D5 A4:3 C5:2 C5:1 D5:2 | A4:4 D4:1 E4 F4 G4 A4:3 C5:2 C5:1 D5:2 | \
             A4:3 A4:1 G4:2 G4 F4:8",
            KICHARI_C_CHORDS,
        ),
    ],
    form: &["A", "A", "B", "B into C", "C"],
    drums_from: 0,
    about: "\"Open the door, dear, I am no thief\": a Bolivian huayno in D minor whose tune \
            has no C sharp, so no dominant chord. Every strain comes to rest on F, the relative \
            major, the way Andean tunes hover between a minor key and its major; the third \
            strain is five bars of 2/4 long, so the second time round it starts halfway \
            through a bar here.",
};

const TITICACA_CHORDS: &str = "i | i bVII i | i | i bVII i | bIII bIII i | i | bIII bIII i | i";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/norbeck/abc_tune_norbeck_2983.abc
// (Henrik Norbeck's wayñu collection, tune 19, "A las orillas del Titicaca", O: Peru/Bolivia,
// D: Los Incas, in E minor and 3/4; eight bars, each line of the words sung twice, and
// four verses)
// Chords: the source has none; these are the plain chords of the style (i, bIII, bVII),
// chosen to fit the melody.
// PD: traditional (Norbeck names no composer)
pub(super) const A_LAS_ORILLAS_DEL_TITICACA: Song = Song {
    name: "A las orillas del Titicaca",
    origin: "Traditional huayno, Peru and Bolivia",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: E,
    mode: NaturalMinor,
    time: THREE_FOUR,
    tempo: 90.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Vidala",
    band: ANDEAN,
    style: "Vidala",
    parts: &[Part::new(
        "Verse",
        "B4:1 B4:2 B4:1 E5:4 G5 | E5:1 D5:2 B4:1 D5:4 E5 | G4:1 G4:2 G4:1 E5:4 G5 | \
         E5:1 D5:2 B4:1 D5:4 E5 | G4:1 A4:2 B4:1 D5:4 E5 | B4:1 A4:2 G4:1 E4:4 G4 | \
         G4:1 A4:2 B4:1 D5:4 E5 | B4:1 A4:2 G4:1 E4:4 E4",
        TITICACA_CHORDS,
    )],
    form: &["Verse", "Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "\"On the shores of Lake Titicaca I wrote your name in the sand\": each line of \
            the words is sung twice, the first one from B and then from G. The tune leans on \
            E, G, B and D, the notes E minor and G major share, and its D is never sharpened, \
            so there is no dominant: D and G turn around E minor instead.",
};

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/norbeck/abc_tune_norbeck_2986.abc
// (Henrik Norbeck's wayñu collection, tune 22, "Chunguinada", O: Peru, in E minor, 2/4:
// a half-bar pickup, a first strain repeated with first and second endings, and a
// second strain that ends on the half bar the pickup took)
// Chords: the source has none; these are the plain chords of the style (i, bIII, bVII),
// chosen to fit the melody.
// PD: traditional (Norbeck names no composer)
pub(super) const CHUNGUINADA: Song = Song {
    name: "Chunguinada",
    origin: "Traditional, Peru",
    feel: "Huayno",
    genre: Genre::Folklore,
    tonic: E,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Huayno",
    band: ANDEAN,
    style: "Huayno",
    parts: &[
        Part::new("Pickup", "-:12 E4:1 G4:2 B4:1", "i"),
        // The first strain's repeat comes back in the middle of a bar here (its first
        // ending is a bar and a half, the pickup completing it), and the last bar is
        // padded with a rest after the final half bar.
        Part::new(
            "Tune",
            "E5:6 G5:2 E5:2 D5 D5 B4 | D5:1 E5:2 G5:1 E5:2 D5 B4:8 | \
             _:4 E4:1 G4:2 B4:1 E5:6 G5:2 | E5:2 D5 D5 B4 D5:1 E5:2 G5:1 E5:2 D5 | \
             B4:8 E5:2 E5 E5 G5 | E5:2 D5 D5 B4 D5:1 E5:2 G5:1 E5:2 D5 | \
             B4:5 A4:1 B4 G4 D4:6 A4:1 B4 | G4:4 E4:8 -:4",
            "i | bIII i | i | i bIII | i | i bIII | bIII bVII | i",
        ),
    ],
    form: &["Pickup", "Tune"],
    drums_from: 1,
    about: "A Peruvian huayno in E minor and 2/4, two bars to a bar here, that starts on a \
            half-bar pickup. Its phrases climb from E to G and fall back to B, the fifth, and \
            only the very last one comes down through G and D to the tonic.",
};

/// The first strain: G and D7, every phrase leaning on a chromatic neighbour.
const JESUSITA_A_TUNE: &str = "B4:4 C5:2 E5 D5:4 C#5:2 D5 | G5:3 D5:1 C#5:2 D5 F#5 E5 D#5 E5 | \
                               A5:4 G5:2 E5 G5:4 F#5:2 C5 | F#5:3 C5:1 B4:2 C5 E5 D5 Db5 C5 | \
                               B4:4 C5:2 E5 D5 C#5:1 D5 E5 F#5 G5 A5 | B5:2 G5 F#5 G5 A5 E5 D#5 E5 | \
                               A5 G5 F#5 E5 G5 D5 C#5 D5 | E5 G5 F#5 A5 G5:4 G5";

const JESUSITA_A_CHORDS: &str = "I | I V7 | V7 | V7 | I | I IV | IV I | II V7 I I";

/// The second strain, in D, the dominant.
const JESUSITA_B_CHORDS: &str = "V7 I | V7 I | V7 I | V7 I";

/// Its second ending slides down D, D flat, C to the B that starts the first strain.
const JESUSITA_B_END_CHORDS: &str = "V7 I | V7 I | V7 I | V7 V7 I I7";

/// The trio, in C, the subdominant, in long notes (thirds in the source, their top note here).
const JESUSITA_C_CHORDS: &str = "I | IV | V7 | I";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201292801.abc
// (John Chambers' ABC of 1999 in G, "Jesucita en Chihuahua", Quirino Mendoza y Cortés 1916;
// the same notes as his A major version, abc_tune_jc_201292798.abc, and as his "Mexican
// Mixer", abc_tune_jc_201292794.abc, which plays the trio four times; the thirds of the trio
// are given as their top note. In the second ending of the second strain the ABC writes
// D, D flat, C under the key of D, which makes the last note C sharp, the same pitch as the
// D flat; it is C natural here, as in the same descent at the end of the first strain's
// first half, under the D7 the ABC writes there.)
// PD: premiered on Christmas Day 1916 (Wikipedia, "Jesusita en Chihuahua") and recorded
// for Columbia by Trío Nava in July 1921 (Columbia matrix 87642, Discography
// of American Historical Recordings), out before 1930; no dated printed edition was found.
pub(super) const JESUSITA_EN_CHIHUAHUA: Song = Song {
    name: "Jesusita en Chihuahua",
    origin: "Quirino Mendoza y Cortés, 1916",
    feel: "Polka",
    genre: Genre::Folklore,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Backbeat,
    bass: Bass::RootFifth,
    rhythm: "Polca",
    band: ACCORDION_BAND,
    style: "Polca",
    parts: &[
        Part::new("A", JESUSITA_A_TUNE, JESUSITA_A_CHORDS),
        Part::new(
            "B",
            "A5:4 G5 F#5:2 E5:1 F#5 G5:2 F#5 | E5 D5:1 E5 F#5:2 E5 E5:1 D5 C#5 B4 A4:4 | \
             A5 G5 F#5:2 E5:1 F#5 G5:2 F#5 | E5 D5:1 E5 F#5:2 E5 D5 F#5 D5:4",
            JESUSITA_B_CHORDS,
        )
        .in_key(D, Major),
        Part::new(
            "B end",
            "A5:4 G5 F#5:2 E5:1 F#5 G5:2 F#5 | E5 D5:1 E5 F#5:2 E5 E5:1 D5 C#5 B4 A4:4 | \
             A5 G5 F#5:2 E5:1 F#5 G5:2 F#5 | E5 D5:1 E5 F#5:2 E5 D5 D5 Db5 C5",
            JESUSITA_B_END_CHORDS,
        )
        .in_key(D, Major),
        Part::new(
            "C",
            "G5:8 G5:2 E5 F5 G5 | A5:8 A5:2 F5 G5 A5 | B5:8 A5:4 B5 | A5 G5 G5:2 E5 F5 F#5",
            JESUSITA_C_CHORDS,
        )
        .in_key(C, Major),
        Part::new(
            "C end",
            "G5:8 G5:2 E5 F5 G5 | A5:8 A5:2 F5 G5 A5 | B5:8 A5:4 B5 | C6:8 C6:2 -:6",
            JESUSITA_C_CHORDS,
        )
        .in_key(C, Major),
        Part::new(
            "D",
            "D4:1 E4 F#4 G4 A4 B4 C5 D5 E5 D5 B4 G4 D4:2 G5 | \
             F#5:1 E5 C5 A4 F#4:2 F#5 E5:1 D5 B4 G4 D4:4",
            "V7 I | V7 I",
        ),
        Part::new(
            "D end",
            "D4:1 E4 F#4 G4 A4 B4 C5 D5 E5 D5 B4 G4 D4:2 G5 | \
             F#5:1 E5 D5 C5 B4 D5 C5 A4 G4:4 G4",
            "V7 I | V7 I",
        ),
    ],
    form: &[
        "A", "A", "B", "B end", "A", "C", "C end", "D", "D end", "D", "D end", "C", "C end", "A",
    ],
    drums_from: 0,
    about: "A Mexican polka from the years of the Revolution, in 2/4 with two of its bars \
            to each bar here, over the norteño oom-pah. The first strain answers G with D7, \
            every phrase leaning on a chromatic neighbour (C sharp to D, D sharp to E); the \
            second moves to D, the dominant, the trio to C, the subdominant, and the last \
            strain runs up the D7 scale before each return.",
};

/// The chorus, "Y por eso los grandes amores", with the pickup back into it.
const DE_COLORES_CHORUS_TUNE: &str = "D5:2 D5 D5 D5 E5:3 D5:1 | D5:2 C5:3 C5:1 C5:2 D5 C5 | \
                                      C5 Bb4 C5 D5 C5 Bb4 | A4:6 -:2 Bb4 C5";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201267508.abc
// (John Chambers' ABC of 2005, "De Colores [C]", Mexico, in 6/8 with a second voice in
// thirds below the tune; here up a fourth, in F. Checked against "De colores", Música
// Tradicional Asturiana (Tello & Tito, 2001) in Jack Campin's ABC,
// https://github.com/syntithenai/abc2book/blob/master/abcresources/misc/abc_tune_misc_0.abc,
// a habanera in 2/4 with the same notes except: the first two bars of the second line,
// which it sings a third higher, A G G F where Chambers has F E E D; the fourth bar of the
// third line, G B flat B flat B flat where Chambers has G A B flat (C E flat E flat E flat
// and C D E flat here); and the chorus's first ending, which it takes down to G through
// F sharp (a half cadence) where Chambers ends on E.)
// The ABC writes the verse once and repeats only the chorus, with endings "1-N" and "last";
// here the whole song is played twice, as for a second stanza, each time with both endings.
// PD: traditional.
pub(super) const DE_COLORES: Song = Song {
    name: "De colores",
    origin: "Traditional, Mexico",
    feel: "Canción",
    genre: Genre::Folklore,
    tonic: F,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 96.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    band: SONG_AND_GUITAR,
    style: "Six-eight ballad",
    parts: &[
        Part::new(
            "Verse",
            "C5:6 _:4 Bb4:2 | Bb4 A4:4 -:2 A4 Bb4 | C5 C5:3 C5:1 C5:2 Bb4 A4 | \
             C5 C5:3 C5:1 C5:2 Bb4 A4 | C5 Bb4:4 _ -:2 | Bb4:6 _:4 A4:2 | A4 G4:4 -:2 Bb4 C5 | \
             D5 D5:3 D5:1 D5:2 C5 Bb4 | D5 D5:3 D5:1 D5:2 C5 Bb4 | D5 C5:4 _ -:2 | \
             C5:6 _:4 Bb4:2 | Bb4 A4:4 -:2 A4 Bb4 | C5 C5 C5 C5 Bb4 A4 | \
             C5 C5:3 C5:1 C5:2 D5 Eb5 | D5:6 -:2 Bb4 C5",
            "I V7 | I | I | I | V7 | V7 | V7 | V7 | V7 | I | I V7 | I | I | I | IV",
        ),
        Part::new("Chorus", DE_COLORES_CHORUS_TUNE, "IV V7 | I vi | ii V7 | I"),
        Part::new(
            "Chorus end",
            "D5:2 D5 D5 D5 E5:3 D5:1 | D5:2 C5:3 C5:1 C5:2 D5 C5 | C5 Bb4 C5 D5 C5 E5 | F5:8 -:4",
            "IV V7 | I vi | ii V7 | I",
        ),
    ],
    form: &[
        "Verse",
        "Chorus",
        "Chorus end",
        "Verse",
        "Chorus",
        "Chorus end",
    ],
    drums_from: 0,
    about: "\"De colores, de colores se visten los campos en la primavera\": each line of \
            the verse is one figure sung again a step higher or lower, over F and C7 only, \
            until an E flat, the flat seventh, leads to B flat. The chorus, \
            \"Y por eso los grandes amores\", walks B flat, C7, F, D minor, G minor, C7 \
            home, in a gentle 6/8.",
};

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/folktunefinder/abc_tune_folktunefinder_6213.txt
// (John Chambers' ABC of 2006, "Over the Waves / Sobre las olas", the copy of his
// trillian.mit.edu/~jc/music/abc/Mexico/waltz/OverTheWaves_G.abc: four waltzes, in G, G, D
// and F, played AABBACCDDA. He writes the waltz in 6/8 at half value; here each 6/8 bar is
// two bars of 3/4, and the whole is up a minor third, in B flat, B flat, F and A flat, to
// lift the low first waltz; the notes named below are in his keys. Grace notes left out.
// The first two waltzes checked against the 3/4 "Over the Waves" (X:28, no transcriber
// named) of the "Waltz Jam (Fiddle Hell Online) 2020" tunebook among his files,
// https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201350397.abc,
// which holds the long notes where the 2006 ABC rests, plays the dotted figures even, ends
// the second waltz like the first (A C E | D) where the 2006 ABC goes down E D# E | C B A | G
// (the 2006 ending is used), and in the second waltz's tenth bar has E D C where the 2006
// ABC has E D C#: C natural here, under the D7.
// The third and fourth waltzes have only this one source.)
// PD: Juventino Rosas died in 1894; published 1888.
pub(super) const SOBRE_LAS_OLAS: Song = Song {
    name: "Sobre las olas",
    origin: "Juventino Rosas, 1888",
    feel: "Waltz",
    genre: Genre::Folklore,
    tonic: ASharp,
    mode: Major,
    time: THREE_FOUR,
    tempo: 176.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    band: SALON,
    style: "Waltz",
    parts: &[
        Part::new(
            "A1",
            "D4:12 | C#4:4 D4 F4 | Bb4:12 | _:4 A4 Bb4 | C5 Bb4 A4 | Bb4 D4 F4 | A4:12 | - | \
             Eb4 | D4:4 Eb4 F4 | A4:12 | _:4 Ab4 A4 | Bb4 A4 Ab4 | A4 Eb4 A4 | D4:12 | -",
            "I | I | I | I | I | I | V7 | V7 | ii | ii | V7 | V7 | V7 | V7 | I | I",
        ),
        Part::new(
            "A2",
            "D4:12 | C#4:4 D4 F4 | Bb4:12 | _:4 A4 Bb4 | C5 Bb4 A4 | Bb4 D4 F4 | G4:12 | - | \
             G4 | C5:4 Eb5 G5 | F5:12 | _:4 Eb5 D5 | C5 Bb4 A4 | G4 A4 C5 | Bb4:12",
            "I | I | I | I | I | I | IV | IV | ii | ii | I | I | V7 | V7 | I",
        ),
        // The last beat of the waltz, and the silent upbeat of the next.
        Part::new("A rest", "-:12", "I"),
        Part::new("Into B", "-:8 D5:4", "I"),
        Part::new(
            "B1",
            "D5:4 C#5 D5 | Bb5 A5:6 G5:2 | Bb5:12 | _:4 A5 G5 | D5 C#5 D5 | G5 F5:6 Eb5:2 | \
             A4:12 | - | A4:4 Ab4 A4 | G5 F5:6 Eb5:2 | G5:12 | _:4 F5 Eb5 | A4 Ab4 A4 | \
             Eb5 D5 C5 | Bb4:12 | -",
            "I | I | I | I | I | I | V7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 | I | I",
        ),
        Part::new(
            "B2",
            "D5:4 C#5 D5 | Bb5 A5:6 G5:2 | Bb5:12 | _:4 A5 G5 | D5:4 C#5 D5 | G5 F5:6 Eb5:2 | \
             G4:12 | - | G4:4 F#4 G4 | Eb5 D5 C5 | Bb4:12 | _:4 A4 Ab4 | A4 C5 Eb5 | G5:8 F5:4 | \
             Bb4:12",
            "I | I | I | I | VI7 | VI7 | ii | ii | ii | ii | I | I | V7 | V7 | I",
        ),
        // "D.C.": a fermata on the last rest, then the first waltz again.
        Part::new("B rest", "-:12", "I").with_tempo(&[fermata(0, 4)]),
        Part::new(
            "C1",
            "A4:8 C5:4 | _ D5:8 | E5 F5:4 | _ D5:8 | E5 F5:4 | _ D5:8 | E5:12 | - | \
             E5:8 F5:4 | _ F#5:8 | A5 G5:4 | _ F5:8 | E5:4 D5 D5 | _ E5:8 | C5:12 | -",
            "I | I | I | I | I | I | V7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 | I | I",
        )
        .in_key(F, Major),
        Part::new(
            "C2",
            "A4:8 C5:4 | _ D5:8 | E5 F5:4 | _ D5:8 | E5 F5:4 | _ D5:8 | E5:12 | - | \
             E5:8 G5:4 | _ C6:8 | C6 G5:4 | _ E5:8 | E5 D5:4 | _ G4:8 | C5:12",
            "I | I | I | I | I | I | V | V | V | V | V | V | II7 | II7 | V",
        )
        .in_key(F, Major),
        Part::new("C rest", "-:12", "V").in_key(F, Major),
        Part::new("Into D", "-:4 B4 C5", "V").in_key(F, Major),
        Part::new(
            "D1",
            "Ab5:12 | _:4 G5 Ab5 | Ab5:12 | _:4 B4 C5 | Ab5:12 | _:4 G5 Ab5 | Db5:12 | _:4 C5 Db5 | \
             G5:12 | _:4 F#5 G5 | G5:12 | _:4 C5 Db5 | G5:12 | _:4 F#5 G5 | C5:12 | _:4 B4 C5",
            "I | I | I | I | I | I | ii | ii | V7 | V7 | V7 | V7 | V7 | V7 | I | I",
        )
        .in_key(GSharp, Major),
        Part::new(
            "D2",
            "Ab5:12 | _:4 G5 Ab5 | Ab5:12 | _:4 B4 C5 | F5:12 | _:4 E5 F5 | F4:12 | _:4 E4 F4 | \
             F5:12 | _:4 Eb5 Db5 | C5:12 | _:4 Bb4 Ab4 | G4:12 | _:4 F4 G4 | Ab4:12",
            "I | I | I | I | VI7 | VI7 | ii | ii | ii | ii | I | I | V7 | V7 | I",
        )
        .in_key(GSharp, Major),
        Part::new("D again", "-:4 B4 C5", "I").in_key(GSharp, Major),
        Part::new("D rest", "-:12", "I").in_key(GSharp, Major),
    ],
    form: &[
        "A1", "A2", "A rest", "A1", "A2", "Into B", "B1", "B2", "Into B", "B1", "B2", "B rest",
        "A1", "A2", "A rest", "C1", "C2", "C rest", "C1", "C2", "Into D", "D1", "D2",
        "D again", "D1", "D2", "D rest", "A1", "A2",
    ],
    drums_from: 0,
    about: "Known in English as Over the Waves. The first waltz starts low, on the third \
            of B flat, and climbs out of a leading note (C sharp to D); the second \
            leaps to the high B flat and turns through G7 to C minor. The third, in F, is all \
            hemiola, three half notes across two bars, and ends on C, its dominant; the \
            fourth, in A flat, holds long notes, each turning on the semitone below it \
            (A flat G A flat, G F sharp G). \
            Then the first waltz once more.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/TraditionalLaGolondrina.abc
// (the Wikifonia lead sheet in C, with its chords; here up a fifth, in G. Its triplets are
// written 2 + 1 + 1 steps.) Checked against Joel Jacklich's cello quartet in C, 2/4,
// https://github.com/cellist/Lilypond-Sheet-Music/tree/master/Vlc,%20Vlc,%20Vlc,%20Vlc/Sevilla,%20Narciso%20Serradel/La%20Golondrina
// (the tune in the first cello, and in the third for the first half of the second part):
// the same notes, with some of the dotted rhythms even and a few passing notes more; in
// the first bar of the second part it has D C B A, D E F# G A where the lead sheet has
// D C A F#, D E F# G A. The lead sheet writes G (D here) for the eighth bar, where the
// first verse ends on a long G; the quartet, in C, holds C E G C there (the home chord),
// and the lead sheet itself writes the home chord at the end of the second verse, so I is
// used.
// PD: Narciso Serradell died in 1910; composed 1862.
pub(super) const LA_GOLONDRINA: Song = Song {
    name: "La golondrina",
    origin: "Narciso Serradell, 1862",
    feel: "Canción",
    genre: Genre::Folklore,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 66.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Habanera",
    band: SONG_AND_GUITAR,
    style: "Habanera",
    parts: &[
        Part::new("Pickup", "-:11 D4:1 G4:3 A4:1", "I"),
        // "A dónde irá veloz y fatigada la golondrina que de aquí se va".
        Part::new(
            "Verse",
            "B4:6 A4:2 B4:3 C5:1 D5:3 E5:1 | E5:4 _:1 D5 C5 B4 A4:3 D4:1 G4:3 A4:1 | \
             B4:6 D5:2 C#5 D5 E5 D5 | A4:6 C5:1 B4 A4:3 D4:1 G4:3 A4:1 | \
             B4:6 A4:2 G4 G5 G5 F#5 | G5:8 E5:2 E5 C5 A4 | B4:6 D5:2 C#5 D5 C5:3 A4:1 | \
             G4:8 _:3 D4:1 G4:3 A4:1",
            "I | V7 | I | V7 | I I I I+ | IV | I I I V7 | I",
        ),
        // The verse again, ending on G, with the pickup into the second part.
        Part::new(
            "Verse 2",
            "B4:6 A4:2 B4:3 C5:1 D5:3 E5:1 | E5:4 _:1 D5 C5 B4 A4:3 D4:1 G4:3 A4:1 | \
             B4:6 D5:2 C#5 D5 E5 D5 | A4:6 C5:1 B4 A4:3 D4:1 G4:3 A4:1 | \
             B4:6 A4:2 G4 G5 G5 F#5 | G5:8 E5:2 E5 C5 A4 | B4:6 D5:2 C#5 D5 C5:3 A4:1 | \
             G4:8 _:2 -:1 D5 E5:3 D5:1",
            "I | V7 | I | V7 | I I I I+ | IV | I I I V7 | I",
        ),
        Part::new(
            "Refrain",
            "D5:4 _:2 C5:1 A4 F#4:2 D5:1 E5 F#5:2 G5:1 A5 | B5:8 G5:3 B5:1 A5:3 G5:1 | \
             G5:4 F#5 _:2 E5:1 F#5 G5:2 F#5:1 E5 | D5:8 B4:2 -:1 D5 D5:3 C#5:1 | \
             D5:8 B4:2 B4:1 C5 D5:2 E5:1 F#5 | G5:8 E5:3 E5:1 C5:3 A4:1 | \
             B4:6 D5:2 C#5:3 D5:1 C5:3 A4:1 | G4:8 -",
            "V7 | I | V7 | I | I | IV | I I I V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse 2", "Refrain"],
    drums_from: 1,
    about: "Mexico's song of farewell. The verse sways between G and D7, turning around D \
            with a chromatic neighbour, C sharp; an augmented G chord pushes it on to C. The \
            second part falls down a D7 chord and runs up the scale to the high B, then \
            comes home by the verse's last line.",
};

const ESTRELLITA_A_TUNE: &str = "E4:4 F4:2 A4:1 C5:1 G5:6 F5:2 | F5:3 G4:1 G4:4 -:6 D5:2 | \
     D5:3 F#4:1 G4:3 A4:1 Bb4:2 -:4 D5:2 | D5:3 E5:1 C5:4 C5:2 C4:1 A4:2 -:2 A4:1 | \
     E4:4 F4:2 A4:1 C5:1 A5:4 -:2 E5:2 | G5:3 F5:1 G4:4 -:2 E5:1 G5:1 F5:1 E5:1 F5:1 D5:1 | \
     D5:3 C5:1 F5:3 A4:1 A4:2 Bb4:2 D5:3 E4:1 | F4:8 -:7 C4:1";
const ESTRELLITA_A_AGAIN_TUNE: &str = "E4:4 F4:2 A4:1 C5:1 G5:6 F5:2 | F5:3 G4:1 G4:4 -:6 D5:2 | \
     D5:3 F#4:1 G4:3 A4:1 Bb4:2 -:4 D5:2 | D5:3 E5:1 C5:4 C5:2 C4:1 A4:2 -:2 A4:1 | \
     E4:4 F4:2 A4:1 C5:1 A5:4 -:2 E5:2 | G5:3 F5:1 G4:4 -:2 E5:1 G5:1 F5:1 E5:1 F5:1 D5:1 | \
     D5:3 C5:1 F5:3 A4:1 A4:2 Bb4:2 D5:3 E4:1 | F4:8 -:4 Bb4:4";
// Cbdim in the source is B diminished (#iv°); Gbaug is D augmented (VI+).
const ESTRELLITA_A_CHORDS: &str =
    "Imaj7 iii | IVmaj7 ii | V7 ii | #iv° iii vi vi | vi I I III | IVmaj7 ii | I V7 | I";
const ESTRELLITA_A_AGAIN_CHORDS: &str =
    "Imaj7 iii | IVmaj7 ii | V7 ii | #iv° iii vi vi | vi I I III | IVmaj7 ii | I V7 | I IV";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/ManuelMariaPonceCuellarEstrellitaLittleStar.abc
// (the Wikifonia lead sheet in F, 2/4, with its chords; two of its bars make one bar here,
// and its triplets are written 2 + 1 + 1 steps.) Checked against the flute part of
// "Estrellita, Mexican love song, for flute and piano, Manuel Ponce arr. Jascha Heifetz",
// https://github.com/baobach/abc_search/blob/master/database/abc/Qma1SS6MKNcx9UjWGpZbX1JLMpidpoZegYkny5jV1zLiR1.abc
// (its first 24 bars, before the variation): the same notes, but it holds the long notes
// where the lead sheet has rests, plays some of the lead sheet's dotted and sixteenth
// figures as triplets, and in the fourth bar of each verse sings C once (C, low C, A) where
// the lead sheet repeats it (C, C, low C, A). Where the lead sheet has a two-note chord, C
// and B flat, as the upbeat of bar 2, the flute has D, as in bar 3, and that is followed
// here. The flute leads into the tune with C D where the lead sheet has a single C, and
// into bar 5 with C D where the lead sheet has A. In the sixth bar of the middle part
// (lead sheet bar 44, flute bar 23) the flute has A for the lead sheet's low G.
// The lead sheet's chord on the opening upbeat is unreadable ("CbminMaj7"); I is used.
// PD: every author died before 1956 (Manuel M. Ponce, 1882-1948).
pub(super) const ESTRELLITA: Song = Song {
    name: "Estrellita",
    origin: "Manuel M. Ponce, Mexico",
    feel: "Canción",
    genre: Genre::Folklore,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 52.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Bolero",
    band: SONG_AND_GUITAR,
    style: "Bolero",
    parts: &[
        Part::new("Pickup", "-:15 C4:1", "I"),
        Part::new("A", ESTRELLITA_A_TUNE, ESTRELLITA_A_CHORDS),
        Part::new(
            "A again",
            ESTRELLITA_A_AGAIN_TUNE,
            ESTRELLITA_A_AGAIN_CHORDS,
        ),
        Part::new(
            "B",
            "A4:2 F#4:1 G4:1 D5:8 C5:2 A4:2 | G4:2 A4:1 F4:1 C4:4 -:4 Bb4:4 | \
             A4:2 F#4:1 G4:1 Bb4:4 A4:6 G4:2 | Bb4:3 A4:1 F4:8 -:2 C4:2 | \
             E4:4 F4:2 A4:1 C5:1 A5:4 -:2 E5:2 | G5:3 F5:1 G4:4 -:2 E5:1 G5:1 F5:1 E5:1 F5:1 D5:1 | \
             D5:3 C5:1 F5:3 A4:1 A4:2 Bb4:2 D5:3 E4:1 | F4:16",
            "VI7 ii V7 V7 | V7 I I IV | VI+ VI+ iii V7 | I V | I iii | vii° vi V V | I V7 | I",
        ),
    ],
    form: &["Pickup", "A", "A again", "B"],
    drums_from: 1,
    about: "A slow song by Manuel M. Ponce. Each line starts low, on E, climbs an F major chord \
            and leaps up to the high G or A. The middle part turns to G minor through D7, \
            with F sharp as a leading note, before the last line of the tune comes back.",
};

const JARABE_A_TUNE: &str = "G5:2 E5 D#5 E5 C5 B4 | C5:2 G4 -:4 E4:2 F4 | G4:2 A4 B4 C5 D5 E5 | \
     F5:2 D5 -:4 F5:2 E5 | F5:2 D5 C#5 D5 B4 A#4 | B4:2 G4 -:4 G5:2 G5 | \
     G5:2 A5 G5 F5 E5 D5 | E5:6 _:2 G5:2 F#5";
const JARABE_A2_TUNE: &str = "G5:2 E5 D#5 E5 C5 B4 | C5:2 G4 -:4 E4:2 F4 | G4:2 A4 B4 C5 D5 E5 | \
     F5:2 D5 -:4 F5:2 E5 | F5:2 D5 C#5 D5 B4 A#4 | B4:2 G4 -:4 G5:2 G5 | \
     G5:2 A5 G5 F5 E5 D5 | C5:6 _:2 D5:2 D5";
const JARABE_A_END_TUNE: &str = "G5:2 E5 D#5 E5 C5 B4 | C5:2 G4 -:4 E4:2 F4 | G4:2 A4 B4 C5 D5 E5 | \
     F5:2 D5 -:4 F5:2 E5 | F5:2 D5 C#5 D5 B4 A#4 | B4:2 G4 -:4 G5:2 G5 | \
     G5:2 A5 G5 F5 E5 D5 | C5:8 -:4";
const JARABE_A_CHORDS: &str = "I | I | I | V7 | V7 | V7 | V7 | I";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/UnknownMEXICANHATDANCE.abc
// ("Mexican hat dance", arr. H. Laukens, a lead sheet in F, 6/8, with its chords; here down
// a fourth, in C). Only the two traditional strains are taken: the lead sheet's own
// introduction (a chromatic run down from F) and its coda are left out, and the tune ends
// on the first strain's own last bar. The lead sheet plays the first eight bars of the
// first strain partly an octave lower; here they are at the octave of the second eight,
// as in the quartet below, and the pickup out of the second strain is moved up an octave
// to match. Checked against two cellist quartets,
// https://github.com/cellist/Lilypond-Sheet-Music/tree/master/Vl,%20Vl,%20Vla,%20Vlc/Traditional/Mexican%20Hat%20Dance
// (Kenneth W. Abeling's arrangement in C, first violin) and
// https://github.com/cellist/Lilypond-Sheet-Music/tree/master/Vlc,%20Vlc,%20Vlc,%20Vlc/Traditional/El%20Jarabe%20Tapatio
// (Tim Curd's arrangement in G, first cello): both have the first strain with the same
// notes, holding the long notes where the lead sheet has rests (the gaps for the dancers'
// stamps are kept here). Abeling's has the second strain with the lead sheet's notes;
// in Curd's each of its lines starts 5 2 2 2 3 4 (D A A A B C here) instead of 5 2 2 2 4 4.
// The lead sheet's D.S. al Coda plays the first strain again up to its fourteenth bar and
// then its own coda; here the strain is played to its last bar instead (C held for a
// dotted quarter and an eighth, as in the strain's second ending).
// PD: traditional.
pub(super) const JARABE_TAPATIO: Song = Song {
    name: "Jarabe tapatío",
    origin: "Traditional, Mexico",
    feel: "Jarabe",
    genre: Genre::Folklore,
    tonic: C,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 180.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Son jalisciense",
    band: MARIACHI,
    style: "Son jalisciense",
    parts: &[
        Part::new("Pickup", "-:8 G5:2 F#5", "I"),
        Part::new("A", JARABE_A_TUNE, JARABE_A_CHORDS),
        Part::new("A2", JARABE_A2_TUNE, JARABE_A_CHORDS),
        Part::new(
            "B",
            "D5:2 A4 A4 A4 C5 C5 | C5:2 B4:4 _:2 D5:2 D5 | D5:2 A4 A4 A4 C5 C5 | \
             B4:6 _:2 D5:2 D5 | D5:2 A4 A4 A4 C5 C5 | C5:2 B4:4 _:2 D5:2 D5 | \
             D5:2 E5 D5 C5 B4 A4 | G4:6 _:2 G5:2 F#5",
            "V7 | I | V7 | I | V7 | I | V7 | I",
        )
        .in_key(G, Major),
        Part::new("A end", JARABE_A_END_TUNE, JARABE_A_CHORDS),
    ],
    form: &["Pickup", "A", "A2", "B", "A", "A end"],
    drums_from: 1,
    about: "The hat dance. The tune falls a third, circles back up through a chromatic \
            neighbour (D sharp to E), and stops short: the gaps are for the dancers' stamps. \
            The second strain moves to G, the dominant, and bounces between D7 and G before \
            the first strain comes back.",
};

const LA_RASPA_A_TUNE: &str = "D5:4 A4:2 D5:4 A4:2 | D5:2 -:8 A4:2 | D5:2 E5 D5 C#5:4 D5:2 | \
     E5:2 -:8 A4:2 | C#5:4 A4:2 C#5:4 A4:2 | C#5:2 -:8 A4:2 | C#5:2 D5 C#5 B4:4 C#5:2 | \
     D5:2 -:8 A4:2";
const LA_RASPA_A_TO_B_TUNE: &str = "D5:4 A4:2 D5:4 A4:2 | D5:2 -:8 A4:2 | D5:2 E5 D5 C#5:4 D5:2 | \
     E5:2 -:8 A4:2 | C#5:4 A4:2 C#5:4 A4:2 | C#5:2 -:8 A4:2 | C#5:2 D5 C#5 B4:4 C#5:2 | \
     D5:2 -:4 F#5:4 A5:2";
const LA_RASPA_A_TO_C_TUNE: &str = "D5:4 A4:2 D5:4 A4:2 | D5:2 -:8 A4:2 | D5:2 E5 D5 C#5:4 D5:2 | \
     E5:2 -:8 A4:2 | C#5:4 A4:2 C#5:4 A4:2 | C#5:2 -:8 A4:2 | C#5:2 D5 C#5 B4:4 C#5:2 | \
     D5:2 -:6 A5:2 G#5";
const LA_RASPA_A_CHORDS: &str = "I | I | I | V7 | V7 | V7 | V7 | I";
const LA_RASPA_B_TUNE: &str = "D5:2 D5 D5 D5 C#5 D5 | E5:2 -:4 E5:4 G5:2 | C#5:2 C#5 C#5 C#5 B4 C#5 | \
     D5:2 -:4 F#5:4 A5:2 | D5:2 D5 D5 D5 C#5 D5 | E5:2 -:4 E5:4 G5:2 | \
     C#5:2 C#5 C#5 C#5 B4 C#5 | D5:2 -:4 F#5:4 A5:2";
const LA_RASPA_B_TO_A_TUNE: &str = "D5:2 D5 D5 D5 C#5 D5 | E5:2 -:4 E5:4 G5:2 | C#5:2 C#5 C#5 C#5 B4 C#5 | \
     D5:2 -:4 F#5:4 A5:2 | D5:2 D5 D5 D5 C#5 D5 | E5:2 -:4 E5:4 G5:2 | \
     C#5:2 C#5 C#5 C#5 B4 C#5 | D5:2 -:8 A4:2";
const LA_RASPA_B_CHORDS: &str = "I | V7 | V7 | I | I | V7 | V7 | I";
const LA_RASPA_C_TUNE: &str = "A5:2 F#5 F5 F#5 D5 C#5 | D5:4 A4:2 _:2 F#4:2 G4 | A4:2 B4 C#5 D5 E5 F#5 | \
     G5:4 E5:2 _:2 G5:2 F#5 | G5:2 E5 D#5 E5 C#5 C5 | C#5:4 A4:2 _:2 A5:2 G#5 | \
     A5:2 B5 A5 G5 F#5 E5 | D5:2 -:6 A5:2 G#5";
const LA_RASPA_C_END_TUNE: &str = "A5:2 F#5 F5 F#5 D5 C#5 | D5:4 A4:2 _:2 F#4:2 G4 | A4:2 B4 C#5 D5 E5 F#5 | \
     G5:4 E5:2 _:2 G5:2 F#5 | G5:2 E5 D#5 E5 C#5 C5 | C#5:4 A4:2 _:2 A5:2 G#5 | \
     A5:2 B5 A5 G5 F#5 E5 | D5:2 -:10";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/folktunefinder/abc_tune_folktunefinder_5780.txt
// (John Chambers' "La Raspa" in D, 6/8, with its chords; from
// http://trillian.mit.edu/~jc/music/abc/Mexico/Raspa.abc). The copy in
// abc_tune_folktunefinder_7571.txt (from .../Mexico/LaRaspa.abc) is the same transcription;
// no independent second source was found. The source's form is A A B B A A C C, as here.
// The pickup into part C is two eighths where the bar before it leaves room for three, so
// an eighth rest is added before it. The source puts a fermata on the last note of the
// first A, which is not played here, since the form goes on. E sharp and B sharp in part
// C are written F and C.
// PD: traditional.
pub(super) const LA_RASPA: Song = Song {
    name: "La raspa",
    origin: "Traditional, Mexico",
    feel: "Raspa",
    genre: Genre::Folklore,
    tonic: D,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 168.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Son jalisciense",
    band: MARIACHI,
    style: "Son jalisciense",
    parts: &[
        Part::new("Pickup", "-:10 A4:2", "I"),
        Part::new("A", LA_RASPA_A_TUNE, LA_RASPA_A_CHORDS),
        Part::new("A to B", LA_RASPA_A_TO_B_TUNE, LA_RASPA_A_CHORDS),
        Part::new("B", LA_RASPA_B_TUNE, LA_RASPA_B_CHORDS),
        Part::new("B to A", LA_RASPA_B_TO_A_TUNE, LA_RASPA_B_CHORDS),
        Part::new("A to C", LA_RASPA_A_TO_C_TUNE, LA_RASPA_A_CHORDS),
        Part::new("C", LA_RASPA_C_TUNE, LA_RASPA_A_CHORDS),
        Part::new("C end", LA_RASPA_C_END_TUNE, LA_RASPA_A_CHORDS),
    ],
    form: &[
        "Pickup", "A", "A to B", "B", "B to A", "A", "A to C", "C", "C end",
    ],
    drums_from: 1,
    about: "Only two chords, D and A7. The first strain answers each short call with a \
            silence, the second hammers repeated notes, and the third runs up the scale, \
            leaning on chromatic neighbours a semitone below (D sharp to E, G sharp to A) before \
            it comes home.",
};

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/misc/abc_tune_misc_0.abc, byte 108772
// (Jack Campin's Nine-Note Tunebook, "La Bamba", "folksong from Veracruz, Mexico", in G,
// 4/4, a quarter = 150, no chords; here up a fourth, in C). Checked against the Wikifonia
// lead sheet of Ritchie Valens' version in C,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/RitchieValensLaBamba.abc
// whose bars line up with Campin's one for one: the same tune, with straighter rhythms
// (Campin ties many notes over from the half bar before), but in bars 11-14 and 17-20
// ("y arriba y arriba", "soy capitán") Valens sings other notes; Campin's are followed.
// The chords are Valens' sheet's: one loop, V7 | I IV, all through (V in the first bar).
// The last four bars of Campin's "bamba, bamba" vary the lead sheet's two-bar repeat.
// PD: traditional.
pub(super) const LA_BAMBA: Song = Song {
    name: "La bamba",
    origin: "Traditional, Veracruz, Mexico",
    feel: "Son jarocho",
    genre: Genre::Folklore,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 150.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Folk",
    band: LLANERO,
    style: "Folk",
    parts: &[
        // "Para bailar la bamba, se necesita una poca de gracia".
        Part::new(
            "Verse",
            "-:2 F5:2 F5 F5 F5:4 F5:2 E5 | _:2 C5:2 _:4 -:8 | -:4 F5:2 F5 F5 F5 F5 F5 | \
             E5:2 C5:4 C5:2 C5:4 C5:2 D5 | _:4 B4:2 D5 F5 E5 D5 E5 | _:2 C5 -:4 -:8 | \
             -:4 F5:2 F5 F5 F5 F5 F5 | E5:2 C5 C5 C5 C5:4 A4:2 D5 | \
             D5:4 B4:2 D5 F5 E5 D5 E5 | _:2 C5 -:4 -:8",
            "V | I IV | V7 | I IV | V7 | I IV | V7 | I IV | V7 | I IV",
        ),
        // "Y arriba y arriba ... yo no soy marinero, soy capitán".
        Part::new(
            "Verse 2",
            "-:4 D5:2 E5 F5 E5 D5 E5 | _:2 C5 -:2 C5:2 C5:4 A4:2 D5 | \
             _:4 B4:2 D5 F5 E5 D5 C5 | _:4 C5:4 A4:4 A4:2 G4 | -:2 F5 F5 F5 F5:4 F5:2 E5 | \
             _:2 C5 -:4 -:8 | -:2 F5 F5 F5 F5:4 F5:2 E5 | _:2 C5:4 A4:2 _:2 C5:4 A4:2 | \
             D5:4 -:2 B4 D5:4 F5:2 E5 | _:4 -:2 C5 C5:4 A4:2 G4 | _:4 -:4 -:8",
            "V7 | I IV | V7 | I IV | V7 | I IV | V7 | I IV | V7 | I IV | V7",
        ),
        // "Bamba, bamba".
        Part::new(
            "Chorus",
            "C5:8 A4:6 B4:2 | _:2 G4:2 -:4 -:8 | E5:8 C5:6 D5:2 | _:2 D5:2 -:4 -:8 | \
             E5:8 C5:6 D5:2 | _:2 D5:2 -:4 -:8 | C5:8 A4:6 B4:2 | _:2 G4:2 -:4 -:8",
            "I IV | V7 | I IV | V7 | I IV | V7 | I IV | V7",
        ),
    ],
    form: &["Verse", "Verse 2", "Chorus"],
    drums_from: 0,
    about: "Three chords, C, F and G7, going round without stopping, as a son jarocho does. \
            The singer starts most lines on F, the seventh of G7, and pushes many notes in \
            early, tied over from the half bar before.",
};

const LA_CUCARACHA_CHORDS: &str = "I | I | I | V7 | V7 | V7 | V7 | I";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/TraditionalMexicanLacucaracha.abc
// (the Wikifonia lead sheet, "Traditional Mexican", in G, with its chords and two Spanish
// verses; here up a fourth, in C). Checked against two shorter versions that have only the
// first half of the verse and the refrain once, repeated:
// https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201358365.abc ("La cucaracha",
// anon., in D, from http://abc.musicaviva.com/tunes/anon/la-cucaracha/la-cucaracha-1.abc)
// and https://github.com/syntithenai/abc2book/blob/master/abcresources/misc/abc_tune_misc_0.abc, byte 104270 (Jack
// Campin's Nine-Note Tunebook, "Mexican satirical song, late 19th century", in F). Both
// agree with the lead sheet's notes except: in bar 3 they have G A G F E G where the lead
// sheet has E G F E D E (the lead sheet is followed); they end the first line of the verse
// and of the refrain on C alone where it has E C; and several upbeats start an eighth
// earlier. The second half of the verse ("Cuando oigo la cucaracha", bars 9-16) is only
// in the lead sheet.
// PD: traditional.
pub(super) const LA_CUCARACHA: Song = Song {
    name: "La cucaracha",
    origin: "Traditional, Mexico",
    feel: "Corrido",
    genre: Genre::Folklore,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 144.0,
    comping: Comping::Backbeat,
    bass: Bass::RootFifth,
    rhythm: "Polca",
    band: MARIACHI,
    style: "Polca",
    parts: &[
        // "Ya llegó la cucaracha ... Cuando oigo la cucaracha".
        Part::new(
            "Verse",
            "-:4 G4:2 G4 C5 C5 E5 E5 | G5:6 E5:2 _:4 -:4 | -:2 E5:4 G5:2 F5 E5 D5 E5 | \
             F5:6 D5:2 _:4 -:4 | -:4 G4:2 G4 B4 B4 D5 D5 | F5:6 D5:2 _:4 -:4 | \
             -:2 G5:4 A5:2 G5 F5 E5 D5 | E5:6 C5:2 _:4 -:4 | -:4 G4:2 G4 C5 C5 E5 E5 | \
             G5:16 | _:2 G5:4 G5:2 G5 F5 E5 G5 | F5:6 D5:2 _:4 -:4 | \
             G4:2 A4:4 B4:2 D5:4 E5:4 | F5:4 G5:12 | _:2 G5:4 A5:2 G5 F5 E5 D5 | \
             C5:8 -:2 G4 G4 G4",
            "I | I | I | V7 | V7 | V7 | V7 | I | I | I | I | V7 | V7 | V7 | V7 | I",
        ),
        // "La cucaracha, la cucaracha, ya no quiere caminar".
        Part::new(
            "Chorus",
            "C5:6 E5:2 _:2 G4 G4 G4 | C5:6 E5:2 _:4 -:4 | -:4 C5:2 C5 B4 B4 A4 A4 | \
             G4:8 -:2 G4 G4 G4 | B4:6 D5:2 _:2 G4 G4 G4 | B4:6 D5:2 _:4 -:4 | \
             -:4 G5:2 A5 G5 F5 E5 D5 | E5:6 C5:2 _:2 G4 G4 G4",
            LA_CUCARACHA_CHORDS,
        ),
        Part::new(
            "Chorus again",
            "C5:6 E5:2 _:2 G4 G4 G4 | C5:6 E5:2 _:4 -:4 | -:4 C5:2 C5 B4 B4 A4 A4 | \
             G4:8 _:2 G4 G4 G4 | B4:6 D5:2 _:2 G4 G4 G4 | B4:6 D5:2 _:4 -:4 | \
             -:4 G5:2 A5 G5 F5 E5 D5 | C5:12 -:4",
            LA_CUCARACHA_CHORDS,
        ),
        Part::new(
            "Chorus end",
            "C5:6 E5:2 _:2 G4 G4 G4 | C5:6 E5:2 _:4 -:4 | -:4 C5:2 C5 B4 B4 A4 A4 | \
             G4:8 _:2 G4 G4 G4 | B4:6 D5:2 _:2 G4 G4 G4 | B4:6 D5:2 _:4 -:4 | \
             -:4 G5:2 A5 G5 F5 E5 D5 | C5:16 | _:4 -:12",
            "I | I | I | V7 | V7 | V7 | V7 | I | I",
        ),
    ],
    form: &[
        "Verse",
        "Chorus",
        "Chorus again",
        "Verse",
        "Chorus",
        "Chorus end",
    ],
    drums_from: 0,
    about: "Two chords, C and G7, and a tune that starts almost every line after the beat. \
            The refrain hammers three quick upbeats into each long note: la cu-ca-RA-cha.",
};

// Source: https://github.com/baobach/abc_search/blob/master/database/abc/QmYRNNHEUn3KC8ecx2Wz1v88QYmjLjMHntZVTPMSNEh2rB.abc
// (abc_search database: "Diana triste, vals para piano", Luis Antonio Calvo, transcribed by Juan Carlos Moreno 2014-2015;
// in B minor, Lento in B major). Melody = the right hand's main voice (voice 1 of the ABC, its
// top notes), chords read from the whole piano texture; everything transposed down a fourth to
// F# minor for range. In the first bar of the second ending another right-hand voice strikes
// the chord again an octave higher under a fermata; that is left to the accompaniment.
// PD: Luis Antonio Calvo died in 1945, before 1956.
const DIANA_INTRO_TUNE: &str = "-:2 C#4:2 D4 C#4 B#3 C#4 | C#5:2 C#4 B#3 A4 G#4:4 | _:2 C#4 D4 C#4 B#3 C#4 | G#4:2 C#4 B3 G#4 F#4:4 | \
    _:2 A4 B4 A4 G#4 A4 | D5:2 E4 F#4 G#4 A4 E4 | D#4:2 C#5 A4 F#5 B4 B5 | A5:1 G#5 A5 G#5 F#5:2 E5 D#5 D5 | \
    C#5:1 B4 C#5 B4 A4:2 F#4 E4 D4 | -:2 C#4 D4 C#4 B#3 C#4 | C#5:2 C#4 B#3 A4 G#4:4 | _:2 C#4 D4 C#4 B#3 C#4 | \
    G#4:2 C#4 B3 G#4 F#4:4 | -:2 F#4 G#4 F#4 E#4 F#4 | A4:2 E#4 F#4 A4 C#5 F#5 | A5:2 F#5 A5:4 G#5 | \
    F#5:4 -:6 C#4:2";
const DIANA_INTRO_CHORDS: &str = "i | II7 | V7 | i V7 i | bVI bVI vii°7 | bIII bIII #iv°7 | IV9 | bVII7 | bIII | i | II7 | V7 | i V7 i | bVI bVI vii°7 | V+ i i | bVI7 bVI7 V7 | i";
const DIANA_A_TUNE: &str = "B4:2 A#4 B4 C#5 -:2 G#4 | B4:2 A4 G#4 F#4 -:2 C#4 | D4:2 C#4 D4 F#4 -:2 D4 | C#4:8 -:2 C#4 | \
    D4:2 C#4 D4 E4 -:2 F#4 | G#4:2 A4 B4 C#5 -:2 C#5 | D5:2 C#5 D5 F#5:4 D5:2 | C#5:8 -:2 C#5 | \
    D5:2 C#5 B#4 C#5 F#5 F#4 | C#5:2 B4 A#4 B4:4 B4:2 | C#5:2 B4 A#4 B4 E5 E4 | B4:2 A4 G#4 A4:4 A4:2 | \
    A4:2 G#4 G4 G#4 C#5 C#4 | B4:2 A4 G#4 A4 C#5 C#4";
const DIANA_A_CHORDS: &str = "V7 | i | iv6 | V7 | bVII7 | bIII bIII iii°7 | iv6 | V | I7 I7 IV7 | bVII | bVII7 bVII7 bIII7 | bVI | V7 | i";
const DIANA_A2_TUNE: &str = "C#5:2 B4 A#4 B4 C#5 D5 | C#5:2 F#5 F#4 C#5 F#5 F#4 | C#5:2 F#5 F#4 C#5 F#5 F#4 | D5:2 F#5 F#4 B4 F#5 F#4 | \
    A4:2 A5 A4 D5 A5 F#5 | C#6:8 -:2 C#5 | B5:2 A#5 B5 C#6:4 G#5:2 | B5:2 A5 G#5 F#5:4 C#5:2 | \
    E5:2 D5 E5 F#5:4 D5:2 | C#5:4 C#5 -:2 F#4 | E5:2 D#5 E5 F#5:4 C#5:2 | E5:2 D5 C#5 B4:4 F#4:2 | \
    A4:2 G#4 F#4 A4:4 G#4:2 | F#4:8 -:2 C#4";
const DIANA_A2_CHORDS: &str =
    "iiø7 | i | I7 | iv | bVI7 | V | V7 | i | iv iv iv6 | V I7 I7 | I7 | iv | V #iv°7 V7 | i";
const DIANA_B_TUNE: &str = "A#4:4 A#4 B4 | C#5:4 C#5 D#5 | E#5:2 D#5 C#5 B4 A#4 B4 | G#4:10 C#4:2 | \
    F#4:4 F#4 G#4 | A#4:4 A#4 A4 | G#4:4 D#4:2 E#4 F#4 G#4 | F#4:2 E#4 D#4 E#4 C#4 C#4 | \
    A#4:4 A#4 B4 | C#5:4 C#5 D#5 | E5:4 D#5:2 C#5 B4 A#4 | C#5:4 B4:2 A#4 G#4 D#4 | \
    F#4:4 F#4 G#4 | A#4:4 B4 C#5";
const DIANA_B_CHORDS: &str = "I I IV | I I IV7 | V7 | V7 | I I iii7 | I vi viø7 | II7 | V7 | I I IV | I I IV7 | bVII I7 I7 | IV | IV iv iv6 | I IV I";

pub(super) const DIANA_TRISTE: Song = Song {
    name: "Diana triste",
    origin: "Luis Antonio Calvo",
    feel: "Vals",
    genre: Genre::Folklore,
    tonic: FSharp,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 90.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    band: PIANO,
    style: "Vals in minor",
    parts: &[
        // "Ad libitum"; "Ligero" in bar 8, "a tempo" in bar 10. Bars 8 and 9 open with an
        // eighth and a sixteenth-note triplet (a turn), 4 notes in 4 steps: with no
        // triplets, they are written as four sixteenths, the first on the beat.
        Part::new("Intro", DIANA_INTRO_TUNE, DIANA_INTRO_CHORDS)
            .with_tempo(&[tempo(86, 100), tempo(110, 90)]),
        // "Gracioso", with a fermata in bar 7.
        Part::new("A", DIANA_A_TUNE, DIANA_A_CHORDS).with_tempo(&[tempo(0, 80), fermata(78, 4)]),
        // "Accelerando" over the rising bass: the score's metronome marks go 90, 100, 110,
        // 120, 130 and 140, one a bar, then "a tempo" (90) before the last eighth of bar 6.
        Part::new("A2", DIANA_A2_TUNE, DIANA_A2_CHORDS).with_tempo(&[
            tempo(0, 90),
            tempo(12, 100),
            tempo(24, 110),
            tempo(36, 120),
            tempo(48, 130),
            tempo(60, 140),
            tempo(70, 90),
        ]),
        Part::new("B", DIANA_B_TUNE, DIANA_B_CHORDS)
            .in_key(FSharp, Major)
            .with_tempo(&[tempo(0, 70)]),
        // Fermata, then "molto rallentando" 60, 50, 40. The augmented chord of bar 4 is
        // held, its raised fifth tied, through bar 5.
        Part::new(
            "B ending 1",
            "D#5:12 | D#4:4 E#4 F#4 | G#4:4 C#5:8 | C#6:4 C#6:8 | _:8 -:2 C#4",
            "ii7 | IV V IV | V7 | V+ | V+",
        )
        .in_key(FSharp, Major)
        .with_tempo(&[fermata(0, 12), tempo(12, 60), tempo(16, 50), tempo(20, 40)]),
        // A fermata on beats 2-3 of bar 1, "molto rallentando" in bar 2 (to 56, so the
        // fermatas after it, held twice as long, stay above a third of the tempo; it starts a
        // step after the fermata ends, as two marks cannot share a step), fermatas on bar 3
        // and on the last chord. The closing arpeggio stops on
        // F#5, held to the end: the score goes on to C#6 and a chord topped by F#6, marked
        // 8va, far above the melody's range.
        Part::new(
            "B ending 2",
            "D#5:12 | D#4:4 E#4 F#4 | C#5:8 A#4:3 A#4:1 | F#4:3 C#5:1 F#5:8 | _:12",
            "ii7 | IV V IV | V7 | I | I",
        )
        .in_key(FSharp, Major)
        .with_tempo(&[
            fermata(4, 8),
            glide(13, 56, 11),
            fermata(24, 8),
            fermata(48, 12),
        ]),
    ],
    form: &[
        "Intro",
        "A",
        "A2",
        "A",
        "A2",
        "B",
        "B ending 1",
        "B",
        "B ending 2",
    ],
    drums_from: 1,
    about: "A piano waltz, here in F# minor: the free introduction turns round the circle of \
            fifths, B9, E7, A, and the waltz theme starts on the dominant, C#7. Its second half \
            chains secondary dominants, F#7-B7-E and E7-A7-D, then speeds up over a bass rising by \
            semitones to a German sixth, D7 over B#, and the dominant. The Lento turns to F# major, \
            the parallel major, with a molto rallentando in each ending.",
};

// Source: https://github.com/profedecuerdas/partituras-2019/blob/master/lagatagolosa.ly
// (LilyPond lead sheet "La Gata Golosa, pasillo", Fulgencio García, "versión abril 23 2020",
// melody with chord names).
// Chords cross-checked with the iReal Pro charts "La Gata Golosa 1 de 2" (in G) and "2 de 2"
// (in Eb) in https://github.com/smashub/choco/blob/main/partitions/ireal-pro/raw/forum/brazilian-latin/columbian%20songs%20-%20m%C3%BAsica%20colombiana.csv,
// and for the trio with the analysis of
// the piece (in G, trio in E flat) in a thesis of the Universidad Pedagógica Nacional,
// https://repositorio.upn.edu.co/bitstreams/2cb61f3c-5ee5-457b-9be2-389de30cf9e8/download
// (read through a summarizer). Bar numbers are the lead sheet's. Where the sources differ:
// bars 21 and 37 take the iReal's C diminished (Bbo in its G), biii°, as the lead sheet's
// F#7 clashes with the tune's A natural; bars 6-7 take the iReal's E9 for the lead sheet's E7,
// its ninth being the F# the tune leans on; bar 63 takes the iReal's D minor, vi of F and iv
// of A, which the tune outlines (D-E-F) and which the thesis names as the link back to the
// home key. The lead sheet's chord is kept in bar 53 (A, held under the pickup into F, where
// the iReal has no chord) and bar 61 (F, where the iReal's C does not fit the tune's A).
// The opening pickup has no chord in either source; it gets I, as the same run has in bars
// 17, 51 and 70.
// Navigation as in the lead sheet: D.C. al Signo after the second strain (the segno at the
// end of the first strain jumps to the trio), D.C. al Coda after the trio; the run that ends
// the second strain and the trio stands for the opening pickup bar when the music goes back.
// PD: Fulgencio García died in 1945, before 1956.
const GATA_B_CHORDS: &str =
    "I | I | biii° | ii | ii | V7 | V7 | I | I7 | IV | iv | I | VI7 | ii | V7 | I";

pub(super) const LA_GATA_GOLOSA: Song = Song {
    name: "La gata golosa",
    origin: "Fulgencio García",
    feel: "Pasillo",
    genre: Genre::Folklore,
    tonic: A,
    mode: Major,
    time: THREE_FOUR,
    tempo: 190.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Pasillo",
    band: GUITARS,
    style: "Pasillo",
    parts: &[
        Part::new("Pickup", "-:2 E4:2 F#4 G#4 A4 B4", "I"),
        Part::new(
            "A first",
            "C#5:2 E4 E4 C#5 C#5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 F#4 F#4 D5 D5:4 | -:2 F#4 G#4 A4 B4 C#5 | \
             D5:2 F#4 F#4 D5 D5:4 | -:2 F#4 F#4 D5 D5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 E4:2 F#4 G#4 A4 B4 | \
             C#5:2 E4 E4 C#5 C#5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 F#4 F#4 B4 B4:4 | -:2 F4 F4 B4 B4:4 | \
             -:2 E4 E4 A4 A4:4 | -:2 E4:2 F#4 E4 G#4 E4 | A4:4 -:2 A4:2 A4:4 | -:2 E4:2 F#4 G#4 A4 B4",
            "I | VI7 | ii | ii | V9 | V9 | I | I | I | I7 | IV | iv | I | V7 | I | I",
        ),
        // The second ending; after the D.C. al Signo, the bar at the segno is the same.
        Part::new(
            "A second",
            "C#5:2 E4 E4 C#5 C#5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 F#4 F#4 D5 D5:4 | -:2 F#4 G#4 A4 B4 C#5 | \
             D5:2 F#4 F#4 D5 D5:4 | -:2 F#4 F#4 D5 D5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 E4:2 F#4 G#4 A4 B4 | \
             C#5:2 E4 E4 C#5 C#5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 F#4 F#4 B4 B4:4 | -:2 F4 F4 B4 B4:4 | \
             -:2 E4 E4 A4 A4:4 | -:2 E4:2 F#4 E4 G#4 E4 | A4:4 -:2 A4:2 A4:4",
            "I | VI7 | ii | ii | V9 | V9 | I | I | I | I7 | IV | iv | I | V7 | I",
        ),
        Part::new(
            "B",
            "-:4 C#5:4 E5 | A5:6 B5 | -:4 A5:2 B5 A5 F#5 | -:4 G#5:6 A5:2 | \
             -:4 G#5:2 A5 G#5 E5 | -:4 F#5:6 G#5:2 | -:4 F#5:2 G#5 F#5 D5 | -:4 E5:6 F#5:2 | \
             -:4 E5:2 F#5 E5 C#5 | -:4 D5:6 E5:2 | -:4 D5:2 E5 D5 B4 | -:4 C#5:6 D5:2 | \
             -:4 C#5:2 D5 C#5 A4 | -:4 B4:6 C#5:2 | -:4 B4:2 C#5 B4 E4 | A4:2 E4 F#4 G#4 A4 B4",
            GATA_B_CHORDS,
        ),
        // The same chords under running eighths, and the run back to the start (D.C. al Signo).
        Part::new(
            "B variation",
            "C#5:2 D5 E5 F#5 G#5 G#5 | A5:2 A5 A5 A5 B5 B5 | B5:2 B5 A5 B5 A5 F#5 | G#5:2 G#5 G#5 G#5 A5 A5 | \
             A5:2 A5 G#5 A5 G#5 E5 | F#5:2 F#5 F#5 F#5 G#5 G#5 | G#5:2 G#5 F#5 G#5 F#5 D5 | E5:2 E5 E5 E5 F#5 F#5 | \
             F#5:2 F#5 E5 F#5 E5 C#5 | D5:2 D5 D5 D5 E5 E5 | E5:2 E5 D5 E5 D5 B4 | C#5:2 C#5 C#5 C#5 D5 D5 | \
             D5:2 D5 C#5 D5 C#5 A4 | B4:2 B4 B4 B4 C#5 C#5 | C#5:2 C#5 B4 C#5 B4 E4 | A4:4 -:2 A4:2 A4:4 | \
             -:2 E4:2 F#4 G#4 A4 B4",
            "I | I | biii° | ii | ii | V7 | V7 | I | I7 | IV | iv | I | VI7 | ii | V7 | I | I",
        ),
        // The key signature turns to F at the trio's first bar, a pickup bar over the A
        // major chord (III of F) that F follows; D minor in the last bar leads back to A.
        Part::new(
            "Trio",
            "-:4 A4:4 Bb4:2 C5 | F4:12 | -:4 D5:4 E5:2 F5 | Bb4:12 | \
             -:4 G5:4 A5:2 Bb5 | E5:4 F5:2 G5 D5:4 | E5:2 F5 C5:4 D5:2 E5 | D5:4 C5:2 Bb4 A4:4 | \
             -:4 A4:4 Bb4:2 C5 | F4:12 | -:4 D5:4 E5:2 F5",
            "III | I | I | ii | ii | V7 | V7 | I | I | I | vi",
        )
        .in_key(F, Major),
        Part::new(
            "Trio end",
            "E5:12 | -:4 E5:4 F#5:2 A5 | G#5:4 D5:2 F#5 E5:4 | B4:2 D5 C#5:4 G#4:2 B4 | \
             A4:4 -:2 A4:2 A4:4",
            "I | I | V7 | V7 | I",
        ),
        // The second ending and the run back to the start (D.C. al Coda).
        Part::new(
            "Trio end 2",
            "E5:12 | -:4 E5:4 F#5:2 A5 | G#5:4 D5:2 F#5 E5:4 | B4:2 D5 C#5:4 G#4:2 B4 | \
             A4:4 -:2 A4:2 A4:4 | -:2 E4:2 F#4 G#4 A4 B4",
            "I | I | V7 | V7 | I | I",
        ),
        Part::new(
            "A coda",
            "C#5:2 E4 E4 C#5 C#5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 F#4 F#4 D5 D5:4 | -:2 F#4 G#4 A4 B4 C#5 | \
             D5:2 F#4 F#4 D5 D5:4 | -:2 F#4 F#4 D5 D5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 E4:2 F#4 G#4 A4 B4 | \
             C#5:2 E4 E4 C#5 C#5:4 | -:2 E4 E4 C#5 C#5:4 | -:2 F#4 F#4 B4 B4:4 | -:2 F4 F4 B4 B4:4 | \
             -:2 E4 E4 A4 A4:4 | -:2 E4:2 F#4 E4 G#4 E4 | A4:4 -:2 A4:2 A4:4 | _:12",
            "I | VI7 | ii | ii | V9 | V9 | I | I | I | I7 | IV | iv | I | V7 | I | I",
        ),
    ],
    form: &[
        "Pickup",
        "A first",
        "A second",
        "B",
        "B variation",
        "A second",
        "Trio",
        "Trio end",
        "Trio",
        "Trio end 2",
        "A coda",
    ],
    drums_from: 1,
    about: "A quick Colombian pasillo in A major. The first strain \
            goes I-VI7-ii-V and turns to D and D minor, IV and iv, before the cadence; the second \
            strain opens A, C diminished, B minor, climbs down by sequence, and comes back as \
            a variation in running eighths. The trio moves to F major for a ii-V7-I; \
            then D minor, vi in F and iv in A, leads back to A major for an E7-A cadence.",
};

// Source: https://raw.githubusercontent.com/flvani/diatonic-map/4f0966a394a536852809e5f4c215d44243854d98/songs/minuano/pezinho.abcx
// (Flávio Vanni's map for the diatonic accordion, in G, 2/4, with the words and chords G and D)
// Second source: John Chambers' tune finder, abc2book collection, abcresources/jc/abc_tune_jc_201323534.abc
// ("Pezinho [G]", J. R. Meletti, 4/4, chords G and D7). The notes agree; where Meletti holds
// a note through the next beat the accordion map rests, and the map's words (two pickup notes,
// "e de-", into the second half) are followed here.
// PD: traditional (Rio Grande do Sul).
pub(super) const PEZINHO: Song = Song {
    name: "Pezinho",
    origin: "Traditional, Brazil (Rio Grande do Sul)",
    feel: "Pezinho",
    genre: Genre::Folklore,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Backbeat,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    band: ACCORDION_BAND,
    style: "Polka",
    parts: &[
        Part::new("Pickup", "-:10 D5:2 D5 D5", "V7"),
        Part::new(
            "A",
            "B4:2 D5 D5 D5 E5 D5 C5 B4 | D5:4 A4 -:2 A4 A4 B4 | C5 B4 A4 B4 C5 B4 C5 D5 | \
             B4:8 -:2 D5 D5 D5",
            "I | V7 | V7 | I",
        ),
        Part::new(
            "A into B",
            "B4:2 D5 D5 D5 E5 D5 C5 B4 | D5:4 A4 -:2 A4 A4 B4 | C5 B4 A4 B4 C5 B4 C5 D5 | \
             B4:8 -:4 D5:2 D5",
            "I | V7 | V7 | I",
        ),
        Part::new(
            "B",
            "G5:8 -:2 F#5 E5 D5 | F#5:4 C5 - D5:2 D5 | F#5:8 -:2 E5 D5 C5 | E5:4 B4 - D5:2 D5",
            "I | V7 | V7 | I",
        ),
        Part::new(
            "B end",
            "G5:8 -:2 F#5 E5 D5 | F#5:4 C5 - D5:2 D5 | F#5:8 -:2 E5 D5 C5 | B4:12 -:4",
            "I | V7 | V7 | I",
        ),
    ],
    form: &["Pickup", "A", "A into B", "B", "B end"],
    drums_from: 1,
    about: "A traditional dance-song of Rio Grande do Sul, in which the singer asks the partner \
            to put the little foot here, then there, close to mine. Both halves of the tune \
            stand on the same four chords, G, D7, D7, G, over the accordion's oom-pah. Each bar \
            here holds two bars of 2/4.",
};

// Source: https://raw.githubusercontent.com/flvani/diatonic-map/4f0966a394a536852809e5f4c215d44243854d98/songs/minuano/balaio.abcx
// (Flávio Vanni's map for the diatonic accordion, in G, 2/4: Parte I, Parte II, D.S. and a
// final Parte I; the tune is the top line, the chords G and D from the accordion's bass)
// Second source for the second part: https://github.com/tcanabrava/harmonicon-songs,
// Traditional/Balaio/song/chart.harpchart (from the UFSCar folklore vocal score, in C): the same
// notes, C C D E F D B G, with small differences of rhythm.
// PD: traditional (Rio Grande do Sul).
pub(super) const BALAIO: Song = Song {
    name: "Balaio",
    origin: "Traditional, Brazil (Rio Grande do Sul)",
    feel: "Balaio",
    genre: Genre::Folklore,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Habanera",
    band: ACCORDION_BAND,
    style: "Habanera",
    parts: &[
        Part::new("Pickup", "-:14 G5:2", "V7"),
        Part::new(
            "Part I",
            "G5:1 E5:2 C5:1 A4:2 G4 F5:1 D5:2 B4:1 A4:2 G4 | A4:1 A4:2 A4:1 F4:2 F4 G4:3 - G5:2 | \
             G5:1 E5:2 C5:1 A4:2 G4 F5:1 D5:2 B4:1 A4:2 G4:1 G4 | A4:2 A4 B4 B4 C5:3 - G4:1 G4",
            "I V7 | V7 I | I V7 | V7 I",
        ),
        Part::new(
            "Part II",
            "C5:2 C5 D5 E5 F5:3 D5:1 B4:2 G4:1 G4 | A4:2 A4 F4 F4 G4:3 - G4:1 G4 | \
             C5:2 C5 D5 E5 F5:3 D5:1 B4:2 G4:1 G4 | A4:2 A4 B4 B4 C5:3 - G5:2",
            "I V7 | V7 I | I V7 | V7 I",
        ),
        // The last bar of the score is a half bar, under a fermata: padded with a rest.
        Part::new(
            "Final",
            "G5:1 E5:2 C5:1 A4:2 G4 F5:1 D5:2 B4:1 A4:2 G4 | A4:1 A4:2 A4:1 F4:2 F4 G4:3 - G5:2 | \
             G5:1 E5:2 C5:1 A4:2 G4 F5:1 D5:2 B4:1 A4:2 G4:1 G4 | A4:2 A4 B4 B4 C5:4 -",
            "I V7 | V7 I | I V7 | V7 I",
        )
        .with_tempo(&[fermata(56, 4)]),
    ],
    form: &["Pickup", "Part I", "Part II", "Final"],
    drums_from: 1,
    about: "A traditional song and dance of Rio Grande do Sul. Both parts go C, G7, G7, C, over \
            the accordion's bass in the habanera rhythm, long, short, long, long. Each bar here \
            holds two bars of 2/4; the source is in G, a fourth lower.",
};

// Source: https://raw.githubusercontent.com/flvani/diatonic-map/4f0966a394a536852809e5f4c215d44243854d98/songs/minuano/luar.do.sertao.abcx
// (Flávio Vanni's map for the diatonic accordion, in C, 4/4, with chord symbols), checked
// against the same arranger's version in F with the words,
// https://raw.githubusercontent.com/flvani/diatonic-map/4f0966a394a536852809e5f4c215d44243854d98/songs/club-br/luar.do.sertao.club.abcx
// The song is strophic: three verses (four in Luiz Gonzaga's recording) to the same tune, each
// followed by the refrain, as Wikipedia's article on the song and the chord sheets of Gonzaga's
// and Milton Nascimento's recordings give it. The map writes the refrain twice, one verse and
// the refrain twice; the form here plays the verse and its two refrains three times.
// PD: Catulo da Paixão Cearense died in 1946 and João Pernambuco in 1947.
pub(super) const LUAR_DO_SERTAO: Song = Song {
    name: "Luar do sertão",
    origin: "Catulo da Paixão Cearense and João Pernambuco",
    feel: "Toada",
    genre: Genre::Folklore,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Backbeat,
    bass: Bass::RootFifth,
    rhythm: "Folk",
    band: SONG_AND_GUITAR,
    style: "Folk",
    parts: &[
        Part::new("Pickup", "-:14 G4:2", "V7"),
        Part::new(
            "Refrain",
            "C5:6 D5:2 E5:6 D5:2 | F5:6 E5:2 D5:6 C5:2 | B4:6 A4:2 G4:4 F4 | E4:14 G4:2",
            "I vi | ii IV | V7 | I I I V7",
        ),
        Part::new(
            "Refrain into verse",
            "C5:6 D5:2 E5:6 D5:2 | F5:6 E5:2 D5:6 C5:2 | B4:6 A4:2 G4:4 D5",
            "I vi | ii IV | V7",
        ),
        // Then D.S. al Fine: the refrain again, ending on C.
        Part::new(
            "Verse",
            "C5:10 G4:2 A4 B4 | C5 C5 C5 D5 E5 E5 E5 D5 | F5 F5 F5 E5 D5 D5 D5 C5 | \
             D5 C5 B4 A4 B4 A4 G4 F4 | E4:10 G4:2 A4 B4 | C5 C5 C5 D5 E5 E5 E5 D5 | \
             F5 F5 F5 E5 D5 D5 D5 C5 | D5 C5 B4 A4 B4 A4 G4 D5 | C5:14 G4:2",
            "I V7 | I vi | ii IV | V7 | I V7 | I vi | ii IV | V7 | I I I V7",
        ),
        Part::new(
            "Refrain end",
            "C5:6 D5:2 E5:6 D5:2 | F5:6 E5:2 D5:6 C5:2 | B4:6 A4:2 G4:4 D5 | C5:16",
            "I vi | ii IV | V7 | I",
        ),
    ],
    form: &[
        "Pickup",
        "Refrain",
        "Refrain into verse",
        "Verse",
        "Refrain",
        "Refrain into verse",
        "Verse",
        "Refrain",
        "Refrain into verse",
        "Verse",
        "Refrain",
        "Refrain end",
    ],
    drums_from: 1,
    about: "A toada in praise of the moonlight of the sertão, Brazil's backlands. The refrain \
            climbs from C over C and A minor and falls back through D minor and F to G7; the \
            verse walks the same chords in steady eighths, three repeated notes and a step. The \
            refrain, sung twice, opens the song and follows each of its three verses.",
};

// Source: https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmVxmDt9pfR6yVSK4FK38kehyaVvEoHJ5UZHcC636wwaAr.abc
// (abc_search database: "La Llorona", an anonymous MuseScore piano lead sheet in D minor, 3/4,
// with chord symbols Dm C Bb A7 Gm; Section A the instrumental theme with Fine at bar 19, B the
// verse and C the refrain inside one repeat, D.C. at the end), written an octave higher.
// Second source: https://github.com/hf7y/lilypond (archive/brass-charts/muertos-dio/la_llorona.ly,
// a brass chart in C minor): its a-melody and b-melody have the verse's and the refrain's pitches
// and chords (Cm Fm G7 / Cm Bb Ab G7, a tone lower); its rhythms are a little different.
// PD: traditional (a son of the Isthmus of Tehuantepec, Oaxaca, with no known author)
pub(super) const LA_LLORONA: Song = Song {
    name: "La Llorona",
    origin: "Traditional, Mexico (Isthmus of Tehuantepec)",
    feel: "Son istmeño",
    genre: Genre::Folklore,
    tonic: D,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 100.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Ranchera",
    band: SONG_AND_GUITAR,
    style: "Ranchera",
    parts: &[
        Part::new(
            "Intro",
            "F5:2 E5 D5 E5 F5:4 | _:2 A5 G5 F5 E5 D5 | C5 B4 C5 D5 E5:4 | _:2 G5 F5 E5 D5 C5 | \
             Bb4 A4 Bb4 C5 D5:4 | _:2 D5 E5 D5 C#5 D5 | E5:4 E5:2 C#5 A4:4 | -:2 A4 B4 C#5 D5 E5 | \
             F5 E5 D5 E5 F5:4 | _:2 A5 G5 F5 E5 D5 | C5 B4 C5 D5 E5:4 | _:2 G5 F5 E5 D5 C5 | \
             Bb4 A4 Bb4 C5 D5:4 | _:2 D5 E5 D5 C#5 D5 | E5:4 E5:2 C#5 A4:4 | -:12",
            "i | i | bVII | bVII | bVI | bVI | V7 | V7 | i | i | bVII | bVII | bVI | bVI | \
             V7 | V7",
        ),
        Part::new("Tag", "A5:4 Bb5 A5 | G5 F5 E5 | D5 -:8", "V7 | V7 | i"),
        Part::new("Vamp", "-:12 | - | -", "V7 | i | V7"),
        Part::new(
            "Verse",
            "A4:6 A4:2 A4:4 | A4:8 A4:4 | C5 Bb4:6 A4:2 | C5:4 Bb4 - | A4:6 A4:2 A4:4 | \
             Bb4:8 A4:4 | A4 G4:8 | -:12 | A4:6 A4:2 A4:4 | A4 D5:6 C5:2 | C5:4 Bb4:6 A4:2 | \
             C5:4 Bb4 - | A4:6 A4:2 A4:4 | Bb4:8 A4:4 | A4 G4:8 | - A4:4",
            "i | i | iv | iv | i | i | V7 | V7 | i | i | iv | iv | i | i | V7 | V7",
        ),
        Part::new(
            "Refrain",
            "D5:6 D5:2 D5:4 | F5:8 F5:4 | E5 C5:6 D5:2 | E5:4 C5 - | Bb4:6 Bb4:2 Bb4:4 | \
             D5:8 D5:4 | C#5 A4:8 | - A4:4 | D5:6 D5:2 D5:4 | F5:8 F5:4 | E5 C5:6 D5:2 | \
             E5:4 C5 - | Bb4:6 Bb4:2 Bb4:4 | D5:8 D5:4 | C#5 A4:8 | -:12",
            "i | i | bVII | bVII | bVI | bVI | V7 | V7 | i | i | bVII | bVII | bVI | bVI | \
             V7 | V7",
        ),
    ],
    form: &[
        "Intro", "Tag", "Vamp", "Verse", "Refrain", "Verse", "Refrain", "Intro", "Tag",
    ],
    drums_from: 0,
    about: "A slow son from Oaxaca in D minor and in three. The instrumental theme and the \
            refrain walk down the Andalusian cadence, D minor, C, B flat and A7, two bars to a \
            chord, while the verse only rocks between D minor, G minor and A7. A7's C sharp \
            pulls every phrase home to D.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/
// wikifonia-dataset/original-abc/PacoPerezLunadeXelaju.abc (Wikifonia lead sheet, "Luna de Xelajú, Paco
// Perez (1917 - 1951)", transcribed by Alberto Betancourt, Vals lento, 1/4 = 120, with words and chords:
// an intro with first and second endings, two verses in A minor, the intro again, a chorus in A
// major from a segno, D.S. al Coda). The chords Bm7-5/F, Bm7-5 written as text in the source are
// iiø7; slash chords are written as the chord itself, except A minor over G sharp, G and F sharp,
// which are Am(maj7), Am7 and Am6 (imaj7, i7, i6), the same notes. "Fdim", "Cdim", "Ddim" are triads.
// In bars 102-103 the source has N.C.; the D chord is held. The N.C. pickup bars 9 and 45 get i.
// The tie from the chorus's last A (bar 116) into the coda (bars 120-121) is kept: the Coda part
// starts with bar 116 and holds that A on.
// PD: Paco Pérez, the only author, died in 1951 (https://en.wikipedia.org/wiki/Paco_P%C3%A9rez_(musician))
pub(super) const LUNA_DE_XELAJU: Song = Song {
    name: "Luna de Xelajú",
    origin: "Paco Pérez, 1944",
    feel: "Vals",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 120.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    band: Band::new(gm::MARIMBA, gm::MARIMBA, gm::ACOUSTIC_BASS),
    style: "Waltz",
    parts: &[
        Part::new("Pickup", "-:2 B4 C5 D5 C5 B4", "i"),
        Part::new(
            "Intro",
            "D5:2 D5 D5 D5 D5 D5 | D5 C5 B4 C5 D5 B4 | C5 C5 C5 C5 C5 C5 | C5 B4 A4 B4 C5 A4 | \
             B4 B4 B4 B4 B4 B4 | B4 A4 G#4 A4 B4 C5 | A4 A4 A4 A4 A4 A4",
            "iv | iv | i | i | V7 | V7 | i",
        ),
        Part::new("Intro turn", "A4:2 B4 C5 D5 C5 B4", "i"),
        Part::new(
            "Intro end",
            "A4:6 E4:2 E4:4 | - G4:8 | -:6 E4:2 E4:4 | - G4:8",
            "i | V7#9 | i | V7#9",
        ),
        Part::new(
            "Verse",
            "A4:4 G#4 A4 | C5 B4:6 A4:2 | A4:4 G#4:2 - A4:4 | C5 B4:6 A4:2 | A4:4 G#4:2 - A4:4 | \
             C5 B4 A4 | F4:12 | _:4 -:8 | E4:4 D#4 E4 | G4 F4:6 E4:2 | E4:4 D#4:2 - E4:4 | \
             G4 F4:6 E4:2 | E4:4 D#4:2 - E4:4 | G4 F4 E4 | E4:12 | -:4 G4:8",
            "i | imaj7 | i7 | i6 | bVImaj7 | i | iv | iiø7 | V | iiø7 | V | iiø7 | V | \
             V7 | i | V7#9",
        ),
        Part::new(
            "Verse 2",
            "A4:4 G#4 A4 | C5 B4:6 A4:2 | A4:4 G#4:2 - A4:4 | C5 B4:6 A4:2 | A4:4 G#4:2 - A4:4 | \
             C5 B4 A4 | F5:12 | _:4 -:8 | D5:4 D5:6 D5:2 | D5:4 C5 B4 | D5 C5 -:2 C5 | \
             C5:4 B4 A4 | C5 B4 -:2 B4 | B4:4 A4 G#4 | A4:12",
            "i | imaj7 | i7 | i6 | bVImaj7 | i | iv | iv | iv | iv | i | i | V7 | V7 | \
             i",
        ),
        Part::new(
            "Interlude",
            "-:2 B4 C5 D5 C5 B4 | D5 D5 D5 D5 D5 D5 | D5 C5 B4 C5 D5 B4 | C5 C5 C5 C5 C5 C5 | \
             C5 B4 A4 B4 C5 A4 | B4 B4 B4 B4 B4 B4 | B4 A4 G#4 A4 B4 C5",
            "i | iv | iv | i | i | V7 | V7",
        ),
        Part::new(
            "Lead-in",
            "C#5:12 | _ | _:4 C#5 D5 | E5 D5 C#5",
            "I | vi7 | ii7 | V7",
        )
        .in_key(A, Major)
        .with_tempo(&[fermata(40, 4)]),
        Part::new(
            "Chorus 1",
            "E5:12 | _ | -:4 A4 B4 | C#5 B4 A4 | C#5:12 | _ | -:4 G#4 A4 | B4 A4 G#4 | \
             A4 A4:6 A4:2 | A4:4 G#4 F#4 | A4 A4 -:2 A4 | A4:4 G#4 F#4 | D5:12 | _ | -:4 B4 C#5 | \
             D5 C#5 B4",
            "I | Imaj7 | I6 | bvi° | I | Imaj7 | I6 | V7 | I | ii7 | I | biii° | ii7 | \
             ii | V7 | V7",
        )
        .in_key(A, Major),
        Part::new(
            "Chorus 2",
            "D5:12 | _ | -:4 G#4 A4 | B4 A4 G#4 | B4:12 | _ | -:4 G#4 A4 | B4 A4 G#4 | \
             A4 A4:6 A4:2 | A4:4 G#4 F#4 | G#4 G#4 -:2 E4 | E4:4 F#4 G#4 | F#4 -:8 | -:12 | \
             -:4 C#5 D5 | E5 D5 C#5",
            "ii | bVII | V7 | V7 | ii | bVII | V7 | V7 | IV6 | ii7 | V7 | V7 | I | V7 | \
             I | V7",
        )
        .in_key(A, Major),
        Part::new(
            "Chorus 3",
            "E5:12 | _ | -:4 A4 B4 | C#5 B4 A4 | C#5:12 | _ | -:4 G#4 A4 | B4 A4 G#4 | \
             G4 G4:6 G4:2 | G4:4 F#4 E4 | G4 G4 - | G4:2 A4 B4 C#5 E5 C#5 | B4:12 | _ | \
             -:4 A4 B4 | C5 B4 A4",
            "I | Imaj7 | I6 | bvi° | I | Imaj7 | vi7 | ii7 | v7 | v7 | I7 | I7 | IV | iv° | \
             IV | IV",
        )
        .in_key(A, Major),
        Part::new(
            "Chorus 4",
            "C5:12 | _ | -:4 A4 B4 | C5 B4 A4 | E5:12 | _ | _:2 - C#5:4 D5 | E5 D5 C#5 | \
             C#5 B4:6 A#4:2 | B4:4 F#5:6 D5:2 | C#5:4 B4:6 A#4:2 | B4:4 E5:6 C#5:2",
            "bVI | bVI | bVI | bVI | I | bVII7 | VI7 | VI7 | ii7 | ii7 | V7 | V7",
        )
        .in_key(A, Major),
        Part::new(
            "Turnaround",
            "A4:12 | _ | _:2 - C#5:4 D5 | E5 D5 C#5",
            "I | vi7 | ii7 | V7",
        )
        .in_key(A, Major),
        Part::new(
            "Coda",
            "A4:12 | _ | _:4 - E5 | A5 -:8",
            "I | iv | I I V7 | I",
        )
        .in_key(A, Major),
    ],
    form: &[
        "Pickup",
        "Intro",
        "Intro turn",
        "Intro",
        "Intro end",
        "Verse",
        "Verse 2",
        "Interlude",
        "Lead-in",
        "Chorus 1",
        "Chorus 2",
        "Chorus 3",
        "Chorus 4",
        "Turnaround",
        "Chorus 1",
        "Chorus 2",
        "Chorus 3",
        "Chorus 4",
        "Coda",
    ],
    drums_from: 1,
    about: "Guatemala's best-loved waltz, from Quetzaltenango. The verses are in A minor over a \
            line cliché, one note falling A, G sharp, G, F sharp under the A minor chord, and \
            the first verse stops on E, rocking between B half-diminished and E; the chorus \
            turns to A major, the parallel key, and visits chords from outside it: G, F sharp 7, \
            F major and D minor.",
};

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201305318.abc
// (John Chambers' "Los Machetes [G,C]", polka, Mexico, 2/4, in thirds with chord symbols: an intro on
// D7, a segno, A with a repeat ending at Fine, B with a repeat, a trio in C with a repeat, D.S.).
// The melody is the upper note of the thirds. Single source (JC's second setting, in A and D, is the
// same tune a tone higher). The 2/4 bars are paired; the eighth-note pickups that start each
// repeated strain end the part before. After the D.S. the A strain is played once, to Fine.
// PD: traditional (JC names no composer; the Hermanos Villarreal recording on Oro 252-B credits only an
// arrangement, https://frontera.library.ucla.edu/es/node/285999)
pub(super) const LOS_MACHETES: Song = Song {
    name: "Los machetes",
    origin: "Traditional, Mexico",
    feel: "Polca",
    genre: Genre::Folklore,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Offbeat,
    bass: Bass::RootFifth,
    rhythm: "Polca",
    band: ACCORDION_BAND,
    style: "Polca",
    parts: &[
        Part::new("Intro", "D5:4 D5 D5 -:2 G5", "V7"),
        Part::new(
            "A",
            "F#5:2 G5 F#5 E5 D5:4 B4:2 D5 | C5 B4 C5 E5 D5:4 B4:2 G5 | F#5 G5 F#5 E5 D5:4 B4:2 D5 | \
             C5 B4 C5 A4 G4:4 -:2 G5",
            "V7 I | V7 I | V7 I | V7 I",
        ),
        Part::new(
            "A end",
            "F#5:2 G5 F#5 E5 D5:4 B4:2 D5 | C5 B4 C5 E5 D5:4 B4:2 G5 | F#5 G5 F#5 E5 D5:4 B4:2 D5 | \
             C5 B4 C5 A4 G4:4 -",
            "V7 I | V7 I | V7 I | V7 I",
        ),
        Part::new(
            "B",
            "B4:2 D5 D5 D5 C5 E5 E5:4 | D5:2 F#5 A5 F#5 G5 B5 B5:4 | B4:2 D5 D5 D5 C5 E5 E5:4 | \
             D5:2 F#5 A5 F#5 G5 G5 G5 -",
            "I IV | V7 I | I IV | V7 I",
        ),
        Part::new(
            "Trio",
            "E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 | D5 E5 F5 G5 A5:4 G5 | \
             E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 | D5 F5 E5 D5 C5:4 C5:2 -",
            "I | V7 V7 IV V | I | V7 I",
        )
        .in_key(C, Major),
        Part::new(
            "Trio to A",
            "E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 | D5 E5 F5 G5 A5:4 G5 | \
             E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 E5:1 F5 G5:2 | D5 F5 E5 D5 C5:4 C5:2 G5",
            "I | V7 V7 IV V | I | V7 I",
        )
        .in_key(C, Major),
    ],
    form: &[
        "Intro",
        "A",
        "A end",
        "B",
        "B",
        "Trio",
        "Trio to A",
        "A end",
    ],
    drums_from: 0,
    about: "A Mexican polka in G major built on only I, IV and V7: the first strain answers D7 \
            with G in every bar, the second climbs through C. The trio moves to C major, the key \
            of IV, as polkas and marches do, before the first strain comes back. The polka is in \
            2/4, two bars to a bar here.",
};

// Source: https://github.com/linkmoises/musica-folklorica (commit d39b73ae9780b0a365337dfcd508d75da25fdd2a),
// "V02 - Pasillo - Alcedo, César - Brisas Mesanas.ly": "Isabel (Brisas Mesanas)", pasillo, a
// LilyPond lead sheet (melody and chord names) in A minor with a trio in A major, 3/4, "Andante
// moderato", MIDI tempo 4 = 105. Single source. Bars with no chord name in the score ("s2.")
// keep the chord before them; the first pickup, which has none, is given i. The second A
// ties its first two notes with a slur where the first A has a tie; it is played as a tie
// (the same part). The trio's bar 6 (G#4, D4, E4, G#4: an E7 arpeggio, the bar-2 figure
// a step lower) has an A chord in the score, which clashes with the G# on the downbeat; it is
// played as E (V), the chord the score gives the next bar. The trio's first bar has no chord
// name; it keeps the E before it, also on the repeat.
// PD: César Alcedo Alvarado (1890-1931): the score's header, and La Estrella de Panamá,
// "Un rescate cultural hacia el legado de los pasillos panameños" (Astrid Chang, 5 Dec 2020),
// which gives the same dates and says the pasillo's real title is "Isabelita".
pub(super) const BRISAS_MESANAS: Song = Song {
    name: "Isabel (Brisas mesanas)",
    origin: "César Alcedo, Panama",
    feel: "Pasillo",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 105.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Pasillo",
    band: GUITARS,
    style: "Pasillo",
    parts: &[
        Part::new("Pickup", "-:2 E4 D#4 E4 A4 C5", "i"),
        Part::new(
            "A",
            "E5:12 | _:4 B4:6 D5:2 | C5:12 | -:2 G4 F#4 G4 C5 E5 | G5:6 F#5:2 G5 F#5 | \
             G5:4 D5 D#5 | E5:12 | _:4 B4 D5 | C5:4 C5:2 D5 E5:4 | -:4 B4 D5 | C5:4 C5:2 D5 E5:4 | \
             _:4 E5 A5 | G#5:2 B5:4 G#5:2 B5:4 | F#5:2 B5:4 F#5:2 B5:4 | E5:6 E5:2 E5:4",
            "i | V | i | i | bIII | bVII | bIII | V | i | V | i | i | II7 | II7 | V",
        ),
        Part::new("A turn", "-:2 E4 D#4 E4 A4 C5", "V"),
        Part::new("A end", "-:2 E5 F5:6 E5:2", "V"),
        Part::new(
            "B",
            "E5:2 D5 D5 C5 B4:4 | -:2 D5 E5:6 D5:2 | D5:2 C5 C5 B4 A4:4 | -:2 A4 C5:4 A4 | \
             F4:2 A4 C5 F4 A4 C5",
            "V | V | i | i | bVI",
        ),
        Part::new(
            "B turn",
            "E4:2 G#4 B4 E4 G#4 B4 | A4:2 G#4 A4 B4 C5 D5",
            "V | i",
        ),
        Part::new(
            "B end",
            "E4:2 G#4 B4 E5 B4 C5 | A4:12 | -:2 E4 D#4 E4 A4 D5",
            "V | i | i",
        ),
        Part::new(
            "Trio",
            "-:2 F#5 E5:4 D5 | C#5:4 E4:2 A4 C#5:4 | _:2 B4 A4:4 G#4 | F#4:4 F#4:2 B4 D5:4 | \
             -:2 C#5 B4:4 A4 | G#4:4 D4:2 E4 G#4:4 | _:2 F#4 E4:4 D4 | C#4:2 C#4:4 F#4:2 E4:4 | \
             -:2 F#5 E5:4 D5 | C#5:4 E4:2 A4 C#5:4 | _:2 B4 A4:4 G#4 | F#4:4 F#4:2 B4 D5:4 | \
             -:2 D5 E5:6 F#5:2 | E5:4 F#5:6 E5:2 | D5:4 C#5:6 B4:2",
            "V | I | I | ii | ii | V | V | I | V | I | I | ii | IV | I | V",
        )
        .in_key(A, Major),
        Part::new("Trio turn", "A4:12", "I").in_key(A, Major),
        Part::new("Trio end", "A4:4 C#5 E5 | A5:2 -:10", "I I V | I").in_key(A, Major),
    ],
    form: &[
        "Pickup",
        "A",
        "A turn",
        "A",
        "A end",
        "B",
        "B turn",
        "B",
        "B end",
        "A",
        "Trio",
        "Trio turn",
        "Trio",
        "Trio end",
    ],
    drums_from: 1,
    about: "A Panamanian pasillo in A minor that visits C major (C, G, C) and comes back to A \
            minor through E; each half of the strain closes on E, the second time after B7, \
            the dominant of the dominant. The middle strain leans on F before E, and the trio \
            turns to the parallel major, A, where B minor (ii) and D (IV) lead to E and home.",
};

// Source: https://github.com/linkmoises/musica-folklorica (commit d39b73ae9780b0a365337dfcd508d75da25fdd2a),
// "V01 - Punto - Desconocido - Punto ocueño.ly": "Punto Ocueño", punto, "Autor Desconocido",
// transcribed by Luis Pedro Quintero C.; a LilyPond lead sheet (melody and chord names) in
// A minor, 6/8: an ad-libitum introduction, then from the segno "Allegretto" a repeated strain
// with two endings, a long middle strain, a repeated closing strain whose second ending says
// "D.S. al Fine", and five more bars after a double bar, the last marked "Fine". Single source.
// The D.S. pass is played without the repeats, as is usual. Its closing strain does not take
// the 2nd ending (that bar's E-F pickup leads back to the segno): the five bars after the
// double bar take its place. Their first bar is the closing strain's 1st-ending bar again
// (C held, then A-C leading on), followed by a last four-bar phrase F | E7 | E7 | A minor,
// as the coda does in this collection's other scores ("D.S. al coda").
// PD: traditional (the score names the author as unknown)
// Wants rhythm: Punto (6/8): the Panamanian punto, a 6/8 couple dance played by violin, guitar
// and drums; its pattern needs a recorded reference.
pub(super) const PUNTO_OCUENO: Song = Song {
    name: "Punto ocueño",
    origin: "Traditional, Panama (Ocú)",
    feel: "Punto",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: SIX_EIGHT,
    tempo: 150.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    band: FIDDLE_AND_GUITAR,
    style: "",
    parts: &[
        Part::new(
            "Intro",
            "E5:6 F5 | G5:6 F5 | E5:6 D5 | C5:6 B4 | A4:6 G#4 | A4:6 C5 | E5:6 C5 | A5:6 E5 | \
             E5:6 E5 | A5:6 E5 | C5:6 B4 | A4:6 -:2 E5 F5",
            "i | i | i | i | i | i | i | i | i | i | i | i",
        )
        // "Ad libitum", with fermatas on the first notes of bars 1 and 9.
        .with_tempo(&[tempo(0, 100), fermata(0, 6), fermata(96, 6)]),
        Part::new(
            "A",
            "G5:2 G5 F#5 G5 E5:4 | F5:2 F5:4 -:2 D5 E5 | F5:2 A5:4 G#5:2 A5:4 | E5:6 -:2 E4 E4 | \
             E4:2 A4 C5 F5 E5:4 | D5:2 C5 B4 _ E4 E4 | E4:2 G#4 B4 E5 D5:4",
            "I7 | iv | bVII7 | bIII | bVI | V7 | V7",
        )
        // "Allegretto" from the segno.
        .with_tempo(&[tempo(0, 150)]),
        Part::new("A turn", "C5:2 B4 A4 - E5 F5", "i"),
        Part::new("A end", "E5:4 A5:2 _ B5:4", "i"),
        Part::new(
            "B1",
            "A5:2 G5:4 F5:2 E5:4 | D5:4 B5:2 _ A5:4 | G5:2 F5:4 E5:2 D5:4 | C5:2 A4 B4 C5 A4 B4 | \
             C5:2 A4 B4 C5 D5 E5 | B5:2 G#5 E5 G#5 E5:4 | B5:2 G#5 E5 G#5 E5:4 | \
             C5:2 A5 E5 A5 E5:4 | C5:2 A5 E5 A5 E5:4 | B5:2 G#5 E5 G#5 E5 G#5 | \
             B5:2 G#5 E5 D5 B4:4 | C5:6 _:2 A4 C5 | F5:6 E5 | B4:2 C5 D5 _ E4 B4",
            "bVI | V7 | V7 | i | bVI | V7 | V7 | i | bVI | V7 | V7 | i | bVI | V7",
        ),
        Part::new(
            "B2",
            "E5:2 E5:4 D5:2 D5:4 | C5:6 _:2 A4 C5 | F5:6 E5 | B4:2 C5 D5 _ E4 B4 | \
             E5:2 E5:4 D5:2 D5:4 | C5:2 A5 A5 A5 A5 A5 | C5:2 A5 A5 A5 A5 A5 | \
             B4:2 G#5 G#5 G#5 G#5 G#5 | B5:2 G#5 G#5 G#5 G#5 G#5 | C5:2 A5 A5 A5 A5 A5 | \
             C5:2 A5 A5 A5 A5 A5 | B4:2 G#5 G#5 G#5 E5 G#5 | B5:2 G#5 E5 D5 B4:4 | C5:6 _:2 A4 C5",
            "V7 | i | bVI | V7 | V7 | i | bVI | V7 | V7 | i | bVI | V7 | V7 | i",
        ),
        Part::new(
            "C",
            "F5:6 E5 | B4:2 C5 D5 _ E4 B4 | E5:2 E5:4 D5:2 D5:4",
            "bVI | V7 | V7",
        ),
        Part::new("C turn", "C5:6 _:2 A4 C5", "i"),
        Part::new("C end", "C5:6 _:2 E5 F5", "i"),
        Part::new(
            "Fine",
            "C5:6 _:2 A4 C5 | F5:6 E5 | B4:12 | E5:2 E5:4 F5:2 G#5:4 | A5:12",
            "i | bVI | V7 | V7 | i",
        ),
    ],
    form: &[
        "Intro", "A", "A turn", "A", "A end", "B1", "B2", "C", "C turn", "C", "C end", "A",
        "A end", "B1", "B2", "C", "Fine",
    ],
    drums_from: 1,
    about: "A punto from Panama in A minor. After a free introduction over A minor, the dance \
            opens on A7, the dominant of D minor, and goes round the circle of fifths, D \
            minor, G7, C, F, to E7; the rest of the tune turns again and again on F, E7 \
            and A minor.",
};

// Source: https://raw.githubusercontent.com/flvani/diatonic-map/4f0966a394a536852809e5f4c215d44243854d98/songs/minuano/boi.barroso.abcx
// (Flávio Vanni's map for the diatonic accordion, "Folclore regional", arranged by Cezar
// Ferreira, in G, 2/4: Parte I with two endings, Parte II repeated, D.S. al coda; the tune is
// the top line, the chords from the accordion's bass: G, D, C with an A minor triad, A minor).
// The same map in C for the Hohner Club (songs/club-br/boi.barroso.club.abcx) has the same
// notes and chords a fourth higher. No independent second transcription was found.
// Repeats: the map marks ":|:" after Parte I's first ending, so Parte II is repeated from the
// second-ending bar, its second time ending "B2 dc" into the D.S.; after the D.S. Parte I is
// played once (to its second ending) and Parte II once, to the coda, which is the same bar as
// the second ending. The D bars are written V7: the tune puts C over them.
// PD: traditional (Rio Grande do Sul).
pub(super) const BOI_BARROSO: Song = Song {
    name: "Boi barroso",
    origin: "Traditional, Brazil (Rio Grande do Sul)",
    feel: "Canção gaúcha",
    genre: Genre::Folklore,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Backbeat,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    band: ACCORDION_BAND,
    style: "Polka",
    parts: &[
        Part::new("Pickup", "-:12 D5:2 C5:2", "V7"),
        Part::new(
            "Part I",
            "B4:2 B4 B4 C5 D5 D5 D5 C5 | B4 E5 E5 D5 C5:4 C5:2 B4:2 | \
             A4 A4 A4 B4 C5 C5 C5 B4 | A4 D5 D5 C5 B4:4 D5:2 C5:2",
            "I | I V7 | V7 | V7 I",
        ),
        Part::new(
            "Part I end",
            "B4:2 B4 B4 C5 D5 D5 D5 C5 | B4 E5 E5 D5 C5:4 C5:2 B4:2 | \
             A4 A4 A4 B4 C5 C5 C5 B4 | A4 D5 D5 C5 B4:4 -:4",
            "I | I V7 | V7 | V7 I",
        ),
        Part::new(
            "Part II",
            "-:2 B4 C5 D5 E5:4 F#5:4 | -:2 E5 D5 C5 B4:4 D5:4 | \
             -:2 D5 C5 B4 A4:4 C5:4 | -:2 D5 D5 C5 B4:4 -:4",
            "I IV6 | ii I | I V7 | V7 I",
        ),
        Part::new(
            "Part II into D.S.",
            "-:2 B4 C5 D5 E5:4 F#5:4 | -:2 E5 D5 C5 B4:4 D5:4 | \
             -:2 D5 C5 B4 A4:4 C5:4 | -:2 D5 D5 C5 B4:4 D5:2 C5:2",
            "I IV6 | ii I | I V7 | V7 I",
        ),
    ],
    form: &[
        "Pickup",
        "Part I",
        "Part I end",
        "Part II",
        "Part II into D.S.",
        "Part I end",
        "Part II",
    ],
    drums_from: 1,
    about: "A gaúcho folk song from Rio Grande do Sul about the ox called Barroso, in G major. The \
            first part answers three 2/4 bars of G with four of D7; the second part climbs to F sharp \
            over C with an A minor triad in the accordion's bass, C6, goes on to A minor and \
            settles back through D7 to G. Each bar here holds two bars of 2/4.",
};

/// Bars 1-12 of the lead sheet, in G major: the riff. Bar 1 opens with
/// percussive x-heads, rested here, and the band waits for the riff's first
/// chord; then Am7 and D7 turn to G four times, D7 moves to Dm7 and G7, C to
/// Cm7 and F7, B flat back to G, and Am7, D7 lead to the verse. Accents as
/// the sheet marks them (bars 1 and 5) and a staccato F sharp in bar 12.
const CANDOMBE_PARA_GARDEL_INTRO_TUNE: &str = "-:4 -:1 D5:1 D5:1 D5:1 G5:1 G5:2 D5:1 G5:2 D5:1 >A5:1 | \
     _:2 -:2 -:1 D5:1 D5:1 D5:1 A5:1 A5:2 D5:1 A5:2 D5:1 B5:1 | \
     -:4 -:1 D5:1 E5:1 F#5:1 G5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 | \
     D5:3 C5:1 -:1 C5:1 D5:1 E5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 E5:1 | \
     D5:4 -:1 D5:1 D5:1 D5:1 G5:1 G5:2 D5:1 G5:2 D5:1 >A5:1 | \
     _:2 -:2 -:1 D5:1 D5:1 D5:1 A5:1 A5:2 D5:1 A5:2 D5:1 B5:1 | \
     -:4 -:1 D5:1 E5:1 F#5:1 G5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 | \
     D5:3 C5:1 -:1 C5:1 D5:1 E5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 E5:1 | \
     D5:4 -:1 F5:1 F5:1 F5:1 F5:1 F5:1 F5:1 F5:1 F5:1 A5:1 G5:1 F5:1 | \
     G5:3 E5:1 -:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 G5:1 F5:1 Eb5:1 | \
     F5:1 -:1 -:2 -:1 D5:1 E5:1 F#5:1 G5:1 D5:1 E5:1 F#5:1 G5:1 A5:1 B5:1 C6:1 | \
     E6:2 -:1 D6:1 C6:1 B5:1 A5:1 -:1 -:1 G5:1 G5:1 G5:1 F#5:2. -:1 A5:1";
const CANDOMBE_PARA_GARDEL_INTRO_CHORDS: &str = "V... | ii7 V7 | I | ii7 V7 | I | ii7 V7 | I | ii7 V7 | \
     I I v7 I7 | IV IV iv7 bVII7 | bIII I | ii7 V7";

/// Bars 13-20, the first time through the A section: the pickup bar over G
/// (the riff's last note, a staccato G, then the first line recited on B flat
/// and tied over into the verse), then the verse in G minor, Gm, Cm7, B flat
/// minor 9, A half-diminished to D7(b9), and Gm with F in the bass, E flat
/// and D7, the bass falling G, F, E flat, D; and the first ending's first bar.
const CANDOMBE_PARA_GARDEL_VERSE_TUNE: &str = "G5:2. -:1 Bb4:1 -:1 Bb4:1 -:1 Bb4:1 Bb4:1 Bb4:2 Bb4:1 A4:2 G4:1 G4:1 | \
     _:2 -:2 -:4 -:8 | \
     -:3 Eb5:1 -:1 Eb5:1 -:1 Eb5:1 Eb5:1 Eb5:2 Eb5:1 D5:2 C5:1 C5:1 | _:4 -:4 -:8 | \
     -:3 D5:1 -:1 D5:1 -:1 D5:1 D5:1 F#5:1 -:1 D5:1 D5:2 C5:1 Bb4:1 | \
     _:1 -:1 -:2 -:1 Bb4:1 Bb4:1 Bb4:1 Bb4:1 D5:1 -:1 Bb4:1 Bb4:2 A4:1 G4:1 | -:16 | \
     -:3 D4:1 _:1 G4:2 A4:1 Bb4:2 A4:2 G4:2 F#4:2";
const CANDOMBE_PARA_GARDEL_VERSE_CHORDS: &str =
    "I | i | iv7 | biii9 | iiø7 V7b9 | i Gm/F | bVI V7 | i";

/// The first ending's second bar (bar 21, the pickup line again, still over
/// G minor), then the verse again (bars 14-19) and the second ending, bar 22.
const CANDOMBE_PARA_GARDEL_VERSE_2_TUNE: &str = "G4:2 -:1 Bb4:1 -:1 Bb4:1 -:1 Bb4:1 Bb4:1 Bb4:2 Bb4:1 A4:2 G4:1 G4:1 | \
     _:2 -:2 -:4 -:8 | \
     -:3 Eb5:1 -:1 Eb5:1 -:1 Eb5:1 Eb5:1 Eb5:2 Eb5:1 D5:2 C5:1 C5:1 | _:4 -:4 -:8 | \
     -:3 D5:1 -:1 D5:1 -:1 D5:1 D5:1 F#5:1 -:1 D5:1 D5:2 C5:1 Bb4:1 | \
     _:1 -:1 -:2 -:1 Bb4:1 Bb4:1 Bb4:1 Bb4:1 D5:1 -:1 Bb4:1 Bb4:2 A4:1 G4:1 | -:16 | -:16";
const CANDOMBE_PARA_GARDEL_VERSE_2_CHORDS: &str =
    "i | i | iv7 | biii9 | iiø7 V7b9 | i Gm/F | bVI V7 | I";

/// After the D.S.: the pickup bar 13, the verse once (bars 14-19) and the
/// second ending, bar 22.
const CANDOMBE_PARA_GARDEL_VERSE_3_TUNE: &str = "G5:2. -:1 Bb4:1 -:1 Bb4:1 -:1 Bb4:1 Bb4:1 Bb4:2 Bb4:1 A4:2 G4:1 G4:1 | \
     _:2 -:2 -:4 -:8 | \
     -:3 Eb5:1 -:1 Eb5:1 -:1 Eb5:1 Eb5:1 Eb5:2 Eb5:1 D5:2 C5:1 C5:1 | _:4 -:4 -:8 | \
     -:3 D5:1 -:1 D5:1 -:1 D5:1 D5:1 F#5:1 -:1 D5:1 D5:2 C5:1 Bb4:1 | \
     _:1 -:1 -:2 -:1 Bb4:1 Bb4:1 Bb4:1 Bb4:1 D5:1 -:1 Bb4:1 Bb4:2 A4:1 G4:1 | -:16 | -:16";
const CANDOMBE_PARA_GARDEL_VERSE_3_CHORDS: &str =
    "I | i | iv7 | biii9 | iiø7 V7b9 | i Gm/F | bVI V7 | I";

/// Bars 23-26, section B, in G major over a D pedal: C/D, B/D, G/D, and D
/// flat 7 sliding to the bridge's C minor 7. The sheet writes only chord
/// slashes here, so the tune rests. Bars 23 and 25 are hits: the chord on
/// the beat, again a sixteenth before beat 2 (tied over it) and a sixteenth
/// before beat 3, written as pushes.
const CANDOMBE_PARA_GARDEL_REFRAIN_TUNE: &str = "-:16 | -:16 | -:16 | -:16";
const CANDOMBE_PARA_GARDEL_REFRAIN_CHORDS: &str =
    "C/D ^^C/D ^^B/D B/D | G/D | C/D ^^C/D ^^B/D B/D | G/D G/D G/D bV7";

/// Bars 27-33, chord slashes only: Cm7 and F7 lead to B flat major 7 and G
/// minor twice, then Am7, D7 back to G, a rising Am7, Bm7, and Cm, F7, B flat,
/// E flat, Am, D7.
const CANDOMBE_PARA_GARDEL_BRIDGE_TUNE: &str = "-:16 | -:16 | -:16 | -:16 | -:16 | -:16 | -:16";
const CANDOMBE_PARA_GARDEL_BRIDGE_CHORDS: &str = "iv7 bVII7 | bIIImaj7 i | iv7 bVII7 | \
     bIII6 bIII6 ii7 V7 | I ii7 | iii7 iii7 iv bVII7 | bIII bVI ii V7";

/// Bar 34, the link back to the riff (its tie runs into the segno bar), then
/// "Del segno": bars 2-12 of the introduction.
const CANDOMBE_PARA_GARDEL_LINK_TUNE: &str = "B4:4 -:1 D5:1 D5:1 D5:1 G5:1 G5:2 D5:1 G5:2 D5:1 A5:1 | \
     _:2 -:2 -:1 D5:1 D5:1 D5:1 A5:1 A5:2 D5:1 A5:2 D5:1 B5:1 | \
     -:4 -:1 D5:1 E5:1 F#5:1 G5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 | \
     D5:3 C5:1 -:1 C5:1 D5:1 E5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 E5:1 | \
     D5:4 -:1 D5:1 D5:1 D5:1 G5:1 G5:2 D5:1 G5:2 D5:1 >A5:1 | \
     _:2 -:2 -:1 D5:1 D5:1 D5:1 A5:1 A5:2 D5:1 A5:2 D5:1 B5:1 | \
     -:4 -:1 D5:1 E5:1 F#5:1 G5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 | \
     D5:3 C5:1 -:1 C5:1 D5:1 E5:1 F#5:1 E5:1 D5:1 C5:1 B4:1 A4:1 G4:1 E5:1 | \
     D5:4 -:1 F5:1 F5:1 F5:1 F5:1 F5:1 F5:1 F5:1 F5:1 A5:1 G5:1 F5:1 | \
     G5:3 E5:1 -:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 G5:1 F5:1 Eb5:1 | \
     F5:1 -:1 -:2 -:1 D5:1 E5:1 F#5:1 G5:1 D5:1 E5:1 F#5:1 G5:1 A5:1 B5:1 C6:1 | \
     E6:2 -:1 D6:1 C6:1 B5:1 A5:1 -:1 -:1 G5:1 G5:1 G5:1 F#5:2. -:1 A5:1";
const CANDOMBE_PARA_GARDEL_LINK_CHORDS: &str = "I | ii7 V7 | I | ii7 V7 | I | ii7 V7 | I | ii7 V7 | \
     I I v7 I7 | IV IV iv7 bVII7 | bIII I | ii7 V7";

/// Bars 35-42, the coda: four bars of vamp (G, Cm7, F7, then B flat, E
/// flat, Am, D7), slashes only, suddenly pianissimo and growing to forte;
/// then the closing run up to E, a last G struck with an accent, and the
/// band's final stop.
const CANDOMBE_PARA_GARDEL_CODA_TUNE: &str = "-:8 !pp -:8 | -:16 | !< -:12 !f -:4 | -:16 | \
     -:4 -:1 D5:1 E5:1 F#5:1 G5:1 D5:1 E5:1 F#5:1 G5:1 A5:1 B5:1 C6:1 | \
     E6:2 -:2 D6:1 C6:1 B5:1 A5:1 -:1 G5:1 G5:1 G5:1 F#5:2 -:1 A5:1 | \
     G5:2. -:1 D5:1 >G5:2 -:2 -:8 | -:16";
const CANDOMBE_PARA_GARDEL_CODA_CHORDS: &str = "I I iv7 bVII7 | bIII bVI ii V7 | I I iv7 bVII7 | \
     bIII bVI ii V7 | I | I | I I. I... I... | I...";

// Source: lead sheet attached by the owner, in G, as written (two pages, 42 bars, "Rada";
// melody with chord symbols, sections A and B, segno, coda). Melody: bars 1-22, 34 and
// 39-42, at their written pitch; bar 1's percussive x-heads are rests here. Sections B
// (bars 23-33) and the coda vamp (bars 35-38) are chord slashes only, so the tune rests
// there. Chords: the sheet's symbols, as numerals in the part's key; the slash chords
// (C/D, B/D, G/D, Gm/F) are written by name to keep their bass. Bars that print no symbol
// keep the chord before (bars 21, 40-42); bar 1 has none and the band waits for the riff.
// Road map as printed: intro, A section with its repeat and endings, B, bar 34, D.S. to
// bar 2, the A section once with the second ending, B to the coda sign, coda.
// Composer: Rubén Rada, "Tengo un candombe para Gardel", on his 1985 album "La yapla mata"
// (https://es.wikipedia.org/wiki/La_yapla_mata credits the song to Rada alone;
// https://www.discogs.com/release/11999031-Ruben-Rada-La-Yapla-Mata credits Rada as composer).
// Tempo: the recording's, about 124 a minute
// (https://chordu.com/chords-tabs-ruben-rada-candombe-para-gardel-id__g29SyYizHg).
// Copyright: a modern song by Rubén Rada (1985), not public domain; added at the owner's request.
pub(super) const CANDOMBE_PARA_GARDEL: Song = Song {
    name: "Candombe para Gardel",
    origin: "Rubén Rada, 1985",
    feel: "Candombe",
    genre: Genre::Folklore,
    tonic: G,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 124.0,
    comping: Comping::Offbeat,
    bass: Bass::Tresillo,
    rhythm: "Candombe",
    band: CANDOMBE,
    style: "Candombe",
    parts: &[
        Part::new(
            "Intro",
            CANDOMBE_PARA_GARDEL_INTRO_TUNE,
            CANDOMBE_PARA_GARDEL_INTRO_CHORDS,
        )
        .in_key(G, Major),
        // The pickup bar 13 begins this part so that its last G stays tied
        // into the verse's first bar; the same for bar 21 in "Verse 2".
        Part::new(
            "Verse",
            CANDOMBE_PARA_GARDEL_VERSE_TUNE,
            CANDOMBE_PARA_GARDEL_VERSE_CHORDS,
        ),
        Part::new(
            "Verse 2",
            CANDOMBE_PARA_GARDEL_VERSE_2_TUNE,
            CANDOMBE_PARA_GARDEL_VERSE_2_CHORDS,
        ),
        Part::new(
            "Refrain",
            CANDOMBE_PARA_GARDEL_REFRAIN_TUNE,
            CANDOMBE_PARA_GARDEL_REFRAIN_CHORDS,
        )
        .in_key(G, Major),
        Part::new(
            "Bridge",
            CANDOMBE_PARA_GARDEL_BRIDGE_TUNE,
            CANDOMBE_PARA_GARDEL_BRIDGE_CHORDS,
        )
        .in_key(G, Major),
        // Bar 34 and the riff from the segno: one part, so that bar 34's
        // last A stays tied into bar 2.
        Part::new(
            "Link",
            CANDOMBE_PARA_GARDEL_LINK_TUNE,
            CANDOMBE_PARA_GARDEL_LINK_CHORDS,
        )
        .in_key(G, Major),
        Part::new(
            "Verse 3",
            CANDOMBE_PARA_GARDEL_VERSE_3_TUNE,
            CANDOMBE_PARA_GARDEL_VERSE_3_CHORDS,
        ),
        Part::new(
            "Coda",
            CANDOMBE_PARA_GARDEL_CODA_TUNE,
            CANDOMBE_PARA_GARDEL_CODA_CHORDS,
        )
        .in_key(G, Major),
    ],
    // The sheet's road map: bars 1-12, 13-21, 14-19 and 22, 23-33, 34 and
    // D.S. to bar 2 (2-12), 13, 14-19 and 22, 23-33 to the coda sign, 35-42.
    form: &[
        "Intro", "Verse", "Verse 2", "Refrain", "Bridge", "Link", "Verse 3", "Refrain", "Bridge",
        "Coda",
    ],
    drums_from: 0,
    about: "A candombe in G that keeps changing mode. The riff runs over Am7, D7 and G, \
            then turns D7 into Dm7 and G7, C into Cm7 and F7, and B flat back to G. The \
            verse is in G minor: Cm7, a B flat minor 9 and A half-diminished to D7(b9), and \
            Gm over a bass falling G, F, E flat, D. The B section returns to G major over a \
            D pedal, slides through D flat 7 and borrows B flat major (Cm7, F7, B flat maj7) \
            before Am7 and D7 bring back G. Listen for the sixteenth-eighth-sixteenth \
            syncopation on nearly every beat of the tune, the candombe drums' off-beat lilt.",
};

/// A minor for three bars, E7 for three, then D minor for "un grito de
/// cora-" and E7 from "-zón" through "¡Viva Perón! ¡Viva Perón!".
const LA_MARCHA_PERONISTA_VERSE_A_CHORDS: &str = "i | i | i | V7 | V7 | V7 | iv | V7 | V7";

/// A minor again, then A7 (the dominant of D minor) leads to D minor, and
/// E7 brings it home to A minor.
const LA_MARCHA_PERONISTA_VERSE_B_CHORDS: &str = "i | i | I7 | iv | iv | i | V7 | i";

/// E7 and A minor bar by bar, D minor for "gran conductor", and E7 to close.
const LA_MARCHA_PERONISTA_REFRAIN_CHORDS: &str = "V7 | i | V7 | i | iv | i | V7";

// Source: score attached by the owner (page images), arranged by Fede Bär,
// "Los Muchachos Peronistas", piano with the words under the right hand, in
// A minor, 4/4, quarter = 80, 26 bars: bars 1-24 between repeat signs, first
// ending bar 25, second ending bar 26, chord symbols Am E7 Dm A7 printed above
// the staff. The tune is the right hand's line with the words. The chords
// are the printed symbols, checked against the left hand's broken chords (A
// E C E, E B G# B, D A F A, and A E C# E in bar 12).
// Cross-checks: the chord sheet at https://acordes.lacuerda.net/hugo_del_carril/marcha_peronista
// (in F minor) has the same chords as the score, except that in the line
// "y como siempre daremos un grito de corazón" it puts B flat minor and D flat
// (iv and bVI) where the score has E7, E7, D minor. The solfège notes at
// https://www.notasparamelodica.com/como-tocar-marcha-peronista-los-post (no
// rhythm) give the same tune, except "la" where the score has G sharp in bar 5
// and "fa mi re" where the score has D, D, D on "gran con-duc-" (bar 22).
// The endings' upper voice (a rest, then three short A5s with no words, a
// band's figure over the sung A4) is left out: the tune is the sung line.
// The score prints no dynamics, articulations or tempo changes; the second
// ending's held left hand is marked as a hold on its chord.
// Authorship: SADAIC lists it as anonymous. Todotango (Néstor Pinsón and
// Ricardo García Blaya) traces the verse's music to Juan Raimundo Streiff's
// march for the Barracas Juniors club (late 1920s, words by "el Turco"
// Mufarri) and the refrain to an anonymous carnival murga tune; the words as
// sung were reworked in 1948 by Rafael Lauría and Oscar Ivanissevich.
// https://www.todotango.com/english/history/chronicle/178/The-origin-of-the-Peronist-march-rooted-in-sports-and-carnival-chants/
// Best-known recording: Hugo del Carril, 1949 (Todotango; English and Spanish
// Wikipedia). It was not heard for this demo: the key, the tempo and the form
// are those of the score.
// Copyright: unclear (SADAIC registers it as anonymous; no death date was found
// for Juan Raimundo Streiff, and the words, which this demo does not use, are
// credited to Rafael Lauría and Oscar Ivanissevich, who died in 1976); added at
// the owner's request.
pub(super) const LA_MARCHA_PERONISTA: Song = Song {
    name: "La marcha peronista",
    origin: "Anonymous (music traced to Juan Raimundo Streiff), Argentina, 1948",
    feel: "Marcha",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Offbeat,
    bass: Bass::RootFifth,
    rhythm: "March",
    band: BRASS_BAND,
    style: "Marchinha",
    parts: &[
        // "Los muchachos peronistas ... un grito de corazón: ¡Viva Perón!
        // ¡Viva Perón!" (bars 1-9).
        Part::new(
            "Verse A",
            "E5:4 C5:2 A4 E5:4 C5:2 A4 | E5:8 C5 | E5:4 C5:2 A4 E5 E5 D5 C5 | D5:8 B4 | \
             D5:4 B4:2 G#4 D5:4 B4:2 G#4 | D5:8 B4 | F5:4 F5:2 F5 F5:4 E5:2 D5 | \
             E5:2 F5 E5 D5 E5 F5 E5 D5 | E5:16",
            LA_MARCHA_PERONISTA_VERSE_A_CHORDS,
        ),
        // "Por ese gran argentino ... combatiendo al capital" (bars 10-17).
        // It ends with the refrain's pickup "¡Pe-rón, Pe-".
        Part::new(
            "Verse B",
            "E5:4 C5:2 A4 E5:4 C5:2 A4 | E5:8 C5 | E5:4 E5:2 E5 G5:4 F5:2 E5 | \
             F5:12 D5:2 E5 | F5:4 F5 E5 D5 | E5:4 E5:8 C5:2 D5 | E5:4 D5 C5 B4 | \
             A4:8 _:2 E5 E5 E5",
            LA_MARCHA_PERONISTA_VERSE_B_CHORDS,
        ),
        // "rón, qué grande sos! ¡Mi general, cuánto valés! ¡Perón, Perón,
        // gran conductor, sos el primer trabaja-" (bars 18-24).
        Part::new(
            "Refrain",
            "E5:8 _:2 D5 C5 B4 | C5:8 _:2 E5 E5 E5 | E5:8 _:2 D5 C5 B4 | \
             C5:8 _:2 A4 A4 A4 | F5:8 _:2 D5 D5 D5 | E5:8 _:2 C5 C5 D5 | E5:4 D5 C5 B4",
            LA_MARCHA_PERONISTA_REFRAIN_CHORDS,
        ),
        // "-dor!", then back to the start for the second verse (bar 25).
        Part::new("First ending", "A4:8 -:8", "i"),
        // "-dor!" for the last time (bar 26): the left hand stops its broken
        // chords and holds a whole-note A, so the band holds the last chord.
        Part::new("Second ending", "A4:8 -:8", "i.."),
    ],
    form: &[
        "Verse A",
        "Verse B",
        "Refrain",
        "First ending",
        "Verse A",
        "Verse B",
        "Refrain",
        "Second ending",
    ],
    drums_from: 0,
    about: "A street march in A minor, sung in Argentina since 1948. The verse rocks between \
            A minor and E7, the dominant, with D minor (iv) just before \"¡Viva Perón!\"; in its \
            second half A7, the dominant of D minor, leads to D minor. The refrain \"¡Perón, \
            Perón!\" changes chord every bar, E7 to A minor and, under \"gran conductor\", D \
            minor to A minor, each line starting on the last three eighths of the bar before.",
};
