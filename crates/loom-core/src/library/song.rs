//! Well-known songs in the public domain, with their melodies.
//!
//! Every song here is old enough to belong to everyone: folk songs
//! nobody owns, and pieces by composers who died long ago. The melodies
//! are written in the notation of [`chart::parse_melody`] and the chords
//! as chord charts, one bar at a time, exactly like a lead sheet. Each
//! song is dressed in a style (a house track, a swing band, lo-fi...)
//! by its comping, bass and rhythm, the same building blocks the
//! harmony styles use.

use crate::harmony::{Scale, ScaleKind};
use crate::meter::TimeSignature;
use crate::pitch::PitchClass;

use super::chart::{self, ChordSpan, Melody};
use super::comping::{Bass, Comping};

/// One section of a song, such as a verse or a chorus. Each part
/// becomes one pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub name: &'static str,
    /// The tune, played by the Lead channel.
    pub melody: &'static str,
    /// The chords, accompanied in the song's comping and bass style.
    pub chords: &'static str,
    /// Notes for the Keys channel written out in full, for pieces whose
    /// accompaniment is part of the music. When there are any, the
    /// chords are not accompanied.
    pub left: &'static str,
    /// Where copies of the melody start, in bars from the start of the
    /// part. One voice is `[0]`. In a round, each voice is another copy
    /// a few bars later. A negative start is a voice that began in the
    /// part before: only its end is heard. Notes outside the part are
    /// left out.
    pub voices: &'static [i32],
}

impl Part {
    /// A part with one voice and no written-out accompaniment.
    const fn new(name: &'static str, melody: &'static str, chords: &'static str) -> Part {
        Part {
            name,
            melody,
            chords,
            left: "",
            voices: &[0],
        }
    }

    const fn with_left(self, left: &'static str) -> Part {
        Part { left, ..self }
    }

    const fn with_voices(self, voices: &'static [i32]) -> Part {
        Part { voices, ..self }
    }

    pub fn melody(&self) -> Melody {
        chart::parse_melody(self.melody).unwrap_or(Melody {
            notes: Vec::new(),
            bars: Vec::new(),
            length: 0,
        })
    }

    pub fn left(&self) -> Melody {
        chart::parse_melody(self.left).unwrap_or(Melody {
            notes: Vec::new(),
            bars: Vec::new(),
            length: 0,
        })
    }

    pub fn spans(&self, bar: usize) -> Vec<ChordSpan> {
        chart::parse_chart(self.chords, bar).unwrap_or_default()
    }

    /// How many steps the part lasts: the longest of its melody, its
    /// chords and its written-out accompaniment.
    pub fn length(&self, bar: usize) -> usize {
        let chords = if self.chords.trim().is_empty() {
            0
        } else {
            self.chords.split('|').count() * bar
        };
        self.melody().length.max(self.left().length).max(chords)
    }
}

/// A song, its parts, and how they are put together.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Song {
    pub name: &'static str,
    /// Who wrote it and when, or where it comes from.
    pub origin: &'static str,
    /// The style it is dressed in here, such as "Swing" or "Lo-fi".
    pub feel: &'static str,
    pub tonic: PitchClass,
    pub mode: ScaleKind,
    pub time: TimeSignature,
    pub tempo: f32,
    pub comping: Comping,
    pub bass: Bass,
    /// A rhythm from [`super::RHYTHMS`], or "" for no drums.
    pub rhythm: &'static str,
    pub parts: &'static [Part],
    /// The order the parts are played in, by name.
    pub form: &'static [&'static str],
    /// The drums come in at this entry of `form` (0 for the start).
    pub drums_from: usize,
    pub about: &'static str,
}

impl Song {
    pub fn scale(&self) -> Scale {
        Scale::new(self.tonic, self.mode)
    }

