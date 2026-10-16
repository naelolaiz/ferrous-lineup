//! Pop: nursery rhymes, carols and folk songs over pop beats.

use super::*;

pub(super) const AULD_LANG_SYNE: Song = Song {
    name: "Auld Lang Syne",
    origin: "Scottish folk song, words by Robert Burns, 1788",
    feel: "Ballad",
    genre: Genre::Pop,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Ballad,
    bass: Bass::Held,
    rhythm: "Pop ballad",
    parts: &[
        Part::new("Intro", "-:16 | -:12 C5:4", "I | V7"),
        Part::new(
            "Verse",
            "F5:6 F5:2 F5:4 A5:4 | G5:6 F5:2 G5:4 A5:4 | F5:6 F5:2 A5:4 C6:4 | D6:12 D6:4 | \
             C6:6 A5:2 A5:4 F5:4 | G5:6 F5:2 G5:4 A5:2 G5:2 | F5:6 D5:2 D5:4 C5:4 | F5:12 D6:4",
            "I | V7 | I | IV | I | V7 | IV V7 | I",
        ),
        Part::new(
            "Chorus",
            "C6:6 A5:2 A5:4 F5:4 | G5:6 F5:2 G5:4 D6:4 | C6:6 A5:2 A5:4 C6:4 | D6:12 F6:4 | \
             C6:6 A5:2 A5:4 F5:4 | G5:6 F5:2 G5:4 A5:2 G5:2 | F5:6 D5:2 D5:4 C5:4 | F5:12 C5:4",
            "I | V7 | I | IV | I | V7 | IV V7 | I",
        ),
    ],
    form: &["Intro", "Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 1,
    about: "Sung at New Year all over the world. Like many Scottish tunes, it uses only the \
            five notes of the pentatonic scale: F G A C D.",
};

pub(super) const LONDON_BRIDGE: Song = Song {
    name: "London Bridge Is Falling Down",
    origin: "English nursery rhyme, 17th century",
    feel: "Dance pop",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Stabs,
    bass: Bass::Octaves,
    rhythm: "Dance pop",
    parts: &[Part::new(
        "Tune",
        "G5:6 A5:2 G5:4 F5 | E5 F5 G5:8 | D5:4 E5 F5:8 | E5:4 F5 G5:8 | \
         G5:6 A5:2 G5:4 F5 | E5 F5 G5:8 | D5:8 G5 | E5:4 C5:12",
        "I | I | V7 | I | I | I | V7 | I",
    )],
    form: &["Tune", "Tune", "Tune", "Tune"],
    drums_from: 0,
    about: "A playground song over octave bass and offbeat stabs. The tune only needs two \
            chords: the home chord, and V7 at the end of each half.",
};

pub(super) const OLD_MACDONALD: Song = Song {
    name: "Old MacDonald Had a Farm",
    origin: "Folk song, early 20th century",
    feel: "Afrobeats",
    genre: Genre::Pop,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Tresillo,
    bass: Bass::Dembow,
    rhythm: "Afrobeats",
    parts: &[Part::new(
        "Song",
        "G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:12 D5:4 | \
         G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:12 D5:4 | \
         D5:2 D5 G5:4 G5 G5 | D5:2 D5 G5:4 G5 G5 | G5:2 G5 G5:4 G5:2 G5 G5:4 | G5:2 G5 G5 G5 G5:4 G5 | \
         G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:16",
        "I | IV I | I V | I | I | IV I | I V | I | I | I | I | I | I | IV I | I V | I",
    )],
    form: &["Song", "Song", "Song"],
    drums_from: 0,
    about: "The tresillo, 3 + 3 + 2, pushes the chords and the bass, as in West African pop. \
            \"E-I-E-I-O\" is B B A A G: the third, the second and home.",
};

pub(super) const DECK_THE_HALLS: Song = Song {
    name: "Deck the Halls",
    origin: "Welsh carol \"Nos Galan\", 16th century",
    feel: "Pop",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Pop beat",
    parts: &[Part::new(
        "Verse",
        "G5:6 F5:2 E5:4 D5 | C5 D5 E5 C5 | D5:2 E5 F5 D5 E5:6 D5:2 | C5:4 B4 C5:8 | \
         G5:6 F5:2 E5:4 D5 | C5 D5 E5 C5 | D5:2 E5 F5 D5 E5:6 D5:2 | C5:4 B4 C5:8 | \
         D5:6 E5:2 F5:4 D5 | E5:6 F5:2 G5:4 D5 | E5:2 F#5 G5:4 A5:2 B5 C6:4 | B5 A5 G5:8 | \
         G5:6 F5:2 E5:4 D5 | C5 D5 E5 C5 | A5:2 A5 A5 A5 G5:6 F5:2 | E5:4 D5 C5:8",
        "I | I | ii I | I V7 I I | I | I | ii I | I V7 I I | V7 | I V | iii II7 | V | I | I | IV V7 | I V7 I I",
    )],
    form: &["Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "The third line steps out of C major: F sharp and the D7 chord lead to G, the key \
            of the dominant, for two bars, before the last line comes home.",
};
