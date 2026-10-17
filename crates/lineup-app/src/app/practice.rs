//! Practising: the count-in, the metronome, the practice speed, tap
//! tempo, and playing a part of the band yourself.
//!
//! None of these is part of the song. The count-in and the metronome are
//! clicks the audio thread makes on its own (see `lineup_audio`'s
//! metronome), so they never end up in a saved file or a WAV export, and
//! the practice speed only changes the tempo the audio thread is told:
//! the song keeps its own tempo, which the TEMPO box shows. The count-in
//! and the metronome are remembered between runs, as settings; the speed
//! starts at 100% each time.

use eframe::egui;
use lineup_audio::{Command, PlayMode};
use lineup_core::{LoopRegion, Player, Tempo, TimeSignature};
use serde::{Deserialize, Serialize};

use super::LineupApp;
use crate::chord_sheet::{PracticeAction, PracticeView, SPEEDS};

/// The practice speed that plays the song at its own tempo, in percent.
pub(super) const FULL_SPEED: u16 = 100;

/// The speeds the Practice menu offers; any other can be typed on the
/// chord sheet.
const MENU_SPEEDS: [u16; 10] = [50, 60, 70, 75, 80, 90, 100, 110, 125, 150];

/// A tap that comes longer than this after the last one, in seconds,
/// starts counting again: slower than 30 BPM is not a tempo anyone taps.
const TAP_GAP: f64 = 2.0;

/// How many taps the tempo is worked out from: the last few, so a tempo
/// that settles as you tap is followed.
const TAPS: usize = 8;

/// The undo step tap tempo makes. Taps in a row join one step.
const TAP_STEP: &str = "tap tempo";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    count_in: bool,
    metronome: bool,
}

/// Where egui keeps the settings between runs.
fn settings_id() -> egui::Id {
    egui::Id::unique("practice-settings")
}

/// The practice settings, and what the audio thread was last told.
pub(super) struct Practice {
    settings: Settings,
    /// The practice speed, in percent of the song's tempo.
    speed: u16,
    /// When the last taps came, in seconds on egui's clock.
    taps: Vec<f64>,
    /// The count-in and metronome the audio thread has, or `None` if it
    /// must be told again.
    flags_sent: Option<(bool, bool)>,
    /// The pattern whose clicks the audio thread has, with the length and
    /// time signature they were worked out for.
    clicks_sent: Option<(usize, usize, TimeSignature)>,
}

impl Practice {
    pub(super) fn new(ctx: &egui::Context) -> Self {
        let settings = ctx
            .data_mut(|data| data.get_persisted(settings_id()))
            .unwrap_or_default();
        Practice {
            settings,
            speed: FULL_SPEED,
            taps: Vec::new(),
            flags_sent: None,
            clicks_sent: None,
        }
    }

    /// The practice speed as a factor: 0.8 for 80%.
    pub(super) fn speed(&self) -> f32 {
        f32::from(self.speed) / 100.0
    }

    /// Has everything sent to the audio thread again, after it was
    /// cleared for another song.
    pub(super) fn resend(&mut self) {
        self.flags_sent = None;
        self.clicks_sent = None;
    }

    /// What the chord sheet's practice row shows.
    pub(super) fn view(&self) -> PracticeView {
        PracticeView {
            count_in: self.settings.count_in,
            metronome: self.settings.metronome,
            speed: self.speed,
        }
    }

    /// Notes a tap at `now` seconds. Returns the tempo of the taps so
    /// far, rounded to a whole BPM, once there are two, and whether it
    /// carries on from an earlier tap's tempo.
    fn tap(&mut self, now: f64) -> Option<(Tempo, bool)> {
        if self.taps.last().is_none_or(|&last| now - last > TAP_GAP) {
            self.taps.clear();
        }
        self.taps.push(now);
        if self.taps.len() > TAPS {
            self.taps.remove(0);
        }
        let (&first, &last) = (self.taps.first()?, self.taps.last()?);
        let beats = self.taps.len() - 1;
        if beats == 0 || last <= first {
            return None;
        }
        let bpm = (60.0 * beats as f64 / (last - first)).round() as f32;
        Some((Tempo::new(bpm), self.taps.len() > 2))
    }
}

