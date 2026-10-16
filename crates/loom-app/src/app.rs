//! The main window.
//!
//! The playlist fills the window, as in other pattern-based studios. The
//! channel rack and the piano roll are windows on top of it, opened with
//! F6 and F7. Opening and saving files lives in the [`files`] module,
//! undo and redo in [`edit`], saving the song as a WAV or MIDI file in
//! [`export`], and playing and recording on a keyboard in [`live`].
//!
//! Every change to the project goes through [`LoomApp::edit`], which
//! records it for undo and tells the audio thread what changed.

mod edit;
mod export;
mod files;
mod library;
mod live;

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui;
use loom_audio::instrument::Sampler;
use loom_audio::{
    AudioEngine, ChannelNotes, Command, Instrument, PlayMode, SampleCache, SampleData,
};
use loom_core::{
    Channel, ChannelKind, DEFAULT_VELOCITY, History, MidiMessage, Note, Pattern, Project,
    SampleSource, Tempo,
};

use crate::channel_rack::{self, RackAction};
use crate::harmony_keys::{HarmonyAction, HarmonyKeys};
use crate::midi::Midi;
use crate::piano_roll::{PianoRoll, RollAction};
use crate::playlist::{self, PlaylistAction, PlaylistView};

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

/// The first Loom Osc channel, where the piano roll opens; or the first
/// channel if there is none.
fn first_synth(project: &Project) -> usize {
    project
        .channels()
        .iter()
        .position(|channel| channel.kind == ChannelKind::Synth)
        .unwrap_or(0)
}

/// All the state of the application window.
pub struct LoomApp {
    audio: Audio,
    project: Project,
    /// The selected pattern: the one the channel rack and the piano roll
    /// show, pattern mode plays, and new playlist clips use.
    pattern: usize,
    mode: PlayMode,
    playing: bool,
    master_volume: f32,
    rack_open: bool,
    piano_roll: PianoRoll,
    playlist: PlaylistView,
    harmony_keys: HarmonyKeys,
    /// The patterns the library wrote, to rewrite on the next click.
    auditions: Vec<library::Audition>,
    file: files::FileState,
    /// Earlier versions of the project, for undo and redo.
    history: History<Project>,
    /// Sample files already read, so undo and export do not read them again.
    samples: SampleCache,
    /// The WAV export running in the background, if any.
    export: Option<export::Export>,
    /// MIDI, the typing keyboard, and recording.
    live: live::Live,
    /// The last problem to show in the status bar, such as a bad WAV file.
    message: Option<String>,
    /// The last piece of good news for the status bar ("Exported song.wav").
    notice: Option<String>,
}