    pub fn part(&self, name: &str) -> Option<&'static Part> {
        self.parts.iter().find(|part| part.name == name)
    }

    /// The song called `name`, if there is one.
    pub fn named(name: &str) -> Option<&'static Song> {
        SONGS.iter().find(|song| song.name == name)
    }
}

use PitchClass::{A, B, C, D, F, G};
use ScaleKind::{Dorian, HarmonicMinor, Major, NaturalMinor};

const COMMON: TimeSignature = TimeSignature::COMMON;
const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
const THREE_EIGHT: TimeSignature = TimeSignature::new(3, 8);

// ----- Ode to Joy -----

const ODE_TO_JOY: &[Part] = &[Part::new(
    "Theme",
    "F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | F#5:6 E5:2 E5:8 | \
     F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | E5:6 D5:2 D5:8 | \
     E5:4 E5 F#5 D5 | E5 F#5:2 G5 F#5:4 D5 | E5 F#5:2 G5 F#5:4 E5 | D5 E5 A4:8 | \
     F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | E5:6 D5:2 D5:8",
    "I | V | I | I V | I | V | I | V I | V I | V I | V I | I V | I | V | I | V I",
)];

// ----- Twinkle, Twinkle, Little Star -----

const TWINKLE: &[Part] = &[Part::new(
    "Tune",
    "C5:4 C5 G5 G5 | A5 A5 G5:8 | F5:4 F5 E5 E5 | D5 D5 C5:8 | \
     G5:4 G5 F5 F5 | E5 E5 D5:8 | G5:4 G5 F5 F5 | E5 E5 D5:8 | \
     C5:4 C5 G5 G5 | A5 A5 G5:8 | F5:4 F5 E5 E5 | D5 D5 C5:8",
    "Imaj7 | IVmaj7 Imaj7 | IVmaj7 Imaj7 | V7 Imaj7 | Imaj7 IVmaj7 | Imaj7 V7 | \
     Imaj7 IVmaj7 | Imaj7 V7 | Imaj7 | IVmaj7 Imaj7 | IVmaj7 Imaj7 | V7 Imaj7",
)];

// ----- Frère Jacques -----

const FRERE_JACQUES_TUNE: &str = "C5:4 D5 E5 C5 | C5 D5 E5 C5 | E5 F5 G5:8 | E5:4 F5 G5:8 | \
     G5:2 A5 G5 F5 E5:4 C5 | G5:2 A5 G5 F5 E5:4 C5 | C5 G4 C5:8 | C5:4 G4 C5:8";
const FRERE_JACQUES_CHORDS: &str = "I | I | I | I | I | I | I | I";

const FRERE_JACQUES: &[Part] = &[
    Part::new("Alone", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS),
    Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[0, 2, 4, 6]),
    // Voices 2 to 4 carry on from the part before, and start again.
    Part::new("Round again", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS)
        .with_voices(&[0, -6, 2, -4, 4, -2, 6]),
    // The first voice has finished; the others sing their last bars.
    Part::new("Last voices", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[-6, -4, -2]),
];

// ----- Jingle Bells -----

const JINGLE_BELLS: &[Part] = &[
    Part::new(
        "Verse",
        "D5:4 B5 A5 G5 | D5:12 D5:2 D5 | D5:4 B5 A5 G5 | E5:16 | \
         E5:4 C6 B5 A5 | F#5:16 | D6:4 D6 C6 A5 | B5:16 | \
         D5:4 B5 A5 G5 | D5:16 | D5:4 B5 A5 G5 | E5:12 E5:4 | \
         E5:4 C6 B5 A5 | D6 D6 D6 D6 | E6 D6 C6 A5 | G5:16",
        "I | I | I | IV | IV | V | V7 | I | I | I | I | IV | IV | V | V7 | I",
    ),
    Part::new(
        "Chorus",
        "B5:4 B5 B5:8 | B5:4 B5 B5:8 | B5:4 D6 G5:6 A5:2 | B5:16 | \
         C6:4 C6 C6:6 C6:2 | C6:4 B5 B5 B5:2 B5 | B5:4 A5 A5 B5 | A5:8 D6:8 | \
         B5:4 B5 B5:8 | B5:4 B5 B5:8 | B5:4 D6 G5:6 A5:2 | B5:16 | \
         C6:4 C6 C6:6 C6:2 | C6:4 B5 B5 B5:2 B5 | D6:4 D6 C6 A5 | G5:16",
        "I | I | I | I | IV | I | II7 | V7 | I | I | I | I | IV | I | V7 | I",
    ),
];

