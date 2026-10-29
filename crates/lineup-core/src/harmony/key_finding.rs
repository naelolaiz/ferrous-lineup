//! Finding the key from the notes themselves, and where it changes.
//!
//! A key is a home note and the scale around it, and it shows in which
//! notes are played most: in C major, C, E and G (the notes of the home
//! chord) come up again and again, then the other white keys, and the
//! black keys hardly at all. A *key profile* says how much each of the
//! twelve notes belongs to a key, from listening tests or from counting
//! notes in real music. Comparing how long each note sounds with the
//! profile of every one of the 24 major and minor keys tells which key
//! fits best: the Krumhansl-Schmuckler method (1990).
//!
//! A piece that *modulates* (changes key) needs more than one answer.
//! The music is cut into bars, and every bar gets the score of every key
//! over the bars around it. Then, as for chords in `lineup-listen`, a key
//! change costs a penalty, and the Viterbi algorithm finds the sequence
//! of keys with the best total score. A new key wins only when it holds
//! for long enough to pay the penalty, so a single chord borrowed from
//! another key (a secondary dominant, say) does not count as a change.
//!
//! Every stretch in one key keeps the whole ranking, not only the
//! winner: C major and A minor have the same notes, and a stretch of
//! music can fit both. [`KeySpan::is_clear`] says whether the best key
//! is well ahead of the next one; when it is not, the app shows the
//! best key and its closest rivals beside it, fainter.
//!
//! [`KeyListener`] does the same while music is played live, from the
//! bars played so far: a new key takes over once it has been ahead for
//! a couple of bars.

use crate::i18n::{key_name, tr, trf};
use crate::pitch::{Note, PitchClass};

use super::keys::Function;
use super::{Chord, ChordQuality, Scale, ScaleKind, with_figures};

/// How long each of the twelve notes sounds, C first: a *chroma* vector,
/// in any unit (steps, seconds, energy). Only the proportions count.
pub type Chroma = [f32; 12];

/// The Albrecht-Shanahan (2013) key profiles: how often each note above
/// the tonic sounds in pieces in a major or a minor key, counted over
/// 490 pieces of classical music. Of the published profiles, these named
/// the keys of the library demos best (see the tests).
const MAJOR_PROFILE: Chroma = [
    0.238, 0.006, 0.111, 0.006, 0.137, 0.094, 0.016, 0.214, 0.009, 0.080, 0.008, 0.081,
];
const MINOR_PROFILE: Chroma = [
    0.220, 0.006, 0.104, 0.123, 0.019, 0.103, 0.012, 0.214, 0.062, 0.022, 0.061, 0.052,
];

/// The 24 keys: the twelve major keys, then the twelve minor ones.
pub fn all_keys() -> impl Iterator<Item = Scale> {
    [ScaleKind::Major, ScaleKind::NaturalMinor]
        .into_iter()
        .flat_map(|kind| {
            PitchClass::ALL
                .into_iter()
                .map(move |tonic| Scale::new(tonic, kind))
        })
}

/// `values` moved to have an average of 0 and a length of 1, so that
/// comparing two of them with a dot product gives their correlation: 1
/// when they rise and fall together, 0 when unrelated, -1 when opposite.
fn normalized(values: &Chroma) -> Option<Chroma> {
    let mean = values.iter().sum::<f32>() / 12.0;
    let centered = values.map(|v| v - mean);
    let length = centered.iter().map(|v| v * v).sum::<f32>().sqrt();
    (length > 1e-9).then(|| centered.map(|v| v / length))
}

/// The profiles of the 24 keys, in the order of [`all_keys`], each
/// normalized.
fn profiles() -> Vec<Chroma> {
    [MAJOR_PROFILE, MINOR_PROFILE]
        .iter()
        .flat_map(|profile| {
            (0..12).map(move |tonic| {
                // The profile counts from the tonic; turn it to count from C.
                let turned: Chroma = std::array::from_fn(|c| profile[(c + 12 - tonic) % 12]);
                normalized(&turned).unwrap_or([0.0; 12])
            })
        })
        .collect()
}

