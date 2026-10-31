//! Sections heard in a song: where its parts start, worked out from the
//! notes, for songs whose playlist names none.
//!
//! A band fits its patterns to the song's parts: a drummer plays a
//! phrase's pattern from its first bar and closes it with a fill on its
//! last. A demo or a song typed into the chord sheet may name its parts as
//! sections of the playlist; an imported MIDI file names none. So the
//! parts are listened for, the way a musician hears them on a recording:
//!
//! - **What comes back.** Each bar is compared with every other by its
//!   harmony (the chord sheet's chords, or else the pitch classes that
//!   sound) and by its tune (which notes start where). A run of bars that
//!   comes back later is a part, as in A A B A, a verse and a refrain.
//! - **What changes.** Where the bars before a bar line sound alike and
//!   the bars after it sound alike, but the two differ, something new
//!   starts there. A change of key, of time signature or of tempo, and
//!   instruments coming in or dropping out, say so too, and so does a
//!   tune resting at the end of its phrase.
//! - **Phrases.** Parts mostly last four or eight bars, counted from the
//!   first full bar of the tune: a pickup before it, a few notes leading
//!   into the first downbeat, is not counted. Other lengths are taken only
//!   where the music asks for them, as with a seven-bar bridge.
//!
//! The best way to cut the song, all of this counted, is found by dynamic
//! programming: the best way to cut the first `j` bars is the best way to
//! cut the first `i`, for the best `i`, plus one part from `i` to `j`.
//! The parts are then named by what they sound like: the same letter for
//! parts that sound alike (A, B, A), an intro before the first part that
//! comes back, and a coda after the last.
//!
//! The band's own parts are not listened to: the band plays what the
//! sections say, so hearing it would go round in circles.
//!
//! **Breaks** are heard the same way (see [`tune_breaks`]): where the
//! tune lands on the home chord and then rests to the end of the bar, a
//! phrase has come home and left room, and the band stops with it: it
//! strikes the chord with the tune and is silent until the next bar,
//! where the drums call it back in.

use crate::chord_sheet::{ChordMarker, HarmonyName, Stop};
use crate::harmony::{ChordFamily, ChordQuality};
use crate::meter::Bar;
use crate::pitch::PitchClass;
use crate::playlist::Section;
use crate::project::{ChannelKind, Project, SampleSource};

/// The fewest bars a song needs for its parts to be heard: a shorter one
/// is a loop, one part.
const FEWEST_BARS: usize = 8;

/// The shortest and longest part, in bars.
const SHORTEST: usize = 2;
const LONGEST: usize = 32;

/// How many bars on each side of a bar line are compared to hear that
/// something new starts there, and that instruments come in or drop out.
const WINDOW: usize = 4;

/// The longest loop a song can go round all through and still be heard
/// as one part: a vamp.
const VAMP: usize = 4;

/// How the ear weighs what it hears. [`EAR`] is the one used; its values
/// were found by scoring the parts heard in every demo against the parts
/// its score writes (see the tests).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Ear {
    /// How much a bar's tune and how the other instruments play in it
    /// count, against its harmony, when bars are compared.
    pub tune_weight: f32,
    pub texture_weight: f32,
    /// How alike two bars must be to count as the same bar played again.
    pub same_bar: f32,
    /// What each new part costs: a bar line must give more reasons than
    /// this to start one.
    pub part_cost: f32,
    /// How much a run of bars coming back counts where it starts, and,
    /// as a share of that, where it ends: a part played again often ends
    /// another way, as with a first and a second ending.
    pub comes_back: f32,
    pub run_end: f32,
    /// How much it counts that the bars before a bar line are alike, the
    /// bars after it are alike, and the two are not.
    pub something_new: f32,
    /// A change of key, of time signature or of tempo on the bar's first
    /// beat.
    pub new_key: f32,
    pub new_time: f32,
    pub new_tempo: f32,
    /// Instruments coming in or dropping out.
    pub new_instruments: f32,
    /// The tune resting through the last beat before the bar line.
    pub tune_rests: f32,
    /// A part starting on the first bar of a phrase of eight, or of four,
    /// counted from the tune's first full bar; or off them.
    pub on_eight: f32,
    pub on_four: f32,
    pub off_four: f32,
    /// What a part costs for being two or three bars long, for an odd
    /// number of bars, and for each eight bars beyond sixteen.
    pub short_part: f32,
    pub odd_part: f32,
    pub long_part: f32,
    /// What a part gains for being eight or sixteen bars long.
    pub eights: f32,
    /// How alike two parts must sound to get the same letter.
    pub same_part: f32,
    /// What counting the parts from the second bar gains when the first
    /// sounds like a pickup, and when it does not.
    pub pickup: f32,
    pub no_pickup: f32,
}

pub(crate) const EAR: Ear = Ear {
    tune_weight: 0.9,
    texture_weight: 0.5,
    same_bar: 0.95,
    part_cost: 0.8,
    comes_back: 1.25,
    run_end: 0.15,
    something_new: 0.05,
    new_key: 1.8,
    new_time: 1.6,
    new_tempo: 0.0,
    new_instruments: 0.0,
    tune_rests: 0.6,
    on_eight: 0.45,
    on_four: -0.05,
    off_four: -0.1,
    short_part: 0.05,
    odd_part: 0.25,
    long_part: 0.45,
    eights: 0.0,
    same_part: 0.7,
    pickup: 0.1,
    no_pickup: -0.9,
};

/// What one bar sounds like.
#[derive(Debug, Clone, Default)]
struct Heard {
    /// The harmony of each half of the bar: the chord sheet's chord, or
    /// else the pitch classes sounding, weighed by how long they sound.
    harmony: [Harmony; 2],
    /// The tune's notes: where each starts in the bar, and its pitch.
    tune: Vec<(usize, u8)>,
    /// Whether the tune is silent through the bar's last beat.
    tune_rests_at_end: bool,
    /// The channels that start notes in it.
    channels: u64,
    /// How the other instruments play in it: how many of their notes
    /// start in each sixteenth of the bar.
    texture: [f32; 16],
    /// Whether the key, the time signature or the tempo changes on its
    /// first beat.
    new_key: bool,
    new_time: bool,
    new_tempo: bool,
    /// The bar's first step and length.
    start: usize,
    length: usize,
}

