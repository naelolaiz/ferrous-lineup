//! Tango: the habanera, the tango, the milonga and the tango waltz.

use super::*;

/// Bizet keeps a D in the bass; the G minor sixth chord sits on it.
const HABANERA_CHORDS: &str = "i | i iv6 | iv6 | V7 i";

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
    style: "Habanera",
    parts: &[
        Part::new("Pickup", "-:12 D5:2 C#5", "i"),
        Part::new(
            "Aria",
            "C5:2 C5:1 C5 B4:2 Bb4 A4 A4:1 A4 G#4:2 G4 | F4:1 G4 E4 F4 G4:2 F4 E4:4 D5:2 C#5 | \
             C5:2 C5:1 C5 B4:2 Bb4 A4:1 A4 -:1 A4 G4:2 F4 | E4:1 F4 D4 E4 F4:2 E4 D4:4 D5:2 C#5",
            HABANERA_CHORDS,
        ),
        Part::new(
            "Aria end",
            "C5:2 C5:1 C5 B4:2 Bb4 A4 A4:1 A4 G#4:2 G4 | F4:1 G4 E4 F4 G4:2 F4 E4:4 D5:2 C#5 | \
             C5:2 C5:1 C5 B4:2 Bb4 A4:1 A4 -:1 A4 G4:2 F4 | E4:1 F4 D4 E4 F4:2 E4 D4:8",
            HABANERA_CHORDS,
        ),
        Part::new(
            "Refrain",
            "D4:3 A4:1 D4:2 E4 F#4:3 A4:1 F#4:2 E4 | D4:3 E4:1 F#4:2 G4 A4:1 A4 A4 A4 B4:2 A4 | \
             G4:3 B4:1 E4:2 F#4 G4:3 B4:1 G4:2 F#4 | E4:3 F#4:1 G4:2 A4 B4:1 B4 B4 B4 C#5:2 B4 | \
             A4:3 A4:1 D5:2 E5 F#5:3 A4:1 F#4:2 E4 | D4:3 E4:1 F#4:2 G4 A4:1 A4 A4 A4 D5:2 C#5 | \
             G4:3 B4:1 E5:2 F#5 G5:3 B4:1 G4:2 F#4 | E4:3 F#4:1 G4:2 A4 C#5:1 B4 G#4 A4 F#5:3 E5:1 | \
             D5:3 A4:1 D5:2 -:10",
            "I | I | ii | ii V7 | I | I | ii | ii V7 | I",
        )
        .in_key(D, Major),
    ],
    form: &["Pickup", "Aria", "Aria end", "Refrain", "Refrain"],
    drums_from: 1,
    about: "Carmen's aria slides down one semitone at a time over the habanera rhythm, \
            long-short-long-long, in D minor; then the refrain, \"L'amour est enfant de \
            Bohème\", turns to D major. The Cuban habanera crossed to Spain and to the \
            River Plate, where it grew into the tango.",
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
    style: "Tango",
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
    style: "Milonga",
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
    style: "Tango vals",
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

/// The first strain of El Choclo: A minor and E7 in sixteenth notes.
const CHOCLO_A_CHORDS: &str = "i | i V7 | V7 | V7 i | i | I7 iv | iv i | V7 i";

/// The second strain leans on the relative major, C, before E7 pulls it back.
const CHOCLO_B_CHORDS: &str = "bVII7 bIII | bVII7 bIII | V7 i | iv V7";

/// The third strain, in A major: a circle of fifths F#7, Bm, B7, E7.
const CHOCLO_C_CHORDS: &str = "I V7 | V7 I | VI7 ii | II7 V7 | I V7 | V7 I | VI7 ii | II7 V7 I I";

/// The second strain, which ends with the pickup into the next part: the
/// G F# G that starts B again, or the E E C# that starts C.
const CHOCLO_B_TUNE: &str = "F5:1 F5:2 F5:1 G5 F5 E5 D#5 F5:1 E5:2 B4:1 D5 C5 B4 A4 | \
                              C5:1 B4:2 F4:1 B4 A4 G4 F#4 A4:2 G4 -:1 E4 D#4 E4 | \
                              D5:1 D5:2 D5:1 E5 D5 C5 B4 D5:2 C5 -:1 C5 D5 C5 | \
                              B4:1 A4 G#4 A4 B4 C5 D5 D#5 E5:2 -:3 G4:1 F#4 G4";

