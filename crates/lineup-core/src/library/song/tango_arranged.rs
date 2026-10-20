//! Tangos and valses written out from arrangements, note for note: the
//! tune, which the singer sings, on the bandoneón, and everything else the
//! arrangement plays, the pianist's two hands or the band, on channels of
//! their own. The chords on the chord sheet are the ones the arrangement
//! plays.

use super::*;

/// The tune on a bandoneón, over the piano of the arrangement.
const BANDONEON_AND_PIANO: Band = Band::new(
    gm::TANGO_ACCORDION,
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_BASS,
);

// Written out from a piano arrangement in B minor (MIDI, 88 bars of 2/4, written here two to a bar of 4/4).
pub(super) const MI_NOCHE_TRISTE: Song = Song {
    name: "Mi noche triste",
    origin: "Samuel Castriota and Pascual Contursi, 1917",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: B,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: BANDONEON_AND_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-4 of the file.
        Part::new(
            "Intro",
            "-:2 D4:1 F#4 D5:2 C#5:1 B4 (3 B4:1 C#5 B4 A#4:5 -:1 | \
            -:3 E4:1 B4:3 A4:1 (3 G4:1 A4 G4 F#4:4 C#5:2 | \
            E5 D5:1 C#5 B4:2 A#4 B4 F#4:3 -:1 F#4:2 | G4:1 B4:2 G4:1 D4:2 E4 F#4 -:6",
            "Bm F#7 | F#7 Bm | A7 D | G F#m",
        )
        .with_left("B1+B2:2 -:6 F#2+F#3:3 G2+G3:1 E2+E3 D2+D3 C#2+C#3 B1+B2 | \
            A#1+A#2:3 C#3+E3+F#3:1 C#3+E3+F#3:2 F#2+F#3 B1+B2:3 D3+F#3+B3:1 D3+F#3+B3:3 -:1 | \
            A1+A2:3 C#3+E3+G3+A3:1 C#3+E3+G3+A3:2 C#2+C#3 D2+D3:3 D3+F#3+A3:1 D3+F#3+A3:3 -:1 | \
            G2+G3:3 B2+D3+G3:1 B2+D3+G3:2 G2+G3 F#2+F#3+A3+C#4 -:6 & -:1 B3 -:14 | \
            -:1 A#3 C#4 -:13 | -:8 D4+F#4:2 D4:3 -:1 D4:2 | -:16"),
        // Bars 5-8 of the file.
        Part::new(
            "A",
            "-:2 E4:1 F#4 G4 G4 F#4 E4 E4 D4 D4 E4 F#4 F#4 E4 D4 | \
            D4 -:2 D4:1 E4 E4 D#4 E4 (3 G4:1 A4 G4 F#4:5 -:1 | \
            -:2 E4:1 F#4 G4 G4 F#4 E4 E4 D4 D4 F#4 F#4 B4 F#4 D4 | D4 -:2 D4:1 E4 E4 -:6 B4:2 -",
            "Em Bm | F#7 Bm | Em Bm | A F#7 Bm Bm",
        )
        .with_left("E2+E3:3 -:1 G2+G3:3 -:1 B1+B2:3 -:1 D2+D3:3 -:1 | \
            A#1+A#2:3 -:1 F#1+F#2:3 -:1 B1+B2:3 D3+F#3+B3:1 D3+F#3+B3:3 -:1 | \
            E2:2 E3 G2 G3 B1 B2 D2 D3 | A1 A2 F#1 F#2 B1+B2+B3 - B1+B2+D4+F#4 - & -:16 | \
            -:1 C#4 C#4 -:13 | -:16 | -:1 C#4 C#4 -:3 C#4:1 A#3 -:8"),
        // Bars 9-12 of the file.
        Part::new(
            "B",
            "-:2 F#4:1 F#4 B4 B4 B4 A#4 C#5:2 B4:3 -:1 B4:2 | \
            -:1 A#4 A#4 A#4 C#5:2 B4:1 A#4 B4:2 F#4:3 -:1 F#4:2 | \
            -:1 E4 F#4 G4 F#4:2 E4 D4 F#4:3 -:1 F#4:2 | \
            - D4:1 D#4 E4:2 D#4:1 E4 G4:2 F#4:3 -:1 F#4:2",
            "Bm | Em Bm | F#7 Bm | F#7 Bm",
        )
        .with_left("B1+B2:3 B2+D3+F#3+D4:1 B2+D3+F#3:2 F#2+F#3 B1+B2:3 B2+D3+F#3:1 B2+D3+F#3:3 -:1 | \
            E2+E3:3 B2+E3+G3+E4:1 B2+E3+G3+E4:2 G2+G3 B1+B2:3 D3+F#3+B3:1 D3+F#3+B3:3 -:1 | \
            F#2+F#3:3 C#3+E3+F#3+A#3+C#4:1 C#3+E3+F#3+A#3:3 A#1+A#2:1 B1+B2:3 B3:2 -:1 B3+D4:2 | \
            F#2+F#3:3 C#3+E3+F#3+A#3:1 C#3+E3+F#3+A#3:3 A#1+A#2+A#3:1 B1+B2:3 B3:2 -:1 B3:2 & -:2 D4:1 - D4+F#4 D4+F#4 D4+F#4 D4+F#4 D4+F#4:2 D4+F#4:3 -:1 D4+F#4:2 | \
            -:1 E4 E4 -:3 E4:1 E4 D4:2 D4:3 -:1 D4:2 | -:1 C#4 C#4 - C#4:2 C#4 B3 D4:3 - | \
            -:1 A#3+C#4 A#3 - A#3:2 A#3:1 - B3:2 B3 D3+F#3+B3:3 -:1 & -:16 | - | \
            -:10 B3:2 D3+F#3+B3:3 -:1 | -:11 D3+F#3:1 -:4 & -:16 | - | -:11 D3+F#3:1 -:4 | -:16"),
        // Bars 13-14 of the file.
        Part::new(
            "A end",
            "-:2 E4:1 F#4 G4 G4 F#4 E4 E4 D4 D4 F#4 F#4 B4 F#4 D4 | D4 -:2 D4:1 E4 E4 -:6 B4:2 -",
            "Em Bm | F#7 Bm",
        )
        .with_left("E2:2 E3 G2 G3 B1 B2 D2 D3 | A#1 A#2 F#1 F#2 B1+B2+B3 - B1+B2+D4+F#4 - & -:16 | \
            -:1 C#4 C#4 -:3 C#4:1 A#3 -:8"),
        // Bars 25-28 of the file.
        Part::new(
            "A into C",
            "-:2 E4:1 F#4 G4 G4 F#4 E4 E4 D4 D4 E4 F#4 F#4 E4 D4 | \
            D4 -:2 D4:1 E4 E4 D#4 E4 (3 G4:1 A4 G4 F#4:5 -:1 | \
            -:2 E4:1 F#4 G4 G4 F#4 E4 E4 D4 D4 F#4 F#4 B4 F#4 D4 | D4 -:2 D4:1 E4 E4 -:6 B4:2 A4",
            "Em Bm | F#7/A# Bm | Em Bm | A F#7 Bm A7/C#",
        )
        .with_left("E2+E3:3 -:1 G2+G3:3 -:1 B1+B2:3 -:1 D2+D3:3 -:1 | \
            A#1+A#2:3 -:1 F#1+F#2:3 -:1 B1+B2:3 D3+F#3+B3:1 D3+F#3+B3:3 -:1 | \
            E2:2 E3 G2 G3 B1 B2 D2 D3 | \
            A1 A2 F#1 F#2 B1+B2+B3 - B1+B2+D4+F#4 C#2+C#3+C#4+E4+G4 & -:16 | -:1 C#4 C#4 -:13 | \
            -:16 | -:1 C#4 C#4 -:3 C#4:1 A#3 -:8"),
        // Bars 29-32 of the file.
        Part::new(
            "C",
            "-:2 D5:1 D5 C#5 C#5 B4 B4 B4:3 -:1 A4:3 -:1 | \
            -:2 C#5:1 C#5 B4 B4 A4 A4 G4:3 -:1 F#4:3 -:1 | \
            -:2 D5:1 D5 C#5 C#5 B4 B4 B4:3 -:1 A4:3 -:1 | \
            -:2 C#5:1 C#5 B4 B4 A4 A4 G4:3 -:1 F#4:3 -:1",
            "D A7 | A7/C# A7 D D | D A7 | A7/C# A7 D D",
        )
        .in_key(D, Major)
        .with_left("D2+D3:2 D4+F#4:1 D4+F#4 D4+F#4 D4+F#4 D4+F#4 D4+F#4 A1+A2+C#4+E4+G4:3 A2+C#3+E3+G3:1 C#4+E4+G4:3 -:1 | \
            C#2+C#3:3 A2+C#3+E3+G3+C#4+E4:1 A2+C#3+E3+G3:2 A1+A2 D2+D3+D4:3 D3+F#3+A3:1 D3+F#3+A3+D4:3 -:1 | \
            D2+D3:2 D4+F#4:1 D4+F#4 D4+F#4 D4+F#4 D4+F#4 D4+F#4 A1+A2+C#4+E4+G4:3 A2+C#3+E3+G3:1 C#4+E4+G4:3 -:1 | \
            C#2+C#3:3 A2+C#3+E3+G3+C#4+E4:1 A2+C#3+E3+G3:2 A1+A2 D2+D3+D4:3 D3+F#3+A3:1 D3+F#3+A3+D4:3 -:1 & -:12 A2+C#3+E3+G3:2 A1+A2 | \
            - C#4+E4:1 - C#4+E4 C#4+E4 C#4+E4 C#4+E4 -:8 | -:12 A2+C#3+E3+G3:2 A1+A2 | \
            - C#4+E4:1 - C#4+E4 C#4+E4 C#4+E4 C#4+E4 -:8"),
        // Bars 33-36 of the file.
        Part::new(
            "C end",
            "-:1 F#4 F#4 F#4 F#4:2 G#4:1 B4 A#4:3 -:1 A#4:3 -:1 | \
            - F#4 F#4 F#4 G#4:2 A4:1 A#4 B4:3 -:1 B4:3 -:1 | \
            -:4 D4:2 E4:1 E#4 F#4:3 -:1 F#4:3 -:1 | -:12 B4:2 -",
            "Bm6/D F#7 | F#7/A# F#7 Bm Bm | Bm Bm F#7/C# F#7 | F#7 F#7/A# Bm Bm",
        )
        .with_left("D2+D3:2 -:6 F#1+F#2+E4:3 F#2+A#2+C#3+E3:1 F#2+A#2+C#3+E3+E4:3 -:1 | \
            A#1+A#2:3 F#2+A#2+C#3+E3:1 F#2+A#2+C#3+E3:3 -:1 B1+B2+D4+F#4:3 B2+D3+F#3:1 B2+D3+F#3+D4+F#4:3 -:1 | \
            B1+B2:3 B2+D3+F#3+C#4:1 B2+D3+F#3:3 -:1 C#2+C#3+A#3:3 F#2+A#2+C#3+E3:1 F#2+A#2+C#3+E3+A#3:3 -:1 | \
            F#1+F#2:3 F#2+A#2+C#3+E3+A#3:1 F#2+A#2+C#3+E3+A#3:2 A#1+A#2 B1+B2+B3 - B1+B2+D4+F#4 - & -:16 | \
            - | -:2 B3:1 -:13 | -:1 A#3 A#3 -:3 B3:1 C#4 -:8"),
        // Bars 41-44 of the file.
        Part::new(
            "Last A",
            "-:2 E4:1 F#4 G4 G4 F#4 E4 E4 D4 D4 E4 F#4 F#4 E4 D4 | \
            D4 -:2 D4:1 E4 E4 D#4 E4 (3 G4:1 A4 G4 F#4:5 -:1 | \
            -:2 E4:1 F#4 G4 G4 F#4 E4 E4 D4 D4 F#4 F#4 B4 F#4 D4 | \
            D4 -:2 D4:1 E4 E4 -:6 B5:3 -:1",
            "Em Bm | F#7 Bm | Em Bm | A F#7 Bm Bm",
        )
        .with_left("E2+E3:3 -:1 G2+G3:3 -:1 B1+B2:3 -:1 D2+D3:3 -:1 | \
            A#1+A#2:3 -:1 F#1+F#2:3 -:1 B1+B2:3 D3+F#3+B3:1 D3+F#3+B3:3 -:1 | \
            E2:2 E3 G2 G3 B1 B2 D2 D3 | A1 A2 F#1 F#2 B1+B2+B3 - B1+B2+B4+D5+F#5:3 -:1 & -:16 | \
            -:1 C#4 C#4 -:13 | -:16 | -:1 C#4 C#4 -:3 C#4:1 A#3 -:8"),
    ],
    form: &["Intro", "A", "B", "A end", "B", "A end", "Intro", "A into C", "C", "C end", "Intro", "Last A"],
    drums_from: 0,
    about: "The first tango with words that tell a story, \"Percanta que me amuraste\": a man \
            left alone talks to the woman who left him. Castriota wrote the tune as \"Lita\"; \
            Contursi's words and Gardel's record of 1917 started the tango canción. B minor, \
            with a strain in D major, the relative major, and back.",
};

