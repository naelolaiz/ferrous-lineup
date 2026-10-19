//! File > Export video: the song's sound, with pictures of the views you
//! pick, as an MP4 file.
//!
//! A dialog asks which views to show and how big the pictures are, and
//! shows a preview of a picture. On it, each view's box can be moved,
//! resized and zoomed. Then the song is played on an audio thread of its
//! own, as for a WAV file ([`super::export`]), a thirtieth of a second at
//! a time. Before each piece of sound, a [`Scene`] draws the views where
//! the music is, asking the audio thread where that is: the pictures
//! follow the sound the video carries, tempo changes and all.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

use eframe::egui::{self, Color32, CursorIcon, Rect, RichText, Sense, Stroke, vec2};
use lineup_core::lead_sheet::default_melody_channel;
use lineup_video::{VideoSettings, VideoWriter};

use super::LineupApp;
use super::export::{CANCELLED, Export, SongRender};
use crate::piano_roll::busiest_channel;
use crate::theme;
use crate::video::{Layout, Placement, Scene, Size, TITLE_HEIGHT, View};

/// Pictures a second.
const FRAME_RATE: u32 = 30;
/// The sound's sample rate, as for a WAV file. 44 100 frames a second
/// make exactly 1470 a picture.
const SAMPLE_RATE: u32 = 44_100;

/// The preview's width in the dialog, in points.
const PREVIEW_WIDTH: f32 = 640.0;

/// What the dialog remembers from one run of the app to the next.
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct Choices {
    layout: Layout,
    size: Size,
}

fn choices_id() -> egui::Id {
    egui::Id::unique("video-export-choices")
}

/// The Export video dialog.
#[derive(Default)]
pub(super) struct VideoDialog {
    open: bool,
    preview: Option<Preview>,
    /// The view last clicked on the preview, framed brighter.
    selected: Option<View>,
}

/// A picture of the video as it will look, at a moment of the song.
struct Preview {
    scene: Scene,
    size: Size,
    layout: Layout,
    /// The song step shown, as it plays.
    step: usize,
    drawn: Option<usize>,
    texture: Option<egui::TextureHandle>,
}

impl Preview {
    /// Draws the picture again if anything changed, and returns it.
    fn texture(&mut self, ctx: &egui::Context) -> Option<&egui::TextureHandle> {
        if self.drawn != Some(self.step) || self.texture.is_none() {
            self.drawn = Some(self.step);
            self.scene.draw(0.0, Some(self.step as f64));
            let (width, height) = self.size.pixels();
            let image =
                egui::ColorImage::from_rgba_premultiplied([width, height], self.scene.picture());
            match &mut self.texture {
                Some(texture) => texture.set(image, egui::TextureOptions::LINEAR),
                None => {
                    self.texture =
                        Some(ctx.load_texture("video-preview", image, egui::TextureOptions::LINEAR))
                }
            }
        }
        self.texture.as_ref()
    }

    /// Places the views as `layout` says, drawn again on the next frame.
    fn set_layout(&mut self, layout: Layout) {
        if layout != self.layout {
            self.layout = layout;
            self.scene.set_layout(layout);
            self.drawn = None;
        }
    }
}

impl LineupApp {
    /// Opens the dialog that asks what the video shows.
    pub(super) fn ask_for_video(&mut self) {
        if self.can_export() {
            self.video_dialog.open = true;
            self.video_dialog.preview = None;
        }
    }

    /// The channel the piano roll shows in a video: the one chosen in it,
    /// or the tune.
    fn video_piano_channel(&self) -> usize {
        self.piano_roll
            .chosen_channel()
            .or_else(|| default_melody_channel(&self.project))
            .or_else(|| busiest_channel(&self.project.patterns()[self.pattern]))
            .unwrap_or(0)
    }

