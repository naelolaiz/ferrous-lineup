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

use serde::{Deserialize, Serialize};

/// Steps in one bar of 4/4 time: four beats of four steps.
pub const STEPS_PER_BAR: usize = 16;

/// How many tracks a new playlist has.
pub const DEFAULT_TRACKS: usize = 16;

/// The most tracks a playlist can have.
pub const MAX_TRACKS: usize = 64;

/// Clips must start before this step: 512 bars, about 16 minutes at 130 BPM.
pub const MAX_SONG_STEPS: usize = 512 * STEPS_PER_BAR;

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
/// The clip does not store its length: it is as long as its pattern, so
/// making a pattern longer makes all its clips longer too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Clip {
    /// The pattern's position in the project's pattern list.
    pub pattern: usize,
    /// The track's position in the playlist, counted from 0 at the top.
    pub track: usize,
    /// The step the clip starts on, counted from the start of the song.
    pub start: usize,
}

impl Clip {
    pub fn new(pattern: usize, track: usize, start: usize) -> Self {
        Clip {
            pattern,
            track,
            start,
        }
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
}

impl Playlist {
    /// An empty playlist with [`DEFAULT_TRACKS`] tracks.
    pub fn new() -> Self {
        Playlist {
            tracks: (1..=DEFAULT_TRACKS).map(Track::numbered).collect(),
            clips: Vec::new(),
        }
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// The clips, in the order they were added.
    pub fn clips(&self) -> &[Clip] {
        &self.clips
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

/// Rounds a number of steps up to a whole number of bars.
pub fn round_up_to_bar(steps: usize) -> usize {
    steps.div_ceil(STEPS_PER_BAR) * STEPS_PER_BAR
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
    fn rounding_to_bars() {
        assert_eq!(round_up_to_bar(0), 0);
        assert_eq!(round_up_to_bar(1), 16);
        assert_eq!(round_up_to_bar(16), 16);
        assert_eq!(round_up_to_bar(17), 32);
    }
}