#[derive(Debug, Clone, Default)]
enum Harmony {
    #[default]
    Silence,
    Named(HarmonyName),
    Sounding([f32; 12]),
}

impl Harmony {
    fn is_silent(&self) -> bool {
        match self {
            Harmony::Silence => true,
            Harmony::Named(name) => *name == HarmonyName::NoChord,
            Harmony::Sounding(chroma) => chroma.iter().all(|&weight| weight <= 0.0),
        }
    }

    fn likeness(&self, other: &Harmony) -> f32 {
        match (self, other) {
            (a, b) if a.is_silent() && b.is_silent() => 1.0,
            (a, b) if a.is_silent() || b.is_silent() => 0.0,
            (Harmony::Named(a), Harmony::Named(b)) => f32::from(u8::from(a == b)),
            (Harmony::Sounding(a), Harmony::Sounding(b)) => cosine(a, b),
            _ => 0.0,
        }
    }
}

fn cosine<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm = |v: &[f32; N]| v.iter().map(|x| x * x).sum::<f32>().sqrt();
    let both = norm(a) * norm(b);
    if both <= f32::EPSILON {
        0.0
    } else {
        dot / both
    }
}

impl Heard {
    /// How alike two bars sound, from 0 to 1: their harmony, their tune
    /// and how the other instruments play, weighed as `ear` says.
    fn likeness(&self, other: &Heard, ear: &Ear) -> f32 {
        let harmony = (self.harmony[0].likeness(&other.harmony[0])
            + self.harmony[1].likeness(&other.harmony[1]))
            / 2.0;
        let mut sum = harmony;
        let mut weights = 1.0;
        if let Some(tune) = tune_likeness(&self.tune, &other.tune) {
            sum += ear.tune_weight * tune;
            weights += ear.tune_weight;
        }
        let quiet = |texture: &[f32; 16]| texture.iter().all(|&count| count <= 0.0);
        let texture = match (quiet(&self.texture), quiet(&other.texture)) {
            (true, true) => 1.0,
            (false, false) => cosine(&self.texture, &other.texture),
            _ => 0.0,
        };
        sum += ear.texture_weight * texture;
        weights += ear.texture_weight;
        sum / weights
    }
}

/// How alike two bars' tunes are: the share of notes that start on the
/// same step (or one step off) on the same pitch. `None` where neither has
/// a tune.
fn tune_likeness(a: &[(usize, u8)], b: &[(usize, u8)]) -> Option<f32> {
    if a.is_empty() && b.is_empty() {
        return None;
    }
    if a.is_empty() || b.is_empty() {
        return Some(0.0);
    }
    let mut used = vec![false; b.len()];
    let mut matched = 0usize;
    for &(at, pitch) in a {
        let found = b
            .iter()
            .enumerate()
            .position(|(i, &(other, p))| !used[i] && p == pitch && at.abs_diff(other) <= 1);
        if let Some(i) = found {
            used[i] = true;
            matched += 1;
        }
    }
    Some(matched as f32 / a.len().max(b.len()) as f32)
}

/// One note of the song, on its timeline.
struct Sounding {
    channel: usize,
    start: usize,
    end: usize,
    pitch: u8,
}

impl Project {
    /// The sections the song's playlist names, or, without any, the ones
    /// heard in its notes (see [`Project::heard_sections`]): where the
    /// band and the library lay out their patterns.
    pub fn form_sections(&self) -> Vec<Section> {
        if self.playlist.sections.is_empty() {
            self.heard_sections()
        } else {
            self.playlist.sections.clone()
        }
    }

    /// The parts heard in the song's notes and chord sheet, in order,
    /// named by what they sound like (see the [module](self)). Empty for a
    /// song too short to have parts, or heard as one part all through.
    pub fn heard_sections(&self) -> Vec<Section> {
        self.heard_sections_by(&EAR)
    }

    /// The parts `ear` hears in the song.
    pub(crate) fn heard_sections_by(&self, ear: &Ear) -> Vec<Section> {
        let bars = self.heard_bars();
        let Some(starts) = cut(&bars, ear) else {
            return Vec::new();
        };
        name(&bars, &starts, ear)
    }

