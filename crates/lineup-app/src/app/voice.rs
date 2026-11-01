//! Takes: singing or whistling along with the song, into notes.
//!
//! The microphone button next to Rec records a *take* while the song (or,
//! in pattern mode, the pattern) plays: the voice, and where in the song
//! each moment of it was sung (see `lineup_audio::input`). While it
//! records, the transport shows how loud the voice is and the note it
//! hears, with how far off it is.
//!
//! When the take stops, a thread of its own hears the melody, bass line,
//! drums and chords (see `lineup_listen`), and the "Your take" window
//! asks which to write in. Melody notes can go into the channel that
//! plays the tune (the Lead), another channel, a new one, or nowhere;
//! bass notes go to a Bass channel, and drums and chords to their own
//! places. The recording itself is kept on a track of the playlist,
//! where it plays with the song as a reference. Writing it in is one step
//! of undo (see `lineup_core::take`).
//!
//! A recording made before, in an audio file, becomes a take the same
//! way, laid on the playlist (see [`Take::laid`]): dropped on the
//! playlist, from the bar and on the track it was dropped on; from the
//! playlist's "Add a recording" button, or a track's menu, from the
//! playhead; dropped anywhere else on the window, from the playhead.
//! Opening one starts a new song with it.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use eframe::egui::{self, RichText, vec2};
use lineup_audio::PlayMode;
use lineup_audio::input::{Microphone, Recorder, Recording};
use lineup_core::general_midi::FLUTE;
use lineup_core::take::{
    Along, BassNote, DrumHit, HeardChord, Keep, Pitches, SungNote, Take, Target, Transcription,
    transcribe_bass, transcribe_chords, transcribe_drums, transcribe_with, tune_channel,
};
use lineup_core::{MAX_TRACKS, Meter, Project};
use lineup_listen::voice;

use super::{APP_NAME, Audio, LineupApp, preferences};
use crate::i18n::{tr, tr_name, trf};
use crate::{icons, theme};

/// The preference that moves takes later in the song (earlier if
/// negative), in milliseconds, for a sound card whose delays are not what
/// it says.
const NUDGE: &str = "take timing";
const MELODY: &str = "take melody";
const DRUMS: &str = "take drums";
const BASS: &str = "take bass";
const CHORDS: &str = "take chords";

/// The furthest a take can be moved, in milliseconds.
const MOST_NUDGE: f64 = 500.0;

/// The grids the notes of a take can snap to, in steps: sixteenths,
/// eighths and quarter notes.
const GRIDS: [(usize, &str); 3] = [
    (1, "Sixteenth notes"),
    (2, "Eighth notes"),
    (4, "Quarter notes"),
];

/// The ways the pitches of a take can be written, with what each does.
const PITCHES: [(Pitches, &str, &str); 3] = [
    (
        Pitches::AsSung,
        "As sung",
        "Each note on the nearest semitone to what was sung.",
    ),
    (
        Pitches::InKey,
        "In the key",
        "Each note on the nearest note of the key there, so an out of tune note does not \
         come out as a wrong one.",
    ),
    (
        Pitches::WithChords,
        "Key and chords",
        "Each note on the nearest note of the scale the soloist plays over the chord \
         there, in the key: over D7 in C major, F♯ instead of F. Bars with no chord keep to \
         the key.",
    ),
];

/// The fewest notes of a take the review shows by name.
const NAMED_NOTES: usize = 12;

/// The take being recorded, heard, or waiting for where its notes go.
#[derive(Default)]
pub(super) struct Voice {
    recording: Option<Taking>,
    hearing: Option<JoinHandle<Heard>>,
    loading: Option<Loading>,
    review: Option<Review>,
    /// The pitch heard last while recording, as the bits of an `f32`
    /// MIDI number; not a number when none.
    live_pitch: Arc<AtomicU32>,
    /// Where takes are saved instead of the usual place, for the tests.
    #[cfg(test)]
    folder: Option<PathBuf>,
}

/// A take being recorded.
struct Taking {
    recorder: Recorder,
    along: Along,
}

/// A take with its notes heard.
struct Heard {
    recording: Recording,
    along: Along,
    notes: Vec<SungNote>,
    tuning: f32,
    drums: Vec<DrumHit>,
    bass: Vec<BassNote>,
    chords: Vec<HeardChord>,
    /// The name of a recording added from a file: the file's, without
    /// its extension. `None` for a take recorded here.
    name: Option<String>,
}

/// What reading an audio file gives: its sound, the notes heard in it
/// and how sharp they were sung, or why it could not be read.
type Loaded = Result<(lineup_listen::Recording, Analysis), String>;

/// All the layers heard in one recording, ready to choose in the review.
struct Analysis {
    notes: Vec<SungNote>,
    tuning: f32,
    drums: Vec<DrumHit>,
    bass: Vec<BassNote>,
    chords: Vec<HeardChord>,
}

/// A recording added from a file, being read and heard, to lay on the
/// playlist from step `step` on.
struct Loading {
    worker: JoinHandle<Loaded>,
    step: usize,
    name: String,
    /// The track the recording goes on, when it was dropped on one or
    /// added from its menu: else the review offers a free one.
    track: Option<usize>,
    /// A take's recording already in the song, heard again.
    again: Option<Again>,
}

/// A take's recording already in the song, transcribed again: its
/// track and the take's name ("Take 1").
#[derive(Debug, Clone)]
struct Again {
    track: usize,
    name: String,
}

/// Where a take's notes go, as the review window offers it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Destination {
    Channel(usize),
    New,
    Nowhere,
}

/// The "Your take" window.
struct Review {
    heard: Heard,
    take: Take,
    destination: Destination,
    grid: usize,
    pitches: Pitches,
    melody: bool,
    drums: bool,
    bass: bool,
    chords: bool,
    keep: bool,
    track: usize,
    transcription: Transcription,
    /// For a take transcribed again, the recording it is: kept already,
    /// so it is not saved or added again.
    again: Option<Again>,
}

