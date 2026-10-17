//! Hip hop and lo-fi: old melodies over slow beats.

use super::*;

pub(super) const TWINKLE: Song = Song {
    name: "Twinkle, Twinkle, Little Star",
    origin: "French folk song, 1761",
    feel: "Lo-fi",
    genre: Genre::HipHop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Lazy,
    bass: Bass::Lazy,
    rhythm: "Lo-fi",
    band: LO_FI,
    style: "Lo-fi",
    parts: &[Part::new(
        "Tune",
        "C5:4 C5 G5 G5 | A5 A5 G5:8 | F5:4 F5 E5 E5 | D5 D5 C5:8 | \
         G5:4 G5 F5 F5 | E5 E5 D5:8 | G5:4 G5 F5 F5 | E5 E5 D5:8 | \
         C5:4 C5 G5 G5 | A5 A5 G5:8 | F5:4 F5 E5 E5 | D5 D5 C5:8",
        "Imaj7 | IVmaj7 Imaj7 | IVmaj7 Imaj7 | V7 Imaj7 | Imaj7 IVmaj7 | Imaj7 V7 | \
         Imaj7 IVmaj7 | Imaj7 V7 | Imaj7 | IVmaj7 Imaj7 | IVmaj7 Imaj7 | V7 Imaj7",
    )],
    form: &["Tune", "Tune"],
    drums_from: 0,
    about: "The French tune \"Ah ! vous dirai-je, maman\", which Mozart wrote variations on. \
            Major seventh chords turn a nursery rhyme into lo-fi.",
};

pub(super) const GYMNOPEDIE: Song = Song {
    name: "Gymnopédie No. 1",
    origin: "Erik Satie, 1888",
    feel: "Lo-fi waltz",
    genre: Genre::HipHop,
    tonic: D,
    mode: Major,
    time: THREE_FOUR,
    tempo: 72.0,
    comping: Comping::Sway,
    bass: Bass::Waltz,
    rhythm: "Jazz waltz",
    band: Band::new(
        gm::ACOUSTIC_GRAND_PIANO,
        gm::ELECTRIC_PIANO,
        gm::FINGER_BASS,
    ),
    style: "Jazz waltz",
    parts: &[
        Part::new(
            "Intro",
            "-:12 | -:12 | -:12 | -:12",
            "IVmaj7 | Imaj7 | IVmaj7 | Imaj7",
        ),
        Part::new(
            "Melody",
            "-:4 F#5:4 A5 | G5 F#5 C#5 | B4 C#5 D5 | A4:12 | F#4:12 | _:12 | _:12 | _:12",
            "IVmaj7 | Imaj7 | IVmaj7 | Imaj7 | IVmaj7 | Imaj7 | IVmaj7 | Imaj7",
        ),
    ],
    form: &["Intro", "Melody", "Melody"],
    drums_from: 1,
    about: "Satie's slow dance rocks between two major seventh chords, Gmaj7 and Dmaj7, for \
            the whole piece.",
};

