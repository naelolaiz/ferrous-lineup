//! The piano roll: a window for drawing notes, with harmony helpers.
//!
//! Like the channel rack, [`PianoRoll::show`] never changes the project:
//! it returns [`RollAction`]s for the app to apply. Unlike the rack, the
//! piano roll has state of its own that must last from one frame to the
//! next (which channel it edits, what a click draws, a drag in progress),
//! so it is a struct with methods instead of a single function.

use eframe::egui::{
    self, Align, Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind,
    Ui, pos2, vec2,
};
use loom_core::harmony::analyze;
use loom_core::{
    ChordQuality, ChordStamp, Note, NoteEvent, PitchClass, Project, Scale, ScaleKind, Spelling,
};

/// Something the user asked the piano roll to do.
#[derive(Debug, Clone, PartialEq)]
pub enum RollAction {
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
    /// Play some notes right now, without adding them.
    Preview {
        channel: usize,
        pitches: Vec<Note>,
    },
    SetScale(Scale),
}

/// The range of notes shown, from C1 to B7.
const LOWEST: u8 = 24;
const HIGHEST: u8 = 107;

const ROW_HEIGHT: f32 = 12.0;
const KEY_WIDTH: f32 = 54.0;
const LANE_HEIGHT: f32 = 22.0;
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

const ROOT_ROW: Color32 = Color32::from_rgb(70, 58, 52);
const SCALE_ROW: Color32 = Color32::from_rgb(52, 56, 64);
const OTHER_ROW: Color32 = Color32::from_rgb(36, 39, 45);
const ROW_LINE: Color32 = Color32::from_rgb(30, 32, 37);
const OCTAVE_LINE: Color32 = Color32::from_rgb(20, 21, 25);
const STEP_LINE: Color32 = Color32::from_rgb(44, 47, 54);
const BEAT_LINE: Color32 = Color32::from_rgb(68, 72, 82);
const BAR_LINE: Color32 = Color32::from_rgb(110, 116, 130);
const WHITE_KEY: Color32 = Color32::from_rgb(222, 224, 228);
const BLACK_KEY: Color32 = Color32::from_rgb(40, 42, 46);
const KEY_TEXT: Color32 = Color32::from_rgb(60, 62, 68);
const SCALE_MARK: Color32 = Color32::from_rgb(120, 190, 255);
const ROOT_MARK: Color32 = Color32::from_rgb(255, 152, 48);
const NOTE: Color32 = Color32::from_rgb(118, 214, 140);
const NOTE_OUTSIDE: Color32 = Color32::from_rgb(240, 120, 110);
const NOTE_BORDER: Color32 = Color32::from_rgb(24, 40, 30);
const NOTE_TEXT: Color32 = Color32::from_rgb(16, 28, 20);
const GHOST: Color32 = Color32::from_rgba_premultiplied(60, 70, 64, 60);
const PLAYHEAD: Color32 = Color32::from_rgba_premultiplied(64, 38, 12, 60);
const LANE_BACKGROUND: Color32 = Color32::from_rgb(30, 32, 37);
const LANE_CHORD: Color32 = Color32::from_rgb(72, 88, 120);
const LANE_TEXT: Color32 = Color32::from_rgb(235, 238, 245);

/// A drag that started on a note and lasts until the mouse button is released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Drag {
    /// Moving a note. `grab` is how many steps after the note's start the
    /// pointer took hold of it, so the note does not jump under the pointer.
    Move { index: usize, grab: usize },
    /// Changing a note's length by dragging its right end.
    Resize { index: usize },
}

impl Drag {
    fn index(self) -> usize {
        match self {
            Drag::Move { index, .. } | Drag::Resize { index } => index,
        }
    }
}

/// Where the grid is on screen, and how to turn steps and pitches into
/// pixels and back. It is `Copy`, so it can be handed around freely.
#[derive(Debug, Clone, Copy)]
struct Grid {
    rect: Rect,
    step_width: f32,
    length: usize,
}

