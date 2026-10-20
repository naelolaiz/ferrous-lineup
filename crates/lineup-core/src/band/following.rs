//! How an orchestra plays along with a score: where it strikes, and how
//! loud.
//!
//! A band style plays the same groove bar after bar. An orchestra that
//! joins a symphony cannot: the music changes meter, grows loud and
//! soft, stops, and puts its accents where the composer wants them. In
//! The Rite of Spring the whole orchestra stamps the same chord on every
//! eighth note, and every few bars the horns throw in an accent on a beat
//! nobody can predict. A bass drum on every bar would flatten that.
//!
//! So a style that follows the song (see [`BandStyle::follows_the_song`])
//! first listens to the song's own notes ([`Following::hear`]), and then
//! changes what its players wrote ([`Following::play_along`]):
//!
//! - it plays as loud as the score, bar by bar, and rests where the
//!   score rests;
//! - where the score accents a chord, the horns strike it with the score,
//!   the timpani its bass, and the bass drum with them;
//! - in a passage of such accents, they strike only there, so the
//!   accents stay where the composer put them;
//! - elsewhere the horns and the timpani keep to the beats each bar is
//!   felt in, and the bass drum marks the first beat only where the music
//!   is loud.
//!
//! [`BandStyle::follows_the_song`]: super::BandStyle::follows_the_song

use std::collections::{BTreeMap, HashMap};

use super::Player;
use crate::meter::Bar;
use crate::pattern::NoteEvent;
use crate::project::Project;

/// How much louder than the rest of its bar, and than the same place in
/// the bars before or after it, the orchestra must strike at a step for
/// it to be heard as an accent: nine fifths, nearly twice.
const TUTTI: (u32, u32) = (9, 5);

/// How many steps before and after an accent the instrument that strikes
/// it is silent: an eighth note either way. A horn that plays every eighth
/// plays a line; one that strikes a short note alone, between rests,
/// accents. One that holds a chord for bars, or plays a tune, does not.
const ALONE: usize = 2;

/// How hard a note must be struck, and how much harder than its part
/// in the same bar, to be a written accent or sforzando of its own.
const MARKED: u8 = 120;
const MARKED_SHARE: (u32, u32) = (6, 5);

/// How far around a bar the accents are looked for that make it part of
/// a passage of accents, in bars either way, and how many bars with
/// accents that takes.
const PASSAGE: usize = 4;
const PASSAGE_BARS: usize = 2;

/// How loud the music must be, as a share of mezzo-forte, for the bass
/// drum to mark the first beat of a bar without an accent: forte.
const LOUD: f32 = 1.05;

/// A chord the score accents: where, and how hard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Accent {
    pub at: usize,
    pub velocity: u8,
}

/// What an orchestra heard in the score it plays along with.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Following {
    bars: Vec<Bar>,
    /// How loud each bar is, as a share of mezzo-forte: the middle of the
    /// loudest note struck at each step of it. `None` where the score
    /// strikes nothing.
    loudness: Vec<Option<f32>>,
    accents: Vec<Accent>,
    /// For each bar, whether it is in a passage of accents.
    accented: Vec<bool>,
}

/// The middle of `values`, the lower one of the two in the middle of an
/// even number, or 0 for none.
fn middle(mut values: Vec<u32>) -> u32 {
    if values.is_empty() {
        return 0;
    }
    values.sort_unstable();
    values[(values.len() - 1) / 2]
}

