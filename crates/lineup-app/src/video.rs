//! The pictures of a video: the views you chose, drawn as the song plays.
//!
//! The views are drawn by the same code as in the window, but not in the
//! window. Each view has a pane: an egui context of its own, which it
//! tells the size of its box in the picture and the time, as eframe tells
//! the window's, and a picture in memory, [`lineup_video::Canvas`], that
//! what egui draws is painted on. A view zoomed in is drawn with more
//! pixels to a point, as egui's own zoom does, so its letters stay sharp.
//! The panes are then copied into the picture, in their boxes, under a
//! title line. The window, the music you hear and the video being made
//! never get in each other's way.
//!
//! Where each box goes and how big its view is drawn is a [`Layout`],
//! which the Export video dialog lets you arrange on a preview.

use eframe::egui::{self, Color32, Pos2, Rect, RichText, Ui, vec2};
use lineup_core::library::apply::StyleParts;
use lineup_core::{Arrangement, Pass, Project};
use lineup_video::Canvas;
use serde::{Deserialize, Serialize};

use crate::chord_sheet::ChordSheet;
use crate::harmony_keys::HarmonyKeys;
use crate::lead_sheet::LeadSheetWindow;
use crate::piano_roll::PianoRoll;
use crate::playlist::PlaylistView;
use crate::theme;

/// The height of the title line, in points. The views share the rest.
pub const TITLE_HEIGHT: f32 = 40.0;

/// The views a video can show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum View {
    ChordSheet,
    LeadSheet,
    Playlist,
    PianoRoll,
    HarmonyKeys,
}

impl View {
    /// In the order they are stacked, and drawn: a later one is drawn
    /// over an earlier one where their boxes overlap.
    pub const ALL: [View; 5] = [
        View::ChordSheet,
        View::LeadSheet,
        View::Playlist,
        View::PianoRoll,
        View::HarmonyKeys,
    ];

    fn index(self) -> usize {
        self as usize
    }

    pub fn name(self) -> &'static str {
        match self {
            View::ChordSheet => "Chord sheet",
            View::LeadSheet => "Lead sheet",
            View::Playlist => "Playlist",
            View::PianoRoll => "Piano roll",
            View::HarmonyKeys => "Harmony keys",
        }
    }

    /// What it shows, for the dialog's tooltips.
    pub fn hint(self) -> &'static str {
        match self {
            View::ChordSheet => "The harmony bar by bar, the bar playing framed",
            View::LeadSheet => "The tune on a staff with its chords, the bar playing lit",
            View::Playlist => "The tracks and their clips, with the playhead",
            View::PianoRoll => "The notes of the channel the piano roll has selected, or the tune",
            View::HarmonyKeys => "The chords of the key, the one playing lit, and the little piano",
        }
    }

    /// Whether its zoom can be changed. The lead sheet always fills the
    /// width of its box: a wider box draws it bigger.
    pub fn zooms(self) -> bool {
        self != View::LeadSheet
    }
}

/// Where a view goes in the picture, and how big it is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Placement {
    pub shown: bool,
    /// Its box: left, top, right and bottom, from 0 to 1 across the room
    /// under the title line.
    pub rect: [f32; 4],
    /// How big it is drawn: 1 as in the window, 2 twice as big.
    pub zoom: f32,
}

impl Default for Placement {
    fn default() -> Self {
        Placement {
            shown: false,
            rect: [0.0, 0.0, 1.0, 1.0],
            zoom: 1.0,
        }
    }
}

impl Placement {
    pub const MIN_ZOOM: f32 = 0.5;
    pub const MAX_ZOOM: f32 = 2.5;
    /// The smallest box, across or down.
    pub const MIN_SIZE: f32 = 0.08;

    /// Moves the box by `by` (in parts of the room), keeping it inside.
    pub fn move_by(&mut self, by: egui::Vec2) {
        let [left, top, right, bottom] = self.rect;
        let dx = by.x.clamp(-left, 1.0 - right);
        let dy = by.y.clamp(-top, 1.0 - bottom);
        self.rect = [left + dx, top + dy, right + dx, bottom + dy];
    }

    /// Moves the box's bottom right corner by `by`, keeping it inside
    /// and not too small.
    pub fn resize_by(&mut self, by: egui::Vec2) {
        let [left, top, right, bottom] = self.rect;
        self.rect[2] = (right + by.x).clamp(left + Self::MIN_SIZE, 1.0);
        self.rect[3] = (bottom + by.y).clamp(top + Self::MIN_SIZE, 1.0);
    }