impl Review {
    fn new(project: &Project, heard: Heard, track: Option<usize>, ctx: &egui::Context) -> Self {
        let take = Take {
            sample_rate: heard.recording.sample_rate,
            frames: heard.recording.samples.len(),
            marks: heard.recording.marks.clone(),
            along: heard.along.clone(),
        };
        let grid = 1;
        let pitches = Pitches::AsSung;
        let melody = preferences::get(ctx, MELODY).unwrap_or(true);
        let drums = preferences::get(ctx, DRUMS).unwrap_or(false);
        let bass = preferences::get(ctx, BASS).unwrap_or(false);
        let chords = preferences::get(ctx, CHORDS).unwrap_or(false);
        let mut transcription =
            transcribe_with(project, &take, &heard.notes, grid, heard.tuning, pitches);
        transcription.drums = transcribe_drums(&take, &heard.drums, grid);
        transcription.bass = transcribe_bass(&take, &heard.bass, grid);
        transcription.chords = transcribe_chords(project, &take, &heard.chords);
        if !melody {
            transcription.notes.clear();
        }
        if !drums {
            transcription.drums.clear();
        }
        if !bass {
            transcription.bass.clear();
        }
        if !chords {
            transcription.chords = None;
        }
        let destination = tune_channel(project).map_or(Destination::New, Destination::Channel);
        let tracks = project.playlist().tracks().len();
        let track = track
            .filter(|&track| track < tracks)
            .unwrap_or_else(|| free_track(project, transcription.recorded().unwrap_or(0..0)));
        Review {
            heard,
            take,
            destination,
            grid,
            pitches,
            melody,
            drums,
            bass,
            chords,
            keep: true,
            track,
            transcription,
            again: None,
        }
    }

    /// The review of a take's recording already in the song, `again`:
    /// nothing to keep, and the notes go where they went before, the
    /// channel whose notes are on the take's notes track if it has one.
    fn again(project: &Project, heard: Heard, again: Again, ctx: &egui::Context) -> Self {
        let mut review = Review::new(project, heard, Some(again.track), ctx);
        let notes = format!("{} notes", again.name);
        let below = again.track + 1;
        let written = project
            .playlist()
            .tracks()
            .get(below)
            .filter(|track| track.name == notes)
            .and_then(|_| {
                project
                    .playlist()
                    .clips()
                    .iter()
                    .filter(|clip| clip.track == below)
                    .filter_map(|clip| project.pattern(clip.pattern))
                    .find_map(|pattern| {
                        (0..project.channels().len()).find(|&channel| {
                            !project.channels()[channel].is_take()
                                && !pattern.notes(channel).is_empty()
                        })
                    })
            });
        if let Some(channel) = written {
            review.destination = Destination::Channel(channel);
        }
        review.keep = false;
        review.again = Some(again);
        review
    }

    /// Writes the notes again on `grid`, with their pitches as `pitches`
    /// says, if either changed.
    fn snap(&mut self, project: &Project, grid: usize, pitches: Pitches) {
        if grid != self.grid || pitches != self.pitches {
            self.grid = grid;
            self.pitches = pitches;
            self.transcription = self.transcribe(project);
        }
    }

    /// The notes of the take as the song is now.
    fn transcribe(&self, project: &Project) -> Transcription {
        let mut transcription = transcribe_with(
            project,
            &self.take,
            &self.heard.notes,
            self.grid,
            self.heard.tuning,
            self.pitches,
        );
        if self.melody {
            transcription.drums = transcribe_drums(&self.take, &self.heard.drums, self.grid);
            transcription.bass = transcribe_bass(&self.take, &self.heard.bass, self.grid);
            transcription.chords = transcribe_chords(project, &self.take, &self.heard.chords);
        } else {
            transcription.notes.clear();
            if self.drums {
                transcription.drums = transcribe_drums(&self.take, &self.heard.drums, self.grid);
            }
            if self.bass {
                transcription.bass = transcribe_bass(&self.take, &self.heard.bass, self.grid);
            }
            if self.chords {
                transcription.chords = transcribe_chords(project, &self.take, &self.heard.chords);
            }
        }
        if !self.drums {
            transcription.drums.clear();
        }
        if !self.bass {
            transcription.bass.clear();
        }
        if !self.chords {
            transcription.chords = None;
        }
        transcription
    }
}

/// The track a take goes on: the first one below every track with clips,
/// or a new one at the bottom (the number of tracks, see [`Keep::track`]).
/// With no room for a new track, the first with no clip where the take
/// was sung, else the last.
fn free_track(project: &Project, sung: std::ops::Range<usize>) -> usize {
    let playlist = project.playlist();
    let tracks = playlist.tracks().len();
    let below = playlist
        .clips()
        .iter()
        .map(|clip| clip.track + 1)
        .max()
        .unwrap_or(0);
    if below < tracks || tracks < MAX_TRACKS {
        return below.min(tracks);
    }
    (0..tracks)
        .find(|&track| {
            !playlist.clips().iter().any(|clip| {
                clip.track == track
                    && clip.start < sung.end
                    && clip.start + project.clip_length(clip) > sung.start
            })
        })
        .unwrap_or(tracks.saturating_sub(1))
}

/// A name for a take that no channel, pattern or track has: "Take 1"
/// (the first number free) for a take recorded here, or the name of the
/// file it came from ("melody", else "melody 2" and so on).
fn take_name(project: &Project, file: Option<&str>) -> String {
    let used = |name: &str| {
        let piece = format!("{name} (");
        let taken = |other: &str| other == name || other.starts_with(&piece);
        project
            .channels()
            .iter()
            .any(|channel| taken(&channel.name))
            || project
                .patterns()
                .iter()
                .any(|pattern| taken(&pattern.name))
            || project
                .playlist()
                .tracks()
                .iter()
                .any(|track| taken(&track.name))
    };
    let (base, first) = match file {
        Some(name) if !used(name) => return name.to_string(),
        Some(name) => (name, 2),
        None => ("Take", 1),
    };
    let number = (first..)
        .find(|number| !used(&format!("{base} {number}")))
        .unwrap_or(first);
    format!("{base} {number}")
}

/// A file name for `name` in `folder` that no file has yet: "name.wav",
/// else "name (2).wav" and so on.
fn free_file(folder: &Path, name: &str) -> PathBuf {
    let name = name.replace(['/', '\\', ':'], "-");
    let mut path = folder.join(format!("{name}.wav"));
    let mut number = 2;
    while path.exists() {
        path = folder.join(format!("{name} ({number}).wav"));
        number += 1;
    }
    path
}

