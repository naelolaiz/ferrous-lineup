//! Tangos and a vals played on the piano, written out from MIDI files of
//! pianists playing freely, without a click: the beats and bar lines were
//! found from the playing, and the tempo follows the player bar by bar. The
//! tune, which the singer sings, is the top line, on the Lead channel;
//! everything else the pianist plays is the left hand. The chords on the
//! chord sheet were read from the notes. Where a lead sheet of the song was
//! found, the reading keeps to the chords that sheet uses, moved to the key
//! the pianist plays in, and the parts follow the song's verse and refrain,
//! each with its own key; for the others it reads the notes alone. A note
//! outside the key is spelled as the chord over it has it (A sharp in F#7,
//! B flat in Gm), or, when the chord does not hold it, by where it goes: a
//! semitone up as a sharp, down as a flat.

use super::*;

/// The tune and the accompaniment on one piano.
const SOLO_PIANO: Band = Band::new(
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_BASS,
);

// Copyright: Mores died in 2016; not public domain; added at the owner's request.
// Written out from a piano performance (MIDI in 2/4, here two bars to a bar of 4/4).
pub(super) const UNO: Song = Song {
    name: "Uno",
    origin: "Mariano Mores, words by Enrique Santos Discépolo, 1943",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 90.2,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:4 G4:1 - G4 - G4 G4 G4 - G4 - F#4 G4 | \
            G#4 - G#4 - G#4 G#4 G#4 - G#4 G4 G#4 C4 A4 - A4 - | \
            A4 A4 A4 - A4 G#4 A4 D4 Bb4:3 D4:1 -:2 A4 | \
            _ -:1 F4 A4 - A4 - A4 A4 A4 F4 A4 - G#4 A4 | \
            Bb4 - Bb4 - Bb4 Bb4 Bb4 - Bb4 A4 Bb4 D4 B4 - B4 - | \
            B4 B4 B4 - B4 A#4 B4 D4 C5:3 D4:1 - D4 B4:2 | \
            _ -:1 G4 D5 C5:2 D5:1 C5 D5 - D5 C5 B4 C5 C5 | \
            B4 A4 B4 A4 B4 A4 B4 B4 A4 E4 -:3 E4:1 C#4:2",
            "C | C C E Am | Am Am C7/E A7 | A Dm Dm Dm | Bb/D Bb Bb G7 | G7 | C | B7/D# A7",
        )
        .with_left("-:4 C2 G2:1 - E3+E4 - C4+E4 G2 -:2 | \
            C2 C4+E4:1 - G#2+C4+E4 C4+E4 E3+C4+E4 - C4+E4 G#2 -:2 C2+E4:1 - C4+E4 - | \
            A2+C4+E4 C4+E4 E3+C4+E4 - C4:2 - E2:3 E3+Bb3:1 - A3:3 | \
            _:1 A2 - D4 D2:3 -:1 A2+D4+F4 D4+F4 F3 D4 D4+F4 A2 -:2 | \
            D2 D4+F4:1 - Bb2+D4+F4 D4+F4 F3+D4+F4 - D4+F4 Bb2 -:2 D2+F4:1 - D4+F4 - | \
            B2+D4+F4 D4+F4 F3+D4+F4 - D4+F4 B2 -:2 F4:5 F3:3 | _:1 G2 - E4:7 G3:1 C5 C2 - Eb2 B4 | \
            D#3:4 C4:2 D#3:1 -:2 E2 G3:1 Bb3 D4 A2:2 & -:4 C4+E4:1 -:11 | C4+E4:1 -:15 | \
            -:4 E4:1 A2 -:2 G4:1 -:4 G3:3 | -:6 D4+F4:1 -:9 | D4+F4:1 -:15 | \
            -:8 D2:3 -:2 G2:1 F4:2 | -:4 C2:1 -:3 E3:2 -:6 | -:1 F#3:6 -:3 E3:2 -:3 G3:1 & -:16 | \
            - | -:13 A2:1 C#4:2 | _ -:14 | -:16 | -:10 D3:1 -:5 | -:16 | -:2 A3:4 -:10 & -:16 | \
            - | -:14 E3:2 | _:1 -:15 | -:16 | - | - | -")
        .with_tempo(&[tempo(16, 92), tempo(64, 94), tempo(80, 92), tempo(96, 89), tempo(112, 81)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "-:3 A4:2 -:1 G4 A4 G4 A4:2 G4:1 - A4 G4 D4 | \
            _ -:2 C#4:1 D4:3 -:1 G4:2 F4 G4:1 F4 - G4 | G4 F4 E4 F4 G4 -:9 D5:2 | \
            C5 D5:1 C5 D5 C5 D5 B4 C5 C5 B4:2 A4:1 - B4 A4 | \
            B4 B4 C#5 A4 C5:3 E4:1 B4 C5 B4 C5 B4 C5 A4 B4 | \
            A4 -:2 C#4:1 E4 A4 -:2 E4:1 G4 - F4 E4 F4 C5 - | \
            A4 - G4 F4 E4:3 -:1 E4 D4 C4 -:3 C4:1 G4 | _:11 D4:2 -:1 E4:2",
            "Dm/A Dm | G G Dm7 G7 | G7 C Em7 Dm7 | Dm7/C B7 | B7/F# C C/G G | A7 Dm7 | Bb7 | \
            G Am7 C7 C7",
        )
        .with_left("-:1 A3:5 -:2 F3 - D2:1 -:3 | G2+B3 A2+Bb2+C4:1 B2:3 -:1 C4:5 -:1 F3+B3:2 | \
            _:1 G2 -:2 C2+E4:1 - C3+E3:2 D3+F3:1 E3+G3:2 -:1 C2 - F4:2 | \
            _:5 -:1 F2+C5 -:2 F#2+B4:1 - D#3:5 | \
            C4:1 A4 D#3+A4 G#4 G2 E3:3 -:1 C4:2 -:1 G3 B4 -:2 | \
            A2:5 -:2 A2:1 - D2:2 D3+D4:1 C#4 D4 F3+A4 - | F4 D2 E4 - F3:8 Ab3:1 -:3 | \
            -:2 B3:4 -:2 Bb2+C#4:3 B2:2 -:3 & -:4 D2:1 D3 -:3 A3:1 -:6 | - G2:2 D4:4 D3:1 - G2 - | \
            F4 -:8 Eb3+F#3:1 -:4 A4:2 | _:3 -:1 F3 -:7 F#3:4 | _:2 -:5 G3:3 -:6 | -:1 E3:5 -:10 | \
            -:4 Bb2:7 -:5 | -:2 G2:3 C4:5 -:6 & -:16 | -:9 D2:3 -:4 | -:15 F2:1 | C3:5 -:9 A3:2 | \
            _ -:14 | -:3 G3:4 -:9 | -:4 Ab3:5 -:7 | -:5 A2:3 -:8 & -:16 | - | - | -:2 A3 -:12 | \
            -:16 | -:3 A3:1 -:12 | -:8 D4:1 -:7 | -:16")
        .with_tempo(&[tempo(0, 85), tempo(32, 87), tempo(48, 92), tempo(64, 88), tempo(80, 82), tempo(96, 89), tempo(112, 86)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "F4:1 E4 -:2 F4:1 - F4 E4 D4 C4 E4:6 | _ C4:1 - E4:2 F4:1 G4 - A4 -:2 | - D4:12 -:2 | \
            F4 G4:1 F4 - G4 F4 G4 E4 D4 F4:6 | _ D4:1 F4:2 G4:1 A4 Eb4 B4:2 G4:1 B4 | \
            E4:4 D4:1 C4 - C4 -:5 E4:2 F4:1 | - G4:2 A4:1 B4 C5 G4 B4 B4 - B4 C5 B4 C5 B4 C5 | \
            B4 - C5:2 B4:1 C5 B4:2 C5:1 B4 C5 A4:4 D4:1",
            "Em/G C | Am Am Em A7 | Dm Bb | G7 Dm | G7 | C Em | G7 C C C | B B/D# B7 Dm7",
        )
        .with_left("-:2 G2:4 E4:1 -:3 C2+C3:1 - C2+C3+G3+C4 -:2 C2+C3+G3+C4:1 | \
            C2+C3+G#3+C4 C2+C3+C4 G#3 - C2+C3+G#3+C4 - C2+C3+A3 -:5 E3:4 | \
            A2:1 A3:2 D2 - D2+D3+F3+A3:1 D2+D3+F3+Bb3 - D2+D3+F3+Bb3 - D2+D3+F3+Bb3 - D2+D3+F3+B3 - | \
            -:7 F4:1 -:3 D2:2 - D2+D3+A3+D4:1 | \
            D2+D3+Bb3+D4 - D2+D3+Bb3+D4 - D2+D3+Bb3+D4 - G2+F3+B3 -:4 F3:2 -:1 G2 G4 | \
            C2 - B3:2 - B3:1 -:2 A3:1 B3:2 -:4 | \
            F3:3 - C2:2 C3+E4+G4:1 - G3+E4+G4:2 C4+E4+G4:1 E4+G4 E3+E4+G4 - | \
            F#4:2 D#4+F#4:1 - D#4:6 -:1 D2:2 C4 -:1 & -:3 G3+B3:2 -:11 | -:12 G3:3 -:1 | \
            -:4 D3+F3+A3:1 -:11 | - D3:1 D3+A3+D4 -:3 | -:11 B3:1 -:4 | -:11 G3:2 -:3 | \
            G2:2 -:4 E4:1 -:9 | D#2+D#4:1 -:3 D#3 -:1 F#4 D#2 -:2 D3:4 & -:16 | -:12 A2:2 - | \
            -:16 | - | - | - | -:1 B3:2 -:13 | -:4 F#4:2 F#3+C4+F#4:1 -:9 & -:16 | \
            -:12 A3:1 C#4+E4 -:2 | -:16 | - | - | - | -:1 Eb4 -:14 | -:16")
        .with_tempo(&[tempo(0, 89), tempo(16, 95), tempo(32, 88), tempo(48, 94), tempo(64, 90), tempo(80, 92), tempo(96, 84), tempo(112, 85)]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "-:1 G4:5 -:1 D4:2 E4:1 - F4 G4 A4 B4 - | \
            B4 A4 A4 - A4 B4 - A4 B4 A4 F4 B4 G4:2 -:1 G4 | \
            - G4 A4 G4 A4 - B4 G4 E4 -:2 D4:1 C4 -:2 C4:1 | \
            -:4 E4:2 F4:1 E4 - F4 - F4 E4 D4 C4 E4 | _:10 C4:1 E4:2 F4:1 G4 - | E4 A4 -:4 D4:10 | \
            _:2 - D4 E4 F4:1 G4 A4 B4 C5:4 | _:1 -:3 C5:1 - C5 C5 F#4 C5:3 C4:1 A4 C5 -",
            "G7 | Dm G7 | G7 C | C7 G7 | C Am | A7 Dm/A Dm Dm | G7 G7 G7 F | F B7",
        )
        .with_left("B3:6 -:5 D4:2 F4:1 G4 - | D2+F4:2 D3+F4:1 - F3:3 F4:1 G4 D2 -:2 G2:4 | \
            _:2 F3:3 G2:1 -:2 C2 -:3 B3:1 -:2 | G2+F3+A3:1 A#3+B3 - C2+G3 -:4 G2 -:3 C2:1 | \
            _ -:2 C2+C3+G3+C4:1 C2+C3+G#3+C4 C2+C3+C4 G#3 - C2+C3+G#3+C4 - C2+C3+A3 -:4 A2+E3:1 | \
            _:2 - A3 D2 D3+F3+A3:1 -:2 D2+D3+F3+A3:1 D2+D3+F3+Bb3 D2+D3+Bb3 F3 - | \
            D2+D3+F3+Bb3 - D2+D3+F3+B3 -:9 A4:4 | \
            _:1 - G3 A3:2 F2:1 A4 A4 D#4:3 F#3 -:2 & F3:3 D3:1 -:8 E4:1 -:3 | \
            G4+A4:1 -:3 F4:1 G4 A3 -:7 B3:2 | _:4 -:5 B3:1 -:6 | -:8 F3:3 E4:1 -:4 | \
            C3+G3+C4:1 -:15 | G3:3 - F3:1 A3 -:8 | -:13 F2:1 - C3 | \
            F3:2 -:1 G3 A4 -:3 F#2:1 A4:3 -:1 D#4:2 -:1 & G2:1 -:15 | -:14 F3:2 | _:1 -:15 | \
            -:9 B3:2 -:5 | -:16 | A3+C#4:1 -:15 | -:16 | -:10 D#3:2 -:1 F#4 -:2 & -:16 | \
            -:15 Eb4:1 | _ -:15 | -:16 | - | - | - | -:11 A3:2 -:3")
        .with_tempo(&[tempo(0, 88), tempo(16, 96), tempo(32, 93), tempo(48, 94), tempo(64, 92), tempo(96, 97), tempo(112, 87)]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "D5:1 C5 E4 C5:4 C4:1 G4 C5 -:6 | E4:1 F4 G4 - A4 - F4 A4 E4 D4 F4:4 -:1 F4 | \
            -:3 C4:1 E4:3 -:2 G4:1 - G4 E4 Eb4 D4 C4 | _:2 - C4:1 G4 C5 A5 C6 - G5 - G5 - G5 G5 | \
            G5 - G5 F#5 G5 - G#5 - Ab5 - Ab5 Ab5 Ab5 Ab5 - G5 | \
            G#5 C5 A5 - A5 - A5 A5 A5 - A5 G#5 A5 Bb5 -:2 | -:3 A5:1 -:4 F5:1 A5 A5 - A5 A5 - A5 | \
            A5 G#5 D5 A5 Bb5 - Bb5 - Bb5 Bb5 Bb5 - Bb5 - A5 Bb5",
            "C C/E Am7 Am7 | A7 Dm | G7 | Am7/C C | C Fm7 | Am | A7 Dm | Dm/A Bb Bb7 Bb7",
        )
        .with_left("F#2:1 -:2 G2:1 E3:2 - E4:1 G4:5 A2:2 | _:6 -:1 F4 - D2:5 A2:2 | \
            F3+A3 B3 G2:6 -:1 E4 G2 - C#4 C2 | \
            _:4 -:1 E4 A4 G2:2 C2:1 G4+E5 - G4+E5 E3:2 G3+G4+E5:1 | \
            G4+E5 - G2:3 C2:1 C3+G#4+C5 - Ab4+C5 - Ab3:2 Ab4+C5:1 Ab4+C5 G2 G4+Eb5 | \
            G#4 A4:2 -:1 A4+C5 - E3:2 A3+C4+A4+C5:1 - E3+A4 G#4 A4 E2:3 | \
            E3:1 -:2 A2:3 -:2 A4 A4+D5+F5:1 - F3+A4+D5+F5 A4+D5+F5 A3 A4+D5+F5 | \
            A2:3 D2:2 -:1 Bb4+D5 - Bb3:2 F3+Bb3+D4+Bb4+D5:1 Ab3 Bb4+D5 Bb2:2 Bb4+D5:1 & -:5 G3:2 -:3 C3:2 B2:1 Bb2 -:2 | \
            -:3 E3+G3 -:5 D3:3 -:2 | -:1 D2 -:2 B3:1 F3:5 -:6 | \
            C3:3 A3:1 -:3 E5+G5:1 -:6 G4+E5:1 - | -:2 G4:1 F#4 G4 -:5 Ab4+C5:1 E3:2 -:3 | \
            -:1 C2 -:4 A4+C5:1 A4+C5 -:5 Bb4+D5:1 -:2 | -:3 A4+C#5:1 - G3 -:2 D2+D5:1 -:7 | \
            A4+D5:1 G#4+D5 - A4 Bb4+D5 -:3 Bb4+D5:1 Bb4+D5 -:4 A4+D5:1 - & -:16 | \
            -:3 A3:1 C#4+E4 -:7 F3:2 - | -:7 G3:3 -:6 | -:2 E3+G3:1 -:13 | -:11 C4+Ab4+C5:1 -:4 | \
            -:16 | - | - & -:16 | -:13 A3:1 -:2 | -:8 B3:2 -:6 | -:16 | - | - | - | -")
        .with_tempo(&[tempo(0, 96), tempo(16, 84), tempo(32, 89), tempo(48, 85), tempo(64, 93), tempo(80, 95), tempo(96, 83), tempo(112, 98)]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "B5:1 - B5 - B5 B5 D4 B5 B5 A#5 B5 C5 C6:4 | \
            -:1 B4 B5:3 -:1 D6 - C6 - D6 C6 D6 - C6 C6 | \
            B5 - A5 B5 A5 B5 - A5 B5 A5 - E5:2 - E4:1 | C#4:3 -:2 A5 G5:1 - A5 G5 A5 G5 A5 - G5 | \
            D5 -:3 C4:1 D4:3 -:2 G5 F5:1 D4 G5 F5 | G5 - F5 E5 F5 G5 -:3 E4:1 G4 -:2 F4:1 D5:2 | \
            C5:1 D5 C5 D5 - D5 C5 C5 B4 C5 - A4 B4 A4 B4 - | \
            B4 A4 A4 C5:2 G4:1 B4:2 C5:1 B4 C5 B4 C5 - A4 B4",
            "G G G Dm7 | G7 C/E | B7 B7 Am C7/E | A7 Dm/F | G7 | C C C Dm7 | Dm7/C B7 | \
            B7 Em Am7/E Am7/E",
        )
        .with_left("D2+B4+D5:1 - B4+D5 - B3:2 -:1 B4+D5 B4+D5 B2+Bb4 B4 D2:2 D3:1 - F3 | \
            _ G2:3 F3:1 - D5 C2+C3 C5 E3:5 G3+C5+B5:1 C5 | \
            D#2+B4 D#3:6 F#3+A4:1 B4 A4 - E2:3 G3+Bb3:1 D4 | \
            A2:3 G3+A3:1 A2 A4 D2 D3+G4 F3:6 A3:1 G4 | G2:4 A2:1 B2:3 G2:1 - G4:2 F4:1 A3 G4 F4 | \
            G2:3 E4:1 F4 C2:4 -:1 G2 - C2 - A4:2 | _ -:1 A3 - C5 F2+B4 - F#2 B4:2 F#3:3 -:2 | \
            F#3+A4:1 G#4 - G2 E3:8 G3+B4:1 -:2 A2:1 & -:4 B4+D5:1 B4+D5 -:6 F5:4 | \
            _:1 - D5:3 -:5 D5:1 C5 C4+D5 -:3 | -:2 F#3+A4:1 B4 C4:2 -:5 E4:2 E3:1 -:2 | \
            -:1 E3:2 -:6 A4:1 A3:2 D4+G4:1 -:3 | D4:1 - B3:2 -:1 Bb2+C#4 -:4 D2:1 -:5 | \
            G4:1 - F3+F4 -:2 G4:1 C3:2 B3:1 -:5 F2:1 - | C3:5 - D#3:6 | \
            _:1 -:5 G3:4 -:6 & -:12 A5:4 | A3:1 -:15 | -:4 A4:1 B4 -:6 Bb2:1 -:3 | \
            -:10 G4:1 A4 -:4 | -:2 F3:1 -:13 | -:7 G3:2 -:7 | -:13 C4:2 -:1 | -:9 C4:2 -:5")
        .with_tempo(&[tempo(0, 90), tempo(16, 78), tempo(32, 86), tempo(48, 84), tempo(64, 91), tempo(80, 82), tempo(96, 86), tempo(112, 95)]),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "A4:1 -:2 E4:1 A4 A5 -:2 E4:1 - G4 F4 E4 F4 A4 C5 | \
            A4 G4 A4 F4 E4:2 -:1 D4 E4 - C4 -:2 C4:1 - G4 | _:12 D4:3 -:1 | \
            E4:2 F4:1 E4 - F4 - F4 E4 - D4 C4 E4:4 | _:8 C4:1 E4:2 F4:1 G4 C#4 A4 - | \
            -:3 D4:12 -:1 | F4:3 G4:1 F4 G4 - G4 F4 E4 F4:6 | \
            _:5 D4:1 - F4 G4 A4 - B4 -:2 B4:1 -",
            "A7 Dm | Dm Bb7 Bb7 Bb7 | G Am7 A7 G/B | Em C | C Am7 | Dm Bb | G7 Dm | Bb G7",
        )
        .with_left("-:1 E3:2 C#4:1 -:3 A2:1 - D2:2 D3+D4:1 C#4 D4 F3 - | \
            F4 D2 G4 - Bb2:6 -:1 Ab3 Bb2:2 - | - B3:4 -:2 Bb2+C#4:4 B2:3 -:1 | \
            -:4 G2 - C2+C3:1 C2 C3+G3+C4 - | \
            C2+C3+G3+C4 - C2+C3+G#3+C4 - C2+C3+G#3+C4 -:2 C2+C3+G#3+C4:1 C2+C3+A3 -:4 E3:3 | \
            A2:1 A3 - D2+D3 D2+D3+F3+A3 -:2 D2+D3+F3+A3:1 D2+D3+F3+Bb3 - D2+D3+F3+Bb3 - D2+D3+F3+Bb3 - D2+D3+F3+B3 - | \
            -:7 F4:1 -:2 D4:1 D2+D3 D2+D3+A3+D4 -:2 D2+D3+A3+D4:1 | \
            D2+D3+Bb3+D4 D3+Bb3+D4 D2 - D2+D3+Bb3+D4 D2+D3+B3 -:4 F3:3 -:1 G4:2 & -:2 G3:3 -:5 E4:1 -:5 | \
            - F3 -:3 C#3:1 -:2 | - G2:3 A2+C4:4 -:7 | -:4 G3:2 -:1 E4 -:8 | -:13 A2+G3:2 -:1 | \
            -:16 | - | -:10 G2:2 - G4:1 - & -:2 A3:1 -:13 | -:7 Ab3:3 -:6 | -:16 | -:5 B3:1 -:10 | \
            -:13 A3:1 E4 - | -:16 | - | -:10 B3:1 F4 -:4 & -:16 | -:8 D4:1 -:7 | -:16 | - | - | \
            - | - | -")
        .with_tempo(&[tempo(0, 85), tempo(32, 88), tempo(48, 84), tempo(64, 87), tempo(80, 90), tempo(96, 94), tempo(112, 87)]),
        // Bars 57-64.
        Part::new(
            "Part 8",
            "E4:1 -:2 D4:1 C4 -:2 C4:1 -:4 E4:2 F4 | \
            G4:1 A4 B4 C5 G4 B4 B4 G4 B4 C5 B4 - C5 E4 C5 - | \
            B4:2 C5 B4:1 C5 B4:2 C5:1 B4 C5 A4:4 C4:1 | - G4:6 -:1 D4:2 E4:1 - F4:2 G4:1 A4 | \
            B4 A4 B4 - A4 - A4 B4 A4 - B4 A4 B4 G4:2 -:1 | \
            G4 G4 A4 G4 A4 B4 G4 B4 E4 -:2 D4:1 C4:2 -:1 C4 | \
            -:4 E4:2 F4:1 E4 - F4 E4 F4 D4 C4 E4:2 | _:8 C4:1 E4:2 F4:1 G4 C#4 A4 -",
            "C G7 | Am7 C Em Em | B B B7 Dm7 | G7 G7 E7 Dm | G7 G7 Dm G7 | G7 G7 C G7 | F F G7 C | \
            C Am7",
        )
        .with_left("C2:2 B3 - B3 G2+F3+A3:1 - B3 C2+G3 -:4 | \
            - E4:2 -:1 E3:3 G3+E4+G4:1 - E4+G4 G2 G4+B4 - | \
            F#4:2 D#4+F#4:1 - D#3:2 F#3+C4+D#4+F#4:1 - D#4:2 -:1 D2:2 A3 -:1 | \
            B3:7 G1:1 -:4 D4:2 E4:1 F4 | G4 - D2+F4:2 F4:1 - F3:2 A3+F4+G4:1 - G4 D2+F4 F4 G2:3 | \
            _:2 -:1 F3:2 G2+Bb2:1 - G4 C2:2 B3:1 -:2 G2+F3:1 B3:2 | \
            F3+A3:1 - B3 C2+G3 -:4 G2+F3:2 -:1 E4 -:2 C2 | \
            -:1 C2+C3+G3+C4 - C2+C3+G#3+C4 C2+C3+G#3+C4 - C3 C2+G#3+C4 C2+C3+A3 -:4 E3+G3:2 -:1 & -:16 | \
            -:4 C2+C3:1 -:2 E4:1 E4+G4:2 -:6 | \
            D#2+D#4:1 -:3 D#4+F#4:1 D#4+F#4 -:2 F#4:1 D#2 -:2 D3:4 | _:1 -:3 D2:1 -:11 | \
            -:3 D3:1 -:2 F4:1 G4 -:7 B3:1 | _:3 -:13 | -:8 B3:1 -:5 C3:1 C3+G3+C4 | \
            -:13 A2+A3:1 E4 - & -:16 | - | - | F3:2 -:14 | -:15 F3:1 | _:2 -:14 | -:16 | \
            - & -:16 | - | - | G2:1 -:15 | -:16 | Eb4+F#4:1 -:15 | -:16 | -")
        .with_tempo(&[tempo(32, 89), tempo(48, 94), tempo(64, 93), tempo(80, 89), tempo(112, 87)]),
        // Bars 65-71.
        Part::new(
            "Part 9",
            "-:3 D4:12 -:1 | D4:2 E4 F4:1 G4 A4 B4 C5:7 C4:1 | \
            C5 - C5 C5 D#4 C5:2 -:1 D#4 A4 C5 D5 - D5 C5:2 | \
            _ C4:1 G4 C5 -:4 C#4:1 E4 F4 G4 C#4 A4 F4 | A4 E4 D4 F4:2 - F4:1 F4 -:4 C4:1 E4:2 | \
            _:1 - G4 - G4 E4 Eb4 - D4 C#4 -:2 G4:1 -:3 | C5:1 -:15",
            "Dm Bb | Em7 F | F B7 B7/F# B7/F# | C7 A7 | Dm Dm Dm Em | G7 G7 Dm7 G | C",
        )
        .with_left("A2:1 A3 - D2+D3+F3+A3 D2+D3+F3+A3 - D2 D3+F3+A3 D2+D3+F3+Bb3 Bb3 D2+D3+F3 - D2+D3+F3+Bb3 - D2+D3+F3+B3 - | \
            -:9 F2:2 -:1 F3 G3 A3 - | - F2 -:2 F#2+C4:1 F#4 D#3 F#3:6 C5:1 - G2 | \
            E3:2 -:1 E4 G4 B2 - Bb2 A2+E3:7 -:1 | A2+F4 -:2 D2:6 -:2 D2+A3:1 B3 - G2:2 | \
            _:5 -:2 G2:1 - C2+C4 -:2 G2+G3+B3:1 -:3 | C2+C3+C4+E4:1 -:9 C2:6 & -:16 | \
            -:10 C3:2 -:4 | -:7 A3:4 -:1 D#3 -:3 | -:1 G3:2 -:5 G3:4 -:1 G3:2 -:1 | \
            -:4 A2:6 -:5 F3:1 | _:6 -:10 | -:16 & -:16 | - | -:8 C4:3 -:5 | \
            -:8 C#3:1 A3 -:3 A3:1 -:2 | -:5 D3:4 -:7 | -:1 G3:3 E4:1 -:11 | -:16 & -:16 | - | \
            -:9 F#4:1 -:6 | -:16 | -:5 F3:1 A3 D4 -:8 | -:16 | -")
        .with_tempo(&[tempo(0, 89), tempo(16, 94), tempo(48, 81), tempo(64, 90), tempo(80, 94), tempo(96, 88)]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8", "Part 9"],
    drums_from: 0,
    about: "Discépolo's words: someone who set out full of hope finds that every love betrays, \
            and wishes he still had the heart he gave away, to love again. Mores, a pianist, \
            wrote the tune; over the first chords it climbs by half steps. Here in C major, \
            with turns to A minor and D minor.",
};

// Copyright: Troilo died in 1975; not public domain; added at the owner's request.
// Written out from a piano performance (MIDI in 2/4, here two bars to a bar of 4/4).
pub(super) const SUR: Song = Song {
    name: "Sur",
    origin: "Aníbal Troilo, words by Homero Manzi, 1948",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 107.4,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-11.
        Part::new(
            "Verse a",
            "-:7 F#4:1 F#4:2 -:1 F#4 -:2 F#4:1 F#4 | F#4:5 -:1 F#4:4 - G4:1 G#4 | \
            A4:4 E4:1 - A4:2 A5:3 - A4:1 C5 | Bb4:3 A4:2 -:1 G4 F4 E4:3 D4:2 E4:1 F4 - | \
            E4:10 -:4 E4:1 F#4 | G4 G5:5 F5:1 - F5:2 E5:1 - E5 D5 D5 C#5 | E5 Eb5 D5:12 E5:1 E5 | \
            _:3 - E5:1 - E5:2 E5 G5:1 F5 E5 - | -:6 D4:2 C#4:3 -:4 A4:1 | \
            A4:2 F#4:1 - A4 F#4 - G4:3 E4:2 G4 E4 | A4:12 -:1 A4:3",
            "D | D | A7 | Gm Bb7 | A7 A | Gm A7 | Dm | E7 Bb7 | A7 | D7 D7 C7/E A7 | D D7",
        )
        .in_key(D, Major)
        .with_left(
            "-:16 | C#4:5 -:1 D3+A3:5 - | A2:13 E3:1 - A#4 | D3:15 -:1 | C#4:5 -:3 A1:4 -:3 F4:1 | \
            -:2 Bb2:3 -:1 Bb2:3 -:1 E3:4 C#3:1 - | \
            D3+F3+A3+B3:3 -:1 F3+A3+B3+D4 - F3+A3+B3+E4 - F3+A3+B3:3 -:1 F3+A3+B3+D4:2 - | \
            G#3:7 -:1 Bb2+D3+Ab3:5 -:3 | A2:12 -:3 D4:1 | \
            D2+D3+D4:3 -:1 D#2+D#3+C4:2 -:1 E2+E3:4 -:1 A2:4 | D3:13 -:3 & -:16 | \
            -:1 D3+A3:5 C#4:4 -:6 | G3:12 -:4 | G2:6 -:2 Bb2:8 | A2+G3:2 - E2:4 -:8 | \
            -:4 G3 A2 G3:2 - | D#5:1 -:7 E4:2 -:6 | E3+D4 -:10 | G3:11 C#3:1 -:4 | \
            -:7 A#3:3 -:2 G3+C#4:3 -:1 | -:2 D2:11 -:3 & -:16 | - | C#4:4 -:12 | -:8 Ab3:6 -:2 | \
            -:4 D#2:1 -:11 | -:16 | - | -:2 B3:3 -:11 | -:5 A#3+B3:1 -:4 E3:2 -:4 | -:16 | \
            -:4 F#4:3 C#4:1 C4:2 -:6 & -:16 | - | - | - | - | - | - | - | - | - | -:6 D4:1 -:9",
        )
        .with_tempo(&[
            tempo(16, 96),
            tempo(32, 88),
            tempo(48, 92),
            tempo(64, 96),
            tempo(80, 84),
            tempo(128, 100),
            tempo(144, 111),
            tempo(160, 107),
        ]),
        // Bars 12-16.
        Part::new(
            "Verse b",
            "Bb4:2 - Bb4:1 A4 - Bb4 D5 C5 - C5 B4 C5:2 -:1 | \
            -:2 Bb5:1 B5 C6:4 C7:1 -:3 B4:1 C5 C#5:2 | _:5 G#4:1 A4:2 G4:1 - G4 F#4 G4:4 | \
            F4:12 -:1 D4 E4:2 | F4:8 E4:2 C#4:4 F4:1 E4",
            "Gm C7 | F A | A7 A7/E | Dm | Bb7 A7",
        )
        .with_left(
            "G2:4 - E3:7 Gb3:1 | F3:3 - C5:1 -:9 | C#3+G3:6 -:2 G3:6 D3+F3+A3+B3:2 | \
            _ -:3 G#2:2 A2 -:7 | Ab3:4 F3:2 -:1 D4 G3:2 A1:6 & -:2 D3:1 -:5 C3:6 -:2 | -:16 | \
            A2:5 -:3 Bb3+C#4:5 -:3 | -:8 D3 | \
            _:1 - Bb2:5 - A2:2 -:1 Eb2 & -:8 Bb3:4 -:1 F4:2 -:1 | -:16 | -:4 A4:1 -:3 E3:4 - | \
            -:8 D2:6 -:2 | D4 -:14 & -:8 E4:2 -:1 E4 -:4 | -:16 | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 110),
            tempo(16, 94),
            tempo(32, 104),
            tempo(48, 119),
            tempo(64, 82),
        ]),
        // Bars 17-24.
        Part::new(
            "Refrain a",
            "D4:3 - F4:1 -:3 C4:1 -:5 | G4:12 E4:1 D4 C#4:2 | \
            -:4 E4:1 F4 F#4 G4 G5:3 -:1 F5 E5 G5:2 | -:4 G4:1 -:3 E5:1 -:3 E6:1 -:3 | \
            -:2 A4:4 - C4:1 -:5 | D4:2 - F4:1 G4 A4:2 A5 G5:1 A5 G5 A5:3 | \
            C#4 Bb4:1 Bb4:2 -:1 A4:4 - D5:1 | D5:12 A4:1 Bb4 C5:2",
            "D7 Dm C7 C7 | A7 C7/E | A7 C7 | A7 A7 A7/E A7 | Dm C7/E | Dm Dm A7 F | Gm D7 | Gm C7",
        )
        .with_left(
            "D2:2 C2:1 C#2 D2:4 C2 Bb1:3 C#2:1 | A1:5 -:1 C4 B3:2 -:1 F#2:5 A2:1 | \
            _ - A1 -:7 E3+Bb3:1 -:3 G4:2 | A2:5 -:3 E2:6 C2:1 C#2 | D2:4 -:2 D2:1 - E2:7 -:1 | \
            -:2 D2:1 -:3 G#4:1 -:5 F5:1 -:3 | Eb3+G3:8 - | \
            -:2 G2:4 Bb4:2 G4:5 -:3 & -:6 A3+D4:1 -:3 G3:1 -:5 | -:2 A3 -:4 Bb3:3 -:2 G2 -:1 | \
            -:16 | -:4 G2 G4+C#5:1 -:3 A1 -:1 | -:2 D4+F4:4 - Bb3:1 - G#3 - A3 - | -:16 | \
            -:3 G4:1 G4 -:2 D3+C4+Eb4+F#4:4 -:5 | -:7 A4:1 -:2 E3+Bb3 -:4 & -:16 | \
            -:10 E2:4 G#2:1 - | -:16 | -:4 C#4+E4:1 -:7 G5+C#6:1 -:3 | -:16 | - | \
            -:7 Eb4+F#4:1 -:8 | -:16",
        )
        .with_tempo(&[
            tempo(0, 95),
            tempo(16, 100),
            tempo(32, 105),
            tempo(48, 101),
            tempo(64, 102),
            tempo(80, 105),
            tempo(112, 101),
        ]),
        // Bars 25-35.
        Part::new(
            "Refrain b",
            "-:13 G4:1 A4 - | Bb4:2 A4:3 G4:1 F4:2 E4:3 -:1 E4 D4 C#4:2 | \
            _:1 - G4 G#4 A4 - Bb4 -:3 A4:1 -:4 G5:1 | \
            D5 -:2 F#5:1 - Gb5 - F5 G4 F5 E5 D5 C#5 E5:3 | _ D5:2 F5:1 F5 F5:6 E5:1 E5 D5 | \
            D5 C#5 C#5:4 E5:1 - E5:2 D5:1 - D5 C#5 E5:2 | _ - C5:1 C#5 D5 Eb5 D5 - C4 -:4 G4:1 | \
            G4:2 -:3 F4:1 F4 E4 E4:2 D4:1 - D4 C#4 E4:2 | _ D4:1 -:3 F4 -:1 E4:4 D4:1 - | \
            E4:2 C#4:1 -:3 D4:1 D#4 E4 C#4 F4:5 E4:1 | D4 -:3 A6:1 -:11",
            "F7 | Bb Bb7 | A7 D7 D7 D7 | Gm7 A7 | Dm E7 | A7 | D7 D7 D7 A7 | C7 A7 | \
            Dm7 Dm7 E7/B Bb7 | A7 A | Dm",
        )
        .with_left(
            "F2:13 -:3 | Bb2+F3:11 -:1 D4 -:3 | \
            A2+G3:2 -:4 Bb3+C4+F#4:1 -:3 C4+Eb4+F#4:1 - D2+E2:2 F2:1 G2 | \
            _:2 -:6 A3+C#4+E4:1 -:4 Eb5:1 -:2 | F3+A3+B3:1 -:9 E3:4 -:2 | \
            A2:10 A2:2 G4:1 - F#4+C5:2 | _ A2:1 -:7 F#3:1 - A3 -:3 | \
            -:2 E3+Bb3:1 - G3 -:5 C#3:3 -:2 A3:1 | _:2 - C2:3 -:1 B1:3 -:1 Bb1:4 | A2:15 -:1 | \
            D2 -:3 A1:4 D2:1 -:7 & -:2 A2:11 -:3 | D4 -:7 Ab3:4 -:2 | -:12 F2+Gb2:1 -:2 F#2:1 | \
            Bb4 -:15 | -:11 G#3:2 D4:1 -:2 | - E3:10 -:4 | D2:2 -:14 | -:10 G3:2 -:4 | \
            D2:3 -:1 C3:2 D4:4 G#3 Ab3:1 Eb2 | A1:5 -:3 A1:7 -:1 | \
            -:4 A5+C#6+F6:1 -:11 & -:4 C3:9 -:3 | -:16 | - | - | - | -:4 G3:8 -:4 | -:16 | - | \
            D3:2 -:4 A3:2 B2 - Bb2:3 -:1 | G3+D4:2 - Bb1:5 -:3 C#2:4 | -:16 & -:6 Eb3:7 -:3 | \
            -:16 | - | - | - | -:8 G4+Bb4:2 -:6 | -:16 | - | - | Bb1:1 -:15 | \
            -:16 & -:6 A4:1 G4:2 - F4:3 -:2 | -:16 | - | - | - | - | - | - | - | - | \
            - & -:8 F4:2 -:6 | -:16 | - | - | - | - | - | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 98),
            tempo(16, 109),
            tempo(32, 99),
            tempo(48, 108),
            tempo(80, 114),
            tempo(96, 104),
            tempo(112, 101),
            tempo(128, 89),
            tempo(144, 93),
            tempo(160, 99),
        ]),
    ],
    form: &["Verse a", "Verse b", "Refrain a", "Refrain b"],
    drums_from: 0,
    about: "Manzi remembers the old south of Buenos Aires, the corner of San Juan and Boedo, \
            Pompeya and the floods beyond it, and a love that ended there; Troilo, the \
            bandoneonist and bandleader, wrote the tune. D minor.",
};

