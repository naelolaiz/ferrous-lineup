//! The View menu and the zoom controls: how big things are drawn, and
//! which windows are open.
//!
//! As in most music programs, the playlist and the piano roll zoom their two
//! directions separately: horizontal zoom makes bars and steps wider, so
//! you see less time in more detail, and vertical zoom makes tracks and
//! piano keys taller. Neither changes the size of the text or the menus.
//!
//! The same keys zoom whatever the pointer is over:
//!
//! - Ctrl and + or − (or the mouse wheel) zooms horizontally, and Alt
//!   with the same keys (or the wheel) vertically, in the playlist, piano
//!   roll or channel rack under the pointer. The step under the pointer stays
//!   under the pointer. Ctrl+0 takes both back to 100%.
//! - Anywhere else (the toolbars, the harmony keys, the status bar, the
//!   Patterns and Library panel) the same keys change the interface
//!   size: how big the text, buttons and menus are everywhere, for big or
//!   very sharp screens. That is egui's own zoom.
//! - The channel rack and the interface have one zoom for both
//!   directions, so Ctrl and Alt do the same there.
//! - With the pointer outside the window, the keys zoom what was clicked
//!   last.
//! - Settings > Appearance has all of these.
//! - The status bar has buttons for the playlist's two zooms.
//!
//! The channel rack draws its contents bigger by scaling their text and
//! spacing (see [`widgets::zoom_contents`]); the playlist and the piano
//! roll read their zoom with [`Area::zoom`] and multiply the sizes of
//! what they draw themselves.
//!
//! [`widgets::zoom_contents`]: crate::widgets::zoom_contents

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers, MouseWheelUnit};

use super::LineupApp;
use crate::piano_roll::PianoRoll;

/// The keys that show and hide the windows and panels, as in other
/// pattern-based studios.
pub(super) const CHORD_SHEET: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F5);
pub(super) const RACK: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F6);
pub(super) const PIANO_ROLL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F7);
pub(super) const HARMONY: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F8);
pub(super) const BROWSER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::F8);
pub(super) const MIXER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F9);
/// Help > Keyboard shortcuts.
pub(super) const HELP: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F1);

/// The zoom keys. `+` is also typed as `=` without Shift on most keyboards.
pub(super) const WIDER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Plus);
pub(super) const WIDER_2: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Equals);
pub(super) const NARROWER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Minus);
pub(super) const TALLER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::Plus);
pub(super) const TALLER_2: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::Equals);
pub(super) const SHORTER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::Minus);
pub(super) const RESET: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Num0);

/// The zooms the keys and buttons step through. Each step is about a
/// fifth bigger than the last, so it takes as many steps to double the
/// size from 100% as from 400%.
const STEPS: [f32; 19] = [
    0.25, 0.33, 0.4, 0.5, 0.6, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0,
    8.0,
];

/// The interface sizes offered in Settings.
const INTERFACE_SIZES: [f32; 11] = [0.5, 0.6, 0.67, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 2.0, 2.5];

/// A part of the screen with its own zoom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum Area {
    /// The playlist, which fills the main window.
    #[default]
    Main,
    Rack,
    PianoRoll,
}

/// Which way a zoom stretches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Axis {
    /// Wider bars and steps: less time on screen, in more detail.
    Horizontal,
    /// Taller tracks and piano keys.
    Vertical,
}

/// What the zoom keys and the wheel zoom: one of the areas, or the whole
/// interface (text, buttons and menus) where no area has a zoom of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Target {
    Area(Area),
    #[default]
    Interface,
}

