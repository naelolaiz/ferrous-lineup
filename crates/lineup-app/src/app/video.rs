//! File > Export video: the song's sound, with pictures of the views you
//! pick, as an MP4 file.
//!
//! A small dialog asks which views to show and how big the pictures are.
//! Then the song is played on an audio thread of its own, as for a WAV
//! file ([`super::export`]), a thirtieth of a second at a time. Before
//! each piece of sound, a [`Scene`] draws the views where the music is,
//! asking the audio thread where that is: the pictures follow the sound
//! the video carries, tempo changes and all.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

use eframe::egui::{self, RichText};
use lineup_core::lead_sheet::default_melody_channel;
use lineup_video::{VideoSettings, VideoWriter};

use super::LineupApp;
use super::export::{CANCELLED, Export, SongRender};
use crate::piano_roll::busiest_channel;
use crate::theme;
use crate::video::{Scene, Size, Views};

/// Pictures a second.
const FRAME_RATE: u32 = 30;
/// The sound's sample rate, as for a WAV file. 44 100 frames a second
/// make exactly 1470 a picture.
const SAMPLE_RATE: u32 = 44_100;

/// What the dialog remembers from one run of the app to the next.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct Choices {
    views: Views,
    size: Size,
}

impl Default for Choices {
    fn default() -> Self {
        Choices {
            views: Views::default(),
            size: Size::Hd,
        }
    }
}

fn choices_id() -> egui::Id {
    egui::Id::unique("video-export-choices")
}

/// The Export video dialog.
#[derive(Default)]
pub(super) struct VideoDialog {
    open: bool,
}

impl LineupApp {
    /// Opens the dialog that asks what the video shows.
    pub(super) fn ask_for_video(&mut self) {
        if self.can_export() {
            self.video_dialog.open = true;
        }
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
        let mut export = false;
        let mut close = false;
        let modal = egui::Modal::new(egui::Id::unique("video-export")).show(ctx, |ui| {
            ui.set_width(340.0);
            ui.heading("Export video");
            ui.label("The song's sound, with the views you pick playing along.");
            ui.add_space(6.0);
            ui.label(RichText::new("Show").strong());
            let views = &mut choices.views;
            let tick = |ui: &mut egui::Ui, on: &mut bool, name: &str, hint: &str| {
                theme::hint(ui.checkbox(on, name), hint);
            };
            tick(
                ui,
                &mut views.chord_sheet,
                "Chord sheet",
                "The harmony bar by bar, the bar playing framed",
            );
            tick(
                ui,
                &mut views.lead_sheet,
                "Lead sheet",
                "The tune on a staff with its chords, the bar playing lit",
            );
            tick(
                ui,
                &mut views.playlist,
                "Playlist",
                "The tracks and their clips, with the playhead",
            );
            tick(
                ui,
                &mut views.piano_roll,
                "Piano roll",
                "The notes of the channel the piano roll has selected, or the tune",
            );
            tick(
                ui,
                &mut views.harmony_keys,
                "Harmony keys",
                "The chords of the key, the one playing lit, and the little piano",
            );
            ui.add_space(6.0);
            ui.label(RichText::new("Size").strong());
            ui.horizontal(|ui| {
                for size in Size::ALL {
                    ui.radio_value(&mut choices.size, size, size.name());
                }
            });
            ui.add_space(6.0);
            ui.label(
                RichText::new("An MP4 file: H.264 pictures, 30 a second, and MP3 sound.")
                    .small()
                    .color(theme::TEXT_DIM),
            );
            ui.separator();
            ui.horizontal(|ui| {
                let ready = choices.views.any() && self.can_export();
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
        if modal.should_close() || close {
            self.video_dialog.open = false;
        }
        if export {
            self.video_dialog.open = false;
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
        self.start_video(path, choices.views, choices.size);
    }

    /// Starts making a video of the song at `path`, in the background.
    fn start_video(&mut self, path: std::path::PathBuf, views: Views, size: Size) {
        let SongRender {
            mut renderer,
            frames,
        } = self.song_render(SAMPLE_RATE);
        let per_picture = (SAMPLE_RATE / FRAME_RATE) as usize;
        let pictures = frames.div_ceil(per_picture);
        // The piano roll shows the channel chosen in it, or the tune.
        let piano_channel = self
            .piano_roll
            .chosen_channel()
            .or_else(|| default_melody_channel(&self.project))
            .or_else(|| busiest_channel(&self.project.patterns()[self.pattern]))
            .unwrap_or(0);
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
                let mut scene = Scene::new(project, title, views, size, FRAME_RATE, piano_channel);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Audio;
    use lineup_core::Project;

    fn app() -> LineupApp {
        let ctx = egui::Context::default();
        LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()))
    }

    /// Every view, so each is drawn at least once without a window.
    const EVERY_VIEW: Views = Views {
        chord_sheet: true,
        lead_sheet: true,
        playlist: true,
        piano_roll: true,
        harmony_keys: true,
    };

    #[test]
    fn exports_a_video_with_every_view() {
        let mut app = app();
        app.replace_project(Project::demo(), None);
        let path =
            std::env::temp_dir().join(format!("lineup-app-video-{}.mp4", std::process::id()));
        app.start_video(path.clone(), EVERY_VIEW, Size::Hd);
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
        app.start_video(path.clone(), Views::default(), Size::Hd);
        let export = app.export.take().expect("the export started");
        export.cancel();
        assert_eq!(export.wait(), Err(CANCELLED.to_string()));
        assert!(!path.exists());
        let mut temporary = path.into_os_string();
        temporary.push(".tmp");
        assert!(!std::path::Path::new(&temporary).exists());
    }
}
