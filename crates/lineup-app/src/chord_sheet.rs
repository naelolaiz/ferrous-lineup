//! The chord sheet window: the song's harmony bar by bar, the way a lead
//! sheet writes it.
//!
//! Bars are drawn four to a line, and each section of the playlist
//! (Intro, Verse, Chorus…) starts a new line under its name, so the
//! sheet reads like a printed chart. So do the parts of the band's form
//! ([`form`]): the intro, the chorus, between repeat signs when the band
//! plays it more than once, and the ending. A bar shows its harmonies where
//! their beats fall, each with a caption: the Roman numeral of a chord
//! in the key, or the notes of a set or a micromode. A bar with nothing
//! written goes on with the harmony before it, shown faintly. While the
//! song plays, the bar playing is framed in white and its harmony is the
//! brightest thing on the sheet.
//!
//! Click a bar and type to write its harmony; Enter takes it in, Tab
//! goes on to the next bar. Holding the mouse down on a harmony plays
//! it. Shift+click (or Shift and an arrow) selects several bars, to loop
//! them from the practice row ([`practice`]). A bar's menu (right-click)
//! also chooses the variation of the band's style from that bar on, or
//! for the bars selected; a boxed "a" or "b" marks where it changes.
//! Ctrl+C copies the bars selected as text, "| Dm7 G7 | C |", and Ctrl+V
//! writes text from the selected bar on (Ctrl+A selects every bar). The
//! reading and writing of what is typed is in `lineup_core::chord_sheet`;
//! this module draws and reads the keyboard and the mouse.

use eframe::egui::{
    self, Align2, Color32, FontId, Key, Modifiers, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, pos2,
    vec2,
};
use lineup_core::band::solo::Soloist;
use lineup_core::band::{Ending, Variation};
use lineup_core::chord_sheet::{Push, Stop, read_bar, text, write_bar};
use lineup_core::general_midi;
use lineup_core::library::Style;
use lineup_core::{
    Band, BandStyle, Bar, ChordMarker, HarmonyName, KeyMap, MAX_SONG_STEPS, Meter, Note, Pass,
    Player, Project,
};

use crate::library_browser::{LibraryAction, LibraryBrowser, LibraryPurpose, Tried};
use crate::theme;
use crate::twelve_tone::Front;

mod form;
mod practice;

pub use form::FormAction;
pub use practice::{PracticeAction, PracticeView, SPEEDS};

/// The id of the window.
pub(crate) fn window_id() -> egui::Id {
    egui::Id::unique("chord-sheet")
}

/// Bars on a line, as on most printed charts.
const BARS_PER_LINE: usize = 4;
const CELL_HEIGHT: f32 = 58.0;
/// The space above a line that starts a section, for its name.
const HEADING_HEIGHT: f32 = 18.0;
const LINE_GAP: f32 = 6.0;
const NAME_SIZE: f32 = 20.0;
const SMALLEST_NAME: f32 = 11.0;
const CAPTION_SIZE: f32 = 13.0;
const SMALL_SIZE: f32 = 10.0;
/// Inside a bar: room for the bar number above the harmonies.
const PADDING: f32 = 6.0;
const TOP: f32 = 15.0;

const PAPER: Color32 = Color32::from_rgb(38, 44, 49);
const PAPER_HOVER: Color32 = Color32::from_rgb(46, 53, 59);
const BAR_LINE: Color32 = Color32::from_rgb(118, 130, 139);
/// The frame of the bar playing, as on the harmony keys.
const PLAYING: Color32 = Color32::WHITE;
/// Sets, micromodes and rows are written in the colour the harmony keys
/// give groups of notes that have no function in a key.
const GROUP_TEXT: Color32 = Color32::from_rgb(196, 176, 246);
/// A harmony that goes on from an earlier bar.
const GOING_ON: Color32 = Color32::from_rgb(92, 104, 112);
/// The harmony of the band's ending, which is not written on the sheet.
const ENDING: Color32 = Color32::from_rgb(150, 162, 170);

/// Something done on the chord sheet.
pub enum SheetAction {
    UserStyle {
        index: usize,
        progression_from: Option<usize>,
    },
    /// Write the harmonies of the steps from `start` to `end`: one bar.
    SetChords {
        start: usize,
        end: usize,
        markers: Vec<ChordMarker>,
    },
    /// Write the chords the song plays on the empty sheet.
    WriteHeardChords,
    /// Have the band play in this style, or stop it.
    SetBand {
        style: Option<&'static BandStyle>,
        harmony: Option<&'static Style>,
    },
    /// Swing the eighth notes this much, from 0 to `MAX_SWING`.
    SetSwing(u8),
    /// Have the band play this part again, where it was edited.
    PlayAgain(Player),
    /// Have a soloist improvise with the band, or not.
    SetSoloist(bool),
    /// Have the soloist play another solo.
    AnotherSolo,
    /// Write a progression's chords from this bar on.
    WriteProgression {
        style: &'static Style,
        from: usize,
    },
    /// End the song with this bar.
    EndHere(usize),
    /// Move the playhead to this step of the playlist's timeline.
    Seek(usize),
    /// Play these notes now, as a harmony key does.
    NoteOn(Vec<Note>),
    NoteOff(Vec<Note>),
    /// Something from the practice row.
    Practice(PracticeAction),
    /// Something from the form row.
    Form(FormAction),
}

/// A bar being typed in.
struct Editing {
    bar: usize,
    text: String,
    /// Whether the text field still has to take the keyboard, with the
    /// text selected or the cursor at its end.
    fresh: Option<Fresh>,
}

#[derive(Clone, Copy)]
enum Fresh {
    /// Typing replaces what was there.
    SelectAll,
    /// Typing goes on from the letter that started it.
    CursorAtEnd,
}

/// A bar on the screen: the bar, where it is drawn, and its line and
/// column.
#[derive(Clone, Copy)]
struct Cell {
    bar: Bar,
    rect: Rect,
    line: usize,
    column: usize,
}

/// What the mouse did on a bar.
struct BarClicks {
    /// It was clicked, with either button.
    clicked: bool,
    /// A bar to start typing in, with its text.
    start_typing: Option<(usize, String)>,
}

/// One line of the sheet: up to four bars, and the name of the section
/// it starts, if it starts one.
struct Line {
    heading: Option<String>,
    bars: Vec<Bar>,
}

/// What the band's form puts on the sheet: the repeat signs around a
/// chorus that plays more than once, and the ending.
#[derive(Clone, Copy)]
struct Marks {
    /// The chorus's first step and the step after its last, if it
    /// repeats.
    repeat: Option<(usize, usize)>,
    ending: Option<Ending>,
}

impl Marks {
    fn of(project: &Project) -> Self {
        let repeats = project.form().is_some_and(|form| form.choruses > 1);
        Marks {
            repeat: project.chorus().filter(|_| repeats),
            ending: project.ending(),
        }
    }

    /// The ending, if `bar` is in it.
    fn ending_in(&self, bar: &Bar) -> Option<Ending> {
        self.ending
            .filter(|ending| (ending.start..ending.end).contains(&bar.start))
    }
}

/// The chord sheet window.
pub struct ChordSheet {
    pub open: bool,
    /// The bar the keyboard works on, by number.
    selected: Option<usize>,
    /// The other end of a selection of several bars, made with Shift.
    anchor: Option<usize>,
    editing: Option<Editing>,
    /// Why the last text typed could not be read, and in which bar.
    error: Option<(usize, String)>,
    /// What the last copy or paste did, and whether it went wrong.
    clipboard: Option<(String, bool)>,
    /// The notes of the harmony held down with the mouse.
    sounding: Vec<Note>,
    /// Whether the sheet had the keyboard in the last frame. The number
    /// keys are then its own, not the harmony keys'.
    focused: bool,
    /// The bar the playhead was in, to scroll along as it moves on.
    followed: Option<usize>,
    /// Whether to scroll to the selected bar, moved there with the keys.
    reveal: bool,
    front: Front,
}

impl ChordSheet {
    pub fn new() -> Self {
        ChordSheet {
            open: false,
            selected: None,
            anchor: None,
            editing: None,
            error: None,
            clipboard: None,
            sounding: Vec::new(),
            focused: false,
            followed: None,
            reveal: false,
            front: Front::default(),
        }
    }

    /// Whether the sheet has the keyboard, so typed digits are chords
    /// and sets, not harmony keys.
    pub fn has_keyboard(&self) -> bool {
        self.open && self.focused
    }