// Copyright: public domain in Argentina, where a work is protected for 70 years after its author's
// death: Gardel and Le Pera both died in 1935.
// Written out from a piano performance (MIDI in 2/4, here two bars to a bar of 4/4).
pub(super) const SOLEDAD: Song = Song {
    name: "Soledad",
    origin: "Carlos Gardel, words by Alfredo Le Pera, 1934",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 83.8,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:7 A4:5 B4:2 C#5:1 D5 | C#5:6 B4:1 C#5 B4:8 | \
            E4:4 A4:1 B4 A4 F#4 A4:2 G4:3 -:1 D#4:2 | F#4 E4 C#4 -:8 A4:1 C#5 | \
            E5:4 -:1 D5 C#5 D5 - G5:5 F#5:2 | A5:5 E5:1 -:2 D#5:1 - E5 - D#5 - E5 - | \
            F#5:6 D#5:1 E5 G5:3 E4:2 F#5:3 | A5:13 C#5:1 F#5 -",
            "D | B7/D# Em | Em | A7 D | D/F# B7 | A7/E A7/E B7 Em7 | Em7 C7/G | Cdim7 Em7",
        )
        .with_left(
            "-:7 F#4:1 F#3:8 | _:7 -:1 E3:8 | _:7 -:1 E3:8 | A2 D2+F#3+A3:7 B4:1 | \
            F#2:4 - D#3:6 -:2 | E3+G3:7 -:1 B2+F#4+A4 - G4 - E3:4 | _:7 -:1 Bb3:5 -:3 | \
            F#3 C#4:1 Eb5:3 -:1 E3+B4+D5:5 E3+G3+A#3+C#4:2 -:1 & -:8 D3+A3 | D#3 -:3 G3:5 | \
            _:6 D#3+A3:2 -:8 | G3:4 -:1 B3 -:10 | -:2 D3:4 -:3 B2+A3:5 -:2 | A3+C#5:6 - D4:4 | \
            _ D4 G3 - | -:1 A4:5 C4:2 B3:4 - & -:16 | -:3 A4 -:7 B3:3 | _:6 G4:1 -:9 | \
            C#4:2 -:14 | -:16 | -:12 F#4:1 - G4 - | - G3:5 -:10 | -:4 F3:2 -:3 D4:1 -:6 & -:16 | \
            - | - | - | - | - | -:3 B3 -:10 | -:16 & -:16 | - | - | - | - | - | -:5 E5:1 -:10 | \
            -:16",
        )
        .with_tempo(&[
            tempo(32, 74),
            tempo(48, 59),
            tempo(64, 72),
            tempo(96, 91),
            tempo(112, 57),
        ]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "A5:1 - B5:2 C#6 D6:3 -:1 A5:2 B5 C#6 | B5:6 A5:2 B5 A5 G5 F#5 | \
            A5:7 -:1 F#5:2 E5 D#5 E5 | A5 C4:3 - D#5:6 A4:1 A4 | A4:4 G4:6 D4:1 A4:2 G4:1 G4:2 | \
            _:5 -:1 D#4:4 -:2 F#4:1 - G4 F#4 | F#4:8 D#4:2 E4 E4 G4:1 F#4 | \
            D4:5 -:3 A5:2 B5 C#6 D6:1 B5",
            "D B7/D# | Em B7 | A7/E A7 | D7/F# B7 | Em7/G C7 | D7 B7 | E7/G# A7 | A D D D",
        )
        .with_left(
            "A3:13 -:3 | E3+G3:7 F#3:4 -:5 | G3:7 -:1 A3+C#4+G4:3 -:5 | F#3 -:1 A3 - B2:7 -:1 | \
            G2:7 -:1 C2:5 -:1 D2:2 | _:3 -:1 C3:2 -:1 B2:6 -:1 F#3 - | G#3:8 -:2 A2+C#3:6 | \
            _:1 -:3 D3:6 G2:2 D3:4 & F#3:7 C#6:4 -:5 | -:8 B2+D#3:3 -:5 | E3 D#5:2 -:9 | \
            -:10 F#3:4 -:2 | - D3:5 -:1 Bb3:2 -:6 | D3:5 -:4 D#3:5 -:2 | D4:7 E4:1 -:2 G3:5 E3:1 | \
            _ -:3 D2:5 -:7 & D3:3 -:5 D#3:8 | _:2 -:14 | -:3 G5 -:10 | -:12 A3:1 A4 -:2 | \
            -:6 A3:2 -:8 | -:2 F#4:4 -:5 A3:3 -:2 | - B3:5 -:9 | -:6 A2:3 -:7 & -:16 | - | - | - | \
            - | - | -:5 E3:3 -:8 | - F#2:1 -:7",
        )
        .with_tempo(&[
            tempo(0, 123),
            tempo(16, 132),
            tempo(32, 145),
            tempo(48, 77),
            tempo(64, 72),
            tempo(80, 91),
            tempo(96, 79),
            tempo(112, 75),
        ]),
        // Bars 17-21.
        Part::new(
            "Part 3",
            "C#6:5 C#4:1 A5 Ab5 G5:4 D4:1 A5 B5 G5 | A5:8 E5:2 F#5 G5:1 B5:2 A5:1 | \
            _:8 F#5:2 - E5:4 | D5:2 G4:1 G#4 A4:3 A5:1 -:3 A6:1 -:4 | \
            D4:2 -:1 E4:2 F4:1 G4:2 A4:3 A#4:2 C#4:1 C5 A#4",
            "Dmaj7 Em | Dmaj7/F# Em7 | D/F# Em7 | D/F# A7 A7 A7 | Em7 A7",
        )
        .with_left(
            "-:1 F#2 - F#3:4 -:1 E3:6 -:2 | F#3:6 F#5:2 D4:8 | _:1 - D4:8 -:2 B4:4 | \
            F#3:3 -:4 Eb3:5 -:4 | D3 - A2+C#3+G3:2 -:1 G4:2 A#3:3 & -:10 B3:5 -:1 | \
            -:2 C#4:4 -:2 E3:7 -:1 | F#3:7 A4:1 E3+G#3+B3:2 - G4:3 -:1 | -:7 G3+A#3+C#4:4 -:5 | \
            D2:3 -:10 E3:2 G4:1 & -:16 | -:5 E4:1 -:2 G3:6 -:2 | -:5 A3:2 -:5 A2:2 - | -:16 | \
            -:14 A4:1 - & -:16 | - | -:5 F#5:1 D5:2 -:5 G3:3 | _:1 -:15 | -:16",
        )
        .with_tempo(&[
            tempo(0, 73),
            tempo(16, 91),
            tempo(32, 69),
            tempo(48, 76),
            tempo(64, 97),
        ]),
        // Bars 22-29.
        Part::new(
            "Part 4",
            "A4:3 G4:1 F4 G4 A4:6 -:4 | G4:3 F4:2 E4 F4:1 G4:5 -:3 | \
            A4:1 F4 D4:3 -:1 G#4 - A4 C5 Bb4:2 - A4:1 - | D5:5 E5:1 F5 G5 A5:5 Bb5:1 C6 Bb5 | \
            A5:2 G5 C6:1 G5 A5:2 -:4 A5:1 -:3 | B4:4 A4:2 G#4:1 A4 B4:4 C5:2 - | \
            A4 -:6 A5:2 -:1 C6:3 C6:1 B5 | C6:2 -:4 Bb4:1 A4 G4:4 F#4:1 G4 Bb4:2",
            "Dm Dm7 | Gm7/Bb A7 | Dm C7/E | Dm Am/C | F | E7 | A7 D7/F# | Gm7 Em7b5",
        )
        .in_key(D, NaturalMinor)
        .with_left(
            "D3+F3+A3:9 -:3 D3:1 C#3:2 Bb2:1 | _:8 A2 | -:1 D3:4 -:1 D3 - E3:6 -:2 | \
            D3:6 -:2 C3:7 -:1 | F2:10 -:2 F2 - | E2:13 -:3 | A2:5 G3:1 -:2 F#3+C4:5 -:1 C5 B4 | \
            G2:4 G3:2 F3 E3:5 -:2 D3:1 & -:14 C3:2 | D4:4 - C#4:5 -:1 G3:2 | \
            -:1 E4 - F3:4 -:3 G3:5 -:1 | -:3 F3:4 -:3 E3:4 -:2 | - C3:8 -:2 C5+F5:1 -:3 | \
            E3:12 Bb2:4 | _:1 -:2 E3:3 -:2 A4:1 -:2 C5:3 -:2 | C5 D3 -:5 G3 -:2 & -:14 B2:1 - | \
            -:10 E3:1 C#3:5 | -:4 A3:2 -:6 Bb3:2 - | -:5 A3:3 -:4 Bb3:1 -:3 | -:4 F5:2 - F3 -:6 | \
            D4:11 -:1 G#4:4 | C#4:2 -:14 | -:11 Bb3:4 -:1 & -:16 | -:12 E3:4 | -:16 | - | - | \
            G#4:3 -:9 D4:2 - | -:16 | -",
        )
        .with_tempo(&[
            tempo(0, 94),
            tempo(32, 83),
            tempo(48, 88),
            tempo(64, 90),
            tempo(80, 84),
            tempo(96, 80),
            tempo(112, 91),
        ]),
        // Bars 30-36.
        Part::new(
            "Part 5",
            "-:4 A4:6 -:3 A4:1 Bb4 A4 | G#4:8 F4:1 E4 F4 D4 C#4 D4 E4:2 | \
            -:4 A5:1 A4 -:2 A6:1 -:7 | D4:2 E4 F4:1 G4 A4:4 Bb4:2 C5:4 | \
            A4:2 -:1 G4:2 F4:1 G4:2 A4 - Bb4:1 -:3 | G4 F4:2 E4:1 F4 G4:2 -:5 C#5:1 - | \
            D5 -:3 A6:1 -:3 D7:1 -:7",
            "F F F/C F/C | E7/B Em7b5 | A A7 | E7/D B7 F#m7b5/A F#m7b5/A | F Em F7/Eb D7 | \
            Em7b5 A7 | E7/D A Dm Dm",
        )
        .in_key(D, NaturalMinor)
        .with_left(
            "-:4 F4:12 | B2:9 Bb2:7 | \
            _:1 A2 -:2 Bb4+C#5+E5:1 -:3 G5+G#5+A5+Bb5+C#6+E6+G#6:1 -:3 A3:2 - | \
            D3 Db3:1 - C3:2 Bb2:1 A2:6 -:1 Bb4:2 | F2:4 E2 Eb2 D2:2 - | Bb3:6 -:2 A2:7 -:1 | \
            D3:2 G2:1 G#2 A2:5 -:7 & -:6 Db3:2 C3:8 | _:4 -:1 E2:4 -:1 E4 -:2 G#3:3 | -:16 | \
            -:5 B2:3 -:4 F#2+E4 | E4:2 -:6 F4:2 - C4+F#4:1 -:3 | E3+D4:5 -:3 G3:4 C#3 | \
            - A5+C#6+E6:1 -:3 D2 -:5 & -:12 C2:3 -:1 | -:6 D4:2 -:8 | -:16 | - | - | \
            -:4 G3:1 -:11 | -:8 D6+A6:1 -:7",
        )
        .with_tempo(&[
            tempo(0, 96),
            tempo(16, 85),
            tempo(32, 74),
            tempo(48, 99),
            tempo(64, 105),
            tempo(80, 77),
            tempo(96, 89),
        ]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5"],
    drums_from: 0,
    about: "Gardel and Le Pera wrote it in 1934: the loneliness of a man who cannot forget the \
            woman who left. Here it starts in D major and turns to D minor.",
};

// Copyright: the tune is public domain in Argentina, where a work is protected for 70 years after
// its author's death: Cobián died in 1953. Cadícamo's words (he died in 1999), which the demo does
// not play, are not.
// Written out from a piano performance (MIDI mostly in 4/4; its two bars of 2/4 are
// joined to their neighbours here).
pub(super) const NOSTALGIAS: Song = Song {
    name: "Nostalgias",
    origin: "Juan Carlos Cobián, words by Enrique Cadícamo, 1936",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 131.9,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:8 A4:4 D4:1 E4:2 F4:1 | A4:3 D4:1 E4:2 F4 A4:5 E4:1 F4:2 | \
            A4 D5:1 Bb4 D5 Bb4 - G4 -:2 G4:1 - G4 F4 G4 F4 | \
            E4:2 D4:1 C#4 Bb4 - A4:5 D4:1 F4 A4:3 | _:5 -:1 G#4:2 A4 Eb4:1 A4 D5 - C5 Bb4 | \
            A4 A4:2 -:1 A4 G4 - Bb4 A4 - Bb4 - A4 - A4 G4 | \
            G4:2 -:1 F4 - G4:2 E4 A4:1 -:2 G4:1 - G4 F4 | E4 -:7 D4:6 -:2",
            "Dm | A/C# Dm7/C | E7/B Gm Gm7/Bb Gm7/Bb | A7 Dm Dm Dm | A E7 D7 D7 | A7/G F | C/E A7 | \
            Dm",
        )
        .with_left("-:9 D3:5 -:2 | C#3:6 -:2 C3:6 -:2 | B2:4 -:1 A4 Bb2:7 -:2 A2+G3:1 | _:4 -:2 D3:6 -:4 | \
            A2:1 -:3 D2:1 -:5 D3+F#3+C4:2 - D2:1 - | -:2 G2:6 G4:1 F2:6 -:1 | \
            E2:5 F4:2 -:1 A2:6 -:2 | D2:6 A2+A3:2 -:4 A2:1 -:3 & -:11 A3:5 | _:1 - A3:6 -:2 A3:6 | \
            -:2 A3:3 - G3 -:5 | -:8 F3:4 - | -:11 F#4:1 -:4 | -:3 G3:4 - F3 -:1 | \
            -:2 E3:5 -:3 G3 - | -:4 F3:5 -:7 & -:16 | -:11 D4:2 -:3 | -:9 Bb3:2 -:5 | \
            -:9 A3:2 -:5 | -:16 | -:4 G4:1 -:11 | -:16 | -:5 Bb3:3 -:8")
        .with_tempo(&[tempo(16, 137), tempo(32, 119), tempo(48, 121), tempo(64, 126), tempo(96, 131), tempo(112, 136)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "-:1 G#4:2 A4:1 C4 A4 D5 - C5 Bb4 - A4 A4:2 G4:1 - | \
            G4 A4 Bb4:2 C5:1 - E5 D5 D5:2 -:1 E4 C5:2 E5:1 D5 | - C5:2 D5:1 Bb4:2 C5 A4 -:6 | \
            C#4:1 E4 A4:4 -:2 A4:4 D4:1 E4:2 F4:1 | \
            C#4 A4:3 D4:1 E4 F4:2 C4:1 - A4:2 D4:1 E4 F4:2 | \
            A4 D5:1 - D5 Bb4:3 G4:1 -:2 D4:1 G4 - G4 F4 | _ E4:2 D4:1 - C#4 E4:2 -:1 D4:7 | \
            _:2 -:5 A4:6 D4:1 - A4",
            "D7 D7 D7 A7 | Gm C7 | C7 C7 F Gm | A7 Dm | A/C# F/C | E7/B Bb Gm/Bb Gm/Bb | A7 Dm | \
            Dm",
        )
        .with_left("D2:1 -:3 D3+F#3:2 -:3 D2:1 -:2 G2:1 G3 -:2 | Bb3:3 - G2:1 -:2 C3 Bb3:1 -:4 | \
            C2 C3:2 - F2:4 -:2 G2 | A2+G3:3 -:2 A2 -:1 D2:4 -:3 C#3:1 | _:8 C3 | \
            B2:3 -:1 Bb4 - A4:2 Bb2:7 -:1 | A2+G3:5 -:3 D2:1 -:3 A2:1 -:3 | \
            D2:1 -:4 A3:5 D3:3 -:1 Eb2 - & -:5 Eb4+F#4:1 -:10 | -:2 D4:1 -:7 G3:2 -:4 | \
            -:15 G#2:1 | -:6 A3:3 -:1 D3:5 -:1 | A3 -:15 | -:10 G3:5 -:1 | -:16 | \
            -:8 D2:4 A3:3 -:1 & -:16 | - | - | - | - | -:11 Bb3:1 -:4 | -:16 | -")
        .with_tempo(&[tempo(0, 113), tempo(16, 121), tempo(32, 138), tempo(48, 133), tempo(64, 130), tempo(80, 133), tempo(96, 134), tempo(112, 130)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "-:4 C#4:1 G4:3 -:1 F4:3 D4:2 E4 | -:1 C#4 F4:2 E4:1 F4 G4 - D4 A4:2 -:1 A4:2 D4:1 E4 | \
            F4 C#4 - E4 F4 G4 - F#4 A4 - D5:2 C#5:1 D5 C4 A4 | _:3 D4 Bb4:2 A4 Ab4:1 - G4:4 | \
            _:6 -:3 D4:1 G4:6 | D4:1 F4 G4 - G4:6 -:2 G4:4 | _ C#4:3 -:1 E4:2 F4 G4 -:1 F#4 | \
            G4 A4 D4 Bb4 -:2 Bb4 G4:1 A4 Bb4:2 A4:1 Bb4 D5 -",
            "C/E Dm/F | A7 Dm | A/C# F D7 D7 | D Gm F Gm | Gm Gm7 | Gm7 C/E C7/E C7/E | \
            Gm A7 A7 A7 | C7 Gm",
        )
        .with_left("E2:6 G3:1 - F2:5 -:3 | A2+G3:4 - D2:6 -:2 | C#2+C#3:3 -:4 C3:3 -:5 F#3:1 | \
            _ F#4 D2 -:3 G2:2 G#2:1 - A2:2 Bb2:1 -:3 | D2:1 -:3 G2:7 Bb3:1 F2:4 | \
            _:1 -:3 E2:9 -:1 G3:2 | _:5 -:1 G3:2 -:8 | \
            -:2 E2 E3:1 -:3 G3:2 Bb3:1 -:2 E2:1 -:2 & -:2 E3:5 -:3 F3:5 -:1 | \
            - A3:2 -:5 A3:1 F4 - D3:3 -:2 | A3 -:5 C2+C4:2 -:7 | -:16 | -:7 G3:3 -:2 D4:1 - F3:2 | \
            _:1 -:5 E3:4 Bb3:2 E3 - | G2 G#2:1 A2:12 -:1 | -:16 & -:3 G3:2 -:6 A3:2 -:3 | -:16 | \
            - | - | -:8 Bb3:2 -:5 Bb3:1 | -:8 G3:4 - | -:1 D4:3 -:12 | -:16")
        .with_tempo(&[tempo(0, 135), tempo(32, 131), tempo(48, 129), tempo(64, 140), tempo(80, 130), tempo(96, 133), tempo(112, 122)]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "C#5:3 E5:1 C#5 E5 Bb4:2 A4:1 -:2 G4:4 -:1 | A4:2 -:1 Ab4 - G4:3 F4:8 | \
            _:1 -:7 D4:1 -:3 A4:1 -:2 G4:1 | D5:5 -:3 A4:1 -:3 F4:2 - | \
            F#4:1 -:2 D5:3 C5:2 Eb5:1 -:2 D5:1 - D5 - C5 | \
            Eb5:2 -:1 Eb5 D5 - D5 C5 Eb5:2 -:1 D5 - D5 C5:2 | \
            Eb5 -:1 D5 - D5 C5:2 D5:1 -:4 A4:1 F#4 - | D5:4 D5:1 -:2 C5 - D5:1 C5:2 Bb4",
            "A7 | Dm | Dm Dm Dm F7 | F7/A A7 | D7 | D7 C | D7 | Gm Gm D7/A C7",
        )
        .with_left("A2:8 G3:3 -:1 A2 E4:2 -:1 | D2:7 -:1 D4:8 | _:1 - D3:6 -:3 A2:1 -:3 Eb4:1 | \
            _:2 - Eb3:4 A3+Eb4+G4:1 -:3 C#4 -:1 | D2+D3+F#3+C4 -:15 | \
            D2:2 D3:1 D5 F#3 -:3 G4:4 G3:1 -:3 | D2+F#4:2 D3:1 - F#3:5 -:3 D2:1 -:3 | \
            Bb4:4 G#2+Bb4:1 - A2:4 A3:1 A4:2 -:1 Bb2+G4:2 & -:2 G3:4 -:10 | \
            F4:2 -:1 E4 -:2 E4:1 -:5 A2:1 -:3 | D2:9 -:7 | A2:6 G3:3 - Eb2+Eb3+G3:2 - | -:16 | \
            -:8 Eb2:2 Eb3:1 -:5 | -:8 C4:1 -:7 | \
            G2:2 G3:1 -:4 A4:2 - C5:1 A2 -:3 & -:5 C#5:1 -:10 | -:16 | -:4 F3:3 -:9 | A4:4 -:12 | \
            -:16 | - | - | -:8 F#3:3 -:5 & -:16 | - | -:6 A3:2 -:8 | -:16 | - | - | - | -")
        .with_tempo(&[tempo(0, 140), tempo(16, 139), tempo(32, 125), tempo(48, 130), tempo(64, 135), tempo(80, 129), tempo(96, 136), tempo(112, 118)]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "-:1 D4 Bb4 D5 D5 D6 - Bb4 D5 Bb4 - A4:2 -:1 D5 - | \
            G#4:2 A4 D5:1 -:3 E4:2 F4 A4 D4:1 E4 | F4 -:6 C#4:2 D4:1 E4 - C#4 F4:2 -:1 | \
            E4:3 -:1 D4:9 -:3 | A4:8 D4:1 E4:2 F4:1 - C#4 A4:2 | \
            D4:1 E4 F4 - C4 A4:3 D4:1 E4 F4:2 A4 D5:1 - | \
            Bb4 D5 A4:2 G4:1 -:2 D4:1 G4 - G4 F4:2 E4 D4:1 | - C#4 Bb4 - A4 -:9 E4:1 -",
            "Gm Gm Gm Dm/A | Dm/A Dm | Dm E A A | A7 Dm Dm Dm | Dm/A Dm/A Dm A/C# | \
            A/C# F/C Dm Dm | Gm7 Gm7 Gm7/Bb A7 | A Dm Dm Dm",
        )
        .with_left("G3:9 G4:2 G#4:1 A2:4 | _:5 -:2 A2:1 -:2 D2:6 | _:1 - A3 D2 B3:3 -:1 E3:3 A3:2 -:3 | \
            G3:1 -:3 F3:9 -:3 | -:2 A3:5 - C#2:4 | _:2 Eb2:1 - C3:4 - F3 | \
            _:1 Bb4 -:2 Bb1+Bb2:7 -:1 A2:4 | _:2 - D2:10 -:2 & -:1 Bb3:8 -:5 F3:2 | _:6 -:10 | \
            D3:2 - G#3 -:6 A2:4 | _:1 -:3 D2:1 -:3 A2:1 -:3 D2:2 - | -:4 D2:5 -:3 C#3:4 | \
            - C2:3 -:5 B1+B2+A3:4 | -:5 G3 -:2 G3:4 | -:6 D3:8 -:2 & -:2 G4:1 -:13 | -:3 A3 -:10 | \
            -:5 E2:6 -:5 | -:16 | -:6 D3:4 -:2 A3 - | -:4 A3:1 F4 -:10 | -:7 Bb3:1 -:8 | \
            - F3:2 A3 B3:1 -:3")
        .with_tempo(&[tempo(0, 120), tempo(16, 121), tempo(64, 134), tempo(80, 126), tempo(112, 136)]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "F4:4 A4:1 -:2 G#4 A4 -:1 G4 D5 - C5 | _ Bb4 A4 - A4:3 D4:1 G4:2 Bb4:1 - A4 Bb4 - A4 | \
            Bb4 A4:2 G4:1 G4:2 -:1 F4 G4 F4 F4 E4 - A4 -:2 | G4:1 - G4 F4 E4 -:7 D4:4 | \
            _:3 - G#4:2 A4 -:1 F#4 D5:2 -:1 C5 | - Bb4 A4 - A4:3 G4:1 - G4 A4 - Bb4:2 C5 | \
            E5:1 - D5 D5:3 -:1 E4 C5:2 E5:1 D5 - C5:2 D5:1 | \
            - Bb4 C5 A4 C5 -:4 C4:1 A4 -:2 C#4:1 -:2",
            "Dm F7 | Gm Gm Gm F | F C/E A7 A7 | A7 E7/D Dm Dm | Dm D7 | Bb A7/G Gm/Bb Gm/Bb | \
            C7 C | F/C F F A7",
        )
        .with_left("A2:3 -:1 D2 -:7 Eb4:2 - | D2:1 -:3 G2:6 -:2 F2:4 | _:2 - E2:4 - A2 | _:2 - D2:9 -:3 | \
            A2:1 -:3 D2:1 -:5 D3:1 Eb4:3 -:2 | D2:1 -:3 G2:2 G3:1 -:2 Bb3:4 -:3 | \
            G2:1 -:3 C3:2 G3 -:4 C2 | C3:2 - F2:5 -:1 G2+F4:2 A2:4 & -:12 A2:1 A4 -:2 | \
            -:6 G3:4 - F3:2 | _ -:4 E3 -:2 G3:4 | _:1 -:7 D3:5 -:3 | -:11 F#3+C4:1 A4:2 - | \
            -:12 D4:1 -:3 | -:7 Bb3:1 -:5 E4:2 -:1 | -:4 A4:1 - F3 - A3:2 -:1 G#2 - G3:3 & -:16 | \
            - | A4:1 -:15 | C#4:1 -:9 F3:6 | _:2 -:14 | -:16 | - | - & -:16 | - | - | -:12 A3:4 | \
            _:2 -:14 | -:16 | - | -")
        .with_tempo(&[tempo(0, 135), tempo(16, 129), tempo(32, 137), tempo(48, 128), tempo(64, 139), tempo(80, 140), tempo(96, 137)]),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "C#4:1 G4:2 - A4:3 D4:2 E4:1 F4 - F4 A4:2 | \
            D4:1 E4:2 F4:1 - F4 A4:2 D4:1 E4:2 F4:1 A4 -:2 D5:1 | \
            Bb4 D5 A4:2 G4:1 -:2 D4:1 G4 - G4 F4 E4:2 -:1 D4 | _ C#4 E4:2 D4:1 -:7 D4:1 -:3 | \
            -:4 A4:5 D4:2 A4:4 -:1 | C#4 G4:3 F4 -:2 D4:1 E4:2 -:1 C#4 F4:2 | \
            E4:1 F4 G4 - D4 A4:2 A4 -:1 D4 E4 F4:2 E4 | F4 G4 A4:3 D5:1 - C#5 D5 C4 D5 A4 F#4 -",
            "A7/E Dm Dm A/C# | A/C# F/C C E7/B | Gm Gm Gm7/Bb A7 | A7 Dm Dm Dm | \
            Dm/A Dm/A Dm A7/E | A7 Dm/F Dm/F C/E | C Dm Dm Dm | Dm D D/F# D/F#",
        )
        .with_left("-:1 E4:4 D2 -:3 C#3:4 | _:3 -:1 C3:8 B2:3 -:1 | - Bb4:2 -:1 Bb2:7 -:1 A2:4 | \
            _:2 - D2:1 -:3 A2+A3:1 -:3 D2:1 -:3 | -:1 A3:4 -:1 D3:4 D#2:1 - E2:4 | \
            _:1 - E2+G3 - F2:6 -:2 E2+E3:4 | _:1 -:3 D2:2 D3:1 -:5 A3:1 -:3 | \
            A2:1 -:2 D2:1 D3 -:2 F#3:9 & -:3 A2:1 A3:8 A3:3 -:1 | -:4 A3:1 C4 -:10 | \
            -:6 G3:5 -:1 G3:4 | _:1 -:15 | -:4 D2 A3:3 - E3:2 | _ -:4 D3 -:3 G3:1 -:2 | \
            -:4 A3:1 F4 -:10 | -:12 A4:1 -:2 D3:1 & -:7 D3:6 C#4:1 -:2 | -:16 | -:7 Bb3:1 -:8 | \
            -:16 | -:15 G3:1 | _ -:6 F3+A3:3 -:6 | -:16 | -")
        .with_tempo(&[tempo(0, 133), tempo(16, 132), tempo(48, 129), tempo(64, 133), tempo(80, 136), tempo(96, 130), tempo(112, 124)]),
        // Bars 57-64.
        Part::new(
            "Part 8",
            "D4:2 Bb4 A4 G#4:1 - G4:8 | _:2 - D4:1 G4:2 D4 F4:3 D4:1 G4 - G4 | \
            _:3 -:1 D4 G4 - G4 G5 G5 E5 D5 Bb4 E4 D4 - | \
            C#4:3 -:2 E4:1 F4:2 G4:1 - F#4 - G4 A4 D4 Bb4 | \
            -:2 Bb4 G4:1 A4 - Bb4 A4 Bb4 D5 C#5:3 E5:1 - | \
            C#5 E5 Bb4:2 A4:1 - G4:2 E4:1 - A4:2 Ab4:1 - G4:2 | F4:3 -:4 A4:9 | \
            _:4 -:2 A4 -:3 F#4:1 D5:2 -",
            "Gm E/G# Gm Gm | Gm Gm7/F | C7/E C7 | A7 | C7 A7 | A7 Dm | Dm | Dm D",
        )
        .with_left("-:2 G2 G#2 A2 Bb2:1 -:3 D2:1 -:3 | G2:6 Bb3:2 F2:6 -:2 | E2:9 -:3 E2+G4:1 -:2 Bb3:1 | \
            A2:9 -:2 A2:1 -:2 E2 | E3:1 -:3 G3 -:2 E2:1 -:2 A2:4 | _:5 -:2 A2:1 -:2 D2:5 -:1 | \
            D4:3 -:1 D3 - A3:10 | _:4 A2:1 -:3 D2:2 D3:1 - A4:3 -:1 & -:16 | -:3 G3 -:4 F3 -:2 | \
            - E3:4 -:2 E3:3 -:5 | G3:1 - E3:9 -:5 | -:6 Bb3:1 -:8 G3:1 | \
            _ C#5:2 -:1 G3:2 -:4 F4:1 - E4 - Eb4:2 | A2:1 -:6 F4:9 | _:5 -:11 & -:16 | \
            -:3 Bb3:1 -:7 Bb3:1 -:4 | -:3 G3+Bb3:1 - E4 -:2 E5:1 -:7 | -:3 G3 -:2 G3:3 -:5 | \
            -:16 | - | -:8 D4 | _:4 -:12 & -:16 | - | - | -:3 A3:1 -:12 | -:16 | - | \
            -:8 D2:4 -:2 E2 | _:1 -:15 & -:16 | - | - | - | - | - | -:11 Eb2:3 -:2 | F2:1 -:15")
        .with_tempo(&[tempo(0, 128), tempo(16, 137), tempo(32, 139), tempo(48, 123), tempo(64, 122), tempo(80, 118), tempo(96, 140), tempo(112, 134)]),
        // Bars 65-74.
        Part::new(
            "Part 9",
            "D5:6 G4:1 - F#4:2 -:1 D5:3 C5:1 - | \
            Eb5:2 -:1 Eb5 D5 - D5 C5 Eb5:2 -:1 D5 Eb5 D5 D5 C5 | \
            - Eb5:2 D5:1 - D5 C5 - Eb5:2 -:1 D5 - D5 C#5 - | D5 -:4 A4:2 -:1 D5:3 -:1 D5:2 - | \
            -:3 C#5:1 - C5:3 Bb4:1 C5 -:2 D4:1 Bb4 D5 D5 | \
            D6 -:2 D5:1 Bb4 G4 G#4 A4:3 D5:2 G#4 A4:1 D5 | -:4 E4:2 F4 A4 D4 E4:1 F4 -:2 | \
            -:6 C#4:2 D4:1 E4:2 -:1 C#4 F4:3 | E4:1 -:3 D4:2 -:4 A5:2 -:4 | D6:6 -:10",
            "Gm D7 | D7 | F7/Eb D7 D D | D7 Gm | D7/A Gm/Bb | Gm/Bb Dm/A | Dm | E7 A | A7 Dm A A | \
            Dm",
        )
        .with_left("Eb2:2 Bb2 C#4 Eb2:1 - D2:2 D3:1 -:2 F#3:3 | _:1 -:2 D5:1 A2 -:3 D2:2 -:6 | \
            Eb2:4 -:3 D2:2 D3:1 -:2 F#3:4 | _:2 - D2:1 -:3 G2:5 G#2:1 -:2 | \
            A2:4 A3:1 A4 A2 - Bb2:8 | _:6 -:2 A2:8 | _:1 -:2 A2 -:1 D2:2 D3:1 - F3:5 -:1 | \
            A3 D2 - G#3+B3:3 -:1 E3:4 A3:3 -:2 | G3:1 -:3 D2+D3+F3:2 -:4 A2+A4:1 -:5 | \
            D2:6 -:10 & D4:1 G4 -:2 G3:1 -:3 C4:1 -:7 | C4:1 -:8 D3:1 -:6 | -:1 G4:2 -:5 F#4:4 - | \
            C4:1 -:7 Bb4:3 -:1 Bb4 -:3 | -:2 F#3:3 - G4:2 G3:6 | _:5 -:4 F3:7 | _:2 -:12 A3:1 - | \
            -:4 E2:7 -:1 A2:4 | _:1 -:15 | D3+D5:5 -:11 & -:16 | - | -:2 Eb3:1 -:13 | \
            -:10 G3:3 - | - A4:1 -:5 Bb4:3 -:1 G4 -:2 | -:3 Bb4:1 -:11 A3:1 | _:2 -:14 | -:16 | \
            - | - & -:16 | - | - | - | -:11 Bb3:5 | _:4 -:12 | -:16 | - | - | -")
        .with_tempo(&[tempo(0, 130), tempo(16, 132), tempo(48, 141), tempo(64, 133), tempo(80, 131), tempo(96, 127), tempo(128, 130), tempo(144, 128)]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8", "Part 9"],
    drums_from: 0,
    about: "\"Quiero emborrachar mi corazón para apagar un loco amor\": a man drinks to forget \
            a love that is more suffering than love. Cobián's tune in D minor, one of the \
            great tangos of the 1930s.",
};

// Copyright: Caldara died in 1967; not public domain; added at the owner's request.
// Written out from a piano performance (MIDI in 4/4).
pub(super) const PASIONAL: Song = Song {
    name: "Pasional",
    origin: "Jorge Caldara, words by Mario Soto, 1951",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:2 A4:1 -:3 B4:1 A4 B4:3 A4:1 B4 -:3 | -:2 A4:1 -:3 B4:1 A4 B4:3 A4:1 B4 -:3 | \
            -:2 G#4:1 -:2 G#4:1 A4 B4:2 -:1 A4 B4 A4:2 G#4:1 - | A4:13 -:3 | \
            -:2 A5:4 B5:1 A5 B5:3 A5:1 B5:3 -:1 | C4:2 A5:4 B5:1 A5 B5:3 A5:1 B5 -:3 | \
            -:2 G#4 -:1 G#4 A4:2 B4 A4:1 B4 A4:2 G#4:1 - | A4:3 -:6 C5:2 - E6:3",
            "Dm G7 | Am E7 | E7 E | Am | Dm G/D | Am/C | E7/B E | Am C7/Bb",
        )
        .with_left(
            "D2:1 - D4+F4 -:5 B3:3 - G2:1 G#2 | A2:3 -:5 B3+C4+E4:1 -:5 D3:1 E3 | \
            _:6 -:1 E2:7 -:2 | C4:6 -:2 C4+E4:5 -:3 | D2:2 D5:5 -:1 D5:6 A3:1 B3 | \
            -:2 C5:5 -:1 C5:5 -:1 A2 A#2 | B2:8 E2:6 -:2 | A2:3 -:6 F3 A3:1 & -:8 D4+F4:2 -:6 | \
            -:2 C4+E4:1 -:12 D#3:1 | -:2 D4:1 -:4 D4:1 -:8 | A2:5 -:3 C3:1 -:3 C#2+C#3:1 -:3 | \
            -:2 F5:4 -:2 D4:4 - | -:2 E5:4 -:2 E5 -:6 | -:2 D4 -:12 | C4:2 -:7 Bb2:5 -:2 & -:16 | \
            - | - | -:4 B2 -:8 | -:16 | - | - | -:9 D4+G#4:1 -:3 E5",
        )
        .with_tempo(&[tempo(112, 86)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "C4:1 E4:11 -:1 E4:2 -:1 | E4 E4 E4:10 -:2 E4:1 - | - E4 E4 - D#4:5 -:1 C4 - C4 -:3 | \
            -:5 E4:8 -:3 | -:1 D4 E4 F4:3 D4:1 F4 A4 D5 F5 F5:2 F5 -:1 | \
            E5 F5 A5:6 -:4 A4:1 - G#4 A4 | - C5:10 -:1 B4 -:2 B4:1 | A4 G#4:15",
            "Am E/G# | E/G# G G G | G B7 G7/B G7/B | Am E E7 E7 | Dm A | Dm7 Dm | B F7 | E E7",
        )
        .with_left(
            "-:1 A2:3 -:2 A2:1 -:3 G#2:6 | _:2 B3:9 -:1 G2:4 | _ B2:9 -:3 | \
            A3 A3:1 - E2:2 B2 E3 D#3 -:3 | - D2+D3:5 -:1 A4 - C#2:5 | \
            -:2 C2+C3:3 - F3:1 - A3 -:5 | -:2 D#4:10 -:4 | \
            -:1 E3 -:3 E4:2 Eb4 D4 B3:1 - G#3 E3 - & -:16 | -:3 G2:7 -:4 B3:1 - | \
            - B3 -:2 A3:4 - F2 | -:2 G#3:1 -:9 D3:2 - | -:5 A3:1 -:5 C#3:4 -:1 | -:3 D5:6 -:7 | \
            -:2 F#2:3 -:1 F#2 -:3 F2:2 -:4 | -:16 & -:16 | - | -:12 B3:3 -:1 | -:16 | \
            -:11 A4+C#5:3 -:2 | -:16 | - | - & -:16 | - | -:12 B3:1 -:3 | -:16 | - | - | - | -",
        )
        .with_tempo(&[tempo(0, 90), tempo(16, 102), tempo(64, 106)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "-:5 C4:2 -:6 D4:1 C4 - | -:2 C4 E4 D4:1 -:3 D4:2 F4 E4:1 - | \
            -:5 D#4:1 F#4 B4:3 A4 B4:1 A4 G#4 | _:12 -:1 E6 C6 E5 | _:6 -:4 E5:3 G5:2 F5:1 | \
            E5:6 -:4 E5:1 -:2 E5:1 E5 D#5 | _:3 - C5:1 - C5 B4:2 A4:1 -:2 G#4:1 B4 | A4:13 -:3",
            "Am E7/G# | C/G Dm/F | B/F# B/F# B7/F# E7 | E7 Dm/F E7 Am/E | Am E/G# | C/G B7/F# | \
            F7 E | B7/A B7/A Am/C A",
        )
        .with_left(
            "-:1 A2:5 -:1 A2+B3 - G#2:4 -:1 G#2 - | G2:4 -:2 G2:1 - F2:7 -:1 | F#2:10 -:5 D4:1 | \
            _:12 E2:2 A5:1 A2 | _:4 A3:1 -:3 G#2:5 -:3 | G2:4 G3:1 -:3 F#2:4 F#3:1 -:2 F3:1 | \
            _:4 - E3:3 -:5 | A2:4 B2 C3:3 -:1 C#3 -:3 & -:3 A3:2 -:6 B3:2 -:3 | -:16 | \
            -:2 B3:3 -:10 E2+D3:1 | _ -:3 F3:4 E3:2 -:6 | -:12 G#3:1 -:3 | -:15 F2:1 | \
            _:3 -:5 B4:1 -:7 | -:16 & -:16 | - | - | - | - | -:15 A4:1 | _ -:15 | -:16",
        ),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "-:2 A5:1 -:3 B5:1 A5 B5:3 A5:1 B5 -:3 | -:2 A4:3 B4:1 A4 - B4:2 -:1 A4 B4 -:3 | \
            -:1 G#4 -:3 G#4:1 A4 B4 -:2 A4:1 B4:2 G#4:1 - A4 | _:13 -:3 | \
            -:1 A4 D4:2 - A4:1 - B4:2 A4:1 - B4 -:3 | -:2 A4 - A4:1 - B4:2 A4 B4:1 -:3 | \
            -:2 G#4 -:1 G#4 A4 B4:5 A4:2 G#4:1 - | A4:14 -:2",
            "Dm G7 | Am C7 | E7/B Am7 | Am | Dm G7 | Am Am C7/Bb G | E7 E7 B/F# E/G# | Am Am/C",
        )
        .with_left(
            "D2:2 D5+F5:1 -:5 D5+F5:2 -:4 G2:1 G#2 | A2:3 -:5 A#3+B3+E4:2 -:4 A2+Bb2:1 B2 | \
            _:7 E2:4 A4:2 G2 C4:1 | _:12 C#3:1 - C3+Db3 D3 | _:11 -:3 G2:1 G#2 | \
            A2:2 E4 -:4 A#3+B3:2 -:4 D3:1 D#3 | E3:4 -:3 F#2:4 -:1 G#2:4 | \
            C4:16 & -:8 F4+B4:1 -:7 | -:2 C4+E4 -:4 C4:1 -:7 | \
            -:1 D4+F#4 -:5 D4:1 -:4 F2:1 - G#2 A2 | _:4 -:3 E4:5 -:4 | \
            -:1 F4:2 -:5 A#3+B3+F4:2 -:6 | -:2 C4:1 -:5 C4+E4:1 -:7 | -:2 D4:4 -:10 | \
            E4:15 -:1 & -:16 | - | - | -:3 B2:4 C3:3 -:6 | -:8 D4+E4+Bb4:1 -:7 | -:16 | \
            -:3 E2:4 -:9 | A2:5 -:3 C3:4 C#3 & -:16 | - | - | - | - | - | - | -:4 B2 -:8",
        )
        .with_tempo(&[tempo(48, 104), tempo(64, 101), tempo(112, 97)]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "-:2 F4:3 -:2 F4:1 F4:3 -:1 F4 - F4 F4 | A4:8 E4:5 -:3 | \
            -:1 F5:3 -:2 F5:1 F5:2 -:1 B5:2 C6:1 B5:2 A5:1 | _:3 C4:2 D4 E5 - E4:4 -:1 | \
            -:2 A4:3 B4:1 - B4 D#4:2 A4:3 -:1 B4 G#4 | _ - E4:3 A4:1 G#4 A4:5 G#4:1 A4 - F#4 | \
            _:2 D4:3 G4:1 F#4 G4:4 F4:1 G4 F4 E4 F4 | _ E4:2 D#4 E4:1 - G#4:6 -:3",
            "Dm7 | Am | Dm7 Dm7 G7 Am | Am7 C7 | B7 B7 B7 E7 | E7 A7 A7 D | G G G7 C | C E7",
        )
        .with_left(
            "D3:15 -:1 | - A2:2 B2 C3 -:1 A3+C4:4 A2:1 B2 C3 D3 | _:6 -:1 G2:4 - C3:1 | \
            _ G3:2 -:4 C5:2 -:3 C3:4 | B2:11 -:2 E2:3 | _:1 -:6 A2+G3:5 -:3 D3:1 | \
            _:7 G2:4 - C2+C3:1 | _:2 -:3 E2:2 D4:5 B2+C3:1 C#3 D3:2 & -:2 A3+C4:8 -:6 | \
            - E3:2 -:7 C#3:1 | - A4+C5:2 -:13 | -:7 E4:1 -:4 Bb3:2 - | \
            - D#4:5 A4:1 A3:3 -:4 E3:1 | _:2 -:5 C#4:2 -:6 D2:1 | _:3 -:4 D4:3 -:6 | \
            -:1 D#4 -:5 E3:3 -:1 A2 -:4 & -:8 D2:4 - | -:16 | - | - | -:15 D4:1 | _ -:15 | \
            -:7 B3:1 -:8 | -:7 C4:2 -:7",
        )
        .with_tempo(&[tempo(0, 100), tempo(64, 109)]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "B4:2 - B4:1 B4 B4:5 A4:1 G#4 - A4:2 | _:5 E4:7 -:3 B4:1 | \
            _ - C5 B4 C5:2 B4 -:1 A#4 B4 D5:5 | _:2 -:1 C5:6 -:4 G#4:1 -:2 | \
            F4:1 A4 B4:3 -:1 A4:3 - A4:1 -:2 E4:1 | A4 B4:4 A4:3 - G#4:2 -:1 G#4 A4 | \
            _ B4:3 A4:2 G#4:1 - A4:4 -:3 C4:1 | E4:8 E4:3 -:1 E4:2 G4:1 F4",
            "G7 G7 Dm Am | Am Am C G7/D | G7/D G | Am G | Dm Am/E | Am/E E7 | B7 Am | Am E",
        )
        .with_left(
            "F4:3 -:1 D4+F4 D4+F4 D4:6 F2:1 - C4:2 | _:3 -:2 C4:7 -:1 D3:3 | _:11 -:2 B2 C3:1 | \
            _ E3:2 E4+A4:5 B2:1 C3 C#3 D3:5 | _:2 F4:4 D4:2 A2 E2:6 | _:5 -:3 A2+Bb2:1 B2:7 | \
            _ -:1 A2+C4+E4:4 -:1 A3:2 -:1 | \
            A2:3 -:1 E3 - A2 - G#2:3 -:1 E3 - G#2 - & D4:2 -:4 F4:5 -:3 A2:2 | \
            _ B2:4 -:3 C#3 - F4:1 | _:2 -:14 | -:3 A3:4 A2:1 -:5 D4:3 | \
            -:2 D4:3 - G#2:1 -:3 C4 -:1 | - C4+E4:4 -:6 D4:4 -:1 | -:16 | \
            -:2 C3 -:6 C3:2 -:4 & -:10 E2:3 -:1 B3 - | -:5 C3:3 -:7 D4:1 | _ -:15 | -:16 | \
            -:4 A3+B3:2 -:6 E4+G#4:1 -:3 | -:16 | - | -",
        ),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "-:1 E4:3 -:6 E4:2 -:1 E4 E4 D#4 | _:4 -:1 C4:2 -:1 C4 -:6 E4:1 | _:12 -:1 D4 E4 F4 | \
            _:3 -:1 F4:3 -:1 F4:3 F4:1 - F4 F4 A4 | _:6 -:3 A4:1 -:2 A4:1 B4 C5:2 | \
            _ B4:9 A4:1 -:3 G#4:1 | _:13 -:3 | -:5 C4:2 -:6 D4:2 C4:1",
            "C/G D7 | F7 E7 | E7 E7/G# | Dm A/C# | F7 | F7 F7 F7 E7 | E7/D E7/D D/A E7 | Am E7",
        )
        .with_left(
            "G2:4 E3:2 G2 F#2:3 -:1 E3 - F#2 - | F2:8 E2 | _:1 -:2 B2:5 G#2 -:1 F#2 A3 | \
            _:13 -:2 C4:1 | _:5 -:1 E3 F3:4 -:3 Eb4:2 | _:11 -:4 D4:1 | _:13 -:3 | \
            A2:5 -:2 A2 G#2:3 - G#2:1 & -:2 C3 -:6 C3:2 -:4 | -:7 B3:1 B3:2 -:1 A3:2 G#3:1 A3 D3 | \
            _:3 -:12 D3:1 | _:8 C#3:7 C3:1 | _:3 -:1 Eb3:2 -:3 F4:1 -:4 F2:2 | _:10 -:5 E2+E3:1 | \
            -:3 B3:4 A3 -:1 G#3 -:3 | - A3:2 - B3:1 -:3 B3:2 -:3 & -:16 | - | -:15 D2:1 | \
            _:3 -:5 C#4:4 - | -:16 | -:7 F3:3 -:6 | -:16 | -",
        ),
        // Bars 57-64.
        Part::new(
            "Part 8",
            "-:2 C4:1 - E4:2 D4:1 -:3 D4:1 - F4:2 E4:1 - | \
            -:3 D#4:1 - F#4 D#4 B4:3 A4:2 B4:1 A4 G#4:2 | _:9 -:6 E6:1 | \
            E5:5 -:6 E5:1 C4 - G5 F5 | F5 -:2 C4:3 E4:2 - E5:1 - C4 E5 E5 - | \
            Eb5:3 -:1 C5 -:2 C5:1 C5 - A4 -:2 G#4:1 B4 A4 | _:13 -:3 | \
            -:2 A5 B5:1 - A5 D5 B5:2 A5:1 B5 -:4",
            "C Dm/F | B/F# E/B | E7/D E7/D C E/B | Am Am E/G# C | C/G C/G D/F# C | F7 E | B7 A | \
            Dm G7",
        )
        .with_left(
            "-:1 G2:2 -:3 G2:1 - F2:5 -:2 F#2:1 | _:6 B2:8 E2:2 | - D3:5 C3 B2:3 -:1 | \
            A2:4 C3:1 -:3 G#3:4 -:3 E5:1 | E5:8 F#3:4 - | F3:3 -:4 B4:1 E3:4 E2:1 F2 G2+G#2 A2 | \
            _:2 -:1 B2:4 C3:1 -:4 C#3:1 B2 C3 C#3 | \
            D3:2 D5+F5 -:3 B4:1 F5:2 -:3 G2:1 G#2 A2 & -:16 | -:1 B3:2 -:13 | -:16 | - | \
            G3:5 -:11 | A4:1 -:7 B4:2 -:6 | -:16 | -",
        ),
        // Bars 65-71.
        Part::new(
            "Part 9",
            "-:1 A4:2 - B4:1 A4 B4:3 A4:1 B4 -:4 | \
            -:1 G#4 -:2 G#4:1 A4 B4:2 A4:1 B4 A4:2 G#4:1 - A4:2 | _:11 -:5 | \
            A4:3 -:1 B4 A4 B4:3 A4:1 B4 -:5 | A4:3 B4:1 A4 B4:3 A4:1 B4 -:5 G#4:1 | \
            -:3 G#4:1 A4 B4 - A4 B4 -:2 G#4:1 A4 -:3 | E5:1 -:5 C5:1 F#5 B5 -:7",
            "Am Am E B7 | E7/B Am | Am Am Am/C D | Dm Dm G7 A7 | Am Am B7 E7 | E/B E/B B7 Am | \
            E7 D7/A B7 B7",
        )
        .with_left(
            "-:1 C4+E4:2 -:4 E4:2 -:4 A2:1 A#2 B2 | _:6 E2:3 -:1 F#2:2 G#2 E4 | _:11 -:3 D2+D3:2 | \
            _ -:4 F4:3 - G2:1 G#2 A2:2 | _:1 -:4 A#3+B3+C4+E4:1 -:5 A2:1 A#2 B2:3 | \
            _:4 -:1 E2:2 -:1 A4 -:3 A2 -:1 | D5:2 -:3 A3:4 -:7 & -:7 A#3+B3+C4:1 -:8 | \
            -:1 D4+F#4 -:4 D4:1 -:7 C4:2 | _:10 C#3:1 -:5 | F4:4 -:2 A#3+B3+D4 -:8 | \
            E4:3 -:12 D4:1 | -:12 E4:1 -:3 | E2+G#4:1 -:5 F#4:4 -:6 & -:16 | -:14 A2:2 | \
            _:1 - B2:3 -:1 B2+C3 -:9 | D4:2 -:14 | C4:2 -:14 | -:16 | -:7 E5:1 -:8",
        ),
    ],
    form: &[
        "Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8", "Part 9",
    ],
    drums_from: 0,
    about: "A lover's feverish longing. Caldara was a bandoneonist in Osvaldo Pugliese's \
            orchestra, which made the song famous with the singer Alberto Morán. A minor.",
};

