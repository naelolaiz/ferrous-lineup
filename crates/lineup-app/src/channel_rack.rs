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
use lineup_core::{
    Channel, ChannelKind, DRUM_CHANNEL, Drum, MAX_CHANNELS, Project, TimeSignature, file,
    general_midi,
};

use crate::i18n::{tr, tr_name, trf};
use crate::midi::Port;
use crate::widgets;

/// Something the user asked the rack to do.
#[derive(Debug, Clone, PartialEq)]
pub enum RackAction {
    PlayPattern,
    PausePattern,
    StopPattern,
    /// Play the pattern over the song while it plays, or stop doing so.
    PlayOverSong(bool),
    SelectChannel(Option<usize>),
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
    /// Lock a channel, so styles leave it alone, or unlock it.
    SetLocked {
        channel: usize,
        locked: bool,
    },
    Remove(usize),
    /// Delete or Backspace in the rack: removes the channel the user chose,
    /// if they chose one (not the one the piano roll picked by itself).
    RemoveChosen,
    OpenPianoRoll(usize),
    /// Show another pattern in the rack.
    SelectPattern(usize),
    AddDrum(Drum),
    AddSynth,
    /// Add a channel that plays an external synth.
    AddMidiOut,
    /// Send a channel over MIDI on MIDI channel `midi_channel` (0 to
    /// 15): a synth becomes a MIDI Out channel, and a MIDI Out channel
    /// changes its channel.
    SetMidiChannel {
        channel: usize,
        midi_channel: u8,
    },
    /// Play a MIDI Out channel on the built-in sound again.
    PlayHere(usize),
    /// Send every MIDI Out channel to this output port, or to none.
    ConnectMidiOutput(Option<Port>),
    /// Change the General MIDI instrument of a synth or MIDI Out channel,
    /// or take it away (`None`).
    SetProgram {
        channel: usize,
        program: Option<u8>,
    },
    /// Load a WAV file into a channel, or into a new channel if `None`.
    LoadSample {
        channel: Option<usize>,
        path: PathBuf,
    },
}

const NAME_WIDTH: f32 = 110.0;
const DROP_HIGHLIGHT: Color32 = Color32::from_rgba_premultiplied(255, 152, 48, 40);

#[derive(Clone, Copy, Default)]
pub struct RackPlayback {
    pub playing: bool,
    pub can_stop: bool,
    /// The song is playing, so the pattern can be played over it.
    pub song_playing: bool,
    /// The pattern is playing over the song.
    pub over_song: bool,
}

/// The MIDI outputs the instrument menus offer. The app keeps them up
/// to date (see [`set_midi_outputs`]), as the rack and the mixer cannot
/// reach the MIDI connection themselves.
#[derive(Clone, Default)]
pub struct MidiOutputs {
    pub ports: Vec<Port>,
    pub connected: Option<Port>,
}

fn midi_outputs_id() -> egui::Id {
    egui::Id::unique("midi-outputs")
}

/// Tells the instrument menus which MIDI outputs there are.
pub fn set_midi_outputs(ctx: &egui::Context, outputs: MidiOutputs) {
    ctx.data_mut(|data| data.insert_temp(midi_outputs_id(), outputs));
}

/// The "Instrument" submenu (see [`instrument_choices`]).
fn instrument_menu(ui: &mut Ui, channel: usize, state: &Channel, actions: &mut Vec<RackAction>) {
    let label = match state.program {
        Some(program) => trf("Instrument: {}", &[&tr(general_midi::name(program))]),
        None => tr("Instrument").to_string(),
    };
    ui.menu_button(label, |ui| instrument_choices(ui, channel, state, actions))
        .response
        .on_hover_text(tr(
            "The General MIDI instrument this channel plays: sent to a MIDI synth as a \
             program change, written in exported MIDI files, and imitated by the Lineup Osc",
        ));
}

