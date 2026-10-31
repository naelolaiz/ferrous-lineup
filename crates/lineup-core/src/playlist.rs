//! The playlist: patterns arranged in time to make a song.
//!
//! The playlist is a grid of tracks (rows) and bars (columns). A [`Clip`]
//! says "play this pattern on this track, starting at this step". The same
//! pattern can be used by many clips: change the pattern, and every place
//! it is used changes with it.
//!
//! Tracks are not channels. A track is just a lane to put clips on, so
//! you can keep the drums on one track and the chords on another. Which
//! instruments play is decided by the notes inside each pattern.
//!
//! On top of the clips, the playlist holds the song's structure:
//!
//! - a clip can repeat its pattern several times in a row ([`Clip::repeats`]),
//!   like stretching a pattern clip in a pattern-based studio;
//! - [`Section`]s name stretches of the song (Intro, Verse, Chorus...) and
//!   can be played more than once, so a chorus written once is heard twice;
//! - a [`LoopRegion`] makes song mode loop one part of the song while you
//!   work on it;
//! - [`TimeMarker`]s change the time signature partway through the song;
//! - [`ChordMarker`]s write the song's harmony, bar by bar, as the chord
//!   sheet shows it.
//!
//! [`crate::arrangement`] turns all of this into the list of things the
//! audio thread plays.

use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::chord_sheet::ChordMarker;
use crate::key_map::KeyMarker;
use crate::meter::TimeMarker;
use crate::pattern::NoteEvent;
use crate::tempo::TempoMarker;

/// Steps in one bar of 4/4 time: four beats of four steps.
pub const STEPS_PER_BAR: usize = 16;

/// How many tracks a new playlist has.
pub const DEFAULT_TRACKS: usize = 16;

/// The most tracks a playlist can have.
pub const MAX_TRACKS: usize = 64;

/// Clips must start before this step: 512 bars, about 16 minutes at 130 BPM.
pub const MAX_SONG_STEPS: usize = 512 * STEPS_PER_BAR;

/// The most times a clip or a section can play in a row.
pub const MAX_REPEATS: usize = 64;

/// The names offered for sections. New sections take the first one not
/// used yet, so the most common come first: a song often goes Intro,
/// Verse, Chorus, then a Bridge and an Outro.
pub const SECTION_NAMES: [&str; 9] = [
    "Intro",
    "Verse",
    "Chorus",
    "Bridge",
    "Outro",
    "Pre-Chorus",
    "Solo",
    "Break",
    "Drop",
];

/// Used by serde for fields that are left out of a file: a clip or a
/// section saved without `repeats` plays once.
fn once() -> usize {
    1
}

/// Lets serde leave `repeats: 1` out of files, so they read as before.
fn is_once(repeats: &usize) -> bool {
    *repeats == 1
}

/// One row of the playlist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    /// A muted track's clips are not played.
    pub muted: bool,
    /// Preferred row height at 100% zoom. Older versions may ignore this
    /// display preference without changing the music.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
}

impl Track {
    /// A track named "Track 1", "Track 2" and so on.
    pub fn numbered(number: usize) -> Self {
        Track {
            name: format!("Track {number}"),
            muted: false,
            height: None,
        }
    }

    pub fn height(&self) -> f32 {
        f32::from(self.height.unwrap_or(36).clamp(24, 240))
    }
}

/// One use of a pattern in the song.
///
/// The clip does not store its length: it is as long as its pattern
/// times `repeats`, so making a pattern longer makes all its clips longer
/// too.
///
/// A clip can also be trimmed, as in most audio workstations: dragging
/// its left edge in hides the start of the pattern (`skip`), and
/// dragging its right edge in hides the end of its last time round
/// (`cut`). The hidden notes are not played, but they are still in the
/// pattern: dragging the edge back out brings them back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Clip {
    /// The pattern's position in the project's pattern list.
    pub pattern: usize,
    /// The track's position in the playlist, counted from 0 at the top.
    pub track: usize,
    /// The first step the clip plays, counted from the start of the song.
    pub start: usize,
    /// How many times round the pattern the clip plays, one right after
    /// the other: 1 to [`MAX_REPEATS`]. A trimmed time round counts.
    #[serde(default = "once", skip_serializing_if = "is_once")]
    pub repeats: usize,
    /// How many steps at the start of the pattern are not played the
    /// first time round: less than the pattern is long.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub skip: usize,
    /// How many steps at the end of the pattern are not played the last
    /// time round: less than the pattern is long.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub cut: usize,
}

