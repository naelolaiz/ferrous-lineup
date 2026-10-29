//! Practising: the count-in, the metronome, the practice speed and the
//! speed trainer, tap tempo, and playing a part of the band yourself.
//!
//! None of these is part of the song. The count-in and the metronome are
//! clicks the audio thread makes on its own (see `lineup_audio`'s
//! metronome), so they never end up in a saved file or a WAV export, and
//! the practice speed only changes the tempo the audio thread is told:
//! the song keeps its own tempo, which the TEMPO box shows. The count-in,
//! the metronome and the trainer's speeds are remembered between runs,
//! as settings; the speed starts at 100% each time, with the trainer off.
//!
//! The speed trainer raises the speed each time the loop comes round.
//! The audio thread counts those times (its laps, see
//! [`lineup_audio::AudioEngine::laps`]), and the window looks at the
//! count once a frame. A frame comes a few milliseconds after the loop's
//! first step has started, too late for that step, so the window sends
//! the tempos of each new speed ahead of time, and the audio thread
//! takes them up as the loop comes round ([`Command::SetLapTempos`]):
//! the new speed starts exactly with the loop.

use eframe::egui;
use lineup_audio::{Command, LapTempos, PlayMode};
use lineup_core::{LoopRegion, Player, STEPS_PER_BEAT, Tempo, TimeSignature};
use serde::{Deserialize, Serialize};

use super::{Audio, LineupApp};
use crate::chord_sheet::{PracticeAction, PracticeView, SPEEDS, Trainer};
use crate::i18n::{tr, tr_string, trf};
use crate::icons;

/// The practice speed that plays the song at its own tempo, in percent.
pub(super) const FULL_SPEED: u16 = 100;

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
    trainer: Trainer,
}

/// Where egui keeps the settings between runs.
fn settings_id() -> egui::Id {
    egui::Id::unique("practice-settings")
}

