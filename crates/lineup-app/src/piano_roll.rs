//! The piano roll: a window for drawing notes, with harmony helpers.
//!
//! Like the channel rack, [`PianoRoll::show`] never changes the project:
//! it returns [`RollAction`]s for the app to apply. Unlike the rack, the
//! piano roll has state of its own that must last from one frame to the
//! next (which channel it edits, what a click draws, the selected notes,
//! a drag in progress), so it is a struct with methods instead of a
//! single function.

use std::rc::Rc;

use eframe::egui::{
    self, Align, Align2, Color32, FontId, Key, KeyboardShortcut, Modifiers, Painter, Pos2, Rect,
    Response, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, pos2, vec2,
};
use lineup_core::band::solo::{NoteRole, Over};
use lineup_core::harmony::keys::{self, ChordSize};
use lineup_core::harmony::voice_leading::{self, Rules, Slip, VoiceLeading};
use lineup_core::harmony::voice_motion::{self, Change};
use lineup_core::harmony::{Harmony, HintLevel, analyze, next_chords_over};
use lineup_core::tools::{self, ArpeggioOrder};
use lineup_core::{
    Chord, ChordQuality, ChordStamp, Note, NoteEvent, Pattern, PitchClass, PitchSet, Project,
    Scale, ScaleKind, Spelling, TICKS_PER_STEP, TimeSignature, tidy_notes,
};

use crate::micromodes::{MicroAction, MicromodeWindow};
use crate::theme;
use crate::twelve_tone::{self, RowAction, RowWindow, SetAction, SetWindow};

/// Something the user asked the piano roll to do.
#[derive(Debug, Clone, PartialEq)]
pub enum RollAction {
    SetLength(usize),
    /// Add one note, or the notes of a chord.
    AddNotes {
        channel: usize,
        notes: Vec<NoteEvent>,
    },
    RemoveNote {
        channel: usize,
        index: usize,
    },
    /// Replace a note that was moved or resized.
    ReplaceNote {
        channel: usize,
        index: usize,
        note: NoteEvent,
    },
    /// Replace some notes with others, all at once: a moved selection,
    /// a tool, new velocities. `label` names the undo step.
    ReplaceNotes {
        channel: usize,
        old: Vec<NoteEvent>,
        new: Vec<NoteEvent>,
        label: &'static str,
    },
    /// Play some notes right now, without adding them.
    Preview {
        channel: usize,
        pitches: Vec<Note>,
    },
    SetScale(Scale),
    /// Show another pattern: one where the channel has notes.
    SelectPattern(usize),
    /// Create and select a separate pattern for generated notes.
    NewPattern {
        channel: usize,
        name: String,
        length: usize,
        notes: Vec<NoteEvent>,
    },
    /// Open the channel rack, where drum steps are edited.
    OpenRack,
}

/// The range of notes shown, from C1 to B7.
const LOWEST: u8 = 24;
const HIGHEST: u8 = 107;

const ROW_HEIGHT: f32 = 12.0;
const KEY_WIDTH: f32 = 54.0;
/// The narrowest a step is at 100%, so a long pattern can still be read;
/// it then scrolls sideways.
const MIN_STEP_WIDTH: f32 = 8.0;
/// Two rows of chords: the track's and all the tracks'.
const LANE_HEIGHT: f32 = 32.0;
/// Three rows when the voice leading is checked: what the check finds
/// goes under the chords.
const LANE_HEIGHT_WITH_VOICES: f32 = 48.0;
/// Four in a twelve-tone row: the forms of the row the check reads come
/// before what it finds.
const LANE_HEIGHT_WITH_ROWS: f32 = 64.0;
const VELOCITY_HEIGHT: f32 = 56.0;
/// How close to a note's right end (in pixels) a click resizes it instead of moving it.
const EDGE_GRAB: f32 = 6.0;

/// The note lengths offered for new notes, in steps.
const NOTE_LENGTHS: [(usize, &str); 5] = [
    (1, "1 step"),
    (2, "2 steps"),
    (4, "1 beat"),
    (8, "2 beats"),
    (16, "1 bar"),
];

/// How fast an arpeggio goes: how long each of its notes is, in steps.
const ARPEGGIO_RATES: [(usize, &str); 3] = [(1, "1 step"), (2, "2 steps"), (4, "1 beat")];

const INVERSIONS: [&str; 6] = [
    "Root position",
    "1st inversion",
    "2nd inversion",
    "3rd inversion",
    "4th inversion",
    "5th inversion",
];

/// The same, short enough for the toolbar.
const INVERSIONS_SHORT: [&str; 6] = [
    "Root", "1st inv.", "2nd inv.", "3rd inv.", "4th inv.", "5th inv.",
];

/// Below this height (in points, under the toolbar) the piano roll
/// leaves out its hint lines, to give the grid the room.
const COMPACT_HEIGHT: f32 = 420.0;

const ROOT_ROW: Color32 = Color32::from_rgb(78, 70, 60);
const SCALE_ROW: Color32 = Color32::from_rgb(60, 70, 76);
const OTHER_ROW: Color32 = Color32::from_rgb(44, 51, 56);
const ROW_LINE: Color32 = Color32::from_rgb(36, 42, 46);
const OCTAVE_LINE: Color32 = Color32::from_rgb(22, 26, 29);
const STEP_LINE: Color32 = Color32::from_rgb(52, 60, 66);
const BEAT_LINE: Color32 = Color32::from_rgb(34, 40, 44);
const BAR_LINE: Color32 = Color32::from_rgb(20, 24, 27);
const WHITE_KEY: Color32 = Color32::from_rgb(222, 224, 228);
const BLACK_KEY: Color32 = Color32::from_rgb(40, 42, 46);
const KEY_TEXT: Color32 = Color32::from_rgb(60, 62, 68);
const KEY_HELD: Color32 = Color32::from_rgb(255, 152, 48);
const SCALE_MARK: Color32 = Color32::from_rgb(120, 190, 255);
const ROW_MARK: Color32 = Color32::from_rgb(54, 96, 168);
const ROOT_MARK: Color32 = Color32::from_rgb(255, 152, 48);
const NOTE: Color32 = Color32::from_rgb(118, 214, 140);
const NOTE_OUTSIDE: Color32 = Color32::from_rgb(240, 120, 110);
/// A soloist's notes, by what they are: chord tones in the usual green,
/// other notes of the scale blue, chromatic notes orange.
const SOLO_SCALE: Color32 = Color32::from_rgb(120, 172, 236);
const SOLO_CHROMATIC: Color32 = Color32::from_rgb(242, 164, 92);
const NOTE_BORDER: Color32 = Color32::from_rgb(24, 40, 30);
const NOTE_TEXT: Color32 = Color32::from_rgb(16, 28, 20);
const SELECTED_BORDER: Color32 = Color32::from_rgb(255, 255, 255);
const STAMP_GHOST: Color32 = Color32::from_rgba_premultiplied(60, 70, 64, 60);
const OTHER_CHANNEL: Color32 = Color32::from_rgba_premultiplied(70, 76, 90, 110);
const OTHER_CHANNEL_BORDER: Color32 = Color32::from_rgb(120, 128, 146);
const SELECTION_BOX: Color32 = Color32::from_rgba_premultiplied(40, 60, 90, 60);
const PLAYHEAD: Color32 = Color32::from_rgba_premultiplied(64, 38, 12, 60);
const LANE_BACKGROUND: Color32 = Color32::from_rgb(36, 42, 46);
const LANE_CHORD: Color32 = Color32::from_rgb(72, 88, 120);
pub(crate) const LANE_TEXT: Color32 = Color32::from_rgb(235, 238, 245);
const NO_NOTES_BACKGROUND: Color32 = Color32::from_rgb(70, 52, 30);
const NO_NOTES_TEXT: Color32 = Color32::from_rgb(150, 156, 170);
/// The voice-leading check: a move classical part writing avoids, over
/// the notes, and its block in the chord lane.
const SLIP: Color32 = Color32::from_rgb(236, 116, 220);
const LANE_SLIP: Color32 = Color32::from_rgb(112, 52, 108);
/// A form of the twelve-tone row, in the chord lane.
const LANE_ROW_FORM: Color32 = Color32::from_rgb(46, 82, 98);
/// The lines that join the notes of each part, with the voice-leading
/// check on: one colour for each part, from the top down, over again
/// after the last.
const PART_LINES: [Color32; 6] = [
    Color32::from_rgb(110, 196, 236),
    Color32::from_rgb(240, 200, 96),
    Color32::from_rgb(126, 214, 140),
    Color32::from_rgb(244, 150, 112),
    Color32::from_rgb(172, 150, 240),
    Color32::from_rgb(220, 220, 220),
];

/// What "Voice leading" checks, for its tooltips.
const VOICE_LEADING_HELP: &str = "Checks the parts as classical harmony teaches four-part \
    writing: no parallel fifths or octaves, the leading tone rising to the tonic in the top \
    part and the bass, a chord's seventh falling a step. Each track is a part, or several \
    when it plays chords; with Ghost notes the other tracks are checked too. These are the \
    rules of the style of Bach's chorales and of harmony exercises, not of all music: jazz \
    and pop move in parallel on purpose. Where the key is a twelve-tone row, it checks the \
    rules of twelve-tone writing instead.";

/// What the line of moves under the chord names says, for its tooltip.
const MOVES_HELP: &str = "A step is a second, a skip a third, a leap a fourth or more. \
    Parallel motion keeps the interval between two parts, similar motion goes the same way \
    by another, contrary motion goes opposite ways and oblique motion holds one part while \
    the other moves. Classical part writing moves the inner parts by step where it can, and \
    keeps the top part and the bass in contrary or oblique motion. The coloured lines over \
    the notes join the notes of each part.";

/// What "Voice leading" checks where the key is a twelve-tone row.
const TWELVE_TONE_HELP: &str = "Checks the rules of twelve-tone writing, as Schoenberg and his \
    students taught them: the notes follow forms of the row (P, I, R and RI, on any note) \
    one after another, each in its order; notes next to each other in the row may sound \
    together as a chord, and a note may be repeated straight away, but it comes back only \
    after the other eleven; a form is played whole; no note is doubled in octaves; and no \
    chord sounds like tonal harmony (a major or minor triad, a dominant seventh). The Row \
    line under the chords shows the forms the check reads in the notes.";

/// A drag that started on a note and lasts until the mouse button is released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Drag {
    /// Moving a note. `grab` is how many steps after the note's start the
    /// pointer took hold of it, so the note does not jump under the pointer.
    Move { index: usize, grab: usize },
    /// Changing a note's length by dragging its right end.
    Resize { index: usize },
}

/// Where the chord that next-chord hints follow comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChordAt {
    /// The chord at a step of the pattern.
    Step(usize),
    /// The keys held down on a keyboard.
    Keys,
}

impl Drag {
    fn index(self) -> usize {
        match self {
            Drag::Move { index, .. } | Drag::Resize { index } => index,
        }
    }
}

/// Moving the selected notes together.
#[derive(Debug, Clone, PartialEq)]
struct GroupDrag {
    /// The cell where the drag started.
    from_step: usize,
    from_pitch: Note,
    /// The notes as they were when the drag started.
    originals: Vec<NoteEvent>,
    /// The notes as they are now, after the moves so far.
    moved: Vec<NoteEvent>,
    /// Whether the notes have moved at all. With snap to scale on, notes
    /// snap only once they are moved: dragging a
    /// selection up a semitone and back down snaps it into the key.
    edited: bool,
}

/// Something that changes the selected notes (or every note, when none
/// is selected), from the Tools menu or a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    Copy,
    Cut,
    Paste,
    Duplicate,
    Transpose(i32),
    InKey(i32),
    Shift(isize),
    Quantize,
    SnapToKey,
    Invert { up: bool },
    Arpeggiate,
    Delete,
    SelectAll,
    SelectNone,
}

/// What the grid needs to know besides the notes, gathered in one value
/// so the drawing functions do not need a long list of arguments.
#[derive(Debug, Clone, Copy)]
struct View<'a> {
    scale: Scale,
    step_width: f32,
    playhead: Option<usize>,
    held: &'a [Note],
    /// For the soloist's part, what each note is.
    roles: Option<&'a [Option<(NoteRole, Over)>]>,
    /// The voice-leading check, when it is on.
    voices: Option<&'a VoiceLeading>,
}

/// What every part of the roll is drawn from in a frame, gathered in one
/// value as [`View`] is for the grid.
#[derive(Clone, Copy)]
struct Drawing<'a> {
    project: &'a Project,
    pattern: &'a Pattern,
    /// The pattern's key: the song's key where the pattern is played.
    scale: Scale,
    /// The channels heard together: this one, and the others when they
    /// are shown behind it.
    heard: &'a [usize],
    /// For the soloist's part, what each note is.
    roles: Option<&'a [Option<(NoteRole, Over)>]>,
    playhead: Option<usize>,
    /// The notes held down on a keyboard.
    held: &'a [Note],
    /// The voice-leading check of the channels heard, when it is on.
    voices: Option<&'a VoiceLeading>,
}

/// What a voice-leading check was made from: while none of it changes,
/// the check stands, and is not made again on every frame.
#[derive(Debug, Clone, PartialEq)]
struct VoiceInput {
    pattern: usize,
    /// The notes of each channel checked.
    notes: Vec<(usize, Vec<NoteEvent>)>,
    time: TimeSignature,
    /// The key on each beat.
    keys: Vec<Scale>,
}

/// Where the grid is on screen, and how to turn steps and pitches into
/// pixels and back. It is `Copy`, so it can be handed around freely.
#[derive(Debug, Clone, Copy)]
struct Grid {
    /// Where all the steps and rows are, even those scrolled out of view.
    rect: Rect,
    step_width: f32,
    /// How tall a row (one pitch) is: the vertical zoom.
    row_height: f32,
    length: usize,
    time: TimeSignature,
    /// The left edge of the keyboard. The keyboard stays at the left of
    /// the view while the grid scrolls sideways under it.
    keys_left: f32,
}

impl Grid {
    /// The keyboard, at the left of the view.
    fn keys(self) -> Rect {
        Rect::from_min_max(
            pos2(self.keys_left, self.rect.top()),
            pos2(self.keys_left + KEY_WIDTH, self.rect.bottom()),
        )
    }

    /// The part of the grid not under the keyboard, where notes are drawn
    /// and clicked.
    fn cells(self) -> Rect {
        let mut cells = self.rect;
        cells.min.x = cells.min.x.max(self.keys().right());
        cells
    }

    /// The x coordinate where `step` begins.
    fn x(self, step: usize) -> f32 {
        self.rect.left() + step as f32 * self.step_width
    }

    /// The step under an x coordinate, kept inside the grid.
    fn step_at(self, x: f32) -> usize {
        let last_step = self.length.saturating_sub(1) as f32;
        ((x - self.rect.left()) / self.step_width)
            .floor()
            .clamp(0.0, last_step) as usize
    }

    /// The y coordinate of the top of a pitch's row (high notes at the top).
    fn row_top(self, midi: u8) -> f32 {
        self.rect.top() + (HIGHEST - midi) as f32 * self.row_height
    }

    /// Where a note is drawn: from its exact start to its exact end, so
    /// a triplet sits between two steps, as it is heard.
    fn note_rect(self, note: &NoteEvent) -> Rect {
        let top = self.row_top(note.pitch.midi());
        let tick = self.step_width / TICKS_PER_STEP as f32;
        let start = self.x(note.start) + f32::from(note.nudge) * tick;
        let end = if note.end() > self.length {
            self.x(self.length)
        } else {
            self.x(note.end()) - f32::from(note.trim) * tick
        };
        Rect::from_min_max(pos2(start, top), pos2(end, top + self.row_height))
    }

    /// The step and pitch under a point. Points outside the grid give the
    /// nearest cell, so a drag can go past the edges.
    fn cell_at(self, pos: Pos2) -> (usize, Note) {
        let row = ((pos.y - self.rect.top()) / self.row_height)
            .floor()
            .clamp(0.0, (HIGHEST - LOWEST) as f32) as u8;
        (self.step_at(pos.x), Note::new(HIGHEST - row))
    }

