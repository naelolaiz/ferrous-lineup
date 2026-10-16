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
//!
//! Above the bar numbers is the section strip, where the song's structure
//! is marked; [`sections`] draws it and the loop region.

mod sections;

use eframe::egui::{
    self, Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    pos2, vec2,
};
use loom_core::{
    Clip, LoopRegion, MAX_REPEATS, MAX_SONG_STEPS, MAX_TRACKS, Pass, Pattern, Project,
    STEPS_PER_BAR, Section, TimeSignature,
};

use crate::library_browser::{LibraryAction, LibraryBrowser, Tried};
use crate::widgets;
use sections::Strips;

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
    /// Play the clip's pattern this many times in a row.
    SetClipRepeats {
        index: usize,
        repeats: usize,
    },
    /// Open the editor for a clip's pattern (a double-click on the clip).
    EditClip(usize),
    ToggleTrackMute(usize),
    RenameTrack {
        track: usize,
        name: String,
    },
    AddTrack,
    /// Insert a new, empty track at this row.
    InsertTrack(usize),
    /// Delete a track and its clips.
    RemoveTrack(usize),
    /// Swap a track with the one below it.
    SwapTracks(usize),
    /// Move the playhead to this step of the playlist's timeline.
    Seek(usize),
    AddSection(Section),
    /// Rename, move, resize or repeat a section.
    SetSection {
        index: usize,
        section: Section,
    },
    RemoveSection(usize),
    DuplicateSection(usize),
    /// Turn a section's repeats into separate copies.
    UnrollSection(usize),
    /// Loop part of the song, or (with `None`) the whole song again.
    SetLoop(Option<LoopRegion>),
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
/// The strip above the bar numbers with the song's sections.
const SECTION_HEIGHT: f32 = 22.0;
/// How close to a clip's right edge the pointer must be to repeat it.
const CLIP_EDGE: f32 = 6.0;
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
/// The lines between the times round of a clip that repeats.
const REPEAT_LINE: Color32 = Color32::from_rgb(20, 22, 26);

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
/// A clip taken by its right edge is not moved but stretched: it repeats
/// its pattern as many times as fit up to the pointer.
#[derive(Debug, Clone, Copy)]
struct ClipDrag {
    index: usize,
    grab: usize,
    stretch: bool,
}

/// Where the grid of tracks and steps is on screen.
#[derive(Debug, Clone, Copy)]
struct Layout {
    /// The top-left corner of step 0 of track 0.
    origin: Pos2,
    tracks: usize,
    /// Whether there is an empty row below the last track, where a clip
    /// starts a new track. There is unless the playlist is full.
    spare: bool,
}

impl Layout {
    fn x(self, step: usize) -> f32 {
        self.origin.x + step as f32 * STEP_WIDTH
    }

