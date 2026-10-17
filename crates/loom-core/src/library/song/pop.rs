//! Pop: nursery rhymes, carols and folk songs over pop beats.

use super::*;

pub(super) const AULD_LANG_SYNE: Song = Song {
    name: "Auld Lang Syne",
    origin: "Scottish folk song, words by Robert Burns, 1788",
    feel: "Ballad",
    genre: Genre::Pop,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 72.0,
    comping: Comping::Ballad,
    bass: Bass::Held,
    rhythm: "Pop ballad",
    style: "Piano ballad",
    parts: &[
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
    ],
    form: &["Intro", "Verse", "Chorus", "Verse", "Chorus"],
    drums_from: 1,
    about: "Sung at New Year all over the world. Like many Scottish tunes, it uses only the \
            five notes of the pentatonic scale: F G A C D.",
};

pub(super) const LONDON_BRIDGE: Song = Song {
    name: "London Bridge Is Falling Down",
    origin: "English nursery rhyme, 17th century",
    feel: "Dance pop",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Stabs,
    bass: Bass::Octaves,
    rhythm: "Dance pop",
    style: "Dance pop",
    parts: &[Part::new(
        "Tune",
        "G5:6 A5:2 G5:4 F5 | E5 F5 G5:8 | D5:4 E5 F5:8 | E5:4 F5 G5:8 | \
         G5:6 A5:2 G5:4 F5 | E5 F5 G5:8 | D5:8 G5 | E5:4 C5:12",
        "I | I | V7 | I | I | I | V7 | I",
    )],
    form: &["Tune", "Tune", "Tune", "Tune"],
    drums_from: 0,
    about: "A playground song over octave bass and offbeat stabs. The tune only needs two \
            chords: the home chord, and V7 at the end of each half.",
};

pub(super) const OLD_MACDONALD: Song = Song {
    name: "Old MacDonald Had a Farm",
    origin: "Folk song, early 20th century",
    feel: "Afrobeats",
    genre: Genre::Pop,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 108.0,
    comping: Comping::Tresillo,
    bass: Bass::Dembow,
    rhythm: "Afrobeats",
    style: "Afropop",
    parts: &[Part::new(
        "Song",
        "G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:12 D5:4 | \
         G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:12 D5:2 D5 | \
         G5:4 G5 G5 D5:2 D5 | G5:4 G5 G5:8 | G5:2 G5 G5:4 G5:2 G5 G5:4 | G5:2 G5 G5 G5 G5:4 G5 | \
         G5:4 G5 G5 D5 | E5 E5 D5:8 | B5:4 B5 A5 A5 | G5:16",
        "I | IV I | I V | I | I | IV I | I V | I | I IV I I | I IV I I | I IV I I | I IV I I | \
         I | IV I | I V | I",
    )],
    form: &["Song", "Song", "Song"],
    drums_from: 0,
    about: "The tresillo, 3 + 3 + 2, pushes the chords and the bass, as in West African pop. \
            \"E-I-E-I-O\" is B B A A G: the third, the second and home.",
};

pub(super) const DECK_THE_HALLS: Song = Song {
    name: "Deck the Halls",
    origin: "Welsh carol \"Nos Galan\", 16th century",
    feel: "Pop",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 112.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Pop beat",
    style: "Four chords",
    parts: &[Part::new(
        "Verse",
        "G5:6 F5:2 E5:4 D5 | C5 D5 E5 C5 | D5:2 E5 F5 D5 E5:6 D5:2 | C5:4 B4 C5:8 | \
         G5:6 F5:2 E5:4 D5 | C5 D5 E5 C5 | D5:2 E5 F5 D5 E5:6 D5:2 | C5:4 B4 C5:8 | \
         D5:6 E5:2 F5:4 D5 | E5:6 F5:2 G5:4 D5 | E5:2 F#5 G5:4 A5:2 B5 C6:4 | B5 A5 G5:8 | \
         G5:6 F5:2 E5:4 D5 | C5 D5 E5 C5 | A5:2 A5 A5 A5 G5:6 F5:2 | E5:4 D5 C5:8",
        "I | I | ii I | I V7 I I | I | I | ii I | I V7 I I | V7 | I V | iii II7 | V | I | I | IV V7 | I V7 I I",
    )],
    form: &["Verse", "Verse", "Verse"],
    drums_from: 0,
    about: "The third line steps out of C major: F sharp and the D7 chord lead to G, the key \
            of the dominant, for two bars, before the last line comes home.",
};