    /// Lets go of a harmony held down with the mouse.
    fn release(&mut self, actions: &mut Vec<SheetAction>) {
        if !self.sounding.is_empty() {
            actions.push(SheetAction::NoteOff(std::mem::take(&mut self.sounding)));
        }
    }

    /// Draws the window over `area`. `playing` is the step of the
    /// playlist's timeline being played, and the pass of the song it is
    /// in, while the song plays;
    /// `typing_keys` says whether letters play notes (the typing
    /// keyboard), so they cannot start typing in a bar.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        area: Rect,
        project: &Project,
        playing: Option<(usize, Pass)>,
        typing_keys: bool,
        controls: (PracticeView, &mut LibraryBrowser),
    ) -> Vec<SheetAction> {
        let (practice, library) = controls;
        let mut actions = Vec::new();
        if !self.open {
            self.release(&mut actions);
            self.editing = None;
            self.focused = false;
            return actions;
        }
        let mut open = self.open;
        self.front.keep(ctx, window_id());
        egui::Window::new("Chord sheet")
            .id(window_id())
            .open(&mut open)
            .default_pos([260.0, 110.0])
            .default_size([760.0, 420.0])
            .min_width(360.0)
            .constrain_to(area)
            .resizable(true)
            .show(ctx, |ui| {
                self.contents(
                    ui,
                    project,
                    playing,
                    typing_keys,
                    (practice, library),
                    &mut actions,
                );
            });
        self.open = open;
        if !self.open {
            self.release(&mut actions);
            self.editing = None;
            self.focused = false;
        }
        actions
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        playing: Option<(usize, Pass)>,
        typing_keys: bool,
        controls: (PracticeView, &mut LibraryBrowser),
        actions: &mut Vec<SheetAction>,
    ) {
        let (practice, library) = controls;
        let markers = project.playlist().chord_markers();
        ui.horizontal_wrapped(|ui| {
            band_menu(ui, project, library, actions);
            progression_menu(ui, project, self.next_bar(project), library, actions);
            ui.label(egui::RichText::new(format!("Key: {}", project.scale.name())).strong());
            ui.weak(format!("· {}", project.time_signature));
            if project.band().is_some() {
                ui.weak(format!("· {} BPM", project.tempo.bpm().round()));
                ui.separator();
                if let Some(swing) = crate::playlist::swing_control(ui, project) {
                    actions.push(SheetAction::SetSwing(swing));
                }
            }
            // A band plays from the sheet, so its own notes are not heard.
            if project.song_length() > 0 && (markers.is_empty() || project.band().is_none()) {
                let button = ui.button("Write the chords the song plays");
                if theme::hint(
                    button,
                    "Fill the sheet with the chords heard in the song, beat by beat, as the \
                     harmony keys hear them, in place of what it says now; in a set or a \
                     row, with the sets or micromodes heard. Undo brings the sheet back",
                )
                .clicked()
                {
                    actions.push(SheetAction::WriteHeardChords);
                }
            }
        });
        let round = playing.and_then(|(_, pass)| pass.chorus);
        form::row(ui, project, round, actions);
        practice::row(ui, project, practice, self.selected_bars(), actions);
        ui.weak(
            "Click a bar and type its harmony: chords (Am7 D7, G/B), Roman numerals (ii7 V7), \
             sets (3-11 T2, {0,1,4}), micromodes (Menor 3), row forms (P0, RI7) or N.C.",
        );
        ui.weak(
            "Each / holds the harmony before it one more beat, as in C / / G7. The band stops \
             with a shot (C.), a hold (C..) or a rest (C...), and ^C comes in early. Ctrl+C and \
             Ctrl+V copy and paste bars as text.",
        );
        for player in Player::ALL {
            if project.is_frozen(player) {
                ui.horizontal_wrapped(|ui| {
                    ui.colored_label(
                        ui.visuals().warn_fg_color,
                        format!(
                            "✎ You edited the {}: the band keeps it as you left it.",
                            player.name().to_lowercase()
                        ),
                    );
                    let again = ui.small_button("Play it again");
                    if theme::hint(again, "Have the band play this part from the sheet again")
                        .clicked()
                    {
                        actions.push(SheetAction::PlayAgain(player));
                    }
                });
            }
        }
        match (&self.error, &self.clipboard) {
            (Some((bar, error)), _) => {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    format!("Bar {}: {error}", bar + 1),
                );
            }
            (None, Some((problem, true))) => {
                ui.colored_label(ui.visuals().error_fg_color, problem);
            }
            (None, Some((done, false))) => {
                ui.weak(done);
            }
            (None, None) => {
                ui.weak(" ");
            }
        }
        ui.separator();
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let playing = playing.map(|(step, _)| step);
                self.sheet(ui, project, playing, typing_keys, actions);
            });
    }

    /// The bars, line by line.
    fn sheet(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        playing: Option<usize>,
        typing_keys: bool,
        actions: &mut Vec<SheetAction>,
    ) {
        let meter = project.meter();
        let keys = project.key_map();
        let lines = self.lines(project, &meter);
        let width = ui.available_width().max(BARS_PER_LINE as f32 * 80.0);
        let cell_width = (width - 2.0) / BARS_PER_LINE as f32;
        let height: f32 = lines
            .iter()
            .map(|line| {
                let heading = if line.heading.is_some() {
                    HEADING_HEIGHT
                } else {
                    0.0
                };
                heading + CELL_HEIGHT + LINE_GAP
            })
            .sum();
        let (canvas, sheet) = ui.allocate_exact_size(vec2(width, height), Sense::click());
        let painter = ui.painter_at(canvas);

        // Where every bar is drawn.
        let mut cells: Vec<Cell> = Vec::new();
        let mut y = canvas.top();
        for (index, line) in lines.iter().enumerate() {
            if let Some(heading) = &line.heading {
                painter.text(
                    pos2(canvas.left() + 2.0, y + 2.0),
                    Align2::LEFT_TOP,
                    heading,
                    FontId::proportional(13.0),
                    theme::ACCENT,
                );
                y += HEADING_HEIGHT;
            }
            for (column, bar) in line.bars.iter().enumerate() {
                let left = canvas.left() + 1.0 + column as f32 * cell_width;
                cells.push(Cell {
                    bar: *bar,
                    rect: Rect::from_min_size(pos2(left, y), vec2(cell_width, CELL_HEIGHT)),
                    line: index,
                    column,
                });
            }
            y += CELL_HEIGHT + LINE_GAP;
        }

        let playing_bar = playing.map(|step| meter.bar_at(step).number);
        let marks = Marks::of(project);
        let mut start_typing = None;
        let mut clicked = sheet.clicked();
        for cell in &cells {
            let starts_section = cell.column == 0 && lines[cell.line].heading.is_some();
            let ends_line = cell.column + 1 == lines[cell.line].bars.len();
            let clicks = self.bar(
                ui,
                &painter,
                project,
                &meter,
                &keys,
                *cell,
                (starts_section, ends_line),
                marks,
                playing,
                actions,
            );
            clicked |= clicks.clicked;
            if clicks.start_typing.is_some() {
                start_typing = clicks.start_typing;
            }
            if self.reveal && self.selected == Some(cell.bar.number) {
                ui.scroll_to_rect(cell.rect, None);
                self.reveal = false;
            }
            if crate::widgets::follow_playback(ui.ctx())
                && playing_bar == Some(cell.bar.number)
                && self.followed != playing_bar
            {
                ui.scroll_to_rect(cell.rect, None);
            }
        }
        self.followed = playing_bar.filter(|_| crate::widgets::follow_playback(ui.ctx()));
        if !ui.input(|input| input.pointer.primary_down()) {
            self.release(actions);
        }
        if clicked {
            sheet.request_focus();
        }

        // What is typed is taken in before another bar is typed in, so
        // a click on another bar never loses it.
        let was_editing = self.editing.is_some();
        self.edit_field(ui, project, &cells, &keys, sheet.id, actions);
        if let Some((bar, text)) = start_typing {
            self.take_in(project, &cells, &keys, actions);
            self.start_editing(bar, text, Fresh::SelectAll);
        }
        // The key that ended the typing (Enter, Escape) is not read
        // again as a key for the sheet.
        let has_keyboard = crate::widgets::editor_keyboard(ui.ctx(), ui.layer_id());
        if has_keyboard && self.editing.is_none() && !was_editing {
            ui.memory_mut(|memory| memory.set_focus_lock_filter(sheet.id, KEEP_KEYS));
            self.keys(ui, project, &cells, typing_keys, actions);
        }
        self.focused = has_keyboard || self.editing.is_some();
    }

    /// Draws one bar and reads the mouse on it.
    #[allow(clippy::too_many_arguments)]
    fn bar(
        &mut self,
        ui: &mut Ui,
        painter: &egui::Painter,
        project: &Project,
        meter: &Meter,
        keys: &KeyMap,
        cell: Cell,
        (starts_section, ends_line): (bool, bool),
        marks: Marks,
        playing: Option<usize>,
        actions: &mut Vec<SheetAction>,
    ) -> BarClicks {
        let Cell { bar, rect, .. } = cell;
        let ending = marks.ending_in(&bar);
        let id = ui.scope_id().with(("chord-sheet-bar", bar.number));
        let response = ui.interact(rect, id, Sense::click_and_drag());
        let key = keys.key_at(bar.start);
        let here = markers_in(project, &bar);
        // After the song and the last harmony written, nothing goes on.
        let last_bar_written = project
            .playlist()
            .chord_markers()
            .last()
            .map_or(0, |last| meter.bar_at(last.start).end());
        let sheet_end = project.song_length().max(last_bar_written);
        let before = (bar.start > 0 && bar.start < sheet_end)
            .then(|| project.chord_at(bar.start - 1))
            .flatten();
        let going_on = here
            .first()
            .is_none_or(|first| first.start > bar.start)
            .then_some(before)
            .flatten();

        // The paper and the bar lines: a double one where a section
        // starts, repeat signs around a chorus played more than once, and
        // the final bar line where the song ends.
        let fill = if response.hovered() {
            PAPER_HOVER
        } else {
            PAPER
        };
        painter.rect_filled(rect.shrink2(vec2(0.0, 0.5)), 0.0, fill);
        let line = Stroke::new(1.0, BAR_LINE);
        let thick = Stroke::new(3.0, BAR_LINE);
        let repeat_from = marks.repeat.is_some_and(|(start, _)| start == bar.start);
        let repeat_to = marks.repeat.is_some_and(|(_, end)| end == bar.end());
        painter.vline(rect.left(), rect.y_range(), line);
        if repeat_from {
            painter.vline(rect.left() + 1.5, rect.y_range(), thick);
            painter.vline(rect.left() + 5.0, rect.y_range(), line);
            repeat_dots(painter, rect.left() + 8.5, rect.center().y);
        } else if starts_section && bar.number > 0 {
            painter.vline(rect.left() + 3.0, rect.y_range(), line);
        }
        let song_end = project.song_length();
        if repeat_to {
            repeat_dots(painter, rect.right() - 8.5, rect.center().y);
            painter.vline(rect.right() - 5.0, rect.y_range(), line);
            painter.vline(rect.right() - 1.5, rect.y_range(), thick);
        } else if song_end > 0 && bar.start < song_end && bar.end() >= song_end {
            painter.vline(rect.right() - 5.0, rect.y_range(), line);
            painter.vline(rect.right() - 1.5, rect.y_range(), thick);
        } else if ends_line {
            painter.vline(rect.right(), rect.y_range(), line);
        }

        // The bar number, which moves the playhead there, and what
        // changes in this bar: the time signature and the key.
        let number_rect = painter.text(
            rect.left_top() + vec2(PADDING, 2.0),
            Align2::LEFT_TOP,
            (bar.number + 1).to_string(),
            FontId::proportional(SMALL_SIZE),
            theme::TEXT_DIM,
        );
        let number = ui.interact(number_rect.expand(3.0), id.with("number"), Sense::click());
        if theme::hint(number, "Move the playhead to this bar").clicked() {
            actions.push(SheetAction::Seek(bar.start));
        }
        let previous = (bar.number > 0).then(|| meter.bar(bar.number - 1));
        let mut marks_right = number_rect.right();
        if previous.is_none_or(|previous| previous.time != bar.time) {
            let time = painter.text(
                number_rect.right_top() + vec2(6.0, 0.0),
                Align2::LEFT_TOP,
                bar.time.to_string(),
                FontId::proportional(SMALL_SIZE),
                theme::ACCENT,
            );
            marks_right = time.right();
        }
        // Where the band changes variation: its letter in a box, as a
        // rehearsal mark is printed.
        if let Some(marker) = project
            .variations()
            .iter()
            .find(|marker| marker.start == bar.start)
        {
            let letter = painter.text(
                pos2(marks_right + 9.0, number_rect.top()),
                Align2::LEFT_TOP,
                marker.variation.letter(),
                FontId::proportional(SMALL_SIZE),
                theme::ACCENT,
            );
            painter.rect_stroke(
                letter.expand2(vec2(3.0, 0.5)),
                2.0,
                Stroke::new(1.0, theme::ACCENT),
                StrokeKind::Middle,
            );
        }
        if bar.number > 0 && keys.key_at(bar.start - 1) != key {
            painter.text(
                rect.right_top() + vec2(-PADDING, 2.0),
                Align2::RIGHT_TOP,
                format!("→ {}", key.name()),
                FontId::proportional(SMALL_SIZE),
                theme::ACCENT,
            );
        } else if ending.is_some_and(|ending| ending.end == bar.end()) {
            painter.text(
                rect.right_top() + vec2(-PADDING - 6.0, 2.0),
                Align2::RIGHT_TOP,
                "held",
                FontId::proportional(SMALL_SIZE),
                ENDING,
            );
        }

        // The harmonies, each where its beat falls. While the song
        // plays, the one sounding is the brightest thing on the sheet.
        let sounding = playing.and_then(|step| match marks.ending {
            Some(ending) if (ending.start..ending.end).contains(&step) => {
                Some(ChordMarker::new(ending.start, ending.harmony))
            }
            _ => project.chord_at(step),
        });
        let playing_here = playing.is_some_and(|step| (bar.start..bar.end()).contains(&step));
        let inner_left = rect.left() + PADDING + if repeat_from { 8.0 } else { 0.0 };
        let inner_right = rect.right() - PADDING - if repeat_to { 8.0 } else { 0.0 };
        let x_of = |step: usize| {
            let along = (step - bar.start) as f32 / bar.length.max(1) as f32;
            inner_left + (inner_right - inner_left) * along
        };
        let slot = |left: f32, right: f32| {
            Rect::from_x_y_ranges(
                left..=right.max(left + 1.0),
                rect.top() + TOP..=rect.bottom(),
            )
        };
        // The bar being typed in shows only the text field.
        let typed_in = self
            .editing
            .as_ref()
            .is_some_and(|editing| editing.bar == bar.number);
        // The ending's harmony is the band's, not written on the sheet.
        let ending_marker = ending.map(|ending| ChordMarker::new(ending.start, ending.harmony));
        let (going_on, shown) = match (typed_in, ending_marker) {
            (true, _) => (None, Vec::new()),
            (false, Some(marker)) if marker.start == bar.start => (None, vec![marker]),
            (false, Some(marker)) => (Some(marker), Vec::new()),
            (false, None) => (going_on, here.clone()),
        };
        let mut names: Vec<Name> = Vec::new();
        if let Some(marker) = going_on {
            let right = shown
                .first()
                .map_or(inner_right, |next| x_of(next.start) - 4.0);
            let color = if playing_here && sounding == Some(marker) {
                PLAYING
            } else {
                GOING_ON
            };
            names.push(Name {
                place: slot(inner_left, right),
                marker: ChordMarker::new(marker.start, marker.harmony),
                color,
                caption: None,
                going_on: true,
            });
        }
        for (index, marker) in shown.iter().enumerate() {
            let right = shown
                .get(index + 1)
                .map_or(inner_right, |next| x_of(next.start) - 4.0);
            let color = match (playing.is_some(), sounding == Some(*marker)) {
                (true, true) => PLAYING,
                (true, false) => theme::TEXT_DIM,
                _ if ending.is_some() => ENDING,
                _ if is_group(marker.harmony) => GROUP_TEXT,
                _ if marker.harmony == HarmonyName::NoChord => theme::TEXT_DIM,
                _ => theme::TEXT,
            };
            names.push(Name {
                place: slot(x_of(marker.start), right),
                marker: *marker,
                color,
                caption: caption(marker, key),
                going_on: false,
            });
        }
        draw_names(painter, &names);
        let slots: Vec<(Rect, ChordMarker)> =
            names.iter().map(|name| (name.place, name.marker)).collect();

        // A selection of several bars is shaded, and the bars that loop
        // have the loop's colour along their top.
        if let Some((first, last)) = self.selected_bars()
            && first < last
            && (first..=last).contains(&bar.number)
        {
            painter.rect_filled(rect.shrink(1.0), 0.0, theme::ACCENT.gamma_multiply(0.12));
        }
        if let Some(region) = project.playlist().loop_region()
            && bar.start < region.end
            && bar.end() > region.start
        {
            let top = Rect::from_x_y_ranges(rect.x_range(), rect.top()..=rect.top() + 3.0);
            painter.rect_filled(top, 0.0, theme::LOOP);
        }

        if playing_here {
            painter.rect_stroke(
                rect.shrink(1.5),
                2.0,
                Stroke::new(2.0, PLAYING),
                egui::StrokeKind::Inside,
            );
        } else if self.selected == Some(bar.number) && self.editing.is_none() {
            painter.rect_stroke(
                rect.shrink(1.0),
                2.0,
                Stroke::new(2.0, theme::ACCENT),
                egui::StrokeKind::Inside,
            );
        }

        // The mouse: hold a harmony down to hear it, click to select the
        // bar, double-click to type in it, right-click for more.
        let under = |pos: Option<Pos2>| {
            let pos = pos?;
            slots
                .iter()
                .find(|(place, _)| place.x_range().contains(pos.x))
                .map(|(_, marker)| *marker)
        };
        let primary = ui.input(|input| input.pointer.primary_down());
        if response.is_pointer_button_down_on() && primary && self.sounding.is_empty() {
            let pointer = response.interact_pointer_pos();
            if let Some(marker) = under(pointer) {
                self.sounding = voicing(marker.harmony);
                if !self.sounding.is_empty() {
                    actions.push(SheetAction::NoteOn(self.sounding.clone()));
                }
            }
        }
        let mut clicks = BarClicks {
            clicked: response.clicked() || response.secondary_clicked(),
            start_typing: None,
        };
        // A right-click inside the bars selected keeps them selected, for
        // the menu to act on.
        let in_selection = self
            .selected_bars()
            .is_some_and(|(first, last)| first < last && (first..=last).contains(&bar.number));
        let keeps_selection = response.secondary_clicked() && in_selection;
        if (clicks.clicked || response.drag_started()) && !keeps_selection {
            let shift = ui.input(|input| input.modifiers.shift);
            self.select_to(bar.number, shift);
            self.error = None;
        }
        if response.double_clicked() {
            clicks.start_typing = Some((bar.number, write_bar(&here, &bar)));
        }
        let hint = match (under(response.hover_pos()), ending) {
            (_, Some(_)) => format!(
                "Bar {}, the band's ending: a bar on the key's tonic, then the chord struck \
                 once and held. Type a chord here, and the chorus takes this bar in",
                bar.number + 1
            ),
            (Some(marker), None) => describe(&marker, key),
            (None, None) => format!(
                "Bar {}: click it and type its harmony, or double-click to change it. \
                 Right-click for more",
                bar.number + 1
            ),
        };
        let selection = self
            .selected_bars()
            .filter(|&(first, last)| first < last && (first..=last).contains(&bar.number));
        theme::hint(response, hint).context_menu(|ui| {
            if ui.button("Type in this bar").clicked() {
                clicks.start_typing = Some((bar.number, write_bar(&here, &bar)));
            }
            let clear = ui.add_enabled(!here.is_empty(), egui::Button::new("Clear this bar"));
            if clear.clicked() {
                actions.push(SheetAction::SetChords {
                    start: bar.start,
                    end: bar.end(),
                    markers: Vec::new(),
                });
            }
            if ui.button("Move the playhead here").clicked() {
                actions.push(SheetAction::Seek(bar.start));
            }
            if project.band().is_some() {
                ui.separator();
                variation_menu(ui, project, meter, bar.number, selection, actions);
                for marker in here
                    .iter()
                    .filter(|marker| marker.harmony != HarmonyName::NoChord)
                {
                    ui.menu_button(format!("Band on {}", marker.harmony), |ui| {
                        breaks_menu(ui, &bar, &here, *marker, actions);
                    });
                }
            }
            if project.band().is_some() && ending.is_none() {
                let end = ui.button("End the song here");
                if theme::hint(
                    end,
                    "The band stops after this bar; chords after it are cleared",
                )
                .clicked()
                {
                    actions.push(SheetAction::EndHere(bar.number));
                }
            }
        });
        clicks
    }

    /// Forgets the bars of the song it showed, for another song: what
    /// was being typed in a bar is not written into the new one.
    pub fn new_song(&mut self) {
        self.selected = None;
        self.anchor = None;
        self.editing = None;
        self.error = None;
        self.clipboard = None;
        self.followed = None;
        self.reveal = false;
    }

    /// Selects `bar`, as a click on it does, so what comes next is
    /// written there.
    pub fn select(&mut self, bar: usize) {
        self.selected = Some(bar);
        self.anchor = None;
        self.editing = None;
        self.clipboard = None;
        self.reveal = true;
    }

    /// Selects `bar`; with `shift`, the bars from the one selected
    /// before up to it.
    fn select_to(&mut self, bar: usize, shift: bool) {
        self.anchor = match (shift, self.anchor, self.selected) {
            (false, ..) => None,
            (true, Some(anchor), _) => Some(anchor),
            (true, None, selected) => selected,
        };
        self.selected = Some(bar);
        self.clipboard = None;
    }

    /// The first and the last bar selected, if any: one bar, or several
    /// selected with Shift.
    fn selected_bars(&self) -> Option<(usize, usize)> {
        let selected = self.selected?;
        let anchor = self.anchor.unwrap_or(selected);
        Some((anchor.min(selected), anchor.max(selected)))
    }

    /// The bar a progression is written from: the selected one, or the
    /// first after everything written.
    fn next_bar(&self, project: &Project) -> usize {
        if let Some(bar) = self.selected {
            return bar;
        }
        let meter = project.meter();
        let after_chords = project
            .playlist()
            .chord_markers()
            .last()
            .map_or(0, |last| meter.bar_at(last.start).number + 1);
        let after_band = project.band().map_or(0, |band| match band.end() {
            0 => 0,
            end => meter.bar_at(end - 1).number + 1,
        });
        after_chords.max(after_band)
    }

    /// The lines of bars: as many as the song has, or as the chords
    /// written reach, and one more empty line to write on.
    fn lines(&self, project: &Project, meter: &Meter) -> Vec<Line> {
        let markers = project.playlist().chord_markers();
        let reach = project
            .song_length()
            .max(markers.last().map_or(0, |marker| marker.start + 1));
        let mut count = match reach {
            0 => 0,
            reach => meter.bar_at(reach - 1).number + 1,
        };
        let typed = self.editing.as_ref().map(|editing| editing.bar);
        for bar in [self.selected, typed].into_iter().flatten() {
            count = count.max(bar + 1);
        }
        let count = (count.div_ceil(BARS_PER_LINE) + 1).max(2) * BARS_PER_LINE;
        let sections = project.playlist().sections();
        let marks = Marks::of(project);
        let form = project.form().filter(|form| !form.is_plain());
        let chorus = project.chorus();
        let mut lines: Vec<Line> = Vec::new();
        let bars = meter
            .bars()
            .take(count)
            .take_while(|bar| bar.start < MAX_SONG_STEPS);
        for bar in bars {
            let section = sections
                .iter()
                .find(|section| meter.bar_at(section.start).number == bar.number);
            let section = section.map(|section| match section.repeats {
                1 => section.name.clone(),
                repeats => format!("{} ×{repeats}", section.name),
            });
            // The parts of the band's form, when it has more than one.
            let part = form.and_then(|form| match chorus {
                _ if form.intro > 0 && bar.start == 0 => Some("Intro".to_string()),
                Some((start, _)) if start == bar.start => Some(match form.choruses {
                    1 => "Chorus".to_string(),
                    choruses => format!("Chorus ×{choruses}"),
                }),
                _ => marks
                    .ending
                    .filter(|ending| ending.start == bar.start)
                    .map(|_| "Ending".to_string()),
            });
            let heading = match (part, section) {
                (Some(part), Some(section)) => Some(format!("{part} · {section}")),
                (part, section) => part.or(section),
            };
            let full = lines
                .last()
                .is_none_or(|line| line.bars.len() == BARS_PER_LINE);
            if full || heading.is_some() {
                lines.push(Line {
                    heading,
                    bars: Vec::new(),
                });
            }
            if let Some(line) = lines.last_mut() {
                line.bars.push(bar);
            }
        }
        lines
    }

    fn start_editing(&mut self, bar: usize, text: String, fresh: Fresh) {
        self.selected = Some(bar);
        self.anchor = None;
        self.clipboard = None;
        self.editing = Some(Editing {
            bar,
            text,
            fresh: Some(fresh),
        });
    }

    /// Reads what was typed in the bar being typed in, and writes it if
    /// it changes the bar. Returns whether it could be read; if not, the
    /// reason is kept to show.
    fn take_in(
        &mut self,
        project: &Project,
        cells: &[Cell],
        keys: &KeyMap,
        actions: &mut Vec<SheetAction>,
    ) -> bool {
        let Some(editing) = &self.editing else {
            return true;
        };
        let Some(cell) = cells.iter().find(|cell| cell.bar.number == editing.bar) else {
            return true;
        };
        let bar = cell.bar;
        match read_bar(&editing.text, &bar, keys.key_at(bar.start)) {
            Ok(markers) => {
                if markers != markers_in(project, &bar) {
                    actions.push(SheetAction::SetChords {
                        start: bar.start,
                        end: bar.end(),
                        markers,
                    });
                }
                self.error = None;
                true
            }
            Err(error) => {
                self.error = Some((bar.number, error));
                false
            }
        }
    }

    /// The text field over the bar being typed in.
    fn edit_field(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        cells: &[Cell],
        keys: &KeyMap,
        sheet: egui::Id,
        actions: &mut Vec<SheetAction>,
    ) {
        let Some(editing) = &mut self.editing else {
            return;
        };
        let Some(&Cell { rect, .. }) = cells.iter().find(|cell| cell.bar.number == editing.bar)
        else {
            self.editing = None;
            return;
        };
        let id = ui.scope_id().with("chord-sheet-field");
        let had_focus = ui.memory(|memory| memory.has_focus(id));
        // Tab goes on to the next bar, not to the next widget.
        let (tab, back) = if had_focus {
            ui.input_mut(|input| {
                let back = input.consume_key(Modifiers::SHIFT, Key::Tab);
                (input.consume_key(Modifiers::NONE, Key::Tab), back)
            })
        } else {
            (false, false)
        };
        let fresh = editing.fresh.take();
        if let Some(fresh) = fresh {
            let mut state = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
            let range = match fresh {
                Fresh::SelectAll => egui::text::CCursorRange::select_all_str(&editing.text),
                Fresh::CursorAtEnd => egui::text::CCursorRange::one(egui::text::CCursor::new(
                    editing.text.chars().count(),
                )),
            };
            state.cursor.set_char_range(Some(range));
            state.store(ui.ctx(), id);
            ui.memory_mut(|memory| memory.request_focus(id));
        }
        let field = egui::TextEdit::singleline(&mut editing.text)
            .id(id)
            .font(FontId::proportional(16.0))
            .hint_text("Am7 D7")
            .desired_width(rect.width() - 2.0 * PADDING);
        let place = Rect::from_min_size(
            rect.left_top() + vec2(PADDING - 2.0, TOP),
            vec2(rect.width() - 2.0 * PADDING + 4.0, 26.0),
        );
        let response = ui.put(place, field);
        ui.memory_mut(|memory| memory.set_focus_lock_filter(id, KEEP_KEYS));
        let bar = editing.bar;
        let last = cells.len().saturating_sub(1);
        let escape = ui.input(|input| input.key_pressed(Key::Escape));
        let enter = response.lost_focus() && ui.input(|input| input.key_pressed(Key::Enter));
        if escape {
            self.editing = None;
            self.error = None;
            ui.memory_mut(|memory| memory.request_focus(sheet));
        } else if tab || back {
            if self.take_in(project, cells, keys, actions) {
                let next = if back {
                    bar.saturating_sub(1)
                } else {
                    (bar + 1).min(last)
                };
                let text = bar_text(project, cells, next);
                self.start_editing(next, text, Fresh::SelectAll);
                self.reveal = true;
            } else if let Some(editing) = &mut self.editing {
                editing.fresh = Some(Fresh::CursorAtEnd);
            }
        } else if enter {
            if self.take_in(project, cells, keys, actions) {
                // Enter goes on to the next bar, ready to type in.
                self.editing = None;
                self.selected = Some((bar + 1).min(last));
                self.reveal = true;
                ui.memory_mut(|memory| memory.request_focus(sheet));
            } else if let Some(editing) = &mut self.editing {
                editing.fresh = Some(Fresh::CursorAtEnd);
            }
        } else if fresh.is_none() && !response.has_focus() {
            // Clicked somewhere else: keep what can be read.
            self.take_in(project, cells, keys, actions);
            self.editing = None;
        }
    }

    /// The keys, while the sheet has the keyboard and no bar is being
    /// typed in.
    fn keys(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        cells: &[Cell],
        typing_keys: bool,
        actions: &mut Vec<SheetAction>,
    ) {
        if self.clipboard_keys(ui, project, actions) {
            return;
        }
        let pressed = |key| ui.input(|input| input.key_pressed(key));
        let Some(selected) = self.selected else {
            if pressed(Key::ArrowRight) || pressed(Key::ArrowDown) {
                self.selected = Some(0);
            }
            return;
        };
        let last = cells.len().saturating_sub(1);
        let cell_of = |bar: usize| cells.iter().find(|cell| cell.bar.number == bar);
        // The bar in `line` nearest to `column`.
        let in_line = |line: usize, column: usize| {
            cells
                .iter()
                .filter(|cell| cell.line == line)
                .min_by_key(|cell| cell.column.abs_diff(column))
                .map(|cell| cell.bar.number)
        };
        let mut moved_to = None;
        if pressed(Key::ArrowRight) || (pressed(Key::Tab) && !ui.input(|i| i.modifiers.shift)) {
            moved_to = Some((selected + 1).min(last));
        }
        if pressed(Key::ArrowLeft) || (pressed(Key::Tab) && ui.input(|i| i.modifiers.shift)) {
            moved_to = Some(selected.saturating_sub(1));
        }
        if let Some(cell) = cell_of(selected) {
            if pressed(Key::ArrowDown) {
                moved_to = in_line(cell.line + 1, cell.column).or(moved_to);
            }
            if pressed(Key::ArrowUp) && cell.line > 0 {
                moved_to = in_line(cell.line - 1, cell.column).or(moved_to);
            }
        }
        if let Some(bar) = moved_to {
            let shift = ui.input(|input| input.modifiers.shift);
            self.select_to(bar, shift);
            self.reveal = true;
            self.error = None;
            return;
        }
        if pressed(Key::Escape) {
            self.selected = None;
            self.anchor = None;
            return;
        }
        if pressed(Key::Delete) || pressed(Key::Backspace) {
            // Every bar selected is cleared, as one change.
            let (first, last) = self.selected_bars().unwrap_or((selected, selected));
            if let (Some(first), Some(last)) = (cell_of(first), cell_of(last)) {
                actions.push(SheetAction::SetChords {
                    start: first.bar.start,
                    end: last.bar.end(),
                    markers: Vec::new(),
                });
            }
            return;
        }
        if pressed(Key::Enter) || pressed(Key::F2) {
            let text = bar_text(project, cells, selected);
            self.start_editing(selected, text, Fresh::SelectAll);
            return;
        }
        // Typing starts typing in the bar, over what it held. With the
        // typing keyboard on, letters play notes instead; L is the key
        // that switches between pattern and song.
        let typed: String = ui.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Text(text) => Some(text.as_str()),
                    _ => None,
                })
                .collect()
        });
        let starts = typed.chars().next().is_some_and(|c| {
            let letter = c.is_ascii_alphabetic();
            !c.is_whitespace() && !(letter && typing_keys) && !matches!(c, 'l' | 'L')
        });
        if starts && !ui.input(|input| input.modifiers.command) {
            self.start_editing(selected, typed, Fresh::CursorAtEnd);
        }
    }
}

