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
    band: SALON,
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
    origin: "Written for Ferrous Lineup",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
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
    origin: "Written for Ferrous Lineup",
    feel: "Milonga",
    genre: Genre::Tango,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Tresillo,
    bass: Bass::Tresillo,
    rhythm: "Milonga",
    band: TANGO,
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
    origin: "Written for Ferrous Lineup",
    feel: "Tango vals",
    genre: Genre::Tango,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 176.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Tango vals",
    band: SALON,
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

/// The third strain, with its first ending: "D.C.", and the E E F pickup
/// that starts the tango again.
const CHOCLO_C_TUNE: &str = "A4:1 E4 -:1 C#5:1 A4 E4 A4 C#5 D5:2 B4 -:1 E4 D5 B4 | \
                              G#4:1 E4 -:1 G#4:1 E4 G#4 B4 D5 C#5:2 A4 -:1 E4 C#5 B4 | \
                              A#4:1 F#4 F4 F#4 G#4 A#4 B4 C#5 E5:2 D5 -:1 B4 A#4 B4 | \
                              C#5:2 B4 -:1 A4 B4 A4 A4:2 G#4 -:1 E4 E5 C#5 | \
                              A4:1 E4 -:1 C#5:1 A4 E4 A4 C#5 D5:2 B4 -:1 E4 D5 B4 | \
                              G#4:1 E4 -:1 G#4:1 E4 G#4 B4 D5 C#5:2 A4 -:1 E4 C#5 B4 | \
                              A#4:1 F#4 F4 F#4 G#4 A#4 B4 C#5 E5:2 D5 -:1 F#4 D5 C#5 | \
                              C#5:1 B4 A#4 B4 F#4 A4 G#4 B4 A4:2 -:3 E4:1 E4 F4";

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
// Form, as Chambers writes it: A once, B twice, then C, whose first ending says
// "D.C."; back at the start the repeats are not taken again, so A, B once and C
// with its second ending close the piece in A major. Other editions end
// differently (Wikifonia's lead sheet and Pîrvu go back to A and stop there).
// PD: Villoldo died in 1919 (the source's header: 1868-1919); IMSLP also holds a
// 1913 printing, O Malho, Rio de Janeiro (https://imslp.org/wiki/El_Choclo_(Villoldo%2C_%C3%81ngel_Gregorio)).
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
    band: TANGO,
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
    form: &[
        "Pickup", "A", "B", "B into C", "C", "A", "B into C", "C end",
    ],
    drums_from: 1,
    about: "The oldest tango still played everywhere, first heard in Buenos Aires in 1903. \
            The first strain runs up and down A minor and E7 in sixteenth notes, each phrase \
            ending in a rest and a three-note pickup; the second leans on C major, and the \
            third turns to A major with a circle of fifths, F#7, B minor, B7, E7, before the \
            whole tango comes round once more to end in A major. Villoldo's tangos were \
            still half milonga, so the bass plays the 3 + 3 + 2 here.",
};

const CUMPARSITA_A_CHORDS: &str = "V7 | i | V7 | i | iv | i | V7 | i";

const CUMPARSITA_B_CHORDS: &str = "i | i V7 | V7 | V7 i | i | i iv | iv i | V7 i";

/// G minor answered by C minor, twice; then a sequence that steps down over
/// D7 and G minor.
const CUMPARSITA_C_CHORDS: &str = "i | iv i | i | iv i | i V7 | V7 i | i V7 | V7 i";

// Source: https://github.com/cellist/Lilypond-Sheet-Music/blob/master/Vlc,%20Vlc,%20Vlc/Rodr%C3%ADguez,%20Gerardo%20Hernan%20Matos/La%20Cumparsita/v1.ily
// (a cello trio in G minor, written with German note names, so "b" is B flat;
// the chords are read from its two lower cellos. Its form is |: A :| Fine,
// |: B :|, |: C :|, D.C. al Fine; back at the start the repeat is not taken.
// Left out: the trio's two-bar lead-in and the G minor chords it strikes in
// the last bar of A and of C, which the band plays. The trio takes the second
// half of B an octave below the first; here it is an octave above, as in
// Eugen Pîrvu's piano ABC, which has A and B:
// https://github.com/baobach/abc_search/blob/master/database/abc/QmXqK3udSEJjr51sd3k7mHyBkRP2JuM1bK3oNdgNdEjcjn.abc
// C checked against Pablo Moro's piano ABC, the same tune with small changes of rhythm:
// https://github.com/baobach/abc_search/blob/master/database/abc/QmVk7Me2CRhrct17L4D8SiMBaHdpL2jYxCVaJJpTQDnVxj.abc)
// PD: Matos Rodríguez died in 1948 (https://en.wikipedia.org/wiki/Gerardo_Matos_Rodr%C3%ADguez).
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
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new(
            "A",
            "D4:2 C5 A4 F#4 -:1 D4 Eb4 D4 C#4:2 D4 | D4:2 D5 Bb4 G4 -:1 D4 Eb4 D4 C#4:2 D4 | \
             D4:2 C5 A4 F#4 -:1 D4 Eb4 D4 C#4:2 D4 | D4:2 D5 Bb4 G4 -:1 D4 Eb4 D4 C#4:2 D4 | \
             C4:2 G4 F#4 G4 -:1 F#4 G4 F#4 G4:2 F#4 | Bb3:2 G4 F#4 G4 -:1 F#4 G4 F#4 G4:2 F#4 | \
             A3:2 D5 C5 D5 -:1 C5 Bb4 A4 G4:2 F#4 | G4:4 -:12",
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
    form: &["A", "A", "B", "B", "C", "C", "A"],
    drums_from: 0,
    about: "Written in Montevideo by a student, as a march for a carnival \"comparsa\", and \
            made a tango by Roberto Firpo's orchestra, which first played it. The first strain \
            is one two-bar idea four times, D7 answered by G minor, with the C sharp that leans \
            into every D; the second runs down from G over G minor and D7, then again an octave \
            higher, touching C minor; the third answers G minor with C minor, then steps down \
            in sequence over D7 and G minor. Piano and bass hit every beat, short and hard.",
};

const PALOMA_VERSE_CHORDS: &str = "I | I V7 | V7 | V7 | I";

const PALOMA_CHORUS_CHORDS: &str = "I | V7 | V7 | I";

/// "Ay, chinita, que sí" twice, then the last "¡Ay!" on G7 and C.
const PALOMA_CODA_CHORDS: &str = "I | V7 | V7 | I | I | V7 | V7 | I | V7 | I";

