//! The pictures of a video: the views you chose, drawn as the song plays.
//!
//! The views are drawn by the same code as in the window, but not in the
//! window: a [`Scene`] has an egui context of its own, which it tells the
//! size of the picture and the time, as eframe tells the window's. What
//! egui draws is painted in memory by [`lineup_video::Canvas`], so the
//! window, the music you hear and the video being made never get in
//! each other's way.
//!
//! The views are stacked: a title line at the top, the harmony keys at
//! the bottom, and the chord sheet, the lead sheet, the playlist and the
//! piano roll sharing the room between, in that order.

use eframe::egui::{self, Color32, Pos2, Rect, RichText, Ui, UiBuilder, vec2};
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

/// Which views a video shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Views {
    pub chord_sheet: bool,
    pub lead_sheet: bool,
    pub playlist: bool,
    pub piano_roll: bool,
    pub harmony_keys: bool,
}

impl Default for Views {
    /// The chords, bar by bar and as keys lighting up.
    fn default() -> Self {
        Views {
            chord_sheet: true,
            lead_sheet: false,
            playlist: false,
            piano_roll: false,
            harmony_keys: true,
        }
    }
}

impl Views {
    /// Whether any view is chosen at all.
    pub fn any(self) -> bool {
        self.chord_sheet || self.lead_sheet || self.playlist || self.piano_roll || self.harmony_keys
    }
}

/// How big the pictures are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    /// 1280 × 720, "HD".
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
    ctx: egui::Context,
    canvas: Canvas,
    size: Size,
    frame_rate: u32,
    project: Project,
    /// The whole song, every repeat played, as the sound is made.
    song: Arrangement,
    title: String,
    views: Views,
    chord_sheet: ChordSheet,
    lead_sheet: Option<LeadSheetWindow>,
    playlist: PlaylistView,
    piano_roll: PianoRoll,
    /// The channel the piano roll shows, and the pattern it shows it in:
    /// the one playing, or the last one that played.
    piano_channel: usize,
    piano_pattern: usize,
    harmony_keys: HarmonyKeys,
    parts: StyleParts,
    /// Whether a picture was drawn yet. The first one is drawn twice:
    /// egui sizes some things on its first frame.
    started: bool,
}