    /// The step under `x`, clamped to the song.
    fn step_at(self, x: f32) -> usize {
        ((x - self.origin.x) / STEP_WIDTH)
            .floor()
            .clamp(0.0, (MAX_SONG_STEPS - 1) as f32) as usize
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

    /// The rows drawn: the tracks, and the spare row if there is one.
    fn rows(self) -> usize {
        self.tracks + usize::from(self.spare)
    }

    /// The track and step under a point, clamped to the grid. Below the
    /// last track that is the spare row, the track a clip there makes.
    fn cell_at(self, pos: Pos2) -> (usize, usize) {
        let last_track = self.rows().saturating_sub(1) as f32;
        let track = ((pos.y - self.origin.y) / TRACK_HEIGHT)
            .floor()
            .clamp(0.0, last_track) as usize;
        (track, self.step_at(pos.x))
    }
}

/// What the playlist remembers between frames.
pub struct PlaylistView {
    snap: Snap,
    drag: Option<ClipDrag>,
    strips: Strips,
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
            strips: Strips::default(),
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
    /// selected pattern; `playhead` is the step of the playlist being
    /// played, and which time round, if the song is playing; `tried` is
    /// what the library put in the selected pattern, if it was the library.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        current: usize,
        playhead: Option<(usize, Pass)>,
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
            if ui
                .button("➕ Section")
                .on_hover_text("Mark the next stretch of the song as a section")
                .clicked()
            {
                actions.push(PlaylistAction::AddSection(next_section(project)));
            }
            if let Some(region) = project.playlist().loop_region() {
                let bars =
                    |step: usize| step as f32 / project.time_signature.steps_per_bar() as f32;
                let text = egui::RichText::new(format!(
                    "🔁 Looping bars {}–{}  ✖",
                    bars(region.start) + 1.0,
                    bars(region.end)
                ))
                .color(sections::LOOP);
                if ui
                    .button(text)
                    .on_hover_text("Song mode loops this part. Click to play the whole song again.")
                    .clicked()
                {
                    actions.push(PlaylistAction::SetLoop(None));
                }
            }
            ui.separator();
            // Cut short with "…" when the window is narrow; the whole
            // sentence shows when you point at it.
            let help = format!(
                "Click to place \"{current_name}\". Double-click a clip to edit it, drag to \
                 move it, drag its right edge to repeat it, right-click to delete. Drag along \
                 the top strip to mark a section, along the bar numbers to loop."
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
        playhead: Option<(usize, Pass)>,
        actions: &mut Vec<PlaylistAction>,
    ) {
        let pass = playhead.map(|(_, pass)| pass);
        let playhead = playhead.map(|(step, _)| step);
        let tracks = project.playlist().tracks();
        let spare = tracks.len() < MAX_TRACKS;
        let rows = tracks.len() + usize::from(spare);
        let song_length = project.song_length();
        let time = project.time_signature;
        // The grid ends on a bar line, some empty bars after the song.
        let steps = time.round_up_to_bar(
            (song_length + EXTRA_BARS * time.steps_per_bar()).max(MIN_BARS * STEPS_PER_BAR),
        );
        let size = vec2(
            HEADER_WIDTH + steps as f32 * STEP_WIDTH,
            SECTION_HEIGHT + RULER_HEIGHT + rows as f32 * TRACK_HEIGHT,
        );
        let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

        // `viewport` is the visible part, measured from the top-left of the
        // content. Moving it by the content's position on screen gives the
        // visible part in screen coordinates, where everything is drawn.
        let visible = viewport.translate(rect.min.to_vec2());
        let layout = Layout {
            origin: pos2(
                rect.left() + HEADER_WIDTH,
                rect.top() + SECTION_HEIGHT + RULER_HEIGHT,
            ),
            tracks: tracks.len(),
            spare,
        };
        // The part of the grid not hidden under the track names, the
        // sections or the ruler.
        let cells = Rect::from_min_max(
            pos2(
                visible.left() + HEADER_WIDTH,
                visible.top() + SECTION_HEIGHT + RULER_HEIGHT,
            ),
            visible.max,
        );

        self.handle_input(ui, &response, layout, cells, project, current, actions);

        let painter = ui.painter_at(visible);
        paint_tracks(&painter, layout, steps, time);
        if let Some(region) = project.playlist().loop_region() {
            // A light shade over the part of the song that loops.
            let shade = Rect::from_x_y_ranges(
                layout.x(region.start)..=layout.x(region.end),
                cells.top()..=layout.track_top(rows).min(cells.bottom()),
            );
            painter.rect_filled(shade, 0.0, sections::LOOP.gamma_multiply(0.06));
        }
        paint_clips(&painter, layout, cells, project, current);
        // Lines from the ruler down to the last track.
        let lines = cells.top()..=layout.track_top(rows).min(cells.bottom());
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

        // The two strips at the top, right of the corner.
        let left = visible.left() + HEADER_WIDTH;
        let strip = Rect::from_min_max(
            pos2(left, visible.top()),
            pos2(visible.right(), visible.top() + SECTION_HEIGHT),
        );
        let ruler = Rect::from_min_max(
            pos2(left, strip.bottom()),
            pos2(visible.right(), strip.bottom() + RULER_HEIGHT),
        );
        self.strips
            .sections(ui, layout, strip, project, self.snap, pass, actions);
        self.strips
            .loop_ruler(ui, layout, ruler, project, self.snap, actions);
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
            let name_response = ui
                .put(name_rect, field)
                .on_hover_text("Click to rename. Right-click to insert, delete or move tracks");
            if name_response.changed() {
                actions.push(PlaylistAction::RenameTrack { track: index, name });
            }
            let tracks = project.playlist().tracks().len();
            name_response.context_menu(|ui| {
                if ui.button("Insert track above").clicked() {
                    actions.push(PlaylistAction::InsertTrack(index));
                }
                if ui.button("Insert track below").clicked() {
                    actions.push(PlaylistAction::InsertTrack(index + 1));
                }
                ui.separator();
                if ui
                    .add_enabled(index > 0, egui::Button::new("Move up"))
                    .clicked()
                {
                    actions.push(PlaylistAction::SwapTracks(index - 1));
                }
                if ui
                    .add_enabled(index + 1 < tracks, egui::Button::new("Move down"))
                    .clicked()
                {
                    actions.push(PlaylistAction::SwapTracks(index));
                }
                ui.separator();
                if ui
                    .add_enabled(tracks > 1, egui::Button::new("Delete track"))
                    .on_hover_text("Deletes its clips too. Undo brings them back")
                    .clicked()
                {
                    actions.push(PlaylistAction::RemoveTrack(index));
                }
            });
        }

