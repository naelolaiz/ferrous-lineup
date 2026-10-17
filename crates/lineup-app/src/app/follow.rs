//! Chord follow: while the music plays, the chord you hold on a MIDI or
//! typing keyboard becomes the chord the accompaniment plays, as on an
//! arranger keyboard.
//!
//! The work is done by [`lineup_core::library::follow`]; this part decides
//! when. In each frame where a key went down, the keys held down are
//! read as a chord. When it
//! differs from the one being followed, the moved notes of every chord
//! and bass channel are sent to the audio thread with
//! [`Command::SetNotes`], which takes effect from the next note it
//! plays. The project itself never changes, so undo, saving and export
//! see the notes as written. When playback stops or the option is turned
//! off, the written notes are sent back.

use lineup_audio::Command;
use lineup_core::harmony::Chord;
use lineup_core::library::follow::{Accompaniment, chord_held};
use lineup_core::swing;

use super::LineupApp;

/// What chord follow remembers.
#[derive(Default)]
pub(super) struct Follow {
    /// Whether the option is on, in the MIDI menu.
    pub(super) on: bool,
    /// The chord the audio thread is playing the accompaniment in, if it
    /// is following one. The last chord held stays after the keys come
    /// up, as an arranger keyboard's "memory" does.
    chord: Option<Chord>,
    /// The chords the patterns play, heard once and kept until the
    /// project changes.
    heard: Option<Accompaniment>,
    /// The channels the audio thread plays moved notes on, by pattern.
    moved: Vec<(usize, usize)>,
    /// Whether a key went down since the keys were last read. Only a key
    /// going down asks for a new chord: letting go of C E G one key at a
    /// time would otherwise ask for C E, then G, on the way up.
    key_went_down: bool,
}

impl Follow {
    /// The chord being followed, if any.
    pub(super) fn chord(&self) -> Option<Chord> {
        self.chord
    }

    /// Called when a key goes down, on either keyboard.
    pub(super) fn key_down(&mut self) {
        self.key_went_down = true;
    }

    /// Forgets everything, after the audio thread was cleared for a new
    /// project: it has the written notes again.
    pub(super) fn forget(&mut self) {
        self.chord = None;
        self.heard = None;
        self.moved.clear();
    }
}

impl LineupApp {
    /// Follows the keys held down, once per frame, after they were read.
    pub(super) fn follow_chords(&mut self) {
        if !self.live.follow.on || !self.playing {
            self.live.follow.key_went_down = false;
            self.restore_accompaniment();
            return;
        }
        if !std::mem::take(&mut self.live.follow.key_went_down) {
            return;
        }
        let Some(chord) = chord_held(&self.live.held_pitches()) else {
            return;
        };
        if self.live.follow.chord != Some(chord) {
            self.play_accompaniment_in(chord);
        }
    }

    /// Sends the accompaniment moved into `chord` to the audio thread.
    fn play_accompaniment_in(&mut self, chord: Chord) {
        let heard = self
            .live
            .follow
            .heard
            .take()
            .unwrap_or_else(|| Accompaniment::hear(&self.project));
        let followed = heard.follow(&self.project, chord);
        self.live.follow.moved = heard.channels().collect();
        self.live.follow.heard = Some(heard);
        self.live.follow.chord = Some(chord);
        for moved in followed {
            // Played with the song's swing, as the written notes are.
            let swing = self.project.pattern_swing(moved.pattern);
            let hits = self.project.plays_hits(moved.channel);
            self.send(Command::SetNotes {
                pattern: moved.pattern,
                channel: moved.channel,
                notes: swing::swung(&moved.notes, swing, hits),
            });
        }
    }

    /// Sends the written notes back for every channel that was following.
    fn restore_accompaniment(&mut self) {
        self.live.follow.chord = None;
        for (pattern, channel) in std::mem::take(&mut self.live.follow.moved) {
            if pattern >= self.project.patterns().len() || channel >= self.project.channels().len()
            {
                continue;
            }
            let notes = self.project.played_notes(pattern, channel).into_owned();
            self.send(Command::SetNotes {
                pattern,
                channel,
                notes,
            });
        }
    }

    /// After the project changed (an edit, undo or redo): the chords are
    /// heard again, and the chord being followed is played again from
    /// the new notes.
    pub(super) fn follow_project_changed(&mut self) {
        self.live.follow.heard = None;
        if let Some(chord) = self.live.follow.chord {
            self.restore_accompaniment();
            self.play_accompaniment_in(chord);
        }
    }
}
