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
use lineup_core::harmony::{Micromode, MicromodeFamily, micromodes_in};
use lineup_core::harmony::{
    ROW_PRESETS, RowForm, RowLabel, RowPreset, SetClass, ToneRow, digits, pc_digit,
};
use lineup_core::serial::{Chain, Register, RowWriting, read_plan, write_plan, write_row};
use lineup_core::{
    Chord, ChordStamp, Note, NoteEvent, NoteName, PitchClass, PitchSet, Scale, ScaleKind, Spelling,
};

use crate::i18n::{tr, tr_name, trc, trf};
use crate::widgets::{self, Opening};

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
/// harmony keys were playing, not a set from somewhere else. A set of
/// three notes is named as a set, 3-3, not as the micromode it also is
/// (see [`Scale::named_as_set`]): "pitch-class set" is what was picked.
///
/// - From a key, it is the key's own scale: C major is the set 7-35 on
///   C, and its keys are still C, Dm, Em, F, G, Am and Bdim, named as
///   sets too (3-11, 3-10).
/// - From a row, it is the row's first group of three, where it is
///   (Webern's Concerto row on C gives C B Eb, 3-3).
/// - From a micromode, it is the same three notes: Menor 3 on C becomes
///   3-3 on C, and every form keeps its degree and its notes.
/// - From the chromatic scale, which holds every note and so says
///   nothing about the harmony, it is the set used last, moved so it
///   starts on `root`.
pub(crate) fn set_key(ctx: &egui::Context, scale: Scale, root: PitchClass) -> Scale {
    let up = scale.root.semitones_up_to(root);
    if let Some(group) = scale
        .tone_row()
        .and_then(|row| row.segments(3).first().copied())
    {
        return Scale::set(root, group.transpose(up)).named_as_set();
    }
    if let ScaleKind::Set(_) = scale.kind {
        return Scale::set(root, scale.pitch_set().transpose(up)).named_as_set();
    }
    if scale.kind.is_tonal() && scale.kind != ScaleKind::Chromatic {
        let set = Scale::set(root, scale.pitch_set().transpose(up));
        // Gb major as a set still writes its notes Gb Ab Bb Cb...
        return if up == 0 {
            set.with_spelling(scale.spelling())
        } else {
            set
        };
    }
    let set = remembered_set(ctx);
    let start = set.normal_form().first().copied().unwrap_or(root);
    Scale::set(root, set.transpose(start.semitones_up_to(root))).named_as_set()
}

/// What a key menu has selected for `scale`: its kind, or, for a key of
/// three notes named as one, its micromode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum KindChoice {
    Kind(ScaleKind),
    Row,
    Set,
    Micromode(&'static Micromode),
}

impl KindChoice {
    /// The choice a key menu shows as selected while the key is `scale`.
    pub(crate) fn of(scale: Scale) -> Self {
        match (scale.kind, scale.micromode()) {
            (_, Some(micromode)) => KindChoice::Micromode(micromode),
            (ScaleKind::Row(_), _) => KindChoice::Row,
            (ScaleKind::Set(_), _) => KindChoice::Set,
            (kind, _) => KindChoice::Kind(kind),
        }
    }