// ----- Canon in D -----

const CANON_TUNE: &str = "F#5:4 E5 D5 C#5 | B4 A4 B4 C#5 | D5 C#5 B4 A4 | G4 F#4 G4 E4 | \
     D5:2 F#5 A5 G5 F#5 D5 F#5 E5 | D5 B4 D5 A4 G4 B4 A4 G4";
const CANON_GROUND: &str = "I V vi iii | IV I IV V";
const CANON_CHORDS: &str =
    "I V vi iii | IV I IV V | I V vi iii | IV I IV V | I V vi iii | IV I IV V";

const CANON: &[Part] = &[
    Part::new("Ground", "-:16 | -:16", CANON_GROUND),
    // Each violin starts two bars after the one before.
    Part::new("Entries", CANON_TUNE, CANON_CHORDS).with_voices(&[0, 2, 4]),
    Part::new("Canon", CANON_TUNE, CANON_CHORDS).with_voices(&[0, -4, 2, -2, 4]),
    Part::new("Last entries", CANON_TUNE, CANON_CHORDS).with_voices(&[-4, -2]),
];

// ----- Für Elise -----

const FUR_ELISE: &[Part] = &[
    Part::new("Pickup", "-:4 E5:1 D#5", "").with_left("-:6"),
    Part::new(
        "A",
        "E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 G#4 B4 | C5:2 -:1 E4 E5 D#5 | \
         E5:1 D#5 E5 B4 D5 C5 | A4:2 -:1 C4 E4 A4 | B4:2 -:1 E4 C5 B4 | A4:2 -:2 E5:1 D#5",
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
         -:1 E5 E6 -:2 D#5:1 | E5:1 -:2 D#5:1 E5 D#5",
        "",
    )
    .with_left("C3:1 G3 C4 -:3 | G2:1 G3 B3 -:3 | A2:1 E3 A3 -:3 | E2:1 E3 -:4 | -:6 | -:6"),
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
];

// ----- Greensleeves -----

const GREENSLEEVES: &[Part] = &[
    Part::new("Intro", "-:12 | -:8 A4:4", "i | V"),
    Part::new(
        "Verse",
        "C5:8 D5:4 | E5:6 F5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
         C5:8 A4:4 | A4:6 G#4:2 A4:4 | B4:8 G#4:4 | E4:8 A4:4 | \
         C5:8 D5:4 | E5:6 F5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
         C5:6 B4:2 A4:4 | G#4:6 F#4:2 G#4:4 | A4:12 | _:12",
        "i | bIII | bVII | v | i | V | V | i | i | bIII | bVII | v | i | V | i | i",
    ),
    Part::new(
        "Chorus",
        "G5:12 | G5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
         C5:8 A4:4 | A4:6 G#4:2 A4:4 | B4:8 G#4:4 | E4:12 | \
         G5:12 | G5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | \
         C5:6 B4:2 A4:4 | G#4:6 F#4:2 G#4:4 | A4:12 | _:8 A4:4",
        "bIII | bVII | bVII | v | i | V | V | i | bIII | bVII | bVII | v | i | V | i | i",
    ),
];

// ----- Amazing Grace -----

