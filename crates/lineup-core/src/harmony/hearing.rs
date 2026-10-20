//! Hearing the harmony of a stretch of real music.
//!
//! [`analyze`](super::analyze) names exactly the notes held at one
//! moment, which is right for a chord held down on a keyboard. Music
//! that plays is harder to name that way:
//!
//! - an **arpeggio** or an ostinato plays its chord one note at a time,
//!   so at any one moment only a note or two of it sounds;
//! - a **tune** adds passing notes that belong to no chord;
//! - an orchestra often plays **two chords at once**, one over the
//!   other: the *polychord*. The Rite of Spring's first dance hammers
//!   E♭7 over F♭ major; Petrushka's clarinets play C major over F♯
//!   major;
//! - some harmonies are **no stack of thirds** at all: a stack of
//!   fourths (E, A, D, G at the start of Petrushka), a cluster, a
//!   whole-tone or octatonic collection. These are named by their
//!   pitch-class set, as set theory does.
//!
//! So this module listens to a whole stretch, such as a beat, and weighs
//! each note by how long it sounds. Every chord (12 roots times every
//! quality) gets a score: how much of the sound it explains, minus a
//! heavier loss for what it leaves out, minus a small loss for each of
//! its notes that is hardly heard and for every note it has beyond a
//! triad (so a thirteenth chord, which holds almost every note, wins
//! only when its notes really sound), plus a bonus when its root is in
//! the bass. Then two chords at once: the notes are split into a lower
//! and an upper layer, by register or by instrument, and each layer is
//! named on its own. Last, the notes heard as a set. The best scores
//! win; when the next best is close, it is kept as an alternative, so
//! an ambiguous passage can show its other readings.

use super::{Chord, ChordQuality, Harmony, PitchSet};
use crate::{Note, PitchClass};

/// One note heard in a stretch of music, or around it: only what sounds
/// inside the stretch is weighed, but the notes around it tell what
/// each part is doing, such as moving its chords along with a tune.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeardNote {
    pub pitch: Note,
    /// When it starts, in steps from the start of the stretch: below 0
    /// for a note that started before it.
    pub start: f32,
    /// When it ends, in steps from the start of the stretch: beyond its
    /// length for a note that goes on after it.
    pub end: f32,
    /// Which part plays it, such as a channel: an instrument's notes are
    /// a layer of their own when two chords sound at once.
    pub part: usize,
}

/// One way of hearing a stretch: a harmony and how well it fits the
/// notes, about 1 for a chord that is all there is, less for a reading
/// that leaves notes out or needs more to explain them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reading {
    pub harmony: Harmony,
    pub fit: f32,
}

/// What a stretch of music is heard as: the best reading first, then
/// the ones close behind it.
#[derive(Debug, Clone, PartialEq)]
pub struct Hearing {
    readings: Vec<Reading>,
    /// The pitch classes heard enough to be part of the harmony, without
    /// the passing notes.
    notes: PitchSet,
    /// How long each pitch class sounds, weighed.
    profile: [f32; 12],
}

impl Hearing {
    /// The best reading: [`Harmony::Silence`] when nothing sounds.
    pub fn best(&self) -> Harmony {
        self.readings
            .first()
            .map_or(Harmony::Silence, |reading| reading.harmony)
    }

    /// How well the best reading fits the notes (see [`Reading::fit`]).
    pub fn fit(&self) -> f32 {
        self.readings.first().map_or(0.0, |reading| reading.fit)
    }

    /// The other readings that fit nearly as well as the best, best
    /// first: empty when the harmony is clear.
    pub fn alternatives(&self) -> &[Reading] {
        self.readings.get(1..).unwrap_or_default()
    }

    /// Whether one reading clearly wins.
    pub fn is_clear(&self) -> bool {
        self.alternatives().is_empty()
    }

    /// Every reading, best first.
    pub fn readings(&self) -> &[Reading] {
        &self.readings
    }

    /// The pitch classes heard enough to be part of the harmony: the
    /// passing notes are left out.
    pub fn notes(&self) -> PitchSet {
        self.notes
    }

    /// Whether `harmony`, heard just before, can go on through this
    /// stretch: nearly every sound is one of its notes, most of its notes
    /// are heard, and it has six notes at most (two triads), as a harmony
    /// of seven notes or more holds nearly a whole scale. Petrushka's
    /// chord, C major over F sharp major, goes on through a bar where
    /// the F sharp rests.
    pub fn goes_on(&self, harmony: Harmony) -> bool {
        let set = harmony.pitch_set();
        let total: f32 = self.profile.iter().sum();
        let inside: f32 = set
            .iter()
            .map(|p| self.profile[p.semitone() as usize])
            .sum();
        let heard = set.iter().filter(|&p| self.notes.contains(p)).count();
        total > 0.0
            && set.len() <= 6
            && inside >= GOES_ON * total
            && 3 * heard >= 2 * set.len()
            && self.notes.len() >= 3
    }
}