    /// The key picking this choice in a key menu makes of `scale`, with
    /// the tonic `tonic`, or `None` if it is already the choice.
    ///
    /// The notes stay as they are wherever the menu says so: picking
    /// "pitch-class set" (see [`set_key`]), or picking the micromode a
    /// set of three notes already is, only names them the other way.
    pub(crate) fn key(self, ctx: &egui::Context, scale: Scale, tonic: NoteName) -> Option<Scale> {
        if KindChoice::of(scale) == self {
            return None;
        }
        let root = tonic.pitch_class();
        Some(match self {
            KindChoice::Kind(kind) => Scale::written(tonic, kind),
            KindChoice::Row => Scale::row(remembered_row(ctx).starting_on(root)),
            KindChoice::Set => set_key(ctx, scale, root),
            KindChoice::Micromode(micromode)
                if Micromode::of(scale.pitch_set()) == Some(micromode) && root == scale.root =>
            {
                scale.named_as_micromode()
            }
            KindChoice::Micromode(micromode) => micromode.key(root),
        })
    }
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

/// The root half of a key menu: the tonics a key of `scale`'s kind can
/// have, each written as the key writes it. A key that can be written
/// either way is listed both ways, F# and Gb (see
/// [`Scale::other_spelling`]), so all fifteen major keys of the circle
/// of fifths are there. A row, a set or the chromatic scale has no key
/// signature, and lists the twelve notes by both their names. Returns
/// the tonic chosen.
pub fn root_menu(ui: &mut Ui, salt: &str, scale: Scale, width: f32, text: RichText) -> NoteName {
    let mut tonic = scale.spell(scale.root);
    let keyed = scale.kind.is_tonal() && scale.kind != ScaleKind::Chromatic;
    egui::ComboBox::from_id_salt(salt)
        .width(width)
        .selected_text(text)
        .show_ui(ui, |ui| {
            for pitch_class in PitchClass::ALL {
                if !keyed {
                    let name = scale.spell(pitch_class);
                    let label = crate::piano_roll::both_names(pitch_class);
                    ui.selectable_value(&mut tonic, name, label);
                    continue;
                }
                let key = Scale::new(pitch_class, scale.kind);
                let mut names: Vec<NoteName> = [Some(key), key.other_spelling()]
                    .into_iter()
                    .flatten()
                    .map(|key| key.spell(key.root))
                    .collect();
                // Sharps first: C# and then Db, B and then Cb.
                names.sort_by_key(|name| std::cmp::Reverse(name.accidental));
                for name in names {
                    ui.selectable_value(&mut tonic, name, name.to_string());
                }
            }
        })
        .response
        .on_hover_text(crate::i18n::tr(crate::piano_roll::KEY_HELP));
    tonic
}

/// The kind half of a key menu: the scales, then a twelve-tone row and a
/// pitch-class set. Returns the scale picked, if it changed; `tonic` is
/// the tonic chosen in the menu beside it (see [`root_menu`]).
pub fn kind_menu(
    ui: &mut Ui,
    salt: &str,
    scale: Scale,
    tonic: NoteName,
    width: f32,
) -> Option<Scale> {
    let ctx = ui.ctx().clone();
    let mut picked = None;
    // A key of three notes is one of the micromodes, named as one unless
    // it was picked as a pitch-class set.
    let chosen = KindChoice::of(scale);
    let selected = match chosen {
        KindChoice::Micromode(micromode) => micromode.name.to_string(),
        _ => crate::i18n::tr(scale.kind.name()).to_string(),
    };
    // For how many frames the menu has been open: in its first frames,
    // it scrolls to the key's micromode, at the bottom of the list.
    let frames_id = egui::Id::unique((salt, "frames open"));
    let frames = ctx.data(|data| data.get_temp::<u8>(frames_id)).unwrap_or(0);
    let mut open = false;
    let response = egui::ComboBox::from_id_salt(salt)
        .width(width)
        // As tall as there is room for: the scales, the row, the set and
        // the micromodes.
        .height(620.0)
        .selected_text(RichText::new(selected).strong())
        .show_ui(ui, |ui| {
            open = true;
            let mut pick = |choice: KindChoice| picked = choice.key(&ctx, scale, tonic);
            for option in ScaleKind::ALL {
                let choice = KindChoice::Kind(option);
                if ui
                    .selectable_label(chosen == choice, crate::i18n::tr(option.name()))
                    .clicked()
                {
                    pick(choice);
                }
            }
            ui.separator();
            if ui
                .selectable_label(
                    chosen == KindChoice::Row,
                    crate::i18n::tr("twelve-tone row"),
                )
                .on_hover_text(tr(ROW_HELP))
                .clicked()
            {
                pick(KindChoice::Row);
            }
            if ui
                .selectable_label(
                    chosen == KindChoice::Set,
                    crate::i18n::tr("pitch-class set"),
                )
                .on_hover_text(tr(SET_HELP))
                .clicked()
            {
                pick(KindChoice::Set);
            }
            ui.separator();
            ui.weak(tr("Micromodes (Kröpfl)"));
            // One short row per family, to keep the menu short: Menor 1
            // to 5, Mayor 1 to 4, and the three triads.
            for family in MicromodeFamily::ALL {
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [46.0, 18.0],
                        egui::Label::new(RichText::new(family_word(family)).weak()),
                    );
                    for option in Micromode::all().iter().filter(|m| m.family == family) {
                        let choice = KindChoice::Micromode(option);
                        let is_key = chosen == choice;
                        let response = micromode_option(ui, option, is_key);
                        if is_key && frames < 3 {
                            response.scroll_to_me(Some(egui::Align::Center));
                        }
                        if response.clicked() {
                            pick(choice);
                        }
                    }
                });
            }
        })
        .response;
    let frames = if open { frames.saturating_add(1) } else { 0 };
    ctx.data_mut(|data| data.insert_temp(frames_id, frames));
    crate::theme::hint(response, crate::i18n::tr(crate::piano_roll::KEY_HELP));
    // A new root keeps the kind, and with it the row or the set, named
    // as it was.
    if picked.is_none() && tonic != scale.spell(scale.root) {
        picked = Some(Scale::written(tonic, scale.kind).named_like(scale));
    }
    picked.filter(|new| *new != scale)
}

/// What picking "twelve-tone row" in a key menu does.
pub(crate) const ROW_HELP: &str = "Music without a key: all twelve notes in an order, used in \
     turn. The piano roll numbers the notes in the order of the row. Notes already written \
     stay as they are.";

