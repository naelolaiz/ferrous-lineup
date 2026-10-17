//! Electronic: classical themes as house, disco, techno and drum and bass.

use super::*;

pub(super) const ODE_TO_JOY: Song = Song {
    name: "Ode to Joy",
    origin: "Ludwig van Beethoven, 1824",
    feel: "House",
    genre: Genre::Electronic,
    tonic: D,
    mode: Major,
    time: COMMON,
    tempo: 124.0,
    comping: Comping::Stabs,
    bass: Bass::Offbeat,
    rhythm: "Four on the floor",
    style: "House piano",
    parts: &[Part::new(
        "Theme",
        "F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | F#5:6 E5:2 E5:8 | \
         F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | E5:6 D5:2 D5:8 | \
         E5:4 E5 F#5 D5 | E5 F#5:2 G5 F#5:4 D5 | E5 F#5:2 G5 F#5:4 E5 | D5 E5 A4:8 | \
         F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | E5:6 D5:2 D5:8",
        "I | V | I | I V | I | V | I | V I | V I | V I | V I | I V | I | V | I | V I",
    )],
    form: &["Theme", "Theme", "Theme"],
    drums_from: 0,
    about: "The tune of the last movement of Beethoven's Ninth Symphony, over a house beat. \
            It moves almost only by step, which is why anyone can sing it.",
};

pub(super) const KOROBEINIKI: Song = Song {
    name: "Korobeiniki",
    origin: "Russian folk song, 1861",
    feel: "Synthwave",
    genre: Genre::Electronic,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 132.0,
    comping: Comping::Arp16,
    bass: Bass::Octaves16,
    rhythm: "Synthwave",
    style: "Synthwave",
    parts: &[
        Part::new(
            "A",
            "E5:4 B4:2 C5 D5:4 C5:2 B4 | A4:4 A4:2 C5 E5:4 D5:2 C5 | B4:6 C5:2 D5:4 E5 | C5 A4 A4:8 | \
             -:2 D5:4 F5:2 A5:4 G5:2 F5 | E5:6 C5:2 E5:4 D5:2 C5 | B4:4 B4:2 C5 D5:4 E5 | C5 A4 A4:8",
            "V | i | V | i | iv | bIII | V | i",
        ),
        Part::new(
            "B",
            "E5:8 C5:8 | D5:8 B4:8 | C5:8 A4:8 | G#4:8 B4:8 | \
             E5:8 C5:8 | D5:8 B4:8 | C5:4 E5:4 A5:8 | G#5:16",
            "i | V | i | V | i | V | i | V",
        ),
    ],
    form: &["A", "A", "B", "A"],
    drums_from: 0,
    about: "\"The Peddlers\", a song about travelling merchants, as 1980s synth music. The \
            minor tune leans on E major, the V chord of harmonic minor.",
};

pub(super) const SYMPHONY_40: Song = Song {
    name: "Symphony No. 40",
    origin: "Wolfgang Amadeus Mozart, 1788",
    feel: "Drum and bass",
    genre: Genre::Electronic,
    tonic: G,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 172.0,
    comping: Comping::Held,
    bass: Bass::Sub,
    rhythm: "Drum and bass",
    style: "Drum and bass",
    parts: &[
        Part::new("Pickup", "-:12 Eb5:2 D5", "i"),
        Part::new(
            "Theme",
            "D5:4 Eb5:2 D5 D5:4 Eb5:2 D5 | D5:4 Bb5 -:4 Bb5:2 A5 | G5:4 G5:2 F5 Eb5:4 Eb5:2 D5 | \
             C5:4 C5 -:4 D5:2 C5 | C5:4 D5:2 C5 C5:4 D5:2 C5 | C5:4 A5 -:4 A5:2 G5 | \
             F#5:4 F5:2 Eb5 D5:4 D5:2 C5 | Bb4:4 Bb4 -:4 Eb5:2 D5",
            "i | i | i | V7 | V7 | V7 | V7 i | i",
        ),
    ],
    form: &["Pickup", "Theme", "Theme", "Theme", "Theme"],
    drums_from: 1,
    about: "Mozart's sighing two-note figure, a semitone falling from E flat to D, over the \
            fast broken beat and deep bass of drum and bass.",
};

