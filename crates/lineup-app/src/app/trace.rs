//! Traces: every note the player plays, written down to read instead of
//! listening to (see [`lineup_audio::trace`]).
//!
//! There are two ways to make one:
//!
//! - `ferrous-lineup --trace song.lineup` (or the name of a demo) plays
//!   the song as File > Export WAV does, on an audio thread of its own,
//!   without a window or a sound card, and writes the trace of it.
//!   Nothing is heard and no sound is saved, but the notes are the
//!   export's, frame for frame.
//! - Settings > Playback > Trace writes one while you play: one table
//!   for each playback, from Play to Stop, added to a file in the app's
//!   folder. The audio thread hands the notes over as it plays them, and
//!   the window writes them down once a frame.

use std::collections::VecDeque;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use eframe::egui;
use lineup_audio::trace::{Labels, TraceEvent, write_table};
use lineup_audio::{Command, Trace, TraceReader, trace_queue};
use lineup_core::Project;
use lineup_core::library::{MICROMODE_STYLES, ROW_STYLES, SONGS, STYLES};

use super::library::Demo;
use super::{APP_NAME, Audio, LineupApp};

/// The sample rate of a trace from the command line: File > Export WAV's.
pub const TRACE_SAMPLE_RATE: u32 = super::export::EXPORT_SAMPLE_RATE;

/// The file a trace made while playing goes to.
const LIVE_FILE: &str = "playback-trace.tsv";

/// What names the channels, patterns and bars of a playback: the song's
/// name and the project as they were when it started, and whether song
/// mode played it (or else which pattern).
struct Playing {
    title: String,
    project: Project,
    song: bool,
    pattern: usize,
}

/// A trace being written while you play.
pub(super) struct LiveTrace {
    reader: TraceReader,
    /// What the audio thread has handed over and is not written yet.
    trace: Trace,
    path: PathBuf,
    file: File,
    /// One for each playback in `trace`, in order.
    playing: VecDeque<Playing>,
}

impl LiveTrace {
    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    /// Writes the playbacks that are over, and keeps the one going.
    fn write_finished(&mut self) -> io::Result<()> {
        for trace in self.trace.take_finished() {
            self.write(&trace)?;
        }
        Ok(())
    }

    /// Writes one playback's table, then a blank line before the next.
    fn write(&mut self, trace: &Trace) -> io::Result<()> {
        let Some(playing) = self.playing.pop_front() else {
            return Ok(());
        };
        let arrangement = playing.project.playback_arrangement();
        let labels = if playing.song {
            Labels::song(&playing.title, &playing.project, &arrangement)
        } else {
            Labels::pattern(&playing.title, &playing.project, playing.pattern)
        };
        let mut table = Vec::new();
        write_table(&mut table, trace, &labels)?;
        table.push(b'\n');
        self.file.write_all(&table)?;
        self.file.flush()
    }
}

impl Drop for LiveTrace {
    /// Writes the playback still going when the trace stops, or the app
    /// closes: its last notes have no end yet.
    fn drop(&mut self) {
        let _ = self.write_finished();
        if !self.trace.events.is_empty() {
            let rate = self.trace.sample_rate;
            let rest = std::mem::replace(&mut self.trace, Trace::new(rate));
            let _ = self.write(&rest);
        }
    }
}

/// Where a trace made while playing is written: in the folder the app
/// keeps its settings in, or else the system's folder for temporary files.
fn live_path() -> PathBuf {
    eframe::storage_dir(APP_NAME)
        .unwrap_or_else(std::env::temp_dir)
        .join(LIVE_FILE)
}