/// What picking "pitch-class set" in a key menu does.
pub(crate) const SET_HELP: &str = "Any group of notes as a scale, such as the octatonic scale or \
     a chord with no name. Notes already written stay as they are: from a key, the set is the \
     key's own notes (C major is 7-35), from a row its first three notes, and from a micromode \
     the same three notes (Menor 3 is 3-3), so the harmony keys still match the song.";

/// The word a family's micromodes start with: "Menor", "Mayor",
/// "Acorde", Kröpfl's names in every language.
pub(crate) fn family_word(family: MicromodeFamily) -> &'static str {
    match family {
        MicromodeFamily::Minor => "Menor",
        MicromodeFamily::Major => "Mayor",
        MicromodeFamily::Chord => "Acorde",
    }
}

/// A micromode by its number in its family, "3" for Menor 3, or the
/// triads by their chord signs: °, m/M, +.
pub(crate) fn micromode_short(micromode: &Micromode) -> &'static str {
    match micromode.name.rsplit_once(' ') {
        Some((_, number)) if number.chars().all(|c| c.is_ascii_digit()) => number,
        _ => match micromode.number {
            10 => "°",
            11 => "m/M",
            _ => "+",
        },
    }
}

/// A micromode's button in the key menu, by its number in its family
/// (the triads by their chord signs: °, m/M, +), with its whole name
/// and what picking it does when pointed at.
fn micromode_option(ui: &mut Ui, option: &Micromode, selected: bool) -> egui::Response {
    let short = micromode_short(option);
    ui.selectable_label(selected, short).on_hover_text(trf(
        "{}: {}. As the key, on the key note. From another set or \
                         micromode, every form keeps its degree and the notes go with \
                         it: T2 of Menor 3 becomes T2 of Menor 5. From a key, the notes \
                         stay as they are.",
        &[&option.full_name(), &option.intervals()],
    ))
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
    let mut text = trf(
        "{} · normal form {} · prime form {} · intervals {}",
        &[
            &class.name(),
            &normal,
            &class.prime_text(),
            &class.vector_text(),
        ],
    );
    if let Some(nickname) = class.nickname() {
        text += &format!(" · {}", tr(nickname));
    }
    if let Some(micromode) = Micromode::of(set) {
        text += &trf(" · micromode {}", &[&micromode.name]);
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
    /// Whether "Then" is a plan of forms typed in `plan_text`, rather
    /// than one of the chains: the forms read from it, or what is wrong
    /// with it.
    planned: bool,
    plan_text: String,
    plan: Result<Vec<RowLabel>, String>,
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
            planned: false,
            plan_text: String::new(),
            plan: Ok(Vec::new()),
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
        let window = egui::Window::new(crate::i18n::tr("Twelve-tone row"))
            .id(row_window_id())
            .open(&mut open)
            .default_pos([260.0, 120.0])
            .constrain_to(area)
            .resizable([false, true]);
        // As wide as the matrix and as tall as its contents, up to the
        // height of the playlist. On a small screen it scrolls, so its
        // buttons at the bottom stay in reach.
        let size = egui::vec2(340.0, area.height());
        widgets::open_at(ctx, window, row_window_id(), size, Opening::EveryTime).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.contents(ui, drawn, length, can_create_pattern, &mut actions);
            });
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
        ui.weak(tr(
            "All twelve notes in an order. Type them (E F G C#… or 4 5 7 1…), draw them in \
             the piano roll, or pick one. Click a label around the matrix to choose a form.",
        ));
        ui.horizontal(|ui| {
            let edit = ui.add(
                egui::TextEdit::singleline(&mut self.text)
                    .desired_width(260.0)
                    .hint_text(tr("E F G C# F# Eb Ab D B C A Bb")),
            );
            let enter = edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button(tr("Use")).clicked() || enter {
                match ToneRow::parse(&self.text) {
                    Ok(row) => self.set_row(row),
                    Err(error) => self.error = Some(error),
                }
            }
            if ui
                .button(tr("From the notes"))
                .on_hover_text(tr(
                    "The first twelve different notes of the selection (or of the channel, \
                     if nothing is selected), in the order they start",
                ))
                .clicked()
            {
                match ToneRow::from_notes(drawn.iter().copied()) {
                    Ok(row) => self.set_row(row),
                    Err(error) => self.error = Some(error),
                }
            }
        });
        ui.horizontal(|ui| {
            if ui.button(tr("🎲 Random")).clicked() {
                self.seed = self.seed.wrapping_add(1);
                self.set_row(ToneRow::random(self.seed.wrapping_mul(7919)));
            }
            if ui
                .button(tr("🎲 All-interval"))
                .on_hover_text(tr(
                    "A random row whose eleven steps are all different sizes",
                ))
                .clicked()
            {
                self.seed = self.seed.wrapping_add(1);
                self.set_row(ToneRow::random_all_interval(self.seed));
            }
            ui.menu_button(tr("Famous and example rows"), |ui| {
                crate::widgets::scroll_menu(ui, |ui| {
                    for preset in ROW_PRESETS {
                        let hover = format!("{}\n\n{}", tr(preset.source), tr(preset.about));
                        if ui
                            .button(tr_name(preset.name))
                            .on_hover_text(hover)
                            .clicked()
                        {
                            self.set_row(preset.row);
                            ui.close();
                        }
                    }
                });
            });
            ui.checkbox(&mut self.numbers, tr("Numbers"))
                .on_hover_text(tr(
                    "Show pitch classes as numbers: C is 0, C# is 1… T is 10, E is 11",
                ));
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
                tr(form.name()),
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
            // With a plan, the form clicked goes at its end.
            if self.planned {
                let text = self.plan_text.trim_end();
                self.plan_text = if text.is_empty() {
                    label.to_string()
                } else {
                    format!("{text} {label}")
                };
                self.plan = read_plan(&self.plan_text);
            }
        }
    }

    /// What is special about the row.
    fn properties(&self, ui: &mut Ui) {
        let form = self.row.form(self.writing.label);
        ui.label(RichText::new(format!("{}: {}", self.writing.label, form.note_names())).strong());
        let intervals: Vec<String> = self.row.intervals().iter().map(u8::to_string).collect();
        let mut facts = vec![trf("Steps up, in semitones: {}", &[&intervals.join(" ")])];
        if self.row.is_all_interval() {
            facts.push(tr("An all-interval row: every step is a different size.").to_string());
        }
        let combinatorial = self.row.combinatorial_forms();
        if !combinatorial.is_empty() {
            let forms: Vec<String> = combinatorial.iter().map(|l| l.to_string()).collect();
            facts.push(trf(
                "Combinatorial: its first six notes and those of {} make all twelve.",
                &[&forms.join(", ")],
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
            facts.push(format!("{}: {}", tr(name), classes.join(", ")));
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
        let planned = &mut self.planned;
        let (plan_text, plan) = (&mut self.plan_text, &mut self.plan);
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Then"));
            let then = if *planned {
                tr("A plan of forms")
            } else {
                tr(writing.chain.name())
            };
            egui::ComboBox::from_id_salt("row-chain")
                .selected_text(then)
                .show_ui(ui, |ui| {
                    for chain in Chain::ALL {
                        let chosen = !*planned && writing.chain == chain;
                        if ui.selectable_label(chosen, tr(chain.name())).clicked() {
                            writing.chain = chain;
                            *planned = false;
                        }
                    }
                    if ui
                        .selectable_label(*planned, tr("A plan of forms"))
                        .on_hover_text(tr(
                            "Forms you choose, one after another, such as P0 RI5 I7 R0",
                        ))
                        .clicked()
                    {
                        *planned = true;
                        if plan_text.trim().is_empty() {
                            *plan_text = writing.label.to_string();
                            *plan = read_plan(plan_text);
                        }
                    }
                })
                .response
                .on_hover_text(tr("What comes after the form, until the pattern is full"));
            ui.label(tr("Together"));
            let together = TOGETHER
                .iter()
                .find(|(n, _)| *n == writing.together)
                .map_or("", |(_, name)| tr(name));
            egui::ComboBox::from_id_salt("row-together")
                .selected_text(together)
                .show_ui(ui, |ui| {
                    for (n, name) in TOGETHER {
                        ui.selectable_value(&mut writing.together, n, tr(name));
                    }
                })
                .response
                .on_hover_text(tr("How many notes of the row sound at once, as a chord"));
            ui.label(tr("Each"));
            let step = STEPS
                .iter()
                .find(|(n, _)| *n == writing.step)
                .map_or("", |(_, name)| tr(name));
            egui::ComboBox::from_id_salt("row-step")
                .selected_text(step)
                .show_ui(ui, |ui| {
                    for (n, name) in STEPS {
                        ui.selectable_value(&mut writing.step, n, tr(name));
                    }
                });
        });
        if *planned {
            ui.horizontal(|ui| {
                ui.label(tr("Plan"));
                let edit = ui
                    .add(
                        egui::TextEdit::singleline(plan_text)
                            .desired_width(240.0)
                            .hint_text("P0 RI5 I7 R0"),
                    )
                    .on_hover_text(tr(
                        "The forms one after another, then again from the first until the \
                         pattern is full. Type them, or click them around the matrix.",
                    ));
                if edit.changed() {
                    *plan = read_plan(plan_text);
                }
            });
            match &*plan {
                Err(error) => {
                    ui.colored_label(Color32::from_rgb(240, 120, 110), error);
                }
                Ok(forms) if forms.is_empty() => {
                    ui.weak(tr(
                        "Type the forms to play, or click them around the matrix.",
                    ));
                }
                Ok(_) => {}
            }
        }
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Octaves"));
            egui::ComboBox::from_id_salt("row-register")
                .selected_text(trc("register", writing.register.name()))
                .show_ui(ui, |ui| {
                    for register in Register::ALL {
                        ui.selectable_value(
                            &mut writing.register,
                            register,
                            trc("register", register.name()),
                        )
                        .on_hover_text(tr(register.about()));
                    }
                });
            ui.label(tr("from"));
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
            ui.label(tr("Serialism:"));
            ui.checkbox(&mut writing.durations, tr("Durations"))
                .on_hover_text(tr(
                    "Each note lasts as many steps as the row's number at its place, plus one",
                ));
            ui.checkbox(&mut writing.dynamics, tr("Dynamics"))
                .on_hover_text(tr(
                    "Twelve levels from very soft to very loud, in the order of the inversion",
                ));
            ui.checkbox(&mut writing.octaves, tr("Octaves"))
                .on_hover_text(tr(
                    "One of three octaves for each note, from the numbers of the retrograde",
                ));
        });
        let partner = self.row.partner(writing.label);
        ui.horizontal_wrapped(|ui| {
            let text = match partner {
                Some(partner) => trf("With its partner, {}", &[&partner]),
                None => tr("With its partner").to_string(),
            };
            ui.add_enabled(
                partner.is_some(),
                egui::Checkbox::new(&mut writing.partner, text),
            )
            .on_hover_text(tr(
                "Its combinatorial partner sounds an octave under it, note for note: the \
                     first six notes of the two make all twelve, and so do the last six",
            ))
            .on_disabled_hover_text(tr(
                "No form of this row has the six notes the first half leaves out as its \
                     own first half, so it has no partner",
            ));
        });
        let planned = self.planned;
        let forms: &[RowLabel] = match &self.plan {
            Ok(forms) if planned => forms,
            _ => &[],
        };
        let notes = if planned {
            write_plan(self.row, writing, forms, length)
        } else {
            write_row(self.row, writing, length)
        };
        // What the buttons write: the form, or the plan.
        let what = if planned {
            forms
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            writing.label.to_string()
        };
        let nothing = notes.is_empty();
        let heard: PitchSet = notes.iter().map(|note| note.pitch.pitch_class()).collect();
        if heard.len() < 12 {
            ui.weak(trf(
                "The pattern only has room for {} of the 12 notes. Make it longer, or the \
                 steps shorter, to hear the whole row.",
                &[&heard.len()],
            ));
        }
        ui.weak(trf(
            "Selected instrument · pattern length: {} steps",
            &[&length],
        ));
        ui.horizontal_wrapped(|ui| {
            if ui
                .add_enabled(
                    !nothing,
                    egui::Button::new(trf("Write {} into current pattern", &[&what])),
                )
                .on_hover_text(
                    tr("Replaces this channel's notes in the pattern. Ctrl+Z puts them back."),
                )
                .on_disabled_hover_text(tr("Type a plan of forms first."))
                .clicked()
            {
                actions.push(RowAction::Write(notes.clone()));
            }
            if ui
                .add_enabled(
                    can_create_pattern && !nothing,
                    egui::Button::new(tr("Save to new pattern")),
                )
                .on_hover_text(tr("Create and select a new pattern with these notes, the selected instrument, and the current pattern’s length and time signature. Ctrl+Z undoes it."))
                .on_disabled_hover_text(if nothing {
                    tr("Type a plan of forms first.").to_string()
                } else {
                    trf(
                        "New patterns are available in the song piano roll, up to {} patterns per project.",
                        &[&lineup_core::MAX_PATTERNS],
                    )
                })
                .clicked()
            {
                actions.push(RowAction::NewPattern {
                    name: trf("Row {}", &[&what]),
                    notes,
                });
            }
            if ui
                .button(tr("Use as the key"))
                .on_hover_text(
                    tr("The piano roll numbers the notes 1 to 12 in this form's order, and the \
                     harmony keys play its groups of notes"),
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
    /// Play the chosen set without writing notes or changing the key.
    Hear(Vec<Note>),
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
        drawing: &mut ChordStamp,
    ) -> Vec<SetAction> {
        let mut actions = Vec::new();
        let mut open = self.open;
        self.front.keep(ctx, set_window_id());
        // A set shown from elsewhere comes to the front, to be seen.
        if self.reveal {
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, set_window_id()));
        }
        let window = egui::Window::new(crate::i18n::tr("Pitch-class sets"))
            .id(set_window_id())
            .open(&mut open)
            .default_pos([320.0, 140.0])
            .constrain_to(area);
        // As tall as its contents, up to the height of the playlist; they
        // scroll past that.
        let size = egui::vec2(560.0, area.height());
        widgets::open_at(ctx, window, set_window_id(), size, Opening::EveryTime).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.contents(ui, names, selected, drawing, &mut actions)
            })
        });
        self.open = open;
        actions
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        names: impl NoteNames,
        selected: PitchSet,
        drawing: &mut ChordStamp,
        actions: &mut Vec<SetAction>,
    ) {
        if selected.len() >= 2 {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(tr("Selected notes:")).strong());
                ui.label(describe_set(selected, names));
                if ui.button(tr("Show it")).clicked() {
                    self.pick(selected);
                }
            });
        } else {
            ui.weak(tr(
                "Select two or more notes in the piano roll to see their set.",
            ));
        }
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Notes"));
            for size in 2..=10 {
                ui.selectable_value(&mut self.size, size, size.to_string());
            }
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text(tr("🔍 triad, octatonic, menor 3…"))
                    .desired_width(160.0),
            );
            if ui
                .button(tr("Micromodes…"))
                .on_hover_text(tr(
                    "The twelve sets of three notes are Kröpfl's micromodes: see them, \
                     with their names, families and sound",
                ))
                .clicked()
            {
                actions.push(SetAction::OpenMicromodes);
            }
        });
        let classes = listed(self.size, &self.search);
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
        self.details(ui, names, drawing, actions);
    }

    fn details(
        &mut self,
        ui: &mut Ui,
        names: impl NoteNames,
        drawing: &mut ChordStamp,
        actions: &mut Vec<SetAction>,
    ) {
        let class = self.class;
        ui.label(RichText::new(class.describe()).strong());
        let mut facts = Vec::new();
        if let Some(partner) = class.z_partner() {
            facts.push(trf(
                "Z-related to {}: the same intervals, yet a different set.",
                &[&partner.name()],
            ));
        }
        let complement = class.prime.complement().set_class();
        facts.push(trf(
            "The notes it leaves out form {}{}.",
            &[
                &complement.name(),
                &complement
                    .nickname()
                    .map(|n| format!(", {}", tr(n)))
                    .unwrap_or_default(),
            ],
        ));
        if class.is_symmetric() {
            facts.push(tr("Upside down it is the same set, moved.").to_string());
        }
        for fact in facts {
            ui.weak(fact);
        }
        self.micromodes(ui, actions);
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Move it up"));
            egui::ComboBox::from_id_salt("set-transposition")
                .width(48.0)
                .selected_text(format!("T{}", self.transposition))
                .show_ui(ui, |ui| {
                    for n in 0..12 {
                        ui.selectable_value(&mut self.transposition, n, format!("T{n}"));
                    }
                })
                .response
                .on_hover_text(tr("Transpose: move every note up this many semitones"));
            ui.checkbox(&mut self.inverted, tr("Inverted (I)"))
                .on_hover_text(tr("Turn every interval upside down first"));
            let set = self.chosen();
            let notes = normal_names(set, names);
            let label = if self.inverted {
                format!("T{}I", self.transposition)
            } else {
                format!("T{}", self.transposition)
            };
            ui.label(RichText::new(format!("{label}: {notes} ({})", digits(set))).strong());
        });
        ui.horizontal_wrapped(|ui| {
            let set = self.chosen();
            if ui
                .button(tr("▶ Hear it"))
                .on_hover_text(tr("Play this set together on the current instrument"))
                .clicked()
            {
                let names: Vec<_> = set
                    .normal_form()
                    .into_iter()
                    .map(|pitch| names.note_name(pitch))
                    .collect();
                actions.push(SetAction::Hear(crate::micromodes::voiced(set, &names)));
            }
            draw_set_control(ui, set, drawing);
            if ui
                .button(tr("Use as the key"))
                .on_hover_text(tr(
                    "The piano roll highlights these notes and snaps to them",
                ))
                .clicked()
                && let Some(&root) = set.normal_form().first()
            {
                // Picked in the set window, a set of three notes is
                // named as a set, not as its micromode.
                actions.push(SetAction::UseAsKey(Scale::set(root, set).named_as_set()));
            }
        });
    }

    /// The micromodes the chosen set is, or holds: each a button to it in
    /// the micromode window.
    fn micromodes(&self, ui: &mut Ui, actions: &mut Vec<SetAction>) {
        let set = self.chosen();
        if let Some(micromode) = Micromode::of(set) {
            ui.horizontal_wrapped(|ui| {
                ui.weak(tr("This is Kröpfl's micromode"));
                if ui
                    .small_button(format!("{} →", micromode.full_name()))
                    .on_hover_text(tr(micromode.about))
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
            ui.weak(trf(
                "Micromodes inside, in its {} groups of three:",
                &[&groups],
            ));
            for (micromode, count) in found {
                let text = crate::micromodes::found_text(micromode, count);
                // One of the groups that is this micromode, to show.
                let example = lineup_core::harmony::trichords(set)
                    .into_iter()
                    .find(|trio| Micromode::of(*trio) == Some(micromode));
                if ui
                    .small_button(text)
                    .on_hover_text(format!(
                        "{}: {}",
                        micromode.full_name(),
                        tr(micromode.about)
                    ))
                    .clicked()
                    && let Some(trio) = example
                {
                    actions.push(SetAction::ShowMicromode(trio));
                }
            }
        });
    }
}

