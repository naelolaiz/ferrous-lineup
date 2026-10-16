//! Country and folk: folk songs, rounds and carols.

use super::*;

const FRERE_JACQUES_TUNE: &str = "C5:4 D5 E5 C5 | C5 D5 E5 C5 | E5 F5 G5:8 | E5:4 F5 G5:8 | \
     G5:2 A5 G5 F5 E5:4 C5 | G5:2 A5 G5 F5 E5:4 C5 | C5 G4 C5:8 | C5:4 G4 C5:8";
const FRERE_JACQUES_CHORDS: &str = "I | I | I | I | I | I | I | I";

pub(super) const FRERE_JACQUES: Song = Song {
    name: "Frère Jacques",
    origin: "French round, 18th century",
    feel: "Round",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Folk",
    parts: &[
        Part::new("Alone", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS),
        Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[0, 2, 4, 6]),
        // Voices 2 to 4 carry on from the part before, and start again.
        Part::new("Round again", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS)
            .with_voices(&[0, -6, 2, -4, 4, -2, 6]),
        // The first voice has finished; the others sing their last bars.
        Part::new("Last voices", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS)
            .with_voices(&[-6, -4, -2]),
    ],
    form: &["Alone", "Round", "Round again", "Last voices"],
    drums_from: 0,
    about: "A round: four voices sing the same tune, each two bars after the last. It works \
            because every two bars of the tune fit the same chord.",
};

pub(super) const GREENSLEEVES: Song = Song {
    name: "Greensleeves",
    origin: "English folk song, 16th century",
    feel: "Folk ballad",
    genre: Genre::Folk,
    tonic: A,
    mode: NaturalMinor,
    time: THREE_FOUR,
    tempo: 96.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Intro", "-:12 | -:8 A4:4", "i | V"),
        Part::new(
            "Verse",
            "C5:8 D5:4 | E5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
             C5:8 A4:4 | A4:6 G#4:2 A4:4 | B4:8 G#4:4 | E4:8 A4:4 | \
             C5:8 D5:4 | E5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
             C5:6 B4:2 A4:4 | G#4:6 F#4:2 G#4:4 | A4:12 | _:12",
            "i | bIII | bVII | v | i | V | V | i | i | bIII | bVII | v | i | V | i | i",
        ),
        Part::new(
            "Chorus",
            "G5:12 | G5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
             C5:8 A4:4 | A4:6 G#4:2 A4:4 | B4:8 G#4:4 | E4:12 | \
             G5:12 | G5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
             C5:6 B4:2 A4:4 | G#4:6 F#4:2 G#4:4 | A4:12 | _:8 A4:4",
            "bIII | bVII | bVII | v | i | V | V | i | bIII | bVII | bVII | v | i | V | i | i",
        ),
    ],
    form: &["Intro", "Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 1,
    about: "A minor key that borrows a major V (E major) to lead home, in a lilting 3/4. \
            Listen for the A held over E in the sixth bar, resolving to G sharp.",
};

pub(super) const OH_SUSANNA: Song = Song {
    name: "Oh! Susanna",
    origin: "Stephen Foster, 1848",
    feel: "Country",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    parts: &[
        Part::new("Intro", "-:16 | -:14 C5:1 D5", "I | V7"),
        Part::new(
            "Song",
            "E5:2 G5 G5 A5 G5 E5 C5:3 D5:1 | E5:2 E5 D5 C5 D5:6 C5:1 D5 | \
             E5:2 G5 G5 A5 G5 E5 C5:3 D5:1 | E5:2 E5 D5 D5 C5:8 | \
             F5:4 F5 A5:2 A5:4 A5:2 | G5 G5 E5 C5 D5:6 C5:1 D5 | \
             E5:2 G5 G5 A5 G5 E5 C5:3 D5:1 | E5:2 E5 D5 D5 C5:6 C5:1 D5",
            "I | I V | I | V7 I | IV | I V | I | V7 I",
        ),
    ],
    form: &["Intro", "Song", "Song", "Song"],
    drums_from: 1,
    about: "Each line asks and answers: the first stops on D over the V chord (a half \
            cadence), the second lands on C, at home (a full cadence).",
};

