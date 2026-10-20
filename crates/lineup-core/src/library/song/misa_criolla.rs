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
            the tonic major, before the Kyrie comes back in A minor and ends under a \
            fermata.",
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
        .with_tempo(&[tempo(28, 70), glide(40, 20, 4), tempo(48, 25)]),
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
