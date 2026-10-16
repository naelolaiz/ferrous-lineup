//! The main window.
//!
//! The playlist fills the window, as in other pattern-based studios. The
//! channel rack and the piano roll are windows on top of it, opened with
//! F6 and F7. Opening and saving files lives in the [`files`] module.

mod files;

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui;
use loom_audio::instrument::Sampler;
use loom_audio::{AudioEngine, ChannelNotes, Command, Instrument, PlayMode, SampleData};
use loom_core::{Channel, ChannelKind, PATTERN_LENGTHS, Pattern, Project, SampleSource, Tempo};

use crate::channel_rack::{self, RackAction};
use crate::piano_roll::{PianoRoll, RollAction};
use crate::playlist::{self, PlaylistAction, PlaylistView};

/// The sample rate used to prepare sounds when there is no sound card.
const FALLBACK_SAMPLE_RATE: u32 = 48_000;

/// Whether we managed to open the sound card.
///
/// The app still runs without audio (for example on a machine with no
/// sound card), and shows the reason in the status bar.
enum Audio {
    Running(AudioEngine),
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
    file: files::FileState,
    /// The last problem to show in the status bar, such as a bad WAV file.
    message: Option<String>,
}

impl LoomApp {
    /// Called once by eframe, before the first frame is drawn. If `path`
    /// is given (a file named on the command line), it is opened;
    /// otherwise the demo song is.
    pub fn new(cc: &eframe::CreationContext<'_>, path: Option<PathBuf>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);

        let audio = match AudioEngine::start() {
            Ok(engine) => Audio::Running(engine),
            Err(err) => Audio::Unavailable(err.to_string()),
        };

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
            message: None,
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

    /// Builds the instrument for a channel. If a sample file cannot be
    /// loaded, the channel gets a silent sampler and the status bar says why.
    fn build_instrument(&mut self, kind: &ChannelKind) -> Instrument {
        match Instrument::for_channel(kind, self.sample_rate()) {
            Ok(instrument) => instrument,
            Err(err) => {
                self.message = Some(err.to_string());
                Instrument::Sampler(Sampler::new(Arc::new(SampleData::new(Vec::new(), 1))))
            }
        }
    }

    /// Tells the audio thread about the whole project, at startup and
    /// after opening a file. Patterns go first, so each channel can bring
    /// its notes for all of them.
    fn send_whole_project(&mut self) {
        self.send(Command::Clear);
        self.send(Command::SetTempo(self.project.tempo));
        self.send(Command::SetMasterVolume(self.master_volume));
        for index in 0..self.project.patterns().len() {
            let length = self.project.patterns()[index].length();
            self.send(Command::AddPattern { length });
        }
        for index in 0..self.project.channels().len() {
            self.send_new_channel(index);
        }
        self.send(Command::SelectPattern(self.pattern));
        self.send(Command::SetMode(self.mode));
        self.send_song();
    }

    /// Sends channel `index` of the project to the audio thread as a new
    /// channel, with its notes in every pattern.
    fn send_new_channel(&mut self, index: usize) {
        let channel = self.project.channels()[index].clone();
        let notes = ChannelNotes::new(
            self.project
                .patterns()
                .iter()
                .map(|pattern| pattern.notes(index).to_vec()),
        );
        let instrument = self.build_instrument(&channel.kind);
        self.send(Command::AddChannel {
            instrument,
            notes,
            volume: channel.volume,
            muted: channel.muted,
        });
    }

    /// Sends a fresh copy of one channel's notes in the selected pattern.
    fn send_notes(&mut self, channel: usize) {
        self.send_pattern_notes(self.pattern, channel);
    }

    fn send_pattern_notes(&mut self, pattern: usize, channel: usize) {
        if let Some(notes) = self
            .project
            .pattern(pattern)
            .map(|p| p.notes(channel).to_vec())
        {
            self.send(Command::SetNotes {
                pattern,
                channel,
                notes,
            });
        }
    }

    /// Sends the song: the clips on tracks that are not muted, and its length.
    fn send_song(&mut self) {
        let clips = self.project.playlist().audible_clips().collect();
        let length = self.project.song_length();
        self.send(Command::SetSong { clips, length });
    }

    /// Changes the selected pattern with `edit`. If `edit` says it changed
    /// something, the channel's new notes are sent to the audio thread.
    fn edit_notes(&mut self, channel: usize, edit: impl FnOnce(&mut Pattern) -> bool) {
        let changed = self.project.pattern_mut(self.pattern).is_some_and(edit);
        if changed {
            self.send_notes(channel);
        }
    }

    fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
        self.send(if playing {
            Command::Play
        } else {
            Command::Stop
        });
    }

