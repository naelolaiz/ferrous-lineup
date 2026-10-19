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
// put together from the same bars instead of being written out again.

/// Two bars of the snare drum's ostinato, the rhythm under the whole
/// piece: an eighth, a triplet of sixteenths, an eighth, a triplet, then
/// two eighths; the second bar ends with two more triplets instead.
macro_rules! bolero_snare {
    () => {
        "x:2 (3 x:1 x x x:2 (3 x:1 x x x:2 x:2 | \
         x:2 (3 x:1 x x x:2 (3 x:1 x x (3 x:1 x x (3 x:1 x x"
    };
}

/// Two bars of the violas' and cellos' pizzicato, together: C and G with
/// G and its octave above, then the cellos' low G and its octave, as two
/// eighths in the second bar.
macro_rules! bolero_pizzicato {
    () => {
        "C3+G3:4 G3+G4:4 G2+G3:4 | C3+G3:4 G3+G4:4 G2+G3:2 G2+G3:2"
    };
}

/// From rehearsal 1 on, the violas add a C on every third beat.
macro_rules! bolero_violas_c {
    () => {
        "-:8 C3:4 | -:8 C3:4"
    };
}

/// The second flute doubling the snare drum's two bars on G, from the
/// flute's last note (an eighth rest instead of the first stroke).
macro_rules! bolero_flute_rhythm_in {
    () => {
        "-:2 (3 G4:1 G4 G4 G4:2 (3 G4:1 G4 G4 G4:2 G4:2 | \
         G4:2 (3 G4:1 G4 G4 G4:2 (3 G4:1 G4 G4 (3 G4:1 G4 G4 (3 G4:1 G4 G4"
    };
}

/// The same two bars of the second flute, complete.
macro_rules! bolero_flute_rhythm {
    () => {
        "G4:2 (3 G4:1 G4 G4 G4:2 (3 G4:1 G4 G4 G4:2 G4:2 | \
         G4:2 (3 G4:1 G4 G4 G4:2 (3 G4:1 G4 G4 (3 G4:1 G4 G4 (3 G4:1 G4 G4"
    };
}

/// Two bars of silence.
macro_rules! bolero_rest {
    () => {
        "-:12 | -:12"
    };
}

/// Eight times the two bars `$bars` names.
macro_rules! bolero_eight {
    ($bars:ident) => {
        concat!(
            $bars!(),
            " | ",
            $bars!(),
            " | ",
            $bars!(),
            " | ",
            $bars!(),
            " | ",
            $bars!(),
            " | ",
            $bars!(),
            " | ",
            $bars!(),
            " | ",
            $bars!()
        )
    };
}

/// A whole statement of the tune: eighteen bars, nine times two.
macro_rules! bolero_nine {
    ($bars:ident) => {
        concat!(bolero_eight!($bars), " | ", $bars!())
    };
}

/// The first tune, A, in C major, at the flute's pitch: sixteen bars, its
/// last note C held into the seventeenth, and a bar of the accompaniment
/// alone before the next instrument takes it.
const BOLERO_A: &str = "C5:4 _:2 B4:1 C5 D5 C5 B4 A4 | C5:2 C5:1 A4 C5:4 _:2 B4:1 C5 | \
     A4:1 G4 E4 F4 G4:8 | _:1 F4:1 E4 D4 E4 F4 G4 A4 G4:4 | \
     _:4 _:1 A4:1 B4 A4 G4 F4 E4 D4 | E4:1 D4 C4:2 _:2 C4:1 D4 E4:2 F4:2 | D4:4 G4:8 | \
     _:8 _:2 -:2 | D5:4 _:3 C5:1 B4 A4 B4 C5 | D5:1 C5 B4:2 _:1 C5:1 B4 A4 C5 B4 A4 F4 | \
     _:2 F4:1 F4 F4:2 A4:2 C5:1 A4 B4 G4 | F4:2 F4:1 F4 F4:2 A4:2 B4:1 G4 A4 F4 | \
     D4:2 D4:1 C4 D4:4 _:2 D4:1 D4 | D4:2 F4:2 A4:1 F4 G4 E4 D4:2 D4:1 C4 | \
     D4:4 _:2 D4:1 C4 D4:2 E4:1 F4 | G4:8 _:1 F4:1 E4 D4 | C4:6 -:2 -:4 | -:12";