/// How well `chroma` fits each of the 24 keys, in the order of
/// [`all_keys`]: the correlation with its profile, from -1 to 1. All 0
/// when nothing sounds.
fn fits(chroma: &Chroma, profiles: &[Chroma]) -> Vec<f32> {
    match normalized(chroma) {
        Some(chroma) => profiles
            .iter()
            .map(|profile| profile.iter().zip(&chroma).map(|(p, c)| p * c).sum())
            .collect(),
        None => vec![0.0; profiles.len()],
    }
}

/// Every major and minor key with how well `durations` fits it, from -1
/// to 1, best first: the correlation of its profile with how long each
/// pitch class (counted up from C) sounds, in any unit. All 0 for
/// silence. [`rank_keys`] gives the same order with shares.
pub fn key_fits(durations: &[f64; 12]) -> Vec<(Scale, f64)> {
    let chroma = durations.map(|d| d as f32);
    let mut fits: Vec<(Scale, f64)> = all_keys()
        .zip(fits(&chroma, &profiles()))
        .map(|(key, fit)| (key, f64::from(fit)))
        .collect();
    // `total_cmp` orders every float, even NaN, so the sort cannot fail.
    fits.sort_by(|a, b| b.1.total_cmp(&a.1));
    fits
}

/// A key and how well the music fits it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyGuess {
    /// A major or a (natural) minor key.
    pub key: Scale,
    /// The correlation of the notes with the key's profile, from -1 to 1.
    pub fit: f32,
    /// How sure we are, compared with the other keys: the shares of all
    /// 24 keys add up to 1.
    pub share: f32,
}

/// How much the shares favour a better fit: a fit better by 0.05 gets a
/// share 1.8 times bigger. Measured on the demos, with this a key that
/// comes out clear is right 97% of the time.
const SHARE_SHARPNESS: f32 = 12.0;

/// When the best key's share is at least this many times the second's,
/// the key is clear and no alternatives are shown.
const CLEAR_RATIO: f32 = 2.0;

/// Alternatives shown beside an unclear key: at most this many...
const MAX_ALTERNATIVES: usize = 2;
/// ...each with at least this fraction of the best key's share.
const ALTERNATIVE_RATIO: f32 = 0.4;

/// The 24 keys ranked by how well `chroma` fits them, best first. Empty
/// when nothing sounds.
pub fn rank_keys(chroma: &Chroma) -> Vec<KeyGuess> {
    ranked(&fits(chroma, &profiles()))
}

fn ranked(fits: &[f32]) -> Vec<KeyGuess> {
    if fits.iter().all(|&fit| fit == 0.0) {
        return Vec::new();
    }
    let best = fits.iter().copied().fold(f32::MIN, f32::max);
    let weights: Vec<f32> = fits
        .iter()
        .map(|&fit| ((fit - best) * SHARE_SHARPNESS).exp())
        .collect();
    let total: f32 = weights.iter().sum();
    let mut guesses: Vec<KeyGuess> = all_keys()
        .zip(fits.iter().zip(&weights))
        .map(|(key, (&fit, &weight))| KeyGuess {
            key,
            fit,
            share: weight / total,
        })
        .collect();
    guesses.sort_by(|a, b| b.fit.total_cmp(&a.fit));
    guesses
}

/// Whether the first of `guesses` is well ahead of the second.
fn clear(guesses: &[KeyGuess]) -> bool {
    match guesses {
        [] => false,
        [_] => true,
        [best, second, ..] => best.share >= CLEAR_RATIO * second.share,
    }
}

/// The keys worth showing beside the first of `guesses`, fainter.
fn alternatives(guesses: &[KeyGuess]) -> &[KeyGuess] {
    if clear(guesses) {
        return &[];
    }
    let best = guesses[0].share;
    let count = guesses
        .iter()
        .skip(1)
        .take(MAX_ALTERNATIVES)
        .take_while(|guess| guess.share >= ALTERNATIVE_RATIO * best)
        .count();
    &guesses[1..1 + count]
}