    /// Zooms by `factor`, within the limits.
    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
    }
}

/// Which views a video shows, where, and how big.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Layout {
    views: [Placement; 5],
}

impl Default for Layout {
    /// The chords, bar by bar and as keys lighting up.
    fn default() -> Self {
        let mut layout = Layout {
            views: [Placement::default(); 5],
        };
        layout[View::ChordSheet].shown = true;
        layout[View::HarmonyKeys].shown = true;
        layout.stack();
        layout
    }
}

impl std::ops::Index<View> for Layout {
    type Output = Placement;
    fn index(&self, view: View) -> &Placement {
        &self.views[view.index()]
    }
}

impl std::ops::IndexMut<View> for Layout {
    fn index_mut(&mut self, view: View) -> &mut Placement {
        &mut self.views[view.index()]
    }
}

impl Layout {
    /// Every view, shown in a stack.
    #[cfg(test)]
    pub fn every_view() -> Self {
        let mut layout = Layout::default();
        for view in View::ALL {
            layout[view].shown = true;
        }
        layout.stack();
        layout
    }

    /// Whether any view is chosen at all.
    pub fn any(&self) -> bool {
        self.views.iter().any(|placement| placement.shown)
    }

    /// The views shown, in the order they are drawn.
    pub fn shown(&self) -> impl Iterator<Item = View> + '_ {
        View::ALL.into_iter().filter(|&view| self[view].shown)
    }

    /// Puts the views shown one above the other, across the whole width:
    /// the harmony keys at the bottom, as tall as they need, and the
    /// others sharing the room above evenly.
    pub fn stack(&mut self) {
        // In parts of the room, 1280 by 680 points.
        let margin = 8.0 / 1280.0;
        let gap = 8.0 / 680.0;
        let (left, right) = (margin, 1.0 - margin);
        let mut bottom = 1.0;
        if self[View::HarmonyKeys].shown {
            let keys = 1.0 - 140.0 / 680.0;
            self[View::HarmonyKeys].rect = [0.0, keys, 1.0, 1.0];
            bottom = keys - gap;
        } else {
            bottom -= gap;
        }
        let stacked: Vec<View> = self
            .shown()
            .filter(|&view| view != View::HarmonyKeys)
            .collect();
        let count = stacked.len() as f32;
        let height = (bottom - gap - gap * (count - 1.0)) / count;
        for (index, view) in stacked.into_iter().enumerate() {
            let top = gap + index as f32 * (height + gap);
            self[view].rect = [left, top, right, top + height];
        }
    }
}

/// How big the pictures are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Size {
    /// 1280 × 720, "HD".
    #[default]
    Hd,
    /// 1920 × 1080, "Full HD".
    FullHd,
}

impl Size {
    pub const ALL: [Size; 2] = [Size::Hd, Size::FullHd];

    pub fn name(self) -> &'static str {
        match self {
            Size::Hd => "1280 × 720",
            Size::FullHd => "1920 × 1080",
        }
    }

    /// The width and height in pixels.
    pub fn pixels(self) -> (usize, usize) {
        match self {
            Size::Hd => (1280, 720),
            Size::FullHd => (1920, 1080),
        }
    }

    /// Pixels per point. Both sizes lay the views out the same, 1280 by
    /// 720 points; Full HD only draws them sharper.
    fn pixels_per_point(self) -> f32 {
        match self {
            Size::Hd => 1.0,
            Size::FullHd => 1.5,
        }
    }
}

/// The views of one video, and the picture they are painted on.
pub struct Scene {
    /// The whole picture, the panes copied into it.
    canvas: Canvas,
    size: Size,
    frame_rate: u32,
    /// The whole song, every repeat played, as the sound is made.
    song: Arrangement,
    layout: Layout,
    title: Pane,
    /// A pane for each view shown, in the order they are drawn.
    panes: Vec<(View, Pane)>,
    views: Views,
    /// Whether the panes must be copied in again: a box moved.
    moved: bool,
}

