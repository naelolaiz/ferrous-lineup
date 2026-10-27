//! The chord sheet: the song's harmony bar by bar, the way a lead sheet
//! writes it, in a window over the playlist or, with "Fill the window",
//! in the playlist's place.
//!
//! A line holds as many bars as fit with every name at its full size, or
//! as many as you choose ([`layout`]), and each section of the playlist
//! (Intro, Verse, Chorus…) starts a new line under its name, so the
//! sheet reads like a printed chart. So do the parts of the band's form
//! ([`form`]): the intro, the chorus, between repeat signs when the band
//! plays it more than once, and the ending. A bar shows its harmonies where
//! their beats fall, each with a caption: the Roman numeral of a chord
//! in the key, or the notes of a set or a micromode. A bar with nothing
//! written goes on with the harmony before it, shown faintly. The row
//! above the bars ([`notation`]) shows the harmony in another notation
//! (as chords, pitch-class sets or micromodes, on the same notes), or the
//! song's own notes grouped into sets, vertically or in a line. While the
//! song plays, the bar playing is framed in white and its harmony is the
//! brightest thing on the sheet.
//!
//! Click a bar and type to write its harmony; Enter takes it in, Tab
//! goes on to the next bar. Holding the mouse down on a harmony plays
//! it. Shift+click (or Shift and an arrow) selects several bars, to loop
//! them from the practice row ([`practice`]). A bar's menu (right-click)
//! inserts an empty bar before or after it, or deletes it (or the bars
//! selected), with everything after moving along (see
//! `lineup_core::bars`). It also chooses the variation of the band's
//! style from that bar on, or for the bars selected; a boxed "a" or "b"
//! marks where it changes.
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
use lineup_core::chord_sheet::function::{Function, Functions, Role};
use lineup_core::chord_sheet::{Push, Stop, read_typed_bar, text, write_bar};
use lineup_core::general_midi;
use lineup_core::library::{Genre, Style};
use lineup_core::{
    Band, BandStyle, Bar, ChordMarker, HarmonyName, KeyMap, MAX_SONG_STEPS, Meter, Note, Pass,
    Player, Project, band_styles,
};

use crate::library_browser::{LibraryAction, LibraryBrowser, LibraryPurpose, Tried};
use crate::theme;
use crate::twelve_tone::Front;

mod band;
mod form;
pub(crate) mod layout;
mod notation;
mod practice;

pub use form::FormAction;
use layout::LayoutView;
use notation::{NotationView, Shown};
pub use practice::{PracticeAction, PracticeView, SPEEDS, Trainer, trainer_controls};

/// The id of the window.
pub(crate) fn window_id() -> egui::Id {
    egui::Id::unique("chord-sheet")
}

const CELL_HEIGHT: f32 = 58.0;
/// The space above a line that starts a section, for its name.
const HEADING_HEIGHT: f32 = 18.0;
const LINE_GAP: f32 = 6.0;
const NAME_SIZE: f32 = 20.0;
const SMALLEST_NAME: f32 = 11.0;
/// The smallest size for the class of a set or a micromode, written
/// where its whole name does not fit.
const SMALLEST_CLASS: f32 = 9.0;
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
/// The numeral under a chord borrowed from the parallel key.
const BORROWED: Color32 = Color32::from_rgb(120, 172, 238);
/// The numeral under a secondary dominant or leading-tone chord.
const SECONDARY: Color32 = Color32::from_rgb(232, 202, 96);
/// The numeral under a Neapolitan chord.
const NEAPOLITAN: Color32 = Color32::from_rgb(230, 142, 176);

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
    /// Choose only the instrument lineup, keeping the playing style.
    BandVoices(&'static BandStyle),
    BandPlayer {
        player: Player,
        on: bool,
    },
    BandProgram {
        channel: usize,
        program: Option<u8>,
    },
    /// Swing the eighth notes this much, from 0 to `MAX_SWING`.
    SetSwing(u8),
    /// Have the band play this part again, where it was edited.
    PlayAgain(Player),
    /// Have a soloist improvise with the band, or not.
    SetSoloist(bool),
    /// Have the soloist play another solo.
    AnotherSolo,
    /// Have the horn section play with the band, or not.
    SetHorns(bool),
    /// Write a progression's chords from this bar on.
    WriteProgression {
        style: &'static Style,
        from: usize,
    },
    /// End the song with this bar.
    EndHere(usize),
    /// Insert or delete bars.
    Bars(BarEdit),
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

/// A change to the song's bars, from the menu on a bar of the sheet or
/// on the playlist's bar numbers. Bars are counted from 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarEdit {
    /// An empty bar next to bar `bar`, after it or before it.
    Insert { bar: usize, after: bool },
    /// Bars `first` to `last` taken out.
    Delete { first: usize, last: usize },
}

/// The menu items that insert a bar next to bar `bar` or delete it, or
/// delete the bars of `selection` when it holds more than one.
pub fn bar_edits(ui: &mut Ui, bar: usize, selection: Option<(usize, usize)>) -> Option<BarEdit> {
    let mut edit = None;
    let moves = "everything after it moves a bar later. The harmony before it goes on \
                 through it until you write one";
    let before = ui.button("Insert a bar before");
    if theme::hint(
        before,
        format!("An empty bar before bar {}: {moves}", bar + 1),
    )
    .clicked()
    {
        edit = Some(BarEdit::Insert { bar, after: false });
    }
    let after = ui.button("Insert a bar after");
    if theme::hint(
        after,
        format!("An empty bar after bar {}: {moves}", bar + 1),
    )
    .clicked()
    {
        edit = Some(BarEdit::Insert { bar, after: true });
    }
    let (first, last) = selection.unwrap_or((bar, bar));
    let (text, hint) = match first == last {
        true => (
            "Delete this bar".to_string(),
            "Takes the bar out with its chords and notes; everything after it moves a bar \
             earlier. Undo puts it back",
        ),
        false => (
            format!("Delete bars {}–{}", first + 1, last + 1),
            "Takes the bars out with their chords and notes; everything after them moves \
             earlier. Undo puts them back",
        ),
    };
    if theme::hint(ui.button(text), hint).clicked() {
        edit = Some(BarEdit::Delete { first, last });
    }
    edit
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

/// One line of the sheet: its bars (as many as fit, or as chosen), and the
/// name of the section it starts, if it starts one.
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

/// One bar being drawn: the song it is in, the bar and where it is.
#[derive(Clone, Copy)]
struct BarView<'a> {
    project: &'a Project,
    meter: &'a Meter,
    keys: &'a KeyMap,
    /// What every chord does in its key, and the key changes.
    functions: &'a Functions,
    bar: Bar,
    rect: Rect,
    marks: Marks,
    /// The band's ending, if the bar is in it.
    ending: Option<Ending>,
}

