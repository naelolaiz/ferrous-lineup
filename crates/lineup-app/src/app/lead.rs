//! The lead sheet window, fed with the song and where it plays.

use eframe::egui;

use super::LineupApp;

impl LineupApp {
    /// Draws the lead sheet window, with the bar that plays lit up.
    pub(super) fn lead_sheet_window(&mut self, ctx: &egui::Context, area: egui::Rect) {
        let playing = self
            .song_position()
            .filter(|_| self.playing)
            .map(|(step, _)| step);
        let title = crate::i18n::tr_name(&self.file.stem()).into_owned();
        let file_name = self.file.export_name("pdf");
        self.lead_sheet
            .show(ctx, area, &self.project, &title, &file_name, playing);
    }
}
