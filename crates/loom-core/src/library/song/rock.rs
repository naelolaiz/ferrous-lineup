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
    style: "Dark rock",
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
    style: "Dark rock",
    parts: &[
        Part::new("Pickup", "-:10 E4:2 A4 B4", "i"),
        Part::new(
            "Verse",
            "C5:2 A4:8 E4:2 A4 B4 | C5:2 A4:8 E4:2 A4 B4 | C5:4 B4:2 A4 C5:4 B4:2 A4 | \
             E5:4 E5 E5:2 E5 D5 E5 | F5:2 F5:8 F5:2 E5 D5 | F5:2 E5:8 E5:2 D5 C5 | \
             B4:4 E5 C5 B4 | A4:8 -:2 E4:2 A4 B4",
            "i | i | i | i | iv | i | V | i",
        ),
    ],
    form: &["Pickup", "Verse", "Verse", "Verse", "Verse"],
    drums_from: 1,
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
    style: "Aeolian climb",
    parts: &[Part::new(
        "Motif",
        "-:2 G4:2 G4 G4 Eb4:8 | -:2 F4:2 F4 F4 D4:8 | -:2 G4:2 G4 G4 Eb4 Ab4 Ab4 Ab4 | \
         G4 Eb5 Eb5 Eb5 C5:8 | -:2 G4:2 G4 G4 D4 Ab4 Ab4 Ab4 | G4 F5 F5 F5 D5:8 | \
         D5:2 G5 G5 F5 Eb5:8 | D5:2 G5 G5 F5 Eb5:8 | D5:2 G5 G5 F5 Eb5:4 -:4 | C5:4 -:4 G5:8",
        "i | V7 | i bVI | i | i vii°7 | V7 | V7 i | V7 i | V7 i | II7 V",
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
    style: "Rock and roll",
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

// ---------------------------------------------------------------- Rock

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2622.abc
// (Henrik Norbeck's ABC tunes, song 50, in 2/4; here two bars to the bar, in G, up a fourth from D)
pub(super) const WHISKEY_IN_THE_JAR: Song = Song {
    name: "Whiskey in the Jar",
    origin: "Irish folk song, traditional",
    feel: "Rock",
    genre: Genre::Rock,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 126.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Rock beat",
    style: "Three-chord rock",
    parts: &[
        Part::new("Pickup", "-:14 D5:2", "I"),
        Part::new(
            "Verse",
            "D5:2 D5 D5 E5 D5 B4:4 D5:2 | E5:2 E5 E5 F#5 E5 B4:4 D5:2 | \
             E5:2 E5 E5 F#5 G5:1 G5:3 F#5:2 E5 | D5:2 D5 G5 F#5 E5 D5:4 B4:2 | \
             D5:3 D5:1 D5:2 E5 D5 B4:4 D5:2 | E5:3 E5:1 E5:2 F#5 E5 B4:4 D5:2 | \
             E5:4 E5:3 F#5:1 G5:1 G5:3 F#5:2 E5 | D5:2 D5 G5 F#5 E5 D5 B4 D5",
            "I | vi | IV | I | I | vi | IV | I",
        ),
        Part::new(
            "Chorus",
            "A4:2 A4:1 A4 A4:2 A4:1 A4 A4:8 | -:2 B4 B4:3 A4:1 B4:2 C5 D5:4 | \
             -:2 E5 E5:3 D5:1 E5:2 F#5 G5 E5 | D5:2 B4 A4 B4 G4:6 D5:2",
            "V7 | I | IV | V7 I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 1,
    about: "A highwayman's ballad that rock bands took up in the 1970s. The verse moves G, \
            E minor, C, G, four chords pop still leans on, and the chorus chants one note, A, \
            over D7 before \"whiskey in the jar\" brings it home.",
};

// Source: https://raw.githubusercontent.com/OS2World/MM-UTIL-ABC2Prt/6828e8d638330110fee3a2c7db9ec01d6a1d4764/101best.abc
// ("The One Hundred and One Best Songs", Chicago, 1919, no. 7, melody voice; in D, up from B flat)
pub(super) const BATTLE_HYMN: Song = Song {
    name: "Battle Hymn of the Republic",
    origin: "Tune \"John Brown's Body\", words by Julia Ward Howe, 1862",
    feel: "March",
    genre: Genre::Rock,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "March",
    style: "Three-chord rock",
    parts: &[
        Part::new("Pickup", "-:14 A4:2", "I"),
        Part::new(
            "Verse",
            "A4:3 A4:1 A4:3 G4:1 F#4:3 A4:1 D5:3 E5:1 | F#5:3 F#5:1 F#5:3 E5:1 D5:4 D5:3 C#5:1 | \
             B4:3 B4:1 B4:3 C#5:1 D5:3 C#5:1 D5:3 B4:1 | A4:3 B4:1 A4:3 F#4:1 A4:4 A4:3 A4:1 | \
             A4:3 A4:1 A4:3 G4:1 F#4:3 A4:1 D5:3 E5:1 | F#5:3 F#5:1 F#5:3 E5:1 D5:4 D5 | \
             E5:4 E5 D5 C#5 | D5:8 -",
            "I | I | IV | I | I | I | IV II7 I V7 | I",
        ),
        Part::new(
            "Chorus",
            "A4:6 G4:2 F#4:3 A4:1 D5:3 E5:1 | F#5:8 D5:4 - | B4:6 C#5:2 D5:3 C#5:1 D5:3 B4:1 | \
             A4:8 F#4 | A4:6 G4:2 F#4:3 A4:1 D5:3 E5:1 | F#5:8 D5:4 D5 | E5:4 E5 D5 C#5 | \
             D5:12 -:2 A4:2",
            "I | I | IV | I | I | I | IV II7 I V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 1,
    about: "The camp-meeting tune of \"John Brown's Body\" with Howe's words of 1862. Almost \
            every beat of the verse is a dotted eighth and a sixteenth, the long-short of a \
            march; the chorus smooths that into long notes over the same chords, with an E7 \
            passing through on the way to the last A7.",
};

// Source: https://raw.githubusercontent.com/SMUGSterling/FretFree/9d95cae4e58abd7d050672ef8a38694665ecd32a/scores/pgh-23003/original.abc
// (Paul Hardy's Session Tunebook; in D minor, up a fourth from A minor)
pub(super) const WHEN_JOHNNY: Song = Song {
    name: "When Johnny Comes Marching Home",
    origin: "Patrick Gilmore, 1863",
    feel: "Rock march",
    genre: Genre::Rock,
    tonic: D,
    mode: NaturalMinor,
    time: SIX_EIGHT,
    tempo: 160.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    style: "Tarantella",
    parts: &[
        Part::new("Pickup", "-:10 A4:2", "i"),
        Part::new(
            "Song",
            "A4:2 D5 D5 D5:4 E5:2 | F5:4 E5:2 F5:4 D5:2 | C5:10 A4:2 | C5:10 D5:2 | \
             A4:2 D5 D5 D5:4 E5:2 | F5:4 E5:2 F5:4 G5:2 | A5:10 F5:2 | A5:10 F5:2 | \
             A5:4 A5:2 A5 G5 F5 | G5:4 G5:2 G5:4 E5:2 | F5:4 F5:2 F5 E5 D5 | B4:4 B4:2 B4 C5 D5 | \
             A5:6 G5 | F5:6 E5:4 A4:2 | A4:2 D5 D5 D5:4 C5:2 | D5:10 A4:2",
            "i | i | bIII | bIII | i | i | bIII | V | i | bVII | i | v | i bVII | bVI V | i | i",
        ),
    ],
    form: &["Pickup", "Song", "Song", "Song"],
    drums_from: 1,
    about: "Gilmore's Civil War song, a march in a minor 6/8. The tune climbs the D minor \
            chord, D F A, and the second half walks back down the scale; the chords sit on \
            D minor and F, its relative major, and the one major V, A, comes at the end of \
            each half.",
};

// Source: https://www.mutopiaproject.org/ftp/BizetG/prelude/prelude.ly
// (the Carmen Prelude for piano, Schirmer 1895, whose middle section is
// the Toreador refrain in F; written here in C, a fifth up)
pub(super) const TOREADOR_SONG: Song = Song {
    name: "Toreador Song",
    origin: "Georges Bizet, Carmen, 1875",
    feel: "March",
    genre: Genre::Rock,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "March",
    style: "Three-chord rock",
    parts: &[
        Part::new(
            "Refrain",
            "G5:4 A5:3 G5:1 E5:4 E5:4 | E5:3 D5:1 E5:3 F5:1 E5:6 -:2 | F5:4 D5:3 G5:1 E5:6 -:2 | \
             C5:4 A4:3 D5:1 G4:6 -:2 | D5:8 D5:2 A5:2 G5:2 F5:2 | E5:2 D5:2 E5:2 F5:2 E5:6 -:2 | \
             B4:4 E5:4 E5:4 D#5:3 F#5:1 | B4:16 | _:2 A4:1 B4:1 G4:2 A4:2 D4:2 E4:2 F4:4 | \
             _:2 E4:1 F4:1 C4:2 A4:2 G4:6 -:2",
            "I | I | ii V7 I iii | vi II7 V V | ii | vi | iii VII7 | III | ii7 | V I",
        ),
        Part::new(
            "Tutti",
            "G5:2 -:2 A5:2 -:1 G5:1 E5:2 -:2 E5:2 -:2 | E5:2 -:1 D5:1 E5:2 -:1 F5:1 E5:6 -:2 | \
             F5:2 -:2 D5:2 -:1 G5:1 E5:6 -:2 | C5:2 -:2 A4:2 -:1 D5:1 G4:6 -:2",
            "I | I | ii V7 I I | vi II7 V V",
        ),
    ],
    form: &["Refrain", "Tutti", "Refrain", "Tutti"],
    drums_from: 0,
    about: "The bullfighter's refrain, \"Toréador, en garde\", as a rock march; Bizet wrote \
            it in F, here it is a fifth up in C so the low second half stays in range. \
            Listen for the turn at \"et que l'amour t'attend\": E minor, then E major, \
            the dominant of the relative minor, before the band answers in clipped chords. \
            The opera's 2/4 bars go two to a bar here.",
};
