//! The chord sheet's practice row: the count-in, the metronome, the
//! practice speed and its trainer, tap tempo, the band's parts you can
//! play yourself, and looping the bars selected on the sheet.

use std::ops::RangeInclusive;

use eframe::egui::{self, Ui};
use lineup_core::{LoopRegion, Player, Project};
use serde::{Deserialize, Serialize};

use super::SheetAction;
use crate::theme;

/// The slowest and the fastest practice speeds, in percent.
pub const SPEEDS: RangeInclusive<u16> = 25..=200;

/// How far the speed trainer can move the speed at a time, in percent.
pub const TRAINER_STEPS: RangeInclusive<u16> = 1..=25;

/// How many passes the speed trainer can play at each speed.
pub const TRAINER_PASSES: RangeInclusive<u16> = 1..=16;

/// The practice settings, for the row to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PracticeView {
    pub count_in: bool,
    pub metronome: bool,
    /// The practice speed, in percent of the song's tempo.
    pub speed: u16,
    /// Whether the speed trainer is on, and how it moves the speed.
    pub training: bool,
    pub trainer: Trainer,
}

/// The speed trainer's settings. Playing starts at `from` percent of
/// the song's tempo, and every `every` times the loop comes round the
/// speed moves `by` percent nearer `to`, where it stays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Trainer {
    pub from: u16,
    pub to: u16,
    pub by: u16,
    pub every: u16,
}

impl Default for Trainer {
    /// From 70% up to the song's own tempo, 5% faster each pass: the
    /// seventh pass plays at full speed.
    fn default() -> Self {
        Trainer {
            from: 70,
            to: 100,
            by: 5,
            every: 1,
        }
    }
}

impl Trainer {
    /// The speed that comes after `speed`: `by` nearer the target, and
    /// never past it. A target slower than the start slows down.
    pub fn next(self, speed: u16) -> u16 {
        let by = self.by.max(1);
        if speed < self.to {
            speed.saturating_add(by).min(self.to)
        } else {
            speed.saturating_sub(by).max(self.to)
        }
    }

    /// The same settings, each kept to what it can be.
    pub fn clamped(self) -> Self {
        let clamp =
            |value: u16, range: &RangeInclusive<u16>| value.clamp(*range.start(), *range.end());
        Trainer {
            from: clamp(self.from, &SPEEDS),
            to: clamp(self.to, &SPEEDS),
            by: clamp(self.by, &TRAINER_STEPS),
            every: clamp(self.every, &TRAINER_PASSES),
        }
    }
}

/// What the practice row asks for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PracticeAction {
    CountIn(bool),
    Metronome(bool),
    Speed(u16),
    /// Turn the speed trainer on (`true`) or off.
    Training(bool),
    /// Train with these settings from now on.
    Trainer(Trainer),
    /// The Tap button was clicked, at this time on egui's clock.
    Tap(f64),
    /// Play this part yourself (`true`), or give it back to the band.
    YouPlay(Player, bool),
    /// Loop these steps of the song, or play it all again.
    Loop(Option<LoopRegion>),
}