pub(super) const EINE_KLEINE: Song = Song {
    name: "Eine kleine Nachtmusik",
    origin: "Wolfgang Amadeus Mozart, 1787",
    feel: "Disco",
    genre: Genre::Electronic,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 118.0,
    comping: Comping::Offbeat,
    bass: Bass::Octaves,
    rhythm: "Disco",
    style: "Disco",
    parts: &[Part::new(
        "Theme",
        "G5:4 -:2 D5:2 G5:4 -:2 D5:2 | G5:2 D5 G5 B5 D6:4 -:4 | C6:4 -:2 A5:2 C6:4 -:2 A5:2 | \
         C6:2 A5 F#5 A5 D5:4 -:4 | G5:2 -:2 G5:6 B5:2 A5 G5 | G5:2 F#5 F#5:6 A5:2 C6 F#5 | \
         A5:2 G5 G5:6 B5:2 A5 G5 | G5:2 F#5 F#5:6 A5:2 C6 F#5 | \
         G5:2 G5 F#5 E5:1 F#5 G5:2 G5 A5 G5:1 A5 | B5:2 B5 C6 B5:1 C6 D6:4 -:4",
        "I | I | V7 | V7 | I | V7 | I | V7 | I | I",
    )],
    form: &["Theme", "Theme", "Theme", "Theme"],
    drums_from: 0,
    about: "\"A little night music\": the opening is nothing but the chords themselves, G \
            major and then D7, broken into notes that rocket upwards; then the tune turns \
            and trills over a G in the bass. Under it, the octave bass and open hats of \
            1970s disco.",
};

pub(super) const TOCCATA: Song = Song {
    name: "Toccata in D minor",
    origin: "Attributed to Johann Sebastian Bach, early 18th century",
    feel: "Techno",
    genre: Genre::Electronic,
    tonic: D,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 128.0,
    comping: Comping::Stabs,
    bass: Bass::Offbeat,
    rhythm: "Techno",
    style: "Techno minor",
    parts: &[Part::new(
        "Toccata",
        "A5:1 G5 A5:6 -:8 | G5:1 F5 E5 D5 C#5:4 D5:8 | A4:1 G4 A4:6 -:8 | E4:2 F4 C#4:4 D4:8 | \
         A3:1 G3 A3:6 -:8 | G3:1 F3 E3 D3 C#3:4 D3:8 | C#4:2 E4 G4 Bb4 C#5 E5:6 | D5:16",
        "i | V7 i | i | V7 i | i | V7 i | vii°7 | I",
    )],
    form: &["Toccata", "Toccata", "Toccata"],
    drums_from: 0,
    about: "The organ's famous flourish, falling an octave at a time, as techno. Bar seven \
            climbs a diminished seventh chord, C sharp E G B flat: four notes a minor third \
            apart, the most restless chord there is. It settles on D major, not minor: a \
            Picardy third.",
};

pub(super) const TURKISH_MARCH: Song = Song {
    name: "Turkish March",
    origin: "Wolfgang Amadeus Mozart, Piano Sonata No. 11, 1783",
    feel: "Breakbeat",
    genre: Genre::Electronic,
    tonic: A,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 126.0,
    comping: Comping::Stabs,
    bass: Bass::Octaves,
    rhythm: "Breakbeat",
    style: "Eurodance",
    parts: &[
        Part::new("Pickup", "-:12 B4:1 A4 G#4 A4", "i"),
        Part::new(
            "Theme",
            "C5:2 -:2 D5:1 C5 B4 C5 E5:2 -:2 F5:1 E5 D#5 E5 | \
             B5:1 A5 G#5 A5 B5 A5 G#5 A5 C6:4 A5:2 C6 | B5:2 A5 G5 A5 B5 A5 G5 A5 | \
             B5:2 A5 G5 F#5 E5:4 B4:1 A4 G#4 A4",
            "i | i | v | v II V V",
        ),
        Part::new(
            "Theme end",
            "C5:2 -:2 D5:1 C5 B4 C5 E5:2 -:2 F5:1 E5 D#5 E5 | \
             B5:1 A5 G#5 A5 B5 A5 G#5 A5 C6:4 A5:2 B5 | C6:2 B5 A5 G#5 A5 E5 F5 D5 | \
             C5:4 B4:4 A4:4 -:4",
            "i | i bVI | i ii° i ii° | i V i i",
        ),
        Part::new("Refrain pickup", "-:12 A5:2 B5", "I").in_key(A, Major),
        Part::new(
            "Refrain",
            "C#6:4 A5:2 B5 C#6 B5 A5 G#5 | F#5:2 G#5 A5 B5 G#5 E5 A5 B5 | \
             C#6:4 A5:2 B5 C#6 B5 A5 G#5 | F#5:2 B5 G#5 E5 A5:4 A5:2 B5",
            "I | IV II7 V V | I | IV V I I",
        )
        .in_key(A, Major),
    ],
    form: &[
        "Pickup",
        "Theme",
        "Theme",
        "Theme end",
        "Refrain pickup",
        "Refrain",
        "Refrain",
    ],
    drums_from: 1,
    about: "Each turn, B A G sharp A, circles a note from the semitone below it. Mozart was \
            imitating the Turkish military bands that were the fashion in Vienna. The theme \
            is in A minor; the refrain that answers it turns to A major, the same key with \
            its third raised, as in the sonata.",
};