/// Lets serde leave `skip: 0` and `cut: 0` out of files, so a clip that
/// is not trimmed reads as before.
fn is_zero(steps: &usize) -> bool {
    *steps == 0
}

/// One time a clip plays its pattern: the steps of the pattern heard
/// that time, and the step of the song the first of them is heard on.
///
/// Only the first and last time round can leave steps out, when the
/// clip is trimmed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Round {
    /// The song step `steps.start` is heard on.
    pub at: usize,
    /// The steps of the pattern heard.
    pub steps: Range<usize>,
}

impl Round {
    /// The song step on which step `step` of the pattern is heard this
    /// time round. `step` may be outside `steps`: a note held past the
    /// end of the round ends where the round ends.
    pub fn song_step(&self, step: usize) -> usize {
        (self.at + step).saturating_sub(self.steps.start)
    }

    /// Whether a note starting on step `start` of the pattern is played
    /// this time round.
    pub fn plays(&self, start: usize) -> bool {
        self.steps.contains(&start)
    }

    /// The song step just after the last one heard this time round.
    pub fn end(&self) -> usize {
        self.at + self.steps.len()
    }

    /// `note`, a note of the pattern, as heard this time round: on the
    /// song's timeline, and ending where the round ends at the latest.
    /// `None` if it does not start in the steps heard.
    pub fn place(&self, note: &NoteEvent) -> Option<NoteEvent> {
        if !self.plays(note.start) {
            return None;
        }
        let room = self.steps.end - note.start;
        Some(match note.length > room {
            true => NoteEvent {
                start: self.song_step(note.start),
                length: room,
                trim: 0,
                ..*note
            },
            false => NoteEvent {
                start: self.song_step(note.start),
                ..*note
            },
        })
    }
}

impl Clip {
    /// A clip that plays its pattern once.
    pub fn new(pattern: usize, track: usize, start: usize) -> Self {
        Clip {
            pattern,
            track,
            start,
            repeats: 1,
            skip: 0,
            cut: 0,
        }
    }

    /// The same clip, playing its pattern `repeats` times.
    pub fn repeated(self, repeats: usize) -> Self {
        Clip { repeats, ..self }
    }

    /// Whether part of its pattern is left out at either end.
    pub fn is_trimmed(&self) -> bool {
        self.skip > 0 || self.cut > 0
    }

    /// The steps left out at the start and at the end, for a pattern
    /// `pattern_length` steps long. A pattern made shorter since the clip
    /// was trimmed leaves at least one step to play.
    fn trims(&self, pattern_length: usize) -> (usize, usize) {
        let whole = pattern_length * self.repeats;
        let skip = self.skip.min(pattern_length.saturating_sub(1));
        let cut = self
            .cut
            .min(pattern_length.saturating_sub(1))
            .min(whole.saturating_sub(skip + 1));
        (skip, cut)
    }

    /// The step of its pattern the clip starts on: 0 unless it is
    /// trimmed at the start.
    pub fn first_step(&self, pattern_length: usize) -> usize {
        self.trims(pattern_length).0
    }

    /// The song step its pattern's first step falls on the first time
    /// round, heard or not: `start`, unless the clip is trimmed at the
    /// start. Never before the start of the song.
    pub fn origin(&self, pattern_length: usize) -> usize {
        self.start.saturating_sub(self.first_step(pattern_length))
    }

    /// How many steps the clip plays, for a pattern `pattern_length`
    /// steps long: its time rounds, without the steps trimmed.
    pub fn length(&self, pattern_length: usize) -> usize {
        let (skip, cut) = self.trims(pattern_length);
        (pattern_length * self.repeats).saturating_sub(skip + cut)
    }

    /// The step just after the last one the clip plays.
    pub fn end(&self, pattern_length: usize) -> usize {
        self.start + self.length(pattern_length)
    }

