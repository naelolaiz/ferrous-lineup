//! Ferrous Lineup: an open source music studio with a backing band.
//!
//! `main` only sets up the window. Everything you see is drawn by
//! [`app::LineupApp`].
//!
//! Run `ferrous-lineup song.lineup` to open a project file straight away,
//! or `ferrous-lineup --trace song.lineup` to write down every note the
//! song plays without opening a window (see `app/trace.rs`).

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
pub(crate) mod i18n;
mod icons;
mod lead_sheet;
mod library_browser;
mod meters;
mod micromodes;
mod midi;
mod open_files;
mod piano_roll;
mod playlist;
mod style_editor;
mod theme;
mod time_picker;
mod twelve_tone;
mod video;
mod widgets;

use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;

const USAGE: &str = "\
Usage:
  ferrous-lineup [FILE]
      Opens FILE, or the starting song.
  ferrous-lineup --trace FILE|DEMO [--out TRACE] [--rate HZ]
      Writes a trace of every note the song plays, as File > Export WAV
      plays it, without opening a window: a table with a line for each
      note, to TRACE or else to the terminal.
  ferrous-lineup --demos
      Lists the names of the demos.";

/// The window icon: 256 x 256 RGBA pixels, made from
/// `packaging/icon/ferrous-lineup.svg` by `packaging/icon/generate.py`.
const WINDOW_ICON: &[u8] = include_bytes!("../assets/icon/icon-256.rgba");
const WINDOW_ICON_SIZE: u32 = 256;

fn window_icon() -> eframe::egui::IconData {
    eframe::egui::IconData {
        rgba: WINDOW_ICON.to_vec(),
        width: WINDOW_ICON_SIZE,
        height: WINDOW_ICON_SIZE,
    }
}

fn main() -> eframe::Result {
    // The first command-line argument, if there is one, is a file to open,
    // or what to do instead of opening the window. `args_os` keeps file
    // names exactly as the system gave them, even if they are not valid
    // UTF-8.
    let mut args = std::env::args_os().skip(1);
    let first = args.next();
    match first.as_ref().and_then(|arg| arg.to_str()) {
        Some("--trace") => std::process::exit(trace(args)),
        Some("--demos") => {
            for name in app::demo_names() {
                println!("{name}");
            }
            return Ok(());
        }
        Some("--help" | "-h") => {
            println!("{USAGE}");
            return Ok(());
        }
        _ => {}
    }
    let path = first.map(PathBuf::from);

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_title(app::APP_NAME)
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([800.0, 520.0])
        .with_icon(window_icon());

    // Inside a Flatpak sandbox the desktop entry is named after the app
    // id, so the window must use the same id for the dock and the icon to
    // find it. Native builds keep eframe's default (the app name), which
    // is also the folder where the settings and the SoundFont live; the
    // desktop entry's StartupWMClass names it, so docks match the window.
    if let Some(app_id) = std::env::var("FLATPAK_ID")
        .ok()
        .filter(|app_id| !app_id.is_empty())
    {
        viewport = viewport.with_app_id(app_id);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    // On macOS a project opened from Finder comes in a message, not on
    // the command line.
    open_files::listen();

    eframe::run_native(
        app::APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc, path)))),
    )
}

/// `--trace`, with the arguments after it. Returns the exit code: 0 when
/// the trace was written, 1 when it could not be, 2 for a mistake in the
/// arguments.
fn trace(mut args: impl Iterator<Item = OsString>) -> i32 {
    let mut source = None;
    let mut out = None;
    let mut rate = app::TRACE_SAMPLE_RATE;
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--out" | "-o") => out = args.next().map(PathBuf::from),
            Some("--rate") => match args.next().and_then(|hz| hz.to_str()?.parse().ok()) {
                Some(hz) if (8_000..=192_000).contains(&hz) => rate = hz,
                _ => {
                    eprintln!("--rate takes a sample rate in Hz, such as 44100");
                    return 2;
                }
            },
            _ if source.is_none() => source = Some(arg),
            _ => {
                eprintln!("{USAGE}");
                return 2;
            }
        }
    }
    let Some(source) = source else {
        eprintln!("{USAGE}");
        return 2;
    };
    let table = match app::trace_table(&source, rate) {
        Ok(table) => table,
        Err(err) => {
            eprintln!("{err}");
            return 1;
        }
    };
    let written = match &out {
        Some(path) => std::fs::write(path, &table),
        None => std::io::stdout().write_all(&table),
    };
    match written {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Could not write the trace: {err}");
            1
        }
    }
}