/// Both browsers edit the piano roll's actual drawing mode, so changing the
/// toolbar or either browser is reflected in all three places.
pub(crate) fn draw_set_control(ui: &mut Ui, set: PitchSet, drawing: &mut ChordStamp) {
    ui.selectable_value(drawing, ChordStamp::Single, tr("Single note"))
        .on_hover_text(tr("Return to drawing one note per click"));
    let Some(&first) = set.normal_form().first() else {
        return;
    };
    let stamp = ChordStamp::Set(set.relative_to(first));
    let active = *drawing == stamp;
    if ui
        .selectable_label(active, tr("Draw with it"))
        .on_hover_text(tr(
            "Draw this set above each clicked note. Click again to return to single notes.",
        ))
        .clicked()
    {
        *drawing = if active { ChordStamp::Single } else { stamp };
    }
}

/// The sets the window lists: those of `size` notes, or, while something
/// is searched, those whose name, nickname or micromode has it. Nicknames
/// are found in English and in the app's language, with or without
/// accents: "octatonic", "octatónica" or "octatonica".
fn listed(size: usize, search: &str) -> Vec<&'static SetClass> {
    let search = search.trim().to_lowercase();
    let words = crate::app::fold(&search);
    SetClass::all()
        .iter()
        .filter(|class| {
            if search.is_empty() {
                class.cardinality as usize == size
            } else {
                class.name().to_lowercase().contains(&search)
                    || class.nickname().is_some_and(|n| {
                        !words.is_empty()
                            && (crate::app::fold(n).contains(&words)
                                || crate::app::fold(tr(n)).contains(&words))
                    })
                    || Micromode::of(class.prime).is_some_and(|m| {
                        m.name.to_lowercase().contains(&search)
                            || m.english.to_lowercase().contains(&search)
                            || "micromodes micromodos".contains(&search)
                    })
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hear_previews_the_chosen_set_with_its_transposition_and_inversion() {
        let ctx = egui::Context::default();
        let mut window = SetWindow::new();
        window.pick(PitchSet::from_semitones(&[9, 10, 0, 2, 5]));
        window.inverted = !window.inverted;
        let expected = window.chosen();
        let mut drawing = ChordStamp::Single;
        let actions = {
            let mut frame = |events| {
                let mut actions = Vec::new();
                let mut output = ctx.run_pass(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(900.0, 700.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        egui::Window::new("PCS preview test").show(ctx, |ui| {
                            window.details(ui, Spelling::Sharps, &mut drawing, &mut actions);
                        });
                    },
                );
                output.textures_delta.clear();
                (output, actions)
            };
            frame(vec![]);
            let (output, _) = frame(vec![]);
            let pos = output
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.text() == "▶ Hear it" => {
                        Some(text.pos + text.galley.size() * 0.5)
                    }
                    _ => None,
                })
                .expect("Hear control is visible");
            frame(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ])
            .1
        };
        let pitches = actions
            .into_iter()
            .find_map(|action| match action {
                SetAction::Hear(pitches) => Some(pitches),
                _ => None,
            })
            .expect("Clicking Hear produces a preview");
        assert_eq!(
            pitches
                .iter()
                .map(|note| note.pitch_class())
                .collect::<PitchSet>(),
            expected
        );
        assert!(pitches.windows(2).all(|notes| notes[0] < notes[1]));
        assert!(pitches[0] >= Note::C4);
        assert_eq!(drawing, ChordStamp::Single);
    }

    #[test]
    fn sets_are_found_by_their_nickname_in_either_language() {
        let names = |search| -> Vec<String> {
            listed(3, search).iter().map(|class| class.name()).collect()
        };
        assert_eq!(names("octatonic"), ["8-28"]);
        crate::i18n::Language::Spanish.with(|| {
            assert_eq!(names("octatónica"), ["8-28"]);
            assert_eq!(names("Octatonica"), ["8-28"]);
            assert_eq!(names("octatonic"), ["8-28"]);
        });
        assert_eq!(names("3-11"), ["3-11"]);
        assert_eq!(listed(3, "").len(), 12);
    }

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

    /// Every choice of a Key menu, from every kind of key: once picked,
    /// it is the choice the menu shows as selected, and picking the
    /// selected one changes nothing.
    #[test]
    fn every_key_menu_choice_is_shown_as_picked() {
        let ctx = egui::Context::default();
        let webern = RowPreset::named("Webern's Concerto").unwrap().row;
        let row = Scale::row(webern.starting_on(PitchClass::C));
        let menor_3 = Micromode::named("Menor 3").unwrap().key(PitchClass::C);
        let from_row = set_key(&ctx, row, PitchClass::C);
        let keys = [
            Scale::new(PitchClass::C, ScaleKind::Major),
            Scale::new(PitchClass::FSharp, ScaleKind::Major)
                .other_spelling()
                .unwrap(),
            Scale::new(PitchClass::A, ScaleKind::HarmonicMinor),
            Scale::new(PitchClass::D, ScaleKind::Chromatic),
            row,
            menor_3,
            menor_3.named_as_set(),
            // Menor 3 with C B Eb as its own form, as from Webern's row.
            from_row,
            from_row.named_as_micromode(),
            Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 3, 6, 9])),
        ];
        let choices: Vec<KindChoice> = ScaleKind::ALL
            .into_iter()
            .map(KindChoice::Kind)
            .chain([KindChoice::Row, KindChoice::Set])
            .chain(Micromode::all().iter().map(KindChoice::Micromode))
            .collect();
        for key in keys {
            let tonic = key.spell(key.root);
            for &choice in &choices {
                match choice.key(&ctx, key, tonic) {
                    None => assert_eq!(KindChoice::of(key), choice, "{key:?}: {choice:?}"),
                    Some(new) => {
                        assert_ne!(new, key, "{key:?}: {choice:?}");
                        assert_eq!(KindChoice::of(new), choice, "{key:?} to {new:?}");
                        assert_eq!(new.root, key.root, "{key:?}: {choice:?}");
                    }
                }
            }
        }
    }

    /// "pitch-class set" names the notes the key already had, so the
    /// song keeps every note; from Webern's row, whose groups of three
    /// are all Menor 3, it is 3-3, named as a set, not the micromode.
    #[test]
    fn picking_pitch_class_set_keeps_every_note() {
        use lineup_core::Project;
        use lineup_core::chord_sheet::HarmonyName;
        use lineup_core::library::{MicromodeStyle, RowStyle, demo};
        let notes = |project: &Project| -> Vec<(usize, usize, NoteEvent)> {
            (0..project.patterns().len())
                .flat_map(|index| {
                    let pattern = project.pattern(index).unwrap();
                    pattern
                        .all_notes()
                        .map(move |(channel, note)| (index, channel, *note))
                        .collect::<Vec<_>>()
                })
                .collect()
        };
        let ctx = egui::Context::default();
        // The set used last is another one: it must not be what the key
        // becomes.
        remember(
            &ctx,
            Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 3, 6, 9])),
        );
        let pick = |project: &mut Project, choice: KindChoice| {
            let key = project.scale;
            let new = choice
                .key(&ctx, key, key.spell(key.root))
                .expect("a new choice");
            project.change_key(new);
            assert_eq!(KindChoice::of(project.scale), choice);
        };
        let menor_3 = KindChoice::Micromode(Micromode::named("Menor 3").unwrap());

        let mut webern = demo::row_demo(RowStyle::named("Webern's trichords").unwrap());
        let written = notes(&webern);
        pick(&mut webern, KindChoice::Set);
        assert_eq!(notes(&webern), written);
        assert_eq!(
            crate::i18n::key_name(webern.scale),
            "set 3-3 on C",
            "named as a set"
        );
        // Menor 3 is the same notes named as the micromode, and back.
        let set = webern.scale.pitch_set();
        pick(&mut webern, menor_3);
        assert_eq!(webern.scale.pitch_set(), set);
        assert_eq!(crate::i18n::key_name(webern.scale), "Menor 3 on C");
        pick(&mut webern, KindChoice::Set);
        assert_eq!(webern.scale.pitch_set(), set);
        assert_eq!(notes(&webern), written);

        // From a micromode demo: the same three notes as a set, and the
        // forms on the chord sheet named as sets too.
        let style = MicromodeStyle::named("Menor 3 colour").unwrap();
        let mut song = demo::micromode_demo(style);
        let written = notes(&song);
        let key = song.scale;
        let sheet = |project: &Project| -> Vec<HarmonyName> {
            project
                .playlist()
                .chord_markers()
                .iter()
                .map(|marker| marker.harmony)
                .collect()
        };
        assert!(
            sheet(&song)
                .iter()
                .all(|harmony| matches!(harmony, HarmonyName::Micromode(_)))
        );
        pick(&mut song, KindChoice::Set);
        assert_eq!(notes(&song), written);
        assert_eq!(song.scale.pitch_set(), key.pitch_set());
        assert!(
            sheet(&song)
                .iter()
                .all(|harmony| matches!(harmony, HarmonyName::Set(_)))
        );
        pick(&mut song, menor_3);
        assert_eq!(song.scale, key);
        assert_eq!(notes(&song), written);
        assert!(
            sheet(&song)
                .iter()
                .all(|harmony| matches!(harmony, HarmonyName::Micromode(_)))
        );
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
