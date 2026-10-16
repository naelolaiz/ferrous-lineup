//! The playlist: patterns arranged on tracks to make a song.
//!
//! Like the channel rack and the piano roll, [`PlaylistView::show`] only
//! draws and reads the mouse. It returns [`PlaylistAction`]s, and the app
//! changes the project.
//!
//! The playlist can be much wider and taller than the window, so it sits
//! in a scroll area. The track names on the left and the bar numbers at
//! the top stay in place while the clips scroll under them: they are
//! drawn at the edge of the *visible* part of the scroll area, which
//! `ScrollArea::show_viewport` tells us on every frame.

use eframe::egui::{
    self, Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    pos2, vec2,
};
use loom_core::{Clip, MAX_SONG_STEPS, MAX_TRACKS, Pattern, Project, STEPS_PER_BAR, TimeSignature};

use crate::library_browser::{LibraryAction, LibraryBrowser, Tried};
use crate::widgets;

/// Something the user asked the playlist to do.
#[derive(Debug, Clone, PartialEq)]
pub enum PlaylistAction {
    /// Make this the pattern the rack shows and new clips use.
    SelectPattern(usize),
    AddClip(Clip),
    MoveClip {
        index: usize,
        track: usize,
        start: usize,
    },
    RemoveClip(usize),
    ToggleTrackMute(usize),
    RenameTrack {
        track: usize,
        name: String,
    },
    AddTrack,
    /// Something picked in the Library tab.
    Library(LibraryAction),
}

/// The two tabs of the panel on the left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Patterns,
    Library,
}

const PICKER_WIDTH: f32 = 210.0;
const HEADER_WIDTH: f32 = 116.0;
const RULER_HEIGHT: f32 = 20.0;
const TRACK_HEIGHT: f32 = 36.0;
/// How wide one step is, in pixels. A bar of 4/4 is 16 steps: 72 pixels.
const STEP_WIDTH: f32 = 4.5;
/// The playlist is always at least as wide as this many bars of 4/4...
const MIN_BARS: usize = 32;
/// ...and has this many empty bars after the end of the song, to add more.
const EXTRA_BARS: usize = 16;
/// The height of the strip at the top of a clip with the pattern's name.
const CLIP_TITLE: f32 = 13.0;

const ROW: [Color32; 2] = [Color32::from_rgb(55, 64, 70), Color32::from_rgb(50, 58, 64)];
const BEAT_LINE: Color32 = Color32::from_rgb(45, 52, 57);
const BAR_LINE: Color32 = Color32::from_rgb(30, 35, 39);
const HEADER: Color32 = Color32::from_rgb(66, 76, 83);
const HEADER_TEXT: Color32 = Color32::from_rgb(226, 232, 236);
const RULER: Color32 = Color32::from_rgb(36, 42, 46);
const RULER_TEXT: Color32 = Color32::from_rgb(190, 200, 206);
const SONG_END: Color32 = Color32::from_rgb(170, 180, 186);
const PLAYHEAD: Color32 = Color32::from_rgb(118, 214, 108);
const CLIP_TEXT: Color32 = Color32::from_rgb(20, 22, 26);
const CLIP_NOTES: Color32 = Color32::from_rgb(232, 236, 242);
const SELECTED_OUTLINE: Color32 = Color32::from_rgb(245, 245, 250);

/// The colours patterns are drawn in, chosen by pattern number.
const PALETTE: [Color32; 8] = [
    Color32::from_rgb(98, 160, 234),
    Color32::from_rgb(232, 146, 84),
    Color32::from_rgb(124, 200, 124),
    Color32::from_rgb(206, 118, 196),
    Color32::from_rgb(226, 198, 92),
    Color32::from_rgb(98, 200, 200),
    Color32::from_rgb(222, 104, 104),
    Color32::from_rgb(160, 142, 232),
];

/// The colour of a pattern, in the picker and on its clips.
pub fn pattern_color(pattern: usize) -> Color32 {
    PALETTE[pattern % PALETTE.len()]
}

/// Where clips land when placed or moved: on a bar line, a beat, or any step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Snap {
    Bar,
    Beat,
    Step,
}