const AMAZING_GRACE: &[Part] = &[
    Part::new("Intro", "-:12 | -:8 D5:4", "I | V"),
    Part::new(
        "Verse",
        "G5:8 B5:2 G5:2 | B5:8 A5:4 | G5:8 E5:4 | D5:8 D5:4 | \
         G5:8 B5:2 G5:2 | B5:8 A5:4 | D6:12 | _:8 B5:4 | \
         D6:8 B5:2 G5:2 | B5:8 A5:4 | G5:8 E5:4 | D5:8 D5:4 | \
         G5:8 B5:2 G5:2 | B5:8 A5:4 | G5:12 | _:8 D5:4",
        "I | I | IV | I | I | I I V | V | V V I | I | I | IV | I | I | I I V | I | I",
    ),
];

// ----- When the Saints Go Marching In -----

const SAINTS: &[Part] = &[Part::new(
    "Chorus",
    "-:4 C5:4 E5 F5 | G5:16 | -:4 C5:4 E5 F5 | G5:16 | \
     -:4 C5:4 E5 F5 | G5:8 E5:8 | C5:8 E5:8 | D5:16 | \
     -:4 E5:4 E5 D5 | C5:12 C5:4 | E5:8 G5:8 | G5:8 F5:8 | \
     -:4 E5:4 F5 G5 | E5:8 C5:8 | D5:16 | C5:16",
    "I | I | I | I | I | I | I | V7 | I7 | IV | I | V7 | I | I | V7 | I",
)];

// ----- Oh! Susanna -----

const OH_SUSANNA: &[Part] = &[
    Part::new("Intro", "-:16 | -:12 C5:2 D5", "I | V7"),
    Part::new(
        "Song",
        "E5:2 G5 G5 A5 G5 E5 C5 D5 | E5 E5 D5 C5 D5:4 C5:2 D5 | \
         E5 G5 G5 A5 G5 E5 C5 D5 | E5 E5 D5 D5 C5:8 | \
         F5:4 F5 A5:6 A5:2 | G5 G5 E5 C5 D5:4 C5:2 D5 | \
         E5 G5 G5 A5 G5 E5 C5 D5 | E5 E5 D5 D5 C5:4 C5:2 D5",
        "I | I V | I | V7 I | IV | I V | I | V7 I",
    ),
];

// ----- Auld Lang Syne -----

const AULD_LANG_SYNE: &[Part] = &[
    Part::new("Intro", "-:16 | -:12 C5:4", "I | V7"),
    Part::new(
        "Verse",
        "F5:6 F5:2 F5:4 A5:4 | G5:6 F5:2 G5:4 A5:4 | F5:6 F5:2 A5:4 C6:4 | D6:12 D6:4 | \
         C6:6 A5:2 A5:4 F5:4 | G5:6 F5:2 G5:4 A5:2 G5:2 | F5:6 D5:2 D5:4 C5:4 | F5:12 D6:4",
        "I | V7 | I | IV | I | V7 | IV V7 | I",
    ),
    Part::new(
        "Chorus",
        "C6:6 A5:2 A5:4 F5:4 | G5:6 F5:2 G5:4 D6:4 | C6:6 A5:2 A5:4 C6:4 | D6:12 F6:4 | \
         C6:6 A5:2 A5:4 F5:4 | G5:6 F5:2 G5:4 A5:2 G5:2 | F5:6 D5:2 D5:4 C5:4 | F5:12 C5:4",
        "I | V7 | I | IV | I | V7 | IV V7 | I",
    ),
];

// ----- Korobeiniki -----

const KOROBEINIKI: &[Part] = &[
    Part::new(
        "A",
        "E5:4 B4:2 C5 D5:4 C5:2 B4 | A4:4 A4:2 C5 E5:4 D5:2 C5 | B4:6 C5:2 D5:4 E5 | C5 A4 A4:8 | \
         -:2 D5:4 F5:2 A5:4 G5:2 F5 | E5:6 C5:2 E5:4 D5:2 C5 | B4:4 B4:2 C5 D5:4 E5 | C5 A4 A4:8",
        "V | i | V | i | iv | bIII | V | i",
    ),
    Part::new(
        "B",
        "E5:8 C5:8 | D5:8 B4:8 | C5:8 A4:8 | G#4:8 B4:8 | \
         E5:8 C5:8 | D5:8 B4:8 | C5:4 E5:4 A5:8 | G#5:16",
        "i | V | i | V | i | V | i | V",
    ),
];