/// The instruments a synth or MIDI Out channel (`state`, number
/// `channel`) can play, inside a menu: sending it over MIDI, "None" for
/// the Lineup Osc's own sound, and the 128 General MIDI programs in their
/// sixteen families. The mixer shows them too.
pub fn instrument_choices(
    ui: &mut Ui,
    channel: usize,
    state: &Channel,
    actions: &mut Vec<RackAction>,
) {
    let current = state.program;
    let sent_on = state.sent_over_midi();
    // Sending it over MIDI comes first, where the long list of
    // instruments below can't scroll it out of sight.
    midi_part(ui, channel, state, actions);
    ui.separator();
    widgets::scroll_menu(ui, |ui| {
        let none = if sent_on.is_some() {
            "None (no program change)"
        } else {
            "None (the Lineup Osc's own sound)"
        };
        let none = ui
            .selectable_label(current.is_none(), tr(none))
            .on_hover_text(tr(
                "No General MIDI program: nothing is sent to a MIDI synth",
            ));
        if none.clicked() {
            actions.push(RackAction::SetProgram {
                channel,
                program: None,
            });
            ui.close();
        }
        ui.separator();
        for (family, family_name) in general_midi::FAMILIES.iter().enumerate() {
            let here = current.is_some_and(|program| general_midi::family(program) == family);
            let family_label = if here {
                format!("● {}", tr(family_name))
            } else {
                tr(family_name).to_string()
            };
            widgets::scroll_menu_button(ui, family_label, |ui| {
                for program in (family * 8..family * 8 + 8).map(|p| p as u8) {
                    let text = format!(
                        "{} {}",
                        u16::from(program) + 1,
                        tr(general_midi::name(program))
                    );
                    if ui
                        .selectable_label(current == Some(program), text)
                        .clicked()
                    {
                        actions.push(RackAction::SetProgram {
                            channel,
                            program: Some(program),
                        });
                        ui.close();
                    }
                }
            });
        }
    });
}

/// Sending channel `channel` (`state`) over MIDI, inside a menu: the
/// MIDI channel and the output, and "Play it here instead" once it is
/// sent. The instrument menus start with it; a built-in drum, which has
/// no instrument to choose, has only this.
pub fn midi_part(ui: &mut Ui, channel: usize, state: &Channel, actions: &mut Vec<RackAction>) {
    let sent_on = state.sent_over_midi();
    midi_choices(ui, channel, sent_on, actions);
    if sent_on.is_some()
        && ui
            .button(tr("Play it here instead"))
            .on_hover_text(tr(
                "Stop sending it over MIDI, and play it with the built-in sound",
            ))
            .clicked()
    {
        actions.push(RackAction::PlayHere(channel));
        ui.close();
    }
}

/// "Send over MIDI": the MIDI channel to send channel `channel` on (it is
/// sent on `sent_on` already, if any), and the output that every channel
/// sent over MIDI goes to.
fn midi_choices(ui: &mut Ui, channel: usize, sent_on: Option<u8>, actions: &mut Vec<RackAction>) {
    let label = match sent_on {
        Some(midi_channel) => trf("● Sent over MIDI, channel {}", &[&(midi_channel + 1)]),
        None => tr("Send over MIDI").to_string(),
    };
    ui.menu_button(label, |ui| {
        ui.label(tr("MIDI channel"));
        egui::Grid::new("midi-channels").show(ui, |ui| {
            for midi_channel in 0..16 {
                let mut response = ui.selectable_label(
                    sent_on == Some(midi_channel),
                    (midi_channel + 1).to_string(),
                );
                if midi_channel == DRUM_CHANNEL {
                    response =
                        response.on_hover_text(tr("General MIDI synths play drums on channel 10"));
                }
                if response.clicked() {
                    actions.push(RackAction::SetMidiChannel {
                        channel,
                        midi_channel,
                    });
                    ui.close();
                }
                if midi_channel % 4 == 3 {
                    ui.end_row();
                }
            }
        });
        ui.separator();
        ui.label(tr("Output, for every channel sent over MIDI"));
        let outputs: MidiOutputs = ui
            .ctx()
            .data(|data| data.get_temp(midi_outputs_id()))
            .unwrap_or_default();
        if ui
            .selectable_label(outputs.connected.is_none(), tr("None"))
            .clicked()
        {
            actions.push(RackAction::ConnectMidiOutput(None));
            ui.close();
        }
        if outputs.ports.is_empty() {
            ui.weak(tr("No outputs found (the MIDI menu looks again)"));
        }
        for port in &outputs.ports {
            if ui
                .selectable_label(outputs.connected.as_ref() == Some(port), &port.name)
                .clicked()
            {
                actions.push(RackAction::ConnectMidiOutput(Some(port.clone())));
                ui.close();
            }
        }
    });
}

