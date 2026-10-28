//! File > Open recent: the projects opened or saved lately.
//!
//! The list is kept in egui's stored data, so it is the same next run, and
//! is capped so the menu stays short.

use std::path::{Path, PathBuf};

use eframe::egui;

use super::LineupApp;
use super::files::Pending;

/// How many paths are remembered.
const MAX_RECENT: usize = 10;

fn state_id() -> egui::Id {
    egui::Id::unique("recent-projects")
}

/// The paths lately opened or saved, most recent first.
#[derive(Default)]
pub(super) struct Recent {
    paths: Vec<PathBuf>,
    /// Whether the list changed since it was last stored.
    changed: bool,
}

impl Recent {
    /// The list kept from the last run.
    pub(super) fn restore(ctx: &egui::Context) -> Self {
        let paths = ctx
            .data_mut(|data| data.get_persisted::<Vec<String>>(state_id()))
            .unwrap_or_default()
            .into_iter()
            .map(PathBuf::from)
            .collect();
        Recent {
            paths,
            changed: false,
        }
    }

    /// Remembers `path` as the most recent one.
    pub(super) fn add(&mut self, path: &Path) {
        let path = path.to_path_buf();
        if self.paths.first() == Some(&path) {
            return;
        }
        self.paths.retain(|known| known != &path);
        self.paths.insert(0, path);
        self.paths.truncate(MAX_RECENT);
        self.changed = true;
    }

    /// Forgets `path`, as when it cannot be opened any more.
    pub(super) fn remove(&mut self, path: &Path) {
        let before = self.paths.len();
        self.paths.retain(|known| known != path);
        self.changed |= self.paths.len() != before;
    }

    /// The remembered paths, most recent first.
    pub(super) fn paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Writes the list into egui's stored data, if it changed.
    pub(super) fn store(&mut self, ctx: &egui::Context) {
        if !self.changed {
            return;
        }
        // Paths that are not UTF-8 cannot be written down and read back
        // as the same path; they stay only for this run.
        let paths: Vec<String> = self
            .paths
            .iter()
            .filter_map(|path| path.to_str().map(str::to_owned))
            .collect();
        ctx.data_mut(|data| data.insert_persisted(state_id(), paths));
        self.changed = false;
    }
}

impl LineupApp {
    /// The File menu's "Open recent" submenu.
    pub(super) fn recent_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let paths = self.recent.paths().to_vec();
        ui.add_enabled_ui(!paths.is_empty(), |ui| {
            ui.menu_button(crate::i18n::tr("Open recent"), |ui| {
                for path in &paths {
                    let name = path.file_name().map_or_else(
                        || path.display().to_string(),
                        |name| name.to_string_lossy().into_owned(),
                    );
                    let response = ui.button(name).on_hover_text(path.display().to_string());
                    if response.clicked() {
                        ui.close();
                        self.request(&ctx, Pending::OpenPath(path.clone()));
                    }
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_paths_are_newest_first_and_unique() {
        let mut recent = Recent::default();
        let a = PathBuf::from("/music/a.lineup");
        let b = PathBuf::from("/music/b.lineup");
        recent.add(&a);
        recent.add(&b);
        recent.add(&a);
        assert_eq!(recent.paths().to_vec(), [a.clone(), b]);
        for index in 0..MAX_RECENT + 2 {
            recent.add(&PathBuf::from(format!("/music/{index}.lineup")));
        }
        assert_eq!(recent.paths().len(), MAX_RECENT);
        assert_eq!(
            recent.paths()[0],
            PathBuf::from(format!("/music/{}.lineup", MAX_RECENT + 1))
        );
    }

    #[test]
    fn the_list_survives_a_restart() {
        let ctx = egui::Context::default();
        let mut recent = Recent::restore(&ctx);
        recent.add(Path::new("/music/song.lineup"));
        recent.store(&ctx);
        assert_eq!(
            Recent::restore(&ctx).paths().to_vec(),
            recent.paths().to_vec()
        );
    }
}
