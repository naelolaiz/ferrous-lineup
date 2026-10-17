//! File > Import chord sheet and Export chord sheet: the chord sheet as
//! text, in a `.txt` file. Reading and writing the text is in
//! `lineup_core::chord_sheet::text`.

use std::path::Path;

use lineup_audio::PlayMode;
use lineup_core::chord_sheet::text;

use super::LineupApp;

impl LineupApp {
    /// Asks for a chord sheet saved as text, then imports it.
    pub(super) fn import_chord_sheet(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Chord sheet as text", &text::EXTENSIONS)
            .pick_file()
        {
            self.import_chord_sheet_path(&path);
        }
    }

    /// Makes a new song from the chord sheet in the text file at `path`,
    /// with a band to play it, in song mode, the chord sheet open.
    /// Dropping a `.txt` file on the window comes here too.
    pub(super) fn import_chord_sheet_path(&mut self, path: &Path) {
        let name = shown_name(path);
        let imported = std::fs::read_to_string(path)
            .map_err(|err| err.to_string())
            .and_then(|content| text::read(&content).map_err(|err| err.to_string()))
            .and_then(|sheet| {
                let bars = sheet.bar_count();
                text::import(&sheet)
                    .map(|imported| (imported, bars, sheet.title))
                    .map_err(|err| err.to_string())
            });
        match imported {
            Ok((imported, bars, title)) => {
                self.replace_project(imported.project, None);
                // The chart's title, or else the file's name.
                let stem = path.file_stem().map(|stem| stem.to_string_lossy());
                if let Some(name) = title.as_deref().or(stem.as_deref()) {
                    self.file.name_imported(name);
                }
                self.chord_sheet.open = true;
                self.set_mode(PlayMode::Song);
                let mut notice = format!(
                    "Imported {name}: {bars} bar{}",
                    if bars == 1 { "" } else { "s" }
                );
                for note in imported.notes {
                    notice.push_str(". ");
                    notice.push_str(note.trim_end_matches('.'));
                }
                self.notice = Some(notice);
            }
            Err(err) => self.message = Some(format!("Could not import {name}: {err}")),
        }
    }

    /// Saves the whole chord sheet as text.
    pub(super) fn export_chord_sheet(&mut self) {
        let Some(mut path) = rfd::FileDialog::new()
            .add_filter("Chord sheet as text", &text::EXTENSIONS)
            .set_file_name(self.file.export_name("txt"))
            .save_file()
        else {
            return;
        };
        if path.extension().is_none() {
            path.set_extension("txt");
        }
        let written = text::write(&self.project, self.file.song_name().as_deref());
        let name = shown_name(&path);
        match std::fs::write(&path, written) {
            Ok(()) => {
                self.notice = Some(format!("Exported {name}"));
                self.message = None;
            }
            Err(err) => self.message = Some(format!("Could not export {name}: {err}")),
        }
    }
}

fn shown_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}
