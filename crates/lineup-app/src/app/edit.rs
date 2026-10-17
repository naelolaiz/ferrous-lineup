//! Changing the project: undo, redo, and keeping the audio thread in step.
//!
//! Every change to the project goes through [`LineupApp::edit`]. It keeps a
//! copy of the project from before the change, for undo, and compares the
//! two versions to tell the audio thread exactly what changed. Undo and
//! redo swap in a whole other version of the project, and the same
//! comparison tells the audio thread about that too. So there is only one
//! way the audio thread learns about changes, and undo cannot get it wrong
//! in a way that normal editing would not show at once.

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers};
use lineup_audio::Command;
use lineup_core::{Change, Project, diff};

use super::LineupApp;

const UNDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Z);
const REDO: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::Z);
/// The other common shortcut for redo, used on Windows.
const REDO_Y: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Y);

impl LineupApp {
    /// Changes the project with `change`, as one undoable step called
    /// `label` ("move clip"), and returns whatever `change` returns.
    ///
    /// If nothing actually changed (a clip moved onto itself, a step that
    /// was already off), nothing is recorded or sent.
    pub(super) fn edit<R>(
        &mut self,
        label: &'static str,
        change: impl FnOnce(&mut Project) -> R,
    ) -> R {
        let before = self.project.clone();
        let result = change(&mut self.project);
        if self.project != before {
            self.sync_audio(&before);
            // `before` is moved into the history here, after its last use.
            self.history.record(label, before);
        }
        result
    }

    pub(super) fn undo(&mut self) {
        let before = self.project.clone();
        if self.history.undo(&mut self.project).is_some() {
            self.sync_audio(&before);
            self.restore_song_library();
            self.keep_place_in_song(&before);
        }
    }

    pub(super) fn redo(&mut self) {
        let before = self.project.clone();
        if self.history.redo(&mut self.project).is_some() {
            self.sync_audio(&before);
            self.restore_song_library();
            self.keep_place_in_song(&before);
        }
    }

    /// Tells the audio thread what changed since `before`, and keeps the
    /// selected pattern and the piano roll's channel pointing at the
    /// right things when patterns or channels come and go.
    fn sync_audio(&mut self, before: &Project) {
        let sample_rate = self.sample_rate();
        let mut patterns_moved = false;
        let mut channels_changed = false;
        for change in diff(before, &self.project) {
            match change {
                Change::RemovePattern(index) => {
                    self.send(Command::RemovePattern(index));
                    // Keep the same pattern selected. If it was the one
                    // removed, the next one takes its place.
                    if index < self.pattern {
                        self.pattern -= 1;
                    }
                    patterns_moved = true;
                }
                Change::InsertPattern(index) => {
                    let length = self.project.patterns()[index].length();
                    self.send(Command::InsertPattern { index, length });
                    // A new pattern (added, cloned, or brought back by
                    // undo) is the one you want to see, from its start.
                    self.pattern = index;
                    self.rewind_for_new_pattern();
                    patterns_moved = true;
                }
                Change::RemoveChannel(index) => {
                    self.send(Command::RemoveChannel(index));
                    channels_changed = true;
                    self.piano_roll
                        .channel_removed(index, self.project.channels().len());
                }
                Change::InsertChannel(index) => {
                    let command = self.channel_command(index, sample_rate);
                    self.send(command);
                    channels_changed = true;
                    self.piano_roll.channel_inserted(index);
                }
                Change::Instrument(channel) => {
                    let state = self.project.channels()[channel].clone();
                    let instrument = self.build_instrument(&state, sample_rate);
                    self.send(Command::SetInstrument {
                        channel,
                        instrument,
                    });
                    channels_changed = true;
                }
                Change::Volume(channel) => {
                    let volume = self.project.channels()[channel].volume;
                    self.send(Command::SetVolume { channel, volume });
                }
                Change::Muted(channel) => {
                    let muted = self.project.channels()[channel].muted;
                    self.send(Command::SetMuted { channel, muted });
                }
                Change::PatternLength(pattern) => {
                    let length = self.project.patterns()[pattern].length();
                    self.send(Command::SetPatternLength { pattern, length });
                }
                Change::Notes { pattern, channel } => {
                    let notes = self.project.patterns()[pattern].notes(channel).to_vec();
                    self.send(Command::SetNotes {
                        pattern,
                        channel,
                        notes,
                    });
                }
                Change::Tempo => self.send(Command::SetTempo(self.project.tempo)),
                Change::LooseTiming => {
                    self.send(Command::SetLooseTiming(self.project.loose_timing));
                }
                Change::Song => {
                    for command in self.song_commands() {
                        self.send(command);
                    }
                }
            }
        }
        if channels_changed {
            self.live.channels_changed(self.project.channels().len());
        }
        if patterns_moved {
            // The audio thread's selected pattern was counted in the old
            // list, so it is sent again.
            let last = self.project.patterns().len() - 1;
            self.select_pattern(self.pattern.min(last));
        }
        self.follow_project_changed();
        self.key_finder.project_changed();
    }

    /// Called at the start of each frame. A press of a mouse button
    /// always begins a new undo step.
    pub(super) fn start_frame_for_undo(&mut self, ctx: &egui::Context) {
        if ctx.input(|input| input.pointer.any_pressed()) {
            self.history.close();
        }
    }

    /// Called at the end of each frame. While a mouse button is held (a
    /// drag), a text field has the keyboard (typing a name) or a take is
    /// being recorded, changes join one undo step; once all of them are
    /// over, the step ends.
    pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
        let holding = ctx.input(|input| input.pointer.any_down());
        if !holding && !ctx.text_edit_focused() && !self.is_recording() {
            self.history.close();
        }
    }

    /// The Edit menu, next to the File menu.
    pub(super) fn edit_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let undo = self.history.undo_label();
        let redo = self.history.redo_label();
        ui.menu_button("Edit", |ui| {
            let text = undo.map_or_else(|| "Undo".to_string(), |label| format!("Undo {label}"));
            let button = egui::Button::new(text).shortcut_text(ctx.format_shortcut(&UNDO));
            if ui.add_enabled(undo.is_some(), button).clicked() {
                self.undo();
                ui.close();
            }
            let text = redo.map_or_else(|| "Redo".to_string(), |label| format!("Redo {label}"));
            let button = egui::Button::new(text).shortcut_text(ctx.format_shortcut(&REDO));
            if ui.add_enabled(redo.is_some(), button).clicked() {
                self.redo();
                ui.close();
            }
        });
    }

    /// Ctrl+Z to undo; Ctrl+Shift+Z or Ctrl+Y to redo (⌘ on a Mac).
    pub(super) fn edit_shortcuts(&mut self, ctx: &egui::Context) {
        let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
        // Ctrl+Z would also match Ctrl+Shift+Z, so redo is checked first.
        if pressed(REDO) || pressed(REDO_Y) {
            self.redo();
        } else if pressed(UNDO) {
            self.undo();
        }
    }
}
