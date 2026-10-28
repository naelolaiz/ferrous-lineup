//! Lead sheets as text: chords by the bar, and melodies note by note.
//!
//! A **lead sheet** is how working musicians write a song down: the
//! melody, with chord names above it, bar by bar. The library writes the
//! same two things as short strings, so a whole song fits in a few lines
//! and can be read (and checked) by a person:
//!
//! ```text
//! chords:  "I | V | vi | IV V"
//! melody:  "E4:4 E4 F4 G4 | G4 F4 E4 D4 | ..."
//! ```
//!
//! In a chord chart, `|` separates bars, and the chords inside a bar
//! share it equally: "IV V" is half a bar each. Charts read with
//! [`parse_chart_in`] may also name chords as lead sheets do, "C | G/B |
//! Am", slash chords included, and a chord may carry the marks of the
//! band's breaks, as on the chord sheet: "i." for a shot. In a melody,
//! each note is a name with its octave and, after a colon, its length in
//! steps; notes joined by `+` make a chord, and `&` adds another voice,
//! so a whole piano part can be written out. Marks between the notes say
//! how loud they are played, as a score's do: `!p`, `!f`, `>` for an
//! accent.

use crate::band::bass_struck;
use crate::band::stops::Breaks;
use crate::chord_sheet::{Push, Stop, read_marks};
use crate::harmony::{Chord, ChordSymbol};
use crate::meter::{Bar, TimeSignature};
use crate::pattern::{DEFAULT_VELOCITY, NoteEvent, TICKS_PER_STEP};
use crate::pitch::{Note, PitchClass};

use super::comping::{Bass, BassPart, Comping, Played};
use super::numeral::{self, RomanChord};
use super::song::Dynamic;
use super::voicing::{bass_line, voice_progression};

/// One chord of a chart, with when it starts and how long it lasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChordSpan {
    pub chord: RomanChord,
    pub start: usize,
    pub length: usize,
    /// Whether the band plays it early (see [`Push`]).
    pub push: Option<Push>,
    /// Whether the band stops playing its style here (see [`Stop`]).
    pub stop: Option<Stop>,
}

/// Reads a chord chart: bars separated by `|`, each with one or more
/// chords that share the bar equally. A chord that repeats the one
/// before it is held on, not played again: "I | I | IV | V" is two bars
/// of I. Returns `None` if a chord cannot be read, a bar is empty, or a
/// bar of `bar` steps cannot be shared equally between its chords. An
/// empty chart has no chords.
///
/// A chord may carry the marks of the chord sheet (see
/// [`crate::chord_sheet::text`]), where the band leaves its groove, as
/// at the end of a tango's phrase, struck "chan-chan" and then silent:
///
/// - `i.` is a **shot**: the whole band strikes the chord once, short,
///   with a kick and a crash cymbal, and is silent until the next chord.
/// - `V7..` is a **hold**: the band strikes the chord and holds it out.
/// - `i...` is a **rest**: the band is silent until the next chord.
/// - `^iv` is a **push**: the band plays the chord an eighth note early
///   and holds it over the beat (`^^iv`, a sixteenth early).
///
/// A marked chord is struck again even if it repeats the one before
/// it, and so is the chord after a shot, a hold or a rest: "i. | i"
/// comes back in on the second bar. A push never reaches back before
/// the chart's first step, nor past the chord before it.
pub fn parse_chart(text: &str, bar: usize) -> Option<Vec<ChordSpan>> {
    read_chart(text, bar, numeral::parse)
}

/// Reads a chord chart whose chords may also be written by name, as on
/// a lead sheet: "C | G/B | Am | Am/G". A chord by name is counted from
/// `tonic` (see [`RomanChord::from_symbol`]), so the chart is still
/// played in any key, and numerals can be mixed in: "I | G7/B | vi".
/// A chord by name starts with a capital letter from A to G; a numeral
/// never does.
pub fn parse_chart_in(text: &str, bar: usize, tonic: PitchClass) -> Option<Vec<ChordSpan>> {
    read_chart(text, bar, |token| {
        if token.starts_with(|c: char| ('A'..='G').contains(&c)) {
            // Charts are written in letters, whatever names the app shows.
            let symbol = crate::NoteNaming::Letters
                .with(|| ChordSymbol::parse(token))
                .ok()?;
            Some(RomanChord::from_symbol(symbol, tonic))
        } else {
            numeral::parse(token)
        }
    })
}

/// Reads a chart, each chord with `read`.
fn read_chart(
    text: &str,
    bar: usize,
    read: impl Fn(&str) -> Option<RomanChord>,
) -> Option<Vec<ChordSpan>> {
    let mut spans: Vec<ChordSpan> = Vec::new();
    if text.trim().is_empty() {
        return Some(spans);
    }
    for (index, cell) in text.split('|').enumerate() {
        let chords = cell
            .split_whitespace()
            .map(|token| {
                let (push, name, stop) = read_marks(token).ok()?;
                Some((read(name)?, push, stop))
            })
            .collect::<Option<Vec<_>>>()?;
        if chords.is_empty() || !bar.is_multiple_of(chords.len()) {
            return None;
        }
        let each = bar / chords.len();
        for (i, (chord, push, stop)) in chords.into_iter().enumerate() {
            let start = index * bar + i * each;
            let plain = push.is_none() && stop.is_none();
            match spans.last_mut() {
                Some(last) if last.chord == chord && last.stop.is_none() && plain => {
                    last.length += each;
                }
                _ => spans.push(ChordSpan {
                    chord,
                    start,
                    length: each,
                    push,
                    stop,
                }),
            }
        }
    }
    Some(spans)
}