        // The spare row's header: a button that adds the track, for those
        // who would rather not place a clip there to make one.
        if layout.spare {
            let top = layout.track_top(layout.tracks);
            let row =
                Rect::from_min_size(pos2(column.left(), top), vec2(HEADER_WIDTH, TRACK_HEIGHT));
            if row.top() >= cells.top() && row.top() <= visible.bottom() {
                let button = egui::Button::new(
                    egui::RichText::new("➕ New track").color(HEADER_TEXT.gamma_multiply(0.6)),
                )
                .frame(false);
                if ui
                    .put(row.shrink2(vec2(6.0, 6.0)), button)
                    .on_hover_text("Add a track here, or place a clip on this row to add one")
                    .clicked()
                {
                    actions.push(PlaylistAction::AddTrack);
                }
            }
        }
    }

    /// Reads the mouse over the grid.
    ///
    /// - left click on empty space: place a clip of the selected pattern;
    ///   keep the button down to drag it. On the spare row below the last
    ///   track, the clip adds a track
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
                    let rect = layout.clip_rect(&clip, project.clip_length(&clip));
                    self.drag = Some(ClipDrag {
                        index,
                        grab: step.saturating_sub(clip.start),
                        stretch: pos.x > rect.right() - CLIP_EDGE,
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
                        stretch: false,
                    });
                }
            }
        }

        // A double-click opens the clip's pattern in its editor.
        let double_clicked = ui.input(|input| {
            input
                .pointer
                .button_double_clicked(egui::PointerButton::Primary)
        });
        if double_clicked
            && response.hovered()
            && let Some(pos) = pointer.filter(|pos| cells.contains(*pos))
            && let Some(index) = clip_at(project, layout, pos)
        {
            actions.push(PlaylistAction::EditClip(clips[index].pattern));
        }

        // Deleting follows the pointer, so a right-drag sweeps clips away.
        let hovered = response.hover_pos().filter(|pos| cells.contains(*pos));
        if secondary
            && let Some(pos) = hovered
            && let Some(index) = clip_at(project, layout, pos)
        {
            actions.push(PlaylistAction::RemoveClip(index));
        }

        // The right edge of a clip stretches it: show it with the cursor.
        if let Some(pos) = hovered
            && self.drag.is_none_or(|drag| drag.stretch)
            && clip_at(project, layout, pos).is_some_and(|index| {
                let clip = &clips[index];
                pos.x > layout.clip_rect(clip, project.clip_length(clip)).right() - CLIP_EDGE
            })
        {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        if !primary_down {
            self.drag = None;
        }
        if let Some(drag) = self.drag
            && drag.stretch
            && let Some(pos) = pointer
            && let Some(clip) = clips.get(drag.index)
        {
            // As many whole patterns as reach the pointer, the nearest.
            let pattern_length = project.pattern(clip.pattern).map_or(0, Pattern::length);
            let reach = (pos.x - layout.x(clip.start)) / STEP_WIDTH;
            let repeats = (reach / pattern_length.max(1) as f32)
                .round()
                .clamp(1.0, MAX_REPEATS as f32) as usize;
            if repeats != clip.repeats {
                actions.push(PlaylistAction::SetClipRepeats {
                    index: drag.index,
                    repeats,
                });
            }
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        } else if let Some(drag) = self.drag
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

/// The section the "➕ Section" button adds: four bars (or up to the end
/// of the song, if that is sooner) right after the last section.
fn next_section(project: &Project) -> Section {
    let playlist = project.playlist();
    let bar = project.time_signature.steps_per_bar();
    let start = playlist.sections().last().map_or(0, Section::end);
    let song_end = project.song_length();
    let length = if song_end > start {
        (song_end - start).min(4 * bar)
    } else {
        4 * bar
    };
    Section::new(playlist.suggested_section_name(), start, length)
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
    for track in 0..layout.rows() {
        let top = layout.track_top(track);
        let row = Rect::from_min_max(pos2(layout.origin.x, top), pos2(right, top + TRACK_HEIGHT));
        let color = ROW[track % 2];
        // The spare row is fainter: it is not a track yet.
        let color = if track < layout.tracks {
            color
        } else {
            color.gamma_multiply(0.5)
        };
        painter.rect_filled(row, 0.0, color);
    }
    let bottom = layout.track_top(layout.rows());
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
        let name = if clip.repeats > 1 {
            format!("{}  ×{}", pattern.name, clip.repeats)
        } else {
            pattern.name.clone()
        };
        painter.with_clip_rect(title.intersect(cells)).text(
            title.left_center() + vec2(3.0, 0.0),
            Align2::LEFT_CENTER,
            name,
            FontId::proportional(10.5),
            CLIP_TEXT,
        );
        // The notes, once for each time round, with a line between them.
        let round = pattern.length() as f32 * STEP_WIDTH;
        let notes_painter = painter.with_clip_rect(cells.intersect(rect));
        for repeat in 0..clip.repeats {
            let left = rect.left() + repeat as f32 * round;
            if repeat > 0 {
                notes_painter.vline(left, rect.y_range(), Stroke::new(1.0, REPEAT_LINE));
            }
            let body = Rect::from_min_max(
                pos2(left, title.bottom()),
                pos2(left + round, rect.bottom()),
            )
            .shrink(2.0);
            if body.intersects(cells) {
                paint_clip_notes(&notes_painter, body, pattern);
            }
        }

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
    let ruler = Rect::from_min_size(
        visible.min + vec2(0.0, SECTION_HEIGHT),
        vec2(visible.width(), RULER_HEIGHT),
    );
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
    // The corner above the track names, which names the two strips.
    let corner = Rect::from_min_size(
        visible.min,
        vec2(HEADER_WIDTH, SECTION_HEIGHT + RULER_HEIGHT),
    );
    painter.rect_filled(corner, 0.0, RULER);
    painter.text(
        pos2(corner.left() + 6.0, corner.top() + SECTION_HEIGHT / 2.0),
        Align2::LEFT_CENTER,
        "Sections",
        FontId::proportional(11.0),
        RULER_TEXT,
    );
    painter.text(
        pos2(corner.left() + 6.0, ruler.center().y),
        Align2::LEFT_CENTER,
        "Bars · Tracks",
        FontId::proportional(11.0),
        RULER_TEXT,
    );
}