// Written out from a piano arrangement in F sharp minor (MIDI, 50 bars of 2/4, here two to
// a bar of 4/4 after the first; a bar of 2/4 before the second strain keeps its tune on the
// downbeat). The tune is the file's own track over the piano.
pub(super) const SENTIMIENTO_GAUCHO: Song = Song {
    name: "Sentimiento gaucho",
    origin: "Francisco and Rafael Canaro, words by Juan Andrés Caruso, 1924",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: FSharp,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 84.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: BANDONEON_AND_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-1 of the file.
        Part::new(
            "Intro",
            "-:8",
            "F#m",
        )
        .in_time(TWO_FOUR)
        .with_left("F#2+A2+C#3+F#3:2 F#4:1 G#4 A4:2 B4:1 C#5"),
        // Bars 2-9 of the file.
        Part::new(
            "A",
            "F#5:4 D5 B4:6 C#5:1 D5 | F#5:4 C#5 A4:6 B4:1 C#5 | E#5:4 C#5 G#4:6 A4:1 B4 | \
            C#5:4 C#5 C#5:8 | F#5:4 D5 B4:6 C#5:1 D5 | E5:4 C#5 A4:6 B4:1 C#5 | \
            D5:4 B4 G#4:6 A4:1 B4 | C#5:4 A5 G#5:6 E#5:2",
            "Bm/F# Bm Bm Bm | F#m F#m/A F#m/A F#m/A | C#7/G# C#7/B C#7/B C#7/B | \
            F#m F#m F#m/A F#m/A | Bm E#dim7 E E/G# | A A7/G F#m F#m | C#7/E# C#7/E# C#7 C#7/E# | \
            F#m F#m C#7/E# C#7",
        )
        .with_left("F#2+B2+D3+F#3+D5:2 B2+D5:1 D3+C#5 B2+D3+F#3+D5:2 B2+D5:1 B2+D3+F#3+C#5 B2+D3+F#3+D5:2 B2+D5:1 D3+C#5 B2+D3+F#3+D5:2 B2+D3+F#3 | \
            F#2+A2+C#3+F#3+C#5 A2+C#5:1 C#3+B#4 A2+C#3+F#3+C#5:2 A2+C#5:1 C#3+B#4 A2+C#3+F#3+C#5:2 A2+C#5:1 C#3+B#4 A2+C#3+F#3+C#5:2 A2+C#3+F#3 | \
            G#2+B2+C#3+E#3+B4 B2+B4:1 C#3+A#4 B2+C#3+E#3+B4:2 B4:1 A#4 B2+C#3+E#3+B4:2 B2+B4:1 C#3+A#4 B2+C#3+E#3+B4:2 B2+C#3+E#3 | \
            F#2+F#3:4 G#2+G#3 A2+A3+C#4+F#4:2 F#4:1 G#4 A4 B4:2 C#5:1 | \
            B1+B2:3 D3+F#3+B3+C#5:1 D3+F#3+B3+D5:2 F#2+F#3+D5:1 E#2+E#3+C#5 E2+E3:3 E3+G#3+B3+E#5+E#6:1 E3+G#3+B3:2 G#2+G#3 | \
            A2+A3:3 E3+A3+C#4+B#4:1 E3+A3+C#4+C#5:2 G#2+G#3+C#5:1 G2+G3+B#4 F#2+F#3:3 F#3+A3+C#4+D#5+D#6:1 F#3+A3+C#4:2 F#2+F#3 | \
            E#2+E#3:3 E#3+G#3+B3+A#4:1 E#3+G#3+B3+B4:2 E#2+E#3 C#2+C#3:3 C#3+E#3+G#3+C#5+C#6:1 C#3+E#3+G#3:2 E#2+E#3 | \
            F#2+F#3+A4+A5 F#3+C#5:1 A3+B#4 F#3+A3+C#4+C#5:2 F#2+F#3 E#2+E#3+G#4 E#3+B4:1 G#3+A#4 E#3+G#3+B3+B4:2 C#2+C#3 & -:14 C#5:1 B#4 | \
            -:14 B4:1 A#4 | -:14 A4:1 G#4 | C#4+F#4+A4:2 A4:1 G#4 B3+C#4+A4:2 A4:1 G#4 -:8 | \
            D5:2 D5:1 -:5 D5:1 F#5+F#6:2 -:1 F#5+F#6 E5+E6:2 D5+D6:1 | \
            C#5+C#6:2 C#5:1 -:5 C#5:1 E5+E6:2 -:1 E5+E6 D5+D6:2 C#5+C#6:1 | \
            B4+B5:2 B4:1 -:3 B4:1 A#4 B4 D5+D6:2 -:1 D5+D6 C#5+C#6:2 B4+B5:1 | \
            -:6 B4:1 A4 -:6 G#4:1 E#4"),
        // Bars 10-11 of the file.
        Part::new(
            "Piano",
            "F#5:2 C#5 F#5:1 -:11 | -:16",
            "F#m E | A/C# E7/B",
        )
        .with_left("F#2+F#3+F#4:2 C#3 F#2+F#3+A4+C#5+F#5 F#4:1 E#4 E4:6 -:1 D4 | \
            C#4:6 -:1 C#4 B3:6 -:1 B3 & -:9 E5:1 G#5 B5 E6:2 - | \
            -:1 E5 A5 C#6 E6:2 -:3 E5:1 G#5 B5 D6:2 - & -:13 E4:2 -:1 | -:5 D4:2 -:6 C#4:2 -:1"),
        // Bars 12-12 of the file.
        Part::new(
            "Piano, into B",
            "-:8",
            "A",
        )
        .in_time(TWO_FOUR)
        .with_left("A3:2 D#5:1 E5:1/2 F#5:1/4 G#5 F#5:1 E5 D#5 E5"),
        // Bars 13-19 of the file.
        Part::new(
            "B",
            "A4:6 B4:1 C#5 E5:4 D5 | C#5 B4 A4:3 F#4:1 E4:4 | A4:6 B4:1 C#5 E5:4 D5 | C#5 B4 A4:8 | \
            A5:4 G5 F#5:6 F5:2 | E5:6 E5:2 F#5:4 D5 | C#5 B4:3 G#4:1 A4:2 -:6",
            "A A/C# E7/B E7/D | E7/B E7/D A A/C# | A A/C# E7/B E7/D | E7/B E7/D A A | \
            A Gmaj7 D/F# D | A/E A/E G#dim7/D G#dim7/D | A E7/G# A A",
        )
        .in_key(A, Major)
        .with_left("A2+C#3+E3+A3:3 C#3+E3+A3+E5:1 C#3+E3+A3:2 C#3+E3+A3 B2+D3+E3+G#3+G#4 D3+E3+G#3+D#5:1 D3+E3+G#3+E5 D3+E3+G#3:2 D3+E3+G#3 | \
            B2+D3+E3+G#3:3 D3+E3+G#3+E5:1 D3+E3+G#3:2 D3+E3+G#3 A2+C#3+E3+A3:3 C#3+E3+A3+E5:1 C#3+E3+A3:2 C#3+E3+A3 | \
            A2+C#3+E3+A3:3 C#3+E3+A3+E5:1 C#3+E3+A3:2 C#3+E3+A3 B2+D3+E3+G#3+G#4 D3+E3+G#3+D#5:1 D3+E3+G#3+E5 D3+E3+G#3:2 D3+E3+G#3 | \
            B2+D3+E3+G#3:3 D3+E3+G#3+E5:1 D3+E3+G#3:2 D3+E3+G#3 A2+C#3+E3+A3+A4 D#5:1 E5 F#5 E5 D#5 E5 | \
            A2+A3:4 G2+G3 F#2+F#3:3 D3+F#3+A3+E5:1 D3+F#3+A3:2 D3+F#3+A3 | \
            E2+E3:3 E3+A3+C#4+D5:1 E3+A3+C#4:2 E3+A3+C#4 D3+F3+G#3+B3:8 | \
            A1+A2:4 G#1+G#2 A2+C#3+E3+A3+A4:2 D#5:1 E5 F#5 E5 D#5 E5 & F#5:1 E5 D#5 - D#5 E5 C#5 A4 -:7/2 F#5:1/4 G#5 F#5:1 E5 D#5 E5 | \
            F#5 E5 D#5 - D#5 E5 D5 B4 A4:2 D#5:1 -:1/2 F#5:1/4 G#5 F#5:1 E5 D#5 E5 | \
            F#5 E5 D#5 - D#5 E5 C#5 A4 -:7/2 F#5:1/4 G#5 F#5:1 E5 D#5 E5 | \
            F#5 E5 D#5 - D#5 E5 D5 B4 -:8 | \
            F#5:1 E5 D#5 E5 G5 F#5 G5 B5 F#5:2 D5:1 - F#5 D5 E5 F#5 | \
            E5:2 C#5:1 - E5 C#5 D5 E5 D5:2 B4:1 C#5 D5 B4 C#5 D5 | \
            C#5 B4 C#3+E3+A3:2 B4:1 A4 D3+E3+G#3:2 -:8 & -:16 | - | - | - | - | - | \
            -:2 A4:1 C#5 -:2 G#4:1 B4 -:8"),
        // Bars 20-26 of the file.
        Part::new(
            "B again",
            "A4:6 B4:1 C#5 E5:4 D5 | C#5 B4 A4:3 F#4:1 E4:4 | A4:6 B4:1 C#5 E5:4 D5 | C#5 B4 A4:8 | \
            A5:4 G5 F#5:6 F5:2 | E5:6 E5:2 F#5:4 D5 | C#5 B4:3 G#4:1 A4:2 E4 A4:4",
            "A A/C# E7/B E7/D | E7/B E7/D A A/C# | A A/C# E7/B E7/D | E7/B E7/D A A | \
            A Gmaj7 D/F# D | A/E A/E G#dim7/D G#dim7/D | A E7/G# A A",
        )
        .in_key(A, Major)
        .with_left("A2+C#3+E3+A3:3 C#3+E3+A3+E5:1 C#3+E3+A3:2 C#3+E3+A3 B2+D3+E3+G#3+G#4 D3+E3+G#3+D#5:1 D3+E3+G#3+E5 D3+E3+G#3:2 D3+E3+G#3 | \
            B2+D3+E3+G#3:3 D3+E3+G#3+E5:1 D3+E3+G#3:2 D3+E3+G#3 A2+C#3+E3+A3:3 C#3+E3+A3+E5:1 C#3+E3+A3:2 C#3+E3+A3 | \
            A2+C#3+E3+A3:3 C#3+E3+A3+E5:1 C#3+E3+A3:2 C#3+E3+A3 B2+D3+E3+G#3+G#4 D3+E3+G#3+D#5:1 D3+E3+G#3+E5 D3+E3+G#3:2 D3+E3+G#3 | \
            B2+D3+E3+G#3:3 D3+E3+G#3+E5:1 D3+E3+G#3:2 D3+E3+G#3 A2+C#3+E3+A3+A4 D#5:1 E5 F#5 E5 D#5 E5 | \
            A2+A3:4 G2+G3 F#2+F#3:3 D3+F#3+A3+E5:1 D3+F#3+A3:2 D3+F#3+A3 | \
            E2+E3:3 E3+A3+C#4+D5:1 E3+A3+C#4:2 E3+A3+C#4 D3+F3+G#3+B3:8 | \
            A1+A2:4 G#1+G#2 A2+A3+A4:2 E3 A2+A3+A4+C#5+E5+A5:4 & F#5:1 E5 D#5 - D#5 E5 C#5 A4 -:7/2 F#5:1/4 G#5 F#5:1 E5 D#5 E5 | \
            F#5 E5 D#5 - D#5 E5 D5 B4 A4:2 D#5:1 -:1/2 F#5:1/4 G#5 F#5:1 E5 D#5 E5 | \
            F#5 E5 D#5 - D#5 E5 C#5 A4 -:7/2 F#5:1/4 G#5 F#5:1 E5 D#5 E5 | \
            F#5 E5 D#5 - D#5 E5 D5 B4 -:8 | \
            F#5:1 E5 D#5 E5 G5 F#5 G5 B5 F#5:2 D5:1 - F#5 D5 E5 F#5 | \
            E5:2 C#5:1 - E5 C#5 D5 E5 D5:2 B4:1 C#5 D5 B4 C#5 D5 | \
            C#5 B4 C#3+E3+A3:2 B4:1 A4 D3+E3+G#3:2 -:8 & -:16 | - | - | - | - | - | \
            -:2 A4:1 C#5 -:2 G#4:1 B4 -:8"),
    ],
    form: &["Intro", "A", "Piano", "Piano, into B", "B", "B again"],
    drums_from: 0,
    about: "The Canaro brothers' tango won the 1924 contest of the Disco Nacional record \
            company; Caruso's words are a gaucho's lament in the city. The first strain sighs \
            down in F sharp minor; after two bars of piano alone, the second turns to A major, \
            the relative major, with the piano trilling high above the tune.",
};