/// Draws the rack for pattern `pattern` and returns what the user did
/// this frame. `selected` is the selected channel, which the piano roll
/// shows and the keyboards play; `playhead` is the step of the pattern
/// being played, if any. `locks` shows a lock on every row, which only
/// the song's rack has: a style has no styles to keep a channel from.
pub fn show(
    ui: &mut Ui,
    project: &Project,
    pattern: usize,
    selected: Option<usize>,
    playhead: Option<usize>,
    playback: Option<RackPlayback>,
    locks: bool,
) -> Vec<RackAction> {
    let mut actions = Vec::new();
    let pattern_index = pattern;
    let Some(pattern) = project.pattern(pattern) else {
        return actions;
    };
    let files_hovering = ui.ctx().input(|input| !input.raw.hovered_files.is_empty());
    let pointer = ui.ctx().input(|input| input.pointer.hover_pos());
    let dismissing = widgets::dismissing(ui.ctx());

    if let Some(playback) = playback {
        playback_row(
            ui,
            project,
            pattern_index,
            playback,
            dismissing,
            &mut actions,
        );
    }
    ui.weak(tr("Click a channel to select and hear it; click it again or empty space to deselect. Double-click for the piano roll."));

    drums_elsewhere(ui, project, pattern, &mut actions);

    let rack = Rows {
        project,
        pattern,
        // The time signature where the pattern is used in the song.
        time: project.pattern_time(pattern_index),
        selected,
        playhead,
        locks,
        dismissing,
    };
    let (row_rects, empty) = rack.show(ui, &mut actions);

    if empty.is_positive()
        && ui
            .interact(
                empty,
                ui.scope_id().with("empty-rack"),
                egui::Sense::click(),
            )
            .clicked()
        && !dismissing
    {
        actions.push(RackAction::SelectChannel(None));
    }

    if widgets::editor_keyboard(ui.ctx(), ui.layer_id())
        && selected.is_some()
        && ui.input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Delete)
                || input.consume_key(egui::Modifiers::NONE, egui::Key::Backspace)
        })
    {
        actions.push(RackAction::RemoveChosen);
    }

    // Highlight the row a dragged file would land on.
    let target_row = pointer.and_then(|p| row_rects.iter().position(|rect| rect.contains(p)));
    if files_hovering && let Some(row) = target_row {
        ui.painter()
            .rect_filled(row_rects[row], 3.0, DROP_HIGHLIGHT);
    }
    load_dropped_files(ui, target_row, &mut actions);

    actions
}

