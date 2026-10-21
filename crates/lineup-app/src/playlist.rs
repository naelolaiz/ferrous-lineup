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
//! is marked; [`sections`] draws it and the loop region. A right-click on
//! the bar numbers inserts an empty bar or deletes that one, and changes
//! the time signature or the key from that bar on, the way a chart writes
//! a new time signature or key signature where it changes.

mod sections;
mod tempo;

use eframe::egui::{
    self, Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    Vec2, pos2, vec2,
};
use lineup_core::band::VariationMarker;
use lineup_core::swing::swings_in;
use lineup_core::{
    Clip, KeyMap, KeyMarker, LoopRegion, MAX_LOOSE_TIMING, MAX_REPEATS, MAX_SONG_STEPS, MAX_SWING,
    MAX_TRACKS, Meter, Pass, Pattern, Project, STEPS_PER_BAR, Section, TempoMarker, TimeMarker,
};

use crate::library_browser::{LibraryAction, LibraryBrowser, Tried};
use crate::{theme, widgets};
use sections::Strips;
use tempo::TempoStrip;

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
    MoveClips {
        indices: Vec<usize>,
        tracks: isize,
        steps: isize,
    },
    SelectSection(Option<LoopRegion>),
    RemoveClip(usize),
    RemoveClips(Vec<usize>),
    PasteClips(Vec<Clip>),
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
    ResizeTrack {
        track: usize,
        height: u16,
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
    /// Insert or delete bars (a right-click on the bar numbers).
    Bars(crate::chord_sheet::BarEdit),
    /// Change the time signature from a bar on (a right-click on the bar
    /// numbers).
    SetTimeMarker(TimeMarker),
    /// Take away the change of time signature on this step.
    RemoveTimeMarker(usize),
    /// Change the key from a bar on, without moving any note (a
    /// right-click on the bar numbers).
    SetKeyMarker(KeyMarker),
    /// Take away the change of key on this step.
    RemoveKeyMarker(usize),
    /// Change the tempo from a step on (a click on the tempo strip).
    SetTempoMarker(TempoMarker),
    /// Take away the change of tempo on this step.
    RemoveTempoMarker(usize),
    /// How loose the timing is, in milliseconds.
    SetLooseTiming(u8),
    /// How much the eighth notes swing, from 0 to [`MAX_SWING`].
    SetSwing(u8),
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
/// The strip at the top with the song's sections.
const SECTION_HEIGHT: f32 = 22.0;
/// The strip under it with the tempo.
const TEMPO_HEIGHT: f32 = 28.0;
/// The strips at the top, which stay put while the tracks scroll.
const TOP: f32 = SECTION_HEIGHT + TEMPO_HEIGHT + RULER_HEIGHT;
/// How close to a clip's right edge the pointer must be to repeat it.
const CLIP_EDGE: f32 = 6.0;
/// How tall a track is at 100%. The vertical zoom multiplies it.
const TRACK_HEIGHT: f32 = 36.0;
/// How wide one step is at 100%, in pixels. A bar of 4/4 is 16 steps:
/// 72 pixels. The horizontal zoom multiplies it.
const STEP_WIDTH: f32 = 4.5;
/// The narrowest a bar number may be written in: when bars are narrower,
/// only every second, fourth or eighth bar is numbered.
const BAR_NUMBER_WIDTH: f32 = 26.0;
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
/// The time signatures written on the bar numbers.
const TIME_TEXT: Color32 = Color32::from_rgb(236, 190, 98);
/// The colour of the key names on the ruler.
const KEY_TEXT: Color32 = Color32::from_rgb(140, 196, 250);
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

/// The colour of pattern number `pattern`, in the picker and on its
/// clips. Each pattern keeps its own (see [`lineup_core::Pattern::color`]).
pub fn pattern_color(project: &Project, pattern: usize) -> Color32 {
    let color = project.pattern(pattern).map_or(pattern, |p| p.color);
    PALETTE[color % PALETTE.len()]
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

    fn label(self) -> &'static str {
        match self {
            Snap::Bar => "Bar",
            Snap::Beat => "Beat",
            Snap::Step => "Step",
        }
    }

    /// The snap point at or before `step`. Where the bars and beats are
    /// depends on the time signature there: a bar of 3/4 is 12 steps, a
    /// beat of 6/8 is 6.
    fn floor(self, step: usize, meter: &Meter) -> usize {
        match self {
            Snap::Bar => meter.floor_to_bar(step),
            Snap::Beat => meter.floor_to_beat(step),
            Snap::Step => step,
        }
    }

    /// The snap point after the one at or before `step`: the end of the
    /// bar, beat or step that `step` is in.
    fn next(self, step: usize, meter: &Meter) -> usize {
        let bar = meter.bar_at(step);
        match self {
            Snap::Bar => bar.end(),
            Snap::Beat => meter.beat_end(step).min(bar.end()),
            Snap::Step => step + 1,
        }
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
struct Layout<'a> {
    /// The top-left corner of step 0 of track 0.
    origin: Pos2,
    /// How wide a step is and how tall a track is: the zoom.
    step_width: f32,
    track_tops: &'a [f32],
    tracks: usize,
    /// Whether there is an empty row below the last track, where a clip
    /// starts a new track. There is unless the playlist is full.
    spare: bool,
}

impl Layout<'_> {
    fn x(self, step: usize) -> f32 {
        self.origin.x + step as f32 * self.step_width
    }

    /// The step under `x`, clamped to the song.
    fn step_at(self, x: f32) -> usize {
        ((x - self.origin.x) / self.step_width)
            .floor()
            .clamp(0.0, (MAX_SONG_STEPS - 1) as f32) as usize
    }

    fn track_top(self, track: usize) -> f32 {
        self.origin.y
            + self
                .track_tops
                .get(track)
                .copied()
                .unwrap_or_else(|| *self.track_tops.last().unwrap_or(&0.0))
    }

    fn track_height(self, track: usize) -> f32 {
        self.track_top(track + 1) - self.track_top(track)
    }

    fn clip_rect(self, clip: &Clip, length: usize) -> Rect {
        let top = self.track_top(clip.track);
        Rect::from_min_max(
            pos2(self.x(clip.start), top),
            pos2(self.x(clip.start + length), self.track_top(clip.track + 1)),
        )
    }

    /// The rows drawn: the tracks, and the spare row if there is one.
    fn rows(self) -> usize {
        self.tracks + usize::from(self.spare)
    }

    /// The track and step under a point, clamped to the grid. Below the
    /// last track that is the spare row, the track a clip there makes.
    fn cell_at(self, pos: Pos2) -> (usize, usize) {
        let track = self
            .track_tops
            .partition_point(|top| *top <= pos.y - self.origin.y)
            .saturating_sub(1)
            .min(self.rows().saturating_sub(1));
        (track, self.step_at(pos.x))
    }
}