/// A stretch of music in one key: from segment `start` up to `end` (bars,
/// usually; see [`key_spans`]).
#[derive(Debug, Clone, PartialEq)]
pub struct KeySpan {
    pub start: usize,
    pub end: usize,
    /// Every key, best first, ranked over the whole stretch.
    pub guesses: Vec<KeyGuess>,
}

impl KeySpan {
    /// The key that fits best.
    pub fn key(&self) -> Option<Scale> {
        self.guesses.first().map(|guess| guess.key)
    }

    /// Whether the best key is well ahead of the others. When it is not,
    /// [`KeySpan::alternatives`] are worth showing too.
    pub fn is_clear(&self) -> bool {
        clear(&self.guesses)
    }

    /// The keys that come close to the best one, best first, when the
    /// key is not clear: often the relative major or minor (same notes,
    /// another home note) or a key a fifth away (one note different).
    pub fn alternatives(&self) -> &[KeyGuess] {
        alternatives(&self.guesses)
    }
}

/// How many segments on each side of a segment count towards its score,
/// with less weight further away.
const WINDOW: usize = 2;
/// What a change of key costs, in units of correlation for one segment:
/// the new key must fit that much better, summed over the segments it
/// holds, to win.
const CHANGE_COST: f32 = 1.5;
/// How far a key change found by the windowed scores may move to the
/// segment where the notes really change.
const REFINE: usize = WINDOW + 1;

/// Where the key changes, given what sounds in each segment (each bar,
/// usually): the stretches in one key, in order, covering every segment.
/// Segments where nothing sounds take the key around them. Empty when
/// nothing sounds at all.
pub fn key_spans(segments: &[Chroma]) -> Vec<KeySpan> {
    let profiles = profiles();
    let count = profiles.len();
    if segments.iter().all(|segment| normalized(segment).is_none()) {
        return Vec::new();
    }

    // Each segment's score for each key, over the segments around it.
    let windowed: Vec<Vec<f32>> = (0..segments.len())
        .map(|i| {
            let mut sum = [0.0; 12];
            let from = i.saturating_sub(WINDOW);
            let to = (i + WINDOW + 1).min(segments.len());
            for (j, segment) in segments.iter().enumerate().take(to).skip(from) {
                let weight = (WINDOW + 1 - i.abs_diff(j)) as f32;
                for (s, v) in sum.iter_mut().zip(segment) {
                    *s += weight * v;
                }
            }
            fits(&sum, &profiles)
        })
        .collect();

    // Viterbi: best[k] is the best total score of a sequence of keys
    // ending in key k; from[i][k] the key before k at segment i on it.
    let mut best = windowed[0].clone();
    let mut from = vec![vec![0_usize; count]; segments.len()];
    for (i, here) in windowed.iter().enumerate().skip(1) {
        let (leader, leader_score) = best_of(&best);
        let mut next = vec![0.0; count];
        for k in 0..count {
            let (previous, total) = if best[k] >= leader_score - CHANGE_COST {
                (k, best[k])
            } else {
                (leader, leader_score - CHANGE_COST)
            };
            from[i][k] = previous;
            next[k] = total + here[k];
        }
        best = next;
    }
    let mut path = vec![0; segments.len()];
    let mut current = best_of(&best).0;
    for i in (0..segments.len()).rev() {
        path[i] = current;
        current = from[i][current];
    }

    // The windows blur where a change happens by a bar or two: move each
    // change to where the segments' own notes fit the new key better.
    let own: Vec<Vec<f32>> = segments.iter().map(|s| fits(s, &profiles)).collect();
    let mut changes: Vec<usize> = (1..path.len())
        .filter(|&i| path[i] != path[i - 1])
        .collect();
    for c in 0..changes.len() {
        let at = changes[c];
        let (old, new) = (path[at - 1], path[at]);
        let low = if c == 0 { 1 } else { changes[c - 1] + 1 }.max(at.saturating_sub(REFINE));
        let high = changes
            .get(c + 1)
            .map_or(path.len() - 1, |&next| next - 1)
            .min(at + REFINE);
        // The score of putting the change at `place`: the old key's fit
        // before it, the new key's from it on.
        let score = |place: usize| -> f32 {
            (low - 1..=high)
                .map(|i| if i < place { own[i][old] } else { own[i][new] })
                .sum()
        };
        let place = (low..=high)
            .max_by(|&a, &b| score(a).total_cmp(&score(b)).then(b.cmp(&a)))
            .unwrap_or(at);
        for (i, key) in path.iter_mut().enumerate().take(high + 1).skip(low) {
            *key = if i < place { old } else { new };
        }
        changes[c] = place;
    }

    // One span per run of the same key, ranked over its own notes.
    let mut spans: Vec<KeySpan> = Vec::new();
    let mut start = 0;
    for i in 1..=path.len() {
        if i == path.len() || path[i] != path[start] {
            let mut sum = [0.0; 12];
            for segment in &segments[start..i] {
                for (s, v) in sum.iter_mut().zip(segment) {
                    *s += v;
                }
            }
            let mut guesses = ranked(&fits(&sum, &profiles));
            // The span's best key is the one the path chose, even when the
            // span alone would rank another a hair ahead.
            if let Some(chosen) = guesses
                .iter()
                .position(|g| Some(g.key) == all_keys().nth(path[start]))
            {
                guesses[..=chosen].rotate_right(1);
            }
            spans.push(KeySpan {
                start,
                end: i,
                guesses,
            });
            start = i;
        }
    }
    spans
}