/// How much a sound left out of a chord counts against it, compared
/// with a sound the chord explains.
const OUTSIDE: f32 = 1.5;
/// The loss for each note of a chord that is not heard at all; a note
/// heard faintly loses part of it.
const MISSING: f32 = 0.15;
/// The loss for a missing fifth, which players leave out freely.
const MISSING_FIFTH: f32 = 0.05;
/// A note heard at least this much, compared with the loudest pitch
/// class, counts as fully there.
const PRESENT: f32 = 0.35;
/// The loss for every note a chord has beyond three: without it, a big
/// chord such as a thirteenth, which holds almost every note, would
/// always win.
const SIZE: f32 = 0.06;
/// The bonus when the bass plays the chord's root, and the loss when it
/// plays a note from outside the chord.
const BASS_ROOT: f32 = 0.06;
const BASS_OUTSIDE: f32 = 0.06;
/// What naming two chords at once costs: a polychord wins only when one
/// chord leaves much of the sound unexplained.
const POLYCHORD: f32 = 0.2;
/// Two chords at once are only looked for when no single chord fits
/// better than this: a tune that doubles its chord's notes a sixth
/// higher is still one harmony.
const ONE_CHORD: f32 = 0.7;
/// What naming the notes as a set costs: it explains every note, but
/// says the least about how they work together.
const SET: f32 = 0.35;
/// What naming a stack of fourths or fifths costs: a harmony of its own
/// in music since Debussy, as plain to the ear as a triad.
const QUARTAL: f32 = 0.1;
/// The sets that are stacks of fourths: C F Bb (3-9, the same notes as a
/// sus chord), C F Bb Eb (4-23) and C F Bb Eb Ab (5-35, the pentatonic
/// scale), as their prime forms.
const QUARTAL_SETS: [&[u8]; 3] = [&[0, 2, 7], &[0, 2, 5, 7], &[0, 2, 4, 7, 9]];
/// And each note of a set beyond four.
const SET_SIZE: f32 = 0.04;
/// A pitch class heard less than this much, compared with the loudest,
/// is a passing note: it is not part of the set the notes make, and it
/// does not count as a note of the harmony.
const SIGNIFICANT: f32 = 0.2;
/// How much a tune's notes weigh, compared with the chords under it,
/// and a bass line's notes from outside them (see [`weights`]).
const TUNE: f32 = 0.25;
const PASSING_BASS: f32 = 0.15;
/// A part whose chords move in parallel changes chord at least this
/// often, in steps (two beats of sixteenths), to be heard as a tune in
/// chords (see [`moves_in_parallel`]).
const PLANING_EVERY: f32 = 8.0;
/// How much of what sounds must belong to a harmony heard just before
/// for it to go on (see [`Hearing::goes_on`]).
const GOES_ON: f32 = 0.9;
/// How close behind the best a reading must be to be kept as an
/// alternative.
const CLOSE: f32 = 0.12;
/// How much of the time anything sounds each chord of a polychord must
/// sound through (see [`covered`]).
const TOGETHER: f32 = 0.8;
/// The longest gap, in steps, that still counts as sounding through (see
/// [`covered`]).
const HELD_OVER: f32 = 2.0;
/// How many chords of each layer are tried together in a polychord.
const LAYER_CHORDS: usize = 4;
/// The most readings kept.
const MOST_READINGS: usize = 3;

/// The qualities each layer of a polychord can have: major and minor
/// triads and the seventh chords built on them, as the polychords of
/// Stravinsky, Milhaud and jazz's upper structures are. Two bigger
/// chords at once would explain any notes at all; two diminished or
/// augmented chords make a symmetrical scale, octatonic or whole-tone,
/// which is heard as a set.
const LAYER_QUALITIES: [ChordQuality; 5] = [
    ChordQuality::Major,
    ChordQuality::Minor,
    ChordQuality::Dominant7,
    ChordQuality::Major7,
    ChordQuality::Minor7,
];

/// A part plays chords one note at a time (see [`breaks_chords`]) with
/// this many notes at least, most of them this many steps long at most
/// (an eighth note), and this share of its moves leaps.
const BROKEN_NOTES: usize = 4;
const BROKEN_LENGTH: f32 = 2.0;
const BROKEN_LEAPS: f32 = 0.75;

/// How well a tune alone must fit a harmony to name it (see
/// [`hear_inside`]).
const ALONE: f32 = 0.85;

/// How long each pitch class sounds in some notes.
#[derive(Debug, Clone, Copy)]
struct Profile {
    weight: [f32; 12],
    total: f32,
    /// How long the pitch class heard longest sounds.
    loudest: f32,
}

impl Profile {
    fn from_notes<'a>(notes: impl IntoIterator<Item = (&'a HeardNote, f32)>) -> Profile {
        let mut weight = [0.0; 12];
        for (note, w) in notes {
            weight[note.pitch.pitch_class().semitone() as usize] += w;
        }
        Profile {
            weight,
            total: weight.iter().sum(),
            loudest: weight.iter().copied().fold(0.0, f32::max),
        }
    }

    /// The profile of a stretch `length` steps long, where a pitch class
    /// sounding all through it is as loud as a pitch class can be, however
    /// many parts double it: a bass holding A flat under a broken chord
    /// that strikes A flat again does not make the chord's C and E flat
    /// passing notes.
    fn within(mut self, length: f32) -> Profile {
        self.loudest = self.loudest.min(length);
        self
    }

    fn of(&self, pitch_class: PitchClass) -> f32 {
        self.weight[pitch_class.semitone() as usize]
    }

    /// The pitch classes heard well enough to be a chord's root or third:
    /// a chord missing either is passed over before it is weighed.
    fn heard(&self) -> PitchSet {
        let floor = 0.5 * PRESENT * self.loudest;
        PitchClass::ALL
            .into_iter()
            .filter(|&p| self.of(p) > 0.0 && self.of(p) >= floor)
            .collect()
    }

    /// The pitch classes heard enough to be part of the harmony.
    fn significant(&self) -> PitchSet {
        let floor = SIGNIFICANT * self.loudest;
        PitchClass::ALL
            .into_iter()
            .filter(|&p| self.of(p) > 0.0 && self.of(p) >= floor)
            .collect()
    }
}

