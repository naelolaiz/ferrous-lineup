//! Which windows are open, kept from one run to the next: the app opens
//! with the windows that were open when it was last closed. The first
//! time, it opens with the chord sheet over the playlist.

use eframe::egui;
use serde::{Deserialize, Serialize};

use super::{LineupApp, preferences};

/// The name they are kept under (see [`preferences`]).
pub(super) const NAME: &str = "open-windows";

/// The windows and panels of the View menu. The style editor and the
/// dialogs are left out: each opens for something, a style or a
/// question, that the next run does not have.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct OpenWindows {
    chord_sheet: bool,
    lead_sheet: bool,
    rack: bool,
    piano_roll: bool,
    harmony_keys: bool,
    /// The Patterns and Library panel.
    browser: bool,
    mixer: bool,
    ear_training: bool,
}

/// The first time: the chord sheet, the harmony keys and the Library,
/// over the playlist. A window added to the list later starts as it does
/// here.
impl Default for OpenWindows {
    fn default() -> Self {
        OpenWindows {
            chord_sheet: true,
            lead_sheet: false,
            rack: false,
            piano_roll: false,
            harmony_keys: true,
            browser: true,
            mixer: false,
            ear_training: false,
        }
    }
}

impl LineupApp {
    /// Which windows are open now.
    pub(super) fn open_windows(&self) -> OpenWindows {
        OpenWindows {
            chord_sheet: self.chord_sheet.open,
            lead_sheet: self.lead_sheet.open,
            rack: self.rack_open,
            piano_roll: self.piano_roll.open,
            harmony_keys: self.harmony_keys.open,
            browser: self.playlist.browser_open,
            mixer: self.mixer_open,
            ear_training: self.ear_training.open,
        }
    }

    /// Opens the windows that were open when the app was last closed,
    /// or those of the first time.
    pub(super) fn restore_windows(&mut self, ctx: &egui::Context) {
        let windows: OpenWindows = preferences::get(ctx, NAME).unwrap_or_default();
        self.chord_sheet.open = windows.chord_sheet;
        self.lead_sheet.open = windows.lead_sheet;
        self.rack_open = windows.rack;
        self.piano_roll.open = windows.piano_roll;
        self.harmony_keys.open = windows.harmony_keys;
        self.playlist.browser_open = windows.browser;
        self.mixer_open = windows.mixer;
        self.ear_training.open = windows.ear_training;
    }
}