// Copyright: public domain in Argentina, where a work is protected for 70 years after its author's
// death: Jovés died in 1927.
// Written out from a piano performance (MIDI in 4/4).
pub(super) const PATOTERO_SENTIMENTAL: Song = Song {
    name: "Patotero sentimental",
    origin: "Manuel Jovés, words by Manuel Romero, 1922",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: C,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 110.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:10 G4:3 -:1 Eb5 - | D5:4 C5:6 B4:2 -:1 D5 C5 - | C5:4 B4:6 -:1 G4 -:2 F5:1 - | \
            Eb5:4 D5:5 F5:2 -:1 Eb5 - D5 Eb5 | -:4 C4:1 Eb4 F4 G4 -:2 G4 - G4:1 C5 | \
            _:3 -:2 C5:1 -:2 Bb4:4 -:1 Bb4 -:2 | Ab4:4 -:1 Ab4 -:2 G4:4 G4:1 -:3 | \
            F4:5 - G4:3 Bb4:1 Ab4 G4",
            "Cm | Cm Cm/Eb | G7 | G G Bdim7 Cm | Cm | Cm Bb | Ab C7b9 | Fm Bb7/Ab",
        )
        .with_left("-:8 C2:5 -:3 | C2 -:1 C3:2 D3 -:8 | G2:7 -:1 F3 -:7 | G2:2 D3 G3 A3 -:7 C2:1 | \
            _:2 -:3 D4:1 - C2+C3:2 -:6 G4:1 | _:5 -:3 D4+F4:6 -:2 | C4:6 Bb2:1 C3:8 -:1 | \
            Ab3+C4:5 -:1 F#2 G2 Ab2:6 -:1 D2 & -:16 | -:2 G2 -:3 Eb3:5 -:4 | -:2 D3:6 -:8 | \
            -:7 B3:3 -:6 | -:15 C3:1 | _:4 - Bb2:5 -:3 | Ab2+Eb4:5 -:2 B2:1 Bb3:6 -:2 | \
            F2:3 -:13 & -:16 | - | -:4 F4:5 -:7 | -:16 | -:15 C4+Eb4:1 | _:3 -:9 Bb3:1 -:3 | \
            -:4 Ab3:1 -:3 Db4+E4:5 -:3 | -:16")
        .with_tempo(&[tempo(80, 101), tempo(96, 95), tempo(112, 109)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "-:4 C6:1 -:3 B5:1 B4 A4 Ab4 G4 - Eb5 - | D5:4 C5:5 -:1 C5 - B4 C5 D5 C5 | \
            C5:2 -:1 B4:8 -:1 G4:2 F5:1 Eb5 | _:3 D5:5 -:1 F5:2 Eb5:1 F5 Eb5 D5 Eb5 | \
            _:9 -:2 G4 -:1 Eb5 - | D5:3 C5:1 - C5 Bb4 Bb4:4 Ab4:1 - Ab4 - G4 | \
            G4:4 F4:3 -:5 F4:1 - Ab4 G4 | _:5 C5:2 -:1 B4:6 D5:1 -",
            "Ab7 Ab7 G7 G7 | D7 Cm/Eb | G7 | G7 G7 G7 F7 | Cm Eb | C7 | Fm Fm7 | Cm/Eb Ddim7",
        )
        .with_left("-:4 Ab4+Eb5+Gb5:1 -:3 G4+D5+F5:1 -:7 | C2:2 G2 C3 D3 Eb3:5 -:2 G2:1 | \
            _ - D3 F4:4 -:1 G3:5 -:2 G2:1 | _:9 -:6 C3:1 | _ - G3:2 C4 D4 -:8 | \
            C3+C4:3 -:4 C3+E4 -:5 | F2:2 C3 F3 G3 -:7 Eb2+Eb3:1 | _:4 - F4:6 -:2 & -:16 | - | \
            -:6 F3+Gb3:1 -:9 | -:4 D3:6 - | -:7 Eb4:2 -:7 | -:8 Bb3:4 - | -:7 Ab3:3 -:6 | \
            -:8 D2:4 - & -:16 | - | - | -:7 F3:2 -:7 | -:16 | - | - | -"),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "C5:1 -:3 G5:1 -:3 C6:2 - C4 D4:1 Eb4 | _:4 D4:2 C4:1 F#4 -:5 C4:2 -:1 | \
            -:12 Ab4:1 Bb4 - C5 | _:2 Bb4:3 -:1 Ab4 -:2 C5 Bb4:1 C5 Bb4 - Ab4 | \
            G4:6 - C6:1 D6 - Eb6 | _ -:4 Eb6:1 Eb6 Eb6 -:2 Eb6:1 D6 - Db6 -:2 | \
            D6 -:3 D5:1 D5 - D5:2 D5 C5:1 B4:2 D5:1 | _:2 -:3 D5:1 D5 D5:2 -:1 D5:2 Eb5 F5",
            "Cm G Cm Cm | Cm Cm C/E Bdim7 | Fm7 Dm7b5 | Fm Bb | Eb Cm6 | Cm7 Eb7/Bb | Dm/A Abdim7 | \
            G7",
        )
        .with_left("C2:4 G2 C2+Eb5+G5:2 -:5 C2:1 | _:5 -:2 C4:5 B3:1 -:2 Ab3:1 | _:5 -:1 Eb3:2 -:7 F2:1 | \
            _:4 -:5 Bb2:7 | Eb4:6 -:2 C3:4 C5+G5:1 A5 - C2 | _:4 C3:1 Eb5 Eb5 Bb2:8 -:1 | \
            A2:8 Ab2:4 -:3 G2:1 | _:3 -:4 G3+B3+D4:5 F3:1 -:3 & Eb4+G4:1 -:3 B4+D5:1 -:11 | \
            -:4 C3:3 E2:4 - F2:1 | _:2 C3 F3 -:1 D3:5 -:3 F4:1 | _:3 -:6 D4:2 -:5 | \
            Eb2:1 -:3 G3:4 -:7 Eb5:1 | _ -:6 Eb5:1 -:2 Eb5:1 -:2 Db5:1 -:2 | \
            D5+A5 -:6 F#4+G4:1 -:6 F4:1 | _:2 -:14"),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "Eb5:7 D4:1 Eb4:2 C6:3 D6:1 -:2 | Eb6:1 -:4 D5:1 C5 - C5:2 B4:3 C5 | \
            Ab4:2 C4:1 D4 Ab4 C5 D5 Ab5:3 - Ab4:1 Bb4 C5 | \
            _:2 Bb4:3 Ab4:1 - C5:3 Bb4:1 C5:2 Bb4:1 Ab4:2 | G4:7 -:5 C5:1 - D5 - | \
            Eb5:3 -:1 Eb5 - Eb5 Eb5:2 - Eb5 D5:1 C5:2 | \
            Ab4:5 Ab4:1 Bb4 C5:2 -:1 C5:2 Bb4:1 - Ab4 G4 | _ -:3 B4:2 C5 D5:1 - F5 -:3 Eb5:1 D5",
            "Cm | Cm C7 | Fm6 Fm | Bb7 | Eb D7/C | Cm7 | Fm | Cm Dm",
        )
        .with_left("C3:2 D3 G3:1 - C4 -:3 G5 D5:1 -:2 | C2:4 - C2+E2+Bb2:3 -:5 | F3:2 - F4:1 -:10 F4:1 | \
            _:5 -:2 Bb2+D4:4 -:1 F3 -:3 | Eb4:7 -:1 C2:4 C3:1 -:3 | C3:6 -:1 C3+Bb3:5 -:4 | \
            F3+Ab3+C4:11 -:4 C2:1 | _:3 -:1 G2:2 Eb2 D2:3 -:5 & -:3 Eb3:1 -:6 C5:2 -:4 | \
            Eb5:2 -:14 | -:1 Ab3 -:13 Ab2:1 | _:3 -:1 Ab3 -:11 | Eb2:1 -:3 G2:4 -:8 | \
            G4:3 -:1 C4 -:2 G4:4 -:5 | -:16 | -:4 F4:1 -:11 & -:16 | - | - | - | -:4 F#2:1 -:11 | \
            -:16 | - | -"),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "C5:1 -:3 G5:1 -:3 C6:2 G4:3 -:1 Eb5 - | D5:4 C5:6 B4:2 -:1 D5 C5 - | \
            C5:4 B4:6 -:1 G4 -:2 F5:1 - | Eb5:4 D5:5 F5:2 -:1 Eb5 - D5 Eb5 | \
            -:4 C4:1 Eb4 F4 G4 -:2 G4 - G4:1 C5 | _:3 -:2 C5:1 -:2 Bb4:4 -:1 Bb4 -:2 | \
            Ab4:4 -:1 Ab4 -:2 G4:4 G4:1 -:3 | F4:5 - G4:3 Bb4:1 Ab4 G4",
            "Cm G Cm Cm | Cm Cm/Eb | G7 | G G Bdim7 Cm | Cm | Cm Bb | Ab C7b9 | Fm Bb7/Ab",
        )
        .with_left("C3:2 C2 G2:4 C2+Eb5+G5:2 -:6 | C2:3 -:1 C3:2 D3 -:8 | G2:7 -:1 F3 -:7 | \
            G2:2 D3 G3 A3 -:7 C2:1 | _:2 -:3 D4:1 - C2+C3:2 -:6 G4:1 | _:5 -:3 D4+F4:6 -:2 | \
            C4:6 Bb2:1 C3:8 -:1 | Ab3+C4:5 -:1 F#2 G2 Ab2:6 -:1 D2 & -:4 B4+D5:1 -:11 | \
            -:2 G2 -:3 Eb3:5 -:4 | -:2 D3:6 -:8 | -:7 B3:3 -:6 | -:15 C3:1 | _:4 - Bb2:5 -:3 | \
            Ab2+Eb4:5 -:2 B2:1 Bb3:6 -:2 | F2:3 -:13 & -:16 | - | -:4 F4:5 -:7 | -:16 | \
            -:15 C4+Eb4:1 | _:3 -:9 Bb3:1 -:3 | -:4 Ab3:1 -:3 Db4+E4:5 -:3 | -:16"),
        // Bars 57-66.
        Part::new(
            "Part 8",
            "Eb5:7 D4:1 Eb4:2 C6:3 D6:1 -:2 | Eb6:1 -:4 D5:1 C5 - C5:2 B4:3 C5 | \
            Ab4:2 C4:1 D4 Ab4 C5 D5 Ab5:3 - Ab4:1 Bb4 C5 | \
            _:2 Bb4:3 Ab4:1 - C5:3 Bb4:1 C5:2 Bb4:1 Ab4:2 | G4:7 -:5 C5:1 - D5 - | \
            Eb5:3 -:1 Eb5 - Eb5 Eb5:2 - Eb5 D5:1 C5:2 | \
            Ab4:5 Ab4:1 Bb4 C5:2 -:1 C5:2 Bb4:1 - Ab4 G4 | _ -:3 B4:2 C5 D5:1 - F5 -:3 Eb5:1 D5 | \
            C5 -:3 G5:1 -:5 C4:1 Eb4 A5 D6:3 | _:6 -:10",
            "Cm | Cm C7 | Fm6 Fm | Bb7 | Eb D7/C | Cm7 | Fm | Cm Dm | Cm G Cm Am7b5/G | Am7b5/G",
        )
        .with_left("C3:2 D3 G3:1 - C4 -:3 G5 D5:1 -:2 | C2:4 - C2+E2+Bb2:3 -:5 | F3:2 - F4:1 -:10 F4:1 | \
            _:5 -:2 Bb2+D4:4 -:1 F3 -:3 | Eb4:7 -:1 C2:4 C3:1 -:3 | C3:6 -:1 C3+Bb3:5 -:4 | \
            F3+Ab3+C4:11 -:4 C2:1 | _:3 -:1 G2:2 Eb2 D2:3 -:5 | C3:2 C2 G2:4 - G4+Eb5 | \
            _:6 -:10 & -:3 Eb3:1 -:6 C5:2 -:4 | Eb5:2 -:14 | -:1 Ab3 -:13 Ab2:1 | \
            _:3 -:1 Ab3 -:11 | Eb2:1 -:3 G2:4 -:8 | G4:3 -:1 C4 -:2 G4:4 -:5 | -:16 | \
            -:4 F4:1 -:11 | -:4 B4+D5:1 -:7 A4:4 | _:5 -:11 & -:16 | - | - | - | -:4 F#2:1 -:11 | \
            -:16 | - | - | - | -")
        .with_tempo(&[tempo(80, 100), tempo(96, 110), tempo(112, 101)]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 2", "Part 3", "Part 8"],
    drums_from: 0,
    about: "The tough guy of the gang, the patotero, confesses that he weeps for a woman. \
            Romero, who wrote the words, later became a film director. C minor.",
};

// Copyright: Pontier died in 1983; not public domain; added at the owner's request.
// Written out from a piano performance (MIDI in 4/4).
pub(super) const QUE_FALTA_QUE_ME_HACES: Song = Song {
    name: "Qué falta que me hacés",
    origin: "Armando Pontier, words by Federico Silva",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 110.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:16 | -:4 C4 -:8 | -:16 | -:4 C4 -:7 D4:1 | D4:13 -:1 F4 F4 | \
            _ Eb4:6 -:1 Bb4:4 -:1 Eb4 F4 D4 | _:7 -:4 F5:1 F4 -:2 F4:1 | \
            _:4 Eb4 -:2 Bb4 -:1 Bb4 - Bb4",
            "Dm | D7 D7 G/B Bb | Dm | D7 G/B | Dm Dm/F | F7 Ebmaj7 | Dm F | F7/Eb F7/Eb Eb Bb",
        )
        .with_left(
            "D2:4 F2 D2 A2 | D2 C3 B2+B3 Bb2+Bb3:2 - | D2:4 F2 D2 A2 | D2 C3 B2:5 -:3 | \
            D2:4 D3:1 -:3 F3:4 -:3 Eb2:1 | _:3 Eb3:1 -:4 Eb3 G3:1 - Eb4 D2 | \
            _ A2:2 D3 E3 F3:3 -:6 | \
            Eb2:4 Eb3:1 -:2 Eb3:4 G3:1 -:3 D2:1 & D3:1 -:3 F3:1 -:3 D3:1 -:3 A3:1 -:3 | \
            D3:1 -:15 | D3:1 -:3 F3:1 -:3 D3:1 -:3 A3:1 -:3 | D3:1 -:7 B3:1 -:7 | -:16 | - | - | \
            -",
        ),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "-:4 A4:6 F4:3 -:1 A4 - | A4:4 G#4:3 - D5:1 - E5 - D5 C#5 | _:11 A4:5 | \
            C5:2 -:1 C4 Eb4 F#4 C5:3 -:1 Bb4:2 A4:1 G4:2 -:1 | \
            F#4:2 -:4 Eb6:2 Eb6:1 -:2 D6:1 - D6 - C6 | C6:4 - B4:3 - G4:1 - | Bb4:16 | \
            _:2 - D5 - D5:3 C5:1 -:2 C5:1 Bb4",
            "Dm | E7 E7 Bb7 A7 | A7 | D7 D7 C7 A7 | D D7 | C/G G G7 G7 | C7 Edim7 | C7 C",
        )
        .with_left(
            "-:4 D4:5 -:1 D4:2 -:4 | E2:6 -:2 Bb2 F4+Ab4:1 -:4 G4:1 | _:11 -:5 | \
            D3:2 A3:1 -:3 A4:1 -:9 | D2+D3:5 -:1 Eb5 - D2+D3:3 D5:1 - D5 - C5 | G2:12 F3:1 -:3 | \
            E4:16 | _:3 -:1 E4:3 -:1 C3:3 -:5 & -:8 D3:7 -:1 | D4:3 -:1 D4:3 -:8 A2:1 | \
            _:4 -:3 A3:8 -:1 | - F#3:2 -:13 | -:8 Eb5:1 -:7 | C5:5 -:3 G3:1 -:7 | \
            C2:2 G2 C3 -:1 E3:2 -:3 G2:4 | C2:11 -:5 & -:16 | - | -:3 E3:4 -:9 | -:16 | - | \
            -:4 D3:3 -:9 | -:5 D3:2 -:9 | -:8 E4:2 -:6",
        ),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "Bb4:2 A4:1 -:2 A4:1 C5 - Bb4:2 A4:1 - A4 - C5 - | Bb4:8 A4:2 -:3 F4:1 E4 Eb4 | \
            D4:13 -:1 F4 F4 | _ - Eb4:2 -:3 Bb5:1 Bb4:3 -:2 F4:1 Eb4 D4 | _:12 -:2 F4:1 F4 | \
            _ Eb4:3 -:6 Bb5:1 -:2 Bb5:1 -:2 | Bb5:1 -:3 A5:2 -:7 F4:1 - A4 | _:3 G4:6 E4:4 F4:3",
            "F A | Bbmaj7/D Em7b5/D F7 F7 | Dm Dm E7 F7 | F7/Eb Ebmaj7 | Dm E7/D Dm/F Dm/F | \
            F7/Eb Eb | Bb/D D7 F/C F/C | B7 Gm/Bb A7 A7",
        )
        .with_left(
            "F2:3 - F2:1 - A2:4 -:2 A2:1 - | D2:2 A2 D3:3 -:1 F3:3 -:5 | \
            D2:4 D3 E3 F3:1 -:2 Eb2:1 | _:3 Eb3:1 -:4 Eb3 G3:1 Eb4 - D2 | \
            _ A2:2 D3:3 E3:2 F3:4 -:3 Eb2:1 | _:4 - Eb2:3 -:2 Bb4:1 -:2 | D2:4 D3 -:7 B2:1 | \
            _:3 Bb2:4 A2 C#2:1 -:4 & F4:2 -:6 C#4:3 -:5 | -:6 E3:2 -:8 | -:16 | - | - | \
            -:10 Bb4:1 -:5 | Bb4:1 -:3 A4 C3:7 -:2 | -:16",
        ),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "D4:1 -:3 D6 -:1 D6:2 -:1 D6 - D6 D6 - | D6 - D6:2 - D6:1 D6 -:2 D6:1 - D6 D6 D6 - | \
            - C6:2 - C5:1 C5 - C5:3 B4:1 C5 C#5:2 -:1 | -:3 E4:1 F4 G4:2 - C#4 D4 E4:1 -:2 | \
            -:1 E5 A4 - E5:3 Bb4:1 -:2 Bb4:1 Bb4 A4:4 | C5:3 -:1 C5:2 -:1 C5 C5 - C5 C5:3 Bb4:2 | \
            -:1 Bb4 - D5:2 -:1 C5 - C5 Bb4 Bb4:2 -:1 A4:2 -:1 | \
            A4 - C5:2 Bb4:1 - Bb4 - A4 A4:2 G4 -:1 G4 -",
            "Dm | Gm Gm C7 Bb | F Am | A7 | A7 Gdim7 Gm Dm | D7 Gm | C7 F | D7 G",
        )
        .with_left(
            "D2:10 -:1 D5 - D5 D5 - | G2:5 -:1 D5+Bb5 C3:4 -:1 D5+Bb5 D5+Bb5 F2:2 | \
            _ -:1 F3 -:2 A3:3 -:4 A2:3 | _:9 G3:1 -:3 A2 | _:1 - G4:2 G3:5 -:3 D3:4 | \
            _:6 -:5 G2:4 -:1 | E3 -:2 C2:4 C3:1 -:2 F2:5 -:1 | \
            -:2 D2:4 D3:1 -:2 G2:4 -:3 & -:4 D5:3 -:1 D5:2 -:6 | \
            D5+Bb5:1 - D5+Bb5:2 G3:1 -:2 C2:3 D5+Bb5:1 -:3 D5+Bb5:1 - | - C5:2 -:13 | \
            -:5 E3:4 -:7 | -:4 Db4+E4 -:8 | -:11 D4:3 -:2 | -:3 Bb4:2 -:9 F3:1 - | \
            -:2 F#4:3 -:4 D4:3 -:4 & -:16 | -:7 D5+Bb5:1 -:8 | -:16 | - | -:5 Bb3+G4:4 -:7 | \
            -:16 | -:3 E4:1 -:12 | -:16",
        ),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "Bb4:3 A4:1 - A4 G4 - F4 -:3 D6 -:1 | D6:2 -:1 D6 - D6 D6 - D6:2 D6 - D6:1 D6 | \
            _ - D6 - D6 - D6 D6 -:2 C6:1 -:3 C5:1 C5 | _ C5:3 -:1 B4 C5 C#5 -:3 Bb5 -:2 | \
            A5:1 -:3 D4:2 - C#4 -:6 | -:5 G4:1 -:2 G4 F4:1 - F4:2 E4:1 - | -:16 | -:4 C4 -:8",
            "Edim7 Dm | Bb Gm | Dm7/C Bb F F | Am Bbm | A Bb7 A7 A7 | A7/C# A7/C# Em Fmaj7 | Dm | \
            D7 G/B",
        )
        .with_left(
            "E2:7 -:1 D2:8 | _:3 D5+Bb5:1 - D5+Bb5 D5+Bb5 - G2:4 D3:1 -:2 C2:1 | \
            _:2 D5+Bb5:1 - D5+Bb5 - D5+Bb5 D5+Bb5 F2:4 F3:1 -:2 A3:1 | \
            _:3 -:4 A2:1 -:3 Db5:2 -:3 | A4+C#5:1 -:3 Bb2+Ab3:1 -:3 A2:6 B2:1 C3 | \
            C#3:8 E2:4 A2:1 -:3 | D2:4 F2 D2 A2 | D2 C3 B2+B3:8 & Db4:3 -:5 D4:1 -:3 D5 -:1 | \
            D5+Bb5:2 -:6 D5:2 D5+Bb5 -:3 D5+Bb5:1 | _ -:9 C5:1 -:5 | -:12 Bb4:3 -:1 | \
            -:8 G3:2 - A2:4 | -:16 | D3:1 -:3 F3:1 -:3 D3:1 -:3 A3:1 -:3 | D3:1 -:15 & -:16 | \
            E5:1 -:7 Bb5:1 -:7 | C3:4 -:12 | -:16 | -:13 Bb2:1 -:2 | -:16 | - | -",
        )
        .with_tempo(&[
            tempo(64, 101),
            tempo(80, 110),
            tempo(96, 91),
            tempo(112, 105),
        ]),
        // Bars 41-47.
        Part::new(
            "Part 6",
            "-:7 D4:2 -:1 D4:5 -:1 | -:2 D4:1 F4 - A4 D5 - F5:2 A5 D6:3 -:1 | \
            -:4 F4:1 F4:5 Eb4:1 F4 Eb4 D4 Eb4:2 | Bb4:6 -:4 A4:6 | _:2 -:9 D4:1 F4 - A4:2 | \
            B4 E5:1 - F5:3 A5:2 B5:4 E6:3 | _ -:13",
            "Dm | Dm/F | F7 | Eb Am7b5 | Dm | Dm | Dm",
        )
        .with_left(
            "-:10 D2:1 -:3 D3:2 | F3:16 | _:1 -:4 Eb3:3 -:8 | Eb3:9 -:7 | -:2 D2:1 -:3 D3:10 | \
            _:16 | _:2 -:14 & -:16 | -:1 A3:15 | -:5 Eb2:2 -:9 | -:6 G3:3 -:7 | -:8 F3 | _:16 | \
            _:3 -:13 & -:16 | - | - | - | -:10 A3:6 | _:16 | _:3 -:13",
        )
        .with_tempo(&[tempo(0, 184), tempo(16, 118), tempo(32, 114)]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6"],
    drums_from: 0,
    about: "How much the one who left is missed. Pontier, a bandoneonist, played in Miguel \
            Caló's orchestra and then led one with the violinist Enrique Francini. D minor.",
};

