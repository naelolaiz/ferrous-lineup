//! The main window.
//!
//! The playlist fills the window, as in other pattern-based studios. The
//! chord sheet, the channel rack and the piano roll are windows on top of
//! it, opened with F5, F6 and F7. Opening and saving files lives in the
//! [`files`] module, undo and redo in [`edit`], saving the song as a WAV
//! or MIDI file in [`export`], and playing and recording on a keyboard in
//! [`live`], and the count-in, metronome and practice speed in
//! [`practice`].
//!
//! Every change to the project goes through [`LineupApp::edit`], which
//! records it for undo and tells the audio thread what changed.

mod chord_text;
mod demo_menu;
mod ear;
mod edit;
mod export;
mod files;
mod follow;
mod form;
mod import;
mod key_finder;
mod lead;
mod library;
mod live;
mod mixer;
mod practice;
mod settings;
mod shortcuts;
mod sounds;
mod styles;
mod swing;
mod trace;
#[cfg(test)]
mod ux_tests;
mod video;
mod view;

pub use trace::{TRACE_SAMPLE_RATE, demo_names, trace_table};

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui;
use lineup_audio::instrument::Sampler;
use lineup_audio::{
    AudioEngine, ChannelNotes, Command, Instrument, PlayMode, SampleCache, SampleData,
};
use lineup_core::{
    Channel, ChannelKind, DEFAULT_VELOCITY, History, Meter, MidiMessage, Note, PartSet, Pass,
    Pattern, Project, SampleSource, Scale, SongPart, Source, SourceKind, Tempo,
};

use crate::channel_rack::{self, RackAction};
use crate::chord_sheet::{ChordSheet, SheetAction};
use crate::ear_training::EarTraining;
use crate::harmony_keys::{HarmonyAction, HarmonyKeys};
use crate::lead_sheet::LeadSheetWindow;
use crate::library_browser::Tried;
use crate::midi::Midi;
use crate::piano_roll::{self, PianoRoll, RollAction};
use crate::playlist::{self, PlaylistAction, PlaylistView};
use crate::theme::{self, HintBar};
use crate::widgets;

/// The app's name, in the title bar and for the folder its settings and
/// downloads are kept in.
pub const APP_NAME: &str = "Ferrous Lineup";

/// How wide the pattern list in the toolbar is.
const PATTERN_LIST_WIDTH: f32 = 104.0;

/// The sample rate used to prepare sounds when there is no sound card.
const FALLBACK_SAMPLE_RATE: u32 = 48_000;

/// Whether we managed to open the sound card.
///
/// The app still runs without audio (for example on a machine with no
/// sound card), and shows the reason in the status bar.
enum Audio {
    /// Boxed: the engine is much bigger than a `String`, and every
    /// `Audio` would otherwise take as much room as the biggest variant.
    Running(Box<AudioEngine>),
    Unavailable(String),
}

/// The first Lineup Osc channel, where the piano roll opens; or the first
/// channel if there is none.
fn first_synth(project: &Project) -> usize {
    project
        .channels()
        .iter()
        .position(|channel| channel.kind == ChannelKind::Synth)
        .unwrap_or(0)
}

/// All the state of the application window.
pub struct LineupApp {
    audio: Audio,
    project: Project,
    /// The selected pattern: the one the channel rack and the piano roll
    /// show, pattern mode plays, and new playlist clips use.
    pattern: usize,
    mode: PlayMode,
    playing: bool,
    /// Where playback starts or carries on from while stopped or paused:
    /// a step of the pattern in pattern mode, of the song in song mode.
    cursor: usize,
    /// Where Stop goes back to: the last place the playhead was moved to.
    marker: usize,
    /// The pattern the audio thread plays over the song, if any (the
    /// rack's "Play over the song"): the selected one, sent again whenever
    /// that changes.
    over_song: Option<usize>,
    mixer_open: bool,
    /// Help > Keyboard shortcuts.
    shortcuts_open: bool,
    settings_open: bool,
    settings: settings::SettingsView,
    style_editor: crate::style_editor::StyleEditor,
    style_files: styles::StyleFiles,
    rack_open: bool,
    /// The size the rack's rows needed last frame. When it changes (more
    /// steps, another channel, another pattern), the window is fitted to
    /// the rows again.
    rack_natural: egui::Vec2,
    piano_roll: PianoRoll,
    playlist: PlaylistView,
    harmony_keys: HarmonyKeys,
    /// The song's harmony bar by bar, as a lead sheet writes it.
    chord_sheet: ChordSheet,
    /// The patterns the library wrote, to rewrite on the next click.
    auditions: Vec<library::Audition>,
    /// The style and rhythm the whole song plays: a demo's own, or the
    /// last one the library gave the song. Shown selected in the library
    /// when the selected pattern is not one the library wrote.
    song_library: Tried,
    /// The song's style and rhythm in versions of the project the library
    /// changed, so undo and redo show the right ones selected.
    song_versions: Vec<(Project, Tried)>,
    /// Sources before meter changes, including the latest independent preset options.
    song_sources: Vec<(Project, library::SongSource)>,
    file: files::FileState,
    /// Earlier versions of the project, for undo and redo.
    history: History<Project>,
    /// Sample files already read, so undo and export do not read them again.
    samples: SampleCache,
    /// The WAV or video export running in the background, if any.
    export: Option<export::Export>,
    /// The dialog that asks what a video shows.
    video_dialog: video::VideoDialog,
    /// The trace of what is played, while Settings > Playback > Trace is on.
    trace: Option<trace::LiveTrace>,
    /// A recording being listened to in the background, if any.
    import: Option<import::Import>,
    /// Find the key: the key worked out from the music.
    key_finder: key_finder::KeyFinder,
    /// MIDI, the typing keyboard, and recording.
    live: live::Live,
    /// The SoundFont the instruments can play from.
    sounds: sounds::Sounds,
    /// The count-in, the metronome and the practice speed.
    practice: practice::Practice,
    /// The swing each style was last played with.
    swings: swing::SwingMemory,
    /// The ear-training window, an experiment.
    ear_training: EarTraining,
    lead_sheet: LeadSheetWindow,
    /// The last problem to show in the status bar, such as a bad WAV file.
    message: Option<String>,
    /// The last piece of good news for the status bar ("Exported song.wav").
    notice: Option<String>,
    /// The line at the top that says what the thing under the mouse does.
    hint_bar: HintBar,
    /// A window to show in front of the others on the next frame.
    bring_to_front: Option<egui::Id>,
    /// What was clicked last, which the zoom keys zoom while the pointer
    /// is outside the window.
    zoom_target: view::Target,
}

/// The id of the channel rack window.
fn rack_id() -> egui::Id {
    egui::Id::unique("channel-rack")
}

impl LineupApp {
    /// Called once by eframe, before the first frame is drawn. If `path`
    /// is given (a file named on the command line), it is opened;
    /// otherwise the demo song is.
    pub fn new(cc: &eframe::CreationContext<'_>, path: Option<PathBuf>) -> Self {
        theme::apply(&cc.egui_ctx);
        // The zoom keys zoom the part of the screen under the pointer, and
        // the whole interface only elsewhere (see view.rs). Alt and the wheel zooms too, so
        // egui must not take it for scrolling: the scroll it smooths out
        // over the next frames would move the view after the zoom.
        cc.egui_ctx.options_mut(|options| {
            options.zoom_with_keyboard = false;
            options.input_options.zoom_modifier = egui::Modifiers::COMMAND | egui::Modifiers::ALT;
        });

        let audio = match AudioEngine::start() {
            Ok(engine) => Audio::Running(Box::new(engine)),
            Err(err) => Audio::Unavailable(err.to_string()),
        };
        Self::with_audio(&cc.egui_ctx, path, audio)
    }