/// The shots, holds, rests and pushes marked on a chart (see
/// [`parse_chart`]), from its first step.
pub(crate) fn breaks(spans: &[ChordSpan]) -> Breaks {
    let marks: Vec<(usize, Option<Push>, Option<Stop>)> = spans
        .iter()
        .map(|span| (span.start, span.push, span.stop))
        .collect();
    let end = spans.last().map_or(0, |span| span.start + span.length);
    Breaks::marked(&marks, end)
}

/// A melody read from text.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Melody {
    pub notes: Vec<NoteEvent>,
    /// How long each bar was, as marked by `|`. The tests use this to
    /// check that every bar adds up.
    pub bars: Vec<usize>,
    /// How many steps the whole melody lasts, rests included.
    pub length: usize,
    /// How loud the first voice is marked to be on each step, as a share
    /// of mezzo-forte's strength (see [`Dynamic::scale`]), from its
    /// dynamics and hairpins. Empty when it has none: mezzo-forte from
    /// start to end.
    pub loudness: Vec<f32>,
    /// The steps the first voice strikes an accent or a sforzando on,
    /// each with how many times harder than the loudness there it is
    /// struck.
    pub accents: Vec<(usize, f32)>,
}

impl Melody {
    /// Whether the melody's first voice has any marks of loudness: a
    /// dynamic, a hairpin, an accent or a sforzando.
    pub fn is_marked(&self) -> bool {
        !self.loudness.is_empty() || !self.accents.is_empty()
    }
}

/// How much harder than the notes around it an accented note (`>`) is
/// struck: a quarter harder.
const ACCENTED: f32 = 1.25;

/// How hard a sforzando (`!sfz`) is struck, as a share of mezzo-forte's
/// strength: harder than fortissimo, whatever the dynamic around it.
const SFORZANDO: f32 = 1.4;

/// How many dynamics a hairpin with no dynamic at its end grows or
/// fades by: a crescendo from p ends at mf.
const HAIRPIN: isize = 2;

/// Reads a melody: notes separated by spaces, such as `"C4:4 E4:2 G4"`.
///
/// - A note is a letter, an optional `#` (sharp) or `b` (flat), and an
///   octave: `C4` is middle C, `F#5` the F sharp an octave and a half
///   above it.
/// - `:4` gives its length in steps (4 is a quarter note). Without it, a
///   note is as long as the one before. A note shorter than a step, as
///   the quick runs and grace notes of an ornament are, takes a fraction:
///   `:1/2` is a thirty-second note, `:1/4` a sixty-fourth. It must come
///   out a whole number of ticks ([`TICKS_PER_STEP`] to the step).
/// - `-` is a rest: `-:8` is half a bar of silence in 4/4.
/// - `_` is a tie: the note before it goes on for that long, so a note
///   can be held across a bar line: `D5:12 | _:8`.
/// - `|` is a bar line. It changes nothing, but the bars it marks are
///   kept, so a test can check that each one has the right length.
/// - `+` joins notes into a chord, struck together and all as long:
///   `A3+C#4+E4:4`. A tie after a chord holds all its notes.
/// - `&` starts another voice, which plays at the same time as the ones
///   before it, from the start, such as a held bass under a moving
///   tune: `"E5:4 D5 C5 B4 & C3:16"`. Each voice is read on its own, and
///   all must have the same bars.
/// - `(3` makes the next three notes (or rests, or ties) a triplet:
///   three in the time of two, as sheet music writes a 3 over them.
///   `(3 G4:2 G4 G4` is three eighth notes in the time of a quarter.
///   Their starts fall between the steps of the grid, so they are
///   placed to the tick (see [`NoteEvent::exact`]).
///
/// The marks a score puts on its notes say how they are played. They
/// take no time, and each changes only the voice it is written in:
///
/// - `!pp`, `!p`, `!mp`, `!mf`, `!f` and `!ff`, standing alone between
///   the notes, are the **dynamics**, from pianissimo (very soft) to
///   fortissimo (very loud): the notes after one are struck that hard
///   (see [`Dynamic`]), until the next. A voice starts at mf, which is
///   how notes are struck without any marks. `!ppp` and `!fff` are read
///   as pp and ff.
/// - `!sfz`, a **sforzando** ("forced"), strikes the next note or chord
///   harder than ff, whatever the dynamic; the notes after it go back.
/// - `!<` and `!>` are **hairpins**: a crescendo, growing louder bit by
///   bit, and a diminuendo, fading. The change is spread evenly over
///   the steps up to the next dynamic, which it reaches there:
///   `!p !< C5:4 D5 E5 F5 | !f G5:16` grows from p to f over the bar.
///   With no dynamic after it, a hairpin ends at `!=`, or else at the
///   end of the voice, two dynamics louder or softer than it started.
/// - `>` right before a note is an **accent**: it is struck a quarter
///   harder than the notes around it, `>A4:2`. Before a chord, it
///   accents the whole chord.
/// - `.` right after a note, after its length if it has one, is a
///   **staccato** dot: the note keeps its place and its written length
///   in the bar, but sounds only half as long, detached from the next:
///   `D5:2.` or `D5.`. After a chord, all its notes are short; after a
///   tie, the note held stops halfway through the tie.
///
/// Returns `None` if anything cannot be read, if the voices' bars
/// differ, or if a bar line falls in the middle of a step.
pub fn parse_melody(text: &str) -> Option<Melody> {
    let mut voices = text.split('&').map(parse_voice);
    let mut melody = voices.next()??;
    for voice in voices {
        let voice = voice?;
        if voice.bars != melody.bars {
            return None;
        }
        melody.notes.extend(voice.notes);
    }
    melody
        .notes
        .sort_by_key(|note| (note.start_tick(), note.pitch));
    Some(melody)
}

