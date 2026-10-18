//! Music without a key: the twelve-tone row window and the set browser.
//!
//! The key menus offer two kinds of "key" besides the scales: a
//! twelve-tone row and a pitch-class set. The row window shows a row's
//! 48 forms in its matrix and writes any of them into the pattern; the
//! set browser lists all 224 set classes and stamps one into the piano
//! roll. The theory is in `lineup_core::harmony` (`ToneRow`, `SetClass`)
//! and `lineup_core::serial`; this module only draws and reads the mouse.
//!
//! The row and the set last used are remembered in egui's memory, so a
//! key menu can switch back to them after the key was a scale for a while.

use eframe::egui::{self, Color32, RichText, Ui};
use lineup_core::harmony::{Micromode, micromodes_in};
use lineup_core::harmony::{
    ROW_PRESETS, RowForm, RowLabel, RowPreset, SetClass, ToneRow, digits, pc_digit,
};
use lineup_core::serial::{Chain, Register, RowWriting, write_row};
use lineup_core::{
    Chord, Note, NoteEvent, NoteName, PitchClass, PitchSet, Scale, ScaleKind, Spelling,
};

/// Where the row and set last used are remembered.
fn row_id() -> egui::Id {
    egui::Id::unique("lineup-last-row")
}

fn set_id() -> egui::Id {
    egui::Id::unique("lineup-last-set")
}

/// The row written for Ferrous Lineup, which the row window starts with.
fn lineup_row() -> ToneRow {
    RowPreset::named("Lineup row").map_or(ToneRow::CHROMATIC, |preset| preset.row)
}

/// The row a key menu switches to: the last one used, or the Lineup row.
pub fn remembered_row(ctx: &egui::Context) -> ToneRow {
    ctx.data(|data| data.get_temp(row_id()))
        .unwrap_or_else(lineup_row)
}

/// The set a key menu switches to: the last one used, or the diminished
/// seventh chord.
pub fn remembered_set(ctx: &egui::Context) -> PitchSet {
    ctx.data(|data| data.get_temp(set_id()))
        .unwrap_or(PitchSet::from_semitones(&[0, 3, 6, 9]))
}

/// The key a key menu switches to when "pitch-class set" is picked
/// while the key is `scale`, on `root`. The notes of the song stay as
/// they are, so the set is the one they were already in: the notes the
/// harmony keys were playing, not a set from somewhere else.
///
/// - From a key, it is the key's own scale: C major is the set 7-35 on
///   C, and its keys are still C, Dm, Em, F, G, Am and Bdim, named as
///   sets too (3-11, 3-10).
/// - From a row, it is the row's first group of three, where it is
///   (Webern's Concerto row on C gives C B Eb, 3-3).
/// - From the chromatic scale, which holds every note and so says
///   nothing about the harmony, it is the set used last, moved so it
///   starts on `root`.
fn set_key(ctx: &egui::Context, scale: Scale, root: PitchClass) -> Scale {
    let up = scale.root.semitones_up_to(root);
    if let Some(group) = scale
        .tone_row()
        .and_then(|row| row.segments(3).first().copied())
    {
        return Scale::set(root, group.transpose(up));
    }
    if scale.kind.is_tonal() && scale.kind != ScaleKind::Chromatic {
        return Scale::set(root, scale.pitch_set().transpose(up));
    }
    let set = remembered_set(ctx);
    let start = set.normal_form().first().copied().unwrap_or(root);
    Scale::set(root, set.transpose(start.semitones_up_to(root)))
}

/// Remembers the row or set of `scale`, if it is one.
pub fn remember(ctx: &egui::Context, scale: Scale) {
    match scale.kind {
        ScaleKind::Row(_) => {
            if let Some(row) = scale.tone_row() {
                ctx.data_mut(|data| data.insert_temp(row_id(), row));
            }
        }
        ScaleKind::Set(set) => {
            let absolute = set.transpose(scale.root.semitone());
            ctx.data_mut(|data| data.insert_temp(set_id(), absolute));
        }
        _ => {}
    }
}

/// The ids of the two windows, to keep them in front (see [`Front`]).
pub(crate) fn row_window_id() -> egui::Id {
    egui::Id::unique("twelve-tone-row-window")
}

pub(crate) fn set_window_id() -> egui::Id {
    egui::Id::unique("pitch-class-set-window")
}

/// Keeps a window in front of the others for its first frames after it
/// opens. egui puts a window that opens on top, but the window whose
/// button opened it was clicked in the same frame and wants the top too,
/// and then the new one can stay behind it.
#[derive(Default)]
pub(crate) struct Front {
    /// The frame the window was last drawn in.
    drawn_in: u64,
    /// The frame it opened in.
    opened_in: u64,
}

