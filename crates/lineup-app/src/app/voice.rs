//! Takes: singing or whistling along with the song, into notes.
//!
//! The microphone button next to Rec records a *take* while the song (or,
//! in pattern mode, the pattern) plays: the voice, and where in the song
//! each moment of it was sung (see `lineup_audio::input`). While it
//! records, the transport shows how loud the voice is and the note it
//! hears, with how far off it is.
//!
//! When the take stops, a thread of its own hears its notes, their pitch,
//! start, end and loudness (see `lineup_listen::voice`), and the "Your
//! take" window asks where they go: into the channel that plays the tune
//! (the Lead), another channel, a new one, or nowhere. The recording
//! itself is kept on a track of the playlist, where it plays with the song
//! as a reference. Writing it in is one step of undo (see
//! `lineup_core::take`).

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
    Along, Keep, SungNote, Take, Target, Transcription, transcribe, tune_channel,
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

/// The furthest a take can be moved, in milliseconds.
const MOST_NUDGE: f64 = 500.0;

/// The grids the notes of a take can snap to, in steps: sixteenths,
/// eighths and quarter notes.
const GRIDS: [(usize, &str); 3] = [
    (1, "Sixteenth notes"),
    (2, "Eighth notes"),
    (4, "Quarter notes"),
];

/// The fewest notes of a take the review shows by name.
const NAMED_NOTES: usize = 12;

/// The take being recorded, heard, or waiting for where its notes go.
#[derive(Default)]
pub(super) struct Voice {
    recording: Option<Taking>,
    hearing: Option<JoinHandle<Heard>>,
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
    keep: bool,
    track: usize,
    transcription: Transcription,
}

impl Review {
    fn new(project: &Project, heard: Heard) -> Self {
        let take = Take {
            sample_rate: heard.recording.sample_rate,
            frames: heard.recording.samples.len(),
            marks: heard.recording.marks.clone(),
            along: heard.along.clone(),
        };
        let grid = 1;
        let transcription = transcribe(project, &take, &heard.notes, grid, heard.tuning);
        let destination = tune_channel(project).map_or(Destination::New, Destination::Channel);
        let track = free_track(project, transcription.recorded().unwrap_or(0..0));
        Review {
            heard,
            take,
            destination,
            grid,
            keep: true,
            track,
            transcription,
        }
    }

    fn snap(&mut self, project: &Project, grid: usize) {
        if grid != self.grid {
            self.grid = grid;
            self.transcription = transcribe(
                project,
                &self.take,
                &self.heard.notes,
                grid,
                self.heard.tuning,
            );
        }
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

/// The name of the next take: "Take 1", or the first number no take has.
fn next_take_name(project: &Project) -> String {
    let used = |number: usize| {
        let name = format!("Take {number}");
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
    let number = (1..).find(|&number| !used(number)).unwrap_or(1);
    format!("Take {number}")
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
    let track = voice::pitch_track(&recording.samples, recording.sample_rate);
    let notes = voice::notes(&track);
    let tuning = voice::tuning(&notes);
    Heard {
        recording,
        along,
        notes,
        tuning,
    }
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
    /// being replaced, whose song the take does not belong to.
    pub(super) fn forget_voice(&mut self) {
        if let Some(recording) = self.voice.recording.take()
            && let Some((_, clock)) = recording.recorder.finish()
            && let Audio::Running(engine) = &mut self.audio
        {
            engine.give_back_song_clock(clock);
        }
        self.voice.hearing = None;
        self.voice.review = None;
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
                Ok(heard) => self.voice.review = Some(Review::new(&self.project, heard)),
                Err(_) => self.message = Some(tr("Listening to the take failed.").to_string()),
            }
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
                        ui.label(tr("Notes go to"));
                        destination_combo(ui, project, &mut review.destination);
                        ui.end_row();

                        ui.label(tr("On the grid of"));
                        ui.horizontal(|ui| {
                            let mut grid = review.grid;
                            for (steps, name) in GRIDS {
                                ui.selectable_value(&mut grid, steps, tr(name));
                            }
                            review.snap(project, grid);
                        });
                        ui.end_row();

                        ui.label(tr("The recording"));
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut review.keep, tr("Keep it"));
                            if review.keep && matches!(review.take.along, Along::Song(_)) {
                                track_combo(ui, project, &mut review.track);
                            }
                        });
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
                    let nothing = review.destination == Destination::Nowhere && !review.keep;
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
        let transcription = transcribe(
            &self.project,
            &review.take,
            &review.heard.notes,
            review.grid,
            review.heard.tuning,
        );
        let name = next_take_name(&self.project);
        let mut recordings = Vec::new();
        if review.keep {
            let folder = self.takes_folder();
            let rate = review.heard.recording.sample_rate;
            for (index, piece) in transcription.pieces.iter().enumerate() {
                let file = if index == 0 {
                    format!("{} - {name}", self.file.stem())
                } else {
                    format!("{} - {name} ({})", self.file.stem(), index + 1)
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
        let target = match review.destination {
            Destination::Channel(channel) => Target::Channel(channel),
            Destination::New => Target::New {
                name: "Sung notes".to_string(),
                program: FLUTE,
            },
            Destination::Nowhere => Target::Nowhere,
        };
        let keep = Keep {
            name,
            track: review.track,
            recordings,
        };
        let along = review.take.along.clone();
        let added = self.edit("record a take", |project| {
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
        (_, Some(_)) => tr("No notes were heard: only the recording can be kept.").to_string(),
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
        };
        self.voice.folder = Some(folder);
        self.voice.review = Some(Review::new(&self.project, heard));
    }
}
