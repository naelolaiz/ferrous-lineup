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
//! Am", slash chords included. In a melody, each note is
//! a name with its octave and, after a colon, its length in steps;
//! notes joined by `+` make a chord, and `&` adds another voice, so a
//! whole piano part can be written out.

use crate::harmony::{Chord, ChordSymbol};
use crate::meter::{Bar, TimeSignature};
use crate::pattern::{DEFAULT_VELOCITY, NoteEvent, TICKS_PER_STEP};
use crate::pitch::{Note, PitchClass};

use super::comping::{Bass, BassPart, Comping, Played};
use super::numeral::{self, RomanChord};
use super::voicing::{bass_line, voice_progression};

/// One chord of a chart, with when it starts and how long it lasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChordSpan {
    pub chord: RomanChord,
    pub start: usize,
    pub length: usize,
}

/// Reads a chord chart: bars separated by `|`, each with one or more
/// chords that share the bar equally. A chord that repeats the one
/// before it is held on, not played again: "I | I | IV | V" is two bars
/// of I. Returns `None` if a chord cannot be read, a bar is empty, or a
/// bar of `bar` steps cannot be shared equally between its chords. An
/// empty chart has no chords.
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
            let symbol = ChordSymbol::parse(token).ok()?;
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
            .map(&read)
            .collect::<Option<Vec<RomanChord>>>()?;
        if chords.is_empty() || !bar.is_multiple_of(chords.len()) {
            return None;
        }
        let each = bar / chords.len();
        for (i, chord) in chords.into_iter().enumerate() {
            let start = index * bar + i * each;
            match spans.last_mut() {
                Some(last) if last.chord == chord => last.length += each,
                _ => spans.push(ChordSpan {
                    chord,
                    start,
                    length: each,
                }),
            }
        }
    }
    Some(spans)
}

/// A melody read from text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Melody {
    pub notes: Vec<NoteEvent>,
    /// How long each bar was, as marked by `|`. The tests use this to
    /// check that every bar adds up.
    pub bars: Vec<usize>,
    /// How many steps the whole melody lasts, rests included.
    pub length: usize,
}

/// Reads a melody: notes separated by spaces, such as `"C4:4 E4:2 G4"`.
///
/// - A note is a letter, an optional `#` (sharp) or `b` (flat), and an
///   octave: `C4` is middle C, `F#5` the F sharp an octave and a half
///   above it.
/// - `:4` gives its length in steps (4 is a quarter note). Without it, a
///   note is as long as the one before.
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

/// Reads one voice of a melody (see [`parse_melody`]).
fn parse_voice(text: &str) -> Option<Melody> {
    const TICKS: usize = TICKS_PER_STEP;
    let mut notes: Vec<NoteEvent> = Vec::new();
    let mut bars = Vec::new();
    // Counted in ticks, for triplets.
    let mut at = 0;
    let mut bar_start = 0;
    let mut length = 4;
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
        let name = match token.split_once(':') {
            Some((name, steps)) => {
                length = steps.parse().ok().filter(|&steps: &usize| steps > 0)?;
                name
            }
            None => token,
        };
        let ticks = if triplet > 0 {
            triplet -= 1;
            length * TICKS * 2 / 3
        } else {
            length * TICKS
        };
        if name == "_" {
            // A tie: the last note or chord goes on, across a bar line.
            let held = notes
                .get_mut(last.clone())
                .filter(|held| !held.is_empty())?;
            for note in held {
                let start = note.start_tick();
                *note = melody_note(note.pitch, start, at + ticks - start);
            }
        } else if name != "-" {
            let first = notes.len();
            for name in name.split('+') {
                notes.push(melody_note(note_named(name)?, at, ticks));
            }
            last = first..notes.len();
        }
        at += ticks;
    }
    if at > bar_start {
        bars.push(steps(at - bar_start)?);
    }
    Some(Melody {
        notes,
        bars,
        length: steps(at)?,
    })
}

/// A note of a melody, from tick `start` for `ticks` ticks.
fn melody_note(pitch: Note, start: usize, ticks: usize) -> NoteEvent {
    NoteEvent {
        velocity: DEFAULT_VELOCITY,
        ..NoteEvent::exact(pitch, start, ticks)
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
pub fn accompany(
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
/// bass plays as in [`accompany`], from the root of every chord.
pub fn accompany_in_bars(
    spans: &[ChordSpan],
    tonic: PitchClass,
    comping: Comping,
    bass: Bass,
    time: TimeSignature,
) -> Backing {
    let mut backing = accompany(spans, tonic, time, comping, bass);
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
    backing
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
}
