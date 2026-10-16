//! The View menu and the zoom control: how big everything is drawn, and
//! which windows are open.
//!
//! Each part of the screen has its own zoom: the main window (playlist,
//! toolbar and bars), the channel rack and the piano roll. Ctrl and + or -
//! (⌘ on a Mac) zoom the one you clicked last, Ctrl+0 takes it back to
//! 100%, and Ctrl with the mouse wheel zooms the one under the pointer,
//! like in a web browser. The main window's zoom is also in the View menu
//! and in a small control at the right end of the status bar.
//!
//! The main window uses egui's own zoom, which makes everything bigger.
//! The windows on top undo it and apply their own, by scaling the text
//! and spacing of their contents (see [`widgets::zoom_contents`]); what they
//! draw themselves, such as the piano roll's rows, they scale with
//! [`widgets::zoom`].
//!
//! [`widgets::zoom_contents`]: crate::widgets::zoom_contents
//! [`widgets::zoom`]: crate::widgets::zoom

use eframe::egui::gui_zoom::kb_shortcuts::{ZOOM_IN, ZOOM_IN_SECONDARY, ZOOM_OUT, ZOOM_RESET};
use eframe::egui::{self, Key, KeyboardShortcut, Modifiers};

use super::LoomApp;
use crate::piano_roll::PianoRoll;

/// The keys that show and hide the windows and panels, as in other
/// pattern-based studios.
pub(super) const RACK: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F6);
pub(super) const PIANO_ROLL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F7);
pub(super) const HARMONY: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F8);
pub(super) const BROWSER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::F8);

/// The sizes offered in the zoom control's menu, as zoom factors.
const ZOOM_PRESETS: [f32; 7] = [0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 2.0];

/// The smallest and largest zoom.
const ZOOM_RANGE: std::ops::RangeInclusive<f32> = 0.5..=3.0;

/// A part of the screen with its own zoom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum Area {
    #[default]
    Main,
    Rack,
    PianoRoll,
}

impl Area {
    /// The area that contains a layer: one of the windows, or the main
    /// window for everything else.
    fn of(layer: egui::LayerId) -> Area {
        if layer.id == super::rack_id() {
            Area::Rack
        } else if layer.id == PianoRoll::id() {
            Area::PianoRoll
        } else {
            Area::Main
        }
    }

    /// Where a window's zoom is kept. egui saves it with the window
    /// positions, so it is the same the next time the app starts.
    fn key(self) -> egui::Id {
        egui::Id::unique(("zoom", self))
    }

    /// How big this area is drawn, where 1.0 is 100%.
    pub(crate) fn zoom(self, ctx: &egui::Context) -> f32 {
        match self {
            Area::Main => ctx.zoom_factor(),
            _ => ctx.data_mut(|data| *data.get_persisted_mut_or(self.key(), 1.0)),
        }
    }

    /// One step bigger or smaller, within [`ZOOM_RANGE`]. Every way of
    /// zooming goes through here (the keys, the wheel, the View menu and
    /// the status bar's buttons), so they all stop at the same sizes.
    fn step_zoom(self, ctx: &egui::Context, bigger: bool) {
        self.set_zoom(ctx, zoom_step(self.zoom(ctx), bigger));
    }

    fn set_zoom(self, ctx: &egui::Context, zoom: f32) {
        let zoom = zoom.clamp(*ZOOM_RANGE.start(), *ZOOM_RANGE.end());
        match self {
            Area::Main => ctx.set_zoom_factor(zoom),
            _ => ctx.data_mut(|data| data.insert_persisted(self.key(), zoom)),
        }
    }
}

/// One step bigger or smaller: 10% at a time, landing on round numbers.
fn zoom_step(zoom: f32, bigger: bool) -> f32 {
    let tenths = (zoom * 10.0).round();
    let tenths = if bigger { tenths + 1.0 } else { tenths - 1.0 };
    tenths / 10.0
}

/// The zoom factor as a percentage, such as "125%".
fn percent(zoom: f32) -> String {
    format!("{:.0}%", zoom * 100.0)
}