    /// The size of the text written on notes and keys, which grows and
    /// shrinks with the rows.
    fn text_size(self) -> f32 {
        (self.row_height * 0.8).clamp(7.0, 14.0)
    }
}

/// What the chord and velocity lanes need to line up with the grid.
#[derive(Debug, Clone, Copy)]
struct Lanes {
    step_width: f32,
    /// How far the grid is scrolled sideways, in pixels.
    scroll_x: f32,
    length: usize,
}

impl Lanes {
    /// A lane in `rect`, its label where the keyboard is, its steps lined
    /// up with the grid's.
    fn grid(self, rect: Rect, time: TimeSignature) -> Grid {
        let left = rect.left() + KEY_WIDTH - self.scroll_x;
        Grid {
            rect: Rect::from_min_max(
                pos2(left, rect.top()),
                pos2(left + self.step_width * self.length as f32, rect.bottom()),
            ),
            step_width: self.step_width,
            row_height: rect.height(),
            length: self.length,
            time,
            keys_left: rect.left(),
        }
    }
}

/// The piano roll window and everything it remembers between frames.
pub struct PianoRoll {
    clipboard: Vec<NoteEvent>,
    /// The Tools menu's Paste asked for the system clipboard, which comes
    /// next frame as a paste event, as with Ctrl+V.
    paste_requested: bool,
    pub instance: Option<egui::Id>,
    pub open: bool,
    channel: usize,
    channel_selected: bool,
    /// An explicit rack or editor choice must survive empty patterns and reopening.
    explicit_channel: bool,
    /// The pattern shown last time, to notice when it changes.
    pattern: usize,
    stamp: ChordStamp,
    /// How many times stamped chords are inverted (0 is root position).
    inversion: usize,
    /// The length of new notes, in steps. Resizing a note changes it, so
    /// the next note drawn is as long as the last one edited.
    note_length: usize,
    drag: Option<Drag>,
    group_drag: Option<GroupDrag>,
    /// Where a selection box started, while Ctrl is held and the button is down.
    selection_box: Option<Pos2>,
    /// The selected notes, by value. A note that is no longer in the
    /// pattern (after an undo, say) is simply not found, and so not
    /// selected: the selection can never point at the wrong note.
    selection: Vec<NoteEvent>,
    /// Show the notes of the other channels behind this one's, and name
    /// the chords they make together.
    ghosts: bool,
    /// Check the parts as classical harmony teaches ("Voice leading"),
    /// and the last check, with what it was made from.
    voice_leading: bool,
    voice_check: Option<(VoiceInput, Rc<VoiceLeading>)>,
    arpeggio: ArpeggioOrder,
    arpeggio_rate: usize,
    /// Scroll to the channel's notes the next time the grid is drawn.
    scroll_to_notes: bool,
    /// The lowest and highest pitch in view when the grid was last drawn.
    in_view: Option<(u8, u8)>,
    /// How far the grid was scrolled last frame, and where on screen its
    /// view was: to keep the notes under the pointer in place when zooming.
    scrolled: Vec2,
    view_rect: Option<Rect>,
    /// Where the "no notes here" message was drawn over the grid last
    /// frame, so a click on it does not also draw a note under it.
    banner: Option<Rect>,
    /// Where to scroll the grid to the next time it is drawn.
    scroll_to: Option<Vec2>,
    reveal: Option<usize>,
    /// Whether the window was open last frame, to notice it opening.
    was_open: bool,
    /// How much wider and taller than usual the steps and rows are: the
    /// piano roll's horizontal and vertical zoom.
    zoom: Vec2,
    /// The step the pointer is over, if any, to name the chord there.
    hovered_step: Option<usize>,
    /// The last step the pointer was over. Unlike `hovered_step`, it stays
    /// when the pointer leaves, so the next-chord hints stay up while you
    /// move to the buttons.
    last_step: Option<usize>,
    /// Whether the arrow keys and Delete go to the piano roll: from a
    /// click on the window until a click anywhere else.
    has_keyboard: bool,
    /// The project's time signature, which says where the bar and beat
    /// lines go. Copied from the project on every frame.
    time: TimeSignature,
    /// How much the next-chord hints explain. Set by the app from the
    /// harmony keys bar, where it is chosen.
    pub hints: HintLevel,
    /// Whether notes drawn or moved land on the notes of the key ("Snap to
    /// scale").
    snap_to_scale: bool,
    /// Whether notes played on the typing or MIDI keyboard are moved to
    /// the nearest note of the key.
    /// The app reads it.
    pub snap_played: bool,
    /// The twelve-tone row window and the set browser.
    row_window: RowWindow,
    set_window: SetWindow,
    /// Kröpfl's micromodes, linked to the set browser.
    micromode_window: MicromodeWindow,
    /// Drawn for a video: the notes, without the menus and the
    /// velocities, which nobody can use there.
    for_video: bool,
}

impl PianoRoll {
    /// A closed piano roll for `channel`.
    pub fn new(channel: usize) -> Self {
        PianoRoll {
            clipboard: Vec::new(),
            paste_requested: false,
            instance: None,
            open: false,
            channel,
            channel_selected: true,
            explicit_channel: false,
            pattern: 0,
            stamp: ChordStamp::Single,
            inversion: 0,
            note_length: 4,
            drag: None,
            group_drag: None,
            selection_box: None,
            selection: Vec::new(),
            ghosts: true,
            voice_leading: false,
            voice_check: None,
            arpeggio: ArpeggioOrder::Up,
            arpeggio_rate: 1,
            scroll_to_notes: true,
            in_view: None,
            scrolled: Vec2::ZERO,
            view_rect: None,
            banner: None,
            scroll_to: None,
            reveal: None,
            was_open: false,
            zoom: Vec2::splat(1.0),
            hovered_step: None,
            last_step: None,
            has_keyboard: false,
            time: TimeSignature::COMMON,
            hints: HintLevel::default(),
            snap_to_scale: false,
            snap_played: false,
            row_window: RowWindow::new(),
            set_window: SetWindow::new(),
            micromode_window: MicromodeWindow::new(),
            for_video: false,
        }
    }

    /// `pitch`, moved to the nearest note of `scale` if snap to scale is on.
    fn snapped(&self, pitch: Note, scale: Scale) -> Note {
        match self.snap_to_scale {
            true => tools::nearest_in_key(pitch, scale),
            false => pitch,
        }
    }

    /// The id of the piano roll window.
    pub fn id() -> egui::Id {
        egui::Id::unique("piano-roll")
    }

    /// The channel shown: the selected channel, which keyboards play.
    pub fn channel(&self) -> usize {
        self.channel
    }

    pub fn selected_channel(&self) -> Option<usize> {
        self.channel_selected.then_some(self.channel)
    }

    /// The selected channel, if the user chose it rather than the piano
    /// roll picking the first one there was.
    pub fn chosen_channel(&self) -> Option<usize> {
        self.selected_channel().filter(|_| self.explicit_channel)
    }

    pub fn reveal_step(&mut self, step: usize) {
        self.reveal = Some(step);
    }

    pub fn deselect_channel(&mut self) {
        self.channel_selected = false;
        self.explicit_channel = true;
        self.open = false;
        self.was_open = false;
        self.has_keyboard = false;
        self.selection.clear();
        self.stop_dragging();
    }

    /// Shows another channel, without opening the window.
    pub fn select(&mut self, channel: usize) {
        let open = self.open;
        self.open_for(channel);
        self.open = open;
    }

    /// Opens the piano roll on a channel.
    pub fn open_for(&mut self, channel: usize) {
        if channel != self.channel {
            self.scroll_to_notes = true;
            self.selection.clear();
        }
        self.open = true;
        self.channel = channel;
        self.channel_selected = true;
        self.explicit_channel = true;
        self.stop_dragging();
    }

    /// Keeps pointing at the same channel after channel `removed` is
    /// deleted, or closes the window if it was this one. `left` is how
    /// many channels are left: the window then points at the channel
    /// that took the removed one's place (or the last one), so it opens
    /// again on a channel that is there.
    pub fn channel_removed(&mut self, removed: usize, left: usize) {
        if removed == self.channel {
            self.deselect_channel();
            self.channel = self.channel.min(left.saturating_sub(1));
        } else if removed < self.channel {
            self.channel -= 1;
        }
        self.stop_dragging();
    }

    /// Called after a channel was inserted above or at the one shown (by
    /// undo, for example), so the piano roll keeps showing the same channel.
    pub fn channel_inserted(&mut self, inserted: usize) {
        if inserted <= self.channel {
            self.channel += 1;
        }
        self.stop_dragging();
    }

    fn stop_dragging(&mut self) {
        self.drag = None;
        self.group_drag = None;
        self.selection_box = None;
    }

