//! Ear training: questions to answer by ear.
//!
//! Hearing harmony is learned like reading: by doing it often, a little
//! at a time. There are three exercises, from small to large:
//!
//! - **Intervals**: two notes; say how far apart they are, from a minor
//!   second (one semitone) to an octave (twelve).
//! - **Chords**: one chord; say what kind it is: major, minor,
//!   diminished, augmented, or one of five seventh chords.
//! - **Progressions**: the band plays a few chords in a key, starting
//!   at home (I); say what each of the others is, as the Roman numeral
//!   of its harmony key.
//!
//! A question is made from a seed, so the same seed always asks the same
//! question, and the tests below can check thousands of them. Each
//! question knows what to play (a list of [`Cue`]s), the right answer,
//! and how to explain it: the notes written as a musician writes them,
//! and what gives that interval, chord or move its sound. Nothing here
//! plays sound or draws: the app does that.

use serde::{Deserialize, Serialize};

use crate::harmony::keys::{self, ChordKey, ChordSize};
use crate::harmony::{
    Chord, ChordQuality, Random, Scale, ScaleKind, interval_name, next_chords,
    spelled_interval_name,
};
use crate::library::voicing::bass_root;
use crate::project::Drum;
use crate::{Note, NoteName, PitchClass, Spelling};

/// The three exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Exercise {
    #[default]
    Intervals,
    Chords,
    Progressions,
}

impl Exercise {
    pub const ALL: [Exercise; 3] = [
        Exercise::Intervals,
        Exercise::Chords,
        Exercise::Progressions,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Exercise::Intervals => "Intervals",
            Exercise::Chords => "Chords",
            Exercise::Progressions => "Progressions",
        }
    }
}

/// Who plays a note of a question.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Piano,
    Bass,
    Drum(Drum),
}

/// One note of a question: who plays it, when, for how long and how loud.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cue {
    pub part: Part,
    /// When it starts, in seconds from the start of the question.
    pub start: f64,
    /// How long it is held, in seconds.
    pub length: f64,
    pub note: Note,
    pub velocity: u8,
}

impl Cue {
    fn piano(start: f64, length: f64, note: Note, velocity: u8) -> Cue {
        Cue {
            part: Part::Piano,
            start,
            length,
            note,
            velocity,
        }
    }

    /// When the note stops.
    pub fn end(&self) -> f64 {
        self.start + self.length
    }
}

/// How long a list of cues lasts, in seconds: until its last note stops.
/// The seed of the question after the one asked with `seed`.
///
/// Every question draws its notes from a small random number generator
/// started at its seed. Stepping the seed the way that generator steps
/// itself would start each question one draw further along the last
/// one's numbers, so its first draw (the chord's kind) would be the last
/// question's second (its root): after C, E or Ab the next chord was
/// always major. A seed taken from the generator's output starts fresh
/// numbers.
pub fn next_seed(seed: u64) -> u64 {
    Random(seed).next()
}

pub fn duration(cues: &[Cue]) -> f64 {
    cues.iter().map(Cue::end).fold(0.0, f64::max)
}

/// Which of a list of answers are asked: one bit for each, by its place
/// in the list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choices(u16);

impl Choices {
    /// The answers at these places.
    pub fn of(places: &[usize]) -> Choices {
        Choices(places.iter().fold(0, |bits, place| bits | 1 << place))
    }

    pub fn has(self, place: usize) -> bool {
        self.0 & (1 << place) != 0
    }

    /// Adds the answer at `place`, or takes it away. The last one left
    /// stays: there is always something to ask.
    pub fn toggle(&mut self, place: usize) {
        let toggled = self.0 ^ (1 << place);
        if toggled != 0 {
            self.0 = toggled;
        }
    }

    pub fn count(self) -> usize {
        self.0.count_ones() as usize
    }

    /// The places chosen among the first `of`, or all of them if none is.
    fn places(self, of: usize) -> Vec<usize> {
        let chosen: Vec<usize> = (0..of).filter(|&place| self.has(place)).collect();
        if chosen.is_empty() {
            (0..of).collect()
        } else {
            chosen
        }
    }
}

/// "a" or "an", whichever goes before `word`.
fn article(word: &str) -> &'static str {
    if word.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    }
}

/// A word list as in a sentence: "a, b and c".
fn listed(words: &[String]) -> String {
    match words.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
        None => String::new(),
    }
}

// ----------------------------------------------------------------------
// Intervals

/// An interval of an octave or less, by its size in semitones, 1 to 12.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval(u8);

impl Interval {
    /// From the minor second (1 semitone) to the octave (12).
    pub const ALL: [Interval; 12] = [
        Interval(1),
        Interval(2),
        Interval(3),
        Interval(4),
        Interval(5),
        Interval(6),
        Interval(7),
        Interval(8),
        Interval(9),
        Interval(10),
        Interval(11),
        Interval(12),
    ];

    /// The interval of `semitones`, 1 to 12.
    pub fn new(semitones: u8) -> Option<Interval> {
        (1..=12).contains(&semitones).then_some(Interval(semitones))
    }

    pub fn semitones(self) -> u8 {
        self.0
    }

    /// Its place in [`Interval::ALL`].
    pub fn place(self) -> usize {
        usize::from(self.0 - 1)
    }