// Copyright: Stamponi died in 1997; not public domain; added at the owner's request.
// Written out from a piano performance (MIDI in 4/4). A second track of high notes plays
// with the tune.
pub(super) const QUE_ME_VAN_A_HABLAR_DE_AMOR: Song = Song {
    name: "Qué me van a hablar de amor",
    origin: "Héctor Stamponi, words by Homero Expósito, 1946",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 95.4,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:2 D5:3 -:1 G5 A5 B5:2 - D6:1 C6 - B5 | _:7 -:1 A5:7 -:1 | \
            -:4 C6:2 A5:1 - B5:2 G5:1 - A5 - F#5 - | G5:3 -:13 | -:16 | - | - | -",
            "G G#dim7 | B7/A A | D7 D7 Em D/F# | A7 D | G Dmaj7 | F#m7b5/A F#m7b5/A Em/G F#7 | \
            Gmaj7/D F#7/E Bm7 Bm7 | A7/G F#m7b5 Em F#maj7/E#",
        )
        .with_left(
            "G2:5 -:1 G4 A4 G#2+G#3:5 C5:1 - A2+B4 | _:7 -:1 A4:7 C#3:1 | \
            D3:3 -:1 D2:3 -:1 E2:4 F#2 | G4:3 - C3:1 C#3 D3:8 | \
            G2:1 - D3:5 C#3:2 - C#3 D3:1 D#3 A2 | _:7 G2:5 F#2:4 | \
            G3:3 -:1 E2:4 -:2 F3:3 -:1 B3:2 | \
            G2:4 -:3 G3:5 -:3 F#2:1 & -:2 D4:3 - B4:2 - D5:1 -:3 | -:8 C#2:3 -:1 A#2 B2 C3 - | \
            -:4 C5:2 A4:1 - B4:2 G4:1 - A4 - F#4 - | G2:2 -:14 | -:8 D3:3 -:4 F#3:1 | \
            _:2 E3 -:1 E3 D#3 E3:3 D#3:2 E3 F#3:1 D2 | _:3 -:2 F#3:1 F3 F#2:4 -:1 D2:4 | \
            -:3 F#2:4 E2 F2 -:1 & -:16 | -:10 A2:2 -:4 | -:16 | - | - | - | \
            -:2 F#3 -:3 F#3:2 -:3 F#3:2 -:1 A3 | _:7 -:9",
        )
        .with_tempo(&[
            tempo(16, 110),
            tempo(32, 95),
            tempo(48, 90),
            tempo(64, 109),
            tempo(80, 110),
        ]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "-:2 D#4:3 -:1 D#4 - Eb4 Ab4 C5 -:2 B4:1 A4 - | A4:4 G4:3 -:5 G5:1 - A5 B5 | \
            _ -:2 A5:3 - A5:1 -:3 G5:1 - G5 | - F#4 - F#4 - E4 -:2 E4 D#4:1 - D#4:2 D4:1 - | \
            D4:6 C#4:2 D4 C#4:3 D4:2 D#4:1 | F#4:2 E4:4 D#4:1 E4:2 -:1 D#4:2 E4:1 F#4 - G4 | \
            _ F#4:4 F4:2 F#4:3 F4:2 F#4:1 B4:2 A4:1 | _:8 G4:6 B4:1 -",
            "B B Ab/Eb B7 | Am/E F#7b9 G G | A A A G | D D E7/D B/F# | G D/F# E7 E7 | \
            F#m7b5/A Em/G F#7 F#7 | B Bm7 | F#m7b5 G",
        )
        .with_left(
            "-:8 Eb2:3 -:5 | E2:2 E3:3 -:1 F#3:2 -:4 G4:1 - A4 A2 | _:5 C#2:4 A4:1 -:3 G4:1 - D3 | \
            _:6 -:1 D2:3 -:2 F#2:4 | G2 F#2:3 -:1 E2:3 -:1 D2:4 | A2 G2 -:3 E2 -:1 F#2 | \
            _:3 -:4 B2:7 -:1 E2 | _ B2:2 -:1 E3:2 F#3 - G2:3 A2:1 A#2 B2 & -:16 | \
            -:7 G3:5 -:3 B4:1 | _:2 -:1 A4:4 A2:2 -:6 D2:1 | _:5 -:11 | -:16 | -:7 F#2:4 - D#4:1 | \
            _:2 -:14 | -:7 G3:1 -:4 G#2:1 -:3 & -:16 | - | -:15 G4:1 | -:16 | - | - | - | -",
        )
        .with_tempo(&[tempo(112, 114)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "A4:4 E4 -:8 | -:2 E4 -:1 C5 B4 A4:2 -:1 G4:4 F#4:1 A4 | \
            G4:2 D4 -:1 B4 G4 - G4:2 D4:1 -:2 D4:1 D#4:2 | E4:4 C5:2 A4:1 - A4:3 E4:1 - E4 F4 - | \
            F#4:2 - D5 C5 B4 C5:1 -:3 E4:1 F#4 | G4:2 D5:3 -:1 G5 A5 B5:2 - D6:1 C6 - B5 | \
            _:7 -:1 A5:7 -:1 | -:4 C6:2 A5:1 - B5:2 G5:1 - A5 - F#5 -",
            "Am7/C Am7/C A7 Em7 | Am/C A7/C# | G E7/G# | Am | D C/E Bm/F# Bm/F# | G G#dim7 | \
            B7/A A | D7 D7 Em D/F#",
        )
        .with_left(
            "C3:4 G2+C4 A2:2 G2:4 D2:1 - | C2:6 -:2 C#2:6 -:2 | G2:8 G#2 | A2:4 E2 A2:8 | \
            D2:4 E2 F#2 D2 | G2:5 -:1 G4 A4 G#2+G#3:5 C5:1 - A2+B4 | _:7 -:1 A4:7 C#3:1 | \
            D3:3 -:1 D2:3 -:1 E2:4 F#2 & C4+E4:3 -:10 E2:2 -:1 | -:2 C4:1 -:9 C#3:2 - | -:16 | - | \
            - | G2:3 -:5 B4:2 - D5:1 -:3 | -:8 C#2:3 -:1 A#2 B2 C3 - | \
            -:4 C5:2 A4:1 - B4:2 G4:1 - A4 - F#4 - & -:16 | - | - | - | - | -:2 D4:3 -:11 | \
            -:10 A2:2 -:4 | -:16",
        )
        .with_tempo(&[
            tempo(0, 106),
            tempo(16, 111),
            tempo(32, 105),
            tempo(48, 99),
            tempo(64, 104),
            tempo(80, 93),
            tempo(96, 102),
        ]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "G5:3 -:13 | -:2 B4:3 E5:1 - F#5 G5:2 B5:4 A5:2 | G5:4 F#5:8 -:2 A5 | \
            G5 F#5:3 -:1 E#5:2 F#5:4 B5:2 G5 | F#5:3 E5:7 -:6 | E4:2 D4:4 E4:2 D4:4 E4:2 D4 | \
            F#4 E4:4 F#4:2 E4:4 F#4:2 E4 | G4 F#4:4 G4:2 F#4:3 G4:2 -:1 F#4:2",
            "G7 B/F# | Em Em G B7 | G/B D/F# D#dim7 D#dim7 | Gmaj7/B F#maj7 B/D# B/D# | \
            F#7 F#7 Em/G B/D# | D | Em/G Em/G Em/G E/G# | D/A",
        )
        .with_left(
            "G4:3 F#2:1 G2:5 -:3 D#2:4 | E2:8 G4:2 B4:4 A4:2 | B2+G4:4 F#4:8 F#2:4 | \
            B2 F#2 D#2+F#4 B4:2 G4 | F#4:4 E3:2 F#3 G3 - D#2:4 | D2 F#2 D2 A2:3 -:1 | \
            G2:4 B2 G2 G#2 | A2 D2 A2 F#2 & G2:2 F2:1 -:5 F#2:4 - | -:2 B3:3 E4:1 - F#4 -:8 | \
            -:4 F#2 D#2 -:2 A4 | G4:3 - E#4:2 -:4 F#2:1 -:3 | E2:2 B2 -:12 | -:16 | - | - & -:16 | \
            - | - | -:2 F#4:3 -:11 | -:3 E4:7 -:6 | -:16 | - | -",
        ),
        // Bars 33-42.
        Part::new(
            "Part 5",
            "A4:8 G4:3 -:5 | B5:2 A#5:4 B5:2 A#5:4 B5:2 A#5 | C#6 B5:4 C#6:2 B5:4 Bb5:2 G#5:1 - | \
            G#5:2 F#5:4 Ab5:2 F#5:4 G#5:2 F#5 | E5:4 D#5:1 -:3 D4:5 -:3 | \
            G4:2 D5:3 -:1 G5 A5 B5:2 - D6:1 C6 - B5 | _:7 -:1 A5:7 -:1 | \
            -:2 A4:1 B4 C5 - A4:2 B4:1 - G4:2 -:1 A4 - F#4 | - G4 -:2 D5 -:5 G4:1 D5 E6 A6:2 | \
            _:4 -:12",
            "A7 G | F# F#7/E | B/D# C#7/B Bm/F# Eb | C# Ab7/Gb F#/A# F#/A# | Em/B D#dim7 D7 D7 | \
            G G#dim7 | Bm7/A A | D7 D7 Eb7b9 F#m7b5/E | Gmaj7 D7 Em Em | Em",
        )
        .with_left(
            "G2:2 D3 G3 A3 B3:3 -:1 G2:4 | _:1 - A#4:4 B4:2 E3+A#4:4 C#3 | \
            D#3 B2 F#3+B4 Bb4:3 -:1 | C#3:4 F#2 A#2+F#4 F#2 | \
            B2+E4 D#4:1 - C2 C#2 D2+F#3+C4:4 F#2 | G2:5 -:1 G4 A4 G#2+G#3:5 C5:1 - A2+B4 | \
            _:7 -:1 A4:7 C#3:1 | D3:3 Bb4:1 -:2 D2 Eb2 E2:4 -:1 F#2 | \
            _ G2:2 -:1 D2+F#4+C5:2 -:6 B4:4 | _:3 -:13 & -:16 | F#2:5 -:7 B4:2 A#4 | \
            C#5 B4:4 C#5:2 -:4 Eb3:2 G#4:1 - | G#4:2 F#4:4 Ab4:2 -:4 G#4:2 F#4 | -:16 | \
            G2+B3+D4:3 -:5 B4:2 - D5:1 -:3 | -:4 E2:1 - D2 - C#2:3 -:1 A#2 B2 C3 - | -:16 | \
            -:13 B5:3 | _:4 -:12 & -:16 | B4:2 - F#3:4 -:8 | -:16 | -:15 A#2:1 | -:16 | \
            -:2 D4:3 -:11 | -:10 A2:2 -:4 | -:16 | -:13 E5:3 | _ -:13",
        )
        .with_tempo(&[
            tempo(16, 110),
            tempo(32, 104),
            tempo(48, 108),
            tempo(64, 102),
            tempo(80, 105),
            tempo(96, 97),
            tempo(112, 101),
            tempo(128, 99),
        ]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5"],
    drums_from: 0,
    about: "\"What can they tell me about love\": a man who has loved and lost answers those \
            who talk to him of love. G major.",
};

// Copyright: unclear (no death date was found for Guichandut); added at the owner's request.
// Written out from a piano performance (MIDI in 4/4). A second track of high notes plays
// with the tune.
pub(super) const MARIONETAS: Song = Song {
    name: "Marionetas",
    origin: "Juan José Guichandut, words by Armando Tagini, 1928",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 110.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:6 C4:2 D4 -:1 E4:2 F4:1 A4 - | C5:3 A4 Bb4:1 - C5:3 Bb4:2 A4:1 Bb4 - | \
            C5:2 G4:4 A4:1 Bb4:2 A4:3 G4:1 - F4 E4 | _:2 D4 - D4:1 E4:4 D4:1 - D4 - Db4 | \
            C4:2 -:14 | -:16 | D4:2 C4:3 -:11 | -:15 C4:1",
            "C C Dm Dm | F Dm7 | Gm Gm Bb/D C | C | F Dm7 | Dm7/A Bb7/Ab | Gm Gm C/G Dm7 | Dm7/C",
        )
        .with_left(
            "-:16 | F2:5 -:3 D2+F#4:4 - | G2 D3:6 -:5 C2:1 | _:4 C3:3 -:1 Gb2:8 | \
            F2:5 -:1 C3:3 -:1 E3:2 F3:1 - D3 A2 | _:8 Ab2:4 F3:1 - D3 - | G2:11 -:1 D3 C3 - C2 | \
            _:15 F2:1 & -:16 | -:4 F3:1 -:11 | -:7 Bb3:3 -:6 | -:16 | -:8 D3:2 -:5 F3:1 | \
            _:2 D3:4 C3:2 D3:3 E3:2 -:3 | -:4 D3:2 C3:3 -:1 E3:2 -:3 D3:1 | \
            _:2 C3 -:1 C3:2 -:1 D3:2 E3 F3 - & -:16 | - | - | - | - | - | -:8 D3:2 -:6 | \
            -:13 A3:2 -:1",
        ),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "-:3 C5:1 C5:3 -:1 D5:2 E5 F5 A5:1 C6 | _:2 A5:4 Bb5:1 C6:3 Bb5 A5:1 Bb5 C6 | \
            _:2 G5:3 -:1 A4 Bb4:3 A4:2 -:1 G4 F4 E4 | _:2 D4:3 E4:1 -:2 E4 D4 -:1 E4 F4 - | \
            A4:4 E4:2 A4 Bb4 A4 -:1 Bb4 C5 D5 | _:2 Bb4:3 -:1 C5 D5:3 C5:2 -:1 B4 D5 - | \
            C5:4 -:2 C4 D4:3 E4:2 D4:1 C4 - | D4:3 C4:2 -:1 C4:2 D4 -:1 E4:2 D4:1 C4 -",
            "C C Dm7 Dm7 | F Gm | C7/G C7 | Bb7 Bb7/Ab | A7 Dm7 | Bb Bb7/Ab | F Dm7/F | Gm C",
        )
        .with_left(
            "-:4 C4:3 -:1 D4:3 -:1 F4:2 A4 | - A4:4 Bb4:2 - Bb4:3 A4:1 Bb2+Bb4 G2 | \
            _:4 -:11 Bb2:1 | _:4 - Ab2:5 -:3 | A2:10 -:5 F4:1 | _:3 -:1 Bb3 -:2 F4+Ab4:3 -:6 | \
            F2:12 -:4 | G2:9 C2:7 & -:10 E4:2 -:3 F2:1 | _:5 -:2 D2:4 D3:3 -:1 C5 | \
            _:2 G4:3 -:11 | -:2 Ab3:1 -:5 B3:3 -:5 | C#4:7 -:9 | Bb2:5 -:3 Ab2+Ab3:4 - | \
            F4:3 -:13 | -:16 & -:15 C5:1 | _:2 - F3 -:1 C5:3 -:6 | -:16 | - | A2+G3:6 -:10 | \
            -:16 | -:2 A4:1 -:13 | -:16",
        )
        .with_tempo(&[tempo(64, 102), tempo(80, 106)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "F4:4 -:1 C5:2 -:1 C6:3 Bb5:2 A5:1 Bb5 A5 | _:6 -:1 G5:9 | \
            -:3 C5 -:1 C6:3 Bb5:2 -:1 Bb5 C6 Bb5 | _:7 A5:4 -:5 | \
            -:4 C4:3 C5 -:1 Bb4:2 A4:1 Bb4 A4 | _ G4:2 - D4 C5:3 Bb4:2 A4:1 Bb4 - A4 | \
            _ G4 -:3 C4:1 - C5:2 Bb4 - A4:1 G#4 - | A4:4 - G4:2 -:1 A4 -:2 G4:1 -",
            "F Dm7 | Gm Gm/D | C C7 | Gm7 Gm7 F/A C | F Gm/D | Gm Gm Bb/D F7 | C C7 | F F7/Eb",
        )
        .with_left(
            "C4:4 -:1 C4:2 -:1 D2:5 A4:1 Bb4 A4+D5 | _:6 -:1 G4+D5:9 | \
            -:3 C4 -:1 C3:5 -:1 Bb4 C5 Bb4 | _:7 A4:4 B2:1 C3:4 | F2:2 -:6 D2:5 -:2 G2:1 | \
            _:3 D3:8 -:2 Eb2:1 - C2 | _:3 C3:1 -:3 C3:4 E3:1 -:4 | \
            C4:5 -:3 Eb2+Eb3:4 - & F2+A3:3 -:5 C5:3 Bb4:2 - G2:1 | _:4 D3:8 -:3 C2:1 | \
            _:9 -:1 Bb4:2 -:3 F2:1 | _ C3:2 F3 - A3:1 -:2 Bb2:1 -:5 | -:16 | -:7 Bb3:4 -:5 | \
            -:16 | F2:4 F3:1 -:3 C#4:2 -:6 & -:16 | -:8 Bb3:3 G3:2 -:3 | -:7 C5:3 -:1 C2 -:4 | \
            - G3:3 -:9 | -:16 | - | - | -",
        )
        .with_tempo(&[tempo(0, 84), tempo(16, 113), tempo(112, 102)]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "F#4:2 - D4:1 -:3 D6:2 C6:1 - C6 - D6 C6 | _:2 -:6 Bb5:5 -:3 | \
            -:4 E6:3 E6:1 -:2 D6:1 - C6 - B5 C6 | _:2 -:6 A5:8 | -:4 C5:2 - C6 Bb5 A5 Bb5 | \
            A5 G5 Bb4 C5 C6 Bb5 A5 Bb5 | A5 G5 G4 C5 D5 E5 E5:1 D5 C5:2 | \
            F5:3 - C4:2 D4 -:1 E4:2 F4:1 A4 -",
            "Dm Dm7 | C/G Dm Gm Gm | C7 Dm7 | F Gm7/F F F/C | F C7 | C7 | C | F Dm",
        )
        .with_left(
            "D2+D3:10 C5+G5:1 - C5+G5 - D5+A5 G2+C5+G5 | _:3 D3:6 -:3 G3:1 -:3 | \
            C2:4 E5+Bb5:2 -:1 C3:4 E3:1 C5 - B4 C5 | _:2 -:1 F3:3 G3:2 A4:8 | F2 - | \
            C4:2 E4 -:4 C4:2 E4 G4:1 - Bb4 - | C4:2 E4 -:4 G4:3 -:1 E4+D5 -:3 | \
            F3:2 -:14 & -:8 D5+A5:2 -:6 | -:8 Bb4+D5:5 -:3 | -:4 C3:1 -:2 Bb5 -:1 D5:2 -:3 F2:1 | \
            _ C3:2 -:5 A3:1 - Bb2 B2 C3:4 | -:16 | - | - | - & -:16 | -:8 Bb3:1 -:7 | - E5:1 -:8 | \
            -:16 | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 105),
            tempo(16, 110),
            tempo(64, 105),
            tempo(80, 100),
            tempo(96, 107),
            tempo(112, 111),
        ]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "C5:3 A4 Bb4:1 - C5:3 Bb4:2 A4:1 Bb4 - | C5:2 G4:4 A4:1 Bb4:2 A4:3 G4:1 - F4 E4 | \
            _:2 D4 - D4:1 E4:4 D4:1 - D4 - Db4 | C4:2 -:14 | -:16 | D4:2 C4:3 -:11 | -:15 C4:1 | \
            _:3 C5:1 C5:3 -:1 D5:2 E5 F5 A5:1 C6",
            "F Dm7 | Gm Gm Bb/D C | C | F Dm7 | Dm7/A Bb7/Ab | Gm Gm C/G Dm7 | Dm7/C | F Dm7",
        )
        .with_left(
            "F2:5 -:3 D2+F#4:4 - | G2 D3:6 -:5 C2:1 | _:4 C3:3 -:1 Gb2:8 | \
            F2:5 -:1 C3:3 -:1 E3:2 F3:1 - D3 A2 | _:8 Ab2:4 F3:1 - D3 - | G2:11 -:1 D3 C3 - C2 | \
            _:15 F2:1 | _:2 - C4:3 -:1 D4:3 -:1 F4:2 A4 & -:4 F3:1 -:11 | -:7 Bb3:3 -:6 | -:16 | \
            -:8 D3:2 -:5 F3:1 | _:2 D3:4 C3:2 D3:3 E3:2 -:3 | -:4 D3:2 C3:3 -:1 E3:2 -:3 D3:1 | \
            _:2 C3 -:1 C3:2 -:1 D3:2 E3 F3 - | -:10 E4:2 -:3 F2+C5:1 & -:16 | - | - | - | - | \
            -:8 D3:2 -:6 | -:13 A3:2 -:1 | -:16",
        )
        .with_tempo(&[tempo(112, 86)]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "-:2 A5:4 Bb5:1 C6:3 Bb5 A5:1 Bb5 C6 | _:2 G5:3 -:1 A4 Bb4:3 A4:2 -:1 G4 F4 E4 | \
            _:2 D4:3 E4:1 -:2 E4 D4 -:1 E4 F4 - | A4:4 E4:2 A4 Bb4 A4 -:1 Bb4 C5 D5 | \
            _:2 Bb4:3 -:1 C5 D5:3 C5:2 -:1 B4 D5 - | C5:4 -:2 C4 D4:3 E4:2 D4:1 C4 - | \
            D4:3 C4:2 -:1 C4:2 D4 -:1 E4:2 D4:1 C4 - | F4:4 -:1 C5:2 -:1 C6:3 Bb5:2 A5:1 Bb5 A5",
            "Dm7/A Gm | C7/G C7 | Bb7 Bb7/Ab | A7 Dm7 | Bb Bb7/Ab | F Dm7/F | Gm C | F Dm7",
        )
        .with_left(
            "-:2 A4:4 Bb4:2 - Bb4:3 A4:1 Bb2+Bb4 G2 | _:4 -:11 Bb2:1 | _:4 - Ab2:5 -:3 | \
            A2:10 -:5 F4:1 | _:3 -:1 Bb3 -:2 F4+Ab4:3 -:6 | F2:12 -:4 | G2:9 C2:7 | \
            C4:4 -:1 C4:2 -:1 D2:5 A4:1 Bb4 G2+A4+D5 & -:4 F3:2 -:1 D2:4 D3:3 -:1 C5 | \
            _:2 G4:3 -:11 | -:2 Ab3:1 -:5 B3:3 -:5 | C#4:7 -:9 | Bb2:5 -:3 Ab2+Ab3:4 - | \
            F4:3 -:13 | -:16 | F2+A3:3 -:5 C5:3 Bb4:2 -:3 & -:7 C5:3 -:6 | -:16 | - | \
            A2+G3:6 -:10 | -:16 | -:2 A4:1 -:13 | -:16 | -",
        )
        .with_tempo(&[
            tempo(0, 112),
            tempo(48, 103),
            tempo(64, 114),
            tempo(96, 103),
            tempo(112, 86),
        ]),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "-:7 G5:9 | -:3 C5 -:1 C6:3 Bb5:2 -:1 Bb5 C6 Bb5 | _:7 A5:4 -:5 | \
            -:4 C4:3 C5 -:1 Bb4:2 A4:1 Bb4 A4 | _ G4:2 - D4 C5:3 Bb4:2 A4:1 Bb4 - A4 | \
            _ G4 -:3 C4:1 - C5:2 Bb4 - A4:1 G#4 - | A4:4 - G4:2 -:1 A4 -:2 G4:1 - | \
            F#4:2 - D4:1 -:3 D6:2 C6:1 - C6 - D6 C6",
            "Gm/D | C C7 | Gm7 Gm7 F/A C | F Gm/D | Gm Gm Bb/D F7 | C C7 | F F7/Eb | Dm Dm7",
        )
        .with_left(
            "-:4 D3:8 -:3 C2:1 | _:9 -:1 Bb4:2 -:1 Bb4 C5 Bb4 | _:7 A4:4 B2:1 C3:4 | \
            F2:2 -:6 D2:5 -:2 G2:1 | _:3 D3:8 -:2 Eb2:1 - C2 | _:3 C3:1 -:3 C3:4 E3:1 -:4 | \
            C4:5 -:3 Eb2+Eb3:4 - | D2+D3:10 C5+G5:1 - C5+G5 - D5+A5 G2+C5+G5 & -:7 G4+D5:9 | \
            -:3 C4 -:1 C3:5 -:3 F2:1 | _ C3:2 F3 - A3:1 -:2 Bb2:1 -:5 | -:16 | -:7 Bb3:4 -:5 | \
            -:16 | F2:4 F3:1 -:3 C#4:2 -:6 | -:8 D5+A5:2 -:6 & -:8 Bb3:3 G3:2 -:3 | \
            -:7 C5:3 -:1 C2 -:4 | - G3:3 -:9 | -:16 | - | - | - | -",
        )
        .with_tempo(&[tempo(0, 111), tempo(96, 97), tempo(112, 111)]),
        // Bars 57-64.
        Part::new(
            "Part 8",
            "-:8 Bb5:5 -:3 | -:4 E6:3 E6:1 -:2 D6:1 - C6 - B5 C6 | _:2 -:6 A5:8 | \
            -:4 C5:2 - C6 Bb5 A5 Bb5 | A5 G5 Bb4 C5 C6 Bb5 A5 Bb5 | \
            A5 G5:1 - E4 - C5 -:2 D5:1 E5 D5:3 C5:1 - | - F5 -:3 C6:1 -:6 D4:1 D5 G5:2 | \
            _:5 -:11",
            "Bb Gm | C7 Dm7 | F Gm7/F F F/C | F C7 | C7 | F7/C Dm7/C | F F Dm F | F",
        )
        .with_left(
            "-:3 D3:6 -:3 G3:1 -:3 | C2:4 E5+Bb5:2 -:1 C3:4 E3:1 C5 - B4 C5 | \
            _:2 -:1 F3:3 G3:2 A4:8 | F2 - | C4:2 E4 -:4 C4:2 E4 G4:1 - Bb4 - | \
            C4:4 -:3 Eb2:1 - C2:5 -:2 | -:1 F2:3 -:1 C2+Bb5 -:5 F3 | _:4 -:12 & -:8 Bb4+D5:5 -:3 | \
            -:4 C3:1 -:2 Bb5 -:1 D5:2 -:3 F2:1 | _ C3:2 -:5 A3:1 - Bb2 B2 C3:4 | -:16 | - | \
            -:13 C3:1 -:2 | -:1 C5 - C#2 -:8 A3+C4:4 | _ -:12 & -:8 Bb3:1 -:7 | - E5:1 -:8 | \
            -:16 | - | - | - | -:13 A4:3 | _:5 -:11",
        )
        .with_tempo(&[tempo(48, 103), tempo(64, 91), tempo(80, 96), tempo(96, 113)]),
    ],
    form: &[
        "Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8",
    ],
    drums_from: 0,
    about: "Tagini's words remember the puppet theatre of childhood. F major, with turns to D \
            minor and to C major.",
};

// Copyright: Canaro died in 1964; not public domain; added at the owner's request.
// Written out from a piano performance (MIDI in 4/4).
pub(super) const TIEMPOS_VIEJOS: Song = Song {
    name: "Tiempos viejos",
    origin: "Francisco Canaro, words by Manuel Romero, 1926",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 85.8,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:8 A4:3 A4:2 D5:1 C#5 - | D5:5 C5:1 B4 C5:6 Bb4:1 A4 C5 | \
            _:4 Bb4 G4:2 G4:3 C5:1 - B4 | C5:2 Bb4:4 A4:2 Bb4:4 A4:1 Bb4 A4 G4 | \
            Bb4:5 -:2 F4 -:1 F4:2 -:1 Bb4 A4 Bb4 | _ A4:3 -:1 G#4:2 A4:5 G4:1 F4 - A4 | \
            _:3 -:1 G4:3 E4:2 - E4 A4:1 G#4 A4 | _:2 G4:4 F#4:2 G4:4 F4:1 G4 F4 G4",
            "A | Cm/Eb D7 | Gm G/D | C7 Edim7 | F Dm | Dm Dm A/E D/F# | Gm A7 | A7",
        )
        .with_left("-:16 | Eb2:6 -:1 F#4:6 -:2 G2:1 | _ - D3:10 -:4 | E4:9 -:7 | F2:2 C3 - G3 -:7 D2:1 | \
            _:6 -:1 E2:4 -:1 F#2:4 | Bb3+D4 -:11 A2:1 | _:11 -:3 E4:1 D2 & -:16 | \
            G4:5 -:3 D2:5 -:3 | -:4 Bb3:2 - G3:4 - | C2:8 E2:3 -:5 | -:3 A4:4 A3 - D4:1 | \
            -:3 D3:4 -:8 G2:1 | _:10 -:6 | -:4 E3 G3 - & -:16 | - | - | - | -:3 F3:2 -:11 | -:16 | \
            - | -")
        .with_tempo(&[tempo(16, 101), tempo(32, 102)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "-:3 F4:2 C5:1 B4 A4:2 Bb4:1 A4:3 D5:1 C#5 Eb5 | _:4 -:1 D5 C5:2 Eb5 D5:1 Eb5 D5:2 C5 | \
            Bb4:4 G4:3 -:1 G4:2 G4:1 -:2 C5:1 B4 D5 | _ C5:4 B4:1 - D5:3 C5:1 D5 C5:2 Bb4 | \
            A4 G5:1 G#5 A5 - A4 F4:2 -:1 F4 - Bb4 A4 - C5 | _:2 Bb4:3 A4:2 -:1 G4:4 F4:2 G4:1 A4 | \
            _:4 -:2 F4:1 G4:6 F4:1 E4 A4 | _ - E4:4 F4:2 G4 F4:1 G4 F4:2 E4",
            "Dm7 A7 | Am7b5 D7b9 | Gm A7/G C7/Bb C7/Bb | C7 C7 D7/F# C7 | F Dm/F | Em7b5 A7/E | \
            A A7/E | A7b9 C#m7b5/B",
        )
        .with_left("-:1 A2:2 D3:3 Bb4:1 -:8 A2:1 | _:7 -:1 D2:4 - | G2:2 D3 G3:3 -:1 Bb3:4 -:3 C2:1 | \
            _:7 Gb2:3 -:6 | F2:13 -:2 E2:1 | _:15 A2:1 | _:7 E2:8 A2:1 | \
            _:3 Bb2:4 -:1 B2:4 C#3 & -:15 G4:1 | _:4 -:12 | -:6 A3:2 -:7 Bb4:1 | _:2 -:14 | \
            -:7 A2:4 - D4:1 | _:2 -:13 E4:1 | _:4 -:12 | -:16 & -:16 | - | -:15 E4:1 | _ -:15 | \
            -:16 | - | -:1 E4:5 -:10 | -:16")
        .with_tempo(&[tempo(48, 108), tempo(64, 100), tempo(80, 104)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "D4:1 -:3 A4:1 -:3 D5:2 A4:3 -:1 Bb4 - | A4:5 G4:1 F#4 G4:5 -:1 Bb4 A4:2 | \
            G4:4 F4:7 A4:2 G4:1 -:2 | F4 E4:4 D#4:2 E4:5 G4:1 F4:2 | E4:4 D4:3 - A5 Bb5:1 -:2 | \
            A5:3 G5:2 F#5 -:1 G5:4 -:1 Bb5 A5:2 | G5:3 F5:6 A5:3 G5:1 -:3 | \
            F5:4 E5:1 F5 E5 D#5 E5:2 G5:3 F5:2 E5:1",
            "Dm A Dm Dm | Em7 A7 | Dm F | E7 A7 | Dm F | Em7b5 A7 | Dm F | E7 A7",
        )
        .with_left("D3:4 A2 A4:2 -:6 | E2:7 A2:8 -:1 | D2:2 A2 D3 E3 -:8 | E2 A2 | D2:2 A2 D3 E3 -:8 | \
            E3:5 -:2 A2:6 -:2 D2:1 | _ - A2:2 D3 -:1 F3:4 -:5 | \
            E2:8 A2:4 - & -:4 C#4:1 -:3 D2+F4:1 -:7 | D4:4 - C#4:6 -:2 | -:7 F3:4 -:5 | \
            G#3:3 -:13 | -:7 F3:5 -:4 | D5:3 -:4 G3 -:5 | - E3:2 -:9 | G#4+D5:4 -:12 & -:16 | - | \
            - | D4:2 -:14 | -:16 | Bb4:2 -:6 C#5:5 -:3 | -:16 | -"),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "-:4 D5 -:8 | -:15 D4:1 | _:4 C4:9 -:1 C4 D4 | _:2 C4:4 -:2 C4:4 -:2 C4 | \
            - G4:1 G#4 A4:2 -:4 Bb5:3 A5:1 -:2 | G5:4 E5:1 - F5 C#5 G5:4 -:1 A5 G5 F5 | \
            _:4 D5:8 G5:2 F5:1 - | E5:5 Db5:1 D5 E5:4 -:2 F5:1 E5 -",
            "Dm Dm Fmaj7 E7 | C7 | Dm7 Dm7 Am G/B | D7/C Gb7 | F Bbmaj7 | Em7b5 A7 | Dm Dm7/C | \
            E/B Bbdim7 A7 A7",
        )
        .with_left("D2:2 F2 A2:1 D3:2 E3:1 F3:3 - D3:2 | C2:11 -:2 A3 -:1 | \
            F2:3 -:1 F3:2 G3 A3:4 -:1 B3:2 -:1 | C2:5 B3:1 -:2 Gb2:8 | F2:2 -:4 A3:5 - | \
            E2:7 -:1 A2:8 | D2:4 D3:5 -:7 | B2:5 -:2 G4:4 -:1 C#2:4 & -:1 E2:2 -:7 E3:4 -:2 | \
            C3:5 E3:2 Bb3:5 -:2 Bb3:1 - | -:2 C3 -:12 | -:8 E3:4 Bb3:2 - | A3:1 -:7 A2:3 -:5 | \
            D5:4 -:12 | -:8 C3 | G#4:4 Bb2:3 -:1 A2:4 - & -:16 | -:6 G3:1 - C3:8 | -:16 | - | - | \
            Bb4:2 -:14 | -:16 | -"),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "D5:1 -:2 A6:4 -:1 A4:3 A4:2 D5:1 C#5 - | D5:5 C5:1 B4 C5:6 Bb4:1 A4 C5 | \
            _:4 Bb4 G4:2 G4:3 C5:1 - B4 | C5:2 Bb4:4 A4:2 Bb4:4 A4:1 Bb4 A4 G4 | \
            Bb4:5 -:2 F4 -:1 F4:2 -:1 Bb4 A4 Bb4 | _ A4:3 -:1 G#4:2 A4:5 G4:1 F4 - A4 | \
            _:3 -:1 G4:3 E4:2 - E4 A4:1 G#4 A4 | _:2 G4:4 F#4:2 G4:4 F4:1 G4 F4 G4",
            "Dm A | Cm/Eb D7 | Gm G/D | C7 Edim7 | F Dm | Dm Dm A/E D/F# | Gm A7 | A7",
        )
        .with_left("D2:4 A5 -:8 | Eb2:6 -:1 F#4:6 -:2 G2:1 | _ - D3:10 -:4 | E4:9 -:7 | \
            F2:2 C3 - G3 -:7 D2:1 | _:6 -:1 E2:4 -:1 F#2:4 | Bb3+D4 -:11 A2:1 | \
            _:11 -:3 E4:1 D2 & -:16 | G4:5 -:3 D2:5 -:3 | -:4 Bb3:2 - G3:4 - | C2:8 E2:3 -:5 | \
            -:3 A4:4 A3 - D4:1 | -:3 D3:4 -:8 G2:1 | _:10 -:6 | -:4 E3 G3 - & -:16 | - | - | - | \
            -:3 F3:2 -:11 | -:16 | - | -")
        .with_tempo(&[tempo(0, 94), tempo(16, 104)]),
        // Bars 41-50.
        Part::new(
            "Part 6",
            "-:3 F4:2 C5:1 B4 A4:2 Bb4:1 A4:3 D5:1 C#5 Eb5 | _:4 -:1 D5 C5:2 Eb5 D5:1 Eb5 D5:2 C5 | \
            Bb4:4 G4:3 -:1 G4:2 G4:1 -:2 C5:1 B4 D5 | _ C5:4 B4:1 - D5:3 C5:1 D5 C5:2 Bb4 | \
            A4 G5:1 G#5 A5 - A4 F4:2 -:1 F4 - Bb4 A4 - C5 | _:2 Bb4:3 A4:2 -:1 G4:4 F4:2 G4:1 A4 | \
            _:4 -:2 F4:1 G4:6 F4:1 E4 A4 | _ - E4:4 F4:2 G4 F4:1 G4 F4:2 E4 | \
            D4:1 -:3 A4:1 -:5 D4:1 F5 E6:4 | _:3 -:13",
            "Dm7 A7 | Am7b5 D7b9 | Gm A7/G C7/Bb C7/Bb | C7 C7 D7/F# C7 | F Dm/F | Em7b5 A7/E | \
            A7 | A7b9 A7b9 G7/B A7/C# | Dm A7 Dm Dm6 | Dm6",
        )
        .with_left("-:1 A2:2 D3:3 Bb4:1 -:8 A2:1 | _:7 -:1 D2:4 - | G2:2 D3 G3:3 -:1 Bb3:4 -:3 C2:1 | \
            _:7 Gb2:3 -:6 | F2:13 -:2 E2:1 | _:15 E4:1 | _:4 - E2:2 - C#2:4 | \
            -:3 Bb2:4 -:1 B2:4 C#3 | D3 A2 -:3 F4+A4:5 | _:3 -:13 & -:15 G4:1 | _:4 -:12 | \
            -:6 A3:2 -:7 Bb4:1 | _:2 -:14 | -:7 A2:4 - D4:1 | _:2 -:13 A2:1 | _:3 G2:5 -:7 A2:1 | \
            _ -:15 | -:4 C#4:1 -:7 D5+B5:4 | _:3 -:13 & -:16 | - | -:15 E4:1 | _ -:15 | -:16 | - | \
            -:1 E4:5 -:10 | -:16 | - | -")
        .with_tempo(&[tempo(64, 84), tempo(80, 94)]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6"],
    drums_from: 0,
    about: "\"¿Te acordás, hermano? ¡Qué tiempos aquellos!\": two old friends remember the \
            Buenos Aires of 1900 and the blonde Mireya, taken from another man at Hansen's. D \
            minor.",
};