    /// What each bar of the song sounds like, from its first to the last
    /// one anything plays in, the band's own parts left out.
    fn heard_bars(&self) -> Vec<Heard> {
        let notes = self.song_notes();
        let markers = &self.playlist.chord_markers;
        let end = notes
            .iter()
            .map(|note| note.end)
            .chain(markers.last().map(|marker| marker.start + 1))
            .max()
            .unwrap_or(0);
        if end == 0 {
            return Vec::new();
        }
        let bars: Vec<Bar> = self
            .meter()
            .bars()
            .take_while(|bar| bar.start < end)
            .collect();
        let tune = self.tune_channel(&notes);
        let drums: Vec<bool> = self
            .channels
            .iter()
            .map(|channel| matches!(channel.kind, ChannelKind::Sampler(SampleSource::Builtin(_))))
            .collect();
        let is_drum = |channel: usize| drums.get(channel).copied().unwrap_or(true);
        let mut heard: Vec<Heard> = bars
            .iter()
            .map(|bar| Heard {
                start: bar.start,
                length: bar.length,
                ..Heard::default()
            })
            .collect();
        let bar_of = |step: usize| {
            bars.partition_point(|bar| bar.start <= step)
                .saturating_sub(1)
        };
        let mut sounding: Vec<[[f32; 12]; 2]> = vec![[[0.0; 12]; 2]; bars.len()];
        for note in &notes {
            let first = bar_of(note.start);
            if let Some(bar) = heard.get_mut(first) {
                bar.channels |= 1u64 << note.channel.min(63);
                if Some(note.channel) == tune {
                    bar.tune.push((note.start - bar.start, note.pitch));
                } else {
                    let at = (note.start - bar.start) * 16 / bar.length.max(1);
                    bar.texture[at.min(15)] += 1.0;
                }
            }
            if is_drum(note.channel) {
                continue;
            }
            // The pitch classes, by how long they sound in each half bar.
            for (index, bar) in bars.iter().enumerate().skip(first) {
                if bar.start >= note.end {
                    break;
                }
                let half = bar.start + bar.length / 2;
                for (side, (from, to)) in [(bar.start, half), (half, bar.end())]
                    .into_iter()
                    .enumerate()
                {
                    let overlap = note.end.min(to).saturating_sub(note.start.max(from));
                    sounding[index][side][usize::from(note.pitch % 12)] += overlap as f32;
                }
            }
        }
        for (index, bar) in heard.iter_mut().enumerate() {
            let halves = [bar.start, bar.start + bar.length / 2];
            for (side, &at) in halves.iter().enumerate() {
                bar.harmony[side] = if markers.is_empty() {
                    if sounding[index][side].iter().all(|&weight| weight <= 0.0) {
                        Harmony::Silence
                    } else {
                        Harmony::Sounding(sounding[index][side])
                    }
                } else {
                    self.chord_at(at)
                        .map_or(Harmony::Silence, |marker| Harmony::Named(marker.harmony))
                };
            }
            bar.tune.sort_unstable();
        }
        // A key, a time signature or a tempo that changes on a bar's
        // first beat. A tempo that slows into the bar, a ritardando,
        // closes the part before it rather than opening one.
        let on_first_beat = |step: usize| {
            let at = bar_of(step);
            bars.get(at)
                .filter(|bar| step - bar.start < bar.time.unit_steps().max(1))
                .map(|_| at)
        };
        let playlist = &self.playlist;
        for marker in &playlist.key_markers {
            if let Some(at) = on_first_beat(marker.start) {
                heard[at].new_key = true;
            }
        }
        for marker in &playlist.time_markers {
            if let Some(at) = on_first_beat(marker.start) {
                heard[at].new_time = true;
            }
        }
        for marker in playlist
            .tempo_markers
            .iter()
            .filter(|marker| marker.glide == 0)
        {
            if let Some(at) = on_first_beat(marker.start) {
                heard[at].new_tempo = true;
            }
        }
        // Whether the tune rests through each bar's last beat.
        if let Some(tune) = tune {
            let tune_notes: Vec<&Sounding> =
                notes.iter().filter(|note| note.channel == tune).collect();
            for (index, bar) in bars.iter().enumerate() {
                let beat = bar.time.unit_steps().max(1);
                let from = bar.end().saturating_sub(beat);
                let here = tune_notes
                    .iter()
                    .filter(|note| note.start < bar.end() && note.end > bar.start);
                let mut any = false;
                let mut plays = false;
                for note in here {
                    any = true;
                    plays |= note.end > from;
                }
                heard[index].tune_rests_at_end = any && !plays;
            }
        }
        heard
    }

    /// Every note the song itself plays, on its timeline: the band's
    /// patterns left out.
    fn song_notes(&self) -> Vec<Sounding> {
        let mut notes = Vec::new();
        for clip in self.playlist.clips() {
            let Some(pattern) = self.pattern(clip.pattern) else {
                continue;
            };
            if pattern.player.is_some() {
                continue;
            }
            let length = pattern.length();
            for repeat in 0..clip.repeats {
                let from = clip.start + repeat * length;
                for channel in 0..self.channels.len() {
                    for note in pattern.notes(channel) {
                        if note.start >= length {
                            continue;
                        }
                        let start = from + note.start;
                        let end = from + (note.start + note.length).min(length);
                        notes.push(Sounding {
                            channel,
                            start,
                            end: end.max(start + 1),
                            pitch: note.pitch.midi(),
                        });
                    }
                }
            }
        }
        notes.sort_by_key(|note| (note.start, note.channel));
        notes
    }

    /// The channel that carries the tune: the one called "Lead", or else
    /// the pitched one whose notes are highest on average.
    fn tune_channel(&self, notes: &[Sounding]) -> Option<usize> {
        let pitched = |channel: usize| {
            self.channels.get(channel).is_some_and(|channel| {
                !matches!(channel.kind, ChannelKind::Sampler(SampleSource::Builtin(_)))
            })
        };
        let mut totals = vec![(0u64, 0u64); self.channels.len()];
        for note in notes.iter().filter(|note| pitched(note.channel)) {
            totals[note.channel].0 += u64::from(note.pitch);
            totals[note.channel].1 += 1;
        }
        let playing = |channel: usize| totals.get(channel).is_some_and(|total| total.1 > 0);
        self.channels
            .iter()
            .position(|channel| channel.name == crate::library::apply::LEAD)
            .filter(|&channel| playing(channel))
            .or_else(|| {
                (0..totals.len())
                    .filter(|&channel| playing(channel))
                    .max_by_key(|&channel| totals[channel].0 * 1000 / totals[channel].1)
            })
    }
}

/// The bars a song breaks in, as indices into `bars`: those it is on the
/// home chord all through, as `home` says, where the tune lands on the
/// first beat and then rests to the bar's end, and comes back after it.
/// `tune` holds the tune's notes, each from its first step to its end,
/// on the timeline of `bars`. A bar of fewer than three beats has no
/// room for a break.
pub(crate) fn tune_breaks(
    bars: &[Bar],
    tune: &[(usize, usize)],
    home: impl Fn(usize) -> bool,
) -> Vec<usize> {
    let Some(last) = tune.iter().map(|&(start, _)| start).max() else {
        return Vec::new();
    };
    let mut breaks = Vec::new();
    for (index, bar) in bars.iter().enumerate() {
        let beat = bar.time.unit_steps();
        if bar.length < 3 * beat || last < bar.end() || !home(index) {
            continue;
        }
        let first_beat = bar.start..bar.start + beat;
        let lands = tune.iter().any(|&(start, end)| {
            (start <= bar.start && end > bar.start) || first_beat.contains(&start)
        });
        let rests = !tune
            .iter()
            .any(|&(start, end)| start < bar.end() && end > first_beat.end);
        if lands && rests {
            breaks.push(index);
        }
    }
    breaks
}