    /// Draws the window, if it is open, and returns what the user did.
    /// It shows the notes of the selected channel in pattern `pattern`;
    /// `playhead` is the step of that pattern being played, if any, and
    /// `held` the keys held down on a keyboard. The window stays inside
    /// `area`, the part of the main window the playlist uses. Its steps
    /// are `zoom.x` times wider than usual and its rows `zoom.y` times taller.
    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        project: &Project,
        pattern: usize,
        playhead: Option<usize>,
        held: &[Note],
        area: egui::Rect,
        zoom: Vec2,
    ) -> Vec<RollAction> {
        let mut actions = Vec::new();
        if self.channel >= project.channels().len() && self.channel_selected {
            self.deselect_channel();
            self.channel = project.channels().len().saturating_sub(1);
        }
        if !self.channel_selected {
            if !self.open || project.channels().is_empty() {
                self.has_keyboard = false;
                return actions;
            }
            // Opening the piano roll explicitly restores its last valid channel.
            self.channel_selected = true;
            self.channel = self.channel.min(project.channels().len() - 1);
        }
        if zoom != self.zoom {
            self.zoom_to(ctx, zoom);
        }
        let Some(pattern_notes) = project.pattern(pattern) else {
            self.open = false;
            return actions;
        };
        let opened = self.open && !self.was_open;
        let new_pattern = pattern != self.pattern;
        if new_pattern {
            self.pattern = pattern;
            self.scroll_to_notes = true;
            self.selection.clear();
            self.stop_dragging();
        }
        if (opened || new_pattern) && !self.explicit_channel && self.open {
            self.follow_notes(pattern_notes);
        }
        if opened {
            self.scroll_to_notes = true;
        }
        let Some(channel) = project.channels().get(self.channel) else {
            self.open = false;
            return actions;
        };

        // A click on the window gives it the keyboard; a click anywhere
        // else takes the keyboard away again, even inside its rectangle
        // when another window (the chord sheet, the rack) is on top
        // there. A click in a menu or a list leaves the keyboard where
        // it was.
        self.has_keyboard = crate::widgets::editor_keyboard(
            ctx,
            egui::LayerId::new(egui::Order::Middle, self.instance.unwrap_or_else(Self::id)),
        );

        // `open` is a copy: the window needs `&mut open` while the closure
        // below needs `&mut self`, and both cannot borrow `self` at once.
        let mut open = self.open;
        let id = self.instance.unwrap_or_else(Self::id);
        let window = egui::Window::new(format!(
            "Piano roll · {} · {}",
            pattern_notes.name, channel.name
        ))
        .id(id)
        .open(&mut open)
        .default_pos([150.0, 160.0])
        .constrain_to(area);
        // The window is built while closed too, and egui then hides it.
        let window = if self.open {
            let size = egui::vec2(1120.0, 580.0);
            crate::widgets::open_at(ctx, window, id, size, crate::widgets::Opening::FirstTime)
        } else {
            window
        };
        window.show(ctx, |ui| {
            self.contents(ui, project, pattern_notes, playhead, held, &mut actions)
        });
        self.open = open;
        self.was_open = open;
        if !open {
            self.has_keyboard = false;
        }
        if open {
            self.post_tonal_windows(ctx, project, pattern_notes, held, area, &mut actions);
        }
        actions
    }

    /// Draws the piano roll's contents in `ui`, for a video: this
    /// channel's notes in `pattern`, with the step `playhead` playing.
    /// When the pattern changes, it scrolls to its notes.
    pub fn video(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        pattern: usize,
        playhead: Option<usize>,
    ) {
        let Some(notes) = project.pattern(pattern) else {
            return;
        };
        if pattern != self.pattern {
            self.pattern = pattern;
            self.scroll_to_notes = true;
        }
        self.for_video = true;
        let mut actions = Vec::new();
        self.contents(ui, project, notes, playhead, &[], &mut actions);
    }

    /// Changes the zoom, keeping the step and the pitch under the pointer
    /// where they are, or those in the middle of the view if the pointer
    /// is elsewhere: you zoom into what you point at.
    fn zoom_to(&mut self, ctx: &egui::Context, zoom: Vec2) {
        let ratio = zoom / self.zoom;
        self.zoom = zoom;
        let Some(view) = self.view_rect else {
            return;
        };
        let anchor = ctx
            .pointer_hover_pos()
            .filter(|pos| view.contains(*pos))
            .unwrap_or(view.center())
            - view.min;
        // The grid starts after the keyboard, which does not stretch.
        let from = self.scroll_to.unwrap_or(self.scrolled);
        let x = KEY_WIDTH + (from.x + anchor.x - KEY_WIDTH) * ratio.x - anchor.x;
        let y = (from.y + anchor.y) * ratio.y - anchor.y;
        self.scroll_to = Some(vec2(x.max(0.0), y.max(0.0)));
    }

    /// The row window and the set browser, when they are open.
    fn post_tonal_windows(
        &mut self,
        ctx: &egui::Context,
        project: &Project,
        pattern: &Pattern,
        held: &[Note],
        area: egui::Rect,
        actions: &mut Vec<RollAction>,
    ) {
        let scale = project.scale;
        twelve_tone::remember(ctx, scale);
        let length = pattern.length();
        let visible: Vec<NoteEvent> = pattern
            .notes(self.channel)
            .iter()
            .filter(|note| note.start < length)
            .copied()
            .collect();
        if self.row_window.open {
            // The notes a row is read from, in the order they start.
            let mut drawn = if self.selection.is_empty() {
                visible.clone()
            } else {
                self.selection.clone()
            };
            drawn.sort_by_key(|note| (note.start, note.pitch));
            let drawn: Vec<Note> = drawn.iter().map(|note| note.pitch).collect();
            let can_create_pattern = self.instance.is_none()
                && project.patterns().len() < lineup_core::project::MAX_PATTERNS;
            for action in self
                .row_window
                .show(ctx, area, scale, &drawn, length, can_create_pattern)
            {
                match action {
                    RowAction::UseAsKey(row) => actions.push(RollAction::SetScale(row)),
                    RowAction::Write(notes) => {
                        let notes: Vec<NoteEvent> =
                            notes.into_iter().filter(|n| fits_view(&[*n])).collect();
                        self.selection.clear();
                        self.replace(pattern, actions, "write a row", visible.clone(), notes);
                    }
                    RowAction::NewPattern { name, notes } => {
                        actions.push(RollAction::NewPattern {
                            channel: self.channel,
                            name,
                            length,
                            notes: notes
                                .into_iter()
                                .filter(|note| fits_view(&[*note]))
                                .collect(),
                        });
                    }
                }
            }
        }
        let selected: PitchSet = if held.is_empty() {
            self.selection
                .iter()
                .map(|note| note.pitch.pitch_class())
                .collect()
        } else {
            held.iter().map(|note| note.pitch_class()).collect()
        };
        let drawing = self.stamp;
        if self.set_window.open {
            for action in self
                .set_window
                .show(ctx, area, scale, selected, &mut self.stamp)
            {
                match action {
                    SetAction::Hear(pitches) => actions.push(RollAction::Preview {
                        channel: self.channel,
                        pitches,
                    }),
                    SetAction::UseAsKey(set) => actions.push(RollAction::SetScale(set)),
                    SetAction::ShowMicromode(trio) => {
                        self.micromode_window.pick(trio);
                        self.micromode_window.open = true;
                    }
                    SetAction::OpenMicromodes => self.micromode_window.open = true,
                }
            }
        }
        if self.micromode_window.open {
            for action in self
                .micromode_window
                .show(ctx, area, scale, selected, &mut self.stamp)
            {
                match action {
                    MicroAction::UseAsKey(set) => actions.push(RollAction::SetScale(set)),
                    MicroAction::Hear(pitches) => actions.push(RollAction::Preview {
                        channel: self.channel,
                        pitches,
                    }),
                    MicroAction::ShowSet(set) => {
                        self.set_window.pick(set);
                        self.set_window.open = true;
                    }
                }
            }
        }
        if self.stamp != drawing {
            self.inversion = 0;
        }
    }

    /// When the channel shown has no notes in `pattern` but another
    /// channel plays a melody or chords there, shows that channel instead,
    /// so the piano roll never opens on an empty grid when there is
    /// something to see.
    fn follow_notes(&mut self, pattern: &Pattern) {
        if !pattern.notes(self.channel).is_empty() {
            return;
        }
        if let Some(channel) = busiest_channel(pattern) {
            self.channel = channel;
            self.selection.clear();
            self.scroll_to_notes = true;
        }
    }

    /// Said over the top of the grid when the channel has no notes in the
    /// pattern: what is going on, and buttons to go where the notes are.
    /// Returns where it was drawn.
    ///
    /// It lies over the grid instead of taking a line above it, so the
    /// grid does not jump up when the first note is drawn and the message
    /// goes away: the note would land rows below where it was clicked.
    fn no_notes(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        pattern: &Pattern,
        actions: &mut Vec<RollAction>,
    ) -> Rect {
        let channel = project
            .channels()
            .get(self.channel)
            .map_or("This channel", |channel| channel.name.as_str());
        // Other patterns where this channel has notes.
        let elsewhere: Vec<(usize, &str)> = project
            .patterns()
            .iter()
            .enumerate()
            .filter(|&(index, other)| {
                index != self.pattern && !other.notes(self.channel).is_empty()
            })
            .map(|(index, other)| (index, other.name.as_str()))
            .collect();
        // Other channels with notes in this pattern.
        let others: Vec<(usize, &str, bool)> = (0..pattern.rows())
            .filter(|&other| other != self.channel && !pattern.notes(other).is_empty())
            .filter_map(|other| {
                let name = project.channels().get(other)?.name.as_str();
                Some((other, name, pattern.is_step_row(other)))
            })
            .collect();
        let mut switch_to = None;
        let frame = egui::Frame::new()
            .fill(NO_NOTES_BACKGROUND)
            .corner_radius(4.0)
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.strong(format!(
                        "{channel} has no notes in \"{}\". Click the grid to draw some.",
                        pattern.name
                    ));
                    if !elsewhere.is_empty() {
                        ui.label(format!("{channel} plays in:"));
                        for &(index, name) in &elsewhere {
                            if ui
                                .button(name)
                                .on_hover_text(format!("Show the pattern \"{name}\""))
                                .clicked()
                            {
                                actions.push(RollAction::SelectPattern(index));
                            }
                        }
                    }
                    let melodies: Vec<_> = others.iter().filter(|other| !other.2).collect();
                    if !melodies.is_empty() {
                        ui.label("Notes here are on:");
                        for &&(index, name, _) in &melodies {
                            if ui.button(name).clicked() {
                                switch_to = Some(index);
                            }
                        }
                    }
                    if others.iter().any(|other| other.2) {
                        ui.label("The drums in this pattern are edited in the");
                        if ui.button("Channel rack").clicked() {
                            actions.push(RollAction::OpenRack);
                        }
                    }
                });
            });
        if let Some(channel) = switch_to {
            self.open_for(channel);
        }
        frame.response.rect
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        pattern: &Pattern,
        playhead: Option<usize>,
        held: &[Note],
        actions: &mut Vec<RollAction>,
    ) {
        // A pattern played where the song has changed key is highlighted
        // and its chords named in that key.
        let scale = project.pattern_key(self.pattern);
        // A pattern used where the song is in 3/4 is drawn in bars of 3/4.
        self.time = project.pattern_time(self.pattern);
        // Selected notes that are no longer there are forgotten.
        let notes = pattern.notes(self.channel);
        self.selection.retain(|note| notes.contains(note));
        // The soloist's notes are shown with what each one is.
        let roles = match project.player_channel(lineup_core::Player::Soloist) {
            Some(channel) if channel == self.channel => project.solo_roles(self.pattern),
            _ => None,
        };

        // The other channels whose notes are shown faintly behind this
        // one's: those that play melodies or chords. Drum channels, whose
        // notes are all plain steps, would only add noise.
        let others: Vec<usize> = (0..pattern.rows())
            .filter(|&other| other != self.channel)
            .filter(|&other| pattern.notes(other).iter().any(|note| !note.is_step()))
            .collect();
        let mut heard = vec![self.channel];
        if self.ghosts {
            heard.extend(&others);
        }
        let voices = self
            .voice_leading
            .then(|| self.check_voices(project, pattern, &heard));
        let drawing = Drawing {
            project,
            pattern,
            scale,
            heard: &heard,
            roles: roles.as_deref(),
            playhead,
            held,
            voices: voices.as_deref(),
        };

        let tool = if self.for_video {
            None
        } else {
            self.toolbar(ui, project.scale, scale, others.is_empty(), actions)
        };
        // In a short window the grid comes first: the hint and set lines
        // move into the tooltip of the line naming the chord, so a small
        // screen still shows a dozen rows of notes.
        let compact = ui.available_height() < COMPACT_HEIGHT;
        let current = self.current_chord(drawing);
        let next_hint = self.chords_in_key(ui, scale, current, compact, actions);
        self.about_the_notes(ui, drawing, &next_hint, compact);
        if let Some(check) = drawing.voices {
            self.voice_line(ui, drawing, check);
        }
        ui.add_space(2.0);
        self.grid_and_lanes(ui, drawing, &others, actions);

        if let Some(step) = self.hovered_step {
            self.last_step = Some(step);
        }
        self.use_picked_tool(ui, tool, pattern, scale, actions);
    }

    /// The voice leading of the channels `heard` in `pattern`, the
    /// pattern shown, checked again only when their notes, the keys or
    /// the meter change.
    fn check_voices(
        &mut self,
        project: &Project,
        pattern: &Pattern,
        heard: &[usize],
    ) -> Rc<VoiceLeading> {
        let time = project.pattern_time(self.pattern);
        let keys = voice_leading::beats(time, pattern.length())
            .iter()
            .map(|&(start, _)| project.pattern_key_at(self.pattern, start))
            .collect();
        let input = VoiceInput {
            pattern: self.pattern,
            notes: heard
                .iter()
                .map(|&channel| (channel, pattern.notes(channel).to_vec()))
                .collect(),
            time,
            keys,
        };
        if let Some((made_from, check)) = &self.voice_check
            && *made_from == input
        {
            return Rc::clone(check);
        }
        let check = Rc::new(project.voice_leading(self.pattern, heard));
        self.voice_check = Some((input, Rc::clone(&check)));
        check
    }

    /// The line under the chord names with what the voice-leading check
    /// found at the step pointed at, or else in the whole pattern. Its
    /// tooltip says why classical part writing avoids it. It is cut to
    /// one line, so the grid does not jump as the pointer moves. With the
    /// pointer on the line itself, it keeps what it said for the step
    /// pointed at last, so that its tooltip can be read.
    fn voice_line(&self, ui: &mut Ui, drawing: Drawing, check: &VoiceLeading) {
        let name = |voice: usize| voice_name(drawing.project, check, voice);
        let line = Rect::from_min_size(
            ui.cursor().min,
            vec2(ui.available_width(), ui.text_style_height(&TextStyle::Body)),
        );
        let step = self
            .hovered_step
            .or(drawing.playhead)
            .or(self.last_step.filter(|_| ui.rect_contains_pointer(line)));
        let here: Vec<&Slip> = step
            .map(|step| check.slips_at(step).collect())
            .unwrap_or_default();
        // Where nothing is found, how each part moves at the change
        // pointed at (or the last one before it).
        let changes = part_moves(check);
        let change = step.and_then(|step| voice_motion::change_at(&changes, step, usize::MAX));
        if here.is_empty()
            && let Some(change) = change
        {
            // On one track, the parts are told apart by their place.
            let one_track = check
                .voices
                .windows(2)
                .all(|pair| pair[0].channel == pair[1].channel);
            let moves = match one_track {
                true => {
                    let track = drawing
                        .project
                        .channels()
                        .get(check.voices[0].channel)
                        .map_or("Track", |channel| channel.name.as_str());
                    let place = |voice: usize| part_place(&check.voices[voice]);
                    format!("{track}: {}", change.summary(drawing.scale, place))
                }
                false => change.summary(drawing.scale, name),
            };
            ui.add(egui::Label::new(egui::RichText::new(moves).color(theme::TEXT_DIM)).truncate())
                .on_hover_text(format!(
                    "How each part moves at step {}, highest first:\n\n{}\n\n{MOVES_HELP}",
                    change.step + 1,
                    change.describe(drawing.scale, name)
                ));
        } else if here.is_empty() {
            let help = match check.rules {
                Rules::TwelveTone => TWELVE_TONE_HELP,
                _ => VOICE_LEADING_HELP,
            };
            ui.add(egui::Label::new(egui::RichText::new(check.summary()).weak()).truncate())
                .on_hover_text(help);
        } else {
            let what: Vec<String> = here.iter().map(|slip| slip.what(name)).collect();
            let why: Vec<String> = here.iter().map(|slip| slip.describe(name)).collect();
            ui.add(egui::Label::new(egui::RichText::new(what.join(" ")).color(SLIP)).truncate())
                .on_hover_text(why.join("\n\n"));
        }
    }

    /// The chord that the next-chord hints follow. A chord held on a
    /// keyboard comes first: it is what you are thinking about right
    /// now. Then the chord at the last step pointed at, or playing.
    fn current_chord(&self, drawing: Drawing) -> Option<(ChordAt, Chord, PitchClass)> {
        let held_chord = match analyze(drawing.held.iter().copied()) {
            Harmony::Chord { chord, bass } => Some((ChordAt::Keys, chord, bass)),
            _ => None,
        };
        held_chord.or_else(|| {
            let step = self.last_step.or(drawing.playhead)?;
            match analyze(sounding(drawing.pattern, drawing.heard, step)) {
                Harmony::Chord { chord, bass } => Some((ChordAt::Step(step), chord, bass)),
                _ => None,
            }
        })
    }

    /// The lines above the grid that name what is pointed at: the chord
    /// at the step under the pointer (or the keys held down, or a
    /// soloist's note), and the set classes of those notes and of the
    /// selection. In a `compact` window, the sets and `next_hint` go in
    /// the first line's tooltip.
    fn about_the_notes(&self, ui: &mut Ui, drawing: Drawing, next_hint: &str, compact: bool) {
        let Drawing {
            scale,
            playhead,
            held,
            ..
        } = drawing;
        // The step under the pointer was found while drawing the last
        // frame; one frame late is too fast for anyone to notice.
        let (info, set) = if held.is_empty() {
            match self.hovered_step.or(playhead) {
                Some(step) => self.step_info(drawing, step),
                None => (
                    "Point at a step to name the chord there.".to_string(),
                    PitchSet::EMPTY,
                ),
            }
        } else {
            let harmony = analyze(held.iter().copied());
            let set: PitchSet = held.iter().map(|note| note.pitch_class()).collect();
            (format!("Keys down: {}", harmony.describe(scale)), set)
        };
        let solo_note = self.solo_note(drawing);
        let info = ui.label(solo_note.unwrap_or(info));
        let sets = self.set_lines(drawing, set);
        if compact {
            let sets = sets.join("\n");
            let more: Vec<&str> = [next_hint, sets.as_str()]
                .into_iter()
                .filter(|text| !text.trim().is_empty())
                .collect();
            if !more.is_empty() {
                info.on_hover_text(more.join("\n\n"));
            }
        } else {
            ui.weak(sets.join("    ")).on_hover_text(
                "Pitch-class set theory names any group of notes: its Forte number, its \
                     notes packed as tightly as they go (normal form), the same packed from 0 \
                     with its inversion (prime form), and how many of each interval it holds. \
                     Sets... opens the list of every set.",
            );
        }
    }

    /// What sounds at `step`: the chord of all the tracks, heard over the
    /// beat, and this track's own when it is another (a piano's C/E over
    /// a bass's C), with the notes of the tracks heard as a set.
    fn step_info(&self, drawing: Drawing, step: usize) -> (String, PitchSet) {
        let Drawing {
            project,
            pattern,
            scale,
            heard,
            ..
        } = drawing;
        let set: PitchSet = sounding(pattern, heard, step)
            .map(|note| note.pitch_class())
            .collect();
        let hearing = project.hearing_with_pattern(self.pattern, step);
        let all = hearing.best();
        let or = hearing
            .alternatives()
            .iter()
            .filter_map(|reading| reading.harmony.label_in(scale))
            .collect::<Vec<_>>()
            .join(", ");
        let track = analyze(sounding(pattern, &[self.channel], step));
        let own = match track.label_in(scale) {
            Some(label) if track != all => {
                let name = project
                    .channels()
                    .get(self.channel)
                    .map_or("This track", |channel| channel.name.as_str());
                match track {
                    Harmony::Chord { chord, bass } => match scale.figured_numeral(chord, bass) {
                        Some(numeral) => format!("{name}: {label} ({numeral}). "),
                        None => format!("{name}: {label}. "),
                    },
                    _ => format!("{name}: {label}. "),
                }
            }
            _ => String::new(),
        };
        (
            format!(
                "Step {}: {own}{}{}{}",
                step + 1,
                if own.is_empty() { "" } else { "All: " },
                all.describe(scale),
                if or.is_empty() {
                    String::new()
                } else {
                    format!(" (or {or})")
                }
            ),
            set,
        )
    }

    /// Over a soloist's note, what that note is.
    fn solo_note(&self, drawing: Drawing) -> Option<String> {
        let Drawing {
            project,
            pattern,
            roles,
            playhead,
            held,
            ..
        } = drawing;
        let notes = pattern.notes(self.channel);
        roles
            .zip(self.hovered_step.or(playhead).filter(|_| held.is_empty()))
            .and_then(|(roles, step)| {
                let (note, role) = notes
                    .iter()
                    .zip(roles)
                    .find(|(note, _)| (note.start..note.start + note.length).contains(&step))?;
                let (role, over) = (*role)?;
                let key = project.key_at(over.start);
                let name = key.note_name(note.pitch);
                // Band parts start at the top of the song, so the note's
                // place is counted as the transport counts it.
                let place = match project.meter().position(note.start) {
                    (bar, beat, 1) => format!("Bar {bar}, beat {beat}"),
                    (bar, beat, step) => format!("Bar {bar}, beat {beat}, step {step}"),
                };
                Some(format!("{place} · {}", role.describe(&name, &over, key)))
            })
    }

    /// The set lines: any group of notes, chord or not, has a set class.
    /// Shown for `set`, the notes pointed at, when it has three notes or
    /// more, and for the selection; for a soloist, what the colours mean.
    fn set_lines(&self, drawing: Drawing, set: PitchSet) -> Vec<String> {
        let scale = drawing.scale;
        let selected: PitchSet = self
            .selection
            .iter()
            .map(|note| note.pitch.pitch_class())
            .collect();
        let mut sets = Vec::new();
        if drawing.roles.is_some() {
            sets.push(
                "Soloist: chord tones green (R 3 5 7), scale notes blue (p passing, n \
                 neighbour, s other), chromatic orange (a approach, cp passing)"
                    .to_string(),
            );
        }
        if set.len() >= 3 {
            sets.push(format!("Set: {}", twelve_tone::describe_set(set, scale)));
        }
        if selected.len() >= 2 && self.selection.len() >= 2 {
            sets.push(format!(
                "Selected: {}",
                twelve_tone::describe_set(selected, scale)
            ));
        }
        // The line is always there, even empty, so the grid below does not
        // jump up and down as the pointer moves over it.
        if sets.is_empty() {
            sets.push(match self.hints {
                HintLevel::Minimal => " ".to_string(),
                _ => "Set: point at three notes or more, or select some, to name their set."
                    .to_string(),
            });
        }
        sets
    }

    /// The grid of notes, scrolling both ways, with the chord lane above
    /// it, the velocity lane below it, and over it a banner when the
    /// channel has no notes. `others` are the channels that can be shown
    /// faintly behind this one's notes.
    fn grid_and_lanes(
        &mut self,
        ui: &mut Ui,
        drawing: Drawing,
        others: &[usize],
        actions: &mut Vec<RollAction>,
    ) {
        let Drawing {
            project,
            pattern,
            scale,
            roles,
            playhead,
            held,
            voices,
            ..
        } = drawing;
        let length = pattern.length();
        // At 100% the whole pattern fits the window. Zooming in makes the
        // steps wider, and the grid scrolls sideways (Shift+wheel).
        let fit = ((ui.available_width() - KEY_WIDTH) / length as f32).max(MIN_STEP_WIDTH);
        let step_width = fit * self.zoom.x;
        self.hovered_step = None;

        // The chord lane's place is taken now, but it is drawn after the
        // grid, once it is known how far the grid is scrolled sideways.
        let lane_height = match voices.map(|check| check.rules) {
            Some(Rules::TwelveTone) => LANE_HEIGHT_WITH_ROWS,
            Some(_) => LANE_HEIGHT_WITH_VOICES,
            None => LANE_HEIGHT,
        };
        let (lane_rect, lane_response) =
            ui.allocate_exact_size(vec2(ui.available_width(), lane_height), Sense::hover());
        // The velocity lane goes below the grid, so the grid leaves room
        // for it: the lane, the gap, and egui's spacing between widgets.
        // Taking even a pixel too much would make the window grow a
        // little on every frame.
        let below = if self.for_video {
            ui.spacing().item_spacing.y
        } else {
            VELOCITY_HEIGHT + 4.0 + 2.0 * ui.spacing().item_spacing.y
        };
        let grid_height = (ui.available_height() - below).max(96.0);
        // Not animated: after a zoom, the notes that were under the
        // pointer must be there again on the very next frame.
        let grid_area = egui::ScrollArea::both()
            .id_salt("piano-roll-grid")
            .animated(false)
            .auto_shrink(false)
            .max_height(grid_height);
        let grid_area = match self.scroll_to.take() {
            Some(offset) => grid_area.scroll_offset(offset),
            None => grid_area,
        };
        let view = self.view_rect;
        crate::widgets::no_rubber_band(ui);
        let output = grid_area.show(ui, |ui| {
            if let (Some(view), Some(step)) = (view, self.reveal.take()) {
                let left = ui.cursor().min.x + step as f32 * step_width;
                ui.scroll_to_rect(
                    Rect::from_min_max(
                        pos2(left, view.center().y),
                        pos2(left + KEY_WIDTH + step_width, view.center().y + 1.0),
                    ),
                    None,
                );
            } else if let (Some(view), Some(step)) = (view, playhead) {
                crate::widgets::follow_x(
                    ui,
                    playhead,
                    ui.cursor().min.x + KEY_WIDTH + step as f32 * step_width,
                    view,
                    KEY_WIDTH,
                );
            }
            let ghosts = if self.ghosts { others } else { &[] };
            let view = View {
                scale,
                step_width,
                playhead,
                held,
                roles,
                voices,
            };
            self.grid(ui, pattern, ghosts, view, actions)
        });
        self.scrolled = output.state.clamped_offset();
        self.view_rect = Some(output.inner_rect);
        self.banner = pattern.notes(self.channel).is_empty().then(|| {
            // Drawn after the grid, so it is painted over it.
            let view = output.inner_rect;
            let place = Rect::from_min_max(
                view.min + vec2(KEY_WIDTH + 6.0, 6.0),
                pos2(view.right() - 6.0, view.bottom()),
            );
            let mut over = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(place)
                    .layout(egui::Layout::top_down(Align::Min)),
            );
            self.no_notes(&mut over, project, pattern, actions)
        });
        // How far the grid was scrolled sideways when it was drawn, which
        // the lanes above and below it follow.
        let scroll_x = output.inner;
        let lanes = Lanes {
            step_width,
            scroll_x,
            length,
        };
        self.chord_lane(ui, (lane_rect, &lane_response), drawing, lanes);
        if !self.for_video {
            ui.add_space(4.0);
            self.velocity_lane(ui, pattern, lanes, actions);
        }
    }

    /// Uses the tool picked from the Tools menu (`tool`), or with a key.
    /// Notes copied in another roll are on the system clipboard, so the
    /// menu's Paste reads it too, the frame after asking.
    fn use_picked_tool(
        &mut self,
        ui: &mut Ui,
        tool: Option<Tool>,
        pattern: &Pattern,
        scale: Scale,
        actions: &mut Vec<RollAction>,
    ) {
        let key_tool = if self.has_keyboard && !ui.ctx().text_edit_focused() {
            self.tool_from_keys(ui.ctx())
        } else {
            None
        };
        let tool = match tool {
            Some(Tool::Paste) => {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::RequestPaste);
                self.paste_requested = true;
                None
            }
            tool => {
                tool.or_else(|| std::mem::take(&mut self.paste_requested).then_some(Tool::Paste))
            }
        };
        if let Some(tool) = tool.or(key_tool) {
            self.use_tool(ui.ctx(), tool, pattern, scale, actions);
        }
    }

    /// The row of menus at the top: key, scale, what to draw, how long,
    /// and the tools. Returns the tool picked from the Tools menu. The key
    /// menus are `song`'s, the key the song starts in; `scale` is the
    /// pattern's, which differs where the song changes key.
    fn toolbar(
        &mut self,
        ui: &mut Ui,
        song: Scale,
        scale: Scale,
        alone: bool,
        actions: &mut Vec<RollAction>,
    ) -> Option<Tool> {
        let mut picked = None;
        ui.horizontal_wrapped(|ui| {
            // Each label stays whole and moves to the next line when it
            // does not fit, rather than being squeezed into a column.
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            ui.label("Key");
            let name = egui::RichText::new(song.spell(song.root).to_string());
            let tonic = twelve_tone::root_menu(ui, "scale-root", song, 64.0, name);
            if let Some(new) = twelve_tone::kind_menu(ui, "scale-kind", song, tonic, 120.0) {
                // Picking a row or a set opens the window to change it.
                match new.kind {
                    ScaleKind::Row(_) if song.kind.is_tonal() => self.row_window.open = true,
                    ScaleKind::Set(_) if song.kind.is_tonal() => self.set_window.open = true,
                    _ => {}
                }
                actions.push(RollAction::SetScale(new));
            }
            if scale != song {
                ui.label(egui::RichText::new(format!("This pattern: {}", scale.name())).strong())
                    .on_hover_text(
                        "The song changes key where this pattern plays (see the key names on \
                         the playlist's bar numbers), so it is highlighted in that key. The \
                         menus are the key the song starts in",
                    );
            }
            let names: Vec<String> = scale
                .pitch_classes()
                .map(|p| scale.spell(p).to_string())
                .collect();
            ui.weak(names.join(" "));
            self.snap_button(ui);
            ui.toggle_value(&mut self.row_window.open, "Row…")
                .on_hover_text(
                    "Twelve-tone rows: type, draw or pick a row, see all its forms in the \
                     matrix, and write them into the pattern",
                );
            ui.toggle_value(&mut self.set_window.open, "Sets…")
                .on_hover_text(
                    "Pitch-class sets: every group of notes there is, by Forte number, to \
                     draw with or use as a scale",
                );
            ui.toggle_value(&mut self.micromode_window.open, "Micromodes…")
                .on_hover_text(
                    "Kröpfl's micromodes: the twelve groups of three notes, the smallest sets, \
                     to hear, draw with or use as a key",
                );

            ui.separator();
            theme::keep_together(ui, 200.0, |ui| {
                ui.label("Draw");
                egui::ComboBox::from_id_salt("chord-stamp")
                    .width(150.0)
                    .truncate()
                    .selected_text(self.stamp.label())
                    .show_ui(ui, |ui| {
                        for stamp in ChordStamp::all() {
                            if stamp == ChordStamp::Fixed(ChordQuality::ALL[0]) {
                                ui.separator();
                            }
                            ui.selectable_value(&mut self.stamp, stamp, stamp.label());
                        }
                        ui.separator();
                        if let ChordStamp::Set(_) = self.stamp {
                            let _ = ui.selectable_label(true, self.stamp.label());
                        }
                        if ui.button("A set from the set browser…").clicked() {
                            self.set_window.open = true;
                        }
                    })
                    .response
                    .on_hover_text("What a click on the grid places: one note, or a whole chord.");
            });

            // A chord of n notes has n - 1 inversions. A single note has
            // none, but the list stays in its place, greyed out, so the
            // toolbar does not rearrange itself when a chord is picked.
            let chord_size = self.stamp.notes_at(Note::C4, scale).len();
            let chord = chord_size > 1;
            if chord {
                self.inversion = self.inversion.min(chord_size - 1);
            }
            theme::keep_together(ui, 90.0, |ui| {
                if !chord {
                    ui.disable();
                }
                egui::ComboBox::from_id_salt("inversion")
                    .width(76.0)
                    .selected_text(INVERSIONS_SHORT[if chord { self.inversion } else { 0 }])
                    .show_ui(ui, |ui| {
                        for (times, name) in INVERSIONS.iter().enumerate().take(chord_size) {
                            ui.selectable_value(&mut self.inversion, times, *name);
                        }
                    })
                    .response
                    .on_hover_text(
                        "Which note of the chord is lowest. Each inversion moves the \
                         lowest note up an octave, above the others. The note you click \
                         stays the root, and the chord is arranged around it.",
                    )
                    .on_disabled_hover_text(
                        "A single note has no inversions: pick a chord under Draw to \
                         choose which of its notes is lowest.",
                    );
            });

            // The label and its list wrap onto the next row together.
            theme::keep_together(ui, 140.0, |ui| {
                ui.label("Length");
                let length_name = NOTE_LENGTHS
                    .iter()
                    .find(|(steps, _)| *steps == self.note_length)
                    .map(|(_, name)| name.to_string())
                    .unwrap_or_else(|| format!("{} steps", self.note_length));
                egui::ComboBox::from_id_salt("note-length")
                    .width(76.0)
                    .selected_text(length_name)
                    .show_ui(ui, |ui| {
                        for (steps, name) in NOTE_LENGTHS {
                            ui.selectable_value(&mut self.note_length, steps, name);
                        }
                    });
            });

            ui.separator();
            if let Some(tool) = self.tools_menu(ui) {
                picked = Some(tool);
            }
            // The two boxes wrap onto the next row together: the check
            // takes the other tracks when their notes are shown.
            theme::keep_together(ui, 220.0, |ui| {
                ui.add_enabled(!alone, egui::Checkbox::new(&mut self.ghosts, "Ghost notes"))
                    .on_hover_text(
                        "Show the notes of the other channels faintly, and name the chords \
                         all of them make together.",
                    )
                    .on_disabled_hover_text("No other channel plays melodies or chords yet.");
                ui.checkbox(&mut self.voice_leading, "Voice leading")
                    .on_hover_text(VOICE_LEADING_HELP);
            });
        });
        picked
    }

    /// The Tools menu. Returns the tool picked, if any.
    fn tools_menu(&mut self, ui: &mut Ui) -> Option<Tool> {
        let ctx = ui.ctx().clone();
        let mut picked = None;
        // Menus normally close on any click. This one has choices for the
        // arpeggio, so it stays open until a tool is used or you click
        // somewhere else.
        let stay_open = egui::containers::menu::MenuConfig::new()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
        let menu = egui::containers::menu::MenuButton::new("Tools").config(stay_open);
        menu.ui(ui, |ui| {
            crate::widgets::scroll_menu(ui, |ui| {
                ui.weak("Changes the selected notes, or every note if none is selected.");
                let mut item =
                    |ui: &mut Ui, text: &str, tool: Tool, shortcut: Option<KeyboardShortcut>| {
                        let mut button = egui::Button::new(text);
                        if let Some(shortcut) = shortcut {
                            button = button.shortcut_text(ctx.format_shortcut(&shortcut));
                        }
                        if ui.add(button).clicked() {
                            picked = Some(tool);
                            ui.close();
                        }
                    };
                item(ui, "Up a semitone", Tool::Transpose(1), Some(KEY_UP));
                item(ui, "Down a semitone", Tool::Transpose(-1), Some(KEY_DOWN));
                item(ui, "Up an octave", Tool::Transpose(12), Some(OCTAVE_UP));
                item(
                    ui,
                    "Down an octave",
                    Tool::Transpose(-12),
                    Some(OCTAVE_DOWN),
                );
                item(ui, "Up a note in the key", Tool::InKey(1), Some(IN_KEY_UP));
                item(
                    ui,
                    "Down a note in the key",
                    Tool::InKey(-1),
                    Some(IN_KEY_DOWN),
                );
                ui.separator();
                item(ui, "A step earlier", Tool::Shift(-1), Some(KEY_LEFT));
                item(ui, "A step later", Tool::Shift(1), Some(KEY_RIGHT));
                item(ui, "A beat earlier", Tool::Shift(-4), Some(BEAT_LEFT));
                item(ui, "A beat later", Tool::Shift(4), Some(BEAT_RIGHT));
                item(ui, "Quantize to the steps", Tool::Quantize, None);
                ui.separator();
                item(ui, "Snap to the key", Tool::SnapToKey, None);
                item(ui, "Invert chords up", Tool::Invert { up: true }, None);
                item(ui, "Invert chords down", Tool::Invert { up: false }, None);
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Arpeggio");
                    for order in ArpeggioOrder::ALL {
                        ui.selectable_value(&mut self.arpeggio, order, order.name());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Each note");
                    for (steps, name) in ARPEGGIO_RATES {
                        ui.selectable_value(&mut self.arpeggio_rate, steps, name);
                    }
                });
                item(ui, "Arpeggiate chords", Tool::Arpeggiate, None);
                ui.separator();
                item(ui, "Select all", Tool::SelectAll, Some(SELECT_ALL));
                for (label, key, tool) in [
                    ("Copy", Key::C, Tool::Copy),
                    ("Cut", Key::X, Tool::Cut),
                    ("Paste", Key::V, Tool::Paste),
                    ("Duplicate", Key::D, Tool::Duplicate),
                ] {
                    item(
                        ui,
                        label,
                        tool,
                        Some(KeyboardShortcut::new(Modifiers::COMMAND, key)),
                    );
                }
                item(ui, "Delete selected notes", Tool::Delete, Some(DELETE));
            });
        });
        picked
    }

    /// Snap to scale: a click turns it on and off, and a
    /// right click opens its menu, which can also snap the notes played on
    /// the keyboards.
    fn snap_button(&mut self, ui: &mut Ui) {
        ui.toggle_value(&mut self.snap_to_scale, "Snap to scale")
            .on_hover_text(
                "Notes you draw or move land on the notes of the key. Notes already \
                 there stay put until you move them: to snap them all, select them \
                 (Ctrl+A) and drag them up a semitone and back. Right-click for more.",
            )
            .context_menu(|ui| {
                ui.checkbox(&mut self.snap_to_scale, "Snap notes you draw or move");
                ui.checkbox(&mut self.snap_played, "Snap notes you play")
                    .on_hover_text(
                        "Keys played on the typing keyboard or a MIDI keyboard sound, \
                         and are recorded, as the nearest note of the key.",
                    );
            });
    }

    /// Buttons for the seven chords of the key, to hear how they sound.
    /// The chords that often follow `current` (the chord held on a
    /// keyboard, or at the last step pointed at) are highlighted.
    fn chords_in_key(
        &self,
        ui: &mut Ui,
        scale: Scale,
        current: Option<(ChordAt, Chord, PitchClass)>,
        compact: bool,
        actions: &mut Vec<RollAction>,
    ) -> String {
        if let Some(groups) = keys::group_keys(scale, ChordSize::Triads) {
            self.groups_in_key(ui, scale, &groups, actions);
            return String::new();
        }
        if !scale.is_heptatonic() {
            return String::new();
        }
        let suggestions = current
            .map(|(_, chord, bass)| next_chords_over(scale, chord, bass))
            .unwrap_or_default();
        ui.horizontal_wrapped(|ui| {
            ui.label(format!("Chords in {}", scale.name()));
            for degree in 0..7 {
                let Some(chord) = scale.triad(degree) else {
                    continue;
                };
                let numeral = scale.roman_numeral(chord).unwrap_or_default();
                let text = format!("{numeral} · {}", scale.chord_name(chord));
                let full_name = format!("{} {}", scale.spell_root(chord), chord.quality.name());
                let mut hint = match self.hints {
                    HintLevel::Detailed => format!(
                        "{full_name}. Click to hear it. Pick \"Chord in key\" under Draw to place chords like it."
                    ),
                    _ => format!("{full_name}. Click to hear it."),
                };
                let suggestion = suggestions.iter().find(|s| s.degree == degree);
                if let Some(suggestion) = suggestion {
                    // In minor keys the suggested V is major, while the
                    // key's own chord on that degree is minor.
                    let played_as = if suggestion.chord == chord {
                        String::new()
                    } else {
                        format!(", played as {}", scale.chord_name(suggestion.chord))
                    };
                    hint = match self.hints {
                        HintLevel::Detailed => format!(
                            "Often comes next{played_as}. {}\n\n{hint}",
                            suggestion.reason
                        ),
                        HintLevel::Short => {
                            format!("Often next{played_as}: {}.\n\n{hint}", suggestion.short)
                        }
                        HintLevel::Minimal => format!("Often next{played_as}.\n\n{hint}"),
                    };
                }
                let button = egui::Button::new(text).selected(suggestion.is_some());
                if ui.add(button).on_hover_text(hint).clicked() {
                    let root = Note::new(Note::C4.midi() + chord.root.semitone());
                    actions.push(RollAction::Preview {
                        channel: self.channel,
                        pitches: chord.notes_from(root),
                    });
                }
            }
        });
        let hint = match current {
            None => match self.hints {
                HintLevel::Detailed => "Point at a chord in the grid, or hold one on a keyboard, \
                                        to see which chords often follow it."
                    .to_string(),
                HintLevel::Short => {
                    "Point at or hold a chord to see what often follows.".to_string()
                }
                HintLevel::Minimal => String::new(),
            },
            Some((at, chord, bass)) => {
                // Named as it sounds, inversion and all: "C/E (I⁶)".
                let name = Harmony::Chord { chord, bass }
                    .label_in(scale)
                    .unwrap_or_else(|| scale.chord_name(chord));
                let place = match at {
                    ChordAt::Step(step) => format!("at step {}", step + 1),
                    ChordAt::Keys => "on the keys".to_string(),
                };
                let numeral = scale
                    .figured_numeral(chord, bass)
                    .map(|numeral| format!(" ({numeral})"))
                    .unwrap_or_default();
                let next: Vec<String> = suggestions
                    .iter()
                    .map(|s| {
                        let numeral = scale.roman_numeral(s.chord).unwrap_or_default();
                        format!("{} ({numeral})", scale.chord_name(s.chord))
                    })
                    .collect();
                let major_or_minor = keys::has_functions(scale.kind);
                // Why nothing is suggested: the root is not in the key, the
                // chord is not one of the key's own, or nothing usually
                // follows it.
                let why = if !scale.contains(chord.root) {
                    "has a root outside the key"
                } else if !scale.is_chord_of_key(chord) {
                    "is not one of the key's chords"
                } else {
                    "has no usual next chord here"
                };
                match (self.hints, next.is_empty(), major_or_minor) {
                    (HintLevel::Minimal, true, true) => format!("{name}: {why}"),
                    (_, true, true) => format!("{name} {place} {why}."),
                    (HintLevel::Minimal, true, false) => String::new(),
                    (_, true, false) => {
                        "Next-chord hints are given in major and minor keys.".to_string()
                    }
                    (HintLevel::Detailed, false, _) => format!(
                        "After {name}{numeral} {place}, try: {}. Hover the highlighted chords to see why.",
                        next.join(", ")
                    ),
                    (HintLevel::Short, false, _) => {
                        format!("After {name}{numeral}, try: {}.", next.join(", "))
                    }
                    (HintLevel::Minimal, false, _) => format!("Next: {}", next.join(", ")),
                }
            }
        };
        if compact {
            return hint;
        }
        if !hint.is_empty() {
            ui.weak(hint);
        }
        String::new()
    }

    /// For a row or a set, which have no chords of the key: buttons for
    /// its groups of notes, the row's trichords or the chords stacked
    /// from the set, to hear how they sound.
    fn groups_in_key(
        &self,
        ui: &mut Ui,
        scale: Scale,
        groups: &[keys::GroupKey],
        actions: &mut Vec<RollAction>,
    ) {
        ui.horizontal_wrapped(|ui| {
            let what = match scale.kind {
                ScaleKind::Row(_) => format!("Trichords of {}", scale.name()),
                _ => format!("Chords of {}", scale.name()),
            };
            ui.label(what);
            for group in groups {
                let names: Vec<String> = group.names.iter().map(ToString::to_string).collect();
                let class = group.pitch_set().set_class();
                let text = format!("{} · {}", group.label, names.join(" "));
                let hover = format!(
                    "{}: set {}. Click to hear it.",
                    names.join(" "),
                    class.describe()
                );
                if ui.button(text).on_hover_text(hover).clicked() {
                    actions.push(RollAction::Preview {
                        channel: self.channel,
                        pitches: group.play(),
                    });
                }
            }
        });
    }

    /// The strip above the grid that names the chord at every step, in
    /// `rect`, the place taken for it: on top the chord of this track's
    /// own notes, below it the harmony of all the tracks playing
    /// together, heard beat by beat (see
    /// [`Project::hearing_with_pattern`]): an arpeggio is its chord, and
    /// two chords at once or a set of notes are named so. Each is named
    /// with its bass after a slash when it is an inversion: a piano
    /// playing C/E over a bass playing C is C/E on the track and C for
    /// all. With the voice leading checked, a third row marks what the
    /// check finds, after a row with the forms of the row in a
    /// twelve-tone key.
    fn chord_lane(
        &mut self,
        ui: &mut Ui,
        (rect, response): (Rect, &Response),
        drawing: Drawing,
        lanes: Lanes,
    ) {
        let Drawing {
            project,
            pattern,
            scale,
            playhead,
            voices,
            ..
        } = drawing;
        let length = lanes.length;
        let track_name = project
            .channels()
            .get(self.channel)
            .map_or("Track", |channel| channel.name.as_str());
        let channel = [self.channel];
        let rows: [(&str, Vec<Harmony>); 2] = [
            (
                track_name,
                (0..length)
                    .map(|step| analyze(sounding(pattern, &channel, step)))
                    .collect(),
            ),
            ("All", heard_by_beat(project, self.pattern, length)),
        ];
        let row_height = LANE_HEIGHT / rows.len() as f32;
        let row = |index: usize| {
            let top = rect.top() + row_height * index as f32;
            Rect::from_min_size(pos2(rect.left(), top), vec2(rect.width(), row_height))
        };
        for (index, (name, harmonies)) in rows.iter().enumerate() {
            self.chord_row(
                ui,
                row(index),
                response,
                name,
                harmonies,
                scale,
                lanes,
                playhead,
            );
        }
        if let Some(check) = voices {
            let mut below = rows.len();
            if check.rules == Rules::TwelveTone {
                self.form_row(ui, row(below), response, drawing, check, lanes);
                below += 1;
            }
            self.slip_row(ui, row(below), response, drawing, check, lanes);
        }
    }

    /// The chord lane's row for the forms of a twelve-tone row that the
    /// voice-leading check reads in the notes: a block over each, named
    /// by its label ("P4", "RI11"). Pointing at one lists its notes.
    fn form_row(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        response: &Response,
        drawing: Drawing,
        check: &VoiceLeading,
        lanes: Lanes,
    ) {
        let lane = lanes.grid(rect, self.time);
        let cells = lane.cells();
        let pointed = response
            .hover_pos()
            .filter(|pos| rect.contains(*pos) && cells.contains(*pos))
            .map(|pos| lane.cell_at(pos).0);
        if pointed.is_some() {
            self.hovered_step = pointed;
        }

        let painter = ui.painter_at(rect);
        paint_lane_name(ui, &painter, rect, "Row");
        painter.rect_filled(cells, 2.0, LANE_BACKGROUND);
        let painter = painter.with_clip_rect(cells);
        // The forms the track shown plays: its own, or those running
        // through all the tracks.
        let shown = check
            .rows
            .iter()
            .filter(|row| row.channel.is_none_or(|channel| channel == self.channel));
        for statement in shown.clone() {
            let block = Rect::from_min_max(
                pos2(lane.x(statement.start), lane.rect.top()),
                pos2(lane.x(statement.end), lane.rect.bottom()),
            )
            .shrink2(vec2(1.0, 1.0));
            painter.rect_filled(block, 3.0, LANE_ROW_FORM);
            let label = match statement.before {
                0 => statement.label.to_string(),
                _ => format!("…{}", statement.label),
            };
            let name = painter.layout_no_wrap(label, FontId::proportional(11.0), LANE_TEXT);
            if name.size().x <= block.width() - 4.0 {
                painter.with_clip_rect(block).galley(
                    pos2(block.left() + 4.0, block.center().y - name.size().y / 2.0),
                    name,
                    LANE_TEXT,
                );
            }
        }
        if let Some(step) = pointed {
            let track = |channel: usize| {
                drawing
                    .project
                    .channels()
                    .get(channel)
                    .map_or("Track", |channel| channel.name.as_str())
                    .to_string()
            };
            let found: Vec<String> = shown
                .filter(|row| (row.start..row.end).contains(&step))
                .map(|row| match row.channel {
                    Some(channel) => format!("{} plays {}", track(channel), row.describe()),
                    None => row.describe(),
                })
                .collect();
            if !found.is_empty() {
                response
                    .clone()
                    .on_hover_text_at_pointer(found.join("\n\n"));
            }
        }
    }

    /// The chord lane's row for the voice-leading check: a block over
    /// the steps of every move classical part writing avoids, or every
    /// note twelve-tone writing avoids, named in a few letters ("5ths",
    /// "LT", "order"). Pointing at one explains it.
    fn slip_row(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        response: &Response,
        drawing: Drawing,
        check: &VoiceLeading,
        lanes: Lanes,
    ) {
        let length = lanes.length;
        let lane = lanes.grid(rect, self.time);
        let cells = lane.cells();
        let pointed = response
            .hover_pos()
            .filter(|pos| rect.contains(*pos) && cells.contains(*pos))
            .map(|pos| lane.cell_at(pos).0);
        if pointed.is_some() {
            self.hovered_step = pointed;
        }

        let painter = ui.painter_at(rect);
        paint_lane_name(ui, &painter, rect, "Voices");
        painter.rect_filled(cells, 2.0, LANE_BACKGROUND);
        let painter = painter.with_clip_rect(cells);
        // What is found on each step, so that a run of steps with the
        // same findings makes one block.
        let marks: Vec<Vec<&str>> = (0..length)
            .map(|step| {
                let mut marks: Vec<&str> = check.slips_at(step).map(|s| s.kind.short()).collect();
                marks.sort_unstable();
                marks.dedup();
                marks
            })
            .collect();
        let mut start = 0;
        for run in marks.chunk_by(|a, b| a == b) {
            let end = start + run.len();
            if !run[0].is_empty() {
                let block = Rect::from_min_max(
                    pos2(lane.x(start), lane.rect.top()),
                    pos2(lane.x(end), lane.rect.bottom()),
                )
                .shrink2(vec2(1.0, 1.0));
                painter.rect_filled(block, 3.0, LANE_SLIP);
                let name =
                    painter.layout_no_wrap(run[0].join(" "), FontId::proportional(11.0), LANE_TEXT);
                if name.size().x <= block.width() - 2.0 {
                    let shown = block.intersect(cells);
                    painter.with_clip_rect(block).galley(
                        shown.center() - name.size() / 2.0,
                        name,
                        LANE_TEXT,
                    );
                }
            }
            start = end;
        }
        if let Some(step) = pointed {
            let name = |voice: usize| voice_name(drawing.project, check, voice);
            let found: Vec<String> = check.slips_at(step).map(|s| s.describe(name)).collect();
            if !found.is_empty() {
                response
                    .clone()
                    .on_hover_text_at_pointer(found.join("\n\n"));
            }
        }
    }

    /// One row of the chord lane: `name` where the keyboard is, and the
    /// chord at every step, runs of the same harmony joined into one
    /// block. Two chords without a name both read "?", but stay two
    /// blocks.
    #[allow(clippy::too_many_arguments)]
    fn chord_row(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        response: &Response,
        name: &str,
        harmonies: &[Harmony],
        scale: Scale,
        lanes: Lanes,
        playhead: Option<usize>,
    ) {
        let length = lanes.length;
        let lane = lanes.grid(rect, self.time);
        let cells = lane.cells();
        if let Some(pos) = response.hover_pos()
            && cells.contains(pos)
        {
            self.hovered_step = Some(lane.cell_at(pos).0);
        }

        let painter = ui.painter_at(rect);
        paint_lane_name(ui, &painter, rect, name);
        painter.rect_filled(cells, 2.0, LANE_BACKGROUND);
        let painter = painter.with_clip_rect(cells);

        // `chunk_by` splits a slice wherever two neighbours differ. A
        // harmony holds its bass, so C and C/E are two blocks.
        let mut start = 0;
        for run in harmonies.chunk_by(|a, b| a == b) {
            let end = start + run.len();
            if let Some(label) = run[0].label_in(scale) {
                let chord_rect = Rect::from_min_max(
                    pos2(lane.x(start), lane.rect.top()),
                    pos2(lane.x(end), lane.rect.bottom()),
                )
                .shrink2(vec2(1.0, 1.0));
                painter.rect_filled(chord_rect, 3.0, LANE_CHORD);
                // A name too long for its block is left out: cut short,
                // "E7/B" would read "7/", another chord. Pointing at the
                // block names it in full, under the toolbar.
                let name = painter.layout_no_wrap(label, FontId::proportional(11.0), LANE_TEXT);
                if name.size().x <= chord_rect.width() - 2.0 {
                    // A long chord's name stays in view while it scrolls.
                    let shown = chord_rect.intersect(cells);
                    painter.with_clip_rect(chord_rect).galley(
                        shown.center() - name.size() / 2.0,
                        name,
                        LANE_TEXT,
                    );
                }
            }
            start = end;
        }

        if let Some(step) = playhead.filter(|&step| step < length) {
            let x = lane.x(step);
            painter.rect_filled(
                Rect::from_min_max(
                    pos2(x, lane.rect.top()),
                    pos2(x + lanes.step_width, lane.rect.bottom()),
                ),
                0.0,
                PLAYHEAD,
            );
        }
    }

    /// The keyboard and the note grid, inside the scroll area. Returns how
    /// far the grid is scrolled sideways, in pixels.
    fn grid(
        &mut self,
        ui: &mut Ui,
        pattern: &Pattern,
        ghosts: &[usize],
        view: View,
        actions: &mut Vec<RollAction>,
    ) -> f32 {
        let View {
            scale,
            step_width,
            playhead,
            held,
            roles,
            voices,
        } = view;
        let length = pattern.length();
        let rows = (HIGHEST - LOWEST + 1) as f32;
        let row_height = ROW_HEIGHT * self.zoom.y;
        let size = vec2(KEY_WIDTH + step_width * length as f32, rows * row_height);
        let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());
        let visible = ui.clip_rect().intersect(rect);
        let grid = Grid {
            rect: Rect::from_min_max(pos2(rect.left() + KEY_WIDTH, rect.top()), rect.max),
            step_width,
            row_height,
            length,
            time: self.time,
            keys_left: visible.left(),
        };

        let notes = pattern.notes(self.channel);
        // Whether the view jumps this frame. It moves only after the grid
        // is drawn, so what is in view now is not worth remembering.
        let jumping = self.scroll_to_notes;
        if self.scroll_to_notes {
            // Centre the view between the lowest and highest note, or on
            // middle C when there are no notes yet, from the first step.
            // The middle is that of the rows from the highest to the
            // lowest: halving pitches would be a row off when they span an
            // odd number of semitones.
            let pitches = || notes.iter().map(|note| note.pitch.midi());
            let lowest = pitches().min().unwrap_or(Note::C4.midi());
            let highest = pitches().max().unwrap_or(Note::C4.midi());
            let top = grid.row_top(highest.clamp(LOWEST, HIGHEST));
            let bottom = grid.row_top(lowest.clamp(LOWEST, HIGHEST)) + row_height;
            let target =
                Rect::from_min_max(pos2(rect.left(), top), pos2(rect.left() + 1.0, bottom));
            ui.scroll_to_rect(target, Some(Align::Center));
            self.scroll_to_notes = false;
        }
        if !jumping {
            self.in_view = Some((
                grid.cell_at(visible.left_bottom() - vec2(0.0, 1.0))
                    .1
                    .midi(),
                grid.cell_at(visible.left_top()).1.midi(),
            ));
        }
        let hit = self.handle_input(ui, &response, grid, pattern, scale, actions);

        let painter = ui.painter_at(visible);
        paint_rows(&painter, grid, scale);
        paint_step_lines(&painter, grid);
        if notes.is_empty() {
            painter.text(
                visible.center() + vec2(KEY_WIDTH / 2.0, 0.0),
                Align2::CENTER_CENTER,
                "No notes here yet: click to draw one",
                FontId::proportional(18.0),
                NO_NOTES_TEXT,
            );
        }
        if let Some(step) = playhead.filter(|&step| step < length) {
            let x = grid.x(step);
            let column = Rect::from_min_max(
                pos2(x, grid.rect.top()),
                pos2(x + step_width, grid.rect.bottom()),
            );
            painter.rect_filled(column, 0.0, PLAYHEAD);
        }
        // The other channels first, so this channel's notes are drawn on top.
        for &other in ghosts {
            for note in pattern
                .notes(other)
                .iter()
                .filter(|note| note.start < length)
            {
                let rect = grid.note_rect(note).shrink2(vec2(0.5, 1.5));
                painter.rect_filled(rect, 2.0, OTHER_CHANNEL);
                painter.rect_stroke(
                    rect,
                    2.0,
                    Stroke::new(1.0, OTHER_CHANNEL_BORDER),
                    StrokeKind::Inside,
                );
            }
        }
        self.paint_notes(&painter, grid, notes, scale, roles);
        if let Some(check) = voices {
            paint_parts(&painter, grid, check, self.hovered_step.or(playhead));
            paint_slips(&painter, grid, check, self.hovered_step.or(playhead));
        }

        // Show what a click would place: an outline of the note or chord.
        let ctrl = ui.input(|input| input.modifiers.command);
        if self.drag.is_none()
            && self.group_drag.is_none()
            && self.selection_box.is_none()
            && hit.is_none()
            && !ctrl
            && let Some(pos) = response.hover_pos()
            && grid.cells().contains(pos)
        {
            let (step, pitch) = grid.cell_at(pos);
            for stamp_pitch in self.stamp_notes(self.snapped(pitch, scale), scale) {
                let stamp = NoteEvent::new(stamp_pitch, step, self.note_length);
                let stamp_rect = grid.note_rect(&stamp).shrink2(vec2(0.5, 1.0));
                painter.rect_filled(stamp_rect, 2.0, STAMP_GHOST);
                painter.rect_stroke(stamp_rect, 2.0, Stroke::new(1.0, NOTE), StrokeKind::Inside);
            }
        }

        if let Some(origin) = self.selection_box
            && let Some(pos) = ui.input(|input| input.pointer.interact_pos())
        {
            let area = Rect::from_two_pos(origin, pos);
            painter.rect_filled(area, 0.0, SELECTION_BOX);
            painter.rect_stroke(
                area,
                0.0,
                Stroke::new(1.0, SELECTED_BORDER),
                StrokeKind::Inside,
            );
        }
        // The keyboard goes on top, since the grid scrolls under it.
        paint_keys(&painter, grid, scale, held);
        visible.left() - rect.left()
    }

    /// The notes a click on `pitch` places, in the inversion chosen.
    ///
    /// Inverting moves the clicked note (the root) up an octave, which
    /// would put the whole chord above the pointer. So the chord moves
    /// back down an octave, and the root stays where you clicked, with
    /// the other notes around it.
    fn stamp_notes(&self, pitch: Note, scale: Scale) -> Vec<Note> {
        let notes = tools::inversion(&self.stamp.notes_at(pitch, scale), self.inversion);
        if notes.contains(&pitch) {
            return notes;
        }
        notes
            .iter()
            .map(|note| note.transpose(-12))
            .collect::<Option<Vec<Note>>>()
            .unwrap_or(notes)
    }

    /// Reads the mouse for the grid and the keyboard. Returns the index of
    /// the note under the pointer, if any.
    ///
    /// - left click on a key: hear it
    /// - left click on empty grid: add a note (or chord); keep the button
    ///   down to drag the new note
    /// - left drag a note: move it; drag its right end: resize it
    /// - left drag a selected note: move all the selected notes
    /// - Ctrl and drag: select the notes in a box; Ctrl and click a note:
    ///   add it to the selection, or take it out
    /// - right click (or right drag over) notes: delete them
    fn handle_input(
        &mut self,
        ui: &Ui,
        response: &Response,
        grid: Grid,
        pattern: &Pattern,
        scale: Scale,
        actions: &mut Vec<RollAction>,
    ) -> Option<usize> {
        let channel = self.channel;
        let notes = pattern.notes(channel);
        let dismissing = crate::widgets::dismissing(ui.ctx());
        let (primary_pressed, primary_down, secondary, pointer, press_origin, ctrl) =
            ui.input(|input| {
                let pointer = &input.pointer;
                (
                    pointer.primary_pressed(),
                    pointer.primary_down(),
                    // A quick click can press and release within one frame,
                    // so "pressed this frame" counts as well as "held down".
                    pointer.secondary_pressed() || pointer.secondary_down(),
                    pointer.interact_pos(),
                    crate::widgets::press_pos(input, Some(egui::PointerButton::Primary)),
                    input.modifiers.command,
                )
            });

        // On the frame the button goes down, look where it went down: the
        // pointer may already have moved on, if the hand was quick.
        let hovered = match response.hover_pos() {
            Some(hover) if primary_pressed => press_origin.or(Some(hover)),
            hover => hover,
        };
        // The "no notes here" message lies over the grid, and a click that
        // closes a menu does nothing else.
        let hovered = hovered
            .filter(|pos| !self.banner.is_some_and(|banner| banner.contains(*pos)))
            .filter(|_| !dismissing);
        let hit = hovered
            .filter(|pos| grid.cells().contains(*pos))
            .and_then(|pos| note_at(notes, grid, pos));

        if let Some(pos) = hovered {
            if grid.keys().contains(pos) && primary_pressed {
                let (_, pitch) = grid.cell_at(pos);
                actions.push(RollAction::Preview {
                    channel,
                    pitches: vec![pitch],
                });
            } else if grid.cells().contains(pos) {
                let (step, pitch) = grid.cell_at(pos);
                self.hovered_step = Some(step);

                if primary_pressed && ctrl {
                    match hit {
                        Some(index) => {
                            let note = notes[index];
                            if let Some(at) = self.selection.iter().position(|&n| n == note) {
                                self.selection.remove(at);
                            } else {
                                self.selection.push(note);
                            }
                        }
                        None => {
                            self.selection.clear();
                            self.selection_box = Some(pos);
                        }
                    }
                } else if primary_pressed {
                    match hit {
                        Some(index) => {
                            let note = notes[index];
                            // A short note keeps most of its length for
                            // moving it.
                            let rect = grid.note_rect(&note);
                            let grab = EDGE_GRAB.min(rect.width() / 3.0);
                            let near_end = pos.x > rect.right() - grab;
                            let selected = self.selection.contains(&note);
                            if selected && !near_end {
                                self.group_drag = Some(GroupDrag {
                                    from_step: step,
                                    from_pitch: pitch,
                                    originals: self.selection.clone(),
                                    moved: self.selection.clone(),
                                    edited: false,
                                });
                            } else {
                                if !selected {
                                    self.selection.clear();
                                    self.selection.push(note);
                                }
                                self.drag = Some(if near_end {
                                    Drag::Resize { index }
                                } else {
                                    Drag::Move {
                                        index,
                                        grab: step.saturating_sub(note.start),
                                    }
                                });
                            }
                            actions.push(RollAction::Preview {
                                channel,
                                pitches: vec![note.pitch],
                            });
                        }
                        None => {
                            // A drawn note is not selected, so the tools and
                            // the arrow keys still change every note until
                            // you click one.
                            self.selection.clear();
                            let pitches = self.stamp_notes(self.snapped(pitch, scale), scale);
                            let new_notes: Vec<NoteEvent> = pitches
                                .iter()
                                .map(|&p| NoteEvent::new(p, step, self.note_length))
                                .collect();
                            // A single new note can be dragged straight
                            // away. It will be added at the end of the list.
                            if new_notes.len() == 1 {
                                self.drag = Some(Drag::Move {
                                    index: notes.len(),
                                    grab: 0,
                                });
                            }
                            actions.push(RollAction::Preview { channel, pitches });
                            actions.push(RollAction::AddNotes {
                                channel,
                                notes: new_notes,
                            });
                        }
                    }
                } else if secondary && let Some(index) = hit {
                    actions.push(RollAction::RemoveNote { channel, index });
                }
            }
        }

        if !primary_down {
            self.stop_dragging();
        }

        // A selection box selects every note it touches.
        if let Some(origin) = self.selection_box
            && let Some(pos) = pointer
        {
            let area = Rect::from_two_pos(origin, pos);
            self.selection = notes
                .iter()
                .filter(|note| note.start < grid.length && grid.note_rect(note).intersects(area))
                .copied()
                .collect();
        }

        // Moving the selection: every note keeps its distance from the
        // cell the drag started on.
        if let Some(mut drag) = self.group_drag.take()
            && let Some(pos) = pointer
        {
            let (step, pitch) = grid.cell_at(pos);
            let steps = step as isize - drag.from_step as isize;
            let semitones = i32::from(pitch.midi()) - i32::from(drag.from_pitch.midi());
            drag.edited |= steps != 0 || semitones != 0;
            let snap = self.snap_to_scale && drag.edited;
            let new = tools::shift(&drag.originals, steps, grid.length)
                .and_then(|shifted| tools::transpose(&shifted, semitones))
                .map(|moved| match snap {
                    true => tools::snap_to_scale(&moved, scale),
                    false => moved,
                })
                .filter(|moved| fits_view(moved));
            if let Some(new) = new
                && new != drag.moved
            {
                let pitch_changed =
                    new.first().map(|n| n.pitch) != drag.moved.first().map(|n| n.pitch);
                if let Some(moved) =
                    self.replace(pattern, actions, "move notes", drag.moved.clone(), new)
                {
                    if pitch_changed {
                        actions.push(RollAction::Preview {
                            channel,
                            pitches: moved.iter().map(|note| note.pitch).collect(),
                        });
                    }
                    drag.moved = moved;
                }
            }
            self.group_drag = Some(drag);
        }

        // The drag follows the pointer even outside the grid.
        if let Some(drag) = self.drag
            && let Some(pos) = pointer
            && let Some(&note) = notes.get(drag.index())
        {
            let (step, pitch) = grid.cell_at(pos);
            let pitch = self.snapped(pitch, scale);
            let changed = match drag {
                Drag::Move { grab, .. } => NoteEvent {
                    pitch,
                    start: step.saturating_sub(grab),
                    ..note
                },
                // A resized note ends on the grid, where the pointer is.
                Drag::Resize { .. } => {
                    let length = (step + 1).saturating_sub(note.start).max(1);
                    self.note_length = length;
                    let trim = if length == note.length { note.trim } else { 0 };
                    NoteEvent {
                        length,
                        trim,
                        ..note
                    }
                }
            };
            if changed != note {
                if changed.pitch != note.pitch {
                    actions.push(RollAction::Preview {
                        channel,
                        pitches: vec![changed.pitch],
                    });
                }
                // A resized note that was selected stays selected.
                if let Some(selected) = self.selection.iter_mut().find(|n| **n == note) {
                    *selected = changed;
                }
                actions.push(RollAction::ReplaceNote {
                    channel,
                    index: drag.index(),
                    note: changed,
                });
            }
        }
        hit
    }

    /// The strip under the grid that shows how hard each note is played.
    /// Click or drag in it to set the velocity of the notes on a step
    /// (only the selected ones, if any notes are selected).
    fn velocity_lane(
        &mut self,
        ui: &mut Ui,
        pattern: &Pattern,
        lanes: Lanes,
        actions: &mut Vec<RollAction>,
    ) {
        let length = lanes.length;
        let step_width = lanes.step_width;
        let (rect, response) = ui.allocate_exact_size(
            vec2(ui.available_width(), VELOCITY_HEIGHT),
            Sense::click_and_drag(),
        );
        let lane = lanes.grid(rect, self.time);
        let cells = lane.cells();
        let notes = pattern.notes(self.channel);

        if response.is_pointer_button_down_on()
            && !crate::widgets::dismissing(ui.ctx())
            && let Some(pos) = response.interact_pointer_pos()
            && (cells.contains(pos) || response.dragged())
        {
            let step = lane.step_at(pos.x);
            self.hovered_step = Some(step);
            let height = (lane.rect.bottom() - pos.y) / lane.rect.height();
            let velocity = (height * 127.0).round().clamp(1.0, 127.0) as u8;
            let selected_here = notes
                .iter()
                .any(|note| note.start == step && self.selection.contains(note));
            let old: Vec<NoteEvent> = notes
                .iter()
                .filter(|note| note.start == step && note.velocity != velocity)
                .filter(|note| !selected_here || self.selection.contains(note))
                .copied()
                .collect();
            let new = old
                .iter()
                .map(|&note| NoteEvent { velocity, ..note })
                .collect();
            self.replace(pattern, actions, "change velocity", old, new);
        } else if let Some(pos) = response.hover_pos() {
            self.hovered_step = Some(lane.step_at(pos.x));
        }

        let painter = ui.painter_at(rect);
        painter.text(
            pos2(rect.left() + 4.0, rect.top() + 8.0),
            Align2::LEFT_CENTER,
            "Velocity",
            FontId::proportional(11.0),
            ui.visuals().weak_text_color(),
        );
        painter.rect_filled(cells, 2.0, LANE_BACKGROUND);
        let painter = painter.with_clip_rect(cells);
        for step in (0..=length).filter(|&step| lane.time.is_beat_start(step)) {
            let color = if lane.time.is_bar_start(step) {
                BAR_LINE
            } else {
                STEP_LINE
            };
            painter.vline(lane.x(step), lane.rect.y_range(), Stroke::new(1.0, color));
        }
        let bar_width = (step_width * 0.4).clamp(2.0, 8.0);
        for note in notes.iter().filter(|note| note.start < length) {
            let height = lane.rect.height() * f32::from(note.velocity) / 127.0;
            let nudge = f32::from(note.nudge) * step_width / TICKS_PER_STEP as f32;
            let left = lane.x(note.start) + nudge + 2.0;
            let bar = Rect::from_min_max(
                pos2(left, lane.rect.bottom() - height),
                pos2(left + bar_width, lane.rect.bottom()),
            );
            let color = if self.selection.contains(note) {
                SELECTED_BORDER
            } else {
                NOTE
            };
            painter.rect_filled(bar, 1.0, color.gamma_multiply(0.85));
            painter.circle_filled(pos2(bar.center().x, bar.top()), bar_width * 0.6, color);
        }
    }

    /// Replaces `old` with `new` (tidied first) if the pattern allows it,
    /// keeps the selection on the same notes, and returns the notes as
    /// they now are. Returns `None`, doing nothing, if it is not allowed.
    fn replace(
        &mut self,
        pattern: &Pattern,
        actions: &mut Vec<RollAction>,
        label: &'static str,
        old: Vec<NoteEvent>,
        new: Vec<NoteEvent>,
    ) -> Option<Vec<NoteEvent>> {
        let new = tidy_notes(new);
        if old == new || !pattern.can_replace_notes(self.channel, &old, &new) {
            return None;
        }
        let was_selected = old.iter().any(|note| self.selection.contains(note));
        self.selection.retain(|note| !old.contains(note));
        if was_selected {
            self.selection.extend(&new);
        }
        actions.push(RollAction::ReplaceNotes {
            channel: self.channel,
            old,
            new: new.clone(),
            label,
        });
        Some(new)
    }

    /// The tool asked for with a key, if any.
    fn tool_from_keys(&self, ctx: &egui::Context) -> Option<Tool> {
        if ctx.input(|input| input.events.contains(&egui::Event::Copy)) {
            return Some(Tool::Copy);
        }
        if ctx.input(|input| input.events.contains(&egui::Event::Cut)) {
            return Some(Tool::Cut);
        }
        if crate::clipboard::paste::<Vec<NoteEvent>>(ctx, "notes").is_some() {
            return Some(Tool::Paste);
        }
        let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
        // egui counts Shift+↑ and Alt+↑ as ↑ too, so the longer shortcuts
        // are checked first.
        let keys = [
            (IN_KEY_UP, Tool::InKey(1)),
            (IN_KEY_DOWN, Tool::InKey(-1)),
            (OCTAVE_UP, Tool::Transpose(12)),
            (OCTAVE_DOWN, Tool::Transpose(-12)),
            (KEY_UP, Tool::Transpose(1)),
            (KEY_DOWN, Tool::Transpose(-1)),
            (BEAT_LEFT, Tool::Shift(-4)),
            (BEAT_RIGHT, Tool::Shift(4)),
            (KEY_LEFT, Tool::Shift(-1)),
            (KEY_RIGHT, Tool::Shift(1)),
            (SELECT_ALL, Tool::SelectAll),
            (
                KeyboardShortcut::new(Modifiers::COMMAND, Key::C),
                Tool::Copy,
            ),
            (KeyboardShortcut::new(Modifiers::COMMAND, Key::X), Tool::Cut),
            (
                KeyboardShortcut::new(Modifiers::COMMAND, Key::V),
                Tool::Paste,
            ),
            (
                KeyboardShortcut::new(Modifiers::COMMAND, Key::D),
                Tool::Duplicate,
            ),
            (DELETE, Tool::Delete),
            (BACKSPACE, Tool::Delete),
            (ESCAPE, Tool::SelectNone),
        ];
        keys.into_iter()
            .find(|&(shortcut, _)| pressed(shortcut))
            .map(|(_, tool)| tool)
    }

    /// Uses a tool on the selected notes, or on every note of the channel
    /// when none is selected.
    fn use_tool(
        &mut self,
        ctx: &egui::Context,
        tool: Tool,
        pattern: &Pattern,
        scale: Scale,
        actions: &mut Vec<RollAction>,
    ) {
        let length = pattern.length();
        let visible: Vec<NoteEvent> = pattern
            .notes(self.channel)
            .iter()
            .filter(|note| note.start < length)
            .copied()
            .collect();
        let targets = if self.selection.is_empty() {
            visible.clone()
        } else {
            self.selection.clone()
        };
        let (label, new) = match tool {
            Tool::Copy | Tool::Cut | Tool::Paste | Tool::Duplicate => {
                self.clipboard_tool(ctx, tool, pattern, actions);
                return;
            }
            Tool::SelectAll => {
                self.selection = visible;
                return;
            }
            Tool::SelectNone => {
                self.selection.clear();
                return;
            }
            // Delete only ever deletes a selection, never every note.
            Tool::Delete if self.selection.is_empty() => return,
            Tool::Delete => ("delete notes", Some(Vec::new())),
            Tool::Transpose(semitones) => {
                ("transpose notes", tools::transpose(&targets, semitones))
            }
            Tool::InKey(steps) => (
                "move notes in key",
                tools::transpose_in_key(&targets, scale, steps),
            ),
            Tool::Shift(steps) => ("move notes", tools::shift(&targets, steps, length)),
            Tool::Quantize => ("quantize notes", Some(tools::quantize(&targets, length))),
            Tool::SnapToKey => (
                "snap notes to key",
                Some(tools::snap_to_scale(&targets, scale)),
            ),
            Tool::Invert { up } => ("invert chords", tools::invert(&targets, up)),
            Tool::Arpeggiate => (
                "arpeggiate chords",
                Some(tools::arpeggiate(
                    &targets,
                    self.arpeggio,
                    self.arpeggio_rate,
                )),
            ),
        };
        let Some(new) = new.filter(|new| fits_view(new)) else {
            return;
        };
        let Some(new) = self.replace(pattern, actions, label, targets, new) else {
            return;
        };
        // Let the user hear what moved: the first chord (or note) of it.
        if matches!(
            tool,
            Tool::Transpose(_) | Tool::InKey(_) | Tool::Invert { .. }
        ) && let Some(first) = tools::chords(&new).into_iter().next()
        {
            actions.push(RollAction::Preview {
                channel: self.channel,
                pitches: first.iter().map(|note| note.pitch).collect(),
            });
        }
    }

    fn clipboard_tool(
        &mut self,
        ctx: &egui::Context,
        tool: Tool,
        pattern: &Pattern,
        actions: &mut Vec<RollAction>,
    ) {
        if matches!(tool, Tool::Copy | Tool::Cut | Tool::Duplicate) {
            if self.selection.is_empty() {
                return;
            }
            self.clipboard = self.selection.clone();
            if tool != Tool::Duplicate {
                crate::clipboard::copy(ctx, "notes", &self.clipboard);
            }
            if tool == Tool::Cut {
                actions.push(RollAction::ReplaceNotes {
                    channel: self.channel,
                    old: self.selection.clone(),
                    new: Vec::new(),
                    label: "cut notes",
                });
                self.selection.clear();
                return;
            }
            if tool == Tool::Copy {
                return;
            }
        }
        if let Some(notes) = crate::clipboard::paste::<Vec<NoteEvent>>(ctx, "notes") {
            self.clipboard = lineup_core::tidy_notes(notes);
        } else if tool == Tool::Paste && crate::clipboard::has_paste(ctx) {
            return;
        }
        let Some(first) = self.clipboard.iter().map(|note| note.start).min() else {
            return;
        };
        let at = if tool == Tool::Duplicate {
            self.clipboard
                .iter()
                .map(|note| note.end())
                .max()
                .unwrap_or(first)
        } else {
            self.hovered_step.or(self.last_step).unwrap_or(0)
        };
        let notes: Vec<_> = self
            .clipboard
            .iter()
            .copied()
            .filter_map(|mut note| {
                note.start = at.saturating_add(note.start - first);
                if note.start >= lineup_core::MAX_STEPS {
                    return None;
                }
                if note.end() > lineup_core::MAX_STEPS {
                    note.length = lineup_core::MAX_STEPS - note.start;
                    note.trim = 0;
                }
                if pattern
                    .notes(self.channel)
                    .iter()
                    .any(|old| old.same_place(&note))
                {
                    return None;
                }
                Some(note)
            })
            .collect();
        let Some(end) = notes.iter().map(|note| note.end()).max() else {
            return;
        };
        if end > pattern.length() {
            let bar = self.time.steps_per_bar();
            actions.push(RollAction::SetLength(
                end.div_ceil(bar)
                    .saturating_mul(bar)
                    .min(lineup_core::MAX_STEPS),
            ));
        }
        self.selection = notes.clone();
        actions.push(RollAction::AddNotes {
            channel: self.channel,
            notes,
        });
    }

    fn paint_notes(
        &self,
        painter: &Painter,
        grid: Grid,
        notes: &[NoteEvent],
        scale: Scale,
        roles: Option<&[Option<(NoteRole, Over)>]>,
    ) {
        let visible = notes
            .iter()
            .enumerate()
            .filter(|(_, note)| note.start < grid.length);
        for (index, note) in visible {
            let rect = grid.note_rect(note).shrink2(vec2(0.5, 1.0));
            let role = roles
                .and_then(|roles| roles.get(index).copied().flatten())
                .map(|(role, _)| role);
            // Notes outside the key are drawn red, so they stand out. A
            // soloist's notes are coloured by what they are.
            let mut fill = match role {
                Some(role) if role.is_chord_tone() => NOTE,
                Some(role) if role.is_chromatic() => SOLO_CHROMATIC,
                Some(_) => SOLO_SCALE,
                None if scale.contains(note.pitch.pitch_class()) => NOTE,
                None => NOTE_OUTSIDE,
            };
            // Softer notes are a little darker, as in the velocity lane.
            fill = fill.gamma_multiply(0.55 + 0.45 * f32::from(note.velocity) / 127.0);
            if self.drag.map(Drag::index) == Some(index) {
                fill = fill.gamma_multiply(1.2);
            }
            let (border, width) = if self.selection.contains(note) {
                (SELECTED_BORDER, 2.0)
            } else {
                (NOTE_BORDER, 1.0)
            };
            painter.rect_filled(rect, 2.0, fill);
            painter.rect_stroke(rect, 2.0, Stroke::new(width, border), StrokeKind::Inside);
            // The name stays in view when the start of the note is
            // scrolled away under the keyboard.
            let shown = rect.intersect(grid.cells());
            let name = scale.note_name(note.pitch);
            let text = match role {
                Some(role) if shown.width() > 44.0 => Some(format!("{name} {}", role.mark())),
                Some(role) if shown.width() > 12.0 => Some(role.mark()),
                None if shown.width() > 26.0 => Some(name),
                _ => None,
            };
            if let Some(text) = text {
                painter.with_clip_rect(shown).text(
                    shown.left_center() + vec2(3.0, 0.0),
                    Align2::LEFT_CENTER,
                    text,
                    FontId::proportional(grid.text_size()),
                    NOTE_TEXT,
                );
            }
        }
    }
}

