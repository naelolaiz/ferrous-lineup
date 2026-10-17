//! The chord sheet's form row: the intro the band makes, how many times
//! it plays the chorus, and whether it plays an ending. See
//! `lineup_core::band::Form`.

use eframe::egui::{self, Ui};
use lineup_core::band::INTRO_BARS;
use lineup_core::{MAX_REPEATS, Project};

use super::SheetAction;
use crate::theme;

/// What the form row asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormAction {
    /// An intro this many bars long, or none with 0.
    Intro(usize),
    /// Play the chorus this many times.
    Choruses(usize),
    /// Play an ending after the last chorus, or not.
    Ending(bool),
}

/// Draws the row, when the song has a band. `round` is which time the
/// band is playing the chorus, out of how many, while the song plays.
pub(super) fn row(
    ui: &mut Ui,
    project: &Project,
    round: Option<(usize, usize)>,
    actions: &mut Vec<SheetAction>,
) {
    let Some(form) = project.form() else {
        return;
    };
    let mut act = |action| actions.push(SheetAction::Form(action));
    let steps_per_bar = project.time_signature.steps_per_bar().max(1);
    let intro_bars = form.intro / steps_per_bar;
    let has_chorus = project.chorus().is_some();
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Form").strong());
        ui.separator();

        let shown = match intro_bars {
            0 => "No intro".to_string(),
            bars => format!("Intro: {bars} bars"),
        };
        ui.add_enabled_ui(has_chorus || intro_bars > 0, |ui| {
            let combo = egui::ComboBox::from_id_salt("chord-sheet-intro")
                .selected_text(shown)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(intro_bars == 0, "No intro").clicked() {
                        act(FormAction::Intro(0));
                    }
                    for bars in INTRO_BARS {
                        let item = ui.selectable_label(intro_bars == bars, format!("{bars} bars"));
                        if item.clicked() {
                            act(FormAction::Intro(bars));
                        }
                    }
                });
            theme::hint(
                combo.response,
                "Bars the band plays before the chorus, with chords that lead into its first \
                 one: a turnaround, ii–V or I–vi–ii–V. They are written on the sheet, so you \
                 can change them; picking a length again writes them anew",
            )
            .on_disabled_hover_text("Write a chord first: the intro leads into it");
        });

        ui.separator();
        ui.label("Choruses");
        let mut choruses = form.choruses;
        let drag = ui.add_enabled(
            has_chorus,
            egui::DragValue::new(&mut choruses).range(1..=MAX_REPEATS),
        );
        if theme::hint(
            drag,
            "How many times the band plays the chorus: every bar written on the sheet after \
             the intro, one time through the tune, as jazz musicians say. The sheet shows it \
             once, between repeat signs. Drag, or double-click to type",
        )
        .changed()
        {
            act(FormAction::Choruses(choruses));
        }
        if let Some((time, times)) = round {
            ui.colored_label(theme::ACCENT, format!("▶ {time} of {times}"));
        }

        ui.separator();
        let ending = ui
            .add_enabled_ui(has_chorus, |ui| {
                theme::light_button(ui, "Ending", form.ending, None)
            })
            .inner;
        if theme::hint(
            ending,
            "Two bars after the last chorus: the band goes to the key's tonic, then strikes \
             it once and holds it, with a crash of the cymbal. Type a chord in them, and the \
             chorus takes that bar in",
        )
        .clicked()
        {
            act(FormAction::Ending(!form.ending));
        }
    });
}