    fn with_audio(ctx: &egui::Context, path: Option<PathBuf>, mut audio: Audio) -> Self {
        // The engine's ways in and out for MIDI go to the MIDI threads.
        let (live_input, midi_output) = match &mut audio {
            Audio::Running(engine) => (engine.take_live_input(), engine.take_midi_output()),
            Audio::Unavailable(_) => (None, None),
        };
        let midi = Midi::new(ctx, live_input, midi_output);

        let project = library::Demo::Start.project();
        let piano_roll = PianoRoll::new(first_synth(&project));
        let mut app = LineupApp {
            audio,
            file: files::FileState::new(&project),
            project,
            pattern: 0,
            mode: PlayMode::Song,
            playing: false,
            over_song: None,
            cursor: 0,
            marker: 0,
            mixer_open: false,
            shortcuts_open: false,
            settings_open: false,
            settings: settings::SettingsView::default(),
            style_editor: crate::style_editor::StyleEditor::new(),
            style_files: styles::StyleFiles::new(ctx),
            rack_open: true,
            rack_natural: egui::Vec2::ZERO,
            piano_roll,
            playlist: PlaylistView::new(),
            harmony_keys: HarmonyKeys::new(),
            chord_sheet: ChordSheet::new(),
            auditions: Vec::new(),
            song_library: Tried::default(),
            song_versions: Vec::new(),
            song_sources: Vec::new(),
            history: History::default(),
            samples: SampleCache::new(),
            export: None,
            video_dialog: video::VideoDialog::default(),
            trace: None,
            import: None,
            key_finder: key_finder::KeyFinder::default(),
            live: live::Live::new(midi),
            sounds: sounds::Sounds::new(ctx),
            practice: practice::Practice::new(ctx),
            swings: swing::SwingMemory::new(ctx),
            ear_training: EarTraining::new(ctx),
            lead_sheet: LeadSheetWindow::new(ctx),
            message: None,
            notice: None,
            hint_bar: HintBar::default(),
            bring_to_front: None,
            zoom_target: view::Target::Interface,
        };
        app.playlist.library_browser_mut().restore_preferences(ctx);
        app.send_whole_project();
        app.reload_user_styles();
        if let Some(path) = path {
            app.open_path(ctx, path);
        }
        app
    }

    /// Sends a command to the audio thread, if audio is running.
    fn send(&mut self, command: Command) {
        if let Audio::Running(engine) = &mut self.audio {
            engine.send(command);
        }
    }

    fn sample_rate(&self) -> u32 {
        match &self.audio {
            Audio::Running(engine) => engine.info().sample_rate,
            Audio::Unavailable(_) => FALLBACK_SAMPLE_RATE,
        }
    }

    /// Builds the instrument for a channel, at `sample_rate`: from the
    /// SoundFont when it is on and has a sound for the channel. If a
    /// sample file cannot be loaded, the channel gets a silent sampler and
    /// the status bar says why.
    fn build_instrument(&mut self, channel: &Channel, sample_rate: u32) -> Instrument {
        if let Some((sound_font, drums)) = self.sounds.in_use()
            && let Some(instrument) =
                Instrument::from_sound_font(channel, sample_rate, sound_font, drums)
        {
            return instrument;
        }
        match Instrument::for_channel(channel, sample_rate, &mut self.samples) {
            Ok(instrument) => instrument,
            Err(err) => {
                self.message = Some(err.to_string());
                Instrument::Sampler(Sampler::new(Arc::new(SampleData::new(Vec::new(), 1))))
            }
        }
    }

    /// Everything an empty audio thread needs to play the project, with
    /// instruments made for `sample_rate` and the tempo at `speed` (1.0
    /// for the song's own): the tempo, the patterns, the channels and the
    /// song. Patterns go first, so each channel can bring its notes for
    /// all of them. Used at startup, after opening a file, and to export
    /// the song.
    fn project_commands(&mut self, sample_rate: u32, speed: f32) -> Vec<Command> {
        let mut commands = vec![
            Command::SetTempo(practice::at_speed(self.project.tempo, speed)),
            Command::SetLooseTiming(self.project.loose_timing),
            Command::SetMasterVolume(self.project.mixer.master),
            Command::SetReverb(self.project.mixer.reverb),
        ];
        for (index, pattern) in self.project.patterns().iter().enumerate() {
            commands.push(Command::InsertPattern {
                index,
                length: pattern.length(),
            });
        }
        for index in 0..self.project.channels().len() {
            commands.push(self.channel_command(index, sample_rate));
        }
        commands.extend(self.song_commands_at(speed));
        commands
    }

    /// Tells the audio thread about the whole project, at startup and
    /// after opening a file.
    fn send_whole_project(&mut self) {
        self.send(Command::Clear);
        let speed = self.practice.speed();
        for command in self.project_commands(self.sample_rate(), speed) {
            self.send(command);
        }
        self.practice.resend();
        self.send(Command::SelectPattern(self.pattern));
        self.send(Command::SetMode(self.mode));
        self.live.resend_channel();
        self.live.follow.forget();
        self.live.channels_changed(self.project.channels().len());
    }

    /// The command that inserts channel `index` of the project, with its
    /// instrument and its notes in every pattern.
    fn channel_command(&mut self, index: usize, sample_rate: u32) -> Command {
        let channel = self.project.channels()[index].clone();
        // The notes as they are played, with the song's swing.
        let notes = ChannelNotes::new(
            (0..self.project.patterns().len())
                .map(|pattern| self.project.played_notes(pattern, index).into_owned()),
        );
        Command::InsertChannel {
            index,
            instrument: self.build_instrument(&channel, sample_rate),
            notes,
            volume: channel.volume,
            muted: self.project.channel_muted(index),
        }
    }

    /// The song as song mode plays it: the clips on tracks that are not
    /// muted, with every repeat unrolled, cut to the loop region if one
    /// is set, the tempo of each of its steps at the practice speed, and
    /// what the metronome plays on each.
    fn song_commands(&self) -> [Command; 3] {
        self.song_commands_at(self.practice.speed())
    }

    /// The song as song mode plays it, at `speed`.
    fn song_commands_at(&self, speed: f32) -> [Command; 3] {
        let song = self.project.playback_arrangement();
        let tempos = self
            .project
            .played_tempos(&song)
            .into_iter()
            .map(|tempo| practice::at_speed(tempo, speed))
            .collect();
        let clicks = self.project.played_clicks(&song);
        [
            Command::SetSong {
                plays: song.plays,
                length: song.length,
            },
            Command::SetSongTempos(tempos),
            Command::SetSongClicks(clicks),
        ]
    }

    /// Changes the selected pattern, as one undoable step called `label`.
    fn edit_pattern(&mut self, label: &'static str, change: impl FnOnce(&mut Pattern)) {
        let index = self.pattern;
        self.edit(label, |project| {
            if let Some(pattern) = project.pattern_mut(index) {
                change(pattern);
            }
        });
    }

    /// Plays from the cursor, or stops and goes back to the marker. A
    /// cursor or marker past the end of the loop (left there by a longer
    /// pattern, or a song that got shorter) goes back to the start.
    fn set_playing(&mut self, playing: bool) {
        let length = self.loop_length();
        if self.marker >= length {
            self.marker = 0;
        }
        if self.cursor >= length {
            self.cursor = 0;
        }
        if playing {
            self.send(Command::Seek(self.cursor));
            self.send(Command::Play);
            self.playing = true;
        } else {
            self.halt();
            self.cursor = self.marker;
            self.send(Command::Seek(self.cursor));
        }
    }

    /// Plays the selected pattern over the playing song, from its next
    /// bar, or stops playing it.
    fn play_over_song(&mut self, on: bool) {
        if on && self.playing && self.mode == PlayMode::Song {
            self.over_song = Some(self.pattern);
            self.send(Command::SetOverlay(Some(self.pattern)));
        } else if !on && self.over_song.take().is_some() {
            self.send(Command::SetOverlay(None));
        }
    }

    /// Once a frame: the pattern played over the song follows the selected
    /// one, and ends with the song, as it does in the audio thread when
    /// playback stops or the mode changes.
    fn keep_over_song(&mut self) {
        match self.over_song {
            Some(_) if !self.playing || self.mode != PlayMode::Song => self.over_song = None,
            Some(pattern) if pattern != self.pattern => self.play_over_song(true),
            _ => {}
        }
    }

    /// Stops where the playhead is, so Play carries on from there.
    fn pause(&mut self) {
        let here = self.playhead().unwrap_or(self.cursor);
        self.halt();
        self.cursor = here;
        self.send(Command::Seek(here));
    }

    /// Stops the audio thread, finishing any recording first.
    fn halt(&mut self) {
        if self.playing {
            // Before the audio thread forgets where it was.
            self.finish_recording();
        }
        self.playing = false;
        self.send(Command::Stop);
    }

    /// How long the loop being played is: the selected pattern, or the
    /// song (one bar of 4/4 when it is empty, as the engine plays it).
    fn loop_length(&self) -> usize {
        let length = match self.mode {
            PlayMode::Pattern => self.project.patterns()[self.pattern].length(),
            PlayMode::Song => self.project.playback_arrangement().length,
        };
        if length == 0 {
            lineup_core::STEPS_PER_BAR
        } else {
            length
        }
    }

