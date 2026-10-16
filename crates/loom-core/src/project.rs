//! The project: channels, the pattern, the tempo and the key, kept in sync.

use std::path::PathBuf;

use crate::harmony::Scale;
use crate::pattern::{NoteEvent, Pattern};
use crate::{Note, Tempo};

/// The most channels a project can have. The audio thread reserves room
/// for this many up front so that adding a channel never allocates.
pub const MAX_CHANNELS: usize = 32;

/// The built-in drum sounds, synthesized by the audio engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum SampleSource {
    /// One of the drums built into Ferrous Loom.
    Builtin(Drum),
    /// A WAV file on disk.
    File(PathBuf),
}

/// What kind of instrument a channel is.
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelKind {
    /// Plays a recorded or built-in sound from start to end for each
    /// note, faster for higher notes and slower for lower ones.
    Sampler(SampleSource),
    /// The built-in "Loom Osc" synthesizer, which can play chords.
    Synth,
}

/// One row of the channel rack.
#[derive(Debug, Clone, PartialEq)]
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

/// Everything the user is working on.
///
/// The fields are private so the channel list and the pattern rows can
/// only change together: row `i` of the pattern always belongs to
/// channel `i`.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub tempo: Tempo,
    /// The key the piano roll highlights and names chords in.
    pub scale: Scale,
    channels: Vec<Channel>,
    pattern: Pattern,
}

impl Project {
    /// An empty project with no channels.
    pub fn empty() -> Self {
        Project {
            tempo: Tempo::default(),
            scale: Scale::default(),
            channels: Vec::new(),
            pattern: Pattern::new(0, 16),
        }
    }

    /// The starting project: two bars of a simple beat, and the Loom Osc
    /// playing the chords C, Am, F and G (I, vi, IV, V in C major).
    pub fn demo() -> Self {
        let mut project = Project::empty();
        project.pattern.set_length(32);
        for drum in [Drum::Kick, Drum::Clap, Drum::HiHat, Drum::Snare] {
            project.add_channel(Channel::drum(drum));
        }
        let synth = project.add_channel(Channel::synth()).unwrap_or(0);

        let beat: [(usize, &[usize]); 3] = [
            (0, &[0, 4, 8, 12]),  // kick: four on the floor
            (1, &[4, 12]),        // clap: beats 2 and 4
            (2, &[2, 6, 10, 14]), // hat: the off-beats
        ];
        for bar_start in [0, 16] {
            for (channel, steps) in beat {
                for &step in steps {
                    project.pattern.set(channel, bar_start + step, true);
                }
            }
        }

        // Each chord lasts half a bar: a bass note and three chord notes,
        // chosen so that each chord moves as little as possible from the
        // last one (good "voice leading").
        let chords: [[u8; 4]; 4] = [
            [48, 60, 64, 67], // C:  C3, C4 E4 G4
            [45, 57, 60, 64], // Am: A2, A3 C4 E4
            [41, 57, 60, 65], // F:  F2, A3 C4 F4
            [43, 59, 62, 67], // G:  G2, B3 D4 G4
        ];
        for (i, chord) in chords.iter().enumerate() {
            for &midi in chord {
                let note = NoteEvent::new(Note::new(midi), i * 8, 8);
                project.pattern.add_note(synth, note);
            }
        }
        project
    }

    pub fn channels(&self) -> &[Channel] {
        &self.channels
    }

    pub fn channel_mut(&mut self, index: usize) -> Option<&mut Channel> {
        self.channels.get_mut(index)
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// The notes of one channel.
    pub fn notes(&self, channel: usize) -> &[NoteEvent] {
        self.pattern.notes(channel)
    }

    /// Flips one step and returns its new value.
    pub fn toggle_step(&mut self, channel: usize, step: usize) -> Option<bool> {
        self.pattern.toggle(channel, step)
    }

    /// Sets one step. Returns `false` if it is outside the grid.
    pub fn set_step(&mut self, channel: usize, step: usize, on: bool) -> bool {
        self.pattern.set(channel, step, on)
    }

    /// Adds a note; see [`Pattern::add_note`].
    pub fn add_note(&mut self, channel: usize, note: NoteEvent) -> Option<usize> {
        self.pattern.add_note(channel, note)
    }

    /// Removes a note; see [`Pattern::remove_note`].
    pub fn remove_note(&mut self, channel: usize, index: usize) -> Option<NoteEvent> {
        self.pattern.remove_note(channel, index)
    }

    /// Replaces a note; see [`Pattern::replace_note`].
    pub fn replace_note(&mut self, channel: usize, index: usize, note: NoteEvent) -> bool {
        self.pattern.replace_note(channel, index, note)
    }

    pub fn set_pattern_length(&mut self, length: usize) {
        self.pattern.set_length(length);
    }

    /// Adds a channel at the bottom of the rack, with no notes.
    /// Returns its index, or `None` if the rack is full.
    pub fn add_channel(&mut self, channel: Channel) -> Option<usize> {
        if self.channels.len() >= MAX_CHANNELS {
            return None;
        }
        self.channels.push(channel);
        self.pattern.push_row();
        Some(self.channels.len() - 1)
    }

    /// Removes a channel and its notes. Returns the removed channel.
    pub fn remove_channel(&mut self, index: usize) -> Option<Channel> {
        if index >= self.channels.len() {
            return None;
        }
        self.pattern.remove_row(index);
        Some(self.channels.remove(index))
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

    #[test]
    fn demo_has_a_beat() {
        let project = Project::demo();
        assert_eq!(project.channels().len(), 5);
        assert_eq!(project.channels()[0].name, "Kick");
        assert!(project.pattern().is_on(0, 0));
        assert!(project.pattern().is_on(0, 16), "the beat repeats in bar 2");
        assert!(project.pattern().is_on(1, 4));
        assert!(!project.pattern().is_on(3, 0), "snare starts empty");
    }

    #[test]
    fn demo_plays_a_chord_progression() {
        use crate::harmony::analyze;
        let project = Project::demo();
        let synth = 4;
        assert!(!project.pattern().is_step_row(synth));
        let chords: Vec<String> = [0, 8, 16, 24]
            .into_iter()
            .map(|step| {
                analyze(project.pattern().sounding_at(synth, step))
                    .label(project.scale.spelling())
                    .unwrap()
            })
            .collect();
        assert_eq!(chords, ["C", "Am", "F", "G"]);
    }

    #[test]
    fn channels_and_rows_stay_in_sync() {
        let mut project = Project::demo();
        let new = project.add_channel(Channel::drum(Drum::Snare)).unwrap();
        assert_eq!(project.pattern().rows(), project.channels().len());
        assert!(project.notes(new).is_empty(), "new rows start empty");

        // Removing the kick moves the clap's steps up to row 0.
        let removed = project.remove_channel(0).unwrap();
        assert_eq!(removed.name, "Kick");
        assert_eq!(project.pattern().rows(), project.channels().len());
        assert_eq!(project.channels()[0].name, "Clap");
        assert!(project.pattern().is_on(0, 4));
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
}
