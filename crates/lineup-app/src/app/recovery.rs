//! Crash recovery: a copy of unsaved work, written now and then while the
//! app runs, and offered at startup when the app did not close cleanly.
//!
//! The copy lives in the app's data folder, not next to the project: a
//! project may be on a read-only medium, and a folder with an autosave in
//! it would surprise anyone looking at the song. The name and path of the
//! project travel in a small file beside it, so the recovered work opens
//! as the song it was, not as "Untitled".
//!
//! Several copies of the app can run at once, so each run keeps its copy
//! under a name of its own in a `recovery` folder, next to a lock file it
//! holds for as long as it runs. The system lets go of a lock when its
//! program ends, even in a crash, so a copy whose lock can be taken was
//! left by a run that is over, and one whose lock is held belongs to a
//! window still open and is left alone.

use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions, TryLockError};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant, SystemTime};

use eframe::egui;
use serde::{Deserialize, Serialize};

use lineup_core::{Project, file};

use super::LineupApp;

/// How often unsaved work is written while it keeps changing.
const INTERVAL: Duration = Duration::from_secs(30);

/// The folder of the runs' copies, in the app's data folder.
const FOLDER: &str = "recovery";
const PROJECT_EXTENSION: &str = "lineup";
const INFO_EXTENSION: &str = "info";
const LOCK_EXTENSION: &str = "lock";

/// The one copy that version 0.2.0 kept, straight in the data folder,
/// still offered after an update.
const OLD_PROJECT_FILE: &str = "recovery.lineup";
const OLD_INFO_FILE: &str = "recovery.info";

/// What was open when the copy was written.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Info {
    /// The file the project came from, if it had one.
    path: Option<PathBuf>,
    /// The name to show when it had no file (a demo or an import).
    name: Option<String>,
}

/// The files of one run's copy.
#[derive(Debug, Clone)]
struct Files {
    project: PathBuf,
    info: PathBuf,
    /// `None` for the copy of version 0.2.0, which had no lock.
    lock: Option<PathBuf>,
}

impl Files {
    /// The files of the run called `run`, in `folder`.
    fn of_run(folder: &Path, run: &str) -> Self {
        let file = |extension| folder.join(format!("{run}.{extension}"));
        Files {
            project: file(PROJECT_EXTENSION),
            info: file(INFO_EXTENSION),
            lock: Some(file(LOCK_EXTENSION)),
        }
    }

    /// The copy version 0.2.0 kept in the data folder.
    fn old(data: &Path) -> Self {
        Files {
            project: data.join(OLD_PROJECT_FILE),
            info: data.join(OLD_INFO_FILE),
            lock: None,
        }
    }

    /// Removes the copy, and the temporary file of a save cut short.
    fn remove_copy(&self) {
        let _ = fs::remove_file(&self.project);
        let _ = fs::remove_file(self.project.with_extension("lineup.tmp"));
        let _ = fs::remove_file(&self.info);
    }

    /// Removes the copy and the lock file.
    fn remove(&self) {
        self.remove_copy();
        if let Some(lock) = &self.lock {
            let _ = fs::remove_file(lock);
        }
    }
}

/// A recovery copy found at startup, waiting for the user's answer.
pub(super) struct Recovered {
    pub(super) project: Project,
    info: Info,
    /// Where the copy is, to remove it once answered; `None` for a copy
    /// made by a test.
    files: Option<Files>,
    /// The lock of the run that wrote the copy, held until the answer so
    /// that a window opened meanwhile does not offer it too.
    lock: Option<File>,
}

impl Recovered {
    /// A copy for tests: the project, and where it came from.
    #[cfg(test)]
    pub(super) fn new(project: Project, path: Option<PathBuf>, name: Option<String>) -> Self {
        Recovered {
            project,
            info: Info { path, name },
            files: None,
            lock: None,
        }
    }

    /// Removes the copy, now that the user has answered for it.
    fn remove(self) {
        if let Some(files) = &self.files {
            files.remove();
        }
        // The lock goes last, so no other window takes the copy halfway.
        drop(self.lock);
    }
}