    /// Moves the playhead to `step`, which also becomes the marker that
    /// Stop goes back to. While playing, playback jumps there. A step past
    /// the end of the loop goes back to the start.
    fn seek(&mut self, step: usize) {
        let step = if step >= self.loop_length() { 0 } else { step };
        self.marker = step;
        self.cursor = step;
        self.send(Command::Seek(step));
        match self.mode {
            PlayMode::Pattern => self.piano_roll.reveal_step(step),
            PlayMode::Song => {
                let song = self.project.playback_arrangement();
                let step = song
                    .pass_at(step)
                    .map_or(step, |pass| pass.from + step - pass.start);
                self.playlist.reveal_step(step);
                self.chord_sheet.reveal_step(step);
                if let Some(step) = self.project.pattern_step_in_song(self.pattern, step) {
                    self.piano_roll.reveal_step(step);
                }
            }
        }
    }

    /// Moves the playhead by `bars` bars (back if negative), to the start
    /// of a bar.
    fn skip_bars(&mut self, bars: isize) {
        let meter = self.meter();
        let here = self.playhead().unwrap_or(self.cursor);
        let bar = meter.bar_at(here);
        let number = bar.number as isize;
        // Going back from inside a bar first goes to its start. While
        // playing, the playhead is never exactly on a bar line, so within
        // the first beat of a bar it counts as being at its start: two
        // quick presses go back two bars, as on a tape machine.
        let into_bar = here - bar.start;
        let at_start = into_bar == 0 || (self.playing && into_bar < bar.time.first_beat());
        let target = if bars < 0 && !at_start {
            number + bars + 1
        } else {
            number + bars
        };
        if target < 0 {
            self.seek(0);
        } else {
            self.seek(meter.bar(target as usize).start.min(self.loop_length() - 1));
        }
    }

    /// Where the bars are in what is playing: the song as it plays, with
    /// every change of time signature, or the selected pattern, in the
    /// time signature it is used in.
    fn meter(&self) -> Meter {
        match self.mode {
            PlayMode::Pattern => Meter::steady(self.project.pattern_time(self.pattern)),
            PlayMode::Song => self
                .project
                .played_meter(&self.project.playback_arrangement()),
        }
    }

    fn set_mode(&mut self, mode: PlayMode) {
        if mode != self.mode {
            self.finish_recording();
            self.mode = mode;
            self.cursor = 0;
            self.marker = 0;
            self.send(Command::SetMode(mode));
        }
    }

    /// Opens the editor that suits a pattern, as a double-click on one of
    /// its clips does: the piano roll when it has melodies or chords, on
    /// the channel that plays them, and the channel rack for drums.
    fn edit_clip(&mut self, pattern: usize) {
        self.select_pattern(pattern);
        let Some(notes) = self.project.pattern(pattern) else {
            return;
        };
        let current = self.piano_roll.channel();
        let melodic = |channel| notes.notes(channel).iter().any(|note| !note.is_step());
        let channel = if melodic(current) {
            Some(current)
        } else {
            piano_roll::busiest_channel(notes)
        };
        match channel {
            Some(channel) => {
                self.piano_roll.open_for(channel);
                self.bring_to_front = Some(PianoRoll::id());
            }
            None => self.show_rack(),
        }
    }

    /// Opens the channel rack and brings it in front of the other windows.
    fn show_rack(&mut self) {
        self.rack_open = true;
        self.bring_to_front = Some(rack_id());
    }

    fn select_pattern(&mut self, pattern: usize) {
        if pattern < self.project.patterns().len() {
            if pattern != self.pattern {
                self.rewind_for_new_pattern();
            }
            self.pattern = pattern;
            self.send(Command::SelectPattern(pattern));
            // Sent even for the same number: after patterns moved, it
            // may be another pattern now.
            if self.over_song.is_some() {
                self.play_over_song(true);
            }
        }
    }

    /// In pattern mode, the cursor and the marker are steps of the
    /// selected pattern. When another one is selected, they go back to its
    /// start, so Play does not begin part way through it. While playing,
    /// playback carries on where it is, and only Stop goes back to the
    /// start.
    fn rewind_for_new_pattern(&mut self) {
        if self.mode != PlayMode::Pattern {
            return;
        }
        self.marker = 0;
        if !self.playing {
            self.cursor = 0;
            self.send(Command::Seek(0));
        }
    }

    // Adding, cloning and removing patterns only change the project. The
    // edit tells the audio thread, and selects the new pattern (see
    // `sync_audio` in `edit.rs`).

    fn add_pattern(&mut self) {
        self.edit("add pattern", |project| project.add_pattern());
    }

    fn clone_pattern(&mut self) {
        let index = self.pattern;
        self.edit("clone pattern", |project| project.clone_pattern(index));
    }

    fn remove_pattern(&mut self) {
        let index = self.pattern;
        self.edit("delete pattern", |project| project.remove_pattern(index));
    }

    /// Applies one action from the channel rack.
    fn apply(&mut self, action: RackAction) {
        match action {
            RackAction::PlayPattern => {
                self.set_mode(PlayMode::Pattern);
                self.set_playing(true);
            }
            RackAction::PausePattern => {
                if self.mode == PlayMode::Pattern {
                    self.pause();
                }
            }
            RackAction::StopPattern => {
                if self.mode == PlayMode::Pattern {
                    self.set_playing(false);
                } else {
                    self.play_over_song(false);
                }
            }
            RackAction::PlayOverSong(on) => self.play_over_song(on),
            RackAction::SelectChannel(channel) => self.select_channel(channel),
            RackAction::ToggleStep { channel, step } => {
                self.edit_pattern("change steps", |pattern| {
                    pattern.toggle(channel, step);
                });
            }
            RackAction::ClearStep { channel, step } => {
                self.edit_pattern("change steps", |pattern| {
                    pattern.set(channel, step, false);
                });
            }
            RackAction::Preview(channel) => {
                self.send(Command::Preview(channel));
            }
            RackAction::SetVolume { channel, volume } => {
                self.edit("change volume", |project| {
                    if let Some(state) = project.channel_mut(channel) {
                        state.volume = volume;
                    }
                });
            }
            RackAction::ToggleMute(channel) => {
                self.edit("mute channel", |project| {
                    if let Some(state) = project.channel_mut(channel) {
                        state.muted = !state.muted;
                    }
                });
            }
            RackAction::SetLocked { channel, locked } => self.set_locked(channel, locked),
            RackAction::RemoveChosen => {
                if let Some(channel) = self.piano_roll.chosen_channel() {
                    self.apply(RackAction::Remove(channel));
                }
            }
            RackAction::Remove(channel) => {
                self.finish_recording();
                self.release_all_keys();
                self.edit("delete channel", |project| project.remove_channel(channel));
            }
            RackAction::OpenPianoRoll(channel) => {
                self.select_channel(Some(channel));
                self.piano_roll.open_for(channel);
                self.bring_to_front = Some(PianoRoll::id());
            }
            RackAction::SelectPattern(pattern) => self.select_pattern(pattern),
            RackAction::AddDrum(drum) => self.add_channel(Channel::drum(drum)),
            RackAction::AddSynth => self.add_channel(Channel::synth()),
            RackAction::AddMidiOut => self.add_channel(Channel::midi_out(0)),
            RackAction::SetMidiChannel {
                channel,
                midi_channel,
            } => {
                self.edit("change MIDI channel", |project| {
                    if let Some(state) = project.channel_mut(channel) {
                        // A name nobody changed follows the channel number.
                        if let ChannelKind::MidiOut { channel: old } = state.kind
                            && state.name == Channel::midi_out(old).name
                        {
                            state.name = Channel::midi_out(midi_channel).name;
                        }
                        state.kind = ChannelKind::MidiOut {
                            channel: midi_channel,
                        };
                    }
                });
            }
            RackAction::SetProgram { channel, program } => {
                self.edit("change instrument", |project| {
                    if let Some(state) = project.channel_mut(channel) {
                        state.program = program;
                    }
                });
            }
            RackAction::LoadSample { channel, path } => self.load_sample(channel, path),
        }
    }

    fn select_channel(&mut self, channel: Option<usize>) {
        let channel = channel.filter(|&index| index < self.project.channels().len());
        if channel != self.piano_roll.selected_channel() {
            self.finish_recording();
            self.release_all_keys();
            match channel {
                Some(channel) => self.piano_roll.select(channel),
                None => self.piano_roll.deselect_channel(),
            }
            self.send(Command::SetLiveChannel(channel));
            self.live.resend_channel();
        }
    }