impl ChordSheet {
    /// Ctrl+C, Ctrl+V and Ctrl+A, while the sheet has the keyboard.
    /// Returns whether one of them was pressed.
    fn clipboard_keys(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        actions: &mut Vec<SheetAction>,
    ) -> bool {
        let (copy, cut, paste) = ui.input(|input| {
            let mut copy = false;
            let mut cut = false;
            let mut paste = None;
            for event in &input.events {
                match event {
                    egui::Event::Copy => copy = true,
                    egui::Event::Cut => cut = true,
                    egui::Event::Paste(text) => paste = Some(text.clone()),
                    _ => {}
                }
            }
            (copy, cut, paste)
        });
        let all = ui.input_mut(|input| {
            input.consume_shortcut(&egui::KeyboardShortcut::new(Modifiers::COMMAND, Key::A))
        });
        let bars = text::sheet_bars(project);
        if all {
            if bars > 0 {
                self.anchor = Some(0);
                self.selected = Some(bars - 1);
                self.error = None;
            }
            return true;
        }
        if copy || cut {
            self.copy(ui.ctx(), project, bars);
            if cut && let Some((first, last)) = self.selected_bars() {
                let meter = project.meter();
                actions.push(SheetAction::SetChords {
                    start: meter.bar(first).start,
                    end: meter.bar(last).end(),
                    markers: Vec::new(),
                });
            }
            return true;
        }
        if let Some(pasted) = paste {
            self.paste(&pasted, project, actions);
            return true;
        }
        false
    }

