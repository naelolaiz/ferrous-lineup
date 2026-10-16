//! Ferrous Loom: an open source pattern-based music studio.
//!
//! `main` only sets up the window. Everything you see is drawn by
//! [`app::LoomApp`].

mod app;
mod channel_rack;
mod piano_roll;
mod widgets;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Ferrous Loom")
            .with_inner_size([1100.0, 760.0])
            .with_min_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferrous Loom",
        options,
        Box::new(|cc| Ok(Box::new(app::LoomApp::new(cc)))),
    )
}