impl Following {
    /// Listens to the notes `project` plays in `bars`: the song's own, on
    /// the tracks and channels that are heard, not the band's.
    pub(super) fn hear(project: &Project, bars: &[Bar]) -> Following {
        let end = bars.last().map_or(0, Bar::end);
        // What starts at each step: the channel, how hard, and how long.
        let mut struck: BTreeMap<usize, Vec<(usize, u8, usize)>> = BTreeMap::new();
        for clip in project.playlist().audible_clips() {
            let Some(pattern) = project.pattern(clip.pattern) else {
                continue;
            };
            let length = pattern.length();
            if pattern.player.is_some() || length == 0 {
                continue;
            }
            for time in 0..clip.repeats {
                let from = clip.start + time * length;
                if from >= end {
                    break;
                }
                for (channel, note) in pattern.all_notes() {
                    let heard = project
                        .channels()
                        .get(channel)
                        .is_some_and(|channel| !channel.muted);
                    let at = from + note.start;
                    if heard && at < end {
                        struck
                            .entry(at)
                            .or_default()
                            .push((channel, note.velocity, note.length));
                    }
                }
            }
        }
        let bar_of = |step: usize| bars.partition_point(|bar| bar.end() <= step);
        // Where each channel strikes, in order, and how hard in each bar.
        let mut strikes: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut velocities: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        for (&at, notes) in &struck {
            for &(channel, velocity, _) in notes {
                let steps = strikes.entry(channel).or_default();
                if steps.last() != Some(&at) {
                    steps.push(at);
                }
                velocities
                    .entry((channel, bar_of(at)))
                    .or_default()
                    .push(u32::from(velocity));
            }
        }
        let strength = |at: usize| -> u32 {
            struck.get(&at).map_or(0, |notes| {
                notes
                    .iter()
                    .map(|&(_, velocity, _)| u32::from(velocity))
                    .sum()
            })
        };
        let loudness = bars
            .iter()
            .map(|bar| {
                let loudest: Vec<u32> = struck
                    .range(bar.start..bar.end())
                    .map(|(_, notes)| {
                        notes
                            .iter()
                            .map(|&(_, v, _)| u32::from(v))
                            .max()
                            .unwrap_or(0)
                    })
                    .collect();
                (!loudest.is_empty()).then(|| middle(loudest) as f32 / 100.0)
            })
            .collect();
        let mut accents = Vec::new();
        for (index, bar) in bars.iter().enumerate() {
            let in_bar = middle(
                struck
                    .range(bar.start..bar.end())
                    .map(|(&at, _)| strength(at))
                    .collect(),
            );
            let before = index.saturating_sub(2)..index;
            let after = index + 1..(index + 3).min(bars.len());
            let mut found = Vec::new();
            for (&at, notes) in struck.range(bar.start..bar.end()) {
                let offset = at - bar.start;
                // The same place in the two bars before and in the two
                // after: the music may change on either side. The first
                // and last bars have only one side.
                let same_place = |others: std::ops::Range<usize>| {
                    let strengths: Vec<u32> = others
                        .filter(|&other| bars[other].length > offset)
                        .map(|other| strength(bars[other].start + offset))
                        .collect();
                    (!strengths.is_empty()).then(|| middle(strengths))
                };
                let elsewhere = match (same_place(before.clone()), same_place(after.clone())) {
                    (Some(before), Some(after)) => before.min(after),
                    (one, other) => one.or(other).unwrap_or(0),
                };
                let (times, by) = TUTTI;
                let stands_out =
                    strength(at) * by >= in_bar * times && strength(at) * by >= elsewhere * times;
                let beat = bar.time.unit_steps();
                let alone = notes.iter().any(|&(channel, _, length)| {
                    if length > beat {
                        return false;
                    }
                    let steps = &strikes[&channel];
                    let place = steps.partition_point(|&step| step < at);
                    let before = place
                        .checked_sub(1)
                        .is_none_or(|before| steps[before] + ALONE < at);
                    // A rest follows it, too: a tune of quarter notes
                    // is not accented.
                    let after = steps
                        .get(place + 1)
                        .is_none_or(|&after| after > at + ALONE.max(length));
                    before && after
                });
                let marked = notes.iter().any(|&(channel, velocity, _)| {
                    let in_its_bar = velocities
                        .get(&(channel, index))
                        .cloned()
                        .unwrap_or_default();
                    let (times, by) = MARKED_SHARE;
                    velocity >= MARKED && u32::from(velocity) * by >= middle(in_its_bar) * times
                });
                if (stands_out && alone) || marked {
                    let velocity = notes.iter().map(|&(_, v, _)| v).max().unwrap_or(0);
                    found.push(Accent { at, velocity });
                }
            }
            // Accents on most of a bar's eighths are not accents: the
            // music got louder.
            let eighths = bar.length.div_ceil(2);
            if found.len() * 2 <= eighths {
                accents.extend(found);
            }
        }
        let with_accents: Vec<bool> = (0..bars.len())
            .map(|index| {
                let bar = &bars[index];
                accents
                    .iter()
                    .any(|accent| (bar.start..bar.end()).contains(&accent.at))
            })
            .collect();
        let accented = (0..bars.len())
            .map(|index| {
                let around = index.saturating_sub(PASSAGE)..=(index + PASSAGE).min(bars.len() - 1);
                around.filter(|&bar| with_accents[bar]).count() >= PASSAGE_BARS
            })
            .collect();
        Following {
            bars: bars.to_vec(),
            loudness,
            accents,
            accented,
        }
    }