// ----- In the Hall of the Mountain King -----

const MOUNTAIN_KING: &[Part] = &[Part::new(
    "Theme",
    "B4:2 C#5 D5 E5 F#5 D5 F#5:4 | F5:2 C#5 F5:4 E5:2 C5 E5:4 | \
     B4:2 C#5 D5 E5 F#5 D5 F#5 B5 | A5 F#5 D5 F#5 A5:8 | \
     B4:2 C#5 D5 E5 F#5 D5 F#5:4 | F5:2 C#5 F5:4 E5:2 C5 E5:4 | \
     B4:2 C#5 D5 E5 F#5 D5 F#5 B5 | A5 F#5 D5 F#5 A5:8",
    "i | II7 bII | i | bIII | i | II7 bII | i | bIII",
)];

// ----- Minuet in G -----

const MINUET: &[Part] = &[
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
        "I | V | I | II | II | II7 | vii | V | I | vi | I | V | V | ii | I V | I",
    ),
];

// ----- Gymnopédie No. 1 -----

const GYMNOPEDIE: &[Part] = &[
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
];

// ----- Drunken Sailor -----

const DRUNKEN_SAILOR: &[Part] = &[
    Part::new(
        "Verse",
        "A5:4 A5:2 A5 A5:4 A5:2 A5 | A5:4 D5 F5 A5 | G5:4 G5:2 G5 G5:4 G5:2 G5 | G5:4 C5 E5 G5 | \
         A5:4 A5:2 A5 A5:4 A5:2 A5 | A5:4 B5 C6 D6 | C6 A5 G5 E5 | D5:8 D5:8",
        "i | i | bVII | bVII | i | i | bVII | i",
    ),
    Part::new(
        "Chorus",
        "A5:8 A5:8 | A5:4 D5 F5 A5 | G5:8 G5:8 | G5:4 C5 E5 G5 | \
         A5:8 A5:8 | A5:4 B5 C6 D6 | C6 A5 G5 E5 | D5:8 D5:8",
        "i | i | bVII | bVII | i | i | bVII | i",
    ),
];