const fn shortcut(modifiers: Modifiers, key: Key) -> KeyboardShortcut {
    KeyboardShortcut::new(modifiers, key)
}

pub(crate) const KEY_UP: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowUp);
pub(crate) const KEY_DOWN: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowDown);
pub(crate) const OCTAVE_UP: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowUp);
pub(crate) const OCTAVE_DOWN: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowDown);
pub(crate) const IN_KEY_UP: KeyboardShortcut = shortcut(Modifiers::ALT, Key::ArrowUp);
pub(crate) const IN_KEY_DOWN: KeyboardShortcut = shortcut(Modifiers::ALT, Key::ArrowDown);
pub(crate) const KEY_LEFT: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowLeft);
pub(crate) const KEY_RIGHT: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowRight);
pub(crate) const BEAT_LEFT: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowLeft);
pub(crate) const BEAT_RIGHT: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowRight);
pub(crate) const SELECT_ALL: KeyboardShortcut = shortcut(Modifiers::COMMAND, Key::A);
pub(crate) const DELETE: KeyboardShortcut = shortcut(Modifiers::NONE, Key::Delete);
pub(crate) const BACKSPACE: KeyboardShortcut = shortcut(Modifiers::NONE, Key::Backspace);
pub(crate) const ESCAPE: KeyboardShortcut = shortcut(Modifiers::NONE, Key::Escape);

