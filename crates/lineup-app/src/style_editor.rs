//! Style drafting reuses the channel rack and piano roll. Its history is
//! independent of the song until the user applies the saved style.

use crate::i18n::{tr, tr_name};
use crate::{
    channel_rack::{self, RackAction},
    piano_roll::{PianoRoll, RollAction},
    widgets,
};
use eframe::egui::{self, Key, Modifiers, Rect};
use lineup_core::library::{StyleLane, UserStyle};
use lineup_core::{Channel, ChannelKind, History, MAX_STEPS, Note, Player, Project, Tempo};
use std::path::PathBuf;

/// What the editor says when it is closed with edits not saved.
const UNSAVED: &str = "Unsaved edits. Save the style, or use Discard edits to close.";

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
    /// How tall the fields over the rows were last drawn.
    header: f32,
    /// The size of the contents the window was last fitted to.
    fitted: egui::Vec2,
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
    /// Send every MIDI Out channel to this output port, or to none.
    ConnectMidiOutput(Option<crate::midi::Port>),
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
        self.roll = PianoRoll::new(0);
        self.roll.instance = Some(roll_id());
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
            header: 0.0,
            fitted: egui::Vec2::ZERO,
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
        self.roll = PianoRoll::new(0);
        self.roll.instance = Some(roll_id());
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
        if !self.active(ctx) || egui::Popup::is_any_open(ctx) || widgets::modal_open(ctx) {
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
        if widgets::editor_keyboard(ctx, egui::LayerId::new(egui::Order::Middle, window_id()))
            && pressed(Key::Escape, Modifiers::NONE)
        {
            self.roll.deselect_channel();
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
        let mut before = self.draft.clone();
        let mut events = Vec::new();
        if self
            .preview_started
            .is_some_and(|(start, period)| ctx.input(|input| input.time) >= start + period * 4.0)
        {
            self.preview_started = None;
        }
        if self.preview_requested {
            self.preview_requested = false;
            if self.preview_started.take().is_some() {
                events.push(EditorEvent::Stop);
            } else {
                let period = self.draft.project.pattern(0).unwrap().length() as f64 * 60.0
                    / f64::from(self.draft.project.tempo.bpm())
                    / 4.0;
                self.preview_started = Some((ctx.input(|input| input.time), period));
                events.push(EditorEvent::Preview(Box::new(self.draft.project.clone())));
            }
        }
        if self.pending_start.is_some() {
            let mut discard = false;
            let modal =
                egui::Modal::new(egui::Id::unique("replace-unsaved-style")).show(ctx, |ui| {
                    ui.heading(tr("Replace unsaved style?"));
                    ui.label(tr("The current style has unsaved edits."));
                    ui.horizontal(|ui| {
                        if ui.button(tr("Keep editing")).clicked() {
                            self.pending_start = None;
                        }
                        if ui.button(tr("Discard and start")).clicked() {
                            discard = true;
                        }
                    });
                });
            if modal.should_close() {
                self.pending_start = None;
            }
            if discard && let Some((style, path)) = self.pending_start.take() {
                self.open = false;
                self.start(style, path);
                // The discarded draft is not an edit of the new style.
                before = self.draft.clone();
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
        // The editor opens as big as its rows and the fields over them, up
        // to the size of the playlist, and is fitted to them again when
        // they change (a longer pattern, a new channel), as the channel
        // rack is. The rack draws no play buttons here: one line less.
        let style = ctx.global_style();
        let line = style.spacing.interact_size.y + style.spacing.item_spacing.y;
        let natural = channel_rack::natural_size(&style, &self.draft.project, 0, 1.0)
            + egui::vec2(0.0, self.header - line);
        let mut window = egui::Window::new(crate::i18n::tr(title))
            .id(window_id())
            .open(&mut open)
            .constrain_to(area);
        if natural != self.fitted || widgets::opens(ctx, window_id()) {
            ctx.request_discard("the style editor is fitted to its rows");
            window = window.fixed_size(widgets::window_size(ctx, natural, area));
            self.fitted = natural;
        }
        window.show(ctx, |ui| {
                let top = ui.cursor().top();
                ui.horizontal_wrapped(|ui| {
                    let menu = egui::containers::menu::MenuButton::new(tr("Start from…")).config(
                        egui::containers::menu::MenuConfig::new().close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside));
                    menu.ui(ui, |ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.source_filter).hint_text(tr("Search presets")));
                        egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                            if ui.button(tr("Empty style")).clicked() { events.push(EditorEvent::StartFrom(UserStyle::blank())); ui.close(); }
                            for preset in crate::library_browser::catalogue::presets(&self.source_filter, user_styles, false) {
                                if ui.button(tr_name(preset.name())).clicked() {
                                    use crate::library_browser::catalogue::Preset;
                                    let copy = match preset {
                                        Preset::Style(style) => UserStyle::from_builtin(style),
                                        Preset::Rhythm(rhythm) => UserStyle::from_rhythm(rhythm),
                                        Preset::User(_, style) => { let mut copy = style.clone(); copy.name.push_str(" copy"); copy },
                                    };
                                    events.push(EditorEvent::StartFrom(copy)); ui.close();
                                }
                            }
                        });
                    });
                    if ui.button(tr("Save style")).clicked() { self.save_requested = true; }
                    if ui.button(tr("Save as…")).clicked() { events.push(EditorEvent::Save { style: self.draft.style.clone(), save_as: true }); }
                    if ui.add_enabled(self.history.undo_label().is_some(), egui::Button::new(tr("Undo"))).clicked() { self.history.undo(&mut self.draft); history_action = true; }
                    if ui.add_enabled(self.history.redo_label().is_some(), egui::Button::new(tr("Redo"))).clicked() { self.history.redo(&mut self.draft); history_action = true; }
                    if ui.button(tr("Preview ×4")).clicked() {
                        let seconds = self.draft.project.pattern(0).unwrap().length() as f64 * 60.0 / f64::from(self.draft.project.tempo.bpm()) / 4.0;
                        self.preview_started = Some((ctx.input(|input| input.time), seconds));
                        events.push(EditorEvent::Preview(Box::new(self.draft.project.clone())));
                    }
                    if ui.add_enabled(self.preview_started.is_some(), egui::Button::new(tr("Stop preview"))).clicked() { events.push(EditorEvent::Stop); self.preview_started = None; }
                });
                // On one line, so the window is fitted to be as wide.
                ui.horizontal(|ui| {
                    ui.label(tr("Name")); ui.text_edit_singleline(&mut self.draft.style.name);
                    let mut tempo = self.draft.project.tempo.bpm();
                    ui.label(tr("BPM")); if ui.add(egui::DragValue::new(&mut tempo).range(Tempo::MIN..=Tempo::MAX)).changed() { self.draft.project.tempo = Tempo::new(tempo); }
                    let time = self.draft.project.time_signature;
                    egui::ComboBox::from_id_salt("style-time").selected_text(time.to_string()).show_ui(ui, |ui| {
                        if let Some(time) = crate::time_picker::time_picker(ui, egui::Id::unique("style-meter"), time, None) { self.draft.project.time_signature = time; }
                    });
                    let max_bars = (MAX_STEPS / self.draft.project.time_signature.steps_per_bar()).clamp(1,16);
                    self.draft.style.bars = self.draft.style.bars.clamp(1,max_bars);
                    ui.label(tr("Bars")); ui.add(egui::DragValue::new(&mut self.draft.style.bars).range(1..=max_bars));
                    ui.label(tr("Swing")); ui.add(egui::DragValue::new(&mut self.draft.project.swing).range(0..=lineup_core::MAX_SWING).suffix("%"));
                });
                let length = self.draft.style.bars * self.draft.project.time_signature.steps_per_bar();
                self.draft.project.pattern_mut(0).unwrap().set_length(length);
                ui.horizontal(|ui| { ui.label(tr("Progression")); ui.text_edit_singleline(&mut self.draft.style.chart).on_hover_text(tr("Roman numerals or chord names; separate bars with | (I | vi | ii7 | V7). In a minor key, write numerals from the major scale, as the library does: i bVI bIII bVII.")); });
                ui.horizontal(|ui| { ui.label(tr("Description")); ui.text_edit_singleline(&mut self.draft.style.about); });
                ui.collapsing(tr("Part assignments"), |ui| {
                    egui::ScrollArea::vertical().max_height(110.0).show(ui, |ui| {
                        for (index, channel) in self.draft.project.channels().iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(tr_name(&channel.name));
                                egui::ComboBox::from_id_salt(("style-role",index)).selected_text(tr(self.draft.roles[index].name())).show_ui(ui, |ui| {
                                    for player in Player::ALL { ui.selectable_value(&mut self.draft.roles[index], player, tr(player.name())); }
                                });
                            });
                        }
                    });
                });
                ui.weak(tr("Write pitched parts over C major. They follow the song’s chords when applied. Click steps to edit drums; click a note preview for the piano roll."));
                if let Some(error) = &self.error { ui.colored_label(ui.visuals().error_fg_color, error); }
                let playhead = self.preview_started.and_then(|(start, period)| {
                    let elapsed = ctx.input(|input| input.time)-start;
                    if elapsed < period * 4.0 { ctx.request_repaint(); Some(((elapsed%period)/period * length as f64) as usize) } else { None }
                });
                self.header = ui.cursor().top() - top;
                for action in channel_rack::show(ui, &self.draft.project, 0, self.roll.selected_channel(), playhead, None, false) { self.rack_action(action, &mut events); }
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
                self.error = Some(tr(UNSAVED).into());
            } else {
                self.open = false;
                self.roll.open = false;
                events.push(EditorEvent::Stop);
            }
        }
        if self
            .error
            .as_deref()
            .is_some_and(|error| error == tr(UNSAVED))
        {
            let modal = egui::Modal::new(egui::Id::unique("close-unsaved-style")).show(ctx, |ui| {
                ui.heading(tr("Unsaved style"));
                ui.label(tr("Save your style before closing?"));
                ui.horizontal(|ui| {
                    if ui.button(tr("Save")).clicked() {
                        self.save_requested = true;
                        self.error = None;
                    }
                    if ui.button(tr("Keep editing")).clicked() {
                        self.error = None;
                    }
                    if ui.button(tr("Discard edits")).clicked() {
                        self.open = false;
                        self.roll.open = false;
                        self.error = None;
                        events.push(EditorEvent::Stop);
                    }
                });
            });
            if modal.should_close() {
                self.error = None;
            }
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
        if action == RackAction::RemoveChosen {
            if let Some(channel) = self.roll.chosen_channel() {
                self.rack_action(RackAction::Remove(channel), events);
            }
            return;
        }
        let project = &mut self.draft.project;
        match action {
            // A style's rack has no locks.
            RackAction::RemoveChosen | RackAction::SetLocked { .. } => {}
            RackAction::SelectChannel(channel) => match channel {
                Some(channel) => self.roll.select(channel),
                None => self.roll.deselect_channel(),
            },
            RackAction::PlayPattern => self.preview_requested = true,
            RackAction::PausePattern | RackAction::StopPattern => {
                self.preview_started = None;
                events.push(EditorEvent::Stop);
            }
            RackAction::ToggleStep { channel, step } => {
                project.pattern_mut(0).unwrap().toggle(channel, step);
            }
            RackAction::ClearStep { channel, step } => {
                project.pattern_mut(0).unwrap().set(channel, step, false);
            }
            RackAction::Preview(index) => {
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
                self.roll.channel_removed(channel, project.channels().len());
            }
            RackAction::OpenPianoRoll(channel) => self.roll.open_for(channel),
            RackAction::AddDrum(drum) => {
                if let Some(channel) = project.add_channel(Channel::drum(drum)) {
                    self.draft.roles.push(Player::Drums);
                    self.roll.select(channel);
                }
            }
            RackAction::AddSynth | RackAction::AddMidiOut => {
                let role = Player::ALL
                    .into_iter()
                    .find(|player| *player != Player::Drums && !self.draft.roles.contains(player))
                    .unwrap_or(Player::Piano);
                let instrument = if matches!(action, RackAction::AddMidiOut) {
                    Channel::midi_out(0)
                } else {
                    Channel::synth()
                };
                if let Some(channel) = project.add_channel(instrument) {
                    self.draft.roles.push(role);
                    self.roll.select(channel);
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
                    state.send_over_midi(midi_channel);
                }
            }
            RackAction::PlayHere(channel) => {
                if let Some(state) = project.channel_mut(channel) {
                    state.play_here();
                }
            }
            RackAction::ConnectMidiOutput(port) => {
                events.push(EditorEvent::ConnectMidiOutput(port))
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
                if let Some(index) = index {
                    self.roll.select(index);
                }
            }
            RackAction::SelectPattern(_) | RackAction::PlayOverSong(_) => {}
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
            RollAction::NewPattern { .. } => {
                self.error = Some(
                    tr("Styles contain a single pattern. Save new patterns from the song piano roll.")
                        .into(),
                );
            }
            RollAction::OpenRack | RollAction::SelectPattern(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_selection_survives_removal_and_new_styles_reset_it() {
        let mut editor = StyleEditor::new();
        editor.start(UserStyle::blank(), None);
        let mut events = Vec::new();
        editor.rack_action(RackAction::SelectChannel(Some(3)), &mut events);
        editor.rack_action(RackAction::Remove(0), &mut events);
        assert_eq!(editor.roll.selected_channel(), Some(2));
        editor.rack_action(RackAction::Remove(2), &mut events);
        assert_eq!(editor.roll.selected_channel(), None);
        editor.rack_action(RackAction::AddSynth, &mut events);
        assert_eq!(editor.roll.selected_channel(), Some(2));
        editor.open = false;
        editor.start(UserStyle::blank(), None);
        assert_eq!(editor.roll.selected_channel(), Some(0));
        assert!(!editor.roll.open);
    }
    use egui::{Event, FullOutput, PointerButton, RawInput, Shape};
    use lineup_core::library::STYLES;

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

    #[test]
    fn discarding_a_draft_leaves_nothing_of_it_to_undo() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut editor = StyleEditor::new();
        editor.start(UserStyle::blank(), None);
        frame(&mut editor, &ctx, vec![]);
        editor.draft.style.about = "Unsaved description".into();
        frame(&mut editor, &ctx, vec![]);
        let opened = UserStyle::from_builtin(&STYLES[0]);
        editor.start(opened.clone(), None);
        frame(&mut editor, &ctx, vec![]);
        let (output, _) = frame(&mut editor, &ctx, vec![]);
        click(&mut editor, &ctx, &output, "Discard and start");
        assert_eq!(editor.draft.style, opened);
        assert!(editor.history.undo_label().is_none(), "nothing to undo");
    }

    #[test]
    fn style_confirmations_block_draft_shortcuts_and_escape_keeps_edits() {
        for replace in [false, true] {
            let ctx = egui::Context::default();
            let mut editor = StyleEditor::new();
            editor.start(UserStyle::blank(), None);
            editor.roll.deselect_channel();
            frame(&mut editor, &ctx, vec![]);
            let (output, _) = frame(&mut editor, &ctx, vec![]);
            let channel = editor.draft.project.channels()[0].name.clone();
            click(&mut editor, &ctx, &output, &channel);
            editor.draft.style.about = "Unsaved description".into();
            if replace {
                editor.start(UserStyle::blank(), None);
            } else {
                editor.error = Some(UNSAVED.into());
            }
            frame(&mut editor, &ctx, vec![]);
            let before = editor.draft.clone();
            for key in [Key::Space, Key::Delete] {
                let (_, events) = frame(
                    &mut editor,
                    &ctx,
                    vec![Event::Key {
                        key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::NONE,
                    }],
                );
                assert!(
                    !events
                        .iter()
                        .any(|event| matches!(event, EditorEvent::Preview(_)))
                );
                assert!(editor.draft == before);
            }
            frame(
                &mut editor,
                &ctx,
                vec![Event::Key {
                    key: Key::Escape,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                }],
            );
            assert!(editor.pending_start.is_none() && editor.error.is_none());
            assert!(editor.open && editor.draft == before);
        }
    }
}
