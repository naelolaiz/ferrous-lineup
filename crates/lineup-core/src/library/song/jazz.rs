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
    band: SWING,
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
    band: DIXIELAND,
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

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/JoplinS/entertainer/entertainer.ly
// (Mutopia, from the original edition of 1902; the top line of the right hand, in C
// as written, two bars of its 2/4 to a bar. The repeat of the second strain, which
// Joplin marks an octave higher, is played as written, and so is the run into it.)
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
    band: PIANO,
    style: "New Orleans",
    parts: &[
        // The introduction is in octaves, without chords: the line alone.
        Part::new(
            "Intro",
            "D6:1 E6 C6 A5:2 B5:1 G5:2 D5:1 E5 C5 A4:2 B4:1 G4:2 | \
             D4:1 E4 C4 A3:2 B3:1 A3 Ab3 G3:2 - G5 D4:1 D#4",
            "",
        ),
        Part::new(
            "A",
            "E4:1 C5:2 E4:1 C5:2 E4:1 C5:6 C6:1 D6 D#6 | \
             E6 C6 D6 E6:2 B5:1 D6:2 C6:6 D4:1 D#4 | \
             E4 C5:2 E4:1 C5:2 E4:1 C5:7 A5:1 G5 | \
             F#5 A5 C6 E6:2 D6:1 C6 A5 D6:6 D4:1 D#4 | \
             E4 C5:2 E4:1 C5:2 E4:1 C5:6 C6:1 D6 D#6 | \
             E6 C6 D6 E6:2 B5:1 D6:2 C6:6 C6:1 D6 | \
             E6 C6 D6 E6:2 C6:1 D6 C6 E6 C6 D6 E6:2 C6:1 D6 C6",
            "I I IV I | I V7 I I | I IV | II7 V7 | I I IV I | I V7 I I | I7 I7 IV iv",
        ),
        // The endings of the first strain: back to its start, or on to B.
        Part::new("A1", "E6:1 C6 D6 E6:2 B5:1 D6:2 C6:6 D4:1 D#4", "I V7 I I"),
        Part::new(
            "A2",
            "E6:1 C6 D6 E6:2 B5:1 D6:2 C6:5 E5:1 F5 F#5",
            "I V7 I I",
        ),
        Part::new(
            "B",
            "G5:2 A5:1 G5:2 E5:1 F5 F#5 G5:2 A5:1 G5:2 E5:1 C5 G4 | \
             A4 B4 C5 D5 E5 D5 C5 D5 G4 E5 F5 G5 A5 G5 E5 F5 | \
             G5:2 A5:1 G5:2 E5:1 F5 F#5 G5:2 A5:1 G5:2 G5:1 A5 A#5 | \
             B5 B5:2 B5 A5:1 F#5 D5 G5:5 E5:1 F5 F#5 | \
             G5:2 A5:1 G5:2 E5:1 F5 F#5 G5:2 A5:1 G5:2 E5:1 C5 G4 | \
             A4 B4 C5 D5 E5 D5 C5 D5 C5:5 G4:1 F#4 G4 | \
             C5:2 A4:1 C5:2 A4:1 C5 A4 G4 C5 E5 G5:2 E5:1 C5 G4",
            "I | IV iv I I | I | V II7 V7 V7 | I | IV iv I I7 | IV #iv°7 I I",
        ),
        // Joplin has the repeat of B played an octave higher ("Repeat 8va"), with this run
        // written an octave up to lead into it; both are played as written here.
        Part::new("B1", "A4:2 C5 E5:1 D5:2 C5:6 E5:1 F5 F#5", "II7 V7 I I"),
        Part::new("B2", "A4:2 C5 E5:1 D5:2 C5:7 D4:1 D#4", "II7 V7 I I"),
        // The first strain again, ending on the tonic before the trio.
        Part::new("A3", "E6:1 C6 D6 E6:2 B5:1 D6:2 C6:4 C6:2 -", "I V7 I I"),
        // The trio, in F major.
        Part::new(
            "C",
            "A5:1 G#5 A5:4 C6:2 D6:8 | \
             F5:1 E5 F5:4 A5:2 Bb5:7 G5:1 | \
             D5:2 G5:1 D5:2 G5:1 D5:2 C5:4 F5 | \
             E5:1 G#5 B5 E6:2 D6:1 B5 C6 A5:4 Bb5 | \
             A5:1 G#5 A5:4 C6:2 D6:8 | \
             F5:1 E5 F5:4 A5:2 Bb5:7 G5:1 | \
             D5:2 G5:1 D5:2 G5:1 D5:2 C5:4 F5:3 F5:1",
            "I IV | vi ii | ii ii I vi | III7 III7 vi V7 | I IV | vi ii | ii ii I bVI7",
        )
        .in_key(F, Major),
        Part::new(
            "C1",
            "A5:1 C6:2 G5 C5:1 D5 E5 F5:2 B4:1 C5 D5 E5 F5 G5",
            "I V7 I I",
        )
        .in_key(F, Major),
        Part::new("C2", "A5:1 C6:2 G5 C5:1 D5 E5 F5:2 - F6 -", "I V7 I I").in_key(F, Major),
        // Four bars back to C major, the end of the second strain again.
        Part::new(
            "Interlude",
            "C5:2 A4:1 C5:2 A4:1 C5 A4 G4 C5 E5 G5:2 E5:1 C5 G4 | \
             A4:2 C5 E5:1 D5:2 C5:5 C6:2 -",
            "IV #iv°7 I I | II7 V7 I I",
        ),
        Part::new(
            "D",
            "F4:2 E4:1 F4:2 E4:1 F4:2 -:1 A4 D5 A4 C5 D5 C5 A4 | \
             G4:2 F#4:1 G4:2 F#4:1 G4:2 -:1 C5 E5 C5 D5 E5 D5 C5 | \
             D5:2 C#5:1 D5:2 C#5:1 D5:2 -:1 F5 A5 F5 G5 A5 G5 F5 | \
             C6 C6 C6:4 A5:2 G5 G4:1 G4 G4:2 G4 | \
             F4 E4:1 F4:2 E4:1 F4:2 -:1 A4 D5 A4 C5 D5 C5 A4 | \
             G4:2 F#4:1 G4:2 F#4:1 G4:2 -:1 C5 E5 C5 D5 E5 D5 C5 | \
             A4 G#4 A4 G5:2 F5 C5:1 E5 D#5 E5 A5:2 C6:1 G5 E5",
            "ii | I | V7 | #ii°7 I | ii | I | IV I",
        ),
        Part::new("D1", "C5:2 C5 E5:1 D5:2 C5:3 G4:1 G4 G4:2 G4", "II7 V7 I I"),
        Part::new("D2", "C5:2 C5 E5:1 D5:2 C5:5 C6:2 -", "II7 V7 I I"),
    ],
    form: &[
        "Intro",
        "A",
        "A1",
        "A",
        "A2",
        "B",
        "B1",
        "B",
        "B2",
        "A",
        "A3",
        "C",
        "C1",
        "C",
        "C2",
        "Interlude",
        "D",
        "D1",
        "D",
        "D2",
    ],
    drums_from: 1,
    about: "Ragtime, the piano music jazz grew out of: the left hand keeps a steady oom-pah, \
            bass then chord, while the right hand pushes against it, landing between the beats \
            and holding on across them. After four bars in octaves, Joplin's strains come A A B \
            B A C C D D: the trio (C) moves to F major, the IV, with a D flat 7 (bVI7) before \
            its cadence, and four bars from the end of the second strain lead back to C major \
            for the last. The piece is in 2/4, two of its bars to a bar here; Joplin marks the \
            repeat of the second strain an octave higher, played here as written.",
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
    band: JAZZ_WALTZ,
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
    origin: "Written for Ferrous Lineup",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 150.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SWING,
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
// (the note table of the DCML corpus score, all 128 bars: the top line of the
// right hand, two 2/4 bars to a bar, with half a bar's rest before the first
// and after the last; the tempo words from the same corpus's score,
// https://raw.githubusercontent.com/DCMLab/debussy_childrens_corner/main/MS3/l113-06_childrens_golliwoggs.mscx)
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
    band: PIANO,
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
        // Bars 10-25; again in bars 90-105.
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
        // Bars 26-37: the same rhythm in thirds over B flat 7, then the
        // right hand holds a chord while the left plays the tune; the low
        // unison run is written an octave up.
        Part::new(
            "Second theme",
            "Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:4 Bb5:2 -:2 | \
             Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:2 Eb4:2 F4:2 C5:2 | Eb4:16 | \
             Eb4:8 -:1 Bb3:1 C4:1 D4:1 C4:2 Bb3:2 | \
             Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:4 Bb5:2 -:2 | \
             Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:2 Eb4:2 F4:2 C5:2",
            "V7 | iii7 | V11 | vi7 V | V7 | iii7",
        ),
        // Bars 38-45, and again 118-125: the opening run over a held C flat
        // and E flat, then the vamp, thinning out.
        Part::new(
            "Vamp",
            "Bb5:1 Ab5:2 Bb5:1 F5:2 Bb5:2 Ab5:1 F5:2 Eb5:1 Cb5:4 | \
             -:1 Ab4:1 F4:1 Eb4:1 Cb4:2 G3:2 -:2 Bb3:2 -:2 Bb3:2 | \
             -:10 Bb3:2 -:2 Bb3:2 | -:10 Bb3:2 -:2 Bb3:2",
            "iv6 | iv6 V I I | I | I",
        ),
        // Bars 46-61: the bass walks up E flat, F to G flat, "Un peu moins
        // vite" (a little slower), the chords off the beat over a G flat
        // pedal; "Cédez" (give way) at the A that starts the quotation.
        Part::new(
            "G flat",
            "-:10 Db4:2 -:2 Db4:2 | -:2 Cb4:2 -:2 F4:2 -:2 Db4:2 -:2 Db4:2 | \
             -:2 Cb4:2 -:2 F4:2 -:2 Db4:2 -:4 | -:4 Db4:2 -:10 | \
             -:4 Db3:2 -:4 Db4:2 -:2 Db4:2 | -:2 Cb4:2 -:2 F4:2 -:2 Db4:2 -:2 Db4:2 | \
             -:2 Cb4:2 -:2 F4:2 -:2 Db4:2 -:4 | -:12 A3:4",
            "VI I | I | I | I | V7 I | I | I | V",
        )
        .in_key(FSharp, Major)
        .with_tempo(&[tempo(8, 96), glide(120, 72, 8)]),
        // Bars 62-89: Tristan's A, F, E, "avec une grande émotion", four
        // times, each answered "a Tempo" by the cakewalk's chords; then a
        // held chord over D flat with the theme's first notes, "Retenu"
        // (held back). The score gives words, not numbers.
        Part::new(
            "Tristan",
            "F4:6 E4:2 Eb4:2 Eb5:2 Db5:2 Eb5:2 | -:2 Eb5:2 Db5:2 Eb5:2 -:4 A3:4 | \
             F4:6 E4:2 Eb4:2 Eb5:2 Db5:2 Eb5:2 | -:2 Eb5:2 Db5:2 Eb5:2 -:4 A4:4 | \
             F5:2 Ab5:4 Gb5:2 Eb5:1 Gb5:2 F5:1 Db5:4 | Ab4:1 Cb5:2 Bb4:1 Db4:4 Db4:4 A3:4 | \
             F4:6 E4:2 Eb4:2 Eb5:2 Db5:2 Eb5:2 | -:4 A3:4 F4:6 E4:2 | \
             Eb4:2 Eb5:2 Db5:2 Eb5:2 -:4 Db5:4 | A5:2 Cb6:4 Bb5:2 F5:1 Ab5:2 Gb5:1 Eb5:4 | \
             Bb4:1 Db5:2 Cb5:1 F4:3 Gb4:1 Eb4:8 | _:11 Bb4:1 Ab4:2 Bb4:2 | \
             -:3 Bb4:1 Ab4:2 Bb4:2 Db4:8 | _:3 Bb4:1 Ab4:2 Bb4:2 Db4:3 Bb4:1 Ab4:2 Bb4:2",
            "VII #IV7 V9 V9 | V9 | V+ I6 | I6 | V7 | ii7 V7 I I | #iø7 V9 | I I VII7 VIImaj7 | \
             I6 | I6 IV6 | IV V7 V11 V11 | V11 | V11 V7sus4 | V7",
        )
        .in_key(FSharp, Major)
        .with_tempo(&[
            tempo(8, 96),
            glide(28, 72, 4),
            tempo(40, 96),
            glide(60, 72, 4),
            tempo(72, 96),
            glide(88, 72, 8),
            tempo(104, 96),
            glide(112, 72, 8),
            tempo(128, 96),
            glide(200, 84, 24),
        ]),
        // Bars 90-105: the theme as in bars 10-25, "Toujours retenu" (still
        // held back) for two bars, then "1o Tempo".
        Part::new(
            "Theme again",
            "Bb4:1 Ab4:2 Bb4:1 F4:2 Bb4:2 Ab4:1 F4:2 Eb4:1 Cb4:2 Eb4:2 | Bb3:1 Bb3:2 C4:1 Bb3:2 Bb4:2 Bb3:8 | \
             Bb3:1 Ab3:2 Bb3:1 C4:2 Ab3:2 Bb3:1 Ab3:2 Bb3:1 Cb4:2 Ab3:2 | \
             Bb4:1 G4:2 F4:1 Eb4:2 D4:2 C4:2 G3:2 C4:2 D4:2 | \
             Bb4:1 Ab4:2 Bb4:1 F4:2 Bb4:2 Ab4:1 F4:2 Eb4:1 Cb4:2 Eb4:2 | Bb3:1 Bb3:2 C4:1 Bb3:2 Bb4:2 Bb3:8 | \
             Bb3:1 Bb3:2 C4:1 Bb3:2 Db4:2 Eb4:2 F4:2 Eb4:2 Gb4:2 | \
             F4:1 F4:2 G4:1 F4:2 D5:2 Bb4:4 Bb5:2 -:2",
            "I | I | I | I I vi V7 | I | I | bIII | II7 V",
        )
        .with_tempo(&[glide(0, 72, 16), tempo(16, 108)]),
        // Bars 106-117: the second theme again, with an A major chord over B
        // sliding down onto B flat 9 in its middle.
        Part::new(
            "Second theme 2",
            "Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:4 Bb5:2 -:2 | \
             Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:2 Eb4:2 F4:2 C5:2 | E4:8 F4:8 | Bb3:16 | \
             Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:4 Bb5:2 -:2 | \
             Eb4:1 F4:2 G4:1 F4:2 Eb4:2 F4:2 Eb4:2 F4:2 C5:2",
            "V7 | iii7 | #V11 V9 | V6 I | V7 | iii7",
        ),
        // Bars 126-128: the last run, C flat and then B flat, A flat, G, F
        // in thirty-seconds (here B flat and G, on their own beats), and two
        // E flats, all an octave up.
        Part::new("Coda", "-:4 Cb4:2 Bb3:1 G3:1 Eb4:2 -:6 | Eb4:2 -:14", "I | I"),
    ],
    form: &[
        "Intro",
        "Theme",
        "Second theme",
        "Vamp",
        "G flat",
        "Tristan",
        "Theme again",
        "Second theme 2",
        "Vamp",
        "Coda",
    ],
    drums_from: 1,
    about: "A cakewalk, the strutting dance of ragtime, with Debussy's blue note: the C flat \
            that keeps falling onto B flat on the off-beat. The theme's second phrase slides \
            into G flat major, the flat-three chord, and comes back through F7 and B flat; the \
            middle section, a little slower, settles in G flat and quotes the opening of \
            Wagner's Tristan und Isolde, A rising to F and sinking to E, \"with great \
            emotion\", each time answered by the cakewalk's chords. It ends on D flat 7, the \
            V7 of G flat, and the theme drops straight back into E flat. The piece is in 2/4, \
            two of its bars to a bar here; the opening run falls through three octaves, and \
            the low unison runs and the last notes are written an octave up.",
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
    band: GYPSY_JAZZ,
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
    band: PIANO,
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
        // The first strain again, its last eighth an A flat leading into the trio.
        Part::new(
            "A3",
            "-:1 Ab4 Eb5 Ab4 C5 Eb5:2 G4:1 Eb5 G4 Bb4 Eb5:5 | \
             -:1 Ab4 Eb5 Ab4 C5 Eb5:2 G4:1 Eb5 G4 Bb4 Eb5:3 -:1 Eb5 | \
             - Ab4 Cb5 Fb5 - Eb5 - Eb5 - Ab4 Cb5 Fb5 - Eb5 -:2 | \
             -:1 Ab3 Cb4 Ab4 - Ab3 Cb4 Ab4 - Ab4 Cb5 Ab5 - Ab4 Cb5 Ab5 | \
             Ab5:2 Ab5 Ab5 Ab5:1 Ab5 _ Eb5 F5 C5 Eb5 F5:2 Ab4:1 | \
             _ Bb4 Cb5 Ab4 Bb4 C5:2 Ab4:1 C5 Ab4 Bb4:2 Ab4 -:1 Ab5 | \
             _:2 Ab5 Ab5 Ab5:1 Ab5 _ Eb5 F5 C5 Eb5 F5:2 Ab4:1 | \
             _ Bb4 Cb5 Ab4 Bb4 C5:2 Ab4:1 C5 Ab4 Bb4:2 Ab4 Ab5",
            "I V7 | I V7 | bVI V bVI V | i | #ii°7 I | bVI I V7 I | #ii°7 I | bVI I V7 I",
        ),
        // The trio, in D flat major.
        Part::new(
            "C",
            "Ab5:3 Ab5:2 Eb5:1 Bb4:2 Ab5:3 Ab5:2 Eb5:1 Bb4 Eb5 | \
             Ab4 Db5 Bb4 Db5 F5 Ab4 Db5 F5 Bb4 Db5 F5 Ab4:2 F5:1 Bb4 F5 | \
             Ab5:3 Ab5:2 Eb5:1 Bb4:2 Ab5:3 Ab5:2 Eb5:1 Bb4 Eb5 | \
             Ab4 Db5 Bb4 Db5 F5 Ab4 Db5 F5 Bb4 Db5 F5 Ab4:2 F5:1 A4 F5 | \
             Bb5:3 Bb5:2 F5:1 C5:2 Bb5:3 Bb5:2 F5:1 C5 F5 | \
             - Eb5 Bb5 Eb5 Gb5 C6:2 Eb5:1 Bb5 Eb5 Gb5 C6:2 Eb5:1 Bb5:2 | \
             Db6 Db6 C6 Bb5 F5:1 Db5 Eb5 Gb5:2 Bb4:1 F5:2",
            "V7 | I | V7 | I | VI7 | ii | #iv°7 #iv°7 I VI7",
        )
        .in_key(CSharp, Major),
        Part::new(
            "C1",
            "F5:1 Db5 Eb5 F5:2 C5:1 Eb5 Db5:2 Ab5 F5:1 Ab5 F5 Ab5 F5",
            "II7 V7 I I",
        )
        .in_key(CSharp, Major),
        Part::new(
            "C2",
            "F5:1 Db5 Eb5 F5:2 C5:1 Eb5 Db5:2 Db5:1 F5 Ab5 Db6:2 -",
            "II7 V7 I I",
        )
        .in_key(CSharp, Major),
        // The last strain, back in A flat. From its eleventh bar to the ending the tune is
        // tied across every bar line, so each ending carries the five bars before it.
        Part::new(
            "D",
            "Ab5:2 F5 Ab5 F5 Ab5 Bb5:1 C6:2 Bb5:1 Ab5 F5 | \
             Eb5 F5:2 C5:5 -:1 Eb5 F5 C5 Eb5 F5:2 C5:1 | \
             Eb5:2 F5:1 Bb4:5 -:1 Db5 F5 Bb4 Db5 F5:2 C5:1 | \
             _ Eb5 F5 C5 Eb5 F5:2 C5 Eb5:1 F5 C5 Eb5 F5:2 Eb5:1 | \
             Ab5:2 F5 Ab5 F5 Ab5 Bb5:1 C6:2 Bb5:1 Ab5 F5",
            "IV | I | V7 | I | IV",
        ),
        Part::new(
            "D1",
            "Ab5:2 F5 Eb5:1 Ab5:2 C5 Eb5:1 F5 C5 Eb5 F5:2 Ab4:1 | \
             _ Bb4 Ab4:2 Ab4 Bb4:1 Ab4:2 Bb4:1 C5 Ab4 Bb4 C5:2 Ab4:1 | \
             _ Bb4 C5 Ab4 - Bb4:2 Eb4:1 Ab4:2 Eb5 Eb5 Eb5",
            "I | IV iv I I | vi6 V7 I I",
        ),
        Part::new(
            "D2",
            "Ab5:2 F5 Eb5:1 Ab5:2 C5 Eb5:1 F5 C5 Eb5 F5:2 Ab4:1 | \
             _ Bb4 Ab4:2 Ab4 Bb4:1 Ab4:2 Bb4:1 C5 Ab4 Bb4 C5:2 Ab4:1 | \
             _ Bb4 C5 Ab4 - Bb4:2 Eb4:1 Ab4:2 Eb5 Ab5 -",
            "I | IV iv I I | vi6 V7 I I",
        ),
    ],
    form: &[
        "A", "A", "B", "B1", "B", "B2", "A3", "C", "C1", "C", "C2", "D", "D1", "D", "D2",
    ],
    drums_from: 0,
    about: "The rag that made ragtime famous. The tune is a broken chord in sixteenths that \
            keeps landing just after the beat, over a left hand that never stops its oom-pah. \
            Listen for the F flat chord (bVI) that darkens the third bar, and the diminished \
            seventh under the repeated A flats that resolves back to the tonic. The strains come \
            A A B B A C C D D: the trio (C) moves to D flat major, the IV, opening on its own \
            dominant, A flat 7, and passing through B flat 7 to E flat minor, and the last \
            strain comes home to A flat. The piece is in 2/4, two of its bars to a bar here; the \
            climb through four octaves is folded into two, and one high phrase is played an \
            octave down.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/I_Got_Rhythm/I_Got_Rhythm.mxl
