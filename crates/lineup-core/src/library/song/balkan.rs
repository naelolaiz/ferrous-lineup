//! Balkan and Turkish: dance songs in bars of long and short beats.
//!
//! Their meters are counted in beats of two and three eighths, a long
//! beat being half as long again as a short one: 3 + 2 + 2 for a
//! Macedonian lesnoto or a Greek kalamatianos, 2 + 2 + 3 for a Bulgarian
//! rachenitsa. Scores of these dances are often written in sixteenths
//! (7/16, 11/16); here they are written in eighths, each note twice as
//! long, which sounds the same at half the tempo.

use super::*;

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201245522.abc
// (John Chambers' transcription, "Srbija", lesnoto, in D minor: the sung melody with its
// chords, four verses, the second line repeated. His instrumental verse is left out.)
pub(super) const AJDE_JANO: Song = Song {
    name: "Ajde Jano",
    origin: "Serbian folk song",
    feel: "Lesnoto",
    genre: Genre::Balkan,
    tonic: D,
    mode: NaturalMinor,
    time: THREE_TWO_TWO,
    tempo: 150.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Lesnoto",
    band: BALKAN,
    style: "Lesnoto",
    parts: &[
        Part::new("Intro", "-:14 | -:14", "i | i"),
        Part::new(
            "Verse",
            "F4:4 E4:2 D4 E4 F4:4 | F4:4 E4:2 E4:4 D4 | D4:2 E4 D4 C4:8 | _:4 G3:2 C4:4 C4",
            "i | i | bVII | bVII",
        ),
        Part::new(
            "Refrain",
            "F4:4 G4:2 A4:4 A4 | G4:4 F4:2 F4 E4 D4:4 | G4:4 F4:2 E4:4 D4:2 E4 | \
             F4:6 _:2 G4 F4 E4 | D4:6 _:8 | _:6 _:8",
            "bIII | iv iv iv i i i i | iv | i | i | i",
        ),
    ],
    form: &[
        "Intro", "Verse", "Refrain", "Refrain", "Verse", "Refrain", "Refrain", "Verse", "Refrain",
        "Refrain", "Verse", "Refrain", "Refrain",
    ],
    drums_from: 0,
    about: "\"Come, Jana, let's dance the kolo\": seven eighths to the bar, counted 1 2 3, \
            1 2, 1 2, a long beat and two short ones. The tune sits in D minor and rests on \
            C major (bVII) at the end of the first line, as much Balkan music does, before \
            the refrain climbs to A over F major and settles back on D. Each verse sells \
            something more, the horse, the house, just to dance.",
};