/// What the playlist remembers between frames.
pub struct PlaylistView {
    selected: std::collections::BTreeSet<usize>,
    shown_clips: Vec<Clip>,
    selection_box: Option<Pos2>,
    clipboard: Vec<Clip>,
    paste_at: usize,
    snap: Snap,
    drag: Option<ClipDrag>,
    strips: Strips,
    tempo_strip: TempoStrip,
    tab: Tab,
    library: LibraryBrowser,
    /// Whether the Patterns and Library panel on the left is shown.
    pub browser_open: bool,
    /// How much wider and taller than usual steps and tracks are: the
    /// horizontal and vertical zoom.
    zoom: Vec2,
    /// How far the grid was scrolled last frame, and where on screen its
    /// view was: to keep what is under the pointer in place when zooming.
    scrolled: Vec2,
    view_rect: Option<Rect>,
    /// Where to scroll the grid to the next time it is drawn.
    scroll_to: Option<Vec2>,
    /// An explicit transport jump is revealed even with playback following off.
    reveal: Option<usize>,
    /// Where on screen the playlist was drawn last frame, beside the
    /// Patterns and Library panel: where the zoom keys zoom the playlist.
    area: Rect,
}

impl PlaylistView {
    pub fn new() -> Self {
        PlaylistView {
            selected: Default::default(),
            shown_clips: Vec::new(),
            selection_box: None,
            clipboard: Vec::new(),
            paste_at: 0,
            snap: Snap::Bar,
            drag: None,
            strips: Strips::default(),
            tempo_strip: TempoStrip::default(),
            tab: Tab::Patterns,
            library: LibraryBrowser::new(),
            browser_open: true,
            zoom: Vec2::splat(1.0),
            scrolled: Vec2::ZERO,
            view_rect: None,
            scroll_to: None,
            reveal: None,
            area: Rect::NOTHING,
        }
    }

    /// Forgets the clips selected, being dragged or copied: they were
    /// another song's.
    pub fn new_song(&mut self) {
        self.selected.clear();
        self.shown_clips.clear();
        self.selection_box = None;
        self.clipboard.clear();
        self.paste_at = 0;
        self.drag = None;
    }