    /// Copies the bars selected as text, or the whole sheet when every bar
    /// (or none) is selected: then with its key, time signature, tempo,
    /// style and sections, as File > Export chord sheet writes it.
    fn copy(&mut self, ctx: &egui::Context, project: &Project, bars: usize) {
        let (first, last) = self.selected_bars().unwrap_or((0, bars.saturating_sub(1)));
        let whole = first == 0 && last + 1 >= bars;
        let copied = if whole {
            text::write(project, None)
        } else {
            text::write_bars(project, first..last + 1)
        };
        ctx.copy_text(copied);
        let count = if whole { bars } else { last + 1 - first };
        self.error = None;
        self.clipboard = Some((
            format!(
                "Copied {count} bar{} as text.",
                if count == 1 { "" } else { "s" }
            ),
            false,
        ));
    }

    /// Writes text from the selected bar on (or from the first bar): a
    /// chart with bars between | signs, or what one bar would hold, as in
    /// "Dm7 G7". The bars written are selected afterwards.
    fn paste(&mut self, pasted: &str, project: &Project, actions: &mut Vec<SheetAction>) {
        let first = self.selected_bars().map_or(0, |(first, _)| first);
        self.error = None;
        let written = if pasted.contains('|') || pasted.trim().lines().count() > 1 {
            text::read(pasted)
                .and_then(|sheet| text::paste(&sheet, project, first))
                .map_err(|err| format!("The text pasted could not be read, {err}"))
        } else {
            let bar = project.meter().bar(first);
            read_bar(pasted, &bar, project.key_map().key_at(bar.start))
                .map(|markers| (bar.start..bar.end(), markers))
                .map_err(|err| format!("Bar {}: {err}", first + 1))
        };
        match written {
            Ok((steps, markers)) if !steps.is_empty() => {
                let meter = project.meter();
                let last = meter.bar_at(steps.end - 1).number;
                let count = last + 1 - first;
                actions.push(SheetAction::SetChords {
                    start: steps.start,
                    end: steps.end,
                    markers,
                });
                self.anchor = (count > 1).then_some(first);
                self.selected = Some(last);
                self.reveal = true;
                self.clipboard = Some((
                    format!("Pasted {count} bar{}.", if count == 1 { "" } else { "s" }),
                    false,
                ));
            }
            Ok(_) => {
                self.clipboard = Some(("The text pasted has no bars.".to_string(), true));
            }
            Err(problem) => self.clipboard = Some((problem, true)),
        }
    }
}