// Source: https://github.com/baobach/abc_search/blob/master/database/abc/QmTKDqnk2k82yFfjfFjCYGKjnnXvg3VnLSRaQSNaDyYvYQ.abc
// (Jean-Paul Finck's choral arrangement in C, with the words; the tenor line is
// the tune, the upper note where the tenors split in thirds (two bars of the
// coda's second half), and the chords are read from its three voices; a passing
// diminished chord at "una pa-" in the chorus is left as C, as Bill Gray's chord
// symbols have it. Checked against Bill Gray's ABC,
// https://github.com/Bill-Gray/abc/blob/master/lapaloma.abc, which writes the
// sixteenth-eighth-sixteenth figures as triplets and ends with a vamp of its own.
// The last bar of the coda is padded with a rest, its 2/4 bar having no partner.)
// PD: Iradier died in 1865 (https://sin80.com/en/artist/iradier).
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
    band: SALON,
    style: "Habanera",
    parts: &[
        Part::new("Pickup", "-:14 G4:2", "I"),
        Part::new(
            "Verse",
            "G4:9 E4:2 F4:1 G4:2 A4:2 | B4:1 C5:2 A4:1 B4:2 G4:2 F4:8 | _:4 -:2 D5:2 D5:8 | \
             _:1 E5:2 C5:1 D5:2 B4:2 C5:1 B4:2 A4:1 G4:2 F4:2 | E4:12 -:2 G4:2",
            PALOMA_VERSE_CHORDS,
        ),
        Part::new(
            "Verse end",
            "G4:9 E4:2 F4:1 G4:2 A4:2 | B4:1 C5:2 A4:1 B4:2 G4:2 F4:8 | _:4 -:2 D5:2 D5:8 | \
             _:1 E5:2 C5:1 D5:2 B4:2 C5:1 B4:2 A4:1 G4:2 F4:2 | E4:12 -:4",
            PALOMA_VERSE_CHORDS,
        ),
        Part::new(
            "Chorus",
            "C5:1 C5:2 C5:1 C5:2 B4:2 D5:1 D5:2 C5:1 B4:2 A4:2 | A4:1 G4:13 -:2 | \
             B4:1 B4:2 B4:1 B4:2 A4:2 A4:1 G4:2 G4:1 G4:2 A4:2 | G4:1 F4:1 E4:12 -:2",
            PALOMA_CHORUS_CHORDS,
        ),
        // The second time, the chorus runs straight into the coda's pickup.
        Part::new(
            "Chorus end",
            "C5:1 C5:2 C5:1 C5:2 B4:2 D5:1 D5:2 C5:1 B4:2 A4:2 | A4:1 G4:13 -:2 | \
             B4:1 B4:2 B4:1 B4:2 A4:2 A4:1 G4:2 G4:1 G4:2 A4:2 | \
             G4:1 F4:1 E4:6 -:1 G4:2 G4:1 G4:1 A4:2 F#4:1",
            PALOMA_CHORUS_CHORDS,
        ),
        Part::new(
            "Coda",
            "G4:8 -:1 G4:2 G4:1 G4:1 G4:2 A4:1 | B4:4 D5:4 _:1 E5:2 C5:1 D5:1 B4:2 C5:1 | \
             A4:1 B4:2 C5:1 E5:2 D5:2 -:1 D4:1 E4:1 F4:1 A4:3 G4:1 | \
             E4:8 -:1 G4:2 G4:1 G4:1 A4:2 F#4:1 | \
             G4:8 -:1 G4:2 G4:1 G4:1 G4:2 A4:1 | B4:4 D5:4 _:1 F5:2 E5:1 D5:1 E5:2 C5:1 | \
             D5:1 B4:2 C5:1 E5:2 D5:2 D4:1 E4:2 F4:1 A4:2 G4:2 | C5:14 -:2 | \
             B4:12 -:2 B4:2 | C5:2 -:14",
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

/// The third strain, in B flat: B flat, C, F7 and back, twice.
const GALLO_C_CHORDS: &str = "I II | V7 I | I V7 | V7 I | I II | V7 I | I V7 | V7 I";

// Source: https://github.com/baobach/abc_search/blob/master/database/abc/QmQfW3aBzYDVq2oyUDGcr4wbZxsFHpGVfiy11JiY9zjksw.abc
// (Pablo Moro's piano arrangement; the melody is the top note of the right hand
// and the chords are read from the left. Its form is A, B, C in B flat with a
// repeat sign at the end and none at the start, so the whole tango is played
// twice and ends in B flat. A turn of two 32nd notes is folded into the note
// before it (D5:3 for D, C sharp, D; C5:3 for C, B, C); the 32nd notes E flat,
// E before an F are one sixteenth E.)
// PD: Bardi died in 1941 (https://en.wikipedia.org/wiki/Agust%C3%ADn_Bardi).
pub(super) const GALLO_CIEGO: Song = Song {
    name: "Gallo ciego",
    origin: "Agustín Bardi",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
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
        // It ends with the pickup, D and E, into the third strain.
        Part::new(
            "B",
            "E4:1 G4 Bb4 D5 C#5 D5 Bb4:2 -:1 D5:3 Bb4:2 -:2 | \
             E4:1 F4 A4 D5 C#5 D5 A4:2 -:1 D5:3 A4:2 -:2 | \
             E4:1 G4 Bb4 D5 C#5 E5 D5:2 -:1 F5:1 E5 A5 G5:2 E5:1 C5 | \
             F5:1 Bb5:2 A5:1 G5 F5 E5 D5 C5:1 B4 Bb4 A4 G4 F4 E4 D#4 | \
             E4:1 G4 Bb4 D5 C#5 D5 Bb4:2 -:1 D5:3 Bb4:2 -:2 | \
             E4:1 F4 A4 D5 C#5 D5 A4:2 -:1 D5:3 A4:2 -:2 | \
             -:1 A4:1 C5 Eb5 G5 F#5 G5:2 -:1 Bb4:1 G5 Gb5 F5:2 -:2 | \
             E5:1 C5 D5 Bb4 A4 C5 Bb4 G4 F4:2 -:2 F5:1 D4:2 E4:1",
            GALLO_B_CHORDS,
        )
        .in_key(F, Major),
        Part::new(
            "C",
            "F4:5 D5:3 E4:1 E4:2 D#4:1 E4:1 D#4:1 E4:2 | \
             Eb4:5 C5:3 D4:1 D4:2 C#4:1 D4:1 A4:1 G4:1 A4:1 | \
             G4:1 F4:2 A4:1 G4:3 A4:1 G4:2 F4:3 A4:1 G4:1 A4:1 | \
             G4:1 F4:2 A4:1 G4:3 A4:1 G4:2 F4:3 D4:2 E4:1 | \
             F4:5 D5:3 E4:1 E4:2 D#4:1 E4:1 D#4:1 E4:2 | \
             Eb4:5 C5:3 D4:1 D4:2 C#4:1 D4:1 A4:1 G4:1 A4:1 | \
             G4:1 F4:2 A4:1 G4:3 A4:1 G4:2 F4:3 A4:1 G4:1 A4:1 | \
             G4:1 F4:2 Eb5:1 D5:2 C5:1 Bb4:3 -:2 Bb5:2 -:2",
            GALLO_C_CHORDS,
        )
        .in_key(ASharp, Major),
    ],
    form: &["A", "B", "C", "A", "B", "C"],
    drums_from: 0,
    about: "Bardi's tango opens not with a tune but with a rhythm: two notes a semitone apart, \
            A and B flat, hammered in the 3 + 3 + 2 of the síncopa over A7 and D minor, then \
            moved up to D7 and G minor before a scale runs down to D. The second strain, in \
            F major, is all arpeggios, each one leaning on a C sharp; the third, in B flat, \
            creeps by semitones, E, D sharp, E, over a C chord before F7 brings it home, and \
            then the whole tango is played again.",
};

/// Six bars of C minor, eight of G7, two of C minor.
const LAGRIMAS_FIRST_CHORDS: &str =
    "i | i | i | i | i | i | V7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 | i | i";

/// The answer turns through C7 to F minor and back.
const LAGRIMAS_SECOND_CHORDS: &str =
    "i | i | i | i | i | I7 | iv | iv | iv | iv | i | i | V7 | V7 | i | i";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201299835.abc
// (John Chambers' ABC of 2018, in C minor, with chords; abc_tune_jc_201299836.abc
// has the same notes laid out differently. Where the score doubles the tune in
// octaves the lower note is kept, except in bars 21-32, where the upper one
// joins the line to the thirds of bars 25-28; where it writes thirds or chords
// the top note is kept. The walking bass under bars 15-16 and 47-48 is written
// as plain C minor. Bar 56 has an eighth rest where the bar needs a quarter.)
// PD: Pascual de Gullo died in 1951 (https://viaf.org/viaf/51412200); the tune is
// dated 1913 in the source and Victor recorded it in 1917 (matrix G-1958,
// https://adp.library.ucsb.edu/index.php/matrix/refer/600001130).
pub(super) const LAGRIMAS_Y_SONRISAS: Song = Song {
    name: "Lágrimas y sonrisas",
    origin: "Pascual de Gullo, 1913",
    feel: "Tango vals",
    genre: Genre::Tango,
    tonic: C,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 168.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Tango vals",
    band: TANGO,
    style: "Vals in minor",
    parts: &[
        Part::new(
            "A1",
            "G4:4 C5 Eb5 | G5:8 Ab5:4 | G5:12 | _:8 -:4 | G5:4 F5 Eb5 | Eb5 D5 C5 | Eb5:12 | \
             D5:12 | D5:4 Eb5 F5 | D5 B4 G4 | Ab4:12 | G4:12 | G4:4 B4 C5 | D5 Eb5 B4 | \
             C5:12 | _:8 -:4",
            LAGRIMAS_FIRST_CHORDS,
        ),
        Part::new(
            "A2",
            "G4:4 C5 Eb5 | G5:8 Ab5:4 | G5:12 | _:8 -:4 | G5:4 F#5 G5 | C6:8 C6:4 | C6:12 | \
             Ab5:12 | Ab5:4 C6 Ab5 | C6 Ab5 C6 | G5 C6 G5 | C6 G5 C6 | G5:12 | B5:12 | C6:12 | \
             _:4 -:8",
            LAGRIMAS_SECOND_CHORDS,
        ),
        Part::new(
            "B1",
            "G5:12 | Eb5:12 | C5:12 | G4:12 | G4:4 C5 D5 | Eb5 D5 C5 | D5:12 | D5:8 -:4 | \
             Ab5:12 | F5:12 | D5:12 | B4:12 | B4:4 C5 D5 | Eb5:8 D5:4 | C5:12 | _:8 -:4",
            LAGRIMAS_FIRST_CHORDS,
        ),
        Part::new(
            "B2",
            "G5:12 | Eb5:12 | C5:12 | G4:12 | G4:4 C5 Eb5 | G4 C5 G4 | Ab4:12 | _:8 -:4 | \
             Ab5:8 F5:2 D5:2 | Ab4:12 | G5:8 Eb5:2 C5:2 | G4:12 | G4:4 B4 C5 | D5 Eb5 B4 | \
             C5:12 | _:4 -:8",
            LAGRIMAS_SECOND_CHORDS,
        ),
        Part::new(
            "C1",
            "E5:4 G5 E5 | C5 E5 C5 | G4 C5 G4 | E4 G4 E4 | C5 B4 A4 | E5 D5 C5 | \
             B4:4 -:2 B5:2 B5:4 | B5:4 -:8 | F5:4 A5 F5 | D5 F5 D5 | B4 D5 B4 | G4 B4 G4 | \
             F4 E4 D4 | A4 G4 F4 | E4:4 -:2 G5:2 G5:4 | C6:4 -:8",
            "I | I | I | I | I | I | V7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 | I | I",
        )
        .in_key(C, Major),
        Part::new(
            "C2",
            "E5:4 G5 E5 | C5 E5 C5 | G4 C5 G4 | E4 G4 E4 | C5 B4 C5 | E5 D5 C5 | \
             A4:4 -:2 A5:2 A5:4 | A5:4 -:8 | A5:4 G#5 A5 | F5 D5 C5 | G5 F#5 G5 | E5 C5 A4 | \
             G4:2 A4 B4 C5 D5 E5 | F5:8 B4:4 | C5:12 | _:4 -:8",
            "I | I | I | I | I | I | IV | IV | IV | ii7 | I | I | V7 | V7 | I | I",
        )
        .in_key(C, Major),
    ],
    form: &[
        "A1", "A2", "A1", "A2", "B1", "B2", "B1", "B2", "C1", "C2", "C1", "C2", "A1", "A2", "B1",
        "B2",
    ],
    drums_from: 0,
    about: "A tango waltz in C minor, felt one beat to the bar. Each minor strain is sixteen \
            bars of C minor and G7 answered by sixteen that pass through C7 to F minor; the \
            trio turns to C major, with a D minor seventh before its last cadence. After the \
            trio both minor strains come back, and the piece ends at the Fine of the second.",
};

const MEDIA_LUZ_REFRAIN_CHORDS: &str = "I | V7 | V7 | I";

const MEDIA_LUZ_REFRAIN_LAST_CHORDS: &str = "I | V7 | V7 | I V7 I I";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/folktunefinder/abc_tune_folktunefinder_19715.txt
// (John Chambers' ABC, http://trillian.mit.edu/~jc/music/abc/tango/A_Media_Luz_G.abc,
// "from an undated edition by O. Di Bella Music Co., New York", in G with chords.
// Checked against the Wikifonia lead sheet in D:
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/EdgardoDonatoAMediaLuz.abc
// Where the two differ in the verse, the Di Bella edition is followed.)
// PD: composed 1925, published 1926 (https://en.wikipedia.org/wiki/A_Media_Luz), before 1930.
pub(super) const A_MEDIA_LUZ: Song = Song {
    name: "A media luz",
    origin: "Edgardo Donato, 1925",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new("Pickup", "-:6 D5:2 B4:2 G4:2 B4:2 F#5:2", "I"),
        Part::new(
            "Refrain",
            "E5:6 D5:2 B4:2 G4:2 B4:2 E5:2 | D5:6 D5:2 D5:2 E5:2 C5:2 D5:2 | \
             B4:4 A4:2 C5:2 C5:2 D5:2 B4:2 A4:2 | D5:4 -:2 D5:2 B4:2 G4:2 B4:2 F#5:2",
            MEDIA_LUZ_REFRAIN_CHORDS,
        ),
        Part::new(
            "Refrain into verse",
            "E5:6 D5:2 B4:2 G4:2 B4:2 E5:2 | D5:6 D5:2 D5:2 E5:2 C5:2 D5:2 | \
             B4:4 A4:2 C5:2 C5:2 D5:2 B4:2 A4:2 | G4:2 -:2 D5:2 -:2 G5:2 -:2 D5:4",
            MEDIA_LUZ_REFRAIN_LAST_CHORDS,
        ),
        Part::new(
            "Verse",
            "D5:4 Bb4:4 _:4 D5:2 D5:2 | Eb5:4 C5:4 _:4 F#5:4 | F#5:6 D5:2 Eb5:2 D5:4 C#5:2 | \
             D5:8 -:8 | -:4 Bb4:2 C5:2 D5:2 F5:2 Eb5:2 D5:2 | D5:4 C5:6 -:6 | \
             -:4 A4:2 Bb4:2 C5:2 Eb5:2 D5:2 C5:2 | Bb4:12 Bb4:4 | C5:2 Bb4:4 C5:2 D5:4 Bb4:4 | \
             C5:4 A4:12",
            "i | iv V7 | V7 | i | i | V7 | V7 | i | i | V7",
        )
        .in_key(G, HarmonicMinor),
        Part::new(
            "Verse end",
            "C5:4 A4:4 Bb4:2 A4:2 Bb4:2 C5:2 | D5:12 D5:4 | G5:2 G5:4 G5:2 G5:4 F5:4 | \
             Eb5:4 C5:12 | A5:2 F#5:4 D5:2 Eb5:2 D5:4 C5:2 | D5:4 Bb4:12 | \
             -:4 Bb4:2 C5:2 D5:2 D5:2 C5:2 Bb4:2 | A4:4 C5:2 C5:2 C5:2 C5:2 Bb4:2 A4:2 | \
             A4:2 G4:2 Bb4:2 A4:2 C5:2 Bb4:2 A4:2 F#4:2 | G4:4 -:2 D5:2 B4:2 G4:2 B4:2 F#5:2",
            "V7 | i | I7 | iv | V7 | i | i | V7 | V7 | I",
        )
        .in_key(G, HarmonicMinor),
        Part::new(
            "Refrain again",
            "E5:6 D5:2 B4:2 G4:2 B4:2 E5:2 | D5:6 D5:2 D5:2 E5:2 C5:2 D5:2 | \
             B4:4 A4:2 C5:2 C5:2 D5:2 B4:2 A4:2 | G4:4 -:2 D5:2 B4:2 G4:2 B4:2 F#5:2",
            MEDIA_LUZ_REFRAIN_CHORDS,
        ),
        Part::new(
            "Refrain end",
            "E5:6 D5:2 B4:2 G4:2 B4:2 E5:2 | D5:6 D5:2 D5:2 E5:2 C5:2 D5:2 | \
             B4:4 A4:2 C5:2 C5:2 D5:2 B4:2 A4:2 | G4:2 -:2 D5:2 -:2 G5:2 -:6",
            MEDIA_LUZ_REFRAIN_LAST_CHORDS,
        ),
    ],
    form: &[
        "Pickup",
        "Refrain",
        "Refrain into verse",
        "Verse",
        "Verse end",
        "Refrain",
        "Refrain again",
        "Refrain",
        "Refrain end",
    ],
    drums_from: 1,
    about: "The refrain is only four bars over G and D7, played twice before the verse and \
            four times after it. The verse turns to G minor, swinging between G minor and D7, \
            with a G7 that leads to C minor; its last bar lands on G major for the refrain.",
};

const ADIOS_A1_CHORDS: &str = "I III7 | IV | V7 | I | I #ii°7 | ii7 | V7 | I";

/// The second half falls through C minor seventh and F7 to B flat minor.
const ADIOS_A2_CHORDS: &str = "I III7 | IV | V7 | I | v7 I7 | iv | I I II7 V7 | I";

const ADIOS_B_CHORDS: &str = "III7 | vi7 | ii7 V7 | I I7 | IV iv | I #ii°7 | ii7 V7 | I";

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/folktunefinder/abc_tune_folktunefinder_619.txt
// (Michel Bellon's ABC of 2005, in F, no chords; it gives the form: A, B with
// two endings, D.S. to the Fine at the end of A.)
// Checked against two Wikifonia lead sheets in C, with chords:
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/JulioSandersRobertLopezPabloTheDreamer.abc
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/JuliusCesarSandersDorcasCochranIGETIDEASWhenIDanceWithYou.abc
// (Bellon's rhythms are short or long by an eighth in a few bars; the
// rhythms and the D flat in bar 14 follow the two lead sheets. Chords are
// from the lead sheets: "Pablo" writes "Eb" (in C) where "I Get Ideas" has
// "Ebdim", read here as #ii°7; A2's C minor seventh and F7 are from Pablo's
// second time through, the B flat minor in A2 and B from I Get Ideas.
// Neither sheet has B's first ending, so its chords (ii7 V7, I) are read
// from the tune.)
// PD: composed and published 1927 (https://en.wikipedia.org/wiki/Adi%C3%B3s_muchachos_(song)), before 1930.
pub(super) const ADIOS_MUCHACHOS: Song = Song {
    name: "Adiós muchachos",
    origin: "Julio César Sanders, 1927",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new("Pickup", "-:10 C5:2 C5:2 C5:2", "I"),
        Part::new(
            "A1",
            "A5:2 F5:2 F5:2 F5:2 G5:2 E5:2 E5:2 E5:2 | F5:4 D5:4 -:2 G5:2 G5:2 F5:2 | \
             F5:4 E5:4 -:2 F5:2 F5:2 E5:2 | D5:4 C5:4 -:2 C5:2 B4:2 C5:2 | \
             F5:2 C5:2 B4:2 C5:2 E5:2 C5:2 B4:2 C5:2 | D5:4 Bb4:4 -:2 C5:2 B4:2 C5:2 | \
             E5:2 C5:2 B4:2 C5:2 D5:2 Bb4:2 A4:2 Bb4:2 | C5:4 A4:4 -:2 C5:2 C5:2 C5:2",
            ADIOS_A1_CHORDS,
        ),
        Part::new(
            "A2",
            "A5:2 F5:2 F5:2 F5:2 G5:2 E5:2 E5:2 E5:2 | F5:4 D5:4 -:2 G5:2 G5:2 F5:2 | \
             F5:4 E5:4 -:2 F5:2 F5:2 E5:2 | D5:4 C5:4 -:2 F5:2 F5:2 C5:2 | \
             C5:2 Bb4:2 Bb4:2 A4:2 A4:2 G4:2 G4:2 F4:2 | C5:4 Bb4:4 -:2 Db5:2 Bb4:2 G4:2 | \
             G4:2 F4:2 E4:2 F4:2 A4:4 G4:4 | F4:4 -:2 A4:2 Bb4:2 A4:2 G#4:2 A4:2",
            ADIOS_A2_CHORDS,
        ),
        Part::new(
            "B",
            "F5:2 E5:4 A4:2 Bb4:2 A4:2 G#4:2 A4:2 | E5:2 D5:4 C5:2 D5:2 C5:2 B4:2 C5:2 | \
             F5:2 E5:4 C5:2 D5:2 E5:2 F5:2 G5:2 | A5:4 -:2 F5:2 G5:2 F5:2 E5:2 F5:2 | \
             A5:2 G5:4 F5:2 Db5:4 Bb4:4 | G5:2 F5:4 C5:2 B4:2 C5:2 E5:2 D5:2 | \
             D5:2 C5:4 C5:2 B4:2 C5:2 E5:2 D5:2 | C5:4 -:2 A4:2 Bb4:2 A4:2 G#4:2 A4:2",
            ADIOS_B_CHORDS,
        ),
        Part::new(
            "B end",
            "F5:2 E5:4 A4:2 Bb4:2 A4:2 G#4:2 A4:2 | E5:2 D5:4 C5:2 D5:2 C5:2 B4:2 C5:2 | \
             F5:2 E5:4 C5:2 D5:2 E5:2 F5:2 G5:2 | A5:4 -:2 F5:2 G5:2 F5:2 E5:2 F5:2 | \
             A5:2 G5:4 F5:2 Db5:4 Bb4:4 | G5:2 F5:4 C5:2 B4:2 C5:2 E5:2 D5:2 | \
             C5:2 Bb4:4 D5:2 A4:4 G4:4 | F4:4 -:6 C5:2 C5:2 C5:2",
            ADIOS_B_CHORDS,
        ),
        Part::new(
            "A2 fine",
            "A5:2 F5:2 F5:2 F5:2 G5:2 E5:2 E5:2 E5:2 | F5:4 D5:4 -:2 G5:2 G5:2 F5:2 | \
             F5:4 E5:4 -:2 F5:2 F5:2 E5:2 | D5:4 C5:4 -:2 F5:2 F5:2 C5:2 | \
             C5:2 Bb4:2 Bb4:2 A4:2 A4:2 G4:2 G4:2 F4:2 | C5:4 Bb4:4 -:2 Db5:2 Bb4:2 G4:2 | \
             G4:2 F4:2 E4:2 F4:2 A4:4 G4:4 | F4:4 -:12",
            ADIOS_A2_CHORDS,
        ),
    ],
    form: &["Pickup", "A1", "A2", "B", "B end", "A1", "A2 fine"],
    drums_from: 1,
    about: "Each phrase of the first strain starts with three repeated notes and a fall, over \
            F and A7, then B flat and C7; the second half sinks through C minor seventh and F7 \
            to B flat minor, a chord borrowed from F minor. The second strain starts on A7 \
            and D minor and climbs back to F through G minor and C7.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/JuanDeDiosFilibertoCaminito.abc