/// Draws the row. `selected` is the first and last bar selected on the
/// sheet, if any.
pub(super) fn row(
    ui: &mut Ui,
    project: &Project,
    practice: PracticeView,
    selected: Option<(usize, usize)>,
    actions: &mut Vec<SheetAction>,
) {
    let mut act = |action| actions.push(SheetAction::Practice(action));
    ui.horizontal_wrapped(|ui| {
        let count_in =
            theme::light_button(ui, crate::i18n::tr("Count-in"), practice.count_in, None);
        if theme::hint(
            count_in,
            "A bar of clicks before the music starts, so you come in on time",
        )
        .clicked()
        {
            act(PracticeAction::CountIn(!practice.count_in));
        }
        let metronome =
            theme::light_button(ui, crate::i18n::tr("Metronome"), practice.metronome, None);
        if theme::hint(metronome, "A click on every beat while the music plays").clicked() {
            act(PracticeAction::Metronome(!practice.metronome));
        }

        ui.separator();
        ui.label(crate::i18n::tr("Speed"));
        let mut speed = practice.speed;
        let drag = ui.add(egui::DragValue::new(&mut speed).range(SPEEDS).suffix("%"));
        if theme::hint(
            drag,
            "Practise slower (or faster): the band plays at this percentage of the song's \
             tempo. The song keeps its own tempo, in saved files and WAV exports too. Drag, \
             or double-click to type",
        )
        .changed()
        {
            act(PracticeAction::Speed(speed));
        }
        let tap = ui.button(crate::i18n::tr("Tap"));
        if theme::hint(
            tap,
            "Click on every beat, a few times, to set the song's tempo to yours",
        )
        .clicked()
        {
            act(PracticeAction::Tap(ui.input(|input| input.time)));
        }
        let trainer = theme::light_button(
            ui,
            crate::i18n::tr("Speed trainer"),
            practice.training,
            None,
        );
        let hint = if practice.training {
            "The speed goes up each time the loop comes round, until it reaches the \
             target. Stop starts again from the first speed; Pause carries on. Click to \
             turn the trainer off and keep the speed it got to"
        } else {
            "Start slow and get faster: each time the loop comes round (the bars you \
             loop, the whole song, or the pattern in pattern mode), the band plays a \
             little faster, up to the speed you aim for"
        };
        if theme::hint(trainer, hint).clicked() {
            act(PracticeAction::Training(!practice.training));
        }
        if practice.training
            && let Some(trainer) = trainer_controls(ui, practice.trainer)
        {
            act(PracticeAction::Trainer(trainer));
        }

        if let Some(band) = project.band() {
            ui.separator();
            ui.label(crate::i18n::tr("You play"));
            // The style's players, and any other that still has a part:
            // one you edited before changing the style.
            let players = Player::ALL.into_iter().filter(|&player| {
                band.players().contains(&player) || project.player_track(player).is_some()
            });
            for player in players {
                let has_part = project.player_track(player).is_some();
                let you = project.you_play(player);
                let button = ui.add_enabled(has_part, egui::Button::selectable(you, player.name()));
                let name = player.name().to_lowercase();
                let hint = match (has_part, you) {
                    (false, _) => format!("The band has no {name} part yet: write some chords"),
                    (true, false) => format!(
                        "Leave the {name} out of the band, to play it yourself{}",
                        match player {
                            Player::Drums => String::new(),
                            _ => format!(": your keyboard plays the {name}'s sound"),
                        }
                    ),
                    (true, true) => format!("Have the band play the {name} again"),
                };
                if theme::hint(button, hint).clicked() {
                    act(PracticeAction::YouPlay(player, !you));
                }
            }
        }

        ui.separator();
        loop_buttons(ui, project, selected, &mut act);
    });
}

/// The speed trainer's settings, as a sentence: "from 70% to 100% by 5%
/// every 1 pass". Returns the settings when one of them was changed.
pub fn trainer_controls(ui: &mut Ui, trainer: Trainer) -> Option<Trainer> {
    let mut changed = trainer;
    ui.label(crate::i18n::tr("from"));
    let from = ui.add(
        egui::DragValue::new(&mut changed.from)
            .range(SPEEDS)
            .suffix("%"),
    );
    theme::hint(
        from,
        "The speed Play starts at, in percent of the song's tempo",
    );
    ui.label(crate::i18n::tr("to"));
    let to = ui.add(
        egui::DragValue::new(&mut changed.to)
            .range(SPEEDS)
            .suffix("%"),
    );
    theme::hint(
        to,
        "The speed to get to: 100% is the song's own tempo. Once there, the speed stays",
    );
    ui.label(crate::i18n::tr("by"));
    let by = ui.add(
        egui::DragValue::new(&mut changed.by)
            .range(TRAINER_STEPS)
            .suffix("%"),
    );
    theme::hint(by, "How much faster each new speed is");
    ui.label(crate::i18n::tr("every"));
    let passes = if changed.every == 1 {
        format!(" {}", crate::i18n::tr("pass"))
    } else {
        format!(" {}", crate::i18n::tr("passes"))
    };
    let every = ui.add(
        egui::DragValue::new(&mut changed.every)
            .range(TRAINER_PASSES)
            .suffix(passes),
    );
    theme::hint(
        every,
        "How many times the loop plays at each speed before the next one",
    );
    (changed != trainer).then_some(changed)
}