/// Whether `harmony` is the home chord of a key whose tonic is `tonic`:
/// its major or minor triad, or one of their sixths, sevenths or ninths,
/// but not a dominant seventh, which leads away, nor a suspended chord,
/// which still has to land on its third.
pub(crate) fn is_home_chord(root: PitchClass, quality: ChordQuality, tonic: PitchClass) -> bool {
    use ChordQuality::*;
    root == tonic
        && matches!(quality.family(), ChordFamily::Major | ChordFamily::Minor)
        && !quality.is_dominant()
        && !matches!(
            quality,
            Augmented | AugmentedMajor7 | Sus2 | Sus4 | Dominant7Sus4
        )
}

impl Project {
    /// The steps where the song breaks (see [`tune_breaks`]): the first
    /// step of each bar its tune lands on the home chord of the key in
    /// and rests after, by the chord sheet. Bars the chord sheet already
    /// stops in, or changes chord in, have none.
    pub fn heard_breaks(&self) -> Vec<usize> {
        let notes = self.song_notes();
        let Some(channel) = self.tune_channel(&notes) else {
            return Vec::new();
        };
        let tune: Vec<(usize, usize)> = notes
            .iter()
            .filter(|note| note.channel == channel)
            .map(|note| (note.start, note.end))
            .collect();
        let end = tune.iter().map(|&(_, end)| end).max().unwrap_or(0);
        let bars: Vec<Bar> = self
            .meter()
            .bars()
            .take_while(|bar| bar.start < end)
            .collect();
        let markers = &self.playlist.chord_markers;
        let keys = self.key_map();
        let home = |index: usize| {
            let bar = bars[index];
            let Some(at) = markers.iter().rposition(|marker| marker.start <= bar.start) else {
                return false;
            };
            let marker = markers[at];
            let changes = markers
                .get(at + 1)
                .is_some_and(|next| next.start < bar.end());
            let HarmonyName::Chord(symbol) = marker.harmony else {
                return false;
            };
            let chord = symbol.chord();
            !changes
                && marker.stop.is_none()
                && is_home_chord(chord.root, chord.quality, keys.key_at(bar.start).root)
        };
        tune_breaks(&bars, &tune, home)
            .into_iter()
            .map(|index| bars[index].start)
            .collect()
    }

    /// Marks the breaks heard in the song on its chord sheet, as shots
    /// (see [`Project::heard_breaks`]): the chord is struck again where
    /// the bar starts, and again where the next bar does. Returns how many
    /// it marked.
    pub fn mark_heard_breaks(&mut self) -> usize {
        let breaks = self.heard_breaks();
        let meter = self.meter();
        let length = self.song_length();
        for &start in &breaks {
            let end = meter.bar_at(start).end();
            let markers = &mut self.playlist.chord_markers;
            let Some(at) = markers.iter().rposition(|marker| marker.start <= start) else {
                continue;
            };
            let harmony = markers[at].harmony;
            if markers[at].start == start {
                markers[at].stop = Some(Stop::Shot);
            } else {
                markers.insert(
                    at + 1,
                    ChordMarker {
                        stop: Some(Stop::Shot),
                        ..ChordMarker::new(start, harmony)
                    },
                );
            }
            let comes_back = markers.iter().any(|marker| marker.start == end);
            let goes_on = markers.iter().any(|marker| marker.start > end) || length > end;
            if !comes_back && goes_on {
                let index = markers.partition_point(|marker| marker.start < end);
                markers.insert(index, ChordMarker::new(end, harmony));
            }
        }
        breaks.len()
    }
}

/// Where the song's parts start, as indices into `bars`, the first being
/// the tune's first full bar. `None` for a song too short to have parts,
/// or heard as a single part.
fn cut(bars: &[Heard], ear: &Ear) -> Option<Vec<usize>> {
    let first = bars
        .iter()
        .position(|bar| bar.channels != 0 || !bar.is_silent())?;
    if bars.len() - first < FEWEST_BARS {
        return None;
    }
    let likeness = likenesses(bars, ear);
    // A song that goes round one loop of a few bars all through, as a
    // vamp does, has no parts: nothing in it starts one more than any
    // other bar.
    let vamp = (1..=VAMP).any(|lag| {
        let lines = first..bars.len().saturating_sub(lag);
        let same = lines
            .clone()
            .filter(|&i| likeness[i][i + lag] >= ear.same_bar)
            .count();
        10 * same >= 9 * lines.len()
    });
    if vamp {
        return None;
    }
    let reasons = reasons(bars, &likeness, ear);
    // The tune may start with a pickup, a bar leading into the first
    // downbeat: the parts are counted from the bar after it. A bar the
    // harmony has not come in yet, or whose tune starts late, sounds
    // like one; the parts that come back say whether it is.
    let pickup = bars[first].is_pickup();
    let anchors = if first + 1 < bars.len() {
        vec![first, first + 1]
    } else {
        vec![first]
    };
    let (_, starts) = anchors
        .into_iter()
        .map(|anchor| {
            let (score, starts) = best_cut(bars.len(), anchor, &reasons, ear);
            let leads_in = match (anchor > first, pickup) {
                (false, _) => 0.0,
                (true, true) => ear.pickup,
                (true, false) => ear.no_pickup,
            };
            (score + leads_in, starts)
        })
        .max_by(|a, b| a.0.total_cmp(&b.0))?;
    (starts.len() >= 2).then_some(starts)
}

impl Heard {
    fn is_silent(&self) -> bool {
        self.harmony.iter().all(Harmony::is_silent) && self.tune.is_empty()
    }

    /// Whether it leads into the tune: its harmony silent and a tune in
    /// it, or its tune coming in after its first beat.
    fn is_pickup(&self) -> bool {
        let Some(&(first, _)) = self.tune.first() else {
            return false;
        };
        let silent = self.harmony.iter().all(Harmony::is_silent);
        silent || first >= self.length / 4
    }
}

/// How alike each bar is to each other one.
fn likenesses(bars: &[Heard], ear: &Ear) -> Vec<Vec<f32>> {
    let n = bars.len();
    let mut likeness = vec![vec![0.0; n]; n];
    for i in 0..n {
        likeness[i][i] = 1.0;
        for j in i + 1..n {
            let alike = bars[i].likeness(&bars[j], ear);
            likeness[i][j] = alike;
            likeness[j][i] = alike;
        }
    }
    likeness
}

