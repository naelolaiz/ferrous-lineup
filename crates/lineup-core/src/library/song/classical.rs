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
// (Mutopia 242, from the Bach-Gesellschaft edition: violin I and continuo)
const AIR_ENDING: &str = "C#5:2 B4 B4:1 C#5 D5:6 C#5:2 B4 | A4:16";
const AIR_B_END: &str = "D5:12 F#5:2 E5 | E5:12 G5:2 F#5 | F#5:12 A5:2 G5 | G5:16 | A4:10 C#5:2 E5 G5 | \
             G5:2 E5 F#5:10 G5:1 A5 | D5:10 F#5:2 A5 C6 | B5:12 D5:4 | C#5:2 E5 G5:8 B4:4 | \
             A4:4 E5:2 F#5:1 G5:3 F#5:4 E5:2 | D5:1 C#5 B4:4 C#5:2 D5:4 C#5:2 D5 | D5:16";
const AIR_B_END_LEFT: &str = "F#2:4 F#3 G3 G2 | G#2:4 G#3 A3 A2 | A#2:4 A#3 B3 B2 | E3:4 E4 D4 D3 | \
             C#3:4 C#4 A3 C#4 | D4:4 D3 C3 C4 | B3:4 B2 A2 A3 | G3:4 G2 F#2 F#3 | \
             E3:4 E2 D2 D3 | C#3:4 A2 D3 G3 | A3:4 G3 A3 A2 | D2:16";

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
    bass: Bass::Held,
    rhythm: "",
    band: Band::new(gm::VIOLIN, gm::CELLO, gm::CONTRABASS),
    style: "Hymn",
    parts: &[
        Part::new(
            "A",
            "F#5:16 | _:16 | _:4 B5:2 G5 E5 D5 C#5 D5 | C#5:8 A4 | A5:16 | \
             _:2 F#5 C5 B4 E5 D#5 A5 G5 | G5:16 | _:2 E5 B4 A4 D5 C#5 G5 F#5 | \
             F#5:12 G#5:2 A5 | D5:4 D5:1 E5 F#5:4 E5:2 E5 D5",
            "",
        )
        .with_left(
            "D3:4 D4 C#4 C#3 | B2:4 B3 A3 A2 | G2:4 G3 G#3 G#2 | A2:4 A3 G3 G2 | \
             F#2:4 F#3 E3 E2 | D#2:4 D#3 B2 B3 | E2:4 E3 D3 D2 | C#2:4 C#3 A2 A3 | \
             D3:4 D4 C#4 C#3 | B2:4 B3 G#3 E3",
        ),
        // The A part ends in A major, the dominant: G sharp in bar 9, and
        // a cadence in A.
        Part::new("First ending", AIR_ENDING, "")
            .with_left("A3:4 D3 E3 E2 | A2:2 B2 C#3 D3 E3 G3 F#3 E3")
            .in_key(A, Major),
        Part::new("Second ending", AIR_ENDING, "")
            .with_left("A3:4 D3 E3 E2 | A2:16")
            .in_key(A, Major),
        // B goes through three keys: E minor (C and D sharp), B minor
        // (A sharp, a cadence in bar 8) and A major (G sharp, a cadence in
        // bar 12), before "B end" turns back to D.
        Part::new(
            "B in E minor",
            "C#5:10 D5:1 C#5 B4 C#5 A4:2 | A5:12 C5:4 | B4:4 B5:6 A5:2 G5 F#5 | \
             G5:9 F#5:1 E5 D5 C#5:2 B4",
            "",
        )
        .with_left("A2:4 A3 G3 G2 | F#2:4 F#3 E3 E2 | D#2:4 D#3 F#3 B2 | E3:4 E4 D4 D3")
        .in_key(E, HarmonicMinor),
        Part::new(
            "B in B minor",
            "A#4:2 B4 C#5:6 D5:2 E5:4 | _:2 F#5 G5:8 F#5:4 | \
             E5:2 D5 C#5 B4 C#5 D5:1 E5 D5:4 | B4:16",
            "",
        )
        .with_left("C#3:4 C#4 B3 B2 | A#2:4 B2 C#3 A#2 | B2:4 G3 E3 F#3 | B2:4 B3 A3 A2")
        .in_key(B, HarmonicMinor),
        Part::new(
            "B in A major",
            "D5:10 F#5:2 E5 D5 | B5:12 A5:2 G#5 | E5:2 A5 A4:4 B4:6 C#5:1 D5 | C#5:6 B4:2 A4:8",
            "",
        )
        .with_left("G#2:4 G#3 F#3 F#2 | E2:4 E3 D3 D2 | C#2:4 C#3 D3 E3 | A2:4 A3 G3 G2")
        .in_key(A, Major),
        Part::new("B end", AIR_B_END, "").with_left(AIR_B_END_LEFT),
        // The second time round is the end of the piece: the last note
        // has the Air's only fermata.
        Part::new("Last end", AIR_B_END, "")
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
            the continuo's bass in steady octave leaps, here on the Keys channel; the inner \
            parts are left out. The bass walks down the D major scale under the first held \
            F sharp, as in Pachelbel's Canon. The first half ends in A major; the second \
            passes through E minor, B minor and A major before it finds D again. Each bar of Bach's is written as two bars \
            with the note values doubled, so his thirty-second notes fit in sixteenths.",
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
