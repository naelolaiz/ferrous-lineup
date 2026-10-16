//! The project: channels, patterns, the playlist, the tempo and the key,
//! kept in sync.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::arrangement::{Arrangement, arrange};
use crate::harmony::Scale;
use crate::meter::TimeSignature;
use crate::pattern::{NoteEvent, Pattern};
use crate::playlist::{
    Clip, LoopRegion, MAX_REPEATS, MAX_SONG_STEPS, MAX_TRACKS, Playlist, Section, Track,
};
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
///
/// The first four came with step 2. The others came with the rhythm
/// library of step 8, which needs them for rock fills, jazz, and Latin
/// grooves; a project that uses them is saved as format 3 (see
/// [`format_for`](crate::file::format_for)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Drum {
    Kick,
    Snare,
    Clap,
    HiHat,
    OpenHat,
    Rim,
    LowTom,
    HighTom,
    Ride,
    Crash,
    Cowbell,
    Shaker,
    Conga,
}

impl Drum {
    /// All drums, in the order they appear in menus.
    pub const ALL: [Drum; 13] = [
        Drum::Kick,
        Drum::Snare,
        Drum::Clap,
        Drum::HiHat,
        Drum::OpenHat,
        Drum::Rim,
        Drum::LowTom,
        Drum::HighTom,
        Drum::Ride,
        Drum::Crash,
        Drum::Cowbell,
        Drum::Shaker,
        Drum::Conga,
    ];

