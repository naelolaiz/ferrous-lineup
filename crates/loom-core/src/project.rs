//! The project: channels, patterns, the playlist, the tempo and the key,
//! kept in sync.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::harmony::Scale;
use crate::pattern::{NoteEvent, Pattern};
use crate::playlist::{Clip, MAX_SONG_STEPS, MAX_TRACKS, Playlist, Track, round_up_to_bar};
use crate::{Note, Tempo};

/// The most channels a project can have. The audio thread reserves room
/// for this many up front so that adding a channel never allocates.
pub const MAX_CHANNELS: usize = 32;

/// The most patterns a project can have. The audio thread reserves room
/// for this many too.
pub const MAX_PATTERNS: usize = 99;

/// How long a new pattern is, in steps: one bar.
const NEW_PATTERN_LENGTH: usize = 16;

/// The built-in drum sounds, synthesized by the audio engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Drum {
    Kick,
    Snare,
    Clap,
    HiHat,
}

impl Drum {
    /// All drums, in the order they appear in menus.
    pub const ALL: [Drum; 4] = [Drum::Kick, Drum::Snare, Drum::Clap, Drum::HiHat];

    /// A short display name.
    pub fn name(self) -> &'static str {
        match self {
            Drum::Kick => "Kick",
            Drum::Snare => "Snare",
            Drum::Clap => "Clap",
            Drum::HiHat => "Hat",
        }
    }
}

/// Where a sampler channel gets its sound from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SampleSource {
    /// One of the drums built into Ferrous Loom.
    Builtin(Drum),
    /// A WAV file on disk.
    File(PathBuf),
}

/// What kind of instrument a channel is.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelKind {
    /// Plays a recorded or built-in sound from start to end for each
    /// note, faster for higher notes and slower for lower ones.
    Sampler(SampleSource),
    /// The built-in "Loom Osc" synthesizer, which can play chords.
    Synth,
}

/// One row of the channel rack.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub kind: ChannelKind,
    /// From 0.0 (silent) to 1.0 (full).
    pub volume: f32,
    pub muted: bool,
}

impl Channel {
    /// A sampler channel playing a built-in drum, named after it.
    pub fn drum(drum: Drum) -> Self {
        Channel {
            name: drum.name().to_string(),
            kind: ChannelKind::Sampler(SampleSource::Builtin(drum)),
            volume: 0.8,
            muted: false,
        }
    }

    /// A Loom Osc synth channel.
    pub fn synth() -> Self {
        Channel {
            name: "Loom Osc".to_string(),
            kind: ChannelKind::Synth,
            volume: 0.6,
            muted: false,
        }
    }
}

/// Everything the user is working on, and everything a project file saves.
///
/// The fields that have rules are private, so they can only change
/// together, through the methods below:
///
/// - every pattern has one row of notes per channel, in the same order;
/// - there is always at least one pattern;
/// - every clip in the playlist refers to a pattern and a track that exist.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub tempo: Tempo,
    /// The key the piano roll highlights and names chords in.
    pub scale: Scale,
    channels: Vec<Channel>,
    patterns: Vec<Pattern>,
    playlist: Playlist,
}

impl Project {
    /// An empty project: no channels, one empty pattern, an empty playlist.
    pub fn empty() -> Self {
        Project {
            tempo: Tempo::default(),
            scale: Scale::default(),
            channels: Vec::new(),
            patterns: vec![Pattern::new("Pattern 1", 0, NEW_PATTERN_LENGTH)],
            playlist: Playlist::new(),
        }
    }

    /// What File > New starts from: the drum kit and a Loom Osc, with no notes.
    pub fn template() -> Self {
        let mut project = Project::empty();
        for drum in [Drum::Kick, Drum::Clap, Drum::HiHat, Drum::Snare] {
            project.add_channel(Channel::drum(drum));
        }
        project.add_channel(Channel::synth());
        project
    }

