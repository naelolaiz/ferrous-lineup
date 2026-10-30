//! Well-known songs, with their melodies.
//!
//! Almost every song here is old enough to belong to everyone: folk
//! songs nobody owns, and pieces by composers who died long ago. A few
//! newer ones are still under copyright in some countries or in all of
//! them, such as Hit the Road Jack (1960); a comment above each says
//! so. The melodies are written in the notation of
//! [`chart::parse_melody`] and the chords as chord charts, one bar at a
//! time, exactly like a lead sheet. Each song is dressed in a style (a
//! house track, a swing band, lo-fi...) by its comping, bass and rhythm,
//! the same building blocks the harmony styles use.
//!
//! The songs live in one file per genre, in the `song/` folder next to
//! this one, and [`SONGS`] lists them all. Songs marked "Written for
//! Ferrous Lineup" are new tunes, written to show off a genre that has
//! few well-known tunes old enough to be free.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::general_midi as gm;
use crate::harmony::{Scale, ScaleKind};
use crate::meter::TimeSignature;
use crate::pattern::{DEFAULT_VELOCITY, NoteEvent, STEP_NOTE};
use crate::pitch::{NoteName, PitchClass, Spelling};
use crate::project::Drum;

use super::Genre;
use super::chart::{self, ChordSpan, Melody};
use super::comping::{Bass, Comping};
use super::rhythm::Rhythm;

/// One section of a song, such as a verse or a chorus. Each part
/// becomes one pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub name: &'static str,
    /// The tune, played by the Lead channel.
    pub melody: &'static str,
    /// The chords, accompanied in the song's comping and bass style, and
    /// written on the chord sheet. A chart may name them as numerals,
    /// "i | V7", or by name as a lead sheet does, "C#m | G#7/B#", read in
    /// the part's key (see [`chart::parse_chart_in`]).
    pub chords: &'static str,
    /// Notes for the Keys channel written out in full, for pieces whose
    /// accompaniment is part of the music. When there are any, the
    /// chords are not accompanied.
    pub left: &'static str,
    /// Who sings the melody, and from which bar of the part: one
    /// `(voice, bar)` for each copy. One voice is `[(1, 0)]`. In a round
    /// or a canon, each voice sings the same tune a few bars after the
    /// one before: `[(1, 0), (2, 2), (3, 4)]`. A negative bar is a voice
    /// that began in the part before: only its end is heard. Notes
    /// outside the part are left out. Voice 1 is the Lead channel; every
    /// other voice gets a channel of its own, "Voice 2", "Voice 3"...
    pub voices: &'static [(usize, i32)],
    /// A time signature of its own, for a part in another time than the
    /// rest of the song. The demo puts a time marker where it starts.
    pub time: Option<TimeSignature>,
    /// The time signature of each bar, for music whose meter changes
    /// from bar to bar, as in Mussorgsky's Promenade: 5/4, 6/4, 5/4...
    /// Empty when every bar is in the part's time. The demo puts a time
    /// marker wherever the meter changes, and a pattern for each run of
    /// bars in one time signature, since a pattern has one.
    pub meters: &'static [TimeSignature],
    /// The key this part is in, when it is not the song's. Its chords
    /// are read in this key, so a piece that modulates has a part for
    /// each key it visits.
    pub key: Option<(PitchClass, ScaleKind)>,
    /// The tempo markings of the score inside this part, in order. The
    /// demo puts a tempo marker on the playlist for each, every time the
    /// part is played. After the part, the song carries on at the tempo
    /// it got to.
    pub tempo: &'static [TempoMark],
    /// How this part is accompanied, when it is not the way the rest of
    /// the song is: its comping, its bass line and its rhythm, a name in
    /// [`super::RHYTHMS`] ("" for no drums). A refrain in 3/4 in a song
    /// in 2/4 needs a waltz's comping and a rhythm in three.
    pub played: Option<(Comping, Bass, &'static str)>,
    /// The instruments that play the melody in this part, when it is not
    /// the band's lead: each a General MIDI program and how many
    /// semitones higher it plays the tune (12 is an octave up, 19 a
    /// twelfth). An orchestra hands its tune from one instrument to
    /// another, as Ravel's Boléro does from the flute to the clarinet,
    /// and doubles it an octave or two higher. Each instrument gets a
    /// channel of its own, named after it. Empty is the Lead channel,
    /// playing the tune as written. In a round, only the first voice
    /// moves.
    pub instruments: &'static [(u8, i8)],
    /// How loud the part is played (see [`Dynamic`]). The marks in the
    /// melody, such as `!p` or `!<` (see [`chart::parse_melody`]), change
    /// the loudness note by note from there, and the band follows them.
    pub dynamic: Dynamic,
    /// A drum part written out in full, for music whose drumming is part
    /// of the score, such as the snare drum of Ravel's Boléro: a line for
    /// each drum, in the notation of melodies (see
    /// [`chart::parse_melody`]), with `x` for a stroke and `X` for an
    /// accented one: `"x:2 (3 x:1 x x"` is an eighth note and a triplet
    /// of sixteenths. A melody's dynamic marks, such as `!p` or `!<`,
    /// work here too. When there is one, the song's rhythm is not played
    /// in this part. A part whose meter changes bar by bar has none.
    pub drums: &'static [(Drum, &'static str)],
    /// The other instruments' parts, written out in full, each on a
    /// channel of its own (see [`Line`]): a choir humming under the
    /// soloists, a second voice singing a third below the first, a
    /// guitar, a double bass. A part with any is written out, as one with
    /// a left hand is: its chords are not accompanied. A part whose
    /// meter changes bar by bar has none.
    pub lines: &'static [Line],
}

/// One instrument's part, written out in full beside the tune: the name
/// of the channel it plays on, the General MIDI program that plays it,
/// and its notes, in the notation of melodies (see
/// [`chart::parse_melody`]), where `&` starts another voice on the same
/// channel, as the four voices of a choir. Lines with the same name, in
/// any part of the song, play on the same channel, with the program of
/// the first. "Voice 2", "Voice 3" and so on are the channels of a
/// round's voices (see [`Part::voices`]), so a second soloist is called
/// something else, such as "Soloist 2".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub name: &'static str,
    pub program: u8,
    pub notes: &'static str,
}

/// A line called `name`, played by `program`.
pub const fn line(name: &'static str, program: u8, notes: &'static str) -> Line {
    Line {
        name,
        program,
        notes,
    }
}

impl Line {
    pub fn melody(&self) -> Melody {
        chart::parse_melody(self.notes).unwrap_or_default()
    }
}

/// How loud a part is played, as a score marks it, from pianissimo (pp,
/// very soft) to fortissimo (ff, very loud). Mezzo-forte (mf, moderately
/// loud) is how a part is played when nothing says otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dynamic {
    /// pp, very soft.
    Pianissimo,
    /// p, soft.
    Piano,
    /// mp, moderately soft.
    MezzoPiano,
    /// mf, moderately loud.
    MezzoForte,
    /// f, loud.
    Forte,
    /// ff, very loud.
    Fortissimo,
}

impl Dynamic {
    /// From the softest to the loudest.
    pub const ALL: [Dynamic; 6] = [
        Dynamic::Pianissimo,
        Dynamic::Piano,
        Dynamic::MezzoPiano,
        Dynamic::MezzoForte,
        Dynamic::Forte,
        Dynamic::Fortissimo,
    ];

    /// The dynamic a score writes as `mark`: "pp", "p", "mp", "mf", "f"
    /// or "ff". The rarer "ppp" and "fff" are read as pp and ff, the
    /// softest and the loudest there are here.
    pub fn named(mark: &str) -> Option<Dynamic> {
        match mark {
            "ppp" | "pp" => Some(Dynamic::Pianissimo),
            "p" => Some(Dynamic::Piano),
            "mp" => Some(Dynamic::MezzoPiano),
            "mf" => Some(Dynamic::MezzoForte),
            "f" => Some(Dynamic::Forte),
            "fff" | "ff" => Some(Dynamic::Fortissimo),
            _ => None,
        }
    }