// (OpenEWLD, the public-domain part of the Wikifonia lead sheets: the two-bar
// introduction, the verse and the refrain with its two endings; the same lead sheet as
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/HEAD/wikifonia-dataset/original-abc/GeorgeGershwinIraGershwinIGotRhythm.abc.
// The refrain's chords checked against the iReal chart of the standard; the sheet's "Em6"
// over the melody's B flat, in bar 8 of the verse, is read as E flat minor 6, as the same
// spot of the refrain is written E flat minor further on.)
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
    band: SWING,
    style: "Rhythm changes",
    parts: &[
        Part::new("Intro", "D5:2 D5:4 D5:2 F5:4 Eb5 | D5:4 D4:12", "vi | III7"),
        // "Days can be sunny, with never a sigh..."
        Part::new(
            "Verse",
            "Bb4:4 G4 Bb4:8 | C5:4 C5:8 G4:4 | Bb4 G4 Bb4:8 | Db5:16 | \
             Bb4:4 G4 Bb4:8 | D5:4 D5:8 Bb4:4 | G4:16 | _:12 -:4 | \
             Bb4:4 G4 Bb4:8 | C5:4 C5:8 G4:4 | Bb4 G4 Bb4:8 | Db5:16 | \
             Bb4:4 G4 Bb4:8 | D5:4 D5:8 D5:4 | Bb4:16 | _:12 -:4",
            "vi | ii | vi6 | IV7 | vi IV | vi6 vi6 vi6 iv6 | vi iii7 vi vi | iii7 iii7 iii7 vi | \
             vi | ii | vi6 | IV7 | vi vi7 | ii7 ii7 ii7 V7 | I v7 | v7 I",
        ),
        // "I'm chipper all the day...", and the lead-in to the refrain.
        Part::new(
            "Verse 2",
            "D5:4 - D4 D4 | F4 F4 D4:8 | Eb4:4 F4 G4 Bb4 | Db5:16 | \
             D5:4 - D4 D4 | F4 F4 D4:8 | Eb4:4 F4 G4 Bb4 | C5:16 | \
             F5:2 F5:4 F5:2 Ab5:4 Gb5 | F5:16",
            "III III7 | V+ III7 | ii7 | IV7 | III III7 | V+ III7 | ii7 | V7 | #iv° bVI7 | \
             V7 i6 iii° V7",
        ),
        Part::new(
            "A",
            "-:4 F4:6 G4 | Bb4 C5:10 | -:4 C5:6 Bb4 | G4 F4:10 | \
             -:4 F4:6 G4 | Bb4 C5:2 - Eb5 - C5 | D5:4 D5 C5:2 D5 C5 Bb4 | _:16",
            "I vi7 | ii7 V7 | I vi7 | ii7 V7 | I I7 | IV iv | I V7 | I #ii° V7 V7",
        ),
        Part::new(
            "A2",
            "-:4 F4:6 G4 | Bb4 C5:10 | -:4 C5:6 Bb4 | G4 F4:10 | \
             -:4 F4:6 G4 | Bb4 C5:2 - Eb5 - C5 | D5:4 D5 C5:2 D5 C5:4 | Bb4:16",
            "I vi7 | ii7 V7 | I vi7 | ii7 V7 | I I7 | IV iv | I V7 | I",
        ),
        Part::new(
            "B",
            "-:4 D5:6 D5 | D5 E5:10 | -:4 D5:6 D5 | D5 G4:10 | \
             -:4 C5:6 C5 | C5 D5:10 | -:4 C5:6 C5 | C5:16",
            "III7 | III7 | VI7 | VI7 | II7 | II7 | V7 | V7",
        ),
        // The last A with the two-bar tag, "who could ask for anything more?", and the
        // first ending, back to the top of the refrain.
        Part::new(
            "A3",
            "-:4 F4:6 G4 | Bb4 C5:10 | -:4 C5:6 Bb4 | G4 F4:10 | \
             -:4 F4:6 G4 | Bb4 C5:4 Eb5 C5:2 | D5:4 D5 C5:2 D5 C5:4 | F5:12 Eb5:2 Bb4 | \
             D5:4 D5 C5:2 D5 C5:4 | Bb4:16",
            "I vi7 | ii7 V7 | I vi7 | ii7 V7 | I I7 | IV iv | I v | VI7 | II7 V7 | I V7",
        ),
        // The same with the second ending.
        Part::new(
            "A end",
            "-:4 F4:6 G4 | Bb4 C5:10 | -:4 C5:6 Bb4 | G4 F4:10 | \
             -:4 F4:6 G4 | Bb4 C5:4 Eb5 C5:2 | D5:4 D5 C5:2 D5 C5:4 | F5:12 Eb5:2 Bb4 | \
             D5:4 D5 C5:2 D5 C5:4 | Bb4:12 -:4",
            "I vi7 | ii7 V7 | I vi7 | ii7 V7 | I I7 | IV iv | I v | VI7 | II7 V7 | I",
        ),
    ],
    form: &[
        "Intro", "Verse", "Verse 2", "A", "A2", "B", "A3", "A", "A2", "B", "A end",
    ],
    drums_from: 0,
    about: "The verse leans on G minor, the relative minor, with a blue D flat over E flat 7, \
            before the refrain sets off in B flat. The chords of the refrain became \"rhythm \
            changes\", the progression jazz musicians have written most new tunes on after \
            the blues. The A section turns I-vi-ii-V twice, then I-I7-IV-iv; the bridge is a \
            chain of dominants, two bars each, falling by fifths from D7 back to F7. The tune \
            opens on four notes, F G B flat C, pushed off the beat.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Hughie_Cannon/Bill_Bailey%2C_Won%27t_You_Please_Come_Home%3F/Bill_Bailey%2C_Won%27t_You_Please_Come_Home%3F.mxl
// (OpenEWLD lead sheet: the chorus with its two endings; the verse is not in it. Chords
// checked against the iReal charts of the standard and of the Dixieland book,
// https://raw.githubusercontent.com/Dazzid/chromaflow/HEAD/data/iRealXML/Bill%20Bailey%20(Dixieland%20Tunes).xml:
// "fine tooth comb" is on C, the IV, reached through G7, where the lead sheet has A minor.)
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
    band: DIXIELAND,
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
        // The second half with the first ending, back to the start.
        Part::new(
            "Chorus 2",
            "-:4 D4 E4:2 E4 G4:4 | B4 A#4 B4 D5 | - D4 E4 G4 | B4:12 D4:4 | \
             D4 D4 G4 B4 | D5:8 F5 | E5:16 | E5:12 G4:4 | \
             G4 G4:8 G4:4 | A4:8 G4 | D5:4 D5:8 E5:4 | B4:12 B4:4 | \
             B4 A4 G#4 A4 | D5:8 B4 | G4:16 | _:4 -:12",
            "I | I | I | I | I | I I7 | IV | IV | IV | #iv°7 | I | VI7 | II7 | V7 | I | II7 V7",
        ),
        // The second half with the second ending.
        Part::new(
            "Chorus end",
            "-:4 D4 E4:2 E4 G4:4 | B4 A#4 B4 D5 | - D4 E4 G4 | B4:12 D4:4 | \
             D4 D4 G4 B4 | D5:8 F5 | E5:16 | E5:12 G4:4 | \
             G4 G4:8 G4:4 | A4:8 G4 | D5:4 D5:8 E5:4 | B4:12 B4:4 | \
             B4 A4 G#4 A4 | D5:8 B4 | G4:16 | _:4 -:12",
            "I | I | I | I | I | I I7 | IV | IV | IV | #iv°7 | I | VI7 | II7 | V7 | I | I",
        ),
    ],
    form: &["Chorus", "Chorus 2", "Chorus", "Chorus end"],
    drums_from: 0,
    about: "A ragtime song from 1902 that Dixieland bands still play. The first half takes \
            its time: six bars of G, then eight of D7, the V7. The second half turns to C \
            through G7, climbs through C sharp diminished, then takes a chain of dominants, \
            E7, A7, D7, each a fifth above the next, that walks back home to G.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Irving_Caesar/Swanee/Swanee.mxl
// (OpenEWLD lead sheet: the chorus; the verse is not in it. Checked against the same
// lead sheet as ABC, https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/HEAD/wikifonia-dataset/original-abc/GeorgeGershwinIrvingCaesarSwanee.abc)
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
    band: SWING,
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
    form: &["Chorus", "Chorus end", "Chorus", "Chorus end"],
    drums_from: 0,
    about: "Gershwin's first big hit. The tune sits on A, the third of F, for two whole \
            bars, then climbs by half steps, C, C sharp, D, while the chords go from F \
            through A major, the III chord, to B flat: the C sharp is the A chord's third, \
            pulling up to the D of B flat.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Al_Jolson-B.G._DeSylva-Vincent_Rose/Avalon/Avalon.mxl
// (OpenEWLD lead sheet: the chorus; the verse is not in it. Chords checked against the
// iReal chart of the standard: the lead sheet leaves out the G minor of bar 21 under the
// held G, though it names D7 again in the middle of the next bar, as it names C7 again
// at the same spot of the first half; G minor there is the standard's chord too.)
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
    band: SWING,
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
        // The second half, the first time: the pickup into the chorus again.
        Part::new(
            "Chorus 2",
            "A4:8 Bb4 | C5 D5 | C5:4 Bb4 A4:8 | _ D4 | \
             G4:16 | _:8 A4 | Bb4:16 | _:4 - D4:8 | \
             C4 F4 | A4 C5 | E5:4 D5 E5:8 | _:4 - D5:8 | \
             Bb4:16 | A4 | F4 | _:12 C4:4",
            "VI7 | VI7 | VI7 | VI7 | ii | ii VI7 | ii | ii | I | I | VI7 | VI7 | ii | V7 | I | I",
        ),
        Part::new(
            "Chorus end",
            "A4:8 Bb4 | C5 D5 | C5:4 Bb4 A4:8 | _ D4 | \
             G4:16 | _:8 A4 | Bb4:16 | _:4 - D4:8 | \
             C4 F4 | A4 C5 | E5:4 D5 E5:8 | _:4 - D5:8 | \
             Bb4:16 | A4 | F4 | _:12 -:4",
            "VI7 | VI7 | VI7 | VI7 | ii | ii VI7 | ii | ii | I | I | VI7 | VI7 | ii | V7 | I | I",
        ),
    ],
    form: &["Pickup", "Chorus", "Chorus 2", "Chorus", "Chorus end"],
    drums_from: 1,
    about: "The song starts away from home: four bars of C7, the V7, before the first F \
            chord arrives. The second half does the same trick a step higher, four bars of \
            D7 that lead to G minor, the ii, and the end is a plain ii-V-I.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Chris_Smith_%5B1%5D-Jim_Burris/Ballin%27_the_Jack/Ballin%27_the_Jack.mxl
// (OpenEWLD lead sheet: the chorus; the verse is not in it. Chords checked against the
// iReal chart, https://raw.githubusercontent.com/Dazzid/chromaflow/HEAD/data/iRealXML/Ballin'%20The%20Jack.xml)
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
    band: SWING,
    style: "Turnaround",
    parts: &[Part::new(
        "Chorus",
        "D5:2 D5 B4 B4 G4:4 A4 | D5:2 B4:4 G4:6 A4:2 Ab4 | G4 G4 A4 A4 Bb4:4 A4:2 G4 | \
         Bb4 Bb4 A4 A4 G4:4 - | F4:2 F4 G4 G4 A4:4 G4:2 F4 | C5 A4:4 F4:6 D5:2 C#5 | \
         D5 C#5 D5 Bb4 C5 B4 C5 A4 | Bb4 A4 G4 A4:6 -:4 | D5:2 D5 B4 B4 G4:4 A4 | \
         D5:2 B4:4 G4:6 A4:2 Ab4 | G4 G4 A4 A4 Bb4:4 A4:2 G4 | Bb4 A4:4 G4:6 Bb4:2 C5 | \
         Db5:4 Bb4 Db5 Bb4 | D5:2 C5:4 Bb4:6 G4:4 | C5 D5 Eb5 C5 | D5:2 C#5 D5 Bb4:6 -:4",
        "VI7 | VI7 | II7 | II7 | V7 | V7 | I III7 | IV7 III7 | VI7 | VI7 | II7 | II7 | bVI7 | \
         I #v°7 vi VI7 | ii VI ii iiø7 | V7 I",
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
// and https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/EudayL.abc
// (the Wikifonia lead sheet of the published rag, in E flat: the two
// variations of the strain and the second introduction, here down a minor
// third, in C. Checked against Colin MacDonald's sequence of the printed score,
// https://raw.githubusercontent.com/HeavenlyBerserker/Midireader/HEAD/midiReader/input/InputV1/notefiles/RAG/RAG/MathewCollection/MacDonald/12th%20Street%20Rag%20-%20MacDonald.notelist,
// which has the same notes but for two, where the sequence and the lead
// sheet's own first strain agree and are followed: the rhythm of the last bar
// of B1 and the last note of the eleventh bar of C2. The 1914 edition's long
// first introduction is in no written source and is left out.)
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
    band: PIANO,
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
        // The first variation: the three notes C, B, A with an E struck
        // between them.
        Part::new(
            "B1",
            "C5:4 E5:2 B4:4 E5:2 A4:2 E5:2 | G4:2 E5:2 C5:2 E5:2 B4:2 E5:2 A4:2 E5:2 | \
             B4:4 F5:2 A4:4 F5:2 G#4:4 | G4:16 | B4:4 F5:2 A4:4 F5:2 G4:2 F5:2 | \
             B4:4 F5:2 A4:4 F5:2 G4:2 F5:2 | C5:4 E5:2 B4:4 E5:2 A4:2 E5:2 | G4:16 | \
             C5:4 E5:2 B4:4 E5:2 A4:2 E5:2 | G4:2 E5:2 C5:2 E5:2 B4:2 E5:2 A4:2 E5:2 | \
             B4:4 F5:2 A4:4 F5:2 G#4:4 | G4:16 | E4:8 D4:8 | B4:8 A4:8 | \
             G4:4 A4:4 B4:6 C5:2 | D5:2 B4:4 A4:4 G4:6",
            "I | I | V7 | V7 | V7 | V7 | I | I | I | I | V7 | V7 | II7 | II7 | V7 | V7",
        ),
        Part::new(
            "B2",
            "C5:4 E5:2 B4:4 E5:2 A4:2 E5:2 | G4:2 E5:2 C5:2 E5:2 B4:2 E5:2 A4:2 E5:2 | \
             B4:4 F5:2 A4:4 F5:2 G#4:4 | G4:16 | B4:4 F5:2 A4:4 F5:2 G4:2 F5:2 | \
             B4:4 F5:2 A4:4 F5:2 G4:2 F5:2 | C5:4 E5:2 B4:4 E5:2 A4:2 E5:2 | G4:16 | \
             E5:12 E5:4 | E5:12 E5:4 | -:2 C5:2 D5:2 Eb5:5 D5:1 C5:4 | E5:16 | \
             E5:4 D5:4 C5:2 A4:6 | E5:8 D5:8 | C5:16 | _:4 -:4 C5:4 -:4",
            "I | I | V7 | V7 | V7 | V7 | I | I | I | III7 | IV iv7 | I | II7 | V7 | I | I",
        ),
        // The second introduction, the opening lick as the printed score
        // has it.
        Part::new(
            "Intro 2",
            "G5:2 F#5:2 G5:2 A5:2 G5:4 -:4 | G4:2 F#4:2 G4:2 A4:2 G4:4 -:4 | \
             G4:3 G4:1 G#4:3 G#4:1 A4:3 A4:1 A#4:3 A#4:1 | B4:4 -:4 G5:4 -:4",
            "V7 | V7 | V7 | V7",
        ),
        // The second variation: G sharp, A and a high G, in dotted rhythm.
        Part::new(
            "C1",
            "G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 G5:4 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | A5:16 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 G5:4 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | A5:16 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 G5:4 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | A5:16 | E5:8 D5:8 | F#5:8 E5:8 | \
             G5:2 G4:4 F4:2 F4:3 F5:1 D4:3 D5:1 | G4:3 G5:1 G#4:3 G#5:1 A4:3 A5:1 B4:3 B5:1",
            "I | I | V7 | V7 | V7 | V7 | I | I | I | I | V7 | V7 | II7 | II7 | V7 | V7",
        ),
        Part::new(
            "C2",
            "G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 G5:4 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | A5:16 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 G5:4 | \
             G#4:3 A4:1 G5:3 G#4:1 A4:3 G5:1 G#4:3 A4:1 | A5:16 | \
             C5:4 G4:2 C5:4 G4:2 C5:4 | C5:4 G4:2 C5:4 G4:2 C5:4 | C5:4 A4:2 C5:4 A4:2 C5:4 | \
             C5:12 -:4 | G5:8 A5:3 G5:1 E5:3 C5:1 | E5:8 D5:8 | C5:4 A4:2 G4:4 E4:2 D4:4 | \
             C4:4 -:4 C5:2 -:6",
            "I | I | V7 | V7 | V7 | V7 | I | I | I | I7 | IV | bVI7 | I vi7 | II7 V7 | I | I",
        ),
    ],
    form: &[
        "Intro", "A1", "A2", "A1", "Tag", "B1", "B2", "Intro 2", "C1", "C2",
    ],
    drums_from: 0,
    about: "The main strain is three notes, C B A, played in bars of eight eighths, so the \
            group of three starts on a different beat each time round: the three-against-four \
            of ragtime. The chords hardly move, C and G7, and the strain's end slides down by \
            half steps, C, B flat, A, A flat, over C, C7, F and F minor. Two variations \
            follow, the second after the introduction comes back: first the three notes with \
            an E struck between them, then G sharp, A and a high G in dotted rhythm, which \
            ends over C, C7, F and A flat 7, the flat-six chord.",
};

// Source: https://tunearch.org/wiki/Annotation:Dill_Pickle_Rag
// (the ABC there, a fiddle version after Phillips, "via PR": the three
// strains, the trio in C, in 4/4 as written there)
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
    band: PIANO,
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
        // The trio, in C major, the IV of G.
        Part::new(
            "Trio",
            "C5:2 A4 G4 C5 A4 G4 E4 F4 | G4 E4 G4 A4:4 G4:2 E4:4 | \
             D4:2 C#4 D4 G4:4 E4:2 D4:4 | C4:8 A4:4 B4 | \
             C5:2 A4 G4 C5 A4 G4 E4 F4 | G4 E4 G4 A4:4 G4:2 E4:4 | \
             A4:2 G#4 A4 D5:4 B4:2 A4:4 | G4:8 A4:4 B4 | \
             C5:2 A4 G4 C5 A4 G4 E4 F4 | G4 E4 G4 A4:4 G4:2 E4:4 | \
             D4:2 C#4 D4 G4:4 E4:2 D4:4 | C4:8 C5:4 B4 | \
             A4 A4:2 B4 C5 B4 C5 D5 | E5 D#5 E5 A5:4 G5:2 E5:4 | \
             D5:2 C#5 D5 G5:4 E5:2 D5:4",
            "I | I | V | I | I | I | II | V | I | I | V | I | vi | vi | II V",
        )
        .in_key(C, Major),
        // The two endings of the trio.
        Part::new("Trio 1", "C5:12 A4:2 B4", "I").in_key(C, Major),
        Part::new("Trio 2", "C5:12 -:4", "I").in_key(C, Major),
    ],
    form: &[
        "Pickup", "A", "A1", "A", "A2", "B", "B1", "B", "B2", "Trio", "Trio 1", "Trio", "Trio 2",
    ],
    drums_from: 1,
    about: "The first strain is a three-note cell, D E G, run through bars of eight \
            eighths, so it lands on a new beat each time: ragtime's three-against-four. The \
            same cell then steps down, onto E minor (vi) and D (V), and the endings turn \
            through E and A major, a chain of dominants that leads back through D to G. The \
            trio moves to C major, the IV of G, as rag trios do, and its last phrase passes \
            through A minor and D major on the way home.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/JoplinS/PineappleRag/PineappleRag.ly
// (Mutopia, from the Seminary Music edition of 1908; the top line of the right
// hand, down a fourth from B flat, so the trio is in B flat instead of E flat. In the
// second strain the broken octaves, a low note and then the chord, are written as
// repeated notes at the top.)
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
    band: PIANO,
    style: "New Orleans",
    parts: &[
        // The introduction is in octaves, without chords: the line alone.
        Part::new(
            "Intro",
            "D4:1 C4:2 Bb3:1 A3 G#3 A3 G3 F3 G3 A3 C4:5 | \
             D4:1 D4:2 C#4:1 D4 E4 F4:2 G4 C4:4 C4:2",
            "",
        ),
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
        // The first strain again, ending on F7 to lead into the trio.
        Part::new("A3", "F4:4 G4:1 A4:2 F4:6 G4:1 F4 Eb4", "II7 V7 I I7"),
        // The trio, in B flat major (E flat in the score), where the piece ends. It lies
        // low, as in the score.
        Part::new(
            "C",
            "D4:1 C#4:2 D4:5 D4:1 C#4:2 D4:5 | \
             Bb3:1 C4:2 Db4 C4:1 Bb3:2 D4:4 Eb4:1 G4 F4 Eb4 | \
             D4 C#4:2 D4:3 Bb4:2 Bb4:1 A4:2 A4:3 F5:2 | \
             F5:1 E5:2 E5 F5:1 E5:2 D5:4 F4:1 G4 F4 Eb4 | \
             D4 C#4:2 D4:5 D4:1 C#4:2 D4:5 | \
             Bb3:1 C4:2 Db4 C4:1 Bb3:2 D4:6 F4:2 | \
             F4:1 Eb4:2 G4 F#4:1 G4:2 G4:1 F4:2 D5 Bb4:1 D4:2",
            "I | IV IV I V7 | I iii | VII7 VII7 iii V7 | I | IV IV I I7 | IV I",
        )
        .in_key(ASharp, Major),
        Part::new(
            "C1",
            "D4:1 C4:2 G4 F4:1 D4:2 Bb3 F3:1 G3 A3 Bb3 C4 C#4",
            "V7 V7 I V7",
        )
        .in_key(ASharp, Major),
        Part::new(
            "C2",
            "D4:1 C4:2 G4 F4:1 D4:2 Bb3:5 A4:1 Bb4 C5",
            "V7 V7 I I",
        )
        .in_key(ASharp, Major),
        Part::new(
            "D",
            "Db5:1 Db5:2 Db5:6 Bb4:1 Db5 Gb4 Bb4 Db5:2 Bb4:1 | \
             D5 D5:2 D5:6 D5:1 F5 Bb4 C5 D5:3 | \
             G5:1 F5:2 Eb5 D5:1 C5:2 B4:1 C5:2 C5 D5:1 Eb5:2 | \
             G5:1 F5:2 D5 C5:1 Bb4:2 C#5:1 D5:2 D5 A4:1 Bb4 C5 | \
             Db5 Db5:2 Db5:6 Bb4:1 Db5 Gb4 Bb4 Db5:2 Bb4:1 | \
             D5 D5:2 D5:6 D5:1 F5 Bb4 C5 D5:3 | \
             D5:1 D5:2 C5:1 C5 Bb4 C5:2 D5:1 D5:2 F5:1 F5 Bb4 D5:2",
            "bVI | I | V7 | I | bVI | I | IV I",
        )
        .in_key(ASharp, Major),
        Part::new(
            "D1",
            "F5:1 Eb5:2 D5 C5:1 D5:2 Bb4:5 A4:1 Bb4 C5",
            "V7 V7 I I",
        )
        .in_key(ASharp, Major),
        Part::new("D2", "F5:1 Eb5:2 D5 C5:1 D5:2 Bb4:4 Bb5:2 -", "V7 V7 I I").in_key(ASharp, Major),
    ],
    form: &[
        "Intro", "A", "A1", "A", "A2", "B", "B1", "B", "B2", "A", "A3", "C", "C1", "C", "C2", "D",
        "D1", "D", "D2",
    ],
    drums_from: 1,
    about: "A slow-march rag. The first strain steps down with a chromatic turn around A, then \
            climbs a broken chord to the top; halfway through it turns B flat major into B flat \
            minor (IV, then iv) for a moment of shade. The second strain opens on the dominant, \
            C7, and hammers each note twice, short then long: that is the ragtime syncopation in \
            its plainest form. After four bars in octaves, the strains come A A B B A C C D D: \
            the trio (C) moves to B flat major, the IV, low and quiet, and the last strain opens \
            on G flat major, the flat six (bVI), before the piece ends in B flat. The piece is \
            in 2/4, two of its bars to a bar here, a fourth below the original key.",
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
    band: PIANO,
    style: "New Orleans",
    parts: &[
        // The introduction is in octaves, without chords: the line alone.
        Part::new(
            "Intro",
            "C6:1 A5 G5 F5:2 A5:1 G5 F5 G5 F5 D5 C5:2 F4:1 G4 A4 | \
             C4 G4:2 F4:1 E4:2 G4 C5 -:1 C4 - D4 - E4",
            "",
        ),
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
        // The first strain again, ending on F7 to lead into the trio.
        Part::new(
            "A3",
            "F4:1 G4 A4:2 G4:1 G4:2 C4:1 F4:2 - F5:4",
            "ii6 V7 I I7",
        ),
        // The trio, in B flat major, where the piece ends.
        Part::new(
            "C",
            "D5:1 G5 F5 Eb5 D5 C5 Bb4 D5 C5 Bb4 G4 F4:2 Bb5 G5:1 | \
             F5 G5 F5 D5 Bb4 C5 D5 C5:7 F4:2 | \
             D5:1 G5 F5 Eb5 D5 Eb5 D5 C5 Bb4 A4 G4 Bb4:2 D5:1 C5 Bb4 | \
             A4 D5 F5 A5 G5 F5:2 E5:1 D5:2 - F5:4 | \
             D5:1 G5 F5 Eb5 D5 C5 Bb4 D5 C5 Bb4 G4 F4:2 Bb5 G5:1 | \
             F5 G5 F5 D5 Bb4 C5 D5 C5:7 F4:2 | \
             D5:1 C#5 D5 Eb5 F5 E5 F5 F#5 G5 F#5 G5 Bb5:2 Bb5:1 A5 G5",
            "I I IV I | I II7 V7 V7 | I III7 vi vi | iii VII7 iii V7 | I I IV I | I II7 V7 V7 | \
             I I IV #iv°7",
        )
        .in_key(ASharp, Major),
        Part::new(
            "C1",
            "F5:1 D6 Bb5 F5 G5 Eb5 C5 A4 Bb4:2 E4:1 F4 G4 A4 Bb4 C5",
            "I V7 I I",
        )
        .in_key(ASharp, Major),
        // The trio's second ending runs into the last strain: its last B flat is tied over the
        // bar line, so the first two bars of D come with it.
        Part::new(
            "C2 into D",
            "F5:1 D6 Bb5 F5 G5 Eb5 C5 A4 Bb4:6 -:1 Bb5 | \
             _ Eb5 C6 Eb5 G5 Bb5:2 Eb5:1 C6 Eb5 G5 Bb5:2 C6 Bb5:1",
            "I V7 I I | IV",
        )
        .in_key(ASharp, Major),
        Part::new(
            "D",
            "F5:1 Bb4 G5 Bb4 D5 F5:2 Bb4:1 G5 Bb4 D5 F5:2 G5 Bb4:1 | \
             F5 A4 G5 A4 C5 F5:2 A4:1 G5 A4 C5 F5:2 G5 A4:1 | \
             F5 Bb4 G5 Bb4 D5 F5:2 Bb4:1 G5 Bb4 D5 F5:2 F5 Bb5:1 | \
             _ Eb5 C6 Eb5 G5 Bb5:2 Eb5:1 C6 Eb5 G5 Bb5:2 C6 Bb5:1",
            "I | V7 | I | IV",
        )
        .in_key(ASharp, Major),
        // The end of D, its first ending, and the tied B flat into the first two bars again.
        Part::new(
            "D1 into D",
            "F5:1 Bb4 G5 Bb4 D5 F5:2 Bb4:1 G5 Bb4 D5 F5:2 F5 Db5:1 | \
             _ Bb4 Db5 Bb4 C5 Db5:2 Bb4:1 D5 Bb4 C5 D5:2 D5 F5:1 | \
             _ C5 G5 C5 Eb5 A5:2 Bb5:5 -:3 Bb5:1 | \
             _ Eb5 C6 Eb5 G5 Bb5:2 Eb5:1 C6 Eb5 G5 Bb5:2 C6 Bb5:1",
            "I | bVI7 I | V7 V7 I I | IV",
        )
        .in_key(ASharp, Major),
        Part::new(
            "D2",
            "F5:1 Bb4 G5 Bb4 D5 F5:2 Bb4:1 G5 Bb4 D5 F5:2 F5 Db5:1 | \
             _ Bb4 Db5 Bb4 C5 Db5:2 Bb4:1 D5 Bb4 C5 D5:2 D5 F5:1 | \
             _ C5 G5 C5 Eb5 A5:2 Bb5:5 Bb5:2 -",
            "I | bVI7 I | V7 V7 I I",
        )
        .in_key(ASharp, Major),
    ],
    form: &[
        "Intro",
        "A",
        "A1",
        "A",
        "A2",
        "B",
        "B1",
        "B",
        "B2",
        "A",
        "A3",
        "C",
        "C1",
        "C",
        "C2 into D",
        "D",
        "D1 into D",
        "D",
        "D2",
    ],
    drums_from: 1,
    about: "The first strain is a two-note figure, D and C, where the C comes a sixteenth early \
            and is held across the beat, while the bass creeps up by half steps from F to A and \
            then slides back down under a diminished chord. The second strain is a single line \
            running down the scale, and in its middle it leans toward D minor and A minor \
            through A7 and E7. After four bars in octaves, the strains come A A B B A C C D D: \
            the trio (C) moves to B flat major, the IV, and the piece ends there, the last \
            strain reaching its cadence through G flat 7 (bVI7). The piece is in 2/4, two of its \
            bars to a bar here.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Irving_Caesar-Vincent_Youmans/Tea_for_Two/Tea_for_Two.mxl
