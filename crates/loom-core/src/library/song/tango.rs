//! Tango: the habanera, the tango, the milonga and the tango waltz.

use super::*;

pub(super) const HABANERA: Song = Song {
    name: "Habanera",
    origin: "Georges Bizet, Carmen, 1875",
    feel: "Habanera",
    genre: Genre::Tango,
    tonic: D,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Habanera",
    parts: &[Part::new(
        "Aria",
        "D5:3 C#5:1 C5:2 C5 B4:3 Bb4:1 A4:4 | A4:3 Ab4:1 G4:2 G4 F4:3 E4:1 F4:2 E4 | \
         D5:3 C#5:1 C5:2 C5 B4:3 Bb4:1 A4:4 | A4:3 G4:1 F4:2 E4 D4:8",
        "i | i | i | V7 i",
    )],
    form: &["Aria", "Aria", "Aria", "Aria"],
    drums_from: 0,
    about: "Carmen's aria slides down one semitone at a time over the habanera rhythm, \
            long-short-long-long. The Cuban habanera crossed to Spain and to the River \
            Plate, where it grew into the tango.",
};

const TELAR_CHORDS: &str = "i | V7 | V7 | i | iv | i | V7 | i";

pub(super) const TANGO_DEL_TELAR: Song = Song {
    name: "Tango del telar",
    origin: "Written for Ferrous Loom",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    parts: &[
        Part::new(
            "Tema",
            "E5:4 -:2 E5:1 F5:1 E5:4 A4:4 | G#4:6 B4:2 D5:4 F5:4 | E5:3 D5:1 C5:3 B4:1 G#4:8 | \
             A4:12 -:4 | D5:4 -:2 D5:1 E5:1 F5:4 A5:4 | E5:6 C5:2 A4:8 | \
             B4:2 C5 D5 E5 F5:4 G#4:4 | A4:8 -:8",
            TELAR_CHORDS,
        ),
        Part::new(
            "Variación",
            "A4:1 B4 C5 D5 E5:4 A5 E5 | D5:1 E5 F5 G#5 B5:4 G#5 E5 | \
             F5:1 E5 D5 C5 B4:4 D5 G#4 | A4:4 E5 A5:8 | F5:1 G5 A5 F5 D5:4 F5 A5 | \
             E5:1 F5 E5 D5 C5:4 E5 A4 | G#4:2 B4 D5 F5 E5:4 G#4 | A4:4 -:12",
            TELAR_CHORDS,
        ),
    ],
    form: &["Tema", "Variación", "Tema"],
    drums_from: 0,
    about: "A tango in the marcato style: piano and bass hit every beat, short and hard, \
            while the tune pulls between long notes and quick runs. The E7 chord's G sharp, \
            from the harmonic minor, gives the drama.",
};

pub(super) const MILONGA_DEL_RIO: Song = Song {
    name: "Milonga del río",
    origin: "Written for Ferrous Loom",
    feel: "Milonga",
    genre: Genre::Tango,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Tresillo,
    bass: Bass::Tresillo,
    rhythm: "Milonga",
    parts: &[Part::new(
        "Milonga",
        "E5:3 E5 G5:2 C6:8 | B5:3 A5 G5:2 E5:8 | D5:3 D5 F5:2 B5:8 | A5:3 G5 F5:2 D5:8 | \
         G5:3 F5 D5:2 B4:8 | F5:3 E5 D5:2 G4:8 | C5:3 E5 G5:2 E5:4 D5 | C5:8 -:8",
        "I | I | V7 | V7 | V7 | V7 | I | I",
    )],
    form: &["Milonga", "Milonga", "Milonga", "Milonga"],
    drums_from: 0,
    about: "The milonga is tango's quicker, happier cousin. The tune itself plays the \
            3 + 3 + 2 of the bass: two dotted notes and a short one, then a long note.",
};

pub(super) const VALS_DEL_PUERTO: Song = Song {
    name: "Vals del puerto",
    origin: "Written for Ferrous Loom",
    feel: "Tango vals",
    genre: Genre::Tango,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 176.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Tango vals",
    parts: &[
        Part::new(
            "A",
            "E5:4 G5 C6 | B5:8 A5:4 | G5:4 F5 D5 | B4:8 G4:4 | F5:4 A5 G5 | F5:8 D5:4 | \
             E5:4 G5 E5 | C5:12",
            "I | I | V7 | V7 | V7 | V7 | I | I",
        ),
        Part::new(
            "B",
            "G5:4 A5 G5 | E6:8 C6:4 | B5:4 D6 B5 | G5:8 F5:4 | D5:4 F5 B5 | A5:8 G5:4 | \
             G5:4 E5 D5 | C5:12",
            "I | I | V7 | V7 | V7 | V7 | I | I",
        ),
    ],
    form: &["A", "B", "A", "B"],
    drums_from: 0,
    about: "The tango orchestras' waltz, the vals criollo: fast, and felt as one beat to the \
            bar. Two bars of C, four of G7, two of C again: the simplest harmony, so the tune \
            can swing high and low.",
};
