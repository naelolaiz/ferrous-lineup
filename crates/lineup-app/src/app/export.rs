//! Saving the song as a WAV file, in the background, or as a MIDI file.
//!
//! The WAV export plays the song through its own audio thread, a
//! [`Renderer`], with its own copy of every instrument and note, so it
//! cannot disturb what you hear. It runs on a thread of its own: a long
//! song takes a few seconds to render, and the window keeps drawing (and
//! the music keeps playing) meanwhile.
//!
//! A MIDI file holds only the notes, a few bytes each, so it is written
//! at once.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use eframe::egui;
use lineup_audio::{Command, PlayMode, Renderer, export_wav};
use lineup_core::sequencer::frames_per_step;
use lineup_core::{pattern_to_midi, song_to_midi};

use super::LineupApp;

/// CD quality: 44 100 frames per second, in stereo.
const EXPORT_SAMPLE_RATE: u32 = 44_100;
const EXPORT_CHANNELS: usize = 2;

/// How long the last notes are given to ring out after the song ends.
const TAIL_SECONDS: f64 = 2.0;

/// An export that is running.
pub(super) struct Export {
    path: PathBuf,
    frames: usize,
    /// How many frames are done. The export thread writes it and the UI
    /// thread reads it; an atomic needs no lock for that.
    done: Arc<AtomicUsize>,
    /// The thread doing the work. Joining it gives back what its closure
    /// returned: here, whether the file was written.
    worker: JoinHandle<Result<(), String>>,
}

impl Export {
    /// How far along the export is, from 0.0 to 1.0.
    pub(super) fn progress(&self) -> f32 {
        self.done.load(Ordering::Relaxed) as f32 / self.frames.max(1) as f32
    }

    /// The name of the file being written, such as "song.wav".
    pub(super) fn file_name(&self) -> String {
        file_name(&self.path)
    }
}

/// What a MIDI export holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MidiExport {
    /// The whole song, as the playlist places the patterns.
    Song,
    /// The selected pattern, once.
    Pattern,
}

fn file_name(path: &std::path::Path) -> String {
    path.file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
}

impl LineupApp {
    /// Whether File > Export WAV can be used now: there must be a song,
    /// and no other export running.
    pub(super) fn can_export(&self) -> bool {
        self.export.is_none() && self.project.song_length() > 0
    }

    /// Asks where to save, then starts writing the song there.
    pub(super) fn export_song(&mut self) {
        if !self.can_export() {
            return;
        }
        let Some(mut path) = rfd::FileDialog::new()
            .add_filter("WAV audio", &["wav"])
            .set_file_name(self.file.export_name("wav"))
            .save_file()
        else {
            return;
        };
        if path.extension().is_none() {
            path.set_extension("wav");
        }

        let mut renderer = Renderer::new(EXPORT_SAMPLE_RATE, EXPORT_CHANNELS);
        for command in self.project_commands(EXPORT_SAMPLE_RATE, 1.0) {
            renderer.send(command);
        }

        // The song loops when it reaches its end, so silent steps are
        // added after it: enough for the tail, and one more, so the
        // export ends before the song would start again.
        // The whole song, every repeat included, even when song mode only
        // loops part of it.
        let song = self.project.arrangement();
        let song_length = song.length;
        // The silent steps play at the tempo the song ends at: the
        // audio thread plays steps past the end of the tempo list at
        // its last tempo.
        let tempos = self.project.played_tempos(&song);
        let end_tempo = tempos.last().copied().unwrap_or(self.project.tempo);
        let step_frames = frames_per_step(end_tempo, EXPORT_SAMPLE_RATE);
        let reverb = self.project.mixer.reverb;
        let tail_seconds = if reverb.enabled {
            TAIL_SECONDS + f64::from(reverb.decay + reverb.pre_delay_ms / 1000.0)
        } else {
            TAIL_SECONDS
        };
        let tail_frames = tail_seconds * f64::from(EXPORT_SAMPLE_RATE);
        let silent_steps = (tail_frames / step_frames).ceil() as usize + 1;
        let song_frames = self.project.played_seconds(&song) * f64::from(EXPORT_SAMPLE_RATE);
        renderer.send(Command::SetSong {
            plays: song.plays,
            length: song_length + silent_steps,
        });
        renderer.send(Command::SetSongTempos(tempos));
        renderer.send(Command::SetMode(PlayMode::Song));
        renderer.send(Command::Play);
        let frames = (song_frames + tail_frames) as usize;

        let done = Arc::new(AtomicUsize::new(0));
        // The thread gets its own `Arc` pointing to the same counter, and
        // its own copy of the path. `move` gives the closure ownership of
        // them, and of the renderer: from now on they belong to the thread.
        let worker = {
            let done = Arc::clone(&done);
            let path = path.clone();
            thread::spawn(move || {
                export_wav(&mut renderer, &path, frames, |frames_done| {
                    done.store(frames_done, Ordering::Relaxed);
                })
                .map_err(|err| err.to_string())
            })
        };
        self.export = Some(Export {
            path,
            frames,
            done,
            worker,
        });
        self.message = None;
        self.notice = None;
    }

    /// Asks where to save, then writes the song or the selected pattern
    /// as a Standard MIDI File.
    pub(super) fn export_midi(&mut self, what: MidiExport) {
        let (bytes, name) = match what {
            MidiExport::Song => (song_to_midi(&self.project), self.file.export_name("mid")),
            MidiExport::Pattern => {
                let pattern = &self.project.patterns()[self.pattern];
                // A pattern name could hold a `/`, which a file name cannot.
                let name = format!("{}.mid", pattern.name.replace(['/', '\\'], "-"));
                (pattern_to_midi(&self.project, self.pattern), name)
            }
        };
        let Some(mut path) = rfd::FileDialog::new()
            .add_filter("MIDI file", &["mid", "midi"])
            .set_file_name(name)
            .save_file()
        else {
            return;
        };
        if path.extension().is_none() {
            path.set_extension("mid");
        }
        let name = file_name(&path);
        match std::fs::write(&path, bytes) {
            Ok(()) => {
                self.notice = Some(format!("Exported {name}"));
                self.message = None;
            }
            Err(err) => self.message = Some(format!("Could not export {name}: {err}")),
        }
    }

    /// Notices when the export has finished, and keeps the progress bar
    /// moving while it runs.
    pub(super) fn check_export(&mut self, ctx: &egui::Context) {
        let Some(export) = &self.export else {
            return;
        };
        if !export.worker.is_finished() {
            ctx.request_repaint_after(Duration::from_millis(100));
            return;
        }
        // `take` moves the export out of the `Option` and leaves `None`,
        // so `join` can take ownership of the thread's handle.
        if let Some(export) = self.export.take() {
            let name = export.file_name();
            match export.worker.join() {
                Ok(Ok(())) => self.notice = Some(format!("Exported {name}")),
                Ok(Err(err)) => self.message = Some(format!("Could not export {name}: {err}")),
                // `join` fails only if the thread panicked.
                Err(_) => self.message = Some(format!("Exporting {name} failed")),
            }
        }
    }
}