const CHOCLO_B_INTO_C_TUNE: &str = "F5:1 F5:2 F5:1 G5 F5 E5 D#5 F5:1 E5:2 B4:1 D5 C5 B4 A4 | \
                                     C5:1 B4:2 F4:1 B4 A4 G4 F#4 A4:2 G4 -:1 E4 D#4 E4 | \
                                     D5:1 D5:2 D5:1 E5 D5 C5 B4 D5:2 C5 -:1 C5 D5 C5 | \
                                     B4:1 A4 G#4 A4 B4 C5 D5 D#5 E5:2 -:3 E4:1 E5 C#5";

/// The third strain, with its first ending (the pickup back into C).
const CHOCLO_C_TUNE: &str = "A4:1 E4 -:1 C#5:1 A4 E4 A4 C#5 D5:2 B4 -:1 E4 D5 B4 | \
                              G#4:1 E4 -:1 G#4:1 E4 G#4 B4 D5 C#5:2 A4 -:1 E4 C#5 B4 | \
                              A#4:1 F#4 F4 F#4 G#4 A#4 B4 C#5 E5:2 D5 -:1 B4 A#4 B4 | \
                              C#5:2 B4 -:1 A4 B4 A4 A4:2 G#4 -:1 E4 E5 C#5 | \
                              A4:1 E4 -:1 C#5:1 A4 E4 A4 C#5 D5:2 B4 -:1 E4 D5 B4 | \
                              G#4:1 E4 -:1 G#4:1 E4 G#4 B4 D5 C#5:2 A4 -:1 E4 C#5 B4 | \
                              A#4:1 F#4 F4 F#4 G#4 A#4 B4 C#5 E5:2 D5 -:1 F#4 D5 C#5 | \
                              C#5:1 B4 A#4 B4 F#4 A4 G#4 B4 A4:2 -:3 E4:1 E5 C#5";

/// The same with its second ending.
const CHOCLO_C_END_TUNE: &str = "A4:1 E4 -:1 C#5:1 A4 E4 A4 C#5 D5:2 B4 -:1 E4 D5 B4 | \
                                  G#4:1 E4 -:1 G#4:1 E4 G#4 B4 D5 C#5:2 A4 -:1 E4 C#5 B4 | \
                                  A#4:1 F#4 F4 F#4 G#4 A#4 B4 C#5 E5:2 D5 -:1 B4 A#4 B4 | \
                                  C#5:2 B4 -:1 A4 B4 A4 A4:2 G#4 -:1 E4 E5 C#5 | \
                                  A4:1 E4 -:1 C#5:1 A4 E4 A4 C#5 D5:2 B4 -:1 E4 D5 B4 | \
                                  G#4:1 E4 -:1 G#4:1 E4 G#4 B4 D5 C#5:2 A4 -:1 E4 C#5 B4 | \
                                  A#4:1 F#4 F4 F#4 G#4 A#4 B4 C#5 E5:2 D5 -:1 F#4 D5 C#5 | \
                                  C#5:1 B4 A#4 B4 F#4 A4 G#4 B4 A4:2 E5 A5 -:2";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201261939.abc
// (John Chambers' ABC of 2005, in A minor; checked note for note against
// Eugen Pîrvu's piano ABC in D minor, https://github.com/baobach/abc_search/blob/master/database/abc/QmdGRGR4MjTJrRJp95QHxCsTao9ZVvDioyd6cVFCUZiuNe.abc)
pub(super) const EL_CHOCLO: Song = Song {
    name: "El Choclo",
    origin: "Ángel Villoldo, 1903",
    feel: "Milonga",
    genre: Genre::Tango,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Tresillo,
    bass: Bass::Tresillo,
    rhythm: "Milonga",
    style: "Tango",
    parts: &[
        Part::new("Pickup", "-:13 E4:1 E4 F4", "i"),
        Part::new(
            "A",
            "F4:1 E4 E4 A4 A4 C5 C5 F5 F5:2 E5 -:1 E5 E5 A5 | \
             A5:1 E5 E5 C5 C5 D5 D5 C5 C5:2 B4 -:1 E4 E4 F4 | \
             F4:1 E4 E4 G#4 G#4 B4 B4 F5 F5:2 E5 -:1 E5 D#5 E5 | \
             B5:1 G#5 G#5 E5 E5 D5 D5 C5 B4:2 A4 -:1 E4 E4 F4 | \
             F4:1 E4 E4 A4 A4 C5 C5 F5 F5:2 E5 -:1 E5 E5 A5 | \
             A5:1 E5 E5 C#5 C#5 E5 E5 G5 G5:2 F5 -:1 F4 F4 F5 | \
             F5:1 E5 E5 D5 D5 C5 C5 B4 B4:2 A4:1 A4 A4 C5 B4 A4 | \
             A4:2 G#4:1 B4 B4 D5 C5 B4 A4:2 -:3 G4:1 F#4 G4",
            CHOCLO_A_CHORDS,
        ),
        Part::new("B", CHOCLO_B_TUNE, CHOCLO_B_CHORDS),
        Part::new("B into C", CHOCLO_B_INTO_C_TUNE, CHOCLO_B_CHORDS),
        Part::new("C", CHOCLO_C_TUNE, CHOCLO_C_CHORDS).in_key(A, Major),
        Part::new("C end", CHOCLO_C_END_TUNE, CHOCLO_C_CHORDS).in_key(A, Major),
    ],
    form: &["Pickup", "A", "B", "B into C", "C", "C end"],
    drums_from: 1,
    about: "The oldest tango still played everywhere, first heard in Buenos Aires in 1903. \
            The first strain runs up and down A minor and E7 in sixteenth notes, each phrase \
            ending in a rest and a three-note pickup; the second leans on C major, and the \
            third turns to A major with a circle of fifths, F#7, B minor, B7, E7. Villoldo's \
            tangos were still half milonga, so the bass plays the 3 + 3 + 2 here.",
};

