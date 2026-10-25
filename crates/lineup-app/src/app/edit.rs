//! Changing the project: undo, redo, and keeping the audio thread in step.
//!
//! Every change to the project goes through [`LineupApp::edit`]. It keeps a
//! copy of the project from before the change, for undo, and compares the
//! two versions to tell the audio thread exactly what changed. Undo and
//! redo swap in a whole other version of the project, and the same
//! comparison tells the audio thread about that too. So there is only one
//! way the audio thread learns about changes, and undo cannot get it wrong
//! in a way that normal editing would not show at once.

use std::time::{Duration, Instant};

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers};
use lineup_audio::Command;
use lineup_core::history::DEFAULT_LIMIT;
use lineup_core::{Change, Project, diff};

use super::LineupApp;
use crate::chord_sheet::BarEdit;

/// How many bytes of undo snapshots to keep at most. A snapshot is about
/// the size of the project, so a large song keeps fewer steps.
const UNDO_BYTES: usize = 256 * 1024 * 1024;

/// The fewest undo steps even a very large song keeps.
const MIN_UNDO_STEPS: usize = 10;

/// How often, at most, a drag's changes reach the audio thread. The
/// window still draws every frame; only the sound waits this long, and
/// the whole project is copied only when this comes round.
const SYNC_INTERVAL: Duration = Duration::from_millis(50);

/// How many undo steps fit in [`UNDO_BYTES`] for `project`, between
/// [`MIN_UNDO_STEPS`] and [`DEFAULT_LIMIT`].
pub(super) fn undo_limit_for(project: &Project) -> usize {
    limit_for_bytes(project.heap_bytes())
}

fn limit_for_bytes(bytes: usize) -> usize {
    (UNDO_BYTES / bytes.max(1)).clamp(MIN_UNDO_STEPS, DEFAULT_LIMIT)
}

pub(super) const UNDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Z);
pub(super) const REDO: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::Z);
/// The other common shortcut for redo, used on Windows.
pub(super) const REDO_Y: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Y);

impl LineupApp {
    /// Changes the project with `change`, as one undoable step called
    /// `label` ("move clip"), and returns whatever `change` returns.
    ///
    /// If nothing actually changed (a clip moved onto itself, a step that
    /// was already off), nothing is recorded or sent. If the change was
    /// to the chord sheet, the key or the bars, the band plays it again
    /// as part of the same step (see [`Project::follow_chord_sheet`]).
    pub(super) fn edit<R>(
        &mut self,
        label: &'static str,
        change: impl FnOnce(&mut Project) -> R,
    ) -> R {
        if self.history.is_open() {
            // A drag or a typing session is under way, and its first change
            // made the undo step. Keep changing the project without copying
            // the whole of it every frame; the audio catches up steadily
            // from the last state it was given.
            let (patterns, channels) =
                (self.project.patterns().len(), self.project.channels().len());
            let result = change(&mut self.project);
            self.project.follow_chord_sheet();
            self.file.project_edited();
            self.recovery.project_changed();
            self.step_unsynced = true;
            if self.project.patterns().len() != patterns
                || self.project.channels().len() != channels
            {
                // A pattern or channel came or went: the views and the
                // audio thread's rows have to move now, not in 50 ms.
                self.sync_step();
            } else {
                self.sync_step_if_due();
            }
            return result;
        }
        // A step left open by a direct history call: send it before the
        // audio's state is assumed to be this project.
        self.finish_step_sync();
        let before = self.project.clone();
        let result = change(&mut self.project);
        // The band plays what the chord sheet says now, in the same step.
        self.project.follow_chord_sheet();
        if self.project != before {
            self.sync_audio(&before);
            self.synced = self.project.clone();
            self.step_unsynced = false;
            self.last_sync = Instant::now();
            self.file.project_edited();
            self.recovery.project_changed();
            // `before` is moved into the history here, after its last use.
            self.history.record(label, before);
        }
        result
    }

    /// Sends a drag's changes to the audio thread, at most every
    /// [`SYNC_INTERVAL`].
    fn sync_step_if_due(&mut self) {
        if self.last_sync.elapsed() >= SYNC_INTERVAL {
            self.sync_step();
        }
    }

    /// Sends everything the audio thread is missing, from the state it
    /// was last given, and keeps that state for the next time.
    fn sync_step(&mut self) {
        let before = std::mem::replace(&mut self.synced, Project::empty());
        self.sync_audio(&before);
        self.synced = self.project.clone();
        self.step_unsynced = false;
        self.last_sync = Instant::now();
    }

    /// Sends the last changes of a step before its state must match the
    /// project's, as when it closes or undo replaces the project.
    fn finish_step_sync(&mut self) {
        if self.step_unsynced {
            self.sync_step();
        }
    }

    pub(super) fn undo(&mut self) {
        self.finish_step_sync();
        let before = self.project.clone();
        if self.history.undo(&mut self.project).is_some() {
            self.sync_audio(&before);
            self.synced = self.project.clone();
            self.step_unsynced = false;
            self.last_sync = Instant::now();
            self.file.project_reverted();
            self.recovery.project_changed();
            self.restore_song_library();
            self.keep_place_in_song(&before);
        }
    }

