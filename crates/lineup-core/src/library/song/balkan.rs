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

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201354585.abc
// (John Chambers' transcription in C, which names Theodore Alevizos' record "Songs of Greece"
// and an Alshire record of Greek folk songs, with his chords and the words of four verses. He
// writes two voices in thirds, the upper note first in every chord and with the note lengths:
// the upper line is the tune, and the one his chords fit, G on the long beats over G7 where
// the lower line holds E, which is not in the chord. The lower line is left out. Each verse
// is sung to the whole tune, the refrain twice with its two endings. The second ending holds
// E for two bars here and in abc_tune_jc_201354582.abc, for one bar in
// abc_tune_jc_201354586.abc, abc_tune_jc_201361006.abc and abc_tune_jc_201358690.abc: one bar
// is played between verses, so that each verse lasts sixteen bars, and two at the very end.
// The notes are otherwise the same in all five. Copies a tone higher in D, the one in
// abc_tune_jc_201354589.abc and the parts "for B flat instruments" after the C version in the
// last three, hold the first note of the fifth bar for three eighths where the sung version
// repeats it, a quarter and an eighth: the sung version is followed. The copy in D in
// abc_tune_jc_201354582.abc has the sung rhythm.)
pub(super) const YERAKINA: Song = Song {
    name: "Yerakina",
    origin: "Greek folk song from Macedonia",
    feel: "Kalamatianos",
    genre: Genre::Balkan,
    tonic: C,
    mode: Major,
    time: THREE_TWO_TWO,
    tempo: 168.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Kalamatianos",
    band: GREEK,
    style: "Kalamatianos",
    parts: &[
        Part::new("Intro", "-:14 | -:14", "I | I"),
        Part::new(
            "Verse",
            "G4:4 A4:2 B4 C5 B4 A4 | G4:14 | G4:4 A4:2 B4 C5 B4 A4 | G4:4 F4:2 G4 F4 E4:4 | \
             E4:4 E4:2 F4 G4 F4 E4 | F4:14 | G4:4 F4:2 F4 G4 F4 E4 | E4:4 F4:2 G4 A4 B4 A4",
            "I | I | I | I | vi | ii | V7 | I",
        ),
        Part::new(
            "Refrain",
            "G4:6 F4:4 F4:2 E4 | G4:6 G4:2 A4 B4 A4 | G4:2 G4:4 F4:2 G4 F4 E4 | \
             E4:4 F4:2 G4 A4 B4 A4 | G4:6 F4:4 F4:2 E4 | G4:6 G4:2 A4 B4 A4 | \
             G4:2 G4:4 F4:2 G4 F4 E4 | E4:14",
            "V7 | I | V7 | I | V7 | I | V7 | I",
        ),
        Part::new(
            "Last refrain",
            "G4:6 F4:4 F4:2 E4 | G4:6 G4:2 A4 B4 A4 | G4:2 G4:4 F4:2 G4 F4 E4 | \
             E4:4 F4:2 G4 A4 B4 A4 | G4:6 F4:4 F4:2 E4 | G4:6 G4:2 A4 B4 A4 | \
             G4:2 G4:4 F4:2 G4 F4 E4 | E4:14 | _:14",
            "V7 | I | V7 | I | V7 | I | V7 | I | I",
        ),
    ],
    form: &[
        "Intro",
        "Verse",
        "Refrain",
        "Verse",
        "Refrain",
        "Verse",
        "Refrain",
        "Verse",
        "Last refrain",
    ],
    drums_from: 0,
    about: "Yerakina goes to the well for cold water and falls in; everyone runs, and the \
            singer promises to pull her out and make her his wife, while her bracelets jingle, \
            \"droum, droum\". A kalamatianos, the Greek circle dance in seven eighths, counted \
            1 2 3, 1 2, 1 2, slow, quick, quick. The tune is in C major: it climbs from G to C \
            and back and holds G for a whole bar, then sinks to E over A minor (vi) and to F \
            over D minor (ii) before G7 (V7) brings it home. The refrain swings between G7 and \
            C, a bar each, and is sung twice. The singers go in thirds; the tune is the upper \
            voice.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201309933.abc
// (John Chambers' ABC of 2017, from tousauxbalkans.net, in C with its chords and the Greek
// words. Three strains, each played twice with two endings: an instrumental one written in
// two voices a third apart all through, of which the violin plays the upper, then the two
// halves of the sung tune, in one voice. The words come in three stanzas, some lines written
// between slashes: as the strains go, those are sung twice, and the others one to each time
// through the second half. So the first half takes one line twice and the second half the
// next line twice, or, in the second and third stanzas, two lines. The whole tune is played
// for each stanza, the instrumental strain first. abc_tune_jc_201309932.abc is the same
// without the words, and abc_tune_jc_201309934.abc the same a tone higher in D, except for
// one chord: in the third bar of the first sung half, under A A A, the copies in C have an
// empty chord symbol where the copy in D has G, its IV. F, the IV in C, is played here.)
pub(super) const MILO_MOU_KOKKINO: Song = Song {
    name: "Milo mou kokkino",
    origin: "Greek folk song",
    feel: "Kalamatianos",
    genre: Genre::Balkan,
    tonic: C,
    mode: Major,
    time: THREE_TWO_TWO,
    tempo: 168.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Kalamatianos",
    band: GREEK,
    style: "Kalamatianos",
    parts: &[
        Part::new(
            "Interlude",
            "E4:1 F4 G4:2 G4 G4 F4 F4 E4 | F4:1 G4 A4:2 A4 A4 G4 G4 F4 | \
             E4:1 F4 G4:2 G4 G4 F4 F4 E4 | E4:2 D4 C4 B3:8 | \
             E4:1 F4 G4:2 G4 G4 F4 F4 E4 | F4:1 G4 A4:2 A4 A4 G4 G4 F4 | \
             E4:1 F4 G4:2 G4 G4 F4 F4 E4 | E4:4 F4:2 G4:1 F4 E4 F4 E4:4",
            "I | IV | I | I I I V7 V7 V7 V7 | I | IV | I | I I I V7 V7 I I",
        ),
        Part::new(
            "First half",
            "C4:6 C4:4 C4 | E4:4 F4:2 G4:8 | A4:6 A4:4 A4 | G4:4 F4:2 E4:3 F4:1 E4:2 D4 | \
             C4:6 C4:4 C4 | E4:4 F4:2 G4:8 | A4:6 A4:4 A4 | G4:4 F4:2 E4:8",
            "I | I | IV | I | I | I | IV | I",
        ),
        Part::new(
            "Second half",
            "G4:6 A4:2 G4 F4 E4 | F4:4 E4:2 D4:4 C4:2 D4 | E4:6 F4:2 E4 D4 C4 | \
             E4:4 D4:2 C4 D4 E4 F4 | G4:6 A4:2 G4 F4 E4 | F4:4 E4:2 D4:4 C4:2 D4 | \
             E4:6 F4:2 E4 D4 C4 | E4:4 D4:2 C4:8",
            "I | ii | V7 | I | I | ii | V7 | I",
        ),
    ],
    form: &[
        "Interlude",
        "First half",
        "Second half",
        "Interlude",
        "First half",
        "Second half",
        "Interlude",
        "First half",
        "Second half",
    ],
    drums_from: 0,
    about: "\"My red apple, dyed pomegranate, why have you made me wither?\" The singer comes \
            and goes and finds her door bolted; her quilt tells him she has gone to the spring \
            for water. A kalamatianos, seven eighths to the bar counted 1 2 3, 1 2, 1 2, slow, \
            quick, quick: the sung tune starts by spelling the count out, three Cs, one on each \
            beat. It is in C major, on C, F (IV), D minor (ii) and G7 (V7), and each half is \
            sung twice. Before each stanza the violin plays a strain the source writes in \
            thirds; the first time through it stops on B over G7, a half cadence, the second \
            time on E over C.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/robinson/abc_tune_robinson_2752.abc
// (Richard Robinson's tunebook, from Cecil Sharp House via Ian McArdle, in 5/8 with chords:
// two strains of four bars, each played twice. Every bar is three eighths and then two: the
// tune moves on the first three and holds a note on the last two, where the chords change, so
// 3 + 2. Twice the tune turns quickly around a note before a dotted eighth, E F E and D E D,
// three thirty-second notes in a triplet, a sixteenth in all: a sixteenth is the shortest
// note written here, so each turn is played as its first note, held for that sixteenth. The
// chords are his: D minor goes on under the first beat of the fourth and eighth bars until A
// minor comes with the A. The copies in John Chambers' collection have the same notes:
// abc_tune_jc_201360585.abc, a copy of this one, writes the two chord changes of the second
// strain on the second note of the turn, as Robinson does; abc_tune_jc_201360324.abc and
// abc_tune_jc_201358690.abc write them on its first, on the beat, where they are played
// here.)
pub(super) const TSAKONIKOS: Song = Song {
    name: "Tsakonikos",
    origin: "Greek folk dance from Tsakonia",
    feel: "Tsakonikos",
    genre: Genre::Balkan,
    tonic: A,
    mode: NaturalMinor,
    time: FIVE_EIGHT_THREE_TWO,
    tempo: 140.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Tsakonikos",
    band: GREEK,
    style: "Tsakonikos",
    parts: &[
        Part::new("Intro", "-:10 | -:10", "i | i"),
        Part::new(
            "A",
            "G4:2 C5 B4:1 G4 A4:4 | G4:2 C5 B4:1 G4 A4:4 | G4:2 C5 B4:1 G4 D5:4 | \
             G4:2 C5 B4:1 G4 A4:4",
            "i | i | i i i iv iv | iv iv iv i i",
        ),
        Part::new(
            "B",
            "E5:2 E5 E5 E5:1 D5:3 | D5:2 D5 D5 D5:1 C5:3 | G4:2 C5 B4:1 C5 D5:4 | \
             G4:2 C5 B4:1 G4 A4:4",
            "i i i iv iv | iv iv iv i i | i i i iv iv | iv iv iv i i",
        ),
    ],
    form: &["Intro", "A", "A", "B", "B"],
    drums_from: 0,
    about: "The dance of Tsakonia, in the Peloponnese, danced in a long winding line. Five \
            eighths to the bar, counted 1 2 3, 1 2: a long beat and a short one, slow, quick, \
            the paidushko's limp the other way round. The tune is in A minor; most bars turn \
            G C B G on the long beat and come to rest on the short one, on A or on D. The \
            chords change there too, A minor and D minor (iv) each lasting from one short beat \
            to the next, so every bar starts over the chord the bar before ended on. Two \
            strains of four bars, the second higher, each played twice.",
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

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2948.abc
// (Henrik Norbeck's ABC tunes, rachenitsa 4, in 2+2+3/16: written here in eighths, every
// note twice as long, without the rolls and grace notes. Six strains, each played twice, then
// da capo to the fine at the end of the second strain, its repeats taken again; the sixth
// strain is the third with a last bar that leads back to the start. The source has no
// chords: these are a plain harmonization of the melody. No second source of this tune was
// found to compare.)
pub(super) const SVATBARSKA_RACHENITSA: Song = Song {
    name: "Svatbarska rachenitsa",
    origin: "Bulgarian wedding dance, traditional",
    feel: "Rachenitsa",
    genre: Genre::Balkan,
    tonic: A,
    mode: NaturalMinor,
    time: TWO_TWO_THREE,
    tempo: 220.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Rachenitsa",
    band: BALKAN,
    style: "Rachenitsa",
    parts: &[
        Part::new(
            "A",
            "C5:4 D5:2 E5 E5:4 D5:2 | D5:2 C5 E5 D5 D5:4 C5:2 | C5:2 D5 D5 C5 C5 B4 G4 | \
             A4:2 G4 A4 B4 C5:4 G4:2 | C5:4 D5:2 E5 E5:4 D5:2 | D5:2 C5 E5 D5 D5:4 C5:2 | \
             C5:2 D5 D5 C5 C5 B4 G4 | A4:4 A4:2 E4 A4:6",
            "i | iv7 | bIII | i | i | iv7 | bIII | i",
        ),
        Part::new(
            "B",
            "C5:2 D5 D5 A4 C5 D5 D5 | C5:2 B4 A4 B4 C5:4 G4:2 | C5:2 D5 B4:4 G4:2 A4 B4 | \
             A4:2 G4 A4 B4 C5:4 G4:2 | C5:2 D5 D5 A4 C5 D5 D5 | C5:2 B4 A4 B4 C5:4 G4:2 | \
             E5:2 D5 D5 C5 C5 B4 G4 | A4:4 C5:2 D5 A4:6",
            "iv7 | i | bVII | i | iv7 | i | bIII | i",
        ),
        Part::new(
            "C",
            "A4:2 E5 D5 E5 C#5:4 C#5:2 | A4:2 B4 B4 A4 C#5:4 C#5:2 | E5:2 A4 D5 E5 C#5:4 C#5:2 | \
             A4:2 B4 B4 A4 A4 E4 A4 | A4:2 E5 D5 E5 C#5:4 C#5:2 | A4:2 B4 B4 A4 C#5:4 C#5:2 | \
             E5:2 A4 D5 E5 C#5:4 C#5:2 | A4:2 B4 B4 A4 A4:6",
            "I | I | I | I | I | I | I | I",
        )
        .in_key(A, Major),
        Part::new(
            "D",
            "A4:2 B4 B4:4 D5:2 C#5 A4 | C#5:4 C#5:2 A4 B4:6 | B4:4 D5:2 E5 C#5 D5 E5 | \
             C#5:4 C#5:2 A4 B4:6 | A4:2 B4 B4:4 D5:2 C#5 A4 | C#5:4 C#5:2 A4 B4:6 | \
             B4:4 D5:2 E5 C#5 D5 E5 | C#5:4 B4:2 A4 A4:6",
            "I I V V I I I | I I I I V V V | V7 V7 V7 V7 I I I | I I I I V V V | \
             I I V V I I I | I I I I V V V | V7 V7 V7 V7 I I I | I",
        )
        .in_key(A, Major),
        // The makam hicaz on A: B flat and C sharp, an augmented second apart.
        Part::new(
            "E",
            "F5:2 E5 D5 C#5 Bb4 A4 G4 | A4:2 Bb4 C#5:4 A4:2 C#5 D5 | F5:2 E5 D5 C#5 Bb4 A4 G4 | \
             A4:4 A4:2 E4 C#5 D5 E5 | F5:2 E5 D5 C#5 Bb4 A4 G4 | A4:2 Bb4 C#5:4 A4:2 C#5 D5 | \
             F5:2 E5 D5 C#5 Bb4 A4 G4 | A4:4 A4:2 E4 A4:6",
            "bvii7 | I | bvii7 | I | bvii7 | I | bvii7 | I",
        )
        .in_key(A, Phrygian),
        // The third strain again, its last bar leading back to the first.
        Part::new(
            "F",
            "A4:2 E5 D5 E5 C#5:4 C#5:2 | A4:2 B4 B4 A4 C#5:4 C#5:2 | E5:2 A4 D5 E5 C#5:4 C#5:2 | \
             A4:2 B4 B4 A4 A4 E4 A4 | A4:2 E5 D5 E5 C#5:4 C#5:2 | A4:2 B4 B4 A4 C#5:4 C#5:2 | \
             E5:2 A4 D5 E5 C#5:4 C#5:2 | A4:2 B4 B4 A4 A4 E4 A4",
            "I | I | I | I | I | I | I | I",
        )
        .in_key(A, Major),
    ],
    form: &[
        "A", "A", "B", "B", "C", "C", "D", "D", "E", "E", "F", "F", "A", "A", "B", "B",
    ],
    drums_from: 0,
    about: "A wedding rachenitsa: seven eighths to the bar, counted 1 2, 1 2, 1 2 3, quick, \
            quick, slow. The first two strains are in A minor and never touch F; under them \
            go A minor, D minor seven (iv7), C (bIII) and G (bVII). The next two turn to A \
            major: the third stays on the A chord, the fourth leans on B over E, the V. The \
            fifth runs down from F through C sharp and B flat, an augmented second apart, the \
            mark of the makam hicaz, over G minor seven and A major. The third strain comes \
            back, and the tune goes back to the start to end after the second.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/robinson/abc_tune_robinson_2930.abc
// (Richard Robinson's tunebook, "Diacove (Bulgaria)", from Cathy Goss, in 9/16 counted
// 2 + 2 + 2 + 3: written here in 9/8, every note twice as long. Six strains, each played
// twice. John Chambers' copy of Andy Hornby and Cathy Goss's ABC,
// https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201268487.abc,
// has the same notes bar for bar. Neither has chords: these are a plain harmonization of the
// melody.)
pub(super) const DAICHOVO_HORO: Song = Song {
    name: "Daichovo horo",
    origin: "Bulgarian dance tune, traditional",
    feel: "Daichovo",
    genre: Genre::Balkan,
    tonic: A,
    mode: Dorian,
    time: NINE_EIGHT_AKSAK,
    tempo: 220.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Daichovo",
    band: KAVAL,
    style: "Daichovo",
    parts: &[
        Part::new(
            "A",
            "E5:4 D5 E5 C5 B4:2 | C5:2 D5 E5:4 C5:2 A4 B4:4 G4:2 | A4:8 C5:2 D5 E5 -:4 | \
             E5:4 D5 E5 C5 B4:2 | C5:2 D5 E5:4 C5:2 A4 B4:4 G4:2 | A4:8 A4:4 A4:2 -:4",
            "i | i i i i i i bVII bVII bVII | i | i | i i i i i i bVII bVII bVII | i",
        ),
        Part::new(
            "B",
            "C5:2 D5 D5 E5 C5 A4 B4:4 A4:2 | D5:4 D5:2 E5 C5 A4 B4:4 A4:2 | \
             C5:2 D5 D5 E5 C5 A4 B4:4 A4:2 | A4:8 C5:2 D5 E5 -:4 | \
             C5:2 D5 D5 E5 C5 A4 B4:4 A4:2 | D5:4 D5:2 E5 C5 A4 B4:4 A4:2 | \
             C5:2 D5 D5 E5 C5 A4 B4:4 A4:2 | A4:8 A4:4 A4:2 -:4",
            "i | bVII bVII bVII bVII i i i i i | i | i | i | bVII bVII bVII bVII i i i i i | i | i",
        ),
        Part::new(
            "C",
            "B4:2 C5 D5 C5 B4 G4 A4:4 B4:2 | C5:4 D5:2 C5 B4 G4 A4:6 | \
             B4:2 C5 D5 C5 B4 G4 A4:4 B4:2 | C5:4 C5:2 B4 C5 B4 G4 A4:4",
            "bVII bVII bVII bVII bVII bVII i i i | i i i i bVII bVII i i i | \
             bVII bVII bVII bVII bVII bVII i i i | i",
        ),
        Part::new(
            "D",
            "C5:4 D5:2 C5 D5:4 D5 C5:2 | D5:4 D5:2 C5 B4 G4 A4:4 B4:2 | \
             C5:4 D5:2 C5 D5:4 D5 C5:2 | E5:4 D5:2 C5 B4 G4 A4:6",
            "i i i i bVII bVII bVII bVII bVII | bVII bVII bVII bVII bVII bVII i i i | \
             i i i i bVII bVII bVII bVII bVII | i i i i bVII bVII i i i",
        ),
        Part::new(
            "E",
            "E5:4 D5:2 C5 B4:4 G4:2 A4 B4 | C5:4 D5:2 C5 B4 G4 A4:6 | \
             E5:4 D5:2 C5 B4:4 A4 B4:2 | C5:4 C5:2 B4 G4:4 A4:6",
            "i i i i bVII bVII bVII bVII bVII | i i i i bVII bVII i i i | \
             i i i i bVII bVII i i i | i i i i bVII bVII i i i",
        ),
        // F sharp at last, over D major: the dorian IV.
        Part::new(
            "F",
            "F#4:2 G4 A4:4 F#4:2 G4 A4:4 B4:2 | C5:4 B4:2 A4 D5:4 A4:6 | \
             F#4:2 G4 A4:4 F#4:2 G4 A4:4 B4:2 | C5:4 B4:2 G4 A4:4 A4:6",
            "IV | i i i i IV IV IV IV IV | IV | i i bVII bVII i i i i i",
        ),
    ],
    form: &["A", "A", "B", "B", "C", "C", "D", "D", "E", "E", "F", "F"],
    drums_from: 0,
    about: "Nine eighths to the bar that are not three times three: 2 + 2 + 2 + 3, quick, \
            quick, quick, slow. Six short strains, each played twice, circle around A and \
            leave out F until the last, which brings in F sharp over D major (IV): the sound \
            of the dorian mode. Most phrases fall to A through B and G, over G major (bVII) \
            and then A minor, the cadence of much Balkan music.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2890.abc
// (Henrik Norbeck's ABC tunes, kopanitsa 3, as Boris Karlov played it, in 2+2+3+2+2/16 with
// its chords: written here in eighths, every note twice as long. Five strains, each played
// twice. Compared bar by bar with John Chambers' "Gankino/Kopanica #2" in D minor,
// https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201297860.abc,
// the same tune in the same key. Chambers often holds a note where Norbeck repeats it or cuts
// it short (D for D D, or for D and a rest; E E for E E E D; G sharp for G sharp and a rest),
// holds E where Norbeck ends the bars of the fourth strain on the low B, starts the second bar
// of the third and of the fifth strain a fourth lower (B for E, E for A), has E for the high
// A of the last bar, and goes from E7 to A at the end of the first two strains, where Norbeck
// has E. His "fine" at the end of the fourth strain asks
// for a da capo that Norbeck does not have. Norbeck's notes and chords are followed, except
// in the last bar of the fourth strain: its E chord on the last two beats is missing there,
// and Chambers has it, as in the strain's other bars.)
pub(super) const GANKINO_HORO: Song = Song {
    name: "Gankino horo",
    origin: "Bulgarian kopanitsa, traditional",
    feel: "Kopanitsa",
    genre: Genre::Balkan,
    tonic: D,
    mode: Dorian,
    time: ELEVEN_EIGHT,
    tempo: 220.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Kopanitsa",
    band: BALKAN,
    style: "Kopanitsa",
    parts: &[
        Part::new(
            "A",
            "D4:4 C#5:2 D5 D5 -:2 D5 D5 C5 B4 A4 | C5:2 B4 A4 G#4 G#4 F4 A4 G#4:4 F4:2 E4 | \
             D4:4 E4:2 F4 F4 D4 A4 G#4:4 F4:2 A4 | G#4:4 F4:2 E4 E4 D4 F4 E4 E4 E4 D4",
            "i | i | i | i i i i i i i II II II II",
        ),
        Part::new(
            "B",
            "D4:4 E4:2 F4 F4 D4 A4 G#4:4 F4:2 E4 | D4:4 D5:2 D5 D5 C5 B4 A4 G#4 F4 E4 | \
             D4:4 E4:2 F4 F4 D4 A4 G#4:4 F4:2 A4 | G#4:4 F4:2 E4 E4 D4 F4 E4 E4 E4 D4",
            "i | i | i | i i i i i i i II II II II",
        ),
        Part::new(
            "C",
            "B4:2 C#5 D5 D5 D5 B4 D5 C#5 C#5 C#5 A4 | E5:2 C#5 D5 D5 D5 B4 D5 C#5 C#5 C#5 A4 | \
             B4:2 C#5 D5 D5 D5 B4 D5 C#5 C#5 C#5 A4 | B4:2 A4 G#4 A4 B4 B4 E5 B4:4 -:4",
            "V7 V7 V7 V7 V7 V7 V7 I I I I | V7 V7 V7 V7 V7 V7 V7 I I I I | \
             V7 V7 V7 V7 V7 V7 V7 I I I I | V",
        )
        .in_key(A, Major),
        // On E, with the notes of the first strains: E, F, G sharp, A, B, C, D.
        Part::new(
            "D",
            "A4:2 B4 G#4:4 F4:2 E4 D4 E4 E4 E4 B3 | A4:2 B4 G#4:4 F4:2 E4 D4 E4 E4 E4 B3 | \
             A4:2 B4 G#4:4 A4:2 B4 G#4 A4 B4 G#4 -:2 | A4:2 B4 G#4:4 F4:2 E4 D4 E4 E4 E4 B3",
            "II II II II i i i II II II II | II II II II i i i II II II II | II | \
             II II II II i i i II II II II",
        ),
        Part::new(
            "E",
            "E5:2 F#5 G5 G5 G5 E5 G5 F#5 F#5 F#5 D5 | A5:2 F#5 G5 G5 G5 E5 G5 F#5 F#5 F#5 D5 | \
             E5:2 F#5 G5 G5 G5 E5 G5 F#5 F#5 F#5 D5 | E5:2 D5 C#5 D5 E5 E5 A5 E5 -:6",
            "V7 V7 V7 V7 V7 V7 V7 I I I I | V7 V7 V7 V7 V7 V7 V7 I I I I | \
             V7 V7 V7 V7 V7 V7 V7 I I I I | V V V V II7 II7 II7 V V V V",
        )
        .in_key(D, Major),
    ],
    form: &["A", "A", "B", "B", "C", "C", "D", "D", "E", "E"],
    drums_from: 0,
    about: "Eleven eighths to the bar, counted 1 2, 1 2, 1 2 3, 1 2, 1 2: five beats with \
            the long one in the middle. The first two strains are in D on a minor scale with \
            B and G sharp: the raised fourth, G sharp, rubs against the D minor chord (i) and \
            makes the chord on E major (II), where each strain stops. The third goes to A \
            major, E7 to A; the fourth hovers on E over E major and D minor, the first \
            strains' notes heard from E, like the makam hicaz; the fifth is in D major, A7 to \
            D, and stops on A, the V, ready to start again.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201355994.abc
// (John Chambers' ABC of 1987, in bars of 8 + 7 eighths: here each pair is one bar of 15/8,
// 2 + 2 + 2 + 2 + 3 + 2 + 2, with his chords, without the rolls. The first two strains are
// marked "4x" and played four times, the others twice; the second ends at "fine", so after
// the seventh the tune goes back to the start and ends there, its repeats taken again. The
// fourth to sixth strains are in thirds: the upper voice. Compared with Henrik Norbeck's
// "Buchimish", in 2+2+2+2+3+2+2/16,
// https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/norbeck/abc_tune_norbeck_2848.abc:
// his first two strains are the same; his third ends its first bar on G for E and has D for
// the high E in its second; his last two strains are other tunes, where Chambers has four.)
pub(super) const BUCIMIS: Song = Song {
    name: "Bučimiš",
    origin: "Bulgarian dance tune, traditional",
    feel: "Bučimiš",
    genre: Genre::Balkan,
    tonic: A,
    mode: Dorian,
    time: FIFTEEN_EIGHT,
    tempo: 220.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Bučimiš",
    band: KAVAL,
    style: "Bučimiš",
    parts: &[
        Part::new(
            "A",
            "A4:4 D5:2 E5 E5 A4 E5 A4 E5:4 D5:2 D5 C5 D5 E5 | \
             C5:2 E4 B4 E4 C5 E4 B4 E4 G4 F#4 G4 A4:4 A4:2 E4",
            "i | i i i i i i i i bVII bVII bVII i i i i",
        ),
        Part::new(
            "B",
            "C5:2 E4 B4 E4 C5 E4 B4 E4 G4 F#4 G4 A4 B4 C5 D5 | \
             C5:2 E4 B4 E4 C5 E4 B4 E4 G4 F#4 G4 A4:4 A4:2 E4",
            "i i i i i i i i bVII bVII bVII i i i i | i i i i i i i i bVII bVII bVII i i i i",
        ),
        Part::new(
            "C",
            "C5:2 D5 D5 B4 C5 A4 B4 G4 A4:4 B4:2 C5:4 C5:2 E4 | \
             C5:2 D5 D5 B4 C5 A4 B4 G4 A4:4 E5:2 A4:4 A4",
            "i | i",
        ),
        // The makam hicaz on A, B flat and C sharp, over A major and G minor.
        Part::new(
            "D",
            "G5:2 F5 E5 D5 E5 C#5 D5 Bb4 C#5:2 D5 E5 C#5:4 C#5:2 Bb4 | \
             A4:2 Bb4 C#5 D5 E5 F5 E5:4 D5:2 C#5 Bb4 C#5:4 C#5:2 A4",
            "I | I I I I I I I I bvii bvii bvii I I I I",
        )
        .in_key(A, Phrygian),
        Part::new(
            "E",
            "C#5:2 D5 E5:4 E5:2 D5 E5:4 D5:2 C#5 Bb4 C#5:4 C#5:2 Bb4 | \
             A4:2 Bb4 C#5 D5 E5 F5 E5:4 D5:2 C#5 Bb4 C#5:4 C#5:2 A4",
            "I I I I I I I I bvii bvii bvii I I I I | I I I I I I I I bvii bvii bvii I I I I",
        )
        .in_key(A, Phrygian),
        Part::new(
            "F",
            "D5:2 E5 E5 A4 D5 E5 E5 A4 F5 E5 D5 C#5:4 Bb4:2 A4 | \
             D5:2 E5 E5 A4 D5 E5 E5 A4 F5 E5 D5 E5:4 E5:2 A4",
            "I I I I I I I I bvii bvii bvii I I I I | I I I I I I I I bvii bvii bvii I I I I",
        )
        .in_key(A, Phrygian),
        Part::new(
            "G",
            "D5:2 E4 C#5:4 D5:2 E4 C#5:4 A4:2 G#4 A4 B4 C#5 D5 E5 | \
             D5:2 E4 C#5:4 D5:2 E4 C#5:4 A4:2 C#5 A4 B4:4 B4",
            "I I I I I I I I I I I V7 V7 V7 V7 | I I I I I I I I II7 II7 II7 V V V V",
        )
        .in_key(A, Major),
    ],
    form: &[
        "A", "A", "A", "A", "B", "B", "B", "B", "C", "C", "D", "D", "E", "E", "F", "F", "G", "G",
        "A", "A", "A", "A", "B", "B", "B", "B",
    ],
    drums_from: 0,
    about: "Fifteen eighths to the bar in seven beats, 2 + 2 + 2 + 2 + 3 + 2 + 2, the long \
            one fifth. The first three strains are in A dorian, with F sharp, over A minor, \
            and G major (bVII) comes on the long beat. The next three move to A with B flat \
            and C sharp, the makam hicaz, over A major and G minor (bvii). The last turns to \
            A major and stops on E, the V, through B7 (II7): a half cadence that leads back \
            to the first strain, and the tune ends after the second.",
};