pub(super) const PRELUDE_IN_C: Song = Song {
    name: "Prelude in C",
    origin: "Johann Sebastian Bach, The Well-Tempered Clavier, 1722",
    feel: "Boom bap",
    genre: Genre::HipHop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 86.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "Boom bap",
    band: BOOM_BAP,
    style: "Jazz rap",
    parts: &[
        Part::new(
            "In C",
            "C4:2 E4 G4 C5 E5 G4 C5 E5 | C4 D4 A4 D5 F5 A4 D5 F5 | B3 D4 G4 D5 F5 G4 D5 F5 | \
             C4 E4 G4 C5 E5 G4 C5 E5",
            "I | ii7 | V7 | I",
        ),
        // Bars 5-11 move to G major, the dominant, with a cadence on G in
        // bar 11; bar 12 starts the way back.
        Part::new(
            "To G",
            "C4:2 E4 A4 E5 A5 A4 E5 A5 | C4 D4 F#4 A4 D5 F#4 A4 D5 | \
             B3 D4 G4 D5 G5 G4 D5 G5 | B3 C4 E4 G4 C5 E4 G4 C5 | A3 C4 E4 G4 C5 E4 G4 C5 | \
             D3 A3 D4 F#4 C5 D4 F#4 C5 | G3 B3 D4 G4 B4 D4 G4 B4 | G3 Bb3 E4 G4 C#5 E4 G4 C#5",
            "ii | V7 | I | IVmaj7 | ii7 | V7 | I | #iv°7",
        )
        .in_key(G, Major),
    ],
    // The demo stops after bar 12, where Bach heads for D minor; it ends
    // on the opening bars instead, back home in C.
    form: &["In C", "To G", "In C", "To G", "In C"],
    drums_from: 0,
    about: "Bach's prelude has no tune at all: each bar is one chord, broken into the same \
            rising figure. That makes it a perfect sample for a hip hop beat. Listen to how \
            each chord changes as few notes as it can. From bar 5 the chords lead to G \
            major, the dominant, which bar 11 reaches with D7 to G: a modulation, so the key \
            changes there. Bar 12 is the diminished seventh on C sharp, over a G in the \
            bass, that Bach uses to slip towards D minor on the way home.",
};

pub(super) const SAKURA: Song = Song {
    name: "Sakura Sakura",
    origin: "Japanese folk song, Edo period",
    feel: "R&B",
    genre: Genre::HipHop,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Lazy,
    bass: Bass::Lazy,
    rhythm: "R&B",
    band: Band::new(gm::KOTO, gm::ELECTRIC_PIANO, gm::FINGER_BASS),
    style: "R&B minor",
    parts: &[Part::new(
        "Song",
        "A4:4 A4 B4:8 | A4:4 A4 B4:8 | A4:4 B4 C5 B4 | A4 B4:2 A4 F4:8 | \
         E4:4 C4 E4 F4 | E4 E4:2 C4 B3:8 | A4:4 B4 C5 B4 | A4 B4:2 A4 F4:8 | \
         E4:4 C4 E4 F4 | E4 E4:2 C4 B3:8 | A4:4 A4 B4:8 | A4:4 A4 B4:8 | \
         E4:4 F4 B4:2 A4 F4:4 | E4:16",
        "i7 | i7 | bVImaj7 | bVImaj7 | iv7 | v7 | bVImaj7 | bVImaj7 | iv7 | v7 | i7 | i7 | \
         bVImaj7 | i7",
    )],
    form: &["Song", "Song"],
    drums_from: 0,
    about: "The cherry blossom song uses five notes, A B C E F, from the miyako-bushi scale \
            of Japanese music. All five are in A minor, so lo-fi seventh chords sit right \
            under it.",
};

pub(super) const DIES_IRAE: Song = Song {
    name: "Dies Irae",
    origin: "Gregorian chant, 13th century",
    feel: "Trap",
    genre: Genre::HipHop,
    tonic: D,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 140.0,
    comping: Comping::Held,
    bass: Bass::Sub,
    rhythm: "Trap",
    band: TRAP,
    style: "Trap minor",
    parts: &[
        Part::new(
            "Chant",
            "F4:4 E4 F4 D4 | E4 C4 D4:8 | F4:4 F4 G4 F4 | E4:2 D4 C4:4 E4:2 F4 E4 D4",
            "i | bVII i | i iv | bVII i",
        ),
        Part::new(
            "Chant above",
            "F5:4 E5 F5 D5 | E5 C5 D5:8 | F5:4 F5 G5 F5 | E5:2 D5 C5:4 E5:2 F5 E5 D5",
            "i | bVII i | i iv | bVII i",
        ),
    ],
    form: &["Chant", "Chant", "Chant above", "Chant above", "Chant"],
    drums_from: 0,
    about: "The medieval chant of the Day of Wrath, quoted by Berlioz, Liszt, Rachmaninoff \
            and countless film scores. Eight notes are enough to sound ominous, here over \
            trap hi-hats and a deep 808 bass.",
};