    /// The dialog, while it is open.
    pub(super) fn video_dialog(&mut self, ctx: &egui::Context) {
        if !self.video_dialog.open {
            return;
        }
        let mut choices: Choices = ctx
            .data_mut(|data| data.get_persisted(choices_id()))
            .unwrap_or_default();
        let before = choices;
        // The preview starts at the playhead, if the song was played.
        let rebuild = self
            .video_dialog
            .preview
            .as_ref()
            .is_none_or(|preview| preview.size != choices.size);
        if rebuild {
            let step = self
                .video_dialog
                .preview
                .as_ref()
                .map_or(0, |preview| preview.step);
            self.video_dialog.preview = Some(Preview {
                scene: Scene::new(
                    self.project.clone(),
                    self.file.stem(),
                    choices.layout,
                    choices.size,
                    FRAME_RATE,
                    self.video_piano_channel(),
                ),
                size: choices.size,
                layout: choices.layout,
                step,
                drawn: None,
                texture: None,
            });
        }
        let mut export = false;
        let mut close = false;
        let can_export = self.can_export();
        let dialog = &mut self.video_dialog;
        let modal = egui::Modal::new(egui::Id::unique("video-export")).show(ctx, |ui| {
            ui.set_width(PREVIEW_WIDTH);
            ui.heading("Export video");
            ui.label("The song's sound, with the views you pick playing along.");
            ui.add_space(6.0);
            let layout = &mut choices.layout;
            egui::Grid::new("video-views")
                .num_columns(2)
                .spacing(vec2(16.0, 2.0))
                .show(ui, |ui| {
                    for view in View::ALL {
                        let placement = &mut layout[view];
                        let mut name = RichText::new(view.name());
                        if dialog.selected == Some(view) && placement.shown {
                            name = name.color(theme::ACCENT);
                        }
                        let shown = ui.checkbox(&mut placement.shown, name);
                        theme::hint(shown.clone(), view.hint());
                        if shown.changed() {
                            dialog.selected = placement.shown.then_some(view);
                            // A view added or taken away: the views are
                            // stacked again to share the picture.
                            layout.stack();
                        }
                        let placement = &mut layout[view];
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(placement.shown && view.zooms(), |ui| {
                                let zoom = ui.add(
                                    egui::Slider::new(
                                        &mut placement.zoom,
                                        Placement::MIN_ZOOM..=Placement::MAX_ZOOM,
                                    )
                                    .custom_formatter(|zoom, _| format!("{:.0}%", zoom * 100.0))
                                    .custom_parser(|text| {
                                        text.trim_end_matches('%').trim().parse::<f64>().ok().map(|percent| percent / 100.0)
                                    })
                                    .text("zoom"),
                                );
                                if !view.zooms() {
                                    zoom.on_disabled_hover_text(
                                        "The lead sheet fills the width of its box: make the box wider to draw it bigger",
                                    );
                                }
                            });
                        });
                        ui.end_row();
                    }
                });
            ui.add_space(6.0);

            if let Some(preview) = &mut dialog.preview {
                preview.set_layout(*layout);
                preview_picture(ui, preview, layout, &mut dialog.selected);
                ui.horizontal(|ui| {
                    if ui
                        .button("Stack")
                        .on_hover_text("Put the views one above the other again, sharing the picture")
                        .clicked()
                    {
                        layout.stack();
                    }
                    ui.separator();
                    ui.label("Preview at");
                    let last = preview.scene.song_length().saturating_sub(1);
                    ui.add(
                        egui::Slider::new(&mut preview.step, 0..=last)
                            .show_value(false)
                            .step_by(1.0),
                    );
                    ui.label(preview.scene.place_name(preview.step));
                });
                ui.label(
                    RichText::new(
                        "Drag a view to move it, drag its corner to resize it, \
                         and scroll over it to zoom.",
                    )
                    .small()
                    .color(theme::TEXT_DIM),
                );
            }

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Size").strong());
                for size in Size::ALL {
                    ui.radio_value(&mut choices.size, size, size.name());
                }
            });
            ui.label(
                RichText::new("An MP4 file: H.264 pictures, 30 a second, and MP3 sound.")
                    .small()
                    .color(theme::TEXT_DIM),
            );
            ui.separator();
            ui.horizontal(|ui| {
                let ready = choices.layout.any() && can_export;
                if ui
                    .add_enabled(ready, egui::Button::new("Export…"))
                    .on_disabled_hover_text("Tick a view to show")
                    .clicked()
                {
                    export = true;
                }
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });
        if choices != before {
            ctx.data_mut(|data| data.insert_persisted(choices_id(), choices));
        }
        if modal.should_close() || close || export {
            self.video_dialog.open = false;
            self.video_dialog.preview = None;
        }
        if export {
            self.export_video(choices);
        }
    }

    /// Asks where to save, then starts making the video there.
    fn export_video(&mut self, choices: Choices) {
        if !self.can_export() {
            return;
        }
        let Some(mut path) = rfd::FileDialog::new()
            .add_filter("MP4 video", &["mp4"])
            .set_file_name(self.file.export_name("mp4"))
            .save_file()
        else {
            return;
        };
        if path.extension().is_none() {
            path.set_extension("mp4");
        }
        self.start_video(path, choices.layout, choices.size);
    }

    /// Starts making a video of the song at `path`, in the background.
    fn start_video(&mut self, path: std::path::PathBuf, layout: Layout, size: Size) {
        let SongRender {
            mut renderer,
            frames,
        } = self.song_render(SAMPLE_RATE);
        let per_picture = (SAMPLE_RATE / FRAME_RATE) as usize;
        let pictures = frames.div_ceil(per_picture);
        let piano_channel = self.video_piano_channel();
        let project = self.project.clone();
        let title = self.file.stem();
        let (width, height) = size.pixels();
        let settings = VideoSettings {
            width,
            height,
            frame_rate: FRAME_RATE,
            sample_rate: SAMPLE_RATE,
        };

        let done = Arc::new(AtomicUsize::new(0));
        let cancel = Arc::new(AtomicBool::new(false));
        let worker = {
            let done = Arc::clone(&done);
            let cancel = Arc::clone(&cancel);
            let path = path.clone();
            thread::spawn(move || {
                // The views are made here, on the export's own thread.
                let mut scene = Scene::new(project, title, layout, size, FRAME_RATE, piano_channel);
                let mut writer = VideoWriter::create(&path, settings)?;
                let mut sound = vec![0.0; per_picture * 2];
                let mut record = || {
                    for picture in 0..pictures {
                        if cancel.load(Ordering::Relaxed) {
                            return Err(CANCELLED.to_string());
                        }
                        let time = picture as f64 / f64::from(FRAME_RATE);
                        // Before the first sound, the song is about to
                        // start; after it, where the sound so far ended.
                        let position = if picture == 0 {
                            Some(0.0)
                        } else {
                            renderer.position()
                        };
                        match scene.draw(time, position) {
                            Some(picture) => writer.add_picture(picture)?,
                            None => writer.repeat_picture(),
                        }
                        renderer.render(&mut sound);
                        writer.add_sound(&sound)?;
                        done.store(picture + 1, Ordering::Relaxed);
                    }
                    Ok(())
                };
                match record() {
                    Ok(()) => writer.finish(),
                    Err(err) => {
                        writer.cancel();
                        Err(err)
                    }
                }
            })
        };
        self.export = Some(Export::new(path, pictures, done, Some(cancel), worker));
        self.message = None;
        self.notice = None;
    }
}