    /// The starting song: eight bars built from three patterns.
    ///
    /// - "Beat", one bar of drums.
    /// - "Chords", two bars of the Loom Osc playing C, Am, F and G
    ///   (I, vi, IV, V in C major).
    /// - "Fill", one bar of drums with a snare roll, to end the song.
    ///
    /// The chords play alone for two bars, then the beat comes in, and
    /// the fill replaces the beat in the last bar.
    pub fn demo() -> Self {
        let mut project = Project::template();
        let (kick, clap, hat, snare, synth) = (0, 1, 2, 3, 4);

        let beat = 0;
        let pattern = &mut project.patterns[beat];
        pattern.name = "Beat".to_string();
        let drums: [(usize, &[usize]); 3] = [
            (kick, &[0, 4, 8, 12]), // four on the floor
            (clap, &[4, 12]),       // beats 2 and 4
            (hat, &[2, 6, 10, 14]), // the off-beats
        ];
        for (channel, steps) in drums {
            for &step in steps {
                pattern.set(channel, step, true);
            }
        }

        let chords = project.add_pattern().unwrap_or(beat);
        let pattern = &mut project.patterns[chords];
        pattern.name = "Chords".to_string();
        pattern.set_length(32);
        // Each chord lasts half a bar: a bass note and three chord notes,
        // chosen so that each chord moves as little as possible from the
        // last one (good "voice leading").
        let progression: [[u8; 4]; 4] = [
            [48, 60, 64, 67], // C:  C3, C4 E4 G4
            [45, 57, 60, 64], // Am: A2, A3 C4 E4
            [41, 57, 60, 65], // F:  F2, A3 C4 F4
            [43, 59, 62, 67], // G:  G2, B3 D4 G4
        ];
        for (i, chord) in progression.iter().enumerate() {
            for &midi in chord {
                pattern.add_note(synth, NoteEvent::new(Note::new(midi), i * 8, 8));
            }
        }

        let fill = project.add_pattern().unwrap_or(beat);
        let pattern = &mut project.patterns[fill];
        pattern.name = "Fill".to_string();
        let drums: [(usize, &[usize]); 4] = [
            (kick, &[0, 4, 8]),
            (clap, &[4]),
            (hat, &[2, 6]),
            (snare, &[10, 12, 13, 14, 15]),
        ];
        for (channel, steps) in drums {
            for &step in steps {
                pattern.set(channel, step, true);
            }
        }

        let (chord_track, drum_track) = (0, 1);
        if let Some(track) = project.track_mut(chord_track) {
            track.name = "Chords".to_string();
        }
        if let Some(track) = project.track_mut(drum_track) {
            track.name = "Drums".to_string();
        }
        for bar in [0, 2, 4, 6] {
            project.add_clip(Clip::new(chords, chord_track, bar * 16));
        }
        for bar in 2..7 {
            project.add_clip(Clip::new(beat, drum_track, bar * 16));
        }
        project.add_clip(Clip::new(fill, drum_track, 7 * 16));
        project
    }

    // ----- Channels -----

    pub fn channels(&self) -> &[Channel] {
        &self.channels
    }

    pub fn channel_mut(&mut self, index: usize) -> Option<&mut Channel> {
        self.channels.get_mut(index)
    }

    /// Adds a channel at the bottom of the rack, with no notes in any
    /// pattern. Returns its index, or `None` if the rack is full.
    pub fn add_channel(&mut self, channel: Channel) -> Option<usize> {
        if self.channels.len() >= MAX_CHANNELS {
            return None;
        }
        self.channels.push(channel);
        for pattern in &mut self.patterns {
            pattern.push_row();
        }
        Some(self.channels.len() - 1)
    }

    /// Removes a channel and its notes in every pattern. Returns the removed channel.
    pub fn remove_channel(&mut self, index: usize) -> Option<Channel> {
        if index >= self.channels.len() {
            return None;
        }
        for pattern in &mut self.patterns {
            pattern.remove_row(index);
        }
        Some(self.channels.remove(index))
    }

    // ----- Patterns -----

    pub fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    pub fn pattern(&self, index: usize) -> Option<&Pattern> {
        self.patterns.get(index)
    }