/// How well `chord` explains `profile`, before its bass is counted.
/// Also says whether its root and third (or the note that stands for
/// it in a sus chord) are both really heard.
fn chord_fit(chord: Chord, profile: &Profile) -> (f32, bool) {
    chord_fit_with(chord, profile, PitchSet::EMPTY)
}

/// [`chord_fit`], with the notes of `also` counted as explained too: in
/// a polychord, a note of one layer can belong to the other chord.
fn chord_fit_with(chord: Chord, profile: &Profile, also: PitchSet) -> (f32, bool) {
    if profile.total <= 0.0 {
        return (f32::NEG_INFINITY, false);
    }
    let explained = PitchSet::from_bits(chord.pitch_set().bits() | also.bits());
    let inside: f32 = explained.iter().map(|p| profile.of(p)).sum();
    let outside = profile.total - inside;
    let full = PRESENT * profile.loudest;
    let mut missing = 0.0;
    let mut core = true;
    for (&interval, &tone) in chord.quality.intervals().iter().zip(chord.quality.tones()) {
        let pitch_class = chord.root.transpose(i32::from(interval));
        let presence = (profile.of(pitch_class) / full).min(1.0);
        let loss = if tone == 5 && interval % 12 == 7 {
            MISSING_FIFTH
        } else {
            MISSING
        };
        missing += loss * (1.0 - presence);
        if matches!(tone, 1..=4) && presence < 0.5 {
            core = false;
        }
    }
    let extra = chord.quality.intervals().len().saturating_sub(3) as f32;
    let fit = (inside - OUTSIDE * outside) / profile.total - missing - SIZE * extra;
    (fit, core)
}

/// The bonus or loss for what the bass plays under `chord`: a bonus for
/// its root, nothing for its third, fifth or seventh (an inversion), a
/// loss for any other note. A chord over its own fourth or ninth is
/// better heard as some other harmony: D under A7sus4's A, D, E and G
/// makes the stack of fourths E, A, D, G over D.
fn bass_fit(chord: Chord, bass: Option<PitchClass>) -> f32 {
    match bass.map(|bass| (bass, chord.tone_of(bass))) {
        Some((bass, _)) if bass == chord.root => BASS_ROOT,
        Some((_, Some(3 | 5 | 7))) | None => 0.0,
        Some(_) => -BASS_OUTSIDE,
    }
}

/// The chords one layer of a polychord can make, best first: a triad or
/// a seventh chord with three of its notes heard in the layer, its root
/// among them, and its third (or the note that stands for it in a sus
/// chord) heard in the layer or in the other one (`all` is both). Rite
/// of Spring, rehearsal 14: the English horn's D flat, B flat, E flat is
/// E flat seventh, its G sounding in the C major under it.
fn layer_chords(layer: &Profile, all: &Profile, bass: Option<PitchClass>) -> Vec<Chord> {
    let mut found: Vec<(Chord, f32)> = Vec::new();
    let heard = layer.heard();
    let anywhere = PitchSet::from_bits(heard.bits() | all.heard().bits());
    for root in heard.iter() {
        for quality in LAYER_QUALITIES {
            let chord = Chord::new(root, quality);
            let tones = PitchSet::from_bits(chord.pitch_set().bits() & heard.bits());
            let core = quality
                .intervals()
                .iter()
                .zip(quality.tones())
                .filter(|&(_, &tone)| matches!(tone, 2..=4))
                .all(|(&interval, _)| anywhere.contains(root.transpose(i32::from(interval))));
            if tones.len() < 3 || !core {
                continue;
            }
            found.push((chord, chord_fit(chord, layer).0 + bass_fit(chord, bass)));
        }
    }
    found.sort_by(|a, b| b.1.total_cmp(&a.1));
    found
        .into_iter()
        .take(LAYER_CHORDS)
        .map(|(chord, _)| chord)
        .collect()
}

/// The [`MOST_READINGS`] chords that fit best of those whose root and
/// third are heard, with their fit, best first.
fn chords(profile: &Profile, bass: Option<PitchClass>) -> Vec<(Chord, f32)> {
    let heard = profile.heard();
    let mut found = Vec::with_capacity(MOST_READINGS + 1);
    for root in heard.iter() {
        for quality in ChordQuality::ALL {
            let core = quality
                .intervals()
                .iter()
                .zip(quality.tones())
                .filter(|&(_, &tone)| matches!(tone, 1..=4))
                .all(|(&interval, _)| heard.contains(root.transpose(i32::from(interval))));
            if !core {
                continue;
            }
            let chord = Chord::new(root, quality);
            let (fit, core) = chord_fit(chord, profile);
            if !core {
                continue;
            }
            // Kept best first; of two that fit as well, the one found
            // first.
            let fit = fit + bass_fit(chord, bass);
            let at = found.partition_point(|&(_, other): &(Chord, f32)| other >= fit);
            if at < MOST_READINGS {
                found.insert(at, (chord, fit));
                found.truncate(MOST_READINGS);
            }
        }
    }
    found
}