impl Snap {
    const ALL: [Snap; 3] = [Snap::Bar, Snap::Beat, Snap::Step];

    /// How many steps apart the snap points are, which depends on the
    /// time signature: a bar of 3/4 is 12 steps, a beat of 6/8 is 6.
    fn steps(self, time: TimeSignature) -> usize {
        match self {
            Snap::Bar => time.steps_per_bar(),
            Snap::Beat => time.steps_per_beat(),
            Snap::Step => 1,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Snap::Bar => "Bar",
            Snap::Beat => "Beat",
            Snap::Step => "Step",
        }
    }

    /// The snap point at or before `step`.
    fn floor(self, step: usize, time: TimeSignature) -> usize {
        let steps = self.steps(time);
        step / steps * steps
    }
}

/// A clip being dragged. `grab` is how many steps after the clip's start
/// the pointer took hold of it, so the clip does not jump under the pointer.
#[derive(Debug, Clone, Copy)]
struct ClipDrag {
    index: usize,
    grab: usize,
}

/// Where the grid of tracks and steps is on screen.
#[derive(Debug, Clone, Copy)]
struct Layout {
    /// The top-left corner of step 0 of track 0.
    origin: Pos2,
    tracks: usize,
}

impl Layout {
    fn x(self, step: usize) -> f32 {
        self.origin.x + step as f32 * STEP_WIDTH
    }

    fn track_top(self, track: usize) -> f32 {
        self.origin.y + track as f32 * TRACK_HEIGHT
    }

    fn clip_rect(self, clip: &Clip, length: usize) -> Rect {
        let top = self.track_top(clip.track);
        Rect::from_min_max(
            pos2(self.x(clip.start), top),
            pos2(self.x(clip.start + length), top + TRACK_HEIGHT),
        )
    }

    /// The track and step under a point, clamped to the grid.
    fn cell_at(self, pos: Pos2) -> (usize, usize) {
        let last_track = self.tracks.saturating_sub(1) as f32;
        let track = ((pos.y - self.origin.y) / TRACK_HEIGHT)
            .floor()
            .clamp(0.0, last_track) as usize;
        let step = ((pos.x - self.origin.x) / STEP_WIDTH)
            .floor()
            .clamp(0.0, (MAX_SONG_STEPS - 1) as f32) as usize;
        (track, step)
    }
}

/// What the playlist remembers between frames.
pub struct PlaylistView {
    snap: Snap,
    drag: Option<ClipDrag>,
    tab: Tab,
    library: LibraryBrowser,
    /// Whether the Patterns and Library panel on the left is shown.
    pub browser_open: bool,
}

impl PlaylistView {
    pub fn new() -> Self {
        PlaylistView {
            snap: Snap::Bar,
            drag: None,
            tab: Tab::Patterns,
            library: LibraryBrowser::new(),
            browser_open: true,
        }
    }

    /// Whether trying something from the library also sets its tempo.
    pub fn library_tempo(&self) -> bool {
        self.library.use_tempo
    }

