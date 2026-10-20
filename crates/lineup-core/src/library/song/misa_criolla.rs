//! The Misa Criolla: Ariel Ramírez's Mass for soloists, choir and the instruments
//! of Argentine folklore, each movement set to a dance or a song of the country.

use super::*;

/// The band of the recording: the soloists' voices, Ramírez's harpsichord and a
/// double bass.
const MISA: Band = Band::new(gm::VOICE_OOHS, gm::HARPSICHORD, gm::ACOUSTIC_BASS);

// Copyright: the Misa Criolla (Ariel Ramírez, 1964; Ramírez died in 2010) is still
// under copyright; it is added here at the request of the project's owner.
// Source: a sequence of the five movements for the two soloists, a third soloist,
// the four voices of the choir (treble, countertenor, tenor, bass), bombo, guitars,
// harpsichord and bass, given by the project's owner, converted by script: quantised
// to sixteenths, triplets kept, the tempo and the fermatas from its tempo map, the
// dynamics from its velocities. Where the soloists sing, the first soloist is the
// melody, the second is "Soloist 2" and the third "Soloist 3"; where they do not, the
// choir's top voice is the melody and "Choir" has the other three. A tremolo, a
// chord struck again and again faster than a sixteenth, is held. The words are left
// out: the voices sing without them.

// Sources: see above. The sequence's bombo plays the first and third beats of every
// bar; it is the only instrument.
pub(super) const MISA_CRIOLLA_KYRIE: Song = Song {
    name: "Misa Criolla: Kyrie",
    origin: "Ariel Ramírez, Misa Criolla, 1964",
    feel: "Vidala-baguala",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 69.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "Vidala",
    band: SONG_AND_GUITAR,
    style: "Vidala",
    parts: &[
        // Bars 1-2.
        Part::new(
            "Intro",
            "-:12 | -",
            "",
        )
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3"),
        ]),
        // Bars 3-8.
        Part::new(
            "Choir",
            "!p A4:8 G#4:4 | A4:8 G4:4 | F4:12 | F4 | E4:8 A4:4 | E4:12",
            "",
        )
        .with_lines(&[
            line(
                "Choir",
                gm::VOICE_OOHS,
                "!p E4:8 E4:4 | C4:8 E4:4 | D4:12 | C4 | C4:8 E4:4 | D4:12 & !p C4:8 E3:4 | \
                E4:8 A3:4 | A3:12 | A3:8 C4:4 | A3:8 C4:4 | A3:12 & !p A2:8 B2:4 | C3:8 C#3:4 | \
                D3:12 | D#3 | E3 | E3",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3"),
        ]),
        // Bars 9-20.
        Part::new(
            "Kyrie",
            "-:8 !mf E4:4 | E4:6 D4:2 C4:4 | B3:6 A3:2 G3:4 | E3 E3 - | -:8 A3:4 | A3:6 A3:2 A3:4 | \
            G4:6 A4:2 G4:4 | F4 F4 - | -:8 F4:4 | F4:6 A3:2 A3:4 | A3:6 A3:2 G#3:4 | A3 A3 -",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:8 !mf C4:4 | C4:6 B3:2 A3:4 | G3:6 F3:2 B2:4 | C3 C3 - | -:8 A3:4 | \
                A3:6 A3:2 A3:4 | E4:6 F4:2 E4:4 | D4 D4 - | -:8 F4:4 | F4:6 A3:2 A3:4 | \
                A3:6 A3:2 G#3:4 | A3 A3 -",
            ),
            line(
                "Choir",
                gm::VOICE_OOHS,
                "!p E4:8 -:4 | !mp A4:6 G#4:2 A4:4 | G4:8 G4:4 | G4:8 -:4 | -:12 | A4 | A4 | \
                A4:8 -:4 | -:12 | F4:8 F4:4 | A4:8 G#4:4 | A4:8 G#4:4 & !p D4:8 -:4 | \
                !mp E4:8 E4:4 | D4:8 D4:4 | C4:8 -:4 | -:12 | E4 | G4 | F4:8 -:4 | -:12 | \
                C4:8 C4:4 | E4:8 E4:4 | E4:8 E4:4 & !p G#3:8 -:4 | !mp C4:6 B3:2 A3:4 | \
                B3:8 F3:4 | E3:8 -:4 | -:12 | G3 | E4 | D4:8 -:4 | -:12 | A3:8 A3:4 | C4:8 B3:4 | \
                C4:8 E3:4 & !p E3:8 -:4 | !mp A2:6 B2:2 C3:4 | D3:8 G2:4 | C3:8 -:4 | -:12 | C#3 | \
                C#4 | D4:8 -:4 | -:12 | D#3:8 D#3:4 | E3:8 E3:4 | A2:8 B2:4",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3"),
        ]),
        // Bars 21-26.
        Part::new(
            "Choir again",
            "!mp A4:8 G4:4 | F4:12 | F4:8 F4:4 | E4:6 E4:2 A4:4 | E4:12 | _",
            "",
        )
        .with_lines(&[
            line(
                "Choir",
                gm::VOICE_OOHS,
                "!mp C4:8 E4:4 | D4:12 | C4:8 C4:4 | C4:6 C4:2 E4:4 | D4:12 | _ & !mp E4:8 A3:4 | \
                A3:12 | A3:8 C4:4 | A3:6 A3:2 C4:4 | A3:12 | G#3 & !mp C3:8 C#3:4 | D3:12 | \
                D#3:8 D#3:4 | E3:6 E3:2 E3:4 | E3:12 | _",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3"),
        ]),
        // Bars 27-44.
        Part::new(
            "Christe",
            "-:12 | - | !mp E4:1 A3:11 | -:4 A3:2 A3 A3 E4 | _:3 E3:1 E3:2 A3:6 | _ - | -:12 | - | \
            - | A3:1 E4:11 | -:4 E4:2 E3 E3 E4 | _:3 E3:1 E3:2 A3:6 | _ - | -:12 | \
            G#3:2 F#3 F#3 F#3 E4:4 | _:6 -:2 D4 C#4 | B3:12 | A3:11 -:1",
            "",
        )
        .in_key(A, Major)
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:12 | - | !mp E4:1 A3:11 | -:4 A3:2 A3 A3 E4 | _:3 E3:1 E3:2 A3:6 | _ - | -:12 | \
                - | - | A3:1 E4:11 | -:4 E4:2 E3 E3 E4 | _:3 E3:1 E3:2 A3:6 | _ - | -:12 | \
                E3:2 D3 D3 D3 C#4:4 | _:6 -:2 B3 A3 | G#3:12 | A3:11 -:1",
            ),
            line(
                "Choir",
                gm::VOICE_OOHS,
                "!p E4:12 | _ | _ | _ | _ | A4:6 A4:2 A4:4 | A4:6 - | - G#4 | A4:12 | _ | _ | _ | \
                A4:6 A4:2 A4:4 | A4:8 -:4 | A4:12 | _ | F#4 | E4:11 -:1 & !p C#4:12 | _ | _ | _ | \
                _ | C#4:6 C#4:2 D4:4 | C#4:6 - | - D4 | C#4:12 | _ | _ | _ | C#4:6 C#4:2 D4:4 | \
                C#4:8 -:4 | F#4:12 | _ | D4 | C#4:11 -:1 & !p E3:12 | _ | _ | _ | _ | \
                E3:6 E3:2 F#3:4 | E3:6 - | - F#3 | E3:12 | _ | _ | _ | E3:6 E3:2 F#3:4 | E3:12 | \
                A3 | B3 | G#3 | E3:11 -:1 & !p A2:12 | _ | _ | _ | _ | A2:6 A2:2 A2:4 | A2:6 - | \
                - E3 | A2:12 | _ | _ | _ | A2:6 A2:2 A2:4 | A2:12 | D3 | E3 | E3 | \
                A2:2 -:6 A2:3 -:1",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3"),
        ]),
        // Bars 45-50.
        Part::new(
            "Choir, third time",
            "!p A4:8 G#4:4 | A4:8 G4:4 | F4:12 | F4 | E4:8 A4:4 | E4:12",
            "",
        )
        .with_lines(&[
            line(
                "Choir",
                gm::VOICE_OOHS,
                "!p E4:8 E4:4 | C4:8 E4:4 | D4:12 | C4 | C4:8 E4:4 | D4:12 & !p C4:8 E3:4 | \
                E4:8 A3:4 | A3:12 | A3:8 C4:4 | A3:8 C4:4 | A3:12 & !p A2:8 B2:4 | C3:8 C#3:4 | \
                D3:12 | D#3 | E3 | E3",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3"),
        ]),
        // Bars 51-62.
        Part::new(
            "Kyrie again",
            "-:8 !mf E4:4 | E4:6 D4:2 C4:4 | B3:6 A3:2 G3:4 | E3 E3 - | -:8 A3:4 | A3:6 A3:2 A3:4 | \
            G4:6 A4:2 G4:4 | F4 F4 - | -:8 F4:4 | F4:6 A3:2 A3:4 | A3:6 A3:2 G#3:4 | A3 A3 -",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:8 !mf C4:4 | C4:6 B3:2 A3:4 | G3:6 F3:2 B2:4 | C3 C3 - | -:8 A3:4 | \
                A3:6 A3:2 A3:4 | E4:6 F4:2 E4:4 | D4 D4 - | -:8 F4:4 | F4:6 A3:2 A3:4 | \
                A3:6 A3:2 G#3:4 | A3 A3 -",
            ),
            line(
                "Choir",
                gm::VOICE_OOHS,
                "!p E4:8 -:4 | !mp A4:6 G#4:2 A4:4 | G4:8 G4:4 | G4:8 -:4 | -:12 | A4 | A4 | \
                A4:8 -:4 | -:12 | F4:8 F4:4 | A4:8 G#4:4 | A4:8 G#4:4 & !p D4:8 -:4 | \
                !mp E4:8 E4:4 | D4:8 D4:4 | C4:8 -:4 | -:12 | E4 | G4 | F4:8 -:4 | -:12 | \
                C4:8 C4:4 | E4:8 E4:4 | E4:8 E4:4 & !p G#3:8 -:4 | !mp C4:6 B3:2 A3:4 | \
                B3:8 F3:4 | E3:8 -:4 | -:12 | G3 | E4 | D4:8 -:4 | -:12 | A3:8 A3:4 | C4:8 B3:4 | \
                C4:8 E3:4 & !p E3:8 -:4 | !mp A2:6 B2:2 C3:4 | D3:8 G2:4 | C3:8 -:4 | -:12 | C#3 | \
                C#4 | D4:8 -:4 | -:12 | D#3:8 D#3:4 | E3:8 E3:4 | A2:8 B2:4",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3"),
        ]),
        // Bars 63-69.
        Part::new(
            "Choir, last time",
            "!mp A4:8 G4:4 | F4:12 | F4:8 F4:4 | E4:6 E4:2 A4:4 | E4:12 | _ | E4",
            "",
        )
        .with_lines(&[
            line(
                "Choir",
                gm::VOICE_OOHS,
                "!mp C4:8 E4:4 | D4:12 | C4:8 C4:4 | C4:6 C4:2 E4:4 | D4:12 | _ | \
                C#4 & !mp E4:8 A3:4 | A3:12 | A3:8 C4:4 | A3:6 A3:2 C4:4 | A3:12 | G#3 | \
                E3 & !mp C3:8 C#3:4 | D3:12 | D#3:8 D#3:4 | E3:6 E3:2 E3:4 | E3:12 | _ | A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | \
             x:1 -:7 x:1 -:3 | x:1 -:7 x:1 -:3 | x:1 -:11"),
        ])
        .with_tempo(&[fermata(72, 12)]),
    ],
    form: &[
        "Intro",
        "Choir",
        "Kyrie",
        "Choir again",
        "Christe",
        "Choir, third time",
        "Kyrie again",
        "Choir, last time",
    ],
    drums_from: 0,
    about: "The first movement of the Misa Criolla, a Mass sung in Spanish, each of its \
            five movements set to a rhythm of Argentine and Andean folklore. The Kyrie is a \
            vidala-baguala, the slow song of the Andean north-west, in 3/4 and A minor, \
            over the bombo legüero on the first and third beats of every bar. The choir \
            sings alone first, softly, while its bass climbs from the tonic, A, to the \
            dominant, E, by semitones from C (A, B, C, C sharp, D, D sharp, E). Then the \
            two soloists sing, mostly a third apart, the choir under them. The middle part, the Christe, turns to A major, \
            the tonic major, before the Kyrie comes back in A minor and ends, under a \
            fermata, on A major, the tonic major again: a Picardy third.",
};