impl LoomApp {
    /// The View menu: zoom, and which windows and panels are shown.
    pub(super) fn view_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("View", |ui| {
            ui.label(format!("Zoom: {}", percent(ui.ctx().zoom_factor())));
            let ctx = ui.ctx().clone();
            let zoom = Area::Main.zoom(&ctx);
            let item = |text, shortcut| {
                egui::Button::new(text).shortcut_text(ctx.format_shortcut(&shortcut))
            };
            if ui
                .add_enabled(zoom < *ZOOM_RANGE.end(), item("Zoom In", ZOOM_IN))
                .clicked()
            {
                Area::Main.step_zoom(&ctx, true);
            }
            if ui
                .add_enabled(zoom > *ZOOM_RANGE.start(), item("Zoom Out", ZOOM_OUT))
                .clicked()
            {
                Area::Main.step_zoom(&ctx, false);
            }
            if ui
                .add_enabled(zoom != 1.0, item("Reset Zoom", ZOOM_RESET))
                .clicked()
            {
                Area::Main.set_zoom(&ctx, 1.0);
            }
            ui.weak(
                "The channel rack and the piano roll have their own zoom:\n\
                 Ctrl+wheel over them, or Ctrl and +/− after clicking in them.",
            );
            for (area, name) in [
                (Area::Rack, "Channel rack"),
                (Area::PianoRoll, "Piano roll"),
            ] {
                let zoom = area.zoom(&ctx);
                let text = format!("{name} at {}: reset to 100%", percent(zoom));
                if ui
                    .add_enabled(zoom != 1.0, egui::Button::new(text))
                    .clicked()
                {
                    area.set_zoom(&ctx, 1.0);
                }
            }
            ui.separator();
            let window = |open: &mut bool, text: &str, shortcut, ui: &mut egui::Ui| {
                let shortcut = ui.ctx().format_shortcut(&shortcut);
                let button = egui::Button::selectable(*open, text).shortcut_text(shortcut);
                if ui.add(button).clicked() {
                    *open = !*open;
                }
            };
            window(&mut self.rack_open, "Channel rack", RACK, ui);
            window(&mut self.piano_roll.open, "Piano roll", PIANO_ROLL, ui);
            window(&mut self.harmony_keys.open, "Harmony keys", HARMONY, ui);
            let browser = &mut self.playlist.browser_open;
            window(browser, "Patterns and library", BROWSER, ui);
        });
    }

    /// Zooms the area the user is pointing at (Ctrl and the mouse wheel)
    /// or last clicked in (Ctrl and + or -, Ctrl+0), and nothing else.
    /// Called at the start of each frame, before anything is drawn.
    pub(super) fn zoom_shortcuts(&mut self, ctx: &egui::Context) {
        let area_at = |pos| ctx.layer_id_at(pos).map_or(Area::Main, Area::of);
        let pressed_at = ctx.input(|input| {
            input
                .pointer
                .any_pressed()
                .then(|| input.pointer.press_origin())
                .flatten()
        });
        if let Some(pos) = pressed_at {
            self.zoom_area = area_at(pos);
        }

        let wheel = ctx.input(|input| input.zoom_delta());
        if wheel != 1.0
            && let Some(pos) = ctx.pointer_hover_pos()
        {
            // One step per notch of the wheel, as with the keys. A
            // touchpad pinch also lands here, a little at a time, so it
            // zooms smoothly.
            let area = area_at(pos);
            let zoom = area.zoom(ctx);
            let notch = (wheel - 1.0).abs() > 0.05;
            let zoom = if notch {
                zoom_step(zoom, wheel > 1.0)
            } else {
                zoom * wheel
            };
            area.set_zoom(ctx, zoom);
        }

        let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
        let area = self.zoom_area;
        if pressed(ZOOM_IN) || pressed(ZOOM_IN_SECONDARY) {
            area.step_zoom(ctx, true);
        }
        if pressed(ZOOM_OUT) {
            area.step_zoom(ctx, false);
        }
        if pressed(ZOOM_RESET) {
            area.set_zoom(ctx, 1.0);
        }
    }

    /// The scale a window's contents are drawn at: its own zoom, with the
    /// main window's zoom undone, since egui applies that to everything.
    pub(super) fn window_scale(ctx: &egui::Context, area: Area) -> f32 {
        area.zoom(ctx) / ctx.zoom_factor()
    }

    /// F6, F7, F8 and Alt+F8 show or hide the windows and panels.
    pub(super) fn view_shortcuts(&mut self, ctx: &egui::Context) {
        let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
        // Alt+F8 is checked first, so it never counts as F8 as well.
        if pressed(BROWSER) {
            self.playlist.browser_open = !self.playlist.browser_open;
        } else if pressed(HARMONY) {
            self.harmony_keys.open = !self.harmony_keys.open;
        }
        if pressed(RACK) {
            self.rack_open = !self.rack_open;
        }
        if pressed(PIANO_ROLL) {
            self.piano_roll.open = !self.piano_roll.open;
        }
    }
}

/// A small zoom control: − and + buttons around the zoom, which opens a
/// list of sizes. Drawn at the right end of the status bar.
pub(super) fn zoom_control(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let zoom = ctx.zoom_factor();
    let small = |text| egui::Button::new(text).small();
    // The status bar lays this out right to left, so + comes first.
    if ui
        .add(small("+"))
        .on_hover_text(format!("Zoom in ({})", ctx.format_shortcut(&ZOOM_IN)))
        .clicked()
    {
        Area::Main.step_zoom(&ctx, true);
    }
    egui::ComboBox::from_id_salt("zoom")
        .width(52.0)
        .selected_text(percent(zoom))
        .show_ui(ui, |ui| {
            for preset in ZOOM_PRESETS {
                let chosen = (zoom - preset).abs() < 0.01;
                if ui.selectable_label(chosen, percent(preset)).clicked() {
                    Area::Main.set_zoom(&ctx, preset);
                }
            }
        })
        .response
        .on_hover_text(format!(
            "How big the main window is drawn. {} goes back to 100%. The channel rack \
             and the piano roll have their own zoom: Ctrl+wheel over them.",
            ctx.format_shortcut(&ZOOM_RESET)
        ));
    if ui
        .add(small("−"))
        .on_hover_text(format!("Zoom out ({})", ctx.format_shortcut(&ZOOM_OUT)))
        .clicked()
    {
        Area::Main.step_zoom(&ctx, false);
    }
    ui.weak("Zoom");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_steps_land_on_tenths() {
        assert_eq!(zoom_step(1.0, true), 1.1);
        assert_eq!(zoom_step(1.0, false), 0.9);
        // A zoom from the wheel, between two steps, snaps to the next one.
        assert_eq!(zoom_step(1.13, true), 1.2);
    }

    #[test]
    fn zoom_shows_as_a_percentage() {
        assert_eq!(percent(1.0), "100%");
        assert_eq!(percent(1.25), "125%");
        assert_eq!(percent(0.9), "90%");
    }

    #[test]
    fn the_presets_grow_and_include_100_percent() {
        assert!(ZOOM_PRESETS.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(ZOOM_PRESETS.contains(&1.0));
        assert!(
            ZOOM_PRESETS
                .iter()
                .all(|preset| ZOOM_RANGE.contains(preset))
        );
    }
}
