//! Soul, funk and R&B: hymns, gospel and grooves.

use super::*;

pub(super) const AMAZING_GRACE: Song = Song {
    name: "Amazing Grace",
    origin: "Hymn tune \"New Britain\", 1835",
    feel: "Gospel waltz",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: THREE_FOUR,
    tempo: 76.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Intro", "-:12 | -:8 D5:4", "I | V"),
        Part::new(
            "Verse",
            "G5:8 B5:2 G5:2 | B5:8 A5:4 | G5:8 E5:4 | D5:8 D5:4 | \
             G5:8 B5:2 G5:2 | B5:8 A5:4 | D6:12 | _:8 B5:4 | \
             D6:8 B5:2 G5:2 | B5:8 A5:4 | G5:8 E5:4 | D5:8 D5:4 | \
             G5:8 B5:2 G5:2 | B5:8 A5:4 | G5:12 | _:8 D5:4",
            "I | I | IV | I | I | I I V | V | V V I | I | I | IV | I | vi | V | I | I",
        ),
    ],
    form: &["Intro", "Verse", "Verse", "Verse"],
    drums_from: 1,
    about: "The whole melody uses five notes, G A B D E: the major pentatonic scale, which \
            is why it sounds so open.",
};

pub(super) const JOY_TO_THE_WORLD: Song = Song {
    name: "Joy to the World",
    origin: "Hymn by Isaac Watts, tune arranged by Lowell Mason, 1839",
    feel: "Motown",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Quarters,
    bass: Bass::Walking,
    rhythm: "Motown",
    parts: &[Part::new(
        "Verse",
        "C6:4 B5:3 A5:1 G5:6 F5:2 | E5:4 D5 C5:6 G5:2 | A5:6 A5:2 B5:6 B5:2 | \
         C6:6 C6:2 C6 B5 A5 G5 | G5:3 F5:1 E5:2 C6 C6 B5 A5 G5 | \
         G5:3 F5:1 E5:2 E5 E5 E5 E5 E5:1 F5 | G5:6 F5:1 E5 D5:2 D5 D5 D5:1 E5 | \
         F5:6 E5:1 D5 C5:2 C6:4 A5:2 | G5:3 F5:1 E5:2 F5 E5:4 D5 | C5:16",
        "I V7 | I | IV V | I | I | I | V7 | V7 IV | I V7 | I",
    )],
    form: &["Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "The first line is just the C major scale, walking down from the top. Here it \
            gets the Motown treatment: piano on every beat, a busy bass, and the snare on \
            all four.",
};

pub(super) const DORIAN_STRUT: Song = Song {
    name: "Dorian Strut",
    origin: "Written for Ferrous Loom",
    feel: "Funk",
    genre: Genre::Soul,
    tonic: E,
    mode: Dorian,
    time: COMMON,
    tempo: 102.0,
    comping: Comping::Funk,
    bass: Bass::Funk,
    rhythm: "Funk",
    parts: &[
        Part::new(
            "Riff",
            "E5:2 -:1 E5:1 G5:2 A5 -:2 B5:1 -:1 A5:2 G5 | E5:3 D5:1 E5:4 -:8 | \
             C#6:2 -:1 C#6:1 E6:2 C#6 -:2 B5:1 -:1 A5:2 G5 | A5:3 G5:1 E5:4 -:8",
            "i7 | i7 | IV7 | IV7",
        ),
        Part::new(
            "Break",
            "B5:1 B5 -:2 A5:1 A5 -:2 G5 E5 D5 E5 | -:4 E5:2 G5 E5:4 D5 | \
             C#5:2 E5 G5 B5 A5:4 G5 | E5:16",
            "i7 | i7 | IV7 | IV7",
        ),
    ],
    form: &["Riff", "Riff", "Break", "Riff", "Riff"],
    drums_from: 0,
    about: "Funk lives on two chords and a lot of rhythm. In E Dorian the IV chord is A7, \
            major, and its C sharp is the note that makes Dorian brighter than plain minor.",
};

pub(super) const SUNDAY_SEVENTH: Song = Song {
    name: "Sunday Seventh",
    origin: "Written for Ferrous Loom",
    feel: "Neo-soul",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Lazy,
    bass: Bass::Lazy,
    rhythm: "Neo soul",
    parts: &[
        Part::new(
            "Call",
            "-:4 A5:3 G5:1 E5:4 C5 | B4:6 D5:2 G#5:8 | -:2 E5:2 G5 A5 C6:4 B5:2 A5 | \
             G5:6 E5:2 Bb5:8",
            "IVmaj7 | III7 | vi7 | I7",
        ),
        Part::new(
            "Answer",
            "-:4 C6:3 A5:1 G5:4 E5 | D5:6 B4:2 G#4:8 | A4:4 C5 E5 G5 | E5:12 -:4",
            "IVmaj7 | III7 | vi7 | I7",
        ),
    ],
    form: &["Call", "Answer", "Call", "Answer"],
    drums_from: 0,
    about: "Lush seventh chords with two borrowed dominants: E7 pulls toward A minor and C7 \
            toward F. The tune follows them, with a G sharp over E7 and a B flat over C7.",
};

// Source: https://github.com/yetist/music-score/blob/e4732e0a659a3a017f7290d6217c9e455d8ed61f/abc/abc_dtrad/WADEWATR.abc
// (Erich Rickheit's "dtrad" ABC collection; in E minor there, up a minor third here.)
pub(super) const WADE_IN_THE_WATER: Song = Song {
    name: "Wade in the Water",
    origin: "African-American spiritual, published 1901",
    feel: "Neo-soul",
    genre: Genre::Soul,
    tonic: G,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 84.0,
    comping: Comping::Lazy,
    bass: Bass::Lazy,
    rhythm: "Neo soul",
    parts: &[
        Part::new(
            "Chorus",
            "G4:10 Bb4:4 G4:2 | Bb4:8 G4:8 | D5:10 F#5:4 D5:2 | G5:4 F5:4 D5:2 C5:2 Bb4:4 | \
             G4:10 Bb4:4 G4:2 | Bb4:8 G4:8 | G4:2 Bb4:2 G4:2 Bb4:2 G4:2 G4:4 D4:2 | F#4:8 G4:8",
            "i7 | i7 | V7 | i7 | i7 | i7 | iv7 i7 | V7 i7",
        ),
        Part::new(
            "Verse",
            "G5:3 F5:1 D5:4 D5:2 C5:2 Bb4:4 | G4:2 Bb4:2 D5:4 G4:1 Bb4:1 G4:6 | \
             G4:2 Bb4:2 G4:2 Bb4:2 G4:2 G4:4 D4:2 | F#4:8 G4:6 D5:2 | \
             G5:3 F5:1 D5:4 D5:2 C5:2 Bb4:4 | C5:2 Bb4:2 D5:4 C5:1 Bb4:1 G4:6 | \
             G4:2 Bb4:2 G4:2 Bb4:2 G4:2 G4:4 D4:2 | F#4:8 G4:8",
            "i7 | i7 | iv7 i7 | V7 i7 | i7 | i7 | iv7 i7 | V7 i7",
        ),
    ],
    form: &["Chorus", "Verse", "Chorus"],
    drums_from: 0,
    about: "A minor-key spiritual that sits on the home chord and rocks between it and the \
            iv. The leading note F sharp comes only at the cadences, \"God's gonna trouble the \
            water\", where it pulls the tune back to G.",
};

// Source: https://github.com/yetist/music-score/blob/e4732e0a659a3a017f7290d6217c9e455d8ed61f/abc/abc_dtrad/SWINGLOW.abc
// (in E there; cross-checked with the Campbell Folk School song book ABC,
// https://github.com/syntithenai/abc2book/blob/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201342421.abc,
// which gives the chords.)
pub(super) const SWING_LOW: Song = Song {
    name: "Swing Low, Sweet Chariot",
    origin: "African-American spiritual, 19th century",
    feel: "Soul",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Gospel,
    bass: Bass::Quarters,
    rhythm: "R&B",
    parts: &[
        Part::new(
            "Chorus",
            "B4:4 G4:8 B4:4 | G4:6 G4:2 E4:2 D4:6 | G4:2 G4:2 G4:2 G4:2 B4:2 B4:2 D5:4 | D5:16 | \
             E5:2 D5:2 B4:8 D5:4 | G4:6 G4:2 E4:2 D4:6 | G4:2 G4:2 G4:2 G4:2 B4:2 B4:2 A4:4 | \
             G4:12 B4:4",
            "I | IV I | I | V7 | I | IV I | I V7 | I",
        ),
        Part::new(
            "Verse",
            "D5:4 G4:2 E4:4 G4:2 G4:2 G4:2 | G4:2 G4:2 G4:4 E4:2 D4:6 | \
             G4:2 G4:2 G4:2 G4:2 B4:2 B4:2 D5:4 | D5:12 D5:4 | E5:2 D5:2 B4:4 B4:4 G4:4 | \
             G4:2 G4:2 G4:2 G4:2 E4:2 D4:6 | G4:2 G4:2 G4:2 G4:2 B4:2 B4:2 A4:4 | G4:16",
            "I | IV V7 | I | V7 | I | IV I | I V7 | I",
        ),
    ],
    form: &["Chorus", "Verse", "Chorus"],
    drums_from: 0,
    about: "The whole tune is the major pentatonic scale, G A B D E. \"Swing low\" starts \
            right on the beat, a falling third; \"comin' for to carry me home\" is the same \
            eight quick notes every time, landing once on the V and once at home.",
};

// Source: https://github.com/syntithenai/abc2book/blob/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201269726.abc
// (John Chambers, 2020, with chords; in G there, up a minor third here. Cross-checked with
// https://github.com/red-jackets-jazzband/website/blob/e9a2dfeb0221d627713ca6dbd3053e1c7fdcfe17/static/songs/down_by_the_riverside.abc,
// which has the chromatic C sharp on "the".)
pub(super) const DOWN_BY_THE_RIVERSIDE: Song = Song {
    name: "Down by the Riverside",
    origin: "African-American spiritual, published 1918",
    feel: "Motown",
    genre: Genre::Soul,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Gospel,
    bass: Bass::Walking,
    rhythm: "Motown",
    parts: &[
        Part::new("Pickup", "-:12 D5:2 C5:2", "I"),
        Part::new(
            "Verse",
            "D5:4 F4:8 G4:4 | Bb4:6 D5:10 | -:4 D5:4 D5:6 C#5:2 | D5:2 C5:4 Bb4:10 | \
             -:4 C5:4 C5:6 Bb4:2 | C5:2 Bb4:4 A4:10 | -:4 D5:4 D5:6 C#5:2 | D5:2 C5:4 Bb4:6 D5:2 C5:2 | \
             D5:4 F4:8 G4:4 | Bb4:6 D5:10 | -:4 D5:4 D5:6 C#5:2 | D5:2 C5:4 Bb4:10 | \
             D5:4 C5:8 Eb5:4 | D5:8 C5:8 | Bb4:16 | Bb4:16",
            "I | I | I | I | V7 | V7 | I | I | I | I | I | I | V7 | V7 | I | I7",
        ),
        Part::new(
            "Chorus",
            "-:4 Bb4:4 C5:4 D5:2 D5:2 | Eb5:2 Eb5:4 Eb5:6 Eb5:4 | Eb5:4 -:4 Bb4:4 C5:2 C5:2 | \
             D5:2 D5:4 D5:6 C5:4 | Bb4:4 -:4 Bb4:4 Bb4:2 Bb4:2 | D5:4 C5:12 | Bb4:8 C5:8 | D5:16 | \
             -:4 Bb4:4 C5:4 D5:2 D5:2 | Eb5:2 Eb5:4 Eb5:6 Eb5:4 | Eb5:4 -:4 Bb4:4 C5:2 C5:2 | \
             D5:2 D5:4 D5:6 C5:4 | Bb4:4 -:4 Bb4:4 Bb4:2 Bb4:2 | D5:4 C5:8 Eb5:4 | D5:8 C5:8 | Bb4:16",
            "I | IV | IV | I | I | V7 | V7 | I | I | IV | IV | I | I | V7 | V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus"],
    drums_from: 1,
    about: "A spiritual in cut time: two big beats a bar. Listen for the C sharp under \"down \
            by the riverside\", a half step below the D, and for the chorus, \"ain't gonna \
            study war no more\", which moves to the IV chord for the first time.",
};

// Source: https://github.com/kenakofer/hymnsinger/blob/0ac62b6560fb86f44dfa5a04417b445af1f68804/lilypond/songs/when-israel-was-in-egypts-land/when-israel-was-in-egypts-land.ly
// (John W. Work's setting, G minor; up a fourth here. The end of "Egypt's land" follows the
// Cyber Hymnal and Lomax readings, which agree against Work's.)
pub(super) const GO_DOWN_MOSES: Song = Song {
    name: "Go Down, Moses",
    origin: "African-American spiritual, published 1862",
    feel: "Soul",
    genre: Genre::Soul,
    tonic: C,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Gospel,
    bass: Bass::Quarters,
    rhythm: "R&B",
    parts: &[
        Part::new("Pickup", "-:12 G4:4", "i"),
        Part::new(
            "Verse",
            "Eb5:4 Eb5:4 D5:4 D5:4 | Eb5:4 Eb5:4 C5:8 | G4:4 G4:4 B4:2 B4:6 | C5:12 G4:4 | \
             Eb5:4 Eb5:4 D5:4 D5:4 | Eb5:4 Eb5:4 C5:8 | G4:4 G4:4 B4:2 B4:6 | C5:16",
            "i iv | i | i V7 | i | i iv | i | i V7 | i",
        ),
        Part::new(
            "Refrain",
            "C5:4 C5:12 | F5:4 F5:12 | G5:8 G5:6 F5:2 | G5:4 G5:4 F5:1 Eb5:1 C5:6 | \
             Eb5:2 C5:2 C5:8 Eb5:4 | C5:2 Bb4:2 G4:12 | G4:4 G4:4 B4:2 B4:6 | C5:12 G4:4",
            "i | iv | i V7 | i | iv7 | i V | i V7 | i",
        ),
    ],
    form: &["Pickup", "Verse", "Refrain", "Verse", "Refrain"],
    drums_from: 1,
    about: "The verse answers every line with \"let my people go\", a drop to the fifth and a \
            climb through the leading note B back to C minor. The refrain opens up: \"Go down\" \
            on the tonic, \"Moses\" a fourth higher on the iv chord.",
};

// Source: https://github.com/oriolcolomefont/The-Sound-of-AI-workshop/blob/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/AfricanAmericanSpiritualJoshuaFitTheBattleOfJericho.abc
// (Wikifonia lead sheet, D minor; up a fourth here. Cross-checked with
// https://github.com/red-jackets-jazzband/website/blob/e9a2dfeb0221d627713ca6dbd3053e1c7fdcfe17/static/songs/battle_of_jericho.abc.)
pub(super) const JOSHUA: Song = Song {
    name: "Joshua Fit the Battle of Jericho",
    origin: "African-American spiritual, 19th century",
    feel: "Funk",
    genre: Genre::Soul,
    tonic: G,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::Funk,
    bass: Bass::Funk,
    rhythm: "Funk",
    parts: &[
        Part::new(
            "Chorus",
            "G4:2 F#4:2 G4:2 A4:2 Bb4:2 A4:2 Bb4:2 C5:2 | D5:2 Bb4:4 D5:10 | C5:2 A4:4 C5:10 | \
             D5:2 Bb4:4 D5:10 | G4:2 F#4:2 G4:2 A4:2 Bb4:2 A4:2 Bb4:2 C5:2 | \
             D5:2 Bb4:4 D5:6 Bb4:2 C5:2 | D5:4 C5:4 Bb4:4 A4:4 | G4:8 -:4 G5:2 G5:2",
            "i | i | V7 | i | i | i | iv V7 | i",
        ),
        Part::new(
            "Verse",
            "G5:2 D5:2 D5:2 D5:2 D5:4 D5:4 | D5:2 D5:2 D5:8 D5:2 D5:2 | \
             G5:2 D5:2 D5:2 D5:2 D5:4 D5:4 | D5:8 -:4 G5:2 G5:2 | G5:4 D5:4 D5:2 D5:6 | \
             D5:2 D5:2 D5:8 Bb4:2 C5:2 | D5:2 D5:2 C5:4 Bb4:4 A4:4 | G4:8 -:8",
            "i | V7 | i | V7 | i | V7 | iv V7 | i",
        ),
    ],
    form: &["Chorus", "Verse", "Chorus"],
    drums_from: 0,
    about: "The first bar circles the tonic, G F sharp G A, then climbs the minor scale: \
            \"Joshua fit the battle\". Each \"Jericho\" is a leap of a third and back, over i, \
            then V7, then i. The verse hammers one note, D, the fifth, like a preacher.",
};

// Source: https://github.com/syntithenai/abc2book/blob/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201360087.abc
// (Frank Nordberg's Musica Viva transcription, D minor, with chords; up a fourth here.
// The half-note triplets on "motherless" are written 3 + 3 + 2 sixteenths.)
pub(super) const MOTHERLESS_CHILD: Song = Song {
    name: "Sometimes I Feel Like a Motherless Child",
    origin: "African-American spiritual, 19th century",
    feel: "Neo-soul",
    genre: Genre::Soul,
    tonic: G,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Lazy,
    bass: Bass::Lazy,
    rhythm: "Neo soul",
    parts: &[
        Part::new(
            "Verse",
            "Bb4:2 D5:4 Bb4:2 D5:4 Bb4:2 C5:2 | D5:3 C5:3 Bb4:2 G4:8 | \
             Bb4:2 C5:4 Bb4:2 C5:4 Bb4:2 C5:2 | D5:3 C5:3 Bb4:2 G4:8 | \
             Bb4:2 D5:4 Bb4:2 D5:4 Bb4:2 C5:2 | D5:3 C5:3 Bb4:2 G4:6 G4:2",
            "i | bVI i | iv7 bVI | V7 i | iv bVI | V7 IV7",
        ),
        Part::new(
            "Refrain",
            "Bb4:8 G4:2 D4:4 G4:2 | A4:6 G4:2 Bb4:6 G4:2 | Bb4:8 G4:2 D4:4 F#4:2 | G4:16",
            "bVI i | iv6 #iv°7 | i V7 | i",
        ),
        Part::new(
            "True believer",
            "G5:6 F5:2 D5:1 Bb4:1 G4:4 G4:2 | Bb4:8 G4:2 D4:4 G4:2 | A4:6 G4:2 Bb4:6 G4:2 | \
             Bb4:8 G4:2 D4:4 F#4:2 | G4:16",
            "i bVI | bVI i | iv6 #iv°7 | i V7 | i",
        ),
    ],
    form: &["Verse", "Refrain", "True believer"],
    drums_from: 0,
    about: "The verse rocks between B flat and D over the home chord and its relative major, \
            E flat. \"A long ways from home\" falls to the low D and climbs back through C \
            minor and a C sharp diminished chord, the passing chord church pianists love.",
};

// Source: https://github.com/syntithenai/abc2book/blob/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201359040.abc
// (Musica Viva transcription in F, with chords; up a fifth here. Cross-checked with the
// four-part setting at
// https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmTLcnmZhwCdnSuXPwATeU6XJU3Smyniyo4SHRS3XHbE6h.abc,
// whose pickup "Some-" on the third is used here.)
pub(super) const NOBODY_KNOWS: Song = Song {
    name: "Nobody Knows the Trouble I've Seen",
    origin: "African-American spiritual, published 1867",
    feel: "Soul",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 66.0,
    comping: Comping::Gospel,
    bass: Bass::Quarters,
    rhythm: "R&B",
    parts: &[
        Part::new(
            "Chorus",
            "E5:2 G4:4 A4:2 C5:6 D5:2 | E5:2 E5:2 E5:2 E5:6 -:4 | E5:2 G4:4 A4:2 C5:4 C5:4 | \
             A4:2 G4:10 -:4 | E5:2 G4:4 A4:2 C5:6 D5:2 | E5:2 E5:2 E5:2 E5:6 -:4 | \
             G5:6 E5:2 D5:4 E5:4 | C5:8 C5:4 E5:4",
            "I IV | I | I IV | V7 | I IV | I | I V7 | IV I",
        ),
        Part::new(
            "Verse",
            "G5:4 G5:4 G5:4 E5:4 | G5:4 G5:4 E5:4 -:4 | G5:8 E5:8 | D5:8 -:4 E5:4 | \
             G5:4 G5:4 G5:4 E5:4 | G5:4 G5:4 E5:4 -:4 | E5:8 D5:8 | C5:12 -:4",
            "I | I | I | V7 | I | I | I V7 | I",
        ),
    ],
    form: &["Chorus", "Verse", "Chorus"],
    drums_from: 0,
    about: "\"Nobody\" starts on the high E and drops a sixth to G before climbing back: the \
            whole chorus lives in those two octaves of the major pentatonic. The verse, \
            \"sometimes I'm up, sometimes I'm down\", is almost spoken on G and E.",
};

// Source: https://github.com/syntithenai/abc2book/blob/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201358720.abc
// (Musica Viva transcription in D, with chords; up a fourth here.)
pub(super) const MICHAEL_ROW: Song = Song {
    name: "Michael, Row the Boat Ashore",
    origin: "African-American spiritual, published 1867",
    feel: "Motown",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Gospel,
    bass: Bass::Walking,
    rhythm: "Motown",
    parts: &[
        Part::new("Pickup", "-:8 G4:4 B4:4", "I"),
        Part::new(
            "Verse",
            "D5:6 B4:2 D5:2 E5:6 | D5:8 B4:4 D5:4 | E5:16 | D5:4 -:4 B4:4 D5:4 | \
             D5:6 B4:2 C5:2 B4:6 | A4:8 G4:4 A4:4 | B4:8 A4:8 | G4:8 G4:4 B4:4",
            "I | I | IV | I | iii | ii | V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse", "Verse"],
    drums_from: 1,
    about: "A rowing song from the Sea Islands of Georgia, written down in 1867. \"Michael\" \
            is the pickup, G up to B, and \"row\" lands on the beat. The second half walks \
            down the chords iii, ii, V7, I under a tune that only steps.",
};

// Source: https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmXN7GTpDKgBuHRbifS6oBm3afQ5yDNRsAoJhsdaxN2f5Z.abc
// (Cyber Hymnal four-part score, D major; up a fourth here. Three of its bars have the
// wrong length; they were corrected against the duet setting at
// https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmdgSPdUQ4Jq3AVi3ScN4iDFUvYwUArVZRSm36v7dnDwEe.abc.)
pub(super) const HIS_EYE_IS_ON_THE_SPARROW: Song = Song {
    name: "His Eye Is on the Sparrow",
    origin: "Charles H. Gabriel, words by Civilla D. Martin, 1905",
    feel: "Gospel",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 72.0,
    comping: Comping::Arp3,
    bass: Bass::Held,
    rhythm: "Six-eight",
    parts: &[
        Part::new(
            "Verse",
            "D5:2 E5:2 D5:2 B4:4 G4:2 | A4:2 B4:10 | C5:2 D5:2 E5:2 G5:4 G5:2 | D5:10 -:2 | \
             D5:2 C5:2 D5:2 E5:4 B4:2 | D5:2 C5:10 | C5:2 B4:2 A4:2 D5:4 D5:2 | B4:10 D5:2 | \
             G5:4 F#5:2 A5:2 G5:2 B4:2 | D5:2 D5:8 D5:2 | G5:4 F#5:2 A5:2 G5:2 D5:2 | E5:10 E5:2 | \
             F#5:4 F#5:2 F#5:2 E5:2 F#5:2 | G5:2 D5:4 D5:4 B4:1 B4:1 | D5:4 E5:2 D5:4 E5:2 | \
             D5:10 D5:2 | G5:4 G5:2 G5:4 G5:2 | G5:2 G4:8 G4:1 G4:1 | D5:4 B4:2 B4:2 A4:4 | \
             G4:10 D5:2",
            "I | I | I | V7 | III7 | ii | V7 | I | I | I #i°7 ii7 V7 | I | IV | V7 | I #i°7 | \
             ii7 V7 | I | I7 | IV #iv°7 | I V7 | I",
        ),
        Part::new(
            "Refrain",
            "D5:4 F#5:2 A5:4 D5:2 | D5:2 G5:8 D5:2 | D5:4 F#5:2 A5:4 D5:2 | B5:10 G5:1 G5:1 | \
             G5:4 G5:2 G5:4 G5:2 | G5:2 G4:8 G4:1 G4:1 | D5:4 B4:2 B4:2 A4:4 | G4:12",
            "V7 | I | V7 | I | I7 | IV #iv°7 | I V7 | I",
        ),
    ],
    form: &["Verse", "Refrain"],
    drums_from: 0,
    about: "A gospel hymn in a lilting 6/8. The verse borrows a B7, the dominant of the vi \
            chord, on \"why should my heart be lonely\". \"His eye is on the sparrow\" leaps \
            an octave down and back, and the I7 under it leads to the IV and a passing \
            diminished chord.",
};

// Source: https://github.com/kjpye/Songs/blob/45e9718e80bf30cf901271ddc2ed12dd263ebe1c/Serendipity/DeepRiver.ly
// (four-part setting in E flat, up a major third here; the melody agrees note for note with
// the MuseScore arrangement in the PDMX set at
// https://github.com/Rilay9/PianoProject/blob/c8b33af64d75c07a8c15efe08f15959d458c50d0/docs/review/pdmx-quarry-2026-10-05/xml/D-hymns/deep-river-african-american-spiritual-QmbP96wZt5Vev9pA8pembvkaMw2wx48M3PR3pWNMfJuASp.musicxml)
pub(super) const DEEP_RIVER: Song = Song {
    name: "Deep River",
    origin: "African-American spiritual, 19th century",
    feel: "Neo-soul",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 66.0,
    comping: Comping::Ballad,
    bass: Bass::Held,
    rhythm: "Neo soul",
    parts: &[
        Part::new(
            "Verse",
            "B4:10 A4:2 G4:4 | A4:4 E4:8 G4:4 | G4:4 G4:4 G5:6 E5:2 | D5:4 B4:8 A4:4 | \
             B4:10 A4:2 G4:4 | A4:4 E4:4 G4:6 G4:2 | G4:3 G4:1 B4:4 A4:3 G4:1 E4:3 E4:1 | G4:4 G4:12",
            "I | IV | I | I V7 | I | IV | vi vi ii V7 | I",
        ),
        Part::new(
            "Chorus",
            "E5:2 E5:4 F#5:2 E5:2 D5:2 B4:4 | D5:4 B4:8 D5:3 D5:1 | E5:4 G5:4 F#5:4 E5:4 | \
             D5:4 B4:8 D5:4 | E5:4 G5:4 F#5:4 E5:4 | D5:4 B4:8 D5:4 | E5:4 G5:4 F#5:4 E5:4 | \
             D5:4 B4:8 D5:4 | B4:3 B4:1 B4:3 B4:1 B4:2 A4:2 G4:2 A4:2 | B4:2 A4:4 G4:2 B4:2 A4:4 G4:2 | \
             B4:10 A4:2 G4:2 A4:2 | B4:4 D5:12",
            "vi | iii | vi | iii | vi II7 | iii | vi | iii | I I IV vi | IV ii | II7 | V7",
        ),
    ],
    form: &["Verse", "Verse", "Chorus", "Verse"],
    drums_from: 0,
    about: "\"Deep river\" is a long third of the scale sighing down to the tonic. The chorus, \
            \"Oh, don't you want to go\", moves to the relative minor and back, then \
            borrows an A7, the dominant of the dominant, to lead to D7 and home.",
};

// Source: https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmY7eS8vGDLags6j6THkiP1CLhBju24aQm3uZHVu8LXZKs.abc
// (Hymnary.org four-part score, B flat; up a tone here. Other lead sheets differ in small
// ways in bars 3 and 5; this follows the hymnal.)
pub(super) const CLOSER_WALK: Song = Song {
    name: "Just a Closer Walk with Thee",
    origin: "American gospel hymn, traditional",
    feel: "Gospel",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 84.0,
    comping: Comping::Gospel,
    bass: Bass::Walking,
    rhythm: "Swing",
    parts: &[Part::new(
        "Verse",
        "G4:6 G#4:2 A4:2 E4:2 G4:2 F#4:2 | F4:16 | D5:6 C5:2 B4:2 G4:2 A4:2 C5:2 | A4:2 G4:14 | \
         E5:6 E5:2 E5:2 C5:2 D5:2 E5:2 | D5:2 C5:10 C5:2 A4:2 | G4:4 G4:4 G4:4 A4:2 B4:2 | C5:16",
        "I | ii7 V7 | V7 | I | I7 | IV #iv°7 | I V7 | I",
    )],
    form: &["Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "A New Orleans favourite, played slow on the way to the cemetery and fast on the \
            way back. \"Thou art strong\" lands on F over the ii chord, a long suspension \
            before G7; \"as long\" is the I7 that pulls to the IV and a passing diminished \
            chord.",
};

// Source: https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmX5XaptYZ12youWdawmnKjjxaNYWnLDahiJw96QgVTj4i.abc
// (Cyber Hymnal four-part score of Gabriel's 1907 hymn, A flat; up a major third here.
// This is Gabriel's tune, not the 1935 Carter Family rewrite.)
pub(super) const WILL_THE_CIRCLE: Song = Song {
    name: "Will the Circle Be Unbroken",
    origin: "Charles H. Gabriel, words by Ada R. Habershon, 1907",
    feel: "Motown",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Gospel,
    bass: Bass::Walking,
    rhythm: "Motown",
    parts: &[
        Part::new("Pickup", "-:12 E4:3 F4:1", "I"),
        Part::new(
            "Verse",
            "G4:4 G4:8 C5:3 D5:1 | E5:4 E5:8 E5:3 D5:1 | C5:6 D5:2 C5:4 A4:4 | G4:12 E4:3 F4:1 | \
             G4:4 G4:8 C5:3 D5:1 | E5:4 E5:8 C5:3 C5:1 | D5:6 D5:2 D5:4 E5:4 | D5:12 C5:3 D5:1",
            "I | I | IV | I | I | I | V7 II7 | V7",
        ),
        Part::new(
            "Refrain",
            "E5:4 G5:8 E5:3 D5:1 | C5:4 E5:8 C5:3 B4:1 | A4:6 D5:2 C5:4 A4:4 | G4:12 G4:3 G4:1 | \
             A4:4 C5:8 C5:3 D5:1 | E5:4 E5:8 E5:3 F5:1 | G5:12 E5:3 D5:1 | C5:12 E4:3 F4:1",
            "I | I | IV | I | IV I | I | I V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Refrain", "Verse", "Refrain"],
    drums_from: 1,
    about: "Gabriel's hymn from 1907, before the Carter Family rewrote it. Every line starts \
            with a dotted pickup, long-short. \"Join them in their bliss\" lands on a D7, the \
            dominant of the dominant, so the refrain can come in on G7 and home.",
};

// Source: https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmbHJ74DLvHdhiottPP2Gkcd1rHPSXnKCWppMiqDyayja5.abc
// (Hymnary.org four-part score of HAPPY DAY, in F and 3/2; up a tone here, and each bar of
// 3/2 written as two bars of 3/4. The refrain's opening "Happy day, happy day" is left out
// so the song fits in 32 bars; it comes back as its last eight.)
pub(super) const O_HAPPY_DAY: Song = Song {
    name: "O Happy Day",
    origin: "Hymn by Philip Doddridge, 1755; tune and refrain attributed to Edward F. Rimbault, 1854",
    feel: "Gospel waltz",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: THREE_FOUR,
    tempo: 112.0,
    comping: Comping::Waltz,
    bass: Bass::Waltz,
    rhythm: "Waltz",
    parts: &[
        Part::new("Pickup", "D4:4 G4:4 A4:4", "V I V"),
        Part::new(
            "Verse",
            "B4:12 | D4:4 G4:4 A4:4 | B4:12 | B4:4 C5:4 B4:4 | A4:12 | G4:4 B4:4 A4:4 | G4:12 | \
             D4:4 G4:4 A4:4 | B4:12 | D4:4 G4:4 A4:4 | B4:12 | B4:4 C5:4 B4:4 | A4:12 | \
             G4:4 B4:4 A4:4 | G4:12 | D5:4 D5:4 D5:4",
            "I | V I V | I | I IV I | V7 | vi V V7 | I | V I V | I | V I V | I | I IV I | V7 | \
             vi V V7 | I | I",
        ),
        Part::new(
            "Refrain",
            "D5:12 | D5:4 E5:4 D5:4 | B4:12 | D5:4 D5:4 D5:4 | D5:12 | D5:4 E5:4 D5:4 | B4:12 | \
             -:4 B4:6 C5:2 | D5:12 | _:4 G4:6 A4:2 | B4:12 | B4:4 C5:4 B4:4 | A4:12 | \
             G4:4 B4:4 A4:4 | G4:12 | D4:4 G4:4 A4:4",
            "I | I IV I | I | I | I | I IV I | I | I | I | I I V | I | I IV I | V7 | vi V V7 | I | \
             V I V",
        ),
    ],
    form: &["Pickup", "Verse", "Refrain"],
    drums_from: 1,
    about: "Not the 1967 gospel arrangement but the hymn it grew from, in its original three. \
            Every long note falls on a downbeat and the three short ones before it, \"O \
            happy\", fill the bar before. \"Happy day\" in the refrain is a whole note that \
            hangs over the bar line.",
};

// Source: https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmcrnJ2b9XrAey8CPrVRj5xn55Tvy4dfZe1aFr6FVTHBBs.abc
// (Cyber Hymnal four-part score in F. Cross-checked with Rodeheaver's Plantation Melodies,
// https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmT2gRkQUcQfdjW2k7arJp1BpWTdPZ8mqg4EYxe3p6jsG1.abc,
// which gives the verse harmony used here.)
pub(super) const STEAL_AWAY: Song = Song {
    name: "Steal Away",
    origin: "African-American spiritual, 19th century",
    feel: "Spiritual",
    genre: Genre::Soul,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 63.0,
    comping: Comping::Ballad,
    bass: Bass::Held,
    rhythm: "Neo soul",
    parts: &[
        Part::new(
            "Chorus",
            "F4:6 F4:2 F4:8 | A4:6 A4:2 A4:8 | C5:6 C5:2 C5:6 D5:2 | G4:4 A4:12 | \
             F4:6 F4:2 F4:8 | D5:2 C5:2 A4:4 C5:6 F4:2 | F4:4 F4:4 A4:6 A4:2 | G4:4 F4:12",
            "I | I | I | V7 I | I | I V7 | vi V7 | V7 I",
        ),
        Part::new(
            "Verse",
            "D5:8 D5:6 C5:2 | D5:4 C5:8 A4:4 | C5:6 A4:2 C5:6 A4:2 | C5:4 A4:8 F4:4 | \
             A4:2 C5:6 C5:6 C5:2 | D5:2 C5:2 A4:4 C5:6 F4:2 | F4:4 F4:4 A4:6 A4:2 | G4:4 F4:12",
            "IV | IV I | I | I | I | I V7 | vi V7 | V7 I",
        ),
    ],
    form: &["Chorus", "Verse", "Chorus"],
    drums_from: 0,
    about: "\"Steal away\" climbs the F major chord one note per bar, F, A, C, each sung \
            long-short-long. The verse, \"my Lord, He calls me\", is sung in unison in the old \
            books, on the high D and C over the IV chord.",
};

// Source: https://github.com/mzealey/openhymnal/blob/d35811b2d5c9b0bbc5887d60a2936454815416e8/Complete/Were_You_There/Were_You_There-Were_You_There.abc
// (Open Hymnal Project, from The Hymnal 1940, in E with free barring; up a minor third here.
// The three irregular bars at "tremble, tremble, tremble" (3 + 3 + 4 + 2 beats) are laid
// out as three bars of four with every length kept.)
pub(super) const WERE_YOU_THERE: Song = Song {
    name: "Were You There",
    origin: "African-American spiritual, published 1899",
    feel: "Spiritual",
    genre: Genre::Soul,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 60.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "",
    parts: &[
        Part::new("Pickup", "-:8 D4:4 G4:4", "V"),
        Part::new(
            "Verse",
            "B4:8 B4:4 B4:4 | A4:4 G4:4 B4:6 A4:2 | G4:8 G4:4 B4:4 | D5:8 D5:4 D5:4 | \
             E5:4 D5:4 D5:6 B4:2 | A4:16 | D5:8 G5:6 E5:2 | D5:16 | E5:4 D5:8 B4:4 | \
             B4:6 A4:2 G4:4 G4:4 | A4:4 G4:8 G4:4 | G4:8 D4:4 D4:4 | _:8 G4:4 C5:4 | \
             B4:8 B4:4 B4:4 | A4:4 G4:4 B4:6 A4:2 | G4:8 D4:4 G4:4",
            "I | V7 | I | I | IV I | V7 | I | V7 I | IV I | III7 vi | IV | I I IV V | V V vi IV | \
             I | V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse"],
    drums_from: 0,
    about: "Sung slow and free, so there are no drums. The question climbs a fifth each time \
            it is asked; \"Oh!\" reaches the high G; and \"tremble, tremble, tremble\" sinks \
            step by step to the low D before the last question.",
};

// Source: https://github.com/MyatToe/kuic-worship-app/blob/2545b206a9adc248d1614c581113e04441da3405/songs/scores/when-the-roll-is-called-up-yonder.abc
// (Open Hymnal Project score from Epworth Songs, 1893, in A flat; up a major third here.)
pub(super) const ROLL_CALLED_UP_YONDER: Song = Song {
    name: "When the Roll Is Called Up Yonder",
    origin: "James M. Black, 1893",
    feel: "Motown",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Gospel,
    bass: Bass::Walking,
    rhythm: "Motown",
    parts: &[
        Part::new("Pickup", "-:12 C5:3 D5:1", "I"),
        Part::new(
            "Verse",
            "E5:3 E5:1 E5:3 E5:1 E5:3 D5:1 C5:3 C5:1 | D5:3 C5:1 C5:3 A4:1 G4:4 C5:3 D5:1 | \
             E5:3 E5:1 E5:3 E5:1 E5:3 E5:1 D5:3 C5:1 | D5:12 C5:3 D5:1 | \
             E5:3 E5:1 E5:3 E5:1 E5:3 D5:1 C5:3 C5:1 | D5:3 C5:1 C5:3 A4:1 G4:4 C5:3 D5:1 | \
             E5:3 E5:1 E5:3 C5:1 D5:3 D5:1 D5:3 B4:1 | C5:12 E5:3 F5:1",
            "I | IV I | I | V7 I | I | IV I | V7 | I",
        ),
        Part::new(
            "Chorus",
            "G5:11 F5:1 E5:3 F5:1 | G5:8 E5:4 E5:3 E5:1 | F5:11 E5:1 D5:3 E5:1 | F5:8 D5:4 E5:3 F5:1 | \
             G5:11 E5:1 D5:3 C5:1 | C5:8 F5:4 F5:3 D5:1 | E5:3 E5:1 E5:3 C5:1 D5:3 D5:1 D5:3 B4:1 | \
             C5:12 C5:3 D5:1",
            "I | I | V7 | V7 | I | IV | V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 1,
    about: "A camp-meeting hymn built on dotted rhythms, long-short all the way through the \
            verse. The chorus, \"when the roll is called up yonder\", opens on the high G and \
            answers itself on F over the V7.",
};

// Source: https://github.com/yetist/music-score/blob/e4732e0a659a3a017f7290d6217c9e455d8ed61f/abc/abc_dtrad/OLTIMREL.abc
// (Erich Rickheit's "dtrad" collection, in C. Bars 4 and 7 follow the Musica Viva and
// Wikifonia readings, https://github.com/syntithenai/abc2book/blob/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201359108.abc,
// which agree against this file there.)
pub(super) const OLD_TIME_RELIGION: Song = Song {
    name: "Give Me That Old Time Religion",
    origin: "African-American spiritual, 19th century",
    feel: "Motown",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Gospel,
    bass: Bass::Walking,
    rhythm: "Motown",
    parts: &[
        Part::new("Pickup", "-:8 G4:2 G4:2 A4:4", "I"),
        Part::new(
            "Verse",
            "C5:8 C5:6 C5:2 | A4:4 G4:4 A4:2 A4:2 C5:4 | D5:8 D5:6 E5:2 | D5:4 C5:4 C5:2 C5:2 D5:4 | \
             E5:8 E5:6 E5:2 | D5:4 C5:8 A4:4 | G4:4 C5:4 D5:4 B4:4 | C5:8 G4:2 G4:2 A4:4",
            "I | I | V7 | I | I7 | IV | I V7 | I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse", "Verse"],
    drums_from: 1,
    about: "Three times \"give me that old time religion\", each a step higher: on C, on D \
            over the V7, on E over a C7 that pulls to the IV chord. Then \"it's good enough for \
            me\" walks G, C, D, B back home.",
};

// Source: https://github.com/baobach/abc_search/blob/42cb4abd42a0ea8b0c5c9b8477ab80ec059f48df/database/abc/QmasiXVwQNxNsQjcHVmq4o4rcHrRwcG6JKE2CfALAG8vEv.abc
// (Rodeheaver's Plantation Melodies, 1918, four-part score in G; up a fourth here.)
pub(super) const NEED_OF_PRAYER: Song = Song {
    name: "Standing in the Need of Prayer",
    origin: "African-American spiritual, published 1918",
    feel: "Funk",
    genre: Genre::Soul,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Funk,
    bass: Bass::Funk,
    rhythm: "Funk",
    parts: &[
        Part::new("Pickup", "-:12 E5:4", "I"),
        Part::new(
            "Chorus",
            "E5:12 E5:4 | E5:2 D5:6 C5:8 | C5:2 C5:2 C5:2 C5:2 D5:2 D5:6 | E5:12 E5:4 | \
             E5:12 E5:4 | E5:2 D5:6 C5:8 | C5:2 C5:2 C5:2 C5:2 E5:2 D5:6 | C5:12 E5:2 D#5:2",
            "I | V7 I | I V7 | I | I | V7 I | V7 | I",
        ),
        Part::new(
            "Verse",
            "E5:2 D#5:2 E5:2 D#5:2 E5:2 D#5:2 E5:2 D#5:2 | E5:2 D5:6 C5:8 | \
             C5:2 C5:2 C5:2 C5:2 D5:2 D5:6 | E5:12 E5:2 D#5:2 | \
             E5:2 D#5:2 E5:2 D#5:2 E5:2 D#5:2 E5:2 D#5:2 | E5:2 D5:6 C5:8 | \
             C5:2 C5:2 C5:2 C5:2 E5:2 D5:6 | C5:12 E5:4",
            "I | I | I V7 | I | I | I | I V7 | I",
        ),
    ],
    form: &["Pickup", "Chorus", "Verse", "Chorus"],
    drums_from: 1,
    about: "\"It's me, it's me, O Lord\" is three notes, E D C, the top of the major scale \
            coming down. The verse, \"not my brother, nor my sister\", trills between E and D \
            sharp, a half step below it, before the same fall.",
};