impl LoomApp {
    /// Called once by eframe, before the first frame is drawn. If `path`
    /// is given (a file named on the command line), it is opened;
    /// otherwise the demo song is.
    pub fn new(cc: &eframe::CreationContext<'_>, path: Option<PathBuf>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);

        let mut audio = match AudioEngine::start() {
            Ok(engine) => Audio::Running(Box::new(engine)),
            Err(err) => Audio::Unavailable(err.to_string()),
        };
        // The engine's ways in and out for MIDI go to the MIDI threads.
        let (live_input, midi_output) = match &mut audio {
            Audio::Running(engine) => (engine.take_live_input(), engine.take_midi_output()),
            Audio::Unavailable(_) => (None, None),
        };
        let midi = Midi::new(&cc.egui_ctx, live_input, midi_output);

        let project = Project::demo();
        let piano_roll = PianoRoll::new(first_synth(&project));
        let mut app = LoomApp {
            audio,
            file: files::FileState::new(&project),
            project,
            pattern: 0,
            mode: PlayMode::Song,
            playing: false,
            master_volume: 0.8,
            rack_open: true,
            piano_roll,
            playlist: PlaylistView::new(),
            harmony_keys: HarmonyKeys::new(),
            auditions: Vec::new(),
            history: History::default(),
            samples: SampleCache::new(),
            export: None,
            live: live::Live::new(midi),
            message: None,
            notice: None,
        };
        app.send_whole_project();
        if let Some(path) = path {
            app.open_path(&cc.egui_ctx, path);
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

    /// Builds the instrument for a channel, at `sample_rate`. If a sample
    /// file cannot be loaded, the channel gets a silent sampler and the
    /// status bar says why.
    fn build_instrument(&mut self, kind: &ChannelKind, sample_rate: u32) -> Instrument {
        match Instrument::for_channel(kind, sample_rate, &mut self.samples) {
            Ok(instrument) => instrument,
            Err(err) => {
                self.message = Some(err.to_string());
                Instrument::Sampler(Sampler::new(Arc::new(SampleData::new(Vec::new(), 1))))
            }
        }
    }

    /// Everything an empty audio thread needs to play the project, with
    /// instruments made for `sample_rate`: the tempo, the patterns, the
    /// channels and the song. Patterns go first, so each channel can bring
    /// its notes for all of them. Used at startup, after opening a file,
    /// and to export the song.
    fn project_commands(&mut self, sample_rate: u32) -> Vec<Command> {
        let mut commands = vec![Command::SetTempo(self.project.tempo)];
        for (index, pattern) in self.project.patterns().iter().enumerate() {
            commands.push(Command::InsertPattern {
                index,
                length: pattern.length(),
            });
        }
        for index in 0..self.project.channels().len() {
            commands.push(self.channel_command(index, sample_rate));
        }
        commands.push(self.song_command());
        commands
    }

    /// Tells the audio thread about the whole project, at startup and
    /// after opening a file.
    fn send_whole_project(&mut self) {
        self.send(Command::Clear);
        self.send(Command::SetMasterVolume(self.master_volume));
        for command in self.project_commands(self.sample_rate()) {
            self.send(command);
        }
        self.send(Command::SelectPattern(self.pattern));
        self.send(Command::SetMode(self.mode));
        self.live.resend_channel();
    }

    /// The command that inserts channel `index` of the project, with its
    /// instrument and its notes in every pattern.
    fn channel_command(&mut self, index: usize, sample_rate: u32) -> Command {
        let channel = self.project.channels()[index].clone();
        let notes = ChannelNotes::new(
            self.project
                .patterns()
                .iter()
                .map(|pattern| pattern.notes(index).to_vec()),
        );
        Command::InsertChannel {
            index,
            instrument: self.build_instrument(&channel.kind, sample_rate),
            notes,
            volume: channel.volume,
            muted: channel.muted,
        }
    }

    /// The song: the clips on tracks that are not muted, and its length.
    fn song_command(&self) -> Command {
        Command::SetSong {
            clips: self.project.playlist().audible_clips().collect(),
            length: self.project.song_length(),
        }
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

    fn set_playing(&mut self, playing: bool) {
        if !playing {
            // Before the audio thread forgets where it was.
            self.finish_recording();
        }
        self.playing = playing;
        self.send(if playing {
            Command::Play
        } else {
            Command::Stop
        });
    }

    fn set_mode(&mut self, mode: PlayMode) {
        if mode != self.mode {
            self.finish_recording();
            self.mode = mode;
            self.send(Command::SetMode(mode));
        }
    }

    fn select_pattern(&mut self, pattern: usize) {
        if pattern < self.project.patterns().len() {
            self.pattern = pattern;
            self.send(Command::SelectPattern(pattern));
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
                // Clicking a channel also selects it, for the piano roll
                // and the keyboards.
                self.piano_roll.select(channel);
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
            RackAction::Remove(channel) => {
                self.edit("delete channel", |project| project.remove_channel(channel));
            }
            RackAction::OpenPianoRoll(channel) => self.piano_roll.open_for(channel),
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
            RackAction::LoadSample { channel, path } => self.load_sample(channel, path),
        }
    }

    /// Applies one action from the piano roll.
    fn apply_roll(&mut self, action: RollAction) {
        match action {
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
            RollAction::SetScale(scale) => self.edit("change key", |project| project.scale = scale),
        }
    }

    /// Applies one action from the harmony keys bar.
    fn apply_harmony(&mut self, action: HarmonyAction) {
        match action {
            HarmonyAction::SetScale(scale) => {
                self.edit("change key", |project| project.scale = scale);
            }
            HarmonyAction::SetTime(time) => {
                self.edit("change time signature", |project| {
                    project.time_signature = time;
                });
            }
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
        }
    }

    /// Applies one action from the playlist.
    fn apply_playlist(&mut self, action: PlaylistAction) {
        match action {
            PlaylistAction::SelectPattern(pattern) => self.select_pattern(pattern),
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
            PlaylistAction::RemoveClip(index) => {
                self.edit("delete clips", |project| project.remove_clip(index));
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
            PlaylistAction::AddTrack => {
                self.edit("add track", |project| project.add_track());
            }
            PlaylistAction::Library(action) => self.try_from_library(action),
        }
    }

    fn add_channel(&mut self, channel: Channel) {
        self.edit("add channel", |project| project.add_channel(channel));
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

        self.edit("load sample", |project| {
            match channel.and_then(|index| project.channel_mut(index)) {
                Some(existing) => {
                    existing.name = name;
                    existing.kind = kind;
                }
                None => {
                    project.add_channel(Channel {
                        name,
                        kind,
                        volume: 0.8,
                        muted: false,
                    });
                }
            }
        });
        self.message = None;
    }

    /// The bar along the top: the file menu, transport, tempo, the
    /// pattern selector, window buttons and master volume.
    fn transport_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.file_menu(ui);
            self.edit_menu(ui);
            self.midi_menu(ui);
            ui.separator();

            let label = if self.playing { "⏹ Stop" } else { "▶ Play" };
            if ui.button(label).on_hover_text("Shortcut: Space").clicked() {
                self.set_playing(!self.playing);
            }
            self.record_button(ui);
            let mut mode = self.mode;
            ui.selectable_value(&mut mode, PlayMode::Pattern, "PAT")
                .on_hover_text("Loop the selected pattern (L switches)");
            ui.selectable_value(&mut mode, PlayMode::Song, "SONG")
                .on_hover_text("Play the playlist (L switches)");
            self.set_mode(mode);

            ui.separator();
            let mut bpm = self.project.tempo.bpm();
            let tempo_field = egui::DragValue::new(&mut bpm)
                .range(Tempo::MIN..=Tempo::MAX)
                .speed(0.5)
                .fixed_decimals(1)
                .suffix(" BPM");
            if ui.add(tempo_field).on_hover_text("Tempo").changed() {
                self.edit("change tempo", |project| project.tempo = Tempo::new(bpm));
            }

            ui.separator();
            self.pattern_selector(ui);

            ui.separator();
            ui.toggle_value(&mut self.rack_open, "Channel rack")
                .on_hover_text("Shortcut: F6");
            ui.toggle_value(&mut self.piano_roll.open, "🎹 Piano roll")
                .on_hover_text("Shortcut: F7");
            ui.toggle_value(&mut self.harmony_keys.open, "🎼 Harmony")
                .on_hover_text(
                    "The key, the time signature and a key for each chord. Shortcut: F8",
                );

            ui.separator();
            let volume_slider = egui::Slider::new(&mut self.master_volume, 0.0..=1.0)
                .show_value(false)
                .text("Master");
            if ui.add(volume_slider).changed() {
                self.send(Command::SetMasterVolume(self.master_volume));
            }
        });
    }

    /// Choosing, adding, cloning, renaming and deleting patterns, and the
    /// selected pattern's length.
    fn pattern_selector(&mut self, ui: &mut egui::Ui) {
        let current = self.pattern;
        let mut selected = current;
        let name = self.project.patterns()[current].name.clone();
        egui::ComboBox::from_id_salt("pattern")
            .width(120.0)
            .selected_text(egui::RichText::new(name).color(playlist::pattern_color(current)))
            .show_ui(ui, |ui| {
                for (index, pattern) in self.project.patterns().iter().enumerate() {
                    let text =
                        egui::RichText::new(&pattern.name).color(playlist::pattern_color(index));
                    ui.selectable_value(&mut selected, index, text);
                }
            })
            .response
            .on_hover_text("The pattern the channel rack shows");
        if selected != current {
            self.select_pattern(selected);
        }

        if ui.button("➕").on_hover_text("New pattern").clicked() {
            self.add_pattern();
        }
        // Menus normally close on any click. This one has a text field, so
        // it stays open until you click somewhere else.
        let stay_open = egui::containers::menu::MenuConfig::new()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
        let menu = egui::containers::menu::MenuButton::new("⋯").config(stay_open);
        menu.ui(ui, |ui| {
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

        let length = self.project.patterns()[self.pattern].length();
        let mut new_length = length;
        let time = self.project.time_signature;
        egui::ComboBox::from_id_salt("pattern-length")
            .selected_text(time.describe_length(length))
            .show_ui(ui, |ui| {
                for option in time.pattern_lengths() {
                    ui.selectable_value(&mut new_length, option, time.describe_length(option));
                }
            })
            .response
            .on_hover_text(format!("How long the pattern is: {length} steps"));
        if new_length != length {
            self.edit_pattern("change pattern length", |pattern| {
                pattern.set_length(new_length);
            });
        }
    }

    /// The bar along the bottom: which sound card is in use, and any problem.
    fn status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
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
            }
            if let Some(notice) = &self.notice {
                ui.separator();
                ui.label(notice);
            }
            if let Some(message) = &self.message {
                ui.separator();
                ui.colored_label(ui.visuals().error_fg_color, message);
            }
        });
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
            PlayMode::Song => self.project.pattern_step_in_song(self.pattern, step),
        }
    }

    /// The chord notes playing at the playhead, for the harmony keys.
    fn chord_playing(&self) -> Vec<Note> {
        match (self.playhead(), self.mode) {
            (Some(step), PlayMode::Pattern) => {
                self.project.chord_notes_in_pattern(self.pattern, step)
            }
            (Some(step), PlayMode::Song) => self.project.chord_notes_in_song(step),
            (None, _) => Vec::new(),
        }
    }

    /// The song step being played, for the playlist (song mode only).
    fn song_playhead(&self) -> Option<usize> {
        match self.mode {
            PlayMode::Song => self.playhead(),
            PlayMode::Pattern => None,
        }
    }

    fn keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        // While a text field has the keyboard (a track or pattern name),
        // Space and L are letters, not shortcuts.
        let typing = ctx.text_edit_focused();
        // `consume_key` reports whether the key was pressed and removes the
        // press, so a focused button does not also react to Space.
        let pressed = |key| ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
        if !typing && pressed(egui::Key::Space) {
            self.set_playing(!self.playing);
        }
        if !typing && pressed(egui::Key::L) {
            self.set_mode(match self.mode {
                PlayMode::Pattern => PlayMode::Song,
                PlayMode::Song => PlayMode::Pattern,
            });
        }
        if pressed(egui::Key::F6) {
            self.rack_open = !self.rack_open;
        }
        if pressed(egui::Key::F7) {
            self.piano_roll.open = !self.piano_roll.open;
        }
        if pressed(egui::Key::F8) {
            self.harmony_keys.open = !self.harmony_keys.open;
        }
        self.file_shortcuts(ctx);
        if !typing {
            // While typing, Ctrl+Z belongs to the text field.
            self.edit_shortcuts(ctx);
        }
    }
}

