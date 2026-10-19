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
    band: ROCK,
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
    band: ROCK,
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
    band: ROCK,
    style: "Aeolian climb",
    parts: &[Part::new(
        "Motif",
        "-:2 G4:2 G4 G4 Eb4:8 | -:2 F4:2 F4 F4 D4:8 | -:2 G4:2 G4 G4 Eb4 Ab4 Ab4 Ab4 | \
         G4 Eb5 Eb5 Eb5 C5:8 | -:2 G4:2 G4 G4 D4 Ab4 Ab4 Ab4 | G4 F5 F5 F5 D5:8 | \
         D5:2 G5 G5 F5 Eb5:8 | D5:2 G5 G5 F5 Eb5:8 | D5:2 G5 G5 F5 Eb5:4 -:4 | C5:4 -:4 G5:8",
        "i | V7 | i bVI | i | i vii°7 | V7 | V7 i | V7 i | V7 i | II7 V",
    )
    // The score's fermatas: the held E flat and D of the motif, and the
    // G before the music breaks off again (bars 2, 5 and 21).
    .with_tempo(&[fermata(8, 8), fermata(24, 8), fermata(152, 8)])],
    form: &["Motif", "Motif", "Motif"],
    drums_from: 0,
    about: "Four notes, short-short-short-long, are the whole idea: Beethoven builds the \
            movement by repeating them a step lower, then higher. A riff, long before rock. \
            The long notes carry fermatas in the score: the music stops and waits on them, \
            then rushes on.",
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
    band: ROCK_AND_ROLL,
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
    band: ROCK,
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
    band: BRASS_BAND,
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
    band: ROCK,
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
    band: BRASS_BAND,
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

// Source: https://abcnotation.com/tunePage?a=trillian.mit.edu%2F%7Ejc%2Fmusic%2Fabc%2Fmirror%2Fmusicaviva.com%2Ftunes%2Fusa%2Fhouse-of-the-rising-01%2F0000
// (ABC transcribed by Frank Nordberg, musicaviva.com; written there in 3/4,
// barred here in 6/8 with the same note values)
pub(super) const HOUSE_OF_THE_RISING_SUN: Song = Song {
    name: "House of the Rising Sun",
    origin: "Traditional, American",
    feel: "Six-eight",
    genre: Genre::Rock,
    tonic: D,
    mode: NaturalMinor,
    time: SIX_EIGHT,
    tempo: 144.0,
    comping: Comping::Arp3,
    bass: Bass::Held,
    rhythm: "Six-eight",
    band: Band::new(gm::DRAWBAR_ORGAN, gm::CLEAN_GUITAR, gm::FINGER_BASS),
    style: "Zamba",
    parts: &[
        Part::new("Pickup", "-:8 A4:4", "i"),
        Part::new(
            "Verse",
            "D4:8 E4:2 D4 | F4:8 G4:2 G#4 | A4:8 F4:2 E4 | D4:8 A4:4 | \
             D5:8 A4:4 | C5:8 A4:2 G4 | A4:12 | _:4 - A4:2 A4 | \
             D5:8 D4:4 | F4:8 G4:4 | A4:2 A4:6 F4:2 E4 | D4:8 A4:4 | \
             A4:8 A4:4 | A4:2 A3:6 F4:2 E4 | D4:12 | _:8 A4:4",
            "i | i | V7 | i | i | bVII7 | bIII | V7 | i | i | i | bVI7 | i | V7 | i | i",
        ),
    ],
    form: &["Pickup", "Verse", "Verse"],
    drums_from: 1,
    about: "A minor-key ballad with the chords picked out one note at a time. The tune \
            climbs the D minor chord, D, F, A, to the high D, and the harmony moves round \
            D minor, C7, F and A7, with B flat 7 (bVI7) shading the last line.",
};