// (Wikifonia lead sheet in E with chords, moved up a fourth to A. In the
// refrain's second phrase the sheet's chords are one bar out of place and
// read F for F sharp; they follow the first phrase, which has the same tune.
// Its F sharp chord (B once moved to A) is a plain triad, so the chart has II.)
// PD: published in Buenos Aires by Pirovano about 1926 (Universidad Nacional de Quilmes
// library, https://biblio.unq.edu.ar/qfind/Record/438) and recorded by Carlos Gardel that
// year (https://en.wikipedia.org/wiki/Caminito_(song)), before 1930.
pub(super) const CAMINITO: Song = Song {
    name: "Caminito",
    origin: "Juan de Dios Filiberto, 1926",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: A,
    mode: Major,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new(
            "Intro",
            "E5:2 E5:4 E5:2 A5:4 E5:2 C#5:2 | D5:2 D5:4 F5:2 A5:4 F5:2 D5:2 | \
             E5:2 E5:4 E5:2 G#5:4 E5:2 D5:2 | A4:4 C#5:2 E5:2 A4:2 C5:2 -:4",
            "I | iv | V7 | I",
        ),
        Part::new(
            "Verse",
            "E5:4 E5:2 E5:2 E5:4 E5:2 E5:2 | F5:4 E5:2 E5:2 G5:4 F5:2 E5:2 | \
             E5:4 D5:2 D5:2 D5:4 D5:2 C#5:2 | D5:8 -:4 D5:2 D5:2 | \
             D5:4 C5:2 C5:2 C5:4 B4:2 B4:2 | B4:8 -:4 B4:2 B4:2 | \
             B4:4 A4:2 A4:2 A4:4 B4:2 A4:2 | G#4:8 -:4 A4:2 C5:2 | \
             E5:4 E5:2 E5:2 E5:4 E5:2 E5:2 | F5:4 E5:2 E5:2 G5:4 F5:2 E5:2 | \
             E5:4 E5:2 E5:2 E5:4 F5:2 G5:2 | F5:8 -:4 F5:2 F5:2 | \
             F5:4 F5:2 F5:2 E5:2 D#5:2 -:4 | E5:8 -:4 D5:2 D5:2 | \
             D5:4 C5:2 C5:2 C5:4 B4:2 B4:2",
            "i V7 | i | i I7 | iv | iv | ii°7 | iv II7 | V7 | i V7 | i | I7 | iv | iv | i | V7",
        )
        .in_key(A, HarmonicMinor),
        Part::new(
            "Bridge",
            "A4:4 -:2 E5:2 F#5:4 E5:2 D#5:2 | E5:4 A5:4 E5:4 D5:4 | \
             C#5:4 A5:4 E5:4 D5:2 C#5:2 | D5:2 C#5:4 C#5:2 B4:6 B4:2",
            "I | I V7 | I V7 | V7",
        ),
        Part::new(
            "Refrain",
            "A4:4 A4:2 C#5:2 E5:6 E5:2 | E5:16 | B4:2 C#5:2 D5:6 D5:2 -:4 | C#5:16 | \
             -:4 A4:2 A4:2 F#5:6 E5:2 | E5:8 D5:8 | -:4 E4:2 E4:2 C#5:6 B4:2 | A4:8 -:8 | \
             A4:4 A4:2 C#5:2 E5:6 E5:2 | E5:16 | -:4 B4:2 C#5:2 D5:6 E5:2 | C#5:16 | \
             -:4 A4:2 C#5:2 F#5:6 E5:2 | E5:8 D5:8 | -:4 E4:2 E4:2 C#5:6 B4:2 | A4:8 -:8",
            "I | V7 | V7 | I | I II | V7 | V7 | I | I | V7 | V7 | I | I II | V7 | V7 | I",
        ),
    ],
    form: &["Intro", "Verse", "Bridge", "Refrain"],
    drums_from: 0,
    about: "A tango-canción in three colours: the introduction in A major, the verse in A \
            minor, where the harmony leans on D minor with a B diminished seventh and B7 on \
            the way to E7, and the refrain back in A major on just A, E7 and B major, its \
            long notes held over whole bars.",
};

const BOEDO_B_CHORDS: &str = "V7 | I | III | vi | IV | I | V7 | I";

const BOEDO_C_CHORDS: &str = "V7 | I | III7 | vi | IV | I | V7 | I";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/JulioDeCaroBoedo.abc
// (Wikifonia lead sheet in D with chords, moved down a step to C. Its bar 41 is
// a quarter short and follows the same bar of the repeat, bar 49; the half bar
// that ends the second strain is filled out with a half rest.)
// PD: premiered on 8 October 1928
// (https://www.todotango.com/english/history/chronicle/337/Boedo-Story-of-the-tango-Boedo/)
// and recorded by De Caro's orchestra that year (Victor BAVE-44378, Victor 80999,
// https://adp.library.ucsb.edu/index.php/matrix/refer/800018508), out before 1930; no
// dated printed edition was found.
pub(super) const BOEDO: Song = Song {
    name: "Boedo",
    origin: "Julio De Caro, 1928",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new("Pickup", "-:6 B4:2 C5:2 E5:2 G5:2 C6:2", "I"),
        Part::new(
            "Intro",
            "B4:2 D5:2 G5:2 B5:2 A4:2 C5:2 F5:2 A5:2 | G4:2 C5:2 E5:2 G5:2 F4:2 A4:2 C5:2 F5:2 | \
             E4:2 G4:2 C5:2 E5:2 D4:2 F4:2 G4:2 B4:2",
            "V IV | I IV | I V7",
        ),
        Part::new(
            "A1",
            "C5:4 -:2 C5:2 B4:2 D5:2 C5:4 | -:2 G4:2 F#4:2 A4:2 G4:6 C#4:2 | \
             E4:2 D4:4 A4:2 G#4:2 B4:2 A4:4 | -:2 F4:2 E4:2 G4:2 F4:6 B3:2 | \
             D4:2 C4:4 G4:2 F#4:2 A4:2 G4:4 | -:2 E4:2 D#4:2 F4:2 E4:6 C4:2 | \
             B3:2 C4:2 C#4:2 D4:2 G4:2 G4:2 G5:4 | B4:2 C5:2 C#5:2 D5:2 G5:2 F#5:4 F5:2",
            "I | I | ii | V7 | I | I | V | V",
        ),
        Part::new(
            "A2",
            "E5:4 -:2 C5:2 B4:2 D5:2 C5:4 | -:2 G4:2 F#4:2 A4:2 G4:6 C#4:2 | \
             E4:2 D4:4 A4:2 G#4:2 B4:2 A4:4 | -:2 F4:2 E4:2 G4:2 F4:6 B3:2 | \
             D4:2 C4:4 G4:2 F#4:2 A4:2 G4:4 | -:2 E5:2 D#5:2 F5:2 E5:4 C5:4 | \
             D5:2 C5:4 B4:2 A4:4 B4:2 C5:2 | E5:2 D5:4 C5:2 B4:4 A4:2 G4:2 | \
             C5:8 -:4 G5:2 F#5:2",
            "I | I | ii | V7 | I | I7 | IV | V7 | I",
        ),
        Part::new(
            "B1",
            "F#5:2 F5:2 F5:2 E5:2 E5:2 D5:2 B4:2 G4:2 | D5:2 Db5:2 C5:4 -:4 E5:2 D#5:2 | \
             D#5:2 D5:2 D5:2 C5:2 C5:2 B4:2 G#4:2 E4:2 | B4:2 Bb4:2 A4:4 -:2 C5:2 B4:2 A4:2 | \
             F4:2 A4:2 D5:2 F5:2 A5:2 C6:2 B5:2 A5:2 | A5:2 G5:2 G5:2 Gb5:2 F5:2 E5:2 D5:2 C5:2 | \
             B4:2 A4:2 G4:2 F4:2 E4:2 G4:2 F4:2 E4:2 | D4:2 Db4:2 C4:4 -:4 G5:2 F#5:2",
            BOEDO_B_CHORDS,
        ),
        Part::new(
            "B2",
            "F#5:2 F5:2 F5:2 E5:2 E5:2 D5:2 B4:2 G4:2 | D5:2 Db5:2 C5:4 -:4 E5:2 D#5:2 | \
             D#5:2 D5:2 D5:2 C5:2 C5:2 B4:2 G#4:2 E4:2 | B4:2 Bb4:2 A4:4 -:2 C5:2 B4:2 A4:2 | \
             F4:2 A4:2 D5:2 F5:2 A5:2 C6:2 B5:2 A5:2 | A5:2 G5:2 G5:2 Gb5:2 F5:2 E5:2 D5:2 C5:2 | \
             B4:2 A4:2 G4:2 F4:2 E4:2 G4:2 F4:2 D4:2 | C4:8 -:8",
            BOEDO_B_CHORDS,
        ),
        Part::new(
            "C1",
            "G4:4 G5:4 F5:2 D5:2 A4:2 B4:2 | C5:8 -:4 D#4:4 | E4:4 E5:4 D5:2 B4:2 F#4:2 G#4:2 | \
             A4:8 -:4 G4:4 | F4:4 A4:4 D5:2 F5:2 E5:2 D5:2 | G5:10 E5:2 D5:2 C5:2 | \
             G5:10 F5:2 E5:2 D5:2 | G5:6 E5:2 C5:2 B4:2 A4:2 Ab4:2",
            BOEDO_C_CHORDS,
        ),
        Part::new(
            "C2",
            "G4:4 G5:4 F5:2 D5:2 A4:2 B4:2 | C5:8 -:4 D#4:4 | E4:4 E5:4 D5:2 B4:2 F#4:2 G#4:2 | \
             A4:8 -:4 G4:4 | F4:4 A4:4 D5:2 F5:2 E5:2 D5:2 | G5:10 E5:2 D5:2 C5:2 | \
             G5:8 F5:2 G4:2 G5:2 E5:2 | C5:8 -:8",
            BOEDO_C_CHORDS,
        ),
    ],
    form: &["Pickup", "Intro", "A1", "A2", "B1", "B2", "C1", "C2"],
    drums_from: 1,
    about: "De Caro's tango is full of half steps. The first strain winds down chromatically \
            over C, D minor and G7; the second runs down whole bars of eighth notes over G7, C, \
            E and A minor; the third leaps between low and high notes over the same circle, \
            G7, C, E7, A minor, F, before the cadence.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/AnselmaAietaAlmaenPena.abc