/// What the views draw from, and what each remembers between pictures.
struct Views {
    project: Project,
    title: String,
    chord_sheet: ChordSheet,
    /// Made with its pane's context, which it installs its fonts in.
    lead_sheet: Option<LeadSheetWindow>,
    playlist: PlaylistView,
    piano_roll: PianoRoll,
    /// The channel the piano roll shows, and the pattern it shows it in:
    /// the one playing, or the last one that played.
    piano_channel: usize,
    piano_pattern: usize,
    harmony_keys: HarmonyKeys,
    parts: StyleParts,
}

/// One box of the picture, drawn by an egui context of its own.
struct Pane {
    ctx: egui::Context,
    canvas: Canvas,
    /// Where it goes in the picture, in pixels.
    left: usize,
    top: usize,
    /// Pixels to a point: the picture's, times the view's zoom.
    pixels_per_point: f32,
    /// Whether a picture was drawn at this size yet. The first one is
    /// drawn twice: egui sizes some things on its first frame.
    started: bool,
}

impl Pane {
    fn new() -> Self {
        let ctx = egui::Context::default();
        theme::apply(&ctx);
        // A view scrolls to the bar playing at once: in the window the
        // scroll glides there, but in a video the bar would show half a
        // second late.
        ctx.all_styles_mut(|style| style.scroll_animation = egui::style::ScrollAnimation::none());
        // Letters egui's fonts lack (🔑, →) come from the system's fonts,
        // as in the window.
        ctx.add_font_provider(std::sync::Arc::new(
            egui_system_font_provider::SystemFontProvider::new(),
        ));
        Pane {
            ctx,
            canvas: Canvas::new(2, 2),
            left: 0,
            top: 0,
            pixels_per_point: 1.0,
            started: false,
        }
    }

    /// Puts the pane at `left`, `top`, `width` by `height` pixels, drawn
    /// `pixels_per_point`.
    fn place(
        &mut self,
        (left, top, width, height): (usize, usize, usize, usize),
        pixels_per_point: f32,
    ) {
        self.left = left;
        self.top = top;
        let (width, height) = (width.max(2), height.max(2));
        if (width, height) != (self.canvas.width(), self.canvas.height()) {
            self.canvas = Canvas::new(width, height);
            self.started = false;
        }
        if pixels_per_point != self.pixels_per_point {
            self.pixels_per_point = pixels_per_point;
            self.started = false;
        }
    }

    /// Draws `contents`, and paints them if anything changed.
    fn paint(
        &mut self,
        time: f64,
        frame_rate: u32,
        background: Color32,
        mut contents: impl FnMut(&mut Ui),
    ) -> bool {
        // The first picture is drawn twice: egui sizes some things on its
        // first frame. After that, a view that scrolls (to the bar playing)
        // only moves on the next frame, and asks for one at once; it gets
        // it in the same picture, so it is not a picture late.
        let mut passes = if self.started { 1 } else { 2 };
        self.started = true;
        let mut changed = false;
        let mut pass = 0;
        while pass < passes {
            pass += 1;
            let ppp = self.pixels_per_point;
            let size = vec2(self.canvas.width() as f32, self.canvas.height() as f32) / ppp;
            let mut input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                time: Some(time),
                predicted_dt: 1.0 / frame_rate as f32,
                ..Default::default()
            };
            input
                .viewports
                .entry(egui::ViewportId::ROOT)
                .or_default()
                .native_pixels_per_point = Some(ppp);
            let ctx = self.ctx.clone();
            let mut output = ctx.run_ui(input, &mut contents);
            let shapes = std::mem::take(&mut output.shapes);
            let primitives = ctx.tessellate(shapes, output.pixels_per_point);
            changed |= self.canvas.paint(
                primitives,
                &mut output.textures_delta,
                output.pixels_per_point,
                background,
            );
            let again = output
                .viewport_output
                .get(&egui::ViewportId::ROOT)
                .is_some_and(|viewport| viewport.repaint_delay.is_zero());
            if again && pass == passes && passes < 3 {
                passes += 1;
            }
        }
        changed
    }
}

