//! Flamenco: the Andalusian cadence and the Phrygian mode.

use super::*;

pub(super) const RUMBA_DEL_SUR: Song = Song {
    name: "Rumba del sur",
    origin: "Written for Ferrous Loom",
    feel: "Rumba flamenca",
    genre: Genre::Flamenco,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 105.0,
    comping: Comping::Rumba,
    bass: Bass::RootFifth,
    rhythm: "Rumba flamenca",
    parts: &[
        Part::new(
            "Copla",
            "E5:3 E5:1 E5:2 F5 E5:4 C5 | D5:3 D5:1 D5:2 E5 D5:4 B4 | \
             C5:3 C5:1 C5:2 D5 C5:4 A4 | B4:2 C5 B4 A4 G#4:8",
            "i | bVII | bVI | V",
        ),
        Part::new(
            "Falseta",
            "A4:1 B4 C5 D5 E5 F5 E5 D5 C5:4 E5 | G5:1 F5 E5 D5 B4:4 G4:8 | \
             F5:1 E5 D5 C5 A4:4 F4:8 | E4:1 F4 G#4 A4 B4 C5 D5 E5 F5:2 E5 D5 B4",
            "i | bVII | bVI | V",
        ),
    ],
    form: &["Copla", "Copla", "Falseta", "Copla", "Falseta"],
    drums_from: 0,
    about: "The Andalusian cadence, Am G F E, under a sung line (the copla) and a guitar \
            run (the falseta). Each line of the copla is the one before, a step lower, \
            following the chords down.",
};

pub(super) const NOCHE_FRIGIA: Song = Song {
    name: "Noche frigia",
    origin: "Written for Ferrous Loom",
    feel: "Rumba flamenca",
    genre: Genre::Flamenco,
    tonic: E,
    mode: Phrygian,
    time: COMMON,
    tempo: 110.0,
    comping: Comping::Rumba,
    bass: Bass::RootFifth,
    rhythm: "Rumba flamenca",
    parts: &[
        Part::new(
            "Tema",
            "G#4:2 A4 B4:4 E5 D5:2 C5 | C5:3 B4:1 A4:4 F4:8 | G4:2 A4 B4 D5 G5:4 F5:2 E5 | \
             F5:3 E5:1 D5:2 C5 B4 A4 G#4:4",
            "I | bII | bIII | bII",
        ),
        Part::new(
            "Remate",
            "E5:4 F5 E5:8 | F5:2 E5 D5 C5 A4:8 | B4:2 D5 G5:4 F5:2 E5 D5:4 | C5:2 B4 A4 G#4 E4:8",
            "I | bII | bIII | bII",
        ),
    ],
    form: &["Tema", "Tema", "Remate", "Tema", "Remate"],
    drums_from: 0,
    about: "The Phrygian mode of the Spanish guitar: E F G A B C D, with a major chord on E \
            and its G sharp. F major, a semitone above home, keeps pulling back down to E.",
};