// (Wikifonia lead sheet with chords, in F sharp minor and A major. Its bars are
// 2/4 bars of sixteenth notes, written here two to a bar; the bridge has five,
// so its last bar is filled out with rests. The sheet has no chord on the
// bridge's first beat, where the verse lands on F sharp; F sharp minor is
// read there from the tune.)
// PD: 1928 (https://todotango.com/english/artists/biography/1017/Anselmo-Aieta); Columbia
// recorded it in New York in 1929 (matrix W97070, Columbia 3506-X,
// https://frontera.library.ucla.edu/es/node/212618; Columbia's matrices W96970
// and W97263 date from January and July 1929), so it was out before 1930; no dated
// printed edition was found.
pub(super) const ALMA_EN_PENA: Song = Song {
    name: "Alma en pena",
    origin: "Anselmo Aieta, 1928",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: FSharp,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new("Pickup", "-:8 -:2 F#4:1 E#4:1 F#4:1 G#4:1 A4:1 A#4:1", "i"),
        Part::new(
            "Verse",
            "B4:2 -:1 F#5:1 E#5:1 G#5:2 F#5:1 D5:1 B4:2 F#5:1 D5:1 B4:2 F#5:1 | \
             C#5:1 A4:2 D5:1 C#5:1 A4:2 F#5:1 C#5:1 A4:2 D5:1 C#5:1 A4:2 F#4:1 | \
             B4:1 G#4:2 C#5:1 B4:1 G#4:2 B4:1 C#5:1 B4:2 C#5:1 B4:1 G#4:2 B4:1 | \
             A4:2 -:6 -:2 F#4:1 E#4:1 F#4:1 G#4:1 A4:1 A#4:1 | \
             B4:2 -:1 F#5:1 E#5:1 G#5:2 F#5:1 D5:1 B4:2 F#5:1 D5:1 B4:2 F#5:1 | \
             C#5:1 A4:2 D5:1 C#5:1 A4:2 F#5:1 C#5:1 A4:2 D5:1 C#5:1 A4:2 F#5:1 | \
             B4:1 G#4:2 C#5:1 B4:1 G#4:2 B4:1 A4:1 F#4:2 A4:1 G#4:1 E#4:2 G#4:1",
            "iv | i | V7 | i | iv | i | V7 V7 i V7",
        ),
        Part::new(
            "Bridge",
            "F#4:2 -:2 A5:1 F#5:1 D5:1 B4:1 G#5:1 E5:1 C#5:1 A4:1 F#5:1 D5:1 B4:1 G#4:1 | \
             E5:2 -:2 F5:1 C5:1 A4:1 F4:1 E5:1 C#5:1 A4:1 E4:1 C#5:1 B4:1 G#4:1 E4:1 | \
             A4:2 -:6 -:8",
            "vi IV iii ii | V bVI I V7 | I",
        )
        .in_key(A, Major),
        Part::new(
            "Refrain",
            "C#5:1 E4:2 E4:1 C#5:1 B#4:1 C#5:1 D5:1 C#5:1 E4:2 E4:1 C#5:1 B#4:1 C#5:1 D5:1 | \
             B4:1 E4:2 E4:1 B#4:1 A#4:1 B#4:1 C#5:1 A4:2 E4:6 | \
             C#5:1 E4:2 E4:1 C#5:1 B#4:1 C#5:1 D5:1 C#5:1 E4:2 E4:1 C#5:1 B#4:1 C#5:1 D5:1 | \
             B4:1 E4:2 E4:1 B#4:1 A#4:1 B#4:1 C#5:1 A4:4 -:1 A4:1 B4:1 C#5:1 | \
             D5:1 D5:2 D5:1 D5:2 C#5:1 B#4:1 D5:1 C#5:2 B#4:1 C#5:2 B4:1 A#4:1 | \
             C#5:1 B4:2 A#4:1 B4:2 A4:1 G#4:1 B4:2 A4:2 -:1 A4:1 B4:1 C#5:1 | \
             D5:1 D5:2 D5:1 D5:2 C#5:1 B#4:1 D5:1 C#5:2 B#4:1 C#5:2 B4:1 A#4:1 | \
             C#5:1 B4:2 A#4:1 B4:2 A4:1 G#4:1 B4:2 A4:2 -:1 A4:1 B4:1 C#5:1 | \
             D5:1 D5:2 D5:1 D5:2 C#5:1 B#4:1 D5:1 C#5:2 B#4:1 C#5:2 B4:1 A#4:1 | \
             C#5:1 B4:2 A#4:1 B4:2 A4:1 G#4:1 A4:4 -:4",
            "I | V7 I | I | V7 I | IV I | V7 I | IV I | V7 I | IV I | V7 I",
        )
        .in_key(A, Major),
    ],
    form: &["Pickup", "Verse", "Bridge", "Refrain"],
    drums_from: 1,
    about: "The verse in F sharp minor is all syncopation, a sixteenth, an eighth and a \
            sixteenth again and again, over B minor, F sharp minor and C sharp 7. A bridge of \
            falling arpeggios leads to the refrain in A major, where chromatic neighbour notes \
            circle each note of the chord over D, A and E7.",
};

const AMANECER_A_CHORDS: &str = "V7 | i | V7 | i | iv | i | V7 | i";

/// The last bar closes C minor, G7, C minor.
const AMANECER_A_END_CHORDS: &str = "V7 | i | V7 | i | iv | i | V7 | i V7 i i";

/// The tune: a rest, three rising eighths, a rest, three falling.
const AMANECER_A_TUNE: &str = "-:2 D5:2 Eb5:2 F5:2 -:2 F5:2 Eb5:2 D5:2 | \
                               -:2 C5:2 D5:2 Eb5:2 -:2 Eb5:2 D5:2 C5:2 | \
                               -:2 D5:2 Eb5:2 F5:2 -:2 F5:2 Eb5:2 D5:2 | \
                               -:2 C5:2 D5:2 Eb5:2 -:2 Eb5:2 D5:2 C5:2 | \
                               -:2 F5:2 G5:2 Ab5:2 -:2 Ab5:2 G5:2 F5:2 | \
                               -:2 Eb5:2 F5:2 G5:2 -:2 G5:2 F5:2 Eb5:2 | \
                               -:2 D5:2 Eb5:2 F5:2 -:2 F5:2 Eb5:2 D5:2 | C5:4 -:12";

const AMANECER_A_END_TUNE: &str = "-:2 D5:2 Eb5:2 F5:2 -:2 F5:2 Eb5:2 D5:2 | \
                                   -:2 C5:2 D5:2 Eb5:2 -:2 Eb5:2 D5:2 C5:2 | \
                                   -:2 D5:2 Eb5:2 F5:2 -:2 F5:2 Eb5:2 D5:2 | \
                                   -:2 C5:2 D5:2 Eb5:2 -:2 Eb5:2 D5:2 C5:2 | \
                                   -:2 F5:2 G5:2 Ab5:2 -:2 Ab5:2 G5:2 F5:2 | \
                                   -:2 Eb5:2 F5:2 G5:2 -:2 G5:2 F5:2 Eb5:2 | \
                                   -:2 D5:2 Eb5:2 F5:2 -:2 F5:2 Eb5:2 D5:2 | C5:4 G5:4 C6:4 -:4";

// Source: https://github.com/baobach/abc_search/blob/master/database/abc/QmQhzsujKd7PVGVaK1fDP9zvd7johEkbHGBns9otY499gW.abc
// (Javier Tucat Moreno's piano arrangement in C minor, www.tangoalpiano.com; the
// tune is the top note of the right hand, and the chords are read from his left
// hand. His eight-bar introduction, the first strain an octave up, "based on
// Beba Pugliese's arrangement", is left out, and four bars he gives to the left
// register are written an octave up, where the lead sheet below has them.)
// Checked against the Wikifonia lead sheet:
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/RobertoFirpoElAmanecer.abc
// (its three strains match Tucat's; its trio keeps the C minor key signature but
// its chords, C and G, agree with Tucat's change to C major. After its final bar
// line it has 22 more bars without chords, an 8-bar slow phrase and 14 bars of
// arpeggios with two empty bars; no second source has them, so they are left out.)
// PD: composed 1910; published in Rio de Janeiro by Futuro das Moças in 1917
// (https://imslp.org/wiki/El_amanecer_(Firpo,_Roberto)), before 1930.
pub(super) const EL_AMANECER: Song = Song {
    name: "El amanecer",
    origin: "Roberto Firpo, 1910",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: C,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new("A", AMANECER_A_TUNE, AMANECER_A_CHORDS),
        Part::new(
            "B",
            "G5:6 F#5:2 G5:6 F#5:2 | G5:4 -:2 F5:2 F5:2 E5:2 F5:4 | F5:6 E5:2 F5:6 G5:2 | \
             F5:4 -:2 Eb5:2 Eb5:2 D5:2 Eb5:4 | G4:2 C5:4 D5:2 Eb5:6 F5:2 | \
             Eb5:4 -:2 D5:2 D5:2 C#5:2 D5:4 | -:4 F5:4 D5:4 G4:4 | Ab4:6 G4:2 G4:2 F#4:2 G4:4 | \
             -:2 G5:4 F#5:2 G5:6 F#5:2 | G5:4 -:2 F5:2 F5:2 E5:2 F5:4 | -:2 F5:4 E5:2 F5:6 G5:2 | \
             F5:4 -:2 Eb5:2 Eb5:2 D5:2 Eb5:4 | -:2 D5:4 Eb5:2 F5:2 Eb5:2 D5:2 C5:2 | \
             B4:4 G4:2 Ab4:2 Bb4:2 Ab4:2 G4:2 F4:2 | -:4 C5:4 -:4 B4:4 | C5:4 -:12",
            "i | V7 | V7 | i | i | i V7 | V7 | i | i | V7 | V7 | i | iiø7 | V7 | i V7 | i",
        ),
        Part::new(
            "C",
            "G4:4 C5:4 -:2 G4:2 A4:2 G4:2 | F4:4 B4:4 D5:4 -:4 | F4:4 B4:4 -:2 F4:2 B4:2 F4:2 | \
             E4:4 G4:4 C5:4 -:4 | G4:4 E5:6 D#5:2 E5:2 F5:2 | E5:6 D5:2 D5:2 C#5:2 D5:4 | \
             F4:4 D5:6 C#5:2 D5:2 E5:2 | D5:2 C5:4 B4:2 C5:4 -:4 | \
             G4:4 C5:4 -:2 G4:2 A4:2 G4:2 | F4:4 B4:4 D5:4 -:4 | F4:4 B4:4 -:2 F4:2 B4:2 F4:2 | \
             E4:4 G4:4 C5:4 -:4 | -:2 Ab4:2 C5:2 Eb5:2 Ab5:4 Ab5:4 | G5:4 -:12 | -:16 | \
             -:4 Eb4:4 D4:4 C4:4",
            "I | V7 | V7 | I | I | V7 | V7 | I | I | V7 | V7 | I | bVI | V7 | V7 | i",
        )
        .in_key(C, Major),
        Part::new("A end", AMANECER_A_END_TUNE, AMANECER_A_END_CHORDS),
    ],
    form: &["A", "B", "C", "A end"],
    drums_from: 0,
    about: "The first strain is one small figure, a rest, three rising eighths, a rest, three \
            falling, moved over G7, C minor and F minor. The second holds long G's over C minor \
            and G7, and the third turns to C major; its last phrase goes from an A flat chord \
            to G7, where the tune rests for a bar, and back to C minor for the first strain.",
};

// Source: https://github.com/syntithenai/abc2book/blob/master/abcresources/jc/abc_tune_jc_201244558.abc
// (John Chambers' ABC in A minor, with chords and the form "A1 B A2 C A2"; A2 is
// his "alternate form of A". One typo, "Z3B" for "z3B", is read as a rest. The
// low E3 in the first strain and the high D6 in A2 are as he writes them. The
// second strain's E minor chord is written E7 where the tune has G sharp. His
// "n.c." (the band stops) after the first beat of bar 15 of A1 and A2 is held as
// A minor, as a chart cannot stop; the A2 bar he marks "often omitted" is kept.)
// PD: dated 1919 in the source; Victor recorded it on 25 August 1920 (matrix
// B-24403, Victor 72805, https://adp.library.ucsb.edu/index.php/matrix/refer/700009617),
// so it was out well before 1930; no dated printed edition was found.
pub(super) const A_LA_GRAN_MUNECA: Song = Song {
    name: "A la gran muñeca",
    origin: "Jesús Ventura, 1919",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new(
            "A1",
            "E4:4 E4:2 F#4:2 G#4:4 G#4:2 A4:2 | B4:4 B4:2 C5:2 D5:2 D5:2 C5:2 B4:2 | \
             A4:4 A4:2 B4:2 C5:4 C5:2 D5:2 | E5:4 F4:2 F4:2 E4:2 E4:2 C4:2 A3:2 | \
             E3:4 E4:2 F#4:2 G#4:4 G#4:2 A4:2 | B4:4 B4:2 C5:2 D5:2 D5:2 C5:2 B4:2 | \
             A4:4 A4:2 B4:2 C5:4 A4:2 B4:2 | C5:4 A4:2 B4:2 C5:2 B4:2 A4:2 B4:2 | \
             A4:4 A4:2 B4:2 C#5:4 C#5:2 D5:2 | E5:4 E5:2 F5:2 G5:2 G5:2 F5:2 E5:2 | \
             D5:4 D5:2 E5:2 F5:4 F5:2 G5:2 | A5:4 F5:2 E5:2 D5:2 E5:2 F5:2 A5:2 | \
             A5:2 E5:2 C5:2 D5:2 E5:2 F5:2 E5:2 F5:2 | E5:2 D5:2 B4:2 C5:2 D5:2 E5:2 C5:2 B4:2 | \
             A4:4 B4:2 C5:2 D5:2 E5:2 C5:2 B4:2 | A4:4 -:8 C5:2 B4:2",
            "V7 | V7 | i | i | V7 | V7 | i | i | I7 | I7 | iv | iv | i | V7 | i | i V7 i i",
        ),
        Part::new(
            "B",
            "B4:4 A4:2 A4:2 A4:4 C5:2 B4:2 | B4:4 A4:8 A4:2 B4:2 | C5:4 C5:2 C5:2 C5:4 D5:2 C5:2 | \
             E5:4 B4:8 B4:2 C5:2 | D5:4 D5:2 C5:2 B4:4 B4:2 A4:2 | A4:4 G#4:8 B4:2 A4:2 | \
             G#4:4 G#4:2 F4:2 E4:4 E4:2 D4:2 | C4:4 -:8 C5:2 B4:2 | \
             B4:4 A4:2 A4:2 A4:4 C5:2 B4:2 | B4:4 A4:8 E5:2 D5:2 | \
             C#5:4 C#5:2 Bb4:2 A4:4 A4:2 G4:2 | F4:4 -:8 D4:2 F4:2 | \
             Bb4:4 Bb4:2 Bb4:2 Bb4:4 C5:2 Bb4:2 | A4:4 E4:4 -:4 E4:2 F4:2 | \
             E4:4 G#4:2 A4:2 B4:4 C5:2 B4:2 | A4:4 -:8 E4:4",
            "i | i | i | v | v | V7 | V7 | i | i | i | I7 | iv | bII | i | V7 | i V7 i i",
        ),
        Part::new(
            "A2",
            "E4:6 G#4:2 B4:6 C5:2 | D5:6 E5:2 F5:6 E5:2 | A5:4 E5:12 | A5:8 -:8 | \
             E4:6 G#4:2 B4:6 C5:2 | D5:6 E5:2 F5:6 E5:2 | A5:4 E5:12 | A5:8 -:8 | \
             E5:6 C#5:1 D5:1 E5:6 F5:2 | G5:6 A5:2 Bb5:6 A5:2 | D6:4 A5:12 | \
             -:4 F5:2 E5:2 D5:2 E5:2 F5:2 A5:2 | A5:2 E5:2 C5:2 D5:2 E5:2 F5:2 E5:2 F5:2 | \
             E5:2 D5:2 B4:2 C5:2 D5:2 E5:2 C5:2 B4:2 | A4:4 B4:2 C5:2 D5:2 E5:2 C5:2 B4:2 | \
             A4:4 -:8 E4:4",
            "V7 | V7 | i | i | V7 | V7 | i | i | I7 | I7 | iv | iv | i | V7 | i | i V7 i i",
        ),
        Part::new(
            "C",
            "C#5:6 C#5:2 C#5:4 D5:2 C#5:2 | E5:8 B4:8 | -:4 B4:2 C#5:2 D5:2 C#5:2 D5:2 E5:2 | \
             D5:8 C#5:8 | -:4 C5:2 B4:2 B4:2 A4:2 G#4:2 A4:2 | C5:8 B4:8 | \
             -:4 B4:2 C5:2 D5:2 D5:2 C5:2 B4:2 | E5:8 -:8 | C#5:6 C#5:2 C#5:4 D5:2 C#5:2 | \
             E5:8 B4:8 | -:6 B4:2 C#5:2 C#5:2 D5:2 E5:2 | E5:8 D5:8 | \
             F4:4 F4:2 F4:2 F4:2 F4:2 G4:2 F4:2 | A4:8 E4:6 F4:2 | E4:4 G#4:2 A4:2 B4:4 C5:2 B4:2 | \
             A4:4 -:8 E4:4",
            "I | V7 | V7 | I | i | v | v | i | I | V7 | I7 | IV | iv | i | V7 | i V7 i i",
        )
        .in_key(A, Major),
        Part::new(
            "A2 fine",
            "E4:6 G#4:2 B4:6 C5:2 | D5:6 E5:2 F5:6 E5:2 | A5:4 E5:12 | A5:8 -:8 | \
             E4:6 G#4:2 B4:6 C5:2 | D5:6 E5:2 F5:6 E5:2 | A5:4 E5:12 | A5:8 -:8 | \
             E5:6 C#5:1 D5:1 E5:6 F5:2 | G5:6 A5:2 Bb5:6 A5:2 | D6:4 A5:12 | \
             -:4 F5:2 E5:2 D5:2 E5:2 F5:2 A5:2 | A5:2 E5:2 C5:2 D5:2 E5:2 F5:2 E5:2 F5:2 | \
             E5:2 D5:2 B4:2 C5:2 D5:2 E5:2 C5:2 B4:2 | A4:4 B4:2 C5:2 D5:2 E5:2 C5:2 B4:2 | \
             A4:4 -:12",
            "V7 | V7 | i | i | V7 | V7 | i | i | I7 | I7 | iv | iv | i | V7 | i | i V7 i i",
        ),
    ],
    form: &["A1", "B", "A2", "C", "A2 fine"],
    drums_from: 0,
    about: "The first strain climbs from E up the A melodic minor scale, with F sharp and G \
            sharp, over E7 and A minor, falls to a low E and climbs again; its middle goes \
            through A7 to D minor. The second strain dips to E minor and E7 and to B flat, the \
            Neapolitan chord of A minor, and the trio turns to A major, slipping back to A \
            minor in its second phrase.",
};

