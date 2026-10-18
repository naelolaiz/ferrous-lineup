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

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201294633.abc
// (John Chambers' transcription in 7/16, in E with G sharp; written here in eighths, each
// note twice as long. Only his sung part, "Vocal": its first phrase is sung alone, then two
// voices a third apart. The melody is the lower of the two: it starts on the G sharp the
// solo phrase started on, and it ends B with the solo phrase itself, G# (B) A G# F E; the
// upper voice is the harmony a third above, ending on G sharp. The held bars he marks
// optional (*1) are kept, so each line of A lasts three bars and B ten. A is sung once and B
// twice, as he writes it. His "Instr." phrases, taken from two recordings, are left out. The
// same transcription in abc_tune_jc_201294636.abc, in D and in E, repeats A as well and sings
// the last beat of bar 4, F E F, as an eighth and two sixteenths instead of a triplet; the
// other notes are the same. The chords are his, the optional (Dm) included.)
pub(super) const JOVANO_JOVANKE: Song = Song {
    name: "Jovano, Jovanke",
    origin: "Macedonian folk song",
    feel: "Lesnoto",
    genre: Genre::Balkan,
    tonic: E,
    mode: Phrygian,
    time: THREE_TWO_TWO,
    tempo: 120.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Lesnoto",
    band: BALKAN,
    style: "Lesnoto",
    parts: &[
        Part::new("Intro", "-:14 | -:14", "I | I"),
        Part::new(
            "A",
            "G#4:6 A4:2 G#4 F4:4 | E4:14 | _:14 | G#4:4 B4:2 A4 G#4 (3 F4:2 E4 F4 | G#4:14 | _:14",
            "I I I bvii bvii bvii bvii | I | I | I I I bvii bvii bvii bvii | I | I",
        ),
        Part::new(
            "B",
            "C5:4 C5:2 C5:4 C5 | C5:2 B4 C5 D5:4 D5:2 C5 | B4:4 B4:2 B4:4 B4 | \
             B4:2 A4 B4 C5:8 | A4:4 A4:2 A4:4 B4:2 C5 | D5:2 C5:4 B4 A4 | \
             G#4:4 G#4:2 A4 G#4 A4 B4 | G#4:4 B4:2 A4 G#4 F4:4 | E4:14 | _:14",
            "iv | iv iv iv bvii bvii bvii bvii | bIII | bIII bIII bIII bVI bVI bVI bVI | \
             iv | bvii | I | I | I | I",
        ),
    ],
    form: &["Intro", "A", "B", "B"],
    drums_from: 0,
    about: "Jovana sits by the river Vardar, bleaching white linen: a Macedonian love song, \
            sung slowly over the lesnoto, seven eighths to the bar counted 1 2 3, 1 2, 1 2, a \
            long beat and two short ones. The tune is in E with a G sharp, E F G# A B C D: \
            the Phrygian mode with a major third, the sound of much Balkan music, whose \
            step from G sharp down to F is an augmented second. One voice sings the first \
            phrase, G# A G# F E, over E major and D minor (bvii); then a second voice joins \
            a third above. The second part leaps up to C and moves through A minor (iv), D \
            minor, G major (bIII) and C major (bVI), whose G is natural, before E major and \
            its G sharp come back and the tune falls to E as the first phrase did. The long \
            notes are held for two bars.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201333824.abc
// (John Chambers' transcription in G with its chords and the words of four verses. He writes
// two voices in thirds and notes that the lower line is the melody: it is the line the words
// go with, and the one he writes first in each chord, with the note lengths. His
// instrumental interlude, "a generic interlude used in much Greek music", is written the
// other way round, the upper note first, and its upper line is the one the chords fit (D C D
// over D7, where the lower line has B and G), so the violin plays that one. The interlude
// opens the song and comes before each verse; each verse sings the first line twice and the
// second line twice, with its two endings. Where one section runs into the next (the
// interlude into the verse's pickup, the first line into the second) and at the very end,
// his bars are two eighths short; a rest fills them. The same transcription, note for note,
// is in abc_tune_jc_201333819.abc, abc_tune_jc_201359716.abc and, with a version a tone
// higher in A, abc_tune_jc_201234052.abc.)
pub(super) const SAMIOTISSA: Song = Song {
    name: "Samiotissa",
    origin: "Greek folk song from Samos",
    feel: "Kalamatianos",
    genre: Genre::Balkan,
    tonic: G,
    mode: Major,
    time: THREE_TWO_TWO,
    tempo: 168.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Kalamatianos",
    band: GREEK,
    style: "Kalamatianos",
    parts: &[
        Part::new("Intro", "-:14 | -:8 G5:2 F#5 E5", "I | I"),
        Part::new(
            "Interlude",
            "D5:4 D5:2 C5 B4 C5 D5 | B4:4 C5:2 D5 G5 F#5 E5 | D5:4 D5:2 C5 B4 C5 D5 | \
             B4:6 -:2 G5:2 F#5 E5 | D5:4 D5:2 C5 B4 C5 D5 | B4:4 C5:2 D5 G5 F#5 E5 | \
             D5:4 D5:2 C5 B4 C5 D5 | B4:6 -:4 D4:4",
            "V7 | I | V7 | I | V7 | I | V7 | I",
        ),
        Part::new(
            "First line",
            "G4:4 F#4:2 G4:4 A4 | B4:4 A4:2 G4:8 | C5:2 C5 C5 C5 B4 A4 G4 | B4:6 -:4 D4:4 | \
             G4:4 F#4:2 G4:4 A4 | B4:4 A4:2 G4:8 | C5:2 C5 C5 C5 B4 A4 G4 | B4:6 -:8",
            "I | I | IV | I | I | I | IV | I",
        ),
        Part::new(
            "Second line",
            "D5:2 D5 D5 E5 D5 C5 B4 | A4:4 C5:2 B4 A4 G4 A4 | B4:4 B4:2 A4 G4 A4 B4 | \
             G4:4 A4:2 B4:4 C5 | D5:2 D5 D5 E5 D5 C5 B4 | A4:4 C5:2 B4 A4 G4 A4 | \
             B4:4 B4:2 A4 G4 A4 B4 | G4:6 -:2 G5:2 F#5 E5",
            "V7 V7 V7 IV IV IV IV | V7 V7 V7 I I I I | I I I V7 V7 V7 V7 | I | \
             V7 V7 V7 IV IV IV IV | V7 V7 V7 I I I I | I I I V7 V7 V7 V7 | I",
        ),
        Part::new(
            "Last second line",
            "D5:2 D5 D5 E5 D5 C5 B4 | A4:4 C5:2 B4 A4 G4 A4 | B4:4 B4:2 A4 G4 A4 B4 | \
             G4:4 A4:2 B4:4 C5 | D5:2 D5 D5 E5 D5 C5 B4 | A4:4 C5:2 B4 A4 G4 A4 | \
             B4:4 B4:2 A4 G4 A4 B4 | G4:6 -:8",
            "V7 V7 V7 IV IV IV IV | V7 V7 V7 I I I I | I I I V7 V7 V7 V7 | I | \
             V7 V7 V7 IV IV IV IV | V7 V7 V7 I I I I | I I I V7 V7 V7 V7 | I",
        ),
    ],
    form: &[
        "Intro",
        "Interlude",
        "First line",
        "Second line",
        "Interlude",
        "First line",
        "Second line",
        "Interlude",
        "First line",
        "Second line",
        "Interlude",
        "First line",
        "Last second line",
    ],
    drums_from: 0,
    about: "\"Girl from Samos, when will you go to Samos?\": a kalamatianos, the Greek \
            circle dance in seven eighths, counted 1 2 3, 1 2, 1 2, slow, quick, quick. The \
            verse starts with a pickup on the last short beat, the D of \"Sa-\". The song is \
            in G major on three chords, G, C (IV) and D7 (V7): the first line stays on G and \
            touches C, and in the second line the chord changes after the long beat, D7 to \
            C, D7 to G, G to D7. The singers go in thirds; the tune is the lower voice. \
            Before each verse the violin plays a stock interlude heard in much Greek music, \
            swinging between D7 and G.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2942.abc
// (Henrik Norbeck's transcription in G of the paidushko Lúnasa recorded on The Kinnitty
// Sessions, at his tempo: four strains, each played twice, A and B with first and second
// endings. Every bar is two eighths and then three, the three as a quarter and an eighth or
// as three eighths: 2 + 3. Chosen of the five sources because it is on a widely heard record
// and Norbeck transcribed it twice: his "Paidushko Choro" (abc_tune_norbeck_2944.abc) has
// its A and B strains in another rhythm (B2 ddd for BG d2G), ending both on A, and then three
// strains in A Mixolydian that are not in this one. The other three are other tunes:
// Norbeck's in A minor (abc_tune_norbeck_2943.abc), and John Chambers' "Paidushko"
// (abc_tune_jc_201356092.abc) and "Kilimarsko" (abc_tune_jc_201296692.abc), which share
// their first strain. Norbeck writes no chords: these are a plain harmonization, G, C,
// A minor and D or D7, changing on the long beat where the tune does.)
pub(super) const PAIDUSHKO_HORO: Song = Song {
    name: "Paidushko horo",
    origin: "Bulgarian folk dance",
    feel: "Paidushko",
    genre: Genre::Balkan,
    tonic: G,
    mode: Major,
    time: FIVE_EIGHT,
    tempo: 180.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Paidushko",
    band: KAVAL,
    style: "Paidushko",
    parts: &[
        Part::new("Intro", "-:10 | -:8 G4:2", "I | I"),
        Part::new(
            "A",
            "B4:2 G4 D5:4 G4:2 | B4:2 G4 C5:4 G4:2 | C5:2 B4 A4:4 G4:2 | A4:2 G4 B4:4 G4:2 | \
             B4:2 G4 D5:4 G4:2 | B4:2 G4 C5:4 G4:2 | C5:2 B4 A4:4 G4:2 | A4:2 E4 A4:4 G4:2 | \
             B4:2 G4 D5:4 G4:2 | B4:2 G4 C5:4 G4:2 | C5:2 B4 A4:4 G4:2 | A4:2 G4 B4:4 G4:2 | \
             B4:2 G4 D5:4 G4:2 | B4:2 G4 C5:4 G4:2 | C5:2 B4 A4:4 G4:2 | A4:2 E4 A4:4 E4:2",
            "I | I I IV IV IV | ii | V7 V7 I I I | I | I I IV IV IV | ii | ii | \
             I | I I IV IV IV | ii | V7 V7 I I I | I | I I IV IV IV | ii | ii",
        ),
        Part::new(
            "B",
            "C5:2 B4 A4:4 G4:2 | F#4:2 G4 A4:4 E4:2 | C5:2 B4 A4:4 G4:2 | B4:2 G4 D5 G4 G4 | \
             C5:2 B4 A4:4 G4:2 | F#4:2 G4 A4:4 E4:2 | C5:2 B4 A4:4 G4:2 | A4:2 E4 A4:4 E4:2 | \
             C5:2 B4 A4:4 G4:2 | F#4:2 G4 A4:4 E4:2 | C5:2 B4 A4:4 G4:2 | B4:2 G4 D5 G4 G4 | \
             C5:2 B4 A4:4 G4:2 | F#4:2 G4 A4:4 E4:2 | C5:2 B4 A4:4 G4:2 | A4:2 E4 A4:4 B4:2",
            "ii | V | ii | I | ii | V | ii | ii | ii | V | ii | I | ii | V | ii | ii",
        ),
        Part::new(
            "C",
            "C5:2 D5 C5 B4 A4 | B4:2 G4 A4:4 G4:2 | B4:2 G4 A4:4 G4:2 | G4:2 A4 B4:4 G4:2 | \
             C5:2 D5 C5 B4 A4 | B4:2 G4 A4:4 G4:2 | B4:2 G4 A4:4 G4:2 | G4:2 D4 G4:4 G4:2",
            "V7 | I I V7 V7 V7 | I I V7 V7 V7 | I | V7 | I I V7 V7 V7 | I I V7 V7 V7 | I",
        ),
        Part::new(
            "D",
            "B4:2 G4 A4:4 G4:2 | G4:2 A4 B4:4 G4:2 | B4:2 G4 A4:4 G4:2 | G4:2 G4 D4 G4 G4 | \
             B4:2 G4 A4:4 G4:2 | G4:2 A4 B4:4 G4:2 | B4:2 G4 A4:4 G4:2 | G4:2 D4 G4:4 G4:2",
            "I I V7 V7 V7 | I | I I V7 V7 V7 | I | I I V7 V7 V7 | I | I I V7 V7 V7 | I",
        ),
    ],
    form: &["Intro", "A", "B", "C", "C", "D", "D"],
    drums_from: 0,
    about: "Five eighths to the bar, counted 1 2, 1 2 3: a quick beat and a slow one, the \
            limp of the paidushko, danced in Bulgaria and, as the pajdushko, in Macedonia. \
            The tune is in G major, but its first two strains turn around A and end on it, \
            over A minor (ii), while the last two come home to G. The chords are a plain \
            harmonization, G, C (IV), A minor, D and D7 (V), several of them changing on the \
            slow beat, where the tune leans on a long note. Four strains of eight bars, each \
            played twice.",
};
