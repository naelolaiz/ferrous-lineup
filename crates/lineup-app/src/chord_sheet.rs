//! The chord sheet window: the song's harmony bar by bar, the way a lead
//! sheet writes it.
//!
//! Bars are drawn four to a line, and each section of the playlist
//! (Intro, Verse, Chorus…) starts a new line under its name, so the
//! sheet reads like a printed chart. A bar shows its harmonies where
//! their beats fall, each with a caption: the Roman numeral of a chord
//! in the key, or the notes of a set or a micromode. A bar with nothing
//! written goes on with the harmony before it, shown faintly. While the
//! song plays, the bar playing is framed in white and its harmony is the
//! brightest thing on the sheet.
//!
//! Click a bar and type to write its harmony; Enter takes it in, Tab
//! goes on to the next bar. Holding the mouse down on a harmony plays
//! it. The reading and writing of what is typed is in
//! `lineup_core::chord_sheet`; this module draws and reads the keyboard
//! and the mouse.

use eframe::egui::{
    self, Align2, Color32, FontId, Key, Modifiers, Pos2, Rect, Sense, Stroke, Ui, pos2, vec2,
};
use lineup_core::chord_sheet::{read_bar, write_bar};
use lineup_core::library::{Genre, STYLES, Style};
use lineup_core::{
    BandStyle, Bar, ChordMarker, HarmonyName, KeyMap, MAX_SONG_STEPS, Meter, Note, Player, Project,
    band_styles,
};

use crate::theme;
use crate::twelve_tone::Front;

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