// Copyright: public domain in Argentina, where a work is protected for 70 years after its author's
// death: Discépolo, who wrote words and music, died in 1951.
// Written out from a piano performance (MIDI in 4/4). The file has the same notes three
// times over, on three tracks; one is enough.
pub(super) const YIRA_YIRA: Song = Song {
    name: "Yira, yira",
    origin: "Enrique Santos Discépolo, 1930",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: B,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 111.1,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-12.
        Part::new(
            "Verse 1a",
            "-:14 E4:1 E#4 | F#4 F#4 F#4 F#4 F#4:5 E4:1 - G4 F#4:2 D4:1 - | \
            -:2 F#4:1 - F#4:3 -:1 E4 - G4 - F#4:2 D4:1 - | -:3 F#4:1 G4 G#4 A4:6 E4:1 G4 F#4:2 | \
            _:13 -:3 | -:1 A5 A4 -:2 A4:1 - A4 B4:3 A4:1 B4 A4 G4:2 | F#4:8 D4 | \
            -:2 F#4 -:1 F#4 - F#4 G4:2 F#4:1 G4 F#4 - E4 - | D4:5 D4:1 - C#4 C4:2 -:6 | \
            -:2 D4 -:1 D4 - C#4 -:2 C#4:4 D4:1 - | F#4:6 -:2 F#4 E4:1 F#4 E4:2 D#4:1 - | \
            E4:4 - G4 F#4:1 G4 F#4 E4",
            "Bm | Bm F#7 | Bm F#7 | Bm A7 | D/F# D/F# F#7 D | A B7/A | D7 D/F# | F#7 | Bm D7 | G | \
            D B7/D# | Em A7",
        )
        .with_left(
            "-:16 | -:12 B2+B3:2 - | -:4 C#2+C#3+C#4 -:2 D2:4 -:2 | -:6 A2:2 E3:1 -:5 D3:2 | \
            F#3:4 G3:1 G#3 A3:2 F#3 E3 D3:3 -:1 | -:8 A2:7 -:1 | D2:3 - C3+D3:1 E3+F3 A3:5 -:3 | \
            E3:8 C#3:3 -:5 | B2 -:3 D2:5 -:1 E#2+D3 F#2 | G2:10 -:6 | D2:4 D3:1 - A3 - D#2:5 -:3 | \
            G3+B3 -:1 E2 E#2 G2:2 A2:8 & -:16 | - | -:4 A#3:2 -:6 D3+B3:2 - | -:16 | -:14 C#3:2 | \
            _ -:14 | -:8 F#3:4 F#2 | _:10 -:6 | -:2 F#3:3 -:5 A3:1 - F#3:2 - | \
            - G3:1 -:3 G3:1 - B3:2 -:6 | A3:3 -:5 A3:1 -:3 D#3:1 -:3 | \
            E2:2 -:5 G#2:1 C#4:3 -:5 & -:16 | - | - | - | - | - | - | \
            -:2 A#3:1 -:5 A#3+C#4:1 -:7 | -:16 | - | - | D#4:1 -:15",
        )
        .with_tempo(&[
            tempo(16, 110),
            tempo(32, 111),
            tempo(48, 117),
            tempo(64, 110),
            tempo(80, 116),
            tempo(96, 108),
            tempo(112, 110),
            tempo(128, 114),
            tempo(144, 113),
            tempo(176, 114),
        ]),
        // Bars 13-21.
        Part::new(
            "Verse 1b",
            "F#4:4 - F#4:1 -:3 F#6:1 -:3 | -:2 F#5:1 - F#5 - F#5 - F#5:2 E5:1 -:3 G5:1 - | \
            F#5:2 D5:1 -:3 F#5:1 - F#5:4 E5:2 G5:1 - | F#5:2 D5:1 -:3 F#5:1 G#5 A5:4 E5:2 G5 | \
            F#5:1 -:3 B4:1 -:3 E4:1 -:3 D#4:1 -:3 | -:1 E5 -:3 E4:1 - E4 E4:2 B4:4 G4:1 - | \
            G4:2 F#4:3 -:1 F#4:2 B4:4 F#4:1 G4 F#4 E#4 | F#4:2 E4 - F#4:1 - G4:4 F#4:1 - C#5 - | \
            B4:3 -:1 A#4:2 - A4:1 -:3 A5:1 -:3",
            "D7 D7/C | Bm | Bm F#7/C# | Bm A7 | Bm/D Bm/D D7/C B7 | Em | Bm7 G/B | F#7 | \
            Bm F# A7 A7",
        )
        .with_left(
            "D2:3 - C3:6 F#5:1 -:3 | B2:2 -:6 B4+D5:2 F#2:1 -:5 | B2:4 -:2 B2:1 C3 C#3:4 - | \
            B2+B4:2 -:5 G5:1 A2:4 - | D3:7 -:1 C3:3 -:1 B2+A3 -:3 | E2:4 - G3:1 -:7 | B2:13 -:3 | \
            F#2:12 -:4 | F#4:3 -:1 A#2+A#3+D4 -:3 A2+A3:2 - A4:1 -:3 & -:7 G3:5 -:4 | -:16 | \
            B4:3 -:5 A#4+C#5:3 -:5 | -:8 G3:3 -:5 | D2:4 - A#3:1 -:7 | -:1 E4 -:14 | \
            -:4 A3:3 -:1 G3:5 -:3 | -:8 E3:1 - F#3:3 - | \
            D4:2 -:6 C#4+G4:1 -:7 & -:7 A#3:1 D4 -:7 | -:16 | - | - | - | - | - | - | B2:1 -:15",
        )
        .with_tempo(&[
            tempo(0, 112),
            tempo(48, 115),
            tempo(64, 113),
            tempo(80, 114),
            tempo(96, 112),
            tempo(128, 110),
        ]),
        // Bars 22-37.
        Part::new(
            "Refrain 1",
            "-:2 D4 -:1 D4 - D4 D4:2 E4:3 F#4:2 -:1 | A4:4 C4:1 E4 F#4 A4 A5 - F#4:2 E4 C4:1 - | \
            -:2 B4 -:1 B4 - B4 B4:2 G4:1 -:3 D5:1 - | A4 -:3 B5:1 A5 F#5 D5 B4 A4 F#4 D4 -:4 | \
            -:2 C#5 - C#5:1 C#5 C#5:5 B4:1 A4 - | B4:2 F#4:1 -:4 B5:1 - A5:5 F#5:1 - | \
            -:4 A4:2 E4:1 - D4 D#4 E4:2 A5 E5:1 - | -:4 A5:2 F#5:1 - F#4 - A6 -:5 | -:16 | \
            -:4 A4:1 -:3 A5:1 -:3 A6:1 -:3 | -:2 B4 -:1 B4 - B4 B4:2 G4:1 -:3 D5:2 | \
            A4:1 -:3 B5:2 A5:1 - D4 - B4:2 A4:1 -:3 | -:1 E5 E4 -:3 E4:1 E4 E4 - F#4:4 G4:1 - | \
            G4:4 F#4:3 -:7 G4:1 - | F#4:4 E4:2 -:8 E4:2 | D4:4 C#4:2 -:5 F#6:1 F#5 -:3",
            "D | D7 D7 D Em | G | D Bm7 | A7 A7/E | Bm D | A7 A7 Em7 A/C# | D B7 | \
            D7 D7 Em7 F#7/E | D7 | G | Bm7 Em7 | Em | D7 G/B | Em7 A | Bm F#7",
        )
        .in_key(D, Major)
        .with_left(
            "D2:10 -:6 | E4:4 -:12 | G2:9 -:7 | D2+D3:3 -:9 B3:1 A3 F#3 D3 | A2:8 G4:5 -:3 | \
            D4 -:4 D5:1 - D5:5 -:2 | G2:1 G#2 A2:2 C#4 -:6 C#5:2 - | \
            D2 D3:1 - D5:2 -:6 A2:3 B2:1 | C3 C#3 D3 - D3 -:2 D3:1 D3 D3:2 E3:3 F#3:1 - | \
            D2:13 -:1 F2 F#2 | G2:16 | D2:3 -:1 D5:2 -:4 D4+E4:1 -:3 D3:1 D#3 | \
            E3:7 B3:1 E2:3 -:5 | D2:4 D3+A3:2 C3 B2:5 -:3 | G#3:5 -:1 G2 G#2 A2:3 -:5 | \
            D2:3 - B3:2 F#2+E3:4 - & D2:2 -:14 | D2+C4:2 -:14 | -:2 D4+G4 -:1 D4 - D4 D4:2 -:6 | \
            F#4:1 -:15 | -:6 G4:1 G4 E2:4 - | D2:2 -:7 F#4:2 -:5 | -:16 | -:14 A#2:1 - | -:16 | \
            F#3+A3:1 -:3 C4+E4+F#4:1 -:3 C5+E5+F#5:1 -:3 C6+E6+F#6:1 -:3 | \
            -:2 G4:3 D4:1 - D4 D4:2 -:4 F#4:1 - | -:4 F#5:1 -:11 | -:6 B3:1 - B3 -:7 | \
            A3+D4:3 -:9 B3:2 - | E2+D4:4 -:6 A3:4 -:2 | -:8 A#3:1 -:7 & -:16 | - | -:8 G3:1 -:7 | \
            -:16 | - | E4:1 -:15 | -:16 | - | - | - | -:2 D4 -:12 | -:16 | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 105),
            tempo(16, 107),
            tempo(48, 112),
            tempo(64, 108),
            tempo(80, 118),
            tempo(96, 113),
            tempo(112, 115),
            tempo(128, 105),
            tempo(144, 112),
            tempo(160, 117),
            tempo(176, 114),
            tempo(192, 113),
            tempo(208, 105),
            tempo(224, 110),
            tempo(240, 103),
        ]),
        // Bars 38-48.
        Part::new(
            "Verse 2a",
            "D4:2 E4:1 E#4 F#4 F#4 F#4 F#4 F#4 - E4 -:3 G4:1 - | \
            F#4:2 D4:1 -:3 F#4:1 - F#4:4 E4:2 G4:1 - | F#4:2 D4:1 -:3 F#4:1 G#4 A4:4 E4:2 G4 | \
            F#4:16 | -:2 A4:1 - G#4 A4 - A4 B4:3 -:1 A4 - G4 - | F#4:8 D4:6 -:2 | \
            - F#4:1 -:2 F#4:1 F#4 - G4:2 F#4 - E4 | D4:4 -:8 D4:2 C4:1 - | \
            -:2 D4 -:1 D4 - C#4 -:2 C#4:4 D4:1 - | F#4:6 -:2 F#4:4 E4:2 D#4:1 - | \
            E4:4 - G4 F#4:1 G4 F#4 E#4",
            "Bm F#7 | Bm F#7 | Bm A7 | Bm7 F#7/E | A A7 | Bm D | F# F#7 | Bm7 D7 | G | D B7/D# | \
            Em A7",
        )
        .with_left(
            "B2+G#3+B3:3 -:13 | B3:2 -:6 F#2:8 | B3:2 -:5 G4:1 A2+G3:4 - | \
            D2:2 - B3 A3 F#3 E3:4 -:1 C#3 | A2:11 -:5 | D2:2 D3:1 - F#3:2 B3 A3:3 -:1 F#3 -:3 | \
            F#2:11 -:5 | B3:3 -:1 A2 B2 C3 C#3 D3:4 -:2 F#2 | G2:15 -:1 | \
            D2+A3:4 D3:1 -:3 D#2:5 -:3 | B3 -:1 E2 E#2 G2 G#2 A2:3 -:5 & -:16 | B2:1 -:7 C#4:4 - | \
            B2:1 -:15 | -:13 D3:1 -:2 | - C#4+E4:1 -:5 E3:3 -:5 | -:16 | \
            -:2 A#3+C#4 -:4 E3:7 -:1 | B2 -:9 F#3:1 -:5 | -:2 G3 -:1 G3 -:2 B3 -:6 | \
            -:8 A3:2 - D#3:1 -:3 | G3:2 -:6 C#4:2 -:6 & -:16 | -:8 A#3:2 -:6 | -:16 | - | - | - | \
            -:8 A#3+C#4:2 -:6 | -:16 | - | - | E2:1 -:15",
        )
        .with_tempo(&[
            tempo(0, 109),
            tempo(16, 112),
            tempo(32, 110),
            tempo(48, 119),
            tempo(64, 115),
            tempo(80, 117),
            tempo(96, 119),
            tempo(112, 117),
            tempo(128, 115),
            tempo(144, 114),
            tempo(160, 119),
        ]),
        // Bars 49-57.
        Part::new(
            "Verse 2b",
            "F#4:1 -:3 F#6:1 -:3 F#4:4 D4:1 -:3 | -:2 F#5:1 - F#5 - F#5 - F#5:2 E5:1 -:3 G5:1 - | \
            F#5:2 D5:1 -:3 F#5:1 - F#5:3 -:1 E5:2 G5:1 - | \
            F#5:2 D5:1 -:2 F#5:1 G5 G#5 A5:4 E5:2 G5 | F#5:12 -:4 | \
            -:1 E5 E4 -:2 E4:1 E4 - E4:2 B4:3 -:1 G4 - | G4:2 F#4 - F#4 B4:4 F#4:1 G4 F#4 E#4 | \
            F#4 - E4:4 F#4:1 - G4:4 A#4:2 C#5:1 - | B4:2 - A#4:1 -:3 A4:1 -:2 A6:1 A5 -:3",
            "D D7/C | Bm | Bm F#7 | Bm A7 | Bm B7/A | Em | Bm Bm7 | F#7/C# F#7 | Bm F# A A",
        )
        .with_left(
            "D2+D3:4 F#5:1 -:3 C3+G3+A#3:4 - | B2+G#3:1 - D5 - B4+D5 - D5 - D5:2 -:6 | \
            B4:2 -:6 C#5:3 -:5 | B2:2 C#3:1 -:5 A2:2 E3:1 -:5 | D3:2 F#3 B3 A#3 A3:4 D#3:1 -:3 | \
            E2:10 -:6 | B2:4 B3:1 -:3 B2+A3:4 - | C#2+C#3 - F#2 E3:1 -:3 | \
            F#4 -:1 A#2+A#3+D4 -:3 A2 -:5 & E#4:1 -:15 | -:8 F#4+B4:1 -:7 | \
            B2:1 -:7 F#2+A#4:1 -:7 | B4:1 -:15 | -:16 | - | - | B3:1 -:15 | \
            D4:2 -:6 A3:2 -:6 & -:16 | - | - | - | - | - | - | - | B2:1 -:7 C#4:1 -:7",
        )
        .with_tempo(&[
            tempo(0, 112),
            tempo(16, 109),
            tempo(32, 113),
            tempo(48, 120),
            tempo(64, 117),
            tempo(80, 111),
            tempo(96, 114),
            tempo(112, 113),
            tempo(128, 105),
        ]),
        // Bars 58-65.
        Part::new(
            "Refrain 2a",
            "-:2 D4 -:1 D4 - D4 D4:2 E4:3 F#4 | A4:4 C4:1 E4 F#4 A4 A5 - F#4:2 E4 C4:1 - | \
            -:2 B4 -:1 B4 - B4 B4:2 G4:1 -:3 D5:1 - | A4 -:3 B5:1 A5 F#5 D5 B4 A4 F#4 D4 -:4 | \
            -:2 C#6:1 -:3 C#5:1 C#5 C#5:3 -:1 B4 C#5 B4 A#4 | \
            B4:2 F#4:1 -:3 B5:2 F#4:1 - B6:2 F#6:1 -:3 | -:2 A4 E4:1 -:5 A5:2 E5:1 -:3 | \
            -:2 F#5 D5:1 - F#4 - A5:4 -",
            "D D Em7 F#7 | D7 D7 D Em | G | D Bm7 | A7 Em | Bm7 G | A7 A7 A Em7 | D A",
        )
        .in_key(D, Major)
        .with_left(
            "D2:9 -:7 | C3:2 E3+F#3:1 A3 -:12 | G2:9 -:7 | D3:2 -:10 B3:1 A3 F#3 D3 | \
            A2:7 E4+G4:1 G4:4 - | D2+D3+D4+A4:2 -:4 D5+F#5:1 -:7 G2:1 G#2 | \
            A2:2 C#4 -:1 E3 F3+G3 G#3 A3:2 C#5 - D2 | _:1 - A4:2 -:8 A2:4 & -:16 | - | \
            -:2 D4+G4:1 -:2 D4:1 - D4 G3+D4 -:7 | D2+F#4:1 -:15 | -:2 C#5:1 -:5 E2:3 -:5 | -:16 | \
            - | D3:1 -:15 & -:16 | - | - | - | -:8 E4:2 -:6 | -:16 | - | -",
        )
        .with_tempo(&[
            tempo(0, 108),
            tempo(16, 110),
            tempo(32, 115),
            tempo(48, 119),
            tempo(64, 117),
            tempo(80, 116),
            tempo(96, 114),
            tempo(112, 119),
        ]),
        // Bars 66-74.
        Part::new(
            "Refrain 2b",
            "-:1 D5 D4 -:2 D4:1 - D4 D4:2 E4:3 F#4 | A4:2 -:3 E4:1 C5 F#5 A5:2 F#5 E5 C5:1 - | \
            -:2 B4 -:1 B4 B4 - B4:2 G4:1 -:3 D5:2 | A4:1 -:3 B5:2 A#5:1 -:3 B4:2 A4 F#4:1 - | \
            - E5 E4 -:2 E4:1 - E4 E4:3 -:1 F#4:2 G4:1 - | \
            G4:3 -:1 F#4:3 -:2 F#4:1 F#4 -:3 F#4:1 - | F#4:4 E4:1 -:3 A#6:1 B5 D5 F4 -:4 | \
            D5:1 -:3 A5:1 -:5 D4:1 F#4 B5 E6 -:2 | -:16",
            "D D Em7 F#7 | D7 D7 D Em | G | D Bm Bm7 Bm7 | Em F#7 | D B7 | Em7 A7 | D A Bm7 Bm7 | \
            G",
        )
        .in_key(D, Major)
        .with_left(
            "D3:9 -:7 | F#4:2 -:3 C4:2 E5:1 -:8 | G2:9 -:5 F#4:1 - | \
            D2 -:3 D5+F#5:2 - A3+Bb3:1 - D4+F#4 -:5 | E2:7 -:5 G#2:1 -:3 | \
            D2+D3:4 -:2 A2:1 A#2 B2:7 -:1 | \
            E2:7 -:1 A2+D6+F6+G6+A6 E5+F5+G5 G4+A4+B4+C5 C4+D4+E4 A2:2 C3 | \
            D3 D2 A2+C#5:1 -:7 A4:4 | _:3 -:13 & -:16 | D2+D3+C4+E4:1 -:5 F#4+A4:1 -:9 | \
            -:2 D4+G4 -:1 D4 D4 - D4:2 -:6 | -:4 B4:1 -:11 | F2:1 -:15 | D4:3 -:7 A3:4 -:2 | \
            G#3+D4:4 -:8 G3+A3+B3:1 B2 - C#3 | F#4+A4 -:11 B4:4 | _:2 -:14 & -:16 | - | \
            -:8 G3:1 -:7 | -:16 | - | A3:2 -:8 D#4:1 -:5 | -:16 | -:12 F#5:1 -:3 | -:16 & -:16 | \
            - | - | - | - | F#4:1 -:15 | -:16 | - | -",
        )
        .with_tempo(&[
            tempo(0, 115),
            tempo(16, 113),
            tempo(32, 117),
            tempo(48, 109),
            tempo(80, 101),
            tempo(96, 103),
            tempo(112, 111),
        ]),
    ],
    form: &[
        "Verse 1a",
        "Verse 1b",
        "Refrain 1",
        "Verse 2a",
        "Verse 2b",
        "Refrain 2a",
        "Refrain 2b",
    ],
    drums_from: 0,
    about: "Discépolo's bitterest song, words and music his own: \"Verás que todo es mentira, \
            verás que nada es amor, que al mundo nada le importa\". Gardel recorded it in \
            1930. B minor, with strains in D major, the relative major.",
};

// Copyright: the tune is public domain in Argentina, where a work is protected for 70 years after
// its author's death: Cobián died in 1953. Cadícamo's words (he died in 1999), which the demo does
// not play, are not.
// Written out from a piano performance (MIDI in 4/4).
pub(super) const LOS_MAREADOS: Song = Song {
    name: "Los mareados",
    origin: "Juan Carlos Cobián, words by Enrique Cadícamo, 1942",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 106.2,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Verse 1a",
            "-:8 D5:4 A4 | _:2 D5:3 -:1 A4 D5 C5:8 | _:2 A4 -:1 Bb4:2 C5:1 Bb4:6 -:2 | \
            - Bb4:3 -:1 F4 Bb4 A4:4 - | -:8 G5 | -:2 G5:1 - C#4:2 A5:1 G5 G5:2 F5:1 -:3 G5:1 G#5 | \
            A5:4 F5:1 G5 F5:4 E5:1 - E5:4 | -:2 B4 D5 C#5:1 E5:3 C#5:1 -:3 A4:2",
            "Dm | G7 C7 | F7/Eb Bb/D | Bb7 A7 | B D Gm Gm | A7 Dm | F/C Em/B | Em Bb7 A7 A7",
        )
        .with_left("-:8 F4:5 -:3 | G2+F3:5 -:3 E4:6 G4:2 | _:3 -:5 D3 -:3 | Bb2:6 -:2 A2:7 E3:1 | \
            D#3 -:3 D2 F#2:1 Bb4:6 D5:2 | _:1 - E3:4 -:2 D3:7 -:1 | C3:7 -:1 B2:7 -:1 | \
            E2:4 Bb2:3 A2:5 G3:1 -:2 Bb3+Cb4:1 & -:8 D2+A2:4 - | -:3 F4:4 -:1 C3+Bb3:5 -:3 | \
            A3:9 -:3 F4:4 | _:2 D4 -:4 G3+C#4:3 - F3:1 - | -:4 E2:2 F2:1 - G2:4 - | \
            A2:2 -:8 A3:6 | -:2 A3:6 -:8 | -:4 Ab4:2 -:1 G4:2 -:1 E3:3 - & -:16 | - | \
            Eb3:6 -:2 Bb3:4 D4:2 - | Ab3:5 -:11 | -:4 F#2:1 -:11 | -:16 | -:2 G5 -:12 | \
            -:4 D4:1 -:11")
        .with_tempo(&[tempo(32, 105), tempo(48, 109), tempo(64, 95), tempo(96, 99), tempo(112, 92)]),
        // Bars 9-16.
        Part::new(
            "Verse 1b",
            "C5:2 B4:1 A4 C5 B4 Bb4 A4 D5:4 A4:1 -:3 | - D5:5 A4:1 D5 C5:6 | \
            _:3 D5:1 A4:2 -:1 Bb4 - C5 Bb4:6 | _:3 -:1 Bb4:3 -:1 F4 Bb4 A4:6 | \
            _:2 -:4 F#4:3 -:1 G4:6 | _ -:3 G4:1 A4 G4 G4:3 F4:1 | \
            _:2 G4 A4 G4:1 F4 G4 - F4:4 -:2 | -:1 E4 F4 D#4 E4:5 Bb4:2 F4:1 D4:4",
            "C7 F Dm Dm | G7 G7 G7 C7/E | C7 F7/Eb F7/Eb Bb/D | Bb/D Bb/D Bb7 A7 | A D Gm/D C7/E | \
            Em7 A7 A7 G7/D | Dm7 Dm7 F/C Em | Em A Dm/A Dm/A",
        )
        .with_left("C4:2 Bb4 B#3+C#4:1 -:3 D3+F4:4 -:3 B3:1 | _ -:11 E3:4 | _:1 - Eb3:10 D3:4 | \
            _:5 -:1 Bb2+G#3:4 C#4:6 | _:1 - Eb3:2 D3:8 E2:4 | _:2 -:1 E4:3 E3:4 -:2 D3:4 | \
            _:6 - E2:4 | _ A2:12 & C#4+G4:1 - B3 A3+Bb3 -:4 A3:3 -:5 | G2+F3:10 -:4 Bb3:2 | \
            _:1 - A3:9 -:5 | F4:4 D4:3 -:1 D4 -:3 A2:4 | _:1 -:7 D2:3 -:5 | -:1 D4:2 -:1 A2:6 - | \
            -:4 C3:9 -:3 | -:1 Bb2:3 -:4 A1 D2 & -:8 G3:1 -:7 | G#2+C#3:1 -:15 | G4:5 - D4:6 | \
            _:3 -:9 G3:4 | E3:3 -:7 Bb3:6 | _:1 -:3 C#4:2 - G3:4 - | -:12 B2:4 | \
            _:3 -:7 A4:2 -:4 & -:16 | - | - | - | - | - | - | -:2 E4:1 -:13")
        .with_tempo(&[tempo(0, 94), tempo(16, 104), tempo(32, 101), tempo(48, 95), tempo(64, 93), tempo(96, 99), tempo(112, 90)]),
        // Bars 17-24.
        Part::new(
            "Refrain 1a",
            "F#4:1 - G4:4 F#4:1 -:3 G4:1 G#4 A4 - E5 F5 | F#5 A5 G5:6 E5:1 D#5 F#5:4 E5:1 F#5 | \
            D#5 E5 G#5 B5 E6 -:5 C#4:3 -:1 E4:2 | _ G4:4 E4:2 D4:1 C#4 E4:5 -:1 | \
            -:2 D4:9 -:3 D4:2 | _:1 F#4:2 E4:1 D4 - C#4 -:5 D5:1 D#5 E5 - | \
            B5 -:3 E6:1 - A6 -:5 D6:1 -:3 | -:1 F5:3 E5 D5:2 C5:1 Bb4 Bb4:3 C4:2",
            "Em7 Em7 E7/D D/F# | B7 B7 B7/D# E7/G# | E E Gm/Bb A | Em/G Em D7 D7 | D Em7 Em7 D/F# | \
            D Bb A7 A7 | A Gm7 | Dm C7 C7 Bb7/F",
        )
        .in_key(D, Major)
        .with_left("C#4:1 - E2+D4:4 D2:5 F2:1 F#2:4 | _:2 - D#3+A3:6 -:2 G#3:3 E5:1 | \
            -:4 E5:1 - E2 - Bb2:4 A2 | G2 E2+F#4:5 -:1 C2:2 D2:4 | _:3 -:1 E2:6 B3:1 A#3:2 -:3 | \
            -:4 Bb2:5 -:1 G3:3 - | A2+A3:2 E3:1 -:3 D#2+A5:1 - F2 F#2 G2:5 D3:1 | \
            -:4 C3:5 -:3 F2:4 & -:6 C#4:1 -:3 E2:2 -:4 | -:6 F#5:2 -:4 D4:2 B3:1 - | \
            -:10 G3:2 -:4 | -:8 A1:3 C#2:2 -:3 | A2:4 -:2 G3 -:1 B2 -:2 F#2+D3:4 | \
            _:1 -:3 F3:4 B3:1 - A2:2 -:4 | A4+E5+F5+A5:1 -:11 D5+G5+Bb5:1 -:3 | \
            -:4 E3+Bb3 G3:1 -:3 Ab2:1 -:3 & -:16 | - | - | - | -:2 G3:3 -:7 A3:2 - | \
            -:7 B3:1 -:8 | -:16 | -")
        .with_tempo(&[tempo(0, 95), tempo(16, 98), tempo(32, 101), tempo(48, 103), tempo(64, 97), tempo(96, 107), tempo(112, 103)]),
        // Bars 25-32.
        Part::new(
            "Refrain 1b",
            "A4:4 -:2 C#4 G4 - G#4:1 -:3 | A4:2 - G4 F#4 -:1 E4:2 D#4:1 F#4:4 | \
            _:2 - G4:1 -:2 F#4:1 - E4 C#4:2 - D#5:1 E5 | G5 F#5:2 E5:1 - D5 E5 -:7 A6:2 | \
            _ -:7 D4:5 F4:2 | E4:6 C4:2 -:8 | D4:1 F#4 - A4 C#5:2 C#5:1 C5 B4 D5:7 | \
            _:1 B4 A4 G4 F#4:3 - F#4:5 D4:1",
            "F A/E Em D | C B7 E/B E/B | E7 A | A E7/D | D7 Bb | C7 D7 | D B Em7 Em7 | \
            Em7 E A7 A7",
        )
        .in_key(D, Major)
        .with_left("-:2 F2:1 - E2:4 -:2 E2:1 - F#4:4 | C2:3 -:1 B1+A3:4 B2 E2 | \
            _:3 A#2:1 B2:2 - A#2:3 -:1 E3:4 | _ -:1 C#5 -:2 D2:8 | \
            _:1 - D2 - D3 C#3:2 B2:1 Bb2:8 | C3:10 A3:6 | -:4 D#3:6 -:1 E3:5 | \
            _:7 -:3 A2:6 & -:12 D2:3 -:1 | C4+E4:2 - D#4 -:6 G#3:4 | \
            _:1 - A2 - G#3+D4 -:4 D4:1 A2:6 | _:5 -:9 G#6:2 | _ -:4 C3:2 -:8 | \
            Bb3:5 -:3 A#3:1 -:3 F#2:4 | _:1 -:3 B2:4 -:6 G3:2 | \
            _:3 B3:1 G#3+B3:3 - C#4:5 D3:1 & -:16 | D#2:1 -:3 B3+F#4:1 -:11 | -:14 D5:1 - | \
            -:3 C#3:2 -:11 | A5:2 -:14 | -:4 D4 -:7 D3:1 | _:3 -:4 A#4+B4:1 -:8 | \
            D4:4 -:6 G3:3 E3:2 -:1 & -:16 | - | - | - | - | - | - | -:14 D#3:1 -")
        .with_tempo(&[tempo(0, 98), tempo(32, 90), tempo(48, 100), tempo(80, 99), tempo(96, 95)]),
        // Bars 33-41.
        Part::new(
            "Third part 1a",
            "-:4 C#4:1 C4 -:2 A4:7 C#4:1 | -:4 A5:3 - F4:1 G4 A4:3 -:1 | \
            A4:2 G4:1 F4 Bb4:7 G4:1 G#4 A4 C#5 E5 | C#5 A5 -:4 A4:1 A4:7 F#4:2 | \
            G4:1 G#4 A4 G4 F#4:2 A4:10 | _:4 -:6 G4:1 -:3 A4:1 G4 | \
            A4 Bb4 G4:2 F4:1 E4 G4:5 F4:1 F4:4 | _:2 - F4:4 E4:3 - F4:2 | \
            E4:6 F4:2 E4:1 -:2 G#4:1 A5 - A4 B4",
            "A | F7 Dm Dm Dm | Dm Gm A7 A7 | A7 D7 | Gm D7 Em7 Em7 | Dm A7 | A7 A7 G7 Dm | \
            Dm7/C E7/B | Em/B Bb7 A7 A7",
        )
        .with_left("-:1 A3 -:2 B3:1 Bb3 A3:9 -:1 | - Eb2:4 -:2 A2:9 | _:2 - G2:1 G#2 A2:2 A1:1 -:6 E3:1 | \
            _:3 G3:1 -:4 D2:1 -:3 F#3+C4+D4:1 -:3 | D2:1 - Bb3 - F#3+A3+C4 -:3 G2 -:1 E4:2 D4 | \
            _:3 -:1 D#2 E2 F2:2 G2 -:6 | -:1 A4 C#4+E4 -:5 D2:1 -:3 D4:4 | _:2 - C2 - B2:8 | \
            _:2 Bb2:7 -:2 G4:1 A4+Bb4+C#5+E5 -:2 Bb4:1 & -:12 A1:4 | _:1 -:3 D2:12 | _:2 -:14 | \
            -:1 C#4+A4 -:14 | -:10 D3+B3:1 -:2 Eb4:1 -:2 | - D2:3 -:2 F#2:1 -:8 | \
            -:12 D3:3 C#3:1 | C3:8 G#3+D4:3 -:1 E2:3 -:1 | -:4 Ab3 G3:2 -:6 & -:16 | -:14 D3:2 | \
            _:1 -:15 | -:16 | - | - | - | - | -:8 A2:1 -:7")
        .with_tempo(&[tempo(0, 96), tempo(16, 92), tempo(32, 97), tempo(48, 98), tempo(80, 104), tempo(96, 99), tempo(112, 96), tempo(128, 92)]),
        // Bars 42-50.
        Part::new(
            "Third part 1b",
            "C6:2 -:1 B5 -:2 Bb5:1 A5 A5:2 - F4:1 F4 F4 - | G4 - A4 - A4:2 G4:1 F4 Bb4:7 -:1 | \
            -:4 Bb4:1 - A4 Bb4 A4 G#4 A4:6 | E4:1 F4 F#4:2 A4:3 G4:1 F#4 - A4:4 E4:1 F#4 | \
            G4 A4 Bb4:8 -:2 G5:1 -:3 | A4:1 G4:2 A4:6 G4:1 F4 E4 F4 -:3 | \
            -:2 A5:1 - A5 G5 F5 - E5 D5 E5:3 - | G5:1 -:3 F5:2 E5:4 D5:1 C#5 D5 -:3 | \
            A5:1 -:3 D6:1 -:11",
            "C F7 Dm Dm | Dm7 A | A F7 | D7 D | Gm Gm7 | A7 A7 A7 Dm | Dm7 Dm7 C Em/B | \
            Bb7 A7 A Dm | A Dm Dm Dm",
        )
        .with_left("C5:2 -:4 Eb2:1 F5 F5:2 D2+A2 D4:1 D4 D4 - | D2+D3:3 -:1 F4:2 D2 C#4:5 -:3 | \
            A1+A2:4 C#4:1 - A1+A2 -:5 Eb2:4 | _:2 C4:6 -:4 D4:2 - | -:8 F2:1 F#2 G2:4 -:2 | \
            -:5 C#3:4 -:3 D3:4 | _:5 -:7 B2:4 | _:1 -:3 A2+C#5:4 C#2 D2:2 - | \
            A1:4 D2+D5+F5+A5:1 -:11 & C#5+G5:1 -:9 Eb2:1 -:5 | \
            Eb2+C3+E4:1 - F4 -:3 D3+E4:1 -:3 A1+A2:5 -:1 | -:14 C4:1 - | \
            -:2 D2:4 D3:1 -:5 G2:1 -:3 | -:12 G4:1 -:3 | -:7 C#4:1 -:8 | D2:1 -:3 C3:8 -:4 | \
            Bb2 G4:3 -:1 B1:2 -:6 | A4+C#5:1 -:2 B#1+C#2:1 -:12 & -:16 | - | - | - | - | - | \
            -:4 G5:1 -:11 | Ab4:2 -:14 | -:16 & -:16 | - | - | - | - | - | - | D5:1 -:15 | -:16")
        .with_tempo(&[tempo(0, 104), tempo(16, 98), tempo(32, 99), tempo(48, 96), tempo(80, 103), tempo(96, 89), tempo(112, 85), tempo(128, 99)]),
        // Bars 51-58.
        Part::new(
            "Verse 2a",
            "-:4 D5 A4:6 D5:2 | _:1 - A4 D5 C5:10 A4:2 | -:1 Bb4:2 C5:1 Bb4:6 -:4 Bb4:2 | \
            _:1 - F4 Bb4 A4:4 -:8 | -:4 G5:8 -:2 G5:1 - | \
            C#4:2 A5:1 G5 G5:2 F5:1 -:3 G5:1 G#5 A5:4 | F5:1 G5 F5:4 E5:1 - E5:4 -:2 B4 | \
            D5 C#5:1 E5:3 C#5:1 -:3 A4:2 C5 B4:1 A4",
            "Dm Dm F G7 | G7 C7 C7 F7/Eb | F7/Eb Bb/D Bb7 Bb7 | Bb7 A7 F7 F7 | D Gm Gm/Bb A7 | \
            A7/E Dm D7 D7 | F/C G7/B Em/B Em | Bb7 A7 C7 C7",
        )
        .with_left("-:4 F4:5 -:3 G2+F3:4 | _:1 -:3 E4:6 G4:5 -:1 | -:4 D3:5 -:3 Bb2:4 | \
            _:2 - A2:7 E3:1 Eb3 -:3 | D2 F#2:1 Bb4:6 D5:3 -:1 E3:2 | _ - D3:7 -:1 C3:4 | \
            _:3 -:1 B2:7 -:1 E2:4 | Bb2:3 A2:5 G3:1 -:2 Bb3+Cb4:1 C4:2 Bb4 & -:4 D2+A2 -:7 F4:1 | \
            _:3 -:1 C3+Bb3:5 -:3 A3:4 | _:5 -:3 F4:6 D4:2 | -:4 G3+C#4:3 - F3:1 -:5 | \
            E2:2 F2:1 - G2:4 - A2:2 - | -:6 A3 -:2 A3 | _:4 -:12 | \
            Ab4:2 -:1 G4:2 -:1 E3:3 - C#4+G4:1 - B3 A3+Bb3 & -:16 | -:12 Eb3:4 | \
            _:2 - Bb3:4 D4:2 - Ab3:4 | _:1 -:15 | F#2:1 -:15 | -:14 G5:2 | -:16 | D4:1 -:15")
        .with_tempo(&[tempo(0, 105), tempo(48, 107), tempo(64, 93), tempo(80, 99), tempo(96, 91), tempo(112, 97)]),
        // Bars 59-66.
        Part::new(
            "Verse 2b",
            "C5:1 B4 Bb4 A4 D5:4 A4:1 -:6 D5:1 | _:4 A4:1 D5 C5:9 D5:1 | \
            A4:2 - Bb4:1 C5 Bb4:9 -:1 | Bb4:3 -:1 F4 Bb4 A4:8 -:2 | - F#4:3 -:1 G4:10 | \
            _:2 -:3 G4:1 A4 G4 G4:3 F4 G4:2 | A4 G4:1 F4 G4 - F4:4 -:3 E4:1 F4 D#4 | \
            E4:5 Bb4:2 F4:1 D4:4 F#4:1 - G4:2",
            "F Dm B7 G7 | G7 C7/E | F7/Eb Bb/D | Bb7 A7 | Gm/D Gm/D C7/E Em7 | A7 G7/D | Dm7/C Em | \
            A D",
        )
        .with_left("B#3+C#4:1 -:3 D3+F4:4 -:3 B3:2 -:3 | -:8 E3:5 -:1 Eb3:2 | _:8 D3 | \
            _:1 - Bb2+Ab3:4 C#4:7 -:1 Eb3:2 | D3:8 E2:6 -:1 E4 | _:2 E3:4 -:2 D3:8 | \
            _:2 -:6 E2:8 | A2:14 E2+D4:2 & -:4 A3:3 -:5 G2+F3:4 | _:6 -:4 Bb3:3 -:1 A3:2 | \
            _:7 -:5 F4:3 -:1 | D4:3 -:1 D4 -:3 A2:5 -:3 | -:4 D2:3 -:6 D4:2 -:1 | A2:6 -:10 | \
            C3:9 -:4 Bb2:3 | -:4 A1 D2:6 -:2 & -:4 G3:1 -:7 G#2+C#3:1 -:3 | -:12 G4:4 | \
            _:2 -:4 D4:9 -:1 | -:8 G3:4 E3:3 -:1 | -:6 Bb3 -:4 | C#4:2 - G3:4 -:8 | - B2:7 -:1 | \
            -:6 A4:2 -:4 C#4:1 -:3 & -:16 | - | - | - | - | - | -:14 E4:1 - | -:16")
        .with_tempo(&[tempo(0, 100), tempo(48, 93), tempo(64, 98), tempo(80, 91), tempo(96, 99), tempo(112, 90)]),
        // Bars 67-74.
        Part::new(
            "Refrain 2a",
            "-:2 F#4:1 -:3 G4:1 G#4 A4 - E5 F5 F#5 A5 G5:2 | \
            _:4 E5:1 D#5 F#5:4 E5:1 F#5 D#5 E5 G#5 B5 | E6 -:5 C#4:3 -:1 E4:4 G4:2 | \
            _ E4 D4:1 C#4 E4:5 -:3 D4:2 | _:7 -:3 D4 F#4:2 E4:1 | \
            D4 - C#4 -:5 D5:1 D#5 E5 - B5 -:3 | E6:1 - A6 -:5 D6:1 -:4 F5:3 | \
            E5 D5:2 C5:1 Bb4 Bb4:3 C4:2 A4:4",
            "E7/D D/F# | B7/D# E7 | C7 A7 | A7/E A7/E E7 D | Em7 D/F# | Gm7/Bb Gm7/Bb Em7 A | \
            B7 G7 Gm7 Gm7 | C7 F",
        )
        .in_key(D, Major)
        .with_left("-:2 D2:5 F2:1 F#2:6 -:2 | D#3+A3:6 -:2 G#3:3 E5:1 -:4 | E5:1 - E2 - Bb2:4 A2 G2 | \
            E2+F#4:5 -:1 C2:2 D2:7 -:1 | E2:6 B3:1 A#3:2 -:7 | Bb2:5 -:1 G3:3 - A2+A3:2 E3:1 - | \
            -:2 D#2+A5:1 - F2 F#2 G2:5 D3:1 -:4 | \
            C3:5 -:3 F2:4 -:2 F2:1 - & -:2 C#4:1 -:3 E2:2 -:8 | -:2 F#5 -:4 D4:2 B3:1 -:5 | \
            -:6 G3:2 -:8 | -:4 A1:3 C#2:2 -:3 A2:4 | -:2 G3 -:1 B2 -:2 F#2+D3:5 -:3 | \
            F3:4 B3:1 - A2:2 -:4 A4+E5+F5+A5:1 -:3 | -:8 D5+G5+Bb5:1 -:7 | \
            E3+Bb3:4 G3:1 -:3 G#2:1 -:7 & -:16 | - | - | -:14 G3:2 | _:1 -:7 A3:2 -:6 | \
            -:3 B3:1 -:12 | -:16 | -")
        .with_tempo(&[tempo(0, 96), tempo(16, 98), tempo(32, 102), tempo(48, 103), tempo(64, 91), tempo(80, 103), tempo(96, 112), tempo(112, 98)]),
        // Bars 75-82.
        Part::new(
            "Refrain 2b",
            "-:2 C#4 G4 - G#4:1 -:3 A4:2 - | G4 F#4 -:1 E4:2 D#4:1 F#4:6 -:2 | \
            G4 -:1 F#4 -:2 E4:1 D4 C#4:2 - D5:1 D#5 E5 G5 | F#5:2 E5:1 - D5 C#5 E5 -:7 A6:2 | \
            _ -:7 D4:5 F4:2 | E4:6 C4:2 -:8 | D4:1 F#4 - A4 C#5:2 C#5:1 C5 B4 D5:7 | \
            _:1 B4 A4 G4 F#4:3 - F#4:5 D4:1",
            "A7/E D7 | B7 E | E7/B Bb A A | A E7/D | D7 Bb | C7 D7 | D B Em7 Em7 | Em7 E A7 A7",
        )
        .in_key(D, Major)
        .with_left("E2:4 -:2 E2:1 - F#4:4 C2:3 -:1 | B1+A3:4 B2 E2:7 A#2:1 | \
            B2+G#3+D4:2 -:3 Bb2:4 -:1 E3:6 | _:4 - D2:8 | _:1 - D2 - D3 C#3:2 B2:1 Bb2:8 | \
            C3:10 A3:6 | -:4 D#3:6 -:1 E3:5 | _:7 -:3 A2:6 & -:8 D2:3 -:1 C4+E4:2 - | \
            D#4 -:6 G#3:5 -:1 A2 - | -:8 A2 | _:4 -:10 G#6:2 | _ -:4 C3:2 -:8 | \
            Bb3:5 -:3 A#3:1 -:3 F#2:4 | _:1 -:3 B2:4 -:6 G3:2 | \
            _:3 B3:1 G#3+B3:3 - C#4:5 D3:1 & -:12 D#2:1 -:3 | B3+F#4:1 -:15 | -:16 | \
            -:2 C#3 -:12 | A5:2 -:14 | -:4 D4 -:7 D3:1 | _:3 -:4 A#4+B4:1 -:8 | \
            D4:4 -:6 G3:3 E3:2 -:1 & -:16 | - | - | - | - | - | - | -:14 D#3:1 -")
        .with_tempo(&[tempo(0, 96), tempo(16, 101), tempo(32, 107), tempo(64, 101), tempo(80, 99), tempo(96, 95)]),
        // Bars 92-100.
        Part::new(
            "Third part 2b",
            "C6:2 -:1 B5 -:2 Bb5:1 A5 A5:2 - F4:1 F4 F4 - | G4 - A4 - A4:2 G4:1 F4 Bb4:7 -:1 | \
            -:4 Bb4:1 - A4 Bb4 A4 G#4 A4:6 | E4:1 F4 F#4:2 A4:3 G4:1 F#4 - A4:4 E4:1 F#4 | \
            G4 A4 Bb4:8 -:2 G5:1 -:3 | A4:1 G4:2 A4:6 G4:1 F4 E4 F4 -:3 | \
            -:2 A5:1 - A5 G5 F5 - E5 D5 E5:3 - | G5:1 -:3 F5:2 E5:4 D5:1 C#5 D5 -:3 | \
            A5:1 -:3 D6:1 -:11",
            "C F7 Dm Dm | Dm7 A | A F7 | D7 D | Gm Gm7 | A7 A7 A7 Dm | Dm7 Dm7 C Em/B | \
            Bb7 A7 A Dm | A Dm Dm Dm",
        )
        .with_left("C5:2 -:4 Eb2:1 F5 F5:2 D2+A2 D4:1 D4 D4 - | D2+D3:3 -:1 F4:2 D2 C#4:5 -:3 | \
            A1+A2:4 C#4:1 - A1+A2 -:5 Eb2:4 | _:2 C4:6 -:4 D4:2 - | -:8 F2:1 F#2 G2:4 -:2 | \
            -:5 C#3:4 -:3 D3:4 | _:5 -:7 B2:4 | _:1 -:3 A2+C#5:4 C#2 D2:2 - | \
            A1:4 D2+D5+F5+A5:1 -:11 & C#5+G5:1 -:9 Eb2:1 -:5 | \
            Eb2+C3+E4:1 - F4 -:3 D3+E4:1 -:3 A1+A2:5 -:1 | -:14 C4:1 - | \
            -:2 D2:4 D3:1 -:5 G2:1 -:3 | -:12 G4:1 -:3 | -:7 C#4:1 -:8 | D2:1 -:3 C3:8 -:4 | \
            Bb2 G4:3 -:1 B1:2 -:6 | A4+C#5:1 -:2 B#1+C#2:1 -:12 & -:16 | - | - | - | - | - | \
            -:4 G5:1 -:11 | Ab4:2 -:14 | -:16 & -:16 | - | - | - | - | - | - | D5:1 -:15 | -:16")
        .with_tempo(&[tempo(0, 104), tempo(16, 98), tempo(32, 99), tempo(48, 96), tempo(80, 103), tempo(96, 89), tempo(112, 85), tempo(128, 92)]),
    ],
    form: &["Verse 1a", "Verse 1b", "Refrain 1a", "Refrain 1b", "Third part 1a", "Third part 1b", "Verse 2a", "Verse 2b", "Refrain 2a", "Refrain 2b", "Third part 1a", "Third part 2b"],
    drums_from: 0,
    about: "Cobián wrote the tune in the 1920s as \"Los dopados\"; Cadícamo's new words of \
            1942 made it famous: two former lovers meet again in a bar, drunk, to say goodbye. \
            D minor, with a refrain in D major.",
};