// ---------------------------------------------------------------- Pop

const THREE_BLIND_MICE_TUNE: &str = "B4:6 A4 | G4:6 -:6 | B4:6 A4 | G4:6 -:6 | D5:6 C5:4 C5:2 | B4:6 -:6 | \
     D5:6 C5:4 C5:2 | B4:6 -:4 D5:2 | G5:4 G5:2 F#5 E5 F#5 | G5:4 D5:2 D5:4 D5:2 | \
     G5:2 G5 G5 F#5 E5 F#5 | G5:4 D5:2 D5 D5 D5 | G5:4 G5:2 F#5 E5 F#5 | G5:2 D5 D5 D5:4 C5:2 | \
     B4:6 A4 | G4:6 -:6";
const THREE_BLIND_MICE_CHORDS: &str =
    "I | I | I | I | I | I | I | I | I | I | I | I | I | I | I | I";

// Source: https://raw.githubusercontent.com/yetist/music-score/e4732e0a659a3a017f7290d6217c9e455d8ed61f/abc/friendly-house-senior-choir.abc
// (John Chambers' ABC of "The Everyday Song Book", 1927, no. 40; in G, up a fifth from the book's C)
pub(super) const THREE_BLIND_MICE: Song = Song {
    name: "Three Blind Mice",
    origin: "English round, 17th century",
    feel: "Round",
    genre: Genre::Pop,
    tonic: G,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 126.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    style: "Jig",
    parts: &[
        Part::new("Alone", THREE_BLIND_MICE_TUNE, THREE_BLIND_MICE_CHORDS),
        Part::new("Round", THREE_BLIND_MICE_TUNE, THREE_BLIND_MICE_CHORDS).with_voices(&[0, 4, 8]),
        // The second and third voices carry on from the part before.
        Part::new(
            "Round again",
            THREE_BLIND_MICE_TUNE,
            THREE_BLIND_MICE_CHORDS,
        )
        .with_voices(&[0, -12, 4, -8, 8]),
        Part::new(
            "Last voices",
            THREE_BLIND_MICE_TUNE,
            THREE_BLIND_MICE_CHORDS,
        )
        .with_voices(&[-12, -8]),
    ],
    form: &["Alone", "Round", "Round again", "Last voices"],
    drums_from: 0,
    about: "A round in 6/8 for three voices, each four bars behind the last. The tune walks \
            down three notes, B A G, then four, D C C B, and when the voices overlap those \
            steps fill in the G chord, which is why one chord is enough.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201212037.abc
// (in C, down a tone from the source's D)
pub(super) const SKIP_TO_MY_LOU: Song = Song {
    name: "Skip to My Lou",
    origin: "American play-party song, 19th century",
    feel: "Country",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Country train",
    style: "Country",
    parts: &[Part::new(
        "Song",
        "E5:4 E5:2 E5 C5 C5 C5:4 | E5:4 E5:2 E5 G5:8 | D5:4 D5:2 D5 B4 B4 B4:4 | D5:4 D5:2 D5 F5:8 | \
         E5:4 E5:2 E5 C5 C5 C5:4 | E5:4 E5:2 E5 G5:8 | D5:4 E5:2 F5 E5:4 D5 | C5:8 C5",
        "I | I | V7 | V7 | I | I | V7 | I",
    )],
    form: &["Song", "Song", "Song", "Song"],
    drums_from: 0,
    about: "A play-party song, sung for dancing where fiddles were frowned on. Two chords, C \
            and G7, and a tune that gives each of them one bar: E E E C C C over C, then D D D \
            B B B over G7.",
};

// Source: https://tunearch.org/wiki/Pop_Goes_the_Weasel (T. Westrop's 120 Country Dances, 1860s)
pub(super) const POP_GOES_THE_WEASEL: Song = Song {
    name: "Pop Goes the Weasel",
    origin: "English dance tune, in print by the 1850s",
    feel: "Pop",
    genre: Genre::Pop,
    tonic: G,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 138.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Six-eight",
    style: "Jig",
    parts: &[
        Part::new(
            "A",
            "G4:4 B4:2 A4:4 C5:2 | B4:2 C5 D5 G4:4 -:2 | G4:4 B4:2 A4:4 C5:2 | B4:6 G4:4 -:2 | \
             G4:4 B4:2 A4:4 C5:2 | B4:2 C5 D5 G4:4 -:2 | E5:4 -:2 A4:4 D5:2 | B4:6 G4:4 -:2",
            "I V7 | I | I V7 | I | I V7 | I | IV V7 | I",
        ),
        Part::new(
            "B",
            "G5:4 F#5:2 E5 F#5 G5 | F#5:2 A5 F#5 D5:4 F#5:2 | G5:4 F#5:2 E5 F#5 G5 | F#5:6 D5 | \
             C5:4 B4:2 C5:4 D5:2 | E5:4 F#5:2 G5:6 | E5:4 -:2 A4:4 D5:2 | B4:6 G4:4 -:2",
            "I | V7 | I | V7 | IV | IV I | IV V7 | I",
        ),
    ],
    form: &["A", "B", "A", "B"],
    drums_from: 0,
    about: "An English country dance before it was a nursery rhyme. Every phrase rocks \
            between G and D7, and the \"pop\" is the jump up to E over the C chord, with a \
            rest after it, in the seventh bar of each half.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/jc/abc_tune_jc_201229790.abc
// (John Chambers' ABC of "The Everyday Song Book", 1927, no. 129, "A Merry Heart", in E flat;
// here in F, up a tone, and the chorus is the book's upper voice. The book has no chords:
// they come from the Wikifonia lead sheet in F,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/LuigiDenzaEdwardOxenfordFuniculiFunicula.abc,
// whose notes agree but for a few rests and held notes.)
pub(super) const FUNICULI_FUNICULA: Song = Song {
    name: "Funiculì, funiculà",
    origin: "Luigi Denza, words by Peppino Turco, 1880",
    feel: "Tarantella",
    genre: Genre::Pop,
    tonic: F,
    mode: Major,
    time: SIX_EIGHT,
    tempo: 150.0,
    comping: Comping::Arp3,
    bass: Bass::Waltz,
    rhythm: "Tarantella",
    style: "Six-eight ballad",
    parts: &[
        Part::new("Pickup", "-:10 C5:2", "I"),
        Part::new(
            "Verse",
            "F5:12 | _:4 F5:2 E5:4 E5:2 | C5:4 C5:2 D5:4 D5:2 | A4:6 A4:6 | _:4 A4:2 G4:4 F4:2 | \
             F4:12 | _:2 -:2 A4:2 G4:4 F4:2 | F4:12 | _:2 -:8 C5:2",
            "I | I | I | I | I V7 | I | I V7 | I | I",
        ),
        Part::new(
            "Verse 2",
            "F5:12 | _:4 F5:2 E5:4 E5:2 | C5:4 C5:2 D5:4 D5:2 | A4:6 A4:6 | _:4 A4:2 G4:4 F4:2 | \
             F4:12 | _:2 -:2 A4:2 G4:4 F4:2 | F4:12 | _:2 -:8 A4:2",
            "I | I | I | I | I V7 | I | I V7 | I | I",
        ),
        Part::new(
            "Minor",
            "A4:12 | _:4 A4:2 B4:4 B4:2 | A4:4 A4:2 B4:4 B4:2 | A4:6 A4:6 | _:4 E4:2 E4:4 E4:2 | \
             E4:12 | _:2 -:2 E4:2 E4:4 E4:2 | E4:12 | _:2 -:8 C5:2",
            "iii | iii III7 | iii III7 | iii | iii III7 | iii | iii III7 | iii | iii",
        ),
        Part::new(
            "Major",
            "C5:12 | _:4 C5:2 D5:4 D5:2 | C5:4 C5:2 D5:4 D5:2 | C5:6 C5:6 | _:4 G4:2 G4:4 G4:2 | \
             G4:12 | _:2 -:2 G4:2 G4:4 G4:2 | G4:12 | _:2 -:10",
            "V | V II7 | V II7 | V | V II7 | V | V II7 | V | V",
        ),
        Part::new(
            "Chorus",
            "E5:6 D5:2 -:4 | E5:6 D5:2 -:4 | F5:4 E5:2 D5:4 F5:2 | E5:8 -:4 | \
             E5:6 D5:2 -:4 | E5:6 D5:2 -:4 | F5:4 E5:2 D5:4 F5:2 | C5:2 -:2 A4:2 A4:4 A4:2 | \
             A4:4 A4:2 A4:4 A4:2 | A4:4 A4:2 A4:4 A4:2 | A4:4 A4:2 A4:4 A4:2 | F5:12 | \
             G5:4 F5:2 D5:4 F5:2 | C5:2 -:2 A4:2 A4:4 Bb4:2 | C5:4 Bb4:2 A4:4 G4:2 | F4:2 -:10",
            "V7 | V7 | V7 | V7 | V7 | V7 | V7 | I | VI7 | vi | VI7 | vi | IV | I | V7 | I",
        ),
    ],
    form: &[
        "Pickup", "Verse", "Verse 2", "Minor", "Major", "Chorus", "Chorus",
    ],
    drums_from: 1,
    about: "Written in 1880 for the funicular railway up Vesuvius, in a quick 6/8. The verse \
            is a long high F and a fall to the low F; the middle sings the same shape on A \
            over A minor and E7, and on C over C and G7. Then the chorus \"funiculì, \
            funiculà\" bounces on E and D over C7, lands on F, and goes round again through \
            D7 and D minor before the last cadence.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/folktunefinder/abc_tune_folktunefinder_83846.txt
// (Michel Bellon's galoubet method, exercise 4, in C. The source has no chords.)
pub(super) const AU_CLAIR_DE_LA_LUNE: Song = Song {
    name: "Au clair de la lune",
    origin: "French folk song, 18th century",
    feel: "Pop",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::Strum,
    bass: Bass::RootFifth,
    rhythm: "Pop beat",
    style: "Plagal pop",
    parts: &[
        Part::new(
            "A",
            "C5:4 C5 C5 D5 | E5:8 D5 | C5:4 E5 D5 D5 | C5:16",
            "I | I V | I V | I",
        ),
        Part::new(
            "B",
            "D5:4 D5 D5 D5 | A4:8 A4 | D5:4 C5 B4 A4 | G4:16",
            "V | II7 | V II7 | V",
        ),
    ],
    form: &["A", "A", "B", "A", "A", "B", "A"],
    drums_from: 0,
    about: "A French song in plain quarter notes. The first line climbs C, D, E and settles \
            back on C; the middle line moves to G, the fifth, with D7 (II7), a secondary \
            dominant, pulling toward it, and then the first line comes back.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/folktunefinder/abc_tune_folktunefinder_109888.txt
// (Erich Rickheit's "dtrad" ABC, in C, written here an octave higher. Chords from Frank
// Nordberg's Musica Viva ABC in G, which has the same notes,
// https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/folktunefinder/abc_tune_folktunefinder_109889.txt.)
pub(super) const ALOUETTE: Song = Song {
    name: "Alouette",
    origin: "French-Canadian folk song, traditional",
    feel: "Pop",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Pulse,
    bass: Bass::Pulse,
    rhythm: "Pop beat",
    style: "Four chords",
    parts: &[
        Part::new(
            "Chorus",
            "C5:6 D5:2 E5:4 E5 | D5:2 C5 D5 E5 C5:4 G4 | C5:6 D5:2 E5:4 E5 | \
             D5:2 C5 D5 E5 C5:6 -:2",
            "I | V7 I | I | V7 I",
        ),
        Part::new(
            "Verse",
            "C5:2 D5 E5 F5 G5 G5 G5:4 | G5:2 A5 G5 F5 E5 D5 C5:4 | G5:2 G5 G5:4 G4:2 G4 G4:4 | \
             G5:2 G5 G5:4 G4:2 G4 G4:4 | G5:16",
            "I | V7 I | V7 | V7 | V7",
        ),
        Part::new(
            "Chorus end",
            "C5:6 D5:2 E5:4 E5 | D5:2 C5 D5 E5 C5:4 G4 | C5:6 D5:2 E5:4 E5 | \
             D5:2 C5 D5 E5 C5:8",
            "I | V7 I | I | V7 I",
        ),
    ],
    form: &["Chorus", "Verse", "Chorus", "Verse", "Chorus end"],
    drums_from: 0,
    about: "A French-Canadian song that grows by a line each verse, with two chords from \
            start to end: C and G7. The verse \"je te plumerai la tête\" climbs the scale from C to G and walks back down, \
            and \"et la tête!\" is called and answered an octave apart over G7, holding a whole \
            bar before the chorus comes back.",
};

// Source: https://raw.githubusercontent.com/cuthbertLab/music21/master/music21/corpus/essenFolksong/fink0.abc
// (tune X:58, from the Essen collection's copy of Fink's Musikalischer Hausschatz, 1843; the
// kern copy https://raw.githubusercontent.com/annamatuszewska/The-Essen-Collection/main/krn/am_fink/am_deut058.krn
// shows the stray D in bar 11 is a grace note; chords from the cello and violin parts of
// https://raw.githubusercontent.com/cellist/Lilypond-Sheet-Music/master/Fl,%20Vl,%20Vlc/Mozart,%20Wolfgang%20Amadeus/Vogelfaenger/v3.ily)
pub(super) const VOGELFAENGER: Song = Song {
    name: "Der Vogelfänger bin ich ja",
    origin: "Wolfgang Amadeus Mozart, The Magic Flute, 1791",
    feel: "Two-step",
    genre: Genre::Pop,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 84.0,
    comping: Comping::Offbeat,
    bass: Bass::RootFifth,
    rhythm: "Two-step",
    style: "Polka",
    parts: &[
        Part::new("Pickup", "-:14 B4:1 A4:1", ""),
        Part::new(
            "Verse",
            "G4:2 G4:2 A4:1 G4:1 F#4:1 G4:1 A4:2 B4:2 A4:2 A4:1 F#4:1 | \
             D4:2 D4:2 D5:3 D5:1 B4:2 A4:2 G4:2 B4:2 | \
             A4:1 G4:1 F#4:1 G4:1 A4:1 G4:1 F#4:1 G4:1 A4:2 B4:2 A4:2 -:1 A4:1 | \
             D5:3 D5:1 A4:3 A4:1 F#4:2 E4:2 D4:2 D4:1 F#4:1 | \
             A4:2 A4:2 B4:1 A4:1 G4:1 A4:1 B4:2 A4:2 G4:2 D5:2 | \
             F#4:3 F#4:1 A4:1 G4:1 F#4:1 G4:1 A4:2 B4:2 A4:2 G4:1 A4:1 | \
             B4:3 B4:1 C5:1 B4:1 A4:1 B4:1 C5:2 D5:2 E5:2 C5:1 A4:1 | \
             F#4:3 F#4:1 G4:1 F#4:1 E4:1 F#4:1 G4:1 B4:1 A4:1 F#4:1 G4:2 B4:1 A4:1",
            "I V | V I | V | V II II7 V | V I | V7 I V V | I7 IV | V7 I",
        ),
        Part::new(
            "Last verse",
            "G4:2 G4:2 A4:1 G4:1 F#4:1 G4:1 A4:2 B4:2 A4:2 A4:1 F#4:1 | \
             D4:2 D4:2 D5:3 D5:1 B4:2 A4:2 G4:2 B4:2 | \
             A4:1 G4:1 F#4:1 G4:1 A4:1 G4:1 F#4:1 G4:1 A4:2 B4:2 A4:2 -:1 A4:1 | \
             D5:3 D5:1 A4:3 A4:1 F#4:2 E4:2 D4:2 D4:1 F#4:1 | \
             A4:2 A4:2 B4:1 A4:1 G4:1 A4:1 B4:2 A4:2 G4:2 D5:2 | \
             F#4:3 F#4:1 A4:1 G4:1 F#4:1 G4:1 A4:2 B4:2 A4:2 G4:1 A4:1 | \
             B4:3 B4:1 C5:1 B4:1 A4:1 B4:1 C5:2 D5:2 E5:2 C5:1 A4:1 | \
             F#4:3 F#4:1 G4:1 F#4:1 E4:1 F#4:1 G4:1 B4:1 A4:1 F#4:1 G4:4",
            "I V | V I | V | V II II7 V | V I | V7 I V V | I7 IV | V7 I",
        ),
    ],
    form: &["Pickup", "Verse", "Verse", "Last verse"],
    drums_from: 1,
    about: "Papageno the bird-catcher introduces himself: \"The bird-catcher, that's me, \
            always merry, heisa, hopsasa!\" It is in 2/4, with two of its bars to each bar \
            here, and sung three times, once for each verse. The tune is the version in \
            Fink's German songbook of 1843; between its lines the opera has Papageno's \
            panpipes, five quick notes up, which are left out here, as is a grace note.",
};

// Source: https://raw.githubusercontent.com/mtgskynote/skynote-node/main/client-jb/public/xmlScores/violin/V_071_surprise_symphony.xml
// (a violin study score in G, with doubled note values, moved down a fifth to Haydn's C and
// halved here; its bar 9, which the guitar tab does not have, is left out; the guitar tab
// https://raw.githubusercontent.com/hyperdrives/classtab/master/classtab/haydn_h01_94_symphony_94_in_g_surprise_2_andante.txt
// has the same tune and gives the chords; the "easy version" in C,
// https://raw.githubusercontent.com/baobach/abc_search/main/database/abc/QmdUauH9VqGnEtRTkQF4EXmh4Kk4UsEoMpLrsmwqs66Bip.abc,
// agrees on the first half)
pub(super) const SURPRISE_SYMPHONY: Song = Song {
    name: "Surprise Symphony",
    origin: "Joseph Haydn, Symphony No. 94, 1791",
    feel: "Pop",
    genre: Genre::Pop,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 96.0,
    comping: Comping::Pulse,
    bass: Bass::Octaves,
    rhythm: "Pop beat",
    style: "Four chords",
    parts: &[
        Part::new(
            "Theme",
            "C4:2 C4:2 E4:2 E4:2 G4:2 G4:2 E4:4 | F4:2 F4:2 D4:2 D4:2 B3:2 B3:2 G3:4 | \
             C4:2 C4:2 E4:2 E4:2 G4:2 G4:2 E4:4 | C5:2 C5:2 F#4:2 F#4:2 G4:4 G3:2 -:2",
            "I | V7 V | I | vi II7 V V",
        ),
        Part::new(
            "Second half",
            "F4:2 G3:2 E4:2 G3:2 D4:2 D4:2 D4:1 E4:1 F4:1 G4:1 | \
             A4:2 G4:2 F4:2 E4:2 D4:2 D4:2 D4:3 D#4:1 | \
             E4:2 E4:2 G4:2 G4:2 C5:2 C5:2 E5:4 | \
             D5:2 D5:2 C5:1 B4:1 A4:1 B4:1 C5:2 C5:2 C5:2 -:2",
            "V7 I V V | V7 V | I | ii V7 I I",
        ),
    ],
    form: &["Theme", "Theme", "Second half", "Second half"],
    drums_from: 0,
    about: "The slow movement of Haydn's Symphony No. 94 opens with a tune as plain as a nursery \
            rhyme, climbing and falling on the notes of its chords, C and G7, played very \
            softly. It is in 2/4, with two of its bars to each bar here. The low G that ends the \
            theme is where, when the theme is repeated, the whole orchestra suddenly crashes in, \
            very loud: that is the surprise.",
};