impl Front {
    /// Called each frame the window `id` is drawn, before it is.
    pub(crate) fn keep(&mut self, ctx: &egui::Context, id: egui::Id) {
        let frame = ctx.cumulative_pass_nr();
        if self.drawn_in + 1 < frame {
            self.opened_in = frame;
        }
        self.drawn_in = frame;
        if frame - self.opened_in < 3 {
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, id));
            ctx.request_repaint();
        }
    }
}

/// The kind half of a key menu: the scales, then a twelve-tone row and a
/// pitch-class set. Returns the scale picked, if it changed; `root` is
/// the root chosen in the menu beside it.
pub fn kind_menu(
    ui: &mut Ui,
    salt: &str,
    scale: Scale,
    root: PitchClass,
    width: f32,
) -> Option<Scale> {
    let ctx = ui.ctx().clone();
    let mut picked = None;
    let response = egui::ComboBox::from_id_salt(salt)
        .width(width)
        .selected_text(RichText::new(scale.kind.name()).strong())
        .show_ui(ui, |ui| {
            for option in ScaleKind::ALL {
                if ui
                    .selectable_label(scale.kind == option, option.name())
                    .clicked()
                {
                    picked = Some(Scale::new(root, option));
                }
            }
            ui.separator();
            let is_row = matches!(scale.kind, ScaleKind::Row(_));
            if ui
                .selectable_label(is_row, "twelve-tone row")
                .on_hover_text(
                    "Music without a key: all twelve notes in an order, used in turn. \
                     The piano roll numbers the notes in the order of the row. Notes \
                     already written stay as they are.",
                )
                .clicked()
                && !is_row
            {
                picked = Some(Scale::row(remembered_row(&ctx).starting_on(root)));
            }
            let is_set = matches!(scale.kind, ScaleKind::Set(_));
            if ui
                .selectable_label(is_set, "pitch-class set")
                .on_hover_text(
                    "Any group of notes as a scale, such as the octatonic scale or a \
                     chord with no name. Notes already written stay as they are: from a \
                     key, the set is the key's own notes (C major is 7-35), so the harmony \
                     keys still match the song.",
                )
                .clicked()
                && !is_set
            {
                picked = Some(set_key(&ctx, scale, root));
            }
        })
        .response;
    crate::theme::hint(response, crate::piano_roll::KEY_HELP);
    // A new root keeps the kind, and with it the row or the set.
    if picked.is_none() && root != scale.root {
        picked = Some(Scale::new(root, scale.kind));
    }
    picked.filter(|new| *new != scale)
}

/// How to write the notes of a set: a [`Scale`] writes them as the key
/// does (in D minor, C# rather than Db; for a row or a set, with the usual
/// names C# Eb F# Ab Bb), a bare [`Spelling`] with all sharps or all
/// flats.
pub trait NoteNames: Copy {
    fn note_name(self, pitch_class: PitchClass) -> NoteName;
    /// The notes of a chord, from its root up.
    fn chord_names(self, chord: Chord) -> Vec<NoteName>;
}

impl NoteNames for Scale {
    fn note_name(self, pitch_class: PitchClass) -> NoteName {
        self.spell(pitch_class)
    }
    fn chord_names(self, chord: Chord) -> Vec<NoteName> {
        self.chord_notes(chord)
    }
}

impl NoteNames for Spelling {
    fn note_name(self, pitch_class: PitchClass) -> NoteName {
        NoteName::plain(pitch_class, self)
    }
    fn chord_names(self, chord: Chord) -> Vec<NoteName> {
        chord.spell(NoteName::plain(chord.root, self))
    }
}

/// The notes of `set` in its normal form, written by `names`.
fn normal_names(set: PitchSet, names: impl NoteNames) -> String {
    let names: Vec<String> = set
        .normal_form()
        .iter()
        .map(|&p| names.note_name(p).to_string())
        .collect();
    names.join(" ")
}

/// The text of a pitch class in the matrix: its name, or its number. A
/// row has no key, so its notes take their usual names (see
/// [`NoteName::chromatic`]).
fn cell_text(semitone: u8, numbers: bool) -> String {
    if numbers {
        pc_digit(semitone).to_string()
    } else {
        NoteName::chromatic(PitchClass::from_semitone(semitone)).to_string()
    }
}