// Copyright: Gardel died in 1935 but Razzano in 1960; not public domain; added at the owner's
// request.
// Written out from a piano performance (MIDI in 4/4).
pub(super) const MANO_A_MANO: Song = Song {
    name: "Mano a mano",
    origin: "Carlos Gardel and José Razzano, words by Celedonio Flores, 1923",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 106.7,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:6 D4:2 -:1 F4 G4 A4 Bb4:3 Ab4:1 | G4 - E4:4 Bb5 A5:1 G#5 A5:2 G5 | \
            C6:10 G5:1 F5 G5:2 F5 | E5:3 - G4:4 F4:1 E4 F4:2 E4:1 Eb4 | \
            D4:2 -:4 D5:2 E5:1 F5 G5 A5 Bb5 - A5 Ab5 | G5 -:5 Bb4:4 G#4:1 - A4:3 Gb4:1 | \
            F4 -:5 A5:2 G5:1 -:3 G5:2 Gb5:1 F5 | E5 -:5 G4:4 F4:1 E4 D4 -:3",
            "Dm Dm A7 Gm | Gm Gm Gm Dm | Dm7 Dm7 Dm7 A7 | A7 A7 A7 Dm | Dm Gm7 | C7 C7 C7 F | \
            Gm7 C7/Bb | A7",
        )
        .with_left("-:9 E4:1 -:2 Bb3:4 | _:1 - Bb3+D4:5 -:1 G3+Bb3:2 - F3+A3:4 | \
            _:1 - D3+F3+A3:5 -:1 D3+F3+A3+B3:2 - A2+C#3+E3+G3:4 | \
            _:1 - A2+C#3+E3+G3:4 -:2 A2+C#3+E3+G3 - D3+F3+A3+B3:4 | \
            _:1 - D3+F3+A3+B3:2 -:6 F5:1 F2+Gb2 G2:4 | _:11 Gb2:1 F2:4 | _ Bb1:8 E2:4 | \
            _:9 -:3 D2:1 - G2 G#2 & -:12 G3+D4+E4:4 | -:2 G3:4 G#5:1 - D4+E4 -:3 D3+B3:4 | \
            F5:6 A5:5 - | -:16 | - | -:2 G1 C2:8 -:2 G4 | F1:3 -:5 Bb2:1 -:2 Eb2:1 F#5 -:3 | \
            -:2 A1:10 -:4 & -:14 A4:1 - | -:16 | -:2 B3:4 -:10 | -:16 | - | -:6 E4:3 -:7 | -:16 | \
            -:6 C#4:2 C#2:4 - & -:16 | - | - | - | - | -:8 E2:4 - | -:16 | - & -:16 | - | - | - | \
            - | -:8 A4:1 -:7 | -:16 | -")
        .with_tempo(&[tempo(32, 109), tempo(48, 110), tempo(96, 108), tempo(112, 112)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "A5:1 -:3 B5:1 -:3 C#6:1 -:3 A5:1 -:2 D4:1 | D4 - E4 - F4:2 - G4:1 F4 E4:4 D4:1 E4 | \
            C#4 D4 D#4 E4 -:2 A4:4 F4:1 E4 E4:4 | -:2 D4:1 E4 F4 G4 A4 Bb4 A4:5 F#4:2 -:1 | \
            -:4 D5 -:1 F#4 A4 Ab4 G4:4 | - C4:3 - C4:1 -:3 G5:1 G5 | \
            G4 - A4 - Bb4 - Bb4 - Bb4 - Bb4 - C5:2 Bb4:1 - | \
            Bb4 Bb4 C5:4 Bb4:1 -:2 Bb4:1 A4 - C5 -:2 A4:1",
            "A A A7/G Dm/F | C C C Dm | D D A7 Dm | Gm7 D | D D/F# D Gm | Gm D7/A D7 Gm | \
            A7 Gm Gm7 C7/G | C7 C7 C7 Gm7/F",
        )
        .with_left("A2:5 -:3 G2:4 F2:2 - | -:4 E4:2 -:6 D2:1 - D3:2 | _:4 A2:5 -:3 D3:4 | F3:1 -:10 A2:5 | \
            _:1 -:3 F#2:5 -:7 | Bb2:1 -:3 A2+F#3:2 - A2:3 Ab2:1 G2 - G4 G4 | \
            -:4 Bb3:1 -:5 F2:1 F#2 G2:4 | \
            _:9 E4:1 -:2 F2+F3+F4+Bb4:4 & D#5+E5:1 -:3 G#2:4 G5:1 -:3 F5:1 -:3 | -:15 D4:1 | \
            -:6 E3:4 - A3:1 - | -:16 | F#3:1 -:7 D3:1 -:7 | -:8 D2:2 F#2 -:4 | \
            -:12 E4+G4:1 - G1:2 | _ E4:3 -:1 E4 -:9 & -:4 D5+G#5:1 -:11 | -:16 | -:6 F#4:3 -:7 | \
            -:16 | - | -:10 B3:1 -:5 | -:16 | G4:1 E4+G4+A4 -:2 C2:5 -:7 & -:16 | - | -:8 G3:4 - | \
            -:16 | - | - | - | -")
        .with_tempo(&[tempo(0, 104), tempo(32, 106), tempo(64, 108), tempo(80, 105), tempo(96, 108)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "-:1 G#4:2 A4:1 Bb4 A4 - A4 G#4 A4:5 G4:1 - | \
            - A4 G#4:2 A4:1 G4 - G4 F#4:2 G4:4 F4:1 - | - A4 G#4 F#4 A4:2 F4 E4:1 D4 F4:4 E4:1 - | \
            -:2 F4:1 E4 F4:2 E4:1 -:3 D4:1 C#4 D4 -:3 | \
            A5:1 -:3 D6:1 -:3 C6:1 -:3 F6:1 -:2 Bb5:1 | B5 C6 -:2 C7:1 - D7 - C6 -:5 D6:2 | \
            C6:1 - C6 - C#6 - D6 - C6 - C6 Bb5 Bb5:3 -:1 | \
            A5 -:3 A5:1 - A5 Bb5 A5 G#5 A5 -:2 C#6:3",
            "F F F C7/E | C7 A7 A7 Dm | Dm7 C | C7/E F/A A/C# Dm | A Dm/A C F | Dm7 Dm7 F C7 | \
            C7 C7 C7 Gm7 | F A",
        )
        .with_left("-:12 E3+Bb3+D4:4 | _:1 -:3 A2+C#3+G3:5 -:3 D3:4 | _:3 -:1 C3:8 E2:4 | \
            _ A2:5 -:3 D2:4 | A1:7 B1:1 C2:4 F1:2 - | -:4 F5:5 - C4+E4+Bb4:2 | \
            _:1 -:3 C4+E4:7 -:3 F3:2 | _:3 - A3:2 -:8 & -:16 | -:14 A3:2 | _:4 G#4:1 -:7 B2:4 | \
            _:1 -:7 C#2:4 - | A4+C#5+E5:1 -:3 D2:2 Bb1:1 - E5+G5 -:3 A5+C6:1 -:3 | \
            -:4 C6+F6+A6:1 - D6 - A5 -:7 | -:4 Bb4:6 G4:1 -:5 | C4:2 -:14 & -:16 | - | - | \
            Bb2:4 -:12 | -:4 D5+F5+A5:1 -:11 | -:16 | - | -")
        .with_tempo(&[tempo(0, 110), tempo(16, 109), tempo(48, 107), tempo(64, 112), tempo(80, 115), tempo(96, 108)]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "-:3 A5:2 G#5:1 A5:3 Ab5:1 -:2 G5:1 Gb5 F5:2 | -:12 C4:1 C#4 D4 - | \
            F4:3 -:1 F4:2 F#4:1 G4:3 F4 G4:2 F4:1 | G4:2 F4:1 -:3 G4:1 F4 G4:2 F4:1 G4 F4 - G4:2 | \
            _:1 - F4 -:3 A5:1 -:3 G6:1 - E6 D6 F6:2 | _:5 -:1 E6:2 F6 E6:1 - F6 E6 E6:2 | \
            _ D6:1 -:3 D5:1 D5 E5 - D5 - D5 D5 E5:2 | \
            D4 C#5:1 -:3 C#5:1 C#5 C#5 - C#5 - B4 - C#5 -",
            "A7 | Gm | Dm7 C | Gm7/Bb | Dm | Dm Gm7 | Dm C | C7 A7",
        )
        .with_left("A2+C#3:14 D3:2 | Bb2:1 A2+B2 G#2 - A2 - Bb2:3 - C3:1 C#3 D3:2 | D2:7 C2:6 -:3 | \
            Bb1:4 F2:1 - D4 D4 Bb1:7 C#2:1 | D2:9 -:1 F6:2 D2:1 -:3 | D4:8 C#4 | _:1 -:15 | \
            E3+Bb3:2 -:4 F4+G4:1 G4 A2+C#3:8 & G3:10 -:6 | - Bb3:2 B2:4 - | -:12 B1:4 | \
            D4:3 -:5 D4:1 - D4 - F2+G2+D4 - C2+A3:2 | - A3+D4:1 - A2:5 -:7 | G#3:6 -:2 Bb3:8 | \
            F3+A3+D4:3 -:13 | -:8 G3:5 -:3 & -:6 E3:5 - | -:8 B3:1 -:7 | -:16 | - | -:8 F2:5 -:3 | \
            E3:5 -:3 G3:7 -:1 | -:16 | -:8 G4:1 - G4 - E3:4 & -:16 | - | - | - | - | \
            -:2 B3:4 -:10 | -:16 | -")
        .with_tempo(&[tempo(0, 105), tempo(32, 121), tempo(48, 105), tempo(80, 103), tempo(96, 106)]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "D5:1 -:3 A5:1 -:3 F5:1 - D4 E4 F4 G4 G#4 A4 | Bb4:3 Ab4:1 G4:6 Bb4:2 A4:1 - G#4 - | \
            A4:3 Gb4:1 F4:6 G4:4 F4:2 | G4 F4:4 E4 G4 F4:1 E4 | \
            F4 E4 D4:5 -:3 D4:1 E4 F4 G4 G#4 A4 | Bb4:2 A4:1 Ab4 G4 -:5 Bb4:2 A4:1 - G#4 - | \
            A4:2 G4:1 Gb4 F4 -:5 A5:2 G5:1 -:3 | G5:2 Gb5:1 F5 E5 -:5 G4:4 F4:1 E4",
            "Dm A Dm Dm | Gm C7/E | Dm C | C7/Bb A7 | Dm C Dm Dm | C7 A | Dm F/C | Gm/Bb A7",
        )
        .with_left("D3:2 G2:1 G#2 A2:4 D2 - | G2 -:2 G3 E3:7 -:1 | D3:6 C#3:2 C3:8 | _:3 -:5 A2:6 C2:2 | \
            D2:5 -:3 F2:4 - | G3:3 - E3:5 -:2 C#2:1 C2:2 | D2:7 -:1 C2:5 -:3 | \
            Bb1:9 -:1 C#4:2 A2:4 & F4+A4:1 -:3 A4+C#5+E5:1 -:3 D5:1 -:7 | -:2 A4:1 -:11 Eb3:2 | \
            - G4 -:12 | -:2 Bb2:6 -:2 C#4:4 -:1 C#2 | -:4 E2:5 -:7 | \
            -:2 Bb3:1 -:5 A2:3 -:4 C#2:1 | -:6 C#2:2 - F5 B1:4 | -:6 Bb2:2 A1:7 -:1 & -:16 | - | \
            - | -:12 A1:4 | -:16 | - | -:12 E5:1 -:3 | -:10 F#4:1 -:5")
        .with_tempo(&[tempo(16, 109), tempo(48, 111), tempo(64, 112), tempo(80, 110)]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "D4:1 -:3 A5:1 -:3 B5:1 -:3 C#6:1 -:3 | A5:1 - D#4 -:3 E4:2 F4 E4 G4:1 F4 E4:2 | \
            _ D4 A4:1 G#4 A4:4 F4:3 -:1 F4 E4 | E4:7 -:3 D4:1 E4 F4 G4 A4 Bb4 | \
            A4:2 -:1 F#4:2 F4:1 F#4 - D5:2 - C4:1 F#4 A4 Ab4 | G4:4 D5:1 -:3 E5:1 -:3 F#5:1 -:3 | \
            Bb4:1 - G4 G4 G4 - A4 - Bb4 - Bb4 - Bb4 Bb4 C5:2 | \
            -:4 Bb4:1 - C5:3 -:1 Bb4 - Bb4 A4 Bb4:2",
            "Dm Dm Dm A7/G | Dm C | Dm A7/E | Dm Dm/F | D | A7/G D A7/C# D7/C | C7 C7 Gm C7 | \
            C C7",
        )
        .with_left("D2:3 G#2:1 A2:5 -:3 G2:4 | _:1 - D4:4 -:10 | D2:5 -:3 E2:6 -:2 | \
            D2:4 D3:1 - F3:6 D3:1 -:3 | D2 -:5 D3 -:3 | G2 C#3:1 D3:5 -:3 C3:4 | \
            _:1 -:9 Bb3+D4:1 -:5 | \
            G2:3 -:1 D4+G4 - G4:3 -:1 G2:2 E4:1 - F4:2 & -:2 G2:1 - D5+F5 -:3 G#2:5 -:3 | \
            F2:2 -:14 | -:10 G2:4 -:2 | - A2 -:4 A3 - | -:2 F#2:3 -:5 F#3:3 - | \
            -:2 C3:1 - F#4 -:3 C#3:4 C5:1 -:3 | Bb2:2 -:14 | \
            -:2 D3:1 -:3 E4:2 C2:4 - & -:8 D5+A5:1 -:3 G5:1 -:3 | F5:1 -:15 | -:12 C#3:1 -:3 | \
            -:16 | - | -:8 G4+C#5:1 -:7 | -:16 | -:6 B4:1 -:3 E4:1 -:5")
        .with_tempo(&[tempo(0, 101), tempo(16, 104), tempo(32, 106), tempo(80, 103), tempo(96, 105)]),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "-:2 A4:1 Bb4 G#4 - A4 - Bb4 - A4 - A4 G#4 A4:2 | \
            _ -:1 G4 -:2 A4:1 A4 A4:2 G4:1 G4 F#4:2 G4 | _:4 F4:6 G4:1 F4 G4 F4 E4 D4 | \
            F#4 - E4 -:2 F4 E4:1 F4:6 D4:1 C#4 | D4 -:3 A5:1 -:3 D6:1 -:3 C6:1 -:3 | \
            F6:1 - D6 -:3 C6:2 D6 C6:1 - B5 C6 D6:2 | _:6 C6:1 B5 C6 - Bb5 - Bb5 C6 Bb5:2 | \
            _:3 - A5:1 - A5 Bb5 G#5 A5 G#5 A5 C#6:2",
            "F D | C7/E A7 | Dm Dm7/C | F Gm7/Bb A7 A7 | Dm A Dm C | F D7 | C C7/G | F F F/C A",
        )
        .with_left("F2:2 F4 -:4 D4+F#4:1 -:7 | E3+Bb3:5 -:2 G#4:1 A2+C#3:8 | D3:9 -:7 | \
            B2:5 -:3 A2:4 C#2 | D2:5 -:2 C#2:1 D2:2 Bb1:1 B1 C2:4 | F1 -:12 | C2:2 G2:11 -:3 | \
            F2:2 C3:10 -:4 & -:2 F3:1 A4 -:12 | D4:3 -:5 G3:6 -:2 | - A3:8 -:6 | \
            F4:2 G#3:1 - Bb2:4 G3:2 E4:4 -:2 | -:4 A1 D5+F5+A5:1 -:3 E5+G5:1 -:3 | \
            A5:1 - C6:2 -:12 | -:4 E3 - C2:1 -:3 | -:4 A3:1 -:11 & -:16 | - | -:8 C3 | _:1 -:15 | \
            -:4 A4+C#5+E5:1 -:11 | -:16 | - | -")
        .with_tempo(&[tempo(0, 109), tempo(16, 103), tempo(32, 106), tempo(96, 105), tempo(112, 106)]),
        // Bars 57-66.
        Part::new(
            "Part 8",
            "-:4 D4:1 - A5:3 -:1 Ab5 -:3 G5:1 F#5 | G5:5 -:1 F5:8 -:2 | \
            - F4:3 -:1 F4:2 G4 F4 G4:1 F4 G4:2 | _ F4:4 G4:1 F4 G4:2 F4:4 G4:1 F4 | \
            G4 -:3 F5:1 - F5 - A5:2 F5:1 - E5 D5 F5:2 | - E5:3 -:1 F5 E5 F5:2 E5:1 - F5 E5 E5:2 | \
            _ -:1 C5 C#5 D5 -:2 D5:3 -:1 E5:3 -:1 | - D5 -:2 E5 Eb5:1 D5 C#5 -:6 C#5:1 | \
            _:4 B4:1 C#5:2 D5:1 -:4 A6:1 -:3 | -:6 D7:1 -:9",
            "C7/E A A7/C# A7/C# | Gm7/D A7 A/C# F/C | F Dm | Gm7/Bb Gm7/Bb Gm7/Bb A7 | Dm Dm7/C | \
            Gm A7 | D7 Dm7 | Dm C7/E Gm A7 | A7 A7 Gm/D A | Dm/A",
        )
        .with_left("E3:5 -:1 C#3:9 -:1 | D3:4 A3:1 - C#3:5 -:1 C3:4 | _:1 -:13 D4:2 | _:14 C3:1 C#3 | \
            D3:8 C3 | B2:5 -:1 G#4:2 A2:8 | D2:4 - C3:3 -:1 F4+A4:3 B2:1 | \
            - F4+A4 -:2 E3+D4+G4:4 - A2+C#3 | _:7 D3:3 G2:2 A2:4 | \
            _:6 D2:2 -:8 & -:2 Bb3 -:4 G3 A2:2 G3:1 - | -:2 F3 -:4 A3:1 - A3:2 - A3 | _:13 -:3 | \
            Bb2:10 -:2 A2:4 | D4:1 -:3 D2:2 D5:1 - D5:2 - C2 - | \
            - D5:4 -:2 G4+C#5:3 -:1 A1:3 -:1 | -:8 F4+A4:2 A3:1 -:5 | -:6 Bb3:3 - G3:4 | \
            _:2 -:9 G#2:1 A5+C#6 -:3 | -:6 F6+A6:1 -:9 & -:10 E3:4 -:2 | -:16 | B2:14 -:2 | \
            Ab3:9 -:3 G3:2 - | -:4 D5:1 -:3 F#5:1 -:7 | -:2 G#4:3 -:9 F4:2 | -:16 | -:15 G4:1 | \
            _ E3:4 -:11 | -:16 & -:16 | - | - | -:12 Eb3:1 -:3 | -:16 | -:4 Bb2 -:8 | -:16 | - | \
            - | -")
        .with_tempo(&[tempo(16, 109), tempo(32, 113), tempo(48, 101), tempo(64, 93), tempo(80, 92), tempo(96, 109), tempo(112, 99), tempo(128, 114), tempo(144, 107)]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8"],
    drums_from: 0,
    about: "Flores' words: a man settles accounts with the woman who left him for a richer \
            life, and wishes her well. The music is by the duo Gardel and Razzano. D minor.",
};

// Copyright: the tune is public domain in Argentina, where a work is protected for 70 years after
// its author's death: Cobián died in 1953. Cadícamo's words (he died in 1999), which the demo does
// not play, are not.
// Written out from a piano performance (MIDI in 4/4).
pub(super) const LA_CASITA_DE_MIS_VIEJOS: Song = Song {
    name: "La casita de mis viejos",
    origin: "Juan Carlos Cobián, words by Enrique Cadícamo, 1931",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: F,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 101.7,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "-:12 F4:1 E4 F4 Gb4 | _:8 F4:1 - E4 F4 A4 C5 Bb4 A4 | C5 Bb4 - Db5:6 Bb4:2 Ab4:5 | \
            C4:1 C5:2 Bb4:1 Ab4 Bb4 G4:6 -:1 F4 E4:2 | F4:3 -:2 C4 -:1 C5 -:2 C6 -:1 Ab4 - | \
            -:7 C6:1 F5 -:7 | F4:5 Db5:1 Eb5 Db5:2 C5:1 Eb5:2 Db5 C5 | \
            _:6 C4:4 Db5:1 Eb5 Db5 C5:2 Eb5:1",
            "F7b9 | B7/A F | Bbdim7 Bbdim7 Gdim7 Fm | Fm Dbdim7 | Fm C7 | Ab Fm | Fm Fm7 | \
            C7/E C7/E C7/E Cm7/Eb",
        )
        .with_left(
            "-:16 | A2:6 - A3:2 - | Bb2+Db4 - C5 G3:4 -:2 F3:4 | _:1 -:3 F3:2 - Db3:8 | \
            F2:3 -:2 B3:1 -:5 E4:4 -:1 | -:7 Ab5:1 Ab3+C4:8 | _:12 -:4 | E3:11 -:1 Eb3:4 & -:16 | \
            -:3 D#3:4 -:6 C4:1 -:2 | -:5 Bb4:2 Bb3:3 - Ab3 | _:1 -:3 G2:1 Ab4 -:10 | \
            -:11 Bb4:3 -:2 | -:8 F3 | _:11 -:5 | -:3 Bb3:13 & -:16 | -:5 F3:1 -:10 | \
            -:7 Ab4:1 G4:2 -:6 | -:16 | -:11 C4+Ab5:2 -:3 | -:8 C5 | _:7 -:9 | -:16 & -:16 | - | \
            -:8 E4:1 -:7 | -:16 | -:11 C5:1 -:4 | -:16 | - | -",
        )
        .with_tempo(&[tempo(32, 94), tempo(48, 101), tempo(64, 99), tempo(96, 105)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "Db5:2 C5:9 Bb4:1 Ab4 G4:2 C5:1 | Ab4:5 E4:3 G4:2 A4:5 F4:1 | \
            _ - C5:5 Ab4:1 E4:2 G4:6 | _:1 F4 - E4 F4 G4 Ab4:4 Bb4:2 C5 C4:1 - | \
            G4 C5 - G5:3 C6:1 - E4:3 C4:1 G4 F#4 G4:2 | \
            Bb4:3 Ab4:1 - Ab4 G4 - G4:3 F4:1 -:2 F4:1 G4 | F4:6 E4:3 -:2 C4:1 Db4:4 | \
            C4:1 -:3 C5:6 C4:2 Db5:1 Eb5 -:2",
            "Dbmaj7 Bbm Bbm6 Bbm6 | C7 F | Fm/Ab Fm/Ab C Gm7/F | Fm Fm Bb7 C/E | C | Bbm7/Db G/D | \
            C7 C7 Em7b5 Edim7 | Ab Fm Fm Fm",
        )
        .with_left(
            "-:4 Bb2:8 -:1 G2+F3 -:2 | E3:5 -:3 F2 -:1 F3 - G3:2 | -:12 F1:4 | - F3:5 -:3 E2:2 - | \
            -:4 C5:3 -:9 | Db2:5 Db4:1 -:2 D2:8 | Bb3:5 -:1 Bb3:3 D#3:1 E3:5 -:1 | \
            -:3 C3:1 F4:10 -:2 & -:5 F3:7 -:4 | C3+Bb3 -:6 C3:2 -:3 Ab3:1 | _:7 -:8 F2:1 | \
            _ -:3 C4:4 Ab3:1 -:7 | -:16 | Db4:3 -:5 D3:7 -:1 | \
            -:2 C2:3 -:1 C3:2 D3:1 -:5 Db3+G3:2 | -:4 Ab4:6 F3+Ab3:3 - & -:6 Db4:4 -:6 | \
            -:3 G3:1 -:12 | -:16 | -:4 Ab3:1 -:11 | -:16 | -:8 B3:4 -:2 B3:1 - | -:4 G2:2 -:10 | \
            -:4 F3:3 -:9 & -:9 Db3:3 -:4 | -:16 | - | - | - | - | - | \
            -:4 F2:2 -:10 & -:9 F4:2 -:5 | -:16 | - | - | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 100),
            tempo(16, 104),
            tempo(32, 103),
            tempo(64, 109),
            tempo(80, 94),
            tempo(96, 99),
            tempo(112, 94),
        ]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "Db5:2 C5 Eb5:1 Db5 C5:5 -:2 Db5:1 Eb5 - | Db5:2 C5 Eb5 Db5:1 -:2 C5:7 | \
            _:1 Bb4:3 G4:2 C5:1 Ab4 E4:2 G4:6 | _:7 F4:2 E4:1 F4 Gb4:2 F4:1 E4 F4 | \
            A4:4 Eb4:1 C5 Bb4 A4 C5:5 Bb4:1 -:2 | Db5 C5:1 Bb4 Ab4 G4 Bb4 Ab4:5 C5:2 Bb4 | \
            Ab4 G4:4 -:1 F4 E4:2 F4:1 -:4 C5:1 | Eb5:5 D5:4 E5:1 -:2 G5:1 -:2 B5:1",
            "Fm/Ab C/E | Ab7/Gb Bbm | Bbm7 C7 Edim7 Fm | Fm Fm Fm F7/A | F Bbm | Bbm7 Fm | \
            Gm Dbmaj7/F | Fm7/Ab C7/E C7 C7",
        )
        .with_left(
            "Ab2:5 -:1 E4:3 -:1 G3:3 - | Gb2:5 -:3 Bb2:8 | _:2 Ab4 G2:1 -:2 Bb3:3 -:1 F3:5 | \
            _:10 -:2 A2 Eb3:1 - | F3:6 -:2 Bb2:5 -:3 | Bb3:2 Ab3:1 G3 F3 E3 G3 F3:5 -:4 | \
            G2:6 -:4 F2+C3 -:2 | Ab2+F3 -:3 E3+Bb3:8 -:3 & F4:3 -:1 F3+F4:2 G2 E3:6 -:2 | \
            Eb4:3 -:1 Gb3+Eb4:2 -:3 Db4:7 | _:2 -:5 E3:2 -:4 Ab3:3 | _:10 -:6 | \
            -:2 A3:4 -:5 Db3:4 -:1 | -:10 C4:2 -:4 | D3:5 Gb2:6 -:5 | -:6 C3:7 -:3 & Bb2:1 -:15 | \
            Ab4:1 -:8 F4:7 | _:1 -:15 | -:3 D4:1 Db4 C4:4 -:7 | -:16 | - | -:5 Db3 -:6 | \
            -:16 & -:16 | -:12 F3:4 | _:1 -:15 | -:16 | - | - | - | -",
        )
        .with_tempo(&[
            tempo(16, 104),
            tempo(32, 100),
            tempo(48, 96),
            tempo(64, 102),
            tempo(80, 95),
            tempo(96, 93),
            tempo(112, 100),
        ]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "C6:1 D6:10 C6:1 - C6 - B5 | B5:2 Bb5:1 Bb5 A5 G#5 A5 C6:5 -:3 Bb5:1 | \
            Bb5 A5 A5:6 G5:1 G5 A5 G5:5 | A4:1 F5:2 E5:1 G5:5 F4:1 F5 D5 E5 D5:3 | \
            _:4 C5:2 - C4:1 - A4 E4 G4:3 -:1 | G4 F4 - F4 D5:2 -:3 E5:1 -:2 D5:4 | \
            C5:1 -:6 E5:1 D5:2 E5:1 -:3 D5:1 A4 | C5:2 -:3 Bb4:2 C5:1 Bb4:2 C5 Bb4 F4",
            "Dm/A Dm7/F | D7 Gm | Gm7b5 Gm7b5 Bbm6 C | F/C G/B G7/B Dm7/C | C7 C | \
            Fdim7 Fdim7 G7b9 C | C D7 | Gm Gm D7/F# Bb/F",
        )
        .in_key(F, Major)
        .with_left(
            "-:1 A2:6 -:1 A3:4 - | D3+F#3+C4:5 -:3 G2 D3:5 | G2:1 - Db4:8 -:1 C4:5 | \
            _:2 - B3:6 -:3 C3 | _:2 Bb3:1 -:3 C2:7 E2:1 F2:2 | \
            _:3 -:1 Ab4+B4:3 -:2 D3+F4:1 - G2 C2:4 | _:2 G2:1 -:5 D2:2 A2:1 - F#3:2 C4:1 - | \
            G2:2 D3 G3:4 F#3 F3 & -:4 F3:8 -:4 | -:14 Bb3:1 - | -:5 F4:4 F4:2 -:3 F4:2 | \
            _ -:4 D4 -:6 | E3:4 -:8 Eb2:2 - | F1 - F4 Ab2:3 - E4:4 | -:16 | \
            -:5 D4:1 -:2 D4+F#4 - D4 - & -:16 | - | -:7 Bb3:3 -:6 | -:16 | - | \
            F4:1 E4 -:2 C#5:1 - A2 -:9 | -:16 | -",
        )
        .with_tempo(&[
            tempo(0, 104),
            tempo(32, 98),
            tempo(64, 104),
            tempo(80, 100),
            tempo(96, 105),
            tempo(112, 101),
        ]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "A4:6 G4:2 - D5 -:1 B4 C5:2 | D5:5 C5:1 C5 B4 Bb4:2 A4 G#4:1 A4 C5:2 | \
            _ Bb4:1 -:2 Bb4 A4:1 A4:3 - G4:1 A4 | G4:2 F4:1 F4 E4 G4:5 F4:2 D4:1 E4 D4:2 | \
            -:5 C4:11 | -:2 C4 Eb4:7 -:1 D4 C#4 D4 - | \
            Bb4:2 A4 G4:1 F#4 - A4 G4 - F#4 G4 Bb4 D5 F5:2 | _:3 Db4:2 Db5 -:1 C5:2 A4 C5 D5:1 -",
            "C7/E D7/C | Dm7/F A7 | Gm Bbdim7 | F/A G#dim7 | C7 Am | B7b9 D7b9 D D | D#dim7 Gm7 | \
            Bbm F/A",
        )
        .in_key(F, Major)
        .with_left(
            "Bb3:3 -:1 Bb3:4 C3:6 -:2 | F2:6 -:2 A2:6 -:2 | G2:5 -:3 Bb2+Db3:7 G4:1 | \
            A2:7 -:1 G#2:8 | G2:3 Bb3:2 -:11 | B3:3 -:1 D2:3 -:1 D3 - F#3:3 - | \
            D#2 -:1 D2+C4 C4 - G2:3 D3:1 -:3 F4:2 | _:6 -:2 F4:6 -:2 & -:2 E3:6 -:2 C5:3 - | \
            -:2 C3:5 -:3 C#3:4 -:2 | - D3:3 - Db4 -:2 F3 -:1 | -:2 F3:14 | -:4 B3:1 -:11 | \
            -:6 A2:2 -:3 A3:1 -:4 | C4+F#4:2 C4:1 -:4 Bb3:1 -:8 | Bb3:9 -:7 & -:6 E4:2 -:8 | \
            -:4 F3 - G3 | _:2 - Bb2:1 -:5 G3:3 - | -:16 | - | - | - | -:8 A3:5 -:3 & -:16 | - | \
            - | - | - | - | - | -:8 C4:4 -",
        )
        .with_tempo(&[
            tempo(0, 96),
            tempo(16, 93),
            tempo(32, 94),
            tempo(48, 111),
            tempo(64, 108),
            tempo(80, 106),
            tempo(96, 108),
            tempo(112, 94),
        ]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "A4:3 - G4 - F4:4 | -:6 C4:1 - C4 F4 C5 D5 F5 A5 C6 C6 | \
            D6:2 F6:1 A6:2 C7:1 D7:2 G7:1 -:7 | -:16 | -:8 E4:1 F4 - G4:5 | \
            -:2 G4 -:1 A4:4 -:1 B4 A4 C5:4 | _:1 F#4 G4:4 A4:2 G4 F4:1 A4 G4:4 | \
            _:12 D4:1 F4 E4:2",
            "G7 Gdim7 C7 F | Dm7/F Dm7 | Dm7 | Dm7 | Dm7 Dm7 Em C | C F F C/E | F | C/G Em/G",
        )
        .in_key(C, Major)
        .with_left(
            "G2:6 C3:7 -:3 | C3:4 F3:2 -:1 A3 -:8 | -:16 | - | -:12 C3+E3+G3:4 | \
            _:1 -:5 F4 -:1 E4:3 -:1 | F4:2 - F2:10 G2:2 | _:7 -:1 G2:6 -:2 & F4:4 Db3:3 -:5 F2:4 | \
            _ -:12 | -:16 | - | - | -:6 F2+C3:4 - C3:2 | -:6 C4:5 -:1 C4:4 | \
            _:8 F4:2 E4 -:4 & D3+B3:3 - Bb3+E4 -:7 | -:2 D3 -:12 | -:16 | - | - | -:14 G3:1 - | \
            -:6 F4:2 -:8 | -:1 E3:6 -:9 & -:16 | - | - | - | - | - | - | -:3 E4:5 -:8",
        )
        .with_tempo(&[
            tempo(0, 98),
            tempo(16, 101),
            tempo(32, 97),
            tempo(48, 100),
            tempo(64, 102),
            tempo(96, 105),
            tempo(112, 101),
        ]),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "-:2 D4:1 Db4 C4:8 E4:1 F4 G4:2 | -:4 G5:1 -:3 A5:1 -:3 A6:1 -:3 | \
            B5:4 Ab5:1 G5 G5:7 A5:1 B5 C6 | B5:2 - F#6:1 - G5:4 A5:3 -:1 G5 F#5 | \
            G5:6 E5:2 -:4 G4:1 A4 G4:2 | _ - F4:1 - C5:10 | -:8 E4:1 - E4 -:3 F4:1 E4 | \
            E4:2 - D4:1 C4 -:8 C4:1 D4",
            "D7/C C | C F#dim7 | Em | B D#dim7 | Em | Fm/Ab | C Fmaj7 | D7 D",
        )
        .in_key(C, Major)
        .with_left(
            "C3:4 -:2 F3:1 F#3 G3:4 -:2 C4+E4 | C3:8 F#2:3 -:1 F#2:3 -:1 | E2:4 - E3:6 G3:1 - | \
            D#2:3 -:1 F#5 -:3 D#3+A3:5 -:3 | E2:4 B4:2 A3:1 A#3 B3:4 - | \
            Ab2:6 -:2 Ab2:5 G4:1 F4:2 | _:1 - G2:4 G3:2 - E3:1 -:5 | \
            D2:2 F#2:1 -:3 A3:4 B3:1 -:5 & -:10 C3+E3:3 - | \
            C2:2 - C2:4 C5+F#5:1 -:3 C6+F#6:1 -:3 | B4:1 - Bb5 A5 -:6 B3:4 -:2 | \
            - D#3:1 -:9 F#3:1 -:3 | -:2 B2 E3:1 -:5 E3+G3:3 - | F3:5 - F3:2 F4:1 -:3 | C2:6 -:10 | \
            -:8 D3+F#3:6 -:2 & -:16 | -:4 C5+E5:1 -:11 | -:12 E4:1 -:3 | -:16 | - | \
            -:2 C3:4 -:10 | -:4 C3:2 -:10 | -:16",
        )
        .with_tempo(&[
            tempo(0, 103),
            tempo(16, 99),
            tempo(32, 95),
            tempo(48, 92),
            tempo(64, 102),
            tempo(96, 109),
            tempo(112, 100),
        ]),
        // Bars 57-64.
        Part::new(
            "Part 8",
            "G4:9 -:3 E4:1 F4 G4:2 | -:4 C4:1 - G4 - A4:3 -:1 B4 A4 C5:2 | \
            _:5 -:1 G4:2 A4 G4:1 - F4 A4 G4:2 | _:8 D4:2 C4:4 -:1 D4 | \
            C4:2 -:4 G6:2 Ab6 C7:4 Ab6:1 F6 | G6:6 E6:2 F6 G#6 F6:1 D6 E6:2 | \
            G4 C5 E5 E6 F6 G#6 F6:1 D6 E6:2 | _ C6 G5 F6:1 - E6 -:3 D6:1 -:3",
            "G7 | C Fdim7 | C/E F | C/E Ddim7 | C Fm | Em G#dim7 | C G#dim7 | C G7",
        )
        .in_key(C, Major)
        .with_left(
            "G2:12 -:4 | C2:1 -:3 E3:1 -:3 F4 -:1 D4+F4 F4 E4:2 | _:4 G2 F2 E3+C4:1 - C4:2 | \
            _:5 -:1 E4:2 D3+F3+Ab3:8 | E3+G3:3 -:5 F4 -:3 | -:2 E5 -:4 G#4:3 -:5 | \
            -:8 G#4+D5:4 - | E4+C5:2 G4:1 -:5 G2+G3 -:2 B2:1 & F3:11 -:5 | \
            -:10 F1+Ab1:2 - B4:1 - | E2:7 A2:1 C4:3 -:5 | E3 -:8 B3:2 -:1 | -:10 C5:3 - | \
            -:8 F5:2 D5:1 -:5 | -:16 | -:8 F5+B5:1 -:3 A2:4 & B3:2 E4:1 Eb4 D4:5 -:7 | \
            -:10 G1:1 -:5 | -:2 C3:6 F4:2 -:6 | -:2 G3:6 -:8 | -:12 Ab4:2 - | -:16 | - | \
            -:12 F5+B5:1 -:3 & -:6 B2:5 - | -:16 | -:6 E4:1 -:9 | -:16 | - | - | - | \
            - & -:8 D3:3 -:5 | -:16 | - | - | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 95),
            tempo(16, 101),
            tempo(48, 95),
            tempo(64, 91),
            tempo(80, 99),
            tempo(96, 104),
            tempo(112, 100),
        ]),
        // Bars 65-72.
        Part::new(
            "Part 9",
            "C6:1 - F4 F#4 G4:4 G5:3 -:1 G5 -:2 G5:1 | F5 F5:9 E5:1 - E5:4 | \
            C4:1 D5 D5 E5 D5:6 C5:2 -:1 G4 G5 - | F#5 G5 F5:7 -:1 E5 - F5:2 E5 | \
            -:1 D5 - E5 F5:3 -:2 A4:1 -:2 A4:1 -:2 A4:1 | A4 G4 G4:6 F4:1 E4 D4:4 E4:2 | \
            F4:1 D4 C4:4 -:10 | -:1 D4 -:4 A4:2 - A4:1 A#4 B4:2 -:1 B4",
            "C Fm7b5 | F/C G7 C E7b9/G# | G7/F Em7 C/E C/E | C C C Eb7/Db | Ebm7/Db Dm F A7/C# | \
            Em7 E7 | Dm7b5/Ab G G7/F G7/F | G7/B G7/B A7b9 B/D#",
        )
        .in_key(C, Major)
        .with_left(
            "C3:2 -:6 F3+B3+Eb4 -:2 | -:1 C2:3 G2:6 -:2 G#2 F3 | _:4 E3:12 | \
            _:1 -:3 C2:6 -:2 Db2+Db3:4 | _:2 - D2+D3:4 -:1 F4 -:2 C#3:4 | F4:1 E4 E4:6 -:4 G#2 | \
            _:5 -:1 B3:4 G3:3 -:1 G3:2 | B1:4 G1 G2:1 -:3 D#2:4 & E5:1 -:15 | -:7 E3:1 - C2 -:6 | \
            B3:4 -:2 G3:10 | _:1 -:5 G2:4 -:2 Eb2:4 | _:1 -:3 A4 -:5 G3:3 F4:1 | -:4 D2:6 - | \
            -:4 G2 F2 D2 | _:1 -:5 F4:2 -:8 & -:16 | - | -:9 C4:7 | _:1 -:7 C3:2 - G4+Db5:3 -:1 | \
            Gb2:2 -:10 F4:1 -:3 | -:6 D3 -:4 | -:12 F#3:1 -:3 | B3:2 -:4 E4+G4:1 -:9",
        )
        .with_tempo(&[
            tempo(0, 103),
            tempo(16, 98),
            tempo(32, 103),
            tempo(48, 101),
            tempo(64, 92),
            tempo(80, 100),
            tempo(96, 105),
            tempo(112, 103),
        ]),
        // Bars 73-81.
        Part::new(
            "Part 10",
            "-:1 B4 - B4 G4 -:6 B4:1 G5:2 -:1 G5 | -:2 G5:1 F5 F5:6 E5:1 - E5:3 -:1 | \
            D5 D5 E5 E4:3 -:2 C5 E4 G4 F#4:1 G4 | Ab4 A4 Bb4:5 -:1 Bb4 - Bb4:2 A4:1 - C5 Bb4 | \
            A4:3 -:1 A4 Bb4 G#4 - A4 - G#4:2 A4:1 - B4 A4 | C5:8 B4:1 A4 G#4 - A4 - B4 A4 | \
            C5:4 G4:1 -:3 G4:2 G5 G5:1 - G5:2 | F5:1 - F5:2 C4:1 -:3 E5:2 - E5 D5:1 - | \
            -:2 D5:1 Db5 C5 -:3 G6:1 -:3 C7:1 -:3",
            "B C/E G7/F G7/F | C/G C/G C G#dim7 | G7 C C C | Bb7 Edim7 C7 C7 | F F Amaj7 B7 | \
            D7/C D7/C E F#dim7 | C7/E A7 | Dm7 Dm7 G7/F G7 | G7 C G C",
        )
        .in_key(C, Major)
        .with_left(
            "D#3:1 -:3 E2+C3:2 -:4 F3:6 | -:4 C2:5 -:1 C2+G4 - G#2:4 | F4:1 F4 D5:8 C4:3 - | \
            -:4 E3+Bb3:2 G3:1 - E4 - C2:4 -:2 | F4:6 A3+C4:1 -:9 | \
            C4:4 F#2:1 -:7 F#3+C4+Eb4:3 -:1 | E4:4 -:2 G3:1 -:3 C#5:2 C#5:1 - C#5:2 | \
            _:1 - A4:2 D3:1 -:3 F4+B4 -:1 G2:4 | _:5 -:3 G2:4 C2:2 - & -:11 G4:5 | \
            _:9 -:3 F4:2 F3 | _ - C3:3 -:9 | -:10 E4:1 - E4 -:3 | -:2 F2:1 -:2 A4:1 -:10 | -:16 | \
            -:2 E3+C4:4 -:10 | -:12 F3:4 | _:2 F4:1 - C3:2 - B5:1 -:3 E6:1 -:3 & -:11 B3:5 | \
            -:6 G2:5 -:4 G2:1 | _ -:3 G3:2 E3:1 -:9 | -:16 | - | - | -:2 Bb3:1 -:13 | \
            -:12 F4+B4:2 - | D3:4 -:12 & -:11 Eb4:4 -:1 | -:8 C3:3 -:5 | -:16 | - | - | - | - | \
            - | - & -:12 Eb5:2 - | -:16 | - | - | - | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 105),
            tempo(16, 94),
            tempo(32, 100),
            tempo(48, 106),
            tempo(96, 94),
            tempo(112, 103),
            tempo(128, 100),
        ]),
    ],
    form: &[
        "Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8", "Part 9",
        "Part 10",
    ],
    drums_from: 0,
    about: "\"Vuelvo vencido a la casita de mis viejos\": a son comes home beaten after years \
            away, to the little house where his old parents live. F minor; this performance \
            turns to F major and ends in C major.",
};