/// A lane row's name, in `rect` where the keyboard is under the grid.
fn paint_lane_name(ui: &Ui, painter: &Painter, rect: Rect, name: &str) {
    let name_rect =
        Rect::from_min_max(rect.min, pos2(rect.left() + KEY_WIDTH - 2.0, rect.bottom()));
    painter.with_clip_rect(name_rect).text(
        pos2(rect.left() + 4.0, rect.center().y),
        Align2::LEFT_CENTER,
        name,
        FontId::proportional(11.0),
        ui.visuals().weak_text_color(),
    );
}

/// The name of part `voice` of a voice-leading check: its track's name,
/// and which of the track's parts it is when the track plays chords.
fn voice_name(project: &Project, check: &VoiceLeading, voice: usize) -> String {
    let part = &check.voices[voice];
    let track = project
        .channels()
        .get(part.channel)
        .map_or("Track", |channel| channel.name.as_str());
    part.name(track)
}

/// Which of its track's parts `voice` is, in a word or two: "top",
/// "2nd", "lowest", or nothing for a track's only part.
fn part_place(voice: &voice_leading::Voice) -> String {
    match (voice.rank, voice.of) {
        (_, 0 | 1) => String::new(),
        (0, _) => "top".to_string(),
        (rank, of) if rank + 1 == of => "lowest".to_string(),
        (1, _) => "2nd".to_string(),
        (2, _) => "3rd".to_string(),
        (rank, _) => format!("{}th", rank + 1),
    }
}