const POR_UNA_CABEZA_A_CHORDS: &str = "I | I ii | ii V7 | V7 I | I | I7 IV | iv I | II7 V7 I I";
const POR_UNA_CABEZA_B_CHORDS: &str = "i v | bVI bIII | iv i";
const POR_UNA_CABEZA_B_TUNE: &str = "-:2 F5:2 F5:2 G5:1 A5:1 A5:4 E5:4 | -:2 D5:2 D5:2 E5:1 F5:1 F5:4 C5:4 | \
     -:2 Bb4:2 Bb4:2 C5:1 D5:1 D5:2 A4:2 D5:1 E5:1 F5:1 D5:1";

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/CarlosGardelAlfredoLeperaPorUnaCabeza.abc
// (Wikifonia lead sheet in A major / A minor with chords, moved up a fourth to D. Its
// ending brackets are misplaced: read as one 16-bar B strain whose first ending goes
// back to the start with the pickup and whose second ending closes the piece.)
// Checked against three arrangements, and where the four differ the reading most of
// them share is used, the lead sheet's otherwise:
// https://github.com/bebox/scifidelity_orchestra/blob/master/source/por_una_cabeza.ly (lead sheet in G),
// https://github.com/cellist/Lilypond-Sheet-Music/blob/master/Vlc,%20Vlc/Gardel,%20Carlos/Por%20Una%20Cabeza/v1.ily (cello duo in G),
// https://github.com/Rilay9/PianoProject/blob/main/docs/review/pdmx-quarry-2026-10-05/xml/E-latin/por-una-cabeza-carlos-gardel-QmNswaWYXpxK1XegKbJVDULwZMjKN6cETGTVfXQKYsYrzs.musicxml (piano, PDMX, in A).
// The pickup is four sixteenths and the first beats of the A strain are two sixteenths
// and a rest (the arrangements). Source bar 8 is F# E C# A A# C# B G, as in the G lead
// sheet and two further PDMX piano scores (QmTBJZ..., QmQSWZ... in the same folder).
// The B strain's rising figures are eighth-note triplets in the arrangements; they are
// written as an eighth and two sixteenths, the rhythm the Wikifonia sheet gives them.
// PD: Carlos Gardel (d. 1935) and Alfredo Le Pera (d. 1935): every author died before 1956.
pub(super) const POR_UNA_CABEZA: Song = Song {
    name: "Por una cabeza",
    origin: "Carlos Gardel, 1935",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new("Pickup", "-:12 F#4:1 G4:1 G#4:1 A4:1", "I"),
        Part::new(
            "A",
            "B4:1 A4:1 -:2 G#4:1 A4:1 B4:1 C#5:1 E5:1 D5:1 -:2 F#5:1 G5:1 E5:1 F#5:1 | \
             D5:1 E5:1 C#5:1 D5:1 C#5:3 A4:1 G4:4 G5:1 F#5:1 A5:1 G5:1 | \
             E5:4 E5:1 D#5:1 F#5:1 E5:1 C#5:4 C#5:1 D5:1 D#5:1 E5:1 | \
             F#5:1 E5:1 C#5:1 A4:1 A#4:1 C#5:1 B4:1 G4:1 F#4:4 F#4:1 G4:1 G#4:1 A4:1 | \
             B4:1 A4:1 -:2 G#4:1 A4:1 B4:1 C#5:1 E5:1 D5:1 -:2 D5:1 E5:1 F#5:1 D5:1 | \
             E5:1 D5:1 E5:1 F#5:1 E5:3 D5:1 B4:4 E5:1 D5:1 B4:1 G4:1 | \
             F#4:1 E4:1 F#4:1 G4:1 Bb4:1 D5:1 C5:1 Bb4:1 D5:2 A4:2 D5:1 E5:1 F#5:1 D5:1 | \
             E5:2 E5:2 C#5:1 D5:1 E5:1 C#5:1 D5:4 -:4",
            POR_UNA_CABEZA_A_CHORDS,
        ),
        Part::new("B", POR_UNA_CABEZA_B_TUNE, POR_UNA_CABEZA_B_CHORDS).in_key(D, HarmonicMinor),
        Part::new(
            "B ending 1",
            "E5:4 D5:1 E5:1 F5:1 D5:1 F5:4 E5:4",
            "II7 II7 vi°7 V7",
        )
        .in_key(D, HarmonicMinor),
        // The second ending lands on D major and plays the pickup back into A.
        Part::new(
            "B ending 2",
            "E5:4 C#5:1 D5:1 E5:1 C#5:1 D5:4 F#4:1 G4:1 G#4:1 A4:1",
            "II7 V7 I I",
        )
        .in_key(D, HarmonicMinor),
        Part::new(
            "B final",
            "E5:4 C#5:1 D5:1 E5:1 C#5:1 D5:6 -:2",
            "II7 V7 i i",
        )
        .in_key(D, HarmonicMinor),
    ],
    form: &[
        "Pickup",
        "A",
        "B",
        "B ending 1",
        "B",
        "B ending 2",
        "A",
        "B",
        "B ending 1",
        "B",
        "B final",
    ],
    drums_from: 1,
    about: "The A strain in D major is all quick chromatic turns over D, E minor and A7, \
            with a D7 that leads to G and a G minor that darkens it. The B strain turns to \
            D minor: one rising figure three times, each a third lower, over D minor and A \
            minor, B flat and F, G minor and D minor; E7 and A7 lead home, and the second \
            ending lands on D major for the A strain again.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/CarlosGardelEldiaquemequieras.abc
// (Wikifonia lead sheet in E flat with chords, voice 1; moved up a fourth to A flat.)
// Checked against https://github.com/maximoskp/chameleon/blob/master/training_data/Tango/El_Dia_Que_Me_Quieras.xml
// (melody and one chord per bar, in D) and the string quartet arranged by Ignacio
// Massun, https://github.com/cellist/Lilypond-Sheet-Music/tree/master/Vl,%20Vl,%20Vlc,%20Vlc/Gardel,%20Carlos/El%20dia%20que%20me%20quieras
// (first violin for the tune, the other three for the chords). Where the three differ
// the reading two of them share is used. The quartet's tempo (4 = 60), fermatas, rall.
// and a tempo marks are kept.
// The verse opens with the A flat that ends the introduction's tune (the quartet's
// second violin and chameleon); source bar 10 is dotted (Wikifonia's c2>B2 and the
// quartet). Source bar 15 is B flat 7 sus 4 then B flat 7 (the quartet; Wikifonia's
// C7/F, F7 in its key): chameleon's F minor there is not used.
// Form: introduction, verse, refrain twice (first and second endings). The refrain's
// lead-in bar makes the 2/4 bars odd in number, so the second time through the refrain
// is paired the other way, and the last written bar is padded with a rest.
// PD: Carlos Gardel (d. 1935) and Alfredo Le Pera (d. 1935): every author died before 1956.
pub(super) const EL_DIA_QUE_ME_QUIERAS: Song = Song {
    name: "El día que me quieras",
    origin: "Carlos Gardel, 1935",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: GSharp,
    mode: Major,
    time: COMMON,
    tempo: 60.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new(
            "Intro",
            "-:2 F5:2 F5:1 E5:1 G5:1 F5:1 F5:4 E5:4 | -:2 Eb5:2 Eb5:1 D5:1 F5:1 Eb5:1 Eb5:4 Db5:4 | \
             -:2 Db5:2 Eb5:1 Db5:1 Ab4:1 Bb4:1 C5:2 Eb5:2 G5:2 F5:2 | \
             Eb5:2 C5:1 Ab4:1 F4:2 Db5:2 Bb4:6 Ab4:2",
            "IV III7 | VI7 ii | IV IV I #iv°7 | I I V7sus4 V7",
        )
        .with_tempo(&[glide(56, 50, 8)]),
        Part::new(
            "Verse",
            "Ab4:2 Eb4:2 Ab4:1 C5:1 Eb5:1 G5:1 F5:3 Eb5:1 Eb5:2 C5:1 Bb4:1 | \
             Bb4:2 Ab4:1 Ab4:1 Ab4:2 F4:1 Ab4:1 Eb5:8 | \
             -:2 Db5:1 Db5:1 Db5:2 C5:1 Db5:1 C5:2 C5:1 Bb4:1 Bb4:2 Ab4:1 F4:1 | \
             C5:2 C5:1 Bb4:1 Ab4:2 E4:1 F4:1 Bb4:8 | \
             -:2 Eb4:2 Ab4:1 C5:1 Eb5:1 G5:1 F5:2 Eb5:1 Eb5:1 Eb5:2 D5:1 C5:1 | \
             Eb5:2 Eb5:1 Eb5:1 Eb5:2 D5:1 C5:1 G5:8 | \
             -:2 G5:1 Ab5:1 G5:2 B4:1 C5:1 G5:4 F5:4 | -:2 F5:1 G5:1 F5:2 A4:1 Bb4:1 F5:4 E5:4",
            "I I I iii | vi V | V #v°7 vi vi | II7sus4 II7 V7 V7 | V I | VI7 #i°7 V V | III7 VI7 | \
             II7 II7 V7 bVII7",
        )
        .with_tempo(&[tempo(0, 60)]),
        Part::new(
            "Refrain",
            "Eb5:2 Eb4:2 C5:1 C5:1 C5:1 Bb4:1 Db5:2 C5:6 | _:2 C5:2 C5:1 Bb4:1 Bb4:1 C5:1 Bb4:2 Ab4:6 | \
             _:2 Ab4:2 Ab4:1 Ab4:1 Ab4:1 Ab4:1 F5:4 F5:4 | _:2 F5:2 F5:1 Eb5:1 Gb5:1 F5:1 F5:8 | \
             -:2 F5:2 F5:1 Eb5:1 A4:1 Bb4:1 F5:4 Eb5:4 | -:2 Db5:2 Db5:1 C5:1 F#4:1 G4:1 Bb4:4 Ab4:4 | \
             _:2 Ab4:2 Bb4:1 Ab4:1 G4:1 Ab4:1 C5:4 C5:4 | _:2 Ab4:2 Bb4:1 Ab4:1 G4:1 Ab4:1 Eb5:8",
            "V7 I | III7 vi | I7 IV | VI7 ii | ii V7 | #v°7 vi | vi iii | vi II7 V7 V7",
        )
        .with_tempo(&[fermata(0, 2), glide(120, 50, 8)]),
        Part::new(
            "Refrain 2",
            "Eb5:2 Eb4:2 C5:1 C5:1 C5:1 Bb4:1 Db5:2 C5:6 | _:2 C5:2 C5:1 Bb4:1 Bb4:1 C5:1 Bb4:2 Ab4:6 | \
             -:2 Ab4:2 Ab4:1 Ab4:1 Ab4:1 Ab4:1 F5:4 F5:4 | _:2 F5:2 F5:1 Eb5:1 Gb5:1 F5:1 F5:8 | \
             _:2 F5:2 F5:1 E5:1 G5:1 F5:1 F5:4 E5:4 | -:2 Eb5:2 Eb5:1 D5:1 F5:1 Eb5:1 Eb5:4 Db5:4 | \
             -:2 Db5:2 Eb5:1 Db5:1 Ab4:1 Bb4:1 C5:2 Eb5:2 G5:2 F5:2 | \
             Eb5:2 C5:1 Ab4:1 F4:2 Db5:2 Bb4:6 Ab4:2",
            "V7 I | III7 vi | I7 IV | VI7 ii | ii IV III7 III7 | VI7 ii | iv iv I #iv°7 | I V7",
        )
        .with_tempo(&[tempo(4, 60), glide(104, 50, 12), tempo(116, 60)]),
        Part::new("Ending 1", "Ab4:4 -:4 -:2 Eb4:2 C5:1 C5:1 C5:1 Bb4:1", "I #iv°7 V7 V7"),
        Part::new(
            "Refrain again",
            "Db5:2 C5:6 _:2 C5:2 C5:1 Bb4:1 Bb4:1 C5:1 | Bb4:2 Ab4:6 _:2 Ab4:2 Ab4:1 Ab4:1 Ab4:1 Ab4:1 | \
             F5:4 F5:4 _:2 F5:2 F5:1 Eb5:1 Gb5:1 F5:1 | F5:8 -:2 F5:2 F5:1 Eb5:1 A4:1 Bb4:1 | \
             F5:4 Eb5:4 -:2 Db5:2 Db5:1 C5:1 F#4:1 G4:1 | Bb4:4 Ab4:4 _:2 Ab4:2 Bb4:1 Ab4:1 G4:1 Ab4:1 | \
             C5:4 C5:4 _:2 Ab4:2 Bb4:1 Ab4:1 G4:1 Ab4:1 | Eb5:8 _:2 Eb4:2 C5:1 C5:1 C5:1 Bb4:1",
            "I III7 | vi I7 | IV VI7 | ii | V7 #v°7 | vi | iii iii vi II7 | V7",
        )
        .with_tempo(&[glide(112, 50, 8), tempo(124, 60)]),
        Part::new(
            "Refrain again 2",
            "Db5:2 C5:6 _:2 C5:2 C5:1 Bb4:1 Bb4:1 C5:1 | Bb4:2 Ab4:6 -:2 Ab4:2 Ab4:1 Ab4:1 Ab4:1 Ab4:1 | \
             F5:4 F5:4 _:2 F5:2 F5:1 Eb5:1 Gb5:1 F5:1 | F5:8 _:2 F5:2 F5:1 E5:1 G5:1 F5:1 | \
             F5:4 E5:4 -:2 Eb5:2 Eb5:1 D5:1 F5:1 Eb5:1 | Eb5:4 Db5:4 -:2 Db5:2 Eb5:1 Db5:1 Ab4:1 Bb4:1 | \
             C5:2 Eb5:2 G5:2 F5:2 Eb5:2 C5:1 Ab4:1 F4:2 Db5:2 | Bb4:6 Ab4:2 Ab4:8 | _:8 -:8",
            "I III7 | vi I7 | IV VI7 | ii ii ii IV | III7 VI7 | ii iv | I #iv°7 I I | V7 V7 I iv | I",
        )
        .with_tempo(&[glide(96, 50, 12), tempo(108, 60), fermata(128, 8)]),
    ],
    form: &[
        "Intro",
        "Verse",
        "Refrain",
        "Refrain 2",
        "Ending 1",
        "Refrain again",
        "Refrain again 2",
    ],
    drums_from: 0,
    about: "A slow tango-canción in A flat. The refrain walks through secondary dominants, \
            C7 to F minor, A flat 7 to D flat, F7 to B flat minor, with diminished sevenths \
            on D and E along the way; at its close the D flat chord turns minor before the \
            last cadence, and the introduction is that closing phrase.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/CarlosGardelVolverTango.abc