/// The keys the sheet keeps while it has the keyboard: the arrows move
/// from bar to bar and Tab goes to the next one.
const KEEP_KEYS: egui::EventFilter = egui::EventFilter {
    tab: true,
    horizontal_arrows: true,
    vertical_arrows: true,
    escape: false,
};

/// Pick from the same styles and rhythms as the Library sidebar.
fn band_menu(
    ui: &mut Ui,
    project: &Project,
    library: &mut LibraryBrowser,
    actions: &mut Vec<SheetAction>,
) {
    let title = project.band().map_or_else(
        || "🎷 Band: none".to_string(),
        |band| format!("🎷 Band: {}", band.name()),
    );
    let menu = egui::containers::menu::MenuButton::new(title).config(
        egui::containers::menu::MenuConfig::new()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside),
    );
    let parts_before = library.parts;
    let (response, _) = menu.ui(ui, |ui| {
        ui.set_width(300.0);
        if ui
            .selectable_label(project.band().is_none(), "No band")
            .clicked()
        {
            actions.push(SheetAction::SetBand {
                style: None,
                harmony: None,
            });
            ui.close();
        }
        ui.separator();
        let tried = project.band().map_or_else(Tried::default, Tried::from_band);
        if let Some(picked) = library.show_for(ui, project, tried, LibraryPurpose::Band) {
            let (style, harmony) = match picked {
                LibraryAction::UserStyle(index) => {
                    actions.push(SheetAction::UserStyle {
                        index,
                        progression_from: None,
                    });
                    ui.close();
                    return;
                }
                LibraryAction::Style(style) => (BandStyle::of_style(style), Some(style)),
                LibraryAction::Rhythm(rhythm) => (BandStyle::named(rhythm.name), None),
                _ => return,
            };
            if let Some(style) = style {
                actions.push(SheetAction::SetBand {
                    style: Some(style),
                    harmony,
                });
                ui.close();
            }
        }
        if let Some(band) = project.band() {
            ui.separator();
            soloist_items(ui, band, actions);
        }
    });
    // A preference edit is not a menu selection. Retain the popup after
    // its container has processed the click, including when it resizes.
    if library.parts != parts_before && !ui.input(|input| input.key_pressed(Key::Escape)) {
        let popup_id = egui::Popup::default_response_id(&response);
        if !egui::Popup::is_id_open(ui.ctx(), popup_id) {
            egui::Popup::open_id(ui.ctx(), popup_id);
        }
    }
    theme::hint(
        response,
        "Choose the band's playing from the Library's styles and rhythms. Search by name, genre or meter.",
    );
}

