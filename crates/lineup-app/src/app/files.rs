//! New, Open, Save and Save As, and asking before unsaved changes are lost.
//!
//! This is a child module of `app` (its file is `app/files.rs`), so it can
//! use the private fields of [`LineupApp`]: in Rust, private means "visible
//! in this module and the modules inside it". The `impl LineupApp` block
//! below adds more methods to the same struct; a type can have as many
//! `impl` blocks as you like, in different files of the same crate.

use std::path::{Path, PathBuf};

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers};
use lineup_audio::PlayMode;
use lineup_core::{Project, file};

use super::LineupApp;
use super::export::MidiExport;
use super::library::Demo;
use crate::channel_rack::is_project_file;
use crate::library_browser::Tried;
use crate::piano_roll::PianoRoll;
use crate::theme;

const NEW: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::N);
const OPEN: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::O);
const SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
const SAVE_AS: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::S);
const QUIT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

/// Something that would throw away the current project, waiting for the
/// user to say what to do with unsaved changes.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum Pending {
    /// A chord sheet with a band.
    New,
    /// An empty pattern and a drum kit.
    NewBeat,
    Open,
    OpenPath(PathBuf),
    Demo(Demo),
    Import,
    ImportMidi,
    ImportChordSheet,
    Quit,
}

/// The answer to "Save changes?".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    Save,
    Discard,
    Cancel,
}

/// What the app remembers about the project's file.
pub(super) struct FileState {
    /// Where the project was last opened from or saved to.
    path: Option<PathBuf>,
    /// The demo's name, while the project is a demo that has not been
    /// saved to a file yet. Demos are built into the app, so they are
    /// read-only: Save asks for a new file, as Save As does.
    demo: Option<&'static str>,
    /// The name of a song made from another kind of file (a MIDI file, a
    /// chord sheet as text), until it is saved.
    name: Option<String>,
    /// A copy of the project as it was when last opened or saved. If the
    /// project differs from it, there are unsaved changes. Comparing the
    /// two (thanks to the derived `PartialEq`) means even undoing a change
    /// by hand counts as "no changes".
    saved: Project,
    pending: Option<Pending>,
    /// Set once the user agreed to quit, so closing is no longer stopped.
    quitting: bool,
    /// The window title last sent, so a new one is only sent when it changes.
    title: String,
}

impl FileState {
    /// The app starts with the starting song, so that is the demo open.
    pub(super) fn new(project: &Project) -> Self {
        FileState {
            path: None,
            demo: Some(Demo::Start.name()),
            name: None,
            saved: project.clone(),
            pending: None,
            quitting: false,
            title: String::new(),
        }
    }

    /// Remembers that the project now is `demo`, not a file.
    pub(super) fn open_demo(&mut self, demo: Demo) {
        self.path = None;
        self.demo = Some(demo.name());
    }

    /// Names a song just made from another kind of file, until it is
    /// saved: "Autumn Leaves" for a chord sheet with that title.
    pub(super) fn name_imported(&mut self, name: &str) {
        let name = name.trim();
        self.name = (!name.is_empty()).then(|| name.to_string());
    }

    /// The file's name, such as "song.lineup", the demo's name, such as
    /// "Für Elise", or "Untitled".
    fn display_name(&self) -> String {
        match &self.path {
            Some(path) => file_name(path, Path::file_name),
            None => self.song_name().unwrap_or_else(|| "Untitled".to_string()),
        }
    }

    /// The song's name, if it has one: the file's name without
    /// ".lineup", the demo's name or the name of a song imported.
    pub(super) fn song_name(&self) -> Option<String> {
        match (&self.path, self.demo) {
            (Some(path), _) => Some(file_name(path, Path::file_stem)),
            (None, Some(demo)) => Some(demo.to_string()),
            (None, None) => self.name.clone(),
        }
    }

    /// The name without ".lineup": "song" for "song.lineup", "Für Elise" for
    /// that demo.
    pub(super) fn stem(&self) -> String {
        // A name such as "AC/DC" would be read as a folder.
        self.song_name().map_or_else(
            || "Untitled".to_string(),
            |name| name.replace(['/', '\\'], "-"),
        )
    }