// Sources: see above. The sequence's guitars, harpsichord and bass play the
// introduction and the dance between the verses; under the voices only the bombo
// and the bass play.
pub(super) const MISA_CRIOLLA_GLORIA: Song = Song {
    name: "Misa Criolla: Gloria",
    origin: "Ariel Ramírez, Misa Criolla, 1964",
    feel: "Carnavalito and yaraví",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: TWO_FOUR,
    tempo: 40.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "Carnavalito (2/4)",
    band: MISA,
    style: "Carnavalito (2/4)",
    parts: &[
        // Bars 1-6.
        Part::new(
            "Intro",
            "-:12 | - | - | - | - | -",
            "",
        )
        .in_time(THREE_FOUR)
        .with_left("-:12 | - | - | - | - | !mf F#3+A3:4 A2+E3:6 E3:2")
        .with_lines(&[
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "-:4 !mf C5+F5+C6 A4+C5+A5 | _ C5+F5+C6:2 A4+C5+A5 C5+F5+C6 A4+C5+A5 | \
                G4+C5+G5:3 E4+G4+E5:7 -:2 | -:4 G4+C5+G5 E4+G4+E5 | \
                _ G4+C5+G5:2 E4+G4+E5 G4+C5+G5 E4+G4+E5 | \
                D4+A4+D5:4 A3+C4+A4:6 B3+E4+G#4:2 & -:4 !mf F4+A4 C4+F4 | \
                _ F4+A4:2 C4+F4 F4+A4 C4+F4 | B3+D4:4 G3+C4:8 | -:4 C4+E4 G3+C4 | _ C4+E4 G3+C4 | \
                -:12",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[fermata(66, 4), tempo(70, 88)]),
        // Bars 7-14.
        Part::new(
            "Carnavalito",
            "-:8 | - | - | - | - | - | - | -:4 !f A4:2 A4",
            "",
        )
        .with_left("!mf A2:4 E3 | A2 E3 | A2 E3 | A2 E3:2 E2 | A2:4 E3:2 E2 | A2:4 E3:2 E2 | \
            A2:4 E3:2 E2 | A2:4 -")
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:8 | - | - | - | - | - | - | -:4 !f E4:2 E4 & -:8 | - | - | - | - | - | - | \
                -:4 !f C4:2 C4 & -:8 | - | - | - | - | - | - | -:4 !f C4:2 C4",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!mf C4+E4+A4:2 C4:1 E4+A4:2 B3:1 E4+G#4:2 | C4+E4+A4 C4:1 E4+A4:2 B3:1 E4+G#4:2 | \
                C4+E4+A4 C4:1 E4+A4:2 B3:1 E4+G#4:2 | C4+E4+A4 C4:1 E4+A4:2 B3:1 E4+G#4:2 | \
                A4+C5 C4:1 A4+C5:2 E4:1 B4+D5:2 | A4+C5 E4:1 A4+C5:2 E4:1 G#4+B4:2 | \
                C4+E4+A4 C4:1 E4+A4:2 B3:1 E4+G#4:2 | C4+E4+A4:4 -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "-:8 | - | !f A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | \
                A2:4 -:2 E2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 | - | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "-:8 | - | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x"),
        ]),
        // Bars 15-36.
        Part::new(
            "Gloria",
            "-:4 !f E4:1 E4:2 D4:1 | C4:2 A3 -:4 | - (3 C4:2 A3 G3 | A3:2 A3 -:4 | \
            - E4:1 E4:2 D4:1 | C4:2 A3 (3 D4:2 B3 G3 | A3:4 - | - E4:1 E4:2 D4:1 | C4:2 A3 -:4 | \
            - (3 C4:2 A3 G3 | A3:2 A3 -:4 | - E4:1 E4:2 D4:1 | C4:2 A3 (3 D4:2 B3 G3 | \
            A3:4 A3:2 A3 | D4 D4 -:4 | - C4:2 C4 | D4 D4 -:4 | - A4:2 A4 | A4 A4 -:4 | - A4:2 A4 | \
            A4 A4 -:4 | -:8",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:4 !f C4:1 C4:2 B3:1 | A3:2 E3 -:4 | - (3 E3:2 E3 E3 | E3:2 E3 -:4 | \
                - E3:1 E3:2 D3:1 | C3:2 A2 (3 D3:2 B2 G2 | A2:4 - | - C4:1 C4:2 B3:1 | \
                A3:2 E3 -:4 | - (3 E3:2 E3 E3 | E3:2 E3 -:4 | - E3:1 E3:2 D3:1 | \
                C3:2 A2 (3 D3:2 B2 G2 | A2:4 A3:2 A3 | D4 D4 -:4 | - C4:2 C4 | D4 D4 -:4 | \
                - C4:2 C4 | D4 D4 -:4 | - C4:2 C4 | D4 D4 -:4 | -:8",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f A4:4 - | - A4:2 A4 | C5:3 A4:1 -:4 | - E5:1 E5:2 D5:1 | C5:2 A4 E5:4 | _ F4 | \
                A4 A4:2 A4 | A4:4 - | - A4:2 A4 | C5:3 A4:1 -:4 | - E5:1 E5:2 D5:1 | \
                C5:2 A4 E5:4 | _ F4 | A4 - | - G4:1 G4:2 G4:1 | A4:2 E4 -:4 | - G4:1 G4:2 G4:1 | \
                A4:2 E4 -:4 | - (3 G5:2 G5 G5 | A5:2 E5 -:4 | - (3 G5:2 G5 G5 | \
                A5:2 E5 (3 E4:2 E5 D5 & !f E4:4 - | - E4:2 E4 | A4:3 E4:1 -:4 | - F4 | _ F#4 | \
                _ B3 | E4 E4:2 E4 | E4:4 - | - E4:2 E4 | A4:3 E4:1 -:4 | - F4 | _ F#4 | _ B3 | \
                E4 - | - F4:1 F4:2 F4:1 | E4:2 C4 -:4 | - F4:1 F4:2 F4:1 | E4:2 C4 -:4 | \
                - (3 G4:2 G4 G4 | A4:2 A4 -:4 | - (3 G4:2 B4 D5 | \
                C5:2 A4 (3 E4:2 C5 B4 & !f C4:4 - | - C4:2 C4 | E4:3 C4:1 -:4 | - C4 | _ C4 | \
                _ D3 | E3 C4:2 C4 | C4:4 - | - C4:2 C4 | E4:3 C4:1 -:4 | - C4 | _ C4 | _ D3 | \
                E3 - | - D4:1 D4:2 D4:1 | E4:2 A3 -:4 | - D4:1 D4:2 D4:1 | E4:2 A3 -:4 | \
                - (3 F4:2 F4 F4 | E4:2 C4 -:4 | - (3 D4:2 F4 G4 | \
                G4:2 G4 (3 E3:2 E4 D4 & !f A3:4 - | - A3:2 A3 | C4:3 A3:1 -:4 | - D3 | _ D3 | \
                _ G2 | A2 C4:2 C4 | A3:4 - | - A3:2 A3 | C4:3 A3:1 -:4 | - D3 | _ D3 | _ G2 | \
                A2 - | - B3:1 B3:2 B3:1 | C4:2 G3 -:4 | - B3:1 B3:2 B3:1 | C4:2 G3 -:4 | \
                - (3 B3:2 B3 B3 | C4:2 G3 -:4 | - (3 B3:2 D4 F4 | E4:2 C4 (3 E3:2 C4 B3",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | \
                A2:4 -:2 E2 | A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | \
                A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 F3 | F3 G3:2 G3 | C3:4 F3 | \
                F3 G3:2 G3 | C3:4 F3 | F3 G3:2 G3 | C3:4 F3 | F3 G3:2 G3 | C3:4 -",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "-:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x"),
        ]),
        // Bars 37-62.
        Part::new(
            "Gloria, bar 37",
            "-:4 (3 !f C4:2 A3 G3 | A3:2 A3 -:4 | - A4:1 G4 E4 G4 | A4:2 A4 -:4 | \
            - E4:1 E4:2 D4:1 | C4:2 A3 -:4 | - (3 C4:2 A3 G3 | A3:2 A3 -:4 | - E4:1 E4:2 D4:1 | \
            C4:2 A3 (3 D4:2 B3 G3 | A3:4 - | - E4:1 E4:2 D4:1 | C4:2 A3 -:4 | - (3 C4:2 A3 G3 | \
            A3:2 A3 -:4 | - E4:1 E4:2 D4:1 | C4:2 A3 (3 D4:2 B3 G3 | A3:4 A3:2 A3 | D4 D4 -:4 | \
            - C4:2 C4 | D4 D4 -:4 | - A4:2 A4 | A4 A4 -:4 | - A4:2 A4 | A4 A4 -:4 | -:8",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:4 (3 !f C4:2 A3 G3 | A3:2 A3 -:4 | - E4:1 D4 B3 D4 | E4:2 E4 -:4 | \
                - C4:1 C4:2 B3:1 | A3:2 E3 -:4 | - (3 E3:2 E3 E3 | E3:2 E3 -:4 | \
                - E3:1 E3:2 D3:1 | C3:2 A2 (3 D3:2 B2 G2 | A2:4 - | - C4:1 C4:2 B3:1 | \
                A3:2 E3 -:4 | - (3 E3:2 E3 E3 | E3:2 E3 -:4 | - E3:1 E3:2 D3:1 | \
                C3:2 A2 (3 D3:2 B2 G2 | A2:4 A3:2 A3 | D4 D4 -:4 | - C4:2 C4 | D4 D4 -:4 | \
                - C4:2 C4 | D4 D4 -:4 | - C4:2 C4 | D4 D4 -:4 | -:8",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f C5:2 C5 -:4 | - (3 C5:2 A4 G4 | A4:2 A4 B4:4 | A4 A4:2 A4 | A4:4 - | \
                - A4:2 A4 | C5:3 A4:1 -:4 | - E5:1 E5:2 D5:1 | C5:2 A4 E5:4 | _ F4 | A4 A4:2 A4 | \
                A4:4 - | - A4:2 A4 | C5:3 A4:1 -:4 | - E5:1 E5:2 D5:1 | C5:2 A4 E5:4 | _ F4 | \
                A4 - | - G4:1 G4:2 G4:1 | A4:2 E4 -:4 | - G4:1 G4:2 G4:1 | A4:2 E4 -:4 | \
                - (3 G5:2 G5 G5 | A5:2 E5 -:4 | - (3 G5:2 G5 G5 | \
                A5:2 E5 (3 E4:2 E5 D5 & !f A4:2 A4 -:4 | - (3 A4:2 F4 E4 | F4:2 F4 D4:4 | \
                C4 E4:2 E4 | E4:4 - | - E4:2 E4 | A4:3 E4:1 -:4 | - F4 | _ F#4 | _ B3 | \
                E4 E4:2 E4 | E4:4 - | - E4:2 E4 | A4:3 E4:1 -:4 | - F4 | _ F#4 | _ B3 | E4 - | \
                - F4:1 F4:2 F4:1 | E4:2 C4 -:4 | - F4:1 F4:2 F4:1 | E4:2 C4 -:4 | \
                - (3 G4:2 G4 G4 | A4:2 A4 -:4 | - (3 G4:2 B4 D5 | \
                C5:2 A4 (3 E4:2 C5 B4 & !f C4:2 C4 -:4 | - (3 C4:2 A3 G3 | A3:2 A3 G3:4 | \
                E3 C4:2 C4 | C4:4 - | - C4:2 C4 | E4:3 C4:1 -:4 | - C4 | _ C4 | _ D3 | \
                E3 C4:2 C4 | C4:4 - | - C4:2 C4 | E4:3 C4:1 -:4 | - C4 | _ C4 | _ D3 | E3 - | \
                - D4:1 D4:2 D4:1 | E4:2 A3 -:4 | - D4:1 D4:2 D4:1 | E4:2 A3 -:4 | \
                - (3 F4:2 F4 F4 | E4:2 C4 -:4 | - (3 D4:2 F4 G4 | \
                G4:2 G4 (3 E3:2 E4 D4 & !f A3:2 A3 -:4 | - (3 A3:2 F3 E3 | D3:2 D3 G2:4 | \
                A2 C4:2 C4 | A3:4 - | - A3:2 A3 | C4:3 A3:1 -:4 | - D3 | _ D3 | _ G2 | \
                A2 C4:2 C4 | A3:4 - | - A3:2 A3 | C4:3 A3:1 -:4 | - D3 | _ D3 | _ G2 | A2 - | \
                - B3:1 B3:2 B3:1 | C4:2 G3 -:4 | - B3:1 B3:2 B3:1 | C4:2 G3 -:4 | \
                - (3 B3:2 B3 B3 | C4:2 G3 -:4 | - (3 B3:2 D4 F4 | E4:2 C4 (3 E3:2 C4 B3",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A3:4 - | D3 - | D3 G2 | A2 - | A2 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | \
                A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E2 | A2:4 -:2 E3 | \
                A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 F3 | \
                F3 G3:2 G3 | C3:4 F3 | F3 G3:2 G3 | C3:4 F3 | F3 G3:2 G3 | C3:4 F3 | F3 G3:2 G3 | \
                C3:4 -",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "-:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x"),
        ]),
        // Bars 63-70.
        Part::new(
            "Gloria, bar 63",
            "-:4 (3 !f C4:2 A3 G3 | A3:2 A3 -:4 | - A4:1 G4 E4 G4 | A4:2 A4 -:4 | -:8 | - | - | -",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:4 (3 !f C4:2 A3 G3 | A3:2 A3 -:4 | - E4:1 D4 B3 D4 | E4:2 E4 -:4 | -:8 | - | - | \
                -",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f C5:2 C5 -:4 | - (3 C5:2 A4 G4 | A4:2 A4 B4:4 | A4 -:2 G4 | \
                (3 A4:2 - A4 -:2 G4 | A4 A4 A4:3 -:1 | \
                !mp E5+A5:2 D5+G5:1 B4+E5 E5+A5 D5+G5 !p C5+E5 B4+D5 | \
                A4+C5:2 A4+C5:6 & !f A4:2 A4 -:4 | - (3 A4:2 F4 E4 | F4:2 F4 D4:4 | C4 -:2 D4 | \
                (3 E4:2 - E4 -:2 D4 | E4 E4 E4:3 -:1 | !mp A4:8 | _ & !f C4:2 C4 -:4 | \
                - (3 C4:2 A3 G3 | A3:2 A3 G3:4 | E3 -:2 B3 | (3 C4:2 - C4 -:2 B3 | \
                C4 C4 D4:3 -:1 | -:8 | - & !f A3:2 A3 -:4 | - (3 A3:2 F3 E3 | D3:2 D3 G2:4 | \
                A2 -:2 G3 | (3 A3:2 - A3 -:2 G3 | A3 A3 A3:3 -:1 | -:8 | -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A3:4 - | D3 - | D3 G2 | A2 -:2 G3 | (3 A3:2 - A3 -:2 G3 | A3 A3 A3:1 -:3 | \
                -:8 | -",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | -:8 | - | x:1 - x - x -:3 | \
             -:8 | -"),
            (Drum::HighTom, "-:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:6 x:1 - | \
             x - (3 -:1 x - -:2 x:1 - | -:8 | - | -"),
        ])
        .with_tempo(&[tempo(28, 70), tempo(40, 64), tempo(41, 52), glide(42, 20, 2), tempo(48, 25)]),
        // Bars 71-72.
        Part::new(
            "Bridge",
            "!p A4+C5:4 - | -:8",
            "",
        )
        .with_left("!pp A2+E3:8 | _")
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!mp A4:4 - | -:8",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!pp C4+E4+A4:8 | _ & !pp C4+E4+A4:8 | _",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[tempo(0, 44)]),
        // Bars 73-91.
        Part::new(
            "Yaraví",
            "-:12 | - | - | - | - | - | -:10 !mp A3:2 | E4:6 D4:1 C4 (3 A3:2 A3 A3 | \
            _:2 D4:1 C4 A3:2 A3:6 | _:4 -:8 | -:12 | - | -:10 A3:1 A3 | \
            E4:4 -:2 D4 (3 C4:2 A3 G3 | A3:7 -:1 (3 A3:2 D4 C4 | A3:2 A3:6 -:2 C4:1 D4 | \
            E4 D4 E4 D4 (3 E4:2 D4 E4 C4:2 A3 | _:5 - C4:1 D4 | (3 E4:2 D4 E4 C4:2 A3:6",
            "",
        )
        .in_time(THREE_FOUR)
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | - | - | -:10 !mp A3:2 | E4:6 D4:1 C4 (3 A3:2 A3 A3 | \
                _:2 D4:1 C4 A3:2 A3:6 | _:4 -:8 | -:12 | - | -:10 A3:1 A3 | \
                E4:4 -:2 D4 (3 C4:2 A3 G3 | A3:7 -:1 (3 A3:2 D4 C4 | A3:2 A3:6 -:2 A3:1 B3 | \
                C4 B3 C4 B3 (3 C4:2 B3 C4 A3:2 E3 | _:5 - A3:1 B3 | (3 C4:2 B3 C4 A3:2 E3:6",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:12 | - | - | (3 !p C5:2 _ A4 _:3 !pp C5:1 A4:2 C5:1 A4 | G4:4 E4:8 | \
                (3 !p G4:2 _ E4 _:3 !pp G4:1 E4:2 G4:1 E4 | D4:4 A3:8 | _:12 | _ | \
                (3 !p C5:2 _ A4 _:3 !pp C5:1 A4:2 C5:1 A4 | G4:4 E4:8 | \
                (3 !p G4:2 _ E4 _:3 !pp G4:1 E4:2 G4:1 E4 | D4:4 A3:8 | _:12 | _ | _ | - | - | \
                - & -:12 | - | - | (3 !p A4:2 _ E4 _:3 !pp A4:1 E4:2 A4:1 E4 | D4:4 C4:8 | \
                (3 !p G4:2 _ E4 _:3 !pp G4:1 E4:2 G4:1 E4 | D4:4 A3:8 | _:12 | _ | \
                (3 !p A4:2 _ E4 _:3 !pp A4:1 E4:2 A4:1 E4 | D4:4 C4:8 | \
                (3 !p G4:2 _ E4 _:3 !pp G4:1 E4:2 G4:1 E4 | D4:4 A3:8 | _:12 | _ | _ | - | - | \
                - & -:12 | - | - | (3 !p E4:2 _ C4 _:3 !pp E4:1 C4:2 F4:1 C4 | B3:4 G3:8 | \
                (3 !p B3+D4:2 _ G3+C4 _:3 !pp B3+D4:1 G3+C4:2 B3+D4:1 G3+C4 | A3:4 E3:8 | _:12 | \
                _ | (3 !p E4:2 _ C4 _:3 !pp E4:1 C4:2 F4:1 C4 | B3:4 G3:8 | \
                (3 !p B3+D4:2 _ G3+C4 _:3 !pp B3+D4:1 G3+C4:2 B3+D4:1 G3+C4 | F#3+A3:4 E3:8 | \
                _:12 | _ | _ | E3 | _ | _ & -:12 | - | - | !p A2:8 !pp A2:4 | A2:8 -:2 !p A2 | \
                _:8 !pp A2:4 | A2:8 A2:4 | A2:8 A2:4 | A2:12 | !p A2:8 !pp A2:4 | A2:8 -:2 !p A2 | \
                _:8 !pp A2:4 | A2:8 A2:4 | A2:8 A2:4 | A2:8 A2:4 | A2:12 | A2 | _ | _",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "-:12 | - | - | !p A2 | _ | _ | _ | _ | _ | A2 | _ | _ | _ | _ | _ | _ | A2 | _ | \
                _:8 -:4",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x -"),
        ])
        .with_tempo(&[tempo(0, 40)]),
        // Bars 92-104.
        Part::new(
            "Yaraví, bar 92",
            "!mp A3:8 -:4 | -:12 | - | - | -:10 C4:1 D4 | E4 E4 E4 D4 (3 E4:2 D4 E4 C4:2 A3 | \
            _:5 - E4:2 | A4:1 G4 E4 D4 (3 E4:2 D4 E4 _:4 | _:8 -:2 C4:1 D4 | \
            E4 E4 D4 E4 (3 C4:1 _ A3 _:1 -:3 D4:1 D4 | E4:2 (3 -:1 A3 G3 A3:2 G3 A3:4 | _:12 | \
            _:8 -:4",
            "",
        )
        .in_time(THREE_FOUR)
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!mp E3:8 -:4 | -:12 | - | - | -:10 A3:1 B3 | C4 C4 C4 B3 (3 C4:2 B3 C4 A3:2 E3 | \
                _:5 - C4:2 | E4:1 D4 C4 B3 (3 C4:2 B3 C4 _:4 | _:8 -:2 A3:1 B3 | \
                C4 C4 B3 C4 (3 A3:1 _ E3 _:1 -:3 B3:1 B3 | C4:2 (3 -:1 E3 D3 E3:2 D3 E3:4 | _:12 | \
                _:8 -:4",
            ),
            line(
                "Soloist 3",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | -:10 !mp A3:1 B3 | C4 C4 C4 B3 (3 C4:2 B3 C4 A3:2 E3 | \
                _:5 - C4:2 | E4:1 D4 C4 B3 (3 C4:2 B3 C4 _:4 | _:8 -:2 A3:1 B3 | \
                C4 C4 B3 C4 (3 A3:1 _ E3 _:1 -:3 B3:1 B3 | C4:2 (3 -:1 A2 G2 A2:2 G2 A2:4 | _:12 | \
                _:8 -:4",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "(3 !p C5:2 _ A4 _:3 !pp C5:1 A4:2 C5:1 A4 | G4:4 E4:8 | \
                (3 !p G4:2 _ E4 _:3 !pp G4:1 E4:2 G4:1 E4 | D4:4 A3:8 | -:12 | - | - | - | - | - | \
                - | (3 -:2 !mp E5+A5 D5+G5 E5+A5:2 D5+G5 E5+A5:4 | \
                _:8 -:4 & (3 !p A4:2 _ E4 _:3 !pp A4:1 E4:2 A4:1 E4 | D4:4 C4:8 | \
                (3 !p G4:2 _ E4 _:3 !pp G4:1 E4:2 G4:1 E4 | D4:4 A3:8 | -:12 | - | - | - | - | - | \
                - | (3 -:2 !mp A4 G4 A4:2 G4 A4:4 | \
                _:8 -:4 & (3 !p E4:2 _ C4 _:3 !pp E4:1 C4:2 F4:1 C4 | B3:4 G3:8 | \
                (3 !p B3+D4:2 _ G3+C4 _:3 !pp B3+D4:1 G3+C4:2 B3+D4:1 G3+C4 | A3:4 E3:8 | E3:12 | \
                _ | _ | _ | _ | _ | _ | _ | _:8 -:4 & !p A2:8 !pp A2:4 | A2:8 -:2 A2 | _:8 A2:4 | \
                A2:12 | A2 | _ | _ | _ | _ | _ | _ | _ | _:8 -:4",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!p A2:12 | _ | _ | _ | A2 | _ | _ | _ | _ | _ | _ | _ | _:8 -:4",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x - x - x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x - x - x - | x x x x x - x - x - x - | x - x - x - x - x - x - | \
             x - x - x - x -:5"),
        ])
        .with_tempo(&[fermata(120, 4), fermata(152, 4)]),
        // Bars 105-126.
        Part::new(
            "Carnavalito again",
            "-:8 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | \
            -:4 !f A4:2 A4",
            "",
        )
        .with_left("-:8 | - | - | - | !mf G2:4 D3:2 B2 | C3:8 | E2:4 B2:2 G#2 | A2:8 | G2:4 D3:2 B2 | \
            C3:8 | E2:4 B2:2 G#2 | E2+F#2:1 G#2 A2:6 | G4+G5:4 D5+D6:2 B4+B5 | C5+C6:8 | \
            E4+E5:4 B4+B5:2 G#4+G#5 | A4+A5:8 | G4+G5:4 D5+D6:2 B4+B5 | C5+C6:8 | \
            E4+E5:4 B4+B5:2 G#4+G#5 | A4+A5:4 -:2 G#5 | A5:6 G#5:2 | A5:4 - & -:8 | - | - | - | \
            - | - | - | - | - | - | - | - | - | - | - | !mf G#5:1 -:7 | -:8 | B5:1 -:7 | -:8 | \
            G#5:1 -:7 | -:8 | -")
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:8 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | \
                - | -:4 !f E4:2 E4 & -:8 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | \
                - | - | - | - | - | - | -:4 !f C4:2 C4 & -:8 | - | - | - | - | - | - | - | - | - | \
                - | - | - | - | - | - | - | - | - | - | - | -:4 !f C4:2 C4",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "-:8 | - | - | - | \
                -:2 !mp F4+G4+B4+D5:1 F4+G4+B4+D5 -:2 F4+G4+B4+D5:1 F4+G4+B4+D5 | \
                -:2 E4+G4+C5:1 E4+G4+C5 -:2 E4+G4+C5:1 E4+G4+C5 | \
                -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 | \
                -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 | \
                -:2 F4+G4+B4+D5:1 F4+G4+B4+D5 -:2 F4+G4+B4+D5:1 F4+G4+B4+D5 | \
                -:2 E4+G4+C5:1 E4+G4+C5 -:2 E4+G4+C5:1 E4+G4+C5 | \
                -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 | \
                -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 | \
                -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 | \
                -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 | \
                -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 | \
                -:2 E4+A4+C5+E5:1 E4+A4+C5+E5 -:2 E4+A4+C5+E5:1 E4+A4+C5+E5 | \
                -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 | \
                -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 | \
                -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 | \
                E4+A4+C5+E5:4 -:2 B3+E4+G#4 | C4+E4+A4:6 B3+E4+G#4:2 | C4+E4+A4:4 - & -:8 | - | \
                - | - | -:2 !mp F4+G4+B4+D5:1 F4+G4+B4+D5 -:2 F4+G4+B4+D5:1 F4+G4+B4+D5 | \
                -:2 E4+G4+C5:1 E4+G4+C5 -:2 E4+G4+C5:1 E4+G4+C5 | \
                -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 | \
                -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 | \
                -:2 F4+G4+B4+D5:1 F4+G4+B4+D5 -:2 F4+G4+B4+D5:1 F4+G4+B4+D5 | \
                -:2 E4+G4+C5:1 E4+G4+C5 -:2 E4+G4+C5:1 E4+G4+C5 | \
                -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 -:2 G#3+D4+E4+B4:1 G#3+D4+E4+B4 | \
                -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 -:2 A3+C4+E4+A4:1 A3+C4+E4+A4 | \
                -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 | \
                -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 | \
                -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 | \
                -:2 E4+A4+C5+E5:1 E4+A4+C5+E5 -:2 E4+A4+C5+E5:1 E4+A4+C5+E5 | \
                -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 -:2 F4+G4+B4+D5+G5:1 F4+G4+B4+D5+G5 | \
                -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 -:2 G4+D5+E5+G5:1 G4+D5+E5+G5 | \
                -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 -:2 D4+E4+G#4+B4+E5:1 D4+E4+G#4+B4+E5 | \
                E4+A4+C5+E5:4 -:2 B3+E4+G#4 | C4+E4+A4:6 B3+E4+G#4:2 | C4+E4+A4:4 -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "-:8 | - | - | - | !mf G2:4 D3 | A2 E3:2 G3 | E2:4 B2 | A2 C3:2 E3 | G2:4 D3 | \
                C3 E3:2 G3 | E2:4 B2 | A2 C3:2 E3 | G2:4 D3 | C3 E3:2 G3 | E2:4 B2 | A2 C3:2 E3 | \
                G2:4 D3 | C3 E3:2 G3 | E2:4 B2 | A2 - | A2:6 -:2 | A2:4 -",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "-:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x"),
        ])
        .with_tempo(&[tempo(0, 88)]),
        // Bars 127-148.
        Part::new(
            "Gloria again",
            "-:4 !f E4:1 E4:2 D4:1 | C4:2 A3 -:4 | - (3 C4:2 A3 G3 | A3:2 A3 -:4 | \
            - E4:1 E4:2 D4:1 | C4:2 A3 (3 D4:2 B3 G3 | A3:4 - | - !mp E4:1 E4:2 D4:1 | \
            C4:2 A3 -:4 | - (3 C4:2 A3 G3 | A3:2 A3 -:4 | - E4:1 E4:2 D4:1 | \
            C4:2 A3 (3 D4:2 B3 G3 | A3:4 !f A4:2 A4 | A4:4 - | - A4:2 A4 | A4:4 - | - A4:2 A4 | \
            A4:4 - | - A4:2 A4 | A4:4 - | - !mp C4:2 A3:1 G3",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:4 !f C4:1 C4:2 B3:1 | A3:2 E3 -:4 | - (3 E3:2 E3 E3 | E3:2 E3 -:4 | \
                - E3:1 E3:2 D3:1 | C3:2 A2 (3 D3:2 B2 G2 | A2:4 - | - !mp C4:1 C4:2 B3:1 | \
                A3:2 E3 -:4 | - (3 E3:2 E3 E3 | E3:2 E3 -:4 | - E3:1 E3:2 D3:1 | \
                C3:2 A2 (3 D3:2 B2 G2 | A2:4 !f C4:2 C4 | D4:4 - | - C4:2 C4 | D4:4 - | \
                - D4:2 D4 | E4:4 - | - D4:2 D4 | E4:4 - | - !mp C4:2 A3:1 G3",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f A4:4 - | - A4:2 A4 | C5:3 A4:1 -:4 | - E5:1 E5:2 D5:1 | C5:2 A4 E5:4 | _ F4 | \
                A4 !p A4:2 A4 | A4:4 - | - A4:2 A4 | C5:3 A4:1 -:4 | - E5:1 E5:2 D5:1 | \
                C5:2 A4 E5:4 | _ F4 | A4 - | - !mf G4:1 G4:2 G4:1 | A4:2 E4 -:4 | \
                - G4:1 G4:2 G4:1 | A4:2 E4 -:4 | - (3 !f G5:2 G5 G5 | A5:2 E5 -:4 | \
                - (3 G5:2 G5 G5 | A5:2 E5 -:4 & !f E4:4 - | - E4:2 E4 | A4:3 E4:1 -:4 | - F4 | \
                _ F#4 | _ B3 | E4 !p E4:2 E4 | E4:4 - | - E4:2 E4 | A4:3 E4:1 -:4 | - F4 | _ F#4 | \
                _ B3 | E4 - | - !mf F4:1 F4:2 F4:1 | E4:2 C4 -:4 | - F4:1 F4:2 F4:1 | \
                E4:2 C4 -:4 | - (3 !f G4:2 G4 G4 | A4:2 A4 -:4 | - (3 G4:2 B4 D5 | \
                C5:2 A4 -:4 & !f C4:4 - | - C4:2 C4 | E4:3 C4:1 -:4 | - C4 | _ C4 | _ D3 | \
                E3 !p C4:2 C4 | C4:4 - | - C4:2 C4 | E4:3 C4:1 -:4 | - C4 | _ C4 | _ D3 | E3 - | \
                - !mf D4:1 D4:2 D4:1 | E4:2 A3 -:4 | - D4:1 D4:2 D4:1 | E4:2 A3 -:4 | \
                - (3 !f F4:2 F4 F4 | E4:2 C4 -:4 | - (3 D4:2 F4 G4 | G4:2 G4 -:4 & !f A3:4 - | \
                - A3:2 A3 | C4:3 A3:1 -:4 | - D3 | _ D3 | _ G2 | A2 !p C4:2 C4 | A3:4 - | \
                - A3:2 A3 | C4:3 A3:1 -:4 | - D3 | _ D3 | _ G2 | A2 - | - !mf B3:1 B3:2 B3:1 | \
                C4:2 G3 -:4 | - B3:1 B3:2 B3:1 | C4:2 G3 -:4 | - (3 !f B3:2 B3 B3 | C4:2 C4 -:4 | \
                - (3 B3:2 D4 F4 | E4:2 C4 -:4",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mf A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | \
                A2:4 -:2 E2 | A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 -:2 E3 | \
                A2:4 -:2 E2 | A2:4 -:2 E3 | A2:4 -:2 E2 | A2:4 - | F3 G3:2 G3 | C3:4 F3 | \
                F3 G3:2 G3 | C3:4 F3 | F3 G3:2 G3 | C3:4 F3 | F3 G3:2 G3 | C3:4 -",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "-:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x"),
        ]),
        // Bars 149-167.
        Part::new(
            "Ending",
            "!mp A3:2 A3:1 E4:2 D4 C4:1 | A3:2 A3 -:4 | -:8 | -:4 (3 !mf E4:2 D4 C4 | \
            (3 A3:2 A3 G3 A3:2 A3 | _:4 - | -:8 | - | - | - | -:4 !f A4:2 G4 | A4:3 A4:1 -:4 | \
            -:8 | - | - | - | - | - | -",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!mp A3:2 A3:1 E4:2 D4 C4:1 | A3:2 A3 -:4 | -:8 | -:4 (3 !mf E4:2 D4 C4 | \
                (3 A3:2 A3 G3 A3:2 A3 | _:4 - | -:8 | - | - | - | -:4 !f E4:2 D4 | E4:3 E4:1 -:4 | \
                -:8 | - | - | - | - | - | -",
            ),
            line(
                "Soloist 3",
                gm::VOICE_OOHS,
                "-:8 | - | - | - | - | - | - | - | - | - | -:4 !f A3:2 G3 | A3:3 A3:1 -:4 | -:8 | \
                - | - | - | - | - | -",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:8 | -:4 !mp A4+C5:2 F4+A4:1 E4+G4 | F4+A4:2 F4+A4:1 C5+E5:2 B4+D5 A4+C5:1 | \
                F4+A4:2 F4+A4 -:4 | -:8 | -:4 (3 C5+E5:2 B4+D5 A4+C5 | \
                (3 F4+A4:2 F4+A4 E4+G4 F4+A4:2 F4+A4 | _:4 - | - !f A4:2 A4 | E5:4 E5 | \
                _ E5+A5:2 D5+G5 | A5:3 A5:1 -:2 !mp G4 | A4:6 G4:2 | !p A4:4 -:2 A4 | A4:8 | \
                _:4 G4 | A4:8 | _ | _:2 -:6 & -:8 | -:4 !mp F4:2 D4:1 C4 | \
                D4:2 D4:1 A4:2 G4 F4:1 | D4:2 D4 -:4 | -:8 | -:4 (3 A4:2 G4 F4 | \
                (3 D4:2 D4 C4 D4:2 D4 | _:4 - | - !f A4:2 A4 | A4:4 A4 | _ A4:2 G4 | \
                A4+E5:3 A4+E5:1 -:2 !mp G4 | E4:6 G4:2 | !p E4:4 -:2 A4 | E4:8 | _:4 F4 | E4:8 | \
                _ | _:2 -:6 & -:8 | - | - | - | - | -:4 (3 !f C4:2 C4 C4 | (3 C4:2 C4 C4 C4:2 C4 | \
                _:4 - | - A3:2 A3 | F#4:4 F#4 | _ D4:2 D4 | E4:3 E4:1 -:2 !mp B3 | C4:6 B3:2 | \
                !p C4:4 -:2 A3 | F#3:8 | _:4 D4 | C4:8 | _ | _:2 -:6 & -:8 | - | - | - | - | \
                -:4 (3 !f D3:2 D3 D3 | (3 D3:2 D3 D3 D3:2 D3 | _:4 - | - A3:2 A3 | E4:4 E4 | \
                _ G3:2 G3 | A3:3 A3:1 -:2 !mp D4 | A3:6 D4:2 | !p A3:4 -:2 A3 | D3:8 | _:4 G3 | \
                A3:8 | _ | _:2 -:6",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mf D3:4 - | D4 - | D3 - | D4 - | D3 - | -:8 | - | - | - | - | - | -:6 G3:2 | \
                A3 -:4 G3:2 | A3 -:6 | D3:8 | _:4 G3 | A3 E3 | A2 E3 | A2:2 -:6",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | \
             x:1 -:3 x:1 -:3 | x:1 -:3 x:1 -:3 | x:1 -:7"),
            (Drum::HighTom, "-:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | \
             -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:2 x:1 x -:2 x:1 x | -:8"),
        ]),
    ],
    form: &[
        "Intro",
        "Carnavalito",
        "Gloria",
        "Gloria, bar 37",
        "Gloria, bar 63",
        "Bridge",
        "Yaraví",
        "Yaraví, bar 92",
        "Carnavalito again",
        "Gloria again",
        "Ending",
    ],
    drums_from: 0,
    about: "The second movement of the Misa Criolla is a carnavalito, the quick carnival \
            dance of the Quebrada de Humahuaca, in 2/4, with a yaraví, the slow lament of \
            the Andes, in its middle. A free introduction in 3/4, the guitars trembling \
            on their chords, leads into the dance: the bombo strikes each beat and adds two \
            sixteenths, the guitars strum A minor and E major, its dominant, a beat each, \
            and the harpsichord and the bass play the roots. The soloists and the choir \
            sing over the bombo and the bass, and the harmony turns to the relative major, \
            C, through its IV and V, F and G. In the middle the tempo falls to the yaraví, \
            in 3/4, the voices over a held A in the bass and the bombo's soft eighth notes; \
            then the carnavalito comes back.",
};

// Sources: see above. The sequence's two bombo tracks are the low tom (its head) and
// the rim, its "Tomtom" the high tom. Its "Treble 2", a high line in the introduction
// (A5 to E6) played on a piccolo in the sequence, is a flute here; "Strings R1" is the
// guitars, "Strings L" the bass. The bombo's roll in the last bar is played in sixteenths,
// and the last chord, which the sequence holds at a quarter of 20, is held at 46.
pub(super) const MISA_CRIOLLA_CREDO: Song = Song {
    name: "Misa Criolla: Credo",
    origin: "Ariel Ramírez, Misa Criolla, 1964",
    feel: "Chacarera trunca",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 152.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "Chacarera trunca",
    band: MISA,
    style: "Chacarera trunca",
    parts: &[
        // Bars 1-1.
        Part::new(
            "Pickup",
            "-:4",
            "",
        )
        .in_time(ONE_FOUR)
        .with_left("!mf A1:1 -:3")
        .with_lines(&[
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "(3 !mp A3+C4:2 _ - & -:1 !mp E4 (3 _:1 - - & -:2 (3 !mp A4:1 - -",
            ),
        ])
        .played(Comping::Held, Bass::None, ""),
        // Bars 2-20.
        Part::new(
            "Intro",
            "-:12 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | -",
            "",
        )
        .with_left("!mf D2:12 | A1:4 - A1:1 -:3 | D2:12 | A1:4 - A1:1 -:3 | D2:12 | A1:4 - A1:1 -:3 | \
            D2:12 | A1:4 A1 D2 | _:12 | A1+A2:4 A1 D2 | _:12 | A1+A2:4 A1 D2 | _:12 | \
            A1+A2:4 A1 D2 | _:12 | A1+A2:4 A1 !mp D2 | _:12 | A1+A2:4 A1 D2 | _:12 & -:12 | - | \
            - | - | - | - | - | -:10 !mf D3:2 | _:12 | -:10 D3:2 | _:12 | -:10 D3:2 | _:12 | \
            -:10 D3:2 | _:12 | -:10 !mp D3:2 | _:12 | -:10 D3:2 | _:12")
        .with_lines(&[
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!mp A3+D4:12 | A3+C4+E4+A4:4 - A3+C4+E4+A4:1 -:3 | A3+D4:12 | \
                A3+C4+E4+A4:1 -:7 A3+C4+E4+A4:1 -:3 | A3+D4:12 | \
                A3+C4+E4+A4:1 -:7 A3+C4+E4+A4:1 -:3 | A3+D4:12 | A3+C4+E4+A4:4 - A3+C4+E4+A4 | \
                A3+D4:12 | A3+C4+E4+A4:4 - A3+C4+E4+A4 | A3+D4:12 | A3+C4+E4+A4:4 - A3+C4+E4+A4 | \
                A3+D4:12 | A3+C4+E4+A4:4 - A3+C4+E4+A4 | A3+D4:12 | A3+C4+E4+A4:4 - A3+C4+E4+A4 | \
                A3+D4:12 | !p A3+C4+E4+A4:4 - A3+C4+E4+A4 | A3+D4:12 & -:1 !mp F#4:11 | -:12 | \
                -:1 F#4:11 | -:12 | -:1 F#4:11 | -:12 | -:1 F#4:11 | -:12 | -:1 F#4:11 | -:12 | \
                -:1 F#4:11 | -:12 | -:1 F#4:11 | -:12 | -:1 F#4:11 | -:12 | -:1 F#4:11 | -:12 | \
                -:1 !p F#4:11 & -:2 !mp A4:10 | -:12 | -:2 A4:10 | -:12 | -:2 A4:10 | -:12 | \
                -:2 A4:10 | -:12 | -:2 A4:10 | -:12 | -:2 A4:10 | -:12 | -:2 A4:10 | -:12 | \
                -:2 A4:10 | -:12 | -:2 A4:10 | -:12 | -:2 !p A4:10",
            ),
            line(
                "Flute",
                gm::FLUTE,
                "-:12 | - | - | - | - | - | - | !mf E6:11 E6:1 | D6:4 -:2 A5 D6 -:1 A5 | \
                D6:4 -:2 A5 D6 -:1 A5 | D6:4 -:2 A5 D6 -:1 A5 | D6:4 -:2 A5 D6 -:1 A5 | \
                D6:4 -:2 A5 D6 -:1 A5 | D6:4 -:2 A5 D6 -:1 A5 | D6:4 -:2 A5 D6 -:1 A5 | \
                D6:4 -:2 !mp A5 D6 -:1 A5 | D6:4 -:2 A5 D6 -:1 A5 | D6:4 -:2 A5 D6 -:1 A5 | \
                D6:4 -:2 A5 !p D6 -:1 A5",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "-:12 | - | - | - | - | - | - | - | - | - | - | -:4 !f A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:12 | - | - | - | - | - | -:8 x:1 -:3 | x:1 -:3 x:1 -:7 | -:8 x:1 -:3 | \
             x:1 -:3 x:1 -:7 | -:8 x:1 -:3 | x:1 -:3 x:1 -:7 | -:8 x:1 -:3 | x:1 -:3 x:1 -:7 | \
             -:8 x:1 -:3 | x:1 -:3 x:1 -:7 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "-:12 | - | - | - | - | -:8 x:1 x x x | x -:9 x:1 - | -:8 x:1 x x x | x -:9 x:1 - | \
             -:8 x:1 x x x | x -:9 x:1 - | -:8 x:1 x x x | x -:9 x:1 - | -:8 x:1 x x x | \
             x -:9 x:1 - | -:8 x:1 x x x | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:12 | - | - | - | - | - | - | - | - | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 21-29.
        Part::new(
            "Credo",
            "-:8 !mf A4:2 G#4 | A4 B4:4 A4 G#4:2 | G#4 A4:4 -:2 G#4 A4 | B4 A4:4 G#4 A4:2 | \
            B4 A4:4 -:2 !p A4 G#4 | A4 B4:4 A4 G#4:2 | G#4 A4:4 -:2 G#4 A4 | B4 A4:4 G#4 A4:2 | \
            B4 A4:4 -:2 !f A4 G#4",
            "",
        )
        .with_left("!p A1+A2:4 -:8 | -:12 | - | - | - | - | - | - | -")
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:8 !mf E4:2 E4 | E4 E4:4 E4 E4:2 | E4 E4:4 -:2 E4 E4 | E4 E4:4 E4 E4:2 | \
                E4 E4:4 -:2 !p E4 E4 | E4 E4:4 E4 E4:2 | E4 E4:4 -:2 E4 E4 | E4 E4:4 E4 E4:2 | \
                E4 E4:4 -:2 !f E4 E4 & -:8 !mf C4:2 B3 | C4 D4:4 C4 B3:2 | B3 C4:4 -:2 B3 C4 | \
                D4 C4:4 B3 C4:2 | D4 C4:4 -:2 !p C4 B3 | C4 D4:4 C4 B3:2 | B3 C4:4 -:2 B3 C4 | \
                D4 C4:4 B3 C4:2 | D4 C4:4 -:2 !f C4 B3 & -:12 | -:6 !mf E3:4 E3:2 | E3 E3:4 -:6 | \
                - E3:4 E3:2 | E3 E3:4 -:6 | - !p E3:4 E3:2 | E3 E3:4 -:6 | - E3:4 E3:2 | \
                E3 E3:4 -:6",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!p A3+C4+E4+A4:4 -:8 | -:12 | - | - | - | - | - | - | -",
            ),
            line(
                "Flute",
                gm::FLUTE,
                "!p D6:4 -:2 A5 D6 -:1 A5 | D6:4 -:8 | -:12 | - | - | - | - | - | -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2 | _ A2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 30-37.
        Part::new(
            "Credo, bar 30",
            "!f E4:6 C4:4 B3:2 | B3 C4:6 -:4 | E4:6 C4:4 B3:2 | B3 C4:6 -:4 | !mp E4:6 C4:4 B3:2 | \
            B3 C4:6 -:4 | E4:6 C4:4 B3:2 | B3 C4:6 !f C4:2 D4",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!f C4:6 A3:4 G#3:2 | G#3 A3:6 -:4 | C4:6 A3:4 G#3:2 | G#3 A3:6 -:4 | \
                !mp C4:6 A3:4 G#3:2 | G#3 A3:6 -:4 | C4:6 A3:4 G#3:2 | G#3 A3:6 !f A3:2 B3",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f A4:2 B4:4 A4 G#4:2 | G#4 A4:4 -:2 G#4 A4 | B4 A4:4 G#4 A4:2 | \
                B4 A4:4 -:2 !mp A4 G#4 | A4 B4:4 A4 G#4:2 | G#4 A4:4 -:2 G#4 A4 | \
                B4 A4:4 G#4 A4:2 | B4 A4:6 -:4 & !f E4:2 E4:4 E4 E4:2 | E4 E4:4 -:2 E4 E4 | \
                E4 E4:4 E4 E4:2 | E4 E4:4 -:2 !mp E4 E4 | E4 E4:4 E4 E4:2 | E4 E4:4 -:2 E4 E4 | \
                E4 E4:4 E4 E4:2 | E4 E4:6 -:4 & !f C4:2 D4:4 C4 B3:2 | B3 C4:4 -:2 B3 C4 | \
                D4 C4:4 B3 C4:2 | D4 C4:4 -:2 !mp C4 B3 | C4 D4:4 C4 B3:2 | B3 C4:4 -:2 B3 C4 | \
                D4 C4:4 B3 C4:2 | D4 C4:6 -:4 & -:6 !f E3:4 E3:2 | E3 E3:4 -:6 | - E3:4 E3:2 | \
                E3 E3:4 -:6 | - !mp E3:4 E3:2 | E3 E3:4 -:6 | - E3:4 E3:2 | E3 E3:6 -:4",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 38-53.
        Part::new(
            "Soloists",
            "!f E4:2 E4:4 E4 F#4:2 | G4:3 F#4 F#4 E4 | E4 D4 D4 D4 | E4:2 C4:6 !mf C4:2 D4 | \
            E4 E4:4 E4 F#4:2 | G4:3 F#4 F#4 E4 | E4 D4 D4 D4 | E4:2 C4:6 -:2 !f E4 | \
            E4 D4 D4 C4 B3 B3 | _ C4:6 B3:2 C4 | B3 C4:4 D4 E4:2 | _ C4:6 -:2 !mf E4 | \
            E4 D4 D4 C4 B3 B3 | _ C4:6 B3:2 C4 | B3 C4:4 D4 E4:2 | _ C4:6 -:4",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!f C4:2 C4:4 C4 D4:2 | E4:3 D4 D4 C4 | C4 B3 B3 B3 | C4:2 A3:6 !mf A3:2 B3 | \
                C4 C4:4 C4 D4:2 | E4:3 D4 D4 C4 | C4 B3 B3 B3 | C4:2 A3:6 -:2 !f E4 | \
                E4 D4 D4 C4 B3 B3 | _ C4:6 B3:2 C4 | B3 C4:4 D4 E4:2 | _ C4:6 -:2 !mf C4 | \
                C4 B3 B3 A3 G3 G#3 | G#3 A3:6 G#3:2 A3 | G#3 A3:4 B3 C4:2 | _ A3:6 -:4",
            ),
            line(
                "Soloist 3",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | - | - | - | - | - | - | - | -:10 !mf E3:2 | E3 D3 D3 C3 B2 B2 | \
                _ C3:6 B2:2 C3 | B2 C3:4 D3 E3:2 | _ C3:6 -:4",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 54-64.
        Part::new(
            "Choir",
            "!f E4:6 F4:2 - F4 | E5:6 B4:2 E5 - | E5 C5:4 B4:2 - E5 | E5:6 - | -:12 | - | \
            E5:2 G5:4 E5:2 - D5 | D5:6 (3 C5:1 _ - !mf A4:4 | G#4:2 A4:4 B4 A4:2 | \
            G#4:6 (3 A4:1 _ - !p A4:4 | G#4:2 A4:4 B4 A4:2",
            "",
        )
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f C5:6 G4:2 - C5 | B4:8 A4:2 - | A4:6 G#4:2 - B4 | A4:6 - | -:12 | - | \
                C5:2 E5:4 C5:2 - B4 | B4:6 (3 A4:1 _ - !mf E4:4 | E4:2 E4:4 E4 E4:2 | \
                E4:6 (3 E4:1 _ - !p E4:4 | E4:2 E4:4 E4 E4:2 & !f E4:6 E4:2 - E4 | E4:8 E4:2 - | \
                E4:6 E4:2 - E4 | E4:6 - | E4 E4:2 - E4 | E4:6 E4:2 -:4 | E4:2 G4:4 E4:2 - E4 | \
                E4:6 (3 E4:1 _ - !mf C4:4 | B3:2 C4:4 D4 C4:2 | B3:6 (3 C4:1 _ - !p C4:4 | \
                B3:2 C4:4 D4 C4:2 & !f C4:6 B3:2 - C4 | D4:8 C4:2 - | C4:6 B3:2 - D4 | C4:6 - | \
                C4 C4:2 - B3+D4 | B3+D4:6 A3+C4:2 -:4 | C4:2 E4:4 C4:2 - B3+D4 | \
                B3+D4:6 (3 A3+C4:1 _ - !mf E3:4 | E3:2 E3:4 E3 E3:2 | E3:6 (3 E3:1 _ - !p E3:4 | \
                E3:2 E3:4 E3 E3:2",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 65-77.
        Part::new(
            "Soloists and choir",
            "-:8 !f C4:2 D4 | E4:3 E4 E4 F#4 | G4 F#4 F#4 E4 | E4 D4 D4 D4 | E4:2 C4:4 -:2 C4 D4 | \
            E4:3 E4 E4 F#4 | G4 F#4 F#4 E4 | E4:4 D4 -:2 D4 | E4 C4:6 -:2 !mf E4 | \
            E4:6 C4:4 B3:2 | B3 C4:6 -:2 E4 | E4:6 C4:4 B3:2 | B3 C4:6 -:4",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:8 !f A3:2 B3 | C4:3 C4 C4 D4 | E4 D4 D4 C4 | C4 B3 B3 B3 | C4:2 A3:4 -:2 A3 B3 | \
                C4:3 C4 C4 D4 | E4 D4 D4 C4 | C4:4 B3 -:2 B3 | C4 A3:6 -:2 !mf C4 | \
                C4:6 A3:4 G#3:2 | G#3 A3:6 -:2 C4 | C4:6 A3:4 G#3:2 | G#3 A3:6 -:4",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p G#4:6 A4:2 -:4 | !mp E4:6 E4 | E4 E4 | E4 E4 | E4 E4 | E4 E4 | E4 E4 | E4 E4 | \
                -:10 !mf E5:2 | _:12 | _ | _ | _:8 !f C5:2 D5 & !p E4:6 E4:2 -:4 | !mp E4:6 E4 | \
                E4 E4 | E4 E4 | E4 E4 | E4 E4 | E4 E4 | E4 E4 | E4 - | -:12 | - | - | \
                -:8 !f A4:2 B4 & !p B3:6 C4:2 -:4 | !mp E3:12 | E3 | E3 | E3 | E3 | E3 | E3 | \
                -:10 !mf E4:2 | _:12 | _ | _ | _:8 !f C4:2 D4 & !p E3:6 E3:2 -:4 | !mp E3:12 | \
                E3 | E3 | E3 | E3 | E3 | E3 | E3:8 -:4 | -:12 | - | - | -:8 !f A3:2 B3",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 78-93.
        Part::new(
            "Choir, bar 78",
            "!f E5:3 E5 E5 F#5 | G5 F#5 F#5 E5 | E5:4 D5 -:2 D5:1 D5 | E5:2 C5:4 -:2 !p A4 G#4 | \
            A4 B4:4 A4 G#4:2 | G#4 A4:4 -:2 G#4 A4 | B4 A4:4 G#4 A4:2 | B4 A4:4 -:2 !f C#5 B4 | \
            C#5 D5:4 C#5 B4:2 | B4 C#5:6 B4:2 C#5 | D5 C#5:4 B4 C#5:2 | D5 C#5:6 E4:2 E4 | \
            E5 E5 E5 E5 F#5 G#5 | G#5 A5 E5:4 E4:2 E4 | E5 E5:4 F#5 E5:2 | E5:8 -:4",
            "",
        )
        .in_key(A, Major)
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f C5:3 C5 C5 D5 | E5 D5 D5 C5 | C5:4 B4 -:2 B4:1 B4 | C5:2 A4:4 -:2 !p E4 E4 | \
                E4 E4:4 E4 E4:2 | E4 E4:4 -:2 E4 E4 | E4 E4:4 E4 E4:2 | E4 E4:4 -:2 !f A4 G#4 | \
                A4 B4:4 A4 G#4:2 | G#4 A4:6 G#4:2 A4 | B4 A4:4 G#4 A4:2 | \
                B4 A4:6 A4+C#5:2 G#4+B4 | A4+C#5 B4+D5:4 A4+C#5 G#4+B4:2 | \
                G#4+B4 A4+C#5:6 A4+C#5:2 B4+D5 | B4+D5 A4+C#5:4 G#4+B4 A4+C#5:2 | \
                B4+D5 A4+C#5:6 -:4 & !f E4:3 E4 E4 F#4 | G4 F#4 F#4 E4 | E4:4 D4 -:2 D4:1 D4 | \
                E4:2 C4:4 -:2 !p C4 B3 | C4 D4:4 C4 B3:2 | B3 C4:4 -:2 B3 C4 | D4 C4:4 B3 C4:2 | \
                D4 C4:4 -:2 !f E4 D4 | E4 F#4:4 E4 D4:2 | D4 E4:6 D4:2 E4 | F#4 E4:4 D4 E4:2 | \
                F#4 E4:6 E4:2 D4 | E4 F#4:4 E4 D4:2 | D4 E4:6 D4:2 E4 | F#4 E4:4 D4 E4:2 | \
                F#4 E4:6 -:4 & !f C4:3 C4 C4 D4 | E4 D4 D4 C4 | C4:4 B3 -:2 B3:1 B3 | \
                C4:2 A3:4 -:6 | - !p E3:4 E3:2 | E3 E3:4 -:6 | - E3:4 E3:2 | \
                E3 E3:4 -:2 !f C#4 B3 | C#4 D4:4 C#4 B3:2 | B3 C#4:6 B3:2 C#4 | \
                D4 C#4:4 B3 C#4:2 | D4 C#4:6 C#4:2 B3 | C#4 D4:4 C#4 B3:2 | B3 C#4:6 B3:2 C#4 | \
                D4 C#4:4 B3 C#4:2 | D4 C#4:6 -:4",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 A2 | \
                _ A2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 94-112.
        Part::new(
            "Answers",
            "!f E4:6 D4:4 C#4:2 | B3:6 C#4:2 -:4 | -:12 | - | G4:6 F4:2 - E4 | D4:6 E4:2 -:4 | \
            -:12 | - | - | - | !mp A3:6 A3:4 G4:2 | G4:6 F#4:4 -:2 | A3:6 A3:4 G4:2 | \
            G4:6 F#4:4 -:2 | -:12 | -:10 E4:2 | D4 C#4:4 B3:2 A3:4 | G3:3 F#3 E3 F#3 | G3:8 -:4",
            "",
        )
        .in_key(A, Major)
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!f C#4:6 B3:4 A3:2 | G#3:6 A3:2 -:4 | -:12 | - | E4:6 D4:2 - C4 | B3:6 C4:2 -:4 | \
                -:12 | - | - | - | !mp A3:6 A3:4 E4:2 | E4:6 D4:4 -:2 | A3:6 A3:4 E4:2 | \
                E4:6 D4:4 -:2 | -:12 | -:10 C#4:2 | B3 A3:4 G3:2 F#3:4 | E3:3 D3 C#3 D3 | \
                E3:8 -:4",
            ),
            line(
                "Soloist 3",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | !f G3:6 G3:2 - G3 | G3:8 -:4 | -:12 | - | - | - | \
                !mp A3:6 A3:4 A3:2 | A3:6 A3:4 -:2 | A3:6 A3:4 A3:2 | A3:6 A3:4 -:2 | -:12 | - | \
                - | - | -",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:12 | - | !f E5:6 E5:4 E5:2 | E5:6 E5:2 -:4 | -:12 | - | G5:6 G5:2 - G5 | \
                G5:8 -:2 E5 | !mf B4+D5 A4+C#5:4 G4+B4:2 F#4+A4:4 | \
                D4+G4:2 D4+F#4:4 C#4+E4:2 D4+F#4:4 | -:12 | - | - | - | E5:6 D5:4 C#5:2 | \
                B4:6 C#5:4 -:2 | -:12 | - | -:8 !p A4:2 A4 & -:12 | - | \
                !f C#5:6 B4+D5:4 A4+C#5:2 | G#4+B4:6 A4+C#5:2 -:4 | -:12 | - | \
                E5:6 D5+F5:2 - C5+E5 | B4+D5:6 C5+E5:2 - A4+C#5 | !mf A4 E4:4 D4:2 C#4:4 | \
                B3:2 B3:4 A3:2 A3:4 | -:12 | - | - | - | C#5:6 B4:4 A4:2 | G#4:6 A4:4 -:2 | -:12 | \
                - | -:8 !p E4:2 E4 & -:12 | - | !f E4:6 E4:4 E4:2 | E4:6 E4:2 -:4 | -:12 | - | \
                G4:6 G4:2 - G4 | G4:8 -:2 E4 | !mf E4 E4:4 E4:2 E4:4 | E4:2 A3:4 A3:2 D3:4 | \
                -:12 | - | - | - | E4:6 E4:4 E4:2 | E4:6 E4:4 -:2 | -:12 | - | \
                -:8 !p C#4:2 C#4 & -:12 | - | !f E3:6 E3:4 E3:2 | E4:6 E4:2 -:4 | -:12 | - | \
                E4:6 D4:2 - C4 | B3:6 C4:2 -:4 | !mf E4:2 E4:4 E4:2 E4:4 | E4:2 A3:4 A3:2 D3:4 | \
                -:12 | - | - | - | A3:6 B3:4 C#4:2 | D4:6 C#4:4 -:2 | -:12 | - | -:8 !p G3:2 G3",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A2:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 - | - C3 C3 | _ C3 C3 | _ C3 C3 | \
                _ C3 - | A2:12 | _ | _:4 A2 A2 | _ A2 A2 | _ A2 A2 | _ A2 - | - E3 E3 | _ E3 - | \
                A2:12 | _ | _:8 G3:2 G3",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 113-121.
        Part::new(
            "Softly",
            "!p A4:2 A4:4 A4 A4:2 | A4:4 A4:2 -:6 | -:12 | - | - | -:8 !mf D5:2 C#5 | \
            D5 E5:4 D5 C#5:2 | C#5 D5:6 C#5:2 D5 | E5 D5:4 C#5 D5:2",
            "",
        )
        .in_key(A, Major)
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p E4:2 E4:4 E4 E4:2 | E4:4 E4:2 - D4 C#4 | D4 E4:4 D4 C#4:2 | C#4 D4:6 C#4:2 D4 | \
                E4 D4:4 C#4 D4:2 | E4 D4:4 -:2 !mf A4 A4 | A4 A4:4 A4 A4:2 | A4 A4:6 A4:2 A4 | \
                A4 A4:4 A4 A4:2 & !p C#4:2 C#4:4 C#4 C#4:2 | C#4:4 C#4:2 - A3 A3 | \
                A3 A3:4 A3 A3:2 | A3:8 A3:2 A3 | A3 A3:4 A3 A3:2 | A3 A3:4 -:2 !mf F#4 E4 | \
                F#4 G4:4 F#4 E4:2 | E4 F#4:6 E4:2 F#4 | \
                G4 F#4:4 F#4 F#4:2 & !p G3:2 G3:4 G3 G3:2 | G3:4 G3:2 - F#3 E3 | \
                F#3 G3:4 F#3 E3:2 | E3 F#3:6 E3:2 F#3 | G3 F#3:4 E3 F#3:2 | G3 F#3:4 -:6 | -:12 | \
                - | -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f G3:2 G3:4 G3 G3:2 | G3:4 G3:2 -:6 | -:4 A3 A3 | _ A3 A3 | _ A3 A3 | _ A3 A3 | \
                _ A3 A3 | _ A3 A3 | _ A3 A3",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 122-133.
        Part::new(
            "Soloists, bar 122",
            "-:8 !mp E4:2 E4 | E4 E4:4 E4 E4:2 | E4:4 E4 - | -:12 | - | E4:2 E4 E4 E4 E4 E4 | \
            E4:4 E4:3 -:1 E4:2 E4 | E4 E4:4 E4 E4:2 | E4:4 E4 - | -:12 | - | -",
            "",
        )
        .in_key(E, Major)
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:8 !mp E4:2 D4 | C#4 B3:4 E4 D4:2 | C#4:4 B3 - | -:12 | - | \
                E4:2 D4 C#4 B3 E4 D4 | C#4:4 B3:3 -:1 E4:2 D4 | C#4 B3:4 E4 D4:2 | C#4:4 B3 - | \
                -:12 | - | -",
            ),
            line(
                "Soloist 3",
                gm::VOICE_OOHS,
                "-:8 !mp C#4:2 B3 | A3 G#3:4 C#4 B3:2 | A3:4 G#3 - | -:12 | - | \
                C#4:2 B3 A3 G#3 C#4 B3 | A3:4 G#3:1 -:3 C#4:2 B3 | A3 G#3:4 C#4 B3:2 | \
                A3:4 G#3 - | -:12 | - | -",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!mf E5:2 D5:6 -:4 | -:12 | -:8 !mp B4:4 | B4:2 A4:4 G#4 F#4:2 | A4 G#4 F#4:4 - | \
                -:12 | - | - | - | - | - | - & !mf A4:2 A4:6 -:4 | -:12 | -:8 !mp G#4:4 | \
                G#4:2 F#4:4 E4 D#4:2 | F#4 E4 D#4:4 - | -:12 | - | - | - | - | - | \
                - & !mf G4:2 F#4:6 -:4 | -:12 | - | - | - | - | - | - | - | \
                !mp B3:2 A3 G#3 F#3 B3 A3 | G#3:4 F#3:3 -:1 B3:2 A3 | G#3 F#3:4 B3 A3:2 & -:12 | \
                - | - | - | - | - | - | - | - | !mp G#3:2 F#3 E3 D#3 G#3 F#3 | \
                E3:4 D#3:3 -:1 G#3:2 F#3 | E3 D#3:4 G#3 F#3:2",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f A3:4 A3 - | E3:12 | _ | B2 | _ | E3 | _ | E3 | _ | B2 | _ | B2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 -"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3"),
        ]),
        // Bars 134-149.
        Part::new(
            "Ending",
            "-:8 !mp A4:4 | G#4 -:2 C#5:6 | B4:4 -:2 C#5+E5:6 | B4+D5:4 -:2 A4+C#5:6 | \
            G#4+B4 -:2 A4+E5:4 | G#4+E5 -:2 A4+E5:6 | B4+E5:4 -:2 C#5+E5:6 | \
            B4+E5:4 -:2 C#5+E5:6 | E5:4 -:2 E5:6 | G#4+B4:4 -:2 !p E4+A4:6 | G#4:4 -:2 E4+A4:6 | \
            G#4:4 -:2 E4+A4:6 | G#4:4 -:2 E4+A4:6 | G#4:4 -:2 E4+A4:6 | G#4:4 -:2 E4+A4:6 | \
            G#4 -",
            "",
        )
        .in_key(E, Major)
        .with_lines(&[
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:8 !mp E4:4 | E4 -:2 A4:6 | G#4 A4 | E4:4 -:2 E4:6 | E4 -:2 E4:4 | E4 -:2 E4:6 | \
                E4:4 -:2 A4:6 | G#4:4 -:2 A4:6 | B4+D5:4 -:2 A4+C#5:6 | E4:4 -:2 !p A4:6 | \
                E4:4 -:2 A4:6 | E4:4 -:2 A4:6 | E4:4 -:2 A4:6 | E4:4 -:2 A4:6 | E4:4 -:2 A4:6 | \
                E4 - & !mp G#3:4 F#3:3 -:1 A3:4 | B3 -:2 E4:6 | E4:4 -:2 E4:6 | F#4:4 -:2 C#4:6 | \
                D4 -:2 C#4:4 | B3 -:2 C#4:6 | D4:4 -:2 F#4:6 | E4:4 -:2 E4:6 | E4:4 -:2 E4:6 | \
                D4:4 -:2 !p C#4:6 | B3:4 -:2 C#4:6 | B3:4 -:2 C#4:6 | B3:4 -:2 C#4:6 | \
                B3:4 -:2 C#4:6 | B3:4 -:2 C#4:6 | B3 - & !mp E3:4 D#3 C#3 | B2 -:2 A2:6 | \
                E3:4 -:2 F#3:6 | G#3:4 -:2 A3:6 | B3 -:2 E3:4 | E3 -:2 E3:6 | E3:4 -:2 E4:6 | \
                D4:4 -:2 C#4:6 | B3:4 -:2 A3:6 | E3:4 -:2 !p E3:6 | E3:4 -:2 E3:6 | \
                E3:4 -:2 E3:6 | E3:4 -:2 E3:6 | E3:4 -:2 E3:6 | E3:4 -:2 E3:6 | E3 -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!f B2:8 -:4 | - E2 E2 | _ E2 E2 | _ E2 E2 | _ E2 E2 | _ E2 E2 | _ E2 E2 | \
                _ E2 E2 | _ !mf E2 E2 | _ E2 E2 | _ E2 !mp E2 | _ E2 E2 | _ E2 E2 | _ !p E2 E2 | \
                _ E2 E2 | _:6 -",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | \
             -:8 x:1 -:3 | -:8 x:1 -:3 | -:8 x:1 -:3 | x:1 x x x x x x -:5"),
            (Drum::Rim, "x:1 -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | \
             x -:9 x:1 - | x -:9 x:1 - | x -:9 x:1 - | - x x x x x x -:5"),
            (Drum::HighTom, "-:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | -:2 x:1 - x -:3 x:1 -:3 | \
             x:1 -:11"),
        ])
        .with_tempo(&[glide(97, 70, 71), glide(172, 46, 8)]),
    ],
    form: &[
        "Pickup",
        "Intro",
        "Credo",
        "Credo, bar 30",
        "Soloists",
        "Choir",
        "Soloists and choir",
        "Choir, bar 78",
        "Answers",
        "Softly",
        "Soloists, bar 122",
        "Ending",
    ],
    drums_from: 0,
    about: "The third movement of the Misa Criolla is a chacarera trunca, a dance of \
            Santiago del Estero, in a quick 3/4. After a one-beat pickup the guitars swing \
            between A minor and D major, whose F sharp is A Dorian's raised sixth, the \
            harpsichord low under them and a flute high above; then the bombo comes in, \
            its head and its rim in a pattern two bars long. The choir and the soloists \
            take turns, and their phrases stop short before the third beat of each second \
            bar: trunca means cut off. Later the music turns to A major, and its last part \
            to E major, slowing down at the end as it swings between E major and A major \
            over E, and closing on E major.",
};

// Sources: see above. The sequence's "Tomtom", on two pitches, is the low and the high
// tom, its wood blocks the rim, its snare the snare, the loudest strokes accented. Its
// "Treble 2", a high line played on a piccolo in the sequence, is a flute here;
// "Strings R1" is the guitars, "Strings L" the bass.
pub(super) const MISA_CRIOLLA_SANCTUS: Song = Song {
    name: "Misa Criolla: Sanctus",
    origin: "Ariel Ramírez, Misa Criolla, 1964",
    feel: "Carnaval cochabambino",
    genre: Genre::Folklore,
    tonic: A,
    mode: Major,
    time: THREE_FOUR,
    tempo: 152.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "Carnaval cochabambino",
    band: MISA,
    style: "Carnaval cochabambino",
    parts: &[
        // Bars 1-12.
        Part::new(
            "Intro",
            "-:12 | - | - | - | - | - | - | - | - | - | - | -",
            "",
        )
        .with_left("!mf A2+E3+C#4:4 A2+E3+C#4 A2+E3+C#4 | A2+E3:12 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | \
            A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 & -:12 | -:4 !mf C#4:8 | -:4 C#4:8 | \
            -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | \
            -:4 C#4:8 | -:4 C#4:8")
        .with_lines(&[
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "(3 !mp E4+A4:2 _ - (3 E4+A4:2 _ - (3 E4+A4:2 _ - | E4+A4:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 !p A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 & -:1 !mp C#5 (3 _:1 - - -:1 C#5 (3 _:1 - - -:1 C#5 (3 _:1 - - | \
                -:1 C#5:5 -:6 | -:1 E5:5 -:6 | -:1 !p E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 & -:2 (3 !mp E5:1 - - -:2 (3 E5:1 - - -:2 (3 E5:1 - - | \
                -:2 E5:4 -:6 | -:12 | - | - | - | - | - | - | - | - | -",
            ),
            line(
                "Flute",
                gm::FLUTE,
                "-:12 | - | - | - | -:10 !mf E6:2 | E6:6 (3 A6:1 G#6 A6 E6:2 - | \
                E6:6 (3 !mp A6:1 G#6 A6 E6:2 - | E6:6 (3 !p A6:1 G#6 A6 E6:2 - | \
                E6:6 (3 A6:1 G#6 A6 E6:2 - | E6:6 (3 A6:1 G#6 A6 E6:2 - | \
                E6:6 (3 A6:1 G#6 A6 E6:2 - | E6:6 (3 A6:1 G#6 A6 E6:2 -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "-:12 | - | - | -:4 !mf E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | \
                _ E2 A2 | _ E2 A2 | _ E2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::Rim, "-:12 | - | - | - | -:10 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 -"),
            (Drum::Snare, "-:12 | - | - | - | - | - | - | - | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3"),
        ]),
        // Bars 13-19.
        Part::new(
            "Intro, bar 13",
            "-:12 | - | - | - | - | - | -",
            "",
        )
        .with_left("!mf A2+E3:12 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | !mp A2+E3 & -:4 !mf C#4:8 | \
            -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 !mp C#4:8")
        .with_lines(&[
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!p A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 !mp A5:2 A4+C#5+E5:4 & -:1 !p E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6",
            ),
            line(
                "Flute",
                gm::FLUTE,
                "!p E6:6 (3 A6:1 G#6 A6 E6:2 - | E6:6 (3 A6:1 G#6 A6 E6:2 - | \
                E6:6 (3 A6:1 G#6 A6 E6:2 - | E6:6 (3 A6:1 G#6 A6 E6:2 - | \
                E6:6 (3 !pp A6:1 G#6 A6 E6:2 - | E6:6 (3 A6:1 G#6 A6 E6:2 - | \
                E6:6 (3 A6:1 G#6 A6 E6:2 -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mf A2:4 E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 !mp A2 | _ E2 A2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5"),
            (Drum::Rim, "x:1 -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 -"),
            (Drum::Snare, "-:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3"),
        ]),
        // Bars 20-40.
        Part::new(
            "Sanctus",
            "!mf E4:6 C#4:2 -:4 | C#4:6 A3:2 -:4 | A3:12 | _ | E4:8 -:4 | -:12 | -:10 E3:2 | \
            A3:11 -:1 | A3:4 B3:2 C#4:4 A3:2 | B3:6 E4 | E3:12 | _:11 -:1 | D4:6 B3:2 -:4 | \
            B3:6 G#3:2 -:4 | G#3:12 | _ | G#4:8 -:4 | -:12 | -:10 E3:2 | G#3:11 -:1 | \
            G#3:4 A3:2 B3:4 G#3:2",
            "",
        )
        .with_left("!mp A2+E3:12 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | \
            B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | \
            B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 & -:4 !mp C#4:8 | \
            -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | \
            -:4 C#4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | \
            -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8")
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!mf E4:6 C#4:2 -:4 | C#4:6 A3:2 -:4 | A3:12 | _ | E4:8 -:4 | -:12 | -:10 E3:2 | \
                A3:11 -:1 | A3:4 B3:2 C#4:4 A3:2 | B3:6 E4 | E3:12 | _:11 -:1 | D4:6 B3:2 -:4 | \
                B3:6 G#3:2 -:4 | G#3:12 | _ | E4:8 -:4 | -:12 | -:10 E3:2 | G#3:11 -:1 | \
                G#3:4 A3:2 B3:4 G#3:2",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:12 | - | - | - | !f E5+A5:6 C#5+E5:2 -:4 | C#5+E5:6 A4+C#5:2 -:4 | A4+C#5:12 | \
                C#5+E5 | - | - | - | - | - | - | - | - | B4+D5:6 G#4+B4:2 -:4 | \
                G#4+B4:6 D4+G#4:2 -:4 | !mf D4+G#4:12 | G#4+B4 | - & -:12 | - | - | - | \
                !f C#5:6 A4:2 -:4 | A4:6 E4:2 -:4 | E4:12 | A4 | - | - | - | - | - | - | - | - | \
                G#4:6 D4:2 -:4 | D4:6 B3:2 -:4 | !mf B3:12 | D4 | -",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!mp A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 & -:1 !mp E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mp A2:4 E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | \
                _ E2 A2 | _ E2 A2 | B2 E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | \
                _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5"),
            (Drum::Rim, "x:1 -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 -"),
            (Drum::Snare, "-:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3"),
        ]),
        // Bars 41-61.
        Part::new(
            "Sanctus, bar 41",
            "!mf A3:6 C#4 | E3:12 | _:11 -:1 | F#4:4 E4 D4 | D4:6 C#4 | C#4:12 | _ | E4 | _ | \
            _:8 E3:2 E3 | A3:4 E3:8 | _:4 D4 C#4 | B3:12 | E3 | _ | - | D4:4 C#4 B3 | B3:6 A3 | \
            G#3:12 | _ | E4",
            "",
        )
        .with_left("!mp A2+E3:12 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | A2+E3 | \
            A2+E3 | A2+E3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | \
            B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 & -:4 !mp C#4:8 | -:4 C#4:8 | \
            -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | \
            -:4 C#4:8 | -:4 C#4:8 | -:4 C#4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | \
            -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8")
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!mf A3:6 C#4 | E3:12 | _:11 -:1 | D4:4 C#4 B3 | B3:6 A3 | A3:12 | _ | C#4 | _ | \
                _:8 E3:2 E3 | A3:4 E3:8 | _:4 D4 C#4 | B3:12 | E3 | _ | - | B3:4 A3 G#3 | \
                G#3:6 F#3 | E3:12 | _ | B3",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!f C#5+E5:12 | A4+C#5 | _ | - | - | - | - | - | - | - | - | - | - | - | - | - | \
                - | - | - | - | - & !f A4:12 | E4 | _ | - | - | - | - | - | - | - | - | - | - | \
                - | - | - | - | - | - | - | -",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!mp A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5:6 A5:2 A4+C#5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 & -:1 !mp E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mp A2:4 E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | \
                _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | _ E2 A2 | B2 E2 B2 | _ E2 B2 | _ E2 B2 | \
                _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3"),
            (Drum::HighTom, "x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5"),
            (Drum::Rim, "x:1 -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 -"),
            (Drum::Snare, "-:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3"),
        ]),
        // Bars 62-67.
        Part::new(
            "Sanctus, bar 62",
            "!mf E4:12 | _:8 E3:2 E3 | F#4:6 E4 | E4 D4 | C#4 E4 | E3:4 -:8",
            "",
        )
        .with_left("!mp B2+E3+G#3:12 | B2+E3+G#3 | B2+E3+G#3 | B2+E3+G#3 | A2+E3 | \
            A2+E3+C#4:4 -:8 & -:4 !mp D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 D4:8 | -:4 C#4:8 | -:12")
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!mf B3:12 | _:8 E3:2 E3 | D4:6 C#4 | C#4 B3 | A3 C#4 | E3:4 -:8",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!mp G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                G#4+D5:6 B5:2 G#4+D5+E5:4 | G#4+D5:6 B5:2 G#4+D5+E5:4 | \
                A4+C#5:6 A5:2 A4+C#5+E5:4 | A4+C#5+E5 -:8 & -:1 !mp E5:5 -:6 | -:1 E5:5 -:6 | \
                -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:1 E5:5 -:6 | -:12",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mp B2:4 E2 B2 | _ E2 B2 | _ E2 B2 | _ E2 B2 | A2 D2 A2 | A2 -:8",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | \
             -:4 x:1 -:3 x:1 -:3 | -:4 x:1 -:3 x:1 -:3 | -:12"),
            (Drum::HighTom, "x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | x:1 -:5 x:1 -:5 | \
             x:1 -:5 x:1 -:5 | x:1 -:11"),
            (Drum::Rim, "x:1 -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | \
             x -:5 x:1 -:3 x:1 - | x -:5 x:1 -:3 x:1 - | -:12"),
            (Drum::Snare, "-:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | \
             -:4 X:1 X X X x -:3 | -:4 X:1 X X X x -:3 | x:1 -:11"),
        ]),
        // Bars 68-73.
        Part::new(
            "Interlude",
            "-:12 | - | - | - | - | -",
            "",
        )
        .with_left("!mp D2+A2+F#3:4 D2+A2+F#3 D2+A2+F#3 | D2+A2+F#3:12 | E2+E3:4 E2+E3 E2+E3 | E2+E3:12 | \
            E2:4 E2 E2 | E2:12")
        .with_lines(&[
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "!mp D4+A4:4 D4+A4+B4+D5 D4+A4+B4+D5 | D4+A4+B4+D5:12 | \
                E4+A4:4 E4+A4+C#5+E5 E4+A4+C#5+E5 | E4+A4+C#5+E5:12 | \
                D4+E4+G#4+B4+E5:4 D4+E4+G#4+B4+E5 D4+E4+G#4+B4+E5 | \
                D4+E4+G#4+B4+E5:12 & -:1 !mp B4:3 -:8 | -:12 | -:1 C#5:3 -:8 | -:12 | - | \
                - & -:2 !mp D5 -:8 | -:12 | -:2 E5 -:8 | -:12 | - | -",
            ),
            line(
                "Flute",
                gm::FLUTE,
                "!mf D6:3 -:1 D6:3 -:1 D6:3 -:1 | D6:12 | E6:3 -:1 E6:3 -:1 E6:3 -:1 | E6:12 | \
                E6:3 -:1 E6:3 -:1 E6:3 -:1 | E6:12",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mp D3:4 D3 D3 | D3:12 | E3:4 E3 E3 | E3:12 | E3:4 E3 E3 | E3:12",
            ),
        ])
        .with_drums(&[
            (Drum::HighTom, "x:1 -:3 x:1 -:3 x:1 -:3 | x:1 -:11 | x:1 -:3 x:1 -:3 x:1 -:3 | x:1 -:11 | \
             x:1 -:3 x:1 -:3 x:1 -:3 | x:1 -:11"),
            (Drum::Rim, "-:12 | - | - | - | - | x:1 -:11"),
            (Drum::Snare, "x:1 -:3 x:1 -:3 x:1 -:3 | x:1 -:11 | x:1 -:3 x:1 -:3 x:1 -:3 | x:1 -:11 | \
             x:1 -:3 x:1 -:3 x:1 -:3 | x:1 -:11"),
        ]),
        // Bars 74-76.
        Part::new(
            "In four",
            "-:16 | - | -",
            "",
        )
        .in_time(COMMON)
        .with_left("!mp A1+E2:4 D2+A2 E2+B2 G2+D3 | A2+E3 D3+A3 E3+B3 G3+D4 | A3+E4:12 -:4")
        .with_lines(&[
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "(3 !mp A2:2 E3 A3 (3 D3:2 A3 D4 (3 E3:2 B3 E4 (3 A3:2 E4 A4 | \
                (3 A3:2 E4 A4 (3 D4:2 A4 D5 (3 E4:2 B4 E5 (3 G4:2 D5 G5 | A4+E5+A5:12 -:4",
            ),
            line(
                "Flute",
                gm::FLUTE,
                "(3 !mf A3:2 E4 A4 (3 D4:2 A4 D5 (3 E4:2 B4 E5 (3 A4:2 E5 A5 | \
                (3 A4:2 E5 A5 (3 D5:2 A5 D6 (3 E5:2 B5 E6 (3 G5:2 D6 G6 | \
                G6:4 (3 F#6:1 G6 F#6 (3 G6:1 F#6 G6 A6:4 -",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[fermata(44, 4)]),
        // Bars 77-88.
        Part::new(
            "Voices",
            "!mf E4:2 E4:4 E4 D4:2 | F#4:4 E4 E4 | _ -:8 | -:12 | - | - | !ff E4:2 E4:4 E4 D4:2 | \
            F#4 E4:4 E4 E4:2 | _:12 | - | - | -",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!mf C#4:2 C#4:4 C#4 B3:2 | D4:4 C#4 C#4 | _ -:8 | -:12 | - | - | \
                !ff C#4:2 C#4:4 C#4 B3:2 | D4 C#4:4 C#4 C#4:2 | _:12 | - | - | -",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "-:12 | - | !p A4:2 -:1 A4:2 -:1 G4:3 -:1 (3 B4:1 _ - | \
                A4:3 -:1 A4:3 -:1 (3 E4:1 _ - (3 E4:1 _ - | \
                A4:2 -:1 A4:2 -:1 G4:3 -:1 (3 B4:1 _ - | A4:8 -:4 | -:12 | - | - | \
                !ff E5:2 E5 (3 _:1 _ - E5:2 (3 _:1 _ - D5:2 | \
                F#5 (3 _:1 _ - E5:2 (3 _:1 _ - E5:4 | _:11 -:1 & -:12 | - | \
                !p E4:2 -:1 E4:2 -:1 D4:3 -:1 (3 G4:1 _ - | \
                E4:3 -:1 E4:3 -:1 (3 C#4:1 _ - (3 C#4:1 _ - | \
                E4:2 -:1 E4:2 -:1 D4:3 -:1 (3 G4:1 _ - | E4:8 -:4 | -:12 | - | - | \
                !ff C#5:2 C#5 (3 _:1 _ - C#5:2 (3 _:1 _ - B4:2 | \
                D5 (3 _:1 _ - C#5:2 (3 _:1 _ - C#5:4 | _:11 -:1 & -:12 | - | \
                !p C#4:2 -:1 C#4:2 -:1 B3:3 -:1 (3 D4:1 _ - | \
                C#4:3 -:1 C#4:3 -:1 (3 A3:1 _ - (3 A3:1 _ - | \
                C#4:2 -:1 C#4:2 -:1 B3:3 -:1 (3 D4:1 _ - | C#4:8 -:4 | -:12 | - | - | \
                !ff A4:2 A4 (3 _:1 _ - A4:2 (3 _:1 _ - A4:2 | A4 (3 _:1 _ - A4:2 (3 _:1 _ - A4:4 | \
                _:11 -:1 & -:12 | - | !p A3:2 -:1 A3:2 -:1 A3:3 -:1 (3 A3:1 _ - | \
                A3:3 -:1 A3:3 -:1 (3 A3:1 _ - (3 A3:1 _ - | \
                A3:2 -:1 A3:2 -:1 A3:3 -:1 (3 A3:1 _ - | A3:8 -:4 | -:12 | - | - | !ff A3:11 -:1 | \
                A3:11 -:1 | A3:11 -:1",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "-:12 | - | - | - | - | - | - | - | - | - | -:8 !mp A4+C#5+E5+A5+C#6:4 | _:12",
            ),
        ])
        .with_drums(&[
            (Drum::LowTom, "-:12 | - | - | - | - | - | - | - | - | - | -:7 x:1 x - x x | - x x - x x x -:5"),
            (Drum::HighTom, "-:12 | - | - | - | - | - | - | - | - | - | -:6 x:1 - x x x - | x - x x x - x -:5"),
            (Drum::Rim, "-:12 | - | - | - | - | - | - | - | - | - | -:6 x:1 - x x x - | x - x x x - x -:5"),
        ])
        .with_tempo(&[glide(72, 60, 56)]),
    ],
    form: &[
        "Intro",
        "Intro, bar 13",
        "Sanctus",
        "Sanctus, bar 41",
        "Sanctus, bar 62",
        "Interlude",
        "In four",
        "Voices",
    ],
    drums_from: 0,
    about: "The fourth movement of the Misa Criolla is a carnaval cochabambino, the \
            carnival dance of Cochabamba, in Bolivia, in a quick 3/4 and A major. The \
            guitars and the harpsichord open on the A major chord, a flute plays above \
            them, and the drums come in one by one: a wood block, a snare with its burst \
            of four loud sixteenths, and two drums answering each other. The two soloists \
            sing and the choir answers them. Three bars of 4/4 broaden into a held chord; \
            then the voices sing alone, softly and then loudly, slowing down, and the drums \
            come back for the last bars, on A major.",
};

// Sources: see above. The sequence's "Strings R1" and "Strings R2" are the two guitars;
// its bass line, doubled on two tracks, is played once, by the bass. There are no drums.
pub(super) const MISA_CRIOLLA_AGNUS_DEI: Song = Song {
    name: "Misa Criolla: Agnus Dei",
    origin: "Ariel Ramírez, Misa Criolla, 1964",
    feel: "Estilo pampeano",
    genre: Genre::Folklore,
    tonic: A,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 44.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SONG_AND_GUITAR,
    style: "Estilo",
    parts: &[
        // Bars 1-7.
        Part::new(
            "Intro",
            "-:12 | - | - | - | - | - | -:11 !mp C4:1",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | - | - | -:11 !mp C4:1",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "-:4 !mp A3+C4:6 (3 _:1 _ - | -:2 A3+D4+F4+A4 - A3+C4+E4+A4 - G#3+D4+E4+G#4 | \
                -:4 A3+C4:6 (3 _:1 _ - | -:1 A3 D4+F4+A4 A3 - A3 C4+E4+A4 A3 - G#3 D4+E4+G#4 G#3 | \
                -:4 A3+C4:6 (3 _:1 _ - | -:2 !p A3+D4+F4+A4 - A3+C4+E4+A4 - G#3+D4+E4+G#4 | \
                -:5 A3:6 -:1 & -:5 !mp E4 (3 _:1 _ - | -:12 | -:5 E4 (3 _:1 _ - | -:12 | \
                -:5 E4 (3 _:1 _ - | -:12 | -:6 !p C4:5 -:1 & -:6 !mp A4:4 (3 _:1 _ - | -:12 | \
                -:6 A4:4 (3 _:1 _ - | -:12 | -:6 A4:4 (3 _:1 _ - | -:12 | -:7 !p E4:4 -:1 & -:12 | \
                - | - | - | - | - | -:8 !p A4:3 -:1 & -:12 | \
                !mp D5:2 (3 _:1 _ - C5:2 (3 _:1 _ - B4:2 (3 _:1 _ - | A4:11 -:1 | \
                D5:3 E5:1 C5:3 D5:1 B4:3 C5:1 | A4:11 -:1 | -:12 | - & -:12 | - | - | \
                -:4 !mp D5:1 -:3 C5:1 -:3 | B4:1 -:11 | -:12 | -",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!mp A1:12 | D2:3 -:1 C2:3 -:1 B1:3 -:1 | A1:12 | D2:4 C2 B1 | A1:12 | \
                !p D2:3 -:1 C2:3 -:1 B1:3 -:1 | A1:12",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[tempo(4, 30), tempo(12, 44), glide(61, 20, 19)]),
        // Bars 8-18.
        Part::new(
            "Agnus Dei",
            "!mp C4:8 E4:2 C4 | B3:4 !p B3:2 A3:6 | G#3:10 -:2 | - !mp D4 D4 E4:6 | \
            _:2 A3 G#3 B3:6 | _:12 | A3:8 -:4 | E3:2 F#3 G#3 A3 G#3 A3 | E4:8 D4:2 C4 | \
            D4:8 C4:2 B3 | C4:10 -:2",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!mp C4:8 E4:2 C4 | B3:4 !p B3:2 A3:6 | G#3:10 -:2 | - !mp D4 D4 E4:6 | \
                _:2 A3 G#3 B3:6 | _:12 | A3:8 -:4 | E3:2 F#3 G#3 A3 G#3 A3 | C4:8 B3:2 A3 | \
                B3:8 A3:2 G#3 | A3:10 -:2",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p E3+C4:12 | D3+B3:6 D#3+A3 | E3+G#3:10 -:2 | A3:12 | _:6 - | - G#3+D4 | \
                E3+C4:12 | - | - | - | - & -:12 | - | - | !p F4:6 E4 | _ - | -:12 | - | - | - | \
                - | - & !p A2:12 | G2:6 F2 | E2:10 -:2 | D3:6 C3 | _ - | - B2 | A2:12 | - | - | \
                - | -",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[tempo(0, 44)]),
        // Bars 19-23.
        Part::new(
            "Interlude",
            "-:12 | - | - | - | -:10 !p C4:2",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | -:10 !p C4:2",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p D4+F4:12 | C4+E4 | G#3+D4 | _ | A3+C4:8 -:4 & !p D3+A3:12 | C3 | B2+E3 | _ | \
                A2+E3:8 -:4 & -:12 | !p B3:6 A3 | -:12 | - | -",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "-:1 !p D4 A4 F4 D5 A4 F5 E5 D5 C5 B4 D5 | - C4 A4 E4 C5 A4 E5 D5 C5 B4 A4 C5 | \
                - B3 G#4 E4 B4 G#4 D5 C5 B4 A4 G#4 B4 | F5 E5 D5 C5 B4 A4 G#4 A4 B4 C5 D5 B4 | \
                C5:8 -:4",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!p D3:12 | E3 | E2+E3 | _ | A2:8 -:4",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[tempo(0, 38), glide(1, 75, 5), glide(7, 36, 5), glide(13, 75, 5), glide(19, 36, 5), glide(25, 72, 5), glide(31, 34, 4), glide(36, 75, 6), glide(43, 29, 4), tempo(48, 32), tempo(49, 43)]),
        // Bars 24-34.
        Part::new(
            "Agnus Dei, bar 24",
            "!p C4:8 E4:2 C4 | B3:4 B3:2 A3:6 | G#3:10 -:2 | - !mp D4 D4 E4:6 | _:2 A3 G#3 B3:6 | \
            _:12 | A3:8 -:4 | E3:2 F#3 G#3 A3 G#3 A3 | E4:8 D4:2 C4 | D4:8 C4:2 B3 | C4:10 -:2",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!p C4:8 E4:2 C4 | B3:4 B3:2 A3:6 | G#3:10 -:2 | - !mp D4 D4 E4:6 | \
                _:2 A3 G#3 B3:6 | _:12 | A3:8 -:4 | E3:2 F#3 G#3 A3 G#3 A3 | C4:8 B3:2 A3 | \
                B3:8 A3:2 G#3 | A3:10 -:2",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p E3+C4:12 | D3+B3:6 D#3+A3 | E3+G#3:10 -:2 | A3:12 | _:6 - | - G#3+D4 | \
                E3+C4:12 | - | - | - | - & -:12 | - | - | !p F4:6 E4 | _ - | -:12 | - | - | - | \
                - | - & !p A2:12 | G2:6 F2 | E2:10 -:2 | D3:6 C3 | _ - | - B2 | A2:12 | - | - | \
                - | -",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[tempo(0, 44)]),
        // Bars 35-39.
        Part::new(
            "Interlude again",
            "-:12 | - | - | - | -:10 !p C4:2",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | -:10 !p C4:2",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p D4+F4:12 | C4+E4 | G#3+D4 | _ | A3+C4:8 -:4 & !p D3+A3:12 | C3 | B2+E3 | _ | \
                A2+E3:8 -:4 & -:12 | !p B3:6 A3 | -:12 | - | -",
            ),
            line(
                "Guitars",
                gm::NYLON_GUITAR,
                "-:1 !p D4 A4 F4 D5 A4 F5 E5 D5 C5 B4 D5 | - C4 A4 E4 C5 A4 E5 D5 C5 B4 A4 C5 | \
                - B3 G#4 E4 B4 G#4 D5 C5 B4 A4 G#4 B4 | F5 E5 D5 C5 B4 A4 G#4 A4 B4 C5 D5 B4 | \
                C5:8 -:4",
            ),
            line(
                "Bass",
                gm::ACOUSTIC_BASS,
                "!p D3:12 | E3 | E2+E3 | _ | A2:8 -:4",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[tempo(0, 38), glide(1, 75, 5), glide(7, 36, 5), glide(13, 75, 5), glide(19, 36, 5), glide(25, 72, 5), glide(31, 34, 4), glide(36, 75, 6), glide(43, 29, 4), tempo(48, 32), tempo(49, 43)]),
        // Bars 40-46.
        Part::new(
            "Agnus Dei, bar 40",
            "!p C4:8 E4:2 C4 | B3:4 B3:2 A3:6 | G#3:10 -:2 | - D4 D4 E4:6 | _:2 A3 G#3 B3:6 | \
            _:12 | A3",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "!p C4:8 E4:2 C4 | B3:4 B3:2 A3:6 | G#3:10 -:2 | - D4 D4 E4:6 | _:2 A3 G#3 B3:6 | \
                _:12 | A3",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p E3+C4:12 | D3+B3:6 D#3+A3 | E3+G#3:10 -:2 | A3:12 | _:6 - | - G#3+D4 | \
                E3+C4:12 & -:12 | - | - | !p F4:6 E4 | _ - | -:12 | - & !p A2:12 | G2:6 F2 | \
                E2:10 -:2 | D3:6 C3 | _ - | - B2 | A2:12",
            ),
        ])
        .played(Comping::Held, Bass::None, ""),
        // Bars 47-55.
        Part::new(
            "Ending",
            "-:12 | - | - | - | - | - | !pp A3:4 A3 G#3 | A3:12 | _",
            "",
        )
        .with_lines(&[
            line(
                "Soloist 2",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | - | - | !pp F3:4 F3 F3 | F3:12 | E3",
            ),
            line(
                "Soloist 3",
                gm::VOICE_OOHS,
                "-:12 | - | - | - | - | - | !pp C3:4 C3 B2 | D3:12 | C3",
            ),
            line(
                "Choir",
                gm::CHOIR_AAHS,
                "!p A4+D5:4 A4+D5 A4+C#5 | A4+D5:12 | - | - | !pp A4:4 A4 G#4 | A4:12 | - | \
                F4:4 F4 D4 | E4:12 & !p F4:4 F4 E4 | F4:12 | - | - | !pp F4:4 F4 F4 | F4:12 | - | \
                D4:4 D4 D4 | E4:12 & -:12 | - | !pp C4+E4:4 C4+E4 B3+D4 | C4+E4:11 -:1 | \
                C4:4 C4 C4 | C4:12 | - | F3:4 A3 B3 | C4:12 & -:12 | - | !pp A3:4 A3 A3 | \
                A3:11 -:1 | D3:12 | D3 | - | A2 | A2",
            ),
        ])
        .played(Comping::Held, Bass::None, "")
        .with_tempo(&[glide(72, 30, 24)]),
    ],
    form: &[
        "Intro",
        "Agnus Dei",
        "Interlude",
        "Agnus Dei, bar 24",
        "Interlude again",
        "Agnus Dei, bar 40",
        "Ending",
    ],
    drums_from: 0,
    about: "The last movement of the Misa Criolla is an estilo pampeano, the slow song of \
            the Pampas to the guitar, in 3/4 and A minor, with no drums. The guitar opens \
            on A minor, then D minor, A minor and E seventh, a beat each, while a second \
            guitar plays a falling line above it, with turns, and the bass walks down, A, \
            D, C, B. The soloists sing over the choir's low voices, softly; between their \
            verses the guitar runs in sixteenths, speeding up into the middle of each bar \
            and slowing down at its end, in the free time of a gaucho's guitar. It ends \
            very softly, the choir moving from D minor to A minor: a plagal close.",
};
