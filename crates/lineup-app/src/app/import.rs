//! File > Import recording: an MP3, WAV, FLAC or Ogg file becomes a new
//! project that plays it, with a harmony track that follows its chords.
//! And File > Import MIDI file, at the end.
//!
//! Listening takes a few seconds, so it runs on a thread of its own, like
//! the WAV export, and the window keeps drawing meanwhile. The thread
//! decodes the file, analyzes it, saves it again as a WAV file with a
//! short lead-in (see [`lineup_listen::arrange`]), and builds the project.
//! The UI thread only swaps the project in when it is ready.

use std::path::{Path, PathBuf};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use eframe::egui;
use lineup_audio::PlayMode;
use lineup_core::harmony::key_finding::KeySpan;
use lineup_core::{Project, midi_file};
use lineup_listen::{arrange, decode_file};

use super::LineupApp;

/// What the import thread gives back: the project, a summary of what it
/// heard and the key changes it heard (in song steps), or why it failed.
type Outcome = Result<(Project, String, Vec<KeySpan>), String>;

/// An import that is running.
pub(super) struct Import {
    worker: JoinHandle<Outcome>,
}

/// Where the recording is saved for the sampler: next to the original,
/// as "song.ferrous-lineup.wav", or in the temporary folder if that folder
/// cannot be written to.
fn wav_path(original: &Path) -> PathBuf {
    let stem = original
        .file_stem()
        .map_or_else(|| "recording".into(), |stem| stem.to_string_lossy());
    let name = format!("{stem}.ferrous-lineup.wav");
    original.with_file_name(&name)
}

/// The whole job, run on the import thread.
fn listen(path: &Path) -> Outcome {
    let recording = decode_file(path).map_err(|err| err.to_string())?;
    let analysis = lineup_listen::analyze(&recording);
    if analysis.beats.len() < 2 {
        return Err(crate::i18n::tr("no beat could be heard in it").to_string());
    }
    let lead_in = arrange::lead_in(&analysis);

    let mut wav = wav_path(path);
    if arrange::write_wav(&recording, lead_in.seconds, &wav).is_err() {
        wav = std::env::temp_dir().join(wav.file_name().unwrap_or_default());
        arrange::write_wav(&recording, lead_in.seconds, &wav).map_err(|err| err.to_string())?;
    }

    let title = path
        .file_stem()
        .map_or_else(|| "Recording".into(), |stem| stem.to_string_lossy());
    let project = arrange::follow(&analysis, lead_in, wav, &title);
    let keys = arrange::key_spans(&analysis, lead_in);
    let key = analysis.scale.map_or_else(
        || crate::i18n::tr("no clear key").to_string(),
        crate::i18n::key_name,
    );
    let chords = analysis.chords.iter().filter(|c| c.is_some()).count();
    let summary = crate::i18n::trf(
        "{}: {} BPM, {}/4, {}, chords on {} of {} beats",
        &[
            &title,
            &format!("{:.1}", analysis.bpm),
            &analysis.beats_per_bar,
            &key,
            &chords,
            &analysis.beats.len(),
        ],
    );
    Ok((project, summary, keys))
}

impl LineupApp {
    /// Asks for a recording, then starts listening to it.
    pub(super) fn import_recording(&mut self) {
        if self.import.is_some() {
            return;
        }
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(crate::i18n::tr("Audio"), &lineup_listen::AUDIO_EXTENSIONS)
            .pick_file()
        {
            self.import_path(path);
        }
    }

    /// Starts listening to the recording at `path`. Opening an audio file
    /// (`ferrous-lineup song.mp3`) comes here too.
    pub(super) fn import_path(&mut self, path: PathBuf) {
        if self.import.is_some() {
            return;
        }
        let name = path
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
        self.notice = Some(crate::i18n::trf("Listening to {}…", &[&name]));
        self.message = None;
        // The summary is written on the worker's thread, in the language
        // and with the note names chosen.
        let language = crate::i18n::Language::current();
        let naming = lineup_core::NoteNaming::current();
        self.import = Some(Import {
            worker: thread::spawn(move || {
                language.set();
                naming.set();
                listen(&path)
            }),
        });
    }

    /// Swaps the new project in when the import has finished, and plays it.
    pub(super) fn check_import(&mut self, ctx: &egui::Context) {
        let Some(import) = &self.import else {
            return;
        };
        if !import.worker.is_finished() {
            ctx.request_repaint_after(Duration::from_millis(100));
            return;
        }
        if let Some(import) = self.import.take() {
            match import.worker.join() {
                Ok(Ok((project, summary, keys))) => {
                    self.replace_project(project, None);
                    self.key_finder.new_project(&self.project, keys);
                    self.notice = Some(summary);
                    self.set_mode(PlayMode::Song);
                }
                Ok(Err(err)) => {
                    self.notice = None;
                    self.message = Some(crate::i18n::trf("Could not import: {}", &[&err]));
                }
                Err(_) => {
                    self.notice = None;
                    self.message = Some(crate::i18n::tr("Importing failed").to_string());
                }
            }
        }
    }
}

