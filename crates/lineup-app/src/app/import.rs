//! File > Import MIDI file: a MIDI file becomes a new project, with its
//! notes, tempo, time signatures and key. Reading one takes a few
//! thousandths of a second, so it is done straight away.
//!
//! A recording (an MP3, WAV, FLAC or Ogg file) is not imported as a
//! project: it is added to the song as a take (see `voice`).

use std::path::Path;

use lineup_audio::PlayMode;
use lineup_core::midi_file;

use super::LineupApp;

impl LineupApp {
    /// Asks for a MIDI file, then imports it.
    pub(super) fn import_midi(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(crate::i18n::tr("MIDI file"), &midi_file::EXTENSIONS)
            .pick_file()
        {
            self.import_midi_path(&path);
        }
    }

    /// Makes a new project from the MIDI file at `path`, in song mode,
    /// ready to play. Opening a MIDI file (`ferrous-lineup song.mid`, or
    /// dropping it on the window) comes here too.
    pub(super) fn import_midi_path(&mut self, path: &Path) {
        let title = path
            .file_stem()
            .map_or_else(|| "MIDI file".into(), |stem| stem.to_string_lossy());
        let imported = std::fs::read(path)
            .map_err(|err| err.to_string())
            .and_then(|bytes| midi_file::read::read(&bytes).map_err(|err| err.to_string()))
            .and_then(|file| {
                midi_file::import::import(&file, &title).map_err(|err| err.to_string())
            });
        match imported {
            Ok(imported) => {
                let summary = imported.report.summary(&title, &imported.project);
                self.replace_project(imported.project, None);
                self.file.name_imported(&title);
                self.set_mode(PlayMode::Song);
                self.notice = Some(summary);
            }
            Err(err) => {
                self.message = Some(crate::i18n::trf(
                    "Could not import {}: {}",
                    &[&path.display(), &err],
                ));
            }
        }
    }
}