    /// Each time the clip plays its pattern, in order.
    pub fn rounds(&self, pattern_length: usize) -> impl Iterator<Item = Round> + '_ {
        let (skip, cut) = self.trims(pattern_length);
        let repeats = if pattern_length == 0 { 0 } else { self.repeats };
        (0..repeats).map(move |round| {
            let first = if round == 0 { skip } else { 0 };
            let last = if round + 1 == repeats {
                pattern_length - cut
            } else {
                pattern_length
            };
            Round {
                // Round 0 is heard from `start`, the others a whole
                // pattern later each, less the steps skipped.
                at: self.start + round * pattern_length - (skip - first),
                steps: first..last,
            }
        })
    }

    /// The same clip, its left edge moved to song step `step`: steps of
    /// the pattern are hidden, or come back, so that every note heard
    /// stays where it was. Past the start of the pattern, the clip plays
    /// the end of it once more before. `None` if the clip would play
    /// nothing, start before the song or repeat too often.
    pub fn trimmed_from(&self, step: usize, pattern_length: usize) -> Option<Clip> {
        if pattern_length == 0 || step >= self.end(pattern_length) {
            return None;
        }
        let (skip, cut) = self.trims(pattern_length);
        // Where the pattern starts its first time round, counted as if a
        // round before could stretch back that far.
        let origin = self.start as isize - skip as isize;
        let from = step as isize - origin;
        // Rounds gained before the first one, or dropped from the start.
        let rounds = from.div_euclid(pattern_length as isize);
        let skip = from.rem_euclid(pattern_length as isize) as usize;
        let repeats = self.repeats as isize - rounds;
        if !(1..=MAX_REPEATS as isize).contains(&repeats) {
            return None;
        }
        Some(Clip {
            start: step,
            repeats: repeats as usize,
            skip,
            cut,
            ..*self
        })
    }

    /// The same clip, its right edge moved to song step `step`: the end
    /// of the last time round is hidden, or comes back, and past the end
    /// of the pattern the clip plays it again. `None` if the clip would
    /// play nothing or repeat too often.
    pub fn trimmed_to(&self, step: usize, pattern_length: usize) -> Option<Clip> {
        if pattern_length == 0 || step <= self.start {
            return None;
        }
        let (skip, _) = self.trims(pattern_length);
        // How far the clip reaches into its pattern, counting its rounds.
        let reach = skip + (step - self.start);
        let repeats = reach.div_ceil(pattern_length);
        if repeats > MAX_REPEATS {
            return None;
        }
        Some(Clip {
            repeats,
            skip,
            cut: repeats * pattern_length - reach,
            ..*self
        })
    }

    /// Whether two clips sit in the same place: same pattern, track and
    /// start. Two such clips would be drawn one on top of the other.
    pub fn same_place(&self, other: &Clip) -> bool {
        (self.pattern, self.track, self.start) == (other.pattern, other.track, other.start)
    }
}

/// A named stretch of the song, such as "Verse" or "Chorus", shown above
/// the bar numbers.
///
/// A section does not own clips: it covers whatever clips are under it.
/// Its `repeats` say how many times that stretch of the song is played
/// before the song moves on. Sections never overlap.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    /// The first step it covers.
    pub start: usize,
    /// How many steps it covers.
    pub length: usize,
    /// How many times it is played: 1 to [`MAX_REPEATS`].
    #[serde(default = "once", skip_serializing_if = "is_once")]
    pub repeats: usize,
}

impl Section {
    /// A section that is played once.
    pub fn new(name: impl Into<String>, start: usize, length: usize) -> Self {
        Section {
            name: name.into(),
            start,
            length,
            repeats: 1,
        }
    }

    /// The step just after its last one.
    pub fn end(&self) -> usize {
        self.start + self.length
    }

    /// Whether it covers `step`.
    pub fn contains(&self, step: usize) -> bool {
        (self.start..self.end()).contains(&step)
    }

    /// Whether it shares at least one step with `other`.
    pub fn overlaps(&self, other: &Section) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

/// The part of the song that song mode loops, from `start` up to (not
/// including) `end`, in steps. Without one, the whole song loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoopRegion {
    pub start: usize,
    pub end: usize,
}

impl LoopRegion {
    pub fn new(start: usize, end: usize) -> Self {
        LoopRegion { start, end }
    }

