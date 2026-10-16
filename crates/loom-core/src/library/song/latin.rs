//! Latin: cumbia, bossa nova and samba.

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
    origin: "Written for Ferrous Loom",
    feel: "Bossa nova",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 128.0,
    comping: Comping::Bossa,
    bass: Bass::Bossa,
    rhythm: "Bossa nova",
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
    origin: "Written for Ferrous Loom",
    feel: "Samba",
    genre: Genre::Latin,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::Bossa,
    bass: Bass::Bossa,
    rhythm: "Samba",
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