/// Hears the notes of a take: run on a thread of its own.
fn hear(recording: Recording, along: Along) -> Heard {
    let analysis = analyze(&recording.samples, recording.sample_rate);
    Heard {
        recording,
        along,
        notes: analysis.notes,
        tuning: analysis.tuning,
        drums: analysis.drums,
        bass: analysis.bass,
        chords: analysis.chords,
        name: None,
    }
}

/// Hears all four layers of a recording on its worker thread.
fn analyze(samples: &[f32], sample_rate: u32) -> Analysis {
    let (notes, tuning) = notes_in(samples, sample_rate);
    Analysis {
        notes,
        tuning,
        drums: lineup_listen::drums::hits(samples, sample_rate),
        bass: lineup_listen::bass::notes(samples, sample_rate),
        chords: lineup_listen::chords::heard(samples, sample_rate),
    }
}

/// The notes sung in `samples`, and how sharp they were sung.
fn notes_in(samples: &[f32], sample_rate: u32) -> (Vec<SungNote>, f32) {
    let track = voice::pitch_track(samples, sample_rate);
    let notes = voice::notes(&track);
    let tuning = voice::tuning(&notes);
    (notes, tuning)
}

/// Reads the audio file at `path` and hears its notes: run on a thread
/// of its own.
fn load(path: &Path) -> Loaded {
    let recording = lineup_listen::decode_file(path).map_err(|err| err.to_string())?;
    let analysis = analyze(&recording.samples, recording.sample_rate);
    Ok((recording, analysis))
}

/// The note `pitch` (a MIDI number with its fraction) as a name and how
/// far off it is, as "A4 +12¢".
fn pitch_label(pitch: f32) -> String {
    let nearest = pitch.round().clamp(0.0, 127.0);
    let cents = ((pitch - nearest) * 100.0).round() as i32;
    let note = lineup_core::Note::new(nearest as u8);
    if cents == 0 {
        note.to_string()
    } else {
        format!("{note} {cents:+}¢")
    }
}

impl LineupApp {
    /// Whether a take is being recorded.
    pub(super) fn recording_voice(&self) -> bool {
        self.voice.recording.is_some()
    }

    /// The microphone button, next to Rec, and while it records, how loud
    /// the voice is and the note heard.
    pub(super) fn voice_button(&mut self, ui: &mut egui::Ui) {
        let on = self.recording_voice();
        let button = icons::toggle_lit(ui, tr("Record a take"), on, theme::RED, icons::microphone);
        let hover = tr(
            "Record a take: sing or whistle along with the song, and your notes are written \
             in. Use headphones, so the microphone hears only you.",
        );
        let button = theme::hint(button, hover);
        if button.clicked() {
            if on {
                self.stop_voice();
            } else {
                self.start_voice(ui.ctx());
            }
        }
        if let Some(recording) = &self.voice.recording {
            let level = recording.recorder.level();
            let seconds = recording.recorder.seconds();
            let pitch = f32::from_bits(self.voice.live_pitch.load(Ordering::Relaxed));
            live_display(ui.ctx(), button.rect, level, seconds, pitch);
        }
    }

    /// Starts recording a take, and plays if not playing.
    fn start_voice(&mut self, ctx: &egui::Context) {
        if self.voice.recording.is_some() || self.voice.hearing.is_some() {
            return;
        }
        if self.voice.loading.is_some() {
            self.message = Some(tr("Wait until the recording added is heard.").to_string());
            return;
        }
        let Audio::Running(engine) = &mut self.audio else {
            self.message =
                Some(tr("A take needs the sound card: audio is not running.").to_string());
            return;
        };
        let microphone = match Microphone::open() {
            Ok(microphone) => microphone,
            Err(err) => {
                self.message = Some(trf("Could not record a take: {}", &[&err]));
                return;
            }
        };
        let Some(clock) = engine.take_song_clock() else {
            return;
        };
        let along = match self.mode {
            PlayMode::Song => Along::Song(self.project.playback_arrangement()),
            PlayMode::Pattern => Along::Pattern {
                pattern: self.pattern,
                length: self.loop_length(),
            },
        };
        let nudge = preferences::get::<f64>(ctx, NUDGE).unwrap_or(0.0) / 1000.0;
        let live = Arc::clone(&self.voice.live_pitch);
        live.store(f32::NAN.to_bits(), Ordering::Relaxed);
        let view: lineup_audio::input::LiveView = Box::new(move |samples, rate| {
            let pitch = voice::pitch_now(samples, rate).unwrap_or(f32::NAN);
            live.store(pitch.to_bits(), Ordering::Relaxed);
        });
        self.voice.review = None;
        self.voice.recording = Some(Taking {
            recorder: Recorder::start(microphone, clock, nudge, view),
            along,
        });
        self.message = None;
        self.notice = None;
        if !self.playing {
            self.set_playing(true);
        }
    }

    /// Stops recording a take, if one is recording, and starts hearing its
    /// notes. Playback goes on.
    pub(super) fn stop_voice(&mut self) {
        let Some(recording) = self.voice.recording.take() else {
            return;
        };
        let along = recording.along;
        let Some((take, clock)) = recording.recorder.finish() else {
            self.message = Some(tr("Recording the take failed.").to_string());
            return;
        };
        if let Audio::Running(engine) = &mut self.audio {
            engine.give_back_song_clock(clock);
        }
        if take.marks.is_empty() {
            self.message = Some(
                tr("The take was not recorded while the song played: nothing to write in.")
                    .to_string(),
            );
            return;
        }
        self.notice = Some(tr("Listening to your take…").to_string());
        self.voice.hearing = Some(thread::spawn(move || hear(take, along)));
    }

    /// Forgets any take: recording, being heard or waiting. For a project
    /// being replaced, whose song the take does not belong to. Returns
    /// whether a recording added from a file was still being heard.
    pub(super) fn forget_voice(&mut self) -> bool {
        if let Some(recording) = self.voice.recording.take()
            && let Some((_, clock)) = recording.recorder.finish()
            && let Audio::Running(engine) = &mut self.audio
        {
            engine.give_back_song_clock(clock);
        }
        self.voice.hearing = None;
        self.voice.review = None;
        self.voice.loading.take().is_some()
    }