    /// Draws the pattern picker and the playlist. `current` is the
    /// selected pattern; `playhead` is the song step being played, if
    /// the song is playing; `tried` is what the library put in the
    /// selected pattern, if it was the library.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        current: usize,
        playhead: Option<usize>,
        tried: Tried,
    ) -> Vec<PlaylistAction> {
        let mut actions = Vec::new();

        let mut browser_open = self.browser_open;
        egui::Panel::left("pattern-picker")
            .resizable(true)
            .default_size(PICKER_WIDTH)
            .size_range(140.0..=420.0)
            .show_collapsible(ui, &mut browser_open, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.tab, Tab::Patterns, "Patterns");
                    ui.selectable_value(&mut self.tab, Tab::Library, "📚 Library")
                        .on_hover_text("Harmony styles and rhythms by genre");
                });
                ui.separator();
                match self.tab {
                    Tab::Patterns => picker(ui, project, current, &mut actions),
                    Tab::Library => {
                        if let Some(action) = self.library.show(ui, project, tried) {
                            actions.push(PlaylistAction::Library(action));
                        }
                    }
                }
            });
        self.browser_open = browser_open;

        let current_name = project.pattern(current).map_or("", |p| p.name.as_str());
        ui.horizontal(|ui| {
            ui.heading("Playlist");
            ui.separator();
            ui.label("Snap");
            egui::ComboBox::from_id_salt("playlist-snap")
                .width(60.0)
                .selected_text(self.snap.label())
                .show_ui(ui, |ui| {
                    for snap in Snap::ALL {
                        ui.selectable_value(&mut self.snap, snap, snap.label());
                    }
                });
            let room = project.playlist().tracks().len() < MAX_TRACKS;
            if ui
                .add_enabled(room, egui::Button::new("➕ Track"))
                .clicked()
            {
                actions.push(PlaylistAction::AddTrack);
            }
            ui.separator();
            // Cut short with "…" when the window is narrow; the whole
            // sentence shows when you point at it.
            let help = format!(
                "Click to place \"{current_name}\". Drag clips to move them, right-click to delete."
            );
            ui.add(egui::Label::new(egui::RichText::new(&help).weak()).truncate());
        });

        egui::ScrollArea::both()
            .auto_shrink(false)
            .show_viewport(ui, |ui, viewport| {
                self.grid(ui, viewport, project, current, playhead, &mut actions)
            });
        actions
    }

    /// The tracks, the bar ruler and the clips.
    fn grid(
        &mut self,
        ui: &mut Ui,
        viewport: Rect,
        project: &Project,
        current: usize,
        playhead: Option<usize>,
        actions: &mut Vec<PlaylistAction>,
    ) {
        let tracks = project.playlist().tracks();
        let song_length = project.song_length();
        let time = project.time_signature;
        // The grid ends on a bar line, some empty bars after the song.
        let steps = time.round_up_to_bar(
            (song_length + EXTRA_BARS * time.steps_per_bar()).max(MIN_BARS * STEPS_PER_BAR),
        );
        let size = vec2(
            HEADER_WIDTH + steps as f32 * STEP_WIDTH,
            RULER_HEIGHT + tracks.len() as f32 * TRACK_HEIGHT,
        );
        let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

        // `viewport` is the visible part, measured from the top-left of the
        // content. Moving it by the content's position on screen gives the
        // visible part in screen coordinates, where everything is drawn.
        let visible = viewport.translate(rect.min.to_vec2());
        let layout = Layout {
            origin: pos2(rect.left() + HEADER_WIDTH, rect.top() + RULER_HEIGHT),
            tracks: tracks.len(),
        };
        // The part of the grid not hidden under the track names or the ruler.
        let cells = Rect::from_min_max(
            pos2(visible.left() + HEADER_WIDTH, visible.top() + RULER_HEIGHT),
            visible.max,
        );

        self.handle_input(ui, &response, layout, cells, project, current, actions);

        let painter = ui.painter_at(visible);
        paint_tracks(&painter, layout, steps, time);
        paint_clips(&painter, layout, cells, project, current);
        // Lines from the ruler down to the last track.
        let lines = cells.top()..=layout.track_top(tracks.len()).min(cells.bottom());
        if let Some(step) = playhead {
            painter.vline(layout.x(step), lines.clone(), Stroke::new(2.0, PLAYHEAD));
        }
        if song_length > 0 {
            let x = layout.x(song_length);
            painter.vline(x, lines, Stroke::new(1.0, SONG_END));
        }

        self.track_headers(ui, &painter, layout, visible, cells, project, actions);
        paint_ruler(
            &painter,
            layout,
            visible,
            steps,
            time,
            song_length,
            playhead,
        );
    }

    /// The column of track names on the left, which stays put when the
    /// playlist scrolls sideways.
    #[allow(clippy::too_many_arguments)]
    fn track_headers(
        &self,
        ui: &mut Ui,
        painter: &Painter,
        layout: Layout,
        visible: Rect,
        cells: Rect,
        project: &Project,
        actions: &mut Vec<PlaylistAction>,
    ) {
        let column = Rect::from_min_max(
            pos2(visible.left(), cells.top()),
            pos2(visible.left() + HEADER_WIDTH, visible.bottom()),
        );
        painter.rect_filled(column, 0.0, HEADER);

        for (index, track) in project.playlist().tracks().iter().enumerate() {
            let top = layout.track_top(index);
            let row =
                Rect::from_min_size(pos2(column.left(), top), vec2(HEADER_WIDTH, TRACK_HEIGHT));
            // Only rows fully below the ruler get their buttons, so a row
            // half-hidden under the ruler cannot be clicked through it.
            if row.top() < cells.top() || row.top() > visible.bottom() {
                continue;
            }
            painter.hline(row.x_range(), row.bottom(), Stroke::new(1.0, ROW[1]));

            // Real widgets, placed with `ui.put` at a rectangle we choose:
            // a light to mute the track, and a frameless text field to rename it.
            let light = Rect::from_min_size(row.min + vec2(4.0, 6.0), vec2(14.0, 24.0));
            let enabled = !track.muted;
            let light_response = ui
                .put(light, |ui: &mut Ui| widgets::mute_light(ui, enabled))
                .on_hover_text("Click to mute or unmute the track");
            if light_response.clicked() {
                actions.push(PlaylistAction::ToggleTrackMute(index));
            }

            let name_rect = Rect::from_min_max(
                pos2(light.right() + 4.0, row.top() + 8.0),
                pos2(row.right() - 6.0, row.bottom() - 8.0),
            );
            let mut name = track.name.clone();
            let field = egui::TextEdit::singleline(&mut name)
                .frame(egui::Frame::NONE)
                .text_color(HEADER_TEXT);
            if ui
                .put(name_rect, field)
                .on_hover_text("Click to rename")
                .changed()
            {
                actions.push(PlaylistAction::RenameTrack { track: index, name });
            }
        }
    }

    /// Reads the mouse over the grid.
    ///
    /// - left click on empty space: place a clip of the selected pattern;
    ///   keep the button down to drag it
    /// - left drag a clip: move it (and select its pattern)
    /// - right click (or right drag over) clips: delete them
    #[allow(clippy::too_many_arguments)]
    fn handle_input(
        &mut self,
        ui: &Ui,
        response: &Response,
        layout: Layout,
        cells: Rect,
        project: &Project,
        current: usize,
        actions: &mut Vec<PlaylistAction>,
    ) {
        let clips = project.playlist().clips();
        let (primary_pressed, primary_down, secondary, pointer, press_origin) = ui.input(|input| {
            let pointer = &input.pointer;
            (
                pointer.primary_pressed(),
                pointer.primary_down(),
                pointer.secondary_pressed() || pointer.secondary_down(),
                pointer.interact_pos(),
                pointer.press_origin(),
            )
        });

        // A press is handled where the button went down. A quick hand can
        // press and move in the same frame, and the clip or empty cell
        // under the pointer *now* may not be the one that was clicked.
        let pressed_at = press_origin
            .filter(|pos| primary_pressed && response.hovered() && cells.contains(*pos));
        if let Some(pos) = pressed_at {
            let (track, step) = layout.cell_at(pos);
            let time = project.time_signature;
            match clip_at(project, layout, pos) {
                Some(index) => {
                    let clip = clips[index];
                    self.drag = Some(ClipDrag {
                        index,
                        grab: step.saturating_sub(clip.start),
                    });
                    if clip.pattern != current {
                        actions.push(PlaylistAction::SelectPattern(clip.pattern));
                    }
                }
                None => {
                    let start = self.snap.floor(step, time);
                    actions.push(PlaylistAction::AddClip(Clip::new(current, track, start)));
                    // The new clip goes at the end of the list.
                    self.drag = Some(ClipDrag {
                        index: clips.len(),
                        grab: step - start,
                    });
                }
            }
        }

        // Deleting follows the pointer, so a right-drag sweeps clips away.
        let hovered = response.hover_pos().filter(|pos| cells.contains(*pos));
        if secondary
            && let Some(pos) = hovered
            && let Some(index) = clip_at(project, layout, pos)
        {
            actions.push(PlaylistAction::RemoveClip(index));
        }

        if !primary_down {
            self.drag = None;
        }
        if let Some(drag) = self.drag
            && let Some(pos) = pointer
            && let Some(clip) = clips.get(drag.index)
        {
            let (track, step) = layout.cell_at(pos);
            let start = self
                .snap
                .floor(step.saturating_sub(drag.grab), project.time_signature);
            if (track, start) != (clip.track, clip.start) {
                actions.push(PlaylistAction::MoveClip {
                    index: drag.index,
                    track,
                    start,
                });
            }
        }
    }
}