// (Wikifonia lead sheet transcribed by Alberto Betancourt: verse in D minor, refrain in
// D major, with chords; moved up a fourth to G. The same sheet as MusicXML,
// https://raw.githubusercontent.com/ptigas/markovjam/master/dataset/4782.xml, gives the same
// notes.) No second transcription was found. Its form: verse, refrain, D.C. al Coda, the
// second refrain leaving its last bar for the coda.
// In refrain bar 3-4 the sheet's chords (D minor over F, D minor over A) contradict its
// own F sharps; the chords of the same tune in bars 16-17 (D over F sharp, D over A) are
// used. A C7 on the last eighth of verse bar 4 and the B flat 7 / A7 of verse bar 19 are
// kept eighth by eighth.
// PD: Carlos Gardel (d. 1935) and Alfredo Le Pera (d. 1935): every author died before 1956.
pub(super) const VOLVER: Song = Song {
    name: "Volver",
    origin: "Carlos Gardel, 1934",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: G,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 62.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        Part::new(
            "Verse",
            "-:2 D5:1 D5:1 D5:1 C5:1 Bb4:1 C5:1 D5:1 D5:1 D5:1 D5:1 D5:1 C5:1 Bb4:1 D5:1 | \
             C5:1 C5:1 C5:1 C5:1 C5:1 Bb4:1 A4:1 C5:1 Bb4:4 G4:2 -:2 | \
             -:2 F5:1 F5:1 F5:1 Eb5:1 D5:1 Eb5:1 F5:1 F5:1 F5:1 F5:1 F5:1 Eb5:1 D5:1 F5:1 | \
             Eb5:1 Eb5:1 Eb5:1 Eb5:1 Eb5:1 D5:1 C5:1 Eb5:1 D5:6 -:2 | \
             -:1 D5:1 D5:1 D5:1 F#5:1 D5:2 C5:1 C5:2 Bb4:2 -:1 Bb4:1 C5:1 C#5:1 | \
             D5:2 C5:1 Bb4:1 A4:2 G4:1 Bb4:1 D5:4 -:1 D5:1 D5:1 D5:1 | \
             Eb5:3 D5:1 C5:1 A5:1 G5:1 F#5:1 G5:2 D5:2 -:1 D5:1 Eb5:1 D5:1 | \
             C#5:1 E5:2 G5:1 Bb5:2 A5:1 G5:1 A5:4 -:1 A5:1 A5:1 A5:1 | \
             Bb5:3 F5:1 Eb5:1 F5:1 G5:1 Eb5:1 G5:1 D5:2 Bb4:1 C5:1 D5:1 Eb5:1 F5:1 | \
             D5:1 Bb4:1 G4:1 D4:1 Bb4:2 A4:2 G4:6 B4:2",
            "i | iv6 iv6 V7 V7 i i i bVII7 | bIII | bVI6 bVII7 bIII bIII | V7 i | bIII bVI7 V V | \
             iv i | II7 V | i iiø7 i iv | i i bVI7 V7 i i i i",
        ),
        Part::new(
            "Refrain",
            "D5:4 _:4 -:2 C5:1 D5:1 E5:2 D5:1 C5:1 | D5:1 B4:2 B4:1 C5:2 B4:1 A4:1 B4:1 G4:2 G4:1 A4:2 G4:1 F#4:1 | \
             G4:6 -:1 E5:1 G5:8 | _:2 F#5:1 G5:1 A5:2 G5:1 F#5:1 G5:4 D5:4 | \
             -:2 D5:1 E5:1 F5:2 E5:1 D5:1 F5:1 E5:1 E5:1 F#5:1 G5:2 F#5:1 E5:1 | \
             G5:1 F#5:2 F#5:1 G5:2 E5:1 E5:1 F#5:2 D5:1 D5:1 E5:2 C5:1 C5:1 | \
             D5:4 B4:2 -:1 B4:1 D5:8 | _:2 C5:1 D5:1 E5:2 D5:1 C5:1 D5:1 B4:2 B4:1 C5:2 B4:1 A4:1",
            "I #i°7 V7 V7 | I I bVI6 bVI6 I I II7 V7 | I7 IV | iv° I | #i°7 #i°7 ii biii°7 | \
             iii iii vi ii V I IV V7 | I #i°7 V7 V7 | V7 V7 I bVI6",
        )
        .in_key(G, Major),
        Part::new(
            "Refrain end",
            "B4:1 G4:2 G4:1 A4:2 G4:1 F#4:1 G4:6 -:2",
            "I I II7 V7 I I I I",
        )
        .in_key(G, Major),
        Part::new(
            "Coda",
            "B4:1 G4:2 G4:1 A4:2 G4:1 F#4:1 G4:4 G5:2 -:2",
            "I I II7 V7 I I I I",
        )
        .in_key(G, Major),
    ],
    form: &["Verse", "Refrain", "Refrain end", "Verse", "Refrain", "Coda"],
    drums_from: 0,
    about: "The verse in G minor repeats one note in quick even notes, then the note a step \
            below: D over G minor, C over C minor, then F over B flat and E flat over E flat; \
            D7 and A7 lift it to a high cadence on D. The refrain turns to G major and borrows \
            from G minor: an E flat sixth chord, and a C diminished chord after C major.",
};

/// Eight bars of D minor over a bass walking down D, C, B, B flat, A, then the
/// answer through D7, G minor and C7 to F major and back on A7.
const ROMANCE_A1_CHORDS: &str = "i | i7 | II7 | II7 | bVI7 | V7 | i i II7 | i | I7 | I7 | iv | \
                                 iv iv vii°7 | bVII7 | bVII7 | bIII | V7";

/// The same opening, then F7 over E flat sliding onto D7, G minor, C7 and F to A7 and a
/// cadence in D minor.
const ROMANCE_A2_CHORDS: &str = "i | i7 | II7 | II7 | bVI7 | V7 | i i II7 | bIII7 I7 I7 | iv | bVII7 | \
                                 bIII | V7 | i | V7 | i | i";

/// A sequence climbing a step every two bars: G minor, A7, D minor (with A half-diminished
/// over C leading on), G minor, C7, F.
const ROMANCE_B1_CHORDS: &str =
    "iv6 | V7 | i | i i vø7 | iv | bVII7 | bIII | bIII | V7 | V7 | i | i | bVI | iv | V7 | V7";

/// The sequence again, falling back to D minor over E7 and A7.
const ROMANCE_B2_CHORDS: &str =
    "iv6 | V7 | i | i7 | II7 | V7 | i | I7 | iv | V7 | i | i7 | II7 | V7";

// Source: score attached by the owner (two page images of a voice and piano
// edition, "Romance de barrio, vals", words by Homero Manzi, music by Aníbal
// Troilo, in D minor). The tune is the voice part; the chords were read from the
// piano part and checked against the guitar chart at
// https://acordes.lacuerda.net/anibal_troilo/romance_de_barrio (in A minor).
// The edition plays the verse and the second part, then goes back to the segno
// for the verse's second words ("Ceniza del tiempo...") and the second part
// again, ending at the second ending (FIN).
// Best-known recording: Troilo's orchestra with Floreal Ruiz, 1947
// (https://en.wikipedia.org/wiki/Floreal_Ruiz); the tempo is an estimate of a
// Troilo vals.
// Copyright: Troilo died in 1975 and Manzi in 1951; the vals is from 1947 and not
// public domain. Added at the owner's request.
pub(super) const ROMANCE_DE_BARRIO: Song = Song {
    name: "Romance de barrio",
    origin: "Aníbal Troilo, 1947",
    feel: "Vals criollo",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 180.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Tango vals",
    band: TANGO,
    style: "Vals in minor",
    parts: &[
        Part::new("Pickup", "-:10 A4:2", "i"),
        Part::new(
            "A1",
            "A4:4 Ab4 G4 | F4:4 E4:6 D4:2 | F4:4 E4 -:2 D4:2 | E4:8 -:2 E4:2 | \
             F4:4 E4 -:2 D4:2 | E4:8 -:2 E4:2 | Bb4:4 A4:6 G#4:2 | A4:8 -:2 D5:2 | \
             D5:4 A4 Bb4 | C5:4 Bb4 F#4 | A4:4 G4:6 A4:2 | Bb4:8 -:2 A4:2 | \
             A4:4 G4:6 A4:2 | Bb4:8 -:2 A4:2 | G4:4 F4:6 G4:2 | A4:8 -:2 A4:2",
            ROMANCE_A1_CHORDS,
        ),
        Part::new(
            "A2",
            "A4:4 Ab4 G4 | F4:4 E4:6 D4:2 | F4:4 E4 -:2 D4:2 | E4:8 -:2 E4:2 | \
             F4:4 E4 -:2 D4:2 | E4:8 -:2 E4:2 | Bb4:4 A4 -:2 G#4:2 | A4:8 -:2 G4:2 | \
             G4:4 A4 Bb4 | C5:4 D5 -:2 E5:2 | F5:8 C5:4 | -:2 C#5:2 D5 C#5 B4 C#5 | \
             D5:8 A4:4 | -:2 Bb4:2 G4:4 -:2 E4:2 | D4:12 | _:4 -:8",
            ROMANCE_A2_CHORDS,
        ),
        Part::new(
            "B1",
            "G4:4 E4:2 F4 G4:4 | E4:2 F4 G4 E4 F4 G4 | A4:4 F4:2 G4 A4:4 | \
             F4:2 G4 A4 F4 G4 A4 | Bb4:4 G4:2 A4 Bb4:4 | G4:2 A4 Bb4 G4 A4 Bb4 | \
             C5:4 A4:2 Bb4 C5:4 | _:4 -:8 | C#5:4 A4:2 B4 C#5:4 | A4:2 B4 C#5 A4 B4 C#5 | \
             D5:12 | A4:12 | Bb4:4 C5 Bb4 | F4:4 G4 Bb4 | A4:12 | _:8 -:4",
            ROMANCE_B1_CHORDS,
        ),
        Part::new(
            "B2",
            "G4:4 E4:2 F4 G4:4 | E4:2 F4 G4 E4 F4 G4 | F4:4 D4:2 E4 F4:4 | \
             D4:2 E4 F4 D4 E4 F4 | E4:4 C#4:2 D4 E4:4 | C#4:2 D4 E4 C#4 D4 E4 | \
             D4:6 C#4:2 D4 F4 | A4:12 | G4:4 E4:2 F4 G4:4 | E4:2 F4 G4 E4 F4 G4 | \
             F4:4 D4:2 E4 F4:4 | D4:2 E4 F4 D4 E4 F4 | E4:4 C#4:2 D4 E4:4 | \
             C#4:2 D4 E4 C#4 F4 E4",
            ROMANCE_B2_CHORDS,
        ),
        Part::new("First ending", "D4:12 | _:4 -:6 A4:2", "i | i"),
        Part::new("Second ending", "D4:4 -:8 | -:12", "i i V7 | i"),
    ],
    form: &[
        "Pickup",
        "A1",
        "A2",
        "B1",
        "B2",
        "First ending",
        "A1",
        "A2",
        "B1",
        "B2",
        "Second ending",
    ],
    drums_from: 1,
    about: "Troilo's vals opens on a chromatic sigh, A, A flat, G, over D minor, while the \
            bass walks down D, C, B, B flat to A: the same notes then lean on E7 and B flat 7 \
            before A7 brings the cadence. The second part climbs in a sequence, a step higher \
            every two bars: G minor, A7, D minor, G minor, then C7 to F major, the relative \
            major, until C sharp turns it back to D minor.",
};