/// How hard a note is struck, beyond the loudness of the voice there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stress {
    Plain,
    /// `>`
    Accent,
    /// `!sfz`
    Sforzando,
}

impl Stress {
    /// How hard a note is struck where the voice is `loudness` loud, as
    /// a share of mezzo-forte's strength.
    fn on(self, loudness: f32) -> f32 {
        match self {
            Stress::Plain => loudness,
            Stress::Accent => loudness * ACCENTED,
            Stress::Sforzando => SFORZANDO.max(loudness * ACCENTED),
        }
    }
}

/// Reads one voice of a melody (see [`parse_melody`]).
fn parse_voice(text: &str) -> Option<Melody> {
    const TICKS: usize = TICKS_PER_STEP;
    let mut notes: Vec<NoteEvent> = Vec::new();
    // How each note is stressed, one for each of `notes`.
    let mut stresses: Vec<Stress> = Vec::new();
    let mut loudness = Loudness::default();
    let mut sforzando = false;
    let mut bars = Vec::new();
    // Counted in ticks, for triplets.
    let mut at = 0;
    let mut bar_start = 0;
    // The length of a note, in ticks.
    let mut length = 4 * TICKS;
    // The notes the last note or chord struck, which a tie holds on.
    let mut last = 0..0;
    let mut triplet = 0;
    let steps = |ticks: usize| ticks.is_multiple_of(TICKS).then_some(ticks / TICKS);
    for token in text.split_whitespace() {
        if token == "|" {
            bars.push(steps(at - bar_start)?);
            bar_start = at;
            continue;
        }
        if token == "(3" {
            triplet = 3;
            continue;
        }
        if let Some(mark) = token.strip_prefix('!') {
            match mark {
                "sfz" => sforzando = true,
                "<" => loudness.hairpin(at, 1),
                ">" => loudness.hairpin(at, -1),
                "=" => loudness.close(at),
                _ => loudness.mark(at, Dynamic::named(mark)?),
            }
            continue;
        }
        let (accent, token) = match token.strip_prefix('>') {
            Some(token) => (true, token),
            None => (false, token),
        };
        let (staccato, token) = match token.strip_suffix('.') {
            Some(token) => (true, token),
            None => (false, token),
        };
        let name = match token.split_once(':') {
            Some((name, steps)) => {
                length = length_in_ticks(steps)?;
                name
            }
            None => token,
        };
        let ticks = if triplet > 0 {
            triplet -= 1;
            (length * 2).is_multiple_of(3).then_some(length * 2 / 3)?
        } else {
            length
        };
        // A staccato note takes its whole length in the bar, but sounds
        // for half of it.
        let sounding = if staccato { (ticks / 2).max(1) } else { ticks };
        if name == "_" {
            // A tie: the last note or chord goes on, across a bar line.
            // It is not struck again, so it takes no accent.
            if accent {
                return None;
            }
            let held = notes
                .get_mut(last.clone())
                .filter(|held| !held.is_empty())?;
            for note in held {
                let start = note.start_tick();
                *note = melody_note(note.pitch, start, at + sounding - start);
            }
        } else if name == "-" {
            // A rest is not struck, short or accented.
            if accent || staccato {
                return None;
            }
        } else {
            let stress = match (std::mem::take(&mut sforzando), accent) {
                (true, _) => Stress::Sforzando,
                (false, true) => Stress::Accent,
                (false, false) => Stress::Plain,
            };
            let first = notes.len();
            for name in name.split('+') {
                notes.push(melody_note(note_named(name)?, at, sounding));
                stresses.push(stress);
            }
            last = first..notes.len();
        }
        at += ticks;
    }
    if at > bar_start {
        bars.push(steps(at - bar_start)?);
    }
    // A hairpin still open at the end of the voice ends there.
    loudness.close(at);
    let mut accents = Vec::new();
    for (note, &stress) in notes.iter_mut().zip(&stresses) {
        let here = loudness.at(note.start_tick());
        let struck = stress.on(here);
        note.velocity = velocity(struck);
        if stress != Stress::Plain && accents.last().is_none_or(|&(step, _)| step != note.start) {
            accents.push((note.start, struck / here));
        }
    }
    let length = steps(at)?;
    let loudness = if loudness.is_marked() {
        (0..length).map(|step| loudness.at(step * TICKS)).collect()
    } else {
        Vec::new()
    };
    Some(Melody {
        notes,
        bars,
        length,
        loudness,
        accents,
    })
}

/// A note's length written in steps, `4`, or in a fraction of them,
/// `1/2`, in ticks: `None` if it is not a whole number of ticks, or none.
fn length_in_ticks(steps: &str) -> Option<usize> {
    let (top, bottom) = steps.split_once('/').unwrap_or((steps, "1"));
    let top: usize = top.parse().ok()?;
    let bottom: usize = bottom.parse().ok().filter(|&bottom| bottom > 0)?;
    let ticks = top * TICKS_PER_STEP;
    (ticks > 0 && ticks.is_multiple_of(bottom)).then_some(ticks / bottom)
}

/// A note of a melody, from tick `start` for `ticks` ticks.
fn melody_note(pitch: Note, start: usize, ticks: usize) -> NoteEvent {
    NoteEvent {
        velocity: DEFAULT_VELOCITY,
        ..NoteEvent::exact(pitch, start, ticks)
    }
}

/// The velocity of a note struck `loudness` times as hard as at
/// mezzo-forte: [`DEFAULT_VELOCITY`] at mf, from 1 to 127.
fn velocity(loudness: f32) -> u8 {
    (f32::from(DEFAULT_VELOCITY) * loudness)
        .round()
        .clamp(1.0, 127.0) as u8
}