    /// The playlist's "Add a recording" (its button, or a track's menu):
    /// asks for an audio file, and adds it to the song as a take, from the
    /// playhead, on `track` if given.
    pub(super) fn add_recording(&mut self, track: Option<usize>) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(tr("Audio"), &lineup_listen::AUDIO_EXTENSIONS)
            .pick_file()
        {
            self.add_recording_path(path, None, track);
        }
    }

    /// Adds the recording at `path` to the song as a take, from step `at`
    /// of the playlist where it was dropped, else from the playhead (from
    /// the start in pattern mode, which switches to song mode), on `track`
    /// if given: reads it and hears its notes in the background, then
    /// asks where they go, as for a take recorded here.
    pub(super) fn add_recording_path(
        &mut self,
        path: PathBuf,
        at: Option<usize>,
        track: Option<usize>,
    ) {
        if self.take_busy() {
            return;
        }
        let step = match (self.mode, at) {
            (PlayMode::Song, None) => {
                let here = self.playhead().unwrap_or(self.cursor);
                self.project
                    .playback_arrangement()
                    .timeline_step(here)
                    .unwrap_or(0)
            }
            (PlayMode::Pattern, None) => {
                self.set_mode(PlayMode::Song);
                0
            }
            (mode, Some(step)) => {
                if mode == PlayMode::Pattern {
                    self.set_mode(PlayMode::Song);
                }
                step
            }
        };
        let name = path.file_stem().map_or_else(
            || "Recording".to_string(),
            |stem| stem.to_string_lossy().into_owned(),
        );
        let file = path
            .file_name()
            .map_or_else(String::new, |file| file.to_string_lossy().into_owned());
        self.notice = Some(trf("Listening to {}…", &[&file]));
        self.message = None;
        // Errors are written on the worker's thread, in the language
        // chosen.
        let language = crate::i18n::Language::current();
        self.voice.loading = Some(Loading {
            worker: thread::spawn(move || {
                language.set();
                load(&path)
            }),
            step,
            name,
            track,
            again: None,
        });
    }

    /// Whether a take is being recorded, heard or reviewed, so another
    /// cannot start: says so if it is.
    fn take_busy(&mut self) -> bool {
        let busy = self.voice.recording.is_some()
            || self.voice.hearing.is_some()
            || self.voice.loading.is_some()
            || self.voice.review.is_some();
        if busy {
            self.message = Some(
                tr("Finish the take first: write it in or throw it away, then add the recording.")
                    .to_string(),
            );
        }
        busy
    }

    /// Hears the recording that `channel` plays from step `from` of
    /// track `track` again, and opens "Your take" for it, to write its
    /// notes with other choices in place of the ones written before.
    pub(super) fn transcribe_again(&mut self, channel: usize, from: usize, track: usize) {
        let Some(take) = self.project.channels().get(channel) else {
            return;
        };
        let lineup_core::ChannelKind::Sampler(lineup_core::SampleSource::Take(path)) = &take.kind
        else {
            return;
        };
        let path = path.clone();
        let name = take.name.clone();
        if self.take_busy() {
            return;
        }
        if self.mode == PlayMode::Pattern {
            self.set_mode(PlayMode::Song);
        }
        self.notice = Some(trf("Listening to {}…", &[&tr_name(&name)]));
        self.message = None;
        let language = crate::i18n::Language::current();
        self.voice.loading = Some(Loading {
            worker: thread::spawn(move || {
                language.set();
                load(&path)
            }),
            step: from,
            name: name.clone(),
            track: Some(track),
            again: Some(Again { track, name }),
        });
    }

    /// Opens the recording at `path` as a new song: an empty one, with the
    /// recording from its start. Opening an audio file comes here
    /// (`ferrous-lineup song.mp3`, or File > Open).
    pub(super) fn open_recording(&mut self, path: PathBuf) {
        let title = path.file_stem().map_or_else(
            || "Recording".to_string(),
            |stem| stem.to_string_lossy().into_owned(),
        );
        self.replace_project(Project::empty(), None);
        self.file.name_imported(&title);
        self.set_mode(PlayMode::Song);
        self.add_recording_path(path, None, None);
    }

    /// Once a frame: stops a take that stopped by itself (the longest
    /// take, or a microphone gone), and opens the review when its notes
    /// are heard.
    pub(super) fn check_voice(&mut self, ctx: &egui::Context) {
        if let Some(recording) = &self.voice.recording {
            if recording.recorder.has_stopped() {
                self.stop_voice();
            } else {
                ctx.request_repaint_after(Duration::from_millis(50));
            }
        }
        self.check_loading(ctx);
        let Some(hearing) = &self.voice.hearing else {
            return;
        };
        if !hearing.is_finished() {
            ctx.request_repaint_after(Duration::from_millis(100));
            return;
        }
        if let Some(hearing) = self.voice.hearing.take() {
            self.notice = None;
            match hearing.join() {
                Ok(heard) => {
                    self.voice.review = Some(Review::new(&self.project, heard, None, ctx));
                }
                Err(_) => self.message = Some(tr("Listening to the take failed.").to_string()),
            }
        }
    }

    /// Lays a recording added from a file on the playlist when its notes
    /// are heard, and opens the review.
    fn check_loading(&mut self, ctx: &egui::Context) {
        let Some(loading) = &self.voice.loading else {
            return;
        };
        if !loading.worker.is_finished() {
            ctx.request_repaint_after(Duration::from_millis(100));
            return;
        }
        let Some(loading) = self.voice.loading.take() else {
            return;
        };
        self.notice = None;
        match loading.worker.join() {
            Ok(Ok((recording, analysis))) => {
                let take = Take::laid(
                    &self.project,
                    loading.step,
                    recording.sample_rate,
                    recording.samples.len(),
                );
                let heard = Heard {
                    recording: Recording {
                        samples: recording.samples,
                        sample_rate: recording.sample_rate,
                        marks: take.marks,
                        lost: 0,
                    },
                    along: take.along,
                    notes: analysis.notes,
                    tuning: analysis.tuning,
                    drums: analysis.drums,
                    bass: analysis.bass,
                    chords: analysis.chords,
                    name: Some(loading.name),
                };
                self.voice.review = Some(match loading.again {
                    Some(again) => Review::again(&self.project, heard, again, ctx),
                    None => Review::new(&self.project, heard, loading.track, ctx),
                });
            }
            Ok(Err(err)) => {
                self.message = Some(trf("Could not add the recording: {}", &[&err]));
            }
            Err(_) => self.message = Some(tr("Listening to the recording failed.").to_string()),
        }
    }

    /// The "Your take" window: where the notes of a take go.
    pub(super) fn take_window(&mut self, ctx: &egui::Context) {
        let Some(review) = &mut self.voice.review else {
            return;
        };
        let project = &self.project;
        let mut open = true;
        let mut write = false;
        let mut discard = false;
        egui::Window::new(tr("Your take"))
            .id(egui::Id::unique("take-review"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(420.0);
                review_summary(ui, project, review);
                ui.separator();
                egui::Grid::new("take-review-choices")
                    .num_columns(2)
                    .spacing(vec2(12.0, 8.0))
                    .show(ui, |ui| {
                        ui.label(tr("Extract"));
                        ui.horizontal(|ui| {
                            let mut changed = false;
                            changed |= ui.checkbox(&mut review.melody, tr("Melody")).changed();
                            changed |= ui.checkbox(&mut review.drums, tr("Drums")).changed();
                            changed |= ui.checkbox(&mut review.bass, tr("Bass line")).changed();
                            ui.add_enabled_ui(matches!(review.take.along, Along::Song(_)), |ui| {
                                changed |=
                                    ui.checkbox(&mut review.chords, tr("Chords")).changed();
                            });
                            if changed {
                                preferences::set(ui.ctx(), MELODY, &review.melody);
                                preferences::set(ui.ctx(), DRUMS, &review.drums);
                                preferences::set(ui.ctx(), BASS, &review.bass);
                                preferences::set(ui.ctx(), CHORDS, &review.chords);
                                review.transcription = review.transcribe(project);
                            }
                        });
                        ui.end_row();

                        ui.label(tr("Notes go to"));
                        ui.add_enabled_ui(review.melody, |ui| {
                            destination_combo(ui, project, &mut review.destination);
                        });
                        ui.end_row();

                        ui.label(tr("On the grid of"));
                        ui.add_enabled_ui(review.melody, |ui| {
                            ui.horizontal(|ui| {
                                let mut grid = review.grid;
                                for (steps, name) in GRIDS {
                                    ui.selectable_value(&mut grid, steps, tr(name));
                                }
                                review.snap(project, grid, review.pitches);
                            });
                        });
                        ui.end_row();

                        ui.label(tr("Pitches"));
                        ui.add_enabled_ui(review.melody, |ui| {
                            ui.horizontal(|ui| {
                                let mut pitches = review.pitches;
                                for (way, name, hover) in PITCHES {
                                    ui.selectable_value(&mut pitches, way, tr(name))
                                        .on_hover_text(tr(hover));
                                }
                                review.snap(project, review.grid, pitches);
                            });
                        });
                        ui.end_row();

                        ui.label(tr("The recording"));
                        match &review.again {
                            Some(again) => {
                                let track = project
                                    .playlist()
                                    .tracks()
                                    .get(again.track)
                                    .map_or_else(String::new, |track| {
                                        tr_name(&track.name).into_owned()
                                    });
                                ui.label(trf("It stays on {}, as it is.", &[&track]));
                            }
                            None => {
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut review.keep, tr("Keep it"));
                                    if review.keep && matches!(review.take.along, Along::Song(_)) {
                                        track_combo(ui, project, &mut review.track);
                                    }
                                });
                            }
                        }
                        ui.end_row();
                    });
                if let Destination::Channel(channel) = review.destination
                    && let Some(channel) = project.channels().get(channel)
                {
                    ui.label(
                        RichText::new(trf(
                            "The notes take the place of what {} played there. Undo brings it back.",
                            &[&tr_name(&channel.name)],
                        ))
                        .color(theme::TEXT_DIM),
                    );
                }
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let nothing = !review.keep
                        && (!review.melody || review.destination == Destination::Nowhere)
                        && !review.drums
                        && !review.bass
                        && !review.chords;
                    if ui
                        .add_enabled(!nothing, egui::Button::new(tr("Write it in")))
                        .clicked()
                    {
                        write = true;
                    }
                    if ui.button(tr("Throw it away")).clicked() {
                        discard = true;
                    }
                });
            });
        if write {
            self.write_take();
        } else if discard || !open {
            self.voice.review = None;
        }
    }

    /// Writes the take under review into the song: its recording saved
    /// next to the project, and its notes where the review says.
    fn write_take(&mut self) {
        let Some(review) = self.voice.review.take() else {
            return;
        };
        // Again, in case the song changed while the window was open.
        let transcription = review.transcribe(&self.project);
        let name = match &review.again {
            Some(again) => again.name.clone(),
            None => take_name(&self.project, review.heard.name.as_deref()),
        };
        let mut recordings = Vec::new();
        if review.keep {
            let folder = self.takes_folder();
            let rate = review.heard.recording.sample_rate;
            for (index, piece) in transcription.pieces.iter().enumerate() {
                let song = self.file.stem();
                let base = if song == name {
                    name.clone()
                } else {
                    format!("{song} - {name}")
                };
                let file = if index == 0 {
                    base
                } else {
                    format!("{base} ({})", index + 1)
                };
                let path = free_file(&folder, &file);
                let samples = &review.heard.recording.samples[piece.frames.clone()];
                if let Err(err) = lineup_listen::wav::write(samples, rate, &path) {
                    self.message = Some(trf("Could not save the take: {}", &[&err]));
                    return;
                }
                if let Err(err) = self.samples.reload(&path) {
                    self.message = Some(format!("{}: {err}", path.display()));
                    return;
                }
                recordings.push(path);
            }
        }
        let target = match (review.melody, review.destination) {
            (true, Destination::Channel(channel)) => Target::Channel(channel),
            (true, Destination::New) => Target::New {
                name: "Sung notes".to_string(),
                program: FLUTE,
            },
            _ => Target::Nowhere,
        };
        let keep = Keep {
            notes: format!("{name} notes"),
            name,
            track: review.track,
            recordings,
            under: review.again.as_ref().map(|again| again.track),
        };
        let along = review.take.along.clone();
        let undo = if review.again.is_some() {
            "transcribe a take again"
        } else {
            "record a take"
        };
        let added = self.edit(undo, |project| {
            project.add_take(&along, &transcription, &target, &keep)
        });
        let mut said = match added.channel {
            Some(channel) => {
                let name = tr_name(&self.project.channels()[channel].name);
                if added.notes == 1 {
                    trf("Wrote 1 note on {}.", &[&name])
                } else {
                    trf("Wrote {} notes on {}.", &[&added.notes, &name])
                }
            }
            None => String::new(),
        };
        if added.drum_hits > 0 {
            if !said.is_empty() {
                said.push(' ');
            }
            said.push_str(&trf("Wrote {} drum hits.", &[&added.drum_hits]));
        }
        if added.bass_notes > 0 {
            if !said.is_empty() {
                said.push(' ');
            }
            said.push_str(&trf("Wrote {} bass notes.", &[&added.bass_notes]));
        }
        if added.chords > 0 {
            if !said.is_empty() {
                said.push(' ');
            }
            said.push_str(&trf(
                "Wrote {} chords on the chord sheet.",
                &[&added.chords],
            ));
        }
        if !added.recordings.is_empty() {
            if !said.is_empty() {
                said.push(' ');
            }
            said.push_str(&trf("The recording plays as {}.", &[&tr_name(&keep.name)]));
        }
        for (pattern, times) in &added.shared {
            said.push(' ');
            said.push_str(&trf(
                "{} plays {} times: your notes play each time.",
                &[&tr_name(&self.project.patterns()[*pattern].name), times],
            ));
        }
        if let Some(channel) = added.channel {
            self.select_channel(Some(channel));
        }
        self.notice = Some(said);
    }

    /// Where takes are saved: next to the project's file, in a folder
    /// named after it, or, for a project not saved yet, in the folder the
    /// app keeps its settings in. The system's folder for temporary files
    /// if neither can be made.
    fn takes_folder(&self) -> PathBuf {
        #[cfg(test)]
        if let Some(folder) = &self.voice.folder {
            return folder.clone();
        }
        let wanted = match self.file.path() {
            Some(path) => path.with_file_name(format!("{} takes", self.file.stem())),
            None => eframe::storage_dir(APP_NAME)
                .unwrap_or_else(std::env::temp_dir)
                .join("takes"),
        };
        if std::fs::create_dir_all(&wanted).is_ok() {
            wanted
        } else {
            std::env::temp_dir()
        }
    }

    /// Settings > Audio: the microphone, for takes.
    pub(super) fn microphone_settings_ui(&mut self, ui: &mut egui::Ui) {
        super::settings::card(
            ui,
            "Microphone",
            "Records takes, from your system’s default input device. If your notes land \
             late or early on the grid, move them here.",
            |ui| {
                let ctx = ui.ctx().clone();
                let mut nudge = preferences::get::<f64>(&ctx, NUDGE).unwrap_or(0.0);
                ui.horizontal(|ui| {
                    ui.label(tr("Move takes later by"));
                    let changed = ui
                        .add(
                            egui::DragValue::new(&mut nudge)
                                .range(-MOST_NUDGE..=MOST_NUDGE)
                                .speed(1.0)
                                .suffix(" ms"),
                        )
                        .changed();
                    if changed {
                        preferences::set(&ctx, NUDGE, &nudge.round());
                    }
                });
            },
        );
    }
}