    pub(super) fn redo(&mut self) {
        self.finish_step_sync();
        let before = self.project.clone();
        if self.history.redo(&mut self.project).is_some() {
            self.sync_audio(&before);
            self.synced = self.project.clone();
            self.step_unsynced = false;
            self.last_sync = Instant::now();
            self.file.project_reverted();
            self.recovery.project_changed();
            self.restore_song_library();
            self.keep_place_in_song(&before);
        }
    }

    /// Inserts or deletes bars, as one step to undo, and selects the bar
    /// inserted, or the one now where the deleted bars were, on the
    /// chord sheet, ready to type in.
    pub(super) fn edit_bars(&mut self, edit: BarEdit) {
        let (label, done, selected) = match edit {
            BarEdit::Insert { bar, after } => (
                "insert bar",
                self.edit("insert bar", |project| project.insert_bars(bar, after, 1)),
                bar + usize::from(after),
            ),
            BarEdit::Delete { first, last } => {
                let label = if first == last {
                    "delete bar"
                } else {
                    "delete bars"
                };
                let done = self.edit(label, |project| project.delete_bars(first, last));
                (label, done, first)
            }
        };
        if done {
            self.chord_sheet.select(selected);
        } else if label == "insert bar" {
            self.message = Some("There is no room for another bar in the song.".into());
        } else {
            self.message = Some("There is no room for the patterns the cut needs.".into());
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
                    let pattern = &self.project.patterns()[index];
                    let (length, bands) = (pattern.length(), pattern.player.is_some());
                    self.send(Command::InsertPattern { index, length });
                    if bands {
                        // A part the band wrote, as you typed a chord:
                        // the pattern you had stays selected.
                        if index <= self.pattern {
                            self.pattern += 1;
                        }
                    } else {
                        // A new pattern (added, cloned, or brought back
                        // by undo) is the one you want to see, from its
                        // start.
                        self.pattern = index;
                        self.rewind_for_new_pattern();
                    }
                    patterns_moved = true;
                }
                Change::RemoveChannel(index) => {
                    self.send(Command::RemoveChannel(index));
                    channels_changed = true;
                    self.piano_roll
                        .channel_removed(index, self.project.channels().len());
                }
                Change::InsertChannel(index) => {
                    for command in self.channel_commands(index, sample_rate) {
                        self.send(command);
                    }
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
                Change::Pan(channel) => {
                    let pan = self.project.channels()[channel].pan;
                    self.send(Command::SetPan { channel, pan });
                }
                Change::ReverbSend(channel) => {
                    let send = self.project.channels()[channel].reverb_send;
                    self.send(Command::SetReverbSend { channel, send });
                }
                Change::Muted(channel) => {
                    let muted = self.project.channel_muted(channel);
                    self.send(Command::SetMuted { channel, muted });
                }
                Change::PatternLength(pattern) => {
                    let length = self.project.patterns()[pattern].length();
                    self.send(Command::SetPatternLength { pattern, length });
                }
                Change::Notes { pattern, channel } => {
                    let notes = self.project.played_notes(pattern, channel).into_owned();
                    self.send(Command::SetNotes {
                        pattern,
                        channel,
                        notes,
                    });
                }
                Change::Tempo => {
                    let tempo =
                        super::practice::at_speed(self.project.tempo, self.practice.speed());
                    self.send(Command::SetTempo(tempo));
                }
                Change::LooseTiming => {
                    self.send(Command::SetLooseTiming(self.project.loose_timing));
                }
                Change::Mixer => {
                    self.send(Command::SetMasterVolume(self.project.mixer.master));
                    self.send(Command::SetReverb(self.project.mixer.reverb));
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
            self.finish_step_sync();
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
            self.finish_step_sync();
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
            ui.separator();
            if ui.button("Settings…").clicked() {
                self.settings_open = true;
                self.bring_to_front = Some(egui::Id::unique("application-settings"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_steps_follow_the_memory_budget() {
        assert_eq!(limit_for_bytes(1), DEFAULT_LIMIT);
        assert_eq!(limit_for_bytes(UNDO_BYTES / 20), 20);
        assert_eq!(limit_for_bytes(UNDO_BYTES), MIN_UNDO_STEPS);
        assert_eq!(limit_for_bytes(UNDO_BYTES * 4), MIN_UNDO_STEPS);
    }

    #[test]
    fn notes_count_toward_the_size_estimate() {
        let empty = Project::template();
        let mut busy = Project::template();
        let pattern = busy.pattern_mut(0).unwrap();
        pattern.set_length(256);
        for step in 0..256 {
            for pitch in 0..12u8 {
                pattern.add_note(
                    0,
                    lineup_core::NoteEvent::new(lineup_core::Note::new(60 + pitch), step, 1),
                );
            }
        }
        assert!(busy.heap_bytes() > empty.heap_bytes());
        assert!(undo_limit_for(&busy) <= undo_limit_for(&empty));
    }
}
