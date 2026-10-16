//! Ferrous Loom: an open source pattern-based music studio.
//!
//! `main` only sets up the window. Everything you see is drawn by
//! [`app::LoomApp`].

mod app;
mod channel_rack;
mod widgets;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Ferrous Loom")
            .with_inner_size([1000.0, 520.0])
            .with_min_inner_size([640.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferrous Loom",
        options,
        Box::new(|cc| Ok(Box::new(app::LoomApp::new(cc)))),
    )
}