const CUMPARSITA_A_CHORDS: &str = "V7 | i | V7 | i | iv | i | V7 | i";

const CUMPARSITA_B_CHORDS: &str = "i viø7 | i V7 | iiø7 V7 | V7 i | i viø7 | i iv | iv i | V7 i";

/// Two bars of G minor and D7 twice over, then the minor's own seventh chords.
const CUMPARSITA_C_CHORDS: &str =
    "i V7 i I7 | iv7 bVII7 bIII V7 | i V7 i I7 | iv7 bVII7 bIII V7 | i V7 | V7 i | i V7 | V7 i";

// Source: https://github.com/cellist/Lilypond-Sheet-Music/blob/master/Vlc,%20Vlc,%20Vlc/Rodr%C3%ADguez,%20Gerardo%20Hernan%20Matos/La%20Cumparsita/v1.ily
// (a cello trio in G minor, written with German note names, so "b" is B flat;
// checked against Javier Tucat Moreno's piano ABC, parts A, B and C:
// https://github.com/baobach/abc_search/blob/master/database/abc/QmXRLueCMcCeFVavmPohAW9vVJe6MXUsamdGb5QLN1KS4x.abc)
pub(super) const LA_CUMPARSITA: Song = Song {
    name: "La Cumparsita",
    origin: "Gerardo Matos Rodríguez, 1916",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: G,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    style: "Tango",
    parts: &[
        Part::new(
            "A",
            "D4:2 C5 A4 F#4 -:1 D4 Eb4 D4 C#4:2 D4 | D4:2 D5 Bb4 G4 -:1 D4 Eb4 D4 C#4:2 D4 | \
             D4:2 C5 A4 F#4 -:1 D4 Eb4 D4 C#4:2 D4 | D4:2 D5 Bb4 G4 -:1 D4 Eb4 D4 C#4:2 D4 | \
             C4:2 G4 F#4 G4 -:1 F#4 G4 F#4 G4:2 F#4 | Bb3:2 G4 F#4 G4 -:1 F#4 G4 F#4 G4:2 F#4 | \
             A3:2 D4 C4 D4 -:1 C5 Bb4 A4 G4:2 F#4 | G4:4 -:12",
            CUMPARSITA_A_CHORDS,
        ),
        Part::new(
            "B",
            "G4:1 A4 Bb4:2 -:1 G4 A4 Bb4 A4:2 G4:6 | -:1 G4 G4 F4 F4 Eb4 Eb4 D4 D4:4 C4 | \
             -:1 A4 A4 G4 G4 F#4 F#4:2 -:1 Eb4 Eb4 D4 D4:4 | -:1 C5 C5:3 C5:1 Bb4 A4 G4:2 D5:6 | \
             G5:1 A5 Bb5:2 -:1 G5 A5 Bb5 A5:2 G5:6 | -:1 G5 G5 F5 F5 Eb5 Eb5 D5 C5:8 | \
             C5:1 Eb5 -:1 G5 G5 F#5 G5:2 Bb4:1 D5 -:1 G5 G5 F#5 G5:2 | \
             -:1 D5 A5 F#5 D5 C5 -:1 F#4 G4:2 -:6",
            CUMPARSITA_B_CHORDS,
        ),
        Part::new(
            "C",
            "G4:1 F#4 G4 A4 -:1 Bb4:2 A4:1 G4:2 D4 -:4 | Eb4:1 D4 Eb4 F4 G4:3 Eb4:1 D4:2 Bb3 -:4 | \
             G4:1 F#4 G4 A4 -:1 Bb4:2 A4:1 G4:2 D4 -:4 | Eb4:1 D4 Eb4 F4 G4:3 Eb4:1 D4:2 Bb3 -:4 | \
             -:2 D5 Eb5:3 D5:1 D5:3 C5:1 C5 Eb5 D5:2 | -:2 C5 D5:3 C5:1 C5:3 Bb4:1 Bb4 D5 C5:2 | \
             -:2 Bb4 C5:3 Bb4:1 Bb4:3 A4:1 A4 C5 Bb4:2 | -:2 A4 G4:3 F#4:1 G4:4 -:4",
            CUMPARSITA_C_CHORDS,
        ),
    ],
    form: &["A", "A", "B", "B", "C", "A"],
    drums_from: 0,
    about: "Written in Montevideo in 1916 by a student, as a march for a carnival \"comparsa\", \
            and turned into a tango by Roberto Firpo's orchestra the next year. The first strain \
            is one two-bar idea four times, D7 answered by G minor, with the C sharp that leans \
            into every D; the second is the tune of \"Si supieras\"; the third climbs over the \
            dominant and falls back home. Piano and bass hit every beat, short and hard.",
};

