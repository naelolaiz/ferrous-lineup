//! Soul, funk and R&B: hymns, gospel and grooves.

use super::*;

pub(super) const AMAZING_GRACE: Song = Song {
    name: "Amazing Grace",
    origin: "Hymn tune \"New Britain\", 1835",
    feel: "Gospel waltz",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: THREE_FOUR,
    tempo: 76.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Intro", "-:12 | -:8 D5:4", "I | V"),
        Part::new(
            "Verse",
            "G5:8 B5:2 G5:2 | B5:8 A5:4 | G5:8 E5:4 | D5:8 D5:4 | \
             G5:8 B5:2 G5:2 | B5:8 A5:4 | D6:12 | _:8 B5:4 | \
             D6:8 B5:2 G5:2 | B5:8 A5:4 | G5:8 E5:4 | D5:8 D5:4 | \
             G5:8 B5:2 G5:2 | B5:8 A5:4 | G5:12 | _:8 D5:4",
            "I | I | IV | I | I | I I V | V | V V I | I | I | IV | I | I | I I V | I | I",
        ),
    ],
    form: &["Intro", "Verse", "Verse", "Verse"],
    drums_from: 1,
    about: "The whole melody uses five notes, G A B D E: the major pentatonic scale, which \
            is why it sounds so open.",
};

pub(super) const JOY_TO_THE_WORLD: Song = Song {
    name: "Joy to the World",
    origin: "Hymn by Isaac Watts, tune arranged by Lowell Mason, 1839",
    feel: "Motown",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Quarters,
    bass: Bass::Walking,
    rhythm: "Motown",
    parts: &[Part::new(
        "Verse",
        "C6:4 B5:3 A5:1 G5:6 F5:2 | E5:4 D5 C5:6 G5:2 | A5:6 A5:2 B5:6 B5:2 | \
         C6:6 C6:2 C6 B5 A5 G5 | G5:3 F5:1 E5:2 C6 C6 B5 A5 G5 | \
         G5:3 F5:1 E5:2 E5 E5 E5 E5 E5:1 F5 | G5:6 F5:1 E5 D5:2 D5 D5 D5:1 E5 | \
         F5:6 E5:1 D5 C5:2 C6:4 A5:2 | G5:3 F5:1 E5:2 F5 E5:4 D5 | C5:16",
        "I V7 | I | IV V | I | I | I | V7 | V7 IV | I V7 | I",
    )],
    form: &["Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "The first line is just the C major scale, walking down from the top. Here it \
            gets the Motown treatment: piano on every beat, a busy bass, and the snare on \
            all four.",
};

pub(super) const DORIAN_STRUT: Song = Song {
    name: "Dorian Strut",
    origin: "Written for Ferrous Loom",
    feel: "Funk",
    genre: Genre::Soul,
    tonic: E,
    mode: Dorian,
    time: COMMON,
    tempo: 102.0,
    comping: Comping::Funk,
    bass: Bass::Funk,
    rhythm: "Funk",
    parts: &[
        Part::new(
            "Riff",
            "E5:2 -:1 E5:1 G5:2 A5 -:2 B5:1 -:1 A5:2 G5 | E5:3 D5:1 E5:4 -:8 | \
             C#6:2 -:1 C#6:1 E6:2 C#6 -:2 B5:1 -:1 A5:2 G5 | A5:3 G5:1 E5:4 -:8",
            "i7 | i7 | IV7 | IV7",
        ),
        Part::new(
            "Break",
            "B5:1 B5 -:2 A5:1 A5 -:2 G5 E5 D5 E5 | -:4 E5:2 G5 E5:4 D5 | \
             C#5:2 E5 G5 B5 A5:4 G5 | E5:16",
            "i7 | i7 | IV7 | IV7",
        ),
    ],
    form: &["Riff", "Riff", "Break", "Riff", "Riff"],
    drums_from: 0,
    about: "Funk lives on two chords and a lot of rhythm. In E Dorian the IV chord is A7, \
            major, and its C sharp is the note that makes Dorian brighter than plain minor.",
};

pub(super) const SUNDAY_SEVENTH: Song = Song {
    name: "Sunday Seventh",
    origin: "Written for Ferrous Loom",
    feel: "Neo-soul",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Lazy,
    bass: Bass::Lazy,
    rhythm: "Neo soul",
    parts: &[
        Part::new(
            "Call",
            "-:4 A5:3 G5:1 E5:4 C5 | B4:6 D5:2 G#5:8 | -:2 E5:2 G5 A5 C6:4 B5:2 A5 | \
             G5:6 E5:2 Bb5:8",
            "IVmaj7 | III7 | vi7 | I7",
        ),
        Part::new(
            "Answer",
            "-:4 C6:3 A5:1 G5:4 E5 | D5:6 B4:2 G#4:8 | A4:4 C5 E5 G5 | E5:12 -:4",
            "IVmaj7 | III7 | vi7 | I7",
        ),
    ],
    form: &["Call", "Answer", "Call", "Answer"],
    drums_from: 0,
    about: "Lush seventh chords with two borrowed dominants: E7 pulls toward A minor and C7 \
            toward F. The tune follows them, with a G sharp over E7 and a B flat over C7.",
};