/// The buttons over the rows that play the pattern, or play it over the
/// song while the song plays, and the menu that picks another pattern.
fn playback_row(
    ui: &mut Ui,
    project: &Project,
    pattern_index: usize,
    playback: RackPlayback,
    dismissing: bool,
    actions: &mut Vec<RackAction>,
) {
    ui.horizontal_wrapped(|ui| {
        if playback.song_playing {
            if ui
                .add(egui::Button::new(tr("▶ Play over the song")).selected(playback.over_song))
                .on_hover_text(tr(
                    "Play this pattern on top of the song from its next bar, in time with \
                     it, to try it out with the rest. Click again to stop it.",
                ))
                .clicked()
                && !dismissing
            {
                actions.push(RackAction::PlayOverSong(!playback.over_song));
            }
        } else {
            let play = tr(if playback.playing {
                "⏸ Pause pattern"
            } else {
                "▶ Play pattern"
            });
            if ui
                .button(play)
                .on_hover_text(tr(
                    "Play every channel in this pattern with its current instruments",
                ))
                .clicked()
                && !dismissing
            {
                actions.push(if playback.playing {
                    RackAction::PausePattern
                } else {
                    RackAction::PlayPattern
                });
            }
        }
        if ui
            .add_enabled(playback.can_stop, egui::Button::new(tr("■ Stop pattern")))
            .clicked()
            && !dismissing
        {
            actions.push(RackAction::StopPattern);
        }
        egui::ComboBox::from_id_salt("rack-pattern")
            .selected_text(tr_name(&project.patterns()[pattern_index].name))
            .show_ui(ui, |ui| {
                for (index, choice) in project.patterns().iter().enumerate() {
                    if ui
                        .selectable_label(index == pattern_index, tr_name(&choice.name))
                        .clicked()
                    {
                        actions.push(RackAction::SelectPattern(index));
                    }
                }
            });
    });
}

/// What every row of the rack is drawn from: the same for all of them.
#[derive(Clone, Copy)]
struct Rows<'a> {
    project: &'a Project,
    pattern: &'a lineup_core::Pattern,
    /// The time signature where the pattern is used in the song.
    time: TimeSignature,
    selected: Option<usize>,
    playhead: Option<usize>,
    locks: bool,
    dismissing: bool,
}

