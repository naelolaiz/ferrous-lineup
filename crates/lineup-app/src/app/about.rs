//! Help > About: the app's name, version and license, and where its
//! manual, its book, its code and its issues are.

use eframe::egui;

use super::LineupApp;
use crate::i18n::{self, Language, tr};

/// The user manual on the web, in English and in Spanish, and as one PDF
/// file.
const MANUAL: &str = "https://naelolaiz.github.io/ferrous-lineup/manual/";
const MANUAL_PDF: &str =
    "https://naelolaiz.github.io/ferrous-lineup/manual/ferrous-lineup-manual.pdf";
const MANUAL_ES: &str = "https://naelolaiz.github.io/ferrous-lineup/es/manual/";
const MANUAL_ES_PDF: &str =
    "https://naelolaiz.github.io/ferrous-lineup/es/manual/ferrous-lineup-manual-es.pdf";
/// The book on the web, in English and in Spanish, and as one PDF file.
const BOOK: &str = "https://naelolaiz.github.io/ferrous-lineup/";
const BOOK_PDF: &str = "https://naelolaiz.github.io/ferrous-lineup/ferrous-lineup-book.pdf";
const BOOK_ES: &str = "https://naelolaiz.github.io/ferrous-lineup/es/";
const BOOK_ES_PDF: &str =
    "https://naelolaiz.github.io/ferrous-lineup/es/ferrous-lineup-book-es.pdf";
const LICENSE: &str = "https://www.gnu.org/licenses/gpl-3.0.html";
const ISSUES: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues");

/// The user manual in the app's language: Help > User manual opens it.
pub(super) fn manual() -> &'static str {
    match i18n::current_language() {
        Language::English => MANUAL,
        Language::Spanish => MANUAL_ES,
    }
}

impl LineupApp {
    pub(super) fn about_window(&mut self, ctx: &egui::Context) {
        if !self.about_open {
            return;
        }
        let mut open = self.about_open;
        let (manual_pdf, book, book_pdf) = match i18n::current_language() {
            Language::English => (MANUAL_PDF, BOOK, BOOK_PDF),
            Language::Spanish => (MANUAL_ES_PDF, BOOK_ES, BOOK_ES_PDF),
        };
        egui::Window::new(tr("About Ferrous Lineup"))
            .id(egui::Id::unique("about"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(360.0);
                ui.horizontal(|ui| {
                    ui.add(egui::Image::new(&icon(ctx)).fit_to_exact_size(egui::vec2(64.0, 64.0)));
                    ui.vertical(|ui| {
                        ui.heading(tr("Ferrous Lineup"));
                        ui.label(format!("{} {}", tr("Version"), env!("CARGO_PKG_VERSION")));
                        ui.weak(tr(env!("CARGO_PKG_DESCRIPTION")));
                    });
                });
                ui.separator();
                ui.label(tr(
                    "Free software: you can share it and change it under the GNU General \
                     Public License, version 3 or later. It comes with no warranty.",
                ));
                ui.hyperlink_to(tr("Read the license"), LICENSE);
                ui.separator();
                ui.hyperlink_to(tr("User manual"), manual());
                ui.hyperlink_to(tr("User manual (PDF)"), manual_pdf);
                ui.hyperlink_to(tr("The book"), book);
                ui.hyperlink_to(tr("The book (PDF)"), book_pdf);
                ui.hyperlink_to(tr("Source code"), env!("CARGO_PKG_REPOSITORY"));
                ui.hyperlink_to(tr("Report an issue"), ISSUES);
            });
        self.about_open = open;
    }
}

/// The app's icon, as a picture egui can draw: the window icon, made into
/// a texture the first time the window opens.
fn icon(ctx: &egui::Context) -> egui::TextureHandle {
    let id = egui::Id::unique("about-icon");
    if let Some(texture) = ctx.data(|data| data.get_temp(id)) {
        return texture;
    }
    let size = crate::WINDOW_ICON_SIZE as usize;
    let image = egui::ColorImage::from_rgba_unmultiplied([size, size], crate::WINDOW_ICON);
    let texture = ctx.load_texture("about-icon", image, egui::TextureOptions::LINEAR);
    ctx.data_mut(|data| data.insert_temp(id, texture.clone()));
    texture
}