/// "Loop bars 3–6" for the bars selected, and, while a part of the song
/// loops, the button that plays it all again.
fn loop_buttons(
    ui: &mut Ui,
    project: &Project,
    selected: Option<(usize, usize)>,
    act: &mut impl FnMut(PracticeAction),
) {
    let meter = project.meter();
    let looping = project.playlist().loop_region();
    // Bars after the end of the song have nothing to loop.
    let end = project.song_length();
    let wanted = selected
        .map(|(first, last)| (meter.bar(first).start, meter.bar(last).end().min(end)))
        .filter(|&(start, end)| start < end)
        .map(|(start, end)| LoopRegion::new(start, end));
    let text = match selected {
        Some((first, last)) if first == last => format!(
            "🔁 {} {} {}",
            crate::i18n::tr("Loop"),
            crate::i18n::tr("bar"),
            first + 1
        ),
        Some((first, last)) => format!(
            "🔁 {} {} {}–{}",
            crate::i18n::tr("Loop"),
            crate::i18n::tr("bars"),
            first + 1,
            last + 1
        ),
        None => crate::i18n::tr("🔁 Loop").to_string(),
    };
    let on = wanted.is_some() && wanted == looping;
    let button = ui.add_enabled(wanted.is_some(), egui::Button::selectable(on, text));
    let hint = match (wanted, selected) {
        (Some(_), _) => {
            "Play these bars over and over, in song mode. Shift+click a bar (or Shift and \
             an arrow) to select more"
        }
        (None, Some(_)) => "The song does not reach these bars: there is nothing to loop",
        (None, None) => "Click a bar, then Shift+click another, to choose the bars to loop",
    };
    if theme::hint(button, hint).clicked() && !on {
        act(PracticeAction::Loop(wanted));
    }
    if let Some(region) = looping {
        let first = meter.bar_at(region.start).number + 1;
        let last = meter.bar_at(region.end.saturating_sub(1)).number + 1;
        let text = egui::RichText::new("✖ Stop looping").color(theme::LOOP);
        let stop = ui.button(text);
        let hint = format!("Bars {first}–{last} loop. Click to play the whole song again");
        if theme::hint(stop, hint).clicked() {
            act(PracticeAction::Loop(None));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_trainer_moves_towards_its_target_and_stays_there() {
        let trainer = Trainer::default();
        let speeds: Vec<u16> = std::iter::successors(Some(trainer.from), |&speed| {
            Some(trainer.next(speed)).filter(|&next| next != speed)
        })
        .collect();
        assert_eq!(speeds, [70, 75, 80, 85, 90, 95, 100]);
        // A step that does not divide the way stops at the target.
        let uneven = Trainer { by: 8, ..trainer };
        assert_eq!(uneven.next(96), 100);
        assert_eq!(uneven.next(100), 100);
        // Past the target (the speed was changed by hand), it comes back.
        assert_eq!(trainer.next(110), 105);
        // A slower target slows down.
        let slower = Trainer {
            from: 100,
            to: 60,
            by: 15,
            every: 1,
        };
        assert_eq!(slower.next(100), 85);
        assert_eq!(slower.next(70), 60);
    }

    #[test]
    fn trainer_settings_are_kept_in_range() {
        let wild = Trainer {
            from: 0,
            to: 1000,
            by: 0,
            every: 99,
        };
        assert_eq!(
            wild.clamped(),
            Trainer {
                from: *SPEEDS.start(),
                to: *SPEEDS.end(),
                by: *TRAINER_STEPS.start(),
                every: *TRAINER_PASSES.end(),
            }
        );
    }
}
