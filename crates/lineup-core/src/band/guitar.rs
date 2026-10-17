//! The band's guitar: chords as a guitarist plays them.
//!
//! A pianist can play any notes of a chord. A guitarist has six strings,
//! tuned E A D G B E, and four fingers, so a guitar chord is a *shape*:
//! one note or none on each string, all within a hand's reach, with the
//! bass note on the lowest string that sounds. Open strings ring without
//! a finger, which is why the keys of E, A, D and G are the guitar's
//! favourites. This module finds such a shape for every chord, as a
//! chord book would list it, and strums it: the strings are struck one
//! after another, low to high on a down-stroke and high to low on an
//! up-stroke.

use std::collections::HashMap;

use crate::harmony::Chord;
use crate::library::comping::{BassPart, Comping, Played};
use crate::meter::Bar;
use crate::pattern::NoteEvent;
use crate::pitch::{Note, PitchClass};

use super::variation::Variation;

/// The open strings, from the lowest: E2, A2, D3, G3, B3 and E4.
const TUNING: [u8; 6] = [40, 45, 50, 55, 59, 64];

/// How many frets a hand covers without moving along the neck: four,
/// one for each finger.
const REACH: u8 = 4;

/// The highest fret a shape may start on.
const HIGHEST_POSITION: u8 = 9;

/// The fewest strings a strummed chord sounds on.
const FEWEST_STRINGS: usize = 4;

/// The lowest note two neighbouring notes of a chord may start from
/// without sounding muddy, by the interval between them in semitones:
/// the "low interval limits" arrangers use. A third is clear from about
/// Bb2 up, a fifth almost anywhere. A shape that goes below them is
/// worse, though some of the best known (the open G) do.
const LOWEST_FOR: [u8; 12] = [0, 52, 51, 48, 46, 45, 46, 34, 43, 41, 41, 41];

/// How many ticks apart a strum strikes its strings: about 5 ms at 120
/// BPM, so a six-string strum takes about 25 ms, as a real one does.
const STRUM_TICKS: usize = 1;

/// How many strings an up-stroke strikes: the highest four. The hand
/// coming back up rarely reaches the low strings.
const UP_STRINGS: usize = 4;

/// How the guitar plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuitarPart {
    /// Full chords strummed in a comping rhythm, or picked one string at
    /// a time where the rhythm is an arpeggio.
    Strum(Comping),
    /// Full chords, in the rhythm the band style's piano plays.
    StylesRhythm,
    /// Short chords on the highest four strings on beats 2 and 4, with
    /// the snare, leaving the low strings to the bass: the "chop" of
    /// reggae and soul, the "chick" of country's boom-chick.
    Chop,
    /// Power chords, the root, the fifth and the octave, in a comping
    /// rhythm: rock's distorted guitars, where a third would sound muddy.
    Power(Comping),
    /// The root, third and seventh on every beat, the second and fourth
    /// leaned on: the swing band guitar's "four to the bar".
    FourToTheBar,
    /// The root with its fifth, then with its sixth, a beat each, on both
    /// eighths of every beat: the boogie shuffle of blues and rock and
    /// roll.
    Boogie,
}

impl GuitarPart {
    /// How it plays in `variation`: a held chord or a picked one is
    /// strummed in variation b, and held power chords become eighths.
    pub fn in_variation(self, variation: Variation) -> GuitarPart {
        match (variation, self) {
            (Variation::B, GuitarPart::Power(Comping::Held)) => GuitarPart::Power(Comping::Pulse),
            (
                Variation::B,
                GuitarPart::Strum(Comping::Held | Comping::ArpUp | Comping::ArpUpDown),
            ) => GuitarPart::Strum(Comping::Strum),
            (_, part) => part,
        }
    }

    /// A short description, for the chord sheet's style menu.
    pub fn describe(self) -> &'static str {
        match self {
            GuitarPart::Strum(Comping::Held) => "a guitar letting chords ring",
            GuitarPart::Strum(Comping::ArpUp | Comping::ArpUpDown | Comping::Arp3) => {
                "a guitar picking the chords"
            }
            GuitarPart::Chop => "a guitar on beats 2 and 4",
            GuitarPart::Strum(Comping::Offbeat) => "a guitar between the beats",
            GuitarPart::Strum(Comping::Funk) => "a funk guitar scratching sixteenths",
            GuitarPart::Strum(_) | GuitarPart::StylesRhythm => "a strummed guitar",
            GuitarPart::Power(_) => "a guitar playing power chords",
            GuitarPart::FourToTheBar => "a guitar four to the bar",
            GuitarPart::Boogie => "a guitar playing the boogie shuffle",
        }
    }
}