/// While a take records: under the microphone button, how loud the
/// microphone hears you, how long the take is, and the note it hears with
/// how far off it is. It floats over the window, so the toolbar keeps its
/// place.
fn live_display(ctx: &egui::Context, button: egui::Rect, level: f32, seconds: f64, pitch: f32) {
    egui::Area::new(egui::Id::unique("take-live"))
        .order(egui::Order::Foreground)
        .fixed_pos(button.left_bottom() + vec2(-8.0, 8.0))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(8.0, 30.0), egui::Sense::hover());
                    let painter = ui.painter();
                    painter.rect_filled(rect, 2.0, theme::BUTTON);
                    let level = level.clamp(0.0, 1.0);
                    let mut lit = rect;
                    // The square root, so a quiet voice still shows.
                    lit.set_top(rect.bottom() - rect.height() * level.sqrt());
                    let color = if level > 0.95 {
                        theme::RED
                    } else {
                        theme::GREEN
                    };
                    painter.rect_filled(lit, 2.0, color);
                    let whole = seconds.max(0.0) as u64;
                    ui.vertical(|ui| {
                        let heard = if pitch.is_finite() {
                            pitch_label(pitch)
                        } else {
                            "–".to_string()
                        };
                        ui.add(
                            egui::Label::new(RichText::new(heard).monospace().size(18.0).strong())
                                .wrap_mode(egui::TextWrapMode::Extend),
                        );
                        ui.label(
                            RichText::new(trf(
                                "Recording {}",
                                &[&format!("{}:{:02}", whole / 60, whole % 60)],
                            ))
                            .small()
                            .color(theme::TEXT_DIM),
                        );
                    });
                });
            });
        });
}