/// How each part of a voice-leading check moves at every change, the
/// part that doubles another in octaves left out.
fn part_moves(check: &VoiceLeading) -> Vec<Change> {
    let mut changes = voice_motion::changes(&check.voices);
    for change in &mut changes {
        change.moves.retain(|part| {
            !check
                .doublings
                .iter()
                .any(|&(_, doubled)| doubled == part.voice)
        });
    }
    changes.retain(|change| !change.moves.is_empty());
    changes
}

/// The parts of a voice-leading check, over the notes: a line from each
/// note of a part to the next one it moves to, in the part's colour, so
/// the eye can follow each part from chord to chord as on a score. The
/// moves at the change pointed at (or the last one before `step`) are
/// drawn bolder. A move the check finds wrong is drawn over it.
fn paint_parts(painter: &Painter, grid: Grid, check: &VoiceLeading, step: Option<usize>) {
    let changes = part_moves(check);
    let pointed = step.and_then(|step| voice_motion::change_at(&changes, step, usize::MAX));
    for change in &changes {
        let bold = pointed.is_some_and(|pointed| pointed.step == change.step);
        for part in &change.moves {
            if part.size() == voice_motion::Size::Held || part.to.start >= grid.length {
                continue;
            }
            let color = PART_LINES[part.voice % PART_LINES.len()];
            let stroke = match bold {
                true => Stroke::new(2.5, color),
                false => Stroke::new(1.25, color.gamma_multiply(0.6)),
            };
            let (from, to) = (grid.note_rect(&part.from), grid.note_rect(&part.to));
            // From near the end of one note to near the start of the
            // next, as the check draws a move it finds wrong.
            let start = pos2((from.right() - 6.0).max(from.center().x), from.center().y);
            let end = pos2((to.left() + 6.0).min(to.center().x), to.center().y);
            painter.line_segment([start, end], stroke);
        }
    }
}