const PALOMA_VERSE_CHORDS: &str = "I | I V7 | V7 | V7 | I";

const PALOMA_CHORUS_CHORDS: &str = "I | V7 | V7 | I";

const PALOMA_CODA_CHORDS: &str = "I V7 | V7 | V7 | V7 I | I V7 | V7 | V7 | V7 I | I";

// Source: https://github.com/baobach/abc_search/blob/master/database/abc/QmTKDqnk2k82yFfjfFjCYGKjnnXvg3VnLSRaQSNaDyYvYQ.abc
// (Jean-Paul Finck's choral arrangement in C, with the words; the tenor line is
// the tune. Checked against Bill Gray's ABC, https://github.com/Bill-Gray/abc/blob/master/lapaloma.abc)
pub(super) const LA_PALOMA: Song = Song {
    name: "La Paloma",
    origin: "Sebastián Iradier, about 1860",
    feel: "Habanera",
    genre: Genre::Tango,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Habanera,
    bass: Bass::Habanera,
    rhythm: "Habanera",
    style: "Habanera",
    parts: &[
        Part::new("Pickup", "-:14 G4:2", "I"),
        Part::new(
            "Verse",
            "G4:9 E4:2 F4:1 G4:2 A4:2 | B4:1 C5:2 A4:1 B4:2 G4:2 F4:8 | F4:4 -:2 D5:10 | \
             D5:1 E5:2 C5:1 D5:2 B4:2 C5:1 B4:2 A4:1 G4:2 F4:2 | E4:12 -:2 G4:2",
            PALOMA_VERSE_CHORDS,
        ),
        Part::new(
            "Verse end",
            "G4:9 E4:2 F4:1 G4:2 A4:2 | B4:1 C5:2 A4:1 B4:2 G4:2 F4:8 | F4:4 -:2 D5:10 | \
             D5:1 E5:2 C5:1 D5:2 B4:2 C5:1 B4:2 A4:1 G4:2 F4:2 | E4:12 -:4",
            PALOMA_VERSE_CHORDS,
        ),
        Part::new(
            "Chorus",
            "C5:1 C5:2 C5:1 C5:2 B4:2 D5:1 D5:2 C5:1 B4:2 A4:2 | A4:1 G4:13 -:2 | \
             B4:1 B4:2 B4:1 B4:2 A4:2 A4:1 G4:2 G4:1 G4:2 A4:2 | G4:1 F4:1 E4:12 -:2",
            PALOMA_CHORUS_CHORDS,
        ),
        Part::new(
            "Chorus end",
            "C5:1 C5:2 C5:1 C5:2 B4:2 D5:1 D5:2 C5:1 B4:2 A4:2 | A4:1 G4:13 -:2 | \
             B4:1 B4:2 B4:1 B4:2 A4:2 A4:1 G4:2 G4:1 G4:2 A4:2 | G4:1 F4:1 E4:6 -:8",
            PALOMA_CHORUS_CHORDS,
        ),
        Part::new(
            "Coda",
            "-:1 G4:2 G4:1 G4:1 A4:2 F#4:1 G4:8 | -:1 G4:2 G4:1 G4:1 G4:2 A4:1 B4:4 D5:4 | \
             _:1 E5:2 C5:1 D5:1 B4:2 C5:1 A4:1 B4:2 C5:1 E5:2 D5:2 | -:1 D4:1 E4:1 F4:1 A4:3 G4:1 E4:8 | \
             -:1 G4:2 G4:1 G4:1 A4:2 F#4:1 G4:8 | -:1 G4:2 G4:1 G4:1 G4:2 A4:1 B4:4 D5:4 | \
             _:1 F5:2 E5:1 D5:1 E5:2 C5:1 D5:1 B4:2 C5:1 E5:2 D5:2 | D4:1 E4:2 F4:1 A4:2 G4:2 C5:8 | \
             C5:6 -:10",
            PALOMA_CODA_CHORDS,
        ),
    ],
    form: &[
        "Pickup",
        "Verse",
        "Verse end",
        "Chorus",
        "Chorus end",
        "Coda",
    ],
    drums_from: 1,
    about: "Iradier wrote this habanera after a visit to Cuba, and it went round the world. \
            Only two chords, C and G7: the tune hangs on F, the seventh of G7, for a whole bar \
            before each cadence, over the bass's long, short, long, long. \"Si a tu ventana llega \
            una paloma\" is the chorus, and \"Ay, chinita, que sí\" the coda.",
};