// Source: https://raw.githubusercontent.com/arbitrary-dev/piano-sheets/master/src/rossini-william-tell.ly
// (a piano fragment in F, written in 4/4 with doubled note values; the Nottingham Music
// Database reel arrangement, jukedeck/nottingham-dataset reelsu-z.abc, has the same pitches)
pub(super) const WILLIAM_TELL: Song = Song {
    name: "William Tell Overture",
    origin: "Gioachino Rossini, William Tell, 1829",
    feel: "Punk",
    genre: Genre::Rock,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 152.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Punk",
    band: ROCK,
    style: "Surf rock",
    parts: &[
        Part::new("Pickup", "-:14 C4:1 C4:1", "I"),
        Part::new(
            "Gallop",
            "C4:2 C4:1 C4:1 C4:2 C4:1 C4:1 F4:2 G4:2 A4:2 C4:1 C4:1 | \
             C4:2 C4:1 C4:1 C4:2 F4:1 A4:1 G4:2 E4:2 C4:2 C4:1 C4:1 | \
             C4:2 C4:1 C4:1 C4:2 C4:1 C4:1 F4:2 G4:2 A4:2 F4:1 A4:1 | \
             C5:4 -:1 Bb4:1 A4:1 G4:1 F4:2 A4:2 F4:2 C5:1 C5:1",
            "I | I V | I | V I",
        ),
        Part::new(
            "Gallop higher",
            "C5:2 C5:1 C5:1 C5:2 C5:1 C5:1 F5:2 G5:2 A5:2 C5:1 C5:1 | \
             C5:2 C5:1 C5:1 C5:2 F5:1 A5:1 G5:2 E5:2 C5:2 C5:1 C5:1 | \
             C5:2 C5:1 C5:1 C5:2 C5:1 C5:1 F5:2 G5:2 A5:2 F5:1 A5:1 | \
             C6:4 -:1 Bb5:1 A5:1 G5:1 F5:2 A5:2 F5:4",
            "I | I V | I | V I",
        ),
    ],
    form: &["Pickup", "Gallop", "Gallop higher"],
    drums_from: 1,
    about: "The cavalry charge at the end of the overture, the Lone Ranger's theme: one \
            note in a galloping rhythm, two quick notes and a longer one, before the tune \
            climbs F, G, A. Only I and V under it. In 2/4, two bars to a bar \
            here, and played a second time an octave higher, as the piano fragment asks.",
};

/// The intro: B flat, A flat and G over a bass climbing G, B, C, then home to E flat.
const DESARMA_INTRO_CHORDS: &str = "V IV | III | IV | I | V | I";
/// The verse: E flat, D flat (bVII, borrowed from E flat minor) and A flat, C minor, B flat;
/// then E flat over D flat (I7, the V7 of IV) into A flat.
const DESARMA_VERSE_CHORDS: &str = "I | bVII IV IV | vi | V | I | bVII IV IV | vi | V | I7 | IV";
/// The bar of 3/4 in the middle of the verse: B flat, A flat, E flat over B flat.
const DESARMA_TURN_CHORDS: &str = "V IV I";
/// F7 over A (II7, the V7 of V: the bar spells A, C, E flat, F) into B flat; the verse's
/// last line; C minor and F lead back to the intro.
const DESARMA_VERSE_END_CHORDS: &str = "II7 | V | I | bVII IV IV | vi vi V | I | vi | II";
/// E flat minor: i and V; then the figure moves onto D flat minor (bvii) for two bars.
const DESARMA_EB_MINOR_CHORDS: &str = "i | i | V | V | i | i | bvii | bvii";
/// G minor: i, v, the bass falling B flat, A, G to the suspended dominant.
const DESARMA_GM_A_CHORDS: &str = "i | i | v | v | i | i | bVI | bVI | V7sus4 | V7 | i | i";
const DESARMA_GM_B_CHORDS: &str = "bVII | bVII | bVI | bVI | bIII | bIII | bVI | bVI | V7sus4 | V7";
/// G major: the tune steps down over G, A minor 7, G major 7, then E flat (bVI).
/// Bar 96 is a shot: both piano scores strike E flat once and stop (the 3/4 score's bar 71 has a
/// sixteenth chord and rests), and the choir rests but for the tenors' pickup.
const DESARMA_G_CHORDS: &str = "I | I | ii7 | ii7 | Imaj7 | Imaj7 | bVI | bVI | bVI | bVI.";
const DESARMA_VERSE3_CHORDS: &str = "I | I | bVI | bVI | I | I | bVI | bVI";
/// In 3/4: A suspended resolving to A (II, the V of V), E minor 7, A minor 9, D13. In bar 112 the
/// left hand rests and the right hand climbs F sharp, G, A alone (both piano scores): the band rests.
const DESARMA_VERSE3_END_CHORDS: &str = "IIsus4 II II | II | vi7 | vi7 | ii9 | ii9 | V13 | V13...";
const DESARMA_CODA_CHORDS: &str = "I | bVII bVII IV";
/// In 3/4: G over F, C over E, the bass walking down to A and D; then C minor 7 and F,
/// borrowed from G minor, turn toward E flat.
const DESARMA_OUTRO_CHORDS: &str =
    "I7 | IV | I ii I | II7 | V | I | bVII IV IV | vi7 V | iv7 | bVII";