impl Grid {
    /// The keyboard, just left of the grid.
    fn keys(self) -> Rect {
        Rect::from_min_max(
            pos2(self.rect.left() - KEY_WIDTH, self.rect.top()),
            pos2(self.rect.left(), self.rect.bottom()),
        )
    }

    /// The x coordinate where `step` begins.
    fn x(self, step: usize) -> f32 {
        self.rect.left() + step as f32 * self.step_width
    }

    /// The y coordinate of the top of a pitch's row (high notes at the top).
    fn row_top(self, midi: u8) -> f32 {
        self.rect.top() + (HIGHEST - midi) as f32 * ROW_HEIGHT
    }

    fn note_rect(self, note: &NoteEvent) -> Rect {
        let top = self.row_top(note.pitch.midi());
        let end = note.end().min(self.length);
        Rect::from_min_max(
            pos2(self.x(note.start), top),
            pos2(self.x(end), top + ROW_HEIGHT),
        )
    }

    /// The step and pitch under a point. Points outside the grid give the
    /// nearest cell, so a drag can go past the edges.
    fn cell_at(self, pos: Pos2) -> (usize, Note) {
        let last_step = self.length.saturating_sub(1) as f32;
        let step = ((pos.x - self.rect.left()) / self.step_width)
            .floor()
            .clamp(0.0, last_step) as usize;
        let row = ((pos.y - self.rect.top()) / ROW_HEIGHT)
            .floor()
            .clamp(0.0, (HIGHEST - LOWEST) as f32) as u8;
        (step, Note::new(HIGHEST - row))
    }
}

/// The piano roll window and everything it remembers between frames.
pub struct PianoRoll {
    pub open: bool,
    channel: usize,
    stamp: ChordStamp,
    /// The length of new notes, in steps. Resizing a note changes it, so
    /// the next note drawn is as long as the last one edited.
    note_length: usize,
    drag: Option<Drag>,
    /// Scroll to the channel's notes the next time the grid is drawn.
    scroll_to_notes: bool,
    /// The step the pointer is over, if any, to name the chord there.
    hovered_step: Option<usize>,
}

impl PianoRoll {
    /// A closed piano roll for `channel`.
    pub fn new(channel: usize) -> Self {
        PianoRoll {
            open: false,
            channel,
            stamp: ChordStamp::Single,
            note_length: 4,
            drag: None,
            scroll_to_notes: true,
            hovered_step: None,
        }
    }

    /// Opens the piano roll on a channel.
    pub fn open_for(&mut self, channel: usize) {
        if channel != self.channel {
            self.scroll_to_notes = true;
        }
        self.open = true;
        self.channel = channel;
        self.drag = None;
    }

    /// Keeps pointing at the same channel after channel `removed` is
    /// deleted, or closes the window if it was this one.
    pub fn channel_removed(&mut self, removed: usize) {
        if removed == self.channel {
            self.open = false;
        } else if removed < self.channel {
            self.channel -= 1;
        }
        self.drag = None;
    }