    /// The short name musicians write: m3 for a minor third, P5 for a
    /// perfect fifth, TT for the tritone and P8 for the octave.
    pub fn short(self) -> &'static str {
        const SHORT: [&str; 12] = [
            "m2", "M2", "m3", "M3", "P4", "TT", "P5", "m6", "M6", "m7", "M7", "P8",
        ];
        SHORT[self.place()]
    }

    /// Its name, such as "minor third".
    pub fn name(self) -> &'static str {
        match self.0 {
            12 => "octave",
            semitones => interval_name(semitones),
        }
    }

    /// How many letters apart its notes are written: two for a third (C
    /// to E or Eb), seven for the octave (C to C). The tritone can be
    /// written either way: as an augmented fourth (C to F#, three
    /// letters) or a diminished fifth (C to Gb, four).
    fn letters(self) -> &'static [u8] {
        const LETTERS: [&[u8]; 12] = [
            &[1],
            &[1],
            &[2],
            &[2],
            &[3],
            &[3, 4],
            &[4],
            &[5],
            &[5],
            &[6],
            &[6],
            &[7],
        ];
        LETTERS[self.place()]
    }

    /// What gives the interval its sound, and where else it is heard.
    pub fn sound(self) -> &'static str {
        const SOUND: [&str; 12] = [
            "The smallest step on the piano: the closest two notes can be, and tense.",
            "A whole step: two semitones, like the first two notes of a major scale.",
            "The bottom of a minor chord: from its root up to its third.",
            "The bottom of a major chord: from its root up to its third.",
            "Open and hollow. Turned upside down, it is a perfect fifth.",
            "Exactly half an octave, three whole steps: restless, it splits the octave in two.",
            "Open and stable: the root and the fifth of every major and minor chord.",
            "A major third turned upside down: C up to E, then E up to the next C.",
            "A minor third turned upside down: C up to Eb, then Eb up to the next C.",
            "From the root up to the seventh of a dominant seventh chord, which pulls to the next chord.",
            "One semitone short of an octave: it leans up to the octave above.",
            "The same note, higher: the two sound almost as one.",
        ];
        SOUND[self.place()]
    }

    /// A tune that starts with this interval going up, to remember it by.
    pub fn tune(self) -> Option<&'static str> {
        const TUNES: [Option<&str>; 12] = [
            Some("the shark's two notes in the film Jaws"),
            Some("Happy Birthday, at its first step up"),
            Some("the first two notes of Greensleeves"),
            Some("the first two notes of When the Saints Go Marching In"),
            Some("the first two notes of Here Comes the Bride"),
            Some("the first two notes of Maria, from West Side Story"),
            Some("Twinkle, Twinkle, Little Star, at its first leap"),
            Some("The Entertainer, at its first leap up"),
            Some("the first two notes of My Bonnie Lies over the Ocean"),
            Some("the first two notes of Somewhere, from West Side Story"),
            None,
            Some("the first two notes of Over the Rainbow"),
        ];
        TUNES[self.place()]
    }

    /// The two notes of the interval up from `low`, written as musicians
    /// write them: the letters say which number it is (C to E is a
    /// third, C to Eb too) and the semitones which kind. A black key at
    /// the bottom is written with a sharp or a flat, whichever writes the
    /// pair with fewer of them: C# up to E is a minor third, Db up to F
    /// a major third; when both are as simple, with its usual name (Bb
    /// up to Db, not A# up to C#). The tritone is written as an augmented
    /// fourth or a diminished fifth by the same rule (F to B, Bb to E),
    /// and as an augmented fourth when both are as simple.
    pub fn spell(self, low: PitchClass) -> (NoteName, NoteName) {
        let usual = NoteName::chromatic(low);
        let other = match usual.accidental {
            0 => usual,
            _ if usual.accidental > 0 => NoteName::plain(low, Spelling::Flats),
            _ => NoteName::plain(low, Spelling::Sharps),
        };
        let lows = [usual, other];
        let accidentals = |(a, b): &(NoteName, NoteName)| a.accidental.abs() + b.accidental.abs();
        lows.into_iter()
            .flat_map(|low| {
                self.letters()
                    .iter()
                    .map(move |&letters| (low, low.above(self.0 % 12, letters)))
            })
            .min_by_key(accidentals)
            .unwrap_or((lows[0], lows[0]))
    }

    /// The name of the interval from `low` up to `high` as they are
    /// written: "perfect fifth", "augmented fourth", "octave".
    pub fn spelled_name(self, low: NoteName, high: NoteName) -> String {
        if self.0 == 12 {
            return "octave".to_string();
        }
        spelled_interval_name(low, high)
    }
}

/// How the two notes of an interval are played.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// The lower note, then the higher.
    Up,
    /// The higher note, then the lower.
    Down,
    /// Both at once.
    Together,
}

impl Direction {
    pub const ALL: [Direction; 3] = [Direction::Up, Direction::Down, Direction::Together];

    pub fn name(self) -> &'static str {
        match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Together => "Together",
        }
    }
}

/// Which intervals are asked, and played how.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IntervalSettings {
    /// By place in [`Interval::ALL`].
    pub intervals: Choices,
    /// By place in [`Direction::ALL`].
    pub directions: Choices,
}

impl Default for IntervalSettings {
    /// A start that is easy to tell apart: the two thirds, the perfect
    /// fourth and fifth, and the octave, going up.
    fn default() -> Self {
        IntervalSettings {
            intervals: Choices::of(&[2, 3, 4, 6, 11]),
            directions: Choices::of(&[0]),
        }
    }
}

/// The lowest note of an interval question: E3. The highest is G4, so
/// an octave up from it is still in the middle of the keyboard.
const INTERVAL_LOWEST: u8 = 52;
const INTERVAL_SPAN: usize = 16;

/// Two notes to name the distance between.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalQuestion {
    pub low: Note,
    pub interval: Interval,
    pub direction: Direction,
}

impl IntervalQuestion {
    /// A question from `seed`, asking one of the intervals chosen.
    pub fn new(settings: &IntervalSettings, seed: u64) -> Self {
        let mut random = Random(seed);
        let intervals = settings.intervals.places(Interval::ALL.len());
        let directions = settings.directions.places(Direction::ALL.len());
        let interval = Interval::ALL[intervals[random.below(intervals.len())]];
        let direction = Direction::ALL[directions[random.below(directions.len())]];
        let low = Note::new(INTERVAL_LOWEST + random.below(INTERVAL_SPAN) as u8);
        IntervalQuestion {
            low,
            interval,
            direction,
        }
    }

    pub fn high(&self) -> Note {
        self.with(self.interval).1
    }

    /// The two notes of `interval` played the way this question plays
    /// its own: from the same lower note going up or together, from the
    /// same higher note going down. To compare with the answer.
    fn with(&self, interval: Interval) -> (Note, Note) {
        let up = |low: Note| {
            let high = low.transpose(i32::from(interval.0)).unwrap_or(Note::MAX);
            (low, high)
        };
        match self.direction {
            Direction::Down => {
                let high = self
                    .low
                    .transpose(i32::from(self.interval.0))
                    .unwrap_or(Note::MAX);
                let low = high.transpose(-i32::from(interval.0)).unwrap_or(Note::MIN);
                (low, high)
            }
            Direction::Up | Direction::Together => up(self.low),
        }
    }

    /// What to play for the question.
    pub fn cues(&self) -> Vec<Cue> {
        self.cues_for(self.interval)
    }

    /// What to play for `interval` instead, in the same place: to hear
    /// the answer given next to the right one.
    pub fn cues_for(&self, interval: Interval) -> Vec<Cue> {
        let (low, high) = self.with(interval);
        match self.direction {
            Direction::Up => vec![
                Cue::piano(0.0, 0.9, low, 84),
                Cue::piano(1.0, 1.3, high, 84),
            ],
            Direction::Down => vec![
                Cue::piano(0.0, 0.9, high, 84),
                Cue::piano(1.0, 1.3, low, 84),
            ],
            Direction::Together => vec![
                Cue::piano(0.0, 1.8, low, 80),
                Cue::piano(0.0, 1.8, high, 80),
            ],
        }
    }