    /// A pattern to edit.
    ///
    /// Handing out `&mut Pattern` is safe: the only `Pattern` methods that
    /// add or remove rows are `pub(crate)`, so code outside this crate
    /// can change notes and lengths but cannot break the "one row per
    /// channel" rule.
    pub fn pattern_mut(&mut self, index: usize) -> Option<&mut Pattern> {
        self.patterns.get_mut(index)
    }

    /// Adds an empty pattern at the end of the list, named "Pattern 2",
    /// "Pattern 3" and so on. Returns its index, or `None` if there are
    /// already [`MAX_PATTERNS`].
    pub fn add_pattern(&mut self) -> Option<usize> {
        let name = format!("Pattern {}", self.patterns.len() + 1);
        self.push_pattern(Pattern::new(name, self.channels.len(), NEW_PATTERN_LENGTH))
    }

    /// Adds a copy of a pattern at the end of the list and returns its index.
    pub fn clone_pattern(&mut self, index: usize) -> Option<usize> {
        let mut copy = self.patterns.get(index)?.clone();
        copy.name = format!("{} copy", copy.name);
        self.push_pattern(copy)
    }

    fn push_pattern(&mut self, pattern: Pattern) -> Option<usize> {
        if self.patterns.len() >= MAX_PATTERNS {
            return None;
        }
        self.patterns.push(pattern);
        Some(self.patterns.len() - 1)
    }

    /// Removes a pattern and every clip that plays it. Clips of the
    /// patterns after it are renumbered, so they keep playing the same
    /// music. The last pattern cannot be removed.
    pub fn remove_pattern(&mut self, index: usize) -> Option<Pattern> {
        if index >= self.patterns.len() || self.patterns.len() == 1 {
            return None;
        }
        self.playlist.clips.retain(|clip| clip.pattern != index);
        for clip in &mut self.playlist.clips {
            if clip.pattern > index {
                clip.pattern -= 1;
            }
        }
        Some(self.patterns.remove(index))
    }

    // ----- Playlist -----

    pub fn playlist(&self) -> &Playlist {
        &self.playlist
    }

    /// A track to rename or mute. Tracks have no rules to keep, so this
    /// is a plain mutable reference.
    pub fn track_mut(&mut self, index: usize) -> Option<&mut Track> {
        self.playlist.tracks.get_mut(index)
    }

    /// Adds a track at the bottom of the playlist. Returns its index, or
    /// `None` if there are already [`MAX_TRACKS`].
    pub fn add_track(&mut self) -> Option<usize> {
        let tracks = &mut self.playlist.tracks;
        if tracks.len() >= MAX_TRACKS {
            return None;
        }
        tracks.push(Track::numbered(tracks.len() + 1));
        Some(tracks.len() - 1)
    }

    /// Replaces all the tracks, for loading a file. Only for use inside
    /// this crate, while there are no clips yet that could point past the end.
    pub(crate) fn replace_tracks(&mut self, tracks: Vec<Track>) {
        self.playlist.tracks = tracks;
    }

    /// Whether a clip could be placed in the playlist.
    fn clip_fits(&self, clip: Clip) -> bool {
        clip.pattern < self.patterns.len()
            && clip.track < self.playlist.tracks.len()
            && clip.start < MAX_SONG_STEPS
    }

    /// Adds a clip and returns its index. Returns `None` if its pattern
    /// or track does not exist, it starts too late, or the very same clip
    /// is already there.
    pub fn add_clip(&mut self, clip: Clip) -> Option<usize> {
        if !self.clip_fits(clip) || self.playlist.clips.contains(&clip) {
            return None;
        }
        self.playlist.clips.push(clip);
        Some(self.playlist.clips.len() - 1)
    }

    /// Moves a clip to another track or time. Returns `false` if there is
    /// no such clip, it would not fit there, or the very same clip is
    /// already there.
    pub fn move_clip(&mut self, index: usize, track: usize, start: usize) -> bool {
        let Some(&clip) = self.playlist.clips.get(index) else {
            return false;
        };
        let moved = Clip {
            track,
            start,
            ..clip
        };
        if !self.clip_fits(moved) || self.playlist.clips.contains(&moved) {
            return false;
        }
        self.playlist.clips[index] = moved;
        true
    }