/// The experimental soloist: on or off, and another solo.
fn soloist_items(ui: &mut Ui, band: &Band, actions: &mut Vec<SheetAction>) {
    let soloist = Soloist::of(band.style.genre);
    let instrument = general_midi::NAMES[usize::from(soloist.program)].to_lowercase();
    let mut on = band.has_soloist();
    let check = ui.checkbox(&mut on, "Soloist (experimental)");
    let hint = format!(
        "A {instrument} improvises over the chorus: chord tones on the beats, scale and \
         chromatic notes between them. The piano roll says what each of its notes is"
    );
    if theme::hint(check, hint).clicked() {
        actions.push(SheetAction::SetSoloist(on));
        ui.close();
    }
    let again = ui.add_enabled(band.has_soloist(), egui::Button::new("Another solo"));
    if theme::hint(again, "Have the soloist improvise again, a different solo").clicked() {
        actions.push(SheetAction::AnotherSolo);
        ui.close();
    }
}

/// The same Library browser, restricted to styles with chord progressions.
fn progression_menu(
    ui: &mut Ui,
    project: &Project,
    from: usize,
    library: &mut LibraryBrowser,
    actions: &mut Vec<SheetAction>,
) {
    let tonal = project.scale.kind.is_tonal();
    ui.add_enabled_ui(tonal, |ui| {
        let menu = egui::containers::menu::MenuButton::new("Progression").config(
            egui::containers::menu::MenuConfig::new()
                .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside),
        );
        let (response, _) = menu.ui(ui, |ui| {
            ui.set_width(300.0);
            let tried = project.band().map_or_else(Tried::default, Tried::from_band);
            match library.show_for(ui, project, tried, LibraryPurpose::Progression) {
                Some(LibraryAction::Style(style)) => {
                    actions.push(SheetAction::WriteProgression { style, from });
                    ui.close();
                }
                Some(LibraryAction::UserStyle(index)) => {
                    actions.push(SheetAction::UserStyle {
                        index,
                        progression_from: Some(from),
                    });
                    ui.close();
                }
                _ => {}
            }
        });
        theme::hint(
            response,
            format!("Write a style's chord progression from bar {} on", from + 1),
        )
        .on_disabled_hover_text("Progressions are chords in a key; this song is in a set or a row");
    });
}

