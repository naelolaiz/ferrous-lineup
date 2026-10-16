//! The main window.

use eframe::egui;
use loom_audio::{AudioEngine, Command};
use loom_core::{Note, Tempo};

/// Lowest and highest notes offered by the test tone slider (C2 to C7).
const TONE_RANGE: std::ops::RangeInclusive<u8> = 36..=96;

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
    playing: bool,
    tempo: Tempo,
    tone_note: Note,
    volume: f32,
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
            playing: false,
            tempo: Tempo::default(),
            tone_note: Note::A4,
            volume: 0.3,
        };
        // Tell the audio thread our starting values.
        app.send(Command::SetFrequency(app.tone_note.frequency()));
        app.send(Command::SetVolume(app.volume));
        app
    }

    /// Sends a command to the audio thread, if audio is running.
    fn send(&mut self, command: Command) {
        if let Audio::Running(engine) = &mut self.audio {
            engine.send(command);
        }
    }

    fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
        self.send(Command::SetToneOn(playing));
    }

    /// The bar along the top: play button, tempo and test tone controls.
    fn transport_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.strong("Ferrous Loom");
            ui.separator();

            let label = if self.playing { "⏹ Stop" } else { "▶ Play" };
            if ui.button(label).on_hover_text("Shortcut: Space").clicked() {
                self.set_playing(!self.playing);
            }

            ui.separator();
            let mut bpm = self.tempo.bpm();
            let tempo_field = egui::DragValue::new(&mut bpm)
                .range(Tempo::MIN..=Tempo::MAX)
                .speed(0.5)
                .fixed_decimals(1)
                .suffix(" BPM");
            if ui.add(tempo_field).on_hover_text("Tempo").changed() {
                self.tempo = Tempo::new(bpm);
            }

            ui.separator();
            let mut midi = self.tone_note.midi();
            let note_slider = egui::Slider::new(&mut midi, TONE_RANGE)
                .text("Test tone")
                .custom_formatter(|n, _| Note::new(n as u8).to_string());
            if ui.add(note_slider).changed() {
                self.tone_note = Note::new(midi);
                self.send(Command::SetFrequency(self.tone_note.frequency()));
            }
            ui.label(format!("{:.1} Hz", self.tone_note.frequency()));

            ui.separator();
            let volume_slider = egui::Slider::new(&mut self.volume, 0.0..=1.0).text("Volume");
            if ui.add(volume_slider).changed() {
                self.send(Command::SetVolume(self.volume));
            }
        });
    }

    /// The bar along the bottom: which sound card is in use.
    fn status_bar(&self, ui: &mut egui::Ui) {
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

    /// The main area. For now it explains what this version can do.
    fn welcome(&self, ui: &mut egui::Ui) {
        ui.heading("Welcome to Ferrous Loom");
        ui.add_space(8.0);
        ui.label(
            "This is step 1, the foundation: a window, a working audio engine and a test tone.",
        );
        ui.label(
            "Press Space or ▶ Play to hear it, and move the Test tone slider to change the note.",
        );
        ui.add_space(8.0);
        ui.label(format!(
            "The tempo is {:.1} BPM, so one beat lasts {:.3} seconds. \
             In step 2 it will drive the step sequencer.",
            self.tempo.bpm(),
            self.tempo.seconds_per_beat()
        ));
    }
}

impl eframe::App for LoomApp {
    /// Called by eframe every time the window needs to be redrawn.
    /// egui is "immediate mode": we describe the whole UI on every call.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let typing = ui.ctx().memory(|memory| memory.focused().is_some());
        if !typing && ui.ctx().input(|input| input.key_pressed(egui::Key::Space)) {
            self.set_playing(!self.playing);
        }

        egui::Panel::top("transport").show(ui, |ui| self.transport_bar(ui));
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        egui::CentralPanel::default_margins().show(ui, |ui| self.welcome(ui));
    }
}