    /// The right answer, explained: the notes with their octaves, the
    /// interval's full name and size, what gives it its sound and, going
    /// up, a tune that starts with it.
    pub fn explain(&self) -> String {
        let (low_name, high_name) = self.interval.spell(self.low.pitch_class());
        let (low, high) = self.with(self.interval);
        let low = low.name_as(low_name);
        let high = high.name_as(high_name);
        let notes = match self.direction {
            Direction::Up => format!("{low} up to {high}"),
            Direction::Down => format!("{high} down to {low}"),
            Direction::Together => format!("{low} and {high} together"),
        };
        let name = self.interval.spelled_name(low_name, high_name);
        let name = if self.interval.0 == 6 {
            format!("a tritone ({} {name})", article(&name))
        } else {
            format!("{} {name}", article(&name))
        };
        let size = match self.interval.0 {
            1 => "1 semitone".to_string(),
            n => format!("{n} semitones"),
        };
        let mut text = format!("{notes}: {name}, {size}. {}", self.interval.sound());
        if self.direction == Direction::Up
            && let Some(tune) = self.interval.tune()
        {
            text += &format!(" Like {tune}.");
        }
        text
    }
}

// ----------------------------------------------------------------------
// Chords

/// The kinds of chord the chord exercise asks: the four triads, then
/// the five seventh chords of major and minor keys.
///
/// Each sounds different from all the others, however it is turned
/// (inverted): no two have the same notes on any roots. That is why the
/// sixth chords and the suspended chords are not here: C6 has the notes
/// of Am7, and Csus2 those of Gsus4.
pub const CHORD_QUALITIES: [ChordQuality; 9] = [
    ChordQuality::Major,
    ChordQuality::Minor,
    ChordQuality::Diminished,
    ChordQuality::Augmented,
    ChordQuality::Dominant7,
    ChordQuality::Major7,
    ChordQuality::Minor7,
    ChordQuality::HalfDiminished7,
    ChordQuality::Diminished7,
];

/// What gives a chord of the exercise its sound, and where it is found.
pub fn chord_sound(quality: ChordQuality) -> &'static str {
    use ChordQuality::*;
    match quality {
        Major => "Bright and settled.",
        Minor => "Darker than major: its third is a semitone lower.",
        Diminished => {
            "Tense: two minor thirds, so its fifth is a semitone lower than a minor chord's."
        }
        Augmented => {
            "Unsettled, floating: two major thirds, so its fifth is a semitone higher than a major chord's."
        }
        Dominant7 => {
            "A major chord with a minor seventh: it pulls to the chord a fifth below, as V7 pulls to I."
        }
        Major7 => {
            "A major chord with a major seventh: soft and dreamy, the Imaj7 and IVmaj7 of a major key."
        }
        Minor7 => {
            "A minor chord with a minor seventh: mellow, the ii7, iii7 and vi7 of a major key."
        }
        HalfDiminished7 => {
            "A diminished chord with a minor seventh: the viiø7 of a major key and the iiø7 of a minor key."
        }
        Diminished7 => {
            "Minor thirds all the way up, splitting the octave in four equal parts: the vii°7 of harmonic minor."
        }
        _ => "",
    }
}

/// Which chords are asked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChordSettings {
    /// By place in [`CHORD_QUALITIES`].
    pub qualities: Choices,
    /// Whether a chord can have another of its notes in the bass.
    pub inversions: bool,
}

impl Default for ChordSettings {
    /// The four triads, root in the bass.
    fn default() -> Self {
        ChordSettings {
            qualities: Choices::of(&[0, 1, 2, 3]),
            inversions: false,
        }
    }
}

/// The lowest root of a chord question: C3, so the chord sits around
/// middle C.
const CHORD_LOWEST: u8 = 48;

/// "Third", "fifth" and "seventh", by number.
const ORDINALS: [&str; 8] = [
    "", "root", "second", "third", "fourth", "fifth", "sixth", "seventh",
];
const NUMBERS: [&str; 8] = ["", "one", "two", "three", "four", "five", "six", "seven"];

/// The inversions' names, by the note of the chord in the bass.
const INVERSIONS: [&str; 4] = [
    "root position",
    "first inversion",
    "second inversion",
    "third inversion",
];

/// One chord to name the kind of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChordQuestion {
    pub chord: Chord,
    /// Which note of the chord is in the bass, counted up from the root
    /// (0, root position), in the order of [`ChordQuality::intervals`].
    pub inversion: usize,
    /// Where the root is played, in root position.
    root_note: Note,
}

impl ChordQuestion {
    /// A question from `seed`, asking one of the kinds chosen.
    pub fn new(settings: &ChordSettings, seed: u64) -> Self {
        let mut random = Random(seed);
        let qualities = settings.qualities.places(CHORD_QUALITIES.len());
        let quality = CHORD_QUALITIES[qualities[random.below(qualities.len())]];
        let root = random.below(12) as u8;
        let inversion = if settings.inversions {
            random.below(quality.intervals().len())
        } else {
            0
        };
        ChordQuestion {
            chord: Chord::new(PitchClass::from_semitone(root), quality),
            inversion,
            root_note: Note::new(CHORD_LOWEST + root),
        }
    }

    /// The notes played, lowest first.
    pub fn notes(&self) -> Vec<Note> {
        self.notes_as(self.chord.quality)
    }

    /// The notes of a chord of `quality` on the same root, with the same
    /// note of the chord in the bass: to hear the answer given next to
    /// the right one.
    pub fn notes_as(&self, quality: ChordQuality) -> Vec<Note> {
        let mut notes: Vec<Note> = quality
            .intervals()
            .iter()
            .filter_map(|&up| self.root_note.transpose(i32::from(up)))
            .collect();
        let turns = self.inversion.min(notes.len().saturating_sub(1));
        for note in &mut notes[..turns] {
            *note = note.transpose(12).unwrap_or(*note);
        }
        notes.sort();
        notes
    }

    /// What to play for the question.
    pub fn cues(&self) -> Vec<Cue> {
        Self::block(&self.notes())
    }

    /// What to play for `quality` instead.
    pub fn cues_as(&self, quality: ChordQuality) -> Vec<Cue> {
        Self::block(&self.notes_as(quality))
    }

    /// The chord one note at a time from the bass up, each held on, so
    /// it ends as the whole chord.
    pub fn broken(&self) -> Vec<Cue> {
        let notes = self.notes();
        let end = 0.45 * notes.len() as f64 + 1.4;
        notes
            .iter()
            .enumerate()
            .map(|(i, &note)| {
                let start = 0.45 * i as f64;
                Cue::piano(start, end - start, note, 80)
            })
            .collect()
    }

    fn block(notes: &[Note]) -> Vec<Cue> {
        notes
            .iter()
            .map(|&note| Cue::piano(0.0, 2.0, note, 78))
            .collect()
    }

    /// The chord's symbol: "Ebmaj7", or "Ebmaj7/G" with G in the bass.
    pub fn symbol(&self) -> String {
        let root = self.chord.root_name();
        let name = self.chord.name_from(root);
        match self.inversion {
            0 => name,
            bass => format!("{name}/{}", self.chord.spell(root)[bass]),
        }
    }