/// What the take sang: how many notes, where, and the first ones by name.
fn review_summary(ui: &mut egui::Ui, project: &Project, review: &Review) {
    let transcription = &review.transcription;
    let notes = &transcription.notes;
    let text = match (transcription.range(), transcription.recorded()) {
        (Some(range), _) if !notes.is_empty() => {
            let meter = match review.take.along {
                Along::Song(_) => project.meter(),
                Along::Pattern { pattern, .. } => Meter::steady(project.pattern_time(pattern)),
            };
            let first = meter.bar_at(range.start).number + 1;
            let last = meter.bar_at(range.end.saturating_sub(1)).number + 1;
            match (notes.len(), first == last) {
                (1, _) => trf("1 note, in bar {}.", &[&first]),
                (count, true) => trf("{} notes, in bar {}.", &[&count, &first]),
                (count, false) => trf("{} notes, from bar {} to bar {}.", &[&count, &first, &last]),
            }
        }
        (_, Some(_)) if !review.melody => tr("Melody is not selected.").to_string(),
        (_, Some(_)) => tr("No melody notes were heard.").to_string(),
        _ => tr("The take was not sung while the song played.").to_string(),
    };
    ui.label(text);
    if !notes.is_empty() {
        let names: Vec<String> = notes
            .iter()
            .take(NAMED_NOTES)
            .map(|note| project.key_at(note.start).note_name(note.pitch))
            .collect();
        let more = if notes.len() > NAMED_NOTES {
            " …"
        } else {
            ""
        };
        ui.label(RichText::new(format!("{}{more}", names.join(" "))).monospace());
    }
    if review.drums {
        ui.label(trf("{} drum hits.", &[&transcription.drums.len()]));
    }
    if review.bass {
        ui.label(trf("{} bass notes.", &[&transcription.bass.len()]));
        if !transcription.bass.is_empty() {
            let names: Vec<String> = transcription
                .bass
                .iter()
                .take(NAMED_NOTES)
                .map(|note| note.pitch.to_string())
                .collect();
            ui.label(RichText::new(names.join(" ")).monospace());
        }
    }
    if review.chords {
        let count = transcription
            .chords
            .as_ref()
            .map_or(0, |chords| chords.markers.len());
        ui.label(trf("{} chords.", &[&count]));
    }
    let cents = (transcription.tuning * 100.0).round() as i32;
    if cents.abs() >= 10 {
        let text = if cents > 0 {
            trf(
                "Sung {} cents sharp: each note goes to the one meant.",
                &[&cents],
            )
        } else {
            trf(
                "Sung {} cents flat: each note goes to the one meant.",
                &[&-cents],
            )
        };
        ui.label(RichText::new(text).color(theme::TEXT_DIM));
    }
    if review.heard.recording.lost > 0 {
        ui.label(
            RichText::new(tr(
                "The computer was too busy to hear all of the take: a moment of it is silent.",
            ))
            .color(theme::RED),
        );
    }
}

