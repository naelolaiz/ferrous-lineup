//! Blues: twelve-bar blues, boogie-woogie and minor blues.

use super::*;

const TWELVE_BAR: &str = "I7 | IV7 | I7 | I7 | IV7 | IV7 | I7 | I7 | V7 | IV7 | I7 | V7";

pub(super) const MIDNIGHT_SHUFFLE: Song = Song {
    name: "Midnight Shuffle",
    origin: "Written for Ferrous Lineup",
    feel: "Shuffle",
    genre: Genre::Blues,
    tonic: A,
    mode: Blues,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Shuffle,
    bass: Bass::Boogie,
    rhythm: "Shuffle",
    band: BLUES_HARP,
    style: "Twelve-bar blues",
    parts: &[Part::new(
        "Chorus",
        "-:4 C5:2 D5:2 E5:4 G5 | A5:2 G5:2 E5:4 D5 C5 | A4:12 -:4 | -:16 | \
         -:4 C5:2 D5:2 E5:4 G5 | A5:2 G5:2 E5:4 D5 C5 | A4:12 -:4 | -:16 | \
         E5:2 E5:2 G5:4 E5 D5 | D5:2 D5:2 C5:4 A4 C5 | A4:8 -:8 | -:4 E4:4 G4 G#4",
        TWELVE_BAR,
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "The twelve-bar blues, sung the way the blues always is: a line, the same line \
            again over the IV chord, and an answer. The tune uses the A blues scale over \
            major chords, and the clash is the sound of the blues.",
};

pub(super) const ROLLING_BOOGIE: Song = Song {
    name: "Rolling Boogie",
    origin: "Written for Ferrous Lineup",
    feel: "Boogie-woogie",
    genre: Genre::Blues,
    tonic: C,
    mode: Blues,
    time: COMMON,
    tempo: 140.0,
    comping: Comping::Charleston,
    bass: Bass::Boogie,
    rhythm: "Shuffle",
    band: PIANO,
    style: "Boogie-woogie",
    parts: &[Part::new(
        "Chorus",
        "G5:2 Bb5:2 G5:2 F5:2 Eb5:2 C5:2 -:4 | C5:4 Eb5:2 C5:2 F5:4 -:4 | \
         G5:2 Bb5:2 G5:2 F5:2 Eb5:2 C5:2 -:4 | C5:8 -:8 | \
         C6:2 Bb5:2 G5:2 F5:2 Eb5:4 F5:4 | F5:8 -:8 | \
         G5:2 Bb5:2 G5:2 F5:2 Eb5:2 C5:2 -:4 | C5:4 -:12 | \
         G5:2 F#5:2 G5:2 Bb5:2 G5:8 | F5:2 F#5:2 F5:2 Eb5:2 C5:8 | C5:2 Eb5:2 C5:12 | -:8 G4:4 Bb4",
        TWELVE_BAR,
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "Boogie-woogie piano: the left hand walks up and down each chord in a rolling \
            line (root, third, fifth, sixth, seventh) while the right hand plays short blues \
            riffs on top.",
};

pub(super) const MINOR_MOOD: Song = Song {
    name: "Minor Mood Blues",
    origin: "Written for Ferrous Lineup",
    feel: "Minor blues",
    genre: Genre::Blues,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Jazz ballad",
    band: BLUES_GUITAR,
    style: "Minor blues",
    parts: &[Part::new(
        "Chorus",
        "-:4 A4:2 C5 D5:4 E5 | G5:6 E5:2 D5:4 C5 | A4:12 -:4 | -:16 | \
         -:4 D5:2 F5 G5:4 A5 | C6:6 A5:2 G5:4 F5 | E5:12 -:4 | -:16 | \
         -:4 F5:2 A5 C6:4 Eb6 | D6:6 B5:2 G#5:4 E5 | A5:4 G5:2 E5 C5:4 A4 | B4:8 G#4",
        "i7 | i7 | i7 | i7 | iv7 | iv7 | i7 | i7 | bVI7 | V7 | i7 | V7",
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "The blues in a minor key, with a walking bass. In bar 9 the F7 chord (bVI7) \
            darkens everything before the E7, the V7, turns back home.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/%5BTraditional%5D/Careless_Love/Careless_Love.mxl
// (OpenEWLD lead sheet: two verses)
pub(super) const CARELESS_LOVE: Song = Song {
    name: "Careless Love",
    origin: "Traditional, American",
    feel: "Shuffle",
    genre: Genre::Blues,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 88.0,
    comping: Comping::Shuffle,
    bass: Bass::Boogie,
    rhythm: "Shuffle",
    band: BLUES_HARP,
    style: "Eight-bar blues",
    parts: &[
        Part::new(
            "Verse",
            "A4:12 F4:4 | E4 C4 E4 G4 | F4:16 | -:4 C5 B4 Bb4 | \
             A4:12 Bb4:4 | C5 C5 D5 C5 | G4:16 | -:4 E4 F4 G4 | \
             A4:12 Bb4:4 | C5:8 C#5 | D5 Bb4 | F4:12 G4:4 | \
             A4:12 F4:4 | E4 C4 E4 G4 | F4:16 | -:4 D5 C5 Bb4",
            "I | V7 | I | I | I | I | V7 | V7 | I | I | IV | IV | I | V7 | I | I V7",
        ),
        Part::new(
            "Verse 2",
            "A4:12 F4:4 | E4 C4:2 C4 E4:4 G4 | F4:16 | -:4 C4 F4 G4 | \
             A4:12 Bb4:4 | C5 C5 D5 C5 | G4:16 | -:4 C4 F4 G4 | \
             A4:12 Bb4:4 | C5:8 C#5 | D5 Bb4 | F4:12 G4:4 | \
             A4:12 F4:4 | E4 C4 E4 G4 | F4:8 E5 | F5:12 -:4",
            "I | V7 | I | I V7 | I | I | V7 | V7 | I | I | IV | IV | I | V7 | I | I",
        ),
    ],
    form: &["Verse", "Verse 2"],
    drums_from: 0,
    about: "A folk blues in sixteen bars. Each verse sings the same line three times and \
            then answers it: the first time it comes to rest on F, the second hangs on \
            C7, the V7, the third climbs through C sharp to B flat, the IV, and the answer \
            brings it home.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/%5BTraditional%5D/Frankie_and_Johnny/Frankie_and_Johnny.mxl
// (OpenEWLD lead sheet: two verses)
pub(super) const FRANKIE_AND_JOHNNY: Song = Song {
    name: "Frankie and Johnny",
    origin: "Traditional, American",
    feel: "Shuffle",
    genre: Genre::Blues,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::Shuffle,
    bass: Bass::Boogie,
    rhythm: "Shuffle",
    band: BLUES_HARP,
    style: "Twelve-bar blues",
    parts: &[
        Part::new(
            "Verse",
            "C4:2 D4 E4 A4:4 G4:2 A4:4 | C4:2 C4:6 -:8 | C4:2 D4 E4 A4:4 G4:2 A4:4 | C4 -:12 | \
             F4:2 G4 A4 C5:4 D5 A4:2 | C5 C5:8 -:2 A4 B4 | C5:4 C5 B4 A4 | G4 E4 E4:2 Eb4:4 D4:2 | \
             _:16 | _:2 - A4 G4 A4 G4 E4:4 | C4:16 | _:4 -:12",
            "I V+ | I V+ | I V+ | I I7 | IV | IV | IV #iv° | I VI7 | II7 | V7 | I IV7 | I V7",
        ),
        Part::new(
            "Verse 2",
            "C4:2 D4 E4 A4:4 G4:2 A4:4 | C4:2 C4:6 - C4:2 | C4 D4 E4 A4:4 G4:2 A4:4 | C4:12 -:4 | \
             F4:2 G4 A4 C5:4 D5 A4:2 | C5 C5:8 -:2 A4 B4 | C5:4 C5 B4 A4 | G4 E4 E4:2 Eb4:4 D4:2 | \
             _:16 | _:2 - A4 G4 A4 G4:6 | C5:16 | _:4 -:12",
            "I V+ | I V+ | I V+ | I I7 | IV | IV | IV #iv° | I VI7 | II7 iv6 | V7 | I IV7 | I",
        ),
    ],
    form: &["Verse", "Verse 2"],
    drums_from: 0,
    about: "A twelve-bar ballad blues. The G chord with a raised fifth (V+) rocks against \
            C in the opening bars, and the turn at the end runs A7, D7, G7: each chord the \
            V of the next, back to C. Listen for the blue note E flat sliding down to D.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Richard_M._Jones/Trouble_in_Mind_Blues/Trouble_in_Mind_Blues.mxl
// (OpenEWLD lead sheet in 4/4, written here in 12/8: each beat is a dotted
// quarter, a pair of eighths is played long-short, the triplets are eighths)
pub(super) const TROUBLE_IN_MIND: Song = Song {
    name: "Trouble in Mind",
    origin: "Richard M. Jones, 1924",
    feel: "Slow blues",
    genre: Genre::Blues,
    tonic: G,
    mode: Major,
    time: TWELVE_EIGHT,
    tempo: 96.0,
    comping: Comping::Triplets,
    bass: Bass::LongShort,
    rhythm: "Slow blues",
    band: SLOW_BLUES,
    style: "Slow blues",
    parts: &[
        Part::new("Pickup", "-:18 D4:2 E4 D4", "V7"),
        Part::new(
            "Verse",
            "B4:18 B4:4 A4:2 | _:18 B4:4 C5:2 | D5:6 D5 Bb4:4 A4:2 G4:4 C4:2 | _:18 G4:4 G4:2 | \
             B4:16 B4:2 B4:4 B4:2 | _:4 G4:2 A4:4 G4:2 Bb4:4 A4:2 G4:4 A4:2 | _:24 | \
             _:6 -:10 D4:2 E4:4 D4:2",
            "I | V7 | I7 | IV IV IV #iv° | I VI7 | II7 | V7sus4 | V7",
        ),
        Part::new(
            "Verse 2",
            "B4:18 B4:4 A4:2 | _:18 B4:4 C5:2 | D5:6 D5 Bb4:4 A4:2 G4:4 C4:2 | _:12 -:4 G4:2 A4:4 G4:2 | \
             B4:6 B4:4 G4:14 | _:6 A4:4 G4:2 Bb4:4 A4:2 G4:4 G4:2 | _:24 | _:6 -:18",
            "I | V7 | I7 | IV IV IV #iv° | I vi | II7 V7 | I IV7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse 2"],
    drums_from: 1,
    about: "An eight-bar blues, here in 12/8 so that each of its four slow beats splits in \
            three, the way blues singers swing it. The tune falls from D onto B flat, the \
            blue third, over G7, and the second half walks home through a chain of dominants, E7, \
            A7, D7, each the V of the next.",
};

// Source: https://abcnotation.com/tunePage?a=trillian.mit.edu%2F%7Ejc%2Fmusic%2Fabc%2Fmirror%2Fmusicaviva.com%2Fusa%2Fst-james-cm%2Fst-james-cm-1%2F0000
// (ABC transcribed by Frank Nordberg, musicaviva.com)
pub(super) const ST_JAMES_INFIRMARY: Song = Song {
    name: "St. James Infirmary",
    origin: "Traditional, American",
    feel: "Minor blues",
    genre: Genre::Blues,
    tonic: C,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 70.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Jazz ballad",
    band: BLUES_GUITAR,
    style: "Minor blues",
    parts: &[
        Part::new("Pickup", "-:12 C4:2 Eb4", "i"),
        Part::new(
            "Verse",
            "G4:4 G4:2 G4 F4:4 G4 | Eb4:2 C4:6 -:4 C4:2 Eb4 | G4 G4:6 C5 Ab4:2 | G4:8 -:6 F#4:2 | \
             G4:4 G4:2 G4 F4:4 G4:2 F4 | Eb4 C4:6 -:4 C4:2 D4 | Eb4 C4:4 Eb4:2 D4:4 Eb4:2 D4 | \
             C4:8 -:4 C4:2 Eb4",
            "i V7 | i | i iv | V | i V7 | i | bVI7 V7 | i",
        ),
    ],
    form: &["Pickup", "Verse", "Verse", "Verse"],
    drums_from: 1,
    about: "Eight slow bars in C minor. The first line sinks from G to C, the second \
            reaches up to the high C and stops on G, and the last slides from A flat 7 \
            (bVI7) down a half step to G7 (V7) before it comes home.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Lena_Arent-Ma_Rainey/See_See_Rider_Blues/See_See_Rider_Blues.mxl
// (OpenEWLD lead sheet: the twelve-bar chorus, first ending)
pub(super) const SEE_SEE_RIDER: Song = Song {
    name: "See See Rider",
    origin: "Traditional, recorded by Ma Rainey, 1924",
    feel: "Shuffle",
    genre: Genre::Blues,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 92.0,
    comping: Comping::Shuffle,
    bass: Bass::Boogie,
    rhythm: "Shuffle",
    band: BLUES_HARP,
    style: "Twelve-bar blues",
    parts: &[Part::new(
        "Chorus",
        "E5:12 D5:2 C5 | _ G4:14 | -:4 E5:2 E5:4 C5:2 G4 Bb4 | _:4 E5 D5 C5 | \
         - A4:3 D5:1 C5:2 A4:6 | -:4 Ab4:3 D5:1 C5:4 Ab4:2 G4 | _:16 | _:8 -:4 E5:2 E5 | \
         _:6 D5:2 C5 A4:6 | -:4 F5:3 E5:1 G4:4 A4:2 C5 | _:16 | _:8 -",
        "I | I | I | I7 | IV | iv | I IV I IV | I I ii7 vi7 | II9 | V7 | I I7 vi° iv | \
         I #iv° V7 V7",
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "A twelve-bar blues with two surprises: F minor (iv) right after F in the sixth \
            bar, and D9 in the ninth, where most blues have G7. The tune leans on the blue \
            notes, B flat over C and the A flat of F minor, and the last bars walk the \
            bass down, C, B flat, A, A flat.",
};

// Still under copyright: written in 1960 (Percy Mayfield died in 1984).
// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/PercyMayfieldHitTheRoadJack.abc
// (the Wikifonia lead sheet in C minor, the whole song as written there: chorus, chorus,
// first verse, chorus, the longer second verse, last chorus and the "repeat and fade" tag,
// written out twice. The lead sheet's chords are Cm | Ab7 G+7; the chart here is the
// Andalusian cadence Cm Bb | Ab G7, two beats each, as in the UkuTabs chord sheet
// (Am G F E7 in every section, https://ukutabs.com/es/r/ray-charles/hit-the-road-jack/) and
// the chords of the record in the LOTRO ABC transcription (Gm F Eb D, two beats each,
// https://raw.githubusercontent.com/ptigas/markovjam/master/abc/lotro-abc/data/rayhit1.abc).
// Melody checked against https://raw.githubusercontent.com/alvaromorales/abcplayer/master/sample_abc/jack.abc.)
pub(super) const HIT_THE_ROAD_JACK: Song = Song {
    name: "Hit the Road Jack",
    origin: "Percy Mayfield, 1960",
    feel: "R&B shuffle",
    genre: Genre::Blues,
    tonic: C,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 172.0,
    comping: Comping::Shuffle,
    bass: Bass::Quarters,
    rhythm: "Shuffle",
    band: SAX_SECTION,
    style: "Andalusian cadence",
    parts: &[
        Part::new("Pickup", "-:12 G4:3 Eb4:1", "V7"),
        Part::new(
            "Chorus",
            "F4:8 C4:6 C4:2 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | G4:4 G4:4 C5:4 C5:4 | \
             Eb5:4 Eb5:4 C5:4 G4:3 Eb4:1 | F4:8 C4:6 C4:2 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | \
             C4:16 | _:12 G4:3 Eb4:1",
            "i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7",
        ),
        // The same, with the pickup into a verse ("Woo", "Now").
        Part::new(
            "Chorus 2",
            "F4:8 C4:6 C4:2 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | G4:4 G4:4 C5:4 C5:4 | \
             Eb5:4 Eb5:4 C5:4 G4:3 Eb4:1 | F4:8 C4:6 C4:2 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | \
             C4:16 | _:12 C5:4",
            "i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7",
        ),
        // "Woman, oh woman, don't treat me so mean..."
        Part::new(
            "Verse 1",
            "C5:3 Bb4:1 G4:4 F4:3 Eb4:1 F4:4 | F4:3 Eb4:1 F4:4 G4:4 Eb4:3 Eb4:1 | \
             Gb4:3 Gb4:1 F4:4 Eb4:2 C4:2 C4:2 C4:2 | Eb4:4 C4:2 C4:8 C4:2 | \
             Eb4:4 F4:4 F4:4 Eb4:4 | F#4:2 G4:12 Eb4:2 | \
             F4:3 Eb4:1 F4:3 Eb4:1 F4:3 Eb4:1 C4:4 | C4:4 C5:4 C5:4 G4:3 Eb4:1",
            "i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7",
        ),
        // "Now baby, listen baby, don't ya treat me this-a way..."
        Part::new(
            "Verse 2",
            "C5:2 Bb4:2 G4:2 Eb4:2 F4:2 Eb4:2 F4:2 Eb4:2 | F4:2 Eb4:2 F4:2 Eb4:2 F4:2 G4:4 C4:2 | \
             G4:4 F4:4 Eb4:2 C4:2 C4:4 | Eb4:4 C4:4 C4:4 C5:4 | \
             C5:4 Bb4:2 Bb4:2 C5:4 G4:3 Eb4:1 | F4:2 Eb4:4 G4:4 C4:2 Eb4:4 | \
             G4:4 F4:4 F4:2 Eb4:2 Eb4:4 | F4:3 C4:1 C4:2 C4:6 C4:3 C4:1 | \
             Eb4:4 F4:3 Eb4:1 F4:4 Eb4:4 | F#4:2 G4:12 Eb4:2 | \
             F4:2 Eb4:2 F4:2 Eb4:2 F4:2 Eb4:2 C4:4 | C4:4 C5:4 C5:4 G4:3 Eb4:1",
            "i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7 | \
             i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7",
        ),
        // The last chorus and the tag, "don't you come back no more", twice.
        Part::new(
            "Chorus end",
            "F4:8 C4:6 C4:2 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | G4:4 G4:4 C5:4 C5:4 | \
             Eb5:4 Eb5:4 C5:4 G4:3 Eb4:1 | F4:8 C4:6 C4:2 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | \
             C4:16 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | C4:16 | Eb4:3 Eb4:1 F4:4 F4:4 Eb4:4 | C4:16",
            "i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7 | i bVII | bVI V7 | \
             i bVII | bVI V7 | i",
        ),
    ],
    form: &[
        "Pickup",
        "Chorus",
        "Chorus 2",
        "Verse 1",
        "Chorus 2",
        "Verse 2",
        "Chorus end",
    ],
    drums_from: 1,
    about: "Four chords walk down, C minor, B flat, A flat, G7, two beats each, and never \
            stop: the Andalusian cadence of flamenco under a rhythm-and-blues shuffle. The tune \
            leans on F against C minor, on \"road\", and on E flat against G7, on the \"no\" of \
            \"come back no more\"; the verses add a blue G flat on \"meanest\".",
};

// Public domain in the US (published in 1914), still under copyright in
// countries where it lasts 70 years after the author's death (W. C. Handy died in 1958).
// Source: https://raw.githubusercontent.com/flothesof/SongCreator/HEAD/MusicXML_files/wikifonia20100503/2648.xml
// (the Wikifonia lead sheet, in G as published: the tango introduction, the
// two twelve-bar blues choruses, the sixteen-bar strain in G minor and the
// last strain, with the repeat from the first blues chorus to the end. A tie
// missing at the start of the minor strain is put back, as in its other
// statements. Checked against the lead sheets at
// https://raw.githubusercontent.com/neonscribe/lilypond-lead-sheets/HEAD/Core/St%20Louis%20Blues%20-%20Ly%20Core%20-%20G.ly
// and https://raw.githubusercontent.com/red-jackets-jazzband/website/HEAD/static/songs/st_louis_blues.abc.
// The C chords are C7, as in the iReal changes: the B flat in the tune is
// their seventh.)
pub(super) const ST_LOUIS_BLUES: Song = Song {
    name: "St. Louis Blues",
    origin: "W. C. Handy, 1914",
    feel: "Swing",
    genre: Genre::Blues,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Quick-change blues",
    parts: &[
        Part::new("Pickup", "-:10 D5:2 D5 D5", "V7").in_key(G, HarmonicMinor),
        // The tango introduction, on the tune of the minor strain.
        Part::new(
            "Intro",
            "D5:2 D5:14 | -:2 C5:2 C5 C#5 D5 Bb4:6 | A4:16 | _:10 C5:2 C5 C5 | \
             C5:2 C5:14 | -:2 C5:2 C5 C#5 D5 A4:6 | G4:8 A4:8 | D5:8 C5:8",
            "i | iv | V7 | V7 | V7 | V7 | i II7 | V V7",
        )
        .in_key(G, HarmonicMinor),
        Part::new(
            "Blues",
            "B4:2 D5 B4 G4:10 | -:2 D#4:2 E4 G4 Bb4:6 A4:2 | G4:16 | _:12 -:4 | \
             -:2 E4:2 G4 A4:10 | -:2 E4:2 G4 A4 Bb4:6 A4:2 | G4:16 | _:8 -:8 | \
             F#4:2 A4 F#4 D4:10 | -:4 Bb4:2 Bb4 Bb4:6 A4:2 | G4:16 | _:8 -:8",
            "I7 | IV7 | I | I7 | IV7 | IV7 | I | I | V7 | V+ | I | I",
        ),
        // The second blues chorus, ending with the pickup into the minor strain.
        Part::new(
            "Blues 2",
            "B4:2 D5 B4 G4:4 G4:6 | -:2 D#4:2 E4 G4 Bb4:6 A4:2 | G4:16 | _:4 -:12 | \
             -:2 E4:2 G4 A4:4 A4:6 | -:2 D#4:2 E4 G4 Bb4:6 A4:2 | G4:16 | _:8 -:8 | \
             F#4:2 A4 F#4 D4:10 | -:4 Bb4:2 Bb4 Bb4:6 A4:2 | G4:16 | _:8 -:2 D5:2 D5 D5",
            "I7 | IV7 | I | I7 | IV7 | IV7 | I | I | V7 | V+ | I | I",
        ),
        // The sixteen bars in G minor, in the habanera rhythm in the score.
        Part::new(
            "Minor",
            "D5:2 D5:14 | -:2 C5:2 C5 C#5 D5 Bb4:6 | A4:16 | _:10 C5:4 C5:2 | \
             C5:2 C5:14 | -:4 C5:2 C#5 D5 A4:6 | G4:16 | _:8 -:4 D5:2 D5 | \
             D5:2 D5:14 | -:2 C5:2 C5 C#5 D5 Bb4:6 | A4:16 | _:8 -:4 C5:2 C5 | \
             C5:2 C5:4 C5:10 | -:4 C5:2 C#5 D5:4 A4 | G4:8 A4:8 | D5:12 Bb4:2 B4",
            "i | iv iv iv #iv°7 | V7 | V7 | V7 | V7 | i | i | \
             i | iv7 iv7 iv7 #iv°7 | V7 | V7 | V7 | V7 | i II7 | V7",
        )
        .in_key(G, HarmonicMinor),
        // The last strain, back in G major.
        Part::new(
            "Last",
            "G4:4 Bb4:2 B4 G4:4 Bb4:2 B4 | G4:4 Bb4:2 B4 G4 Bb4 B4 G4 | _:16 | _:12 Bb4:2 B4 | \
             A4:4 G4:2 E4 G4:4 A4:2 Bb4 | A4:4 G4:2 E4 G4 Bb4 B4 G4 | _:16 | _:12 Bb4:2 B4 | \
             D5:4 Bb4:2 B4 D4 Bb4 B4 C4 | _:2 Bb4 B4 D4:4 Bb4:2 B4 G4 | _:16 | _:16",
            "I I6 I I6 | I I6 I I6 | I I6 I I6 | I I6 I I6 | IV7 | IV7 | I I6 I I6 | I | \
             I | V7 | I | I",
        ),
    ],
    form: &[
        "Pickup", "Intro", "Blues", "Blues 2", "Minor", "Last", "Blues", "Blues 2", "Minor", "Last",
    ],
    drums_from: 1,
    about: "Handy's blues in strains, like a rag: a tango introduction in G minor, two \
            twelve-bar blues choruses with the IV chord already in the second bar, a \
            sixteen-bar strain in G minor that the score sets in the habanera rhythm, and a \
            last twelve-bar strain back in G major. Listen for the blue notes: B flat against \
            the G chord, and D sharp leaning up to E.",
};
