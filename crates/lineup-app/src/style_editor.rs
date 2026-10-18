//! Style drafting reuses the channel rack and piano roll. Its history is
//! independent of the song until the user applies the saved style.

use crate::{
    channel_rack::{self, RackAction},
    piano_roll::{PianoRoll, RollAction},
    widgets,
};
use eframe::egui::{self, Key, Modifiers, Rect};
use lineup_core::library::{STYLES, StyleLane, UserStyle};
use lineup_core::{Channel, ChannelKind, History, MAX_STEPS, Note, Player, Project, Tempo};
use std::path::PathBuf;

pub fn window_id() -> egui::Id {
    egui::Id::unique("style-editor")
}
fn roll_id() -> egui::Id {
    egui::Id::unique("style-editor-piano-roll")
}

#[derive(Clone, PartialEq)]
struct Draft {
    style: UserStyle,
    project: Project,
    roles: Vec<Player>,
}

pub struct StyleEditor {
    pub open: bool,
    draft: Draft,
    roll: PianoRoll,
    history: History<Draft>,
    pub path: Option<PathBuf>,
    saved: UserStyle,
    source_filter: String,
    preview_started: Option<(f64, f64)>,
    pub error: Option<String>,
    save_requested: bool,
    save_as_requested: bool,
    preview_requested: bool,
    pending_start: Option<(UserStyle, Option<PathBuf>)>,
}

pub enum EditorEvent {
    Save {
        style: UserStyle,
        save_as: bool,
    },
    Preview(Box<Project>),
    Hear {
        channel: Channel,
        pitches: Vec<Note>,
    },
    Stop,
    StartFrom(UserStyle),
}