/// C major under every bar of a statement.
const BOLERO_C: &str = "C | C | C | C | C | C | C | C | C | C | C | C | C | C | C | C | C | C";

// Source: Ravel's autograph full score (pencil, 1928: 43 scanned pages, the music on pages 7
// to 37), the authority here for the bars, instruments, octaves, dynamics and
// accompaniment, and for the tune's notes the Wikifonia lead sheet
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/RavelBolero.abc
// checked bar by bar against the manuscript. A FIRST VERSION: bars 1 to 38 of the 340 the
// manuscript writes (the introduction, the flute's statement and the clarinet's); the rest
// follows. Bars 1 and 2 are marked "bis" in the manuscript, played twice: the introduction
// is four bars, as in the printed score, where the flute comes in at bar 5 (bar 3 of the
// manuscript). The tune agrees with the lead sheet in every bar but its last note, which
// the manuscript holds for a quarter tied to an eighth (the lead sheet has an eighth). The
// clarinet in B flat is written a tone higher and plays the tune at the flute's pitch.
// Under it the violas and cellos pluck their figure, written here as one line (the cellos'
// C on the first beat is the violas' lower note), with the violas' C on the third beat,
// added from rehearsal 1 (bar 19), as a second voice; the second flute doubles the snare
// drum's rhythm on G in bars 19 to 22, written with the strings on the Keys channel. Left
// out: the harp, which comes in at rehearsal 2 (bar 37) with chords in harmonics on the
// second and third beats, not yet read with certainty. The manuscript marks the violas and
// cellos p in bar 1, and the snare drum p (or pp: the mark is faint); the flute and the
// clarinet come in with no mark of their own, so all three parts are marked p (the printed
// score has pp). The tempo, ♩ = 72, "Tempo di Bolero, moderato assai", is the printed
// score's: no tempo mark could be read on the manuscript's first page of music.
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
        // Two bars, played twice ("bis"): the snare drum and the pizzicato alone.
        Part::new("Introduction", "-:12 | -:12", "C | C")
            .with_left(bolero_pizzicato!())
            .with_drums(&[(Drum::Snare, bolero_snare!())])
            .marked(Dynamic::Piano),
        Part::new("Flute", BOLERO_A, BOLERO_C)
            .with_left(concat!(
                bolero_nine!(bolero_pizzicato),
                " & ",
                bolero_eight!(bolero_rest),
                " | ",
                bolero_violas_c!(),
                " & ",
                bolero_eight!(bolero_rest),
                " | ",
                bolero_flute_rhythm_in!()
            ))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::FLUTE, 0)])
            .marked(Dynamic::Piano),
        Part::new("Clarinet", BOLERO_A, BOLERO_C)
            .with_left(concat!(
                bolero_nine!(bolero_pizzicato),
                " & ",
                bolero_nine!(bolero_violas_c),
                " & ",
                bolero_flute_rhythm!(),
                " | ",
                bolero_eight!(bolero_rest)
            ))
            .with_drums(&[(Drum::Snare, bolero_nine!(bolero_snare))])
            .played_by(&[(gm::CLARINET, 0)])
            .marked(Dynamic::Piano),
    ],
    form: &["Introduction", "Introduction", "Flute", "Clarinet"],
    drums_from: 0,
    about: "A first version: bars 1 to 38 of 340; the rest follows. A Spanish dance, the \
            bolero, slowed down and turned into one long crescendo. The snare drum plays the \
            same two bars from the first bar to the last, an eighth note and a quick triplet \
            of sixteenths, over and over, while the violas and cellos pluck C and G, the \
            tonic and the dominant of C major. Count it in three: the triplets fill the \
            second half of beats 1 and 2, and beat 3 is two plain eighths. \
            Over it a single tune, in two halves, passes from one instrument to the next, \
            each time a little louder and fuller: here the flute plays it first, softly, low \
            in its range, then the clarinet. The first half stays in C major, rising from C \
            and falling back by step; it holds its long notes across the bar lines and lands \
            late, after the beat, with little rushing runs of sixteenths, so the tune seems \
            to sway over the strict drum. It ends on the low C, and a bar of the drum alone \
            hands it on.",
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