/// The list of patterns on the left. Clicking one selects it: the channel
/// rack shows it, pattern mode plays it and new clips use it.
fn picker(ui: &mut Ui, project: &Project, current: usize, actions: &mut Vec<PlaylistAction>) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (index, pattern) in project.patterns().iter().enumerate() {
            ui.horizontal(|ui| {
                let (swatch, _) = ui.allocate_exact_size(vec2(10.0, 16.0), Sense::hover());
                ui.painter()
                    .rect_filled(swatch.shrink2(vec2(0.0, 2.0)), 2.0, pattern_color(index));
                let label = ui.selectable_label(index == current, &pattern.name);
                if label.clicked() && index != current {
                    actions.push(PlaylistAction::SelectPattern(index));
                }
            });
        }
    });
}

/// The index of the clip under `pos`. Later clips are drawn on top, so
/// they are searched first.
fn clip_at(project: &Project, layout: Layout, pos: Pos2) -> Option<usize> {
    let clips = project.playlist().clips();
    (0..clips.len()).rev().find(|&index| {
        let clip = &clips[index];
        layout
            .clip_rect(clip, project.clip_length(clip))
            .contains(pos)
    })
}

/// Row backgrounds and the lines for beats and bars, `steps` long.
fn paint_tracks(painter: &Painter, layout: Layout, steps: usize, time: TimeSignature) {
    let right = layout.x(steps);
    for track in 0..layout.tracks {
        let top = layout.track_top(track);
        let row = Rect::from_min_max(pos2(layout.origin.x, top), pos2(right, top + TRACK_HEIGHT));
        painter.rect_filled(row, 0.0, ROW[track % 2]);
    }
    let bottom = layout.track_top(layout.tracks);
    for step in (0..=steps).filter(|&step| time.is_beat_start(step)) {
        let color = if time.is_bar_start(step) {
            BAR_LINE
        } else {
            BEAT_LINE
        };
        painter.vline(
            layout.x(step),
            layout.origin.y..=bottom,
            Stroke::new(1.0, color),
        );
    }
}

