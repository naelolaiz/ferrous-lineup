//! The channel rack: one row per channel, with its steps.
//!
//! [`show`] only draws and reads the mouse. It does not change the
//! project: it returns a list of [`RackAction`]s, and the app applies
//! them. Keeping drawing and changing apart means the same action does
//! the same thing wherever it comes from (a click today, a keyboard
//! shortcut or an undo history tomorrow).

use std::path::PathBuf;

use eframe::egui::{self, Color32, Rect, Ui};
use loom_core::{Drum, MAX_CHANNELS, Project};

use crate::widgets;

/// Something the user asked the rack to do.
#[derive(Debug, Clone, PartialEq)]
pub enum RackAction {
    ToggleStep {
        channel: usize,
        step: usize,
    },
    ClearStep {
        channel: usize,
        step: usize,
    },
    Preview(usize),
    SetVolume {
        channel: usize,
        volume: f32,
    },
    ToggleMute(usize),
    Remove(usize),
    OpenPianoRoll(usize),
    AddDrum(Drum),
    AddSynth,
    /// Load a WAV file into a channel, or into a new channel if `None`.
    LoadSample {
        channel: Option<usize>,
        path: PathBuf,
    },
}

const NAME_WIDTH: f32 = 110.0;
const DROP_HIGHLIGHT: Color32 = Color32::from_rgba_premultiplied(255, 152, 48, 40);

/// Draws the rack and returns what the user did this frame.
pub fn show(ui: &mut Ui, project: &Project, playhead: Option<usize>) -> Vec<RackAction> {
    let mut actions = Vec::new();
    let mut row_rects: Vec<Rect> = Vec::with_capacity(project.channels().len());
    let length = project.pattern().length();
    let files_hovering = ui.ctx().input(|input| !input.raw.hovered_files.is_empty());
    let pointer = ui.ctx().input(|input| input.pointer.hover_pos());

    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
        for (index, channel) in project.channels().iter().enumerate() {
            let row = ui.horizontal(|ui| {
                if widgets::mute_light(ui, !channel.muted)
                    .on_hover_text("Click to mute or unmute")
                    .clicked()
                {
                    actions.push(RackAction::ToggleMute(index));
                }

                let mut volume = channel.volume;
                let knob = widgets::knob(ui, &mut volume, 0.8)
                    .on_hover_text(format!("Volume {:.0}%", volume * 100.0));
                if knob.changed() {
                    actions.push(RackAction::SetVolume {
                        channel: index,
                        volume,
                    });
                }

                let name = egui::Button::new(&channel.name)
                    .min_size(egui::vec2(NAME_WIDTH, widgets::STEP_SIZE.y))
                    .truncate();
                let name = ui.add(name).on_hover_text(
                    "Click to hear it. Right-click for options. Drop a WAV file here to load it.",
                );
                if name.clicked() {
                    actions.push(RackAction::Preview(index));
                }
                name.context_menu(|ui| {
                    if ui.button("Piano roll").clicked() {
                        actions.push(RackAction::OpenPianoRoll(index));
                        ui.close();
                    }
                    if ui.button("Remove channel").clicked() {
                        actions.push(RackAction::Remove(index));
                        ui.close();
                    }
                });

                ui.add_space(6.0);
                // Plain steps are shown as buttons; melodies and chords as
                // a small preview of the piano roll.
                if project.pattern().is_step_row(index) {
                    for step in 0..length {
                        let on = project.pattern().is_on(index, step);
                        let beat_group = step / 4;
                        let response =
                            widgets::step_button(ui, on, beat_group, playhead == Some(step));
                        if response.clicked() {
                            actions.push(RackAction::ToggleStep {
                                channel: index,
                                step,
                            });
                        } else if response.secondary_clicked() {
                            actions.push(RackAction::ClearStep {
                                channel: index,
                                step,
                            });
                        }
                    }
                } else {
                    let preview = widgets::mini_roll(ui, project.notes(index), length, playhead)
                        .on_hover_text("Click to open the piano roll");
                    if preview.clicked() {
                        actions.push(RackAction::OpenPianoRoll(index));
                    }
                }
            });
            row_rects.push(row.response.rect);
        }

        ui.add_space(4.0);
        let rack_full = project.channels().len() >= MAX_CHANNELS;
        ui.add_enabled_ui(!rack_full, |ui| {
            ui.menu_button("➕ Add channel", |ui| {
                for drum in Drum::ALL {
                    if ui.button(drum.name()).clicked() {
                        actions.push(RackAction::AddDrum(drum));
                        ui.close();
                    }
                }
                ui.separator();
                if ui.button("Loom Osc (synth)").clicked() {
                    actions.push(RackAction::AddSynth);
                    ui.close();
                }
            });
        });
    });

    // Highlight the row a dragged file would land on.
    let target_row = pointer.and_then(|p| row_rects.iter().position(|rect| rect.contains(p)));
    if files_hovering && let Some(row) = target_row {
        ui.painter()
            .rect_filled(row_rects[row], 3.0, DROP_HIGHLIGHT);
    }

    // A file dropped on a row loads into that channel; anywhere else, into a new one.
    let dropped: Vec<PathBuf> = ui.ctx().input(|input| {
        input
            .raw
            .dropped_files
            .iter()
            .map(|file| file.path().to_path_buf())
            .collect()
    });
    for (i, path) in dropped.into_iter().enumerate() {
        // Only the first file replaces the channel under the pointer.
        let channel = if i == 0 { target_row } else { None };
        actions.push(RackAction::LoadSample { channel, path });
    }

    actions
}