/// Something done on the chord sheet.
pub enum SheetAction {
    /// Write the harmonies of the steps from `start` to `end`: one bar.
    SetChords {
        start: usize,
        end: usize,
        markers: Vec<ChordMarker>,
    },
    /// Write the chords the song plays on the empty sheet.
    WriteHeardChords,
    /// Have the band play in this style, or stop it.
    SetBand(Option<&'static BandStyle>),
    /// Have the band play this part again, where it was edited.
    PlayAgain(Player),
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

/// The chord sheet window.
pub struct ChordSheet {
    pub open: bool,
    /// The bar the keyboard works on, by number.
    selected: Option<usize>,
    editing: Option<Editing>,
    /// Why the last text typed could not be read, and in which bar.
    error: Option<(usize, String)>,
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
            editing: None,
            error: None,
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
    /// playlist's timeline being played, while the song plays;
    /// `typing_keys` says whether letters play notes (the typing
    /// keyboard), so they cannot start typing in a bar.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        area: Rect,
        project: &Project,
        playing: Option<usize>,
        typing_keys: bool,
    ) -> Vec<SheetAction> {
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
            .default_size([640.0, 420.0])
            .min_width(360.0)
            .constrain_to(area)
            .resizable(true)
            .show(ctx, |ui| {
                self.contents(ui, project, playing, typing_keys, &mut actions);
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
        playing: Option<usize>,
        typing_keys: bool,
        actions: &mut Vec<SheetAction>,
    ) {
        let markers = project.playlist().chord_markers();
        ui.horizontal_wrapped(|ui| {
            band_menu(ui, project, actions);
            progression_menu(ui, project, self.next_bar(project), actions);
            ui.label(egui::RichText::new(format!("Key: {}", project.scale.name())).strong());
            ui.weak(format!("· {}", project.time_signature));
            if project.band().is_some() {
                ui.weak(format!("· {} BPM", project.tempo.bpm().round()));
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
        ui.weak(
            "Click a bar and type its harmony: chords (Am7 D7, G/B), Roman numerals (ii7 V7), \
             sets (3-11 T2, {0,1,4}), micromodes (Menor 3), row forms (P0, RI7) or N.C.",
        );
        ui.weak("Each / holds the harmony before it one more beat, as in C / / G7.");
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
        match &self.error {
            Some((bar, error)) => {
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    format!("Bar {}: {error}", bar + 1),
                );
            }
            None => {
                ui.weak(" ");
            }
        }
        ui.separator();
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
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
            if playing_bar == Some(cell.bar.number) && self.followed != playing_bar {
                ui.scroll_to_rect(cell.rect, None);
            }
        }
        self.followed = playing_bar;
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
        let has_keyboard = sheet.has_focus();
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
        playing: Option<usize>,
        actions: &mut Vec<SheetAction>,
    ) -> BarClicks {
        let Cell { bar, rect, .. } = cell;
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
        // starts, and the final bar line where the song ends.
        let fill = if response.hovered() {
            PAPER_HOVER
        } else {
            PAPER
        };
        painter.rect_filled(rect.shrink2(vec2(0.0, 0.5)), 0.0, fill);
        let line = Stroke::new(1.0, BAR_LINE);
        painter.vline(rect.left(), rect.y_range(), line);
        if starts_section && bar.number > 0 {
            painter.vline(rect.left() + 3.0, rect.y_range(), line);
        }
        let song_end = project.song_length();
        if song_end > 0 && bar.start < song_end && bar.end() >= song_end {
            painter.vline(rect.right() - 5.0, rect.y_range(), line);
            painter.vline(
                rect.right() - 1.5,
                rect.y_range(),
                Stroke::new(3.0, BAR_LINE),
            );
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
        if previous.is_none_or(|previous| previous.time != bar.time) {
            painter.text(
                number_rect.right_top() + vec2(6.0, 0.0),
                Align2::LEFT_TOP,
                bar.time.to_string(),
                FontId::proportional(SMALL_SIZE),
                theme::ACCENT,
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
        }

        // The harmonies, each where its beat falls. While the song
        // plays, the one sounding is the brightest thing on the sheet.
        let sounding = playing.and_then(|step| project.chord_at(step));
        let playing_here = playing.is_some_and(|step| (bar.start..bar.end()).contains(&step));
        let inner_left = rect.left() + PADDING;
        let inner_right = rect.right() - PADDING;
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
        let (going_on, shown) = match typed_in {
            true => (None, Vec::new()),
            false => (going_on, here.clone()),
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
                harmony: marker.harmony,
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
                _ if is_group(marker.harmony) => GROUP_TEXT,
                _ if marker.harmony == HarmonyName::NoChord => theme::TEXT_DIM,
                _ => theme::TEXT,
            };
            names.push(Name {
                place: slot(x_of(marker.start), right),
                harmony: marker.harmony,
                color,
                caption: marker.harmony.caption(key),
                going_on: false,
            });
        }
        draw_names(painter, &names);
        let slots: Vec<(Rect, HarmonyName)> = names
            .iter()
            .map(|name| (name.place, name.harmony))
            .collect();

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
                .map(|(_, harmony)| *harmony)
        };
        let primary = ui.input(|input| input.pointer.primary_down());
        if response.is_pointer_button_down_on() && primary && self.sounding.is_empty() {
            let pointer = response.interact_pointer_pos();
            if let Some(harmony) = under(pointer) {
                self.sounding = voicing(harmony);
                if !self.sounding.is_empty() {
                    actions.push(SheetAction::NoteOn(self.sounding.clone()));
                }
            }
        }
        let mut clicks = BarClicks {
            clicked: response.clicked() || response.secondary_clicked(),
            start_typing: None,
        };
        if clicks.clicked || response.drag_started() {
            self.selected = Some(bar.number);
            self.error = None;
        }
        if response.double_clicked() {
            clicks.start_typing = Some((bar.number, write_bar(&here, &bar)));
        }
        let hint = match under(response.hover_pos()) {
            Some(harmony) => harmony.describe(key),
            None => format!(
                "Bar {}: click it and type its harmony, or double-click to change it. \
                 Right-click for more",
                bar.number + 1
            ),
        };
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

    /// Selects `bar`, as a click on it does, so what comes next is
    /// written there.
    pub fn select(&mut self, bar: usize) {
        self.selected = Some(bar);
        self.editing = None;
        self.reveal = true;
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
        let mut lines: Vec<Line> = Vec::new();
        let bars = meter
            .bars()
            .take(count)
            .take_while(|bar| bar.start < MAX_SONG_STEPS);
        for bar in bars {
            let section = sections
                .iter()
                .find(|section| meter.bar_at(section.start).number == bar.number);
            let heading = section.map(|section| match section.repeats {
                1 => section.name.clone(),
                repeats => format!("{} ×{repeats}", section.name),
            });
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
        if pressed(Key::ArrowRight) || pressed(Key::Tab) {
            moved_to = Some((selected + 1).min(last));
        }
        if pressed(Key::ArrowLeft) {
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
            self.selected = Some(bar);
            self.reveal = true;
            self.error = None;
            return;
        }
        if pressed(Key::Escape) {
            self.selected = None;
            return;
        }
        if pressed(Key::Delete) || pressed(Key::Backspace) {
            if let Some(cell) = cell_of(selected) {
                actions.push(SheetAction::SetChords {
                    start: cell.bar.start,
                    end: cell.bar.end(),
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

/// The keys the sheet keeps while it has the keyboard: the arrows move
/// from bar to bar and Tab goes to the next one.
const KEEP_KEYS: egui::EventFilter = egui::EventFilter {
    tab: true,
    horizontal_arrows: true,
    vertical_arrows: true,
    escape: false,
};

/// The band's style, to pick from by genre, or no band.
fn band_menu(ui: &mut Ui, project: &Project, actions: &mut Vec<SheetAction>) {
    let current = project.band().map(|band| band.style);
    let title = match current {
        Some(style) => format!("🎷 Band: {}", style.name),
        None => "🎷 Band: none".to_string(),
    };
    let menu = ui.menu_button(title, |ui| {
        if ui.selectable_label(current.is_none(), "No band").clicked() {
            actions.push(SheetAction::SetBand(None));
            ui.close();
        }
        ui.separator();
        for genre in Genre::ALL {
            let styles: Vec<&'static BandStyle> = band_styles()
                .iter()
                .filter(|style| style.genre == genre)
                .collect();
            if styles.is_empty() {
                continue;
            }
            ui.menu_button(genre.name(), |ui| {
                for style in styles {
                    let label = match style.time.is_common() {
                        true => style.name.to_string(),
                        false => format!("{}  ({})", style.name, style.time),
                    };
                    let item = ui.selectable_label(current == Some(style), label);
                    let about = format!(
                        "{} {}, about {} BPM.",
                        style.about,
                        style.time,
                        style.tempo.round()
                    );
                    if theme::hint(item, about).clicked() {
                        actions.push(SheetAction::SetBand(Some(style)));
                        ui.close();
                    }
                }
            });
        }
    });
    theme::hint(
        menu.response,
        "The band plays the sheet: piano, bass and drums, in the style you pick. It plays \
         again whenever the sheet changes",
    );
}

/// The library's progressions, to write on the sheet from bar `from`.
fn progression_menu(ui: &mut Ui, project: &Project, from: usize, actions: &mut Vec<SheetAction>) {
    let tonal = project.scale.kind.is_tonal();
    ui.add_enabled_ui(tonal, |ui| {
        let menu = ui.menu_button("Progression", |ui| {
            for genre in Genre::ALL {
                let styles: Vec<&'static Style> =
                    STYLES.iter().filter(|style| style.genre == genre).collect();
                if styles.is_empty() {
                    continue;
                }
                ui.menu_button(genre.name(), |ui| {
                    for style in styles {
                        let chords = style.chord_names(project.scale.root);
                        let item = ui.button(style.name);
                        if theme::hint(item, format!("{chords}. {}", style.about)).clicked() {
                            actions.push(SheetAction::WriteProgression { style, from });
                            ui.close();
                        }
                    }
                });
            }
        });
        let hint = format!(
            "Write the chords of a well-known progression from bar {} on",
            from + 1
        );
        theme::hint(menu.response, hint).on_disabled_hover_text(
            "Progressions are chords in a key; this song is in a set or a row",
        );
    });
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
    harmony: HarmonyName,
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
        let text = name.harmony.to_string();
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
            name.harmony.to_string(),
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
}