impl Target {
    /// What is zoomed at a point of the screen: the channel rack or the
    /// piano roll over their windows, the playlist over the playlist, and
    /// the interface over everything else (the toolbars, the harmony keys,
    /// the status bar, the Patterns and Library panel, the row and set
    /// windows, menus).
    fn at(ctx: &egui::Context, pos: egui::Pos2, playlist: egui::Rect) -> Target {
        match ctx.layer_id_at(pos) {
            Some(layer) if layer.id == super::rack_id() => Target::Area(Area::Rack),
            Some(layer) if layer.id == PianoRoll::id() => Target::Area(Area::PianoRoll),
            Some(layer) if layer.order == egui::Order::Background && playlist.contains(pos) => {
                Target::Area(Area::Main)
            }
            _ => Target::Interface,
        }
    }

    /// One step bigger or smaller, one way. The rack and the interface
    /// have one zoom, so both ways change it.
    fn step(self, ctx: &egui::Context, axis: Axis, bigger: bool) {
        match self {
            Target::Area(area) => area.step_zoom(ctx, axis, bigger),
            Target::Interface => {
                ctx.set_zoom_factor(interface_step(ctx.zoom_factor(), bigger));
            }
        }
    }

    /// Keyboard zoom shares its stored values with Settings.
    fn keyboard(self, ctx: &egui::Context) {
        let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
        if pressed(WIDER) || pressed(WIDER_2) {
            self.step(ctx, Axis::Horizontal, true);
        }
        if pressed(NARROWER) {
            self.step(ctx, Axis::Horizontal, false);
        }
        if pressed(TALLER) || pressed(TALLER_2) {
            self.step(ctx, Axis::Vertical, true);
        }
        if pressed(SHORTER) {
            self.step(ctx, Axis::Vertical, false);
        }
        if pressed(RESET) {
            self.reset(ctx);
        }
    }

    /// Multiplies the zoom by `factor`: a touchpad's smooth zoom.
    fn scale(self, ctx: &egui::Context, axis: Axis, factor: f32) {
        match self {
            Target::Area(area) => area.set_zoom(ctx, axis, area.zoom(ctx, axis) * factor),
            Target::Interface => {
                let (low, high) = (
                    INTERFACE_SIZES[0],
                    INTERFACE_SIZES[INTERFACE_SIZES.len() - 1],
                );
                ctx.set_zoom_factor((ctx.zoom_factor() * factor).clamp(low, high));
            }
        }
    }

    fn reset(self, ctx: &egui::Context) {
        match self {
            Target::Area(area) => area.reset(ctx),
            Target::Interface => ctx.set_zoom_factor(1.0),
        }
    }
}

impl Area {
    const ALL: [Area; 3] = [Area::Main, Area::PianoRoll, Area::Rack];

    fn name(self) -> &'static str {
        match self {
            Area::Main => "Playlist",
            Area::Rack => "Channel rack",
            Area::PianoRoll => "Piano roll",
        }
    }

    /// Whether the area zooms its two directions separately. The channel
    /// rack's steps are square buttons, so it zooms both at once.
    fn two_way(self) -> bool {
        self != Area::Rack
    }

    /// Where a zoom is kept. egui saves it with the window positions, so
    /// it is the same the next time the app starts.
    fn key(self, axis: Axis) -> egui::Id {
        let axis = if self.two_way() {
            axis
        } else {
            Axis::Horizontal
        };
        egui::Id::unique(("zoom-2", self, axis))
    }

    /// The smallest and largest zoom. Tracks and piano keys can only get
    /// so short before their names no longer fit.
    fn range(self, axis: Axis) -> (f32, f32) {
        match (self, axis) {
            (Area::Rack, _) => (0.5, 3.0),
            (Area::Main, Axis::Horizontal) => (0.25, 8.0),
            (Area::Main, Axis::Vertical) => (0.6, 3.0),
            (Area::PianoRoll, Axis::Horizontal) => (0.25, 8.0),
            (Area::PianoRoll, Axis::Vertical) => (0.6, 3.0),
        }
    }

    /// How much this area is stretched one way, where 1.0 is 100%.
    pub(crate) fn zoom(self, ctx: &egui::Context, axis: Axis) -> f32 {
        ctx.data_mut(|data| *data.get_persisted_mut_or(self.key(axis), 1.0))
    }

    fn set_zoom(self, ctx: &egui::Context, axis: Axis, zoom: f32) {
        let (low, high) = self.range(axis);
        let zoom = zoom.clamp(low, high);
        ctx.data_mut(|data| data.insert_persisted(self.key(axis), zoom));
    }

    /// One step bigger or smaller. Every way of zooming in steps goes
    /// through here (the keys, the wheel, Settings and the status
    /// bar's buttons), so they all stop at the same sizes.
    fn step_zoom(self, ctx: &egui::Context, axis: Axis, bigger: bool) {
        self.set_zoom(ctx, axis, zoom_step(self.zoom(ctx, axis), bigger));
    }

    fn can_step(self, ctx: &egui::Context, axis: Axis, bigger: bool) -> bool {
        let (low, high) = self.range(axis);
        let zoom = self.zoom(ctx, axis);
        if bigger { zoom < high } else { zoom > low }
    }

    fn reset(self, ctx: &egui::Context) {
        self.set_zoom(ctx, Axis::Horizontal, 1.0);
        self.set_zoom(ctx, Axis::Vertical, 1.0);
    }

    fn is_reset(self, ctx: &egui::Context) -> bool {
        self.zoom(ctx, Axis::Horizontal) == 1.0 && self.zoom(ctx, Axis::Vertical) == 1.0
    }
}

