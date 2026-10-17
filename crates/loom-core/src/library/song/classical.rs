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
    style: "Hymn",
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
    style: "",
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
    style: "Epic minor",
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
    style: "Waltz",
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
    style: "Waltz",
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
    style: "Waltz",
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
    style: "Six-eight ballad",
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
    style: "Hymn",
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
    style: "",
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
    style: "Epic minor",
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

// Source: https://raw.githubusercontent.com/cuthbertLab/music21/master/music21/corpus/verdi/laDonnaEMobile.mxl
// (MusicXML in the music21 corpus, in B flat; Verdi's key is B major)
pub(super) const LA_DONNA_E_MOBILE: Song = Song {
    name: "La donna è mobile",
    origin: "Giuseppe Verdi, Rigoletto, 1851",
    feel: "Six-eight",
    genre: Genre::Classical,
    tonic: ASharp,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 192.0,
    comping: Comping::Sway,
    bass: Bass::Held,
    rhythm: "Six-eight",
    style: "Six-eight ballad",
    parts: &[
        Part::new(
            "Verse",
            "D5:2 D5:2 D5:2 F5:1 Eb5:1 C5:4 | C5:2 C5:2 C5:2 Eb5:1 D5:1 Bb4:4 | \
             D5:2 C5:2 Bb4:2 Bb4:1 A4:1 A4:4 | C5:2 Bb4:2 G4:2 G4:1 F4:1 F4:4 | \
             D5:2 D5:2 D5:2 F5:1 Eb5:1 C5:4 | C5:2 C5:2 C5:2 Eb5:1 D5:1 Bb4:4 | \
             D5:2 C5:2 Bb4:2 Bb4:1 A4:1 A4:4 | C5:2 Bb4:2 G4:2 G4:1 F4:1 F4:4",
            "I V7 | V7 I | I V7 | V7 I | I V7 | V7 I | I V7 | V7 I",
        ),
        Part::new(
            "Refrain",
            "C5:1 D5:1 C5:2 C5:2 F5:2 -:4 | D5:1 Eb5:1 D5:2 D5:2 G5:2 -:4 | \
             F5:1 G5:1 F5:2 F5:2 G5:4 F5:2 | Eb5:1 F5:1 D5:1 -:1 C5:1 -:1 Bb4:4 -:2",
            "ii7 V | III7 vi | V7 I | ii V7 I I",
        ),
    ],
    form: &["Verse", "Refrain", "Verse", "Refrain"],
    drums_from: 0,
    about: "The Duke's song is in 3/8; two of Verdi's bars make one bar of 6/8 here, so the \
            six-eight drums fit. It sits in B flat, as in the source, a semitone below \
            Verdi's B major. The verse rocks between I and V7 and nothing else; the \
            refrain's high notes come over the ii chord and over D7, the dominant of \
            the relative minor. The leaps to the top F at the very end are left out.",
};

// Source: https://www.mutopiaproject.org/ftp/HolstGT/Thaxted/Thaxted.ly
// (the hymn tune Thaxted, Methodist Hymnbook 1933, written in C; here in F)
pub(super) const JUPITER_HYMN: Song = Song {
    name: "Jupiter Hymn",
    origin: "Gustav Holst, The Planets, 1916",
    feel: "Hymn",
    genre: Genre::Classical,
    tonic: F,
    mode: Major,
    time: THREE_FOUR,
    tempo: 84.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    style: "Waltz",
    parts: &[
        Part::new("Pickup", "-:8 A4:2 C5:2", "I"),
        Part::new(
            "Verse",
            "D5:6 F5:2 E5:3 C5:1 | F5:2 G5:2 F5:4 E5:4 | D5:2 E5:2 D5:4 C5:4 | A4:8 A4:2 C5:2 | \
             D5:6 F5:2 E5:3 C5:1 | F5:2 G5:2 A5:4 A5:4 | A5:2 G5:2 F5:4 G5:4 | F5:8 C5:2 A4:2",
            "IV IV ii | I | IV IV V | vi vi V | IV IV ii | I | IV | I",
        ),
        Part::new(
            "Middle",
            "G4:6 G4:2 F4:2 A4:2 | G4:4 C5:4 C5:2 A4:2 | G4:6 G4:2 A4:2 C5:2 | D5:8 D5:2 E5:2 | \
             F5:3 F5:1 E5:4 D5:4 | C5:4 F5:4 A4:4 | G4:2 F4:2 G4:4 A4:4 | C5:8 A4:2 C5:2",
            "V V vi | V V I | V V I | IV IV ii | vi I IV | I ii7 I | IV ii vi | V vi7 V",
        ),
        Part::new(
            "Verse end",
            "D5:6 F5:2 E5:3 C5:1 | F5:2 G5:2 F5:4 E5:4 | D5:2 E5:2 D5:4 C5:4 | A4:8 A4:2 C5:2 | \
             D5:6 F5:2 E5:3 C5:1 | F5:2 G5:2 A5:4 A5:4 | A5:2 G5:2 F5:4 G5:4 | F5:12",
            "IV IV V7 | I I iii | IV IV V | vi vi V | IV IV V7 | I | IV IV ii | I",
        ),
    ],
    form: &["Pickup", "Verse", "Middle", "Verse end"],
    drums_from: 0,
    about: "The broad tune from the middle of Jupiter, which Holst later made into the hymn \
            \"I Vow to Thee, My Country\" (the tune is called Thaxted). Every line starts \
            with two eighth notes that lead into the next bar, and the first chord of each \
            line is IV, not I. The middle strain sits on the dominant and climbs back. The \
            hymnbook has it in C; here it is in F.",
};