// Source: https://raw.githubusercontent.com/DCMLab/debussy_other_piano_pieces/main/notes/l068_reverie.tsv
// (the note table of the DCML corpus score)
pub(super) const REVERIE: Song = Song {
    name: "Rêverie",
    origin: "Claude Debussy, 1890",
    feel: "Lo-fi",
    genre: Genre::HipHop,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 70.0,
    comping: Comping::Lazy,
    bass: Bass::Lazy,
    rhythm: "Lo-fi",
    band: LO_FI,
    style: "Lo-fi",
    parts: &[
        Part::new("Intro", "-:16 | -:16", "IV | IV"),
        Part::new(
            "Theme",
            "G5:8 D5:8 | _:4 E5:2 F5:2 G5:4 E5:2 D5:2 | E5:3 C5:3 E5:2 D5:8 | _:8 Bb4:4 D5:4 | \
             E5:4 F5:4 C5:8 | _:8 G4:8 | A4:16 | _:16 | \
             A5:8 E5:8 | _:4 C5:2 E5:2 D5:4 Bb4:2 G4:2 | A5:8 E5:8 | _:4 C5:2 E5:2 D5:4 Bb4:2 G4:2 | \
             G5:8 D5:8 | _:4 Bb4:2 D5:2 C5:4 A4:2 G4:2 | A4:4 E4:2 A4:2 F4:4 D4:2 F4:2 | D4:8 C4:8",
            "IV | IV | IV | IV | I | V7 | I | I | vi7 | I ii | vi7 | I ii | V7 | ii I | iii vi | ii7 V7",
        ),
    ],
    form: &["Intro", "Theme", "Theme"],
    drums_from: 0,
    about: "Debussy's early piano piece as lo-fi hip hop. The tune floats for four bars over \
            a B flat chord with a G and a C in it, the IV of F major, before F itself \
            arrives; the quarter-note triplet in its third bar is written as 3 + 3 + 2 \
            sixteenths. In the score the left hand plays these chords as slow arpeggios.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/SatieE/Gnossienne/no_1/no_1.ly
// (Mutopia 2035, from the Salabert edition of 1913)
pub(super) const GNOSSIENNE: Song = Song {
    name: "Gnossienne No. 1",
    origin: "Erik Satie, 1890",
    feel: "Lo-fi",
    genre: Genre::HipHop,
    tonic: F,
    mode: Dorian,
    time: COMMON,
    tempo: 70.0,
    comping: Comping::Lazy,
    bass: Bass::Held,
    rhythm: "Lo-fi",
    band: LO_FI,
    style: "Boom bap minor",
    parts: &[
        Part::new(
            "Theme",
            "-:4 C5:2 Eb5 D5:4 C5 | B4:8 B4 | -:4 C5:2 Eb5 D5:4 C5 | F5:8 F5 | \
             -:4 C5:2 Eb5 D5:4 C5 | B4:8 G4 | G4:4 G4:10 -:2 | G4:4 F4:12 | _:16",
            "i | i | i | i | i | i | v | i | i",
        ),
        Part::new(
            "Shining",
            "Bb4:8 G4 | Bb4:8 G4 | G4:4 F4:12 | _:16",
            "iv | iv | i | i",
        ),
        Part::new(
            "Questioning",
            "-:2 C5 D5 E5 F5 G5 B5 G5 | F5:4 F5:12 | F5:8 E5 | C5:8 B4 | G4:4 F4:12 | _:16",
            "i | i | i | i | i | i",
        ),
        Part::new(
            "From the tip of the thought",
            "-:4 C5:2 Eb5 D5:4 C5 | B4:16 | -:4 C5:2 Eb5 D5:4 C5 | F5:16",
            "i | i | i | i",
        ),
    ],
    form: &[
        "Theme",
        "Theme",
        "Shining",
        "Shining",
        "Questioning",
        "Questioning",
        "Shining",
        "Shining",
        "From the tip of the thought",
        "From the tip of the thought",
        "Shining",
        "Shining",
        "Questioning",
        "Questioning",
        "Shining",
        "Shining",
    ],
    drums_from: 0,
    about: "Satie wrote it without bar lines or a time signature, over a left hand that is \
            only ever three chords: F minor, B flat minor and, once, C minor. The key \
            signature is F minor, but the tune's D natural sounds Dorian, and it leans on \
            B natural below the C. The grace notes before almost every long note are left \
            out. Three of the part names are Satie's instructions to the player: \"Très \
            luisant\", \"Questionnez\", \"Du bout de la pensée\".",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/ChopinFF/O28/Chop-28-4/Chop-28-4.ly
// (Mutopia 468, from the Peters edition of 1879)
pub(super) const PRELUDE_IN_E_MINOR: Song = Song {
    name: "Prelude in E minor",
    origin: "Frédéric Chopin, Op. 28 No. 4, 1839",
    feel: "Lo-fi",
    genre: Genre::HipHop,
    tonic: E,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 60.0,
    comping: Comping::Pulse,
    bass: Bass::Held,
    rhythm: "Lo-fi",
    band: LO_FI,
    style: "Boom bap minor",
    parts: &[
        Part::new("Pickup", "-:12 B3:3 B4:1", ""),
        Part::new(
            "A",
            "B4:12 C5:4 | B4:12 C5:4 | B4:12 C5:4 | B4:12 Bb4:4 | A4:12 B4:4 | A4:12 B4:4 | \
             A4:12 B4:3 A4:1 | A4:12 G#4:4 | _:4 A4:2 B4 D5 C5 E4 A4 | F#4:12 A4:4 | \
             F#4:12 A4:4 | G4:2 F#4 C4 B3 D#4 F#4 D5:1 C5:2 B4:1",
            "i | V7sus4 V7 | bII7 bII7 bvii #v°7 | I7 I7 i7 vi° | bVI iiø7 | iiø7 vii°7 | \
             bVII7 | bvii7 v° | bVImaj7 iv iv iv | V7sus4 V7 iv iv | V7 iv | V7",
        ),
        Part::new(
            "B",
            "B4:12 C5:4 | B4:12 C5:4 | B4:12 C5:4 | B4:3 A#4:1 A#4:4 A5:4 F#5:3 E5:1 | \
             E5:2 D#5 C6 D#5 D#5 E5 G5 B4 | D5:2 C5:2 E5:1 E4:2 A4:1 F#4:6 A4:2 | \
             F#4:12 A4:4 | F#4:15 E4:1 | E4:12 F#4:4 | E4:12 F#4:4 | E4:8 -:8 | \
             E4:8 D#4:8 | E4:16",
            "i | V7sus4 bII7 | biiø7 #v°7 I7 I7 | i7 vi° #iv° iv | V ii° ii° ii° bIII+ i i i | \
             iv iiø7 V7sus4 iv | V7sus4 iv | V7sus4 V7sus4 V V7 | bVI bVI bVI7 iv | \
             V7sus4 I i i | bVI7 | Vsus4 V | i",
        )
        // "stretto" (pressing on) from bar 16 to the climax in bar 18, then
        // back to the tempo; the fermata on the silence in bar 23 and on the
        // last chord.
        .with_tempo(&[
            glide(54, 69, 34),
            tempo(88, 60),
            fermata(168, 8),
            fermata(192, 16),
        ]),
    ],
    form: &["Pickup", "A", "B"],
    drums_from: 1,
    about: "The right hand hardly moves, B and C, then A and B, while the left hand repeats \
            its chords in eighth notes and lets them sink by semitones, one note at a time. \
            Many of those chords have no name of their own, so the chart gives the nearest \
            chord that holds their notes: F, A and D sharp, B7 with its fifth lowered, \
            becomes F7. The triplets in bars 12 and 18 are written as sixteenths, \
            the turn and the grace notes are left out, and the G double sharp is an A.",
};