fn best_of(scores: &[f32]) -> (usize, f32) {
    scores
        .iter()
        .copied()
        .enumerate()
        .fold((0, f32::MIN), |a, (i, s)| if s > a.1 { (i, s) } else { a })
}

/// How much of `steps` long a note sounds within the segment from
/// `start` to `end`, all in steps.
fn overlap(note_start: usize, note_end: usize, start: usize, end: usize) -> usize {
    note_end.min(end).saturating_sub(note_start.max(start))
}

/// What sounds in each segment, from notes given as (pitch, start,
/// length) in steps and the segments' edges (`edges[i]` to
/// `edges[i + 1]` is segment `i`). Each note counts for as long as it
/// sounds in the segment.
pub fn chromas_of_notes(
    notes: impl IntoIterator<Item = (Note, usize, usize)>,
    edges: &[usize],
) -> Vec<Chroma> {
    let mut segments = vec![[0.0; 12]; edges.len().saturating_sub(1)];
    for (pitch, start, length) in notes {
        let end = start + length.max(1);
        let pc = usize::from(pitch.pitch_class().semitone());
        // The first segment the note can touch.
        let first = edges
            .partition_point(|&edge| edge <= start)
            .saturating_sub(1);
        for (i, segment) in segments.iter_mut().enumerate().skip(first) {
            if edges[i] >= end {
                break;
            }
            segment[pc] += overlap(start, end, edges[i], edges[i + 1]) as f32;
        }
    }
    segments
}

/// What a chord is in one key: its Roman numeral and the job it does
/// there. The same chord means something else in each key it could be
/// heard in: G7 is the V7 of C major, pulling home, but in A minor it is
/// bVII7, the dominant of C (bIII), pulling away to the relative major.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChordReading {
    /// The numeral, counted along the major scale of the tonic as
    /// everywhere in Ferrous Lineup (see [`Scale::roman_numeral`]): G7 in
    /// A minor is bVII7.
    pub numeral: String,
    /// What the chord does, when it is one of the key's chords.
    pub function: Option<Function>,
    /// How else it is heard: "V7/bIII" for a dominant seventh that pulls
    /// to another chord of the key, "borrowed from C minor", or "outside
    /// the key".
    pub role: Option<String>,
}