// (OpenEWLD lead sheet, "Easy version", the refrain in F; here up a minor third in A flat,
// the standard key. Melody checked against the Wikifonia lead sheet with the verse,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/master/wikifonia-dataset/original-abc/VincentYoumansIrvingCaesarTeaForTwo.abc,
// which also gives the first ending with its fill, and against Karl Dallas's ABC,
// https://raw.githubusercontent.com/avnerdorman/abc-solfege-tool/main/songs/Tea_for_two.abc.
// Chords: the standard changes (iReal, standards.py "Tea For Two") where they fit the
// melody; in the last section, bar 3 from the Wikifonia sheet and bars 5-6 from the
// OpenEWLD sheet, where the standard Cm7 B°7 clashes with the tune.)
pub(super) const TEA_FOR_TWO: Song = Song {
    name: "Tea for Two",
    origin: "Vincent Youmans, No, No, Nanette, 1924",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: GSharp,
    mode: Major,
    time: COMMON,
    tempo: 160.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "Ab4:6 F4:2 G4:6 F4:2 | Ab4:6 F4:2 G4:6 Eb4:2 | G4:6 Eb4:2 F4:6 Eb4:2 | \
             G4:6 Eb4:2 F4:6 Eb4:2 | Ab4:6 F4:2 G4:6 F4:2 | Ab4:6 F4:2 G4:6 Eb4:2 | \
             C5:16 | _:4 -:12",
            "ii7 V7 | ii7 V7 | Imaj7 IV7 | iii7 VI7 | ii7 V7 | ii7 V7 | Imaj7 | Imaj7",
        ),
        // The same figure a major third higher, in C; E flat 7 leads back to A flat.
        Part::new(
            "B",
            "C5:4 C5:2 A4 B4:4 B4:2 A4 | C5:4 C5:2 A4 B4:4 B4:2 G4 | \
             B4:4 B4:2 G4 A4:4 A4:2 G4 | B4:4 B4:2 G4 A4:4 A4:2 G4 | \
             C5:4 C5:2 A4 B4:4 B4:2 A4 | C5:4 C5:2 A4 B4:4 B4 | E5:16 | Eb5:8 -:8",
            "ii7 V7 | ii7 V7 | Imaj7 IV7 | iii7 VI7 | ii7 V7 | ii7 V7 | Imaj7 | bIII7",
        )
        .in_key(C, Major),
        Part::new(
            "A2",
            "Ab4:6 F4:2 G4:6 F4:2 | Ab4:6 F4:2 G4:6 Eb4:2 | G4:6 Eb4:2 F4:6 Eb4:2 | \
             G4:6 Eb4:2 F4:6 Eb4:2 | Ab4:6 F4:2 G4:6 F4:2 | Ab4:6 F4:2 G4:6 Eb4:2 | \
             Eb5:16 | _:8 -:8",
            "ii7 V7 | ii7 V7 | Imaj7 IV7 | iii7 VI7 | ii7 V7 | ii7 V7 | iiiø7 | VI7b9",
        ),
        // The last section, with the first ending: the fill and the chromatic
        // E flat 7, E7, F7 back to the top.
        Part::new(
            "C",
            "F5:6 F5:2 Eb5:6 Eb5:2 | Db5:6 Db5:2 C5:6 C5:2 | Eb5:6 Eb5:2 Db5:6 Db5:2 | \
             C5:6 C5:2 Bb4:6 Bb4:2 | Ab4:6 F4:2 G4:6 F4:2 | Ab4:6 F4:2 G4:6 C5:2 | \
             Ab4:4 G5 F5 C5 | Bb4 Eb4 E4 F4",
            "ii7 iii°7 | IVmaj7 iii°7 | #i°7 ii7 | bVII7 | ii7 V7 | ii7 V7 | I vi6 | V7 V7 #V7 VI7",
        ),
        Part::new(
            "C end",
            "F5:6 F5:2 Eb5:6 Eb5:2 | Db5:6 Db5:2 C5:6 C5:2 | Eb5:6 Eb5:2 Db5:6 Db5:2 | \
             C5:6 C5:2 Bb4:6 Bb4:2 | Ab4:6 F4:2 G4:6 F4:2 | Ab4:6 F4:2 G4:6 C5:2 | \
             Ab4:16 | _:12 -:4",
            "ii7 iii°7 | IVmaj7 iii°7 | #i°7 ii7 | bVII7 | ii7 V7 | ii7 V7 | I6 | I6",
        ),
    ],
    form: &["A", "B", "A2", "C", "A", "B", "A2", "C end"],
    drums_from: 0,
    about: "The whole chorus is one little figure, long-short, long-short, turned over a \
            ii-V, B flat minor 7 to E flat 7, twice before it lands on A flat. The second \
            phrase jumps up a major third and plays the same game in C major, over D minor 7 \
            and G7, until one E flat 7 brings it home. The last section walks down the scale \
            in pairs of repeated notes over B flat minor 7, C diminished and D flat, then G \
            flat 7, the flat-seven chord, before a last ii-V.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/Oh,_Lady_Be_Good!/Oh,_Lady_Be_Good!.mxl
// (OpenEWLD lead sheet, the refrain in G, with both endings. Melody checked against the
// lead sheet in F at https://raw.githubusercontent.com/avnerdorman/abc-solfege-tool/main/songs/Oh_Lady_be_good.abc,
// which agrees note for note. Chords from the OpenEWLD sheet; the bridge and the first
// ending from the standard changes (standards.py "Oh, Lady Be Good"), which the F lead
// sheet shares, where the OpenEWLD chords are unusual or clash with the held notes.
// The quarter-note triplets are rounded to 3 + 2 + 3 sixteenths.)
pub(super) const LADY_BE_GOOD: Song = Song {
    name: "Oh, Lady Be Good!",
    origin: "George Gershwin, Lady, Be Good!, 1924",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 200.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "D5:8 C5:4 B4 | D5:8 D5 | D5:3 B4:2 G4:3 D4:8 | _:8 D5:8 | \
             D5:3 C5:2 A4:3 D4:8 | _:8 B4:8 | G4:16 | _:12 -:4",
            "I I V7 I | IV7 | I | I #i°7 | V7 | V7 | I | I",
        ),
        Part::new(
            "A2",
            "D5:8 C5:4 B4 | D5:8 D5 | D5:3 B4:2 G4:3 D4:8 | _:8 D5:8 | \
             D5:3 C5:2 A4:3 D4:8 | _:8 B4:8 | G4:16 | _:8 -:8",
            "I I V7 I | IV7 | I | I #i°7 | V7 | V7 | I | I I7",
        ),
        Part::new(
            "B",
            "E5:16 | -:4 E5 E5 E5 | E5:8 D5 | _:8 -:8 | -:4 B4 B4 B4 | B4:4 B4 B4 B4 | \
             B4:8 A4 | -:4 B4 C5 C#5",
            "IVmaj7 | #iv°7 | I6 | I6 III7 | vi7 | II7 | ii7 | V7",
        ),
        // The last A with the first ending, a turnaround back to the top.
        Part::new(
            "A3",
            "D5:8 C5:4 B4 | D5:8 D5 | D5:3 B4:2 G4:3 D4:8 | _:8 D5:8 | \
             D5:3 C5:2 A4:3 D4:8 | _:8 B4:8 | G4:16 | _:4 -:12",
            "I I V7 I | IV7 | I | I #i°7 | V7 | V7 | I6 vi7 | ii7 V7",
        ),
        // The last A with the second ending.
        Part::new(
            "A3 end",
            "D5:8 C5:4 B4 | D5:8 D5 | D5:3 B4:2 G4:3 D4:8 | _:8 D5:8 | \
             D5:3 C5:2 A4:3 D4:8 | _:8 B4:8 | G4:16 | _:8 -:8",
            "I I V7 I | IV7 | I | I #i°7 | V7 | V7 | I IV7 | I",
        ),
    ],
    form: &["A", "A2", "B", "A3", "A", "A2", "B", "A3 end"],
    drums_from: 0,
    about: "The tune keeps tumbling down the G major chord in quarter-note triplets, D B G, \
            onto a low D held across the bar line, and the second bar is already C7, the \
            IV chord with a flat seventh, a touch of the blues. The bridge sits on C, climbs \
            through a C sharp diminished seventh back to G, then walks home by fifths: B7, \
            E minor 7, A7, A minor 7, D7.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/master/wikifonia-dataset/original-abc/ThomasWallerAndyRazafHoneysuckleRose.abc
// (the Wikifonia lead sheet of the refrain in F, 32 bars with its ending, as ABC. The
// OpenEWLD file of the same song,
// https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Andy_Razaf-Fats_Waller/Honeysuckle_Rose/Honeysuckle_Rose.mxl,
// is cut short (its last A has seven bars); it gives the D in the last bar of the bridge,
// E flat D C as in the bar four before, where the ABC has E flat C C. Chords from the ABC,
// checked against standards.py "Honeysuckle Rose": the second ending (F6, F6) and the
// second bar of the bridge (F7, not Cm7 F7) from the standard changes.)
pub(super) const HONEYSUCKLE_ROSE: Song = Song {
    name: "Honeysuckle Rose",
    origin: "Fats Waller, 1929",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 192.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:4 A4 | \
             A4:8 A4:2 G4 F4 D4 | F4:4 F4 F4:8 | _:8 A4:2 G4 F4 D4 | F4:16 | _:4 -:12",
            "ii7 V7 | ii7 V7 | ii7 V9 | ii7 V9 | I6 I7 | IV6 V7 | I7 IV7 | iii7 VI7",
        ),
        Part::new(
            "A2",
            "C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:4 A4 | \
             A4:8 A4:2 G4 F4 D4 | F4:4 F4 F4:8 | _:8 A4:2 G4 F4 D4 | F4:16 | _:4 -:12",
            "ii7 V7 | ii7 V7 | ii7 V9 | ii7 V9 | I6 I7 | IV6 V7 | I6 | I6",
        ),
        Part::new(
            "B",
            "F4:8 G4 | Ab4:8 A4 | -:4 Bb4:2 C5:4 Bb4:2 C5:4 | Db5:4 C5:2 Bb4:10 | \
             G4:8 A4 | Bb4:8 B4 | -:4 C5:2 D5:4 C5:2 D5:4 | Eb5:4 D5:2 C5:10",
            "I7 | I7 | IV6 | IV6 | II7 | II7 | V7 | V7",
        ),
        // The last A the first time round, with the turnaround back to the top.
        Part::new(
            "A3",
            "C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:4 A4 | \
             A4:8 A4:2 G4 F4 D4 | F4:4 F4 F4:8 | _:8 A4:2 G4 F4 D4 | F4:16 | _:4 -:12",
            "ii7 V9 | ii7 V9 | ii7 V9 | ii7 V7 | I6 I | IV6 V7 | I7 IV7 | iii7 VI7",
        ),
        // The last A as written, the final note held under a fermata.
        Part::new(
            "A end",
            "C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:8 | C5:2 Bb4 D4 F4 A4:4 A4 | \
             A4:8 A4:2 G4 F4 D4 | F4:4 F4 F4:8 | _:8 A4:2 G4 F4 D4 | F4:16 | _:16",
            "ii7 V9 | ii7 V9 | ii7 V9 | ii7 V7 | I6 I | IV6 V7 | I6 | I6",
        )
        .with_tempo(&[fermata(112, 16)]),
    ],
    form: &["A", "A2", "B", "A3", "A", "A2", "B", "A end"],
    drums_from: 0,
    about: "The A section is one riff, C B flat D F and a long A, over G minor 7 and C7, a \
            ii-V that goes round four times before F arrives in the fifth bar. The bridge \
            takes two bars per chord, F7 to B flat, then G7 to C7, and every other bar bends \
            a blue note: A flat over F7, D flat over B flat, B flat over G7, E flat over C7.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Clifford_Grey-Irving_Caesar-Vincent_Youmans/Sometimes_I%27m_Happy_(Sometimes_I%27m_Blue)/Sometimes_I%27m_Happy_(Sometimes_I%27m_Blue).mxl