/// One line about a set: "4-28 · normal form C Eb F# A · …", its notes
/// written by `names`: pass the key, so they are spelled as it spells
/// them.
pub fn describe_set(set: PitchSet, names: impl NoteNames) -> String {
    if set.is_empty() {
        return String::new();
    }
    let class = set.set_class();
    let normal = normal_names(set, names);
    let mut text = format!(
        "{} · normal form {} · prime form {} · intervals {}",
        class.name(),
        normal,
        class.prime_text(),
        class.vector_text()
    );
    if let Some(nickname) = class.nickname() {
        text += &format!(" · {nickname}");
    }
    if let Some(micromode) = Micromode::of(set) {
        text += &format!(" · micromode {}", micromode.name);
    }
    text
}

/// A set class's line in the list, with its micromode if it has three
/// notes: "3-3 [014] <101100> · Menor 3".
fn list_line(class: &SetClass) -> String {
    match Micromode::of(class.prime) {
        Some(micromode) => format!("{} · {}", class.describe(), micromode.name),
        None => class.describe(),
    }
}

/// Something done in the row window.
pub enum RowAction {
    /// Make this row the key.
    UseAsKey(Scale),
    /// Put these notes in place of the channel's notes in the pattern.
    Write(Vec<NoteEvent>),
    /// Put these notes in a new pattern, keeping the current pattern intact.
    NewPattern { name: String, notes: Vec<NoteEvent> },
}

/// How long each note lasts, for the menu.
const STEPS: [(usize, &str); 4] = [(1, "1 step"), (2, "2 steps"), (4, "1 beat"), (8, "2 beats")];
/// How many notes sound together.
const TOGETHER: [(usize, &str); 5] = [
    (1, "1: a melody"),
    (2, "2: dyads"),
    (3, "3: trichords"),
    (4, "4: tetrachords"),
    (6, "6: hexachords"),
];
/// The lowest note to write from.
const LOWS: [u8; 4] = [36, 48, 60, 72];

const SELECTED: Color32 = Color32::from_rgb(74, 98, 140);
const LABEL: Color32 = Color32::from_rgb(160, 168, 186);

/// The row window: a row, its matrix, and how to write it.
pub struct RowWindow {
    pub open: bool,
    row: ToneRow,
    text: String,
    error: Option<String>,
    writing: RowWriting,
    numbers: bool,
    seed: u64,
    /// The key the window last saw, to follow changes made elsewhere.
    seen: Option<Scale>,
    front: Front,
}

impl RowWindow {
    pub fn new() -> Self {
        let row = lineup_row();
        RowWindow {
            open: false,
            row,
            text: row.note_names(),
            error: None,
            writing: RowWriting {
                label: row.label(),
                ..RowWriting::default()
            },
            numbers: false,
            seed: 1,
            seen: None,
            front: Front::default(),
        }
    }

    /// Looks at another row, starting on its own `P` form.
    fn set_row(&mut self, row: ToneRow) {
        self.row = row;
        self.text = row.note_names();
        self.error = None;
        self.writing.label = row.label();
    }

    /// When the key becomes a row, the window shows it: as the form it
    /// is of the row shown, if it is one, or as a row of its own.
    fn follow(&mut self, scale: Scale) {
        if self.seen == Some(scale) {
            return;
        }
        self.seen = Some(scale);
        if let Some(row) = scale.tone_row() {
            match self.row.label_of(row) {
                Some(label) => self.writing.label = label,
                None => self.set_row(row),
            }
        }
    }

