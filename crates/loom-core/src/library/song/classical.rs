//! Classical and film: pieces from the concert hall, close to how they were written.

use super::*;

const CANON_TUNE: &str = "F#5:4 E5 D5 C#5 | B4 A4 B4 C#5 | D5 C#5 B4 A4 | G4 F#4 G4 E4 | \
     D4:2 F#4 A4 G4 F#4 D4 F#4 E4 | D4 B3 D4 A4 G4 B4 A4 G4";
const CANON_GROUND: &str = "I V vi iii | IV I IV V";
const CANON_CHORDS: &str =
    "I V vi iii | IV I IV V | I V vi iii | IV I IV V | I V vi iii | IV I IV V";

pub(super) const CANON: Song = Song {
    name: "Canon in D",
    origin: "Johann Pachelbel, around 1700",
    feel: "Baroque",
    genre: Genre::Classical,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 64.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    parts: &[
        Part::new("Ground", "-:16 | -:16", CANON_GROUND),
        // Each violin starts two bars after the one before.
        Part::new("Entries", CANON_TUNE, CANON_CHORDS).with_voices(&[0, 2, 4]),
        Part::new("Canon", CANON_TUNE, CANON_CHORDS).with_voices(&[0, -4, 2, -2, 4]),
        Part::new("Last entries", CANON_TUNE, CANON_CHORDS).with_voices(&[-4, -2]),
    ],
    form: &["Ground", "Entries", "Canon", "Canon", "Last entries"],
    drums_from: 0,
    about: "Three violins play the same line, each two bars behind the last, over a bass of \
            eight notes that repeats from start to end: a ground bass.",
};

pub(super) const FUR_ELISE: Song = Song {
    name: "Für Elise",
    origin: "Ludwig van Beethoven, 1810",
    feel: "Piano",
    genre: Genre::Classical,
    tonic: A,
    mode: HarmonicMinor,
    time: THREE_EIGHT,
    tempo: 66.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    parts: &[
        Part::new("Pickup", "-:4 E5:1 D#5", "").with_left("-:6"),
        Part::new(
            "A",
            "E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 G#4 B4 | C5:2 -:1 E4 E5 D#5 | \
             E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 C5 B4 | A4:4 E5:1 D#5",
            "",
        )
        .with_left(
            "-:6 | A2:1 E3 A3 -:3 | E2:1 E3 G#3 -:3 | A2:1 E3 A3 -:3 | \
             -:6 | A2:1 E3 A3 -:3 | E2:1 E3 G#3 -:3 | A2:1 E3 A3 -:3",
        ),
        Part::new(
            "A into B",
            "E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 G#4 B4 | C5:2 -:1 E4 E5 D#5 | \
             E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 C5 B4 | A4:2 -:1 B4:1 C5 D5",
            "",
        )
        .with_left(
            "-:6 | A2:1 E3 A3 -:3 | E2:1 E3 G#3 -:3 | A2:1 E3 A3 -:3 | \
             -:6 | A2:1 E3 A3 -:3 | E2:1 E3 G#3 -:3 | A2:1 E3 A3 -:3",
        ),
        Part::new(
            "B",
            "E5:3 G4:1 F5 E5 | D5:3 F4:1 E5 D5 | C5:3 E4:1 D5 C5 | B4:2 -:1 E4:1 E5 -:1 | \
             -:1 E5 E6 -:2 D#5:1 | E5:2 -:1 D#5:1 E5 D#5",
            "",
        )
        .with_left(
            "C3:1 G3 C4 -:3 | G2:1 G3 B3 -:3 | A2:1 E3 A3 -:3 | E2:1 E3 E4 -:2 E4:1 | \
             E5:1 -:2 D#5:1 E5 -:1 | -:1 D#5:1 E5 -:3",
        ),
        Part::new(
            "A to the end",
            "E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 G#4 B4 | C5:2 -:1 E4 E5 D#5 | \
             E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 C5 B4 | A4:6",
            "",
        )
        .with_left(
            "-:6 | A2:1 E3 A3 -:3 | E2:1 E3 G#3 -:3 | A2:1 E3 A3 -:3 | \
             -:6 | A2:1 E3 A3 -:3 | E2:1 E3 G#3 -:3 | A2:1 E3 A3:4",
        ),
    ],
    form: &["Pickup", "A", "A into B", "B", "A", "A to the end"],
    drums_from: 0,
    about: "The right hand rocks between E and D sharp, the leading note of A minor, while \
            the left hand answers with broken chords. In 3/8: three eighth notes to the bar.",
};

