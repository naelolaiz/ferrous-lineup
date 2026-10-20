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
    band: Band::new(gm::VIOLIN, gm::HARPSICHORD, gm::CELLO),
    style: "Hymn",
    parts: &[
        Part::new("Ground", "-:16 | -:16", CANON_GROUND),
        // Each violin starts two bars after the one before.
        Part::new("Entries", CANON_TUNE, CANON_CHORDS).with_voices(&[(1, 0), (2, 2), (3, 4)]),
        Part::new("Canon", CANON_TUNE, CANON_CHORDS).with_voices(&[
            (1, 0),
            (2, -4),
            (2, 2),
            (3, -2),
            (3, 4),
        ]),
        Part::new("Last entries", CANON_TUNE, CANON_CHORDS).with_voices(&[(2, -4), (3, -2)]),
    ],
    form: &["Ground", "Entries", "Canon", "Canon", "Last entries"],
    drums_from: 0,
    about: "Three violins play the same line, each two bars behind the last, over a bass of \
            eight notes that repeats from start to end: a ground bass.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/BeethovenLv/WoO59/fur_Elise_WoO59/fur_Elise_WoO59.ly
// (Mutopia 931, from the Breitkopf & Härtel edition of 1888)
pub(super) const FUR_ELISE: Song = Song {
    name: "Für Elise",
    origin: "Ludwig van Beethoven, 1810",
    feel: "Piano",
    genre: Genre::Classical,
    tonic: A,
    mode: HarmonicMinor,
    // Each bar of 3/8 is a bar of 3/4 here, every note twice as long, so
    // the thirty-second notes fit: "Poco moto" at 72 quarter notes a
    // minute is 144 in the doubled values.
    time: THREE_FOUR,
    tempo: 144.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: PIANO,
    style: "",
    parts: &[
        Part::new("Pickup", "-:8 E5:2 D#5", "E").with_left("-:12"),
        Part::new(
            "Theme",
            "E5:2 D#5 E5 B4 D5 C5 | A4:4 -:2 C4 E4 A4 | B4:4 -:2 E4 G#4 B4 | \
             C5:4 -:2 E4 E5 D#5 | E5 D#5 E5 B4 D5 C5 | A4:4 -:2 C4 E4 A4 | B4:4 -:2 E4 C5 B4",
            "E | Am | E | Am | E | Am | E",
        )
        .with_left(
            "-:12 | A2:2 E3 A3 -:6 | E2:2 E3 G#3 -:6 | A2:2 E3 A3 -:6 | -:12 | \
                    A2:2 E3 A3 -:6 | E2:2 E3 G#3 -:6",
        ),
        Part::new("First ending", "A4:8 E5:2 D#5", "Am").with_left("A2:2 E3 A3 -:6"),
        Part::new("Second ending", "A4:4 -:2 B4 C5 D5", "Am").with_left("A2:2 E3 A3 -:6"),
        Part::new(
            "Middle",
            "E5:6 G4:2 F5 E5 | D5:6 F4:2 E5 D5 | C5:6 E4:2 D5 C5 | B4:4 -:2 E4 E5 - | \
             - E5 E6 -:4 D#5:2 | E5:4 -:2 D#5 E5 D#5",
            "C | G7 | Am | E | E | E",
        )
        .with_left(
            "C3:2 G3 C4 -:6 | G2:2 G3 B3 -:6 | A2:2 E3 A3 -:6 | E2:2 E3 E4 -:4 E4:2 | \
                    E5 -:4 D#5:2 E5 - | - D#5 E5 -:6",
        ),
        Part::new(
            "Into the episode",
            "A4:4 -:2 E4+C5 F4+C5 E4+G4+C5",
            "Am Am Am C7/Bb F/A C7/G",
        )
        .with_left("A2:2 E3 A3 Bb3+C4 A3+C4 G3+Bb3+C4"),
        Part::new(
            "Episode in F",
            "C5:8 F5:3 E5:1 | E5:4 D5 Bb5:3 A5:1 | A5:2 G5 F5 E5 D5 C5 | \
             Bb4:4 A4 A4:1 G4 A4 Bb4 | C5:8 D5:2 D#5 | E5:6 E5:2 F5 A4 | C5:8 D5:3 B4:1",
            "F | Bb/F | C7/F | F | F | Am/E Am/E Dm | C/G C/G G7",
        )
        .with_left(
            "F3:2 A3 C4 A3 C4 A3 | F3 Bb3 D4 Bb3 D4 Bb3 | \
                    F3 E4 F3+G3+Bb3 E4 F3+G3+Bb3 E4 | F3 A3 C4 A3 C4 A3 | F3 A3 C4 A3 C4 A3 | \
                    E3 A3 C4 A3 D3+D4 F3 | G3 E4 G3 E4 G3 F4",
        )
        .in_key(F, Major),
        Part::new(
            "Episode in C",
            "C5:1 G5 G4 G5 A4 G5 B4 G5 C5 G5 D5 G5 | E5 G5 C6 B5 A5 G5 F5 E5 D5 G5 F5 D5 | \
             C5 G5 G4 G5 A4 G5 B4 G5 C5 G5 D5 G5 | E5 G5 C6 B5 A5 G5 F5 E5 D5 G5 F5 D5",
            "C C C G7/F C/E G7/D | C F G7 | C C C G7/F C/E G7/D | C F G7",
        )
        .with_left(
            "C4+E4:4 -:2 F4+G4 E4+G4 D4+F4+G4 | C4+E4+G4:4 F3+A3 G3+B3 | \
                    C4 -:2 F4+G4 E4+G4 D4+F4+G4 | C4+E4+G4:4 F3+A3 G3+B3",
        )
        .in_key(C, Major),
        Part::new(
            "Back to A minor",
            "E5:1 F5 E5 D#5 E5 B4 E5 D#5 E5 B4 E5 D#5 | E5:6 B4:2 E5 D#5 | E5:6 B4:2 E5 D#5 | \
             E5 D#5 E5 D#5 E5 D#5",
            "E/G# | E | E | E",
        )
        .with_left("G#3+B3:4 -:8 | -:12 | - | -"),
        Part::new(
            "Middle again",
            "E5:6 G4:2 F5 E5 | D5:6 F4:2 E5 D5 | C5:6 E4:2 D5 C5 | B4:4 -:2 E4 E5 - | \
             - E5 E6 -:4 D#5:2 | E5 -:4 D#5:2 E5 D#5",
            "C | G7 | Am | E | E | E",
        )
        .with_left(
            "C3:2 G3 C4 -:6 | G2:2 G3 B3 -:6 | A2:2 E3 A3 -:6 | E2:2 E3 E4 -:4 E4:2 | \
                    E5 -:4 D#5:2 E5 - | - D#5 E5 -:6",
        ),
        Part::new(
            "Pedal",
            "A4:4 -:8 | E4+G4+Bb4+C#5:12 | F4+A4+D5:8 C#5+E5:2 D5+F5 | \
             G#4+D5+F5:8 G#4+D5+F5:4 | A4+C5+E5:12 | F4+D5:8 E4+C5:2 D4+B4 | \
             C4+F#4+A4:8 C4+A4:4 | C4+A4 E4+C5 D4+B4 | C4+A4:12 | E4+G4+Bb4+C#5 | \
             F4+A4+D5:8 C#5+E5:2 D5+F5 | D5+F5:8 D5+F5:4 | D5+F5:12 | \
             G4+Eb5:8 F4+D5:2 Eb4+C5 | D4+F4+Bb4:8 D4+F4+A4:4 | D4+F4+G#4:8 D4+F4+G#4:4 | \
             C4+E4+A4:8 -:4 | E4+B4 -:8",
            "Am | A7b9 | Dm/A | G#dim7/A | Am | Dm | D#dim7 | Am/E Am/E E7 | Am | A7b9 | Dm/A | \
             Dm/A | Bb | Eb/Bb Eb/Bb Bb | Bb Bb Bbmaj7 | G#dim7/B | Am/C | E",
        )
        .with_left(
            "A2:2 A2 A2 A2 A2 A2 | A2 A2 A2 A2 A2 A2 | A2 A2 A2 A2 A2 A2 | \
                    A2 A2 A2 A2 A2 A2 | A2 A2 A2 A2 A2 A2 | \
                    D2+A2 D2+A2 D2+A2 D2+A2 D2+A2 D2+A2 | \
                    D#2+A2 D#2+A2 D#2+A2 D#2+A2 D#2+A2 D#2+A2 | \
                    E2+A2 E2+A2 E2+A2 E2+A2 E2+G#2 E2+G#2 | A1+A2 A2 A2 A2 A2 A2 | \
                    A2 A2 A2 A2 A2 A2 | A2 A2 A2 A2 A2 A2 | A2 A2 A2 A2 A2 A2 | \
                    Bb2 Bb2 Bb2 Bb2 Bb2 Bb2 | Bb2 Bb2 Bb2 Bb2 Bb2 Bb2 | \
                    Bb2 Bb2 Bb2 Bb2 Bb2 Bb2 | B2 B2 B2 B2 B2 B2 | C3:8 -:4 | E3+G#3 -:8",
        ),
        Part::new(
            "Run",
            "A3:2 C4 E4 A4 C5 E5 D5 C5 B4 | A4 C5 E5 A5 C6 E6 D6 C6 B5 | \
             A5 C6 E6 A6 C7 E7 D7 C7 B6 | Bb6 A6 G#6 G6 F#6 F6 E6 D#6 D6 | \
             C#6 C6 B5 Bb5 A5 G#5 G5 F#5 F5",
            "Am | Am | Am | Am | Am",
        )
        .with_left(
            "A1:6 - A3+C4+E4 | A3+C4+E4 - A3+C4+E4 | A3+C4+E4 - A3+C4+E4 | \
                    A3+C4+E4 -:12 | -:18",
        )
        .in_time(NINE_EIGHT)
        .with_tempo(&[tempo(0, 216), tempo(90, 144)]),
        Part::new("Last bar", "A4:4 -:8", "Am").with_left("A1+A2:4 -:8"),
    ],
    form: &[
        "Pickup",
        "Theme",
        "First ending",
        "Theme",
        "Second ending",
        "Middle",
        "Theme",
        "Second ending",
        "Middle",
        "Theme",
        "Into the episode",
        "Episode in F",
        "Episode in C",
        "Back to A minor",
        "Theme",
        "Second ending",
        "Middle again",
        "Theme",
        "Pedal",
        "Run",
        "Theme",
        "Second ending",
        "Middle again",
        "Theme",
        "Last bar",
    ],
    drums_from: 0,
    about: "The whole rondo, with its repeats as Beethoven wrote them. The theme rocks \
            between E and D sharp, the leading note of A minor, over broken chords in the \
            left hand; its middle phrase passes through C major and G7. The first episode \
            is in F major and turns to C major in runs of thirty-second notes, then E and \
            D sharp lead back to the theme. The second episode is stormy, over an A drummed \
            in the bass: A7 with a flat ninth, D minor, a diminished seventh, then B flat \
            major, the Neapolitan chord of A minor, before an arpeggio climbs three octaves \
            and falls back down the chromatic scale to the theme. Each bar of Beethoven's \
            3/8 is written as a bar of 3/4 with the note values doubled; the five bars of \
            triplet sixteenths are bars of 9/8 that last as long. The three grace notes are \
            left out.",
};

/// Grieg keeps the bass on the key note while the theme climbs; only the
/// fourth bar lands on the chord of the third.
const MOUNTAIN_CHORDS: &str = "i | i | i | bIII | i | i | i | bIII";

/// The theme, on B, and the same a fifth higher, on F sharp major with
/// its sixth, D, left flat: the augmented chord in the even bars.
const MOUNTAIN_THEME: &str = "B4:2 C#5 D5 E5 F#5 D5 F#5:4 | F5:2 C#5 F5:4 E5:2 C5 E5:4 | \
             B4:2 C#5 D5 E5 F#5 D5 F#5 B5 | A5 F#5 D5 F#5 A5:8 | \
             B4:2 C#5 D5 E5 F#5 D5 F#5:4 | F5:2 C#5 F5:4 E5:2 C5 E5:4 | \
             B4:2 C#5 D5 E5 F#5 D5 F#5 B5 | A5 F#5 D5 F#5 A5:8";
const MOUNTAIN_HIGHER: &str = "F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:2 A#5 C#6:4 | \
             F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:8 | \
             F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:2 A#5 C#6:4 | \
             F#5:2 G#5 A#5 B5 C#6 A#5 C#6:4 | D6:2 A#5 D6:4 C#6:8";
const MOUNTAIN_HIGHER_CHORDS: &str = "I | I+ | I | I+ | I | I+ | I | I+";

// Tempo marks from Grieg's own piano version: Source:
// https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/GriegE/O46/Dans_l_antre_du_roi_de_la_montagne/Dans_l_antre_du_roi_de_la_montagne.ly
// ("Alla marcia e molto marcato ♩=138", "Più vivo ♩=154" at bar 50,
// "sempre stretto sin al fine" at bar 58).
pub(super) const MOUNTAIN_KING: Song = Song {
    name: "In the Hall of the Mountain King",
    origin: "Edvard Grieg, Peer Gynt, 1875",
    feel: "Epic",
    genre: Genre::Classical,
    tonic: B,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 138.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Epic toms",
    band: Band::new(gm::BASSOON, gm::PIZZICATO_STRINGS, gm::PIZZICATO_STRINGS),
    style: "Epic minor",
    parts: &[
        Part::new("Theme", MOUNTAIN_THEME, MOUNTAIN_CHORDS),
        Part::new("Higher", MOUNTAIN_HIGHER, MOUNTAIN_HIGHER_CHORDS).in_key(FSharp, Major),
        // "Più vivo": faster at once, at Grieg's own metronome mark.
        Part::new("Più vivo", MOUNTAIN_THEME, MOUNTAIN_CHORDS).with_tempo(&[tempo(0, 154)]),
        // "Sempre stretto sin al fine": pressing on faster and faster to
        // the end. The score gives no number; each time round is about a
        // tenth faster than the last.
        Part::new("Stretto", MOUNTAIN_HIGHER, MOUNTAIN_HIGHER_CHORDS)
            .in_key(FSharp, Major)
            .with_tempo(&[glide(0, 165, 128)]),
        Part::new("Stretto theme", MOUNTAIN_THEME, MOUNTAIN_CHORDS)
            .with_tempo(&[glide(0, 177, 128)]),
    ],
    form: &[
        "Theme",
        "Higher",
        "Theme",
        "Theme",
        "Higher",
        "Theme",
        "Più vivo",
        "Stretto",
        "Stretto theme",
    ],
    drums_from: 0,
    about: "Trolls creeping up the B minor scale, over a bass that stays on B. Then the \
            whole theme moves up a fifth onto F sharp, major but with its sixth left flat, \
            as it does in Grieg's score, in the score's order. The trolls get more and more \
            excited: \"Più vivo\" jumps from 138 to 154 beats a minute, and then \"sempre \
            stretto\", pressing on, speeds up bit by bit to the end. Watch the tempo strip \
            at the top of the playlist.",
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
    band: Band::solo(gm::HARPSICHORD),
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
        // The first half of B is in D major: C sharp replaces C, and it
        // ends on a D major cadence. The second half is back in G.
        Part::new(
            "B in D",
            "B5:4 G5:2 A5 B5 G5 | A5:4 D5:2 E5 F#5 D5 | G5:4 E5:2 F#5 G5 D5 | C#5:4 B4:2 C#5 A4:4 | \
             A4:2 B4 C#5 D5 E5 F#5 | G5:4 F#5 E5 | F#5:4 A4 C#5 | D5:12",
            "IV | I | ii | V | V | IV I V | I | I",
        )
        .in_key(D, Major),
        Part::new(
            "B in G",
            "D5:4 G4:2 F#4 G4:4 | E5:4 G4:2 F#4 G4:4 | D5:4 C5 B4 | A4:2 G4 F#4 G4 A4:4 | \
             D4:2 E4 F#4 G4 A4 B4 | C5:4 B4 A4 | B4:2 D5 G4:4 F#4 | G4:12",
            "I | IV | I | V | V | IV I V | I V | I",
        ),
    ],
    form: &["A", "A", "B in D", "B in G", "B in D", "B in G"],
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
    band: Band::new(gm::VIOLIN, gm::STRING_ENSEMBLE, gm::CONTRABASS),
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
    band: Band::new(
        gm::VOICE_OOHS,
        gm::ACOUSTIC_GRAND_PIANO,
        gm::ACOUSTIC_GRAND_PIANO,
    ),
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
    band: Band::new(gm::FLUTE, gm::STRING_ENSEMBLE, gm::CELLO),
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
    band: Band::new(gm::ENGLISH_HORN, gm::STRING_ENSEMBLE, gm::CONTRABASS),
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
    band: CHOIR,
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
    band: PIANO,
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
    band: Band::new(gm::VOICE_OOHS, gm::PIZZICATO_STRINGS, gm::CONTRABASS),
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
    band: Band::new(gm::FRENCH_HORN, gm::STRING_ENSEMBLE, gm::CONTRABASS),
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
    band: Band::new(
        gm::VOICE_OOHS,
        gm::ACOUSTIC_GRAND_PIANO,
        gm::ACOUSTIC_GRAND_PIANO,
    ),
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
    band: PIANO,
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
    band: Band::solo(gm::CHURCH_ORGAN),
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
    band: Band::new(gm::CHOIR_AAHS, gm::STRING_ENSEMBLE, gm::CELLO),
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
    band: Band::new(gm::TRUMPET, gm::BRASS_SECTION, gm::TUBA),
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
    band: Band::new(gm::VIOLIN, gm::PIZZICATO_STRINGS, gm::PIZZICATO_STRINGS),
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
    band: Band::new(gm::CHOIR_AAHS, gm::STRING_ENSEMBLE, gm::CELLO),
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
            "F5:12 C5:4 | A4:4 F4:4 A4:4 C5:4 | Eb5:8 D5:8 | G5:12 D5:4 | B4:4 G4:4 B4:4 D5:4",
            "bIII | bIII | iii°7 | iv | iv",
        ),
        // G sharp diminished seventh leads to A minor, and the second
        // "Dies irae" stays there: the chorus sings it a fifth higher, to a
        // new soprano line.
        Part::new(
            "Quantus tremor to A minor",
            "F5:8 E5:8 | E5:8 E5:8 | E5:8 D5:8 | D5:4 C5:4 B4:8 | A4:4 -:12",
            "vii°7 | i | bIII iv | V7 | i",
        )
        .in_key(A, HarmonicMinor),
        Part::new(
            "Dies irae in A minor",
            "E5:8 E5:8 | E5:4 B4:4 -:8 | E5:8 E5:8 | E5:4 C5:4 -:4 A4:2 A4:2 | \
             F5:4 E5:4 -:4 A4:2 A4:2 | D5:4 C5:4 E5:6 E5:2 | D5:4 C5:4 Bb4:4 A4:4 | \
             A4:4 G#4:4 -:8 | -:16",
            "i | i | V | i | iv i | V7 i | iv i bII i | V | V",
        )
        .in_key(A, HarmonicMinor),
    ],
    form: &[
        "Dies irae",
        "Quantus tremor",
        "Quantus tremor to A minor",
        "Dies irae in A minor",
    ],
    drums_from: 0,
    about: "The Day of Wrath as Mozart set it: D minor in bare octaves, D and A, the chorus \
            answering the strings' hammering. \"Quantus tremor\" climbs by step, F major, \
            then G minor, then A minor, each new key reached through a diminished seventh \
            chord. Then \"Dies irae\" comes back in A minor, with a new soprano line (the \
            two bars the orchestra plays alone before it are left out). The soprano line, \
            from the Mutopia full score.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/BeethovenLv/O13/pathetique-2/pathetique-2.ly
// (Mutopia 295, from the Peters edition of 1910)
const PATHETIQUE_CHORDS: &str = "I V7 | I V7 | I V vi II7 | V | V7 | I VI7 | ii V7 | V7 I";

pub(super) const PATHETIQUE: Song = Song {
    name: "Pathétique Adagio",
    origin: "Ludwig van Beethoven, Piano Sonata No. 8, 1798",
    feel: "Classical",
    genre: Genre::Classical,
    tonic: GSharp,
    mode: Major,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Alberti,
    bass: Bass::Held,
    rhythm: "",
    band: PIANO,
    style: "Classical cadence",
    parts: &[
        Part::new(
            "Theme",
            "C4:8 Bb3:8 | Eb4:12 Db4:4 | C4:4 Eb4 Ab4 Bb4 | Eb4:12 E4:4 | \
             F4:8 Bb3:6 C4:1 Db4:1 | Eb4:8 A3:8 | Db4:8 C4:2 Bb3 Ab3 G3 | Bb3:8 Ab3:4 -:4",
            PATHETIQUE_CHORDS,
        ),
        Part::new(
            "Theme higher",
            "C5:8 Bb4:8 | Eb5:12 Db5:4 | C5:4 Eb5 Ab5 Bb5 | Eb5:12 E5:4 | \
             F5:8 Bb4:6 C5:1 Db5:1 | Eb5:8 A4:8 | Db5:8 C5:2 Bb4 Ab4 G4 | Bb4:8 Ab4:4 -:4",
            PATHETIQUE_CHORDS,
        ),
    ],
    form: &["Theme", "Theme higher"],
    drums_from: 0,
    about: "The Adagio cantabile in A flat: a song in the middle of the piano, sung first in \
            the tenor and then an octave higher, over an inner voice that murmurs in \
            sixteenths (the Alberti comping here). Nearly every other beat is the dominant, \
            E flat seven, until the F7 and B flat minor of the last bars lead home. Each 2/4 \
            bar is written as one bar of 4/4 with the note values doubled, so the two \
            thirty-second notes in the fifth bar fit.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/BeethovenLv/O27/moonlight/moonlight-lys/moonlight1-a4.ly
// (Mutopia 276)
pub(super) const MOONLIGHT_SONATA: Song = Song {
    name: "Moonlight Sonata",
    origin: "Ludwig van Beethoven, Piano Sonata No. 14, 1801",
    feel: "Piano",
    genre: Genre::Classical,
    tonic: CSharp,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 56.0,
    comping: Comping::ArpUp,
    bass: Bass::Held,
    rhythm: "",
    band: PIANO,
    style: "Neapolitan",
    parts: &[
        Part::new(
            "Intro",
            "-:16 | -:16 | -:16 | -:16 | -:12 G#4:3 G#4:1",
            "i | i7 | bVI bII | V7 i Vsus4 V7 | i",
        ),
        Part::new("Theme", "G#4:12 G#4:3 G#4:1 | G#4:8 A4:8", "V7 | i iv"),
        Part::new("E major", "G#4:8 F#4:4 B4:4 | E4:4 -:12", "I V7 | I").in_key(E, Major),
        Part::new(
            "B minor",
            "-:12 G4:3 G4:1 | G4:12 G4:3 G4:1 | G4:12 F#4:4 | F#4:8 G4:4 E4:4 | F#4:8 F#4:8",
            "iv | bVI7 | bII iv viiø7 V7 | i i iv6 iv6 | i V",
        )
        .in_key(B, HarmonicMinor),
        Part::new(
            "E minor",
            "B3:4 -:8 B4:4 | C5:12 A#4:4 | B4:12 B4:4 | C5:12 A#4:4 | B4:8 B4:8",
            "v V | i | V | i | V iii°7",
        )
        .in_key(E, HarmonicMinor),
        Part::new(
            "F sharp minor",
            "B4:8 A4:8 | G4:8 F#4:8 | C#4:8 C#4:4 C#4:4 | F#3:4 -:12",
            "V7 i | bII #iv°7 | i i Vsus4 V | i",
        )
        .in_key(FSharp, HarmonicMinor),
    ],
    form: &[
        "Intro",
        "Theme",
        "E major",
        "B minor",
        "E minor",
        "F sharp minor",
    ],
    drums_from: 0,
    about: "The first 23 bars of the Adagio sostenuto, in C sharp minor as Beethoven wrote it. \
            His right hand plays triplets under the tune from start to end; they do not fit \
            in sixteenths, so the comping climbs the chords in eighth notes instead. In the \
            third bar comes D major, the Neapolitan chord, and later C major plays the same \
            part in B minor. The tune passes through E major, B minor and E minor, over a \
            long B in the bass, and stops in F sharp minor.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/BachJS/BWV1068/bach-air/bach-air-lys/bach-air-notes.ly
// (Mutopia 242, from the Bach-Gesellschaft edition: all four parts)
const AIR_ENDING: &str = "C#5:2 B4 B4:1 C#5 D5:6 C#5:2 B4 | A4:16";
const AIR_B_END: &str = "D5:12 F#5:2 E5 | E5:12 G5:2 F#5 | F#5:12 A5:2 G5 | G5:16 | \
             A4:10 C#5:2 E5 G5 | G5 E5 F#5:10 G5:1 A5 | D5:10 F#5:2 A5 C6 | B5:12 D5:4 | \
             C#5:2 E5 G5:8 B4:4 | A4 E5:2 F#5:1 G5:3 F#5:4 E5:2 | \
             D5:1 C#5 B4:4 C#5:2 D5:4 C#5:2 D5 | D5:16";
const AIR_B_END_LEFT: &str = "A4:4 B4:2 C5 B4 C#5 D5:4 | _ C#5:2 B4 C#5 D#5 E5:4 | \
             _ D#5:2 C#5 D#5 E5 F#5:4 | _:2 D#5 E5 B4 E4:8 | _:2 C#4 E4 A4 C#5:4 A4 | \
             _ C#5:2 D5 D4:8 | _:4 E4 F#4:8 | G4:16 | _:4 B4 E5:8 | _:2 D5 C#5 B4 A4:4 B4 | \
             A4:8 G4:2 F#4 G4:4 | F#4:16 & A3:4 D4:8 B3:4 | _ E4:8 C#4:4 | _ F#4:8 D#4:4 | \
             B3:10 B4:2 G4 E4 | A4:4 G4 F#4 E4 | D4:8 A4 | _:4 G4 A4:8 | D4:16 | \
             E4:2 B3 E4 G4 B4 A4 G4 F#4 | E4:4 A4:8 G4:4 | F#4:8 E4:4 A3 | \
             A3:16 & F#2:4 F#3 G3 G2 | G#2 G#3 A3 A2 | A#2 A#3 B3 B2 | E3 E4 D4 D3 | \
             C#3 C#4 A3 C#4 | - D3 C3 C4 | B3 B2 A2 A3 | G3 G2 F#2 F#3 | E3 E2 D2 D3 | \
             C#3 A2 D3 G3 | A3 G3 A3 A2 | D2:16";
const AIR_B_END_CHORDS: &str = "D/F# G | E7/G# A | F#7/A# B | Em Em/D | A7/C# A7 | D D7/C | \
             G/B D7/A | G G/F# | Em Em/D | A/C# A/C# D G | D/A A7/G A7sus4 A7 | D";

pub(super) const AIR_ON_THE_G_STRING: Song = Song {
    name: "Air on the G String",
    origin: "Johann Sebastian Bach, Orchestral Suite No. 3, about 1730",
    feel: "Baroque",
    genre: Genre::Classical,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::VIOLIN, gm::STRING_ENSEMBLE, gm::CONTRABASS),
    style: "Hymn",
    parts: &[
        Part::new(
            "A",
            "F#5:16 | _ | _:4 B5:2 G5 E5 D5 C#5 D5 | C#5:8 A4 | A5:16 | \
             _:2 F#5 C5 B4 E5 D#5 A5 G5 | G5:16 | _:2 E5 B4 A4 D5 C#5 G5 F#5 | \
             F#5:12 G#5:2 A5 | D5:4 D5:1 E5 F#5:4 E5:2 E5 D5",
            "D D/C# | Bm Bm/A | Gmaj7 E7/G# | A A7/G | F#m7b5 Am/E | B7/D# B7 | Em Em/D | \
             A7/C# A7 | D D D/C# A/C# | Bm7 E/G#",
        )
        .with_left(
            "D5:16 | _ | _:8 B4 | A4:16 | _:4 C5:2 B4 C5:4 A5:2 C5 | B4:4 -:12 | \
                    B4:4 E5:2 D5 E5 F#5 G5 E5 | A4:4 -:12 | A4:16 | \
                    _:4 G#4:2 A4 B4:4 G#4 & A4:16 | B4 | B3:8 E4 | E4:16 | _:4 D#4 D#4 E4 | \
                    F#4 -:12 | E4:4 B3 B3 E4 | E4 -:12 | D4 E4:4 | \
                    F#4 D4 B3 E4 & D3:4 D4 C#4 C#3 | B2 B3 A3 A2 | G2 G3 G#3 G#2 | \
                    A2 A3 G3 G2 | F#2 F#3 E3 E2 | D#2 D#3 B2 B3 | E2 E3 D3 D2 | \
                    C#2 C#3 A2 A3 | D3 D4 C#4 C#3 | B2 B3 G#3 E3",
        ),
        // The first half ends in A major, the dominant: G sharp in bar 9,
        // and a cadence in A.
        Part::new("First ending", AIR_ENDING, "A D E7sus4 E | A A7/E")
            .with_left(
                "A4:4 A4 A4 G#4 | E4:16 & E4:4 F#4 B3 E4 | C#4:16 & A3:4 D3 E3 E2 | \
                    A2:2 B2 C#3 D3 E3 G3 F#3 E3",
            )
            .in_key(A, Major),
        Part::new("Second ending", AIR_ENDING, "A D E7sus4 E | A")
            .with_left("A4:4 A4 A4 G#4 | E4:16 & E4:4 F#4 B3 E4 | C#4:16 & A3:4 D3 E3 E2 | A2:16")
            .in_key(A, Major),
        // The second half goes through three keys: E minor (C and D
        // sharp), B minor (A sharp, a cadence in bar 8) and A major (G
        // sharp, a cadence in bar 12), before "B end" turns back to D.
        Part::new(
            "B in E minor",
            "C#5:10 D5:1 C#5 B4 C#5 A4:2 | A5:12 C5:4 | B4 B5:6 A5:2 G5 F#5 | \
             G5:9 F#5:1 E5 D5 C#5:2 B4",
            "A A7/G | F#m7b5 F#m7b5/E | B/D# B/F# | Em Em/D",
        )
        .with_left(
            "E4:16 | _:4 D#4:2 E4 F#4:8 | _:2 G4 A4 F#4 D#4:4 B4 | B4:8 B3 & A4:16 | \
                    _:2 B4 C5:6 B4:2 A4 G4 | F#4:12 D#5:4 | E5:16 & A2:4 A3 G3 G2 | \
                    F#2 F#3 E3 E2 | D#2 D#3 F#3 B2 | E3 E4 D4 D3",
        )
        .in_key(E, HarmonicMinor),
        Part::new(
            "B in B minor",
            "A#4:2 B4 C#5:6 D5:2 E5:4 | _:2 F#5 G5:8 F#5:4 | E5:2 D5 C#5 B4 C#5 D5:1 E5 D5:4 | \
             B4:16",
            "F#7/C# Em/B | F#7/A# F#/C# | Bm G Em F# | Bm Bm/A",
        )
        .with_left(
            "E5:16 | _:2 D5 C#5 B4 A#4 B4 C#5:4 | B4 B4 B4 A#4 | \
                    F#4:16 & C#3:4 C#4 B3 B2 | A#2 B2 C#3 A#2 | B2 G3 E3 F#3 | \
                    B2 B3 A3 A2 & C#4:2 D4 E4 F#4 G4 F#4 G4 E4 | F#4:4 E4:2 D4 C#4:4 F#4 | \
                    F#4 E4:2 D4 G4:4 F#4:2 E4 | D4:16",
        )
        .in_key(B, HarmonicMinor),
        Part::new(
            "B in A major",
            "D5:10 F#5:2 E5 D5 | B5:12 A5:2 G#5 | E5 A5 A4:4 B4:6 C#5:1 D5 | C#5:6 B4:2 A4:8",
            "E7/G# D/F# | E E7/D | A/C# A/C# Bm7/D E | A A7/G",
        )
        .with_left(
            "E4:8 F#4 | B3:4 E4:2 F#4 G#4 A4 B4:4 | _ A4:8 G#4:4 | \
                    A4:16 & B3:4 B4 A4:2 G#4 A4:4 | G#4:6 -:2 E4:8 | _:4 E4 F#4 E4 | \
                    E4:6 D4:2 C#4 D4 E4 C#4 & G#2:4 G#3 F#3 F#2 | E2 E3 D3 D2 | \
                    C#2 C#3 D3 E3 | A2 A3 G3 G2",
        )
        .in_key(A, Major),
        Part::new("B end", AIR_B_END, AIR_B_END_CHORDS).with_left(AIR_B_END_LEFT),
        // The second time round is the end of the piece: the last chord
        // has the Air's only fermata.
        Part::new("Last end", AIR_B_END, AIR_B_END_CHORDS)
            .with_left(AIR_B_END_LEFT)
            .with_tempo(&[fermata(176, 16)]),
    ],
    form: &[
        "A",
        "First ending",
        "A",
        "Second ending",
        "B in E minor",
        "B in B minor",
        "B in A major",
        "B end",
        "B in E minor",
        "B in B minor",
        "B in A major",
        "Last end",
    ],
    drums_from: 0,
    about: "The first violin's long held notes, each one breaking into quick figures, over \
            the continuo's bass in steady octave leaps; the second violin and the viola \
            fill in the harmony between them, here with the continuo on the Keys channel as \
            a string section. The bass walks down the D major scale under the first held F \
            sharp, as in Pachelbel's Canon. The first half ends in A major; the second \
            passes through E minor, B minor and A major before it finds D again, and both \
            halves are repeated as Bach marks them. Each bar of Bach's is written as two \
            bars with the note values doubled, so his thirty-second notes fit in \
            sixteenths. The three grace notes are left out, and a note held from one part \
            into the next is played again where the next part starts.",
};

// Source: https://raw.githubusercontent.com/baobach/abc_search/main/database/abc/QmfHw5KYvE3na6LB6W77W5KqiSuhtgqPdkLmr4tuWFUowx.abc
// (an ABC arrangement for two violins and cello; SBurrell23/Sheets danse-macabre.abc, in
// A minor, has the same two themes)
pub(super) const DANSE_MACABRE: Song = Song {
    name: "Danse macabre",
    origin: "Camille Saint-Saëns, 1874",
    feel: "Waltz",
    genre: Genre::Classical,
    tonic: G,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 160.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    band: Band::new(gm::VIOLIN, gm::PIZZICATO_STRINGS, gm::CONTRABASS),
    style: "La folia",
    parts: &[
        Part::new(
            "Tuning",
            "Eb5:12 | A4:12 | Eb5:12 | A4:8 A4:4 | Eb5:2 Eb5:2 Eb5:4 A4:2 -:2 | \
             Eb5:2 Eb5:2 Eb5:4 A4:2 -:2 | Eb5:4 A4:4 Eb5:4 | A4:4 Eb5:4 -:4",
            "",
        ),
        Part::new(
            "Skeletons",
            "G4:4 G4:2 Bb4:2 G4:2 A4:2 | Bb4:4 A4:2 Bb4:2 G4:2 Bb4:2 | \
             A4:2 G4:2 G4:2 Bb4:2 G4:2 A4:2 | Bb4:4 A4:2 Bb4:2 G4:2 Bb4:2",
            "i | i | i | i",
        ),
        Part::new(
            "A step lower",
            "Ab4:4 Ab4:2 C5:2 Ab4:2 Bb4:2 | C5:4 Bb4:2 C5:2 Ab4:2 C5:2 | \
             Bb4:2 Ab4:2 Ab4:2 C5:2 Ab4:2 Bb4:2 | C5:4 Ab4:2 C5:2 Bb4:2 Ab4:2",
            "i7 | i7 | i7 | i7",
        )
        .in_key(F, NaturalMinor),
        Part::new("Turn", "G4:4 -:4 D5:4", "i"),
        Part::new(
            "Waltz",
            "G5:8 G5:4 | F5:8 F5:4 | E5:8 E5:4 | Eb5:8 Eb5:4 | D5:4 E5:4 G5:4 | F#5:4 E5:4 D5:4 | \
             E5:8 E5:4 | F#5:4 -:4 D5:4 | G5:8 G5:4 | F5:8 F5:4 | E5:8 E5:4 | Eb5:8 Eb5:4 | \
             D5:4 G5:4 Bb5:4 | A5:4 F#5:4 D5:4 | F#5:8 E5:4 | D5:4 -:8",
            "i | i7 | i6 | i | IV | ii | ii | V7b9 | i | i7 | IV | i | bVI | V | II | V",
        ),
    ],
    form: &[
        "Tuning",
        "Skeletons",
        "A step lower",
        "Skeletons",
        "A step lower",
        "Turn",
        "Waltz",
    ],
    drums_from: 1,
    about: "Death tunes his fiddle at midnight: the open A against an E flat, a tritone, \
            the devil's interval. Then the skeletons dance, first a quick tune that circles \
            G and B flat and is repeated a step lower, in F minor, then the violin's waltz, \
            which slides down by semitones, G, F, E, E flat, D, over a bass that stays \
            on G.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/main/wikifonia-dataset/original-abc/AntoninDvorakHumoresqueopus101no.abc
// (Wikifonia lead sheet in G; bar 3 there has reversed dots, d<e d<g e<d, so it follows
// https://raw.githubusercontent.com/SBurrell23/Sheets/main/songs/classical/humoresque.abc,
// in C, which has the dotted pairs; the two agree on every other bar)
pub(super) const HUMORESQUE: Song = Song {
    name: "Humoresque",
    origin: "Antonín Dvořák, Humoresques Op. 101 No. 7, 1894",
    feel: "Piano",
    genre: Genre::Classical,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Quarters,
    bass: Bass::Held,
    rhythm: "",
    band: PIANO,
    style: "Classical cadence",
    parts: &[
        Part::new(
            "Theme",
            "G4:3 A4:1 G4:3 A4:1 B4:3 D5:1 E5:3 D5:1 | G5:3 F#5:1 A5:3 G5:1 F#5:3 A5:1 G5:3 E5:1 | \
             D5:3 D5:1 E5:3 D5:1 G5:3 E5:1 D5:3 B4:1 | A4:15 -:1 | \
             G4:3 A4:1 G4:3 A4:1 B4:3 D5:1 E5:3 D5:1 | G5:3 F#5:1 A5:3 G5:1 F#5:3 A5:1 G5:3 E5:1 | \
             D5:3 D5:1 G5:3 G4:1 A4:4 D5:4 | G4:12 -:4",
            "I | IV IV ii°7 IV | I | V7 | I | IV IV ii°7 IV | I V7 | I",
        ),
        Part::new(
            "Middle",
            "B4:4 B5:3 A5:1 A5:4 G5:4 | G5:4 F#5:3 E5:1 E5:4 D5:4 | D5:4 C5:2 E5:2 D5:4 C5:4 | \
             B4:2 A4:2 G4:8 -:4 | B4:4 B5:3 A5:1 A5:4 G5:4 | G5:4 F#5:3 E5:1 E5:4 D5:4 | \
             D5:2 C5:2 D5:2 E5:2 B4:2 A4:2 B4:2 C5:2 | B4:4 Bb4:8 A4:4",
            "I | IV VI7 | ii V7 | I | I | IV VI7 | ii V7 | I i°7 i°7 V7",
        ),
        Part::new(
            "Last theme",
            "G4:3 A4:1 G4:3 A4:1 B4:3 D5:1 E5:3 D5:1 | G5:3 F#5:1 A5:3 G5:1 F#5:3 A5:1 G5:3 E5:1 | \
             D5:3 D5:1 E5:3 D5:1 G5:3 E5:1 D5:3 B4:1 | A4:15 -:1 | \
             G4:3 A4:1 G4:3 A4:1 B4:3 D5:1 E5:3 D5:1 | G5:3 F#5:1 A5:3 G5:1 F#5:3 A5:1 G5:3 E5:1 | \
             D5:3 D5:1 Bb5:3 D5:1 C#5:3 B5:1 A5:3 E5:1 | G5:12 -:4",
            "I | IV IV ii°7 IV | I | V7 | I | IV IV ii°7 IV | I i II7 V7 | I",
        ),
    ],
    form: &["Theme", "Middle", "Last theme"],
    drums_from: 0,
    about: "Dvořák's little piano piece, in 2/4 with two of its bars to each bar here, \
            moved from G flat major to G. Almost every beat is a dotted pair, long then \
            short. The middle phrase drops the dots and ends on a chromatic slide, B, B flat, \
            A; the last bars borrow a B flat from G minor before the tune comes home.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/SchumannR/O15/SchumannOp15No07/SchumannOp15No07.ly
// (Mutopia, the top voice; chords read from the other four voices; the top line agrees
// note for note with https://raw.githubusercontent.com/SBurrell23/Sheets/main/songs/classical/traumerei.abc, in C)
pub(super) const TRAUMEREI: Song = Song {
    name: "Träumerei",
    origin: "Robert Schumann, Kinderszenen Op. 15 No. 7, 1838",
    feel: "Piano",
    genre: Genre::Classical,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 56.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    band: PIANO,
    style: "Circle of fifths",
    parts: &[
        Part::new("Pickup", "-:12 C4:4", ""),
        Part::new(
            "Dream",
            "F4:10 E4:2 F4:2 A4:2 | C5:2 F5:2 F5:8 E5:2 D5:2 | \
             C5:2 F5:2 G4:2 A4:2 Bb4:2 D5:2 F4:2 G4:2 | A4:2 C5:2 G4:8 C4:4 | \
             F4:10 E4:2 F4:2 A4:2 | C5:2 A5:2 A5:6 G5:2 F5:2 E5:2 | \
             F5:2 A5:2 D5:2 F5:2 E5:6 Eb5:2 | D5:4 E5:4 C5:4 -:2 C4:2",
            "I | I IV IV IV | I V7 V7 I | I V V7 V7 | I | I VI7 VI7 VI7 | vi vi V VI7 | \
             II7 II7 V V7",
        )
        // Back to the tempo after the repeat; "ritard." in bar 8.
        .with_tempo(&[tempo(0, 56), glide(112, 42, 12)]),
        Part::new(
            "Middle",
            "F4:10 E4:2 F4:2 A4:2 | C5:2 Eb5:2 Eb5:8 D5:2 C5:2 | \
             Bb4:2 D5:2 G4:2 A4:2 Bb4:6 A4:2 | G4:6 D4:6 -:2 F4:2 | \
             Bb4:10 A4:2 Bb4:2 D5:2 | F5:2 Bb5:2 Bb5:8 A5:2 G5:2 | \
             F5:2 A5:2 D5:2 E5:2 F5:6 E5:2 | D5:6 A4:6 G4:4",
            "I | v7 VI7b9 VI7b9 VI7 | ii VI7 ii ii | ii VI7 ii I | IV | ii7 III7b9 III7b9 III7 | \
             vi III7 vi vi | vi III7 IV V7",
        )
        // "a tempo" in bar 9; "ritard." in bar 16.
        .with_tempo(&[tempo(0, 56), glide(120, 42, 8)]),
        Part::new(
            "Dream again",
            "F4:10 E4:2 F4:2 A4:2 | C5:2 F5:2 F5:8 E5:2 D5:2 | \
             C5:2 F5:2 G4:2 A4:2 Bb4:2 D5:2 F4:2 G4:2 | A4:2 C5:2 G4:8 C4:4 | \
             F4:10 E4:2 F4:2 A4:2 | C5:2 A5:2 A5:6 G5:2 F5:2 D5:2 | \
             C5:2 F5:2 G4:2 A4:2 Bb4:2 D5:2 G4:2 A4:2 | Bb4:2 D5:2 D4:2 E4:2 F4:8",
            "I | I IV IV IV | I V7 V7 I | I V V7 V7 | I | I II7 II7 II7 | I V7 V7 ii | \
             ii V7 I I",
        )
        // "a tempo" in bar 17; the fermata on the high A in bar 22, then
        // "ritardando" to the end and a fermata on the last chord.
        .with_tempo(&[
            tempo(0, 56),
            fermata(84, 6),
            glide(90, 42, 30),
            fermata(120, 8),
        ]),
    ],
    form: &["Pickup", "Dream", "Dream", "Middle", "Dream again"],
    drums_from: 1,
    about: "Schumann's daydream from his Scenes from Childhood. Every phrase starts the same \
            way, a leap up to a long F, then a climb: to the F above it, then to A, and in the \
            middle to E flat and to B flat. The middle phrases each lean on a secondary \
            dominant, D7 or A7, with a flat ninth in the tune. Each half slows down at its end \
            (ritard.) and starts again in tempo, and the last phrase waits on its high A \
            under a fermata. The chords are the score's, simplified to one per beat; the \
            grace notes are left out.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/BeethovenLv/O92/Symphony7_2/Symphony7_2-lys/violinotwo.ly
// (Mutopia: the second violins, bars 27 to 74; violinoone.ly bars 51 to 74 for the theme
// an octave up; chords from viola.ly, violoncello.ly and basso.ly, bars 3 to 26)
pub(super) const SYMPHONY_7_ALLEGRETTO: Song = Song {
    name: "Symphony No. 7, Allegretto",
    origin: "Ludwig van Beethoven, 1812",
    feel: "Processional",
    genre: Genre::Classical,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Epic toms",
    band: Band::new(gm::VIOLA, gm::STRING_ENSEMBLE, gm::CONTRABASS),
    style: "Epic minor",
    parts: &[
        Part::new(
            "Theme",
            "E4:4 E4:2 E4:2 E4:4 E4:4 | E4:4 E4:2 E4:2 E4:4 E4:4 | E4:4 E4:2 F#4:2 G4:4 G4:4 | \
             G4:4 G4:2 G4:2 G4:4 -:4 | G4:4 G4:2 A4:2 B4:4 B4:4 | F#4:4 F#4:2 G#4:2 A4:4 A4:4 | \
             E4:4 E4:2 E4:2 E4:4 E4:4 | E4:4 F#4:2 G#4:2 A4:4 -:4 | G4:4 G4:2 A4:2 B4:4 B4:4 | \
             F#4:4 F#4:2 G#4:2 A4:4 A4:4 | E4:4 E4:2 E4:2 E4:4 E4:4 | E4:4 F#4:2 G#4:2 A4:4 -:4",
            "i V | V i | i i7 | bVII bIII | bIII II | ii ii I i | i i V i | V i | bIII II | \
             ii ii I i | i i V i | V i",
        ),
        Part::new(
            "Counter-melody",
            "C5:8 B4:8 | D5:6 C5:1 B4:1 B4:2 C5:2 C5:4 | C5:6 C5:1 D5:1 D#5:2 E5:2 E5:2 E5:2 | \
             F5:6 E5:1 D5:1 D5:2 E5:2 E5:4 | E5:8 D#5:8 | \
             D5:2 B4:1 C#5:1 D5:2 E5:1 D5:1 D5:2 C#5:2 C5:4 | C5:8 B4:4 C5:4 | \
             D5:6 C5:1 B4:1 A4:2 A4:1 B4:1 C5:2 C5:1 D5:1 | E5:8 D#5:8 | \
             D5:2 B4:1 C#5:1 D5:2 E5:1 D5:1 D5:2 C#5:2 C5:4 | C5:8 B4:4 C5:4 | \
             D5:6 C5:1 B4:1 A4:4 -:4",
            "i V | V i | i i7 | bVII bIII | bIII II | ii ii I i | i i V i | V i | bIII II | \
             ii ii I i | i i V i | V i",
        ),
        Part::new(
            "Theme above",
            "E5:4 E5:2 E5:2 E5:4 E5:4 | E5:4 E5:2 E5:2 E5:4 E5:4 | E5:4 E5:2 F#5:2 G5:4 G5:4 | \
             G5:4 G5:2 G5:2 G5:4 -:4 | G5:4 G5:2 A5:2 B5:4 B5:4 | F#5:4 F#5:2 G#5:2 A5:4 A5:4 | \
             E5:4 E5:2 E5:2 E5:4 E5:4 | E5:4 F#5:2 G#5:2 A5:4 -:4 | G5:4 G5:2 A5:2 B5:4 B5:4 | \
             F#5:4 F#5:2 G#5:2 A5:4 A5:4 | E5:4 E5:2 E5:2 E5:4 E5:4 | E5:4 F#5:2 G#5:2 A5:4 -:4",
            "i V | V i | i i7 | bVII bIII | bIII II | ii ii I i | i i V i | V i | bIII II | \
             ii ii I i | i i V i | V i",
        ),
    ],
    form: &["Theme", "Counter-melody", "Theme above"],
    drums_from: 1,
    about: "The slow movement that made the first audience ask for it again. It is in 2/4, \
            with two of its bars to each bar here. The theme is hardly a tune: one long and \
            two short notes, then two long ones, mostly on E, over chords that move \
            beneath it. When the theme comes back, Beethoven sets a singing counter-melody \
            against it, just below; here that counter-melody, as the second violins play \
            it, gets a turn of its own, between the theme and the theme an octave higher.",
};

// Source: https://raw.githubusercontent.com/baobach/abc_search/main/database/abc/QmXEX4PRkSRKpgibvKQTEkiErGaDjeYoEZvPQfas2Rs7Dy.abc
// (an ABC piano score marked public domain; the melody and chords up to the trio, with the
// tune an octave lower from bar 5 on, as in the Wikifonia lead sheet
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/main/wikifonia-dataset/original-abc/Joh.abc,
// which has the same tune, with some notes held longer and D for F sharp in bars 29 and 30)
pub(super) const RADETZKY_MARCH: Song = Song {
    name: "Radetzky March",
    origin: "Johann Strauss I, Op. 228, 1848",
    feel: "March",
    genre: Genre::Classical,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Offbeat,
    bass: Bass::RootFifth,
    rhythm: "March",
    band: Band::new(gm::VIOLIN, gm::FRENCH_HORN, gm::CONTRABASS),
    style: "Classical cadence",
    parts: &[
        Part::new(
            "Intro",
            "D5:2 D5:2 D5:2 -:2 D5:2 D5:2 D5:2 -:2 | F#5:4 E5:4 D5:6 C#5:2 | \
             B4:2 A#4:2 B4:2 C#5:2 D5:4 E5:4 | A4:4 -:8 F#4:2 E#4:2",
            "I | I | II7 | V",
        ),
        Part::new(
            "March",
            "F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 E4:2 -:2 D4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 F#4:2 | G4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 E4:2 | \
             F#4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 F#4:2 | E4:2 -:2 C#5:4 D4:2 -:2 B4:4 | \
             A4:2 -:2 A4:2 G#4:2 A4:2 B4:2 A4:2 G4:2",
            "I | I | I | I | V7 | I | V II7 | V",
        ),
        Part::new(
            "March end",
            "F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 E4:2 -:2 D4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 F#4:2 | G#4:2 -:2 F#5:2 -:2 E5:8 | \
             F#4:2 -:2 E5:2 -:2 D5:8 | C#5:6 B4:2 A4:2 G4:2 F#4:2 E4:2 | \
             D4:4 D4:4 D4:4 D5:2 C#5:2",
            "I | I | I | I | II7 | I | V7 | I",
        ),
        Part::new(
            "Second theme",
            "D5:2 -:2 D5:2 C#5:2 D5:2 -:2 D5:2 C#5:2 | D5:2 -:2 G5:2 -:2 B4:2 -:2 D5:2 C#5:2 | \
             D5:2 -:2 D5:2 C#5:2 D5:2 -:2 D5:2 C#5:2 | D5:2 -:2 F#5:2 -:2 A4:2 -:2 A4:2 -:2 | \
             A4:8 G4:2 -:2 B4:2 -:2 | B4:8 A4:2 -:2 D5:2 -:2 | \
             C#5:2 -:2 C#5:2 B4:2 A4:2 -:2 G4:2 -:2 | B4:2 -:2 B4:2 A4:2 G4:2 -:2 F#4:2 -:2 | \
             A4:8 G4:2 -:2 B4:2 -:2 | B4:8 A4:2 -:2 D5:2 -:2 | \
             C#5:2 -:2 C#5:2 A#4:2 B4:2 -:2 D5:2 -:2 | C#5:2 -:2 C#5:2 A#4:2 B4:2 -:2 D5:2 -:2 | \
             F#5:4 F#5:2 F#5:2 F#5:4 F#5:2 F#5:2 | F#5:4 F#4:2 F#4:2 F#4:4 F#4:2 F#4:2",
            "IV | IV | I | I | V7 | I | V7 | I | V7 | I | III vi | III vi | III vi III vi | \
             III vi III vi",
        ),
        Part::new(
            "March again",
            "F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 E4:2 -:2 D4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 F#4:2 | G4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 E4:2 | \
             F#4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 F#4:2 | E4:2 -:2 C#5:4 D4:2 -:2 B4:4 | \
             A4:2 -:2 A4:2 G#4:2 A4:2 B4:2 A4:2 G4:2",
            "III | III | I | I | I | I | V7 | I | V II7 | V",
        ),
        Part::new(
            "Last march end",
            "F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 E4:2 -:2 D4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 F#4:2 E#4:2 F#4:2 -:2 F#4:2 E#4:2 | \
             F#4:2 -:2 B4:2 -:2 A4:2 -:2 A4:2 F#4:2 | G#4:2 -:2 F#5:2 -:2 E5:8 | \
             F#4:2 -:2 E5:2 -:2 D5:8 | C#5:6 B4:2 A4:2 G4:2 F#4:2 E4:2 | D4:4 D4:4 D4:4 -:4",
            "I | I | I | I | II7 | I | V7 | I",
        ),
    ],
    form: &[
        "Intro",
        "March",
        "March end",
        "Second theme",
        "March again",
        "Last march end",
    ],
    drums_from: 0,
    about: "The march the Vienna New Year's Concert ends with, the audience clapping along. \
            It is in cut time, one of its bars to each bar here. The tune taps on F sharp \
            with the E sharp just below it. The second theme ends on F sharp major chords, \
            and the tune comes back over them before D major returns. The trio is left out.",
};

/// The Promenade's 5/4 as Mussorgsky writes it: five quarter notes, each
/// a beat of its own, not the 3 + 2 of `FIVE_FOUR`.
const FIVE_STEPS: TimeSignature = TimeSignature::new(5, 4);

/// The walking tune, a bar of 5/4 and one of 6/4, as the right hand plays
/// it alone and then on top of the first chords.
const PROMENADE_THEME: &str = "G4:4 F4 Bb4 C5:2 F5 D5:4 | C5:2 F5 D5:4 Bb4 C5 G4 F4";

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/2144afd6f52d56c5b6995b8b589ef1268b3139f0/ftp/MussorgskyM/pictures-at-an-exhibition/pictures-at-an-exhibition-lys/promenade-1.ly
// (Pavel Lamm's edition of the piano original, the only source here: the first Promenade,
// all 24 bars, once, each bar in the meter it is written in. The melody is the top note of
// the right hand. The chart is read from both hands, one chord to each quarter-note beat;
// where a beat has two, one on each eighth, it keeps the beat's own chord and leaves out the
// one passing by: the six-four chords over the stepping bass in bars 10 and 13, the E flat
// chord leaning on G minor and the F chord passing up to B flat in bars 17 and 19, and the
// inner voice's E flat, rising to E, in the C major chords of bar 16. Bare octaves get the
// chord they imply: F (V), A flat (bVII), G minor (vi). The E flat held over G flat in bars
// 9 and 11, and the G over B flat in bar 12, make sixth chords (bVI6, I6); vi6 in bar 14 is
// the score's E, B flat, D over G. The band's bass plays each chord's root, so the score's
// inversions, such as F/A and C/E, and the bass's neighbour notes in bars 9, 11 and 12, are
// not heard. Bars 6 and 8 begin with two beats of the right hand alone, which a chart cannot
// leave silent: their left hand is written out instead, its octaves as single notes, and
// their chords are only shown on the chord sheet. The tempo, 88 to the quarter, is the
// edition's own MIDI tempo: a walking pace, an allegro held back.)
pub(super) const PROMENADE: Song = Song {
    name: "Promenade",
    origin: "Modest Mussorgsky, Pictures at an Exhibition, 1874",
    feel: "Piano",
    genre: Genre::Classical,
    tonic: ASharp,
    mode: Major,
    time: FIVE_STEPS,
    tempo: 88.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    band: PIANO,
    style: "",
    parts: &[
        Part::new("Call", PROMENADE_THEME, "").in_meters(&[FIVE_STEPS, SIX_FOUR]),
        Part::new("Answer", PROMENADE_THEME, "vi V vi V iii | V I vi II II V")
            .in_meters(&[FIVE_STEPS, SIX_FOUR]),
        Part::new("Second call", "F4:4 G4 D4 F4:2 G4 C4:4", "").in_meters(&[FIVE_STEPS]),
        Part::new(
            "Second answer",
            "G4:2 A4 F4:4 F5 D5 C5:2 Bb4 F4:4",
            "V V V I vi7 V",
        )
        .with_left("-:8 F2:4 Bb2 G2 F2")
        .in_meters(&[SIX_FOUR]),
        Part::new("Third call", "F4:4 G4 D4 F4:2 G4 Eb4:4", "").in_meters(&[FIVE_STEPS]),
        Part::new(
            "Third answer",
            "Bb4:2 C5 Ab4:4 Ab5 F5 Eb5:2 Db5 Ab4:4",
            "bVII bVII bVII bIII i7 bVII",
        )
        .with_left("-:8 Ab2:4 Db3 Bb2 Ab2")
        .in_meters(&[SIX_FOUR]),
        // The tune in octaves over a held E flat, then the chords in D
        // flat major, and the first bar again, cut short.
        Part::new(
            "In D flat",
            "Ab4:4 Bb4 Ab4 Bb4:2 C5 Eb5 Bb4 Ab4:4 | \
             Db5:2 Eb5 F5 Ab5 Gb5 F5 Eb5 Gb5 F5 Db5 Eb5:4 | \
             Ab4:4 Bb4 Ab4 Bb4:2 C5 Eb5 Bb4",
            "bVI6 | bIII bIII iv bVII i bVII | bVI6",
        )
        .in_meters(&[SIX_FOUR, SIX_FOUR, FIVE_STEPS]),
        // The same a major third higher, over a held G, in F.
        Part::new(
            "In F",
            "C5:4 D5 C5 D5:2 F5 G5 D5 C5:4 | F5:2 G5 A5 C6 Bb5 A5 G5 Bb5 A5 F5 G5:4 | \
             A5:2 E5 F5:4 A5 D5 A5 D5 | F5:2 C5 D5:4 F5 D5 F5:2 C5 D5:4 | \
             C5:4 A4 Bb4 C5 A4 Bb4:2 D5",
            "I6 | V V vi II iii II | vii I V vi6 V vi6 | iii vi7 iii vi7 iii vi7 | \
             II V vi7 II V vi7",
        )
        .in_meters(&[SIX_FOUR; 5]),
        Part::new(
            "Home",
            "C5:4 A4 C5 F5 Eb5:2 D5 C5 Bb4 | C5:4 D5 F5 G5:2 Bb5 F5:4 G5 | \
             F5:4 Eb5:2 D5 C5 Bb4 C5:4 D5 F5 | G5:2 Bb5 F5:4 G5 F5 G4 F4 | \
             G5:2 Bb5 F5:4 G5 F5 G4 F4 | Bb4:4 C5:2 F5 D5:4 C5:2 F5 D5:4 Bb4 | \
             C5:4 G4 F4 G4 F4 Bb4 | C5:2 F5 D5:4 Bb4 Eb5 C5 Bb4",
            "II V6 II V vi I | V I V IV V vi | V vi I V I V | IV V vi V vi V | \
             II7 V vi V IV V | I V I V vi7 vi7 | ii II V vi V vi | V I vi ii V I",
        )
        .in_meters(&[SIX_FOUR; 8]),
    ],
    form: &[
        "Call",
        "Answer",
        "Second call",
        "Second answer",
        "Third call",
        "Third answer",
        "In D flat",
        "In F",
        "Home",
    ],
    drums_from: 0,
    about: "The composer himself, walking from one picture to the next. The meter changes \
            from bar to bar, five quarter notes, then six, as a sentence or a Russian folk \
            song takes the breath it needs instead of keeping a dance beat. Count every \
            quarter, 1 2 3 4 5, 1 2 3 4 5 6, each step as heavy as the last: the marking asks \
            for a strict allegro \"in the Russian manner, without gaiety, but a little held \
            back\", and nearly every note of the lone tune is marked tenuto, leaned on. The \
            right hand sings the phrase alone and the whole piano answers in chords of B flat \
            major, a new one on almost every beat: G minor, F, G minor, F, D minor (vi V vi V \
            iii), then F, B flat, G minor, C major (II, the V of V) and F. Twice more the tune \
            sets off alone; the answers turn to F, then to D flat major (bIII), where the tune \
            circles over a held E flat. The same passage comes back a major third higher, in \
            F, and the last bars bring it home to B flat in chords and bare octaves, ending \
            ii V I.",
};

const GRAZIA_CELLOS: &str = "F#3:4 G3:4 (3 A3:2 G3 A3 B3:4 C#4:4 | D4:4 B3:4 C#4:12 | \
     A3:4 B3:4 (3 C#4:2 B3 C#4 D4:4 E4:4 | F#4:4 D4:4 E4:12 | \
     G4:4 F#4:4 (3 E4:2 F#4 E4 D4:4 C#4:4 | F#4:2 -:1 G4:1 F#4:2 -:1 E#4:1 F#4:12 | \
     F#4:4 E4:4 (3 D4:2 E4 D4 C#4:4 B3:4 | B4:2 A4 G4 F#4 E4 Eb4 D4 C#4 B3 Bb3";
const GRAZIA_CELLOS_CHORDS: &str = "I | D/F# D/F# V V V | D/F# | I I V V V | \
     ii ii III7 III7 III7 | vi | vi vi II7 II7 II7 | V7";
const GRAZIA_WOODWINDS: &str = "F#5:4 G5:4 (3 A5:2 G5 A5 B5:4 C#6:4 | D6:4 B5:4 C#6:12 | \
     A5:4 B5:4 (3 C#6:2 B5 C#6 D6:4 E6:4 | F#6:4 D6:4 E6:12 | \
     E6:4 F#6:4 (3 E6:2 F#6 E6 D6:4 C#6:4 | B5:2 -:1 C#6:1 B5:2 -:1 A#5:1 B5:12 | \
     D6:4 E6:4 (3 D6:2 E6 D6 C#6:4 B5:4";
const GRAZIA_WOODWINDS_CHORDS: &str = "I | I I V7 V7 V7 | I I V V V | I I V7 V7 V7 | \
     Em/G Em/G III7 III7 III7 | Bm/D | Bm7/F# Bm7/F# II7 II7 II7";
const GRAZIA_OVER_A: &str = "B5:4 E5:4 (3 E5:2 D5 E5 F#5:4 G5:4 | \
     A5:2 -:1 B5:1 A5:2 -:1 G5:1 A5:12 | A5:4 D6:4 (3 A5:2 B5 A5 G5:4 F#5:4 | \
     F#5:2 -:1 A5:1 G5:2 -:1 D5:1 E5:12 | B5:4 E5:4 (3 E5:2 D5 E5 F#5:4 G5:4 | \
     A5:2 -:1 B5:1 A5:2 -:1 G5:1 A5:12 | A5:4 B5:4 (3 A5:2 G#5 A5 C#6:4 B5:4 | \
     A5:2 -:1 B5:1 A5:2 -:1 G#5:1 F#5:2 D5 E#5 F#5 G#5 A5";
const GRAZIA_OVER_A_CHORDS: &str =
    "V9 | D/A | D/A | D/A D/A V7 V7 V7 | V9 | D/A | VII7 | VII7 VII7 iii iii iii";
const GRAZIA_OBOE: &str = "B5:4 G5:4 (3 E5:2 D5 E5 F#5:4 G5:4 | \
     A5:2 -:1 B5:1 A5:2 -:1 G5:1 A5:12 | A5:4 D6 A5 G5 F#5 | \
     F#5:2 -:1 A5:1 G5:2 -:1 D5:1 E5:12 | B5:4 E5 E5 F#5 G5 | \
     A5:2 -:1 B5:1 A5:2 -:1 G5:1 A5:12 | A5:4 B5 C#6 E6 D6 | \
     C#6:2 -:1 A5:1 B5:2 -:1 G#5:1 A5:6 -:6";
const GRAZIA_OBOE_CHORDS: &str = "V9 | D/A | D/A | D/A D/A V7 V7 V7 | V9 | D/A | \
     F#m/C# F#m/C# iii iii iii | II7 II7 V V V";
const GRAZIA_UNDER_SCALES: &str = "F#4:4 G4:4 (3 A4:2 G#4 A4 B4:4 C#5:4 | D5:4 B4:4 C#5:12 | \
     A4:4 B4:4 (3 C#5:2 B4 C#5 D5:4 E5:4 | F#5:4 D5:4 E5:12 | \
     G5:4 F#5:4 (3 E5:2 F#5 E5 D5:4 C#5:4 | F#5:2 -:1 G5:1 F#5:2 -:1 E#5:1 F#5:12 | \
     F#5:4 E5:4 (3 D5:2 E5 D5 C#5:4 B4:4 | B5:2 A5 G5 F#5 E5 Eb5 D5 C#5 B4 Bb4";
const GRAZIA_UNDER_SCALES_CHORDS: &str = "I | I I V V V | I | I I V7 V7 V7 | ii | vi | vi | V7";
const GRAZIA_VIOLINS: &str = "F#5:4 G5:4 (3 A5:2 G5 A5 B5:4 C#6:4 | D6:4 B5:4 C#6:12 | \
     A5:4 B5:4 (3 C#6:2 B5 C#6 D6:4 E6:4 | F#6:4 D6:4 E6:8 F6:4 | \
     C5:4 A4:4 (3 B4:2 A4 B4 C5:4 D5:4 | E5:4 C5:4 D5:12 | \
     C5:4 D5:4 (3 E5:2 D5 E5 F5:4 F#5:4 | G5:4 E5:4 D5:12";
const GRAZIA_VIOLINS_CHORDS: &str = "I | D/F# D/F# V V V | D/F# | I I V V V | \
     C/G C/G IV IV IV | C/G C/G IV IV IV | C/G | C/G C/G IV IV IV";
const GRAZIA_CLIMAX: &str = "Bb6:8 A6:8 G6:4 | F6:4 G6:4 A6:12 | A6:8 G#6:8 F#6:4 | \
     E6:4 F#6:4 G#6:12 | G6:4 A6:4 (3 G6:2 A6 G6 F#6:2 -:2 E#6:4 | \
     F#6:2 -:2 D6:4 E6:2 D6 C#6 B5 A5 G5 | D4:4 E4:4 (3 F#4:2 E4 F#4 G4:4 A4:4 | \
     B4:4 C#5:4 D5:2 -:10";
const GRAZIA_CLIMAX_CHORDS: &str = "Bb/F Bb/F bIII7 bIII7 bIII7 | \
     Bb/F Bb/F bIII7 bIII7 bIII7 | A/E A/E II7 II7 II7 | A/E A/E II7 II7 II7 | \
     A7b5/Eb | I I A7/G A7/G A7/G | D/A | V7 V7 I I I";
// The trio, in B minor over a held D: its chords are named as on a lead
// sheet, with the D after the slash.
const GRAZIA_TRIO: &str = "F#5:6 F#5:2 F#5:8 E5:4 | D5:6 D5:2 D5:8 C#5:4 | \
     B4:6 B4:2 D5:8 C#5:4 | B4:6 B4:2 G4:8 F#4:4 | F#5:6 F#5:2 F#5:8 E5:4 | \
     D5:6 D5:2 D5:8 C#5:4 | B4:6 B4:2 D5:8 C#5:4 | B4:6 B4:2 G5:8 F#5:4";
const GRAZIA_TRIO_CHORDS: &str = "Bm/D Bm/D A#°7/D A#°7/D A#°7/D | \
     Bm/D Bm/D A#°7/D A#°7/D A#°7/D | Bm/D Bm/D A#°7/D A#°7/D A#°7/D | \
     Bm/D Bm/D A#°7/D A#°7/D A#°7/D | Bm/D Bm/D A#°7/D A#°7/D A#°7/D | \
     Bm/D Bm/D A#°7/D A#°7/D A#°7/D | Bm/D Bm/D A#°7/D A#°7/D A#°7/D | \
     Bm/D Bm/D A#°7/D A#°7/D A#°7/D";
const GRAZIA_STRAIN: &str = "F#5:6 G5:2 A5:8 B5:4 | A5:6 G5:2 F#5:8 E5:4 | \
     E5:6 F#5:2 G5:8 A5:4 | G5:6 F#5:2 E5:8 D5:4 | D5:6 E5:2 F5:8 D5:4 | \
     G5:6 A5:2 Bb5:8 G5:4 | B5:6 C#6:2 D6:8 B5:4 | D6:6 E6:2 F#6:8 D6:4";
const GRAZIA_STRAIN_CHORDS: &str = "Bm/D Bm/D D#°7/D D#°7/D D#°7/D | Em/D | \
     Em/D Em/D C#°7/D C#°7/D C#°7/D | D | Bb/D | Eb/D | G#°7/D | Bm/D";
const GRAZIA_SIGHS: &str = "B5:6 B5:2 C#5:8 B4:4 | G#4:4 A#4:4 (3 B4:2 A#4 B4 C#5:4 D5:4 | \
     G5:6 G5:2 B4:8 A#4:4 | B5:4 C#6:4 (3 D6:2 C#6 D6 E6:4 F#6:4";
const GRAZIA_SIGHS_CHORDS: &str =
    "Em/D Em/D C#7b5/D C#7b5/D C#7b5/D | Bm/D | Em/D Em/D A#°7/D A#°7/D A#°7/D | Bm/D";
const GRAZIA_BACK: &str = "C5:4 D5:4 (3 E5:2 D5 E5 F#5:4 G5:4 | \
     B4:4 C#5:4 (3 D5:2 C#5 D5 E5:4 E#5:4 | G#4:4 A#4:4 (3 B4:2 A#4 B4 C#5:4 D5:4 | \
     B4:4 C#5:4 (3 D5:2 C#5 D5 E5:4 F#5:4 | C4:4 D4:4 (3 E4:2 D4 E4 F#4:4 G4:4 | \
     B3:4 C#4:4 (3 D4:2 C#4 D4 E4:4 E#4:4 | A3:4 B3:4 (3 C#4:2 B3 C#4 D4:4 E4:4";
const GRAZIA_BACK_CHORDS: &str = "C/D | G#°7/D | Bm/D | Bm/D | C/D | G#°7/D | I";
const GRAZIA_RETURN: &str = "F#4:4 G4:4 (3 A4:2 G4 A4 B4:4 C#5:4 | D5:4 B4:4 C#5:12 | \
     A4:4 B4:4 (3 C#5:2 B4 C#5 D5:4 E5:4 | F#5:4 D5:4 E5:12 | \
     G5:4 F#5:4 (3 E5:2 F#5 E5 D5:4 C#5:4 | F#5:2 -:1 G5:1 F#5:2 -:1 E#5:1 F#5:12 | \
     F#5:4 E5:4 (3 D5:2 E5 D5 C#5:4 B4:4 | B5:2 A5 G5 F#5 E5 Eb5 D5 C#5 B4 Bb4";
const GRAZIA_CODA: &str = "D6:8 C#6:12 | B5:8 A5:12 | G5:8 F#5:12 | E5:8 D5:12 | \
     D5:8 C#5:12 | B4:8 A4:12 | G4:8 F#4:12 | E4:8 D4:12";
const GRAZIA_CODA_CHORDS: &str = "I I C#°/D C#°/D C#°/D | Bm/D Bm/D F#m/D F#m/D F#m/D | \
     G/D G/D I I I | A/D A/D I I I | I I C#°/D C#°/D C#°/D | Bm/D Bm/D I I I | \
     G/D G/D I I I | A/D A/D Bm/D Bm/D Bm/D";
const GRAZIA_FAREWELL: &str = "F4:6 F4:2 E4:8 D4:4 | A5:20 | F4:6 F4:2 E4:8 D4:4 | A5:20 | \
     F4:6 F4:2 E4:8 D4:4 | Bb4:6 Bb4:2 A4:8 G4:4 | D5:6 D5:2 C#5:8 B4:4 | \
     F#5:6 F#5:2 E5:8 D5:4 | A5:6 A5:2 G5:8 F#5:4 | B4:6 B4:2 A4:8 G4:4 | \
     D4:6 D4:2 C#4:8 B3:4 | F#3:6 F#3:2 E3:8 D3:4";
const GRAZIA_FAREWELL_CHORDS: &str =
    "Bb/D | I | Bb/D | I | Bb/D | Eb/D | G#°7/D | I | I7 | Em/D | G#°7/D | I";

// Source: https://raw.githubusercontent.com/kyuchia/mctl-symphony-dataset/8353dc785159a1da6e86aa06965c46abaeca227a/mctl_symphony_dataset/tc2/sheet.xml
// and its harmony analysis,
// https://raw.githubusercontent.com/kyuchia/mctl-symphony-dataset/8353dc785159a1da6e86aa06965c46abaeca227a/mctl_symphony_dataset/tc2/annotations/form/harmony.csv
// (The full score of the MCTL symphony dataset, the only score here: all 178 bars, with the
// repeats it writes, 210 bars as heard. Bars 1 to 16 twice, with their first and second
// endings, then 17 to 56; the trio, 57 to 64 twice, 65 to 72 twice, then 73 to 95 (73 to 80
// are 57 to 64 again, and 85 to 88 are 81 to 84); the return, 96 to 151, written out: the
// first eight bars with the tune in the first violins, an octave above the cellos, then
// bars 9 to 15, the second ending and 17 to 56 note for note; the coda, 152 to 178. The
// melody is the tune as heard, wherever it is: the cellos in bars 1 to 8, the flutes in 9
// to 16, the first violins in 17 to 24, the first oboe in 25 to 32 (the dataset has the
// flutes an octave below the oboes there), the flutes and oboes in 33 to 40 under the
// violins' falling chromatic scales, the first violins in 41 to 44 and 49 to 54, the
// woodwinds in 45 to 48 under the violins' held G, and the flutes in 55 and 56. Each ending
// leads on with the line of the instrument that takes the tune next: the cellos' back to
// bar 1, the violins' scale into bar 17, the oboes' into bar 25; at the end of bar 32 the
// oboes hold their A and the violins' scale down to the next section is left out. In the
// trio the first violins have the tune, doubled by the flutes and the cellos, the woodwinds
// take bars 70 and 72, and from bar 81 strings and woodwinds answer each other, bar by bar.
// In the coda the oboes, then the trumpets, walk down the scale while the cellos and the
// strings climb it; then the cellos and the first oboe take turns, then the flute, oboe,
// clarinet, flutes, oboe, bassoon, horn and clarinet, a bar each, and at last the first
// violins alone, answered by the clarinet. The chart follows the analysis, one chord to
// each felt beat, the short one (quarters 1 and 2) and the long one (3 to 5), except where
// the notes say otherwise: F#7 (III7) on the long beat of bar 5, from the winds' A sharp,
// where the analysis keeps E minor; E7 (II7) for its E minor on the long beat of bar 7,
// with the G sharp in the score; V7 in bar 8 (the winds' G); A on the long beat of bar 11;
// B minor seventh over F# (Bm7/F#), then E7, in bar 15, where the analysis repeats bar 13's
// E minor over G and the score has no G; A9 (V9) in bars 17, 21, 25 and 29, the E, G and B
// over C# and A; D/A in bars 18 and 22 and on the short beat of 20 and 28, all over the
// basses' held A; C#7 then F# minor in bar 24; E7 then A in bar 32, where the analysis
// gives the first quarter to D#ø7; C/G then G over the basses' held G in bars 45 to 48,
// where its labels are a fourth off; B flat over F, then F7, in bars 49 and 50, and A over
// E, then E7, in 51 and 52; the French sixth A, C#, E flat, G over E flat in bar 53
// (A7b5/Eb) for its V43. In the trio, over the basses' D, bars 61, 73 and 77 have the
// chords of bar 57, which they repeat note for note; bars 81 and 85 E minor, then the
// French sixth G, B, C#, E# (C#7b5/D), where the analysis keeps B minor; 82 and 86 B minor
// for its F# minor; 87 the chords of 83. In the coda, D for the analysis' F# minor on the
// long beat of bar 157 (there is no C#); B flat over D in bars 160, 162 and 164 (B flat, D,
// F), where it has D minor; E flat over D for its augmented chord in 165; a diminished
// seventh, G#, B, D, F, in 166 for its half-diminished one; and D7 for its D major in 168,
// with the C in the score. Slash chords show the bass where the score's bass and the
// analysis agree, and over every held bass note, so that the band's bass plays the score's.
// Bars 172 and 174, the first violins alone, show their chord, A, on the chord sheet but
// have no accompaniment; the last two bars, a chord plucked on the third beat and a chord
// held by the winds over the strings' last pluck, are written out. The dataset marks the
// movement "Allegretto con grazia" (printed scores say Allegro con grazia) and plays it at
// 144 quarter notes a minute, the tempo here: it has no other tempo marks, only fermatas
// over the final rest, kept. In bars 56 and 151 the dataset has the flutes' scale, D E F#
// G A B, go on to C#4 and D4, a seventh below the B, which looks like an octave slip: here
// the scale goes on up to C#5 and D5, the tonic the trio's first F# sits above.)
pub(super) const PATHETIQUE_WALTZ: Song = Song {
    name: "Pathétique Symphony, Allegro con grazia",
    origin: "Pyotr Ilyich Tchaikovsky, Symphony No. 6 \"Pathétique\", second movement, 1893",
    feel: "Waltz in five",
    genre: Genre::Classical,
    tonic: D,
    mode: Major,
    time: FIVE_FOUR_TWO_THREE,
    tempo: 144.0,
    comping: Comping::Accents,
    bass: Bass::Accents,
    rhythm: "",
    band: Band::new(gm::STRING_ENSEMBLE, gm::PIZZICATO_STRINGS, gm::CONTRABASS),
    style: "",
    parts: &[
        Part::new("Cellos", GRAZIA_CELLOS, GRAZIA_CELLOS_CHORDS),
        Part::new("Woodwinds", GRAZIA_WOODWINDS, GRAZIA_WOODWINDS_CHORDS),
        // The flutes' last bar, and the cellos lead back to the start.
        Part::new(
            "First ending",
            "A5:2 -:1 B5:1 A5:2 -:1 G#5:1 A5:2 A3 B3 A3 G#3 G3",
            "V",
        ),
        // The same, and the violins run up to the next section.
        Part::new(
            "Second ending",
            "A5:2 -:1 B5:1 A5:2 -:1 G#5:1 A5:2 D5 E5 F#5 G5 A5",
            "V",
        ),
        Part::new("Over A", GRAZIA_OVER_A, GRAZIA_OVER_A_CHORDS),
        Part::new("Oboe over A", GRAZIA_OBOE, GRAZIA_OBOE_CHORDS),
        Part::new(
            "Under scales",
            GRAZIA_UNDER_SCALES,
            GRAZIA_UNDER_SCALES_CHORDS,
        ),
        Part::new("Violins", GRAZIA_VIOLINS, GRAZIA_VIOLINS_CHORDS),
        Part::new("Climax", GRAZIA_CLIMAX, GRAZIA_CLIMAX_CHORDS),
        Part::new("Trio", GRAZIA_TRIO, GRAZIA_TRIO_CHORDS).in_key(B, HarmonicMinor),
        Part::new("Second strain", GRAZIA_STRAIN, GRAZIA_STRAIN_CHORDS).in_key(B, HarmonicMinor),
        Part::new("Sighs", GRAZIA_SIGHS, GRAZIA_SIGHS_CHORDS).in_key(B, HarmonicMinor),
        Part::new("Back to D", GRAZIA_BACK, GRAZIA_BACK_CHORDS),
        Part::new("Return", GRAZIA_RETURN, GRAZIA_CELLOS_CHORDS),
        Part::new("Coda", GRAZIA_CODA, GRAZIA_CODA_CHORDS),
        Part::new("Farewell", GRAZIA_FAREWELL, GRAZIA_FAREWELL_CHORDS),
        // The violins alone: the chord is only on the chord sheet.
        Part::new("Violins alone", "A3:4 B3:4 (3 C#4:2 B3 C#4 D4:4 E4:4", "V").with_left("-:20"),
        Part::new("Clarinet", "F#3:6 F#3:2 E3:8 D3:4", "I"),
        Part::new("Last chords", "-:8 F#5:2 -:10 | A3:18 -:2", "I | I")
            .with_left("-:8 D3+A3+D4+F#4+A4:2 -:10 | D2+A2+D3+F#3:18 -:2")
            .with_tempo(&[fermata(38, 2)]),
    ],
    form: &[
        "Cellos",
        "Woodwinds",
        "First ending",
        "Cellos",
        "Woodwinds",
        "Second ending",
        "Over A",
        "Oboe over A",
        "Under scales",
        "Violins",
        "Climax",
        "Trio",
        "Trio",
        "Second strain",
        "Second strain",
        "Trio",
        "Sighs",
        "Sighs",
        "Back to D",
        "Return",
        "Woodwinds",
        "Second ending",
        "Over A",
        "Oboe over A",
        "Under scales",
        "Violins",
        "Climax",
        "Coda",
        "Farewell",
        "Violins alone",
        "Clarinet",
        "Violins alone",
        "Clarinet",
        "Clarinet",
        "Last chords",
    ],
    drums_from: 0,
    about: "The \"waltz in five\": five quarter notes to the bar, felt as two beats, a short \
            one and a long one, counted 1 2, 1 2 3. A waltz counts 1 2 3 in every bar; here \
            two quarters cut in before every three, so the dance limps, gracefully, always \
            leaning into the long beat, and the accompaniment marks the two beats, on 1 and on \
            3. The cellos sing the tune in D major, up the scale F# G A B C# to D with a \
            little triplet turn on the third quarter, each phrase coming to rest on the long \
            beat. The middle section, the trio, marked \"con dolcezza e flebile\", sweetly and \
            plaintively, turns to B minor, and under it the double basses play nothing but D \
            from its first bar to its last, a pedal point that never moves while the chords \
            change above it: B minor over D, then the diminished seventh A# C# E G over D. Its \
            tune hardly moves either, one note repeated and falling a step, F# F# F# E, D D D \
            C#: a sigh. The waltz comes back, then the coda remembers both: the winds walk \
            down the scale of D major, two octaves, while the strings climb it, and the trio's \
            sighing rhythm comes back, F F E D, over the same held D, darkened by B flat and E \
            flat major chords, before the music breaks off into fragments, the violins alone, \
            a clarinet, and two last chords of D major.",
};

// Source: IMSLP 422976 (Parera, arranged for strings by Jeff Manookian, Windsor Editions, 2016); only the tune and its harmony are taken.
// The tune is the line the voice sings, in the singer's octave: the first violins, an octave
// lower where they double it above. In the instrumental bars it is the first violins' top
// line (an octave lower in bars 53-60, where it would pass C6), except bar 54, where it is
// the second violins' run (the top line of Esnaola's piano part) while the firsts hold D.
// The harmony is read from the four lower string parts. All 77 bars, in the score's order,
// with its metronome marks and fermatas; its triplets (bars 9-10 and 12-19) are exact.
// Cross-checked with a voice-and-piano edition engraved by Héctor M. Monacci (2001).
// PD: music by Blas Parera (d. 1840), words by Vicente López y Planes (d. 1856), arrangement Esnaola (d. 1878).
pub(super) const HIMNO_NACIONAL_ARGENTINO: Song = Song {
    name: "Himno Nacional Argentino",
    origin: "Blas Parera, 1813; arranged by Juan Pedro Esnaola, 1860",
    feel: "Anthem",
    genre: Genre::Classical,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    band: STRING_ORCHESTRA,
    style: "Classical cadence",
    parts: &[
        // Bars 1-11, ♩ = 76; a fermata on the rest that ends bar 11.
        Part::new(
            "Intro",
            "!f Bb4:8 D5:7 Bb4:1 | F5:10 -:2 Eb5:3 C5:1 | A4:4 A4 A4 A4 | A4:6 Bb4:1 C5:1 Bb4:4 -:4 | \
             -:4 !p D5:4 G5:4 F5:4 | E5:4 Eb5:4 C5:4 F5:4 | D5:4 Bb4 Eb5 D5 | C5:4 -:4 !mf Eb5:4 D5:4 | \
             C5:4 !p (3 -:2 F4:2 G4:2 (3 A4:2 Bb4:2 C5:2 (3 D5:2 Eb5:2 F5:2 | \
             F5:3 Eb5:1 (3 C5:2 C5:2 D5:2 F5:3 Eb5:1 !mf (3 C5:2 C5:2 D5:2 | F5:6 E5:2 Eb5:4 -:4",
            "I | V7 | V7 | V7 V7 I I... | I I vi VI7 | II V7 V V | I I V7 I | V V... V7 I | V V V7 I | V7 | \
             I I V7 V7...",
        )
        .with_tempo(&[fermata(172, 4)]),
        // Bars 12-24, ♩ = 116: the march, in triplet eighths, from bar 16 over a tonic
        // pedal; a fermata on the half note that starts bar 24, then "Oíd" comes in.
        Part::new(
            "March",
            "!p (3 D4:2 C4:2 Bb3:2 (3 Bb3:2 Bb3:2 Bb3:2 (3 Bb3:2 Bb3:2 Bb3:2 (3 Bb3:2 Bb3:2 Bb3:2 | \
             Bb3:4 !f D5:3 D5:1 F5:3 F5:1 D5:3 D5:1 | \
             !p (3 Eb4:2 D4:2 C4:2 (3 C4:2 C4:2 C4:2 (3 C4:2 C4:2 C4:2 (3 C4:2 C4:2 C4:2 | \
             C4:4 !f C5:3 C5:1 Eb5:3 Eb5:1 C5:3 C5:1 | \
             !p (3 D4:2 C4:2 Bb3:2 (3 Bb3:2 Bb3:2 Bb3:2 (3 D4:2 C4:2 Bb3:2 (3 Bb3:2 Bb3:2 Bb3:2 | \
             (3 Eb4:2 D4:2 C4:2 (3 C4:2 C4:2 C4:2 !< (3 Eb4:2 D4:2 C4:2 (3 C4:2 C4:2 C4:2 | \
             (3 F4:2 Eb4:2 D4:2 (3 D4:2 D4:2 D4:2 (3 F4:2 Eb4:2 D4:2 (3 D4:2 D4:2 D4:2 | \
             (3 G4:2 F4:2 Eb4:2 (3 Eb4:2 Eb4:2 Eb4:2 (3 A4:2 G4:2 F4:2 (3 F4:2 F4:2 F4:2 | \
             !f Bb4:4 -:6 F5:2. F5:1 Eb5:1 C5:1 A4:1 | Bb4:4 -:6 F5:2. F5:1 Eb5:1 C5:1 A4:1 | \
             Bb4:4 -:2 >D5:2 >Bb4:2 >D5:2 >Bb4:2 >D5:2 | >Bb4:8 >Bb3:6 >Bb3:2 | >Bb3:8 -:2 !mf F4:2 G4:2 A4:2",
            "I | I | V7 | V7 | I | V7 | I7 | IV V7 | I V7 | I V7 | I | I | I I I... V7",
        )
        .with_tempo(&[tempo(0, 116), fermata(192, 8)]),
        // Bars 25-32, ♩ = 76: "Oíd, mortales" ... "ved en trono a la noble igualdad",
        // then the strings' fanfare climbs into the minor.
        Part::new(
            "Verse",
            "!mf Bb4:4 Bb4:3 D5:1 C5:2 Bb4:2 A4:3 G4:1 | F4:6 D4:2 Bb3:2 -:2 D4:3 F4:1 | \
             Eb4:4 Eb4:3 G4:1 F4:4 G4:3 A4:1 | Bb4:10 -:2 Bb3:3 C4:1 | \
             D4:4 D4:3 Bb3:1 D4:2 F4:2 E4:3 G4:1 | G4:4 F4:4 -:4 F4:3 A4:1 | \
             G4:4 G4:3 Bb4:1 E4:4 E4:3 G4:1 | F4:4 F4:4 !ff F4:2 F5:2 G5:2 A5:2",
            "I | I | ii ii V7 ii | I | I I I II7 | V | vi II | V",
        )
        .with_tempo(&[tempo(0, 76)]),
        // Bars 33-44, in B flat minor: the strings' fanfare (bar 33), "Ya su trono
        // dignísimo abrieron las Provincias Unidas del Sud", a half cadence through a
        // German sixth (bar 37) to F, then "Y los libres del mundo responden" over an F
        // pedal, back to F for the major.
        Part::new(
            "Verse minor",
            "!ff Bb5:4 F5:3 F5:1 Gb5:2 F5:2 Eb5:2 Db5:2 | Db5:8 C5:2 !p F4:2 G4:2 A4:2 | \
             Bb4:4 F4:3 F4:1 Gb4:2 F4:2 Eb4:3 Db4:1 | Db4:8 C4:2 -:2 !f F4:3 F4:1 | \
             !< Bb4:4 C5:3 C5:1 Db5:4 E5:3 E5:1 | !ff F5:10 -:2 !p F4:3 F4:1 | \
             Bb4:4 Bb4:3 Bb4:1 C5:4 C5:3 C5:1 | Db5:4 Bb4:4 -:4 F4:3 F4:1 | \
             Bb4:4 Bb4:3 Bb4:1 C5:4 C5:3 C5:1 | Db5:4 Bb4:4 -:4 Bb4:3 Db5:1 | \
             Db5:2 C5:2 C5:2 Bb4:2 C5:2 Bb4:2 A4:2 Bb4:2 | F4:10 !p F4:2 G4:2 A4:2",
            "i i i iv | #iv°7 #iv°7 V V7 | i i i iv | #iv°7 #iv°7 V V | i V i bVI7 | V V V V... | \
             i V7 | i V7 | i V7 | i | i i i #iv°7 | V V V V7",
        )
        .in_key(ASharp, HarmonicMinor),
        // Bars 45-52: "Y los libres del mundo responden: ¡Al gran pueblo argentino,
        // salud!" twice, back in B flat major, each ending on a cadential six-four.
        Part::new(
            "Verse end",
            "!p Bb4:4 Bb4:3 A4:1 G4:2 F4:2 Eb4:2 D4:2 | F4:4 Eb4:2 -:2 !f -:4 !p Eb4:3 G4:1 | \
             F4:2 Bb4:2 D4:2 F4:2 F4:3 Eb4:1 D4:2 C4:2 | D4:10 F4:2 G4:2 A4:2 | \
             Bb4:4 Bb4:3 A4:1 G4:2 F4:2 Eb4:2 D4:2 | F4:4 Eb4:2 -:2 !f -:4 Eb4:3 G4:1 | \
             F4:2 Bb4:2 D5:2 F5:2 F5:3 Eb5:1 D5:2 C5:2 | Bb4:10 -:2 !ff F5:3 D5:1",
            "I I I V7 | VI7 ii ii. ii... | I I I V | I I I V7 | I I I V7 | VI7 ii ii. ii... | I I I V7 | I",
        ),
        // Bars 53-57, ♩ = 92: the strings alone, to a held B flat.
        Part::new(
            "Interlude",
            "!ff C5:6 D5:2 Eb5:2 D5:2 Eb5:2 C5:2 | \
             D5:1 Bb4:1 C5:1 D5:1 Eb5:1 F5:1 G5:1 A5:1 Bb5:1 A5:1 G5:1 F5:1 F5:3 D5:1 | \
             C5:6 D5:2 Eb5:2 D5:2 Eb5:2 C5:2 | Bb4:4 -:4 Bb3:4 -:4 | Bb3:16",
            "V V V7 V7 | I | V V V7 V7 | I I... I I... | I..",
        )
        .with_tempo(&[tempo(0, 92), fermata(64, 16)]),
        // Bars 58-65, ♩ = 132: three bars of the strings alone, then the chorus, "Sean
        // eternos los laureles que supimos conseguir".
        Part::new(
            "Chorus",
            "!f Bb3:4 Bb3:1 A3:1 Bb3:1 A3:1 Bb3:1 A3:1 Bb3:1 A3:1 Bb3:1 C4:1 D4:1 Eb4:1 | \
             >F4:4 F4:1 E4:1 F4:1 E4:1 F4:1 E4:1 F4:1 E4:1 F4:1 G4:1 A4:1 F4:1 | \
             >Bb4:4 Bb4:1 A4:1 Bb4:1 C5:1 D5:2 D5:1 Eb5:1 C5:2 C5:1 D5:1 | \
             Bb4:2 -:2 >Bb3:2 -:2 >Bb3:2 -:2 !ff F4:3 F4:1 | Bb4:4 F4:4 D4:4 F4:4 | \
             Bb4:10 -:2 F4:3 F4:1 | Bb4:4 F4:4 D4:4 F4:4 | Bb4:10 -:2 !f Bb4:3 D5:1",
            "I | V | I I I V7 | I. I. I. V | I | I | I | I",
        )
        .with_tempo(&[tempo(0, 132)]),
        // Bars 66-77: "que supimos conseguir" again broadens (♩ = 76, a fermata), then
        // "Coronados de gloria vivamos" (♩ = 60, a fermata on the C of bar 69), then
        // ♩ = 132 for "O juremos con gloria morir" three times, to the held last chord.
        Part::new(
            "Chorus end",
            "!f C5:4 Bb4:4 A4:4 G4:4 | F4:12 !p Bb4:3 A4:1 | A4:4 G4:3 F4:1 F4:2 Eb4:2 Eb4:3 D4:1 | \
             D4:3 F4:1 E4:1 F4:1 G4:1 F4:1 C4:2 !f F4:2 G4:2 A4:2 | \
             Bb4:4 Bb4:2 Bb4:2 A4:4 A4:2 A4:2 | Bb4:4 -:4 Eb5:6 Eb5:2 | \
             Eb5:2 D5:2 C5:2 Bb4:2 F4:4 A4:2 A4:2 | Bb4:4 -:4 Eb5:6 Eb5:2 | \
             Eb5:2 D5:2 C5:2 Bb4:2 F4:4 A4:2 A4:2 | Bb4:4 -:2 >D5:2 >Bb4:2 >D5:2 >Bb4:2 >D5:2 | \
             >Bb4:8 >Bb4:6 >Bb4:2 | >Bb4:16",
            "V I V II | V | Imaj7 IV I V7 | I I V V | I V7 | I I... IV IV | IV I V V7 | I I... IV IV | \
             IV I V V7 | I | I | I..",
        )
        .with_tempo(&[
            tempo(0, 76),
            fermata(16, 12),
            tempo(32, 60),
            fermata(56, 2),
            tempo(64, 132),
            fermata(176, 16),
        ]),
    ],
    form: &[
        "Intro",
        "March",
        "Verse",
        "Verse minor",
        "Verse end",
        "Interlude",
        "Chorus",
        "Chorus end",
    ],
    drums_from: 0,
    about: "The anthem as the strings play it, in B flat major: a slow introduction, then \
            a march in triplets that settles over a held B flat, and the verse, which turns \
            to B flat minor and reaches F through diminished sevenths and a German sixth \
            before coming back to the major. The chorus is much faster, broadens for \
            \"Coronados de gloria vivamos\", and ends with \"O juremos con gloria morir\" \
            three times, the last two going from IV to V.",
};

// Boléro is built of a few bars repeated many times, so its pieces are
// macros rather than consts: `concat!` joins only literals, and each of
// these expands to one, so every statement's accompaniment and drums are
// put together from the same two bars instead of being written out again.

/// The two bars `$bars` names, twice.
macro_rules! bolero_two {
    ($bars:ident) => {
        concat!($bars!(), " | ", $bars!())
    };
}

/// The two bars `$bars` names, four times.
macro_rules! bolero_four {
    ($bars:ident) => {
        concat!(bolero_two!($bars), " | ", bolero_two!($bars))
    };
}

/// The two bars `$bars` names, eight times.
macro_rules! bolero_eight {
    ($bars:ident) => {
        concat!(bolero_four!($bars), " | ", bolero_four!($bars))
    };
}

/// A whole statement of the tune: eighteen bars, nine times two.
macro_rules! bolero_nine {
    ($bars:ident) => {
        concat!(bolero_eight!($bars), " | ", $bars!())
    };
}

/// Two bars of the snare drum's ostinato, the rhythm under the whole
/// piece: an eighth, a triplet of sixteenths, an eighth, a triplet, then
/// two eighths; the second bar ends with two more triplets instead.
macro_rules! bolero_snare {
    () => {
        "x:2 (3 x:1 x x x:2 (3 x:1 x x x:2 x:2 | \
         x:2 (3 x:1 x x x:2 (3 x:1 x x (3 x:1 x x (3 x:1 x x"
    };
}

/// Two bars of silence.
macro_rules! bolero_rest {
    () => {
        "-:12 | -:12"
    };
}

/// The snare drum's two bars as the orchestra doubles them, one chord on
/// each of the six beats.
macro_rules! bolero_ostinato {
    ($a:literal, $b:literal, $c:literal, $d:literal, $e:literal, $f:literal) => {
        concat!(
            $a, ":2 (3 ", $a, ":1 ", $a, " ", $a, " ", $b, ":2 (3 ", $b, ":1 ", $b, " ", $b, " ",
            $c, ":2 ", $c, " | ", $d, ":2 (3 ", $d, ":1 ", $d, " ", $d, " ", $e, ":2 (3 ", $e,
            ":1 ", $e, " ", $e, " (3 ", $f, ":1 ", $f, " ", $f, " (3 ", $f, " ", $f, " ", $f
        )
    };
}

// The accompaniment, two bars at a time, each named after the bar where
// it is first heard: `bolero_beats_*` is everything but the snare drum's
// rhythm (the pizzicato, the harps, the held chords, the bass and the
// timpani), `bolero_rhythm_*` the instruments that double that rhythm.
// A chord keeps its lowest and highest notes and every other pitch once,
// at the highest of its octaves.

/// Two bars from bar 5: violas and cellos.
macro_rules! bolero_beats_1 {
    () => {
        "C3+G3:4 G3+G4 G2+G3 | C3+G3 G3+G4 G2+G3:2 G3"
    };
}

/// Two bars from bar 21: violas and cellos.
macro_rules! bolero_beats_2 {
    () => {
        "C3+G3:4 G3+G4 G2+C3+G3 | C3+G3 G3+G4 G2+C3+G3:2 G3"
    };
}

/// The ostinato from bar 21: flutes.
macro_rules! bolero_rhythm_1 {
    () => {
        "-:2 (3 G4:1 G4 G4 G4:2 (3 G4:1 G4 G4 G4:2 G4 | \
         G4 (3 G4:1 G4 G4 G4:2 (3 G4:1 G4 G4 (3 G4 G4 G4 (3 G4 G4 G4"
    };
}

/// The ostinato from bar 23: flutes.
macro_rules! bolero_rhythm_2 {
    () => {
        bolero_ostinato!("G4", "G4", "G4", "G4", "G4", "G4")
    };
}

/// Two bars from bar 39: harp, violas and cellos.
macro_rules! bolero_beats_3 {
    () => {
        "C3+G3:4 D3+E3+G4 G2+C3+D3+G3 | C3+G3 D3+E3+G4 G2+C3+D3+G3:2 G3"
    };
}

/// Two bars from bar 49: harp, violas and cellos.
macro_rules! bolero_beats_4 {
    () => {
        "C3+G3:4 C3+D3+G4 G2+B2+C3+G3 | C3+G3 C3+D3+G4 G2+B2+C3+G3:2 G3"
    };
}

/// Two bars from bar 53: harp, violas and cellos.
macro_rules! bolero_beats_5 {
    () => {
        "C3+G3:4 D3+E3+G4 G2+C3+D3+G3 | C3+G3 C3+D3+G4 G2+Bb2+C3+G3:2 G3"
    };
}

/// Two bars from bar 55: harp, violas and cellos.
macro_rules! bolero_beats_6 {
    () => {
        "C3+G3:4 Bb2+C3+G4 G2+Ab2+C3+G3 | C3+G3 Ab2+C3+G4 G2+C3+G3:2 G3"
    };
}

/// Two bars from bar 57: harp, violas and cellos.
macro_rules! bolero_beats_7 {
    () => {
        "C2+G3:4 G3+C4+D4+E4+G4 G2+Bb3+C4+D4 | C2+G3 G3+C4+D4+E4+G4 G2+Bb3+C4+D4:2 G3"
    };
}

/// Two bars from bar 67: harp, violas and cellos.
macro_rules! bolero_beats_8 {
    () => {
        "C2+G3:4 G3+C4+D4+F4+G4 G2+B3+C4+D4 | C2+G3 G3+C4+D4+F4+G4 G2+B3+C4+D4:2 G3"
    };
}

/// Two bars from bar 71: harp, violas and cellos.
macro_rules! bolero_beats_9 {
    () => {
        "C2+G3:4 G3+C4+D4+E4+G4 G2+Ab3+C4+D4 | C2+G3 G3+Ab3+C4+D4+G4 G2+Bb3+C4:2 G3"
    };
}

/// Two bars from bar 73: harp, violas and cellos.
macro_rules! bolero_beats_10 {
    () => {
        "C2+G3:4 G3+Bb3+C4+G4 G2+Ab3+C4 | C2+G3 G3+Ab3+C4+G4 G2+C4:2 G3"
    };
}

/// Two bars from bar 75: flutes, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_11 {
    () => {
        "C2+E3+G4:4 G3+C4+D4+E4+G4 G2+B3+C4+D4+G4 | \
         C2+E3+G3 G3+C4+D4+E4+G4 G2+B3+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 77: second violins, violas, cellos and double basses.
macro_rules! bolero_beats_12 {
    () => {
        "C2+E3+G3:4 G3+C4+D4+E4+G4 G2+B3+C4+D4+G4 | \
         C2+E3+G3 G3+C4+D4+E4+G4 G2+B3+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 81: second violins, violas, cellos and double basses.
macro_rules! bolero_beats_13 {
    () => {
        "C2+E3+G3:4 G3+C4+D4+E4+G4 G2+B3+C4+D4+G4 | \
         C2+E3+G3 G3+C4+D4+E4+G4 G2+B3+C4+E4+G4:2 G3"
    };
}

/// Two bars from bar 83: second violins, violas, cellos and double basses.
macro_rules! bolero_beats_14 {
    () => {
        "C2+D3+G3:4 G3+C4+D4+F4+G4 G2+B3+C4+D4+G4 | \
         C2+D3+G3 G3+C4+D4+F4+G4 G2+B3+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 91: second violins, violas, cellos and double basses.
macro_rules! bolero_beats_15 {
    () => {
        "C2+D3+G3:4 G3+C4+D4+F4+G4 G2+B3+C4+D4+G4 | C2+E3+G3 G3+C4+E4+G4 G2+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 93: bassoons, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_16 {
    () => {
        "C2+E3+G4:4 G3+C5+D5+E5 G2+B4+C5+D5 | C2+E3+G3 G3+C5+D5+E5 G2+B4+C5+D5:2 G3"
    };
}

/// Two bars from bar 95: first violins, violas, cellos and double basses.
macro_rules! bolero_beats_17 {
    () => {
        "C2+E3+G3:4 G3+C5+D5+E5 G2+B4+C5+D5 | C2+E3+G3 G3+C5+D5+E5 G2+B4+C5+D5:2 G3"
    };
}

/// Two bars from bar 99: first violins, violas, cellos and double basses.
macro_rules! bolero_beats_18 {
    () => {
        "C2+E3+G3:4 G3+C5+D5+E5 G2+B4+C5+D5 | C2+E3+G3 G3+C5+D5+E5 G2+B4+C5+E5:2 G3"
    };
}

/// Two bars from bar 101: first violins, violas, cellos and double basses.
macro_rules! bolero_beats_19 {
    () => {
        "C2+D3+G3:4 G3+C5+D5+F5 G2+B4+C5+D5 | C2+D3+G3 G3+C5+D5+F5 G2+B4+C5+D5:2 G3"
    };
}

/// Two bars from bar 109: first violins, violas, cellos and double basses.
macro_rules! bolero_beats_20 {
    () => {
        "C2+D3+G3:4 G3+C5+D5+F5 G2+B4+C5+D5 | C2+E3+G3 G3+C5+D5+E5 G2+C5+D5:2 G3"
    };
}

/// Two bars from bar 111: flutes, horns, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_21 {
    () => {
        "C2+E3+G4:4 G3+C4+D4+E4 G1+B3+C4+D4 | C2+E3+G3 G3+C4+D4+E4 G1+B3+C4+D4:2 G3"
    };
}

/// Two bars from bar 113: flutes, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_22 {
    () => {
        "C2+E3+G3:4 G3+C4+D4+E4 G1+Bb3+C4+D4 | C2+E3+G3 G3+C4+D4+E4 G1+Bb3+C4+D4:2 G3"
    };
}

/// Two bars from bar 121: flutes, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_23 {
    () => {
        "C2+D3+G3:4 G3+C4+D4+F4 G1+B3+C4+D4 | C2+D3+G3 G3+C4+D4+F4 G1+B3+C4+D4:2 G3"
    };
}

/// Two bars from bar 125: flutes, clarinets, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_24 {
    () => {
        "C2+E3+G3:4 G3+C4+D4+E4 G1+Ab3+C4+D4 | C2+D3+G3 G3+Ab3+C4+D4 G1+Bb3+C4:2 G3"
    };
}

/// Two bars from bar 127: flutes, clarinets, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_25 {
    () => {
        "C2+G3:4 G3+Bb3+C4 G1+Ab3+C4 | C2+G3 G3+Ab3+C4 G1+C4:2 G3"
    };
}

/// Two bars from bar 129: oboes, English horn, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_26 {
    () => {
        "C2+E3+G3:4 G3+C5+D5+E5 G1+Bb4+C5+D5 | C2+E3+G3 G3+C5+D5+E5 G1+Bb4+C5+D5:2 G3"
    };
}

/// Two bars from bar 139: oboes, English horn, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_27 {
    () => {
        "C2+E3+G3:4 G3+C5+D5+F5 G1+B4+C5+D5 | C2+E3+G3 G3+C5+D5+F5 G1+B4+C5+D5:2 G3"
    };
}

/// Two bars from bar 143: oboes, English horn, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_28 {
    () => {
        "C2+E3+G3:4 G3+C5+D5+E5 G1+Ab4+C5+D5 | C2+D3+G3 G3+Ab4+C5+D5 G1+Bb4+C5:2 G3"
    };
}

/// Two bars from bar 145: oboes, English horn, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_29 {
    () => {
        "C2+G3:4 G3+Bb4+C5 G1+Ab4+C5 | C2+G3 G3+Ab4+C5 G1+C5:2 G3"
    };
}

/// Two bars from bar 147: bass clarinet, bassoons, trumpets, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_30 {
    () => {
        "C2+E3+G4:4 G3+C4+D4+E4+G4 G1+B3+C4+D4+G4 | \
         C2+E3+G3 G3+C4+D4+E4+G4 G1+B3+C4+D4+G4:2 G3"
    };
}

/// The ostinato from bar 147: flutes and horns.
macro_rules! bolero_rhythm_3 {
    () => {
        "-:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 G3+G4 | \
         G3+G4 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4"
    };
}

/// Two bars from bar 149: bass clarinet, bassoons, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_31 {
    () => {
        "C2+E3+G3:4 G3+C4+D4+E4+G4 G1+B3+C4+D4+G4 | \
         C2+E3+G3 G3+C4+D4+E4+G4 G1+B3+C4+D4+G4:2 G3"
    };
}

/// The ostinato from bar 149: flutes and horns.
macro_rules! bolero_rhythm_4 {
    () => {
        bolero_ostinato!("G3+G4", "G3+G4", "G3+G4", "G3+G4", "G3+G4", "G3+G4")
    };
}

/// Two bars from bar 153: bass clarinet, bassoons, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_32 {
    () => {
        "C2+E3+G3:4 G3+C4+D4+E4+G4 G1+B3+C4+D4+G4 | \
         C2+E3+G3 G3+C4+D4+E4+G4 G1+B3+C4+E4+G4:2 G3"
    };
}

/// Two bars from bar 155: bass clarinet, bassoons, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_33 {
    () => {
        "C2+D3+G3:4 G3+C4+D4+F4+G4 G1+B3+C4+D4+G4 | \
         C2+D3+G3 G3+C4+D4+F4+G4 G1+B3+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 163: bass clarinet, bassoons, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_34 {
    () => {
        "C2+D3+G3:4 G3+C4+D4+F4+G4 G1+B3+C4+D4+G4 | C2+E3+G3 G3+C4+E4+G4 G1+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 165: flutes, bass clarinet, bassoons, horns, trumpets, harp, first violins, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_35 {
    () => {
        "C2+E3+G4:4 G3+C5+E5+G5 G1+B4+C5+D5:2 G3+C4+G4 | \
         C2+G2+E3:4 G3+C5+E5+G5 G1+B4+C5+D5:2 G3"
    };
}

/// The ostinato from bar 165: horns, trumpets, second violins and violas.
macro_rules! bolero_rhythm_5 {
    () => {
        "-:2 (3 C3+G4:1 G3+G4 G3+C5 G3+C5+G5:2 (3 C3+G4:1 G3+G4 G3+C5 G3+C5+G5:2 G3+C5+G5 | \
         C3+G5 (3 C3+G4:1 G3+G4 G3+C5 G3+C5+G5:2 (3 C3+G4:1 G3+G4 G3+C5 (3 C3+G5 G3+G4 G3+C5 (3 C3+G5 G3+G4 G3+C5"
    };
}

/// The ostinato from bar 167: horns, trumpets, second violins and violas.
macro_rules! bolero_rhythm_6 {
    () => {
        "G3+C5+G5:2 (3 C3+G4:1 G3+G4 G3+C5 G3+C5+G5:2 (3 C3+G4:1 G3+G4 G3+C5 G3+C5+G5:2 G3+C5+G5 | \
         C3+G5 (3 C3+G4:1 G3+G4 G3+C5 G3+C5+G5:2 (3 C3+G4:1 G3+G4 G3+C5 (3 C3+G5 G3+G4 G3+C5 (3 C3+G5 G3+G4 G3+C5"
    };
}

/// Two bars from bar 171: bass clarinet, bassoons, trumpets, harp, first violins, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_36 {
    () => {
        "C2+E3+G4:4 G3+C5+E5+G5 G1+B4+C5+D5:2 G3+C4+G4 | \
         C2+G2+E3:4 G3+C5+E5+G5 G1+B4+C5+E5:2 G3"
    };
}

/// Two bars from bar 173: bass clarinet, bassoons, trumpets, harp, first violins, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_37 {
    () => {
        "C2+D3+G4:4 G3+C5+D5+F5+G5 G1+C4+B4+D5:2 G3+C4+G4 | \
         C2+G2+D3:4 C4+D5+F5+G5 G1+C4+B4+D5:2 G3"
    };
}

/// Two bars from bar 181: bass clarinet, bassoons, trumpets, harp, first violins, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_38 {
    () => {
        "C2+D3+G4:4 G3+C5+D5+F5+G5 G1+C4+B4+D5:2 G3+C4+G4 | \
         C2+G2+E3:4 G3+C5+D5+E5+G5 G1+C5+D5:2 G3"
    };
}

/// Two bars from bar 183: clarinets, bass clarinet, contrabassoon, horns, trumpets, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_39 {
    () => {
        "C2+E3+G5:4 G3+C4+D4+E4+G4 G1+B3+C4+D4+G4 | \
         C2+G2+E3 G3+C4+D4+E4+G4 G1+B3+C4+D4+G4:2 G2"
    };
}

/// The ostinato from bar 183: flutes, horns and violas.
macro_rules! bolero_rhythm_7 {
    () => {
        "C3+C5:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 G3+G4 | \
         C3+G4 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4"
    };
}

/// Two bars from bar 185: clarinets, bass clarinet, contrabassoon, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_40 {
    () => {
        "C2+G2+E3:4 G3+C4+D4+E4+G4 G1+Bb3+C4+D4+G4 | \
         C2+G2+E3 G3+C4+D4+E4+G4 G1+Bb3+C4+D4+G4:2 G2"
    };
}

/// The ostinato from bar 185: flutes, horns and violas.
macro_rules! bolero_rhythm_8 {
    () => {
        "C3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 G3+G4 | \
         C3+G4 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4"
    };
}

/// Two bars from bar 193: clarinets, bass clarinet, contrabassoon, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_41 {
    () => {
        "C2+G2+D3:4 G3+C4+D4+E4+F4+G4 G1+B3+C4+D4+G4 | \
         C2+G2+D3 G3+C4+D4+E4+F4+G4 G1+B3+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 195: clarinets, bass clarinet, contrabassoon, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_42 {
    () => {
        "C2+G2+E3:4 G3+C4+D4+E4+G4 G1+Bb3+C4+D4+G4 | \
         C2+G2+E3 G3+C4+D4+E4+G4 G1+Bb3+C4+D4+G4:2 G3"
    };
}

/// Two bars from bar 197: clarinets, bass clarinet, contrabassoon, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_43 {
    () => {
        "C2+G2+E3:4 G3+C4+D4+E4+G4 G1+Ab3+C4+D4+G4 | \
         C2+G2+D3 G3+Ab3+C4+D4+G4 G1+Bb3+C4+G4:2 G3"
    };
}

/// Two bars from bar 199: clarinets, bass clarinet, contrabassoon, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_44 {
    () => {
        "C2+G3:4 G3+Bb3+C4+G4 G1+Ab3+C4+G4 | C2+G3 G3+Ab3+C4+G4 G1+C4+G4:2 G3"
    };
}

/// Two bars from bar 201: flutes, bass clarinet, bassoons, contrabassoon, horns, harp, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_45 {
    () => {
        "C2+E3+G4:4 C3+G4+D5+E5 G1+Bb4+C5+D5 | C2+E3+G4 C3+G4+D5+E5 G1+Bb4+C5+D5:2 G2"
    };
}

/// Two bars from bar 207: flutes, bass clarinet, bassoons, contrabassoon, harp, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_46 {
    () => {
        "C2+G4+E6+Bb6:2 E6+Bb6 C3+G4+D5+E5 E6+Bb6 G1+C5+D5+E6+Bb6 E6+Bb6 | \
         C2+E3+G4:4 C3+G4+D5+E5 G1+Bb4+C5+D5:2 G2"
    };
}

/// Two bars from bar 211: bass clarinet, bassoons, contrabassoon, harp, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_47 {
    () => {
        "C2+D3+G4:4 C3+G4+D5+F5 G1+B4+C5+D5 | C2+D3+G4 C3+G4+D5+F5 G1+B4+C5+D5:2 G2"
    };
}

/// Two bars from bar 215: bass clarinet, bassoons, contrabassoon, harp, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_48 {
    () => {
        "C2+E3+G4:4 C3+G4+D5+E5 G1+Ab4+C5+D5:2 G2 | C2+D3+G4:4 C3+G4+Ab4+D5 G1+Bb4+C5:2 G2"
    };
}

/// Two bars from bar 217: bass clarinet, bassoons, contrabassoon, harp, first violins, violas, cellos and double basses.
macro_rules! bolero_beats_49 {
    () => {
        "C2+G4:4 C3+G4+Bb4+C5 G1+Ab4+C5 | C2+G4 C3+G4+Ab4+C5 G1+C5:2 G2"
    };
}

/// Two bars from bar 219: oboes, clarinets, bassoons, contrabassoon, horns, trumpets, timpani, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_50 {
    () => {
        "C2+E3+G4:4 C3+D5+E5+G5 G1+B4+C5+D5+G5 | C2+E3+G3 C3+D5+E5+G5 G1+B4+C5+D5+G5:2 G2+G3"
    };
}

/// Two bars from bar 221: bassoons, contrabassoon, horns, timpani, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_51 {
    () => {
        "C2+E3+G3:4 C3+D5+E5+G5 G1+B4+C5+D5+G5 | C2+E3+G3 C3+D5+E5+G5 G1+B4+C5+D5+G5:2 G2+G3"
    };
}

/// Two bars from bar 225: bassoons, contrabassoon, horns, timpani, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_52 {
    () => {
        "C2+E3+G3:4 C3+D5+E5+G5 G1+B4+C5+D5+G5 | \
         C2+E3+G4 C3+D5+E5+G5 G1+D4+B4+C5+E5+G5:2 G2+G3"
    };
}

/// Two bars from bar 227: bassoons, contrabassoon, horns, timpani, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_53 {
    () => {
        "C2+D3+G4:4 C3+D5+F5+G5 G1+B4+C5+D5+G5 | C2+D3+G4 C3+D5+F5+G5 G1+B4+C5+D5+G5:2 G2+G3"
    };
}

/// Two bars from bar 235: bassoons, contrabassoon, horns, timpani, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_54 {
    () => {
        "C2+D3+G4:4 C3+D5+F5+G5 G1+B4+C5+D5+G5 | C2+E3+G4 C3+D4+E5+G5 G1+B3+C5+D5+G5:2 G2+G3"
    };
}

/// Two bars from bar 237: clarinets, bass clarinet, bassoons, contrabassoon, soprano saxophone, horns, timpani, harp, violas, cellos and double basses.
macro_rules! bolero_beats_55 {
    () => {
        "C2+E3+G4:4 C3+D5+E5+G5 G1+B4+C5+D5+G5 | C2+E3+G4 C3+D5+E5+G5 G1+B4+C5+D5+G5:2 G2"
    };
}

/// Two bars from bar 241: bass clarinet, bassoons, contrabassoon, soprano saxophone, horns, timpani, harp, violas, cellos and double basses.
macro_rules! bolero_beats_56 {
    () => {
        "C2+E3+G4:4 C3+D5+E5+G5 G1+B4+C5+D5+G5 | C2+E3+G4 C3+D5+E5+G5 G1+B4+C5+D5+G5:2 G2+G3"
    };
}

/// Two bars from bar 243: bass clarinet, bassoons, contrabassoon, soprano saxophone, horns, timpani, harp, violas, cellos and double basses.
macro_rules! bolero_beats_57 {
    () => {
        "C2+E3+G4:4 C3+D5+E5+G5 G1+B4+C5+D5+G5 | \
         C2+E3+G4 C3+D5+E5+G5 G1+D4+B4+C5+E5+G5:2 G2+G3"
    };
}

/// Two bars from bar 253: bass clarinet, bassoons, contrabassoon, soprano saxophone, horns, timpani, harp, violas, cellos and double basses.
macro_rules! bolero_beats_58 {
    () => {
        "C2+D3+G4:4 C3+D5+F5+G5 G1+B4+C5+D5+G5 | C2+E3+G4 C3+D5+E5+G5 G1+B4+C5+D5+G5:2 G2+G3"
    };
}

/// Two bars from bar 255: flutes, oboes, clarinets, bassoons, contrabassoon, soprano saxophone, tenor saxophone, horns, trombones, tuba, timpani, harp, violas, cellos and double basses.
macro_rules! bolero_beats_59 {
    () => {
        "C2+E5+G5:4 C3+D5+E5+G5 G1+B4+C5+D5+G5 | C2+E4+G4 C3+D5+E5+G5 G1+B4+C5+D5+G5:2 G2+G3"
    };
}

/// The ostinato from bar 255: horns.
macro_rules! bolero_rhythm_9 {
    () => {
        "C4+E4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 G3+G4 | \
         G3+G4 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4 (3 G3+G4 G3+G4 G3+G4"
    };
}

/// Two bars from bar 257: clarinets, bassoons, contrabassoon, soprano saxophone, tenor saxophone, horns, trombones, tuba, timpani, harp, violas, cellos and double basses.
macro_rules! bolero_beats_60 {
    () => {
        "C2+E4+G4:4 C3+D5+E5+G5 G1+Bb4+C5+D5+G5 | \
         C2+E4+G4 C3+D5+E5+G5 G1+Bb4+C5+D5+G5:2 G2+G3"
    };
}

/// Two bars from bar 265: clarinets, bassoons, contrabassoon, soprano saxophone, tenor saxophone, horns, trombones, tuba, timpani, harp, violas, cellos and double basses.
macro_rules! bolero_beats_61 {
    () => {
        "C2+D3+F4+G4:4 C3+D5+F5+G5 G1+B4+C5+D5+G5 | \
         C2+D4+F4+G4 C3+D5+F5+G5 G1+B4+C5+D5+G5:2 G2+A2+G3"
    };
}

/// Two bars from bar 269: clarinets, bassoons, contrabassoon, soprano saxophone, tenor saxophone, horns, trombones, tuba, timpani, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_62 {
    () => {
        "C2+E4+G4:4 C3+D5+E5+G5 G1+Ab4+C5+D5+G5 | \
         C2+G2+D3+C4 C3+Ab4+D5+G5 G1+Bb4+C5+G5:2 G2+G3"
    };
}

/// Two bars from bar 271: clarinets, bassoons, contrabassoon, soprano saxophone, tenor saxophone, horns, trombones, tuba, timpani, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_63 {
    () => {
        "C2+G3+C4:4 C3+Bb4+G5 G1+Ab4+C5+G5 | C2+G3+C4 C3+Ab4+G5 G1+C5+G5:2 G2+G3"
    };
}

/// Two bars from bar 273: bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, second violins, cellos and double basses.
macro_rules! bolero_beats_64 {
    () => {
        "C2+G3+E4:4 C3+D5+E5+G5 G1+B3+Bb4+C5+D5+G5 | \
         C2+G3+E4 C3+D5+E5+G5 G1+B3+Bb4+C5+D5+G5:2 C3"
    };
}

/// Two bars from bar 275: bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_65 {
    () => {
        "C2+G3+E5:4 C3+D4+E5+G5:1 D5 C5 Bb4 G1+C4+Bb4+D5+E5+G5 F5 D5 C5 | \
         C2+G3+E5:2 D5:1 C5 C3+D4+E5+G5:4 G1+C4+Bb4+D5+G5:1 E5 G2+C3+D5 C5"
    };
}

/// Two bars from bar 277: bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_66 {
    () => {
        "C2+G3+E4:4 C3+D4+E5+G5:1 Bb4 G4 F4 G1+C4+Bb4+D5+G5:4 | \
         C2+E4+G4 C3+D4+E5+G5:2 D5 G1+C4+Bb4+D5+E5+G5 G2+C3+E5"
    };
}

/// Two bars from bar 279: flutes, bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_67 {
    () => {
        "C2+G4+E6+Bb6:2 E5+Bb6 C3+D4+E5+G5 E5+Bb6 G1+C4+D5+E6+Bb6 E5+Bb6 | \
         (3 C2+G4+E5 E5 E5 C3+D4+E5+G5 Eb5:1 Db5 G1+C4+Bb4+D5+E5+G5:2 G2+C3+Eb5:1 Db5"
    };
}

/// Two bars from bar 281: bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_68 {
    () => {
        "C2+G4+E5:1 Eb5 Db5 C5 C3+Bb4+D5+E5+G5 Ab4 G4:2 G1+Bb4+C5+D5+G5:4 | \
         C2+E4+G4 C3+D5+E5+G5 G1+Bb4+C5+D5+G5:2 G2+C3"
    };
}

/// Two bars from bar 283: bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_69 {
    () => {
        "C2+G3+F4:4 C3+D5+F5+G5:2 G4 G1+F4+B4+C5+D5+G5:1 G4 A4:2 | \
         C2+G3+D4+F4:4 C3+D5+F5+G5 G1+B4+C5+D5+G5:2 G2+C3"
    };
}

/// The ostinato from bar 283: horns and violas.
macro_rules! bolero_rhythm_10 {
    () => {
        "G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 (3 G3+G4:1 G3+G4 G3+G4 G3+G4:2 G3+G4 | \
         G3+G4 (3 G3+G4:1 G3+G4 G3+G4 G3+B4:2 (3 G3+D5:1 G3+G4 G3+G4 (3 G3+Ab4 G3+G4 G3+B4 (3 G3+G4 G3+G4 G3+G4"
    };
}

/// Two bars from bar 285: bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_70 {
    () => {
        "C2+E4+G4:1 A4 G4 E4 C3+D5+E5+G5:2 D4 G1+Bb4+C5+D5+G5:4 | \
         C2+G3+E4:1 E4 D4 E4 C3+D5+E5+G5 A4 G4 E4 G1+Bb4+C5+D5+G5 E4 G2+C3+D4 C4"
    };
}

/// Two bars from bar 287: bass clarinet, bassoons, contrabassoon, tenor saxophone, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_71 {
    () => {
        "C2+G3+E4:2 C4 C3+D5+E5+G5 F3+Ab3 G1+Ab4+C5+D5+G5:4 | \
         C2+G3+D4 C3+Ab4+D5+G5:1 G3+Bb3 F3+Ab3 Eb3+G3 G1+Bb4+C5+G5:2 G2+C3"
    };
}

/// Two bars from bar 289: bass clarinet, bassoons, contrabassoon, tenor saxophone, trumpets, trombones, tuba, timpani, harp, violas and double basses.
macro_rules! bolero_beats_72 {
    () => {
        "C2+G3+Bb3+C4:4 C3+F3+Ab3+Bb4+G5:1 Eb3+G3 F3+Ab3 Db3+F3 G1+Ab4+C5+G5:4 | \
         C2+G3+Ab3+C4 C3+Ab4+G5:1 Db3+F3 Eb3+G3 Db3+F3 G1+F3+Ab3+B3+C5+G5 Eb3+G3 G2+C3+Db3+F3 C3+E3"
    };
}

/// Two bars from bar 291: bass clarinet, bassoons, contrabassoon, trumpets, trombones, tuba, timpani, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_73 {
    () => {
        "C2+G3+C4:4 C2+D5+E5+G5 G1+B4+C5+D5+G5:2 C2+B4+D5+G5 | \
         C2+D4+E4+G5:4 C2+D5+E5+G5 G1+B4+C5+D5+G5:2 G1+C4+D4+B4+G5"
    };
}

/// The ostinato from bar 291: piccolo, flutes, oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_11 {
    () => {
        "G3+A3+C4+G4:2 (3 C2+D5+E6+G6:1 C2+E6+G6 C2+D5+E6+G6 C2+D5+E6+G6:2 (3 C2+D5+E6+G6:1 C2+E6+G6 C2+D5+E6+G6 C2+B5+D6+G6:2 C2+B5+D6+G6 | \
         C2+D5+E6+G6 (3 C2+D5+E6+G6:1 C2+E6+G6 C2+D5+E6+G6 C2+D5+E6+G6:2 (3 C2+D5+E6+G6:1 C2+E6+G6 C2+D5+E6+G6 (3 C2+B5+D6+G6 C2+E5+B5+D6+G6 C2+B5+D6+G6 (3 C2+B5+D6+G6 C2+E5+B5+D6+G6 C2+B5+D6+G6"
    };
}

/// Two bars from bar 293: bass clarinet, bassoons, contrabassoon, trombones, tuba, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_74 {
    () => {
        "C2+D4+E5+G5:4 C2+D4+E5+G5 G1+C4+B4+D5+G5:2 C2+B4+D5+G5 | \
         C2+D4+E4+G5:4 C2+D4+E4+G5 G1+C4+D4+B4+G5:2 G1+C4+D4+B4+G5"
    };
}

/// The ostinato from bar 293: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_12 {
    () => {
        "C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 C2+B4+D5+G5:2 C2+B4+D5+G5 | \
         C2+D5+E5+G5 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 (3 C2+B4+D5+G5 C2+B4+D5+E5+G5 C2+B4+D5+G5 (3 C2+B4+D5+G5 C2+B4+D5+E5+G5 C2+B4+D5+G5"
    };
}

/// Two bars from bar 297: bass clarinet, bassoons, contrabassoon, trombones, tuba, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_75 {
    () => {
        "C2+D4+E5+G5:4 C2+D4+E5+G5 G1+C4+B4+D5+G5:2 C2+B4+D5+G5 | \
         C2+D4+E4+G5:4 C2+D4+E4+G5 G1+C4+E4+B4+G5:2 G1+C4+E4+B4+G5"
    };
}

/// The ostinato from bar 297: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_13 {
    () => {
        "C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 C2+B4+D5+G5:2 C2+B4+D5+G5 | \
         C2+D5+E5+G5 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 (3 C2+B4+Db5+E5+G5 C2+B4+D5+E5+G5 C2+B4+Db5+E5+G5 (3 C2+B4+Db5+E5+G5 C2+B4+D5+E5+G5 C2+B4+Db5+E5+G5"
    };
}

/// Two bars from bar 299: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_76 {
    () => {
        "C2+D5+F5+G5:4 C2+D5+F5+G5 G1+C4+B4+D5+G5:2 C2+B4+D5+G5 | \
         C2+F4+D5+G5:4 C2+F4+D5+G5 G1+C4+D4+B4+G5:2 G1+C4+D4+B4+G5"
    };
}

/// The ostinato from bar 299: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_14 {
    () => {
        "C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+B4+Db5+D5+G5:2 C2+B4+Db5+D5+G5 | \
         C2+D5+F5+G5 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 (3 C2+B4+Db5+D5+G5 C2+B4+D5+G5 C2+B4+Db5+D5+G5 (3 C2+B4+Db5+D5+G5 C2+B4+D5+G5 C2+B4+Db5+D5+G5"
    };
}

/// The ostinato from bar 303: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_15 {
    () => {
        "C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+B4+Db5+D5+G5:2 C2+B4+Db5+D5+G5 | \
         C2+D5+F5+G5 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 (3 C2+B4+Db5+D5+F5+G5 C2+B4+D5+F5+G5 C2+B4+Db5+D5+F5+G5 (3 C2+B4+Db5+D5+F5+G5 C2+B4+D5+G5 C2+B4+Db5+D5+G5"
    };
}

/// Two bars from bar 307: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_77 {
    () => {
        "C2+D5+F5+G5:4 C2+D5+F5+G5 G1+C4+B4+D5+G5:2 C2+B4+D5+G5 | \
         C2+D4+E4+G5:4 C2+D4+E4+G5 G1+D4+F4+C5+G5:2 G1+D4+F4+C5+G5"
    };
}

/// The ostinato from bar 307: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_16 {
    () => {
        "C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+D5+F5+G5:2 (3 C2+D5+F5+G5:1 C2+D5+E5+F5+G5 C2+D5+F5+G5 C2+B4+Db5+D5+G5:2 C2+B4+Db5+D5+G5 | \
         C2+D5+E5+G5 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+E5+G5 C2+D5+E5+G5 (3 C2+D5+G5 C2+D5+E5+G5 C2+D5+G5 (3 C2+D5+G5 C2+D5+E5+G5 C2+D5+G5"
    };
}

/// Two bars from bar 309: bass clarinet, bassoons, contrabassoon, sopranino saxophone, tenor saxophone, trumpets, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_78 {
    () => {
        "C2+D4+E4+G5:4 C2+D5+E5+G5 G1+B4+C5+D5+G5:2 G2+C4+D4+B4+G5 | \
         C2+D4+E4+G5:4 C2+D5+E5+G5 G1+B4+C5+D5+G5:2 G1+C4+D4+B4+G5"
    };
}

/// The ostinato from bar 309: piccolo, flutes, oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_17 {
    () => {
        "C2+D5+E5+G5+C6:2 (3 C2+D5+E6+G6:1 C2+D5+E6+G6 C2+D5+E6+G6 C2+D5+E6+G6:2 (3 C2+D5+E6+G6:1 C2+D5+E6+G6 C2+D5+E6+G6 C2+B4+Db5+D6+G6:2 C2+B4+Db5+D6+G6 | \
         C2+D5+E6+G6 (3 C2+D5+E6+G6:1 C2+D5+E6+G6 C2+D5+E6+G6 C2+D5+E6+G6:2 (3 C2+E5+D6+G6:1 C2+E5+D6+G6 C2+E5+D6+G6 (3 C2+B4+Db5+D6+G6 C2+B4+E5+D6+G6 C2+B4+Db5+D6+G6 (3 C2+B4+Db5+D6+G6 C2+B4+E5+D6+G6 C2+B4+Db5+D6+G6"
    };
}

/// Two bars from bar 311: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_79 {
    () => {
        "C2+E4+G5:4 C2+D4+E4+G5 G1+C4+D4+Bb4+G5:2 G2+C4+D4+Bb4+G5 | \
         C2+E4+G5:4 C2+D4+E4+G5 G1+C4+D4+Bb4+G5:2 G1+C4+D4+Bb4+G5"
    };
}

/// The ostinato from bar 311: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_18 {
    () => {
        bolero_ostinato!(
            "C2+D5+E5+G5",
            "C2+D5+E5+G5",
            "C2+Bb4+D5+G5",
            "C2+D5+E5+G5",
            "C2+D5+E5+G5",
            "C2+Bb4+D5+G5"
        )
    };
}

/// Two bars from bar 319: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_80 {
    () => {
        "C2+F4+D5+G5:4 C2+F4+D5+G5 G1+C4+D4+B4+G5:2 G2+C4+D4+B4+G5 | \
         C2+F4+D5+G5:4 C2+F4+D5+G5 G1+C4+D4+B4+G5:2 G1+C4+D4+B4+G5"
    };
}

/// Two bars from bar 321: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_81 {
    () => {
        "C2+E4+G5:4 C2+D4+E4+G5 G1+C4+D4+Bb4+G5:2 G2+C4+D4+Bb4+G5 | \
         C2+E4+G5:4 C2+E4+G5 G1+C4+D4+Bb4+G5:2 G1+C4+D4+Bb4+G5"
    };
}

/// Two bars from bar 323: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_82 {
    () => {
        "C2+E4+G5:4 C2+D4+E4+G5 G1+C4+D5+G5:2 G2+C4+D5+G5 | \
         C2+E4+G5:4 C2+D4+E4+G5 G1+C4+D5+G5:2 G1+C4+D5+G5"
    };
}

/// The ostinato from bar 323: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_19 {
    () => {
        "C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+D5+E5+G5 C2+D5+E5+G5 C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+D5+E5+G5 C2+D5+E5+G5 C2+D5+G5:2 C2+D5+G5 | \
         C2+D5+E5+G5 (3 C2+D5+E5+G5:1 C2+D5+E5+G5 C2+D5+E5+G5 C2+D5+E5+G5:2 (3 C2+D5+E5+G5:1 C2+D5+E5+G5 C2+D5+E5+G5 (3 C2+D5+G5 C2+D5+E5+G5 C2+D5+G5 (3 C2+D5+G5 C2+D5+E5+G5 C2+D5+G5"
    };
}

/// Two bars from bar 325: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_83 {
    () => {
        "C2+F4+D5+G5:4 C2+F4+D5+G5 G1+C4+D4+B4+G5:2 G2+C4+D4+B4+G5 | \
         C2+F4+D5+G5:4 C2+F4+D5+G5 G1+C4+D4+B4+G5:2 G1+B3+C4+D4"
    };
}

/// Two bars from bar 327: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_84 {
    () => {
        "E1+B4+G#5:4 E2+B4+G#5 B1+E3+D5+F#5:2 B2+D5+F#5 | \
         E1+B4+G#5:4 E2+B4+G#5 B1+E3+D5+F#5:2 B1+D5+F#5"
    };
}

/// The ostinato from bar 327: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_20 {
    () => {
        "E2+A#4+B4+G#5:2 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 E2+A#4+B4+G#5:2 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 B2+G#4+D5+E5+F#5:2 B2+G#4+D5+E5+F#5 | \
         E2+A#4+B4+G#5 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 E2+A#4+B4+G#5:2 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 (3 B2+G#4+D5+E5+F#5 B2+C#5+D5+E5+F#5 B2+G#4+D5+E5+F#5 (3 B2+G#4+D5+E5+F#5 B2+C#5+D5+E5+F#5 B2+G#4+D5+E5+F#5"
    };
}

/// Two bars from bar 333: bass clarinet, bassoons, contrabassoon, trombones, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_85 {
    () => {
        "E1+B4+G#5:4 E2+B4+G#5 B1+E3+A3+C#4+D5+F#5:2 B2+D5+F#5 | \
         E1+B4+G#5:4 E2+B4+G#5 G1+F3+A3+D4+B4+G5:1 G3+B3 G1+F3+A3+D4+B4+G5 E3+G3"
    };
}

/// The ostinato from bar 333: oboes, clarinets, horns, second violins, violas and cellos.
macro_rules! bolero_rhythm_21 {
    () => {
        "E2+A#4+B4+G#5:2 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 E2+A#4+B4+G#5:2 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 B2+G#4+D5+E5+F#5:2 B2+G#4+D5+E5+F#5 | \
         E2+A#4+B4+G#5 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 E2+A#4+B4+G#5:2 (3 E2+A#4+B4+G#5:1 E2+B4+C#5+G#5 E2+A#4+B4+G#5 (3 G2+A4+B4+D5+G5 G2+B4+C#5+D5+G5 G2+A4+B4+D5+G5 (3 G2+A4+B4+D5+G5 G2+B4+C#5+D5+G5 G2+A4+B4+D5+G5"
    };
}

/// Two bars from bar 335: oboes, English horn, clarinets, bass clarinet, bassoons, contrabassoon, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_86 {
    () => {
        "C2+A4+E5+G5:4 C2+D5+E5+G5+C6 G1+B4+Db5+D5+C6:2 G1+B4+Db5+D5+C6 | \
         C2+D5+E5+G5+C6:4 C2+D5+E5+G5+C6 G1+B4+Db5+D5+C6:2 G1+B4+Db5+D5+C6"
    };
}

/// The ostinato from bar 335: piccolo, flutes, horns, D trumpet, trumpets, first violins, second violins, violas and cellos.
macro_rules! bolero_rhythm_22 {
    () => {
        "C2+D5+E5+G5+C6:2 (3 C2+D5+E6+G6:1 C2+D5+E6+G6 C2+D5+E6+G6 C2+D5+E6+G6:2 (3 C2+D5+E6+G6:1 C2+D5+E6+G6 C2+D5+E6+G6 C2+B5+D6+G6:2 C2+B5+D6+G6 | \
         C2+D5+E6+G6 (3 C2+D5+E6+G6:1 C2+D5+E6+G6 C2+D5+E6+G6 C2+D5+E6+G6:2 (3 C2+D5+E6+G6:1 C2+D5+E6+G6 C2+D5+E6+G6 (3 C2+B5+D6+G6 C2+B5+D6+G6 C2+B5+D6+G6 (3 C2+B5+D6+G6 C2+B5+D6+G6 C2+B5+D6+G6"
    };
}

/// Two bars from bar 337: oboes, English horn, clarinets, bass clarinet, bassoons, contrabassoon, tuba, timpani, harp and double basses.
macro_rules! bolero_beats_87 {
    () => {
        "C2+D5+E5+G5+C6:4 C2+D5+E5+G5+C6 G1+B4+Db5+D5+C6:2 G1+B4+Db5+D5+C6 | \
         C2+D5+E5+G5+C6:4 C2+D5+E5+G5+C6 G1+B4+Db5+D5+C6:2 G1+B4+Db5+D5+C6"
    };
}

/// The ostinato from bar 337: piccolo, flutes, horns, D trumpet, trumpets, first violins, second violins, violas and cellos.
macro_rules! bolero_rhythm_23 {
    () => {
        bolero_ostinato!(
            "C2+D5+E6+G6",
            "C2+D5+E6+G6",
            "C2+B5+D6+G6",
            "C2+D5+E6+G6",
            "C2+D5+E6+G6",
            "C2+B5+D6+G6"
        )
    };
}

/// Two bars from bar 339: oboes, English horn, clarinets, bass clarinet, bassoons, contrabassoon, horns, trombones, tuba, timpani, harp, second violins, violas, cellos and double basses.
macro_rules! bolero_beats_88 {
    () => {
        "F1+Ab3+C4:2 (3 F1+Ab5+B5+C6+Db6:1 E5+G5 Db5+F5 _:4 (3 _:1 F1+Ab3+D4+Eb5+B5+C6+Db6 F1+Ab3+Eb4+Bb4+B5+C6+Db6 (3 F1+A3+E4+Ab4+B5+C6+Db6 F1+Ab3+G4+Bb4+B5+C6+Db6 F1+F#4+Ab4+B5+C6+Db6 | \
         C1+E5+F#5+G5+C6:2 -:10"
    };
}

/// The first tune, A, in C major, as the flute plays it: sixteen bars, its
/// last note C on the first beat of the seventeenth, and a bar of the
/// accompaniment alone before the next instrument takes it.
const BOLERO_A: &str = "C5:6 B4:1 C5 D5 C5 B4 A4 | C5:2 C5:1 A4 C5:6 B4:1 C5 | A4 G4 E4 F4 G4:8 | \
     _:1 F4 E4 D4 E4 F4 G4 A4 G4:4 | _:5 A4:1 B4 A4 G4 F4 E4 D4 | \
     E4 D4 C4:4 C4:1 D4 E4:2 F4 | D4:4 G4:8 | _:10 -:2 | D5:7 C5:1 B4 A4 B4 C5 | \
     D5 C5 B4:3 C5:1 B4 A4 C5 B4 A4 F4 | _:2 F4:1. F4. F4:2. A4. C5:1 A4 B4 G4 | \
     F4:2 F4:1. F4. F4:2. A4. B4:1 G4 A4 F4 | D4:2 D4:1 C4 D4:6 D4:1. D4. | \
     D4:2. F4. A4:1 F4 G4 E4 D4:2 D4:1 C4 | D4:6 D4:1 C4 D4:2 E4:1 F4 | G4:9 F4:1 E4 D4 | \
     C4:2 -:10 | -:12";

/// The second tune, B, as the bassoon plays it, from B flat high in its
/// range: C major darkened by B flat, D flat, A flat and E flat, falling
/// to the low C.
const BOLERO_B: &str = "Bb4:5 A4:1 G4 F4 Bb4 C5 A4 G4 | Bb4:2 A4:1 G4 Bb4:4 A4:1 Bb4 A4 G4 | \
     _:5 F4:1 E4 D4 E4:4 | _ -:2 Bb4 C5 Db5 | _ Db5:4 Db5:2 Db5 Db5 | \
     (3 Db5 Db5 Db5 Db5 C5:1 Bb4 Db5:2 C5:1 Bb4 | Db5 C5 Bb4 Ab4 G4 F4 E4:6 | _:10 -:2 | \
     D4:6 E4:2 D4:1 E4 F4:2 | _:4 G4:2 Ab4 (3 F4 G4 E4 | D4:1 E4 D4 C4:3 Bb3:6 | \
     _:1 C4 Bb3 C4 D4 E4 D4 C4 D4 C4 Bb3 Ab3 | Bb3 Ab3 G3:4 F3:6 | _:5 G3:1 F3 Eb3:5 | \
     _:4 F3:1 Eb3 F3 Db3:5 | _ Db3:1 Eb3 Db3 F3 Eb3 Db3 C3 | _:4 -:8 | -:12";

/// Woodwinds
const BOLERO_WOODWINDS: &str = "Bb4+E5+G5+Bb5+E6+G6+Bb6:5 A4+D5+F5+A5+D6+F6+A6:1 G4+C5+E5+G5+C6+E6+G6 F4+Bb4+D5+F5+Bb5+D6+F6 Bb4+E5+G5+Bb5+E6+G6+Bb6 C5+F5+A5+C6+F6+A6+C7 A4+D5+F5+A5+D6+F6+A6 G4+C5+E5+G5+C6+E6+G6 | \
     Bb4+E5+G5+Bb5+E6+G6+Bb6:2 A4+D5+F5+A5+D6+F6+A6:1 G4+C5+E5+G5+C6+E6+G6 Bb4+E5+G5+Bb5+E6+G6+Bb6:4 A4+D5+F5+A5+D6+F6+A6:1 Bb4+E5+G5+Bb5+E6+G6+Bb6 A4+D5+F5+A5+D6+F6+A6 G4+C5+E5+G5+C6+E6+G6 | \
     _:5 F4+Bb4+D5+F5+Bb5+D6+F6:1 E4+G4+C5+E5+G5+C6+E6 D4+F4+Bb4+D5+F5+Bb5+D6 E4+G4+C5+E5+G5+C6+E6:4 | \
     _ -:2 Bb4+D5+G5+Bb5+D6+G6+Bb6 C5+E5+A5+C6+E6+A6+C7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 | \
     _ Db5+E5+Bb5+Db6+Db7:4 Db5+E5+Bb5+Db6+Db7:2 Db5+E5+Bb5+Db6+Db7 Db5+E5+Bb5+Db6+Db7 | \
     (3 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 C5+Eb5+A5+C6+Eb6+A6+C7:1 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 Db5+E5+Bb5+Db6+E6+Bb6+Db7:2 C5+Eb5+A5+C6+Eb6+A6+C7:1 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 | \
     Db5+E5+Bb5+Db6+E6+Bb6+Db7 C5+Eb5+A5+C6+Eb6+A6+C7 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 Ab4+C5+F5+Ab5+C6+F6+Ab6 G4+Bb4+E5+G5+Bb5+E6+G6 F4+Ab4+D5+F5+Ab5+D6+F6 E4+G4+C5+E5+G5+C6+E6:6 | \
     _:10 -:2 | \
     D4+A4+D5+F5+A5+D6:6 E4+B4+E5+G5+B5+E6:2 D4+A4+D5+F5+A5+D6:1 E4+B4+E5+G5+B5+E6 F4+C5+F5+A5+C6+F6:2 | \
     _:4 G4+D5+G5+B5+D6+G6:2 Ab4+F5+Ab5+D6+F6+Ab6 (3 F4+B4+F5+Ab5+B5+F6 G4+D5+G5+B5+D6+G6 E4+B4+E5+G5+B5+E6 | \
     D4+G4+A4+D5+G5+A5+D6:1 E4+A4+C5+E5+A5+C6+E6 D4+G4+A4+D5+G5+A5+D6 C4+E4+G4+C5+E5+G5+C6:3 Bb3+D4+G4+Bb4+D5+G5+Bb5:6 | \
     _:1 C4+E4+A4+C5+E5+A5+C6 Bb3+D4+G4+Bb4+D5+G5+Bb5 C4+E4+A4+C5+E5+A5+C6 D4+G4+Bb4+D5+G5+Bb5+D6 E4+A4+C5+E5+A5+C6+E6 D4+G4+Bb4+D5+G5+Bb5+D6 C4+E4+A4+C5+E5+A5+C6 D4+G4+Bb4+D5+G5+Bb5+D6 C4+E4+A4+C5+E5+A5+C6 Bb3+D4+G4+Bb4+D5+G5+Bb5 Ab3+C4+E4+Ab4+C5+E5+Ab5 | \
     Bb3+E4+G4+Bb4+E5+G5+Bb5 Ab3+Ab4+Ab5 G3+C4+E4+G4+C5+E5+G5:4 F3+Ab3+C4+F4+C5+F5:6 | \
     _:5 G3+Bb3+D4+G4+D5+G5:1 F3+Ab3+C4+F4+C5+F5 Eb3+G3+Bb3+Eb4+Bb4+Eb5:5 | \
     _:4 F3+Ab3+C4+F4+C5+F5:1 Eb3+G3+Bb3+Eb4+Bb4+Eb5 F3+Ab3+C4+F4+C5+F5 Db3+F3+Ab3+Db4+Ab4+Db5:5 | \
     _ Db3+F3+Ab3+Db4+Ab4+Db5:1 Eb3+G3+Bb3+Eb4+Bb4+Eb5 Db3+F3+Ab3+Db4+Ab4+Db5 F3+Ab3+C4+F4+C5+F5 Eb3+G3+Bb3+Eb4+Bb4+Eb5 Db3+F3+Ab3+Db4+Ab4+Db5 C3+E3+G3+C4+G4+C5 | \
     _:4 -:8 | -:12";

/// Violins in harmony
const BOLERO_VIOLINS_IN_HARMONY: &str = "C5+E5+G5:6 B4+D5+F#5:1 C5+E5+G5 D5+F#5+A5 C5+E5+G5 B4+D5+F#5 A4+C5+E5 | \
     C5+E5+G5:2 C5+E5+G5:1 A4+C5+E5 C5+E5+G5:6 B4+D5+F#5:1 C5+E5+G5 | \
     A4+C5+E5 G4+B4+D5 E4+G4+B4 F4+A4+C5 G4+C5+E5:8 | \
     _:1 F4+A4+C5 E4+G4+B4 D4+F4+A4 E4+G4+B4 F4+A4+C5 G4+B4+D5 A4+C5+E5 G4+B4+D5:4 | \
     _:5 A4+C5+E5:1 B4+D5+F#5 A4+C5+E5 G4+B4+D5 F4+A4+C5 E4+G4+B4 D4+F4+A4 | \
     E4+G4+B4 D4+F4+A4 C4+E4+G4:4 C4+E4+G4:1 D4+F4+A4 E4+G4+B4:2 F4+A4+C5 | \
     D4+F4+A4:4 G4+B4+D5:8 | _:10 -:2 | \
     D5+F5+A5:7 C5+E5+G5:1 B4+D5+F5 A4+C5+E5 B4+D5+F5 C5+E5+G5 | \
     D5+F5+A5 C5+E5+G5 B4+D5+F5:3 C5+E5+G5:1 B4+D5+F5 A4+C5+E5 C5+E5+G5 B4+D5+F5 A4+C5+E5 F4+A4+C5 | \
     _:2 F4+A4+C5:1. F4+A4+C5. F4+A4+C5:2. A4+C5+E5. C5+E5+G5:1 A4+C5+E5 B4+D5+F5 G4+B4+D5 | \
     F4+A4+C5:2 F4+A4+C5:1. F4+A4+C5. F4+A4+C5:2. A4+C5+E5. B4+D5+F5:1 G4+B4+D5 A4+C5+E5 F4+A4+C5 | \
     D4+F4+A4:2 D4+F4+A4:1 C4+E4+G4 D4+F4+A4:6 D4+F4+A4:1. D4+F4+A4. | \
     D4+F4+A4:2. F4+A4+C5. A4+C5+E5:1 F4+A4+C5 G4+B4+D5 E4+G4+B4 D4+F4+A4:2 D4+F4+A4:1 C4+E4+G4 | \
     D4+F4+A4:6 D4+F4+A4:1 C4+E4+G4 D4+F4+A4:2 E4+G4+B4:1 F4+A4+C5 | \
     G4+C5+E5:9 F4+A4+C5:1 E4+G4+B4 D4+F4+A4 | C4+E4+G4:2 -:10 | -:12 & -:12 | - | \
     -:8 D5:4 | E5:1 -:11 | -:12 | - | - | - | - | - | - | - | - | - | - | - | - | -";

/// Trombone and strings
const BOLERO_TROMBONE_AND_STRINGS: &str = "Bb4+E5+G5+Bb5+E6+G6+Bb6:5 A4+D5+F5+A5+D6+F6+A6:1 G4+C5+E5+G5+C6+E6+G6 F4+Bb4+D5+F5+Bb5+D6+F6 Bb4+E5+G5+Bb5+E6+G6+Bb6 C5+F5+A5+C6+F6+A6+C7 A4+D5+F5+A5+D6+F6+A6 G4+C5+E5+G5+C6+E6+G6 | \
     Bb4+E5+G5+Bb5+E6+G6+Bb6:2 A4+D5+F5+A5+D6+F6+A6:1 G4+C5+E5+G5+C6+E6+G6 Bb4+E5+G5+Bb5+E6+G6+Bb6:4 A4+D5+F5+A5+D6+F6+A6:1 Bb4+E5+G5+Bb5+E6+G6+Bb6 A4+D5+F5+A5+D6+F6+A6 G4+C5+E5+G5+C6+E6+G6 | \
     _:5 F4+Bb4+D5+F5+Bb5+D6+F6:1 E4+G4+C5+E5+G5+C6+E6 D4+F4+Bb4+D5+F5+Bb5+D6 E4+G4+C5+E5+G5+C6+E6:4 | \
     _ -:2 Bb4+D5+G5+Bb5+D6+G6+Bb6 C5+E5+A5+C6+E6+A6+C7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 | \
     _ Db5+E5+Bb5+Db6+Db7:4 Db5+E5+Bb5+Db6+Db7:2 Db5+E5+Bb5+Db6+Db7 Db5+E5+Bb5+Db6+Db7 | \
     (3 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 C5+Eb5+A5+C6+Eb6+A6+C7:1 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 Db5+E5+Bb5+Db6+E6+Bb6+Db7:2 C5+Eb5+A5+C6+Eb6+A6+C7:1 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 | \
     Db5+E5+Bb5+Db6+E6+Bb6+Db7 C5+Eb5+A5+C6+Eb6+A6+C7 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 Ab4+C5+F5+Ab5+C6+F6+Ab6 G4+Bb4+E5+G5+Bb5+E6+G6 F4+Ab4+D5+F5+Ab5+D6+F6 E4+G4+C5+E5+G5+C6+E6:6 | \
     _:10 -:2 | \
     D4+F4+A4+D5+F5+A5+D6:6 E4+G4+B4+E5+G5+B5+E6:2 D4+F4+A4+D5+F5+A5+D6:1 E4+G4+B4+E5+G5+B5+E6 F4+A4+C5+F5+A5+C6+F6:2 | \
     _:4 G4+B4+D5+G5+B5+D6+G6:2 Ab4+D5+F5+Ab5+D6+F6+Ab6 (3 F4+Ab4+B4+F5+Ab5+B5+F6 G4+B4+D5+G5+B5+D6+G6 E4+G4+B4+E5+G5+B5+E6 | \
     D4+G4+A4+D5+G5+A5+D6:1 E4+A4+C5+E5+A5+C6+E6 D4+G4+A4+D5+G5+A5+D6 C4+E4+G4+C5+E5+G5+C6:3 Bb3+D4+G4+Bb4+D5+G5+Bb5:6 | \
     _:1 C4+E4+A4+C5+E5+A5+C6 Bb3+D4+G4+Bb4+D5+G5+Bb5 C4+E4+A4+C5+E5+A5+C6 D4+G4+Bb4+D5+G5+Bb5+D6 E4+A4+C5+E5+A5+C6+E6 D4+G4+Bb4+D5+G5+Bb5+D6 C4+E4+A4+C5+E5+A5+C6 D4+G4+Bb4+D5+G5+Bb5+D6 C4+E4+A4+C5+E5+A5+C6 Bb3+D4+G4+Bb4+D5+G5+Bb5 Ab3+C4+E4+Ab4+C5+E5+Ab5 | \
     Bb3+E4+G4+Bb4+E5+G5+Bb5 Ab3+Ab4+Ab5 G3+C4+E4+G4+C5+E5+G5:4 F3+Ab3+C4+F4+C5+F5:6 | \
     _:5 G3+Bb3+D4+G4+D5+G5:1 F3+Ab3+C4+F4+C5+F5 Eb3+G3+Bb3+Eb4+Bb4+Eb5:5 | \
     _:4 F3+Ab3+C4+F4+C5+F5:1 Eb3+G3+Bb3+Eb4+Bb4+Eb5 F3+Ab3+C4+F4+C5+F5 Db3+F3+Ab3+Db4+Ab4+Db5:5 | \
     _ Db3+F3+Ab3+Db4+Ab4+Db5:1 Eb3+G3+Bb3+Eb4+Bb4+Eb5 Db3+F3+Ab3+Db4+Ab4+Db5 F3+Ab3+C4+F4+C5+F5 Eb3+G3+Bb3+Eb4+Bb4+Eb5 Db3+F3+Ab3+Db4+Ab4+Db5 C3+E3+G3+C4+G4+C5 | \
     _:4 -:8 | -:12";

/// Trumpets
const BOLERO_TRUMPETS: &str = "C5+E5+G5:6 B4+D5+F#5:1 C5+E5+G5 D5+F#5+A5 C5+E5+G5 B4+D5+F#5 A4+C5+E5 | \
     C5+E5+G5:2 C5+E5+G5:1 A4+C5+E5 C5+E5+G5:6 B4+D5+F#5:1 C5+E5+G5 | \
     A4+C5+E5 G4+B4+D5 E4+G4+B4 F4+A4+C5 G4+C5+E5:8 | \
     _:1 F4+A4+C5 E4+G4+B4 D4+F4+A4 E4+G4+B4 F4+A4+C5 G4+B4+D5 A4+C5+E5 G4+B4+D5:4 | \
     _:5 A4+C5+E5:1 B4+D5+F#5 A4+C5+E5 G4+B4+D5 F4+A4+C5 E4+G4+B4 D4+F4+A4 | \
     E4+G4+B4 D4+F4+A4 C4+E4+G4:4 C4+E4+G4:1 D4+F4+A4 E4+G4+B4:2 F4+A4+C5 | \
     D4+F4+A4:4 G4+B4+D5:8 | _:10 -:2 | \
     D5+F5+A5:7 C5+E5+G5:1 B4+D5+F5 A4+C5+E5 B4+D5+F5 C5+E5+G5 | \
     D5+F5+A5 C5+E5+G5 B4+D5+F5:3 C5+F5+A5:1 B4+E5+G5 A4+D5+F5 C5+F5+A5 B4+E5+G5 A4+D5+F5 F4+A4+D5 | \
     _:2 F4+A4+D5:1. F4+A4+D5. F4+A4+D5:2. A4+D5+F5. C5+F5+A5:1 A4+D5+F5 B4+E5+G5 G4+B4+E5 | \
     F4+A4+D5:2 F4+A4+D5:1. F4+A4+D5. F4+A4+D5:2. A4+D5+F5. B4+E5+G5:1 G4+B4+E5 A4+D5+F5 F4+A4+D5 | \
     D4+F4+A4:2 D4+F4+A4:1 C4+E4+G4 D4+F4+A4:6 D4+F4+A4:1. D4+F4+A4. | \
     D4+F4+A4:2. F4+A4+C5. A4+C5+E5:1 F4+A4+C5 G4+B4+D5 E4+G4+B4 D4+F4+A4:2 D4+F4+A4:1 C4+E4+G4 | \
     D4+F4+A4:6 D4+F4+A4:1 C4+E4+G4 D4+F4+A4:2 E4+G4+B4:1 F4+A4+C5 | \
     G4+C5+E5:9 F4+A4+C5:1 E4+G4+B4 D4+F4+A4 | C4+E4+G4:2 -:10 | -:12 & -:12 | - | \
     -:8 D5:4 | E5:1 -:11 | C5+E5:5 -:7 | -:12 | - | - | - | - | - | - | - | - | - | - | \
     - | -";

/// Last statement
const BOLERO_LAST_STATEMENT: &str = "Bb4+E5+G5+Bb5+E6+G6+Bb6:5 A4+D5+F5+A5+D6+F6+A6:1. G4+C5+E5+G5+C6+E6+G6. F4+Bb4+D5+F5+Bb5+D6+F6. Bb4+E5+G5+Bb5+E6+G6+Bb6 C5+F5+A5+C6+F6+A6+C7 A4+D5+F5+A5+D6+F6+A6 G4+C5+E5+G5+C6+E6+G6 | \
     Bb4+E5+G5+Bb5+E6+G6+Bb6:2 A4+D5+F5+A5+D6+F6+A6:1. G4+C5+E5+G5+C6+E6+G6. Bb4+E5+G5+Bb5+E6+G6+Bb6:4 A4+D5+F5+A5+D6+F6+A6:1 Bb4+E5+G5+Bb5+E6+G6+Bb6 A4+D5+F5+A5+D6+F6+A6 G4+C5+E5+G5+C6+E6+G6 | \
     _:5 F4+Bb4+D5+F5+Bb5+D6+F6:1 E4+G4+C5+E5+G5+C6+E6 D4+F4+Bb4+D5+F5+Bb5+D6 E4+G4+C5+E5+G5+C6+E6:4 | \
     _ -:2 Bb4+D5+G5+Bb5+D6+G6+Bb6 C5+E5+A5+C6+E6+A6+C7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 | \
     _ Db5+E5+Bb5+Db6+E6+Bb6+Db7:4 Db5+E5+Bb5+Db6+E6+Bb6+Db7:2 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 | \
     (3 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 Db5+E5+Bb5+Db6+E6+Bb6+Db7 C5+Eb5+A5+C6+Eb6+A6+C7:1 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 Db5+E5+Bb5+Db6+E6+Bb6+Db7:2 C5+Eb5+A5+C6+Eb6+A6+C7:1 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 | \
     Db5+E5+Bb5+Db6+E6+Bb6+Db7 C5+Eb5+A5+C6+Eb6+A6+C7 Bb4+Db5+G5+Bb5+Db6+G6+Bb6 Ab4+C5+F5+Ab5+C6+F6+Ab6 G4+Bb4+E5+G5+Bb5+E6+G6 F4+Ab4+D5+F5+Ab5+D6+F6 E4+G4+C5+E5+G5+C6+E6:6 | \
     _:10 -:2 | \
     D4+F4+A4+D5+F5+A5+D6:6 E4+G4+B4+E5+G5+B5+E6:2 D4+F4+A4+D5+F5+A5+D6:1 E4+G4+B4+E5+G5+B5+E6 F4+A4+C5+F5+A5+C6+F6:2 | \
     _:4 G4+B4+D5+G5+B5+D6+G6:2 Ab4+D5+F5+Ab5+D6+F6+Ab6 (3 F4+Ab4+B4+F5+Ab5+B5+F6 G4+B4+D5+G5+B5+D6+G6 E4+G4+B4+E5+G5+B5+E6 | \
     D4+G4+A4+D5+G5+A5+D6:1 E4+A4+C5+E5+A5+C6+E6 D4+G4+A4+D5+G5+A5+D6 C4+E4+G4+C5+E5+G5+C6:3 Bb3+D4+G4+Bb4+D5+G5+Bb5:6 | \
     _:1 C4+E4+A4+C5+E5+A5+C6 Bb3+D4+G4+Bb4+D5+G5+Bb5 C4+E4+A4+C5+E5+A5+C6 D4+G4+Bb4+D5+G5+Bb5+D6 E4+A4+C5+E5+A5+C6+E6 D4+G4+Bb4+D5+G5+Bb5+D6 C4+E4+A4+C5+E5+A5+C6 D4+G4+Bb4+D5+G5+Bb5+D6 C4+E4+A4+C5+E5+A5+C6 Bb3+D4+G4+Bb4+D5+G5+Bb5 Ab3+C4+F4+Ab4+C5+F5+Ab5 | \
     Bb3+E4+G4+Bb4+E5+G5+Bb5 Ab3+Ab4+Ab5 G3+C4+E4+G4+C5+E5+G5:10 | \
     _:8 -:2 D4+F4+A4+D5+F5+A5+D6 | _:6 E4+G4+B4+E5+G5+B5+E6 | \
     D4+F4+A4+D5+F5+A5+D6:1 E4+G4+B4+E5+G5+B5+E6 F4+A4+C5+F5+A5+C6+F6:6 G4+B4+D5+G5+B5+D6+G6:2 B4+E5+Ab5+B5+E6+Ab6+B6";

/// E major
const BOLERO_E_MAJOR: &str = "B4+E5+G#5+B5+E6+G#6+B6:5 A4+D5+F#5+A5+D6+F#6+A6:1 G#4+B4+E5+G#5+B5+E6+G#6 F#4+A4+D5+F#5+A5+D6+F#6 G#4+B4+E5+G#5+B5+E6+G#6 A4+D5+F#5+A5+D6+F#6+A6 B4+E5+G#5+B5+E6+G#6+B6 C5+F5+A5+C6+F6+A6+C7 | \
     B4+E5+G#5+B5+E6+G#6+B6:5 C5+F5+A5+C6+F6+A6+C7:1 D5+G5+B5+D6+G6+B6+D7 C5+F5+A5+C6+F6+A6+C7 B4+E5+G#5+B5+E6+G#6+B6 A4+D5+F#5+A5+D6+F#6+A6 G#4+B4+E5+G#5+B5+E6+G#6 F#4+A4+D5+F#5+A5+D6+F#6 | \
     G#4+B4+E5+G#5+B5+E6+G#6 A4+D5+F#5+A5+D6+F#6+A6 B4+E5+G#5+B5+E6+G#6+B6:3 C5+F5+A5+C6+F6+A6+C7:1 D5+G5+B5+D6+G6+B6+D7 C5+F5+A5+C6+F6+A6+C7 B4+E5+G#5+B5+E6+G#6+B6 A4+C5+F5+A5+C6+F6+A6 G#4+B4+E5+G#5+B5+E6+G#6 F#4+A4+D5+F#5+A5+D6+F#6 | \
     G#4+B4+E5+G#5+B5+E6+G#6 F#4+A4+D5+F#5+A5+D6+F#6 E4+G#4+B4+E5+G#5+B5+E6:6 D4+F#4+A4+D5+F#5+A5+D6:1 C4+E4+G4+C5+E5+G5+C6 D4+F#4+A4+D5+F#5+A5+D6 E4+G#4+B4+E5+G#5+B5+E6 | \
     _:8 F4+A4+C5+F5+A5+C6+F6:1 G4+B4+D5+G5+B5+D6+G6 F4+A4+C5+F5+A5+C6+F6 E4+G#4+B4+E5+G#5+B5+E6 | \
     _:8 D4+F#4+A4+D5+F#5+A5+D6:1 C4+E4+G#4+C5+E5+G#5+C6 D4+F#4+A4+D5+F#5+A5+D6 E4+G#4+B4+E5+G#5+B5+E6 | \
     _:8 F#4+A4+C#5+F#5+A5+C#6+F#6:1 G#4+B4+D#5+G#5+B5+D#6+G#6 F#4+A4+C#5+F#5+A5+C#6+F#6 E4+G#4+B4+E5+G#5+B5+E6 | \
     _:8 D4+F4+A4+D5+F5+A5+D6:1 E4+G4+B4+E5+G5+B5+E6 D4+F4+A4+D5+F5+A5+D6 C4+E4+G4+C5+E5+G5+C6";

/// Ending
const BOLERO_ENDING: &str = "E3+G3+C4+C5:4 (3 F#3+Bb3+D4+F#4+F#5:1 G3+B3+Eb4+G4+G5 Ab3+C4+E4+Ab4+Ab5 _:2 (3 E3+A3+C4+E4+E5:1 F3+Bb3+Db4+F4+F5 F#3+B3+D4+F#4+F#5 _:2 | \
     -:4 (3 F#3+Bb3+D4+F#4+F#5:1 G3+B3+Eb4+G4+G5 Ab3+C4+E4+Ab4+Ab5 _:2 (3 E3+A3+C4+E4+E5:1 F3+Bb3+Db4+F4+F5 F#3+B3+D4+F#4+F#5 _:2 | \
     -:4 (3 F#3+Bb3+D4+F#4+F#5:1 G3+B3+Eb4+G4+G5 Ab3+C4+E4+Ab4+Ab5 _:2 (3 E3+A3+C4+E4+E5:1 F3+Bb3+Db4+F4+F5 F#3+B3+D4+F#4+F#5 _:2 | \
     -:4 (3 F#3+Bb3+D4+F#4+F#5:1 G3+B3+Eb4+G4+G5 Ab3+C4+E4+Ab4+Ab5 _:2 (3 E3+A3+C4+E4+E5:1 F3+Bb3+Db4+F4+F5 F#3+B3+D4+F#4+F#5 _:2 | \
     - (3 B3+Db4+C5+F5+Ab5+C6+F6+Ab6+C7:1 B4+E5+G5+B5+E6+G6+B6 Bb4+Db5+F5+Bb5+Db6+F6+Bb6 _:4 (3 _:1 B3+D4+Ab4+C5+Eb5+Ab5+C6+Eb6+Ab6 C4+Eb4+G4+Bb4+Db5+G5+Bb5+Db6+G6 (3 Db4+E4+F4+Ab4+C5+F5+Ab5+C6+F6 D4+Eb4+F4+G4+Bb4+Eb5+G5+Bb5+Eb6 Db4+Eb4+F4+F#4+Ab4+Db5+F5+Ab5+Db6 | \
     G3+C4+E4+G4+C5+E5+G5+C6:2 -:10 & -:12 | G3+C4+E4:4 -:8 | G3+C4+E4:4 -:8 | \
     G3+C4+E4:4 -:8 | F3+Ab3:2 -:10 | -:12";

/// C major under every bar of a statement.
const BOLERO_C: &str = "C | C | C | C | C | C | C | C | C | C | C | C | C | C | C | C | C | C";

// Source: the printed full score (Durand, 1929) as engraved in LilyPond by Nicolas Sceaux
// (CC0), https://github.com/nsceaux/Ravel_Bolero at commit
// e46e0da5b2518ac9c2521308351ce02c30a4b0da, its notes and dynamics read staff by staff at
// sounding pitch, cross-checked with Ravel's autograph full score (pencil, 1928). The
// edition's MIDI is not at sounding pitch everywhere, so these are put right here, as the
// score prints them: the two piccolos of rehearsal 8 play the tune in E and in G, two
// octaves and a third and an octave and a fifth above the horn (the fifth and third
// harmonics of its C, with the celesta's two octaves), the third piccolo and the celesta
// sound an octave above their written notes, the D trumpet a tone above, and the double
// basses and the contrabassoon an octave below.
//
// 340 bars of 3/4, as printed: four bars of the snare drum and the pizzicato, then eighteen
// statements of the tunes, each eighteen bars (sixteen of tune, its last note, and a bar of
// the accompaniment alone), the last one cut short by the turn to E major (bars 327 to 334)
// and the ending (335 to 340). Each statement is a part with the instruments that play its
// tune (`played_by`), at the dynamic the score marks for them. Where they double it at
// fixed intervals (octaves; the horn's harmonics; the oboe d'amore a fifth up, in G, in
// statement 10), the tune is written
// once and played by each at its interval. Where it is harmonised in chords that move
// along the scale (statements 12, 14, 16 and 17, and the last), the melody is in chords:
// in 14 and 17, the tune and its harmony (thirds and fifths) in the octave above it, which the
// flutes, oboes, clarinets, violins and trumpets double an octave higher; in 12, 16 and
// the last, whose harmony is spread over three octaves and leaves the tune's low notes,
// every note the winds and violins play. The accompaniment is in two lines, the
// instruments that double the snare drum's rhythm, and all the rest (see the macros).
//
// Left out: the grace notes and glissandi (the trombone's and the saxophones'); the octave
// doublings in the accompaniment; in statements 14 and 17 the third piccolo's tune two
// octaves up (it would double the chords too), and the octave doubling then adds an
// octave the score has not on a few notes (bars 249 to 251, and the last note of each); in
// statement 15 the instruments that take the end of the tune (bass clarinet, violas,
// horns) are not told apart from those that start it; the clarinets' and flutes' few
// repeated sixteenths that the score writes as eighths (bars 177 to 179, 231 to 233). The
// second and third tune parts end and start on a note tied across the bar line between
// them (bars 326 to 327 and 334 to 335), struck again in the new part. The cymbals and the
// tam-tam both play on the Crash channel.
//
// Where the manuscript differs from the print, the print is followed: the manuscript ends
// the first tune on a quarter tied to an eighth (print: an eighth); it marks bars 1 and 2
// "bis" where the print writes four bars, and bars 337 and 338 of its own count, and writes
// two bars more in the coda, so it lasts four bars longer than the print's 340 (not read
// in detail); the second flute doubles the snare drum only in bars 21 to 24 there (print:
// through the clarinet's statement); the strings and the snare drum start p (print: pp).
// The tempo, ♩ = 72, "Tempo di Bolero, moderato assai", is the print's.
pub(super) const BOLERO: Song = Song {
    name: "Boléro",
    origin: "Maurice Ravel, 1928",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 72.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FLUTE, gm::PIZZICATO_STRINGS, gm::CONTRABASS),
    style: "",
    parts: &[
        Part::new("Introduction", "-:12 | -:12", "C | C")
            .with_left(concat!(bolero_beats_1!(), " & ", bolero_rest!()))
            .with_drums(&[(Drum::Snare, bolero_snare!())])
            .marked(Dynamic::Pianissimo),
        Part::new("Flute", BOLERO_A, BOLERO_C)
            .with_left(concat!(bolero_eight!(bolero_beats_1), " | ", bolero_beats_2!(), " & ", bolero_eight!(bolero_rest), " | ", bolero_rhythm_1!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::FLUTE, 0)])
            .marked(Dynamic::Pianissimo),
        Part::new("Clarinet", BOLERO_A, BOLERO_C)
            .with_left(concat!(bolero_eight!(bolero_beats_2), " | ", bolero_beats_3!(), " & ", bolero_eight!(bolero_rhythm_2), " | ", bolero_rhythm_2!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::CLARINET, 0)])
            .marked(Dynamic::Piano),
        Part::new("Bassoon", BOLERO_B, BOLERO_C)
            .with_left(concat!(bolero_four!(bolero_beats_3), " | ", bolero_beats_4!(), " | ", bolero_beats_3!(), " | ", bolero_beats_5!(), " | ", bolero_beats_6!(), " | ", bolero_beats_7!(), " & ", bolero_eight!(bolero_rhythm_2), " | ", bolero_rhythm_2!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::BASSOON, 0)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::MezzoPiano),
        Part::new("E-flat clarinet", BOLERO_B, BOLERO_C)
            .with_left(concat!(bolero_four!(bolero_beats_7), " | ", bolero_beats_8!(), " | ", bolero_beats_7!(), " | ", bolero_beats_9!(), " | ", bolero_beats_10!(), " | ", bolero_beats_11!(), " & ", bolero_eight!(bolero_rhythm_2), " | ", bolero_rhythm_1!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::CLARINET, 12)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::Piano),
        Part::new("Oboe d'amore", BOLERO_A, BOLERO_C)
            .with_left(concat!(bolero_two!(bolero_beats_12), " | ", bolero_beats_13!(), " | ", bolero_four!(bolero_beats_14), " | ", bolero_beats_15!(), " | ", bolero_beats_16!(), " & ", bolero_eight!(bolero_rhythm_2), " | ", bolero_rhythm_1!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::OBOE, 0)])
            .marked(Dynamic::MezzoPiano),
        Part::new("Trumpet and flute", BOLERO_A, BOLERO_C)
            .with_left(concat!(bolero_two!(bolero_beats_17), " | ", bolero_beats_18!(), " | ", bolero_four!(bolero_beats_19), " | ", bolero_beats_20!(), " | ", bolero_beats_21!(), " & ", bolero_eight!(bolero_rhythm_2), " | ", bolero_rhythm_1!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::MUTED_TRUMPET, 0), (gm::FLUTE, 12)])
            .marked(Dynamic::MezzoPiano),
        Part::new("Tenor saxophone", BOLERO_B, BOLERO_C)
            .with_left(concat!(bolero_four!(bolero_beats_22), " | ", bolero_beats_23!(), " | ", bolero_beats_22!(), " | ", bolero_beats_24!(), " | ", bolero_beats_25!(), " | ", bolero_beats_26!(), " & ", bolero_eight!(bolero_rhythm_2), " | ", bolero_rhythm_2!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::TENOR_SAX, 0)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::MezzoPiano),
        Part::new("Sopranino saxophone", BOLERO_B, BOLERO_C)
            .with_left(concat!(bolero_four!(bolero_beats_26), " | ", bolero_beats_27!(), " | ", bolero_beats_26!(), " | ", bolero_beats_28!(), " | ", bolero_beats_29!(), " | ", bolero_beats_30!(), " & ", bolero_eight!(bolero_rhythm_2), " | ", bolero_rhythm_3!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::SOPRANO_SAX, 12)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::MezzoPiano),
        Part::new("Horn, celesta and piccolos", BOLERO_A, BOLERO_C)
            .with_left(concat!(bolero_two!(bolero_beats_31), " | ", bolero_beats_32!(), " | ", bolero_four!(bolero_beats_33), " | ", bolero_beats_34!(), " | ", bolero_beats_35!(), " & ", bolero_eight!(bolero_rhythm_4), " | ", bolero_rhythm_5!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::FRENCH_HORN, 0), (gm::CELESTA, 12), (gm::CELESTA, 24), (gm::PICCOLO, 19), (gm::PICCOLO, 28)])
            .marked(Dynamic::MezzoForte),
        Part::new("Oboes and clarinets", BOLERO_A, BOLERO_C)
            .with_left(concat!(bolero_two!(bolero_beats_35), " | ", bolero_beats_36!(), " | ", bolero_four!(bolero_beats_37), " | ", bolero_beats_38!(), " | ", bolero_beats_39!(), " & ", bolero_eight!(bolero_rhythm_6), " | ", bolero_rhythm_7!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::OBOE, 7), (gm::OBOE, 12), (gm::ENGLISH_HORN, 0), (gm::CLARINET, 0), (gm::CLARINET, 12)])
            .marked(Dynamic::MezzoForte),
        Part::new("Trombone", BOLERO_B, BOLERO_C)
            .with_left(concat!(bolero_four!(bolero_beats_40), " | ", bolero_beats_41!(), " | ", bolero_beats_42!(), " | ", bolero_beats_43!(), " | ", bolero_beats_44!(), " | ", bolero_beats_45!(), " & ", bolero_eight!(bolero_rhythm_8), " | ", bolero_rhythm_3!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::TROMBONE, 0)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::MezzoForte),
        Part::new("Woodwinds", BOLERO_WOODWINDS, BOLERO_C)
            .with_left(concat!(bolero_two!(bolero_beats_45), " | ", bolero_beats_46!(), " | ", bolero_beats_45!(), " | ", bolero_beats_47!(), " | ", bolero_beats_45!(), " | ", bolero_beats_48!(), " | ", bolero_beats_49!(), " | ", bolero_beats_50!(), " & ", bolero_eight!(bolero_rhythm_4), " | ", bolero_rhythm_3!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::OBOE, 0), (gm::CLARINET, 0)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::Forte),
        Part::new("Violins", BOLERO_A, BOLERO_C)
            .with_left(concat!(bolero_two!(bolero_beats_51), " | ", bolero_beats_52!(), " | ", bolero_four!(bolero_beats_53), " | ", bolero_beats_54!(), " | ", bolero_beats_55!(), " & ", bolero_eight!(bolero_rhythm_4), " | ", bolero_rhythm_3!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::STRING_ENSEMBLE, 0), (gm::STRING_ENSEMBLE, 12), (gm::FLUTE, 0), (gm::FLUTE, 12), (gm::OBOE, 0), (gm::OBOE, 12), (gm::CLARINET, 0), (gm::CLARINET, 12), (gm::PICCOLO, 24)])
            .marked(Dynamic::Forte),
        Part::new("Violins in harmony", BOLERO_VIOLINS_IN_HARMONY, BOLERO_C)
            .with_left(concat!(bolero_beats_55!(), " | ", bolero_beats_56!(), " | ", bolero_beats_57!(), " | ", bolero_four!(bolero_beats_53), " | ", bolero_beats_58!(), " | ", bolero_beats_59!(), " & ", bolero_eight!(bolero_rhythm_4), " | ", bolero_rhythm_9!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::STRING_ENSEMBLE, 0), (gm::STRING_ENSEMBLE, 12), (gm::TENOR_SAX, 0), (gm::ENGLISH_HORN, 0), (gm::CLARINET, 0), (gm::OBOE, 12), (gm::FLUTE, 12)])
            .marked(Dynamic::Forte),
        Part::new("Violins and trumpet", BOLERO_B, BOLERO_C)
            .with_left(concat!(bolero_four!(bolero_beats_60), " | ", bolero_beats_61!(), " | ", bolero_beats_60!(), " | ", bolero_beats_62!(), " | ", bolero_beats_63!(), " | ", bolero_beats_64!(), " & ", bolero_eight!(bolero_rhythm_4), " | ", bolero_rhythm_4!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::STRING_ENSEMBLE, 0), (gm::STRING_ENSEMBLE, 12), (gm::TRUMPET, 0), (gm::ENGLISH_HORN, 0), (gm::OBOE, 12), (gm::FLUTE, 12), (gm::PICCOLO, 24)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::Forte),
        Part::new("Trombone and strings", BOLERO_TROMBONE_AND_STRINGS, BOLERO_C)
            .with_left(concat!(bolero_beats_65!(), " | ", bolero_beats_66!(), " | ", bolero_beats_67!(), " | ", bolero_beats_68!(), " | ", bolero_beats_69!(), " | ", bolero_beats_70!(), " | ", bolero_beats_71!(), " | ", bolero_beats_72!(), " | ", bolero_beats_73!(), " & ", bolero_four!(bolero_rhythm_4), " | ", bolero_rhythm_10!(), " | ", bolero_two!(bolero_rhythm_4), " | ", bolero_rhythm_4!(), " | ", bolero_rhythm_11!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::TROMBONE, 0), (gm::STRING_ENSEMBLE, 0), (gm::ENGLISH_HORN, 0), (gm::CLARINET, 0), (gm::STRING_ENSEMBLE, 12), (gm::OBOE, 12), (gm::FLUTE, 12), (gm::SOPRANO_SAX, 12)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::Forte),
        Part::new("Trumpets", BOLERO_TRUMPETS, BOLERO_C)
            .with_left(concat!(bolero_two!(bolero_beats_74), " | ", bolero_beats_75!(), " | ", bolero_four!(bolero_beats_76), " | ", bolero_beats_77!(), " | ", bolero_beats_78!(), " & ", bolero_two!(bolero_rhythm_12), " | ", bolero_rhythm_13!(), " | ", bolero_two!(bolero_rhythm_14), " | ", bolero_two!(bolero_rhythm_15), " | ", bolero_rhythm_16!(), " | ", bolero_rhythm_17!()))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::TRUMPET, 0), (gm::TENOR_SAX, 0), (gm::STRING_ENSEMBLE, 0), (gm::TRUMPET, 12), (gm::SOPRANO_SAX, 12), (gm::STRING_ENSEMBLE, 12), (gm::FLUTE, 12)])
            .marked(Dynamic::Fortissimo),
        Part::new("Last statement", BOLERO_LAST_STATEMENT, "C | C | C | C | C | C | C | C | C | C | C | C | C | C | Dm7 Dm7 G | Dm7 Dm7 G")
            .with_left(concat!(bolero_four!(bolero_beats_79), " | ", bolero_beats_80!(), " | ", bolero_beats_81!(), " | ", bolero_beats_82!(), " | ", bolero_beats_83!(), " & ", bolero_four!(bolero_rhythm_18), " | ", bolero_rhythm_14!(), " | ", bolero_rhythm_18!(), " | ", bolero_rhythm_19!(), " | ", bolero_rhythm_14!()))
            .with_drums(&[(Drum::Snare, bolero_eight!(bolero_snare))])
            .played_by(&[(gm::TRUMPET, 0), (gm::STRING_ENSEMBLE, 0), (gm::TENOR_SAX, 0)])
            .in_key(C, Mixolydian)
            .marked(Dynamic::Fortissimo),
        Part::new("E major", BOLERO_E_MAJOR, "E | E | E | E | E | E | E | E E G")
            .with_left(concat!(bolero_two!(bolero_beats_84), " | ", bolero_beats_84!(), " | ", bolero_beats_85!(), " & ", bolero_two!(bolero_rhythm_20), " | ", bolero_rhythm_20!(), " | ", bolero_rhythm_21!()))
            .with_drums(&[(Drum::Snare, bolero_four!(bolero_snare))])
            .played_by(&[(gm::TRUMPET, 0), (gm::STRING_ENSEMBLE, 0), (gm::TENOR_SAX, 0)])
            .in_key(E, Major)
            .marked(Dynamic::Fortissimo),
        Part::new("Ending", BOLERO_ENDING, "C | C | C | C | Fm | C")
            .with_left(concat!(bolero_beats_86!(), " | ", bolero_beats_87!(), " | ", bolero_beats_88!(), " & ", bolero_rhythm_22!(), " | ", bolero_rhythm_23!(), " | ", bolero_rest!()))
            .with_drums(&[
                (Drum::Snare, concat!(bolero_two!(bolero_snare), " | ", "x:2 x:2 -:4 (3 -:1 x x (3 x x x | x:2 -:10")),
                (Drum::Kick, "x:4 -:8 | x:4 -:8 | x:4 -:8 | x:4 -:8 | x:4 -:8 | x:2 -:10"),
                // The cymbals on the second beat, the tam-tam on the third.
                (Drum::Crash, "-:4 x:4 x:4 | -:4 x:4 x:4 | -:4 x:4 x:4 | -:4 x:4 x:4 | -:2 x:6 -:4 | x:2 -:10"),
            ])
            .played_by(&[(gm::TRUMPET, 0), (gm::STRING_ENSEMBLE, 0), (gm::TROMBONE, 0)])
            .marked(Dynamic::Fortissimo),
    ],
    form: &[
        "Introduction",
        "Introduction",
        "Flute",
        "Clarinet",
        "Bassoon",
        "E-flat clarinet",
        "Oboe d'amore",
        "Trumpet and flute",
        "Tenor saxophone",
        "Sopranino saxophone",
        "Horn, celesta and piccolos",
        "Oboes and clarinets",
        "Trombone",
        "Woodwinds",
        "Violins",
        "Violins in harmony",
        "Violins and trumpet",
        "Trombone and strings",
        "Trumpets",
        "Last statement",
        "E major",
        "Ending",
    ],
    drums_from: 0,
    about: "A Spanish dance, the bolero, slowed down and turned into one long crescendo, \
            a quarter of an hour of it. The snare drum plays the same two bars from the \
            first bar to the last, an eighth note and a quick triplet of sixteenths, over \
            and over, while the violas and cellos pluck C and G, the tonic and the \
            dominant of C major. Count it in three: the triplets fill the second half of \
            beats 1 and 2, and beat 3 is two plain eighths. Over it two tunes of sixteen \
            bars pass from instrument to instrument, each time a little louder and \
            fuller. The first stays in C major, rising from C and falling back by step; \
            the second is in C Mixolydian, C major with its seventh flattened to B \
            flat, on which it starts, and it borrows three more flat notes from the \
            Phrygian mode on C, D flat, A flat and E flat (its flattened second, sixth \
            and third), climbing high and sinking to the low C. Both hold their long notes across the \
            bar lines and land late, after the beat, so the tunes seem to sway over the \
            strict drum. Each is played twice: the flute and the clarinet play the first, \
            the bassoon, high in its range, and the little E-flat clarinet the second, \
            then the oboe d'amore and a muted trumpet with a flute, the tenor and the \
            sopranino saxophones. Then Ravel mixes colours into new ones: a horn plays \
            the tune with the celesta one and two octaves above it and two piccolos in G \
            and in E, the overtones of the horn's C, so that they blend like the stops of \
            an organ; the oboes, the oboe d'amore (in G again), the English horn and the \
            clarinets play it together; the trombone plays the second. Then the tunes are \
            harmonised in chords that move along the scale with them, the woodwinds, the \
            violins, the trombone with the strings, the trumpets, while under them more \
            and more of the orchestra joins the snare drum's rhythm, the harps and the \
            strings' chords grow, and the timpani come in. In the last statement the \
            whole orchestra plays the second tune and, for the first time, the music \
            leaves C: eight bars in E major, a jolt, then back to C, the trombones \
            sliding up, the bass drum, the cymbals and the tam-tam crashing in every bar, \
            a dissonant chord on F, and the final crash on C.",
};

// Source: Debussy's autograph manuscript full score of La Mer (1905), ink, PDF pages 9 and 11
// (the person's own copy). Bars 1 to 11 of the first movement, "De l'aube à midi sur la
// mer": the Très lent introduction up to rehearsal number 1; the rest of the movement
// follows. Sounding pitches throughout: the clarinets in A sound a minor third lower than
// written, the cor anglais in F a fifth lower, the trumpet in F a fourth higher. The tempo
// is the manuscript's own, "Très lent (116 = ♩)", a quarter note, and "(♩ = ♩)" where 4/4
// begins in bar 6. The melody is the line in front: the divided cellos' upper octave in
// bars 3 to 5, the first oboe in bars 6 to 8, the cor anglais in bars 9 to 11 with the muted
// first trumpet an octave above it. Everything else is on the Keys channel: the timpani's B
// roll and the double basses' low B, the two harps (their alternating strokes, harp 2 on
// beats 1 and 4 and harp 1 on beats 2 and 5, merged into one line), the cellos' lower octave,
// the divided violas' octaves, the bassoons' B and E sharp, the first clarinet's echo of
// the oboe, and the tremolo strings' falling B, A, G sharp, F sharp, handed down from the
// violins to the violas and cellos (the tremolos are written as held notes). Left out: the
// double basses' pizzicato B on beats 1 and 4 of bars 2 to 5, which doubles their held B;
// the cellos' first B in bar 11, which is the timpani's B on the same channel; and what
// could not be read for sure: the violas' bar 5 after its first half note (a B over C sharp
// and a chord on beat 6), the end of the cellos' bar 5, and the lower bassoon's figure in
// bars 8 and 9 (only the upper C natural is kept). No chord symbol is honest here: B is
// held in the bass while pentatonic dyads move above it, so the charts are empty.
pub(super) const LA_MER_DAWN: Song = Song {
    name: "La Mer, From Dawn to Noon on the Sea",
    origin: "Claude Debussy, La Mer, 1905",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: B,
    mode: Dorian,
    time: SIX_FOUR,
    tempo: 116.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::CELLO, gm::STRING_ENSEMBLE, gm::CONTRABASS),
    style: "",
    parts: &[
        // Bars 1 to 5: the pedal B, the harps' F sharp and G sharp, and the cellos' rocking
        // octaves.
        Part::new(
            "Dawn",
            "-:24 | -:24 | F#3:1 G#3:3 _:4 G#3:4 G#3:4 G#3:8 | \
             F#3:1 G#3:3 _:8 G#3:8 G#3:4 | F#3:1 G#3:3 _:8 G#3:4 -:8",
            "",
        )
        .with_left(
            "B2:24 | B2:24 | B2:24 | B2:24 | B2:24 \
             & B1:24 | _:24 | _:24 | _:24 | _:24 \
             & -:24 | F#2+F#3:4 F#2+F#3:8 G#2+G#3:4 G#2+G#3:8 | \
             F#2+F#3:4 F#2+F#3:8 G#2+G#3:4 G#2+G#3:8 | \
             F#2+F#3:4 F#2+F#3:8 G#2+G#3:4 G#2+G#3:8 | \
             F#2+F#3:4 F#2+F#3:8 G#2+G#3:4 G#2+G#3:8 \
             & -:24 | -:24 | F#2:1 G#2:3 _:4 G#2:4 G#2:4 G#2:8 | \
             F#2:1 G#2:3 _:8 G#2:8 G#2:4 | F#2:1 G#2:3 _:8 G#2:4 -:8 \
             & -:24 | -:24 | -:20 C#3+C#4:4 | _:8 F#3+F#4:4 _:4 G#3+G#4:8 | B3+B4:8 -:16",
        )
        .marked(Dynamic::Pianissimo),
        // Bars 6 to 8, in 4/4: the oboe's sigh, G sharp to A, answered by the clarinet,
        // over the bassoons' B and E sharp and the violins' tremolo.
        Part::new(
            "Oboe",
            "-:8 G#4:1 A4:3 _:4 | _:8 G#4:8 | G#4:1 F#4:3 _:12",
            "",
        )
        .with_left(
            "B2:16 | B2:16 | B2:16 \
             & B1:8 -:8 | B1:8 -:8 | B1:8 -:8 \
             & -:8 E#3+B3:8 | _:8 -:8 | C4:16 \
             & -:16 | D4:1 D#4:3 _:12 | D#4:1 E4:3 _:12 \
             & B5+B6:16 | B5+B6:16 | B5+B6:4 A5+A6:4 G#5+G#6:4 F#5+F#6:4",
        )
        .in_time(COMMON)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Piano),
        // Bars 9 to 11: the call of the cor anglais and the muted trumpet, on C, while
        // the falling tremolo line goes down through the strings.
        Part::new("Call", "-:8 C4:8 | (3 _:4 C4 C4 C4:8 | C4:4 -:12", "")
            .with_left(
                "F#4:4 -:12 | -:16 | -:16 \
                 & E4:4 -:12 | -:16 | -:16 \
                 & C4:4 -:12 | -:16 | -:16 \
                 & B2:16 | B2:16 | B2:16 \
                 & B1:8 -:8 | -:16 | -:16 \
                 & B5:4 A5:4 G#5:4 F#5:4 | -:16 | -:16 \
                 & B4:4 A4:4 G#4:4 F#4:4 | B4:4 A4:4 G#4:4 F#4:4 | -:16 \
                 & -:16 | B3:4 A3:4 G#3:4 F#3:4 | B3:4 A3:4 G#3:4 F#3:4 \
                 & -:16 | -:16 | -:4 A2:4 G#2:4 F#2:4",
            )
            .in_time(COMMON)
            .in_key(C, MinorPentatonic)
            .played_by(&[(gm::ENGLISH_HORN, 0), (gm::MUTED_TRUMPET, 12)])
            .marked(Dynamic::Pianissimo),
    ],
    form: &["Dawn", "Oboe", "Call"],
    drums_from: 0,
    about: "The first bars of La Mer, bars 1 to 11, the slow introduction up to rehearsal \
            number 1; the rest of the movement follows. Dawn on the sea: the timpani roll a \
            B and the double basses hold the B below it for five bars of 6/4, \"très lent\", \
            while the two harps strike F sharp and G sharp, an octave apart, one beat after \
            the other, and the cellos, divided in octaves, rock between F sharp and G sharp. \
            Nothing here is a chord in the usual sense: a held bass and, above it, notes of a \
            pentatonic scale (B, C sharp, F sharp, G sharp), so the chord sheet stays empty. \
            In bar 6 the meter turns to 4/4 at the same beat and the oboe sighs G sharp to A \
            and back, the clarinet echoes it a step lower, and the bassoons hold B with E \
            sharp, a tritone, the interval of the whole-tone scale. The key signature is B \
            minor's, but the G sharp makes it B Dorian. Then the cor anglais and the first \
            trumpet, muted, an octave apart, call on a single C, a semitone above the pedal \
            B: the first notes of the movement's motto, which goes on in a pentatonic scale \
            on C (C, E flat, F, G, B flat), so this part is in C minor pentatonic over the B. \
            Under the call, a falling line, B, A, G sharp, F sharp, trembles down through the \
            strings, from the violins to the violas and the cellos, an octave lower each \
            time.",
};

// Source: Jun Nishio's sequence of Part I of The Rite of Spring for the Roland SC-88,
// converted by script, checked against the full orchestral score (Russischer Musikverlag,
// 1921, pages 11 to 27): rehearsal 13 to 37, 172 bars. Each rehearsal number is a part,
// split where its tune passes to another instrument. The tune of each passage is the
// melody, played by the instrument that has it: the eight horns' accented chords at 13, 14
// and 18, the cor anglais' or the trumpet's ostinato, the bassoons', the oboes', the
// horn's, the flutes', the trumpets', the violins' and the piccolos' tunes. Everything
// else (the chords, the bass, the ostinatos, the instruments answering the tune, the
// timpani) is the accompaniment, on the strings. Quantised to sixteenths, triplets kept.
// The dynamics come from the sequence's velocities: each part's from its tune's, the
// accompaniment's marks bar by bar against it, a sforzando or an accent on a chord struck
// harder than the rest of its bar. Where the sequence differs from the score, the score is
// followed:
// - the tempo is the score's, "Tempo giusto", a half note 56 a minute (a quarter 112),
//   steady throughout, not the sequence's rubato; the two bars before 22 get the score's
//   fermatas, over their second beat, instead of the sequence's slowing down;
// - the two bars of 3/4 before 28, which the sequence writes as three bars of 2/4, are in
//   3/4, their notes unchanged;
// - the horns' accents of 13, 14 and 18 are the strings' chord, F flat major under E flat
//   seventh, as the score writes them (horns 1 to 4 on E flat seventh, horns 5 to 8 on F
//   flat major); the sequence gives the low horns A, C sharp, E, G and B flat, a chord
//   the score does not have;
// - from 33 the parts are as loud as the score marks them, f and then ff, where the
//   sequence keeps its piccolos at mf.
// Left out: the trills held under the music (the bassoons' at 16, 17, 28 and 29, the two
// solo violins' at 24 to 27, the clarinets' at 28 and 29) and the triangle; the bass
// drum's one stroke, before 22, is the only drum. Every instrument of the accompaniment is
// played by the strings.
pub(super) const AUGURS_OF_SPRING: Song = Song {
    name: "The Rite of Spring, The Augurs of Spring",
    origin: "Igor Stravinsky, The Rite of Spring, 1913",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: DSharp,
    mode: Mixolydian,
    time: TWO_FOUR,
    tempo: 112.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FRENCH_HORN, gm::STRING_ENSEMBLE, gm::CONTRABASS),
    style: "",
    parts: &[
        Part::new(
            "13",
            "-:8 | -:8 | -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:3 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | -:8 | \
             -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:5 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:7 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:7 | -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:5",
            "",
        )
        .with_left(
            "E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1",
        )
        .marked(Dynamic::Fortissimo),
        Part::new(
            "14",
            "Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1",
            "",
        )
        .with_left(
            "!mp G3:1 E3:1 C3:1 G2:1 E2:1 G2:1 B2:1 E3:1 | G3:1 E3:1 C3:1 G2:1 E2:1 G2:1 B2:1 E3:1 | \
             G3:1 E3:1 C3:1 G2:1 E2:1 G2:1 B2:1 E3:1 | G3:1 E3:1 C3:1 G2:1 E2:1 G2:1 B2:1 E3:1 & \
             !mp E2:2 Ab2:2 B2:2 E3:2 | G3:2 E3:2 C3:2 G2:2 | \
             E2:2 Ab2:2 B2:2 E3:2 | G3:2 E3:2 C3:2 G2:2",
        )
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "14, bar 5",
            "-:8 | -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:3 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | -:8 | -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:5",
            "",
        )
        .with_left(
            "E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1",
        )
        .marked(Dynamic::Fortissimo),
        Part::new(
            "15",
            "-:8 | (3 Bb4:2 Bb4:2 Bb4:2 Bb4:4 | (3 _:2 A4:2 Ab4:2 G4:1 -:1 Gb4:1 -:1 | Bb4:1 -:1 A4:1 -:1 (3 Ab4:2 G4:2 -:2 | \
             (3 Bb4:2 A4:2 Ab4:2 (3 G4:2 Gb4:2 F4:2 | (3 E4:2 E4:2 E4:2 E4:3 -:1",
            "",
        )
        .with_left(
            "E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 & \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 & \
             -:8 | (3 Bb5:2 Bb5:2 Bb5:2 Bb5:4 | \
             _:1 -:7 | Bb5:1 -:1 A5:1 -:1 (3 Ab5:2 G5:2 Gb5:2 | \
             -:8 | -:8 & \
             -:2 F6+C7+F7:1 -:3 G6+C7+G7:1 -:1 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             -:2 B4+E5+B5:1 -:3 Ab4+Db5+Ab5:1 -:1 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             -:2 E5+B5:2 -:2 Db5+Ab5:2 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8",
        )
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "16",
            "Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1",
            "",
        )
        .with_left(
            "-:8 | -:8 | \
             -:4 C4+C5:4 | _:4 C4+C5:4 | \
             _:4 C4+C5:4 | _:4 C4+C5:4 | \
             _:4 C4+C5:4 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff Bb3+F4+C5+G5+D6:2 (3 _:1 _:1 -:1 (3 Bb3+F4+C5+G5+D6:1 Bb3+F4+C5+G5+D6:1 Bb3+F4+C5+G5+D6:1 | \
             -:8 | -:4 (3 Bb3+F4+C5+G5+D6:1 Bb3+F4+C5+G5+D6:1 Bb3+F4+C5+G5+D6:1 (3 -:1 -:1 -:1 | \
             -:2 Bb3+F4+C5+G5+D6:2 (3 _:1 _:1 -:1 (3 Bb3+F4+C5+G5+D6:1 Bb3+F4+C5+G5+D6:1 Bb3+F4+C5+G5+D6:1 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             !ff (3 C6+C7:1 C7+C8:1 C6+C7:1 (3 Bb5+Bb6:1 C6+C7:1 Bb5+Bb6:1 (3 A5+A6:1 Bb5+Bb6:1 A5+A6:1 (3 G5+G6:1 G6+G7:1 G5+G6:1 | A5+A6:1 -:7 | \
             -:8 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff >Eb3:2 (3 _:1 _:1 -:1 (3 >Eb3:1 Eb3:1 Eb3:1 | \
             -:8 | -:4 (3 >Eb3:1 Eb3:1 Eb3:1 (3 -:1 -:1 -:1 | \
             -:2 >Eb3:2 (3 _:1 _:1 -:1 (3 >Eb3:1 Eb3:1 Eb3:1 & \
             Eb2:2 Bb2:2 F3:2 C4:2 | F3:2 Bb2:2 Eb2:2 Bb2:2 | \
             F3:2 C4:2 F3:2 Bb2:2 | Eb2:2 >Bb2+Eb3+Bb3+F4+C5+G5+D6:2 F3:2 C4:2 | \
             F3+C5:2 Bb2+Bb5:2 Eb2+A4:2 Bb2+G5:2 | F3+A4:2 C4:2 F3:2 Bb2:2 | \
             Eb2:2 >Bb2+Eb3+Bb3+F4+C5+G5+D6:2 F3:2 C4:2 & \
             -:8 | !mp (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | \
             (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | \
             (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | \
             (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             !mp (3 C7:1 B6:1 Bb6:1 (3 A6:1 Ab6:1 G6:1 (3 Gb6:1 F6:1 E6:1 (3 Eb6:1 D6:1 Db6:1 | C6:1 -:7 | \
             -:8 & \
             -:8 | -:8 | \
             -:8 | -:4 (3 -:1 -:1 -:1 (3 Eb3+Bb3+F4+C5:1 Bb3+F4+C5+G5:1 F4+C5+G5+D6:1 | \
             -:8 | -:4 (3 Eb3+Bb3+F4+C5:1 Bb3+F4+C5+G5:1 F4+C5+G5+D6:1 (3 -:1 -:1 -:1 | \
             -:4 (3 -:1 -:1 -:1 (3 Eb3+Bb3+F4+C5:1 Bb3+F4+C5+G5:1 F4+C5+G5+D6:1",
        )
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "17",
            "Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1",
            "",
        )
        .with_left(
            "-:4 C4+C5:4 | _:4 C4+C5:4 | \
             _:4 C4+C5:4 | _:4 C4+C5:4 | \
             _:4 C4+C5:4 & \
             -:8 | -:8 | \
             -:8 | -:4 !ff (3 -:2 C5:2 C5+F5:2 | \
             -:2 G4+C5+F5+Bb5:1 -:1 (3 -:2 -:2 >G4+C5+F5+Bb5+Eb6:2 & \
             (3 -:1 -:1 -:1 !ff (3 G5+G6:1 G6+G7:1 G5+G6:1 (3 A5+A6:1 Bb5+Bb6:1 A5+A6:1 (3 Bb5+Bb6:1 C6+C7:1 Bb5+Bb6:1 | (3 C6+C7:1 C7+C8:1 C6+C7:1 (3 Bb5+Bb6:1 C6+C7:1 Bb5+Bb6:1 (3 A5+A6:1 Bb5+Bb6:1 A5+A6:1 (3 G5+G6:1 G6+G7:1 G5+G6:1 | \
             (3 A5+A6:1 Bb5+Bb6:1 A5+A6:1 (3 Bb5+Bb6:1 C6+C7:1 Bb5+Bb6:1 (3 C6+C7:1 C7+C8:1 C6+C7:1 (3 Bb5+Bb6:1 C6+C7:1 Bb5+Bb6:1 | (3 A5+A6:1 Bb5+Bb6:1 A5+A6:1 (3 G5+G6:1 G6+G7:1 G5+G6:1 A5+A6:1 -:3 | \
             -:8 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:4 !ff (3 -:2 -:2 D4:2 & \
             F3:2 Bb2+G5:2 Eb2+A4:2 Bb2+Bb5:2 | F3+C5:2 C4+Bb5:2 F3+A4:2 Bb2+G5:2 | \
             Eb2+A4:2 Bb2+Bb5:2 F3+C5:2 C4+Bb5:2 | F3+A4:2 Bb2+G5:2 (3 Eb2:1 _:1 _:1 (3 Bb2:1 _:1 _:1 | \
             F3:2 >A3+C4+D4+G4+C5+F5+Bb5:2 (3 F3:1 _:1 _:1 (3 Bb2:1 _:1 _:1 & \
             -:8 | -:8 | \
             -:8 | -:4 (3 A4:1 _:1 B2+E3+A3+D4+G4+C5:1 (3 _:1 E3+A3+D4+G4+C5+F5:1 _:1 | \
             -:4 (3 -:1 -:1 -:1 !ff (3 -:1 D4+G4+C5+F5+Bb5+Eb6:1 _:1 & \
             !mp (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | \
             (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 | \
             (3 E4:1 E4:1 C4:1 (3 C4:1 E4:1 E4:1 (3 G4:1 G4:1 E4:1 (3 E4:1 C4:1 C4:1 & \
             (3 -:1 -:1 -:1 !mp (3 Ab4:1 A4:1 Bb4:1 (3 B4:1 C5:1 Db5:1 (3 D5:1 Eb5:1 E5:1 | F5:1 -:7 | \
             (3 F4+C6:1 Gb4+Db6:1 G4+D6:1 (3 Ab4+Eb6:1 A4+E6:1 Bb4+F6:1 (3 B4+Gb6:1 C5+G6:1 Db5+Ab6:1 (3 D5+A6:1 -:1 -:1 | -:8 | \
             -:8 & \
             -:8 | !mp (3 G5+C6:1 Gb5+B5:1 F5+Bb5:1 (3 E5+A5:1 Eb5+Ab5:1 D5+G5:1 (3 Db5+Gb5:1 C5+F5:1 B4+E5:1 (3 Bb4+Eb5:1 A4+D5:1 Ab4+Db5:1 | \
             !mf G4+C5:1 -:3 (3 C6:1 B5:1 Bb5:1 (3 E5+A5:1 Eb5+Ab5:1 D5+G5:1 | !mp (3 Ab4+Db5+Gb5:1 G4+C5+F5:1 Gb4+B4+E5:1 (3 C4+F4+Bb4+Eb5:1 B3+E4+A4+D5:1 Bb3+Eb4+Ab4+Db5:1 A3+D4+G4+C5:1 -:3 | \
             -:8",
        )
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "18",
            "-:8 | -:8 | -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:3 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | -:8 | \
             -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:5 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:7 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:7 | -:2 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:5",
            "",
        )
        .with_left(
            "!sfz E2+Ab2+B2+E3+G3+Bb3+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1",
        )
        .marked(Dynamic::Fortissimo),
        Part::new(
            "19",
            "-:2 !f Bb3:1 -:1 Bb3:1 -:1 Bb3:1 -:1 | !mf Bb3:1 -:1 >Bb3:1 -:1 Bb3:1 -:1 >Bb3:1 -:1 | >Bb3:1 -:1 Ab3:1 -:1 G3:1 -:1 F3:1 -:1 | G3:1 -:7 | \
             -:8 | -:2 F3:1 -:1 G3:1 -:1 Ab3:1 -:1 | Bb3:1 -:1 Ab3:1 -:1 G3:1 -:1 F3:1 -:1 | G3:1 -:7 | \
             -:8",
            "",
        )
        .with_left(
            "!pp E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | !ff E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             !pp E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | !f E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 >E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1",
        )
        .played_by(&[(gm::BASSOON, 0)]),
        Part::new(
            "20",
            "-:8 | -:6 F3:1 -:1 | G3:1 -:1 Ab3:1 -:1 Bb3:1 -:1 Ab3:1 -:1 | G3:1 -:1 F3:1 -:1 G3:1 -:3 | \
             -:8 | -:8 | -:8 | -:8",
            "",
        )
        .with_left(
            "!pp >E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             !f E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 !sfz E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 & \
             -:2 !f F3:1 -:1 G3:1 -:1 Ab3:1 -:1 | Bb3:1 -:1 Ab3:1 -:1 G3:1 -:1 F3:1 -:1 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8",
        )
        .played_by(&[(gm::BASSOON, 0)]),
        Part::new(
            "21",
            "-:8 | -:8 | -:2 Bb5:1 -:1 Bb5:1 -:1 Bb5:1 -:1 | Bb5:1 -:1 Bb5:1 -:1 Bb5:1 -:1 Bb5:1 -:1 | \
             Bb5:1 -:1 Ab5:1 -:1 G5:1 -:1 F5:1 -:1 | G5:1 -:7 | -:2 F5:1 -:1 G5:1 -:1 Ab5:1 -:1 | Bb5:1 -:1 Ab5:1 -:1 G5:1 -:1 F5:1 -:1 | \
             G5:1 -:1 Ab5:1 -:1 Bb5:1 -:1 Ab5:1 -:1 | G5:1 -:1 F5:1 -:1 G5:1 -:3 | -:8 | -:8",
            "",
        )
        .with_left(
            "!pp >E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 E2+Ab2+B2+E3+G3+Bb3+Db4+Eb4:1 -:1 | \
             -:8 | !mf (3 D3+D4:2 Gb2+Gb4:2 G2:2 F1+F2:1 -:3 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:6 !f F3:1 -:1 | \
             G3:1 -:1 Ab3:1 -:1 Bb3:1 -:1 Ab3:1 -:1 | G3:1 -:1 F3:1 -:1 G3:1 -:3 | \
             -:8 | -:8 | \
             -:4 !ff Eb3+Eb4:1 -:3 | -:8 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:2 !f Ab5+Bb6:1 -:3 F5+G6:1 -:1 | -:8 | \
             -:2 F5+G6:1 -:3 Ab5+Bb6:1 -:1 | -:2 Ab5+Bb6:1 -:3 F5+G6:1 -:1 | \
             -:2 Ab5+Bb6:1 -:3 Ab5+Bb6:1 -:1 | -:2 F5+G6:1 -:5 | \
             -:8 | -:4 (3 -:1 -:1 -:1 !mf (3 -:1 B6:1 C7+D7:1 & \
             -:2 !f Bb2+Bb3:1 -:1 Bb2+Bb3:1 -:1 Bb2+Bb3:1 -:1 | !mf Bb2+Bb3:1 -:1 >Bb2+Bb3:1 -:1 Bb2+Bb3:1 -:1 >Bb2+Bb3:1 -:1 | \
             >Bb2+Bb3:1 -:1 Ab2+Ab3:1 -:1 G2+G3:1 -:1 F2+F3:1 -:1 | G2+G3:1 -:7 | \
             -:2 F2+F3:1 -:1 G2+G3:1 -:1 Ab2+Ab3:1 -:1 | Bb2+Bb3:1 -:1 Ab2+Ab3:1 -:1 G2+G3:1 -:1 F2+F3:1 -:1 | \
             G2+G3:1 -:7 | -:2 F2+F3:1 -:1 G2+G3:1 -:1 Ab2+Ab3:1 -:1 | \
             Bb2+Bb3:1 -:1 Ab2+Ab3:1 -:1 G2+G3:1 -:1 F2+F3:1 -:1 | G2+G3:1 -:7 | \
             -:8 | -:8 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:4 !ff Eb1+Eb2+Eb3+Eb4:4 | -:4 F1+F2:4 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:2 C2+C3+G3+C4+Bb4+Db5+Eb5+Gb5+Bb5:2 -:4 | -:8 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | !ff (3 D3:2 D3:2 G2:2 -:4",
        )
        .with_tempo(&[fermata(84, 4), fermata(92, 4)])
        .with_drums(&[(Drum::Kick, "-:8 | -:8 | -:8 | -:8 | -:8 | -:8 | -:8 | -:8 | -:8 | -:8 | -:8 | -:4 X:4")])
        .played_by(&[(gm::OBOE, 0)]),
        Part::new(
            "22",
            "-:4 !ff Eb6:1 -:1 Bb5:1 -:1 | !mf Db5:1 -:1 Bb4:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1",
            "",
        )
        .with_left(
            "-:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:4 (3 -:1 -:1 -:1 !ff (3 -:1 -:1 G4+A4:1 & \
             !ff Db6:1 -:1 Bb5:1 -:1 Eb5:1 -:1 Bb4:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             !ff (3 E7:1 _:1 _:1 (3 Bb6:1 E6:1 _:1 B5:2 -:2 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             !ff (3 >Db7:1 -:1 -:1 (3 -:1 Ab6+B6:1 D6:1 -:4 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             -:8 | !mp Db4:1 -:1 Bb3:1 -:1 Eb5:1 -:1 Bb4:1 -:1 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             -:4 !ff G5+A5:1 G5:1 G5+A5:1 G5:1 | (3 G5+A5:1 C6:1 G5:1 (3 E5:1 C5:1 G4:1 C4:1 -:3 | \
             -:8 | -:8 | \
             -:8 | -:8",
        )
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "23",
            "Db5:1 -:1 Bb4:1 -:1 Eb5:1 -:1 Bb4:1 -:1 | Db5:1 -:1 Bb4:1 -:1 Eb5:1 -:1 Bb4:1 -:1 | Db5:1 -:1 Bb4:1 -:1 Eb5:1 -:1 Bb4:1 -:1 | -:8 | \
             Db5:1 -:1 Bb4:1 -:1 Eb5:1 -:1 Bb4:1 -:1",
            "",
        )
        .with_left(
            "(3 C5+Db5:1 _:1 -:1 (3 -:1 -:1 G4+A4:1 (3 C5+Eb5:1 _:1 -:1 (3 -:1 -:1 G4+A4:1 | (3 C5+Db5:1 _:1 -:1 (3 -:1 -:1 G4+A4:1 (3 C5+Eb5:1 _:1 -:1 (3 -:1 -:1 G4+A4:1 | \
             (3 C5+Db5:1 _:1 -:1 (3 -:1 -:1 G4+A4:1 B4+C5+Eb5:1 -:3 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 G4+A4:1 | \
             (3 C5+Db5:1 _:1 -:1 (3 -:1 -:1 G4+A4:1 B4+C5+Eb5:1 -:3 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 B4:1 -:1 -:1 (3 -:1 -:1 -:1 | (3 B4:1 -:1 -:1 (3 -:1 -:1 -:1 (3 B4:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 B4:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 B4:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 & \
             -:8 | -:8 | \
             -:8 | !mp Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             -:8 & \
             -:4 !mp E4:1 G4:1 C5:1 E5:1 | G5:1 E5:1 C5:1 G4:1 E4:1 G4:1 C5:1 E5:1 | \
             G5:1 E5:1 C5:1 G4:1 E4:1 G4:1 C5:1 E5:1 | G5:1 E5:1 C5:1 G4:1 E4:1 G4:1 C5:1 E5:1 | \
             G5:1 -:7 & \
             -:8 | -:8 | \
             !mp Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | -:8 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1",
        )
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "24",
            "Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             -:8",
            "",
        )
        .with_left(
            "!p C4:2 C4:2 C4:2 C4:2 | C4:2 C4:2 C4:2 C4:2 | \
             C4+F4:2 C4+F4:2 C4+F4:2 C4+F4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             !p Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1",
        )
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "25",
            "-:8 | G4:2 A4:1 F4:1 G4:2 C5:2 | Bb4:2 A4:1 G4:1 A4:2 F4:2 | G4:5 C5:1 A4:2 | \
             G4:8",
            "",
        )
        .with_left(
            "!p C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 & \
             !p Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1",
        )
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "25, bar 6",
            "G5:2 C6:2 F6:1 Bb5:1 C6:1 F5:1 | G5:2 C6:2 Bb5:2 A5:1 F5:1 | (3 G5:1 _:1 C6:1 (3 A5:1 _:1 _:1 G5:4 | _:6 -:2 | \
             -:8 | -:8",
            "",
        )
        .with_left(
            "-:8 | -:8 | \
             (3 -:1 -:1 C5:1 (3 A4:1 _:1 _:1 G4:4 | _:6 -:2 | \
             -:8 | -:8 & \
             !p C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 & \
             !p Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1",
        )
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "26",
            "(3 E6:2 E6:2 E6:2 E6:4 | _:1 -:1 Eb6:1 -:1 Gb5:1 -:3 | -:8 | (3 E6:2 E6:2 E6:2 -:2 Eb6:1 -:1 | \
             Gb5:1 -:7 | -:8 | -:8 | -:8",
            "",
        )
        .with_left(
            "-:8 | -:4 D5:4 | \
             (3 -:2 Db5:2 C5:2 B4:1 -:1 Bb4:1 -:1 | -:8 | \
             (3 D5:2 Db5:2 C5:2 B4:1 -:1 Bb4:1 -:1 | -:8 | \
             -:8 | -:8 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | !ff D5:1 -:1 Db5:1 -:1 (3 C5:2 B4:2 Bb4:2 | \
             -:8 | -:8 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             !mp F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 & \
             !p C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | C4+F4+G4:2 C4+F4+A4:2 C4+F4+G4:2 C4+F4+A4:2 | \
             C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 & \
             !p Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1",
        )
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "27",
            "Bb4:2 C5:1 Ab4:1 Bb4:2 Eb5:2 | Db5:2 C5:1 Bb4:1 C5:2 Ab4:2 | Bb4:1 -:1 Eb5:1 -:1 Bb4:4 | _:1 -:1 Eb6:1 -:1 Bb5:4 | \
             _:1 -:1 Eb5:1 -:1 Bb4:2 Eb5:2 | Db5:2 C5:1 Bb4:1 C5:2 Ab4:2 | Bb4:2 Eb5:2 Db5:2 C5:1 Bb4:1 C5:2 Ab4:2 | Bb4:1 -:1 Eb5:1 -:1 Bb4:5 -:1 Eb6:1 -:1",
            "",
        )
        .with_left(
            "!mp Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 | \
             Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 Db4:1 -:1 Bb3:1 -:1 | Eb4:1 -:1 Bb3:1 -:1 Db4:1 -:1 Bb3:1 -:1 Eb4:1 -:1 Bb3:1 -:1 & \
             !p F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | \
             F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | \
             F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | \
             F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 | F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 F4+Bb4:1 -:1 G4+C5:1 -:1 & \
             !pp C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | \
             C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | \
             C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | \
             C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 | C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 C4+F4+Bb4:2 C4+F4+G4+C5:2 & \
             !pp Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 | \
             Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1 Db4:1 Db4:1 Bb3:1 Bb3:1 | Eb4:1 Eb4:1 Bb3:1 Bb3:1 Db4:1 Db4:1 Bb3:1 Bb3:1 Eb4:1 Eb4:1 Bb3:1 Bb3:1",
        )
        .in_meters(&[TWO_FOUR, TWO_FOUR, TWO_FOUR, TWO_FOUR, TWO_FOUR, TWO_FOUR, THREE_FOUR, THREE_FOUR])
        .played_by(&[(gm::FLUTE, 0)]),
        Part::new(
            "28",
            "Bb5:2 C6:1 Ab5:1 Bb5:2 Eb6:2 | Db6:2 C6:1 Bb5:1 C6:2 Ab5:2 | Bb5:2 Eb6:2 Ab6:1 Db6:1 Eb6:1 Ab5:1 | Bb5:2 Eb6:2 Db6:2 C6:1 Ab5:1 | \
             Bb5:2 C6:1 Ab5:1 Bb5:2 Eb6:2 | Db6:2 C6:1 Bb5:1 C6:2 Ab5:2 | Bb5:2 Eb6:2 Db6:2 C6:1 Bb5:1 | C6:2 Ab5:2 Bb5:2 Eb6:2 | \
             Ab6:1 Db6:1 Eb6:1 Ab5:1 Bb5:2 Eb6:2 | Ab6:1 Db6:1 Eb6:1 Ab5:1 Bb5:2 Eb6:2",
            "",
        )
        .with_left(
            "-:8 | -:8 | \
             -:8 | -:8 | \
             Db4+F4+Gb4+Bb4:4 Db4+F4+Gb4+Bb4:4 | Db4+F4+Gb4+Bb4:4 Db4+F4+Gb4+Bb4:4 | \
             Eb4+Gb4+Ab4+C5:2 F4+Ab4+Bb4+Db5:2 Gb4+Bb4+C5+Eb5:4 | F4+Ab4+Bb4+Db5:4 Eb4+Gb4+Ab4+C5:4 | \
             Db4+F4+Gb4+Bb4:4 Db4+F4+Gb4+Bb4:4 | Eb4+Gb4+Ab4+C5:2 F4+Ab4+Bb4+Db5:2 Gb4+Bb4+C5+Eb5:3 -:1 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             !mp Bb4:4 Bb4:4 | Bb4:4 Bb4:4 | \
             C5:2 Db5:2 Eb5:4 | Db5:4 C5:4 | \
             Bb4:4 Bb4:4 | C5:2 Db5:2 Eb5:3 -:1 & \
             !mp Db2:8 | _:8 | \
             _:8 | _:8 | \
             Db2:8 | _:8 | \
             _:8 | _:8 | \
             Db2:8 | _:8 & \
             !mp G2+Bb2:8 | _:8 | \
             _:8 | _:8 | \
             G2+Bb2:8 | _:8 | \
             _:8 | _:8 | \
             G2+Bb2:8 | _:8 & \
             !mp Eb2:1 -:1 Bb2:1 -:1 F3:1 -:1 C4:1 -:1 | F3:1 -:1 Bb2:1 -:1 Eb2:1 -:1 Bb2:1 -:1 | \
             F3:1 -:1 C4:1 -:1 F3:1 -:1 Bb2:1 -:1 | Eb2:1 -:1 Bb2:1 -:1 F3:1 -:1 C4:1 -:1 | \
             F3:1 -:1 Bb2:1 -:1 Eb2:1 -:1 Bb2:1 -:1 | !mf F3:1 -:1 C4:1 -:1 F3:1 -:1 Bb2:1 -:1 | \
             Eb2:1 -:1 Bb2:1 -:1 F3:1 -:1 C4:1 -:1 | F3:1 -:1 Bb2:1 -:1 Eb2:1 -:1 Bb2:1 -:1 | \
             F3:1 -:1 C4:1 -:1 F3:1 -:1 Bb2:1 -:1 | Eb2:1 -:1 Bb2:1 -:1 F3:1 -:1 C4:1 -:1 & \
             !p Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             !mp Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 & \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 F5+Gb5:1 | \
             Ab5+Bb5:1 Ab5:1 Gb5:1 F5:1 Eb5:1 Db5:1 C5:1 Bb4:1 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 Db5+Eb5+F5+Gb5:1 | \
             F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4:1 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 Db5+Eb5+F5+Gb5:1",
        )
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "29",
            "Db4+F4+Gb4+Bb4:4 Db4+F4+Gb4+Bb4:4 | Eb4+Gb4+Ab4+C5:2 F4+Ab4+Bb4+Db5:2 Gb4+Bb4+C5+Eb5:4 | F4+Ab4+Bb4+Db5:4 Eb4+Gb4+Ab4+C5:4 | Db4+F4+Gb4+Bb4:4 Db4+F4+Gb4+Bb4:4 | \
             Eb4+Gb4+Ab4+C5:2 F4+Ab4+Bb4+Db5:2 Gb4+Bb4+C5+Eb5:4 | -:2 Db4+F4+Gb4+Bb4:6 | Db4+F4+Gb4+Bb4:4 Db4+F4+Gb4+Bb4:4 | Eb4+Gb4+Ab4+C5:2 F4+Ab4+Bb4+Db5:2 Gb4+Bb4+C5+Eb5:4",
            "",
        )
        .with_left(
            "Ab6:8 | Ab6:8 | \
             Ab6:8 | Ab6:8 | \
             Ab6:8 | Ab6:8 | \
             Ab6:8 | Ab6:8 & \
             -:4 Bb6:4 | _:4 Bb6:4 | \
             _:4 Bb6:4 | _:4 Bb6:4 | \
             _:4 Bb6:4 | _:4 Bb6:4 | \
             _:4 Bb6:4 | _:4 Bb6:4 & \
             Db5+Db6:2 C5+C6:1 Bb4+Bb5:1 C5+C6:2 Ab4+Ab5:2 | Bb4+Bb5:2 C5+C6:2 Bb4+Bb5:2 Eb5+Eb6:2 | \
             Db5+Db6:2 C5+C6:1 Bb4+Bb5:1 C5+C6:2 Ab4+Ab5:2 | Bb4+Bb5:2 C5+C6:1 Ab4+Ab5:1 Bb4+Bb5:2 Eb5+Eb6:2 | \
             Db5+Db6:2 C5+C6:1 Bb4+Bb5:1 C5+C6:2 Ab4+Ab5:2 | Bb4+Bb5:2 Eb5+Eb6:2 Db5+Db6:2 C5+C6:1 Ab4+Ab5:1 | \
             Bb4+Bb5:2 Eb5+Eb6:2 Ab5+Ab6:1 Db5+Db6:1 Eb5+Eb6:1 Ab4+Ab5:1 | Bb4+Bb5:2 Eb5+Eb6:2 Db5+Db6:2 C5+C6:1 Ab4+Ab5:1 & \
             !mp Bb4:4 Bb4:4 | C5:2 Db5:2 Eb5:4 | \
             Db5:4 C5:4 | Bb4:4 Bb4:4 | \
             C5:2 Db5:2 Eb5:4 | -:2 Bb4:6 | \
             Bb4:4 Bb4:4 | C5:2 Db5:2 Eb5:4 & \
             -:8 | Bb4+Bb5:1 -:1 C5+C6:1 -:1 Bb4+Bb5:1 -:1 Eb5+Eb6:1 -:1 | \
             !ff Gb4:8 | Gb4:8 | \
             Gb4:8 | Gb4:8 | \
             Gb4:8 | Gb4:8 & \
             -:8 | -:8 | \
             C5:2 C5+C6:1 -:1 C5+C6:1 -:1 Ab4+Ab5:1 -:1 | C5:2 C5+C6:1 -:1 Bb4+Bb5:1 -:1 Eb5+Eb6:1 -:1 | \
             C5:2 C5+C6:1 -:1 C5+C6:1 -:1 Ab4+Ab5:1 -:1 | C5:4 Db5+Db6:1 -:1 C5+C6:1 -:1 | \
             C5:4 Ab5:1 -:1 Eb5+Eb6:1 -:1 | C5:4 Db5+Db6:1 -:1 C5+C6:1 -:1 & \
             -:8 | -:8 | \
             !ff C4+Ab4+Db5+Db6:1 -:7 | C4+Ab4+Bb4+Bb5:1 -:7 | \
             C4+Ab4+Db5+Db6:1 -:7 | !mf C4+Ab4+Bb4+Bb5:1 -:1 Eb5+Eb6:1 -:5 | \
             C4+Ab4+Bb4+Bb5:1 -:1 Eb5+Eb6:1 -:5 | C4+Ab4+Bb4+Bb5:1 -:1 Eb5+Eb6:1 -:5 & \
             -:8 | -:8 | \
             !mp Db2:8 | _:8 | \
             _:8 | _:8 | \
             Db2:8 | _:8 & \
             -:8 | -:8 | \
             !ff Eb1+G2+Bb2+Db4:8 | Eb1+G2+Bb2+Db4:8 | \
             Eb1+G2+Bb2+Db4:8 | Eb1+G2+Bb2+Db4:8 | \
             Eb1+G2+Bb2+Db4:8 | Eb1+G2+Bb2+Db4:8 & \
             -:8 | -:8 | \
             !mp G2+Bb2:8 | _:8 | \
             _:8 | _:8 | \
             G2+Bb2:8 | _:8 & \
             F3:1 -:1 Bb2:1 -:1 Eb2:1 -:1 Bb2:1 -:1 | !ff F3:1 -:1 C4:1 -:1 F3:1 -:1 Bb2:1 -:1 | \
             Eb2:1 -:1 Bb2:1 -:1 F3:1 -:1 C4:1 -:1 | F3:1 -:1 Bb2:1 -:1 Eb2:1 -:1 Bb2:1 -:1 | \
             F3:1 -:1 C4:1 -:1 F3:1 -:1 Bb2:1 -:1 | Eb2:1 -:1 Bb2:1 -:1 F3:1 -:1 C4:1 -:1 | \
             F3:1 -:1 Bb2:1 -:1 Eb2:1 -:1 Bb2:1 -:1 | F3:1 -:1 C4:1 -:1 F3:1 -:1 Bb2:1 -:1 & \
             !mp Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | \
             Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 | Db3:1 -:1 Bb2:1 -:1 Eb3:1 -:1 Bb2:1 -:1 & \
             Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 | Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 | \
             Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 | Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 | \
             Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 | Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 | \
             Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 | Ab6+Eb7:1 Bb6:1 Eb6+Ab6:1 -:1 Bb6+Eb7:1 C7:1 Eb6+Bb6:1 -:1 & \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | \
             Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 | Eb4+Bb4:1 -:1 C5+Gb5:1 -:1 F4+C5:1 -:1 C5+Ab5:1 -:1 & \
             Gb5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4+Db5+F5:1 | Eb5+F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4+Db5+F5:1 | \
             Eb5+F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4+Db5+F5:1 | Eb5+F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4+Db5+F5:1 | \
             Eb5+F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4+Db5+F5:1 | Eb5+F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4+Db5+F5:1 | \
             Eb5+F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4+Db5+F5:1 | Eb5+F5+Gb5+Ab5+Bb5:1 F5+Ab5:1 Eb5+Gb5:1 Db5+F5:1 C5+Eb5:1 Bb4+Db5:1 Ab4+C5:1 Gb4+Bb4:1",
        )
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "30",
            "!mp F4:1 Gb4:1 Ab4:1 Ab4:1 G4:1 G4:1 Bb4:1 Bb4:1 | F4:1 F4:1 Ab4:1 Ab4:1 G4:1 G4:1 Bb4:1 Bb4:1 | !mf F4:1 F4:1 Ab4:1 Ab4:1 G4:1 G4:1 Bb4:1 Bb4:1 | A4:1 A4:1 C5:1 C5:1 B4:1 B4:1 D5:1 D5:1 | \
             F4:1 F4:1 Ab4:1 Ab4:1 G4:1 G4:1 Bb4:1 Bb4:1 | A4:1 Bb4:1 C5:1 C5:1 B4:1 B4:1 D5:1 D5:1 | A4:1 A4:1 C5:1 C5:1 B4:1 B4:1 D5:1 D5:1 | A4:1 A4:1 C5:1 C5:1 B4:1 B4:1 D5:1 D5:1",
            "",
        )
        .with_left(
            "-:8 | -:8 | \
             -:2 D4+Ab4+Bb4+D5+Ab5:1 -:3 E4+Bb4+C5+E5+Bb5:1 -:1 | -:8 | \
             -:2 D4+Ab4+Bb4+D5+Ab5:1 -:5 | Eb4+A4+Db5+Eb5+A5:1 -:7 | \
             Eb4+G4+Db5+Eb5+A5:1 -:7 | -:2 E4+Gb4+Bb4+Gb5+C6:1 -:5 & \
             !mp (3 Eb2+Db4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 (3 F3+Eb4:1 _:1 _:1 (3 Bb3:1 _:1 _:1 | (3 F3+Db4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 (3 Eb2+Eb4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 | \
             (3 F3+Db4:1 _:1 _:1 (3 Bb3:1 _:1 _:1 (3 F3+Eb4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 | (3 Eb2+Db4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 (3 F3+Eb4:1 _:1 _:1 (3 Bb3:1 _:1 _:1 | \
             (3 F3+Db4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 (3 Eb2+Eb4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 | (3 F3+Db4:1 _:1 _:1 (3 Bb3:1 _:1 _:1 (3 F3+Eb4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 | \
             !mf (3 Eb2+Db4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 (3 F3+Eb4:1 _:1 _:1 (3 Bb3:1 _:1 _:1 | (3 F3+Db4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 (3 Eb2+Eb4:1 _:1 _:1 (3 Bb2+Bb3:1 _:1 _:1 & \
             !p (3 B3:1 -:1 D4:1 (3 -:1 F4:1 -:1 (3 Db4:1 -:1 E4:1 (3 C4:1 G4:1 -:1 | (3 B3:1 -:1 D4:1 (3 -:1 F4:1 -:1 (3 E4:1 -:1 G4:1 (3 -:1 Bb4:1 -:1 | \
             (3 B3:1 -:1 D4:1 (3 >C4+Bb4+Ab5:1 F4:1 -:1 (3 E4:1 -:1 G4:1 (3 >Bb4+E5+Bb5:1 Bb4:1 -:1 | (3 C4:1 -:1 Eb4:1 (3 -:1 Gb4:1 -:1 (3 Ab4:1 -:1 B4:1 (3 >C4:1 D5:1 -:1 | \
             (3 D4:1 -:1 F4:1 (3 >Ab4+Bb4+Ab5:1 A4:1 -:1 (3 E4:1 -:1 G4:1 (3 -:1 Bb4:1 -:1 | (3 >Db5+A5:1 _:1 Gb4:1 (3 -:1 A4:1 -:1 (3 Ab4:1 -:1 B4:1 (3 -:1 D5:1 -:1 | \
             (3 >Db5+A5:1 _:1 Gb4:1 (3 -:1 A4:1 -:1 (3 F4:1 -:1 Ab4:1 (3 >C4:1 B4:1 -:1 | (3 Gb4:1 -:1 A4:1 (3 >Gb4+Bb4+E5+C6:1 C5:1 -:1 (3 Ab4:1 -:1 B4:1 (3 -:1 D5:1 -:1 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | !p (3 Eb4:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 Eb4:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1",
        )
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::MezzoPiano),
        Part::new(
            "31",
            "A6:2 C7:1 G6:1 A6:2 D7:1 -:1 | D7:1 -:1 B6:1 A6:1 B6:2 G6:2 | -:8 | -:8 | \
             A6:1 -:1 D7:1 -:1 G7:1 C7:1 D7:1 G6:1 | A6:2 D7:2 D7:1 -:1 B6:1 G6:1 | -:8 | -:8",
            "",
        )
        .with_left(
            "!mp C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 | C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 | \
             C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 | C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 | \
             C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 | C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 | \
             C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 | C4+D4+C5+D5:1 -:1 A3+F4+A4+F5:1 -:1 D4+E4+D5+E5:1 -:1 A3+G4+A4+G5:1 -:1 & \
             -:8 | -:8 | \
             -:2 Ab2:2 -:2 G2:2 | -:2 Gb1:2 -:2 G1:2 | \
             -:8 | -:8 | \
             -:2 Ab2:2 -:2 G2:2 | -:2 Gb1:2 -:2 G1:2 & \
             -:2 !f Ab1:3 -:1 G2:2 | _:1 -:1 Gb2:3 -:1 G1:2 | \
             _:1 -:7 | -:8 | \
             -:2 Ab1:3 -:1 G2:2 | _:1 -:1 Gb2:3 -:1 G1:2 | \
             _:1 -:7 | -:8 & \
             !pp C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 | C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 | \
             C4+D4+C5+D5:1 C4+D4+C5+D5:1 >C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 >C4+D4+C5+D5:1 C4+D4+C5+D5:1 | C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 | \
             C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 | C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 | \
             C4+D4+C5+D5:1 C4+D4+C5+D5:1 >C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 | >C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1 C4+D4+C5+D5:1",
        )
        .in_key(G, Major)
        .played_by(&[(gm::PICCOLO, 0)]),
        Part::new(
            "32",
            "A5:1 -:1 B5:1 G5:1 A5:1 -:1 B5:1 G5:1 | A5:1 -:1 B5:1 G5:1 A5:1 -:1 B5:1 G5:1 | A5:2 D6:2 D6:1 C6:1 D6:1 A5:1 | B5:2 G5:2 A5:2 D6:1 -:1 | \
             B6:1 A6:1 D7:1 G6:1 A6:2 D7:2 | D7:1 C7:1 D7:1 A6:1 B6:2 G6:2 | B6:1 A6:1 D7:1 G6:1 A6:2 D7:1 -:1 | B6:1 A6:1 D7:2 D7:1 C7:1 D7:1 G6:1",
            "",
        )
        .with_left(
            "-:2 !f F3:2 -:2 Eb3:2 | -:2 F2:2 -:2 G2:2 | \
             -:2 F3:2 -:2 Eb3:2 | -:2 F2:2 -:2 G2:2 | \
             -:2 F2+F3:2 -:2 Eb2+Eb3:2 | -:2 F1+F2:2 -:2 G1+G2:2 | \
             -:2 F2+F3:2 -:2 Eb2+Eb3:2 | -:2 F1+F2:2 -:2 G1+G2:2 & \
             !mp A4:1 -:1 C5:1 -:1 B4:1 -:1 D5:1 -:1 | A4:1 -:1 C5:1 -:1 B4:1 -:1 D5:1 -:1 | \
             A4:1 -:1 C5:1 -:1 B4:1 -:1 D5:1 -:1 | A4:1 -:1 C5:1 -:1 B4:1 -:1 D5:1 -:1 | \
             A4:1 -:1 C5:1 -:1 B4:1 -:1 D5:1 -:1 | A4:1 -:1 C5:1 -:1 B4:1 -:1 D5:1 -:1 | \
             !mf (3 C5+Eb5+G5:1 _:1 -:1 (3 C5:1 -:1 A4+C5+Eb5+G5:1 (3 D5+F5+A5:1 _:1 -:1 (3 D5:1 -:1 B4+D5+F5+A5:1 | !mp A4+C5+Eb5+G5:1 -:1 C5:1 -:1 B4:1 -:1 D5:1 -:1 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             !mp (3 A4:1 -:1 -:1 (3 -:1 -:1 -:1 (3 B4:1 -:1 -:1 (3 -:1 -:1 -:1 | -:8 & \
             A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 | A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 | \
             A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 | A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:1 A4+C5+Eb5+G5:1 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             !mp C4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 D4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 | C4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 D4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 | \
             C4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 D4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 | C4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 D4+A4+C5+Eb5+G5:1 -:1 A3:1 -:1 | \
             !mf C4+A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A3:1 -:1 D4+A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A3:1 -:1 | C4+A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A3:1 -:1 D4+A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A3:1 -:1 | \
             C4+A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A3:1 -:1 D4+B4+D5+F5+A5:1 B4+D5+F5+A5:1 A3:1 -:1 | C4+A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 A3:1 -:1 D4+B4+D5+F5+G5:1 B4+D5+F5+G5:1 A3:1 -:1 & \
             -:8 | -:2 F2:4 G2:1 -:1 | \
             -:8 | -:2 F2:4 G2:1 -:1 | \
             -:2 F2:3 -:1 Eb2:2 | _:1 -:1 F1+F2:3 -:1 G1+G2:2 | \
             _:1 -:1 F2:3 -:1 Eb2:2 | _:1 -:1 F1+F2:3 -:1 G1+G2:2 & \
             !mp A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 & \
             A4+A5:1 -:1 B4+B5+D6:1 G4:1 A4+A5:1 -:1 B4+B5+D6:1 G4:1 | A4+A5:1 -:1 B4+B5+D6:1 G4:1 A4+A5:1 -:1 B4+B5+D6:1 G4:1 | \
             A4+A5:2 D5:2 C5+D5:1 C5:1 B4+D5:1 A4:1 | B4:2 G4:2 A4:2 B4:2 | \
             A4+C5+Eb5+G5:1 -:2 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:2 A4+C5+Eb5+G5:1 | A4+C5+Eb5+G5:1 -:2 A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:2 A4+C5+Eb5+G5:1 | \
             A4+C5+Eb5+G5:1 -:2 A4+C5+Eb5+G5:1 B4+D5+F5+A5:1 -:2 B4+D5+F5+A5:1 | A4+C5+Eb5+G5:1 -:2 A4+C5+Eb5+G5:1 B4+D5+F5+G5:1 -:2 B4+D5+F5+G5:1 & \
             -:8 | -:8 | \
             -:8 | -:6 D5:1 -:1 | \
             -:8 | -:8 | \
             -:8 | -:8 & \
             -:1 A3:1 B3:1 C4:1 D4:1 Eb4:1 F4:1 G4:1 | A4:1 A3+C4:1 B3+D4:1 C4+Eb4:1 D4+F4:1 Eb4+G4:1 F4+A4:1 G4+B4:1 | \
             A4+C5:1 A3+C4+Eb4:1 B3+D4+F4:1 C4+Eb4+G4:1 D4+F4+A4:1 Eb4+G4+B4:1 F4+A4+C5:1 G4+B4+D5:1 | A4+C5+Eb5:1 A3+C4+Eb4+G4:1 B3+D4+F4+A4:1 C4+Eb4+G4+B4:1 D4+F4+A4+C5:1 Eb4+G4+B4+D5:1 F4+A4+C5+Eb5:1 G4+B4+D5+F5:1 | \
             A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 | A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 | \
             Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 B3+Eb4+G4+D5:1 F4+B4+D5+A5:1 Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 | A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 A4+C5+Eb5+G5:1 G4+B4+D5+F5:1 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             (3 F4:1 Gb4:1 G4:1 (3 A4:1 _:1 B4:1 (3 _:1 Db5:1 D5:1 (3 E5:1 _:1 Gb5:1 | G5:2 -:6 & \
             -:8 | -:8 | \
             -:8 | -:8 | \
             -:8 | -:8 | \
             (3 -:1 -:1 -:1 (3 Ab4:1 Bb4:1 _:1 (3 C5:1 -:1 -:1 (3 Eb5:1 F5:1 _:1 | -:8",
        )
        .in_key(G, Major)
        .played_by(&[(gm::FLUTE, 0)]),
        Part::new(
            "33",
            "B6:1 A6:1 D7:1 -:1 A6:2 D7:2 | D7:1 C7:1 D7:1 A6:1 B6:2 G6:1 -:1 | A6:2 D7:2 G7:1 C7:1 D7:1 G6:1 | A6:2 D7:2 D7:1 C7:1 D7:1 G6:1",
            "",
        )
        .with_left(
            "-:2 !f F3:2 -:2 Eb3:2 | -:2 F2:2 -:2 G2:2 | \
             -:2 F3:2 -:2 Eb3:2 | -:2 F2:2 -:2 G2:2 & \
             -:2 !f F2:1 -:3 Eb2:1 -:1 | -:2 F1:1 -:3 G1:1 -:1 | \
             -:2 F2:1 -:3 Eb2:1 -:1 | -:2 F1:1 -:3 G1:1 -:1 & \
             !ff (3 C4+A4+C5+Eb5+G5:1 -:1 -:1 (3 A3+C5:1 -:1 A4+C5+Eb5+G5:1 (3 D4+B4+D5+F5+A5:1 -:1 -:1 (3 A3+D5:1 -:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | \
             C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5+B5:1 -:1 A3+C5:1 C5+Eb5+G5+B5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 & \
             !f A5:1 A5:1 B5:1 B5:1 A5:1 A5:1 D6:1 D6:1 | C6:1 C6:1 B5:1 A5:1 B5:1 B5:1 G5:1 G5:1 | \
             A5:1 A5:1 D6:1 D6:1 G6:1 C6:1 D6:1 G5:1 | A5:1 A5:1 D6:1 D6:1 C6:1 C6:1 B5:1 G5:1 & \
             A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:2 B4+D5+F5+A5:1 B4+D5+F5+A5:1 -:2 | A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:2 B4+D5+F5+A5:1 B4+D5+F5+A5:1 -:2 | \
             A4+C5+Eb5+G5:1 A4+C5+Eb5+G5:1 -:2 B4+D5+F5+A5:1 B4+D5+F5+A5:1 -:2 | C5+Eb5+G5+B5:1 C5+Eb5+G5+B5:1 -:2 B4+D5+F5+A5:1 B4+D5+F5+A5:1 -:2 & \
             -:2 F2+F3:3 -:1 Eb2+Eb3:2 | _:1 -:1 F1+F2:3 -:1 G1+G2:2 | \
             _:1 -:1 F2+F3:3 -:1 Eb2+Eb3:2 | _:1 -:1 F1+F2:3 -:1 G1+G2:2 & \
             !mp A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 & \
             A4+C5+Eb5+G5:1 -:7 | -:8 | \
             -:8 | -:8 & \
             -:2 F3:2 -:2 Eb3:2 | -:2 F2:2 -:2 G2:2 | \
             -:2 F3:2 -:2 Eb3:2 | -:2 F2:2 -:2 G2:2 & \
             Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 B3+Eb4+G4+D5:1 F4+B4+D5+A5:1 Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 | Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 B3+Eb4+G4+D5:1 F4+B4+D5+A5:1 Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 | \
             Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 B3+Eb4+G4+D5:1 F4+B4+D5+A5:1 Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 | Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 B3+Eb4+G4+D5:1 F4+B4+D5+A5:1 Eb4+A4+C5+G5:1 D4+G4+B4+F5:1 C4+F4+A4+Eb5:1 & \
             (3 F4:1 Gb4:1 G4:1 (3 A4:1 _:1 B4:1 (3 _:1 Db5:1 D5:1 (3 E5:1 _:1 Gb5:1 | (3 _:1 Gb4:1 G4:1 (3 A4:1 _:1 B4:1 (3 _:1 Db5:1 D5:1 (3 E5:1 _:1 Gb5:1 | \
             (3 _:1 C5:1 Db5:1 (3 D5:1 Eb5:1 E5:1 (3 D5:1 _:1 E5:1 (3 _:1 F5+A5:1 Gb5+Bb5:1 | G5+B5:1 Gb5:1 E5+F5:1 Eb5:1 D5:1 -:3 & \
             (3 -:1 -:1 -:1 (3 Ab4:1 Bb4:1 _:1 (3 C5:1 -:1 -:1 (3 Eb5:1 F5:1 _:1 | (3 F4+G5:1 -:1 -:1 (3 Ab4:1 Bb4:1 _:1 (3 C5:1 -:1 -:1 (3 Eb5:1 F5:1 _:1 | \
             (3 B4+G5:1 -:1 -:1 (3 -:1 -:1 -:1 (3 F5:1 Eb5:1 _:1 (3 Ab5:1 -:1 -:1 | -:8 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 Gb5:1 G5:1 (3 -:1 -:1 -:1 | -:8",
        )
        .in_key(G, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "34",
            "A6:2 B6:1 G6:1 A6:2 D7:2 | C7:2 B6:1 A6:1 B6:2 G6:2 | A6:2 D7:2 G7:1 C7:1 D7:1 G6:1 | A6:2 D7:2 C7:2 B6:1 G6:1",
            "",
        )
        .with_left(
            "-:2 !f F2:2 F2:1 F2+F3:1 Eb3:2 | _:1 Eb2:1 F2:2 F2:1 F1+F2:1 G1:2 | \
             _:1 G2:1 F2:2 F2:1 F2+F3:1 Eb3:2 | _:1 -:1 F1:2 -:2 G1:2 & \
             -:2 !f F3:1 -:3 Eb2:1 Eb2:1 | Eb2:1 -:1 F1:1 -:3 G2:1 G2:1 | \
             G2:1 -:1 F3:1 -:3 Eb2:1 Eb2:1 | Eb2:2 -:6 & \
             !ff C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5+B5:1 -:1 A3+C5:1 C5+Eb5+G5+B5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | \
             C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5:1 -:1 F2:2 D4+B4:1 -:1 G2:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff A3+A4+C5+Eb5+G5:1 -:3 A3+A4+C5+D5+Eb5+G5:1 -:1 & \
             !f A6:2 B6:1 G6:1 A6:2 D6:2 | C6:2 B5:1 A5:1 B5:2 G6:2 | \
             A6:2 D6:2 G6:1 C6:1 D6:1 G5:1 | A5:2 D6:2 C6:2 B5:1 G6:1 & \
             !f C5+Eb5+G5:1 C5+Eb5+G5:1 -:2 D5+F5+A5:1 D5+F5+A5:1 -:2 | Eb5+G5+B5:1 Eb5+G5+B5:1 -:2 D5+F5+A5:1 D5+F5+A5:1 -:2 | \
             C5+Eb5:1 C5+Eb5:1 -:2 D5+F5:1 D5+F5:1 -:2 | -:2 C5+Eb5+G5:1 -:3 C5+Eb5+G5:1 -:1 & \
             -:2 !f F2:3 -:1 Eb2:2 | _:1 -:1 F1:3 -:1 G1:2 | \
             _:1 -:1 F2:3 -:1 Eb2:2 | _:1 -:1 !ff F1:2 -:2 G1:2 & \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 >G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 >G3+B3+D4+F4:1 -:1 & \
             -:8 | -:2 !f F1:3 -:1 G1:2 | \
             _:1 -:7 | -:2 !ff F1:3 -:1 G1:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !f D6:2 C6:2 B5:1 G5:1 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff F2:2 -:2 G2:2 & \
             !f Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 B3+D4+Eb4+G4+D5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | G4+B4+C5+Eb5+B5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | \
             Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 B3+D4+Eb4+G4+D5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | -:2 !ff F2+F3+Eb4:2 -:2 G2+G3+Eb4:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff A4+G5:1 -:3 A4+G5:1 -:1 & \
             (3 A5:1 _:1 _:1 (3 D6:1 Eb6:1 E6:1 (3 D6:1 _:1 E6:1 (3 _:1 F6+A6:1 Gb6:1 | (3 _:1 Gb6:1 _:1 (3 Ab6:1 E6+G6:1 Eb6+Gb6:1 (3 D6:1 _:1 Eb6:1 (3 D6:1 Db6:1 C6:1 | \
             !mp B5:1 -:3 D6:1 Eb6:1 E6+F6:1 Gb6:1 | !mf G6:1 -:3 C6:2 B5:1 G5:1 & \
             (3 B5:1 C6:1 Db6:1 (3 -:1 -:1 -:1 (3 F6:1 Eb6:1 _:1 (3 Ab6:1 -:1 Bb6:1 | (3 G6:1 _:1 F6:1 (3 _:1 -:1 -:1 (3 F6:1 E6:1 -:1 (3 -:1 -:1 -:1 | \
             -:8 | -:8 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 Gb6:1 G6:1 (3 -:1 -:1 -:1 | (3 B6:1 Bb6:1 A6:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             -:8 | -:8 & \
             (3 C5+G5:1 Db5+Ab5:1 D5+A5:1 (3 Eb5+E5+Bb5+B5:1 F5+C6:1 Gb5+Db6:1 G5+D6:1 Ab5:1 A5+Bb5:1 B5:1 | C6:1 B5:1 A5+Bb5:1 Ab5:1 (3 G5+D6:1 Db6:1 C6:1 (3 Bb5+B5:1 A5:1 Ab5:1 | \
             (3 A4+C5+G5:1 Bb4+Db5+Ab5:1 B4+D5+A5:1 (3 E5:1 _:1 D5+Gb5+Db6:1 (3 G5:1 _:1 A5:1 (3 _:1 G5+Bb5:1 Ab5+B5:1 | A5+C6:1 -:7 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 C5+Eb5+Bb5+B5:1 F5:1 _:1 (3 Eb5+D6:1 Ab5:1 _:1 (3 Gb5:1 -:1 -:1 | -:8 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 -:1 Db5+C6:1 -:1 (3 -:1 E5:1 F5:1 (3 -:1 -:1 -:1 | -:8",
        )
        .in_key(G, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Fortissimo),
        Part::new(
            "35",
            "A6:2 B6:1 G6:1 A6:2 D7:2 | C7:2 B6:1 A6:1 B6:2 G6:2 | A6:2 D7:2 G7:1 C7:1 D7:1 G6:1 | A6:2 D7:2 C7:2 B6:1 G6:1",
            "",
        )
        .with_left(
            "-:2 !f F2:2 F2:1 F2+F3:1 Eb3:2 | _:1 Eb2:1 F2:2 F2:1 F1+F2:1 G1:2 | \
             _:1 G2:1 F2:2 F2:1 F2+F3:1 Eb3:2 | _:1 -:1 F1:2 -:2 G1:2 & \
             -:2 !f F3:1 -:3 Eb2:1 Eb2:1 | Eb2:1 -:1 F1:1 -:3 G2:1 G2:1 | \
             G2:1 -:1 F3:1 -:3 Eb2:1 Eb2:1 | Eb2:2 -:6 & \
             !ff C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5+B5:1 -:1 A3+C5:1 C5+Eb5+G5+B5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | \
             C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5:1 -:1 F2:2 D4+B4:1 -:1 G2:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff A3+A4+C5+Eb5+G5:1 -:3 A3+A4+C5+D5+Eb5+G5:1 -:1 & \
             !f A4+A5+A6:2 B5+B6:1 G5+G6:1 A5+A6:2 D6:2 | C6:2 B5:1 A5:1 B5:2 G5+G6:2 | \
             A5+A6:2 D6:2 G6:1 C6:1 D6:1 G5:1 | A5:2 D6:2 C6:2 B5:1 G5+G6:1 & \
             !f C5+Eb5+G5:1 C5+Eb5+G5:1 -:2 D5+F5+A5:1 D5+F5+A5:1 -:2 | Eb5+G5+B5:1 Eb5+G5+B5:1 -:2 D5+F5+A5:1 D5+F5+A5:1 -:2 | \
             C5+Eb5:1 C5+Eb5:1 -:2 D5+F5:1 D5+F5:1 -:2 | -:2 C5+Eb5+G5:1 -:3 C5+Eb5+G5:1 -:1 & \
             -:2 !f F2:3 -:1 Eb2:2 | _:1 -:1 F1:3 -:1 G1:2 | \
             _:1 -:1 F2:3 -:1 Eb2:2 | _:1 -:1 !ff F1:2 -:2 G1:2 & \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 >G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 >G3+B3+D4+F4:1 -:1 & \
             -:8 | -:2 !ff F1:3 -:1 G1:2 | \
             _:1 -:7 | A5:2 F1:3 -:1 G1:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !f D6:2 C6:2 B5:1 G5:1 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff F2:2 -:2 G2:2 & \
             !f Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 B3+D4+Eb4+G4+D5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | G4+B4+C5+Eb5+B5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | \
             Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 B3+D4+Eb4+G4+D5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | -:2 !ff F2+F3+Eb4:2 -:2 G2+G3+Eb4:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff A4+G5:1 -:3 A4+G5:1 -:1 & \
             (3 A5+B5:1 _:1 Db6:1 (3 _:1 Eb6:1 _:1 (3 D6+F6:1 _:1 E6+G6:1 (3 _:1 A6:1 _:1 | (3 G6+B6:1 _:1 F6+A6:1 (3 _:1 G6:1 _:1 (3 D6+F6:1 _:1 Eb6:1 (3 _:1 Db6:1 _:1 | \
             B5:1 -:3 D6:1 Eb6:1 E6+F6:1 Gb6:1 | A5:2 D6:2 C6:2 B5:1 G5:1 & \
             (3 -:1 C6:1 _:1 (3 D6:1 _:1 E6:1 (3 _:1 Eb6+Gb6:1 _:1 (3 Ab6:1 _:1 Gb6+Bb6:1 | (3 _:1 Gb6+Bb6:1 _:1 (3 Ab6:1 _:1 Gb6:1 (3 _:1 E6:1 _:1 (3 D6:1 _:1 C6:1 | \
             -:8 | !f G6:1 -:7 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 (3 -:1 F6:1 -:1 | (3 -:1 -:1 -:1 (3 -:1 E6:1 Eb6:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             -:8 | -:8 & \
             (3 C5+G5:1 Db5+Ab5:1 D5+A5:1 (3 Eb5+E5+Bb5+B5:1 F5+C6:1 Gb5+Db6:1 G5+D6:1 Ab5:1 A5+Bb5:1 B5:1 | C6:1 B5:1 A5+Bb5:1 Ab5:1 (3 G5+D6:1 Db6:1 C6:1 (3 Bb5+B5:1 A5:1 Ab5:1 | \
             (3 A4+C5+G5:1 Bb4+Db5+Ab5:1 B4+D5+A5:1 (3 E5:1 _:1 D5+Gb5+Db6:1 (3 G5:1 _:1 A5:1 (3 _:1 G5+Bb5:1 Ab5+B5:1 | A5+C6:1 -:7 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 C5+Eb5+Bb5+B5:1 F5:1 _:1 (3 Eb5+D6:1 Ab5:1 _:1 (3 Gb5:1 -:1 -:1 | -:8 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | -:4 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 -:1 Db5+C6:1 -:1 (3 -:1 E5:1 F5:1 (3 -:1 -:1 -:1 | -:8",
        )
        .in_key(G, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Fortissimo),
        Part::new(
            "36",
            "A6:2 D7:2 G7:1 C7:1 D7:1 G6:1 | A6:2 D7:2 G7:1 C7:1 D7:1 G6:1 | A6:2 D7:2 G7:1 C7:1 D7:1 G6:1 | A6:2 D7:2 C7:2 B6:1 G6:1",
            "",
        )
        .with_left(
            "-:2 !f F2:2 F2:1 F2+F3:1 Eb3:2 | _:1 Eb2:1 F2:2 F2:1 F1+F2:1 G1:2 | \
             _:1 G2:1 F2:2 F2:1 F2+F3:1 Eb3:2 | _:1 -:1 F1:2 -:2 G1:2 & \
             -:2 !f F3:1 -:3 Eb2:1 Eb2:1 | Eb2:1 -:1 F1:1 -:3 G2:1 G2:1 | \
             G2:1 -:1 F3:1 -:3 Eb2:1 Eb2:1 | Eb2:2 -:6 & \
             !ff C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5+B5:1 -:1 A3+C5:1 C5+Eb5+G5+B5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | \
             C4+A4+C5+Eb5+G5:1 -:1 A3+C5:1 A4+C5+Eb5+G5:1 D4+B4+D5+F5+A5:1 -:1 A3+D5:1 B4+D5+F5+A5:1 | C4+A4+C5+Eb5+G5:1 -:1 F2:2 D4+B4:1 -:1 G2:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff A3+A4+C5+Eb5+G5:1 -:3 A3+A4+C5+D5+Eb5+G5:1 -:1 & \
             !f A5+A6:2 D6:2 G6:1 C6:1 D6:1 G5:1 | A5+A6:2 D6:2 G6:1 C6:1 D6:1 G5:1 | \
             A5+A6:2 D6:2 G6:1 C6:1 D6:1 G5:1 | A5+A6:2 D6:2 C6:2 B5:1 G5:1 & \
             !f C5+Eb5+G5:1 C5+Eb5+G5:1 -:2 D5+F5+A5:1 D5+F5+A5:1 -:2 | Eb5+G5+B5:1 Eb5+G5+B5:1 -:2 D5+F5+A5:1 D5+F5+A5:1 -:2 | \
             C5+Eb5:1 C5+Eb5:1 -:2 D5+F5:1 D5+F5:1 -:2 | -:2 C5+Eb5+G5:1 -:3 C5+Eb5+G5:1 -:1 & \
             -:2 !f F2:3 -:1 Eb2:2 | _:1 -:1 F1:3 -:1 G1:2 | \
             _:1 -:1 F2:3 -:1 Eb2:2 | _:1 -:1 !ff F1:2 -:2 G1:2 & \
             !ff A3:3 F4:1 G4+A4:1 -:3 | C4:2 Eb4+F4:1 G4:1 A4+C5:1 -:3 | \
             A3+C4+Eb4:2 C4+Eb4+F4+G4:1 F4+G4+A4:1 G4+A4+C5+Eb5:1 -:3 | -:8 & \
             -:2 !ff Eb4:2 -:4 | -:8 | \
             -:8 | -:8 & \
             -:2 !ff C4:1 -:5 | -:8 | \
             -:8 | -:8 & \
             -:8 | -:2 !ff F1:3 -:1 G1:2 | \
             _:1 -:7 | -:2 F1:3 -:1 G1:2 & \
             !mp A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | \
             A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 G3+B3+D4+F4:1 -:1 | A3+C4+Eb4+G4:1 -:1 !sfz G3+B3+D4+F4:1 -:1 A3+C4+Eb4+G4:1 -:1 !sfz G3+B3+D4+F4:1 -:1 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff F2:2 -:2 G2:2 & \
             !f Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 B3+D4+Eb4+G4+D5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | G4+B4+C5+Eb5+B5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | \
             Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 B3+D4+Eb4+G4+D5:1 F4+A4+B4+D5+A5:1 Eb4+G4+A4+C5+G5:1 D4+F4+G4+B4+F5:1 C4+Eb4+F4+A4+Eb5:1 | -:2 !ff F2+F3+Eb4:2 -:2 G2+G3+Eb4:2 & \
             -:8 | -:8 | \
             -:8 | -:2 !ff A4+G5:1 -:3 A4+G5:1 -:1 & \
             !mp (3 G5+A5:1 Ab5:1 A5:1 (3 B5:1 _:1 Db6:1 D6:1 Eb6:1 F6:2 | !mf (3 G6:1 _:1 A5:1 (3 B5:1 _:1 Db6:1 D6:1 Eb6:1 F6:2 | \
             (3 G6:1 _:1 A5:1 (3 B5:1 _:1 Db6:1 D6:1 Eb6:1 F6:2 | A5:2 D6:2 C6:2 B5:1 G5:1 & \
             (3 -:1 -:1 -:1 (3 Bb5:1 C6:1 -:1 -:2 E6:1 Gb6:1 | (3 _:1 Ab5:1 -:1 (3 Bb5:1 C6:1 -:1 -:2 E6:1 Gb6:1 | \
             (3 _:1 Ab5:1 -:1 (3 Bb5:1 C6:1 -:1 -:2 E6:1 Gb6:1 | G6:1 -:7 & \
             (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | !mp (3 G5:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | \
             (3 G5:1 -:1 -:1 (3 -:1 -:1 -:1 -:4 | -:8 & \
             (3 Db5+E5:1 D5+F5:1 Eb5+Gb5:1 (3 Ab5:1 _:1 Gb5+Bb5:1 (3 G5+B5:1 Ab5:1 A5:1 (3 Bb5:1 Bb5+B5:1 B5+C6:1 | (3 Db5+E5+C6+Db6:1 D5+F5:1 Eb5+Gb5:1 (3 Ab5:1 _:1 Gb5+Bb5:1 (3 G5+B5:1 Ab5:1 A5:1 (3 Bb5:1 Bb5+B5:1 B5+C6:1 | \
             (3 Db5+E5+C6+Db6:1 D5+F5:1 Eb5+Gb5:1 (3 Ab5:1 _:1 Gb5+Bb5:1 (3 G5+B5:1 Ab5:1 A5:1 (3 Bb5:1 Bb5+B5:1 B5+C6:1 | A5:2 D6:2 C6:2 B5:1 G5:1 & \
             (3 -:1 -:1 -:1 !mp (3 E5+G5:1 A5:1 _:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | (3 -:1 -:1 -:1 (3 E5+G5:1 A5:1 _:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 (3 E5+G5:1 A5:1 _:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | !mf C6+Db6:1 -:7 & \
             (3 -:1 -:1 -:1 !mp (3 -:1 F5:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | (3 -:1 -:1 -:1 (3 -:1 F5:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | \
             (3 -:1 -:1 -:1 !mf (3 -:1 F5:1 -:1 (3 -:1 -:1 -:1 (3 -:1 -:1 -:1 | -:8",
        )
        .in_key(G, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Fortissimo),
    ],
    form: &["13", "14", "14, bar 5", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "25, bar 6", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36"],
    drums_from: 0,
    about: "The first dance of the ballet, \"Tempo giusto\", in strict time: the young people \
            stamp the earth in spring. The strings hammer one chord in even eighth notes, and \
            it is two chords at once, a polychord: F flat major (written here E, G sharp, B) \
            in the cellos and basses under E flat seventh (E flat, G, B flat, D flat) in the \
            violas and second violins. The meter stays 2/4, but eight horns strike the chord \
            on accents where no beat leads the ear to expect them: the second and fourth \
            eighth notes of the third bar, the second of the fifth, the first of the sixth and \
            seventh, the second of the eighth, so the steady pulse lurches. Between the \
            stampings comes a four-note ostinato, D flat, B flat, E flat, B flat, the E flat \
            seventh chord broken, in the cor anglais, later in the trumpet and a horn, and \
            over it short tunes like folk songs that go round in a few notes: the bassoons and \
            then the oboes on B flat, A flat, G, F; a horn, then a flute; the alto flute and \
            the flutes; the trumpets in parallel chords. At rehearsal 31 a piccolo plays a \
            tune in G major, A, C, G, A, D, and from 32 it comes back again and again, in the \
            flutes and then the piccolos, over a bass stamping F, E flat, F, G off the beat \
            and horns rocking between two chords, every instrument joining it, louder each \
            time, until the dance breaks off into the next one, the Ritual of Abduction.",
};

// Source: Jun Nishio's sequence of Part I of The Rite of Spring for the Roland SC-88,
// converted by script, checked against the full orchestral score (Russischer Musikverlag,
// 1921, pages 28 to 37): rehearsal 37 to 48, 67 bars. Each rehearsal number is a part,
// split where its tune passes to another instrument. The tune of each passage is the
// melody, played by the instrument that has it: the violins' chord, the D clarinet's
// fanfare, the bassoons' arpeggio, the trombones', the pizzicato basses', the horns' call,
// the oboes', the piccolos', the flutes', the trumpets' chords. Everything else is the
// accompaniment, on the strings, the timpani at their pitches. Quantised to sixteenths.
// The dynamics come from the sequence's velocities, as in the Augurs of Spring. Where the
// sequence differs from the score, the score is followed:
// - the bars and meters are the score's: the sequence writes the bar of 4/8 and the bar
//   of 5/8 at 39 as one bar of 9/8;
// - the tempo is the score's, "Presto", a dotted quarter 132 a minute (a quarter 198),
//   steady throughout, not the sequence's (a quarter 170, slowing down to 130 at the end
//   of 44, then 160); the last two bars get the score's fermatas;
// - the strings' eighth notes, which the sequence plays as pairs of sixteenths at 37, 38,
//   40, 41 and 47, are eighth notes, and the chords of 45, which it strikes in eighth
//   notes, are held for a dotted quarter;
// - the D clarinet's tune at 37 is ff, the bassoons' at 38 and before 41 f, and the
//   chords of 45 ff (fff in the score), where the sequence plays them softer.
// Left out: the trills, which are held on their lower notes (the clarinets' and the
// violins' at 39, the flutes' at the end), and the tremolos, held as chords of their
// notes, as are the clarinets' sweeps at 42; the flutter-tongue of the oboes at 39, the
// glissandos, and the bass drum.
pub(super) const RITUAL_OF_ABDUCTION: Song = Song {
    name: "The Rite of Spring, Ritual of Abduction",
    origin: "Igor Stravinsky, The Rite of Spring, 1913",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: C,
    mode: Chromatic,
    time: NINE_EIGHT,
    tempo: 198.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FLUTE, gm::STRING_ENSEMBLE, gm::CONTRABASS),
    style: "",
    parts: &[
        Part::new(
            "37",
            "Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 | !f Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2 Eb5:2",
            "",
        )
        .with_left(
            "G4+Bb4+Db5+Eb5:18 | _:18 & \
             !f C4+E4+G4:18 | _:18 & \
             !f C2+C3+G3:1 -:17 | -:18 & \
             !f C3:3 -:15 | -:3 Gb2:3 -:9 Gb2:3",
        )
        .in_meters(&[NINE_EIGHT, NINE_EIGHT])
        .played_by(&[(gm::VIOLIN, 0)]),
        Part::new(
            "37, bar 3",
            "-:8 A5:1 -:1 D6:1 -:1 A5:1 -:1 D6:1 -:1 E6:1 -:1 | G6:1 -:1 Gb6:1 -:1 E6:1 -:1 D6:1 -:1 A5:1 -:1 B5:1 -:1 D6:1 -:1 G6:1 -:1 Gb6:1 -:1 | E6:1 -:1 D6:1 -:1 A5:1 -:3 B5:1 -:1 D6:1 -:1 C6:1 -:1 A5:1 -:1 A5:1 -:1",
            "",
        )
        .with_left(
            "C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 | C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 | \
             C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 C4+E4+G4+Bb4+Db5+Eb5:2 & \
             -:8 A4:1 A4:1 D5:1 D5:1 A4:1 A4:1 D5:1 D5:1 E5:1 E5:1 | G5:1 G5:1 Gb5:1 Gb5:1 E5:1 E5:1 D5:1 D5:1 A4:1 A4:1 B4:1 B4:1 D5:1 D5:1 G5:1 G5:1 Gb5:1 Gb5:1 | \
             E5:1 E5:1 D5:1 D5:1 A4:1 A4:1 -:2 B4:1 B4:1 D5:1 D5:1 C5:1 C5:1 A4:1 A4:1 A4:1 A4:1 & \
             -:8 !f A5:1 -:1 D6:1 -:1 A5:1 -:1 D6:1 -:1 E6:1 -:1 | G6:1 -:1 Gb6:1 -:1 E6:1 -:1 D6:1 -:1 A5+A6:1 -:1 B5+B6:1 -:1 D6+D7:1 -:1 G6+G7:1 -:1 Gb6+Gb7:1 -:1 | \
             E6+E7:1 -:1 D6+D7:1 -:1 A5+A6:1 -:3 B5+B6:1 -:1 D6+D7:1 -:1 C6+C7:1 -:1 A5+A6:1 -:1 A5+A6:1 -:1 & \
             -:8 A5:1 -:1 D6:1 -:1 A5:1 -:1 D6:1 -:1 E6:1 -:1 | G6:1 -:1 Gb6:1 -:1 E6:1 -:1 D6:1 -:1 A5:1 -:9 | \
             -:18 & \
             -:3 Gb2:3 -:12 | -:18 | \
             -:18",
        )
        .in_meters(&[NINE_EIGHT, NINE_EIGHT, NINE_EIGHT])
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Fortissimo),
        Part::new(
            "38",
            "Ab3:1 -:1 C4:1 -:1 Eb4:1 -:1 Gb4:1 -:1 A4:1 -:1 B4:1 -:1 Ab3:1 -:1 C4:1 -:1 Eb4:1 -:1 | Gb4:1 -:1 A4:1 -:1 B4:1 -:1 Ab3:1 -:1 C4:1 -:1 Eb4:1 -:1 Gb4:1 -:1 A4:1 -:1 B4:1 -:1 | Ab3:1 -:1 C4:1 -:1 Eb4:1 -:1 Gb4:1 -:1 A4:1 -:1 B4:1 -:7",
            "",
        )
        .with_left(
            "!ff Eb4+Gb4+A4+B4+Eb5:2 A4:2 Gb4:2 Eb4:2 C4:2 Ab3:2 B4:2 A4:2 Gb4:2 | Eb4:2 C4:2 Ab3:2 B4:2 A4:2 Gb4:2 Eb4:2 C4:2 Ab3:2 | \
             B4:2 A4:2 Gb4:2 Eb4:2 C4:2 Ab3:2 Bb4+C5:2 F4+Ab4+B4+Db5:2 B3+Eb4+Gb4+A4+C5+D5:2 & \
             Ab3+Eb4+A4:18 | _:18 | \
             _:18 & \
             C4+Gb4+B4:18 | _:18 | \
             _:12 !ff A3+G4+C5:6 & \
             C4+Gb4+B4:18 | _:18 | \
             _:18 & \
             !ff Ab3+Eb4+A4:18 | _:18 | \
             _:12 Db4+E4+Bb4:6 & \
             -:4 (3 -:1 -:1 B3+C4+Db4+Eb4+E4+Gb4+G4:1 >Ab3+A3+D4+Eb4+F4+Gb4+Ab4+A4+B4:1 -:9 (3 -:1 -:1 B3+C4+Db4+Eb4+E4+Gb4+G4:1 | !ff Ab3+A3+D4+Eb4+F4+Gb4+Ab4+A4+B4:1 -:9 (3 -:1 -:1 B3+C4+Db4+Eb4+E4+Gb4+G4:1 Ab3+A3+D4+Eb4+F4+Gb4+Ab4+A4+B4:1 -:5 | \
             -:4 !mf (3 -:1 -:1 B3+C4+Db4+Eb4+E4+Gb4+G4:1 >Ab3+A3+D4+Eb4+F4+Gb4+Ab4+A4+B4:1 -:11 & \
             !ff Ab3+C4+Eb4+Gb4:2 -:16 | -:3 B3:1 -:11 B3:2 -:1 | \
             -:9 B3:1 -:2 A3+Db4+E4+G4+Bb4+C5:2 Bb3+D4+F4+Ab4+B4+Db5:2 B3+Eb4+Gb4+A4+C5+D5:2 & \
             !ff Ab2+Eb3+C4:2 -:13 C3:3 | -:3 B3:3 C3:3 -:6 B3:3 | \
             -:3 C3:3 -:3 B3:3 C3:3 -:3 & \
             -:15 !ff C3:3 | -:3 B3:3 C3:3 -:6 B3:3 | \
             -:3 C3:3 -:3 B3:3 C3:3 -:3 & \
             -:15 !ff C2:3 | -:3 B2:3 C2:3 -:6 B2:3 | \
             -:3 C2:3 -:3 B2:3 C2:3 -:3 & \
             -:18 | -:18 | \
             !ff Eb4:12 -:6 & \
             -:18 | -:18 | \
             !ff Ab3+Bb3+C4+Db4+E4:6 F4+Gb4+Ab4+Bb4+B4:6 -:6",
        )
        .in_meters(&[NINE_EIGHT, NINE_EIGHT, NINE_EIGHT])
        .played_by(&[(gm::BASSOON, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "39",
            "-:2 Db3:1 -:1 E3:1 -:1 C3:1 -:1 | E3:1 -:1 G3:1 -:1 A3:1 -:1 Db4:1 -:1 E4:1 -:1",
            "",
        )
        .with_left(
            "-:8 | !p G5+Bb5+Db6+Eb6:1 Gb5+A5+C6+D6:1 (3 F5+Ab5+B5+Db6:1 _:1 E5+G5+Bb5+C6:1 Eb5+Gb5+A5+B5:1 -:1 (3 D5+F5+Ab5+Bb5:1 Db5+E5+G5+A5:1 -:1 C5+Eb5+Gb5+Ab5:1 B4+D5+F5+G5:1 & \
             -:8 | !mp G5+Bb5+Db6+Eb6:1 Gb5+A5+C6+D6:1 (3 F5+Ab5+B5+Db6:1 -:1 E5+G5+Bb5+C6:1 Eb5+Gb5+A5+B5:1 -:1 (3 D5+F5+Ab5+Bb5:1 Db5+E5+G5+A5:1 -:1 C5+Eb5+Gb5+Ab5:1 B4+D5+F5+G5:1 & \
             -:2 !p G5+Bb5+Db6+Eb6:1 Gb5+A5+C6+D6:1 F5+Ab5+B5+Db6:1 E5+G5+Bb5+C6:1 Eb5+Gb5+A5+B5:1 D5+F5+Ab5+Bb5:1 | Db5+E5+G5+A5:1 -:9 & \
             C4+E4+G4+Bb4+Db5+Eb5:2 -:6 | -:10 & \
             C4+E4+G4+Bb4+Db5+Eb5:2 -:6 | -:10 & \
             !mp G4+Bb4+Db5+Eb5:8 | G4+Bb4+Db5+Eb5:10",
        )
        .in_meters(&[FOUR_EIGHT, FIVE_EIGHT_UNGROUPED])
        .played_by(&[(gm::TROMBONE, 0)])
        .marked(Dynamic::Fortissimo),
        Part::new(
            "39, bar 3",
            "C2:2 E2:2 G2:2 A2:2 Db3:2 E3:2 G3:2 C4:2 E4:2",
            "",
        )
        .with_left(
            "!f Bb4+Db5+E5+Gb5:18 & \
             Bb4+Db5+E5+Gb5:1 -:17",
        )
        .in_meters(&[NINE_EIGHT])
        .played_by(&[(gm::PIZZICATO_STRINGS, 0)]),
        Part::new(
            "40",
            ">A4:3 -:1 D4:1 -:1 >A4:3 -:1 D4:1 -:1 >A4:3 -:1 D4:2 | _:1 -:1 A4:1 -:1 D4:1 -:1 A4:1 -:11 | -:12 A4:3 -:1 D4:2 | _:1 -:1 A4:1 -:1 D4:1 -:1 A4:1 -:11 | \
             -:6 >A4:3 -:1 D4:1 -:1 >A4:3 -:1 D4:2 | _:1 -:1 A4:1 -:1 D4:1 -:1 A4:7 -:1 D4:1 -:1 A4:1 -:1",
            "",
        )
        .with_left(
            "-:18 | -:8 Bb5+Bb6:1 -:1 Eb6+Eb7:1 -:1 Bb5+Bb6:1 -:1 Eb6+Eb7:1 -:1 F6+F7:1 -:1 | \
             Ab6+Ab7:1 -:1 G6+G7:1 -:1 F6+F7:1 -:1 Eb6+Eb7:1 -:1 Bb5+Bb6:1 -:9 | -:6 Eb6+Eb7:1 -:1 Ab6+Ab7:1 -:1 G6+G7:1 -:1 F6+F7:1 -:1 Eb6+Eb7:1 -:1 Bb5+Bb6:1 -:1 | \
             C6+C7:1 -:1 Eb6+Eb7:1 -:1 Db6+Db7:1 -:1 Bb5+Bb6:1 -:1 Bb5+Bb6:1 -:1 Bb5+Bb6:1 -:7 | -:18 & \
             !p >A4+Eb5:2 Db5:2 Bb4:2 G4:2 Eb5:2 Db5:2 Bb4:2 G4:2 Eb5:2 | Db5:2 Bb4:2 G4:2 Eb5:2 Db5:2 Bb4:2 G4:2 Eb5:2 Db5:2 | \
             Bb4:2 G4:2 Eb5:2 Db5:2 Bb4:2 G4:2 Eb5:2 Db5:2 Bb4:2 | G4:2 Eb5:2 Db5:2 Bb4:2 G4:2 Eb5:2 Db5:2 Bb4:2 G4:2 | \
             Eb5:2 Db5:2 Bb4:2 G4:2 Eb5:2 Db5:2 Bb4:2 G4:2 Eb5:2 | Db5:2 Bb4:2 G4:2 Eb5:2 Db5:2 Bb4:2 A4:2 Db5:2 E5:2 & \
             !p >G4+Bb4+Db5+Eb5:2 Bb4:2 G4:2 Bb4:2 G4:2 Bb4:2 G4:2 >Bb4:2 >G4:2 | !f Bb4:2 G4:2 Bb4:2 G4:2 Bb4+Bb5:2 Eb6:2 Bb5:2 Eb6:2 F6:2 | \
             Ab6:2 G6:2 F6:2 Eb6:2 Bb5:2 -:8 | -:6 Eb6:2 Ab6:2 G6:2 F6:2 Eb6:2 Bb5:2 | \
             C6:2 Eb6:2 Db6:2 Bb5:2 Bb5:2 Bb5:2 -:6 | G4:2 G4:2 G4+Bb4:2 G4+Bb4:2 G4+Bb4+Db5:2 G4+Bb4+Db5:2 Gb4+A4+C5+D5:2 F4+Ab4+B4+Db5:2 E4+G4+Bb4+C5:2 & \
             G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 | G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 | \
             G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 | G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 | \
             G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 | G4+Bb4+Db5+Eb5:6 G4+Bb4+Db5+Eb5:6 -:6",
        )
        .in_meters(&[NINE_EIGHT, NINE_EIGHT, NINE_EIGHT, NINE_EIGHT, NINE_EIGHT, NINE_EIGHT])
        .played_by(&[(gm::FRENCH_HORN, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "40, bar 7",
            "Ab3:1 -:1 C4:1 -:1 Eb4:1 -:1 Gb4:1 -:1 A4:1 -:1 B4:1 -:1 Ab3:1 -:1 C4:1 -:1 Eb4:1 -:1 | Gb4:1 -:1 A4:1 -:1 B4:1 -:1 Ab3:1 -:1 C4:1 -:1 Eb4:1 -:1 Gb4:1 -:1 A4:1 -:1 B4:1 -:7",
            "",
        )
        .with_left(
            "!ff Eb4+Gb4+A4+B4:2 A4:2 Gb4:2 Eb4:2 C4:2 Ab3:2 B4:2 A4:2 Gb4:2 | Eb4:2 C4:2 Ab3:2 B4:2 A4:2 Gb4:2 Eb4:2 C4:2 Ab3:2 Gb3+A3+Db4+E4+G4+Bb4+C5:2 Bb3+D4+F4+Ab4+B4+Db5:2 Ab3+B3+Eb4+Gb4+A4+C5+D5:2 & \
             Ab3+C4+Eb4+Gb4+A4+B4:18 | Ab3+Eb4+A4:24 & \
             -:18 | C4+Gb4+B4:18 A3+G4+C5:6 & \
             !ff Ab3+C4+Eb4+Gb4+A4+B4:18 | !mf C4+Gb4+B4:24 & \
             -:18 | !ff Ab3+Eb4+A4:18 Db4+E4+Bb4:6 & \
             -:4 (3 -:1 -:1 B3+C4+Db4+Eb4+E4+Gb4+G4:1 >Ab3+A3+D4+Eb4+F4+Gb4+Ab4+A4+B4:1 -:9 (3 -:1 -:1 B3+C4+Db4+Eb4+E4+Gb4+G4:1 | !ff Ab3+A3+D4+Eb4+F4+Gb4+Ab4+A4+B4:1 -:9 (3 -:1 -:1 B3+C4+Db4+Eb4+E4+Gb4+G4:1 Ab3+A3+D4+Eb4+F4+Gb4+Ab4+A4+B4:1 -:11 & \
             -:9 !ff B3:1 -:8 | -:3 B3:2 -:10 B3:1 -:8 & \
             !ff A5:2 -:1 C3:3 -:3 B3:3 C3:3 -:3 | -:3 B3:3 -:3 C3:3 -:3 B3:3 C3:3 -:3 & \
             -:3 !ff C3:3 -:3 B3:3 C3:3 -:3 | -:3 B3:3 -:3 C3:3 -:3 B3:3 C3:3 -:3 & \
             -:3 !ff C2:3 -:3 B2:3 C2:3 -:3 | -:3 B2:3 -:3 C2:3 -:3 B2:3 C2:3 -:3 & \
             -:3 !ff C2:6 -:3 C2:6 | C2:3 -:6 C2:6 -:3 C2:6 & \
             -:12 !ff Ab3+Bb3+C4+Db4+Eb4+E4:6 | Eb4:18 -:6 & \
             -:18 | !ff F4+Gb4+Ab4+A4+Bb4:6 Ab3+Bb3+C4+Db4+E4:6 F4+Gb4+Ab4+Bb4+B4:6 -:6",
        )
        .in_meters(&[NINE_EIGHT, TWELVE_EIGHT])
        .played_by(&[(gm::BASSOON, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "41",
            "Eb5:2 E5:2 F5:2 Gb5:2 Gb5:2 Gb5:2 Eb5:2 Eb5:2 Eb5:2 | Eb5:2 E5:2 F5:2 Gb5:2 F5:2 E5:2 Eb5:2 Eb5:2 Eb5:2",
            "",
        )
        .with_left(
            "!f A2:1 -:1 Db3:1 -:1 E3:1 -:1 C3:1 -:1 E3:1 -:9 | C3+E3+G3:1 -:1 E3+Ab3+B3:1 -:1 G3+B3+D4:1 -:1 Eb3+G3+Bb3:1 -:1 G3+B3+D4:1 -:1 Bb3+D4+F4:1 -:1 C4+E4+G4:1 -:1 E4+Ab4+B4:1 -:1 G4+B4+D5:1 -:1 & \
             !f A1:1 -:1 Db2:1 -:1 E2:1 -:1 C2:1 -:1 E2:1 -:1 G2:1 -:1 A2:1 -:1 Db3:1 -:1 E3:1 -:1 | C3:1 -:17 & \
             -:7 !f E3:2 G3:2 A3:2 Db4:2 E4:2 C4:1 | _:1 -:17",
        )
        .in_meters(&[NINE_EIGHT, NINE_EIGHT])
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "42",
            "!mp Bb4:1 -:1 Db5:1 -:1 Bb4:1 -:1 G4:1 -:1 E4:1 -:1 Db4:1 -:1 | !mf Db5:1 -:1 E5:1 -:1 Db5:1 -:1 Bb4:1 -:1 G4:1 -:1 E4:1 -:1 | E5:1 -:1 G5:1 -:1 E5:1 -:1 Db5:1 -:1 Bb4:1 -:1 G4:1 -:1 | G5:1 -:1 Bb5:1 -:1 G5:1 -:1 E5:1 -:1 Db5:1 -:1 Bb4:1 -:1 | \
             Bb5:1 -:1 Db6:1 -:1 Bb5:1 -:1 G5:1 -:1 E5:1 -:1 Db5:1 -:1 | Db6:1 -:1 E6:1 -:1 Db6:1 -:1 Bb5:1 -:1 G5:1 -:1 E5:1 -:1",
            "",
        )
        .with_left(
            "!ff Eb4+G4+Bb4:1 -:11 | !mf Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 -:6 | \
             -:12 | !ff Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 -:6 | \
             -:12 | Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 Db5+E5+G5+A5:1 -:6 & \
             -:12 | -:12 | \
             -:12 | -:12 | \
             -:12 | -:10 !ff (3 -:1 -:1 E6+Gb6+G6:1 & \
             -:6 !ff Bb5+Db6+E6+Gb6:6 | Bb5+Db6+E6+Gb6:6 -:6 | \
             -:6 Bb5+Db6+E6+Gb6:6 | Bb5+Db6+E6+Gb6:6 -:6 | \
             -:6 Bb5+Db6+E6+Gb6:6 | Bb5+Db6+E6+Gb6:6 -:6 & \
             G4:12 | G4:12 | \
             _:12 | !ff G4+Bb4:12 | \
             _:12 | _:12 & \
             -:12 | Bb4:12 | \
             !ff Bb4:12 | Db5:12 | \
             Db5:12 | Eb5:12 & \
             Bb4:12 | _:12 | \
             _:12 | -:12 | \
             -:12 | -:12 & \
             Db5:12 | _:12 | \
             -:12 | -:12 | \
             -:12 | -:12 & \
             Eb5:12 | -:12 | \
             -:12 | -:12 | \
             -:12 | -:12 & \
             E1:2 A1:2 Db2:2 Eb2:2 G2:2 Bb2:2 | Bb1+Eb3:2 Db2+G3:2 Gb2+Bb3:2 Bb2+C4:2 Db3+E4:2 Gb3+G4:2 | \
             !ff G2+A3+C5:2 Bb2+Db4+E5:2 Eb3+E4+G5:2 E3+A4+Bb5:2 G3+Db5+Db6:2 C4+E5+Gb6:2 | E2+E4+G5:2 A2+G4+Bb5:2 Db3+C5+Eb6:2 E4+E5:2 G4+A5:2 C5+Db6:2 | \
             Db4+E5:2 Gb4+G5:2 Bb4+C6:2 Db5:2 Gb5:2 Bb5:2 | Bb4:2 Eb5:2 G5:2 -:6 & \
             !ff >C4+E4+G4+Bb4+Db5+Eb5:1 G4:1 Db5:1 Bb4:1 Bb4:1 G4:1 G4:1 E4:1 E4:1 Db4:1 Db4:1 Bb3:1 | Db5:1 Bb4:1 E5:1 Db5:1 Db5:1 Bb4:1 Bb4:1 G4:1 G4:1 E4:1 E4:1 Db4:1 | \
             E5:1 Db5:1 G5:1 E5:1 E5:1 Db5:1 Db5:1 Bb4:1 Bb4:1 G4:1 G4:1 E4:1 | G5:1 E5:1 Bb5:1 G5:1 G5:1 E5:1 E5:1 Db5:1 Db5:1 Bb4:1 Bb4:1 G4:1 | \
             Bb5:1 G5:1 Db6:1 Bb5:1 Bb5:1 G5:1 G5:1 E5:1 E5:1 Db5:1 Db5:1 Bb4:1 | Db6:1 Bb5:1 E6:1 Db6:1 Db6:1 Bb5:1 Bb5:1 G5:1 G5:1 E5:1 E5:1 Db5:1 & \
             Bb4+Db5+Eb5+G5+Bb5+Db6:12 | _:12 | \
             _:12 | _:12 | \
             _:12 | _:12 & \
             G4:6 Eb6:6 | G4:6 Eb6:6 | \
             G4:6 Eb6:6 | G4:6 Eb6:6 | \
             G4:6 Eb6:6 | G4:6 Eb6:6 & \
             -:12 | -:12 | \
             -:12 | E5+G5:6 G4+Bb4+Db5:6 | \
             !ff Eb5+Gb5+G5+Bb5:6 Gb4+A4+Bb4+C5+Db5+E5:6 | Gb5+A5+Bb5+Db6:6 A4+C5+Db5+Eb5+E5+G5:6",
        )
        .in_meters(&[SIX_EIGHT, SIX_EIGHT, SIX_EIGHT, SIX_EIGHT, SIX_EIGHT, SIX_EIGHT])
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Piano),
        Part::new(
            "43",
            "!sfz Bb6:1 -:1 Ab6:1 -:1 Bb6:1 -:1 Ab6:1 -:1 Gb6:1 -:1 F6:1 -:1 | Bb6:1 -:1 Ab6:1 -:1 Bb6:1 -:1 Ab6:1 -:1 Gb6:1 -:1 F6:1 -:1 | Bb6:1 -:1 Ab6:1 -:1 Bb6:1 -:1 Ab6:1 -:1 Gb6:1 -:1 F6:1 -:1 Gb6:1 -:1 | Eb6:1 -:1 Eb6:1 -:1 Eb6:1 -:1 F6:1 -:1 Gb6:1 -:1 Ab6:1 -:1 | \
             Bb6:1 -:1 Ab6:1 -:1 Bb6:1 -:1 Ab6:1 -:1 Gb6:1 -:1 F6:1 -:1 | Eb6:1 -:1 F6:1 -:1 Gb6:1 -:1 Ab6:1 -:1 | A6:1 -:1 G6:1 -:1 A6:1 -:1 G6:1 -:1 F6:1 -:1 E6:1 -:1 | D6:1 -:1 D6:1 -:1 D6:1 -:1 E6:1 -:1 F6:1 -:1 G6:1 -:1",
            "",
        )
        .with_left(
            "-:12 | -:12 | \
             G3+Db4+G4+Gb5+Bb5:1 -:1 G3+Db4+G4+F5+Ab5:1 -:1 G3+Db4+G4+Gb5+Bb5:1 -:1 G3+Db4+G4+F5+Ab5:1 -:1 G3+Db4+G4+Eb5+Gb5:1 -:1 G3+Db4+G4+Db5+F5:1 -:1 G3+Db4+G4+Eb5+Gb5:1 -:1 | G3+Db4+G4+C5+Eb5:1 -:1 G3+Db4+G4+C5+Eb5:1 -:1 G3+Db4+G4+C5+Eb5:1 -:1 G3+Db4+G4+Db5+F5:1 -:1 G3+Db4+G4+Eb5+Gb5:1 -:1 G3+Db4+G4+F5+Ab5:1 -:1 | \
             G3+Db4+G4+Gb5+Bb5:1 -:11 | -:8 | \
             C3+Gb3+C4+F5+A5:1 -:1 C3+Gb3+C4+E5+G5:1 -:1 C3+Gb3+C4+F5+A5:1 -:1 C3+Gb3+C4+E5+G5:1 -:1 C3+Gb3+C4+D5+F5:1 -:1 C3+Gb3+C4+C5+E5:1 -:1 | C3+Gb3+C4+B4+D5:1 -:1 C3+Gb3+C4+B4+D5:1 -:1 C3+Gb3+C4+B4+D5:1 -:1 C3+Gb3+C4+C5+E5:1 -:1 C3+Gb3+C4+D5+F5:1 -:1 C3+Gb3+C4+E5+G5:1 -:1 & \
             -:12 | G4+Bb4+Gb5+Bb5:1 -:3 G4+Bb4+Gb5+Bb5:1 -:7 | \
             G4+Bb4+Gb5+Bb5:1 -:1 F4+A4+F5+Ab5:1 -:1 G4+Bb4+Gb5+Bb5:1 -:1 F4+A4+F5+Ab5:1 -:1 Eb4+G4+Eb5+Gb5:1 -:1 D4+F4+Db5+F5:1 -:1 Eb4+G4+Eb5+Gb5:1 -:1 | C4+E4+C5+Eb5:1 -:1 C4+E4+C5+Eb5:1 -:1 C4+E4+C5+Eb5:1 -:1 D4+F4+Db5+F5:1 -:1 Eb4+G4+Eb5+Gb5:1 -:1 F4+A4+F5+Ab5:1 -:1 | \
             !f G4+Bb4+Gb5+Bb5:1 -:11 | -:8 | \
             !mf Gb4+A4+F5+A5:1 -:1 E4+Ab4+E5+G5:1 -:1 Gb4+A4+F5+A5:1 -:1 E4+Ab4+E5+G5:1 -:1 D4+Gb4+D5+F5:1 -:1 Db4+E4+C5+E5:1 -:1 | B3+Eb4+B4+D5:1 -:1 B3+Eb4+B4+D5:1 -:1 B3+Eb4+B4+D5:1 -:1 Db4+E4+C5+E5:1 -:1 D4+Gb4+D5+F5:1 -:1 E4+Ab4+E5+G5:1 -:1 & \
             G3+Db4+G4+Bb4:1 -:1 G3+Db4+F4+G4+A4:1 -:1 G3+Db4+G4+Bb4:1 -:1 G3+Db4+F4+G4+A4:1 -:1 G3+Db4+Eb4+G4:1 -:1 G3+Db4+D4+F4+G4:1 -:1 | G3+Db4+G4+Bb4:1 -:1 G3+Db4+F4+G4+A4:1 -:1 G3+Db4+G4+Bb4:1 -:1 G3+Db4+F4+G4+A4:1 -:1 G3+Db4+Eb4+G4:1 -:1 G3+Db4+D4+F4+G4:1 -:1 | \
             G3+Db4+G4+Bb4:1 -:1 F4+A4:1 -:1 G4+Bb4:1 -:1 F4+A4:1 -:1 Eb4+G4:1 -:1 D4+F4:1 -:1 Eb4+G4:1 -:1 | C4+E4:1 -:1 C4+E4:1 -:1 C4+E4:1 -:1 D4+F4:1 -:1 Eb4+G4:1 -:1 F4+A4:1 -:1 | \
             G3+Db4+G4+Bb4:1 -:1 G3+Db4+F4+G4+A4:1 -:1 G3+Db4+G4+Bb4:1 -:1 G3+Db4+F4+G4+A4:1 -:1 G3+Db4+Eb4+G4:1 -:1 G3+Db4+D4+F4+G4:1 -:1 | G3+C4+Db4+E4+G4:1 -:1 G3+Db4+D4+F4+G4:1 -:1 G3+Db4+Eb4+G4:1 -:1 G3+Db4+F4+G4+A4:1 -:1 | \
             Gb3+C4+Gb4+A4:1 -:1 E4+Ab4:1 -:1 Gb4+A4:1 -:1 E4+Ab4:1 -:1 D4+Gb4:1 -:1 Db4+E4:1 -:1 | B3+Eb4:1 -:1 B3+Eb4:1 -:1 B3+Eb4:1 -:1 Db4+E4:1 -:1 D4+Gb4:1 -:1 E4+Ab4:1 -:1 & \
             !mp G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 | G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 | \
             G3+Db4+G4:1 -:13 | -:12 | \
             G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 | G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 G3+Db4+G4:1 -:1 | \
             G3+Db4+G4:1 -:11 | -:12 & \
             !mp Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Eb5+Gb5:1 -:1 Db5+F5:1 -:1 | Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Eb5+Gb5:1 -:1 Db5+F5:1 -:1 | \
             Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Eb5+Gb5:1 -:1 Db5+F5:1 -:1 Db5+Eb5+Gb5:1 -:1 | C5+Db5+Eb5:1 -:1 C5+Db5+Eb5:1 -:1 C5+Db5+Eb5:1 -:1 Db5+F5:1 -:1 Db5+Eb5+Gb5:1 -:1 Db5+F5+Ab5:1 -:1 | \
             Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Gb5+Bb5:1 -:1 Db5+F5+Ab5:1 -:1 Db5+Eb5+Gb5:1 -:1 Db5+F5:1 -:1 | C5+Db5+Eb5:1 -:1 Db5+F5:1 -:1 Db5+Eb5+Gb5:1 -:1 Db5+F5+Ab5:1 -:1 | \
             Gb4+F5+A5:1 -:1 Gb4+E5+G5:1 -:1 Gb4+F5+A5:1 -:1 Gb4+E5+G5:1 -:1 Gb4+D5+F5:1 -:1 Gb4+C5+E5:1 -:1 | Gb4+B4+D5:1 -:1 Gb4+B4+D5:1 -:1 Gb4+B4+D5:1 -:1 Gb4+C5+E5:1 -:1 Gb4+D5+F5:1 -:1 Gb4+E5+G5:1 -:1 & \
             !mp G5+Bb5:1 -:1 F5+A5:1 -:1 G5+Bb5:1 -:1 F5+A5:1 -:1 Eb5+G5:1 -:1 D5+F5:1 -:1 | G5+Bb5:1 -:1 F5+A5:1 -:1 G5+Bb5:1 -:1 F5+A5:1 -:1 Eb5+G5:1 -:1 D5+F5:1 -:1 | \
             G5+Bb5:1 -:1 F5+A5:1 -:1 G5+Bb5:1 -:1 F5+A5:1 -:1 Eb5+G5:1 -:1 D5+F5:1 -:1 Eb5+G5:1 -:1 | C5+E5:1 -:1 C5+E5:1 -:1 C5+E5:1 -:1 D5+F5:1 -:1 Eb5+G5:1 -:1 F5+A5:1 -:1 | \
             G5+Bb5:1 -:1 F5+A5:1 -:1 G5+Bb5:1 -:1 F5+A5:1 -:1 Eb5+G5:1 -:1 D5+F5:1 -:1 | C5+E5:1 -:1 D5+F5:1 -:1 Eb5+G5:1 -:1 F5+A5:1 -:1 | \
             Gb5+A5:1 -:1 E5+G5+Ab5:1 -:1 Gb5+A5:1 -:1 E5+G5+Ab5:1 -:1 D5+Gb5:1 -:1 Db5+E5:1 -:1 | B4+Eb5:1 -:1 B4+Eb5:1 -:1 B4+Eb5:1 -:1 Db5+E5:1 -:1 D5+Gb5:1 -:1 E5+Ab5:1 -:1 & \
             Db5:2 -:10 | -:12 | \
             -:14 | -:12 | \
             -:12 | -:8 | \
             -:12 | -:12 & \
             !mp Gb4+Bb4:2 -:10 | -:12 | \
             -:14 | -:12 | \
             -:12 | -:8 | \
             -:12 | -:12 & \
             -:12 | -:12 | \
             -:14 | -:12 | \
             -:12 | -:8 | \
             C3:1 -:1 C3:1 -:1 C3:1 -:1 C3:1 -:1 C3:1 -:1 C3:1 -:1 | C3:1 -:1 C3:1 -:1 C3:1 -:1 C3:1 -:1 C3:1 -:1 C3:1 -:1",
        )
        .in_meters(&[
            SIX_EIGHT,
            SIX_EIGHT,
            SEVEN_EIGHT,
            THREE_FOUR,
            SIX_EIGHT,
            TWO_FOUR,
            SIX_EIGHT,
            THREE_FOUR,
        ])
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "44",
            "D5:3 -:1 A4:1 -:1 D5:3 -:1 A4:1 -:1 D5:3 -:1 A4:1 -:1 | D5:1 -:1 A4:1 -:1 D5:1 -:1 A4:6 -:2 D5:1 -:1 A4:1 -:1 | D5:1 -:1 A4:1 -:1 D5:1 -:1 A4:6 -:6 | -:18 | \
             -:18",
            "",
        )
        .with_left(
            "!p A4+C5+Eb5+F5:6 A4+C5+Eb5+F5:6 A4+C5+Eb5+F5:6 | A4+C5+Eb5+F5:6 A4+C5+Eb5+F5:6 A4+C5+Eb5+F5:6 | \
             A4+C5+Eb5+F5:6 A4+C5+Eb5+F5:6 A4+C5+Eb5+F5:6 | A4+C5+Eb5+F5:6 -:12 | \
             -:18 & \
             -:18 | -:18 | \
             -:18 | !mp Eb4+Gb4+A4+C5+Eb5:18 | \
             _:18 & \
             A4+C5+Eb5:6 A4+C5+Eb5+F5:6 A4+C5+Eb5:6 | A4+C5+Eb5+F5:6 A4+C5+Eb5:6 A4+C5+Eb5+F5:6 | \
             A4+C5+Eb5:6 A4+C5+Eb5+F5:6 A4+C5+Eb5:6 | Eb4+Gb4+A4+B4+C5+Eb5+F5:6 C4+Eb4+F4+A4+C5+D5+Gb5:6 C4+Eb4+Gb4+Ab4+A4+C5+Eb5+F5:6 | \
             A3+C4+Eb4+F4+C5+Eb5+Gb5+Ab5:6 A3+C4+D4+Gb4+A4+C5+Eb5+F5:6 F3+A3+C4+Eb4+Eb5+Gb5+Ab5+Bb5+C6+Db6+D6+Eb6+E6+F6+Gb6+G6+Ab6+A6+Bb6+B6:6 & \
             -:18 | -:18 | \
             -:18 | -:18 | \
             -:15 F1:1 G1:1 Ab1:1",
        )
        .in_meters(&[NINE_EIGHT, NINE_EIGHT, NINE_EIGHT, NINE_EIGHT, NINE_EIGHT])
        .played_by(&[(gm::FRENCH_HORN, 0)])
        .marked(Dynamic::Forte),
        Part::new(
            "45",
            "C7:6 A6:6 Gb6:6 | Eb6:6 A6:6 Gb6:6 | Eb6:6 C6:6 A5:6",
            "",
        )
        .with_left(
            "!mp C5:1 -:1 C5:1 -:1 C5:1 -:1 A4:1 -:1 A4:1 -:1 A4:1 -:1 Gb4+D5:1 -:1 D5:1 -:1 !sfz Gb3+Gb4+D5:1 -:1 | !sfz Db3+Db4:12 D4+Gb4+G4+D5+Gb5:1 -:1 D4+Gb4+G4+D5+Gb5:1 -:1 D4+Gb4+G4+D5+Gb5:1 -:1 | \
             Eb4+E4+B4+Eb5:1 -:1 Eb4+E4+B4+Eb5:1 -:1 Eb4+E4+B4+Eb5:1 -:1 Eb4+Ab4+C5:1 -:1 Eb4+Ab4+C5:1 -:1 Eb4+Ab4+C5:1 -:1 Bb3+F4+A4:1 -:1 Bb3+F4+A4:1 -:1 Bb3+F4+A4:1 -:1 & \
             -:18 | !mp B4+Eb5:1 -:1 B4+Eb5:1 -:1 B4+Eb5:1 -:1 F4+A4+Bb4+F5+A5:1 -:1 F4+A4+Bb4+F5+A5:1 -:1 F4+A4+Bb4+F5+A5:1 -:7 | \
             -:18 & \
             !f Db6+Ab7+C8:6 Bb5+F7+A7:6 G5+D7+Gb7:6 | !mf E5+B5+Eb6+B6+Eb7:6 Bb5+F6+A6:6 G5+D6+Gb6:6 | \
             E5+B5+Eb6:6 Db5+Ab5+C6:6 Bb4+F5+A5:6 & \
             A1+A2+A3:6 E2+E3+E4:6 B1+B2+B3:4 -:2 | -:10 (3 E1:1 F1:1 _:1 Ab1+Ab2+Ab3:1 -:5 | \
             -:18 & \
             -:18 | -:10 (3 -:1 -:1 G1:1 -:6 | \
             -:18 & \
             A1+A2+A3:6 E2+E3+E4:6 B1+B2+B3:4 -:2 | -:12 Ab1+Ab2+Ab3:1 -:5 | \
             -:18 & \
             Db4+Ab4:1 -:1 Db4+Ab4:1 -:1 Db4+Ab4:1 -:1 Bb3+F4:1 -:1 Bb3+F4:1 -:1 Bb3+F4:1 -:1 G3+Gb4+G4:1 -:1 Gb4+G4:1 -:1 Gb4+G4:1 -:1 | Eb4+E4:1 -:1 Eb4+E4:1 -:1 Eb4+E4:1 -:1 Bb3+F4:1 -:9 !sfz Eb3+Eb4:1 -:1 | \
             !f Bb3+Bb4:17 -:1 & \
             !mp Ab5+C6+Db6:6 F5+A5+Bb5:6 D5+Gb5+G5:6 | B4+Eb5+E5:6 F5+A5+Bb5:6 D5+Gb5+G5:6 | \
             B4+Eb5+E5:6 Ab4+C5+Db5:6 F4+A4+Bb4:6 & \
             !mp Ab5+Db6+Ab6:6 F5+Bb5+F6:6 D5+Gb5+G5+D6:6 | B4+E5+B5:6 A5+Bb5+F6+A6:6 Gb5+G5+D6+Gb6:6 | \
             Eb5+E5+B5+Eb6:6 C5+Db5+Ab5+C6:6 A4+Bb4+F5+A5:6 & \
             -:18 | -:12 !f Ab2:2 -:4 | \
             -:18",
        )
        .in_meters(&[NINE_EIGHT, NINE_EIGHT, NINE_EIGHT])
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        Part::new(
            "46",
            "-:6 | F6:1 -:1 Bb6:1 -:1 F6:1 -:1 Bb6:1 -:1 C7:1 -:1 | Eb7:1 -:1 D7:1 -:1 C7:1 -:1 Bb6:1 -:1 F6:1 -:1 | -:6 | \
             Bb6:1 -:1 F6:1 -:1 Bb6:1 -:1 C7:1 -:1 | Eb7:1 -:1 D7:1 -:1 C7:1 -:1 Bb6:1 -:1 F6:1 -:1 | Bb6:1 -:1 Eb7:1 -:1 D7:1 -:1 C7:1 -:1 Bb6:1 -:1 F6:1 -:1 | G6:1 -:1 Bb6:1 -:1 Ab6:1 -:1 F6:1 -:1 F6:1 -:1",
            "",
        )
        .with_left(
            "-:6 | !mp F4:1 -:1 Bb4:1 -:1 F4:1 -:1 Bb4:1 -:1 C5:1 -:1 | \
             Eb5:1 -:1 D5:1 -:1 C5:1 -:1 Bb4:1 -:1 F4:1 -:1 | -:6 | \
             Bb4:1 -:1 F4:1 -:1 Bb4:1 -:1 C5:1 -:1 | Eb5:1 -:1 D5:1 -:1 C5:1 -:1 Bb4:1 -:1 F4:1 -:1 | \
             >Bb4:1 -:1 Eb5:1 -:1 D5:1 -:1 C5:1 -:1 Bb4:1 -:1 F4:1 -:1 | >G4:1 -:1 Bb4:1 -:1 Ab4:1 -:1 F4:1 -:1 F4:1 -:1 & \
             !ff F1+F2:2 -:4 | -:10 | \
             -:10 | F1+F2:2 -:4 | \
             -:8 | -:10 | \
             -:12 | -:10 & \
             -:6 | !mp C5+F5+F6:1 -:1 Eb5+F5+Eb6:1 -:1 D5+Bb5+D6:1 -:1 C5+C6+Eb6:1 -:1 D5+Bb5+D6:1 -:1 | \
             Eb5+F5+Eb6:1 -:1 C5+F5+F6:1 -:1 Eb5+F5+Eb6:1 -:1 D5+Bb5+D6:1 -:1 C5+C6+Eb6:1 -:1 | -:6 | \
             Eb5+F5+Eb6:1 -:1 D5+Bb5+D6:1 -:1 C5+C6+Eb6:1 -:1 D5+Bb5+D6:1 -:1 | Eb5+F5+Eb6:1 -:1 C5+F5+F6:1 -:1 Eb5+F5+Eb6:1 -:1 D5+Bb5+D6:1 -:1 C5+C6+Eb6:1 -:1 | \
             D5+Bb5+D6:1 -:1 Eb5+F5+Eb6:1 -:1 C5+F5+F6:1 -:1 Eb5+F5+Eb6:1 -:1 D5+Bb5+D6:1 -:1 C5+C6+Eb6:1 -:1 | D5+Bb5+D6:1 -:1 Eb5+F5+Eb6:1 -:1 C5+F5+F6:1 -:1 Eb5+F5+Eb6:1 -:1 D5+Bb5+D6:1 -:1 & \
             !ff F2:2 -:4 | -:10 | \
             -:10 | !f F2:2 -:4 | \
             -:8 | -:10 | \
             -:12 | -:10",
        )
        .in_meters(&[
            THREE_EIGHT,
            FIVE_EIGHT_UNGROUPED,
            FIVE_EIGHT_UNGROUPED,
            THREE_EIGHT,
            FOUR_EIGHT,
            FIVE_EIGHT_UNGROUPED,
            SIX_EIGHT,
            FIVE_EIGHT_UNGROUPED,
        ])
        .played_by(&[(gm::FLUTE, 0)]),
        Part::new(
            "47",
            "!f C6:2 -:2 | !mp C4:2 F4:2 G4:2 >C5:2 Bb4:2 G4:2 | C4:2 >F4:2 >G4:2 C5:2 Bb4:2 G4:2 | C4:2 >F4:2 >G4:2 C5:2 Bb4:2 G4:2 | \
             C4:2 F4:2 G4:2 >C5:2 Bb4:2 G4:2 | !f C6:2 -:2 | !mp C4:2 F4:2 G4:2 >C5:2 Bb4:2 G4:2 | !f E6:2 -:2 C6:2 -:2 | \
             !mp C5:2 F5:2 G5:2 >C6:2 Bb5:2 G5:2 | C5:2 >F5:2 >G5:2 C6:2 Bb5:2 G5:2 | -:4 !f C6:2 -:2 | !mp C5:2 >F5:2 >G5:2 C6:2 Bb5:2 G5:2",
            "",
        )
        .with_left(
            "!ff C2+C3+A3+Gb4+Ab4+F5+C6:2 -:2 | -:12 | \
             -:12 | -:12 | \
             -:12 | C2+C3+A3+Gb4+Ab4+F5+C6:2 -:2 | \
             -:12 | C2+C3+A3+Gb4+Ab4+F5+E6:2 -:2 C2+C3+A3+Gb4+Ab4+F5+C6:2 -:2 | \
             -:12 | -:12 | \
             -:4 C2+C3+A3+Gb4+Ab4+F5+C6:2 -:2 | -:12 & \
             !f Ab4+C5+F5+Ab5+C6:2 -:2 | -:12 | \
             -:12 | -:12 | \
             -:12 | Ab4+C5+F5+Ab5+C6:2 -:2 | \
             -:12 | Ab4+E5+F5+Ab5+E6:2 -:2 Ab4+C5+F5+Ab5+C6:2 -:2 | \
             -:12 | -:12 | \
             -:4 Ab4+C5+F5+Ab5+C6:2 -:2 | -:12 & \
             Gb3+A3+C4:2 -:2 | -:12 | \
             -:12 | -:12 | \
             -:12 | Gb3+A3+C4:2 -:2 | \
             -:12 | Gb3+A3+C4:2 -:2 Gb3+A3+C4:2 -:2 | \
             -:12 | -:12 | \
             -:4 Gb3+A3+C4:2 -:2 | -:12 & \
             !f C2+C3:2 -:2 | -:12 | \
             -:12 | -:12 | \
             -:12 | C2+C3:2 -:2 | \
             -:12 | C2+C3:2 -:2 C2+C3:2 -:2 | \
             -:12 | -:12 | \
             -:4 C2+C3:2 -:2 | -:12 & \
             !f C3:2 -:2 | -:12 | \
             -:12 | -:12 | \
             -:12 | C3:2 -:2 | \
             -:12 | C3:2 -:2 C3:2 -:2 | \
             -:12 | -:12 | \
             -:4 C3:2 -:2 | -:12",
        )
        .in_meters(&[
            TWO_EIGHT,
            SIX_EIGHT,
            THREE_FOUR,
            THREE_FOUR,
            SIX_EIGHT,
            TWO_EIGHT,
            SIX_EIGHT,
            FOUR_EIGHT,
            SIX_EIGHT,
            THREE_FOUR,
            FOUR_EIGHT,
            THREE_FOUR,
        ])
        .played_by(&[(gm::VIOLIN, 0)]),
        Part::new(
            "47, bar 13",
            "C4+Ab4+C5+F5+Ab5:2 -:10 C4+Ab4+C5+F5+Ab5:2 -:2 | -:8 C4+Ab4+C5+F5+Ab5:2 -:6 | C4+Ab4+C5+F5+Ab5:2 -:14 | -:16",
            "",
        )
        .with_left(
            "C2+C3+Gb3:2 -:10 C2+C3+Gb3:2 -:2 | -:8 C2+C3+Gb3:2 -:6 | \
             C2+C3+Gb3:2 -:14 | -:16 & \
             !f E7:2 -:10 E7:2 -:2 | -:8 E7:2 -:6 | \
             E7:2 -:14 | -:16 & \
             Ab4+E5+F5+Ab5+E6:2 -:10 Ab4+E5+F5+Ab5+E6:2 -:2 | -:8 Ab4+E5+F5+Ab5+E6:2 -:6 | \
             Ab4+E5+F5+Ab5+E6:2 -:14 | -:16 & \
             A3+Gb4+A4:2 -:10 A3+Gb4+A4:2 -:2 | -:8 A3+Gb4+A4:2 -:6 | \
             A3+Gb4+A4:2 -:14 | -:16 & \
             C2+C3:2 -:10 C2+C3:2 -:2 | -:8 C2+C3:2 -:6 | \
             C2+C3:2 -:14 | -:16 & \
             F5+Ab5+E6:2 -:10 F5+Ab5+E6:2 -:2 | -:8 F5+Ab5+E6:2 -:6 | \
             F5+Ab5+E6:2 -:14 | -:16 & \
             E4+E5+E6:16 | _:16 | \
             _:4 -:12 | -:16 & \
             Eb4+Eb5+Eb6:16 | _:16 | \
             -:16 | -:16 & \
             -:16 | -:16 | \
             !pp Eb4+Eb5+Eb6:16 | _:16 & \
             C3:2 -:10 C3:2 -:2 | -:8 C3:2 -:6 | \
             C3:2 -:14 | -:16",
        )
        .in_meters(&[COMMON, COMMON, COMMON, COMMON])
        .with_tempo(&[fermata(40, 8), fermata(48, 16)])
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
    ],
    form: &[
        "37",
        "37, bar 3",
        "38",
        "39",
        "39, bar 3",
        "40",
        "40, bar 7",
        "41",
        "42",
        "43",
        "44",
        "45",
        "46",
        "47",
        "47, bar 13",
    ],
    drums_from: 0,
    about: "The second dance of the ballet, \"Presto\", a dotted quarter 132 a minute: the young \
            men chase the girls in a game of abduction. Its meter changes all the time: 9/8, then \
            6/8, 7/8, 3/4 and 2/4, and towards the end bars of 5/8, 3/8 and 2/8, the shortest a \
            single stroke. It is in no key, and the score has no key signature: most of its notes \
            come from one octatonic scale, half step and whole step in turn, C, D flat, E flat, E, \
            F sharp, G, A, B flat. The violins start it on a chord of that scale; the high D \
            clarinet, ff marcatissimo, the flutes, the oboes and the piccolo trumpet shriek a \
            fanfare over it. The bassoons run up an arpeggio, a trombone answers, the horns call, \
            A, D, A, with the piccolo and the flute replying. The oboes' tune climbs over the \
            clarinets' sweeps, crescendo poco a poco, until the whole wind band takes over; a horn \
            calls again, and at 45 the whole orchestra plays chords, fff, the horns with their \
            bells raised. From 46 the flutes' runs alternate with strokes of the timpani and the \
            basses, and from 47 the strings' figure with strokes of the whole orchestra; four bars \
            of chords end it, and a trill in the flutes, under a fermata, leads into the Spring \
            Rounds.",
};

// Copyright: this is Stravinsky's 1947 revision of Petrushka, which is still under
// copyright (Boosey & Hawkes); it is added here at the request of the project's owner.
// Source: Stravinsky's Petrushka, 1947 revision, first tableau, "The Shrovetide Fair", MIDI
// bars 2 to 298: from the beginning up to the Russian Dance, which follows as a song of its own
// ("Petrushka, Russian Dance"), because the whole tableau needs more than 99 patterns. The
// notes are converted by script from a MIDI file of the 1947 version (the person's own copy, at
// sounding pitch) and checked bar by bar against that file, not against a printed score; MIDI
// bar numbers are given below (MIDI bar 1 is a silent lead-in). The melody is the line in
// front, in the octave of the instrument that has it, and played_by follows it from instrument
// to instrument. Everything else that matters is written out on the Keys channel: the harmony,
// the bass, the ostinati and the timpani; lines doubled in unison or octaves are written once.
// Rhythms are quantised to sixteenths and eighth-note triplets; tremolos and rolls are written
// as held notes, grace notes are left out. The dynamic of each part is read from the MIDI's
// velocities. The percussion is written out (triangle as Ride, tambourine as Shaker, bass drum
// as Kick, side drum as Snare, cymbals as Crash). Where the meter changes, every run of bars in
// one meter is a part of its own, named "..., bar N" after the bar it starts on, with in_time:
// in_meters cannot cut a written-out left hand of several voices into its runs, nor carry
// drums. No chord symbol is honest for this music, which moves in drones, ostinati and parallel
// chords, so the charts are empty.
//
// Left out: where too many lines play at once, the piano (MIDI bars 86 to 89, 147 to 155, 219
// to 222, 259 and 287 to 298), which mostly doubles the orchestra here, and the harps (the
// second harp in bars 147 to 155, both in bars 259 and 287 to 298); in the climax, bars 249 to
// 258, the woodwinds' and harps' quintuplet figures, keeping the tune, the horns' chords, the
// sixteenths of the second flute and the bassoon, and the strings' runs.
//
// Not certain: the piccolo's and first flute's septuplets (seven notes to a bar of 3/4, MIDI
// bars 26 to 41 and 169 to 174) are written as the nearest sixteenths; quintuplet and septuplet
// runs (bars 12, 14, 22, 23) as sextuplets; the 5/8 bars are counted 2 + 3, which the MIDI
// cannot tell. The Charlatan's flute cadenza (bars 277 to 286) is free in the score and has odd
// meters in the MIDI (6/8, 21/16, four bars of 32/32, two of 15/16, 3/8 and 15/32): it is
// written here in eight bars of 4/4 and one of 3/4, with the MIDI's notes and tempos and a
// rhythm of this transcription's own. The keys of the Charlatan and of the puppets coming to
// life (bars 270 to 276 and 287 to 298) are chromatic: no scale fits them.
pub(super) const PETRUSHKA_FAIR: Song = Song {
    name: "Petrushka, The Shrovetide Fair",
    origin: "Igor Stravinsky, Petrushka, 1911, revised 1947",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: D,
    mode: Dorian,
    time: THREE_FOUR,
    tempo: 138.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FLUTE, gm::ACOUSTIC_GRAND_PIANO, gm::CONTRABASS),
    style: "",
    parts: &[
        // MIDI bars 2 to 17.
        Part::new(
            "Fair",
            "A5:2 D6 A5:1 D6:7 | A5:1 D6:7 A5:1 D6:3 | (3 _:2 E6 A6 (3 G6 E6 D6 A5:4 | _:1 D6:3 \
             (3 _:2 E6 A6 (3 G6 E6 D6 | A5 D6 A5:1 D6:7 | A5:12 | _:6 D6:2 A5:4 | _:2 D6 A5:4 (3 \
             _:2 D6 A5 | D6:4 A5:2 D6 (3 A5 B5 E6 | D6 B5 A5:4 D6:2 B5 | A5:4 - (3 D4:1 A4 D5 (3 \
             _ E5 A5 | Bb5 C6 Bb5 C6 Bb5 C6 Bb5 C6 Bb5 C6 Bb5 C6 | Bb5 C6 Bb5 C6 Bb5 C6 Bb5 C6 \
             Bb5 C6 Bb5 C6 | A5 G5 A5 G5 A5 G5 A5 G5 A5 G5 A5 G5 | A5 G5 A5 G5 A5 G5 A5 G5 A5 G5 \
             A5 G5 | A5 G5 A5 G5 A5 G5 A5 G5 A5 G5 A5 G5",
            "",
        )
        .with_left(
            "-:12 | - | - | - | - | - | - | - | - | - | - | - | - | -:4 Bb1+G2+Bb2 Bb1+G2+Bb2 | \
             Bb1+G2+Bb2 Bb1+G2+Bb2 E2+C3+E3 | D2+Bb2+D3 C2+A2+C3 Bb1+G2+Bb2 & -:12 | - | - | - | \
             - | - | - | - | - | - | - | - | - | -:4 G1+G3 G1+G3 | G1+G3 G1+G3 C4:2 - | -:4 A3 \
             G3 & D4+A4:2 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | \
             D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 \
             E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 \
             D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 \
             E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 \
             A4 E4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | \
             Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 & -:12 | - | - | - | - | -:2 B4 -:4 E5:1 -:3 | (3 E5:2 D5 C#5 \
             B4:4 (3 E5:2 D5 C#5 | B4:6 -:2 (3 E5 D5 C#5 | B4:4 -:8 | -:12 | - | - | -:8 (3 \
             Bb4+Bb5:1 - G4 (3 F4+F5 E5 D4+D5 | -:2 A5:1 G5 A5 G5 A5 G5 -:2 A5:1 G5 | A5 G5 A5 \
             G5 -:2 A5:1 G5 A5 G5 A5 G5 | -:2 A5:1 G5 A5 G5 A5 G5 -:2 A5:1 G5 & -:12 | - | - | - \
             | - | - | - | - | - | - | -:8 (3 -:1 E4 - (3 A4 - - | Bb5 -:11 | -:12 | A5:1 G5 -:6 \
             A5:1 G5 -:2 | -:4 A5:1 G5 -:6 | A5:1 G5 -:6 A5:1 G5 -:2 & D5+A5:1 E5+G5 D5+A5 E5+G5 \
             D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 \
             E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 \
             D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 \
             E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 \
             D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 \
             E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 \
             D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 \
             E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | \
             D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 \
             E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D4+D5+A5 E4+G4+E5+G5 G4+C5+D5+A5 \
             A4+D5+E5+G5 | Bb4+D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 \
             E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | \
             Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 | \
             Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 | \
             Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bars 18 to 25.
        Part::new(
            "Flute again",
            "A5:2 D6 A5:1 D6:7 | A5:1 D6:5 -:2 (3 D6 E6 A6 | (3 G6 E6 D6 A5:4 - | A5:2 D6 (3 A5 \
             D6 A5 D6:4 | (3 A5:2 B5 E6 D6 B5 (3 A5:1 A4 D5 (3 _ E5 A5 | Bb5 -:11 | A5:1 G5 A5 \
             G5 A5 G5 A5 G5 A5 G5 A5 G5 | A5 G5 A5 G5 A5 G5 A5 G5 A5 G5 A5 G5",
            "",
        )
        .with_left(
            "-:12 | - | - | - | - | - | -:4 Bb1+G2+Bb2 Bb1+G2+Bb2 | Bb1+G2+Bb2 Bb1+G2+Bb2 \
             Bb1+G2+Bb2+Bb4+D5:1 - Bb1+G2+Bb2+Bb4+D5 - & -:12 | - | - | - | - | - | -:4 G1+G3 \
             G1+G3 | G1+G3 G1+G3 G1+G3+A5:1 - G1+G3+A5 - & D4+A4:2 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 \
             | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 \
             E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 A4 E4 | D4+A4+Bb4 E4+G4+C5 \
             D4+A4+Bb4 E4+G4+C5 D4+A4+Bb4 E4+G4 | Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & \
             D3:4 -:8 | -:12 | -:6 B4:2 -:4 | E5:1 -:3 (3 E5:2 D5 C#5 B4:4 | (3 E5:2 D5 C#5 B4:4 \
             - | -:8 (3 -:1 - G4 (3 F4+F5 E5 D4+D5 | -:2 A5:1 G5 A5 G5 A5 G5 -:2 A5:1 G5 | A5 G5 \
             A5 G5 -:2 A5:1 G5 - G5 - G5 & -:12 | - | - | - | -:8 (3 -:1 E4 - (3 A4 E5 - | -:12 \
             | A5:1 G5 -:6 A5:1 G5 -:2 | -:4 A5:1 G5 -:6 & D5+A5:1 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 \
             D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 \
             E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 \
             D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 \
             E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D4+D5+A5 \
             E4+G4+E5+G5 G4+C5+D5+A5 A4+D5+E5+G5 | D5+A5+Bb5 E5+G5+C6 D5+A5+Bb5 E5+G5+C6 \
             D5+A5+Bb5 E5+G5+C6 D5+A5+Bb5 E5+G5+C6 D5+A5+Bb5 E5+G5 D5+A5 E5+G5 | Bb4+D5 C5+E5 \
             Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 | Bb4+D5 C5+E5 \
             Bb4+D5 C5+E5 Bb4+D5 C5+E5 Bb4+D5 C5+E5 - C5+E5 - C5+E5",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bars 26 to 30.
        Part::new(
            "Piccolo",
            "-:2 G6:1 G6:4 G6:2 G6:1 G6 - | -:2 G6:1 G6:4 G6:2 G6:1 G6 - | -:2 G6:1 G6:4 G6:2 \
             G6:1 C7:2 | (3 Bb6:1 C7 Bb6 (3 A6 _ G6 - F6:2 A6 G6:1 -:2 | - G6:1 G6:4 G6:2 G6:1 \
             G6 -",
            "",
        )
        .with_left(
            "-:12 | C2+F3:4 Bb1 A1+A3:2 G1+G3 | -:12 | - | - & Bb1+G2+Bb2:4 Bb1+G2+Bb2 \
             Bb1+G2+Bb2 | E2+C3+E3 D2+Bb2+D3 C2+A2+C3+Bb4+D5+A5:1 - Bb1+G2+Bb2+Bb4+D5+A5 - | \
             A1+F2+A2:4 C2+A2+C3 Bb1+G2+Bb2 | Bb1+G2+Bb2 C2+A2+C3 Bb1+G2+Bb2+Bb4+D5+A5:1 - \
             A1+F2+A2+Bb4+D5+A5 - | Bb1+G2+Bb2:4 A1+F2+A2 C2+A2+C3 & G1+G3:4 G1+G3 G1+G3 | C4:2 \
             -:10 | F3:4 A3 G3 | G3 A3 G3:2 F3 | G3:4 F3 A3 & Bb3+D4+A4:2 C4+E4+G4 Bb3+D4+A4 \
             C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 \
             C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 \
             C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 \
             C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 | Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 - C5+E5+G5 - \
             C5+E5+G5 | Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 | Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 - C5+E5+G5 - C5+E5+G5 | Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 31 to 32.
        Part::new(
            "Piccolo, bar 6",
            "-:2 G6:1 G6:3 G6:2 | G6 G6:1 G6:2 G6:1 G6 -",
            "",
        )
        .with_left(
            "Bb1+Bb2:4 - | -:8 & G1+G2+G3:4 - | -:8 & Bb3+D4+A4:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | \
             Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 | Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 33 to 34.
        Part::new(
            "Piccolo, bar 8",
            "G6:1 G6:2 A6:1 (3 _ G6 _ (3 F6 _ G6 - F6:2 A6:1 | -:2 G6:1 G6:4 G6:2 G6:1 C7 -",
            "",
        )
        .with_left(
            "Bb3+D4+A4:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & -:12 | -:1 C4+E4+G4 Bb3+D4+A4 -:2 \
             C4+E4+G4:1 Bb3+D4+A4 -:2 C4+E4+G4:1 Bb3+D4+A4 - & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 | Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 35.
        Part::new(
            "Piccolo, bar 10",
            "Bb6:1 C7 A6 G6:2 F6:1 A6 -",
            "",
        )
        .with_left(
            "Bb3+D4+A4:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & -:1 C4+E4+G4 Bb3+D4+A4 -:2 C4+E4+G4:1 \
             Bb3+D4+A4 - & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 36.
        Part::new(
            "Piccolo, bar 11",
            "-:2 G6:1 G6:4 G6:2 G6:1 C7 -",
            "",
        )
        .with_left(
            "Bb3+D4+A4:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & -:1 C4+E4+G4 Bb3+D4+A4 \
             -:2 C4+E4+G4:1 Bb3+D4+A4 -:2 C4+E4+G4:1 Bb3+D4+A4 - & Bb4+D5+A5:1 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)]),
        // MIDI bar 37.
        Part::new(
            "Piccolo, bar 12",
            "Bb6:1 C7 A6 G6:2 F6:1 A6 -",
            "",
        )
        .with_left(
            "-:1 C4+E4+G4 Bb3+D4 -:2 C4+E4+G4:1 Bb3+D4 - & (3 Bb3+D4+A4:1 _ A4 (3 _ _ - (3 - F4 \
             - (3 - A4 - & -:2 C4+E4+G4 Bb3+D4+A4 (3 C4+E4+G4:1 _ - & Bb4+D5+A5:1 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 38 to 39.
        Part::new(
            "Piccolo, bar 13",
            "G6:1 G6:2 A6:1 (3 _ G6 _ (3 F6 _ G6 - F6:2 A6:1 | -:2 G6:1 G6:4 G6:2 G6:1 C7 -",
            "",
        )
        .with_left(
            "-:1 C4+E4+G4 Bb3+D4+A4 -:2 C4+E4+G4:1 Bb3+D4+A4 -:2 C4+E4+G4:1 Bb3+D4+A4 - | - \
             C4+E4 Bb3+D4 -:2 C4+E4:1 Bb3+D4 -:2 C4+E4:1 Bb3+D4 - & Bb3+D4+A4:2 C4+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 | Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 40 to 41.
        Part::new(
            "Piccolo, bar 15",
            "Bb6:1 C7 A6 G6:2 F6:1 C7:2 | Bb6:1 C7 A6 G6:2 F6:1 C7 -",
            "",
        )
        .with_left(
            "-:1 C4+E4+G4 Bb3+D4 -:2 C4+E4+G4:1 Bb3+D4+A4 - | - C4+E4+G4 Bb3+D4 -:2 C4+E4+G4:1 \
             Bb3+D4+A4 - & -:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | (3 Bb3+D4+A4:1 _ - C4+E4+G4:2 \
             Bb3+D4+A4 (3 C4+E4+G4:1 _ - & (3 Bb3+D4+A4:1 _ A4 (3 _ _ - (3 - F4 - (3 - C5 - | (3 \
             - - A4 (3 _ _ - (3 - F4 - (3 - C5 - & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 | Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 42.
        Part::new(
            "Crowd",
            "(3 Bb6:1 C7 Bb6 (3 A6 G6 _ (3 F6 G5 A5 (3 Bb5 C6 E6 F6 -:3",
            "",
        )
        .with_left(
            "(3 Bb3+E4+A4:1 _ A5 C4+E4+G4:2 Bb3+E4+F4+A4 C4+E4+G4+A4 C4+E4+G4+Bb4+D5:4 & (3 -:1 \
             - A4 (3 _ G5 - -:8 & (3 -:1 C5 Bb4 (3 - E5 - (3 D4 - G4 (3 - B4 D5 E5 -:3 & -:4 (3 \
             -:1 C4+G4 - (3 - C5 Bb4+G5 A4+F5+A5 -:3 & D4:2 - (3 -:1 C5 D4+D5 (3 E5 F4+F5 - -:4 \
             & (3 Bb4+D5+A5+Bb5:1 C6 C5+E5+G5+Bb5 Bb4+D5+A5+A6 C5+E5+G5+G6 Bb4+D5+F5+F6 C5+E5 - \
             C5+E5 -:4 & -:4 A5:2. Bb4+D5+A5+A6:1 - G5+Bb5+D6+G7:4",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "-:8 X:4"),
            (Drum::Crash, "-:8 x:4"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 43 to 59.
        Part::new(
            "Crowd tune",
            "G7:4 G7 | G7 G7:2 G7 | G7:4 G7 | G7 G7 | G7 G7 | G7 C8 | Bb7 A7:2 G7 | F7:4 A7 | G7 \
             G7 | G7 C8 | Bb7 A7:2 G7 | F7:4 A7 | G7 C8 | Bb7 A7:2 G7 | F7:4 A7 | G7 G7 | A7 \
             G7:2 F7",
            "",
        )
        .with_left(
            "-:8 | - | - | - | - | - | - | - | - | - | - | - | F4+Bb4:2 -:6 | -:8 | -:4 A3+E4+E5 \
             | G3+Bb4+D5 G3 | -:8 & Bb3+G4+D5+F5:4 A3+E4+G4+C5+D5 | G3+Bb4+D5 C4+E4+G4+Bb4+D5:2 \
             C4+E4+G4+Bb4+D5 | Bb3+G4+D5+F5:4 A3+E4+G4+C5+D5 | Bb3+F4+G4+D5+A5 C4+G4+Bb4+D5+A5 | \
             Bb3+G4+D5+F5+A5 A3+G4+C5+D5+A5 | G3+Bb4+D5+A5 E3+C5+A5 | F3+Bb4+C5+D5+G5 A4+F5:2 \
             Bb4+D5+E5 | A3+D5+F5:4 E4+A4+F5 | -:8 | Bb3+D4:2 - A3+C4 - | -:4 G3+C4+E4+G4+C5 | - \
             E3+A4+C5:2 F3+Bb4+D5 | G3+C4+E4+G4 F3+Bb3+D4+F4+Bb4 -:4 | - G3+C4+E4+G4+C5 | - \
             E3+A4+C5:2 F3+Bb4+D5 | C4+E4+G4+C5+E5:4 Bb3+G4+D5+A5 | F3+A4+C5+E5 G3+Bb3+D4+G4+Bb4 \
             & -:4 Bb4 | -:8 | -:4 Bb4 | G3 A3 | - Bb4 | -:8 | -:4 A3+E5:2 - | C5:4 E5 | \
             G3+Bb4+D5 G3+D5 | G3 - | - E5:2 - | C5:4 A3+E4+E5 | D5 - | - E5:2 - | C5:4 - | -:8 \
             | -:6 F4:2 & -:8 | - | - | - | - | - | -:4 G3+C4+E4+G4+C5 | - E3+A3+C4+C5:2 \
             F3+Bb3+D4+Bb4+D5 | C4+D4+G4+E5:4 C4+F4+G4+Bb4+E5 | F3+G4+Bb4+D5+E5 E3+C5+A5 | \
             F3+Bb4+C5+D5+G5 A4+F5:2 Bb4+D5+E5 | A3+D5+F5:4 F5 | C5+E5 E3+C5+A5 | \
             F3+Bb4+C5+D5+G5 A4+F5:2 Bb4+D5+E5 | A3+D5+F5:4 F5 | -:8 | -:4 D5:2 A4+C5 & Bb5+D6:4 \
             Bb5+D6 | Bb5+D6 Bb5+D6:2 Bb5+D6 | Bb5+D6:4 Bb5+D6 | Bb5+D6 Bb5+D6 | Bb5+D6 Bb5+D6 | \
             G5+Bb5+D6+G6 G5+C6+E6+G6 | F5+Bb5+D6+F6 A5+C6+E6+A6:2 G5+Bb5+D6+G6 | A5+C6:4 C6+E6 \
             | Bb5+D6 Bb5+D6 | G5+Bb5+D6+G6 G5+C6+E6+G6 | F5+Bb5+D6+F6 A5+C6+E6+A6:2 \
             G5+Bb5+D6+G6 | A5+C6:4 C6+E6 | G5+Bb5+D6+G6 G5+C6+E6+G6 | F5+Bb5+D6+F6 \
             A5+C6+E6+A6:2 G5+Bb5+D6+G6 | A5+C6:4 C6+E6 | Bb5+D6 Bb5+D6 | C6+E6 Bb5+D6:2 A5+C6 & \
             G5+G6:4 G5+G6 | G5+G6 G5+G6:2 G5+G6 | G5+G6:4 G5+G6 | G5+G6 G5+G6 | G5+G6 G5+G6 | - \
             C7 | Bb6 - | F6 A5+A6 | G5+G6 G5+G6 | - C7 | Bb6 - | F6 A5+A6 | - C7 | Bb6 - | F6 \
             A5+A6 | G5+G6 G5+G6 | A5+G6+G7 G5+F6+F7:2 F5+E6+E7",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "X:4 x | X x:2 x | X:4 X | X x | X x | X x | X X:2 X | X:4 x | X x | X \
                          X | X x:2 x | X:4 x | X X | X X:2 X | X:4 X | X X | X X:2 x"),
            (Drum::Crash, "X:4 X | X x:2 X | X:4 X | X X | X X | X X | X X | X X:2 x | X:4 X | X \
                           X | X X | X X:2 x | X:4 X:2 X | X:4 x | X X:2 x | X:4 X | X X"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 60.
        Part::new(
            "Crowd, 3/4",
            "G7:4 F7 A7",
            "",
        )
        .with_left(
            "-:8 E4:4 & G3+Bb4+C5+D5+E5:4 A3+F4+C5+E5 E3+F4+A4+C5+E5 & Bb5+D6:4 A5+C6 C6+E6 & \
             G5+F6+F7:4 F5+E6+E7 A5+G6+G7",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "X:4 X X"),
            (Drum::Crash, "X:4 X x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 61 to 62.
        Part::new(
            "Crowd, end",
            "G7:2 F7 G7:4 | G7:1 -:7",
            "",
        )
        .with_left(
            "G4:2 F4 Bb3+D4+Bb4:4 | G3 - & A3+Bb3+D4+A4+Bb4:4 - | Bb3+D4+D5:1 -:3 Bb3+G4:4 & \
             D5:2 C5+E5 G3+C5+D5+E5:4 | G4+Bb4:2. -:6 & -:8 | G5+Bb5:1 -:3 Bb4+G5:4 & Bb5+D6:2 \
             A5+C6 Bb5+D6:4 | D6:2. -:6 & G5+F6+F7:2 F5+E6+E7 G5+F6+F7:4 | F6+F7:1 -:7",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "X:2 x X:4 | X:8"),
            (Drum::Crash, "X:4 X | X:8"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 63 to 69.
        Part::new(
            "Merrymakers",
            "G5:4 G5 | G5 C6:2 C6 | C6:4 C6:2 C6 | C6:4 F6:2 F6 | Eb6:4 D6:2 C6 | Bb5:4 D6 | C6 \
             C6:2 F5:1 -",
            "",
        )
        .with_left(
            "-:8 | -:4 Eb4:2 - | -:4 Eb4:2 Eb4:1 - | -:8 | - | - | - & Bb3+G4:4 Bb3+G4 | Bb3+G4 \
             Bb3+G4+G5 | Bb3+G4+G5 C5+C6 | D4+Bb4+Bb5 C4:2 Bb3+G4+G5 | A3+F4+F5:4 C4+A4+A5 | \
             D4+G4+D5+G5 Bb3+G4+G5 | C4+A4+A5 Bb3+G4+G5:2 A3+F4+F5 & Bb4:4 Bb4 | Bb4 Eb5:2. \
             Eb4+Eb5 | Eb4+Bb4+Eb5:4 - | Eb4+D5+Eb5 - | G4+A4+G5 F4+F5:2 Eb4+Eb5:1 - | -:8 | -:4 \
             Bb4:2 A4:1 - & -:8 | -:4 Bb4 | - Eb5 | - A4+C5+A5:2 A4+Bb4+A5 | -:4 C5 | Bb4+F5 \
             Bb4+F5 | C5+Eb5 Eb5 & -:8 | -:4 C5+C6 | C5 - | C5 F5+F6 | Eb5 - | F4 F4 | Eb4 Eb4",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "-:8 | -:4 x | - x | - x | - x | - x | - x"),
            (Drum::Shaker, "-:8 | -:6 x:2 | -:6 x:2 | -:6 x:2 | -:6 x:2 | -:6 x:2 | -:6 x:2"),
            (Drum::Crash, "-:8 | - | x | x | x | x | x"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 70.
        Part::new(
            "Merrymakers, 3/4",
            "D6:4 C6:2 Bb5 C6:4",
            "",
        )
        .with_left(
            "Bb3+G4:2 - A3+F4:4 C4+A4 & F4:4 Eb4:2 D4 Eb4:4 & Bb4+G5:2 - A4+F5:4 - & F5:4 Eb5:2 \
             D5 C5+Eb5+A5:4",
        )
        .in_key(C, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "-:4 x:8"),
            (Drum::Shaker, "-:6 x"),
            (Drum::Crash, "x:8 x:4"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 71 to 74.
        Part::new(
            "Merrymakers, end",
            "Bb5:4 D6 | C6:1 -:7 | -:8 | -",
            "",
        )
        .with_left(
            "F4:4 - | C6 - | -:8 | -:6 (3 -:1 D2 C2 & Bb3+D5:4 G4 | -:8 | - | -:6 E2:1 - & \
             D4+G4:4 Bb3+F4 | C4:8 | _ | _:6 -:2 & Bb4+F5+G5:4 Bb4+F5+G5 | E4+Bb4+E5+G5:1 -:7 | \
             -:8 | -:6 (3 E3:1 D3 C3",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "x:8 | - | - | -"),
            (Drum::Shaker, "-:2 x:4 (3 _:1 x x | X:8 | - | -"),
            (Drum::Crash, "-:4 x | -:8 | - | -"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 75.
        Part::new(
            "Revellers",
            "-:2 E5 E5",
            "",
        )
        .with_left(
            "Bb2:2 -:4 & Bb1:2 D4+E4+D5 D4+E4+D5",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 76.
        Part::new(
            "Revellers, bar 2",
            "-:2 E5 E5 A5 G5 F#5:1 -",
            "",
        )
        .with_left(
            "-:6 D4:2 G4 E4:1 - & -:2 D4+E4+D5 D4+E4+D5 -:6 & -:6 A4:2 D4+E5 D4+F#4+E5:1 - & -:8 \
             D5:2 - & -:4 E5 -",
        )
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 77 to 79.
        Part::new(
            "Revellers, bar 3",
            "A5:2 G5 F#5 A5 G5 | E5 E5 E5:4 E5:2 | A5 G5 F#5 A5 G5:1 -",
            "",
        )
        .with_left(
            "D4:2 - E4 - G4:1 - | -:10 | D4:2 - E4 - G4:1 - & -:10 | -:8 D4:2 | -:6 D4:2 - & \
             E5:2 D4+E5 D4+E5 D4:3 -:1 | E4:2 D4+E4 D4+E4:4 E4:2. | E5 D4+E5 D4+F#4+E5 - D4+E5:1 \
             - & E4+A4:2 G4+D5 F#4+D5 A4+E5 E4. | -:10 | E4:2 G4+D5 D5 A4+E5 D5+G5",
        )
        .in_time(FIVE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 80.
        Part::new(
            "Revellers, bar 6",
            "E5:2 E5 E5",
            "",
        )
        .with_left(
            "E4:2 D4+E4 D4+E4",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 81.
        Part::new(
            "Piccolo in 5/8",
            "-:6 (3 E6:1 F6 G6",
            "",
        )
        .with_left(
            "-:2 D4+E4+D5+E5 - (3 E4+E5:1 F4+F5 G4+G5 & -:4 D4+E4+D5+E5:2 D4+D5:1 - & -:6 (3 \
             C6:1 D6 C6+E6",
        )
        .in_time(TWO_FOUR)
        .in_key(D, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "-:6 (3 x:1 x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 82 to 85.
        Part::new(
            "Piccolo in 5/8, bar 2",
            "A6:2 D7 D7 A6 A6 | D7 D7 A6 A6 D7 | D7 A6 A6 D7 D7 | A6 A6 D7 D7 A6:1 -",
            "",
        )
        .with_left(
            "D4+F4+A4:2 G4+B4+D5 G4+B4+D5 D4+F4+A4 D4+F4+A4 | G4+B4+D5 G4+B4+D5 D4+F4+A4 \
             D4+F4+A4 G4+B4+D5 | G4+B4+D5 D4+F4+A4 D4+F4+A4 G4+B4+D5 G4+B4+D5 | D4+F4+A4 \
             D4+F4+A4 G4+B4+D5 G4+B4+D5 D4+F4+A4:1 - & E4+D5+A5:1 -:9 | -:10 | - | - & E5:2 -:8 \
             | D4+E5+G5+D6:2 -:8 | D4+E5+G5+D6:2 -:8 | D5+E5+A5:2 -:8 & D2+D3+F5+D6+F6:2 \
             B5+G6+B6 B5+G6+B6 F5+D6+F6 F5+D6+F6 | B5+G6+B6 B5+G6+B6 F5+D6+F6 F5+D6+F6 B5+G6+B6 \
             | B5+G6+B6 F5+D6+F6 F5+D6+F6 B5+G6+B6 B5+G6+B6 | F5+D6+F6 F5+D6+F6 B5+G6+B6 \
             B5+G6+B6 F5+D6+F6:1 -",
        )
        .in_time(FIVE_EIGHT)
        .in_key(D, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "x:10 | - | - | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 86 to 89.
        Part::new(
            "Flute tune",
            "A5:2 D6 A5:1 D6:7 | (3 _:2 E6 A6 (3 G6 E6 D6 A5:4 | A5:1 D6:3 A5:1 D6:7 | A5:1 D6:7 \
             A5:1 D6:2 -:1",
            "",
        )
        .with_left(
            "-:12 | - | - | -:10 E2:1 - & -:12 | - | - | -:10 (3 -:1 D2 C2 & D4+A4:2 E4+G4 D4+A4 \
             E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 \
             D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 & -:12 | (3 -:1 - E5. (3 _ _ - (3 \
             - - E5 (3 _ D5. _ _ -:3 | -:12 | -:10 (3 E3:1 D3 C3 & D3+A3+E5+F5:1 E4+G4+E5+G5 \
             D4+A4+A5 D4+E5+G5 E5 E4+G4+E5+G5 D4+A4+D5+A5 D4 E5 E4+G4+E5+G5 D4+A4+D5+A5 D4 | E5 \
             E4+G4+G5 D4+A4+D5+A5 D4 E5 E4+G4+G5 D4+A4+A5 D4+E5 E5 E4+G4+E5+G5 D4+A4+D5+A5 D4 | \
             E5 E4+G4+E5+G5 D4+A4+D5+A5 D4 E5 E4+G4+E5+G5 D4+A4+D5+A5 D4 E5 E4+G4+E5+G5 \
             D4+A4+D5+A5 D4 | E5 E4+G4+E5+G5 D4+A4+D5+A5 D4 E5 E4+G4+E5+G5 D4+A4+D5+A5 D4 E5 \
             E4+G4+E5+G5 (3 D4+A4+D5+A5 _ - & D5:2 D5+E5+G5:1 - D5+A5:2 E5+G5 D5+A5 E5+G5 | \
             D5+A5 G5 D5+A5 G5 - E5+G5 | D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 D5+A5 \
             E5+G5 D5+A5 E5+G5",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bar 90.
        Part::new(
            "Violins",
            "-:6",
            "",
        )
        .with_left(
            "Bb2:2 -:4 & Bb1:2 E4 E4 & -:2 D4+D5+E5 D4+D5+E5",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 91.
        Part::new(
            "Violins, bar 2",
            "-:6 D5:1 D5 G5 G5 E5 E5 A5 A5 G5 G5",
            "",
        )
        .with_left(
            "-:4 E4:1 E4 A4+Bb5 A4+Bb5 G4 G4 F#4+F#5 F#4+F#5 E4+A4 E4+A4 -:2 & -:2 D4+D5+E5 \
             D4+D5+E5 D4+D5+E5 D4+D5+E5 D4+D5+E5 D4+D5+E5 D4+D5+E5",
        )
        .in_time(COMMON)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)]),
        // MIDI bar 92.
        Part::new(
            "Violins, bar 3",
            "F#5:1 F#5 E5 -:3",
            "",
        )
        .with_left(
            "Bb1:6 & D4+E4+D5+E5:2 D4+E4+D5+E5 D4+E4+D5+E5 & -:2 Bb2+F5+D6:1 D6+F6 D6+F6 F5+D6",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 93 to 94.
        Part::new(
            "Organ grinder",
            "-:6 F5:2 Bb5:4 | _:2 D5 D5 D5 C5 Bb4",
            "",
        )
        .with_left(
            "(3 Bb3+D4:1 _ - -:2 Bb3+D4:1 - F3:2 Bb3:4 | _:2 D3 D3+Bb3:1 - D3:2 (3 F2+F3:1 _ - \
             -:2 & E4:4 F4:8 | _:12 & (3 -:1 F5+D6 D6+F6 (3 F5 D6+F6. _ _ -:3 (3 Bb2:1 _ - -:2 | \
             (3 D4:1 _ - -:8 Bb3:2 & -:8 (3 -:1 F5+D6 D6+F6 (3 F5 D6+F6. _ | (3 _ F5+D6 D6+F6 (3 \
             F5 D6+F6. _ _ -:3 (3 -:1 F5+E6 E6+F6 (3 F5 E6+F6 -",
        )
        .with_tempo(&[tempo(0, 88)])
        .in_key(F, Major)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 95.
        Part::new(
            "Organ grinder, bar 3",
            "F5:4 F5",
            "",
        )
        .with_left(
            "D4+F4+G4:8 & (3 Bb3+Eb4:1 F5+Eb6 Eb6+F6 (3 F5 Eb6+F6 - Bb3+Eb4 -:3",
        )
        .in_time(TWO_FOUR)
        .in_key(F, Major)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 96.
        Part::new(
            "Organ grinder, bar 4",
            "-:4 D4:1 -:3 D4:1 -:3",
            "",
        )
        .with_left(
            "-:10 (3 A1:1 G1 F1 & (3 F2:1 _ - -:2 (3 Bb3:1 _ - -:2 (3 Bb3:1 _ - (3 A2+A3 G2+G3 \
             F2+F3 & D4+F4+A4:10 (3 _:1 _ - & (3 -:1 F5+D6 D6+F6 (3 F5 D6+F6. _ (3 _ F5+D6 D6+F6 \
             (3 F5 D6+F6 - -:4",
        )
        .in_key(F, Major)
        .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bar 97.
        Part::new(
            "Revellers again",
            "-:2 E5 E5",
            "",
        )
        .with_left(
            "E3:2 -:4 & E1+E2:2 D4+E4+D5 D4+E4+D5",
        )
        .in_time(THREE_EIGHT)
        .with_tempo(&[tempo(0, 140)])
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 98.
        Part::new(
            "Revellers again, bar 2",
            "-:2 E5 E5 A5 G5 F#5:1 -",
            "",
        )
        .with_left(
            "-:6 D4:2 G4 E4:1 - & -:6 A4:2 - D4+F#4+E5:1 - & -:2 D4+E4+D5 D4+E4+D5 -:6 & -:8 \
             D4+D5:1 -:3 & -:4 E5 E5:2 -",
        )
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 99 to 104.
        Part::new(
            "Piccolo again",
            "-:8 (3 E6:1 F6 G6 | A6:2 A6 D7 D7 A6 | A6 E6 E6 A6 A6 | A6 D7 D7 A6 A6 | D7 D7 A6 \
             A6 D7 | A6 E6 E6 A6 A6:1 -",
            "",
        )
        .with_left(
            "D4:2 - E4 - G4:1 - | D4+F4+A4:2 D4+F4+A4 G4+B4+D5 G4+B4+D5 D4+F4+A4 | D4+F#4+A4:1 - \
             A3+C4+E4:2 C4+E4 D4+F#4+A4 D4+F#4+A4 | D4+F4+A4 G4+B4+D5 G4+B4+D5 D4+F4+A4 D4+F4+A4 \
             | G4+B4+D5 G4+B4+D5 D4+F4+A4 D4+F4+A4 G4+B4+D5 | D4+F#4+A4:1 -:5 D4+F#4+A4:2 \
             D4+F#4+A4:1 - & E4+A4:2 G4+D5 F#4+D5 A4+E5 E4+G5. | D2+D3+F5+A5+D6+F6:1 -:9 | -:4 \
             A3:2. -:4 | -:10 | - | -:2 A3+C4+E4 A3+C4+E4:1 -:5 & E5+A5:2 D4+E5+G5 D4+E5+F#5 \
             D4+A5 D4:1 - | -:2 D5 D6 G5. - | - C5 C5:1 -:5 | D5+A5:2 G5+D6 G5:1 -:5 | G5+D6:2 \
             D6 -:6 | -:10 & -:10 | D5:2 -:8 | -:10 | - | -:2 G5. D5+A5 D5+A5. - | - C5 C5:1 -:5 \
             & -:9 E6:1 | -:2 F5+A5+D6+F6 G5+B5+G6+B6 B5+D6+G6+B6 G5+B5+D6+F6 | D5+F#5+A5+D6+F#6 \
             A4+E5+A5+C6 A4+E5+A5+C6 D5+F#5+A5+D6+F#6 D5+F#5+A5+D6+F#6:1 - | F5+D6+F6:2 B5+G6+B6 \
             B5+G6+B6 F5+D6+F6 F5+D6+F6 | B5+G6+B6 B5+G6+B6 F5+D6+F6 F5+D6+F6 B5+G6+B6 | \
             D5+F#5+A5+D6+F#6 A4+E5+A5+C6 A4+E5+A5+C6 D5+F#5+A5+D6+F#6 D5+F#5+A5+D6+F#6:1 -",
        )
        .in_time(FIVE_EIGHT)
        .in_key(D, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "-:8 (3 x:1 x x | x:10 | - | - | - | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 105 to 106.
        Part::new(
            "Violins again",
            "A5:1 D6:3 A5:1 D6:7 | A5:1 D6:7 A5:1 D6:3",
            "",
        )
        .with_left(
            "D3+G3:1 -:11 | -:12 & -:1 E4+G4 D4+G4+A4 E4+G4 A4 E4+G4 D4+G4+A4 E4+G4 A4 E4+G4 \
             D4+G4+A4 E4+G4 | A4 E4+G4 D4+G4+A4 E4+G4 A4 E4+G4 D4+G4+A4 E4+G4 A4 E4+G4 D4+G4+A4 \
             E4+G4 & A4:1 D5:3 -:1 D5:3 -:4 | -:1 D5:3 -:5 D5:3 & D4+D5+A5+D6+A6:1 E6+G6 \
             E4+E5+G5+D6+A6 E6+G6 D4+D5+A5+D6+A6 E6+G6 E4+E5+G5+D6+A6 E6+G6 D4+D5+A5+D6+A6 E6+G6 \
             E4+E5+G5+D6+A6 E6+G6 | D4+D5+A5+D6+A6 E6+G6 E4+E5+G5+D6+A6 E6+G6 D4+D5+A5+D6+A6 \
             E6+G6 E4+E5+G5+D6+A6 E6+G6 D4+D5+A5+D6+A6 E6+G6 E4+E5+G5+D6+A6 E6+G6",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 107.
        Part::new(
            "Violins again, bar 3",
            "(3 -:2 E6 A6 (3 G6 E6 D6 A5:4 A5:1 D6:3",
            "",
        )
        .with_left(
            "A4:2 G4 (3 A4+G5:1 _ E5 G4:2 A4 G4 A4 G4 & (3 -:1 - E5 (3 - A5 _ -:2 (3 -:1 D5. _ \
             -:5 D5:3 & D4+D5+A5+D6+A6:1 E6+G6 E4+E5+G5+D6+A6 E6+G6+A6 D4+D5+A5+D6+A6 E6+G6 \
             E4+E5+G5+D6+A6 D6+E6+G6 D4+D5+D6+A6 E6+G6 E4+E5+G5+D6+A6 E6+G6 D4+D5+A5+D6+A6 E6+G6 \
             E4+E5+G5+D6+A6 E6+G6",
        )
        .in_time(COMMON)
        .in_key(D, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 108 to 115.
        Part::new(
            "Barrel organ",
            "Bb5:6 D5:2 D5 D5 | C5 Bb4 F5:8 | - F5:4 | F6:6 C5:2 Bb5 A5 | - Bb5 A5:4 G5 | _ G5 \
             A5 | Bb5:2 G5 F5:6 F5:2 | G5 F5 C5 D5 Bb4 -",
            "",
        )
        .with_left(
            "A4+A5:1 D5:7 A4:1 D5:3 | -:4 A3:2 -:6 | Bb2:2 -:6 F3:4 | Bb2:2 - Bb3 -:4 A3:2 | \
             Eb3+C4 - C4 -:6 | F2:2 -:10 | -:12 | - & (3 Bb2:1 _ - -:2 D4 - Bb3 - | F2+F3 Bb3 (3 \
             Eb4:1 - - -:2 A3+Eb4:1 -:3 | -:4 (3 Bb3+D4:1 _ - -:2 D4 - | D4:6 C4:2 Bb3+D4 - | - \
             G4 (3 Eb4:1 - - -:2 C4+Eb4:1 -:3 | -:4 (3 C4+Eb4:1 _ - -:2 C4+Eb4:1 -:3 | G2+Bb4:2 \
             -:6 C4+Eb4:2 F4 | Bb2+G4+Bb5+D6+Bb6:1 -:3 Bb3+D4+Bb5+D6+Bb6:1 -:3 Bb3+D4:1 -:3 & (3 \
             -:1 F5+D6 D6+F6 (3 F5 D6+F6. _ (3 _ F5+D6 D6+F6 (3 F5 D6+F6. _ _ -:3 | (3 -:1 \
             G5+Eb6 Eb6+G6 (3 G5 Eb6+G6. _ (3 _ G5+Eb6 Eb6+G6 (3 G5 Eb6+G6. _ _ -:3 | (3 -:1 \
             G5+D6 Eb6+Bb6 (3 Bb5 D6+G6. _ (3 _ G5+D6 Eb6+Bb6 (3 Bb5 D6+G6. _ _ -:3 | (3 -:1 \
             G5+D6 Eb6+Bb6 (3 Bb5 D6+G6. _ (3 _ G5+D6 Eb6+Bb6 (3 Bb5 D6+G6. _ _ -:3 | (3 -:1 \
             Eb5+C6+Eb6 Eb5+C6+C7 (3 C6+Eb6 C6+G6+C7. _ (3 _ Eb5+C6+Eb6 D5+C6+C7 (3 C6+Eb6 \
             C6+G6+C7. _ _ -:3 | (3 -:1 Eb5+C6+Eb6 Eb5+C6+C7 (3 C6+Eb6 C6+Eb6+C7. _ (3 _ \
             Eb5+C6+Eb6 Eb5+C6+C7 (3 C6+Eb6 C6+Eb6+C7. _ _ -:3 | -:2 G4+F5+Eb6+A6:1 F5+Eb6+A6 \
             C4+Eb4+Eb6+A6 - F5+Eb6+A6:2 (3 -:1 F5+D6 G5+Eb6 (3 A5+F6 Bb5+G6 C6+A6 | - \
             Bb5+D6+Bb6 F4+Bb5+D6+Bb6 Bb5+D6+Bb6 - Bb5+D6+Bb6 -:6",
        )
        .with_tempo(&[tempo(0, 88)])
        .in_key(ASharp, Major)
        .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bars 116 to 134.
        Part::new(
            "Wooden leg",
            "-:8 | -:4 F6:3 Eb6:1 | D6:2 D6 D6 D6 | D6:4 Bb5:2 C6 | D6 C6 Bb5 D6 | C6 - Eb6:3 \
             D6:1 | C6:2 C6 C6 C6 | C6 - C6 C6 | F6 Eb6 D6 C6 | D6:1 -:3 F6:4 | _ F6:3 Eb6:1 | \
             D6:2 D6 D6 D6 | D6:4 Bb5:2 C6 | D6 C6 Bb5 D6 | C6 - Eb6:3 D6:1 | C6:2 C6 C6 C6 | \
             C6:4 C6:2 C6 | F6 Eb6 D6 C6 | Bb5:1 -:7",
            "",
        )
        .with_left(
            "-:8 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | A2:2 A3+Eb4 -:4 | \
             A2+A5:2 -:6 | -:8 & -:8 | - | - | Bb2:2 Bb3+D4 Bb3+D4+D5 Bb3+D4+Eb5 | Bb2+F5 \
             Bb3+D4+Eb5 Bb3+D4+D5 Bb3+D4+F5 | A2+Eb5 A3+Eb4 A3+Eb4+G5 A3+Eb4 | A2+Eb5 A3+Eb4+A5 \
             A3+Eb4+A5 A3+Eb4+A5 | A2+A5 A3+Eb4 A3+Eb4+A5 A3+Eb4+A5 | A2+A5 A3+Eb4+G5 A3+Eb4+F5 \
             A3+Eb4+Eb5 | -:8 | - | Bb2+F5:2 Bb3+D4+Bb5 Bb3+D4+Bb5 Bb3+D4+Bb5 | Bb2 Bb3+D4 \
             Bb3+D4+D5 Bb3+D4+Eb5 | Bb2+F5 Bb3+D4+Eb5 Bb3+D4+D5 Bb3+D4+F5 | A2+D5 A3+Eb4 \
             A3+Eb4+F5 A3+Eb4 | A2+D5 -:4 C5:2 | _ (3 _:1 _ - A3+Eb4+A5+A6 - A3+Eb4+A5:2 | - \
             A3+Eb4+G5 A3+Eb4+F5 A3+Eb4+Eb5 | - Eb5+Eb6 D5+D6 C5+C6 & -:8 | - | - | - | - | - | \
             - | - | - | - | - | - | - | - | - | - | Bb4:1 -:3 A4:1 -:3 | -:8 | - & -:8 | - | - \
             | - | - | - | - | - | - | - | - | - | -:4 Eb4:1 -:3 | -:6 F4:2 | (3 _:1 _ F4+G4 \
             F4+G4:2 -:1 G4 -:2 | (3 -:1 - G4 (3 G4 _ - (3 G4 _ G4+A4+G5 G4:2 | _:1 G4+A4 G4 \
             F4+G4 - G4 G4:2 | _:1 G4 G4:3 G4:1 G4:2 | -:8 & -:8 | - | - | - | - | - | - | - | - \
             | - | - | -:6 E4:1 E4 | D4+F4 D4+F4 F4:3 Eb4+F#4:1 Eb4+F#4:2 | _:1 Eb4+F#4 \
             Eb4+F#4:2 F4:1 F4+F#4 F#4:2 | (3 G4+G5:1 C6 _ (3 _ G5 C6 _:2 G4+D6:1 G4+G5+G6 | (3 \
             G4+F5+F6 D6 _ (3 - G5 C6 (3 _ _ - C6:2 | _:3 -:5 | -:8 | - & -:8 | - | - | - | - | \
             - | - | - | - | - | - | - | - | - | -:7 Eb5:1 | -:3 C5 A3+Eb4+A5:2 | A5:4 -:1 G6 \
             -:2 | -:8 | - & -:8 | -:4 A5:3 G5:1 | Bb2+F5:2 Bb3+D4+Bb5 Bb3+D4+Bb5 Bb3+D4+Bb5 | \
             Bb5:4 - | -:8 | -:7 F5:1 | -:8 | - | - | D5+F5:1 -:3 D6:4 | D6 A5:3 G5:1 | -:8 | \
             Bb5:4 - | -:8 | (3 -:1 C5 _ (3 _ _ C5 _:2 D5. | - A3+Eb4+C5+A5:1 - (3 A3+Eb4+A5+G6 \
             _ A6 (3 _ C7 _ | Bb6 C7 (3 G6 G5+F6 C5+C6 _ -:3 | -:8 | D5:1 -:7",
        )
        .in_time(TWO_FOUR)
        .in_key(ASharp, Major)
        .played_by(&[(gm::FLUTE, 0), (gm::CLARINET, -12)])
        .with_drums(&[
            (Drum::Ride, "x:4 x | x x | X x | x x | X x | x x | x x | x x | x x | -:8 | - | - | \
                          - | - | - | - | - | - | -"),
        ]),
        // MIDI bars 135 to 146.
        Part::new(
            "Music box",
            "-:10 F5:2 | Bb5:6 D5:2 D5 D5 | C5 Bb4 F5:4 F5 | -:8 F5:4 | Bb5:6 D5:2 D5 D5 | C5 \
             Bb4 F5:8 | - F5:4 | D6:6 C5:2 Bb5 A5 | - Bb5 A5:4 G5 | _ G5 A5 | Bb5:2 G5 F5:6 F5:2 \
             | G5 F5 C5 D5 Bb4 -",
            "",
        )
        .with_left(
            "-:12 | - | -:4 (3 A3:1 _ - -:2 A3:1 -:3 | (3 Bb2:1 _ - -:10 | (3 Bb2:1 _ - -:10 | \
             (3 A2+F3:1 _ - -:10 | (3 Bb2:1 _ - -:6 F3:4 | (3 Bb2:1 _ - -:10 | -:4 C4:2 -:6 | \
             F2:2 -:10 | -:12 | - & (3 Bb2:1 _ - -:2 (3 Bb3+D4:1 _ - -:2 Bb3+D4:1 - F3:2 | (3 \
             Bb2:1 _ - -:2 (3 D4:1 _ - -:2 Bb3:1 -:3 | (3 F2+F3:1 _ - Bb3:2 Eb4:4 Eb4 | - (3 \
             Bb3:1 _ - -:2 D4:1 -:3 | -:4 (3 D4:1 _ - -:2 Bb3:1 -:3 | -:2 Bb3 (3 A3+Eb4:1 _ - \
             -:2 A3+Eb4:1 -:3 | -:4 (3 Bb3+D4:1 _ - -:2 D4:1 -:3 | -:4 (3 Bb3:1 _ - -:2 D4:1 -:3 \
             | Eb3+C4:2 G4 (3 Eb4:1 - - -:2 C4+Eb4:1 -:3 | -:4 (3 C4+Eb4:1 _ - -:2 C4+Eb4:1 -:3 \
             | G2+Bb4:2 -:6 C4+Eb4:2 F4 | Bb2+G4+Bb5+D6+Bb6:1 -:3 Bb3+D4+Bb5+D6+Bb6:1 -:3 \
             Bb3+D4:1 -:3 & (3 F5:1 G5 F5 (3 G5 F5 _ (3 F5 G5 F5 (3 G5 F5 _ -:2 C5+C6 | D5:4 (3 \
             Bb4+F5:1 G5 F5 -:2 C5+C6 Bb4+Bb5 | (3 Bb4+F5:1 G5 F5 (3 G5 F5+Bb5 _ (3 Bb4 G5 F5 \
             Eb5+Eb6:2 -:4 | (3 F5:1 G5 F5 (3 G5 F5 _ (3 F5 G5 F5 (3 G5 F5 _ -:2 C5+C6 | D5:4 (3 \
             Bb4+F5:1 G5 F5 -:2 C5+C6 Bb4+Bb5 | (3 Bb4:1 - - (3 - Bb5 _ (3 Bb4 - - (3 - Bb5 _ \
             -:4 | (3 Bb4+F5:1 - F5 (3 - F5+Bb5 _ (3 Bb4+F5 - Bb4+F5 (3 F5 F5+Bb5 _ -:2 F5+F6 | \
             G5+G6:4 G5+G6:6 F5+F6:2 | (3 Bb4+F5:1 G5 F5 (3 G5 F5+Bb5 _ (3 Bb4+F5 G5 Bb4+F5 (3 \
             F5+G5 F5+Bb5 _ G6:4 | F5+F6 F5+F6:6 Eb5+Eb6:2 | D5+D6:4 (3 Bb4:1 - Bb4+F5 -:6 | (3 \
             F5:1 G5 F5 (3 G5 F5 _ (3 F5 G5 F5 (3 G5 F5+Bb5 _ -:2 C5+C6 & Bb4+Bb5:4 Bb4+Bb5:6 \
             -:2 | (3 Bb4+F5:1 G5 F5 (3 G5 F5+Bb5 _ Eb5+Eb6:2 (3 G5:1 F5+Bb5 _ -:4 | (3 F6:1 _ - \
             -:4 (3 G5:1 F5+Bb5 _ D5+D6:2 C5+C6 | Bb4+Bb5:4 Bb4+Bb5:6 -:2 | (3 Bb4+F5:1 G5 F5 (3 \
             G5 F5+Bb5 _ Eb5+Eb6:2 (3 G5:1 F5+Bb5 _ -:4 | F5+F6:8 F5+F6:4 | (3 G6:1 _ - -:4 (3 \
             -:1 - F6 Bb6:2 (3 G6:1 - Bb5. | (3 Bb4+F5 - F5 (3 - F5+Bb5 _ (3 Bb4+F5 - Bb4+F5 (3 \
             F5 F5+Bb5 _ -:2 (3 -:1 - Bb5. | (3 _ _ - D5+D6:2 C5:4 - | (3 Bb4:1 G5 - (3 G5 - - \
             (3 Bb4 - Bb4 -:6 | (3 F5:1 G5 F5 -:2 C5+C6 (3 -:1 - F6 F6+Bb6:2 (3 G6:1 F6 - | -:12 \
             & -:12 | - | - | - | - | - | - | -:6 (3 -:1 - F6 -:2 (3 G6:1 - - | -:6 (3 -:1 - F6 \
             Bb6:2 (3 -:1 F6 Bb5. | (3 _ _ - (3 - Bb5 _ (3 - G5 - (3 G5 Bb5 _ Bb6:2 (3 G6:1 F6 \
             Bb5. | (3 _ _ - (3 - Bb5 _ (3 - G5 - (3 G5 Bb5 _ -:2 (3 C6:1 _ Bb5 | Bb4:4 Bb4:6 \
             -:2 & (3 -:1 F5+D6 D6+F6 (3 F5 D6+F6. _ (3 _ F5+D6 D6+F6 (3 F5 D6+F6. _ _ -:3 | (3 \
             -:1 F5+D6 D6+F6 (3 F5 D6+F6. _ (3 _ F5+D6 D6+F6 (3 F5 D6+F6. _ _ -:3 | (3 -:1 F5+E6 \
             E6+F6 (3 F5 E6+F6. _ (3 _ F5+E6 E6+F6 (3 F5 E6+F6. _ _ -:3 | (3 -:1 F5+D6 D6+F6 (3 \
             F5 D6+F6. _ (3 _ F5+D6 D6+F6 (3 F5 D6+F6. _ _ -:3 | (3 -:1 F5+D6 D6+F6 (3 F5 D6+F6. \
             _ (3 _ F5+D6 D6+F6 (3 F5 D6+F6. _ _ -:3 | (3 -:1 G5+Eb6 Eb6+G6 (3 G5 G6. _ (3 _ \
             G5+Eb6 Eb6+G6 (3 G5 G6. _ _ -:3 | (3 -:1 G5+D6 D6+G6 (3 G5 D6+G6. _ (3 _ G5+D6 \
             D6+G6 (3 G5 D6+G6. _ _ -:3 | (3 -:1 Bb5+D6 D6+Bb6 (3 Bb5 D6+Bb6. _ (3 _ Bb5+D6 \
             D6+Bb6 (3 Bb5 D6+Bb6. _ _ -:3 | (3 -:1 Eb5+C6+Eb6 C6+C7 (3 C6+Eb6 C6+Eb6+C7. _ (3 _ \
             Eb5+C6+Eb6 C6+C7 (3 C6+Eb6 C6+Eb6+C7. _ _ -:3 | (3 -:1 Eb5+C6+Eb6 C6+C7 (3 C6+Eb6 \
             C6+Eb6+C7. _ (3 _ Eb5+C6+Eb6 C6+C7 (3 C6+Eb6 C6+Eb6+C7. _ _ -:3 | -:2 \
             G4+F5+Eb6+A6:1 F5+Eb6+A6 C4+Eb4+Eb6+A6 - F5+Eb6+A6:2 (3 -:1 F5+D6 G5+Eb6 (3 A5+F6 \
             Bb5+G6 C6+A6 | - Bb5+D6+Bb6 F4+Bb5+D6+Bb6 Bb5+D6+Bb6 - Bb5+D6+Bb6 -:6",
        )
        .in_key(ASharp, Major)
        .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bars 147 to 154.
        Part::new(
            "Wooden leg again",
            "-:12 | F6:3 Eb6:1 D6:2 D6 D6 D6 | D6:4 Bb5:2 C6 D6 C6 | Bb5 D6 C6 - Eb6:3 D6:1 | \
             C6:2 C6 C6 C6 C6:4 | C6:2 C6 F6 Eb6 D6 C6 | D6:1 -:3 F6:8 | F6:3 Eb6:1 D6:2 D6 D6 \
             D6",
            "",
        )
        .with_left(
            "-:12 | - | Bb2:2 Bb3+D4 Bb3+D4+D5 Bb3+D4+Eb5 Bb2+F5 Bb3+D4+Eb5 | Bb3+D4+D5 \
             Bb3+D4+F5 A2+Eb5 A3+Eb4 A3+Eb4+G5 A3+Eb4 | A2+Eb5 A3+Eb4+A5 A3+Eb4+A5 A3+Eb4+A5 A2 \
             A3+Eb4 | A3+Eb4+A5 A3+Eb4+A5 A2+A5 A3+Eb4+G5 A3+Eb4+F5 A3+Eb4+Eb5 | -:12 | -:4 \
             Bb2+F5:2 Bb3+D4+Bb5 Bb3+D4+Bb5 Bb3+D4+Bb5 & -:12 | -:10 E4:1 E4 | D4+F4 D4+F4 F4:2 \
             Eb4+F4:1 Eb4+F#4 Eb4+F#4:3 Eb4+F#4:1 Eb4+F#4:2 | F4:1 F4 F4:2 F4+G4:1 F4+G4 F4+G4:3 \
             F4+G4:1 F4+G4:2 | _:1 F4+G4 F4+G4:3 F4+G4:1 F4+G4:3 F4+G4:1 F4+G4 F4+G4 | _ F4+G4 \
             F4+G4:3 F4+G4:1 F4+G4:3 F4+G4:1 F4+G4:2 | _:1 F4+G4 F4+G4:2 F4+G#4:1 F4+G#4 \
             F4+G#4:2 F#4+A4+C6:1 F#4+A4+F6 F#4+A4:2 | _:1 A3+F#4+A4 A3+F#4+A4 Bb3+G4+Bb4 G#4 \
             Bb3+G#4 G#4+Bb4:2 -:1 C5:2 G#4+Bb4+C5:1 & -:12 | A5:3 G5:1 Bb2+F5:2 Bb3+D4+Bb5 \
             Bb3+D4+Bb5 Bb3+D4+Bb5 | Bb5:4 -:8 | -:11 F5:1 | -:8 A5:4 | -:12 | D5+F5:1 -:3 D6:4 \
             D6 | A5:3 G5:1 -:8 & -:12 | (3 Bb4:1 G5 - (3 G5 Bb5 _ (3 - G5 - Eb5+Eb6:2 - C5+C6 | \
             (3 F5:1 G5 F5 (3 G5 F5 _ Bb4+Bb5:6 C5:2 | (3 Bb4+F5:1 - - -:2 (3 Bb4+F5:1 - - (3 G5 \
             F5+Bb5 _ C5+C6:2 Bb4+Bb5 | (3 Bb4:1 - - -:2 (3 Bb4:1 - - -:6 | (3 Bb4+F5:1 - F5 -:2 \
             (3 Bb4+F5:1 - Bb4+F5 (3 - - F6 Bb6:2 - | (3 Bb4:1 - - -:2 (3 Bb4+F5:1 - Bb4+F5 (3 - \
             - F6 Bb6:2 (3 G6:1 - - | (3 Bb4+F5 G5 F5 (3 G5 F5+Bb5 _ C5+C6:4 G5+G6 & (3 Bb4+F5:1 \
             G5 F5 (3 G5 F5+Bb5 _ (3 Bb4+F5 G5 F5 (3 G5 F5+Bb5 _ C5+C6:2 Bb4+Bb5 | F5:4 (3 Bb4:1 \
             - - (3 G5 F5 _ D5:2 - | Bb4:4 (3 F5:1 G5 F5 (3 G5 F5 _ -:4 | (3 -:1 G5 F5 (3 G5 Bb5 \
             _ (3 - G5 F5 D5+D6:2 -:4 | F5+F6:8 F5+F6:4 | G5+G6 G6:6 (3 G6:1 - Bb5. | (3 _ _ F5 \
             (3 - F5+Bb5 _ -:2 (3 F5:1 F5+Bb5 _ -:2 (3 -:1 - Bb5. | (3 _ _ - -:2 (3 -:1 G5 - (3 \
             F5+G5 F5 _ -:4 & -:12 | - | - | - | - | - | - | C6:1 F6:6 Eb5:1 G#4+Bb4+D5+C6 F6 \
             G#4 - & D5+D6:4 Eb5+Eb6:2 D5+D6 -:4 | F6:6 - | -:12 | D6:4 Eb6:2 -:6 | (3 -:1 G5 - \
             (3 G5 Bb5 _ (3 - G5 - (3 G5 Bb5 _ -:4 | -:2 (3 -:1 F5+Bb5 _ G5:2 (3 F5:1 F5+Bb5 _ \
             -:2 F5+F6 | G5+G6:4 G5+G6:6 F5+F6:2 | Eb5+Eb6 D5+D6 (3 Bb4:1 - Bb4 (3 - - F6 Bb6:2 \
             (3 -:1 F6 -",
        )
        .in_key(ASharp, Major)
        .played_by(&[(gm::FLUTE, 0), (gm::CLARINET, -12)])
        .with_drums(&[
            (Drum::Ride, "x:4 x x | X x x | x x x | x x x | x x x | X x x | x x x | X x x"),
        ]),
        // MIDI bar 155.
        Part::new(
            "Fair returns",
            "(3 C7:2 D7 F7 (3 Eb7 D7 C7 Bb6:1 Bb6 D7 D7 C7:3 -:1",
            "",
        )
        .with_left(
            "-:15 Bb2:1 & Bb2:2 Bb3+D4 -:11 A1+Bb3:1 & D4:2 -:4 C4:2 D4 C4 - D4:1 - & \
             Bb3+G#4+D5:1 Bb3+G#4+Bb4+D5 Bb4+C5+D5 C5 C6 Bb4 Bb4+D5 Bb4+D5 Bb3+Bb4+D5+C6 \
             Bb3+Bb4+D5 C5 C5 C5+E5 C5+E5 D5 A2 & Bb5+D6:4 (3 Bb3+D4+D5+Bb5+Eb6:1 _ - \
             Bb3+D4+Eb5+C6:2 Bb2+F5+Bb5+D6 Bb3+D4+Eb5+C6+D6 Bb3+D4+D5+Bb5+C6 Bb3+F5+D6:1 - & \
             F5:4 F5 - (3 Bb4+F5:1 - - -:2 & (3 Bb4:1 G5 - (3 G5 - - (3 Bb4 G5 - (3 G5 Bb5 _ -:4 \
             (3 -:1 G5 F5 (3 G5 Bb5 _ & (3 C6:1 _ - (3 - F6 _ (3 - - D6 (3 _ - - -:8 & F6:2 - \
             F6:6 Eb6:2 D6 -",
        )
        .in_time(COMMON)
        .in_key(ASharp, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "x:4 x x x"),
        ]),
        // MIDI bar 156.
        Part::new(
            "Revellers 4",
            "-:2 E5 E5",
            "",
        )
        .with_left(
            "E3:2 -:4 & E1+E2:2 D4+E4+D5 D4+E4+D5",
        )
        .in_time(THREE_EIGHT)
        .with_tempo(&[tempo(0, 140)])
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 157.
        Part::new(
            "Revellers 4, bar 2",
            "-:2 E5 E5:3 -:1",
            "",
        )
        .with_left(
            "-:6 (3 A1+A2:1 G1+G2 F1+F2 & -:6 Bb2:2 & -:6 Bb3:1 A3 & -:2 D4+E4+D5. -:4 & -:4 \
             D4+D5:2 (3 _:1 _ -",
        )
        .in_time(TWO_FOUR)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 158.
        Part::new(
            "Revellers 4, bar 3",
            "-:2 E5 E5:4 E5:1 -",
            "",
        )
        .with_left(
            "E1+E2:8 -:2 & -:8 Bb2:2 & -:8 (3 A1+A2+D4+E4+D5:1 G1+G2 F1+F2 & E3:2 -:6 Bb3:1 A3 & \
             -:2 D4+E4+D5 D4+E4+D5 -:4",
        )
        .in_time(FIVE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 159.
        Part::new(
            "Revellers 4, bar 4",
            "-:2 E5 E5",
            "",
        )
        .with_left(
            "E3:2 -:4 & E1+E2:2 D4+E4+D5 D4+E4+D5",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 160.
        Part::new(
            "Revellers 4, bar 5",
            "-:2 E5 E5 A5 G5 F#5:1 -",
            "",
        )
        .with_left(
            "-:2 D4+E4+D5 D4+E4+D5 -:6 & -:6 A4+E5:2 D4+G4+E5 F#4+E5:1 - & -:8 D5:2 D4:1 -",
        )
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 161.
        Part::new(
            "Revellers 4, bar 6",
            "A5:2 G5 F#5 A5 G5:1 -",
            "",
        )
        .with_left(
            "-:8 Bb2:2 & D4+E5:2 D4+E5 D4+E5 D4 (3 A1+A2+D4+E5:1 G1+G2 F1+F2 & E4+A4:2 G4+D5 \
             F#4+D5 A4+E5 Bb3+E4:1 A3",
        )
        .in_time(FIVE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 164.
        Part::new(
            "Revellers 4, bar 9",
            "A5:2 G5 F#5 A5 G5:1 -",
            "",
        )
        .with_left(
            "E4+A4:2 G4+D5 F#4+D5 A4+E5 E4:1 - & D4+E5:2 D4+E5 D4+E5 D4 D4+E5:1 -",
        )
        .in_time(FIVE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 165 to 168.
        Part::new(
            "Fair, flute",
            "A5:2 D6 A5:1 D6:7 | A5:1 D6:7 A5:1 D6:3 | (3 _:2 E6 A6 (3 G6 E6 D6 (3 A5:1 A4 D5 (3 \
             _ E5 A5 | Bb5 C6 Bb5 C6 Bb5 C6 Bb5 C6 Bb5 C6 Bb5 C6",
            "",
        )
        .with_left(
            "D4+A4:2 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 E4+G4 D4+A4 E4+G4 D4+A4 E4+G4 | D4+A4 \
             E4+G4 D4+A4 E4+G4 A4 E4 | D4+A4 E4+G4+C5 D4+A4+Bb4 E4+G4+C5 D4+A4+Bb4 E4+G4+C5 & \
             -:12 | - | -:8 (3 -:1 E4 - (3 A4 E5 - | -:8 (3 Bb5:1 - G4 (3 F4+F5 E5 D4+D5 & \
             D5+A5:1 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 \
             E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 | D5+A5 E5+G5 \
             D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D4+D5+A5 E4+G4+E5+G5 G4+C5+D5+A5 A4+D5+E5+G5 | \
             Bb4+D5+A5+Bb5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5 D5+A5 E5+G5",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bars 169 to 170.
        Part::new(
            "Piccolo, return",
            "-:2 G6:1 G6:4 G6:2 G6:3 | G6:2 G6:1 G6:4 G6:2 G6:1 C7 -",
            "",
        )
        .with_left(
            "Bb3+D4+A4:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | Bb3+D4+A4 C4+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 | \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)]),
        // MIDI bar 171.
        Part::new(
            "Piccolo, return, bar 3",
            "Bb6:1 C7 A6 -:2 F6:1 A6 -",
            "",
        )
        .with_left(
            "C4:2 Bb3 C4 Bb3 & (3 Bb3+D4+A4:1 _ A4 (3 _ - - (3 - F4 - (3 - A4 - & -:2 C4+E4+G4 \
             Bb3+D4+A4 (3 C4+E4+G4:1 _ - & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)]),
        // MIDI bar 172.
        Part::new(
            "Piccolo, return, bar 4",
            "-:2 G6:1 G6:4 G6:2 G6:1 C7 -",
            "",
        )
        .with_left(
            "Bb3+D4+A4:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 & Bb4+D5+A5:1 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 \
             Bb4+D5+A5 C5+E5+G5",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)]),
        // MIDI bars 173 to 174.
        Part::new(
            "Piccolo, return, bar 5",
            "Bb6:1 C7 A6 -:2 F6:1 C7:2 | Bb6:1 C7 A6 G6:2 F6:1 C7 -",
            "",
        )
        .with_left(
            "-:2 C4+E4+G4 Bb3+D4+A4 C4+E4+G4 | (3 Bb3+D4+A4:1 _ - C4+E4+G4:2 Bb3+D4+A4 (3 \
             C4+E4+G4:1 _ - & (3 Bb3+D4+A4:1 _ A4 (3 _ - - (3 - F4 - (3 - C5 - | (3 - - A4 (3 _ \
             _ - (3 - F4 - (3 - C5 - & Bb4+D5+A5:1 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5 | Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 C5+E5+G5 Bb4+D5+A5 \
             C5+E5+G5 Bb4+D5+A5 C5+E5+G5",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 175.
        Part::new(
            "Crowd 2",
            "(3 Bb6:1 C7 Bb6 (3 A6 G6 _ (3 F6 G5 A5 (3 Bb5 C6 E6 F6 -:3",
            "",
        )
        .with_left(
            "(3 Bb3+D4+A4:1 _ A5 C4+E4+G4:2 Bb3+D4+A4 C4+E4+G4 -:4 & (3 -:1 - A4 (3 _ G5 - (3 - \
             F4 - (3 - A4 _ C4+E4+G4+Bb4+D5:4 & -:4 (3 -:1 C4+G4 - (3 - C5 Bb4+G5 A4+F5+A5 -:3 & \
             (3 -:1 C5 Bb4 (3 - E5 - (3 F4+C5+E5+G5+C6 E4 F4 (3 B5 - - -:4 & -:4 (3 -:1 C5 D5 (3 \
             E5 F4+F5 - -:4 & (3 Bb4+D5+A5+Bb5:1 C6 C5+E5+G5+Bb5 Bb4+D5+A5+A6 C5+E5+G5+G6 \
             Bb4+D5+F5+F6 C5+E5 - C5+E5 -:4 & -:4 A5:2. Bb4+D5+A5+A6:1 - G5+Bb5+D6+G7:4",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "-:8 X:4"),
            (Drum::Crash, "-:8 X:4"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 176 to 192.
        Part::new(
            "Crowd tune 2",
            "G7:4 G7 | G7 G7:2 G7 | G7:4 G7 | G7 G7 | G7 G7 | G7 C8 | Bb7 A7:2 G7 | F7:4 A7 | G7 \
             G7 | G7 C8 | Bb7 A7:2 G7 | F7:4 A7 | G7 C8 | Bb7 A7:2 G7 | F7:4 A7 | G7 G7 | A7 \
             G7:2 F7",
            "",
        )
        .with_left(
            "-:8 | - | - | - | - | - | - | - | - | - | - | - | F4+Bb4:2 -:6 | -:8 | -:4 A3+E4+E5 \
             | G3+Bb4+D5 G3 | -:8 & -:8 | - | - | - | - | - | -:4 G3+C4+E4+G4+C5 | - \
             E3+A3+C4+C5:2 F3+Bb3+D4+Bb4+D5 | C4+D4+G4+E5:4 C4+F4+G4+Bb4+E5 | F3+G4+Bb4+D5+E5 \
             E3+C5+A5 | - G3+C4+E4+G4+C5 | - E3+A4+C5:2 F3+Bb4+D5 | G3+C4+E4+G4 F3+Bb3+D4+F4+Bb4 \
             -:4 | - G3+C4+E4+G4+C5 | - E3+A4+C5:2 F3+Bb4+D5 | C4+E4+G4+C5+E5:4 Bb3+G4+D5+A5 | \
             F3+A4+C5+E5 G3+Bb3+D4+G4+Bb4 & -:4 Bb4 | -:8 | -:4 Bb4 | G3 A3 | - Bb4 | -:8 | -:4 \
             A3+E5:2 - | C5:4 E5 | G3+Bb4+D5 G3+D5 | G3 - | - E5:2 - | C5:4 A3+E4+E5 | D5 G5 | - \
             E5:2 - | C5:4 - | -:8 | -:6 F4:2 & Bb3+G4+D5+F5:4 A3+E4+G4+C5+D5 | G3+Bb4+D5 \
             C4+E4+G4+Bb4+D5:2 C4+E4+G4+Bb4+D5 | Bb3+G4+D5+F5:4 A3+E4+G4+C5+D5 | Bb3+F4+G4+D5+A5 \
             C4+G4+Bb4+D5+A5 | Bb3+G4+D5+F5+A5 A3+G4+C5+D5+A5 | G3+Bb4+D5+A5 E3+C5+A5 | \
             F3+Bb4+C5+D5+G5 A4+F5:2 Bb4+D5+E5 | A3+D5+F5:4 E4+A4+F5 | -:8 | - | \
             F3+Bb4+C5+D5+G5:4 A4+F5:2 Bb4+D5+E5 | A3+D5+F5:4 F5 | C5+E5 E3+C5+A5 | \
             F3+Bb4+C5+D5+G5 A4+F5:2 Bb4+D5+E5 | A3+D5+F5:4 F5 | -:8 | -:4 D5:2 A4+C5 & Bb5+D6:4 \
             Bb5+D6 | Bb5+D6 Bb5+D6:2 Bb5+D6 | Bb5+D6:4 Bb5+D6 | Bb5+D6 Bb5+D6 | Bb5+D6 Bb5+D6 | \
             G5+Bb5+D6+G6 G5+C6+E6+G6 | F5+Bb5+D6+F6 A5+C6+E6+A6:2 G5+Bb5+D6+G6 | A5+C6:4 C6+E6 \
             | Bb5+D6 Bb5+D6 | G5+Bb5+D6+G6 G5+C6+E6+G6 | F5+A5+Bb5+D6+F6 A5+C6+E6+A6:2 \
             G5+Bb5+D6+G6 | A5+C6:4 C6+E6 | Bb5+D6+G6 C6+E6+G6 | F5+Bb5+D6+F6 A5+C6+E6+A6:2 \
             G5+Bb5+D6+G6 | A5+C6:4 C6+E6 | Bb5+D6 Bb5+D6 | C6+E6 Bb5+D6:2 A5+C6 & G5+G6:4 G5+G6 \
             | G5+G6 G5+G6:2 G5+G6 | G5+G6:4 G5+G6 | G5+G6 G5+G6 | G5+G6 G5+G6 | - C7 | Bb6 - | \
             F6 A5+A6 | G5+G6 G5+G6 | - C7 | Bb6 - | F6 A5+A6 | G5 C7 | Bb6 - | F6 A5+A6 | G5+G6 \
             G5+G6 | A5+G6+G7 G5+F6+F7:2 F5+E6+E7",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "X:4 X | X x:2 x | X:4 X | X X | X X | X x | X x:2 x | X:4 X | X X | X \
                          X | X x:2 x | X:4 X | X x | X X:2 x | X:4 x | X X | X X:2 x"),
            (Drum::Crash, "X:4 x | X x:2 x | X:4 x | X x | X X | X x | X X | X X:2 x | X:4 x | X \
                           X | X x | X X:2 x | X:4 X:2 x | X:4 X | X X:2 X | X:4 x | X x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 193.
        Part::new(
            "Crowd 2, 3/4",
            "G7:4 F7 G7",
            "",
        )
        .with_left(
            "-:8 E4:4 & G3+C5+D5+E5+G5:4 A3+C5+E5+F5 E3+F4+C5+A5 & Bb5+D6+G6:4 A5+C6+F6 C6+E6+A6 \
             & F6+F7:4 E6+E7 G6",
        )
        .in_key(G, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "X:4 x x"),
            (Drum::Crash, "X:4 X x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 194 to 195.
        Part::new(
            "Crowd 2, end",
            "G7:2 F7 G7:4 | G7:1 -:7",
            "",
        )
        .with_left(
            "G4:2 F4 Bb3+D4+Bb4:4 | G3 - & A3+Bb3+D4+A4+Bb4:4 - | Bb3+D4+D5:1 -:3 Bb3+G4:4 & \
             D5:2 C5+E5 G3+C5+D5+E5:4 | G4+Bb4:2. -:6 & -:8 | G5+Bb5:1 -:3 Bb4+G5:4 & Bb5+D6:2 \
             A5+C6 Bb5+D6:4 | D6:2. -:6 & G5+F6+F7:2 F5+E6+E7 G5+F6+F7:4 | F6+F7:1 -:7",
        )
        .in_time(TWO_FOUR)
        .in_key(G, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "X:2 x x:4 | X:8"),
            (Drum::Crash, "X:4 X | X:8"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 196 to 202.
        Part::new(
            "Merrymakers 2",
            "G5:4 G5 | G5 C6:2 C6 | C6:4 C6:2 C6 | C6:4 F6:2 F6 | Eb6:4 D6:2 C6 | Bb5:4 D6 | C6 \
             C6:2 F5:1 -",
            "",
        )
        .with_left(
            "-:8 | -:4 Eb4:2 - | -:4 Eb4:2 Eb4:1 - | -:8 | - | - | - & Bb3+G4:4 Bb3+G4 | Bb3+G4 \
             Bb3+G4+G5 | Bb3+G4+G5 C5+C6 | D4+Bb4+Bb5 C4:2 Bb3+G4+G5 | A3+F4+F5:4 C4+A4+A5 | \
             D4+G4+D5+G5 Bb3+G4+G5 | C4+A4+A5 Bb3+G4+G5:2 A3+F4+F5 & Bb4:4 Bb4 | Bb4 Eb5:2. \
             Eb4+Eb5 | Eb4+Bb4+Eb5:4 - | Eb4+D5+Eb5 - | G4+A4+G5 F4+F5:2 Eb4+Eb5:1 - | -:8 | -:4 \
             Bb4:2 A4:1 - & -:8 | -:4 Bb4 | - Eb5 | - A4+C5+A5:2 A4+Bb4+A5 | -:4 C5 | Bb4+F5 \
             Bb4+F5 | C5+Eb5 Eb5 & -:8 | -:4 C5+C6 | C5 - | C5 F5+F6 | Eb5 - | F4 F4 | Eb4 Eb4",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "-:8 | -:4 x | - x | - x | - x | - x | - x"),
            (Drum::Snare, "-:8 | -:6 x:2 | -:6 x:2 | -:6 x:2 | -:6 x:2 | -:6 x:2 | -:6 x:2"),
            (Drum::Crash, "-:8 | - | x | x | x | x | x"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 203.
        Part::new(
            "Merrymakers 2, 3/4",
            "D6:4 C6:2 Bb5 C6:4",
            "",
        )
        .with_left(
            "Bb3+G4:2 - A3+F4:4 C4+A4 & F4:4 Eb4:2 D4 Eb4:4 & Bb4+G5:2 - A4+F5:4 - & F5:4 Eb5:2 \
             D5 C5+Eb5+A5:4",
        )
        .in_key(C, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "-:4 x:8"),
            (Drum::Snare, "-:6 x"),
            (Drum::Crash, "x:8 x:4"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 204 to 207.
        Part::new(
            "Merrymakers 2, end",
            "Bb5:4 D6 | C6:1 -:7 | -:8 | -",
            "",
        )
        .with_left(
            "F4:4 - | C6 - | -:8 | -:6 (3 -:1 D2 C2 & Bb3+D5:4 G4 | -:8 | - | -:6 E2:1 - & \
             D4+G4:4 Bb3+F4 | C4 C4+E5+G5 | _:8 | _:6 -:2 & Bb4+F5+G5:4 Bb4+F5+G5 | \
             E4+Bb4+E5+G5:1 -:7 | -:8 | -:6 (3 E3:1 D3 C3",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Ride, "x:8 | - | - | -"),
            (Drum::Snare, "-:2 x:4 (3 _:1 x x | X:8 | - | -"),
            (Drum::Crash, "-:4 x | -:8 | - | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 223.
        Part::new(
            "Revellers 3",
            "-:2 E5 E5",
            "",
        )
        .with_left(
            "Bb1+Bb2:1 -:5 & E3:2 -:4 & E1+E2:2 D4+E4+D5 D4+E4+D5",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 225 to 230.
        Part::new(
            "Piccolo 3",
            "-:8 (3 E6:1 F6 G6 | A6:2 A6 D7 D7 A6 | A6 E6 E6 A6 A6 | A6 D7 D7 A6 A6 | D7 D7 A6 \
             A6 D7 | A6 E6 E6 A6 A6:1 -",
            "",
        )
        .with_left(
            "-:10 | D5:2 -:8 | -:4 A3:2. -:4 | -:10 | - | -:2 A3+C4+E4 A3+C4+E4:1 -:5 & D4:2 - \
             E4 - G4:1 - | D4+F4+A4:2 D4+F4+A4 G4+B4+D5 G4+B4+D5 D4+F4+A4 | D4+F#4+A4:1 - \
             A3+C4+E4:2 C4+E4 D4+F#4+A4 D4+F#4+A4 | D4+F4+A4 G4+B4+D5 G4+B4+D5 D4+F4+A4 D4+F4+A4 \
             | G4+B4+D5 G4+B4+D5 D4+F4+A4 D4+F4+A4 G4+B4+D5 | D4+F#4+A4:1 -:5 D4+F#4+A4:2 \
             D4+F#4+A4:1 - & E4+A4:2 G4+D5 F#4+D5 A4+E5 E4+G5. | D2+D3+F5+A5+D6+F6:1 -:9 | -:10 \
             | - | - | -:8 (3 E2+A5+D6+F#6:1 D2+D3 C2+C3 & E5+A5:2 D4+E5+G5 D4+E5+F#5 D4+A5 D4:1 \
             - | -:2 D5 D6 G5. - | - C5 C5:1 -:5 | D5+A5:2 G5+D6 G5:1 -:5 | G5+D6:2 D6 -:6 | -:8 \
             E3:2 & -:10 | - | - | - | -:2 G5. D5+A5 D5+A5. - | - C5 C5:1 -:5 & -:9 E6:1 | -:2 \
             F5+A5+D6+F6 G5+B5+G6+B6 B5+D6+G6+B6 G5+B5+D6+F6 | D5+F#5+A5+D6+F#6 A4+E5+A5+C6 \
             A4+E5+A5+C6 D5+F#5+A5+D6+F#6 D5+F#5+A5+D6+F#6:1 - | F5+D6+F6:2 B5+G6+B6 B5+G6+B6 \
             F5+D6+F6 F5+D6+F6 | B5+G6+B6 B5+G6+B6 F5+D6+F6 F5+D6+F6 B5+G6+B6 | D5+F#5+A5+D6+F#6 \
             A4+E5+A5+C6 A4+E5+A5+C6 D5+F#5+A5+D6+F#6 -",
        )
        .in_time(FIVE_EIGHT)
        .in_key(D, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "-:8 (3 x:1 x x | x:10 | - | - | - | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 231.
        Part::new(
            "Clarinets",
            "-:2 E5 E5",
            "",
        )
        .with_left(
            "Bb1:2 E4 E4 & Bb2:2 D5 D5 & -:2 D4 D4",
        )
        .in_time(THREE_EIGHT)
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::CLARINET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:6"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 232.
        Part::new(
            "Clarinets, bar 2",
            "-:2 E5 E5 E5 E5 E5:1 -",
            "",
        )
        .with_left(
            "-:2 D4:1 D4 D4 D4 D4 D4 D4 D4 D4 D4 & -:2 E4 E4 E4 E4 E4+E5 & -:6 A4+A5:1 A4+A5 \
             G4+G5 G4+G5 F#4+F#5 F#4+F#5 & -:2 D5 D5 D5 D5 D5:1 -",
        )
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 233.
        Part::new(
            "Clarinets, bar 3",
            "E5:2 E5 E5 E5 E5:1 -",
            "",
        )
        .with_left(
            "-:8 (3 -:1 D2 C2 & E4:2 -:6 E2:1 - & D4:1 D4 D4 D4 D4 D4 D4 D4 D4 - & D5:2 D5 D5 D5 \
             (3 E3+D5:1 D3 C3 & A4+A5:1 A4+A5 G5 G5 F#4+F#5 F#4+E5+F#5 A4+A5 A4+E5+A5 (3 E4+G5 _ \
             -",
        )
        .in_time(FIVE_EIGHT)
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 234.
        Part::new(
            "Clarinets, bar 4",
            "-:2 E5 E5",
            "",
        )
        .with_left(
            "Bb1:2 E4 E4 & Bb2:2 D5 D5 & -:2 D4 D4",
        )
        .in_time(THREE_EIGHT)
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 235.
        Part::new(
            "Flutes",
            "-:4 D6:2 A6 G6 F#6:1 -",
            "",
        )
        .with_left(
            "-:2 D4+E4:1 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 & -:6 A4:1 A4+D5 \
             G4+G5 G4+D5+G5 F#4+F#5 F#4+E5+F#5 & -:2 D5+E5 D5+E5+E6 D5+E5+A5:1 - D5+E5+D6+E6 - \
             D5+E5 - & -:5 D6:1 - A5+A6 - D6+G6 - F#6",
        )
        .in_key(E, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 236.
        Part::new(
            "Flutes, bar 2",
            "A6:2 G6 F#6 A6 G6:1 -",
            "",
        )
        .with_left(
            "-:8 E2:1 - & -:8 (3 -:1 D2 C2 & D4+E4:1 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 D4+E4 \
             D4+E4 - & A4:1 A4 G5 D5+G5 F#4+F#5 F#4+F#5 A4 A4+E5 (3 G5 _ - & D5+E5+A5+E6:1 A5+A6 \
             D5+E5+D6 D6+G6 D5+E5+E6:2 D5+E5+A5:1 - (3 E3+D5+E5+D6+E6 D3 C3 & -:5 F#6:1 - A5+A6 \
             -:2",
        )
        .in_time(FIVE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 237.
        Part::new(
            "Flutes, bar 3",
            "-:6",
            "",
        )
        .with_left(
            "-:2 E4 E4:1 - & Bb1:2 D4+E5 D4+D5+E5 & Bb2:2 D5 D6:1 - & -:2 D6:1 D6 E6 E6",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 238.
        Part::new(
            "Flutes, bar 4",
            "-:2 D6 E6 A6 G6:1 -",
            "",
        )
        .with_left(
            "D4+E4+E5:1 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 \
             D4+E4+E5 & A6:1 A6 G5+G6 G5+G6 F#5 F#5 A4 A4 G4 G4+D5+D6 & A5:2. D5+E6 D5+D6+F#6 \
             D5+C6+D6+E6 D5+D6+E6:1 - & D5+D6+E6:2 -:1 D6 - E6 - A6 - G6",
        )
        .in_time(FIVE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 239.
        Part::new(
            "Flutes, bar 5",
            "F#6:2 E6 D6 E6:1 -",
            "",
        )
        .with_left(
            "D4+E4+E5:1 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 D4+E4+E5 & F#4:1 \
             F#4 A6 A6 G5+G6 G5+G6 F#5 D5+F#5+F#6 & -:1 F#6 A5:2. D5+E6 D5+D6+F#6:1 - & \
             D5+D6+E6:2 D5+D6 -:4 & -:3 E6:1 - D6 - E6",
        )
        .in_time(TWO_FOUR)
        .in_key(E, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 240.
        Part::new(
            "Flutes, bar 6",
            "A6:2 G6 F#6:1 -",
            "",
        )
        .with_left(
            "-:4 (3 -:1 F4 D4+E4+G4 & A4:1 A4 G4 G4 F#4 F#4+D5+E5 & D4+E4+E5:1 D4+E4+E5 D4+E4+E5 \
             D4+E4+E5 (3 D4+E4 _ - & D5+A5+D6:2 D5+D6+E6 (3 D5+E5+D6+E6+F#6:1 F5+F6 - & E6:2. - \
             (3 -:1 - G5+F#6+G6 & -:1 A6 - G6 -:2 & -:4 (3 C6:1 D6 E6",
        )
        .in_time(THREE_EIGHT)
        .in_key(E, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "-:4 (3 x:1 x x"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 241 to 248.
        Part::new(
            "Piccolo 4",
            "A6:2 D7 D7 A6 A6 | D7 D7 A6 A6 D7 | D7 A6 A6 D7 D7 | A6 A6 D7 D7 A6 | A6 E6 E6 A6 \
             A6 | A6 D7 D7 A6 A6 | D7 D7 A6 A6 D7 | A6 E6 E6 A6 A6:1 -",
            "",
        )
        .with_left(
            "E4+E5:2 -:8 | -:10 | - | - | A2:2 D3 A2 D3 A2 | D3 -:8 | -:10 | A2:2 D3 A2 D3 A2 & \
             D4+F4:2. - G4. D4+F4+A4 D4+F4+A4 | G4 G4+B4 D4+F4 D4+F4 - | G4 D4+F4 D4+F4 -:4 | \
             D4+F4:2 D4+F4 - G4 D4+F4 | A3+D4+F#4+A4 A3+C4+E4 A3+E4. D4+F#4+A4 D4+F#4 | D4+F4 \
             G4+B4 B4+D5. -:4 | D5:2. -:6 G4+B4:2 | -:6 D4+F#4+A4:2 D4+F#4:1 - & A4:2 G4+B4+D5 \
             B4+D5 -:4 | B4+D5:2. -:6 G4+B4:2 | B4+D5. -:4 G4+B4:2 G4+B4 | -:4 G4+B4:2 B4+D5. - \
             | -:4 C4:2 -:4 | C4:2. - G4 D4+F4 D4+F4+A4 | G4+B4 G4+B4 D4+F4 D4+F4 - | \
             A3+D4+F#4+A4 A3+C4+E4 A3+C4+E4:1 -:5 & A3:2 -:8 | D4+E5+D6:2 -:8 | D4+E5+D6:2 -:8 | \
             A4+E5:2 -:8 | -:10 | A4+E5:2 -:8 | D4+E5+D6:2 -:8 | -:10 & D2+D3+D5+F5+A5+D6+F6:1 - \
             G5+B5+G6+B6:2 G5+B5+G6+B6 D5+F5+A5+D6+F6 D5+F5+A5+D6+F6 | G5+B5+G6+B6 G5+B5+G6+B6 \
             D5+F5+A5+D6+F6 D5+F5+A5+D6+F6 G5+B5+G6+B6 | G5+B5+G6+B6 D5+F5+A5+D6+F6 \
             D5+F5+A5+D6+F6 G5+B5+G6+B6 G5+B5+G6+B6 | D5+F5+A5+D6+F6 D5+F5+A5+D6+F6 G5+B5+G6+B6 \
             G5+B5+G6+B6 D5+F5+G5+A5+D6+F6:1 - | D5+F#5+A5+D6+F#6:2 A4+C5+E5+A5+C6 \
             A4+C5+E5+A5+C6 D5+F#5+A5+D6+F#6 D5+F#5+A5+D6+F#6:1 - | D5+F5+A5+D6+F6:2 G5+B5+G6+B6 \
             G5+B5+G6+B6 D5+F5+A5+D6+F6 D5+F5+A5+D6+F6 | G5+B5+G6+B6 G5+B5+G6+B6 D5+F5+A5+D6+F6 \
             D5+F5+A5+D6+F6 G5+B5+G6+B6 | D5+F#5+A5+D6+F#6 A4+C5+E5+A5+C6 A4+C5+E5+A5+C6 \
             D5+F#5+A5+D6+F#6 D5+F#5+A5+D6+F#6:1 -",
        )
        .in_time(FIVE_EIGHT)
        .in_key(D, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "x:10 | - | - | - | - | - | - | -"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 249 to 258.
        Part::new(
            "Climax",
            "A6:2 D7 A6:1 D7:7 | A6:1 D7:3 (3 _:2 E7 A7 (3 G7 E7 D7 | A6:4 A6:2 D7 A6:1 D7:3 | \
             (3 _:2 E7 A7 (3 G7 E7 D7 A6:4 | (3 A5:2 D7 E7 (3 A7 G7 E7 (3 D7 A6 D7 | (3 E7 A7 G7 \
             (3 E7 D7 A6 (3 D7 E7 A7 | (3 G7 E7 D7 A6:6 -:2 | (3 Bb5:1 C6 Bb5 (3 _ C6 Bb5 (3 C6 \
             Bb5 C6 (3 Bb5 C6 Bb5 C6 Bb5 (3 C6 Bb5 C6 | -:4 (3 C5:1 C#5 Eb5 (3 F5 G5 Bb5 (3 C6 \
             C#6 C#6 (3 C6 C#6 C#6 | (3 _ E5 F#5 (3 G#5 Bb5 C#6 (3 Eb6 E6 E6 (3 Eb6 E6 E6 (3 Eb6 \
             G5 A5 (3 B5 C#6 E6",
            "",
        )
        .with_left(
            "-:12 | - | - | - | - | - | -:10 Bb1:2. | C2:4 F2:2 E2 (3 F2 E2 D2 | C#2:4 Eb2 (3 \
             G#2:1 _ - (3 - F2+C6 - | E2:2 -:10 & D3:2 -:10 | -:12 | - | - | - | - | - | - | - | \
             -:4 F#3:1 F#3 F#3 F#3 F#3 F#3 F#3 F#3 & F4+A4:2 E4+G4:1. - A4:2 E4+G4:1. - A4:2 \
             E4+G4:1. - | A4:2 E4+G4:1. - A4:2 E4+G4:1. - A4:2 E4+G4:1. - | A4:2 E4+G4:1. - A4:2 \
             E4+G4:1. - A4:2 E4+G4:1. - | A4:2 E4+G4:1. - A4:2 E4+G4:1. - A4:2 E4+G4:1. - | A4:2 \
             E4+G4+E5:1 - A4+G5 - E4+G4. - (3 A4 _ G5 (3 E4+G4. F5 _ | A4+E5:2 E4+G4:1. - A4:2 \
             E4+G4:1 - (3 A4 _ G4 (3 E4. F4 G4 | E4+A4 - E4+G4+G5 - (3 A4 _ G4 (3 E4. F4+F5 G4 \
             D4+E4+A4+E5:2 E4+G4. | D4+A4 E4+G4. D4+A4 E4+G4. D4+A4 E4+G4 | F3+C4+D4+A4 \
             Bb3+E4+G4+Bb4 C4+F4+C5 Bb3+G4+Bb4 C4+C#4+F4+C5 Bb3+Eb4+G4+Bb4:1 G4+Bb4 | \
             G#3+Eb4+G#4+Eb5:2 F#2+Bb3+C#4+Bb4+C#5 (3 B2+E4+G#4+Eb5:1 _ Bb2 (3 \
             G3+Bb3+F#4+Bb4+C#5 G#2 _ G2+B3+F#4+B4+F5:2 A2+C3+C#4+E4+C#5:1 C#5+E5 & D4+A5:1 \
             E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 (3 D4+A5 E6 E4+D6 (3 D4 _ E4+G5 | D4+A5 \
             E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 | (3 D4+A5 - \
             E4+D6 (3 D4 _ E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 | D4+A5 E4+G5 \
             D4+A5 E4+G5 (3 D4+A5 - E4+D6 (3 D4 _ E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 | D4 E4+G5 D4+A5 \
             E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 (3 D4+A5 - E4+D6 (3 D4 _ E4+G5 | D4+A5 E4+G5 D4+A5 \
             E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 | (3 D4+A5 - E4+D6 (3 D4 _ \
             E4+G5 D4+A5 E4+G5 D4+A5 E4+G5 -:2 Bb2+A5:1 - | C3:4 -:6 (3 -:1 D3 _ | C#3:4 Eb3 (3 \
             G#3:1 _ - (3 - F3+G#5 - | E3:2 -:6 G3:2 - & -:10 (3 -:1 A5 - | -:12 | (3 -:1 E6 - \
             (3 - A5 - -:8 | -:4 (3 -:1 E6 - (3 - A5 - -:4 | -:8 (3 -:1 E6 - (3 - A5 - | -:12 | \
             (3 -:1 E6 - (3 - A5 - -:4 A5:1 G5 - G5 | Bb5 C6 Bb5 G5 F3+Bb5 C6 E3+Bb5 G5 (3 \
             F3+Bb5 _ E3+C6 (3 Bb5 _ G5 | (3 C6 C#6 C6 C#6 C6 (3 C#6 C6 C#6 (3 C6 C#6 C6 (3 _ \
             Eb5 G3+F5 (3 G5 - C6 | (3 Eb6 E6 Eb6 F#3+E6 Eb6 (3 B3+E6 F#5 Bb3+G#5 (3 Bb5 G#3+B5 \
             Eb6 (3 E6 F#6 G6 A3+F#6 F#6 & D5:2 -:10 | -:4 (3 C5:1 E5 F5 (3 G5 - Bb5 (3 C6 D6 E6 \
             (3 D6 E6 D6 | (3 E6 D6 E6 (3 D6 E6 D6 E6 D6 (3 E6 D6 E6. _:2 - | (3 D5:1 E5 F5 (3 \
             G5 - Bb5 (3 C6 D6 E6 (3 D6 E6 D6 (3 E6 D6 E6 D6 E6 | (3 D6 E6 D6 (3 E6 D6 E6 (3 D6 \
             E6 D6 E6 D6 (3 E6 D6 E6 (3 D6 E6 D6 | (3 D5 - F5 (3 G5 - Bb5 (3 C6 D6 E6 (3 D6 E6 \
             D6 (3 E6 D6 E6 D6 E6 | D6 - (3 E6 D6 E6 (3 D6 E6 D6 E6 D6 (3 - Bb4 C5 (3 D5 E5 F5. \
             | _:2 -:10 | -:8 (3 -:1 - G2+C6 (3 C#6 - - | (3 C6 - - -:2 (3 -:1 - F#6 (3 E6 F#6 - \
             (3 F#6 - - -:2",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "x:12 | x | x | x | x | x | x | x | x:8 (3 x:1 x x (3 x x x | x:4 (3 \
                          x:1 x x (3 x x x x:4"),
            (Drum::Shaker, "-:4 (3 x:1 x x x x x:4 | - (3 x:1 x x x x X:4 | - (3 x:1 x x x x X:4 \
                            | - (3 x:1 x x x x X:4 | - (3 x:1 x x x x x:4 | - (3 x:1 x x x x X:4 \
                            | - (3 x:1 x x x x X:4 | - (3 x:1 x x x x X:4 | - (3 x:1 x x x x X:4 \
                            | (3 x:1 x x x x X:4 (3 x:1 x x x x"),
            (Drum::Crash, "-:8 x:4 | -:8 x:4 | -:8 x:4 | -:8 x:4 | -:8 x:4 | -:8 x:4 | -:8 x:4 | \
                           -:8 x:4 | x:12 | x:8 x:4"),
        ]),
        // MIDI bar 259.
        Part::new(
            "Climax, 5/8",
            "(3 A6:1 Bb6 A6 (3 Bb6 A6 A6 (3 Bb6 C7 Bb6 (3 C7 Bb6 Bb6 C7 -",
            "",
        )
        .with_left(
            "-:4 Bb3:3 - & F#3:1 F#3 F#3 F#3 F#3 F#3 F#3 F#3 -:2 & D3:2 C#3 C3:4 -:2 & -:2 \
             A3+E4+A4 D4+A4 E4+F5:1 -:3 & G3+F#4+G4+B4+F5:2 C#5:1 - A5 -:5 & D4+F#5:2 C#4+E5 \
             C4+Bb4:4 -:2 & -:6 (3 -:1 C5 - C5 - & (3 -:1 A4+A6 - (3 A6 D5 F#5 (3 G5+A6 - C5 \
             B5:2 - & (3 C#5+G5+E6+G6:1 D5+A5+D6+F#6 E5+G5+B5+C#6+G6 (3 F#5+C#6+F#6 F#5+B5+D6+G6 \
             C#5+G5+A5+F#6+G6 (3 B4+D5+E6+F#6+G6 E5+D6+A6+Bb6 F5+A5+C6+G6 (3 A4+Bb5+E6+A6 \
             G5+A5+Bb5+C6 G5+A5 A4+Bb5+A6 -",
        )
        .in_time(FIVE_EIGHT)
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "(3 x:1 x x (3 x x x x:6"),
            (Drum::Shaker, "(3 x:1 x x (3 x x x (3 X x x _:4"),
            (Drum::Crash, "-:4 x:6"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 260 to 269.
        Part::new(
            "Drum roll",
            "-:8 | - | - | - | - | -:7 G7:1 | F#7 -:7 | -:8 | - | -",
            "",
        )
        .with_left(
            "F#3:1 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 \
             F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 \
             F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3:2 F#2 -:4 | -:8 | - | - & -:8 | - | - | \
             - | - | -:7 G4:1 | -:2 Bb2 -:4 | -:8 | - | - & -:8 | - | - | - | - | -:7 G5:1 | \
             F#4+F#5+Bb6 -:7 | -:8 | - | -",
        )
        .in_time(TWO_FOUR)
        .in_key(G, HarmonicMinor)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Ride, "-:8 | - | - | - | - | - | x | - | - | -"),
            (Drum::Snare, "X:1 X X X X X X X | X X X X X X X X | X X X X X X X X | X X x X X X x \
                           X | X X X X X X X X | X X X x X X X X | X:8 | - | - | -"),
            (Drum::Crash, "-:8 | - | - | - | - | - | x | - | - | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 270 to 276.
        Part::new(
            "Charlatan",
            "-:3 C#3:1 C3:4 B2:2 Bb2:1 - | -:12 | -:3 C#3:1 C3:4 B2:2 Bb2:1 - | -:3 C#3:1 C3:4 \
             B2:2 Bb2:1 - | -:3 C#4:1 F#3:6 D3:1 - | -:3 C#4:1 F#3:2 E3 Eb3 D3:1 - | -:12",
            "",
        )
        .with_left(
            "F#1:4 -:8 | -:12 | F#1:4 -:8 | F#1:4 -:8 | F#1:4 F#4:1 -:7 | F#1:4 F#4:1 -:7 | -:4 \
             Bb1+D3+G#3+Bb3:8 & -:12 | - | - | -:3 F#2:7 (3 D2+F#3:1 - - | -:3 F#2:7 D2:1 - | \
             -:3 F#2:7 (3 F2+Bb3:1 - - | -:12 & -:3 F#2:7 D2:1 - | -:4 F#1:1 -:7 | -:3 F#2:7 \
             D2:1 - | -:3 D3+F#3:1 (3 E3+A3 _ F#3 (3 _ F3 _ _:2 - | -:12 | -:3 D3+F#3+F#4:1 (3 \
             E3+G#3 _ F#3 (3 _ G3 _ (3 F#3+A3 _ G3 (3 _ G#3 _ | -:12 & -:12 | - | - | - | -:3 \
             C#3:1 C3+C4:4 B2+B3:2 Bb2+Bb3:1 - | -:3 C#3:1 C3+C4:4 B2+B3:2 Bb2:1 - | -:12 & -:12 \
             | - | - | -:6 Bb3:2 (3 _:1 _ A3 (3 _ Bb3. _ | _ -:2 F#4:1 -:8 | -:6 Bb3:2 -:4 | \
             -:12 & -:12 | - | - | - | (3 -:1 - G#4+C#5+E5 (3 Eb5 D5 C5 Bb4:2. -:6 | (3 -:1 - \
             G#4+C#5+E5 (3 Eb5 D5 C5 Bb4:2. -:6 | -:12 & -:12 | - | - | - | (3 -:1 - D6+D8 (3 \
             E6+E7 E5+G#7 G#5+D7 G#4+G#6:2. -:6 | (3 -:1 - D6+D8 (3 E6+E7 E5+G#7 G#5+D7 G#4+G#6 \
             -:7 | -:12",
        )
        .with_tempo(&[tempo(0, 50)])
        .in_key(FSharp, Chromatic)
        .played_by(&[(gm::BASSOON, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 277 to 286.
        Part::new(
            "Cadenza",
            "D4:7 (3 Ab4:1 Bb4 D5 C5:1 Bb5:4 -:2 | D5:1 Eb5 G5 Bb5:5 Ab5:1 F5 D5 Bb4:5 | D5:1 F5 \
             Ab5 C6:5 Bb5:1 Ab5 F5 Eb5:5 | D5:1 Eb5 G5 Bb5:5 Ab5:1 F5 D5 Bb4:5 | D5:1 F5 Ab5 \
             D6:5 C6:1 Bb5 D5 Eb5:5 | G5:1 Bb5 Eb6 D6:5 C6:1 Ab5 Eb5 D5:5 | F5:1 Ab5 D6 C6:5 \
             Bb5:1 G5 Eb5 Bb4:5 | D5:1 Eb5 G5 Bb5:5 Ab5:1 F5 D5 Bb4:5 | D5:1 F5 Ab5 C6 Bb5 G6:5 \
             F6:2",
            "",
        )
        .in_meters(&[COMMON, COMMON, COMMON, COMMON, COMMON, COMMON, COMMON, COMMON, THREE_FOUR])
        .with_tempo(&[tempo(0, 50), tempo(16, 72), tempo(128, 50)])
        .in_key(ASharp, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 287 to 298.
        Part::new(
            "Puppets",
            "Eb6:1 -:2 F#6:1 F6:4 E6:2 Eb6. | (3 -:1 - G5 (3 F#5 F5 Eb5 C#5:2. - E6 Eb6. | (3 \
             -:1 - G5 (3 F#5 F5 Eb5 C#5:2. - E6 Eb6. | -:6 B5:2 Eb6 C6 | _ C#6 B6 G#6:1 - (3 - \
             G#6 A6 (3 A6 A6. _ | _ -:5 C6:2 E6 C#6 | _ D6 C7 A6:1 - (3 - - A6 (3 Bb6 Bb6. _ | _ \
             -:11 | -:3 G#6:1 D6:2 -:6 | -:7 G#6:1 F#6:2. - | -:11 Bb6:1 | F#6 -:11",
            "",
        )
        .with_left(
            "-:3 B2+B4+B5:7 G2+Eb5+G5:2. | -:3 B2+B5:7 G2+Eb5+G5:2. | -:3 B2+B5:7 G2:1 - | F1:4 \
             -:8 | -:12 | F#1:4 -:8 | -:12 | - | - | - | - | - & B1:4 -:8 | -:3 B4:2 -:7 | -:10 \
             Eb5+G5:2. | -:8 G#3:4 | G3+B3+A4:3 -:9 | -:8 A3:4 | G#3+C4+Bb4:3 -:9 | -:12 | - | - \
             | - | - & -:12 | - | - | -:7 A3:1 Bb3+F4 -:3 | -:8 Eb4:4 | -:3 A3+Eb4:1 -:3 \
             Bb3+E4:1 B3 -:3 | -:8 E4:4 | -:12 | - | - | - | - & -:12 | - | - | -:3 G#3+D4:1 -:3 \
             Eb4:4 Eb4:1 | F4:4 E4+F#4 - | -:11 F4:1 | F#4:4 G4 G#4:1 -:3 | -:12 | - | - | - | - \
             & -:12 | (3 -:2 - A5 (3 B3 B4 B4 (3 B4:1 B3 _ (3 _ _ B3. | _:2 -:1 G3+B3+B4+A5 (3 \
             A3+B3+C#4+D5 _ B4 (3 _ B4 _ -:4 | (3 -:2 - A4. (3 _:1 - - (3 G#4 F#4. _ (3 _ Bb4 E5 \
             (3 Eb5 E5 _ | Eb5:2. - (3 -:1 F#5 E5 (3 Eb5 E5 _ Eb5:2. - | -:3 A4:1 (3 Bb3+G#4 E4 \
             - -:2 F4 - | -:8 (3 -:1 - G#4 (3 A4 G#4 G#4 | (3 E4+G#4 F4+A4 F4+G#4 (3 F4+G#4 \
             F4+A4 E4+A4 G#4:2 (3 -:1 A4 - -:4 | -:12 | - | - | - & -:12 | - | -:6 B3:2 (3 \
             Bb3+B4:1 B3 _ (3 _ _ B3 | -:4 (3 A3+G4:1 Eb4 - -:2 E4 - | -:8 (3 -:1 C#5 G4 (3 A4 \
             G4 A4 | A4:2. -:1 Bb4. (3 _ _ G#4 (3 A4 F#4+G4+G#4 - A4:2. - | -:4 (3 -:1 G5 F5 (3 \
             G5 F5 F5 (3 E5+G5 D5+E5 - -:2 | -:4 (3 E4:1 F4 F4 (3 E4+G#4 - F4+G#4 (3 E4+A4 F4+A4 \
             E4+G#4 (3 F4+G#4 F4+A4 E4 | -:12 | - | - | - & -:12 | - | -:6 Eb4+Eb5:2 (3 _:1 _ \
             D4+D5 (3 _ Eb3+C#4+G4 _ | (3 -:2 - G#4 (3 -:1 - G4 (3 - F4+G4 _ G#4:2. - | (3 -:1 \
             F5 A5 (3 B5 A5 A5 (3 B5+C6 C6. _ (3 _ - - -:4 | -:8 (3 -:1 B4 F5 (3 F#5 F5 F5 | (3 \
             E5 F#5 Bb5 (3 C6 Bb5 Bb5 (3 C6+C#6 C#6. _ (3 _ - - -:4 | -:12 | - | - | - | - & -:3 \
             G6:2. -:7 | -:3 G6:2. -:7 | -:3 G6:1 -:8 | -:11 G#4:1 | -:3 A4+C#5:1 C5:3 C5:1 -:4 \
             | -:11 A4:1 | -:3 Bb4+D5:1 C#5:3 C#5:1 -:4 | -:12 | D5+F#5:4 -:8 | -:12 | - | - & \
             -:12 | (3 -:1 - G6 (3 A5 C#6 G5 -:8 | (3 -:1 - G6 (3 A5 C#6 G5 -:8 | -:12 | -:7 \
             C6:1 F#5:2. -:1 Eb5 | G4:2. -:10 | -:7 C#6:1 -:3 E5:1 | -:12 | - | - | - | -",
        )
        .in_key(B, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Piano),
    ],
    form: &["Fair", "Flute again", "Piccolo", "Piccolo, bar 6", "Piccolo, bar 8", "Piccolo, bar 10", "Piccolo, bar 11", "Piccolo, bar 12", "Piccolo, bar 13", "Piccolo, bar 15", "Crowd", "Crowd tune", "Crowd, 3/4", "Crowd, end", "Merrymakers", "Merrymakers, 3/4", "Merrymakers, end", "Revellers", "Revellers, bar 2", "Revellers, bar 3", "Revellers, bar 6", "Piccolo in 5/8", "Piccolo in 5/8, bar 2", "Flute tune", "Violins", "Violins, bar 2", "Violins, bar 3", "Organ grinder", "Organ grinder, bar 3", "Organ grinder, bar 4", "Revellers again", "Revellers again, bar 2", "Piccolo again", "Violins again", "Violins again, bar 3", "Barrel organ", "Wooden leg", "Music box", "Wooden leg again", "Fair returns", "Revellers 4", "Revellers 4, bar 2", "Revellers 4, bar 3", "Revellers 4, bar 4", "Revellers 4, bar 5", "Revellers 4, bar 6", "Revellers 4, bar 4", "Revellers 4, bar 5", "Revellers 4, bar 9", "Fair, flute", "Piccolo, return", "Piccolo, return, bar 3", "Piccolo, return, bar 4", "Piccolo, return, bar 5", "Crowd 2", "Crowd tune 2", "Crowd 2, 3/4", "Crowd 2, end", "Merrymakers 2", "Merrymakers 2, 3/4", "Merrymakers 2, end", "Revellers", "Revellers, bar 2", "Revellers, bar 3", "Revellers, bar 6", "Piccolo in 5/8", "Piccolo in 5/8, bar 2", "Flute tune", "Revellers 3", "Revellers again, bar 2", "Piccolo 3", "Clarinets", "Clarinets, bar 2", "Clarinets, bar 3", "Clarinets, bar 4", "Flutes", "Flutes, bar 2", "Flutes, bar 3", "Flutes, bar 4", "Flutes, bar 5", "Flutes, bar 6", "Piccolo 4", "Climax", "Climax, 5/8", "Drum roll", "Charlatan", "Cadenza", "Puppets"],
    drums_from: 0,
    about: "The first tableau of Petrushka, the ballet Stravinsky wrote for Diaghilev's Ballets \
            Russes (Paris, 1911), here in his revision of 1947 for a smaller orchestra. A \
            Shrovetide fair in St Petersburg in the 1830s: the crowd, a barrel organ and a \
            dancer, a music box and a rival dancer, drunken revellers, and at the end the old \
            Charlatan, who plays his flute and brings his three puppets to life. The tunes are \
            modal, as Russian folk tunes are, and stand over drones and ostinati rather than \
            chord progressions. The flute's opening tune, a fourth A to D and then E, G, A, is \
            in D Dorian (D minor with B natural), over the clarinets and a horn rocking between \
            D with A and E with G. The crowd's tune comes in thick block chords, G Dorian, under \
            the piccolo's high G, with triangle and cymbals; the merrymakers answer on C. The \
            revellers' tune in E Dorian changes meter almost bar by bar, 2/4, 5/8, 3/8, and the \
            piccolo's 5/8 bars swing between two chords a fourth apart. The barrel organ, \
            slower, wheezes in the flutes while the clarinets play its tune in B flat, and a \
            dancer with a triangle dances to the flute's \"Elle avait un' jambe en bois\", a \
            French music-hall song of the day; a music box (celesta and harp) answers it. The \
            fair comes back and builds to a climax, a drum roll cuts it off, and the Charlatan \
            appears, Lento, over low chromatic lines; his flute cadenza runs up the notes of B \
            flat seventh and E flat major. Then the puppets come to life in short chromatic \
            phrases. Petrushka's own chord, C major and F sharp major at once, a tritone apart, \
            belongs to the next tableau, \"Petrushka's Room\".",
};

// Copyright: this is Stravinsky's 1947 revision of Petrushka, which is still under
// copyright (Boosey & Hawkes); it is added here at the request of the project's owner.
// Source: Stravinsky's Petrushka, 1947 revision, the Russian Dance that ends the first tableau,
// MIDI bars 299 to 432, a song of its own because the whole tableau needs more than 99 patterns
// (the fair before it is "Petrushka, The Shrovetide Fair"). The notes are converted by script
// from a MIDI file of the 1947 version (the person's own copy, at sounding pitch) and checked
// bar by bar against that file, not against a printed score; MIDI bar numbers are given below
// (MIDI bar 1 is a silent lead-in). The melody is the line in front, in the octave of the
// instrument that has it, and played_by follows it from instrument to instrument. Everything
// else that matters is written out on the Keys channel: the harmony, the bass, the ostinati and
// the timpani; lines doubled in unison or octaves are written once. Rhythms are quantised to
// sixteenths and eighth-note triplets; tremolos and rolls are written as held notes, grace
// notes are left out. The dynamic of each part is read from the MIDI's velocities. The
// percussion is written out (triangle as Ride, tambourine as Shaker, bass drum as Kick, side
// drum as Snare, cymbals as Crash). Where the meter changes, every run of bars in one meter is
// a part of its own, named "..., bar N" after the bar it starts on, with in_time: in_meters
// cannot cut a written-out left hand of several voices into its runs, nor carry drums. No chord
// symbol is honest for this music, which moves in drones, ostinati and parallel chords, so the
// charts are empty.
//
// Left out: the harps and the celesta in bars 357 to 368, where too many lines play at once.
// The piano, which has a part of its own in this dance, is kept; where it leads (bars 394 to
// 401 and 410 to 416) the tune is its top line, and the rest of its chords are on the Keys
// channel.
//
// Not certain: bar 393 is 9/16 in the MIDI: written here as 2/4, the last sixteenth (a breath
// before the piano comes back a tempo) left out. Bars 388 and 391 are 5/8 in the MIDI, each
// with a silent last eighth that may be a pause rather than a bar of 5/8; they are kept as 5/8,
// counted 2 + 3. The tempo of the slow bars, 386 to 393, is the MIDI's: a fall from 116 within
// the first beat to 34, written as 35 (the slowest the demos allow at a tempo of 116), and 116
// again from bar 394.
pub(super) const PETRUSHKA_RUSSIAN_DANCE: Song = Song {
    name: "Petrushka, Russian Dance",
    origin: "Igor Stravinsky, Petrushka, 1911, revised 1947",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: G,
    mode: Mixolydian,
    time: TWO_FOUR,
    tempo: 116.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FLUTE, gm::ACOUSTIC_GRAND_PIANO, gm::CONTRABASS),
    style: "",
    parts: &[
        // MIDI bars 299 to 306.
        Part::new(
            "Russian Dance",
            "B5:1 C6 D6:2 E6:1 D6 C6 B5 | A5:2 A5 B5:4 | B5:1 C6 D6 E6 D6:2 C6:1 B5 | A5:2 A5 \
             B5:4 | B5:1 C6 D6:2 E6:1 D6 C6 B5 | A5:2 A5 B5 B5 | B5:1 C6 D6 E6 D6 C6 B5 A5 | \
             G6:2 F6 E6:4",
            "",
        )
        .with_left(
            "-:8 | - | G2+F4:4 - | -:8 | G2+F4+B5:4 - | -:8 | G2+F4:4 - | -:2 C4 A4 - & \
             G2+F4+B5:4 - | - A3:2. - | B5:3 -:5 | G3+A4:2 G3+A4 A3. - | -:8 | G3+A4:2 G3+A4 - \
             A3. | B5:3 -:5 | -:4 B3 & G3+G4+A4+A5:1 G3+G4+A4+A5 G3+G4+A4:2 G3+G4+A4+A5:1 \
             G3+G4+A4 G3+G4+A4+A5 G3+G4+A4+A5 | G3+A4:2 G3+A4 G3+G4+A4+A5:4 | G3+G4+A4+A5:1 \
             G3+G4+A4+A5 G3+G4+A4 G3+G4+A4+A5 G3+G4+A4:2 G3+G4+A4+A5:1 G3+G4+A4+A5 | -:4 \
             G3+G4+A4+A5 | G3+G4+A4+A5:1 G3+G4+A4+A5 G3+G4+A4:2 G3+G4+A4+A5:1 G3+G4+A4 \
             G3+G4+A4+A5 G3+G4+A4+A5 | -:4 G3+G4+A4+A5:2 G3+G4+A4+A5 | G3+G4+A4+A5:1 G3+G4+A4+A5 \
             G3+G4+A4 G3+G4+A4+A5 G3+G4+A4 G3+G4+A4+A5 G3+G4+A4+A5 G3+G4+A4 | G3+G4+A4+G5:2 \
             G3+G4+A4 F4:1 -:3 & B3+D5+F5:1 B3+D5+E5+F5+G5 B3+D5+F5+A5:2 B3+D5+F5+G5+B5:1 \
             B3+D5+F5+A5 B3+D5+E5+F5+G5 B3+D5+F5 | C4+E5+G5:2 C4+E5+G5 B3+D5+F5:4 | B3+D5+F5:1 \
             B3+D5+E5+F5+G5 B3+D5+F5+A5 B3+D5+F5+G5+B5 B3+D5+F5+A5:2 B3+D5+E5+F5+G5:1 B3+D5+F5 | \
             C4+E5+G5:2 C4+E5+G5 B3+D5+F5:4 | B3+D5+F5:1 B3+D5+E5+F5+G5 B3+D5+F5+A5:2 \
             B3+D5+F5+G5+B5:1 B3+D5+F5+A5 B3+D5+E5+F5+G5 B3+D5+F5 | C4+E5+G5:2 C4+E5+G5 B3+D5+F5 \
             B3+D5+F5 | B3+D5+F5:1 B3+D5+E5+F5+G5 B3+D5+F5+A5 B3+D5+F5+G5+B5 B3+D5+F5+A5 \
             B3+D5+E5+F5+G5 B3+D5+F5 B3+C5+D5+E5+F5 | B3+D5+F5+A5:2 B3+D5+F5+A5 G3+E5+B5:4 & -:8 \
             | - | - | - | - | - | - | -:2 C5 (3 -:1 C5+C6 D5+D6 (3 F5+F6 G5+G6 B6",
        )
        .in_key(G, Mixolydian)
        .played_by(&[(gm::FLUTE, 0), (gm::PICCOLO, 12)])
        .with_drums(&[(
            Drum::Shaker,
            "-:8 | - | - | - | - | - | - | -:4 (3 x:1 x x (3 x x x",
        )])
        .marked(Dynamic::Forte),
        // MIDI bar 307.
        Part::new("Piccolo and celesta", "E7:1 -:7", "")
            .with_left(
                "A2:4 D4:2 - & B4+A5:1 G4 G4+B4 G4 A4+C5 F#4 D3+F#4+A4+C5 F#4 & G4+D5+E5+E7:1 \
             E4+B4+D5+E5 A3+D5+E5 E4+B4+D5+E5 F#4+D5+E6 E4+A4+C5+D5+E5 D4+D5+E5 E4+A4+C5+D5+E5 & \
             (3 -:1 - Eb6 (3 E6 - - E5 -:3",
            )
            .in_key(A, Dorian)
            .played_by(&[(gm::PICCOLO, 0)])
            .with_drums(&[(Drum::Shaker, "x:8")])
            .marked(Dynamic::Forte),
        // MIDI bar 308.
        Part::new("Trumpet", "E5:2 E5:1 E5 E5 E5 E5 E5", "")
            .with_left(
                "A2:4 - & A4+C5:1 F#4 A3+F#4+A4+C5 F#4 D4+B4 G4 D3+G4+B4 G4 & F#4+D5:1 \
             E4+A4+C5+D5+E5 D4+D5 E4+A4+C5+D5 G4+D5 E4+B4+D5 A3+D5 E4+B4+D5",
            )
            .in_key(A, Dorian)
            .played_by(&[(gm::TRUMPET, 0)])
            .marked(Dynamic::Forte),
        // MIDI bar 309.
        Part::new("Clarinet", "E5:2 G5 F#5:1 E5 A5 -", "")
            .with_left(
                "A2:4 - & B4+E5:1 G4 G4+B4 G4 D4+A4+C5 F#4 D3+F#4+A4+C5 F#4 & G4+D5:1 E4+B4+D5+E5 \
             A3+D5+E5 E4+B4+D5+E5 F#4+D5+E5 E4+A4+C5+D5 D4+D5+E5 E4+A4+C5+D5+E5",
            )
            .in_key(A, Dorian)
            .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bar 310.
        Part::new("Oboe", "E5:2 G5:1 F#5 E5 D5 E5 F#5", "")
            .with_left(
                "A2:4 - & A4+C5:1 F#4 A3+F#4+A4+C5 F#4 D4+B4 G4 D3+G4+B4 G4 & F#4+D5:1 \
             E4+A4+C5+D5+E5 D4+D5+E5 E4+A4+C5+D5+E5 G4+D5 E4+B4+E5 A3+D5+E6 E4+B4+D5+E5+F6",
            )
            .in_key(A, Dorian)
            .played_by(&[(gm::OBOE, 0)])
            .with_drums(&[(Drum::Shaker, "-:6 (3 x:1 x x")]),
        // MIDI bar 312.
        Part::new("Trumpet again", "E4:1 E4 E4 E4 E4 E4 E4 E4", "")
            .with_left(
                "A2:4 - & A4+C5:1 F#4 A3+F#4+A4+C5 F#4 D4+B4 G4 D3+G4+B4 G4 & F#4+D5+E5:1 \
             A4+C5+D5+E5 D4+D5+E5 A4+C5+D5+E5 G4+D5+E5 B4+D5+E5 A3+D5+E5 B4+D5+E5",
            )
            .in_key(A, Dorian)
            .played_by(&[(gm::TRUMPET, 0)])
            .marked(Dynamic::Forte),
        // MIDI bars 313 to 314.
        Part::new(
            "Clarinet again",
            "E5:2 F#5:1 G5 F#5 E5 A5 G5 | F#5 E5 G5 F#5 E5 D5 E5 G5",
            "",
        )
        .with_left(
            "A2:4 D4:1 F#4 D3+F#4+A4+C5 F#4 | A2:4 - & B4+E5:1 G4 G4+B4 G4 A4+C5. -:3 | A4+C5:1 \
             F#4 A3+F#4+A4+C5 F#4 D4+B4 G4 D3+G4+B4 G4 & G4+D5:1 E4+B4+D5+E5 A3+D5+E5 \
             E4+B4+D5+E5 F#4+D5+E5 E4+A4+C5+D5 D4+D5+E5 E4+A4+C5+D5+E5 | F#4+D5+E5 E4+A4+C5+D5 \
             D4+D5+E5 E4+A4+C5+D5+E5 G4+D5 E4+B4+E5 A3+D5 E4+B4+D5+E5",
        )
        .in_key(A, Dorian)
        .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bars 315 to 318.
        Part::new(
            "Oboe again",
            "F#5:1 E5 F#5 G5 F#5 E5 A5 G5 | F#5 E5 G5 F#5 E5 D5 E5 F#5 | E5 F#5 G5 F#5 E5 D5 E5 \
             F#5 | G5 F#5 E5 D5 E5 D5 E5 G5",
            "",
        )
        .with_left(
            "-:8 | -:4 D4:1 - D3+G4+B4 - | A2:4 - | -:6 D3:2 & A2:4 D4:1 F#4 D3+F#4+A4+C5 F#4 | \
             A2+F#4:4 G4:2. -:1 G4 | - G4 G4+B4 G4 D4 F#4 D3+F#4+A4+C5 F#4 | A2+F#4:4 D4:2 - & \
             B4:1 G4 G4+B4 G4 A4+C5. -:3 | A4+C5:1 - A3+A4+C5 - B4. -:3 | B4+E5:1 -:3 A4+C5:1. \
             -:3 | A4+C5:1 - A3+A4+C5 - (3 E4 F#4 - -:2 & G4+D5+E5:1 E4+B4+D5 A3+D5+E5 \
             E4+B4+D5+E5 F#4+D5+E5 E4+A4+C5+D5 D4+D5+E5 E4+A4+C5+D5+E5 | D5+E5 A4+C5+D5+A5 D5+E5 \
             A4+C5+D5+E5+A5 D5+G5 B4+E5+B5 A4+D5 B4+D5+E5+B5 | G4+D5 E4+B4+D5+E5 A3+D5+E5 \
             E4+B4+D5+E5 F#4+D5 E4+A4+C5+E5 D4+D5 E4+A4+C5+D5+E5 | D5+E5+F#5 A4+C5+D5+E5+A5 D5 \
             A4+C5+E5+A5 G5 E5+B5 A4 E5+B5 & -:8 | - | - | -:4 G4+B4+D5+E6:1 G4+B4+F#6 \
             G4+B4+D5+A6 G4+B4+D5+C7",
        )
        .in_key(A, Dorian)
        .played_by(&[(gm::OBOE, 0)]),
        // MIDI bars 319 to 322.
        Part::new(
            "Second tune",
            "F#6:1 E6 F#6 G6 F#6 E6 A6 G6 | F#6 E6 G6 F#6 E6 D6 E6 F#6 | E6 F#6 G6 F#6 E6 D6 E6 \
             F#6 | G6 F#6 E6 D6 E6 F#6 G#6 A6",
            "",
        )
        .with_left(
            "(3 D4:1 D4 D4+E4 (3 D4 E4+B4 E4 (3 D4+A4+B4 A4 D4+E4 (3 - - E4 | (3 D4 D4 D4+E4 (3 \
             - E4+B4 E4 (3 D4+A4+B4 A4 - -:2 | - (3 -:1 E4+B4 - (3 D4+A4+B4 A4 - -:2 | - (3 -:1 \
             E4+B4 - (3 E4+B4 - E4+B4 -:2 & -:8 | -:4 (3 -:1 E4 D4+E4 (3 - - E4 | (3 D4 D4 D4+E4 \
             (3 D4 - E4 (3 - E4 D4+E4 (3 D4 - E4 | (3 D4 D4 D4+E4 (3 - - E4 (3 - F4 - (3 E4 - - \
             & D3+E4+G4+C5+D5+E5:1 D3+D5+E5 C3+C4+F#4+A4+D5+E5 C3+D5+E5 D3+G4+C5+D5+E5 \
             D3+A4+D5+E5 C3+C4+E4+Bb4+D5+E5 C3+D5+E5 | D3+E4+G4+C5+D5+E5 D3+D5+E5 \
             C3+C4+F#4+A4+D5+E5 C3+D5+E5 D3+G4+C5+D5+E5 D3+A4+D5+E5 C3+C4+E4+Bb4+D5+E5 C3+D5+E5 \
             | D3+E4+G4+C5+D5+E5 D3+D5+E5 C3+C4+F#4+A4+D5+E5 C3+D5+E5 D3+G4+C5+D5+E5 D3+A4+D5+E5 \
             C3+C4+E4+Bb4+D5+E5 C3+D5+E5 | D3+E4+G4+C5+D5+E5 D3+D5+E5 C3+C4+F#4+A4+D5+E5 \
             C3+G4+D5+E5 E3+G4+A4+D5+E5+F#5 E3+A4+E5+F#5 D3+D4+G#4+B4+E5+F#5 D3+E5+F#5 & B5:1 \
             B4+C5+C6 F#5+A5+D6 A4+C5+C6 E4+F#5+B5+D6 B4+C5+G5+C6 D4+F#5+A5+D6 A4+Bb4+C5+C6 | \
             C4+A4+A5+B5+D6 B4+C5+C6 D4+A5 A4+C5+C6 B5 B4+C5+C6 D4+F#5+A5+D6 A4+Bb4+C5+C6 | \
             G5+B5+D6 B4+C5+F#5+C6 A5+D6 A4+C5+C6 B5+D6 B4+C5+C6 A5 A4+Bb4+C5+C6 | B5 B4+C5+C6 \
             D4+A5 D4+A4+C6+F#6 D4+F#4+G5+C6+G6 D4+F#4+C5+D5+D6 F#4+D5+B5+E6 D4+F#4+B4+D5+D6",
        )
        .in_key(C, Lydian)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 323 to 324.
        Part::new(
            "Second tune, a third up",
            "A#6:1 G#6 A#6 B6 A#6 G#6 C#7 B6 | A#6 G#6 A#6 B6 G#6 F#6 G#6 A#6",
            "",
        )
        .with_left(
            "(3 -:1 G4 F#4 (3 C#5 G#4+D5 G#4 (3 - G4 - (3 - G#4 G#4 | (3 F#4 F#4+G4 F#4 (3 - C5 \
             - (3 - G4 - (3 - - G#4 & F#3+G#4+B4+E5+F#5+G#5:1 F#3+G#4+F#5+G#5 E3+E4+A#4+F#5+G#5 \
             E3+F#5+G#5 F#3+B4+E5+F#5+G#5 F#3+G#4+C#5+F#5+G#5 E3+E4+D5+F#5+G#5 E3+F#5+G#5 | \
             F#3+G#4+B4+E5+F#5+G#5 F#3+G#4+F#5+G#5 E3+E4+A#4+F#5+G#5 E3+F#5+G#5 \
             F#3+B4+E5+F#5+G#5 F#3+G#4+C#5+F#5+G#5 E3+E4+D5+F#5+G#5 E3+F#5+G#5 & -:2 (3 -:1 C5 - \
             (3 F#4+C5+D5 G#4+C#5 F#4+D5 -:2 | - (3 F#4+C#5+B5:1 G#4+D5 G#4+C#5 (3 F#4+C5+D5 \
             G#4+C#5 F#4+D5 (3 - G#4+A#5 - & E4+F#4+D6+D#6:1 D#5+E5+E6 F#4+C6 C5+C#5+E5+E6 D#6 \
             D#5+E5+E6 F#4+A#5+C6+F6 C5+D5+E5+E6 | A#5+D#6+F#6 D#5+E5+B5+E6 A#5+C6+F#6 C5+E5+E6 \
             C#6+D#6+F#6 D#5+E5+E6 F#4+C6 C5+C#5+E5+E6",
        )
        .in_key(E, Lydian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 325 to 334.
        Part::new(
            "Second tune, a fourth up",
            "B6:1 A6 B6 C7 B6 A6 D7 C7 | B6 A6 B6 C7 B6 A6 D7 C7 | B6 A6 B6 C7 B6 A6 D7 C7 | B6 \
             A6 C7 B6 A6 G6 A6 B6 | A6 B6 C7 B6 A6 G6 A6 B6 | C7 B6 A6 G6 A6 B6 C7 D7 | C7 B6 A6 \
             G6 A6 B6 C7 D7 | C7 B6 A6 G6 A6 B6 C7 D7 | C7:2. -:6 | E6:1 F6 G6 A6 B6 C7 D7 E7",
            "",
        )
        .with_left(
            "-:2 (3 -:1 D5 - (3 G4+D5+E5 D5 - -:2 | - (3 D5:1 A4+E5 A4 (3 - - G4 (3 - A4 A4 | (3 \
             G4 G4 G4+A4 (3 G4 - A4 (3 - A4 G4+A4 (3 G4 - A4 | (3 G4 G4 G4+A4 (3 - - A4 (3 - - \
             G4+A4 (3 - - A4 | (3 G4 G4 G4+A4 (3 G4 - A4 (3 - - G4+A4 (3 G4 - A4 | (3 G4 G4 \
             G4+A4 (3 G4 - A4 (3 - - G4+A4 (3 G4 - A4 | (3 G4 G4 G4+A4 (3 G4 - A4 (3 - - G4+A4 \
             (3 G4 - A4 | (3 G4 G4 G4+A4 (3 G4 - A4 (3 - - G4+A4 (3 G4 - - | - E4 F4 F4 - E4+B4 \
             D4+C5 D4+D5 | C4+B4 C4+C5 B3+D5 B3 A3 - G3 - & (3 G4:1 G4 G4 (3 G4+D5 A4+E5 A4 (3 - \
             - G4 (3 - A4 A4 | (3 G4 G4 G4 (3 - D5 - (3 G4+D5+E5 A4+D5 - -:2 | - (3 -:1 A4+E5 - \
             (3 G4+E5 - - -:2 | - (3 -:1 A4+E5 - (3 G4+E5 - - -:2 | - (3 -:1 A4+E5 - (3 G4+E5 - \
             - -:2 | - (3 -:1 A4+E5 - (3 G4+E5 - - -:2 | - (3 -:1 A4+E5 - (3 G4+E5 - - -:2 | - \
             (3 -:1 A4+E5 - (3 G4+E5 - - -:2 | C5:1 -:7 | -:5 A3+G5:1 - G3+B5 & \
             G3+A4+C5+F5+G5+A5:1 G3+A4+G5+A5 F3+F4+B4+G5+A5 F3+G5+A5 G3+C5+F5+G5+A5 \
             G3+A4+D5+G5+A5 F3+F4+Eb5+G5+A5 F3+G5+A5 | G3+A4+C5+F5+G5+A5 G3+A4+G5+A5 \
             F3+F4+B4+G5+A5 F3+G5+A5 G3+C5+F5+G5+A5 G3+A4+D5+G5+A5 F3+F4+Eb5+G5+A5 F3+G5+A5 | \
             G3+A4+C5+F5+A5+B5 G3+F5+A5 F3+F4+B4+D5+G5+B5 F3+B4+C6 G3+D5+F5+B5+C6+A6 \
             G3+C5+D5+A5+D6 F3+A4+Eb5+G5+C6+D6+Eb6 F3+G5+C6 | G3+A4+C5+F5+A5+B5 G3+F5+A5 C6:2. \
             -:1 G5:2. -:1 | -:2 C6. -:1 G5:2. B5:1. | _ -:7 | -:8 | -:6 F4:2 | _:1 \
             G3+B3+G4+B4+E5 F3+A3+C4+A4+C5+F5 -:5 | -:8 & -:8 | - | - | -:2 F3+D5+G5+B5:1 \
             F3+B4+B5 G3+A4+C5+D5+F5+A5+C6 G3+C5+D5+D6 F3+A4+Eb5+A5+Eb6 F3+G5+B5 | \
             G3+A4+C5+F5+A5 G3+F5+B5 F3+D5+G5+B5 F3+B4+B5 G3+A4+C5+D5+F5+A5+C6 G3+C5+D5+D6 \
             F3+A4+Eb5+A5+Eb6 F3+G5 | G3+A4+C5+F5+A5 G3+F5 F3+F4+B4+D5+G5+B5 F3+B4 \
             G3+A4+C5+D5+F5+C6 G3+C5+D5+D6 F3+A4+Eb5+G5+Eb6 F3+G5 | G3+A4+C5+F5+A5 G3+F5 \
             F3+F4+B4+D5+G5+B5 F3+B4 G3+A4+C5+D5+F5+C6 G3+C5+D5+D6 F3+A4+Eb5+G5+Eb6 F3+G5 | \
             G3+A4+C5+F5+A5 G3+F5 F3+F4+B4+D5+G5+B5 F3+B4 G3+A4+C5+D5+F5+C6 G3+C5+D5+D6 \
             F3+A4+Eb5+G5+Eb6:2 | F6. -:6 | E6:2. -:6 & -:8 | - | - | - | - | - | - | - | \
             G3+A4+E5+F5+A5:1 -:2 F3+B4+D5+G5:1 E3+C5+A5 E3+D5+F5+B5 D3+E4+G4+E5+G5+C6 \
             D3+F5+A5+D6 | C3+G4+E5+G5+B5 C3+A4+A5+C6 B2+B4+B5+D6 B2+C5+C6+E6 A2+B4+D5+D6+F6 \
             A2+C5+E5+E6+G6 G2+D5+F5+F6+A6 G2+E5+G5+G6+B6 & -:8 | - | -:1 B5 A5 A5+B5 A5 B5 \
             A5+B5 A5+B5 | - B5 A5 A5 B5 A5+B5 B5 A5 | B5 A5 A5 A5 B5 A5+B5 B5 A5 | - A5+B5 A5 \
             A5+B5 A5+B5 A5+B5 A5+B5 A5+B5 | B5 A5+B5 A5 A5+B5 A5+B5 A5+B5 A5+B5 A5+B5 | B5 \
             A5+B5 A5 A5+B5 A5+B5 A5+B5 A5+B5 A5+B5 | G4+B5 A4 G4 G4+A4 G4+A4 G4+A4 A4 G4 | A4 \
             G4 G4+A4 G4+A4 G4+A4 G4+A4 G4+A4 G4+A4 & E6:1 E5+F5+F6 B5+D6+G6 D5+F5+F6 \
             A4+B5+E6+G6 E5+F5+C6+F6 G4+B5+D6+G6 D5+Eb5+F5+F6 | F4+D5+D6+E6+G6 E5+F5+F6 G4+D6 \
             B4+D5+F5+F6 E6 E5+F5+F6 G4+B5+D6+G6 D5+Eb5+F5+F6 | G5+E6+G6 E5+C6+F6 D6+G6 \
             D5+F5+G5+F6 C5+G5+D6+E6+G6 E5+F5+F6 - D5+Eb5+F5+F6 | G5+E6 E5+F6 G4+D6 D5+F5+F6 \
             G5+E6 E5+F5+F6 G4+D6+G6 D5+Eb5+F5+F6 | G5+C6+E6+G6 E5+F6 D6+G6 D5+F5+G5+F6 G5+E6+G6 \
             E5+F5+F6 D6 D5+Eb5+F5+F6 | G5+E6 E5+F6 B3+D6 F4+D5+F5+F6 G5+E6 E5+F5+F6 G3+F5+D6 \
             G3+D5+F6 | F4+G5+E6 F4+B4+E5+F6 G3+C5+F5+C6+D6 G3+F4+D5+F5+F6 F4+G5+E6 \
             F4+B4+E5+F5+F6 G3+C5+F5+C6+D6 G3+B4+D5+F6 | F4+G5+E6 F4+B4+E5+F6 G3+C5+F5+C6+D6 \
             G3+F4+D5+F5+F6 F4+G5+E6 F4+B4+E5+F5+F6 G3+C5+F5+C6+D6 G3+B4+D5+F6 | A3+B4+G5+C6+E6 \
             F5 D6 E5+E6 C6 F3+C5+D6 A3+B5 C4+B4+C5+C6 | F4+A5 B4 C5+G5 E5+B5 (3 F5+A5 F4+A5+C6 \
             - (3 E5 E4+A5 C6+C7",
        )
        .in_key(F, Lydian)
        .played_by(&[(gm::PICCOLO, 0)]),
        // MIDI bars 335 to 342.
        Part::new(
            "Tutti",
            "D7:2 C7:1 B6 A6:2 A6 | B6:4 B6:1 C7 D7:2 | E7:1 D7 C7 B6 A6:2 A6 | B6:4 B6:1 C7 D7 \
             E7 | D7:2 C7:1 B6 A6:2 A6 | B6 B6 B6:1 C7 D7 E7 | D7 C7 B6 A6 G6:2 F6 | E6:4 -",
            "",
        )
        .with_left(
            "B3+D4+F4+A4:2 B3+D4+F4+A4+A5:1 B3+D4+A4+A5 C4+G4+A4:2 C4+G4+A4 | B3+D4+A4+A5:4 \
             B3+D4+A4+A5:1 B3+D4+F4+A4+A5 B3+D4+F4:2 | B3+D4+F4+G4+A5:1 B3+D4+F4 B3+D4+F4+A4+A5 \
             B3+D4+A4+A5 C4+G4+A4:2 C4+G4+A4 | B3+D4+A4+A5:4 B3+D4+A4+A5:1 B3+D4+F4+A4+A5 \
             B3+D4+F4 B3+D4+F4+A4+A5 | B3+D4+F4:2 B3+D4+F4+A4+A5:1 B3+D4+A4 C4+G4+A4:2 C4+G4+A4 \
             | B3+D4+A4+A5 B3+D4+A4+A5 B3+D4+A4+A5:1 B3+D4+F4+A4+A5 B3+D4+F4 B3+D4+F4+A4+A5 | \
             B3+D4+F4+G4 B3+D4+F4+A4+A5 B3+D4+A4+A5 B3+D4+F4+A4 B3+F4+A4+A5:2 B3+D4+F4+A4 | G4:1 \
             -:7 & -:8 | -:1 G5 G5 G4+G5 C4+E4+G4+B5 C4+E4+G5+C6 C4+E4+G4+G5+D6 G4+G5 | - G3+G5 \
             - G4+G5 -:4 | -:1 G5 G5 G4+G5 -:4 | -:1 A4+G5 -:6 | -:1 G5 - G4+G5 -:4 | -:1 G3 -:6 \
             | -:4 E2+E3:1 - B1 - & -:2 C5+E5:1 -:2 C5:1 B3:2. | -:1 F5 D5 -:2 G3+E5:1 - A3 | \
             E5+G5 - C5+E5 -:2 C5:1 B3:2. | -:1 F5 D5 - G4 G3+E4+E5 C4 A3 | -:2 C5+E5:1 -:2 C5:1 \
             B3:2 | -:1 F5 - B4 G4 G3+E4+C5+E5 C4 A3+E5 | - C5+E5 - C5+E5 - D5 - A5 | E4:4 - & \
             G3+B4+D6+F6+A6:1 A3 G3+F5+C6+E6+G6 G3+B5+D6+F6 G3+E4+G5+A5+C6+E6 A3 G3+E4+G5+E6 G3 \
             | G3 A3 G3 G3 -:3 G3:1 | -:5 A3:1 A5+C6:2. | F4+B4+D6+F6:4 G3+B4+D6+F6:1 \
             A3+D5+F5+E6+G6 G3+B4+D5+F6+A6 G3+D5+F5+B6 | G3+A4+B4+D6+F6+A6 - G3+F5+C6+E6+G6 \
             G3+B4+A5+D6+F6 G3+E4+G5+A5+C6+E6 - G3+E4+G5+E6 G3 | - A3 - G3 -:4 | -:5 A3:1 - G3 | \
             -:3 D3:1 Eb3:2 B2:1 B2 & -:6 A5+C6:2. | F4+B4+D6+F6:4 G3+B4+D6+F6:1 A3+D5+F5+E6+G6 \
             G3+A4+B4+D5+F6+A6 - | G3+D5+F5+E6+B6 A3+B4+D6+F6+A6 G3+F5+C6+E6+G6 G3+B5+D6+F6 \
             G3+E4+G5+A5+C6+E6 - G3+E4+G5+E6 G3 | G3 A3 G3 G3 -:4 | -:1 A3 -:3 A3:1 A5+C6:2. | \
             G3+F4+B4+D6+F6:1 - G3+F4+B4+D6+F6 - G3+B4+D6+F6 A3+D5+F5+E6+G6 G3+B4+D5+F6+A6 \
             G3+D5+F5+B6 | G3+B4+D6+F6+A6 A3+F5+C6+E6+G6 G3+B5+D6+F6 G3+F5+A5+C6+E6 \
             G3+D4+F5+B5+D6 - G3+C4+B4+D5+F5+A5+C6 - | G3+B3+B4+G5+B5:3 -:2 E3:1 -:2 & G6:1 G5 - \
             G4+G5 - G5 - G5 | B5:4 - | -:5 G5:1 - G5 | B5:4 B5:1 G5+C6 G5+D6 G4+G5+E6 | -:3 \
             G4+G5+B5:1 - G5 - G5 | B5:2 G5+B5:1 - B5 G5+C6 G5+D6 G4+G5+E6 | -:2 G5:1 G4+G5 G5:2 \
             G5:1 G4+G5 | E5:2 -:6",
        )
        .in_key(G, Mixolydian)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[(Drum::Kick, "-:8 | - | - | - | - | - | x | -")])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 343 to 356.
        Part::new(
            "Bassoon",
            "(3 E4:1 D4 A3 E3:2 (3 E4:1 D4 A3 E3:2 | (3 F#4:1 D4 A3 E3:2 (3 G4:1 D4 A3 E3:2 | (3 \
             G#4:1 D4 A3 E3:2 (3 A4:1 D4 A3 E3:2 | Bb3:1 B3 D4 D3 D#3 E3:2. -:1 | (3 F4 E4 D4 \
             F3:2 (3 F#4:1 E4 D4 F#3:2 | (3 G4:1 E4 D4 G3:2 (3 G#4:1 E4 D4 G#3:2 | (3 A4:1 E4 D4 \
             A3:2 (3 Bb4:1 E4 D4 Bb3:2 | Bb3:1 B3 D4 D3 D#3 E3:2. -:1 | (3 E3 A3 B3 E4:2 (3 E3:1 \
             A3 B3 F4:2 | (3 E3:1 A3 B3 F#4:2 (3 E3:1 A3 B3 G#4:2 | (3 E3:1 A3 B3 A4:2 (3 E3:1 \
             A3 B3 G4:2 | (3 E3:1 A3 B3 F4:2 (3 E3:1 A3 B3 D#4:2 | (3 E3:1 A3 B3 D4:2 (3 E3:1 A3 \
             B3 E4:2 | (3 E3:1 A3 B3 F#4:2 (3 E3:1 A3 B3 G#4:2",
            "",
        )
        .with_left(
            "E2+B4:1 - E5:2 D5:1 B4 E5:2 | D5 E5 D5 B4 | E5:4 D5:1 -:2 Bb6:1 | Bb4+D6+D7 - \
             G3+G4:2. E3 B2:1 B2 | F2+B4 -:7 | -:8 | - | -:2 G3. E3 B2:1 B2 | E2+E3+B4 E3 -:6 | \
             -:1 E3 -:3 E3:1 -:2 | -:1 E3 -:6 | E3:1 E3 E3:2 E3:1 E3 E3:2 | E3:1 E3 E3:2 E3:1 E3 \
             E3:2 | E3:1 E3 E3:2 E3:1 E3 E3 - & E1:2 -:6 | -:8 | - | (3 -:1 F4+C5 E4+G#4+C#5 (3 \
             D#4+D#5 - D3 D#3:2. - | F1 -:6 | -:8 | - | (3 -:1 A4+B4 - D4:2 -:4 | A3+B3:8 | _ | \
             _ | B3 | _ | _:2 C#4 D4:4 & -:8 | - | - | (3 -:1 A4+B4 - -:6 | -:8 | - | - | (3 -:1 \
             F4+C5 E4+G#4+C#5 (3 D#4+D#5 - D3 D#3:2. - | -:8 | - | - | A3 | _ | _:6 B3+D5:2 & \
             E4:1 E4 E4:2 F4:1 F4 F4:2 | F#4:1 F#4 F#4:2 G4:1 G4 G4:2 | G#4:1 G#4 G#4:2 Bb4:1 \
             Bb4 Bb4 - | -:2 F#3+D4+F#4+D5+E5. -:4 | F4:1 F4 F4:2 F#4:1 F#4 F#4:2 | G4:1 G4 G4:2 \
             G#4:1 G#4 G#4:2 | A4:1 A4 A4:2 Bb4:1 Bb4 Bb4 - | -:2 F#3+F#4+D5+E5:1 -:5 | -:8 | - \
             | - | - | - | - & -:8 | - | - | -:4 E2:1 - B1 - | -:8 | - | - | -:4 E2:1 - B1 - | \
             F1+F2 -:7 | -:8 | - | B4:2 E5 D5:1 C5 B4:2 | D5+E5 E5+A5 D5+G#5:1 E5 C#5+A5:2 | \
             G#5+B5 A5+E6 G#5+D6:1 B5 F#5+E6:2 & -:8 | - | -:7 Bb5:1. | _ A3+A4+B4+B5+B6 \
             A3+A4+F#5+B5+E6+E7 -:5 | -:2 E5 D5:1 B4 E5:2 | D5 E5 D5 B4 | E5:4 D5:1 -:2 \
             Bb5+Bb6:1 | Bb4+D6+D7 A3+A4+B4+B5+B6 A3+G4+A4+F#5+B5+E7 -:5 | -:2 E3+E5 E3+D5:1 \
             E3+B4 E3+E5:2 | E3+D5:1 - E3+E5:2 E3+D5:1 - E3+C#5:2 | E3+B4:1 - E3+E5:2 E3+D5:1 \
             E3+B4 E3+E5 - | -:8 | - | -",
        )
        .in_key(E, Mixolydian)
        .played_by(&[(gm::BASSOON, 0)])
        .with_drums(&[
            (
                Drum::Ride,
                "-:8 | - | - | -:2 x:6 | -:8 | - | - | -:2 x:6 | -:8 | - | - | - | - | \
                          -",
            ),
            (
                Drum::Kick,
                "-:8 | - | - | - | X | - | - | - | X | - | - | - | - | -",
            ),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 357 to 368.
        Part::new(
            "Violins",
            "(3 E5:1 A4 E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 A4 \
             E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 \
             (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 \
             E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 \
             A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 \
             E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 \
             E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 (3 A4 E5 E5 (3 A4 E5 E5 | (3 E5 A4 E5 (3 A4 E5 E5 \
             (3 A4 E5 E5 (3 A4 E5 E5",
            "",
        )
        .with_left(
            "A3:8 | A3 | A3 | A3 | A3 | A3 | A3 | A3 | A3 | A3 | A3 | A3 & -:4 (3 E4:1 - - (3 E4 \
             C#4 - | (3 - - C#4 (3 E4 - - (3 E4 - - (3 E4 C#4 - | (3 D4 - - -:2 (3 F#4:1 D4 \
             F#4+A4 (3 D4+F#4 D4 - | -:2 (3 D4:1 - - (3 D4 F#4 F#4 (3 - D4+F#4 - | -:2 (3 -:1 \
             F#4 F#4 (3 D5 _ - -:2 | - (3 -:1 A4 - (3 C#5 _ A4 -:2 | - (3 -:1 A4 - (3 C#5 _ - \
             -:2 | - (3 -:1 A4 - (3 D5 _ - -:2 | - (3 -:1 F#4 F#4 (3 - F#4 D4+F#4 (3 C#4 D4+F#4 \
             D4 | (3 D4 - C#4+D4+F#4 (3 C#4 D4+F#4 D4+F#4 (3 C#4 E4 - (3 - C#4+E4 - | (3 - - C#4 \
             (3 _ E4 E4 (3 - E4 - (3 - C#4+E4 - | (3 - - E4 (3 - E4 E4 - C#4+F#4 C#4+E4 \
             C#4+D4+E4 & (3 C#4:1 E4 C#4 (3 E4 A4 - (3 C#5 _ A4 -:2 | - (3 -:1 A4 - (3 C#5 _ A4 \
             -:2 | (3 -:1 F#4 - (3 F#4 A4 - (3 D5 _ - (3 - - A4 | (3 - - A4 (3 - A4 - (3 D5 _ - \
             B5:2 | -:4 (3 -:1 F#4 D4+F#4 (3 C#4 D4+F#4 D4 | (3 D4 - C#4+D4+F#4 (3 C#4 D4+F#4 \
             D4+F#4 (3 C#4 E4 - (3 - C#4+E4 - | (3 - - C#4+E4 (3 - E4 E4 (3 - E4 C#4+A4 (3 D4 \
             C#4+E4 C#4 | (3 C#4 - C#4+E4 (3 D4 C#4+E4 C#4+E4 (3 D4 F#4 F#4 (3 - D4+F#4 - | -:2 \
             (3 -:1 A4 - (3 D5 _ - -:2 | - (3 -:1 A4 - (3 C#5 _ A4 -:2 | - (3 -:1 A4 - (3 C#5 _ \
             A4 -:2 | - (3 -:1 A4 - D5 -:3 & -:4 (3 -:1 - C#5 (3 Bb4 A4 E4. | (3 _ _ E4 (3 - B4 \
             - (3 A5 G#5. _ (3 _ - - | (3 - D4 - -:2 (3 -:1 F#5 D5 (3 B4 A4 F#4. | (3 _ _ F#4 (3 \
             - B4 F#5 (3 A5 G#5. _ (3 _ A4 F#4. | (3 _ _ F#4 (3 G#5+G#6 - - (3 A5+B5+B6 F#5 D5 \
             (3 B4+F5+G#6 - - | (3 E6 C#4 Eb4 -:2 (3 -:1 - C#5 (3 B4 A4 E4. | (3 _ _ Eb4 (3 - B4 \
             - (3 - G#5. _ (3 _ - - | (3 - D4 - -:4 (3 -:1 A4 F#4. | (3 _ _ F#4 (3 - B4 F#5 (3 \
             A5+C#6 G#5. _ (3 _ - - | (3 - C#4 - -:4 (3 -:1 A4 E4. | (3 _ _ E4 (3 - B4 - (3 - \
             G#5. _ (3 _ - - | (3 - D4 - -:6 & A4+E5:1 E5 A5 A4 A4 E4+E5+F#5 B4+B5 - | A4+A5 E5 \
             B5 A4 A4 E4+E5 B5 - | A4+B5 E5 - A4 A4 E4+E5 C#6 - | A4+D6 E4+E5 G#5 A4 A4+E5 E4+E5 \
             -:2 | (3 A4:1 D5+E5 F#5 (3 B4+A5 A4+B5 F#6 (3 A6 E5+G#6 D6 (3 B5 G#5 D5 | A4+E5 \
             E4+E5 A5 A4 A4 E4+E5+F#5 B5 - | A4 E5 B5 A4 A4 E4+E5 -:2 | A4+B5:1 E5 - A4 A4 E4+E5 \
             -:2 | A4+B5:1 E4+E5 G#5 A4 A4+E5 E4+E5 -:2 | A4+E5:1 E5 A5 A4 A4 E4+E5+F#5 E5 - | \
             A4 E5 B5 A4+A5 A4 E4+E5 F#5 - | A4+E5 E5 B5 A4 A4 E4+E5 G#5 F#5 & (3 -:1 C#4 E4 (3 \
             - B4 - (3 A5 G#5. _ (3 _ - - | (3 - C#4 - -:2 (3 -:1 - C#5 (3 B4 A4 E4. | (3 _ _ \
             F#4 (3 - B4 F#5 (3 A5 G#5. _ (3 _ - - | (3 - D4 - -:2 (3 -:1 F#5 D5 (3 B4+B6 - - | \
             (3 A5+A6 D4 - (3 - B4 F#5 (3 - G#5. _ (3 _ A4+F#5+F#6 F#4. | (3 _ _ - (3 - B4 - (3 \
             A5 G#5. _ (3 _ - - | A5:2 E6 (3 A5+C#6:1 B5 _ (3 B4+F6 - - | D6:2 E6 (3 A5+D6:1 F#5 \
             D5 (3 B4+C#6 - - | (3 - D4 - -:4 (3 B4:1 A4 F#4. | (3 _ _ E4 -:2 (3 -:1 - C#5 (3 \
             B4+C#6 - - | (3 - C#4 - -:2 (3 -:1 - C#5 (3 B4+A5 A4 E4. | (3 _ _ F#4 -:4 (3 -:1 A4 \
             F#4 & -:8 | - | - | - | - | - | (3 -:1 C#4 - -:2 (3 -:1 - C#5 (3 - A4 E4. | (3 _ _ \
             F#4 (3 - B4 F#5 (3 - G#5. _ (3 _ - - | E6:4 (3 -:1 F#5 D5 F#6:2 | E6 (3 C#6:1 B4+B5 \
             - (3 A5 G#5. _ (3 _ - - | (3 A5 - - (3 D6 - - (3 A5+C#6 B5 _ -:2 | D6 (3 E6:1 B4+D6 \
             F#5 (3 A5+C#6 F#5+G#5 D5 (3 B4+B5 - -",
        )
        .in_key(A, Major)
        .played_by(&[(gm::VIOLIN, 0)])
        .with_drums(&[
            (
                Drum::Ride,
                "-:8 | - | - | -:6 x:2 | - x:4 x:2 | -:8 | - | - | - | - | - | -",
            ),
            (
                Drum::Crash,
                "-:8 | - | - | - | x:4 x | x:8 | - | - | - | - | - | -",
            ),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 369 to 385.
        Part::new(
            "Violins and horns",
            "A5:2 D6 C6:1 B5 D6:2 | A5 D6 C6 B5 | A5 D6 C6 B5 | A5 D6 C6:1 B5 D6:2 | C6 D6 C6:1 \
             B5 D6:2 | A5 D6 C6 B5 | A5 D6 C6:1 B5 D6 - | C6:2 D6 C6 B5:1 - | A5:2 D6 C6 B5:1 - \
             | C6:2 D6 C6 B5:1 - | A5:2 D6 C6:1 B5 D6 - | C6:2 D6 C6 B5 | A5 D6 C6:1 B5 A5:2 | \
             D6:4 B5:2 E6 | A5:1 B5 C6 A5 D6:2 C6:1 B5 | A5 E6 D6 C6 B5 A5 E6 D6 | C6 B5 A5 E6 \
             D6:2 C6:1 B5",
            "",
        )
        .with_left(
            "D4+F4+A4:2 A3+D4+F4+A4:4 A3+D4+F4+A4:2 | _ A3+D4+F4+A4:4 A3+D4+F4+A4:2 | _ \
             A3+D4+F4+A4:4 A3+D4+F4+A4:2 | _ A3+D4+F4+A4:4 A3+D4+F4+A4:2 | _ D4:4 D4:2 | - \
             A3+D4+F4+A4:4 A3+D4+F4+A4:2 | _ D4:4 D4:2 | _ A3+D4+F4+A4:4 A3+D4+F4+A4:2 | _ D4:4 \
             D4:2 | _ D4:4 D4:2 | _ -:4 D4:2 | _ D4:4 D4:2 | _ -:4 D4:2 | _ D4:4 D4:2 | _ -:4 \
             D4:2 | _ D4:4 D4:2 | D4 -:4 D4:2 & -:8 | - | - | - | - | - | - | - | - | - | - | - \
             | -:6 G4+B4+D5:2 | (3 A3+F4+A4+D5:1 - - A3+F4+A4+A6:2 B4+D5 G4+B4+D5 | (3 \
             A3+F4+A4:1 B3 C4 (3 _ D4 E4 B4+D5:2 G4+B4+D5 | - A3+F4+A4 (3 F4+G4:1 G4 A4 (3 _ - \
             C5 | -:2 A3+F4+A4+A6 (3 F4+G4+B6:1 G4 A4 (3 _ - C5 & A3:2. -:6 | -:8 | - | - | -:2 \
             A3+F4+A4 G4+B4+D5 G4+B4+D5 | A3+F4+A4:1 -:7 | -:2 A3+F4+A4 G4+B4+D5 G4+B4+D5 | \
             A3+F4+A4:1 -:7 | -:2 A3+F4+A4 G4+B4+D5 G4+B4+D5 | A3+F4+A4 A3+F4+A4 G4+B4+D5 \
             G4+B4+D5 | (3 A3+F4+A4:1 B3 C4 (3 _ D4 E4 B4+D5:2 G4+B4+D5 | (3 A3+F4+A4+D5:1 - - \
             A3+F4+A4:2 (3 F4+G4:1 G4 A4 (3 _ - C5 | (3 A3+F4+A4+B6 B3+C7 - A3+F4+A4+D7:2 (3 \
             F4+G4+E7:1 G4+D7 A4 (3 _ - C5 | (3 - E5 F5 (3 _ G5 A5 (3 F4+G4 G4 A4 (3 _ - C5 | \
             -:2 A3+F4+A4 (3 F4+G4:1 G4 A4 (3 _ - C5 | (3 A3+F4+A4+D5+B6 - F5 (3 _ - A5 \
             B4+D5+D7:2 G4+B4+D5 | (3 A3+F4+A4:1 B3 C4 (3 _ D4 E4 B4+D5:2 G4+B4+D5 & (3 -:1 B3 \
             C4 (3 _ - E4 (3 - E4 - (3 - C4 B3 | (3 - B3 C4 (3 _ - E4 (3 - E4 - (3 - C4 B3 | (3 \
             - B3 C4 (3 _ - E4 (3 - E4 - (3 - C4 B3 | (3 - B3 C4 (3 _ - E4 (3 - E4 - (3 - C4 B3 \
             | (3 - B3 C4 (3 _ - E4 (3 F4 - A4 (3 _ - C5 | (3 - B3 C4 (3 _ - E4 (3 - E4 - (3 - \
             C4 B3 | (3 - B3 C4 (3 _ - E4 (3 F4 - A4 (3 _ - C5 | (3 D5 E5 F5 (3 _ G5 A5 (3 - G4 \
             - (3 - B4 C5 | (3 - B3 C4 (3 _ - E4 (3 F4 - A4 (3 _ - C5 | (3 D5 E5 F5 (3 _ G5 A5 \
             (3 F4 - A4 (3 _ - C5 | -:4 (3 -:1 E4 D4 (3 - C4 B3 | (3 - B4 C5 (3 _ D5 E5 (3 F5 E5 \
             - (3 - C5 - | B5 C6 -:2 E6:1 D6 C6 B5 | A5:2 A5:1 -:5 | (3 -:1 B4 C5 (3 _ D5 E5 - \
             C5 - A4 | B5 C6 - E6 D6:2 C6:1 B5 | A5:2 A5 B5 - & C#4+E4:1 C#4+E4 C#4+E4 C#4 -:4 | \
             -:8 | - | - | - | - | - | - | - | - | -:2 A3+F4+A4 (3 F4+G4:1 G4 A4 (3 _ - C5 | (3 \
             - E5 F5 (3 _ G5 A5 B4+D5:2 G4+B4+D5 | (3 -:1 - C4 (3 _ D4 E4 B4+D5:2 (3 C7:1 B6 - | \
             A6:2 - B6. - | -:8 | (3 -:1 E5+C7 - (3 D7 G5+E7 - -:2 (3 C7:1 B6 - | A6:2 -:6",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 386.
        Part::new("Slower, oboe", "A5:2 A5 B5:3 -:1", "")
            .with_left(
                "B3:2 B3+D4+F4+A4 A3+Eb4+F#4+B4:3 -:1 & D4:2 - A2+F4+B5:4 & A3+F4+A4+D5+F5:1 - \
             D5+F5:2 Eb5+F#5:3 -:1 & A5:4 -",
            )
            .with_tempo(&[glide(0, 35, 4)])
            .in_key(A, NaturalMinor)
            .played_by(&[(gm::OBOE, 0)])
            .marked(Dynamic::Forte),
        // MIDI bar 387.
        Part::new("Slower", "B4:1 C5 D5:2 E5:1 D5 C5 B4", "")
            .with_left("G3:8 & F4+A4:4 E4+G4")
            .in_key(A, NaturalMinor)
            .played_by(&[(gm::CLARINET, 0)])
            .marked(Dynamic::Piano),
        // MIDI bar 388.
        Part::new("Slower, bar 2", "A4:2 A4 B4 -:4", "")
            .with_left("Eb4+F#4:4 D4+F4 -:2")
            .in_time(FIVE_EIGHT)
            .in_key(A, NaturalMinor)
            .played_by(&[(gm::CLARINET, 0)])
            .marked(Dynamic::Piano),
        // MIDI bars 389 to 390.
        Part::new(
            "Slower, bar 3",
            "B4:1 C5 D5 E5 D5:2 C5:1 B4 | A4:2 A4 B4 -",
            "",
        )
        .with_left("G3:8 | _ & D4+F4:4 Eb4+F#4 | E4+G4 F4+G#4")
        .in_key(A, NaturalMinor)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 391.
        Part::new("Slower, bar 5", "B4:1 C5 D5:2 E5:1 D5 C5 B4 -:2", "")
            .with_left("G3:8 -:2 & F4+A4:4 F4+G#4 -:2")
            .in_time(FIVE_EIGHT)
            .in_key(A, NaturalMinor)
            .played_by(&[(gm::CLARINET, 0)])
            .marked(Dynamic::Piano),
        // MIDI bars 392 to 393.
        Part::new(
            "Slower, bar 6",
            "B4:1 C5 D5 E5 D5:2 C5:1 B4 | A4:2 A4 B4 -",
            "",
        )
        .with_left("G3:8 | _:7 -:1 & F4+A4:4 E4+G#4 | Eb4+G4:2 (3 _:1 _ D4+F#4 _:3 -:1")
        .in_key(A, NaturalMinor)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 394 to 401.
        Part::new(
            "Piano",
            "B5:1 C6 D6:2 E6:1 D6 C6 B5 | A5:2 A5 B5:4 | B5:1 C6 D6 E6 E6 D6 C6 B5 | A5:2 A5 B5 \
             B5 | B5:1 C6 D6:2 E6:1 D6 C6 B5 | A5:2 A5 B5 B5 | B5:1 C6 D6 E6 D6:2 C6:1 B5 | A5:2 \
             A5 B5 B5",
            "",
        )
        .with_left(
            "-:8 | - | - | - | -:4 A4:2 A4:1 - | -:8 | -:4 A4:2 A4:1 - | -:4 A4:2 A4:1 - & -:8 | \
             - | - | - | -:4 G4:2. - | -:8 | -:4 G4:1 -:3 | -:8 & G3+D4+B4+F5+A5:1 \
             A3+E4+C5+G5+B5 B3+F4+D5+A5+C6:2 A3+G4+E5+B5+C6:1 G3+F4+D5+A5+B5 F3+E4+C5+G5+A5 \
             E3+D4+B4+F5+G5 | D3+C4+A4+E5+F5:2 D3+C4+A4+E5+F5 G3+D4+B4+F5+A5:4 | \
             G3+D4+B4+F5+A5:1 A3+E4+C5+G5+B5 B3+F4+D5+A5+C6 C4+G4+E5+B5+C6 A3+G4+E5+B5+C6 \
             G3+F4+D5+A5+B5 F3+E4+C5+G5+A5 E3+D4+B4+F5+G5 | D3+C4+A4+E5+F5:2 D3+C4+A4+E5+F5 \
             G3+D4+B4+F5+A5 G3+D4+B4+F5+A5 | G3+D4+B4+F5+A5:1 A3+E4+C5+G5+B5 B3+F4+D5+A5+C6:2 \
             A3+E5+B5+C6:1 G3+F4+D5+A5+B5 F3+E4+C5+G5+A5 E3+D4+B4+F5+G5 | D3+C4+A4+E5+F5:2 \
             D3+C4+A4+E5+F5 G3+D4+B4+F5+A5 G3+D4+B4+F5+A5 | G3+D4+B4+F5+A5:1 A3+E4+C5+G5+B5 \
             B3+F4+D5+A5+C6 C4+G4+E5+B5+C6 G3+F4+D5+A5+B5:2 F3+E4+C5+G5+A5:1 E3+D4+B4+F5+G5 | \
             D3+C4+A4+E5+F5:2 D3+C4+A4+E5+F5 G3+D4+B4+F5+A5 G3+D4+B4+F5+A5 & -:8 | - | - | - | \
             (3 G5:1 - A4. (3 _ _ - -:4 | -:8 | (3 G5:1 - A4. (3 _ _ - -:4 | (3 G5:1 A5 - -:6",
        )
        .with_tempo(&[tempo(0, 116)])
        .in_key(G, Mixolydian)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 402 to 409.
        Part::new(
            "Reprise",
            "B5:1 C6 D6:2 E6:1 D6 C6 B5 | A5:2 A5 B5:4 | B5:1 C6 D6 E6 D6 C6 B5 A5 | A5:4 B5:2 \
             B5 | B5:1 C6 D6 E6. D6 - C6 B5 | A5 - A5:2. B5:1 - B5:2. | B5:1 C6 D6. E6 D6 C6 B5 \
             A5 | A5 - A5:2. B5:1 - B5 -",
            "",
        )
        .with_left(
            "-:4 (3 -:1 - G4 (3 A4 - - | -:8 | (3 -:1 - G4 (3 A4 - - -:4 | - (3 -:1 - G4 (3 A4 - \
             - | -:8 | (3 -:1 - G4 -:2 (3 -:1 - G4 (3 A4 - - | -:6 (3 A4:1 - - | (3 - - G4 -:2 \
             (3 -:1 - G4 (3 A4 - - & G3+D4+F4+B4+F5:1 A3+E4+G4+C5+G5 B3+F4+A4+D5+A5+A7 - \
             A3+G4+B4+E5+B5 G3+F4+A4+D5+A5 F3+E4+G4+C5+G5 E3+D4+F4+B4+F5 | D3+C4+E4+A4+E5:2 \
             D3+C4+E4+A4+E5 G3+D4+F4+B4+F5:1 -:3 | -:8 | -:2 A7:1 - G3+D4+F4+F5:4 | \
             G3+D4+F4+B4+F5:1 A3+E4+G4+C5+G5 B3+F4+A4+D5+A5 C4+G4+B4+E5+B5 G3+F4+A4+D5+A5:2 \
             F3+E4+G4+C5+G5:1 E3+D4+F4+B4+F5 | D3+C4+E4+A4+E5:2 D3+C4+E4+A4+E5 G3+D4+F4+B4+F5 \
             G3+D4+F4+B4+F5 | G3+D4+F4+B4+F5:1 A3+E4+G4+C5+G5 B3+F4+D5+A7 C4+G4+B4+E5+B5 \
             G3+F4+A4+D5+A5 F3+E4+G4+C5+G5 E3+D4+F4+B4+F5 D3+C4+E4+A4+E5 | D3+C4+E4+A4+E5:2 \
             D3+C4+E4+A4+E5 G3+D4+F4+B4+F5 G3+D4+F4+B4+F5:1 - & -:8 | -:4 A4:1 -:3 | \
             D4+F4+A4+F5:1 E4+B4+G5+B5 F4+C5+C6 G4+B4+D5+B5+D6 G4+B4+C5+C6 F4+B4+A5+B5 \
             E4+G4+A4+G5 D4+F4+G4+F5+G5 | C4+E4+F4+E5+F5:4 A4:1 -:3 | -:8 | -:2 F4:1 - A4 -:3 | \
             -:8 | -:2 F4 -:4 & (3 A3+G6:1 A6 A4 -:2 A5:1 - A5:2 | A3+D6:1 E6 - A6 A3+G5 A4 \
             F5+A5 G5 | A3+A5 A4 A5:2 A3+B5:1 A4+B6 A5+A6 E6 | (3 A3+D6 E6 A4 (3 A6 A5 _ A3+A5 \
             A4 A5:2 | A3+D6:1 A4+E6 - A6 A3+G5 - F5+A5 G5 | A3 -:3 A3:1 A4+B6 A5+A6 E6 | A3+D6 \
             A4+E6 - A6 A3+G6 A4 A5+F6 G6 | A3 E6 - A6 A3+G5 A4+B6 A5+A6 E6 & -:8 | -:4 A5:2 - | \
             -:8 | - | - | -:4 B5:2 - | -:8 | -:6 B5:2",
        )
        .in_key(G, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bars 410 to 416.
        Part::new(
            "Piano again",
            "D6:1 E6 A6 B5 G5 B6 A6 E6 | D6 E6 A5 A6 G5 G4 F5 G5 | A5 B6 A6 E6 D6 E6 A5 A6 | G5 \
             G4 F5 G5 A5 B6 A6 E6 | D6 E6 A6 B5 G5 B6 A6 E6 | D6 E6 A5 A6 G5 G4 F5 G5 | A5 B6 A6 \
             E6 D6 E6 A5 A6",
            "",
        )
        .with_left(
            "G3:1 G4 A4 G4 G3 G4 A4 G4 | G3 G4 A4 G4 G3 - A4 G4 | G3 G4 A4 G4 G3 G4 A4 G4 | G3 - \
             A4 G4 G3 G4 A4 G4 | G3 G4 A4 G4 G3 G4 A4 G4 | G3 G4 A4 G4 G3 - A4 G4 | G3 G4 A4 G4 \
             G3 G4 A4 G4 & G4:1 - G3+D4+F4+B4:2 G4:1 - G3+C4+E4+B4:2 | G4:1 - G3+D4+F4+C5:2 G4 \
             G3+E4+C5 | G4:1 - G3+D4+F4+B4:2 G4:1 - G3+C4+E4+B4:2 | G4 G3+D4+F4+C5 G4:1 - \
             G3+E4+C5:2 | G4:1 - G3+D4+F4+C5:2 G4:1 - G3+E4+C5:2 | G4:1 - G3+D4+F4+C5:2 G4 \
             G3+E4+C5 | G4:1 -:3 G4:1 - G3+E4+C6:2 & -:8 | -:3 A4+D5:1 -:4 | -:8 | - | - | - | \
             -:2 G3+B4+F5+C6 -:4",
        )
        .in_key(G, Mixolydian)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 417 to 419.
        Part::new(
            "Reprise, end",
            "G5:1 -:5 C6:2. | - C6. - C6 | D6:1 E6 C6 C6 B5:2 A5:1 B5",
            "",
        )
        .with_left(
            "-:6 E3+G3+C4+A4+C5:2. | - E3+G3+C4+A4+C5. - E3+G3+C4+A4+C5:1 - | -:8 & G3:1 G4 \
             A4+F5 G4+G5 -:2 C3+A3+E4+G4 | -:6 G2+E5+G5:1 - | G3 G4 A4 G4 G3 G4 A4 G4 & G4:1 - \
             G3+B4+C6:2 -:4 | -:2 E4 - E4 | G4:1 - G3+E4+C6:2 G4:1 - G3+B4+F5+C6:2 & -:6 \
             G2+E5+G5+A5:2. | - G2+E5+G5+A5. - A5. | -:1 G5 A5 C7 -:4",
        )
        .in_key(C, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 420.
        Part::new("Reprise, 1/4", "-:2 C6:1 -", "")
            .with_left("-:2 F3+B3+A4+C5:1 - & -:2 G2+D3+D5+F5+A5:1 - & -:2 G3+D4+B4+C6")
            .in_time(ONE_FOUR)
            .in_key(C, Major)
            .played_by(&[(gm::FLUTE, 0)])
            .marked(Dynamic::Fortissimo),
        // MIDI bars 421 to 432.
        Part::new(
            "Coda",
            "C6:1 D6 C6 G6 D6 E6 C6 C7. | _ - C6:2. - C6 | D6:1 E6 C6 C7 D6 E6 C6 C7 | D6:2 C7 \
             D6 C6:1 C7. | _ - C6:2. - C6 | D6 C7 D6:1 E6 C6:2 | D6 C7 D6:1 E6 C6:2 | D6 C7 D6:1 \
             E6 C6:2 | D6:1 E6 C6:2 D6:1 E6 C6:2 | D6:1 E6 C6:2 D6:1 E6 C6 - | -:2 C6:4 C6:2 | \
             C6:1 -:7",
            "",
        )
        .with_left(
            "-:8 | -:2 F3+B3+A4+C5. - F3+B3+A4+C5. | -:8 | - | -:2 G3+C4+E4+A4+C5. - B3+A4+C5:1 \
             - | -:6 (3 -:1 G3 A3 | (3 G3+G4 - G3+G4 C5:2 (3 G3+G4+B5:1 A3+F4+Bb5 - (3 G3+A5 A3 \
             G3 | (3 G3+A4 - G3+B4 (3 A3+A5 G3+Bb5+G6 A3 -:2 (3 -:1 G5. _ | (3 _ - - F5:2 G4:4 | \
             _:8 | -:2 C4+E4+G4+A4:4 G3+C4+F4+A4:1 - | G3+C4+E4+A4 -:7 & -:8 | - | -:2 G3+B4+F5 \
             G4:1 - G3+E4:2 | G4:1 - G3+E4+C6:2 G4:1 - G3+B4+F5:2 | -:6 G3+D4+B4+G5:2 | - \
             E4+C5+G5+C6 G4:1 - D4+F5:2 | - E4+G5+C6 - D4+B4+F5 | - E4+C6 - D4+B4 | - D4+B4 - \
             D4+B4+F5 | - B4+F5 - B4+F5 | -:6 F3:2 | G2+C3:1 -:7 & -:8 | - | - | - | - | - | - | \
             - | - | (3 G3+D4+C6:1 A3+E4+G5 G3+D4+C5+D5 (3 A3+E4+G5+A5 - A3+E4+G5 (3 \
             G3+D4+C5+D5+B5 A3+E4+E5+G5+Bb5 G3+D4+D5+C6 (3 - - A3+E5 | G3+D5:2 -:6 | -:8 & G3:1 \
             G4+D7 A4+C7 G4 G3 G4 A4 G4 | -:2 C3+A3 - C3+A3 | G3+D6:1 G4 A4 G4+C6 G3+D6 G4 A4 \
             G4+C6 | G3 G4 A4+A6 G4+E6 G3 G4 A4 G4+C6 | -:2 C3+A3 - C3+A3 | -:1 G4 A6 G4+E6 - G4 \
             A4 G4+C6 | -:2 A4+A6:1 G4+E6 - G4 A4 G4+C6 | -:2 A4+A6:1 G4+E6 -:2 A4:1 G4+C6 | -:2 \
             A4:1 G4+C6 -:2 A4:1 C6 | -:2 A4:1 -:3 A4:1 C6 | -:4 C6:1 - C5:2 | C2+C5+E5:1 -:7 & \
             G4:1 - G3+E4+C6:2 G4:1 - G3+B4+F5+C6:2 | - G3+D4+B4+G5 - G3+D4+B4+G5 | G4:1 - C6 \
             -:3 C6:1 - | -:6 C6:1 - | -:6 F3+C4:2. | -:8 | - | - | - | -:6 C6:1 - | -:8 | C6:2 \
             -:6 & -:1 G5 A5 Bb5 - Bb5 A5 G5. | _ - G2+D3+D5+F5+A5:2. - G2+D3+D5+F5+A5. | -:8 | \
             - | -:2 G2+E5+A5. - G2+D5+F5+A5. | (3 -:1 A3+C4+G5+D7 - (3 A3+A5 G3+Bb5+G6 A3 (3 \
             G3+A4+B5 A3+D4+Bb5 G3+F4 (3 A3+A5 G5+C7. _ | (3 _ - - -:2 D6 (3 -:1 G5+C7. _ | (3 _ \
             - - -:2 D6 F5+B5 | D6 - C6+D6 (3 -:1 G5. _ | (3 _ - - (3 - G3+D4+Bb5+C6+C7 - -:2 (3 \
             A3+E5+G5+A5:1 G3+D4+C5+D5+C7 E4. | _:2 D2+D3+C5+G5+A5:4 G1+G2+B4+G5+A5:1 - | -:8 & \
             C6:2 - B5+D6:1 -:3 | -:8 | -:1 G5 A5 Bb5+C7:2 G5:1 A5 Bb5+C7 | _ G5+D7 A5 Bb5+G6 B5 \
             Bb5 A5 G5. | _ -:5 D3+F4:1 - | (3 G3+G4 - G3+E4 A4:2 D6 B4 | (3 -:1 A3+E4+G5+D7 - \
             (3 A3+A5 G3+Bb5+G6 A3 (3 - - G3+A4 D5:2 | (3 -:1 A3+G4+G5+D7 - E5:2 (3 \
             G3+C5+F5+B5:1 A3+G4+C5+Bb5 G3+C5+F5 (3 A3+A5 G3+C7 A3 | (3 A3+C5 G3+G4+G5 A3+C5 (3 \
             G3+G5+A5 A3+D5+Bb5+C7 G3+G5 (3 A3+C5+B5 G3+Bb5 A3+D4 (3 G3+G5+A5 A3+D5+C7 G3 | D6:2 \
             - D6 (3 -:1 G5. _ | _ -:7 | G5+A5:1 -:7",
        )
        .in_key(C, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[(
            Drum::Kick,
            "-:8 | - | - | - | - | - | - | - | - | - | - | X",
        )])
        .marked(Dynamic::Forte),
    ],
    form: &[
        "Russian Dance",
        "Piccolo and celesta",
        "Trumpet",
        "Clarinet",
        "Oboe",
        "Piccolo and celesta",
        "Trumpet again",
        "Clarinet again",
        "Oboe again",
        "Second tune",
        "Second tune, a third up",
        "Second tune, a fourth up",
        "Tutti",
        "Bassoon",
        "Violins",
        "Violins and horns",
        "Slower, oboe",
        "Slower",
        "Slower, bar 2",
        "Slower, bar 3",
        "Slower, bar 5",
        "Slower, bar 6",
        "Piano",
        "Reprise",
        "Piano again",
        "Reprise, end",
        "Reprise, 1/4",
        "Coda",
    ],
    drums_from: 0,
    about: "The Russian Dance, the end of the first tableau of Petrushka (1911, revised 1947): \
            the Charlatan has touched his three puppets, Petrushka, the Ballerina and the Moor, \
            with his flute, and they dance. The tune, B, C, D, E, D, C, B, A, A, B, moves in \
            parallel chords, every voice of the chord moving with it, over a bass on G: G \
            Mixolydian, the major scale with a flat seventh, F natural. Short answering phrases \
            (piccolo with celesta, trumpet, clarinet, oboe) circle E, F sharp and G over a drone \
            on A and D, A Dorian. The second tune moves up a major third and then to a fourth \
            above where it began, each time in Lydian (C, E and F Lydian, the major scale with a \
            raised fourth). The bassoons' broken figure E, D, A, E rises chromatically on top \
            over a drone on E (E Mixolydian); the violins' oscillating triplets ring A major; a \
            softer tune in D Dorian follows, and a slow, almost stopped phrase in A minor, with \
            bars of 5/8, before the piano brings the dance back with the tune in parallel chords \
            and a glittering figure of its own over a G pedal. At the end the same pedal on G \
            turns out to be the dominant of C: the tune is cut off by loud chords over D and \
            over G, and the dance lands, fff, on C major with an added sixth (C, E, G, A), so \
            the last parts are in C major.",
};

// Copyright: this is Stravinsky's 1947 revision of Petrushka, which is still under
// copyright (Boosey & Hawkes); it is added here at the request of the project's owner.
// Source: Stravinsky's Petrushka, 1947 revision, second tableau, "Petrushka's Room", MIDI bars
// 434 to 561: from the drum roll that links it to the first tableau up to Petrushka's cry; MIDI
// bar 433, a silent bar between the two tableaux, is left out. The notes are converted by
// script from a MIDI file of the 1947 version (the person's own copy, at sounding pitch) and
// checked bar by bar against that file, not against a printed score; MIDI bar numbers are given
// below (MIDI bar 1 is a silent lead-in). The melody is the line in front, in the octave of the
// instrument that has it, and played_by follows it from instrument to instrument. Everything
// else that matters is written out on the Keys channel: the harmony, the bass, the ostinati and
// the timpani; lines doubled in unison or octaves are written once. Rhythms are quantised to
// sixteenths and eighth-note triplets; tremolos and rolls are written as held notes, grace
// notes are left out. The dynamic of each part is read from the MIDI's velocities. The
// percussion is written out (triangle as Ride, tambourine as Shaker, bass drum as Kick, side
// drum as Snare, cymbals as Crash). Where the meter changes, every run of bars in one meter is
// a part of its own, named "..., bar N" after the bar it starts on, with in_time: in_meters
// cannot cut a written-out left hand of several voices into its runs, nor carry drums. No chord
// symbol is honest for this music, which moves in drones, ostinati and parallel chords, so the
// charts are empty. Unlike the first tableau, a tremolo between two chords (32nd notes taking
// turns) is written as the two chords taking turns in sixteenths, and a run of 32nd notes keeps
// every other note. Where a part starts on a chord held over from the bar before, the chord is
// struck again.
//
// Left out: in the fortissimo tremolos (MIDI bars 480 to 488 and 552 to 558) the woodwinds,
// horns and strings, which play the piano's two chords, and the flutes' and clarinets' trills;
// the Keys channel keeps the piano's chords, the bass and the trombone. Grace notes are left
// out throughout.
//
// Not certain: the meters are the MIDI's: 4/8 in bars 491 to 495 and 497 to 502 (kept as 4/8,
// not rewritten as 2/4), 3/8 in bar 496, 1/4 in bars 538 and 540, 3/2 in bars 559 and 560, 4/4
// in bars 532 to 534 and 536. Bar 454 is silent in the MIDI but for the piano's chord held over
// from bar 453, which this part strikes again. The tempos are the MIDI's, with two exceptions:
// in bar 494 the MIDI slows from 40 to 15 within the first beat and comes back to 40, written
// here as half a beat at 30 (the slowest the demos allow at a tempo of 100); in bar 536 the
// MIDI's ritardando from 100 to 28 stops at 30. The piano's runs of quintuplets and sextuplets
// (bars 475 to 479) and the quintuplets of the oboes in bars 440 and 441 are written as the
// nearest triplets or sixteenths. The keys: the bars built on Petrushka's chord keep to the
// octatonic scale on C (see the song's description), which a key of the chord sheet cannot
// name, so they are marked chromatic like the bars that keep to no scale at all; the rest have
// the mode their tune is in.
pub(super) const PETRUSHKA_ROOM: Song = Song {
    name: "Petrushka, Petrushka's Room",
    origin: "Igor Stravinsky, Petrushka, 1911, revised 1947",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: C,
    mode: Chromatic,
    time: TWO_FOUR,
    tempo: 100.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FLUTE, gm::ACOUSTIC_GRAND_PIANO, gm::CONTRABASS),
    style: "",
    parts: &[
        // MIDI bars 434 to 438.
        Part::new(
            "Drum roll",
            "-:8 | - | - | -:4 (3 -:2 - C5 | (3 F#5 F#5 - -:4",
            "",
        )
        .with_left(
            "F#3:1 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 \
             F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | -:8 & -:8 | - | - | -:4 (3 \
             -:2 - B4 | (3 E5 E5 - -:4",
        )
        .with_tempo(&[tempo(0, 116)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .with_drums(&[
            (Drum::Snare, "X:1 X X X X X X X | X X X X X X X X | X X X X X X X X | X X X X X X X \
                           X | -:8"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 439 to 441.
        Part::new(
            "Oboes",
            "-:4 (3 -:2 - C5 | F#5 C5:1 C5:2 F#5:1 F#5:2 | F#5 F#5:1 C5:2 F#5:1 F#5:2",
            "",
        )
        .with_left(
            "-:4 (3 -:2 - B4 | E5 B4:1 B4:2 E5:1 E5:2 | E5 E5:1 B4:2 E5:1 E5:2",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 442 to 446.
        Part::new(
            "Drum roll again",
            "-:8 | - | - | - | -",
            "",
        )
        .with_left(
            "F#3:1 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 \
             F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 F#3 | F#3 F#3 F#3 F#3 F#3 F#3 F#3 \
             F#3",
        )
        .in_key(C, Chromatic)
        .with_drums(&[
            (Drum::Snare, "X:1 X X X X X X X | X X X X X X X X | X X X X X X X X | X X X X X X X \
                           X | X X X X X X (3 X _ x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 447.
        Part::new(
            "Petrushka's Room",
            "(3 G7:2 C7 G7 (3 F#7:1 E7 Eb7 -:2",
            "",
        )
        .with_left(
            "F#3:1 -:7 & (3 C4+G5:2 G4+C5+C6 C5+G5 (3 F#4+F#5 Eb5 - & (3 C5+G6:2 F4 C4+G6 (3 -:1 \
             E4 Eb4+Eb7 (3 _ - -",
        )
        .with_tempo(&[tempo(0, 100)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "X:8"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 448.
        Part::new(
            "Muted trumpet",
            "Eb5:8",
            "",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 449 to 450.
        Part::new(
            "Flute",
            "-:6 (3 F#5:1 F5 E5 | Eb5:8",
            "",
        )
        .with_left(
            "-:6 D4+G#4+B4:2 | A4+C#5:8 & Eb5:7 -:1 | -:8",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Pianissimo),
        // MIDI bars 451 to 452.
        Part::new(
            "Violins",
            "-:6 Eb5:1 D5 | B4:2 (3 B4:1 A4 G#4 G4 -:3",
            "",
        )
        .with_left(
            "-:8 | -:6 F#1:2 & -:8 | F#3+Eb4:4 F#1+E3+G3+C4+E4:1 -:3 & -:6 E3+Bb3+C#4+E4+Bb4:1 \
             F3+B3+D4+F4+A4 | C4+F#4:2 (3 C4+F#4:1 C#4 C4+D4 -:4",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bar 453.
        Part::new(
            "Piano chord",
            "G6:1 E6:7",
            "",
        )
        .with_left(
            "Eb4+G5+C#6+Eb6:1 E4+C5+E5+C6:7",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .with_drums(&[
            (Drum::Crash, "x:8"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 454.
        Part::new(
            "Piano chord, bar 2",
            "G6:12",
            "",
        )
        .with_left(
            "E4+G5+C6+E6:12",
        )
        .in_time(THREE_FOUR)
        .with_tempo(&[tempo(5, 50)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 455 to 462.
        Part::new(
            "Petrushka's chord",
            "-:6 (3 -:1 C4 E4 | C5:8 | _:1 G4 C5 G4 C5:4 | _:6 C5:1 G4 | C5:8 | _ | _:5 -:3 | \
             -:8",
            "",
        )
        .with_left(
            "-:6 (3 -:1 Bb3 C#4 | Bb4:8 | _:1 F#4 Bb4 F#4 Bb4:4 | _:6 Bb4:1 - | Bb4:8 | _ | _:5 \
             -:3 | - G#4 (3 _:1 - - & -:8 | - | -:4 G#4:1 F#4:3 | G#4:1 F#4:7 | -:4 G#4:1 F#4:3 \
             | _:8 | G#4:1 F#4:5 -:2 | -:6 (3 -:1 F#4 Eb4",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 463 to 464.
        Part::new(
            "Trumpet",
            "A3:2 C#4:5 G#4:1 | G4:3 G#4:1 G4:3 G#4:1",
            "",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 465 to 466.
        Part::new(
            "Clarinet",
            "(3 C#4:1 B4 Bb4 G5:4 E5:1 G4 | (3 G5 E5 C5 (3 G4 - - -:4",
            "",
        )
        .with_left(
            "(3 -:2 C#4 _ _ Bb3:1 C#3 | (3 C#4 Bb3 F#3 (3 C#3 - - -:4 & G4:3 -:5 | (3 -:2 C#4+G4 \
             _ _:4 & (3 -:1 F#4 D5 C#5:2. - (3 -:1 C#4+G4 F#4+B4 | C#5+G5 -:7",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bar 467.
        Part::new(
            "Piano run",
            "-:4 (3 A4:1 D5 Bb4 (3 Eb4 A4 G#4",
            "",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)]),
        // MIDI bars 468 to 472.
        Part::new(
            "Clarinet again",
            "-:4 (3 A4:1 B4 G#4 (3 C#5 G4. _ | _ -:3 (3 G4:1 B4 D5 (3 C#5 - - | (3 G4 B4 D5 (3 \
             C#5 - - (3 G4 B4 D5 (3 C#5 - - | (3 G5 D5 B4 (3 C#5 - - (3 G5 D5 B4 (3 C#5 - - | (3 \
             G5 D5 B4 (3 C#5 - - (3 G5 E5 C5 (3 C#5 - -",
            "",
        )
        .with_left(
            "-:8 | -:6 (3 C#4:1 Bb3 F#3 | (3 G3 - - (3 C#4 Bb3 F#3 (3 G3 - - (3 C#4 Bb3 F#3 | (3 \
             C#4 - - (3 C#4 Bb3 F#3 (3 C#4 - - (3 C#4 Bb3 F#3 | (3 C#4 - - (3 C#4 Bb3 F#3 (3 C#4 \
             - - (3 C#4 Bb3 F#3 & -:8 | -:4 C#4:1 F#4 C#5 E5 | C#4 F#4 C#5 E5 G4 F#4 C#5 E5 | (3 \
             C#5 Bb4 F#4 (3 G5 E5 C5 (3 G4 C#5 F#4 (3 G5 E5 C5 | (3 C#5 Bb4 F#4 (3 G5 E5 C5 (3 \
             G4 C#5 F#4 (3 G5 E5 C5",
        )
        .with_tempo(&[tempo(8, 76)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 473 to 479.
        Part::new(
            "Piano cadenza",
            "F#4:1 G4 B4 D5 G5 C5 E5 G5 | C#5 E5 G5 C6 E6 G5 B5 D6 | (3 F#6 Bb6 D6 (3 G5 C#6 C6 \
             (3 B5 C#6 D6 (3 G5 C#6 C6 | (3 F#6 Bb6 D6 (3 G5 C#6 C6 (3 B5 F#6 D6 (3 G5 C#6 A5 | \
             (3 F#6 G6 D6 (3 F6 B5 _ _:2 (3 C#6:1 A5 F6 | F5:4 (3 G#5:1 A5 D5 (3 B4 F5 _ | (3 \
             F#5 G5 C#5 (3 Eb5 C#5 C#5 (3 G#4 Bb4 G#4 (3 G#4 Eb4 F#4",
            "",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 480 to 484.
        Part::new(
            "Petrushka's curses",
            "G5:4 (3 _:2 E5 C5 | G4:8 | G5:4 (3 _:2 E5 C5 | G4:8 | G5:4 (3 G5:2 E5 C5",
            "",
        )
        .with_left(
            "E4:1 Bb3 E4 Bb3 E4 Bb3 E4 Bb3 | E4 Bb3 E4 Bb3 E4 Bb3 E4 Bb3 | E4 Bb3 E4 Bb3 E4 Bb3 \
             E4 Bb3 | E4 Bb3 E4 Bb3 E4 Bb3 E4 Bb3 | E4 Bb3 E4 Bb3 E4 Bb3 E4 Bb3 & Bb3+C#4+F#4:1 \
             E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 | Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 | Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 | Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 | Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 & C4:8 | _ | _ | _ | _",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x | x x x x x x x x | x x x x x x x x | x x x x x x x \
                           x | x x x x x x x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 485.
        Part::new(
            "Petrushka's curses, bar 6",
            "Bb4:4 (3 C5:2 C#5 F#5 (3 E5 C5 G4",
            "",
        )
        .with_left(
            "-:1 Bb3 - Bb3 - Bb3 E4 Bb3 E4 Bb3 E4 Bb3 & Bb3+C#4+F#4:1 E4+G4 Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 & C4:12",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x x x x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 486 to 488.
        Part::new(
            "Petrushka's curses, bar 7",
            "Bb4:2 -:1 C4 C4:2 F4 | (3 Eb4 C4 F#3 A3:4 | -:8",
            "",
        )
        .with_left(
            "E4:1 Bb3 E4 Bb3 E4 Bb3 E4 Bb3 | E4 Bb3 E4 Bb3 E4 Bb3 E4 Bb3 | E4 Bb3 E4 Bb3 E4 \
             Bb3+D4 D4 Bb3+D4 & Bb3+C#4+F#4:1 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 | Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 | Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 Bb3+C#4+F#4 E4+G4 \
             Bb3+C#4+F#4 E4+G4 & C4:8 | _ | _",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x | x x x x x x x x | x x x x x x X X"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 489 to 490.
        Part::new(
            "Cry",
            "F#4:2 -:6 | (3 D4:1 F#4 G#4 (3 A4 C#5 D5 (3 F#5 G#5 A5 (3 C#6 D6 F#6",
            "",
        )
        .with_left(
            "D2+G3+C4:1 -:7 | -:8 & A3+D4+F4:1 -:7 | -:8 & A2+E4:1 D4 D4 D4 D4 -:3 | -:8",
        )
        .in_key(D, Lydian)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .with_drums(&[
            (Drum::Snare, "X:8 | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 491 to 494.
        Part::new(
            "Piccolo",
            "G#5:4 G#5 | C#6:1 -:2 B5:1 F#6:4 | -:2 B5 - B5 | G#6:1 -:7",
            "",
        )
        .with_left(
            "D4+A4:4 D4+A4 | D4+A4 D4+A4 | D4+A4 D4+A4 | D4+A4:1 -:7 & -:2 D5+F#5 - D5+F#5 | - \
             D5+F#5 - D5+F#5 | - D5+F#5 - D5+F#5 | -:8 & G#6:4 G#6 | C#7:1 G#6 G#6 B6 -:4 | \
             C#6:1 F#6 -:2 F#6:1 B5 -:2 | (3 G#6:1 _ B6 (3 C#7 E6+G#6 G#5+B5+C#6 B4+D5+E5 -:3",
        )
        .in_time(FOUR_EIGHT)
        .with_tempo(&[tempo(0, 40), tempo(24, 30), tempo(26, 40)])
        .in_key(D, Lydian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 495.
        Part::new(
            "Oboe",
            "-:4 (3 Bb4:1 C5 A4 (3 G#4 - -",
            "",
        )
        .with_left(
            "-:4 (3 -:1 A4 G#4 _:2 & (3 G#5:1 C#6 A5 (3 D5 G#5 G5 F#5 -:3",
        )
        .in_time(FOUR_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::OBOE, 0)]),
        // MIDI bar 496.
        Part::new(
            "Clarinet arpeggio",
            "(3 G#4:1 B4 E5 (3 E5 G#4 E5 (3 E5 D5 G#5",
            "",
        )
        .with_left(
            "-:4 G#4:1 A4 & (3 D5:1 F#5 B5 (3 D5 D5 B5 (3 D5 E5 -",
        )
        .in_time(THREE_EIGHT)
        .in_key(D, Lydian)
        .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bars 497 to 502.
        Part::new(
            "Flute tune",
            "F#5:2 G#5 F#5 C#5 | D5 G#5 (3 F#5:1 D5 _ (3 C#5 E5 E5 | G5:2 A5 G5 D5 | E5 A5 G5 D5 \
             | F#5 B5 A5 E5 | G5 C#6 B5 F#5:1 -",
            "",
        )
        .with_left(
            "D4+A4+G#5+G#6:2 B4+D5+G#6 D5+G#5+C#6+G#6 B4+D5 | D4+A4+C#6+C#7:1 G#5+G#6 B4+D5+G#6 \
             B5+B6 D5+C#6+F#6:2 B4+D5 | D4+A4+C#6:1 F#6 G4+D5+B5:2 D5+A5+F#6:1 B5 G4+B5:2 | \
             D4+A4+C#6:1 F#6 (3 G4+D5+F#6 B6 F#6 C#6 B5 G4+B5:2 | (3 E4+B4+F#6:1 A6 B6 (3 A6 \
             G4+D5+F#6 B5 F#6 B5 D5+A5+B5:2 | (3 F#4+C#5+F#6:1 A6 B6 (3 B6 A4+E5+F#6 B5 F#6 B5 \
             E5+B5:2 & D6:2 -:6 | -:8 | - | - | - | -",
        )
        .in_time(FOUR_EIGHT)
        .in_key(D, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 503 to 504.
        Part::new(
            "Cor anglais",
            "F#4:2 F#4 G4:4 | E4:2 E4 F#4:4",
            "",
        )
        .with_left(
            "E2:4 E2 | E2 E2 & A2+A5:1 A2 E3:2 B2:1 B2 E3:2 | C3:1 C3 E3:2 B2:1 B2 E3 - & \
             E1+C5:1 -:3 E1+D5:1 - F#5:2. | E1+D5+E5:1 -:3 E1+D5:1 -:3 & F#6:1 E3 A3+F#5 -:2 \
             E3:1 A3 - | E6 E3 A3+E5 -:2 E3:1 A3 E3 & B4:2. A5:1 -:3 G5:1 - | -:2 F#5:1 -:3 \
             F#5+G5:1 -",
        )
        .in_key(E, NaturalMinor)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .with_drums(&[
            (Drum::Snare, "x:4 x | x x"),
            (Drum::Crash, "x:4 x | x x"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bar 505.
        Part::new(
            "Cor anglais, bar 3",
            "F#4:1 G4 F#4:2 B3 B3 B3:1 C4 D4 E4",
            "",
        )
        .with_left(
            "E2:4 E2 E2 & A2:1 A2 E3:2 B2:1 B2 E3:2 C3:1 C3 E3 - & E1+C5:1 -:3 E1+D5:1 -:3 \
             E1+D5+E5:1 -:3 & -:1 E3+G5 A3+F#5 - B4 E3 A3 - B4 E3+C6 A3+D5 E3+E6 & B4:2. A5:1 \
             -:3 F#5+G5:1 -:3 F#5:1 -",
        )
        .in_time(THREE_FOUR)
        .in_key(E, NaturalMinor)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .with_drums(&[
            (Drum::Snare, "x:4 x x"),
            (Drum::Crash, "x:4 x x"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bars 506 to 507.
        Part::new(
            "Cor anglais, bar 4",
            "F#4:2 F#4 F#4:1 G4 F#4:2 | E4 E4 F#4:4",
            "",
        )
        .with_left(
            "B2:1 B2 E3:2 E2:4 | G2:1 G2 E3:2 E2:4 & E2:4 A2:1 A2 E3:2 | E2:4 A2:1 A2 E3 - & -:6 \
             D5:2. | E1+A4+B4:1 -:3 E1+A4:1 -:3 & E1+G4:1 - E5 - E1+A4 - E5 - | -:2 C5:1 -:3 \
             C5+D5:1 - & F#6:1 E3 A3+F#5 - F#5 E3+G5 A3 - | E6 E3 A3+E5 -:2 E3:1 A3 E3",
        )
        .in_key(E, NaturalMinor)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .with_drums(&[
            (Drum::Snare, "x:4 x | x x"),
            (Drum::Crash, "x:4 x | x x"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bar 508.
        Part::new(
            "Cor anglais, bar 6",
            "F#4:1 G4 F#4:2 B3 B3 B3:4",
            "",
        )
        .with_left(
            "B2:1 B2 E3:2 E2:4 E2 & E2:4 A2:1 A2 E3 G2 G2 E3 -:2 & E1:1 - G4+E5 - E1 - A4+C5+D5 \
             - E1 - A4+B4+C5 - & F#6:1 E3+G5 A3 - B4 E3 A3+B5 E3 C5+B5 - A3 E3",
        )
        .in_time(THREE_FOUR)
        .in_key(E, NaturalMinor)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .with_drums(&[
            (Drum::Snare, "x:4 x x"),
            (Drum::Crash, "x:4 x x"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bars 509 to 514.
        Part::new(
            "Flute and bassoon",
            "F#5:2 F#5 A5:1 G5 F#5 E5 | E5:2 E5 F#5:1 G5 F#5:2 | F#5 G5 B5:1 F#5 B5 F#5 | B4:2 \
             B4 B4:4 | E5:2 E5 E5:1 G5 F#5:2 | G5:1 E5 F#5 G5 E5 E6 G5:2",
            "",
        )
        .with_left(
            "D#4:2 D4:6 | D4:8 | D#4 | _:4 D4 | F#4:2 G4 B4 A4 | Bb4:8 & -:8 | - | -:3 B4:2 -:3 \
             | -:8 | C#4 | C4:2 C#4 F#4 D#4 & -:1 A4 Bb4 Bb4:2 Bb4:1 B4 B4 | _ B4 C5 C5:2 C5:1 \
             C#5 C#5 | _ C5 E5 D5 D5 C#5 C#5 C5 | A4+B6 G#4+F#6 G#4+B6 G4+F#6 G4+B5 F#4 F#4+B5 \
             F4 | B5:4 - | -:8 & (3 -:1 A4 G#5 (3 A6 Bb6 - (3 - - A5 (3 Bb6 B6 - | (3 - - Bb5 (3 \
             B6 C7 - (3 - - B5 (3 C7 C#7 _ | (3 B6 C7 C6 (3 B5 B6 _ (3 A5 Bb5 Bb4 (3 A5 A4 _ | \
             (3 G6 G#6 G#5 (3 G5 G6 _ (3 F5 F#5 - (3 F5 F4 - | (3 E4 E5 D#5 (3 D#6 E5 F#5 (3 D#6 \
             F#6 E6 (3 E5 E5 D#5 | (3 E5 E4 D#5 (3 D#6 E5 D#6 E7 -:3",
        )
        .in_key(E, NaturalMinor)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 515.
        Part::new(
            "Piano alone",
            "E5:2 E5 E5 C#4:1 G#5",
            "",
        )
        .with_left(
            "E4+B4:2 Eb4 D4 -",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 516.
        Part::new(
            "Piano alone, bar 2",
            "F#5:1 E5 F#5 G#5 E5:2 E5 E5 G4",
            "",
        )
        .with_left(
            "-:2 B4:1 - Bb4:2 A4 G#4 -",
        )
        .in_time(THREE_FOUR)
        .with_tempo(&[tempo(0, 36), tempo(10, 100)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 517.
        Part::new(
            "Oboe figure",
            "E5:2 E5 E5:1 -:3",
            "",
        )
        .with_left(
            "-:4 D4:2 -:1 G4 & C3+G3+E4+C5:1 B4+D5 C3+G3+E4+A4+C5+D6+E7 G4+B4 C3+G3+E4+F#4+A4 \
             G4+B4 A4+C5 B4+D5 & -:2 (3 -:1 E6 E5 -:4",
        )
        .in_key(E, NaturalMinor)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 518.
        Part::new(
            "Trumpet answer",
            "F#4:1 G4 F#4:2 F#4:1 E4 F#4 -:5",
            "",
        )
        .with_left(
            "-:8 E3:1 -:3 & -:6 (3 -:1 - D3 -:4 & (3 C3+C4+E4:1 G3+B3+D4 C4 (3 A3+C4+G4 C5 \
             G3+B3+G5 (3 F#3+A3+A5 G3+B3+D5 A4 (3 A3+C4+D4 A3 B3+D4. -:2 D4+B5:1 - & -:3 G4:1 A4 \
             D4 -:2 (3 B3:1 A4 E4+F#4+B4 A4:2 & F#4:3 B4:1 F#4:3 B4:1 -:4 & C5:2. -:6 (3 \
             D4+E4+B4+D5+E5:1 B3+D5+D#5+A5 A3+A4+E5+F#5+B5 (3 D3+F#4+D5+A5+E6 - - & E5:1 -:11",
        )
        .in_time(THREE_FOUR)
        .in_key(E, NaturalMinor)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "-:9 x:1 x:2"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 521.
        Part::new(
            "Oboe figure 3",
            "-:4 E5:2 E5 E5:1 -:3",
            "",
        )
        .with_left(
            "-:8 C#4:2 - & -:10 (3 C#4:1 _ - & A4:1 D4 -:10 & -:4 C3+G3+C4+E4+C#5:1 C#4+B4 \
             C3+G3+D4+E4+Bb4+D6+E7 Eb4+A4 C#3+G#3+E4+G#4 Eb4+A4 D4+Bb4 C#4+E4+B4+G#6 & (3 \
             F#3+A3+A5:1 G3+B3+D5 A4 (3 A3+C4+D4 A3 D3+B3+D4 -:2 (3 -:1 E6 E5 (3 - - C#6 B5+E6:2 \
             & -:10 (3 E4+C#5+E5:1 - -",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 522.
        Part::new(
            "Piccolo cry",
            "F#6:1 -:2 G#6:1 F6:3 G#6:1 -:4",
            "",
        )
        .with_left(
            "-:7 D3:1 -:4 & C#3+C4+F#4+C5+F#5:1 G#3+B3+C#4+B4+E5 C#3+Bb3+D4+Bb4+F#5 \
             G#3+Eb4+E4+A4+G#5 D3+Bb4+F5 G#3+A3+E4+G#4+E5 D3+A3+Eb4+A4+F5 A3+Bb3+D4+Bb4 -:4 & \
             G#3:1 E4 C#4+G#4 - A3 - D4+G4 A4+G5 C4 F4 A5 - & -:2 F#4:1 G#4+G5 A4 D4 F4:2. -:3 \
             D3+D4+A4:1 & -:7 G#4:2. -:3 & -:7 D5:1 E3+E4+B4+E5:2 F3+E4+F4+C5+E5:1 -",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 523.
        Part::new(
            "Oboe figure 4",
            "E5:2 E5 E5:1 -:3",
            "",
        )
        .with_left(
            "C3+G3+C4+E4+C5:1 C#4+B4 C3+G3+D4+E4+Bb4 Eb4+A4 C#3+G#3+E4+G#4 Eb4+A4 D4+Bb4 \
             C#4+E4+B4+G#6 & -:4 C#4:2 - & -:6 (3 C#4:1 _ - & -:6 (3 E5:1 - - & C6+E6:2 (3 D6:1 \
             E6 E5 (3 - - C#6 B5+E6:2",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 526 to 528.
        Part::new(
            "Piccolo cry 3",
            "F#6:1 -:2 G#6:1 F6:3 G#6:1 | F#6 -:2 G#6:1 F6:3 G#6:1 | F#6 -:2 G#6:1 F6:3 G#6:1",
            "",
        )
        .with_left(
            "G#3:1 E4 C#4+G#4 - A3 - D4+G#4 A4 | G#3+C#4 G#4 G#4+C#5 G#4 A3 A4 G#4+D5 A4 | \
             G#3+C#4 G#4 G#4+C#5 G#4 A3 A4 G#4+D5 A4+G#5 & -:2 C4+F#4:1 G#4 -:2 F4:1 G#4 | - E4 \
             C4+F#4 -:3 D4+F4:1 G#4 | - E4 C4+F#4 -:3 D4+F4:1 G#4 & C#3+C4+F#4+C5+F#5:1 \
             G#3+B3+C#4+B4+E5 C#3+Bb3+D4+Bb4+F#5 G#3+Eb4+E4+A4+G#5 D3+Bb4+F5 G#3+A3+E4+G#4+E5 \
             D3+A3+Eb4+A4+F5 A3+Bb3+D4+E4+Bb4+G#5 | C#3+C4+F#4+C5+F#5 G#3+C#4+B4+E6 \
             C#3+D4+Bb4+F#6 G#3+Eb4+E4+A4+G#5 D3+Bb4+F5 G#3+A3+E6 D3+A3+Eb4+A4+F5+F6 \
             A3+Bb3+D4+E4+Bb4+G#5 | C#3+C4+F#4+C5+F#5 G#3+C#4+B4+E6 C#3+D4+Bb4+F#6 \
             G#3+Eb4+E4+A4+G#5 D3+Bb4+F5 G#3+A3+E6 D3+Eb4+A4+F6 A3+Bb3+D4+Bb4",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 529 to 531.
        Part::new(
            "C major",
            "E6:1 F6 G6 A6 B6 A6 G6 F6 | E6 F6 G6 A6 B6 A6 G6 F6 | E6 F6 G6 A6 B6 C7 D7 E7",
            "",
        )
        .with_left(
            "-:4 A3 | - D3 | - D3 & -:8 | -:4 A3:1 -:3 | -:4 F3+A3:1 -:3 & -:4 C4+E4+G4:2. - | \
             -:4 C4+E4+G4:2. - | -:8 & -:1 E4 -:2 B5. B4:1 E5 | - E4 B4 - B5:2. B4:1 E5 | - E4 \
             B4 - B5 E4 B4 - & (3 C4+E4+G4+B4:1 D4+G4+A4+B4+C5 C4+G4+B4+C5+E5 (3 D4+A4 \
             C4+B4+E5+G5 D4+C5+E5+G5+B5 D4+F5+A5+C6:2. - | (3 C4+E4+G4+B4:1 D4+G4+A4+B4+C5 \
             C4+G4+B4+C5+E5 (3 D4+A4+C5+E5+G5 C4+G4+E5+G5+B5 D4+A4+G5+B5+C6 D4+F5+A5+C6+E6:2. - \
             | (3 C4+Eb4+E4+G4+B4:1 D4+F4+G4+A4+B4+C5 C4+G4+B4+C5+E5 (3 D4+G4+A4+C5+E5+G5 \
             C4+G4+E5+G5+B5 D4+A4+G5+B5+C6 C4+D5+F5+A5+E6 C5 D5 E5 & E5:1 F4+F5 G4 A4+A5 B4 A4 \
             G4+G5 F4+F5 | E5 F4+F5 - A5 B4 A4 G4+G5 F4+F5 | E5 F5 - A5 - C6 D6 E6",
        )
        .in_key(C, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x:4 | x:1 x x x x:4 | x:1 x x x X:3 x:1"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 532 to 534.
        Part::new(
            "Clarinet cadenza",
            "(3 Bb5:2 _ Bb5 (3 _ _ Bb5 (3 _ _ Bb5 (3 C#6:1 Bb5 G5 (3 E5 Bb5 C#5 | B4:4 (3 C#5:2 \
             E5 D5 B4:4 (3 _:1 C#5 E5 (3 G5 F5 D5 | B4 C#5 G5 G#5 D5:2. - C#5 E5:1 G5:2 F5:1 \
             B5:2",
            "",
        )
        .with_left(
            "(3 F3+G#3+B3:2 - - -:12 | -:16 | - & D3:1 -:15 | -:16 | - & (3 D4+F4+G#4:2 _ \
             D4+F4+G#4+B4 (3 _ _ D4+F4+G#4+B4 (3 _ _ D4+F4+G#4+B4 _:4 | _:2 -:14 | -:16 & (3 \
             B4:2 _ - -:12 | -:16 | - & (3 G4+D5+B5+Bb6+F7:2 - Bb6. (3 _ - Bb6. (3 _ - Bb6 -:4 | \
             -:16 | -",
        )
        .in_time(COMMON)
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .with_drums(&[
            (Drum::Snare, "X:16 | - | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 535.
        Part::new(
            "Clarinet cadenza, bar 4",
            "(3 E6:2 C#6 C#6 E6:1 C#6 -:6",
            "",
        )
        .with_left(
            "-:8 (3 B4:1 D5 F5 (3 Bb5 B5 F6",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 536.
        Part::new(
            "Piano",
            "C#7:1 C#7 C#7 C#7:2 C#7:1 C#7 C#7 (3 Bb6 G#6 F6 (3 G#6 D6 Bb5 F5 -:3",
            "",
        )
        .with_left(
            "-:12 (3 B4:1 B4 B4 (3 _ B4 B4",
        )
        .in_time(COMMON)
        .with_tempo(&[glide(10, 30, 3), tempo(15, 100)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 537.
        Part::new(
            "Piano, bar 2",
            "(3 C#5:1 F#5 D5 (3 _ G5 G4 (3 C#5 Eb5 C5 (3 _ F5 B4",
            "",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)]),
        // MIDI bar 538.
        Part::new(
            "Cor anglais, 1/4",
            "B4:2 B4",
            "",
        )
        .in_time(ONE_FOUR)
        .with_tempo(&[tempo(0, 50)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bar 539.
        Part::new(
            "Piano, 3/4",
            "(3 C#5:1 F#5 D5 (3 _ G5 G4 (3 F#5 G#5 D5 (3 _ A5 A4 (3 C#5 Eb5 C5 (3 _ F5 B4",
            "",
        )
        .in_time(THREE_FOUR)
        .with_tempo(&[tempo(0, 100)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)]),
        // MIDI bar 540.
        Part::new(
            "Cor anglais, 1/4 again",
            "(3 B4:2 B4 B4",
            "",
        )
        .in_time(ONE_FOUR)
        .with_tempo(&[tempo(0, 50)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 541 to 549.
        Part::new(
            "Piano figure",
            "(3 F#5:1 G#5 D5 (3 _ A5 A4 (3 C#5 Eb5 C5 (3 _ F5 B4 | (3 F#5 G#5 D5 (3 _ A5 A4 (3 \
             C#5 Eb5 C5 (3 _ F5 B4 | (3 C#5 B4 Eb5 (3 D5 F#5 F5 (3 G#5 A5 F#5 (3 F5 Eb5 D5 | (3 \
             F#5 G#5 D5 (3 _ A5 A4 (3 C#5 Eb5 C5 (3 _ F5 B4 | (3 C#5 B4 Eb5 (3 D5 F#5 F5 (3 G#5 \
             A5 F#5 (3 F5 Eb5 D5 | (3 G#5 A5 F#5 (3 F5 Eb5 D5 (3 G#5 A5 F#5 (3 F5 Eb5 D5 | (3 \
             G#5 A5 F#5 (3 F5 Eb5 D5 (3 G#5 A5 F#5 (3 F5 Eb5 D5 | (3 G#5 A5 F#5 (3 F5 Eb5 D5 (3 \
             G#5 A5 F#5 (3 F5 Eb5 D5 | (3 G#5 A5 F#5 (3 F5 Eb5 D5 (3 G#5 A5 F#5 (3 F5 Eb5 D5",
            "",
        )
        .with_left(
            "-:8 | - | -:6 (3 -:1 - D5. | _:2 -:6 | - (3 -:1 - D5. | (3 _ _ - (3 - - D5. (3 _ _ \
             - (3 - - D5. | (3 _ _ - (3 - - D5. (3 _ _ - (3 - - D5. | (3 _ _ - (3 - - D5. (3 _ _ \
             - (3 - - D5. | (3 _ _ - -:6 & -:8 | - | -:2 (3 -:1 - F5. (3 _:2 F5 D5 | -:8 | -:4 \
             (3 -:2 F5 D5 | (3 - - D5 (3 - - D5 | (3 - - D5 (3 - - D5 | (3 - - D5 (3 - - D5 | (3 \
             - - D5 (3 - - D5 & -:8 | - | -:4 (3 A5:2 - - | -:8 | -:4 (3 A5:2 - - | (3 G#5+A5 \
             F5+F#5 - (3 G#5+A5 F5+F#5 - | (3 G#5+A5 F5+F#5 - (3 G#5+A5 F5+F#5 - | (3 G#5+A5 \
             F5+F#5 - (3 G#5+A5 F5+F#5 - | (3 G#5+A5 F5+F#5 - (3 G#5+A5 F5+F#5 -",
        )
        .with_tempo(&[tempo(0, 100)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)]),
        // MIDI bar 550.
        Part::new(
            "Flutes",
            "-:4 (3 Bb5:1 D6 C#6 (3 F6 Eb6 A6",
            "",
        )
        .with_left(
            "(3 Eb4:2 A4+F5 C5+F#5 (3 F5+B5 B4+Bb5 E5 & (3 Eb5:1 F5 F#5 (3 A5 G#5 B5 Bb5 -:3",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 551.
        Part::new(
            "Flutes, bar 2",
            "(3 B6:2 A6 F6 (3 D6 B5 A5 (3 F5 D5 B4",
            "",
        )
        .with_left(
            "(3 A5:2 F5+B5 D5+G#5 (3 B4+F5 A4+Eb5 G4+C#5 (3 F4+B4 E4+Bb4 Eb4+A4 & (3 F#6:2 Eb6 \
             C#6 (3 Bb5 G#5 F#5 (3 Eb5 C#5 Bb4 & (3 -:1 B6 - (3 A6 - F6 (3 - D6 - (3 B5 - A5 (3 \
             - F5 - (3 D5 - B4",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 552 to 553.
        Part::new(
            "Rage",
            "(3 A4:1 F5 A5 _:10 | (3 G5:1 E5 C5 G4:10",
            "",
        )
        .with_left(
            "C#4:1 - C#4 - C#4 - C#4 - C#4 - C#4 - | -:12 & A4:1 C#4+F#4+Bb4 D4+F4+A4 \
             C#4+F#4+Bb4 D4+F4+A4 C#4+F#4+Bb4 D4+F4+A4 C#4+F#4+Bb4 D4+F4+A4 C#4+F#4+Bb4 D4+F4+A4 \
             C#4+F#4+Bb4 | E4+G4+C5 C#4+F#4+Bb4 C5 C#4+F#4+Bb4 E4+G4+C5 C#4+F#4+Bb4 E4+G4+C5 \
             C#4+F#4+Bb4 E4+G4+C5 C#4+F#4+Bb4 E4+G4+C5 C#4+F#4+Bb4 & (3 D4:1 F4 A4 (3 F5 A5 _ \
             -:8 | -:2 (3 E4:1 G4 _ -:8",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x x x x x | x x x x x x x x x x x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 554.
        Part::new(
            "Rage, bar 3",
            "(3 A4:1 F5 A5 _:6",
            "",
        )
        .with_left(
            "A4:1 C#4+F#4+Bb4 D4+F4+A4 C#4+F#4+Bb4 D4+F4+A4 C#4+F#4+Bb4 D4+F4+A4 C#4+F#4+Bb4 & \
             C#4:1 - C#4 - C#4 - C#4 - & (3 D4:1 F4 A4 (3 F5 A5 _ -:4",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 555 to 558.
        Part::new(
            "Rage, bar 4",
            "(3 G5:1 E5 C5 G4:10 | Bb4:1 C5 C#5 F#5 (3 E5 C5 G4 Bb4:2 - F#4 | G4 C5 (3 Bb4:1 F#4 \
             C#4 E4:2 G5:4 | A5:1 A5 G5 G5 A5 A5 G5 G5 A5 A5 G5 G5",
            "",
        )
        .with_left(
            "G5:1 E5 -:10 | -:6 (3 G4:1 Bb4 _ -:4 | - F#4:1 G4 F#4 G4 F#4 G4 F#4 G4 | -:12 & \
             E4+G4+C5:1 C#4+F#4+Bb4 E4+C5 C#4+F#4+Bb4 E4+G4+C5 C#4+F#4+Bb4 E4+G4+C5 C#4+F#4+Bb4 \
             E4+G4+C5 C#4+F#4+Bb4 E4+G4+C5 C#4+F#4+Bb4 | E4+G4+C5 C#4+F#4+Bb4 E4+G4+C5 \
             C#4+F#4+Bb4 E4+G4+C5 C#4+F#4+Bb4 E4+C5 C#4+F#4 E4+G4+C5 C#4+F#4+Bb4 E4+G4+C5 \
             C#4+F#4+Bb4 | E4+C5 C#4+F#4+Bb4 E4+G4 C#4+F#4+Bb4 G4+C5+E5 F4+A4+C#5 G4+C5+E5 \
             F4+A4+C#5 C5+E5 A4+C5+F#5 C5+E5+G5 A4+C5+F#5 | B4+D5+F5 Bb4+C#5+F#5 C5+E5 \
             Bb4+C#5+F#5 B4+D5+F5 Bb4+C#5+F#5 C5+E5 Bb4+C#5+F#5 B4+D5+F5 Bb4+C#5+F#5 C5+E5 \
             Bb4+C#5+F#5",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x x x x x | x x x x x x x x x x x x | x x x x x x x x \
                           x x x x | x:12"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 559 to 560.
        Part::new(
            "Petrushka's chord again",
            "-:22 (3 C5:1 E5 G5 | C6:10 G5:2 C6 G5 C6:1 -:7",
            "",
        )
        .with_left(
            "-:4 G3:2. -:6 G3:2. -:10 | -:24 & G3+B3+D4+F4+A4:2 G3+B3+D4+F4+A4 C4+E4+G4 \
             G3+C4+E4+G4 G3+B3+D4+F4+A4 G3+B3+D4+F4+A4 C4+E4+G4 G3+C4+E4+G4 G3+B3+D4+F4+A4 \
             G3+B3+D4+F4+A4 G3+C4+E4+G4:1 - G3+C4+E4+G4 - | -:24 & -:22 (3 Bb4:1 C#5 F#5 | \
             Bb5:10 F#5:2 Bb5 F#5 Bb5:1 -:7",
        )
        .in_time(THREE_TWO)
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 561.
        Part::new(
            "Petrushka's cry",
            "-:2 G#5:1 G#5 -:2 (3 G#5:1 G#5 G#5 G#5 G#5 (3 G#5 G#5 G#5 A#5:2 F#5:1 -",
            "",
        )
        .with_left(
            "-:12 F#3:1 -:3 & -:12 F#2+C#3+A#4+C#5+F#5:1 -:3 & G#2+C5+D#5+G#5:4 -:8 A#5:4 & -:10 \
             G#5:2 -:4",
        )
        .in_time(COMMON)
        .in_key(FSharp, Major)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Fortissimo),
    ],
    form: &["Drum roll", "Oboes", "Drum roll again", "Petrushka's Room", "Muted trumpet", "Flute", "Violins", "Piano chord", "Piano chord, bar 2", "Petrushka's chord", "Trumpet", "Clarinet", "Piano run", "Clarinet again", "Piano cadenza", "Petrushka's curses", "Petrushka's curses, bar 6", "Petrushka's curses, bar 7", "Cry", "Piccolo", "Oboe", "Clarinet arpeggio", "Flute tune", "Cor anglais", "Cor anglais, bar 3", "Cor anglais, bar 4", "Cor anglais, bar 6", "Flute and bassoon", "Piano alone", "Piano alone, bar 2", "Oboe figure", "Trumpet answer", "Oboe figure", "Trumpet answer", "Oboe figure 3", "Piccolo cry", "Oboe figure 4", "Piccolo cry", "Oboe figure 4", "Piccolo cry 3", "C major", "Clarinet cadenza", "Clarinet cadenza, bar 4", "Piano", "Piano, bar 2", "Cor anglais, 1/4", "Piano, 3/4", "Cor anglais, 1/4 again", "Piano figure", "Flutes", "Flutes, bar 2", "Rage", "Rage, bar 3", "Rage, bar 4", "Petrushka's chord again", "Petrushka's cry"],
    drums_from: 0,
    about: "The second tableau of Petrushka (1911, revised 1947): Petrushka's room, a dark cell \
            in the Charlatan's booth, where the puppet is thrown, curses the Charlatan, \
            despairs, and frightens away the Ballerina when she visits him. A drum roll on F \
            sharp links it to the first tableau; over it two clarinets, then two oboes, play C \
            and F sharp, a tritone apart. After Petrushka is kicked in (piccolo, oboes and \
            piano, fortissimo), two clarinets play his chord: one an arpeggio of C major, the \
            other one of F sharp major, at the same time. This is the \"Petrushka chord\", two \
            major triads a tritone apart that share no note. Its six notes, C, E, G and F sharp, \
            A sharp, C sharp, all lie in one octatonic scale, half step and whole step in turn \
            (C, D flat, E flat, E, F sharp, G, A, B flat); no major or minor key holds them, and \
            the parts built on it are marked chromatic. The piano answers in a cadenza of \
            arpeggios on both chords, and the whole orchestra hurls his curse, fortissimo: a \
            trumpet fanfare on C major and F sharp over the two chords in tremolo, with a side \
            drum. Then, slowly, his loneliness: the piccolo over a drone of D and A in D Lydian \
            (the major scale with a raised fourth, G sharp), the flute in D major, and the cor \
            anglais in E minor over a pedal on E, the flute and bassoon after it. When the \
            Ballerina comes, the oboe's figure on E and the trumpet's answer, in E minor, \
            alternate in bars of 2/4 and 3/4; the piccolo's cries and the clarinets' chromatic \
            descent grow frantic, a scale of C major runs up, and she is gone. A clarinet \
            cadenza on a diminished seventh, the piano and the cor anglais in bars of 1/4 and \
            3/4, a piano figure in triplets; then his rage, the whole orchestra in tremolo \
            chords, D minor and C major against F sharp major, under the trumpet. The clarinets \
            play his chord once more, pianissimo, and the tableau ends on his cry: a trumpet on \
            G sharp and a chord of F sharp major, fortissimo.",
};

// Copyright: this is Stravinsky's 1947 revision of Petrushka, which is still under
// copyright (Boosey & Hawkes); it is added here at the request of the project's owner.
// Source: Stravinsky's Petrushka, 1947 revision, third tableau, "The Moor's Room", MIDI bars
// 562 to 793: from the drum roll that links it to the second tableau up to the Moor chasing
// Petrushka out. The whole tableau fits in 99 patterns, so the Dance of the Ballerina and the
// Waltz stay in it. The notes are converted by script from a MIDI file of the 1947 version (the
// person's own copy, at sounding pitch) and checked bar by bar against that file, not against a
// printed score; MIDI bar numbers are given below (MIDI bar 1 is a silent lead-in). The melody
// is the line in front, in the octave of the instrument that has it, and played_by follows it
// from instrument to instrument. Everything else that matters is written out on the Keys
// channel: the harmony, the bass, the ostinati and the timpani; lines doubled in unison or
// octaves are written once. Rhythms are quantised to sixteenths and eighth-note triplets;
// tremolos and rolls are written as held notes, grace notes are left out. The dynamic of each
// part is read from the MIDI's velocities. The percussion is written out (triangle as Ride,
// tambourine as Shaker, bass drum as Kick, side drum as Snare, cymbals as Crash). Where the
// meter changes, every run of bars in one meter is a part of its own, named "..., bar N" after
// the bar it starts on, with in_time: in_meters cannot cut a written-out left hand of several
// voices into its runs, nor carry drums. No chord symbol is honest for this music, which moves
// in drones, ostinati and parallel chords, so the charts are empty. Unlike the first tableau, a
// tremolo between two chords (32nd notes taking turns) is written as the two chords taking
// turns in sixteenths, and a run of 32nd notes keeps every other note. Where a part starts on a
// chord held over from the bar before, the chord is struck again.
//
// Left out: MIDI bar 606, a silent bar of 2/8, written instead as a pause on the last sixteenth
// of bar 605; a roll on the lowest A of the keyboard (A0) on the MIDI's celesta track in bars
// 580 to 585, which is most likely a drum roll put on the wrong track; the piano, which doubles
// the orchestra, where too many lines play at once (bars 773 to 778, 787, 792 and 793); in bars
// 779 to 783 the Keys channel keeps the second flute, the first clarinet, the first bassoon,
// the trumpets, the first violins and the double basses, and leaves out the third flute, the
// oboes, the cor anglais, the second clarinet, the horns, the piano, the inner strings and the
// pizzicato violins. Grace notes are left out throughout.
//
// Not certain: the meters are the MIDI's: 2/8 in bars 604 and 609, 3/8 in bars 605, 610, 612,
// 785 to 789 and 791, 4/8 in bars 611, 790, 792 and 793, 9/8 in bar 617, 5/8 in bar 667
// (counted 2 + 3, which the MIDI cannot tell), 13/16 in bar 730 and 6/8 in bars 764 to 783,
// between bars of 2/4, 3/4 and 4/4. The MIDI's ritardandos (bars 667 and 668, 735) are written
// as glides. The keys: the bars that keep to no scale are marked chromatic (see the song's
// description): the flourish of bar 570, the trombones' and strings' chromatic descents, the
// whole-tone chords of the Lento (bars 580 to 585), the Moor's frantic bars 600 to 621, and
// Petrushka's entrance, the fight and the chase (bars 751 to 793), where two keys sound at
// once. The opening (bars 566 to 568) is marked G sharp Phrygian, though bar 568's chord of E
// has a D natural, its seventh, which is outside it. The Moor's tune is marked B flat Lydian,
// the mode of its notes over the pedal on B flat, though in bars 595 to 598 the pizzicato
// strings add A flat (written G sharp) under it; the oboes' parallel chords in bars 622 and 623
// move by half steps (C sharp, E flat) around D minor, marked D minor; the clarinets' chords in
// bars 665 and 666, before the Waltz, borrow C flat (written B) from E flat minor, marked E
// flat major like the Waltz; bar 633, the trumpet's upbeat to the Dance of the Ballerina over
// the Moor's pedal on B flat, is marked F major like the Dance.
pub(super) const PETRUSHKA_MOOR: Song = Song {
    name: "Petrushka, The Moor's Room",
    origin: "Igor Stravinsky, Petrushka, 1911, revised 1947",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: C,
    mode: Chromatic,
    time: THREE_FOUR,
    tempo: 126.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FLUTE, gm::ACOUSTIC_GRAND_PIANO, gm::CONTRABASS),
    style: "",
    parts: &[
        // MIDI bars 562 to 565.
        Part::new(
            "Drum roll",
            "-:16 | - | - | -:15 A5:1",
            "",
        )
        .with_left(
            "F#2:1 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 | F#2 F#2 F#2 F#2 \
             F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 | F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 \
             F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 | F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 F#2 \
             F#2 F#2 F#2 F#2 & -:16 | - | - | -:15 Eb4+E4+A4+E5:1",
        )
        .in_time(COMMON)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x x x x x x x x x | x x x x x x x x x x x x x x x x | \
                           x x x x x x x x x x x x x x x x | x x x x x x x x x x x x x x x x"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 566.
        Part::new(
            "The Moor",
            "G#5:3 A5:1 G#5:3 A5:1",
            "",
        )
        .with_left(
            "-:2 (3 -:1 - D#4 _:3 -:1 & -:4 E4+A4:2. - & D#4+G#4+D#5+G#5:2 (3 _:1 _ E4+A4+E5+A5 \
             G#4+D#5:3 D#4+E4+A4+E5:1 & -:4 G#5",
        )
        .in_time(TWO_FOUR)
        .in_key(GSharp, Phrygian)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 567 to 568.
        Part::new(
            "The Moor, bar 2",
            "G#5:3 A5:1 G#5:3 A5:1 G#5:4 | (3 B5:1 G#5 A5 (3 _ G#5 A5 G#5 -:7",
            "",
        )
        .with_left(
            "-:4 E4:2. - E4. - | D#4+F4+B4:1 -:4 E4+A4:1 -:6 & -:4 D#4:3 E4:1 D#4:4 | (3 \
             D4+G#4:1 G#4+A4 D4+G#4+B4 (3 E4+A4 D4+G#4+B4+D5 E4+A4+D5+E5 D#4+G#4+G#5:4 \
             D#4+G#4+G#5 & D#4+G#4+D#5:3 E4:1 A4+E5:2. - A4+E5. - | E4+E5+D6. - D4+D#5:1 -:7 & \
             -:3 A4+E5:1 G#4+D#5:3 A4+E5:1 G#4+D#5:4 | (3 -:1 D#5 A4 (3 - D#5 - -:8 & G#5:4 G#5 \
             - | B5 -:8 & -:12 | D5:2 -:10",
        )
        .in_key(GSharp, Phrygian)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 569 to 571.
        Part::new(
            "Flourish",
            "-:12 | (3 D4:1 Eb4 E4 F#4:2 -:8 | -:12",
            "",
        )
        .with_left(
            "-:12 | (3 B3+B4:1 A3+G#4 G#3+A4 F#3+C4+F#4:10 | _:12 & -:12 | D4+E4:2. -:10 | -:12",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bar 572.
        Part::new(
            "Piano and violins",
            "-:2 (3 G3:1 D4 G4 F#4 -:7",
            "",
        )
        .with_left(
            "(3 -:1 C#2 F#2 (3 G2 - - -:8 & (3 F#1+F#2:1 C#3 F#3 -:10 & -:4 F#4:8",
        )
        .in_key(FSharp, Phrygian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 573 to 575.
        Part::new(
            "Piano and violins, bar 2",
            "-:2 (3 G3:1 D4 G4 F#4 -:3 | -:2 (3 G3:1 D4 G4 F#4 -:3 | -:2 (3 G3:1 D4 G4 F#4 -:3",
            "",
        )
        .with_left(
            "(3 -:1 C#2 F#2 (3 G2 - - -:4 | (3 -:1 C#2 F#2 (3 G2 - - -:4 | (3 -:1 C#2 F#2 (3 G2 \
             - - -:4 & (3 F#1+F#2:1 C#3 F#3 -:6 | (3 F#1+F#2:1 C#3 F#3 -:6 | (3 F#1+F#2:1 C#3 \
             F#3 -:6 & -:4 F#4:2 - | -:4 F#4:2 - | -:4 F#4:2 -",
        )
        .in_time(TWO_FOUR)
        .in_key(FSharp, Phrygian)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 576 to 577.
        Part::new(
            "Trombones",
            "(3 D4:2 C4 B3 B3:4 | _:8",
            "",
        )
        .with_left(
            "(3 D3:2 C3 B2 B2:4 | _:8 & -:2 (3 -:1 - C#3+G#3+C#4 Bb2 -:3 | -:8 & (3 A3:1 \
             C#3+G#3+C#4 G3 (3 C#3+G#3+C#4 F#3 _ F3:2. - | -:7 G#2+Eb3+C4:1",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TROMBONE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 578 to 579.
        Part::new(
            "Trombones, bar 3",
            "A3:2 G3 F#3:1 -:7 | -:12",
            "",
        )
        .with_left(
            "A2:2 G2 -:8 | -:12 & -:4 F#2+C3+F#3:8 | _:12 & E3:1 G#2+Eb3+G#3 D3 G#2+Eb3+G#3 -:8 \
             | -:12",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TROMBONE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 580.
        Part::new(
            "Timpani and strings",
            "-:8",
            "",
        )
        .with_left(
            "G#1:8 & -:2 G#2 - G#2:1 -",
        )
        .in_time(TWO_FOUR)
        .with_tempo(&[tempo(0, 48)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 581.
        Part::new(
            "Timpani and strings, bar 2",
            "-:12",
            "",
        )
        .with_left(
            "G#2:2 G#2 - G#2 - G#2:1 -",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 582 to 585.
        Part::new(
            "Timpani and strings, bar 3",
            "D3:4 E3 | F#3 G#3 | Bb3:2 C4 D4 E4 | (3 F#4 G#4 Bb4 (3 C5 D5 -",
            "",
        )
        .with_left(
            "G#2:2 G#2 - G#2:1 - | -:2 G#2 G#2 G#2:1 - | -:2 G#2 - G#2:1 - | -:8 & -:8 | - | -:2 \
             C3 D3+F3+Bb3 - | (3 C2+F#3+A3+D4 C#2+E3 _ -:4 & Bb2:4 A2+C3 | C3+D3 C#3+E3 | \
             C#3+E3+F#3:2 E3+G#3:4 C3+E3+A3+C4:2 | (3 - Eb3+G#3+C#4+E4 F3+Bb3+C#4+E4+F#4 (3 \
             C2+F#3+F4+G#4 F2+D4+G#4+Bb4 -",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 586.
        Part::new(
            "Piccolo",
            "-:4 G6:1 G7 Eb7 -:5",
            "",
        )
        .with_left(
            "-:6 A3+A4:1 -:3 A3+A4:1 - & -:4 G5:1 F#6 -:6",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Crash, "x:12"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 587.
        Part::new(
            "Bassoon",
            "-:8",
            "",
        )
        .with_left(
            "Bb1+Bb2:4 Bb1+Bb2 & -:1 Bb3:4 Bb3:3 & -:2 A3+A4:4 A3+A4:2 & -:2 Bb4:3 -:1 Bb4:2",
        )
        .in_time(TWO_FOUR)
        .in_key(ASharp, Lydian)
        .played_by(&[(gm::BASSOON, 0)])
        .with_drums(&[
            (Drum::Kick, "-:2 x:6"),
            (Drum::Crash, "-:1 x:4 x:2 x:1"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bar 588.
        Part::new(
            "Bassoon, bar 2",
            "-:2 A4:4 G4:1 F4 G4:2 F4",
            "",
        )
        .with_left(
            "Bb1+Bb2:4 Bb1+Bb2 Bb1+Bb2 & -:2 A3:4 -:6 & -:1 Bb3:4 Bb3 Bb3:3 & -:2 Bb4:3 -:1 \
             Bb4:3 -:1 Bb4:2",
        )
        .in_key(ASharp, Lydian)
        .played_by(&[(gm::BASSOON, 0)])
        .with_drums(&[
            (Drum::Kick, "-:2 x:8 x:2"),
            (Drum::Crash, "-:1 x:4 x:2 x x:3"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bars 590 to 599.
        Part::new(
            "The Moor's tune",
            "-:2 A4:4 G4:1 F4 G4:2 F4 | E4 D4 E4 F4 G4 F4 | E4 D4 E4 F4 G4 F4 | E4:1 D4 C4:2 \
             D4:4 - | -:2 A4:4 G4:1 F4 G4:2 F4 | E4 D4 E4 F4 G4 F4 | E4 D4 E4 F4 G4 F4 | E4:1 D4 \
             C4:2 D4:6 F4:2 | E4:1 D4 C4:2 D4:8 | -:12",
            "",
        )
        .with_left(
            "Bb1+Bb2:4 Bb1+Bb2 Bb1+Bb2 | Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | \
             Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | \
             Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | Bb1+Bb2 Bb1+Bb2 Bb1+Bb2 | -:12 \
             & -:12 | - | - | E2:1 D2 E2:2 D2:4 - | -:12 | - | - | E2:1 D2 E2:2 D2:6 F2:2 | E2:1 \
             D2 E2:2 D2:8 | -:12 & -:1 Bb3:4 Bb3 Bb3:3 | _:1 Bb3:4 Bb3 Bb3:3 | _:1 Bb3:4 Bb3 \
             Bb3:3 | _:1 Bb3:4 Bb3 Bb3:3 | _:1 Bb3:4 Bb3 Bb3:3 | _:1 G#3:4 Bb3 G#3:3 | _:1 Bb3:4 \
             G#3 Bb3:3 | _:1 G#3:4 Bb3 G#3:3 | _:1 Bb3:4 G#3 Bb3:3 | -:12 & -:12 | - | - | -:6 \
             D4:3 E2+E4:1 F2+F4 G2+G4. | _ - A2 -:9 | -:12 | - | - | - | - & -:2 Bb4:3 -:1 Bb4:3 \
             -:1 Bb4:2 | _:1 - Bb4:3 -:1 Bb4:3 -:1 Bb4:2 | _:1 - Bb4:3 -:1 Bb4:3 -:1 Bb4:2 | _:1 \
             - Bb4:3 -:1 Bb4:3 -:1 Bb4:2 | _:1 - Bb4:3 -:1 Bb4:3 -:1 Bb4:2 | _:1 - G#4:3 -:1 \
             Bb4:3 -:1 G#4:2 | _:1 - Bb4:3 -:1 G#4:3 -:1 Bb4:2 | _:1 - G#4:3 -:1 Bb4:3 -:1 G#4:2 \
             | _:1 - Bb4:3 -:1 G#4:3 -:1 Bb4:2 | -:12",
        )
        .in_key(ASharp, Lydian)
        .played_by(&[(gm::CLARINET, 0)])
        .with_drums(&[
            (Drum::Kick, "-:2 x:6 x:4 | -:2 x:6 x:4 | -:2 x:6 x:4 | -:2 x:6 x:4 | -:2 x:6 x:4 | \
                          -:2 x:6 x:4 | -:2 x:6 x:4 | -:2 x:6 x:4 | -:2 x:6 x:4 | -:2 x:10"),
            (Drum::Crash, "-:1 x:4 x:2 x:3 x:2 | -:1 x:4 x:2 x:3 x:2 | -:1 x:4 x:2 x:3 x:2 | -:1 \
                           x:4 x:2 x:3 x:2 | -:1 x:4 x:2 x:3 x:2 | -:1 x:4 x:2 x:3 x:2 | -:1 x:4 \
                           x:2 x:3 x:2 | -:1 x:4 x:2 x:3 x:2 | -:1 x:4 x:2 x:3 x:2 | -:1 x:4 x:2 \
                           x:5"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bars 600 to 603.
        Part::new(
            "Cor anglais",
            "-:16 | -:2 A3:1 A3 G#3 G#3:11 | (3 A3:2 _ C4 (3 _ G#3 _ Bb3:8 | -:2 A3:1 A3 G#3 \
             G#3:11",
            "",
        )
        .with_left(
            "E1:2 C2+E2 E1 C2+E2 E1 C2+E2 E1 C2+E2 | E1 C2+E2 E1 C2+E2 E1 C2+E2 E1 C2+E2 | E1 \
             C2+E2 E1 C2+E2 E1 C2+E2 E1 C2+E2 | E1 C2+E2 E1 C2+E2 E1 C2+E2 E1 C2+E2:1 -",
        )
        .in_time(COMMON)
        .with_tempo(&[tempo(0, 96)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 604.
        Part::new(
            "Bassoons",
            "(3 C#2:1 Bb2 Bb3 (3 F#3 - -",
            "",
        )
        .with_left(
            "Bb1:1 -:3 & (3 Bb2:1 Bb2+Bb3 _ -:2 & -:2 F#2+C3+F#3:1 -",
        )
        .in_time(TWO_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::BASSOON, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 605.
        Part::new(
            "Bassoons held",
            "F#3:4 F#3:1 -",
            "",
        )
        .with_left(
            "C3:4 C3:1 -",
        )
        .in_time(THREE_EIGHT)
        .with_tempo(&[fermata(5, 1)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::BASSOON, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 607 to 608.
        Part::new(
            "Cor anglais again",
            "-:2 A3:1 A3 G#3 G#3:11 | (3 A3:2 _ C4 (3 _ G#3 _ Bb3:8",
            "",
        )
        .with_left(
            "E1:2 C2+E2 E1 C2+E2 E1 C2+E2 E1 C2+E2 | E1 C2+E2 E1 C2+E2 E1 C2+E2 E1 C2+E2:1 -",
        )
        .in_time(COMMON)
        .in_key(C, Chromatic)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 609.
        Part::new(
            "Bassoons again",
            "(3 C#2:1 Bb2 Bb3 (3 F#3 - -",
            "",
        )
        .with_left(
            "Bb1:1 -:3 & (3 -:1 A2 - -:2 & (3 Bb2:1 Bb2+Bb3 _ -:2 & -:2 F#2+C3+F#3:1 -",
        )
        .in_time(TWO_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::BASSOON, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 610.
        Part::new(
            "Horn",
            "-:2 (3 C#4:1 _ E4 (3 _ C4 _",
            "",
        )
        .with_left(
            "C3+F#3:4 C3+F#3:1 -",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FRENCH_HORN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 611.
        Part::new(
            "Piccolo flourish",
            "-:6 (3 D6:1 C#6 G#6",
            "",
        )
        .with_left(
            "D3+D4:2 -:6 & -:6 (3 E5:1 - - & -:6 D5:1 G#5",
        )
        .in_time(FOUR_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 612.
        Part::new(
            "Piccolo flourish, bar 2",
            "D7:1 -:5",
            "",
        )
        .with_left(
            "(3 Bb1+Bb2:1 Bb2+Bb3 _ -:4 & (3 C2:1 - - (3 F#2+C3+C#3+F#3+C#4 _ E3+E4 (3 _ C3+C4 _",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::PICCOLO, 0)])
        .with_drums(&[
            (Drum::Snare, "-:4 (3 -:1 x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 613 to 615.
        Part::new(
            "Trumpets",
            "(3 F#5:2 G#5 E5 F#5:4 (3 _:2 E5 F#5 | (3 D5 E5 C5 D5:8 | _:12",
            "",
        )
        .with_left(
            "(3 D3+C4+C5:2 C#4+C#5 A3+A4 C4+G#4+C5:4 (3 _:2 A3+A4 C4+C5 | (3 G#3+G#4 A3+A4 F3+F4 \
             G#3+E4+G#4:8 | _:12 & (3 B4:2 Bb4 - -:4 (3 -:2 - G#4 | (3 G4 F#4 - -:8 | -:12",
        )
        .with_tempo(&[tempo(0, 138)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:12 | - | -"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 616.
        Part::new(
            "Rush",
            "(3 Eb2:1 Bb2 D4 B4:2. Eb5 -:6",
            "",
        )
        .with_left(
            "(3 Eb1:2 Bb2+B2 Eb3+E3 Eb3+Bb3:1 -:7 & -:2 (3 Eb2+Bb3:1 - - -:8 & Eb3:2 -:10 & (3 \
             -:1 E2+F2 G#2 (3 A2 B2 C3 E4+A4+B4+C5+D5 -:7 & (3 E1:1 Bb1+G#3+E4 B3+G#4 (3 _ \
             Bb2+Eb4+E4+B4 G#4+A4+E5 -:7 E4+A4+B4+E5:1 & -:4 Eb4+Bb4:1 -:7",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .with_drums(&[
            (Drum::Kick, "x:12"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 617.
        Part::new(
            "Oboe",
            "Eb5:4 -:2 Eb5:4 -:2 Eb5:4 -:2",
            "",
        )
        .with_left(
            "Bb4:4 -:2 Bb4:4 -:2 Bb4:4 -:2 & -:4 (3 -:1 - A4 Eb4:4 -:2 Eb4:4 -:2 & Eb4:4 (3 -:1 \
             - E4+B4+E5. (3 _ _ - -:2 (3 -:1 - E4+A4+B4+E5 -:6",
        )
        .in_time(NINE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 618.
        Part::new(
            "Oboe, bar 2",
            "-:4 Eb5 -",
            "",
        )
        .with_left(
            "-:4 Bb4 - & -:3 E4+A4+B4+E5:1 Eb4:4 -:2 (3 -:1 C#4 D4",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 619 to 621.
        Part::new(
            "Trumpets again",
            "F5:1 -:3 E5:1 -:3 (3 Eb5:2 D5 C5 | C5:1 -:7 (3 Bb4:2 A4 A4 | -:12",
            "",
        )
        .with_left(
            "-:12 | C4:1 -:7 (3 Bb3:2 A3 A3 | _:12 & F4:1 -:3 E4:1 -:3 (3 Eb4:2 D4 C4 | \
             A3+G4+C5:8 (3 -:2 - Eb4 | Eb4+A4:12 & C5+F5:4 B4+E5 (3 Bb4:1 E4+B4+E5 A4 (3 \
             E4+B4+E5 G4 E4+B4+E5 | -:8 (3 E4:1 B3+F4+B4 Eb4 (3 B3+F4+B4 - - | -:12 & -:2 (3 -:1 \
             Eb4 G#4 Eb5:2. -:6 | E5:1 -:11 | -:12",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 622 to 623.
        Part::new(
            "Oboes",
            "-:2 A5:4 G5:1 F5 G5:2 A5:1 - | -:12",
            "",
        )
        .with_left(
            "D2+A2:12 | -:2 C#4+Eb4+G4:1 Bb3+C#4+F4 C#4+Eb4+G4:2 Bb3+C#4+F4 C2+G2 C3+G3 & -:2 \
             D5+F5:4 C#5+Eb5:1 Bb4+C#5 C#5+Eb5:2 D5+F5:1 - | -:12",
        )
        .with_tempo(&[tempo(0, 48)])
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::OBOE, 0)])
        .with_drums(&[
            (Drum::Crash, "-:12 | -:9 x:2 x:1"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bar 624.
        Part::new(
            "Trumpet",
            "-:4 A4 A4:2 A4",
            "",
        )
        .with_left(
            "D2+A2:4 C2+G2 E2+A2 & -:2 D3+A3 - D3+A3 - D3+A3 & -:4 D4 D4:2 D4",
        )
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Kick, "-:1 x:11"),
            (Drum::Crash, "-:2 x:3 x:2 x:3 x:2"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bars 625 to 632.
        Part::new(
            "The Moor's tune, bassoon",
            "-:2 A4:4 G4:1 F4 G4:2 F4 | E4 D4 E4 F4 G4 F4 | E4 D4 E4 F4 G4 F4 | E4:1 D4 C4:2 \
             D4:6 -:2 | - A4:4 G4:1 F4 G4:2 F4 | E4 D4 E4 F4 G4 F4 | E4 D4 E4 F4 G4 F4 | E4:1 D4 \
             C4:2 D4:8",
            "",
        )
        .with_left(
            "C2+G2:4 D2+A2 C2+G2 | C2+G2 C2+G2 C2+G2 | C2+G2 C2+G2 C2+G2 | C2+G2 D2+A2 C2+G2:2 - \
             | F2 Bb2+F3 Bb1:4 Eb2:2 Bb2+F3 | Bb1:4 F2:2 Bb2+Eb3 Bb1:4 | F2:2 Bb2+F3 Bb1:4 Eb2:2 \
             Bb2+F3 | Bb1:4 Bb1 Bb1 & -:6 Eb3:2 -:4 | -:2 D3+A3 - Eb3 -:4 | -:2 D3+A3 - Eb3 -:4 \
             | -:12 | Bb1:4 F2:2 Bb2+Eb3 Bb1:4 | F2:2 Bb2+F3 Bb1:4 Eb2:2 Bb2+F3 | Bb1:4 F2:2 \
             Bb2+Eb3 Bb1:4 | F2:2 G#2+Eb3 F2 G2+D3 F2 G2+D3 & -:2 D4:4 C#4:1 Bb3 C#4:2 Bb3 | Bb3 \
             G3 Bb3 Bb3 C#4 Bb3 | Bb3 G3 Bb3 Bb3 C#4 Bb3 | Bb3:1 G3 F#3:2 G3:6 -:2 | - D4:4 \
             C#4:1 Bb3 C#4:2 Bb3 | Bb3 G3 Bb3 Bb3 C#4 Bb3 | Bb3 G3 Bb3 Bb3 C#4 Bb3 | Bb3:1 G3 \
             F#3:2 G3:8 & -:2 D3+A3 - Bb3:1 -:3 F3+C4:2 | -:10 F3+C4:2 | -:10 F3+C4:2 | - \
             Eb3+Bb3 - D3+A3 -:1 C4+E4 C3+G3+Bb3+C#4+F4 C#4+Eb4+G4 | -:12 | - | - | - & -:2 F4:4 \
             Eb4:1 C#4 Eb4:2 C#4 | C4 Bb3 C4 C#4 Eb4 C#4 | C4 Bb3 C4 C#4 Eb4 C#4 | C4:1 Bb3 \
             G#3:2 Bb3:6 -:2 | -:7 C#4:1 -:2 C#4 | C4 Bb3 C4 C#4 - C#4 | C4 Bb3 C4 C#4 - C#4 | \
             C4:1 Bb3 G#3:2 Bb3:8 & -:12 | - | - | -:7 D4:2 -:3 | -:2 F4:4 E4:1 - E4:2 - | -:8 \
             Eb4:2 - | -:8 Eb4:2 - | -:12",
        )
        .in_key(ASharp, Lydian)
        .played_by(&[(gm::BASSOON, 0)])
        .with_drums(&[
            (Drum::Kick, "-:5 x:7 | -:1 x:11 | -:5 x:7 | -:1 x:11 | -:5 x:7 | -:1 x:11 | -:5 x:7 \
                          | -:1 x:11"),
            (Drum::Crash, "-:1 x:2 x:3 x x:2 x:1 | -:2 x:3 x:2 x:3 x:2 | -:1 x:2 x:3 x x:2 x:1 | \
                           -:2 x:3 x:2 x:3 x:2 | -:1 x:2 x:3 x x:2 x:1 | -:2 x:3 x:2 x:3 x:2 | \
                           -:1 x:2 x:3 x x:2 x:1 | -:2 x:3 x:2 x:3 x:2"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bar 633.
        Part::new(
            "Side drum",
            "(3 -:1 - G4 C5 C4:4 -:5",
            "",
        )
        .with_left(
            "Bb1:4 -:8 & F2:2 G2+D3 -:8",
        )
        .in_key(F, Major)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "-:5 x:1 (3 x x x (3 x x x (3 x x x"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bars 634 to 662.
        Part::new(
            "Dance of the Ballerina",
            "-:8 | - | - | -:6 A4:1 C5 | F5:2 C5:1 A4 F4:2 A4:1 C5 | E5:2 C5:1 A4 F4:2 G4:1 A4 | \
             Bb4 C5 D5 E5 D5 C5 Bb4 C5 | D5 E5 D5 C5 F4:2 A4:1 C5 | F5:2 C5:1 A4 F4:2 A4:1 C5 | \
             E5:2 C5:1 A4 F4:2 A4:1 Bb4 | C5 D5 E5 C5 G4 A4 Bb4 C5 | D5 E5 C5 G4 F4:2 G4:1 G4 | \
             G4 Bb4 Bb4 D5 D5 F5 F5 A5 | A5:3 F5:1 A5 F5 F5 D5 | D5 Bb4 Bb4 G4 G4:2 G5:1 D5 | \
             Bb4:2 G5:1 E5 C5:2 G5:1 D5 | D5 C5 E5 D5 Bb4:2 C5:1 D5 | E5:2 D5:1 C5 Bb4:2 C5:1 D5 \
             | E5:2 D5:1 C5 F4:2 A4:1 C5 | F5:2 C5:1 A4 F4:2 A4:1 C5 | E5:2 C5:1 A4 F4:2 G4:1 A4 \
             | Bb4 C5 D5 E5 D5 C5 Bb4 C5 | D5 E5 D5 C5 F4:2 A4:1 C5 | F5:2 C5:1 A4 F4:2 A4:1 C5 \
             | E5:2 C5:1 A4 F4:2 A4:1 Bb4 | C5 D5 E5 C5 G4 A4 Bb4 C5 | D5 E5 D5 G4 F4 -:3 | F5:1 \
             F5 F5 F5 F5 F5 F5 F5 | F5 -:7",
            "",
        )
        .with_left(
            "-:8 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - \
             | - | - | - | - | - | - | - | -:7 F4+C5+F5:1",
        )
        .in_time(TWO_FOUR)
        .with_tempo(&[tempo(0, 116)])
        .in_key(F, Major)
        .played_by(&[(gm::TRUMPET, 0)])
        .with_drums(&[
            (Drum::Snare, "x:3 x:1 x x x x | x:2 (3 x:1 x x x:2 (3 x:1 x x | x:2 (3 x:1 x x x:2 \
                           (3 x:1 x x | x:2 (3 x:1 x x x x x x | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 \
                           x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 \
                           x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 \
                           x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | \
                           x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 \
                           | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 \
                           x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:3 x:1 x:3 x:1 | x:8"),
        ]),
        // MIDI bars 663 to 664.
        Part::new(
            "Strings",
            "E5:1 -:2 Eb5:1 D5 -:2 C#5:1 | C5:3 B4:1 Bb4 -:3",
            "",
        )
        .with_left(
            "Eb4:2. - Eb4. - | E4. - E4:1 -:3 & B4:1 -:2 Bb4:1 -:3 G#4:1 | -:3 F#4:1 -:4 & \
             E4+A4+E5:2. - D4+G4+D5. - | C4+F4. - Bb3+Eb4:1 -:3",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::VIOLIN, 0)]),
        // MIDI bars 665 to 666.
        Part::new(
            "Clarinets",
            "-:7 Bb4:1 | F4:3 Bb4:1 F4:2 -",
            "",
        )
        .with_left(
            "-:7 Eb4:1 | Eb3+B3:3 Bb3+Eb4:1 F3+B3:2 -:1 Ab5",
        )
        .in_time(TWO_FOUR)
        .in_key(DSharp, Major)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 667.
        Part::new(
            "Flute",
            "Ab6:7 F6:1 D6 Bb5",
            "",
        )
        .in_time(FIVE_EIGHT)
        .in_key(DSharp, Major)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bar 668.
        Part::new(
            "Flute, bar 2",
            "Ab5:1 F5 D5 Bb4 Ab4 F4 D4 Bb4 -:4",
            "",
        )
        .with_tempo(&[tempo(0, 108), glide(0, 50, 7), tempo(8, 80)])
        .in_key(DSharp, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 669 to 688.
        Part::new(
            "Waltz",
            "-:12 | - | - | - | Eb4:2 G4 C5:6 G4:2 | G4 Bb4 Ab4:1 -:3 Ab4:1 -:3 | Ab5:2 D6 Ab6:6 \
             D6:2 | D6 F6 Eb5:1 -:3 Eb5:1 -:3 | Eb5:2 G5 C6:6 G5:2 | G5 Bb5 Ab4:1 -:3 Ab4:1 -:3 \
             | Ab4:2 D5 Ab5 F5 D5 Ab4 | G4 Bb4 G4:4 Eb4:2 F4 | Eb4:1 Eb5 G4 G5 C5 C6 C6 C6 C6 C6 \
             G4 G5 | G4 G5 Bb4 Bb5 Ab4 Ab5 Ab5 Ab5 Ab4 Ab5 Ab5 Ab5 | Ab4:2 D5 Ab5:1 Ab6 Ab6 Ab6 \
             Ab6 Ab6 D5 D6 | D5 D6 F5 F6 Eb5 Eb6 Eb6 Eb6 Eb5 Eb6 Eb6 Eb6 | Eb4 Eb5 G4 G5 C5 C6 \
             C6 C6 C6 C6 G4 G5 | G4 G5 Bb4 Bb5 Ab4 Ab5 Ab5 Ab5 Ab4 Ab5 Ab5 Ab5 | Ab4 Ab5 D5 D6 \
             Ab5 Ab6 F5 F6 C5 C6 D5 D6 | -:12",
            "",
        )
        .with_left(
            "Eb3:2 G3 Bb3 Eb4 Bb3 G3 | D3 F3 Ab3 D4 Ab3 F3 | D3 F3 Ab3 D4 Ab3 F3 | Eb3 G3 Bb3 \
             Eb4 Bb3 G3 | Eb3 G3 Bb3 Eb4 Bb3 G3 | D3 F3 Ab3 D4 Ab3 F3 | D3 F3 Ab3 D4 Ab3 F3 | \
             Eb3 G3 Bb3 Eb4 Bb3 G3 | Eb3 G3 Bb3 Eb4 Bb3 G3 | D3 F3 Ab3 D4 Ab3 F3 | D3 F3 Ab3 D4 \
             Ab3 F3 | Eb3 G3 Bb3 Eb4 Bb3 G3 | Eb3 G3 Bb3 Eb4 Bb3 G3 | D3 F3 Ab3 D4 Ab3 F3 | D3 \
             F3 Ab3 D4 Ab3 F3 | Eb3 G3 Bb3 Eb4 Bb3 G3 | Eb3 G3 Bb3 Eb4 Bb3 G3 | D3 F3 Ab3 D4 Ab3 \
             F3 | D3 F3 Ab3 D4 Ab3 F3 | -:12",
        )
        .with_tempo(&[tempo(235, 195)])
        .in_key(DSharp, Major)
        .played_by(&[(gm::TRUMPET, 0)]),
        // MIDI bars 689 to 696.
        Part::new(
            "The Moor's waltz",
            "-:3 D6:1 D#6:3 D6:1 D#6:3 D6:1 | D#6:2 -:6 F#5:2 - | -:3 D6:1 D#6:3 D6:1 D#6:3 D6:1 \
             | D#6:2 -:6 F#5:2 - | -:3 C6:1 C#6:3 D6:1 D#6:3 D#6:1 | E6:2 -:6 G#5:2 - | -:3 G5:1 \
             G#5:2 - A#5:3 A#5:1 | B5:2 -:6 F#5:2 -",
            "",
        )
        .with_left(
            "-:12 | - | - | - | - | - | - | -:4 D#4 D#4:2 - & B3:4 D#4+B4+D#6 D#4+B4+D#6 | \
             B3+D#6 D#4+B4 D#4+B4+F#5 | B3 D#4+B4+D#6 D#4+B4+D#6 | B3+D#6 D#4+B4 D#4+B4+F#5 | B3 \
             D#4+B4+C#6 D#4+B4+D#6 | A#3+E6 C#4+A#4 C#4+A#4+G#5 | A#3 C#4+G#4+A#4+F5 C#4+A#4 | \
             B3+F#4 B4:2 - B4+F#5:4 & -:12 | - | - | - | - | - | - | -:2 F#5 D#5 B4 F#4 D#4:1 - \
             & -:4 F#5:2 - F#5 - | F#5 -:6 D#5:2 - | -:4 F#5:2 - F#5 - | F#5 -:6 D#5:2 - | -:4 \
             E5:2 - F#5 - | G#5 -:6 E5:2 - | -:4 E5:2 - F#5 - | D#5 -:6 D#5:2 -",
        )
        .in_key(B, Major)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bars 697 to 712.
        Part::new(
            "The Moor's tune, cor anglais",
            "-:6 D#4 | _ C#4:3 B3 | C#4:6 B3 | - C#4:3 B3 | C#4:6 B3 | A#3 G#3 | A#3 B3 | C#4 B3 \
             | A#3:3 G#3 F#3:6 | G#3:12 | D#4:6 C#4:3 B3 | C#4:6 B3 | A#3 G#3 | A#3 B3 | C#4 B3 \
             | A#3:3 G#3 F#3:6",
            "",
        )
        .with_left(
            "-:9 D#2+D#3:3 | -:12 | -:9 B1:3 | -:12 | -:9 B1:3 | -:9 D#3:3 | A#1:6 B2 | C#2 B2 | \
             -:3 G#1 -:6 | -:12 | D#2+D#3:6 - | - B1 | - G#2 | A#1 B2 | C#2 B2 | G#1:3 -:9 & \
             -:12 | - | - | - | - | -:8 D#4+B4:4 | C#4:2 - F4+C#5 - F4+C#5+F#5 - | B3 F#4+A#4 \
             F#4 F#4+A#4 F#4 F#4+A#4 | F#4:4 -:8 | B3:4 D#4+B4 D#4+B4 | B3 D#4+B4 D#4+B4 | B3 \
             D#4+B4 D#4+B4 | B3 D#4+B4 D#4+B4 | B3 D#4+B4 D#4+B4 | A#3 C#4+A#4 C#4+A#4 | A#3 -:8 \
             & B3:4 D#4+B4 D#4+B4 | B3 D#4+B4 D#4+B4 | B3 D#4+B4 D#4+B4 | B3 D#4+B4 D#4+B4 | B3 \
             D#4+B4 D#4+B4 | B3 D#4+B4 C#6:2 - | -:10 G#5:2 | -:12 | -:2 F#5 D#5 B4 F#4 D#4:1 - \
             | -:12 | - | - | - | - | - | -:4 C#4+G#4+A#4+F5 C#4+A#4 & -:3 D6:1 F#5+D#6:2 -:1 D6 \
             F#5+D#6:2 -:1 D6 | F#5+D#6:2 -:6 D#5+F#5:2 - | -:3 D6:1 F#5+D#6:2 -:1 D6 F#5+D#6:2 \
             -:1 D6 | F#5+D#6:2 -:6 D#5+F#5:2 - | -:3 D6:1 F#5+D#6:2 -:1 D6 F#5+D#6:2 -:1 D6 | \
             F#5+D#6:2 -:6 F5+D6:2 - | - F5+C#6 F#5+D#6 F5+C#6 C#6+D#6+A#6 B5+F6+G#6 | \
             F#5+A#5+F#6 F#5+A#5+F#6 F#5+A#5+F#6 F#5+A#5+F#6 F#5+A#5+F#6 F#5+A#5+F#6 | \
             F#5+A#5+F#6:1 -:11 | -:3 D6:1 F#5+D#6:2 -:1 D6 F#5+D#6:2 -:1 D6 | F#5+D#6:2 -:6 \
             D#5+F#5:2 - | -:3 D6:1 F#5+D#6:2 -:1 D6 F#5+D#6:2 -:1 D6 | F#5+D#6:2 -:6 D#5+F#5:2 \
             - | -:3 C6:1 E5+C#6:2 -:1 D6 F#5+D#6:2 -:1 D#6 | G#5+E6:2 -:6 E5+G#5:2 - | -:3 G5:1 \
             E5+G#5:2 - F#5+A#5 -:1 A#5",
        )
        .in_key(B, Major)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .with_drums(&[
            (Drum::Kick, "-:12 | -:6 x | -:12 | -:6 x | -:12 | - | - | - | - | - | -:6 x | -:12 \
                          | - | - | - | -"),
            (Drum::Crash, "-:12 | -:9 x:3 | -:6 x | -:9 x:3 | -:6 x | -:12 | -:6 x | - x | -:3 \
                           x:9 | x:12 | -:9 x:3 | -:6 x | - x | - x | - x | -:3 x:9"),
        ]),
        // MIDI bars 713 to 728.
        Part::new(
            "The Moor's tune, cor anglais 2",
            "G#3:11 -:1 | -:6 D#4 | D#4 C#4:3 B3 | C#4:6 B3 | A#3 G#3 | A#3 B3 | C#4 B3 | A#3:3 \
             G#3 F#3:6 | G#3:12 | _:3 - B3:6 | A#3:3 G#3 F#3:6 | G#3:12 | _:6 D#4 | C#4 B3:3 A#3 \
             | G#3 A#3 G#3 F#3 | -:12",
            "",
        )
        .with_left(
            "-:2 F#5 D#5 B4 F#4 D#4:1 - | -:12 | - | -:6 B1 | - G#2 | G#1 B2 | C#2 B2 | -:3 G#1 \
             -:6 | -:12 | -:2 F#5 D#5 B4 F#4 D#4:1 - | -:3 G#1 -:6 | - G#2 | G#1 - | C#2 B2 | \
             G#1 G#3 | -:12 & -:4 D#4 D#4:2 - | B3:4 -:8 | B3:4 -:8 | B3:4 -:8 | B3:4 -:8 | B3:4 \
             -:8 | A#3:4 -:8 | -:12 | - | - | - | - | - | - | - | - & B3+F#4:4 B4:2 - B4:4 | - \
             D#4+B4 D#4+B4 | - D#4+B4 D#4+B4 | - D#4+B4 D#4+B4 | - D#4+B4 D#4+B4 | - D#4+B4 \
             D#4+B4 | - C#4+A#4 C#4+G#4+A#4+E5 | A#3+A#4+F#5 C#4+A#4+B4+G#5 C#4+A#4+C#5 | \
             B3+D#5:2 D#4+B4+D#5 B3+D#5 D#4+B4+D#5 B3+D#5 D#4+B4+D#5 | B3+D#5:4 -:8 | -:12 | - | \
             - | - | - | - & D#5+B5:2 -:6 D#5+F#5:2 - | -:3 D6:1 F#5+D#6:2 -:1 D6 F#5+D#6:2 -:1 \
             D6 | F#5+D#6:2 -:6 D#5+F#5:2 - | -:3 D6:1 F#5+D#6:2 -:1 D6 F#5+D#6:2 -:1 D6 | \
             F#5+D#6:2 -:6 D#5+F#5:2 - | -:3 C6:1 E5+C#6:2 -:1 D6 F#5+D#6:2 -:1 D#6 | G#5+E6:4 - \
             G#5+F6 | A#5+F#6 B5+G#6 A#5+C#6+A#6 | B5+D#6+B6:2 B5+D#6+B6 B5+D#6+B6 B5+D#6+B6 \
             B5+D#6+B6 B5+D#6+B6 | B5+D#6+B6:1 -:11 | -:12 | - | - | - | - | -",
        )
        .in_key(GSharp, NaturalMinor)
        .played_by(&[(gm::ENGLISH_HORN, 0)])
        .with_drums(&[
            (Drum::Kick, "-:12 | - | -:6 x | -:12 | - | - | - | - | x | - | - | x | - | - | - | \
                          -"),
            (Drum::Crash, "x:12 | - | -:9 x:3 | -:6 x | - x | - x | - x | -:3 x:9 | x:12 | -:6 x \
                           | -:3 x:9 | x:12 | - | - | -:9 x:3 | -:6 x"),
        ]),
        // MIDI bar 729.
        Part::new(
            "Horns",
            "-:3 A4:1 C#5:8",
            "",
        )
        .with_left(
            "-:3 D4+D5:1 C#4+G#4:2 -:6 & -:4 F5 -",
        )
        .with_tempo(&[tempo(0, 138)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::FRENCH_HORN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 730.
        Part::new(
            "Horns, bar 2",
            "-:3 A4:1 C#5:3 A4:1 C#5:3 -:2",
            "",
        )
        .with_left(
            "-:3 D4:1 C#4:3 D4:1 C#4:3 -:2 & -:3 D5:1 G#4:3 D5:1 G#4:3 -:2 & -:4 C#5+F5 F5:3 -:1 \
             D4+A4+D5",
        )
        .in_time(THIRTEEN_SIXTEEN)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FRENCH_HORN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 731 to 732.
        Part::new(
            "Horns, bar 3",
            "(3 C#5:2 C5 B4 B4 -:6 | (3 A4:2 G4 F#4 F#4 -:6",
            "",
        )
        .with_left(
            "(3 C#4:2 C4 B3 B3+B4:8 | (3 A3:2 G3 F#3 F#3+F#4:8 & (3 G#4:1 D4+A4+D5 F4 (3 \
             C#4+F#4+C#5 F4 C#4+F#4+C#5 F4:2. -:6 | (3 E4:1 G#3+Eb4+G#4 C4 (3 G#3+Eb4+G#4 C4 \
             G#3+A3+C#4+G#4 C4 -:7",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FRENCH_HORN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 733 to 734.
        Part::new(
            "Pizzicato cellos",
            "G#2:2 G#3 G#2 G#3 -:4 | - A3:2 A2 -:4",
            "",
        )
        .with_tempo(&[tempo(0, 80)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::PIZZICATO_STRINGS, 0)])
        .marked(Dynamic::Piano),
        // MIDI bar 735.
        Part::new(
            "Pizzicato cellos, bar 3",
            "Bb2:2 Bb3 Bb3 Bb2 Bb2 Bb3 -:4",
            "",
        )
        .in_time(COMMON)
        .with_tempo(&[glide(2, 61, 9), tempo(12, 80)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::PIZZICATO_STRINGS, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 736 to 750.
        Part::new(
            "Waltz again",
            "Eb4:2 G4 C5:6 G4:2 | G4 Bb4 Ab4:1 -:3 Ab4:1 -:3 | Ab5:2 D6 Ab6:6 D6:2 | D6 F6 Eb5:1 \
             -:3 Eb5:1 -:3 | Eb5:2 G5 C6:6 G5:2 | G5 Bb5 Ab4:1 -:3 Ab4:1 -:3 | Ab4:2 D5 Ab5 F5 \
             D5 Ab4 | G4 Bb4 G4:4 Eb4:1 Eb4 F4 F4 | Eb4 Eb4 G4 G4 C5 C5 C5 C5 C5 C5 G4 G4 | G4 \
             G4 Bb4 Bb4 Ab4 Ab4 Ab4 Ab4 Ab4 Ab4 Ab4 Ab4 | Ab4:2 D5 Ab5:6 D5:2 | D5 F5 Eb5 Bb4 G4 \
             Bb4 | (3 Eb4:1 Eb4 Eb4 (3 G4 G4 G4 (3 C5 C5 C5 (3 C5 C5 C5 (3 C5 C5 C5 (3 G4 G4 G4 \
             | (3 G4 G4 G4 (3 Bb4 Bb4 Bb4 (3 Ab4 Ab4 Ab4 (3 Ab4 Ab4 Ab4 (3 Ab4 Ab4 Ab4 (3 Ab4 \
             Ab4 Ab4 | (3 Ab4 Ab5 Ab5 (3 D5 D6 D6 (3 Ab5 Ab6 Ab6 (3 F5 F6 F6 (3 C5 Eb6 Eb6 (3 D5 \
             _ -",
            "",
        )
        .with_left(
            "Eb2:4 -:6 Bb2:2 | Bb2 Bb3 -:6 Bb3:2 | Bb3:1 - Bb2:2 Bb2 Bb3 -:4 | Eb2 -:2 Bb2 Bb2 - \
             | Eb2:4 -:6 Bb2:2 | Bb2 Bb3 -:6 Bb3:2 | Bb3 Bb2 Bb2 Bb3 -:4 | Eb2 -:2 Bb2 Bb2 - | \
             Eb2:4 - Eb2 | - Eb2 - | Eb2 - Eb2 | - Eb2 - | Eb2 - Eb2 | - Eb2 - | Eb2 - Eb2 & \
             Bb3:2 Bb2 Bb2 Bb3 -:4 | Eb2 -:2 Bb2 Bb2 - | Eb2:4 -:6 Bb2:2 | Bb2 Bb3 -:6 Bb3:2 | \
             Bb3 Bb2 Bb2 Bb3 -:4 | Eb2 -:2 Bb2 Bb2 - | Eb2:4 -:6 Bb2:2 | Bb2 Bb3 -:6 Bb3:2 | Bb3 \
             Bb2+Bb3 Bb2 Bb3 - Bb2+Bb3 | Bb2 Bb3 - Bb2+Bb3 Bb2 Bb3 | Bb3 Bb2+Bb3 Bb2 Bb3 - \
             Bb2+Bb3 | Bb2 Bb3 - Bb2+Bb3 Bb2 Bb3 | Bb3 Bb2+Bb3 Bb2 Bb3 - Bb2+Bb3 | Bb2 Bb3 - \
             Bb2+Bb3 Bb2 Bb3 | Bb3:1 - Bb2:2 Bb2+D4:1 - Bb3:2 D4:1 - Bb2+Bb3:2 & -:12 | - | - | \
             - | - | - | - | - | -:4 C#4:1 C#4:7 | -:2 G3:1 G3 F#3 F#3:7 | (3 G3:1 _ C4 (3 _ F#3 \
             _ Ab3:8 | -:2 D4:1 D4 C#4 C#4:7 | -:4 C#4:1 C#4:7 | -:2 G3+Bb4:1 G3 F#3 F#3:7 | (3 \
             G3:1 _ Bb3 (3 _ F#3 _ Ab3:6 -:2 & -:12 | - | - | - | - | - | - | - | (3 -:1 Eb4 Eb4 \
             -:2 Bb3+Eb4 - C4+G4:4 | _ C4+G4 - | G4 Bb3+D4:2 - C4+G4:4 | _ C4+G4 - | C4+G4 - \
             C4+G4 | _ C4+G4 - | C4+G4 - Bb3+C4+Eb4+G4:2 - & -:12 | -:2 G3:1 G3 F#3 F#3:5 -:2 | \
             (3 G3 Bb3 F#3 Ab3:8 | -:12 | - | -:2 G3:1 G3 F#3 F#3:7 | (3 G3:2 C4 F#3 Ab3:8 | \
             -:12 | -:2 Bb4:4 Bb4 Bb4:2 | _ Bb4:4 Bb4 Bb4:2 | _ Bb4:4 Bb4 Bb4:2 | - Bb4 (3 -:1 \
             Eb5 Eb5 (3 Eb5 Eb5 Eb5 (3 Eb5 Eb5 Eb5 (3 Eb5 Eb5 Eb5 | -:2 Bb4 - Bb4 - Bb4 | -:6 \
             Bb4:2 - Bb4 | - Bb4 - Bb4 - Ab3+Bb4:1 - & C4:4 Bb3+Eb4:2. - Bb3+Eb4. - | C4:4 \
             Bb3+D4:2. - Bb3+D4. - | C4:4 Bb3+D4:2. - Bb3+D4. - | C4:4 Bb3+Eb4:2. - Bb3+Eb4. - | \
             C4:4 Bb3+Eb4:2. - Bb3+Eb4. - | C4:4 Bb3+D4:2. - Bb3+D4. - | C4:1 -:3 Bb3+D4:2. - \
             Bb3+D4. - | C4:4 Bb3+Eb4:2. - Bb3+Eb4. - | C4+G4:4 (3 -:1 C5 C5 (3 - C5 C5 \
             Bb3+Eb4:2 - | -:4 Bb3+D4:2 - Bb3+D4 - | (3 C4:1 _ Ab4 (3 - - D5 (3 - - Ab5 (3 _ - \
             Ab5 (3 _ Ab6 Ab5 (3 - D6 D5 | (3 _ D6 D5 (3 - F6 F5 -:8 | -:12 | - | - & -:12 | - | \
             - | - | - | - | - | - | -:8 (3 -:1 C5 C5 -:2 | -:4 Ab4:8 | (3 _:1 Ab5 - (3 - D6 - \
             (3 - Ab6 - (3 - Ab6 - Bb3+D4:2 - | -:4 Bb3+Eb4:2 - Bb3+Eb4 - | -:4 Bb3+Eb4:2 - \
             Bb3+Eb4 - | -:4 Bb3+D4:2 - Bb3+D4 - | -:4 Bb3+Eb4:2 -:6",
        )
        .in_key(DSharp, Major)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 751 to 758.
        Part::new(
            "Petrushka comes",
            "-:16 | -:12 (3 F#3:2 B3 D4 | F#4:10 -:6 | -:4 (3 F#3:2 B3 D4 F#4:8 | _:2 - (3 F#3 \
             B3 D4 F#4:4 (3 F#4:2 B4 D5 | F#5:8 (3 E5:2 C#5 A4 E4:4 | _:8 -:4 (3 C#5:2 A4 E4 | \
             A4:1 -:3 (3 C#5:2 A4 E4 A4:1 -:3 (3 C#5:2 A4 E4",
            "",
        )
        .with_left(
            "-:1 B3+D4+F#4 - B3+D4+F#4 - B3+D4+F#4 - B3+D4+F#4 - B3+D4+F#4 - B3+D4+F#4 - \
             B3+D4+F#4 - B3+D4+F#4 | Eb4 B3+D4+F#4 Eb4 B3+D4+F#4 Eb4 B3+D4+F#4 Eb4 B3+D4+F#4 Bb3 \
             B3+D4+F#4 Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 | Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 \
             Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 G4 B3+D4+F#4 G4 B3+D4+F#4 G4 B3+D4+F#4 G4 B3+D4+F#4 | \
             Bb3+G4 B3+D4+F#4 Bb3+G4 B3+D4+F#4 Bb3+G4 B3+D4+F#4 Bb3+G4 B3+D4+F#4 Bb3 B3+D4+F#4 \
             Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 | Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 Bb3 \
             B3+D4+F#4 Bb3 B3+D4+F#4 Bb3+Eb4 B3+D4+F#4 Bb3+Eb4 B3+D4+F#4 Bb3+Eb4 B3+D4+F#4 \
             Bb3+Eb4 B3+D4+F#4 | Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 Bb3 B3+D4+F#4 G4 \
             A3+C#4+E4 G4 A3+C#4+E4 G4 A3+C#4+E4 G4 A3+C#4+E4 | Bb3+G4 A3+C#4+E4 Bb3+G4 \
             A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 \
             Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 | Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+G4 \
             A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+G4 A3+C#4+E4 \
             Bb3+G4 A3+C#4+E4 & Bb3:16 | _:8 Eb4 | _:16 | _:4 Eb4:12 | _:8 - | Eb4:16 | _ | _ & \
             Eb4+G4:16 | G4 | _:8 Bb3 | - G4 | _ G4 | _ Bb3 | - (3 A4:2 Bb4 Eb5 -:4 | (3 A4:2 \
             Bb4 Eb5 -:4 (3 A4:2 Bb4 Eb5 -:4",
        )
        .in_time(COMMON)
        .with_tempo(&[tempo(0, 160)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 759 to 760.
        Part::new(
            "Petrushka comes, bar 9",
            "G4:1 -:11 | -:2 A3:1 Bb3 Bb3 Eb4 C#4 - (3 C#4:2 A3 E3",
            "",
        )
        .with_left(
            "-:12 | C#4:1 -:3 Bb3:8 & Bb3+Eb4:1 A3+C#4+E4 Bb3+G4 A3+C#4+E4 Bb3+Eb4+G4 A3+C#4 \
             Bb3+Eb4+G4 A3+C#4+E4 Bb3+Eb4 A3+C#4+E4 G4 A3+C#4+E4 | Bb3 A3+C#4+E4 Bb3 A3+C#4+E4 \
             Eb4 A3+C#4+E4 Eb4 A3+C#4+E4 Eb4 A3+C#4+E4 Eb4 A3+C#4+E4 & -:2 Eb4 E4 A4 (3 G4 Eb4 \
             Bb3 | Eb4:4 -:8 & F4:12 | G4",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 761.
        Part::new(
            "Petrushka comes, bar 11",
            "(3 G3:2. _ A3 (3 G3. _ Bb2",
            "",
        )
        .with_left(
            "(3 Eb3:2 E3. _ (3 - Eb3 - & Bb3+Eb4+G4:1 A3+C#4+E4 Bb3+Eb4+G4 A3+C#4+E4 Bb3+Eb4+G4 \
             A3+C#4+E4 Bb3+Eb4+G4 A3+C#4+E4",
        )
        .in_time(TWO_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 762 to 763.
        Part::new(
            "Flutes and clarinets",
            "-:3 Eb6:1 G5 -:2 Eb6:1 G5 -:6 Eb6:1 | (3 G5 - - (3 B5 D6 B5 G5 -:6 Eb6:1 G5 -:2 \
             Eb6:1",
            "",
        )
        .with_left(
            "-:12 (3 A3:2 Bb3 - | (3 - A3 - -:4 Bb3:8 & Bb3+Eb4+G4:1 A3+C#4+E4 Bb3+Eb4+G4 \
             A3+C#4+E4 Bb3+Eb4 A3+C#4+E4 Bb3+Eb4+G4 A3+C#4+E4 Bb3 A3+C#4+E4 Bb3+G4 A3+C#4+E4 \
             Bb3+G4 C#4+E4 G4 A3+C#4+E4 | Bb3+Eb4 E4 Bb3+Eb4+G4 A3+C#4+E4 Bb3+Eb4 A3+C#4+E4 \
             Bb3+Eb4+G4 A3+C#4+E4 Eb4+G4 A3+C#4+E4 Eb4+G4 A3+C#4 Eb4 A3+C#4+E4 Eb4+G4 A3+C#4+E4 \
             & C#3:1 -:7 Eb4:8 | (3 C#4:2 - - (3 A4 Bb4 Eb5 (3 C#5 A4 E4 -:4 & -:2 (3 -:1 - \
             B4+F5 G4+C#5 -:2 B4+F5:1 G4+C#5 -:5 (3 -:1 - B4+F5 | (3 G4+C#5 - - (3 - G5. _ _ -:5 \
             (3 -:1 - B5. (3 _ _ - -:2 & -:2 (3 -:1 - B5. _:2 -:1 B5:2. -:5 (3 -:1 - B5. | (3 _ \
             _ - (3 G5+D6 E5+B5 C5 G4+C#5 -:5 (3 -:1 - B4+F5 (3 G4+C#5 - - (3 - - B4+Eb5+B5 & \
             -:3 E5+B6:1 (3 G5 _ - (3 - - E5+Eb6+B6 -:7 E5+B6:1 | (3 G5 _ - (3 - - E5+Eb6+B6 -:6 \
             (3 -:1 - E5+Eb6+B6 -:3 E5+B6:1",
        )
        .in_time(COMMON)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 764 to 768.
        Part::new(
            "The fight",
            "G3:2 B3 D4 G4 E4 C4 | G3 B3 D4 G4 E4 C4 | G3 B3 D4 G4 E4 C4 | G4 E4 C4 E4 C4 G3 | \
             G3 B3 D4 G4 G3 B3:1 -",
            "",
        )
        .with_left(
            "F#2+E4:1 -:11 | -:12 | - | - | - & -:3 B3:1 Bb3:2 Eb4 Bb3+D4 F#3 | -:1 G3 -:10 | \
             -:1 G3 -:10 | -:11 G3:1. | _ G3 -:5 G4:1 - G3 A3:2 & F#3+C#5+G5:1 - Bb3:2 C#4 F#4 \
             C#4 Bb3 | F#3 Bb3 C#4 F#4 C#4 Bb3 | F#3 Bb3 C#4 F#4 C#4 Bb3 | F#4 C#4 Bb3 F#4 C#4 \
             Bb3 | F#3 Bb3 C#4 F#4 F#3 Bb3:1 - & -:1 G3 F#3:2 -:1 D4:2. G4:1 - E4 - C4. | _ -:2 \
             B3:1 - D4:2. G4:1 - E4 - C4. | _ -:2 B3:1 - D4:2. G4:1 - E4 - C4. | _ G4 - E4 - \
             C4:2. E4:1 - C4 -:2 | -:3 B3:2. D4:1 F4:2 F3 -:1 B3 & G4:1 -:11 | -:12 | - | - | -",
        )
        .in_time(SIX_EIGHT)
        .with_tempo(&[tempo(0, 150)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::BASSOON, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 769 to 772.
        Part::new(
            "Clarinet",
            "D4:2 G4 B4 C5 G4 E4:1 - | -:6 C5:2 G4 E4:1 - | -:6 C5:2 G4 E4 | E5 C5 G4 C5 G4 E4:1 \
             -",
            "",
        )
        .with_left(
            "C#4:2 F#4 Bb4 C#5 Bb4 F#4:1 - | Bb4+C5:2 F#4+G4 C#4+E4:1 - C#5:2 Bb4 F#4:1 - | \
             Bb4+C5:2 F#4+G4 C#4+E4:1 - C#5:2 Bb4 F#4 | C#5 Bb4 F#4 C#5 Bb4 F#4:1 - & -:1 D4 - \
             G4 - B4 - C5 - G4 E4:2 | -:5 E4:1 - C5 - G4 E4:2 | -:5 E4:1 - C5 - G4 - E4 | - E5 - \
             C#5 - E4 - C5 - G4 E4:2",
        )
        .in_time(SIX_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)]),
        // MIDI bars 773 to 778.
        Part::new(
            "Petrushka's call",
            "-:2 F#4:1 B4:2 D5:1 F#5 -:5 | F#4:1 B4 D5 F#5:2 -:7 | C5:2 C#5:1 F#5:2 E5:1 C5 -:5 \
             | E5:1 C#5 G4 Bb4:2 -:7 | -:12 | -:6 C5:1 C#5 F#5 E5:2 -:1",
            "",
        )
        .with_left(
            "-:12 | - | - | -:3 G2 G2:6 | -:12 | - & -:12 | - | - | -:3 G3 -:6 | G3:1 - C4 C#4 - \
             Bb3 -:6 | -:12 & -:12 | - | -:7 G4:2 Bb4:1 -:2 | -:3 C#4:2 C4+G4:1 G3+C#4+F#4+F#5 \
             -:5 | -:1 F#4 -:2 E4:1 - G4 -:4 Bb4:1 | - F#4+D5 -:9 Bb4+B4:1 & G4:1 F#4 G4+B4 \
             F#4+G4 F#4+G4+D5 F#4+G4 F#4+G4+F#5+G5 F#4+G4 F#4+G4+E5 F#4+G4 F#4+G4+C5 F#4+G4 | \
             F#4+G4 F#4 F#4+G4+B4 F#4+G4 F#4+G4+D5 F#4+G4 F#4+G4+G5 F#4+G4 F#4+G4+E5 F#4+G4 \
             F#4+G4+C5 F#4+G4 | G4 F#4 F#4+G4+B4 F#4+G4 F#4+G4+D5 F#4+G4 F#4+G4+G5 F#4 F#4+E5 \
             F#4+G4 F#4+G4+C5 F#4+G4 | F#4+G4+E5 F#4+G4 F#4+C5 G4 G4 F#4 G4+G5 F#4+G4 F#4+G4+E5 \
             F#4+G4 F#4+G4+C5 G4 | G4 G4 F#4+G4+Bb4+C5 F#4+G4+Bb4+C5 F#3+G4+Bb4+C5+C#5+E5 \
             F#3+Bb4+C5+C#5+E5 Bb3+C5+C#5+E5+F#5+G5 Bb3+C#5+E5+G5 F#3+F#4+G4 F#4+G4 \
             F#4+G4+Bb4+B4 F#4+G4+B4 | F#3+Bb4+B4+D5 F#3+Bb4+B4+C#5 F#3+F#4+G5 F#4+G4 - \
             F#4+G4+Bb4+B4 - F#3+E4+G4+Bb4+C5 - E4+F#4+G4+Bb4 - E4+F#4 & -:12 | - | - | -:11 \
             F#4:1. | _ -:6 C5:2. -:3 | -:1 G4:2. -:1 F#4+G4+Bb4+B5 - F#3+E4+Bb4+C#5+C6+C#6 - \
             F#3+E4+F#4+G4+Bb4+G5+Bb5 - E4+F#4+F#5 - & F#4:1 G4 Bb4 - C#5 -:2 G5:1 C#5 E5 Bb4 C5 \
             | - G4 Bb4 B4 C#5 D5 F#5 G5 C#5 E5 Bb4 C5 | F#4 G4 Bb4 B4 C#5 D5 F#5 G5 C#5 E5 Bb4 \
             C5 | F#5 E5 C4+C#5 F#4+C5:2. -:1 C4+C5:2. - Bb4:1 C5 | -:8 B4:1 C#5 D5 - | \
             G4+C#5+G5 -:5 G4+F#5:1 - B4+C5 C#5 C#5+E5 - & -:12 | - | - | -:4 Bb4:1 -:2 G5:1 C#5 \
             E5 -:2 | -:7 F#5:1 -:4 | -:2 D5+F#5:1 Bb4+A5 B4+Bb5 C#5+G5 - F#5+G5 -:4",
        )
        .in_time(SIX_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 779 to 783.
        Part::new(
            "Flutes",
            "-:6 G5:1 F#6 C6 C#6 E6 Bb5 | G6 C#6 E6 F#6 C6 Bb6 -:6 | G6:1 C#6 E6 F#6 C6 Bb6 -:6 \
             | Bb5:1 E5 C#6 G5 F#6 C6 Bb5:2. -:4 | Bb5:1 C6 C#6 E6 F#6 G6 C#6 C6 F#6 E6 Bb6 G6",
            "",
        )
        .with_left(
            "-:4 C5:1 - Bb4 -:3 (3 -:1 C5 _ | C#5 -:3 G4:1 Bb4 C5 -:3 C5:1 G4 | Bb4 - C5 Bb4 F#4 \
             C#4 E4 -:5 | -:8 G4:1 -:3 | G4:2 - F#4:1 C#4 E4 -:5 & C#3:1 C#3+C5 C#3 G4 E4 E4 \
             F#3+C5 F#3+C5 F#3+G4 G4 E4 E4 | C5 C5 C5 - C5 C5 -:2 C5:1 C5 - C5 | C5 C5 -:2 C5:1 \
             C5 C5 C5 C5 C5 C5 C5 | C5 -:3 C5:1 C5 C5 - C5 C5 C5 C5 | C5 C5 - C5 C5 C5 C5 - C5 \
             C5 C5 C5 & -:6 F#4:1 G4 C5 Bb4 F#4 C#4 | F#4 G4 Bb4 C5 C#5 F#4+E5 G4+F#5+G5 C5+F#6 \
             Bb4+C6 F#4+C#6 C#4+E6 E4+Bb5 | -:3 C5:1 C#5 E5 F#5+E6 Bb5 C6 C#6 G5 F#6 | -:3 Bb3:1 \
             (3 _ _ C4 A3+C#5 -:5 | C#5:1 - C4+E5+F#5 Bb3+F#5+G5 - C#5+E5 C#5+E5 -:4 F#5+G5:1 & \
             (3 C5:1 - G4 - E5:2 -:7 | -:1 F#5 E5 -:6 E5:1 -:2 | -:12 | -:2 C5:1 C#5 - F#5 E5 C5 \
             - Bb4:2 -:1 | -:2 C5:1 Bb4 -:8 & F#4+C#5+G5:1 D5+C#6 G4+F#5+C6 C5+A5+Bb5 Bb5+E6 \
             Bb4+F#5+G5 F#5:2. -:4 | E4:3 -:9 | F#4:1 G4 Bb4 -:9 | C#5:1 C5 C#4+Bb4 C5 (3 C#5 E5 \
             _ - G5 C#6 C6 F#6 E6 | _ E5+C#6 -:2 F#5+G5:1 -:2 C5+C#6:1 F#5+G5 E5+Bb5 Bb5+C6 -",
        )
        .in_time(SIX_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 784.
        Part::new(
            "The chase",
            "-:2 Bb6 Bb6 Bb6. - Bb6:1 -",
            "",
        )
        .with_left(
            "B1:2 -:10 & B2:2 -:10 & F2:2 -:10 & -:2 B3 B3 B3 - B3 & -:2 (3 F#4:1 F4 _ -:8 & -:2 \
             B4. (3 F#4+Bb4:1 F4+B4 _ (3 F#4+Bb4 F4+B4. _ (3 _ - - (3 F#4+Bb4 F4+B4 - & (3 -:1 - \
             F#4+Bb4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ \
             F#4+F#5+Bb5+Bb6 G5+F6:2. (3 -:1 - F#4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ \
             F#4+Bb4+F#5+Bb5+Bb6",
        )
        .with_tempo(&[tempo(0, 100)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Kick, "x:12"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 785.
        Part::new(
            "The chase, bar 2",
            "B6:2 Bb6 Bb6:1 -",
            "",
        )
        .with_left(
            "B3:2 B3 - & (3 F#4:1 F4 _ (3 F#4 F4 _ -:2 & B4:2 B4 F4+G4+B4+F5 & (3 \
             G4+F5+G5+B5+F6:1 _ F#4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 \
             B3+F#4+G5+B5+F6 -",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 786.
        Part::new(
            "Trumpet call",
            "-:2 A4 A4:1 -",
            "",
        )
        .with_left(
            "-:2 D3 D3 & (3 -:1 - B3+B4 (3 A3 _ G4 A3+Eb4+A4:2 & -:1 G4 B3+Eb4+B4:2 B3+F#4+B4:1 \
             -",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 787.
        Part::new(
            "The chase 2",
            "-:2 Bb6 Bb6:1 -",
            "",
        )
        .with_left(
            "-:2 B3 - & -:2 (3 F#4:1 F4 _ -:2 & -:5 F#4:1 & -:2 B4. B3+G4+Bb4+B5+F6:1 - & -:4 \
             F4+B4+F5:2 & (3 -:1 - F#4+Bb4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 -:2",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 788.
        Part::new(
            "Trumpet call 2",
            "A4:2 A4 A4:1 -",
            "",
        )
        .with_left(
            "D3:2 D3+A3+Eb4 D3 & A3:2 - (3 G4:1 A3+Eb4+A4 B3+G4+B4 & B3+Eb4+B4:2 B3+G4+B4 \
             B3+F#4+B4:1 F4",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 789.
        Part::new(
            "The chase 3",
            "-:6",
            "",
        )
        .with_left(
            "D3:2 -:4 & A3+B3+Eb4+A4+B4:1 -:5 & -:4 (3 -:1 - F#4+Bb4+F#5+Bb5",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 790.
        Part::new(
            "The chase 3, bar 2",
            "B6:2 Bb6 Bb6 Bb6:1 -",
            "",
        )
        .with_left(
            "B3:2 B3 B3 B3 & (3 F#4:1 F4 _ (3 F#4 F4 _ (3 F#4 F4 _ (3 - F4 - & B4:2 B4 B4 \
             G4+B4+F5 & (3 G4+F5+G5+B5+F6:1 _ F#4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ \
             F#4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 (3 F#4+G5+B5+F6 - -",
        )
        .in_time(FOUR_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 791.
        Part::new(
            "The chase 3, bar 3",
            "(3 -:1 - Bb6 (3 B6 - - -:2",
            "",
        )
        .with_left(
            "-:2 B3 - & -:2 G4+B4+F5 - & (3 -:1 - F#4+Bb4+F#5+Bb5 (3 F#4+G5+B5+F6 F4 - -:2",
        )
        .in_time(THREE_EIGHT)
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 792 to 793.
        Part::new(
            "The chase 3, bar 4",
            "-:2 Bb6 Bb6 Bb6 | Bb6 Bb6 Bb6:1 -:3",
            "",
        )
        .with_left(
            "B2:2 -:6 | -:8 & -:2 B3 B3 B3 | B3 B3 -:4 & (3 B1:1 - - -:2 (3 F4+B4:1 - - -:2 | B4 \
             Bb4+B4 F4+Bb4+B4:1 -:3 & F2:2 -:6 | -:4 G4+F5:2 - & -:2 (3 F#4:1 F4 _ -:4 | (3 \
             F#4:1 F4 _ (3 F#4 F4 _ -:4 & -:2 B4. (3 F#4+Bb4:1 F4+B4 _ (3 F#4+Bb4 F4+B4 - | -:8 \
             & (3 -:1 - F#4+Bb4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 (3 \
             G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ F#4+Bb4+F#5+Bb5+Bb6 | (3 \
             G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 (3 G4+F5+G5+B5+F6 _ F#4+F#5+Bb5+Bb6 \
             B3+F#4+G5+B5+F6 -:3",
        )
        .in_time(FOUR_EIGHT)
        .with_tempo(&[tempo(14, 126)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Kick, "x:8 | -"),
        ])
        .marked(Dynamic::Fortissimo),
    ],
    form: &["Drum roll", "The Moor", "The Moor, bar 2", "Flourish", "Piano and violins", "Piano and violins, bar 2", "Trombones", "Trombones, bar 3", "Timpani and strings", "Timpani and strings, bar 2", "Timpani and strings, bar 3", "Piccolo", "Bassoon", "Bassoon, bar 2", "Bassoon", "The Moor's tune", "Cor anglais", "Bassoons", "Bassoons held", "Cor anglais again", "Bassoons again", "Horn", "Piccolo flourish", "Piccolo flourish, bar 2", "Trumpets", "Rush", "Oboe", "Oboe, bar 2", "Trumpets again", "Oboes", "Trumpet", "The Moor's tune, bassoon", "Side drum", "Dance of the Ballerina", "Strings", "Clarinets", "Flute", "Flute, bar 2", "Waltz", "The Moor's waltz", "The Moor's tune, cor anglais", "The Moor's tune, cor anglais 2", "Horns", "Horns, bar 2", "Horns, bar 3", "Pizzicato cellos", "Pizzicato cellos, bar 3", "Waltz again", "Petrushka comes", "Petrushka comes, bar 9", "Petrushka comes, bar 11", "Flutes and clarinets", "The fight", "Clarinet", "Petrushka's call", "Flutes", "The chase", "The chase, bar 2", "Trumpet call", "The chase 2", "Trumpet call 2", "The chase 3", "The chase 3, bar 2", "The chase 3, bar 3", "The chase 3, bar 4"],
    drums_from: 0,
    about: "The third tableau of Petrushka (1911, revised 1947): the Moor's room, a gaudy \
            chamber in the Charlatan's booth. A side drum roll links it to the second tableau. \
            The opening is hurled out fortissimo: the whole orchestra hammers the fifth G sharp \
            and D sharp, each beat ending on A and E, half a step above, in G sharp Phrygian \
            (the minor mode whose second step is a half step above the tonic); after a quick \
            flourish come an ostinato of F sharp minor and G major (F sharp Phrygian) on the \
            piano and strings and the trombones' chromatic descent. In a slow, whole-tone haze \
            over the timpani the Moor lies on his couch playing with a coconut. His tune, on the \
            clarinet and bass clarinet two octaves apart, is in B flat Lydian (the major scale \
            with a raised fourth, E natural) over a pizzicato pedal on B flat, with cymbal and \
            bass drum. The cor anglais laments over a bass of E, C and D; he cannot split the \
            coconut, and the music grows frantic in bars of 2/8, 3/8 and 4/8, keeping to no key; \
            he takes it for a god and bows to it, and the bassoons take up his tune again. A \
            drum roll: the Ballerina enters. Her dance is a trumpet tune in F major (a cornet in \
            1911) over the side drum alone. Then she and the Moor waltz to two waltzes by Joseph \
            Lanner, in E flat major, the trumpet and the flute over a bassoon's oom-pah; the \
            Moor joins in, faster, in B major, the flutes trilling over the harp while the cor \
            anglais and contrabassoon plod through his tune, in B major and then G sharp minor; \
            the waltz comes back in E flat. Petrushka bursts in: his trumpet call in B minor \
            sounds against chords of E flat major, and in the fight the Moor's C major and G \
            major stand against F sharp in the bass, the tritone of Petrushka's chord again. \
            These bars, like the Moor's frantic ones, are in two keys at once or in none, and \
            are marked chromatic. The tableau ends on the chase, a shrill trill of B flat and B \
            over the tritone B and F, cut by the trumpet's A over E flat, as the Moor throws \
            Petrushka out.",
};

// Copyright: this is Stravinsky's 1947 revision of Petrushka, which is still under
// copyright (Boosey & Hawkes); it is added here at the request of the project's owner.
// Source: Stravinsky's Petrushka, 1947 revision, fourth tableau, "The Shrovetide Fair (Towards
// Evening)", MIDI bars 794 to 1194: from the drum roll that links it to the third tableau up to
// the MIDI's last bar. The whole tableau fits in 99 patterns, so the Wet-nurses' Dance and the
// Coachmen's Dance stay in it. The notes are converted by script from a MIDI file of the 1947
// version (the person's own copy, at sounding pitch) and checked bar by bar against that file,
// not against a printed score; MIDI bar numbers are given below (MIDI bar 1 is a silent
// lead-in). The melody is the line in front, in the octave of the instrument that has it, and
// played_by follows it from instrument to instrument. Everything else that matters is written
// out on the Keys channel: the harmony, the bass, the ostinati and the timpani; lines doubled
// in unison or octaves are written once. Rhythms are quantised to sixteenths and eighth-note
// triplets; tremolos and rolls are written as held notes, grace notes are left out. The dynamic
// of each part is read from the MIDI's velocities. The percussion is written out (triangle as
// Ride, tambourine as Shaker, bass drum as Kick, side drum as Snare, cymbals as Crash). Where
// the meter changes, every run of bars in one meter is a part of its own, named "..., bar N"
// after the bar it starts on, with in_time: in_meters cannot cut a written-out left hand of
// several voices into its runs, nor carry drums. No chord symbol is honest for this music,
// which moves in drones, ostinati and parallel chords, so the charts are empty. Unlike the
// first tableau, a tremolo between two chords (32nd notes taking turns) is written as the two
// chords taking turns in sixteenths, and a run of 32nd notes keeps every other note. Where a
// part starts on a chord held over from the bar before, the chord is struck again.
//
// Left out: the piano, which doubles the orchestra, in bars 797 to 805 and 824 to 831, and the
// piano and the harp in bars 1054 to 1065. Where many lines play at once, the Keys channel
// keeps seven of them and leaves out the rest, which mostly double these: in bars 806 to 817 it
// keeps the first oboe, the ostinato of the clarinets, bass clarinet and first bassoon, the
// first trumpet and the cellos (leaving out the piccolo, the second flute, the second oboe, the
// cor anglais, the horns, the second trumpet, the piano, the celesta, the harp, the violins and
// the violas, also pizzicato); in bars 838 to 844 the first clarinet, the first bassoon, the
// trumpets, the first violins and the pizzicato violas and cellos (leaving out the second oboe,
// the cor anglais, the second clarinet, the second bassoon and the pizzicato second violins);
// in bars 845 to 852 the first oboe, the first clarinet, the first bassoon, the trumpets and
// the pizzicato violas and cellos (leaving out the piccolo, the flutes, the second oboe, the
// cor anglais, the second clarinet, the bass clarinet, the second bassoon, the second horn, the
// violins, the double basses and the pizzicato second violins); in bars 853 to 860 the first
// oboe, the trumpets, the first trombone, the first violas, the second cellos and the double
// basses (leaving out the flutes, the second oboe, the cor anglais, the horns, the tuba, the
// violins, the second violas and the first cellos); in bars 1066 to 1103 the third flute, the
// second oboe, the clarinet, bass clarinet and first bassoon, the second horn and the double
// basses (leaving out the second flute, the first oboe, the cor anglais, the second clarinet
// and bassoon, the contrabassoon, the first horn, the trumpets, the trombones, the tuba, the
// piano, the harp and the other strings); in bars 1113 to 1122 the oboes, the cor anglais, the
// first horn, the trumpets and the double basses (leaving out the second and third flutes, the
// clarinets, the bassoons, the second horn, the trombones, the tuba, the piano, the harp and
// the other strings); in bars 1147 to 1158 the first clarinet, the bass clarinet, the first
// horn, the first trumpet, the harp, the first violins and the pizzicato cellos (leaving out
// the flutes, the oboes, the cor anglais, the second clarinet, the second horn, the piano, the
// other strings with the bow and the other pizzicato strings); in the closing tremolo the
// flutes, the cor anglais, the second clarinet and the first violins (bars 1186 to 1191), then
// the flutes, the first horn, the first trumpet, the piano, the first violins and the cellos
// (bars 1192 and 1193), leaving out the other woodwinds, brass and strings, which play the same
// two chords. Grace notes are left out throughout.
//
// Not certain: the meters are the MIDI's: 4/4 in bars 794 to 796, 861 to 895 and 1104 to 1122,
// 6/8 in bars 797 to 833, 912 to 933 and 958 to 969, 9/8 in bar 834, 6/4 in bars 835 to 860,
// 2/2 in bars 896 to 899, 3/2 in bars 900 to 911, 3/4 in bars 1041, 1126 to 1129 and 1140, and
// 5/8 in bars 1130 to 1139 and 1141 to 1158, where every eighth note is a beat, as the MIDI
// marks no grouping; the rest is 2/4. The peasant's quintuplet in bar 900 and the strings'
// dotted figures in bars 1123 to 1129 are written as the nearest triplets. The MIDI ends at bar
// 1194 without Petrushka's death, the policeman, the Charlatan and Petrushka's ghost, which
// close the ballet: its last bars (1180 to 1194) look like the short ending Stravinsky wrote
// for concert performance, which the MIDI does not name. In the last bar the run C, D, E, F
// sharp, in 32nd-note sextuplets, is written as sixteenth-note triplets a sixteenth early, so
// that it lands on the last chord on time. The keys: the bars that keep to no scale or sound
// two keys at once are marked chromatic (see the song's description): the drum roll, the
// peasant and the bear (bars 900 to 911), and in the masqueraders' music the chromatic clarinet
// (bars 1106 to 1112), the chords of F major against F sharp (1113 to 1122), the whole-tone
// strings (1123 to 1129), the clarinet and horns (1174 to 1179), the chords over a rising bass
// (1180 to 1185) and the ending on Petrushka's chord (1186 to 1194). The merchant's tune keeps
// to the white notes and is marked D Dorian, after the D minor chords under the gypsy girls'
// dance: it could as well be read in A minor or C major. The tutti in bars 824 to 831 is marked
// D major though its bass holds A for most of it (A Mixolydian has the same notes). The
// wet-nurses' tune in C (bars 888 to 899) is marked C Mixolydian, for its B flat over the pedal
// on C. The passage in A flat major (bars 1159 to 1173) is written with the tonic G sharp, its
// enharmonic name, and its notes are spelt with flats.
pub(super) const PETRUSHKA_FAIR_EVENING: Song = Song {
    name: "Petrushka, The Shrovetide Fair in the Evening",
    origin: "Igor Stravinsky, Petrushka, 1911, revised 1947",
    feel: "Orchestra",
    genre: Genre::Classical,
    tonic: D,
    mode: Major,
    time: TWO_FOUR,
    tempo: 126.0,
    comping: Comping::Held,
    bass: Bass::None,
    rhythm: "",
    band: Band::new(gm::FLUTE, gm::ACOUSTIC_GRAND_PIANO, gm::CONTRABASS),
    style: "",
    parts: &[
        // MIDI bars 794 to 796.
        Part::new(
            "Drum roll",
            "-:16 | - | -",
            "",
        )
        .with_left(
            "F2:1 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 | F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 \
             F2 F2 F2 F2 F2 F2 | F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2 F2",
        )
        .in_time(COMMON)
        .in_key(C, Chromatic)
        .with_drums(&[
            (Drum::Snare, "x:1 x x x x x x x x x x x x x x x | x x x x x x x x x x x x x x x x | \
                           x x x x x x x x x x x x x x x x"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 797 to 805.
        Part::new(
            "Evening",
            "-:12 | - | -:1 B5 A5 B5 C#6 B5 A5 G5 F#5 E5 D5 C#5 | -:12 | -:10 (3 A4:1 D5 F#5 | \
             A5:2. -:10 | - (3 A4:1 D5 F#5 | A5:2. -:10 | - (3 C#5:1 F#5 A5",
            "",
        )
        .with_left(
            "-:12 | - | - | -:10 D3:2. | -:12 | -:10 D3:2. | -:10 D3:2. | -:12 | -:10 (3 \
             D3+A3+B3+C#4:1 - - & -:12 | - | -:6 D4 | _:12 | _:10 -:2 | D4:12 | _:10 -:2 | D4:12 \
             | _ & -:12 | - | - | -:10 (3 B4+C#5:1 _ - | -:12 | -:10 (3 E5:1. _ - | A5:2 -:8 (3 \
             B4+C#5:1 _ - | -:12 | -:10 (3 E4+G4+B4:1 _ - & A5:12 | _ | _:8 -:4 | -:12 | - | \
             F#4+A4:1 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 (3 D4+F#4+A4+D5+F#5 F#4+A4 - \
             | F#4+A4 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 (3 F#4+A4+D5+F#5 _ - & \
             D4+F#4+G4+B4:1 E4+G4+A4+C#5 G4+B4+C#5+E5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | \
             F#4+A4+D5+F#5 G4+B4+C#5+E5+G5 F#4+A4+D5+F#5 G4+B4+C#5+E5+G5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5+G5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5 G4+B4+C#5 F#4+A4+F#5 \
             E4+G4+B4+E5 | F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 | F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 (3 \
             D4+F#4+A4+D5+F#5 F#4+A4 G4+B4+C#5+D5+E5 | D5+F#5:2. -:10 | - (3 -:1 - \
             G4+B4+C#5+D5+E5 | D5+F#5:2. -:10 | - (3 -:1 C#5 G4+B4+C#5+E5+F#5 & A4+C#5+D5:1 \
             B4+E5 E4+G5 E4+G5 D4 E4+G5 D4 E4+G5 D4 E4+G5 D4 E4+G5 | D4 E4+G5 D4 E4+G5 D4 E4+G5 \
             D4 E4+G5 D4 E4+G5 D4 E4+G5 | D4 E4+D5 D4+E5 E4+D5 D4+E5 E4+D5 E5 E4+D5 E5+A5 \
             E4+D5+G5 B4+C#5+E5 D5+G5 | A5 E4+G5 A5 E4+G5 A5 E4+G5 A5 E4+G5 A5 E4+G5 A5 E4+G5 | \
             - C#4+E4+G5+B5 A5 C#4+E4+G5+B5 A5 C#4+E4+G5+B5 A5 E4+G5 A5 E4+G5 D3+A5 E3+E4+G5 | - \
             E4+A4+G5 A5 E4+A4+G5 A5 E4+A4+G5 A5 E4+A4+G5 A5 E4+A4+G5 A5 E4+A4+G5 | - E4+G5 A5 \
             E4+G5 A5 E4+G5 A5 C#4+E4+G5+B5 B3+C#6 C#4+E4+G5+B5 A5 E3+C#4+G5+B5 | - E4+G5 A5 \
             E4+G5 A5 E4+G5 A5 E4+G5 A5 E4+G5 A5 E4+G5 | A5 C#4+E4+G5+B5 A5 C#4+E4+G5+B5 B3+C#6 \
             C#4+E4+G5+B5 A5 E4+G5 A5 E4+G5 - E3+G5 & -:12 | - | - | -:10 (3 E5:1. _ - | A5:2 \
             -:10 | - (3 B4+C#5:1 _ - | -:10 (3 E5:1. _ - | -:12 | -:10 (3 E5:1 _ -",
        )
        .in_time(SIX_EIGHT)
        .with_tempo(&[tempo(0, 95)])
        .in_key(D, Major)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 806 to 817.
        Part::new(
            "Flutes and piccolo",
            "-:4 (3 C#6:1 E6 G6 F#6 A5:5 | _:8 (3 C#6:1 E6 G6 (3 F#6 _ - | -:4 A6:1 F#6 G5:2. \
             -:4 | -:12 | A6:1 B6 D6:2. - A6:1 B6 F#6:2. - | B6:1 A6 F#6:2. -:8 | A6:1 B6 F#6:2. \
             - B6:1 A6 F#6:2. - | - (3 A6:1 F#6 D6 A5:8 | _:2 - (3 C#6:1 E6 G6 (3 F#6 _ - -:4 | \
             -:8 (3 C#6:1 E6 G6 (3 F#6 _ - | -:4 A6:1 F#6 G5:2. -:4 | B6:1 A6 C#6:2 B6:1 F#6 A6 \
             C#6 B6:2 F#6:1 -",
            "",
        )
        .with_left(
            "D4:1 C#4 D4 C#4 B3 C#4 D4 E4 D4 E4 -:2 | -:3 E4:1 D4 C#4 D4 C#4 B3 C#4 D4 C#4 | D4 \
             E4 - E4 -:3 E4:1 D4 C#4 D4 C#4 | B3 C#4 D4 - D4 -:6 E4:1 | D4 C#4 D4 C#4 B3 C#4 D4 \
             - D4 -:3 | - E4:1 D4 C#4 D4 C#4 B3 C#4 D4 - | D4 -:6 E4:1 D4 C#4 D4 C#4 | B3 C#4 D4 \
             - D4 -:6 E4:1 | D4 C#4 D4 C#4 B3 C#4 D4 - D4 -:3 | - E4:1 D4 C#4 D4 C#4 B3 C#4 D4 - \
             | D4 -:6 E4:1 D4 C#4 D4 C#4 | B3 C#4 D4 - D4 -:6 E4:1 & -:6 F#4+A4+D5+F#5:1 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+C#5+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5+D6 \
             G4+B4+C#5+E5+B5 F#4+A4+D5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 | F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+D5+E5 F#4+A4+D5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 | F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 | F#4+A4+C#5+D5+F#5 G4+B4+C#5+E5+F#5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5+C#6 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5+C#6 G4+B4+C#5+E5+A5 | F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | F#4+A4+D5+F#5+C#6 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5+F#5 F#4+A4+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+C#5+D5+F#5 - | - G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+C#5+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+C#5+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 | \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5+D6 \
             G4+B4+C#5+E5+B5 F#4+A4+D5+F#5 G4+B4+C#5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 | F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+D5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 F#4+A4+D5+F#5 G4+B4+C#5+E5 \
             F#4+A4+D5+F#5 G4+B4+C#5+E5 & F#4+A4+D5+F#5+C#6:1 G4+B4+C#5+E5 F#4+A4+D5+F#5 \
             G4+B4+C#5+E5 F#4+A4+C#5+D5+F#5 G4+B4+C#5+E5 G5:2. -:4 | -:10 G5:2. | -:6 F#5:2. -:4 \
             | - F#5:2. -:6 | -:12 | -:2 A5. -:8 | -:12 | -:6 D5:2. -:3 G4+B4+C#5+E5+F#5:1 | \
             F#4+A4+D5+F#5+C#6 -:5 G5:2. -:4 | -:10 G5:2. | -:6 E5:2. -:4 | -:12 & -:12 | -:10 \
             (3 A5:1 A5 F#5+A5 | A5 -:9 (3 -:1 C#5 - | -:10 (3 C#5:1 C#5+E5 G5 | A5 -:5 (3 C#5:1 \
             C#5+E5 G5 A5 -:3 | -:12 | - | - | - | -:4 C#5:1 -:5 (3 A5:1 A5 F#5+A5 | A5 -:9 (3 \
             -:1 C#5 - | -:10 (3 C#5:1 C#5+E5 G5",
        )
        .in_time(SIX_EIGHT)
        .in_key(D, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 818 to 819.
        Part::new(
            "Oboe",
            "D5:1 E5 G5:2 F#5:1 E5 D5 E5 G5:2 F#5:1 E5 | D5 E5 B4:2 D5:1 F#5 E5 D5 B4 -:3",
            "",
        )
        .with_left(
            "B4:12 | _ & F#4:10 -:2 | -:12 & A4+F#5:1 -:5 D4:1 E4 G4:2 F#4:1 E4 | -:12 & D4+A5:1 \
             -:11 | -:12",
        )
        .in_time(SIX_EIGHT)
        .in_key(D, Major)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 820 to 823.
        Part::new(
            "Trumpets",
            "-:4 D5:1 E5 B4 D5 F#5:2 E5:1 D5 | Bb4 -:11 | -:12 | -:6 A5",
            "",
        )
        .with_left(
            "F#4:11 -:1 | -:12 | -:6 B3 | _ A3+C#4 & D4:12 | _ | _ | _:6 - & -:12 | -:6 G4 | \
             _:12 | _ & -:10 E4:1 F#4 | A4 G4 F#4:2 E4:1 F#4 C4 -:5 | Bb4:6 - | C#6:12 & -:12 | \
             - | -:6 B4+F#5 | _:11 -:1 & D5:12 | _ | E4+D5 | _ & A5:12 | _ | _ | _",
        )
        .in_time(SIX_EIGHT)
        .with_tempo(&[tempo(36, 84)])
        .in_key(D, Major)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 824 to 831.
        Part::new(
            "The whole orchestra",
            "F#6:1 E6 F#6 E6 D6 E6 F#6 G6 F#6 G6 A6 G6 | A6 B6 A6 G6 F#6 G6 F#6 G6 A6 G6 F#6 E6 \
             | F#6 E6 D6 E6 F#6 E6 F#6 E6 F#6 E6 F#6 E6 | D6 E6 F#6 G6 F#6 G6 A6 G6 A6 B6 A6 G6 \
             | F#6 G6 F#6 G6 A6 G6 F#6 E6 F#6 E6 D6 E6 | F#6 E6 F#6 E6 F#6 E6 F#6 E6 D6 E6 F#6 \
             G6 | F#6 G6 A6 G6 A6 B6 A6 G6 F#6 G6 F#6 G6 | A6 G6 F#6 E6 F#6 E6 D6 E6 F#6 E6 F#6 \
             E6",
            "",
        )
        .with_left(
            "D2:12 | _ | _ | _ | D2 | _ | _ | _ & A2:12 | _ | A2 | _ | A2 | _ | A2 | _ & \
             D3+F#3+A3:2 E4:1 -:3 G4:1 - F#4+A4:2 E4+B4 | - E4 A3+C4+F#4 G3+B3+E4 F#3+A3+D4 \
             E3+G3+B3 | F#3+A3 G3+B3 A3+C4 E4:1 - F#4+A4:2 G4+B4 | - E4 A3+C4+F#4 G3+E4 F#3+D4 - \
             | - G3+B3+E4 A3+C4+F#4 B3+G4 - E4:1 - | D4+A4:2 C#4+G4 A3+C4+F#4 G3+B3 F#3+A3+D4 \
             E3+G3+B3 | F#3+A3 G3+E4 A3:1 - E4:2 - G4:1 - | D4+F#4+A4:2 C#4+G4 A3+C4+F#4 G3+B3 \
             F#3+A3+D4 E3+G3+B3 & -:2 G3+B3 A3+C4 B3+E4 - G4:1 - | D4+F#4+A4:2 C#4+G4:1 -:9 | \
             -:2 E4:1 -:3 B3+G4:2 - E4:1 - | D4+A4:2 C#4+G4:1 -:5 A3:1 - E3+G3:2 | D3+F#3+A3 -:4 \
             E4:1 - F#4+A4:2 G4+B4 | - E4:1 -:3 E4:1 -:5 | -:4 C4+F#4:2 G4:1 - F#4+A4:2 E4+B4 | \
             - E4:1 -:3 E4:1 -:5 & -:12 | -:8 G4:1 - F#4+B4+C#5+E5:2 | F#4+D5+A5 G4+B4+C#5+E5 \
             D4+D5+F#5 B4+C#5+E5 - C#5+E5 | - B4+C#5+E5 - F#4+B4+C#5+E5 - F#4+B4+C#5+E5 | - G4:1 \
             -:9 | -:6 F#4+B4+C#5+E5:2 G4+D5+F#5 F#4+B4+C#5+E5 | F#4+D5+A5 B4+C#5+E5 D4+F#5+A5 \
             B4+C#5+E5 D5+A5 C#5+E5 | F#5 B4+C#5+E5 D5+A5 B4+C#5+E5 D5+F#5 B4+C#5+E5 & \
             F#4+D5+A5:2 G4+B4+C#5+E5 D4+D5+F#5 B4+C#5+E5 D5+A5 C#5+E5 | F#5+A5 B4+C#5+E5 D5+A5 \
             F#4+B4+C#5+E5 F#5+A5 - | -:8 D5+A5:2 - | D5+F#5 - D5+A5 - G4+F#5+A5 - | F#4+D5+A5 \
             B4+C#5+E5 D4+F#5+A5 B4+C#5+E5 D5+A5 C#5+E5 | D5+F#5 B4+C#5+E5 D5+A5 -:6 | -:2 G4:1 \
             -:9 | -:8 G4:1 -:3 & D4+F#5+A6:1 E4+B5+C#6+G6 D4+F#5+A6 E4+B5+C#6+G6 \
             F#4+A5+B5+C#6+F#6 E4+B5+C#6+G6 D4+F#5+A6 C#4+G4+E6+B6 D4+F#5+A6 C#4+G4+E6+B6 \
             B3+D5+F#6+C#7 C#4+G4+E6+B6 | B3+D5+F#6+C#7 A3+C#5+E5+G6+D7 B3+D5+F#6+C#7 \
             C#4+G4+E6+B6 D4+F#5+A6 C#4+G5+E6+B6 D4+F#5+A6 C#4+G4+E6+B6 B3+D5+F#6+C#7 \
             C#4+G4+E6+B6 D4+F#5+A6 E4+B5+C#6+G6 | D4+F#5+A6 E4+B5+C#6+G6 F#4+A5+B5+C#6+F#6 \
             E4+B5+C#6+G6 F#4+A5+B5+C#6+D6 G4+E5+A5+B5+C#6 F#4+A5+B5+C#6+D6 E4+B5+C#6+G6 \
             D4+F#5+A6 E4+B5+C#6+G6 D4+F#5+A6 E4+B5+C#6+G6 | F#4+A5+B5+C#6+F#6 E4+B5+C#6+G6 \
             D4+F#5+A6 C#4+G4+E6+B6 D4+F#5+A6 C#4+G4+E6+B6 B3+D5+F#6+C#7 C#4+G4+E6+B6 \
             B3+D5+F#6+C#7 A3+C#5+E5+G6+D7 B3+D5+F#6+C#7 C#4+G4+E6+B6 | D4+F#5+A6 C#4+G5+E6+B6 \
             D4+F#5+A6 C#4+G4+E6+B6 B3+D5+F#6+C#7 C#4+G4+E6+B6 D4+F#5+A6 E4+B5+C#6+G6 D4+F#5+A6 \
             E4+B5+C#6+G6 F#4+A5+B5+C#6+F#6 E4+B5+C#6+G6 | F#4+A5+B5+C#6+D6 G4+E5+A5+B5+C#6 \
             F#4+A5+B5+C#6+D6 E4+B5+C#6+G6 D4+F#5+A6 E4+B5+C#6+G6 D4+F#5+A6 E4+B5+C#6+G6 \
             F#4+A5+B5+C#6+F#6 E4+B5+C#6+G6 D4+F#5+A6 C#4+G5+E6+B6 | D4+F#5+A6 C#4+G4+E6+B6 \
             B3+D5+F#6+C#7 C#4+G4+E6+B6 B3+D5+F#6+C#7 A3+C#5+E5+D6+G6 B3+D5+F#6+C#7 C#4+G4+E6+B6 \
             D4+F#5+A6 C#4+G5+E6+B6 D4+F#5+A6 C#4+G4+E6+B6 | B3+D5+A5+F#6+C#7 C#4+G5+E6+B6 \
             D4+F#5+A6 E4+B5+C#6+G6 D4+F#5+A6 E4+B5+C#6+G6 F#4+A5+B5+C#6+F#6 E4+B5+C#6+G6 \
             F#4+A5+B5+C#6+D6 G4+E5+A5+B5+C#6 F#4+A5+B5+C#6+D6 E4+B5+C#6+G6",
        )
        .in_time(SIX_EIGHT)
        .in_key(D, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 832 to 833.
        Part::new(
            "Oboes and horns",
            "A5:2 G5 A5 G5 A5 G5 | A5 G5 A5 G5 A5 G5",
            "",
        )
        .with_left(
            "F#3+A3+D4:2 E3+G3+B3+E4 D3+F#3+A3+D4 E3+G3+B3+E4 D3+F#3+A3+D4 E3+G3+B3+E4 | \
             D3+F#3+A3+D4 E3+G3+B3+E4 D3+F#3+A3+D4 E3+G3+B3 D3+F#3+A3+D4 E3+G3+B3 & \
             F#4+A4+D5+F#5:1 - C#4+C#5+E5:2 F#4+D5+F#5 C#4+C#5+E5 F#4+D5+F#5 C#4+C#5+E5 | \
             F#4+D5+F#5 C#4+C#5+E5 F#4+D5+F#5 E4+C#5+E5 C#4+D5+F#5 E4+C#5+E5",
        )
        .in_time(SIX_EIGHT)
        .with_tempo(&[tempo(0, 95)])
        .in_key(D, Major)
        .played_by(&[(gm::OBOE, 0)]),
        // MIDI bar 834.
        Part::new(
            "Oboes and horns, bar 3",
            "A5:2 G5 A5 G5 A5 G5 A5 G5 A5:1 -",
            "",
        )
        .with_left(
            "D3+F#3+A3+D4:2 E3+G3+B3+E4 D3+F#3+A3+D4 E3+G3+B3+E4 D3+F#3+A3+D4 E3+G3+B3+E4 \
             D3+F#3+A3+D4 E3+G3+B3+E4 D3+F#3+A3+D4:1 - & F#4+D5+F#5:2 C#4+C#5+E5 F#4+D5+F#5 \
             C#4+C#5+E5 F#4+D5+F#5 C#4+C#5+E5 F#4+D5+F#5 C#4+C#5+E5 F#4+D5+F#5:1 -",
        )
        .in_time(NINE_EIGHT)
        .in_key(D, Major)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 835 to 837.
        Part::new(
            "Wet-nurses' Dance",
            "-:24 | -:8 C5:2 A4 A4 F4 F4 E4 F4 E4 | G4 F4 A4 A4 C5 Bb4 Bb4 G4 G4 E4 F4 E4:1 -",
            "",
        )
        .with_left(
            "-:2 A3 C4 A3 A3 A3 A3 A3 - A3 C4 A3 | A3 A3 A3 A3 (3 G4:1 _ G4 (3 F4 - - (3 - - E4 \
             (3 D4 - - -:8 | -:4 C4:2 A3 - A3 A3 A3 - A3 C4 A3+E4 & -:24 | -:10 A3:2 C4 A3 A3 A3 \
             A3 A3 | - A3 (3 -:1 - G4 (3 F4 - - (3 A3+G4 _ A4 (3 G4 - - -:12 & -:7 D4:2. \
             Bb3+D4:1 - Bb3:2. -:3 A3:1 -:7 | - Bb3:2. Bb3+D4:1 C4+E4 Bb3+D4 -:3 D4:2. Bb3+D4:1 \
             - Bb3:2. -:3 | A3:1 Bb3+D4 - Bb3+D4 -:4 C4+E4:1 - C4+E4 -:2 D4+F4:1 C4+E4 Bb3+D4+A4 \
             E4:2. - A3+E4+G4:1 Bb3+D4+A4 C4+G4 D4+F4 & A3+E4:1 Bb3+D4 C4+E4 Bb3+D4 A3+E4+G4 \
             Bb3+D4+A4 C4+E4+G4 F4 E4 - C4+E4 D4 C4+E4+G4 D4+F4 C4+E4+G4 Bb3+D4+A4 C4+E4:2. \
             C4+E4:1 - A3+E4+G4 Bb3+D4+A4 C4+E4+G4 F4 | C4+E4 Bb3+D4 - Bb3+D4 C4:2. - A3:1 -:7 \
             C4:2. C4:1 D4 -:4 | C4+E4:2. C4+E4:1 -:4 D4:2. Bb3+D4:1 - Bb3+D4 C4:2. - A3:1 \
             Bb3+D4 C4 Bb3+D4 -:4 & -:8 C4:2. -:7 Bb3+D4:1 - Bb3+D4 -:3 D4:1. | -:2 C4+E4:1 - \
             E4+G4 D4+F4 C4+E4+G4 D4+A4 E4:2. - A3+E4+G4:1 Bb3+D4+A4 C4+E4+G4 F4 E4 -:3 \
             C4+E4+G4:1 D4+F4 C4+G4 Bb3+D4+A4 | -:4 A3+E4+G4:1 Bb3+D4+A4 C4+E4+G4 F4 -:4 E4+G4:1 \
             -:11 & -:12 F4:1 - F4:2 D4:1 - D4 -:5 | -:2 D4:1 - F4 - F4:2 C4+D4:1 - D4 -:7 D4:1 \
             -:3 F4:2 | D4:1 - D4 -:7 D4:1 - F4 - F4:2 C4+D4:1 - D4 -:5 & C4+D4:1 - D4 -:7 D4:1 \
             - A4:2 A4:1 -:9 | -:4 A4:2 A4:1 -:13 A4:2 A4:1 - | -:12 A4:2 A4:1 -:9",
        )
        .in_time(SIX_FOUR)
        .with_tempo(&[tempo(0, 118)])
        .in_key(F, Major)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 838 to 844.
        Part::new(
            "The wet-nurses' tune",
            "-:8 C5:6 A4:2 Bb4:4 A4:2 G4 | F4:6 - C5:2 A4 D5:4 D5 | C5 C5:2 A4 D5:4 D5 C5 - | \
             -:24 | -:22 G5:1 A5 | Bb5:2 A5 A5 F5 F5 E5 F5 E5 G5 F5 A5 A5:1 C6 | Bb5:2 Bb5:1 Bb5 \
             A5:2 G5 G5 E5 F5 E5 G5 F5 G5 G5:1 A5",
            "",
        )
        .with_left(
            "A3+F4:2 A3+G4 A3+Bb4 A3+Bb4 D5. -:14 | -:24 | - | -:6 A3+C4:2 A3+F4 A3 A3 A3+C4 - \
             A3+F4 C4 A3 | -:14 A3+C4:2 -:8 | -:16 A3:2 A3 A3+C4 A3+C4 | A3 A3 A3 A3+C4 - A3 C4 \
             A3 A3 A3 A3 A3 & -:8 A3:2 A3 A3+C4 A3+C4 A3 A3 A3 A3+C4 | - A3 C4 A3 A3 A3 A3 A3 A3 \
             A3 A3+C4 A3+C4 | A3 A3 A3 A3+C4 - A3 C4 A3 A3 A3 A3 A3 | A3+C5 A3+A4 A3+C4 F4:1 -:9 \
             G4:2 -:6 | A3+C5:2 A3+Bb4 A3+Bb4 A3+G4 A3+G4 A3+E4 A3+C4+F4 - A3+G4 A3+F4 A3+G4 \
             A3+C4+G4:1 - | Bb4:2 A3+F5 C4+G5 A3+D5 A3+E5 A3+C5 A3+C5 A3+C5:1 -:9 | -:24 & C4:2. \
             -:5 Bb3+D4:1 -:8 C4+E4:1 Bb3+D4 C4+E4 Bb3+D4 C4:2. - | A3:1 Bb3 C4 Bb3 -:4 C4:2. \
             -:5 Bb3+D4:1 -:8 | C4+E4:1 Bb3+D4 C4+E4 Bb3+D4 C4:2. - A3:1 Bb3 C4 Bb3 -:4 C4:2. \
             -:5 Bb3+D4:1 | -:8 C4+E4:1 Bb3+D4 C4+E4 Bb3+D4 C4:2. - A3:1 Bb3 C4 Bb3 -:4 | C4:2. \
             -:5 Bb3+D4:1 -:8 C4+E4:1 Bb3+D4 C4+E4 Bb3+D4 C4:2. - | A3:1 Bb3 C4 Bb3 -:4 C4:2. \
             -:5 Bb3+D4:1 -:8 | C4+E4:1 Bb3+D4 C4+E4 Bb3+D4 C4:2. - A3:1 Bb3 C4 Bb3 -:4 C4:2. \
             -:5 Bb3+D4:1 & E4:1 D4 E4 D4 C4+E4 D4 C4+E4 - C4+E4:2. - G4:1 A4 G4 F4 -:4 E4:1 D4 \
             E4 D4 | E4:2. - A3+G4:1 Bb3+A4 C4+G4 D4+F4 E4 D4 E4 D4 C4+E4 D4 C4+E4 - C4+E4:2. - \
             G4:1 A4 G4 F4 | -:4 E4:1 D4 E4 D4 E4:2. - A3+G4:1 Bb3+A4 C4+G4 D4+F4 E4 D4 E4 D4 \
             C4+E4 D4 C4+E4 - | C4+E4:2. - G4:1 A4 G4 F4 -:4 E4:1 D4 E4 D4 E4:2. - A3+G4:1 \
             Bb3+A4 C4+G4 D4+F4 | E4 D4 E4 D4 C4+E4 D4 C4+E4 - C4+E4:2. - G4:1 A4 G4 F4 -:4 E4:1 \
             D4 E4 D4 | E4:2. - A3+G4:1 Bb3+A4 C4+G4 D4+F4 E4 D4 E4 D4 C4+E4 D4 C4+E4 - C4+E4:2. \
             - G4:1 A4 G4 F4 | -:4 E4:1 D4 E4 D4 E4:2. - A3+G4:1 Bb3+A4 C4+G4 D4+F4 E4 D4 E4 D4 \
             C4+E4 D4 C4+E4 - & D4:1 - C4+D4 - F4+A4:2 F4+A4 D4:1 D4 C4+D4 D4 E4:2 E4 D4:1 - D4 \
             - F4:2 F4+A4 | C4+D4:1 D4 D4 D4 E4:2 E4 D4:1 - C4+D4 - F4+A4:2 F4 D4:1 D4 C4+D4 D4 \
             E4:2 E4 | D4:1 - D4 - F4+A4:2 F4 C4+D4:1 D4 D4 D4 E4:2 E4 D4:1 - C4+D4 - F4+A4:2 \
             F4+A4 | D4:1 D4 C4+D4 D4 A4 - E4+A4:2 -:4 F4+G4:2 F4+G4 -:4 A4:1 - E4+A4:2 | -:4 \
             F4+A4:2 F4+A4 -:12 F4+A4:2 F4+A4 | -:12 F4+A4:2 F4+A4 -:8 | -:4 F4+A4:2 F4+A4 -:12 \
             F4+A4:2 F4+A4:1 - & -:24 | - | - | -:4 E4:2 - D4:1 - D4 -:5 C4+D4:1 D4 D4 D4 E4:2 - \
             | D4:1 - C4+D4 -:5 D4:1 D4 C4+D4 D4 E4:2 E4 D4:1 - D4 -:5 | C4+D4:1 D4 D4 D4 E4:2 \
             E4 D4:1 - C4+D4 -:5 D4:1 D4 C4+D4 D4 E4:2 E4 | D4:1 - D4 -:5 C4+D4:1 D4 D4 D4 E4:2 \
             E4 D4:1 - C4+D4 -:5",
        )
        .in_time(SIX_FOUR)
        .in_key(F, Major)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 845 to 852.
        Part::new(
            "Horn",
            "C5:6 A4:2 Bb4:4 A4:2 G4 F4:6 -:2 | -:24 | -:4 C5:2 A4 D5:4 D5 C5 C5:2 - | -:24 | \
             C5:6 A4:2 Bb4:4 A4:2 G4 F4:6 -:2 | -:4 C5:2 A4 D5:4 D5 C5 C5:2 - | G4:4 G4:2 A4 \
             G4:4 F4:2 E4 D4:4 C4 | F4 F4 G4 G4 C5:8",
            "",
        )
        .with_left(
            "A3:2 A3 A3+C4 A3+C4 A3 A3 A3 A3+C4 - A3 C4 A3 | A3 A3 A3 A3 A3 A3 A3+C4 A3+C4 A3 A3 \
             A3 A3+C4 | - A3 C4 A3 A3 A3 A3 A3 A3 A3 A3+C4 A3+C4 | A3 A3 A3 A3+C4 - A3 C4 A3 A3 \
             A3 A3 A3 | A3 A3 A3+C4 A3+C4 A3 A3 A3 A3+C4 - A3 C4 A3 | A3 A3 A3 A3 A3 A3 A3+C4 \
             A3+C4 A3 A3 A3 A3+C4 | - A3 C4 A3 A3 A3 A3 A3 A3 A3 A3 A3+C4 | A3 A3 A3 A3+C4 - A3 \
             C4 A3 A3 A3 A3 A3 & D4:1 D4 C4+D4 D4 E4:2 E4 D4:1 - D4 - F4:2 F4+A4 C4+D4:1 D4 D4 \
             D4 E4:2 E4 | D4:1 - C4+D4 - F4+A4:2 F4+A4 D4:1 D4 C4+D4 D4 E4:2 E4 D4:1 - D4 - \
             F4+A4:2 F4+A4 | C4+D4:1 D4 D4 D4 E4:2 E4 D4:1 - C4+D4 - F4+A4:2 F4+A4 D4:1 D4 C4+D4 \
             D4 E4:2 E4 | D4:1 - D4 - F4+A4:2 F4+A4 C4+D4:1 D4 D4 D4 E4:2 E4 D4:1 - C4+D4 - \
             F4+A4:2 F4+A4 | D4:1 D4 C4+D4 D4 E4:2 E4 D4:1 - D4 - F4:2 F4+A4 C4+D4:1 D4 D4 D4 \
             E4:2 E4 | D4:1 - C4+D4 - F4+A4:2 F4 D4:1 D4 C4+D4 D4 E4:2 E4 D4:1 - D4 - F4+A4:2 \
             F4+A4 | C4+D4:1 D4 D4 D4 E4:2 E4 D4:1 - C4+D4 - A4:2 F4+A4 -:1 D4 C4+D4 D4 E4:2 E4 \
             | D4:1 - D4 - A4:2 F4+A4 C4+D4:1 D4 D4 D4 E4:2 E4 D4:1 - C4+D4 - F4+A4:2 F4+A4:1 - \
             & C4+E4:2. -:5 F4:1 C4+E4+F5 Bb3+D4 - Bb3+D4 C4:2. -:1 D4 E4:2. C4+F5:1 - A3+G4+A5 \
             Bb3+A4 C4+G4+A5 D4+F4+C6 | E4+Bb5 D4 E4+A5 - C4+E4+A5 - C4+E4+F5 - F5:2 E5 G4+F5:1 \
             A4 G4+E5 -:3 C4+E4+F5:1 - E4+A5 D4 E4+A5 - | A3+Bb5 Bb3 - Bb3 -:4 C4:2. -:1 D4 - D4 \
             - Bb3+D4 C4+E4:2. -:4 (3 -:1 F4 _ | C4+E4+Bb5 Bb3+D4 C4+E4+A5 Bb3+D4 C4:2. -:1 D4 \
             E4:2. C4+E5:1 - A3+G4+F5 Bb3+A4 C4+G4+E5 D4+F4 E4+G5 D4 E4+F5 - C4+E4+A5 - C4+E4+A5 \
             Bb3+D4+C6 | Bb5:2 A5 G4+A5:1 A4 G4+F5 -:3 C4+E4+E5:1 - E4+F5 D4 E4+E5 - A3+G5 Bb3 - \
             Bb3 -:4 | C4:2. -:1 D4 - D4 - Bb3+D4 C4+E4:2. -:5 F4:1 C4+E4+G5 Bb3+D4 - Bb3+D4 \
             C4:2. (3 -:1 D4 _ | A3+Bb5 Bb3 - Bb3 -:4 C4:2. -:1 D4 - D4 - Bb3+D4 C4+E4:2. -:4 (3 \
             -:1 F4 _ | C4+E4+Bb5 Bb3+D4 C4+E4+A5 Bb3+D4 C4:2. -:1 D4 E4:2. C4+E5:1 - A3+F5 \
             Bb3+A4 C4+G4+E5 D4+F4 E4+G5 D4 E4+F5 - C4+E4+A5 - C4+E4+A5 Bb3+D4+C6 & Bb5:2 A5 \
             G4+A5:1 A4 G4+F5 -:3 C4+E4+E5:1 - E4+F5 D4 E4+E5 - A3+G5 Bb3 - Bb3 -:4 | C4:2. -:1 \
             D4 - D4 - Bb3+D4 C4+E4:2. -:5 F4:1 C4+E4+G5 Bb3+D4 - Bb3+D4 C4:2. -:1 D4 | E4:2. \
             C4+A5:1 - A3+G4+A5 Bb3+A4 C4+G4+F5 D4+F4 E4+F5 D4 E4+E5 - C4+E4+F5 - C4+E4+E5 - \
             G5:2 F5 G4+A5:1 A4 (3 G4+A5 - C6 | -:4 E4+A5:1 D4 E4+F5 - A3+F5 Bb3 - Bb3 -:4 C4:2. \
             -:1 D4 - D4 -:2 | C4+E4. -:5 F4:1 C4+E4+F5 Bb3+D4 - Bb3+D4 C4:2. -:1 D4 E4:2. \
             C4+F5:1 - A3+G4+A5 Bb3+A4 C4+G4+A5 D4+F4+C6 | E4+Bb5 D4 E4+A5 - C4+E4+A5 - C4+E4+F5 \
             - F5:2 E5 G4+F5:1 A4 G4+E5 -:3 C4+E4+F5:1 - E4+A5 D4 (3 E4+A5 - C6 | E4:2. C4+A5:1 \
             - A3+A5 Bb3+A4 C4+G4+F5 D4+F4 E4+F5 D4 E4+E5 - C4+E4+F5 - C4+E5 - G5:2 F5 G4+A5:1 \
             A4 (3 G4+A5 - C6 | -:4 E4+A5:1 D4 E4+F5 - A3+F5 Bb3 - Bb3 -:4 C4:2. -:1 D4 - D4 -:2",
        )
        .in_time(SIX_FOUR)
        .in_key(F, Major)
        .played_by(&[(gm::FRENCH_HORN, 0)]),
        // MIDI bars 853 to 860.
        Part::new(
            "Piccolo, horn and violins",
            "C7:6 A6:2 Bb6:4 A6:2 G6 F6:8 | C7:4 C7:2 A6 D7:4 D7 C7 C7:2 A6 | G6:4 G6:2 A6 G6:4 \
             F6:2 E6 D6:4 C6 | F6 F6 E6 D6 C6:8 | C7:6 A6:2 Bb6:4 A6:2 G6 F6:8 | C7:4 C7:2 A6 \
             D7:4 D7 C7 C7:2 A6 | G6:4 G6:2 A6 G6:4 F6:2 E6 D6:4 C6 | F6 F6 G6:2 F6 E6 D6 C7:8",
            "",
        )
        .with_left(
            "F2+C3:4 F2+C3:20 | F2+C3:4 F2+C3:20 | F2+C3:4 F2+C3:20 | F2+C3:4 F2+C3:20 | C2+G2:4 \
             C2+G2:8 F2+C3:4 F2+C3:8 | _:12 C2+G2:4 C2+G2:8 | _:12 F2+C3:4 F2+C3:8 | _:12 \
             C2+G2:4 C2+G2:8 & Bb3+D4+F4+A4:2 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 \
             Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 D4+F4+A4+C5 D4+F4+A4+C5 \
             D4+F4+A4+C5 D4+F4+A4+C5 | F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 \
             F4+A4+C5+E5 F4+A4+C5 F4+A4+C5+E5 F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 \
             D4+F4+A4+C5 | Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 \
             Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 \
             D4+F4+A4+C5 | F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5 \
             F4+A4+C5+E5 F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 | Bb3+D4+F4+A4 \
             Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 \
             Bb3+D4+F4+A4 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 | F4+A4+C5+E5 \
             F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5 F4+A4+C5+E5 F4+A4+C5 \
             D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 | Bb3+D4+F4+A4 Bb3+D4+F4+A4 \
             Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 Bb3+D4+F4+A4 \
             D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5 | F4+A4+C5+E5 F4+A4+C5+E5 \
             F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5+E5 F4+A4+C5 F4+A4+C5+E5 F4+A4+C5 D4+F4+A4+C5 \
             D4+F4+A4+C5 D4+F4+A4+C5 D4+F4+A4+C5:1 - & -:1 C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 \
             - C4+G4 - C4+G4 - C4+G4 - Bb4 - Bb4 - Bb4 - Bb4 | - G4+D5 - G4+D5 - G4+D5 - G4+D5 - \
             G4+D5 - G4+D5 - G4+D5 - G4+D5 - E4+Bb4 - E4+Bb4 - E4+Bb4 - E4+Bb4 | - C4+G4 - C4+G4 \
             - C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 - E4+Bb4 - E4+Bb4 - E4+Bb4 - E4+Bb4 \
             | - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - E4+Bb4 - E4+Bb4 \
             - E4+Bb4 - E4+Bb4 | - C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 \
             - E4+Bb4 - E4+Bb4 - E4+Bb4 - E4+Bb4 | - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - \
             G4+D5 - G4+D5 - G4+D5 - E4+Bb4 - E4+Bb4 - E4+Bb4 - E4+Bb4 | - C4+G4 - C4+G4 - C4+G4 \
             - C4+G4 - C4+G4 - C4+G4 - C4+G4 - C4+G4 - E4+Bb4 - E4+Bb4 - E4+Bb4 - E4+Bb4 | - \
             G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - G4+D5 - E4+Bb4 - E4+Bb4 - \
             E4+Bb4 - E4+Bb4 & Bb5:2 A5 A5 F5 F5 E5 F5 E5 G5 F5 A5 A5:1 C6 | Bb5:2 A5 A5 F5 F5 \
             E5 F5 E5 G5 F5 A5 A5:1 C6 | Bb5:2 A5 A5 F5 F5 E5 F5 E5 G5 F5 A5 A5:1 C6 | Bb5:2 A5 \
             A5 F5 F5 E5 F5 E5 G5 F5 A5 A5:1 C6 | Bb5:2 A5 A5 F5 F5 E5 F5 E5 G5 F5 A5 A5:1 C6 | \
             Bb5:2 A5 A5 F5 F5 E5 F5 E5 G5 F5 A5 A5:1 C6 | Bb5:2 A5 A5 F5 F5 E5 F5 E5 G5 F5 A5 \
             A5:1 C6 | Bb5:2 A5 A5 F5 F5 E5 F5 E5 G5 F5 A5 A5:1 -",
        )
        .in_time(SIX_FOUR)
        .in_key(F, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 861 to 864.
        Part::new(
            "Flutes in fourths",
            "A5:2 A5 D5 D5 G5 G5 D5 D5 | A5 A5 D5 D5 G5 G5 D5 D5 | A5 A5 D5 D5 G5 G5 D5 D5 | A5 \
             A5 D5 D5 G5 G5 D5 D5",
            "",
        )
        .with_left(
            "F4+A4:2 F4 C4+D4 C4+D4 E4+G4 E4+G4 C4+D4 C4+D4 | F4 F4 C4+D4 C4+D4 E4+G4 E4+G4 \
             C4+D4 C4+D4 | F4 F4 C4+D4 C4+D4 E4+G4 E4+G4 C4+D4 C4+D4 | F4 F4 C4+D4 C4+D4 E4+G4 \
             E4+G4 C4+D4 C4+D4 & -:1 G3+Bb4+D5 A3+A4+C5+F5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 | A3+A4+C5+F5 G3+Bb4+D5 A3+A4+C5+F5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 | A3+A4+C5+F5 G3+Bb4+D5 A3+A4+C5+F5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 | A3+A4+C5+F5 G3+Bb4+D5 A3+A4+C5+F5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5+E5 G3+Bb4+D5 A3+A4+C5 G3+Bb4+D5 A3+A4+C5 \
             G3+Bb4+D5 & F5:2 -:14 | -:16 | - | - & C5:2 -:14 | -:16 | - | -",
        )
        .in_time(COMMON)
        .in_key(D, NaturalMinor)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 865 to 868.
        Part::new(
            "Flutes and harp",
            "G5:2 G5:1 -:5 F#5:2 F#5:1 -:5 | G5:2 G5:1 -:5 F#5:2 F#5:1 -:5 | G5:2 G5:1 -:5 F#5:2 \
             F#5:1 -:5 | E5:2 E5:1 -:5 F#5:2 F#5:1 -:5",
            "",
        )
        .with_left(
            "A4+C5+F#5:1 G4+B4+D5 A4+C5+F#5 G4+B4+D5 A4+C5 G4+B4+D5 A4+C5 G4+B4+D5 A4+C5+E5 \
             G4+B4+D5 A4+C5+E5 G4+B4+D5 A4+C5 G4+B4+D5 A4+C5 G4+B4+D5 | A4+C5+F#5 G4+B4+D5 \
             A4+C5+F#5 G4+B4+D5 A4+C5 G4+B4+D5 A4+C5 G4+B4+D5 A4+C5+E5 G4+B4+D5 A4+C5+E5 \
             G4+B4+D5 A4+C5 G4+B4+D5 A4+C5 G4+B4+D5 | A4+C5+F#5 G4+B4+D5 A4+C5+F#5 G4+B4+D5 \
             A4+C5 G4+B4+D5 A4+C5 G4+B4+D5 A4+C5+E5 B4+D5 A4+C5+E5 B4+D5 G4+A4+C5 B4+D5 G4+A4+C5 \
             B4+D5 | G4+A4+C5+D5 B4+D5 G4+A4+C5+D5 B4+D5 A4+C5 B4+D5 G4+A4+C5 B4+D5 G4+A4+C5+E5 \
             B4+D5 G4+A4+C5+E5 B4+D5 G4:2 G4+G#4+D5:1 - & -:2 G5 -:6 F#5:2 -:4 | -:2 G5 -:6 \
             F#5:2 -:4 | -:2 G5 -:6 F#5:2 -:4 | -:2 E5 -:6 F#5:2 - D4 & -:4 D5:1 - D5 -:5 D5:1 - \
             D5 - | -:4 D5:1 - D5 -:5 D5:1 - D5 - | -:4 D5:1 - D5 -:5 D5:1 - D5 - | -:4 D5:1 - \
             D5 -:9",
        )
        .in_time(COMMON)
        .in_key(D, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Pianissimo),
        // MIDI bars 869 to 872.
        Part::new(
            "Oboes",
            "G5:2 G5 G5 G5 F#5 D5 D5 D5 | G5 G5 G5 G5 F#5:1 -:3 A5:2 F#5 | E5 E5 E5 E5 F#5 F#5 \
             A5 F#5 | E5 E5 E5 E5 F#5:1 -:7",
            "",
        )
        .with_left(
            "-:16 | - | - | -:12 F#4+F#5:1 G4+G5 - B4 & D4:2 D5:4 G4+D5:2 D4+F#4:1 - D5:4 D4:2 | \
             D4+G4:1 - D5:4 G4+D5:2 D4+F#4:1 - D5:4 F#4+D5:2 | D4+E4:1 - D5:4 E4+D5:2 D4+F#4:1 - \
             D5:4 F#4+D5:2 | D4+E4:1 - D5:4 E4+D5:2 D4+F#4:1 C#4+C#5 D5:4 A4+D5+A5:1 - & \
             G4+C#5:1 - C#5:2 C#6. -:5 C#7:1 D7:2. - | -:4 C#5:2 C#5 C5:1 -:3 Eb5:2 C5 | Bb4 Bb4 \
             Bb4 Bb4 C5 C5 Eb5 C5 | Bb4 Bb4 Bb4 Bb4 C5:1 -:6 B5:1 & -:4 D6 - D6 | - D6 - D6 | - \
             D6 - D6 | - D6 -:2 D4:1 E4+E5 D6:4 & -:4 C#5:2 C#5 C5 G#4 G#4 G#4 | C#5 C#5 C#6. \
             -:5 C#7:1 D7:2. - | -:4 C#6:2. -:5 C#7:1 D7:2. - | -:4 C#6:2. -:5 C#7:1 D7 -:3",
        )
        .in_time(COMMON)
        .in_key(D, Major)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 873 to 876.
        Part::new(
            "Flutes and piano",
            "B5:1 B5 B5 B5 B5 B5 B5 B5 A5 A5 G5 G5 G5 G5 G5 G5 | B5 B5 B5 B5 B5 B5 B5 B5 A5 A5 \
             A5 A5 D6 D6 B5 B5 | A5 A5 A5 A5 A5 A5 A5 A5 B5 B5 B5 B5 D6 D6 B5 B5 | A5 A5 A5 A5 \
             A5 A5 A5 A5 B5 -:7",
            "",
        )
        .with_left(
            "-:16 | -:9 B4:1 -:3 B4:1 B4 B4 | B4 B4 B4 B4 B4 B4 B4 B4 - B4 -:2 B4:1 B4 B4 B4 | \
             D5 D5 D5 D5 D5 D5 D5 D5 B4 -:7 & -:4 C4:2. C#5. D4 - D6:4 | D4:2 D5:4 Eb5:2. D4 - \
             D4. C#5. | D4 D5:4 D5:2 C#4. - D4. Eb5. | D4+Eb4:1 Eb4 D4 D4 C#4+C#5 C#4+C#5 C4+C5 \
             C4+C5 B3 -:7 & D4:2 D5:4 D5:2 C#4. - D4. Eb5. | Eb4. E5. E4. D5 Eb4. D5 D6:4 | \
             C#4:2. C5. C#4. C#5. D4 D5 D6:4 | -:16 & (3 G5:1 G5 _ -:2 C5+C6+D6. (3 -:1 - A5 -:2 \
             (3 E5:1 E5 _ - E5 D5+F#5:2. | (3 G5:1 G5 _ -:2 C5+C6+D6. (3 -:1 - A5 -:5 B5:1 \
             D5+F#5:2. | A4+A5. A4+E5. (3 E5+F#5:1 F#5 _ A4+E5:2. - B4+G5. - D5+F#5. | A4+A5. \
             A4+E5. -:6 F4+F5:4 F4+F5:2 & C5+C6:2. C5+G5. (3 G5:1 G5 _ C5+G5:2. B4+F#5+B5:1 F#5 \
             D5+F#5+G5:2. D5+E5+F#5+G5:1 - (3 E5+G5 E5 _ | C5+C6:2. C5+G5. (3 G5:1 G5 _ C5+G5:2. \
             B4+F#5+B5:1 F#5 B4+F#5+B5:2. B4+D5+G5+B5:1 - (3 G5 G5 _ | (3 E5+F#5 F#5 _ (3 F#5+A5 \
             F#5 _ A4+A5+D6:2. (3 F#5+A5:1 F#5 _ B4+G5+B5 G5 -:2 D5+A5+B5:1 B5 (3 G5 G5 _ | (3 \
             E5+F#5 F#5 _ (3 F#5+A5 F#5 _ A4+E5+F#5+A5+D6 F#5 A4+F#5+A5 F#5 F#5+G5 -:7 & -:3 \
             C6:1 -:3 C6:1 C6 B5 -:3 D5:1 -:2 | -:3 C6:1 -:3 C6:1 - B5 -:2 A5:1 A5 -:2 | -:1 E5 \
             -:3 E5:1 -:7 A5:1 -:2 | -:1 E5 -:3 E5:1 E5 E5 B5:2 -:6 & (3 -:1 - A5 (3 C6 - B5+C#6 \
             (3 - - A5 (3 C6 - - -:2 (3 -:1 - C#7 D7:2. (3 -:1 - B5 | (3 - - A5 (3 C6 - B5+C#6 \
             (3 - - A5 (3 C6 - - -:2 (3 -:1 - C#7 D7:2. - | - (3 -:1 - G5+C#6 -:2 (3 -:1 - A5 \
             -:2 (3 B5:1 - B5+C#7 D7:2. - | - (3 -:1 - G5+C#6 -:12",
        )
        .in_time(COMMON)
        .in_key(G, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 877 to 883.
        Part::new(
            "Trumpet tune",
            "-:2 F5:4 F5 F5 F5:2 | _ F5:4 F5 F5 F5:2 | _ F5:4 F5 F5:2 G4 F5 | C5 C5 C5 C5 B4 G4 \
             G4 G4 | C5 C5 C5 C5 B4:4 D5:2 B4 | A4 A4 A4 A4 B4 B4 D5 B4 | A4 A4 A4 A4 B4 - G5:1 \
             -:3",
            "",
        )
        .with_left(
            "-:16 | -:2 F#4+F#5:4 F#4+F#5 F#4+F#5 F#4+F#5:2 | D3:4 D3 D3 D3 | G2+D3 G2+D3+F4 \
             G2+D3+Eb4 G2+D3+Eb4 | G2+D3+F4 G2+D3 G2+D3+F4 G2+D3+Eb4 | G2+D3+Eb4 G2+D3+F4 G2+D3 \
             G2+D3+F4 | G2+D3+Eb4 G2+D3+Eb4 G2+D3+F4 G2+D3 & -:16 | G3:2 A3 G3 A3 G3 A3 G3 A3 | \
             G3 A3 G3 A3 G3 A3 G3 A3:1 - | -:16 | - | - | -:11 F#6:2. -:1 G5+G6:2 & -:16 | -:2 \
             F4 - F4 - F4 - F4 | - E4+E5 - D4+D5 - C4+C5 - B3+B4 | G3+G4 A3+A4 G3+G4 A3+A4 G3+G4 \
             A3+A4 G3 A3+A4 | G3+G4 A3+A4 G3+G4 A3+A4 G3+G4 A3+A4 G3+G4 A3+A4 | G3+G4 A3 G3+G4 \
             A3 G3+G4 A3+A4 G3+G4 A3+A4 | G3+G4 A3 G3+G4 A3 G3+G4 A3+A4 G3+G4 A3+A4 & -:16 | - | \
             -:2 F#4+F#5:4 G4+G5 G4+G5 G4+G5:2 | - F#4+F#5 F5 E4+E5 Eb5 D4+D5 Eb5 E4+E5 | F5 \
             F#4+F#5 G5 F#4+F#5 F5 E4+E5 Eb5 D4+D5 | Eb5 E4+E5 F5 F#4+F#5 G5 F#4+F#5 F5 E4+E5 | \
             Eb5 D4+D5 Eb5 E4+E5 F#5 F#4+F#5 D4+F4+D5+F5+G5 Eb4+G4+E5 & -:16 | - | - | - | - | - \
             | -:12 G6:2 -",
        )
        .in_time(COMMON)
        .in_key(G, Mixolydian)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 884 to 887.
        Part::new(
            "Tutti in G",
            "C7:1 B6 C7:2 C7:1 B6 C7:2 B6 G6 G6 G6 | C7:1 B6 C7:2 C7:1 B6 C7:2 B6:4 D7:2 B6 | A6 \
             F#6 F#6 F#6 B6 G6 D7 G6 | A6 F#6 A6 F#6 B6 -:6",
            "",
        )
        .with_left(
            "G2:2 D3 G2 D3 G2 D3 G2 D3 | G2 D3 G2 D3 G2 D3 G2 D3 | G2 D3 G2 D3 G2 D3 G2 D3 | G2 \
             D3 G2 D3 G2 -:6 & -:16 | -:8 G1:2 -:6 | -:16 | -:8 (3 -:1 A2 C3 (3 F#3 B3 E4 (3 \
             D4+A4 - F4 -:2 & Eb4+C5+Eb5:2 G2+D4+D5 C#4+C5+C#5 G2+D4+D5 B4 G2+E4+G4+E5 - \
             G2+D4+G4 | C#4+C5+C#5 G2+D4+D5 Eb4+C5+Eb5 G2+E4+E5 - G2+F#4 D5+G5 G2+F#4+B4 | A4+G5 \
             G2+F#4+F#5 A4 G2+E4 Eb4+B4+Eb5 G2+E4+B4+E5 D5+D6 G2+F#4+B4 | A4+G5 G2+F#4+F#5 \
             F4+A4+F5 G2 Eb4+Eb5 - (3 D5:1 C5+E5 F#4+B4+F5 (3 G4+A4+G5 G4+A4+A5 E4+B4+B5 & \
             G4+B4:1 B4 A3+A4+C5 C5 G4+B4 B4 A3+A4+C5 C5 G4+A4 A4 A3+A4 A4 G4+A4 A4 A3+A4 A4 | \
             G4+B4 B4 A3+A4+C5 C5 G4+B4 B4 A3+A4+C5 C5 G4+B4 B4 A3+A4+B4 B4 G4+B4 B4 A3+A4+C5 C5 \
             | G4+D5 D5 A3+A4+D5 D5 G4+D5 D5 A3+A4+D5 D5 G4+D5 D5 A3+A4+D5 D5 G4+C5 C5 A3+A4+D5 \
             D5 | G4:2 A3+A4+Eb5:1 Eb5 G4+D5 D5 A3+A4+C#5 C#5 G4+B4:2 -:6 & D3+G3+G5+C6:2. \
             G5+C6. (3 G1+G6+C7:1 _ B5+B6 G6:2 D3+G3+F#5+B5. D5+G5. G1+D6 - | D3+G3+G5+C6. \
             G5+C6. (3 G1+G6+C7:1 _ B5+B6 G6:2 D3+G3. -:6 | (3 G1+E6:1 _ F#5+F#6 (3 E6+A6 _ F#6 \
             D3+G3+E5+A5:2. E5+A5. (3 G1+F#6:1 _ G5+G6 F#6+B6:2 D3+G3+A5. F#5+B5. | (3 G1+E6:1 _ \
             F#6 E6+A6:2 D3+G3+E5+A5. E5+A5. G1+D3+G3+F#5+B5+F#6:1 -:7 & -:2 F#4+F#5:1 F#5 F4+F5 \
             F5 E4+E5 E5 Eb4+Eb5 Eb5 D4 - Eb4+Eb5 Eb5 E4+E5 E5 | F4+F5 F5 F#4+F#5 F#5 -:2 \
             F#4+F#5:1 F#5 F4+F5 F5 E4+E5 E5 Eb4+Eb5 Eb5 D4+D5 D5 | Eb4+Eb5 Eb5 E4 - F4+F5 F5 \
             F#4+F#5 F#5 G5 - F#4 - F4+F5 F5 E4+E5 E5 | Eb4+Eb5 Eb5 D4+D5 D5 Eb4+Eb5 Eb5 E4 - \
             F4+F5:2. - (3 -:1 E4+D5 - (3 B4 D5 F5 & (3 G1+G6+C7:1 _ B5+B6 G6:2 D3+G3+G5+C6. \
             G5+C6. (3 G1+F#6:1 _ F#6 D6:2 - D5+G5+D6 | (3 G1+G6+C7:1 _ B5+B6 G6:2 D3+G3+G5+C6. \
             G5+C6. F#5+B5+F#6:4 G1+B5+D6+A6:2 F#5+B5+F#6 | D3+G3+E5+A5. E5+A5. (3 G1+E6+A6:1 _ \
             F#5+F#6 E6+A6:2 D3+G3+F#5+B5. F#5+B5. (3 G1+A6:1 _ G5+G6 F#6+B6:2 | D3+G3+E5+A5. \
             E5+A5. (3 G1+E6:1 _ F#6 E6+A6:2 -:8",
        )
        .in_time(COMMON)
        .in_key(G, Major)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 888 to 895.
        Part::new(
            "Trumpets in C",
            "C5:2 C5 D5 D5 C5 C5 D5 D5 | C5 C5 D5 D5 C5 C5 D5 D5 | C5:1 D5 C5 D5 C5 D5 C5 D5 D5 \
             E5 D5 E5 D5 E5 D5 E5 | D5 E5 D5 E5 F5 -:3 C5:2 C5 D5 D5 | C5 C5 D5:1 E5 D5 E5 D5 E5 \
             D5 E5 D5 E5 D5 E5 | D5 E5 D5 E5 D5:2 - C5 C5 D5 D5 | C5 C5 D5 D5 C5 C5 D5:1 E5 D5 \
             E5 | D5 E5 D5 E5 D5 E5 D5 E5 D5 E5 D5 E5 D5 E5 D5 E5",
            "",
        )
        .with_left(
            "F4+A4:2 F4+A4 G4+Bb4 G4+Bb4 F4+A4 F4+A4 G4+Bb4 G4+Bb4 | F4+A4 F4+A4 G4+Bb4 G4+Bb4 \
             F4+A4 F4+A4 G4+Bb4 G4+Bb4 | F4+A4:1 G4+Bb4 F4+A4 G4+Bb4 F4+A4 G4 F4 Bb4 E4+G4+Bb4 \
             A4 C4+G4 C5 C4+G4 - C4:2 | F4 G4:1 -:5 F4+A4:2 F4+A4 G4+Bb4 G4+Bb4 | F4+A4 F4+A4 \
             G4+Bb4:1 - G4+Bb4 - E4+G4+Bb4 - D4+G4 - D4+G4 - D4:2 | - G4:1 -:5 F4+A4:2 F4+A4 \
             G4+Bb4 G4+Bb4 | F4+A4 F4+A4 G4+Bb4 G4+Bb4 F4+A4 F4+A4 G4+Bb4:1 - E4+G4+Bb4 - | \
             D4+G4+Bb4 - D4+G4 - D4+G4+Bb4 A4 D4+G4 - E4+G4+Bb4 A4 G4 - C3+C4+G4+Bb4 - C3+C4+G4 \
             - & -:16 | - | -:15 C5:1 | G3 A3 Bb3 C4 D4 E4 F4 G4 -:8 | -:5 A4+C5:1 - A4+C5 - A4 \
             - C5 -:3 C5:1 | G3 A3 Bb3 C4 D4 E4 F4 G4 -:8 | -:13 A4+C5:1 - A4+C5 | - A4 - C5 -:3 \
             C5:1 -:3 C5:1 - A4 - C5 & -:16 | -:14 (3 -:1 - A4. | C5:6 -:10 | (3 F3:1 - - -:4 (3 \
             -:1 G4 E4+Bb4 -:8 | (3 C4:1 D4 - -:4 A4:1 -:9 | -:4 (3 -:1 E4+C5+E5 Bb4+F5 (3 \
             G4+A4+G5 A4 - -:8 | - (3 -:1 D4+D5 E4+E5 (3 - G4+G5 - Bb5:2. A5 | G5:4 G5:2 A5 G5 - \
             (3 C5:1 D4+D5 - D4+D5:2 & -:16 | -:12 (3 -:1 D5 E4+E5 (3 F4+F5 - A5 | -:6 A5:2 \
             Bb5:4 A5:2 G5 | F5 - (3 -:1 E4+C5+E5 Bb4+F5 (3 G4+A4+G5 A4 - -:8 | (3 -:1 - E4+E5 \
             (3 - G4+G5 - Bb5:2. A5 D5+D6:8 | E5+C6:2 -:4 (3 -:1 G4 E4+Bb4 -:8 | - (3 C4:1 - - \
             -:4 A4:1 - | -:6 A4:1 -:3 F4+F5:2 E4+E5 (3 F4+F5:1 G5 A5 & -:16 | - | - | -:14 (3 \
             -:1 - D5. | (3 - - D4+D6 (3 - D4+D6 - (3 D4+C5+E5+G5+D6 C4+C6 D4+D6 C4+C6:2. -:1 C5 \
             Bb4 A4 Bb4 A4+C5 G4+Bb4 F4+A4 | G4+Bb4 A4+C5 Bb4 A4+C5 G4+Bb4+F5 F4+C5+E5+G5 \
             E4+D5+F5+A5 D4+E5+G5+Bb5 (3 C4+F5+A5+C6 D4+D5+D6 C4+C5+C6 (3 D4+D5+D6 - D4+D5+D6 (3 \
             C4+C5+E5+G5+C6 D4+D5+D6 C4+C5+C6 (3 D4+D6 C4+C6 D4+D5+D6 | (3 - D4+D6 C4+C6 \
             F5+A5+C6:2 - C5+E5+G5 (3 D4+F5+A5+C6+D6:1 - - F5+A5+C6:2 -:4 | -:16 & -:2 F5+A5+C6 \
             - C5+E5+G5 (3 C4+F5+A5+C6:1 - - (3 D4+D6 C4 D4+D6 (3 C4+C5+E5+G5+C6 D4+D6 C4+C6 (3 \
             D4+D6 C4+C6 D4+D6 | (3 - C4+C6 D4+D6 (3 C4 D4+D6 C4 -:6 F5+A5+C6:2 - C5+E5+G5 | (3 \
             D4+F5+A5+C6+D6:1 - - -:10 Bb4:1 A4+C5 -:2 | -:1 A4+C5 Bb4 A4+C5 G4+Bb4+D5 \
             F4+C5+E5+G5 E4+D5+F5+A5 D4+E5+G5+Bb5 (3 C4+F5+A5+C6 D4+D5+D6 C4+C5+C6 (3 D4+D5+D6 \
             C4+C5 D4+D5+D6 (3 C4+C5+E5+G5+C6 D4+D5+D6 C4+C5+C6 (3 D4+D6 C4+C6 D4+D6 | (3 - \
             C4+C6 - (3 C4 - C4 -:12 | -:10 (3 -:1 C4+C5 - -:4 | -:2 (3 -:1 C4 - -:6 (3 C4:1 - \
             C4 -:4 | -:16 & (3 C4+F5+A5+C6:1 D4+D6 C4+C6 (3 D4+D6 C4 D4+D6 (3 C4+C5+E5+G5+C6 \
             D4+D6 C4+C6 (3 D4+D6 C4+C6 D4+D6 (3 - D4+D6 C4+C6 F5+A5+C6:2 - C5+E5+G5 | (3 \
             D4+F5+A5+C6+D6:1 - - F5+A5+C6:2 (3 D4+C5+E5+G5+D6:1 C4+C6 C4+C6 D4+C5+E5+G5+D6 \
             C4+C6 (3 C4+F5+A5+C6 D4+D6 C4+C6 (3 D4+D6 C4 D4+D6 (3 C4+C5+E5+G5+C6 D4+D6 C4+C6 (3 \
             D4+D6 C4+C6 D4+D6 | (3 - C4+C6 D4+D6. (3 _ _ - - Bb4 A4 G4 - C5 Bb4 A4:2. -:1 \
             G4+Bb4 F4+A4 | G4+Bb4 -:9 F5+A5+C6:2 - C5+E5+G5 | (3 D4+F5+A5+C6+D6:1 - - \
             F5+A5+C6:2 -:12 | -:10 F5+A5+C6:2 - C5+E5+G5 | (3 C4+F5+A5+C6:1 - - (3 D4+D6 - \
             D4+D6 (3 C4+C5+E5+G5+C6 C4+C6 D4+D6 (3 C4+C6 D4+D6 C4+C6 (3 - C4+C6 D4+D6 (3 - \
             D4+D6 - (3 D4+C5+E5+G5+D6 C4+C6 D4+D6 (3 D4+D6 - - | - C5 Bb4 A4 - C5 Bb4 A4 - C5 \
             Bb4 A4 - C5 Bb4 A4",
        )
        .in_time(COMMON)
        .in_key(C, Mixolydian)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 896 to 899.
        Part::new(
            "The wet-nurses' tune, tutti",
            "C7:6 A6:2 Bb6:4 A6:2 G6 | F6:8 C7:4 C7:2 A6 | D7:4 D7 C7 C7:2 A6 | G6:4 G6:2 A6 \
             G6:4 (3 G5:1 A5 Bb5 (3 C6 D6 E6",
            "",
        )
        .with_left(
            "F3+C4+F4+A4+C5:2 F3+C4+F4+A4+C5 F3+E4+G4+Bb4+D5 F3+E4+G4+Bb4+D5 E3+F4+A4+C5 \
             C3+C4+F4+A4+C5 C3+E4+G4+Bb4+D5 C3+E4+G4+Bb4+D5 | F3+C4+F4+A4+C5 F3+C4+F4+A4+C5 \
             F3+E4+G4+Bb4+D5 F3+E4+G4+Bb4+D5 C4+F4+A4+C5 C4+F4+A4+C5 G3+C4+E4+Bb4+D5 \
             E3+C4+G4+Bb4+D5 | D3+F4+A4+C5 D3+F4+A4+C5 D3+E4+G4+Bb4+D5 D3+E4+G4+Bb4+D5 \
             E3+F4+A4+C5 E3+F4+A4+C5 G3+E4+Bb4+D5 E3+C4+G4+Bb4+D5 | D3+F4+A4+C5 D3+F4+A4+C5 \
             D3+E4+G4+Bb4+D5 D3+E4+G4+Bb4+D5 E3+C4+G4+Bb4+D5:1 -:7 & -:5 C5:1 A4+A5:2 Bb4+Bb5:4 \
             A4+A5:2 - | -:8 E3+E4:4 - | D6:8 - | G4:4 -:2 A4 (3 -:1 - C#4+C#5. (3 _ _ C4+C5 (3 \
             _ B3+B4. _ (3 _ Bb3+Bb4+A5 - & -:16 | - | -:12 (3 C5+E5+G5:1 - - C5+E5+G5+A5:2 | (3 \
             E5+F5+A5+C6:1 - - E5+A5+C6:2 (3 C5+E5:1 - - -:2 (3 D4+C5+E5+G5:1 E4+C5+E5 F4+D5+F5 \
             (3 _ G4+E5+G5 - (3 G4+Bb4+Bb5 - - -:2 & -:4 C6:2 -:7 C5:1 -:2 | -:5 C5:1 -:7 C5:1 \
             -:2 | D5:4 -:1 C5 -:8 A4:2 | D5 C#5:1 - (3 - B4. _ (3 _ - - -:4 (3 D5:1 E5 F5 (3 G5 \
             - Bb5 & E5+F5+A5+C6:1 - E5+A5+C6:2 C5+E5+G5:1 - C5+E5+G5:2 E5+F5+A5+C6:1 - \
             E5+A5+C6:2 C5+E5+G5:1 - C5+E5+G5:2 | E5+F5+A5+C6:1 - E5+A5+C6:2 C5+E5+G5:1 - \
             C5+E5+G5:2 E5+F5+A5+C6:1 - E5+A5+C6:2 C5+E5+G5:1 - C5+E5+G5:2 | E5+F5+A5+C6:1 - \
             E5+A5+C6:2 C5+E5+G5:1 - C5+E5+G5:2 (3 D4+E5+F5+A5+C6:1 - C#4. (3 _ _ C4 (3 _ B3. _ \
             (3 _ Bb3 _ | (3 D4 _ C#4. (3 _ _ C4 (3 _ B3. _ (3 _ Bb3 _ -:8 & -:16 | - | -:10 \
             E5+A5+C6:2 C6 - | G5:4 G5:2 C5+E5+G5+A5 - (3 -:1 - A4+F5+A5 (3 - A4+C5+C6 \
             Bb4+D5+Bb5+D6 (3 _ C5+E5+C6+E6 D5+F5+D6+F6",
        )
        .in_time(TWO_TWO)
        .in_key(C, Mixolydian)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 900.
        Part::new(
            "A peasant with a bear",
            "(3 G6:1 G6 G6 (3 _ G6 G6 (3 G5 G5 G5 (3 _ G5 G5 G4 -:15",
            "",
        )
        .with_left(
            "F#1+F#2:4 F#1+F#2 E1+E2 E1+E2 F#1+F#2 F#1+F#2 & (3 D4:1 Eb4 - (3 - F4 F#4 Bb2+F#3:4 \
             F#1+F#3 C3+E3 E1+E3 Bb2:2 - & -:4 (3 D3:1 Eb3 E3 (3 _ F3 - G3 G#3 A3 Bb3 -:8 F#3:4 \
             & (3 E5:1 _ - -:6 B3:4 B3 -:8 & (3 D6:1 C#4+C#5+C#6 C4+C5+C6 (3 _ B3+B4+B5 \
             Bb3+Bb4+Bb5 (3 D4+D5 C#3+C#4+C#5 C3+C4+C5 (3 _ B2+B3+B4 Bb3+Bb4 (3 D3+D4 C#3+C#4 \
             C3+C4 (3 - B2 Bb2 -:8 Eb2:4 & -:6 (3 A4:1 - E4+G4+E5 (3 E4 E4+G4 E4+G4 (3 _ E4+G4 \
             E4+G4 -:12 & (3 E4+D5+G5+E6:1 E4+Eb5+G5+E6 E4+G4+E5+G5+E6 (3 _ E4+F5+G5+E6 \
             E4+F#5+G5+E6 (3 E4+G4+E5 E4+G4+G#4+E5 E4+G4+E5 (3 _ E4+G4+Bb4+E5 - -:16",
        )
        .in_time(THREE_TWO)
        .with_tempo(&[tempo(0, 59)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 901 to 906.
        Part::new(
            "Clarinet and tuba",
            "-:2 Bb5:1 B5 D6:2 Eb6:1 D6 C6:2 Bb5 E6:4 -:2 E6 E6 E6 | (3 D6:1 E6 Eb6 (3 _ D6 C6 \
             Bb5:2 Bb5 Bb5 C6:1 D6 E6 Eb6 D6 C6 Bb5:2 Bb5 E6:4 | _:1 B5 C6 D6 E6 Eb6 D6 C6 Bb5:2 \
             Bb5 Bb5 -:10 | -:2 Bb5:1 B5 D6:2 Eb6:1 D6 C6:2 Bb5 E6:4 -:2 E6 E6 E6 | (3 D6:1 E6 \
             Eb6 (3 _ D6 C6 Bb5:2 Bb5 Bb5 C6:1 D6 E6 Eb6 D6 C6 Bb5:2 Bb5 E6:4 | _:1 B5 C6 D6 E6 \
             Eb6 D6 C6 Bb5:2 Bb5 Bb5 -:10",
            "",
        )
        .with_left(
            "-:24 | - | - | - | - | -:16 F#1+F#2:4 F#1+F#2 & E1+E2:4 E1+E2 F#1+F#2 F#1+F#2 E1+E2 \
             E1+E2 | F#1+F#2 F#1+F#2 E1+E2 E1+E2 F#1+F#2 F#1+F#2 | E1+E2 E1+E2 F#1+F#2 F#1+F#2 \
             E1+E2 E1+E2 | F#1+F#2 F#1+F#2 E1+E2 E1+E2 F#1+F#2 F#1+F#2 | E1+E2 E1+E2 F#1+F#2 \
             F#1+F#2 E1+E2 E1+E2 | F#1+F#2 F#1+F#2 E1+E2 E1+E2 (3 D4:1 _ D4 (3 _ D4 _ (3 D4 _ D4 \
             (3 _ C4+D4 - & -:4 C3 - Eb2+Bb2 - C3 | - Eb2+Bb2 - C3 - Eb2+Bb2 | - C3 - Eb2+Bb2 - \
             C3 | - Eb2+Bb2 - C3 - Eb2+Bb2 | - C3 - Eb2+Bb2 - C3 | - Eb2+Bb2 - C3 - Eb2+Bb2 & \
             F#1+F#3:4 E3 E1+E3 F#3 F#1+F#3 E3 | E1+E3 F#3 F#1+F#3 E3 E1+E3 F#3 | F#1+F#3 E3 \
             E1+E3 F#3 F#1+F#3 -:2 G#3 | _:12 E3:4 E1+E3 - | F#1+F#3 E3 E1+E3 F#3 F#1+F#3+G#3 \
             -:2 G#3 | _:12 E3:4 E1+E3 F#3 & -:24 | - | -:10 G#3 E3:4 | E1+E3 F#3 F#1+F#3 -:2 \
             A3:4 D4:1 - F#3:4 | -:2 D4 D4 D4 C4:1 D4 C4 Bb3 G#3:2 G#3 -:4 E3 | E1+E3 F#3 \
             F#1+F#3 -:2 G#3:10 & -:24 | - | - | - | - | -:16 (3 -:1 C4 - (3 C4 - C4 (3 - C4 _ \
             -:2",
        )
        .in_time(THREE_TWO)
        .with_tempo(&[tempo(0, 69)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 907 to 911.
        Part::new(
            "Woodwind trills",
            "-:8 (3 F4:1 E4 F4 (3 E4 F4 E4 F4 E4 F4 E4 F4 E4 F4 E4 F4 E4 F4 E4 | F4 E4 F4 E4 F4 \
             E4 F4 E4 F4 -:7 A4:1 G4 A4 G4 A4 G4 A4 G4 | A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 \
             G4 A4 G4 A4 -:7 | A4:1 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 \
             G4 A4 G4 | A4 -:7 A4:1 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4 A4 G4",
            "",
        )
        .with_left(
            "-:4 C3 - Eb2+Bb2 - C3 | - Eb2+Bb2 - C3 - Eb2+Bb2 | E1+E2 E2+C3+E3 F#1+F#2 \
             Eb2+F#2+Bb2+F#3 E1+E2 E2+C3+E3 | -:24 | - & -:11 Bb3:1 G#3:2 G#3 G#3 - E3:4 | E1+E3 \
             F#3 F#1+F#3 - E1+E3 - | F#1+F#3 -:20 | -:24 | (3 -:2 D4+F4 - (3 - D4+A4 A4 (3 A4 A4 \
             - (3 A4 A4 A4 (3 A4 F4+A4 F4+A4 (3 F4+A4 F4+A4 F4+A4 & F#1+F#3:4 E3 E1+E3 F#3 \
             F#1+F#3 -:3 Bb3:1 | G#3:2 G#3 G#3:8 E3:4 - F#3 | -:24 | -:8 (3 -:2 D4 D4 (3 - D4 D4 \
             (3 - D4+F4 D4+F4 (3 - D4+F4 - | (3 A4 A4 A4 (3 A4 F4 D4 (3 D4 F4 D4+A4 (3 - D4+F4 - \
             (3 - D4 - (3 - D4 - & E1+E2:4 E1+E2 F#1+F#2 F#1+F#2 E1+E2 E1+E2 | F#1+F#2 F#1+F#2 \
             E1+E2 E1+E2 F#1+F#2 F#1+F#2 | -:16 (3 -:1 C4+E4+G4 _ (3 - C4+E4+F4+G4+A4 _ (3 \
             D4+F4+A4 C4+E4+G4 _ -:2 | - (3 -:1 C4+D4+E4+F4+A4 _ (3 - E4 _ -:6 (3 \
             C4+D4+F4+A4+B4:1 E4+C5 _ (3 C4+D4+B4 - - -:2 (3 C4+D4+F4+C5+D5:1 C4+E4+A4+B4+E5 - \
             D4+F4+B4+D5 C4+E4+A4+C5+E5 D4+F4+B4+D5 C4+E4+A4+C5+E5 | D4+F4+B4+D5 C4+E4+C5+E5 \
             D4+F4+B4+D5 C4+E4+B4+E5 D4+F4+C5+D5 - C4+F4+B4+D5 D4+E4+C5+E5 C4+F4+B4+D5+F5 -:3 \
             C4+D4+F4+B4+D5+F5:1 - C4+D4+F4+B4+D5+F5 - C4+D4+F4+B4+D5+F5 - C4+D4+B4+D5+F5 -:5 & \
             (3 -:1 C4 _ -:2 (3 -:1 C4 _ -:2 (3 -:1 C4 _ -:2 (3 -:1 C4 _ -:2 (3 -:1 C4 _ -:2 (3 \
             -:1 C4 _ -:2 | (3 -:1 C4 _ -:2 (3 -:1 C4 _ -:2 (3 -:1 C4+E4 _ -:2 (3 -:1 C4+E4 _ \
             -:2 (3 -:1 C4+E4 _ -:2 (3 -:1 C4+E4 _ -:2 | (3 -:1 C4+E4 _ -:2 (3 -:1 C4+E4 - (3 \
             C4+F4 - C4 (3 - C4+E4 _ -:2 (3 -:1 C4+E4 _ -:2 D4 (3 D4:1 - - -:4 | (3 -:1 C4+E4 _ \
             -:4 (3 C4:1 - - -:16 | -:24 & (3 D4:1 _ D4 (3 _ C4+D4 _ (3 D4 _ D4 (3 _ C4+D4 _ (3 \
             D4+F4 _ D4+F4 (3 _ C4+D4+F4 _ (3 D4+F4 _ D4+F4 (3 _ C4+D4+F4 _ (3 D4+F4 _ D4+F4 (3 \
             _ C4+D4+F4 _ (3 D4+F4 _ D4+F4 (3 _ C4+D4+F4 _ | (3 D4+F4 _ D4+F4 (3 _ C4+D4+F4 _ (3 \
             D4+F4 _ D4+F4 (3 _ C4+D4+F4 _ (3 D4+F4 _ D4+F4 (3 _ C4+D4+E4+F4 _ (3 D4+F4 _ D4+F4 \
             (3 _ C4+D4+E4+F4 _ (3 D4+F4+A4 _ D4+F4+A4 (3 _ C4+D4+E4+F4+A4 _ (3 D4+F4+A4 _ \
             D4+F4+A4 (3 _ C4+D4+E4+F4+A4 _ | (3 D4+F4+A4 - D4+F4+A4 (3 _ C4+D4+E4+F4+A4 _ (3 \
             D4+F4+A4 - D4+F4+A4 (3 - D4+E4+F4+A4 - (3 D4+F4+A4 - D4+F4+A4 (3 _ C4+D4+E4+F4+A4 _ \
             (3 D4+F4+A4 - D4+F4+A4 (3 _ C4+D4+E4+F4+A4 _ (3 F4+A4 _ F4+A4 (3 _ - - (3 - - \
             D4+F4+A4 (3 _ C4+E4+F4+G4+A4 _ | (3 D4+F4+A4 - D4+F4+A4 (3 _ - - (3 D4+F4+A4 - \
             D4+F4+A4 (3 _ D4+E4+F4+A4 _ (3 C4+F4+A4+B4 D4+E4+C5 F4+A4+B4 (3 F4+C5 E4+F4+A4+B4 \
             C5 (3 - - F4+A4 (3 _ E4+F4+A4+C5 _ (3 C4+D4+F4+A4+B4+D5 C4+E4+C5+E5 A4+B4 (3 - - C5 \
             -:4 | -:5 E4+C5+E5:1 -:3 D4+E4+C5+E5+G5:1 F4+B4+D5+F5 E4+C5+E5+G5 - E4+C5+E5+G5 - \
             E4+C5+E5+G5 - C4+C5+E5+G5 - C4+C5+E5+G5 D4+B4+D5+F5 C4+C5+E5+G5 D4+B4+D5+F5 \
             C4+C5+E5+G5",
        )
        .in_time(THREE_TWO)
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 912 to 917.
        Part::new(
            "A merchant",
            "F5:1 G5 F5 E5 F5 E5 D5:2 D5:1 E5 F5 G5 | F5 G5 F5 G5 F5 G5 F5 G5 F5 G5 F5:2 | F5:1 \
             G5 F5 G5 F5 G5 A5 G5 A5 G5 A5 G5 | F5 G5 F5 G5 F5 G5 F5 G5 F5 G5 F5 G5 | F5 -:3 \
             B4:1 C5 D5 C5 B4 C5 D5 E5 | F5 G5 F5 G5 F5 G5 F5 G5 F5 -:3",
            "",
        )
        .with_left(
            "D4+F4+A4+D5:1 C4+E5 D4+F4+A4+B4+D5 E4+F4+A4+B4+C5 D4+G4+A4+B4 E4+F4+A4+B4 \
             F4+G4+A4+C5 E4+G4+A4+B4+E5 D4+F4+A4+B4+F5 C4+C5+G5 D4+D5 C4+E5 | D4+F4 C4+G4 D4+F4 \
             C4+E4 D4+F4 C4+E4+G4 D4+A4+D5 C4+C5 D4+D5 C4+E4+E5 D4+F4 C4+G4+G5 | D4+F4+A4 \
             C4+E4+E5 D4+F4 C4+E4+G4+E5 D4+F4 C4+E4+G4+E5 B3+D4+A4+F5 C4+E5 B3+D4+F5 C4+E5 \
             B3+D4+F5 C4+E5 | D4+F4 C4+G4 D4+F4 C4+E4 D4+F4 C4+E4+G4 D4+A4+D5 C4+C5 D4+D5 \
             C4+E4+E5 D4+F4 C4+G4 | D4+A4+B4 E4+A4+E5 D4+F4+B4+F5 C4+E4+C5+G5 D4+D5+F5 C4+E5+G5 \
             B3+D4+F5+A5 C4+E5+G5 D4+D5+F5 E4+E5 D4+F4+B4+F5 E4+G4+A4 | D4+A4+B4+D5 C4 D4 C4+E4 \
             D4+F4 C4+G4 D4+A4+B4 C4+C5 D4+D5 C4+E4+E5+G5 D4+F4+F5 C4+G4+E5+G5 & B4:1 E4+G4+C5 - \
             G4 F4+D5 G4+C5 B4 C5 - E4+G4 F4+A4+B4 E4+G4+C5 | A4+B4+D5 E4+C5+E5 A4+B4+D5 \
             G4+C5+E5 A4+B4+D5 C5+E5 F4+B4 E4+G4+E5 F4+A4+B4 G4+C5 A4+B4+D5 E4+C5+E5 | B4+D5 \
             G4+C5 A4+B4+D5 C5 A4+B4+D5 C5 F4+B4+D5 E4+G4+C5 F4+A4+B4+D5 E4+G4+C5 F4+A4+B4+D5 \
             E4+G4+C5 | A4+B4+D5 E4+C5+E5 A4+B4+D5 G4+C5+E5 A4+B4+D5 C5+E5 F4+B4 E4+G4+E5 \
             F4+A4+B4 G4+C5 A4+B4+D5 E4+C5+E5 | F4+D5 G4+C5 A4+D5+E5 G4+E5 F4+A4 E4+G4 F4+A4+B4 \
             E4+G4 F4+A4 G4 A4 C5 | F4 E4+G4+C5+E5 F4+A4+B4+D5 G4+C5+E5 A4+B4+D5 E4+C5+E5 F4+D5 \
             E4+G4+E5 F4+A4+B4 G4+C5 A4+B4+D5 E4+C5",
        )
        .in_time(SIX_EIGHT)
        .with_tempo(&[tempo(0, 104)])
        .in_key(D, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 918 to 933.
        Part::new(
            "The merchant's tune",
            "F5:1 G5 F5 E5 F5 E5 D5 E5 F5 G5 F5 G5 | F5:2 F5:1 G5 A5 G5 A5 B5 A5 B5 C6 B5 | A5 \
             G5 A5 G5 A5 B5 A5 G5 F5 G5 F5 E5 | F5 -:3 F5:1 G5 F5 G5 F5 E5 F5 G5 | F5 G5 F5:2 \
             F5:1 E5 D5 E5 F5 G5 F5 G5 | F5 G5 F5:2 F5:1 E5 D5 E5 F5 G5 F5 G5 | F5 G5 F5:2 F5:1 \
             E5 D5 E5 F5 G5 F5 G5 | F5:2 F5:1 G5 A5 G5 A5 B5 A5 B5 C6 B5 | A5 G5 A5 G5 A5 B5 A5 \
             G5 F5 G5 F5 E5 | F5 -:3 F5:1 G5 F5 G5 F5 E5 F5 G5 | A5 G5 A5 B5 A5 B5 C6 B5 A5 G5 \
             A5 G5 | A5 B5 A5 G5 F5 G5 F5 E5 F5:2 D5:1 E5 | F5 G5 F5 G5 F5:2 F5:1 G5 A5 G5 A5 B5 \
             | A5 B5 C6 B5 A5 G5 A5 G5 A5 B5 A5:2 | A5:1 B5 C6 B5 A5 G5 A5 G5 A5 B5 A5:2 | A5:1 \
             B5 C6 B5 A5 G5 A5 G5 A5 B5 A5 -",
            "",
        )
        .with_left(
            "-:12 | - | - | - | - | - | - | - | - | G3:2 (3 _:1 _ A3 (3 B3 - - -:6 | -:3 B3:1 \
             -:6 G3:2 | -:6 G3:4 (3 A3:1 B3 C4 | -:3 B3:1 -:2 A3:1 -:5 | G3:2 (3 _:1 _ - (3 - C4 \
             - -:6 | -:3 B3:2 -:1 A3 A3:2 -:3 | G3:2 (3 _:1 _ - -:8 & -:12 | - | - | - | - | - | \
             - | -:2 (3 -:1 - A3+A4 (3 - C4+C5 D4+D5 -:6 | -:3 B3+B4:1 -:2 A3:1 -:2 D4+D5:1 -:2 \
             | (3 G4:1 _ - -:2 (3 -:1 C4+C5 D4+D5 -:6 | -:5 B3+B4:1 C4+C5 -:2 A3+A4:1 -:2 | G4:1 \
             -:11 | -:12 | (3 G4:1 _ - (3 - - A3+A4 (3 B3 C5 D4+D5 -:6 | -:3 B4:1 - B3+B4 -:4 \
             G4+G5:1 - | (3 G4 _ - (3 - - A3+A4 (3 B3 C4 D4+D5 -:6 & -:12 | - | - | - | - | - | \
             - | -:6 B3+D4+F4+C5+F5:1 A3+C4+E4+B4 B3+D4+A4+F5 A3+C4+E4 G3+B3+D4+F4+A5 A3+C4+E4 | \
             B3+D4+F5 C4+E4+E5 B3+D4+F4+F5 C4+E4+G4+E5 B3+D4+F4+A4+F5 A3+C4+G4+B4+G5 B3+D4+C5+F5 \
             C4+E4+B4+E5 D4+F4+A4+D5 C4+E4+G4 D4+F4+A4+D5 E4+B4+C5 | D4+F4+A4+B4 E4+A4+B4 \
             F4+G4+C5 G4+A4+B4 D4+A4+B4 C4+E4+G4+B4+C5 D4+F4+C5+D5 C4+B4+C5 D4+F4+A4+B4 - \
             D4+F4+A4+B4 C4+E4 | -:12 | -:5 C4+E4+G4:1 -:6 | -:12 | -:6 D4:2 -:4 | -:1 C4+E4 \
             -:10 | -:12 & D4+A4+D5:1 C4+E4+C5+E5 D4+F4+B4+D5 G4+A4+B4+C5 D4+A4+B4+D5 A4+B4+C5 \
             G4+B4+C5 A4+B4 D4+A4+D5 C4+G4+C5 D4+F4+A4+B4 C4+E4+G4+C5 | D4+F4+B4 E4+A4 D4+F4+A4 \
             C4+G4 B3+D4+F4+A4+F5 C4+E4+B4+E5 B3+D4+F4+C5+F5 A3+C4+E4+B4+G5 B3+D4+A4+F5 \
             A3+C4+E4+G4+G5 G3+B3+D4+F4+A5 A3+C4+E4+G5 | B3+D4+F5 C4+E4+E5 B3+D4+F4+F5 \
             C4+E4+G4+E5 B3+D4+F4+A4+F5 A3+C4+G4+B4+G5 B3+D4+C5+F5 C4+E4+B4 D4+F4+A4 C4+E4+G4 \
             D4+F4+A4+D5 E4+B4+C5 | D4+F4+A4+B4 E4+A4+B4 F4+G4+C5 G4+A4+B4 D4+A4+B4 \
             C4+E4+G4+B4+C5 D4+F4+A4+C5+D5 C4+G4+B4+C5 D4+F4+A4+B4 G4+A4+B4 D4+F4+A4+B4 \
             C4+E4+G4+C5 | D4+F4+A4+D5 C4+E4+C5 D4+F4+A4+B4 E4+G4+A4+B4 D4+F4+A4+D5 \
             E4+G4+A4+B4+C5 F4+G4+A4+B4+C5 A4+B4 D4+A4+B4 C4+G4 D4+F4+A4+B4 C4+E4+G4+C5 | \
             D4+F4+D5 C4+E4+C5 D4+F4+A4+B4 E4+G4+A4+B4 D4+F4+A4+D5 E4+G4+A4+B4+C5 F4+G4+A4+B4+C5 \
             A4+B4 D4+A4+B4 C4+G4 D4+F4+A4+B4 C4+E4+G4+C5 | D4+F4+D5 C4+E4+C5 D4+F4+A4+B4 \
             E4+G4+A4+B4 D4+F4+A4+D5 E4+G4+A4+B4+C5 F4+G4+A4+B4+C5 A4+B4 D4+A4+B4 C4+G4 \
             D4+F4+A4+B4 C4+E4+G4+C5 | D4+F4+B4 E4+A4 D4+F4+A4+B4 C4+G4+E5 B3+D4+F4+A4+F5 \
             C4+E4+B4+E5 G4+G5:6 | _:3 -:9 | -:6 G4+G5 | _:3 -:9 | -:12 | -:6 A4+C5+F5:1 -:5 | \
             -:6 G4+G5 | _:3 -:8 E4+B4+G5:1 | -:2 B3+D4+E5+A5:1 C4+E4+D5+G5 D4+F4+C5+F5 - \
             D4+G4+A4+E5+F5+G5 E4+B4+D5+E5 -:3 E4+A4+G5:1 & -:12 | - | - | - | - | - | - | - | - \
             | -:6 A4:2 -:1 A4+B4 -:2 | B3+D4+F4+A4+F5:1 C4+E4+B4+E5 B3+D4+F4+C5+F5 \
             A3+C4+E4+B4+G5 B3+D4+A4+F5 A3+C4+E4+E5+G5 G3+B3+D4+F5+A5 A3+C4+E4+G4+G5 B3+D4+F5 \
             C4+E4+E5 B3+D4+F4+F5 C4+E4+G4+E5 | B3+D4+F4+A4+F5 A3+C4+G4+B4+G5 B3+D4+C5+F5 \
             C4+E4+B4+E5 D4+F4+A4+D5 - D4+F4+A4+D5 E4+G4+B4+C5 D4+F4+A4+B4 E4+A4+B4 F4+G4+A4+C5 \
             E4+G4+A4+B4 | D4+A4+B4+C5 E4+B4+C5 F4+A4+C5+D5 G4+B4+E5 F4+A4+C5+D5 E4+G4+B4+D5+E5 \
             - G4+B4 D4+F4+A4+F5 E4+G4+B4+E5 D4+F4+A4+F5 C4+G4+G5 | D4+F4+A4+F5 C4+E4+G5 \
             B3+D4+F5+A5 C4+E4+E5+G5 D4+F4+D5+F5 E4+G4+C5 A4+F5 E4+B4+E5 D4+F4+A4+D5+F5 C4+E5 \
             D4+A4+F5 E4+B4 | D4+F4+A4+F5 - B3+D4+F5+A5 C4+E4+E5+G5 D4+F4+D5+F5 E4+G4+C5 \
             D4+A4+F5 E4+B4+E5 D4+F4+A4+D5+F5 C4+G4+E5+G5 D4+A4+F5 - | D4+F4+A4+F5 C4+E4+G5 -:3 \
             E4+G4+B4:1 -:2 D4+F4+A4+C5+F5:1 C4+G4+D5+G5 D4+G4+A4+F5 - & F4+B4:1 G4 A4 E4 F4 \
             E4+G4 F4+A4 E4+G4+C5 F4+B4 E4+E5 D5 E5 | A4 G4+C5+E5 B4+D5 E4+C5+E5 B4+D5 G4+A4+C5 \
             G4+A4+B4+D5 G4+A4+C5+E5 F4+B4+D5 C5+E5 A4+B4+D5 G4+C5+E5 | F4+A4+B4+D5 G4+C5 \
             A4+B4+D5 C5 B4+D5 E4+A4+C5+E5 F4+G4+A4+B4+D5 G4+A4+C5+E5 B4+D5 C5+E5 B4 G4 | - \
             G4+C5+E5 A4+B4+D5 E4+C5+E5 F4+D5 A4+E5 G4+B4 E4+A4+E5 D5 E4+C5 D5 E5 | B4 G4+E5 D5 \
             C5+E5 B4 -:2 E4+G4+C5:1 F4+D5 E4+C5+E5 D5 E5 | A4+B4 G4+E5 D5 C5+E5 B4 -:2 \
             E4+G4+C5:1 F4+D5 E4+C5+E5 D5 E5 | A4+B4 G4+E5 D5 C5+E5 B4 -:2 E4+G4+C5:1 F4+D5 \
             E4+C5+E5 D5 E5 | A4 G4+C5+E5 D5 E4+C5 B4+D5 G4+A4+C5 A4+B4+D5 A4+C5+E5 F4+B4+D5 \
             C5+E5 A4+B4+D5 C5+E5 | F4+A4+B4+D5 C5 A4+B4+D5 C5 B4+D5 E4+A4+C5+E5 F4+G4+A4+B4+D5 \
             G4+A4+C5 B4 C5+E5 B4 G4 | - G4+C5+E5 A4+B4+D5 E4+C5+E5 F4+D5 A4+E5 B4 E4+E5 D5 \
             E4+C5 D5 C5+E5 | B4+D5 C5 A4+B4+D5 G4+C5+E5 F4+B4+D5 G4+A4+C5 F4+G4+A4+B4+D5 \
             A4+C5+E5 F4+A4+B4+D5 G4+C5 A4+B4+D5 C5 | B4 E4+C5+E5 F4+A4+B4+D5 G4+C5 B4 A4+C5+E5 \
             G4+B4 A4 D5 G4+C5+E5 B4 C5 | F4+D5 G4+E5 B4 E4+C5 B4 A4+C5 F4+G4+B4+D5 E4+A4+C5+E5 \
             B4+D5 C5 B4+D5 E4+C5+E5 | B4 G4+C5+E5 F4+A4+B4+D5 G4+C5 A4+B4 A4+E5 F4+B4+D5 A4+C5 \
             B4 E4+C5 F4+B4+D5 C5+E5 | B4+D5 C5+E5 F4+A4+B4+D5 G4+C5 A4+B4 A4+E5 F4+G4+B4+D5 \
             G4+A4+C5 B4 E4+C5 F4+B4+D5 G4+C5+E5 | B4 G4+C5+E5 F4+A4+B4+D5 G4+C5+E5 A4+B4+D5 \
             A4+C5+E5 F4+B4+D5 G4+A4+C5 B4+D5 E4+C5+E5 F4+B4+D5 G4+C5+E5",
        )
        .in_time(SIX_EIGHT)
        .in_key(D, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Snare, "-:12 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | -:10 \
                           (3 -:1 x x"),
        ])
        .marked(Dynamic::Piano),
        // MIDI bars 934 to 937.
        Part::new(
            "Two gypsy girls",
            "F5:2. -:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5 | F5 F5:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5:1 \
             -",
            "",
        )
        .with_left(
            "D2:1 -:7 | -:8 | - | - & F2+A2+F3+A4:1 -:7 | -:8 | - | - & -:1 D4 - D4 - D4 D4 D4 | \
             - D4 D4 D4 - D4 - D4 | - D4 - D4 - D4 D4 D4 | - D4 D4 D4 - D4 - D4 & D4:1 C4+D5+A5 \
             D4+D5+A5 E4+A5 D4+F4+D5+A5 G4+A5 A4+D5+A5 B4+D5 | D4+C5+D5+A5 B4+D5+A5 A4+D5+A5 \
             G4+D5+A5 D4+F4+D5+A5 E4+D5+A5 D4+D5+A5 C4+D5+A5 | D4+D5+A5 C4+D5+A5 D4+D5+A5 E4+A5 \
             D4+F4+D5+A5 G4+A5 A4+D5+A5 B4+D5 | D4+C5+D5+A5 B4+D5+A5 A4+D5+A5 G4+D5+A5 \
             D4+F4+D5+A5 E4+D5+A5 D4+D5+A5 C4+D5+A5 & G4+D5+A5:1 -:7 | -:8 | - | -",
        )
        .with_tempo(&[tempo(0, 138)])
        .in_key(D, Dorian)
        .played_by(&[(gm::OBOE, 0)])
        .with_drums(&[
            (Drum::Snare, "x:8 | - | - | -"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 938 to 939.
        Part::new(
            "Clarinets and horns",
            "A4:4 A4 | A4 A4",
            "",
        )
        .with_left(
            "F4:4 F4 | F4 F4 & -:2 G4 - G4 | _ G4 - G4 & A4:8 | _",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::CLARINET, 0)])
        .with_drums(&[
            (Drum::Snare, "-:8 | -:6 (3 -:1 x x"),
        ]),
        // MIDI bars 940 to 943.
        Part::new(
            "Gypsy girls again",
            "F5:2. -:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5 | F5 F5:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5:1 \
             -",
            "",
        )
        .with_left(
            "D2:1 -:7 | -:8 | - | - & F2+A2+F3:1 -:7 | -:8 | - | - & -:1 D4 - D4 - D4 D4 D4 | - \
             D4 D4 D4 - D4 - D4 | - D4 - D4 - D4 D4 D4 | - D4 D4 D4 - D4 - D4 & D4:1 C4+D5+A5 \
             D4+D5+A5 E4+A5 D4+F4+D5+A5 G4+A5 A4+D5+A5 B4+D5 | D4+C5+D5+A5 B4+D5+A5 A4+D5+A5 \
             G4+D5+A5 D4+F4+D5+A5 E4+D5+A5 D4+D5+A5 C4+D5+A5 | D4+D5+A5 C4+D5+A5 D4+D5+A5 E4+A5 \
             D4+F4+D5+A5 G4+A5 A4+D5+A5 B4+D5 | D4+C5+D5+A5 B4+D5+A5 A4+D5+A5 G4+D5+A5 \
             D4+F4+D5+A5 E4+D5+A5 D4+D5+A5 C4+D5+A5 & D5+A5:1 -:7 | -:8 | - | -",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::OBOE, 0)])
        .with_drums(&[
            (Drum::Snare, "x:8 | - | - | -"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 944 to 948.
        Part::new(
            "Celesta and violins",
            "-:2 (3 B4:1 C5 D5 (3 E5 - - (3 D5 C5 B4 | A4 -:7 | -:8 | -:2 (3 B4:1 C5 D5 (3 E5 - \
             - (3 D5 C5 B4 | A4 -:7",
            "",
        )
        .with_left(
            "F4+A4:8 | _:2 - F4+A4:4 | F4+A4 F4+A4 | F4+A4:8 | _:2 - F4+A4:4 & -:8 | -:2 G4 - G4 \
             | _ G4 - G4 | G4 -:6 | G4:2 G4 - G4",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::VIOLIN, 0)])
        .with_drums(&[
            (Drum::Snare, "-:2 (3 x:1 x x X:2 (3 x:1 x x | X:8 | - | -:2 (3 x:1 x x X:2 (3 x:1 x \
                           x | X:8"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 949 to 957.
        Part::new(
            "Oboes in thirds",
            "-:2 (3 E5:1 F5 G5 (3 A5 - - (3 G5 F5 E5 | D5 -:7 | -:8 | -:2 (3 E5:1 F5 G5 (3 A5 - \
             - (3 G5 F5 E5 | D5 -:7 | -:2 (3 E5:1 F5 G5 (3 A5 - - (3 G5 F5 E5 | D5 -:5 (3 E5:1 \
             F5 G5 | (3 A5 - - (3 G5 F5 E5 (3 D5 - - (3 E5 F5 G5 | (3 A5 - - (3 G5 F5 E5 (3 D5 - \
             - (3 E5 F5 G5",
            "",
        )
        .with_left(
            "F4+A4:8 | _:2 - F4+A4:4 | F4+A4 F4+A4 | F4:6 -:2 | F4:4 F4+A4 | F4 F4 | _:8 | _:6 \
             (3 C4+E4+G4+C5:1 B3+D4+F4+B4+D5 A3+C4+E4+C5+E5 | (3 G3+B3+D4+D5+F5 - - (3 \
             A3+C4+E4+C5+E5 B3+D4+B4+D5 C4+E4+G4+A4+C5 (3 D4+G4+B4 - - (3 - B3+D4+B4+D5 - & -:8 \
             | - | - | -:2 (3 C5:1 B4+D5 C5+E5 (3 D5+F5 - - (3 C5+E5 B4+D5 C5 | G4+B4 -:7 | -:2 \
             (3 C4+E4+G4+C5:1 B3+D4+B4+D5 A3+C4+E4+C5+E5 (3 G3+B3+D4+D5+F5 - - (3 A3+C4+E4+C5+E5 \
             B3+D4+B4+D5 C4+E4+G4+C5 | D4+G4+B4 -:5 (3 C4+E4+G4+C5:1 B3+D4+B4+D5 A3+C4+E4+C5+E5 \
             | (3 G3+B3+D4+D5+F5 - - (3 A3+C4+E4+C5+E5 B3+D4+B4+D5 C4+E4+G4+C5 (3 D4+G4+B4 - - \
             -:2 | F4:4 -:2 (3 C4+E4+G4+C5:1 - A3+C4+E4+C5+E5 & G4:2 -:6 | -:2 G4+B4+D5 G4+B4+D5 \
             G4+B4+D5 | G4+B4+D5 G4+B4+D5 G4+B4+D5 G4+B4+D5 | G4+B4+D5 -:6 | -:2 G4+B4+D5 \
             G4+B4+D5 G4+B4+D5 | G4+B4+D5 -:6 | -:2 G4+B4+D5 G4+B4+D5 - | -:8 | - & -:2 (3 C5:1 \
             B4+D5 C5+E5 (3 D5+F5 - - (3 C5+E5 B4+D5 C5 | G4+B4 -:7 | -:8 | A4 | _:2 -:6 | A4:8 \
             | _:2 - A4:4 | A4:8 | _:2 - F4+A4:4",
        )
        .in_key(D, Dorian)
        .played_by(&[(gm::OBOE, 0)])
        .with_drums(&[
            (Drum::Snare, "-:2 (3 x:1 x x X:2 (3 x:1 x x | X:8 | - | -:2 (3 x:1 x x X:2 (3 x:1 x \
                           x | X:8 | -:2 (3 x:1 x x X:2 (3 x:1 x x | X:6 (3 x:1 x x | X:2 (3 x:1 \
                           x x X:2 (3 x:1 x x | X:2 (3 x:1 x x X:2 (3 x:1 x x"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bars 958 to 969.
        Part::new(
            "The merchant again",
            "A5:1 B5 A5 B5 A5 -:7 | A5:1 B5 A5 B5 C6 B5 A5 -:5 | A5:1 B5 A5 B5 C6 B5 A5 -:5 | \
             F5:1 G5 A5 B5 C6 B5 A5 B5 A5 B5 C6 B5 | A5 G5 A5 G5 A5 B5 A5 G5 F5 G5 F5 E5 | F5 \
             -:3 F5:1 G5 F5 G5 F5 E5 F5 G5 | A5 G5 A5 B5 A5 B5 C6 B5 A5 G5 A5 G5 | A5 B5 A5 G5 \
             F5 G5 F5 E5 F5:2 D5:1 E5 | F5 G5 F5 G5 F5:2 F5:1 G5 A5 G5 A5 B5 | A5 B5 C6 B5 A5 G5 \
             A5 G5 A5 B5 A5:2 | A5:1 B5 C6 B5 A5 G5 A5 G5 A5 B5 A5:2 | A5:1 B5 C6 B5 A5 G5 A5 G5 \
             A5 B5 A5 -",
            "",
        )
        .with_left(
            "-:12 | - | - | - | - | G3:2 (3 _:1 _ A3 (3 B3 - - -:6 | -:3 B3:1 -:6 G3:2 | -:6 \
             G3:4 (3 A3:1 - - | -:6 A3:1 -:5 | G3:2 (3 _:1 _ - (3 - C4 - -:6 | -:3 B3:2 -:1 A3 \
             A3:2 -:3 | G3:2 (3 _:1 _ - -:8 & (3 -:1 A3 - (3 A3 G3 - (3 G3 A3 - (3 G3 A3 G3 (3 \
             G3 A3 G3 (3 A3 A3 G3 | (3 G3 - - (3 A3 - C4 -:2 (3 C4:1 - - G3:4 | _ -:2 (3 A3:1 - \
             - -:2 (3 C4:1 B3 - | (3 G3 _ - (3 - G3+G4 A4 (3 - C4+C5 D4+D5 -:6 | -:3 B3+B4:1 -:2 \
             A3:1 -:2 D4+D5:1 -:2 | (3 G4:1 _ - -:2 (3 -:1 C4+C5 D4+D5 -:6 | -:5 B3+B4:1 C4+C5 \
             -:2 A3+A4:1 -:2 | G3+G4:1 -:9 (3 -:1 B3 C4 | -:3 B3:1 -:8 | (3 G4:1 _ - (3 - - \
             A3+A4 (3 B3 C5 D4+D5 -:6 | -:3 B4:1 - B3+B4 -:4 G4+G5:1 - | (3 G4 _ - (3 - - A3+A4 \
             (3 B3 C4 D4+D5 -:6 & -:12 | - | - | -:6 B3+D4+F4+C5+F5:1 A3+C4+E4+B4 B3+D4+A4+F5 \
             A3+C4+E4+E5 G3+B3+D4+F5+A5 A3+C4+E4 | B3+D4+F5 C4+E4+E5 B3+D4+F4+F5 C4+E4+G4+E5 \
             B3+D4+F4+A4+F5 A3+C4+G4+B4+G5 B3+D4+C5+F5 C4+E4+B4+E5 D4+F4+A4+D5 C4+E4+G4 \
             D4+F4+A4+D5 E4+B4+C5 | D4+F4+A4+B4 E4+A4+B4 F4+G4+C5 G4+A4+B4 D4+A4+B4 \
             C4+E4+G4+B4+C5 D4+F4+C5+D5 C4+B4+C5 D4+F4+A4+B4 - D4+F4+A4+B4 C4+E4 | -:12 | -:5 \
             C4+E4+G4:1 -:6 | -:12 | -:6 D4:2 -:4 | -:1 C4+E4 -:10 | -:12 & -:12 | - | - | - | - \
             | -:6 A4:2 -:1 A4+B4 -:2 | B3+D4+F4+A4+F5:1 C4+E4+B4+E5 B3+D4+F4+C5+F5 \
             A3+C4+E4+B4+G5 B3+D4+A4+F5 A3+C4+E4+E5+G5 G3+B3+D4+F5+A5 A3+C4+E4+G4+G5 B3+D4+F5 \
             C4+E4+E5 B3+D4+F4+F5 C4+E4+G4+E5 | B3+D4+F4+A4+F5 A3+C4+G4+B4+G5 B3+D4+C5+F5 \
             C4+E4+B4+E5 D4+F4+A4+D5 - D4+F4+A4+D5 E4+G4+B4+C5 D4+F4+A4+B4 E4+A4+B4 F4+G4+A4+C5 \
             E4+G4+A4+B4 | D4+A4+B4+C5 E4+B4+C5 F4+A4+C5+D5 G4+B4+E5 F4+A4+C5+D5 E4+G4+B4+D5+E5 \
             - G4+B4 D4+F4+A4+F5 E4+G4+B4+E5 D4+F4+A4+F5 C4+G4+G5 | D4+F4+A4+F5 C4+E4+G5 \
             B3+D4+F5+A5 C4+E4+E5+G5 D4+F4+D5+F5 E4+G4+C5 A4+F5 E4+B4+E5 D4+F4+A4+D5+F5 C4+E5 \
             D4+A4+F5 E4+B4 | D4+F4+A4+F5 - B3+D4+F5+A5 C4+E4+E5+G5 D4+F4+D5+F5 E4+G4+C5 \
             D4+A4+F5 E4+B4+E5 D4+F4+A4+D5+F5 C4+G4+E5+G5 D4+A4+F5 - | D4+F4+A4+F5 C4+E4+G5 -:3 \
             E4+G4+B4:1 -:2 D4+F4+A4+C5+F5:1 C4+G4+D5+G5 D4+G4+A4+F5 - & G3:2 -:3 C4:1 - C4 - C4 \
             - C4 | D4 -:6 C4:1 - C4 - C4 | D4 -:11 | -:3 C4+C5:1 B3+F4+A4+B4 -:7 | \
             F4+A4+B4+D5:1 -:4 E4+A4+C5+E5:1 F4+G4+A4+B4+D5 G4+A4+C5 -:2 B4:1 G4 | -:9 E4+C5:1 \
             -:2 | -:8 F4+A4+B4+D5:1 G4+C5 -:2 | - F4+A4+B4+D5:1 G4+C5 -:2 G4+B4:1 A4 -:4 | \
             F4+D5:1 -:2 E4+C5:1 -:2 F4+G4+B4+D5:1 -:4 E4+C5+E5:1 | B4 G4+C5+E5 F4+A4+B4+D5 \
             G4+C5 A4+B4 - F4+B4+D5 A4+C5 B4 E4+C5 F4+B4+D5 - | -:2 F4+A4+B4+D5:1 G4+C5 A4+B4 - \
             F4+G4+B4+D5 G4+A4+C5 - E4+C5 -:2 | - F4+A4+B4+D5:1 -:3 F4+B4+D5:1 G4+A4+C5 - \
             E4+C5+E5 F4+B4+D5 - & B3+D4+F4+A4+D5+F5:1 C4+E4+E5+G5 D4+F4+D5+F5 A3+E4+E5+G5 \
             B3+F4+D5+F5 A3+E4+E5+G5+B5 B3+F4+D5+F5+A5 A3+E4+E5+G5+B5 B3+F4+D5+F5+A5 \
             A3+E4+E5+G5+B5 B3+F4+D5+F5+A5 A3+E4+E5+G5+B5 | B3+F4+B4+D5+F5 A3+E4+C5+G5 B3+D5+F5 \
             A3+C5+E5+G5 G3+B3+D5+F5+A5 A3+C4+E4+E5+G5 B3+D4+F4+D5+F5 A3+E4+E5+G5+B5 \
             B3+F4+D5+F5+A5 A3+E4+E5+G5+B5 B3+F4+D5+F5+A5 A3+E4+E5+G5+B5 | B3+F4+B4+D5+F5 \
             A3+E4+C5+G5 B3+D4+F4+D5+F5 A3+C4+E4+E5+G5 G3+B3+D4+F5+A5 A3+C4+E4+G5 B3+D4+F4+B4+F5 \
             A3+E4+C5+G5+B5 B3+D5+F5+A5 A3+C4+E5+G5+B5 G3+D4+F5+A5+C6 A3+E4+C5+G5+B5 | \
             B3+D4+F4+B4+A5 A3+C4+E4+C5+B5 B3+D4+F4+D5+F5 A3+E4+E5+G5 G3+D4+D5+F5+A5 A3+E4+G5 \
             G4+G5:6 | _:3 -:9 | -:6 G4+G5 | _:3 -:9 | -:12 | -:6 A4+C5+F5:1 -:5 | -:6 G4+G5 | \
             _:3 -:8 E4+B4+G5:1 | -:2 B3+D4+E5+A5:1 C4+E4+D5+G5 D4+F4+C5+F5 - D4+G4+A4+E5+F5+G5 \
             E4+B4+D5+E5 -:3 E4+A4+G5:1 & -:1 A3 B3+A4 C4 D4+A4 - D4+A4 - D4+A4 - D4+A4 - | - \
             C4+E5 D4+F4+A4:2 D4+F4+A4+B4:1 C5 A4+B4 - D4+A4 - D4+A4 - | - C4+E5 A4+B4 C5 \
             F4+A4+B4+D5 C5+E5 A4+D5 C4+E5 D4+F4+A4+B4 C5 F4+A4+B4+D5+E5 E5+F5 | E5 E5+F5 A4+B4 \
             -:2 C4+A4+C5+E5:1 A4+B4+D5 A4+C5+E5 F4+B4+D5 C5 F4+A4+B4+D5 C5+E5 | - C5 A4+B4+D5 \
             C5 B4+D5 -:3 B4:1 C5+E5 -:2 | -:1 G4+C5+E5 A4+B4+D5 E4+C5+E5 F4+D5 A4+E5 B4 E4+E5 \
             D5 - D5 C5+E5 | B4+D5 C5 A4+B4+D5 G4+C5+E5 F4+B4+D5 G4+A4+C5 F4+G4+A4+B4+D5 \
             A4+C5+E5 -:2 A4+B4+D5:1 C5 | B4 E4+C5+E5 -:2 B4:1 A4+C5+E5 -:2 D5:1 G4+C5+E5 B4 C5 \
             | - G4+E5 B4 - B4 A4+C5 - E4+A4+C5+E5 B4+D5 C5 B4+D5 - | -:5 A4+E5:1 -:5 C5+E5:1 | \
             B4+D5 C5+E5 -:3 A4+E5:1 -:2 B4:1 - F4+B4+D5 G4+C5+E5 | B4 G4+C5+E5 - G4+C5+E5 \
             A4+B4+D5 A4+C5+E5 -:2 B4+D5:1 -:2 G4+C5+E5:1",
        )
        .in_time(SIX_EIGHT)
        .with_tempo(&[tempo(0, 104)])
        .in_key(D, Dorian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Snare, "X:2 x:1 x x x x x x x X:2 | - x:1 x X:2 x:1 x X:4 | x:1 x x x X:2 x:1 \
                           x X:2 x:1 x | X:2 x:1 x X:8 | -:12 | - | - | - | - | - | - | -:10 (3 \
                           -:1 x x"),
        ])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 970 to 977.
        Part::new(
            "Gypsy girls, last time",
            "F5:2. -:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5 | F5 F5:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5 | \
             F5 F5:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5 | F5 F5:1 D5 F5 D5 F5 A5 | E5:2 C5 E5 G5:1 -",
            "",
        )
        .with_left(
            "D2:1 -:7 | -:8 | - | - | - | - | - | - & F2+A2+F3+A4:1 -:7 | -:8 | - | - | - | - | \
             - | - & -:1 C4+D4 D4 D4+E4 F4 D4+G4 D4+A4 D4+B4 | C5 D4+B4 D4+A4 D4+G4 F4 D4+E4 D4 \
             C4+D4 | - C4+D4 D4 D4+E4 F4 D4+G4 D4+A4 D4+B4 | C5 D4+B4 D4+A4 D4+G4 F4 D4+E4 D4 \
             C4+D4 | - C4+D4 D4 D4+E4 F4 D4+G4 D4+A4 D4+B4 | C5 D4+B4 D4+A4 D4+G4 F4 D4+E4 D4 \
             C4+D4 | - C4+D4 D4 D4+E4 F4 D4+G4 D4+A4 D4+B4 | C5 D4+B4 D4+A4 D4+G4 F4 D4+E4 D4 \
             C4+D4 & G4+D5:1 -:7 | -:8 | - | - | - | - | - | - & D4:1 D5 D5+A5 A5 D4+D5+A5 A5 \
             D5+A5 D5 | D4+D5+A5 D5+A5 D5+A5 D5+A5 D4+D5+A5 D5+A5 D5+A5 D5+A5 | D4+D5+A5 D5+A5 \
             D5+A5 A5 D4+D5+A5 A5 D5+A5 D5 | D4+D5+A5 D5+A5 D5+A5 D5+A5 D4+D5+A5 D5+A5 D5+A5 \
             D5+A5 | D4+D5+A5 D5+A5 D5+A5 A5 D4+D5+A5 A5 D5+A5 D5 | D4+D5 D5 D5 D5 D4+D5+A5 \
             D5+A5 D5+A5 D5+A5 | D4+D5+A5 D5+A5 D5+A5 A5 D4+D5+A5 A5 D5+A5 D5 | D4+D5 D5 D5 D5 \
             D4+D5+A5 D5+A5 D5+A5 D5+A5 & A5:2. -:6 | -:8 | - | - | E5+E6 | A5+A6:4 - | E5+E6:8 \
             | A5+A6:4 -",
        )
        .with_tempo(&[tempo(0, 138)])
        .in_key(D, Dorian)
        .played_by(&[(gm::OBOE, 0)])
        .with_drums(&[
            (Drum::Snare, "x:8 | - | - | - | - | - | - | -"),
        ]),
        // MIDI bars 978 to 985.
        Part::new(
            "Trumpet and harp",
            "F4:2 F4:1 D4 F4 D4 F4 A4 | E4 -:7 | F4:2 F4:1 D4 F4 D4 F4 A4 | E4 -:7 | F4:2 F4:1 \
             D4 F4 D4 F4 A4 | E4 -:7 | F4:2 D4 F4 A4:1 - | -:8",
            "",
        )
        .with_left(
            "C4+G4:1 G4 C4+G4 G4 C4+G4 G4 C4+G4 G4 | C4+G4 G4 G4 G4 C4+G4 G4 C4 G4 | C4+G4 G4 \
             C4+G4 G4 C4+G4 G4 C4+G4 G4 | C4+G4 G4 G4 G4 C4+G4 G4 C4 G4 | Bb3 F4 Bb3 F4 Bb3 F4 \
             Bb3 F4 | Bb3+F4 F4 Bb3+F4 F4 Bb3+F4 F4 Bb3+F4 F4 | Bb3 F4 Bb3+F4 F4 Bb3 F4 Bb3+F4 \
             F4 | Bb3+F4 F4 Bb3+F4 F4 Bb3+F4 F4 Bb3+F4 F4 & -:8 | -:2 C4 E4 G4:1 - | -:8 | -:2 \
             C4 E4 G4:1 - | -:8 | -:2 C4 E4 G4:1 - | -:8 | E4:2 C4 E4 G4:1 -",
        )
        .with_tempo(&[glide(53, 106, 5)])
        .in_key(F, Major)
        .played_by(&[(gm::TRUMPET, 0)]),
        // MIDI bars 986 to 1004.
        Part::new(
            "Coachmen's Dance",
            "-:2 E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 \
             F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. \
             -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 \
             | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 \
             F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. \
             -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2 | - E4:1 F#3:2. -:1 E4:2",
            "",
        )
        .with_left(
            "E2:4 E2 | E2 E2 | E2 E2 | E2 E2 | -:8 | - | E2:4 E2 | E2 E2 | -:8 | E2:4 E2 | E2 E2 \
             | E2 E2 | E2 E2 | -:8 | E2:4 E2 | E2 E2 | -:8 | E2:4 E2 | E2 E2 & \
             E1+F#3+A3+D4+E4:2. -:6 | -:8 | - | -:4 E1+E4 | E1+E4 E1+E4 | E1+E4 E1+E4 | -:8 | \
             -:4 E1+E4 | -:8 | E1+E4:4 - | - E1+E4 | E1+E4 - | - E1+E4 | E1+E4 - | -:8 | - | -:4 \
             E1+E4 | E1+E4 - | -:8 & -:8 | - | - | -:4 F#3:3 -:1 | -:4 F#3:3 -:1 | -:4 F#3:3 -:1 \
             | -:8 | -:4 F#3:3 -:1 | E1+C#4+E4:4 E1+B3+E4 | -:8 | -:4 F#3:3 -:1 | -:8 | -:4 \
             F#3:3 -:1 | -:4 E1+C#4+E4 | E1+B3+E4 - | - E1+B3+E4 | E1+B3+E4 F#3:3 -:1 | -:8 | - \
             & -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 \
             A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 \
             A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 \
             F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | \
             -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 \
             -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3. | -:2 A3:1 -:3 A3:1 F#3 & E3:4 E3 | E3 E3 | \
             E3 E3 | E3 E3 | -:8 | - | E3:4 E3 | E3 E3 | -:8 | E3:4 E3 | E3 E3 | E3 E3 | E3 E3 | \
             -:8 | E3:4 E3 | E3 E3 | -:8 | E3:4 E3 | E3 E3 & -:8 | - | - | -:4 A3+D4:2 - | A3+D4 \
             - A3+D4 - | A3+D4 - A3+D4 - | -:8 | -:4 A3+D4:2 - | A3+D4 - A3+D4 - | A3+D4 -:6 | \
             -:4 A3+D4:2 - | A3+D4 -:6 | -:4 A3+D4:2 - | A3+D4 - A3+D4 - | A3+D4 -:6 | -:4 \
             A3+D4:2 - | A3+D4 - A3+D4 - | A3+D4 -:6 | -:8 & -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | \
             -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - \
             D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | \
             -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - \
             D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 - | -:2 D4 - D4:1 -",
        )
        .with_tempo(&[tempo(0, 112)])
        .in_key(E, Mixolydian)
        .played_by(&[(gm::FRENCH_HORN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1005 to 1014.
        Part::new(
            "Trumpet, violins, trombone",
            "-:2 E4:1 F#3. E5:4 | E5 E5 | E5 - | - B4:2 D5 | C#5:4 B4 | A4 E4:2 E4 | A4 A4 G#4 \
             G#4 | F#4 -:6 | -:4 E4:2 G#4 | B4 B4 E5:4",
            "",
        )
        .with_left(
            "E2:4 E2 | -:8 | E2:4 E2 | E2 E2 | E2 E2 | E2 E2 | E2 E2 | E2 E2 | E2 E2 | E2 E2 & \
             E3:4 E3 | -:8 | E3:4 E3 | E3 E3 | E3 E3 | E3 E3 | E3 E3 | E3 E3 | E3 E3 | E3 E3 & \
             -:2 A3:1 F#3:2. -:1 A3+D5+E5 F#3+F#4. | -:2 A3+D5+E5:1 F#3+F#4:2. -:1 A3+D5+E5 \
             F#3+F#4. | -:2 A3+D5+E5:1 F#3+F#4:2. -:1 A3+D5+E5 F#3+F#4. | -:2 A3+D5+E5:1 \
             F#3+F#4:2. -:1 A3+D4+E4+A4+E5 F#3+F#4. | -:2 A3+D5+E5:1 F#3+F#4:2. -:1 A3+D5+E5 \
             F#3+F#4. | -:2 A3+D5+E5:1 F#3+F#4:2. -:1 A3+D4+A4+D5+E5 F#3+F#4. | -:2 \
             A3+D4+E4+D5+E5:1 F#3+F#4:2. -:1 A3+D5+E5 F#3+F#4. | -:2 A3+D5+E5:1 F#3+F#4:2. -:1 \
             A3+D5+E5 F#3+F#4. | -:2 A3+D5+E5:1 F#3+F#4:2. -:1 A3+D5+E5 F#3+F#4. | -:2 \
             A3+D5+E5:1 F#3+F#4:2. -:1 A3+D5+E5 F#3+F#4 & -:2 D4 -:4 | -:8 | - | - | - | - | - | \
             - | - | -",
        )
        .in_key(E, Mixolydian)
        .played_by(&[(gm::TRUMPET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 1015 to 1029.
        Part::new(
            "Woodwinds",
            "-:2 E5 D6:1 -:3 | D5:2. -:4 E5:1 - | -:2 E5:1 -:3 E5:1 - | -:2 E5:1 -:3 E5:1 - | \
             -:2 E5:1 -:3 E5:1 - | -:2 E5 D6:1 -:3 | D6:1 -:3 D5:1 -:3 | D4:2. -:4 D6:1 - | -:2 \
             C#6:1 -:3 B5:1 - | -:2 A5:1 -:3 A5:1 - | -:2 D6:1 -:3 C#6:1 - | -:2 B5:1 -:3 E6:1 - \
             | -:2 D6:1 -:3 C#6:2 | B5:1 -:3 D6:1 -:3 | D6:1 -:3 D5:1 -:3",
            "",
        )
        .with_left(
            "E2+E5:4 - | - E2 | E2 E2 | E2 E2 | -:8 | E4:2 - B4+B5:4 | - F#3+A3:1 -:3 | E2:2 -:6 \
             | -:8 | - | - | - | - | - | -:4 F#3+A3:1 -:3 & -:2 A3+D4+E4+A4+D5:1 F#4 F#5+A5+E6 \
             -:3 | -:4 E3 | E3 E3 | E3 E3 | E3 E3 | -:3 F#3:2. -:3 | -:8 | F#3+A3+E4:2. -:6 | \
             -:8 | - | - | - | - | - | -:4 E4+F#4+A4:1 -:3 & E4:2 - A4+D5+E5:1 -:3 | D4+E4+E5:2. \
             -:6 | -:8 | -:4 G#3+G#4:2 F#3+F#4:1 - | E2+B3+B4:4 E2+E4+E5 | E2+E5 - | C#4+C#5 \
             D3+D4 | -:6 E4:1 - | -:2 D4+E4+F#4:1 -:3 G#3+D4+E4+G#4:1 - | -:2 D4+E4:1 -:3 \
             D4+E4:1 - | -:2 D4+E4:1 -:3 D4+E4+F#4:1 - | -:2 G#3+D4+E4+G#4:1 -:3 A3+D4+E4+A4:1 - \
             | -:2 B3+D4+E4+B4:1 -:3 C#4+D4+E4+C#5:2 | C#4+E4+C#5:1 -:3 B4+B5:4 | C#4+C#5 D3+D4 \
             & -:8 | F#3+A3:1 -:3 B3+B4:2 - | C#4+C#5:4 B3+B4 | A3+A4:2 -:6 | -:8 | -:4 E5:1 -:3 \
             | E5:1 -:3 E5:1 -:3 | -:8 | - | - | - | - | -:6 F#4+E5:2 | F#3+B3+E5:4 E5+E6:1 -:3 \
             | E5:1 -:3 E5:1 -:3 & -:3 F#3:2. -:3 | F#4+A4:2. -:4 A3+D4+E4+A4+D5:1 F#3+F#4. | \
             -:2 A3+D4+A4+D5:1 F#3+F#4:2. -:1 A3+D4+A4+D5 F#3+F#4. | -:2 A3+D4+A4+D5:1 \
             F#3+F#4:2. -:1 A3+D4+A4+D5 F#3+F#4. | -:2 A3+D4+A4+D5:1 F#3+F#4:2. -:1 A3+D4+A4+D5 \
             F#3+F#4. | -:2 A3+D4+E4+A4+D5:1 F#4 F#5+A5+F#6+A6+E7 -:3 | F#4+A4+F#5+A5+E6:1 -:3 \
             E4+F#4+A4:1 -:3 | -:6 E3+D5+F#5+E6:1 - | -:2 E3+F#5+D6+E6:1 -:3 E3+F#5+D6+E6:1 - | \
             -:2 E3+F#5+D6+E6:1 -:3 E3+F#5+D6+E6:1 - | -:2 E3+F#3+D5+F#5+E6:1 -:3 E3+F#5+D6+E6:1 \
             - | -:2 E3+F#5+D6+E6:1 -:3 E3+F#3+D5+F#5+D6:1 - | -:2 E3+F#3+D5+F#5+E6:1 -:3 \
             E3+F#3+D5+F#5+D6+E6:1 - | E3+B4+F#5+D6+E6 -:3 F#5+A5+F#6+A6+E7:1 -:3 | \
             F#4+A4+F#5+A5+E6:1 -:7",
        )
        .in_key(E, Mixolydian)
        .played_by(&[(gm::OBOE, 0)])
        .with_drums(&[
            (Drum::Kick, "-:8 | x | - | - | - | - | - | x | - | - | - | - | - | - | -"),
            (Drum::Crash, "-:4 x | -:8 | - | - | - | -:4 x | -:8 | - | - | - | - | - | - | -:4 x \
                           | -:8"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1030 to 1040.
        Part::new(
            "Flutes and clarinets",
            "-:6 A5:1 - | -:2 A5 - B5 | - C#6 - D6 | - E6 - F#6 | - E6 - F#6 | - G#6 - A6 | - B6 \
             A6:1 -:3 | -:6 D6:1 - | -:2 E6:1 -:3 F#6:1 - | -:2 G#6:1 -:3 F#6:1 - | -:2 G#6:1 \
             -:3 A6:1 -",
            "",
        )
        .with_left(
            "-:8 | -:2 B3:1 - B3:4 | A3 - | -:8 | -:4 E3:2 G#3 | B3 B3 -:4 | -:8 | E2:2 -:6 | \
             -:8 | -:4 G#3:2 F#3 | -:8 & E2:2 - B3 B3:1 - | C#4:4 -:2 C#4:1 - | -:2 D4:1 -:5 | \
             A4:2 F#4:1 - G#4:2 G#4:1 - | F#4:4 - | -:8 | -:4 B4+B5 | C#5 B3+E4:2 D4 | C#4+E4:4 \
             B3+E4 | A3+E4 E4 | B3+E4 E4 & -:6 A5:2 | -:6 A4:2 | - G#4+B4 - F#4+A4+C#5 | - \
             G#4+B4 - D4+F#4+A4+C#5 | - C#4+G#4+B4 - B3+D4+F#4+A4 | - A3+C#4+G#4 - G#3+B3+D4+F#4 \
             | F#3+A3:4 - | E3 - | -:8 | -:2 D4+F#4 - C#4+G#4+B4 | - D4+F#4+A4 - A3+C#4+G#4 & \
             -:6 B4:1 - | -:2 B4:1 -:3 C#5:1 - | -:8 | - | - | - | -:4 E5+E6:1 -:3 | -:6 F#5:2 | \
             -:8 | -:7 F#4+A4+A5:1. | -:3 G#4+B4+B5:2. -:3 & F#3+A3+D4+E4:2. - E4+D5+E5:4 | \
             E4+D5+E5 E4+D5+E5 | E4+D5+E5 E4+D5+E5 | E4+D5+E5 E4+D5+E5 | E4+D5+E5 E4+D5+E5 | \
             E4+D5+E5 E4+D5+E5 | E4+D5+E5 - | F#3+A3+C#4+D4+E4:2. -:4 E4+G#4+B4+G#5+B5:1 - | -:2 \
             F#4+A4+C#5+A5+C#6:1 F#5:2. -:1 G#4+B4+B5+D6 F#5+G#5. | -:2 A4+C#5+C#6+E6:1 \
             G#5+A5:2. -:1 B3+F#5+D6+A6 B5 | -:2 C#4+G#5+E6+B6:1 C#6 -:2 D4+A5+F#6+C#7:1 \
             A4+C#5+C#6+D6 & -:6 D4+F#4+F#5:1 A4+B4. | -:2 D4+F#4+F#5:1 A4+B4:2. -:1 G#4+G#5 \
             B4+C#5. | -:2 F#4+A4+F#5+A5:1 C#5:2. -:1 G#4+B4+G#5+B5 - | -:2 A4+C#5+A5+C#6:1 \
             F#5:2. -:1 B4+B5+D6 F#5+G#5. | -:2 A4+C#5+C#6+G#6:1 G#5+A5:2. -:1 B4+F#5+D6+A6 \
             A5+B5. | -:2 C#5+G#5+E6+B6:1 B5+C#6:2. -:1 F#5+A5+F#6+C#7 C#6+D6. | -:2 \
             G#5+B5+G#6+D7:1 D6+E6 D5+A5+C#6+F#6+E7 -:3 | -:4 D5+E5 | D5+E5 D5+E5 | D5+E5 D5+E5 \
             | D5+E5 D5+E5",
        )
        .in_key(E, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Kick, "x:8 | - | - | - | - | - | - | x | - | - | -"),
            (Drum::Crash, "-:8 | - | - | - | - | - | -:4 x | -:8 | - | - | -"),
        ])
        .marked(Dynamic::Forte),
        // MIDI bar 1041.
        Part::new(
            "Flutes and clarinets, bar 12",
            "-:2 G#6:1 E6:2. -:1 A6 F#6:2. -:1 B6 G#6",
            "",
        )
        .with_left(
            "-:2 G#3+B3+D4+F#4 - F#3+A3+C#4 - E3+G#3+B3+D4 & E4:2 - E4:4 E4 & -:3 B4:2. - C#5. \
             -:3 & D5+E5:4 D5+E5 D5+E5 & -:2 E4+G#5+B5+D7:1 D6 -:2 F#4+A5+C#6+E7:1 E6 -:2 \
             G#4+B5+D6+F#7:1 F#5+F#6",
        )
        .in_time(THREE_FOUR)
        .in_key(E, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1042 to 1053.
        Part::new(
            "Clarinet and trombones",
            "G5:6 E5:2 | F5:4 E5:2 D5 | C5:8 | G5:4 G5:2 E5 | A5:4 A5 | G5 G5:2 E5 | D5:4 D5:2 \
             E5 | D5:4 C5:2 B4 | A4:4 G4 | C5 C5 | B4 A4 | G4:6 -:2",
            "",
        )
        .with_left(
            "G1+G3+E4+C5+E5:1 -:7 | -:8 | -:1 A3+C4+F4 G3+B3+E4 F3+A3+D4 E3+G3+C4:2 E3+G3+C4:1 - \
             | -:8 | - | -:1 G3+B3+D4 A3+C4+E4 B3+D4+F4 C4+E4+G4:2 C4+E4+G4:1 - | -:8 | - | -:1 \
             B3+E4+G4 A3+D4+F4 G3+C4+E4 F3+B3+D4:2 F3+B3+D4:1 - | -:8 | - | -:1 E3+G3+C4 \
             F3+A3+D4 G3+B3+E4 A3+C4+F4:2 A3+C4+F4:1 - & -:2 B2+D3+G3:1 -:3 C3+E3+A3:1 - | -:2 \
             D3+F3+B3:1 -:3 D3+F3+B3:1 - | -:8 | -:2 G3+B3+D4:1 -:3 A3+C4+E4:1 - | -:2 \
             B3+D4+F4:1 -:3 B3+D4+F4:1 - | -:8 | -:2 C4+F4+A4:1 -:3 B3+E4+G4:1 - | -:2 \
             A3+D4+F4:1 -:3 A3+D4+F4:1 - | -:8 | -:2 A3+C4+F4:1 -:3 G3+B3+E4:1 - | -:2 \
             F3+A3+D4:1 -:3 F3+A3+D4:1 - | (3 G3 A3+A4 B3+B4 (3 C4+C5 D5 E5 (3 F5 G4+G5 B4+B5 (3 \
             C5+C6 D5+D6 E5+E6 & E6+C7+G7:1 -:7 | -:8 | - | - | - | - | - | - | - | - | - | -",
        )
        .in_key(G, Mixolydian)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 1054 to 1065.
        Part::new(
            "Woodwinds in C",
            "-:2 G4. - A4. | - B4. - B4. | -:1 F5 E5 D5 C5:2 C5:1 - | -:2 D5. - E5. | - F5. - \
             F5. | -:1 D5 E5 F5 G5:2 G5:1 - | -:2 A5. - G5. | - F5. - F5. | -:1 G5 F5 E5 D5:2 \
             D5:1 - | -:2 F5. - E5. | - D5. - D5:1 - | - C5 D5 E5 F5:2 F5:1 -",
            "",
        )
        .with_left(
            "-:8 | - | - | - | - | - | - | - | - | -:2 A3+C4. - G3+B3. | - F3+A3. - F3+A3+D4:1 - \
             | - E3 F3 G3 A3:2 - & -:2 B2+D3+G3. - C3+E3+A3. | - D3+F3+B3. - D3+F3+B3. | -:1 \
             A3+A4 G3+G4 F3+F4 E3+E4:2 E3+E4:1 - | -:2 G3+B3. - A3+C4+E4. | - B3. - B3. | -:1 G3 \
             A3 B3 C4:2 C4:1 - | -:6 B3:2. | - A3+F4. - A3. | -:1 B3+B4 A3+A4 G3+G4 F3:2 F3:1 - \
             | -:2 F4:1. -:3 E4:1. - | -:8 | -:6 A3+A4:1 - & G4:1 -:7 | -:4 E4+E6:2 - | C4+C6:8 \
             | G4:1 -:3 G4:1 -:3 | -:8 | G4:1 -:3 G4:1 - E4+E6:2 | -:4 D4:1 -:3 | D4:2 - C4+C6 \
             B3 | A3 - G3:4 | C4:2 - C4:4 | - A3:2 - | (3 G3:1 A3 B3 (3 C4 D4 E4 F4+G4 A4 C5 E5 \
             & D4+F4+A4:1 G4+B4+D5 D4+F4+A4 G4+B4+D5 D4+F4+A4 G4+B4 C5+E5+G5 G4+B4 | - G4+B4+D5 \
             - G4+B4+D5 - G4+B4+D5 - G4+B4+D5 | - G4+B4+D5 A4+C5+E5 B4+F5 E5+G5 - E5+G5 - | -:2 \
             D4+F4+A4:1 D5 -:2 G5:1 G4+B4 | D4+F4+A4 - D4+F4+A4 G4 D4+F4+A4 - D4+F4+A4 G4 | \
             D4+F4+A4 -:7 | D4+F4+A4:1 E4:2. -:3 C4+E4+G4+C5:1 - | E4+F4+G4+B4 F4+A4+C5 D4+G4+B4 \
             E4+C5+E5 D4+F4+B4+D5+F5 - D4+F4+A5 - | - F4+A4+C5 D4+G4+B4+D5 E4+A4+C5 D4+F4+B4+F5 \
             -:3 | -:2 A4+C5. - G4+B4. | -:1 C4+E4+A4+C5+E5 F4+A4:2. -:4 | (3 B3+D4+G4:1 \
             C4+E4+A4 D4+F4+B4 (3 E4+G4+C5 F4+A4+D5 - -:4 & -:8 | - | - | - | - | - | -:2 \
             C4+F4+A4:1 D4+G4+B4+D5 C4+E4+A4+C5+E5 D4+F4+B4+D5+F5 - F4+A4+D5+F5+A5 | -:2 A4+D5. \
             -:1 E4+G4+C5+E5+G5 A4+D5:2. | -:5 E4+G4+C5+E5+G5:1 D4+F4+A4+F5+A5 - | - G4+B4+D5 \
             E4+E5 F4+B4+D5+F5 D4+G4+C5+E5+G5 -:3 | -:2 D4+B4+F5:1 E4+G4+C5+E5+G5 D4+F4+D5+F5+A5 \
             -:3 | -:2 (3 -:1 - G4+B4+E5 (3 A4+B4+D5+G5 B4+C5+E5+A5 - -:2 & -:2 B3. - C4+E4. | \
             D4+F4+A4:1 - D4+F4+A4 - D4+F4+A4 - D4+F4+A4 - | D4+F4+A4 -:4 G4+B4+D5:1 - G4+B4+D5 \
             | D4+F4+A4 G4+B4+D5 G4+B4:2. D4+F4+A4:1 G4+B4 A4+C5:2. | -:1 G4+B4+D5 B4+D5:2. -:1 \
             G4+B4+D5 B4+D5:2. | -:1 G4+B4 A4+C5+E5 B4+D5 C5+E5 G4+B4+D5 C5+E5 G4+B4+D5 | - \
             G4+B4 C5+F5:2. - B4+E5. | E5+G5+B5. -:5 E4+G5+B5:1 | F4+A4+C5+F5+A5+C6 -:6 \
             E4+G5+B5:1 | F4+A4+C5+F5+A5+C6 -:4 E4+A4+D5+F5+A5:1 D4+G5+B5 E4+F5+A5+C6 | B3+G5+D6 \
             -:4 E4+G4+E5+G5+B5:1 F4+A4+F5+A5+C6 G4+B4+G5+B5+D6 | -:4 (3 -:1 - D5+F5+B5 (3 \
             E5+G5+C6 G5+B5+E6 A5+C6+F6 & G6:6 E6:2 | F6:4 -:2 D6 | -:8 | G6:4 G6:2 E6 | A6:4 A6 \
             | G6 G6:2 - | D6:4 D6:2 E6 | D6:4 -:2 B5:1 - | -:4 G5:1 -:3 | -:4 C6:3 -:1 | B5:4 - \
             | G6 -",
        )
        .in_key(C, Major)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 1066 to 1083.
        Part::new(
            "The coachmen, tutti",
            "A6:1 F#6 E6 F#6 E6 F#6 E6 F#6 | E6 F#6 E6 F#6 E6 F#6 E6 F#6 | E6 -:7 | E6:1 D6 E6 \
             D6 E6 D6 E6 D6 | E6 D6 E6 D6 C#6 D6 E6 F#6 | E6 -:7 | E6:1 F#6 E6 F#6 G#6 F#6 E6 D6 \
             | E6 D6 E6 D6 C#6 B5 C#6 D6 | E6 -:7 | E6:1 F#6 G#6 F#6 G#6 A6 G#6 F#6 | E6 F#6 E6 \
             F#6 G#6 F#6 E6 D6 | E6 -:7 | E6:1 D6 E6 D6 C#6 D6 E6 F#6 | E6 F#6 G#6 F#6 G#6 A6 \
             G#6 F#6 | E6 -:7 | E6:1 D6 E6 D6 C#6 B5 C#6 D6 | E6 D6 E6 D6 C#6 D6 E6 F#6 | E6 -:7",
            "",
        )
        .with_left(
            "E1+E2:2. - E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 & -:3 \
             F#4:1 -:3 F#4:1 | -:3 F#4:1 -:3 F#4:1 | -:2 A3+E4:1 F#4 -:2 A3+E4:1 F#4 | -:8 | -:7 \
             F#4:1 | -:2 A3+E4:1 F#4 -:2 A3+E4:1 F#4 | -:3 F#4:1 -:4 | -:6 A3:1 - | -:2 A3+E4:1 \
             F#4 -:2 A3+E4:1 F#4 | -:2 A3+E4:1 F#4 -:2 A3+E4:1 F#4 | -:3 F#4:1 -:4 | -:2 E4:1 \
             F#4 -:2 A3:1 F#4 | -:2 A3:1 -:3 A3:1 F#4 | -:2 E4:1 F#4 -:2 E4:1 F#4 | -:2 E4:1 F#4 \
             -:2 E4:1 F#4 | -:6 A3+E4:1 - | -:2 A3:1 -:3 A3:1 F#4 | -:2 E4:1 F#4 -:2 E4:1 F#4 & \
             A3+E4+E5+C#6:1 G#3+D4+F#5 A3+E4+E5 G#3+D4+F#5 A3+E4+E5 G#3+D4+F#5 A3+E4+E5 \
             G#3+D4+F#5 | A3+C#4+E4+C#6 G#3+B3+D4+D6 A3+C#4+E4+C#6 G#3+B3+D4+D6 A3+C#4+E4+C#6 \
             G#3+B3+D4+D6 A3+C#4+E4+C#6 G#3+B3+D4+D6 | A3+C#4+E5+C#6 B3+F#5+D6 C#4+E5+C#6 \
             B3+F#5+D6 A3+E5+E6 B3+D6 C#4+C#6 D4+B5 | A3+C#4+E4+A5+C#6 B3+F#4+G#5 A3+E4+A5 \
             B3+F#4+G#5 A3+E4+A5 B3+F#4+G#5 A3+E4+A5 B3+F#4+G#5 | A3+C#4+E4+A5+C#6 \
             B3+D4+F#4+G#5+B5 A3+C#4+E4+A5+C#6 B3+D4+F#4+G#5+B5 C#4+E4+G#4+F#5+A5 \
             B3+D4+F#4+G#5+B5 A3+C#4+E4+A5+C#6 G#3+B3+D4+B5+D6 | A3+C#4+A5+C#6 D4+B5 C#4+C#6 \
             D4+B5 C#4+C#6 D4+B5 C#4+C#6 D4+B5 | A3+C#4+E4+C#6 G#3+D4 A3+E4 G#3+D4 F#3+C#4 \
             G#3+D4 A3+E4 B3+F#4 | A3+C#4+E4+A5+C#6 B3+D4+F#4+G#5+B5 A3+C#4+E4+A5+C#6 \
             B3+D4+F#4+G#5+B5 C#4+E4+G#4+F#5+A5 D4+F#4+A4+E5+G#5 C#4+E4+G#4+F#5+A5 \
             B3+D4+F#4+G#5+B5 | A3+A5+C#6 D4+G#5+B5 C#4+A5+C#6 D4+G#5+B5 E4+F#5+A5 D4+G#5+B5 \
             C#4+A5+C#6 B3+B5+D6 | E4+C#6 D4 C#4 D4 C#4 B3 C#4 D4 | A3+C#4+E4+C#6 G#3+B3+D4+D6 \
             A3+C#4+E4+C#6 G#3+B3+D4+D6 F#3+A3+C#4+E6 G#3+B3+D4+D6 A3+C#4+E4+C#6 B3+D4+F#4+B5 | \
             A3+C#4+A5+C#6 B3+D4+G#5+B5 A3+C#4+A5+C#6 B3+D4+G#5+B5 C#4+E4+F#5+A5 D4+F#4+E5+G#5 \
             C#4+E4+F#5+A5 B3+D4+G#5+B5 | A3+C#4+E4+A5+C#6 F#4+G#5 E4+A5 F#4+G#5 G#4+F#5 F#4+G#5 \
             E4+A5 D4+B5 | E4 B3+D4+D6 A3+C#4+E6 B3+D4+D6 A3+C#4+E6 G#3+B3+F#6 A3+C#4+E6 \
             B3+D4+D6 | A3+C#4+C#6 G#3+B3+D6 A3+C#4+C#6 G#3+B3+D6 F#3+A3+E6 G#3+B3+D6 A3+C#4+C#6 \
             B3+D4+B5 | A3+C#4+E4+A5+C#6 B3+F#4+G#5 A3+E4+A5 B3+F#4+G#5 C#4+G#4+F#5 D4+A4+E5 \
             C#4+G#4+F#5 B3+F#4+G#5 | A3+C#4+E4+A5+C#6 D4+F#4+G#5+B5 C#4+E4+A5+C#6 D4+F#4+G#5+B5 \
             E4+G#4+F#5+A5 D4+F#4+G#5+B5 C#4+E4+A5+C#6 B3+D4+B5+D6 | C#4 B3 A3 B3 A3 G#3 A3 B3",
        )
        .in_key(E, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Kick, "-:4 x | x x | x x | x x | x x | x x | x x | x x | x x | x x | x x | x \
                          x | x x | x x | x x | x x | x x | x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1084 to 1103.
        Part::new(
            "The coachmen dance on",
            "E6:1 F#6 E6 F#6 G#6 F#6 E6 D6 | E6 D6 E6 D6 C#6 B5 C#6 D6 | E6 -:7 | E6:1 F#6 G#6 \
             F#6 G#6 A6 G#6 F#6 | E6 F#6 E6 F#6 G#6 F#6 E6 D6 | E6 -:7 | E6:1 D6 E6 D6 C#6 D6 E6 \
             F#6 | E6 F#6 G#6 F#6 G#6 A6 G#6 F#6 | E6 -:7 | E6:1 D6 E6 D6 C#6 B5 C#6 D6 | E6 D6 \
             E6 D6 C#6 D6 E6 F#6 | E6 -:7 | E6:1 F#6 E6 F#6 G#6 F#6 E6 D6 | E6 D6 E6 D6 C#6 B5 \
             C#6 D6 | E6 -:7 | E6:1 F#6 G#6 F#6 G#6 A6 G#6 F#6 | E6 F#6 E6 F#6 G#6 F#6 E6 D6 | \
             E6 -:7 | E6:1 D6 E6 D6 C#6 D6 E6 F#6 | E6 F#6 G#6 F#6 G#6 A6 G#6 F#6",
            "",
        )
        .with_left(
            "E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 | \
             E1+E2:1 -:3 E1+E2:1 -:3 | E1+E2:1 -:3 E1+E2:1 -:3 & -:3 F#4:1 -:4 | -:6 A3:1 - | \
             -:2 A3+E4:1 F#4 -:2 A3+E4:1 F#4 | -:2 A3+E4:1 F#4 -:2 A3+E4:1 F#4 | -:3 F#4:1 -:4 | \
             -:2 E4:1 F#4 -:2 A3:1 F#4 | -:2 A3:1 -:3 A3:1 F#4 | -:2 E4:1 F#4 -:2 E4:1 F#4 | -:2 \
             E4:1 F#4 -:2 E4:1 F#4 | -:6 A3+E4:1 - | -:2 A3:1 -:3 A3:1 F#4 | -:2 E4:1 F#4 -:2 \
             E4:1 F#4 | -:3 F#4:1 -:4 | -:6 A3:1 - | -:2 A3+E4:1 F#4 -:2 A3+E4:1 F#4 | -:2 \
             A3+E4:1 F#4 -:2 A3+E4:1 F#4 | -:3 F#4:1 -:4 | -:2 E4:1 F#4 -:2 A3:1 F#4 | -:2 A3:1 \
             -:3 A3:1 F#4 | -:2 E4:1 F#4 -:2 E4:1 F#4 & A3+C#4+E4+C#6:1 G#3+D4 A3+E4 G#3+D4 \
             F#3+C#4 G#3+D4 A3+E4 B3+F#4 | A3+C#4+E4+A5+C#6 B3+D4+F#4+G#5+B5 A3+C#4+E4+A5+C#6 \
             B3+D4+F#4+G#5+B5 C#4+E4+G#4+F#5+A5 D4+F#4+A4+E5+G#5 C#4+E4+G#4+F#5+A5 \
             B3+D4+F#4+G#5+B5 | A3+A5+C#6 D4+G#5+B5 C#4+A5+C#6 D4+G#5+B5 E4+F#5+A5 D4+G#5+B5 \
             C#4+A5+C#6 B3+B5+D6 | E4+C#6 D4 C#4 D4 C#4 B3 C#4 D4 | A3+C#4+E4+C#6 G#3+B3+D4+D6 \
             A3+C#4+E4+C#6 G#3+B3+D4+D6 F#3+A3+C#4+E6 G#3+B3+D4+D6 A3+C#4+E4+C#6 B3+D4+F#4+B5 | \
             A3+C#4+A5+C#6 B3+D4+G#5+B5 A3+C#4+A5+C#6 B3+D4+G#5+B5 C#4+E4+F#5+A5 D4+F#4+E5+G#5 \
             C#4+E4+F#5+A5 B3+D4+G#5+B5 | A3+C#4+E4+A5+C#6 F#4+G#5 E4+A5 F#4+G#5 G#4+F#5 F#4+G#5 \
             E4+A5 D4+B5 | E4 B3+D4+D6 A3+C#4+E6 B3+D4+D6 A3+C#4+E6 G#3+B3+F#6 A3+C#4+E6 \
             B3+D4+D6 | A3+C#4+C#6 G#3+B3+D6 A3+C#4+C#6 G#3+B3+D6 F#3+A3+E6 G#3+B3+D6 A3+C#4+C#6 \
             B3+D4+B5 | A3+C#4+E4+A5+C#6 B3+F#4+G#5 A3+E4+A5 B3+F#4+G#5 C#4+G#4+F#5 D4+A4+E5 \
             C#4+G#4+F#5 B3+F#4+G#5 | A3+C#4+E4+A5+C#6 D4+F#4+G#5+B5 C#4+E4+A5+C#6 D4+F#4+G#5+B5 \
             E4+G#4+F#5+A5 D4+F#4+G#5+B5 C#4+E4+A5+C#6 B3+D4+B5+D6 | C#4 B3 A3 B3 A3 G#3 A3 B3 | \
             A3+C#4+E4+C#6 G#3+D4 A3+E4 G#3+D4 F#3+C#4 G#3+D4 A3+E4 B3+F#4 | A3+C#4+E4+A5+C#6 \
             B3+D4+F#4+G#5+B5 A3+C#4+E4+A5+C#6 B3+D4+F#4+G#5+B5 C#4+E4+G#4+F#5+A5 \
             D4+F#4+A4+E5+G#5 C#4+E4+G#4+F#5+A5 B3+D4+F#4+G#5+B5 | A3+C#4+A5+C#6 D4+G#5+B5 \
             C#4+A5+C#6 D4+G#5+B5 E4+F#5+A5 D4+G#5+B5 C#4+A5+C#6 B3+B5+D6 | E4+C#6 D4 C#4 D4 C#4 \
             B3 C#4 D4 | A3+C#4+E4+C#6 G#3+B3+D4+D6 A3+C#4+E4+C#6 G#3+B3+D4+D6 F#3+A3+C#4+E6 \
             G#3+B3+D4+D6 A3+C#4+E4+C#6 B3+D4+F#4+B5 | A3+C#4+A5+C#6 B3+D4+G#5+B5 A3+C#4+A5+C#6 \
             B3+D4+G#5+B5 C#4+E4+F#5+A5 D4+F#4+E5+G#5 C#4+E4+F#5+A5 B3+D4+G#5+B5 | \
             A3+C#4+E4+A5+C#6 F#4+G#5 E4+A5 F#4+G#5 G#4+F#5 F#4+G#5 E4+A5 D4+B5 | E4 B3+D4+D6 \
             A3+C#4+E6 B3+D4+D6 A3+C#4+E6 G#3+B3+F#6 A3+C#4+E6 B3+D4+D6",
        )
        .in_key(E, Mixolydian)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Kick, "x:4 x | x x | x x | x x | x x | x x | x x | x x | x x | x x | x x | x \
                          x | x x | x x | x x | x x | x x | x x | x x | x x"),
        ])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1104 to 1105.
        Part::new(
            "The masqueraders",
            "D5:1 E5 D5 F#5 D5 E5 D5 F#5 D5 E5 D5 F#5 D5 E5 D5 F#5 | D5 E5 D5 F#5 D5 E5 D5 F#5 \
             D5 E5 D5 F#5 D5 E5 D5 F#5",
            "",
        )
        .with_left(
            "D4:1 F#4 D4 F#4 D4 F#4 D4 F#4 D4 F#4 D4 F#4 D4 F#4 D4 F#4 | D4 F#4 D4 F#4 D4 F#4 D4 \
             F#4 D4 F#4 D4 F#4 D4 F#4 D4 F#4",
        )
        .in_time(COMMON)
        .in_key(D, Major)
        .played_by(&[(gm::ACOUSTIC_GRAND_PIANO, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 1106 to 1112.
        Part::new(
            "Clarinet, chromatic",
            "-:16 | (3 D5:1 Eb5 D5 (3 C#5 - - -:8 (3 E5:1 F5 E5 (3 Eb5 - - | -:8 (3 D5:1 Eb5 D5 \
             (3 C#5 - - (3 F#5 G5 F#5 (3 F5 - - | (3 E5 F5 E5 (3 Eb5 - - (3 E5 F5 E5 (3 Eb5 - - \
             (3 E5 F5 E5 (3 Eb5 - - (3 E5 F5 E5 (3 Eb5 - - | (3 F#5 G5 F#5 (3 F5 - - (3 F#5 G5 \
             F#5 (3 F5 - - (3 F#5 G5 F#5 (3 F5 - - (3 G#5 A5 G#5 (3 G5 - - | (3 G#5 A5 G#5 (3 G5 \
             - - (3 Bb5 A5 G#5 (3 Bb5 - - (3 G#5 A5 G#5 (3 G5 - - (3 A5 Bb5 A5 (3 G#5 - - | (3 \
             G#5 A5 G#5 (3 G5 - - (3 Bb5 A5 G#5 (3 Bb5 - - (3 G#5 A5 G#5 (3 G5 - - (3 A5 Bb5 A5 \
             (3 G#5 - -",
            "",
        )
        .with_left(
            "F#4:1 G#4 B3+F4+A4 - F#4 G#4 B3+F4+A4 - F#4 G#4 B3+F4+A4 - G#4 A4 B3+F4+A4 - | F#4 \
             G#4 B3+F4+A4 - G#4 A4 B3+F4+A4 - F#4 G#4 B3+F4+A4 - F#4 G#4 Bb3+F4+C#5 - | F#4 G#4 \
             B3+F4+A4 - G#4 A4 B3+F4+A4 - F#4 G#4 B3+F4+A4 - G#4 Bb4 Bb3+G4+C#5 - | G#4 Bb4 \
             C#4+G4+B4 - Bb4 B4 C#4+G4+B4 - Bb4 B4 C#4+G4+B4 - Bb4 C5 C4+A4+Eb5 - | Bb4 C5 \
             Eb4+A4+C#5 - Bb4 C5 Eb4+A4+C#5 - Bb4 C5 Eb4+A4+C#5 - C5 D5 D4+B4+F5 - | C5 D5 \
             F4+B4+Eb5 - C5 D5 F4+B4+Eb5 - C5 D5 F4+B4+Eb5 - C5 D5 Eb4+B4+F#5 - | C5 D5 \
             F4+B4+Eb5 - C5+F#5+A5+C6 D5+D6 Eb6+F6:2. C5:1 D5 F4+B4+Eb5 -:5 & -:16 | (3 -:1 G#4 \
             - (3 - F#4+Bb4+C5 A4. _ -:2 F#4:1 -:3 F#4:1 (3 - G#4 - (3 - F#4+C5+D5 B4+C#5. | -:3 \
             F#4:1 -:2 A3+B4 D4+E4 A3+B4 D4+E4 - | E4+F#4 B3+C#5 D4+F4 B3+C#5 E4+F#4 B3+C#5 \
             E4+F#4 - | F#4+G#4 C#4+Eb5 F#4+G#4 C#4+Eb5 F#4+A4 C#4+Eb5 F#4+G#4 - | G#4+Bb4 \
             Eb4+F5 F#4+A4 Eb4+F5 G#4+Bb4 Eb4+F5 (3 -:1 D5 - (3 - C5+F5+G5 E5+F#5 | (3 - G5 F#5 \
             -:2 (3 -:1 B5 _ (3 G5 C5+F#5+A5 - (3 Bb5 G5 F#5 -:2 (3 B5:1 G#5 G5 -:2 & -:3 F#4:1 \
             -:3 F#4:1 -:3 F#4:1 -:2 A3+B4 | D4+E4 A3+B4 C4+Eb4 A3+B4 D4+E4 A3+B4 D4+E4 - | \
             D4+E4 A3+B4 C4+Eb4 -:1 F#4 -:4 (3 -:1 Bb4 - -:2 | (3 -:1 Bb4 - (3 - G#4+C5 B4+C5 (3 \
             - B4 - (3 - G#4+F5 C5 (3 - B4 - (3 - G#4+C5 B4+C5 (3 - C5 - (3 - Bb4+C#5+E5 C5+Eb5 \
             | (3 - C5 - (3 - Bb4+D5+E5 C#5 (3 - C5 - (3 - Bb4+D5+E5 C#5 (3 - C5 - (3 - \
             Bb4+D5+E5 C#5 (3 - D5 - (3 - E5 Eb5+F5 | (3 - D5 - (3 - C5+E5+F#5 Eb5 (3 - D5 - -:2 \
             (3 -:1 D5 - (3 - C5+E5+F#5 Eb5 (3 F#4+A4+B5 _ - E4:2 | G#4+Bb4 Eb4+F5 F#4+A4 Eb4+F5 \
             G#4+Bb4 Eb4+F5 F#4+A4 E4 & D4+E4:2 A3+B4 D4+E4 A3+B4 D4+E4 A3+B4 C4+Eb4 -:1 F#4 | \
             (3 E5 C#5 C5 -:10 (3 F#5:1 Eb5 D5 -:2 | -:8 (3 G#4+E5:1 A4+C#5 C5 (3 - F#4+Bb4+C5 \
             A4 (3 G#5 F5 E5 (3 Eb5 G#4+D5+E5 C#5+Eb5 | (3 F#5 Eb5 D5 -:2 (3 F#5+G#5:1 G5 _ (3 \
             F#5 - - (3 F#5 Eb5 D5 -:2 (3 F#5+G#5:1 G5 F#5 (3 F5 - - | (3 G#5 F5 E5 -:2 (3 G#5:1 \
             F5 E5 -:2 (3 G#5:1 F5 E5 -:2 (3 Bb5:1 G5 F#5 (3 F#5 _ - | (3 Bb5 G5 F#5 -:2 (3 C6:1 \
             B5 _ (3 G5 C5+F#5+A5 - (3 Bb5 G5 F#5 -:2 (3 -:1 G#5 G5 -:2 | (3 Bb5:1 D5 - (3 - \
             C5+E5+F#5 Eb5 (3 - D5 - -:2 (3 -:1 D5 - (3 - C5+E5+F#5 Eb5 (3 - D5 - (3 - C5+F5+G5 \
             E5+F#5 & -:16 | - | - | - | - | - | -:6 F4+B4+Eb5+B5:1 C6:2. -:3 C5+F#5+A5+C6:1 \
             D5+D6 Eb4+F#5+B5+F6 C6",
        )
        .in_time(COMMON)
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 1113 to 1122.
        Part::new(
            "Chords of F against F sharp",
            "F6:2 F6 F6 F6:1 F6 F6:2 F6 F6 F6:1 F6 | F6:2 F6 F6 F6:1 F6 F6:2 F6 F6 F6:1 F6 | \
             F6:2 F6 F6 F6:1 F6 F6:2 F6 F6 F6:1 F6 | F6:2 F6 F6 F6:1 F6 F6:2 F6 F6 F6:1 F6 | \
             F6:2 F6 F6 F6:1 F6 F6:2 F6 F6 F6:1 F6 | F6:2 F6 F6 F6:1 F6 F6:2 F6 F6 F6:1 F6 | (3 \
             -:2 F6 F6. (3 _ F6 F6 _:1 F6 F6 F6 (3 F6:2 F6 F6. | (3 - F6 F6. (3 _ F6 F6 _:1 F6 \
             F6 F6 (3 F6:2 F6 F6. | (3 - F6 F6. (3 _ F6 F6 _:1 F6 F6 F6 (3 F6:2 F6 F6. | (3 - F6 \
             F6. (3 _ F6 F6 _:1 F6 F6 F6 (3 F6:2 F6 F6",
            "",
        )
        .with_left(
            "-:16 | - | -:15 A1:1 | C2:2. -:14 | -:15 F1:1 | A1:2. -:14 | - (3 -:1 D2 C2 | B1:2. \
             -:14 | -:15 A1:1 | A1 -:15 & -:16 | - | - | - | F4:2 F4 F4 F4:1 F4 F4:2 F4 F4 F4:1 \
             F4 | -:16 | (3 -:2 F4 F4+A4. (3 _ F4+A4 F4 _:1 F4 F4+A4 F4 C5+Eb5:4 | -:12 (3 \
             F4+C5+F5:2 - - | Eb5:8 Eb5:4 C5+Eb5 | -:12 (3 F4+C5+F5:2 - - & F4+C5:2 F4+C5 F4+C5 \
             F4+C5:1 F4+C5 F4+C5:2 F4+C5 F4+C5 F4+C5:1 F4+C5 | F4+C5:2 F4+C5 F4+C5 F4+C5:1 F4+C5 \
             F4+C5:2 F4+C5 F4+C5 F4+C5:1 F4+C5 | F4+C5:2 F4+C5 F4+C5 F4+C5:1 F4+C5 F4+C5:2 F4+C5 \
             F4+C5 F4+C5:1 F4+C5 | F4+C5:2 F4+C5 F4+C5 F4+C5:1 F4+C5 F4+C5:2 F4+C5 F4+C5 F4+C5:1 \
             F4+C5 | C4+C5:8 C4+C5:4 C4+C5 | F4+C5:2 F4+C5 F4+C5 F4+C5:1 F4+C5 F4+C5:2 F4+C5 \
             F4+C5 F4+C5:1 F4+C5 | C5+Eb5:8 C5+Eb5:4 (3 F4+F5:2 F4 F4. | (3 - F4+C5 F4+A4+C5. (3 \
             _ F4+A4+C5 F4+C5 _:1 F4+C5 F4+A4+C5 F4+C5 (3 -:2 F4+C5 F4+C5. | (3 - F4+C5 \
             F4+A4+C5. (3 _ F4+A4+C5 F4+C5 _:1 F4+C5 F4+A4+C5 F4+C5 (3 F4+F5:2 F4 F4. | (3 - \
             F4+C5 F4+A4+C5. (3 _ F4+A4+C5 F4+C5 _:1 F4+C5 F4+A4+C5 F4+C5 (3 -:2 F4+C5 F4+C5 & \
             F5+F#5+A5:1 F5:2. -:1 F5+F#5+A5 F5:2. F5+A5:1 - F5:2. - F5. -:1 | F5+F#5+A5 F5:2. \
             -:1 F5+F#5+A5 F5:2. F5+A5:1 - F5:2. - F5. -:1 | F5+F#5+A5 F5:2. -:1 F5+F#5+A5 F5:2. \
             F5+A5:1 - F5:2. - F5. -:1 | F5+F#5+A5 F5:2. -:1 F5+F#5+A5 F5:2. F5+A5:1 - F5:2. - \
             F5. -:1 | F5+F#5+A5 F5:2. -:1 F5+F#5+A5 F5:2. F5+A5:1 - F5:2. - F5. -:1 | F5+F#5+A5 \
             F5:2. -:1 F5+F#5+A5 F5:2. F5+A5:1 - F5:2. - F5. -:1 | -:16 | - | - | - & -:2 A5+C6 \
             - A5+C6:1 - F#5+A5+C6 - A5:2 F#5+A5+C6:1 - A5 A5+C6 | -:2 A5+C6 - A5+C6:1 - \
             F#5+A5+C6 - A5:2 F#5+A5+C6:1 - A5 A5+C6 | -:2 A5+C6 - A5+C6:1 - F#5+A5+C6 - A5:2 \
             F#5+A5+C6:1 - A5 A5+C6 | -:2 A5+C6 - A5+C6:1 - F#5+A5+C6 - A5:2 F#5+A5+C6:1 - A5 \
             A5+C6 | -:2 A5+C6 - A5+C6:1 - F#5+A5+C6 - A5:2 F#5+A5+C6:1 - A5 A5+C6 | -:2 A5+C6 - \
             A5+C6:1 - F#5+A5+C6 - A5:2 F#5+A5+C6:1 - A5 A5+C6 | (3 -:2 F#5+A5+C6 F#5+A5+C6. (3 \
             _ F#5+A5+C6 F#5+A5+C6 _:1 F#5+A5+C6 F#5+A5+C6 F#5+A5+C6 (3 F#5+A5+C6:2 F#5+A5+C6 \
             F#5+A5+C6. | (3 - F#5+A5+C6 F#5+A5+C6. (3 _ F#5+A5+C6 F#5+A5+C6 _:1 F#5+A5+C6 \
             F#5+A5+C6 F#5+A5+C6 (3 F#5+A5+C6:2 F#5+A5+C6 F#5+A5+C6. | (3 - F#5+A5+C6 F#5+A5+C6. \
             (3 _ F#5+A5+C6 F#5+A5+C6 _:1 F#5+A5+C6 F#5+A5+C6 F#5+A5+C6 (3 F#5+A5+C6:2 F#5+A5+C6 \
             F#5+A5+C6. | (3 - F#5+A5+C6 F#5+A5+C6. (3 _ F#5+A5+C6 F#5+A5+C6 _:1 F#5+A5+C6 \
             F#5+A5+C6 F#5+A5+C6 (3 F#5+A5+C6:2 F#5+A5+C6 F#5+A5+C6",
        )
        .in_time(COMMON)
        .with_tempo(&[tempo(0, 132)])
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1123 to 1125.
        Part::new(
            "Strings",
            "(3 F5:1 _ G5 (3 F5 _ G5 (3 F5 _ G5 (3 F5 _ G5 | (3 F5 _ G5 (3 F5 _ G5 (3 F5 _ G5 (3 \
             F5 _ A5 | G5 A5 G5 A5 (3 G5 _ B5 (3 A5 _ G5",
            "",
        )
        .with_left(
            "-:8 | - | -:6 B3+Eb4+A4:1 - & -:2 Eb4+G4+A4+C5+C#5. Eb4+G4+A4+C5+C#5. \
             Eb4+G4+A4+C5+C#5. | Eb4+G4+A4+C5+C#5. Eb4+G4+A4+C5+C#5. Eb4+G4+A4+C5+C#5. \
             Eb4+G4+A4+C5+C#5. | C#4+F4+B4+D5. C#4+F4+B4+D5. C#4+F4+B4+D5. - & \
             Eb4+A4+C5+C#5+F5:2 -:6 | -:8 | -:6 C#5+E5:1 - & (3 -:1 - G5. (3 _ _ G5. (3 _ _ G5. \
             (3 _ _ G5. | (3 - - G5. (3 _ _ G5. (3 _ _ G5. (3 _ _ A5. | (3 - - A5. (3 _ _ A5. (3 \
             _ _ B5. (3 _ _ -",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 1126 to 1129.
        Part::new(
            "Strings, bar 4",
            "F5:1 A5 G5 A5 G5 A5 G5 A5 G5 A5 G5 A5 | G5 A5 G5 A5 (3 G5 _ A5 (3 G5 _ B5 (3 A5 _ \
             B5 (3 A5 _ B5 | (3 A5 _ C#6 (3 B5 _ A5 (3 G5 _ B5 (3 A5 _ B5 (3 A5 _ C#6 (3 B5 _ \
             C#6 | (3 B5 _ C#6 (3 B5 _ C#6 (3 B5 _ C#6 (3 B5 _ C#6 B5 C#6 B5 -",
            "",
        )
        .with_left(
            "-:2 C#4+F4. -:8 | -:12 | -:2 A3+C#4+G4. - B3+Eb4+A4. - A3+C#4+G4:1 - | -:12 & \
             Eb4+G4+A4+C5+C#5:2. - C#4+F4+B4+D5. C#4+F4+B4+D5. C#4+F4+B4+D5. C#4+F4+B4+D5. | \
             C#4+F4+B4+D5. C#4+F4+B4+D5. C#4+F4+B4+D5. C#4+F4+B4+D5. B3+Eb4+A4+C#5+E5. \
             B3+Eb4+A4+C#5+E5. | B3+Eb4+A4+C#5+E5. - C4+F4+B4+D5. - B3+Eb4+A4+C#5+E5. C4. | \
             A3+C#4+G4+Eb5+F#5. A3+C#4+G4+Eb5+F#5. A3+C#4+G4+Eb5+F#5. A3+C#4+G4+Eb5+F#5. \
             A3+C#4+G4+Eb5+F#5 A3+C#4+G4+Eb5+F#5:1 - & -:2 B4+D5+G5. -:8 | -:12 | -:2 \
             Eb5+F#5+B5. - C#5+E5+A5. - Eb5+F#5:1 - | -:12 & (3 -:1 - A5. (3 _ _ A5. (3 _ _ A5. \
             (3 _ _ A5. (3 _ _ A5. (3 _ _ A5. | (3 - - A5. (3 _ _ A5. (3 _ _ A5. (3 _ _ B5. (3 _ \
             _ B5. (3 _ _ B5. | (3 - - C#6. (3 _ _ A5. (3 _ _ B5. (3 _ _ B5. (3 _ _ C#6. (3 _ _ \
             C#6. | (3 - - C#6. (3 _ _ C#6. (3 _ _ C#6. (3 _ _ - -:4",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Chromatic)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 1130 to 1139.
        Part::new(
            "Trombones",
            "Bb2:2 C3 Eb3 D3 C3 | Bb2 C3 G2 Bb2 D3 | C3 Bb2 G2. -:4 | F5:2 G5 G5 G5 G5 | C6 G5 \
             G5. -:4 | Bb2:2 C3 Eb3 D3 C3 | Bb2 C3 G2 Bb2 D3 | C3 Bb2 D3 C3 G2 | Bb2 D3 C3 G2:1 \
             -:3 | F5:2 G5 G5 G5 G5:1 -",
            "",
        )
        .with_left(
            "G2:2 Bb2 G2 Bb2 G2 | - G2 Bb2 G2 Bb2 | G2 -:8 | F4+C5:2 D4+D5 D4+D5 D4+G4+D5 \
             D4+G4+D5 | Eb4+C5+F5 D4+G#4+D5 D4+G#4+D5. -:4 | G2:2 Bb2 G2 Bb2 G2 | - G2 Bb2 G2 \
             Bb2 | G2 - G2 Bb2 - | - G2 Bb2 -:4 | F4+C5:2 D4+D5 D4+D5 D4+D5 D4+G4+D5:1 - & -:10 \
             | - | - | G#3:2 -:8 | F3+C4+F4+G#5:2 - G3 -:4 | -:10 | - | - | - | G#3:2 -:8 & -:10 \
             | - | - | Eb3+Bb3+C4+Eb4+Bb4:2 Eb3+G#3+Eb4+G4+G#4 Eb3+G#3+Eb4+G4+G#4 \
             Eb3+G#3+Eb4+G#4+C5 Eb3+G#3+Eb4+G#4 | Eb3+Bb3+Bb4 Eb3+G#3+Eb4+G4 Eb3+G#3+Eb4. -:4 | \
             -:10 | - | - | - | Eb3+Bb3+C4+Eb4+Bb4:2 Eb3+G#3+Eb4+G4+G#4 Eb3+G#3+Eb4+G4+G#4 \
             Eb3+G#3+Eb4+G#4+C5 Eb3+G#3+Eb4+G#4:1 - & -:10 | - | - | - | -:4 G4:1 -:5 | -:10 | - \
             | - | - | -",
        )
        .in_time(FIVE_EIGHT_UNGROUPED)
        .with_tempo(&[tempo(0, 144)])
        .in_key(C, Dorian)
        .played_by(&[(gm::TROMBONE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bar 1140.
        Part::new(
            "Trombones, bar 11",
            "C6:2 A5 A5 A5 A5:1 -:3",
            "",
        )
        .with_left(
            "F3+Bb3+F4+Bb4+C5:2 Eb3+Bb3+Eb4+Bb4 Eb3+Bb3+D4+Eb4+Bb4 Eb3+D4+A4+Bb4 \
             Eb3+Bb3+D4+Eb4+Bb4:1 -:3 & G4+G5+Bb5:2 D4+A4+D5:1 - A4+D5:2 D5 A4+D5:1 -:3 & -:8 \
             A5:4",
        )
        .in_time(THREE_FOUR)
        .in_key(C, Dorian)
        .played_by(&[(gm::TROMBONE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1141 to 1146.
        Part::new(
            "Flute and clarinets",
            "F5:2 G5 G5 G5 G5 | C6 A5 A5 A5 A5 | G5 A5 A5 A5 A5 | D6 B5 B5 B5 B5 | D6 E6 E6 E6 \
             E6 | D6 E6 E6 E6 E6:1 -",
            "",
        )
        .with_left(
            "-:10 | - | - | - | - | Bb3+D5:2 Bb3 Bb3+F5:1 - Bb3+D5:2 Bb3+D5:1 - & Bb4+C5+Eb5:2 \
             G#4+D5+Eb5 G#4+D5+Eb5 G#4+D5+Eb5 G#4+D5+Eb5 | Bb4+F5+G5 Bb4+D5+Eb5 Bb4+D5+Eb5 \
             Bb4+D5+Eb5 Bb4+D5+Eb5 | C5+D5+F5 Bb4+E5+F5 Bb4+E5+F5 Bb4+E5+F5 Bb4+E5+F5 | \
             C5+G5+G#5 C5+E5+F5 C5+E5+F5 C5+E5+F5 C5+E5+F5:1 - | C4+G4+G#4+A5:2 C4+F4+B4+C5+B5 \
             C4+F4+B4+C5+B5 C4+F4+B4+C5+B5 C4+F4+B4+C5+B5 | C4+G4+G#4+C5+A5 C4+F4+B4+C5+B5 \
             C4+F4+B4+C5+B5 C4+F4+E5+B5 C4+F4+E5+B5:1 - & -:1 D5 (3 C5 - C5 (3 - C5 - C5 - (3 C5 \
             - C5 | D5 C5 (3 - C5 - C5 - (3 C5 - C5 - C5 | (3 F#5 G5 G5 (3 D5 C5+G5 G5 (3 \
             C5+F#5+G5 D5+G5 C5+F#5 (3 D5+G5 G5 F#5+G5 (3 G5 D5+F#5 C5+G5 | (3 D5+F5+F#5 - \
             D5+F#5 (3 _ - D5+F#5 (3 - D5+F#5 - (3 - D5+F#5+G5 - (3 D5 D5+G5 C5+D5+F5+F#5 | (3 - \
             C5+F5+C6 D5+G5 (3 F5+F#5 D5+G5+C6 F5+F#5 (3 D5+G5+C6 F5+F#5 G5 (3 D5+F5+C6 F#5 - (3 \
             D5+F5 - D5+F5+G5 | -:2 (3 -:1 F5+F#5 - (3 - - D5+G5 -:2 (3 -:1 - C5+F5+C6 & -:10 | \
             - | - | (3 -:1 - F5 (3 - G5 - (3 G5 - G5 (3 - - G5 -:2 | (3 G5+B5:1 - - -:4 (3 -:1 \
             - G5+C6 (3 - G5+C6 - | G5+C6 F5+F#5+B5 (3 D5+G5+C6 - D5+G5+C6 (3 G5 F#5+C6 - (3 \
             F5+C6 G5 F5+G5 (3 F#5+C6 G5 -",
        )
        .in_time(FIVE_EIGHT_UNGROUPED)
        .in_key(C, Major)
        .played_by(&[(gm::FLUTE, 0)]),
        // MIDI bars 1147 to 1158.
        Part::new(
            "Piccolo and harp",
            "-:2 Bb6:4 C7 | Eb7 D7 C7:2 | _ Bb6 C7 G6:4 | Bb6 D7:2 C7 Bb6 | G6 Bb6 C7 G6 Bb6 | \
             D7 C7 Bb6 G6:4 | -:2 Bb6:4 C7 | Eb7 D7 C7:2 | _ Bb6 C7 G6:4 | Bb6 D7:2 C7 Bb6 | G6 \
             Bb6 C7 G6 Bb6 | D7 C7 Bb6 G6:4",
            "",
        )
        .with_left(
            "-:10 | - | - | - | - | Bb4:4 C5:2 - Eb5 | Eb3 Eb3 Eb3:4 Eb3:2 | Eb3+Bb4 Eb3:4 \
             Eb3+F4:2 Eb3+F4 | Eb3+F4:4 Eb3+F4+C5:2 Eb3+F4+Bb4 Eb3+F4 | _ Eb3+F4+C5 Eb3+F4+G4 \
             Eb3+F4:4 | Eb3+F4+C5:2 Eb3+F4+Bb4 Eb3+F4+Ab4:4 Eb3+F4:2 | Eb3+F4 Eb3+F4:4 Eb3 & \
             -:10 | - | - | - | - | - | -:6 C5:4 | - G4 Bb4:2 | _ -:6 G4:2 | Bb4 -:4 Bb4:2 - | \
             -:10 | - & D3+C6:1 Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 | C3+Ab5 G5 D3+C6 \
             Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 | D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 \
             Bb5 | C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 | D3+C6 Bb5 C3+Ab5 G5 D3+C6 \
             Bb5 C3+Ab5 G5 D3+C6 Bb5 | C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 | D3+C6 \
             Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 | C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 \
             Bb5 C3+Ab5 G5 | D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 | C3+Ab5 G5 D3+C6 \
             Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 | D3+C6 Bb5 C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 D3+C6 \
             Bb5 | C3+Ab5 G5 D3+C6 Bb5 C3+Ab5 G5 Bb3:4 & F5+Bb5:1 Eb5 G5 C5+F5 Bb4 Ab4+D5 F5 \
             Eb5+Ab5 D5+G5 F5 | Bb4+Eb5 Ab4 F5+Bb5 - G5 C5+F5 Bb4+Eb5 Ab4+D5 F5 Eb5+Ab5 | D5+G5 \
             C5+F5 Bb4+Eb5 Ab4 F5+Bb5 Eb5 G5 C5+F5 Bb4 Ab4+D5 | F5 Eb5+Ab5 D5+G5 F5 Bb4+Eb5 Ab4 \
             F5+Bb5 - G5 C5+F5 | Bb4+Eb5 Ab4+D5 F5 Eb5+Ab5 D5+G5 C5+F5 Bb4+Eb5 Ab4 F5+Bb5 Eb5 | \
             G5 C5+F5 - Ab4+D5 F5 Eb5+Ab5 D5+G5 F5 Bb4 Ab4 | F5+Bb5 - G5 C5+F5 Bb4+Eb5 Ab4+D5 F5 \
             Eb5+Ab5 D5+G5 F5 | Eb5 Ab4 F5+Bb5 Eb5 G5 C5+F5 Bb4 Ab4+D5 F5 Eb5+Ab5 | D5+G5 F5 \
             Bb4+Eb5 Ab4 F5+Bb5 - G5 C5+F5 Bb4+Eb5 Ab4+D5 | F5 Eb5+Ab5 D5+G5 F5 Bb4+Eb5 Ab4 \
             F5+Bb5 Eb5 G5 C5+F5 | Bb4 Ab4+D5 F5 Eb5+Ab5 D5+G5 F5 Bb4+Eb5 - F5+Bb5 - | G5 C5+F5 \
             Bb4+Eb5 Ab4+D5 F5 Eb5+Ab5 Bb4+Bb5:4 & C5+Ab5:2 D5+Bb5 Eb5+Ab5 D5+Bb5 C5+Ab5 | \
             D5+Bb5 Eb5+Ab5 D5+Bb5 C5+Ab5 D5+Bb5 | Eb5+Ab5 D5+Bb5 C5+Ab5 D5+Bb5 Eb5+Ab5 | D5+Bb5 \
             C5+Ab5 D5+Bb5 Eb5+Ab5 D5+Bb5 | C5+Ab5 D5+Bb5 Eb5+Ab5 D5+Bb5 C5+Ab5 | D5+Bb5 Eb5+Ab5 \
             D5+Bb5 C5+Ab5 D5+Bb5 | Eb5+Ab5 D5+Bb5 C5+Ab5 D5+Bb5 Eb5+Ab5 | D5+Bb5 C5+Ab5 D5+Bb5 \
             Eb5+Ab5 D5+Bb5 | C5+Ab5 D5+Bb5 Eb5+Ab5 D5+Bb5 C5+Ab5 | D5+Bb5 Eb5+Ab5 D5+Bb5 C5+Ab5 \
             D5+Bb5 | Eb5+Ab5 D5+Bb5 C5+Ab5 D5+Bb5 Eb5+Ab5 | D5+Bb5 C5+Ab5 D5+Bb5 Eb5+Eb6:4",
        )
        .in_time(FIVE_EIGHT_UNGROUPED)
        .in_key(C, NaturalMinor)
        .played_by(&[(gm::PICCOLO, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 1159 to 1161.
        Part::new(
            "Oboes in A flat",
            "Db6:4 Ab5:2 Ab5 | Ab5:4 Bb4 | Bb4 Eb5",
            "",
        )
        .with_left(
            "Ab2+Eb3:4 Ab2:2 Ab2 | -:8 | - & Ab1+Bb3:4 Ab1+C4:2 Ab1+C4 | Ab1+C4:4 - | -:8 & \
             Ab5:4 Eb5:2 Eb5 | Eb5:4 Ab2+Eb3+Db4:2 Ab2+Eb3+Db4 | Ab2+Eb3+Bb3:4 Ab2+Eb3+Bb3:2 \
             Ab2+Eb3+Bb3:1 - & Db4+Ab4+Db5:4 G3+Ab5:2 (3 _:1 - Bb5 | G3+Ab4:4 - | -:8",
        )
        .in_key(GSharp, Major)
        .played_by(&[(gm::OBOE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bars 1162 to 1168.
        Part::new(
            "Flutes in A flat",
            "-:3 F6:1 Eb6:3 Eb6:1 | Db6:3 Bb5:1 Ab5:3 F6:1 | Eb6:3 Eb6:1 Db6:3 Bb5:1 | Ab5:3 \
             F6:1 Eb6:3 Eb6:1 | Db6:3 Bb5:1 Ab5:2 Ab5 | Ab5:3 C6:1 Bb5:3 F6:1 | Eb6:3 Eb6:1 \
             Db6:3 Bb5:1",
            "",
        )
        .with_left(
            "-:8 | - | - | - | -:4 Ab2+G3+C4 | Ab2+G3+C4+Eb5 Ab2+F3+Db4+F5 | Ab2+Eb3+Bb3+Eb5 \
             Ab2+F3+Bb3+Ab5 & Ab2+Eb3+Bb3:4 Ab2+Eb3+C4+Eb5 | Ab2+F3+Db4+Ab5 Ab2+G3+C4+Eb5 | \
             Ab2+Eb3+C4+Bb5 Ab2+F3+Db4+Db5 | Ab2+G3+C4+Eb5 Ab2+Eb3+C4+Bb5 | Ab2+F3+Db4+Db5 Db5:2 \
             Db5 | -:8 | - & Db5:4 C5:2 Bb4 | Ab4:4 C5 | Bb4 Bb4 | C5 Bb4:2 Ab4 | Bb4:4 Ab4 | C5 \
             Bb4:2 Ab4 | Bb4:4 Bb4 & -:4 Bb4:2 - | -:4 Ab4+Ab5 | -:8 | Ab4+Ab5:4 - | - Eb5+Ab5 | \
             Ab4+Ab5 - | - Ab4+Db5+Db6 & -:4 Bb5+Eb6 | Db5+Db6 - | Eb5+Eb6 Ab4+Ab5+Db6 | - \
             Eb5+Eb6 | Ab5+Db6 - | - Bb5 | Bb5+Eb6 -",
        )
        .in_key(GSharp, Major)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::MezzoPiano),
        // MIDI bars 1169 to 1173.
        Part::new(
            "Violins in A flat",
            "-:3 F6:1 Eb6:3 Eb6:1 | Db6:3 Bb5:1 Ab5:3 F6:1 | Eb6:3 Eb6:1 Db6:3 Bb5:1 | Ab5:3 \
             F6:1 Eb6:3 Eb6:1 | Db6:3 Bb5:1 Ab5:4",
            "",
        )
        .with_left(
            "Ab2+Ab4:4 Ab2+Eb5+Bb5 | Ab2+Eb3+Bb3+Db5+Ab5 Ab2+G3+C4+Ab4+Eb5 | Ab2+Eb5+Bb5 \
             Ab2+Eb3+Bb3+Db5+Ab5 | Ab2+F3+C4+Ab4+Eb5 Ab2+Eb5+Bb5 | Bb2+F3+Db5+Ab5 \
             C3+G3+C4+Ab4+Eb5 & F3+C4+Eb5+Ab5:4 - | -:8 | - | - | -:7 F#5:1 & -:3 F6:2. - Eb6:1. \
             | -:3 Bb5:2. - F6:1. | -:3 Eb6:2. - Bb5:1. | -:3 F6:2. - Eb6:1. | -:8",
        )
        .in_key(GSharp, Major)
        .played_by(&[(gm::VIOLIN, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1174 to 1179.
        Part::new(
            "Clarinet and horns",
            "G5:3 E5:1 F5:2 - | B4. -:1 F#5 G5:2 - | E5. - B4. -:1 F#5 | G5:3 E5:1 F5:2 - | A4. \
             -:1 F#5 G5:3 E5:1 | F5:3 B4:1 C5:4",
            "",
        )
        .with_left(
            "G3+A3+C4:2 - A3+Bb3+D4 - | B3+E4 - B3+C4 - | C4+D4+Eb4 - C#4+Eb4+E4 - | E2+G#3+D4 - \
             F2+A3+Eb4 - | F#2+Bb3+E4 - E2+G#3+B3+D4:4 | A3+Eb4 Bb3+E4 & C3+G5:4 D3+F5 | E3+C5 \
             E3+G5 | F3+F5 F#3 | E3+G5 F3+F5 | F#3 E3+G5 | F3+C4+F5 F#3+C#4 & -:8 | - | - | - | \
             -:4 G4 | F4 G4:2 - & G4+D5:2 - F4+C5 - | C4+G4 - D4+G4+D5 - | F4+C5 - C4+G4+C5 - | \
             G4+D5 - F4+C5 - | C4+G4+C5 - D5 - | C5 - C4:4",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::CLARINET, 0)])
        .marked(Dynamic::Piano),
        // MIDI bars 1180 to 1185.
        Part::new(
            "Chords over a rising bass",
            "G6:1 -:3 F6:1 -:3 | C6:1 -:3 G6:1 -:3 | F6:1 -:3 C6:1 -:3 | G6:1 -:3 F6:1 -:3 | \
             C6:1 -:3 G6:1 -:3 | F6:1 -:3 C6:1 -:3",
            "",
        )
        .with_left(
            "G4+B4+G5:1 -:3 Bb3+E4+F4+A4+F5:1 -:3 | A1+C4+C5:1 -:3 Bb1+G3+B3+G4+B4:1 -:3 | \
             B1+Bb3+F4:1 -:3 C2+C5:1 -:3 | C#2+G3+D4+G4+G5:1 -:3 D2+Bb3+E4+F4+F5:1 -:3 | \
             Eb2+C4+E4+C5:1 -:3 E2+G3+B3+G4:1 -:3 | F2+Bb3+E4:1 -:3 F#2+C4+G4+C5:1 -:3 & \
             G1+G3+B3+D4+F4:1 -:3 G#1+G#3+C4+Eb4+F#4:1 -:3 | A3+C#4+E4+G4:1 -:3 Bb3+D4+F4+G#4:1 \
             -:3 | B3+Eb4+E4+F#4+A4:1 -:3 C4+E4+G4+Bb4:1 -:3 | C#4+F4+G#4+B4:1 -:3 D4+F#4+A4:1 \
             -:3 | Eb4+G4+Bb4+C#5:1 -:3 E4+G#4+B4+D5:1 -:3 | F4+A4+C5+Eb5:1 -:3 F#4+Bb4+C#5:1 \
             -:3 & G2+F3+D5+B5+D6:1 -:3 G#2+F#3+C5+A5+C6:1 -:3 | A2+G3+E5+G5:1 -:3 \
             Bb2+G#3+G5+B5+D6:1 -:3 | B2+F5+A5+C6:1 -:3 C3+Bb3+E5+G5:1 -:3 | C#3+B3+D5+B5+D6:1 \
             -:3 D3+C4+C5+A5+C6:1 -:3 | Eb3+C#4+E5+G5:1 -:3 E3+D4+G5+B5+D6:1 -:3 | \
             F3+Eb4+F5+A5+C6:1 -:3 F#3+E4+E5+G5:1 -:3",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Fortissimo),
        // MIDI bars 1186 to 1191.
        Part::new(
            "Tremolo",
            "F#6:1 E6 F#6 E6 F#6 E6 F#6 E6 | F#6 E6 F#6 E6 F#6 E6 F#6 E6 | F#6 E6 F#6 E6 F#6 E6 \
             F#6 E6 | F#6 E6 F#6 E6 F#6 E6 F#6 E6 | F#6 E6 F#6 E6 F#6 E6 F#6 E6 | F#6 E6 F#6 E6 \
             F#6 E6 F#6 E6",
            "",
        )
        .with_left(
            "-:1 G6 - G6 -:4 | Bb5:1 - Bb5 - Bb5 - Bb5 - | Bb5 - Bb5 - Bb5 - Bb5 - | Bb5 - Bb5 - \
             Bb5 - Bb5 - | Bb5 - Bb5 - Bb5 - Bb5 - | Bb5 - Bb5 - Bb5 - Bb5 - & -:4 G6:1 G6 G6 G6 \
             | C#6+G6 C6+G6 C#6+G6 C6+G6 C#6+G6 C6+G6 C#6+G6 C6+G6 | C#6+G6 C6+G6 C#6+G6 C6+G6 \
             C#6+G6 C6+G6 C#6+G6 C6+G6 | E5+C#6+G6 C#5+C6+G6 E5+C#6+G6 C#5+C6+G6 E5+C#6+G6 \
             C#5+C6+G6 E5+C#6+G6 C#5+C6+G6 | E5+C#6+G6 C#5+C6+G6 E5+C#6+G6 C#5+C6+G6 E5+C#6+G6 \
             C#5+C6+G6 E5+C#6+G6 C#5+C6+G6 | Bb4+E5+C#6+G6 C5+C#5+C6+G6 Bb4+E5+C#6+G6 \
             C5+C#5+C6+G6 Bb4+E5+C#6+G6 C5+C#5+C6+G6 Bb4+E5+C#6+G6 C5+C#5+C6+G6",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Pianissimo),
        // MIDI bars 1192 to 1193.
        Part::new(
            "Crescendo",
            "F#6:1 E6 F#6 E6 F#6 E6 F#6 E6 | F#6 E6 F#6 E6 F#6 E6 F#6 E6",
            "",
        )
        .with_left(
            "-:4 C#4:1 C#4 C#4 C#4 | C#4 C#4 C#4 C#4 C#4 C#4 C#4 C#4 & F#4+Bb4+C#5+F#5:1 G4+G5 \
             F#4+Bb4+C#5+F#5 G4+G5 F#4+Bb4+C#5+F#5 G4+G5 F#4+Bb4+C#5+F#5 G4+G5 | F#4+Bb4 E5+G5 \
             F#4+Bb4 E5+G5 F#4+Bb4 E5+G5 F#4+Bb4 G4+C5+E5+G5 & C5+E5:8 | G4+C5+C#5+F#5:7 -:1 & \
             C#6+G6:1 C6+G6 C#6+G6 C6+G6 C#6+G6 C6+G6 C#6+G6 C6+G6 | C#6+G6 C6+G6 C#6+G6 C6+G6 \
             C#6+G6 C6+G6 C#6+G6 C6+G6",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .marked(Dynamic::Forte),
        // MIDI bar 1194.
        Part::new(
            "The last chord",
            "-:2 (3 C4:1 D4 E4 F#4 -:3",
            "",
        )
        .with_left(
            "-:2 (3 C2:1 B1 A1+C3 G1 -:3 & -:3 F#2:2 -:3 & -:2 Bb2+E3:1 A2+Eb3 F#1+G2+C#3+C4+F#4 \
             -:3 & C#4:1 C#4 C#4 C#4 -:4 & G6:1 G6 G6 E2+E3+G6 F2+D3+F3 -:3",
        )
        .in_key(C, Chromatic)
        .played_by(&[(gm::FLUTE, 0)])
        .with_drums(&[
            (Drum::Snare, "-:3 x:1 X:4"),
        ])
        .marked(Dynamic::Fortissimo),
    ],
    form: &["Drum roll", "Evening", "Flutes and piccolo", "Oboe", "Trumpets", "The whole orchestra", "Oboes and horns", "Oboes and horns, bar 3", "Wet-nurses' Dance", "The wet-nurses' tune", "Horn", "Piccolo, horn and violins", "Flutes in fourths", "Flutes and harp", "Oboes", "Flutes and piano", "Trumpet tune", "Tutti in G", "Trumpets in C", "The wet-nurses' tune, tutti", "A peasant with a bear", "Clarinet and tuba", "Woodwind trills", "A merchant", "The merchant's tune", "Two gypsy girls", "Clarinets and horns", "Gypsy girls again", "Celesta and violins", "Oboes in thirds", "The merchant again", "Gypsy girls, last time", "Trumpet and harp", "Coachmen's Dance", "Trumpet, violins, trombone", "Woodwinds", "Flutes and clarinets", "Flutes and clarinets, bar 12", "Clarinet and trombones", "Woodwinds in C", "The coachmen, tutti", "The coachmen dance on", "The masqueraders", "Clarinet, chromatic", "Chords of F against F sharp", "Strings", "Strings, bar 4", "Trombones", "Trombones, bar 11", "Flute and clarinets", "Piccolo and harp", "Oboes in A flat", "Flutes in A flat", "Violins in A flat", "Clarinet and horns", "Chords over a rising bass", "Tremolo", "Crescendo", "The last chord"],
    drums_from: 0,
    about: "The fourth tableau of Petrushka (1911, revised 1947): the Shrovetide fair again, \
            towards evening. A side drum roll links it to the third tableau. The crowd's music \
            comes back in D major, in 6/8: the oboes, the flutes and piccolo over an ostinato of \
            the clarinets and bassoon, the trumpets, and the whole orchestra. The wet-nurses \
            dance to a folk tune in F major (C, A, B flat, A, G, F), on the oboe, then the horn, \
            then the piccolo, horn and violins together; after it come flutes in fourths in D \
            minor, a figure over the harp in D Mixolydian (D major with C natural), a tune in G \
            major and G Mixolydian (with F natural) on the flutes and the trumpet, and, over a \
            pedal on C, the trumpets' figure and the wet-nurses' tune once more, fortissimo, in \
            C Mixolydian (with B flat). A peasant leads in a bear: high on the clarinet he plays \
            his pipe, and the tuba, the bear, lumbers in 3/2 between B flat, F sharp, C and E, \
            notes of a whole-tone scale, which keeps to no major or minor key. A merchant \
            arrives with two gypsy girls: his tune keeps to the white notes and turns around D \
            (D Dorian, the minor mode with a raised sixth, B natural); the girls dance over \
            chords of D minor, and a trumpet and the harp take up their figure in F major. Then \
            the coachmen and grooms stamp in, a heavy dance in E Mixolydian (E major with D \
            natural) over a pedal on E, with a clarinet tune in G Mixolydian and an episode of \
            the woodwinds in C major. Masqueraders burst in: a figure of the piano and harp in D \
            major, a chromatic clarinet, chords of F major hammered against F sharp, A and C, \
            the strings in the whole-tone scale, the trombones and tuba in bars of 5/8 in C \
            Dorian (the minor mode with A natural), the piccolo over the harp in C minor, and a \
            tune in A flat major over a pedal on A flat. The MIDI ends here, without Petrushka's \
            death: chords over a bass rising by half steps, then a tremolo that starts \
            pianissimo in the flutes and violins and grows to the whole orchestra, the piano \
            alternating the chords of F sharp major and C major. This is Petrushka's chord from \
            the second tableau: two major triads a tritone apart that share no note, whose six \
            notes all lie in the octatonic scale (half step and whole step in turn), which no \
            major or minor key holds. A last fortissimo run, C, D, E, F sharp, lands on F sharp \
            and C, the tritone again. The bars that keep to no scale, or sound two keys at once \
            like these, are marked chromatic.",
};