/// The next of the [`STEPS`] above or below `zoom`. A zoom between two
/// steps, from the wheel on a touchpad, goes to the next one.
fn zoom_step(zoom: f32, bigger: bool) -> f32 {
    const CLOSE: f32 = 0.005;
    if bigger {
        STEPS
            .iter()
            .copied()
            .find(|&step| step > zoom + CLOSE)
            .unwrap_or(STEPS[STEPS.len() - 1])
    } else {
        STEPS
            .iter()
            .rev()
            .copied()
            .find(|&step| step < zoom - CLOSE)
            .unwrap_or(STEPS[0])
    }
}

/// The next of the [`INTERFACE_SIZES`] above or below `size`.
fn interface_step(size: f32, bigger: bool) -> f32 {
    const CLOSE: f32 = 0.005;
    let last = INTERFACE_SIZES[INTERFACE_SIZES.len() - 1];
    if bigger {
        INTERFACE_SIZES
            .into_iter()
            .find(|&step| step > size + CLOSE)
            .unwrap_or(last)
    } else {
        INTERFACE_SIZES
            .into_iter()
            .rev()
            .find(|&step| step < size - CLOSE)
            .unwrap_or(INTERFACE_SIZES[0])
    }
}

/// The zoom factor as a percentage, such as "125%".
fn percent(zoom: f32) -> String {
    format!("{:.0}%", zoom * 100.0)
}

/// What the mouse wheel did this frame with a zoom key held: which way a
/// mouse wheel turned, and the smooth movement of a touchpad.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct WheelZoom {
    /// The wheel's notches added up: up is positive.
    notches: f32,
    /// A factor to multiply the zoom by, from a touchpad.
    smooth: f32,
}

impl WheelZoom {
    fn none() -> Self {
        WheelZoom {
            notches: 0.0,
            smooth: 1.0,
        }
    }

    fn add(&mut self, unit: MouseWheelUnit, amount: f32) {
        match unit {
            MouseWheelUnit::Line | MouseWheelUnit::Page => self.notches += amount,
            // A touchpad sends many small movements: 100 points of
            // scrolling is about one step.
            MouseWheelUnit::Point => self.smooth *= (amount * 0.002).exp(),
        }
    }

    /// Zooms one step for a turn of the wheel. Only one step a frame:
    /// on Linux (X11) one notch can arrive as two events, and a frame is
    /// short enough that a quick turn still zooms step by step.
    fn apply(self, target: Target, ctx: &egui::Context, axis: Axis) {
        if self.notches != 0.0 {
            target.step(ctx, axis, self.notches > 0.0);
        }
        if self.smooth != 1.0 {
            target.scale(ctx, axis, self.smooth);
        }
    }
}