/// The recovery state of one run.
pub(super) struct Recovery {
    /// Where this run writes its copy, or `None` on a system without a
    /// data folder.
    files: Option<Files>,
    /// This run's lock, taken before its first copy is written and held
    /// until the app closes.
    lock: Option<File>,
    /// When unsaved work is next written; `None` means now.
    due: Option<Instant>,
    /// Whether a write has been tried since the last save or discard. A
    /// failed write (a read-only data folder, say) waits for the next
    /// interval instead of being retried on every edit.
    attempted: bool,
    /// Whether this run has written a copy.
    written: bool,
    /// Copies from runs that are over, newest first, until the user says
    /// what to do with each.
    pub(super) pending: Vec<Recovered>,
}

impl Recovery {
    /// Finds the copies that runs which are over left in `data`, the app's
    /// data folder, and picks a name for this run's own.
    pub(super) fn new(data: Option<PathBuf>) -> Self {
        let mut recovery = Recovery {
            files: None,
            lock: None,
            due: None,
            attempted: false,
            written: false,
            pending: Vec::new(),
        };
        let Some(data) = data else {
            return recovery;
        };
        let folder = data.join(FOLDER);
        recovery.pending = left_behind(&folder);
        if let Some(old) = load(Files::old(&data), None) {
            recovery.pending.push(old);
        }
        recovery.files = Some(Files::of_run(&folder, &run_name()));
        recovery
    }

    /// The project changed: write it now if nothing has been written yet,
    /// so the first edit of a session is not the one a crash loses.
    pub(super) fn project_changed(&mut self) {
        if !self.written && !self.attempted {
            self.due = None;
        }
    }

    /// Writes this run's copy of `project`. Returns whether it was written.
    fn write(&mut self, project: &Project, info: &Info) -> bool {
        let Some(files) = self.files.clone() else {
            return false;
        };
        if let Some(folder) = files.project.parent()
            && fs::create_dir_all(folder).is_err()
        {
            return false;
        }
        // Without its lock, the copy would look left behind to another
        // window, which would offer it while this one still runs.
        if self.lock.is_none() {
            self.lock = files.lock.as_deref().and_then(take_own_lock);
            if self.lock.is_none() {
                return false;
            }
        }
        write(&files, project, info)
    }

    /// Removes this run's copy. The lock stays until the app closes.
    fn remove_copy(&mut self) {
        if let Some(files) = &self.files {
            files.remove_copy();
        }
        self.written = false;
        self.attempted = false;
    }
}

impl Drop for Recovery {
    /// The app is closing: its lock file goes with it. A copy of unsaved
    /// work stays, to be offered at the next start.
    fn drop(&mut self) {
        if let Some(lock) = self.lock.take()
            && let Some(path) = self.files.as_ref().and_then(|files| files.lock.as_ref())
        {
            let _ = fs::remove_file(path);
            drop(lock);
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
        let info = Info {
            path: self.file.path().map(Path::to_path_buf),
            name: self.file.song_name(),
        };
        if self.recovery.write(&self.project, &info) {
            self.recovery.written = true;
        }
    }

    /// Forgets the copy this run wrote, after the work was saved or
    /// discarded. Copies from earlier runs wait for their question.
    pub(super) fn forget_recovery(&mut self) {
        if self.recovery.written {
            self.recovery.remove_copy();
        }
    }

    /// Offers unsaved work from a run that did not close cleanly.
    pub(super) fn recovery_dialog(&mut self, ctx: &egui::Context) {
        if self.recovery.pending.is_empty() {
            return;
        }
        // With copies from more than one run, the next waits until the
        // work recovered before it is saved or let go, so recovering one
        // never replaces another.
        if self.has_unsaved_changes() {
            return;
        }
        let recovered = &self.recovery.pending[0];
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
        let Some(recover) = choice else {
            return;
        };
        let mut recovered = self.recovery.pending.remove(0);
        if recover {
            // The file on disk is what "saved" means for the recovered
            // project, so undoing back to it clears the star.
            let saved = recovered
                .info
                .path
                .as_ref()
                .and_then(|path| file::load(path).ok());
            let project = std::mem::replace(&mut recovered.project, Project::empty());
            self.replace_recovered(
                project,
                recovered.info.path.clone(),
                recovered.info.name.clone(),
                saved,
            );
            // The work is this run's now: its own copy is written at once.
            self.recovery.remove_copy();
            self.recovery.due = None;
            self.notice = Some("Recovered unsaved work.".to_string());
        }
        recovered.remove();
    }
}

/// A name for this run's files that no other run has: the process number,
/// the time it started, and a count for a process with more than one.
fn run_name() -> String {
    static COUNT: AtomicU32 = AtomicU32::new(0);
    let millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |since| since.as_millis());
    let count = COUNT.fetch_add(1, Ordering::Relaxed);
    format!("{}-{millis}-{count}", std::process::id())
}