    pub fn remove_clip(&mut self, index: usize) -> Option<Clip> {
        let clips = &mut self.playlist.clips;
        (index < clips.len()).then(|| clips.remove(index))
    }

    /// How many steps a clip lasts: the length of its pattern.
    pub fn clip_length(&self, clip: &Clip) -> usize {
        self.patterns.get(clip.pattern).map_or(0, Pattern::length)
    }

    /// The length of the song in steps: up to the end of the last clip,
    /// rounded up to a whole bar. An empty playlist has length 0.
    pub fn song_length(&self) -> usize {
        let end = self
            .playlist
            .clips
            .iter()
            .map(|clip| clip.start + self.clip_length(clip))
            .max()
            .unwrap_or(0);
        round_up_to_bar(end)
    }

    /// Where `pattern` is during `song_step` of the song, if one of its
    /// clips is playing then: the step inside the pattern. The channel
    /// rack and the piano roll use this to show the playhead in song mode.
    pub fn pattern_step_in_song(&self, pattern: usize, song_step: usize) -> Option<usize> {
        self.playlist
            .audible_clips()
            .filter(|clip| clip.pattern == pattern)
            .find(|clip| (clip.start..clip.start + self.clip_length(clip)).contains(&song_step))
            .map(|clip| song_step - clip.start)
    }
}