impl Rows<'_> {
    /// Draws a row for each channel, the lights over their steps and the
    /// "Add channel" menu, scrolling when they don't fit. Returns where
    /// each row is, for dropped files, and the empty space under them,
    /// where a click deselects.
    fn show(self, ui: &mut Ui, actions: &mut Vec<RackAction>) -> (Vec<Rect>, Rect) {
        let project = self.project;
        let mut row_rects: Vec<Rect> = Vec::with_capacity(project.channels().len());
        // The rack scrolls when the window is smaller than its rows. When
        // all the rows fit, the mouse wheel scrolls the steps sideways,
        // without having to hold Shift.
        ui.style_mut().always_scroll_the_only_direction = true;
        let rows_height = project.channels().len() as f32
            * (widgets::step_size(ui).y + ui.spacing().item_spacing.y)
            + widgets::step_lights_height(ui)
            + ui.spacing().item_spacing.y
            + 4.0
            + ui.spacing().interact_size.y
            + ui.spacing().item_spacing.y;
        let vertical = rows_height > ui.available_height();
        let mut content_bottom = ui.cursor().top();
        let scroll = egui::ScrollArea::new([true, vertical])
            .auto_shrink(false)
            .show(ui, |ui| {
                // A row of lights over the steps, lit over the one
                // sounding. Its lights are drawn once the rows below have
                // shown where the steps start.
                let (lights, _) = ui.allocate_exact_size(
                    egui::vec2(1.0, widgets::step_lights_height(ui)),
                    egui::Sense::hover(),
                );
                let mut steps_left = None;
                for (index, channel) in project.channels().iter().enumerate() {
                    let row = ui.horizontal(|ui| self.row(ui, index, channel, actions));
                    steps_left.get_or_insert(row.inner);
                    row_rects.push(row.response.rect);
                }
                if let Some(left) = steps_left {
                    let marks = step_marks(self.time, self.pattern.length());
                    widgets::step_lights(ui, lights, left, &marks, self.playhead);
                }

                ui.add_space(4.0);
                add_channel_menu(ui, project, actions);
                content_bottom = ui.cursor().top();
            });
        let empty = Rect::from_min_max(
            egui::pos2(
                scroll.inner_rect.left(),
                content_bottom.max(scroll.inner_rect.top()),
            ),
            scroll.inner_rect.max,
        );
        (row_rects, empty)
    }

    /// Draws channel `index`'s row: its light, knob and lock, its name,
    /// and its steps. Returns where its steps start.
    fn row(
        self,
        ui: &mut Ui,
        index: usize,
        channel: &Channel,
        actions: &mut Vec<RackAction>,
    ) -> f32 {
        let locked = self.controls(ui, index, channel, actions);
        self.name(ui, index, channel, locked, actions);
        ui.add_space(6.0 * widgets::zoom(ui));
        let steps_left = ui.cursor().left();
        self.steps(ui, index, actions);
        steps_left
    }

    /// The mute light, the volume knob and, in the song's rack, the
    /// lock. Returns whether the channel is locked.
    fn controls(
        self,
        ui: &mut Ui,
        index: usize,
        channel: &Channel,
        actions: &mut Vec<RackAction>,
    ) -> bool {
        let (project, dismissing) = (self.project, self.dismissing);
        // Lit while the channel is heard: not muted, and not silenced by
        // another channel's solo in the mixer.
        let hint = if !channel.muted && project.channel_muted(index) {
            "Silent while another channel plays solo (see the Mixer). \
             Click to mute or unmute"
        } else {
            "Click to mute or unmute"
        };
        if widgets::mute_light(ui, !project.channel_muted(index))
            .on_hover_text(tr(hint))
            .clicked()
            && !dismissing
        {
            actions.push(RackAction::ToggleMute(index));
        }

        let mut volume = channel.volume;
        let knob = widgets::knob(ui, &mut volume, 0.8)
            .on_hover_text(trf("Volume {}%", &[&format!("{:.0}", volume * 100.0)]));
        if knob.changed() && !dismissing {
            actions.push(RackAction::SetVolume {
                channel: index,
                volume,
            });
        }

        let locked = project.is_locked(index);
        if self.locks
            && widgets::lock_light(ui, locked)
                .on_hover_text(tr(if locked {
                    "Locked: styles keep its notes and sound. Click to unlock"
                } else {
                    "Lock: styles keep its notes and sound"
                }))
                .clicked()
            && !dismissing
        {
            actions.push(RackAction::SetLocked {
                channel: index,
                locked: !locked,
            });
        }
        locked
    }

    /// The channel's name: a click selects it and plays it, a double
    /// click opens its piano roll, and a right click opens its menu.
    fn name(
        self,
        ui: &mut Ui,
        index: usize,
        channel: &Channel,
        locked: bool,
        actions: &mut Vec<RackAction>,
    ) {
        let selected = self.selected;
        let name = egui::Button::new(tr_name(&channel.name))
            .min_size(egui::vec2(NAME_WIDTH, widgets::STEP_SIZE.y) * widgets::zoom(ui))
            .selected(selected == Some(index))
            .truncate();
        let hover = tr(
            "Click to select and hear this channel; click again to deselect. \
                     Double-click to open its piano roll. Keyboards play the selected \
                     channel. Delete removes the selected channel. Right-click for options. \
                     Drop a WAV file here to load it.",
        );
        let hover = if self.locks && locked {
            format!("{}\n\n{hover}", tr("🔒 Locked"))
        } else {
            hover.to_string()
        };
        let name = match channel.program {
            Some(program) => ui.add(name).on_hover_text(format!(
                "{}\n\n{hover}",
                trf(
                    "Plays: {} (General MIDI {})",
                    &[&tr(general_midi::name(program)), &(u16::from(program) + 1)],
                ),
            )),
            None => ui.add(name).on_hover_text(hover),
        };
        if !self.dismissing {
            if name.double_clicked() {
                actions.push(RackAction::OpenPianoRoll(index));
            } else if name.clicked() {
                if selected == Some(index) {
                    actions.push(RackAction::SelectChannel(None));
                } else {
                    actions.push(RackAction::SelectChannel(Some(index)));
                    actions.push(RackAction::Preview(index));
                }
            }
        }
        name.context_menu(|ui| self.channel_menu(ui, index, channel, locked, actions));
    }

    /// The menu on a channel's name.
    fn channel_menu(
        self,
        ui: &mut Ui,
        index: usize,
        channel: &Channel,
        locked: bool,
        actions: &mut Vec<RackAction>,
    ) {
        if ui.button(tr("Piano roll")).clicked() {
            actions.push(RackAction::OpenPianoRoll(index));
            ui.close();
        }
        if matches!(
            channel.kind,
            ChannelKind::Synth | ChannelKind::MidiOut { .. }
        ) {
            instrument_menu(ui, index, channel, actions);
        } else if channel.can_go_over_midi() {
            midi_part(ui, index, channel, actions);
        }
        if self.locks
            && ui
                .button(tr(if locked { "Unlock" } else { "Lock" }))
                .clicked()
        {
            actions.push(RackAction::SetLocked {
                channel: index,
                locked: !locked,
            });
            ui.close();
        }
        if ui.button(tr("Remove channel")).clicked() {
            actions.push(RackAction::Remove(index));
            ui.close();
        }
    }

    /// Plain steps are shown as buttons; melodies and chords as a small
    /// preview of the piano roll.
    fn steps(self, ui: &mut Ui, index: usize, actions: &mut Vec<RackAction>) {
        let (pattern, playhead) = (self.pattern, self.playhead);
        let length = pattern.length();
        if pattern.is_step_row(index) {
            for step in 0..length {
                let on = pattern.is_on(index, step);
                // Beats alternate colours, so you can count them,
                // starting again in every bar so all bars look alike:
                // in 3/4, dark, light, dark.
                let beat_group = self.time.beat_at(step).index;
                let response = widgets::step_button(ui, on, beat_group, playhead == Some(step));
                if response.clicked() && !widgets::dismissing(ui.ctx()) {
                    actions.push(RackAction::ToggleStep {
                        channel: index,
                        step,
                    });
                } else if response.secondary_clicked() && !self.dismissing {
                    actions.push(RackAction::ClearStep {
                        channel: index,
                        step,
                    });
                }
            }
        } else {
            let preview = widgets::mini_roll(ui, pattern.notes(index), length, playhead)
                .on_hover_text(tr("Click to open the piano roll"));
            if preview.clicked() && !self.dismissing {
                actions.push(RackAction::OpenPianoRoll(index));
            }
        }
    }
}