/// File > Import MIDI file. Reading a MIDI file takes a few thousandths
/// of a second, so unlike a recording it is done straight away.
impl LineupApp {
    /// Asks for a MIDI file, then imports it.
    pub(super) fn import_midi(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(crate::i18n::tr("MIDI file"), &midi_file::EXTENSIONS)
            .pick_file()
        {
            self.import_midi_path(&path);
        }
    }

    /// Makes a new project from the MIDI file at `path`, in song mode,
    /// ready to play. Opening a MIDI file (`ferrous-lineup song.mid`, or
    /// dropping it on the window) comes here too.
    pub(super) fn import_midi_path(&mut self, path: &Path) {
        let title = path
            .file_stem()
            .map_or_else(|| "MIDI file".into(), |stem| stem.to_string_lossy());
        let imported = std::fs::read(path)
            .map_err(|err| err.to_string())
            .and_then(|bytes| midi_file::read::read(&bytes).map_err(|err| err.to_string()))
            .and_then(|file| {
                midi_file::import::import(&file, &title).map_err(|err| err.to_string())
            });
        match imported {
            Ok(imported) => {
                let summary = imported.report.summary(&title, &imported.project);
                self.replace_project(imported.project, None);
                self.file.name_imported(&title);
                self.set_mode(PlayMode::Song);
                self.notice = Some(summary);
            }
            Err(err) => {
                self.message = Some(crate::i18n::trf(
                    "Could not import {}: {}",
                    &[&path.display(), &err],
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Audio;
    use super::*;

    /// The song's sound from `step` on, for `frames` frames, as song mode
    /// plays it: what the app sends the audio thread, then a jump to
    /// `step` and Play.
    fn played_from(app: &mut LineupApp, step: usize, frames: usize) -> Vec<f32> {
        use lineup_audio::{Command, PlayMode, Renderer};
        let rate = 44_100;
        let mut renderer = Renderer::new(rate, 1);
        for command in app.project_commands(rate, 1.0) {
            renderer.send(command);
        }
        let song = app.project.playback_arrangement();
        renderer.send(Command::SetSongTempos(app.project.played_tempos(&song)));
        renderer.send(Command::SetSong {
            plays: song.plays,
            length: song.length,
        });
        renderer.send(Command::SetMode(PlayMode::Song));
        renderer.send(Command::Seek(step));
        renderer.send(Command::Play);
        let mut out = vec![0.0; frames];
        renderer.render(&mut out);
        out
    }

    fn loudness(sound: &[f32]) -> f32 {
        (sound.iter().map(|s| s * s).sum::<f32>() / sound.len() as f32).sqrt()
    }

    #[test]
    fn an_imported_recording_plays_from_wherever_playback_starts() {
        // Twenty seconds of a C major chord, with a click on every beat at
        // 120 BPM.
        let rate = 44_100;
        let samples = (0..20 * rate)
            .map(|frame| {
                let t = frame as f32 / rate as f32;
                let click = if (t * 2.0).fract() < 0.01 { 0.5 } else { 0.0 };
                let chord: f32 = [261.63_f32, 329.63, 392.0]
                    .iter()
                    .map(|hz| (t * hz * std::f32::consts::TAU).sin() * 0.1)
                    .sum();
                click + chord
            })
            .collect();
        let dir = std::env::temp_dir().join(format!("lineup-resume-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("clicks.wav");
        let recording = lineup_listen::Recording {
            samples,
            sample_rate: rate,
        };
        arrange::write_wav(&recording, 0.0, &path).unwrap();
        let (mut project, ..) = listen(&path).unwrap();
        // Only the recording sounds: the harmony track is muted.
        project.channel_mut(1).unwrap().muted = true;

        let ctx = egui::Context::default();
        let mut app = LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()));
        app.replace_project(project, None);
        // Bar 5, step 64: a second of it, from the start and from there.
        let song = app.project.playback_arrangement();
        let tempos = app.project.played_tempos(&song);
        let before = lineup_core::tempo::seconds(64, |step| {
            tempos.get(step).copied().unwrap_or(app.project.tempo)
        });
        let at = (before * f64::from(rate)) as usize;
        let whole = played_from(&mut app, 0, at + rate as usize);
        let from_bar_5 = played_from(&mut app, 64, rate as usize);
        let heard = loudness(&whole[at..]);
        assert!(heard > 0.05, "{heard}");
        let resumed = loudness(&from_bar_5);
        assert!(
            (resumed - heard).abs() < heard * 0.2,
            "{resumed} against {heard}"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn opening_another_project_stops_the_import_that_was_listening() {
        let ctx = egui::Context::default();
        let mut app = LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()));
        app.import = Some(Import {
            worker: thread::spawn(|| Ok((Project::empty(), String::new(), Vec::new()))),
        });

        let opened = Project::template();
        app.replace_project(opened.clone(), None);

        assert!(app.import.is_none(), "the import was let go");
        assert_eq!(app.project, opened, "the opened project stays");
        assert_eq!(
            app.notice.as_deref(),
            Some("Stopped importing the recording.")
        );
    }
}