    pub fn length(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

/// The tracks and the clips on them.
///
/// Clips refer to patterns by number, so they must change when patterns
/// are removed. The project does that, which is why the fields can only
/// be changed from inside this crate.
#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    pub(crate) tracks: Vec<Track>,
    pub(crate) clips: Vec<Clip>,
    /// Sorted by start, and never overlapping.
    pub(crate) sections: Vec<Section>,
    pub(crate) loop_region: Option<LoopRegion>,
    /// Sorted by start, all after step 0, no two on the same step.
    pub(crate) time_markers: Vec<TimeMarker>,
    /// Sorted by start, all after step 0, no two on the same step.
    pub(crate) tempo_markers: Vec<TempoMarker>,
    /// Sorted by start, all after step 0, no two on the same step.
    pub(crate) key_markers: Vec<KeyMarker>,
    /// Sorted by start, no two on the same step. Unlike the other
    /// markers, one can be on step 0: the song's first chord.
    pub(crate) chord_markers: Vec<ChordMarker>,
}

impl Playlist {
    /// An empty playlist with [`DEFAULT_TRACKS`] tracks.
    pub fn new() -> Self {
        Playlist {
            tracks: (1..=DEFAULT_TRACKS).map(Track::numbered).collect(),
            clips: Vec::new(),
            sections: Vec::new(),
            loop_region: None,
            time_markers: Vec::new(),
            tempo_markers: Vec::new(),
            key_markers: Vec::new(),
            chord_markers: Vec::new(),
        }
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// The clips, in the order they were added.
    pub fn clips(&self) -> &[Clip] {
        &self.clips
    }

    /// The sections, from the start of the song to the end.
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    /// The section that covers `step`, if any, and its index.
    pub fn section_at(&self, step: usize) -> Option<(usize, &Section)> {
        self.sections
            .iter()
            .enumerate()
            .find(|(_, section)| section.contains(step))
    }

    /// The part of the song that song mode loops, if one is set.
    pub fn loop_region(&self) -> Option<LoopRegion> {
        self.loop_region
    }

    /// The changes of time signature after the start of the song, in
    /// order. The song starts in the project's own time signature.
    pub fn time_markers(&self) -> &[TimeMarker] {
        &self.time_markers
    }

    /// The changes of tempo, in order.
    pub fn tempo_markers(&self) -> &[TempoMarker] {
        &self.tempo_markers
    }

    /// The changes of key after the start of the song, in order. The
    /// song starts in the project's own key.
    pub fn key_markers(&self) -> &[KeyMarker] {
        &self.key_markers
    }

    /// The harmonies of the chord sheet, in order. Each lasts until the
    /// next; before the first, no harmony is written.
    pub fn chord_markers(&self) -> &[ChordMarker] {
        &self.chord_markers
    }

    /// A name for a new section: the first of [`SECTION_NAMES`] not used
    /// yet, or "Section 10" and so on once they all are.
    pub fn suggested_section_name(&self) -> String {
        let used = |name: &str| self.sections.iter().any(|section| section.name == name);
        SECTION_NAMES.iter().find(|name| !used(name)).map_or_else(
            || format!("Section {}", self.sections.len() + 1),
            |name| name.to_string(),
        )
    }

    /// The clips that should be heard: those on tracks that are not muted.
    pub fn audible_clips(&self) -> impl Iterator<Item = Clip> + '_ {
        self.clips.iter().copied().filter(|clip| {
            self.tracks
                .get(clip.track)
                .is_some_and(|track| !track.muted)
        })
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_playlists_have_numbered_tracks() {
        let playlist = Playlist::new();
        assert_eq!(playlist.tracks().len(), DEFAULT_TRACKS);
        assert_eq!(playlist.tracks()[0].name, "Track 1");
        assert_eq!(playlist.tracks()[15].name, "Track 16");
        assert!(playlist.clips().is_empty());
    }

    #[test]
    fn muted_tracks_are_not_heard() {
        let mut playlist = Playlist::new();
        playlist.clips = vec![Clip::new(0, 0, 0), Clip::new(0, 1, 0)];
        playlist.tracks[1].muted = true;
        let heard: Vec<Clip> = playlist.audible_clips().collect();
        assert_eq!(heard, vec![Clip::new(0, 0, 0)]);
    }

    /// The (song step, pattern steps) of each time round.
    fn rounds(clip: &Clip, length: usize) -> Vec<(usize, Range<usize>)> {
        clip.rounds(length)
            .map(|round| (round.at, round.steps))
            .collect()
    }

    #[test]
    fn a_clip_plays_its_pattern_whole_each_time_round() {
        let clip = Clip::new(0, 0, 32).repeated(3);
        assert_eq!(clip.length(16), 48);
        assert_eq!(clip.end(16), 80);
        assert_eq!(rounds(&clip, 16), [(32, 0..16), (48, 0..16), (64, 0..16)]);
    }

    #[test]
    fn a_trimmed_clip_leaves_out_both_ends() {
        // A 16-step pattern three times from step 32, its first 4 steps
        // and its last 2 left out.
        let clip = Clip {
            start: 36,
            skip: 4,
            cut: 2,
            ..Clip::new(0, 0, 0).repeated(3)
        };
        assert!(clip.is_trimmed());
        assert_eq!(clip.length(16), 42);
        assert_eq!(clip.origin(16), 32);
        assert_eq!(rounds(&clip, 16), [(36, 4..16), (48, 0..16), (64, 0..14)]);
        // A note held past the end of the last round ends there.
        let last = clip.rounds(16).last().unwrap();
        let held = last.place(&NoteEvent::new(crate::Note::C4, 12, 4)).unwrap();
        assert_eq!((held.start, held.length), (76, 2));
        assert_eq!(last.place(&NoteEvent::new(crate::Note::C4, 14, 1)), None);
        let first = clip.rounds(16).next().unwrap();
        assert_eq!(first.place(&NoteEvent::new(crate::Note::C4, 2, 4)), None);
    }

    #[test]
    fn trimming_an_edge_keeps_every_note_where_it_was() {
        let clip = Clip::new(0, 0, 32);
        // The left edge in by 4 steps, then out past the start of the
        // pattern: the end of the pattern plays once more before.
        let from = clip.trimmed_from(36, 16).unwrap();
        assert_eq!(
            (from.start, from.skip, from.repeats, from.cut),
            (36, 4, 1, 0)
        );
        let before = clip.trimmed_from(28, 16).unwrap();
        assert_eq!(rounds(&before, 16), [(28, 12..16), (32, 0..16)]);
        // The right edge in by 2 steps, then out past the end: the
        // pattern plays again.
        let to = clip.trimmed_to(46, 16).unwrap();
        assert_eq!((to.start, to.repeats, to.cut), (32, 1, 2));
        let longer = clip.trimmed_to(56, 16).unwrap();
        assert_eq!(rounds(&longer, 16), [(32, 0..16), (48, 0..8)]);
        // Back where they were: nothing left out.
        assert_eq!(from.trimmed_from(32, 16), Some(clip));
        assert_eq!(longer.trimmed_to(48, 16), Some(clip));
        // Never empty, never before the song, never too many repeats.
        assert_eq!(clip.trimmed_from(48, 16), None);
        assert_eq!(clip.trimmed_to(32, 16), None);
        assert_eq!(clip.trimmed_to(32 + 16 * MAX_REPEATS + 1, 16), None);
    }

    #[test]
    fn a_pattern_made_shorter_still_leaves_a_trimmed_clip_a_step() {
        let clip = Clip {
            skip: 12,
            cut: 2,
            ..Clip::new(0, 0, 12)
        };
        assert_eq!(clip.length(16), 2);
        assert_eq!(clip.length(8), 1);
        assert_eq!(rounds(&clip, 8), [(12, 7..8)]);
    }

    #[test]
    fn sections_know_their_steps() {
        let verse = Section::new("Verse", 16, 32);
        assert_eq!(verse.end(), 48);
        assert!(verse.contains(16) && verse.contains(47));
        assert!(!verse.contains(48));
        assert!(verse.overlaps(&Section::new("Chorus", 40, 16)));
        assert!(!verse.overlaps(&Section::new("Chorus", 48, 16)));
    }

    #[test]
    fn new_sections_get_a_name_not_used_yet() {
        let mut playlist = Playlist::new();
        assert_eq!(playlist.suggested_section_name(), "Intro");
        playlist.sections.push(Section::new("Intro", 0, 16));
        assert_eq!(playlist.suggested_section_name(), "Verse");
        playlist.sections = SECTION_NAMES
            .iter()
            .enumerate()
            .map(|(i, name)| Section::new(*name, i * 16, 16))
            .collect();
        assert_eq!(playlist.suggested_section_name(), "Section 10");
    }
}