// Source: https://www.mutopiaproject.org/ftp/SchubertF/D839/SchubertF-D839_AveMaria/SchubertF-D839_AveMaria.ly
pub(super) const AVE_MARIA: Song = Song {
    name: "Ave Maria",
    origin: "Franz Schubert, \"Ellens Gesang III\", 1825",
    feel: "Art song",
    genre: Genre::Classical,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 56.0,
    comping: Comping::Arp16,
    bass: Bass::Held,
    rhythm: "",
    style: "Classical cadence",
    parts: &[
        Part::new("Intro", "-:16 | -:16", "I I I I7 | IV vii°7 I I"),
        Part::new(
            "Verse",
            "Bb4:6 A4:1 Bb4:1 D5:7 C5:1 | Bb4:4 -:4 C5:4 Bb4:1 A4:1 G4:1 A4:1 | \
             Bb4:4 -:2 D5:2 D5:3 C5:1 A4:1 G4:1 D5:1 E5:1 | \
             D5:4 C#5:3 A4:1 C5:3 Bb4:1 A4:1 D5:1 Eb5:1 A4:1 | \
             Bb4:6 D5:1 C5:1 C5:3 A4:1 G4:1 D5:1 F5:1 B4:1 | \
             C5:4 G4:1 A4:1 Bb4:1 A4:1 F4:4 -:2 F4:2 | \
             C5:3 C5:1 C5:1 B4:1 C5:1 D5:1 C5:1 D5:1 Bb4:2 -:2 Bb4:2 | \
             C5:3 C5:1 C5:1 B4:1 Eb5:1 D5:1 Bb4:4 -:2 Bb4:2 | \
             C5:3 C5:1 D5:2 D5:1 C5:1 F5:1 Eb5:1 Eb5:2 -:3 G4:1 | \
             C5:3 C5:1 Bb4:1 A4:1 Db5:1 Bb4:1 C5:6 -:2 | \
             Bb4:6 A4:1 Bb4:1 D5:7 C5:1 | Bb4:4 -:12 | -:16",
            "I #ivø7 I V7 | vi vi ii V7 | I I I+ vi | VII7 VII7 #v°7 #v°7 | vi vi6 V VI7 | \
             V II7 V V | V7 V7 I I | V7 V7 vi vi | V III ii ii | ii #iv°7 V V7 | \
             I #ivø7 I V7 | I I I I7 | IV vii°7 I I",
        ),
    ],
    form: &["Intro", "Verse", "Verse"],
    drums_from: 0,
    about: "Schubert's prayer to the Virgin, from his Walter Scott songs, in B flat as he \
            wrote it. The piano rolls its chords in groups of six from the first bar to the \
            last; the sixteenth-note arpeggio stands in for them. The voice part is full of \
            grace notes and little triplet turns, which are evened out to sixteenths here, \
            and almost every bar has a chromatic chord in it: a diminished seventh on the \
            fourth beat of the second bar, A7 and F sharp diminished in the fourth.",
};