/// What the voice-leading check found, over the notes: a line along
/// every move classical part writing avoids, from the note a part
/// leaves to the one it goes to, an outline around every note
/// twelve-tone writing avoids, joined to the notes it sounds with, and
/// a dashed outline where a leading tone, a seventh or the row should
/// have gone. Those at `step` (pointed at, or playing) are drawn bolder.
fn paint_slips(painter: &Painter, grid: Grid, check: &VoiceLeading, step: Option<usize>) {
    for slip in &check.slips {
        let bold = step.is_some_and(|step| slip.steps().contains(&step));
        let stroke = match bold {
            true => Stroke::new(3.0, SLIP),
            false => Stroke::new(1.5, SLIP.gamma_multiply(0.75)),
        };
        for part in &slip.moves {
            if part.to.start >= grid.length {
                continue;
            }
            let (from, to) = (grid.note_rect(&part.from), grid.note_rect(&part.to));
            // From near the end of one note to near the start of the
            // next, leaving the note names clear.
            let start = pos2((from.right() - 6.0).max(from.center().x), from.center().y);
            let end = pos2((to.left() + 6.0).min(to.center().x), to.center().y);
            painter.line_segment([start, end], stroke);
            painter.circle_filled(start, stroke.width + 0.5, stroke.color);
            painter.circle_filled(end, stroke.width + 0.5, stroke.color);
        }
        let notes: Vec<Rect> = slip
            .notes
            .iter()
            .filter(|played| played.note.start < grid.length)
            .map(|played| grid.note_rect(&played.note).expand(1.0))
            .collect();
        for rect in &notes {
            painter.rect_stroke(*rect, 3.0, stroke, StrokeKind::Outside);
        }
        // Notes sounding together, joined where the later one starts.
        if let (Some(top), Some(bottom)) = (
            notes.iter().map(|rect| rect.top()).reduce(f32::min),
            notes.iter().map(|rect| rect.bottom()).reduce(f32::max),
        ) && notes.len() > 1
        {
            let x = notes
                .iter()
                .map(|rect| rect.left())
                .fold(f32::MIN, f32::max)
                + 3.0;
            painter.line_segment([pos2(x, top), pos2(x, bottom)], stroke);
        }
        let landed = slip
            .moves
            .first()
            .map(|part| part.to)
            .or_else(|| slip.notes.first().map(|played| played.note));
        if let (Some(expected), Some(landed)) = (slip.expected, landed)
            && landed.start < grid.length
        {
            let target = NoteEvent {
                pitch: expected,
                ..landed
            };
            let rect = grid.note_rect(&target).shrink2(vec2(0.5, 1.0));
            let outline = [
                rect.left_top(),
                rect.right_top(),
                rect.right_bottom(),
                rect.left_bottom(),
                rect.left_top(),
            ];
            let dashes = Stroke::new(stroke.width.min(2.0), stroke.color);
            painter.extend(egui::Shape::dashed_line(&outline, dashes, 4.0, 3.0));
        }
    }
}