/// The run a file in the recovery folder belongs to, if it is one of a
/// run's files.
fn run_of(name: &str) -> Option<&str> {
    let (run, extension) = name.split_once('.')?;
    let ours = [
        PROJECT_EXTENSION,
        "lineup.tmp",
        INFO_EXTENSION,
        LOCK_EXTENSION,
    ];
    (!run.is_empty() && ours.contains(&extension)).then_some(run)
}

/// The copies in `folder` whose runs are over, newest first. Their locks
/// are taken, so no other window offers them too. Lock files with no copy,
/// and copies that cannot be read, are removed.
fn left_behind(folder: &Path) -> Vec<Recovered> {
    let Ok(entries) = fs::read_dir(folder) else {
        return Vec::new();
    };
    let runs: BTreeSet<String> = entries
        .filter_map(|entry| {
            let name = entry.ok()?.file_name().into_string().ok()?;
            run_of(&name).map(str::to_string)
        })
        .collect();
    let mut found = Vec::new();
    for run in runs {
        let files = Files::of_run(folder, &run);
        let lock = match files.lock.as_deref().map(take_lock) {
            Some(Lock::Taken(lock)) => Some(lock),
            Some(Lock::Held) => continue,
            // A system that cannot lock files cannot tell either: offering
            // the copy loses nothing.
            Some(Lock::Unknown) | None => None,
        };
        let written = fs::metadata(&files.project).and_then(|meta| meta.modified());
        match load(files, lock) {
            Some(recovered) => found.push((written.ok(), recovered)),
            None => continue,
        }
    }
    found.sort_by(|(a, _), (b, _)| b.cmp(a));
    found.into_iter().map(|(_, recovered)| recovered).collect()
}

/// What trying to take another run's lock found.
enum Lock {
    /// The run is over: its lock is ours now.
    Taken(File),
    /// The run is still going.
    Held,
    /// The lock could not be tried.
    Unknown,
}

fn take_lock(path: &Path) -> Lock {
    // A copy whose lock file is gone was left by a run that closed with
    // work unsaved: the file is made again, to be taken.
    let Ok(file) = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
    else {
        return Lock::Unknown;
    };
    match file.try_lock() {
        Ok(()) => Lock::Taken(file),
        Err(TryLockError::WouldBlock) => Lock::Held,
        Err(TryLockError::Error(_)) => Lock::Unknown,
    }
}

/// Makes and takes this run's lock file.
fn take_own_lock(path: &Path) -> Option<File> {
    // A window starting at the same moment may clean the file up just as
    // it is made, taking it for a crashed run's: then it is made again.
    for _ in 0..3 {
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .ok()?;
        if lock.try_lock().is_ok() && path.exists() {
            return Some(lock);
        }
    }
    None
}

/// Reads the copy in `files`, if it is whole, keeping `lock` with it. A
/// copy that cannot be read is removed, with its lock file.
fn load(files: Files, lock: Option<File>) -> Option<Recovered> {
    let project = match file::load(&files.project) {
        Ok(project) => project,
        Err(_) => {
            files.remove();
            return None;
        }
    };
    let info = fs::read_to_string(&files.info)
        .ok()
        .and_then(|text| ron::from_str::<Info>(&text).ok())
        .unwrap_or_default();
    Some(Recovered {
        project,
        info,
        files: Some(files),
        lock,
    })
}