// Source: https://www.mutopiaproject.org/ftp/RachmaninoffS/O3/rach-prelude-op3-no2/rach-prelude-op3-no2.ly
pub(super) const PRELUDE_IN_C_SHARP_MINOR: Song = Song {
    name: "Prelude in C-sharp minor",
    origin: "Sergei Rachmaninoff, Op. 3 No. 2, 1892",
    feel: "Piano",
    genre: Genre::Classical,
    tonic: CSharp,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 46.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "Epic toms",
    style: "Epic minor",
    parts: &[
        Part::new("Pickup", "-:8 A4:4 G#4:4", "").with_left("-:8 A2:4 G#2:4"),
        // The right hand: its chords on the off-beat eighths, with the
        // motto A, G sharp, C sharp between them. The left hand has the
        // motto in octaves; its lower chord notes are left out.
        Part::new(
            "Lento",
            "C#4:16 | C#4:2 C#5:2 E5:2 D#5:2 A4:2 D5:2 G#4:2 B#4:2 | \
             C#4:2 C#5:2 E5:2 D#5:2 A4:2 D5:2 G#4:2 B#4:2 | \
             C#4:2 C#5:2 G#5:2 F#5:2 C5:2 E5:2 B4:2 D#5:2 | \
             E4:2 E5:2 B5:2 A5:2 E5:2 G#5:2 D#5:2 G5:2 | \
             G#4:2 G#5:2 A5:2 F#5:2 G#5:2 E5:2 F#5:2 D#5:2 | \
             E5:2 E5:2 F#5:2 D#5:2 E5:2 C#5:2 D#5:2 B#4:2 | \
             C#5:2 C#5:2 E5:2 D#5:2 A4:2 D5:2 G#4:2 B#4:2 | \
             C#4:2 C#5:2 G#5:2 F#5:2 A4:2 E5:2 G#4:2 B#4:2 | \
             C#4:2 C#5:2 D5:2 B4:2 C#5:2 A4:2 B4:2 G#4:2 | \
             A5:2 A5:2 C#6:2 B5:2 G4:2 A5:2 G#4:2 G#5:2 | \
             C#4:2 C#5:2 E5:2 D#5:2 A4:2 D5:2 G#4:2 B#4:2 | \
             C#4:2 C#5:2 E5:2 D#5:2 A4:2 D5:2 G#4:2 B#4:2",
            "",
        )
        .with_left(
            "C#2:16 | _:8 A2:4 G#2:4 | C#2:8 A2:4 G#2:4 | C#2:8 C3:4 B2:4 | E2:8 E3:4 D#3:4 | \
             G#2:16 | _:2 -:14 | -:8 A2:4 G#2:4 | C#2:8 A2:4 G#2:4 | C#2:16 | \
             _:2 -:6 G2:4 G#2:4 | C#2:8 A2:4 G#2:4 | C#2:8 A2:4 G#2:4",
        ),
    ],
    form: &["Pickup", "Lento"],
    drums_from: 1,
    about: "Three notes, A, G sharp, C sharp, tolling in octaves in the left hand (the Keys \
            channel) under heavy chords: the first fourteen bars, marked Lento. The chords \
            slide by semitones over a C sharp that never moves, so there is no chart; the \
            piano part is written out, with the left hand's octaves reduced to one note. \
            The famous Agitato comes after this.",
};