    /// A short display name.
    pub fn name(self) -> &'static str {
        match self {
            Drum::Kick => "Kick",
            Drum::Snare => "Snare",
            Drum::Clap => "Clap",
            Drum::HiHat => "Hat",
            Drum::OpenHat => "Open hat",
            Drum::Rim => "Rim",
            Drum::LowTom => "Low tom",
            Drum::HighTom => "High tom",
            Drum::Ride => "Ride",
            Drum::Crash => "Crash",
            Drum::Cowbell => "Cowbell",
            Drum::Shaker => "Shaker",
            Drum::Conga => "Conga",
        }
    }

    /// Whether this drum was added in step 8, so a project using it
    /// cannot be opened by older versions of Ferrous Loom.
    pub fn is_new_in_format_3(self) -> bool {
        !matches!(self, Drum::Kick | Drum::Snare | Drum::Clap | Drum::HiHat)
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
    /// Makes no sound itself: sends its notes out through MIDI, to an
    /// external synth or another program, on MIDI channel `channel`
    /// (0 to 15, shown as 1 to 16).
    MidiOut { channel: u8 },
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

    /// A channel that sends its notes to the MIDI output, on MIDI channel
    /// `channel` (counted from 0).
    pub fn midi_out(channel: u8) -> Self {
        let channel = channel.min(15);
        Channel {
            name: format!("MIDI Out {}", channel + 1),
            kind: ChannelKind::MidiOut { channel },
            volume: 0.8,
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
/// - every clip in the playlist refers to a pattern and a track that exist;
/// - sections are sorted and never overlap, and repeats stay in range.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub tempo: Tempo,
    /// The key the piano roll highlights and names chords in.
    pub scale: Scale,
    /// How many beats make a bar. It only changes where bar lines are
    /// drawn and how long new patterns are, never the notes.
    pub time_signature: TimeSignature,
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
            time_signature: TimeSignature::COMMON,
            channels: Vec::new(),
            patterns: vec![Pattern::new("Pattern 1", 0, NEW_PATTERN_LENGTH)],
            playlist: Playlist::new(),
        }
    }

    /// What File > New starts from: the drum kit and a Loom Osc, with no
    /// notes, and one playlist track. Placing a clip on the empty row
    /// below the last track adds another.
    pub fn template() -> Self {
        let mut project = Project::kit();
        project.remove_unused_tracks();
        project
    }

    /// An empty project with the drum kit and a Loom Osc in the rack.
    fn kit() -> Self {
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
        let mut project = Project::kit();
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
        project.remove_unused_tracks();
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
    /// "Pattern 3" and so on, one bar long (16 steps in 4/4, 12 in 3/4).
    /// Returns its index, or `None` if there are already [`MAX_PATTERNS`].
    pub fn add_pattern(&mut self) -> Option<usize> {
        let name = format!("Pattern {}", self.patterns.len() + 1);
        let length = self.time_signature.steps_per_bar();
        self.push_pattern(Pattern::new(name, self.channels.len(), length))
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

    /// Inserts a new, empty track at `index` (at the bottom if `index` is
    /// past the end), moving the tracks from there down one row with
    /// their clips. Returns its index, or `None` if there are already
    /// [`MAX_TRACKS`].
    pub fn insert_track(&mut self, index: usize) -> Option<usize> {
        let tracks = &mut self.playlist.tracks;
        if tracks.len() >= MAX_TRACKS {
            return None;
        }
        let index = index.min(tracks.len());
        tracks.insert(index, Track::numbered(tracks.len() + 1));
        for clip in &mut self.playlist.clips {
            if clip.track >= index {
                clip.track += 1;
            }
        }
        Some(index)
    }

    /// Deletes track `index` and its clips; the tracks below move up one
    /// row. The last track cannot be deleted: returns `false` then, or if
    /// there is no such track.
    pub fn remove_track(&mut self, index: usize) -> bool {
        let tracks = &mut self.playlist.tracks;
        if index >= tracks.len() || tracks.len() == 1 {
            return false;
        }
        tracks.remove(index);
        self.playlist.clips.retain(|clip| clip.track != index);
        for clip in &mut self.playlist.clips {
            if clip.track > index {
                clip.track -= 1;
            }
        }
        true
    }

    /// Swaps track `index` with the one below it, clips and all. Returns
    /// `false` if either track does not exist.
    pub fn swap_tracks(&mut self, index: usize) -> bool {
        let below = index + 1;
        if below >= self.playlist.tracks.len() {
            return false;
        }
        self.playlist.tracks.swap(index, below);
        for clip in &mut self.playlist.clips {
            if clip.track == index {
                clip.track = below;
            } else if clip.track == below {
                clip.track = index;
            }
        }
        true
    }

    /// Replaces all the tracks, for loading a file. Only for use inside
    /// this crate, while there are no clips yet that could point past the end.
    pub(crate) fn replace_tracks(&mut self, tracks: Vec<Track>) {
        self.playlist.tracks = tracks;
    }

    /// Drops the empty tracks at the bottom of the playlist, keeping at
    /// least one. Projects are built on a playlist of [`crate::playlist::DEFAULT_TRACKS`]
    /// rows; this leaves only the ones the song uses, so a demo with a
    /// melody and drums opens with two tracks.
    pub fn remove_unused_tracks(&mut self) {
        self.trim_tracks(|_, _| true);
    }

    /// Like [`Project::remove_unused_tracks`], but only drops tracks that
    /// are as a new track starts: called "Track N" for their row, and not
    /// muted. Files saved before tracks were trimmed carry sixteen of
    /// those; a track someone renamed or muted stays.
    pub(crate) fn remove_blank_tracks(&mut self) {
        self.trim_tracks(|index, track| *track == Track::numbered(index + 1));
    }

    fn trim_tracks(&mut self, drop: impl Fn(usize, &Track) -> bool) {
        let tracks = &mut self.playlist.tracks;
        let used = self
            .playlist
            .clips
            .iter()
            .map(|clip| clip.track + 1)
            .max()
            .unwrap_or(0)
            .max(1);
        while tracks.len() > used {
            let last = tracks.len() - 1;
            if !drop(last, &tracks[last]) {
                break;
            }
            tracks.pop();
        }
    }

    /// Adds a track if `track` is the row just below the last one, which
    /// [`Project::clip_fits`] allows so a clip can start a new track.
    fn open_track(&mut self, track: usize) {
        let tracks = &mut self.playlist.tracks;
        if track == tracks.len() {
            tracks.push(Track::numbered(track + 1));
        }
    }

    /// Whether a clip could be placed in the playlist: on a track, or on
    /// the row just below the last track, if there is room for one more.
    fn clip_fits(&self, clip: Clip) -> bool {
        let tracks = self.playlist.tracks.len();
        clip.pattern < self.patterns.len()
            && (clip.track < tracks || (clip.track == tracks && tracks < MAX_TRACKS))
            && clip.start < MAX_SONG_STEPS
            && (1..=MAX_REPEATS).contains(&clip.repeats)
    }

    /// Whether a clip other than number `except` sits in the same place.
    fn clip_taken(&self, clip: &Clip, except: Option<usize>) -> bool {
        self.playlist
            .clips
            .iter()
            .enumerate()
            .any(|(index, other)| Some(index) != except && other.same_place(clip))
    }

    /// Adds a clip and returns its index. Returns `None` if its pattern
    /// or track does not exist, it starts too late, it repeats too often,
    /// or a clip of the same pattern already starts there.
    pub fn add_clip(&mut self, clip: Clip) -> Option<usize> {
        if !self.clip_fits(clip) || self.clip_taken(&clip, None) {
            return None;
        }
        self.open_track(clip.track);
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
        if !self.clip_fits(moved) || self.clip_taken(&moved, Some(index)) {
            return false;
        }
        self.open_track(track);
        self.playlist.clips[index] = moved;
        true
    }

    /// Makes a clip play its pattern `repeats` times in a row. Returns
    /// `false` if there is no such clip or `repeats` is not 1 to
    /// [`MAX_REPEATS`].
    pub fn set_clip_repeats(&mut self, index: usize, repeats: usize) -> bool {
        match self.playlist.clips.get_mut(index) {
            Some(clip) if (1..=MAX_REPEATS).contains(&repeats) => {
                clip.repeats = repeats;
                true
            }
            _ => false,
        }
    }

    pub fn remove_clip(&mut self, index: usize) -> Option<Clip> {
        let clips = &mut self.playlist.clips;
        (index < clips.len()).then(|| clips.remove(index))
    }

    /// How many steps a clip lasts: the length of its pattern, times
    /// the number of times it repeats.
    pub fn clip_length(&self, clip: &Clip) -> usize {
        self.patterns.get(clip.pattern).map_or(0, Pattern::length) * clip.repeats
    }

    /// The length of the playlist's timeline in steps: up to the end of
    /// the last clip or section, rounded up to a whole bar of the time
    /// signature. An empty playlist has length 0.
    ///
    /// This is the song as drawn. With repeated sections, the song as
    /// played is longer: see [`Project::arrangement`].
    pub fn song_length(&self) -> usize {
        let clips = self
            .playlist
            .clips
            .iter()
            .map(|clip| clip.start + self.clip_length(clip));
        let sections = self.playlist.sections.iter().map(Section::end);
        let end = clips.chain(sections).max().unwrap_or(0);
        self.time_signature.round_up_to_bar(end)
    }

    /// The whole song as it plays, with every repeat unrolled. Exports
    /// use this.
    pub fn arrangement(&self) -> Arrangement {
        arrange(self, 0, self.song_length())
    }

    /// What song mode plays: the loop region if one is set, or else the
    /// whole song.
    pub fn playback_arrangement(&self) -> Arrangement {
        match self.playlist.loop_region {
            Some(region) => arrange(self, region.start, region.end),
            None => self.arrangement(),
        }
    }

    // ----- Sections and the loop region -----

    /// Whether `section` could be in the playlist, next to every section
    /// but number `except`.
    fn section_fits(&self, section: &Section, except: Option<usize>) -> bool {
        section.length > 0
            && section.end() <= MAX_SONG_STEPS
            && (1..=MAX_REPEATS).contains(&section.repeats)
            && !self
                .playlist
                .sections
                .iter()
                .enumerate()
                .any(|(index, other)| Some(index) != except && other.overlaps(section))
    }

    /// Puts a section in its place in the sorted list and returns its index.
    fn insert_section(&mut self, section: Section) -> usize {
        let sections = &mut self.playlist.sections;
        let index = sections.partition_point(|other| other.start < section.start);
        sections.insert(index, section);
        index
    }

    /// Adds a section and returns its index. Returns `None` if it is
    /// empty, ends after [`MAX_SONG_STEPS`], or overlaps another section.
    pub fn add_section(&mut self, section: Section) -> Option<usize> {
        if !self.section_fits(&section, None) {
            return None;
        }
        Some(self.insert_section(section))
    }

    /// Replaces a section: to rename it, move it, resize it or change
    /// how often it repeats. Returns its new index (sections stay in
    /// order), or `None`, changing nothing, if the new one would not fit.
    pub fn set_section(&mut self, index: usize, section: Section) -> Option<usize> {
        if index >= self.playlist.sections.len() || !self.section_fits(&section, Some(index)) {
            return None;
        }
        self.playlist.sections.remove(index);
        Some(self.insert_section(section))
    }

    /// Removes a section. The clips under it stay where they are.
    pub fn remove_section(&mut self, index: usize) -> Option<Section> {
        let sections = &mut self.playlist.sections;
        (index < sections.len()).then(|| sections.remove(index))
    }

    /// Inserts `copies` copies of a section right after it: its clips
    /// and its marker, with the same repeats. Everything after the
    /// section moves later to make room. Returns `false`, changing
    /// nothing, if there is no such section or the song would get too long.
    ///
    /// Only the clips that start inside the section are copied, and they
    /// are copied whole. A clip that starts before the section and lasts
    /// into it stays a single clip; one that starts inside it and lasts
    /// past its end is copied whole too, so it can overlap the copy.
    pub fn duplicate_section(&mut self, index: usize, copies: usize) -> bool {
        let Some(section) = self.playlist.sections.get(index).cloned() else {
            return false;
        };
        let room = section.length * copies;
        if copies == 0 || !self.make_room(section.end(), room) {
            return false;
        }
        let inside: Vec<Clip> = self
            .playlist
            .clips
            .iter()
            .filter(|clip| section.contains(clip.start))
            .copied()
            .collect();
        for copy in 1..=copies {
            let shift = copy * section.length;
            for clip in &inside {
                self.playlist.clips.push(Clip {
                    start: clip.start + shift,
                    ..*clip
                });
            }
            let name = format!("{} {}", section.name, copy + 1);
            self.insert_section(Section {
                name,
                start: section.start + shift,
                ..section.clone()
            });
        }
        true
    }

    /// Turns a repeated section into separate copies, one per repeat,
    /// each played once, so each time round can be changed on its own.
    /// The song sounds the same before and after.
    pub fn unroll_section(&mut self, index: usize) -> bool {
        let Some(repeats) = self.playlist.sections.get(index).map(|s| s.repeats) else {
            return false;
        };
        if repeats < 2 {
            return false;
        }
        self.playlist.sections[index].repeats = 1;
        if self.duplicate_section(index, repeats - 1) {
            true
        } else {
            self.playlist.sections[index].repeats = repeats;
            false
        }
    }

    /// Moves every clip, section and loop edge at or after step `at`
    /// later by `steps`, leaving an empty gap. Returns `false`, changing
    /// nothing, if something would end up too late.
    fn make_room(&mut self, at: usize, steps: usize) -> bool {
        let playlist = &self.playlist;
        let too_late = playlist
            .clips
            .iter()
            .any(|clip| clip.start >= at && clip.start + steps >= MAX_SONG_STEPS)
            || playlist
                .sections
                .iter()
                .any(|section| section.start >= at && section.end() + steps > MAX_SONG_STEPS);
        if too_late {
            return false;
        }
        let shift = |step: &mut usize| {
            if *step >= at {
                *step += steps;
            }
        };
        for clip in &mut self.playlist.clips {
            shift(&mut clip.start);
        }
        for section in &mut self.playlist.sections {
            shift(&mut section.start);
        }
        if let Some(region) = &mut self.playlist.loop_region {
            shift(&mut region.start);
            shift(&mut region.end);
            region.end = region.end.min(MAX_SONG_STEPS);
        }
        true
    }

    /// Sets or clears the part of the song that song mode loops. Returns
    /// `false`, changing nothing, if the region is empty or too late.
    pub fn set_loop_region(&mut self, region: Option<LoopRegion>) -> bool {
        if region.is_some_and(|r| r.start >= r.end || r.end > MAX_SONG_STEPS) {
            return false;
        }
        self.playlist.loop_region = region;
        true
    }

    /// The notes held during `step` of pattern `pattern` by the channels
    /// that accompany: see [`Project::chord_notes_in_song`].
    pub fn chord_notes_in_pattern(&self, pattern: usize, step: usize) -> Vec<Note> {
        let mut parts = Vec::new();
        self.collect_parts(pattern, step, &mut parts);
        chord_notes(parts)
    }

    /// The notes held during timeline step `song_step` of the song (see
    /// [`Arrangement::timeline_step`]) by the channels that play chords, so the harmony keys can show which chord is playing.
    ///
    /// Only synths and MIDI Out channels that are not muted count, and of
    /// those only the ones holding two notes or more: a melody or a bass
    /// line plays one note at a time, and its passing notes would hide
    /// the chord. If no channel holds two notes, all the notes count.
    pub fn chord_notes_in_song(&self, song_step: usize) -> Vec<Note> {
        let mut parts = Vec::new();
        for clip in self.playlist.audible_clips() {
            if let Some(step) = self.step_in_clip(&clip, song_step) {
                self.collect_parts(clip.pattern, step, &mut parts);
            }
        }
        chord_notes(parts)
    }

    /// Adds the notes each melodic channel holds at `step` of `pattern`
    /// to `parts`, one list per channel.
    fn collect_parts(&self, pattern: usize, step: usize, parts: &mut Vec<Vec<Note>>) {
        let Some(pattern) = self.patterns.get(pattern) else {
            return;
        };
        for (index, channel) in self.channels.iter().enumerate() {
            let melodic = matches!(
                channel.kind,
                ChannelKind::Synth | ChannelKind::MidiOut { .. }
            );
            if melodic && !channel.muted {
                parts.push(pattern.sounding_at(index, step).collect());
            }
        }
    }

    /// Where `pattern` is during timeline step `song_step` of the song, if
    /// one of its clips is playing then: the step inside the pattern. The channel
    /// rack and the piano roll use this to show the playhead in song mode.
    pub fn pattern_step_in_song(&self, pattern: usize, song_step: usize) -> Option<usize> {
        self.playlist
            .audible_clips()
            .filter(|clip| clip.pattern == pattern)
            .find_map(|clip| self.step_in_clip(&clip, song_step))
    }

    /// The step of its pattern that `clip` plays on timeline step `step`,
    /// if it is playing then. A clip that repeats its pattern starts it
    /// again at step 0 each time round.
    fn step_in_clip(&self, clip: &Clip, step: usize) -> Option<usize> {
        let pattern_length = self.patterns.get(clip.pattern).map_or(0, Pattern::length);
        let end = clip.start + pattern_length * clip.repeats;
        (clip.start..end)
            .contains(&step)
            .then(|| (step - clip.start) % pattern_length)
    }
}

/// The notes of the parts that hold two notes or more, or of all the
/// parts if none does.
fn chord_notes(parts: Vec<Vec<Note>>) -> Vec<Note> {
    let chords: Vec<Note> = parts
        .iter()
        .filter(|part| part.len() >= 2)
        .flatten()
        .copied()
        .collect();
    if chords.is_empty() {
        parts.into_iter().flatten().collect()
    } else {
        chords
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
    fn the_chord_playing_is_found_under_the_melody() {
        let mut project = Project::demo();
        let spelling = project.scale.spelling();
        let label = |notes: Vec<Note>| analyze(notes).label(spelling);
        let first = label(project.chord_notes_in_pattern(CHORDS, 0));
        assert!(first.is_some());
        // The chords clip starts the song.
        assert_eq!(label(project.chord_notes_in_song(0)), first);
        // A melody note on another synth does not change the chord.
        let lead = project.add_channel(Channel::synth()).unwrap();
        let note = NoteEvent::new(Note::new(86), 0, 4);
        project.pattern_mut(CHORDS).unwrap().add_note(lead, note);
        assert_eq!(label(project.chord_notes_in_song(0)), first);
        // Muted channels are not heard.
        project.channel_mut(SYNTH).unwrap().muted = true;
        assert_eq!(project.chord_notes_in_song(0), [Note::new(86)]);
        // Nothing plays past the end of the song.
        assert!(project.chord_notes_in_song(10_000).is_empty());
    }

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
        assert!(!project.move_clip(0, 3, 32), "no track 3");
        assert!(project.move_clip(0, 0, 32));
        assert_eq!(project.playlist().clips()[0], Clip::new(0, 0, 32));
        assert!(!project.move_clip(0, 99, 0));
        assert_eq!(project.remove_clip(0), Some(Clip::new(0, 0, 32)));
        assert_eq!(project.remove_clip(0), None);
    }

    #[test]
    fn a_clip_below_the_last_track_adds_a_track() {
        let mut project = Project::template();
        assert_eq!(project.playlist().tracks().len(), 1);
        assert!(project.add_clip(Clip::new(0, 1, 0)).is_some());
        assert_eq!(project.playlist().tracks().len(), 2);
        assert_eq!(project.playlist().tracks()[1].name, "Track 2");
        // Moving a clip there does the same.
        assert!(project.move_clip(0, 2, 16));
        assert_eq!(project.playlist().tracks().len(), 3);
        assert!(
            project.add_clip(Clip::new(0, 4, 0)).is_none(),
            "one row at a time"
        );
    }

    #[test]
    fn projects_have_only_the_tracks_they_use() {
        assert_eq!(Project::template().playlist().tracks().len(), 1);
        let names: Vec<String> = Project::demo()
            .playlist()
            .tracks()
            .iter()
            .map(|track| track.name.clone())
            .collect();
        assert_eq!(names, ["Chords", "Drums"]);

        // Empty tracks in the middle stay: only the bottom is trimmed.
        let mut project = Project::empty();
        project.add_clip(Clip::new(0, 2, 0));
        project.remove_unused_tracks();
        assert_eq!(project.playlist().tracks().len(), 3);
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
    fn clips_can_repeat_their_pattern() {
        let mut project = Project::template();
        project.add_clip(Clip::new(0, 0, 16));
        assert!(project.set_clip_repeats(0, 3));
        assert!(!project.set_clip_repeats(0, 0));
        assert!(!project.set_clip_repeats(0, MAX_REPEATS + 1));
        assert_eq!(project.song_length(), 64);
        // Each time round, the pattern starts again.
        assert_eq!(project.pattern_step_in_song(0, 16), Some(0));
        assert_eq!(project.pattern_step_in_song(0, 37), Some(5));
        assert_eq!(project.pattern_step_in_song(0, 64), None);
        // The same pattern cannot start in the same place twice, whatever
        // its repeats.
        assert!(project.add_clip(Clip::new(0, 0, 16).repeated(2)).is_none());
    }

    #[test]
    fn sections_never_overlap_and_stay_in_order() {
        let mut project = Project::demo();
        assert_eq!(project.add_section(Section::new("Chorus", 64, 64)), Some(0));
        assert_eq!(project.add_section(Section::new("Intro", 0, 32)), Some(0));
        assert_eq!(project.add_section(Section::new("Verse", 32, 48)), None);
        assert_eq!(project.add_section(Section::new("Empty", 32, 0)), None);
        assert_eq!(project.add_section(Section::new("Verse", 32, 32)), Some(1));
        let names: Vec<&str> = project
            .playlist()
            .sections()
            .iter()
            .map(|s| s.name.as_str())
            .collect();
        assert_eq!(names, ["Intro", "Verse", "Chorus"]);

        let mut chorus = project.playlist().sections()[2].clone();
        chorus.repeats = 4;
        assert_eq!(project.set_section(2, chorus.clone()), Some(2));
        chorus.start = 48;
        assert_eq!(
            project.set_section(2, chorus),
            None,
            "would cover the verse"
        );
        assert_eq!(project.playlist().sections()[2].start, 64);
        assert_eq!(
            project.remove_section(0).map(|s| s.name),
            Some("Intro".into())
        );
    }

    #[test]
    fn sections_reach_past_the_last_clip() {
        let mut project = Project::template();
        project.add_section(Section::new("Outro", 0, 48));
        assert_eq!(project.song_length(), 48);
    }

    #[test]
    fn duplicating_a_section_makes_room_for_the_copy() {
        let mut project = Project::demo();
        let chorus = project.add_section(Section::new("Chorus", 32, 32)).unwrap();
        let clips_before = project.playlist().clips().len();
        assert!(project.duplicate_section(chorus, 1));
        // The chorus covers one chords clip and two beat clips.
        assert_eq!(project.playlist().clips().len(), clips_before + 3);
        assert_eq!(project.song_length(), 128 + 32);
        let copy = &project.playlist().sections()[1];
        assert_eq!((copy.name.as_str(), copy.start), ("Chorus 2", 64));
        // The fill moved along with the rest of the song.
        let fill = project
            .playlist()
            .clips()
            .iter()
            .find(|c| c.pattern == FILL);
        assert_eq!(fill.map(|c| c.start), Some(7 * 16 + 32));
    }

    #[test]
    fn unrolling_a_section_sounds_the_same() {
        let mut project = Project::demo();
        let mut chorus = Section::new("Chorus", 32, 64);
        chorus.repeats = 3;
        let index = project.add_section(chorus).unwrap();
        let before = project.arrangement();
        assert!(project.unroll_section(index));
        let after = project.arrangement();
        assert_eq!(after.length, before.length);
        assert_eq!(project.playlist().sections().len(), 3);
        assert!(project.playlist().sections().iter().all(|s| s.repeats == 1));
        // The same pieces of patterns start at the same moments.
        let starts = |song: &Arrangement| {
            let mut plays: Vec<(usize, usize, usize)> = song
                .plays
                .iter()
                .map(|play| (play.start, play.pattern, play.length))
                .collect();
            plays.sort();
            plays
        };
        assert_eq!(starts(&after), starts(&before));
        assert!(!project.unroll_section(index), "nothing left to unroll");
    }

    #[test]
    fn loop_regions_must_hold_something() {
        let mut project = Project::demo();
        assert!(!project.set_loop_region(Some(LoopRegion::new(32, 32))));
        assert!(project.set_loop_region(Some(LoopRegion::new(32, 64))));
        assert_eq!(project.playback_arrangement().length, 32);
        assert!(project.set_loop_region(None));
        assert_eq!(project.playback_arrangement().length, 128);
    }

    #[test]
    fn muted_tracks_hide_the_playhead() {
        let mut project = Project::demo();
        project.track_mut(1).unwrap().muted = true;
        assert_eq!(project.pattern_step_in_song(BEAT, 50), None);
    }

    #[test]
    fn inserting_a_track_moves_the_clips_below_it() {
        let mut project = Project::empty();
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.add_clip(Clip::new(0, 2, 0)).unwrap();
        assert_eq!(project.insert_track(1), Some(1));
        let tracks: Vec<usize> = project.playlist().clips().iter().map(|c| c.track).collect();
        assert_eq!(tracks, vec![0, 3]);
        assert_eq!(project.playlist().tracks().len(), 17);
    }

    #[test]
    fn removing_a_track_removes_its_clips() {
        let mut project = Project::empty();
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.add_clip(Clip::new(0, 1, 0)).unwrap();
        project.add_clip(Clip::new(0, 2, 0)).unwrap();
        assert!(project.remove_track(1));
        let tracks: Vec<usize> = project.playlist().clips().iter().map(|c| c.track).collect();
        assert_eq!(tracks, vec![0, 1]);
        assert_eq!(project.playlist().tracks().len(), 15);
    }

    #[test]
    fn the_last_track_stays() {
        let mut project = Project::empty();
        while project.playlist().tracks().len() > 1 {
            assert!(project.remove_track(0));
        }
        assert!(!project.remove_track(0));
    }

    #[test]
    fn swapping_tracks_swaps_their_clips() {
        let mut project = Project::empty();
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.add_clip(Clip::new(0, 1, 4)).unwrap();
        assert!(project.swap_tracks(0));
        let clips = project.playlist().clips();
        assert_eq!((clips[0].track, clips[1].track), (1, 0));
        assert_eq!(project.playlist().tracks()[0].name, "Track 2");
        assert!(!project.swap_tracks(15));
    }
}