/// The best way to hear the notes as two chords at once, one over the
/// other, with its fit. The notes are split into a lower and an upper
/// layer at every gap between two pitches, and between instruments
/// from the lowest to the highest; each layer must make a triad or a
/// seventh chord of its own, with at least three notes heard, and the
/// two chords must differ by at least two notes.
fn best_polychord(
    weighed: &[Weighed],
    total: f32,
    length: f32,
    bass: Option<PitchClass>,
) -> Option<(Chord, Chord, f32)> {
    let layered: Vec<&Weighed> = weighed.iter().filter(|w| !w.tune).collect();
    let weighted: Vec<(&HeardNote, f32)> = layered.iter().map(|w| (w.note, w.weight)).collect();
    let tune = Profile::from_notes(
        weighed
            .iter()
            .filter(|w| w.tune)
            .map(|w| (w.note, w.weight)),
    );
    let all = Profile::from_notes(weighted.iter().copied());
    // Two chords of one key, F major and G major, are one harmony of that
    // key, G eleventh: two chords at once are two keys at once.
    if in_one_key(all.significant()) {
        return None;
    }
    let spans = spans(&layered, length);
    let sounding = covered(&spans, PitchSet::from_bits(0xfff), length);
    let mut splits: Vec<Vec<bool>> = Vec::new();
    // By register: everything below a pitch is the lower layer, as long
    // as no line has notes on both sides. A line that runs from A up to
    // G is an arpeggio, A seventh, not the top of G major.
    let mut lines: Vec<(usize, Note, Note)> = Vec::new();
    for w in layered.iter().filter(|w| w.line) {
        let pitch = w.note.pitch;
        match lines.iter_mut().find(|(part, ..)| *part == w.note.part) {
            Some((_, low, high)) => (*low, *high) = ((*low).min(pitch), (*high).max(pitch)),
            None => lines.push((w.note.part, pitch, pitch)),
        }
    }
    let mut pitches: Vec<Note> = weighted.iter().map(|(note, _)| note.pitch).collect();
    pitches.sort();
    pitches.dedup();
    for &split in pitches.iter().skip(1) {
        if lines
            .iter()
            .any(|&(_, low, high)| low < split && split <= high)
        {
            continue;
        }
        splits.push(
            weighted
                .iter()
                .map(|(note, _)| note.pitch >= split)
                .collect(),
        );
    }
    // By instrument, from the lowest sounding to the highest.
    let mut parts: Vec<(usize, f32)> = Vec::new();
    for &(note, w) in &weighted {
        match parts.iter_mut().find(|(part, _)| *part == note.part) {
            Some((_, sum)) => *sum += f32::from(note.pitch.midi()) * w,
            None => parts.push((note.part, f32::from(note.pitch.midi()) * w)),
        }
    }
    let mut middles: Vec<(usize, f32)> = parts
        .iter()
        .map(|&(part, sum)| {
            let weight: f32 = weighted
                .iter()
                .filter(|(note, _)| note.part == part)
                .map(|(_, w)| w)
                .sum();
            (part, sum / weight.max(f32::EPSILON))
        })
        .collect();
    middles.sort_by(|a, b| a.1.total_cmp(&b.1));
    for split in 1..middles.len() {
        let upper: Vec<usize> = middles[split..].iter().map(|&(part, _)| part).collect();
        splits.push(
            weighted
                .iter()
                .map(|(note, _)| upper.contains(&note.part))
                .collect(),
        );
    }

    let mut best: Option<(Chord, Chord, f32)> = None;
    for split in splits {
        let layer = |upper: bool| {
            Profile::from_notes(
                weighted
                    .iter()
                    .zip(&split)
                    .filter(|(_, side)| **side == upper)
                    .map(|(&(note, w), _)| (note, w)),
            )
        };
        let (lower, upper) = (layer(false), layer(true));
        if lower.significant().len() < 3 || upper.significant().len() < 3 {
            continue;
        }
        for low in layer_chords(&lower, &all, bass) {
            for high in layer_chords(&upper, &all, None) {
                // The upper chord must add two notes of its own: C over
                // Am is just Am7.
                let added = high
                    .pitch_set()
                    .iter()
                    .filter(|&p| !low.pitch_set().contains(p));
                // Nor may the two chords make one key between them.
                let both: PitchSet = low
                    .pitch_set()
                    .iter()
                    .chain(high.pitch_set().iter())
                    .collect();
                if high.root == low.root || added.count() < 2 || in_one_key(both) {
                    continue;
                }

                // And the two must sound at once, each through most of
                // the stretch: C major and then G seventh is no
                // polychord.
                let only = |a: Chord, b: Chord| -> PitchSet {
                    a.pitch_set()
                        .iter()
                        .filter(|&p| !b.pitch_set().contains(p))
                        .collect()
                };
                let (only_low, only_high) = (only(low, high), only(high, low));
                if covered(&spans, only_low, length) < TOGETHER * sounding
                    || covered(&spans, only_high, length) < TOGETHER * sounding
                {
                    continue;
                }
                // A note of one layer that belongs to the other chord,
                // such as a doubling that crosses below the lower chord,
                // is explained.
                let low_fit = chord_fit_with(low, &lower, high.pitch_set()).0 + bass_fit(low, bass);
                let high_fit = chord_fit_with(high, &upper, low.pitch_set()).0;
                // A tune's notes count as for one chord: explained if
                // they are in either chord.
                let tune_inside: f32 = both.iter().map(|p| tune.of(p)).sum();
                let tune_fit = tune_inside - OUTSIDE * (tune.total - tune_inside);
                let fit =
                    (lower.total * low_fit + upper.total * high_fit + tune_fit) / total - POLYCHORD;
                if best.is_none_or(|(.., best)| fit > best) {
                    best = Some((high, low, fit));
                }
            }
        }
    }
    best
}

