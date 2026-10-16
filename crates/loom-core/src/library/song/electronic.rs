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