impl Scene {
    /// The views of `project`, called `title`, placed as `layout` says, in
    /// pictures of `size`, `frame_rate` of them a second. The piano roll
    /// shows `piano_channel`.
    pub fn new(
        project: Project,
        title: String,
        layout: Layout,
        size: Size,
        frame_rate: u32,
        piano_channel: usize,
    ) -> Self {
        let (width, height) = size.pixels();
        let piano_pattern = (0..project.patterns().len())
            .find(|&pattern| has_notes(&project, pattern, piano_channel))
            .unwrap_or(0);
        let mut scene = Scene {
            canvas: Canvas::new(width, height),
            size,
            frame_rate,
            song: project.arrangement(),
            layout: Layout {
                views: [Placement::default(); 5],
            },
            title: Pane::new(),
            panes: Vec::new(),
            views: Views {
                project,
                title,
                chord_sheet: ChordSheet::new(),
                lead_sheet: None,
                playlist: PlaylistView::new(),
                piano_roll: PianoRoll::new(piano_channel),
                piano_channel,
                piano_pattern,
                harmony_keys: HarmonyKeys::new(),
                parts: StyleParts::default(),
            },
            moved: true,
        };
        scene.set_layout(layout);
        scene
    }

    /// Places the views as `layout` says, from the next picture on.
    pub fn set_layout(&mut self, layout: Layout) {
        let ppp = self.size.pixels_per_point();
        let (width, height) = self.size.pixels();
        let title_height = (TITLE_HEIGHT * ppp).round() as usize;
        self.title.place((0, 0, width, title_height), ppp);
        // Views no longer shown are let go of; new ones get a pane.
        self.panes.retain(|(view, _)| layout[*view].shown);
        for view in layout.shown() {
            if !self.panes.iter().any(|(shown, _)| *shown == view) {
                let pane = Pane::new();
                if view == View::LeadSheet && self.views.lead_sheet.is_none() {
                    self.views.lead_sheet = Some(LeadSheetWindow::new(&pane.ctx));
                }
                self.panes.push((view, pane));
            }
        }
        self.panes.sort_by_key(|(view, _)| view.index());
        if !layout[View::LeadSheet].shown {
            self.views.lead_sheet = None;
        }
        let room = (width as f32, (height - title_height) as f32);
        for (view, pane) in &mut self.panes {
            let placement = layout[*view];
            let [left, top, right, bottom] = placement.rect;
            let x = |part: f32| (part * room.0).round() as usize;
            let y = |part: f32| title_height + (part * room.1).round() as usize;
            let zoom = if view.zooms() { placement.zoom } else { 1.0 };
            pane.place(
                (x(left), y(top), x(right) - x(left), y(bottom) - y(top)),
                ppp * zoom,
            );
        }
        if layout != self.layout {
            self.layout = layout;
            self.moved = true;
        }
    }

    /// The picture at `time` seconds into the video, while the song is
    /// at `position`: steps of the song as it plays (a chorus played
    /// twice counts twice), 4.5 being halfway through step 4. `None`
    /// after the end. Four bytes a pixel (red, green, blue, opacity), or
    /// `None` if it is the same as the last picture.
    pub fn draw(&mut self, time: f64, position: Option<f64>) -> Option<&[u8]> {
        let at = position.and_then(|position| {
            let step = position.max(0.0) as usize;
            let pass = *self.song.pass_at(step)?;
            Some((pass.from + (step - pass.start), pass))
        });
        let step = at.map(|(step, _)| step);
        let frame_rate = self.frame_rate;
        let views = &mut self.views;
        let mut changed = self.title.paint(time, frame_rate, theme::PANEL, |ui| {
            views.title_line(ui, step);
        });
        for (view, pane) in &mut self.panes {
            let background = match view {
                View::HarmonyKeys => theme::PANEL,
                _ => theme::BACKGROUND,
            };
            changed |= pane.paint(time, frame_rate, background, |ui| views.view(ui, *view, at));
        }
        let moved = std::mem::take(&mut self.moved);
        if !changed && !moved {
            return None;
        }
        self.canvas.fill(theme::BACKGROUND);
        self.canvas
            .copy_from(&self.title.canvas, self.title.left, self.title.top);
        for (_, pane) in &self.panes {
            self.canvas.copy_from(&pane.canvas, pane.left, pane.top);
        }
        Some(self.canvas.rgba())
    }

    /// The last picture drawn.
    pub fn picture(&self) -> &[u8] {
        self.canvas.rgba()
    }

    /// The song's length as it plays, in steps.
    pub fn song_length(&self) -> usize {
        self.song.length
    }

    /// Where song step `step` (as it plays) is: the bar and beat.
    pub fn place_name(&self, step: usize) -> String {
        let here = self.song.timeline_step(step).unwrap_or(0);
        let (bar, beat, _) = self.views.project.meter().position(here);
        crate::i18n::trf("Bar {}, beat {}", &[&bar, &beat])
    }
}

