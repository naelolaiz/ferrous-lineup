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
// the long B flats. The tempo, 114 to the quarter, is the metronome mark of SymbTr's
// MusicXML version.)
pub(super) const ENTARISI: Song = Song {
    name: "Entarisi ala benziyor",
    origin: "Turkish folk song (türkü) from Istanbul",
    feel: "Raks aksağı",
    genre: Genre::Balkan,
    tonic: A,
    mode: Phrygian,
    time: RAKS_AKSAGI,
    tempo: 114.0,
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

// Source: https://raw.githubusercontent.com/MTG/SymbTr/246bbe362ef17cf9d200b5296e06473f29768be3/txt/huseyni--turku--curcuna--sari_gelin--erzurum.txt
// (SymbTr's "Erzurum çarşı pazar", makam hüseyni, usul curcuna, 10/8, from Erzurum: no. 615
// of the TRT folk song repertoire, collected by Muzaffer Sarısözen from the singer Faruk
// Kaleli, whom SymbTr's MusicXML names as its composer; the words are anonymous. All its 20
// bars, in order: the verse once and its refrain twice, as SymbTr writes them. Folk musicians
// count the curcuna 3 + 2 + 2 + 3 (see the "Curcuna" rhythm). Hüseyni is A, B, C, D, E, F
// sharp, G: its B (segâh) is a comma or two below the piano's B, one in SymbTr's AEU column,
// two in its 53-comma one, and its F sharp (eviç) a comma or two below the piano's F sharp;
// both are written as the nearest key, B and F sharp, which gives the Dorian mode. The grace
// notes are left out: an F after the E of "neynim" and "sarı", and a D, C or E struck before
// a note of the same pitch. The MusicXML version has the same notes, its B a quarter tone
// flat. The song is sung without chords; the chords here are added from the notes the tune
// leans on: A minor under its A's and C's, E minor 7 under its E's and D's, D7 under its D's
// and C's and the F sharp, E minor under a B on the beat.)
pub(super) const SARI_GELIN: Song = Song {
    name: "Sarı gelin",
    origin: "Turkish folk song (türkü) from Erzurum",
    feel: "Curcuna",
    genre: Genre::Balkan,
    tonic: A,
    mode: Dorian,
    time: CURCUNA,
    tempo: 80.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Curcuna",
    band: TURKISH,
    style: "Curcuna",
    parts: &[
        Part::new(
            "Erzurum",
            "-:4 A4:2 A4:4 A4 E5:6 | E5:4 D5:6 E5:2 D5 E5:6 | \
             E5:4 E5:2 E5:3 D5:1 D5:3 C5:1 D5:2 E5 -:2 | E5:4 F#5:2 D5:4 C5:3 B4:1 C5:2 D5 -:2 | \
             C5:4 D5:2 B4:4 A4 A4:2 C5 -:2 | B4:4 C5:2 A4:4 A4:10",
            "i | v7 | v7 | v7 v7 IV7 IV7 IV7 IV7 IV7 IV7 IV7 IV7 | i i i v v i i i i i | \
             v v v i i i i i i i",
        ),
        Part::new(
            "İçinde bir kız",
            "-:4 C5:2 C5:4 D5 D5:6 | D5:4 C5:6 D5:2 C5 D5 E5:4 | E5:2 -:2 C5:2 D5:4 E5 E5:6 | \
             E5:4 E5:2 D5:4 D5:3 C5:1 D5:2 E5 -:2 | E5:4 F#5:2 D5:4 C5:3 B4:1 C5:2 D5 -:2 | \
             C5:4 D5:2 B4:4 A4 A4:2 C5 -:2 | B4:4 C5:2 A4:4 A4:10",
            "IV7 | IV7 | i i i v7 v7 v7 v7 v7 v7 v7 | v7 | \
             v7 v7 IV7 IV7 IV7 IV7 IV7 IV7 IV7 IV7 | i i i v v i i i i i | v v v i i i i i i i",
        ),
    ],
    form: &["Erzurum", "İçinde bir kız", "İçinde bir kız"],
    drums_from: 0,
    about: "\"In the market of Erzurum a girl walks about\": the song of the fair-haired bride \
            (sarı gelin), sung slowly, like a lament, its lines answered by \"neynim aman \
            aman\" and \"suna yârim\", my graceful love. Ten eighths to the bar in the Turkish \
            curcuna, counted 1 2 3, 1 2, 1 2, 1 2 3: a long beat, two short ones and a long \
            one. The tune is in makam hüseyni on A: A, B, C, D, E, F sharp, G, close to the \
            Dorian mode. It climbs to E, the note hüseyni dwells on above A, touches the F \
            sharp and comes down to A. Turkish singers and players go in unison over a drum; \
            the chords here, A minor, E minor (7) and D7, the Dorian mode's major IV, are \
            added from the notes the tune leans on.",
};

// "İzmir'in kavakları" and "Servim senden uzun yok" share a tune.
const KAVAK_LINE: &str = "G4:2 B4:1 B4 B4:2 D5 C5 B4:1 A4 G4:6 | \
                          -:2 G5:1 G5 G5:2 G5:1 F#5 F#5:2 E5:1 D5 E5:2 F#5 E5:1 D5 | \
                          D5:4 D5:1 G5 F5 E5 D5 C5 B4 A#4 B4:3 C5:1 B4 A#4 | \
                          G4:2 B4:1 B4 B4:2 D5 C5 B4:1 A4 G4:6 | \
                          -:2 G5:1 G5 G5:2 G5:1 F#5 F#5:2 E5:1 D5 E5:2 F#5 E5:1 D5 | \
                          D5:4 D5:1 G5 F5 E5 D5 C5 B4 A#4 B4 C5 D5 E5 F#5 G5";
const KAVAK_LINE_CHORDS: &str = "bVI | bVI bVI bVI bVI bIII bIII bIII bIII bIII | \
                                 bIII bIII bVI bVI i i i i i | bVI | \
                                 bVI bVI bVI bVI bIII bIII bIII bIII bIII | \
                                 bIII bIII bVI bVI i i i i i";
// "Bize de derler Çakıcı" and "Kamalı zeybek vurulmuş" share a tune too, the one the
// instrumental turn plays.
const KAVAK_REFRAIN: &str = "G5:1 G5 G5 G5 G5:2 D5 E5 F#5 G5 F5 E5 | \
                             D5:3 E5:1 F5 E5:2 D5:1 D5 C5 C5 B4 E5:6 | \
                             -:2 D5:1 E5 F5:2 E5:1 D5 D5:2 C5:1 B4 B4 C5 D5:2 C5:1 B4 | \
                             B4:4 B4:1 G5 F5 E5 D5 C5 B4 A#4 B4 C5 D5 E5 F#5 G5 | \
                             G5:1 G5 G5 G5 G5:2 D5 E5 F#5 G5 F5 E5 | \
                             D5:3 E5:1 F5 E5:2 D5:1 D5 C5 C5 B4 E5:6 | \
                             -:2 D5:1 E5 F5:2 E5:1 D5 D5:2 C5:1 B4 B4 C5 D5:2 C5:1 B4 | \
                             B4:4 A#4 G4:2 A#4 B4:4 -:2";
const KAVAK_REFRAIN_CHORDS: &str = "bVI bVI bVI bVI iv iv iv iv iv | \
                                    bVI bVI bVI bVI bVI bVI iv iv iv | \
                                    bVI bVI bVI bVI i i i i i | i i bVI bVI i i i i i | \
                                    bVI bVI bVI bVI iv iv iv iv iv | \
                                    bVI bVI bVI bVI bVI bVI iv iv iv | \
                                    bVI bVI bVI bVI i i i i i | \
                                    i i vii°7 vii°7 vii°7 vii°7 i i i";

// Source: https://raw.githubusercontent.com/MTG/SymbTr/246bbe362ef17cf9d200b5296e06473f29768be3/txt/segah--turku--aksak--izmirin_kavaklari--.txt
// (SymbTr: makam segâh, usul aksak, 9/8 counted 2 + 2 + 2 + 3, as its long notes show; no.
// 337 of the TRT folk song repertoire, from İzmir, collected by Muzaffer Sarısözen from Ekrem
// Güyer, whom SymbTr's MusicXML names as its composer; the words are anonymous. All its 44
// bars, in order: the instrumental turn (aranağme), played twice, the verse's two lines, sung
// twice, and the refrain, sung twice; then all of it again with the second verse. Segâh rests
// on B a comma flat, under a comma below the piano's B; its E is a comma flat too, its F
// sharp (eviç) under a comma below the piano's and its A sharp within a comma of the piano's.
// Each is written as the nearest key: B, C, D, E, F sharp, G, A, the Phrygian mode on B, with
// F natural (acem) on the way down and the A sharp leading back to B from below. Four
// ornaments (a G touching the A above) are left out. The MusicXML version has the same notes,
// its B's and E's a quarter tone flat. The song is sung without chords; the chords here are
// added from the notes the tune leans on: G major under its G's, B's and D's, E minor under
// its E's, D major under F sharp and D, B minor under the B's it comes down to, and the
// diminished seventh on A sharp (A sharp, C sharp, E, G) under the A sharp and G of the last
// bar.)
pub(super) const IZMIRIN_KAVAKLARI: Song = Song {
    name: "İzmir'in kavakları",
    origin: "Turkish folk song (türkü) from İzmir",
    feel: "Aksak",
    genre: Genre::Balkan,
    tonic: B,
    mode: Phrygian,
    time: NINE_EIGHT_AKSAK,
    tempo: 60.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Aksak",
    band: TURKISH,
    style: "Aksak",
    parts: &[
        Part::new(
            "Aranağme",
            "F#5:1 G5:2 G5:1 G5:2 D5 E5 F#5 G5 F5 E5 | D5:3 E5:1 F5 E5:2 D5:1 D5 C5 C5 B4 E5:6 | \
             -:2 D5:1 E5 F5:2 E5:1 D5 D5:2 C5:1 B4 B4 C5 D5:2 C5:1 B4 | \
             B4:4 B4:1 G5 F5 E5 D5 C5 B4 A#4 B4 C5 D5 E5 F#5 G5 | \
             F#5:1 G5:2 G5:1 G5:2 D5 E5 F#5 G5 F5 E5 | D5:3 E5:1 F5 E5:2 D5:1 D5 C5 C5 B4 E5:6 | \
             -:2 D5:1 E5 F5:2 E5:1 D5 D5:2 C5:1 B4 B4 C5 D5:2 C5:1 B4 | \
             B4:4 B4:1 G5 F5 E5 D5 C5 B4 A#4 B4:3 C5:1 B4 A#4",
            "bVI bVI bVI bVI iv iv iv iv iv | bVI bVI bVI bVI bVI bVI iv iv iv | \
             bVI bVI bVI bVI i i i i i | i i bVI bVI i i i i i | \
             bVI bVI bVI bVI iv iv iv iv iv | bVI bVI bVI bVI bVI bVI iv iv iv | \
             bVI bVI bVI bVI i i i i i | i i bVI bVI i i i i i",
        ),
        Part::new("İzmir'in kavakları", KAVAK_LINE, KAVAK_LINE_CHORDS),
        Part::new("Bize de derler", KAVAK_REFRAIN, KAVAK_REFRAIN_CHORDS),
        Part::new("Servim senden", KAVAK_LINE, KAVAK_LINE_CHORDS),
        Part::new("Kamalı zeybek", KAVAK_REFRAIN, KAVAK_REFRAIN_CHORDS),
    ],
    form: &[
        "Aranağme",
        "İzmir'in kavakları",
        "Bize de derler",
        "Aranağme",
        "Servim senden",
        "Kamalı zeybek",
    ],
    drums_from: 0,
    about: "\"The poplars of İzmir shed their leaves; they call us Çakıcı, we burn down the \
            mansions\": the song of Çakıcı, the outlaw hero (efe) of the hills around İzmir. \
            Nine eighths to the bar in the Turkish aksak, counted 1 2, 1 2, 1 2, 1 2 3: three \
            short beats and a long one, slowly. The tune is in makam segâh, which rests on B: \
            B, C, D, E, F sharp, G, A, the Phrygian mode, with F natural on the way down and \
            an A sharp leading back up to B. The verse circles G, below and above; the refrain \
            falls from the high G, step by step, to B. Turkish singers and players go in \
            unison over a drum; the chords here are added from the notes the tune leans on: G \
            major (bVI), E minor (iv), D major (bIII) and B minor (i), and the diminished \
            seventh chord on A sharp (vii°7) under the last bar's A sharp and G.",
};

// Source: https://raw.githubusercontent.com/MTG/SymbTr/246bbe362ef17cf9d200b5296e06473f29768be3/txt/hicaz--turku--devrihindi--cayelinden_oteye--rize.txt
// (SymbTr: makam hicaz, usul devr-i hindî, 7/8 counted 3 + 2 + 2; no. 1765 of the TRT folk
// song repertoire, from Çayeli in Rize, collected by Yücel Paşmakçı from Dursun Tanyaş, whom
// SymbTr's MusicXML names as its composer; the words are anonymous. All its 48 bars, in
// order: "Çayeli'nden öteye" once, "Sırtındaki sepetun" twice, and all of it again. Hicaz as
// in Entarisi ala benziyor: its B flat, C sharp and the F sharp below G are four commas from
// B, C and F, which a twelve-tone semitone matches closely. Its mordents, trills and grace
// notes are left out. The MusicXML version has the same notes, but its key signature has the
// F sharp of hicaz's upper notes and its F's carry no natural; the txt gives them as F
// natural, 340 commas, the F of Entarisi, and so does this. The song is sung without chords;
// the chords here are added from the notes the tune leans on, often one to a beat: D minor
// under its F's and D's, A major under its E's, C sharps and A's, B flat under its B flats
// and the D's before them, and A7 under the G.)
pub(super) const CAYELINDEN_OTEYE: Song = Song {
    name: "Çayeli'nden öteye",
    origin: "Turkish folk song (türkü) from Rize, on the Black Sea",
    feel: "Devr-i hindi",
    genre: Genre::Balkan,
    tonic: A,
    mode: Phrygian,
    time: THREE_TWO_TWO,
    tempo: 120.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Devr-i hindi",
    band: KEMENCE,
    style: "Devr-i hindi",
    parts: &[
        Part::new(
            "Çayeli'nden öteye",
            "F5:2 E5 F5 E5:4 D5 | E5:4 F5:2 F5 E5 D5 E5 | C#5:1 D5:3 D5:2 C#5:4 Bb4 | \
             D5:4 D5:2 D5 C#5 Bb4 C#5 | Bb4:4 Bb4:2 A4:4 G4:1 F#4 G4:2 | Bb4:4 Bb4:2 A4:4 A4 | \
             A4:4 A4:2 A4:4 A4 | A4:4 -:2 A4:4 A4",
            "iv iv iv I I iv iv | I I I iv iv iv iv | iv iv iv I I bII bII | bII | \
             bII bII bII I I I7 I7 | bII bII bII I I I I | I | I",
        ),
        Part::new(
            "Sırtındaki sepetun",
            "F5:2 E5 F5 E5:1 F5 E5:2 D5:4 | E5:1 F5 E5:2 F5 F5 E5 D5 E5 | D5:4 D5:2 C#5:4 Bb4 | \
             D5:4 D5:2 D5 C#5 Bb4 C#5 | Bb4:4 Bb4:2 A4:4 G4:1 F#4 G4:2 | Bb4:4 Bb4:2 A4:4 A4 | \
             A4:4 A4:2 A4:4 A4 | A4:4 -:2 A4:4 A4",
            "iv iv iv I I iv iv | I I I iv iv iv iv | iv iv iv I I bII bII | bII | \
             bII bII bII I I I7 I7 | bII bII bII I I I I | I | I",
        ),
    ],
    form: &[
        "Çayeli'nden öteye",
        "Sırtındaki sepetun",
        "Sırtındaki sepetun",
        "Çayeli'nden öteye",
        "Sırtındaki sepetun",
        "Sırtındaki sepetun",
    ],
    drums_from: 0,
    about: "\"Let us go on past Çayeli, yâli yâli\", and \"let me be the porter of the basket \
            on your back\": a song of Rize, on the Black Sea, in its own dialect. Seven \
            eighths to the bar in the Turkish devr-i hindî, counted 1 2 3, 1 2, 1 2: a long \
            beat and two short ones. The tune is in makam hicaz on A: A, B flat, C sharp, D, \
            E, F, G, the Phrygian mode with a raised third, as in Entarisi ala benziyor. It \
            starts high, around E and F, and walks down through C sharp and B flat to A, with \
            the step and a half from B flat to C sharp. Turkish singers and players go in \
            unison over a drum; the chords here, D minor, A major and B flat, often one to a \
            beat, are added from the notes the tune leans on.",
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

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/robinson/abc_tune_robinson_7697.abc
// (Richard Robinson's tunebook, with its chords: three strains, each played twice with a first
// and a second ending, written out here in full. His notes place it, on a correspondent's word,
// in Dobrogea, and say that it came to him from Alistair Cochrane, who had it from an LP of the
// Swedish folk band Vargavinter, and that the name may not be right; no composer is named. The copies in John
// Chambers' collection (abc_tune_jc_201360585.abc) and on FolkTuneFinder are the same ABC, note
// for note. The first ending of the second strain is marked "A2": its notes have C sharp, so it
// is played as A. No tempo is given: 220 to the quarter is Henrik Norbeck's for his two
// geamparale, written in 7/16 at 220 eighths.)
pub(super) const GEAMPARAUA_DE_LA_MURFATLAR: Song = Song {
    name: "Geamparaua de la Murfatlar",
    origin: "Romanian geampara, Dobrogea",
    feel: "Rachenitsa",
    genre: Genre::Balkan,
    tonic: A,
    mode: Phrygian,
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
            "C#5:2 A4 C#5 A4 C#5 D5 E5 | D5:4 D5:2 C#5 Bb4 A4 G4 | C#5:2 A4 C#5 A4 C#5 D5 E5 | \
             D5:4 D5:2 C#5 Bb4 A4 G4 | C#5:2 A4 C#5 A4 C#5 D5 E5 | D5:4 D5:2 C#5 D5 E5 F5 | \
             G5:2 F5 F5 E5 D5 C#5 D5 | E5:4 E5:2 A4 E5 A4 A4 | \
             C#5:2 A4 C#5 A4 C#5 D5 E5 | D5:4 D5:2 C#5 Bb4 A4 G4 | C#5:2 A4 C#5 A4 C#5 D5 E5 | \
             D5:4 D5:2 C#5 Bb4 A4 G4 | G5:2 F5 F5 E5 D5 E5 D5 | C#5:4 C#5:2 A4 Bb4 A4 G4 | \
             A4:2 Bb4 Bb4 A4 G4 F4 G4 | A4:4 A4 A4:2 C#5 E5",
            "I | iv | I | iv | I | iv | bII | I | I | iv | I | iv | bvii | I | bII | I",
        ),
        Part::new(
            "B",
            "A5:4 A5 A4:2 C#5 A5 | G5:4 G5 A4:2 C#5 G5 | F5:4 F5:2 E5 D5 E5 D5 | \
             C#5:4 C#5:2 A4 Bb4 A4 G4 | F5:4 F5:2 E5 D5 E5 D5 | C#5:4 C#5:2 A4 Bb4 A4 G4 | \
             F5:4 F5:2 E5 D5 C#5 D5 | E5:4 E5 A4:2 C#5 E5 | \
             A5:4 A5 A4:2 C#5 A5 | G5:4 G5 A4:2 C#5 G5 | F5:4 F5:2 E5 D5 E5 D5 | \
             C#5:4 C#5:2 A4 Bb4 A4 G4 | F5:4 F5:2 E5 D5 E5 D5 | C#5:4 C#5:2 A4 Bb4 A4 G4 | \
             A4:2 Bb4 Bb4 A4 G4 F4 G4 | A4:4 A4:2 E4 A4 E4 E4",
            "I | I7 | iv | I | iv | I | bII | I | I | I7 | iv | I | iv | I | bII | I",
        ),
        Part::new(
            "C",
            "E5:4 D5:2 C#5 D5 E5 F5 | G5:2 F5 G5 A5 G5 F5 E5 | G5:2 F5 F5 E5 D5 E5 D5 | \
             C#5:4 C#5:2 A4 Bb4 A4 G4 | E5:2 F5 F5 E5 D5 E5 D5 | C#5:4 C#5:2 A4 Bb4 A4 G4 | \
             E5:2 F5 F5 E5 D5 C#5 D5 | E5:4 E5:2 A4 E5 A4 A4 | \
             E5:4 D5:2 C#5 D5 E5 F5 | G5:2 F5 G5 A5 G5 F5 E5 | G5:2 F5 F5 E5 D5 E5 D5 | \
             C#5:4 C#5:2 A4 Bb4 A4 G4 | E5:2 F5 F5 E5 D5 E5 D5 | C#5:4 C#5:2 A4 Bb4 A4 G4 | \
             A4:2 Bb4 Bb4 A4 G4 F4 G4 | A4:4 A4:2 E4 A4 E4 E4",
            "I | bvii | iv | I | iv | I | bII | I | I | bvii | iv | I | iv | I | bII | I",
        ),
    ],
    form: &["A", "B", "C"],
    drums_from: 0,
    about: "A geampara, a dance of Dobrogea, between the Danube and the Black Sea: seven \
            eighths to the bar, counted 1 2, 1 2, 1 2 3, quick, quick, slow, the same \
            2 + 2 + 3 as the Bulgarian rachenitsa. The tune is on A with B flat and C sharp, \
            an augmented second apart, the scale of the makam hicaz. A major (I) and D minor \
            (iv) take turns, G minor (bvii) comes in now and then, and every strain ends on B \
            flat (bII), a semitone above A, falling to A. Three strains, each played twice, \
            the second time with another ending.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201356092.abc
// (the ABC in John Chambers' collection, as Mihai and Alexandru David presented the dance at
// Gypsy Camp II, at its tempo. Every bar is six eighths and a half note, or three quarters and
// a half note, and the MIDI accents it asks for fall on the first eighth and, less strongly,
// on the seventh: 3 + 2. Five strains of four bars, the first four played twice, then the
// first once more; the first is in two voices a third apart, an octave below the others: the
// upper one. All the
// other copies the index lists, in the balk2 and jc_regional folders, abc_tune_jc_201358690.abc,
// abc_tune_jc_201359703.abc and abc_tune_misc_20.abc, are the same ABC; "Mindrele 2", in 6/8, is
// another tune. Its key signature is E minor's, but every strain ends on C: the tune is in C
// with F sharp, the Lydian mode. It has no chords: these are a plain harmonization, C, with D
// major (II) under the first strain's held F sharp, where the lower voice has D, G (V) under
// the second strain's G, A, B, and C7 (I7) where B flat comes in.)
pub(super) const MANDRELE: Song = Song {
    name: "Mândrele",
    origin: "Romanian folk dance",
    feel: "Five-four",
    genre: Genre::Balkan,
    tonic: C,
    mode: Lydian,
    time: FIVE_FOUR,
    tempo: 192.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Five-four",
    band: TARAF,
    style: "Five-four",
    parts: &[
        Part::new(
            "A",
            "E4:2 F#4 G4 A4 G4 E4 F#4:8 | G4:4 E4 C4 E4:8 | E4:2 F#4 G4 A4 G4 E4 F#4:8 | \
             G4:4 E4 C4 C4:8",
            "I I I II II | I | I I I II II | I",
        ),
        Part::new(
            "B",
            "G5:2 A5 B5 G5 A5 F#5 G5:8 | G5:4 E5 C5 E5:8 | G5:2 A5 B5 G5 A5 F#5 G5:8 | \
             G5:4 E5 C5 C5:8",
            "V | I | V | I",
        ),
        Part::new(
            "C",
            "G5:4 E5 C5 E5:8 | G5:4 E5 C5 Bb5:8 | G5:4 E5 C5 E5:8 | G5:4 E5 C5 C5:8",
            "I | I7 | I | I",
        ),
        Part::new(
            "D",
            "Bb5:4 A5 G5 Bb5:8 | G5:4 E5 C5 E5:8 | Bb5:4 A5 G5 Bb5:8 | G5:4 E5 C5 C5:8",
            "I7 | I | I7 | I",
        ),
        Part::new(
            "E",
            "E5:2 F#5 G5 A5 G5 F#5 G5:8 | G5:4 E5 C5 E5:8 | E5:2 F#5 G5 A5 G5 F#5 G5:8 | \
             G5:4 E5 C5 C5:8",
            "I | I | I | I",
        ),
    ],
    form: &["A", "A", "B", "B", "C", "C", "D", "D", "E", "A"],
    drums_from: 0,
    about: "A Romanian dance in five: five quarter notes to the bar, counted 1 2 3, 1 2, a \
            bar of three and a bar of two in one. Every bar moves over the three and holds a \
            half note over the two. The tune is in C, but with F sharp: the Lydian mode, whose \
            raised fourth sits on D major (II) in the first strain and falls back to C. The \
            second strain goes from G (V) to C; the third and fourth bring in B flat, which \
            turns C into C7 (I7); the last goes back to the plain C, and the first strain \
            closes the dance.",
};