    /// The dynamic `levels` marks louder, or softer if `levels` is
    /// negative, going no further than pp and ff: two louder than mp is f.
    pub fn louder(self, levels: isize) -> Dynamic {
        let here = Dynamic::ALL.iter().position(|&dynamic| dynamic == self);
        let there = here.unwrap_or(0).saturating_add_signed(levels);
        Dynamic::ALL[there.min(Dynamic::ALL.len() - 1)]
    }

    /// How much harder than at mezzo-forte the notes are struck: what
    /// their velocities are multiplied by.
    pub fn scale(self) -> f32 {
        match self {
            Dynamic::Pianissimo => 0.4,
            Dynamic::Piano => 0.55,
            Dynamic::MezzoPiano => 0.75,
            Dynamic::MezzoForte => 1.0,
            Dynamic::Forte => 1.15,
            Dynamic::Fortissimo => 1.3,
        }
    }

    /// Plays `notes` this loud: their velocities scaled, from 1 to 127.
    pub fn mark(self, notes: &mut [NoteEvent]) {
        if self == Dynamic::MezzoForte {
            return;
        }
        for note in notes {
            let velocity = (f32::from(note.velocity) * self.scale()).round();
            note.velocity = velocity.clamp(1.0, 127.0) as u8;
        }
    }
}

/// The General MIDI instruments a song's channels play (see
/// [`crate::general_midi`]): the melody (the Lead channel, and every other
/// voice of a round or a canon), the chords or written-out accompaniment
/// (Keys) and the bass, and a big band's horn section. A piece from the
/// concert hall gets the instruments it was written for, and a song
/// dressed in a style gets that style's band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Band {
    pub lead: u8,
    pub keys: u8,
    pub bass: u8,
    pub horns: Option<Horns>,
}

/// A horn section behind the melody, on a channel of its own ("Horns",
/// or "Strings" for a string section): it plays the song's chords, voiced
/// as the Keys channel voices them, in a comping of its own. A big band's
/// saxophones hold the chords under the trumpet's tune
/// ([`Comping::Held`]), and its brass punch them on the beat and just
/// before beat 3 ([`Comping::Charleston`]). A tango orchestra's violins
/// hold them under the bandoneones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Horns {
    pub program: u8,
    pub comping: Comping,
}

impl Band {
    const fn new(lead: u8, keys: u8, bass: u8) -> Band {
        Band {
            lead,
            keys,
            bass,
            horns: None,
        }
    }

    /// One instrument playing everything, such as a piano piece.
    const fn solo(instrument: u8) -> Band {
        Band::new(instrument, instrument, instrument)
    }

    /// The same band with a horn section playing `program` in `comping`.
    const fn with_horns(self, program: u8, comping: Comping) -> Band {
        Band {
            horns: Some(Horns { program, comping }),
            ..self
        }
    }
}

/// A tempo marking in a score, inside a part. `at` counts steps from the
/// start of the part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempoMark {
    /// From `at` on, `bpm` beats a minute, at once: a new section with
    /// a new tempo, such as "Più mosso" (more movement), "Adagio" or
    /// "Tempo I" (the first tempo again).
    At { at: usize, bpm: u16 },
    /// From `at`, slowing down or speeding up bit by bit until `bpm`,
    /// reached `over` steps later: "rit." or "rall." (slowing down),
    /// "accel." or "stringendo" (speeding up).
    Glide { at: usize, bpm: u16, over: usize },
    /// A fermata (𝄐), the "bird's eye" over a note or a rest: from `at`,
    /// for `steps` steps, the music waits, lasting twice as long; then
    /// it carries on at the tempo before.
    Fermata { at: usize, steps: usize },
}

/// A new tempo from `at`, at once.
const fn tempo(at: usize, bpm: u16) -> TempoMark {
    TempoMark::At { at, bpm }
}

/// A gradual change from `at`, reaching `bpm` after `over` steps.
const fn glide(at: usize, bpm: u16, over: usize) -> TempoMark {
    TempoMark::Glide { at, bpm, over }
}

/// A fermata from `at`, `steps` steps long.
const fn fermata(at: usize, steps: usize) -> TempoMark {
    TempoMark::Fermata { at, steps }
}

impl Part {
    /// A part with one voice and no written-out accompaniment.
    const fn new(name: &'static str, melody: &'static str, chords: &'static str) -> Part {
        Part {
            name,
            melody,
            chords,
            left: "",
            voices: &[(1, 0)],
            time: None,
            meters: &[],
            key: None,
            tempo: &[],
            played: None,
            instruments: &[],
            dynamic: Dynamic::MezzoForte,
            drums: &[],
            lines: &[],
        }
    }

    const fn with_left(self, left: &'static str) -> Part {
        Part { left, ..self }
    }

    const fn with_voices(self, voices: &'static [(usize, i32)]) -> Part {
        Part { voices, ..self }
    }

    /// The same part in another time signature than the song's, such as
    /// a verse in 3/4 in a song in 4/4.
    const fn in_time(self, time: TimeSignature) -> Part {
        Part {
            time: Some(time),
            ..self
        }
    }

    /// The same part with a time signature for each bar, for music whose
    /// meter changes from bar to bar.
    const fn in_meters(self, meters: &'static [TimeSignature]) -> Part {
        Part { meters, ..self }
    }

    /// The same part with the tempo markings of the score.
    const fn with_tempo(self, tempo: &'static [TempoMark]) -> Part {
        Part { tempo, ..self }
    }

    /// The same part accompanied another way than the rest of the song.
    const fn played(self, comping: Comping, bass: Bass, rhythm: &'static str) -> Part {
        Part {
            played: Some((comping, bass, rhythm)),
            ..self
        }
    }

    /// The same part in another key: a modulation.
    const fn in_key(self, tonic: PitchClass, mode: ScaleKind) -> Part {
        Part {
            key: Some((tonic, mode)),
            ..self
        }
    }

    /// The same part with its tune played by other instruments than the
    /// band's lead (see [`Part::instruments`]).
    pub const fn played_by(self, instruments: &'static [(u8, i8)]) -> Part {
        Part {
            instruments,
            ..self
        }
    }

    /// The same part played this loud.
    pub const fn marked(self, dynamic: Dynamic) -> Part {
        Part { dynamic, ..self }
    }

    /// The same part with its drums written out (see [`Part::drums`]).
    pub const fn with_drums(self, drums: &'static [(Drum, &'static str)]) -> Part {
        Part { drums, ..self }
    }

    /// The same part with other instruments' parts written out (see
    /// [`Part::lines`]).
    pub const fn with_lines(self, lines: &'static [Line]) -> Part {
        Part { lines, ..self }
    }

    /// Whether its accompaniment is written out, in a left hand or in
    /// lines of other instruments, so its chords are not accompanied.
    pub fn written_out(&self) -> bool {
        !self.left.is_empty() || !self.lines.is_empty()
    }

    pub fn melody(&self) -> Melody {
        chart::parse_melody(self.melody).unwrap_or_default()
    }

    pub fn left(&self) -> Melody {
        chart::parse_melody(self.left).unwrap_or_default()
    }

    /// The strokes of one written-out drum line (see [`Part::drums`]),
    /// on the step sequencer's note, a triplet's nudged to its tick.
    pub fn drum_hits(line: &str) -> Vec<NoteEvent> {
        drum_line(line)
            .map(|melody| melody.notes)
            .unwrap_or_default()
    }

    /// The chords of the chart, `bar` steps to the bar, in the key whose
    /// tonic is `tonic`.
    pub fn spans(&self, bar: usize, tonic: PitchClass) -> Vec<ChordSpan> {
        chart::parse_chart_in(self.chords, bar, tonic).unwrap_or_default()
    }

    /// How many steps the part lasts: the longest of its melody, its
    /// chords, its written-out accompaniment, its other instruments'
    /// lines and its drums.
    pub fn length(&self, bar: usize) -> usize {
        let chords = if self.chords.trim().is_empty() {
            0
        } else {
            self.chords.split('|').count() * bar
        };
        let drums = self
            .drums
            .iter()
            .filter_map(|(_, line)| drum_line(line))
            .map(|melody| melody.length)
            .max()
            .unwrap_or(0);
        let lines = self
            .lines
            .iter()
            .map(|line| line.melody().length)
            .max()
            .unwrap_or(0);
        self.melody()
            .length
            .max(self.left().length)
            .max(chords)
            .max(drums)
            .max(lines)
    }
}