/// The guitar's notes for `harmonies`, in `bars`. `styles` is the
/// rhythm the band style's piano plays.
pub(crate) fn play(
    part: GuitarPart,
    styles: Comping,
    bars: &[Bar],
    harmonies: &[Played],
) -> Vec<NoteEvent> {
    match part {
        GuitarPart::Strum(rhythm) => strummed(rhythm, bars, harmonies, shape),
        GuitarPart::StylesRhythm => strummed(styles, bars, harmonies, shape),
        GuitarPart::Chop => strummed(Comping::Backbeat, bars, harmonies, top_strings),
        GuitarPart::Power(rhythm) => strummed(rhythm, bars, harmonies, power_chord),
        GuitarPart::FourToTheBar => {
            let mut notes = strummed(Comping::Quarters, bars, harmonies, shell);
            lean_on_two_and_four(&mut notes, bars);
            notes
        }
        GuitarPart::Boogie => boogie(bars, harmonies),
    }
}

/// `rhythm` played with each harmony voiced by `voice`, and strummed.
fn strummed(
    rhythm: Comping,
    bars: &[Bar],
    harmonies: &[Played],
    voice: fn(&Played) -> Vec<Note>,
) -> Vec<NoteEvent> {
    let mut shapes: HashMap<(Vec<Note>, BassKey), Vec<Note>> = HashMap::new();
    let voiced: Vec<Played> = harmonies
        .iter()
        .map(|harmony| {
            let key = (harmony.voicing.clone(), BassKey::of(&harmony.bass));
            let voicing = shapes.entry(key).or_insert_with(|| voice(harmony)).clone();
            Played {
                voicing,
                ..harmony.clone()
            }
        })
        .collect();
    let notes = rhythm.play_in_bars(bars, &voiced);
    strum(notes, &voiced)
}

/// What a harmony's bass is, as a key for remembering its shape: two
/// harmonies with the same notes and bass get the same shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BassKey {
    Chord(Chord, PitchClass),
    Pedal(PitchClass),
    Rest,
}

impl BassKey {
    fn of(bass: &BassPart) -> BassKey {
        match *bass {
            BassPart::Chord { chord, note, .. } => BassKey::Chord(chord, note.pitch_class()),
            BassPart::Pedal(note) => BassKey::Pedal(note.pitch_class()),
            BassPart::Rest => BassKey::Rest,
        }
    }
}

/// Spreads the chords of `notes` into strums: the notes a hit strikes
/// together are struck one string after another. A hit that starts on
/// the shape's lowest string is a down-stroke, played low to high;
/// another is an up-stroke, played high to low on the highest strings
/// only. Each string still stops where the hit does.
fn strum(notes: Vec<NoteEvent>, harmonies: &[Played]) -> Vec<NoteEvent> {
    let lowest_at = |step: usize| {
        let index = harmonies.partition_point(|harmony| harmony.start <= step);
        index
            .checked_sub(1)
            .and_then(|index| harmonies[index].voicing.first().copied())
    };
    let mut strummed = Vec::with_capacity(notes.len());
    let mut index = 0;
    while index < notes.len() {
        let start = notes[index].start;
        let count = notes[index..]
            .iter()
            .take_while(|note| note.start == start)
            .count();
        let mut chord: Vec<NoteEvent> = notes[index..index + count].to_vec();
        index += count;
        if chord.len() < 2 {
            strummed.extend(chord);
            continue;
        }
        chord.sort_by_key(|note| note.pitch);
        let down = lowest_at(start) == chord.first().map(|note| note.pitch);
        if !down {
            chord.reverse();
            chord.truncate(UP_STRINGS);
        }
        for (string, note) in chord.into_iter().enumerate() {
            let late = string * STRUM_TICKS;
            let ticks = note.ticks().saturating_sub(late).max(1);
            strummed.push(NoteEvent {
                velocity: note.velocity,
                ..NoteEvent::exact(note.pitch, note.start_tick() + late, ticks)
            });
        }
    }
    strummed.sort_by_key(|note| (note.start_tick(), note.pitch));
    strummed
}

