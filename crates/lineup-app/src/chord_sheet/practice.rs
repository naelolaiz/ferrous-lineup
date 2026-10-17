//! The chord sheet's practice row: the count-in, the metronome, the
//! practice speed, tap tempo, the band's parts you can play yourself, and
//! looping the bars selected on the sheet.

use eframe::egui::{self, Ui};
use lineup_core::{LoopRegion, Player, Project};

use super::SheetAction;
use crate::theme;

/// The slowest and the fastest practice speeds, in percent.
pub const SPEEDS: std::ops::RangeInclusive<u16> = 25..=200;

/// The practice settings, for the row to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PracticeView {
    pub count_in: bool,
    pub metronome: bool,
    /// The practice speed, in percent of the song's tempo.
    pub speed: u16,
}

/// What the practice row asks for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PracticeAction {
    CountIn(bool),
    Metronome(bool),
    Speed(u16),
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
        let count_in = theme::light_button(ui, "Count-in", practice.count_in, None);
        if theme::hint(
            count_in,
            "A bar of clicks before the music starts, so you come in on time",
        )
        .clicked()
        {
            act(PracticeAction::CountIn(!practice.count_in));
        }
        let metronome = theme::light_button(ui, "Metronome", practice.metronome, None);
        if theme::hint(metronome, "A click on every beat while the music plays").clicked() {
            act(PracticeAction::Metronome(!practice.metronome));
        }

        ui.separator();
        ui.label("Speed");
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
        let tap = ui.button("Tap");
        if theme::hint(
            tap,
            "Click on every beat, a few times, to set the song's tempo to yours",
        )
        .clicked()
        {
            act(PracticeAction::Tap(ui.input(|input| input.time)));
        }

        if let Some(band) = project.band() {
            ui.separator();
            ui.label("You play");
            // The style's players, and any other that still has a part:
            // one you edited before changing the style.
            let players = Player::ALL.into_iter().filter(|&player| {
                band.style.players().contains(&player) || project.player_track(player).is_some()
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
        Some((first, last)) if first == last => format!("🔁 Loop bar {}", first + 1),
        Some((first, last)) => format!("🔁 Loop bars {}–{}", first + 1, last + 1),
        None => "🔁 Loop".to_string(),
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