/// The list of where the notes can go: each channel that is not a take,
/// a new channel, or nowhere.
fn destination_combo(ui: &mut egui::Ui, project: &Project, destination: &mut Destination) {
    let name = |destination: Destination| match destination {
        Destination::Channel(channel) => project
            .channels()
            .get(channel)
            .map_or_else(String::new, |channel| tr_name(&channel.name).into_owned()),
        Destination::New => tr("A new channel").to_string(),
        Destination::Nowhere => tr("Nowhere: keep only the recording").to_string(),
    };
    egui::ComboBox::from_id_salt("take-destination")
        .selected_text(name(*destination))
        .width(220.0)
        .height(400.0)
        .show_ui(ui, |ui| {
            for (index, channel) in project.channels().iter().enumerate() {
                if channel.is_take() {
                    continue;
                }
                let choice = Destination::Channel(index);
                ui.selectable_value(destination, choice, name(choice));
            }
            ui.separator();
            for choice in [Destination::New, Destination::Nowhere] {
                ui.selectable_value(destination, choice, name(choice));
            }
        });
}

/// The list of the playlist's tracks, for the recording.
fn track_combo(ui: &mut egui::Ui, project: &Project, track: &mut usize) {
    let tracks = project.playlist().tracks();
    let name = |index: usize| {
        tracks
            .get(index)
            .map_or_else(String::new, |track| tr_name(&track.name).into_owned())
    };
    let new = tracks.len() < MAX_TRACKS;
    let name = |index: usize| {
        if index >= tracks.len() {
            tr("A new track").to_string()
        } else {
            name(index)
        }
    };
    egui::ComboBox::from_id_salt("take-track")
        .selected_text(name(*track))
        .show_ui(ui, |ui| {
            for index in 0..tracks.len() + usize::from(new) {
                ui.selectable_value(track, index, name(index));
            }
        });
}

#[cfg(test)]
impl LineupApp {
    /// For the tests: a take sung along with the song from step `from`
    /// on, for `seconds`, in which `notes` were heard, waiting in the
    /// review window. Its recording is silence, saved in `folder`.
    pub(super) fn heard_take(
        &mut self,
        from: f64,
        seconds: f64,
        notes: Vec<SungNote>,
        folder: PathBuf,
    ) {
        const RATE: u32 = 8_000;
        let speed = f64::from(self.project.tempo.bpm()) / 60.0 * 4.0;
        let frames = (seconds * f64::from(RATE)) as usize;
        let marks = (0..frames)
            .step_by(80)
            .map(|frame| lineup_core::take::Mark {
                frame,
                position: from + frame as f64 / f64::from(RATE) * speed,
                steps_per_second: speed,
            })
            .collect();
        let heard = Heard {
            recording: Recording {
                samples: vec![0.0; frames],
                sample_rate: RATE,
                marks,
                lost: 0,
            },
            along: Along::Song(self.project.playback_arrangement()),
            notes,
            tuning: 0.0,
            drums: Vec::new(),
            bass: Vec::new(),
            chords: Vec::new(),
            name: None,
        };
        self.voice.folder = Some(folder);
        self.voice.review = Some(Review::new(
            &self.project,
            heard,
            None,
            &egui::Context::default(),
        ));
    }
}

#[cfg(test)]
impl LineupApp {
    /// For the tests: whether a recording added from a file is being
    /// heard.
    pub(super) fn hearing_recording(&self) -> bool {
        self.voice.loading.is_some()
    }

    /// For the tests: whether a take waits in the review window.
    pub(super) fn take_waiting(&self) -> bool {
        self.voice.review.is_some()
    }

    /// For the tests: saves takes in `folder`.
    pub(super) fn keep_takes_in(&mut self, folder: PathBuf) {
        self.voice.folder = Some(folder);
    }

    /// For the tests: the step the waiting take's recording starts on,
    /// and the track the review offers to keep it on.
    pub(super) fn take_place(&self) -> Option<(usize, usize)> {
        let review = self.voice.review.as_ref()?;
        Some((review.transcription.recorded()?.start, review.track))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The song's sound from `step` on, for `frames` frames, as song mode
    /// plays it: what the app sends the audio thread, then a jump to
    /// `step` and Play.
    fn played_from(app: &mut LineupApp, step: usize, frames: usize) -> Vec<f32> {
        use lineup_audio::{Command, Renderer};
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

    fn app() -> LineupApp {
        let ctx = egui::Context::default();
        LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()))
    }