/// How each of the `length` steps is marked in the row of lights: the
/// first step of a bar, of a beat, or neither.
fn step_marks(time: TimeSignature, length: usize) -> Vec<widgets::StepMark> {
    let steps_per_bar = time.steps_per_bar();
    (0..length)
        .map(|step| {
            if step % steps_per_bar == 0 {
                widgets::StepMark::Bar
            } else if time.beat_at(step).start == step % steps_per_bar {
                widgets::StepMark::Beat
            } else {
                widgets::StepMark::Plain
            }
        })
        .collect()
}

/// The "Add channel" menu under the rows: a drum, a synth or a MIDI Out
/// channel, while the rack has room.
fn add_channel_menu(ui: &mut Ui, project: &Project, actions: &mut Vec<RackAction>) {
    let rack_full = project.channels().len() >= MAX_CHANNELS;
    ui.add_enabled_ui(!rack_full, |ui| {
        ui.menu_button(tr("➕ Add channel"), |ui| {
            widgets::scroll_menu(ui, |ui| {
                for drum in Drum::ALL {
                    if ui.button(tr(drum.name())).clicked() {
                        actions.push(RackAction::AddDrum(drum));
                        ui.close();
                    }
                }
                ui.separator();
                if ui.button(tr("Lineup Osc (synth)")).clicked() {
                    actions.push(RackAction::AddSynth);
                    ui.close();
                }
                if ui
                    .button(tr("MIDI Out (external synth)"))
                    .on_hover_text(tr(
                        "Plays its notes on a synth outside Ferrous Lineup: pick where in \
                     MIDI > Output.",
                    ))
                    .clicked()
                {
                    actions.push(RackAction::AddMidiOut);
                    ui.close();
                }
            });
        });
    });
}