impl Scene {
    /// The views of `project`, called `title`, in pictures of `size`,
    /// `frame_rate` of them a second. The piano roll shows `piano_channel`.
    pub fn new(
        project: Project,
        title: String,
        views: Views,
        size: Size,
        frame_rate: u32,
        piano_channel: usize,
    ) -> Self {
        let ctx = egui::Context::default();
        theme::apply(&ctx);
        // Letters egui's fonts lack (🔑, →) come from the system's fonts,
        // as in the window.
        ctx.add_font_provider(std::sync::Arc::new(
            egui_system_font_provider::SystemFontProvider::new(),
        ));
        let lead_sheet = views.lead_sheet.then(|| LeadSheetWindow::new(&ctx));
        let (width, height) = size.pixels();
        let piano_pattern = (0..project.patterns().len())
            .find(|&pattern| has_notes(&project, pattern, piano_channel))
            .unwrap_or(0);
        Scene {
            ctx,
            canvas: Canvas::new(width, height),
            size,
            frame_rate,
            song: project.arrangement(),
            project,
            title,
            views,
            chord_sheet: ChordSheet::new(),
            lead_sheet,
            playlist: PlaylistView::new(),
            piano_roll: PianoRoll::new(piano_channel),
            piano_channel,
            piano_pattern,
            harmony_keys: HarmonyKeys::new(),
            parts: StyleParts::default(),
            started: false,
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
        let mut changed = false;
        if !self.started {
            self.started = true;
            changed |= self.paint(time, at);
        }
        changed |= self.paint(time, at);
        changed.then(|| self.canvas.rgba())
    }

    /// Draws the views, and paints them if anything changed.
    fn paint(&mut self, time: f64, at: Option<(usize, Pass)>) -> bool {
        let ppp = self.size.pixels_per_point();
        let (width, height) = self.size.pixels();
        let mut input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::ZERO,
                vec2(width as f32, height as f32) / ppp,
            )),
            time: Some(time),
            predicted_dt: 1.0 / self.frame_rate as f32,
            ..Default::default()
        };
        input
            .viewports
            .entry(egui::ViewportId::ROOT)
            .or_default()
            .native_pixels_per_point = Some(ppp);
        let ctx = self.ctx.clone();
        let mut output = ctx.run_ui(input, |ui| self.ui(ui, at));
        let shapes = std::mem::take(&mut output.shapes);
        let primitives = ctx.tessellate(shapes, output.pixels_per_point);
        self.canvas.paint(
            primitives,
            &mut output.textures_delta,
            output.pixels_per_point,
            theme::BACKGROUND,
        )
    }

    /// Everything in the picture.
    fn ui(&mut self, ui: &mut Ui, at: Option<(usize, Pass)>) {
        let step = at.map(|(step, _)| step);
        let bar = |fill| {
            egui::Frame::new()
                .fill(fill)
                .inner_margin(egui::Margin::symmetric(10, 6))
                .stroke(egui::Stroke::new(1.0, theme::OUTLINE))
        };
        egui::Panel::top("video-title")
            .frame(bar(theme::PANEL))
            .show(ui, |ui| self.title_line(ui, step));
        if self.views.harmony_keys {
            let here = step.map_or(self.project.scale, |step| self.project.key_at(step));
            let heard = step.map_or_else(Vec::new, |step| self.project.chord_notes_in_song(step));
            egui::Panel::bottom("video-harmony-keys")
                .frame(bar(theme::PANEL).inner_margin(egui::Margin::symmetric(8, 4)))
                .show(ui, |ui| {
                    self.harmony_keys.show(
                        ui,
                        &self.project,
                        here,
                        "",
                        &[],
                        &heard,
                        false,
                        None,
                        &mut self.parts,
                    );
                });
        }

        let mut shown: Vec<View> = Vec::new();
        if self.views.chord_sheet {
            shown.push(View::ChordSheet);
        }
        if self.views.lead_sheet {
            shown.push(View::LeadSheet);
        }
        if self.views.playlist {
            shown.push(View::Playlist);
        }
        if self.views.piano_roll {
            shown.push(View::PianoRoll);
        }
        egui::CentralPanel::default_margins().show(ui, |ui| {
            if shown.is_empty() {
                return;
            }
            // The room is shared out evenly, with a gap between views.
            const GAP: f32 = 8.0;
            let room = ui.available_rect_before_wrap();
            let height = (room.height() - GAP * (shown.len() - 1) as f32) / shown.len() as f32;
            for (index, view) in shown.into_iter().enumerate() {
                let top = room.top() + index as f32 * (height + GAP);
                let rect =
                    Rect::from_min_size(egui::pos2(room.left(), top), vec2(room.width(), height));
                ui.scope_builder(UiBuilder::new().max_rect(rect).id_salt(index), |ui| {
                    ui.set_clip_rect(rect);
                    self.view(ui, view, at);
                });
            }
        });
    }

    /// The song's title, where it is, and its key, time and tempo there.
    fn title_line(&self, ui: &mut Ui, step: Option<usize>) {
        let here = step.unwrap_or(0);
        let meter = self.project.meter();
        let key = self.project.key_at(here);
        let time = meter.time_at(here);
        let tempo = self.project.tempo_map().tempo_at(here);
        ui.horizontal(|ui| {
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
                        key.name(),
                        tempo.bpm().round()
                    ))
                    .size(15.0),
                );
                if let Some(step) = step {
                    let section = self
                        .project
                        .playlist()
                        .section_at(step)
                        .map(|(_, section)| format!(" · {}", section.name))
                        .unwrap_or_default();
                    let (bar, beat, _) = meter.position(step);
                    ui.label(
                        RichText::new(format!("Bar {bar}, beat {beat}{section}    "))
                            .size(15.0)
                            .color(theme::ACCENT),
                    );
                }
            });
        });
    }

    fn view(&mut self, ui: &mut Ui, view: View, at: Option<(usize, Pass)>) {
        let step = at.map(|(step, _)| step);
        let caption = |ui: &mut Ui, text: &str| {
            ui.label(RichText::new(text).small().color(Color32::from_gray(150)));
        };
        match view {
            View::ChordSheet => {
                caption(ui, "Chord sheet");
                self.chord_sheet.video(ui, &self.project, step);
            }
            View::LeadSheet => {
                if let Some(lead_sheet) = &mut self.lead_sheet {
                    lead_sheet.video(ui, &self.project, &self.title, step);
                }
            }
            View::Playlist => {
                caption(ui, "Playlist");
                self.playlist.video(ui, &self.project, at);
            }
            View::PianoRoll => {
                // The pattern playing this channel's notes, if any.
                let playing = step.and_then(|step| {
                    (0..self.project.patterns().len())
                        .filter(|&pattern| has_notes(&self.project, pattern, self.piano_channel))
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
                caption(ui, &format!("Piano roll · {pattern} · {channel}"));
                self.piano_roll.video(
                    ui,
                    &self.project,
                    self.piano_pattern,
                    playing.map(|(_, at)| at),
                );
            }
        }
    }
}

/// The views that share the middle of the picture.
#[derive(Debug, Clone, Copy)]
enum View {
    ChordSheet,
    LeadSheet,
    Playlist,
    PianoRoll,
}

/// Whether `channel` plays any note in `pattern`.
fn has_notes(project: &Project, pattern: usize, channel: usize) -> bool {
    project
        .pattern(pattern)
        .is_some_and(|pattern| channel < pattern.rows() && !pattern.notes(channel).is_empty())
}