/// Writes `project` and its `info` to `files`. Returns whether both were
/// written.
fn write(files: &Files, project: &Project, info: &Info) -> bool {
    // The project is renamed into place by `file::save`, so a crash while
    // writing leaves the copy before it whole. The info goes after it: a
    // crash between them opens the project without its name rather than
    // the wrong one.
    if file::save(project, &files.project).is_err() {
        return false;
    }
    let Ok(text) = ron::to_string(info) else {
        return false;
    };
    fs::write(&files.info, text).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An empty data folder for one test.
    fn data(test: &str) -> PathBuf {
        let folder =
            std::env::temp_dir().join(format!("ferrous-recovery-{}-{test}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        folder
    }

    /// Ends a run as a crash would: its lock is let go, but its files stay.
    fn crash(mut recovery: Recovery) {
        drop(recovery.lock.take());
        recovery.files = None;
    }

    fn info(path: &str) -> Info {
        Info {
            path: Some(PathBuf::from(path)),
            name: None,
        }
    }

    #[test]
    fn a_recovery_copy_comes_back_with_its_file() {
        let data = data("comes-back");
        let mut run = Recovery::new(Some(data.clone()));
        let project = Project::template();
        assert!(run.write(&project, &info("/music/song.lineup")));
        crash(run);

        let mut next = Recovery::new(Some(data.clone()));
        assert_eq!(next.pending.len(), 1);
        let recovered = next.pending.remove(0);
        assert_eq!(recovered.project, project);
        assert_eq!(
            recovered.info.path,
            Some(PathBuf::from("/music/song.lineup"))
        );
        recovered.remove();
        drop(next);
        assert!(Recovery::new(Some(data.clone())).pending.is_empty());
        let _ = fs::remove_dir_all(&data);
    }

    #[test]
    fn a_window_still_open_keeps_its_copy_to_itself() {
        let data = data("still-open");
        let mut first = Recovery::new(Some(data.clone()));
        assert!(first.write(&Project::template(), &info("/music/first.lineup")));

        let mut second = Recovery::new(Some(data.clone()));
        assert!(second.pending.is_empty(), "the first window still runs");
        assert!(second.write(&Project::empty(), &info("/music/second.lineup")));
        // Each window wrote its own copy: neither replaced the other.
        crash(second);
        let third = Recovery::new(Some(data.clone()));
        assert_eq!(third.pending.len(), 1);
        assert_eq!(
            third.pending[0].info.path,
            Some(PathBuf::from("/music/second.lineup"))
        );
        // While the third window holds the second's copy, a fourth leaves
        // it alone too.
        assert!(Recovery::new(Some(data.clone())).pending.is_empty());

        crash(first);
        drop(third);
        let fifth = Recovery::new(Some(data.clone()));
        assert_eq!(fifth.pending.len(), 2, "both copies are offered");
        let _ = fs::remove_dir_all(&data);
    }

    #[test]
    fn closing_cleanly_leaves_no_lock_behind() {
        let data = data("clean");
        let mut run = Recovery::new(Some(data.clone()));
        assert!(run.write(&Project::template(), &info("/music/song.lineup")));
        run.remove_copy();
        drop(run);
        let left = fs::read_dir(data.join(FOLDER)).unwrap().count();
        assert_eq!(left, 0);
        let _ = fs::remove_dir_all(&data);
    }

    #[test]
    fn the_copy_of_version_0_2_0_is_still_offered() {
        let data = data("old");
        fs::create_dir_all(&data).unwrap();
        let project = Project::template();
        assert!(write(
            &Files::old(&data),
            &project,
            &info("/music/old.lineup")
        ));
        let recovery = Recovery::new(Some(data.clone()));
        assert_eq!(recovery.pending.len(), 1);
        assert_eq!(recovery.pending[0].project, project);
        let _ = fs::remove_dir_all(&data);
    }
}