impl LineupApp {
    /// The View menu: which windows and panels are shown.
    pub(super) fn view_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("View", |ui| {
            let window = |open: &mut bool, text: &str, shortcut, ui: &mut egui::Ui| {
                let shortcut = ui.ctx().format_shortcut(&shortcut);
                let button = egui::Button::selectable(*open, text).shortcut_text(shortcut);
                if ui.add(button).clicked() {
                    *open = !*open;
                }
            };
            window(&mut self.chord_sheet.open, "Chord sheet", CHORD_SHEET, ui);
            let lead = egui::Button::selectable(self.lead_sheet.open, "Lead sheet");
            if ui.add(lead).clicked() {
                self.lead_sheet.open = !self.lead_sheet.open;
            }
            window(&mut self.rack_open, "Channel rack", RACK, ui);
            if ui
                .button("Style editor…")
                .on_hover_text(
                    "See and edit repeating parts, or create a style from an existing one",
                )
                .clicked()
            {
                if self.style_editor.open {
                    self.bring_to_front = Some(crate::style_editor::window_id());
                } else {
                    self.edit_current_style();
                }
            }
            window(&mut self.piano_roll.open, "Piano roll", PIANO_ROLL, ui);
            window(&mut self.harmony_keys.open, "Harmony keys", HARMONY, ui);
            let browser = &mut self.playlist.browser_open;
            window(browser, "Patterns and library", BROWSER, ui);
            window(&mut self.mixer_open, "Mixer", MIXER, ui);
            ui.separator();
            let ear = egui::Button::selectable(self.ear_training.open, "Ear training")
                .shortcut_text("experimental");
            if ui.add(ear).clicked() {
                self.ear_training.open = !self.ear_training.open;
            }
        });
    }

    /// Zooms what the pointer is over, with the zoom keys, the wheel or a
    /// touchpad pinch: the playlist, the piano roll or the channel rack,
    /// or the whole interface anywhere else. When the pointer is outside
    /// the app's window, the keys zoom what was clicked last.
    /// Called at the start of each frame, before anything is drawn.
    pub(super) fn zoom_shortcuts(&mut self, ctx: &egui::Context) {
        let playlist = self.playlist.area();
        let pressed_at = ctx.input(|input| crate::widgets::press_pos(input, None));
        if let Some(pos) = pressed_at {
            self.zoom_target = Target::at(ctx, pos, playlist);
        }
        let target = match ctx.pointer_hover_pos() {
            Some(pos) => Target::at(ctx, pos, playlist),
            None => self.zoom_target,
        };

        // The wheel, read from its events: egui itself only knows one zoom
        // for both directions.
        let (wide, tall, pinch) = ctx.input(|input| {
            let mut wide = WheelZoom::none();
            let mut tall = WheelZoom::none();
            for event in &input.raw.events {
                if let egui::Event::MouseWheel {
                    unit,
                    delta,
                    modifiers,
                    ..
                } = event
                {
                    // Up or right zooms in. Shift turns a wheel sideways.
                    let amount = if delta.y != 0.0 { delta.y } else { delta.x };
                    if modifiers.alt {
                        tall.add(*unit, amount);
                    } else if modifiers.command {
                        wide.add(*unit, amount);
                    }
                }
            }
            let wheel_used = wide != WheelZoom::none() || tall != WheelZoom::none();
            let pinch = match input.multi_touch() {
                Some(touch) if !wheel_used => touch.zoom_delta,
                _ => 1.0,
            };
            (wide, tall, pinch)
        });
        if ctx.pointer_hover_pos().is_some() {
            wide.apply(target, ctx, Axis::Horizontal);
            tall.apply(target, ctx, Axis::Vertical);
            if pinch != 1.0 {
                target.scale(ctx, Axis::Horizontal, pinch);
            }
        }

        target.keyboard(ctx);
    }

    /// The scale the channel rack's contents are drawn at: its own zoom.
    pub(super) fn rack_scale(ctx: &egui::Context) -> f32 {
        Area::Rack.zoom(ctx, Axis::Horizontal)
    }

    /// F1, F5 to F9 and Alt+F8 show or hide the windows and panels.
    pub(super) fn view_shortcuts(&mut self, ctx: &egui::Context) {
        let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
        // Alt+F8 is checked first, so it never counts as F8 as well.
        if pressed(BROWSER) {
            self.playlist.browser_open = !self.playlist.browser_open;
        } else if pressed(HARMONY) {
            self.harmony_keys.open = !self.harmony_keys.open;
        }
        if pressed(CHORD_SHEET) {
            self.chord_sheet.open = !self.chord_sheet.open;
        }
        if pressed(RACK) {
            self.rack_open = !self.rack_open;
        }
        if pressed(PIANO_ROLL) {
            self.piano_roll.open = !self.piano_roll.open;
        }
        if pressed(MIXER) {
            self.mixer_open = !self.mixer_open;
        }
        if pressed(HELP) {
            self.shortcuts_open = !self.shortcuts_open;
        }
    }
}