impl ChordReading {
    /// The same reading with the figures of an inversion (see
    /// [`Chord::inversion_figures`]) on its numerals: G7/F in C major
    /// reads "V⁴₂, dominant", and E7/G# "III⁶₅ = V⁶₅/vi".
    pub fn inverted(mut self, figures: &str) -> ChordReading {
        self.numeral = with_figures(&self.numeral, figures);
        if let Some(role) = &self.role
            && role.starts_with('V')
        {
            self.role = Some(with_figures(role, figures));
        }
        self
    }

    /// The reading in a few words: "V7, dominant", "bVII7 = V7/bIII",
    /// "III7 = V7/vi", "bVI, borrowed from C minor".
    pub fn label(&self) -> String {
        match (&self.role, self.function) {
            (Some(role), _) if role.starts_with('V') => format!("{} = {role}", self.numeral),
            (Some(role), _) => format!("{}, {role}", self.numeral),
            (None, Some(function)) => format!("{}, {}", self.numeral, tr(function.name())),
            (None, None) => self.numeral.clone(),
        }
    }
}

/// What `chord` is in `key`, a major or minor key. `None` for other
/// scales, which have no numerals of this kind.
pub fn read_chord(chord: Chord, key: Scale) -> Option<ChordReading> {
    if !matches!(
        key.kind,
        ScaleKind::Major
            | ScaleKind::NaturalMinor
            | ScaleKind::HarmonicMinor
            | ScaleKind::MelodicMinor
    ) {
        return None;
    }
    let numeral = key.roman_numeral(chord)?;
    let suffix = chord.quality.numeral_suffix();
    // The chord a fourth above, as a numeral, if it is a major or minor
    // chord of the key other than the tonic: what a dominant there
    // would pull to.
    let pulls_to = || {
        let degree = key.degree_of(chord.root.transpose(5))?;
        let target = key.triad(degree)?;
        (degree != 0 && matches!(target.quality, ChordQuality::Major | ChordQuality::Minor))
            .then(|| key.roman_numeral(target))
            .flatten()
    };
    if key.is_chord_of_key(chord) {
        let degree = key
            .degree_of(chord.root)
            .or_else(|| key.uses_raised_seventh(chord).then_some(6))?;
        // A dominant seventh is heard as the dominant of the chord it
        // pulls to; only V7 pulls home.
        let role = (chord.quality.is_dominant() && degree != 4)
            .then(pulls_to)
            .flatten()
            .map(|target| format!("V{suffix}/{target}"));
        return Some(ChordReading {
            numeral,
            function: Some(Function::of(degree)),
            role,
        });
    }
    let role = if let Some(parallel) = key.borrowed_from(chord) {
        trf("borrowed from {}", &[&key_name(parallel)])
    } else if let Some(target) = key.secondary_dominant_target(chord) {
        format!(
            "V{suffix}/{}",
            key.roman_numeral(target).unwrap_or_default()
        )
    } else {
        tr("outside the key").to_string()
    };
    Some(ChordReading {
        numeral,
        function: None,
        role: Some(role),
    })
}

/// How much each bar played before counts, bar after bar: a bar played
/// four bars ago counts half as much as the one being played.
const LISTENER_MEMORY: f32 = 0.84;
/// How many bars in a row a new key must be ahead before it takes over.
const LISTENER_HOLD: usize = 2;

/// Works out the key while music is played live, such as the chords you
/// play on a MIDI keyboard: from the bars played so far, the recent ones
/// counting most.
#[derive(Debug, Clone, Default)]
pub struct KeyListener {
    /// What sounded in the bars before, fading bar after bar.
    past: Chroma,
    /// What sounds in the bar being played.
    bar: Chroma,
    /// The key it settled on, if any.
    key: Option<Scale>,
    /// A key that has been ahead, and for how many bars in a row.
    challenger: Option<(Scale, usize)>,
    /// The ranking at the end of the last bar, and during this one.
    guesses: Vec<KeyGuess>,
}