// Source: score attached by the owner (two page images): "Desde el
// alma", Vals, Letra: Homero Manzi y Víctor Velez, Música: Rosita Melo; voice and piano, 72 bars,
// key signature of two sharps, TRIO in one sharp, "D.C." at the end with no Fine and no segno.
// Melody = the vocal line; chords read from the piano's bass and right-hand chords bar by bar.
// The first strain (bars 1-32) is in B minor, the second (33-48) in D major, the trio (49-72) in
// G major ending on B minor. The score prints "D.C." at bar 72 with no Fine and no segno, so
// where the da capo stops is not written. The form goes back to the start and stops at bar 32,
// the double bar where the first strain closes on B minor, the tonic (the integrator's choice;
// the other double bar, bar 48 before "TRIO", would end in D major).
// Recording: the best-known vocal version is Nelly Omar's (Odeon, 22 October 1947, named one of
// her "remarkable hits" in https://www.todotango.com/english/artists/biography/291/Nelly-Omar/;
// that page does not name the accompaniment). Roberto Firpo's orchestra recorded the waltz first
// (https://todotango.com/english/history/chronicle/183/Desde-el-alma-Desde-el-alma-a-romantic-criollo-waltz).
// The tempo is an estimate for a sung vals criollo, not measured from a recording.
// Copyright: Rosita Melo died in 1981 and the score's words are Manzi's (1948): not public domain; added at the owner's request.

/// The first strain: F#7 for two bars, then B minor; E minor in its third phrase.
const DESDE_A_CHORDS: &str =
    "V7 | V7 | i | i | V7 | V7 | i | i i I7 | iv | iv | i | i | V7 | V7 | i | V7";
const DESDE_A2_CHORDS: &str =
    "V7 | V7 | i | i | V7 | V7 | i | i i I7 | iv | iv | i | i | V7 | V7 | i | i";
/// The second strain, in D major: A7 (over A, then over C sharp) to D, B7 over D sharp, A7 over
/// E and A, and D.
const DESDE_B_CHORDS: &str = "V7 | V7 | I | I I VI7 | V7 | V7 | I | I | \
                              V7 | V7 | I | I I VI7 | V7 | V7 | I | I";
const DESDE_TRIO_CHORDS: &str =
    "I | I | V | V7 | I | I | V7 | V7 | I | I7 | IV | IV | VI | VI7 | V | V7";
const DESDE_TRIO_END_CHORDS: &str = "III7 | III7 | VIsus4 | vi | VII7 | VII7 | iii | iii";

pub(super) const DESDE_EL_ALMA: Song = Song {
    name: "Desde el alma",
    origin: "Rosita Melo, 1911",
    feel: "Vals criollo",
    genre: Genre::Tango,
    tonic: B,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 156.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Tango vals",
    band: TANGO,
    style: "Tango vals",
    parts: &[
        // Bars 1-16 ("Alma, si tanto te han herido...").
        Part::new(
            "A",
            "F#4:8 F#4:4 | -:2 F#4:2 G#4 A#4 G#4 F#4 | B4:12 | B4:8 B4:4 | C#5:12 | \
             -:2 C#5:2 D5 E5 D5 C#5 | D5:8 D5:4 | D5:4 C#5:6 B4:2 | B4:8 B4:4 | \
             -:2 B4:2 C#5 D5 C#5 B4 | B4:8 B4:4 | -:2 F#4:2 G4 A4 G4 F#4 | F#4:8 F#4:4 | \
             -:2 F#4:2 G4 A4 G4 F#4 | F#4:12 | _:8 -:4",
            DESDE_A_CHORDS,
        ),
        // Bars 17-32 ("Vives inútilmente triste..."), ending on B minor before a double bar.
        Part::new(
            "A2",
            "F#4:8 F#4:4 | -:2 F#4:2 G#4 A#4 G#4 F#4 | B4:12 | B4:8 B4:4 | C#5:12 | \
             -:2 C#5:2 D5 E5 D5 C#5 | D5:8 D5:4 | D5:4 C#5:6 B4:2 | B4:8 B4:4 | \
             -:2 B4:2 C#5 D5 C#5 B4 | B4:8 B4:4 | -:2 F#4:2 G4 A4 G4 F#4 | F#4:8 F#4:4 | \
             A#4:8 A#4:4 | B4:12 | _:4 -:8",
            DESDE_A2_CHORDS,
        ),
        // Bars 33-48 ("Fue lo que empezó una vez..."), in the relative major.
        Part::new(
            "B",
            "G4:12 | -:2 F#4:2 E4 D#4 E4 E#4 | F#4:12 | -:2 B4:2 D5:6 B4:2 | C#5:12 | \
             -:2 A4:2 C#5:6 A4:2 | B4:12 | -:2 E4:2 G4:6 E4:2 | G4:12 | \
             -:2 F#4:2 E4 D#4 E4 E#4 | F#4:12 | -:2 B4:2 D5:6 B4:2 | C#5:8 A4:4 | \
             A4:4 G4 C#4 | D4:12 | _:4 -:8",
            DESDE_B_CHORDS,
        )
        .in_key(D, Major),
        // Bars 49-64 ("¡Deja esas cartas!...").
        Part::new(
            "Trio",
            "D5:12 | _:4 E5 F#5 | D5:12 | C5:12 | B4:4 D5 G5 | A5:4 G5:6 E5:2 | F#5:12 | _:12 | \
             G5:12 | F5:8 G5:4 | E5:12 | _:12 | E5:12 | D5:8 E5:4 | D5:12 | C5:12",
            DESDE_TRIO_CHORDS,
        )
        .in_key(G, Major),
        // Bars 65-72 ("...llega la vida trayendo otro amor"), closing on B minor before the D.C.
        Part::new(
            "Trio end",
            "B4:12 | A4:8 B4:4 | A4:12 | G4:12 | F#4:12 | G4:4 F#4 C#5 | B4:12 | _:8 -:4",
            DESDE_TRIO_END_CHORDS,
        )
        .in_key(G, Major),
    ],
    form: &["A", "A2", "B", "Trio", "Trio end", "A", "A2"],
    drums_from: 0,
    about: "A vals that starts on the dominant, F#7, and only lands on B minor in its third bar. \
            The second strain turns to the relative major, D, through A7, with C# in the bass under a \
            chromatic E, D#, E, E# in the tune. The trio moves to G major, touches C through G7, \
            and closes on B minor through F#7 over A#; the da capo then ends the vals on B minor.",
};

/// A7, then A7 with its flat ninth, D minor with a ninth on top; D minor and A7,
/// A7 and D minor, D minor and G minor, and A7 home to D minor.
const MUCHACHO_INTRO_CHORDS: &str = "V7 V7b9 i9 i9 | i V7 | V7 i | i iv | V7 i";

/// Two bars of A7 under "Muchacho", then D minor and A7, D minor (over a moving
/// bass) and G minor, and the same again; the middle visits F major (F, C7, C7, F)
/// before A7 brings it back to D minor.
const MUCHACHO_CANTO_CHORDS: &str = "V7 | i V7 | i iv | i V7 | i V7 | V7 i | V7 i | iv i | V7 i | \
                                     bIII bVII7 | bVII7 bIII | V7 V7 V7b9 V7 | V7 i | V7 | V7 i";

// Source: score attached by the owner (three page images of a typeset edition for
// voice and piano, "Muchacho, tango", "Celedonio Flores" / "Edgardo Donato", in D
// minor and 2/4: "INTROD." of ten bars, then "CANTO" with a repeat, first and
// second endings and two verses of words; no publisher or date printed). The tune
// is the vocal line, which the piano's right hand plays; in the intro it is the top
// of the right hand, written at its sounding pitch where the score has 8va (bars
// 3-6 and the chord on the second beat of bars 10, 28, 42 and 43). Where the
// canto's right hand plays the tune in octaves filled out with chords (bars 29-30
// and 33-34) the tune is written in the singer's octave, the lower one. The chords
// were read from the piano's left hand and inner voices. Bar 1's two 32nd notes
// C sharp, D are one sixteenth C sharp. In bar 9 the score prints a sharp before the
// first F and no natural later in the bar; the other Fs are written F natural, as
// in the same figure in bar 41, where the score does print the natural. In bar 7
// the top voice is A repeated over an inner voice F F F E D D E; the top is kept.
// Bar 15 (and 24) is charted D minor throughout: the left hand only runs a bass
// line D C sharp D B flat | C A B flat under the held A. The four bars of F major
// (29-32) are a tonicization of the relative major, not a new key: no `.in_key`.
// A well-known recording: Ángel D'Agostino's orchestra with Ángel Vargas, 1940
// (Victor 39136, https://adp.library.ucsb.edu/index.php/matrix/refer/200038642);
// the tempo is an estimate of a D'Agostino tango.
// Public domain in Argentina is not claimed; published by 1925 (recorded by Rosita Quiroga, Victor, 1925); added at the owner's request with the score they attached.
pub(super) const MUCHACHO: Song = Song {
    name: "Muchacho",
    origin: "Edgardo Donato, 1923",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 124.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        // It ends with the pickup "Mu-" into the canto.
        Part::new(
            "Intro",
            "-:1 C#5:2 E5:1 G5:1 Bb5:2 C#6:1 F6:2 C6:2 _:4 | \
             -:1 F6:1 F6 E6 E6 D6 D6 E6 D6:2 C#6:2 _:4 | \
             -:1 G6:1 G6 F6 F6 E6 E6 F6 E6:2 D6:2 _:4 | \
             -:1 A5:1 A5 A5 A5 A5 A5 A5 A5:2 G5:4 -:2 | \
             A4:1 G4 G4 F#4 G4 F4 E4 F4 D4:4 D6:2 -:1 A4:1",
            MUCHACHO_INTRO_CHORDS,
        ),
        Part::new(
            "Canto",
            "A4:2 G4:2 _:4 -:1 G4:1 Bb4 G4 Bb4 A4:2 G4:1 | \
             Bb4:2 A4:1 A4 C5 F4 F4 F4 G4 G4 Bb4 A4 C5 Bb4 A4 G#4 | \
             A4:6 -:2 -:1 G4:1 G4 G4 Bb4 A4 Bb4 A4 | \
             G4:1 F4 F4 F4 A4 G#4 A4 F4 G4 E4 E4 F4 G4 Bb4 A4 G4 | \
             A4:6 -:1 A4:1 A4:2 G4:2 _:4 | \
             -:1 G4:1 Bb4 G4 Bb4 A4:2 G4:1 Bb4:2 A4:1 A4 C5 F4 F4 F4 | \
             G4:1 G4 Bb4 A4 C5 Bb4 A4 G#4 A4:6 -:2 | \
             -:1 G4:1 G4 G4 G4:1 G4:2 G4:1 G4:1 F4 F4 F4 A4:1 F4:2 F4:1 | \
             G4:1 E4 E4 E4 D4 C#4 D4 E4 D4:4 D6:2 -:2 | \
             -:1 C5:2 C5:1 C5 C5 C5 C5 D5:4 C5:4 | \
             -:1 C5:1 E5 C5 C5:3 Bb4:1 Bb4:4 A4:4 | \
             -:1 A4:1 A4 A4 Bb4:3 A4:1 C#5:4 A4:4 | \
             -:1 G4:2 G4:1 G4:1 E4 F4 G4 A4:6 -:2 | \
             -:1 E4:1 F4 F#4 G4:1 A4:2 G#4:1 A4:4 G4:4 | \
             -:2 G4:1 G4 Bb4 A4 A4 G4 G4 F4 D4 E4 F4 G4 A4 Bb4",
            MUCHACHO_CANTO_CHORDS,
        ),
        // It ends with the pickup "De-" into the second verse.
        Part::new(
            "First ending",
            "A4:1 G4 G4 F#4 G4 F4 E4 F4 D4:4 D6:2 -:1 A4:1",
            "V7 i",
        ),
        Part::new(
            "Second ending",
            "A4:1 G4 G4 F#4 G4 F4 E4 F4 D4:4 D6:2 -:2",
            "V7 i",
        ),
    ],
    form: &["Intro", "Canto", "First ending", "Canto", "Second ending"],
    drums_from: 0,
    about: "Donato's tango lives on A7 and D minor: \"Muchacho\" is sung on A and G over A7, \
            and the next line leans on B flat, the flat ninth of A7, and on G sharp before \
            each return to A. In the middle the tune visits F major, the relative \
            major (F, C7, C7, F), then A7 with its B flat brings it back to D minor; the intro and each \
            half of the canto end on D with the tango's short, high chord.",
};

/// Verse: two-bar phrases, A7 to D minor, then C7 to F, then E7 and A7.
const GARUA_VERSE_CHORDS: &str = "V7 i | V7 i | bVII7 bIII | bVII7 bIII | II7 V7 | \
                                  i i bVI V7 V7 V7 V7 V7 | V7 i | V7 i | II7 V7 | \
                                  i i bVI V7 i i i i | i";