/// Every clip: a box in its pattern's colour, its name, and a sketch of
/// the notes inside.
fn paint_clips(painter: &Painter, layout: Layout, cells: Rect, project: &Project, current: usize) {
    let tracks = project.playlist().tracks();
    for clip in project.playlist().clips() {
        let length = project.clip_length(clip);
        let rect = layout.clip_rect(clip, length).shrink2(vec2(0.5, 1.5));
        let Some(pattern) = project.pattern(clip.pattern) else {
            continue;
        };
        if !rect.intersects(cells) {
            continue;
        }
        let mut color = pattern_color(clip.pattern);
        if tracks.get(clip.track).is_some_and(|track| track.muted) {
            color = color.gamma_multiply(0.35);
        }

        painter.rect_filled(rect, 3.0, color.gamma_multiply(0.45));
        let title = Rect::from_min_size(rect.min, vec2(rect.width(), CLIP_TITLE));
        painter.rect_filled(title, 3.0, color);
        painter.with_clip_rect(title.intersect(cells)).text(
            title.left_center() + vec2(3.0, 0.0),
            Align2::LEFT_CENTER,
            &pattern.name,
            FontId::proportional(10.5),
            CLIP_TEXT,
        );
        let body = Rect::from_min_max(pos2(rect.left(), title.bottom()), rect.max).shrink(2.0);
        paint_clip_notes(&painter.with_clip_rect(cells), body, pattern);

        if clip.pattern == current {
            painter.rect_stroke(
                rect,
                3.0,
                Stroke::new(1.0, SELECTED_OUTLINE),
                StrokeKind::Inside,
            );
        }
    }
}