// (OpenEWLD lead sheet, the refrain in F, in cut time. In the last B the held note "do"
// is written D natural without the tie; it is the D flat tied over, as in the first B.
// Chords from the sheet, Gm7/C written as V7sus4; checked against standards.py
// "Sometimes I'm Happy", whose changes are used in bar 6 of the first B (Am7 D7 for the
// sheet's Eb11 D9), its last bar (C7 for D flat and C9 over the held G) and the first
// ending's turnaround. The first four bars of both Bs are one part, with the first B's
// chords: Cm7 B7 (the second B has C7 against the melody's E flat) and Bbm7 Eb9 (the
// second B has Bbm7 alone; the standard has Eb7 both times).)
pub(super) const SOMETIMES_IM_HAPPY: Song = Song {
    name: "Sometimes I'm Happy",
    origin: "Vincent Youmans, Hit the Deck!, 1927",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 132.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "Backdoor ii–V",
    parts: &[
        Part::new(
            "A",
            "-:4 G#4 A4 A4 | A4:4 Bb4:12 | -:4 G#4 A4 A4 | A4:4 G4:12 | \
             -:4 G#4 A4 A4 | A4:4 Bb4:12 | -:4 G#4 A4 A4 | A4:4 G4:12",
            "Imaj7 | V7sus4 | Imaj7 | V7 | Imaj7 | V7sus4 | Imaj7 | V7",
        ),
        // The first four bars of the B section, the same both times.
        Part::new(
            "B",
            "-:4 G#4 A4 C5 | Eb5:8 Eb5 | D5:4 E4 F4 Db5 | _:16",
            "I Imaj7 | v7 #IV7 | IVmaj7 | iv7 bVII9",
        ),
        // How the first B ends, back to the A.
        Part::new(
            "B1",
            "-:4 E4 F4 A4 | C5:8 C5 | Bb4:4 F4 A4 G4 | _:16",
            "I | iii7 VI7 | ii7 | V7",
        ),
        // How the last B ends the first time round (first ending), back to the top.
        Part::new(
            "B2",
            "-:4 G4 A4 C5 | Bb4:4 G4 A4:8 | F4:16 | _:8 -:8",
            "I Imaj7 | ii7 V7b9 | I6 | ii7 V7",
        ),
        Part::new(
            "B2 end",
            "-:4 G4 A4 C5 | Bb4:4 G4 A4:8 | F4:16 | _:8 -:8",
            "I Imaj7 | ii7 V7b9 | I6 | I6",
        ),
    ],
    form: &[
        "A", "B", "B1", "A", "B", "B2", "A", "B", "B1", "A", "B", "B2 end",
    ],
    drums_from: 0,
    about: "Each phrase of the A section leans into A from the G sharp a semitone below, \
            over F major 7 and a suspended C7 (G minor 7 over a C in the bass). The B section \
            turns to B flat through C minor 7 and B7, the tritone substitute for F7, and \
            comes home by the back door: B flat minor 7 and E flat 9 resolve straight to F.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Whiting-Walter_Donaldson/My_Blue_Heaven/My_Blue_Heaven.mxl
// (OpenEWLD lead sheet, the refrain in E flat with its pickup. The pickup notes in the
// last bar are bracketed, for the repeat only, and are left out of the final ending.
// Chords from the sheet, checked against the iReal chart,
// https://raw.githubusercontent.com/trekar99/musicxml-tokenizer/main/iRealXML/My%20Blue%20Heaven.xml,
// and https://raw.githubusercontent.com/carey-bunks/Jazz-Chord-Progressions-Corpus/main/SongDB/Songs%5BH-O%5D/MyBlueHeaven.txt:
// Eb6 for the first three bars of each A, under the melody's C, and Fm7 for the sheet's
// Fm in the bridge, from those charts.)
pub(super) const MY_BLUE_HEAVEN: Song = Song {
    name: "My Blue Heaven",
    origin: "Walter Donaldson, 1927",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 144.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Turnaround",
    parts: &[
        Part::new("Pickup", "-:4 G4:4 Bb4:2 C5 Eb5:4", "V7"),
        Part::new(
            "A",
            "C5:16 | _:4 Eb4 G4:2 Bb4 C5:4 | Bb4:16 | _:4 Bb3 C4:2 Eb4 G4:4 | \
             F4:16 | G4 | F4:4 Eb4:12 | -:4 G4 Bb4:2 C5 Eb5:4",
            "I6 | I6 | I6 | VI7 | II7 | V7 | I | ii7 V7",
        ),
        Part::new(
            "A2",
            "C5:16 | _:4 Eb4 G4:2 Bb4 C5:4 | Bb4:16 | _:4 Bb3 C4:2 Eb4 G4:4 | \
             F4:16 | G4 | F4:4 Eb4:12 | -:4 Eb5 Eb5 Eb5",
            "I6 | I6 | I6 | VI7 | II7 | V7 | I | v7 I7",
        ),
        Part::new(
            "B",
            "Eb5:4 D5 D5 C5 | C5 Bb4 Bb4 C5 | Bb4 Ab4 C5:8 | _:12 D5:4 | \
             Bb4 Bb4 C5 Bb4 | Bb4 Ab4 Ab4 Bb4 | Ab4 G4 Bb4:8 | _:4 G4 Bb4:2 C5 Eb5:4",
            "IV | VI7 | ii7 | ii7 | V7 | V7 | I #i°7 | ii7 V7",
        ),
        // The last A, with the pickup back to the top.
        Part::new(
            "A3",
            "C5:16 | _:4 Eb4 G4:2 Bb4 C5:4 | Bb4:16 | _:4 Bb3 C4:2 Eb4 G4:4 | \
             F4:16 | G4 | F4:4 Eb4:12 | -:4 G4 Bb4:2 C5 Eb5:4",
            "I6 | I6 | I6 | VI7 | II7 | V7 | I IV | I V7",
        ),
        Part::new(
            "A3 end",
            "C5:16 | _:4 Eb4 G4:2 Bb4 C5:4 | Bb4:16 | _:4 Bb3 C4:2 Eb4 G4:4 | \
             F4:16 | G4 | F4:4 Eb4:12 | -:16",
            "I6 | I6 | I6 | VI7 | II7 | V7 | I IV | I",
        ),
    ],
    form: &["Pickup", "A", "A2", "B", "A3", "A", "A2", "B", "A3 end"],
    drums_from: 1,
    about: "Each line of the tune climbs an E flat sixth chord, G B flat C E flat, and \
            settles on C, the sixth. Then a chain of dominants, C7, F7, B flat 7, each the V \
            of the next, walks home to E flat. The bridge moves to A flat, the IV, and rests \
            on F minor 7 before B flat 7 and a passing E diminished chord bring the tune back.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Douglas_Furber-Philip_Braham/Limehouse_Blues/Limehouse_Blues.mxl
// (OpenEWLD lead sheet: the refrain, in A flat, 32 bars. Notes checked against the
// Wikifonia ABC, https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/PhilipBrahamDouglasFurberLimehouseBlues.abc,
// and the fiddle ABC at https://tunearch.org/wiki/Annotation:Limehouse_Blues (in C);
// chords checked against the iReal Pro changes. The lead sheet's chords on the fourth
// beat, pushed with the tied melody note, are kept there. No written verse found.)
pub(super) const LIMEHOUSE_BLUES: Song = Song {
    name: "Limehouse Blues",
    origin: "Philip Braham, 1922",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: GSharp,
    mode: Major,
    time: COMMON,
    tempo: 216.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "F4:4 G4 Ab4 F4 | _:4 G4:8 Ab4:4 | F4:4 G4 Ab4 F4 | _:16 | \
             F4:4 G4 Ab4 F4 | _:4 G4:8 Ab4:4 | F4:4 G4 Ab4 F4 | _:16",
            "IV7 | IV7 | IV7 | IV7 | II7 | II7 | II7 | II7",
        ),
        Part::new(
            "B",
            "Eb4:4 F4 G4 Ab4 | _:4 Bb4:8 B4:4 | C5:4 B4 Bb4 Ab4 | _:16 | \
             F4:4 G4 Ab4 A4 | _:4 Bb4:8 C5:4 | Bb4:4 A4 Ab4 G4 | _:16",
            "I | I | III7 III7 III7 vi | vi | II7 | II7 | V7 | V7",
        ),
        Part::new(
            "A2",
            "F4:4 G4 Ab4 F4 | _:4 G4:8 Ab4:4 | F4:4 G4 Ab4 F4 | _:16 | \
             Ab4:4 Bb4 C5 Ab4 | _:4 Bb4:8 C5:4 | Ab4:4 Bb4 C5 Ab4 | _:16",
            "IV7 | IV7 | IV7 | IV7 | II7 | II7 | II7 | II7",
        ),
        Part::new(
            "C",
            "Eb5:4 D5 Eb5 F5 | _:4 Eb5:8 C5:4 | Db5:4 C5 Db5 Bb4 | _:16 | \
             Db5:4 C5 Db5 Eb5 | _:4 C5:8 Bb4:4 | Ab4:4 Ab4 Ab4 Ab4 | _:16",
            "I I I VI7 | VI7 | ii ii ii iv6 | iv6 | iv6 iv6 iv6 V7 | V7 | I I iv I | I",
        ),
    ],
    form: &["A", "B", "A2", "C", "A", "B", "A2", "C"],
    drums_from: 0,
    about: "The tune starts away from home: four bars of D flat 7 (IV7) and four of B flat \
            7 (II7) go by before A flat arrives in bar 9, and the same three notes, F, G and \
            A flat, sound over both. Later B flat 7 moves on to E flat 7, the V7, down the \
            cycle of fifths, and the last eight bars bring in D flat minor, the minor iv, \
            ending A flat, D flat minor, A flat: a minor plagal cadence.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Raymond_B._Egan-Richard_A._Whiting/Japanese_Sandman/Japanese_Sandman.mxl
// (OpenEWLD lead sheet: the refrain in F, cut time, with its pickup, first ending and the
// four-bar second ending; chords checked against the iReal Pro forum chart "Japanese
// Sandman 3". The C chord of refrain bars 5-6 is played as C7, as there; "A dim" as A
// diminished seven, which holds the F sharp of the tune. No written verse found.)
pub(super) const JAPANESE_SANDMAN: Song = Song {
    name: "Japanese Sandman",
    origin: "Richard A. Whiting, 1920",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 184.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "Turnaround",
    parts: &[
        Part::new("Pickup", "-:4 C5:2 A4 C5 A4 C5:4", ""),
        Part::new(
            "A",
            "A4:8 A4 | _:4 C5:2 A4 C5 A4 C5:4 | A4:4 E4:2 D4 E4:4 E4:2 D4 | \
             E4:4 C5:2 A4 C5 A4 C5:4 | D5:8 C5 | _:2 A4 C5 A4 C5 A4 G4:4 | \
             F4:4 A5:2 F5 D5 C5 A4 F4 | C4:4 C5:2 A4 C5 A4 C5:4",
            "I | I | VI7 | VI7 | II7 V7 | V7 | I | I",
        ),
        Part::new(
            "B",
            "A4:8 A4 | _:4 C5:2 A4 C5 A4 C5:4 | C#5:4 E5:2 C#5 E5:4 E5:2 C#5 | \
             E5:4 E5:2 C#5 E5 C#5 E5:4 | C#5:8 B4 | _:4 B4:2 G#4 B4 G#4 B4:4 | \
             A4:4 A4:2 F#4 A4:4 A4:2 F#4 | A4:2 - C5 A4 C5 A4 C5:4",
            "I | I | III | III | VII7 | VII7 | III iii°7 | ii V7 V7 V7",
        ),
        Part::new(
            "C",
            "A4:8 A4 | _:4 C5:2 D5 C5 A4 G4:4 | F4:4 D4:2 C#4 D4:4 D4:2 C#4 | \
             D4:4 A4:2 F4 A4 F4 A4:4 | F4:8 F4 | F4:4 A4:2 F4 A4 F4 A4:4 | \
             G4:4 D4:2 E4 D4:4 D4:2 E4 | C4:4 C5:2 A4 C5 A4 C5:4",
            "I | I7 | IV | IV | iv | iv | ii | V7",
        ),
        Part::new(
            "D",
            "A4:8 A4 | _:4 C5:2 A4 C5 A4 C5:4 | F5:4 F5:2 D5 F5:4 F5:2 D5 | \
             F5:2 - D5 A4 D5 A4 D5:4 | A4:8 A4 | _:2 - C5 A4 C5 A4 G4:4 | \
             F4:4 C5:2 A4 C5:4 C5:2 A4 | F4:4 C5:2 A4 C5 A4 C5:4",
            "I | I | vi IV | II7 | I II7 | V7 | I | I",
        ),
        // The second ending: a tag in F minor before the last F.
        Part::new(
            "D end",
            "A4:8 A4 | _:4 C5:2 A4 C5 A4 C5:4 | F5:4 F5:2 D5 F5:4 F5:2 D5 | \
             F5:2 - D5 A4 D5 A4 D5:4 | A4:8 A4 | _:2 - C5 A4 C5 A4 G4:4 | \
             F4:2 G4 Ab4 Bb4 C5 Bb4 Ab4 G4 | F4:2 E4 F4 Eb4 _:4 C#4 | C4:16 | _:4 -:12",
            "I | I | vi IV | II7 | I II7 | V7 | i iv | i i bVI7 #IV7 | I | I",
        ),
    ],
    form: &["Pickup", "A", "B", "C", "D", "A", "B", "C", "D end"],
    drums_from: 1,
    about: "Every phrase opens with the same figure, C A C A C, landing on a long A, the \
            third of F. The second phrase lifts the tune to A major, the III chord, and \
            confirms it with its own dominant, E7; the third turns B flat (IV) into B flat \
            minor (iv). The last time through, a tag runs up and down the F minor scale over \
            F minor and B flat minor, then slips through D flat 7 and B7 to the final F.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Gus_Kahn-Ira_Gershwin/Liza_(All_the_Clouds%27ll_Roll_Away)/Liza_(All_the_Clouds%27ll_Roll_Away).mxl
// (OpenEWLD lead sheet: the verse, 18 bars, and the refrain, AABA, with its repeat, in E
// flat. Chords checked against the iReal Pro changes and the second Wikifonia lead sheet,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/GeorgeGershwinIraGershwinandGusKahnLizaAllthecloudsllrollaway.abc.
// The diminished chords of the refrain are named on the bass notes those two give
// (F sharp, A). Where the first sheet's chords clash with a held note, the others
// decide: the bridge's first bar is C minor (not C, C7, under E flat), its fourth B flat
// minor 7 and E flat 7 (not C diminished under B flat); B7 in verse bar 8 is B flat 7
// (the tune holds B flat) and Bm7 in the turnaround is B7 (the tune holds E flat).)
pub(super) const LIZA: Song = Song {
    name: "Liza (All the Clouds'll Roll Away)",
    origin: "George Gershwin, Show Girl, 1929",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 184.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Major-minor cycle",
    parts: &[
        Part::new(
            "Verse",
            "Eb5:8 D5:3 D5:1 D5:3 D5:1 | C5:8 D5 | C5:4 D5 C5 D5 | G4:8 Bb4 | \
             Eb5:8 D5:3 D5:1 D5:3 D5:1 | C5:8 D5 | C5:4 D5 C5 D5 | Eb5:8 Bb4",
            "I iii | vi iii | vi iii vi iii | vi6 V7 | I iii | vi iii | vi iii vi iii | II7 V7",
        ),
        Part::new(
            "Verse 2",
            "F4:3 G4:1 Ab4:3 Bb4:1 C5:3 D5:1 Eb5:3 F5:1 | Eb5:3 D5:1 C5:3 Bb4:1 D5:8 | \
             Bb4:3 Ab4:1 G4:3 F4:1 Ab4:6 G4:2 | Bb4:4 Bb4:12 | \
             F4:3 G4:1 Ab4:3 Bb4:1 C5:3 D5:1 Eb5:3 F5:1 | Eb5:3 D5:1 C5:3 Bb4:1 D5:8 | \
             F5:2 Bb4:4 Bb4:2 C5:8 | F5:2 Bb4:4 Bb4:2 C5:8 | F5:4 Bb4 D5 G4 | Bb4:16",
            "IV IV IV iv6 | I i°7 | ii7 ii7 ii7 ii7 iv6 iv6 iv6 V7 | iii7 VI7 VI7 VI7 | \
             IV6 IV6 IV6 iv6 | I II7b9 | V V I i | V V VI7 i6 | V iii7 vi7 II9 | V ii7 iii V7",
        ),
        Part::new(
            "A",
            "Bb4:8 Bb4 | C5:8 C5 | Eb5:8 Eb5 | F5:16 | -:4 Eb5 C5 Ab4 | D5:4 Bb4 G4:8 | \
             -:4 C5:3 Ab4:1 F4:3 Eb4:1 D4:3 F4:1 | Eb4:16",
            "I V7 | #ii°7 VI7 | iv6 #iv°7 | v6 I9 | IV ii7 | iii VI9 | ii V7 | I IV I ii7",
        ),
        Part::new(
            "A2",
            "Bb4:8 Bb4 | C5:8 C5 | Eb5:8 Eb5 | F5:16 | -:4 Eb5 C5 Ab4 | D5:4 Bb4 G4:8 | \
             -:4 C5:3 Ab4:1 F4:3 Eb4:1 D4:3 F4:1 | Eb4:12 -:4",
            "I V7 | #ii°7 VI7 | IV #iv°7 | v6 I9 | IV ii7 | iii VI9 | ii V7 | I I III+ III7",
        ),
        Part::new(
            "B",
            "Eb5:16 | -:3 F5:1 Eb5:3 D5:1 Eb5:4 D5 | C5:12 G4:4 | Bb4:16 | C5:16 | \
             -:3 Db5:1 C5:3 Bb4:1 C5:4 Bb4 | Ab4:12 Eb4:4 | Gb4:12 G4:4",
            "vi | vi°7 III7 | vi | v7 I7 | IV ii | v7 I7 | IV6 | i6 i6 i6 V7",
        ),
        // The first ending: a turnaround back to the top of the refrain.
        Part::new(
            "A3",
            "Bb4:8 Bb4 | C5:8 C5 | Eb5:8 Eb5 | F5:16 | -:4 Eb5 C5 Ab4 | D5:4 Bb4 G4:8 | \
             -:4 C5:3 Ab4:1 F4:3 Eb4:1 D4:3 F4:1 | Eb4:12 -:4",
            "I V7 | #ii°7 VI7 | iv6 #iv°7 | v6 I9 | IV ii7 | iii VI9 | ii V7 | I bVII #V7 V7",
        ),
        Part::new(
            "A3 end",
            "Bb4:8 Bb4 | C5:8 C5 | Eb5:8 Eb5 | F5:16 | -:4 Eb5 C5 Ab4 | D5:4 Bb4 G4:8 | \
             -:4 C5:3 Ab4:1 F4:3 Eb4:1 D4:3 F4:1 | Eb4:12 -:4",
            "I V7 | #ii°7 VI7 | iv6 #iv°7 | v6 I9 | IV ii7 | iii VI9 | ii V7 | I",
        ),
    ],
    form: &[
        "Verse", "Verse 2", "A", "A2", "B", "A3", "A", "A2", "B", "A3 end",
    ],
    drums_from: 0,
    about: "The refrain climbs one note at a time, B flat, C, E flat, F, each held for two \
            beats, while diminished seventh chords on F sharp and A pass between the chords \
            of the key. The bridge moves to C minor, the relative minor, reached by its own \
            V7, G7, with an augmented G7 at the end of the second A. The verse settles on B \
            flat, the V, for its last four bars before B flat 7 leads into the refrain.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/Strike_Up_the_Band/Strike_Up_the_Band.mxl
// (OpenEWLD lead sheet: the refrain in B flat, cut time, with its pickup and two endings;
// chords checked against the iReal Pro changes (in E flat). In the first ending the
// lead sheet's G and C triads clash with the held B flat: they are played as G minor 7
// and C minor 7, the iReal's turnaround. The pickup's F7, on its fourth beat in the
// lead sheet, is held for the whole pickup bar. No written verse found.)
pub(super) const STRIKE_UP_THE_BAND: Song = Song {
    name: "Strike Up the Band",
    origin: "George Gershwin, Strike Up the Band, 1927",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 224.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "ii–V–I",
    parts: &[
        Part::new("Pickup", "-:12 F4:3 E4:1", "V7"),
        Part::new(
            "A",
            "F4:4 G4 G4:8 | _:12 F4:3 E4:1 | F4:4 Ab4 Ab4:8 | _:12 F4:3 E4:1 | \
             F4:4 A4 A4:8 | _:8 F4:2 G4:4 F4:2 | Bb4:16 | _:12 Bb4:3 A4:1",
            "I | I6 I6 I6 I | #i°7 | #i°7 | V7 | V7 | I | bVII bVII bVII I7",
        ),
        Part::new(
            "B",
            "Bb4:4 C5 C5:8 | _:12 Bb4:3 A4:1 | Bb4:4 Db5 Db5:8 | _:12 Bb4:3 A4:1 | \
             Bb4:4 D5 D5:8 | _:8 Bb4:2 C5:4 Bb4:2 | Eb5:16 | _:8 F5:4 Eb5",
            "ii | ii ii ii IV | #iv°7 | #iv°7 | I7 | I7 | IV | IV iv6",
        ),
        Part::new(
            "C",
            "D5:4 D5:2 Bb4 D5:4 D5:2 Bb4 | D5:8 Eb5:4 D5 | C5:4 C5:2 A4 C5:4 C5:2 A4 | \
             C5:8 D5:4 C5 | Bb4:4 Bb4:2 G4 Bb4:4 Bb4:2 G4 | Bb4:8 Bb4:4 D5 | F5:16 | \
             _:10 -:2 F4:3 E4:1",
            "I I6 | I I IV IVmaj7 | vii III9 | vii III7 | vi II9 | vi II7 | V7 | V7",
        ),
        // The first ending, with the pickup back to the top.
        Part::new(
            "D",
            "F4:4 G4 G4:8 | _:12 D5:3 C#5:1 | D5:4 F5 F5:8 | _:12 -:4 | \
             Eb5:8 D5:4 C5 | G5:4 F5:8 D5:4 | Bb4:16 | _:4 -:8 F4:3 E4:1",
            "I | I | iii7 VI9 | VI9 VI9 VI9 VI7 | ii7 ii7 vi IV6 | vi6 V7sus4 V7sus4 V7 | \
             I vi7 | ii7 V7",
        ),
        Part::new(
            "D end",
            "F4:4 G4 G4:8 | _:12 D5:3 C#5:1 | D5:4 F5 F5:8 | _:12 -:4 | \
             Eb5:8 D5:4 C5 | G5:4 F5:8 D5:4 | Bb4:16 | _:12 -:4",
            "I | I | iii7 VI9 | VI9 VI9 VI9 VI7 | ii7 ii7 vi IV6 | vi6 V7sus4 V7sus4 V7 | I | I",
        ),
    ],
    form: &["Pickup", "A", "B", "C", "D", "A", "B", "C", "D end"],
    drums_from: 1,
    about: "Gershwin wrote it as a march, and swing bands took it at a gallop. In the first \
            two phrases a diminished seventh a half step above the bass (B diminished after B \
            flat, E diminished after E flat) carries the tune's top note, A flat and then D \
            flat; the third phrase walks down two ii–V pairs, A minor to D9 and G \
            minor to C9, onto F7.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/%27S_Wonderful/%27S_Wonderful.mxl
// (OpenEWLD lead sheet: the refrain, AABA, in E flat; chords checked against the iReal Pro
// changes. Bars 9-10 have no chord there: E flat, as in the first A. Bar 22 has none
// either: C7, the iReal's, between G9 and F9. In bar 7, C7b9 under the held E flat is
// played as C minor 7, the iReal's; B flat 7 flat 5 is written B flat 7. The bridge's
// first four bars are in G major. No written verse found.)
pub(super) const S_WONDERFUL: Song = Song {
    name: "'S Wonderful",
    origin: "George Gershwin, Funny Face, 1927",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 168.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "Bb4:6 Bb4:2 G4:8 | _:16 | Bb4:6 Bb4:2 G4:8 | _:16 | \
             Bb4:6 Bb4:2 G4:8 | _:8 Bb4 | Eb4:16 | _:12 -:4",
            "I | I | #i°7 | #i°7 | ii7 | ii7 V7 | I vi7 | II9 II9 II9 V7",
        ),
        Part::new(
            "A2",
            "Bb4:6 Bb4:2 G4:8 | _:16 | Bb4:6 Bb4:2 G4:8 | _:16 | \
             Bb4:6 Bb4:2 G4:8 | _:8 Bb4 | C5:16 | _:8 Eb5",
            "I | I | #i°7 | #i°7 | ii7 | ii7 V7 | I | I #ivø7",
        ),
        Part::new(
            "B",
            "D5:16 | -:4 D5 D5 D5 | E5:6 D5:2 D5:8 | _:16",
            "I | V7 | I | ii7 V7",
        )
        .in_key(G, Major),
        Part::new(
            "B2",
            "-:4 D5 D5 D5 | D5:4 D5 D5 D5 | D5:6 C5:2 C5:8 | _:8 Cb5",
            "III9 | VI7 | II9 | V13 V7b9",
        ),
        Part::new(
            "A3",
            "Bb4:6 Bb4:2 G4:8 | _:16 | Eb5:6 Eb5:2 C5:8 | _:16 | \
             -:2 Eb5:6 Ab4:4 G4 | Bb4:8 G4 | Eb4:16 | _:12 -:4",
            "I | I | vi II9 | vi II7 | ii7 VI7b9 | ii7 V13 | I | I",
        ),
    ],
    form: &["A", "A2", "B", "B2", "A3", "A", "A2", "B", "B2", "A3"],
    drums_from: 0,
    about: "The tune holds one note, G, for six beats at a time while the chords move \
            beneath it: E flat, then E diminished seven, a half step up, then F minor 7. The \
            bridge jumps to G major, a major third above E flat, and a chain of dominants, \
            G9, C7, F9 and B flat 13, falls by fifths back home.",
};

// Still under copyright: published in 1939 (Joe Garland died in 1985).
// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/JoeGarlandInTheMood.abc
// (the Wikifonia lead sheet in B flat, cut time: the introduction, the twelve-bar riff strain
// twice, the "in the mood" strain twice with its two endings, and the coda; chords checked
// against the Jazz Chord Progressions Corpus chart in A flat,
// https://raw.githubusercontent.com/carey-bunks/Jazz-Chord-Progressions-Corpus/master/SongDB/Songs%5BH-O%5D/InTheMood.txt,
// whose sixth chords are used for the riff strain's B flat and E flat)
pub(super) const IN_THE_MOOD: Song = Song {
    name: "In the Mood",
    origin: "Joe Garland, words by Andy Razaf, 1939",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 168.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "Jump blues",
    parts: &[
        Part::new(
            "Intro",
            "Bb4:2 D5:2 F5:2 Bb5:4 Bb5:4 Bb5:2 | _:2 Bb5:4 Bb5:2 A5:2 Bb5:2 F5:2 D5:2 | \
             F5:2 E5:2 Eb5:2 D5:2 Db5:2 C5:4 Bb4:2 | _:2 D5:2 F5:2 Bb5:4 F5:2 D5:2 Bb4:2 | \
             C5:2 D5:2 C5:2 Bb4:2 G4:4 C5:2 D5:2 | C5:2 Bb4:2 G4:2 G4:2 -:2 D5:4 -:2 | \
             C5:2 D5:2 C5:2 A4:2 G4:4 C5:2 D5:2 | C5:2 A4:2 G4:2 G4:2 -:2 Bb4:2 Bb4:2 -:2",
            "I | I | I | I | II7 | II7 | ii7 | V7",
        ),
        // The riff on the twelve-bar blues.
        Part::new(
            "A",
            "D4:2 F4 Bb4 D4 F4 Bb4 D4 F4 | Bb4 D4 F4 Bb4:6 -:4 | \
             D4:2 F4 Bb4 D4 F4 Bb4 D4 F4 | Bb4 D4 F4 Bb4:6 -:4 | \
             Eb4:2 G4 Bb4 Eb4 G4 Bb4 Eb4 G4 | Bb4 Eb4 G4 Bb4:6 -:4 | \
             D4:2 F4 Bb4 D4 F4 Bb4 D4 F4 | Bb4 D4 F4 Bb4:6 -:4 | \
             F4:2 A4 C5 F4 A4 C5 F4 A4 | C5 F4 A4 C5:4 Bb4:4 Bb4:2 | \
             _:2 Bb4 Bb4 Bb4 Bb4 Bb4 Bb4 Bb4 | Bb4 Bb4:4 Bb4:6 -:4",
            "I6 | I6 | I6 | I6 I6 I6 I7 | IV6 | IV6 | I6 | I6 | V7 | V7 | I6 IV6 | bVI7 I",
        ),
        // "In the mood, that's what she told me": first ending.
        Part::new(
            "B",
            "Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 F5:4 D5:2 | \
             F5:2 F5 F5 F5 F5 F5 F5 F5 | F5:4 Db5:2 C5:4 Bb4:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 F5:4 D5:2 | \
             F5:2 F5 F5 F5 F5 F5 F5 F5 | F5:4 Db5:2 C5:4 Bb4:4 -:2",
            "I6 I6 I6 biii°7 | ii7 V7b9 | I6 I6 I6 biii°7 | ii7 V7b9 | I6 I6 I6 biii°7 | ii7 | \
             V7 v°7 | V7 bVI9 V9 I | I6 I6 I6 biii°7 | ii7 V7b9 | I6 I6 I6 biii°7 | ii7 V7b9 | \
             I6 I6 I6 biii°7 | ii7 | V7 v°7 | V7 bVI7 V7 I",
        ),
        // The same with the second ending, into the coda.
        Part::new(
            "B2",
            "Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 F5:4 D5:2 | \
             F5:2 F5 F5 F5 F5 F5 F5 F5 | F5:4 Db5:2 C5:4 Bb4:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 D5:6 | \
             Bb4:2 G4:4 Bb4:6 -:4 | -:2 Eb4:2 G4:2 Bb4:2 D5:2 F5:4 D5:2 | \
             F5:2 F5 F5 F5 F5 F5 F5 F5 | F5:4 Db5:2 C5:4 Bb4:4 -:2",
            "I6 I6 I6 biii°7 | ii7 V7b9 | I6 I6 I6 biii°7 | ii7 V7b9 | I6 I6 I6 biii°7 | ii7 | \
             V7 v°7 | V7 bVI9 V9 I | I6 I6 I6 biii°7 | ii7 V7b9 | I6 I6 I6 biii°7 | ii7 V7b9 | \
             I6 I6 I6 biii°7 | ii7 | V7 v°7 | V7 bVI7 V9 I",
        ),
        Part::new(
            "Coda",
            "Bb3:2 C4:2 D4:2 Eb4:2 F4:2 Bb4:4 F4:2 | Gb4:4 Eb4:2 F4:6 -:4 | \
             -:2 Gb4:2 F4:4 Eb4:2 Db4:4 Gb4:2 | F4:4 Db4:2 Bb3:4 -:6",
            "I | bII I I I | V7 | V7 I",
        ),
    ],
    form: &["Intro", "A", "A", "B", "B2", "Coda"],
    drums_from: 0,
    about: "The riff is a B flat arpeggio, D F B flat, played over a twelve-bar blues: four \
            bars of B flat, two of E flat (IV), two of B flat, two of F7 (V7), then B flat and \
            E flat, and G flat 7 (bVI7) back to B flat. The second strain answers \"in the \
            mood\" with a diminished chord and a ii7-V7; its cadence and the coda bend the tune \
            with the blue notes D flat and G flat.",
};

// Still under copyright: published in 1936 (Louis Prima died in 1978).
// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/LouisPrimaSingSingSing.abc
// (the Wikifonia lead sheet in E minor: the 32-bar refrain, AABA, with its repeat; chords
// checked against the iReal Pro forum chart in the ChoCo corpus,
// https://raw.githubusercontent.com/smashub/choco/master/partitions/ireal-pro/choco/forum/jams/ireal-pro-forum_20884.jams.
// The bridge is in G major, the relative major.)
pub(super) const SING_SING_SING: Song = Song {
    name: "Sing, Sing, Sing",
    origin: "Louis Prima, 1936",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: E,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 200.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Gypsy jazz",
    parts: &[
        Part::new(
            "A",
            "B4:8 A4:8 | G4:8 A4:8 | -:2 B4:4 B4:2 A4:4 A4:4 | G4:4 A4:4 B4:8 | \
             E5:4 E5:4 B4:8 | E5:4 E5:4 B4:8 | B4:4 G4:4 A4:4 B4:4 | A4:4 G4:2 E4:6 -:4",
            "i V7 | i V7 | i V7 | i V7 | i V7 | i V7 | i | iv V7 i i",
        ),
        Part::new(
            "B",
            "G4:4 G4:4 A4:4 B4:4 | G4:4 B4:4 A4:8 | D5:4 D5:4 D5:6 C5:2 | B4:4 G4:4 G4:8 | \
             G4:4 G4:4 A4:4 B4:4 | G4:4 B4:4 A4:8 | B4:4 D5:4 D5:4 E5:4 | B4:4 G4:4 G4:4 -:4",
            "I V7 | I V7 | ii7 V7 | I | I V7 | I V7 | I V7 | I VI7",
        )
        .in_key(G, Major),
    ],
    form: &["A", "A", "B", "A", "A", "A", "B", "A"],
    drums_from: 0,
    about: "A minor-key riff tune: E minor and B7, the i and the V7, two beats each for six \
            bars, then E minor, and A minor (iv), B7 and E minor to close. The bridge moves to \
            G major, the relative major, with its own D7, and the B7 at its end leads back to \
            E minor.",
};

// Still under copyright: written in 1939 (Billy Strayhorn died in 1967).
// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/BillyStrayhornTakeTheATrain.abc
// (the Wikifonia lead sheet in C, AABA, checked note by note against the two lead sheets of
// the Open Songbook, https://raw.githubusercontent.com/jsnelgro/open-songbook/master/songs/d/duke-ellington-take-the-a-train/h-21b7b6a5.abc
// and https://raw.githubusercontent.com/jsnelgro/open-songbook/master/songs/b/billy-strayhorn-take-the-a-train/h-1008f739.abc;
// where they differ, two of the three are followed: the F sharp of the bridge's sixth bar
// is from the Songbook sheets. The coda is the first Songbook sheet's ending. Chords from
// the jazz standards' changes, "Take The A Train".)
pub(super) const TAKE_THE_A_TRAIN: Song = Song {
    name: "Take the \"A\" Train",
    origin: "Billy Strayhorn, 1939",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 176.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "G4:16 | _:2 E5:6 G4:4 C5:4 | E5:2 G#4:14 | _:16 | \
             A4:16 | A4:2 A#4:2 B4:2 E5:2 G4:2 Gb4:2 F4:2 Db5:2 | C5:2 E4:14 | _:16",
            "I6 | I6 | II7 | II7 | ii7 | V7 | I6 | ii7 V7",
        ),
        Part::new(
            "A2",
            "G4:16 | _:2 E5:6 G4:4 C5:4 | E5:2 G#4:14 | _:16 | \
             A4:16 | A4:2 A#4:2 B4:2 E5:2 G4:2 Gb4:2 F4:2 Db5:2 | C5:2 E4:14 | _:16",
            "I6 | I6 | II7 | II7 | ii7 | V7 | I6 | v7 I7",
        ),
        Part::new(
            "B",
            "A4:2 C5:14 | E5:2 F4:6 A4:4 C5:4 | E5:2 A4:14 | _:16 | \
             A4:2 C5:14 | E5:2 F#4:6 A4:4 C5:4 | E5:2 A4:14 | _:8 Ab4:8",
            "IVmaj7 | IVmaj7 | IVmaj7 | IVmaj7 | II7 | II7 | ii7 | V7 V7b9",
        ),
        // The last A, which leaves for the coda after its sixth bar: "Harlem",
        // then up the scale to C.
        Part::new(
            "A end",
            "G4:16 | _:2 E5:6 G4:4 C5:4 | E5:2 G#4:14 | _:16 | \
             A4:16 | A4:2 A#4:2 B4:2 E5:2 G4:2 Gb4:2 F4:2 Db5:2 | \
             C5:2 E4:6 F4:4 F#4:4 | G4:2 A4:2 B4:2 C5:2 -:4 C5:4 | _:16 | _:16",
            "I6 | I6 | II7 | II7 | ii7 | V7 | Imaj7 | Imaj7 | Imaj7 | Imaj7",
        ),
    ],
    form: &["A", "A2", "B", "A", "A", "A2", "B", "A end"],
    drums_from: 0,
    about: "The surprise is the second chord: D7 under a long G sharp, its raised eleventh, \
            where C major's own ii would be D minor. It is a II7, the dominant of the \
            dominant, and it settles to D minor 7 and G7, a ii-V back to C. The bridge sits \
            on F major 7 (IV) for four bars, then D7 again.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/Fascinating_Rhythm/Fascinating_Rhythm.mxl
// (OpenEWLD lead sheet: the refrain, in F as written there; chords checked against
// the iReal Pro chart in JazzStandards.json. The lead sheet's Em7 under the repeated
// C and A is the chart's E half-diminished, its B flat 7 flat 5 is written as E
// half-diminished, and bar 8, where it stacks Cm7, F7 and B flat 9, is Cm7 then F7
// like bar 4.)
pub(super) const FASCINATING_RHYTHM: Song = Song {
    name: "Fascinating Rhythm",
    origin: "George Gershwin, Lady, Be Good!, 1924",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 176.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "G4:2 Bb4 A4 G4 C5 C5 - G4 | Bb4 A4 G4 C5 C5 - G4 Bb4 | A4 G4 C5 C5 - G4 Bb4 A4 | \
             G4:4 C4:12 | C5:2 Eb5 D5 C5 F5 F5 - C5 | Eb5 D5 C5 F5 F5 - C5 Eb5 | \
             D5 C5 F5 F5 - C5 Eb5 D5 | C5:4 F4:12",
            "ii7 V7 | ii7 V7 | ii7 V7 | ii7 V7 V7 V7 | v7 I7 | v7 I7 | v7 I7 | v7 I7 I7 I7",
        ),
        Part::new(
            "B",
            "-:4 F4 Bb4 C5 | D5 C5:2 E5 _ C5 D5:4 | A4:16 | C5:2 A4 A4 A4 C5 A4 A4 A4 | \
             -:4 F4 A4 C5 | F5 E5:2 G5 _ D5 E5:4 | C5:16 | -:4 D5 Bb4 A4",
            "IV IV IV iii° | ii7 V7 | I | viiø7 III7 | vi7 | vi7 II7 | V7 | iiiø7 VI7",
        ),
        Part::new(
            "C",
            "-:4 F4 Bb4 C5 | D5 D5:2 E5 _ D5 E5:4 | -:4 F5 G5:2 E5:6 | D5:16 | \
             G4:2 Bb4 A4 G4 C5 C5 - G4 | Bb4 A4 G4 D5 _ F5 A4:4 | F4:16 | _:12 -:4",
            "IV IV IV iii° | ii7 ii7 viiø7 III7 | vi7 | II9 | ii7 V7 | ii7 ii7 II9 V13 | I | I",
        ),
    ],
    form: &["A", "B", "A", "C", "A", "B", "A", "C"],
    drums_from: 0,
    about: "The famous trick is in the first bars: a figure of six notes and a rest, seven \
            eighths long, played over and over, so each time it starts an eighth earlier \
            against the bar line, over G minor 7 and C7, a ii-V. It then moves up a fourth, \
            over C minor 7 and F7. The two middle sections differ: the first goes through \
            E half-diminished and A7 to D minor, the second through G9 to a ii-V home.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Gus_Kahn-Walter_Donaldson/Love_Me_or_Leave_Me/Love_Me_or_Leave_Me.mxl