/// Whether `notes` all belong to one major scale (or one of its modes).
/// The harmonic minor is left out: its notes are those of the Rite of
/// Spring's polychord, E flat seventh over F flat major.
fn in_one_key(notes: PitchSet) -> bool {
    let major = PitchSet::from_semitones(&[0, 2, 4, 5, 7, 9, 11]);
    (0..12).any(|tonic| notes.is_subset_of(major.transpose(tonic)))
}

/// When each note sounds in a stretch `length` steps long, with its
/// pitch class, earliest first, for [`covered`].
fn spans(weighed: &[&Weighed], length: f32) -> Vec<(f32, f32, PitchClass)> {
    let mut spans: Vec<(f32, f32, PitchClass)> = weighed
        .iter()
        .map(|w| {
            let note = w.note;
            (
                note.start.max(0.0),
                note.end.min(length),
                note.pitch.pitch_class(),
            )
        })
        .collect();
    spans.sort_by(|a, b| a.0.total_cmp(&b.0));
    spans
}

/// How much of a stretch `length` steps long the notes of `pitch_classes`
/// sound through, from 0 to 1, from the [`spans`] of its notes. A gap of
/// [`HELD_OVER`] steps or less between them counts as sounding, as the
/// ear holds the notes of an arpeggio or a staccato chord struck again
/// and again.
fn covered(spans: &[(f32, f32, PitchClass)], pitch_classes: PitchSet, length: f32) -> f32 {
    let mut sounding = 0.0;
    let mut current: Option<(f32, f32)> = None;
    for &(start, end, _) in spans
        .iter()
        .filter(|(.., pitch_class)| pitch_classes.contains(*pitch_class))
    {
        current = match current {
            Some((from, to)) if start <= to + HELD_OVER => Some((from, to.max(end))),
            Some((from, to)) => {
                sounding += to - from;
                Some((start, end))
            }
            None => Some((start, end)),
        };
    }
    if let Some((from, to)) = current {
        sounding += to - from;
    }
    sounding / length.max(f32::EPSILON)
}

/// How much each note weighs: how long it sounds in the stretch, less
/// for a tune and for a bass line's passing notes.
///
/// Over a part that plays chords (three notes or more at once), a part
/// that plays one note at a time is a bass line if all its notes are
/// below the chords, and a tune otherwise. The chords say what the
/// harmony is: a tune's notes weigh a quarter as much, so its passing
/// notes do not turn C into C6 or Cadd9, and a bass line's notes from
/// outside the chords weigh less still, so a walking bass passing
/// through A under C, E and G stays C, and one walking C, B flat, A
/// down to A seventh does not make it C seventh. With no part playing
/// chords, as in an orchestra whose instruments each play one line,
/// every line counts in full.
///
/// `planing` are the parts that move their chords along with a tune
/// (see [`moves_in_parallel`]): their chords colour the tune. `broken`
/// are the parts that play chords one note at a time (see
/// [`breaks_chords`]): they play chords as much as a part striking
/// them does.
fn weights<'a>(
    notes: &'a [HeardNote],
    length: f32,
    planing: &[usize],
    broken: &[usize],
) -> Vec<Weighed<'a>> {
    let span = |note: &HeardNote| (note.end.min(length) - note.start.max(0.0)).max(0.0);
    let inside = |note: &&HeardNote| span(note) > 0.0;
    // A part plays chords when three different notes sound together in
    // it; a tune doubled in octaves, thirds or sixths is still a tune,
    // and so is a tune harmonised in chords that move along with it.
    let together = |part: usize| {
        let own: Vec<&HeardNote> = notes
            .iter()
            .filter(|n| n.part == part && inside(n))
            .collect();
        own.iter().any(|a| {
            let with: PitchSet = own
                .iter()
                .filter(|b| {
                    let overlap = a.end.min(b.end) - a.start.max(b.start);
                    let shorter = (a.end - a.start).min(b.end - b.start);
                    overlap >= 0.5 * shorter
                })
                .map(|b| b.pitch.pitch_class())
                .collect();
            with.len() >= 3
        })
    };
    let mut parts: Vec<usize> = notes.iter().map(|note| note.part).collect();
    parts.sort_unstable();
    parts.dedup();
    // Chords that move along in parallel colour a tune, unless they
    // hold the bass themselves: the off-beat chords of Rachmaninoff's
    // C sharp minor prelude step down in parallel too, with the bells
    // of its bass among them, and they are the harmony.
    let bass_part = notes
        .iter()
        .filter(inside)
        .min_by_key(|note| note.pitch)
        .map(|note| note.part);
    let chordal: Vec<usize> = parts
        .iter()
        .copied()
        .filter(|&p| {
            (together(p) || broken.contains(&p)) && (!planing.contains(&p) || bass_part == Some(p))
        })
        .collect();
    let in_chords = |note: &&HeardNote| chordal.contains(&note.part) && inside(note);
    let lowest_chord = notes.iter().filter(in_chords).map(|note| note.pitch).min();
    let chord_notes: PitchSet = notes
        .iter()
        .filter(in_chords)
        .map(|note| note.pitch.pitch_class())
        .collect();
    let below = |part: usize| {
        lowest_chord.is_some_and(|low| {
            notes
                .iter()
                .filter(|note| note.part == part && inside(note))
                .all(|note| note.pitch < low)
        })
    };
    // A line that plays three notes or more of one triad or seventh
    // chord, none of them the chords', is a layer of the harmony of its
    // own, not a tune: the Rite of Spring's ostinato D flat, B flat, E
    // flat, B flat (E flat seventh, broken) over C major and E minor
    // arpeggios. A tune that shares a note with the chords, as Für
    // Elise's B, E, C over E major does, is a tune.
    let layer = |part: usize| {
        let own: PitchSet = notes
            .iter()
            .filter(|note| note.part == part && inside(note))
            .map(|note| note.pitch.pitch_class())
            .collect();
        let added = own.iter().filter(|&p| !chord_notes.contains(p)).count();
        own.len() >= 3
            && added == own.len()
            && PitchClass::ALL.into_iter().any(|root| {
                LAYER_QUALITIES
                    .iter()
                    .any(|&quality| own.is_subset_of(Chord::new(root, quality).pitch_set()))
            })
    };
    // What each part does: whether it plays a tune, and whether it
    // plays below the chords.
    let roles: Vec<(usize, bool, bool)> = parts
        .iter()
        .map(|&part| {
            let below = below(part);
            let tune = lowest_chord.is_some() && !chordal.contains(&part) && !below && !layer(part);
            (part, tune, below)
        })
        .collect();
    notes
        .iter()
        .map(|note| {
            let w = span(note);
            let (tune, below) = roles
                .iter()
                .find(|(part, ..)| *part == note.part)
                .map_or((false, false), |&(_, tune, below)| (tune, below));
            let weight = if tune {
                w * TUNE
            } else if lowest_chord.is_some()
                && below
                && !chord_notes.contains(note.pitch.pitch_class())
            {
                w * PASSING_BASS
            } else {
                w
            };
            Weighed {
                note,
                weight,
                tune,
                line: !chordal.contains(&note.part),
            }
        })
        .filter(|weighed| weighed.weight > 0.0)
        .collect()
}