/// The practice settings, and what the audio thread was last told.
pub(super) struct Practice {
    settings: Settings,
    /// The settings as they were last remembered.
    saved: Settings,
    /// The practice speed, in percent of the song's tempo.
    speed: u16,
    /// Whether the speed trainer is on.
    training: bool,
    /// How many passes have played at this speed since the trainer last
    /// moved it, while it is on.
    passes: u16,
    /// The audio thread's laps when the trainer last looked, or `None`
    /// until it looks.
    laps_seen: Option<usize>,
    /// The speed change the audio thread was asked to make as the loop
    /// next comes round, from one speed to another, with the tempos
    /// sent when it was asked; `None` when it may have one this does
    /// not know of, and must be told again.
    lap_change: Option<(Option<(u16, u16)>, u64)>,
    /// How many times the audio thread was sent the tempos to play:
    /// the speed asked for at the next lap was worked out from the
    /// tempos of the time, so it is worked out again when they change.
    tempos_sent: u64,
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
        let mut settings: Settings = ctx
            .data_mut(|data| data.get_persisted(settings_id()))
            .unwrap_or_default();
        settings.trainer = settings.trainer.clamped();
        Practice::with(settings)
    }

    fn with(settings: Settings) -> Self {
        Practice {
            saved: settings.clone(),
            settings,
            speed: FULL_SPEED,
            training: false,
            passes: 0,
            laps_seen: None,
            lap_change: None,
            tempos_sent: 0,
            taps: Vec::new(),
            flags_sent: None,
            clicks_sent: None,
        }
    }

    /// The practice speed as a factor: 0.8 for 80%.
    pub(super) fn speed(&self) -> f32 {
        f32::from(self.speed) / 100.0
    }

    /// Notes that the audio thread was sent the tempos to play.
    pub(super) fn tempos_sent(&mut self) {
        self.tempos_sent = self.tempos_sent.wrapping_add(1);
    }

    /// Has everything sent to the audio thread again, after it was
    /// cleared for another song.
    pub(super) fn resend(&mut self) {
        self.flags_sent = None;
        self.clicks_sent = None;
        self.lap_change = None;
    }

    /// What the chord sheet's practice row shows.
    pub(super) fn view(&self) -> PracticeView {
        PracticeView {
            count_in: self.settings.count_in,
            metronome: self.settings.metronome,
            speed: self.speed,
            training: self.training,
            trainer: self.settings.trainer,
        }
    }

    /// Notes that the audio thread has come round its loop `laps` times
    /// in all. While the trainer is on, returns the speed the passes
    /// since the last look move on to, if that is another speed.
    fn came_round(&mut self, laps: usize) -> Option<u16> {
        let seen = self.laps_seen.replace(laps);
        if !self.training {
            return None;
        }
        // A new audio thread, after a restart, counts from 0 again.
        let new = laps.checked_sub(seen?)?;
        if new == 0 {
            return None;
        }
        // The change asked for is made, unless it came too late.
        self.lap_change = None;
        let trainer = self.settings.trainer;
        let every = usize::from(trainer.every.max(1));
        let passes = usize::from(self.passes) + new;
        let mut speed = self.speed;
        for _ in 0..passes / every {
            let next = trainer.next(speed);
            if next == speed {
                break;
            }
            speed = next;
        }
        self.passes = (passes % every) as u16;
        (speed != self.speed).then_some(speed)
    }

    /// The speed change the trainer makes when the loop next comes
    /// round, if it makes one: from the speed now to the next.
    fn next_change(&self) -> Option<(u16, u16)> {
        let trainer = self.settings.trainer;
        if !self.training || self.passes + 1 < trainer.every.max(1) {
            return None;
        }
        let next = trainer
            .next(self.speed)
            .clamp(*SPEEDS.start(), *SPEEDS.end());
        (next != self.speed).then_some((self.speed, next))
    }

    /// Notes a tap at `now` seconds, on beats `quarters` quarter notes
    /// long. Returns the tempo of the taps so far, in quarter notes a
    /// minute (the tempo's unit) rounded to a whole BPM, once there are
    /// two, and whether it carries on from an earlier tap's tempo.
    fn tap(&mut self, now: f64, quarters: f64) -> Option<(Tempo, bool)> {
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
        let bpm = (60.0 * quarters * beats as f64 / (last - first)).round() as f32;
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
    /// Called every frame: moves the speed trainer on as the loop comes
    /// round, and sends the tempos of its next speed ahead of time;
    /// tells the audio thread about the count-in, the metronome and the
    /// clicks of the selected pattern when they change, and remembers
    /// the settings.
    pub(super) fn practice_frame(&mut self, ctx: &egui::Context) {
        if self.practice.settings != self.practice.saved {
            let settings = self.practice.settings.clone();
            self.practice.saved = settings.clone();
            ctx.data_mut(|data| data.insert_persisted(settings_id(), settings));
        }
        if let Audio::Running(engine) = &self.audio {
            let laps = engine.laps();
            self.train(laps);
        }
        let change = self.practice.next_change();
        let sent = change.map_or(0, |_| self.practice.tempos_sent);
        if self.practice.lap_change != Some((change, sent)) {
            let tempos = change.map(|(_, next)| {
                let (tempo, song) = self.tempos_at(next);
                LapTempos { tempo, song }
            });
            self.send(Command::SetLapTempos(tempos));
            self.practice.lap_change = Some((change, sent));
        }
        let flags = (
            self.practice.settings.count_in,
            self.practice.settings.metronome,
        );
        if self.practice.flags_sent != Some(flags) {
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
            PracticeAction::Training(on) => self.set_training(on),
            PracticeAction::Trainer(trainer) => self.set_trainer(trainer),
            PracticeAction::Tap(now) => self.tap_tempo(now),
            PracticeAction::YouPlay(player, you) => self.set_you_play(player, you),
            PracticeAction::Loop(region) => self.loop_bars(region),
        }
    }

    /// Loops `region` of the song, from its start, or plays the whole
    /// song again, carrying on from where the playhead is in it.
    pub(super) fn loop_bars(&mut self, region: Option<LoopRegion>) {
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

    /// Frequently used playback controls stay beside the main transport.
    pub(super) fn playback_controls(&mut self, ui: &mut egui::Ui) {
        crate::widgets::follow_button(ui);
        let practice = self.practice.view();
        let count_in = icons::toggle(ui, "Count-in", practice.count_in, icons::count_in);
        if crate::theme::hint(
            count_in,
            tr("Count-in: one bar of clicks before playback starts"),
        )
        .clicked()
        {
            self.set_count_in(!practice.count_in);
        }
        let metronome = icons::toggle(ui, "Metronome", practice.metronome, icons::metronome);
        if crate::theme::hint(
            metronome,
            tr("Metronome: a click on every beat during playback"),
        )
        .clicked()
        {
            self.set_metronome(!practice.metronome);
        }
    }

    /// Plays at `speed` percent of the song's tempo, from the next step,
    /// as chosen by hand: the speed trainer, if it is on, carries on from
    /// there.
    pub(super) fn set_speed(&mut self, speed: u16) {
        self.practice.passes = 0;
        self.play_at_speed(speed);
    }

    /// Turns the speed trainer on, from its first speed, or off, leaving
    /// the speed where it got to.
    pub(super) fn set_training(&mut self, on: bool) {
        self.practice.training = on;
        self.practice.passes = 0;
        let trainer = self.practice.settings.trainer;
        if on {
            self.play_at_speed(trainer.from);
            let every = match trainer.every {
                1 => tr("each time").to_string(),
                passes => trf("every {} times", &[&passes]),
            };
            let template = if trainer.to < trainer.from {
                "Speed trainer: {}%, then {}% slower {} the loop comes round, down to {}%"
            } else {
                "Speed trainer: {}%, then {}% faster {} the loop comes round, up to {}%"
            };
            self.notice = Some(trf(
                template,
                &[&trainer.from, &trainer.by, &every, &trainer.to],
            ));
        } else {
            self.notice = Some(trf(
                "Speed trainer off: the band stays at {}%",
                &[&self.practice.speed],
            ));
        }
    }

    /// Trains with `trainer`'s speeds from now on. A new first speed is
    /// the speed at once while stopped, so Play starts there.
    pub(super) fn set_trainer(&mut self, trainer: Trainer) {
        let trainer = trainer.clamped();
        let before = std::mem::replace(&mut self.practice.settings.trainer, trainer);
        if self.practice.training && !self.playing && trainer.from != before.from {
            self.practice.passes = 0;
            self.play_at_speed(trainer.from);
        }
    }

    /// Called every frame with the audio thread's laps: moves the speed
    /// on while the trainer is on, as the loop comes round. The audio
    /// thread has the new tempos already (see the top of this file), so
    /// sending them again changes nothing; it covers tempos that came
    /// too late for the lap, or song tempos sent in between.
    pub(super) fn train(&mut self, laps: usize) {
        let Some(speed) = self.practice.came_round(laps) else {
            return;
        };
        self.play_at_speed(speed);
        let target = self.practice.settings.trainer.to;
        self.notice = Some(if speed == target {
            trf("Speed trainer: {}%, the speed you aimed for", &[&speed])
        } else {
            trf("Speed trainer: {}%, on the way to {}%", &[&speed, &target])
        });
    }

    /// Stop, with the trainer on, goes back to its first speed, so the
    /// next run starts slow again. Pause keeps the speed.
    pub(super) fn trainer_stopped(&mut self) {
        if self.practice.training {
            self.practice.passes = 0;
            self.play_at_speed(self.practice.settings.trainer.from);
        }
    }

    /// Plays at `speed` percent of the song's tempo, from the next step.
    fn play_at_speed(&mut self, speed: u16) {
        let speed = speed.clamp(*SPEEDS.start(), *SPEEDS.end());
        if speed == self.practice.speed {
            return;
        }
        self.practice.speed = speed;
        let (tempo, song) = self.tempos_at(speed);
        self.send(Command::SetTempo(tempo));
        self.send(Command::SetSongTempos(song));
    }

    /// The tempo and the song's tempos at `speed` percent, as the audio
    /// thread is sent them.
    fn tempos_at(&self, speed: u16) -> (Tempo, Vec<Tempo>) {
        let factor = f32::from(speed) / 100.0;
        let song = self.project.playback_arrangement();
        let tempos = self
            .project
            .played_tempos(&song)
            .into_iter()
            .map(|tempo| at_speed(tempo, factor))
            .collect();
        (at_speed(self.project.tempo, factor), tempos)
    }

    /// A tap of the Tap button: two or more in a row set the song's
    /// tempo to theirs, as one undo step.
    pub(super) fn tap_tempo(&mut self, now: f64) {
        // The taps follow the beats the metronome clicks, which are not
        // always quarter notes: a dotted quarter in 6/8, an eighth in
        // 7/8 (on average, as its beats are uneven), a half note in 2/2.
        let time = self.project.time_signature;
        let beats = time.beat_starts().len().max(1);
        let quarters = time.steps_per_bar() as f64 / beats as f64 / f64::from(STEPS_PER_BEAT);
        let Some((tempo, carries_on)) = self.practice.tap(now, quarters) else {
            self.notice = Some(tr("Tap again on every beat to set the tempo").to_string());
            return;
        };
        if carries_on {
            self.history.reopen(TAP_STEP);
        }
        self.edit(TAP_STEP, |project| project.scale_tempo(tempo));
        self.notice = Some(trf("Tempo {} BPM, tapped", &[&tempo.bpm()]));
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
        // Whole sentences, as a Spanish article goes with its noun.
        if !you {
            self.notice = Some(tr_string(format!("The band plays the {name} again")));
            return;
        }
        match self.project.player_channel(player) {
            Some(channel) => {
                self.piano_roll.select(channel);
                self.notice = Some(tr_string(format!(
                    "You play the {name}: the band leaves it out, and your keyboard plays \
                     its sound"
                )));
            }
            None => {
                self.notice = Some(tr_string(format!(
                    "You play the {name}: the band leaves it out"
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn practice() -> Practice {
        Practice::with(Settings::default())
    }

    #[test]
    fn the_trainer_moves_on_as_the_loop_comes_round() {
        let mut practice = practice();
        // Off, laps change nothing.
        assert_eq!(practice.came_round(3), None);
        practice.training = true;
        practice.speed = 70;
        assert_eq!(practice.came_round(3), None, "no new lap");
        assert_eq!(practice.came_round(4), Some(75));
        practice.speed = 75;
        // Frames missed while the window was hidden: a pass each.
        assert_eq!(practice.came_round(7), Some(90));
        practice.speed = 90;
        assert_eq!(practice.came_round(20), Some(100), "never past the target");
        practice.speed = 100;
        assert_eq!(practice.came_round(21), None, "the target holds");
        // A new audio thread, after a restart, counts from 0.
        practice.speed = 80;
        assert_eq!(practice.came_round(0), None);
        assert_eq!(practice.came_round(1), Some(85));
    }

    #[test]
    fn the_next_speed_is_asked_for_ahead_of_the_lap_it_starts_on() {
        let mut practice = practice();
        assert_eq!(practice.next_change(), None, "the trainer is off");
        practice.training = true;
        practice.speed = 70;
        assert_eq!(practice.next_change(), Some((70, 75)));
        practice.speed = 100;
        assert_eq!(practice.next_change(), None, "at the target");
        practice.settings.trainer.every = 2;
        practice.speed = 80;
        assert_eq!(practice.next_change(), None, "one more pass at 80%");
        practice.passes = 1;
        assert_eq!(practice.next_change(), Some((80, 85)));
    }

    #[test]
    fn the_trainer_can_play_each_speed_more_than_once() {
        let mut practice = practice();
        practice.settings.trainer.every = 3;
        practice.training = true;
        practice.speed = 70;
        practice.came_round(0);
        assert_eq!(practice.came_round(1), None);
        assert_eq!(practice.came_round(2), None);
        assert_eq!(practice.came_round(3), Some(75));
        practice.speed = 75;
        assert_eq!(practice.came_round(5), None);
        assert_eq!(practice.came_round(9), Some(85));
    }

    #[test]
    fn taps_set_the_tempo_of_their_beats() {
        let mut practice = practice();
        assert_eq!(practice.tap(10.0, 1.0), None, "one tap is not a tempo yet");
        // Half a second apart: 120 BPM.
        assert_eq!(practice.tap(10.5, 1.0), Some((Tempo::new(120.0), false)));
        // A little uneven, as hands are: the average is taken.
        assert_eq!(practice.tap(11.02, 1.0), Some((Tempo::new(118.0), true)));
        assert_eq!(practice.tap(11.5, 1.0), Some((Tempo::new(120.0), true)));
        // After a pause, counting starts again.
        assert_eq!(practice.tap(20.0, 1.0), None);
        assert_eq!(practice.tap(21.0, 1.0), Some((Tempo::new(60.0), false)));
        // In 6/8 the beat is a dotted quarter: 80 beats a minute are 120
        // quarter notes.
        assert_eq!(practice.tap(30.0, 1.5), None);
        assert_eq!(practice.tap(30.75, 1.5), Some((Tempo::new(120.0), false)));
    }

    #[test]
    fn only_the_last_taps_count() {
        let mut practice = practice();
        let mut now = 0.0;
        for _ in 0..10 {
            practice.tap(now, 1.0);
            now += 1.0;
        }
        // Then faster: the slow taps drop out of the average.
        let mut last = None;
        for _ in 0..TAPS {
            last = practice.tap(now, 1.0);
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