/// How loud a voice of a melody is marked to be, from tick to tick, as
/// its dynamics and hairpins say (see [`parse_melody`]).
#[derive(Debug, Default)]
struct Loudness {
    /// The dynamic in force: the last one marked, or reached at the end
    /// of a hairpin. `None` before any, which is mezzo-forte.
    level: Option<Dynamic>,
    /// The hairpin under way: the tick it starts on, and which way it
    /// goes, 1 louder and -1 softer.
    hairpin: Option<(usize, isize)>,
    /// The stretches of the voice, in order: from one tick to another,
    /// from one share of mezzo-forte's strength to another. A dynamic is
    /// a stretch that starts and ends on its tick; the notes after it
    /// keep its loudness until the next stretch.
    stretches: Vec<(usize, usize, f32, f32)>,
}

impl Loudness {
    fn level(&self) -> Dynamic {
        self.level.unwrap_or(Dynamic::MezzoForte)
    }

    /// A dynamic from tick `at` on: the end of a hairpin under way.
    fn mark(&mut self, at: usize, dynamic: Dynamic) {
        let from = self.level().scale();
        if let Some((start, _)) = self.hairpin.take() {
            self.stretches.push((start, at, from, dynamic.scale()));
        }
        self.stretches
            .push((at, at, dynamic.scale(), dynamic.scale()));
        self.level = Some(dynamic);
    }

    /// A hairpin from tick `at`, going `way`. One under way ends there.
    fn hairpin(&mut self, at: usize, way: isize) {
        self.close(at);
        self.hairpin = Some((at, way));
    }

    /// Ends the hairpin under way, if any, on tick `at`, [`HAIRPIN`]
    /// dynamics from where it started.
    fn close(&mut self, at: usize) {
        if let Some((_, way)) = self.hairpin {
            self.mark(at, self.level().louder(way * HAIRPIN));
        }
    }

    /// Whether any dynamic or hairpin is marked.
    fn is_marked(&self) -> bool {
        !self.stretches.is_empty()
    }

    /// How loud a note that starts on tick `at` is, as a share of
    /// mezzo-forte's strength: where a hairpin is under way, the share
    /// of the way it has come.
    fn at(&self, at: usize) -> f32 {
        let stretch = self.stretches.iter().rev().find(|stretch| stretch.0 <= at);
        match stretch {
            None => Dynamic::MezzoForte.scale(),
            Some(&(_, end, _, to)) if at >= end => to,
            Some(&(start, end, from, to)) => {
                from + (to - from) * (at - start) as f32 / (end - start) as f32
            }
        }
    }
}

/// The note with a name such as "C4", "F#5" or "Bb3".
pub fn note_named(name: &str) -> Option<Note> {
    let mut chars = name.chars();
    let letter = match chars.next()? {
        'C' => PitchClass::C,
        'D' => PitchClass::D,
        'E' => PitchClass::E,
        'F' => PitchClass::F,
        'G' => PitchClass::G,
        'A' => PitchClass::A,
        'B' => PitchClass::B,
        _ => return None,
    };
    let rest = chars.as_str();
    let (shift, octave) = if let Some(octave) = rest.strip_prefix('#') {
        (1, octave)
    } else if let Some(octave) = rest.strip_prefix('b') {
        (-1, octave)
    } else {
        (0, rest)
    };
    let octave: i32 = octave.parse().ok()?;
    let midi = (octave + 1) * 12 + i32::from(letter.semitone()) + shift;
    u8::try_from(midi)
        .ok()
        .filter(|&midi| midi <= 127)
        .map(Note::new)
}

/// The notes a progression is accompanied with: chords and a bass line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Backing {
    pub chords: Vec<NoteEvent>,
    pub bass: Vec<NoteEvent>,
}

/// Plays a chart in the key whose tonic is `tonic`: voices each chord
/// smoothly from the last, plays the voicings in the rhythm of
/// `comping`, and the roots in the rhythm of `bass`. A slash chord's
/// bass note is played instead of its root, and the line around it
/// moves by step where it can (see [`bass_line`]).
///
/// The bass line may lead into the next chord. After the last chord
/// comes the first one again, because patterns loop. The chart is in
/// bars of `time` from its first step, which the ways of playing that
/// follow the beats of the bar ([`Comping::Aksak`]) need to know.
///
/// Where the chart marks a shot, a hold, a rest or a push (see
/// [`parse_chart`]), the band leaves its rhythm and plays the mark.
pub fn accompany(
    spans: &[ChordSpan],
    tonic: PitchClass,
    time: TimeSignature,
    comping: Comping,
    bass: Bass,
) -> Backing {
    let backing = play_style(spans, tonic, time, comping, bass);
    play_breaks(spans, tonic, time, bass, backing)
}

/// Plays a chart as [`accompany`] does, with no breaks.
fn play_style(
    spans: &[ChordSpan],
    tonic: PitchClass,
    time: TimeSignature,
    comping: Comping,
    bass: Bass,
) -> Backing {
    let chords: Vec<Chord> = spans.iter().map(|span| span.chord.in_key(tonic)).collect();
    let basses: Vec<(PitchClass, bool)> = spans
        .iter()
        .map(|span| (span.chord.bass_in_key(tonic), span.chord.bass.is_some()))
        .collect();
    let line = bass_line(&basses);
    let voicings = voice_progression(&chords);
    let mut backing = Backing::default();
    for (index, span) in spans.iter().enumerate() {
        let (next, _) = basses[(index + 1) % basses.len()];
        backing
            .chords
            .extend(comping.play(time, &voicings[index], span.start, span.length));
        backing.bass.extend(bass.play_over(
            time,
            chords[index],
            line[index],
            next,
            span.start,
            span.length,
        ));
    }
    // Nothing is held over into the first chord: it starts on its root
    // (see `Bass::held_over`).
    let held = bass.held_over();
    if let Some(first) = spans.first()
        && held > 0
    {
        let length = held.min(first.length);
        backing
            .bass
            .push(NoteEvent::new(line[0], first.start, length));
    }
    let end = spans.last().map_or(0, |span| span.start + span.length);
    super::comping::one_at_a_time(&mut backing.bass, end);
    backing
}