    /// The right answer, explained: the chord's symbol and name, which
    /// note is in the bass, its notes, the thirds it is stacked from,
    /// and what gives it its sound.
    pub fn explain(&self) -> String {
        let quality = self.chord.quality;
        let root = self.chord.root_name();
        let names = self.chord.spell(root);
        let position = match self.inversion {
            0 => "root position".to_string(),
            bass => format!("{}, with {} in the bass", INVERSIONS[bass], names[bass]),
        };
        let notes: Vec<String> = names.iter().map(NoteName::to_string).collect();
        // A note written with another letter than its key's usual names
        // (Cb for the B key, E# for F, Bbb for A) gets a word on why.
        let odd: Vec<String> = names
            .iter()
            .zip(quality.tones())
            .filter(|(name, _)| {
                let key = name.pitch_class();
                name.letter != NoteName::plain(key, Spelling::Sharps).letter
                    && name.letter != NoteName::plain(key, Spelling::Flats).letter
            })
            .map(|(name, &tone)| {
                let letters: Vec<String> = (0..tone)
                    .map(|up| root.letter.up(i32::from(up)).as_char().to_string())
                    .collect();
                format!(
                    "{name} is the {} key, written {name} because it is the chord's {}: \
                     {} letters from {} ({}).",
                    NoteName::chromatic(name.pitch_class()),
                    ORDINALS[usize::from(tone)],
                    NUMBERS[usize::from(tone)],
                    root.letter.as_char(),
                    letters.join(" ")
                )
            })
            .collect();
        let thirds: Vec<String> = quality
            .intervals()
            .windows(2)
            .map(|pair| {
                format!(
                    "{} {}",
                    article(interval_name(pair[1] - pair[0])),
                    interval_name(pair[1] - pair[0])
                )
            })
            .collect();
        let mut text = format!(
            "{}: {root} {}, {position}. {}: {} stacked up from {root}. ",
            self.symbol(),
            quality.name(),
            notes.join(" "),
            listed(&thirds),
        );
        for line in odd {
            text += &line;
            text += " ";
        }
        // These two split the octave in equal parts, so an inversion has
        // the same keys as the chord in root position on its bass note:
        // the ear cannot tell which note is the root.
        if matches!(quality, ChordQuality::Augmented | ChordQuality::Diminished7)
            && self.inversion > 0
        {
            let bass = NoteName::chromatic(names[self.inversion].pitch_class());
            let same = Chord::new(bass.pitch_class(), quality).name_from(bass);
            text += &format!(
                "By ear it is also {same} in root position: its notes are evenly spaced \
                 around the octave, so any of them can be heard as the root. "
            );
        }
        text + chord_sound(quality)
    }
}

// ----------------------------------------------------------------------
// Progressions

/// Which keys the progressions are in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum KeyChoice {
    #[default]
    Major,
    Minor,
    Both,
}

impl KeyChoice {
    pub const ALL: [KeyChoice; 3] = [KeyChoice::Major, KeyChoice::Minor, KeyChoice::Both];

    pub fn name(self) -> &'static str {
        match self {
            KeyChoice::Major => "Major",
            KeyChoice::Minor => "Minor",
            KeyChoice::Both => "Both",
        }
    }
}

/// Which chords of the key the progressions use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ChordPool {
    /// The three chords many songs are made of: I, IV and V.
    #[default]
    Three,
    /// Those and the two minor chords most often with them, ii and vi
    /// (in a minor key, ii° and bVI).
    Five,
    /// All seven chords of the key.
    Seven,
}

impl ChordPool {
    pub const ALL: [ChordPool; 3] = [ChordPool::Three, ChordPool::Five, ChordPool::Seven];

    /// The degrees of the scale the chords are built on (0 is I).
    pub fn degrees(self) -> &'static [usize] {
        match self {
            ChordPool::Three => &[0, 3, 4],
            ChordPool::Five => &[0, 1, 3, 4, 5],
            ChordPool::Seven => &[0, 1, 2, 3, 4, 5, 6],
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            ChordPool::Three => "I IV V",
            ChordPool::Five => "I ii IV V vi",
            ChordPool::Seven => "All seven",
        }
    }
}

/// Who plays a progression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Players {
    /// The piano alone, with the root in its left hand.
    Piano,
    /// The piano, and the bass playing each chord's root.
    PianoAndBass,
    /// Piano, bass and drums.
    #[default]
    Band,
}

impl Players {
    pub const ALL: [Players; 3] = [Players::Piano, Players::PianoAndBass, Players::Band];

    pub fn name(self) -> &'static str {
        match self {
            Players::Piano => "Piano",
            Players::PianoAndBass => "Piano and bass",
            Players::Band => "Band",
        }
    }
}

/// How the progressions are made and played.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProgressionSettings {
    pub keys: KeyChoice,
    pub chords: ChordPool,
    /// Seventh chords (V7, ii7...) instead of triads.
    pub sevenths: bool,
    pub players: Players,
    /// Beats per minute; each chord lasts a bar of four beats.
    pub tempo: u16,
    /// How many chords, the first one (I) included.
    pub length: usize,
}

impl Default for ProgressionSettings {
    fn default() -> Self {
        ProgressionSettings {
            keys: KeyChoice::Major,
            chords: ChordPool::Three,
            sevenths: false,
            players: Players::Band,
            tempo: 100,
            length: 4,
        }
    }
}

/// The shortest and longest progressions asked.
pub const PROGRESSION_LENGTHS: [usize; 2] = [4, 8];

/// The tempos a progression can be played at.
pub const PROGRESSION_TEMPOS: std::ops::RangeInclusive<u16> = 60..=160;

/// A few chords in a key to name, starting at home.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressionQuestion {
    pub scale: Scale,
    /// The seven harmony keys of the key, I to vii, as the answers.
    pub keys: Vec<ChordKey>,
    /// The chords played, by degree (0 is I). The first is always I.
    pub degrees: Vec<usize>,
    /// The degrees the answers come from.
    pub pool: Vec<usize>,
}