// Written out from a piano arrangement in F sharp minor (MIDI, 89 bars of 2/4: a bar of
// pickup, then here two to a bar of 4/4). The tune is the top note of the right hand.
pub(super) const ZORRO_GRIS: Song = Song {
    name: "Zorro gris",
    origin: "Rafael Tuegols, words by Francisco García Jiménez, 1920",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: FSharp,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 82.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: BANDONEON_AND_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-1 of the file.
        Part::new(
            "Pickup",
            "-:5 F#4:1 F#4 G#4",
            "",
        )
        .in_time(TWO_FOUR),
        // Bars 2-9 of the file.
        Part::new(
            "A",
            "A4:3 A4:1/3 B4 C#5 B4:1 A#4 B4 B#4 C#5:2 C#5 -:1 C#5 D5 D#5 | \
            E5:3 D5:1/2 E5 D5:1 C#5 C#5 D5:1/3 C#5 D5 C#5:2 B4:3 F#6:1 F#6 G#6 | \
            E#6:3 C#6:1 C#6 B5 B5 A#5 B5 A#5 A5:2 F#4:1 A5 G#5 F#5 | \
            D5:3 F#5:1 F#5 E#5 F#5 G#5 E#5:2 -:3 F#4:1 F#4 G#4 | \
            A4:3 A4:1/3 B4 C#5 B4:1 A#4 B4 B#4 C#5:2 C#5 -:1 C#5 D5 D#5 | \
            E5:3 D5:1/2 E5 D5:1 C#5 C#5 D5:1/3 C#5 D5 C#5:2 B4 -:1 F#6 F#6 G#6 | \
            E#6:3 C#6:1 C#6 B5 B5 A#5 B5 A#5 A5:2 -:1 A5 G#5 F#5 | \
            A5:3 F#5:1 E#5 G#5 F#5 E#5 F#5:2 - F#5:1 F#4 F#4 G#4",
            "F#m C#7/E# F#m F#m | F#7/A# F#7/A# Bm G#m7b5/D | C#7 C#7/E# F#m F#m/C# | \
            D D C#7 C#7/G# | F#m C#7/E# F#m F#m | F#7/A# F#7/A# Bm G#m7b5/D | \
            C#7 C#7/E# F#m F#m/C# | F#m C#7 F#m/A F#m",
        )
        .with_left("F#4:4 G#4 A4:8 | _:5 -:1 A#1+A#2:2 B4:5 F#5:1 B#4:2 | \
            C#5:4 B4:7/3 -:2/3 A#4:1 A4:4 F#3+A3+C#4:2 C#2+C#3 | \
            D4:4 F#3+A3+D4:2 D2+D3+B3 C#2+C#3+C#4+E#4+C#5 B2+B3 A2+A3 G#2+G#3 | \
            F#4:4 G#4 A4:5 -:1 B2+B3:2 | \
            A#2+A#3+A#4:3 A3+C#4+E4:1 A3+C#4+E4:2 A#1+A#2 B1+B2 D4+F#4 D3+F#3+B3:4 | \
            C#2+C#3+E#5:3 E#3+G#3+B3+C#5:1 E#3+G#3+B3:2 E#2+E#3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 C#2+C#3 | \
            C#5+F#5:3 F#4:1 C#2+C#3:2 E#3+G#3+B3 A2+C#3+F#3+F#4 A2:1 C#3 F#2+F#3:2 - & F#2+F#3:2 F#3+A3+C#4 E#2+E#3 E#3+G#3+B3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 B2+B3 | \
            A4:8 B1+B2:2 D3:1 F#3 D3+F#3+B3:4 | \
            C#2+C#3+E#5:3 E#3+G#3+B3+C#5:1 E#3+G#3+B3:2 E#2+E#3+B4 F#2+F#3:3 F#3+A3+C#4:1 - A4 G#4 F#4 | \
            A4:3 A3+F#4:1 F#4 E#4 F#4 G#4 -:8 | \
            F#2+F#3:2 F#3+A3+C#4 E#2+E#3 E#3+G#3+B3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 - | \
            -:10 D3:1 F#3 - F#5 F#5 G#5 | -:4 C#5:1 B4 B4 A#4 B4 A#4 A4:2 -:1 A4 G#4 F#4 | \
            F#2+F#3:2 F#3+A3+C#4 E#4:1 G#4 F#4 E#4 -:8 & -:16 | \
            A#2+A#3:2 F#3+A#3+C#4 F#3+A#3+C#4 -:8 F#5:1 G#5 | \
            -:4 C#5:1 B4:7/3 -:2/3 B4:1 A#4 A4:2 -:4 | D2+F#2+A2+D3:2 F#3:1 -:13 | -:10 A4:6 | \
            _:10 -:6 | -:16 | -"),
        // Bars 10-17 of the file.
        Part::new(
            "A again",
            "A4:3 A4:1/3 B4 C#5 B4:1 A#4 B4 B#4 C#5:2 C#5 -:1 C#5 D5 D#5 | \
            E5:3 D5:1/2 E5 D5:1 C#5 C#5 D5:1/3 C#5 D5 C#5:2 B4:3 F#6:1 F#6 G#6 | \
            E#6:3 C#6:1 C#6 B5 B5 A#5 B5 A#5 A5:2 F#4:1 A5 G#5 F#5 | \
            D5:3 F#5:1 F#5 E#5 F#5 G#5 E#5:2 -:3 F#4:1 F#4 G#4 | \
            A4:3 A4:1/3 B4 C#5 B4:1 A#4 B4 B#4 C#5:2 C#5 -:1 C#5 D5 D#5 | \
            E5:3 D5:1/2 E5 D5:1 C#5 C#5 D5:1/3 C#5 D5 C#5:2 B4 -:1 F#6 F#6 G#6 | \
            E#6:3 C#6:1 C#6 B5 B5 A#5 B5 A#5 A5:2 -:1 A5 G#5 F#5 | \
            A5:3 F#5:1 E#5 G#5 F#5 E#5 F#5:2 - F#5:1 F#4:1/2 G#4 A4:1 G#4",
            "F#m C#7/E# F#m F#m | F#7/A# F#7/A# Bm G#m7b5/D | C#7 C#7/E# F#m F#m/C# | \
            D D C#7 C#7/G# | F#m C#7/E# F#m F#m | F#7/A# F#7/A# Bm G#m7b5/D | \
            C#7 C#7/E# F#m F#m/C# | F#m C#7 F#m/A F#m",
        )
        .with_left("F#4:4 G#4 A4:8 | _:5 -:1 A#1+A#2:2 B4:5 F#5:1 B#4:2 | \
            C#5:4 B4:7/3 -:2/3 A#4:1 A4:4 F#3+A3+C#4:2 C#2+C#3 | \
            D4:4 F#3+A3+D4:2 D2+D3+B3 C#2+C#3+C#4+E#4+C#5 B2+B3 A2+A3 G#2+G#3 | \
            F#4:4 G#4 A4:5 -:1 B2+B3:2 | \
            A#2+A#3+A#4:3 A3+C#4+E4:1 A3+C#4+E4:2 A#1+A#2 B1+B2 D4+F#4 D3+F#3+B3:4 | \
            C#2+C#3+E#5:3 E#3+G#3+B3+C#5:1 E#3+G#3+B3:2 E#2+E#3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 C#2+C#3 | \
            C#5+F#5:3 F#4:1 C#2+C#3:2 E#3+G#3+B3 A2+C#3+F#3+F#4 C#3 F#2+F#3 - & F#2+F#3:2 F#3+A3+C#4 E#2+E#3 E#3+G#3+B3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 B2+B3 | \
            A4:8 B1+B2:2 D3:1 F#3 D3+F#3+B3:4 | \
            C#2+C#3+E#5:3 E#3+G#3+B3+C#5:1 E#3+G#3+B3:2 E#2+E#3+B4 F#2+F#3:3 F#3+A3+C#4:1 - A4 G#4 F#4 | \
            A4:3 A3+F#4:1 F#4 E#4 F#4 G#4 -:8 | \
            F#2+F#3:2 F#3+A3+C#4 E#2+E#3 E#3+G#3+B3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 - | \
            -:10 D3:1 F#3 - F#5 F#5 G#5 | -:4 C#5:1 B4 B4 A#4 B4 A#4 A4:2 -:1 A4 G#4 F#4 | \
            F#2+F#3:2 F#3+A3+C#4 E#4:1 G#4 F#4 E#4 -:4 F#4+A4+C#5:1 -:3 & -:16 | \
            A#2+A#3:2 F#3+A#3+C#4 F#3+A#3+C#4 -:8 F#5:1 G#5 | \
            -:4 C#5:1 B4:7/3 -:2/3 B4:1 A#4 A4:2 -:4 | D2+F#2+A2+D3:2 F#3:1 -:13 | -:10 A4:6 | \
            _:10 -:6 | -:16 | -"),
        // Bars 18-25 of the file.
        Part::new(
            "B",
            "F#4:5 E#4:1/2 F#4 G#4:1 F#4 E#4:2 C#4:1 D4 C#4 E#4:1/2 F#4 G#4:1 F#4 | \
            E#4:2 C#4:1 D4 C#4:2 B3:1 A#3 B3 A#3 A3:3 F#4:1/2 G#4 A4:1 G#4 | \
            F#4:5 E#4:1/2 F#4 G#4:1 F#4 E#4:2 C#4:1 D4 C#4 E#4:1/2 F#4 G#4:1 F#4 | \
            E#4:2 C#4:1 D4 B3:2 E#4:1 C#4 F#4 - C#4:2 F#5:1 F#4:1/2 G#4 A4:1 G#4 | \
            F#4:5 E#4:1/2 F#4 G#4:1 F#4 E#4:2 C#4:1 D4 C#4 E#4:1/2 F#4 G#4:1 F#4 | \
            E#4:2 C#4:1 D4 C#4:2 B3:1 A#3 B3 A#3 A3:3 F#4:1/2 G#4 A4:1 G#4 | \
            F#4:5 E#4:1/2 F#4 G#4:1 F#4 E#4:2 C#4:1 D4 C#4 E#4:1/2 F#4 G#4:1 F#4 | \
            E#4:2 C#4:1 D4 B3:2 E#4:1 C#4 F#4 - C#4:2 F#5 G#5",
            "F#m/A G#7b5/B# C#m7 G#7/B# | C#7 C#7/E# F#m/A F#m/A | F#m/A G#7b5/B# C#7 G#7/B# | \
            C#7 C#7/E# F#m F#m | F#m/A G#7b5/B# C#m7 G#7/B# | C#7 C#7/E# F#m/A F#m/A | \
            F#m/A G#7b5/B# C#7 G#7/B# | C#7 C#7/E# F#m F#m",
        )
        .with_left("A2+C#3+F#3:2 C#3+C#4:1 A2+D4 A2+C#3+F#3:2 B#1+B#2+D4 C#4:4 E3+G#3+B3:2 B#1+B#2+B#3 | \
            B3:4 E#3+G#3+B3:2 E#2+E#3+A#3 A3:4 A2+C#3+F#3:2 - | \
            A2+C#3+F#3 C#3+C#4:1 A2+D4 A2+C#3+F#3:2 B#1+B#2+D4 C#4:4 E#3+G#3+B3:2 B#1+B#2+B#3 | \
            B3:4 E#3+G#3:2 E#2 F#2+F#3 A2:1 C#3 F#2+F#3:2 - | \
            A2+C#3+F#3 C#3+C#4:1 A2+D4 A2+C#3+F#3:2 B#1+B#2+D4 C#4:4 E3+G#3+B3:2 B#1+B#2+B#3 | \
            B3:4 E#3+G#3+B3:2 E#2+E#3+A#3 A3:4 A2+C#3+F#3:2 - | \
            A2+C#3+F#3 C#3+C#4:1 A2+D4 A2+C#3+F#3:2 B#1+B#2+D4 C#4:4 E#3+G#3+B3:2 B#1+B#2+B#3 | \
            B3:4 E#3+G#3:2 E#2 F#2+F#3 A2:1 C#3 F#2+F#3+F#4+A4+C#5:2 G#3+B3+D4+D5+E5 & -:4 C#4:1 -:3 C#2+C#3:2 E3+G#3+B3:1 E3+G#3+B3 -:4 | \
            C#2+C#3:2 B3:7/3 -:11/3 A2+C#3+F#3:2 C#3:1 A2 -:4 | \
            - B3:1 -:3 C#2+C#3:2 E#3+G#3+B3:1 E#3+G#3+B3 -:4 | \
            C#2+C#3:2 B3:7/3 -:5/3 G#3:1 -:5 F#4+A4+C#5:1 -:3 | \
            -:4 C#4:1 -:3 C#2+C#3:2 E3+G#3+B3:1 E3+G#3+B3 -:4 | \
            C#2+C#3:2 B3:7/3 -:11/3 A2+C#3+F#3:2 C#3:1 A2 -:4 | \
            - B3:1 -:3 C#2+C#3:2 E#3+G#3+B3:1 E#3+G#3+B3 -:4 | \
            C#2+C#3:2 B3:7/3 -:5/3 G#3:1 -:9 & -:16 | -:2 E#3+G#3:1 B3:7/3 -:32/3 | -:16 | \
            -:2 E#3+G#3:1 B3:7/3 -:32/3 | -:16 | -:2 E#3+G#3:1 B3:7/3 -:32/3 | -:16 | \
            -:2 E#3+G#3:1 B3:7/3 -:32/3 & -:16 | -:3 E#3+G#3:1 -:12 | -:16 | -:3 E#3+G#3:1 -:12 | \
            -:16 | -:3 E#3+G#3:1 -:12 | -:16 | -:3 E#3+G#3:1 -:12"),
        // Bars 26-31 of the file.
        Part::new(
            "C",
            "A5:2 E5:1 F#5 G#5 A5 B5 C#6 D6 D5:1/3 E5 E#5 F#5:1 D5 B4 D5:2 B4:1 | \
            G#4:2 B5:1 A5 G#5 F#5 E5 D5 C#5 C#5:1/2 D5 C#5:2 B#4 C#5 | \
            D5 B4:1 G#4 G#5:2 E5:1 D5:1/3 C#5 D5 C#5:2 A4:3 E5:1 D5 C#5 | \
            B4:2 G#4:1 E#4 D5:2 C#5:1 B4:1/3 A4 B4 A4:2 F#4 F#4:1 F#4 G#4 A4 | \
            C#5 C#5:2 C#5:1 C#5:2 B#4:1 C#5 F#5 F#5:2 F#5 G#5:1 A5 B5 | \
            C#6:2 A5:1 C#6 B5:2 G#5:1 E#5 F#5:2 - F#6 G#5",
            "A E7/G# | E7 A | E7 E7/G# A A | C#7 F#m | A C#7/E# F#m F#m | F#m/A C#7/E# F#m E7/D",
        )
        .with_left("E3+A3+C#4+A4+C#5+E5:2 E3+E4:1 D#3+D#4+F#4 D3+D4+G#4 C#3+C#4+A4 B2+B3+B4 A2+A3+C#5 G#2+G#3:2 -:6 | \
            E4:2 G#5:1 F#5 E5 D5 C#5 B4 A2+C#3+E3+A3:2 -:6 | \
            E2+E3:3 G#3+B3+D4:1 G#3+B3+D4+G#4:2 G#1+G#2 A1+A2:3 C#3+E3+A3:1 C#3+E3+A3:2 C#3+E3+A3 | \
            C#2+C#3:3 E#3+G#3+B3:1 E#3+G#3+B3+E#4:2 E2+E#3 F#2+F#3:3 C#3+F#3+A3:1 C#3+F#3+A3:4 | \
            A1+A2:3 E3+A3+C#4+E4+A4:1 E3+A3+C#4+E4+A4:2 E#2+E#3 F#2+F#3:3 A4+C#5:2 G#4:1 A4 B4 | \
            A2+A3+C#5:2 F#3+A3+C#4 E#2+E#3+B4 E#3+G#3+B3 F#2+F#3+F#4 A2:1 C#3 F#2+F#3+A5+C#6:2 G#3+B3+D4+D5+E5 & -:8 D5:1 -:7 | \
            -:8 A4:1 -:7 | E4+G#4:2 -:8 C#4+E4:4 -:2 | E#4+G#4 -:8 C#4:2 C#4:1 -:3 | \
            E4+A4:1 E4+A4:2 -:5 A4+C#5:1 A4+C#5:2 F#3+A3+C#4:1 F#3+A3+C#4:4 | \
            -:2 A4:1 C#5 -:2 G#4:1 E#4 -:8"),
        // Bars 32-37 of the file.
        Part::new(
            "C again",
            "A5:2 E5:1 F#5 G#5 A5 B5 C#6 D6 D5:1/3 E5 E#5 F#5:1 D5 B4 D5:2 B4:1 | \
            G#4:2 B5:1 A5 G#5 F#5 E5 D5 C#5 C#5:1/2 D5 C#5:2 B#4 C#5 | \
            D5 B4:1 G#4 G#5:2 E5:1 D5:1/3 C#5 D5 C#5:2 A4:3 E5:1 D5 C#5 | \
            B4:2 G#4:1 E#4 D5:2 C#5:1 B4:1/3 A4 B4 A4:2 F#4 F#4:1 F#4 G#4 A4 | \
            C#5 C#5:2 C#5:1 C#5:2 B#4:1 C#5 F#5 F#5:2 F#5 G#5:1 A5 B5 | \
            C#6:2 A5:1 C#6 B5:2 G#5:1 E#5 F#5:2 - E6:1 F#4 F#4 G#4",
            "A E7/G# | E7 A | E7 E7/G# A A | C#7 F#m | A C#7/E# F#m F#m | F#m/A C#7/E# F#m F#m",
        )
        .with_left("E3+A3+C#4+A4+C#5+E5:2 E3+E4:1 D#3+D#4+F#4 D3+D4+G#4 C#3+C#4+A4 B2+B3+B4 A2+A3+C#5 G#2+G#3:2 -:6 | \
            E4:2 G#5:1 F#5 E5 D5 C#5 B4 A2+C#3+E3+A3:2 -:6 | \
            E2+E3:3 G#3+B3+D4:1 G#3+B3+D4+G#4:2 G#1+G#2 A1+A2:3 C#3+E3+A3:1 C#3+E3+A3:2 C#3+E3+A3 | \
            C#2+C#3:3 E#3+G#3+B3:1 E#3+G#3+B3+E#4:2 E2+E#3 F#2+F#3:3 C#3+F#3+A3:1 C#3+F#3+A3:4 | \
            A1+A2:3 E3+A3+C#4+E4+A4:1 E3+A3+C#4+E4+A4:2 E#2+E#3 F#2+F#3:3 A4+C#5:2 G#4:1 A4 B4 | \
            A2+A3+C#5:2 F#3+A3+C#4 E#2+E#3+B4 E#3+G#3+B3 F#2+F#3+F#4 A2:1 C#3 F#2+F#3:2 - & -:8 D5:1 -:7 | \
            -:8 A4:1 -:7 | E4+G#4:2 -:8 C#4+E4:4 -:2 | E#4+G#4 -:8 C#4:2 C#4:1 -:3 | \
            E4+A4:1 E4+A4:2 -:5 A4+C#5:1 A4+C#5:2 F#3+A3+C#4:1 F#3+A3+C#4:4 | \
            -:2 A4:1 C#5 -:2 G#4:1 E#4 -:4 A5+C#6:1 -:3"),
        // Bars 38-45 of the file.
        Part::new(
            "Last A",
            "A4:3 A4:1/3 B4 C#5 B4:1 A#4 B4 B#4 C#5:2 C#5 -:1 C#5 D5 D#5 | \
            E5:3 D5:1/2 E5 D5:1 C#5 C#5 D5:1/3 C#5 D5 C#5:2 B4:3 F#6:1 F#6 G#6 | \
            E#6:3 C#6:1 C#6 B5 B5 A#5 B5 A#5 A5:2 F#4:1 A5 G#5 F#5 | \
            D5:3 F#5:1 F#5 E#5 F#5 G#5 E#5:2 -:3 F#4:1 F#4 G#4 | \
            A4:3 A4:1/3 B4 C#5 B4:1 A#4 B4 B#4 C#5:2 C#5 -:1 C#5 D5 D#5 | \
            E5:3 D5:1/2 E5 D5:1 C#5 C#5 D5:1/3 C#5 D5 C#5:2 B4 -:1 F#6 F#6 G#6 | \
            E#6:3 C#6:1 C#6 B5 B5 A#5 B5 A#5 A5:2 -:1 A5 G#5 F#5 | \
            A5:3 F#5:1 E#5 G#5 F#5 E#5 F#5:2 - F#6:4",
            "F#m C#7/E# F#m F#m | F#7/A# F#7/A# Bm G#m7b5/D | C#7 C#7/E# F#m F#m/C# | \
            D D C#7 C#7/G# | F#m C#7/E# F#m F#m | F#7/A# F#7/A# Bm G#m7b5/D | \
            C#7 C#7/E# F#m F#m/C# | F#m C#7 F#m/A F#m",
        )
        .with_left("F#4:4 G#4 A4:8 | _:5 -:1 A#1+A#2:2 B4:5 F#5:1 B#4:2 | \
            C#5:4 B4:7/3 -:2/3 A#4:1 A4:4 F#3+A3+C#4:2 C#2+C#3 | \
            D4:4 F#3+A3+D4:2 D2+D3+B3 C#2+C#3+C#4+E#4+C#5 B2+B3 A2+A3 G#2+G#3 | \
            F#4:4 G#4 A4:5 -:1 B2+B3:2 | \
            A#2+A#3+A#4:3 A3+C#4+E4:1 A3+C#4+E4:2 A#1+A#2 B1+B2 D4+F#4 D3+F#3+B3:4 | \
            C#2+C#3+E#5:3 E#3+G#3+B3+C#5:1 E#3+G#3+B3:2 E#2+E#3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 C#2+C#3 | \
            C#5+F#5:3 F#4:1 C#2+C#3:2 E#3+G#3+B3 A2+C#3+F#3+F#4 A2:1 C#3 F#2+F#3+F#5+A5+C#6:4 & F#2+F#3:2 F#3+A3+C#4 E#2+E#3 E#3+G#3+B3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 B2+B3 | \
            A4:8 B1+B2:2 D3:1 F#3 D3+F#3+B3:4 | \
            C#2+C#3+E#5:3 E#3+G#3+B3+C#5:1 E#3+G#3+B3:2 E#2+E#3+B4 F#2+F#3:3 F#3+A3+C#4:1 - A4 G#4 F#4 | \
            A4:3 A3+F#4:1 F#4 E#4 F#4 G#4 -:8 | \
            F#2+F#3:2 F#3+A3+C#4 E#2+E#3 E#3+G#3+B3 F#2+F#3:3 F#3+A3+C#4:1 F#3+A3+C#4:2 - | \
            -:10 D3:1 F#3 - F#5 F#5 G#5 | -:4 C#5:1 B4 B4 A#4 B4 A#4 A4:2 -:1 A4 G#4 F#4 | \
            F#2+F#3:2 F#3+A3+C#4 E#4:1 G#4 F#4 E#4 -:8 & -:16 | \
            A#2+A#3:2 F#3+A#3+C#4 F#3+A#3+C#4 -:8 F#5:1 G#5 | \
            -:4 C#5:1 B4:7/3 -:2/3 B4:1 A#4 A4:2 -:4 | D2+F#2+A2+D3:2 F#3:1 -:13 | -:10 A4:6 | \
            _:10 -:6 | -:16 | -"),
    ],
    form: &["Pickup", "A", "A again", "B", "C", "C again", "Last A"],
    drums_from: 0,
    about: "A tango from 1920, named after the grey fox fur coat of a lady of the night. The \
            first strain runs up in quick triplets and leaps to a high E sharp, the leading \
            note, over C sharp seven; the second sinks to the low register; the third runs \
            down a scale, and the first comes back to end it.",
};
