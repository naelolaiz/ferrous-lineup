//! The main window.

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui;
use loom_audio::instrument::Sampler;
use loom_audio::{AudioEngine, Command, Instrument, SampleData};
use loom_core::{Channel, ChannelKind, MAX_STEPS, PATTERN_LENGTHS, Project, SampleSource, Tempo};

use crate::channel_rack::{self, RackAction};

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

/// All the state of the application window.
pub struct LoomApp {
    audio: Audio,
    project: Project,
    playing: bool,
    master_volume: f32,
    /// The last problem to show in the status bar, such as a bad WAV file.
    message: Option<String>,
}

impl LoomApp {
    /// Called once by eframe, before the first frame is drawn.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);

        let audio = match AudioEngine::start() {
            Ok(engine) => Audio::Running(engine),
            Err(err) => Audio::Unavailable(err.to_string()),
        };

        let mut app = LoomApp {
            audio,
            project: Project::demo(),
            playing: false,
            master_volume: 0.8,
            message: None,
        };
        app.send_whole_project();
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

    /// Tells the audio thread about the whole project, at startup.
    fn send_whole_project(&mut self) {
        self.send(Command::SetTempo(self.project.tempo));
        self.send(Command::SetPatternLength(self.project.pattern().length()));
        self.send(Command::SetMasterVolume(self.master_volume));
        for index in 0..self.project.channels().len() {
            self.send_new_channel(index);
        }
    }

    /// Sends channel `index` of the project to the audio thread as a new channel.
    fn send_new_channel(&mut self, index: usize) {
        let channel = self.project.channels()[index].clone();
        let steps = self
            .project
            .steps(index)
            .copied()
            .unwrap_or([false; MAX_STEPS]);
        let instrument = self.build_instrument(&channel.kind);
        self.send(Command::AddChannel {
            instrument,
            steps,
            volume: channel.volume,
            muted: channel.muted,
        });
    }

    fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
        self.send(if playing {
            Command::Play
        } else {
            Command::Stop
        });
    }

    /// Applies one action from the channel rack to the project and the audio thread.
    fn apply(&mut self, action: RackAction) {
        match action {
            RackAction::ToggleStep { channel, step } => {
                if let Some(on) = self.project.toggle_step(channel, step) {
                    self.send(Command::SetStep { channel, step, on });
                }
            }
            RackAction::ClearStep { channel, step } => {
                if self.project.set_step(channel, step, false) {
                    self.send(Command::SetStep {
                        channel,
                        step,
                        on: false,
                    });
                }
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
                }
            }
            RackAction::AddDrum(drum) => self.add_channel(Channel::drum(drum)),
            RackAction::AddSynth => self.add_channel(Channel::synth()),
            RackAction::LoadSample { channel, path } => self.load_sample(channel, path),
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
                if self.project.add_channel(new_channel).is_some() {
                    self.send(Command::AddChannel {
                        instrument,
                        steps: [false; MAX_STEPS],
                        volume: 0.8,
                        muted: false,
                    });
                }
            }
        }
        self.message = None;
    }

    /// The bar along the top: play button, tempo, pattern length and master volume.
    fn transport_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.strong("Ferrous Loom");
            ui.separator();

            let label = if self.playing { "⏹ Stop" } else { "▶ Play" };
            if ui.button(label).on_hover_text("Shortcut: Space").clicked() {
                self.set_playing(!self.playing);
            }

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
            let mut length = self.project.pattern().length();
            egui::ComboBox::from_id_salt("pattern-length")
                .selected_text(format!("{length} steps"))
                .show_ui(ui, |ui| {
                    for option in PATTERN_LENGTHS {
                        ui.selectable_value(&mut length, option, format!("{option} steps"));
                    }
                });
            if length != self.project.pattern().length() {
                self.project.set_pattern_length(length);
                self.send(Command::SetPatternLength(length));
            }

            ui.separator();
            let volume_slider = egui::Slider::new(&mut self.master_volume, 0.0..=1.0)
                .show_value(false)
                .text("Master");
            if ui.add(volume_slider).changed() {
                self.send(Command::SetMasterVolume(self.master_volume));
            }
        });
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

    fn playhead(&self) -> Option<usize> {
        match &self.audio {
            Audio::Running(engine) if self.playing => engine.playhead(),
            _ => None,
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

        let typing = ui.ctx().memory(|memory| memory.focused().is_some());
        if !typing && ui.ctx().input(|input| input.key_pressed(egui::Key::Space)) {
            self.set_playing(!self.playing);
        }
        if self.playing {
            // Keep redrawing so the playhead moves.
            ui.ctx().request_repaint();
        }

        egui::Panel::top("transport").show(ui, |ui| self.transport_bar(ui));
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));

        let playhead = self.playhead();
        let actions = egui::CentralPanel::default_margins()
            .show(ui, |ui| {
                ui.heading("Channel rack");
                ui.add_space(4.0);
                channel_rack::show(ui, &self.project, playhead)
            })
            .inner;
        for action in actions {
            self.apply(action);
        }
    }
}
