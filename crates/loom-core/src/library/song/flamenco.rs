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
    style: "Andalusian cadence",
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
    style: "Phrygian flamenco",
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

// Source: https://raw.githubusercontent.com/mrgrenier/guitar_music/master/albeniz.ly
// (a LilyPond guitar transcription in E minor, written as guitar music is, an octave above
// the sound; the first sixteen bars. Checked bar by bar against Goran Stefanovic's tablature
// of Segovia's arrangement, https://raw.githubusercontent.com/baweaver/classtab/master/app/tabs/albeniz_isaac_op047_no5_espanola_asturias.txt.)
// The chords are the pedal notes, E and B.
pub(super) const ASTURIAS: Song = Song {
    name: "Asturias (Leyenda)",
    origin: "Isaac Albéniz, Cantos de España, 1892",
    feel: "Flamenco minor",
    genre: Genre::Flamenco,
    tonic: E,
    mode: NaturalMinor,
    time: THREE_FOUR,
    tempo: 132.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    style: "La folia",
    parts: &[
        Part::new(
            "On E",
            "E4:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 G4 B4 | E4:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 G4 B4 | \
             E4:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 G4 B4 | A4:1 B4 F#4 B4 G4 B4 A4 B4 F#4 B4 G4 B4 | \
             E4:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 G4 B4 | E4:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 G4 B4 | \
             E4:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 G4 B4 | A4:1 B4 F#4 B4 G4 B4 E4 B4 D4 B4 C4 B4",
            "i | i | i | i | i | i | i | i",
        ),
        Part::new(
            "On B",
            "B3:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 F#4 B4 | B3:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 F#4 B4 | \
             B3:1 B4 F#4 B4 A4 B4 B4 B4 C5 B4 D5 B4 | B4:1 B4 C5 B4 A4 B4 B4 B4 G4 B4 F#4 B4 | \
             B3:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 F#4 B4 | B3:1 B4 B4 B4 C5 B4 A4 B4 B4 B4 F#4 B4 | \
             B3:1 B4 A4 B4 B4 B4 C5 B4 D5 B4 B4 B4 | C5:1 B4 A4 B4 B4 B4 G4 B4 A4 B4 F#4 B4",
            "V | V | V | V | V | V | V | V",
        ),
    ],
    form: &["On E", "On B", "On E", "On B"],
    drums_from: 0,
    about: "Albéniz wrote it for piano in 1892, imitating the flamenco guitar, and on the \
            guitar, in E minor, it became one of the instrument's best-known pieces. Every \
            other sixteenth note is the same B, a pedal, while the tune moves under it, C B A \
            B G; the second half plays the same figure over a low B, the dominant. Here \
            without drums, over held chords.",
};