/// How hard a drum line's stroke (`x`) and accented stroke (`X`) are
/// played, as in the rhythms' own lines.
const STROKE: u8 = 96;
const ACCENT: u8 = 120;

/// Reads a written-out drum line (see [`Part::drums`]): each stroke is
/// read as a note, C4 (the step sequencer's) or D4 if accented, so its
/// rhythm is read exactly as a melody's, then put on the step
/// sequencer's note, one step long.
fn drum_line(line: &str) -> Option<Melody> {
    let text: Vec<String> = line
        .split_whitespace()
        .map(|token| match token.split_at_checked(1) {
            Some(("x", rest)) => format!("C4{rest}"),
            Some(("X", rest)) => format!("D4{rest}"),
            _ => token.to_string(),
        })
        .collect();
    let mut melody = chart::parse_melody(&text.join(" "))?;
    for note in &mut melody.notes {
        let accented = note.pitch != STEP_NOTE;
        let written = if accented { ACCENT } else { STROKE };
        // As loud as the line's dynamic marks say, too (see
        // `chart::parse_melody`): at mezzo-forte, as written.
        let loudness = f32::from(note.velocity) / f32::from(DEFAULT_VELOCITY);
        *note = NoteEvent {
            pitch: STEP_NOTE,
            length: 1,
            trim: 0,
            velocity: (f32::from(written) * loudness).round().clamp(1.0, 127.0) as u8,
            ..*note
        };
    }
    Some(melody)
}

/// A song, its parts, and how they are put together.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Song {
    pub name: &'static str,
    /// Who wrote it and when, or where it comes from.
    pub origin: &'static str,
    /// The style it is dressed in here, such as "Swing" or "Lo-fi".
    pub feel: &'static str,
    /// The family of that style, which the Demos menu sorts songs by.
    pub genre: Genre,
    pub tonic: PitchClass,
    pub mode: ScaleKind,
    pub time: TimeSignature,
    pub tempo: f32,
    pub comping: Comping,
    pub bass: Bass,
    /// A rhythm from [`super::RHYTHMS`], or "" for no drums.
    pub rhythm: &'static str,
    /// The instruments that play it.
    pub band: Band,
    /// The style in [`super::STYLES`] whose harmony and playing the song
    /// is closest to, or "" for none. The library shows it, and the
    /// rhythm, selected while the demo is open.
    pub style: &'static str,
    pub parts: &'static [Part],
    /// The order the parts are played in, by name.
    pub form: &'static [&'static str],
    /// The drums come in at this entry of `form` (0 for the start).
    pub drums_from: usize,
    pub about: &'static str,
}

impl Song {
    /// How much the song swings (see [`crate::swing`]): as much as its
    /// rhythm does, except in folk and classical music, whose eighths
    /// stay even over any beat: Scarborough Fair keeps its even eighths
    /// over the ride cymbal of a jazz waltz.
    pub fn swing(&self) -> u8 {
        match self.genre {
            Genre::Folk | Genre::Classical => 0,
            _ => Rhythm::named(self.rhythm).map_or(0, |rhythm| rhythm.swing),
        }
    }

    /// The key the song is in, written as its tune is (see
    /// [`written_as_tune`]): Take Five is in Eb minor, not D# minor.
    pub fn scale(&self) -> Scale {
        let tune = self.parts.iter().filter(|part| part.key.is_none());
        written_as_tune(Scale::new(self.tonic, self.mode), tune)
    }

    /// The time signature `part` is in: its own, or else the song's. A
    /// part whose meter changes from bar to bar is in the time of its
    /// first bar.
    pub fn part_time(&self, part: &Part) -> TimeSignature {
        part.meters
            .first()
            .copied()
            .or(part.time)
            .unwrap_or(self.time)
    }

    /// `part` cut where its meter changes: one part for each run of bars
    /// in one time signature, each with its own time and its own share
    /// of the melody, the chords and the written-out accompaniment. A
    /// part in one time signature is a single run, itself. The pieces
    /// keep the part's name, voices and key, and its tempo markings go
    /// with the first.
    pub fn part_runs(&self, part: &Part) -> Vec<Part> {
        if part.meters.is_empty() {
            return vec![*part];
        }
        let mut runs: Vec<(usize, usize, TimeSignature)> = Vec::new();
        for (bar, &time) in part.meters.iter().enumerate() {
            match runs.last_mut() {
                Some((_, bars, last)) if *last == time => *bars += 1,
                _ => runs.push((bar, 1, time)),
            }
        }
        runs.iter()
            .enumerate()
            .map(|(index, &(first, bars, time))| Part {
                melody: bars_of(part.melody, first, bars),
                chords: bars_of(part.chords, first, bars),
                left: bars_of(part.left, first, bars),
                time: Some(time),
                meters: &[],
                tempo: if index == 0 { part.tempo } else { &[] },
                drums: &[],
                lines: &[],
                ..*part
            })
            .collect()
    }

    /// How `part` is accompanied: its comping, bass line and rhythm, its
    /// own or else the song's.
    pub fn part_playing(&self, part: &Part) -> (Comping, Bass, &'static str) {
        part.played
            .unwrap_or((self.comping, self.bass, self.rhythm))
    }

    /// The key `part` is in: its own, if it modulates, or the song's.
    pub fn part_scale(&self, part: &Part) -> Scale {
        match part.key {
            Some((tonic, mode)) => written_as_tune(Scale::new(tonic, mode), [part]),
            None => self.scale(),
        }
    }

    /// How many voices sing the melody at most: 1, or more in a round.
    pub fn voices(&self) -> usize {
        self.parts
            .iter()
            .flat_map(|part| part.voices)
            .map(|&(voice, _)| voice)
            .max()
            .unwrap_or(1)
    }

    pub fn part(&self, name: &str) -> Option<&'static Part> {
        self.parts.iter().find(|part| part.name == name)
    }

    /// The song called `name`, if there is one.
    pub fn named(name: &str) -> Option<&'static Song> {
        SONGS.iter().find(|song| song.name == name)
    }
}