impl BarView<'_> {
    /// Whether a repeated chorus starts on this bar.
    fn repeat_from(&self) -> bool {
        self.marks
            .repeat
            .is_some_and(|(start, _)| start == self.bar.start)
    }

    /// Whether a repeated chorus ends with this bar.
    fn repeat_to(&self) -> bool {
        self.marks
            .repeat
            .is_some_and(|(_, end)| end == self.bar.end())
    }

    /// The paper and the bar lines: a double one where a section
    /// starts, repeat signs around a chorus played more than once, and
    /// the final bar line where the song ends.
    fn bar_lines(
        &self,
        painter: &egui::Painter,
        hovered: bool,
        starts_section: bool,
        ends_line: bool,
    ) {
        let (bar, rect) = (self.bar, self.rect);
        let fill = if hovered { PAPER_HOVER } else { PAPER };
        painter.rect_filled(rect.shrink2(vec2(0.0, 0.5)), 0.0, fill);
        let line = Stroke::new(1.0, BAR_LINE);
        let thick = Stroke::new(3.0, BAR_LINE);
        painter.vline(rect.left(), rect.y_range(), line);
        if self.repeat_from() {
            painter.vline(rect.left() + 1.5, rect.y_range(), thick);
            painter.vline(rect.left() + 5.0, rect.y_range(), line);
            repeat_dots(painter, rect.left() + 8.5, rect.center().y);
        } else if starts_section && bar.number > 0 {
            painter.vline(rect.left() + 3.0, rect.y_range(), line);
        }
        let song_end = self.project.song_length();
        if self.repeat_to() {
            repeat_dots(painter, rect.right() - 8.5, rect.center().y);
            painter.vline(rect.right() - 5.0, rect.y_range(), line);
            painter.vline(rect.right() - 1.5, rect.y_range(), thick);
        } else if song_end > 0 && bar.start < song_end && bar.end() >= song_end {
            painter.vline(rect.right() - 5.0, rect.y_range(), line);
            painter.vline(rect.right() - 1.5, rect.y_range(), thick);
        } else if ends_line {
            painter.vline(rect.right(), rect.y_range(), line);
        }
    }

    /// The bar number, which moves the playhead there, and what changes
    /// in this bar: the time signature, the band's variation and style,
    /// and the key; or "held" on the ending's last bar.
    fn changes(
        &self,
        ui: &mut Ui,
        painter: &egui::Painter,
        id: egui::Id,
        actions: &mut Vec<SheetAction>,
    ) {
        let (project, bar, rect) = (self.project, self.bar, self.rect);
        let key = self.keys.key_at(bar.start);
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
        let previous = (bar.number > 0).then(|| self.meter.bar(bar.number - 1));
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
            marks_right = letter.right() + 3.0;
        }
        // Where the band changes style: the style's name.
        if project
            .section_styles()
            .iter()
            .any(|marker| marker.start == bar.start)
        {
            painter.text(
                pos2(marks_right + 9.0, number_rect.top()),
                Align2::LEFT_TOP,
                project.style_name_at(bar.start),
                FontId::proportional(SMALL_SIZE),
                theme::ACCENT,
            );
        }
        if bar.number > 0 && self.keys.key_at(bar.start - 1) != key {
            let label = painter.text(
                rect.right_top() + vec2(-PADDING, 2.0),
                Align2::RIGHT_TOP,
                format!("→ {}", key.name()),
                FontId::proportional(SMALL_SIZE),
                theme::ACCENT,
            );
            // Where the key changes, how the music gets there: through a
            // pivot chord, or directly.
            let change = self
                .functions
                .modulations
                .iter()
                .find(|change| (bar.start..bar.end()).contains(&change.start));
            if let Some(change) = change {
                let hint = ui.interact(label.expand(3.0), id.with("key change"), Sense::hover());
                theme::hint(hint, change.describe());
            }
        } else if self.ending.is_some_and(|ending| ending.end == bar.end()) {
            painter.text(
                rect.right_top() + vec2(-PADDING - 6.0, 2.0),
                Align2::RIGHT_TOP,
                "held",
                FontId::proportional(SMALL_SIZE),
                ENDING,
            );
        }
    }

    /// The menu on a bar. `here` are the harmonies written in it, and
    /// `selection` the bars selected, when it is one of several.
    fn menu(
        &self,
        ui: &mut Ui,
        here: &[ChordMarker],
        selection: Option<(usize, usize)>,
        clicks: &mut BarClicks,
        actions: &mut Vec<SheetAction>,
    ) {
        let (project, meter, bar) = (self.project, self.meter, self.bar);
        if ui.button("Type in this bar").clicked() {
            clicks.start_typing = Some((bar.number, write_bar(here, &bar)));
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
        ui.separator();
        if let Some(edit) = bar_edits(ui, bar.number, selection) {
            actions.push(SheetAction::Bars(edit));
        }
        if project.band().is_some() {
            ui.separator();
            variation_menu(ui, project, meter, bar.number, selection, actions);
            style_menu(ui, project, meter, bar.number, selection, actions);
            for marker in here
                .iter()
                .filter(|marker| marker.harmony != HarmonyName::NoChord)
            {
                ui.menu_button(format!("Band on {}", marker.harmony), |ui| {
                    breaks_menu(ui, &bar, here, *marker, actions);
                });
            }
        }
        if project.band().is_some() && self.ending.is_none() {
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
    reveal_step: Option<usize>,
    /// How the harmony is named, and which notes are grouped.
    notation: NotationView,
    /// How many bars a line holds, and whether the sheet fills the
    /// main window.
    layout: LayoutView,
    /// Where the sheet is drawn in the main window, when it fills it.
    main_area: Option<Rect>,
    front: Front,
    /// What every chord does in its key, read from the chords and the
    /// keys it was read from, again only when they change.
    functions: Option<(Vec<ChordMarker>, KeyMap, std::rc::Rc<Functions>)>,
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
            reveal_step: None,
            notation: NotationView::new(),
            layout: LayoutView::new(),
            main_area: None,
            front: Front::default(),
            functions: None,
        }
    }

    /// Whether the sheet has the keyboard, so typed digits are chords
    /// and sets, not harmony keys.
    pub fn has_keyboard(&self) -> bool {
        self.open && self.focused
    }

    /// Reads back the choices of the last run, the first time; then
    /// saves them for the next when they change. Called before anything
    /// asks where the sheet goes, so it opens where it was left.
    pub fn remember(&mut self, ctx: &egui::Context) {
        self.notation.remember(ctx);
        self.layout.remember(ctx);
    }

    /// Whether the sheet is shown in the main window, in the playlist's
    /// place, rather than in a window over it.
    pub fn fills_window(&self) -> bool {
        self.open && self.layout.fills()
    }

    /// Shows the sheet in the main window, in the playlist's place, or
    /// in a window over the playlist.
    pub fn fill_window(&mut self, fills: bool) {
        self.layout.set_fills(fills);
        self.open = true;
    }

    pub fn reveal_step(&mut self, step: usize) {
        self.reveal_step = Some(step);
        self.reveal = false;
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
        self.remember(ctx);
        if !self.open {
            self.release(&mut actions);
            self.editing = None;
            self.focused = false;
            return actions;
        }
        // In the main window, [`ChordSheet::main`] draws it.
        if self.layout.fills() {
            return actions;
        }
        self.main_area = None;
        let mut open = self.open;
        self.front.keep(ctx, window_id());
        let window = egui::Window::new("Chord sheet")
            .id(window_id())
            .open(&mut open)
            .default_pos([260.0, 110.0])
            .min_width(360.0)
            .constrain_to(area)
            .resizable(true);
        let size = vec2(760.0, 420.0);
        crate::widgets::open_at(
            ctx,
            window,
            window_id(),
            size,
            crate::widgets::Opening::FirstTime,
        )
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

    /// Draws the sheet in `ui`, the main window's room for the playlist,
    /// when it fills it (see [`ChordSheet::fills_window`]). The rest is as
    /// in [`ChordSheet::show`].
    pub fn main(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        playing: Option<(usize, Pass)>,
        typing_keys: bool,
        controls: (PracticeView, &mut LibraryBrowser),
    ) -> Vec<SheetAction> {
        let mut actions = Vec::new();
        self.main_area = Some(ui.available_rect_before_wrap());
        ui.heading("Chord sheet");
        self.contents(ui, project, playing, typing_keys, controls, &mut actions);
        actions
    }

    /// Draws only the bars in `ui`, for a video: no window, no buttons.
    /// `playing` is the step of the playlist's timeline that plays; the
    /// sheet scrolls along with it.
    pub fn video(&mut self, ui: &mut Ui, project: &Project, playing: Option<usize>) {
        let mut actions = Vec::new();
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                self.sheet(ui, project, playing, false, &mut actions)
            });
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
            band::menu(ui, project, actions);
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
                    "Fill the sheet with the chords heard in the song, in place of what it \
                     says now: an arpeggio counts as its chord, two chords at once are \
                     written Eb7|Fb, notes that make no chord as their set; in a set or a \
                     row, the sets or micromodes heard. Undo brings the sheet back",
                )
                .clicked()
                {
                    actions.push(SheetAction::WriteHeardChords);
                }
            }
        });
        let layout = &mut self.layout;
        self.notation.row(ui, project, |ui| layout.items(ui));
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
            let name = player.name().to_lowercase();
            if project.plays_locked(player) {
                ui.colored_label(
                    ui.visuals().warn_fg_color,
                    format!("🔒 The {name} is locked: it keeps its notes, not new chords."),
                );
            } else if project.song_keeps(player) {
                let channel = match player {
                    Player::Piano => lineup_core::library::apply::KEYS,
                    _ => lineup_core::library::apply::BASS,
                };
                ui.weak(format!(
                    "🔒 {channel} is locked: the band leaves the {name} to it."
                ));
            } else if project.is_frozen(player) {
                ui.horizontal_wrapped(|ui| {
                    ui.colored_label(
                        ui.visuals().warn_fg_color,
                        format!("✎ You edited the {name}: the band keeps it as you left it."),
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
        let heard = self.notation.heard(project, &keys);
        let marks = Marks::of(project);
        self.read_functions(project, &keys);
        // Lines hold as many bars as chosen, or as fit with every name
        // in full.
        let needed = match self.layout.columns() {
            layout::Columns::Fit => meter
                .bars()
                .take(self.bar_count(project, &meter))
                .map(|bar| {
                    self.needed_width(ui.painter(), project, &meter, &keys, &bar, marks, &heard)
                })
                .fold(0.0, f32::max),
            layout::Columns::Bars(_) => 0.0,
        };
        let per_line = self
            .layout
            .columns()
            .count(ui.available_width() - 2.0, needed);
        self.layout.shown = per_line;
        let lines = self.lines(project, &meter, per_line);
        let width = ui.available_width().max(per_line as f32 * 40.0);
        let cell_width = (width - 2.0) / per_line as f32;
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
        let mut heading_selection = None;
        let mut y = canvas.top();
        for (index, line) in lines.iter().enumerate() {
            if let Some(heading) = &line.heading {
                let hint = match project.band() {
                    Some(_) => {
                        "Click to select this section’s bars; click again or press Escape to clear the selection. Right-click to have the band play it another way"
                    }
                    None => {
                        "Click to select this section’s bars; click again or press Escape to clear the selection"
                    }
                };
                let response = ui
                    .interact(
                        Rect::from_min_size(pos2(canvas.left(), y), vec2(width, HEADING_HEIGHT)),
                        ui.scope_id().with(("section-heading", index)),
                        Sense::click(),
                    )
                    .on_hover_text(hint);
                let opens = response.secondary_clicked() && project.band().is_some();
                if (response.clicked() || opens) && !crate::widgets::dismissing(ui.ctx()) {
                    let first = line.bars[0].number;
                    let next = lines
                        .iter()
                        .skip(index + 1)
                        .find(|line| line.heading.is_some())
                        .and_then(|line| line.bars.first())
                        .map(|bar| bar.number);
                    let explicit_end = project
                        .playlist()
                        .sections()
                        .iter()
                        .find(|section| meter.bar_at(section.start).number == first)
                        .map(|section| meter.bar_at(section.end().saturating_sub(1)).number);
                    let reach = project.song_length().max(
                        project
                            .playlist()
                            .chord_markers()
                            .last()
                            .map_or(0, |marker| marker.start + 1),
                    );
                    let last = explicit_end
                        .unwrap_or_else(|| meter.bar_at(reach.saturating_sub(1)).number)
                        .min(next.map_or(usize::MAX, |next| next.saturating_sub(1)))
                        .max(first);
                    // A right-click selects the section too, so the menu's
                    // choices apply to the bars that show selected.
                    heading_selection = Some((first, last, opens));
                }
                if project.band().is_some() {
                    response.context_menu(|ui| {
                        ui.set_min_width(220.0);
                        if let Some(bars) = self.selected_bars() {
                            ui.label(heading);
                            ui.separator();
                            variation_menu(ui, project, &meter, bars.0, Some(bars), actions);
                            ui.separator();
                            style_menu(ui, project, &meter, bars.0, Some(bars), actions);
                        }
                    });
                }
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
        let reveal_bar = self
            .reveal_step
            .take()
            .map(|step| meter.bar_at(step).number);
        let mut start_typing = None;
        let mut clicked = sheet.clicked() || heading_selection.is_some();
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
                (playing, &heard[..]),
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
            if reveal_bar == Some(cell.bar.number)
                || (reveal_bar.is_none()
                    && crate::widgets::follow_playback(ui.ctx())
                    && playing_bar == Some(cell.bar.number)
                    && self.followed != playing_bar)
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
        if let Some((first, last, opens)) = heading_selection {
            self.take_in(project, &cells, &keys, actions);
            if self.selected_bars() == Some((first, last)) && !opens {
                self.selected = None;
                self.anchor = None;
            } else {
                self.select_to(first, false);
                self.select_to(last, true);
            }
        } else if sheet.clicked()
            && !crate::widgets::dismissing(ui.ctx())
            && sheet
                .interact_pointer_pos()
                .is_some_and(|pos| !cells.iter().any(|cell| cell.rect.contains(pos)))
        {
            self.take_in(project, &cells, &keys, actions);
            self.selected = None;
            self.anchor = None;
        }
        if let Some((bar, text)) = start_typing {
            self.take_in(project, &cells, &keys, actions);
            self.start_editing(bar, text, Fresh::SelectAll);
        }
        // The key that ended the typing (Enter, Escape) is not read
        // again as a key for the sheet.
        // In the main window, the sheet shares its layer with the
        // toolbars and the side panel: only a click on it gives it the
        // keys.
        let has_keyboard = match self.main_area {
            Some(area) => crate::widgets::editor_keyboard_in(ui.ctx(), ui.layer_id(), area),
            None => crate::widgets::editor_keyboard(ui.ctx(), ui.layer_id()),
        };
        if has_keyboard && self.editing.is_none() && !was_editing {
            ui.memory_mut(|memory| memory.set_focus_lock_filter(sheet.id, KEEP_KEYS));
            self.keys(ui, project, &cells, typing_keys, actions);
        }
        self.focused = has_keyboard || self.editing.is_some();
    }

    /// Reads what every chord of the sheet does in its key, if the
    /// chords or the keys changed since it was last read.
    fn read_functions(&mut self, project: &Project, keys: &KeyMap) {
        let markers = project.playlist().chord_markers();
        let fresh = self
            .functions
            .as_ref()
            .is_some_and(|(read, read_keys, _)| read == markers && read_keys == keys);
        if !fresh {
            let functions = Functions::read(markers, keys);
            self.functions = Some((markers.to_vec(), keys.clone(), std::rc::Rc::new(functions)));
        }
    }

    /// What every chord of the sheet does in its key, as last read.
    fn functions(&self) -> std::rc::Rc<Functions> {
        self.functions
            .as_ref()
            .map_or_else(Default::default, |(_, _, functions)| {
                std::rc::Rc::clone(functions)
            })
    }

    /// What bar `bar` shows, in the notation chosen: the harmony going on
    /// from an earlier bar, if the bar does not start with one of its
    /// own, and the bar's own harmonies. `heard` is what the song plays,
    /// when the sheet reads its notes. The bar being typed in shows only
    /// the text field.
    #[allow(clippy::too_many_arguments)]
    fn harmonies(
        &self,
        project: &Project,
        meter: &Meter,
        keys: &KeyMap,
        bar: &Bar,
        marks: Marks,
        heard: &[Shown],
    ) -> (Option<Shown>, Vec<Shown>) {
        let typed_in = self
            .editing
            .as_ref()
            .is_some_and(|editing| editing.bar == bar.number);
        if typed_in {
            return (None, Vec::new());
        }
        if self.notation.reads_notes() {
            let first = heard.partition_point(|shown| shown.start < bar.start);
            let end = heard.partition_point(|shown| shown.start < bar.end());
            let shown = heard[first..end].to_vec();
            let going_on = (first > 0
                && bar.start < project.song_length()
                && shown.first().is_none_or(|first| first.start > bar.start))
            .then(|| heard[first - 1].clone());
            return (going_on, shown);
        }
        let functions = self.functions();
        let written = |marker: &ChordMarker| {
            // Each harmony is read in the key on its own step: a key can
            // change partway through a bar.
            let key = keys.key_at(marker.start);
            let function = |shown: &ChordMarker| marked(&functions, shown);
            self.notation.written(
                marker,
                key,
                |shown| caption(shown, key, function(shown)),
                |shown| describe(shown, key, function(shown)),
                |shown| function(shown).map(tint),
                voicing,
            )
        };
        let here = markers_in(project, bar);
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
        // The ending's harmony is the band's, not written on the sheet.
        let ending = marks
            .ending_in(bar)
            .map(|ending| ChordMarker::new(ending.start, ending.harmony));
        let (going_on, markers) = match ending {
            Some(marker) if marker.start == bar.start => (None, vec![marker]),
            Some(marker) => (Some(marker), Vec::new()),
            None => (going_on, here),
        };
        // A harmony going on from before is not struck again: its marks
        // are left out.
        (
            going_on.map(|marker| written(&ChordMarker::new(marker.start, marker.harmony))),
            markers.iter().map(written).collect(),
        )
    }

    /// How wide bar `bar` must be to show its harmonies and the words
    /// under them in full, at their own size: each name has the part of
    /// the bar from its beat to the next name's.
    #[allow(clippy::too_many_arguments)]
    fn needed_width(
        &self,
        painter: &egui::Painter,
        project: &Project,
        meter: &Meter,
        keys: &KeyMap,
        bar: &Bar,
        marks: Marks,
        heard: &[Shown],
    ) -> f32 {
        let (going_on, shown) = self.harmonies(project, meter, keys, bar, marks, heard);
        let width = |text: &str, size: f32| {
            painter
                .layout_no_wrap(text.to_string(), FontId::proportional(size), theme::TEXT)
                .size()
                .x
        };
        let length = bar.length.max(1) as f32;
        let starts: Vec<usize> = going_on
            .iter()
            .map(|_| bar.start)
            .chain(shown.iter().map(|item| item.start.max(bar.start)))
            .collect();
        let inner = going_on
            .iter()
            .chain(&shown)
            .enumerate()
            .map(|(index, item)| {
                let end = starts.get(index + 1).copied();
                // A name before another keeps a gap from it.
                let gap = if end.is_some() { 4.0 } else { 0.0 };
                let end = end.unwrap_or(bar.end()).max(starts[index] + 1);
                let share = (end - starts[index]) as f32 / length;
                let text = width(&item.text, NAME_SIZE);
                let caption = item
                    .caption
                    .as_deref()
                    .map_or(0.0, |caption| width(caption, CAPTION_SIZE));
                (text.max(caption) + gap) / share
            })
            .fold(0.0, f32::max);
        let repeats = marks.repeat.map_or(0.0, |(start, end)| {
            let from = if start == bar.start { 8.0 } else { 0.0 };
            let to = if end == bar.end() { 8.0 } else { 0.0 };
            from + to
        });
        inner + 2.0 * PADDING + repeats
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
        (playing, heard): (Option<usize>, &[Shown]),
        actions: &mut Vec<SheetAction>,
    ) -> BarClicks {
        let Cell { bar, rect, .. } = cell;
        let functions = self.functions();
        let view = BarView {
            project,
            meter,
            keys,
            functions: &functions,
            bar,
            rect,
            marks,
            ending: marks.ending_in(&bar),
        };
        let id = ui.scope_id().with(("chord-sheet-bar", bar.number));
        let response = ui.interact(rect, id, Sense::click_and_drag());
        view.bar_lines(painter, response.hovered(), starts_section, ends_line);
        view.changes(ui, painter, id, actions);
        let playing_here = playing.is_some_and(|step| (bar.start..bar.end()).contains(&step));
        let names = self.names(view, (playing, heard), playing_here);
        draw_names(painter, &names);
        self.highlight(painter, view, playing_here);
        self.read_mouse(ui, view, response, &names, actions)
    }

    /// The harmonies of a bar, each where its beat falls, in the
    /// notation chosen: what is written, or the groups of notes the song
    /// plays. While the song plays, the one sounding is the brightest
    /// thing on the sheet.
    fn names(
        &self,
        view: BarView,
        (playing, heard): (Option<usize>, &[Shown]),
        playing_here: bool,
    ) -> Vec<Name> {
        let BarView {
            project,
            bar,
            rect,
            marks,
            ending,
            ..
        } = view;
        let reads_notes = self.notation.reads_notes();
        let sounding = playing.and_then(|step| {
            if reads_notes {
                let after = heard.partition_point(|shown| shown.start <= step);
                return after.checked_sub(1).map(|index| heard[index].start);
            }
            match marks.ending {
                Some(ending) if (ending.start..ending.end).contains(&step) => Some(ending.start),
                _ => project.chord_at(step).map(|marker| marker.start),
            }
        });
        let inner_left = rect.left() + PADDING + if view.repeat_from() { 8.0 } else { 0.0 };
        let inner_right = rect.right() - PADDING - if view.repeat_to() { 8.0 } else { 0.0 };
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
        let (going_on, shown) = self.harmonies(project, view.meter, view.keys, &bar, marks, heard);
        let mut names: Vec<Name> = Vec::new();
        if let Some(item) = going_on {
            let right = shown
                .first()
                .map_or(inner_right, |next| x_of(next.start) - 4.0);
            let color = if playing_here && sounding == Some(item.start) {
                PLAYING
            } else {
                GOING_ON
            };
            names.push(Name {
                place: slot(inner_left, right),
                shown: Shown {
                    caption: None,
                    ..item
                },
                color,
                going_on: true,
            });
        }
        for (index, item) in shown.iter().enumerate() {
            let right = shown
                .get(index + 1)
                .map_or(inner_right, |next| x_of(next.start) - 4.0);
            let color = match (playing.is_some(), sounding == Some(item.start)) {
                (true, true) => PLAYING,
                (true, false) => theme::TEXT_DIM,
                _ if ending.is_some() && !reads_notes => ENDING,
                _ if item.group => GROUP_TEXT,
                _ if item.no_chord => theme::TEXT_DIM,
                _ => theme::TEXT,
            };
            names.push(Name {
                place: slot(x_of(item.start), right),
                shown: item.clone(),
                color,
                going_on: false,
            });
        }
        names
    }

    /// Shades a bar in a selection of several, gives the bars that loop
    /// the loop's colour along their top, and frames the bar playing, or
    /// else the bar selected.
    fn highlight(&self, painter: &egui::Painter, view: BarView, playing_here: bool) {
        let BarView { bar, rect, .. } = view;
        if let Some((first, last)) = self.selected_bars()
            && first < last
            && (first..=last).contains(&bar.number)
        {
            painter.rect_filled(rect.shrink(1.0), 0.0, theme::ACCENT.gamma_multiply(0.12));
        }
        if let Some(region) = view.project.playlist().loop_region()
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
    }

    /// The mouse on a bar: hold a harmony down to hear it, click to
    /// select the bar, double-click to type in it, right-click for more.
    fn read_mouse(
        &mut self,
        ui: &mut Ui,
        view: BarView,
        response: egui::Response,
        names: &[Name],
        actions: &mut Vec<SheetAction>,
    ) -> BarClicks {
        let BarView { bar, ending, .. } = view;
        let here = markers_in(view.project, &bar);
        let under = |pos: Option<Pos2>| {
            let pos = pos?;
            names
                .iter()
                .find(|name| name.place.x_range().contains(pos.x))
        };
        let primary = ui.input(|input| input.pointer.primary_down());
        let dismissing = crate::widgets::dismissing(ui.ctx());
        if response.is_pointer_button_down_on()
            && primary
            && self.sounding.is_empty()
            && !dismissing
        {
            let pointer = response.interact_pointer_pos();
            if let Some(name) = under(pointer) {
                self.sounding = name.shown.notes.clone();
                if !self.sounding.is_empty() {
                    actions.push(SheetAction::NoteOn(self.sounding.clone()));
                }
            }
        }
        let mut clicks = BarClicks {
            clicked: (response.clicked() || response.secondary_clicked()) && !dismissing,
            start_typing: None,
        };
        // A right-click inside the bars selected keeps them selected, for
        // the menu to act on.
        let in_selection = self
            .selected_bars()
            .is_some_and(|(first, last)| first < last && (first..=last).contains(&bar.number));
        let keeps_selection = response.secondary_clicked() && in_selection;
        if (clicks.clicked || response.drag_started()) && !keeps_selection && !dismissing {
            let shift = ui.input(|input| input.modifiers.shift);
            self.select_to(bar.number, shift);
            self.error = None;
        }
        if response.double_clicked() && !dismissing {
            clicks.start_typing = Some((bar.number, write_bar(&here, &bar)));
        }
        let hint = match (under(response.hover_pos()), ending) {
            (_, Some(_)) => format!(
                "Bar {}, the band's ending: a bar on the key's tonic, then the chord struck \
                 once and held. Type a chord here, and the chorus takes this bar in",
                bar.number + 1
            ),
            (Some(name), None) => name.shown.about.clone(),
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
            view.menu(ui, &here, selection, &mut clicks, actions);
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
        self.reveal_step = None;
        self.notation.new_song();
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
    pub(crate) fn selected_bars(&self) -> Option<(usize, usize)> {
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

    /// How many bars the sheet has: as many as the song has, or as the
    /// chords written reach, or up to the one selected or typed in.
    fn bar_count(&self, project: &Project, meter: &Meter) -> usize {
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
        count
    }

    /// The lines of bars, `per_line` to a line: as many as the sheet has
    /// (see [`ChordSheet::bar_count`]), and one more empty line to write
    /// on.
    fn lines(&self, project: &Project, meter: &Meter, per_line: usize) -> Vec<Line> {
        let count = self.bar_count(project, meter);
        let count = (count.div_ceil(per_line) + 1).max(2) * per_line;
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
            let full = lines.last().is_none_or(|line| line.bars.len() == per_line);
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
        match read_typed_bar(&editing.text, &bar, keys.key_at(bar.start)) {
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
        // Cut with no bar selected would copy the whole sheet and clear
        // nothing, so it leaves the clipboard alone.
        if cut && self.selected_bars().is_none() {
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
            read_typed_bar(pasted, &bar, project.key_map().key_at(bar.start))
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

/// The horn section: on or off, and what it plays in this style.
fn horns_items(ui: &mut Ui, band: &Band, actions: &mut Vec<SheetAction>) {
    let mut on = band.has_horns();
    let check = ui.checkbox(&mut on, "Horns");
    let hint = format!(
        "A horn section plays the chords in close harmony, each horn moving as little as it \
         can from chord to chord. In {}, {}.",
        band.style.name,
        band.horn_section().describe()
    );
    if theme::hint(check, hint).clicked() {
        actions.push(SheetAction::SetHorns(on));
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
            // Nothing shows as chosen: the band's style says how it plays,
            // not where the chords on the sheet came from.
            match library.show_for(ui, project, Tried::default(), LibraryPurpose::Progression) {
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

/// The styles the band can play part of the song in, in a bar's menu.
/// They apply to the bars `selection` holds, when the bar right-clicked
/// is one of them, or else from bar `number` on. Only the styles in that
/// bar's time signature, or that follow every bar's meter, are offered:
/// another would change the bars.
fn style_menu(
    ui: &mut Ui,
    project: &Project,
    meter: &Meter,
    number: usize,
    selection: Option<(usize, usize)>,
    actions: &mut Vec<SheetAction>,
) {
    let Some(band) = project.band() else {
        return;
    };
    let (start, end) = match selection {
        Some((first, last)) => (meter.bar(first).start, Some(meter.bar(last).end())),
        None => (meter.bar(number).start, None),
    };
    let bars = match selection {
        Some((first, last)) => format!("Bars {}–{}: the band", first + 1, last + 1),
        None => format!("From bar {} on, the band", number + 1),
    };
    let time = meter.bar_at(start).time;
    let playing = project.section_style_at(start);
    let mut choose = |ui: &mut Ui, style: Option<&'static BandStyle>| {
        actions.push(SheetAction::Form(FormAction::SectionStyle {
            start,
            end,
            style,
        }));
        ui.close();
    };
    let menu = ui.menu_button(format!("Style: {}", project.style_name_at(start)), |ui| {
        ui.set_min_width(220.0);
        crate::widgets::scroll_menu(ui, |ui| {
            let own = ui.radio(
                playing.is_none(),
                format!("The song's style: {}", band.name()),
            );
            if theme::hint(own, format!("{bars} plays the band's own style")).clicked() {
                choose(ui, None);
            }
            ui.separator();
            for genre in Genre::ALL {
                let styles: Vec<&'static BandStyle> = band_styles()
                    .iter()
                    .filter(|style| {
                        style.genre == genre && (style.time == time || style.follows_the_song())
                    })
                    .collect();
                if styles.is_empty() {
                    continue;
                }
                let here = playing.is_some_and(|playing| styles.contains(&playing));
                let name = match here {
                    true => format!("{} •", genre.name()),
                    false => genre.name().to_string(),
                };
                crate::widgets::scroll_menu_button(ui, name, |ui| {
                    for style in styles {
                        let item = ui.radio(playing == Some(style), style.name);
                        let about = format!(
                            "{bars} plays {}: {} The drums fill into it",
                            style.name, style.about
                        );
                        if theme::hint(item, about).clicked() {
                            choose(ui, Some(style));
                        }
                    }
                });
            }
        });
    });
    theme::hint(
        menu.response,
        format!(
            "Have the band play these bars in another style, in {time}: its piano, bass, \
             drums and swing, on the band's instruments"
        ),
    );
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

/// A harmony to draw in a bar: where, what, and in which colour.
struct Name {
    place: Rect,
    shown: Shown,
    color: Color32,
    /// It goes on from an earlier bar: drawn faintly, in what room is
    /// left before the bar's own first harmony.
    going_on: bool,
}

/// Draws the harmonies of a bar, each with its caption under it. The
/// bar's own names are all drawn at one size, as big as the most crowded
/// one allows, so a bar does not mix big and small names; a name going
/// on from before only shrinks itself.
///
/// A set or a micromode whose name does not fit even at the smallest
/// size is written by its class alone, a little smaller still: "Menor 3"
/// for "Menor 3 T8I", and the notes under it say which one it is. Where
/// not even that fits, it is "…" with no notes under it, since a name or
/// notes cut short would read as others.
fn draw_names(painter: &egui::Painter, names: &[Name]) {
    let fit = |name: &Name, text: &str| {
        let font = FontId::proportional(NAME_SIZE);
        let width = painter
            .layout_no_wrap(text.to_string(), font, name.color)
            .size()
            .x;
        NAME_SIZE * (name.place.width().max(1.0) / width.max(1.0)).min(1.0)
    };
    // Each name's text, and the smallest size it may take.
    let text = |name: &Name| match &name.shown.short {
        Some(short) if fit(name, &name.shown.text) < SMALLEST_NAME => {
            match fit(name, short) < SMALLEST_CLASS {
                true => ("…".to_string(), SMALLEST_CLASS),
                false => (short.clone(), SMALLEST_CLASS),
            }
        }
        _ => (name.shown.text.clone(), SMALLEST_NAME),
    };
    let texts: Vec<(String, f32)> = names.iter().map(text).collect();
    let own = || names.iter().zip(&texts).filter(|(name, _)| !name.going_on);
    let smallest = own()
        .map(|(_, (_, smallest))| *smallest)
        .fold(SMALLEST_NAME, f32::min);
    let size = own()
        .map(|(name, (text, _))| fit(name, text))
        .fold(NAME_SIZE, f32::min)
        .max(smallest);
    for (name, (text, smallest)) in names.iter().zip(texts) {
        let size = match name.going_on {
            true => fit(name, &text).max(smallest),
            false => size,
        };
        let clipped = painter.with_clip_rect(name.place.intersect(painter.clip_rect()));
        let cut = text == "…";
        clipped.text(
            name.place.left_top(),
            Align2::LEFT_TOP,
            text,
            FontId::proportional(size),
            name.color,
        );
        if !cut && let Some(caption) = &name.shown.caption {
            clipped.text(
                pos2(name.place.left(), name.place.top() + NAME_SIZE + 4.0),
                Align2::LEFT_TOP,
                caption,
                FontId::proportional(CAPTION_SIZE),
                name.shown.tint.unwrap_or(theme::TEXT_DIM),
            );
        }
    }
}

/// What the sheet says a chord does in its key, if it marks it: the
/// function read for the chord written on `shown`'s step, when `shown`
/// is that chord (not the same notes named as a set) and is borrowed,
/// secondary, Neapolitan or a pivot.
fn marked<'a>(functions: &'a Functions, shown: &ChordMarker) -> Option<&'a Function> {
    let HarmonyName::Chord(symbol) = shown.harmony else {
        return None;
    };
    functions
        .at(shown.start)
        .filter(|function| function.symbol == symbol && function.is_marked())
}

/// The colour of the numeral under a chord the sheet marks: the pivot
/// of a key change in the colour of the key change, a borrowed chord in
/// blue, a secondary chord in yellow, the Neapolitan in pink, and a chord
/// in a key that came before its marker in the colour of the key change.
fn tint(function: &Function) -> Color32 {
    if function.pivot.is_some() {
        return theme::ACCENT;
    }
    match function.role {
        Role::Borrowed { .. } => BORROWED,
        Role::SecondaryDominant { .. } | Role::SecondaryLeadingTone { .. } => SECONDARY,
        Role::Neapolitan => NEAPOLITAN,
        Role::Plain if function.early => theme::ACCENT,
        Role::Plain => theme::TEXT_DIM,
    }
}

/// The words under a harmony on the sheet: its numeral or its notes (see
/// [`HarmonyName::caption`]), what it does in the key when `function`
/// says (see [`Function::numeral`]), and what the band does there if it
/// leaves its style: "V7/V · push · shot", "bVI · borrowed".
fn caption(
    marker: &ChordMarker,
    key: lineup_core::Scale,
    function: Option<&Function>,
) -> Option<String> {
    let numeral = match function {
        Some(function) => function.numeral().map(|numeral| match function.tag() {
            Some(tag) => format!("{numeral} · {tag}"),
            None => numeral,
        }),
        None => marker.harmony.caption(key),
    };
    let words: Vec<String> = numeral
        .into_iter()
        .chain(marker.push.map(|_| "push".to_string()))
        .chain(marker.stop.map(|stop| stop.name().to_string()))
        .collect();
    (!words.is_empty()).then(|| words.join(" · "))
}

/// A sentence about a harmony and what the band does there, for its
/// hint. A chord from outside the key, or the pivot of a key change, is
/// described as `function` reads it.
fn describe(marker: &ChordMarker, key: lineup_core::Scale, function: Option<&Function>) -> String {
    let mut text = match function {
        Some(function) => function.describe(),
        None => marker.harmony.describe(key),
    };
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
/// C3 and its notes from the root, from C4 up; two chords at once, the
/// lower so, the upper stacked right above it; a set or a micromode in
/// normal form, from C4 up. A row form and no chord have none.
fn voicing(harmony: HarmonyName) -> Vec<Note> {
    match harmony {
        HarmonyName::Polychord(upper, lower) => {
            let mut notes = voicing(HarmonyName::Chord(lower));
            let top = notes.last().map_or(72, |note| note.midi() + 1);
            let upper = upper.chord();
            let root = top + Note::new(top).pitch_class().semitones_up_to(upper.root);
            notes.extend(
                upper
                    .quality
                    .intervals()
                    .iter()
                    .map(|&interval| Note::new(root + interval)),
            );
            notes
        }
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
    fn section_headings_toggle_all_their_bars_and_shift_ranges_clear_with_escape() {
        fn draw(
            ctx: &egui::Context,
            sheet: &mut ChordSheet,
            library: &mut LibraryBrowser,
            project: &Project,
            events: Vec<Event>,
        ) -> FullOutput {
            let mut output = ctx.run_pass(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 1000.0))),
                    time: Some(ctx.cumulative_frame_nr() as f64 * 0.2),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    sheet.show(
                        ctx,
                        ctx.content_rect(),
                        project,
                        None,
                        false,
                        (
                            PracticeView {
                                count_in: false,
                                metronome: false,
                                speed: 100,
                                training: false,
                                trainer: Trainer::default(),
                            },
                            library,
                        ),
                    );
                },
            );
            output.textures_delta.clear();
            output
        }
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut project = Project::empty();
        project.pattern_mut(0).unwrap().set_length(128);
        project.add_clip(lineup_core::Clip::new(0, 0, 0));
        project.add_section(lineup_core::Section::new("Verse", 0, 96));
        project.add_section(lineup_core::Section::new("Chorus", 96, 32));
        let mut sheet = ChordSheet::new();
        sheet.open = true;
        let mut library = LibraryBrowser::new();
        draw(&ctx, &mut sheet, &mut library, &project, vec![]);
        for expected in [Some((0, 5)), None] {
            let output = draw(&ctx, &mut sheet, &mut library, &project, vec![]);
            let pos = output
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    Shape::Text(text) if text.galley.text() == "Verse" => {
                        Some(text.pos + text.galley.size() * 0.5)
                    }
                    _ => None,
                })
                .unwrap();
            for pressed in [true, false] {
                draw(
                    &ctx,
                    &mut sheet,
                    &mut library,
                    &project,
                    vec![
                        Event::PointerMoved(pos),
                        Event::PointerButton {
                            pos,
                            button: PointerButton::Primary,
                            pressed,
                            modifiers: Modifiers::NONE,
                        },
                    ],
                );
            }
            assert_eq!(sheet.selected_bars(), expected);
        }
        sheet.select(1);
        draw(
            &ctx,
            &mut sheet,
            &mut library,
            &project,
            vec![
                Event::ModifiersChanged(Modifiers::SHIFT),
                Event::Key {
                    key: Key::ArrowRight,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::SHIFT,
                },
            ],
        );
        assert_eq!(sheet.selected_bars(), Some((1, 2)));
        draw(
            &ctx,
            &mut sheet,
            &mut library,
            &project,
            vec![
                Event::ModifiersChanged(Modifiers::NONE),
                Event::Key {
                    key: Key::Escape,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
        assert_eq!(sheet.selected_bars(), None);
    }

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
                                training: false,
                                trainer: Trainer::default(),
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

    #[test]
    fn cut_with_no_bar_selected_leaves_the_clipboard_alone() {
        let ctx = egui::Context::default();
        let project = Project::demo();
        let mut sheet = ChordSheet::new();
        sheet.open = true;
        let mut library = LibraryBrowser::new();
        let mut copied = |events: Vec<Event>| {
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
                                training: false,
                                trainer: Trainer::default(),
                            },
                            &mut library,
                        ),
                    );
                },
            );
            output.textures_delta.clear();
            output
                .platform_output
                .commands
                .iter()
                .any(|command| matches!(command, egui::OutputCommand::CopyText(_)))
        };
        copied(vec![]);
        copied(vec![]);
        assert!(!copied(vec![Event::Cut]));
        // Copy still copies the whole sheet.
        assert!(copied(vec![Event::Copy]));
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
        picked: Option<LibraryAction>,
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
                picked: None,
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
                                self.picked = self.library.show(
                                    ui,
                                    &self.project,
                                    Tried::from_band(self.project.band().unwrap()),
                                );
                            } else {
                                ui.horizontal(|ui| {
                                    band::menu(ui, &self.project, &mut actions);
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
            self.click_with_modifiers(output, text, Modifiers::NONE)
        }

        fn click_with_modifiers(
            &mut self,
            output: &FullOutput,
            text: &str,
            modifiers: Modifiers,
        ) -> (FullOutput, Vec<SheetAction>) {
            // The last drawn is on top: the popup's option, not the
            // Library's line about the same part.
            let pos = output
                .shapes
                .iter()
                .rev()
                .find_map(|shape| match &shape.shape {
                    Shape::Text(shape) if shape.galley.text() == text => {
                        Some(shape.pos + shape.galley.size() * 0.5)
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("Missing menu item: {text}"));
            self.frame(vec![
                Event::ModifiersChanged(modifiers),
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers,
                },
            ]);
            self.frame(vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers,
            }])
        }
    }

    #[test]
    fn preset_popup_edits_are_applied_together_and_remembered() {
        for height in [1200.0, 720.0, 480.0] {
            let mut screen = MenuScreen::new();
            screen.screen_size.y = height;
            screen.sidebar = true;
            screen.frame(vec![]);
            let (output, _) = screen.frame(vec![]);
            screen.click(&output, "🔍 Search: bossa, 6/8, tango…");
            screen.frame(vec![Event::Text("royal road".into())]);
            let (output, _) = screen.frame(vec![]);
            let (_, actions) = screen.click(&output, "Royal road");
            assert!(actions.is_empty(), "a normal click must only open options");
            let (mut output, _) = screen.frame(vec![]);
            let before = screen.library.parts;
            // Tempo gets ticked: plain accents bring no chord progression,
            // so without it there would be nothing to apply.
            for label in [
                "Playing patterns",
                "Voices / instruments",
                "Time signature",
                "Chord progression",
                "Tempo",
                "Swing",
                "Plain accents",
            ] {
                let (_, actions) = screen.click(&output, label);
                assert!(actions.is_empty());
                assert_eq!(
                    screen.library.parts, before,
                    "options are drafts until Apply"
                );
                (output, _) = screen.frame(vec![]);
                assert!(
                    has_text(&output, "Apply"),
                    "the popup closed after changing {label}"
                );
            }
            let (_, actions) = screen.click(&output, "Apply");
            assert!(actions.is_empty());
            assert!(
                matches!(screen.picked, Some(LibraryAction::Style(style)) if style.name == "Royal road")
            );
            let parts = screen.library.parts;
            assert!(!parts.rhythm && !parts.instruments && !parts.time_signature);
            assert!(parts.progression && parts.tempo);
            assert!(!parts.swing && !parts.matching_style);
            let (output, _) = screen.frame(vec![]);
            assert!(!has_text(&output, "Apply"));

            // Reopening starts with the confirmed options. Cancel discards changes.
            screen.click(&output, "Royal road");
            let (output, _) = screen.frame(vec![]);
            screen.click(&output, "Tempo");
            let (output, _) = screen.frame(vec![]);
            screen.click(&output, "Cancel");
            assert_eq!(screen.library.parts, parts);
            let (output, _) = screen.frame(vec![]);
            // A second click applies immediately, with no dialog.
            let (_, actions) = screen.click_with_modifiers(&output, "Royal road", Modifiers::CTRL);
            assert!(actions.is_empty());
            assert!(
                matches!(screen.picked, Some(LibraryAction::Style(style)) if style.name == "Royal road")
            );
            assert_eq!(screen.library.parts, parts);
            let (output, _) = screen.frame(vec![]);
            assert!(!has_text(&output, "Apply"));
        }
    }

    fn has_text(output: &FullOutput, text: &str) -> bool {
        output.shapes.iter().any(|shape| {
            matches!(&shape.shape,
                Shape::Text(shape) if shape.galley.text() == text
            )
        })
    }

    #[test]
    fn chords_from_outside_the_key_are_named_by_what_they_do() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut project = Project::empty();
        project.pattern_mut(0).unwrap().set_length(128);
        project.add_clip(lineup_core::Clip::new(0, 0, 0));
        let markers: Vec<ChordMarker> = ["C", "D7", "G7", "C", "Ab", "Am", "D7", "G"]
            .iter()
            .enumerate()
            .map(|(bar, name)| ChordMarker::new(bar * 16, HarmonyName::parse(name).unwrap()))
            .collect();
        assert!(project.set_chords(0, 128, &markers));
        // G major from bar 8, the G: Am in bar 6 is the pivot.
        let g = lineup_core::Scale::new(lineup_core::PitchClass::G, lineup_core::ScaleKind::Major);
        project.set_key_marker(lineup_core::KeyMarker::new(7 * 16, g));
        let mut sheet = ChordSheet::new();
        sheet.open = true;
        let mut library = LibraryBrowser::new();
        let mut output: Option<FullOutput> = None;
        for _ in 0..2 {
            let mut pass = ctx.run_pass(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 1000.0))),
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
                                training: false,
                                trainer: Trainer::default(),
                            },
                            &mut library,
                        ),
                    );
                },
            );
            pass.textures_delta.clear();
            output = Some(pass);
        }
        let output = output.unwrap();
        let colour = |text: &str| {
            output.shapes.iter().find_map(|shape| match &shape.shape {
                Shape::Text(shape) if shape.galley.text() == text => Some(shape.fallback_color),
                _ => None,
            })
        };
        assert_eq!(colour("V7/V"), Some(SECONDARY));
        assert_eq!(colour("bVI · borrowed"), Some(BORROWED));
        assert_eq!(colour("vi = ii · pivot"), Some(theme::ACCENT));
        // The D7 after the pivot is G major's, before the key change.
        assert_eq!(colour("V7 · in G major"), Some(theme::ACCENT));
        assert_eq!(colour("V7"), Some(theme::TEXT_DIM));
    }

    #[test]
    fn sidebar_click_confirms_options_and_ctrl_click_uses_the_last_selection() {
        use lineup_core::library::{Rhythm, Style, UserStyle};
        let choices = [
            (
                "Royal road",
                LibraryAction::Style(Style::named("Royal road").unwrap()),
            ),
            (
                "Pop beat",
                LibraryAction::Rhythm(Rhythm::named("Pop beat").unwrap()),
            ),
            ("User groove", LibraryAction::UserStyle(0)),
        ];
        for grouped in [false, true] {
            for (name, expected) in choices {
                let mut screen = MenuScreen::new();
                screen.sidebar = true;
                let mut user = UserStyle::blank();
                user.name = "User groove".into();
                screen.library.user_styles.push(user);
                crate::library_browser::set_grouped_presets(&screen.ctx, grouped);
                screen.frame(vec![]);
                let (output, _) = screen.frame(vec![]);
                screen.click(&output, "🔍 Search: bossa, 6/8, tango…");
                screen.frame(vec![Event::Text(name.to_lowercase())]);
                let (output, _) = screen.frame(vec![]);
                screen.click(&output, name);
                assert_eq!(screen.picked, None);
                let (output, _) = screen.frame(vec![]);
                assert!(
                    has_text(&output, "Playing patterns"),
                    "missing copy popup for {name}, grouped={grouped}"
                );
                screen.click(&output, "Tempo");
                let (output, _) = screen.frame(vec![]);
                screen.click(&output, "Apply");
                assert_eq!(screen.picked, Some(expected));
                assert!(screen.library.parts.tempo);
                let (output, _) = screen.frame(vec![]);
                screen.click_with_modifiers(&output, name, Modifiers::CTRL);
                assert_eq!(screen.picked, Some(expected));
                assert!(screen.library.parts.tempo);
                let (output, _) = screen.frame(vec![]);
                assert!(!has_text(&output, "Apply"));
            }
        }
    }

    #[test]
    fn the_sidebar_groups_all_presets_by_genre_with_optional_chords() {
        for grouped in [false, true] {
            let mut screen = MenuScreen::new();
            screen.sidebar = true;
            crate::library_browser::set_grouped_presets(&screen.ctx, grouped);
            screen.frame(vec![]);
            let (output, _) = screen.frame(vec![]);
            screen.click(&output, "🔍 Search: bossa, 6/8, tango…");
            screen.frame(vec![Event::Text(
                if grouped { "bossa" } else { "pop" }.into(),
            )]);
            let (output, _) = screen.frame(vec![]);
            let has = |label: &str| {
                output.shapes.iter().any(|shape| {
                    matches!(&shape.shape,
                        Shape::Text(text) if text.galley.text() == label
                    )
                })
            };
            assert!(
                has(if grouped { "Bossa nova" } else { "Four chords" }),
                "the progression preset must remain available"
            );
            assert!(
                has(if grouped { "Bossa nova" } else { "Pop beat" }),
                "the pattern preset must be in the same list"
            );
            assert!(has("+ chords"));
            if !grouped {
                assert!(has("keeps chords"));
            }
            assert!(
                !has("Copy from preset"),
                "copy options belong to the preset popup"
            );
            for prefix in ["Harmony styles (", "Rhythms ("] {
                let has_group = output.shapes.iter().any(|shape| {
                    matches!(&shape.shape,
                        Shape::Text(text) if text.galley.text().starts_with(prefix)
                    )
                });
                assert!(!has_group, "presets share one set of genre groups");
            }
        }
    }

    #[test]
    fn band_voices_are_separate_from_progression_and_library_choices() {
        let mut screen = MenuScreen::new();
        screen.frame(vec![]);
        let (output, _) = screen.frame(vec![]);
        screen.click(&output, "🎷 Band");
        let (output, _) = screen.frame(vec![]);
        assert!(has_text(&output, "Voices and players"));
        assert!(!has_text(&output, "Copy from preset"));
        assert!(!has_text(&output, "Playing patterns"));
        screen.click(&output, "Instrument lineup");
        let (output, _) = screen.frame(vec![]);
        let (_, actions) = screen.click(&output, "Electric piano + synth bass");
        assert!(actions.iter().any(|action| matches!(action,
            SheetAction::BandVoices(style) if style.name == "Boom bap"
        )));
        screen.frame(vec![Event::Key {
            key: Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }]);
        let (output, _) = screen.frame(vec![]);
        screen.click(&output, "Progression");
        let (output, _) = screen.frame(vec![]);
        screen.click(&output, "🔍 Search: bossa, 6/8, tango…");
        screen.frame(vec![Event::Text("royal road".to_string())]);
        let (output, _) = screen.frame(vec![]);
        let (_, actions) = screen.click(&output, "Royal road");
        assert!(actions.iter().any(|action| matches!(action,
            SheetAction::WriteProgression { style, from: 2 } if style.name == "Royal road"
        )));
    }
}