// (OpenEWLD lead sheet: the refrain, AABA with first and second endings, in A minor as
// written there. Chords from the iReal Pro chart in JazzStandards.json, moved from F minor
// to A minor: they fit the melody where the lead sheet's do not, B7 under the F sharps
// where the sheet has B half-diminished, E minor 7 and E flat dim 7 under the G and
// F sharp of the bridge where it has C major 7. The last bars, C major 7 and F minor 6,
// are the lead sheet's second ending. The key is A Dorian: the tune's F sharps are the
// raised sixth of A minor, as in the sheet's A minor 6.)
pub(super) const LOVE_ME_OR_LEAVE_ME: Song = Song {
    name: "Love Me or Leave Me",
    origin: "Walter Donaldson, Whoopee!, 1928",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: A,
    mode: Dorian,
    time: COMMON,
    tempo: 160.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "Minor ii–V–i",
    parts: &[
        Part::new(
            "A",
            "E5:2 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:6 | \
             E5:2 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:4 E5:2 | \
             G5 G5:4 G5:2 G4 G4:4 G4:2 | A4 A4:4 A4:2 D5 C5 D5 C5 | _:16 | _:4 -:8 D5:4",
            "i7 | II7 V7 | i7 | II7 V7 | bIIImaj7 i7 | iv7 bVII7 | bIII6 | iiø7 V7b9",
        ),
        Part::new(
            "A2",
            "E5:2 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:4 G#4:2 | \
             E5 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:4 E5:2 | \
             G5 G5:4 G5:2 G4 G4:4 G4:2 | A4 A4:4 A4:2 D5 C5 D5 C5 | _:16 | _:8 -:8",
            "i7 | II7 V7 | i7 | II7 V7 | bIIImaj7 i7 | iv7 bVII7 | bIII6 | bIII6",
        ),
        Part::new(
            "B",
            "E5:4 G5:2 E5 E5 G5 E5:4 | C#5 A4:2 B4 _ C#5 A4:4 | F5:8 E5 | F5:16 | \
             D5:4 F5:2 D5 _ F5 D5:4 | B4 D5:2 A4 _ B4 G4:4 | G5:8 F#5 | F5:16",
            "v7 | I7 | iv7 I7b9 | iv7 | iv7 | bVII7 | v7 #iv°7 | iv7 V7b9",
        ),
        // The last A with the first ending, back to the top.
        Part::new(
            "A3",
            "E5:2 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:4 G#4:2 | \
             E5 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:4 E5:2 | \
             G5 G5:4 G5:2 G4 G4:4 G4:2 | A4 A4:4 A4:2 D5 C5 D5 C5 | _:4 -:12 | -:16",
            "i7 | II7 V7 | i7 | II7 V7 | bIIImaj7 i7 | iv7 bVII7 | bIII6 | iiø7 V7b9",
        ),
        // The last A with the second ending.
        Part::new(
            "A3 end",
            "E5:2 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:4 G#4:2 | \
             E5 E5:4 E5:2 E4 E4:4 E4:2 | F#4 F#4:4 F#4:2 G#4 G#4:4 E5:2 | \
             G5 G5:4 G5:2 G4 G4:4 G4:2 | A4 A4:4 A4:2 D5 C5 D5 C5 | C5:16 | _:12 -:4",
            "i7 | II7 V7 | i7 | II7 V7 | bIIImaj7 i7 | iv7 bVII7 | bIIImaj7 | \
             bvi6 bvi6 bvi6 bIIImaj7",
        ),
    ],
    form: &["A", "A2", "B", "A3", "A", "A2", "B", "A3 end"],
    drums_from: 0,
    about: "The tune hammers one note, E, then climbs F sharp, G sharp over B7 and E7, the \
            V of V and the V of A minor. In the fifth bar it jumps from A minor straight to \
            C, the relative major, with no chord to prepare it. The bridge runs E minor 7, \
            A7, D minor 7, G7, and the last bars go from C major 7 to F minor 6 and back: \
            the minor iv, borrowed from C minor.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Gus_Kahn-Walter_Donaldson/Makin%27_Whoopee%21/Makin%27_Whoopee%21.mxl
// (OpenEWLD lead sheet: the refrain, AABA with first and second endings and a segno after
// the pickup, in F as written there; chords checked against the iReal Pro chart in
// JazzStandards.json (Making Whoopee, in G), which has the same changes. The turnaround
// of the last bar, in brackets on the sheet, is played the first time only.)
pub(super) const MAKIN_WHOOPEE: Song = Song {
    name: "Makin' Whoopee",
    origin: "Walter Donaldson, Whoopee!, 1928",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 132.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Backdoor ii–V",
    parts: &[
        Part::new("Pickup", "-:10 C4:2 D4 C4", "V7"),
        Part::new(
            "A",
            "A4:10 C4:2 D4 C4 | Bb4:10 C4:2 D4 C4 | C5:10 F4:2 G4 F4 | C5:10 Bb4:2 C5 Bb4 | \
             A4:4 A4:6 C4:2 D4 C4 | Ab4:4 Ab4:6 F#4:2 G4 G#4 | A4:4 F4:12 | _:8 -:2 C4 D4 C4",
            "Imaj7 #i°7 | ii7 V7 | v7 I7 | IVmaj7 bVII7 | Imaj7 vi7 | bVI7 V7 | I6 vi7 vi7 vi7 | \
             ii7 V7",
        ),
        Part::new(
            "A2",
            "A4:10 C4:2 D4 C4 | Bb4:10 C4:2 D4 C4 | C5:10 F4:2 G4 F4 | C5:10 Bb4:2 C5 Bb4 | \
             A4:4 A4:6 C4:2 D4 C4 | Ab4:4 Ab4:6 F#4:2 G4 G#4 | A4:4 F4:12 | _:8 -:8",
            "Imaj7 #i°7 | ii7 V7 | v7 I7 | IVmaj7 bVII7 | Imaj7 vi7 | bVI7 V7 | I6 | I6",
        ),
        Part::new(
            "B",
            "C5:2 Eb5:4 C5:2 Eb5:4 C5 | Bb4 Bb4:12 | Bb4:2 Db5:4 Bb4:2 Db5:4 Bb4 | A4:16 | \
             C5:2 Eb5:4 C5:2 Eb5:4 C5 | Bb4 Bb4:12 | Bb4:2 Db5:4 Bb4:2 Db5:4 Bb4 | \
             A4:10 C4:2 D4 C4",
            "iiiø7 VI7b9 | ii7 | iv7 bVII7 | Imaj7 | iiiø7 VI7b9 | ii7 | iv7 bVII7 | \
             iii7 VI7b9 ii7 V7",
        ),
        // The last A, with the turnaround back to the segno.
        Part::new(
            "A3",
            "A4:10 C4:2 D4 C4 | Bb4:10 C4:2 D4 C4 | C5:10 F4:2 G4 F4 | C5:10 Bb4:2 C5 Bb4 | \
             A4:4 A4:6 C4:2 D4 C4 | Ab4:4 Ab4:6 F#4:2 G4 G#4 | A4:4 F4:12 | _:8 -:8",
            "Imaj7 #i°7 | ii7 V7 | v7 I7 | IVmaj7 bVII7 | Imaj7 vi7 | bVI7 V7 | I6 vi7 vi7 vi7 | \
             ii7 V7",
        ),
    ],
    form: &["Pickup", "A", "A2", "B", "A3", "A", "A2", "B", "A2"],
    drums_from: 1,
    about: "The A section is a string of ii-V pairs, G minor 7 to C7, then C minor 7 to F7, \
            which lands on B flat, the IV; later D flat 7 slides down a half step to C7. \
            The bridge comes home by the back door: B flat minor 7 and E flat 7, borrowed \
            from F minor, lead to F major instead of the usual G minor 7 and C7.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Billy_Rose-Edward_Eliscu-Vincent_Youmans/Without_a_Song/Without_a_Song.mxl
// (OpenEWLD lead sheet: the refrain, AABA with the four-bar tag that ends the last A, in
// E flat as written there, with the lead sheet's chords. The iReal Pro chart in
// JazzStandards.json is a jazz reharmonisation in bars twice as long, with a different
// bridge, so it was used only to check the A sections, where both agree on E flat,
// A flat major 7 and B flat 7. In bar 4 of the bridge the sheet stacks B flat 7 and
// E flat on one note: E flat is kept.)
pub(super) const WITHOUT_A_SONG: Song = Song {
    name: "Without a Song",
    origin: "Vincent Youmans, Great Day, 1929",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 120.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "ii–V–I",
    parts: &[
        Part::new("Pickup", "-:10 Bb4:2 C5 D5", "V7"),
        Part::new(
            "A",
            "D5:10 Bb4:2 Bb4 G4 | G4 Eb4 Eb4:4 -:2 G4 G4 Bb4 | Bb4:10 G4:2 G4 Eb4 | \
             Eb4 C4 C4:4 -:2 Eb4 Eb4 G4 | G4:10 Eb4:2 Eb4 C4 | C4 Bb3 Bb3:4 -:2 C4 C4 Eb4 | \
             Eb4:16 | _:4 -:6 Bb4:2 Bb4 D5",
            "Imaj7 | IVmaj7 | Imaj7 | IV6 IV | I | V7 V11 | I | ii7 V7 V7 V7",
        ),
        Part::new(
            "A2",
            "D5:10 Bb4:2 Bb4 G4 | G4 Eb4 Eb4:4 -:2 G4 G4 Bb4 | Bb4:10 G4:2 G4 Eb4 | \
             Eb4 C4 C4:4 -:2 Eb4 Eb4 G4 | G4:10 Eb4:2 Eb4 C4 | C4 Bb3 Bb3:4 -:2 C4 C4 Eb4 | \
             Eb4:16 | _:2 -:2 Eb4:4 F4 G4",
            "Imaj7 | IVmaj7 | Imaj7 | IV6 IV | I | V7 | I6 | I6 I6 v7 I7",
        ),
        Part::new(
            "B",
            "Ab4:4 G4:2 F4 G4:4 Ab4 | Bb4 Ab4:2 G4 Ab4:4 Bb4 | C5 Bb4:2 Ab4 Bb4:8 | _:12 -:4 | \
             Bb4:4 A4:2 G4 A4:4 Bb4 | C5 Bb4:2 A4 Bb4:4 C5 | D5 Eb5:2 C5 F5:8 | \
             _:8 -:2 Bb4 Bb4 D5",
            "IV IV I7 IV | I7 I7 IV I7 | IV V7 I I | I I I II9 | iii iii VII7 iii7 | \
             VI7 I iii II9 | V7 I V9 V9 | Imaj7 Imaj7 ii7 V7",
        ),
        // The last A and its tag: the closing line again, ending an octave higher.
        Part::new(
            "A3",
            "D5:10 Bb4:2 Bb4 G4 | G4 Eb4 Eb4:4 -:2 G4 G4 Bb4 | Bb4:10 G4:2 G4 Eb4 | \
             Eb4 C4 C4:4 -:2 Eb4 Eb4 G4 | G4:10 Eb4:2 Eb4 C4 | C4 Bb3 Bb3:4 -:2 C4 C4 Eb4 | \
             Eb4:10 G4:2 G4 Bb4 | Bb4:10 G4:2 G4 Eb4 | Eb4 C4 C4:6 Eb4:2 Eb4 G4 | \
             G4:10 Eb4:2 G4 C5 | C5 Bb4 Bb4:6 C5:2 C5 Eb5 | Eb5:16",
            "Imaj7 | IVmaj7 ii7 | Imaj7 | IV6 IV | Imaj7 | V7 V11 | I Imaj7 | I9 I9 I9 I+ | \
             IV6 II9 II9 II9 | I | V7 V7 V7 V7 V7 #iv° V9 bVI7 | I6",
        ),
    ],
    form: &["Pickup", "A", "A2", "B", "A", "A", "A2", "B", "A3"],
    drums_from: 1,
    about: "The A section rocks between E flat and A flat, I and IV, while the tune falls \
            mostly in thirds, D, B flat, G, E flat, C, until B flat 7 brings it home. The \
            bridge starts in A flat with E flat 7 as its dominant, then turns to G minor \
            through D7. The last A adds a four-bar tag that sings the closing line again, \
            this time ending an octave higher.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/Embraceable_You/Embraceable_You.mxl
// (OpenEWLD lead sheet: the refrain, ABAC with first and second endings, in G as written
// there. Chords from the iReal Pro chart in JazzStandards.json, which agrees with most of
// the sheet's (its C sharp dim is the chart's B flat dim 7, its Am/F sharp the chart's
// F sharp half-diminished), except where the chart's chord fights a held melody note:
// there the lead sheet's is kept, G in bar 8 (the chart's F sharp half-diminished under
// a held G), and E minor, E flat augmented, G and E minor 6 in bars 27 and 28.)
pub(super) const EMBRACEABLE_YOU: Song = Song {
    name: "Embraceable You",
    origin: "George Gershwin, Girl Crazy, 1930",
    feel: "Ballad",
    genre: Genre::Jazz,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 66.0,
    comping: Comping::Ballad,
    bass: Bass::RootFifth,
    rhythm: "Jazz ballad",
    band: BALLAD_BAND,
    style: "Jazz ballad",
    parts: &[
        Part::new(
            "A",
            "-:4 E4 F#4 G4 | -:4 E4 F#4 G4 | A4:2 -:4 D4:2 D4:4 D4 | _:16 | \
             -:4 A4 B4 C5 | -:4 A4 B4 C5 | D5:2 -:4 G4:2 G4:4 G4 | _:16",
            "Imaj7 | biii°7 | ii7 | V7 | ii7 | iv6 | Imaj7 | Imaj7",
        ),
        Part::new(
            "B",
            "B4:4 G4 A4 G4 | B4 G4 A4 G4 | B4:2 -:4 F#4:2 F#4:4 F#4 | _:16 | \
             A4:4 F#4 G4 F#4 | G4 A4 B4 C#5 | D5:2 -:4 D4:2 D4:4 D4 | _:16",
            "vi vi7 | #ivø7 VII7 | iii7 | vi7 II7 | Vmaj7 iii7 | vi7 II7 | ii7 | V7",
        ),
        Part::new(
            "A2",
            "-:4 E4 F#4 G4 | -:4 E4 F#4 G4 | A4:2 -:4 D4:2 D4:4 D4 | _:16 | \
             -:4 A4 B4 C5 | -:4 A4 B4 C5 | D5:2 -:4 G4:2 G4:4 G4 | _:16",
            "Imaj7 | biii°7 | ii7 | V7 | ii7 | iv6 | Imaj7 | v7 I7",
        ),
        // The last section with the first ending, back to the top.
        Part::new(
            "C",
            "-:4 D5 C5 B4 | C5 B4 A4 B4 | -:4 G4:2 G4 G4 G4 G4 G4 | G4 G4 B4:12 | \
             -:4 B4 C5 D5 | Eb5:2 -:4 G4:2 F#4:8 | G4:16 | -:16",
            "IVmaj7 | viiø7 III7b9 | vi vi vi bVI+ | I vi6 vi6 vi6 | iii7 VI7 | iiø7 V7b9 | \
             Imaj7 | ii7 V7",
        ),
        // The last section with the second ending.
        Part::new(
            "C end",
            "-:4 D5 C5 B4 | C5 B4 A4 B4 | -:4 G4:2 G4 G4 G4 G4 G4 | G4 G4 B4:12 | \
             -:4 B4 C5 D5 | Eb5:2 -:4 G4:2 F#4:8 | G4:16 | _:4 -:12",
            "IVmaj7 | viiø7 III7b9 | vi vi vi bVI+ | I vi6 vi6 vi6 | iii7 VI7 | iiø7 V7b9 | \
             Imaj7 | Imaj7",
        ),
    ],
    form: &["A", "B", "A2", "C", "A", "B", "A2", "C end"],
    drums_from: 0,
    about: "Each phrase starts after a rest on the downbeat, with three rising notes. In the \
            second bar a diminished seventh, B flat dim 7, passes between G and A minor, and \
            C minor 6, the iv borrowed from G minor, shades the sixth. The bridge leans \
            toward D, the V, through A7; near the end an inner voice slides down by half \
            steps, E, E flat, D, C sharp, through E minor, E flat augmented, G and E minor 6.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/George_Gershwin-Ira_Gershwin/But_Not_for_Me/But_Not_for_Me.mxl
// (OpenEWLD lead sheet: the refrain, in E flat as written there. The sheet writes the
// three-note pickup as a whole first bar and so stops one bar short; the chords show that
// "songs" is the first downbeat, as in the iReal Pro chart in JazzStandards.json, whose
// chords are used: in bars 1-2 its alternatives E flat major 7, C minor 7, F minor 7,
// B flat 7, which are the lead sheet's. Where the chart's chord fights a held note the
// lead sheet's is kept: E flat major 7 alone under the long G of bar 3, and the sheet's
// E flat augmented under the B naturals that open the bridge and the last section.
// The last bar, missing from the sheet, holds the final note.)
pub(super) const BUT_NOT_FOR_ME: Song = Song {
    name: "But Not for Me",
    origin: "George Gershwin, Girl Crazy, 1930",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 160.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Turnaround",
    parts: &[
        Part::new("Pickup", "-:4 F4 G4 F4", "V7"),
        Part::new(
            "A",
            "G4:8 F4:4 Eb4 | _:2 -:2 F4:4 G4 F4 | G4:16 | -:4 F4 G4 F4 | \
             G4:8 F4:4 Eb4 | _:2 -:2 F4:4 G4 F4 | G4:16 | -:4 G4 Ab4 Bb4",
            "Imaj7 vi7 | ii7 V7 | Imaj7 | iii7 VI7 | II7 | V7 | Imaj7 | v7 I7",
        ),
        Part::new(
            "B",
            "B4:8 C5:4 Eb5 | -:4 F4 G4 Ab4 | A4:8 Bb4:4 Eb5 | -:4 Eb4 F4 G4 | \
             G4:8 Ab4:4 Eb5 | -:4 Eb5 C5 Ab4 | F4:16 | -:4 F4 G4 F4",
            "I+ IVmaj7 | bVII7 | Imaj7 | vi7 | ii7 | ii7 | ii7 | V7",
        ),
        // The last section, with the pickup back to the top.
        Part::new(
            "C",
            "B4:8 C5:4 Eb5 | -:4 F4 G4 Ab4 | A4:8 Bb4:4 F5 | -:4 Eb5 C5 Ab4 | \
             G4:16 | F4 | Eb4 | -:4 F4 G4 F4",
            "I+ IVmaj7 | bVII7 | Imaj7 IV7 | iii7 VI7 | ii7 | V7 | I6 | I6 V7 V7 V7",
        ),
        Part::new(
            "C end",
            "B4:8 C5:4 Eb5 | -:4 F4 G4 Ab4 | A4:8 Bb4:4 F5 | -:4 Eb5 C5 Ab4 | \
             G4:16 | F4 | Eb4 | _:12 -:4",
            "I+ IVmaj7 | bVII7 | Imaj7 IV7 | iii7 VI7 | ii7 | V7 | I6 | I6",
        ),
    ],
    form: &["Pickup", "A", "B", "A", "C", "A", "B", "A", "C end"],
    drums_from: 1,
    about: "The A sections are sung almost all on three notes, E flat, F and G, while the \
            chords move under them: a turnaround through G minor 7 and C7 to F7, the V of V. \
            The bridge opens on a B natural over an augmented E flat chord that pushes up \
            to C over A flat, then reaches E flat through D flat 7: IV, bVII7, I, the back \
            door home.",
};

// Public domain in the US (published in 1925), still under copyright in
// countries where it lasts 70 years after the authors' deaths (Maceo Pinkard died in 1962).
// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Ben_Bernie-Kenneth_Casey-Maceo_Pinkard/Sweet_Georgia_Brown/Sweet_Georgia_Brown.mxl
// (OpenEWLD lead sheet, the refrain, in F. The sixth bar has an F sharp in the
// file where the same bar later has F natural, the seventh of G7; F natural
// here too, as in John Chambers's ABC of the 1925 Remick edition,
// https://raw.githubusercontent.com/syntithenai/abc2book/HEAD/abcresources/jc/abc_tune_jc_201342268.abc.
// Chords checked against the iReal changes; the last bar's F7 E7 E flat 7
// turnaround becomes F6 the last time, as there.)
pub(super) const SWEET_GEORGIA_BROWN: Song = Song {
    name: "Sweet Georgia Brown",
    origin: "Ben Bernie, Maceo Pinkard and Kenneth Casey, 1925",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 200.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Turnaround",
    parts: &[
        Part::new(
            "A",
            "D4:4 E4 F#4 D4 | A4:4 F#4 B4 A4 | D5:8 A4:4 F#4 | D4:16 | \
             D4:4 E4 F4 D4 | A4:4 F4 B4 A4 | D5:8 B4:4 A4 | G4:16 | \
             C4:4 D4 E4 C4 | G4:4 E4 A4 G4 | C5:8 G4:4 E4 | C4:4 C4:2 D4:4 C4:2 D4:4 | \
             A4:16 | -:4 C4:2 D4:4 C4:2 D4:4 | A4:12 A4:4 | G4:4 E4 A4:8",
            "VI7 | VI7 | VI7 | VI7 | II7 | II7 | II7 | II7 | V7 | V7 | V7 | V7 | \
             I | V+ | I | vii7 III7",
        ),
        Part::new(
            "B",
            "D4:4 E4 F#4 D4 | A4:4 F#4 B4 A4 | D5:8 A4:4 F#4 | D4:16 | \
             D4:4 E4 F4 D4 | A4:4 F4 B4 A4 | D5:8 B4:4 A4 | G4:16 | \
             A4:6 A4:6 -:4 | G4:2 E4:4 A4:6 -:4 | A4:4 A4 A4 A4 | G4:2 E4:4 A4:6 -:4 | \
             C4:4 D4 F4 A4 | C5:4 C#5 D5 A4 | G4:8 C5:4 A4 | F4:4 -:12",
            "VI7 | VI7 | VI7 | VI7 | II7 | II7 | II7 | II7 | vi | III7 | vi | III7 | \
             I VII7 | bVII7 VI7 | II7 V7 | I7 I7 VII7 bVII7",
        ),
        // The last time: home on F instead of the turnaround.
        Part::new(
            "B end",
            "D4:4 E4 F#4 D4 | A4:4 F#4 B4 A4 | D5:8 A4:4 F#4 | D4:16 | \
             D4:4 E4 F4 D4 | A4:4 F4 B4 A4 | D5:8 B4:4 A4 | G4:16 | \
             A4:6 A4:6 -:4 | G4:2 E4:4 A4:6 -:4 | A4:4 A4 A4 A4 | G4:2 E4:4 A4:6 -:4 | \
             C4:4 D4 F4 A4 | C5:4 C#5 D5 A4 | G4:8 C5:4 A4 | F4:4 -:12",
            "VI7 | VI7 | VI7 | VI7 | II7 | II7 | II7 | II7 | vi | III7 | vi | III7 | \
             I VII7 | bVII7 VI7 | II7 V7 | I6",
        ),
    ],
    form: &["A", "B", "A", "B end"],
    drums_from: 0,
    about: "A chain of dominant sevenths, four bars each and each the V of the next: D7, G7, \
            C7, and only then F, the home chord, in the thirteenth bar. The tune keeps one \
            shape, D E F sharp D, and turns F sharp into F natural when the chord becomes G7. \
            The second half heads for D minor (vi), and near the end the chords slide down \
            by half steps, F, E7, E flat 7, D7.",
};

// Public domain in the US (published in 1929), still under copyright in
// countries where it lasts 70 years after the authors' deaths (Harry Brooks died in 1970).
// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Andy_Razaf-Fats_Waller-Harry_Brooks/Ain%27t_Misbehavin%27/Ain%27t_Misbehavin%27.mxl
// (OpenEWLD lead sheet, the refrain, in F. Its first bar puts the eighth rest
// at the end; it is at the start here, as in every other phrase of the file
// and in the C lead sheet at
// https://raw.githubusercontent.com/neonscribe/lilypond-lead-sheets/HEAD/Core/Ain%27t%20Misbehavin%27%20-%20Ly%20Core%20-%20C.ly.
// The A held at the first ending is tied, as in the file's tie mark. Chords
// from the lead sheet; the second ending's, which the file stacks on one
// beat, and the turnaround that leads back to the top, from the iReal
// changes.)
pub(super) const AINT_MISBEHAVIN: Song = Song {
    name: "Ain't Misbehavin'",
    origin: "Fats Waller and Harry Brooks, words by Andy Razaf, Hot Chocolates, 1929",
    feel: "Swing",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 116.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SWING,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "-:2 F4:2 G4 F4 C5 C5:6 | -:2 G4:2 A4 G4 D5:8 | -:2 C5:2 D5 C5 F5 F5:4 E5:2 | \
             G5:2 F5 D5 A4:4 Ab4:2 G4:4 | -:2 F4:2 G4 F4 C5 C5:6 | \
             -:2 G4:2 A4 G4 D5 D5:4 C5:2 | A4:16 | _:8 -:8",
            "I | ii7 V7 | I | IV iv | I vi | ii7 V7 | I | ii7 V7",
        ),
        Part::new(
            "A2",
            "-:2 F4:2 G4 F4 C5 C5:6 | -:2 G4:2 A4 G4 D5:8 | -:2 C5:2 D5 C5 F5 F5:4 E5:2 | \
             G5:2 F5 D5 A4:4 Ab4:2 G4:4 | -:2 F4:2 G4 F4 C5 C5:6 | \
             -:2 G4:2 A4 G4 D5 D5:4 C5:2 | F4:16 | _:8 -:8",
            "I | ii7 V7 | I | IV iv | I vi | ii7 V7 | I6 iv6 | I6 III7",
        ),
        Part::new(
            "B",
            "-:2 F5:2 D5 F5 D5:6 -:2 | -:2 F5:2 D5 F5 D5:6 -:2 | -:2 F5:2 D5 F5 D5:6 -:2 | \
             -:2 F#5:2 D5 F#5 D5:4 -:4 | -:4 G5:4 G5 G5 | G5:4 F5 E5 D5 | C5:8 D5 | A4:8 G4",
            "vi | IV7 | II7 | VI7 | V | vi7 II7 | V7 VI7 | II7 V7",
        ),
        Part::new(
            "A3",
            "-:2 F4:2 G4 F4 C5 C5 -:4 | -:2 G4:2 A4 G4 D5:8 | -:2 C5:2 D5 C5 F5 F5:4 E5:2 | \
             G5:2 F5 D5 A4:4 Ab4:2 G4:4 | -:2 F4:2 G4 F4 C5 C5:6 | \
             -:2 G4:2 A4 G4 D5 D5:4 C5:2 | F5:16 | _:12 -:4",
            "I | ii7 V7 | I | IV iv | I vi | ii7 V7 | I vi7 | ii7 V7",
        ),
        // The last time: the held F stays on F.
        Part::new(
            "A3 end",
            "-:2 F4:2 G4 F4 C5 C5 -:4 | -:2 G4:2 A4 G4 D5:8 | -:2 C5:2 D5 C5 F5 F5:4 E5:2 | \
             G5:2 F5 D5 A4:4 Ab4:2 G4:4 | -:2 F4:2 G4 F4 C5 C5:6 | \
             -:2 G4:2 A4 G4 D5 D5:4 C5:2 | F5:16 | _:12 -:4",
            "I | ii7 V7 | I | IV iv | I vi | ii7 V7 | I | I",
        ),
    ],
    form: &["A", "A2", "B", "A3", "A", "A2", "B", "A3 end"],
    drums_from: 0,
    about: "Most phrases start an eighth note late, after a rest, and the fourth bar turns \
            B flat major (IV) into B flat minor (iv), with an A flat in the tune. In the \
            bridge the tune repeats one figure, F D F D, while the chords move under it from \
            D minor (vi) to B flat 7 and G7; it becomes F sharp D over D7, and a chain of \
            dominants, G7 and C7, leads back home to F.",
};

// Public domain in the US (published in 1924), still under copyright in
// countries where it lasts 70 years after the authors' deaths (Isham Jones died in 1956).
// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Gus_Kahn-Isham_Jones/It_Had_to_Be_You/It_Had_to_Be_You.mxl
// (OpenEWLD lead sheet: the verse and the refrain, in G. The refrain's
// "you", pushed an eighth early and tied over the bar line, is written on
// the downbeat where it falls at the start of a part, since a part cannot
// begin with a tie. Chords from the lead sheet; where the file stacks two
// chords on one beat or misplaces one (refrain bars 1, 3, 7, 17 to 20, 23
// to 25, 31 and 32), from the iReal changes.)
pub(super) const IT_HAD_TO_BE_YOU: Song = Song {
    name: "It Had to Be You",
    origin: "Isham Jones, words by Gus Kahn, 1924",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 132.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Turnaround",
    parts: &[
        Part::new(
            "Verse",
            "D5:4 B4 D5 B4 | D5:2 E5 D5 B4:10 | A4:4 G4 A4 G4 | A4:2 B4 A4 G4:10 | \
             E4:2 G4 A4 B4:10 | E4:2 G4 A4 B4:4 D5:2 B4:4 | G4:12 -:4 | -:4 B4:4 C5 C#5 | \
             D5:4 B4 D5 B4 | D5:2 E5 D5 B4:10 | A4:4 G4 A4 G4 | A4:2 B4 A4 G4:10 | \
             E4:2 G4 A4 B4:10 | E4:2 G4 A4 B4:4 E5:2 D5:4 | A4:16 | _:4 D4:4 E4:2 D4 E4:4",
            "Imaj7 | I7 | IV6 iv6 | iii7 vi | II7 | ii7 V7 | I | V7sus4 V7sus4 V7 i° | \
             I Imaj7 | I7 | IV6 iv6 | I vi | II7 | ii7 V7 | V7 | V7 V+",
        ),
        Part::new(
            "Chorus",
            "F#4:16 | _:4 F#4:4 G4:2 F#4 G4 G#4 | _:16 | _:4 G#4:4 A4:2 G#4 A4 B4 | \
             _:6 G4:2 A4 G#4 A4 B4 | _:6 G4:2 A4 G#4 A4 B4 | _:16 | _:4 A4:4 B4:2 A4 B4 D5 | \
             _:16 | _:4 B4:4 A4:2 B4 A4 G4 | _:16 | _:4 G4:4 G4:2 F#4 G4 A4 | \
             _:8 G4:2 F#4 G4 A4 | _:8 B4:2 E5 B4 A4 | _:16 | _:4 D4:4 E4:2 D4 E4:4",
            "Imaj7 V+ | Imaj7 bVII9 | VI7 iii7 | VI7 | II9 vi7 | II9 vi7 | II9 vi7 | II9 | \
             V7 | V7 #v° | vi7 | vi7 bIII7 | II7 vi7 | II7 | ii7 | V7 V+",
        ),
        // The second half, with the first ending: back to the chorus.
        Part::new(
            "Chorus 2",
            "F#4:16 | _:4 F#4:4 G4:2 F#4 G4 G#4 | _:16 | _:4 G#4:4 A4:2 G#4 A4 B4 | \
             _:6 G4:2 A4 G#4 A4 B4 | _:6 G4:2 A4 G#4 A4 B4 | _:16 | _:4 B4:4 B4:2 C5 D5 E5 | \
             _:8 E4:2 F#4 G4 A4 | _:8 A4:2 B4 C5 D5 | _:8 A4:2 B4 A4 G4 | \
             _:4 F#4:4 E4:2 D4 E4 F#4 | _:8 E4:2 D4 E4 F#4 | _:8 B4:2 D5 B4 G4 | _:16 | \
             -:4 D4:4 E4:2 D4 E4:4",
            "Imaj7 V+ | Imaj7 bVII9 | VI7 iii7 | VI7 | II9 vi7 | II9 vi7 | II9 vi7 | \
             II7 II7 v7 I7 | IVmaj7 | ii°7 V7 | I #v° | vi #iv° | V7 bii°7 | V7 iii | I6 VI7 | \
             ii7 V7",
        ),
        // The second half with the second ending, the last time.
        Part::new(
            "Chorus end",
            "F#4:16 | _:4 F#4:4 G4:2 F#4 G4 G#4 | _:16 | _:4 G#4:4 A4:2 G#4 A4 B4 | \
             _:6 G4:2 A4 G#4 A4 B4 | _:6 G4:2 A4 G#4 A4 B4 | _:16 | _:4 B4:4 B4:2 C5 D5 E5 | \
             _:8 E4:2 F#4 G4 A4 | _:8 A4:2 B4 C5 D5 | _:8 A4:2 B4 A4 G4 | \
             _:4 F#4:4 E4:2 D4 E4 F#4 | _:8 E4:2 D4 E4 F#4 | _:8 B4:2 D5 B4 G4 | _:16 | \
             _:12 -:4",
            "Imaj7 V+ | Imaj7 bVII9 | VI7 iii7 | VI7 | II9 vi7 | II9 vi7 | II9 vi7 | \
             II7 II7 v7 I7 | IVmaj7 | ii°7 V7 | I #v° | vi #iv° | V7 bii°7 | V7 iii | I | Imaj7",
        ),
    ],
    form: &["Verse", "Chorus", "Chorus 2", "Chorus", "Chorus end"],
    drums_from: 0,
    about: "After a sixteen-bar verse, the chorus walks round a chain of dominants, E7, A7 \
            and D7, each the V of the next. Its five-note phrase comes three \
            times, each a little higher, landing on F sharp (the major seventh of G), \
            G sharp (the third of E7) and B (the ninth of A9).",
};

// Public domain in the US (published in 1917), still under copyright in
// countries where it lasts 70 years after the authors' deaths (Tony Sbarbaro died in 1969).
// Source: https://raw.githubusercontent.com/wwrinkle/WilliesLilyPondSetup/HEAD/files/Brass%20Band/Tiger%20Rag.ly
// (a brass band lead sheet with chords: the first strain in B flat, A A B A,
// and the two strains in E flat, with their solo breaks; in the B section of
// the first strain the top voice, the stop-time hits, is the melody, and the
// walking line under it is left out) and
// https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Eddie_Edwards-Henry_Ragas-Larry_Shields-Nick_LaRocca-Tony_Sbarbaro/Tiger_Rag/Tiger_Rag.mxl
// (OpenEWLD lead sheet of the "Hold that tiger" strain, with its two endings,
// up a semitone from G to A flat, the key it has after the E flat strains).
// The order of the strains and their chords match the iReal changes.
pub(super) const TIGER_RAG: Song = Song {
    name: "Tiger Rag",
    origin: "Original Dixieland Jass Band, 1917",
    feel: "Dixieland",
    genre: Genre::Jazz,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 200.0,
    comping: Comping::Quarters,
    bass: Bass::RootFifth,
    rhythm: "Swing",
    band: DIXIELAND,
    style: "New Orleans",
    parts: &[
        Part::new(
            "A",
            "Bb4:4 Bb4 Bb4 Bb4 | Bb4:2 Bb4:4 F4:2 D4:4 F4 | Bb4:4 D5:2 C5:4 F4:2 G4 A4 | \
             Bb4:2 A4 Bb4 C5 D5:8 | Bb4:4 Bb4 Bb4 Bb4 | Bb4:2 Bb4:4 F4:2 D4:4 F4 | \
             Bb4:4 D5:2 C5:4 F4:2 G4 A4 | Bb4:10 F4:2 G4 A4",
            "I | I | I V7 | I | I | I | I V7 | I",
        ),
        Part::new(
            "A2",
            "Bb4:4 Bb4 Bb4 Bb4 | Bb4:2 Bb4:4 F4:2 D4:4 F4 | Bb4:4 D5:2 C5:4 F4:2 G4 A4 | \
             Bb4:2 A4 Bb4 C5 D5:8 | Bb4:4 Bb4 Bb4 Bb4 | Bb4:2 Bb4:4 F4:2 D4:4 F4 | \
             Bb4:4 D5:2 C5:4 F4:2 G4 A4 | Bb4:12 -:4",
            "I | I | I V7 | I | I | I | I V7 | I",
        ),
        // Stop-time: the band hits the chord on beats 1 and 4.
        Part::new(
            "B",
            "C5:4 -:8 Bb4:4 | A4:4 -:8 A4:4 | Bb4:4 -:8 Bb4:4 | A4:4 -:12 | \
             C5:4 -:8 Bb4:4 | A4:4 -:8 A4:4 | Bb4:4 -:8 G4:4 | F4:4 -:4 D5:2 Db5 C5 B4",
            "II7 | V | II7 | V | II7 | V | II7 | V",
        ),
        // The last A turns B flat into B flat 7, the V7 of E flat.
        Part::new(
            "A3",
            "Bb4:4 Bb4 Bb4 Bb4 | Bb4:2 Bb4:4 F4:2 D4:4 F4 | Bb4:4 D5:2 C5:4 F4:2 G4 A4 | \
             Bb4:2 A4 Bb4 C5 D5:8 | Bb4:4 Bb4 Bb4 Bb4 | Bb4:2 Bb4:4 F4:2 D4:4 F4 | \
             Bb4:4 D5:2 C5:4 F4:2 G4 A4 | Bb4:4 Ab4:2 G4:4 G4:2 F4:4",
            "I | I | I V7 | I | I | I | I V7 | I7",
        ),
        // The second strain, with two two-bar breaks.
        Part::new(
            "C",
            "Eb4:12 F4:4 | G4:4 Bb4:2 C5:4 C5:2 Bb4:4 | Eb5:4 -:12 | -:16 | \
             Eb4:12 F4:4 | G4:4 Bb4:2 C5:4 C5:2 Bb4:4 | D5:4 -:12 | -:16",
            "I | I | I | I | I | I | V7 | V7",
        )
        .in_key(DSharp, Major),
        Part::new(
            "D",
            "Bb4:4 D5 C5:8 | Bb4:2 D5:4 C5:10 | Bb4:4 Eb5 C5:8 | Bb4:2 Eb5:4 C5:10 | \
             Bb4:4 D5 C5:8 | Bb4:2 D5:4 C5:4 Bb4:2 C5:4 | Eb5:4 -:12 | -:16",
            "V7 | V7 | I | I | V7 | V7 | I | I",
        )
        .in_key(DSharp, Major),
        // The third strain's second ending turns E flat into E flat 7, the
        // V7 of A flat.
        Part::new(
            "D2",
            "Bb4:4 D5 C5:8 | Bb4:2 D5:4 C5:10 | Bb4:4 Eb5 C5:8 | Bb4:2 Eb5:4 C5:10 | \
             C5:8 G4:8 | Ab4:8 A4:8 | Bb4:2 A4 Bb4 C5 Bb4 Ab4 G4 F4 | Eb4:8 -:8",
            "V7 | V7 | I | I | IV | IV #iv°7 | V7 | I I7",
        )
        .in_key(DSharp, Major),
        // "Hold that tiger": six bars of A flat, eight of E flat 7.
        Part::new(
            "Tiger",
            "-:4 Ab4:8 Ab4:4 | Ab4:8 C5:2 -:6 | -:4 Ab4:8 Ab4:4 | Ab4:8 C5:2 -:6 | \
             -:4 Ab4:8 Ab4:4 | Ab4:8 C5:2 -:6 | -:4 Ab4:8 Ab4:4 | Ab4:8 Eb4:2 -:6 | \
             -:4 G4:8 G4:4 | Ab4:8 Bb4:2 -:6 | -:4 G4:8 G4:4 | Ab4:8 Bb4:2 -:6 | \
             -:4 G4:8 G4:4 | Ab4:8 Bb4:2 -:6 | Ab4:4 C4 Db4 D4 | Eb4:2 Eb4 E4:4 F4 G4",
            "I | I | I | I | I | I | V7 | V7 | V7 | V7 | V7 | V7 | V7 | V7 | I | V7",
        )
        .in_key(GSharp, Major),
        Part::new(
            "Tiger 2",
            "-:4 Ab4:8 Ab4:4 | Ab4:8 C5:2 -:6 | -:4 Ab4:8 Ab4:4 | Ab4:8 C5:2 -:6 | \
             Ab4:16 | _:4 Ab4:2 Bb4:4 Ab4:2 Bb4:4 | Ab4:16 | _:4 -:12 | \
             -:4 B4:8 B4:4 | B4:8 Ab4:2 -:6 | -:4 C5:8 C5:4 | C5:8 A4:2 -:6 | \
             -:4 C5 Bb4 Ab4 | Bb4:2 C5:4 Eb5:2 _:4 B4 | Ab4:16 | _:4 -:12",
            "I | I | I | I | I7 | I7 | IV | IV | #iv°7 | #iv°7 | I | VI7 | II7 | V7 V7 V7 V+ | \
             I | I",
        )
        .in_key(GSharp, Major),
        Part::new(
            "Tiger end",
            "-:4 Ab4:8 Ab4:4 | Ab4:8 C5:2 -:6 | -:4 Ab4:8 Ab4:4 | Ab4:8 C5:2 -:6 | \
             Ab4:16 | _:4 Ab4:2 Bb4:4 Ab4:2 Bb4:4 | Ab4:16 | _:4 -:12 | \
             -:4 B4:8 B4:4 | B4:8 Ab4:2 -:6 | -:4 C5:8 C5:4 | C5:8 A4:2 -:6 | \
             -:4 C5 Bb4 Ab4 | Bb4:2 C5:4 Eb5:2 _:4 B4 | Ab4:16 | _:8 -:8",
            "I | I | I | I | I7 | I7 | IV | IV | #iv°7 | #iv°7 | I | VI7 | II7 | V7 V7 V7 V+ | \
             I | I",
        )
        .in_key(GSharp, Major),
    ],
    form: &[
        "A",
        "A2",
        "B",
        "A3",
        "C",
        "D",
        "D2",
        "Tiger",
        "Tiger 2",
        "Tiger",
        "Tiger end",
    ],
    drums_from: 0,
    about: "Four strains, and the key drops a fifth twice: the first strain is in B flat, \
            the next two in E flat, and the \"Hold that tiger\" strain in A flat, each new key \
            reached through a dominant seventh on the old tonic, B flat 7, then E flat 7. The \
            second strain stops dead for two-bar breaks, left for a soloist, and the last \
            strain holds A flat for six bars under its repeated call.",
};

// Public domain in the US (published in 1918), still under copyright in
// countries where it lasts 70 years after the authors' deaths (Turner Layton died in 1978).
// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Henry_Creamer-Turner_Layton/After_You%27ve_Gone/After_You%27ve_Gone.mxl
// (OpenEWLD lead sheet, the refrain, in B flat: twenty bars of eighth notes,
// as in John Chambers's ABC of the 1918 Broadway Music edition,
// https://raw.githubusercontent.com/syntithenai/abc2book/HEAD/abcresources/jc/abc_tune_jc_201245271.abc;
// the last B flat is tied over, as there. Chords from the lead sheet,
// checked against that ABC and the iReal changes.)
pub(super) const AFTER_YOUVE_GONE: Song = Song {
    name: "After You've Gone",
    origin: "Turner Layton, words by Henry Creamer, 1918",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 168.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "Backdoor ii–V",
    parts: &[
        Part::new(
            "Chorus",
            "G4:2 Bb4 C5 D5:10 | -:2 C5:2 D5 C5 D5 C5:6 | D4:2 F4 G4 A4:10 | \
             -:2 G4:2 A4 G4 A4 G4:6 | G4:2 D5:4 D5:10 | G4:2 C5:4 C5:10 | \
             -:2 F4:2 G4 F4 A4 F4 G4 F4 | Bb4:2 D4:4 F4:10 | \
             G4:2 Bb4 C5 D5:10 | -:2 C5:2 D5 C5 D5 C5:6 | D4:2 F4 G4 A4:10 | \
             -:2 G4:2 A4 G4 A4 G4:6",
            "IVmaj7 | iv6 | Imaj7 | VI9 | II9 | V9 | I | I I7 | IVmaj7 | bVII9 | Imaj7 | VI7",
        ),
        // The last eight bars, with the first ending: back to the top.
        Part::new(
            "Chorus 2",
            "Eb5:8 D5 | C5:2 F#4 G4 D5:4 C5:6 | -:2 C#5:2 D5 Bb4 C5 B4 C5 A4 | \
             Bb4:2 C#4 D4 A4:4 G4:6 | F4:2 G4 Bb4 D5:10 | -:2 Eb4:2 F4 A4 C5:4 D5 | \
             D5:2 Bb4:14 | _:2 -:14",
            "ii VI7 | ii7 bVII9 | I III7 | vi7 VII7 | I vi7 | ii7 V7 | I I I IV | I I V7 I7",
        ),
        // The last eight bars with the second ending, the last time.
        Part::new(
            "Chorus end",
            "Eb5:8 D5 | C5:2 F#4 G4 D5:4 C5:6 | -:2 C#5:2 D5 Bb4 C5 B4 C5 A4 | \
             Bb4:2 C#4 D4 A4:4 G4:6 | F4:2 G4 Bb4 D5:10 | -:2 Eb4:2 F4 A4 C5:4 D5 | \
             D5:2 Bb4:14 | _:12 -:4",
            "ii VI7 | ii7 bVII9 | I III7 | vi7 VII7 | I vi7 | ii7 V7 | I I I IV | I",
        ),
    ],
    form: &["Chorus", "Chorus 2", "Chorus", "Chorus end"],
    drums_from: 0,
    about: "A twenty-bar chorus that starts away from home, on E flat major 7 (IV), turns \
            it minor and slides back to B flat: the back door home, for E flat minor 6 holds \
            the same notes as A flat 9 without its root, and the sheet writes it both ways. \
            The tune climbs four notes, G B flat C D, then the same shape a fourth lower, \
            D F G A.",
};

// Source: https://raw.githubusercontent.com/baobach/abc_search/master/database/abc/QmZic6BMm4v8ndW9uFmei9FX55g8M4GChcZVpxpAv8NrkF.abc
// (an ABC transcription of the piano score, "By Ferd 'Jelly Roll' Morton": the top line of the
// right hand, every strain in the order of the score; the chords read from both hands. The
// opening call, written two octaves above its echo, is played in the echo's register; in the
// interlude the right hand's octaves are taken at their lower note and its 8va run two octaves
// down; in the last strain the octaves are taken at their lower note. The score's tempo is
// half note = 80.)
pub(super) const KING_PORTER_STOMP: Song = Song {
    name: "King Porter Stomp",
    origin: "Jelly Roll Morton, 1924",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: GSharp,
    mode: Major,
    time: COMMON,
    tempo: 192.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "New Orleans",
    parts: &[
        Part::new(
            "Intro",
            "C5:1 Db5:2 C5:1 B4:3 C5:1 G5:2 F5:6 | C5:1 Db5:2 C5:1 B4:3 C5:1 G5:2 F5:6 | \
             G5:3 Bb5:1 Gb5:3 Bb5:1 F5:3 Bb5:1 Fb5:3 Bb5:1 | Eb5:4 -:4 Eb6:4 -:4",
            "I i°7 | I i°7 | V7 bVII7 ii bVII7 | V7",
        ),
        Part::new(
            "A",
            "Eb4:4 Eb4:2 F4 Eb4 F4 Eb4:4 | Eb4:4 Eb4:2 F4 Eb4 F4 Eb4:4 | \
             -:2 Bb4 A4 Bb4 G5:6 F5:2 | A4:2 Bb4:4 G5:2 _:2 F5 Db5 Bb4 | \
             -:2 G4 F#4 G4 F5 Eb5:6 | F#4:2 G4:6 F5:2 Eb5:6 | \
             -:2 C5 B4 C5 G5:4 F5 | Eb5:2 D5 Db5 A4 C5 Bb4 G4 F4 | \
             Eb4:4 Eb4:2 F4 Eb4 F4 Eb4:4 | Eb4:4 Eb4:2 F4 Eb4 F4 Eb4:4 | \
             -:2 Bb4 A4 Bb4 G5:6 F5:2 | A4:2 Bb4:4 G5:2 _:2 F5 Db5 Bb4 | \
             -:2 Bb4 A4 Bb4 G5:6 F5:2 | F5:2 Eb5 C5 Ab4 Db5:4 C5 | \
             C5:2 Ab4 Bb4 C5 _ Ab4 Bb4:4",
            "I I I bVII6 | VI7 | ii | ii | V7 | V7 | I I I #ii°7 | V7 | I I I bVII6 | VI7 | ii | \
             ii | IV VII7 | I I VI+ VI7 | II9 II9 V7sus4 V7",
        ),
        // The two endings of the first strain: back to its start, then on to B.
        Part::new("A1", "Ab4:4 F5:2 Eb5 F5 Eb5 -:4", "I #i°7 V7 V7"),
        Part::new("A2", "Ab4:4 D5:2 Eb5 E5 F5 F#5 G5", "I"),
        Part::new(
            "B",
            "-:2 F5 Ab5 C5 F5 Ab5 _:4 | -:2 E5 G5 C5 E5 G5 _:4 | -:2 C5 F5 A4 C5 F5 _:4 | \
             -:2 Db5 F5 Bb4 Db5 F5 _:4 | -:2 C5 Ab4 F5 _ Eb5 C5 Ab4 | -:2 G5 G5 G5 _:4 Eb5:2 F5 | \
             Eb5:2 -:14 | -:4 Eb5:2 F5 G5 Ab5 Bb5 B5 | \
             -:2 Ab5 C6 F5 Ab5 C6 _:4 | -:2 G5 Bb5 E5 G5 Bb5 _:4 | -:2 F5 Ab5 C5 F5 Ab5 _:4 | \
             Bb5:6 Ab5:2 Bb5 Ab5 G5 F5 | -:2 Eb5 F5 Eb5 _ F5 Eb5:4 | -:2 Eb5 F5 Eb5 _ F5 Eb5:4 | \
             Ab5:4 -:12 | -:2 Eb5 F5 B4 C5:4 -:4",
            "vi | III7 | VI7 | ii | I | II7 | V7 | V7 | vi | III7 | vi | IV6 II7 | I | V7 | I | \
             I VII7 I I",
        ),
        // In octaves, both hands together; it turns to A flat 7, the V7 of D flat.
        Part::new(
            "Interlude",
            "Ab4:2 G4 Ab4 Bb4 C5 G4 Ab4 Bb4 | C5:2 B4 C5 Db5 Eb5:4 -:4 | \
             Ab5:4 G5:2 Gb5 _ F5 Eb5 Db5 | C5:4 Bb4:2 Ab4 _:4 Ab3:4",
            "I | I V7 | I7 | I7",
        ),
        Part::new(
            "Trio",
            "Ab5:4 Gb5 Ab5 Gb5 | E5:2 F5:4 Db5:2 _:4 Ab5:4 | Ab5:4 Gb5 Ab5 Gb5 | \
             E5:2 F5:4 Db5:2 _:4 Ab5:4 | Ab5:4 Gb5 Ab5 Gb5 | E5:2 F5:4 Bb5:2 _:8 | \
             -:4 C5:2 Bb4 C5 Eb5 C5:4 | Eb5:2 C5:4 Ab4:2 _:4 Ab5:4 | \
             Ab5:4 Gb5 Ab5 Gb5 | E5:2 F5:4 Db5:2 _:4 Ab5:4 | Ab5:4 Gb5 Ab5 Gb5 | \
             E5:2 F5:4 Db5:2 _:4 Ab5:4 | Ab5:4 Gb5 Ab5 Gb5 | E5:2 F5:4 Bb5:2 _:8 | \
             -:2 E5 F5 E5 Db5 E5:6 | Db5:4 -:12",
            "IV iv | I I I I7 | IV iv | I I I I7 | IV iv | I I VI7 VI7 | II7 | II7 II7 V7 I7 | \
             IV iv | I I I I7 | IV iv | I I I I7 | IV iv | I I VI7 VI7 | II7 II7 V7sus4 V7 | I",
        )
        .in_key(CSharp, Major),
        // The right hand strikes chords on the beats over the striding left hand; the tune
        // is their top note.
        Part::new(
            "Stomp",
            "Db5:4 -:4 Db5:4 -:4 | Db5:4 -:2 Db5:2 _:4 Db5:4 | Db5:4 -:4 Db5:4 -:4 | \
             Db5:4 -:2 Db5:2 _:4 Db5:4 | Db5:4 -:4 Db5:4 -:4 | Db5:4 -:2 Db5:2 _:4 Db5:4 | \
             -:4 C5:2 Bb4 C5 Eb5 C5:4 | Eb5:2 C5:4 Ab4:2 _:4 Ab4:4 | \
             Db5:4 -:4 Db5:4 -:4 | Db5:4 -:2 Db5:2 _:4 Db5:4 | Db5:4 -:4 Db5:4 -:4 | \
             Db5:4 -:2 Db5:2 _:4 Db5:4 | Db5:4 -:4 Db5:4 -:4 | Db5:4 -:2 Db5:2 _:4 Db5:4 | \
             -:2 E5 F5 E5 Db5 E5:6 | Db5:4 E5:2 E5 _ Db5 Eb5:4",
            "IV #iv°7 | I7 I+ | IV #iv°7 | I7 I+ | IV #iv°7 | I I vi7 I | II7 | II7 II7 V7 I7 | \
             IV #iv°7 | I7 I+ | IV #iv°7 | I7 I+ | IV #iv°7 | I I vi7 I | II7 II7 V7sus4 V7 | I",
        )
        .in_key(CSharp, Major),
        // The same chords under a riff on E, D flat and E flat; the left hand plays only
        // bass notes in its last two bars, and the chords there are read from them.
        Part::new(
            "Stomp 2",
            "Db5:4 -:12 | -:4 E5:2 E5 _ Db5 Eb5:4 | Db5:4 -:12 | -:4 E5:2 E5 _ Db5 Eb5:4 | \
             Db5:4 -:12 | E5:2 F5:4 Bb4:2 _:8 | -:4 C5:2 Bb4 C5 Eb5 C5:4 | \
             Eb5:2 C5:4 Ab4:2 _:4 E5:4 | E5:4 -:4 Db5:3 Eb5:1 Db5:3 Eb5:1 | Db5:4 -:8 E5:4 | \
             E5:4 -:4 Db5:3 Eb5:1 Db5:3 Eb5:1 | Db5:4 -:8 E5:4 | E5:4 -:4 Db5:3 Eb5:1 Db5:3 Eb5:1 | \
             E5:2 Db5 Eb5 Db5 _:4 E5:2 Db5 | Eb5:4 Db5 E5:2 Db5 Eb5:4 | Db5:4 E5:2 Db5 Eb5:4 Db5",
            "IV #iv°7 | I7 | IV #iv°7 | I7 | IV #iv°7 | I I VI7 VI7 | II7 | II7 II7 V7 I7 | \
             IV #iv°7 | I7 | IV #iv°7 | I7 | IV #iv°7 | I I vi°7 vi°7 | II7 IV7 | I vi°7 #v°7 V7",
        )
        .in_key(CSharp, Major),
        Part::new(
            "Coda",
            "E5:2 Db5 Eb5:4 Db5 Ab5:2 Bb5 | Eb6:12 -:4",
            "#iv°7 IV7 I V7 | I9",
        )
        .in_key(CSharp, Major)
        .with_tempo(&[fermata(16, 12)]),
    ],
    form: &[
        "Intro", "A", "A1", "A", "A2", "B", "Interlude", "Trio", "Stomp", "Stomp 2", "Coda",
    ],
    drums_from: 0,
    about: "Jelly Roll Morton's piano stomp, which the bands of Fletcher Henderson and Benny \
            Goodman made an anthem of the swing era. The first strain rocks between E flat and \
            F over A flat; the trio moves to D flat, where the bass slides down B flat, A, A \
            flat under G flat, G flat minor and D flat (IV, iv, I). The last strains are the \
            stomp: first chords struck on D flat over a striding bass, then a riff on E, D flat \
            and E flat, the blue third rubbing against the tonic.",
};

// Source: https://raw.githubusercontent.com/neonscribe/lilypond-lead-sheets/master/Core/Indiana%20-%20Ly%20Core%20-%20F.ly
// (a lead sheet of the refrain after Lester Young's 1952 recording. Its pitches were checked
// against the ABC at https://tunearch.org/wiki/Back_Home_Again_in_Indiana, in G, and the Red
// Jackets jazz band's ABC, https://raw.githubusercontent.com/red-jackets-jazzband/website/master/static/songs/indiana.abc:
// they agree but for a passing note or two, and in bar 10 both have D flat for this lead
// sheet's D. Chords: the iReal changes (standards.py), but bar 10 (B flat, B diminished) and
// bar 29 (A minor 7, D minor 7) from the lead sheet, where the iReal chords clash with its
// melody.)
pub(super) const INDIANA: Song = Song {
    name: "Indiana (Back Home Again in Indiana)",
    origin: "James F. Hanley, words by Ballard MacDonald, 1917",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: F,
    mode: Major,
    time: COMMON,
    tempo: 200.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "Turnaround",
    parts: &[
        Part::new("Pickup", "-:2 C4:6 F4:4 A4", "V7"),
        Part::new(
            "A",
            "C5:8 -:8 | -:2 D5:6 C5:4 A4 | G4:6 D4:2 _:8 | -:8 E4:4 F4 | \
             A4:12 E4:4 | A4:8 G4:6 F4:2 | _:16 | -:2 F4:6 E4:4 Eb4",
            "Imaj7 bVII7 | VI7 | II7 | II7 | ii7 | V7 | Imaj7 | v7 I7",
        ),
        Part::new(
            "B",
            "D4:4 F4:2 Bb4 _:8 | _:8 D4:8 | C4:4 F4:2 A4 _:8 | _:8 A4:4 Bb4 | \
             B4:4 A4 E4:8 | _:8 D4:6 G4:2 | _:16 | -:2 C4:6 F4:4 A4",
            "IVmaj7 | IV #iv°7 | Imaj7 | VI7 | II7 | II7 | ii7 | V7",
        ),
        Part::new(
            "A2",
            "C5:8 -:8 | -:2 D5:6 C5:4 A4 | G4:6 D4:2 _:8 | -:8 E4:4 F4 | \
             A4:12 E4:4 | A4:8 G4:6 F4:2 | _:16 | -:8 F4:4 G4",
            "Imaj7 bVII7 | VI7 | II7 | II7 | viiø7 | III7b9 | vi | viiø7 III7b9",
        ),
        // The last eight, with the pickup back to the top of the chorus.
        Part::new(
            "C",
            "A4:4 G#4 A4:6 F4:2 | G4:4 A4 G4 E4 | F4:6 D4:2 _:8 | _:4 -:4 E4 D4 | \
             C4:8 F4:4 A4 | C5:4 G4 Bb4:2 A4:4 F4:2 | _:8 -:8 | -:2 C4:6 F4:4 A4",
            "vi | viiø7 III7 | vi vi7 | #iv°7 | iii7 vi7 | ii7 V7 | I6 #i°7 | ii7 V7",
        ),
        Part::new(
            "C end",
            "A4:4 G#4 A4:6 F4:2 | G4:4 A4 G4 E4 | F4:6 D4:2 _:8 | _:4 -:4 E4 D4 | \
             C4:8 F4:4 A4 | C5:4 G4 Bb4:2 A4:4 F4:2 | _:8 -:8 | -:16",
            "vi | viiø7 III7 | vi vi7 | #iv°7 | iii7 vi7 | ii7 V7 | I6 | I6",
        ),
    ],
    form: &["Pickup", "A", "B", "A2", "C", "A", "B", "A2", "C end"],
    drums_from: 1,
    about: "A 1917 song that jazz bands made their own. The first bar already leaves F for E \
            flat 7 (bVII7), and then the chords fall down a chain of dominants, D7, G7, C7, each \
            the V of the next, until F returns in bar seven. The second half turns toward D \
            minor (vi) through its own ii-V, E half-diminished and A7.",
};

// Source: https://raw.githubusercontent.com/neonscribe/lilypond-lead-sheets/master/Core/Rose%20Room%20-%20Ly%20Core%20-%20Ab.ly
// (a lead sheet of the refrain after Benny Goodman's 1939 recording, with its chords; they
// agree with the iReal changes in standards.py)
pub(super) const ROSE_ROOM: Song = Song {
    name: "Rose Room",
    origin: "Art Hickman, words by Harry Williams, 1917",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: GSharp,
    mode: Major,
    time: COMMON,
    tempo: 146.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "Backdoor ii–V",
    parts: &[
        Part::new("Pickup", "-:4 Eb4:4 Eb4 E4", "V7"),
        Part::new(
            "A",
            "F4:6 C5:2 _:8 | -:2 Eb4 F4 F#4 G4 C5:4 G4:2 | Bb4:16 | Ab4 | \
             Ab4 | -:2 Ab4 Bb4 B4 C5 F5:4 C5:2 | Eb5:16 | Db5",
            "II7 | V7 | Imaj7 | I6 | v7 | I7 | IVmaj7 | IV6",
        ),
        Part::new(
            "B",
            "Db5:16 | _:4 Ab4:2 Bb4 Db5 Ab4 Bb4:4 | C5:16 | Eb5 | \
             F4:6 C5:2 _:8 | _:4 F4:2 G4 Ab4 Bb4 C5:4 | Bb4:16 | _:4 Eb4 Eb4 E4",
            "iv7 | bVII7 | Imaj7 | VI7 | II7 | II7 | ii7 | V7",
        ),
        // The last eight, with the pickup back to the top of the chorus.
        Part::new(
            "C",
            "Db5:6 Db5:2 _:8 | _:4 Ab4:2 Bb4 Db5 Ab4 Bb4:4 | C5:16 | Eb5:4 Eb4 Eb4 E4 | \
             F4:6 C5:2 _:8 | -:2 Eb4 F4 F#4 G4 Eb5 C5:4 | Ab4:16 | -:4 Eb4 Eb4 E4",
            "iv7 | bVII7 | Imaj7 | VI7 | II7 | V7 | I6 VII7b9 | bVII7 VI7",
        ),
        Part::new(
            "C end",
            "Db5:6 Db5:2 _:8 | _:4 Ab4:2 Bb4 Db5 Ab4 Bb4:4 | C5:16 | Eb5:4 Eb4 Eb4 E4 | \
             F4:6 C5:2 _:8 | -:2 Eb4 F4 F#4 G4 Eb5 C5:4 | Ab4:16 | -:16",
            "iv7 | bVII7 | Imaj7 | VI7 | II7 | V7 | I6 | I6",
        ),
    ],
    form: &["Pickup", "A", "B", "A", "C", "A", "B", "A", "C end"],
    drums_from: 1,
    about: "Art Hickman's 1917 hit, whose chords Duke Ellington later used for In a Mellotone. \
            It starts away from home, on B flat 7 and E flat 7 (II7, V7), and reaches A flat \
            only in the third bar; the middle comes home by the back door, D flat minor 7 to G \
            flat 7 (iv7, bVII7) to A flat. In the second bar the tune climbs by half steps, E \
            flat, F, F sharp, G.",
};

// Source: https://raw.githubusercontent.com/flaminghakama/lessons/master/ly/songs/jazz/charleston.ly
// (a lead sheet of the chorus, with its chords and first and second endings. Chords: the iReal
// changes (standards.py), except bars 12 and 15 (G minor, D minor) from the lead sheet, where
// the iReal chords, E diminished and D major, clash with its melody.)
pub(super) const CHARLESTON: Song = Song {
    name: "Charleston",
    origin: "James P. Johnson, Runnin' Wild, 1923",
    feel: "Two-beat",
    genre: Genre::Jazz,
    tonic: ASharp,
    mode: Major,
    time: COMMON,
    tempo: 208.0,
    comping: Comping::Charleston,
    bass: Bass::RootFifth,
    rhythm: "Swing",
    band: DIXIELAND,
    style: "Turnaround",
    parts: &[
        Part::new(
            "A",
            "F4:4 -:2 F4:2 _:8 | F#4:4 -:2 F#4:2 _:8 | G4:4 -:2 G4:2 _:2 F#4:2 G4:4 | \
             D5:4 -:2 G4:2 _:8 | G4:4 -:2 D5:2 _:8 | G4:4 -:2 D5:2 _:8 | \
             -:2 G4:4 -:2 Bb4:8 | A4:2 C5 A4 G4 _ A4 G4 Gb4",
            "I | III7 | VI7 | VI7 | II7 | V7 | I biii°7 | ii7 V7",
        ),
        Part::new(
            "B",
            "F4:4 -:2 F4:2 _:8 | F#4:4 -:2 F#4:2 _:8 | G4:4 -:2 G4:2 _:2 F#4:2 G4:4 | \
             D5:4 -:2 D5:2 _:8 | -:2 A4 A4 A4 _ G4 F4:4 | -:2 G4 G4 G4 _ F4 E4:4 | \
             -:2 A4 A4 A4 _ G4 F4:4 | -:2 A4 A4 A4 _ G4 F4:4",
            "I | III7 | VI7 | vi | iii7 | VII7 | iii | V7",
        ),
        Part::new(
            "A3",
            "F4:4 -:2 F4:2 _:8 | F#4:4 -:2 F#4:2 _:8 | G4:4 -:2 G4:2 _:2 F#4:2 G4:4 | \
             D5:4 -:2 G4:2 _:8 | G4:4 -:2 D5:2 _:8 | G4:4 -:2 D5:2 _:8 | \
             -:2 G4:4 -:2 Bb4:8 | A4:2 C5 A4 G4 _ F4:6",
            "I | III7 | VI7 | VI7 | II7 | V7 | I biii°7 | ii7 V7",
        ),
        // The last eight with the first ending, back to the top.
        Part::new(
            "C",
            "Bb4:4 -:2 Bb4:2 _:8 | -:4 C5:4 _:2 Bb4:2 C5:4 | Bb4:4 -:2 Bb4:2 _:8 | \
             -:4 Bb4:2 C5 _ Bb4 C5:4 | G4:4 -:2 G4:2 _:8 | -:2 F4 G4 A4 _ G4 F4:4 | \
             Bb4:16 | A4:2 C5:4 A4:2 G4:4 Gb4",
            "I7 | I7 | IV7 | iv7 | I vi7 | ii7 V7 | I | ii7 V7",
        ),
        Part::new(
            "C end",
            "Bb4:4 -:2 Bb4:2 _:8 | -:4 C5:4 _:2 Bb4:2 C5:4 | Bb4:4 -:2 Bb4:2 _:8 | \
             -:4 Bb4:2 C5 _ Bb4 C5:4 | G4:4 -:2 G4:2 _:8 | -:2 F4 G4 A4 _ G4 F4:4 | \
             Bb4:16 | _:12 -:4",
            "I7 | I7 | IV7 | iv7 | I vi7 | ii7 V7 | I | I",
        ),
    ],
    form: &["A", "B", "A3", "C", "A", "B", "A3", "C end"],
    drums_from: 0,
    about: "The song that set off the dance craze of the 1920s. Its rhythm, a note on beat \
            one and another just before beat three, held over, is in almost every bar of the \
            tune, and the jazz comping pattern is named after it. The chords are a ragtime \
            chain of dominants, D7, G7, C7, F7 (III7, VI7, II7, V7), while the tune climbs by \
            half steps, F, F sharp, G.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Billy_Rose-Edward_Eliscu-Vincent_Youmans/Great_Day/Great_Day.mxl
// (OpenEWLD lead sheet: the refrain, in 2/2, with its chords; not in the iReal changes)
pub(super) const GREAT_DAY: Song = Song {
    name: "Great Day",
    origin: "Vincent Youmans, Great Day!, 1929",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 184.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: BIG_BAND,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "G4:8 F4:4 G4 | _:4 F4 G4:8 | F4:4 Eb4 F4 G4 | _:4 F4 G4:8 | \
             F4:4 Eb4 F4 G4 | _:4 Eb4 F4:8 | Eb4:16 | _:16",
            "I | I | I | I | IV IV IV I | I V7 | I | I",
        ),
        Part::new(
            "A2",
            "Bb4:8 Ab4:4 Bb4 | _:4 Ab4 Bb4:8 | Ab4:4 G4 Ab4 Bb4 | _:4 Ab4 Bb4:8 | \
             Ab4:4 G4 Ab4 Bb4 | _:4 G4 Ab4:8 | G4:16 | _:16",
            "I | I | I | I | ii7 ii7 ii7 I | I V7 | I | I",
        ),
        Part::new(
            "B",
            "Ab4:8 Bb4 | C5:8 D5 | Eb5:12 Eb4:4 | _:4 G4 Bb4 Eb5 | \
             D5:12 Eb4:4 | _:4 F4 A4 D5 | C5:12 C4:4 | _:4 D4 Eb4 F4",
            "IV I7 | IV V7 | I | vi7 | V | II7 | V7sus4 | V7",
        ),
    ],
    form: &["A", "A2", "B", "A", "A", "A2", "B", "A"],
    drums_from: 0,
    about: "The title song of Vincent Youmans's 1929 show. The first phrase rocks between G \
            and F over a held E flat chord, and the second says the same a third higher, B \
            flat and A flat. The bridge climbs the scale, A flat, B flat, C, D, E flat, then \
            stops on B flat (V) with its own dominant, F7 (II7), and leads back home.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Henry_Creamer-James_P._Johnson/If_I_Could_Be_with_You_(One_Hour_Tonight)/If_I_Could_Be_with_You_(One_Hour_Tonight).mxl
// (OpenEWLD lead sheet: the verse and the refrain with its two endings, in 2/2; here in E flat,
// up a minor third from its C, the key of the iReal chart. Chords: the iReal changes
// (standards.py), which fit the melody where the lead sheet's are misplaced; the passing
// diminished chord of the first ending and the minor iv of the second, written on the same
// beat as the tonic, are left out.)
pub(super) const IF_I_COULD_BE_WITH_YOU: Song = Song {
    name: "If I Could Be with You (One Hour Tonight)",
    origin: "James P. Johnson, words by Henry Creamer, 1926",
    feel: "Ballad",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: Major,
    time: COMMON,
    tempo: 90.0,
    comping: Comping::Ballad,
    bass: Bass::Walking,
    rhythm: "Jazz ballad",
    band: BALLAD_BAND,
    style: "Jazz ballad",
    parts: &[
        Part::new(
            "Verse",
            "G4:4 Bb4 Bb4:6 G4:2 | Ab4:3 Bb4:1 Ab4:3 G4:1 F4:8 | G4:4 Bb4 Bb4:6 G4:2 | \
             Ab4:3 Bb4:1 Ab4:3 G4:1 F4:8 | Eb4:4 F4 G4 Bb4:3 B4:1 | \
             C5:3 D5:1 C5:3 Bb4:1 G4:6 Gb4:2 | F4:3 G4:1 A4:3 Bb4:1 C5:3 D5:1 C5:3 F4:1 | \
             Bb4:12 Bb4:3 C5:1 | D5:3 C#5:1 D5:3 Bb4:1 C5:2 F4:4 Bb4:2 | \
             _:11 G4:1 G4:3 Gb4:1 | F4:3 G4:1 A4:3 Bb4:1 C5:3 D5:1 C5:3 F4:1 | \
             Bb4:8 _:2 -:1 D5 Eb5:3 E5:1",
            "Imaj7 #i°7 | ii7 V7 | Imaj7 #i°7 | ii7 V7 | I6 | I6 VI7 | II7 | V7 | \
             V III7 VI7 II7 | V6 | II7 | V7",
        ),
        Part::new(
            "A",
            "F5:6 D5:2 Eb5:6 C5:2 | D5:2 B4:4 C5:2 _:3 B4:1 C5:3 D5:1 | \
             Eb5:6 C5:2 D5:6 B4:2 | C5:2 G4:4 Ab4:2 _:4 G4 | C5:2 G4 F#4 G4 _:8 | \
             Bb4:2 G4 F#4 G4 _:6 F4:2 | Eb4:3 F4:1 G4:3 Ab4:1 Bb4:3 C5:1 D5:3 C5:1 | \
             G4:2 Gb4:4 F4:2 _:3 D5:1 Eb5:3 E5:1",
            "ii7 | V7 | I6 III7 | VI7 | II7 | V7 | I6 VI7 | II7 V7",
        ),
        // The second half with the first ending, back to the top of the refrain.
        Part::new(
            "B",
            "F5:6 D5:2 Eb5:6 C5:2 | D5:2 B4:4 C5:2 _:3 B4:1 C5:3 D5:1 | \
             Eb5:6 C5:2 D5:6 B4:2 | C5:2 G4:4 Ab4:2 _:4 G4 | F4:2 E4 F4 D5 _:4 C5:3 D5:1 | \
             Eb5:3 F#4:1 G4:3 D5:1 C5:6 Bb4:2 | C5:6 G4:2 Bb4:6 G4:2 | Eb4:8 -:3 D5:1 Eb5:3 E5:1",
            "ii7 | V7 | I6 III7 | VI7 | IV6 #iv°7 | I6 VI7 | II7 V7 | I6 V7",
        ),
        Part::new(
            "B end",
            "F5:6 D5:2 Eb5:6 C5:2 | D5:2 B4:4 C5:2 _:3 B4:1 C5:3 D5:1 | \
             Eb5:6 C5:2 D5:6 B4:2 | C5:2 G4:4 Ab4:2 _:4 G4 | F4:2 E4 F4 D5 _:4 C5:3 D5:1 | \
             Eb5:3 F#4:1 G4:3 D5:1 C5:6 Bb4:2 | C5:6 G4:2 Bb4:6 G4:2 | Eb4:12 -:4",
            "ii7 | V7 | I6 III7 | VI7 | IV6 #iv°7 | I6 VI7 | II7 V7 | I6",
        ),
    ],
    form: &["Verse", "A", "B", "A", "B end"],
    drums_from: 0,
    about: "A slow-swinging song by the stride pianist James P. Johnson, recorded by Louis \
            Armstrong in 1930. The verse climbs from E flat through E diminished to F minor (I, \
            #i°7, ii7); the refrain starts on F minor 7 and runs a chain of dominants, G7, C7, \
            F7, B flat 7, each the V of the next, back to E flat. The tune leans on half-step \
            neighbors: B natural to C, F sharp to G.",
};

// Source: https://raw.githubusercontent.com/00sapo/OpenEWLD/master/dataset/Gus_Kahn-Walter_Donaldson/Carolina_in_the_Morning/Carolina_in_the_Morning.mxl
// (OpenEWLD lead sheet: the chorus with its two endings, and its chords; not in the iReal
// changes. Its "dim" chords are read as diminished sevenths: the melody's A over C dim is that
// chord's seventh. In bar 14 the D minor 6 and E7, written together, are put on beats 2 and 3,
// and the passing G diminished of bar 16, written on the same beat as G7, is left out.)
pub(super) const CAROLINA_IN_THE_MORNING: Song = Song {
    name: "Carolina in the Morning",
    origin: "Walter Donaldson, words by Gus Kahn, 1922",
    feel: "Big band",
    genre: Genre::Jazz,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 152.0,
    comping: Comping::Charleston,
    bass: Bass::Walking,
    rhythm: "Swing",
    band: SAX_SECTION,
    style: "ii–V–I",
    parts: &[
        Part::new(
            "A",
            "G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 | G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 | \
             G4:8 _:2 F#4 G4:4 | F4:16 | \
             A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 | A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 | \
             A4:8 _:2 G#4 A4:4 | G4:16",
            "I | I v°7 | V7 | V7 | ii | ii V7 | i°7 | I",
        ),
        Part::new(
            "B",
            "A4:2 B4 C5:8 A4:4 | G4 E4:12 | A4:2 B4 C5:8 A4:4 | G4:16 | \
             F#4:2 G4 G#4 A4 _:4 A#4 | B4 D5:12 | C5:2 D5 C5 B4 _:4 B4 | G4:16",
            "IV | I | IV | VI7 | II7 | V ii6 III7 III7 | vi II7 | V7",
        ),
        Part::new(
            "A2",
            "G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 | G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 G4:3 E4:1 | \
             G4:8 _:2 F#4 G4:4 | F4:16 | \
             A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 | A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 A4:3 F4:1 | \
             A4:8 _:2 G#4 A4:4 | G4:16",
            "I | I v°7 | V7 | V7 | ii | ii V7 | #iv°7 | V7",
        ),
        // The last eight with the first ending, back to the top.
        Part::new(
            "C",
            "C5:3 G4:1 A4:3 G4:1 C5:3 G4:1 A4:3 G4:1 | C5:3 A4:1 G4:2 C4 _:8 | \
             C5:8 A4:3 G#4:1 A4:3 B4:1 | C5:3 A4:1 C5:2 B4 _:8 | \
             G4:3 E4:1 G4:3 E4:1 A4:3 B4:1 A4:3 Ab4:1 | G4:3 E4:1 G4:3 E4:1 A4:3 B4:1 C5:3 A4:1 | \
             D5:8 _:2 C#5 D5:4 | C5:4 -:12",
            "I | I I7 | IV | II7 V7 | I | I | II7 II7 II7 V7 | I",
        ),
        Part::new(
            "C end",
            "C5:3 G4:1 A4:3 G4:1 C5:3 G4:1 A4:3 G4:1 | C5:3 A4:1 G4:2 C4 _:8 | \
             C5:8 A4:3 G#4:1 A4:3 B4:1 | C5:3 A4:1 C5:2 B4 _:8 | \
             G4:3 E4:1 G4:3 E4:1 A4:3 B4:1 A4:3 Ab4:1 | G4:3 E4:1 G4:3 E4:1 A4:3 B4:1 C5:3 A4:1 | \
             D5:8 _:2 C#5 D5:4 | C5:12 -:4",
            "I | I I7 | IV | II7 V7 | I | I | II7 II7 II7 V7 | I",
        ),
    ],
    form: &["A", "B", "A2", "C", "A", "B", "A2", "C end"],
    drums_from: 0,
    about: "Walter Donaldson's 1922 hit. The tune hammers G and E in a dotted rhythm over C, \
            slips through a G diminished seventh into G7, and says it again a step higher over \
            D minor (ii). The middle eight starts on F (IV) and climbs by half steps, F sharp, \
            G, G sharp, A, over D7, the V of V, to land on G.",
};

// Still under copyright: written in 1959 (Paul Desmond died in 1977).
// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/PaulDesmondTakeFive.abc
// (Desmond's alto saxophone part of the Time Out recording, written for the E flat alto, a
// major sixth above the sound; here at concert pitch, in E flat minor. The intro is the
// eleven bars the piano and drums play before he comes in, shortened to four. The four
// sixteenths of "dedc" are written as eighths there, which overfills the bar; they are
// sixteenths in the lead sheet below, as here. Checked note by note against the Wikifonia
// lead sheet of Carmen McRae's sung version,
// https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/PaulDesmondCarmenMcRaeTakeFive.abc,
// which gives the chords: E flat minor for three beats and B flat minor seven for two, and
// the bridge's. Desmond's bridge is the same; his A plays a little chromatic run, B, C, D
// flat, where the singer holds G flat, B flat, D flat.)
pub(super) const TAKE_FIVE: Song = Song {
    name: "Take Five",
    origin: "Paul Desmond, for the Dave Brubeck Quartet's Time Out, 1959",
    feel: "Five-four swing",
    genre: Genre::Jazz,
    tonic: DSharp,
    mode: NaturalMinor,
    time: FIVE_FOUR,
    tempo: 172.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Five-four swing",
    band: SWING,
    style: "Five-four swing",
    parts: &[
        Part::new(
            "Intro",
            "-:20 | -:20 | -:20 | -:12 Bb3:2 Eb4 Gb4 Ab4",
            "i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | i i i v7 v7",
        ),
        Part::new(
            "A",
            "A4:2 Bb4 A4 Ab4 Gb4 -:2 Bb3:2 B3:1 C4 Db4:2 -:2 | Eb4:12 F4:1 Gb4 F4 Eb4 Db4:4 | \
             Eb4:12 Db4:1 Eb4 Db4 Bb3 Ab3:4 | Bb3:12 Bb3:2 Eb4 Gb4 Ab4 | \
             A4:2 Bb4 A4 Ab4 Gb4 -:2 Bb3:2 B3:1 C4 Db4:2 -:2 | Eb4:12 Db4:1 Eb4 Db4 Bb3 Ab3:4 | \
             Bb3:12 F4:1 Gb4 F4 Eb4 Db4:4 | Eb4:16 -:4",
            "i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | \
             i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | i i i v7 v7",
        ),
        Part::new(
            "Bridge",
            "Eb5:2 Gb5:4 Eb5:2 B4:4 Ab4:2 Bb4 B4 C5 | Db5:2 F5:4 Db5:2 Bb4:4 Gb4:2 Ab4 A4 Bb4 | \
             B4:2 Eb5:4 B4:2 Ab4:4 F4:2 Gb4 Ab4 A4 | Bb4:2 A4 Bb4 B4 Db5:4 Db5:2 C5 Db5 D5 | \
             Eb5:2 Gb5:4 Eb5:2 B4:4 Ab4:2 Bb4 B4 C5 | Db5:2 F5:4 Db5:2 Bb4:4 Gb4:2 Ab4 A4 Bb4 | \
             B4:2 Eb5:4 B4:2 Ab4:4 F4:2 Ab4 Db5 B4 | Bb4:12 Bb3:2 Eb4 Gb4 Ab4",
            "bVImaj7 bVImaj7 bVImaj7 iv6 iv6 | v7 v7 v7 i7 i7 | iv7 iv7 iv7 bVII7 bVII7 | \
             bIIImaj7 | bVImaj7 bVImaj7 bVImaj7 iv6 iv6 | v7 v7 v7 i7 i7 | \
             iv7 iv7 iv7 bVII7 bVII7 | ii7 ii7 ii7 V7 V7",
        ),
        Part::new(
            "Vamp",
            "-:20 | -:20 | -:20 | -:20 | -:20 | -:20 | -:20 | -:12 Bb3:2 Eb4 Gb4 Ab4",
            "i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | \
             i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | i i i v7 v7",
        ),
        Part::new(
            "Last A",
            "A4:2 Bb4 A4 Ab4 Gb4 -:2 Bb3:2 B3:1 C4 Db4:2 -:2 | Eb4:12 F4:1 Gb4 F4 Eb4 Db4:4 | \
             Eb4:12 Db4:1 Eb4 Db4 Bb3 Ab3:4 | Bb3:12 Bb3:2 Eb4 Gb4 Ab4 | \
             A4:2 Bb4 A4 Ab4 Gb4 -:2 Bb3:2 B3:1 C4 Db4:2 -:2 | Eb4:12 Db4:1 Eb4 Db4 Bb3 Ab3:4 | \
             Bb3:12 F4:1 Gb4 F4 Eb4 Db4:4 | Eb4:16 -:4",
            "i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | \
             i i i v7 v7 | i i i v7 v7 | i i i v7 v7 | i",
        ),
    ],
    form: &["Intro", "A", "Bridge", "A", "Vamp", "A", "Bridge", "Last A"],
    drums_from: 0,
    about: "Five beats to the bar, counted 1 2 3, 1 2: the piano's vamp holds E flat minor \
            for three beats and B flat minor seven for two, and the bass and the drums lean \
            on 1 and 4. Over it the alto saxophone's A section climbs through A natural, the \
            sharp fourth of the key, the bluesy note of the whole tune. The bridge leaves the \
            vamp for C flat major seven (bVI), then walks round the circle of fifths, B flat \
            minor, E flat minor, A flat minor, D flat seven, to G flat major, the relative \
            major; the second time round it ends on F minor seven and B flat seven, the V7 \
            that leads home to the vamp. The head, A B A, is played \
            twice around eight bars of vamp, where the solos go. The app names the key D sharp \
            minor, the same six-accidental key spelled with sharps.",
};

// Still under copyright: written in 1966 for the television series.
// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/138a0d35ab47782acc1da8abce4ad83057904961/wikifonia-dataset/original-abc/LaloSchifrinMISSIONIMPOSSIBLETHEME.abc
// (A 24-bar piano lead sheet in G minor, L:1/8 in 5/4, with the chords. Its first four
// bars are the ostinato alone, written as the tune; here the piano's left hand plays them,
// in octaves, and keeps playing the ostinato under the tune, on C under the C minor
// section, as the chords imply. The last three bars play the chords the lead sheet gives,
// in the ostinato's rhythm, which the tune's notes follow. Its Bb/Ab and Eb/Db are
// written here as Bb7/Ab and Eb7/Db, the same four notes, so the bass is spelled as the
// chord's seventh.)
pub(super) const MISSION_IMPOSSIBLE: Song = Song {
    name: "Mission: Impossible theme",
    origin: "Lalo Schifrin, for the television series Mission: Impossible, 1966",
    feel: "Five-four spy",
    genre: Genre::Jazz,
    tonic: G,
    mode: NaturalMinor,
    time: FIVE_FOUR,
    tempo: 160.0,
    comping: Comping::Aksak,
    bass: Bass::Aksak,
    rhythm: "Five-four spy",
    band: Band::new(
        gm::BRASS_SECTION,
        gm::ACOUSTIC_GRAND_PIANO,
        gm::ACOUSTIC_BASS,
    ),
    style: "Five-four spy",
    parts: &[
        Part::new("Intro", "-:20 | -:20 | -:20 | -:20", "Gm | Gm | Gm | Gm").with_left(
            "G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3 | \
             G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3",
        ),
        Part::new(
            "A",
            "(3 Bb5:2 G5 D5 _:12 -:4 | (3 Bb5:2 G5 Db5 _:12 -:4 | (3 Bb5:2 G5 C5 _:12 -:4 | \
             Bb4:2 C5 -:16 | G5:4 -:2 G5:4 -:2 Bb5:4 C6 | G5:4 -:2 G5:4 -:2 F5:4 F#5 | \
             G5:4 (3 Bb5:2 G5 F#6 _:8 -:4 | -:4 (3 Bb5:2 G5 F6 _:8 -:4 | \
             -:4 (3 Bb5:2 G5 E6 _:8 -:4 | Eb6:2 D6 -:16",
            "Gm | Gm | Gm | Gm | Gm Gm Gm Bbm Cm | Gm Gm Gm Fm F#m | Gm | Gm | Gm | Gm",
        )
        .with_left(
            "G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3 | \
             G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3 | \
             G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3 | \
             G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3 | \
             G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3",
        ),
        Part::new(
            "B",
            "Eb4:2 C4 G3:12 -:4 | Eb4:2 C4 Gb3:12 -:4 | Eb4:2 C4 F3:12 -:4 | Eb3:2 F3 -:16",
            "Cm | Cm | Cm | Cm",
        )
        .with_left(
            "C2+C3:4 -:2 C2+C3:4 -:2 Eb2+Eb3:4 F2+F3 | C2+C3:4 -:2 C2+C3:4 -:2 Bb1+Bb2:4 B1+B2 | \
             C2+C3:4 -:2 C2+C3:4 -:2 Eb2+Eb3:4 F2+F3 | C2+C3:4 -:2 C2+C3:4 -:2 Bb1+Bb2:4 B1+B2",
        ),
        Part::new(
            "Ending",
            "Bb4:2 G4 F#5:12 -:4 | Bb4:2 G4 F5:12 -:4 | Bb4:2 G4 E5:12 -:4 | \
             Eb5:2 D5 -:2 A4:4 -:2 Bb4:4 C5 | A4:4 -:8 F5:2 G5:6 | _:16 -:4",
            "Gm | Gm | Gm | Dsus4 Dsus4 Dsus4 Ebsus4 Fsus4 | \
             Dsus4 Dsus4 Dsus4 Dsus4 Dsus4 Dsus4 Bb7/Ab Eb7/Db Eb7/Db Eb7/Db | Eb7/Db",
        )
        .with_left(
            "G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | G2+G3:4 -:2 G2+G3:4 -:2 F2+F3:4 F#2+F#3 | \
             G2+G3:4 -:2 G2+G3:4 -:2 Bb2+Bb3:4 C3+C4 | \
             D2+D3+G3+A3:4 -:2 D2+D3+G3+A3:4 -:2 Eb2+Eb3+Ab3+Bb3:4 F2+F3+Bb3+C4 | \
             D2+D3+G3+A3:4 -:8 Ab2+Bb3+D4+F4:2 Db2+Eb3+G3+Bb3:6 | _:16 -:4",
        )
        .with_tempo(&[fermata(108, 8)]),
    ],
    form: &["Intro", "A", "B", "Ending"],
    drums_from: 0,
    about: "The theme of a television series about secret agents, in 5/4. The piano's \
            ostinato splits the bar's ten eighths 3 + 3 + 2 + 2, long, long, short, short: \
            G, G, then B flat and C; G, G, then F and F sharp, the leading note, creeping \
            back to G. Over it the brass fall in a triplet from B flat and G to D, the fifth \
            of G minor, then to D flat and C, sinking a half step at a time. The tune proper \
            is the ostinato's own notes two octaves up, harmonized with parallel minor \
            chords, B flat minor and C minor, F minor and F sharp minor; then the triplet \
            leaps up a major seventh to F sharp, F and E, and falls to E flat and D. The \
            call comes back on C minor, the iv, as a blues moves to its IV, then on G minor \
            again. The ending climbs in parallel sus4 chords, D, E flat, F, and stops \
            without resolving: B flat seven, then E flat seven, each with its seventh in \
            the bass, A flat and D flat, under a long G.",
};
