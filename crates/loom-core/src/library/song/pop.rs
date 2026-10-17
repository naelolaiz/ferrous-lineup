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
    style: "Piano ballad",
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
    style: "Dance pop",
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
    style: "Afropop",
    parts: &[Part::new(
        "Song",
        "G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:12 D5:4 | \
         G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:12 D5:2 D5 | \
         G5:4 G5 G5 D5:2 D5 | G5:4 G5 G5:8 | G5:2 G5 G5:4 G5:2 G5 G5:4 | G5:2 G5 G5 G5 G5:4 G5 | \
         G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:16",
        "I | IV I | I V | I | I | IV I | I V | I | I IV I I | I IV I I | I IV I I | I IV I I | \
         I | IV I | I V | I",
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
    style: "Four chords",
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

// ---------------------------------------------------------------- Pop

const THREE_BLIND_MICE_TUNE: &str = "B4:6 A4 | G4:6 -:6 | B4:6 A4 | G4:6 -:6 | D5:6 C5:4 C5:2 | B4:6 -:6 | \
     D5:6 C5:4 C5:2 | B4:6 -:4 D5:2 | G5:4 G5:2 F#5 E5 F#5 | G5:4 D5:2 D5:4 D5:2 | \
     G5:2 G5 G5 F#5 E5 F#5 | G5:4 D5:2 D5 D5 D5 | G5:4 G5:2 F#5 E5 F#5 | G5:2 D5 D5 D5:4 C5:2 | \
     B4:6 A4 | G4:6 -:6";
const THREE_BLIND_MICE_CHORDS: &str =
    "I | I | I | I | I | I | I | I | I | I | I | I | I | I | I | I";

// Source: https://raw.githubusercontent.com/yetist/music-score/e4732e0a659a3a017f7290d6217c9e455d8ed61f/abc/friendly-house-senior-choir.abc
// (John Chambers' ABC of "The Everyday Song Book", 1927, no. 40; in G, up a fifth from the book's C)
pub(super) const THREE_BLIND_MICE: Song = Song {
    name: "Three Blind Mice",
    origin: "English round, 17th century",
    feel: "Round",
    genre: Genre::Pop,
    tonic: G,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 126.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    style: "Jig",
    parts: &[
        Part::new("Alone", THREE_BLIND_MICE_TUNE, THREE_BLIND_MICE_CHORDS),
        Part::new("Round", THREE_BLIND_MICE_TUNE, THREE_BLIND_MICE_CHORDS).with_voices(&[0, 4, 8]),
        // The second and third voices carry on from the part before.
        Part::new(
            "Round again",
            THREE_BLIND_MICE_TUNE,
            THREE_BLIND_MICE_CHORDS,
        )
        .with_voices(&[0, -12, 4, -8, 8]),
        Part::new(
            "Last voices",
            THREE_BLIND_MICE_TUNE,
            THREE_BLIND_MICE_CHORDS,
        )
        .with_voices(&[-12, -8]),
    ],
    form: &["Alone", "Round", "Round again", "Last voices"],
    drums_from: 0,
    about: "A round in 6/8 for three voices, each four bars behind the last. The tune walks \
            down three notes, B A G, then four, D C C B, and when the voices overlap those \
            steps fill in the G chord, which is why one chord is enough.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201212037.abc
// (in C, down a tone from the source's D)
pub(super) const SKIP_TO_MY_LOU: Song = Song {
    name: "Skip to My Lou",
    origin: "American play-party song, 19th century",
    feel: "Country",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Country train",
    style: "Country",
    parts: &[Part::new(
        "Song",
        "E5:4 E5:2 E5 C5 C5 C5:4 | E5:4 E5:2 E5 G5:8 | D5:4 D5:2 D5 B4 B4 B4:4 | D5:4 D5:2 D5 F5:8 | \
         E5:4 E5:2 E5 C5 C5 C5:4 | E5:4 E5:2 E5 G5:8 | D5:4 E5:2 F5 E5:4 D5 | C5:8 C5",
        "I | I | V7 | V7 | I | I | V7 | I",
    )],
    form: &["Song", "Song", "Song", "Song"],
    drums_from: 0,
    about: "A play-party song, sung for dancing where fiddles were frowned on. Two chords, C \
            and G7, and a tune that gives each of them one bar: E E E C C C over C, then D D D \
            B B B over G7.",
};

// Source: https://tunearch.org/wiki/Pop_Goes_the_Weasel (T. Westrop's 120 Country Dances, 1860s)
pub(super) const POP_GOES_THE_WEASEL: Song = Song {
    name: "Pop Goes the Weasel",
    origin: "English dance tune, in print by the 1850s",
    feel: "Pop",
    genre: Genre::Pop,
    tonic: G,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 138.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    style: "Jig",
    parts: &[
        Part::new(
            "A",
            "G4:4 B4:2 A4:4 C5:2 | B4:2 C5 D5 G4:4 -:2 | G4:4 B4:2 A4:4 C5:2 | B4:6 G4:4 -:2 | \
             G4:4 B4:2 A4:4 C5:2 | B4:2 C5 D5 G4:4 -:2 | E5:4 -:2 A4:4 D5:2 | B4:6 G4:4 -:2",
            "I V7 | I | I V7 | I | I V7 | I | IV V7 | I",
        ),
        Part::new(
            "B",
            "G5:4 F#5:2 E5 F#5 G5 | F#5:2 A5 F#5 D5:4 F#5:2 | G5:4 F#5:2 E5 F#5 G5 | F#5:6 D5 | \
             C5:4 B4:2 C5:4 D5:2 | E5:4 F#5:2 G5:6 | E5:4 -:2 A4:4 D5:2 | B4:6 G4:4 -:2",
            "I | V7 | I | V7 | IV | IV I | IV V7 | I",
        ),
    ],
    form: &["A", "B", "A", "B"],
    drums_from: 0,
    about: "An English country dance before it was a nursery rhyme. Every phrase rocks \
            between G and D7, and the \"pop\" is the jump up to E over the C chord, with a \
            rest after it, in the seventh bar of each half.",
};