// Source: https://www.mutopiaproject.org/ftp/Mendelssohn-BartholdyF/O61/Sommernachtstraum-wedding-march/Sommernachtstraum-wedding-march.ly
// (Théodore Dubois's organ transcription, in C)
pub(super) const WEDDING_MARCH: Song = Song {
    name: "Wedding March",
    origin: "Felix Mendelssohn, A Midsummer Night's Dream, 1842",
    feel: "March",
    genre: Genre::Classical,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Pulse,
    bass: Bass::Quarters,
    rhythm: "March",
    style: "Classical cadence",
    parts: &[
        Part::new(
            "Fanfare",
            "-:12 C5:1 C5:1 C5:2 | C5:8 -:4 C5:1 C5:1 C5:2 | C5:8 -:4 C5:1 C5:1 C5:2 | \
             E5:4 E5:1 E5:1 E5:2 E5:4 E5:1 E5:1 E5:2 | G5:4 G5:1 G5:1 G5:2 G5:4 -:4",
            "I | I | I | I | I",
        ),
        Part::new(
            "March",
            "C6:8 B5:6 B5:2 | A5:4 G5:4 F5:4 D5:4 | C5:8 D5:4 G4:3 D5:1 | E5:4 -:4 -:2 C5:2 E5:2 G5:2 | \
             C6:8 B5:6 B5:2 | A5:4 G5:4 F5:4 D5:4 | C5:8 E5:4 D5:3 E5:1 | D5:8 C5:4 -:4",
            "vi6 VII7 | VII7 iii ii ii | I V7 | I | vi6 VII7 | VII7 iii ii ii | I V7 | V7 I",
        ),
        Part::new(
            "Trio",
            "C5:8 C5:6 C5:2 | E5:4 D5:4 B4:4 G4:4 | G4:6 C5:2 C5:6 E5:2 | E5:4 D5:4 B4:4 G4:4 | \
             G4:6 E5:2 E5:6 G5:2 | G5:8 F5:4 E5:4 | D5:4 C#5:3 E5:1 D5:4 A4:3 C5:1 | B4:4 G5:4 A5:4 B5:4",
            "I | I V7 | I | I V7 | I | vi7 vi7 ii vi | ii VI7 II II | V",
        ),
    ],
    form: &["Fanfare", "March", "Trio", "March"],
    drums_from: 0,
    about: "The trumpets' fanfare on C, E and G, then the march, whose first chord already has \
            an F sharp in it (A minor with a sixth) and whose second is B major, a key far \
            from C: Mendelssohn's bright wrong notes, which resolve on the C major trill. \
            From the organ transcription on Mutopia, with the fanfare an octave up and its \
            triplets written as two sixteenths and an eighth.",
};

// Source: https://www.mutopiaproject.org/ftp/HandelGF/hallelujah/hallelujah-lys.zip
// (soprano.ly and bass.ly of the full score)
pub(super) const HALLELUJAH_CHORUS: Song = Song {
    name: "Hallelujah Chorus",
    origin: "George Frideric Handel, Messiah, 1741",
    feel: "Choral",
    genre: Genre::Classical,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    style: "Hymn",
    parts: &[
        Part::new(
            "Hallelujah",
            "D5:6 A4:2 B4:2 A4:2 -:4 | D5:6 A4:2 B4:2 A4:2 -:2 D5:1 D5:1 | \
             D5:2 D5:2 -:2 D5:1 D5:1 D5:2 D5:2 -:2 D5:2 | C#5:2 D5:4 C#5:2 D5:4 -:4 | \
             E5:6 A4:2 F#5:2 E5:2 -:4 | E5:6 A4:2 F#5:2 E5:2 -:2 E5:1 E5:1 | \
             F#5:2 E5:2 -:2 E5:1 E5:1 F#5:2 E5:2 -:2 E5:2 | F#5:2 E5:2 D5:4 C#5:4 -:4",
            "I I IV I | I I IV I | IV I IV I | V V I I | V V I V | V V I V | I V I V | I V vi V",
        ),
        Part::new(
            "For the Lord God",
            "A4:8 B4:4 C#5:4 | D5:2 D4:2 D5:6 D5:2 C#5:4 | B4:8 A4:4 -:2 E5:1 E5:1 | \
             D5:2 C#5:2 -:2 E5:1 E5:1 D5:2 C#5:2 -:2 E5:1 E5:1 | \
             F#5:2 E5:2 -:2 E5:1 E5:1 F#5:2 E5:2 -:4 | -:16 | -:16 | -:14 D5:1 D5:1 | \
             D5:2 D5:2 -:2 D5:1 D5:1 D5:2 D5:2 -:2 D5:1 D5:1 | D5:2 D5:2 -:2 D5:1 D5:1 D5:2 D5:2 -:4",
            "V | I I I V | vi V | I V I V | I V I V | I | IV IV IV I | ii I | IV I IV I | IV I IV I",
        ),
        Part::new(
            "The Kingdom",
            "D5:8 E5:4 F#5:4 | G5:2 G4:2 G5:6 G5:2 F#5:4 | E5:8 D5:4 -:2 D5:1 D5:1 | \
             C#5:2 A4:2 -:2 C#5:1 C#5:1 D5:2 D5:1 D5:1 E5:2 E5:2 | \
             A4:4 -:2 D5:1 C#5:1 B4:2 B4:2 -:2 E5:2 | \
             D5:2 C#5:2 D5:2 E5:1 D5:1 C#5:2 C#5:1 C#5:1 E5:2 C#5:2 | \
             -:2 A4:1 A4:1 D5:2 A4:2 -:2 C#5:1 C#5:1 E5:2 C#5:2 | \
             -:2 A4:1 A4:1 F#5:2 D5:2 -:2 C#5:1 C#5:1 D5:2 A4:2 | \
             -:2 B4:1 B4:1 E5:2 E5:2 -:2 C#5:1 C#5:1 D5:2 D5:1 D5:1 | \
             D5:4 C#5:4 D5:4 D5:4 | D5:4 D5:4 D5:8",
            "I | IV IV IV I | V I | V V vi V | I I I V | vi vi V V | I I V V | I I V I | \
             IV ii V vi | IV V I I | IV V I I",
        ),
    ],
    form: &["Hallelujah", "For the Lord God", "The Kingdom"],
    drums_from: 0,
    about: "The chorus shouts \"Hallelujah\" on D, then on A, the dominant, always in the same \
            rhythm; \"For the Lord God omnipotent reigneth\" is sung in unison, and the \
            hallelujahs answer it. This is the soprano line of the full score; the sopranos \
            rest for two bars in the middle while the basses take the tune.",
};