/// Makes the notes on beats 2 and 4 of each bar a little louder than the
/// ones on 1 and 3: the swing guitarist's snap of the wrist on the
/// backbeat.
fn lean_on_two_and_four(notes: &mut [NoteEvent], bars: &[Bar]) {
    for note in notes {
        let Some(bar) = bars
            .iter()
            .find(|bar| (bar.start..bar.end()).contains(&note.start))
        else {
            continue;
        };
        note.velocity = match bar.beat_at(note.start).index % 2 {
            1 => note.velocity.saturating_add(6).min(127),
            _ => note.velocity.saturating_sub(10).max(1),
        };
    }
}

/// The note of `pitch_class` on the lowest string's first frets: from
/// E2 to D#3, where a guitarist puts a chord's bass.
fn lowest(pitch_class: PitchClass) -> Note {
    Note::new(TUNING[0] + PitchClass::E.semitones_up_to(pitch_class))
}

/// The notes a guitar chord must have, and the ones it would like to.
///
/// A guitar has room for about four different notes, so like a pianist
/// it leaves some out. It keeps the root, the third (or the second or
/// fourth of a sus chord), the sixth or seventh, a fifth that is not a
/// perfect one, and the highest extension, which names the chord (the 13
/// of a G13). A perfect fifth and lower extensions are welcome when they
/// fit; in a plain triad the fifth is needed too.
fn chord_tones(chord: Chord) -> (Vec<PitchClass>, Vec<PitchClass>) {
    let intervals = chord.quality.intervals();
    let tones = chord.quality.tones();
    let triad = intervals.len() <= 3;
    let highest = tones.iter().copied().filter(|&tone| tone > 7).max();
    let mut needed = Vec::new();
    let mut welcome = Vec::new();
    for (&interval, &tone) in intervals.iter().zip(tones) {
        let pitch_class = chord.root.transpose(i32::from(interval));
        let needs = match tone {
            5 => triad || interval != 7,
            9 | 11 | 13 => Some(tone) == highest,
            _ => true,
        };
        match needs {
            true => needed.push(pitch_class),
            false => welcome.push(pitch_class),
        }
    }
    (needed, welcome)
}

/// The shape a guitarist would strum for `harmony`.
fn shape(harmony: &Played) -> Vec<Note> {
    let (bass, needed, welcome) = match harmony.bass {
        BassPart::Chord { chord, note, .. } => {
            let (needed, welcome) = chord_tones(chord);
            (note.pitch_class(), needed, welcome)
        }
        BassPart::Pedal(note) => {
            let mut needed: Vec<PitchClass> = harmony
                .voicing
                .iter()
                .map(|note| note.pitch_class())
                .collect();
            needed.dedup();
            (note.pitch_class(), needed, Vec::new())
        }
        BassPart::Rest => return Vec::new(),
    };
    find_shape(bass, &needed, &welcome).map_or_else(
        || {
            // No shape in reach (a chord of five or six different notes):
            // the piano's notes over the bass, on the guitar's lowest string.
            let mut notes = vec![lowest(bass)];
            notes.extend(harmony.voicing.iter().copied());
            notes
        },
        |shape| shape.notes(),
    )
}

/// A chord shape: the frets of the strings that sound, from string `low`
/// (0 is the low E) up. The strings below and above it are not played.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Shape {
    low: usize,
    frets: Vec<u8>,
}

impl Shape {
    fn notes(&self) -> Vec<Note> {
        self.frets
            .iter()
            .enumerate()
            .map(|(string, fret)| Note::new(TUNING[self.low + string] + fret))
            .collect()
    }
}