/// A sketch of a pattern's notes inside a clip.
///
/// A drum pattern (every channel plays plain steps) is drawn like a tiny
/// channel rack: one line per channel that has notes. Anything else is
/// drawn like a tiny piano roll: higher notes higher up.
fn paint_clip_notes(painter: &Painter, body: Rect, pattern: &Pattern) {
    let length = pattern.length();
    let notes = || pattern.all_notes().filter(|(_, note)| note.start < length);
    let channels = pattern.rows();
    let steps_only = (0..channels).all(|channel| pattern.is_step_row(channel));

    // Turns a channel and a note into a height from 0.0 (bottom) to 1.0 (top).
    let height: Box<dyn Fn(usize, u8) -> f32> = if steps_only {
        let used: Vec<usize> = (0..channels)
            .filter(|&channel| !pattern.notes(channel).is_empty())
            .collect();
        let count = used.len().max(1) as f32;
        Box::new(move |channel, _| {
            let row = used.iter().position(|&c| c == channel).unwrap_or(0) as f32;
            1.0 - (row + 0.5) / count
        })
    } else {
        let lowest = notes().map(|(_, n)| n.pitch.midi()).min().unwrap_or(0);
        let highest = notes().map(|(_, n)| n.pitch.midi()).max().unwrap_or(0);
        let span = (highest - lowest).max(12) as f32;
        let centering = (span - (highest - lowest) as f32) / 2.0;
        Box::new(move |_, midi| ((midi - lowest) as f32 + centering) / span)
    };

    for (channel, note) in notes() {
        let y = body.bottom() - height(channel, note.pitch.midi()) * body.height();
        let x_start = body.left() + note.start as f32 * STEP_WIDTH;
        let held = note.length.min(length - note.start) as f32;
        let x_end = (x_start + held * STEP_WIDTH - 1.0).max(x_start + 1.5);
        painter.line_segment(
            [pos2(x_start, y), pos2(x_end, y)],
            Stroke::new(1.5, CLIP_NOTES),
        );
    }
}

/// The strip at the top with bar numbers, which stays put when the
/// playlist scrolls down.
fn paint_ruler(
    painter: &Painter,
    layout: Layout,
    visible: Rect,
    steps: usize,
    time: TimeSignature,
    song_length: usize,
    playhead: Option<usize>,
) {
    let ruler = Rect::from_min_size(visible.min, vec2(visible.width(), RULER_HEIGHT));
    painter.rect_filled(ruler, 0.0, RULER);
    let ruler_painter = painter.with_clip_rect(Rect::from_min_max(
        pos2(visible.left() + HEADER_WIDTH, ruler.top()),
        ruler.max,
    ));
    let bar_steps = time.steps_per_bar();
    for bar in 0..steps / bar_steps {
        let x = layout.x(bar * bar_steps);
        let color = if bar * bar_steps < song_length {
            HEADER_TEXT
        } else {
            RULER_TEXT
        };
        ruler_painter.vline(x, ruler.y_range(), Stroke::new(1.0, BAR_LINE));
        ruler_painter.text(
            pos2(x + 3.0, ruler.center().y),
            Align2::LEFT_CENTER,
            (bar + 1).to_string(),
            FontId::proportional(11.0),
            color,
        );
    }
    if let Some(step) = playhead {
        let x = layout.x(step);
        let tip = pos2(x, ruler.bottom());
        ruler_painter.add(egui::Shape::convex_polygon(
            vec![tip, tip + vec2(-5.0, -8.0), tip + vec2(5.0, -8.0)],
            PLAYHEAD,
            Stroke::NONE,
        ));
    }
    // The corner above the track names.
    let corner = Rect::from_min_size(visible.min, vec2(HEADER_WIDTH, RULER_HEIGHT));
    painter.rect_filled(corner, 0.0, RULER);
    painter.text(
        pos2(corner.left() + 6.0, corner.center().y),
        Align2::LEFT_CENTER,
        "Tracks",
        FontId::proportional(11.0),
        RULER_TEXT,
    );
}