// Source: https://www.mutopiaproject.org/ftp/MussorgskyM/pictures-at-an-exhibition/pictures-at-an-exhibition-lys.zip
// (great-gate-allegro.ily, Pavel Lamm's edition)
pub(super) const GREAT_GATE_OF_KIEV: Song = Song {
    name: "The Great Gate of Kiev",
    origin: "Modest Mussorgsky, Pictures at an Exhibition, 1874",
    feel: "Epic",
    genre: Genre::Classical,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "Epic toms",
    style: "Hymn",
    parts: &[
        Part::new(
            "Theme",
            "Eb5:16 | F5:16 | G5:8 Eb5:4 G5:4 | F5:8 Bb4:8 | G4:4 Bb4:4 F4:4 Eb4:4 | D4:8 Bb3:8 | \
             G4:4 Bb4:4 F4:4 Eb4:4 | D4:8 Bb3:8",
            "I | V | I vi | V | I I V vi | V I | I I V vi | V I",
        ),
        Part::new(
            "Theme end",
            "Eb5:16 | F5:16 | G5:8 Eb5:4 G5:4 | F5:8 Bb4:8 | Eb5:4 Eb5:4 D5:4 Bb4:4 | \
             C5:4 C5:4 Ab4:4 C5:4 | Bb4:8 Ab4:4 G4:4",
            "I | V | I vi | V | vi vi V iii | IV IV ii IV | I I ii I",
        ),
        Part::new(
            "Bridge",
            "F4:16 | Bb4:8 C5:8 | Eb5:4 Eb5:4 D5:4 C5:4 | Bb4:8 Ab4:4 G4:4 | F4:16 | Bb4:16",
            "ii7 | V vi | vi vi V IV | I I ii I | ii7 | iii",
        ),
    ],
    form: &["Theme", "Theme end", "Bridge", "Theme"],
    drums_from: 0,
    about: "The last picture: a hymn in E flat in whole notes and halves, marked alla breve, \
            which the orchestra plays in Ravel's version with the whole brass section. The \
            second half of the tune drops to the low B flat; the quiet bridge passes through \
            F minor and G minor before the gate opens again. From the Lamm edition on \
            Mutopia; the melody is the top of its chords.",
};