    /// Applies one action from the piano roll.
    fn apply_roll(&mut self, action: RollAction) {
        match action {
            RollAction::SetLength(length) => {
                self.edit_pattern("extend pattern", |pattern| pattern.set_length(length))
            }
            RollAction::AddNotes { channel, notes } => {
                self.edit_pattern("add notes", |pattern| {
                    for note in notes {
                        pattern.add_note(channel, note);
                    }
                });
            }
            RollAction::RemoveNote { channel, index } => {
                self.edit_pattern("delete notes", |pattern| {
                    pattern.remove_note(channel, index);
                });
            }
            RollAction::ReplaceNote {
                channel,
                index,
                note,
            } => {
                self.edit_pattern("change notes", |pattern| {
                    pattern.replace_note(channel, index, note);
                });
            }
            RollAction::ReplaceNotes {
                channel,
                old,
                new,
                label,
            } => {
                self.edit_pattern(label, |pattern| {
                    pattern.replace_notes(channel, &old, &new);
                });
            }
            RollAction::Preview { channel, pitches } => {
                for pitch in pitches {
                    self.send(Command::PreviewNote { channel, pitch });
                }
            }
            RollAction::SetScale(scale) => {
                self.edit("change key", |project| project.change_key(scale))
            }
            RollAction::SelectPattern(pattern) => self.select_pattern(pattern),
            RollAction::NewPattern {
                channel,
                name,
                length,
                notes,
            } => {
                if self.project.patterns().len() >= lineup_core::MAX_PATTERNS {
                    self.message = Some("There is no room for another pattern.".into());
                    return;
                }
                let time = self.project.pattern_time(self.pattern);
                let added = self.edit("save row to new pattern", |project| {
                    let index = project.add_pattern()?;
                    let pattern = project.pattern_mut(index)?;
                    pattern.name = name;
                    pattern.time = Some(time);
                    pattern.set_length(length);
                    pattern.set_notes(channel, notes);
                    Some(index)
                });
                if let Some(pattern) = added {
                    self.select_pattern(pattern);
                } else {
                    self.message = Some("There is no room for another pattern.".into());
                }
            }
            RollAction::OpenRack => self.show_rack(),
        }
    }

    /// Applies one action from the harmony keys bar.
    fn apply_harmony(&mut self, action: HarmonyAction) {
        match action {
            HarmonyAction::SetScale(scale) => {
                self.edit("change key", |project| project.change_key(scale));
            }
            HarmonyAction::SetTime(time) => self.set_song_time(time),
            // The notes go the same way as the typing keyboard's: to the
            // audio thread, which plays them on the selected channel, and
            // to the recorder.
            HarmonyAction::NoteOn(notes) => {
                for pitch in notes {
                    self.play_typed(MidiMessage::NoteOn {
                        channel: 0,
                        pitch,
                        velocity: DEFAULT_VELOCITY,
                    });
                }
            }
            HarmonyAction::NoteOff(notes) => {
                for pitch in notes {
                    self.play_typed(MidiMessage::NoteOff { channel: 0, pitch });
                }
            }
            HarmonyAction::MarkHeardKeys => {
                let spans = self.song_keys_heard();
                self.edit("mark the keys heard", |project| {
                    project.mark_heard_keys(&spans);
                });
            }
        }
    }

    /// Applies one action from the chord sheet.
    fn apply_sheet(&mut self, action: SheetAction) {
        match action {
            SheetAction::UserStyle {
                index,
                progression_from,
            } => self.apply_user_library(index, progression_from),
            SheetAction::SetChords {
                start,
                end,
                markers,
            } => {
                self.edit("write chords", |project| {
                    project.set_chords(start, end, &markers)
                });
            }
            SheetAction::WriteHeardChords => {
                let written = self.edit("name the song's chords", |project| {
                    project.write_heard_chords()
                });
                if written == 0 {
                    self.message = Some(
                        "No chords to name: the song plays no chords yet. Type them in the \
                         bars instead."
                            .to_string(),
                    );
                } else {
                    self.notice = Some(format!(
                        "Wrote {written} chord{} on the chord sheet",
                        if written == 1 { "" } else { "s" }
                    ));
                }
            }
            SheetAction::SetSwing(swing) => self.set_swing(swing),
            SheetAction::BandVoices(style) => {
                self.edit("change band voices", |project| {
                    project.apply_band_parts(
                        style,
                        None,
                        lineup_core::library::apply::StyleParts {
                            rhythm: false,
                            instruments: true,
                            time_signature: false,
                            progression: false,
                            tempo: false,
                            swing: false,
                            matching_style: true,
                        },
                    );
                });
            }
            SheetAction::BandPlayer { player, on } => {
                self.edit("change band players", |project| {
                    project.set_you_play(player, !on)
                });
            }
            SheetAction::BandProgram { channel, program } => {
                self.apply(RackAction::SetProgram { channel, program });
            }
            SheetAction::SetBand { style, harmony } => {
                let had = self.project.band().is_some();
                let before = self.project.clone();
                let previous = self.tried();
                let swing = style.and_then(|style| self.swings.of(style.name));
                let parts = if harmony.is_some() {
                    self.playlist.library_parts()
                } else {
                    self.playlist.library_parts().for_rhythm()
                };
                // Taking the band away is your edit; a style brings it in.
                let source = style.map(|style| match harmony {
                    Some(harmony) => Source::new(SourceKind::Style, harmony.name),
                    None => Source::new(SourceKind::Rhythm, style.name),
                });
                self.edit_song(
                    "change the band",
                    style
                        .filter(|_| had)
                        .map(|style| harmony.map_or(style.time, |harmony| harmony.time)),
                    parts,
                    false,
                    source.as_ref().map(library::Bringing::from),
                    |project, parts| {
                        match style {
                            Some(style) if project.band().is_some() => {
                                project.apply_band_parts(style, harmony, parts)
                            }
                            _ => {
                                project.set_band_style(style);
                                if let (Some(style), Some(harmony)) = (style, harmony) {
                                    project.apply_band_parts(style, Some(harmony), parts);
                                }
                            }
                        }
                        // The swing this style was last played with.
                        if parts.swing
                            && let Some(swing) = swing
                        {
                            project.swing = swing;
                        }
                    },
                );
                self.song_library = self
                    .project
                    .band()
                    .map_or_else(Tried::default, Tried::from_band);
                self.keep_place_in_song(&before);
                self.remember_song_version(before, previous);
                self.remember_song_version(self.project.clone(), self.song_library);
                self.notice = match (style, self.project.band()) {
                    (Some(style), Some(band)) if !had && band.muted() != &Default::default() => {
                        // The channels muted, then the drum tracks.
                        let muted: Vec<String> = [&band.muted().channels, &band.muted().tracks]
                            .into_iter()
                            .flatten()
                            .cloned()
                            .collect();
                        let names = match muted.split_last() {
                            Some((last, [])) => last.clone(),
                            Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
                            None => String::new(),
                        };
                        Some(format!(
                            "The band plays {}; the song's own {names} {} muted.{}",
                            harmony.map_or(style.name, |style| style.name),
                            if muted.len() == 1 { "is" } else { "are" },
                            library::kept(&self.project)
                        ))
                    }
                    (Some(style), _) => Some(format!(
                        "Applied selected parts of {}.{}",
                        harmony.map_or(style.name, |style| style.name),
                        library::kept(&self.project)
                    )),
                    (None, _) => Some(format!("No band.{}", library::kept(&self.project))),
                };
                if style.is_some() {
                    self.set_mode(PlayMode::Song);
                }
            }
            SheetAction::PlayAgain(player) => {
                self.edit("play the part again", |project| project.play_again(player));
            }
            _ if matches!(
                action,
                SheetAction::SetSoloist(_) | SheetAction::AnotherSolo
            ) && self.project.plays_locked(lineup_core::Player::Soloist) =>
            {
                self.notice =
                    Some("The soloist is locked: unlock it in the rack first.".to_string());
            }
            SheetAction::SetSoloist(on) => {
                let what = if on {
                    "add the soloist"
                } else {
                    "take the soloist out"
                };
                self.edit(what, |project| project.set_soloist(on));
                self.notice = Some(match on {
                    true => "The soloist improvises over the chorus".to_string(),
                    false => "No soloist".to_string(),
                });
            }
            SheetAction::AnotherSolo => {
                self.edit("play another solo", Project::another_solo);
                self.notice = Some("The soloist plays another solo".to_string());
            }
            SheetAction::WriteProgression { style, from } => {
                let source = Source::new(SourceKind::Style, style.name);
                let chords = PartSet::of(&[SongPart::Chords]);
                let bars = self.edit("write a progression", |project| {
                    project.bring_in(&source, chords, PartSet::NONE, |project| {
                        project.write_progression(style, from)
                    })
                });
                self.notice = Some(format!("Wrote {} from bar {}", style.name, from + 1));
                if bars == 0 {
                    self.message = Some("There is no room for it on the sheet.".to_string());
                } else {
                    // The next progression goes on after this one.
                    self.chord_sheet.select(from + bars);
                }
            }
            SheetAction::EndHere(bar) => {
                self.edit("end the song", |project| project.end_song_at(bar));
            }
            SheetAction::Seek(step) => self.apply_playlist(PlaylistAction::Seek(step)),
            SheetAction::NoteOn(notes) => self.apply_harmony(HarmonyAction::NoteOn(notes)),
            SheetAction::NoteOff(notes) => self.apply_harmony(HarmonyAction::NoteOff(notes)),
            SheetAction::Practice(action) => self.apply_practice(action),
            SheetAction::Form(action) => self.apply_form(action),
        }
    }