// Copyright: public domain in Argentina, where a work is protected for 70 years after its author's
// death: Maderna died in 1951.
// Written out from a piano performance (MIDI in 2/4, here two bars to a bar of 4/4).
pub(super) const LLUVIA_DE_ESTRELLAS: Song = Song {
    name: "Lluvia de estrellas",
    origin: "Osmar Maderna, 1948",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: E,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 79.2,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "D4:4 G4:1 A4 C5 D5 A5:6 -:2 | F5:1 E5 D5 C5 B4 G4 F4 C4 E4:5 -:3 | \
            D4:5 C4:1 G4 C5 D5 -:7 | B4:5 -:3 B4:1 B4 A#4 B4 A4 B4 G#4 B4 | \
            G4 B4 F#4 B4 F4 B4 E4 B4 D#4 B4 D4 B4 C#4 B4 C4 B4 | -:16 | \
            -:9 C4:1 C#4 D4 Eb4 D4 Db4 C4 | - C4 C#4 D4 Eb4 D4 Db4 C4 - C#4 D4 Eb4 - D4 C#4 -",
            "G7/F Edim7 | Dm7 G# | G7 F#m7b5 | B7 E | Em B | A#7b9 A#7b9 Gmaj7 E7b9 | B C#7 Bm Bm | \
            Bm7/A G",
        )
        .with_left("F3+G3+B3:6 B4:1 - E3+Bb3+Db4:6 -:2 | D3+A3+C4:4 A4:1 - D4+E4 A3 G#2+C4:5 -:3 | \
            G2:5 -:1 D4+E4+F4 A4+B4 F#3:8 | _:1 -:15 | -:16 | \
            B3:1 - A#3+B3 B3 A3 B3 G#3 B3 G3 B3 F#3 B3 F3 B3 E3 B3 | \
            D#3 B3 D3 B3 C#3 B3 C3 B3 B2:8 | A2:9 C4:1 -:4 C4:1 F#2+B3 & -:8 G3:5 -:3 | \
            -:1 F3:3 -:12 | -:8 A3+E4 | B3+D#4+A4:5 -:11 | -:16 | - | -:8 B3:1 -:7 | \
            B3:1 -:7 G2:8 & -:16 | - | -:8 C4:7 -:1 | -:16 | - | - | - | -:8 B3:1 -:7")
        .with_tempo(&[tempo(16, 82), tempo(32, 93), tempo(48, 87), tempo(64, 146), tempo(80, 132), tempo(96, 133), tempo(112, 132)]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "C4:2 D4:1 - D#4 Db4 C4 - C4 -:4 G4:1 F#4 - | \
            C4 -:3 G4:1 F#4 E4 C4 D4 C4 - C4 A4 G4 F#4:2 | \
            D4:1 C4 - C4 C5 B4 A#4 B4 - C#4 D#4 E4 F#4 G4 A4 B4 | \
            F#4 F4 F#4 B4 A4 G4 - F#4 F#4 E4 D#4 E4 E5 D5 C5 B4 | \
            D5 C5 B4 A4 B4 A4 G4 -:2 C4:1 -:2 G4:1 F#4 E4 - | \
            C4 -:3 G4:1 F#4 E4 C4 D4 C4 - C4 A4 G4 F#4 C4 | \
            D4 C4 - C4 C5 B4 B4:3 -:2 D#4:1 E4 F#4 A4 B4 | _ F#4 F4 F#4 B4 A4 G4 F#4 E4 -:7",
            "C#/E# Bm Em Em | G Am7/G D7 D7 | Bm7 B7 | B7/D# Em7 | B7 Em | Bmaj7 Am7 Am7 Am7 | \
            A#7b9 B7 | B7 Em",
        )
        .with_left("E#2:3 -:1 B1:3 E2:8 G2:1 | _ B3 A#3 B3 - G2:2 A2 A1:3 -:2 C4:1 A2 | \
            _:3 -:4 B2:5 -:3 D#3:1 | _:6 -:2 E3:3 -:5 | B2 -:1 F#4 B3 E2 B3 A#3 B3 -:3 B2+B3:1 | \
            - B3 A#3 B3 -:3 A2:1 -:7 A2+E3:1 | -:2 B3:1 -:3 A#4:2 B2+D#3+A3:1 B3 -:4 G4:1 - | \
            D#3+F#3+A3+B3 -:8 B2:1 A#2 B2 A2+B2 - G2 B2 & -:2 C#4:1 - D4 - D2+Eb2 B3 - B3 A#3 B3 B2:4 | \
            _:1 -:5 G#2:1 -:2 A#1:1 B3 -:5 | -:2 B3:1 -:5 B3:1 D#3+A3:4 -:3 | \
            -:1 A3:7 -:2 G3+B3:1 -:5 | -:2 D#3+F#3+A3:3 -:11 | -:16 | - | - & -:14 E4:1 B3 | \
            -:16 | - | -:1 F#3+B3:6 -:9 | -:16 | - | - | -")
        .with_tempo(&[tempo(16, 133), tempo(32, 140), tempo(64, 128), tempo(80, 133), tempo(96, 135), tempo(112, 134)]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "-:8 E6:1 D#6 E6 D#6 E6 D6 C6 B5 | \
            D6 C#6 D6 C#6 D6 C6 B5 A#5 B5 A#5 B5 A#5 B5 A5 G5 F#5 | \
            A5 G#5 A5 G#5 A5 G5 F#5 E5 D5 C5 D5 E5 F#5 E5 D5 C5 | \
            C5 B4 C#5 D#5 E5 F#5 G5 A5 B5 F#5 F5 F#5 G5 A5 D6 C6 | \
            B5 C6 B5 A#5 B5 C6 D6 D#6 E6 D#6 E6 D#6 E6 D6 C6 B5 | \
            D6 C#6 D6 C#6 D6 C6 B5 A#5 B5 A#5 B5 A#5 B5 A5 G5 F#5 | \
            A5 G#5 A5 G#5 A5 G5 F#5 E5 D5 C5 D5 F#5:2 E5:1 D5 C5 | \
            C5:2 B4:1 C#5 E5 F#5 A5 B5:2 -:1 F#5 F5 B5 A5 G5 F#5",
            "B Em E7/G# E7/G# | A B7/D# | E Am | Em D#dim7 | Em7/G E7 | A B7/D# | E Am | Em B7",
        )
        .with_left("F#2:1 B2 F2 B2 E2 -:3 G#3+D4:4 B3:1 -:3 | A2+E3:4 - D#4:5 -:3 | \
            E2:4 - A2:2 E3 C4:3 A3:1 | _ - B3:3 - D#3:5 -:3 | G2:4 - G#2:2 E3:1 - G#3:3 -:1 | \
            A2:2 E3:3 - D#3:6 A3:1 - | E3:5 -:3 A2+E3+A3:2 -:1 E5 -:4 | \
            E3+G3+B3:3 -:1 D#5 - G5 - B2+D#3:8 & -:16 | -:10 F#4:3 - | -:2 B2 -:12 | \
            E3:5 - F#3:3 - | -:2 E3 -:10 D4:1 - | -:4 A3:1 -:5 F#3:4 -:2 | - B3 E4:1 -:11 | \
            -:8 F#3+A3:7 -:1 & -:16 | -:12 A4:1 -:3 | -:16 | -:4 E4:1 -:7 A3:1 -:3 | -:16 | \
            -:12 B3:2 - | -:16 | -:12 F#5:1 -:3")
        .with_tempo(&[tempo(0, 127), tempo(16, 132), tempo(48, 129), tempo(64, 133), tempo(80, 129), tempo(96, 134), tempo(112, 131)]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "E5:1 -:6 D6:1 B5:2 -:5 E5:1 | D5:2 G4:1 -:2 F#4:1 G4 - C5:2 - C4:1 Eb4 F#4 A4 | \
            C5 Eb5 F#5 A5 C6 F#6 A6 C7 B6 -:3 D6:1 C6 B5:2 | _:4 E5:1 G5 E5 G5 B5 -:6 D#4:1 | \
            -:2 D#4:1 F#4 A4 D#5:2 F#5:1 E5:6 F#5:2 | \
            G5:1 - Db4 E4 F#5:2 E5 D5:1 -:2 E5:3 G5:1 B5 | \
            D6:3 -:1 C6 B5 E6:2 -:3 C#4:1 E4 Eb4 D4 C#4 | \
            A4 - E5:3 F#5:1 G5 - A#5 -:2 A#4:1 A#4 -:2 A4:1",
            "Em7 Em7 Gmaj7 C | G Cdim7 | Cdim7 F Em Em | Em B7 | B7 C | Gdim7 G | E7 A | A7 D#7",
        )
        .with_left("E3+G3+B3:1 -:4 D2:3 G2+D5 -:4 C5:1 | B4:2 B3:1 -:5 C3:2 F#3:1 A3 -:4 | \
            -:5 F6:1 -:2 E3:8 | E3:6 -:2 B2:5 -:1 B3 - | B3:4 - C3:2 G3:1 C4 E3:2 C4:1 - | \
            G3:2 - G3+Bb3 Db4+E4:1 - G3+B3:4 - | G#3:3 -:5 A2:3 -:5 | \
            -:2 A2+G3:1 -:5 D#2+C#5:1 -:2 C#4+G4:1 D#3+C#4+G4 -:2 C4+F#4:1 & -:5 E2:1 E2 F2 F#2+G5 -:7 | \
            -:9 Eb3:2 -:5 | -:10 G3+B3:6 | -:2 G3+B3:3 -:4 D#3 -:3 | -:9 E3:2 -:1 C3 G3 -:2 | \
            -:1 Bb3:2 -:5 D4+G4:3 -:5 | E3+B3+D4:2 -:8 E3:1 -:5 | -:16 & -:16 | - | \
            -:12 C6:1 -:3 | -:11 F#3:2 -:3 | -:16 | - | - | - & -:16 | - | - | -:12 A3:1 -:3 | \
            -:16 | - | - | -")
        .with_tempo(&[tempo(0, 114), tempo(16, 125), tempo(32, 126), tempo(48, 123), tempo(64, 119), tempo(80, 121), tempo(96, 119), tempo(112, 117)]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "A4:1 -:3 A5:1 -:2 D6:1 B5 -:5 F5:1 E5 | D5 -:3 F#4:1 G4 C5:2 -:3 E4:1 F#4 A4 C5:2 | \
            B4:1 C4 Eb4 A4 C5 Eb5 F#5 B4:5 -:2 D5:1 C5 | B4:4 E4:1 G4 E4 G4 B4:6 Ab4:1 G4 | \
            -:6 F6:1 C6 -:4 F#5:1 - G5:2 | _ F#5 E5:1 C#5 D5:3 -:1 E5:4 G5:1 B5 | \
            D6:2 D4:1 - C6 B5 E6:2 G4 E5 -:1 F#5 G5:2 | Bb5:1 -:3 B5:1 -:3 G5:2 -:4 D4:2",
            "D7 Em7/G | D7 F#m7b5 | F#dim7 Em | Em/B B7 | B7 C C C | A#dim7 Em7 | E7 A7 | \
            Eb7 D7 G G",
        )
        .with_left("D2+D3+C4+F#4:1 -:3 F#4+C5+F#5:1 -:3 G2:4 -:2 E5:1 - | -:7 C3:2 F#3:1 A3:4 -:1 F#3 | \
            _:4 -:3 E2:4 -:1 E3:4 | _ -:6 B2+D#3+A3 | _:2 -:4 E6:1 E5:3 G2:1 -:5 | \
            A#3+C#4:3 - C5:1 - G2+D3 -:7 | D5:2 E3+G#3:1 -:5 A3+C#4:2 -:6 | \
            Eb3:5 -:2 C#3:1 G2:3 -:5 & -:8 D5:2 D3:3 - | -:8 E3:2 -:6 | A3:3 -:6 B2:2 -:3 B2:2 | \
            _:6 -:9 F#4:1 | -:7 C2+C3:2 -:7 | G4:2 E4:1 -:13 | -:16 | \
            Db5:1 -:3 D3:4 B4+D5:2 -:6 & -:8 G5:1 -:7 | -:16 | -:14 C5:1 - | -:2 G3:4 -:10 | \
            -:7 F5:1 -:8 | -:16 | - | -:4 C5+F#5:1 -:11")
        .with_tempo(&[tempo(0, 105), tempo(16, 114), tempo(32, 124), tempo(48, 116), tempo(64, 121), tempo(112, 118)]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "E4:2 F#4 G4 B4 D5 -:6 | G5:2 -:1 C4 F#5:2 E5 D5 -:6 | \
            C5:4 B4:2 A4 G4:1 -:2 E4:1 F#5 - G5:2 | B5:3 -:1 A5 -:3 A5:1 -:2 C4:1 - F#4 A4 D5 | \
            D4 F#4 C5 E5 F#5 G5 B5 C6 D6:2 -:6 | G5:2 -:1 C4 F#5:7 -:1 D5:2 D#5:1 - | \
            E5 -:2 D4:1 D5 -:3 C5 -:1 B4 - A4:2 | D5:1 -:3 F#4 -:1 G4:3 -:5",
            "F#7 G G G | C G | B7 Em7 | A7 D7 | D7 G | C B7 | E7 Am | D7 G",
        )
        .in_key(G, Major)
        .with_left("-:8 G2:1 B2:2 G3:1 G2:2 D3+G3:1 - | \
            C3 E3:2 -:1 E3:2 G3+C4:1 - G2:2 D3:1 G3 B2:2 D3:1 G3 | \
            B2:2 F#3:1 A3 B2:2 F#3:1 A3 D3+E3 G3:2 -:1 G3:2 - | \
            C#3 G3:1 A3 C#3:3 G3+A3:1 D3:2 F#3+A3:1 -:2 C4+D4:1 -:2 | \
            - A4:1 D5 -:2 A5:1 D3 G2 B2 D3 G3 B2:2 G3:1 - | C3+E3:2 - E3 E5 D#5:4 B2:2 F#3:1 A3 | \
            E3:2 B3:1 - E3:3 -:1 A2+A3:4 E4:3 -:1 | D2:7 Ab2:1 G2:4 - & -:10 D3:1 -:2 B2 -:1 | \
            -:2 G3:1 -:6 B2:2 -:1 G2 -:3 | -:1 D#3:2 - D#3 -:3 B3:1 -:2 B3:1 -:2 | \
            -:1 E3:2 - E3 -:9 | -:13 D3:1 -:2 | -:1 G3 -:3 G3:1 C4 - B2:2 F#3:1 A3 - D#3:2 -:1 | \
            - G#3:2 - G#3 -:1 E4:2 -:6 | D3:5 A2:2 -:1 B3+D4:3 -:5 & -:16 | - | - | - | - | \
            -:9 D#3:2 -:5 | - B3:1 -:10 | -:4 C4:2 -:10")
        .with_tempo(&[tempo(0, 121), tempo(16, 123), tempo(48, 125), tempo(64, 128), tempo(80, 125), tempo(96, 119), tempo(112, 125)]),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "B4:1 B4 A#4 B4 A4 B4 G#4 B4 G4 B4 F#4 B4 F4 B4 E4 B4 | D#4 B4 D4 B4 C#4 B4 C4 B4 -:8 | \
            -:16 | -:1 C4 C#4 D4 Eb4 D4 Db4 C4 - C#4 - D4 Eb4 D4 Db4 C4 | \
            - C4 C#4 D4 Eb4 D4 Db4 C4 C4 C#4 D4 - Eb4 D4 - C4 | \
            C4 -:4 G4:1 F#4:2 C4:1 -:3 G4:1 F#4 E4 C4 | \
            D4 C4 - C4 A4 G4 F#4 - D4 C4 - C4 C5 B4:2 -:1 | \
            - C#4 D#4 E4 F#4 G4 A4 B4 F#4 F4:2 F#4:1 B4 A4 G4 F#4",
            "E Em | B A#7b9 | Gmaj7 E7b9 B C#7 | Bm Bm7/A | G Bm | Em Em/G | Am B7b9/C | B7",
        )
        .with_left("-:16 | -:8 B3:1 B3 A#3 B3 A3 B3 G#3 B3 | \
            G3 B3 F#3 B3 F3 - E3+B3 B3 D#3 B3 D3 B3 C#3 B3 C3 B3 | B2:9 C4:1 -:6 | \
            G2:8 -:3 B1:4 D#2+B3:1 | E2:8 -:1 B3 A#3 E2:3 G#2:1 A2 | _:3 E2:4 C2:5 -:3 B1:1 | \
            _:3 -:4 B2:6 -:3 & -:16 | - | - | B3:1 -:7 A2:8 | B3:2 -:5 F#2:4 -:2 C#4:1 D2:2 | \
            - B3:1 A#3 B2:4 -:3 F#2:2 -:3 | -:2 B3:1 -:3 C4:1 -:3 B3:1 A1:4 B3:1 | \
            _ -:8 D#3+A3:5 -:2 & -:16 | - | - | -:8 B3:1 -:7 | - B3:1 -:8 | \
            -:4 B3:1 - E4 G2:7 -:2 | -:16 | - & -:16 | - | - | - | - | -:7 B3:1 -:3 B3:1 -:4 | \
            -:16 | -")
        .with_tempo(&[tempo(0, 127), tempo(16, 135), tempo(32, 127), tempo(48, 124), tempo(64, 126), tempo(96, 130), tempo(112, 141)]),
        // Bars 57-64.
        Part::new(
            "Part 8",
            "F#4:1 E4 D#4 E4 E5 D5 C5 B4 D5 C5 B4 A4 B4 A4 G4 F#4 | \
            - C4 -:3 G4:1 F#4 E4 - C4 -:2 G4:1 F#4 E4 C4 | \
            D4 C4 - C4 A4 G4 F#4 C4 D4 C4:2 C4:1 C5 B4:3 | \
            -:1 D#4 E4 F#4:2 G4:1 A4 B4 F#4 F4 F#4 B4 A4:2 G4:1 F#4 | \
            -:6 C5:1 B4 B4 B4 C#5 D#5 F#5 G5 B5 G5 | F#5 -:5 C6:1 B5 B5 B5 D#6 E6 G6 - B6 A6 | \
            F#6 -:3 B6:1 -:3 E4:1 -:7 | -:16",
            "Em7 Bm7 | Em G | Am Bm7/A | B7 | Em B7/F# | Em B7 | Em B7 | Em",
        )
        .with_left("E3+G3+B3:1 -:15 | E2:5 -:3 G2:7 A2:1 | _:3 E2:4 A1:6 -:1 B1:2 | \
            _:1 B2+C#4 -:5 B2 -:4 | E3:6 -:1 A#4:2 -:1 A3+B3+D#4:3 -:1 A5:2 | \
            E5 G3+B3:1 -:4 A#5:2 -:1 A3+B3+D#4+C#6 - F#6 - B3 A3+G6 | \
            E3+G3+E6 -:3 B4+B5:1 -:7 B3:2 A2+B2 | -:16 & -:16 | \
            B3:1 - B3 A#3 B2:4 B3:1 - A#3+B3 B3 -:2 G#2:1 - | -:2 B3:1 -:7 B3:1 -:3 A#4:1 B3 | \
            _ -:8 D#3+F#3+A3:4 -:3 | F#3:1 - G3+B3:4 -:2 F#3:5 -:3 | -:1 E3:3 -:4 F#3:3 -:5 | \
            -:12 A#3:1 -:2 E2:1 | _ -:15 & -:16 | -:4 B3:1 -:11 | -:16 | - | -:12 E5:1 -:3 | \
            -:16 | - | -")
        .with_tempo(&[tempo(0, 132), tempo(16, 122), tempo(32, 135), tempo(48, 145), tempo(64, 129), tempo(80, 126), tempo(96, 109), tempo(112, 115)]),
    ],
    form: &["Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8"],
    drums_from: 0,
    about: "\"Shower of stars\": an instrumental tango by Osmar Maderna, the pianist of Miguel \
            Caló's orchestra and later the leader of his own. E minor.",
};

// Copyright: Maciel died in 1962; not public domain; added at the owner's request.
// Written out from a piano performance (MIDI in 3/8, its eighths the quarters of 3/4
// here).
pub(super) const LA_PULPERA_DE_SANTA_LUCIA: Song = Song {
    name: "La pulpera de Santa Lucía",
    origin: "Enrique Maciel, words by Héctor Pedro Blomberg, 1929",
    feel: "Vals criollo",
    genre: Genre::Tango,
    tonic: C,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 195.9,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Vals in minor",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Part 1",
            "C5:2 B4 C5 E5:1 - G5:2 E5 | C5:1 - Db5:2 C5:1 - Bb4:4 G4:2 | \
            F4 E4:1 C4 F4:2 G4:1 Ab4:2 Bb4:3 | C5:4 -:8 | Bb4:2 A4 Bb4:1 - D5:2 F5 D5 | \
            Bb4 C5:6 G4:2 F4 | Eb4 D4 Eb4:1 - F4:2 G4 Ab4 | G4:4 -:8",
            "C | C7 | Fm | Fm | Bb7 | Bb7/Ab | Eb | Eb",
        )
        .with_left(
            "-:12 | E3+Bb3:5 -:2 Ab4:3 -:2 | F3:5 -:7 | F2:4 -:8 | Bb2:4 -:8 | Ab3:6 Ab4:4 -:2 | \
            Eb2:4 Bb2:1 -:7 | -:4 Eb3:1 -:7 & -:12 | C3:4 -:8 | -:12 | -:3 Ab2+C3+F3:1 -:8 | \
            -:3 Ab3:1 -:8 | -:4 Bb4:6 -:2 | -:12 | - & -:12 | - | - | - | - | -:4 Bb3+D4:1 -:7 | \
            -:12 | -",
        )
        .with_tempo(&[
            tempo(12, 197),
            tempo(24, 205),
            tempo(36, 200),
            tempo(48, 206),
            tempo(60, 205),
            tempo(72, 200),
            tempo(84, 193),
        ]),
        // Bars 9-16.
        Part::new(
            "Part 2",
            "F4:2 G4:3 Ab4:1 A#4 - B4 - D5:2 | _:4 F4:1 -:5 G4:2 | C5:9 -:3 | \
            -:4 C6:1 -:3 D6:1 C4 Eb4 G5 | _ Eb6:4 -:2 D6 -:1 C6:2 | -:1 Eb6:2 -:1 D6:2 C6 Eb6:4 | \
            _:1 -:9 Ab5:1 - | -:4 C5 D5:2 F5",
            "Fm | G | Cm | Cm | Cm | Cm | Fm7 | Fm Fm G7",
        )
        .with_left(
            "F#3:2 -:10 | B4:11 -:1 | C3:4 B2:2 C3:3 -:1 Eb2:2 | _:1 -:3 C5:1 -:3 D5:2 -:1 Eb5 | \
            _:2 -:10 | C3:3 -:1 G3 -:7 | F2:4 C3:1 -:5 C5:1 - | F3:5 - B4:2 & -:1 Ab3:5 -:6 | \
            -:1 G3:4 -:7 | -:8 G2:3 -:1 | C2:2 -:9 G4:1 | _ -:11 | -:1 F5+G5:2 -:9 | -:12 | \
            -:4 Ab3+C4:1 -:7 & -:1 F3+C4:4 -:7 | -:4 B3+D4:1 -:7 | -:12 | - | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 196),
            tempo(12, 191),
            tempo(24, 209),
            tempo(36, 189),
            tempo(48, 200),
            tempo(60, 214),
            tempo(72, 191),
            tempo(84, 196),
        ]),
        // Bars 17-24.
        Part::new(
            "Part 3",
            "-:12 | C5:2 -:1 D4 -:2 B4 D5:4 | -:8 G4:1 -:3 | -:2 Eb4:1 Eb4 C4 -:3 F4:1 -:3 | \
            G4:4 F#4:1 -:3 G4:1 -:3 | C5:8 G4:2 Ab4 | F4:4 -:5 D4:1 -:2 | \
            -:3 Bb4:1 -:4 Ab4:1 -:3",
            "G7 | G7/F | Cm | Cm | Cm | Cm | G7/D | G7",
        )
        .with_left(
            "G2:3 -:2 Eb5:5 -:2 | F3:6 -:4 C5:2 | _:9 -:3 | C3:4 - Eb3+G3+C4:1 -:3 | \
            C4:5 -:3 Eb3+G3+C4:1 -:3 | Eb3:4 - Eb3+G3+C4:1 -:3 | D3:9 -:3 | \
            G2 D3+F3+Ab4:1 -:4 F3+G3+B3:1 -:3 & -:2 D3+F3:1 -:4 D5 -:1 | -:3 B3:1 -:8 | \
            Eb2:4 G2:1 -:3 Eb3:1 -:3 | -:2 C4:1 Eb3+G3 -:8 | C3:4 Eb3+G3:1 -:7 | -:3 G3+C4:1 -:8 | \
            -:3 F3+G3+B3:2 -:3 G3+B3:1 -:3 | -:12 & -:12 | - | - | - | Eb4:3 -:9 | -:12 | - | -",
        )
        .with_tempo(&[
            tempo(0, 183),
            tempo(12, 194),
            tempo(24, 195),
            tempo(36, 200),
            tempo(48, 191),
            tempo(60, 198),
            tempo(72, 188),
            tempo(84, 200),
        ]),
        // Bars 25-32.
        Part::new(
            "Part 4",
            "G4:8 -:1 E4:3 | F4:2 - Eb4 -:3 D4 | Eb4:9 -:3 | - C5:1 -:3 D5:1 -:3 Eb5:1 | \
            G5:3 D5:7 C5:2 | Eb5:8 D5:3 C5:1 | Eb5:3 -:5 Ab5:2 - | -:3 C5:4 -:1 D5 -:2 F5:1",
            "G7 | G7 | Cm | Cm | Cm | Cm | Fm7 | Fm Cm G7",
        )
        .with_left(
            "D3:4 -:2 F4 F3+G3+B3:1 -:3 | G2:4 -:3 D3:4 -:1 | G3:7 G2:3 -:2 | C2:1 -:11 | \
            -:1 Eb5:3 -:4 Eb3+G3+C4:1 -:3 | C3 G3+C4:1 -:4 Eb3+G3+C4:1 -:3 | \
            F2 C3+F3:1 -:4 C4+Ab4:1 Ab3 -:2 | F3:4 -:6 D3:2 & -:3 G3+B3:1 -:8 | \
            -:3 B2+D3+F3:1 -:6 Db3:1 - | C3:4 -:1 C3:3 -:1 Eb2:3 | _:1 -:11 | \
            -:1 C3:2 G3+C4:1 -:8 | -:12 | - | -:3 Ab3+C4:1 -:7 B4:1 & -:12 | - | -:3 B2:1 -:8 | \
            -:12 | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 198),
            tempo(12, 193),
            tempo(24, 212),
            tempo(36, 198),
            tempo(48, 204),
            tempo(60, 200),
            tempo(84, 194),
        ]),
        // Bars 33-40.
        Part::new(
            "Part 5",
            "-:6 Eb5:2 D5 - | C5:4 B4:3 D5:4 C5:1 | _:5 -:1 G5 Eb5 G4:2 - | \
            - C5:1 C4 -:4 C5:1 -:3 | C5:5 -:1 Bb4:2 A4:1 -:3 | C5:4 -:2 Bb4:4 -:2 | \
            G4:1 -:8 Eb4:1 -:2 | -:3 C5:1 - C5 -:3 C5:1 -:2",
            "G | G7 | Cm | Cm | Cm7 | Bb7 | Eb | Cm",
        )
        .with_left(
            "G3:4 -:2 B4 - B3 | F3:5 -:6 Eb2:1 | _:3 G2+C3:1 -:3 C5:2 - Eb3:1 | _:4 -:8 | -:12 | \
            Bb2+D4:4 F3:1 - D4:3 - | Eb2+Eb3:5 -:4 G3:1 -:2 | Eb2:1 -:11 & G2:3 D3:1 -:8 | \
            -:3 B3:1 D4 -:7 | -:12 | -:3 G3:1 -:8 | -:12 | Ab3:3 -:5 Ab4:3 -:1 | \
            - F#2 -:2 Bb2:4 - | -:12",
        )
        .with_tempo(&[
            tempo(0, 200),
            tempo(12, 198),
            tempo(24, 204),
            tempo(36, 200),
            tempo(48, 207),
            tempo(60, 197),
            tempo(72, 200),
            tempo(84, 198),
        ]),
        // Bars 41-48.
        Part::new(
            "Part 6",
            "-:1 B4 -:4 G4:1 - G4:2 - | -:1 F4 G4 Eb4 -:4 D4:1 -:3 | D4:5 C4:1 -:6 | \
            -:4 C5:1 -:3 D5:1 -:2 Db5:1 | Eb5 Db5:3 C5:1 B4 C5 - B4:2 C5 | \
            Db5:3 C5:2 B4:1 C5:2 B4 C5 | Db5:4 C5:2 B4 C5:1 -:3 | -:2 C6:1 - C6 -:3 C6:1 -:3",
            "G7 | G7 | Cm | C7 | Eb7/Bb C C | C7 | Fm | Fm",
        )
        .with_left(
            "-:1 D3:4 -:3 F3+G3:1 -:3 | -:2 F4:1 - G2:7 -:1 | C3:4 B2:1 - C3:3 Eb2:2 C2:1 | \
            -:8 C5:1 -:2 Bb3:1 | _:2 - E4:1 -:3 E4+G4:1 -:3 | -:1 E3:5 -:2 Bb3+C4:1 -:3 | \
            -:1 F3:4 -:3 Ab3+C4:1 -:3 | -:4 Ab3+C4:1 -:3 Ab3+C4:1 -:3 & -:4 G3+B3:1 -:7 | \
            -:4 B2:6 -:2 | -:8 G2:3 -:1 | -:12 | - | -:4 Bb3:1 C4 -:6 | -:4 Ab3+C4:1 -:7 | \
            -:12 & -:12 | -:4 D3+F3:1 -:3 D3:2 - | -:12 | - | - | - | - | -",
        )
        .with_tempo(&[
            tempo(0, 207),
            tempo(12, 189),
            tempo(24, 200),
            tempo(60, 204),
            tempo(72, 199),
            tempo(84, 204),
        ]),
        // Bars 49-56.
        Part::new(
            "Part 7",
            "C6:8 C4:1 A5:3 | Bb5:8 C6:2 Ab5 | G5:8 F5:1 -:3 | \
            -:2 Bb4:1 C4 Bb4 -:3 Bb4:1 -:2 C5:1 | -:6 Bb4:1 - Bb4 -:3 | \
            C5:1 -:3 D4:1 - Bb4 Bb4 F4 -:3 | C5:4 - Eb4:1 Bb4 -:2 | - Ab4:1 - Ab4 -:3 Ab4:1 -:3",
            "C7/E | C7/G | Fm | Fm | Bb7 | Bb7 | Cm7 | Fm7",
        )
        .with_left(
            "E3:8 G3+Bb3:1 -:3 | G2:10 -:2 | F2:3 -:1 C3+F3 -:3 Ab3+C4:1 -:2 F3:1 | \
            _:2 -:1 Ab3 -:4 Ab3+C4:1 -:2 Bb2:1 | _:2 -:1 Ab3 -:8 | Ab3:5 -:1 F4+Ab4 -:2 Bb2 -:1 | \
            Eb2+Eb4:4 Bb2:1 -:4 G4:1 -:2 | \
            Eb3:4 Eb4:1 -:3 Eb4:1 -:2 F3:1 & B5:1 -:3 G3+Bb3+C4:1 - Bb5:3 - | \
            -:4 Bb2+C3+E3:2 - Bb2+C3+E3:1 -:3 | -:12 | - | - | F4:1 -:3 Bb3:1 -:7 | \
            -:3 Eb3:1 -:8 | -:2 Eb4:1 Bb3 -:8",
        )
        .with_tempo(&[
            tempo(0, 196),
            tempo(12, 193),
            tempo(24, 189),
            tempo(48, 222),
            tempo(60, 208),
            tempo(72, 204),
            tempo(84, 212),
        ]),
        // Bars 57-64.
        Part::new(
            "Part 8",
            "-:1 Ab4 -:2 G4:1 -:3 G4:1 -:3 | Ab4:1 D4:2 -:1 G4 -:2 G4 -:3 | Ab4:11 -:1 | \
            -:4 B4:1 -:3 C5:1 -:2 Db5:1 | _:2 -:1 E4 -:2 C5:1 - C5 -:3 | \
            -:1 Db5:3 E4:1 - C5 - C5 -:3 | Db5:4 C4:1 -:3 C5:2 - | - C6:1 Ab5 C5:2 - B4:3 -:1",
            "G7 | G | Cm | Cm | C7 | Eb7 C7 C7 | Fm | Fm",
        )
        .with_left(
            "-:1 B2:3 F3+G3+B3+D4:1 -:3 B3+D4+F#4:1 - Eb2 - | G1:4 D2+B3+D4:1 -:2 B3:1 -:2 G2 | \
            C4:8 D3:2 -:1 D3 | _ -:10 C5:1 | _:2 -:1 Bb3+C4 -:4 Bb3+C4+E4:1 -:3 | \
            -:4 Bb3+C4:1 -:3 Bb3+C4+E4:1 -:3 | F3:9 -:3 | -:1 F2:4 -:7 & -:1 B3+D4 -:10 | \
            -:1 G2:4 -:7 | Eb4 -:2 Db3 -:1 | Eb3:2 -:10 | -:12 | - | \
            F4:3 -:1 Ab3 -:3 Ab3+C4+F4+Ab4:1 -:3 | - F5:1 -:8 & -:12 | -:1 B3:2 -:9 | \
            C2:2 G2 C3:4 - | -:12 | - | - | Bb4:1 -:11 | -:12",
        )
        .with_tempo(&[
            tempo(0, 188),
            tempo(12, 200),
            tempo(24, 186),
            tempo(36, 185),
            tempo(48, 241),
            tempo(60, 186),
            tempo(72, 219),
            tempo(84, 198),
        ]),
        // Bars 65-72.
        Part::new(
            "Part 9",
            "-:1 C5:2 -:5 Bb4:2 -:1 A4 | _ Bb4:3 C5:2 -:4 Ab4:2 | _ -:10 | \
            Ab4:2 -:1 Ab4 -:6 Ab4:1 - | Ab4:3 G4:5 B4:4 | D5 C5:5 -:1 B4:2 | C5:7 -:2 G4:1 -:2 | \
            - G4:1 - G4 -:3 Ab4:1 -:3",
            "C7 | C7 | G7 | Fm | G7/D | G7 | Cm | Cm",
        )
        .with_left(
            "-:1 E3:4 -:7 | -:1 C2:4 -:3 G2:1 -:3 | G4:6 -:3 G3 | _:1 -:11 | D3:8 -:2 Db2:1 - | \
            G1:9 -:3 | Eb4:5 -:7 | Eb3:5 -:3 Eb3+G3+C4:1 -:3 & -:4 Bb3:1 -:7 | -:4 E2 - | \
            F2:5 -:7 | Ab3:6 - | -:3 F3:4 -:5 | F4:6 -:2 D2:1 -:3 | C2:4 C3:1 -:7 | \
            -:4 G3+C4:1 -:7 & -:12 | -:4 G2:2 -:6 | -:3 C3:2 -:7 | -:12 | -:4 G3 - | - B1 - | \
            -:3 G2:1 -:8 | -:12 & -:12 | - | -:4 F3:3 -:5 | -:12 | -:4 B3:3 -:5 | -:12 | - | -",
        )
        .with_tempo(&[
            tempo(0, 202),
            tempo(12, 192),
            tempo(24, 197),
            tempo(36, 203),
            tempo(48, 204),
            tempo(84, 202),
        ]),
        // Bars 73-80.
        Part::new(
            "Part 10",
            "G4:1 Ab4 G4:2 F4:1 -:3 E4:1 -:3 | F4:1 -:3 Eb4 - D4:2 | C4:6 - | -:12 | \
            C5:1 - B4 - C5:2 E5 G5 E5 | C5:1 - Db5:2 C5:1 - Bb4:2 Ab4 G4 | F4 E4 F4 G4 Ab4 Bb4 | \
            _:1 C5:5 -:2 C4:1 -:3",
            "G7/D | G7 | Cm | Cm | C7 | C7 | Fm | Fm",
        )
        .with_left(
            "-:1 D3:11 | -:1 D3:4 -:7 | C3:4 B2:1 - C3:2 - Eb2 | _:1 -:11 | Bb3:3 -:9 | C3:4 -:8 | \
            F3:4 Ab3:1 -:7 | -:1 F2:3 C3:1 -:5 F2:2 & -:4 F3+B3:8 | _:4 -:8 | -:7 G2:3 -:2 | \
            C2 -:10 | G3+E4:2 -:10 | -:3 E3:2 -:7 | -:12 | - & -:4 G3:8 | _:3 -:9 | -:12 | - | - | \
            - | - | -",
        )
        .with_tempo(&[
            tempo(0, 198),
            tempo(12, 212),
            tempo(24, 216),
            tempo(36, 195),
            tempo(48, 202),
            tempo(60, 204),
            tempo(72, 234),
            tempo(84, 189),
        ]),
        // Bars 81-88.
        Part::new(
            "Part 11",
            "Bb4:2 A4 Bb4:1 - D5:2 F5 D5:1 - | Bb4:2 C5:6 G4:1 - F4:2 | \
            Eb4 D4 Eb4:1 - F4:2 G4 Ab4 | Bb4 -:10 | G4:1 Ab4 G4:2 F#4:1 G4:2 -:4 B4:1 | \
            _ G4:2 Ab4:3 F4:1 - Eb4:2 D4 | C4 - C4:1 Eb4 -:2 F4:3 F#4:1 | G4:3 -:9",
            "Bb | Bb7/F | Fm7 | Bb | G | G | Cm | Cm",
        )
        .with_left(
            "Bb2:3 -:9 | F3:6 Ab4:3 - | Eb2 -:9 | -:12 | -:3 B3 -:6 | -:2 G2:10 | -:1 C3:10 -:1 | \
            -:4 C2:1 -:7 & Bb1:2 -:10 | -:4 Bb4 - | -:12 | - | -:4 D3:1 -:7 | \
            -:2 G3 -:1 G4:2 - Db3:3 | -:2 B3 -:1 G2:7 | -:12 & -:12 | -:4 Ab3+Bb3:2 -:6 | -:12 | \
            - | - | -:6 B2 | -:9 Eb2:3 | _:2 -:10 & -:12 | -:4 D4:1 -:7 | -:12 | - | - | - | - | \
            -",
        )
        .with_tempo(&[
            tempo(0, 200),
            tempo(12, 214),
            tempo(24, 191),
            tempo(36, 222),
            tempo(48, 196),
            tempo(60, 189),
            tempo(72, 236),
            tempo(84, 196),
        ]),
        // Bars 89-92.
        Part::new(
            "Part 12",
            "F4:1 Bb4 G4:2 Ab4:1 Bb4 - C5 - D5:3 | B4:2 -:7 D5:1 - C5 | \
            - D5 C5 B4 - C5:2 Eb5:1 - G5:2 C6:1 | _ -:11",
            "Fm | G | Cm | Cm",
        )
        .with_left(
            "Ab3:6 - | G2:12 | _:3 B2:1 - C3:2 G2 Eb2:3 | -:12 & -:1 F3+C4:5 -:6 | G3:11 -:1 | \
            C3:3 -:8 C2:1 | _:3 -:9 & -:1 D4:4 -:7 | -:10 B2:2 | -:12 | - & -:1 F#4 -:10 | -:12 | \
            - | -",
        )
        .with_tempo(&[tempo(12, 197), tempo(24, 220), tempo(36, 173)]),
    ],
    form: &[
        "Part 1", "Part 2", "Part 3", "Part 4", "Part 5", "Part 6", "Part 7", "Part 8", "Part 9",
        "Part 10", "Part 11", "Part 12",
    ],
    drums_from: 0,
    about: "\"Era rubia y sus ojos celestes reflejaban la gloria del día\": a vals criollo \
            about the fair-haired keeper of a country store in Buenos Aires in the days of \
            Rosas, whom every gaucho loved. Ignacio Corsini's record of 1929 made it a hit. C \
            minor.",
};