impl LineupApp {
    /// Starts or stops writing a trace of what is played.
    pub(super) fn set_tracing(&mut self, on: bool) {
        if !on {
            if self.trace.is_some() {
                self.send(Command::SetTrace(None));
                self.check_trace();
                self.trace = None;
            }
            return;
        }
        if self.trace.is_some() || !matches!(self.audio, Audio::Running(_)) {
            return;
        }
        let path = live_path();
        let file = path
            .parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| File::create(&path));
        let mut file = match file {
            Ok(file) => file,
            Err(err) => {
                self.message = Some(format!("Could not write {}: {err}", path.display()));
                return;
            }
        };
        let _ = writeln!(
            file,
            "# Ferrous Lineup playback traces: one table for each playback, from Play to Stop.\n"
        );
        let (tracer, reader) = trace_queue();
        self.send(Command::SetTrace(Some(Box::new(tracer))));
        self.trace = Some(LiveTrace {
            reader,
            trace: Trace::new(self.sample_rate()),
            path,
            file,
            playing: VecDeque::new(),
        });
    }

    /// The sound card was opened again: the new audio thread is given a
    /// way to hand over its notes, and the trace goes on in the same file.
    /// A playback the old audio thread was in the middle of ends here.
    pub(super) fn resume_trace(&mut self) {
        self.check_trace();
        let sample_rate = self.sample_rate();
        let Some(live) = &mut self.trace else {
            return;
        };
        let rest = std::mem::replace(&mut live.trace, Trace::new(sample_rate));
        let written = if rest.events.is_empty() {
            Ok(())
        } else {
            live.write(&rest)
        };
        live.playing.clear();
        let (tracer, reader) = trace_queue();
        live.reader = reader;
        if let Err(err) = written {
            self.message = Some(format!("Could not write {}: {err}", live.path.display()));
        }
        self.send(Command::SetTrace(Some(Box::new(tracer))));
    }

    /// Once a frame: writes down what the audio thread traced.
    pub(super) fn check_trace(&mut self) {
        let Some(live) = &mut self.trace else {
            return;
        };
        let before = live.trace.events.len();
        live.reader.read_into(&mut live.trace);
        // Each playback that started plays the song as it is now.
        for event in &live.trace.events[before..] {
            if let TraceEvent::Play { song, pattern, .. } = *event {
                live.playing.push_back(Playing {
                    title: self
                        .file
                        .song_name()
                        .unwrap_or_else(|| "Untitled".to_string()),
                    project: self.project.clone(),
                    song,
                    pattern,
                });
            }
        }
        if let Err(err) = live.write_finished() {
            self.message = Some(format!("Could not write {}: {err}", live.path.display()));
        }
    }
}

/// Every demo the File menu opens.
fn demos() -> Vec<Demo> {
    let mut demos = vec![Demo::Start];
    demos.extend(SONGS.iter().map(Demo::Song));
    demos.extend(STYLES.iter().map(Demo::Style));
    demos.extend(ROW_STYLES.iter().map(Demo::Row));
    demos.extend(MICROMODE_STYLES.iter().map(Demo::Micromode));
    demos
}

/// The names of every demo, for `--demos`.
pub fn demo_names() -> Vec<&'static str> {
    demos().into_iter().map(Demo::name).collect()
}

/// The demo called `name`: exactly, or else ignoring capitals, or else
/// the only one whose name has `name` in it.
fn demo_named(name: &str) -> Result<Demo, String> {
    let demos = demos();
    if let Some(&demo) = demos.iter().find(|demo| demo.name() == name) {
        return Ok(demo);
    }
    let lower = name.to_lowercase();
    if let Some(&demo) = demos
        .iter()
        .find(|demo| demo.name().to_lowercase() == lower)
    {
        return Ok(demo);
    }
    let matching: Vec<Demo> = demos
        .into_iter()
        .filter(|demo| demo.name().to_lowercase().contains(&lower))
        .collect();
    match matching.as_slice() {
        [demo] => Ok(*demo),
        [] => Err(format!(
            "There is no file or demo called \"{name}\" (--demos lists the demos)."
        )),
        several => {
            let names: Vec<&str> = several.iter().map(|demo| demo.name()).collect();
            Err(format!("\"{name}\" could be any of: {}.", names.join(", ")))
        }
    }
}

/// The app with `source` open, without a window or a sound card:
/// a project file (or anything File > Open opens but a recording) or
/// the name of a demo.
fn open_headless(source: &OsStr) -> Result<LineupApp, String> {
    let ctx = egui::Context::default();
    let mut app = LineupApp::with_audio(&ctx, None, Audio::Unavailable("tracing".into()));
    let path = Path::new(source);
    let name = source.to_string_lossy();
    if path.is_file() {
        if lineup_listen::is_audio_file(path) {
            return Err(format!(
                "{name} is a recording, which has no notes to trace yet: open it in Ferrous Lineup and save the song first."
            ));
        }
        app.open_path(&ctx, path.to_path_buf());
        // A file that could not be opened leaves the starting song open.
        let opened = app.file.song_name().as_deref() != Some(Demo::Start.name());
        match app.message.take() {
            Some(message) if !opened => return Err(message),
            Some(message) => eprintln!("{message}"),
            None if !opened => return Err(format!("Could not open {name}")),
            None => {}
        }
    } else {
        app.open_demo(demo_named(&name)?);
    }
    Ok(app)
}