/// How alike bars `from..from + length` are, on average, to the bars
/// `lag` later.
fn run_likeness(likeness: &[Vec<f32>], from: usize, length: usize, lag: usize) -> f32 {
    if length == 0 {
        return 0.0;
    }
    (from..from + length)
        .map(|i| likeness[i][i + lag])
        .sum::<f32>()
        / length as f32
}

/// For each bar line, how surely a part starts there because bars come
/// back.
fn returns(likeness: &[Vec<f32>], ear: &Ear) -> Vec<f32> {
    let same = ear.same_bar;
    let n = likeness.len();
    // What comes back: a run of bars like the bars some way later. Where
    // it starts and where its return starts are where parts start; where
    // they end, less surely. A run that goes on longer than the distance
    // to its return is a part played over and over: each time round
    // starts one, unless it is a shorter part played over and over (a
    // vamp of one or two bars is not a part).
    let mut back = vec![0.0f32; n + 1];
    let mut mark = |at: usize, weight: f32| {
        if at <= n {
            back[at] = back[at].max(weight);
        }
    };
    for lag in 2..n {
        let mut i = 0;
        while i + lag < n {
            if likeness[i][i + lag] < same {
                i += 1;
                continue;
            }
            let start = i;
            while i + lag < n && likeness[i][i + lag] >= same {
                i += 1;
            }
            let length = i - start;
            if length < SHORTEST {
                continue;
            }
            if length < lag + SHORTEST {
                let weight = length.min(8) as f32 / 8.0;
                mark(start, weight);
                mark(start + lag, weight);
                mark(i, weight * ear.run_end);
                mark(i + lag, weight * ear.run_end);
            } else if lag >= 4
                && (1..lag)
                    .filter(|shorter| lag.is_multiple_of(*shorter))
                    .all(|shorter| {
                        run_likeness(likeness, start, length + lag - shorter, shorter) < same
                    })
            {
                let weight = lag.min(8) as f32 / 8.0;
                let mut at = start;
                while at <= i + lag {
                    mark(at, weight);
                    at += lag;
                }
            }
        }
    }
    back
}

/// How many reasons each bar line gives to start a part there, before
/// the phrases are counted.
fn reasons(bars: &[Heard], likeness: &[Vec<f32>], ear: &Ear) -> Vec<f32> {
    let n = bars.len();
    let back = returns(likeness, ear);
    // Something new: the bars on each side alike among themselves and
    // unlike across the line, beyond what is usual in the song.
    let mean = |rows: std::ops::Range<usize>, columns: std::ops::Range<usize>| {
        let mut sum = 0.0;
        let mut count = 0.0;
        for i in rows {
            for j in columns.clone() {
                sum += likeness[i][j];
                count += 1.0;
            }
        }
        if count > 0.0 { sum / count } else { 0.0 }
    };
    let mut new = vec![0.0f32; n + 1];
    for (line, slot) in new.iter_mut().enumerate().take(n).skip(1) {
        let before = line.saturating_sub(WINDOW)..line;
        let after = line..(line + WINDOW).min(n);
        let within =
            (mean(before.clone(), before.clone()) + mean(after.clone(), after.clone())) / 2.0;
        let across = mean(before, after);
        *slot = (within - across).max(0.0);
    }
    let usual = new.iter().sum::<f32>() / n.max(1) as f32;
    let most = new.iter().copied().fold(0.0f32, f32::max) - usual;
    for value in &mut new {
        *value = if most > f32::EPSILON {
            ((*value - usual) / most).max(0.0)
        } else {
            0.0
        };
    }
    // The instruments that play in most bars of a stretch.
    let playing = |range: std::ops::Range<usize>| {
        let bars_long = range.len().max(1);
        let mut counts = [0usize; 64];
        for bar in range {
            for (channel, count) in counts.iter_mut().enumerate() {
                if bars[bar].channels & (1u64 << channel) != 0 {
                    *count += 1;
                }
            }
        }
        counts
            .iter()
            .enumerate()
            .filter(|(_, count)| 2 * **count >= bars_long)
            .fold(0u64, |set, (channel, _)| set | (1u64 << channel))
    };
    let mut reasons = vec![0.0f32; n + 1];
    for line in 1..n {
        let mut reason = ear.comes_back * back[line] + ear.something_new * new[line];
        let before = playing(line.saturating_sub(WINDOW)..line);
        let after = playing(line..(line + WINDOW).min(n));
        let union = (before | after).count_ones();
        if union > 0 {
            let changed = 1.0 - (before & after).count_ones() as f32 / union as f32;
            reason += ear.new_instruments * changed;
        }
        if bars[line - 1].tune_rests_at_end && !bars[line].tune.is_empty() {
            reason += ear.tune_rests;
        }
        let bar = &bars[line];
        reason += [
            (bar.new_key, ear.new_key),
            (bar.new_time, ear.new_time),
            (bar.new_tempo, ear.new_tempo),
        ]
        .iter()
        .filter(|(changes, _)| *changes)
        .map(|(_, weight)| weight)
        .fold(0.0f32, |most, &weight| most.max(weight));
        reasons[line] = reason;
    }
    reasons
}

/// What a part of `length` bars costs for its length: parts are mostly
/// four, eight or sixteen bars long.
fn length_cost(length: usize, ear: &Ear) -> f32 {
    let mut cost = 0.0;
    if length < 4 {
        cost += ear.short_part;
    }
    if length % 2 == 1 {
        cost += ear.odd_part;
    } else if length % 4 == 2 {
        cost += ear.odd_part / 2.0;
    }
    if length > 16 {
        cost += ear.long_part * (length - 16) as f32 / 8.0;
    }
    if length == 8 || length == 16 {
        cost -= ear.eights;
    }
    cost
}