/// The best shape for a chord over `bass` with the notes `needed`, and
/// as many of `welcome` as fit: every string from the one that plays the
/// bass up to the highest that sounds plays a note of the chord. Shapes
/// low on the neck, with open strings, and on more strings are better,
/// as are ones with more welcome notes. `None` if there is none within a
/// hand's reach.
fn find_shape(bass: PitchClass, needed: &[PitchClass], welcome: &[PitchClass]) -> Option<Shape> {
    let in_chord = |pitch_class: PitchClass| {
        pitch_class == bass || needed.contains(&pitch_class) || welcome.contains(&pitch_class)
    };
    let mut best: Option<(i32, Shape)> = None;
    for position in 0..=HIGHEST_POSITION {
        // At the nut, open strings and the first three frets; further up,
        // the four frets from the position.
        let frets = match position {
            0 => 0..REACH,
            _ => position..position + REACH,
        };
        let choices: Vec<Vec<u8>> = TUNING
            .iter()
            .map(|&open| {
                frets
                    .clone()
                    .filter(|fret| in_chord(PitchClass::from_semitone(open + fret)))
                    .collect()
            })
            .collect();
        for low in 0..TUNING.len() - FEWEST_STRINGS + 1 {
            for &bass_fret in &choices[low] {
                if PitchClass::from_semitone(TUNING[low] + bass_fret) != bass {
                    continue;
                }
                for top in low + FEWEST_STRINGS - 1..TUNING.len() {
                    let mut frets = vec![bass_fret];
                    try_frets(&choices, low, top, &mut frets, &mut |frets| {
                        let Some(cost) = cost(frets, low, top, needed, welcome) else {
                            return;
                        };
                        if best.as_ref().is_none_or(|(least, _)| cost < *least) {
                            let frets = frets.to_vec();
                            best = Some((cost, Shape { low, frets }));
                        }
                    });
                }
            }
        }
    }
    best.map(|(_, shape)| shape)
}

/// Calls `found` with every way to fret the strings after `low` up to
/// `top`, one of each string's `choices` each, after the `frets` chosen.
fn try_frets(
    choices: &[Vec<u8>],
    low: usize,
    top: usize,
    frets: &mut Vec<u8>,
    found: &mut impl FnMut(&[u8]),
) {
    let string = low + frets.len();
    if string > top {
        found(frets);
        return;
    }
    for &fret in &choices[string] {
        frets.push(fret);
        try_frets(choices, low, top, frets, found);
        frets.pop();
    }
}

/// How much worse a shape is than the ideal, or `None` if it cannot be
/// played or misses a needed note. `frets` are the strings' frets, from
/// string `low` to string `top`. Shapes high up the neck, with strings
/// left out or with muddy low notes cost more; welcome notes and open
/// strings, which ring, less.
fn cost(
    frets: &[u8],
    low: usize,
    top: usize,
    needed: &[PitchClass],
    welcome: &[PitchClass],
) -> Option<i32> {
    let sounding: Vec<PitchClass> = frets
        .iter()
        .enumerate()
        .map(|(string, fret)| PitchClass::from_semitone(TUNING[low + string] + fret))
        .collect();
    if !needed.iter().all(|tone| sounding.contains(tone)) {
        return None;
    }
    // Fingers: one for each fretted note, or, for more than four, the
    // first finger barred across the lowest fret and one for each note
    // above it. A barre stops every string above the one it starts on,
    // so none of those can ring open.
    let fretted = frets.iter().filter(|&&fret| fret > 0).count();
    let lowest_fret = frets.iter().copied().filter(|&fret| fret > 0).min();
    let highest_fret = frets.iter().copied().max().unwrap_or(0);
    if fretted > 4 {
        let barre = lowest_fret?;
        let from = frets.iter().position(|&fret| fret == barre)?;
        let open_under = frets[from..].contains(&0);
        let fingers = 1 + frets.iter().filter(|&&fret| fret > barre).count();
        if open_under || fingers > 4 {
            return None;
        }
    }
    let muted = TUNING.len() - (top - low + 1);
    let notes: Vec<u8> = frets
        .iter()
        .enumerate()
        .map(|(string, fret)| TUNING[low + string] + fret)
        .collect();
    let muddy = notes
        .windows(2)
        .filter(|pair| pair[0] < LOWEST_FOR[usize::from(pair[1] - pair[0]).min(11)])
        .count();
    let welcome_notes = welcome
        .iter()
        .filter(|tone| sounding.contains(tone))
        .count();
    let open = frets.iter().filter(|&&fret| fret == 0).count();
    Some(
        2 * i32::from(highest_fret) + 3 * muted as i32 + 2 * muddy as i32
            - 3 * welcome_notes as i32
            - open as i32,
    )
}

