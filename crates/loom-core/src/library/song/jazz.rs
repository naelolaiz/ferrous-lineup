//! Jazz: ragtime, swing and a jazz waltz.

use super::*;

pub(super) const JINGLE_BELLS: Song = Song {
    name: "Jingle Bells",
    origin: "James Lord Pierpont, 1857",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 150.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    parts: &[
        Part::new(
            "Verse",
            "D5:4 B5 A5 G5 | D5:12 D5:2 D5 | D5:4 B5 A5 G5 | E5:16 | \
             E5:4 C6 B5 A5 | F#5:16 | D6:4 D6 C6 A5 | B5:16 | \
             D5:4 B5 A5 G5 | D5:16 | D5:4 B5 A5 G5 | E5:12 E5:4 | \
             E5:4 C6 B5 A5 | D6 D6 D6:6 D6:2 | E6:4 D6 C6 A5 | G5:16",
            "I | I | I | IV | IV | V | V7 | I | I | I | I | IV | IV | V | V7 | I",
        ),
        Part::new(
            "Chorus",
            "B5:4 B5 B5:8 | B5:4 B5 B5:8 | B5:4 D6 G5:6 A5:2 | B5:16 | \
             C6:4 C6 C6:6 C6:2 | C6:4 B5 B5 B5:2 B5 | B5:4 A5 A5 B5 | A5:8 D6:8 | \
             B5:4 B5 B5:8 | B5:4 B5 B5:8 | B5:4 D6 G5:6 A5:2 | B5:16 | \
             C6:4 C6 C6:6 C6:2 | C6:4 B5 B5 B5:2 B5 | D6:4 D6 C6 A5 | G5:16",
            "I | I | I | I | IV | I | II7 | V7 | I | I | I | I | IV | I | V7 | I",
        ),
    ],
    form: &["Verse", "Chorus", "Chorus"],
    drums_from: 0,
    about: "In the chorus, A7 is a secondary dominant (the V of V): it leads to D7, which \
            leads home to G.",
};

pub(super) const SAINTS: Song = Song {
    name: "When the Saints Go Marching In",
    origin: "American spiritual, traditional",
    feel: "Dixieland",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 168.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Swing",
    parts: &[Part::new(
        "Chorus",
        "-:4 C5:4 E5 F5 | G5:16 | -:4 C5:4 E5 F5 | G5:16 | \
         -:4 C5:4 E5 F5 | G5:8 E5:8 | C5:8 E5:8 | D5:16 | \
         -:4 E5:4 E5 D5 | C5:12 C5:4 | E5:8 G5:8 | G5:4 F5:12 | \
         -:8 E5:4 F5 | G5:8 E5:8 | C5:8 D5:8 | C5:16",
        "I | I | I | I | I | I | I | V7 | V7 | I | I7 | IV | IV | I | ii7 V7 | I",
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "The New Orleans brass band classic. The tune starts with only four notes, C E F \
            G, and the tuba plays two beats to the bar.",
};

pub(super) const ENTERTAINER: Song = Song {
    name: "The Entertainer",
    origin: "Scott Joplin, 1902",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    parts: &[
        Part::new("Pickup", "-:12 D5:2 D#5", "V"),
        Part::new(
            "A",
            "E5:2 C6:4 E5:2 C6:4 E5:2 C6:2 | _:10 C6:2 D6 D#6 | E6:2 C6 D6 E6:4 B5:2 D6:4 | \
             C6:12 D5:2 D#5 | E5:2 C6:4 E5:2 C6:4 E5:2 C6:2 | _:12 A5:2 G5 | \
             F#5:2 A5 C6 E6:4 D6:2 C6 A5 | D6:12 D5:2 D#5 | \
             E5:2 C6:4 E5:2 C6:4 E5:2 C6:2 | _:10 C6:2 D6 D#6 | E6:2 C6 D6 E6:4 B5:2 D6:4 | \
             C6:12 C6:2 D6 | E6:2 C6 D6 E6:4 C6:2 D6 C6 | E6:2 C6 D6 E6:4 C6:2 D6 C6 | \
             E6:2 C6 D6 E6:4 B5:2 D6:4 | C6:12 D5:2 D#5",
            "I | IV I | I V7 | I | I | IV | II7 | V7 | I | IV I | I V7 | I | I7 | IV iv | I V7 | I",
        ),
    ],
    form: &["Pickup", "A", "A"],
    drums_from: 1,
    about: "Ragtime, the piano music jazz grew out of: the left hand keeps a steady \
            oom-pah, bass then chord, while the right hand pushes against it, landing \
            between the beats and holding on across them.",
};

pub(super) const HAPPY_BIRTHDAY: Song = Song {
    name: "Happy Birthday to You",
    origin: "Mildred and Patty Hill, \"Good Morning to All\", 1893",
    feel: "Jazz waltz",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 132.0,
    comping: Comping::Sway,
    bass: Bass::Waltz,
    rhythm: "Jazz waltz",
    parts: &[Part::new(
        "Song",
        "-:8 G4:3 G4:1 | A4:4 G4 C5 | B4:8 G4:3 G4:1 | A4:4 G4 D5 | C5:8 G4:3 G4:1 | \
         G5:4 E5 C5 | B4 A4 F5:3 F5:1 | E5:4 C5 D5 | C5:12",
        "I | I | V7 | V7 | I | I7 | IV | I V7 | I",
    )],
    form: &["Song", "Song", "Song"],
    drums_from: 0,
    about: "The most sung song in the world, as a jazz waltz: three beats to the bar with a \
            swing ride. Each line climbs a little higher, a fourth, then a fifth, then an \
            octave, for the name.",
};

pub(super) const TWO_FIVE_WALK: Song = Song {
    name: "Two-Five Walk",
    origin: "Written for Ferrous Loom",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 150.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    parts: &[Part::new(
        "Head",
        "-:2 D5:2 F5 A5 C6:4 A5 | B5:2 A5 G5 F5 E5 D5 B4:4 | C5 E5:2 G5 B5:8 | _:8 -:8 | \
         -:2 F5:2 A5 C6 E6:4 D6 | B5:2 G5 F5 D5 B4:4 G4 | A4:2 C5 E5 G5 B5:4 A5 | \
         G5:2 E5 C#5 A4 G5:8 | F5:4 A5 D6:8 | -:2 B5:2 A5 G5 F5:4 D5 | E5 G5 C#5 E5 | \
         D5 F5 B4 D5 | -:2 A5:2 F5 D5 C6:4 A5 | B5:2 D6 F6 D6 B5:4 G5 | C6:16 | -:16",
        "ii7 | V7 | Imaj7 | Imaj7 | ii7 | V7 | Imaj7 | VI7 | ii7 | V7 | iii7 VI7 | ii7 V7 | \
         ii7 | V7 | Imaj7 | Imaj7",
    )],
    form: &["Head", "Head", "Head"],
    drums_from: 0,
    about: "The ii-V-I, the progression jazz is made of, in sixteen bars. Each line spells \
            out its chord: up a Dm7, down a G7, landing on the major seventh of Cmaj7.",
};