    /// Draws the window, if it is open, and returns what the user did.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        project: &Project,
        playhead: Option<usize>,
    ) -> Vec<RollAction> {
        let mut actions = Vec::new();
        let Some(channel) = project.channels().get(self.channel) else {
            self.open = false;
            return actions;
        };

        // `open` is a copy: the window needs `&mut open` while the closure
        // below needs `&mut self`, and both cannot borrow `self` at once.
        let mut open = self.open;
        egui::Window::new(format!("Piano roll · {}", channel.name))
            .id(egui::Id::unique("piano-roll"))
            .open(&mut open)
            .default_pos([12.0, 246.0])
            .default_size([1060.0, 486.0])
            .show(ctx, |ui| self.contents(ui, project, playhead, &mut actions));
        self.open = open;
        actions
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        playhead: Option<usize>,
        actions: &mut Vec<RollAction>,
    ) {
        let scale = project.scale;
        self.toolbar(ui, scale, actions);
        self.chords_in_key(ui, scale, actions);

        // The step under the pointer was found while drawing the last
        // frame; one frame late is too fast for anyone to notice.
        let info = match self.hovered_step.or(playhead) {
            Some(step) => {
                let harmony = analyze(project.pattern().sounding_at(self.channel, step));
                format!("Step {}: {}", step + 1, harmony.describe(scale))
            }
            None => "Point at a step to name the chord there.".to_string(),
        };
        ui.label(info);
        ui.add_space(2.0);

        let length = project.pattern().length();
        let step_width = ((ui.available_width() - KEY_WIDTH) / length as f32).max(8.0);
        self.hovered_step = None;

        self.chord_lane(ui, project, step_width, playhead);
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                self.grid(ui, project, step_width, playhead, actions)
            });
    }

    /// The row of menus at the top: key, scale, what to draw and how long.
    fn toolbar(&mut self, ui: &mut Ui, scale: Scale, actions: &mut Vec<RollAction>) {
        ui.horizontal_wrapped(|ui| {
            ui.label("Key");
            let mut root = scale.root;
            egui::ComboBox::from_id_salt("scale-root")
                .width(64.0)
                .selected_text(root.name(scale.spelling()))
                .show_ui(ui, |ui| {
                    for pitch_class in PitchClass::ALL {
                        ui.selectable_value(&mut root, pitch_class, both_names(pitch_class));
                    }
                });
            let mut kind = scale.kind;
            egui::ComboBox::from_id_salt("scale-kind")
                .selected_text(kind.name())
                .show_ui(ui, |ui| {
                    for option in ScaleKind::ALL {
                        ui.selectable_value(&mut kind, option, option.name());
                    }
                });
            if root != scale.root || kind != scale.kind {
                actions.push(RollAction::SetScale(Scale::new(root, kind)));
            }
            let spelling = scale.spelling();
            let names: Vec<&str> = scale.pitch_classes().map(|p| p.name(spelling)).collect();
            ui.weak(names.join(" "));

            ui.separator();
            ui.label("Draw");
            egui::ComboBox::from_id_salt("chord-stamp")
                .width(170.0)
                .selected_text(self.stamp.label())
                .show_ui(ui, |ui| {
                    for stamp in ChordStamp::all() {
                        if stamp == ChordStamp::Fixed(ChordQuality::ALL[0]) {
                            ui.separator();
                        }
                        ui.selectable_value(&mut self.stamp, stamp, stamp.label());
                    }
                })
                .response
                .on_hover_text("What a click on the grid places: one note, or a whole chord.");

            ui.label("Length");
            let length_name = NOTE_LENGTHS
                .iter()
                .find(|(steps, _)| *steps == self.note_length)
                .map(|(_, name)| name.to_string())
                .unwrap_or_else(|| format!("{} steps", self.note_length));
            egui::ComboBox::from_id_salt("note-length")
                .selected_text(length_name)
                .show_ui(ui, |ui| {
                    for (steps, name) in NOTE_LENGTHS {
                        ui.selectable_value(&mut self.note_length, steps, name);
                    }
                });
        });
    }

    /// Buttons for the seven chords of the key, to hear how they sound.
    fn chords_in_key(&self, ui: &mut Ui, scale: Scale, actions: &mut Vec<RollAction>) {
        if !scale.is_heptatonic() {
            return;
        }
        let spelling = scale.spelling();
        ui.horizontal_wrapped(|ui| {
            ui.label(format!("Chords in {}", scale.name()));
            for degree in 0..7 {
                let Some(chord) = scale.triad(degree) else {
                    continue;
                };
                let numeral = scale.roman_numeral(chord).unwrap_or_default();
                let text = format!("{numeral} · {}", chord.name(spelling));
                let hint = format!(
                    "{} {}. Click to hear it. Pick \"Chord in key\" under Draw to place chords like it.",
                    chord.root.name(spelling),
                    chord.quality.name()
                );
                if ui.button(text).on_hover_text(hint).clicked() {
                    let root = Note::new(Note::C4.midi() + chord.root.semitone());
                    actions.push(RollAction::Preview {
                        channel: self.channel,
                        pitches: chord.notes_from(root),
                    });
                }
            }
        });
    }

    /// The strip above the grid that names the chord at every step.
    fn chord_lane(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        step_width: f32,
        playhead: Option<usize>,
    ) {
        let length = project.pattern().length();
        let spelling = project.scale.spelling();
        let width = KEY_WIDTH + step_width * length as f32;
        let (rect, response) = ui.allocate_exact_size(vec2(width, LANE_HEIGHT), Sense::hover());
        let lane = Grid {
            rect: Rect::from_min_max(pos2(rect.left() + KEY_WIDTH, rect.top()), rect.max),
            step_width,
            length,
        };
        if let Some(pos) = response.hover_pos()
            && lane.rect.contains(pos)
        {
            self.hovered_step = Some(lane.cell_at(pos).0);
        }

        let painter = ui.painter_at(rect);
        painter.text(
            pos2(rect.left() + 4.0, rect.center().y),
            Align2::LEFT_CENTER,
            "Chords",
            FontId::proportional(11.0),
            ui.visuals().weak_text_color(),
        );
        painter.rect_filled(lane.rect, 2.0, LANE_BACKGROUND);

        // Name the harmony at every step, then join runs of equal names:
        // `chunk_by` splits a slice wherever two neighbours differ.
        let labels: Vec<Option<String>> = (0..length)
            .map(|step| analyze(project.pattern().sounding_at(self.channel, step)).label(spelling))
            .collect();
        let mut start = 0;
        for run in labels.chunk_by(|a, b| a == b) {
            let end = start + run.len();
            if let Some(label) = &run[0] {
                let chord_rect = Rect::from_min_max(
                    pos2(lane.x(start), lane.rect.top()),
                    pos2(lane.x(end), lane.rect.bottom()),
                )
                .shrink2(vec2(1.0, 2.0));
                painter.rect_filled(chord_rect, 3.0, LANE_CHORD);
                painter.with_clip_rect(chord_rect).text(
                    chord_rect.center(),
                    Align2::CENTER_CENTER,
                    label,
                    FontId::proportional(12.0),
                    LANE_TEXT,
                );
            }
            start = end;
        }

        if let Some(step) = playhead.filter(|&step| step < length) {
            let x = lane.x(step);
            painter.rect_filled(
                Rect::from_min_max(
                    pos2(x, lane.rect.top()),
                    pos2(x + step_width, lane.rect.bottom()),
                ),
                0.0,
                PLAYHEAD,
            );
        }
    }

    /// The keyboard and the note grid, inside the scroll area.
    fn grid(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        step_width: f32,
        playhead: Option<usize>,
        actions: &mut Vec<RollAction>,
    ) {
        let length = project.pattern().length();
        let rows = (HIGHEST - LOWEST + 1) as f32;
        let size = vec2(KEY_WIDTH + step_width * length as f32, rows * ROW_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());
        let grid = Grid {
            rect: Rect::from_min_max(pos2(rect.left() + KEY_WIDTH, rect.top()), rect.max),
            step_width,
            length,
        };

        let notes = project.notes(self.channel);
        if self.scroll_to_notes {
            // Centre the view between the lowest and highest note, or on
            // middle C when there are no notes yet.
            let pitches = || notes.iter().map(|note| note.pitch.midi());
            let lowest = pitches().min().unwrap_or(Note::C4.midi());
            let highest = pitches().max().unwrap_or(Note::C4.midi());
            let middle = lowest / 2 + highest / 2;
            let top = grid.row_top(middle.clamp(LOWEST, HIGHEST));
            let target = Rect::from_min_size(pos2(rect.left(), top), vec2(1.0, ROW_HEIGHT));
            ui.scroll_to_rect(target, Some(Align::Center));
            self.scroll_to_notes = false;
        }
        let hit = self.handle_input(ui, &response, grid, notes, project.scale, actions);

        let painter = ui.painter_at(rect);
        paint_rows(&painter, grid, project.scale);
        paint_step_lines(&painter, grid);
        if let Some(step) = playhead.filter(|&step| step < length) {
            let x = grid.x(step);
            let column = Rect::from_min_max(
                pos2(x, grid.rect.top()),
                pos2(x + step_width, grid.rect.bottom()),
            );
            painter.rect_filled(column, 0.0, PLAYHEAD);
        }
        self.paint_notes(&painter, grid, notes, project.scale);

        // Show what a click would place: a "ghost" of the note or chord.
        if self.drag.is_none()
            && hit.is_none()
            && let Some(pos) = response.hover_pos()
            && grid.rect.contains(pos)
        {
            let (step, pitch) = grid.cell_at(pos);
            for ghost_pitch in self.stamp.notes_at(pitch, project.scale) {
                let ghost = NoteEvent::new(ghost_pitch, step, self.note_length);
                let ghost_rect = grid.note_rect(&ghost).shrink2(vec2(0.5, 1.0));
                painter.rect_filled(ghost_rect, 2.0, GHOST);
                painter.rect_stroke(ghost_rect, 2.0, Stroke::new(1.0, NOTE), StrokeKind::Inside);
            }
        }
    }

    /// Reads the mouse for the grid and the keyboard. Returns the index of
    /// the note under the pointer, if any.
    ///
    /// - left click on a key: hear it
    /// - left click on empty grid: add a note (or chord); keep the button
    ///   down to drag the new note
    /// - left drag a note: move it; drag its right end: resize it
    /// - right click (or right drag over) notes: delete them
    fn handle_input(
        &mut self,
        ui: &Ui,
        response: &Response,
        grid: Grid,
        notes: &[NoteEvent],
        scale: Scale,
        actions: &mut Vec<RollAction>,
    ) -> Option<usize> {
        let channel = self.channel;
        let (primary_pressed, primary_down, secondary, pointer) = ui.input(|input| {
            let pointer = &input.pointer;
            (
                pointer.primary_pressed(),
                pointer.primary_down(),
                // A quick click can press and release within one frame,
                // so "pressed this frame" counts as well as "held down".
                pointer.secondary_pressed() || pointer.secondary_down(),
                pointer.interact_pos(),
            )
        });

        let hovered = response.hover_pos();
        let hit = hovered.and_then(|pos| note_at(notes, grid, pos));

        if let Some(pos) = hovered {
            if grid.keys().contains(pos) && primary_pressed {
                let (_, pitch) = grid.cell_at(pos);
                actions.push(RollAction::Preview {
                    channel,
                    pitches: vec![pitch],
                });
            } else if grid.rect.contains(pos) {
                let (step, pitch) = grid.cell_at(pos);
                self.hovered_step = Some(step);

                if primary_pressed {
                    match hit {
                        Some(index) => {
                            let note = notes[index];
                            let near_end = pos.x > grid.note_rect(&note).right() - EDGE_GRAB;
                            self.drag = Some(if near_end {
                                Drag::Resize { index }
                            } else {
                                Drag::Move {
                                    index,
                                    grab: step.saturating_sub(note.start),
                                }
                            });
                            actions.push(RollAction::Preview {
                                channel,
                                pitches: vec![note.pitch],
                            });
                        }
                        None => {
                            let pitches = self.stamp.notes_at(pitch, scale);
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
            self.drag = None;
        }
        // The drag follows the pointer even outside the grid.
        if let Some(drag) = self.drag
            && let Some(pos) = pointer
            && let Some(&note) = notes.get(drag.index())
        {
            let (step, pitch) = grid.cell_at(pos);
            let changed = match drag {
                Drag::Move { grab, .. } => NoteEvent {
                    pitch,
                    start: step.saturating_sub(grab),
                    ..note
                },
                Drag::Resize { .. } => {
                    let length = (step + 1).saturating_sub(note.start).max(1);
                    self.note_length = length;
                    NoteEvent { length, ..note }
                }
            };
            if changed != note {
                if changed.pitch != note.pitch {
                    actions.push(RollAction::Preview {
                        channel,
                        pitches: vec![changed.pitch],
                    });
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

    fn paint_notes(&self, painter: &Painter, grid: Grid, notes: &[NoteEvent], scale: Scale) {
        let spelling = scale.spelling();
        let visible = notes
            .iter()
            .enumerate()
            .filter(|(_, note)| note.start < grid.length);
        for (index, note) in visible {
            let rect = grid.note_rect(note).shrink2(vec2(0.5, 1.0));
            // Notes outside the key are drawn red, so they stand out.
            let mut fill = if scale.contains(note.pitch.pitch_class()) {
                NOTE
            } else {
                NOTE_OUTSIDE
            };
            if self.drag.map(Drag::index) == Some(index) {
                fill = fill.gamma_multiply(1.2);
            }
            painter.rect_filled(rect, 2.0, fill);
            painter.rect_stroke(rect, 2.0, Stroke::new(1.0, NOTE_BORDER), StrokeKind::Inside);
            if rect.width() > 26.0 {
                painter.with_clip_rect(rect).text(
                    rect.left_center() + vec2(3.0, 0.0),
                    Align2::LEFT_CENTER,
                    note.pitch.name(spelling),
                    FontId::proportional(10.0),
                    NOTE_TEXT,
                );
            }
        }
    }
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

/// "C" for white keys, "C#/Db" for black keys.
fn both_names(pitch_class: PitchClass) -> String {
    let sharp = pitch_class.name(Spelling::Sharps);
    let flat = pitch_class.name(Spelling::Flats);
    if sharp == flat {
        sharp.to_string()
    } else {
        format!("{sharp}/{flat}")
    }
}

/// Paints the keyboard and the row backgrounds. Rows of notes in the key
/// are lighter, and the key's root is tinted, so the scale shows at a glance.
fn paint_rows(painter: &Painter, grid: Grid, scale: Scale) {
    for midi in LOWEST..=HIGHEST {
        let pitch_class = PitchClass::from_semitone(midi);
        let top = grid.row_top(midi);
        let in_scale = scale.contains(pitch_class);
        let is_root = pitch_class == scale.root;

        let row = Rect::from_min_size(
            pos2(grid.rect.left(), top),
            vec2(grid.rect.width(), ROW_HEIGHT),
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

        let key = Rect::from_min_size(pos2(grid.keys().left(), top), vec2(KEY_WIDTH, ROW_HEIGHT));
        let key_fill = if pitch_class.is_black_key() {
            BLACK_KEY
        } else {
            WHITE_KEY
        };
        painter.rect_filled(key.shrink2(vec2(0.0, 0.5)), 1.0, key_fill);
        if in_scale {
            let mark = if is_root { ROOT_MARK } else { SCALE_MARK };
            painter.circle_filled(pos2(key.right() - 7.0, key.center().y), 3.0, mark);
        }
        if pitch_class == PitchClass::C {
            painter.text(
                pos2(key.left() + 4.0, key.center().y),
                Align2::LEFT_CENTER,
                Note::new(midi).to_string(),
                FontId::proportional(10.0),
                KEY_TEXT,
            );
        }
    }
}

/// Vertical lines: faint for steps, stronger for beats, strongest for bars.
fn paint_step_lines(painter: &Painter, grid: Grid) {
    for step in 0..=grid.length {
        let color = match step {
            s if s % 16 == 0 => BAR_LINE,
            s if s % 4 == 0 => BEAT_LINE,
            _ => STEP_LINE,
        };
        painter.vline(grid.x(step), grid.rect.y_range(), Stroke::new(1.0, color));
    }
}
