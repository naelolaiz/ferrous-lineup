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

// The instrumental turn of Entarisi ala benziyor, which the refrain sings too.
const ENTARI_TURN: &str = "D5:2 E5 F5 E5 D5 C#5:4 C#5 | A4:2 Bb4 D5 C#5 Bb4 A4:4 E5 | \
                           D5:2 E5 F5 E5 D5 C#5:4 C#5 | A4:2 Bb4 D5 C#5 Bb4 A4:4 -:4";
const ENTARI_TURN_CHORDS: &str = "iv iv iv iv iv I I I I | I | iv iv iv iv iv I I I I | I";
// "Entarisi ala benziyor" and "Benim yârim sana benziyor" share a tune.
const ENTARI_LINE: &str = "A4:4 D5 C#5:2 D5:4 -:4 | C#5:2 D5 E5:4 F5:2 E5 D5 C#5 Bb4 | \
                           A4:4 D5 C#5:2 D5:4 -:4 | C#5:2 D5 E5:4 F5:2 E5 D5 C#5 Bb4";

// Source: https://raw.githubusercontent.com/MTG/SymbTr/246bbe362ef17cf9d200b5296e06473f29768be3/txt/hicaz--turku--raksaksagi--entarisi_ala--istanbul.txt
// (the SymbTr corpus of Turkish makam music: makam hicaz, usul raks aksağı, 9/8, from
// Istanbul; 64 bars, every one of them here, in its order. Turkish music is written in
// commas, 53 to the octave: its B flat and C sharp are four commas from B and C, which a
// twelve-tone semitone matches closely. The usul is "a five and a four" (TDV İslâm
// Ansiklopedisi, "Raks aksağı"): 2 + 3 + 2 + 2, as the tune's long notes show. The song is
// sung without chords; the chords here are added, from the notes the tune leans on: D
// minor under its runs from D to F, A major under its C sharps and A's, B flat major under
// the long B flats.)
pub(super) const ENTARISI: Song = Song {
    name: "Entarisi ala benziyor",
    origin: "Turkish folk song (türkü) from Istanbul",
    feel: "Raks aksağı",
    genre: Genre::Balkan,
    tonic: A,
    mode: Phrygian,
    time: RAKS_AKSAGI,
    tempo: 200.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Raks aksağı",
    band: TURKISH,
    style: "Raks aksağı",
    parts: &[
        Part::new("Aranağme", ENTARI_TURN, ENTARI_TURN_CHORDS),
        Part::new("Entarisi", ENTARI_LINE, "iv | I | iv | I"),
        Part::new(
            "Şeftalisi",
            "E5:2 D5 C#5:4 A4:2 Bb4:6 A4:2 | E5:2 D5 C#5:3 Bb4:1 Bb4 A4 A4:4 -:4 | \
             E5:2 D5 C#5:4 A4:2 Bb4:6 A4:2 | E5:2 D5 C#5:3 Bb4:1 Bb4 A4 A4:4 -:4",
            "I I I I I bII bII I I | I | I I I I I bII bII I I | I",
        ),
        Part::new("Benim yârim", ENTARI_LINE, "iv | I | iv | I"),
        Part::new(
            "Olamaz",
            "E5:2 D5 C#5:4 A4:2 Bb4:4 Bb4:2 A4 | E5:2 D5 C#5:3 Bb4:1 Bb4 A4 A4:4 -:4 | \
             E5:2 D5 C#5:4 Bb4:2 Bb4:4 Bb4:2 A4 | E5:2 D5 C#5:3 Bb4:1 Bb4 A4 A4:4 -:4",
            "I I I I I bII bII I I | I | I I I I I bII bII I I | I",
        ),
        Part::new("Şekerli misin", ENTARI_TURN, ENTARI_TURN_CHORDS),
    ],
    form: &[
        "Aranağme",
        "Aranağme",
        "Entarisi",
        "Şeftalisi",
        "Benim yârim",
        "Olamaz",
        "Şekerli misin",
        "Şekerli misin",
        "Aranağme",
        "Aranağme",
        "Entarisi",
        "Şeftalisi",
        "Benim yârim",
        "Olamaz",
        "Şekerli misin",
        "Şekerli misin",
    ],
    drums_from: 0,
    about: "\"Her dress looks scarlet, her cheek like honey, my love looks like you\"; then \
            the sad news, \"it cannot be, he is engaged\", and a teasing refrain, \"are you \
            sugar, are you cream\", sung to the instrumental turn (aranağme). Nine eighths \
            to the bar in the Turkish raks aksağı, a five and a four, counted 1 2, 1 2 3, \
            1 2, 1 2. The tune is in makam hicaz on A: A, B flat, C sharp, D, E, F, G, with \
            the step and a half from B flat up to C sharp that gives it its eastern colour, \
            the same notes as the Phrygian mode with a raised third. Turkish singers and \
            players go in unison over a drum; the chords here, D minor, A major and B flat, \
            are added from the notes the tune leans on.",
};