/// Bars `first` to `first + count` of a melody or a chart: the text
/// between its bar lines. Text that is a slice of `'static` text lives
/// as long, so the pieces of a part are parts too.
fn bars_of(text: &'static str, first: usize, count: usize) -> &'static str {
    if text.trim().is_empty() {
        return text;
    }
    // Each voice of a written-out accompaniment (see `chart::parse_melody`)
    // has bars of its own: they are cut one by one and put together again,
    // in text made once for each piece and kept for as long as the program
    // runs, as the songs are.
    if text.contains('&') {
        type Pieces = HashMap<(usize, usize, usize, usize), &'static str>;
        static PIECES: OnceLock<Mutex<Pieces>> = OnceLock::new();
        let key = (text.as_ptr() as usize, text.len(), first, count);
        let mut pieces = PIECES
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        return pieces.entry(key).or_insert_with(|| {
            let voices: Vec<&str> = text
                .split('&')
                .map(|voice| bars_of(voice, first, count))
                .collect();
            Box::leak(voices.join(" & ").into_boxed_str())
        });
    }
    // Where each bar starts and ends, in bytes.
    let mut cuts = vec![0];
    cuts.extend(text.match_indices('|').map(|(at, _)| at));
    cuts.push(text.len());
    let start = cuts[first.min(cuts.len() - 1)];
    let end = cuts[(first + count).min(cuts.len() - 1)];
    // Leave out the bar line the piece starts with.
    text[start..end].trim_start_matches('|').trim()
}

// Every genre file starts with `use super::*;`, which brings in all the
// names below: `Song`, `Part`, the time signatures, and the notes and
// scales without their type in front, so a song can say `tonic: A`.
use PitchClass::*;
use ScaleKind::*;

const COMMON: TimeSignature = TimeSignature::COMMON;
const TWO_FOUR: TimeSignature = TimeSignature::new(2, 4);
const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);
const NINE_EIGHT: TimeSignature = TimeSignature::new(9, 8);
const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);
// Six quarter notes, each a beat, as in Mussorgsky's Promenade.
const SIX_FOUR: TimeSignature = TimeSignature::new(6, 4);
// Odd meters, with the beats they are felt in: 3 + 2 quarters, 2 + 3
// eighths...
const FIVE_FOUR: TimeSignature = TimeSignature::of_groups(&[3, 2], 4);
// Five quarter notes felt the other way round, 2 + 3: Tchaikovsky's waltz in five.
const FIVE_FOUR_TWO_THREE: TimeSignature = TimeSignature::of_groups(&[2, 3], 4);
const FIVE_EIGHT: TimeSignature = TimeSignature::of_groups(&[2, 3], 8);
// 5/8 the other way round, a long beat and then a short one, 3 + 2.
const FIVE_EIGHT_THREE_TWO: TimeSignature = TimeSignature::of_groups(&[3, 2], 8);
const THREE_TWO_TWO: TimeSignature = TimeSignature::of_groups(&[3, 2, 2], 8);
const TWO_TWO_THREE: TimeSignature = TimeSignature::of_groups(&[2, 2, 3], 8);
const NINE_EIGHT_AKSAK: TimeSignature = TimeSignature::of_groups(&[2, 2, 2, 3], 8);
const RAKS_AKSAGI: TimeSignature = TimeSignature::of_groups(&[2, 3, 2, 2], 8);
const CURCUNA: TimeSignature = TimeSignature::of_groups(&[3, 2, 2, 3], 8);
const ELEVEN_EIGHT: TimeSignature = TimeSignature::of_groups(&[2, 2, 3, 2, 2], 8);
const FIFTEEN_EIGHT: TimeSignature = TimeSignature::of_groups(&[2, 2, 2, 2, 3, 2, 2], 8);
// The short and odd bars of the Rite of Spring, for which Stravinsky marks
// no grouping: every eighth note is a beat.
const TWO_EIGHT: TimeSignature = TimeSignature::new(2, 8);
const THREE_EIGHT: TimeSignature = TimeSignature::new(3, 8);
const FOUR_EIGHT: TimeSignature = TimeSignature::new(4, 8);
const FIVE_EIGHT_UNGROUPED: TimeSignature = TimeSignature::new(5, 8);
const SEVEN_EIGHT: TimeSignature = TimeSignature::new(7, 8);
// A single beat, a bar of one quarter note (Petrushka).
const ONE_FOUR: TimeSignature = TimeSignature::new(1, 4);
// A bar of one eighth note: the upbeat of a song in 6/8 (Alfonsina y el mar).
const ONE_EIGHT: TimeSignature = TimeSignature::new(1, 8);
// Two half notes to the bar, cut time (Petrushka).
const TWO_TWO: TimeSignature = TimeSignature::new(2, 2);
// Three half notes to the bar (Petrushka).
const THREE_TWO: TimeSignature = TimeSignature::new(3, 2);
// Thirteen sixteenth notes, a single odd bar of Petrushka's third tableau.
const THIRTEEN_SIXTEEN: TimeSignature = TimeSignature::new(13, 16);