/// `tempo` at `speed` (1.0 for the song's own tempo).
pub(super) fn at_speed(tempo: Tempo, speed: f32) -> Tempo {
    if speed == 1.0 {
        tempo
    } else {
        Tempo::new(tempo.bpm() * speed)
    }
}

impl LineupApp {
    /// Called every frame: tells the audio thread about the count-in,
    /// the metronome and the clicks of the selected pattern when they
    /// change, and remembers the settings.
    pub(super) fn practice_frame(&mut self, ctx: &egui::Context) {
        let flags = (
            self.practice.settings.count_in,
            self.practice.settings.metronome,
        );
        if self.practice.flags_sent != Some(flags) {
            if self.practice.flags_sent.is_some() {
                let settings = self.practice.settings.clone();
                ctx.data_mut(|data| data.insert_persisted(settings_id(), settings));
            }
            self.send(Command::SetCountIn(flags.0));
            self.send(Command::SetMetronome(flags.1));
            self.practice.flags_sent = Some(flags);
        }
        let length = self.project.patterns()[self.pattern].length();
        let time = self.project.pattern_time(self.pattern);
        let clicks = Some((self.pattern, length, time));
        if self.practice.clicks_sent != clicks {
            self.send(Command::SetPatternClicks(
                self.project.pattern_clicks(self.pattern),
            ));
            self.practice.clicks_sent = clicks;
        }
    }

    /// Does what the chord sheet's practice row asks.
    pub(super) fn apply_practice(&mut self, action: PracticeAction) {
        match action {
            PracticeAction::CountIn(on) => self.set_count_in(on),
            PracticeAction::Metronome(on) => self.set_metronome(on),
            PracticeAction::Speed(speed) => self.set_speed(speed),
            PracticeAction::Tap(now) => self.tap_tempo(now),
            PracticeAction::YouPlay(player, you) => self.set_you_play(player, you),
            PracticeAction::Loop(region) => self.loop_bars(region),
        }
    }

    /// Loops `region` of the song, from its start, or plays the whole
    /// song again, carrying on from where the playhead is in it.
    fn loop_bars(&mut self, region: Option<LoopRegion>) {
        let before = self.project.playlist().loop_region();
        let here = self.playhead().unwrap_or(self.cursor);
        self.edit("set loop", |project| project.set_loop_region(region));
        if region.is_some() {
            self.set_mode(PlayMode::Song);
            self.seek(0);
        } else if let Some(before) = before
            && self.mode == PlayMode::Song
        {
            // The playhead counted from the loop's start; now it counts
            // from the song's.
            self.seek(before.start + here);
        }
    }

    pub(super) fn set_count_in(&mut self, on: bool) {
        self.practice.settings.count_in = on;
    }

    pub(super) fn set_metronome(&mut self, on: bool) {
        self.practice.settings.metronome = on;
    }

    /// Plays at `speed` percent of the song's tempo, from the next step.
    pub(super) fn set_speed(&mut self, speed: u16) {
        let speed = speed.clamp(*SPEEDS.start(), *SPEEDS.end());
        if speed == self.practice.speed {
            return;
        }
        self.practice.speed = speed;
        let factor = self.practice.speed();
        self.send(Command::SetTempo(at_speed(self.project.tempo, factor)));
        let song = self.project.playback_arrangement();
        let tempos = self
            .project
            .played_tempos(&song)
            .into_iter()
            .map(|tempo| at_speed(tempo, factor))
            .collect();
        self.send(Command::SetSongTempos(tempos));
    }

