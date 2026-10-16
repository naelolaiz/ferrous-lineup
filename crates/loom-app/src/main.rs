//! Ferrous Loom: an open source pattern-based music studio.
//!
//! `main` only sets up the window. Everything you see is drawn by
//! [`app::LoomApp`].
//!
//! Run `ferrous-loom song.loom` to open a project file straight away.

// On Windows, a program is either a "console" program, which opens a
// terminal window, or a "windows" program, which does not. Release builds
// are the second kind; debug builds keep the console, so you can see
// `println!` and panic messages while developing. `cfg_attr` applies the
// attribute only when its condition is true.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod channel_rack;
mod harmony_keys;
mod library_browser;
mod midi;
mod piano_roll;
mod playlist;
mod theme;
mod widgets;

use std::path::PathBuf;

fn main() -> eframe::Result {
    // The first command-line argument, if there is one, is a file to open.
    // `args_os` keeps file names exactly as the system gave them, even if
    // they are not valid UTF-8.
    let path = std::env::args_os().nth(1).map(PathBuf::from);

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Ferrous Loom")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferrous Loom",
        options,
        Box::new(|cc| Ok(Box::new(app::LoomApp::new(cc, path)))),
    )
}