const DESARMA_FINAL_CHORDS: &str = "V IV | III | IV | I | V | I | IV | I | I";

// Source: score attached by the owner (page images), "DESARMA Y SANGRA", Charly García:
// piano arrangement with chord symbols, "Introducción", 6/8 with bars of 3/4, dotted quarter
// = 80 (= 68 at bar 128, = 62 at bar 132), 136 bars, first and second endings at bars
// 116-117. Chords = the printed symbols, checked against the piano's bass and right-hand
// chords bar by bar; the tune of the instrumental sections (intro, the middle section in
// E flat minor, G minor and G major, coda, outro, final) = the top line of the right hand.
// Second source: score attached by the owner (page images), "Desarma y Sangra", Charly
// García: piano, 3/4, quarter = 115, 112 bars with a repeat of bars 7-31 (the same music,
// arranged differently); used to cross-check the notes and for bars 17-18 (bass D, C, B flat,
// A: F over A, as the chord sheet acordesweb.com/cancion/seru-giran/desarma-y-sangra has it).
// Sung line: choral arrangement by Joaquín Martínez Dávila attached by the owner (PDF),
// "Desarma y sangra", Charly García, SATB a cappella, 3/4, quarter = 110, 140 bars, in E flat
// like the demo (G minor and G major in the middle), so nothing is transposed. Its bars of
// 3/4 line up one for one with the demo's bars of 6/8 (12 steps each); its first intro has
// one more bar of held E flat. The tune is taken from whichever voice sings it with the
// words: in the first verse the basses ("Tu tiempo es un vidrio", "mi cuerpo una aguja"),
// then the tenors ("tu amor un faquir", "tu mente un tapiz, si las sanguijuelas no pueden
// herirte"), then the sopranos ("no existe una escuela que enseñe a vivir"); in the second
// verse the sopranos, with the altos for "se olvida del hombre"; in "Miro alrededor" the
// tenors, while the other voices repeat "miro alrededor, sospechas que van y aquí estoy" in
// chords, and the sopranos for "alma, desarma y sangra". The men's lines are written an
// octave up, where the sopranos sing the same tune in the second verse.
// The choir's "na ra ra" and "ah" sections (the intro, middle and coda) are not sung words:
// there the piano's tune stays.
// Marks: the piano scores print no dynamics; the dynamics and hairpins are the choral score's,
// those of the voice that has the tune, or of the whole choir where they agree (its bars one
// later than the demo's: choral bar n is the demo's bar n - 1 from the verse on). The tempo
// marks are the 6/8 score's (dotted quarter = 80, 68, 62) and the 3/4 score's final fermata;
// the choral arranger's own rit. (bars 55-56 and 110-113), fermata and caesura (bar 57), "Più
// mosso" (58) and "Tempo primo" (114) are left out, as they are not in the band's scores.
// Under copyright (Charly García); added at the owner's request.
pub(super) const DESARMA_Y_SANGRA: Song = Song {
    name: "Desarma y sangra",
    origin: "Charly García, Serú Girán, 1979",
    feel: "Rock ballad",
    genre: Genre::Rock,
    tonic: DSharp,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 120.0,
    comping: Comping::Arp3,
    bass: Bass::Held,
    rhythm: "Six-eight",
    band: SERU,
    style: "Six-eight ballad",
    parts: &[
        // Bars 1-6 (and, as "Intro 2", 26-31) of the 6/8 score; choral bars 1-7 and 27-32 (the choir's first
        // intro holds the last E flat one bar longer); bars 2-6 are marked 8va. The last
        // eighth is the sung pickup into the verse ("Tu", "el"), from the choral score.
        // The choir sings it mp, fading (a diminuendo) over bar 5; the basses' "Tu" is mp.
        Part::new(
            "Intro",
            "!mp D5:2 Eb5 D5 C5 D5 C5 | B4 G4 B4 D5 G5 F5 | Eb5 Ab4 C5 Ab5 Eb5 Ab4 | \
             Eb5 G4 Bb4 G4 Eb5 Bb4 | !> F5:4 Eb5 D5 | != Eb5:8 -:2 !mp Bb3:2",
            DESARMA_INTRO_CHORDS,
        ),
        // The same intro after the first verse (bars 26-31; choral bars 27-32), mf, with the
        // diminuendo over bar 30 and the sopranos' "el" mf.
        Part::new(
            "Intro 2",
            "!mf D5:2 Eb5 D5 C5 D5 C5 | B4 G4 B4 D5 G5 F5 | Eb5 Ab4 C5 Ab5 Eb5 Ab4 | \
             Eb5 G4 Bb4 G4 Eb5 Bb4 | !> F5:4 Eb5 D5 | != Eb5:8 -:2 !mf Bb3:2",
            DESARMA_INTRO_CHORDS,
        ),
        // Bars 7-16; choral bars 8-17: "Tu tiempo es un vidrio, tu amor un faquir, mi cuerpo
        // una aguja, tu mente un tapiz, si las sanguijuelas". The basses sing the first and
        // third lines, the tenors (with the basses below them) the second and fourth, the
        // tenors alone "si las sanguijuelas"; all an octave up here.
        Part::new(
            "Verse",
            "!mp G4:6 Ab4:4 G4:2 | F4:4 Eb4 -:2 Ab4 | G4:6 Ab4:4 Bb4:2 | Bb4:8 -:2 Bb3 | \
             G4:6 Ab4:4 G4:2 | F4:4 Eb4 -:2 Ab4 | G4:6 Ab4:4 Bb4:2 | Bb4:8 -:2 Bb4 | \
             !mf Bb4:6 Eb5:4 Eb5:2 | Eb5:4 Ab4 -:2 Ab4",
            DESARMA_VERSE_CHORDS,
        ),
        // Bar 17 (and 42), in 3/4; choral bars 18 and 43 ("no pueden he-", "o apenas e-"; in the
        // first verse the choir's basses sing D, C, B flat under it). The 6/8 score prints Bb,
        // Ab/C, G/Bb over a bass Bb, Ab, Bb; the 3/4 score has the bass D, C, B flat, and the chord
        // sheet Bb/D Ab/C Eb/Bb: beat 3 is written I (E flat over B flat).
        Part::new("Verse turn", "!mf Bb4:4 Ab4 G4", DESARMA_TURN_CHORDS)
            .in_time(THREE_FOUR)
            .played(Comping::Arp3, Bass::Held, "Waltz"),
        // Bars 18-25; choral bars 19-26: "-rir-te, no existe una escuela que enseñe a vivir",
        // the tenors (an octave up) to "herirte", then the sopranos; the last two bars are
        // the sopranos' rising "ah" (E flat, F, G, A, B flat, C, as the piano plays it). Bar
        // 18 follows the 3/4 score (A natural, F over A, as the chord sheet's F/A; with the
        // E flat of the piano's figure it is F7); the 6/8 score prints Ab with A flats.
        Part::new(
            "Verse end",
            "!mf F4:12 | F4:8 -:2 !p Bb3:2 | G4:6 Ab4:2 G4:4 | F4 Eb4 -:2 C4 | Eb4:6 F4:2 D4:4 | \
             Eb4:12 | !< Eb4:4 F4 G4 | A4 Bb4 C5 !mf",
            DESARMA_VERSE_END_CHORDS,
        ),
        // Bars 32-41; choral bars 33-42, the sopranos: "El ángel vigía descubre al ladrón, le
        // corta las manos, le quita la voz, la gente se esconde". The tune is the first
        // verse's, with the pickups and one rhythm changed by the words.
        Part::new(
            "Verse 2",
            "!mf G4:6 Ab4:4 G4:2 | F4:4 Eb4 -:2 Bb3 | G4:6 Ab4:4 Bb4:2 | Bb4:8 -:2 Bb3 | \
             G4:6 Ab4:2 G4:4 | F4 Eb4 -:2 Bb3 | G4:6 Ab4:4 Bb4:2 | Bb4:8 -:2 !mf Bb4 | \
             Bb4:6 Eb5:4 Eb5:2 | Eb5:4 Ab4 -:2 Ab4",
            DESARMA_VERSE_CHORDS,
        ),
        // Bars 43-50; choral bars 44-51: "-xis-te, se olvida del hombre, se olvida de Dios":
        // the sopranos to "existe", the altos "se olvida del hombre" (the sopranos hold an
        // "ah" above), the sopranos again "se olvida de Dios", then their rising "ah".
        Part::new(
            "Verse 2 end",
            "!mf F4:12 | F4:8 -:2 Bb3:2 | !> G4:6 Ab4:4 G4:2 | F4:4 Eb4 -:2 !p C4 | Eb4:6 F4:2 D4:4 | \
             Eb4:12 | !< Eb4:4 F4 G4 | A4 Bb4 C5 !mf",
            DESARMA_VERSE_END_CHORDS,
        ),
        // Bars 51-56; choral bars 52-57, "na ra ra": the intro again, leading to the middle
        // section.
        Part::new(
            "Interlude",
            "!mf D5:2 Eb5 D5 C5 D5 C5 | B4 G4 B4 D5 G5 F5 | Eb5 Ab4 C5 Ab5 Eb5 Ab4 | \
             Eb5 G4 Bb4 G4 Eb5 Bb4 | !> F5:4 Eb5 D5 | != Eb5:12",
            DESARMA_INTRO_CHORDS,
        ),
        // Bars 57-64 (choral bars 58-65, "na ra ra" and "ah"): the key signature stays three flats,
        // but the music is in E flat minor. Bars 63-64 are printed with five flats: two bars of D
        // flat minor (E is its F flat), one chord with no cadence of its own, so they are read as
        // bvii of E flat minor rather than as a key.
        Part::new(
            "E flat minor",
            "!mp Gb4:8 F4:4 | Eb4:4 F4 Gb4 | F4:2 Bb3 Bb3 Bb3 Bb3 Bb3 | F4 Bb3 Bb3 Bb3 Bb3 Bb3 | \
             Gb4:8 F4:4 | !< Eb4:4 F4 Gb4 | Ab4:8 Gb4:4 | E4:4 Gb4 Ab4 !mf",
            DESARMA_EB_MINOR_CHORDS,
        )
        .in_key(DSharp, NaturalMinor),
        // Bars 65-76 (choral bars 66-77, "na ra ra"), two flats. The D chords carry C in the right
        // hand ("Dsus", "D"): V7sus4 and V7.
        Part::new(
            "G minor",
            "!mf Bb4:8 A4:4 | G4:4 A4 Bb4 | A4:2 D4 D4 D4 D4 D4 | A4 D4 D4 D4 D4 D4 | \
             !> Bb4:8 A4:4 | G4:4 A4 Bb4 | !p Bb4:12 | Bb4:8 C5:2 D4 | C5:12 | D5:8 C5:4 | \
             D5:8 G4:2 D4 | D5:4 C5 Bb4",
            DESARMA_GM_A_CHORDS,
        )
        .in_key(G, HarmonicMinor),
        // Bars 77-86 (choral bars 78-87, "ah"). Bar 82 is printed Gm/Bb, but its notes are bar 81's
        // B flat chord over F.
        Part::new(
            "G minor 2",
            "!p C5:8 G4:2 C4 | C5:4 Bb4 A4 | Bb4:12 | Bb4:4 A4 G4 | F4:12 | F4:12 | !< Bb4:12 | \
             Bb4:8 C5:2 D4 | C5:12 | D5:8 C5:4 !f",
            DESARMA_GM_B_CHORDS,
        )
        .in_key(G, HarmonicMinor),
        // Bars 87-96, one sharp; choral bars 88-97 ("ah"). The last bar ends with the tenors'
        // pickup "Mi-ro al re-de-" (an octave up), from the choral score.
        Part::new(
            "G major",
            "!f D5:8 G4:2 D4 | D5:4 C5 B4 | C5:8 G4:2 C4 | C5:4 B4 A4 | B4:12 | B4:4 A4 G4 | \
             !< G4:8 Eb4:2 Bb3 | Bb4 Bb3 Eb4 G4 Eb4 Bb3 | !ff Bb4 Bb3 Eb4 G4 Eb4 Bb3 | \
             Bb4:4 !p G4:2 G4 A4 G4",
            DESARMA_G_CHORDS,
        )
        .in_key(G, Major),
        // Bars 97-104; choral bars 98-105, the tenors, an octave up: "Miro alrededor,
        // heridas que vienen, sospechas que van y aquí estoy, pensando en el alma que piensa
        // y por pensar no es", over the other voices' chords on "miro alrededor, sospechas
        // que van y aquí estoy".
        Part::new(
            "Verse 3",
            "!p B4:12 | B4:8 -:2 B4:2 | Bb4 G4 G4 G4 G4 G4 | G4 G4 G4 G4 A4 G4 | B4:12 | \
             B4:8 -:2 B4:2 | !< Bb4 G4 G4 G4 G4 G4 | G4 G4 G4 G4 A4 G4 !f",
            DESARMA_VERSE3_CHORDS,
        )
        .in_key(G, Major),
        // Bars 105-112, in 3/4; choral bars 106-113, the sopranos: "alma, desarma y sangra"
        // (the tenors sing D, C sharp below them, the suspension of the chord). The last bar
        // is the choir's "na ra" into the coda: the piano's rising F sharp, G, A stays.
        Part::new(
            "Verse 3 end",
            "!f B4:8 A4:4 | _:8 G4:4 | B4:8 A4:4 | _:8 G4:4 | B4:12 | !> A4:8 G4:4 | F#4:12 | \
             F#5:4 G5 A5 !pp",
            DESARMA_VERSE3_END_CHORDS,
        )
        .in_key(G, Major)
        .in_time(THREE_FOUR)
        .played(Comping::Arp3, Bass::Held, "Waltz"),
        // Bars 113-114, in 3/4: the start of the repeat (choral bars 114-115 and 118-119,
        // "na ra ra rai").
        Part::new(
            "Coda",
            "!pp B5:6 C6:2 B5:2 C6:1 B5:1 | A5:6 B5:1 A5:1 G5:4",
            DESARMA_CODA_CHORDS,
        )
        .in_key(G, Major)
        .in_time(THREE_FOUR)
        .played(Comping::Arp3, Bass::Held, "Waltz"),
        // Bar 115, back in 6/8 (choral bars 116 and 120).
        Part::new("Coda 6/8", "!pp B5:6 C6:2 D6:4", "vi").in_key(G, Major),
        // Bar 116, the first ending (D over F sharp; choral bar 117).
        Part::new("First ending", "!pp D6:6 A4:2 F#4 D5", "V").in_key(G, Major),
        // Bar 117, the second ending (choral bar 121).
        Part::new("Second ending", "!pp F#4:2 A4 D5 D5 D5 D5", "V").in_key(G, Major),
        // Bars 118-127, in 3/4 (choral bars 122-131, "na ra ra rai" and "ah").
        Part::new(
            "Outro",
            "!pp !< D5:2 G5:8 G5:2 | G5:6 C5:4 C5:2 | !mf D5:1 G4 B4 D5 C5 E4 G4 C5 B4 D4 G4 B4 | \
             A4:2 D4 C#4 A3 A4 G4 | F#4 D4 F#4 A4 D5 D5 | B4:6 C5:2 B4:2 C5:1 B4:1 | \
             A4:2 B4:1 A4:1 G4:6 G4:2 | G4:6 A4:2 F#4:4 | !< G4:4 F4 G4 | A4 Bb4 C5 !f",
            DESARMA_OUTRO_CHORDS,
        )
        .in_key(G, Major)
        .in_time(THREE_FOUR)
        .played(Comping::Arp3, Bass::Held, "Waltz"),
        // Bars 128-136 (choral bars 132-140, "na ra ra"): three flats again, the intro an octave
        // lower, slower, and still in 3/4 (no new time signature after bar 118; the right hand is
        // beamed in pairs, the bass moves in half + quarter). The dashed "8" line at bars 132-135
        // hangs under the right-hand staff with its end hook pointing up: an 8va bassa for the
        // right hand (the 3/4 score has the same F3 E flat 3 D3 and an E flat chord topped by E
        // flat 4). The last chord is tied over two bars; the fermata on it is the 3/4 score's.
        Part::new(
            "Final",
            "!f D4:2 Eb4 D4 C4 D4 C4 | B3 G3 B3 D4 G4 F4 | Eb4 Ab3 C4 Ab4 Eb4 Ab3 | \
             Eb4 G3 Bb3 Eb4 Bb3 G3 | !> F3:4 Eb3 D3 | !mp Eb4:12 | Eb4:12 | !> Eb4:12 | != _:12",
            DESARMA_FINAL_CHORDS,
        )
        .in_key(DSharp, Major)
        .in_time(THREE_FOUR)
        .played(Comping::Arp3, Bass::Held, "Waltz")
        .with_tempo(&[tempo(0, 102), tempo(48, 93), fermata(96, 12)]),
    ],
    form: &[
        "Intro",
        "Verse",
        "Verse turn",
        "Verse end",
        "Intro 2",
        "Verse 2",
        "Verse turn",
        "Verse 2 end",
        "Interlude",
        "E flat minor",
        "G minor",
        "G minor 2",
        "G major",
        "Verse 3",
        "Verse 3 end",
        "Coda",
        "Coda 6/8",
        "First ending",
        "Coda",
        "Coda 6/8",
        "Second ending",
        "Outro",
        "Final",
    ],
    drums_from: 0,
    about: "A ballad in E flat that borrows D flat (bVII) from E flat minor; its third line \
            goes through E flat 7 (I7, the V7 of IV) to A flat and through F7 (II7, the V7 of \
            V) to B flat. The middle turns to E flat minor, touches D flat minor, settles \
            in G minor and reaches G major through D7, where E flat comes back as bVI; C minor \
            7 and F lead home to E flat. It sways in 6/8, with whole passages in 3/4.",
};