/// The trace of `source` (see [`open_headless`]) played from start to
/// end as File > Export WAV plays it, at `sample_rate`, as a table.
pub fn trace_table(source: &OsStr, sample_rate: u32) -> Result<Vec<u8>, String> {
    let mut app = open_headless(source)?;
    let trace = app.trace_song(sample_rate);
    let title = app
        .file
        .song_name()
        .unwrap_or_else(|| source.to_string_lossy().into_owned());
    let arrangement = app.project.arrangement();
    let labels = Labels::song(&title, &app.project, &arrangement);
    let mut table = Vec::new();
    write_table(&mut table, &trace, &labels).map_err(|err| err.to_string())?;
    Ok(table)
}

impl LineupApp {
    /// The trace of the whole song played from start to end, at
    /// `sample_rate`, as File > Export WAV plays it.
    pub(super) fn trace_song(&mut self, sample_rate: u32) -> Trace {
        let mut render = self.song_render_traced(sample_rate);
        render.renderer.skip(render.frames);
        render.renderer.take_trace().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_audio::trace::{COLUMNS, End};
    use lineup_core::{Pattern, TempoMarker, TimeSignature};

    fn app_with(project: Project) -> LineupApp {
        let ctx = egui::Context::default();
        let mut app = LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()));
        app.replace_project(project, None);
        app
    }

    #[test]
    fn a_demo_is_found_by_its_name() {
        assert_eq!(demo_named("The starting song"), Ok(Demo::Start));
        assert_eq!(demo_named("the STARTING song"), Ok(Demo::Start));
        assert!(demo_named("no such song at all").is_err());
        let first = SONGS[0].name;
        assert_eq!(demo_named(first), Ok(Demo::Song(&SONGS[0])));
    }

    #[test]
    fn the_trace_plays_every_note_of_the_export() {
        let mut app = app_with(Project::demo());
        let trace = app.trace_song(TRACE_SAMPLE_RATE);
        let notes = trace.notes();
        // Every note of every clip, as the MIDI export counts them.
        let song = app.project.arrangement();
        let mut expected = 0;
        for play in &song.plays {
            let pattern: &Pattern = &app.project.patterns()[play.pattern];
            for channel in 0..app.project.channels().len() {
                if app.project.channel_muted(channel) {
                    continue;
                }
                let notes = app.project.played_notes(play.pattern, channel);
                expected += (play.start..play.end())
                    .filter_map(|step| play.pattern_step(step, pattern.length()))
                    .map(|at| notes.iter().filter(|note| note.start == at).count())
                    .sum::<usize>();
            }
        }
        assert!(expected > 0);
        assert_eq!(notes.len(), expected);
        assert!(notes.iter().all(|note| note.end == Some(End::Gate)));
        let last = notes.iter().map(|note| note.start).fold(0.0, f64::max);
        assert!(last < app.project.played_seconds(&song));
    }

    #[test]
    fn swing_plays_the_off_beats_late() {
        let mut project = Project::demo();
        project.swing = 50;
        let mut app = app_with(project);
        let notes = app.trace_song(TRACE_SAMPLE_RATE).notes();
        let late: Vec<_> = notes.iter().filter(|note| note.late > 0.0).collect();
        assert!(!late.is_empty(), "swing delays the off-beat notes");
        // Only the second eighth of each beat is swung.
        assert!(late.iter().all(|note| note.origin.pattern_step % 4 == 2));
        assert!(late.iter().all(|note| note.step.fract() > 0.0));
    }

    #[test]
    fn a_tempo_marker_changes_the_seconds_not_the_steps() {
        let mut project = Project::demo();
        let steady = app_with(project.clone()).trace_song(TRACE_SAMPLE_RATE);
        let tempo = project.tempo.bpm();
        assert!(project.set_tempo_marker(TempoMarker::new(16, tempo / 2.0)));
        let slower = app_with(project).trace_song(TRACE_SAMPLE_RATE);
        let (steady, slower) = (steady.notes(), slower.notes());
        assert_eq!(steady.len(), slower.len());
        for (before, after) in steady.iter().zip(&slower) {
            assert_eq!(before.step, after.step);
            // Step 16 starts when it did: it is the first step played slower.
            if before.step <= 16.0 {
                assert_eq!(before.start, after.start);
            } else {
                assert!(after.start > before.start);
                assert_eq!(after.tempo.bpm(), tempo / 2.0);
            }
        }
    }

    #[test]
    fn re_metering_keeps_each_bar_as_long_in_seconds() {
        // The bug a trace catches and a screenshot does not: a song put
        // into 3/4 that looks right but plays its bars at the old pace.
        let common = Project::demo();
        let mut waltz = common.clone();
        assert!(lineup_core::remeter::remeter(
            &mut waltz,
            TimeSignature::new(3, 4)
        ));
        let bar_starts = |project: Project| {
            let mut app = app_with(project);
            let meter = app.project.played_meter(&app.project.arrangement());
            let notes = app.trace_song(TRACE_SAMPLE_RATE).notes();
            let mut starts: Vec<(usize, f64)> = notes
                .iter()
                .filter(|note| note.step.fract() == 0.0 && meter.is_bar_start(note.step as usize))
                .map(|note| (meter.bar_at(note.step as usize).number, note.start))
                .collect();
            starts.dedup_by_key(|(bar, _)| *bar);
            starts
        };
        let (four, three) = (bar_starts(common), bar_starts(waltz));
        assert!(four.len() > 2);
        assert_eq!(four.len(), three.len());
        for ((bar, before), (same_bar, after)) in four.iter().zip(&three) {
            assert_eq!(bar, same_bar);
            assert!(
                (before - after).abs() < 0.001,
                "bar {bar}: {before} s, then {after} s"
            );
        }
    }

    #[test]
    fn a_trace_while_playing_writes_a_table_for_each_playback() {
        let mut app = app_with(Project::demo());
        let song = app.trace_song(TRACE_SAMPLE_RATE);
        let end = song.events.last().map_or(0, |event| match *event {
            TraceEvent::NoteOff { frame, .. } | TraceEvent::Step { frame, .. } => frame,
            _ => 0,
        });
        // The song played to its end and stopped, then pattern 1 started
        // and was still playing when the trace was turned off.
        let mut trace = song.clone();
        trace.events.push(TraceEvent::Stop { frame: end });
        trace
            .events
            .extend(song.events.iter().take(40).map(|&event| match event {
                TraceEvent::Play { frame, step, .. } => TraceEvent::Play {
                    frame,
                    step,
                    song: false,
                    pattern: 0,
                },
                other => other,
            }));
        let playing = |song| Playing {
            title: "Traced".to_string(),
            project: app.project.clone(),
            song,
            pattern: 0,
        };
        let path = std::env::temp_dir().join(format!("lineup-trace-{}.tsv", std::process::id()));
        let (_, reader) = trace_queue();
        let mut live = LiveTrace {
            reader,
            trace,
            path: path.clone(),
            file: File::create(&path).unwrap(),
            playing: VecDeque::from([playing(true), playing(false)]),
        };
        live.write_finished().unwrap();
        let written = fs::read_to_string(&path).unwrap();
        assert_eq!(
            written.matches("# Ferrous Lineup playback trace").count(),
            1
        );
        drop(live);
        let written = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();
        let tables: Vec<&str> = written.split("# Ferrous Lineup playback trace").collect();
        assert_eq!(tables.len(), 3, "a table for each playback");
        assert!(tables[2].contains("notes were still sounding"));
        assert!(tables[2].contains(&format!("sample rate: {TRACE_SAMPLE_RATE} Hz")));
        let lines = |table: &str| table.lines().filter(|line| !line.starts_with('#')).count();
        assert!(lines(tables[1]) > lines(tables[2]));
    }

    #[test]
    fn the_table_has_a_line_for_each_note() {
        let name = OsStr::new(Demo::Start.name());
        let table = String::from_utf8(trace_table(name, TRACE_SAMPLE_RATE).unwrap()).unwrap();
        let lines: Vec<&str> = table
            .lines()
            .filter(|line| !line.starts_with('#'))
            .collect();
        assert_eq!(lines[0], COLUMNS.join("\t"));
        let mut app = open_headless(name).unwrap();
        let notes = app.trace_song(TRACE_SAMPLE_RATE).notes();
        assert_eq!(lines.len() - 1, notes.len());
        assert!(
            lines[1..]
                .iter()
                .all(|line| line.split('\t').count() == COLUMNS.len())
        );
        assert!(table.contains("# Ferrous Lineup playback trace: The starting song"));
        let time = Project::demo().time_signature;
        assert!(table.contains(&format!("# time: step 0, bar 1: {time}")));
    }
}
