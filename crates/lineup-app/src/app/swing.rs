//! Swing, and the amount each style was last played with.
//!
//! The swing is part of the song (see `lineup_core::swing`): it is saved
//! with it, and undo takes it back. Picking a style or a rhythm gives the
//! song that style's swing: its own, or the amount it was last set to,
//! which the app remembers between runs, as a setting. So a shuffle you
//! like a little lighter stays that way in every song you play it in.

use std::collections::BTreeMap;

use eframe::egui;
use serde::{Deserialize, Serialize};

use super::LineupApp;

/// Where egui keeps the amounts between runs.
fn settings_id() -> egui::Id {
    egui::Id::unique("swing-by-style")
}

/// The swing last set in each style, by the style's name.
#[derive(Debug, Default)]
pub(super) struct SwingMemory {
    amounts: BTreeMap<String, u8>,
    /// Whether an amount changed since they were last stored.
    changed: bool,
}

impl SwingMemory {
    pub(super) fn new(ctx: &egui::Context) -> Self {
        let amounts = ctx
            .data_mut(|data| data.get_persisted::<Remembered>(settings_id()))
            .unwrap_or_default()
            .0;
        SwingMemory {
            amounts,
            changed: false,
        }
    }

    /// The swing style `name` was last set to, if it ever was.
    pub(super) fn of(&self, name: &str) -> Option<u8> {
        self.amounts.get(name).copied()
    }
}

/// What is stored: a newtype, so it has a type of its own in egui's
/// store.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Remembered(BTreeMap<String, u8>);

impl LineupApp {
    /// Sets the song's swing, and remembers it for the style that plays:
    /// the band's, or else the rhythm the library shows.
    pub(super) fn set_swing(&mut self, swing: u8) {
        self.edit("change swing", |project| project.swing = swing);
        let style = match self.project.band() {
            Some(band) => Some(band.style.name),
            None => self.tried().rhythm.map(|rhythm| rhythm.name),
        };
        if let Some(style) = style {
            self.swings.amounts.insert(style.to_string(), swing);
            self.swings.changed = true;
        }
    }

    /// Once per frame: stores the amounts if one changed.
    pub(super) fn store_swings(&mut self, ctx: &egui::Context) {
        if self.swings.changed {
            let remembered = Remembered(self.swings.amounts.clone());
            ctx.data_mut(|data| data.insert_persisted(settings_id(), remembered));
            self.swings.changed = false;
        }
    }
}
