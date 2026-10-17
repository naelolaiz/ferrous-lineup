//! Reggae and ska: offbeat chords and one-drop drums.

use super::*;

pub(super) const MARY_HAD_A_LITTLE_LAMB: Song = Song {
    name: "Mary Had a Little Lamb",
    origin: "American nursery rhyme, 1830",
    feel: "Ska",
    genre: Genre::Reggae,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 160.0,
    comping: Comping::Offbeat,
    bass: Bass::Walking,
    rhythm: "Ska",
    band: SKA,
    style: "Ska",
    parts: &[Part::new(
        "Tune",
        "E5:6 D5:2 C5:4 D5 | E5 E5 E5:8 | D5:4 D5 D5:8 | E5:4 G5 G5:8 | \
         E5:6 D5:2 C5:4 D5 | E5 E5 E5 E5 | D5 D5 E5 D5 | C5:16",
        "I | I | V | I | I | I | V | I",
    )],
    form: &["Tune", "Tune", "Tune", "Tune"],
    drums_from: 0,
    about: "Ska, Jamaica's music before reggae: fast, with the guitar chords between the \
            beats and a walking bass. A three-note tune, E D C, is all it takes.",
};

pub(super) const THIS_OLD_MAN: Song = Song {
    name: "This Old Man",
    origin: "English nursery rhyme, 19th century",
    feel: "Reggae",
    genre: Genre::Reggae,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Offbeat,
    bass: Bass::Reggae,
    rhythm: "One drop",
    band: REGGAE,
    style: "Reggae",
    parts: &[Part::new(
        "Song",
        "G5:4 E5 G5:8 | G5:4 E5 G5:8 | A5:4 G5 F5 E5 | D5 E5 F5:4 E5:2 F5 | \
         G5:4 C5:4 C5:2 C5 C5:4 | C5:2 D5 E5 F5 G5:8 | G5:4 D5 D5 F5 | E5 D5 C5:8",
        "I | I | IV | V7 | I | I | V7 | I",
    )],
    form: &["Song", "Song", "Song", "Song"],
    drums_from: 0,
    about: "In the one drop, beat one is empty: the kick and the rim land together on beat \
            three, and the guitar only plays between the beats. That space is reggae.",
};

pub(super) const VALLEY_DUB: Song = Song {
    name: "Valley Dub",
    origin: "Written for Ferrous Loom",
    feel: "Roots reggae",
    genre: Genre::Reggae,
    tonic: A,
    mode: Dorian,
    time: COMMON,
    tempo: 74.0,
    comping: Comping::Offbeat,
    bass: Bass::Reggae,
    rhythm: "Steppers",
    band: REGGAE,
    style: "Roots minor",
    parts: &[
        Part::new(
            "A",
            "-:4 E5:4 G5 A5 | F#5:6 E5:2 D5:8 | -:4 C5:2 D5 E5:4 G5 | A5:6 F#5:2 D5:8",
            "i7 | IV7 | i7 | IV7",
        ),
        Part::new(
            "B",
            "A5:2 A5 -:4 G5:2 E5 -:4 | F#5:2 F#5 -:4 E5:2 D5 -:4 | C5:4 D5 E5 G5 | A5:16",
            "i7 | IV7 | i7 | IV7",
        ),
    ],
    form: &["A", "A", "B", "A", "B"],
    drums_from: 0,
    about: "Two chords of A Dorian, Am7 and D7, for heavy roots reggae. The F sharp in the \
            tune is the raised sixth that makes Dorian different from plain minor.",
};