impl ProgressionQuestion {
    /// A progression from `seed`.
    ///
    /// It starts at home (I) and mostly goes on by the moves music makes
    /// most often (see [`next_chords`]), the most usual ones most often,
    /// using only the chords chosen and never the same chord twice in a
    /// row. It goes home again only at the end, and ends on I or V: a
    /// full stop, or a question waiting for an answer.
    pub fn new(settings: &ProgressionSettings, seed: u64) -> Self {
        let mut random = Random(seed);
        let minor = match settings.keys {
            KeyChoice::Major => false,
            KeyChoice::Minor => true,
            KeyChoice::Both => random.below(2) == 1,
        };
        let kind = if minor {
            ScaleKind::NaturalMinor
        } else {
            ScaleKind::Major
        };
        let scale = Scale::new(PitchClass::from_semitone(random.below(12) as u8), kind);
        let size = if settings.sevenths {
            ChordSize::Sevenths
        } else {
            ChordSize::Triads
        };
        let keys = keys::chord_keys(scale, size);
        let triads = keys::chord_keys(scale, ChordSize::Triads);
        let pool = settings.chords.degrees().to_vec();
        let length = settings.length.clamp(2, 16);

        let mut degrees = vec![0];
        for place in 1..length {
            let current = degrees[place - 1];
            let last = place == length - 1;
            // The moves from here, the most usual first, to chords
            // chosen; home only at the end.
            let mut moves: Vec<usize> = Vec::new();
            for suggestion in next_chords(scale, triads[current].chord) {
                let degree = suggestion.degree;
                if pool.contains(&degree) && degree != current && !moves.contains(&degree) {
                    moves.push(degree);
                }
            }
            let fits = |degree: &usize| {
                if last {
                    matches!(degree, 0 | 4)
                } else {
                    *degree != 0
                }
            };
            let wanted: Vec<usize> = moves.iter().copied().filter(fits).collect();
            // Now and then any chord chosen, so that chords the usual
            // moves seldom reach (iii, vii°) are heard too.
            let others: Vec<usize> = pool
                .iter()
                .copied()
                .filter(|&degree| degree != current && degree != 0)
                .collect();
            let next = if !last && (wanted.is_empty() || random.below(4) == 0) {
                others[random.below(others.len())]
            } else if !wanted.is_empty() {
                // The first move is the most usual: it gets the most weight.
                let weights: Vec<usize> = (1..=wanted.len()).rev().collect();
                let mut pick = random.below(weights.iter().sum());
                let mut chosen = wanted[0];
                for (degree, weight) in wanted.iter().zip(&weights) {
                    if pick < *weight {
                        chosen = *degree;
                        break;
                    }
                    pick -= weight;
                }
                chosen
            } else if current == 4 {
                0
            } else {
                4
            };
            degrees.push(next);
        }
        ProgressionQuestion {
            scale,
            keys,
            degrees,
            pool,
        }
    }

    /// The chord played at `place`.
    pub fn chord(&self, place: usize) -> &ChordKey {
        &self.keys[self.degrees[place]]
    }

    /// How long each chord lasts at `tempo`: a bar of four beats.
    pub fn bar_seconds(tempo: u16) -> f64 {
        4.0 * 60.0 / f64::from(tempo.max(1))
    }

    /// What the players play: each chord for a bar. The piano plays the
    /// chord on beats 1 and 3, each voiced close to the one before, as
    /// a pianist moves from chord to chord. The bass plays the root on
    /// the same beats. The drums keep time: bass drum on 1 and 3, snare
    /// on 2 and 4, hi-hat on every eighth. The last chord is held for
    /// its whole bar, over one crash.
    pub fn cues(&self, players: Players, tempo: u16) -> Vec<Cue> {
        let bar = Self::bar_seconds(tempo);
        let beat = bar / 4.0;
        let mut cues = Vec::new();
        let mut previous: Option<Vec<Note>> = None;
        for (place, &degree) in self.degrees.iter().enumerate() {
            let chord = self.keys[degree].chord;
            let (voicing, with_root) = keys::play(chord, previous.as_deref());
            let start = place as f64 * bar;
            let last = place == self.degrees.len() - 1;
            let hits: &[(f64, f64, u8)] = if last {
                &[(0.0, 4.0, 84)]
            } else {
                &[(0.0, 2.0, 84), (2.0, 2.0, 70)]
            };
            let piano = if players == Players::Piano {
                &with_root
            } else {
                &voicing
            };
            for &(on, beats, velocity) in hits {
                for &note in piano {
                    cues.push(Cue::piano(
                        start + on * beat,
                        beats * beat - 0.04,
                        note,
                        velocity,
                    ));
                }
                if players != Players::Piano {
                    cues.push(Cue {
                        part: Part::Bass,
                        start: start + on * beat,
                        length: beats * beat - 0.06,
                        note: bass_root(chord),
                        velocity: 96,
                    });
                }
            }
            if players == Players::Band {
                cues.extend(drums(start, beat, last));
            }
            previous = Some(voicing);
        }
        cues.sort_by(|a, b| a.start.total_cmp(&b.start));
        cues
    }

    /// The chord of the harmony key `degree` on the piano, with its root
    /// below: to hear any chord of the key after answering.
    pub fn chord_cues(&self, degree: usize) -> Vec<Cue> {
        let Some(key) = self.keys.get(degree) else {
            return Vec::new();
        };
        let (_, notes) = keys::play(key.chord, None);
        notes
            .into_iter()
            .map(|note| Cue::piano(0.0, 1.6, note, 80))
            .collect()
    }

    /// The key itself, to hear where home is: I, IV, V and I again (i,
    /// iv, V and i in a minor key), a cadence, on the piano.
    pub fn key_cues(&self) -> Vec<Cue> {
        let mut cues = Vec::new();
        let mut previous: Option<Vec<Note>> = None;
        for (place, degree) in [0, 3, 4, 0].into_iter().enumerate() {
            let chord = self.keys[degree].chord;
            let (voicing, notes) = keys::play(chord, previous.as_deref());
            let length = if place == 3 { 1.6 } else { 0.7 };
            for note in notes {
                cues.push(Cue::piano(0.75 * place as f64, length, note, 78));
            }
            previous = Some(voicing);
        }
        cues
    }

    /// The chords as the key writes them: "G – Em – C – D".
    pub fn chord_names(&self) -> Vec<String> {
        (0..self.degrees.len())
            .map(|place| self.scale.chord_name(self.chord(place).chord))
            .collect()
    }

    /// Why each chord follows the one before, as the harmony keys
    /// explain the moves: "IV to V: a step up to the dominant, ready to
    /// come home." A move the tables do not explain is only named.
    pub fn moves(&self) -> Vec<String> {
        let triads = keys::chord_keys(self.scale, ChordSize::Triads);
        self.degrees
            .windows(2)
            .map(|pair| {
                next_chords(self.scale, triads[pair[0]].chord)
                    .into_iter()
                    .find(|suggestion| suggestion.degree == pair[1])
                    .map_or_else(
                        || {
                            format!(
                                "{} to {}.",
                                triads[pair[0]].numeral, triads[pair[1]].numeral
                            )
                        },
                        |suggestion| suggestion.reason.to_string(),
                    )
            })
            .collect()
    }
}

/// A bar of drums from `start`: time kept, or the last bar's crash.
fn drums(start: f64, beat: f64, last: bool) -> Vec<Cue> {
    let hit = |drum, at: f64, velocity| Cue {
        part: Part::Drum(drum),
        start: start + at * beat,
        length: 0.1,
        note: crate::STEP_NOTE,
        velocity,
    };
    if last {
        return vec![hit(Drum::Kick, 0.0, 96), hit(Drum::Crash, 0.0, 84)];
    }
    let mut cues = Vec::new();
    for eighth in 0..8 {
        let at = eighth as f64 / 2.0;
        cues.push(hit(Drum::HiHat, at, if eighth % 2 == 0 { 62 } else { 44 }));
    }
    cues.push(hit(Drum::Kick, 0.0, 96));
    cues.push(hit(Drum::Kick, 2.0, 88));
    cues.push(hit(Drum::Snare, 1.0, 80));
    cues.push(hit(Drum::Snare, 3.0, 80));
    cues
}