impl StyleEditor {
    pub fn has_unsaved_changes(&self) -> bool {
        self.open && self.draft.style != self.saved
    }
    pub fn current_style(&self) -> UserStyle {
        self.draft.style.clone()
    }
    pub fn discard_edits(&mut self) {
        let style = self.saved.clone();
        let path = self.path.clone();
        self.open = false;
        self.start(style, path);
        self.open = false;
        self.roll.open = false;
    }
    pub fn new() -> Self {
        let style = UserStyle::blank();
        let project = style.pattern_project();
        let roles = style.lanes.iter().map(|lane| lane.player).collect();
        let mut roll = PianoRoll::new(0);
        roll.instance = Some(roll_id());
        Self {
            open: false,
            draft: Draft {
                style: style.clone(),
                project,
                roles,
            },
            roll,
            history: History::default(),
            path: None,
            saved: style,
            source_filter: String::new(),
            preview_started: None,
            error: None,
            save_requested: false,
            save_as_requested: false,
            preview_requested: false,
            pending_start: None,
        }
    }
    pub fn start(&mut self, style: UserStyle, path: Option<PathBuf>) {
        if self.open && self.draft.style != self.saved {
            self.pending_start = Some((style, path));
            return;
        }
        self.draft = Draft {
            project: style.pattern_project(),
            roles: style.lanes.iter().map(|lane| lane.player).collect(),
            style: style.clone(),
        };
        self.saved = style;
        self.path = path;
        self.history = History::default();
        self.roll.open = false;
        self.preview_started = None;
        self.error = None;
        self.open = true;
        self.pending_start = None;
    }
    pub fn saved(&mut self, path: PathBuf) {
        self.saved = self.draft.style.clone();
        self.path = Some(path);
        self.error = None;
    }
    fn active(&self, ctx: &egui::Context) -> bool {
        self.open
            && [window_id(), roll_id()]
                .iter()
                .any(|id| widgets::editor_active(ctx, egui::LayerId::new(egui::Order::Middle, *id)))
    }
    /// Read before the song's global file/edit shortcuts.
    pub fn shortcuts(&mut self, ctx: &egui::Context) {
        if !self.active(ctx) {
            return;
        }
        let pressed = |key, modifiers| ctx.input_mut(|input| input.consume_key(modifiers, key));
        if pressed(Key::S, Modifiers::COMMAND | Modifiers::SHIFT) {
            self.save_requested = true;
            self.save_as_requested = true;
        } else if pressed(Key::S, Modifiers::COMMAND) {
            self.save_requested = true;
        }
        if ctx.text_edit_focused() {
            return;
        }
        if pressed(Key::Space, Modifiers::NONE) {
            self.preview_requested = true;
        }
        if pressed(Key::Z, Modifiers::COMMAND | Modifiers::SHIFT)
            || pressed(Key::Y, Modifiers::COMMAND)
        {
            self.history.redo(&mut self.draft);
        } else if pressed(Key::Z, Modifiers::COMMAND) {
            self.history.undo(&mut self.draft);
        }
    }
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        area: Rect,
        user_styles: &[UserStyle],
    ) -> Vec<EditorEvent> {
        if !self.open {
            return Vec::new();
        }
        let before = self.draft.clone();
        let mut events = Vec::new();
        if self.preview_requested {
            self.preview_requested = false;
            let period = self.draft.project.pattern(0).unwrap().length() as f64 * 60.0
                / f64::from(self.draft.project.tempo.bpm())
                / 4.0;
            self.preview_started = Some((ctx.input(|input| input.time), period));
            events.push(EditorEvent::Preview(Box::new(self.draft.project.clone())));
        }
        if self.pending_start.is_some() {
            let mut discard = false;
            egui::Window::new("Replace unsaved style?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("The current style has unsaved edits.");
                    ui.horizontal(|ui| {
                        if ui.button("Keep editing").clicked() {
                            self.pending_start = None;
                        }
                        if ui.button("Discard and start").clicked() {
                            discard = true;
                        }
                    });
                });
            if discard && let Some((style, path)) = self.pending_start.take() {
                self.open = false;
                self.start(style, path);
                events.push(EditorEvent::Stop);
            }
        }
        let mut open = self.open;
        let mut history_action = false;
        let dirty = self.draft.style != self.saved;
        let title = if dirty {
            "Style editor · channel rack *"
        } else {
            "Style editor · channel rack"
        };
        egui::Window::new(title).id(window_id()).open(&mut open)
            .default_size([950.0, 530.0]).constrain_to(area).show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let menu = egui::containers::menu::MenuButton::new("Start from…").config(
                        egui::containers::menu::MenuConfig::new().close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside));
                    menu.ui(ui, |ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.source_filter).hint_text("Search styles"));
                        egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                            if ui.button("Empty style").clicked() { events.push(EditorEvent::StartFrom(UserStyle::blank())); ui.close(); }
                            for style in user_styles.iter().filter(|style| style.name.to_lowercase().contains(&self.source_filter.to_lowercase())) {
                                if ui.button(&style.name).clicked() { let mut copy = style.clone(); copy.name.push_str(" copy"); events.push(EditorEvent::StartFrom(copy)); ui.close(); }
                            }
                            for style in crate::library_browser::catalogue::alphabetical(STYLES.iter().filter(|style| style.name.to_lowercase().contains(&self.source_filter.to_lowercase())), |style| style.name) {
                                if ui.button(style.name).clicked() { events.push(EditorEvent::StartFrom(UserStyle::from_builtin(style))); ui.close(); }
                            }
                        });
                    });
                    if ui.button("Save style").clicked() { self.save_requested = true; }
                    if ui.button("Save as…").clicked() { events.push(EditorEvent::Save { style: self.draft.style.clone(), save_as: true }); }
                    if ui.add_enabled(self.history.undo_label().is_some(), egui::Button::new("Undo")).clicked() { self.history.undo(&mut self.draft); history_action = true; }
                    if ui.add_enabled(self.history.redo_label().is_some(), egui::Button::new("Redo")).clicked() { self.history.redo(&mut self.draft); history_action = true; }
                    if ui.button("Preview ×4").clicked() {
                        let seconds = self.draft.project.pattern(0).unwrap().length() as f64 * 60.0 / f64::from(self.draft.project.tempo.bpm()) / 4.0;
                        self.preview_started = Some((ctx.input(|input| input.time), seconds));
                        events.push(EditorEvent::Preview(Box::new(self.draft.project.clone())));
                    }
                    if ui.button("Stop preview").clicked() { events.push(EditorEvent::Stop); self.preview_started = None; }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label("Name"); ui.text_edit_singleline(&mut self.draft.style.name);
                    let mut tempo = self.draft.project.tempo.bpm();
                    ui.label("BPM"); if ui.add(egui::DragValue::new(&mut tempo).range(Tempo::MIN..=Tempo::MAX)).changed() { self.draft.project.tempo = Tempo::new(tempo); }
                    let time = self.draft.project.time_signature;
                    egui::ComboBox::from_id_salt("style-time").selected_text(time.to_string()).show_ui(ui, |ui| {
                        if let Some(time) = crate::time_picker::time_picker(ui, egui::Id::unique("style-meter"), time, None) { self.draft.project.time_signature = time; }
                    });
                    let max_bars = (MAX_STEPS / self.draft.project.time_signature.steps_per_bar()).clamp(1,16);
                    self.draft.style.bars = self.draft.style.bars.clamp(1,max_bars);
                    ui.label("Bars"); ui.add(egui::DragValue::new(&mut self.draft.style.bars).range(1..=max_bars));
                    ui.label("Swing"); ui.add(egui::DragValue::new(&mut self.draft.project.swing).range(0..=lineup_core::MAX_SWING).suffix("%"));
                });
                let length = self.draft.style.bars * self.draft.project.time_signature.steps_per_bar();
                self.draft.project.pattern_mut(0).unwrap().set_length(length);
                ui.horizontal(|ui| { ui.label("Progression"); ui.text_edit_singleline(&mut self.draft.style.chart).on_hover_text("Roman numerals or chord names; separate bars with | (I | vi | ii7 | V7)"); });
                ui.horizontal(|ui| { ui.label("Description"); ui.text_edit_singleline(&mut self.draft.style.about); });
                ui.collapsing("Part assignments", |ui| {
                    egui::ScrollArea::vertical().max_height(110.0).show(ui, |ui| {
                        for (index, channel) in self.draft.project.channels().iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(&channel.name);
                                egui::ComboBox::from_id_salt(("style-role",index)).selected_text(self.draft.roles[index].name()).show_ui(ui, |ui| {
                                    for player in Player::ALL { ui.selectable_value(&mut self.draft.roles[index], player, player.name()); }
                                });
                            });
                        }
                    });
                });
                ui.weak("Write pitched parts over C major. They follow the song’s chords when applied. Click steps to edit drums; click a note preview for the piano roll.");
                if let Some(error) = &self.error { ui.colored_label(ui.visuals().error_fg_color, error); }
                let playhead = self.preview_started.and_then(|(start, period)| {
                    let elapsed = ctx.input(|input| input.time)-start;
                    if elapsed < period * 4.0 { ctx.request_repaint(); Some(((elapsed%period)/period * length as f64) as usize) } else { None }
                });
                for action in channel_rack::show(ui, &self.draft.project, 0, self.roll.channel(), playhead) { self.rack_action(action, &mut events); }
            });
        for action in self.roll.show(
            ctx,
            &self.draft.project,
            0,
            None,
            &[],
            area,
            egui::Vec2::splat(1.0),
        ) {
            self.roll_action(action, &mut events);
        }
        self.sync();
        if self.save_requested {
            self.save_requested = false;
            // The Save event below carries the requested destination mode.
            events.push(EditorEvent::Save {
                style: self.draft.style.clone(),
                save_as: self.save_as_requested,
            });
        }
        self.save_as_requested = false;
        // Saving should capture edits made by any widget in this same frame.
        for event in &mut events {
            if let EditorEvent::Save { style, .. } = event {
                *style = self.draft.style.clone();
            }
        }
        if !history_action && before != self.draft {
            self.history.record("edit style", before);
        }
        if !ctx.input(|input| input.pointer.primary_down()) && !ctx.text_edit_focused() {
            self.history.close();
        }
        if !open {
            if self.draft.style != self.saved {
                self.error =
                    Some("Unsaved edits. Save the style, or use Discard edits to close.".into());
            } else {
                self.open = false;
                self.roll.open = false;
                events.push(EditorEvent::Stop);
            }
        }
        if self
            .error
            .as_deref()
            .is_some_and(|error| error.starts_with("Unsaved edits"))
        {
            egui::Window::new("Unsaved style")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Save your style before closing?");
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            self.save_requested = true;
                            self.error = None;
                        }
                        if ui.button("Keep editing").clicked() {
                            self.error = None;
                        }
                        if ui.button("Discard edits").clicked() {
                            self.open = false;
                            self.roll.open = false;
                            self.error = None;
                            events.push(EditorEvent::Stop);
                        }
                    });
                });
        }
        events
    }
    fn sync(&mut self) {
        let draft = &mut self.draft;
        draft.style.time = draft.project.time_signature;
        draft.style.tempo = draft.project.tempo;
        draft.style.swing = draft.project.swing;
        draft.style.lanes = draft
            .project
            .channels()
            .iter()
            .enumerate()
            .map(|(index, channel)| StyleLane {
                player: draft.roles[index],
                channel: channel.clone(),
                notes: draft
                    .project
                    .pattern(0)
                    .unwrap()
                    .notes(index)
                    .iter()
                    .copied()
                    .filter(|note| note.start < draft.style.length())
                    .map(|mut note| {
                        if note.end() > draft.style.length() {
                            note.length = draft.style.length() - note.start;
                            note.trim = 0;
                        }
                        note
                    })
                    .collect(),
            })
            .collect();
    }
    fn rack_action(&mut self, action: RackAction, events: &mut Vec<EditorEvent>) {
        let project = &mut self.draft.project;
        match action {
            RackAction::ToggleStep { channel, step } => {
                project.pattern_mut(0).unwrap().toggle(channel, step);
            }
            RackAction::ClearStep { channel, step } => {
                project.pattern_mut(0).unwrap().set(channel, step, false);
            }
            RackAction::Preview(index) => {
                self.roll.select(index);
                events.push(EditorEvent::Hear {
                    channel: project.channels()[index].clone(),
                    pitches: vec![lineup_core::STEP_NOTE],
                });
            }
            RackAction::SetVolume { channel, volume } => {
                if let Some(state) = project.channel_mut(channel) {
                    state.volume = volume;
                }
            }
            RackAction::ToggleMute(channel) => {
                if let Some(state) = project.channel_mut(channel) {
                    state.muted = !state.muted;
                }
            }
            RackAction::Remove(channel) => {
                project.remove_channel(channel);
                self.draft.roles.remove(channel);
            }
            RackAction::OpenPianoRoll(channel) => self.roll.open_for(channel),
            RackAction::AddDrum(drum) => {
                if project.add_channel(Channel::drum(drum)).is_some() {
                    self.draft.roles.push(Player::Drums);
                }
            }
            RackAction::AddSynth | RackAction::AddMidiOut => {
                let role = Player::ALL
                    .into_iter()
                    .find(|player| *player != Player::Drums && !self.draft.roles.contains(player))
                    .unwrap_or(Player::Piano);
                if project.add_channel(Channel::synth()).is_some() {
                    self.draft.roles.push(role);
                }
            }
            RackAction::SetProgram { channel, program } => {
                if let Some(state) = project.channel_mut(channel) {
                    state.program = program;
                }
            }
            RackAction::SetMidiChannel {
                channel,
                midi_channel,
            } => {
                if let Some(state) = project.channel_mut(channel) {
                    state.kind = ChannelKind::MidiOut {
                        channel: midi_channel,
                    };
                }
            }
            RackAction::LoadSample { channel, path } => {
                let index = channel.or_else(|| {
                    let index = project.add_channel(Channel::synth())?;
                    self.draft.roles.push(Player::Drums);
                    Some(index)
                });
                if let Some(state) = index.and_then(|index| project.channel_mut(index)) {
                    state.kind = ChannelKind::Sampler(lineup_core::SampleSource::File(path));
                }
            }
            RackAction::SelectPattern(_) => {}
        }
    }
    fn roll_action(&mut self, action: RollAction, events: &mut Vec<EditorEvent>) {
        let project = &mut self.draft.project;
        match action {
            RollAction::SetLength(length) => {
                let bar = project.time_signature.steps_per_bar();
                self.draft.style.bars = length.div_ceil(bar).min(16);
                project
                    .pattern_mut(0)
                    .unwrap()
                    .set_length(self.draft.style.bars * bar);
            }
            RollAction::AddNotes { channel, notes } => {
                for note in notes {
                    project.pattern_mut(0).unwrap().add_note(channel, note);
                }
            }
            RollAction::RemoveNote { channel, index } => {
                project.pattern_mut(0).unwrap().remove_note(channel, index);
            }
            RollAction::ReplaceNote {
                channel,
                index,
                note,
            } => {
                project
                    .pattern_mut(0)
                    .unwrap()
                    .replace_note(channel, index, note);
            }
            RollAction::ReplaceNotes {
                channel, old, new, ..
            } => {
                project
                    .pattern_mut(0)
                    .unwrap()
                    .replace_notes(channel, &old, &new);
            }
            RollAction::Preview { channel, pitches } => {
                events.push(EditorEvent::Hear {
                    channel: project.channels()[channel].clone(),
                    pitches,
                });
            }
            RollAction::SetScale(scale) => {
                project.scale = scale;
            }
            RollAction::OpenRack | RollAction::SelectPattern(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Event, FullOutput, PointerButton, RawInput, Shape};

    fn frame(
        editor: &mut StyleEditor,
        ctx: &egui::Context,
        events: Vec<Event>,
    ) -> (FullOutput, Vec<EditorEvent>) {
        let mut actions = Vec::new();
        let mut output = ctx.run_pass(
            RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                time: Some(ctx.cumulative_frame_nr() as f64 * 0.1),
                events,
                ..Default::default()
            },
            |ctx| {
                editor.shortcuts(ctx);
                actions = editor.show(ctx, ctx.content_rect(), &[]);
            },
        );
        output.textures_delta.clear();
        (output, actions)
    }
    fn click(
        editor: &mut StyleEditor,
        ctx: &egui::Context,
        output: &FullOutput,
        label: &str,
    ) -> Vec<EditorEvent> {
        let pos = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                Shape::Text(text) if text.galley.text() == label => {
                    Some(text.pos + text.galley.size() * 0.5)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("Missing {label}"));
        frame(
            editor,
            ctx,
            vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
        frame(
            editor,
            ctx,
            vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            }],
        )
        .1
    }
    #[test]
    fn style_window_previews_and_saves_the_editable_pattern() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let style = UserStyle::from_builtin(&STYLES[0]);
        let mut editor = StyleEditor::new();
        editor.start(style.clone(), None);
        frame(&mut editor, &ctx, vec![]);
        let (output, _) = frame(&mut editor, &ctx, vec![]);
        let events = click(&mut editor, &ctx, &output, "Preview ×4");
        assert!(events.iter().any(|event| matches!(event,EditorEvent::Preview(project) if project.pattern(0).unwrap().all_notes().count()>0)));
        let (output, _) = frame(&mut editor, &ctx, vec![]);
        click(&mut editor, &ctx, &output, &style.name);
        frame(
            &mut editor,
            &ctx,
            vec![Event::Key {
                key: Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::COMMAND,
            }],
        );
        frame(&mut editor, &ctx, vec![Event::Text("Edited style".into())]);
        let (_, events) = frame(
            &mut editor,
            &ctx,
            vec![Event::Key {
                key: Key::S,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::COMMAND,
            }],
        );
        assert!(events.iter().any(|event| matches!(event,EditorEvent::Save {style,..} if style.name == "Edited style" && style.validate().is_ok())));
    }
}