impl eframe::App for LoomApp {
    /// Called by eframe every time the window needs to be redrawn.
    /// egui is "immediate mode": we describe the whole UI on every call.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Audio::Running(engine) = &mut self.audio {
            engine.collect_garbage();
        }
        let ctx = ui.ctx().clone();
        self.start_frame_for_undo(&ctx);
        self.keyboard_shortcuts(&ctx);
        self.live_input(&ctx);
        self.handle_dropped_files(&ctx);
        self.check_export(&ctx);
        if self.playing {
            // Keep redrawing so the playhead moves.
            ctx.request_repaint();
        }

        egui::Panel::top("transport").show(ui, |ui| self.transport_bar(ui));
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        // Added after the status bar, so it sits just above it.
        let harmony_actions = if self.harmony_keys.open {
            let held = self.live.held_pitches();
            let channel = self.piano_roll.channel();
            let channel_name = self
                .project
                .channels()
                .get(channel)
                .map_or("", |channel| channel.name.as_str());
            let heard = self.chord_playing();
            let digits = !self.live.typing() && !ctx.text_edit_focused();
            egui::Panel::bottom("harmony-keys")
                .show(ui, |ui| {
                    self.harmony_keys
                        .show(ui, &self.project, channel_name, &held, &heard, digits)
                })
                .inner
        } else {
            self.harmony_keys.release()
        };
        for action in harmony_actions {
            self.apply_harmony(action);
        }

        let song_playhead = self.song_playhead();
        let tried = self.tried();
        let playlist_actions = egui::CentralPanel::default_margins()
            .show(ui, |ui| {
                self.playlist
                    .show(ui, &self.project, self.pattern, song_playhead, tried)
            })
            .inner;
        for action in playlist_actions {
            self.apply_playlist(action);
        }

        // The channel rack and the piano roll float on top of the playlist.
        let playhead = self.pattern_playhead();
        let mut rack_open = self.rack_open;
        let pattern_name = &self.project.patterns()[self.pattern].name;
        let rack_actions = egui::Window::new(format!("Channel rack · {pattern_name}"))
            .id(egui::Id::unique("channel-rack"))
            .open(&mut rack_open)
            .default_pos([240.0, 380.0])
            .resizable(false)
            .show(&ctx, |ui| {
                let selected = self.piano_roll.channel();
                channel_rack::show(ui, &self.project, self.pattern, selected, playhead)
            })
            .and_then(|window| window.inner)
            .unwrap_or_default();
        self.rack_open = rack_open;
        for action in rack_actions {
            self.apply(action);
        }

        let held = self.live.held_pitches();
        let roll_actions = self
            .piano_roll
            .show(&ctx, &self.project, self.pattern, playhead, &held);
        for action in roll_actions {
            self.apply_roll(action);
        }

        self.unsaved_changes_dialog(&ctx);
        self.update_title(&ctx);
        self.end_frame_for_undo(&ctx);
    }
}