/// How many questions were answered, and how many right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Score {
    pub asked: u32,
    pub right: u32,
    /// Right answers in a row, up to now.
    pub streak: u32,
}

impl Score {
    pub fn add(&mut self, right: bool) {
        self.asked += 1;
        if right {
            self.right += 1;
            self.streak += 1;
        } else {
            self.streak = 0;
        }
    }
}

/// Everything the ear-training window remembers between runs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EarSettings {
    pub exercise: Exercise,
    pub intervals: IntervalSettings,
    pub chords: ChordSettings,
    pub progressions: ProgressionSettings,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::{Harmony, analyze};

    /// Every question of a kind made from the first `n` seeds.
    fn seeds(n: u64) -> impl Iterator<Item = u64> {
        (0..n).map(|seed| seed.wrapping_mul(0x9E37_79B9))
    }

    #[test]
    fn one_question_does_not_give_the_next_away() {
        // Asked one after another as the window asks them, the chord
        // after a C can be of any kind: the next question's numbers are
        // not the last one's, shifted.
        let settings = ChordSettings::default();
        let mut seed = 1;
        let mut after_c = std::collections::BTreeSet::new();
        let mut last = ChordQuestion::new(&settings, seed);
        for _ in 0..2_000 {
            seed = next_seed(seed);
            let question = ChordQuestion::new(&settings, seed);
            if last.chord.root == PitchClass::C {
                after_c.insert(question.chord.quality.name());
            }
            last = question;
        }
        assert_eq!(after_c.len(), 4, "{after_c:?}");
    }

    #[test]
    fn every_interval_is_spelled_as_its_name_says_from_every_note() {
        let expected = [
            &["minor second"][..],
            &["major second"],
            &["minor third"],
            &["major third"],
            &["perfect fourth"],
            &["augmented fourth", "diminished fifth"],
            &["perfect fifth"],
            &["minor sixth"],
            &["major sixth"],
            &["minor seventh"],
            &["major seventh"],
            &["octave"],
        ];
        for interval in Interval::ALL {
            for low in PitchClass::ALL {
                let (a, b) = interval.spell(low);
                assert_eq!(a.pitch_class(), low);
                assert_eq!(
                    b.pitch_class(),
                    low.transpose(i32::from(interval.semitones()))
                );
                assert!(
                    a.accidental.abs() < 2 && b.accidental.abs() < 2,
                    "{a} {b} for {}",
                    interval.name()
                );
                let name = interval.spelled_name(a, b);
                assert!(
                    expected[interval.place()].contains(&name.as_str()),
                    "{a} up to {b} is a {name}, not a {}",
                    interval.name()
                );
            }
        }
    }

    #[test]
    fn intervals_are_written_the_simple_way() {
        let spelled = |semitones, low| {
            let (a, b) = Interval::new(semitones).unwrap().spell(low);
            format!("{a} {b}")
        };
        assert_eq!(spelled(3, PitchClass::CSharp), "C# E");
        assert_eq!(spelled(4, PitchClass::CSharp), "Db F");
        assert_eq!(spelled(6, PitchClass::F), "F B");
        assert_eq!(spelled(6, PitchClass::E), "E A#");
        assert_eq!(spelled(6, PitchClass::C), "C F#");
        assert_eq!(spelled(8, PitchClass::E), "E C");
        assert_eq!(spelled(11, PitchClass::DSharp), "Eb D");
        assert_eq!(spelled(12, PitchClass::FSharp), "F# F#");
        assert_eq!(spelled(3, PitchClass::ASharp), "Bb Db");
        assert_eq!(spelled(6, PitchClass::ASharp), "Bb E");
        assert_eq!(spelled(7, PitchClass::GSharp), "Ab Eb");
    }

    #[test]
    fn interval_questions_ask_only_what_was_chosen() {
        let settings = IntervalSettings {
            intervals: Choices::of(&[0, 6]),
            directions: Choices::of(&[1, 2]),
        };
        for seed in seeds(500) {
            let question = IntervalQuestion::new(&settings, seed);
            assert!(matches!(question.interval.semitones(), 1 | 7));
            assert_ne!(question.direction, Direction::Up);
            let semitones = question.high().midi() - question.low.midi();
            assert_eq!(semitones, question.interval.semitones());
            assert!((52..=67).contains(&question.low.midi()));
        }
    }

    #[test]
    fn interval_questions_play_their_two_notes() {
        let question = IntervalQuestion {
            low: Note::new(55),
            interval: Interval::new(7).unwrap(),
            direction: Direction::Down,
        };
        let cues = question.cues();
        assert_eq!(cues[0].note.midi(), 62);
        assert_eq!(cues[1].note.midi(), 55);
        assert!(cues[1].start > cues[0].start);
        assert_eq!(
            question.explain(),
            "D4 down to G3: a perfect fifth, 7 semitones. Open and stable: the root and the \
             fifth of every major and minor chord."
        );
        // Another answer, heard down from the same top note.
        let other = question.cues_for(Interval::new(9).unwrap());
        assert_eq!(other[0].note.midi(), 62);
        assert_eq!(other[1].note.midi(), 53);
    }

    #[test]
    fn going_up_names_a_tune() {
        let question = IntervalQuestion {
            low: Note::new(60),
            interval: Interval::new(6).unwrap(),
            direction: Direction::Up,
        };
        assert_eq!(
            question.explain(),
            "C4 up to F#4: a tritone (an augmented fourth), 6 semitones. Exactly half an \
             octave, three whole steps: restless, it splits the octave in two. Like the first \
             two notes of Maria, from West Side Story."
        );
        let octave = IntervalQuestion {
            low: Note::new(59),
            interval: Interval::new(12).unwrap(),
            direction: Direction::Together,
        };
        assert!(
            octave
                .explain()
                .starts_with("B3 and B4 together: an octave, 12 semitones.")
        );
    }

    #[test]
    fn no_two_kinds_of_chord_sound_alike_on_any_root() {
        for (i, a) in CHORD_QUALITIES.iter().enumerate() {
            for b in &CHORD_QUALITIES[i + 1..] {
                for up in 0..12 {
                    assert_ne!(
                        a.pitch_set().transpose(up),
                        b.pitch_set(),
                        "{} and {}",
                        a.name(),
                        b.name()
                    );
                }
            }
        }
    }

    #[test]
    fn chord_questions_play_the_chord_with_the_right_bass() {
        let settings = ChordSettings {
            qualities: Choices::of(&(0..9).collect::<Vec<_>>()),
            inversions: true,
        };
        let mut inversions = [0; 4];
        for seed in seeds(2000) {
            let question = ChordQuestion::new(&settings, seed);
            let notes = question.notes();
            inversions[question.inversion] += 1;
            let pitch_set = notes.iter().map(|note| note.pitch_class()).collect();
            assert_eq!(question.chord.pitch_set(), pitch_set);
            let tones = question.chord.quality.intervals();
            let bass = question
                .chord
                .root
                .transpose(i32::from(tones[question.inversion]));
            assert_eq!(notes[0].pitch_class(), bass, "{}", question.symbol());
        }
        assert!(inversions.iter().all(|&count| count > 0));
    }

    #[test]
    fn the_app_names_a_chord_in_root_position_as_the_question_does() {
        let settings = ChordSettings {
            qualities: Choices::of(&(0..9).collect::<Vec<_>>()),
            inversions: false,
        };
        for seed in seeds(500) {
            let question = ChordQuestion::new(&settings, seed);
            // Augmented and diminished seventh chords have the same notes
            // on other roots, so only their kind and notes must match.
            let Harmony::Chord { chord, .. } = analyze(question.notes()) else {
                panic!("{} is not heard as a chord", question.symbol());
            };
            assert_eq!(
                chord.quality,
                question.chord.quality,
                "{}",
                question.symbol()
            );
            assert_eq!(chord.pitch_set(), question.chord.pitch_set());
        }
    }

    #[test]
    fn chords_are_spelled_from_their_root() {
        let question = |root, quality, inversion| ChordQuestion {
            chord: Chord::new(root, quality),
            inversion,
            root_note: Note::new(48 + PitchClass::semitone(root)),
        };
        assert_eq!(
            question(PitchClass::B, ChordQuality::Diminished7, 0).explain(),
            "Bdim7: B diminished seventh, root position. B D F Ab: a minor third, a minor \
             third and a minor third stacked up from B. Minor thirds all the way up, \
             splitting the octave in four equal parts: the vii°7 of harmonic minor."
        );
        assert_eq!(
            question(PitchClass::DSharp, ChordQuality::Major7, 1).explain(),
            "Ebmaj7/G: Eb major seventh, first inversion, with G in the bass. Eb G Bb D: a \
             major third, a minor third and a major third stacked up from Eb. A major chord \
             with a major seventh: soft and dreamy, the Imaj7 and IVmaj7 of a major key."
        );
        assert_eq!(
            question(PitchClass::GSharp, ChordQuality::Augmented, 0).symbol(),
            "Abaug"
        );
        assert_eq!(
            question(PitchClass::F, ChordQuality::Diminished, 2).explain(),
            "Fdim/Cb: F diminished, second inversion, with Cb in the bass. F Ab Cb: a minor \
             third and a minor third stacked up from F. Cb is the B key, written Cb because \
             it is the chord's fifth: five letters from F (F G A B C). Tense: two minor \
             thirds, so its fifth is a semitone lower than a minor chord's."
        );
        assert!(
            question(PitchClass::C, ChordQuality::Diminished7, 0)
                .explain()
                .contains(
                    "Bbb is the A key, written Bbb because it is the chord's seventh: seven \
                     letters from C (C D E F G A B)."
                )
        );
        assert_eq!(
            question(PitchClass::CSharp, ChordQuality::Minor, 2).symbol(),
            "C#m/G#"
        );
        // An inverted augmented or diminished seventh chord has the same
        // keys as the one on its bass note, so the text says so.
        assert!(
            question(PitchClass::C, ChordQuality::Augmented, 1)
                .explain()
                .contains("By ear it is also Eaug in root position: its notes are evenly spaced")
        );
        assert!(
            question(PitchClass::D, ChordQuality::Diminished7, 3)
                .explain()
                .contains("Ddim7/Cb: D diminished seventh, third inversion, with Cb in the bass.")
        );
        assert!(
            question(PitchClass::D, ChordQuality::Diminished7, 3)
                .explain()
                .contains("By ear it is also Bdim7 in root position")
        );
        assert!(
            !question(PitchClass::C, ChordQuality::Augmented, 0)
                .explain()
                .contains("By ear")
        );
        assert!(
            !question(PitchClass::C, ChordQuality::Major, 1)
                .explain()
                .contains("By ear")
        );
        // Every inverted one names a chord with the same keys.
        for root in 0..12 {
            for quality in [ChordQuality::Augmented, ChordQuality::Diminished7] {
                for inversion in 1..quality.intervals().len() {
                    let asked = question(PitchClass::from_semitone(root), quality, inversion);
                    let bass = asked.notes()[0].pitch_class();
                    assert_eq!(
                        Chord::new(bass, quality).pitch_set(),
                        asked.chord.pitch_set(),
                        "{}",
                        asked.symbol()
                    );
                }
            }
        }
        let first = question(PitchClass::A, ChordQuality::Major, 1);
        let notes: Vec<u8> = first.notes().iter().map(|n| n.midi()).collect();
        assert_eq!(notes, [61, 64, 69]);
        // Compared with minor, the same note of the chord stays in the bass.
        let minor: Vec<u8> = first
            .notes_as(ChordQuality::Minor)
            .iter()
            .map(|n| n.midi())
            .collect();
        assert_eq!(minor, [60, 64, 69]);
    }

    #[test]
    fn numerals_are_those_of_the_harmony_keys() {
        let numerals = |kind, sevenths| {
            let settings = ProgressionSettings {
                keys: kind,
                sevenths,
                ..Default::default()
            };
            let question = ProgressionQuestion::new(&settings, 7);
            question
                .keys
                .iter()
                .map(|key| key.numeral.clone())
                .collect::<Vec<_>>()
                .join(" ")
        };
        assert_eq!(numerals(KeyChoice::Major, false), "I ii iii IV V vi vii°");
        assert_eq!(
            numerals(KeyChoice::Minor, false),
            "i ii° bIII iv V bVI bVII"
        );
        assert_eq!(
            numerals(KeyChoice::Major, true),
            "Imaj7 ii7 iii7 IVmaj7 V7 vi7 viiø7"
        );
        assert_eq!(
            numerals(KeyChoice::Minor, true),
            "i7 iiø7 bIIImaj7 iv7 V7 bVImaj7 bVII7"
        );
    }

    #[test]
    fn progressions_start_at_home_and_end_on_i_or_v() {
        for chords in ChordPool::ALL {
            for keys in KeyChoice::ALL {
                for length in PROGRESSION_LENGTHS {
                    let settings = ProgressionSettings {
                        keys,
                        chords,
                        length,
                        ..Default::default()
                    };
                    for seed in seeds(300) {
                        let question = ProgressionQuestion::new(&settings, seed);
                        let degrees = &question.degrees;
                        assert_eq!(degrees.len(), length);
                        assert_eq!(degrees[0], 0);
                        assert!(matches!(degrees[length - 1], 0 | 4), "{degrees:?}");
                        assert!(
                            degrees[1..length - 1].iter().all(|&d| d != 0),
                            "{degrees:?}"
                        );
                        assert!(degrees.windows(2).all(|pair| pair[0] != pair[1]));
                        assert!(degrees.iter().all(|d| chords.degrees().contains(d)));
                    }
                }
            }
        }
    }

    #[test]
    fn progressions_use_every_chord_chosen() {
        let settings = ProgressionSettings {
            chords: ChordPool::Seven,
            length: 8,
            ..Default::default()
        };
        let mut used = [false; 7];
        for seed in seeds(300) {
            for degree in ProgressionQuestion::new(&settings, seed).degrees {
                used[degree] = true;
            }
        }
        assert_eq!(used, [true; 7]);
    }

    #[test]
    fn each_bar_plays_its_chord_over_its_root() {
        let settings = ProgressionSettings {
            keys: KeyChoice::Both,
            chords: ChordPool::Seven,
            sevenths: true,
            ..Default::default()
        };
        for seed in seeds(200) {
            let question = ProgressionQuestion::new(&settings, seed);
            let bar = ProgressionQuestion::bar_seconds(settings.tempo);
            let cues = question.cues(Players::Band, settings.tempo);
            assert!(cues.windows(2).all(|pair| pair[0].start <= pair[1].start));
            for place in 0..question.degrees.len() {
                let chord = question.chord(place).chord;
                let in_bar = |cue: &&Cue| {
                    cue.start >= place as f64 * bar && cue.start < (place + 1) as f64 * bar
                };
                let piano: Vec<Note> = cues
                    .iter()
                    .filter(in_bar)
                    .filter(|cue| cue.part == Part::Piano && cue.start == place as f64 * bar)
                    .map(|cue| cue.note)
                    .collect();
                let pitch_set = piano.iter().map(|note| note.pitch_class()).collect();
                assert_eq!(chord.pitch_set(), pitch_set);
                let bass: Vec<&Cue> = cues
                    .iter()
                    .filter(in_bar)
                    .filter(|cue| cue.part == Part::Bass)
                    .collect();
                assert!(!bass.is_empty());
                assert!(bass.iter().all(|cue| cue.note.pitch_class() == chord.root));
                let lowest = piano.iter().min().unwrap();
                assert!(bass.iter().all(|cue| cue.note < *lowest));
            }
        }
    }

    #[test]
    fn the_piano_alone_plays_the_root_itself() {
        let settings = ProgressionSettings::default();
        let question = ProgressionQuestion::new(&settings, 3);
        let cues = question.cues(Players::Piano, 100);
        assert!(cues.iter().all(|cue| cue.part == Part::Piano));
        let first: Vec<Note> = cues
            .iter()
            .filter(|cue| cue.start == 0.0)
            .map(|cue| cue.note)
            .collect();
        let lowest = first.iter().min().unwrap();
        assert_eq!(lowest.pitch_class(), question.scale.root);
        assert!(matches!(
            analyze(first.iter().copied()),
            Harmony::Chord { chord, .. } if chord == question.chord(0).chord
        ));
    }

    #[test]
    fn moves_are_explained() {
        let question = ProgressionQuestion {
            scale: Scale::new(PitchClass::G, ScaleKind::Major),
            keys: keys::chord_keys(
                Scale::new(PitchClass::G, ScaleKind::Major),
                ChordSize::Triads,
            ),
            degrees: vec![0, 5, 3, 4],
            pool: vec![0, 3, 4, 5],
        };
        assert_eq!(question.chord_names(), ["G", "Em", "C", "D"]);
        let moves = question.moves();
        assert_eq!(moves.len(), 3);
        assert!(moves[0].starts_with("I to vi"), "{}", moves[0]);
        assert!(moves[1].starts_with("vi to IV"), "{}", moves[1]);
        assert!(moves[2].starts_with("IV to V"), "{}", moves[2]);
    }

    #[test]
    fn the_key_is_heard_as_a_cadence() {
        let settings = ProgressionSettings {
            keys: KeyChoice::Minor,
            ..Default::default()
        };
        let question = ProgressionQuestion::new(&settings, 11);
        let cues = question.key_cues();
        let mut chords = Vec::new();
        for place in 0..4 {
            let notes = cues
                .iter()
                .filter(|cue| cue.start == 0.75 * place as f64)
                .map(|cue| cue.note);
            if let Harmony::Chord { chord, .. } = analyze(notes) {
                chords.push(question.scale.roman_numeral(chord).unwrap());
            }
        }
        assert_eq!(chords, ["i", "iv", "V", "i"]);
    }

    #[test]
    fn questions_come_in_every_key_and_on_every_note() {
        let settings = EarSettings {
            progressions: ProgressionSettings {
                keys: KeyChoice::Both,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut keys = std::collections::HashSet::new();
        let mut roots = std::collections::HashSet::new();
        let mut lows = std::collections::HashSet::new();
        let mut seed: u64 = 1_760_000_000_123_456_789;
        for _ in 0..400 {
            seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
            keys.insert(ProgressionQuestion::new(&settings.progressions, seed).scale);
            roots.insert(ChordQuestion::new(&settings.chords, seed).chord.root);
            lows.insert(IntervalQuestion::new(&settings.intervals, seed).low);
        }
        assert_eq!(keys.len(), 24);
        assert_eq!(roots.len(), 12);
        assert_eq!(lows.len(), 16);
    }

    #[test]
    fn the_same_seed_asks_the_same_question() {
        let settings = EarSettings::default();
        assert_eq!(
            IntervalQuestion::new(&settings.intervals, 42),
            IntervalQuestion::new(&settings.intervals, 42)
        );
        assert_eq!(
            ProgressionQuestion::new(&settings.progressions, 42),
            ProgressionQuestion::new(&settings.progressions, 42)
        );
    }

    #[test]
    fn the_last_choice_cannot_be_taken_away() {
        let mut choices = Choices::of(&[3]);
        choices.toggle(3);
        assert!(choices.has(3));
        choices.toggle(5);
        choices.toggle(3);
        assert!(!choices.has(3));
        assert_eq!(choices.count(), 1);
    }

    #[test]
    fn scores_count_streaks() {
        let mut score = Score::default();
        score.add(true);
        score.add(true);
        score.add(false);
        score.add(true);
        assert_eq!(
            score,
            Score {
                asked: 4,
                right: 3,
                streak: 1
            }
        );
    }

    #[test]
    fn settings_survive_a_round_trip() {
        let settings = EarSettings {
            exercise: Exercise::Progressions,
            ..Default::default()
        };
        let text = ron::to_string(&settings).unwrap();
        assert_eq!(ron::from_str::<EarSettings>(&text).unwrap(), settings);
        // Settings saved by an older version keep the new defaults.
        let old: EarSettings = ron::from_str("(exercise: Chords)").unwrap();
        assert_eq!(old.progressions, ProgressionSettings::default());
    }
}