    /// A tap of the Tap button: two or more in a row set the song's
    /// tempo to theirs, as one undo step.
    pub(super) fn tap_tempo(&mut self, now: f64) {
        let Some((tempo, carries_on)) = self.practice.tap(now) else {
            self.notice = Some("Tap again on every beat to set the tempo".to_string());
            return;
        };
        if carries_on {
            self.history.reopen(TAP_STEP);
        }
        self.edit(TAP_STEP, |project| project.scale_tempo(tempo));
        self.notice = Some(format!("Tempo {} BPM, tapped", tempo.bpm()));
    }

    /// Leaves `player`'s part to you, or gives it back to the band. For
    /// every player but the drums, your keyboard then plays its sound.
    pub(super) fn set_you_play(&mut self, player: Player, you: bool) {
        let label = if you {
            "play a part yourself"
        } else {
            "give the part back"
        };
        let done = self.edit(label, |project| project.set_you_play(player, you));
        if !done {
            return;
        }
        let name = player.name().to_lowercase();
        if !you {
            self.notice = Some(format!("The band plays the {name} again"));
            return;
        }
        match self.project.player_channel(player) {
            Some(channel) => {
                self.piano_roll.select(channel);
                self.notice = Some(format!(
                    "You play the {name}: the band leaves it out, and your keyboard plays \
                     its sound"
                ));
            }
            None => self.notice = Some(format!("You play the {name}: the band leaves it out")),
        }
    }

    /// The Practice menu, after the Sounds menu.
    pub(super) fn practice_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Practice", |ui| {
            let mut count_in = self.practice.settings.count_in;
            if ui
                .checkbox(&mut count_in, "Count-in")
                .on_hover_text("A bar of clicks before the music starts, so you come in on time")
                .changed()
            {
                self.set_count_in(count_in);
            }
            let mut metronome = self.practice.settings.metronome;
            if ui
                .checkbox(&mut metronome, "Metronome")
                .on_hover_text("A click on every beat while the music plays")
                .changed()
            {
                self.set_metronome(metronome);
            }
            ui.menu_button(format!("Speed ({}%)", self.practice.speed), |ui| {
                ui.weak("How fast to play, as a percentage of the song's tempo.");
                for speed in MENU_SPEEDS {
                    let chosen = self.practice.speed == speed;
                    if ui.selectable_label(chosen, format!("{speed}%")).clicked() {
                        self.set_speed(speed);
                    }
                }
            });
            ui.separator();
            ui.weak(
                "Tap tempo, playing a part yourself and looping\n\
                 bars are on the chord sheet (F5).",
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn practice() -> Practice {
        Practice {
            settings: Settings::default(),
            speed: FULL_SPEED,
            taps: Vec::new(),
            flags_sent: None,
            clicks_sent: None,
        }
    }

    #[test]
    fn taps_set_the_tempo_of_their_beats() {
        let mut practice = practice();
        assert_eq!(practice.tap(10.0), None, "one tap is not a tempo yet");
        // Half a second apart: 120 BPM.
        assert_eq!(practice.tap(10.5), Some((Tempo::new(120.0), false)));
        // A little uneven, as hands are: the average is taken.
        assert_eq!(practice.tap(11.02), Some((Tempo::new(118.0), true)));
        assert_eq!(practice.tap(11.5), Some((Tempo::new(120.0), true)));
        // After a pause, counting starts again.
        assert_eq!(practice.tap(20.0), None);
        assert_eq!(practice.tap(21.0), Some((Tempo::new(60.0), false)));
    }

    #[test]
    fn only_the_last_taps_count() {
        let mut practice = practice();
        let mut now = 0.0;
        for _ in 0..10 {
            practice.tap(now);
            now += 1.0;
        }
        // Then faster: the slow taps drop out of the average.
        let mut last = None;
        for _ in 0..TAPS {
            last = practice.tap(now);
            now += 0.5;
        }
        assert_eq!(last.map(|(tempo, _)| tempo), Some(Tempo::new(120.0)));
    }

    #[test]
    fn the_speed_scales_the_tempo() {
        assert_eq!(at_speed(Tempo::new(120.0), 0.75), Tempo::new(90.0));
        assert_eq!(at_speed(Tempo::new(120.0), 1.0), Tempo::new(120.0));
    }
}