/// The best way to cut bars `anchor` to `n` into parts, with `reasons`
/// to start one at each bar line: its score and where each part starts.
fn best_cut(n: usize, anchor: usize, reasons: &[f32], ear: &Ear) -> (f32, Vec<usize>) {
    let phrase = |line: usize| {
        let from = line - anchor;
        if from.is_multiple_of(8) {
            ear.on_eight
        } else if from.is_multiple_of(4) {
            ear.on_four
        } else {
            ear.off_four
        }
    };
    let mut best = vec![f32::NEG_INFINITY; n + 1];
    let mut from = vec![usize::MAX; n + 1];
    best[anchor] = 0.0;
    for end in anchor + 1..=n {
        for start in end.saturating_sub(LONGEST).max(anchor)..end {
            if best[start] == f32::NEG_INFINITY {
                continue;
            }
            let length = end - start;
            // The last part may be any length: the song ends where it
            // ends.
            let last = end == n;
            if length < SHORTEST && !last {
                continue;
            }
            let opens = if start == anchor {
                0.0
            } else {
                reasons[start] + phrase(start) - ear.part_cost
            };
            let shape = if last { 0.0 } else { length_cost(length, ear) };
            let score = best[start] + opens - shape;
            if score > best[end] {
                best[end] = score;
                from[end] = start;
            }
        }
    }
    let mut starts = Vec::new();
    let mut at = n;
    while at != anchor && from[at] != usize::MAX {
        at = from[at];
        starts.push(at);
    }
    starts.reverse();
    (best[n], starts)
}