/// The settings dialog edits the same persisted values as the shortcuts
/// and status bar, so changes through any of them stay in sync.
pub(super) fn zoom_settings_ui(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    super::settings::card(
        ui,
        "Interface size",
        "Scale text, buttons and menus throughout the app.",
        |ui| {
            egui::ComboBox::from_id_salt("settings-interface-size")
                .width(110.0)
                .selected_text(percent(ctx.zoom_factor()))
                .show_ui(ui, |ui| {
                    for size in INTERFACE_SIZES {
                        if ui
                            .selectable_label(
                                (ctx.zoom_factor() - size).abs() < 0.01,
                                percent(size),
                            )
                            .clicked()
                        {
                            ctx.set_zoom_factor(size);
                        }
                    }
                });
        },
    );
    super::settings::card(
        ui,
        "Editor zoom",
        "Adjust time detail and row height independently.",
        |ui| {
            egui::Grid::new("settings-zoom")
                .spacing(egui::vec2(14.0, 12.0))
                .show(ui, |ui| {
                    ui.weak("Editor");
                    ui.weak("Horizontal");
                    ui.weak("Vertical");
                    ui.end_row();
                    for area in Area::ALL {
                        ui.label(area.name());
                        for axis in [Axis::Horizontal, Axis::Vertical] {
                            if axis == Axis::Vertical && !area.two_way() {
                                ui.weak("Linked").on_hover_text(
                                    "The channel rack uses one scale for both directions.",
                                );
                                continue;
                            }
                            let current = area.zoom(&ctx, axis);
                            egui::ComboBox::from_id_salt(("settings-zoom", area, axis))
                                .width(86.0)
                                .selected_text(percent(current))
                                .show_ui(ui, |ui| {
                                    let (low, high) = area.range(axis);
                                    for step in
                                        STEPS.into_iter().filter(|step| (low..=high).contains(step))
                                    {
                                        if ui
                                            .selectable_label(
                                                (current - step).abs() < 0.01,
                                                percent(step),
                                            )
                                            .clicked()
                                        {
                                            area.set_zoom(&ctx, axis, step);
                                        }
                                    }
                                });
                        }
                        ui.end_row();
                    }
                });
            ui.add_space(6.0);
            let changed =
                ctx.zoom_factor() != 1.0 || Area::ALL.into_iter().any(|area| !area.is_reset(&ctx));
            if ui
                .add_enabled(changed, egui::Button::new("Reset all zoom levels to 100%"))
                .clicked()
            {
                for area in Area::ALL {
                    area.reset(&ctx);
                }
                ctx.set_zoom_factor(1.0);
            }
        },
    );
    super::settings::card(
        ui,
        "Zoom shortcuts",
        "Shortcuts act on the view under the pointer.",
        |ui| {
            egui::Grid::new("settings-zoom-shortcuts")
                .spacing(egui::vec2(20.0, 8.0))
                .show(ui, |ui| {
                    for (keys, action) in [
                        ("Ctrl +/− · Ctrl+wheel", "Horizontal zoom"),
                        ("Alt +/− · Alt+wheel", "Vertical zoom"),
                        ("Ctrl+0", "Reset the current view"),
                    ] {
                        ui.label(keys);
                        ui.weak(action);
                        ui.end_row();
                    }
                });
            ui.weak("Over toolbars and panels, zoom changes the interface size. Use ⌘ in place of Ctrl on Mac.");
        },
    );
}

