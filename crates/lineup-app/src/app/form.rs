//! The band's song form, from the chord sheet's form row: the intro,
//! how many times the chorus plays, and the ending; and from a bar's
//! menu, which variation of the style the band plays there. The work is
//! done by `lineup_core::band::Form` and the project's `set_intro`,
//! `set_choruses`, `set_ending` and `set_variation`.

use lineup_core::band::Variation;

use super::LineupApp;
use crate::chord_sheet::FormAction;

/// The undo step of the number of choruses. Changes in a row, as the
/// number is dragged, join one step.
const CHORUSES_STEP: &str = "choruses";

impl LineupApp {
    /// Does what the chord sheet's form row asks.
    pub(super) fn apply_form(&mut self, action: FormAction) {
        match action {
            FormAction::Intro(bars) => self.set_intro(bars),
            FormAction::Choruses(choruses) => {
                self.history.reopen(CHORUSES_STEP);
                self.edit(CHORUSES_STEP, |project| project.set_choruses(choruses));
            }
            FormAction::Ending(on) => {
                self.edit("ending", |project| project.set_ending(on));
                self.notice = Some(match (on, self.project.ending()) {
                    (true, Some(ending)) => format!(
                        "The band ends the song on {}, struck once and held",
                        ending.harmony
                    ),
                    (true, None) => "The band plays an ending once there are chords".to_string(),
                    (false, _) => "The band stops at the end of the last chorus".to_string(),
                });
            }
            FormAction::Variation {
                start,
                end,
                variation,
            } => {
                self.edit("variation", |project| {
                    project.set_variation(start, end, variation)
                });
                let meter = self.project.meter();
                let first = meter.bar_at(start).number + 1;
                let bars = match end {
                    Some(end) => format!(
                        "Bars {first}–{}: the band",
                        meter.bar_at(end - 1).number + 1
                    ),
                    None => format!("From bar {first} on, the band"),
                };
                self.notice = Some(match variation {
                    Variation::A => format!("{bars} plays the style as it is"),
                    Variation::B => {
                        format!("{bars} plays the style fuller, with a fill before it")
                    }
                });
            }
        }
    }

    fn set_intro(&mut self, bars: usize) {
        let label = if bars == 0 {
            "remove the intro"
        } else {
            "intro"
        };
        if !self.edit(label, |project| project.set_intro(bars)) {
            self.message =
                Some("There is no room for that intro: the song would be too long.".to_string());
            return;
        }
        // The chorus moved; play from the top, intro and all.
        self.seek(0);
        let into = self
            .project
            .chorus()
            .and_then(|(start, _)| self.project.chord_at(start));
        self.notice = Some(match (bars, into) {
            (0, _) => "No intro: the band starts with the chorus".to_string(),
            (bars, Some(marker)) => {
                format!("A {bars}-bar intro that leads into {}", marker.harmony)
            }
            (bars, None) => format!("A {bars}-bar intro"),
        });
    }
}