/// The highest four strings of `harmony`'s shape.
fn top_strings(harmony: &Played) -> Vec<Note> {
    let notes = shape(harmony);
    notes[notes.len().saturating_sub(UP_STRINGS)..].to_vec()
}

/// A power chord for `harmony`: its root, fifth and octave from the low
/// strings, or over another bass note, that note, then the root and the
/// fifth above it. A group of notes with no root gets a full shape.
fn power_chord(harmony: &Played) -> Vec<Note> {
    let BassPart::Chord { chord, note, .. } = harmony.bass else {
        return shape(harmony);
    };
    let fifth = chord
        .quality
        .intervals()
        .iter()
        .copied()
        .find(|interval| (6..=8).contains(interval))
        .unwrap_or(7);
    let low = lowest(note.pitch_class());
    let root = low.midi() + note.pitch_class().semitones_up_to(chord.root);
    let notes = match root == low.midi() {
        true => [root, root + fifth, root + 12],
        false => [low.midi(), root, root + fifth],
    };
    notes.into_iter().map(Note::new).collect()
}

/// The swing guitarist's three-note chord: the bass on a low string, the
/// third and the seventh above it (a sixth chord's sixth, or a triad's
/// fifth, in place of the seventh), and a fifth that is not perfect.
/// Over a root from E to A# the seventh comes first, then the third an
/// octave up; from B to D#, the third and then the seventh.
fn shell(harmony: &Played) -> Vec<Note> {
    let BassPart::Chord { chord, note, .. } = harmony.bass else {
        return shape(harmony);
    };
    let intervals = chord.quality.intervals();
    let tones = chord.quality.tones();
    let interval_of = |wanted: &[u8]| {
        intervals
            .iter()
            .zip(tones)
            .find(|(_, tone)| wanted.contains(tone))
            .map(|(&interval, _)| interval % 12)
    };
    let third = interval_of(&[3, 2, 4]).unwrap_or(4);
    let seventh = interval_of(&[7, 6])
        .or_else(|| interval_of(&[5]))
        .unwrap_or(7);
    let altered_fifth = intervals
        .iter()
        .copied()
        .find(|&interval| interval == 6 || interval == 8);
    let low = lowest(note.pitch_class()).midi();
    let root = low + note.pitch_class().semitones_up_to(chord.root);
    let mut notes = match root <= TUNING[1] + 1 {
        true => vec![root, root + seventh, root + 12 + third],
        false => vec![root, root + third, root + seventh],
    };
    if let Some(fifth) = altered_fifth {
        notes.push(root + 12 + fifth);
    }
    if root != low {
        notes.insert(0, low);
    }
    notes.sort_unstable();
    notes.dedup();
    notes.into_iter().map(Note::new).collect()
}