    /// Draws the window. `drawn` are the notes to read a row from (the
    /// selected notes, or all of the channel's, in time order) and
    /// `length` the pattern's length.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        area: egui::Rect,
        scale: Scale,
        drawn: &[Note],
        length: usize,
        can_create_pattern: bool,
    ) -> Vec<RowAction> {
        self.follow(scale);
        let mut actions = Vec::new();
        let mut open = self.open;
        self.front.keep(ctx, row_window_id());
        egui::Window::new("Twelve-tone row")
            .id(row_window_id())
            .open(&mut open)
            .default_pos([260.0, 120.0])
            .default_height(720.0)
            .constrain_to(area)
            // On a small screen the window scrolls, so its buttons at the
            // bottom stay in reach, and it can be made taller.
            .resizable([false, true])
            .vscroll(true)
            .show(ctx, |ui| {
                self.contents(ui, drawn, length, can_create_pattern, &mut actions);
            });
        self.open = open;
        if let Some(RowAction::UseAsKey(scale)) = actions.first() {
            self.seen = Some(*scale);
        }
        actions
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        drawn: &[Note],
        length: usize,
        can_create_pattern: bool,
        actions: &mut Vec<RowAction>,
    ) {
        ui.weak(
            "All twelve notes in an order. Type them (E F G C#… or 4 5 7 1…), draw them in \
             the piano roll, or pick one. Click a label around the matrix to choose a form.",
        );
        ui.horizontal(|ui| {
            let edit = ui.add(
                egui::TextEdit::singleline(&mut self.text)
                    .desired_width(260.0)
                    .hint_text("E F G C# F# Eb Ab D B C A Bb"),
            );
            let enter = edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button("Use").clicked() || enter {
                match ToneRow::parse(&self.text) {
                    Ok(row) => self.set_row(row),
                    Err(error) => self.error = Some(error),
                }
            }
            if ui
                .button("From the notes")
                .on_hover_text(
                    "The first twelve different notes of the selection (or of the channel, \
                     if nothing is selected), in the order they start",
                )
                .clicked()
            {
                match ToneRow::from_notes(drawn.iter().copied()) {
                    Ok(row) => self.set_row(row),
                    Err(error) => self.error = Some(error),
                }
            }
        });
        ui.horizontal(|ui| {
            if ui.button("🎲 Random").clicked() {
                self.seed = self.seed.wrapping_add(1);
                self.set_row(ToneRow::random(self.seed.wrapping_mul(7919)));
            }
            if ui
                .button("🎲 All-interval")
                .on_hover_text("A random row whose eleven steps are all different sizes")
                .clicked()
            {
                self.seed = self.seed.wrapping_add(1);
                self.set_row(ToneRow::random_all_interval(self.seed));
            }
            ui.menu_button("Famous and example rows", |ui| {
                crate::widgets::scroll_menu(ui, |ui| {
                    for preset in ROW_PRESETS {
                        let hover = format!("{}\n\n{}", preset.source, preset.about);
                        if ui.button(preset.name).on_hover_text(hover).clicked() {
                            self.set_row(preset.row);
                            ui.close();
                        }
                    }
                });
            });
            ui.checkbox(&mut self.numbers, "Numbers")
                .on_hover_text("Show pitch classes as numbers: C is 0, C# is 1… T is 10, E is 11");
        });
        if let Some(error) = &self.error {
            ui.colored_label(Color32::from_rgb(240, 120, 110), error);
        }
        ui.add_space(4.0);
        self.matrix(ui);
        ui.add_space(4.0);
        self.properties(ui);
        ui.separator();
        self.write_options(ui, length, can_create_pattern, actions);
    }

    /// The 12×12 matrix with its labels: P on the left, R on the right,
    /// I along the top and RI along the bottom.
    fn matrix(&mut self, ui: &mut Ui) {
        let matrix = self.row.matrix();
        let selected = self.writing.label;
        // Which line or column the selected form runs along.
        let on_line = |i: usize, j: usize| {
            let line = RowLabel::new(RowForm::Prime, matrix[i][0]);
            let column = RowLabel::new(RowForm::Inversion, matrix[0][j]);
            match selected.form {
                RowForm::Prime | RowForm::Retrograde => line.number == selected.number,
                RowForm::Inversion | RowForm::RetrogradeInversion => {
                    column.number == selected.number
                }
            }
        };
        let mut chosen = None;
        let numbers = self.numbers;
        let mut label = |ui: &mut Ui, form: RowForm, number: u8| {
            let label = RowLabel::new(form, number);
            let color = if label == selected {
                crate::theme::ON_ACCENT
            } else {
                LABEL
            };
            let text = RichText::new(label.to_string())
                .monospace()
                .small()
                .color(color);
            let hover = format!(
                "{} {}: {}",
                form.name(),
                number,
                self.row.form(label).note_names()
            );
            if ui
                .selectable_label(label == selected, text)
                .on_hover_text(hover)
                .clicked()
            {
                chosen = Some(label);
            }
        };
        egui::Grid::new("row-matrix")
            .spacing([4.0, 2.0])
            .min_col_width(24.0)
            .show(ui, |ui| {
                ui.label("");
                for &top in &matrix[0] {
                    label(ui, RowForm::Inversion, top);
                }
                ui.label("");
                ui.end_row();
                for (i, line) in matrix.iter().enumerate() {
                    label(ui, RowForm::Prime, line[0]);
                    for (j, &cell) in line.iter().enumerate() {
                        let mut text = RichText::new(cell_text(cell, numbers)).monospace();
                        if on_line(i, j) {
                            text = text.background_color(SELECTED).strong();
                        }
                        ui.label(text);
                    }
                    label(ui, RowForm::Retrograde, line[0]);
                    ui.end_row();
                }
                ui.label("");
                for &top in &matrix[0] {
                    label(ui, RowForm::RetrogradeInversion, top);
                }
                ui.label("");
                ui.end_row();
            });
        if let Some(label) = chosen {
            self.writing.label = label;
        }
    }

    /// What is special about the row.
    fn properties(&self, ui: &mut Ui) {
        let form = self.row.form(self.writing.label);
        ui.label(RichText::new(format!("{}: {}", self.writing.label, form.note_names())).strong());
        let intervals: Vec<String> = self.row.intervals().iter().map(u8::to_string).collect();
        let mut facts = vec![format!("Steps up, in semitones: {}", intervals.join(" "))];
        if self.row.is_all_interval() {
            facts.push("An all-interval row: every step is a different size.".to_string());
        }
        let combinatorial = self.row.combinatorial_forms();
        if !combinatorial.is_empty() {
            let forms: Vec<String> = combinatorial.iter().map(|l| l.to_string()).collect();
            facts.push(format!(
                "Combinatorial: its first six notes and those of {} make all twelve.",
                forms.join(", ")
            ));
        }
        for size in [3, 4, 6] {
            // Groups of three are micromodes too.
            let classes: Vec<String> = self
                .row
                .segments(size)
                .iter()
                .map(|set| match Micromode::of(*set) {
                    Some(micromode) => format!("{} ({})", set.set_class().name(), micromode.name),
                    None => set.set_class().name(),
                })
                .collect();
            let name = match size {
                3 => "Trichords",
                4 => "Tetrachords",
                _ => "Hexachords",
            };
            facts.push(format!("{name}: {}", classes.join(", ")));
        }
        for fact in facts {
            ui.weak(fact);
        }
    }

    fn write_options(
        &mut self,
        ui: &mut Ui,
        length: usize,
        can_create_pattern: bool,
        actions: &mut Vec<RowAction>,
    ) {
        let writing = &mut self.writing;
        ui.horizontal_wrapped(|ui| {
            ui.label("Then");
            egui::ComboBox::from_id_salt("row-chain")
                .selected_text(writing.chain.name())
                .show_ui(ui, |ui| {
                    for chain in Chain::ALL {
                        ui.selectable_value(&mut writing.chain, chain, chain.name());
                    }
                })
                .response
                .on_hover_text("What comes after the form, until the pattern is full");
            ui.label("Together");
            let together = TOGETHER
                .iter()
                .find(|(n, _)| *n == writing.together)
                .map_or("", |(_, name)| *name);
            egui::ComboBox::from_id_salt("row-together")
                .selected_text(together)
                .show_ui(ui, |ui| {
                    for (n, name) in TOGETHER {
                        ui.selectable_value(&mut writing.together, n, name);
                    }
                })
                .response
                .on_hover_text("How many notes of the row sound at once, as a chord");
            ui.label("Each");
            let step = STEPS
                .iter()
                .find(|(n, _)| *n == writing.step)
                .map_or("", |(_, name)| *name);
            egui::ComboBox::from_id_salt("row-step")
                .selected_text(step)
                .show_ui(ui, |ui| {
                    for (n, name) in STEPS {
                        ui.selectable_value(&mut writing.step, n, name);
                    }
                });
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Octaves");
            egui::ComboBox::from_id_salt("row-register")
                .selected_text(writing.register.name())
                .show_ui(ui, |ui| {
                    for register in Register::ALL {
                        ui.selectable_value(&mut writing.register, register, register.name())
                            .on_hover_text(register.about());
                    }
                });
            ui.label("from");
            egui::ComboBox::from_id_salt("row-low")
                .selected_text(writing.low.to_string())
                .show_ui(ui, |ui| {
                    for low in LOWS {
                        ui.selectable_value(
                            &mut writing.low,
                            Note::new(low),
                            Note::new(low).to_string(),
                        );
                    }
                });
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Serialism:");
            ui.checkbox(&mut writing.durations, "Durations")
                .on_hover_text(
                    "Each note lasts as many steps as the row's number at its place, plus one",
                );
            ui.checkbox(&mut writing.dynamics, "Dynamics")
                .on_hover_text(
                    "Twelve levels from very soft to very loud, in the order of the inversion",
                );
            ui.checkbox(&mut writing.octaves, "Octaves").on_hover_text(
                "One of three octaves for each note, from the numbers of the retrograde",
            );
        });
        let notes = write_row(self.row, writing, length);
        let heard: PitchSet = notes.iter().map(|note| note.pitch.pitch_class()).collect();
        if heard.len() < 12 {
            ui.weak(format!(
                "The pattern only has room for {} of the 12 notes. Make it longer, or the \
                 steps shorter, to hear the whole row.",
                heard.len()
            ));
        }
        ui.weak(format!(
            "Selected instrument · pattern length: {length} steps"
        ));
        ui.horizontal_wrapped(|ui| {
            if ui
                .button(format!("Write {} into current pattern", writing.label))
                .on_hover_text(
                    "Replaces this channel's notes in the pattern. Ctrl+Z puts them back.",
                )
                .clicked()
            {
                actions.push(RowAction::Write(notes.clone()));
            }
            if ui
                .add_enabled(can_create_pattern, egui::Button::new("Save to new pattern"))
                .on_hover_text("Create and select a new pattern with these notes, the selected instrument, and the current pattern’s length and time signature. Ctrl+Z undoes it.")
                .on_disabled_hover_text("New patterns are available in the song piano roll, up to 99 patterns per project.")
                .clicked()
            {
                actions.push(RowAction::NewPattern {
                    name: format!("Row {}", writing.label),
                    notes,
                });
            }
            if ui
                .button("Use as the key")
                .on_hover_text(
                    "The piano roll numbers the notes 1 to 12 in this form's order, and the \
                     harmony keys play its groups of notes",
                )
                .clicked()
            {
                actions.push(RowAction::UseAsKey(Scale::row(
                    self.row.form(writing.label),
                )));
            }
        });
    }
}

