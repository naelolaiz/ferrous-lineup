//! Crash recovery: a copy of unsaved work, written now and then while the
//! app runs, and offered at startup when the app did not close cleanly.
//!
//! The copy lives in the app's data folder, not next to the project: a
//! project may be on a read-only medium, and a folder with an autosave in
//! it would surprise anyone looking at the song. The name and path of the
//! project travel in a small file beside it, so the recovered work opens
//! as the song it was, not as "Untitled".

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use eframe::egui;
use serde::{Deserialize, Serialize};

use lineup_core::{Project, file};

use super::LineupApp;

/// How often unsaved work is written while it keeps changing.
const INTERVAL: Duration = Duration::from_secs(30);

const PROJECT_FILE: &str = "recovery.lineup";
const INFO_FILE: &str = "recovery.info";

/// What was open when the copy was written.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Info {
    /// The file the project came from, if it had one.
    path: Option<PathBuf>,
    /// The name to show when it had no file (a demo or an import).
    name: Option<String>,
}

/// A recovery copy found at startup, waiting for the user's answer.
pub(super) struct Recovered {
    pub(super) project: Project,
    info: Info,
}

impl Recovered {
    /// A copy for tests: the project, and where it came from.
    pub(super) fn new(project: Project, path: Option<PathBuf>, name: Option<String>) -> Self {
        Recovered {
            project,
            info: Info { path, name },
        }
    }
}

/// The recovery state of one run.
pub(super) struct Recovery {
    /// The app's data folder, or `None` on a system without one.
    folder: Option<PathBuf>,
    /// When unsaved work is next written; `None` means now.
    due: Option<Instant>,
    /// Whether a write has been tried since the last save or discard. A
    /// failed write (a read-only data folder, say) waits for the next
    /// interval instead of being retried on every edit.
    attempted: bool,
    /// Whether this run has written a copy.
    written: bool,
    /// A copy from an earlier run, until the user says what to do with it.
    pub(super) pending: Option<Recovered>,
}

impl Recovery {
    /// Loads any copy an earlier run left.
    pub(super) fn new(folder: Option<PathBuf>) -> Self {
        let pending = folder.as_deref().and_then(load);
        Recovery {
            folder,
            due: None,
            attempted: false,
            written: false,
            pending,
        }
    }

    /// The project changed: write it now if nothing has been written yet,
    /// so the first edit of a session is not the one a crash loses.
    pub(super) fn project_changed(&mut self) {
        if !self.written && !self.attempted {
            self.due = None;
        }
    }
}

impl LineupApp {
    /// Writes unsaved work when its time comes, and removes the copy once
    /// the work is saved or discarded.
    pub(super) fn autosave(&mut self) {
        let now = Instant::now();
        if self.recovery.due.is_some_and(|due| now < due) {
            return;
        }
        self.recovery.due = Some(now + INTERVAL);
        if self.has_unsaved_changes() {
            self.recovery.attempted = true;
            self.write_recovery();
        } else if self.recovery.written {
            self.forget_recovery();
        }
    }

    fn write_recovery(&mut self) {
        let Some(folder) = self.recovery.folder.clone() else {
            return;
        };
        let info = Info {
            path: self.file.path().map(Path::to_path_buf),
            name: self.file.song_name(),
        };
        if write(&folder, &self.project, &info) {
            self.recovery.written = true;
        }
    }

    /// Forgets the copy this run wrote, after the work was saved or
    /// discarded. A copy from an earlier run waits for its question.
    pub(super) fn forget_recovery(&mut self) {
        if self.recovery.written {
            self.discard_recovery_files();
        }
    }

    fn discard_recovery_files(&mut self) {
        if let Some(folder) = &self.recovery.folder {
            remove_files(folder);
        }
        self.recovery.written = false;
        self.recovery.attempted = false;
    }

    /// Offers unsaved work from a run that did not close cleanly.
    pub(super) fn recovery_dialog(&mut self, ctx: &egui::Context) {
        let Some(recovered) = self.recovery.pending.as_ref() else {
            return;
        };
        let name = recovered
            .info
            .path
            .as_deref()
            .and_then(Path::file_name)
            .map(|name| name.to_string_lossy().into_owned())
            .or_else(|| recovered.info.name.clone())
            .unwrap_or_else(|| "Untitled".to_string());
        let mut choice = None;
        egui::Modal::new(egui::Id::unique("recover-unsaved-work")).show(ctx, |ui| {
            ui.set_width(340.0);
            ui.heading("Recover unsaved work?");
            ui.add_space(4.0);
            ui.label(format!(
                "\"{name}\" was not saved when Ferrous Lineup last closed."
            ));
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Recover it").clicked() {
                    choice = Some(true);
                }
                if ui.button("Discard it").clicked() {
                    choice = Some(false);
                }
            });
        });
        match choice {
            Some(true) => {
                let Some(recovered) = self.recovery.pending.take() else {
                    return;
                };
                // The file on disk is what "saved" means for the recovered
                // project, so undoing back to it clears the star.
                let saved = recovered
                    .info
                    .path
                    .as_ref()
                    .and_then(|path| file::load(path).ok());
                self.replace_recovered(
                    recovered.project,
                    recovered.info.path,
                    recovered.info.name,
                    saved,
                );
                self.discard_recovery_files();
                self.recovery.due = None;
                self.notice = Some("Recovered unsaved work.".to_string());
            }
            Some(false) => {
                self.recovery.pending = None;
                self.discard_recovery_files();
            }
            None => {}
        }
    }
}

/// Reads the copy in `folder`, if it is whole. A damaged copy is removed.
fn load(folder: &Path) -> Option<Recovered> {
    let project = match file::load(&folder.join(PROJECT_FILE)) {
        Ok(project) => project,
        Err(_) => {
            remove_files(folder);
            return None;
        }
    };
    let info = fs::read_to_string(folder.join(INFO_FILE))
        .ok()
        .and_then(|text| ron::from_str::<Info>(&text).ok())
        .unwrap_or_default();
    Some(Recovered::new(project, info.path, info.name))
}

/// Writes `project` and its `info` to `folder`. Returns whether both were
/// written.
fn write(folder: &Path, project: &Project, info: &Info) -> bool {
    if fs::create_dir_all(folder).is_err() {
        return false;
    }
    // The project is renamed into place by `file::save`, so a crash while
    // writing leaves the copy before it whole. The info goes after it: a
    // crash between them opens the project without its name rather than
    // the wrong one.
    if file::save(project, &folder.join(PROJECT_FILE)).is_err() {
        return false;
    }
    let Ok(text) = ron::to_string(info) else {
        return false;
    };
    fs::write(folder.join(INFO_FILE), text).is_ok()
}

fn remove_files(folder: &Path) {
    let _ = fs::remove_file(folder.join(PROJECT_FILE));
    let _ = fs::remove_file(folder.join(INFO_FILE));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder() -> PathBuf {
        std::env::temp_dir().join(format!("ferrous-recovery-{}", std::process::id()))
    }

    #[test]
    fn a_recovery_copy_comes_back_with_its_file() {
        let folder = folder();
        remove_files(&folder);
        let project = Project::template();
        let info = Info {
            path: Some(PathBuf::from("/music/song.lineup")),
            name: None,
        };
        assert!(write(&folder, &project, &info));
        let recovered = load(&folder).unwrap();
        assert_eq!(recovered.project, project);
        assert_eq!(recovered.info.path, info.path);
        remove_files(&folder);
        assert!(load(&folder).is_none());
    }
}