/// Grieg keeps the bass on the key note while the theme climbs; only the
/// fourth bar lands on the chord of the third.
const MOUNTAIN_CHORDS: &str = "i | i | i | bIII | i | i | i | bIII";

pub(super) const MOUNTAIN_KING: Song = Song {
    name: "In the Hall of the Mountain King",
    origin: "Edvard Grieg, Peer Gynt, 1875",
    feel: "Epic",
    genre: Genre::Classical,
    tonic: B,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 132.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Epic toms",
    parts: &[
        Part::new(
            "Theme",
            "B4:2 C#5 D5 E5 F#5 D5 F#5:4 | F5:2 C#5 F5:4 E5:2 C5 E5:4 | \
             B4:2 C#5 D5 E5 F#5 D5 F#5 B5 | A5 F#5 D5 F#5 A5:8 | \
             B4:2 C#5 D5 E5 F#5 D5 F#5:4 | F5:2 C#5 F5:4 E5:2 C5 E5:4 | \
             B4:2 C#5 D5 E5 F#5 D5 F#5 B5 | A5 F#5 D5 F#5 A5:8",
            MOUNTAIN_CHORDS,
        ),
        // The theme a fifth higher, on F sharp major with its sixth, D,
        // left flat: the augmented chord in the even bars.
        Part::new(
            "Higher",
            "F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:2 A#5 C#6:4 | \
             F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:8 | \
             F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:2 A#5 C#6:4 | \
             F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:8",
            "I | I+ | I | I+ | I | I+ | I | I+",
        )
        .in_key(FSharp, Major),
    ],
    form: &["Theme", "Theme", "Higher", "Higher", "Theme", "Theme"],
    drums_from: 0,
    about: "Trolls creeping up the B minor scale, over a bass that stays on B. Then the \
            whole theme moves up a fifth onto F sharp, major but with its sixth left flat, \
            as it does in Grieg's score, before it comes back. In the original the music \
            keeps getting faster; try raising the tempo while it plays.",
};

pub(super) const MINUET: Song = Song {
    name: "Minuet in G",
    origin: "Christian Petzold, about 1725",
    feel: "Classical",
    genre: Genre::Classical,
    tonic: G,
    mode: Major,
    time: THREE_FOUR,
    tempo: 112.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "",
    parts: &[
        Part::new(
            "A",
            "D5:4 G4:2 A4 B4 C5 | D5:4 G4 G4 | E5:4 C5:2 D5 E5 F#5 | G5:4 G4 G4 | \
             C5:4 D5:2 C5 B4 A4 | B4:4 C5:2 B4 A4 G4 | F#4:4 G4:2 A4 B4 G4 | A4:12 | \
             D5:4 G4:2 A4 B4 C5 | D5:4 G4 G4 | E5:4 C5:2 D5 E5 F#5 | G5:4 G4 G4 | \
             C5:4 D5:2 C5 B4 A4 | B4:4 C5:2 B4 A4 G4 | A4:4 B4:2 A4 G4 F#4 | G4:12",
            "I | I | IV | I | ii | I | V | V | I | I | IV | I | ii | I | V | I",
        ),
        Part::new(
            "B",
            "B5:4 G5:2 A5 B5 G5 | A5:4 D5:2 E5 F#5 D5 | G5:4 E5:2 F#5 G5 D5 | C#5:4 B4:2 C#5 A4:4 | \
             A4:2 B4 C#5 D5 E5 F#5 | G5:4 F#5 E5 | F#5:4 A4 C#5 | D5:12 | \
             D5:4 G4:2 F#4 G4:4 | E5:4 G4:2 F#4 G4:4 | D5:4 C5 B4 | A4:2 G4 F#4 G4 A4:4 | \
             D4:2 E4 F#4 G4 A4 B4 | C5:4 B4 A4 | B4:2 D5 G4:4 F#4 | G4:12",
            "I | V | vi | II | II | I V II | V | V | I | IV | I | V | V | IV I V | I V | I",
        ),
    ],
    form: &["A", "A", "B", "B"],
    drums_from: 0,
    about: "From Anna Magdalena Bach's notebook, long thought to be by Bach himself. The B \
            part travels to D major, the key of the dominant, before coming home.",
};