impl Views {
    /// The song's title, where it is, and its key, time and tempo there.
    fn title_line(&self, ui: &mut Ui, step: Option<usize>) {
        let here = step.unwrap_or(0);
        let meter = self.project.meter();
        let key = self.project.key_at(here);
        let time = meter.time_at(here);
        let tempo = self.project.tempo_map().tempo_at(here);
        let frame = egui::Frame::new()
            .fill(theme::PANEL)
            .inner_margin(egui::Margin::symmetric(10, 6))
            .stroke(egui::Stroke::new(1.0, theme::OUTLINE));
        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(
                    RichText::new(&self.title)
                        .size(20.0)
                        .strong()
                        .color(theme::TEXT),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} · {time} · {} BPM",
                            crate::i18n::key_name(key),
                            tempo.bpm().round()
                        ))
                        .size(15.0),
                    );
                    if let Some(step) = step {
                        let section = self
                            .project
                            .playlist()
                            .section_at(step)
                            .map(|(_, section)| {
                                format!(" · {}", crate::i18n::tr_name(&section.name))
                            })
                            .unwrap_or_default();
                        let (bar, beat, _) = meter.position(step);
                        ui.label(
                            RichText::new(format!(
                                "{}{section}    ",
                                crate::i18n::trf("Bar {}, beat {}", &[&bar, &beat])
                            ))
                            .size(15.0)
                            .color(theme::ACCENT),
                        );
                    }
                });
            });
        });
    }

    fn view(&mut self, ui: &mut Ui, view: View, at: Option<(usize, Pass)>) {
        let step = at.map(|(step, _)| step);
        let caption = |ui: &mut Ui, text: &str| {
            ui.label(RichText::new(text).small().color(Color32::from_gray(150)));
        };
        if view == View::HarmonyKeys {
            let here = step.map_or(self.project.scale, |step| self.project.key_at(step));
            let heard = step.map_or_else(Vec::new, |step| self.project.chord_notes_in_song(step));
            let hearing = step.map(|step| self.project.hearing_at(step));
            let frame = egui::Frame::new()
                .fill(theme::PANEL)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .stroke(egui::Stroke::new(1.0, theme::OUTLINE));
            egui::CentralPanel::default().frame(frame).show(ui, |ui| {
                self.harmony_keys.show(
                    ui,
                    &self.project,
                    here,
                    "",
                    &[],
                    &heard,
                    hearing.as_ref(),
                    step.map(crate::harmony_keys::Playhead::Song),
                    false,
                    None,
                    &mut self.parts,
                );
            });
            return;
        }
        let frame = egui::Frame::new()
            .fill(theme::BACKGROUND)
            .inner_margin(egui::Margin::same(4));
        egui::CentralPanel::default()
            .frame(frame)
            .show(ui, |ui| match view {
                View::ChordSheet => {
                    caption(ui, crate::i18n::tr("Chord sheet"));
                    self.chord_sheet.video(ui, &self.project, step);
                }
                View::LeadSheet => {
                    if let Some(lead_sheet) = &mut self.lead_sheet {
                        lead_sheet.video(ui, &self.project, &self.title, step);
                    }
                }
                View::Playlist => {
                    caption(ui, crate::i18n::tr("Playlist"));
                    self.playlist.video(ui, &self.project, at);
                }
                View::PianoRoll => {
                    // The pattern playing this channel's notes, if any.
                    let playing = step.and_then(|step| {
                        (0..self.project.patterns().len())
                            .filter(|&pattern| {
                                has_notes(&self.project, pattern, self.piano_channel)
                            })
                            .find_map(|pattern| {
                                let at = self.project.pattern_step_in_song(pattern, step)?;
                                Some((pattern, at))
                            })
                    });
                    if let Some((pattern, _)) = playing {
                        self.piano_pattern = pattern;
                    }
                    let channel = self
                        .project
                        .channels()
                        .get(self.piano_channel)
                        .map_or("", |channel| channel.name.as_str());
                    let pattern = self
                        .project
                        .pattern(self.piano_pattern)
                        .map_or("", |pattern| pattern.name.as_str());
                    caption(
                        ui,
                        &format!(
                            "{} · {} · {}",
                            crate::i18n::tr("Piano roll"),
                            crate::i18n::tr_name(pattern),
                            crate::i18n::tr_name(channel)
                        ),
                    );
                    self.piano_roll.video(
                        ui,
                        &self.project,
                        self.piano_pattern,
                        playing.map(|(_, at)| at),
                    );
                }
                View::HarmonyKeys => {}
            });
    }
}