const GARUA_VERSE_TUNE: &str = "-:1 E4:1 E4:1 E4:1 G4:2 C#4:1 C#4:1 E4:2 D4:1 F4:1 E4:2 D4:2 | \
     -:1 E4:1 E4:1 E4:1 G4:2 C#4:1 C#4:1 E4:2 D4:1 F4:1 E4:2 D4:2 | \
     -:1 G4:1 G4:1 G4:1 Bb4:2 E4:1 E4:1 G4:2 F4:1 A4:1 G4:2 F4:2 | \
     -:1 G4:1 G4:1 G4:1 Bb4:2 E4:1 E4:1 G4:2 F4:1 A4:1 G4:2 F4:2 | \
     -:1 G#4:2 A4:1 B4:1 C#5:1 D5:1 E5:1 C#5:1 A4:1 Bb4:1 G4:1 A4:1 F4:1 G4:1 E4:1 | \
     F4:1 D4:1 E4:1 C#4:1 D4:3 G4:1 A3:4 -:4 | \
     -:1 E4:1 E4:1 E4:1 G4:2 C#4:1 C#4:1 E4:2 D4:1 F4:1 E4:2 D4:2 | \
     -:1 E4:1 E4:1 E4:1 G4:2 C#4:1 C#4:1 E4:2 D4:1 F4:1 E4:2 D4:2 | \
     -:1 G#4:2 A4:1 B4:1 C#5:1 D5:1 E5:1 C#5:1 A4:1 Bb4:1 G4:1 A4:1 F4:1 G4:1 E4:1 | \
     F4:1 D4:1 D4:1 E4:1 F4:2 E4:2 D4:1 D5:2 E5:1 D5:2 -:2 | \
     -:8 -:1 D5:2 E5:1 D5:2 -:1 A4:1";

/// Refrain: D minor over a bass falling D, C, B, B flat; B flat 7 and A7; D7 to G minor.
const GARUA_REFRAIN_CHORDS: &str =
    "i | i i7 IV7 iv | bVI7 V7 V7 V7 | V7 | V7 i | i I7 | I7 iv | V7 i";

const GARUA_REFRAIN_TUNE: &str = "A4:4 D4:4 -:2 A4:1 D5:1 D5:1 C#5:1 C5:1 B4:1 | \
     Bb4:1 A4:1 F4:1 A4:1 A4:1 G#4:1 G4:1 F#4:1 F4:1 E4:1 E4:1 D4:1 D4:1 D4:1 C#4:1 D4:1 | \
     F4:4 E4:3 Bb4:1 _:4 E4:3 G4:1 | \
     G4:4 C#4:4 -:2 Bb3:1 F4:1 D4:1 C#4:1 F4:1 E4:1 | \
     Bb4:1 A4:1 A4:1 Ab4:1 G4:1 Gb4:1 F4:1 A4:1 E4:4 D4:2 -:1 D5:1 | \
     _:4 A4:2 -:2 -:2 D4:1 E4:1 F#4:1 G4:1 A4:1 Bb4:1 | \
     C5:1 B4:1 Bb4:1 A4:1 Eb5:1 D5:1 D5:1 C5:1 C5:4 Bb4:2 -:1 Bb4:1 | \
     _:4 E4:3 A4:1 A4:4 D4:2 D4:1 E4:1";

// Source: score attached by the owner (page images), "Garúa... Tango", letra de Enrique
// Cadícamo, música de Aníbal Troilo (Pichuco), Editorial Julio Korn, Buenos Aires,
// copyright MCMXLIII (1943); piano solo with the sung tune in the right hand and, for
// seven bars of the refrain, a "Canto" staff above the piano.
// The tune is the "Canto" staff where there is one (refrain bars 1-7), else the top
// note of the right hand, the "Orquesta" bars included. The chords are read from the
// left hand and the inner voices. The 2/4 bars are written two to a bar.
// Form as printed: a one-bar introduction, segno, verse (21 bars), refrain (17 bars),
// first ending "para seguir... al segno", verse again, refrain, second ending "para FIN".
// The verse has an odd number of 2/4 bars: its last one, the orchestra's second bar
// with the "Ga-" of the refrain, is written alone, with two beats of rest added before it.
// Grace notes are left out. Refrain bar 39 is shared by both endings; the first ending
// (bar 40) is the introduction's figure with the tune's D on its first beat, and the
// second (bar 41) ends D, A over A7, D over D minor. In verse bar 12 the turn E, C sharp (two 32nd notes) after
// a D eighth is written as four sixteenths, F D E C sharp.
// The best-known recording is Aníbal Troilo y su Orquesta Típica with Francisco
// Fiorentino, Victor, 1943; the tempo is an estimate of a 1940s Troilo dance tempo, not
// measured from the record.
// Copyright: Troilo died in 1975; not public domain; added at the owner's request.
pub(super) const GARUA: Song = Song {
    name: "Garúa",
    origin: "Aníbal Troilo, 1943",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        // The piano's introduction: D minor 7 (F, C, A over D), A minor over its C, then
        // B flat, a bass falling D, C, B flat to the A of the verse's first bar.
        Part::new("Pickup", "-:8 -:2 F4:2 E4:2 D4:2", "i i i i i i v bVI"),
        Part::new("Verse", GARUA_VERSE_TUNE, GARUA_VERSE_CHORDS),
        Part::new("Refrain", GARUA_REFRAIN_TUNE, GARUA_REFRAIN_CHORDS),
        // The refrain's last bar: G sharp diminished 7 (G sharp in the bass under D and
        // F, E7 without its root) to A7. The first ending then plays the introduction's
        // chords again, D minor 7, A minor, B flat, back to the segno for the second verse.
        Part::new(
            "Refrain ending 1",
            "F4:2 E4:1 D4:1 E4:2 D4:1 C#4:1 D4:2 F4:2 E4:2 D4:2",
            "#iv°7 #iv°7 V7 V7 i i v bVI",
        ),
        Part::new(
            "Refrain ending 2",
            "F4:2 E4:1 D4:1 E4:2 D4:1 C#4:1 D4:2 A4:2 D5:2 -:2",
            "#iv°7 #iv°7 V7 V7 i V7 i i",
        ),
    ],
    form: &[
        "Pickup",
        "Verse",
        "Refrain",
        "Refrain ending 1",
        "Verse",
        "Refrain",
        "Refrain ending 2",
    ],
    drums_from: 1,
    about: "The verse in D minor answers A7 with D minor, then C7 with F, and climbs over \
            E7 to A7, whose bass walks down A, G, F, E, D, C, B flat. The refrain sings a \
            chromatic line over D minor while the bass falls D, C, B, B flat under G7 and \
            G minor, and B flat 7 slides down a half step to A7; later D7 leads to G minor, \
            and G sharp diminished 7 and A7 close in D minor.",
};

// The verse, in B flat major (it opens on F7 and every phrase closes on B flat; G minor
// is only touched as vi): 20 bars of 2/4, two to a chart bar.
const ESTA_NOCHE_DE_LUNA_A_CHORDS: &str = "V7 I | III7 vi | ii #ii°7 I #i°7 | V7 V7 I #ii°7 | \
     V7 I | III7 vi | ii #ii°7 I #i°7 | V7 I | V7 I | V7 I";
const ESTA_NOCHE_DE_LUNA_A_TUNE: &str = "F4:2 F4:1 G4:1 A4:1 Bb4:1 C5:1 C#5:1 D5:3 Bb4:1 D5:2 C5:1 Bb4:1 | \
     D5:1 A4:2 Bb4:1 C5:2 Bb4:1 A4:1 D5:3 G4:1 Bb4:2 A4:1 G4:1 | \
     Bb4:3 G4:1 Bb4:2 A4:1 G4:1 F4:3 F4:1 G4:2 F4:1 E4:1 | \
     F4:2 Eb4:1 D4:1 Eb4:2 D4:1 C4:1 F4:2 -:1 F4:1 G4:2 F4:1 E4:1 | \
     F4:2 F4:1 G4:1 A4:1 Bb4:1 C5:1 C#5:1 D5:3 Bb4:1 D5:2 C5:1 Bb4:1 | \
     D5:1 A4:2 Bb4:1 C5:2 Bb4:1 A4:1 D5:3 G4:1 Bb4:2 A4:1 G4:1 | \
     Bb4:3 G4:1 Bb4:2 A4:1 G4:1 F4:3 F4:1 G4:2 F4:1 E4:1 | \
     F4:2 Eb4:1 D4:1 Eb4:2 D4:1 C4:1 Bb3:2 Bb5:1 Bb5:1 A5:1 A5:1 G5:1 G5:1 | \
     F5:1 F5:1 Eb5:1 Eb5:1 D5:1 D5:1 C5:1 C5:1 Bb4:2 Bb5:1 Bb5:1 A5:1 A5:1 G5:1 G5:1 | \
     F5:1 F5:1 Eb5:1 Eb5:1 D5:1 D5:1 C5:1 C5:1 Bb4:2 -:6";

// The refrain, also in B flat major: 28 bars of 2/4. Its last bar lands on B flat and
// plays the pickup back into the verse.
const ESTA_NOCHE_DE_LUNA_B_CHORDS: &str = "I ii | I V7 | ii6 vi | III7 | III7 III7 vi iv6 | I II7 | \
     V7 #ii°7 I I | ii I | V7 ii6 | vi III7 | III7 | ii iv6 I I | I ii | V7 V7 I #ii°7";
const ESTA_NOCHE_DE_LUNA_B_TUNE: &str = "D5:8 _:2 Bb4:1 C5:1 D5:2 C5:1 Bb4:1 | F5:8 _:2 C5:1 D5:1 Eb5:2 D5:1 C5:1 | \
     G4:8 G4:2 G4:1 A4:1 Bb4:2 C5:1 C#5:1 | D5:4 A4:4 _:2 A4:2 Bb4:2 A4:2 | \
     C5:4 A4:4 _:2 Eb4:1 F4:1 Ab4:2 C5:1 Bb4:1 | \
     Bb4:1 F4:1 Bb4:1 C5:1 D5:2 C5:1 Bb4:1 D5:1 C5:1 C5:1 D5:1 C5:2 Bb4:1 G4:1 | \
     C5:4 C#5:4 D5:8 | _:2 Bb4:1 C5:1 D5:2 C5:1 Bb4:1 F5:8 | \
     _:2 C5:1 D5:1 Eb5:2 D5:1 C5:1 G4:8 | G4:2 G4:1 A4:1 Bb4:2 C5:1 C#5:1 D5:4 A4:4 | \
     _:2 A4:2 Bb4:2 A4:2 C5:4 A4:4 | G4:2 Eb4:1 F4:1 Gb4:2 Bb4:1 A4:1 Bb4:2 -:2 F4:4 | \
     _:2 D4:1 Eb4:1 F4:2 Eb4:1 D4:1 G4:2 Eb4:1 F4:1 G4:2 F4:1 Eb4:1 | \
     A4:2 A4:1 Bb4:1 C5:2 Bb4:1 A4:1 Bb4:2 -:1 F4:1 G4:2 F4:1 E4:1";

// Source: score attached by the owner (page images), piano with the lyric between the
// staves, "Letra de Héctor Marcó, Música de Graciano Gómez y José García", stamped
// "Texto aprobado por Radiocomunicaciones"; no publisher or plate number shows on the
// two pages. Two flats, read in B flat major as printed (no transposition).
// The runs of sixteenths under the "8a" lines (intro and "Corre, corre barcarola") are
// written at their printed pitch, an octave below where the piano plays them, so the
// whole tune stays between B flat 3 and C6.
// The ⊕ signs only mark where the violin obbligato at the foot of page 2 plays ("violín
// para Canto de ⊕ a ⊕"); it is left out. The sung order is the lyric blocks I, II, I bis:
// intro, verse, refrain, verse again, ending on the verse's last B flat.
// Recordings (Todotango): José García y sus Zorros Grises with Alfredo Rojas (August
// 1943) and Carlos Di Sarli with Roberto Rufino (December 1943), taken as the best known (a judgement, no source ranks them).
// The tempo is an estimate for the Di Sarli recording, not a measured figure.
// Copyright: not public domain; added at the owner's request.
pub(super) const ESTA_NOCHE_DE_LUNA: Song = Song {
    name: "Esta noche de luna",
    origin: "Graciano Gómez and José García, 1943",
    feel: "Tango",
    genre: Genre::Tango,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Quarters,
    bass: Bass::Quarters,
    rhythm: "Tango marcato",
    band: TANGO,
    style: "Tango",
    parts: &[
        // Half a bar of silence, then the first run of the piano's intro.
        Part::new("Pickup", "-:8 -:2 Bb5:1 Bb5:1 A5:1 A5:1 G5:1 G5:1", "I"),
        Part::new(
            "Intro",
            "F5:1 F5:1 Eb5:1 Eb5:1 D5:1 D5:1 C5:1 C5:1 Bb4:2 Bb5:1 Bb5:1 A5:1 A5:1 G5:1 G5:1 | \
             F5:1 F5:1 Eb5:1 Eb5:1 D5:1 D5:1 C5:1 C5:1 Bb4:2 -:1 F4:1 G4:2 F4:1 E4:1",
            "V7 I | V7 V7 I #ii°7",
        ),
        Part::new("A", ESTA_NOCHE_DE_LUNA_A_TUNE, ESTA_NOCHE_DE_LUNA_A_CHORDS),
        Part::new("B", ESTA_NOCHE_DE_LUNA_B_TUNE, ESTA_NOCHE_DE_LUNA_B_CHORDS),
    ],
    form: &["Pickup", "Intro", "A", "B", "A"],
    drums_from: 1,
    about: "The whole song is in B flat major. The verse starts on the dominant, F7 to \
            B flat, then D7 turns to G minor; a C minor chord climbs through a C sharp \
            diminished chord to B flat over F, a B diminished chord leads to F7 and home \
            to B flat, and the intro's falling runs of sixteenths close it. The refrain \
            holds long notes on \"Soy\" and \"mar\" over B flat, a D7 turns towards \
            G minor, and twice an E flat minor sixth over G flat, the minor plagal colour \
            of the tango, leads back to B flat. Then the verse comes back to end the song.",
};