// Copyright: Demare died in 1974; not public domain; added at the owner's request.
// Written out from an arrangement (MIDI in 4/4): the tune on the piano, with bandoneón,
// guitar, double bass and strings.
pub(super) const MALENA: Song = Song {
    name: "Malena",
    origin: "Lucio Demare, words by Homero Manzi, 1942",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: C,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 89.6,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: SOLO_PIANO,
    style: "Tango",
    parts: &[
        // Bars 1-8.
        Part::new(
            "Verse a",
            "-:3 G4 C5:2 -:1 C5 -:2 Eb5:1 Eb5 C#5 D5 | _:3 -:5 F4:2 F5 -:1 Ab5 - D6 | \
            C6:7 G5:5 Ab5:1 - Bb5 Ab5 | G5 B5:13 D5:2 | \
            _ Eb5:1 F5 G5 - F5 F#5 G5 - G4:2 C6:1 - G5 - | Eb6 -:4 D6:1 -:3 D7:1 -:3 D6:1 - F4 | \
            _ Ab4 D5 C5:3 - C4:2 Eb4:1 -:2 Eb4:1 - | Ab4 - Ab4 - G4:12",
            "Cm Cm Cm Fm7 | Fm Fm7/Eb | Ebmaj7/D Cm | G7/B G7 | G7/B Cm Cm Cm | Fm7/C | \
            Fm7/Ab Cm/G | G7",
        )
        .with_left("-:6 Eb3+G3+A3:4 -:5 Ab3+C4+D4:1 | _:3 -:5 Ab3+D4:4 - | \
            C4+Eb4+G4+A4:7 -:3 C4+Eb4+G4:4 -:2 | -:3 G3+B3:6 -:3 F3:4 | \
            _:1 -:3 Eb3+G3+C4 - C4+F#4 - | C2:4 E2:1 F2:5 -:3 Ab1 | _:5 C2:2 G1:5 -:1 C2 C4 - | \
            Ab2+F#3:3 Db3:1 G2:12 & -:7 C3:4 - F3:1 | _:2 -:8 C4:2 -:4 | -:16 | \
            -:3 F4:5 G5:6 -:2 | -:12 G5:1 -:3 | Db2:2 -:1 Eb2:2 F5:1 -:3 C2:4 F5:1 -:2 | \
            -:1 D2:2 Eb4:3 -:10 | C4+Eb4:1 - C4+Eb4 - B3:10 -:2 & -:16 | - | - | -:12 B3+D4:4 | \
            -:16 | F2+G5:1 -:8 F6+Ab6:1 -:6 | -:2 A#1 B1:1 -:11 | -:4 F3:6 E3:2 -:1 D4:3 & -:16 | \
            - | - | - | - | - | - | -:10 Eb3:1 D3:5")
        .with_lines(&[
            line(
                "Bandoneón",
                gm::TANGO_ACCORDION,
                "-:15 F3:1 | _:15 B4:1 | _ D5:6 -:1 C5:8 | _:9 B4:3 - G4:1 | \
                _:5 -:1 G4 -:3 D5:1 C5+D5 -:2 Eb4 | \
                -:4 F3:3 -:1 Ab3+D5 -:3 F3+Ab3+C4+F4+Ab4+D5:1 -:3 | \
                F3+Ab3+C4+F4+Ab4+C5:1 -:3 G3+C4+Eb4:1 - G4+C5+Eb5 - D5 - C5 - G3+C4+Eb4+G4+C5+Eb5 -:3 | \
                G4+C5:1 -:3 G3:2 D4+G4+B4:1 -:3 G4:1 F4+Gb4 Eb4:2 - & -:15 F4:1 | _:9 C5:7 | \
                -:6 G3:9 -:1 | -:4 D3:12 | _:1 -:3 C4:11 -:1 | -:16 | - | \
                -:5 D3+B3:1 -:6 D3+F#3:1 -:3 & -:16 | -:6 G4:2 -:8 | -:6 Db5:1 -:2 C4:5 -:2 | \
                -:11 D5:5 | _ -:9 C5:1 - | -:16 | - | - & -:16 | -:7 Ab4:1 -:8 | -:11 Eb4:5 | \
                -:16 | - | - | - | -",
            ),
            line(
                "Guitar",
                gm::NYLON_GUITAR,
                "-:13 C4:1 D4 Eb4 | _ -:7 Eb4 C4+Db4:1 | D4:8 C4 | G3:7 A3:1 A#3 B3:6 C4:1 | \
                D4:4 Eb4:5 -:3 C4:4 | D4:1 - Eb4:2 F4:1 -:3 Eb4:1 -:3 D4:1 -:3 | \
                -:2 F4:1 D4+Eb4 Eb4 - D4 -:3 C4:1 - Eb4 -:3 | D4:1 - A3 A#3 B3:4 A3 B3 & -:16 | \
                F4:8 - | -:16 | - | -:2 E4:1 -:5 D4:3 -:5 | -:16 | - | -",
            ),
            line(
                "Contrabass",
                gm::CONTRABASS,
                "-:15 F3:1 | _:8 -:1 Eb3:7 | _:1 -:8 C3:7 | _:1 B2:8 G2:7 | B2:2 C3:11 -:1 C3 - | \
                D3 - E3:2 F3:1 -:3 Eb3:1 -:3 D3:1 -:3 | C3:1 -:3 Eb3:1 -:3 D3:1 -:3 C3:1 -:3 | \
                G2:4 D2:5 -:1 F#2 - G2:4 & -:16 | - | D3:8 - | -:15 A2:1 | -:16 | - | - | -",
            ),
            line(
                "Strings",
                gm::STRING_ENSEMBLE,
                "-:15 D4+Eb4:1 | E4:2 -:10 G4:3 Ab4:1 | _ -:7 Eb4:4 C4 | F4:8 Ab4 | \
                G4:9 C4:3 D4:2 Eb4 | _ - F4:6 G4:2 Ab4:3 C5:1 | _:2 D5 -:3 G4:5 -:1 F4:3 | \
                -:4 D4:9 -:3 & -:16 | -:1 F4:12 -:3 | G4:8 - | -:16 | - | -:1 E4:2 -:13 | \
                -:3 C5:4 - Ab4:2 - G4:1 | _:4 -:6 Eb4:2 F4:4",
            ),
        ])
        .with_tempo(&[tempo(32, 91), tempo(48, 89), tempo(64, 94), tempo(80, 96), tempo(96, 92), tempo(112, 96)]),
        // Bars 9-16.
        Part::new(
            "Verse b",
            "-:1 Eb4 F4 G4:2 -:3 G4:2 Bb4:1 Ab4:2 G4:1 F#4 G4 | Bb4:6 Ab4 F4:1 -:3 | \
            C5:1 B4 C5 B4 Bb4:2 - Bb4 C5 Bb4:1 - A4 Bb4 | D5:9 C5:3 A4:1 -:2 B4:1 | \
            C5 D5 -:3 G6:8 Eb6:1 Eb6 D6 | _:4 -:3 Ab4:4 F4:3 Ab4:1 D5 | \
            C5:3 -:1 C4 -:3 Eb4:1 -:3 Ab5:1 -:3 | G5:1 -:5 F6:1 -:3 G4:1 -:3 D4:1 G4",
            "Ebmaj7 Cm C7/Bb C7/Bb | Fm7 Bb7 | Cm7/Eb Cm7 | D Dm7 | G G/B | Fm7 | Cm Cm7/G | G7/D",
        )
        .with_left("-:2 C3+Eb3:4 -:3 G3:5 -:2 | - F2:5 -:1 F2:3 - Bb2:2 | _ Eb2:5 -:1 Eb2:3 - Eb2:2 | \
            _:1 - D2 - F#4:3 -:1 B2:2 C#3:1 D3:3 -:2 | -:5 G5:7 -:4 | -:1 Ab2:11 -:1 G2:3 | \
            _:1 -:3 C2:1 -:3 Ab2:6 F2:1 F#2 | G2:5 F2:4 -:1 F4:6 & -:3 G3 - Bb3:4 -:3 | \
            -:2 Bb2:1 - C4:2 -:4 C4:1 -:5 | D4:2 -:6 Eb4+G4:2 Eb4:1 -:5 | \
            F#4:4 A2:1 A4:3 -:1 F#4:4 -:3 | -:9 C5+Eb5:3 -:4 | -:1 F3:2 E3:1 Eb3 D3:6 -:2 F3:3 | \
            C3:4 - C4:1 -:3 Eb2 -:1 | B4+D5 -:7 D2 Bb1+Eb4:1 & -:10 Db4+E4:3 - | -:16 | \
            Ab4:1 -:15 | Db5:1 -:5 Bb2:1 -:2 C3+A4:1 -:6 | -:9 C6:1 - C6 -:4 | -:16 | \
            Eb4:3 -:9 C5+Eb5+F#5:1 -:3 | -:12 G1:4")
        .with_lines(&[
            line(
                "Bandoneón",
                gm::TANGO_ACCORDION,
                "D4:1 -:3 C3+C4:4 - E4+G4+Bb4+Db5:1 -:2 E4:1 | _ F3:8 -:1 F3:6 | _:1 - Eb3:14 | \
                _:1 - D3+A4 - Ab4+C5+Eb5 -:3 D3:2 F#4+A4+C5+Eb5:1 -:4 D3+F#3+A3+F4:1 | \
                F#4+G4 G3 G5 - F#5 G5 G6 G5 G6 -:7 | F3:6 G4:1 Ab4 Eb3:7 F4:1 | \
                _:4 -:1 D4:2 Eb4:3 F4 G4 | \
                _:4 D3+F4:1 -:3 D3+G3+Eb4+Eb5:1 -:3 D3+G3+D4+D5:1 -:3 & -:15 Bb4+Db5:1 | \
                C5:6 Bb4:1 - C5:3 - Ab3:2 | F5:1 - C5:2 D5:4 C5:2 G4:6 | _:2 -:14 | -:16 | \
                -:2 F4:3 - G4:5 G4+Ab4:1 F#4 - | C3:2 G3:6 Bb2:7 -:1 | G3 -:15 & -:16 | \
                -:3 C4:6 -:1 Bb3+D4:6 | -:10 Bb3:6 | -:16 | - | -:3 C4:4 -:5 C4:2 - | \
                -:3 C4:2 -:11 | -:16 & -:16 | -:4 Ab3 -:2 F5:5 G5:1 | -:12 Ab3:4 | _:1 -:15 | \
                -:16 | - | - | -",
            ),
            line(
                "Guitar",
                gm::NYLON_GUITAR,
                "-:2 D4 C4:7 -:5 | Eb3:1 F3:7 -:1 Ab3:7 | _:1 Bb3 C4:4 Bb3 D4:3 - | \
                Eb4:1 D4 F4 - Eb4 -:3 D4:1 - C4:2 D4:1 -:2 C4:1 | _ B3:4 -:3 G3:1 -:7 | \
                -:2 F3:4 G3:2 Ab3:8 | C3:2 D3:1 Eb3 G3 C4 D4 D4:5 C4:4 | \
                B3:1 -:4 G3:9 A3:1 B3 & -:16 | -:7 G3:2 -:7 | -:12 C4:1 -:3 | -:16 | \
                -:4 A3:1 -:11 | -:16 | -:7 Eb4:1 -:8 | -:16",
            ),
            line(
                "Contrabass",
                gm::CONTRABASS,
                "-:2 A2:1 B2 C3:6 Bb2:5 -:1 | Eb2 F2:8 Bb2:7 | _:2 Eb2:1 E2 - Eb2:6 -:5 | \
                F2:1 G2:2 -:1 G2 -:3 D2:6 -:1 F2+Gb2 | G2:4 A2 B2 D3 | C3:2 F2:13 C3:1 | \
                _:7 G2:9 | -:2 D2:4 -:1 F2+Gb2 G2:4 A2:2 B2 & -:16 | - | - | -:2 Ab2 -:12 | -:16 | \
                - | - | -:15 C3:1",
            ),
            line(
                "Strings",
                gm::STRING_ENSEMBLE,
                "-:2 Eb4:1 D4:2 - D4:4 E4:5 | _:2 -:7 Eb4:5 Bb4:2 | _:5 -:3 G4:4 F4:2 Eb4 | \
                D4:6 F4:5 Ab4:4 -:1 | G4:8 -:7 D4:1 | F4:12 Eb4:2 D4 | \
                Eb4 C4:3 D4:2 Eb4:3 G4:5 F4:1 | Eb4:5 -:3 C4:4 B3 & -:4 C4:3 -:9 | -:1 F4:9 -:6 | \
                -:4 Ab4:5 -:7 | -:14 D4:2 | -:16 | Eb4+Fb4:1 -:15 | -:16 | -:4 D4 -:8",
            ),
        ])
        .with_tempo(&[tempo(0, 89), tempo(16, 96), tempo(48, 89), tempo(96, 91), tempo(112, 96)]),
        // Bars 17-24.
        Part::new(
            "Refrain a",
            "C4:1 -:6 C4:1 C#4:2 D4:5 E4:1 | F4 - G4:5 -:1 G4 Gb4 F4:2 -:3 A4:1 | \
            G4:2 C5 C4:4 D5 E5:2 D5:1 C5 | E5:9 -:4 C5:2 A4:1 | _:4 -:2 C#4 -:4 B4:1 C5 B4 C5 | \
            D5:6 C5:2 B4 C5 B4:1 A4 C5 - | -:2 F6:1 F#6 G6:2 G5:1 -:5 F5:2 E5:1 - | \
            D5 -:5 D7:1 -:3 F5:1 - D4 E4 D4 E4",
            "D7 D7 C/G G7 | C C C Dm7 | C Am7 | Em7/G Em | Dm7 G7/D | Dm7 B7 | C Em | Dm Fmaj7",
        )
        .in_key(C, Major)
        .with_left("C2:1 -:3 F#3:4 F3:8 | _:2 C3:7 -:1 D3:6 | -:2 E3:7 -:1 A2:6 | -:2 G2:6 -:1 E2:7 | \
            -:1 F2:5 C4:1 - A2+G3:5 -:3 | D3+F3+A3:7 -:3 D#3+C4:6 | E3:2 -:4 F#5:1 -:5 A2:4 | \
            D3 A2 F2:8 & -:4 Ab2:3 -:1 G2:7 -:1 | -:4 E3:6 -:2 F3:4 | _:2 -:4 G3:2 -:4 E3 | \
            -:3 E3:4 B4:3 -:1 B2:5 | _:1 -:2 C3:3 -:6 E3:1 -:3 | F4:6 -:4 F#3:5 -:1 | \
            -:12 G4+C#5:3 C#3:1 | F4+A4 -:5 F6+A6:1 -:5 A1:4 & -:6 B3:2 -:5 D3:3 | \
            _:1 -:5 B3:2 -:6 A3:2 | _ -:12 A3:1 - | -:5 G4:4 -:5 B4:1 - | -:5 B3:1 -:10 | -:16 | \
            -:12 Bb2:1 - C3:2 | -:16")
        .with_lines(&[
            line(
                "Bandoneón",
                gm::TANGO_ACCORDION,
                "C3+C4+C5:1 -:7 G3:1 -:7 | C3:9 B4:2 -:4 D5:1 | D#5 E5:8 D5:7 | -:1 B4:13 -:2 | \
                -:1 F3:3 A4:1 C5 F5+G5 - G5:6 F5:2 | - A4:8 -:5 C5:1 | \
                _:6 C3+D5:1 -:3 C5:1 -:3 C3:1 - | \
                -:4 D3+G4+B4+D5:1 -:3 G3+D5:1 -:3 D5:1 - E5 - & -:16 | -:3 G4:5 -:2 C5:6 | \
                -:1 C3:3 G4:8 -:3 C5:1 | _ -:6 E3:3 -:6 | -:2 A4+B4:1 G#4 -:11 E5:1 | _:8 - | \
                -:16 | - & -:16 | -:7 A4:2 -:7 | -:4 G3:3 -:2 C3 -:5 | -:9 B3:2 -:5 | -:16 | \
                -:7 D5:9 | -:16 | -",
            ),
            line(
                "Guitar",
                gm::NYLON_GUITAR,
                "C4:8 - | -:3 C4:4 -:1 D4 D#4 E4:5 F4:1 | F#4 G4:13 -:2 | \
                -:3 E4:4 D#4+E4:1 B4:6 A4:2 | _:6 G4:1 A4 F4:4 E4 | D4:7 -:1 F4:7 D4:1 | \
                - E4:3 -:4 B3:7 -:1 | C4 -:3 D4:1 -:3 C4:1 -:3 A3 -:1 & -:16 | - | - | - | \
                -:8 G4:1 -:7 | -:6 E4:2 -:8 | -:3 C4:5 -:8 | -:14 Bb3:1 -",
            ),
            line(
                "Contrabass",
                gm::CONTRABASS,
                "-:6 G2:3 -:7 | -:1 C3:8 D3:7 | _:1 E3:8 -:1 D#3:6 | _:1 E3:7 E2:4 Eb2 | \
                D2:12 F2:2 G2 | _:7 -:1 B2:8 | C3:7 -:1 E3:8 | \
                B2:1 -:7 E3:1 -:3 D3:1 -:3 & -:8 C3:7 -:1 | -:16 | - | - | -:12 E2:1 -:3 | \
                -:6 A2:2 -:8 | -:6 D3:2 -:8 | -:16",
            ),
            line(
                "Strings",
                gm::STRING_ENSEMBLE,
                "C4:4 D4 E4:5 -:3 | G4:9 -:3 F4 G4:1 | _:7 C5:9 | E5:1 D5:6 -:1 G4:8 | \
                A4:11 -:3 D5:2 | _:4 B4:1 - C5:6 A4:3 -:1 | -:2 C5:5 - F4:3 -:1 | \
                D4:8 E4:5 -:1 F#4:2 & -:12 F4:4 | -:8 E4:4 -:3 F#4:1 | -:16 | -:6 B4:2 -:8 | \
                -:10 B4:4 -:2 | -:3 C5:1 -:7 B4:1 -:2 B4 | _ -:4 G4:5 -:3 E4:2 | -:12 F4:2 -",
            ),
        ])
        .with_tempo(&[tempo(0, 93), tempo(16, 88), tempo(32, 90), tempo(48, 86), tempo(64, 91), tempo(80, 96), tempo(96, 97), tempo(112, 100)]),
        // Bars 25-30.
        Part::new(
            "Refrain b",
            "F4:4 G4:1 G#4 A4:3 -:1 A4 G4 F#4 A4:2 -:1 | A4 G4 G4 C5 D5 E5 - A4:6 F#4:3 | \
            _ E4:1 D#4:2 -:1 F#5 -:3 G4:2 A4:3 | _:1 A#4 B4 C5:4 B4:8 -:1 | \
            G4:3 -:1 G4 G#4 A4:4 G4:1 F#4 G4:4 | F4 E4 G4 F4:1 D4 C4:2",
            "G7/D B7/D# | C C A7/E G7/B | B7 B7 B7/D# D7/F# | B7/F# B7 | Em7 Dm7/A | \
            Dm7/C Em7/D G7 G7",
        )
        .in_key(C, Major)
        .with_left("-:2 D2:6 D#2 E2:2 | C3+G4:1 - F#4 - F4+Gb4 G4 - G4 -:2 G4:3 - | \
            - C3+Bb3:1 B2:4 -:7 F#3+A3:1 | _:6 -:4 B1:3 D#2:1 E2:2 | _:5 -:3 A2+C#3:6 -:2 | \
            A3+D4:5 -:1 G2:8 C3:2 & -:10 G4:1 -:5 | -:16 | -:4 A3:2 -:1 F#4 -:7 Eb4:1 | \
            _:5 B3:1 -:6 A4:3 -:1 | -:8 G3:5 D3:3 | _:9 D3:2 F3:1 D3:2 - & -:16 | - | - | \
            -:12 D2:1 -:3 | -:16 | -:2 D2:4 -:2 B3:4 -:1 C#3 -:2")
        .with_lines(&[
            line(
                "Bandoneón",
                gm::TANGO_ACCORDION,
                "G3:1 - F5 -:3 E5:1 Eb5 D5:6 -:2 | -:4 A4:5 G4:3 F#4:4 | \
                -:2 B3 D#4:3 -:1 B4:2 C5 D5:4 | _:3 -:1 D#5:4 F#5:6 -:2 | \
                G4+E5:1 - G4+Eb5 - G4+E5 -:3 F5:1 - G5 - G#5 - A5 - | \
                G5 - F5+G5 E5+F5 D5 E5 F5 E5 E5+G5 F5 D5 C5 -:2 B4:1 - & -:8 G3:1 -:3 C5:1 B4:3 | \
                _:4 Bb4:1 -:11 | -:6 F#4:2 -:8 | -:2 E5 -:12 | -:16 | -",
            ),
            line(
                "Guitar",
                gm::NYLON_GUITAR,
                "B3:6 C4:1 D4 E4:2 F4:3 G4:1 A4:2 | G4:5 F4:4 E4 -:3 | F#4:4 D#4 B3:7 -:1 | \
                -:4 D#4:6 F#4:1 D#4:2 F#4 -:1 | E4:4 Eb4:1 - D4:8 C4:2 | \
                _:6 B3 C4+D4:1 C4:3 & -:10 E4:1 -:5 | -:12 G4:4 | -:14 C#4:2 | _:4 -:12 | -:16 | \
                -",
            ),
            line(
                "Contrabass",
                gm::CONTRABASS,
                "B2:9 -:5 E3:2 | _:8 -:2 A2:1 A#2 B2:4 | _ D#3:6 E3:2 F#3:4 | _:5 -:4 D#3:5 E3:2 | \
                _:6 D3:8 C3:2 | _:3 -:1 F#2 G2:3 A2:4 B2:2 C3 & A2:1 -:7 D#3:6 -:2 | -:16 | \
                -:2 C#3 -:12 | -:4 E3 -:8 | -:16 | -",
            ),
            line(
                "Strings",
                gm::STRING_ENSEMBLE,
                "G4:7 -:1 A4:5 A4:3 | -:1 G4:9 G4:1 Gb4 F4:2 F#4 | _ G4 A4:5 -:3 C5:4 | \
                _:3 -:1 D5:4 Db5:2 C5 B4:4 | _:5 -:2 C5 -:1 A4:2 G4 F4 | \
                _:3 E4 G4 - D4 C4:1 & -:6 G#4:2 -:4 B4:1 -:3 | -:9 A4:1 -:6 | -:8 B4:4 - | \
                -:2 B4 -:12 | -:4 A4:3 -:1 B4:2 -:6 | -:8 F4:4 -",
            ),
        ])
        .with_tempo(&[tempo(0, 91), tempo(16, 93), tempo(32, 95), tempo(48, 97), tempo(64, 89), tempo(80, 90)]),
        // Bars 31-39.
        Part::new(
            "Last bars",
            "-:4 G6:6 Eb6:1 D6:5 | _:7 F5:3 Ab5:1 - D6 C6:3 | _:4 -:3 G4:2 Ab4:1 Bb4 Ab4 G4 B4:3 | \
            _ G4:1 -:3 D4 Eb4:1 F4 G4 -:3 | Eb6:1 -:3 C5:1 - C5 - Eb5 - Eb5 -:3 G5:1 - | \
            -:3 F6:1 B5 -:3 Eb5:1 D5 - G5 -:3 Eb5:1 | \
            - C4 D5 Eb5 C5 Eb4 Bb4 - Ab4 -:3 Bb4:1 - G4 Ab4 | F#4 - G4:2 G5:1 -:5 D5:1 F5 C5 -:3 | \
            G6:1 -:3 C7:1 -:11",
            "Cm Fm7 | Fm Fm Bb7/Ab Cm/G | Cm/G Cm/G Fm7 G7 | G7 G7 Ebmaj7/D Cm | Cm7/G Fm7 | \
            G7 Bb7/Ab Cm7 Cm7 | G7/F Ebmaj7 Dm7 Dm7 | D/F# G G7 C | G Cm Cm Cm",
        )
        .with_left("C2:1 -:3 G5 -:1 C5+Eb5+C6 -:4 Ab2+F3:3 | _:1 C3 Ab2:4 -:3 Ab2+F3:4 Eb5:3 | \
            _:5 -:4 Ab2+F#3:3 -:1 G2+F3:3 | _:1 -:2 G3:3 - G2 C3:4 | G2:10 Eb2:2 F2:4 | \
            F1:1 -:3 D3:6 -:1 F5:4 -:1 | -:3 G4:2 -:1 D4 - C4 - B3 - C4:2 B3:1 B3+C4+G4 | \
            A#3 - B3:2 -:4 G2 C3:2 -:1 F2+Gb2 | G2+B5+D6 - F#1 - Eb6 -:11 & -:6 C6:1 -:9 | \
            -:3 F3:4 -:6 G2+Eb3:3 | - G2+Eb3:4 -:9 | -:3 G2:2 -:5 Bb2:2 C4+Eb4:1 -:3 | \
            G5+C6:1 -:3 Eb2:4 C2:3 E2:1 -:2 Ab4+C5+F#5:1 - | -:4 F5:1 - F#3:5 G4:3 G3:2 | \
            -:3 D5:1 F4 -:7 Ab4:1 -:3 | -:8 F3:2 B2 -:4 | -:16 & -:16 | - | - | -:11 B2:1 -:4 | \
            - G4:1 - G4 - G4 - G4 -:5 | -:6 Ab3:4 -:2 C3:3 -:1 | -:16 | -:11 Db3+Eb5:1 -:4 | \
            -:16")
        .with_lines(&[
            line(
                "Bandoneón",
                gm::TANGO_ACCORDION,
                "C5:4 -:7 Eb3+D4:1 Eb4+Fb4 F3+F4:3 | \
                _:1 -:3 F3+Ab3+F4+A4+C5:1 -:3 Ab3+C4+F4+Ab4+C5:1 -:2 Ab3+G4:1 C3+C5+Eb5+G5 -:3 | \
                Eb3:4 G3+G4+C5+D5+G5:1 -:3 C4+G4+C5+Eb5:1 -:3 G3 D4+D5:1 | \
                - Ab4:2 F#4:1 G4:8 C3:3 -:1 | D4 Eb4 F4 F#4 G4:4 C5:1 - C4:2 F3 Eb5:1 - | \
                -:2 D5:1 C5 D5:4 F3+Bb3+D4+F5:1 E5 Eb5 D5 Eb3+G3+Bb3+D5 - D5 - | \
                Eb3+G3+Bb3+C5 - Bb4 - Eb3+G3+Bb3+Ab4 - G4+G5 - Eb3+F4+F5 - Eb4+Eb5 - D3+F#3+A3+D4+D5 - Db4+Db5 - | \
                D4+D5 - G4+G5 - D3+G3+B3+F#4+F#5 - G4+G5 - D3+G3+B3+A4+A5 - B4+B5 - C3+E3+G3+C5+C6 -:2 Ab4:1 | \
                D3+G3+B3+D4+G4+B4+D5 -:3 C3+Eb3+G3+Eb4+G4+C5:1 -:11 & -:16 | F5:1 -:15 | \
                D5:1 -:13 G4:1 - | -:2 F4 G3:1 -:2 C4:3 -:4 C4:2 | -:10 Ab3:1 -:5 | \
                -:4 F3+Bb3+D4:1 -:11 | -:16 | - | - & -:16 | - | - | -:14 G3:1 - | -:16 | - | - | \
                - | -",
            ),
            line(
                "Guitar",
                gm::NYLON_GUITAR,
                "-:11 D3:1 F3:4 | G3 Ab3 -:2 D4 C4:4 | G3:5 -:3 F3:4 G3 | \
                _:1 -:2 D4:5 Eb4:1 D4 D4:2 -:1 C4:3 | _:4 G3:5 -:1 F3:4 G3:1 - | \
                Ab3 - C4 -:3 Eb4:1 - D4 - C4 Db4 C#4+D4 -:2 C4:1 | \
                B3 - C4 - G4 - F4 - Eb4 - D4 - C4 - A#3+B3 - | A3 - G3 - G3 - A3 - B3 - D4:2 -:4 | \
                G3:1 -:3 C3:1 -:11 & -:12 Eb3+Fb3:1 -:3 | -:16 | -:4 Ab3 -:8 | B3:4 -:6 C4:1 -:5 | \
                -:16 | - | - | - | -",
            ),
            line(
                "Contrabass",
                gm::CONTRABASS,
                "-:11 D2:1 Eb2+Fb2 F2:3 | _ -:1 Ab2:4 D3 C3 | G2 Ab2 F2:2 F#2 G2:4 | \
                F2:3 -:1 Eb2:4 D2:3 -:1 C2:4 | G2:6 -:2 F3:4 F2 | G2 Ab2 C3 G2 | \
                F2 Eb2 D2:3 -:1 D2:4 | F#2 G2 A2:2 B2 C3:1 -:3 | G2:1 -:3 C2:1 -:11 & -:16 | \
                -:2 G2 -:12 | -:16 | - | -:4 C3 -:8 | -:16 | - | - | -",
            ),
            line(
                "Strings",
                gm::STRING_ENSEMBLE,
                "-:13 F3:3 | _ -:5 Ab3:4 C4 | _:1 -:3 Eb4:5 -:1 F#4:2 G4:4 | \
                _:5 -:1 G4:3 Eb4:2 D4 -:3 | -:4 Bb3 C4:5 -:1 Eb4:2 | G4 F4 Ab4 G4 Bb4 Ab4 G4:4 | \
                F4 Eb4 - C4 | D4:5 - Eb5:1 D5 C5 -:3 | G4:1 -:3 C4:1 -:11 & -:16 | -:2 G3:6 -:8 | \
                D4:4 - F4:3 -:5 | -:3 Ab4 -:6 C4:4 | _ -:8 F4:3 -:1 | -:16 | -:7 D4:5 -:4 | \
                - G4 -:8 | -:16",
            ),
        ])
        .with_tempo(&[tempo(0, 82), tempo(16, 94), tempo(48, 89), tempo(64, 91), tempo(96, 85), tempo(128, 89)]),
    ],
    form: &["Verse a", "Verse b", "Refrain a", "Refrain b", "Last bars"],
    drums_from: 0,
    about: "\"Malena canta el tango como ninguna\": Manzi's portrait of a singer whose voice \
            holds all the sorrow of the tango; Demare, a pianist and bandleader, wrote the \
            tune. C minor, with the refrain in C major.",
};