/// The boogie shuffle: on each beat, the chord's root with its fifth,
/// then on the next with its sixth, both eighths of the beat, the second
/// lighter. A minor or diminished chord keeps its fifth, as its sixth
/// would sound like another chord. A group of notes with no root is
/// strummed in the same rhythm.
fn boogie(bars: &[Bar], harmonies: &[Played]) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    for harmony in harmonies {
        let BassPart::Chord { chord, .. } = harmony.bass else {
            if !harmony.voicing.is_empty() {
                notes.extend(strummed(
                    Comping::Shuffle,
                    bars,
                    std::slice::from_ref(harmony),
                    shape,
                ));
            }
            continue;
        };
        let root = lowest(chord.root).midi();
        let major = chord.quality.intervals().contains(&4);
        for bar in bars {
            for (number, step) in bar.beat_starts().enumerate() {
                if step < harmony.start || step >= harmony.end {
                    continue;
                }
                let upper = match major && number % 2 == 1 {
                    true => 9,
                    false => 7,
                };
                // Two eighths in a beat of two; in a beat of three
                // eighths (12/8), the long-short of the triplet itself.
                let beat = bar.beat_at(step).length;
                let second = match beat % 3 {
                    0 => beat * 2 / 3,
                    _ => beat / 2,
                }
                .max(1);
                for (at, length, velocity) in
                    [(step, second, 96), (step + second, beat - second, 72)]
                {
                    if length == 0 || at >= harmony.end || at >= bar.end() {
                        continue;
                    }
                    let length = length.min(harmony.end - at);
                    for pitch in [root, root + upper] {
                        notes.push(NoteEvent {
                            velocity,
                            ..NoteEvent::new(Note::new(pitch), at, length)
                        });
                    }
                }
            }
        }
    }
    notes.sort_by_key(|note| (note.start, note.pitch));
    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ChordQuality::{self, *};
    use crate::meter::{Meter, TimeSignature};
    use crate::pattern::TICKS_PER_STEP;

    /// A shape as chord books write it: the fret of each string from the
    /// low E, "x" for a string not played.
    fn written(shape: &Shape) -> String {
        (0..TUNING.len())
            .map(|string| match string.checked_sub(shape.low) {
                Some(index) if index < shape.frets.len() => shape.frets[index].to_string(),
                _ => "x".to_string(),
            })
            .collect()
    }

    fn chord_shape(root: PitchClass, quality: ChordQuality) -> String {
        let (needed, welcome) = chord_tones(Chord::new(root, quality));
        written(&find_shape(root, &needed, &welcome).unwrap())
    }

    #[test]
    fn open_chords_are_the_ones_in_chord_books() {
        use PitchClass::*;
        assert_eq!(chord_shape(C, Major), "x32010");
        assert_eq!(chord_shape(G, Major), "320003");
        assert_eq!(chord_shape(D, Major), "xx0232");
        assert_eq!(chord_shape(A, Major), "x02220");
        assert_eq!(chord_shape(E, Major), "022100");
        assert_eq!(chord_shape(A, Minor), "x02210");
        assert_eq!(chord_shape(E, Minor), "022000");
        assert_eq!(chord_shape(D, Minor), "xx0231");
        assert_eq!(chord_shape(G, Dominant7), "320001");
        assert_eq!(chord_shape(C, Dominant7), "x32310");
        assert_eq!(chord_shape(E, Dominant7), "020100");
        assert_eq!(chord_shape(C, Major7), "x32000");
    }

    #[test]
    fn other_keys_take_barre_chords() {
        use PitchClass::*;
        // The E shape barred at the first fret, and the A shape at the
        // second.
        assert_eq!(chord_shape(F, Major), "133211");
        assert_eq!(chord_shape(B, Major), "x24442");
    }

    #[test]
    fn a_slash_chord_has_its_bass_lowest() {
        use PitchClass::*;
        let shape_over = |root, bass| {
            let (needed, welcome) = chord_tones(Chord::new(root, Major));
            written(&find_shape(bass, &needed, &welcome).unwrap())
        };
        assert_eq!(shape_over(C, E), "032010");
        assert_eq!(shape_over(G, B), "x20003");
    }

    #[test]
    fn every_chord_in_every_key_has_a_shape_with_its_needed_notes() {
        for root in PitchClass::ALL {
            for quality in ChordQuality::ALL {
                let chord = Chord::new(root, quality);
                let (needed, _) = chord_tones(chord);
                let harmony = Played {
                    start: 0,
                    end: 16,
                    voicing: vec![Note::new(60)],
                    bass: BassPart::Chord {
                        chord,
                        note: Note::new(36 + root.semitone()),
                        next: root,
                    },
                };
                let notes = shape(&harmony);
                assert_eq!(notes[0].pitch_class(), root, "{root:?} {quality:?}");
                for tone in &needed {
                    assert!(
                        notes.iter().any(|note| note.pitch_class() == *tone),
                        "{root:?} {quality:?} lacks {tone:?}: {notes:?}"
                    );
                }
                assert!(notes.iter().all(|note| (40..=88).contains(&note.midi())));
            }
        }
    }

    #[test]
    fn power_chords_and_shells() {
        let played = |root: PitchClass, quality, bass: PitchClass| Played {
            start: 0,
            end: 16,
            voicing: Vec::new(),
            bass: BassPart::Chord {
                chord: Chord::new(root, quality),
                note: Note::new(36 + bass.semitone()),
                next: root,
            },
        };
        let midi = |notes: Vec<Note>| notes.iter().map(|note| note.midi()).collect::<Vec<_>>();
        use PitchClass::*;
        // A5: A2 E3 A3, even for A minor.
        assert_eq!(midi(power_chord(&played(A, Minor, A))), [45, 52, 57]);
        // C/E: E2, then C3 and G3.
        assert_eq!(midi(power_chord(&played(C, Major, E))), [40, 48, 55]);
        // G7 from the low E string: G2, F3, B3.
        assert_eq!(midi(shell(&played(G, Dominant7, G))), [43, 53, 59]);
        // C7 from the A string: C3, E3, Bb3.
        assert_eq!(midi(shell(&played(C, Dominant7, C))), [48, 52, 58]);
        // Bm7b5 keeps its flat fifth: B2 D3 A3 F4.
        assert_eq!(
            midi(shell(&played(B, HalfDiminished7, B))),
            [47, 50, 57, 65]
        );
    }

    #[test]
    fn a_down_stroke_goes_up_the_strings_and_an_up_stroke_comes_down() {
        let harmony = Played {
            start: 0,
            end: 16,
            voicing: Vec::new(),
            bass: BassPart::Chord {
                chord: Chord::new(PitchClass::C, Major),
                note: Note::new(48),
                next: PitchClass::C,
            },
        };
        let bar = Meter::steady(TimeSignature::COMMON).bar(0);
        let notes = play(
            GuitarPart::Strum(Comping::Strum),
            Comping::Pulse,
            &[bar],
            &[harmony],
        );
        // The first hit: C3 E3 G3 C4 E4, a tick apart from the bottom.
        let first: Vec<(u8, usize)> = notes
            .iter()
            .filter(|note| note.start == 0)
            .map(|note| (note.pitch.midi(), note.start_tick()))
            .collect();
        assert_eq!(first, [(48, 0), (52, 1), (55, 2), (60, 3), (64, 4)]);
        // The up-stroke on step 6: the top four, from the top down.
        let mut up: Vec<(u8, usize)> = notes
            .iter()
            .filter(|note| note.start == 6)
            .map(|note| (note.pitch.midi(), note.start_tick()))
            .collect();
        up.sort_by_key(|&(_, tick)| tick);
        assert_eq!(up, [(64, 144), (60, 145), (55, 146), (52, 147)]);
        // Every string stops with the hit.
        assert!(
            notes
                .iter()
                .filter(|note| note.start == 0)
                .all(|note| note.end_tick() == 4 * TICKS_PER_STEP)
        );
    }

    #[test]
    fn a_chop_strikes_the_top_four_strings_on_two_and_four() {
        let harmony = Played {
            start: 0,
            end: 16,
            voicing: Vec::new(),
            bass: BassPart::Chord {
                chord: Chord::new(PitchClass::G, Major),
                note: Note::new(43),
                next: PitchClass::G,
            },
        };
        let bar = Meter::steady(TimeSignature::COMMON).bar(0);
        let notes = play(GuitarPart::Chop, Comping::Pulse, &[bar], &[harmony]);
        // G 320003 without its low E and A strings: D3 G3 B3 G4, down.
        let at = |step| -> Vec<(u8, usize)> {
            notes
                .iter()
                .filter(|note| note.start == step)
                .map(|note| (note.pitch.midi(), note.start_tick()))
                .collect()
        };
        assert_eq!(at(4), [(50, 96), (55, 97), (59, 98), (67, 99)]);
        assert_eq!(at(12).len(), 4);
        assert_eq!(notes.len(), 8);
    }

    #[test]
    fn the_boogie_alternates_fifth_and_sixth() {
        let harmony = |quality| Played {
            start: 0,
            end: 16,
            voicing: Vec::new(),
            bass: BassPart::Chord {
                chord: Chord::new(PitchClass::A, quality),
                note: Note::new(45),
                next: PitchClass::A,
            },
        };
        let bar = Meter::steady(TimeSignature::COMMON).bar(0);
        let tops = |quality| -> Vec<u8> {
            boogie(&[bar], &[harmony(quality)])
                .iter()
                .filter(|note| note.pitch.midi() != 45)
                .map(|note| note.pitch.midi())
                .collect()
        };
        // A with E, E, then F#, F#, on each beat's two eighths.
        assert_eq!(tops(Dominant7), [52, 52, 54, 54, 52, 52, 54, 54]);
        assert_eq!(tops(Minor), [52; 8]);
    }
}