/// The parts starting at `starts` (indices into `bars`), named by what
/// they sound like, as sections.
fn name(bars: &[Heard], starts: &[usize], ear: &Ear) -> Vec<Section> {
    let n = bars.len();
    let spans: Vec<(usize, usize)> = starts
        .iter()
        .enumerate()
        .map(|(i, &start)| (start, starts.get(i + 1).copied().unwrap_or(n)))
        .collect();
    let likeness = likenesses(bars, ear);
    let alike = |a: (usize, usize), b: (usize, usize)| {
        let (la, lb) = (a.1 - a.0, b.1 - b.0);
        let shorter = la.min(lb);
        if shorter == 0 || (la.max(lb) as f32) > 1.5 * shorter as f32 {
            return 0.0;
        }
        (0..shorter)
            .map(|k| likeness[a.0 + k][b.0 + k])
            .sum::<f32>()
            / shorter as f32
    };
    let mut letters: Vec<usize> = Vec::new();
    let mut next = 0;
    for (i, &span) in spans.iter().enumerate() {
        let earlier = (0..i).find(|&j| alike(spans[j], span) >= ear.same_part);
        letters.push(match earlier {
            Some(j) => letters[j],
            None => {
                next += 1;
                next - 1
            }
        });
    }
    let comes_back = |letter: usize| letters.iter().filter(|&&l| l == letter).count() > 1;
    let any_back = letters.iter().any(|&letter| comes_back(letter));
    let last = spans.len() - 1;
    let without_tune = |(from, to): (usize, usize)| (from..to).all(|bar| bars[bar].tune.is_empty());
    let intro = spans.len() >= 3
        && any_back
        && !comes_back(letters[0])
        && (spans[0].1 - spans[0].0 <= 8 || without_tune(spans[0]));
    let coda = spans.len() >= 3
        && any_back
        && !comes_back(letters[last])
        && spans[last].1 - spans[last].0 <= 8;
    // The letters count from the first part that is not the intro.
    let mut renamed: Vec<Option<char>> = vec![None; next];
    let mut letter = b'A';
    spans
        .iter()
        .enumerate()
        .map(|(i, &(start, end))| {
            let name = if i == 0 && intro {
                "Intro".to_string()
            } else if i == last && coda {
                "Coda".to_string()
            } else {
                let named = renamed[letters[i]].get_or_insert_with(|| {
                    let this = char::from(letter);
                    letter = letter.saturating_add(1).min(b'Z');
                    this
                });
                named.to_string()
            };
            let from = bars[start].start;
            let to = bars
                .get(end)
                .map_or(bars[n - 1].start + bars[n - 1].length, |bar| bar.start);
            Section::new(name, from, to - from)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::Genre;
    use crate::library::demo::song_demo;
    use crate::library::song::SONGS;

    /// The bar each part of a demo starts on, as its form writes it.
    fn written(project: &Project) -> Vec<usize> {
        let bars: Vec<Bar> = project.meter().bars().take(4096).collect();
        let bar_of = |step: usize| {
            bars.partition_point(|bar| bar.start <= step)
                .saturating_sub(1)
        };
        let mut written: Vec<usize> = project
            .playlist
            .clips()
            .iter()
            .filter(|clip| clip.track == 0)
            .filter(|clip| {
                project.pattern(clip.pattern).is_some_and(|pattern| {
                    !pattern.name.contains(", bar ") && pattern.name != "Ending"
                })
            })
            .map(|clip| bar_of(clip.start))
            .collect();
        written.sort_unstable();
        written.dedup();
        written
    }

    /// How many of `written`'s starts (the first left out) `heard` finds
    /// within `slack` bars, and how many of `heard`'s are near one.
    fn matches(written: &[usize], heard: &[usize], slack: usize) -> (usize, usize) {
        let near = |a: usize, list: &[usize]| list.iter().any(|&b| a.abs_diff(b) <= slack);
        let found = written.iter().skip(1).filter(|&&w| near(w, heard)).count();
        let right = heard.iter().skip(1).filter(|&&h| near(h, written)).count();
        (found, right)
    }

    struct Demo {
        name: &'static str,
        /// "C" with its chord sheet, "N" from its notes alone, as an
        /// imported MIDI file comes, and "H" with the chords heard in its
        /// notes, as a band writes them for an imported file.
        mode: &'static str,
        bars: Vec<Heard>,
        written: Vec<usize>,
    }

    /// The demos to score against, each in the `modes` given (see
    /// [`Demo::mode`]): songs, not the classical pieces, whose parts are
    /// cut finer than a band plays.
    fn demos(modes: &[&str]) -> Vec<Demo> {
        let mut demos = Vec::new();
        for song in SONGS
            .iter()
            .filter(|song| song.genre != Genre::Classical && !song.parts_are_numbered())
        {
            let project = song_demo(song);
            let written = written(&project);
            let mut notes_only = project.clone();
            notes_only.playlist.chord_markers.clear();
            let heard_chords = modes.contains(&"H").then(|| {
                let mut heard_chords = notes_only.clone();
                heard_chords.write_heard_chords();
                heard_chords
            });
            for (mode, project) in [
                ("C", Some(&project)),
                ("N", Some(&notes_only)),
                ("H", heard_chords.as_ref()),
            ] {
                let Some(project) = project.filter(|_| modes.contains(&mode)) else {
                    continue;
                };
                demos.push(Demo {
                    name: song.name,
                    mode,
                    bars: project.heard_bars(),
                    written: written.clone(),
                });
            }
        }
        demos
    }

    #[derive(Default, Debug)]
    struct Score {
        written: usize,
        heard: usize,
        found: usize,
        right: usize,
    }

    impl Score {
        fn f1(&self) -> f32 {
            let recall = self.found as f32 / self.written.max(1) as f32;
            let precision = self.right as f32 / self.heard.max(1) as f32;
            if recall + precision <= 0.0 {
                0.0
            } else {
                2.0 * recall * precision / (recall + precision)
            }
        }
    }

    fn heard(demo: &Demo, ear: &Ear) -> Vec<usize> {
        cut(&demo.bars, ear).unwrap_or_default()
    }

    fn score(demos: &[Demo], ear: &Ear) -> (Score, Score) {
        let mut exact = Score::default();
        let mut near = Score::default();
        for demo in demos {
            let heard = heard(demo, ear);
            for (slack, total) in [(0, &mut exact), (1, &mut near)] {
                let (found, right) = matches(&demo.written, &heard, slack);
                total.written += demo.written.len().saturating_sub(1);
                total.heard += heard.len().saturating_sub(1);
                total.found += found;
                total.right += right;
            }
        }
        (exact, near)
    }

    /// Demos the ear hears just as their scores write them, with their
    /// chord sheets and from their notes alone: a change to the ear must
    /// not lose them.
    #[test]
    fn the_ear_hears_these_demos_as_written() {
        for name in [
            "Garota de Ipanema",
            "La Cumparsita",
            "Tea for Two",
            "Sweet Georgia Brown",
            "Deep River",
            "Torna a Surriento",
            "Chamamé del estero",
            "Dorian Strut",
        ] {
            let song = SONGS.iter().find(|song| song.name == name).unwrap();
            let project = song_demo(song);
            let written = written(&project);
            let mut notes_only = project.clone();
            notes_only.playlist.chord_markers.clear();
            for project in [&project, &notes_only] {
                let heard = cut(&project.heard_bars(), &EAR).unwrap_or_default();
                assert_eq!(heard, written, "{name}");
            }
        }
    }

    /// Over all the demos, the ear finds most parts where their scores
    /// start them, with the chord sheet and without: it found 63.7% and
    /// 65.1% (F1) when this test was written, and should not get worse.
    #[test]
    fn the_ear_hears_most_parts_of_the_demos() {
        let demos = demos(&["C", "N"]);
        for mode in ["C", "N"] {
            let some: Vec<&Demo> = demos.iter().filter(|demo| demo.mode == mode).collect();
            let mut exact = Score::default();
            for demo in some {
                let heard = heard(demo, &EAR);
                let (found, right) = matches(&demo.written, &heard, 0);
                exact.written += demo.written.len().saturating_sub(1);
                exact.heard += heard.len().saturating_sub(1);
                exact.found += found;
                exact.right += right;
            }
            assert!(exact.f1() >= 0.63, "{mode}: {exact:?} F1 {:.3}", exact.f1());
        }
    }

    /// Scores the recognizer against the demos' written forms. Run with
    /// `--ignored --nocapture` to see each demo.
    #[test]
    #[ignore = "a report, not a check"]
    fn score_against_the_demos() {
        let demos = demos(&["C", "N", "H"]);
        for demo in &demos {
            let heard = heard(demo, &EAR);
            println!(
                "{} {:40} written {:?}\n{:42}heard   {heard:?}",
                demo.mode, demo.name, demo.written, ""
            );
        }
        for mode in ["C", "N", "H"] {
            let some: Vec<&Demo> = demos.iter().filter(|demo| demo.mode == mode).collect();
            let (mut exact, mut near) = (Score::default(), Score::default());
            for demo in some {
                let heard = heard(demo, &EAR);
                for (slack, total) in [(0, &mut exact), (1, &mut near)] {
                    let (found, right) = matches(&demo.written, &heard, slack);
                    total.written += demo.written.len().saturating_sub(1);
                    total.heard += heard.len().saturating_sub(1);
                    total.found += found;
                    total.right += right;
                }
            }
            println!("{mode} exact {exact:?} F1 {:.3}", exact.f1());
            println!("{mode} near  {near:?} F1 {:.3}", near.f1());
        }
    }

    /// A value of the [`Ear`] to tune, by name.
    type Field = (&'static str, fn(&mut Ear) -> &mut f32);

    /// Tunes the [`Ear`] on the demos, one value at a time, and prints the
    /// best found. Run with `--ignored --nocapture`.
    #[test]
    #[ignore = "tunes the ear, slowly"]
    fn tune_the_ear() {
        let demos = demos(&["C", "N", "H"]);
        let mut ear = EAR;
        let fields: [Field; 22] = [
            ("pickup", |e| &mut e.pickup),
            ("no_pickup", |e| &mut e.no_pickup),
            ("tune_weight", |e| &mut e.tune_weight),
            ("texture_weight", |e| &mut e.texture_weight),
            ("same_bar", |e| &mut e.same_bar),
            ("part_cost", |e| &mut e.part_cost),
            ("comes_back", |e| &mut e.comes_back),
            ("run_end", |e| &mut e.run_end),
            ("something_new", |e| &mut e.something_new),
            ("new_key", |e| &mut e.new_key),
            ("new_time", |e| &mut e.new_time),
            ("new_tempo", |e| &mut e.new_tempo),
            ("new_instruments", |e| &mut e.new_instruments),
            ("tune_rests", |e| &mut e.tune_rests),
            ("on_eight", |e| &mut e.on_eight),
            ("on_four", |e| &mut e.on_four),
            ("off_four", |e| &mut e.off_four),
            ("short_part", |e| &mut e.short_part),
            ("odd_part", |e| &mut e.odd_part),
            ("long_part", |e| &mut e.long_part),
            ("eights", |e| &mut e.eights),
            ("same_part", |e| &mut e.same_part),
        ];
        let value = |ear: &Ear| {
            let (exact, near) = score(&demos, ear);
            exact.f1() + 0.5 * near.f1()
        };
        let mut best = value(&ear);
        println!("start {best:.4}");
        for round in 0..3 {
            for (name, field) in &fields {
                for step in [-0.3f32, -0.1, 0.1, 0.3] {
                    let mut trial = ear;
                    *field(&mut trial) += step;
                    let got = value(&trial);
                    if got > best + 1e-4 {
                        best = got;
                        ear = trial;
                        println!("round {round} {name} {step:+} -> {best:.4}");
                    }
                }
            }
        }
        println!("{ear:#?}");
        let (exact, near) = score(&demos, &ear);
        println!("exact {exact:?} F1 {:.3}", exact.f1());
        println!("near  {near:?} F1 {:.3}", near.f1());
    }
}

#[cfg(test)]
mod debug {
    use super::*;
    use crate::library::demo::song_demo;
    use crate::library::song::SONGS;

    /// Prints what the ear hears in the demos named in `HEAR` (a comma
    /// separated list): each bar's reasons. Run with `--ignored
    /// --nocapture`.
    #[test]
    #[ignore = "a report"]
    fn reasons_in_named_demos() {
        let names = std::env::var("HEAR").unwrap_or_default();
        for name in names.split(',').filter(|name| !name.is_empty()) {
            let song = SONGS.iter().find(|song| song.name == name).unwrap();
            let project = song_demo(song);
            let bars = project.heard_bars();
            let likeness = likenesses(&bars, &EAR);
            let reasons = reasons(&bars, &likeness, &EAR);
            println!("{name}: pickup {}", bars[0].is_pickup());
            for (index, bar) in bars.iter().enumerate() {
                println!(
                    "  {index:3} reason {:5.2} tune {:?} next {:.2} lag8 {:.2}",
                    reasons[index],
                    bar.tune.iter().map(|(at, _)| *at).collect::<Vec<_>>(),
                    likeness[index].get(index + 1).copied().unwrap_or(0.0),
                    likeness[index].get(index + 8).copied().unwrap_or(0.0),
                );
            }
            println!("  cut {:?}", cut(&bars, &EAR));
        }
    }
}

#[cfg(test)]
mod breaks {
    use super::*;
    use crate::TimeSignature;
    use crate::library::demo::song_demo;
    use crate::library::song::SONGS;
    use crate::meter::Meter;

    fn bars(count: usize) -> Vec<Bar> {
        Meter::new(TimeSignature::COMMON, [])
            .bars()
            .take(count)
            .collect()
    }

    #[test]
    fn the_tune_breaks_where_it_lands_on_the_home_chord_and_rests() {
        let bars = bars(4);
        // Bar 1 sung through, bar 2 one beat then a rest, bar 3 sung
        // through, bar 4 one beat and the end.
        let tune = [(0, 16), (16, 20), (32, 48), (48, 52)];
        assert_eq!(tune_breaks(&bars, &tune, |_| true), vec![1]);
        // Not on the home chord: no break.
        assert_eq!(
            tune_breaks(&bars, &tune, |bar| bar != 1),
            Vec::<usize>::new()
        );
        // A note held into the bar lands on it too.
        let held = [(0, 20), (32, 48)];
        assert_eq!(tune_breaks(&bars, &held, |_| true), vec![1]);
        // A tune that goes on singing after the first beat does not break.
        let sung = [(0, 16), (16, 20), (24, 28), (32, 48)];
        assert_eq!(tune_breaks(&bars, &sung, |_| true), Vec::<usize>::new());
        // A tune that has not come back after the bar is the song's end.
        assert_eq!(
            tune_breaks(&bars, &tune[..2], |_| true),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn a_bar_of_two_beats_has_no_room_for_a_break() {
        let bars: Vec<Bar> = Meter::new(TimeSignature::new(2, 4), [])
            .bars()
            .take(3)
            .collect();
        let tune = [(0, 8), (8, 10), (16, 24)];
        assert_eq!(tune_breaks(&bars, &tune, |_| true), Vec::<usize>::new());
    }

    #[test]
    fn the_home_chord_is_the_tonic_triad_and_its_colours_but_not_a_seventh() {
        use ChordQuality::*;
        let c = PitchClass::C;
        for quality in [Major, Minor, Major6, Major7, Minor7, Add9] {
            assert!(is_home_chord(c, quality, c), "{quality:?}");
        }
        for quality in [Dominant7, Diminished, Augmented, Sus4, Dominant7Sus4] {
            assert!(!is_home_chord(c, quality, c), "{quality:?}");
        }
        assert!(!is_home_chord(PitchClass::G, Major, c));
    }

    /// The bars the chord sheet shoots in, numbered from 1.
    fn shot_bars(project: &Project) -> Vec<usize> {
        let meter = project.meter();
        project
            .playlist()
            .chord_markers()
            .iter()
            .filter(|marker| marker.stop == Some(Stop::Shot))
            .map(|marker| meter.bar_at(marker.start).number + 1)
            .collect()
    }

    #[test]
    fn candombe_para_gardel_breaks_where_it_resolves_home() {
        let song = SONGS
            .iter()
            .find(|song| song.name == "Candombe para Gardel")
            .unwrap();
        let mut project = song_demo(song);
        let shots = shot_bars(&project);
        for bar in [14, 22, 53] {
            assert!(shots.contains(&bar), "bar {bar}: {shots:?}");
        }
        // The ear finds the same breaks on the chord sheet without them,
        // and marks them again.
        let marked = project.playlist().chord_markers().to_vec();
        let meter = project.meter();
        let heard = [14, 22, 53].map(|bar| meter.bar(bar - 1).start);
        for marker in &mut project.playlist.chord_markers {
            if heard.contains(&marker.start) {
                marker.stop = None;
            }
        }
        assert_eq!(project.heard_breaks(), heard);
        assert_eq!(project.mark_heard_breaks(), 3);
        assert_eq!(project.playlist().chord_markers(), marked);
    }
}