/// A file dropped on a row loads into that channel; anywhere else, into
/// a new one. Project files are left for the app to open.
fn load_dropped_files(ui: &Ui, target_row: Option<usize>, actions: &mut Vec<RackAction>) {
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
}

/// When a pattern has no drum steps but other patterns do, says so, with
/// a button for each of those patterns. Songs often keep their drums in a
/// pattern of their own ("A" and "A drums"), and without this the drum
/// rows of "A" just look broken.
fn drums_elsewhere(
    ui: &mut Ui,
    project: &Project,
    pattern: &lineup_core::Pattern,
    actions: &mut Vec<RackAction>,
) {
    let with_drums = patterns_with_drums(project, pattern);
    if with_drums.is_empty() {
        return;
    }
    ui.horizontal_wrapped(|ui| {
        ui.label(trf(
            "No drum steps in \"{}\": click a step to add one. Drums are in:",
            &[&tr_name(&pattern.name)],
        ));
        for (index, name) in with_drums {
            if ui.button(tr_name(name)).clicked() {
                actions.push(RackAction::SelectPattern(index));
            }
        }
    });
}

/// The patterns with drum steps, by index and name, when `pattern` has
/// none; empty when it has some.
fn patterns_with_drums<'a>(
    project: &'a Project,
    pattern: &lineup_core::Pattern,
) -> Vec<(usize, &'a str)> {
    let has_steps = |pattern: &lineup_core::Pattern| {
        (0..pattern.rows()).any(|row| pattern.notes(row).iter().any(|note| note.is_step()))
    };
    if has_steps(pattern) {
        return Vec::new();
    }
    project
        .patterns()
        .iter()
        .enumerate()
        .filter(|(_, other)| has_steps(other))
        .map(|(index, other)| (index, other.name.as_str()))
        .collect()
}

/// How big the rack is with nothing hidden: as wide as a row (the light,
/// the knob, the lock, the name, a small gap and the steps, with spacing
/// between them all) and as tall as all the rows, the "Add channel" button and
/// the line that says where the drums are, if it is shown. `zoom` is the
/// rack's zoom; `style` must already be scaled by it.
pub fn natural_size(
    style: &egui::Style,
    project: &Project,
    pattern: usize,
    zoom: f32,
) -> egui::Vec2 {
    let Some(pattern) = project.pattern(pattern) else {
        return egui::Vec2::ZERO;
    };
    let steps = pattern.length();
    let gap = style.spacing.item_spacing;
    let step = widgets::STEP_SIZE * zoom;
    let controls = (14.0 + 24.0 + 14.0 + NAME_WIDTH + 6.0) * zoom + 5.0 * gap.x;
    let width = controls + steps as f32 * (step.x + gap.x);
    // The steps, and the row of lights over them.
    let rows = project.channels().len() as f32 * (step.y + gap.y)
        + widgets::STEP_LIGHTS_HEIGHT * zoom
        + gap.y;
    let line = style.spacing.interact_size.y + gap.y;
    let add_channel = 4.0 + line;
    let hint = if patterns_with_drums(project, pattern).is_empty() {
        0.0
    } else {
        line
    };
    // Room for a scroll bar, so one never covers the last row or step.
    let bar = style.spacing.scroll.bar_width + style.spacing.scroll.bar_outer_margin;
    egui::vec2(width + bar, 2.0 * line + hint + rows + add_channel + bar)
}

/// Whether a path opens as a project: a `.lineup` file, or a MIDI
/// file, a chord sheet saved as text or a recording, which are imported.
/// Other audio files are sounds for the sampler.
pub fn is_project_file(path: &std::path::Path) -> bool {
    file::is_project(path)
        || lineup_core::midi_file::is_midi_file(path)
        || lineup_core::chord_sheet::text::is_text_file(path)
        || (lineup_listen::is_audio_file(path) && lineup_audio::sample::is_recording(path))
}