    /// Applies one action from the playlist.
    fn apply_playlist(&mut self, action: PlaylistAction) {
        match action {
            PlaylistAction::SelectPattern(pattern) => self.select_pattern(pattern),
            PlaylistAction::SelectSection(region) => {
                self.playlist.select_section(&self.project, region)
            }
            PlaylistAction::MoveClips {
                indices,
                tracks,
                steps,
            } => {
                self.edit("move selected clips", |project| {
                    project.move_clips(&indices, tracks, steps);
                });
            }
            PlaylistAction::AddClip(clip) => {
                self.edit("add clip", |project| project.add_clip(clip));
            }
            PlaylistAction::MoveClip {
                index,
                track,
                start,
            } => {
                self.edit("move clip", |project| {
                    project.move_clip(index, track, start)
                });
            }
            PlaylistAction::EditClip(pattern) => self.edit_clip(pattern),
            PlaylistAction::RemoveClip(index) => {
                self.edit("delete clips", |project| project.remove_clip(index));
            }
            PlaylistAction::RemoveClips(indices) => {
                self.edit("delete selected clips", |project| {
                    for index in indices.into_iter().rev() {
                        project.remove_clip(index);
                    }
                });
            }
            PlaylistAction::PasteClips(clips) => {
                self.edit("paste clips", |project| {
                    for clip in clips {
                        project.add_clip(clip);
                    }
                });
            }
            PlaylistAction::ToggleTrackMute(track) => {
                self.edit("mute track", |project| {
                    if let Some(track) = project.track_mut(track) {
                        track.muted = !track.muted;
                    }
                });
            }
            PlaylistAction::RenameTrack { track, name } => {
                self.edit("rename track", |project| {
                    if let Some(track) = project.track_mut(track) {
                        track.name = name;
                    }
                });
            }
            PlaylistAction::ResizeTrack { track, height } => {
                self.edit("resize track", |project| {
                    if let Some(track) = project.track_mut(track) {
                        let height = height.clamp(24, 240);
                        track.height = (height != 36).then_some(height);
                    }
                });
            }
            PlaylistAction::AddTrack => {
                self.edit("add track", |project| project.add_track());
            }
            PlaylistAction::InsertTrack(index) => {
                self.edit("insert track", |project| project.insert_track(index));
            }
            PlaylistAction::RemoveTrack(index) => {
                self.edit("delete track", |project| project.remove_track(index));
            }
            PlaylistAction::SwapTracks(index) => {
                self.edit("move track", |project| project.swap_tracks(index));
            }
            PlaylistAction::Seek(step) => {
                self.set_mode(PlayMode::Song);
                // The playlist's timeline counts a repeated stretch once;
                // the audio thread counts every time round.
                let song = self.project.playback_arrangement();
                let step = song
                    .passes
                    .iter()
                    .find(|pass| (pass.from..pass.from + pass.length).contains(&step))
                    .map_or(0, |pass| pass.start + (step - pass.from));
                self.seek(step);
            }
            PlaylistAction::SetClipRepeats { index, repeats } => {
                self.edit("repeat clip", |project| {
                    project.set_clip_repeats(index, repeats)
                });
            }
            PlaylistAction::AddSection(section) => {
                self.edit("add section", |project| project.add_section(section));
            }
            PlaylistAction::SetSection { index, section } => {
                self.edit("change section", |project| {
                    project.set_section(index, section)
                });
            }
            PlaylistAction::RemoveSection(index) => {
                self.edit("delete section", |project| project.remove_section(index));
            }
            PlaylistAction::DuplicateSection(index) => {
                self.edit("duplicate section", |project| {
                    project.duplicate_section(index, 1)
                });
            }
            PlaylistAction::UnrollSection(index) => {
                self.edit("write out repeats", |project| project.unroll_section(index));
            }
            // Back to the whole song from the same place in it, as the
            // practice bar's loop does.
            PlaylistAction::SetLoop(None) => self.loop_bars(None),
            PlaylistAction::SetLoop(region) => {
                self.edit("set loop", |project| project.set_loop_region(region));
            }
            // Bar 1 is the time signature the song starts in, as in the
            // Time box.
            PlaylistAction::SetTimeMarker(marker) if marker.start == 0 => {
                self.set_song_time(marker.time);
            }
            PlaylistAction::SetTimeMarker(marker) => {
                self.edit("change time signature", |project| {
                    project.set_time_marker(marker)
                });
            }
            PlaylistAction::RemoveTimeMarker(start) => {
                self.edit("remove time signature change", |project| {
                    project.remove_time_marker(start)
                });
            }
            PlaylistAction::SetKeyMarker(marker) => {
                self.edit("change key from a bar", |project| {
                    project.set_key_marker(marker)
                });
            }
            PlaylistAction::RemoveKeyMarker(start) => {
                self.edit("remove key change", |project| {
                    project.remove_key_marker(start)
                });
            }
            PlaylistAction::SetTempoMarker(marker) => {
                self.edit("change tempo", |project| project.set_tempo_marker(marker));
            }
            PlaylistAction::SetLooseTiming(ms) => {
                self.edit("change loose timing", |project| project.loose_timing = ms);
            }
            PlaylistAction::SetSwing(swing) => self.set_swing(swing),
            PlaylistAction::RemoveTempoMarker(start) => {
                self.edit("remove tempo change", |project| {
                    project.remove_tempo_marker(start)
                });
            }
            PlaylistAction::Library(action) => self.try_from_library(action),
        }
    }

    fn add_channel(&mut self, channel: Channel) {
        if let Some(channel) = self.edit("add channel", |project| project.add_channel(channel)) {
            self.select_channel(Some(channel));
        }
    }

    /// Loads a sample file into channel `channel`, or into a new channel.
    fn load_sample(&mut self, channel: Option<usize>, path: PathBuf) {
        // Read the file now, to report a bad one before changing anything.
        // It stays in the cache, so building the instrument does not
        // read it again. `reload` because the file may have changed since
        // it was last dropped.
        if let Err(err) = self.samples.reload(&path) {
            self.message = Some(format!("{}: {err}", path.display()));
            return;
        }
        let kind = ChannelKind::Sampler(SampleSource::File(path.clone()));
        let name = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Sample".to_string());

        let loaded = self.edit("load sample", |project| {
            match channel.filter(|&index| index < project.channels().len()) {
                Some(index) => {
                    let existing = project.channel_mut(index).unwrap();
                    existing.name = name;
                    existing.kind = kind;
                    Some(index)
                }
                None => project.add_channel(Channel {
                    name,
                    kind,
                    volume: 0.8,
                    muted: false,
                    program: None,
                }),
            }
        });
        if let Some(channel) = loaded {
            self.select_channel(Some(channel));
        }
        self.message = None;
    }

