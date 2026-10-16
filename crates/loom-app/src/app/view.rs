//! The View menu and the zoom control: how big everything is drawn, and
//! which windows are open.
//!
//! egui already zooms the whole window with Ctrl and + or - (⌘ on a Mac),
//! and Ctrl+0 goes back to 100%, like a web browser. This module puts the
//! same zoom where you can see it: in the View menu, and as a small
//! control at the right end of the status bar.

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers, gui_zoom};

use super::LoomApp;

/// The keys that show and hide the windows and panels, as in other
/// pattern-based studios.
pub(super) const RACK: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F6);
pub(super) const PIANO_ROLL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F7);
pub(super) const HARMONY: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F8);
pub(super) const BROWSER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::F8);

/// The sizes offered in the zoom control's menu, as zoom factors.
const ZOOM_PRESETS: [f32; 7] = [0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 2.0];

/// The zoom factor as a percentage, such as "125%".
fn percent(zoom: f32) -> String {
    format!("{:.0}%", zoom * 100.0)
}

impl LoomApp {
    /// The View menu: zoom, and which windows and panels are shown.
    pub(super) fn view_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("View", |ui| {
            ui.label(format!("Zoom: {}", percent(ui.ctx().zoom_factor())));
            // egui's own zoom buttons, with their shortcuts written beside them.
            gui_zoom::zoom_menu_buttons(ui);
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
        .on_hover_text(format!(
            "Zoom in ({})",
            ctx.format_shortcut(&gui_zoom::kb_shortcuts::ZOOM_IN)
        ))
        .clicked()
    {
        gui_zoom::zoom_in(&ctx);
    }
    egui::ComboBox::from_id_salt("zoom")
        .width(52.0)
        .selected_text(percent(zoom))
        .show_ui(ui, |ui| {
            for preset in ZOOM_PRESETS {
                let chosen = (zoom - preset).abs() < 0.01;
                if ui.selectable_label(chosen, percent(preset)).clicked() {
                    ctx.set_zoom_factor(preset);
                }
            }
        })
        .response
        .on_hover_text(format!(
            "How big everything is drawn. {} goes back to 100%.",
            ctx.format_shortcut(&gui_zoom::kb_shortcuts::ZOOM_RESET)
        ));
    if ui
        .add(small("−"))
        .on_hover_text(format!(
            "Zoom out ({})",
            ctx.format_shortcut(&gui_zoom::kb_shortcuts::ZOOM_OUT)
        ))
        .clicked()
    {
        gui_zoom::zoom_out(&ctx);
    }
    ui.weak("Zoom");
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}