/// The variations of the band's style, in a bar's menu. They apply to
/// the bars `selection` holds, when the bar right-clicked is one of
/// them, or else from bar `number` on.
fn variation_menu(
    ui: &mut Ui,
    project: &Project,
    meter: &Meter,
    number: usize,
    selection: Option<(usize, usize)>,
    actions: &mut Vec<SheetAction>,
) {
    let (start, end) = match selection {
        Some((first, last)) => (meter.bar(first).start, Some(meter.bar(last).end())),
        None => (meter.bar(number).start, None),
    };
    let playing = project.variation_at(start);
    let bars = match selection {
        Some((first, last)) => format!("Bars {}–{}: the band", first + 1, last + 1),
        None => format!("From bar {} on, the band", number + 1),
    };
    for variation in [Variation::A, Variation::B] {
        let about = match variation {
            Variation::A => "plays the style as it is",
            Variation::B => {
                "plays the style fuller: the drums on the ride cymbal, a busier bass, \
                 piano or guitar, strings or an organ joining in where the style has them, \
                 a little louder. The drums fill into it"
            }
        };
        let item = ui.radio(
            playing == variation,
            format!("Variation {}", variation.letter()),
        );
        if theme::hint(item, format!("{bars} {about}")).clicked() {
            actions.push(SheetAction::Form(FormAction::Variation {
                start,
                end,
                variation,
            }));
            ui.close();
        }
    }
}

/// The two dots of a repeat sign, around the middle of a bar line.
fn repeat_dots(painter: &egui::Painter, x: f32, middle: f32) {
    for y in [middle - 5.0, middle + 5.0] {
        painter.circle_filled(pos2(x, y), 1.8, BAR_LINE);
    }
}

/// The harmonies written in `bar`.
fn markers_in(project: &Project, bar: &Bar) -> Vec<ChordMarker> {
    project
        .playlist()
        .chord_markers()
        .iter()
        .filter(|marker| (bar.start..bar.end()).contains(&marker.start))
        .copied()
        .collect()
}

/// What bar `bar` holds, as text to type over.
fn bar_text(project: &Project, cells: &[Cell], bar: usize) -> String {
    cells
        .iter()
        .find(|cell| cell.bar.number == bar)
        .map(|cell| write_bar(&markers_in(project, &cell.bar), &cell.bar))
        .unwrap_or_default()
}

/// Whether a harmony is a group of notes with no function in a key: a
/// set, a micromode or a row form.
fn is_group(harmony: HarmonyName) -> bool {
    matches!(
        harmony,
        HarmonyName::Set(_) | HarmonyName::Micromode(_) | HarmonyName::Row(_)
    )
}

/// A harmony to draw in a bar: where, in which colour, and its caption.
struct Name {
    place: Rect,
    marker: ChordMarker,
    color: Color32,
    caption: Option<String>,
    /// It goes on from an earlier bar: drawn faintly, in what room is
    /// left before the bar's own first harmony.
    going_on: bool,
}

/// Draws the harmonies of a bar, each with its caption under it. The
/// bar's own names are all drawn at one size, as big as the most crowded
/// one allows, so a bar does not mix big and small names; a name going
/// on from before only shrinks itself.
fn draw_names(painter: &egui::Painter, names: &[Name]) {
    let fits = |name: &Name| {
        let text = name.marker.text();
        let font = FontId::proportional(NAME_SIZE);
        let width = painter.layout_no_wrap(text, font, name.color).size().x;
        NAME_SIZE * (name.place.width().max(1.0) / width).min(1.0)
    };
    let size = names
        .iter()
        .filter(|name| !name.going_on)
        .map(fits)
        .fold(NAME_SIZE, f32::min)
        .max(SMALLEST_NAME);
    for name in names {
        let size = match name.going_on {
            true => fits(name).max(SMALLEST_NAME),
            false => size,
        };
        let clipped = painter.with_clip_rect(name.place.intersect(painter.clip_rect()));
        clipped.text(
            name.place.left_top(),
            Align2::LEFT_TOP,
            name.marker.text(),
            FontId::proportional(size),
            name.color,
        );
        if let Some(caption) = &name.caption {
            clipped.text(
                pos2(name.place.left(), name.place.top() + NAME_SIZE + 4.0),
                Align2::LEFT_TOP,
                caption,
                FontId::proportional(CAPTION_SIZE),
                theme::TEXT_DIM,
            );
        }
    }
}

/// The words under a harmony on the sheet: its numeral or its notes (see
/// [`HarmonyName::caption`]), and what the band does there if it leaves
/// its style: "V · push · shot".
fn caption(marker: &ChordMarker, key: lineup_core::Scale) -> Option<String> {
    let words: Vec<String> = marker
        .harmony
        .caption(key)
        .into_iter()
        .chain(marker.push.map(|_| "push".to_string()))
        .chain(marker.stop.map(|stop| stop.name().to_string()))
        .collect();
    (!words.is_empty()).then(|| words.join(" · "))
}

/// A sentence about a harmony and what the band does there, for its
/// hint.
fn describe(marker: &ChordMarker, key: lineup_core::Scale) -> String {
    let mut text = marker.harmony.describe(key);
    if let Some(push) = marker.push {
        text.push_str(&format!(" Push: {}.", push.describe()));
    }
    if let Some(stop) = marker.stop {
        text.push_str(&format!(
            " {}: {}.",
            capitalized(stop.name()),
            stop.describe()
        ));
    }
    text
}

/// The menu that marks a harmony of `bar` (one of its markers, `here`)
/// as a shot, a hold, a rest or a push, as typing the marks would.
fn breaks_menu(
    ui: &mut Ui,
    bar: &Bar,
    here: &[ChordMarker],
    marker: ChordMarker,
    actions: &mut Vec<SheetAction>,
) {
    let mut chosen = None;
    let style = ui.radio(marker.stop.is_none(), "As the style goes");
    if theme::hint(style, "The band plays its style through this harmony").clicked() {
        chosen = Some(marker.stopped(None));
    }
    for stop in Stop::ALL {
        let label = format!("{} ({})", capitalized(stop.name()), stop.marks());
        let item = ui.radio(marker.stop == Some(stop), label);
        if theme::hint(item, format!("{}: {}", marker.harmony, stop.describe())).clicked() {
            chosen = Some(marker.stopped(Some(stop)));
        }
    }
    ui.separator();
    let on_time = ui.radio(marker.push.is_none(), "On the beat");
    if theme::hint(on_time, "The band plays it on the beat it is written on").clicked() {
        chosen = Some(marker.pushed(None));
    }
    for (push, label) in [
        (Push::Eighth, "Push (^)"),
        (Push::Sixteenth, "Push a sixteenth (^^)"),
    ] {
        let item = ui.radio(marker.push == Some(push), label);
        if theme::hint(item, format!("{}: {}", marker.harmony, push.describe())).clicked() {
            chosen = Some(marker.pushed(Some(push)));
        }
    }
    if let Some(changed) = chosen {
        let markers = here
            .iter()
            .map(|other| match other.start == marker.start {
                true => changed,
                false => *other,
            })
            .collect();
        actions.push(SheetAction::SetChords {
            start: bar.start,
            end: bar.end(),
            markers,
        });
        ui.close();
    }
}

/// `word` with a capital first letter.
fn capitalized(word: &str) -> String {
    let mut letters = word.chars();
    letters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(letters).collect()
    })
}