pub(super) const BLUE_DANUBE: Song = Song {
    name: "The Blue Danube",
    origin: "Johann Strauss II, 1866",
    feel: "Viennese waltz",
    genre: Genre::Classical,
    tonic: D,
    mode: Major,
    time: THREE_FOUR,
    tempo: 156.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Pickup", "-:8 D4:4", "I"),
        Part::new(
            "A",
            "D4:4 F#4 A4 | A4:8 A5:4 | A5:8 F#5:4 | F#5:8 D4:4 | D4:4 F#4 A4 | A4:8 A5:4 | \
             A5:8 G5:4 | G5:8 C#4:4 | C#4:4 E4 B4 | B4:8 B5:4 | B5:8 G5:4 | G5:8 C#4:4 | \
             C#4:4 E4 B4 | B4:8 B5:4 | B5:8 F#5:4 | F#5:8 D4:4",
            "I | I | I | I | I | V7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 | I | I | I",
        ),
        Part::new(
            "B",
            "D4:4 F#4 A4 | D5:8 D6:4 | D6:8 A5:4 | A5:8 D4:4 | D4:4 F#4 A4 | D5:8 D6:4 | \
             D6:8 B5:4 | B5:8 E4:4 | E4:4 G4 B4 | B4:12 | B4:4 G#4 A4 | F#5:12 | F#5:4 D5 F#4 | \
             F#4:8 E4:4 | B4:8 A4:4 | D4:6 D5:2 D5:4 | D5:4 -:4 D4:4",
            "I | I | I | I | I | IV | IV | IV | IV | V7 | V7 | I | I | IV | V7 | I | I",
        ),
    ],
    form: &["Pickup", "A", "B", "A", "B"],
    drums_from: 1,
    about: "The waltz climbs the D major chord, D F sharp A, from a pickup on the third \
            beat, and answers with two high notes: one on the third beat, one on the next \
            downbeat. The second time it climbs the A7 chord instead, C sharp E G: the same \
            shape over the dominant. The B strain leaps up to the high D, as in the piano \
            score.",
};

pub(super) const BRAHMS_LULLABY: Song = Song {
    name: "Brahms' Lullaby",
    origin: "Johannes Brahms, \"Wiegenlied\", 1868",
    feel: "Lullaby",
    genre: Genre::Classical,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 72.0,
    comping: Comping::Sway,
    bass: Bass::Held,
    rhythm: "",
    parts: &[Part::new(
        "Lullaby",
        "-:8 E5:2 E5 | G5:6 E5:2 E5:4 | G5:4 -:4 E5:2 G5 | C6:4 B5:6 A5:2 | A5:4 G5 D5:2 E5 | \
         F5:4 D5 D5:2 E5 | F5:4 -:4 D5:2 F5 | B5:2 A5 G5:4 B5 | C6:4 -:4 C5:2 C5 | C6:8 A5:2 F5 | \
         G5:8 E5:2 C5 | F5:4 G5 A5 | G5:8 C5:2 C5 | C6:8 A5:2 F5 | G5:8 E5:2 C5 | \
         F5:4 E5 D5 | C5:12",
        "I | I | I | V7 | V7 | V7 | V7 | V7 | I | IV | I | IV | I | IV | I | V7 | I",
    )],
    form: &["Lullaby", "Lullaby"],
    drums_from: 0,
    about: "A cradle song that rocks between two chords, C and G7, and then leaps an \
            octave, from low C to high C, for the second half.",
};

pub(super) const MORNING_MOOD: Song = Song {
    name: "Morning Mood",
    origin: "Edvard Grieg, Peer Gynt, 1875",
    feel: "Pastoral",
    genre: Genre::Classical,
    tonic: C,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 66.0,
    comping: Comping::Arp3,
    bass: Bass::Held,
    rhythm: "",
    parts: &[Part::new(
        "Sunrise",
        "G5:2 E5 D5 C5 D5 E5 | G5:2 E5 D5 C5 D5:1 E5 D5 E5 | G5:2 E5 G5 A5 E5 A5 | G5:2 E5 D5 C5:6",
        "I | I | I IV | I",
    )],
    form: &["Sunrise", "Sunrise", "Sunrise", "Sunrise"],
    drums_from: 0,
    about: "The flute's tune uses only the five notes of the major pentatonic, C D E G A, \
            like a shepherd's pipe, rocking gently in 6/8 as the sun comes up.",
};