    /// A name for an exported file: "song.wav" for "song.lineup", when
    /// `extension` is "wav".
    pub(super) fn export_name(&self, extension: &str) -> String {
        format!("{}.{extension}", self.stem())
    }

    /// What the title bar shows: the name, a star when there are unsaved
    /// changes, and whether it is a demo.
    fn title(&self, changed: bool) -> String {
        let star = if changed { "*" } else { "" };
        let demo = if self.path.is_none() && self.demo.is_some() {
            " (demo, read-only)"
        } else {
            ""
        };
        format!("{}{star}{demo} · Ferrous Lineup", self.display_name())
    }
}

/// One part of a path as text: `part` is `Path::file_name` or
/// `Path::file_stem`. Both return an `Option`, because a path such as
/// "/" has no name; "Untitled" stands in then.
fn file_name(path: &Path, part: fn(&Path) -> Option<&std::ffi::OsStr>) -> String {
    part(path).map_or_else(
        || "Untitled".to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// Writes the chords a song plays on its chord sheet if the sheet is
/// empty, so a song opens with its chords written, bar by bar (see
/// [`Project::write_heard_chords`]). A demo built from a chart has them
/// written already, from the chart.
pub(super) fn fill_chord_sheet(project: &mut Project) {
    if project.playlist().chord_markers().is_empty() && project.band().is_none() {
        project.write_heard_chords();
    }
}

/// A file dialog that shows project files, the old `.loom` ones too.
fn project_dialog() -> rfd::FileDialog {
    let extensions = [file::EXTENSION, file::OLD_EXTENSION];
    rfd::FileDialog::new().add_filter("Ferrous Lineup project", &extensions)
}

impl LineupApp {
    fn has_unsaved_changes(&self) -> bool {
        self.project != self.file.saved
    }

    /// The File menu at the left of the transport bar.
    pub(super) fn file_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let item = |text, shortcut: KeyboardShortcut| {
            egui::Button::new(text).shortcut_text(ctx.format_shortcut(&shortcut))
        };
        ui.menu_button("File", |ui| {
            let new = ui.add(item("New", NEW));
            if theme::hint(new, "An empty chord sheet with a band to play it").clicked() {
                self.request(&ctx, Pending::New);
                ui.close();
            }
            let beat = ui.button("New beat");
            if theme::hint(beat, "An empty pattern and a drum kit").clicked() {
                self.request(&ctx, Pending::NewBeat);
                ui.close();
            }
            if ui.add(item("Open…", OPEN)).clicked() {
                self.request(&ctx, Pending::Open);
                ui.close();
            }
            self.demos_menu(ui);
            if ui
                .button("Import recording…")
                .on_hover_text("Open an MP3 or WAV file and add chords that follow it")
                .clicked()
            {
                self.request(&ctx, Pending::Import);
                ui.close();
            }
            if ui
                .button("Import MIDI file…")
                .on_hover_text("Open a .mid file: its notes, tempo, time signatures and key")
                .clicked()
            {
                self.request(&ctx, Pending::ImportMidi);
                ui.close();
            }
            if ui
                .button("Import chord sheet…")
                .on_hover_text("Open a chord chart saved as text, as in | C | Am7 | Dm7 G7 |")
                .clicked()
            {
                self.request(&ctx, Pending::ImportChordSheet);
                ui.close();
            }
            ui.separator();
            let save = ui.add(item("Save", SAVE));
            let save = if self.file.path.is_none() && self.file.demo.is_some() {
                save.on_hover_text("Demos are read-only: this saves a copy to a new file")
            } else {
                save
            };
            if save.clicked() {
                self.save();
                ui.close();
            }
            if ui.add(item("Save as…", SAVE_AS)).clicked() {
                self.save_as();
                ui.close();
            }
            ui.separator();
            let export = ui
                .add_enabled(self.can_export(), egui::Button::new("Export WAV…"))
                .on_hover_text("Save the song as a sound file")
                .on_disabled_hover_text("Put some clips in the playlist first");
            if export.clicked() {
                self.export_song();
                ui.close();
            }
            let has_song = self.project.song_length() > 0;
            let song_midi = ui
                .add_enabled(has_song, egui::Button::new("Export song as MIDI…"))
                .on_hover_text("Save the notes of the song, to open in another studio")
                .on_disabled_hover_text("Put some clips in the playlist first");
            if song_midi.clicked() {
                self.export_midi(MidiExport::Song);
                ui.close();
            }
            if ui
                .button("Export pattern as MIDI…")
                .on_hover_text("Save the notes of the selected pattern")
                .clicked()
            {
                self.export_midi(MidiExport::Pattern);
                ui.close();
            }
            if ui
                .button("Export chord sheet…")
                .on_hover_text("Save the chord sheet as text, to share or paste anywhere")
                .clicked()
            {
                self.export_chord_sheet();
                ui.close();
            }
            ui.separator();
            if ui.add(item("Quit", QUIT)).clicked() {
                self.request(&ctx, Pending::Quit);
                ui.close();
            }
        });
    }

    pub(super) fn file_shortcuts(&mut self, ctx: &egui::Context) {
        // `consume_shortcut` checks for the keys and, if they were pressed,
        // removes them so no text field sees them too. Ctrl+Shift+S is
        // checked before Ctrl+S, because Ctrl+S would also match it.
        let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
        if pressed(SAVE_AS) {
            self.save_as();
        } else if pressed(SAVE) {
            self.save();
        }
        if pressed(NEW) {
            self.request(ctx, Pending::New);
        }
        if pressed(OPEN) {
            self.request(ctx, Pending::Open);
        }
        if pressed(QUIT) {
            self.request(ctx, Pending::Quit);
        }
    }

    /// Opens a project file, after asking about unsaved changes.
    pub(super) fn open_path(&mut self, ctx: &egui::Context, path: PathBuf) {
        self.request(ctx, Pending::OpenPath(path));
    }

    /// Does `action` now if nothing would be lost, or asks first.
    pub(super) fn request(&mut self, ctx: &egui::Context, action: Pending) {
        if self.has_unsaved_changes()
            || (matches!(&action, Pending::Quit) && self.style_editor.has_unsaved_changes())
        {
            self.file.pending = Some(action);
        } else {
            self.perform(ctx, action);
        }
    }

    fn perform(&mut self, ctx: &egui::Context, action: Pending) {
        match action {
            Pending::New => {
                let mut project = Project::with_band();
                if let Some(swing) = project
                    .band()
                    .and_then(|band| self.swings.of(band.style.name))
                {
                    project.swing = swing;
                }
                self.replace_project(project, None);
                self.chord_sheet.open = true;
                self.set_mode(PlayMode::Song);
            }
            Pending::NewBeat => self.replace_project(Project::template(), None),
            Pending::Open => {
                // This waits until the user picks a file or cancels. The
                // window does not redraw meanwhile, but the music goes on:
                // it runs on the audio thread.
                if let Some(path) = project_dialog().pick_file() {
                    self.open_now(path);
                }
            }
            Pending::OpenPath(path) => self.open_now(path),
            Pending::Demo(demo) => self.open_demo(demo),
            Pending::Import => self.import_recording(),
            Pending::ImportMidi => self.import_midi(),
            Pending::ImportChordSheet => self.import_chord_sheet(),
            Pending::Quit => {
                self.file.quitting = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }

    fn open_now(&mut self, path: PathBuf) {
        if lineup_listen::is_audio_file(&path) {
            self.import_path(path);
            return;
        }
        if lineup_core::midi_file::is_midi_file(&path) {
            self.import_midi_path(&path);
            return;
        }
        if lineup_core::chord_sheet::text::is_text_file(&path) {
            self.import_chord_sheet_path(&path);
            return;
        }
        match file::load(&path) {
            Ok(project) => self.replace_project(project, Some(path)),
            Err(err) => self.message = Some(format!("Could not open {}: {err}", path.display())),
        }
    }

    /// Swaps in another project and tells the audio thread about it.
    pub(super) fn replace_project(&mut self, mut project: Project, path: Option<PathBuf>) {
        fill_chord_sheet(&mut project);
        self.project = project;
        self.auditions.clear();
        self.song_library = Tried::default();
        self.song_versions.clear();
        self.song_sources.clear();
        self.file.saved = self.project.clone();
        self.file.path = path;
        self.file.demo = None;
        self.file.name = None;
        self.pattern = 0;
        self.playing = false;
        self.cursor = 0;
        self.marker = 0;
        self.piano_roll = PianoRoll::new(super::first_synth(&self.project));
        self.chord_sheet.new_song();
        self.harmony_keys.forget_chords();
        self.key_finder.new_project(&self.project, Vec::new());
        self.message = None;
        self.notice = None;
        // Undo cannot go back into another file, and its samples may
        // not be needed any more.
        self.history.clear();
        self.samples.clear();
        self.send_whole_project();
    }

    /// Saves to the project's file, or asks for one if it has none yet,
    /// as for a demo, which cannot be changed. Returns whether the project
    /// was saved.
    fn save(&mut self) -> bool {
        match self.file.path.clone() {
            Some(path) => self.save_to(path),
            None => self.save_as(),
        }
    }

    /// Asks where to save, then saves. Returns whether the project was saved.
    fn save_as(&mut self) -> bool {
        let suggested = self.file.export_name(file::EXTENSION);
        let Some(mut path) = project_dialog().set_file_name(suggested).save_file() else {
            return false;
        };
        if path.extension().is_none() {
            path.set_extension(file::EXTENSION);
        }
        self.save_to(path)
    }

    fn save_to(&mut self, path: PathBuf) -> bool {
        match file::save(&self.project, &path) {
            Ok(()) => {
                self.file.saved = self.project.clone();
                self.file.path = Some(path);
                self.file.demo = None;
                self.file.name = None;
                self.message = None;
                true
            }
            Err(err) => {
                self.message = Some(format!("Could not save {}: {err}", path.display()));
                false
            }
        }
    }

    /// A project file dropped on the window is opened. While the channel
    /// rack is closed, dropped samples become new channels (when it is
    /// open, the rack handles them, so they can land on a row).
    pub(super) fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped: Vec<PathBuf> = ctx.input(|input| {
            input
                .raw
                .dropped_files
                .iter()
                .map(|dropped| dropped.path().to_path_buf())
                .collect()
        });
        let (projects, samples): (Vec<PathBuf>, Vec<PathBuf>) =
            dropped.into_iter().partition(|path| is_project_file(path));
        if let Some(path) = projects.into_iter().next() {
            self.open_path(ctx, path);
        }
        if !self.rack_open {
            for path in samples {
                self.load_sample(None, path);
            }
        }
    }

    /// Shows "Save changes?" while an action is waiting for an answer.
    /// Also stops the window from closing while there are unsaved changes.
    pub(super) fn unsaved_changes_dialog(&mut self, ctx: &egui::Context) {
        let close_requested = ctx.input(|input| input.viewport().close_requested());
        if close_requested
            && !self.file.quitting
            && (self.has_unsaved_changes() || self.style_editor.has_unsaved_changes())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.file.pending = Some(Pending::Quit);
        }

        let Some(pending) = self.file.pending.clone() else {
            return;
        };
        if matches!(pending, Pending::Quit) && self.style_editor.has_unsaved_changes() {
            let mut choice = None;
            let modal =
                egui::Modal::new(egui::Id::unique("unsaved-style-on-quit")).show(ctx, |ui| {
                    ui.heading("Save style changes?");
                    ui.label("The style draft has unsaved edits.");
                    ui.horizontal(|ui| {
                        if ui.button("Save style").clicked() {
                            choice = Some(Choice::Save);
                        }
                        if ui.button("Don't save style").clicked() {
                            choice = Some(Choice::Discard);
                        }
                        if ui.button("Cancel").clicked() {
                            choice = Some(Choice::Cancel);
                        }
                    });
                });
            if modal.should_close() && choice.is_none() {
                choice = Some(Choice::Cancel);
            }
            match choice {
                Some(Choice::Save) => {
                    self.save_user_style(self.style_editor.current_style(), false)
                }
                Some(Choice::Discard) => {
                    self.style_editor.discard_edits();
                    self.send(lineup_audio::Command::StopPhrase);
                }
                Some(Choice::Cancel) => {
                    self.file.pending = None;
                    return;
                }
                None => return,
            }
            if self.style_editor.has_unsaved_changes() {
                return;
            }
        }
        if matches!(pending, Pending::Quit) && !self.has_unsaved_changes() {
            self.file.pending = None;
            self.perform(ctx, pending);
            return;
        }
        let name = self.file.display_name();
        let mut choice = None;
        let modal = egui::Modal::new(egui::Id::unique("unsaved-changes")).show(ctx, |ui| {
            ui.set_width(320.0);
            ui.heading("Save changes?");
            ui.add_space(4.0);
            ui.label(format!("\"{name}\" has changes that are not saved yet."));
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    choice = Some(Choice::Save);
                }
                if ui.button("Don't save").clicked() {
                    choice = Some(Choice::Discard);
                }
                if ui.button("Cancel").clicked() {
                    choice = Some(Choice::Cancel);
                }
            });
        });
        // Escape, or a click outside the dialog, means "Cancel".
        if modal.should_close() && choice.is_none() {
            choice = Some(Choice::Cancel);
        }

        match choice {
            Some(Choice::Save) => {
                self.file.pending = None;
                if self.save() {
                    self.perform(ctx, pending);
                }
            }
            Some(Choice::Discard) => {
                self.file.pending = None;
                self.perform(ctx, pending);
            }
            // Nothing happens, which is easy to miss: say so.
            Some(Choice::Cancel) => {
                self.file.pending = None;
                self.notice = Some(format!("Cancelled: \"{name}\" stays open as it is."));
            }
            None => {}
        }
    }

    /// Shows the file or demo name in the window's title bar, with a star
    /// after it when there are unsaved changes.
    pub(super) fn update_title(&mut self, ctx: &egui::Context) {
        let title = self.file.title(self.has_unsaved_changes());
        if title != self.file.title {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            self.file.title = title;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_demo_shows_its_name_and_a_star_once_changed() {
        let mut state = FileState::new(&Project::template());
        assert_eq!(
            state.title(false),
            "The starting song (demo, read-only) · Ferrous Lineup"
        );
        let song = lineup_core::library::SONGS
            .iter()
            .find(|song| song.name == "Für Elise")
            .unwrap();
        state.open_demo(Demo::Song(song));
        assert_eq!(
            state.title(false),
            "Für Elise (demo, read-only) · Ferrous Lineup"
        );
        assert_eq!(
            state.title(true),
            "Für Elise* (demo, read-only) · Ferrous Lineup"
        );
        // Save suggests the demo's name for the new file.
        assert_eq!(state.export_name(file::EXTENSION), "Für Elise.lineup");
    }

    #[test]
    fn a_saved_file_shows_its_file_name() {
        let mut state = FileState::new(&Project::template());
        state.path = Some(PathBuf::from("/music/song.lineup"));
        state.demo = None;
        assert_eq!(state.title(true), "song.lineup* · Ferrous Lineup");
        assert_eq!(state.export_name("wav"), "song.wav");
        state.path = None;
        assert_eq!(state.title(false), "Untitled · Ferrous Lineup");
    }

    #[test]
    fn a_slash_in_a_demo_name_does_not_become_a_folder() {
        let mut state = FileState::new(&Project::template());
        state.demo = Some("A/B");
        assert_eq!(state.export_name("mid"), "A-B.mid");
    }

    #[test]
    fn an_imported_song_keeps_its_name_until_saved() {
        let mut state = FileState::new(&Project::template());
        state.demo = None;
        assert_eq!(state.song_name(), None);
        state.name_imported(" Autumn Leaves ");
        assert_eq!(state.title(false), "Autumn Leaves · Ferrous Lineup");
        assert_eq!(state.export_name(file::EXTENSION), "Autumn Leaves.lineup");
        // Once saved, the file's name is the song's.
        state.path = Some(PathBuf::from("/music/leaves.lineup"));
        assert_eq!(state.song_name().as_deref(), Some("leaves"));
        state.path = None;
        state.name_imported("  ");
        assert_eq!(state.title(false), "Untitled · Ferrous Lineup");
    }
}
