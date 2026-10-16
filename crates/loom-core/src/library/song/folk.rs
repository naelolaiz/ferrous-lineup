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
            "C5:8 D5:4 | E5:6 F5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
             C5:8 A4:4 | A4:6 G#4:2 A4:4 | B4:8 G#4:4 | E4:8 A4:4 | \
             C5:8 D5:4 | E5:6 F5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
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
         A5:4 B5 G5 | A5:12 | -:4 D6:4 D6 | D6:8 C6:4 | A5:8 A5:4 | G5:4 F5 E5 | C5:12 | \
         D5:4 A5 G5 | F5:4 E5 D5:2 C5 | D5:12",
        "i | i | bVII | i | i | bIII | IV | i | i | bVII | i | bVII | bVII | i | bVII | i",
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
