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
//!   like stretching a pattern clip in FL Studio;
//! - [`Section`]s name stretches of the song (Intro, Verse, Chorus...) and
//!   can be played more than once, so a chorus written once is heard twice;
//! - a [`LoopRegion`] makes song mode loop one part of the song while you
//!   work on it;
//! - [`TimeMarker`]s change the time signature partway through the song.
//!
//! [`crate::arrangement`] turns all of this into the list of things the
//! audio thread plays.

use serde::{Deserialize, Serialize};

use crate::meter::TimeMarker;
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
}

impl Track {
    /// A track named "Track 1", "Track 2" and so on.
    pub fn numbered(number: usize) -> Self {
        Track {
            name: format!("Track {number}"),
            muted: false,
        }
    }
}

/// One use of a pattern in the song.
///
/// The clip does not store its length: it is as long as its pattern
/// times `repeats`, so making a pattern longer makes all its clips longer
/// too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Clip {
    /// The pattern's position in the project's pattern list.
    pub pattern: usize,
    /// The track's position in the playlist, counted from 0 at the top.
    pub track: usize,
    /// The step the clip starts on, counted from the start of the song.
    pub start: usize,
    /// How many times the pattern plays, one right after the other:
    /// 1 to [`MAX_REPEATS`].
    #[serde(default = "once", skip_serializing_if = "is_once")]
    pub repeats: usize,
}

impl Clip {
    /// A clip that plays its pattern once.
    pub fn new(pattern: usize, track: usize, start: usize) -> Self {
        Clip {
            pattern,
            track,
            start,
            repeats: 1,
        }
    }

    /// The same clip, playing its pattern `repeats` times.
    pub fn repeated(self, repeats: usize) -> Self {
        Clip { repeats, ..self }
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