// Source: https://www.mutopiaproject.org/ftp/GriegE/O46/03-anitrasdance/03-anitrasdance-lys.zip
// (violiniI.ly and contrabassi.ly of the string parts)
pub(super) const ANITRAS_DANCE: Song = Song {
    name: "Anitra's Dance",
    origin: "Edvard Grieg, Peer Gynt, 1875",
    feel: "Waltz",
    genre: Genre::Classical,
    tonic: A,
    mode: NaturalMinor,
    time: THREE_FOUR,
    tempo: 152.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    style: "La folia",
    parts: &[
        Part::new("Pickup", "-:8 E4:4", "i"),
        Part::new(
            "A",
            "A4:2 G#4:2 A4:2 B4:2 C5:2 D5:2 | E5:2 A5:2 E5:4 D5:1 E5:1 D5:1 E5:1 | \
             C5:2 A5:2 C5:4 B4:1 C5:1 B4:1 C5:1 | A4:8 -:4 | \
             A4:2 C5:2 A4:4 G4:1 A4:1 G4:1 A4:1 | F#4:2 C5:2 F#4:8 | \
             F4:2 C5:2 F4:4 E4:1 F4:1 E4:1 F4:1 | D#4:2 B4:2 D#4:8 | \
             B4:2 A#4:2 F#4:2 B4:2 A4:2 F4:2 | B4:2 G#4:2 E4:2 B4:2 G4:2 D#4:2 | \
             B4:2 F#4:2 D4:2 B4:2 F4:2 Db4:2 | B4:2 E4:2 C4:2 B4:2 D#4:2 B3:2 | \
             E4:4 -:8 | -:12 | E4:2 F#4:2 G4:2 A4:2 B4:4 | E5:4 -:4 E4:4",
            "i | i | i | i | i | vi° | bVI | II | II7 | II7 | II7 | II7 | V | V | V | V",
        ),
        Part::new(
            "A end",
            "A4:2 G#4:2 A4:2 B4:2 C5:2 D5:2 | E5:2 A5:2 E5:4 D5:1 E5:1 D5:1 E5:1 | \
             C5:2 A5:2 C5:4 B4:1 C5:1 B4:1 C5:1 | A4:8 -:4 | \
             A4:2 C5:2 A4:4 G4:1 A4:1 G4:1 A4:1 | F#4:2 C5:2 F#4:8 | \
             F4:2 C5:2 F4:4 E4:1 F4:1 E4:1 F4:1 | D#4:2 B4:2 D#4:8 | \
             B4:2 A#4:2 F#4:2 B4:2 A4:2 F4:2 | B4:2 G#4:2 E4:2 B4:2 G4:2 D#4:2 | \
             B4:2 F#4:2 D4:2 B4:2 F4:2 Db4:2 | B4:2 E4:2 C4:2 B4:2 D#4:2 B3:2 | \
             E4:4 -:8 | -:12 | E4:2 F#4:2 G4:2 A4:2 B4:4 | E5:4 -:8",
            "i | i | i | i | i | vi° | bVI | II | II7 | II7 | II7 | II7 | V | V | V | V",
        ),
    ],
    form: &["Pickup", "A", "A end"],
    drums_from: 1,
    about: "Anitra dances for Peer Gynt in a quick 3/4 over a plucked bass. The tune circles \
            A minor for four bars, then the same figure slides down by semitones, F sharp, \
            F, D sharp, and sits on B7, the dominant of the dominant, while the violins \
            creep down chromatically under it; E leads it home. The first strain, from the \
            string parts on Mutopia.",
};

// Source: https://www.mutopiaproject.org/ftp/MozartWA/KV626/dies_irae/dies_irae-lys.zip
// (soprano.ily and basso.ily of the full score)
pub(super) const DIES_IRAE_MOZART: Song = Song {
    name: "Dies irae (Mozart)",
    origin: "Wolfgang Amadeus Mozart, Requiem, 1791",
    feel: "Epic",
    genre: Genre::Classical,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 132.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Epic toms",
    style: "Epic minor",
    parts: &[
        Part::new(
            "Dies irae",
            "D5:8 A4:8 | E5:4 A4:4 -:8 | E5:8 A4:8 | F5:4 D5:4 -:4 D5:2 D5:2 | \
             C#5:4 D5:4 -:4 D5:2 D5:2 | E5:4 F5:4 D5:6 D5:2 | G5:4 F5:4 E5:4 D5:4 | \
             D5:4 C#5:4 -:8 | -:16",
            "i | i | V7 | i | V7 i | V7 i | iv i bVII i | V | V",
        ),
        Part::new(
            "Quantus tremor",
            "F5:12 C5:4 | A4:4 F4:4 A4:4 C5:4 | Eb5:8 D5:8 | G5:12 D5:4 | B4:4 G4:4 B4:4 D5:4 | \
             F5:8 E5:8 | E5:8 E5:8 | E5:8 D5:8 | D5:4 C5:4 B4:8 | A4:4 -:12",
            "bIII | bIII | iii°7 | iv | iv | #iv°7 | v | bVII i | II7 | v",
        ),
    ],
    form: &["Dies irae", "Quantus tremor", "Dies irae"],
    drums_from: 0,
    about: "The Day of Wrath as Mozart set it: D minor in bare octaves, D and A, the chorus \
            answering the strings' hammering. \"Quantus tremor\" climbs by step, F major, \
            then G minor, then A minor, each new key reached through a diminished seventh \
            chord. The soprano line, from the Mutopia full score.",
};