/// The notes to hear for a harmony: a chord with its bass under it from
/// C3 and its notes from the root, from C4 up; a set or a micromode in
/// normal form, from C4 up. A row form and no chord have none.
fn voicing(harmony: HarmonyName) -> Vec<Note> {
    match harmony {
        HarmonyName::Chord(symbol) => {
            let chord = symbol.chord();
            let root = 60 + chord.root.semitone();
            let mut notes = vec![Note::new(48 + symbol.bass_pitch_class().semitone())];
            notes.extend(
                chord
                    .quality
                    .intervals()
                    .iter()
                    .map(|&interval| Note::new(root + interval)),
            );
            notes
        }
        HarmonyName::Set(set) | HarmonyName::Micromode(set) => {
            let mut notes: Vec<Note> = Vec::new();
            for pitch_class in set.normal_form() {
                let mut midi = 60 + pitch_class.semitone();
                while notes.last().is_some_and(|last| last.midi() >= midi) {
                    midi += 12;
                }
                notes.push(Note::new(midi));
            }
            notes
        }
        HarmonyName::Row(_) | HarmonyName::NoChord => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::{Event, FullOutput, PointerButton, RawInput, Shape};

    #[test]
    fn select_all_works_when_the_chord_window_owns_the_keyboard() {
        let ctx = egui::Context::default();
        let project = Project::demo();
        let mut sheet = ChordSheet::new();
        sheet.open = true;
        let mut library = LibraryBrowser::new();
        let mut frame = |events: Vec<Event>| {
            let mut output = ctx.run_pass(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0))),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    sheet.show(
                        ctx,
                        ctx.content_rect(),
                        &project,
                        None,
                        false,
                        (
                            PracticeView {
                                count_in: false,
                                metronome: false,
                                speed: 100,
                            },
                            &mut library,
                        ),
                    );
                },
            );
            output.textures_delta.clear();
        };
        frame(vec![]);
        frame(vec![]);
        // No bar-canvas click is required: the active window receives it.
        frame(vec![Event::Key {
            key: Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::COMMAND,
        }]);
        assert_eq!(
            sheet.selected_bars(),
            Some((0, text::sheet_bars(&project) - 1))
        );
    }

    fn heard(text: &str) -> Vec<u8> {
        voicing(HarmonyName::parse(text).unwrap())
            .iter()
            .map(|note| note.midi())
            .collect()
    }

    #[test]
    fn a_harmony_is_heard_with_its_bass_under_it() {
        assert_eq!(heard("C"), [48, 60, 64, 67]);
        assert_eq!(heard("G7/B"), [59, 67, 71, 74, 77]);
        // A set climbs from C4 in normal form: B C# E is 11 1 4.
        assert_eq!(heard("{B C# E}"), [71, 73, 76]);
        assert!(heard("P0").is_empty());
        assert!(heard("N.C.").is_empty());
    }

    struct MenuScreen {
        ctx: egui::Context,
        project: Project,
        library: LibraryBrowser,
        sidebar: bool,
        screen_size: egui::Vec2,
    }

    impl MenuScreen {
        fn new() -> Self {
            let ctx = egui::Context::default();
            ctx.all_styles_mut(|style| style.animation_time = 0.0);
            Self {
                ctx,
                project: Project::with_band(),
                library: LibraryBrowser::new(),
                sidebar: false,
                screen_size: vec2(1280.0, 1200.0),
            }
        }

        fn frame(&mut self, events: Vec<Event>) -> (FullOutput, Vec<SheetAction>) {
            let mut actions = Vec::new();
            let mut output = self.ctx.run_pass(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.screen_size)),
                    time: Some(self.ctx.cumulative_frame_nr() as f64 * 0.1),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::Window::new("Style selectors")
                        .fixed_pos(Pos2::ZERO)
                        .default_size(vec2(900.0, 850.0))
                        .show(ctx, |ui| {
                            if self.sidebar {
                                self.library.show(
                                    ui,
                                    &self.project,
                                    Tried::from_band(self.project.band().unwrap()),
                                );
                            } else {
                                ui.horizontal(|ui| {
                                    band_menu(ui, &self.project, &mut self.library, &mut actions);
                                    progression_menu(
                                        ui,
                                        &self.project,
                                        2,
                                        &mut self.library,
                                        &mut actions,
                                    );
                                });
                            }
                        });
                },
            );
            output.textures_delta.clear();
            (output, actions)
        }

        fn click(&mut self, output: &FullOutput, text: &str) -> (FullOutput, Vec<SheetAction>) {
            let pos = output
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    Shape::Text(shape) if shape.galley.text() == text => {
                        Some(shape.pos + shape.galley.size() * 0.5)
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("Missing menu item: {text}"));
            self.frame(vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Modifiers::NONE,
                },
            ]);
            self.frame(vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            }])
        }
    }

    #[test]
    fn band_copy_options_can_be_changed_without_reopening_the_menu() {
        for height in [1200.0, 720.0, 480.0] {
            let mut screen = MenuScreen::new();
            screen.screen_size.y = height;
            screen.frame(vec![]);
            let (output, _) = screen.frame(vec![]);
            screen.click(&output, "🎷 Band: Pop beat");
            let (output, _) = screen.frame(vec![]);
            screen.click(&output, "Apply from styles and rhythms");
            let (mut output, _) = screen.frame(vec![]);
            for label in [
                "Rhythm patterns",
                "Voices / instruments",
                "Time signature",
                "Chord progression",
                "Tempo",
                "Swing",
                "Use matching style for rhythms",
            ] {
                let pos = output
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        Shape::Text(shape) if shape.galley.text() == label => {
                            Some(shape.pos + shape.galley.size() * 0.5)
                        }
                        _ => None,
                    })
                    .unwrap();
                screen.frame(vec![Event::PointerMoved(pos)]);
                for _ in 0..10 {
                    screen.frame(vec![]);
                }
                (output, _) = screen.frame(vec![]);
                let (_, actions) = screen.click(&output, label);
                assert!(
                    actions.is_empty(),
                    "changing {label} should only update copy preferences"
                );
                (output, _) = screen.frame(vec![]);
                assert!(
                    output.shapes.iter().any(|shape| matches!(&shape.shape,
                        Shape::Text(text) if text.galley.text() == "Apply from styles and rhythms"
                    )),
                    "the style menu closed after changing {label}"
                );
            }
            let parts = screen.library.parts;
            assert!(!parts.rhythm && !parts.instruments && !parts.time_signature);
            assert!(parts.progression);
            assert!(!parts.tempo && !parts.swing);
            assert!(!parts.matching_style);

            // Picking a style still closes the menu after several edits.
            screen.click(&output, "Apply from styles and rhythms");
            let (output, _) = screen.frame(vec![]);
            screen.click(&output, "🔍 Search: bossa, 6/8, tango…");
            let (output, _) = screen.frame(vec![Event::Text("royal road".to_string())]);
            let (_, actions) = screen.click(&output, "Royal road");
            assert!(
                actions
                    .iter()
                    .any(|action| matches!(action, SheetAction::SetBand { .. }))
            );
            let (output, _) = screen.frame(vec![]);
            assert!(!output.shapes.iter().any(|shape| matches!(&shape.shape,
                Shape::Text(text) if text.galley.text() == "Apply from styles and rhythms"
            )));

            // An ordinary outside click still dismisses it.
            screen.click(&output, "🎷 Band: Pop beat");
            screen.frame(vec![]);
            let pos = egui::pos2(screen.screen_size.x - 5.0, height - 5.0);
            screen.frame(vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Modifiers::NONE,
                },
            ]);
            screen.frame(vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            }]);
            let (output, _) = screen.frame(vec![]);
            assert!(!output.shapes.iter().any(|shape| matches!(&shape.shape,
                Shape::Text(text) if text.galley.text() == "Apply from styles and rhythms"
            )));
        }
    }

    #[test]
    fn chord_menus_share_the_sidebar_search_and_style_choices() {
        let mut screen = MenuScreen::new();
        screen.frame(vec![]);
        let (output, _) = screen.frame(vec![]);
        screen.click(&output, "🎷 Band: Pop beat");
        let (output, _) = screen.frame(vec![]);
        screen.click(&output, "🔍 Search: bossa, 6/8, tango…");
        let (output, _) = screen.frame(vec![Event::Text("royal road".to_string())]);
        let (_, actions) = screen.click(&output, "Royal road");
        assert!(actions.iter().any(|action| matches!(action,
            SheetAction::SetBand { style: Some(_), harmony: Some(style) } if style.name == "Royal road"
        )));

        let (output, _) = screen.frame(vec![]);
        screen.click(&output, "Progression");
        let (output, _) = screen.frame(vec![]);
        let (_, actions) = screen.click(&output, "Royal road");
        assert!(actions.iter().any(|action| matches!(action,
            SheetAction::WriteProgression { style, from: 2 } if style.name == "Royal road"
        )));

        screen.sidebar = true;
        screen.frame(vec![]);
        let (output, _) = screen.frame(vec![]);
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape,
            Shape::Text(text) if text.galley.text() == "Royal road"
        )));
        assert!(!output.shapes.iter().any(|shape| matches!(&shape.shape,
            Shape::Text(text) if text.galley.text() == "Four chords"
        )));
    }
}