/// A7 and D minor in the síncopa, then D7 to G minor, then the run home.
const GALLO_A_CHORDS: &str = "V7 i | V7 i | I7 iv | I7 iv | V7 i | V7 i";

const GALLO_B_CHORDS: &str = "V7 | I | V7 | I | V7 | I | I7 IV | V7 I";

// Source: https://github.com/baobach/abc_search/blob/master/database/abc/QmQfW3aBzYDVq2oyUDGcr4wbZxsFHpGVfiy11JiY9zjksw.abc
// (Pablo Moro's piano arrangement; the melody is the top note of the right hand.)
pub(super) const GALLO_CIEGO: Song = Song {
    name: "Gallo ciego",
    origin: "Agustín Bardi, 1916",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    style: "Tango",
    parts: &[
        Part::new(
            "A",
            "A4:1 Bb4:2 A4:1 Bb4:2 A4:1 Bb4:1 A4:1 Bb4:2 A4:1 Bb4:2 A4:1 Bb4:1 | \
             A4:1 Bb4:2 A4:1 Bb4:2 A4:1 Bb4:1 A4:1 Bb4:2 A4:1 Bb4:2 A4:1 Bb4:1 | \
             D5:1 Eb5:2 D5:1 Eb5:2 D5:1 Eb5:1 D5:1 Eb5:2 D5:1 Eb5:2 D5:1 Eb5:1 | \
             D5:1 Eb5:2 D5:1 Eb5:2 D5:1 Eb5:1 D5:1 G5:2 D5:1 Bb4:1 A4:1 G4:1 G#4:1 | \
             A4:1 Bb4:2 A4:1 Bb4:2 A4:1 Bb4:1 A4:1 Bb4:2 A4:1 Bb4:2 A4:1 Bb4:1 | \
             A4:1 G4:2 F4:1 E4:2 D4:1 C#4:1 D4:2 -:2 D5:2 -:2",
            GALLO_A_CHORDS,
        ),
        Part::new(
            "B",
            "E4:1 G4 Bb4 D5 C#5 D5 Bb4:2 -:1 D5:3 Bb4:2 -:2 | \
             E4:1 F4 A4 D5 C#5 D5 A4:2 -:1 D5:3 A4:2 -:2 | \
             E4:1 G4 Bb4 D5 C#5 E5 D5:2 -:1 F5:1 E5 A5 G5:2 E5:1 C5 | \
             F5:1 Bb5:2 A5:1 G5 F5 E5 D5 C5:1 B4 Bb4 A4 G4 F4 E4 D#4 | \
             E4:1 G4 Bb4 D5 C#5 D5 Bb4:2 -:1 D5:3 Bb4:2 -:2 | \
             E4:1 F4 A4 D5 C#5 D5 A4:2 -:1 D5:3 A4:2 -:2 | \
             -:1 A4:1 C5 Eb5 G5 F#5 G5:2 -:1 Bb4:1 G5 Gb5 F5:2 -:2 | \
             E5:1 C5 D5 Bb4 A4 C5 Bb4 G4 F4:2 -:2 F5:1 -:3",
            GALLO_B_CHORDS,
        )
        .in_key(F, Major),
    ],
    form: &["A", "A", "B", "A"],
    drums_from: 0,
    about: "Bardi's tango opens not with a tune but with a rhythm: two notes a semitone apart, \
            A and B flat, hammered in the 3 + 3 + 2 of the síncopa over A7 and D minor, then \
            moved up to D7 and G minor before a scale runs down to D. The second strain, in \
            F major, is all arpeggios, each one leaning on a C sharp.",
};