pub(super) const NEW_WORLD: Song = Song {
    name: "New World Largo",
    origin: "Antonín Dvořák, Symphony No. 9 \"From the New World\", 1893",
    feel: "Film score",
    genre: Genre::Classical,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 60.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    parts: &[Part::new(
        "Largo",
        "E5:6 G5:2 G5:8 | E5:6 D5:2 C5:8 | D5:6 E5:2 G5:6 E5:2 | D5:16 | E5:6 G5:2 G5:8 | \
         E5:6 D5:2 C5:8 | D5:4 E5:4 D5:6 C5:2 | C5:16",
        "I | I | IV | V | I | I | IV V7 | I",
    )],
    form: &["Largo", "Largo"],
    drums_from: 0,
    about: "The cor anglais solo from the slow movement, later sung as \"Goin' Home\". Like \
            a spiritual, it uses only the major pentatonic, and it ends the first line on \
            D, over V, left open like a question.",
};

pub(super) const SHCHEDRYK: Song = Song {
    name: "Shchedryk (Carol of the Bells)",
    origin: "Mykola Leontovych, 1914",
    feel: "Choral",
    genre: Genre::Classical,
    tonic: G,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 168.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    parts: &[
        Part::new(
            "Bells",
            "Bb5:4 A5:2 Bb5 G5:4 | Bb5:4 A5:2 Bb5 G5:4 | Bb5:4 A5:2 Bb5 G5:4 | Bb5:4 A5:2 Bb5 G5:4",
            "i | i | i | i",
        ),
        Part::new(
            "Melody",
            "D6:4 C6:2 D6 Bb5:4 | D6:4 C6:2 D6 Bb5:4 | D6:4 C6:2 D6 Bb5:4 | D6:4 C6:2 D6 Bb5:4 | \
             G6:4 G6:2 G6 F6 Eb6 | D6:4 D6:2 D6 C6 Bb5 | C6:4 C6:2 C6 D6 C6 | G5:4 G5:2 G5 G5:4",
            "",
        )
        .with_left(
            "Bb4:4 A4:2 Bb4 G4:4 | Bb4:4 A4:2 Bb4 G4:4 | Bb4:4 A4:2 Bb4 G4:4 | Bb4:4 A4:2 Bb4 G4:4 | \
             Bb4:4 A4:2 Bb4 G4:4 | Bb4:4 A4:2 Bb4 G4:4 | Bb4:4 A4:2 Bb4 G4:4 | Bb4:4 A4:2 Bb4 G4:4",
        ),
    ],
    form: &["Bells", "Bells", "Melody", "Melody", "Bells"],
    drums_from: 0,
    about: "A Ukrainian New Year song built on one four-note figure, B flat A B flat G, \
            repeated like a bell from start to end while other voices sing above it. An \
            ostinato: a pattern that never stops.",
};

pub(super) const FUNERAL_MARCH: Song = Song {
    name: "Funeral March",
    origin: "Frédéric Chopin, Piano Sonata No. 2, 1839",
    feel: "March",
    genre: Genre::Classical,
    tonic: ASharp,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 56.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "March",
    parts: &[Part::new(
        "March",
        "Bb4:4 Bb4:3 Bb4:1 Bb4:8 | Bb4:4 Bb4:3 Bb4:1 Bb4:8 | Bb4:4 Bb4:3 Bb4:1 Bb4:4 Db5:3 C5:1 | \
         C5:3 Bb4:1 Bb4:3 Bb4:1 Bb4:8 | Db5:4 Db5:3 Db5:1 Db5:4 F5:3 Eb5:1 | \
         Eb5:3 Db5:1 Db5:3 Db5:1 Db5:8 | Bb5:3 Ab5:1 Gb5:3 F5:1 F5:4 Db5:4 | \
         Bb5:3 Ab5:1 Gb5:3 F5:1 F5:4 Db5:4",
        "i bVI i bVI | i bVI i bVI | i bVI i bVI | i bVI i bVI | i bVI i bVI | i bVI i bVI | \
         i bVI i bVI | i bVI i bVI",
    )],
    form: &["March", "March"],
    drums_from: 0,
    about: "The slowest of marches in B flat minor: one note, again and again, over a bass \
            that rocks between B flat and G flat on every beat, two bare fifths without a \
            third. The tune climbs a step at a time, to D flat, then to F.",
};
