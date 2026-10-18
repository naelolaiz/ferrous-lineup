//! Ferrous Lineup: an open source music studio with a backing band.
//!
//! `main` only sets up the window. Everything you see is drawn by
//! [`app::LineupApp`].
//!
//! Run `ferrous-lineup song.lineup` to open a project file straight away.

// On Windows, a program is either a "console" program, which opens a
// terminal window, or a "windows" program, which does not. Release builds
// are the second kind; debug builds keep the console, so you can see
// `println!` and panic messages while developing. `cfg_attr` applies the
// attribute only when its condition is true.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod channel_rack;
mod chord_sheet;
mod clipboard;
mod ear_training;
mod harmony_keys;
mod lead_sheet;
mod library_browser;
mod micromodes;
mod midi;
mod piano_roll;
mod playlist;
mod style_editor;
mod theme;
mod time_picker;
mod twelve_tone;
mod widgets;

use std::path::PathBuf;

fn main() -> eframe::Result {
    // The first command-line argument, if there is one, is a file to open.
    // `args_os` keeps file names exactly as the system gave them, even if
    // they are not valid UTF-8.
    let path = std::env::args_os().nth(1).map(PathBuf::from);

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title(app::APP_NAME)
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        app::APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc, path)))),
    )
}