impl Default for Project {
    fn default() -> Self {
        Project::demo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::analyze;

    const BEAT: usize = 0;
    const CHORDS: usize = 1;
    const FILL: usize = 2;
    const SYNTH: usize = 4;

    #[test]
    fn demo_has_a_beat() {
        let project = Project::demo();
        assert_eq!(project.channels().len(), 5);
        assert_eq!(project.channels()[0].name, "Kick");
        let beat = project.pattern(BEAT).unwrap();
        assert_eq!(beat.name, "Beat");
        assert!(beat.is_on(0, 0));
        assert!(beat.is_on(1, 4));
        assert!(!beat.is_on(3, 0), "no snare in the beat");
        assert!(
            project.pattern(FILL).unwrap().is_on(3, 15),
            "the fill has a snare roll"
        );
    }

    #[test]
    fn demo_plays_a_chord_progression() {
        let project = Project::demo();
        let chords = project.pattern(CHORDS).unwrap();
        assert!(!chords.is_step_row(SYNTH));
        let names: Vec<String> = [0, 8, 16, 24]
            .into_iter()
            .map(|step| {
                analyze(chords.sounding_at(SYNTH, step))
                    .label(project.scale.spelling())
                    .unwrap()
            })
            .collect();
        assert_eq!(names, ["C", "Am", "F", "G"]);
    }

    #[test]
    fn demo_song_is_eight_bars() {
        let project = Project::demo();
        assert_eq!(project.song_length(), 8 * 16);
        assert_eq!(project.playlist().tracks()[0].name, "Chords");
        // During bar 4 (song steps 48..64) the second half of the chords
        // and the beat are both playing.
        assert_eq!(project.pattern_step_in_song(CHORDS, 50), Some(18));
        assert_eq!(project.pattern_step_in_song(BEAT, 50), Some(2));
        assert_eq!(project.pattern_step_in_song(FILL, 50), None);
        assert_eq!(
            project.pattern_step_in_song(BEAT, 0),
            None,
            "no beat in the intro"
        );
    }

    #[test]
    fn channels_and_rows_stay_in_sync() {
        let mut project = Project::demo();
        let new = project.add_channel(Channel::drum(Drum::Snare)).unwrap();
        for pattern in project.patterns() {
            assert_eq!(pattern.rows(), project.channels().len());
            assert!(pattern.notes(new).is_empty(), "new rows start empty");
        }

        // Removing the kick moves the clap's steps up to row 0, in every pattern.
        let removed = project.remove_channel(0).unwrap();
        assert_eq!(removed.name, "Kick");
        for pattern in project.patterns() {
            assert_eq!(pattern.rows(), project.channels().len());
        }
        assert_eq!(project.channels()[0].name, "Clap");
        assert!(project.pattern(BEAT).unwrap().is_on(0, 4));
        assert!(project.pattern(FILL).unwrap().is_on(0, 4));
    }

    #[test]
    fn the_rack_has_a_limit() {
        let mut project = Project::empty();
        for _ in 0..MAX_CHANNELS {
            assert!(project.add_channel(Channel::synth()).is_some());
        }
        assert_eq!(project.add_channel(Channel::synth()), None);
        assert_eq!(project.remove_channel(MAX_CHANNELS), None);
    }

    #[test]
    fn new_patterns_have_a_row_per_channel() {
        let mut project = Project::template();
        let index = project.add_pattern().unwrap();
        let pattern = project.pattern(index).unwrap();
        assert_eq!(pattern.name, "Pattern 2");
        assert_eq!(pattern.rows(), 5);
        assert!(pattern.is_empty());
    }

    #[test]
    fn cloned_patterns_are_independent() {
        let mut project = Project::demo();
        let copy = project.clone_pattern(BEAT).unwrap();
        assert_eq!(project.pattern(copy).unwrap().name, "Beat copy");
        assert!(project.pattern(copy).unwrap().is_on(0, 0));
        project.pattern_mut(copy).unwrap().set(0, 0, false);
        assert!(
            project.pattern(BEAT).unwrap().is_on(0, 0),
            "the original is unchanged"
        );
    }

    #[test]
    fn removing_a_pattern_renumbers_its_followers() {
        let mut project = Project::demo();
        let clips_before = project.playlist().clips().len();
        let removed = project.remove_pattern(BEAT).unwrap();
        assert_eq!(removed.name, "Beat");
        // The five Beat clips are gone; Chords and Fill moved up one place.
        assert_eq!(project.playlist().clips().len(), clips_before - 5);
        let chords = project.patterns().iter().position(|p| p.name == "Chords");
        assert_eq!(chords, Some(0));
        assert!(
            project
                .playlist()
                .clips()
                .iter()
                .all(|clip| clip.pattern < 2)
        );
        assert_eq!(
            project.pattern_step_in_song(1, 7 * 16),
            Some(0),
            "the fill still plays"
        );
    }

    #[test]
    fn the_last_pattern_stays() {
        let mut project = Project::empty();
        assert!(project.remove_pattern(0).is_none());
        assert_eq!(project.patterns().len(), 1);
    }

    #[test]
    fn clips_must_point_somewhere() {
        let mut project = Project::template();
        assert!(project.add_clip(Clip::new(0, 0, 0)).is_some());
        assert!(
            project.add_clip(Clip::new(0, 0, 0)).is_none(),
            "no exact duplicates"
        );
        assert!(
            project.add_clip(Clip::new(1, 0, 0)).is_none(),
            "no pattern 1"
        );
        assert!(
            project.add_clip(Clip::new(0, 99, 0)).is_none(),
            "no track 99"
        );
        assert!(project.add_clip(Clip::new(0, 0, MAX_SONG_STEPS)).is_none());
        assert!(project.move_clip(0, 3, 32));
        assert_eq!(project.playlist().clips()[0], Clip::new(0, 3, 32));
        assert!(!project.move_clip(0, 99, 0));
        assert_eq!(project.remove_clip(0), Some(Clip::new(0, 3, 32)));
        assert_eq!(project.remove_clip(0), None);
    }

    #[test]
    fn songs_end_on_a_bar_line() {
        let mut project = Project::template();
        assert_eq!(project.song_length(), 0);
        project.add_clip(Clip::new(0, 0, 4));
        // A one-bar pattern starting on step 4 ends on step 20: two bars.
        assert_eq!(project.song_length(), 32);
        project.pattern_mut(0).unwrap().set_length(64);
        assert_eq!(project.song_length(), 80, "clips grow with their pattern");
    }

    #[test]
    fn muted_tracks_hide_the_playhead() {
        let mut project = Project::demo();
        project.track_mut(1).unwrap().muted = true;
        assert_eq!(project.pattern_step_in_song(BEAT, 50), None);
    }
}