/// Plays a chart as [`accompany`] does, but with the chords in step with
/// bars of `time`, the way a band keeps its groove whatever the chords
/// do (see [`Comping::play_in_bars`]): a chord that comes on beat 2 is
/// struck on beat 2, and a chord too short for any hit of the pattern is
/// still struck where it starts, so every chord of a song is heard. The
/// bass plays as in [`accompany`], from the root of every chord, and the
/// band plays the chart's breaks as there.
pub fn accompany_in_bars(
    spans: &[ChordSpan],
    tonic: PitchClass,
    comping: Comping,
    bass: Bass,
    time: TimeSignature,
) -> Backing {
    let mut backing = play_style(spans, tonic, time, comping, bass);
    let chords: Vec<Chord> = spans.iter().map(|span| span.chord.in_key(tonic)).collect();
    let voicings = voice_progression(&chords);
    let harmonies: Vec<Played> = spans
        .iter()
        .zip(voicings)
        .map(|(span, voicing)| Played {
            start: span.start,
            end: span.start + span.length,
            voicing,
            bass: BassPart::Rest,
        })
        .collect();
    let end = spans.last().map_or(0, |span| span.start + span.length);
    let length = time.steps_per_bar().max(1);
    let bars: Vec<Bar> = (0..end.div_ceil(length))
        .map(|number| Bar {
            number,
            start: number * length,
            length: length.min(end - number * length),
            time,
        })
        .collect();
    backing.chords = comping.play_in_bars(&bars, &harmonies);
    play_breaks(spans, tonic, time, bass, backing)
}