/// Something done in the set browser.
pub enum SetAction {
    /// Click the grid to place this set above the note clicked.
    Stamp(PitchSet),
    /// Make this set the key.
    UseAsKey(Scale),
    /// Show the micromode of these three notes in the micromode window.
    ShowMicromode(PitchSet),
    /// Open the micromode window.
    OpenMicromodes,
}

/// The set browser: every set class, to look at, draw with or use as a
/// scale.
pub struct SetWindow {
    pub open: bool,
    size: usize,
    search: String,
    class: &'static SetClass,
    transposition: u8,
    inverted: bool,
    /// Whether the list scrolls to the class shown, after it was picked
    /// from elsewhere.
    reveal: bool,
    front: Front,
}

impl SetWindow {
    pub fn new() -> Self {
        SetWindow {
            open: false,
            size: 3,
            search: String::new(),
            class: PitchSet::from_semitones(&[0, 4, 7]).set_class(),
            transposition: 0,
            inverted: false,
            reveal: false,
            front: Front::default(),
        }
    }

    /// The set chosen: the class's prime form, inverted if asked, moved up.
    fn chosen(&self) -> PitchSet {
        let prime = self.class.prime;
        let set = if self.inverted { prime.invert() } else { prime };
        set.transpose(self.transposition)
    }

