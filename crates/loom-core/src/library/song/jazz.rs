//! Jazz: ragtime, swing and a jazz waltz.

use super::*;

pub(super) const JINGLE_BELLS: Song = Song {
    name: "Jingle Bells",
    origin: "James Lord Pierpont, 1857",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 150.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    style: "ii–V–I",
    parts: &[
        Part::new(
            "Verse",
            "D5:4 B5 A5 G5 | D5:12 D5:2 D5 | D5:4 B5 A5 G5 | E5:16 | \
             E5:4 C6 B5 A5 | F#5:16 | D6:4 D6 C6 A5 | B5:16 | \
             D5:4 B5 A5 G5 | D5:16 | D5:4 B5 A5 G5 | E5:12 E5:4 | \
             E5:4 C6 B5 A5 | D6 D6 D6:6 D6:2 | E6:4 D6 C6 A5 | G5:16",
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
    ],
    form: &["Verse", "Chorus", "Chorus"],
    drums_from: 0,
    about: "In the chorus, A7 is a secondary dominant (the V of V): it leads to D7, which \
            leads home to G.",
};

pub(super) const SAINTS: Song = Song {
    name: "When the Saints Go Marching In",
    origin: "American spiritual, traditional",
    feel: "Dixieland",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 168.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Swing",
    style: "New Orleans",
    parts: &[Part::new(
        "Chorus",
        "-:4 C5:4 E5 F5 | G5:16 | -:4 C5:4 E5 F5 | G5:16 | \
         -:4 C5:4 E5 F5 | G5:8 E5:8 | C5:8 E5:8 | D5:16 | \
         -:4 E5:4 E5 D5 | C5:12 C5:4 | E5:8 G5:8 | G5:4 F5:12 | \
         -:8 E5:4 F5 | G5:8 E5:8 | C5:8 D5:8 | C5:16",
        "I | I | I | I | I | I | I | V7 | V7 | I | I7 | IV | IV | I | ii7 V7 | I",
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "The New Orleans brass band classic. The tune starts with only four notes, C E F \
            G, and the tuba plays two beats to the bar.",
};

pub(super) const ENTERTAINER: Song = Song {
    name: "The Entertainer",
    origin: "Scott Joplin, 1902",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "New Orleans",
    parts: &[
        Part::new("Pickup", "-:12 D5:2 D#5", "V"),
        Part::new(
            "A",
            "E5:2 C6:4 E5:2 C6:4 E5:2 C6:2 | _:10 C6:2 D6 D#6 | E6:2 C6 D6 E6:4 B5:2 D6:4 | \
             C6:12 D5:2 D#5 | E5:2 C6:4 E5:2 C6:4 E5:2 C6:2 | _:12 A5:2 G5 | \
             F#5:2 A5 C6 E6:4 D6:2 C6 A5 | D6:12 D5:2 D#5 | \
             E5:2 C6:4 E5:2 C6:4 E5:2 C6:2 | _:10 C6:2 D6 D#6 | E6:2 C6 D6 E6:4 B5:2 D6:4 | \
             C6:12 C6:2 D6 | E6:2 C6 D6 E6:4 C6:2 D6 C6 | E6:2 C6 D6 E6:4 C6:2 D6 C6 | \
             E6:2 C6 D6 E6:4 B5:2 D6:4 | C6:12 D5:2 D#5",
            "I | IV I | I V7 | I | I | IV | II7 | V7 | I | IV I | I V7 | I | I7 | IV iv | I V7 | I",
        ),
    ],
    form: &["Pickup", "A", "A"],
    drums_from: 1,
    about: "Ragtime, the piano music jazz grew out of: the left hand keeps a steady \
            oom-pah, bass then chord, while the right hand pushes against it, landing \
            between the beats and holding on across them.",
};

pub(super) const HAPPY_BIRTHDAY: Song = Song {
    name: "Happy Birthday to You",
    origin: "Mildred and Patty Hill, \"Good Morning to All\", 1893",
    feel: "Jazz waltz",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: THREE_FOUR,
    tempo: 132.0,
    comping: Comping::Sway,
    bass: Bass::Waltz,
    rhythm: "Jazz waltz",
    style: "Jazz waltz",
    parts: &[Part::new(
        "Song",
        "-:8 G4:3 G4:1 | A4:4 G4 C5 | B4:8 G4:3 G4:1 | A4:4 G4 D5 | C5:8 G4:3 G4:1 | \
         G5:4 E5 C5 | B4 A4 F5:3 F5:1 | E5:4 C5 D5 | C5:12",
        "I | I | V7 | V7 | I | I7 | IV | I V7 | I",
    )],
    form: &["Song", "Song", "Song"],
    drums_from: 0,
    about: "The most sung song in the world, as a jazz waltz: three beats to the bar with a \
            swing ride. Each line climbs a little higher, a fourth, then a fifth, then an \
            octave, for the name.",
};

pub(super) const TWO_FIVE_WALK: Song = Song {
    name: "Two-Five Walk",
    origin: "Written for Ferrous Loom",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 150.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    style: "ii–V–I",
    parts: &[Part::new(
        "Head",
        "-:2 D5:2 F5 A5 C6:4 A5 | B5:2 A5 G5 F5 E5 D5 B4:4 | C5 E5:2 G5 B5:8 | _:8 -:8 | \
         -:2 F5:2 A5 C6 E6:4 D6 | B5:2 G5 F5 D5 B4:4 G4 | A4:2 C5 E5 G5 B5:4 A5 | \
         G5:2 E5 C#5 A4 G5:8 | F5:4 A5 D6:8 | -:2 B5:2 A5 G5 F5:4 D5 | E5 G5 C#5 E5 | \
         D5 F5 B4 D5 | -:2 A5:2 F5 D5 C6:4 A5 | B5:2 D6 F6 D6 B5:4 G5 | C6:16 | -:16",
        "ii7 | V7 | Imaj7 | Imaj7 | ii7 | V7 | Imaj7 | VI7 | ii7 | V7 | iii7 VI7 | ii7 V7 | \
         ii7 | V7 | Imaj7 | Imaj7",
    )],
    form: &["Head", "Head", "Head"],
    drums_from: 0,
    about: "The ii-V-I, the progression jazz is made of, in sixteen bars. Each line spells \
            out its chord: up a Dm7, down a G7, landing on the major seventh of Cmaj7.",
};

// Source: https://raw.githubusercontent.com/DCMLab/debussy_childrens_corner/main/notes/l113-06_childrens_golliwoggs.tsv
// (the note table of the DCML corpus score)
pub(super) const GOLLIWOGGS_CAKEWALK: Song = Song {
    name: "Golliwogg's Cakewalk",
    origin: "Claude Debussy, Children's Corner, 1908",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "New Orleans",
    parts: &[
        // The run down three octaves, a bar's silence, and the vamp.
        Part::new(
            "Intro",
            "-:8 Bb5:1 Ab5:2 Bb5:1 F5:2 Bb5:2 | Ab5:1 F5:2 Eb5:1 Cb5:4 -:1 Ab4:1 F4:1 Eb4:1 Cb4:4 | \
             -:1 Ab3:1 F3:1 Eb3:1 Cb3:2 G3:2 -:8 | -:2 Bb3:2 -:2 Bb3:1 Bb3:1 -:2 Bb3:2 -:2 Bb3:2 | \
             -:2 Bb3:1 Bb3:1 -:2 Bb3:2 -:2 Bb3:2 -:2 Bb3:2",
            "I | I | I | I | I",
        ),
        Part::new(
            "Theme",
            "Bb4:1 Ab4:2 Bb4:1 F4:2 Bb4:2 Ab4:1 F4:2 Eb4:1 Cb4:2 Eb4:2 | Bb3:1 Bb3:2 C4:1 Bb3:2 Bb4:2 Bb3:8 | \
             Bb3:1 Ab3:2 Bb3:1 C4:2 Ab3:2 Bb3:1 Ab3:2 Bb3:1 Cb4:2 Ab3:2 | \
             Bb4:1 G4:2 F4:1 Eb4:2 D4:2 C4:2 G3:2 C4:2 D4:2 | \
             Bb4:1 Ab4:2 Bb4:1 F4:2 Bb4:2 Ab4:1 F4:2 Eb4:1 Cb4:2 Eb4:2 | Bb3:1 Bb3:2 C4:1 Bb3:2 Bb4:2 Bb3:8 | \
             Bb3:1 Bb3:2 C4:1 Bb3:2 Db4:2 Eb4:2 F4:2 Eb4:2 Gb4:2 | \
             F4:1 F4:2 G4:1 F4:2 D5:2 Bb4:4 Bb5:2 -:2",
            "I | I | I | I I vi V7 | I | I | bIII | II7 V",
        ),
    ],
    form: &["Intro", "Theme", "Theme"],
    drums_from: 1,
    about: "A cakewalk, the strutting dance of ragtime, with Debussy's blue note: the C flat \
            that keeps falling onto B flat on the off-beat. The second time round the tune \
            slides into G flat major, the flat-three chord, and comes back through F7 and \
            B flat. The piece is in 2/4, two of its bars to a bar here; the opening run falls \
            through three octaves, and the low unison run in the fourth bar of the theme is \
            written an octave up.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/RussianFolkSongDarkEyes.abc
// (the Wikifonia lead sheet in D minor, 3/4, with chords; here in C minor, down a tone, so
// that the last high note fits. Checked against Frank Nordberg's Musica Viva ABC in G minor,
// https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/folktunefinder/abc_tune_folktunefinder_151450.txt,
// which differs in a few notes, such as the third note of the fifth bar.)
pub(super) const DARK_EYES: Song = Song {
    name: "Dark Eyes",
    origin: "Florian Hermann, Valse hommage, op. 21, 1879; words by Yevhen Hrebinka",
    feel: "Jazz waltz",
    genre: Genre::Jazz,
    tonic: C,
    mode: HarmonicMinor,
    time: THREE_FOUR,
    tempo: 160.0,
    comping: Comping::Waltz,
    bass: Bass::Walking,
    rhythm: "Jazz waltz",
    style: "Minor jazz waltz",
    parts: &[
        Part::new("Pickup", "-:4 F#4:4 G4", "V7"),
        Part::new(
            "A",
            "Ab4:6 G4:2 G4:4 | -:4 F#4 G4 | Ab4:6 G4:2 G4:4 | -:4 G4 C5 | C5:6 B4:2 B4:4 | \
             -:4 D5 Eb5 | D5:6 C5:2 C5:4 | -:4 C5 Eb5 | Eb5:6 D5:2 Ab4:4 | -:4 D5 Eb5 | \
             D5:6 C5:2 G4:4 | -:4 F#4 G4 | Ab4:6 G4:2 G4:4 | -:4 D4 G4 | F4:6 Eb4:2 Eb4:4 | \
             -:4 F#4 G4",
            "V7 | V7 | i | i | V7 | V7 | i | i | iv | iv | i | i | V7 | V7 | i | i",
        ),
        Part::new(
            "A end",
            "Ab4:6 G4:2 G4:4 | -:4 F#4 G4 | Ab4:6 G4:2 G4:4 | -:4 G4 C5 | C5:6 B4:2 B4:4 | \
             -:4 D5 Eb5 | D5:6 C5:2 C5:4 | -:4 C5 Eb5 | Eb5:6 D5:2 Ab4:4 | -:4 D5 Eb5 | \
             D5:6 C5:2 G4:4 | -:4 F#4 G4 | Ab4:6 G4:2 G4:4 | -:4 D5 Eb5 | D5:6 C5:2 C5:4 | \
             C6:4 -:8",
            "V7 | V7 | i | i | V7 | V7 | i | i | iv | iv | i | i | V7 | V7 | i | i",
        ),
    ],
    form: &["Pickup", "A", "A end"],
    drums_from: 1,
    about: "A piano waltz of 1879 that became Russia's best-known romance, and a gypsy-jazz \
            standard. Every other bar starts with a rest on beat one and a two-note pickup into \
            the next, the first leaning up a semitone, F sharp to G; the tune climbs in sequence over G7 and C \
            minor, peaks on F minor (iv), and leaps up to the high C at the very end.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/JoplinS/maple/maple.ly
// (Mutopia, from the original 1899 edition; the top line of the right hand)
pub(super) const MAPLE_LEAF_RAG: Song = Song {
    name: "Maple Leaf Rag",
    origin: "Scott Joplin, 1899",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: GSharp,
    mode: Major,
    time: COMMON,
    tempo: 80.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "New Orleans",
    parts: &[
        Part::new(
            "A",
            "-:1 Ab4 Eb5 Ab4 C5 Eb5:2 G4:1 Eb5 G4 Bb4 Eb5:5 | \
             -:1 Ab4 Eb5 Ab4 C5 Eb5:2 G4:1 Eb5 G4 Bb4 Eb5:3 -:1 Eb5 | \
             - Ab4 Cb5 Fb5 - Eb5 - Eb5 - Ab4 Cb5 Fb5 - Eb5 -:2 | \
             -:1 Ab3 Cb4 Ab4 - Ab3 Cb4 Ab4 - Ab4 Cb5 Ab5 - Ab4 Cb5 Ab5 | \
             Ab5:2 Ab5 Ab5 Ab5:1 Ab5 _ Eb5 F5 C5 Eb5 F5:2 Ab4:1 | \
             _ Bb4 Cb5 Ab4 Bb4 C5:2 Ab4:1 C5 Ab4 Bb4:2 Ab4 -:1 Ab5 | \
             _:2 Ab5 Ab5 Ab5:1 Ab5 _ Eb5 F5 C5 Eb5 F5:2 Ab4:1 | \
             _ Bb4 Cb5 Ab4 Bb4 C5:2 Ab4:1 C5 Ab4 Bb4:2 Ab4 -",
            "I V7 | I V7 | bVI V bVI V | i | #ii°7 I | bVI I V7 I | #ii°7 I | bVI I V7 I",
        ),
        Part::new(
            "B",
            "-:1 G5 Eb6 G5 Bb5 D6:2 G5:1 Db6 G5 Bb5 C6:2 Eb5:1 Bb5 Eb5 | \
             - C5 Ab5 C5 Eb5 F5:2 C5:1 Ab5 C5 Eb5 F5:2 C5:1 F5:2 | \
             -:1 Eb5 G5 Bb4 Db5 F5:2 Eb5:1 G5 Bb4 Db5 F5:2 Db5:1 F5:2 | \
             -:1 C5 Ab5 C5 Eb5 F5:2 C5:1 Ab5 C5 Eb5 F5:2 C5:1 F5:2 | \
             -:1 G5 Eb6 G5 Bb5 D6:2 G5:1 Db6 G5 Bb5 C6:2 Eb5:1 Bb5 Eb5 | \
             - C5 Ab5 C5 Eb5 F5:2 C5:1 Ab5:2 Ab5 G5 Gb5 | \
             -:1 F4 A4 C5 F5 C5 A4 F4 - F4 Bb4 Db5 F5:2 Db5",
            "V7 | I | V7 | I | V7 | I | VI ii",
        ),
        // The two endings of the B strain: back to its start, then on to A.
        Part::new(
            "B1",
            "C5:2 -:1 C5 - Bb4:2 Eb4:1 Ab4:2 Eb5 Eb5 Eb5",
            "II7 V7 I I",
        ),
        Part::new(
            "B2",
            "C5:2 -:1 C5 - Bb4:2 Eb4:1 - Ab4 C5 Eb5 Ab5:2 -",
            "II7 V7 I I",
        ),
    ],
    form: &["A", "A", "B", "B1", "B", "B2", "A"],
    drums_from: 0,
    about: "The rag that made ragtime famous. The tune is a broken chord in sixteenths that \
            keeps landing just after the beat, over a left hand that never stops its \
            oom-pah. Listen for the F flat chord (bVI) that darkens the third bar, and the \
            diminished seventh under the repeated A flats that resolves back to the tonic. The \
            piece is in 2/4, two of its bars to a bar here; the climb through four octaves \
            is folded into two, and one high phrase is played an octave down.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/I_Got_Rhythm/I_Got_Rhythm.mxl
// (OpenEWLD, the public-domain part of the Wikifonia lead sheets: the refrain)
pub(super) const I_GOT_RHYTHM: Song = Song {
    name: "I Got Rhythm",
    origin: "George Gershwin, Girl Crazy, 1930",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 176.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    style: "Rhythm changes",
    parts: &[
        Part::new(
            "A",
            "-:4 F4:6 G4 | Bb4 C5:10 | -:4 C5:6 Bb4 | G4 F4:10 | \
             -:4 F4:6 G4 | Bb4 C5:2 - Eb5 - C5 | D5:4 D5 C5:2 D5 C5 Bb4 | _:16",
            "I vi7 | ii7 V7 | I vi7 | ii7 V7 | I I7 | IV iv | I V7 | I",
        ),
        Part::new(
            "B",
            "-:4 D5:6 D5 | D5 E5:10 | -:4 D5:6 D5 | D5 G4:10 | \
             -:4 C5:6 C5 | C5 D5:10 | -:4 C5:6 C5 | C5:16",
            "III7 | III7 | VI7 | VI7 | II7 | II7 | V7 | V7",
        ),
        // The last A, with the two-bar tag: "who could ask for anything more?"
        Part::new(
            "A end",
            "-:4 F4:6 G4 | Bb4 C5:10 | -:4 C5:6 Bb4 | G4 F4:10 | \
             -:4 F4:6 G4 | Bb4 C5:4 Eb5 C5:2 | D5:4 D5 C5:2 D5 C5:4 | F5:12 Eb5:2 Bb4 | \
             D5:4 D5 C5:2 D5 C5:4 | Bb4:12 -:4",
            "I vi7 | ii7 V7 | I vi7 | ii7 V7 | I I7 | IV iv | I v | VI7 | II7 V7 | I",
        ),
    ],
    form: &["A", "A", "B", "A end"],
    drums_from: 0,
    about: "The chords of this chorus became \"rhythm changes\", the progression jazz \
            musicians have written most new tunes on after the blues. The A section turns \
            I-vi-ii-V twice, then I-I7-IV-iv; the bridge is a chain of dominants, two bars \
            each, falling by fifths from D7 back to F7. The tune opens on four notes, F G \
            B flat C, pushed off the beat.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Hughie_Cannon/Bill_Bailey%2C_Won%27t_You_Please_Come_Home%3F/Bill_Bailey%2C_Won%27t_You_Please_Come_Home%3F.mxl
// (OpenEWLD lead sheet: the chorus)
pub(super) const BILL_BAILEY: Song = Song {
    name: "Bill Bailey, Won't You Please Come Home",
    origin: "Hughie Cannon, 1902",
    feel: "Dixieland",
    genre: Genre::Jazz,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Second line",
    style: "New Orleans",
    parts: &[
        Part::new(
            "Chorus",
            "D4:4 E4:8 G4:4 | B4 A#4 B4 D5 | D4 E4:8 G4:4 | B4:16 | \
             -:4 D4 E4 G4 | B4:8 D5 | D5:4 F#4:12 | _:8 - | \
             D4:4 F#4:8 A4:4 | C5 B4 C5 D5 | D4 F#4:8 A4:4 | C5:16 | \
             -:4 D4 F#4 A4 | D5:8 E5 | E5:4 B4:12 | _:16",
            "I | I | I | I | I | I #i°7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 V+ | I | I",
        ),
        Part::new(
            "Chorus end",
            "-:4 D4 E4:2 E4 G4:4 | B4 A#4 B4 D5 | - D4 E4 G4 | B4:12 D4:4 | \
             D4 D4 G4 B4 | D5:8 F5 | E5:16 | E5:12 G4:4 | \
             G4 G4:8 G4:4 | A4:8 G4 | D5:4 D5:8 E5:4 | B4:12 B4:4 | \
             B4 A4 G#4 A4 | D5:8 B4 | G4:16 | _:4 -:12",
            "I | I | I | I | I | I iii° | ii VI7 | ii | IV | #iv°7 | I | VI7 | II7 | V7 | I | II7 V7",
        ),
    ],
    form: &["Chorus", "Chorus end", "Chorus", "Chorus end"],
    drums_from: 0,
    about: "A ragtime song from 1902 that Dixieland bands still play. The first half takes \
            its time: six bars of G, then eight of D7, the V7. The second half changes chord \
            almost every bar, through C and C sharp diminished, then a chain of dominants, E7, A7, \
            D7, each a fifth above the next, that walks back home to G.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Irving_Caesar/Swanee/Swanee.mxl
// (OpenEWLD lead sheet: the chorus)
pub(super) const SWANEE: Song = Song {
    name: "Swanee",
    origin: "George Gershwin, 1919",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 160.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    style: "Turnaround",
    parts: &[
        Part::new(
            "Chorus",
            "A4:16 | A4 | A4:4 G4 G4 F4 | A4 G4 G4 F4 | \
             G4:16 | F4:8 D4 | C4:4 F4:12 | _:4 G4 A4 Bb4 | \
             C5:16 | C#5 | D5 | -:4 Bb4 C5 C#5 | \
             D5:16 | E5 | F5:4 E5 D5 C5 | D5 C5 B4 Bb4",
            "I | I | I | I | IV | ii V7 | I | I | I | III | IV | IV | ii | V7 | I | I",
        ),
        Part::new(
            "Chorus end",
            "A4:16 | A4 | A4:4 G4 G4 F4 | A4 G4 G4 F4 | \
             G4:16 | F4:8 D4 | C4:4 F4:12 | _:4 G4 A4 Bb4 | \
             C5:16 | C#5 | E5:4 D5 C#5 D5 | _:12 D#5:4 | \
             E5 D5 C5 Bb4 | A4 G4:8 F4:4 | _:16 | _:12 -:4",
            "I | I | I | I | IV | ii7 V7 | I | I | I | III | IV | ii | V | V V V I | I | I",
        ),
    ],
    form: &["Chorus", "Chorus end"],
    drums_from: 0,
    about: "Gershwin's first big hit. The tune sits on A, the third of F, for two whole \
            bars, then climbs by half steps, C, C sharp, D, while the chords go from F \
            through A major, the III chord, to B flat: the C sharp is the A chord's third, \
            pulling up to the D of B flat.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Al_Jolson-B.G._DeSylva-Vincent_Rose/Avalon/Avalon.mxl
// (OpenEWLD lead sheet: the chorus)
pub(super) const AVALON: Song = Song {
    name: "Avalon",
    origin: "Vincent Rose, Al Jolson and Buddy DeSylva, 1920",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 168.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    style: "ii–V–I",
    parts: &[
        Part::new("Pickup", "-:12 C4:4", "V7"),
        Part::new(
            "Chorus",
            "G4:8 A4 | Bb4 C5 | Bb4:4 A4 G4:8 | _ C4 | \
             F4:16 | _:8 G4 | A4:16 | _:8 C4 | \
             E4 F4 | G4 F4 | E4:4 D4 C4:8 | _ D4 | \
             A4:16 | _:8 D4 | A4:16 | _:8 D4",
            "V7 | V7 | V7 | V7 | I | I V7 | I | I | V7 ii7 | V7 ii7 | V7 | V7 | I | I V7 | I | I",
        ),
        Part::new(
            "Chorus end",
            "A4:8 Bb4 | C5 D5 | C5:4 Bb4 A4:8 | _ D4 | \
             G4:16 | _:8 A4 | Bb4:16 | _:4 - D4:8 | \
             C4 F4 | A4 C5 | E5:4 D5 E5:8 | _:4 - D5:8 | \
             Bb4:16 | A4 | F4 | _:12 -:4",
            "VI7 | VI7 | VI7 | VI7 | VI7 | VI7 | ii | ii | I | I | VI7 | VI7 | ii | V7 | I | I",
        ),
    ],
    form: &["Pickup", "Chorus", "Chorus end"],
    drums_from: 1,
    about: "The song starts away from home: four bars of C7, the V7, before the first F \
            chord arrives. The second half does the same trick a step higher, six bars of \
            D7 that lead to G minor, the ii, and the end is a plain ii-V-I.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Chris_Smith_%5B1%5D-Jim_Burris/Ballin%27_the_Jack/Ballin%27_the_Jack.mxl
// (OpenEWLD lead sheet: the chorus)
pub(super) const BALLIN_THE_JACK: Song = Song {
    name: "Ballin' the Jack",
    origin: "Chris Smith and Jim Burris, 1913",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 132.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    style: "Turnaround",
    parts: &[Part::new(
        "Chorus",
        "D5:2 D5 B4 B4 G4:4 A4 | D5:2 B4:4 G4:6 A4:2 Ab4 | G4 G4 A4 A4 Bb4:4 A4:2 G4 | \
         Bb4 Bb4 A4 A4 G4:4 - | F4:2 F4 G4 G4 A4:4 G4:2 F4 | C5 A4:4 F4:6 D5:2 C#5 | \
         D5 C#5 D5 Bb4 C5 B4 C5 A4 | Bb4 A4 G4 A4:6 -:4 | D5:2 D5 B4 B4 G4:4 A4 | \
         D5:2 B4:4 G4:6 A4:2 Ab4 | G4 G4 A4 A4 Bb4:4 A4:2 G4 | Bb4 A4:4 G4:6 Bb4:2 C5 | \
         Db5:4 Bb4 Db5 Bb4 | D5:2 C5:4 Bb4:6 G4:4 | C5 D5 Eb5 C5 | D5:2 C#5 D5 Bb4:6 -:4",
        "VI7 | VI7 | II7 | II7 | V7 | V7 | I III7 | IV7 III7 | VI7 | VI7 | II7 | II7 | bVI7 | \
         I I vi VI7 | ii VI7 ii iiø7 | V7 I",
    )],
    form: &["Chorus", "Chorus", "Chorus"],
    drums_from: 0,
    about: "A dance craze of 1913 built on the ragtime chain of dominants: G7, C7, F7, two \
            bars each, every chord the V of the next, before B flat finally arrives in the \
            seventh bar. The tune answers with its own chromatic slips, A to A flat to G.",
};

// Source: https://tunearch.org/wiki/Annotation:Twelfth_Street_Rag
// (the ABC there, a fiddle version after Phillips, Traditional American Fiddle
// Tunes vol. 2: the introduction and the famous first strain, in cut time)
pub(super) const TWELFTH_STREET_RAG: Song = Song {
    name: "Twelfth Street Rag",
    origin: "Euday L. Bowman, 1914",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 152.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "New Orleans",
    parts: &[
        Part::new(
            "Intro",
            "G5:2 F#5 G5 A5 G5 -:6 | G4:2 F#4 G4 A4 G4 -:6 | G4:2 G4:4 G4:2 A4:4 A#4 | \
             B4:2 -:6 G4:8",
            "V7 | V7 | V7 | V7",
        ),
        Part::new(
            "A1",
            "C5:2 B4 A4 C5 B4 A4 C5 B4 | A4 C5 B4 A4 C5 B4 A4 - | C5 B4 A4 C5 B4 A4 C5 B4 | \
             A4:4 -:12 | C5:2 B4 A4 C5 B4 A4 C5 B4 | A4 C5 B4 A4 C5 B4 A4 - | \
             C5 B4 A4 C5 B4 A4 C5 B4 | A4 -:14",
            "I | I | V7 | V7 | V7 | V7 | I | I",
        ),
        Part::new(
            "A2",
            "C5:2 B4 A4 C5 B4 A4 C5 B4 | A4 C5 B4 A4 C5 B4 A4 - | C5 B4 A4 C5 B4 A4 C5 B4 | \
             A4:4 -:12 | E4:8 D4:4 - | B4:8 A4:4 - | G4:2 G4:4 G4:2 A4:4 A#4 | B4 G4 A4 B4",
            "I | I | V7 | V7 | II9 | II9 | V7 | V7",
        ),
        Part::new(
            "Tag",
            "C5:6 C5:4 D5:2 C5:4 | Bb4:6 Bb4:4 C5:2 Bb4:4 | A4:6 A4:4 Bb4:2 A4:4 | \
             Ab4:6 Ab4:4 Bb4:2 Ab4:4 | G4 G4:2 A4:4 G4:2 E4 C4 | E4:4 E4:2 D4:4 E4:2 D4:4 | \
             C4 C4:2 D4 E4 G4 A4 G4 | C5 G4 A4 C5:6 -:4",
            "I | I7 | IV | iv | I | II7 V7 | I | I",
        ),
    ],
    form: &["Intro", "A1", "A2", "A1", "Tag"],
    drums_from: 0,
    about: "The main strain is three notes, C B A, played in bars of eight eighths, so the \
            group of three starts on a different beat each time round: the three-against-four \
            of ragtime. The chords hardly move, C and G7, and the last strain slides down \
            by half steps, C, B flat, A, A flat, over C, C7, F and F minor.",
};

// Source: https://tunearch.org/wiki/Annotation:Dill_Pickle_Rag
// (the ABC there, a fiddle version after Phillips, "via PR": the first and
// second strains, in 4/4 as written there)
pub(super) const DILL_PICKLES: Song = Song {
    name: "Dill Pickles Rag",
    origin: "Charles L. Johnson, 1906",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 152.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "New Orleans",
    parts: &[
        Part::new("Pickup", "-:4 B4:4 C5 C#5", "V7"),
        Part::new(
            "A",
            "D5:2 E5 G5 D5 E5 G5 D5 E5 | G5 D5 E5 G5:4 E5:2 D5:4 | \
             G4:2 A4 B4 G4 A4 B4 G4 A4 | B4 G4 A4 B4:4 A4:2 G4:4 | \
             D4:2 E4 F#4 D4 E4 F#4 D4 E4 | F#4 D4 E4 F#4:4 E4:2 D4:4",
            "I | I | vi | vi | V | V",
        ),
        // The two endings of the first strain.
        Part::new(
            "A1",
            "G4:2 F#4 G4 E4:4 F#4:2 G4 G#4 | A4 G#4 A4 D5:4 B4:2 C5 C#5",
            "I VI | II V",
        ),
        Part::new(
            "A2",
            "A4:2 G#4 A4 D5:4 B4:2 A4:4 | G4:10 G4:2 F#4 E4",
            "II V | I",
        ),
        Part::new(
            "B",
            "D4:4 Eb4 E4 F#4 | G4 E5:2 D5:4 E5:2 D5:4 | F#4 E5:2 D5:4 E5:2 D5:4 | \
             G4 E5:2 D5:4 E5:2 D5:4 | D4:2 A4 E4 A4 F4 A4 F#4 A4 | G4:4 E5:2 D5:4 E5:2 D5:4",
            "V | I | V | I | V | I",
        ),
        // The two endings of the second strain.
        Part::new(
            "B1",
            "A4:2 B4 C#5 A4 B4 C#5:6 | D5:10 G4:2 F#4 E4",
            "II | V",
        ),
        Part::new("B2", "D5:2 E5 F#5 D5 E5 F#5:6 | G5:16", "V | I"),
    ],
    form: &["Pickup", "A", "A1", "A", "A2", "B", "B1", "B", "B2"],
    drums_from: 1,
    about: "The first strain is a three-note cell, D E G, run through bars of eight \
            eighths, so it lands on a new beat each time: ragtime's three-against-four. The \
            same cell then steps down, onto E minor (vi) and D (V), and the endings turn \
            through E and A major, a chain of dominants that leads back through D to G.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/JoplinS/PineappleRag/PineappleRag.ly
// (Mutopia, from the Seminary Music edition of 1908; the top line of the right
// hand, down a fourth from B flat. In the second strain the broken octaves,
// a low note and then the chord, are written as repeated notes at the top.)
pub(super) const PINE_APPLE_RAG: Song = Song {
    name: "Pine Apple Rag",
    origin: "Scott Joplin, 1908",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 100.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "New Orleans",
    parts: &[
        Part::new(
            "A",
            "D5:1 C5:2 Bb4:1 A4 G#4 A4 G4 F4 G4 A4 C5:2 F5:1 A5:2 | \
             D5:1 C5:2 Bb4:1 A4 G#4 A4 Bb4 C5:2 C5:1 A4 C5 A4 C5:2 | \
             F5:1 F5:2 F5:1 F5:2 F5:1 F5:2 A5:1 F5 A4 C5 B4 C5 D5 | \
             E5 C5:2 E5:1 D5:2 E5:1 C5:3 E4:1 F4 G4 A4 Bb4 C5 | \
             D5 C5:2 Bb4:1 A4 G#4 A4 G4 F4 G4 A4 C5:2 F5:1 A5:2 | \
             D5:1 C5:2 Bb4:1 A4 G#4 A4 Bb4 C5:2 C5:1 A4 C5 A4 C5:2 | \
             F5:1 F5:2 F5:1 F5:2 F5:1 F5:2 A5:1 F5 C5 A4 C5 D5 F5",
            "I | I I7 | IV iv I I | V II7 V V | I | I I7 | IV iv I I",
        ),
        // The endings of the first strain: back to its start, or on to B.
        Part::new(
            "A1",
            "F4:4 G4:1 A4:2 F4:3 F4:1 G4 A4 Bb4 C5:2",
            "II7 V7 I I",
        ),
        Part::new(
            "A2",
            "F4:4 G4:1 A4:2 F4:3 C5:1 B4 C5 C#5 D5 D#5",
            "II7 V7 I I",
        ),
        Part::new(
            "B",
            "E5:1 E5:2 D5:1 D5:2 E5:1 E5:2 D5:1 D5:2 E5 D5 | \
             D5:1 D5:2 C5:1 C5:2 D5:1 D5:2 C5:1 C5:2 D5 C5 | \
             C6:1 C6:2 Bb5:1 Bb5:2 A5:1 A5:2 Bb5:1 Bb5:2 A5 G5 | \
             G5:1 G5:2 F5:1 F5:2 D5 C5 C5:1 B4 C5 C#5 D5 D#5 | \
             E5 E5:2 D5:1 D5:2 E5:1 E5:2 D5:1 D5:2 E5 D5 | \
             D5:1 D5:2 C5:1 C5:2 D5:1 D5:2 C5:1 C5:2 D5 C5 | \
             Db5:1 Db5:2 F5:1 F5:2 Db5 C5:1 C5:2 B4:1 B4:2 C5",
            "V7 | I | V7 | I | V7 | I | iv I",
        ),
        Part::new("B1", "-:1 G5:3 A5:2 G5 F5 C5:1 B4 C5 C#5 D5 D#5", "V7 I"),
        Part::new("B2", "-:1 G5:3 A5:2 G5 F5 F4:1 G4 A4 Bb4 C5:2", "V7 I"),
        // The first ending again, without the run back to the start.
        Part::new("End", "F4:4 G4:1 A4:2 F4:9", "II7 V7 I I"),
    ],
    form: &["A", "A1", "A", "A2", "B", "B1", "B", "B2", "A", "End"],
    drums_from: 0,
    about: "A slow-march rag. The first strain steps down with a chromatic turn around A, \
            then climbs a broken chord to the top; halfway through it turns B flat major into B flat minor \
            (IV, then iv) for a moment of shade. The second strain opens on the dominant, \
            C7, and hammers each note twice, short then long: that is the ragtime \
            syncopation in its plainest form. The piece is in 2/4, two of its bars to a bar \
            here, a fourth below the original key.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/JoplinS/EliteSyncopations/EliteSyncopations.ly
// (Mutopia, from the John Stark & Son edition of 1902; the top line of the
// right hand, in the original F)
pub(super) const ELITE_SYNCOPATIONS: Song = Song {
    name: "Elite Syncopations",
    origin: "Scott Joplin, 1902",
    feel: "Ragtime",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 76.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "New Orleans",
    parts: &[
        Part::new(
            "A",
            "-:1 F4 D5 C5:2 D5:1 C5:2 -:1 E4 D5 C5:2 D5:1 C5:2 | \
             -:1 C5 - C5 - Cb5 - Bb4 A4 C5 D5 F5 E5:2 G5 | \
             A5:1 G#5 A5 C6:2 A5:1 G5 F5 G5 F5 D5 F5:2 D5:1 C5 A4 | \
             G4 G5 F5 E5 D5 A4:2 B4:1 C5:2 - C6 - | \
             -:1 F4 D5 C5:2 D5:1 C5:2 -:1 E4 D5 C5:2 D5:1 C5:2 | \
             -:1 C5 - C5 - Cb5 - Bb4 A4 C5 D5 F5 E5:2 G5 | \
             A5:1 G#5 A5 C6:2 A5:1 G5 F5 G5 F5 D5 C5:2 D5:1 A4 G4",
            "I V7 | I7 #iv°7 I V7 | I I IV I | II7 V | I V7 | I7 #iv°7 I V7 | I I IV I",
        ),
        // The endings of the first strain: back to its start, or on to B.
        Part::new(
            "A1",
            "F4:1 G4 A4:2 G4:1 G4:2 C4:1 F4:2 -:1 C4 - D4 - E4",
            "ii6 V7 I I",
        ),
        Part::new(
            "A2",
            "F4:1 G4 A4:2 G4:1 G4:2 C4:1 F4:2 C5:1 C5 C5:2 C5",
            "ii6 V7 I I",
        ),
        Part::new(
            "B",
            "E5:1 D5 C5 Bb4 A4 Bb4:2 G4:1 F4 G4 A4 C4:2 A4:1 G4 F4 | \
             E4 F4 G4 C4:2 Bb4:1 A4 G4 F4 A4 D5 C5:3 C5:2 | \
             E5:1 D5 C5 Bb4 A4 Bb4:2 E4:1 F4 E4 F4 A4:2 A4:1 G4 F4 | \
             E4 C5 B4 A4 G#4 B4:2 E4:1 A4:2 - C5:4 | \
             E5:1 D5 C5 Bb4 A4 Bb4:2 G4:1 F4 G4 A4 C4:2 A4:1 G4 F4 | \
             E4 F4 G4 C4:2 Bb4:1 A4 G4 F4 A4 D5 C5:3 C5:2 | \
             D5:1 G5:2 D5 G5:1 D5:2 C5:1 F5:2 C5 F5:1 C5:2",
            "V7 I | V7 I | V7 VI7 vi vi | iii VII7 iii V7 | V7 I | V7 I | ii I",
        ),
        Part::new(
            "B1",
            "Bb4:1 G4 D5 C5:2 Bb4:1 E4:2 F4 C5:1 C5 C5:2 C5",
            "V7 I",
        ),
        Part::new(
            "B2",
            "Bb4:1 G4 D5 C5:2 Bb4:1 E4:2 F4 -:1 C4 - D4 - E4",
            "V7 I",
        ),
        // The first ending again, without the run back to the start.
        Part::new("End", "F4:1 G4 A4:2 G4:1 G4:2 C4:1 F4:2 -:6", "ii6 V7 I I"),
    ],
    form: &["A", "A1", "A", "A2", "B", "B1", "B", "B2", "A", "End"],
    drums_from: 0,
    about: "The first strain is a two-note figure, D and C, where the C comes a sixteenth \
            early and is held across the beat, while the bass creeps up by half steps from F \
            to A and then slides back down under a diminished chord. The second strain is a \
            single line running down the scale, and in its middle it leans toward D minor \
            and A minor through A7 and E7. The piece is in 2/4, two of its bars to a bar here.",
};
