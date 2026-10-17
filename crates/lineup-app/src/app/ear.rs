//! Ear training: plays the window's questions on instruments of their own.
//!
//! A question is a [`Phrase`] for the audio thread, apart from the song:
//! its piano, bass and drums are built here the way channels are, from
//! the SoundFont when it is on, so they sound like the band.

use eframe::egui;
use lineup_audio::{Command, Phrase, PhraseNote};
use lineup_core::ear_training::{Cue, Part};
use lineup_core::{Channel, general_midi};

use super::LineupApp;
use crate::ear_training::EarAction;

/// The channel a part of a question is played like.
fn channel_for(part: Part) -> Channel {
    match part {
        Part::Piano => Channel {
            program: Some(general_midi::ACOUSTIC_GRAND_PIANO),
            ..Channel::synth()
        },
        Part::Bass => Channel {
            program: Some(general_midi::FINGER_BASS),
            ..Channel::synth()
        },
        Part::Drum(drum) => Channel::drum(drum),
    }
}

impl LineupApp {
    /// Draws the ear-training window and plays what it asks for.
    pub(super) fn ear_training_window(&mut self, ctx: &egui::Context, area: egui::Rect) {
        for action in self.ear_training.show(ctx, area) {
            match action {
                EarAction::Play(cues) => self.play_cues(&cues),
                EarAction::Stop => self.send(Command::StopPhrase),
            }
        }
    }

    /// Plays `cues` as a phrase, in place of the last one. The song
    /// stops first, so the question is heard alone.
    fn play_cues(&mut self, cues: &[Cue]) {
        if self.playing {
            self.pause();
        }
        let sample_rate = self.sample_rate();
        let mut parts: Vec<Part> = Vec::new();
        let mut instruments = Vec::new();
        let mut notes = Vec::with_capacity(cues.len());
        for cue in cues {
            let instrument = match parts.iter().position(|&part| part == cue.part) {
                Some(index) => index,
                None => {
                    parts.push(cue.part);
                    instruments.push(self.build_instrument(&channel_for(cue.part), sample_rate));
                    parts.len() - 1
                }
            };
            notes.push(PhraseNote {
                instrument,
                start: cue.start,
                length: cue.length,
                pitch: cue.note,
                velocity: cue.velocity,
            });
        }
        let phrase = Phrase::new(instruments, &notes, sample_rate);
        self.send(Command::PlayPhrase(Box::new(phrase)));
    }
}