    /// The accents the score has.
    #[cfg(test)]
    pub(super) fn accents(&self) -> &[Accent] {
        &self.accents
    }

    /// The bar `step` is in, by its index.
    fn bar_index(&self, step: usize) -> Option<usize> {
        let index = self.bars.partition_point(|bar| bar.end() <= step);
        (index < self.bars.len()).then_some(index)
    }

    /// How loud the score is where `step` is, if it plays there.
    fn loudness_at(&self, step: usize) -> Option<f32> {
        self.bar_index(step).and_then(|index| self.loudness[index])
    }

    /// Whether `step` is in a passage of accents.
    fn in_accents(&self, step: usize) -> bool {
        self.bar_index(step)
            .is_some_and(|index| self.accented[index])
    }

    /// What `player` plays along with the score, from what its style
    /// wrote, `notes`, by channel. The horns (the piano) and the timpani
    /// (the bass) strike `held` at the accents: the chords, or the bass
    /// notes, held through each harmony. The bass drum strikes on
    /// `kick`, the channel of the kick drum.
    pub(super) fn play_along(
        &self,
        player: Player,
        notes: Vec<(usize, NoteEvent)>,
        held: &[NoteEvent],
        channel: Option<usize>,
        kick: Option<usize>,
    ) -> Vec<(usize, NoteEvent)> {
        let mut played: Vec<(usize, NoteEvent)> = notes
            .into_iter()
            .filter_map(|(channel, mut note)| {
                let loudness = self.loudness_at(note.start)?;
                let strikes = matches!(player, Player::Piano | Player::Bass | Player::Drums);
                if strikes && self.in_accents(note.start) {
                    return None;
                }
                if player == Player::Drums {
                    let first = self
                        .bar_index(note.start)
                        .is_some_and(|index| self.bars[index].start == note.start);
                    if !first || loudness < LOUD {
                        return None;
                    }
                }
                note.velocity = (f32::from(note.velocity) * loudness)
                    .round()
                    .clamp(1.0, 127.0) as u8;
                Some((channel, note))
            })
            .collect();
        let strike =
            |played: &mut Vec<(usize, NoteEvent)>, channel: usize, note: NoteEvent| match played
                .iter_mut()
                .find(|(other, known)| {
                    *other == channel && known.start == note.start && known.pitch == note.pitch
                }) {
                Some((_, known)) => known.velocity = known.velocity.max(note.velocity),
                None => played.push((channel, note)),
            };
        match (player, channel, kick) {
            (Player::Piano | Player::Bass, Some(channel), _) => {
                for (index, accent) in self.accents.iter().enumerate() {
                    let next = self
                        .accents
                        .get(index + 1)
                        .map_or(usize::MAX, |next| next.at);
                    let beat = self
                        .bar_index(accent.at)
                        .map_or(1, |bar| self.bars[bar].time.unit_steps());
                    for note in held.iter().filter(|note| {
                        note.start <= accent.at && accent.at < note.start + note.length
                    }) {
                        let length = (next - accent.at)
                            .min(note.start + note.length - accent.at)
                            .min(beat)
                            .max(1);
                        let struck = NoteEvent {
                            velocity: accent.velocity,
                            ..NoteEvent::new(note.pitch, accent.at, length)
                        };
                        strike(&mut played, channel, struck);
                    }
                }
            }
            (Player::Drums, _, Some(kick)) => {
                for accent in &self.accents {
                    let struck = NoteEvent {
                        velocity: accent.velocity,
                        ..NoteEvent::new(crate::pattern::STEP_NOTE, accent.at, 1)
                    };
                    strike(&mut played, kick, struck);
                }
            }
            _ => {}
        }
        played.sort_by_key(|(channel, note)| (note.start, *channel, note.pitch));
        played
    }
}