impl KeyListener {
    /// A listener that starts out believing `key`, such as the song's key,
    /// so that the first chords played are heard in its light.
    pub fn starting_in(key: Option<Scale>) -> Self {
        let past = key
            .map(|key| {
                let profile = match key.kind {
                    ScaleKind::Major => MAJOR_PROFILE,
                    _ => MINOR_PROFILE,
                };
                let tonic = usize::from(key.root.semitone());
                // A bar's worth of notes shaped like the key.
                std::array::from_fn(|c| 4.0 * profile[(c + 12 - tonic) % 12])
            })
            .unwrap_or([0.0; 12]);
        KeyListener {
            past,
            key: key.filter(|key| matches!(key.kind, ScaleKind::Major | ScaleKind::NaturalMinor)),
            ..KeyListener::default()
        }
    }

    /// `notes` sounded for `steps` steps.
    pub fn hear(&mut self, notes: impl IntoIterator<Item = Note>, steps: f32) {
        for note in notes {
            self.bar[usize::from(note.pitch_class().semitone())] += steps;
        }
        self.guesses = rank_keys(&self.heard());
    }

    /// A bar ended: it joins the past, and the key may change.
    pub fn end_bar(&mut self) {
        if normalized(&self.bar).is_none() {
            return;
        }
        let heard = self.heard();
        self.past = heard.map(|v| v * LISTENER_MEMORY);
        self.bar = [0.0; 12];
        self.guesses = rank_keys(&heard);
        let Some(best) = self.guesses.first().map(|guess| guess.key) else {
            return;
        };
        if self.key.is_none() || self.key == Some(best) {
            self.key = Some(best);
            self.challenger = None;
            return;
        }
        let held = match self.challenger {
            Some((key, bars)) if key == best => bars + 1,
            _ => 1,
        };
        if held >= LISTENER_HOLD {
            self.key = Some(best);
            self.challenger = None;
        } else {
            self.challenger = Some((best, held));
        }
    }

    fn heard(&self) -> Chroma {
        std::array::from_fn(|c| self.past[c] + self.bar[c])
    }

    /// The key it settled on.
    pub fn key(&self) -> Option<Scale> {
        self.key
    }

    /// The keys ranked by what was heard so far, the settled key first.
    pub fn guesses(&self) -> Vec<KeyGuess> {
        let mut guesses = self.guesses.clone();
        if let Some(at) = guesses.iter().position(|g| Some(g.key) == self.key) {
            guesses[..=at].rotate_right(1);
        }
        guesses
    }

    /// Whether the settled key is well ahead of the others.
    pub fn is_clear(&self) -> bool {
        clear(&self.guesses())
    }