/// Whether `part` moves its chords along with a tune, every voice the
/// same way by a step or a third, the way Boléro's last statements
/// harmonise their tune in chords that move along the scale: twice at
/// least, in most of the part's changes of chord, changing chord at
/// least every other time it plays, and at least every
/// [`PLANING_EVERY`] steps. Its
/// chords colour the tune; the harmony is what goes on under them. An
/// accompaniment that strikes A minor over and over and moves to G and
/// back now and then, or that rocks from G major seventh to D major
/// seventh once a bar, is not one.
fn moves_in_parallel(notes: &[HeardNote], part: usize) -> bool {
    let mut own: Vec<(f32, Note)> = notes
        .iter()
        .filter(|note| note.part == part)
        .map(|note| (note.start, note.pitch))
        .collect();
    own.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    own.dedup();
    // The chords it strikes: three notes or more starting together.
    let chords: Vec<&[(f32, Note)]> = own
        .chunk_by(|a, b| a.0 == b.0)
        .filter(|chord| chord.len() >= 3)
        .collect();
    let (mut changes, mut parallel) = (0, 0);
    for pair in chords.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        if from.iter().map(|n| n.1).eq(to.iter().map(|n| n.1)) {
            continue;
        }
        changes += 1;
        if from.len() == to.len() {
            let moves = || {
                from.iter()
                    .zip(to)
                    .map(|(a, b)| i32::from(b.1.midi()) - i32::from(a.1.midi()))
            };
            let one_way = moves().all(|m| m > 0) || moves().all(|m| m < 0);
            if one_way && moves().all(|m| m.abs() <= 4) {
                parallel += 1;
            }
        }
    }
    // A tune's chords change with most of its notes, and often; an
    // accompaniment strikes the same chord again and again, or moves on
    // once a bar or two.
    let strikes = chords.len().saturating_sub(1);
    let span = match (chords.first(), chords.last()) {
        (Some(first), Some(last)) => last[0].0 - first[0].0,
        _ => 0.0,
    };
    parallel >= 2
        && 2 * parallel >= changes
        && 2 * changes >= strikes
        && changes as f32 * PLANING_EVERY >= span
}

/// Whether `part` plays chords one note at a time, as an Alberti bass,
/// the left hand of Für Elise or the triplets of the Moonlight Sonata
/// do: short notes (an eighth of a 4/4 bar at most, as a rule), nearly
/// all a leap of a minor third or more from the one before, on three
/// pitch classes at least. A tune moves mostly by step; a bass line
/// leaping from root to fifth plays longer notes. The ear holds a
/// broken chord's notes together, so they say what the harmony is, and
/// a tune sung over them is a tune: its passing notes weigh little.
fn breaks_chords(notes: &[HeardNote], part: usize) -> bool {
    let mut own: Vec<&HeardNote> = notes.iter().filter(|note| note.part == part).collect();
    own.sort_by(|a, b| a.start.total_cmp(&b.start));
    if own.len() < BROKEN_NOTES {
        return false;
    }
    // One note at a time: each ends before the next starts, or nearly.
    let one_at_a_time = own
        .windows(2)
        .all(|pair| pair[0].end <= pair[1].start + 0.5 * (pair[1].end - pair[1].start));
    let mut lengths: Vec<f32> = own.iter().map(|note| note.end - note.start).collect();
    lengths.sort_by(f32::total_cmp);
    let short = lengths[lengths.len() / 2] <= BROKEN_LENGTH;
    let leaps = own
        .windows(2)
        .filter(|pair| {
            (i32::from(pair[1].pitch.midi()) - i32::from(pair[0].pitch.midi())).abs() >= 3
        })
        .count();
    let pitch_classes: PitchSet = own.iter().map(|note| note.pitch.pitch_class()).collect();
    one_at_a_time
        && short
        && leaps as f32 >= BROKEN_LEAPS * (own.len() - 1) as f32
        && pitch_classes.len() >= 3
}

