//! Blues: twelve-bar blues, boogie-woogie and minor blues.

use super::*;

const TWELVE_BAR: &str = "I7 | IV7 | I7 | I7 | IV7 | IV7 | I7 | I7 | V7 | IV7 | I7 | V7";

pub(super) const MIDNIGHT_SHUFFLE: Song = Song {
    name: "Midnight Shuffle",
    origin: "Written for Ferrous Loom",
    feel: "Shuffle",
    genre: Genre::Blues,
    tonic: A,
    mode: Blues,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Shuffle,
    bass: Bass::Boogie,
    rhythm: "Shuffle",
    style: "Twelve-bar blues",
    parts: &[Part::new(
        "Chorus",
        "-:4 C5:3 D5:1 E5:4 G5 | A5:3 G5:1 E5:4 D5 C5 | A4:12 -:4 | -:16 | \
         -:4 C5:3 D5:1 E5:4 G5 | A5:3 G5:1 E5:4 D5 C5 | A4:12 -:4 | -:16 | \
         E5:3 E5:1 G5:4 E5 D5 | D5:3 D5:1 C5:4 A4 C5 | A4:8 -:8 | -:4 E4:4 G4 G#4",
        TWELVE_BAR,
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "The twelve-bar blues, sung the way the blues always is: a line, the same line \
            again over the IV chord, and an answer. The tune uses the A blues scale over \
            major chords, and the clash is the sound of the blues.",
};

pub(super) const ROLLING_BOOGIE: Song = Song {
    name: "Rolling Boogie",
    origin: "Written for Ferrous Loom",
    feel: "Boogie-woogie",
    genre: Genre::Blues,
    tonic: C,
    mode: Blues,
    time: COMMON,
    tempo: 140.0,
    comping: Comping::Charleston,
    bass: Bass::Boogie,
    rhythm: "Shuffle",
    style: "Twelve-bar blues",
    parts: &[Part::new(
        "Chorus",
        "G5:3 Bb5:1 G5:3 F5:1 Eb5:3 C5:1 -:4 | C5:4 Eb5:3 C5:1 F5:4 -:4 | \
         G5:3 Bb5:1 G5:3 F5:1 Eb5:3 C5:1 -:4 | C5:8 -:8 | \
         C6:3 Bb5:1 G5:3 F5:1 Eb5:4 F5:4 | F5:8 -:8 | \
         G5:3 Bb5:1 G5:3 F5:1 Eb5:3 C5:1 -:4 | C5:4 -:12 | \
         G5:3 F#5:1 G5:3 Bb5:1 G5:8 | F5:3 F#5:1 F5:3 Eb5:1 C5:8 | C5:3 Eb5:1 C5:12 | -:8 G4:4 Bb4",
        TWELVE_BAR,
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "Boogie-woogie piano: the left hand walks up and down each chord in a rolling \
            line (root, third, fifth, sixth, seventh) while the right hand plays short blues \
            riffs on top.",
};

pub(super) const MINOR_MOOD: Song = Song {
    name: "Minor Mood Blues",
    origin: "Written for Ferrous Loom",
    feel: "Minor blues",
    genre: Genre::Blues,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Jazz ballad",
    style: "Minor blues",
    parts: &[Part::new(
        "Chorus",
        "-:4 A4:2 C5 D5:4 E5 | G5:6 E5:2 D5:4 C5 | A4:12 -:4 | -:16 | \
         -:4 D5:2 F5 G5:4 A5 | C6:6 A5:2 G5:4 F5 | E5:12 -:4 | -:16 | \
         -:4 F5:2 A5 C6:4 Eb6 | D6:6 B5:2 G#5:4 E5 | A5:4 G5:2 E5 C5:4 A4 | B4:8 G#4",
        "i7 | i7 | i7 | i7 | iv7 | iv7 | i7 | i7 | bVI7 | V7 | i7 | V7",
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "The blues in a minor key, with a walking bass. In bar 9 the F7 chord (bVI7) \
            darkens everything before the E7, the V7, turns back home.",
};