/// The songs, in the order the Demos menu lists them.
pub static SONGS: &[Song] = &[
    Song {
        name: "Ode to Joy",
        origin: "Ludwig van Beethoven, 1824",
        feel: "House",
        tonic: D,
        mode: Major,
        time: COMMON,
        tempo: 124.0,
        comping: Comping::Stabs,
        bass: Bass::Offbeat,
        rhythm: "Four on the floor",
        parts: ODE_TO_JOY,
        form: &["Theme", "Theme", "Theme"],
        drums_from: 1,
        about: "The tune of the last movement of Beethoven's Ninth Symphony, over a house beat. \
                It moves almost only by step, which is why anyone can sing it.",
    },
    Song {
        name: "Twinkle, Twinkle, Little Star",
        origin: "French folk song, 1761",
        feel: "Lo-fi",
        tonic: C,
        mode: Major,
        time: COMMON,
        tempo: 80.0,
        comping: Comping::Lazy,
        bass: Bass::Lazy,
        rhythm: "Lo-fi",
        parts: TWINKLE,
        form: &["Tune", "Tune"],
        drums_from: 0,
        about: "The French tune \"Ah ! vous dirai-je, maman\", which Mozart wrote variations on. \
                Major seventh chords turn a nursery rhyme into lo-fi.",
    },
    Song {
        name: "Frère Jacques",
        origin: "French round, 18th century",
        feel: "Round",
        tonic: C,
        mode: Major,
        time: COMMON,
        tempo: 104.0,
        comping: Comping::ArpUpDown,
        bass: Bass::RootFifth,
        rhythm: "Folk",
        parts: FRERE_JACQUES,
        form: &["Alone", "Round", "Round again", "Last voices"],
        drums_from: 1,
        about: "A round: four voices sing the same tune, each two bars after the last. It works \
                because every two bars of the tune fit the same chord.",
    },
    Song {
        name: "Jingle Bells",
        origin: "James Lord Pierpont, 1857",
        feel: "Swing",
        tonic: G,
        mode: Major,
        time: COMMON,
        tempo: 150.0,
        comping: Comping::Charleston,
        bass: Bass::Walking,
        rhythm: "Swing",
        parts: JINGLE_BELLS,
        form: &["Verse", "Chorus", "Chorus"],
        drums_from: 0,
        about: "In the chorus, A7 is a secondary dominant (the V of V): it leads to D7, which \
                leads home to G.",
    },
    Song {
        name: "Canon in D",
        origin: "Johann Pachelbel, around 1700",
        feel: "Baroque",
        tonic: D,
        mode: Major,
        time: COMMON,
        tempo: 64.0,
        comping: Comping::Held,
        bass: Bass::Held,
        rhythm: "",
        parts: CANON,
        form: &["Ground", "Entries", "Canon", "Canon", "Last entries"],
        drums_from: 0,
        about: "Three violins play the same line, each two bars behind the last, over a bass of \
                eight notes that repeats from start to end: a ground bass.",
    },
    Song {
        name: "Für Elise",
        origin: "Ludwig van Beethoven, 1810",
        feel: "Piano",
        tonic: A,
        mode: HarmonicMinor,
        time: THREE_EIGHT,
        tempo: 66.0,
        comping: Comping::Held,
        bass: Bass::None,
        rhythm: "",
        parts: FUR_ELISE,
        form: &["Pickup", "A", "A into B", "B", "A", "A to the end"],
        drums_from: 0,
        about: "The right hand rocks between E and D sharp, the leading note of A minor, while \
                the left hand answers with broken chords. In 3/8: three eighth notes to the bar.",
    },
    Song {
        name: "Greensleeves",
        origin: "English folk song, 16th century",
        feel: "Folk ballad",
        tonic: A,
        mode: NaturalMinor,
        time: THREE_FOUR,
        tempo: 96.0,
        comping: Comping::Arp3,
        bass: Bass::Waltz,
        rhythm: "",
        parts: GREENSLEEVES,
        form: &["Intro", "Verse", "Chorus", "Verse", "Chorus"],
        drums_from: 0,
        about: "A minor key that borrows a major V (E major) to lead home, in a lilting 3/4. \
                Listen for the A held over E in the sixth bar, resolving to G sharp.",
    },
    Song {
        name: "Amazing Grace",
        origin: "Hymn tune \"New Britain\", 1835",
        feel: "Gospel waltz",
        tonic: G,
        mode: Major,
        time: THREE_FOUR,
        tempo: 76.0,
        comping: Comping::Waltz,
        bass: Bass::Waltz,
        rhythm: "Waltz",
        parts: AMAZING_GRACE,
        form: &["Intro", "Verse", "Verse", "Verse"],
        drums_from: 2,
        about: "The whole melody uses five notes, G A B D E: the major pentatonic scale, which \
                is why it sounds so open.",
    },
    Song {
        name: "When the Saints Go Marching In",
        origin: "American spiritual, traditional",
        feel: "Dixieland",
        tonic: C,
        mode: Major,
        time: COMMON,
        tempo: 168.0,
        comping: Comping::Quarters,
        bass: Bass::RootFifth,
        rhythm: "Swing",
        parts: SAINTS,
        form: &["Chorus", "Chorus", "Chorus"],
        drums_from: 1,
        about: "The New Orleans brass band classic. The tune starts with only four notes, C E F \
                G, and the tuba plays two beats to the bar.",
    },
    Song {
        name: "Oh! Susanna",
        origin: "Stephen Foster, 1848",
        feel: "Country",
        tonic: C,
        mode: Major,
        time: COMMON,
        tempo: 120.0,
        comping: Comping::Strum,
        bass: Bass::RootFifth,
        rhythm: "Two-step",
        parts: OH_SUSANNA,
        form: &["Intro", "Song", "Song", "Song"],
        drums_from: 1,
        about: "Each line asks and answers: the first stops on D over the V chord (a half \
                cadence), the second lands on C, at home (a full cadence).",
    },
    Song {
        name: "Auld Lang Syne",
        origin: "Scottish folk song, words by Robert Burns, 1788",
        feel: "Ballad",
        tonic: F,
        mode: Major,
        time: COMMON,
        tempo: 72.0,
        comping: Comping::Ballad,
        bass: Bass::Held,
        rhythm: "Pop ballad",
        parts: AULD_LANG_SYNE,
        form: &["Intro", "Verse", "Chorus", "Verse", "Chorus"],
        drums_from: 3,
        about: "Sung at New Year all over the world. Like many Scottish tunes, it uses only the \
                five notes of the pentatonic scale: F G A C D.",
    },
    Song {
        name: "Korobeiniki",
        origin: "Russian folk song, 1861",
        feel: "Synthwave",
        tonic: A,
        mode: HarmonicMinor,
        time: COMMON,
        tempo: 132.0,
        comping: Comping::Arp16,
        bass: Bass::Octaves16,
        rhythm: "Synthwave",
        parts: KOROBEINIKI,
        form: &["A", "A", "B", "A"],
        drums_from: 1,
        about: "\"The Peddlers\", a song about travelling merchants, as 1980s synth music. The \
                minor tune leans on E major, the V chord of harmonic minor.",
    },
    Song {
        name: "In the Hall of the Mountain King",
        origin: "Edvard Grieg, Peer Gynt, 1875",
        feel: "Epic",
        tonic: B,
        mode: NaturalMinor,
        time: COMMON,
        tempo: 132.0,
        comping: Comping::Pulse,
        bass: Bass::Pulse,
        rhythm: "Epic toms",
        parts: MOUNTAIN_KING,
        form: &["Theme", "Theme", "Theme", "Theme"],
        drums_from: 1,
        about: "Trolls creeping up the B minor scale. The second bar slips through two chords \
                from outside the key, C sharp major and C major. In the original the music keeps \
                getting faster; try raising the tempo while it plays.",
    },
    Song {
        name: "Minuet in G",
        origin: "Christian Petzold, about 1725",
        feel: "Classical",
        tonic: G,
        mode: Major,
        time: THREE_FOUR,
        tempo: 112.0,
        comping: Comping::Waltz,
        bass: Bass::Waltz,
        rhythm: "",
        parts: MINUET,
        form: &["A", "A", "B", "B"],
        drums_from: 0,
        about: "From Anna Magdalena Bach's notebook, long thought to be by Bach himself. The B \
                part travels to D major, the key of the dominant, before coming home.",
    },
    Song {
        name: "Gymnopédie No. 1",
        origin: "Erik Satie, 1888",
        feel: "Lo-fi waltz",
        tonic: D,
        mode: Major,
        time: THREE_FOUR,
        tempo: 72.0,
        comping: Comping::Sway,
        bass: Bass::Waltz,
        rhythm: "Jazz waltz",
        parts: GYMNOPEDIE,
        form: &["Intro", "Melody", "Melody"],
        drums_from: 2,
        about: "Satie's slow dance rocks between two major seventh chords, Gmaj7 and Dmaj7, for \
                the whole piece.",
    },
    Song {
        name: "Drunken Sailor",
        origin: "Sea shanty, traditional",
        feel: "Folk punk",
        tonic: D,
        mode: Dorian,
        time: COMMON,
        tempo: 150.0,
        comping: Comping::Pulse,
        bass: Bass::Pulse,
        rhythm: "Punk",
        parts: DRUNKEN_SAILOR,
        form: &["Verse", "Chorus", "Verse", "Chorus"],
        drums_from: 1,
        about: "Sailors sang shanties to pull ropes together. This one is Dorian: minor, but \
                with a B natural, the raised sixth, in the sixth bar.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::RHYTHMS;
    use crate::pattern::MAX_STEPS;
    use std::collections::HashSet;

    #[test]
    fn every_bar_of_every_part_adds_up() {
        for song in SONGS {
            let bar = song.time.steps_per_bar();
            for part in song.parts {
                let what = format!("{}, {}", song.name, part.name);
                for text in [part.melody, part.left] {
                    if text.is_empty() {
                        continue;
                    }
                    let melody = chart::parse_melody(text)
                        .unwrap_or_else(|| panic!("{what}: cannot read {text}"));
                    for (index, length) in melody.bars.iter().enumerate() {
                        assert_eq!(*length, bar, "{what}: bar {} has {length} steps", index + 1);
                    }
                }
                assert!(
                    chart::parse_chart(part.chords, bar).is_some(),
                    "{what}: cannot read the chords"
                );
                let length = part.length(bar);
                assert!(length > 0 && length <= MAX_STEPS, "{what}: {length} steps");
                assert_eq!(part.melody().length, length, "{what}: melody");
                if !part.chords.trim().is_empty() {
                    assert_eq!(
                        part.chords.split('|').count() * bar,
                        length,
                        "{what}: chords"
                    );
                }
                if !part.left.is_empty() {
                    assert_eq!(part.left().length, length, "{what}: left hand");
                }
            }
        }
    }

    #[test]
    fn forms_name_real_parts_and_rhythms_exist() {
        let mut names = HashSet::new();
        for song in SONGS {
            assert!(names.insert(song.name), "{} twice", song.name);
            for name in song.form {
                assert!(song.part(name).is_some(), "{}: no part {name}", song.name);
            }
            assert!(song.drums_from < song.form.len().max(1), "{}", song.name);
            assert!(
                song.rhythm.is_empty() || RHYTHMS.iter().any(|r| r.name == song.rhythm),
                "{}: no rhythm {}",
                song.name,
                song.rhythm
            );
        }
        assert!(SONGS.len() >= 15);
    }

    #[test]
    fn melodies_are_mostly_in_their_key() {
        // A few notes from outside the key are normal (passing notes,
        // borrowed leading notes), but a song with many is more likely
        // a typo, such as F written for F#.
        for song in SONGS {
            let scale = song.scale();
            let notes: Vec<_> = song
                .parts
                .iter()
                .flat_map(|part| part.melody().notes)
                .collect();
            let inside = notes
                .iter()
                .filter(|note| scale.contains(note.pitch.pitch_class()))
                .count();
            assert!(
                inside * 100 >= notes.len() * 80,
                "{}: only {inside} of {} notes in {}",
                song.name,
                notes.len(),
                scale.name()
            );
        }
    }

    #[test]
    fn the_first_notes_are_right() {
        let first = |song: &str| {
            let song = Song::named(song).unwrap();
            let part = song.part(song.form[0]).unwrap();
            let notes: Vec<u8> = part
                .melody()
                .notes
                .iter()
                .take(4)
                .map(|n| n.pitch.midi())
                .collect();
            notes
        };
        assert_eq!(first("Ode to Joy"), [78, 78, 79, 81], "F# F# G A");
        assert_eq!(
            first("Twinkle, Twinkle, Little Star"),
            [72, 72, 79, 79],
            "C C G G"
        );
        assert_eq!(first("Korobeiniki"), [76, 71, 72, 74], "E B C D");
    }
}