// The bands the songs are dressed in, named after their style. Each is
// melody, chords, bass. A piece from the concert hall names its own
// instruments instead.
const PIANO: Band = Band::solo(gm::ACOUSTIC_GRAND_PIANO);
const CHOIR: Band = Band::solo(gm::CHOIR_AAHS);
const SYNTHS: Band = Band::new(gm::SAW_LEAD, gm::WARM_PAD, gm::SYNTH_BASS);
const DISCO: Band = Band::new(gm::STRING_ENSEMBLE, gm::ELECTRIC_PIANO, gm::FINGER_BASS);
const BLUES_HARP: Band = Band::new(gm::HARMONICA, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const BLUES_GUITAR: Band = Band::new(gm::JAZZ_GUITAR, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
const SLOW_BLUES: Band = Band::new(gm::CLEAN_GUITAR, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const GUITARS: Band = Band::new(gm::NYLON_GUITAR, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const SONG_AND_GUITAR: Band = Band::new(gm::VOICE_OOHS, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const CAROL: Band = Band::new(gm::CHOIR_AAHS, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const FLUTE_AND_GUITAR: Band = Band::new(gm::FLUTE, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const COUNTRY: Band = Band::new(gm::FIDDLE, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
const BLUEGRASS: Band = Band::new(gm::FIDDLE, gm::BANJO, gm::ACOUSTIC_BASS);
const FOLK: Band = Band::new(gm::FLUTE, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
const JIG: Band = Band::new(gm::FIDDLE, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
const ACCORDION_BAND: Band = Band::new(gm::ACCORDION, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const BALLAD: Band = Band::new(gm::VIOLIN, gm::ACOUSTIC_GRAND_PIANO, gm::CELLO);
const KLEZMER: Band = Band::new(gm::CLARINET, gm::ACCORDION, gm::ACOUSTIC_BASS);
const BALKAN: Band = Band::new(gm::CLARINET, gm::ACCORDION, gm::ACOUSTIC_BASS);
const KAVAL: Band = Band::new(gm::FLUTE, gm::ACCORDION, gm::ACOUSTIC_BASS);
const GREEK: Band = Band::new(gm::VIOLIN, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
// The kanun, the Turkish zither, is played by a koto, the nearest General MIDI sound.
const TURKISH: Band = Band::new(gm::CLARINET, gm::KOTO, gm::ACOUSTIC_BASS);
// The kemençe of the Black Sea coast, a small fiddle played upright, by a fiddle.
const KEMENCE: Band = Band::new(gm::FIDDLE, gm::KOTO, gm::ACOUSTIC_BASS);
const POLKA: Band = Band::new(gm::VIOLIN, gm::ACCORDION, gm::TUBA);
// A Bavarian dance band: clarinet, the button accordion and the tuba.
const BAVARIAN: Band = Band::new(gm::CLARINET, gm::ACCORDION, gm::TUBA);
// A Romanian band of lăutari: the violin leads, over accordion and double bass.
const TARAF: Band = Band::new(gm::VIOLIN, gm::ACCORDION, gm::ACOUSTIC_BASS);
const ANDEAN: Band = Band::new(gm::PAN_FLUTE, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const LLANERO: Band = Band::new(gm::HARP, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const FIDDLE_AND_GUITAR: Band = Band::new(gm::VIOLIN, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const CANDOMBE: Band = Band::new(gm::NYLON_GUITAR, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const LO_FI: Band = Band::new(gm::ELECTRIC_PIANO, gm::ELECTRIC_PIANO, gm::FINGER_BASS);
const BOOM_BAP: Band = Band::new(
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_BASS,
);
const TRAP: Band = Band::new(gm::CHOIR_AAHS, gm::STRING_ENSEMBLE, gm::SYNTH_BASS);
const SWING: Band = Band::new(gm::ALTO_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
// A big band: a trumpet with the tune over the saxophones holding the
// chords; the saxophones' tune with the brass punching the chords; and a
// tenor saxophone's ballad over the trombones.
const BIG_BAND: Band = Band::new(gm::TRUMPET, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::TENOR_SAX, Comping::Held);
const SAX_SECTION: Band = Band::new(gm::ALTO_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::BRASS_SECTION, Comping::Charleston);
const BALLAD_BAND: Band = Band::new(gm::TENOR_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::TROMBONE, Comping::Held);
const DIXIELAND: Band = Band::new(gm::TRUMPET, gm::ACOUSTIC_GRAND_PIANO, gm::TUBA);
const JAZZ_WALTZ: Band = Band::new(gm::VIBRAPHONE, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
const GYPSY_JAZZ: Band = Band::new(gm::VIOLIN, gm::JAZZ_GUITAR, gm::ACOUSTIC_BASS);
/// Getz's tenor saxophone over Gilberto's guitar.
const BOSSA: Band = Band::new(gm::TENOR_SAX, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const SAMBA: Band = Band::new(gm::FLUTE, gm::NYLON_GUITAR, gm::FINGER_BASS);
const AFROBEATS: Band = Band::new(gm::MARIMBA, gm::ELECTRIC_PIANO, gm::SYNTH_BASS);
const POP: Band = Band::new(gm::SQUARE_LEAD, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const SKA: Band = Band::new(gm::TRUMPET, gm::CLEAN_GUITAR, gm::FINGER_BASS);
const REGGAE: Band = Band::new(gm::CLEAN_GUITAR, gm::DRAWBAR_ORGAN, gm::FINGER_BASS);
const ROCK: Band = Band::new(
    gm::OVERDRIVEN_GUITAR,
    gm::DISTORTION_GUITAR,
    gm::FINGER_BASS,
);
const ROCK_AND_ROLL: Band = Band::new(gm::TENOR_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
// Serú Girán: Charly García singing at the piano, Pedro Aznar on fretless bass.
const SERU: Band = Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::FRETLESS_BASS);
const BRASS_BAND: Band = Band::new(gm::TRUMPET, gm::BRASS_SECTION, gm::TUBA);
const GOSPEL: Band = Band::new(gm::CHOIR_AAHS, gm::DRAWBAR_ORGAN, gm::FINGER_BASS);
const SOUL: Band = Band::new(gm::VOICE_OOHS, gm::DRAWBAR_ORGAN, gm::FINGER_BASS);
const MOTOWN: Band = Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const FUNK: Band = Band::new(gm::BRASS_SECTION, gm::CLAVINET, gm::SLAP_BASS);
const NEO_SOUL: Band = Band::new(gm::VOICE_OOHS, gm::ELECTRIC_PIANO, gm::FINGER_BASS);
const SPIRITUAL: Band = Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
/// The orquesta típica of the 1940s: bandoneones on the tune, the violins
/// holding the chords under it, the piano and the double bass.
const TANGO: Band = Band::new(
    gm::TANGO_ACCORDION,
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_BASS,
)
.with_horns(gm::STRING_ENSEMBLE, Comping::Held);
/// The string orchestra of the anthem: violins on the tune, the section holding
/// the chords, cellos on the bass.
const STRING_ORCHESTRA: Band = Band::new(gm::VIOLIN, gm::STRING_ENSEMBLE, gm::CELLO);
const SALON: Band = Band::new(gm::VIOLIN, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
/// Trumpet, the steel strings of the Cuban tres, and the bass.
const SON: Band = Band::new(gm::TRUMPET, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
/// Trumpets, the vihuela and guitars, and the guitarrón.
const MARIACHI: Band = Band::new(gm::TRUMPET, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);

mod balkan;
mod blues;
mod classical;
mod electronic;
mod flamenco;
mod folk;
mod folklore;
mod folklore_arranged;
mod hiphop;
mod jazz;
mod latin;
mod misa_criolla;
mod pop;
mod reggae;
mod rock;
mod soul;
mod tango;
mod tango_arranged;
mod tango_played;

/// `key` written as the tune of `parts` is. A key that can be written
/// either way (see [`Scale::other_spelling`]) takes the name its tune's
/// notes are written in, since the songs here write their tunes as their
/// scores do: Take Five's tune, with its Gb and Db, is in Eb minor, not
/// D# minor, and the middle of Golliwogg's Cakewalk in Gb major. A tune
/// with as many sharps as flats, or none, leaves the key its usual name.
fn written_as_tune<'a>(key: Scale, parts: impl IntoIterator<Item = &'a Part>) -> Scale {
    let (sharps, flats) = parts
        .into_iter()
        .map(|part| accidentals(part.melody))
        .fold((0, 0), |(sharps, flats), (s, f)| (sharps + s, flats + f));
    match sharps.cmp(&flats) {
        Ordering::Greater => key.with_spelling(Spelling::Sharps),
        Ordering::Less => key.with_spelling(Spelling::Flats),
        Ordering::Equal => key,
    }
}

/// How many of the notes of `tune` (see [`chart::parse_melody`]) are
/// written with sharps, and how many with flats.
fn accidentals(tune: &str) -> (usize, usize) {
    let mut counts = (0, 0);
    for (at, c) in tune.char_indices() {
        // A note's name starts with its letter: the "b" of "Bb4" is not
        // a note B.
        if !('A'..='G').contains(&c) || tune[..at].ends_with(char::is_alphanumeric) {
            continue;
        }
        match NoteName::read(&tune[at..]).map(|(name, _)| name.accidental.signum()) {
            Some(1) => counts.0 += 1,
            Some(-1) => counts.1 += 1,
            _ => {}
        }
    }
    counts
}

/// All the songs, sorted by genre in the order of [`Genre::ALL`].
pub static SONGS: &[Song] = &[
    pop::AULD_LANG_SYNE,
    pop::LONDON_BRIDGE,
    pop::OLD_MACDONALD,
    pop::DECK_THE_HALLS,
    pop::THREE_BLIND_MICE,
    pop::SKIP_TO_MY_LOU,
    pop::POP_GOES_THE_WEASEL,
    pop::FUNICULI_FUNICULA,
    pop::AU_CLAIR_DE_LA_LUNE,
    pop::ALOUETTE,
    pop::VOGELFAENGER,
    pop::SURPRISE_SYMPHONY,
    pop::BIG_BANG_THEORY,
    rock::DRUNKEN_SAILOR,
    rock::BELLA_CIAO,
    rock::BEETHOVENS_FIFTH,
    rock::YANKEE_DOODLE,
    rock::WHISKEY_IN_THE_JAR,
    rock::BATTLE_HYMN,
    rock::WHEN_JOHNNY,
    rock::TOREADOR_SONG,
    rock::HOUSE_OF_THE_RISING_SUN,
    rock::WILLIAM_TELL,
    rock::DESARMA_Y_SANGRA,
    blues::MIDNIGHT_SHUFFLE,
    blues::ROLLING_BOOGIE,
    blues::MINOR_MOOD,
    blues::CARELESS_LOVE,
    blues::FRANKIE_AND_JOHNNY,
    blues::TROUBLE_IN_MIND,
    blues::ST_JAMES_INFIRMARY,
    blues::SEE_SEE_RIDER,
    blues::HIT_THE_ROAD_JACK,
    blues::ST_LOUIS_BLUES,
    jazz::JINGLE_BELLS,
    jazz::SAINTS,
    jazz::ENTERTAINER,
    jazz::HAPPY_BIRTHDAY,
    jazz::TWO_FIVE_WALK,
    jazz::GOLLIWOGGS_CAKEWALK,
    jazz::DARK_EYES,
    jazz::MAPLE_LEAF_RAG,
    jazz::I_GOT_RHYTHM,
    jazz::BILL_BAILEY,
    jazz::SWANEE,
    jazz::AVALON,
    jazz::BALLIN_THE_JACK,
    jazz::TWELFTH_STREET_RAG,
    jazz::DILL_PICKLES,
    jazz::PINE_APPLE_RAG,
    jazz::ELITE_SYNCOPATIONS,
    jazz::TEA_FOR_TWO,
    jazz::LADY_BE_GOOD,
    jazz::HONEYSUCKLE_ROSE,
    jazz::SOMETIMES_IM_HAPPY,
    jazz::MY_BLUE_HEAVEN,
    jazz::LIMEHOUSE_BLUES,
    jazz::JAPANESE_SANDMAN,
    jazz::LIZA,
    jazz::STRIKE_UP_THE_BAND,
    jazz::S_WONDERFUL,
    jazz::IN_THE_MOOD,
    jazz::SING_SING_SING,
    jazz::TAKE_THE_A_TRAIN,
    jazz::FASCINATING_RHYTHM,
    jazz::LOVE_ME_OR_LEAVE_ME,
    jazz::MAKIN_WHOOPEE,
    jazz::WITHOUT_A_SONG,
    jazz::EMBRACEABLE_YOU,
    jazz::BUT_NOT_FOR_ME,
    jazz::SWEET_GEORGIA_BROWN,
    jazz::AINT_MISBEHAVIN,
    jazz::IT_HAD_TO_BE_YOU,
    jazz::TIGER_RAG,
    jazz::AFTER_YOUVE_GONE,
    jazz::KING_PORTER_STOMP,
    jazz::INDIANA,
    jazz::ROSE_ROOM,
    jazz::CHARLESTON,
    jazz::GREAT_DAY,
    jazz::IF_I_COULD_BE_WITH_YOU,
    jazz::CAROLINA_IN_THE_MORNING,
    jazz::TAKE_FIVE,
    jazz::MISSION_IMPOSSIBLE,
    soul::AMAZING_GRACE,
    soul::JOY_TO_THE_WORLD,
    soul::DORIAN_STRUT,
    soul::SUNDAY_SEVENTH,
    soul::WADE_IN_THE_WATER,
    soul::SWING_LOW,
    soul::DOWN_BY_THE_RIVERSIDE,
    soul::GO_DOWN_MOSES,
    soul::JOSHUA,
    soul::MOTHERLESS_CHILD,
    soul::NOBODY_KNOWS,
    soul::MICHAEL_ROW,
    soul::HIS_EYE_IS_ON_THE_SPARROW,
    soul::DEEP_RIVER,
    soul::CLOSER_WALK,
    soul::WILL_THE_CIRCLE,
    soul::O_HAPPY_DAY,
    soul::STEAL_AWAY,
    soul::WERE_YOU_THERE,
    soul::ROLL_CALLED_UP_YONDER,
    soul::OLD_TIME_RELIGION,
    soul::NEED_OF_PRAYER,
    hiphop::TWINKLE,
    hiphop::GYMNOPEDIE,
    hiphop::PRELUDE_IN_C,
    hiphop::SAKURA,
    hiphop::DIES_IRAE,
    hiphop::REVERIE,
    hiphop::GNOSSIENNE,
    hiphop::PRELUDE_IN_E_MINOR,
    electronic::ODE_TO_JOY,
    electronic::KOROBEINIKI,
    electronic::SYMPHONY_40,
    electronic::EINE_KLEINE,
    electronic::TOCCATA,
    electronic::TURKISH_MARCH,
    electronic::KALINKA,
    electronic::CELLO_SUITE_PRELUDE,
    electronic::BADINERIE,
    electronic::SPRING,
    electronic::CAN_CAN,
    latin::RIO_AFTERNOON,
    latin::CARNIVAL_MORNING,
    latin::TICO_TICO,
    latin::EL_MANISERO,
    latin::CIELITO_LINDO,
    latin::QUIEREME_MUCHO,
    latin::SIBONEY,
    latin::APANHEI_TE_CAVAQUINHO,
    latin::ATRAENTE,
    latin::CARIOCA,
    latin::SONS_DE_CARRILHOES,
    latin::O_ABRE_ALAS,
    latin::AQUELLOS_OJOS_VERDES,
    latin::MARIA_ELENA,
    latin::TE_QUIERO_DIJISTE,
    latin::ZE_PEREIRA,
    latin::FEIJOADA_DO_BRASIL,
    latin::A_MULATINHA,
    latin::RODA_YOYO,
    latin::DONA_ADELAIDE,
    latin::GAROTA_DE_IPANEMA,
    latin::O_SAMBA_E_O_TANGO,
    folklore_arranged::QUE_NADIE_SEPA_MI_SUFRIR,
    tango::HABANERA,
    tango::TANGO_DEL_TELAR,
    tango::MILONGA_DEL_RIO,
    tango::VALS_DEL_PUERTO,
    tango::EL_CHOCLO,
    tango::LA_CUMPARSITA,
    tango::LA_PALOMA,
    tango::GALLO_CIEGO,
    tango::LAGRIMAS_Y_SONRISAS,
    tango::A_MEDIA_LUZ,
    tango::ADIOS_MUCHACHOS,
    tango::CAMINITO,
    tango::BOEDO,
    tango::ALMA_EN_PENA,
    tango::EL_AMANECER,
    tango::A_LA_GRAN_MUNECA,
    tango::POR_UNA_CABEZA,
    tango::EL_DIA_QUE_ME_QUIERAS,
    tango::VOLVER,
    tango::ROMANCE_DE_BARRIO,
    tango::DESDE_EL_ALMA,
    tango::MUCHACHO,
    tango::GARUA,
    tango::ESTA_NOCHE_DE_LUNA,
    tango_arranged::MI_NOCHE_TRISTE,
    tango_arranged::SENTIMIENTO_GAUCHO,
    tango_arranged::ZORRO_GRIS,
    tango_played::UNO,
    tango_played::SUR,
    tango_played::SOLEDAD,
    tango_played::NOSTALGIAS,
    tango_played::PASIONAL,
    tango_played::PATOTERO_SENTIMENTAL,
    tango_played::QUE_FALTA_QUE_ME_HACES,
    tango_played::QUE_ME_VAN_A_HABLAR_DE_AMOR,
    tango_played::MARIONETAS,
    tango_played::TIEMPOS_VIEJOS,
    tango_played::YIRA_YIRA,
    tango_played::LOS_MAREADOS,
    tango_played::MANO_A_MANO,
    tango_played::LA_CASITA_DE_MIS_VIEJOS,
    tango_played::LLUVIA_DE_ESTRELLAS,
    tango_played::LA_PULPERA_DE_SANTA_LUCIA,
    tango_played::MALENA,
    folklore::EL_CONDOR_PASA,
    folklore::ALMA_LLANERA,
    folklore::DOS_PALOMITAS,
    folklore::PAPEL_DE_PLATA,
    folklore::LA_QUEBRADA_DE_HUMAHUACA,
    folklore::HUAYTA_HUAYTUCHA,
    folklore::KICHARIRILLAWAY,
    folklore::A_LAS_ORILLAS_DEL_TITICACA,
    folklore::CHUNGUINADA,
    folklore::JESUSITA_EN_CHIHUAHUA,
    folklore::DE_COLORES,
    folklore::SOBRE_LAS_OLAS,
    folklore::LA_GOLONDRINA,
    folklore::ESTRELLITA,
    folklore::JARABE_TAPATIO,
    folklore::LA_RASPA,
    folklore::LA_BAMBA,
    folklore::LA_CUCARACHA,
    folklore::DIANA_TRISTE,
    folklore::LA_GATA_GOLOSA,
    folklore::PEZINHO,
    folklore::BALAIO,
    folklore::LUAR_DO_SERTAO,
    folklore::LA_LLORONA,
    folklore::LUNA_DE_XELAJU,
    folklore::LOS_MACHETES,
    folklore::BRISAS_MESANAS,
    folklore::PUNTO_OCUENO,
    folklore::BOI_BARROSO,
    folklore::CANDOMBE_PARA_GARDEL,
    folklore::LA_MARCHA_PERONISTA,
    misa_criolla::MISA_CRIOLLA_KYRIE,
    misa_criolla::MISA_CRIOLLA_GLORIA,
    misa_criolla::MISA_CRIOLLA_CREDO,
    misa_criolla::MISA_CRIOLLA_SANCTUS,
    misa_criolla::MISA_CRIOLLA_AGNUS_DEI,
    folklore::ZAMBA_DEL_TELAR,
    folklore::CHACARERA_DEL_PATIO,
    folklore::GATO_DEL_ARROYO,
    folklore::CHAMAME_DEL_ESTERO,
    folklore::CUECA_DE_LA_CALETA,
    folklore::MARINERA_DEL_CALLEJON,
    folklore::BAMBUCO_DEL_PARAMO,
    folklore::MALAMBO_DEL_FOGON,
    folklore::VIDALA_DE_LA_QUEBRADA,
    folklore::CARNAVALITO_DE_LA_QUEBRADA,
    folklore::CANDOMBE_DEL_BARRIO,
    folklore_arranged::CINCO_SIGLOS_IGUAL,
    folklore_arranged::LA_CULTURA_ES_LA_SONRISA,
    folklore_arranged::ALFONSINA_Y_EL_MAR,
    folklore_arranged::ZAMBA_DE_MI_ESPERANZA,
    folklore_arranged::ZAMBA_POR_VOS,
    folklore_arranged::BALDERRAMA,
    folklore_arranged::ACUARELA_DEL_RIO,
    flamenco::RUMBA_DEL_SUR,
    flamenco::NOCHE_FRIGIA,
    flamenco::ASTURIAS,
    balkan::AJDE_JANO,
    balkan::JOVANO_JOVANKE,
    balkan::SAMIOTISSA,
    balkan::YERAKINA,
    balkan::MILO_MOU_KOKKINO,
    balkan::TSAKONIKOS,
    balkan::PAIDUSHKO_HORO,
    balkan::SVATBARSKA_RACHENITSA,
    balkan::DAICHOVO_HORO,
    balkan::GANKINO_HORO,
    balkan::BUCIMIS,
    balkan::GEAMPARAUA_DE_LA_MURFATLAR,
    balkan::MANDRELE,
    balkan::ENTARISI,
    balkan::SARI_GELIN,
    balkan::IZMIRIN_KAVAKLARI,
    balkan::CAYELINDEN_OTEYE,
    reggae::MARY_HAD_A_LITTLE_LAMB,
    reggae::THIS_OLD_MAN,
    reggae::VALLEY_DUB,
    folk::FRERE_JACQUES,
    folk::GREENSLEEVES,
    folk::OH_SUSANNA,
    folk::SCARBOROUGH_FAIR,
    folk::CLEMENTINE,
    folk::CAMPTOWN_RACES,
    folk::ROW_YOUR_BOAT,
    folk::SILENT_NIGHT,
    folk::WE_WISH_YOU,
    folk::WILD_ROVER,
    folk::SHENANDOAH,
    folk::RED_RIVER_VALLEY,
    folk::HOME_ON_THE_RANGE,
    folk::DANNY_BOY,
    folk::LOCH_LOMOND,
    folk::WAYFARING_STRANGER,
    folk::COTTON_EYED_JOE,
    folk::OLD_JOE_CLARK,
    folk::SOLDIERS_JOY,
    folk::TURKEY_IN_THE_STRAW,
    folk::IRISH_WASHERWOMAN,
    folk::WALTZING_MATILDA,
    folk::MOLLY_MALONE,
    folk::HAVA_NAGILA,
    folk::SANTA_LUCIA,
    folk::TARANTELLA_NAPOLETANA,
    folk::TORNA_A_SURRIENTO,
    folk::ARIRANG,
    folk::TUMBALALAIKA,
    folk::MO_LI_HUA,
    folk::HUNGARIAN_DANCE,
    folk::SUSSERL,
    folklore_arranged::EL_NOI_DE_LA_MARE,
    classical::CANON,
    classical::FUR_ELISE,
    classical::MOUNTAIN_KING,
    classical::MINUET,
    classical::BLUE_DANUBE,
    classical::BRAHMS_LULLABY,
    classical::MORNING_MOOD,
    classical::NEW_WORLD,
    classical::SHCHEDRYK,
    classical::FUNERAL_MARCH,
    classical::LA_DONNA_E_MOBILE,
    classical::JUPITER_HYMN,
    classical::AVE_MARIA,
    classical::DIE_FORELLE,
    classical::PRELUDE_IN_C_SHARP_MINOR,
    classical::WEDDING_MARCH,
    classical::HALLELUJAH_CHORUS,
    classical::FOR_UNTO_US,
    classical::GREAT_GATE_OF_KIEV,
    classical::ANITRAS_DANCE,
    classical::DIES_IRAE_MOZART,
    classical::PATHETIQUE,
    classical::MOONLIGHT_SONATA,
    classical::AIR_ON_THE_G_STRING,
    classical::SHEEP_MAY_SAFELY_GRAZE,
    classical::DANSE_MACABRE,
    classical::HUMORESQUE,
    classical::TRAUMEREI,
    classical::SYMPHONY_7_ALLEGRETTO,
    classical::RADETZKY_MARCH,
    classical::PROMENADE,
    classical::PATHETIQUE_WALTZ,
    classical::HIMNO_NACIONAL_ARGENTINO,
    classical::BOLERO,
    classical::LA_MER_DAWN,
    classical::AUGURS_OF_SPRING,
    classical::RITUAL_OF_ABDUCTION,
    classical::PETRUSHKA_FAIR,
    classical::PETRUSHKA_RUSSIAN_DANCE,
    classical::PETRUSHKA_ROOM,
    classical::PETRUSHKA_MOOR,
    classical::PETRUSHKA_FAIR_EVENING,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::RHYTHMS;
    use crate::pattern::MAX_STEPS;
    use std::collections::HashSet;

    #[test]
    fn written_out_lines_play_on_channels_of_their_own() {
        const PARTS: &[Part] = &[
            Part::new("Verse", "A4:12 | E4:12", "i | V").with_lines(&[
                line("Soloist 2", gm::VOICE_OOHS, "C4:12 | G#3:12"),
                line("Choir", gm::CHOIR_AAHS, "E4:12 | B3:12 & A2:12 | E2:12"),
            ]),
            Part::new("Coda", "A4:12", "i").with_lines(&[line(
                "Choir",
                gm::VOICE_OOHS,
                "A2+E3:12",
            )]),
        ];
        const SONG: Song = Song {
            name: "Lines",
            origin: "",
            feel: "",
            genre: Genre::Folklore,
            tonic: A,
            mode: NaturalMinor,
            time: THREE_FOUR,
            tempo: 72.0,
            comping: Comping::Waltz,
            bass: Bass::None,
            rhythm: "",
            band: Band::new(gm::VOICE_OOHS, gm::HARPSICHORD, gm::ACOUSTIC_BASS),
            style: "",
            parts: PARTS,
            form: &["Verse", "Coda"],
            drums_from: 0,
            about: "",
        };
        let project = super::super::demo::song_demo(&SONG);
        let channel = |name: &str| {
            let index = project.channels().iter().position(|c| c.name == name);
            index.unwrap_or_else(|| panic!("no {name} channel"))
        };
        let (voice, choir) = (channel("Soloist 2"), channel("Choir"));
        // A line's channel plays the program of its first line.
        assert_eq!(project.channels()[voice].program, Some(gm::VOICE_OOHS));
        assert_eq!(project.channels()[choir].program, Some(gm::CHOIR_AAHS));
        let pattern = |name: &str| project.patterns().iter().find(|p| p.name == name).unwrap();
        let verse = pattern("Verse");
        assert_eq!(verse.notes(voice).len(), 2);
        let mut pitches: Vec<_> = verse.notes(choir).iter().map(|n| n.pitch).collect();
        pitches.sort();
        assert_eq!(pitches.len(), 4);
        // Written out, with no left hand: the chart is not accompanied,
        // and there is no Keys channel.
        assert!(project.channels().iter().all(|c| c.name != "Keys"));
        let coda = pattern("Coda");
        assert_eq!(coda.notes(choir).len(), 2);
        assert!(coda.notes(voice).is_empty());
        // A song whose last part is written out ends where its score ends.
        assert_eq!(project.song_length(), 36);
    }

    #[test]
    fn every_bar_of_every_part_adds_up() {
        for song in SONGS {
            for whole in song.parts {
                if !whole.meters.is_empty() {
                    let bars = chart::parse_melody(whole.melody).unwrap().bars.len();
                    assert_eq!(bars, whole.meters.len(), "{}: a meter per bar", song.name);
                    assert_eq!(whole.voices, [(1, 0)], "{}: one voice", song.name);
                    assert!(whole.drums.is_empty(), "{}: drums written out", song.name);
                    assert!(whole.lines.is_empty(), "{}: lines written out", song.name);
                }
            }
            for part in song.parts.iter().flat_map(|part| song.part_runs(part)) {
                let part = &part;
                let bar = song.part_time(part).steps_per_bar();
                let what = format!("{}, {}", song.name, part.name);
                for text in [part.melody, part.left] {
                    if text.is_empty() {
                        continue;
                    }
                    let melody = chart::parse_melody(text)
                        .unwrap_or_else(|| panic!("{what}: cannot read {text}"));
                    for (index, length) in melody.bars.iter().enumerate() {
                        assert_eq!(*length, bar, "{what}: bar {} has {length} steps", index + 1);
                    }
                }
                let tonic = song.part_scale(part).root;
                assert!(
                    chart::parse_chart_in(part.chords, bar, tonic).is_some(),
                    "{what}: cannot read the chords"
                );
                let length = part.length(bar);
                assert!(length > 0 && length <= MAX_STEPS, "{what}: {length} steps");
                assert_eq!(part.melody().length, length, "{what}: melody");
                if !part.chords.trim().is_empty() {
                    assert_eq!(
                        part.chords.split('|').count() * bar,
                        length,
                        "{what}: chords"
                    );
                }
                if !part.left.is_empty() {
                    assert_eq!(part.left().length, length, "{what}: left hand");
                }
                for line in part.lines {
                    let notes = chart::parse_melody(line.notes)
                        .unwrap_or_else(|| panic!("{what}: cannot read the {} line", line.name));
                    for (index, length) in notes.bars.iter().enumerate() {
                        assert_eq!(*length, bar, "{what}: {} bar {}", line.name, index + 1);
                    }
                    assert_eq!(notes.length, length, "{what}: {}", line.name);
                }
                for (drum, line) in part.drums {
                    let drums = drum_line(line)
                        .unwrap_or_else(|| panic!("{what}: cannot read the {drum:?} line"));
                    for (index, length) in drums.bars.iter().enumerate() {
                        assert_eq!(*length, bar, "{what}: {drum:?} bar {}", index + 1);
                    }
                    assert_eq!(drums.length, length, "{what}: {drum:?}");
                }
            }
        }
    }

    #[test]
    fn a_run_of_bars_keeps_every_voice_of_the_accompaniment() {
        let left = "D2:3 | D2:5 | D2:3 & A3:3 | B3:5 | C4:3";
        assert_eq!(bars_of(left, 1, 1), "D2:5 & B3:5");
        assert_eq!(bars_of(left, 1, 2), "D2:5 | D2:3 & B3:5 | C4:3");
        assert_eq!(bars_of(left, 0, 1), "D2:3 & A3:3");
    }

    #[test]
    fn dynamics_are_read_as_scores_write_them() {
        assert_eq!(Dynamic::named("mp"), Some(Dynamic::MezzoPiano));
        assert_eq!(Dynamic::named("fff"), Some(Dynamic::Fortissimo));
        assert_eq!(Dynamic::named("sfz"), None);
        assert_eq!(Dynamic::MezzoPiano.louder(2), Dynamic::Forte);
        assert_eq!(Dynamic::Piano.louder(-2), Dynamic::Pianissimo);
        assert_eq!(Dynamic::Forte.louder(2), Dynamic::Fortissimo);
    }

    #[test]
    fn forms_name_real_parts_and_rhythms_exist() {
        let mut names = HashSet::new();
        for song in SONGS {
            assert!(names.insert(song.name), "{} twice", song.name);
            for name in song.form {
                assert!(song.part(name).is_some(), "{}: no part {name}", song.name);
            }
            assert!(song.drums_from < song.form.len().max(1), "{}", song.name);
            if !song.rhythm.is_empty() {
                let rhythm = RHYTHMS.iter().find(|r| r.name == song.rhythm);
                let rhythm =
                    rhythm.unwrap_or_else(|| panic!("{}: no rhythm {}", song.name, song.rhythm));
                assert_eq!(
                    rhythm.time, song.time,
                    "{}: the drums are in {}",
                    song.name, rhythm.name
                );
            }
        }
        assert!(SONGS.len() >= 60);
    }

    /// A comping written for four beats lands across the beats of a bar
    /// of three, so a song in 3/4 or 6/8 plays one written for three, as
    /// the styles do.
    /// The style a song names is in the library and in the song's time
    /// signature, or follows the song's meter, so selecting it never
    /// changes the bars.
    #[test]
    fn every_song_names_a_real_style() {
        for song in SONGS {
            if song.style.is_empty() {
                continue;
            }
            let style = crate::library::Style::named(song.style)
                .unwrap_or_else(|| panic!("{}: no style called {}", song.name, song.style));
            assert!(
                style.time == song.time || style.follows_the_song(),
                "{}: {} is in another time",
                song.name,
                song.style
            );
        }
    }

    /// A song whose meter changes from bar to bar has a style to play
    /// it with: the library's matching style follows the meter.
    #[test]
    fn songs_that_change_meter_have_a_style_that_follows_it() {
        for song in SONGS {
            let meters: std::collections::HashSet<_> = song
                .parts
                .iter()
                .flat_map(|part| part.meters.iter().copied().chain([song.part_time(part)]))
                .collect();
            if meters.len() < 3 || song.genre != Genre::Classical {
                continue;
            }
            let style = crate::library::Style::named(song.style);
            assert!(
                style.is_some_and(|style| style.follows_the_song()),
                "{} changes meter but has no style that follows it",
                song.name
            );
        }
    }

    #[test]
    fn songs_in_three_play_in_three() {
        for song in SONGS {
            for part in song.parts {
                let (comping, _, rhythm) = song.part_playing(part);
                let time = song.part_time(part);
                if time.steps_per_bar() == 12 {
                    assert!(
                        matches!(
                            comping,
                            Comping::Waltz | Comping::Sway | Comping::Arp3 | Comping::Held
                        ),
                        "{}, {}: in three but plays {comping:?}",
                        song.name,
                        part.name,
                    );
                }
                if part.played.is_some() && !rhythm.is_empty() {
                    let rhythm = RHYTHMS.iter().find(|r| r.name == rhythm);
                    assert_eq!(
                        rhythm.map(|r| r.time),
                        Some(time),
                        "{}, {}: its rhythm is in another time",
                        song.name,
                        part.name
                    );
                }
            }
        }
    }

    #[test]
    fn every_genre_has_songs_and_they_are_in_order() {
        for genre in Genre::ALL {
            let count = SONGS.iter().filter(|song| song.genre == genre).count();
            assert!(count >= 2, "{}: only {count} songs", genre.name());
        }
        assert!(
            SONGS.windows(2).all(|pair| pair[0].genre <= pair[1].genre),
            "SONGS is sorted by genre"
        );
    }

    #[test]
    fn keys_are_written_as_their_tunes_are() {
        let key = |song: &str, part: Option<&str>| {
            let song = Song::named(song).unwrap();
            match part {
                Some(part) => song.part_scale(song.part(part).unwrap()),
                None => song.scale(),
            }
            .name()
        };
        // Tunes written in flats, in keys that are often named with sharps.
        assert_eq!(key("Take Five", None), "Eb minor");
        assert_eq!(key("Golliwogg's Cakewalk", Some("G flat")), "Gb major");
        assert_eq!(key("Golliwogg's Cakewalk", Some("Tristan")), "Gb major");
        assert_eq!(key("Desarma y sangra", Some("E flat minor")), "Eb minor");
        // Tunes written as their keys are usually named keep the name.
        assert_eq!(
            key("In the Hall of the Mountain King", Some("Stretto")),
            "F# major"
        );
        assert_eq!(
            key("Petrushka, The Moor's Room", Some("The Moor's waltz")),
            "B major"
        );
        assert_eq!(key("Maple Leaf Rag", Some("C")), "Db major");
        assert_eq!(key("Funeral March", None), "Bb minor");

        assert_eq!(
            accidentals("Bb3:2 Eb4 F#4 -:2 !pp >C#5. A4+Cb5 | !sfz B4"),
            (2, 3)
        );
    }

    #[test]
    fn melodies_are_mostly_in_their_key() {
        // A few notes from outside the key are normal (passing notes,
        // borrowed leading notes), but a song with many is more likely
        // a typo, such as F written for F#. The Phrygian of flamenco and
        // of the Turkish makam hicaz raises its third (E F G# A B C D):
        // that G# counts as in the key.
        for song in SONGS {
            let mut total = 0;
            let mut inside = 0;
            for part in song.parts {
                let scale = song.part_scale(part);
                let raised_third =
                    (scale.kind == ScaleKind::Phrygian).then(|| scale.root.transpose(4));
                for note in part.melody().notes {
                    total += 1;
                    let pitch = note.pitch.pitch_class();
                    if scale.contains(pitch) || Some(pitch) == raised_third {
                        inside += 1;
                    }
                }
            }
            assert!(
                inside * 100 >= total * 80,
                "{}: only {inside} of {total} notes in {}",
                song.name,
                song.scale().name()
            );
        }
    }

    #[test]
    fn the_first_notes_are_right() {
        let first = |song: &str| {
            let song = Song::named(song).unwrap();
            let part = song.part(song.form[0]).unwrap();
            let notes: Vec<u8> = part
                .melody()
                .notes
                .iter()
                .take(4)
                .map(|n| n.pitch.midi())
                .collect();
            notes
        };
        assert_eq!(first("Ode to Joy"), [78, 78, 79, 81], "F# F# G A");
        assert_eq!(
            first("Twinkle, Twinkle, Little Star"),
            [72, 72, 79, 79],
            "C C G G"
        );
        assert_eq!(first("Korobeiniki"), [76, 71, 72, 74], "E B C D");
    }
}