/// The harmony of all the tracks at every step of pattern `pattern`,
/// `length` steps long, heard beat by beat with the whole band (see
/// [`Project::hearing_with_pattern`]).
fn heard_by_beat(project: &Project, pattern: usize, length: usize) -> Vec<Harmony> {
    let mut beats = project
        .hearings_with_pattern(pattern)
        .into_iter()
        .peekable();
    let mut heard = Vec::with_capacity(length);
    let mut harmony = Harmony::Silence;
    for step in 0..length {
        while let Some((_, hearing)) = beats.next_if(|(start, _)| *start <= step) {
            harmony = hearing.best();
        }
        heard.push(harmony);
    }
    heard
}

/// The pitches held during `step` on any of `channels`.
fn sounding<'a>(
    pattern: &'a Pattern,
    channels: &'a [usize],
    step: usize,
) -> impl Iterator<Item = Note> + 'a {
    channels
        .iter()
        .flat_map(move |&channel| pattern.sounding_at(channel, step))
}

/// Whether every note is on a row the piano roll shows.
fn fits_view(notes: &[NoteEvent]) -> bool {
    notes
        .iter()
        .all(|note| (LOWEST..=HIGHEST).contains(&note.pitch.midi()))
}

/// The index of the note under `pos`. Later notes are drawn on top, so
/// they are searched first.
fn note_at(notes: &[NoteEvent], grid: Grid, pos: Pos2) -> Option<usize> {
    notes
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, note)| note.start < grid.length)
        .find(|(_, note)| grid.note_rect(note).contains(pos))
        .map(|(index, _)| index)
}

/// What the key menus say about changing the key, here and in the
/// harmony keys bar.
pub(crate) const KEY_HELP: &str = "The key of the song. A new key or mode moves the \
    chords, bass and melody into it (the drums stay), so the song plays in the new key. \
    Ctrl+Z puts them back.";

/// "C" for white keys, "C#/Db" for black keys.
pub(crate) fn both_names(pitch_class: PitchClass) -> String {
    let sharp = pitch_class.name(Spelling::Sharps);
    let flat = pitch_class.name(Spelling::Flats);
    if sharp == flat {
        sharp.to_string()
    } else {
        format!("{sharp}/{flat}")
    }
}

/// Paints the row backgrounds. Rows of notes in the key are lighter, and
/// the key's root is tinted, so the scale shows at a glance.
fn paint_rows(painter: &Painter, grid: Grid, scale: Scale) {
    for midi in LOWEST..=HIGHEST {
        let pitch_class = PitchClass::from_semitone(midi);
        let top = grid.row_top(midi);
        let in_scale = scale.contains(pitch_class);
        let is_root = pitch_class == scale.root;

        let row = Rect::from_min_size(
            pos2(grid.rect.left(), top),
            vec2(grid.rect.width(), grid.row_height),
        );
        let fill = match (is_root, in_scale) {
            (true, _) => ROOT_ROW,
            (false, true) => SCALE_ROW,
            (false, false) => OTHER_ROW,
        };
        painter.rect_filled(row, 0.0, fill);
        let line = if pitch_class == PitchClass::C {
            OCTAVE_LINE
        } else {
            ROW_LINE
        };
        painter.hline(row.x_range(), row.bottom(), Stroke::new(1.0, line));
    }
}

/// Paints the keyboard at the left of the view, with dots on the notes of
/// the key (or numbers, for a row). Keys held down on a keyboard light up.
fn paint_keys(painter: &Painter, grid: Grid, scale: Scale, held: &[Note]) {
    let keys = grid.keys();
    painter.rect_filled(keys, 0.0, OCTAVE_LINE);
    for midi in LOWEST..=HIGHEST {
        let pitch_class = PitchClass::from_semitone(midi);
        let top = grid.row_top(midi);
        let in_scale = scale.contains(pitch_class);
        let is_root = pitch_class == scale.root;
        let key = Rect::from_min_size(pos2(keys.left(), top), vec2(KEY_WIDTH, grid.row_height));
        let key_fill = if held.contains(&Note::new(midi)) {
            KEY_HELD
        } else if pitch_class.is_black_key() {
            BLACK_KEY
        } else {
            WHITE_KEY
        };
        painter.rect_filled(key.shrink2(vec2(0.0, 0.5)), 1.0, key_fill);
        if let ScaleKind::Row(_) = scale.kind {
            // A row numbers its notes in order, 1 to 12, instead of dots.
            let place = scale.degree_of(pitch_class).unwrap_or(0) + 1;
            let mark = if is_root { ROOT_MARK } else { ROW_MARK };
            painter.text(
                pos2(key.right() - 3.0, key.center().y),
                Align2::RIGHT_CENTER,
                place.to_string(),
                FontId::monospace(grid.text_size() - 1.0),
                mark,
            );
        } else if in_scale {
            let mark = if is_root { ROOT_MARK } else { SCALE_MARK };
            painter.circle_filled(pos2(key.right() - 7.0, key.center().y), 3.0, mark);
        }
        if pitch_class == PitchClass::C {
            painter.text(
                pos2(key.left() + 4.0, key.center().y),
                Align2::LEFT_CENTER,
                Note::new(midi).to_string(),
                FontId::proportional(grid.text_size()),
                KEY_TEXT,
            );
        }
    }
}

/// Vertical lines: faint for steps, stronger for beats, strongest for bars.
fn paint_step_lines(painter: &Painter, grid: Grid) {
    for step in 0..=grid.length {
        let color = match step {
            s if grid.time.is_bar_start(s) => BAR_LINE,
            s if grid.time.is_beat_start(s) => BEAT_LINE,
            _ => STEP_LINE,
        };
        painter.vline(grid.x(step), grid.rect.y_range(), Stroke::new(1.0, color));
    }
}

/// The channel with the most melody or chord notes in a pattern, if any
/// channel has some. Drum channels, whose notes are plain steps, are left
/// out: the channel rack is the place for them.
pub fn busiest_channel(pattern: &Pattern) -> Option<usize> {
    (0..pattern.rows())
        .map(|channel| {
            let melodic = pattern.notes(channel).iter().filter(|note| !note.is_step());
            (channel, melodic.count())
        })
        .filter(|&(_, count)| count > 0)
        .max_by_key(|&(channel, count)| (count, std::cmp::Reverse(channel)))
        .map(|(channel, _)| channel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_clipboard_events_copy_paste_and_cut_notes_between_rolls() {
        fn frame(
            ctx: &egui::Context,
            roll: &mut PianoRoll,
            project: &Project,
            events: Vec<egui::Event>,
        ) -> (egui::FullOutput, Vec<RollAction>) {
            let mut actions = Vec::new();
            let screen = Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0));
            let mut output = ctx.run_pass(
                egui::RawInput {
                    screen_rect: Some(screen),
                    events,
                    time: Some(ctx.cumulative_frame_nr() as f64 * 0.1),
                    ..Default::default()
                },
                |ctx| {
                    actions = roll.show(ctx, project, 0, None, &[], screen, Vec2::splat(1.0));
                },
            );
            output.textures_delta.clear();
            (output, actions)
        }
        let mut project = Project::empty();
        project.add_channel(lineup_core::Channel::synth());
        let mut quiet = NoteEvent::new(Note::C4, 0, 4);
        quiet.velocity = 43;
        project.pattern_mut(0).unwrap().add_note(0, quiet);
        project
            .pattern_mut(0)
            .unwrap()
            .add_note(0, NoteEvent::new(Note::new(67), 8, 2));
        let ctx = egui::Context::default();
        let mut roll = PianoRoll::new(0);
        roll.open = true;
        frame(&ctx, &mut roll, &project, vec![]);
        frame(&ctx, &mut roll, &project, vec![]);
        frame(
            &ctx,
            &mut roll,
            &project,
            vec![egui::Event::Key {
                key: Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::COMMAND,
            }],
        );
        assert_eq!(roll.selection.len(), 2);
        let (output, _) = frame(&ctx, &mut roll, &project, vec![egui::Event::Copy]);
        let copied = output
            .platform_output
            .commands
            .iter()
            .find_map(|command| match command {
                egui::OutputCommand::CopyText(text) => Some(text.clone()),
                _ => None,
            })
            .expect("copy writes to the system clipboard");
        let other_ctx = egui::Context::default();
        let mut other = PianoRoll::new(0);
        other.open = true;
        frame(&other_ctx, &mut other, &project, vec![]);
        frame(&other_ctx, &mut other, &project, vec![]);
        other.last_step = Some(4);
        let (_, actions) = frame(
            &other_ctx,
            &mut other,
            &project,
            vec![egui::Event::Paste(copied)],
        );
        let pasted = actions
            .iter()
            .find_map(|action| match action {
                RollAction::AddNotes { notes, .. } => Some(notes.clone()),
                _ => None,
            })
            .unwrap();
        assert_eq!(
            pasted.iter().map(|note| note.start).collect::<Vec<_>>(),
            vec![4, 12]
        );
        assert_eq!(pasted[0].velocity, 43);
        for note in &pasted {
            project.pattern_mut(0).unwrap().add_note(0, *note);
        }
        let (_, actions) = frame(&other_ctx, &mut other, &project, vec![egui::Event::Cut]);
        assert!(actions.iter().any(|action| matches!(action,RollAction::ReplaceNotes {old,new,..} if *old == pasted && new.is_empty())));
    }

    const CHORDS: usize = 1;
    const SYNTH: usize = 4;

    /// Draws piano rolls without a window, a frame at a time.
    struct Screen {
        ctx: egui::Context,
        frame: usize,
    }

    impl Screen {
        fn new() -> Self {
            Screen {
                ctx: egui::Context::default(),
                frame: 0,
            }
        }

        /// Draws `frames` frames of `roll` showing `pattern` of `project`.
        fn run(
            &mut self,
            roll: &mut PianoRoll,
            project: &Project,
            pattern: usize,
            zoom: Vec2,
            frames: usize,
        ) {
            let screen = Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0));
            for _ in 0..frames {
                // Scrolling is animated, so time must pass.
                let input = egui::RawInput {
                    screen_rect: Some(screen),
                    time: Some(self.frame as f64 * 0.1),
                    ..Default::default()
                };
                self.frame += 1;
                let mut output = self.ctx.run_pass(input, |ctx| {
                    roll.show(ctx, project, pattern, None, &[], screen, zoom);
                });
                // Nothing is drawn, so the font textures are never uploaded.
                output.textures_delta.clear();
            }
        }
    }

    /// Draws `frames` frames of a piano roll on `pattern` of the demo song.
    fn run(roll: &mut PianoRoll, project: &Project, pattern: usize, frames: usize) {
        Screen::new().run(roll, project, pattern, Vec2::splat(1.0), frames);
    }

    /// The pitch in the middle of the view.
    fn middle(roll: &PianoRoll) -> u8 {
        let (lowest, highest) = roll.in_view.expect("the grid was drawn");
        lowest / 2 + highest / 2
    }

    #[test]
    fn zooming_keeps_the_middle() {
        let project = Project::demo();
        let mut roll = PianoRoll::new(SYNTH);
        roll.open = true;
        let mut screen = Screen::new();
        screen.run(&mut roll, &project, CHORDS, Vec2::splat(1.0), 20);
        let before = middle(&roll);
        for zoom in [1.1, 1.25, 1.5] {
            screen.run(&mut roll, &project, CHORDS, vec2(zoom, zoom), 1);
        }
        screen.run(&mut roll, &project, CHORDS, Vec2::splat(1.5), 5);
        assert!(
            middle(&roll).abs_diff(before) <= 1,
            "{before} became {}",
            middle(&roll)
        );
    }

    #[test]
    fn explicit_navigation_reveals_pattern_ends_without_moving_the_pitch_view() {
        let project = Project::demo();
        let mut roll = PianoRoll::new(SYNTH);
        roll.open = true;
        let mut screen = Screen::new();
        screen
            .ctx
            .data_mut(|data| data.insert_persisted(egui::Id::unique("follow-playback"), false));
        screen.run(&mut roll, &project, CHORDS, vec2(2.0, 1.0), 3);
        let top = roll.scrolled.y;
        roll.reveal_step(project.pattern(CHORDS).unwrap().length() - 1);
        screen.run(&mut roll, &project, CHORDS, vec2(2.0, 1.0), 2);
        assert!(roll.scrolled.x > 0.0);
        assert!((roll.scrolled.y - top).abs() < 1.0);
        roll.reveal_step(0);
        screen.run(&mut roll, &project, CHORDS, vec2(2.0, 1.0), 2);
        assert_eq!(roll.scrolled.x, 0.0);
        assert!((roll.scrolled.y - top).abs() < 1.0);
    }

    #[test]
    fn follows_the_channel_with_notes() {
        let project = Project::demo();
        // Opened on the kick, which plays nothing in "Chords".
        let mut roll = PianoRoll::new(0);
        roll.open = true;
        run(&mut roll, &project, CHORDS, 2);
        assert_eq!(roll.channel(), SYNTH);
    }

    #[test]
    fn explicit_channel_choices_survive_reopening_and_pattern_changes() {
        let project = Project::demo();
        let mut roll = PianoRoll::new(SYNTH);
        roll.select(0); // Kick has no notes in the Chords pattern.
        run(&mut roll, &project, CHORDS, 2);
        assert_eq!(roll.selected_channel(), Some(0));
        roll.open_for(0);
        run(&mut roll, &project, CHORDS, 2);
        assert_eq!(roll.selected_channel(), Some(0));
        run(&mut roll, &project, 0, 2);
        run(&mut roll, &project, CHORDS, 2);
        assert_eq!(roll.selected_channel(), Some(0));
        roll.deselect_channel();
        run(&mut roll, &project, CHORDS, 2);
        assert_eq!(roll.selected_channel(), None);
        assert!(!roll.open);
        roll.select(SYNTH);
        run(&mut roll, &project, CHORDS, 2);
        assert_eq!(roll.selected_channel(), Some(SYNTH));
    }

    #[test]
    fn stays_on_its_channel_when_no_other_has_melodies() {
        let project = Project::demo();
        let mut roll = PianoRoll::new(SYNTH);
        roll.open = true;
        run(&mut roll, &project, 0, 2);
        assert_eq!(roll.channel(), SYNTH);
    }

    #[test]
    fn opens_scrolled_to_the_notes() {
        let project = Project::demo();
        let mut roll = PianoRoll::new(SYNTH);
        roll.open = true;
        run(&mut roll, &project, CHORDS, 20);
        // The chords go from F2 (41) to G4 (67).
        let (lowest, highest) = roll.in_view.expect("the grid was drawn");
        assert!(
            lowest <= 41 && highest >= 67,
            "in view: {lowest}..={highest}"
        );
    }

    #[test]
    fn empty_pattern_shows_middle_c() {
        let project = Project::demo();
        let mut roll = PianoRoll::new(SYNTH);
        run(&mut roll, &project, 0, 3);
        roll.open = true;
        run(&mut roll, &project, 0, 20);
        let (lowest, highest) = roll.in_view.expect("the grid was drawn");
        assert!(
            lowest <= 60 && highest >= 60,
            "in view: {lowest}..={highest}"
        );
    }
}