    /// Shows `set` in the browser: its class, and how it is moved from
    /// the prime form.
    pub(crate) fn pick(&mut self, set: PitchSet) {
        self.class = set.set_class();
        self.size = set.len();
        self.search.clear();
        self.reveal = true;
        let (n, inverted) = self.class.prime.moved_to(set).unwrap_or((0, false));
        self.transposition = n;
        self.inverted = inverted;
    }

    /// Draws the window. `selected` is the set of the selected notes (or
    /// of the notes held on a keyboard), to analyse; `names` writes the
    /// notes: pass the key, so they are spelled as it spells them.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        area: egui::Rect,
        names: impl NoteNames,
        selected: PitchSet,
    ) -> Vec<SetAction> {
        let mut actions = Vec::new();
        let mut open = self.open;
        self.front.keep(ctx, set_window_id());
        // A set shown from elsewhere comes to the front, to be seen.
        if self.reveal {
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, set_window_id()));
        }
        egui::Window::new("Pitch-class sets")
            .id(set_window_id())
            .open(&mut open)
            .default_pos([320.0, 140.0])
            .default_size([560.0, 520.0])
            .constrain_to(area)
            .show(ctx, |ui| self.contents(ui, names, selected, &mut actions));
        self.open = open;
        actions
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        names: impl NoteNames,
        selected: PitchSet,
        actions: &mut Vec<SetAction>,
    ) {
        if selected.len() >= 2 {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Selected notes:").strong());
                ui.label(describe_set(selected, names));
                if ui.button("Show it").clicked() {
                    self.pick(selected);
                }
            });
        } else {
            ui.weak("Select two or more notes in the piano roll to see their set.");
        }
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            ui.label("Notes");
            for size in 2..=10 {
                ui.selectable_value(&mut self.size, size, size.to_string());
            }
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("🔍 triad, octatonic, menor 3…")
                    .desired_width(160.0),
            );
            if ui
                .button("Micromodes…")
                .on_hover_text(
                    "The twelve sets of three notes are Kröpfl's micromodes: see them, \
                     with their names, families and sound",
                )
                .clicked()
            {
                actions.push(SetAction::OpenMicromodes);
            }
        });
        let search = self.search.trim().to_lowercase();
        let classes: Vec<&'static SetClass> = SetClass::all()
            .iter()
            .filter(|class| {
                if search.is_empty() {
                    class.cardinality as usize == self.size
                } else {
                    class.name().to_lowercase().contains(&search)
                        || class
                            .nickname()
                            .is_some_and(|n| n.to_lowercase().contains(&search))
                        || Micromode::of(class.prime).is_some_and(|m| {
                            m.name.to_lowercase().contains(&search)
                                || m.english.to_lowercase().contains(&search)
                                || "micromodes micromodos".contains(&search)
                        })
                }
            })
            .collect();
        egui::ScrollArea::vertical()
            .max_height(220.0)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for class in classes {
                    let text = RichText::new(list_line(class)).monospace();
                    let response = ui.selectable_label(self.class == class, text);
                    if self.reveal && self.class == class {
                        response.scroll_to_me(Some(egui::Align::Center));
                    }
                    if response.clicked() {
                        self.class = class;
                        self.transposition = 0;
                        self.inverted = false;
                    }
                }
            });
        self.reveal = false;
        ui.separator();
        self.details(ui, names, actions);
    }

    fn details(&mut self, ui: &mut Ui, names: impl NoteNames, actions: &mut Vec<SetAction>) {
        let class = self.class;
        ui.label(RichText::new(class.describe()).strong());
        let mut facts = Vec::new();
        if let Some(partner) = class.z_partner() {
            facts.push(format!(
                "Z-related to {}: the same intervals, yet a different set.",
                partner.name()
            ));
        }
        let complement = class.prime.complement().set_class();
        facts.push(format!(
            "The notes it leaves out form {}{}.",
            complement.name(),
            complement
                .nickname()
                .map(|n| format!(", {n}"))
                .unwrap_or_default()
        ));
        if class.is_symmetric() {
            facts.push("Upside down it is the same set, moved.".to_string());
        }
        for fact in facts {
            ui.weak(fact);
        }
        self.micromodes(ui, actions);
        ui.horizontal_wrapped(|ui| {
            ui.label("Move it up");
            egui::ComboBox::from_id_salt("set-transposition")
                .width(48.0)
                .selected_text(format!("T{}", self.transposition))
                .show_ui(ui, |ui| {
                    for n in 0..12 {
                        ui.selectable_value(&mut self.transposition, n, format!("T{n}"));
                    }
                })
                .response
                .on_hover_text("Transpose: move every note up this many semitones");
            ui.checkbox(&mut self.inverted, "Inverted (I)")
                .on_hover_text("Turn every interval upside down first");
            let set = self.chosen();
            let notes = normal_names(set, names);
            let label = if self.inverted {
                format!("T{}I", self.transposition)
            } else {
                format!("T{}", self.transposition)
            };
            ui.label(RichText::new(format!("{label}: {notes} ({})", digits(set))).strong());
        });
        ui.horizontal(|ui| {
            let set = self.chosen();
            if ui
                .button("Draw with it")
                .on_hover_text(
                    "A click on the grid places the set, packed as in its normal form, \
                     starting on the note you click",
                )
                .clicked()
                && let Some(&first) = set.normal_form().first()
            {
                actions.push(SetAction::Stamp(set.relative_to(first)));
            }
            if ui
                .button("Use as the key")
                .on_hover_text("The piano roll highlights these notes and snaps to them")
                .clicked()
                && let Some(&root) = set.normal_form().first()
            {
                actions.push(SetAction::UseAsKey(Scale::set(root, set)));
            }
        });
    }

    /// The micromodes the chosen set is, or holds: each a button to it in
    /// the micromode window.
    fn micromodes(&self, ui: &mut Ui, actions: &mut Vec<SetAction>) {
        let set = self.chosen();
        if let Some(micromode) = Micromode::of(set) {
            ui.horizontal_wrapped(|ui| {
                ui.weak("This is Kröpfl's micromode");
                if ui
                    .small_button(format!("{} →", micromode.full_name()))
                    .on_hover_text(micromode.about)
                    .clicked()
                {
                    actions.push(SetAction::ShowMicromode(set));
                }
            });
            return;
        }
        let found = micromodes_in(set);
        if found.is_empty() {
            return;
        }
        let groups = set.len() * (set.len() - 1) * (set.len() - 2) / 6;
        ui.horizontal_wrapped(|ui| {
            ui.weak(format!(
                "Micromodes inside, in its {groups} groups of three:"
            ));
            for (micromode, count) in found {
                let text = crate::micromodes::found_text(micromode, count);
                // One of the groups that is this micromode, to show.
                let example = lineup_core::harmony::trichords(set)
                    .into_iter()
                    .find(|trio| Micromode::of(*trio) == Some(micromode));
                if ui
                    .small_button(text)
                    .on_hover_text(format!("{}: {}", micromode.full_name(), micromode.about))
                    .clicked()
                    && let Some(trio) = example
                {
                    actions.push(SetAction::ShowMicromode(trio));
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_key_becomes_its_first_group_of_three() {
        let ctx = egui::Context::default();
        let webern = RowPreset::named("Webern's Concerto").unwrap().row;
        let row = Scale::row(webern.starting_on(PitchClass::C));
        let key = set_key(&ctx, row, PitchClass::C);
        assert_eq!(key.root, PitchClass::C);
        let group: PitchSet = [PitchClass::C, PitchClass::B, PitchClass::DSharp]
            .into_iter()
            .collect();
        assert_eq!(key.pitch_set(), group);
        assert_eq!(key.pitch_set().set_class().name(), "3-3");
        // From a key, the key's own notes, so nothing has to move: D
        // major is the set 7-35 on D.
        let d_major = Scale::new(PitchClass::D, ScaleKind::Major);
        let key = set_key(&ctx, d_major, PitchClass::D);
        assert_eq!(key.root, PitchClass::D);
        assert_eq!(key.pitch_set(), d_major.pitch_set());
        assert_eq!(key.pitch_set().set_class().name(), "7-35");
        // A minor too, and harmonic minor with its G#.
        let a_minor = Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        assert_eq!(
            set_key(&ctx, a_minor, PitchClass::A).pitch_set(),
            a_minor.pitch_set()
        );
        let harmonic = Scale::new(PitchClass::A, ScaleKind::HarmonicMinor);
        let key = set_key(&ctx, harmonic, PitchClass::A);
        assert_eq!(key.pitch_set(), harmonic.pitch_set());
        assert_eq!(key.spell(PitchClass::GSharp).to_string(), "G#");
        // From the chromatic scale, with no set used yet: the diminished
        // seventh chord.
        let chromatic = Scale::new(PitchClass::D, ScaleKind::Chromatic);
        let key = set_key(&ctx, chromatic, PitchClass::D);
        assert_eq!(key.pitch_set().set_class().name(), "4-28");
        assert!(key.contains(PitchClass::D));
    }

    #[test]
    fn a_key_as_a_set_keeps_its_chords() {
        use lineup_core::harmony::keys::{ChordSize, group_keys};
        let ctx = egui::Context::default();
        let c_major = Scale::new(PitchClass::C, ScaleKind::Major);
        let key = set_key(&ctx, c_major, PitchClass::C);
        let keys = group_keys(key, ChordSize::Triads).unwrap();
        let described: Vec<String> = keys
            .iter()
            .map(|key| {
                let names: Vec<String> = key.names.iter().map(|n| n.to_string()).collect();
                let class = key.pitch_set().set_class().name();
                format!("{} {} {class}", key.label, names.join(" "))
            })
            .collect();
        assert_eq!(
            described,
            [
                "C C E G 3-11",
                "Dm D F A 3-11",
                "Em E G B 3-11",
                "F F A C 3-11",
                "G G B D 3-11",
                "Am A C E 3-11",
                "Bdim B D F 3-10",
            ]
        );
    }

    #[test]
    fn a_set_is_spelled_as_the_key_spells_it() {
        let set = PitchSet::from_semitones(&[1, 5, 8]);
        let normal = |text: String| text.split(" · ").nth(1).unwrap_or_default().to_string();
        // In D minor, C# is the raised seventh and G# the sharp fourth.
        let d_minor = Scale::new(PitchClass::D, ScaleKind::NaturalMinor);
        assert_eq!(normal(describe_set(set, d_minor)), "normal form C# F G#");
        // Without a key, the usual names.
        let as_key = Scale::set(PitchClass::CSharp, set);
        assert_eq!(normal(describe_set(set, as_key)), "normal form C# F Ab");
        // A bare spelling still works as before.
        assert_eq!(
            normal(describe_set(set, Spelling::Flats)),
            "normal form Db F Ab"
        );
    }

    #[test]
    fn the_matrix_uses_the_usual_names() {
        let names: Vec<String> = (0..12).map(|n| cell_text(n, false)).collect();
        assert_eq!(names.join(" "), "C C# D Eb E F F# G Ab A Bb B");
    }
}