/// The preview, with a frame on each view's box to drag, resize and
/// zoom it by.
fn preview_picture(
    ui: &mut egui::Ui,
    preview: &mut Preview,
    layout: &mut Layout,
    selected: &mut Option<View>,
) {
    let (width, height) = preview.size.pixels();
    let size = vec2(PREVIEW_WIDTH, PREVIEW_WIDTH * height as f32 / width as f32);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter_at(rect);
    match preview.texture(ui.ctx()) {
        Some(texture) => {
            painter.image(
                texture.id(),
                rect,
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        None => {
            painter.rect_filled(rect, 0.0, theme::BACKGROUND);
        }
    }
    // The room under the title line, which the boxes are parts of.
    // Both sizes are 720 points high.
    let title = TITLE_HEIGHT / 720.0 * rect.height();
    let room = Rect::from_min_max(rect.min + vec2(0.0, title), rect.max);
    let on_screen = |placement: &Placement| {
        let [left, top, right, bottom] = placement.rect;
        Rect::from_min_max(
            room.min + vec2(left * room.width(), top * room.height()),
            room.min + vec2(right * room.width(), bottom * room.height()),
        )
    };
    let id = egui::Id::unique("video-preview");
    let shown: Vec<View> = layout.shown().collect();
    // Later views are on top, so they take the pointer first: egui gives
    // it to what was asked about last.
    for &view in &shown {
        let placement = &mut layout[view];
        let area = on_screen(placement);
        let body = ui
            .interact(
                area,
                id.with((view.name(), "move")),
                Sense::click_and_drag(),
            )
            .on_hover_cursor(CursorIcon::Grab);
        let corner = Rect::from_center_size(area.max - vec2(5.0, 5.0), vec2(12.0, 12.0));
        let handle = ui
            .interact(corner, id.with((view.name(), "size")), Sense::drag())
            .on_hover_cursor(CursorIcon::ResizeNwSe);
        let scale = vec2(1.0 / room.width(), 1.0 / room.height());
        if body.dragged() {
            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
            placement.move_by(body.drag_delta() * scale);
        }
        if handle.dragged() {
            placement.resize_by(handle.drag_delta() * scale);
        }
        if body.drag_started() || body.clicked() || handle.drag_started() {
            *selected = Some(view);
        }
        if body.hovered() && view.zooms() {
            let scroll = ui.input(|input| input.smooth_scroll_delta.y);
            if scroll != 0.0 {
                placement.zoom_by((scroll / 500.0).exp());
                *selected = Some(view);
            }
        }
    }
    // The frames, drawn after all the dragging so they show where the
    // boxes are now.
    for &view in &shown {
        let placement = layout[view];
        let area = on_screen(&placement).shrink(0.5);
        let bright = *selected == Some(view);
        let stroke = if bright {
            Stroke::new(2.0, theme::ACCENT)
        } else {
            Stroke::new(1.0, theme::TEXT_DIM.gamma_multiply(0.7))
        };
        painter.rect_stroke(area, 0.0, stroke, egui::StrokeKind::Inside);
        let label = if view.zooms() {
            format!("{} · {:.0}%", view.name(), placement.zoom * 100.0)
        } else {
            view.name().to_string()
        };
        let galley = painter.layout_no_wrap(label, egui::FontId::proportional(11.0), theme::TEXT);
        let tag = Rect::from_min_size(area.min, galley.size() + vec2(8.0, 4.0));
        painter.rect_filled(tag, 0.0, theme::OUTLINE.gamma_multiply(0.85));
        painter.galley(tag.min + vec2(4.0, 2.0), galley, theme::TEXT);
        let corner = Rect::from_min_size(area.max - vec2(8.0, 8.0), vec2(8.0, 8.0));
        painter.rect_filled(
            corner,
            0.0,
            if bright {
                theme::ACCENT
            } else {
                theme::TEXT_DIM
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Audio;
    use lineup_core::Project;

    fn app() -> LineupApp {
        let ctx = egui::Context::default();
        LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()))
    }

    #[test]
    fn exports_a_video_with_every_view() {
        let mut app = app();
        app.replace_project(Project::demo(), None);
        let path =
            std::env::temp_dir().join(format!("lineup-app-video-{}.mp4", std::process::id()));
        app.start_video(path.clone(), Layout::every_view(), Size::Hd);
        let export = app.export.take().expect("the export started");
        export.wait().unwrap();
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).ok();
        assert_eq!(&bytes[4..8], b"ftyp");
        assert!(bytes.windows(4).any(|name| name == b"moov"));
    }

    #[test]
    fn a_stopped_export_leaves_no_file() {
        let mut app = app();
        app.replace_project(Project::demo(), None);
        let path = std::env::temp_dir().join(format!("lineup-app-stop-{}.mp4", std::process::id()));
        app.start_video(path.clone(), Layout::default(), Size::Hd);
        let export = app.export.take().expect("the export started");
        export.cancel();
        assert_eq!(export.wait(), Err(CANCELLED.to_string()));
        assert!(!path.exists());
        let mut temporary = path.into_os_string();
        temporary.push(".tmp");
        assert!(!std::path::Path::new(&temporary).exists());
    }
}