    /// Changes the zoom, keeping the step and the track under the pointer
    /// where they are, or those in the middle of the view if the pointer
    /// is elsewhere.
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
        // The track names, the sections and the ruler do not stretch.
        let corner = vec2(HEADER_WIDTH, TOP);
        let from = self.scroll_to.unwrap_or(self.scrolled);
        let to = corner + (from + anchor - corner) * ratio - anchor;
        self.scroll_to = Some(to.max(Vec2::ZERO));
    }

    /// Where the playlist was drawn last frame, not counting the Patterns
    /// and Library panel.
    pub fn area(&self) -> Rect {
        self.area
    }

    pub fn reveal_step(&mut self, step: usize) {
        self.reveal = Some(step);
    }

    /// Whether trying something from the library also sets its tempo.
    pub fn library_tempo(&self) -> bool {
        self.library.parts.tempo
    }

    pub fn library_parts(&self) -> lineup_core::library::apply::StyleParts {
        self.library.parts.for_preset()
    }

    pub fn library_parts_mut(&mut self) -> &mut lineup_core::library::apply::StyleParts {
        &mut self.library.parts
    }

    pub fn library_browser_mut(&mut self) -> &mut LibraryBrowser {
        &mut self.library
    }

    /// Draws the pattern picker and the playlist. `current` is the
    /// selected pattern; `playhead` is the step of the playlist being
    /// played, and which time round, if the song is playing; `tried` is
    /// what the library put in the selected pattern, if it was the library.
    /// Steps are drawn `zoom.x` times wider than usual, tracks `zoom.y`
    /// times taller.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        current: usize,
        playhead: Option<(usize, Pass)>,
        tried: Tried,
        zoom: Vec2,
    ) -> Vec<PlaylistAction> {
        let mut actions = Vec::new();
        self.reconcile_selection(project.playlist().clips());
        if zoom != self.zoom {
            self.zoom_to(ui.ctx(), zoom);
        }

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
                        .on_hover_text("Search all presets by name, genre or meter");
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
        self.area = ui.available_rect_before_wrap();

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
            ui.separator();
            if let Some(swing) = swing_control(ui, project) {
                actions.push(PlaylistAction::SetSwing(swing));
            }
            loose_timing(ui, project, &mut actions);
            if let Some(region) = project.playlist().loop_region() {
                let meter = project.meter();
                let first = meter.bar_at(region.start).number + 1;
                let last = meter.bar_at(region.end.saturating_sub(1)).number + 1;
                let text = egui::RichText::new(format!("🔁 Looping bars {first}–{last}  ✖"))
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

        let area = egui::ScrollArea::both()
            .id_salt("playlist")
            .animated(false)
            .auto_shrink(false);
        let area = match self.scroll_to.take() {
            Some(offset) => area.scroll_offset(offset),
            None => area,
        };
        widgets::no_rubber_band(ui);
        let output = area.show_viewport(ui, |ui, viewport| {
            self.grid(ui, viewport, project, current, playhead, &mut actions)
        });
        self.scrolled = output.state.clamped_offset();
        self.view_rect = Some(output.inner_rect);
        self.selected
            .retain(|index| *index < project.playlist().clips().len());
        if widgets::editor_keyboard_in(ui.ctx(), ui.layer_id(), self.area) {
            self.keys(ui.ctx(), project, &mut actions);
        }
        actions
    }

    /// Draws only the tracks in `ui`, for a video: no pattern picker and
    /// no toolbar. `playhead` is the step of the playlist that plays, and
    /// which time round; the tracks scroll along with it.
    pub fn video(&mut self, ui: &mut Ui, project: &Project, playhead: Option<(usize, Pass)>) {
        let mut actions = Vec::new();
        egui::ScrollArea::both()
            .id_salt("playlist")
            .animated(false)
            .auto_shrink(false)
            .show_viewport(ui, |ui, viewport| {
                self.grid(ui, viewport, project, usize::MAX, playhead, &mut actions)
            });
    }

    /// Preserve clip identities when deleting an earlier clip shifts its index.
    fn reconcile_selection(&mut self, clips: &[Clip]) {
        if self.shown_clips.len() != clips.len() {
            if clips.starts_with(&self.shown_clips) {
                self.shown_clips = clips.to_vec();
                self.selected.retain(|&index| index < clips.len());
                return;
            }
            let mut selected = std::collections::BTreeSet::new();
            for &index in &self.selected {
                if let Some(old) = self.shown_clips.get(index) {
                    if let Some((index, _)) = clips
                        .iter()
                        .enumerate()
                        .find(|(index, clip)| !selected.contains(index) && *clip == old)
                    {
                        selected.insert(index);
                    }
                } else if index < clips.len() {
                    // A clip just added or pasted is selected before its first draw.
                    selected.insert(index);
                }
            }
            self.selected = selected;
            self.drag = None;
        }
        self.shown_clips = clips.to_vec();
    }

    pub fn select_section(&mut self, project: &Project, region: Option<LoopRegion>) {
        self.strips.selected = region;
        self.selected = project
            .playlist()
            .clips()
            .iter()
            .enumerate()
            .filter(|(_, clip)| {
                region.is_some_and(|region| {
                    clip.start < region.end && clip.start + project.clip_length(clip) > region.start
                })
            })
            .map(|(index, _)| index)
            .collect();
        self.shown_clips = project.playlist().clips().to_vec();
        self.drag = None;
        self.selection_box = None;
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
        let meter = project.meter();
        // The grid ends on a bar line, some empty bars after the song.
        let extra = meter
            .bar(meter.bar_at(song_length).number + EXTRA_BARS)
            .start;
        let step_width = STEP_WIDTH * self.zoom.x;
        let mut track_tops = Vec::with_capacity(rows + 1);
        track_tops.push(0.0);
        for row in 0..rows {
            let height = tracks.get(row).map_or(TRACK_HEIGHT, |track| track.height()) * self.zoom.y;
            track_tops.push(track_tops.last().copied().unwrap_or(0.0) + height);
        }
        // The grid always reaches the right edge of the view: zoomed out,
        // and after a zoom in that keeps a step far to the right in place.
        let fill = ((viewport.right() - HEADER_WIDTH) / step_width).ceil() as usize;
        let steps = meter.round_up_to_bar(extra.max(MIN_BARS * STEPS_PER_BAR).max(fill));
        let size = vec2(
            HEADER_WIDTH + steps as f32 * step_width,
            TOP + track_tops.last().copied().unwrap_or(0.0),
        );
        let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

        // `viewport` is the visible part, measured from the top-left of the
        // content. Moving it by the content's position on screen gives the
        // visible part in screen coordinates, where everything is drawn.
        let visible = viewport.translate(rect.min.to_vec2());
        let layout = Layout {
            origin: pos2(rect.left() + HEADER_WIDTH, rect.top() + TOP),
            step_width,
            track_tops: &track_tops,
            tracks: tracks.len(),
            spare,
        };
        // The part of the grid not hidden under the track names, the
        // sections or the ruler.
        let cells = Rect::from_min_max(
            pos2(visible.left() + HEADER_WIDTH, visible.top() + TOP),
            visible.max,
        );

        self.handle_input(ui, &response, layout, cells, project, current, actions);

        if let Some(step) = self.reveal.take() {
            // Include the fixed track headers, so step zero is fully visible.
            ui.scroll_to_rect(
                Rect::from_min_max(
                    pos2(layout.x(step) - HEADER_WIDTH, cells.center().y),
                    pos2(layout.x(step) + step_width, cells.center().y + 1.0),
                ),
                None,
            );
        } else if let Some(step) = playhead {
            widgets::follow_x(ui, playhead, layout.x(step), cells, HEADER_WIDTH);
        }

        let painter = ui.painter_at(visible);
        paint_tracks(&painter, layout, steps, &meter);
        if let Some(region) = project.playlist().loop_region() {
            // A light shade over the part of the song that loops.
            let shade = Rect::from_x_y_ranges(
                layout.x(region.start)..=layout.x(region.end),
                cells.top()..=layout.track_top(rows).min(cells.bottom()),
            );
            painter.rect_filled(shade, 0.0, sections::LOOP.gamma_multiply(0.06));
        }
        paint_clips(&painter, layout, cells, project, current);
        for &index in &self.selected {
            if let Some(clip) = project.playlist().clips().get(index) {
                painter.rect_stroke(
                    layout
                        .clip_rect(clip, project.clip_length(clip))
                        .shrink(1.5),
                    2.0,
                    Stroke::new(2.0, theme::ACCENT),
                    StrokeKind::Inside,
                );
            }
        }
        if let (Some(start), Some(pos)) = (self.selection_box, ui.ctx().pointer_interact_pos()) {
            let selection = Rect::from_two_pos(start, pos).intersect(cells);
            painter.rect_filled(selection, 0.0, theme::ACCENT.gamma_multiply(0.15));
            painter.rect_stroke(
                selection,
                0.0,
                Stroke::new(1.0, theme::ACCENT),
                StrokeKind::Inside,
            );
        }
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
            &meter,
            &project.key_map(),
            project.variations(),
            song_length,
            playhead,
        );

        // The two strips at the top, right of the corner.
        let left = visible.left() + HEADER_WIDTH;
        let strip = Rect::from_min_max(
            pos2(left, visible.top()),
            pos2(visible.right(), visible.top() + SECTION_HEIGHT),
        );
        let tempo = Rect::from_min_max(
            pos2(left, strip.bottom()),
            pos2(visible.right(), strip.bottom() + TEMPO_HEIGHT),
        );
        let ruler = Rect::from_min_max(
            pos2(left, tempo.bottom()),
            pos2(visible.right(), tempo.bottom() + RULER_HEIGHT),
        );
        self.strips
            .sections(ui, layout, strip, project, self.snap, pass, actions);
        self.tempo_strip
            .show(ui, layout, tempo, project, self.snap, steps, actions);
        self.strips.loop_ruler(
            ui,
            layout,
            ruler,
            project,
            self.snap,
            &mut self.library.parts,
            actions,
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
            let row = Rect::from_min_size(
                pos2(column.left(), top),
                vec2(HEADER_WIDTH, layout.track_height(index)),
            );
            // Only rows fully below the ruler get their buttons, so a row
            // half-hidden under the ruler cannot be clicked through it.
            if row.top() < cells.top() || row.top() > visible.bottom() {
                continue;
            }
            painter.hline(row.x_range(), row.bottom(), Stroke::new(1.0, ROW[1]));

            // Real widgets, placed with `ui.put` at a rectangle we choose:
            // a light to mute the track, and a frameless text field to rename it.
            let light = Rect::from_center_size(
                pos2(row.left() + 11.0, row.center().y),
                vec2(14.0, 24.0f32.min(row.height() - 4.0)),
            );
            let enabled = !track.muted;
            let light_response = ui
                .put(light, |ui: &mut Ui| widgets::mute_light(ui, enabled))
                .on_hover_text("Click to mute or unmute the track");
            if light_response.clicked() {
                actions.push(PlaylistAction::ToggleTrackMute(index));
            }

            // One line of text, in the middle of the row however tall it is.
            let line = (ui.text_style_height(&egui::TextStyle::Body) + 2.0).min(row.height() - 2.0);
            let name_rect = Rect::from_min_max(
                pos2(light.right() + 4.0, row.center().y - line / 2.0),
                pos2(row.right() - 6.0, row.center().y + line / 2.0),
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
                ui.horizontal(|ui| {
                    ui.label("Track height");
                    let mut height = track.height() as u16;
                    if ui
                        .add(
                            egui::DragValue::new(&mut height)
                                .range(24..=240)
                                .suffix(" px"),
                        )
                        .changed()
                    {
                        actions.push(PlaylistAction::ResizeTrack {
                            track: index,
                            height,
                        });
                    }
                });
                if ui.button("Reset track height").clicked() {
                    actions.push(PlaylistAction::ResizeTrack {
                        track: index,
                        height: 36,
                    });
                }
                ui.separator();
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
            let edge =
                Rect::from_x_y_ranges(row.x_range(), row.bottom() - 4.0..=row.bottom() + 4.0);
            let id = ui.scope_id().with(("track-height", index));
            let resize = ui
                .interact(edge, id, Sense::click_and_drag())
                .on_hover_cursor(egui::CursorIcon::ResizeVertical)
                .on_hover_text("Drag to resize this track; double-click to reset its height");
            if resize.drag_started() {
                ui.ctx()
                    .data_mut(|data| data.insert_temp(id, track.height()));
            }
            if let Some(delta) = resize.total_drag_delta() {
                let base = ui
                    .ctx()
                    .data(|data| data.get_temp::<f32>(id))
                    .unwrap_or(track.height());
                let height = (base + delta.y / self.zoom.y).round().clamp(24.0, 240.0) as u16;
                actions.push(PlaylistAction::ResizeTrack {
                    track: index,
                    height,
                });
            }
            if resize.double_clicked() {
                actions.push(PlaylistAction::ResizeTrack {
                    track: index,
                    height: 36,
                });
            }
            if resize.hovered() || resize.dragged() {
                painter.hline(row.x_range(), row.bottom(), Stroke::new(2.0, theme::ACCENT));
            }
        }

        // The spare row's header: a button that adds the track, for those
        // who would rather not place a clip there to make one.
        if layout.spare {
            let top = layout.track_top(layout.tracks);
            let row = Rect::from_min_size(
                pos2(column.left(), top),
                vec2(HEADER_WIDTH, layout.track_height(layout.tracks)),
            );
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
        if widgets::editor_keyboard_in(ui.ctx(), ui.layer_id(), self.area)
            && ui.input(|input| input.key_pressed(egui::Key::Escape))
        {
            self.drag = None;
            self.selection_box = None;
            self.strips.cancel_drag();
            return;
        }
        let clips = project.playlist().clips();
        let (primary_pressed, primary_down, secondary, pointer, press_origin) = ui.input(|input| {
            let pointer = &input.pointer;
            (
                pointer.primary_pressed(),
                pointer.primary_down(),
                pointer.secondary_pressed() || pointer.secondary_down(),
                pointer.interact_pos(),
                widgets::press_pos(input, Some(egui::PointerButton::Primary)),
            )
        });

        // A press is handled where the button went down. A quick hand can
        // press and move in the same frame, and the clip or empty cell
        // under the pointer *now* may not be the one that was clicked.
        // A click that closes a menu does nothing else.
        let dismissing = widgets::dismissing(ui.ctx());
        let pressed_at = press_origin.filter(|pos| {
            primary_pressed && !dismissing && response.hovered() && cells.contains(*pos)
        });
        if let Some(pos) = pressed_at {
            let (track, step) = layout.cell_at(pos);
            let meter = project.meter();
            match clip_at(project, layout, pos) {
                Some(index) => {
                    self.strips.selected = None;
                    let add = ui.input(|input| input.modifiers.command || input.modifiers.shift);
                    if add {
                        if !self.selected.insert(index) {
                            self.selected.remove(&index);
                        }
                        self.drag = None;
                        return;
                    }
                    if !self.selected.contains(&index) {
                        self.selected.clear();
                        self.selected.insert(index);
                    }
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
                    self.strips.selected = None;
                    let start = self.snap.floor(step, &meter);
                    self.paste_at = start;
                    self.selected.clear();
                    // Ctrl-click empty space moves the paste point without placing a clip.
                    if ui.input(|input| input.modifiers.command || input.modifiers.shift) {
                        self.selection_box = Some(pos);
                        return;
                    }
                    self.selected.insert(clips.len());
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
            && !dismissing
            && response.hovered()
            && let Some(pos) = pointer.filter(|pos| cells.contains(*pos))
            && let Some(index) = clip_at(project, layout, pos)
        {
            actions.push(PlaylistAction::EditClip(clips[index].pattern));
        }

        // Deleting follows the pointer, so a right-drag sweeps clips away.
        let hovered = response.hover_pos().filter(|pos| cells.contains(*pos));
        if secondary
            && !dismissing
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

        if let (Some(start), Some(pos)) = (self.selection_box, pointer) {
            let selection = Rect::from_two_pos(start, pos).intersect(cells);
            if start.distance(pos) > 3.0 {
                self.selected = clips
                    .iter()
                    .enumerate()
                    .filter(|(_, clip)| {
                        selection.intersects(layout.clip_rect(clip, project.clip_length(clip)))
                    })
                    .map(|(index, _)| index)
                    .collect();
            }
        }
        if !primary_down {
            self.drag = None;
            self.selection_box = None;
        }
        if let Some(drag) = self.drag
            && drag.stretch
            && let Some(pos) = pointer
            && let Some(clip) = clips.get(drag.index)
        {
            // As many whole patterns as reach the pointer, the nearest.
            let pattern_length = project.pattern(clip.pattern).map_or(0, Pattern::length);
            let reach = (pos.x - layout.x(clip.start)) / layout.step_width;
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
                .floor(step.saturating_sub(drag.grab), &project.meter());
            if (track, start) != (clip.track, clip.start) {
                if self.selected.len() > 1 {
                    let selected: Vec<_> = self
                        .selected
                        .iter()
                        .filter_map(|&index| clips.get(index))
                        .collect();
                    let min_track = selected.iter().map(|clip| clip.track).min().unwrap_or(0);
                    let max_track = selected.iter().map(|clip| clip.track).max().unwrap_or(0);
                    let min_start = selected.iter().map(|clip| clip.start).min().unwrap_or(0);
                    let max_start = selected.iter().map(|clip| clip.start).max().unwrap_or(0);
                    let tracks = (track as isize - clip.track as isize)
                        .clamp(-(min_track as isize), (MAX_TRACKS - 1 - max_track) as isize);
                    let steps = (start as isize - clip.start as isize).clamp(
                        -(min_start as isize),
                        (MAX_SONG_STEPS - 1 - max_start) as isize,
                    );
                    actions.push(PlaylistAction::MoveClips {
                        indices: self.selected.iter().copied().collect(),
                        tracks,
                        steps,
                    });
                } else {
                    actions.push(PlaylistAction::MoveClip {
                        index: drag.index,
                        track,
                        start,
                    });
                }
            }
        }
    }

    fn keys(&mut self, ctx: &egui::Context, project: &Project, actions: &mut Vec<PlaylistAction>) {
        use egui::{Key, KeyboardShortcut, Modifiers};
        let pressed = |key| {
            ctx.input_mut(|input| {
                input.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, key))
            })
        };
        let clips = project.playlist().clips();
        if pressed(Key::A) {
            self.selected = (0..clips.len()).collect();
            self.strips.selected = None;
        }
        let copy = pressed(Key::C) || ctx.input(|input| input.events.contains(&egui::Event::Copy));
        let cut = pressed(Key::X) || ctx.input(|input| input.events.contains(&egui::Event::Cut));
        // With nothing selected, these leave the clipboard as it was.
        let duplicate = pressed(Key::D) && !self.selected.is_empty();
        if (copy || cut || duplicate) && !self.selected.is_empty() {
            self.clipboard = self
                .selected
                .iter()
                .filter_map(|&index| clips.get(index).copied())
                .collect();
            if copy || cut {
                crate::clipboard::copy(ctx, "clips", &self.clipboard);
            }
        }
        if cut
            || ctx.input_mut(|input| {
                input.consume_key(Modifiers::NONE, Key::Delete)
                    || input.consume_key(Modifiers::NONE, Key::Backspace)
            })
        {
            actions.push(PlaylistAction::RemoveClips(
                self.selected.iter().copied().collect(),
            ));
            self.selected.clear();
        }
        let system_paste = crate::clipboard::paste::<Vec<Clip>>(ctx, "clips");
        let paste = pressed(Key::V) && !crate::clipboard::has_paste(ctx);
        let pasted = system_paste.is_some();
        if let Some(clips) = system_paste {
            self.clipboard = clips
                .into_iter()
                .filter(|clip| clip.pattern < project.patterns().len())
                .map(|mut clip| {
                    clip.repeats = clip.repeats.clamp(1, MAX_REPEATS);
                    clip.start = clip.start.min(MAX_SONG_STEPS - 1);
                    clip.track = clip
                        .track
                        .min(project.playlist().tracks().len().saturating_sub(1));
                    clip
                })
                .collect();
        }
        if (paste || pasted || duplicate) && !self.clipboard.is_empty() {
            let first = self
                .clipboard
                .iter()
                .map(|clip| clip.start)
                .min()
                .unwrap_or(0);
            let at = if duplicate {
                self.clipboard
                    .iter()
                    .map(|clip| clip.start + project.clip_length(clip))
                    .max()
                    .unwrap_or(first)
            } else {
                self.paste_at
            };
            let pasted: Vec<_> = self
                .clipboard
                .iter()
                .copied()
                .map(|mut clip| {
                    clip.start = at
                        .saturating_add(clip.start - first)
                        .min(MAX_SONG_STEPS - 1);
                    clip
                })
                .collect();
            self.selected = (clips.len()..clips.len() + pasted.len()).collect();
            actions.push(PlaylistAction::PasteClips(pasted));
        }
        if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
            self.selected.clear();
            self.selection_box = None;
            self.strips.selected = None;
            self.drag = None;
        }
    }
}

/// How much the song swings, from "straight" to [`MAX_SWING`] percent:
/// a triplet. Returns the new amount when it is changed. The chord sheet
/// shows it too.
pub(crate) fn swing_control(ui: &mut Ui, project: &Project) -> Option<u8> {
    let meter = project.meter();
    let swings =
        swings_in(meter.first()) || meter.markers().iter().any(|marker| swings_in(marker.time));
    let mut swing = project.swing;
    let label = ui.label("Swing");
    let value = ui.add_enabled(
        swings,
        egui::DragValue::new(&mut swing)
            .range(0..=MAX_SWING)
            .speed(0.5)
            .custom_formatter(|amount, _| {
                if amount == 0.0 {
                    "straight".to_string()
                } else {
                    format!("{amount:.0}%")
                }
            })
            .custom_parser(|text| {
                let text = text.trim().trim_end_matches('%').trim();
                if text.eq_ignore_ascii_case("straight") {
                    Some(0.0)
                } else {
                    text.parse().ok()
                }
            }),
    );
    let text = "Swing: the second eighth note of every beat comes late, so pairs of eighths \
                are played long-short, as in jazz, blues and shuffles. \"straight\" plays them \
                even, 50% is a light swing and 100% a triplet: two thirds and one third. Each \
                style remembers what you set. Drag up or down, or double-click to type";
    theme::hint(label, text);
    let value = theme::hint(value, text).on_disabled_hover_text(format!(
        "The song is in {}: its beats are eighth notes, or already split in three, so \
         there are no pairs of eighths to swing",
        meter.first()
    ));
    value.changed().then_some(swing)
}

/// How loose the timing is: how far off the grid notes may land as they
/// play, from "off" to [`MAX_LOOSE_TIMING`] milliseconds.
fn loose_timing(ui: &mut Ui, project: &Project, actions: &mut Vec<PlaylistAction>) {
    let mut loose = project.loose_timing;
    let label = ui.label("Loose");
    let value = ui.add(
        egui::DragValue::new(&mut loose)
            .range(0..=MAX_LOOSE_TIMING)
            .speed(0.2)
            .custom_formatter(|ms, _| {
                if ms == 0.0 {
                    "off".to_string()
                } else {
                    format!("{ms:.0} ms")
                }
            })
            .custom_parser(|text| {
                let text = text.trim().trim_end_matches("ms").trim();
                if text.eq_ignore_ascii_case("off") {
                    Some(0.0)
                } else {
                    text.parse().ok()
                }
            }),
    );
    let text = "Loose timing: as the song plays, notes land up to this many milliseconds \
                after their step, a different amount each time, as when a person plays. \
                \"off\" plays every note exactly on the grid. Drag up or down, or \
                double-click to type";
    label.on_hover_text(text);
    if value.on_hover_text(text).changed() {
        actions.push(PlaylistAction::SetLooseTiming(loose));
    }
}

/// The list of patterns on the left. Clicking one selects it: the channel
/// rack shows it, pattern mode plays it and new clips use it.
fn picker(ui: &mut Ui, project: &Project, current: usize, actions: &mut Vec<PlaylistAction>) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (index, pattern) in project.patterns().iter().enumerate() {
            ui.horizontal(|ui| {
                let (swatch, _) = ui.allocate_exact_size(vec2(10.0, 16.0), Sense::hover());
                ui.painter().rect_filled(
                    swatch.shrink2(vec2(0.0, 2.0)),
                    2.0,
                    pattern_color(project, index),
                );
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
    let meter = project.meter();
    let start = playlist.sections().last().map_or(0, Section::end);
    let four_bars = meter.bar(meter.bar_at(start).number + 4).start;
    let song_end = project.song_length();
    let length = if song_end > start {
        song_end.min(four_bars) - start
    } else {
        four_bars - start
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
fn paint_tracks(painter: &Painter, layout: Layout, steps: usize, meter: &Meter) {
    let right = layout.x(steps);
    for track in 0..layout.rows() {
        let top = layout.track_top(track);
        let row = Rect::from_min_max(
            pos2(layout.origin.x, top),
            pos2(right, layout.track_top(track + 1)),
        );
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
    for bar in meter.bars().take_while(|bar| bar.start <= steps) {
        // Zoomed far out, beat lines would melt into a grey blur: then
        // only the bar lines are drawn. The shortest beat decides, as the
        // beats of a grouped bar such as 2+2+3/8 are not all as long.
        let shortest = bar.time.groups().into_iter().min().unwrap_or(1);
        let shortest =
            usize::from(shortest) * bar.time.steps_per_bar() / usize::from(bar.time.beats.max(1));
        let lines: Vec<usize> = if shortest as f32 * layout.step_width < 4.0 {
            vec![bar.start]
        } else {
            bar.beat_starts().collect()
        };
        for step in lines.into_iter().filter(|&step| step <= steps) {
            let color = if step == bar.start {
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
        let mut color = pattern_color(project, clip.pattern);
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
        let round = pattern.length() as f32 * layout.step_width;
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
                paint_clip_notes(&notes_painter, body, pattern, layout.step_width);
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
fn paint_clip_notes(painter: &Painter, body: Rect, pattern: &Pattern, step_width: f32) {
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
        let x_start = body.left() + note.start as f32 * step_width;
        let held = note.length.min(length - note.start) as f32;
        let x_end = (x_start + held * step_width - 1.0).max(x_start + 1.5);
        painter.line_segment(
            [pos2(x_start, y), pos2(x_end, y)],
            Stroke::new(1.5, CLIP_NOTES),
        );
    }
}

/// The strip at the top with bar numbers, which stays put when the
/// playlist scrolls down.
#[allow(clippy::too_many_arguments)]
fn paint_ruler(
    painter: &Painter,
    layout: Layout,
    visible: Rect,
    steps: usize,
    meter: &Meter,
    keys: &KeyMap,
    variations: &[VariationMarker],
    song_length: usize,
    playhead: Option<usize>,
) {
    let ruler = Rect::from_min_size(
        visible.min + vec2(0.0, SECTION_HEIGHT + TEMPO_HEIGHT),
        vec2(visible.width(), RULER_HEIGHT),
    );
    painter.rect_filled(ruler, 0.0, RULER);
    let ruler_painter = painter.with_clip_rect(Rect::from_min_max(
        pos2(visible.left() + HEADER_WIDTH, ruler.top()),
        ruler.max,
    ));
    // The time signature is written where the song starts and where it
    // changes, after the bar number.
    let changes: Vec<usize> = meter.markers().iter().map(|marker| marker.start).collect();
    // Zoomed out, the bars get too narrow for their numbers: then every
    // second, fourth, ... bar is numbered.
    let bar_width = layout.step_width * STEPS_PER_BAR as f32;
    let every = [1, 2, 4, 8, 16, 32]
        .into_iter()
        .find(|&every| bar_width * every as f32 >= BAR_NUMBER_WIDTH)
        .unwrap_or(64);
    // Where the last text written ends: a long key name runs over the
    // next bars, whose numbers are then left out.
    let mut covered = f32::NEG_INFINITY;
    for bar in meter.bars().take_while(|bar| bar.end() <= steps) {
        let x = layout.x(bar.start);
        let color = if bar.start < song_length {
            HEADER_TEXT
        } else {
            RULER_TEXT
        };
        let change = bar.start == 0 || changes.contains(&bar.start);
        // So is the key, where the song starts and where it modulates.
        let key_change = bar.start == 0 || keys.markers().iter().any(|m| m.start == bar.start);
        // And the band's variation, where it changes.
        let variation = variations
            .iter()
            .find(|marker| marker.start == bar.start)
            .map(|marker| marker.variation);
        let marked = change || key_change || variation.is_some();
        if bar.number % every != 0 && !marked {
            continue;
        }
        ruler_painter.vline(x, ruler.y_range(), Stroke::new(1.0, BAR_LINE));
        if x < covered && !marked {
            continue;
        }
        let number = ruler_painter.text(
            pos2(x + 3.0, ruler.center().y),
            Align2::LEFT_CENTER,
            (bar.number + 1).to_string(),
            FontId::proportional(11.0),
            color,
        );
        let mut right = number.right();
        if change {
            right = ruler_painter
                .text(
                    pos2(right + 4.0, ruler.center().y),
                    Align2::LEFT_CENTER,
                    bar.time.to_string(),
                    FontId::proportional(10.0),
                    TIME_TEXT,
                )
                .right();
        }
        if key_change {
            right = ruler_painter
                .text(
                    pos2(right + 4.0, ruler.center().y),
                    Align2::LEFT_CENTER,
                    keys.key_at(bar.start).name(),
                    FontId::proportional(10.0),
                    KEY_TEXT,
                )
                .right();
        }
        if let Some(variation) = variation {
            let letter = ruler_painter.text(
                pos2(right + 7.0, ruler.center().y),
                Align2::LEFT_CENTER,
                variation.letter(),
                FontId::proportional(10.0),
                TIME_TEXT,
            );
            ruler_painter.rect_stroke(
                letter.expand2(vec2(3.0, 0.0)),
                2.0,
                Stroke::new(1.0, TIME_TEXT),
                StrokeKind::Middle,
            );
            right = letter.right() + 3.0;
        }
        covered = right + 4.0;
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
    let corner = Rect::from_min_size(visible.min, vec2(HEADER_WIDTH, TOP));
    painter.rect_filled(corner, 0.0, RULER);
    painter.text(
        pos2(corner.left() + 6.0, corner.top() + SECTION_HEIGHT / 2.0),
        Align2::LEFT_CENTER,
        "Sections",
        FontId::proportional(11.0),
        RULER_TEXT,
    );
    painter.text(
        pos2(
            corner.left() + 6.0,
            corner.top() + SECTION_HEIGHT + TEMPO_HEIGHT / 2.0,
        ),
        Align2::LEFT_CENTER,
        "Tempo",
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

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Event, PointerButton, RawInput};

    fn selection_project() -> Project {
        let mut project = Project::empty();
        project.pattern_mut(0).unwrap().set_length(16);
        project.add_clip(Clip::new(0, 0, 0));
        project.add_clip(Clip::new(0, 1, 16));
        project.add_clip(Clip::new(0, 0, 64));
        project.add_section(Section::new("Verse", 0, 32));
        project
    }

    fn pointer(pos: Pos2, pressed: bool, modifiers: egui::Modifiers) -> Vec<Event> {
        vec![
            Event::ModifiersChanged(modifiers),
            Event::PointerMoved(pos),
            Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed,
                modifiers,
            },
        ]
    }

    #[test]
    fn section_selection_toggles_and_tracks_the_clips_after_deletion() {
        let ctx = egui::Context::default();
        let mut view = PlaylistView::new();
        view.browser_open = false;
        let mut project = selection_project();
        frame(&ctx, &mut view, &project, vec![]);
        frame(&ctx, &mut view, &project, vec![]);
        let visible = view.view_rect.unwrap();
        let section = pos2(
            visible.left() + HEADER_WIDTH + 8.0 * STEP_WIDTH,
            visible.top() + SECTION_HEIGHT / 2.0,
        );
        for expected in [Some(LoopRegion::new(0, 32)), None] {
            frame(
                &ctx,
                &mut view,
                &project,
                pointer(section, true, egui::Modifiers::COMMAND),
            );
            let actions = frame(
                &ctx,
                &mut view,
                &project,
                pointer(section, false, egui::Modifiers::COMMAND),
            );
            assert!(
                actions.contains(&PlaylistAction::SelectSection(expected)),
                "{actions:?}"
            );
            view.select_section(&project, expected);
            assert_eq!(
                view.selected.iter().copied().collect::<Vec<_>>(),
                if expected.is_some() {
                    vec![0, 1]
                } else {
                    vec![]
                }
            );
        }
        view.selected.insert(2);
        project.remove_clip(0);
        frame(
            &ctx,
            &mut view,
            &project,
            vec![Event::ModifiersChanged(egui::Modifiers::NONE)],
        );
        assert_eq!(view.selected.iter().copied().collect::<Vec<_>>(), vec![1]);
        assert_eq!(project.playlist().clips()[1].start, 64);
    }

    #[test]
    fn ctrl_drag_selects_a_clip_range_and_dragging_moves_it_together() {
        let ctx = egui::Context::default();
        let mut view = PlaylistView::new();
        view.browser_open = false;
        let project = selection_project();
        frame(&ctx, &mut view, &project, vec![]);
        frame(&ctx, &mut view, &project, vec![]);
        let origin = view.view_rect.unwrap().min + vec2(HEADER_WIDTH, TOP);
        let from = origin + vec2(33.0 * STEP_WIDTH, 2.0 * TRACK_HEIGHT + 5.0);
        let to = origin + vec2(1.0, 1.0);
        let actions = frame(
            &ctx,
            &mut view,
            &project,
            pointer(from, true, egui::Modifiers::COMMAND),
        );
        assert!(actions.is_empty());
        frame(&ctx, &mut view, &project, vec![Event::PointerMoved(to)]);
        frame(
            &ctx,
            &mut view,
            &project,
            pointer(to, false, egui::Modifiers::COMMAND),
        );
        assert_eq!(
            view.selected.iter().copied().collect::<Vec<_>>(),
            vec![0, 1]
        );
        let from = origin + vec2(4.0 * STEP_WIDTH, TRACK_HEIGHT * 0.5);
        let to = from + vec2(16.0 * STEP_WIDTH, TRACK_HEIGHT);
        frame(
            &ctx,
            &mut view,
            &project,
            pointer(from, true, egui::Modifiers::NONE),
        );
        let actions = frame(&ctx, &mut view, &project, vec![Event::PointerMoved(to)]);
        assert!(
            actions.contains(&PlaylistAction::MoveClips {
                indices: vec![0, 1],
                tracks: 1,
                steps: 16
            }),
            "{actions:?}"
        );
    }

    #[test]
    fn select_all_and_native_paste_work_in_the_playlist_window() {
        let ctx = egui::Context::default();
        let mut view = PlaylistView::new();
        view.browser_open = false;
        let project = Project::demo();
        frame(&ctx, &mut view, &project, vec![]);
        frame(&ctx, &mut view, &project, vec![]);
        frame(
            &ctx,
            &mut view,
            &project,
            vec![Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            }],
        );
        assert_eq!(view.selected.len(), project.playlist().clips().len());
        let copied = vec![project.playlist().clips()[0]];
        view.paste_at = 64;
        let actions = frame(
            &ctx,
            &mut view,
            &project,
            vec![Event::Paste(format!(
                "Ferrous Lineup clips\n{}",
                ron::to_string(&copied).unwrap()
            ))],
        );
        assert!(actions.iter().any(|action| matches!(action,PlaylistAction::PasteClips(clips) if clips.len() == 1 && clips[0].start == 64)));
    }

    #[test]
    fn copying_with_nothing_selected_keeps_the_clipboard() {
        let ctx = egui::Context::default();
        let mut view = PlaylistView::new();
        view.browser_open = false;
        let project = Project::demo();
        frame(&ctx, &mut view, &project, vec![]);
        frame(&ctx, &mut view, &project, vec![]);
        let copied = vec![project.playlist().clips()[0]];
        view.clipboard = copied.clone();
        frame(&ctx, &mut view, &project, vec![Event::Copy]);
        assert_eq!(view.clipboard, copied);
        let duplicate = Event::Key {
            key: egui::Key::D,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        };
        let actions = frame(&ctx, &mut view, &project, vec![duplicate]);
        assert!(
            !actions
                .iter()
                .any(|action| matches!(action, PlaylistAction::PasteClips(_))),
            "nothing selected, nothing duplicated"
        );
    }

    fn frame(
        ctx: &egui::Context,
        view: &mut PlaylistView,
        project: &Project,
        events: Vec<Event>,
    ) -> Vec<PlaylistAction> {
        let mut actions = Vec::new();
        let mut output = ctx.run_pass(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1200.0, 800.0))),
                time: Some(ctx.cumulative_frame_nr() as f64 * 0.1),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::Window::new("Playlist test")
                    .fixed_pos(Pos2::ZERO)
                    .default_size([1000.0, 600.0])
                    .show(ctx, |ui| {
                        actions =
                            view.show(ui, project, 0, None, Tried::default(), Vec2::splat(1.0));
                    });
            },
        );
        output.textures_delta.clear();
        actions
    }
    #[test]
    fn dragging_a_track_edge_resizes_only_that_track_and_preserves_clip_hit_testing() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut view = PlaylistView::new();
        view.browser_open = false;
        let mut project = Project::demo();
        frame(&ctx, &mut view, &project, vec![]);
        frame(&ctx, &mut view, &project, vec![]);
        let rect = view.view_rect.unwrap();
        let edge = pos2(rect.left() + HEADER_WIDTH * 0.5, rect.top() + TOP + 36.0);
        frame(
            &ctx,
            &mut view,
            &project,
            vec![
                Event::PointerMoved(edge),
                Event::PointerButton {
                    pos: edge,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let target = edge + vec2(0.0, 48.0);
        let actions = frame(&ctx, &mut view, &project, vec![Event::PointerMoved(target)]);
        assert!(
            actions.contains(&PlaylistAction::ResizeTrack {
                track: 0,
                height: 84
            }),
            "{actions:?}"
        );
        project.track_mut(0).unwrap().height = Some(84);
        let text = lineup_core::file::to_text(&project).unwrap();
        assert_eq!(lineup_core::file::from_text(&text).unwrap(), project);
        let tops = [0.0, 84.0, 120.0, 156.0];
        let layout = Layout {
            origin: Pos2::ZERO,
            track_tops: &tops,
            step_width: 4.5,
            tracks: 2,
            spare: true,
        };
        assert_eq!(layout.cell_at(pos2(4.5, 83.0)), (0, 1));
        assert_eq!(layout.cell_at(pos2(4.5, 85.0)), (1, 1));
        assert_eq!(layout.clip_rect(&Clip::new(0, 1, 0), 16).height(), 36.0);
    }
}