    fn set_mode(&mut self, mode: PlayMode) {
        if mode != self.mode {
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

    fn add_pattern(&mut self) {
        if let Some(index) = self.project.add_pattern() {
            let length = self.project.patterns()[index].length();
            self.send(Command::AddPattern { length });
            self.select_pattern(index);
        }
    }

    fn clone_pattern(&mut self) {
        if let Some(index) = self.project.clone_pattern(self.pattern) {
            let length = self.project.patterns()[index].length();
            self.send(Command::AddPattern { length });
            for channel in 0..self.project.channels().len() {
                self.send_pattern_notes(index, channel);
            }
            self.select_pattern(index);
        }
    }

    fn remove_pattern(&mut self) {
        let removed = self.pattern;
        if self.project.remove_pattern(removed).is_some() {
            self.send(Command::RemovePattern(removed));
            // Select the pattern that took its place, or the new last one.
            let last = self.project.patterns().len() - 1;
            self.select_pattern(removed.min(last));
            self.send_song();
        }
    }

    fn set_pattern_length(&mut self, length: usize) {
        if let Some(pattern) = self.project.pattern_mut(self.pattern) {
            pattern.set_length(length);
            self.send(Command::SetPatternLength {
                pattern: self.pattern,
                length,
            });
            // Clips are as long as their pattern, so the song may be longer now.
            self.send_song();
        }
    }

    /// Applies one action from the channel rack to the project and the audio thread.
    fn apply(&mut self, action: RackAction) {
        match action {
            RackAction::ToggleStep { channel, step } => {
                self.edit_notes(channel, |pattern| pattern.toggle(channel, step).is_some());
            }
            RackAction::ClearStep { channel, step } => {
                self.edit_notes(channel, |pattern| pattern.set(channel, step, false));
            }
            RackAction::Preview(channel) => self.send(Command::Preview(channel)),
            RackAction::SetVolume { channel, volume } => {
                if let Some(state) = self.project.channel_mut(channel) {
                    state.volume = volume;
                    self.send(Command::SetVolume { channel, volume });
                }
            }
            RackAction::ToggleMute(channel) => {
                if let Some(state) = self.project.channel_mut(channel) {
                    state.muted = !state.muted;
                    let muted = state.muted;
                    self.send(Command::SetMuted { channel, muted });
                }
            }
            RackAction::Remove(channel) => {
                if self.project.remove_channel(channel).is_some() {
                    self.send(Command::RemoveChannel(channel));
                    self.piano_roll.channel_removed(channel);
                }
            }
            RackAction::OpenPianoRoll(channel) => self.piano_roll.open_for(channel),
            RackAction::AddDrum(drum) => self.add_channel(Channel::drum(drum)),
            RackAction::AddSynth => self.add_channel(Channel::synth()),
            RackAction::LoadSample { channel, path } => self.load_sample(channel, path),
        }
    }

    /// Applies one action from the piano roll to the project and the audio thread.
    fn apply_roll(&mut self, action: RollAction) {
        match action {
            RollAction::AddNotes { channel, notes } => {
                self.edit_notes(channel, |pattern| {
                    let mut added = false;
                    for note in notes {
                        added |= pattern.add_note(channel, note).is_some();
                    }
                    added
                });
            }
            RollAction::RemoveNote { channel, index } => {
                self.edit_notes(channel, |pattern| {
                    pattern.remove_note(channel, index).is_some()
                });
            }
            RollAction::ReplaceNote {
                channel,
                index,
                note,
            } => {
                self.edit_notes(channel, |pattern| {
                    pattern.replace_note(channel, index, note)
                });
            }
            RollAction::Preview { channel, pitches } => {
                for pitch in pitches {
                    self.send(Command::PreviewNote { channel, pitch });
                }
            }
            RollAction::SetScale(scale) => self.project.scale = scale,
        }
    }

    /// Applies one action from the playlist to the project and the audio thread.
    fn apply_playlist(&mut self, action: PlaylistAction) {
        let song_changed = match action {
            PlaylistAction::SelectPattern(pattern) => {
                self.select_pattern(pattern);
                false
            }
            PlaylistAction::AddClip(clip) => self.project.add_clip(clip).is_some(),
            PlaylistAction::MoveClip {
                index,
                track,
                start,
            } => self.project.move_clip(index, track, start),
            PlaylistAction::RemoveClip(index) => self.project.remove_clip(index).is_some(),
            PlaylistAction::ToggleTrackMute(track) => match self.project.track_mut(track) {
                Some(track) => {
                    track.muted = !track.muted;
                    true
                }
                None => false,
            },
            PlaylistAction::RenameTrack { track, name } => {
                if let Some(track) = self.project.track_mut(track) {
                    track.name = name;
                }
                false
            }
            PlaylistAction::AddTrack => {
                self.project.add_track();
                false
            }
        };
        if song_changed {
            self.send_song();
        }
    }

    fn add_channel(&mut self, channel: Channel) {
        if let Some(index) = self.project.add_channel(channel) {
            self.send_new_channel(index);
        }
    }

    fn load_sample(&mut self, channel: Option<usize>, path: PathBuf) {
        let kind = ChannelKind::Sampler(SampleSource::File(path.clone()));
        let instrument = match Instrument::for_channel(&kind, self.sample_rate()) {
            Ok(instrument) => instrument,
            Err(err) => {
                self.message = Some(format!("{}: {err}", path.display()));
                return;
            }
        };
        let name = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Sample".to_string());

        match channel.and_then(|index| self.project.channel_mut(index).map(|c| (index, c))) {
            Some((index, existing)) => {
                existing.name = name;
                existing.kind = kind;
                self.send(Command::SetInstrument {
                    channel: index,
                    instrument,
                });
            }
            None => {
                let new_channel = Channel {
                    name,
                    kind,
                    volume: 0.8,
                    muted: false,
                };
                if let Some(index) = self.project.add_channel(new_channel) {
                    let notes = ChannelNotes::new(
                        self.project
                            .patterns()
                            .iter()
                            .map(|pattern| pattern.notes(index).to_vec()),
                    );
                    self.send(Command::AddChannel {
                        instrument,
                        notes,
                        volume: 0.8,
                        muted: false,
                    });
                }
            }
        }
        self.message = None;
    }

    /// The bar along the top: the file menu, transport, tempo, the
    /// pattern selector, window buttons and master volume.
    fn transport_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.file_menu(ui);
            ui.separator();

            let label = if self.playing { "⏹ Stop" } else { "▶ Play" };
            if ui.button(label).on_hover_text("Shortcut: Space").clicked() {
                self.set_playing(!self.playing);
            }
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
                self.project.tempo = Tempo::new(bpm);
                self.send(Command::SetTempo(self.project.tempo));
            }

            ui.separator();
            self.pattern_selector(ui);

            ui.separator();
            ui.toggle_value(&mut self.rack_open, "Channel rack")
                .on_hover_text("Shortcut: F6");
            ui.toggle_value(&mut self.piano_roll.open, "🎹 Piano roll")
                .on_hover_text("Shortcut: F7");

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
            if let Some(pattern) = self.project.pattern_mut(self.pattern) {
                ui.text_edit_singleline(&mut pattern.name);
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
        egui::ComboBox::from_id_salt("pattern-length")
            .selected_text(format!("{length} steps"))
            .show_ui(ui, |ui| {
                for option in PATTERN_LENGTHS {
                    ui.selectable_value(&mut new_length, option, format!("{option} steps"));
                }
            });
        if new_length != length {
            self.set_pattern_length(new_length);
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
        self.file_shortcuts(ctx);
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
        self.keyboard_shortcuts(&ctx);
        self.handle_dropped_files(&ctx);
        if self.playing {
            // Keep redrawing so the playhead moves.
            ctx.request_repaint();
        }

        egui::Panel::top("transport").show(ui, |ui| self.transport_bar(ui));
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));

        let song_playhead = self.song_playhead();
        let playlist_actions = egui::CentralPanel::default_margins()
            .show(ui, |ui| {
                self.playlist
                    .show(ui, &self.project, self.pattern, song_playhead)
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
            .default_pos([170.0, 380.0])
            .resizable(false)
            .show(&ctx, |ui| {
                channel_rack::show(ui, &self.project, self.pattern, playhead)
            })
            .and_then(|window| window.inner)
            .unwrap_or_default();
        self.rack_open = rack_open;
        for action in rack_actions {
            self.apply(action);
        }

        let roll_actions = self
            .piano_roll
            .show(&ctx, &self.project, self.pattern, playhead);
        for action in roll_actions {
            self.apply_roll(action);
        }

        self.unsaved_changes_dialog(&ctx);
        self.update_title(&ctx);
    }
}
