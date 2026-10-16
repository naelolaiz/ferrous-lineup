//! The channel rack: one row per channel, with its steps in the selected
//! pattern.
//!
//! [`show`] only draws and reads the mouse. It does not change the
//! project: it returns a list of [`RackAction`]s, and the app applies
//! them. Keeping drawing and changing apart means the same action does
//! the same thing wherever it comes from (a click today, a keyboard
//! shortcut or an undo history tomorrow).

use std::path::PathBuf;

use eframe::egui::{self, Color32, Rect, Ui};
use loom_core::{Drum, MAX_CHANNELS, Project, file};

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

/// Draws the rack for pattern `pattern` and returns what the user did
/// this frame. `playhead` is the step of that pattern being played, if any.
pub fn show(
    ui: &mut Ui,
    project: &Project,
    pattern: usize,
    playhead: Option<usize>,
) -> Vec<RackAction> {
    let mut actions = Vec::new();
    let Some(pattern) = project.pattern(pattern) else {
        return actions;
    };
    let mut row_rects: Vec<Rect> = Vec::with_capacity(project.channels().len());
    let length = pattern.length();
    let files_hovering = ui.ctx().input(|input| !input.raw.hovered_files.is_empty());
    let pointer = ui.ctx().input(|input| input.pointer.hover_pos());

    // The rack window is as wide as a row, up to the size of the main
    // window; beyond that, it scrolls. A row is the light, the knob, the
    // name, a small gap and the steps, with spacing between them all.
    let gap = ui.spacing().item_spacing.x;
    let controls = 14.0 + 24.0 + NAME_WIDTH + 6.0 + 4.0 * gap;
    let row_width = controls + length as f32 * (widgets::STEP_SIZE.x + gap);
    let room = ui.ctx().content_rect().size() - egui::vec2(60.0, 120.0);
    egui::ScrollArea::both()
        .min_scrolled_width(row_width.min(room.x))
        .max_width(room.x)
        .max_height(room.y)
        .show(ui, |ui| {
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
                    if pattern.is_step_row(index) {
                        for step in 0..length {
                            let on = pattern.is_on(index, step);
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
                        let preview =
                            widgets::mini_roll(ui, pattern.notes(index), length, playhead)
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

    // A file dropped on a row loads into that channel; anywhere else, into
    // a new one. Project files are left for the app to open.
    let dropped: Vec<PathBuf> = ui.ctx().input(|input| {
        input
            .raw
            .dropped_files
            .iter()
            .map(|dropped| dropped.path().to_path_buf())
            .filter(|path| !is_project_file(path))
            .collect()
    });
    for (i, path) in dropped.into_iter().enumerate() {
        // Only the first file replaces the channel under the pointer.
        let channel = if i == 0 { target_row } else { None };
        actions.push(RackAction::LoadSample { channel, path });
    }

    actions
}

/// Whether a path ends in `.loom`.
pub fn is_project_file(path: &std::path::Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case(file::EXTENSION))
}