    /// The menu bar along the very top, with the hint bar beside it.
    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.file_menu(ui);
            self.edit_menu(ui);
            self.view_menu(ui);
            self.midi_menu(ui);
            self.help_menu(ui);
            ui.separator();
            self.hint_bar.show(ui);
        });
    }

    /// The toolbar under the menus: the transport, the tempo and song
    /// position displays, the pattern selector, buttons for the windows
    /// and the master volume, each in its own group. When the window is
    /// too narrow for all of it (or zoomed in), the groups wrap onto a
    /// second row instead of running off the edge.
    fn transport_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            theme::group(ui, |ui| {
                let mut mode = self.mode;
                let pattern = theme::light_button(ui, "PAT", mode == PlayMode::Pattern, None);
                if theme::hint(pattern, "Pattern mode: loop the selected pattern (L)").clicked() {
                    mode = PlayMode::Pattern;
                }
                let song =
                    theme::light_button(ui, "SONG", mode == PlayMode::Song, Some(theme::GREEN));
                if theme::hint(song, "Song mode: play the playlist (L)").clicked() {
                    mode = PlayMode::Song;
                }
                self.set_mode(mode);
            });

            theme::group(ui, |ui| {
                let start = theme::light_button(ui, "⏮", false, None);
                if theme::hint(start, "Go to the start (Home)").clicked() {
                    self.seek(0);
                }
                let back = theme::light_button(ui, "⏪", false, None);
                if theme::hint(back, "Back one bar (Page Up)").clicked() {
                    self.skip_bars(-1);
                }
                let (icon, text) = if self.playing {
                    ("❚❚", "Pause: stop here, and carry on from here next time")
                } else {
                    ("▶", "Play from the playhead (Space)")
                };
                let play = theme::light_button(ui, icon, self.playing, Some(theme::GREEN));
                if theme::hint(play, text).clicked() {
                    if self.playing {
                        self.pause();
                    } else {
                        self.set_playing(true);
                    }
                }
                let stop = theme::light_button(ui, "⏹", false, None);
                if theme::hint(
                    stop,
                    "Stop and go back to where you last put the playhead (Space)",
                )
                .clicked()
                {
                    self.set_playing(false);
                }
                let forward = theme::light_button(ui, "⏩", false, None);
                if theme::hint(forward, "Forward one bar (Page Down)").clicked() {
                    self.skip_bars(1);
                }
                self.record_button(ui);
            });

            theme::group(ui, |ui| self.playback_controls(ui));

            // While a song whose tempo changes plays, the box follows the
            // tempo at the playhead; stopped, it is
            // the tempo the song starts at, to change.
            let map = self.project.tempo_map();
            let now = if map.is_steady() || !self.playing {
                None
            } else {
                self.song_position()
                    .map(|(step, _)| map.tempo_at(step).bpm())
            };
            // At a practice speed, the box still shows the song's own
            // tempo, and says how fast it is played.
            let speed = self.practice.view().speed;
            let (at_speed, width, slower) = if speed == practice::FULL_SPEED {
                (String::new(), 62.0, String::new())
            } else {
                (
                    format!(" {speed}%"),
                    80.0,
                    format!(". You practise at {speed}% of it (Settings > Playback)"),
                )
            };
            if let Some(now) = now {
                let label = format!("TEMPO NOW{at_speed}");
                let lcd = theme::lcd(ui, &label, &format!("{now:.1}"), width);
                theme::hint(
                    lcd,
                    format!(
                        "The tempo playing now: this song changes tempo (see the tempo strip \
                         at the top of the playlist). Stop to change the tempo it starts at; \
                         every change moves with it{slower}"
                    ),
                );
            } else {
                let mut bpm = self.project.tempo.bpm();
                let label = format!("TEMPO{at_speed}");
                let tempo = theme::lcd_frame(ui, &label, width, |ui| {
                    ui.add(
                        egui::DragValue::new(&mut bpm)
                            .range(Tempo::MIN..=Tempo::MAX)
                            .speed(0.5)
                            .fixed_decimals(1),
                    )
                });
                let text = if map.is_steady() {
                    "Tempo in beats a minute. Drag up or down, or double-click to type"
                } else {
                    "The tempo the song starts at, in beats a minute. This song changes \
                     tempo (see the tempo strip at the top of the playlist), and every \
                     change moves with this one. Drag up or down, or double-click to type"
                };
                let text = format!("{text}{slower}");
                if theme::hint(tempo, text).changed() {
                    self.edit("change tempo", |project| {
                        project.scale_tempo(Tempo::new(bpm))
                    });
                }
            }
            let position = theme::lcd(ui, "BAR : BEAT : STEP", &self.position_text(), 92.0);
            theme::hint(
                position,
                "Where the playhead is: in the pattern (PAT) or the song (SONG). Click the \
             playlist's bar numbers to move it",
            );

            theme::group(ui, |ui| self.pattern_selector(ui));

            theme::group(ui, |ui| {
                let sheet = theme::light_button(ui, "📝 Chords", self.chord_sheet.open, None);
                if theme::hint(
                    sheet,
                    "Show or hide the chord sheet: the song's harmony bar by bar, to read and \
                     write (F5)",
                )
                .clicked()
                {
                    self.chord_sheet.open = !self.chord_sheet.open;
                }
                let rack = theme::light_button(ui, "▦ Rack", self.rack_open, None);
                if theme::hint(rack, "Show or hide the channel rack: the steps (F6)").clicked() {
                    self.rack_open = !self.rack_open;
                }
                let roll = theme::light_button(ui, "🎹 Piano roll", self.piano_roll.open, None);
                if theme::hint(
                    roll,
                    "Show or hide the piano roll: melodies and chords (F7)",
                )
                .clicked()
                {
                    self.piano_roll.open = !self.piano_roll.open;
                }
                if theme::light_button(ui, "Mixer", self.mixer_open, None)
                    .on_hover_text("Master and instrument levels, solo, mute and reverb (F9)")
                    .clicked()
                {
                    self.mixer_open = !self.mixer_open;
                }
                let harmony = theme::light_button(ui, "🎼 Harmony", self.harmony_keys.open, None);
                if theme::hint(
                    harmony,
                    "Show or hide the harmony keys: the key, the time signature and a key \
                     for each chord (F8)",
                )
                .clicked()
                {
                    self.harmony_keys.open = !self.harmony_keys.open;
                }
            });

            theme::group(ui, |ui| {
                // The label goes first, as its own widget, so the slider
                // and its name wrap to the next row together.
                ui.label(egui::RichText::new("VOL").small().color(theme::TEXT_DIM));
                // Shorter than egui's usual slider, so the toolbar fits
                // on one row in a window 1280 points wide.
                ui.spacing_mut().slider_width = 64.0;
                let mut master = self.project.mixer.master;
                let volume_slider = egui::Slider::new(&mut master, 0.0..=1.0).show_value(false);
                let volume = ui.add_sized([64.0, 22.0], volume_slider);
                let text = format!("Master volume: {:.0}%", master * 100.0);
                if theme::hint(volume, text).changed() {
                    self.set_master_volume(master);
                }
            });
        });
    }

    /// The playhead as bar, beat and step, all counted from 1, for the
    /// song position display. Stopped, it shows the start.
    fn position_text(&self) -> String {
        let step = self.playhead().unwrap_or(self.cursor);
        let (bar, beat, sub) = self.meter().position(step);
        // A loop plays from its own start, so its bars are counted from
        // the bar it starts on, as the ruler numbers them.
        let first = match (self.mode, self.project.playlist().loop_region()) {
            (PlayMode::Song, Some(region)) => self.project.meter().bar_at(region.start).number,
            _ => 0,
        };
        let bar = bar + first;
        format!("{bar:>3}:{beat:02}:{sub:02}")
    }

    /// Choosing, adding, cloning, renaming and deleting patterns, and the
    /// selected pattern's length.
    fn pattern_selector(&mut self, ui: &mut egui::Ui) {
        let current = self.pattern;
        let mut selected = current;
        let name = self.project.patterns()[current].name.clone();
        ui.label(
            egui::RichText::new("PATTERN")
                .small()
                .color(theme::TEXT_DIM),
        );
        // A long name is cut short with "…" rather than widening the
        // toolbar onto a second row; the hint gives it whole. `width` is
        // only the narrowest the list can be, so it is given a space no
        // wider than that to fit in.
        let room = egui::vec2(PATTERN_LIST_WIDTH, ui.spacing().interact_size.y);
        let combo = ui
            .allocate_ui(room, |ui| {
                egui::ComboBox::from_id_salt("pattern")
                    .width(PATTERN_LIST_WIDTH)
                    .truncate()
                    .selected_text(
                        egui::RichText::new(&name)
                            .color(playlist::pattern_color(&self.project, current)),
                    )
                    .show_ui(ui, |ui| {
                        for (index, pattern) in self.project.patterns().iter().enumerate() {
                            let text = egui::RichText::new(&pattern.name)
                                .color(playlist::pattern_color(&self.project, index));
                            ui.selectable_value(&mut selected, index, text);
                        }
                    })
                    .response
            })
            .inner;
        theme::hint(
            combo,
            format!(
                "The selected pattern, \"{name}\": the channel rack and the piano roll show \
                 it, and new clips in the playlist use it"
            ),
        );
        if selected != current {
            self.select_pattern(selected);
        }

        if theme::hint(ui.button("➕"), "Add a new, empty pattern").clicked() {
            self.add_pattern();
        }
        // Menus normally close on any click. This one has a text field, so
        // it stays open until you click somewhere else.
        let stay_open = egui::containers::menu::MenuConfig::new()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
        let menu = egui::containers::menu::MenuButton::new("⋯").config(stay_open);
        let (menu, _) = menu.ui(ui, |ui| {
            ui.label("Name");
            // The field edits a copy, so the change can go through `edit`.
            let mut name = self.project.patterns()[self.pattern].name.clone();
            if ui.text_edit_singleline(&mut name).changed() {
                self.edit_pattern("rename pattern", |pattern| pattern.name = name);
            }
            if ui.button("Clone").clicked() {
                self.clone_pattern();
                ui.close();
            }
            let can_delete = self.project.patterns().len() > 1;
            if ui
                .add_enabled(can_delete, egui::Button::new("Delete"))
                .on_hover_text("Also removes its clips from the playlist")
                .clicked()
            {
                self.remove_pattern();
                ui.close();
            }
        });

        theme::hint(menu, "Rename, clone or delete the selected pattern");

        let length = self.project.patterns()[self.pattern].length();
        let mut new_length = length;
        let time = self.project.pattern_time(self.pattern);
        // As narrow as "16 bars" needs, so the toolbar fits on one row.
        let length_combo = egui::ComboBox::from_id_salt("pattern-length")
            .width(64.0)
            .selected_text(time.describe_length(length))
            .show_ui(ui, |ui| {
                for option in time.pattern_lengths() {
                    ui.selectable_value(&mut new_length, option, time.describe_length(option));
                }
            })
            .response;
        theme::hint(
            length_combo,
            format!("How long the selected pattern is: {length} steps"),
        );
        if new_length != length {
            self.edit_pattern("change pattern length", |pattern| {
                pattern.set_length(new_length);
            });
        }
    }

    /// The bar along the bottom: messages on the left, zoom on the right.
    fn status_bar(&self, ui: &mut egui::Ui) {
        // The zoom control goes in first, from the right, so it always
        // has room; the messages get what is left, cut short with "…".
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            view::zoom_control(ui);
            ui.separator();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                self.status_messages(ui);
            });
        });
    }

    /// The audio output and format, shared by Settings and the status bar.
    fn audio_status(&self, ui: &mut egui::Ui) {
        match &self.audio {
            Audio::Running(engine) => {
                let info = engine.info();
                ui.label(format!(
                    "Audio: {} · {} Hz · {} channels",
                    info.device_name, info.sample_rate, info.channels
                ));
            }
            Audio::Unavailable(reason) => {
                ui.colored_label(
                    ui.visuals().warn_fg_color,
                    format!("Audio unavailable: {reason}"),
                );
            }
        }
    }

    /// Which sound card is in use, the chord held, an export running, and
    /// the last piece of news or problem.
    fn status_messages(&self, ui: &mut egui::Ui) {
        self.audio_status(ui);
        if let Some(chord) = self.held_chord() {
            ui.separator();
            ui.label(chord);
        }
        if let Some(export) = &self.export {
            ui.separator();
            ui.add(
                egui::ProgressBar::new(export.progress())
                    .desired_width(160.0)
                    .text(format!("Exporting {}", export.file_name())),
            );
            if export.can_cancel() && ui.small_button("Stop").clicked() {
                export.cancel();
            }
        }
        self.sounds_status(ui);
        if let Some(notice) = &self.notice {
            ui.separator();
            ui.label(notice);
        }
        if let Some(message) = &self.message {
            ui.separator();
            ui.colored_label(ui.visuals().error_fg_color, message);
        }
    }

    /// The step the sequencer is on, or `None` when stopped. In pattern
    /// mode it is a step of the selected pattern; in song mode, of the song.
    fn playhead(&self) -> Option<usize> {
        match &self.audio {
            Audio::Running(engine) if self.playing => engine.playhead(),
            _ => None,
        }
    }

    /// The step of the selected pattern being played, for the rack and the
    /// piano roll. In song mode that depends on which clip is playing.
    fn pattern_playhead(&self) -> Option<usize> {
        let step = self.playhead()?;
        match self.mode {
            PlayMode::Pattern => Some(step),
            PlayMode::Song => {
                if let Audio::Running(engine) = &self.audio
                    && self.over_song.is_some()
                    && let Some((pattern, step)) = engine.overlay()
                    && pattern == self.pattern
                {
                    return Some(step);
                }
                let (step, _) = self.song_position()?;
                self.project.pattern_step_in_song(self.pattern, step)
            }
        }
    }

    /// The chord notes playing at the playhead, for the harmony keys.
    fn chord_playing(&self) -> Vec<Note> {
        match (self.playhead(), self.mode) {
            (Some(step), PlayMode::Pattern) => {
                self.project.chord_notes_in_pattern(self.pattern, step)
            }
            (Some(_), PlayMode::Song) => match self.song_position() {
                Some((step, _)) => self.project.chord_notes_in_song(step),
                None => Vec::new(),
            },
            (None, _) => Vec::new(),
        }
    }

    /// Where the song is, in song mode: the step of the playlist's
    /// timeline being played (or where Play starts, while stopped), and which time round it is if it repeats.
    /// The audio thread counts steps of the song as played, so a chorus
    /// played twice counts twice; the arrangement turns that back into a
    /// place in the playlist.
    /// The key the music is in where you are: in song mode, the key at
    /// the playhead (or where Play will start), which a key change in
    /// the playlist moves on from; in pattern mode, the selected
    /// pattern's.
    fn key_here(&self) -> Scale {
        match self.song_position() {
            Some((step, _)) => self.project.key_at(step),
            None => self.project.pattern_key(self.pattern),
        }
    }

    fn song_position(&self) -> Option<(usize, Pass)> {
        if self.mode != PlayMode::Song {
            return None;
        }
        // While stopped, where Play will start.
        let step = self.playhead().unwrap_or(self.cursor);
        let song = self.project.playback_arrangement();
        let pass = *song.pass_at(step)?;
        Some((pass.from + (step - pass.start), pass))
    }

    fn keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        if widgets::modal_open(ctx) {
            return;
        }
        self.style_editor.shortcuts(ctx);
        // While a text field has the keyboard (a track or pattern name),
        // Space and L are letters, not shortcuts.
        let controls_active = [
            (self.settings_open, egui::Id::unique("application-settings")),
            (self.mixer_open, egui::Id::unique("mixer")),
        ]
        .iter()
        .any(|&(open, id)| {
            open && widgets::editor_active(ctx, egui::LayerId::new(egui::Order::Middle, id))
        });
        let typing = ctx.text_edit_focused() || egui::Popup::is_any_open(ctx);
        // `consume_key` reports whether the key was pressed and removes the
        // press, so a focused button does not also react to Space.
        let pressed = |key| ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
        if !typing && !controls_active && pressed(egui::Key::Space) {
            let pattern_editor = widgets::clicked_editor(ctx).is_some_and(|layer| {
                layer.order == egui::Order::Middle
                    && [rack_id(), PianoRoll::id()].contains(&layer.id)
            });
            // While the song plays, Space stops it, wherever you clicked;
            // once it has stopped, Space in the rack or the piano roll
            // plays the pattern.
            let song_playing = self.mode == PlayMode::Song && self.playing;
            if pattern_editor && !song_playing {
                if self.mode == PlayMode::Pattern && self.playing {
                    self.pause();
                } else {
                    self.apply(RackAction::PlayPattern);
                }
            } else {
                self.set_playing(!self.playing);
            }
        }
        if widgets::editor_keyboard(ctx, egui::LayerId::new(egui::Order::Middle, rack_id()))
            && pressed(egui::Key::Escape)
        {
            self.select_channel(None);
        }
        let boundary = |key| {
            ctx.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, key)) || pressed(key)
        };
        if !typing && !controls_active && boundary(egui::Key::Home) {
            self.seek(0);
        }
        if !typing && !controls_active && boundary(egui::Key::End) {
            self.seek(self.loop_length().saturating_sub(1));
        }
        if !typing && !controls_active && pressed(egui::Key::PageUp) {
            self.skip_bars(-1);
        }
        if !typing && !controls_active && pressed(egui::Key::PageDown) {
            self.skip_bars(1);
        }
        if !typing && !controls_active && pressed(egui::Key::L) {
            self.set_mode(match self.mode {
                PlayMode::Pattern => PlayMode::Song,
                PlayMode::Song => PlayMode::Pattern,
            });
        }
        self.view_shortcuts(ctx);
        self.zoom_shortcuts(ctx);
        self.file_shortcuts(ctx);
        if !typing {
            // While typing, Ctrl+Z belongs to the text field.
            self.edit_shortcuts(ctx);
        }
    }
}

