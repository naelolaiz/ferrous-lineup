//! How the chord sheet lays out its bars: how many bars a line holds,
//! and whether the sheet fills the main window in the playlist's place.
//!
//! A line holds as many bars as you choose, or, by default, as many as
//! fit: every harmony's name and the words under it at their full size,
//! never shrunk. The count is kept to 1, 2, 4, 8, 12 or 16, so a line
//! starts on every fourth bar where printed music puts its phrases.

use eframe::egui::{self, Ui};
use serde::{Deserialize, Serialize};

use crate::theme;

fn choices_id() -> egui::Id {
    egui::Id::unique("chord-sheet-layout")
}

/// The most bars a line can hold.
pub const MOST_BARS: usize = 16;

/// The counts of bars a line holds when they fit, from the most: each
/// divides a four-bar phrase or is made of them.
const FITS: [usize; 6] = [16, 12, 8, 4, 2, 1];

/// No bar is made narrower than this to fit more on a line, however
/// short its names.
const NARROWEST_BAR: f32 = 110.0;

/// How many bars a line holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Columns {
    /// As many as fit with every name at its full size.
    #[default]
    Fit,
    /// This many, from 1 to [`MOST_BARS`]: names that don't fit shrink.
    Bars(usize),
}

impl Columns {
    /// How many bars a line holds when it is `width` wide and the widest
    /// bar needs `needed` to show its names in full.
    pub fn count(self, width: f32, needed: f32) -> usize {
        match self {
            Columns::Bars(bars) => bars.clamp(1, MOST_BARS),
            Columns::Fit => {
                let bar = needed.max(NARROWEST_BAR);
                FITS.into_iter()
                    .find(|&bars| width / bars as f32 >= bar)
                    .unwrap_or(1)
            }
        }
    }
}

/// What the sheet remembers from one run of the app to the next.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    columns: Columns,
    /// Whether the sheet takes the playlist's place in the main window.
    fills: bool,
}

/// The layout choices and the "Bars per line" control.
pub struct LayoutView {
    choices: Choices,
    saved: Option<Choices>,
    /// How many bars the lines held when last drawn, to say so.
    pub shown: usize,
}

impl LayoutView {
    pub fn new() -> Self {
        LayoutView {
            choices: Choices::default(),
            saved: None,
            shown: 4,
        }
    }

    /// Reads back the choices of the last run, the first time; then
    /// saves them for the next when they change.
    pub fn remember(&mut self, ctx: &egui::Context) {
        match self.saved {
            None => {
                self.choices = ctx
                    .data_mut(|data| data.get_persisted(choices_id()))
                    .unwrap_or_default();
                self.saved = Some(self.choices);
            }
            Some(saved) if saved != self.choices => {
                let choices = self.choices;
                ctx.data_mut(|data| data.insert_persisted(choices_id(), choices));
                self.saved = Some(choices);
            }
            Some(_) => {}
        }
    }

    pub fn columns(&self) -> Columns {
        self.choices.columns
    }

    /// Whether the sheet fills the main window, in the playlist's place.
    pub fn fills(&self) -> bool {
        self.choices.fills
    }

    pub fn set_fills(&mut self, fills: bool) {
        self.choices.fills = fills;
    }

    /// The "Bars per line" choice and the button that moves the sheet
    /// into the main window or back out of it.
    pub fn items(&mut self, ui: &mut Ui) {
        ui.label("Bars per line");
        let shown = |columns: Columns| match columns {
            Columns::Fit => "Fit the names".to_string(),
            Columns::Bars(bars) => bars.to_string(),
        };
        let selected = match self.choices.columns {
            Columns::Fit => format!("Fit ({})", self.shown),
            bars => shown(bars),
        };
        // Tall enough for every choice, with no scrolling.
        let combo = egui::ComboBox::from_id_salt("chord-sheet-columns")
            .selected_text(selected)
            .height(f32::INFINITY)
            .show_ui(ui, |ui| {
                let fit = ui.selectable_value(
                    &mut self.choices.columns,
                    Columns::Fit,
                    shown(Columns::Fit),
                );
                theme::hint(
                    fit,
                    "As many bars as fit with every name in full at its normal size: 1, 2, 4, \
                     8, 12 or 16, so lines start where four-bar phrases do",
                );
                for bars in 1..=MOST_BARS {
                    let choice = Columns::Bars(bars);
                    ui.selectable_value(&mut self.choices.columns, choice, shown(choice));
                }
            });
        theme::hint(
            combo.response,
            "How many bars a line of the sheet holds. With a number, names that don't fit \
             are written smaller",
        );
        ui.separator();
        let (text, hint) = match self.choices.fills {
            false => (
                "⛶ Fill the window",
                "Show the sheet in the main window, in the playlist's place, with room for \
                 more bars on each line",
            ),
            true => (
                "Back to the playlist",
                "Show the playlist in the main window again, and the sheet over it",
            ),
        };
        if theme::hint(ui.button(text), hint).clicked() {
            self.choices.fills = !self.choices.fills;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_fit_as_many_whole_phrases_as_there_is_room_for() {
        // Short names: the narrowest bar decides.
        assert_eq!(Columns::Fit.count(760.0, 60.0), 4);
        assert_eq!(Columns::Fit.count(1300.0, 60.0), 8);
        assert_eq!(Columns::Fit.count(1800.0, 60.0), 16);
        // Long names take more room, and whole phrases still start lines:
        // room for three bars makes lines of two.
        assert_eq!(Columns::Fit.count(760.0, 240.0), 2);
        assert_eq!(Columns::Fit.count(300.0, 400.0), 1);
        // A number chosen is kept, whatever fits.
        assert_eq!(Columns::Bars(3).count(300.0, 400.0), 3);
        assert_eq!(Columns::Bars(40).count(300.0, 400.0), MOST_BARS);
    }
}