// Source: https://raw.githubusercontent.com/syntithenai/abc2book/db1681da9417cea25072527b9e75fbe7f28186a5/abcresources/folkinfo/abc_tune_folkinfo_1541.abc
// (BBC "Singing Together", Spring 1975, in 2/4; here two bars to the bar, in A minor, up a
// fourth from E minor. The two held bars with fermatas are written as a whole bar each.
// The source has no chords: they come from the D minor ABC at
// https://raw.githubusercontent.com/avnerdorman/abc-solfege-tool/main/songs/Kalinka_Dm.abc,
// which has the same chorus and a slightly different verse.)
pub(super) const KALINKA: Song = Song {
    name: "Kalinka",
    origin: "Ivan Larionov, 1860",
    feel: "Techno",
    genre: Genre::Electronic,
    tonic: A,
    mode: NaturalMinor,
    time: COMMON,
    tempo: 140.0,
    comping: Comping::Stabs,
    bass: Bass::Offbeat,
    rhythm: "Techno",
    style: "Techno minor",
    parts: &[
        Part::new("Intro", "E5:8 E5:8", "i"),
        Part::new(
            "Chorus",
            "D5:4 B4:2 C5 D5:4 B4:2 C5 | D5:4 C5:2 B4 A4:4 E5:4 | \
             D5:3 C5:1 B4:2 C5 D5:4 B4:2 C5 | D5:4 C5:2 B4 A4:4 E5:4 | \
             D5:4 B4:2 C5 D5:4 B4:2 C5 | D5:4 C5:2 B4 A4:4 E5:4 | \
             D5:3 C5:1 B4:2 C5 D5:4 B4:2 C5 | D5:4 C5:2 B4 A4:8",
            "V7 | V7 i | V7 | V7 i | V7 | V7 i | V7 | V7 i",
        ),
        Part::new("Oh", "A5:8 G5:8", "i bVII7"),
        Part::new(
            "Verse",
            "E5:2 G5 F5 E5:1 C5 C5:4 G4 | E5:2 G5 F5 E5:1 D5 C5:4 G4 | \
             G4:4 A4:2 B4 D5 C5 B4 A4 | G4:4 G5 G5:8 | \
             E5:2 G5 F5 E5:1 D5 C5:4 A4 | E5:2 G5 F5 E5:1 D5 C5:4 G4 | \
             A4:4 A4:2 B4 D5 C5 B4 A4 | G5:4 F5 E5:8",
            "bIII bVII7 bIII bIII | bIII bVII7 bIII bIII | bVI bVI IV7 IV7 | bVII | \
             bIII bVII7 bIII bIII | bIII bVII7 bIII bIII | bVI bVI IV7 IV7 | bVII iv V7 V7",
        ),
    ],
    form: &["Intro", "Chorus", "Oh", "Verse", "Intro", "Chorus"],
    drums_from: 1,
    about: "Larionov's song of 1860, sung everywhere as a Russian folk song, in 2/4 written \
            two bars to the bar. The chorus hangs on E7 with C, the minor sixth, leaning on \
            B, and lands on A minor every second bar; the slower verse turns to C major, the \
            relative major, and comes back through D7 and E7. Here it is techno: a kick on \
            every beat, stabs and an offbeat bass.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/BachJS/BWV1007/bwv1007/bwv1007-lys/bwv1007_prelude.ly
// (Mutopia 517; the file is written an octave above the cello, as here)
pub(super) const CELLO_SUITE_PRELUDE: Song = Song {
    name: "Cello Suite No. 1 Prelude",
    origin: "Johann Sebastian Bach, BWV 1007, about 1720",
    feel: "Trance",
    genre: Genre::Electronic,
    tonic: G,
    mode: Major,
    time: COMMON,
    tempo: 126.0,
    comping: Comping::Held,
    bass: Bass::Held,
    rhythm: "Trance",
    style: "Ambient",
    parts: &[
        Part::new(
            "Opening",
            "G3:1 D4 B4 A4 B4 D4 B4 D4 G3 D4 B4 A4 B4 D4 B4 D4 | \
             G3 E4 C5 B4 C5 E4 C5 E4 G3 E4 C5 B4 C5 E4 C5 E4 | \
             G3 F#4 C5 B4 C5 F#4 C5 F#4 G3 F#4 C5 B4 C5 F#4 C5 F#4 | \
             G3 G4 B4 A4 B4 G4 B4 G4 G3 G4 B4 A4 B4 G4 B4 F#4",
            "I | IV | V7 | I",
        ),
        Part::new(
            "Onward",
            "G3:1 E4 B4 A4 B4 G4 F#4 G4 E4 G4 F#4 G4 B3 D4 C#4 B3 | \
             C#4 G4 A4 G4 A4 G4 A4 G4 C#4 G4 A4 G4 A4 G4 A4 G4 | \
             F#4 A4 D5 C#5 D5 A4 G4 A4 F#4 A4 G4 A4 D4 F#4 E4 D4 | \
             E3 B3 G4 F#4 G4 B3 G4 B3 E3 B3 G4 F#4 G4 B3 G4 B3 | \
             E3 C#4 D4 E4 D4 C#4 B3 A3 G4 F#4 E4 D5 C#5 B4 A4 G4 | \
             F#4 E4 D4 D5 A4 D5 F#4 A4 D4 E4 F#4 A4 G4 F#4 E4 D4 | \
             G#4 D4 F4 E4 F4 D4 G#4 D4 B4 D4 F4 E4 F4 D4 G#4 D4 | \
             C4 E4 A4 B4 C5 A4 E4 D4 C4 E4 A4 B4 C5 A4 F#4 E4",
            "vi | II7 | V | vi | II7 | V | III7 | ii",
        ),
    ],
    form: &["Opening", "Onward", "Opening", "Onward"],
    drums_from: 0,
    about: "The first twelve bars of the prelude, a solo cello playing nothing but \
            sixteenth notes, here as a trance arpeggio. Under the first four bars the \
            open G string sounds on every half bar while the chord above it changes: G, \
            C over G, D7 over G. Then the line travels through E minor, A7 and D, the \
            dominant, and E7 to A minor. Written an octave above the cello.",
};

// Source: https://raw.githubusercontent.com/cellist/Lilypond-Sheet-Music/master/Fl,%20Vl,%20Vlc/Bach,%20Johann%20Sebastian/Badinerie/v1.ily
// (the flute part, and v3.ily for the bass, of a trio arrangement by Ioan Dobrinescu)
pub(super) const BADINERIE: Song = Song {
    name: "Badinerie",
    origin: "Johann Sebastian Bach, Orchestral Suite No. 2, about 1738",
    feel: "Techno",
    genre: Genre::Electronic,
    tonic: B,
    mode: HarmonicMinor,
    time: COMMON,
    tempo: 128.0,
    comping: Comping::Stabs,
    bass: Bass::Offbeat,
    rhythm: "Techno",
    style: "Techno minor",
    parts: &[
        Part::new("Pickup", "-:12 B5:2 D6:1 B5:1", "i"),
        Part::new(
            "A",
            "F#5:2 B5:1 F#5:1 D5:2 F#5:1 D5:1 B4:4 F#4:1 B4:1 D5:1 B4:1 | \
             C#5:1 B4 C#5 B4 A#4 C#5 E5 C#5 D5:2 B4:2 D5:2 -:2 | \
             B4:2 -:2 B4:2 -:2 B4:4 D5:2 D5:2 | D5:2 D5 B5 D5 D5 C#5 F#5 F#5",
            "i | iiø7 V7 i i | i | IV7 IV7 bVII v",
        ),
        Part::new(
            "F sharp minor",
            "F#5:2 F#5 D6 F#5 F#5 E#5 C#5:1 F#5 A5 F#5 | \
             G#5:1 F#5 G#5 F#5 E#5 G#5 B5 G#5 A5 G#5 A5 G#5 F#5 A5 F#5 E#5 | \
             F#5:1 B5 F#5 E#5 F#5 C#6 F#5 E#5 F#5 D6 F#5 E#5 F#5 D6 C#6 B5 | \
             C#6:1 A5 G#5 F#5 A5:2 G#5 F#5:4 B5:2 D6:1 B5:1",
            "iv iiø7 V7 i | iiø7 V7 i i | iv V iv iiø7 | i V7 i i",
        )
        .in_key(FSharp, HarmonicMinor),
        Part::new(
            "F sharp minor end",
            "F#5:2 F#5 D6 F#5 F#5 E#5 C#5:1 F#5 A5 F#5 | \
             G#5:1 F#5 G#5 F#5 E#5 G#5 B5 G#5 A5 G#5 A5 G#5 F#5 A5 F#5 E#5 | \
             F#5:1 B5 F#5 E#5 F#5 C#6 F#5 E#5 F#5 D6 F#5 E#5 F#5 D6 C#6 B5 | \
             C#6:1 A5 G#5 F#5 A5:2 G#5 F#5:4 -:4",
            "iv iiø7 V7 i | iiø7 V7 i i | iv V iv iiø7 | i V7 i i",
        )
        .in_key(FSharp, HarmonicMinor),
    ],
    form: &["Pickup", "A", "F sharp minor", "A", "F sharp minor end"],
    drums_from: 1,
    about: "The flute's showpiece that ends Bach's suite, in B minor and in 2/4: two of \
            Bach's bars make one bar here. The tune leaps down the B minor chord, B, F sharp, \
            D, B, then runs on in sixteenths and turns to F sharp minor, the minor dominant, \
            to end its first half. The trills and grace notes are left out. Only the first \
            half is here; the second has runs in thirty-second notes.",
};

// Source: https://raw.githubusercontent.com/MutopiaProject/MutopiaProject/master/ftp/VivaldiA/O8/spring/spring-lys/spring1a.ly
// (Mutopia 301: violino primo, with spring1d.ly and the figures of spring1f.ly for the chords)
pub(super) const SPRING: Song = Song {
    name: "Spring",
    origin: "Antonio Vivaldi, The Four Seasons, 1725",
    feel: "Dance pop",
    genre: Genre::Electronic,
    tonic: E,
    mode: Major,
    time: COMMON,
    tempo: 118.0,
    comping: Comping::Stabs,
    bass: Bass::Offbeat,
    rhythm: "Dance pop",
    style: "House piano",
    parts: &[
        Part::new("Pickup", "-:14 E4:2", "I"),
        Part::new(
            "Theme",
            "G#4:2 G#4:2 G#4:2 F#4:1 E4:1 B4:6 B4:1 A4:1 | G#4:2 G#4:2 G#4:2 F#4:1 E4:1 B4:6 B4:1 A4:1 | \
             G#4:2 A4:1 B4:1 A4:2 G#4:2 F#4:2 D#4:2 B3:2 E4:2 | \
             G#4:2 G#4:2 G#4:2 F#4:1 E4:1 B4:6 B4:1 A4:1 | G#4:2 G#4:2 G#4:2 F#4:1 E4:1 B4:6 B4:1 A4:1 | \
             G#4:2 A4:1 B4:1 A4:2 G#4:2 F#4:4 -:2 E4:2",
            "I | I | I I ii II7 V V V V | I | I | I I ii II7 V V V V",
        ),
        Part::new(
            "Refrain",
            "B4:2 A4:1 G#4:1 A4:2 B4:2 C#5:2 B4:4 E4:2 | B4:2 A4:1 G#4:1 A4:2 B4:2 C#5:2 B4:4 E4:2 | \
             C#5:2 B4:4 A4:2 G#4:2 F#4:1 E4:1 F#4:4 | E4:4 -:2 E4:2 B4:2 A4:1 G#4:1 A4:2 B4:2 | \
             C#5:2 B4:4 E4:2 B4:2 A4:1 G#4:1 A4:2 B4:2 | C#5:2 B4:4 E4:2 C#5:2 B4:4 A4:2 | \
             G#4:2 F#4:1 E4:1 F#4:4 E4:4 -:4",
            "I I IV I | I I IV I | IV I I V | I | IV I I I | IV I IV I | I V I I",
        ),
    ],
    form: &["Pickup", "Theme", "Refrain"],
    drums_from: 1,
    about: "The opening ritornello of the first of the Four Seasons, played by all the \
            strings over a bass that hardly leaves E. Each phrase comes twice, loud and \
            then soft as an echo, and the second tune leans on A over E, the IV chord \
            above the tonic in the bass. The trills are left out; an octave below the \
            violins.",
};

// Source: https://raw.githubusercontent.com/oriolcolomefont/The-Sound-of-AI-workshop/main/wikifonia-dataset/original-abc/JacquesOffenbachCanCan.abc
// (a Wikifonia lead sheet in D, here a fourth lower in A)
pub(super) const CAN_CAN: Song = Song {
    name: "Can-can",
    origin: "Jacques Offenbach, Orpheus in the Underworld, 1858",
    feel: "Techno",
    genre: Genre::Electronic,
    tonic: A,
    mode: Major,
    time: COMMON,
    tempo: 138.0,
    comping: Comping::Stabs,
    bass: Bass::Octaves,
    rhythm: "Techno",
    style: "EDM drop",
    parts: &[
        Part::new(
            "Intro",
            "E4:4 E5:4 E4:4 E5:4 | E4:2 E4 E4 E4 E5 E5 E5 E5",
            "V | V",
        ),
        Part::new(
            "A",
            "E4:2 B4 B4 C#5 B4 A4 A4 C#5 | D5:2 F#5 A5 F#5 F#5 E5 E5:4 | \
             F#5:2 G#4 G#4 F#5 E5 A4 A4 C#5 | C#5:2 B4 C#5 B4 C#5 B4 C#5 B4",
            "V7 I | IV I | V7 I | II7 V7",
        ),
        Part::new(
            "A again",
            "E4:2 B4 B4 C#5 B4 A4 A4 C#5 | D5:2 F#5 A5 F#5 F#5 E5 E5:4 | \
             F#5:2 G#4 G#4 F#5 E5 A4 A4 C#5 | C#5:2 B4 C#5 B4 B4 A4 A4:4",
            "V7 I | IV I | V7 I | V7 I",
        ),
        Part::new(
            "B",
            "C#5:4 A4 F#4 E4 | E5:2 B4 C#5 D5 C#5 B4 A4:4 | C#5:4 A4 F#4 E4 | \
             D#5:2 E5 F#5 G#5 B5 A5 A5:4",
            "I | V7 I | I | V7 I",
        ),
        Part::new(
            "Galop",
            "A4:8 B4:2 D5 C#5 B4 | E5:4 E5 E5:2 F#5 C#5 D5 | B4:4 B4 B4:2 D5 C#5 B4 | \
             A4:2 A5 G#5 F#5 E5 D5 C#5 B4 | A4:8 B4:2 D5 C#5 B4 | E5:4 E5 E5:2 F#5 C#5 D5 | \
             B4:4 B4 B4:2 D5 C#5 B4 | A4:2 E5 B4 C#5 A4:4 A4:4",
            "I V7 | I | V7 | I V7 | I V7 | I | V7 | I V7 I I",
        ),
        Part::new(
            "Galop end",
            "A4:8 B4:2 D5 C#5 B4 | E5:4 E5 E5:2 F#5 C#5 D5 | B4:4 B4 B4:2 D5 C#5 B4 | \
             A4:2 A5 G#5 F#5 E5 D5 C#5 B4 | A4:8 B4:2 D5 C#5 B4 | E5:4 E5 E5:2 F#5 C#5 D5 | \
             B4:4 B4 B4:2 D5 C#5 B4 | A4:2 E5 B4 C#5 A4:4 -:4",
            "I V7 | I | V7 | I V7 | I V7 | I | V7 | I V7 I I",
        ),
    ],
    form: &["Intro", "A", "A again", "B", "Galop", "Galop end"],
    drums_from: 0,
    about: "The galop from Offenbach's operetta, danced as the can-can. It is in 2/4, so \
            two of its bars make one bar here, and almost all of it is two chords, I and \
            V7. The famous strain comes last: a long note, a quick turn, three repeated \
            notes, and a run down the scale from the octave above. In A, a fourth below \
            the lead sheet.",
};
