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
    parts: &[Part::new(
        "Prelude",
        "C4:2 E4 G4 C5 E5 G4 C5 E5 | C4 D4 A4 D5 F5 A4 D5 F5 | B3 D4 G4 D5 F5 G4 D5 F5 | \
         C4 E4 G4 C5 E5 G4 C5 E5 | C4 E4 A4 E5 A5 A4 E5 A5 | C4 D4 F#4 A4 D5 F#4 A4 D5 | \
         B3 D4 G4 D5 G5 G4 D5 G5 | B3 C4 E4 G4 C5 E4 G4 C5 | A3 C4 E4 G4 C5 E4 G4 C5 | \
         D3 A3 D4 F#4 C5 D4 F#4 C5 | G3 B3 D4 G4 B4 D4 G4 B4 | G3 Bb3 E4 G4 C#5 E4 G4 C#5",
        "I | ii7 | V7 | I | vi | II7 | V | Imaj7 | vi7 | II7 | V | #iv°7",
    )],
    form: &["Prelude", "Prelude", "Prelude"],
    drums_from: 0,
    about: "Bach's prelude has no tune at all: each bar is one chord, broken into the same \
            rising figure. That makes it a perfect sample for a hip hop beat. Listen to how \
            each chord changes as few notes as it can. Bar 12 is the diminished seventh on \
            C sharp, over a G in the bass, that Bach uses to slip towards D minor.",
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