impl eframe::App for LineupApp {
    /// Called by eframe every time the window needs to be redrawn.
    /// egui is "immediate mode": we describe the whole UI on every call.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Audio::Running(engine) = &mut self.audio {
            engine.collect_garbage();
        }
        self.check_trace();
        let ctx = ui.ctx().clone();
        // Before anything is drawn: whether a menu is open as the mouse
        // button goes down.
        widgets::track_dismissing(&ctx);
        self.start_frame_for_undo(&ctx);
        self.keyboard_shortcuts(&ctx);
        self.keep_over_song();
        self.live_input(&ctx);
        self.practice_frame(&ctx);
        self.handle_dropped_files(&ctx);
        self.check_export(&ctx);
        self.check_sounds(&ctx);
        self.check_import(&ctx);
        self.store_swings(&ctx);
        if self.playing {
            // Keep redrawing so the playhead moves.
            ctx.request_repaint();
        }

        self.hint_bar.start_frame(&ctx);
        let bar = |fill| {
            egui::Frame::new()
                .fill(fill)
                .inner_margin(egui::Margin::symmetric(6, 3))
                .stroke(egui::Stroke::new(1.0, theme::OUTLINE))
        };
        egui::Panel::top("menu-bar")
            .frame(bar(theme::BACKGROUND))
            .show(ui, |ui| self.menu_bar(ui));
        egui::Panel::top("transport")
            .frame(bar(theme::PANEL).inner_margin(egui::Margin::symmetric(6, 5)))
            .show(ui, |ui| self.transport_bar(ui));
        egui::Panel::bottom("status")
            .frame(bar(theme::BACKGROUND))
            .show(ui, |ui| self.status_bar(ui));
        // Added after the status bar, so it sits just above it.
        let harmony_actions = if self.harmony_keys.open {
            let seconds = ctx.input(|input| input.stable_dt).min(0.25);
            self.find_key_live(seconds);
            let key_heard = self.key_heard();
            let held = self.live.held_pitches();
            let channel = self.piano_roll.selected_channel();
            let channel_name = self
                .project
                .channels()
                .get(channel.unwrap_or(usize::MAX))
                .map_or("", |channel| channel.name.as_str());
            let heard = self.chord_playing();
            // While the chord sheet has the keyboard, digits are sets
            // and figures typed in a bar.
            let digits = !self.live.typing()
                && !ctx.text_edit_focused()
                && !self.chord_sheet.has_keyboard()
                && !widgets::modal_open(&ctx);
            let here = self.key_here();
            egui::Panel::bottom("harmony-keys")
                .frame(bar(theme::PANEL).inner_margin(egui::Margin::symmetric(8, 4)))
                .show(ui, |ui| {
                    self.harmony_keys.show(
                        ui,
                        &self.project,
                        here,
                        channel_name,
                        &held,
                        &heard,
                        digits,
                        key_heard.as_ref(),
                        self.playlist.library_parts_mut(),
                    )
                })
                .inner
        } else {
            self.harmony_keys.release()
        };
        for action in harmony_actions {
            self.apply_harmony(action);
        }

        let song_position = self.song_position();
        let tried = self.tried();
        let playlist_zoom = egui::vec2(
            view::Area::Main.zoom(&ctx, view::Axis::Horizontal),
            view::Area::Main.zoom(&ctx, view::Axis::Vertical),
        );
        let playlist = egui::CentralPanel::default_margins().show(ui, |ui| {
            self.playlist.show(
                ui,
                &self.project,
                self.pattern,
                song_position,
                tried,
                playlist_zoom,
            )
        });
        // The windows stay over the playlist, so they never hide the
        // toolbar, the harmony keys or the status bar.
        let area = playlist.response.rect;
        let playlist_actions = playlist.inner;
        for action in playlist_actions {
            self.apply_playlist(action);
        }

        // The channel rack and the piano roll float on top of the playlist.
        let playhead = self.pattern_playhead();
        let mut rack_open = self.rack_open;
        let pattern_name = &self.project.patterns()[self.pattern].name;
        if let Some(id) = self.bring_to_front.take() {
            let layer = egui::LayerId::new(egui::Order::Middle, id);
            ctx.move_to_top(layer);
            // Its keys go with it: Delete after double-clicking a clip
            // deletes notes in the piano roll, not clips in the playlist.
            widgets::give_keyboard(&ctx, layer);
        }
        // The rack opens as big as its rows, up to the size of the
        // playlist; it can be resized, and scrolls when made smaller.
        // When the rows grow or shrink (a longer pattern, a new channel),
        // it is fitted to them again, so no step hides past its edge.
        let rack_scale = Self::rack_scale(&ctx);
        let style = ctx.global_style();
        let natural = (channel_rack::natural_size(&style, &self.project, self.pattern, 1.0)
            + egui::vec2(
                0.0,
                style.spacing.interact_size.y + style.spacing.item_spacing.y,
            ))
            * rack_scale;
        // A window's default size is its outside: the contents, the frame
        // around them and the title bar.
        let style = ctx.global_style();
        let frame = style.spacing.window_margin.sum();
        let title = style.text_styles[&egui::TextStyle::Heading].size + 2.0 * frame.y;
        let rack_size =
            (natural + frame + egui::vec2(0.0, title)).min(area.size() - egui::vec2(40.0, 80.0));
        let mut rack = egui::Window::new(format!("Channel rack · {pattern_name}"))
            .id(rack_id())
            .open(&mut rack_open)
            .default_pos([240.0, 380.0])
            .default_size(rack_size)
            .constrain_to(area)
            .resizable(true);
        if natural != self.rack_natural {
            if self.rack_natural != egui::Vec2::ZERO {
                rack = rack.fixed_size(rack_size);
            }
            self.rack_natural = natural;
        }
        let rack_actions = rack
            .show(&ctx, |ui| {
                widgets::zoom_contents(ui, rack_scale);
                let selected = self.piano_roll.selected_channel();
                ui.horizontal(|ui| {
                    if ui
                        .button("Edit style…")
                        .on_hover_text("Edit a copy of the current style’s repeating parts")
                        .clicked()
                    {
                        self.edit_current_style();
                    }
                    if ui.button("New style…").clicked() {
                        self.start_style_editor(lineup_core::library::UserStyle::blank(), None);
                    }
                });
                channel_rack::show(
                    ui,
                    &self.project,
                    self.pattern,
                    selected,
                    playhead,
                    Some(channel_rack::RackPlayback {
                        playing: self.playing && self.mode == PlayMode::Pattern,
                        can_stop: self.over_song.is_some()
                            || self.mode == PlayMode::Pattern
                                && (self.playing || self.cursor != self.marker),
                        song_playing: self.playing && self.mode == PlayMode::Song,
                        over_song: self.over_song.is_some(),
                    }),
                    true,
                )
            })
            .and_then(|window| window.inner)
            .unwrap_or_default();
        self.rack_open = rack_open;
        for action in rack_actions {
            self.apply(action);
        }

        let held = self.live.held_pitches();
        self.piano_roll.hints = self.harmony_keys.hints;
        let roll_zoom = egui::vec2(
            view::Area::PianoRoll.zoom(&ctx, view::Axis::Horizontal),
            view::Area::PianoRoll.zoom(&ctx, view::Axis::Vertical),
        );
        let roll_actions = self.piano_roll.show(
            &ctx,
            &self.project,
            self.pattern,
            playhead,
            &held,
            area,
            roll_zoom,
        );
        for action in roll_actions {
            self.apply_roll(action);
        }

        // The chord sheet is drawn last, so it opens over the others.
        let playing = self.song_position().filter(|_| self.playing);
        let sheet_actions = self.chord_sheet.show(
            &ctx,
            area,
            &self.project,
            playing,
            self.live.typing(),
            (self.practice.view(), self.playlist.library_browser_mut()),
        );
        for action in sheet_actions {
            self.apply_sheet(action);
        }

        self.ear_training_window(&ctx, area);
        self.lead_sheet_window(&ctx, area);
        self.sound_font_window(&ctx);
        self.mixer_window(&ctx, area);
        self.shortcuts_window(&ctx, area);
        self.style_editor_window(&ctx, area);
        self.settings_window(&ctx);
        self.video_dialog(&ctx);
        self.playlist.library_browser_mut().store_preferences(&ctx);
        self.unsaved_changes_dialog(&ctx);
        self.update_title(&ctx);
        self.end_frame_for_undo(&ctx);
    }
}