    #[test]
    fn a_recording_added_plays_from_wherever_playback_starts() {
        // Twenty seconds of a C major chord, with a click on every beat at
        // 120 BPM.
        let rate = 44_100;
        let samples: Vec<f32> = (0..20 * rate)
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
        lineup_listen::wav::write(&samples, rate, &path).unwrap();

        let mut app = app();
        app.voice.folder = Some(dir.clone());
        app.open_recording(path);
        while app.hearing_recording() {
            std::thread::sleep(Duration::from_millis(20));
            app.check_loading(&egui::Context::default());
        }
        let review = app.voice.review.as_mut().unwrap();
        review.destination = Destination::Nowhere;
        app.write_take();
        assert!(app.project.channels().iter().any(|c| c.is_take()));

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
    fn a_take_moved_with_its_song_still_plays() {
        // Four seconds of an A, sung into a song with nothing else in it,
        // kept in the song's takes folder.
        let rate = 44_100;
        let samples: Vec<f32> = (0..4 * rate)
            .map(|frame| (frame as f32 / rate as f32 * 440.0 * std::f32::consts::TAU).sin() * 0.3)
            .collect();
        let root = std::env::temp_dir().join(format!("lineup-moved-{}", std::process::id()));
        std::fs::remove_dir_all(&root).ok();
        let before = root.join("before");
        std::fs::create_dir_all(&before).unwrap();
        let sung = root.join("sung.wav");
        lineup_listen::wav::write(&samples, rate, &sung).unwrap();

        let mut app = app();
        app.replace_project(Project::empty(), None);
        app.voice.folder = Some(before.join("song takes"));
        std::fs::create_dir_all(before.join("song takes")).unwrap();
        app.open_recording(sung);
        while app.hearing_recording() {
            std::thread::sleep(Duration::from_millis(20));
            app.check_loading(&egui::Context::default());
        }
        app.voice.review.as_mut().unwrap().destination = Destination::Nowhere;
        app.write_take();
        lineup_core::file::save(&app.project, &before.join("song.lineup")).unwrap();

        // The song and its takes folder go somewhere else together.
        let after = root.join("after");
        std::fs::rename(&before, &after).unwrap();
        let ctx = egui::Context::default();
        let mut moved = self::app();
        moved.open_path(&ctx, after.join("song.lineup"));
        let heard = loudness(&played_from(&mut moved, 0, rate as usize));
        assert!(heard > 0.1, "the take plays: {heard}");

        // Without its takes folder, the song is silent.
        std::fs::remove_dir_all(after.join("song takes")).unwrap();
        let mut lost = self::app();
        lost.open_path(&ctx, after.join("song.lineup"));
        let silent = loudness(&played_from(&mut lost, 0, rate as usize));
        assert!(silent < 0.001, "nothing to play: {silent}");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_recording_trimmed_at_both_ends_plays_only_what_its_clip_shows() {
        // Eight seconds of a tone that rises, so each moment of it sounds
        // different: four bars at 120 BPM.
        let rate = 44_100;
        let samples: Vec<f32> = (0..8 * rate)
            .map(|frame| {
                let t = frame as f32 / rate as f32;
                (t * (220.0 + 20.0 * t) * std::f32::consts::TAU).sin() * 0.3
            })
            .collect();
        let dir = std::env::temp_dir().join(format!("lineup-trimmed-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("rising.wav");
        lineup_listen::wav::write(&samples, rate, &path).unwrap();
        let mut app = app();
        app.voice.folder = Some(dir.clone());
        app.open_recording(path);
        while app.hearing_recording() {
            std::thread::sleep(Duration::from_millis(20));
            app.check_loading(&egui::Context::default());
        }
        app.voice.review.as_mut().unwrap().destination = Destination::Nowhere;
        app.write_take();
        // The frame song step `step` starts on.
        let song = app.project.playback_arrangement();
        let tempos = app.project.played_tempos(&song);
        let tempo = app.project.tempo;
        let at = |step: usize| {
            let seconds = lineup_core::tempo::seconds(step, |step| {
                tempos.get(step).copied().unwrap_or(tempo)
            });
            (seconds * f64::from(rate)) as usize
        };
        let whole = played_from(&mut app, 0, at(64));

        // Its clip, from bar 2 to bar 4 only.
        let (index, clip) = app
            .project
            .playlist()
            .clips()
            .iter()
            .copied()
            .enumerate()
            .find(|(_, clip)| clip.start == 0)
            .unwrap();
        let length = app.project.patterns()[clip.pattern].length();
        let trimmed = clip
            .trimmed_from(16, length)
            .and_then(|clip| clip.trimmed_to(48, length))
            .unwrap();
        assert!(app.project.trim_clip(index, trimmed));
        let out = played_from(&mut app, 0, at(64));
        assert!(
            loudness(&out[..at(16)]) < 0.001,
            "silent before the left edge"
        );
        // Under the clip, the very same sound as before, from the middle
        // of the recording on.
        let (from, to) = (at(16) + 100, at(48));
        let difference: Vec<f32> = out[from..to]
            .iter()
            .zip(&whole[from..to])
            .map(|(a, b)| a - b)
            .collect();
        assert!(loudness(&out[from..to]) > 0.1);
        assert!(loudness(&difference) < 0.01, "{}", loudness(&difference));
        assert!(
            loudness(&out[at(49)..]) < 0.001,
            "silent after the right edge"
        );
        // Played from step 20, it comes in at once, as it would.
        let from_20 = played_from(&mut app, 20, at(21) - at(20));
        let difference: Vec<f32> = from_20
            .iter()
            .zip(&whole[at(20)..at(21)])
            .map(|(a, b)| a - b)
            .collect();
        assert!(loudness(&difference) < 0.01, "{}", loudness(&difference));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn opening_another_project_stops_hearing_the_recording_added() {
        let mut app = app();
        app.voice.loading = Some(Loading {
            worker: thread::spawn(|| Err(String::new())),
            step: 0,
            name: String::new(),
            track: None,
            again: None,
        });

        let opened = Project::template();
        app.replace_project(opened.clone(), None);

        assert!(!app.hearing_recording(), "the recording was let go");
        assert_eq!(app.project, opened, "the opened project stays");
        assert_eq!(
            app.notice.as_deref(),
            Some("Stopped listening to the recording.")
        );
    }

    #[test]
    fn a_takes_name_is_free() {
        let mut project = Project::empty();
        assert_eq!(take_name(&project, None), "Take 1");
        assert_eq!(take_name(&project, Some("melody")), "melody");
        project.add_channel(lineup_core::Channel::take("Take 1", PathBuf::from("a.wav")));
        project.add_channel(lineup_core::Channel::take(
            "melody (2)",
            PathBuf::from("b.wav"),
        ));
        assert_eq!(take_name(&project, None), "Take 2");
        assert_eq!(take_name(&project, Some("melody")), "melody 2");
    }
}