/// Whether `channel` plays any note in `pattern`.
fn has_notes(project: &Project, pattern: usize, channel: usize) -> bool {
    project
        .pattern(pattern)
        .is_some_and(|pattern| channel < pattern.rows() && !pattern.notes(channel).is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stacking_shares_the_room_and_keeps_the_keys_at_the_bottom() {
        let layout = Layout::every_view();
        let keys = layout[View::HarmonyKeys].rect;
        assert_eq!(keys[3], 1.0);
        let mut bottom = 0.0;
        for view in [
            View::ChordSheet,
            View::LeadSheet,
            View::Playlist,
            View::PianoRoll,
        ] {
            let [_, top, _, end] = layout[view].rect;
            assert!(top > bottom, "{view:?} below the one before");
            assert!(end < keys[1], "{view:?} above the keys");
            bottom = end;
        }
    }

    #[test]
    fn boxes_stay_inside_and_not_too_small() {
        let mut placement = Placement {
            shown: true,
            rect: [0.5, 0.5, 0.8, 0.9],
            zoom: 1.0,
        };
        let near = |rect: [f32; 4], expected: [f32; 4]| {
            let close = rect.iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-5);
            assert!(close, "{rect:?}, not {expected:?}");
        };
        placement.move_by(vec2(0.5, -0.7));
        near(placement.rect, [0.7, 0.0, 1.0, 0.4]);
        placement.resize_by(vec2(-1.0, 1.0));
        near(placement.rect, [0.7, 0.0, 0.7 + Placement::MIN_SIZE, 1.0]);
        placement.zoom_by(10.0);
        assert_eq!(placement.zoom, Placement::MAX_ZOOM);
    }

    #[test]
    fn views_are_drawn_in_their_boxes() {
        // The chord sheet alone, in the right half of the room.
        let mut layout = Layout::default();
        layout[View::HarmonyKeys].shown = false;
        layout[View::ChordSheet].rect = [0.5, 0.0, 1.0, 1.0];
        let mut scene = Scene::new(Project::demo(), "Demo".into(), layout, Size::Hd, 30, 0);
        scene.draw(0.0, Some(0.0)).expect("the first picture");
        let picture = scene.picture();
        let pixel = |x: usize, y: usize| {
            let at = (y * 1280 + x) * 4;
            [picture[at], picture[at + 1], picture[at + 2]]
        };
        let background = [
            theme::BACKGROUND.r(),
            theme::BACKGROUND.g(),
            theme::BACKGROUND.b(),
        ];
        let below_title = TITLE_HEIGHT as usize + 1;
        // Nothing on the left, under the title...
        for y in below_title..720 {
            for x in 0..630 {
                assert_eq!(pixel(x, y), background, "({x}, {y})");
            }
        }
        // ...and the chord sheet on the right.
        let drawn = (below_title..720)
            .flat_map(|y| (640..1280).map(move |x| (x, y)))
            .filter(|&(x, y)| pixel(x, y) != background)
            .count();
        assert!(drawn > 10_000, "{drawn} pixels drawn");
    }

    #[test]
    fn zoomed_in_views_are_drawn_bigger() {
        // How many bright pixels (letters) the chord sheet has.
        fn ink(zoom: f32) -> usize {
            let mut layout = Layout::default();
            layout[View::HarmonyKeys].shown = false;
            layout[View::ChordSheet].rect = [0.0, 0.0, 1.0, 1.0];
            layout[View::ChordSheet].zoom = zoom;
            let mut scene = Scene::new(Project::demo(), "Demo".into(), layout, Size::Hd, 30, 0);
            scene.draw(0.0, Some(0.0));
            scene
                .picture()
                .as_chunks::<4>()
                .0
                .iter()
                .filter(|pixel| pixel[0] > 180)
                .count()
        }
        // Twice as big, each letter has four times the pixels; fewer bars
        // fit, but there is still much more ink.
        let (normal, big) = (ink(1.0), ink(2.0));
        assert!(big * 2 > normal * 3, "{normal} at 1, {big} at 2");
    }
}