/// A note with how much it weighs (see [`weights`]), and whether it
/// belongs to a tune: a tune's notes never make a layer of a polychord.
#[derive(Debug, Clone, Copy)]
struct Weighed<'a> {
    note: &'a HeardNote,
    weight: f32,
    tune: bool,
    /// Whether its part plays one line, not chords: a line is never
    /// split between the two layers of a polychord.
    line: bool,
}

/// The bass: the lowest note of those sounding when the stretch starts,
/// or of the first ones to start in it.
fn bass_of(weighted: &[Weighed]) -> Option<Note> {
    let first = weighted
        .iter()
        .map(|w| w.note.start.max(0.0))
        .fold(f32::INFINITY, f32::min);
    weighted
        .iter()
        .filter(|w| w.note.start.max(0.0) <= first + 0.5)
        .map(|w| w.note.pitch)
        .min()
}

/// Notes to hear stretch after stretch, such as a bar with the bars
/// around it, beat by beat: what each part does over all of them is
/// worked out once.
#[derive(Debug, Clone)]
pub struct Listening<'a> {
    notes: &'a [HeardNote],
    /// The parts that move their chords along with a tune.
    planing: Vec<usize>,
    /// The parts that play chords one note at a time.
    broken: Vec<usize>,
}

impl<'a> Listening<'a> {
    /// Listens to `notes`, timed from any point.
    pub fn new(notes: &'a [HeardNote]) -> Self {
        let mut parts: Vec<usize> = notes.iter().map(|note| note.part).collect();
        parts.sort_unstable();
        parts.dedup();
        let planing = parts
            .iter()
            .copied()
            .filter(|&part| moves_in_parallel(notes, part))
            .collect();
        let broken = parts
            .into_iter()
            .filter(|&part| breaks_chords(notes, part))
            .collect();
        Listening {
            notes,
            planing,
            broken,
        }
    }

    /// How the stretch from `start` to `end` is heard, timed as the
    /// notes are.
    pub fn hear(&self, start: f32, end: f32) -> Hearing {
        let inside: Vec<HeardNote> = self
            .notes
            .iter()
            .filter(|note| note.end > start && note.start < end)
            .map(|note| HeardNote {
                start: note.start - start,
                end: note.end - start,
                ..*note
            })
            .collect();
        hear_inside(&inside, end - start, &self.planing, &self.broken)
    }
}

/// Hears the harmony of `notes`, a stretch of music `length` steps long
/// (see the module documentation).
pub fn hear(notes: &[HeardNote], length: f32) -> Hearing {
    Listening::new(notes).hear(0.0, length)
}