    /// The keys close to the settled one, best first, when it is not clear.
    pub fn alternatives(&self) -> Vec<KeyGuess> {
        alternatives(&self.guesses()).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PitchClass::*;

    fn notes(names: &[u8]) -> Chroma {
        let mut chroma = [0.0; 12];
        for &pc in names {
            chroma[usize::from(pc)] += 1.0;
        }
        chroma
    }

    /// A I-IV-V-I cadence's notes, in the major key on `tonic`.
    fn cadence(tonic: u8) -> Chroma {
        let t = |i: u8| (tonic + i) % 12;
        notes(&[
            t(0),
            t(4),
            t(7),
            t(0), // I
            t(5),
            t(9),
            t(0), // IV
            t(7),
            t(11),
            t(2),
            t(5), // V7
            t(0),
            t(4),
            t(7),
            t(0), // I
        ])
    }

    #[test]
    fn a_cadence_names_its_key() {
        for tonic in 0..12 {
            let ranking = rank_keys(&cadence(tonic));
            assert_eq!(
                ranking[0].key,
                Scale::new(PitchClass::from_semitone(tonic), ScaleKind::Major)
            );
        }
    }

    #[test]
    fn harmonic_minor_names_the_minor_key() {
        // A C E | D F A | E G# B D | A C E
        let ranking = rank_keys(&notes(&[9, 0, 4, 9, 2, 5, 9, 4, 8, 11, 2, 9, 0, 4, 9]));
        assert_eq!(ranking[0].key, Scale::new(A, ScaleKind::NaturalMinor));
    }

    #[test]
    fn silence_has_no_key() {
        assert!(rank_keys(&[0.0; 12]).is_empty());
        assert!(key_spans(&[[0.0; 12]; 4]).is_empty());
    }

    #[test]
    fn shares_add_up_to_one() {
        let total: f32 = rank_keys(&cadence(2)).iter().map(|g| g.share).sum();
        assert!((total - 1.0).abs() < 1e-4);
    }

    #[test]
    fn the_pentatonic_alone_is_not_clear() {
        // The five notes of the C major pentatonic, which is also the A minor
        // pentatonic: C major and A minor fit about as well.
        let span = KeySpan {
            start: 0,
            end: 1,
            guesses: rank_keys(&notes(&[0, 2, 4, 7, 9])),
        };
        assert!(!span.is_clear());
        let keys: Vec<Scale> = std::iter::once(span.key().unwrap())
            .chain(span.alternatives().iter().map(|g| g.key))
            .collect();
        assert!(keys.contains(&Scale::new(C, ScaleKind::Major)));
        assert!(keys.contains(&Scale::new(A, ScaleKind::NaturalMinor)));
    }

    #[test]
    fn a_modulation_is_found_on_its_bar() {
        // Eight bars of C major, then eight of E major.
        let mut bars = vec![cadence(0); 8];
        bars.extend(vec![cadence(4); 8]);
        let spans = key_spans(&bars);
        assert_eq!(spans.len(), 2, "{spans:?}");
        assert_eq!(spans[0].key(), Some(Scale::new(C, ScaleKind::Major)));
        assert_eq!(spans[1].key(), Some(Scale::new(E, ScaleKind::Major)));
        assert_eq!(spans[1].start, 8);
        assert_eq!(spans[1].end, 16);
    }

    #[test]
    fn one_borrowed_bar_is_not_a_modulation() {
        // C major, with one bar on the dominant of the dominant (D F# A C).
        let mut bars = vec![cadence(0); 8];
        bars[4] = notes(&[2, 6, 9, 0, 2, 6, 9, 0]);
        let spans = key_spans(&bars);
        assert_eq!(spans.len(), 1, "{spans:?}");
        assert_eq!(spans[0].key(), Some(Scale::new(C, ScaleKind::Major)));
    }

    #[test]
    fn silent_bars_take_the_key_around_them() {
        let mut bars = vec![cadence(7); 6];
        bars[2] = [0.0; 12];
        let spans = key_spans(&bars);
        assert_eq!(spans.len(), 1);
        assert_eq!((spans[0].start, spans[0].end), (0, 6));
    }

    #[test]
    fn notes_count_for_as_long_as_they_sound() {
        let c = Note::new(60);
        let g = Note::new(67);
        // Bars of 16 steps; C holds across the bar line.
        let chromas = chromas_of_notes([(c, 8, 16), (g, 0, 4)], &[0, 16, 32]);
        assert_eq!(chromas[0][0], 8.0);
        assert_eq!(chromas[1][0], 8.0);
        assert_eq!(chromas[0][7], 4.0);
        assert_eq!(chromas[1][7], 0.0);
    }

    #[test]
    fn g7_is_read_in_each_key_it_could_be_in() {
        let g7 = Chord::new(G, ChordQuality::Dominant7);
        let in_c = read_chord(g7, Scale::new(C, ScaleKind::Major)).unwrap();
        assert_eq!(in_c.numeral, "V7");
        assert_eq!(in_c.function, Some(Function::Dominant));
        assert_eq!(in_c.label(), "V7, dominant");
        // In A minor, G7 is the seventh chord on G: bVII7, numbered along
        // A major as all numerals are here, and it pulls to C (bIII).
        let in_a_minor = read_chord(g7, Scale::new(A, ScaleKind::NaturalMinor)).unwrap();
        assert_eq!(in_a_minor.numeral, "bVII7");
        assert_eq!(in_a_minor.role.as_deref(), Some("V7/bIII"));
        assert_eq!(in_a_minor.label(), "bVII7 = V7/bIII");
        // In G major it is the tonic chord with a flat seventh: the
        // dominant of IV.
        let in_g = read_chord(g7, Scale::new(G, ScaleKind::Major)).unwrap();
        assert_eq!(in_g.label(), "I7 = V7/IV");
    }

    #[test]
    fn chords_from_outside_the_key_say_where_they_come_from() {
        let c_major = Scale::new(C, ScaleKind::Major);
        let e7 = read_chord(Chord::new(E, ChordQuality::Dominant7), c_major).unwrap();
        assert_eq!(e7.label(), "III7 = V7/vi");
        let ab = read_chord(Chord::new(GSharp, ChordQuality::Major), c_major).unwrap();
        assert_eq!(ab.label(), "bVI, borrowed from C minor");
        let e = read_chord(
            Chord::new(E, ChordQuality::Major),
            Scale::new(A, ScaleKind::NaturalMinor),
        )
        .unwrap();
        assert_eq!(e.label(), "V, dominant");
        let am = read_chord(Chord::new(A, ChordQuality::Minor), c_major).unwrap();
        assert_eq!(am.label(), "vi, tonic");
    }

    #[test]
    fn a_minor_tune_fits_its_key_best() {
        // "Greensleeves" in A minor, the opening phrase.
        let tune: [(u8, f64); 20] = [
            (69, 1.0),
            (72, 2.0),
            (74, 1.0),
            (76, 1.5),
            (77, 0.5),
            (76, 1.0),
            (74, 2.0),
            (71, 1.0),
            (67, 1.5),
            (69, 0.5),
            (71, 1.0),
            (72, 2.0),
            (69, 1.0),
            (69, 1.5),
            (68, 0.5),
            (69, 1.0),
            (71, 2.0),
            (68, 1.0),
            (64, 2.0),
            (69, 3.0),
        ];
        let mut durations = [0.0; 12];
        for (midi, beats) in tune {
            durations[usize::from(midi % 12)] += beats;
        }
        assert_eq!(
            key_fits(&durations)[0].0,
            Scale::new(A, ScaleKind::NaturalMinor)
        );
    }

    #[test]
    fn every_key_is_fitted_once_best_first() {
        let mut durations = [0.0; 12];
        for pc in [2, 6, 9] {
            durations[pc] = 1.0;
        }
        let fits = key_fits(&durations);
        assert_eq!(fits.len(), 24);
        assert_eq!(fits[0].0, Scale::new(D, ScaleKind::Major));
        assert!(fits.windows(2).all(|pair| pair[0].1 >= pair[1].1));
        assert!(key_fits(&[0.0; 12]).iter().all(|(_, fit)| *fit == 0.0));
    }

    #[test]
    fn the_listener_changes_key_once_the_new_one_holds() {
        let mut listener = KeyListener::starting_in(Some(Scale::new(C, ScaleKind::Major)));
        let play = |listener: &mut KeyListener, tonic: u8| {
            let chroma = cadence(tonic);
            for (pc, &amount) in chroma.iter().enumerate() {
                if amount > 0.0 {
                    listener.hear([Note::new(60 + pc as u8)], amount * 4.0);
                }
            }
            listener.end_bar();
        };
        play(&mut listener, 0);
        assert_eq!(listener.key(), Some(Scale::new(C, ScaleKind::Major)));
        // One bar in A major is not enough...
        play(&mut listener, 9);
        assert_eq!(listener.key(), Some(Scale::new(C, ScaleKind::Major)));
        // ...but A major bar after bar is.
        for _ in 0..4 {
            play(&mut listener, 9);
        }
        assert_eq!(listener.key(), Some(Scale::new(A, ScaleKind::Major)));
    }
}