/// Plays the shots, holds, rests and pushes marked on a chart over the
/// `backing` its style plays, as the chord sheet's band does (see
/// [`Breaks::apply`]): where the band breaks off its rhythm, the chords
/// are struck and held in the voicings of [`accompany`], and the bass on
/// its note of the line, unless the style has no bass.
fn play_breaks(
    spans: &[ChordSpan],
    tonic: PitchClass,
    time: TimeSignature,
    bass: Bass,
    backing: Backing,
) -> Backing {
    let breaks = breaks(spans);
    if breaks.is_empty() {
        return backing;
    }
    let chords: Vec<Chord> = spans.iter().map(|span| span.chord.in_key(tonic)).collect();
    let basses: Vec<(PitchClass, bool)> = spans
        .iter()
        .map(|span| (span.chord.bass_in_key(tonic), span.chord.bass.is_some()))
        .collect();
    let line = bass_line(&basses);
    let harmonies: Vec<Played> = spans
        .iter()
        .zip(voice_progression(&chords))
        .enumerate()
        .map(|(index, (span, voicing))| Played {
            start: span.start,
            end: span.start + span.length,
            voicing,
            bass: BassPart::Chord {
                chord: chords[index],
                note: line[index],
                next: basses[(index + 1) % basses.len()].0,
            },
        })
        .collect();
    let struck = |played: &Played| {
        let length = played.end - played.start;
        Comping::Held.play(time, &played.voicing, played.start, length)
    };
    Backing {
        chords: breaks.apply(backing.chords, &harmonies, struck),
        bass: match bass {
            Bass::None => backing.bass,
            _ => breaks.apply(backing.bass, &harmonies, bass_struck),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ChordQuality;

    fn spans(text: &str, bar: usize) -> Vec<(u8, usize, usize)> {
        parse_chart(text, bar)
            .unwrap_or_else(|| panic!("{text} did not parse"))
            .iter()
            .map(|span| (span.chord.semitones, span.start, span.length))
            .collect()
    }

    #[test]
    fn chords_share_their_bar() {
        assert_eq!(
            spans("I | IV V | vi", 16),
            [(0, 0, 16), (5, 16, 8), (7, 24, 8), (9, 32, 16)]
        );
        // Three chords in a bar of 3/4: one per beat.
        assert_eq!(spans("I IV V", 12), [(0, 0, 4), (5, 4, 4), (7, 8, 4)]);
    }

    #[test]
    fn repeated_chords_are_held() {
        assert_eq!(
            spans("I | I | IV | I I", 16),
            [(0, 0, 32), (5, 32, 16), (0, 48, 16)]
        );
        // The same root with another quality is a new chord.
        assert_eq!(spans("I | I7", 16), [(0, 0, 16), (0, 16, 16)]);
    }

    #[test]
    fn bad_charts_are_refused() {
        assert_eq!(parse_chart("I | | V", 16), None, "an empty bar");
        assert_eq!(parse_chart("I IV V", 16), None, "16 steps in three");
        assert_eq!(parse_chart("I | X", 16), None);
        assert_eq!(parse_chart("  ", 16), Some(Vec::new()));
    }

    #[test]
    fn melodies_are_notes_and_lengths() {
        let melody = parse_melody("C4:4 E4:2 G4 | -:8 C5:8").unwrap();
        let notes: Vec<(u8, usize, usize)> = melody
            .notes
            .iter()
            .map(|note| (note.pitch.midi(), note.start, note.length))
            .collect();
        assert_eq!(notes, [(60, 0, 4), (64, 4, 2), (67, 6, 2), (72, 16, 8)]);
        assert_eq!(melody.bars, [8, 16]);
        assert_eq!(melody.length, 24);
    }

    #[test]
    fn ties_hold_a_note_across_the_bar() {
        let melody = parse_melody("D5:12 | _:8 B4:4").unwrap();
        let notes: Vec<(usize, usize)> = melody.notes.iter().map(|n| (n.start, n.length)).collect();
        assert_eq!(notes, [(0, 20), (20, 4)]);
        assert_eq!(melody.bars, [12, 12]);
        assert_eq!(parse_melody("_:4"), None, "nothing to tie");
    }

    #[test]
    fn notes_shorter_than_a_step_are_fractions_of_one() {
        // A turn: a thirty-second note's run of two, then the note it
        // ends on, and a triplet of thirty-seconds in the time of a step.
        let melody = parse_melody("F5:1/2 G5 F5:1 -:2 (3 A4:1/2 B4 C#5 D5:3 | -:8").unwrap();
        let ticks: Vec<(u8, usize, usize)> = melody
            .notes
            .iter()
            .map(|note| {
                (
                    note.pitch.midi(),
                    note.start_tick(),
                    note.end_tick() - note.start_tick(),
                )
            })
            .collect();
        assert_eq!(
            ticks,
            [
                (77, 0, 12),
                (79, 12, 12),
                (77, 24, 24),
                (69, 96, 8),
                (71, 104, 8),
                (73, 112, 8),
                (74, 120, 72)
            ]
        );
        assert_eq!(melody.bars, [8, 8]);
        assert_eq!(parse_melody("C4:1/5"), None, "not a whole number of ticks");
        assert_eq!(parse_melody("C4:1/0"), None);
        assert_eq!(parse_melody("C4:0/2"), None);
    }

    #[test]
    fn chords_are_struck_together_and_tied_together() {
        let melody = parse_melody("A3+C#4+E4:4 _:4 B3:8").unwrap();
        let notes: Vec<(u8, usize, usize)> = melody
            .notes
            .iter()
            .map(|note| (note.pitch.midi(), note.start, note.length))
            .collect();
        assert_eq!(notes, [(57, 0, 8), (61, 0, 8), (64, 0, 8), (59, 8, 8)]);
        assert_eq!(parse_melody("A3+:4"), None);
        assert_eq!(parse_melody("A3+H4:4"), None);
    }

    #[test]
    fn voices_play_at_the_same_time() {
        // A tune over a held bass, and an inner voice with rests.
        let melody =
            parse_melody("E5:4 D5 C5 B4 | C5:16 & C3:16 | _:16 & -:8 G4:8 | E4:16").unwrap();
        let notes: Vec<(u8, usize, usize)> = melody
            .notes
            .iter()
            .map(|note| (note.pitch.midi(), note.start, note.length))
            .collect();
        assert_eq!(
            notes,
            [
                (48, 0, 32),
                (76, 0, 4),
                (74, 4, 4),
                (67, 8, 8),
                (72, 8, 4),
                (71, 12, 4),
                (64, 16, 16),
                (72, 16, 16)
            ]
        );
        assert_eq!(melody.bars, [16, 16]);
        assert_eq!(melody.length, 32);
        // The voices must have the same bars.
        assert_eq!(parse_melody("C5:16 & C3:8"), None);
        assert_eq!(parse_melody("C5:16 | D5:16 & C3:16"), None);
    }

    #[test]
    fn note_names() {
        let midi = |name| note_named(name).map(Note::midi);
        assert_eq!(midi("C4"), Some(60));
        assert_eq!(midi("A4"), Some(69));
        assert_eq!(midi("F#5"), Some(78));
        assert_eq!(midi("Bb3"), Some(58));
        assert_eq!(midi("C-1"), Some(0));
        assert_eq!(midi("H4"), None);
        assert_eq!(midi("C"), None);
        assert_eq!(midi("G9"), Some(127));
        assert_eq!(midi("G#9"), None);
        assert_eq!(parse_melody("C4:0"), None);
        assert_eq!(parse_melody("C4:x"), None);
    }

    #[test]
    fn charts_can_name_their_chords() {
        let named = parse_chart_in("Dm7 | G7 | Cmaj7 Am7", 16, PitchClass::C).unwrap();
        let numerals = parse_chart("ii7 | V7 | Imaj7 vi7", 16).unwrap();
        assert_eq!(named, numerals, "the same chords, counted from C");
        let mixed = parse_chart_in("I | G7/B | vi", 16, PitchClass::C).unwrap();
        let chords: Vec<(u8, Option<u8>)> = mixed
            .iter()
            .map(|span| (span.chord.semitones, span.chord.bass))
            .collect();
        assert_eq!(chords, [(0, None), (7, Some(11)), (9, None)]);
        assert_eq!(parse_chart_in("C | H7", 16, PitchClass::C), None);
        assert_eq!(parse_chart_in("C | Gxyz", 16, PitchClass::C), None);
        // C and C/E are different chords, so both are played.
        assert_eq!(
            parse_chart_in("C | C/E", 16, PitchClass::C).unwrap().len(),
            2
        );
    }

    #[test]
    fn slash_chords_walk_the_bass_down() {
        let chart = "C | G/B | Am | Am/G | F | C/E | Dm | G";
        let bass = |tonic: PitchClass| -> Vec<u8> {
            let spans = parse_chart_in(chart, 16, PitchClass::C).unwrap();
            let backing = accompany(
                &spans,
                tonic,
                TimeSignature::COMMON,
                Comping::Held,
                Bass::Held,
            );
            backing.bass.iter().map(|note| note.pitch.midi()).collect()
        };
        // C2 B1 A1 G1 F1 E1, then D2: D1 is below a bass guitar.
        assert_eq!(bass(PitchClass::C), [36, 35, 33, 31, 29, 28, 38, 43]);
        // In D the same line: D2 C#2 B1 A1 G1 F#1 E1, then A2.
        assert_eq!(bass(PitchClass::D), [38, 37, 35, 33, 31, 30, 28, 45]);
        // The chords above are G major, not B something: G/B is G with
        // B in the bass.
        let spans = parse_chart_in("G/B", 16, PitchClass::C).unwrap();
        let backing = accompany(
            &spans,
            PitchClass::C,
            TimeSignature::COMMON,
            Comping::Held,
            Bass::Held,
        );
        let mut chord: Vec<u8> = backing
            .chords
            .iter()
            .map(|note| note.pitch.pitch_class().semitone())
            .collect();
        chord.sort_unstable();
        assert_eq!(chord, [2, 7, 11], "D, G and B");
    }

    #[test]
    fn a_chart_is_accompanied_in_any_key() {
        let chart = parse_chart("I | V", 16).unwrap();
        let backing = accompany(
            &chart,
            PitchClass::D,
            TimeSignature::COMMON,
            Comping::Held,
            Bass::Held,
        );
        let bass: Vec<u8> = backing.bass.iter().map(|note| note.pitch.midi()).collect();
        assert_eq!(bass, [38, 45], "D2 and A2");
        assert_eq!(backing.chords.len(), 6, "two triads");
        assert!(backing.chords.iter().all(|note| note.length == 16));
        assert_eq!(chart[1].chord.quality, ChordQuality::Major);
    }

    #[test]
    fn chords_on_the_beat_are_struck_on_the_beat() {
        // A waltz bar of three chords, a beat each: the waltz's keys play
        // on beats 2 and 3, so the second and third chords are struck
        // there, and the first, where no hit falls, on beat 1.
        let three = TimeSignature::new(3, 4);
        let chart = parse_chart("I V7 IV | I", 12).unwrap();
        let backing = accompany_in_bars(&chart, PitchClass::C, Comping::Waltz, Bass::Waltz, three);
        let struck: Vec<(usize, Vec<u8>)> = [0, 4, 8, 16, 20]
            .iter()
            .map(|&at| {
                let mut notes: Vec<u8> = backing
                    .chords
                    .iter()
                    .filter(|note| note.start == at)
                    .map(|note| note.pitch.pitch_class().semitone())
                    .collect();
                notes.sort_unstable();
                notes.dedup();
                (at, notes)
            })
            .collect();
        assert_eq!(
            struck,
            [
                (0, vec![0, 4, 7]),
                (4, vec![2, 5, 7, 11]),
                (8, vec![0, 5, 9]),
                (16, vec![0, 4, 7]),
                (20, vec![0, 4, 7]),
            ]
        );
        // The per-chord rhythm of `accompany` never reaches beat 2 of a
        // chord a beat long, so it leaves V7 and IV out.
        let restarted = accompany(&chart, PitchClass::C, three, Comping::Waltz, Bass::Waltz);
        assert!(restarted.chords.iter().all(|note| note.start >= 12));
        // The bass is the same either way: every chord's root.
        assert_eq!(backing.bass, restarted.bass);
    }

    fn velocities(text: &str) -> Vec<u8> {
        let melody = parse_melody(text).unwrap_or_else(|| panic!("{text} did not parse"));
        melody.notes.iter().map(|note| note.velocity).collect()
    }

    #[test]
    fn dynamics_set_how_hard_the_notes_are_struck() {
        // Mezzo-forte until the first mark, as without any.
        assert_eq!(velocities("C4:4 !p D4 E4 !f F4"), [100, 55, 55, 115]);
        assert_eq!(
            velocities("!pp C4:4 !mp D4 !mf E4 !ff F4"),
            [40, 75, 100, 127]
        );
        assert_eq!(velocities("!ppp C4:4 !fff D4"), [40, 127], "as pp and ff");
        let melody = parse_melody("C4:4 !p D4:12").unwrap();
        assert_eq!(melody.loudness.len(), 16);
        assert_eq!((melody.loudness[3], melody.loudness[4]), (1.0, 0.55));
        // Without marks, every note as it always was, and no loudness.
        let plain = parse_melody("C4:4 D4 E4 F4").unwrap();
        assert!(plain.notes.iter().all(|n| n.velocity == DEFAULT_VELOCITY));
        assert!(!plain.is_marked());
    }

    #[test]
    fn accents_and_sforzandos_stand_out_once() {
        // In piano: an accent a quarter harder, a sforzando harder than
        // fortissimo, and back to piano after each.
        assert_eq!(velocities("!p C4:4 >D4 !sfz E4 F4"), [55, 69, 127, 55]);
        let melody = parse_melody("!p C4:4 >D4 !sfz E4 F4").unwrap();
        let steps: Vec<usize> = melody.accents.iter().map(|&(step, _)| step).collect();
        assert_eq!(steps, [4, 8]);
        assert!((melody.accents[0].1 - 1.25).abs() < 1e-6);
        // An accented chord: every note of it, and the step once.
        let chord = parse_melody(">A3+C#4+E4:4 B3:4").unwrap();
        let chord_velocities: Vec<u8> = chord.notes.iter().map(|n| n.velocity).collect();
        assert_eq!(chord_velocities, [125, 125, 125, 100]);
        assert_eq!(chord.accents, [(0, 1.25)]);
        // The sforzando waits for a note to be struck.
        assert_eq!(velocities("C4:4 !sfz -:4 D4"), [100, 127]);
    }

    #[test]
    fn hairpins_grow_and_fade_step_by_step() {
        // From p to f over a bar, reached on the f.
        assert_eq!(
            velocities("!p !< C4:4 D4 E4 F4 | !f G4:16"),
            [55, 70, 85, 100, 115]
        );
        // With no dynamic after it, two dynamics further: mp to f at the
        // closing mark...
        assert_eq!(velocities("!mp !< C4:4 D4 != E4:8"), [75, 95, 115]);
        // ...or at the end of the voice: f to mp.
        assert_eq!(velocities("!f !> C4:4 D4 E4 F4"), [115, 105, 95, 85]);
        // The loudness is kept step by step, for the band to follow.
        let melody = parse_melody("!p !< C4:4 D4 E4 F4 | !f G4:16").unwrap();
        assert!((melody.loudness[2] - (0.55 + 0.6 * 2.0 / 16.0)).abs() < 1e-6);
        assert_eq!(melody.loudness[20], 1.15);
    }

    #[test]
    fn staccato_notes_keep_their_place_but_sound_short() {
        let melody = parse_melody("C4:4. D4. E4:2. -:6 | A3+C4:8. D5:4 _:4.").unwrap();
        let notes: Vec<(u8, usize, usize)> = melody
            .notes
            .iter()
            .map(|note| (note.pitch.midi(), note.start, note.length))
            .collect();
        assert_eq!(
            notes,
            [
                (60, 0, 2),
                (62, 4, 2),
                (64, 8, 1),
                (57, 16, 4),
                (60, 16, 4),
                (74, 24, 6)
            ]
        );
        assert_eq!(melody.bars, [16, 16], "the bars still add up");
        // A triplet eighth note, short: half of its 32 ticks.
        let triplet = parse_melody("(3 G4:2. G4 G4 -:4").unwrap();
        let first = triplet.notes[0];
        assert_eq!(first.end_tick() - first.start_tick(), 16);
        assert_eq!(triplet.notes[1].start_tick(), 32);
    }

    #[test]
    fn each_voice_has_its_own_marks() {
        let melody = parse_melody("!p C5:8 >D5:8 & C3:16").unwrap();
        let notes: Vec<(u8, u8)> = melody
            .notes
            .iter()
            .map(|note| (note.pitch.midi(), note.velocity))
            .collect();
        assert_eq!(notes, [(48, 100), (72, 55), (74, 69)]);
        // The loudness and accents are the first voice's.
        assert_eq!(melody.loudness[0], 0.55);
        assert_eq!(melody.accents.len(), 1);
        assert_eq!(melody.accents[0].0, 8);
    }

    #[test]
    fn marks_that_mean_nothing_are_refused() {
        assert_eq!(parse_melody("!x C4:4"), None);
        assert_eq!(parse_melody("C4:4 >-:4"), None, "an accented rest");
        assert_eq!(parse_melody("C4:4 -:4."), None, "a short rest");
        assert_eq!(parse_melody("C4:4 >_:4"), None, "an accented tie");
        assert_eq!(parse_melody("C4:4 > D4"), None);
        assert_eq!(parse_melody("C4:4 !"), None);
    }

    #[test]
    fn charts_carry_the_breaks_of_the_chord_sheet() {
        let spans = parse_chart("i. | i | ^iv V7.. | i... | ^^i", 16).unwrap();
        let marks: Vec<_> = spans
            .iter()
            .map(|s| (s.chord.semitones, s.start, s.length, s.push, s.stop))
            .collect();
        assert_eq!(
            marks,
            [
                // After a shot, the same chord comes in again.
                (0, 0, 16, None, Some(Stop::Shot)),
                (0, 16, 16, None, None),
                (5, 32, 8, Some(Push::Eighth), None),
                (7, 40, 8, None, Some(Stop::Hold)),
                (0, 48, 16, None, Some(Stop::Rest)),
                (0, 64, 16, Some(Push::Sixteenth), None),
            ]
        );
        // A marked chord is struck again, even on the same chord.
        assert_eq!(parse_chart("i | i.", 16).unwrap().len(), 2);
        // By name too, and no more dots than a rest has.
        let named = parse_chart_in("Am. | ^E7", 16, PitchClass::A).unwrap();
        assert_eq!(
            (named[0].stop, named[1].push),
            (Some(Stop::Shot), Some(Push::Eighth))
        );
        assert_eq!(parse_chart("i.... | V", 16), None);
        assert_eq!(parse_chart("^^^i", 16), None);
    }

    #[test]
    fn the_band_plays_a_shot_and_stops() {
        // "Chan-chan": i | V7 struck short and silent, then i again.
        let spans = parse_chart("i | V7. | i", 16).unwrap();
        let backing = accompany_in_bars(
            &spans,
            PitchClass::A,
            Comping::Quarters,
            Bass::Quarters,
            TimeSignature::COMMON,
        );
        for notes in [&backing.chords, &backing.bass] {
            let in_shot: Vec<(usize, usize)> = notes
                .iter()
                .filter(|note| (16..32).contains(&note.start))
                .map(|note| (note.start, note.end()))
                .collect();
            assert!(!in_shot.is_empty());
            assert!(
                in_shot.iter().all(|&(start, end)| start == 16 && end == 18),
                "{in_shot:?}"
            );
            assert!(notes.iter().any(|note| note.start == 32), "back in");
            assert!(
                notes
                    .iter()
                    .all(|note| note.start >= 32 || note.end() <= 18)
            );
        }
        // A rest is silent; the horns' chords too.
        let spans = parse_chart("i | V7... | i", 16).unwrap();
        let horns = accompany(
            &spans,
            PitchClass::A,
            TimeSignature::COMMON,
            Comping::Held,
            Bass::None,
        );
        assert!(horns.chords.iter().all(|n| !(16..32).contains(&n.start)));
        assert!(horns.bass.is_empty());
        // Without marks, the band plays its style throughout.
        assert!(breaks(&parse_chart("i | V7 | i", 16).unwrap()).is_empty());
    }
}