/// [`hear`], with the parts that move their chords along with a tune
/// already known.
fn hear_inside(notes: &[HeardNote], length: f32, planing: &[usize], broken: &[usize]) -> Hearing {
    let weighted = weights(notes, length, planing, broken);
    let profile = Profile::from_notes(weighted.iter().map(|w| (w.note, w.weight))).within(length);
    let significant = profile.significant();
    let Some(bass_note) = bass_of(&weighted) else {
        return Hearing {
            readings: Vec::new(),
            notes: PitchSet::EMPTY,
            profile: profile.weight,
        };
    };
    let bass = bass_note.pitch_class();
    let bass = if significant.contains(bass) {
        bass
    } else {
        // A passing note in the bass is not the harmony's bass: the
        // lowest note of the harmony is.
        weighted
            .iter()
            .filter(|w| significant.contains(w.note.pitch.pitch_class()))
            .map(|w| w.note.pitch)
            .min()
            .map_or(bass, Note::pitch_class)
    };
    let mut readings: Vec<Reading> = Vec::new();
    match significant.len() {
        0 => {}
        1 => readings.push(Reading {
            harmony: Harmony::Single(bass),
            fit: 1.0,
        }),
        2 => {
            let high = significant.iter().find(|&p| p != bass).unwrap_or(bass);
            readings.push(Reading {
                harmony: Harmony::Interval {
                    low: bass,
                    high,
                    semitones: bass.semitones_up_to(high),
                },
                fit: 1.0,
            });
        }
        _ => {
            for (chord, fit) in chords(&profile, Some(bass)) {
                readings.push(Reading {
                    harmony: Harmony::Chord { chord, bass },
                    fit,
                });
            }
            let total = profile.total;
            let one_chord = readings.first().map_or(f32::NEG_INFINITY, |r| r.fit);
            let polychord = (one_chord < ONE_CHORD)
                .then(|| best_polychord(&weighted, total, length, Some(bass)))
                .flatten();
            if let Some((upper, lower, fit)) = polychord {
                readings.push(Reading {
                    harmony: Harmony::Polychord { upper, lower, bass },
                    fit,
                });
            }
            // Last, the notes as a set: a stack of fourths is cheaper.
            let named: f32 = significant.iter().map(|p| profile.of(p)).sum();
            let extra = significant.len().saturating_sub(4) as f32;
            let prime = significant.prime_form();
            let cost = if QUARTAL_SETS
                .iter()
                .any(|set| PitchSet::from_semitones(set) == prime)
            {
                QUARTAL
            } else {
                SET
            };
            readings.push(Reading {
                harmony: Harmony::Unknown(significant),
                fit: (named - OUTSIDE * (total - named)) / total - cost - SET_SIZE * extra,
            });
        }
    }
    readings.sort_by(|a, b| b.fit.total_cmp(&a.fit));
    // A tune alone, one note at a time, names a harmony only when its
    // notes make one clearly, as a broken chord or a figure on a stack of
    // fourths does: Für Elise's E, D sharp, E, B, D, C with nothing under
    // it is a tune, not C major ninth, and the harmony goes on from
    // before it.
    let part = weighted[0].note.part;
    let alone = weighted.iter().all(|w| w.line && w.note.part == part);
    if alone && readings.first().is_some_and(|reading| reading.fit < ALONE) {
        readings = vec![Reading {
            harmony: Harmony::Single(bass),
            fit: 1.0,
        }];
    }
    let best = readings.first().map_or(0.0, |reading| reading.fit);
    readings.retain(|reading| reading.fit >= best - CLOSE);
    readings.truncate(MOST_READINGS);
    Hearing {
        readings,
        notes: significant,
        profile: profile.weight,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A note held from `start` to `end` by part 0.
    fn held(midi: u8, start: f32, end: f32) -> HeardNote {
        HeardNote {
            pitch: Note::new(midi),
            start,
            end,
            part: 0,
        }
    }

    fn chord_of(hearing: &Hearing) -> Option<String> {
        hearing.best().label(crate::Spelling::Flats)
    }

    #[test]
    fn a_held_chord_is_itself() {
        let notes = [held(48, 0.0, 4.0), held(52, 0.0, 4.0), held(55, 0.0, 4.0)];
        assert_eq!(chord_of(&hear(&notes, 4.0)).unwrap(), "C");
    }

    #[test]
    fn an_arpeggio_is_its_chord() {
        // A minor broken up, one note per step: no two notes ever sound
        // together.
        let notes = [
            held(45, 0.0, 1.0),
            held(52, 1.0, 2.0),
            held(57, 2.0, 3.0),
            held(60, 3.0, 4.0),
        ];
        assert_eq!(chord_of(&hear(&notes, 4.0)).unwrap(), "Am");
    }

    #[test]
    fn a_tune_s_passing_notes_leave_the_chord_alone() {
        let mut notes = vec![held(48, 0.0, 4.0), held(52, 0.0, 4.0), held(55, 0.0, 4.0)];
        // The tune runs up C D E F over it, in another part.
        for (step, midi) in [72, 74, 76, 77].into_iter().enumerate() {
            notes.push(HeardNote {
                part: 1,
                ..held(midi, step as f32, step as f32 + 1.0)
            });
        }
        assert_eq!(chord_of(&hear(&notes, 4.0)).unwrap(), "C");
    }

    #[test]
    fn the_augurs_chord_is_e_flat_seven_over_f_flat() {
        // The Rite of Spring, rehearsal 13: E Ab B E under G Bb Db Eb.
        let notes: Vec<HeardNote> = [40, 44, 47, 52, 55, 58, 61, 63]
            .into_iter()
            .map(|midi| held(midi, 0.0, 1.0))
            .collect();
        let hearing = hear(&notes, 4.0);
        let Harmony::Polychord { upper, lower, .. } = hearing.best() else {
            panic!("{:?}", hearing);
        };
        assert_eq!(
            upper,
            Chord::new(PitchClass::DSharp, ChordQuality::Dominant7)
        );
        assert_eq!(lower, Chord::new(PitchClass::E, ChordQuality::Major));
    }

    #[test]
    fn petrushka_s_chord_is_two_arpeggios_a_tritone_apart() {
        // Two clarinets: C major and F# major broken up, their notes
        // crossing, so only the instruments tell the layers apart.
        let mut notes = Vec::new();
        for (step, (c, f_sharp)) in [(72, 70), (67, 66), (64, 61), (60, 58)]
            .into_iter()
            .enumerate()
        {
            let at = step as f32;
            notes.push(HeardNote {
                part: 1,
                ..held(c, at, at + 1.0)
            });
            notes.push(HeardNote {
                part: 2,
                ..held(f_sharp, at, at + 1.0)
            });
        }
        let hearing = hear(&notes, 4.0);
        let Harmony::Polychord { upper, lower, .. } = hearing.best() else {
            panic!("{:?}", hearing);
        };
        assert_eq!(upper, Chord::new(PitchClass::C, ChordQuality::Major));
        assert_eq!(lower, Chord::new(PitchClass::FSharp, ChordQuality::Major));
    }

    #[test]
    fn a_stack_of_fourths_is_a_set() {
        // Petrushka's fair: D E G A churning in the winds.
        let notes = [
            held(62, 0.0, 1.0),
            held(64, 1.0, 2.0),
            held(67, 2.0, 3.0),
            held(69, 3.0, 4.0),
        ];
        let hearing = hear(&notes, 4.0);
        assert_eq!(
            hearing.best(),
            Harmony::Unknown(PitchSet::from_semitones(&[2, 4, 7, 9])),
            "{hearing:?}"
        );
    }

    #[test]
    fn a_chord_with_a_seventh_heard_well_is_a_seventh_chord() {
        let notes = [
            held(43, 0.0, 4.0),
            held(59, 0.0, 4.0),
            held(62, 0.0, 4.0),
            held(65, 0.0, 4.0),
        ];
        assert_eq!(chord_of(&hear(&notes, 4.0)).unwrap(), "G7");
    }

    #[test]
    fn nothing_heard_is_silence() {
        assert_eq!(hear(&[], 4.0).best(), Harmony::Silence);
    }
}