/// The playlist's zoom at the right end of the status bar: − and +
/// around each of its two zooms, which open a list of sizes.
pub(super) fn zoom_control(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    // The status bar lays this out right to left, so the vertical zoom
    // comes first, and + before −.
    for (axis, symbol, what) in [
        (Axis::Vertical, "↕", "Taller or shorter tracks (Alt+wheel)"),
        (Axis::Horizontal, "↔", "Wider or narrower bars (Ctrl+wheel)"),
    ] {
        let area = Area::Main;
        let zoom = area.zoom(&ctx, axis);
        let small = |text| egui::Button::new(text).small();
        if ui
            .add_enabled(area.can_step(&ctx, axis, true), small("+"))
            .on_hover_text(what)
            .clicked()
        {
            area.step_zoom(&ctx, axis, true);
        }
        egui::ComboBox::from_id_salt(("zoom", axis))
            .width(48.0)
            .selected_text(percent(zoom))
            .show_ui(ui, |ui| {
                let (low, high) = area.range(axis);
                for step in STEPS.into_iter().filter(|step| (low..=high).contains(step)) {
                    let chosen = (zoom - step).abs() < 0.01;
                    if ui.selectable_label(chosen, percent(step)).clicked() {
                        area.set_zoom(&ctx, axis, step);
                    }
                }
            })
            .response
            .on_hover_text(what);
        if ui
            .add_enabled(area.can_step(&ctx, axis, false), small("−"))
            .on_hover_text(what)
            .clicked()
        {
            area.step_zoom(&ctx, axis, false);
        }
        ui.label(symbol).on_hover_text(what);
    }
    ui.weak("Playlist zoom");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_steps_go_up_and_down_the_ladder() {
        assert_eq!(zoom_step(1.0, true), 1.1);
        assert_eq!(zoom_step(1.0, false), 0.9);
        // A zoom from a touchpad, between two steps, goes to the next one.
        assert_eq!(zoom_step(1.13, true), 1.25);
        assert_eq!(zoom_step(1.13, false), 1.1);
        // The ends stay put.
        assert_eq!(zoom_step(8.0, true), 8.0);
        assert_eq!(zoom_step(0.25, false), 0.25);
    }

    #[test]
    fn zoom_shows_as_a_percentage() {
        assert_eq!(percent(1.0), "100%");
        assert_eq!(percent(1.25), "125%");
        assert_eq!(percent(0.9), "90%");
    }

    #[test]
    fn the_steps_grow_and_include_100_percent() {
        assert!(STEPS.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(STEPS.contains(&1.0));
        assert!(INTERFACE_SIZES.contains(&1.0));
        for area in Area::ALL {
            for axis in [Axis::Horizontal, Axis::Vertical] {
                let (low, high) = area.range(axis);
                assert!(STEPS.contains(&low) && STEPS.contains(&high));
            }
        }
    }

    #[test]
    fn each_direction_zooms_on_its_own() {
        let ctx = egui::Context::default();
        Area::Main.step_zoom(&ctx, Axis::Horizontal, true);
        assert_eq!(Area::Main.zoom(&ctx, Axis::Horizontal), 1.1);
        assert_eq!(Area::Main.zoom(&ctx, Axis::Vertical), 1.0);
        Area::PianoRoll.step_zoom(&ctx, Axis::Vertical, false);
        assert_eq!(Area::PianoRoll.zoom(&ctx, Axis::Vertical), 0.9);
        assert_eq!(Area::PianoRoll.zoom(&ctx, Axis::Horizontal), 1.0);
        // The rack's two directions are one.
        Area::Rack.step_zoom(&ctx, Axis::Vertical, true);
        assert_eq!(Area::Rack.zoom(&ctx, Axis::Horizontal), 1.1);
        Area::Main.reset(&ctx);
        assert!(Area::Main.is_reset(&ctx));
    }

    #[test]
    fn settings_and_keyboard_shortcuts_share_the_same_zoom_levels() {
        let ctx = egui::Context::default();
        ctx.options_mut(|options| options.zoom_with_keyboard = false);
        let frame = |shortcut: KeyboardShortcut, target: Target| {
            let mut output = ctx.run_pass(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(900.0, 700.0),
                    )),
                    events: vec![egui::Event::Key {
                        key: shortcut.logical_key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: shortcut.modifiers,
                    }],
                    ..Default::default()
                },
                |ctx| {
                    target.keyboard(ctx);
                    egui::Window::new("Zoom settings").show(ctx, zoom_settings_ui);
                },
            );
            output.textures_delta.clear();
            output
        };
        for area in Area::ALL {
            area.set_zoom(&ctx, Axis::Horizontal, 1.25);
            area.set_zoom(&ctx, Axis::Vertical, 1.25);
            frame(WIDER_2, Target::Area(area));
            assert_eq!(area.zoom(&ctx, Axis::Horizontal), 1.5);
            if area.two_way() {
                assert_eq!(area.zoom(&ctx, Axis::Vertical), 1.25);
            }
            let output = frame(TALLER, Target::Area(area));
            let expected = if area.two_way() { 1.5 } else { 1.75 };
            assert!(output.shapes.iter().any(|shape| matches!(&shape.shape,
                egui::Shape::Text(text) if text.galley.text() == percent(expected)
            )));
            assert_eq!(area.zoom(&ctx, Axis::Vertical), expected);
            frame(RESET, Target::Area(area));
            assert!(area.is_reset(&ctx));
        }
        frame(WIDER, Target::Interface);
        // egui applies interface zoom on the following pass.
        frame(NARROWER, Target::Area(Area::Main));
        assert_eq!(ctx.zoom_factor(), 1.1);
        frame(RESET, Target::Interface);
        frame(NARROWER, Target::Area(Area::Main));
        assert_eq!(ctx.zoom_factor(), 1.0);
    }

    #[test]
    fn the_mouse_wheel_zooms_one_step_a_frame() {
        let ctx = egui::Context::default();
        // One notch that came as two events.
        let mut wheel = WheelZoom::none();
        wheel.add(MouseWheelUnit::Line, 1.0);
        wheel.add(MouseWheelUnit::Line, 1.0);
        wheel.apply(Target::Area(Area::Main), &ctx, Axis::Horizontal);
        assert_eq!(Area::Main.zoom(&ctx, Axis::Horizontal), 1.1);
        let mut wheel = WheelZoom::none();
        wheel.add(MouseWheelUnit::Line, -1.0);
        wheel.apply(Target::Area(Area::Main), &ctx, Axis::Horizontal);
        assert_eq!(Area::Main.zoom(&ctx, Axis::Horizontal), 1.0);
    }

    #[test]
    fn the_interface_steps_through_its_sizes() {
        assert_eq!(interface_step(1.0, true), 1.1);
        assert_eq!(interface_step(1.0, false), 0.9);
        assert_eq!(interface_step(1.17, true), 1.25);
        assert_eq!(interface_step(2.5, true), 2.5);
        assert_eq!(interface_step(0.75, false), 0.67);
        assert_eq!(interface_step(0.5, false), 0.5);
    }
}
