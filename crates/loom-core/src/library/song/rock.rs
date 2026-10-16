//! Rock: folk songs and a symphony played as rock and punk.

use super::*;

pub(super) const DRUNKEN_SAILOR: Song = Song {
    name: "Drunken Sailor",
    origin: "Sea shanty, traditional",
    feel: "Folk punk",
    genre: Genre::Rock,
    tonic: D,
    mode: Dorian,
    time: COMMON,
    tempo: 150.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Punk",
    parts: &[
        Part::new(
            "Verse",
            "A5:4 A5:2 A5 A5:4 A5:2 A5 | A5:4 D5 F5 A5 | G5:4 G5:2 G5 G5:4 G5:2 G5 | G5:4 C5 E5 G5 | \
             A5:4 A5:2 A5 A5:4 A5:2 A5 | A5:4 B5 C6 D6 | C6 A5 G5 E5 | D5:8 D5:8",
            "i | i | bVII | bVII | i | i | bVII | i",
        ),
        Part::new(
            "Chorus",
            "A5:8 A5:6 A5:2 | A5:4 D5 F5 A5 | G5:8 G5:6 G5:2 | G5:4 C5 E5 G5 | \
             A5:8 A5:6 A5:2 | A5:4 B5 C6 D6 | C6 A5 G5 E5 | D5:8 D5:8",
            "i | i | bVII | bVII | i | i | bVII | i",
        ),
    ],
    form: &["Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 0,
    about: "Sailors sang shanties to pull ropes together. This one is Dorian: minor, but \
            with a B natural, the raised sixth, in the sixth bar.",
};

pub(super) const BELLA_CIAO: Song = Song {
    name: "Bella Ciao",
    origin: "Italian folk song, 19th century",
    feel: "Rock",
    genre: Genre::Rock,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Rock ride",
    parts: &[Part::new(
        "Verse",
        "E4:2 A4 B4 C5 A4:8 | E4:2 A4 B4 C5 A4:8 | E4:2 A4 B4 C5:6 B4:2 A4 | C5:6 B4:2 A4 E5:6 | \
         E5:4 D5:2 E5 F5:8 | F5:4 E5:2 D5 F5:8 | E5:4 D5:2 C5 B4:4 E5 | C5 B4 A4:8",
        "i | i | i | i | iv | iv | i V | i",
    )],
    form: &["Verse", "Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "A song of the rice-field workers of northern Italy, later of the resistance. The \
            fifth bar moves to the iv chord, D minor, which makes the tune climb.",
};

pub(super) const BEETHOVENS_FIFTH: Song = Song {
    name: "Beethoven's Fifth",
    origin: "Ludwig van Beethoven, Symphony No. 5, 1808",
    feel: "Hard rock",
    genre: Genre::Rock,
    tonic: C,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Half-time rock",
    parts: &[Part::new(
        "Motif",
        "-:2 G4:2 G4 G4 Eb4:8 | -:2 F4:2 F4 F4 D4:8 | -:2 G4:2 G4 G4 Eb4 Ab4 Ab4 Ab4 | \
         G4 Eb5 Eb5 Eb5 C5:8 | -:2 G4:2 G4 G4 D4 Ab4 Ab4 Ab4 | G4 F5 F5 F5 D5:8 | \
         -:2 G4:2 G4 G4 Eb4:8 | -:2 F4:2 F4 F4 D4:8",
        "i | V7 | i iv | i | V iv | V7 | i | V7",
    )],
    form: &["Motif", "Motif", "Motif"],
    drums_from: 0,
    about: "Four notes, short-short-short-long, are the whole idea: Beethoven builds the \
            movement by repeating them a step lower, then higher. A riff, long before rock.",
};

pub(super) const YANKEE_DOODLE: Song = Song {
    name: "Yankee Doodle",
    origin: "American folk song, 18th century",
    feel: "Rock and roll",
    genre: Genre::Rock,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 168.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Rock beat",
    parts: &[Part::new(
        "Song",
        "C5:2 C5 D5 E5 C5 E5 D5:4 | C5:2 C5 D5 E5 C5:4 B4 | C5:2 C5 D5 E5 F5 E5 D5 C5 | \
         B4 G4 A4 B4 C5:4 C5 | A4:3 B4:1 A4:2 G4 A4 B4 C5:4 | G4:3 A4:1 G4:2 F4 E4:4 G4 | \
         A4:3 B4:1 A4:2 G4 A4 B4 C5 A4 | G4 C5 B4 D5 C5:4 C5",
        "I | I V | I | V I | IV | V7 I | IV | V I",
    )],
    form: &["Song", "Song", "Song", "Song"],
    drums_from: 0,
    about: "Three chords, fast and loud. The chorus starts on A over the IV chord, F, which \
            makes it sound like a new section even though the tune hardly changes.",
};