pub(super) const SCARBOROUGH_FAIR: Song = Song {
    name: "Scarborough Fair",
    origin: "English ballad, traditional",
    feel: "Dorian ballad",
    genre: Genre::Folk,
    tonic: D,
    mode: Dorian,
    time: THREE_FOUR,
    tempo: 96.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Jazz waltz",
    parts: &[Part::new(
        "Verse",
        "D5:8 D5:4 | A5:8 A5:4 | E5:6 F5:2 E5:4 | D5:12 | -:4 A5:4 C6 | D6:8 C6:4 | \
         A5:4 B5 G5 | A5:12 | _:8 D6:4 | D6:8 D6:4 | C6:8 A5:4 | A5:4 G5 F5 | E5:4 C5:8 | \
         _:12 | D5:8 A5:4 | G5:8 F5:4 | E5:4 D5 C5 | D5:12 | _:12",
        "i | i | bVII | i | bIII | i | IV | i | i | i | bIII | bIII | bVII | bVII | i | bVII | \
         bVII | i | i",
    )],
    form: &["Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "A ballad in the Dorian mode: minor, but with a raised sixth. Listen for the B \
            natural over the G major chord in bar seven, the sound of Dorian.",
};

pub(super) const CLEMENTINE: Song = Song {
    name: "Oh My Darling, Clementine",
    origin: "American folk song, 1884",
    feel: "Country waltz",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 100.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Pickup", "-:8 C5:3 C5:1", "I"),
        Part::new(
            "Verse",
            "C5:4 G4 E5:3 E5:1 | E5:4 C5 C5:3 E5:1 | G5:6 G5:2 F5 E5 | D5:8 D5:3 E5:1 | \
             F5:4 F5 E5:3 D5:1 | E5:4 C5 C5:3 E5:1 | D5:4 G4 B4:2 D5 | C5:8 C5:3 C5:1",
            "I | I | I | V7 | V7 | I | V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse", "Verse"],
    drums_from: 1,
    about: "A gold-rush ballad as a country waltz: the bass on beat one, the chord on two \
            and three. The first line climbs the C major chord, C E G, and the second walks \
            back down over G7.",
};

pub(super) const CAMPTOWN_RACES: Song = Song {
    name: "Camptown Races",
    origin: "Stephen Foster, 1850",
    feel: "Country",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Country train",
    parts: &[Part::new(
        "Song",
        "G5:2 G5 E5 G5 A5 G5 E5:4 | E5:2 D5:6 E5:2 D5:6 | G5:2 G5 E5 G5 A5 G5 E5:4 | \
         D5:2 D5:1 D5 E5:2 D5 C5:8 | C5:2 C5:1 C5 E5:2 G5 C6:8 | A5:2 A5:1 A5 C6:2 A5 G5:8 | \
         G5:2 G5 E5:1 E5 G5 G5 A5:2 G5 E5:4 | D5:2 D5:1 D5 E5:2 D5 C5:8",
        "I | V7 | I | V7 I | I | IV I | I | V7 I",
    )],
    form: &["Song", "Song", "Song"],
    drums_from: 0,
    about: "Foster's tune uses only the major pentatonic, C D E G A. The boom-chick of \
            the two-step puts the bass on beats one and three, the snare on two and four.",
};

const ROW_TUNE: &str = "C5:6 C5 | C5:4 D5:2 E5:6 | E5:4 D5:2 E5:4 F5:2 | G5:12 | \
     C6:2 C6 C6 G5 G5 G5 | E5 E5 E5 C5 C5 C5 | G5:4 F5:2 E5:4 D5:2 | C5:12";
const ROW_CHORDS: &str = "I | I | I | I | I | I | I | I";

pub(super) const ROW_YOUR_BOAT: Song = Song {
    name: "Row, Row, Row Your Boat",
    origin: "American round, 19th century",
    feel: "Round",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 100.0,
    comping: Comping::Arp3,
    bass: Bass::Held,
    rhythm: "Six-eight",
    parts: &[
        Part::new("Alone", ROW_TUNE, ROW_CHORDS),
        Part::new("Round", ROW_TUNE, ROW_CHORDS).with_voices(&[0, 2, 4, 6]),
        Part::new("Round again", ROW_TUNE, ROW_CHORDS).with_voices(&[0, -6, 2, -4, 4, -2, 6]),
        Part::new("Last voices", ROW_TUNE, ROW_CHORDS).with_voices(&[-6, -4, -2]),
    ],
    form: &["Alone", "Round", "Round again", "Last voices"],
    drums_from: 0,
    about: "A round in 6/8 that never leaves the home chord, which is why any voice can \
            start anywhere. \"Merrily\" jumps to the top C and comes down the chord: C G E C.",
};

pub(super) const SILENT_NIGHT: Song = Song {
    name: "Silent Night",
    origin: "Franz Xaver Gruber, 1818",
    feel: "Carol",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 84.0,
    comping: Comping::Arp3,
    bass: Bass::Held,
    rhythm: "Six-eight",
    parts: &[
        Part::new(
            "Verse",
            "G5:6 A5:2 G5:4 | E5:12 | G5:6 A5:2 G5:4 | E5:12 | D6:8 D6:4 | B5:12 | \
             C6:8 C6:4 | G5:12 | A5:8 A5:4 | C6:6 B5:2 A5:4 | G5:6 A5:2 G5:4 | E5:12",
            "I | I | I | I | V7 | V7 | I | I | IV | IV | I | I",
        ),
        Part::new(
            "Ending",
            "A5:8 A5:4 | C6:6 B5:2 A5:4 | G5:6 A5:2 G5:4 | E5:12 | D6:8 D6:4 | F6:6 D6:2 B5:4 | \
             C6:12 | E6:12 | C6:6 G5:2 E5:4 | G5:6 F5:2 D5:4 | C5:12 | _:12",
            "IV | IV | I | I | V7 | V7 | I | I | I | V7 | I | I",
        ),
    ],
    form: &["Verse", "Ending", "Verse", "Ending"],
    drums_from: 0,
    about: "Written for guitar on Christmas Eve 1818, in a gentle 6/8. Only three chords, \
            I, IV and V7, and the tune reaches its highest note, E, on the word \"peace\".",
};

pub(super) const WE_WISH_YOU: Song = Song {
    name: "We Wish You a Merry Christmas",
    origin: "English carol, 16th century",
    feel: "Waltz",
    genre: Genre::Folk,
    tonic: G,
    mode: Major,
    time: THREE_FOUR,
    tempo: 132.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[Part::new(
        "Song",
        "-:8 D5:4 | G5:4 G5:2 A5 G5 F#5 | E5:4 E5 E5 | A5:4 A5:2 B5 A5 G5 | F#5:4 D5 D5 | \
         B5:4 B5:2 C6 B5 A5 | G5:4 E5 D5:2 D5 | E5:4 A5 F#5 | G5:8 D5:4 | G5:4 G5 G5 | \
         F#5:8 F#5:4 | G5:4 F#5 E5 | D5:8 A5:4 | B5:4 A5 G5 | D6:4 D5 D5:2 D5 | E5:4 A5 F#5 | \
         G5:12",
        "I | I | IV | II7 | V | III7 | vi | IV V | I | I | V | V | V | I | V | IV V | I",
    )],
    form: &["Song", "Song", "Song"],
    drums_from: 0,
    about: "Each \"we wish you a merry Christmas\" starts a step higher, over a new chord: \
            C, then A7, then B7. A7 and B7 are secondary dominants, each borrowed to lead to \
            the chord after it.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2623.abc
// (Henrik Norbeck's ABC tunes, song 51; in C, up a fourth from G)
pub(super) const WILD_ROVER: Song = Song {
    name: "The Wild Rover",
    origin: "Irish folk song, traditional",
    feel: "Waltz",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 160.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Pickup", "-:8 C5:4", "I"),
        Part::new(
            "Verse",
            "C5:6 D5:2 C5:4 | A4:4 G4 E5 | E5:4 D5 E5 | F5:12 | _:8 G5:2 G5 | G5:4 E5 G5 | \
             F5:4 D5 B4 | G4:4 E5 D5 | C5:12 | _:8 C5:4",
            "I | I | I | IV | IV | I | V7 | V7 | I | I",
        ),
        Part::new(
            "Verse 2",
            "C5:4 D5 C5 | A4:4 G4 E5 | E5:4 D5 E5 | F5:12 | _:8 E5:2 F5 | G5:4 E5 G5 | \
             F5:4 D5 B4 | G4:4 E5 D5 | C5:4 B4 C5",
            "I | I | I | IV | IV | I | V7 | V7 | I",
        ),
        // The room claps in the empty fourth bar.
        Part::new(
            "Chorus",
            "D5:4 -:8 | D5:4 -:8 | B4:2 G4:6 -:4 | -:12 | -:4 E5:4 E5 | E5:4 D5 E5 | F5:12 | \
             -:4 E5:4 F5 | G5:12 | _:4 E5:4 C5 | B4:4 A4:8 | _:8 A4:4 | G4:4 E5:8 | _:8 D5:4 | \
             C5:12 | _:8 C5:4",
            "V7 | V7 | V7 | V7 | I | I | IV | IV | I | I | V7 | V7 | I | V7 | I | I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse 2", "Chorus"],
    drums_from: 1,
    about: "A pub song in a quick 3/4, sung as a waltz. The verse is all C, F and G7; in the \
            chorus \"no, nay, never\" hangs on G7 for four bars, with a whole bar of rest where \
            the room claps.",
};

// ---------------------------------------------------------------- Folk

// Source: https://raw.githubusercontent.com/SMUGSterling/FretFree/9d95cae4e58abd7d050672ef8a38694665ecd32a/scores/pgh-19010/original.abc
// (Paul Hardy's Session Tunebook; in A, up a tone from G)
pub(super) const SHENANDOAH: Song = Song {
    name: "Shenandoah",
    origin: "American river song, 19th century",
    feel: "Folk ballad",
    genre: Genre::Folk,
    tonic: A,
    mode: Major,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Ballad,
    bass: Bass::Held,
    rhythm: "Folk",
    parts: &[
        Part::new("Pickup", "-:12 E4:4", "I"),
        Part::new(
            "Song",
            "A4:2 A4 A4:6 B4:2 C#5 D5 | F#5:8 E5:4 A5:2 G#5 | F#5:10 E5:2 F#5 E5 | \
             C#5:1 E5:9 -:4 E5:2 | F#5:2 F#5 F#5:6 C#5:2 E5 C#5 | B4:4 A4:6 -:2 A4 B4 | \
             C#5:10 A4:2 C#5 F#5 | E5:4 A4:2 B4 C#5:6 A4:2 | B4:4 A4:8 E4:4",
            "I | IV I | IV | I | vi | IV | I | V I IV IV | V7 I",
        ),
    ],
    form: &["Pickup", "Song", "Song"],
    drums_from: 1,
    about: "A river song that sailors sang as a shanty, slow, with its first word on the \
            pickup. The tune reaches up to the fifth, E, and the high A, and comes down the \
            A major chord; the fifth bar leans on F sharp minor before the last line settles.",
};

// Source: https://www.tunearch.org/wiki/Red_River_Valley
// (Henry Ford, "Good Morning", 1926, in 2/4; here two bars to the bar, in C, up a fifth from F)
pub(super) const RED_RIVER_VALLEY: Song = Song {
    name: "Red River Valley",
    origin: "American folk song, 19th century",
    feel: "Country",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Country train",
    parts: &[
        Part::new("Pickup", "-:12 G4:2 C5", "I"),
        Part::new(
            "Song",
            "E5:4 E5 E5 D5:2 E5 | D5:2 C5:6 _:4 G4:2 C5 | E5:4 C5:2 E5 G5:4 F5:2 E5 | \
             D5:8 D5:2 - G5 F5 | E5:4 E5:2 D5 C5:4 D5:2 E5 | G5:2 F5:6 A4:4 B4:2 A4 | \
             G4:4 B4:2 C5 D5:4 E5:2 D5 | C5:8 C5:2 - G4 C5",
            "I | V7 I | I | V7 | I | I7 IV | I V7 | I",
        ),
    ],
    form: &["Pickup", "Song", "Song", "Song"],
    drums_from: 1,
    about: "A prairie love song in 2/4, written here two bars to the bar, from a 1926 \
            collection of old-time dance tunes. Each line starts on the pickup and climbs to \
            the third, E; in the sixth bar C turns into C7 to reach F, the only chord outside \
            C and G7.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201287218.abc
// (Frank Nordberg, Musica Viva; in C, up a fourth from G)
pub(super) const HOME_ON_THE_RANGE: Song = Song {
    name: "Home on the Range",
    origin: "Daniel E. Kelley and Brewster Higley, 1870s",
    feel: "Country waltz",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 132.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Pickup", "-:8 G4:4", "I"),
        Part::new(
            "Verse",
            "G4:4 C5 D5 | E5:8 C5:2 B4 | A4:4 F5 F5 | F5:8 E5:2 F5 | G5:6 C5:2 C5:4 | \
             C5:4 B4 C5 | D5:12 | _:4 - G4:2 G4 | G4:4 C5 D5 | E5:8 C5:2 B4 | A4:4 F5 F5 | \
             F5:8 F5:2 F5 | E5:6 D5:2 C5:4 | B4:4 C5 D5 | C5:12 | _:4 -:8",
            "I | I | IV | IV | I | I | V7 | V7 | I | I | IV | IV | I | V7 | I | I",
        ),
        Part::new(
            "Chorus",
            "G5:12 | F5:4 E5:6 D5:2 | E5:12 | _:4 - G4:2 G4 | C5:6 C5:2 C5:4 | C5:4 B4 C5 | \
             D5:12 | _:4 - G4:2 G4 | G4:4 C5 D5 | E5:8 C5:2 B4 | A4:4 F5 F5 | F5:8 F5:2 F5 | \
             E5:6 D5:2 C5:4 | B4:4 C5 D5 | C5:12 | _:4 - G4:4",
            "I | V7 | I | I | I | I | V7 | V7 | I | I | IV | IV | I | V7 | I | I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus"],
    drums_from: 1,
    about: "Higley's poem of the 1870s set to Kelley's tune. Each line starts on the pickup \
            and climbs the C major chord, C D E, to F over the IV chord; the chorus opens on \
            the high G, held for a whole bar, and then sings the verse's second half again.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2726.abc
// (Henrik Norbeck's ABC tunes, song 154; in F, up a minor third from D)
pub(super) const DANNY_BOY: Song = Song {
    name: "Danny Boy",
    origin: "Irish air \"Londonderry Air\", words by Frederic Weatherly, 1913",
    feel: "Ballad",
    genre: Genre::Folk,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 66.0,
    comping: Comping::Ballad,
    bass: Bass::Held,
    rhythm: "Pop ballad",
    parts: &[
        Part::new("Pickup", "-:10 E4:2 F4 G4", "I"),
        Part::new(
            "Verse",
            "A4:6 G4:2 A4 D5 C5 A4 | G4:2 F4 D4:6 F4:2 A4 Bb4 | C5:6 D5:2 C5 A4 F4 A4 | \
             G4:8 -:2 E4 F4 G4 | A4:6 G4:2 A4 D5 C5 A4 | G4:2 F4 D4:6 E4:2 F4 G4 | \
             A4:6 Bb4:2 A4 G4 F4 E4 | F4:8 -:2 C5 D5 E5",
            "I | IV | I I I vi | II7 V7 | I I I I7 | IV V7 | I V7 | I",
        ),
        Part::new(
            "Chorus",
            "F5:6 E5:2 E5 D5 C5 D5 | C5:2 A4 F4:6 C5:2 D5 E5 | F5:6 E5:2 E5 D5 C5 A4 | \
             G4:8 -:2 C5 C5 C5 | A5:6 G5:2 G5 F5 D5 F5 | C5:2 A4 F4:6 E4:2 F4 G4 | \
             A4:2 D5 C5 A4 G4 F4 D4 E4 | F4:8 -:2 E4 F4 G4",
            "I | IV V7 | I I IV III7 | II7 V7 | I IV | I vi7 IV V7 | I V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus"],
    drums_from: 1,
    about: "The Londonderry Air, printed in 1855, with Weatherly's words of 1913. The verse \
            stays low and the chorus leaps up to the high F and A; listen for the G7, a \
            secondary dominant, pulling toward C7 at the end of each fourth bar.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201303027.abc
// (Digital Tradition; in C, up a fourth from G)
pub(super) const LOCH_LOMOND: Song = Song {
    name: "Loch Lomond",
    origin: "Scottish folk song, published 1841",
    feel: "Folk",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Folk",
    parts: &[
        Part::new("Pickup", "-:12 G4:4", "I"),
        Part::new(
            "Verse",
            "C5:4 C5:2 D5 E5:4 D5:2 C5 | D5:4 D5:2 C5 A4:4 G4:2 G4 | C5:4 C5 C5 E5:2 G5 | \
             A5:8 G5:4 G5 | A5:4 A5:2 G5 E5:4 E5:2 G5 | F5:2 E5 D5 C5 A4:4 G4:2 A4 | \
             C5:2 C5 E5 G5 A5:4 G5:2 E5 | D5:8 C5:4 G4",
            "I | ii V7 | I | vi | IV | ii V7 | I IV | V7 I",
        ),
        Part::new(
            "Chorus",
            "C5:4 C5:3 D5:1 E5:4 E5:2 D5 | C5:4 C5:3 A4:1 G4:4 G4:2 G4 | C5:4 C5:2 C5 C5:4 E5:2 G5 | \
             A5:8 G5:4 G5 | A5:4 A5:2 G5 E5:4 E5:2 G5 | F5:2 E5 D5 C5 A4:4 G4:2 A4 | \
             C5:2 C5 E5 G5 A5:4 G5:2 E5 | D5:8 C5:4 G4",
            "I | I V7 | I | vi | IV | ii V7 | I IV | V7 I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus"],
    drums_from: 1,
    about: "First printed in 1841. The verse and the chorus share their second half, so the \
            song is really one tune with two openings. It stays on the pentatonic scale, \
            C D E G A, except for the F that starts the sixth bar of each.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201360904.abc
pub(super) const WAYFARING_STRANGER: Song = Song {
    name: "Wayfaring Stranger",
    origin: "American folk hymn, 19th century",
    feel: "Folk ballad",
    genre: Genre::Folk,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::ArpUpDown,
    bass: Bass::Held,
    rhythm: "Folk",
    parts: &[
        Part::new("Pickup", "-:10 A4:2 A4 E5", "i"),
        Part::new(
            "Verse",
            "E5:10 D5:2 E5 D5 | C5:2 A4:6 -:2 A4 A4 E5 | D5:10 A4:2 C5 D5 | E5:8 -:2 A4 A4 E5 | \
             E5:10 D5:2 E5 D5 | C5:2 A4:6 -:2 A4 A4 E5 | D5:8 D5:2 C5 A4 A4 | A4:8 -:2 E5 E5 G5",
            "i | i | iv | V | i | i | iv | i",
        ),
        Part::new(
            "Chorus",
            "A5:10 G5:2 A5 G5 | E5:2 C5:6 -:2 E5 E5 G5 | A5:10 G5:2 A5 G5 | E5:8 -:2 E5 F5 G5 | \
             E5:10 C5:2 D5 E5 | C5:2 B4:6 -:2 A4 A4 E5 | D5:10 C5:2 A4 G4 | A4:8 -:2 A4 A4 E5",
            "bVI bVII | bIII | bVI bVII | i | i | V | iv | i",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus"],
    drums_from: 1,
    about: "A folk hymn in A minor whose tune never raises its seventh: the verse moves \
            between A minor, D minor and E, and the chorus goes to F and G, the relative \
            major's IV and V, before it comes home. Every phrase starts on the pickup \
            \"I am a\".",
};

// Source: https://tunearch.org/wiki/Annotation:Cotton_Eyed_Joe_(1)
// (R. P. Christeson, Old Time Fiddlers Repertory vol. 1, 1973, in 2/4; here two bars to the bar)
pub(super) const COTTON_EYED_JOE: Song = Song {
    name: "Cotton-Eyed Joe",
    origin: "American fiddle tune, 19th century",
    feel: "Bluegrass",
    genre: Genre::Folk,
    tonic: A,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Bluegrass",
    parts: &[
        Part::new("Pickup", "-:14 E4:1 F#4", "I"),
        Part::new(
            "A",
            "A4:2 A4:1 C#5 B4 A4 F#4 G#4 A4 G#4 A4 B4 C#5:2 C#5:1 B4 | \
             A4:2 A4:1 C#5 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 E4:1 F#4 | \
             A4:2 A4:1 C#5 B4 A4 F#4 G#4 A4 G#4 A4 B4 C#5:2 C#5:1 D5 | \
             E5:1 F#5 E5 C#5 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 E4:1 F#4 | \
             A4:2 A4:1 C#5 B4 A4 F#4 G#4 A4 G#4 A4 B4 C#5:2 C#5:1 B4 | \
             A4:2 A4:1 C#5 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 E4:1 F#4 | \
             A4:2 A4:1 C#5 B4 A4 F#4 G#4 A4 G#4 A4 B4 C#5:2 C#5:1 D5 | \
             E5:1 F#5 E5 C#5 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 C#5:1 B4",
            "I | I V | I | V I | I | I V | I | V I",
        ),
        Part::new(
            "B",
            "A4:1 B4 C#5 D5 E5 C#5 E5 C#5 F#5:2 E5 A5:4 | \
             F#5:1 E5 C#5 A4 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 E4 | \
             A4:1 B4 C#5 D5 E5 C#5 E5 C#5 F#5:2 E5 A5:4 | \
             F#5:1 E5 C#5 A4 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 C#5:1 B4 | \
             A4:1 B4 C#5 D5 E5 C#5 E5 C#5 F#5:2 E5 A5:4 | \
             F#5:1 E5 C#5 A4 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 E4 | \
             A4:1 B4 C#5 D5 E5 C#5 E5 C#5 F#5:2 E5 A5:4 | \
             F#5:1 E5 C#5 A4 B4 A4 F#4 G#4 A4 B4 A4 F#4 E4:2 E4:1 F#4",
            "I | I V | I | V I | I | I V | I | V I",
        ),
    ],
    form: &["Pickup", "A", "B"],
    drums_from: 1,
    about: "An old Southern fiddle tune, two bars of 2/4 to each bar here. It is almost all \
            sixteenth notes circling the A chord, with G sharp as the leading note back to A; \
            the high part leaps up to the A an octave above and runs back down.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201319226.abc
pub(super) const OLD_JOE_CLARK: Song = Song {
    name: "Old Joe Clark",
    origin: "American fiddle tune, 19th century",
    feel: "Bluegrass",
    genre: Genre::Folk,
    tonic: A,
    mode: Mixolydian,
    time: COMMON,
    tempo: 160.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Bluegrass",
    parts: &[
        Part::new(
            "A",
            "E5:4 F#5 G5 F#5 | E5:4 D5 C#5:8 | E5:4 F#5 G5 F#5 | E5:8 E5 | E5:4 F#5 G5 F#5 | \
             E5:4 D5 C#5:8 | A4:2 B4 C#5 A4 B4:4 G4 | A4:8 A4",
            "I | I | I | V | I | I | I bVII | I",
        ),
        Part::new(
            "B",
            "A4:4 A4:2 B4 C#5 B4 A4:4 | E5:4 D5 C#5:8 | A4:4 A4:2 B4 C#5 B4 A4:4 | G4:8 G4 | \
             A4:4 A4:2 B4 C#5 B4 A4:4 | E5:4 D5 C#5:8 | A4:2 B4 C#5 A4 B4:4 G4 | A4:8 A4",
            "I | I | I | bVII | I | I | I bVII | I",
        ),
    ],
    form: &["A", "A", "B", "B"],
    drums_from: 0,
    about: "A fiddle tune in A Mixolydian: A major with a G natural, which you hear in the G \
            chord at the end of the seventh bar and in the low part's bar of G. The chords \
            barely move, so the drive is all in the tune.",
};

// Source: https://hg.lunch.org.uk/dottes/annotate/1022593e211a/Session/SoldiersJoy.abc
// (agrees with McGlashan's Collection of Scots Measures, c. 1781, at https://tunearch.org/wiki/Annotation:Soldier's_Joy_(1))
pub(super) const SOLDIERS_JOY: Song = Song {
    name: "Soldier's Joy",
    origin: "Fiddle tune, Scotland and America, 18th century",
    feel: "Bluegrass",
    genre: Genre::Folk,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 160.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Bluegrass",
    parts: &[
        Part::new("Pickup", "-:12 F#4:2 G4", "I"),
        Part::new(
            "A",
            "A4:2 F#4 D4 F#4 A4 F#4 D4 F#4 | A4:4 D5 D5 C#5:2 B4 | A4:2 F#4 D4 F#4 A4 F#4 D4 F#4 | \
             G4:4 E4 E4 F#4:2 G4 | A4:2 F#4 D4 F#4 A4 F#4 D4 F#4 | A4:4 D5 D5 E5 | \
             F#5:2 A5 F#5 D5 E5 G5 E5 C#5 | D5:4 D5 D5 F#4:2 G4 | \
             A4:2 F#4 D4 F#4 A4 F#4 D4 F#4 | A4:4 D5 D5 C#5:2 B4 | A4:2 F#4 D4 F#4 A4 F#4 D4 F#4 | \
             G4:4 E4 E4 F#4:2 G4 | A4:2 F#4 D4 F#4 A4 F#4 D4 F#4 | A4:4 D5 D5 E5 | \
             F#5:2 A5 F#5 D5 E5 G5 E5 C#5 | D5:4 D5 D5 D5:2 E5",
            "I | I IV | I | II7 V7 | I | I IV | I V7 | I | I | I IV | I | II7 V7 | I | I IV | I V7 | I",
        ),
        Part::new(
            "B",
            "F#5:4 F#5 F#5:2 A5 G5 F#5 | E5:4 E5 E5:2 G5 F#5 E5 | F#5:4 F#5 F#5:2 A5 G5 F#5 | \
             E5:2 D5 C#5 B4 A4:4 D5:2 E5 | F#5:4 F#5 F#5:2 A5 G5 F#5 | E5:4 E5 E5:2 G5 F#5 E5 | \
             F#5:2 A5 F#5 D5 E5 G5 E5 C#5 | D5:4 D5 D5 D5:2 E5 | \
             F#5:4 F#5 F#5:2 A5 G5 F#5 | E5:4 E5 E5:2 G5 F#5 E5 | F#5:4 F#5 F#5:2 A5 G5 F#5 | \
             E5:2 D5 C#5 B4 A4:4 D5:2 E5 | F#5:4 F#5 F#5:2 A5 G5 F#5 | E5:4 E5 E5:2 G5 F#5 E5 | \
             F#5:2 A5 F#5 D5 E5 G5 E5 C#5 | D5:4 D5 D5 F#4:2 G4",
            "I | V7 | I | II7 V | I | V7 | I V7 | I | I | V7 | I | II7 V | I | V7 | I V7 | I",
        ),
    ],
    form: &["Pickup", "A", "B"],
    drums_from: 1,
    about: "One of the oldest fiddle tunes still played, in print in Scotland by the 1780s. \
            The low part runs up and down the D chord in eighth notes; the high part answers \
            on F sharp and E over D and A7, with an E7 pulling toward A in every fourth bar.",
};

// Source: https://tunearch.org/wiki/Annotation:Turkey_in_the_Straw
// (O'Neill's Music of Ireland, 1903, no. 1520, as a reel; in C, up a fourth from G)
pub(super) const TURKEY_IN_THE_STRAW: Song = Song {
    name: "Turkey in the Straw",
    origin: "American fiddle tune, 1830s",
    feel: "Country",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Country train",
    parts: &[
        Part::new("Pickup", "-:12 E5:2 D5", "I"),
        Part::new(
            "A",
            "C5:2 A4:4 B4:2 A4 G4 E4 F4 | G4:2 A4 G4 E4 G4 A4 C5 D5 | E5:2 D5 E5 F5 G5 E5 C5 D5 | \
             E5:2 D5:4 C5:2 D5 F5 E5 D5 | C5:2 A4:4 B4:2 A4 G4 E4 F4 | G4:2 A4 G4 E4 G4 A4 C5 D5 | \
             E5:2 G5:4 A5:2 G5 E5 C5 D5 | E5:2 C5 D5 B4 C5:4 E5:2 D5 | \
             C5:2 A4:4 B4:2 A4 G4 E4 F4 | G4:2 A4 G4 E4 G4 A4 C5 D5 | E5:2 D5 E5 F5 G5 E5 C5 D5 | \
             E5:2 D5:4 C5:2 D5 F5 E5 D5 | C5:2 A4:4 B4:2 A4 G4 E4 F4 | G4:2 A4 G4 E4 G4 A4 C5 D5 | \
             E5:2 G5:4 A5:2 G5 E5 C5 D5 | E5:2 C5 D5 B4 C5:4 C5:2 D5",
            "I | I | I | I V7 | I | I | I | V7 I | I | I | I | I V7 | I | I | I | V7 I",
        ),
        // The triplet in the fourth bar is written as two sixteenths and an eighth.
        Part::new(
            "B",
            "E5:2 G5:4 A5:2 G5 E5 C5 D5 | E5:2 G5 G5 A5 G5 F5 E5 D5 | E5:2 G5 A5 B5 C6 B5 A5 G5 | \
             E5:2 D5 E5:1 F5 G5:2 A5:4 A5:2 B5 | C6:2 B5 C6 A5 G5 C6 A5 G5 | E5:2 G5 D5 C5 A4:4 C5:2 D5 | \
             E5:2 G5 D5 C5 A4 G4 E4 G4 | A4:2 C5:4 D5:2 C5:4 C5:2 D5 | \
             E5:2 G5:4 A5:2 G5 E5 C5 D5 | E5:2 G5 G5 A5 G5 F5 E5 D5 | E5:2 G5 A5 B5 C6 B5 A5 G5 | \
             E5:2 D5 E5:1 F5 G5:2 A5:4 A5:2 B5 | C6:2 B5 C6 A5 G5 C6 A5 G5 | E5:2 G5 D5 C5 A4:4 C5:2 D5 | \
             E5:2 G5 D5 C5 A4 G4 E4 G4 | A4:2 C5:4 D5:2 C5:4 E5:2 D5",
            "I | V7 | I | V7 IV | I | I IV | I V7 | I | I | V7 | I | V7 IV | I | I IV | I V7 | I",
        ),
    ],
    form: &["Pickup", "A", "B"],
    drums_from: 1,
    about: "A fiddle tune of the 1830s minstrel stage, here from O'Neill's 1903 collection, \
            where it is a reel. The low part circles C and G7 in eighth notes; the high part \
            leaps to the C above and runs back down the scale.",
};

// Source: https://tunearch.org/wiki/Irish_Washerwoman_(1)
// (Thomas Wilson, A Companion to the Ball Room, London, 1816)
pub(super) const IRISH_WASHERWOMAN: Song = Song {
    name: "The Irish Washerwoman",
    origin: "Irish jig, 18th century",
    feel: "Jig",
    genre: Genre::Folk,
    tonic: G,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 176.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    parts: &[
        Part::new("Pickup", "-:10 D5:1 C5", "I"),
        Part::new(
            "A",
            "B4:2 G4 G4 D4 G4 G4 | B4:2 G4 B4 D5 C5 B4 | C5:2 A4 A4 E4 A4 A4 | C5:2 A4 C5 E5 D5 C5 | \
             B4:2 G4 G4 D4 G4 G4 | B4:2 G4 B4 D5 C5 B4 | C5:2 B4 C5 A4 D5 D5 | B4:2 G4 G4 G4:4 D5:1 C5 | \
             B4:2 G4 G4 D4 G4 G4 | B4:2 G4 B4 D5 C5 B4 | C5:2 A4 A4 E4 A4 A4 | C5:2 A4 C5 E5 D5 C5 | \
             B4:2 G4 G4 D4 G4 G4 | B4:2 G4 B4 D5 C5 B4 | C5:2 B4 C5 A4 D5 D5 | B4:2 G4 G4 G4:4 G5:1 A5",
            "I | I | ii | V7 | I | I | IV V7 | I | I | I | ii | V7 | I | I | IV V7 | I",
        ),
        Part::new(
            "B",
            "B5:2 G5 G5 D5 G5 G5 | B5:2 G5 B5 B5 A5 G5 | A5:2 F#5 D5 A4 D5 F#5 | F#5:2 D5 F#5 A5 G5 F#5 | \
             E5:2 G5 G5 D5 G5 G5 | C5:2 G5 G5 B4 G5 G5 | C5:2 B5 C5 A4 D5 D5 | B4:2 G4 G4 G4:4 G5:1 A5 | \
             B5:2 G5 G5 D5 G5 G5 | B5:2 G5 B5 B5 A5 G5 | A5:2 F#5 D5 A4 D5 F#5 | F#5:2 D5 F#5 A5 G5 F#5 | \
             E5:2 G5 G5 D5 G5 G5 | C5:2 G5 G5 B4 G5 G5 | C5:2 B5 C5 A4 D5 D5 | B4:2 G4 G4 G4:4 D5:1 C5",
            "I | I | V7 | V7 | IV I | IV I | IV V7 | I | I | I | V7 | V7 | IV I | IV I | IV V7 | I",
        ),
    ],
    form: &["Pickup", "A", "B"],
    drums_from: 1,
    about: "The best known of all jigs, in print by the 1780s and here from a London dance \
            manual of 1816. Each bar is two groups of three eighth notes, the pulse of 6/8; \
            the tune bounces off the low G and D while the chords trade G and D7.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/folkinfo/abc_tune_folkinfo_1616.abc
// (BBC "Singing Together", 1985, the 1903 Cowan version, in 2/4; here two bars to the bar, in G, up a tone from F)
pub(super) const WALTZING_MATILDA: Song = Song {
    name: "Waltzing Matilda",
    origin: "Banjo Paterson and Marie Cowan, Australia, 1903",
    feel: "Folk",
    genre: Genre::Folk,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Folk",
    parts: &[
        Part::new(
            "Verse",
            "B4:1 B4 B4 B4 A4:2 A4 G4:1 A4 B4 G4 E4 F#4 G4:2 | \
             D4:2 G4:1 B4 D5:2 D5:1 D5 D5:2 D5:1 D5 D5:2 G4:1 A4 | \
             B4:2 B4:1 B4 A4:2 A4 G4:1 A4 B4 G4 E4 F#4 G4:2 | \
             B4:1 D4:2 G4:1 D5:2 C5:1 B4 A4:2 A4:1 A4 G4:4",
            "I V I I | I I V7 V7 | I V I I | I I V7 I",
        ),
        Part::new(
            "Chorus",
            "D5:2 D5:1 D5 D5:2 B4 G5:2 G5:1 G5 F#5:2 E5 | \
             D5:2 D5:1 D5 E5:2 D5:1 D5 D5:2 C5:1 B4 A4:2 G4:1 A4 | \
             B4:2 B4:1 B4 A4:2 A4 G4:1 A4 B4 G4 E4 F#4 G4:2 | \
             D4:2 G4:1 B4 D5:2 C5:1 B4 A4:2 A4:1 A4 G4:4",
            "I I IV IV | I I V7 V7 | I V I I | I I V7 I",
        ),
    ],
    form: &["Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 0,
    about: "Australia's unofficial anthem in the 1903 Cowan version, two bars of 2/4 to the \
            bar here. The tune starts on the third, B, and the chorus jumps up to the high G \
            over the C chord; every line ends back on G.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2718.abc
// (Henrik Norbeck's ABC tunes, song 146; in C, up a fourth from G)
pub(super) const MOLLY_MALONE: Song = Song {
    name: "Molly Malone",
    origin: "Irish song, 1880s",
    feel: "Waltz",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 150.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Pickup", "-:8 G4:4", "I"),
        Part::new(
            "Verse",
            "C5:4 C5 C5 | C5:2 E5:6 C5:2 C5 | D5:4 D5 D5 | D5:2 F5:6 D5:4 | E5:4 D5 C5 | \
             G5:4 F5 E5 | E5:4 D5 C5 | D5:8 G4:2 G4 | C5:4 C5 C5 | C5:2 E5:6 C5:2 C5 | \
             D5:4 D5 D5 | D5:2 F5:6 D5:2 D5 | E5:2 G5:6 F5:4 | E5:2 G5:6 F5:4 | E5:6 C5:2 D5:4 | \
             C5:8 G4:4",
            "I | I | V7 | V7 | I | I | I | V7 | I | I | V7 | V7 | I | I | V7 | I",
        ),
        Part::new(
            "Chorus",
            "C5:6 C5:2 C5:4 | C5:2 E5:6 C5:4 | D5:6 D5:2 D5:4 | D5:2 F5:6 D5:2 D5 | \
             E5:2 G5:6 F5:4 | E5:2 G5:6 F5:4 | E5:6 C5:2 D5:4 | C5:8 G4:4",
            "I | I | V7 | V7 | I | I | V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus"],
    drums_from: 1,
    about: "The Dublin street-seller's song, printed in the 1880s, in a quick 3/4 felt one \
            beat to the bar. Two chords, C and G7, trade places every two bars, and the \
            chorus \"alive, alive, oh\" is the verse's second half sung again.",
};
