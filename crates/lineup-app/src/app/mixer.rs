//! One mixer for every internal instrument and the master reverb.

use eframe::egui::{self, Rect};
use lineup_core::score_order::score_order;
use lineup_core::{Channel, ChannelKind, Mixer, SampleSource, general_midi};

use super::LineupApp;
use crate::widgets::{self, Opening};
use crate::{channel_rack, theme};

/// How far channel names sit in from their section's heading.
const INDENT: f32 = 10.0;

impl LineupApp {
    pub(super) fn mixer_window(&mut self, ctx: &egui::Context, area: Rect) {
        if !self.mixer_open {
            return;
        }
        let mut open = self.mixer_open;
        let mut mix = self.project.mixer;
        let mut levels = self.project.channels().to_vec();
        let locked = self.project.locked();
        let mut picked = Vec::new();
        let id = egui::Id::unique("mixer");
        let window = egui::Window::new("Mixer")
            .id(id)
            .open(&mut open)
            .constrain_to(area);
        // As wide as its columns and as tall as its channels, up to the
        // height of the playlist; the channels scroll past that.
        // So it is fitted again when they change: another channel or
        // instrument can make a column narrower. The channels' grid is
        // then a new one, which measures its columns afresh.
        let size = egui::vec2(320.0, area.height());
        let rows: Vec<(&str, String)> = levels
            .iter()
            .map(|channel| (channel.name.as_str(), instrument(channel)))
            .collect();
        let grid = egui::Id::unique(("mixer-channels", &rows));
        let window = widgets::open_at(ctx, window, id, size, Opening::EveryTime);
        widgets::fit_when_changed(ctx, window, id, size, &rows).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Master");
                percent_slider(ui, &mut mix.master);
                if ui
                    .button("Reset mix")
                    .on_hover_text(
                        "Reset master, reverb, solo, levels and mutes (the band's mutes stay); \
                             Undo restores them",
                    )
                    .clicked()
                {
                    let mut reset = self.project.clone();
                    reset.reset_mix();
                    mix = reset.mixer;
                    levels = reset.channels().to_vec();
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox(&mut mix.reverb.enabled, "Reverb");
                ui.weak("Stereo room · SoundFont + Lineup Osc + samples");
            });
            ui.add_enabled_ui(mix.reverb.enabled, |ui| {
                egui::Grid::new("reverb-controls")
                    .num_columns(4)
                    .show(ui, |ui| {
                        ui.label("Wet / dry");
                        percent_slider(ui, &mut mix.reverb.mix);
                        ui.label("Decay");
                        ui.add(egui::Slider::new(&mut mix.reverb.decay, 0.1..=10.0).suffix(" s"));
                        ui.end_row();
                        ui.label("Damping");
                        percent_slider(ui, &mut mix.reverb.damping);
                        ui.label("Pre-delay");
                        ui.add(
                            egui::Slider::new(&mut mix.reverb.pre_delay_ms, 0.0..=200.0)
                                .suffix(" ms"),
                        );
                        ui.end_row();
                    });
            });
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new(grid)
                    .striped(true)
                    .min_col_width(45.0)
                    .show(ui, |ui| {
                        ui.strong("Channel");
                        ui.strong("Instrument");
                        ui.strong("Volume");
                        ui.strong("Mute");
                        ui.strong("Solo");
                        ui.end_row();
                        for (section, indices) in score_order(&levels) {
                            ui.label(egui::RichText::new(section.name()).color(theme::ACCENT));
                            ui.end_row();
                            for index in indices {
                                let channel = &mut levels[index];
                                ui.horizontal(|ui| {
                                    ui.add_space(INDENT);
                                    ui.label(&channel.name);
                                    if locked.contains(index) {
                                        ui.label("🔒").on_hover_text(
                                            "Locked: styles keep its notes and sound",
                                        );
                                    }
                                });
                                // A synth's instrument is picked here as in
                                // the rack; a drum or a sample has none.
                                if matches!(
                                    channel.kind,
                                    ChannelKind::Synth | ChannelKind::MidiOut { .. }
                                ) {
                                    ui.menu_button(instrument(channel), |ui| {
                                        channel_rack::instrument_choices(
                                            ui,
                                            index,
                                            channel,
                                            &mut picked,
                                        );
                                    })
                                    .response
                                    .on_hover_text("Choose the instrument it plays");
                                } else {
                                    ui.label(instrument(channel));
                                }
                                percent_slider(ui, &mut channel.volume);
                                ui.checkbox(&mut channel.muted, "")
                                    .on_hover_text("Mute this channel");
                                if ui
                                    .selectable_label(mix.solo == Some(index), "S")
                                    .on_hover_text(
                                        "Hear only this channel; click again to clear solo",
                                    )
                                    .clicked()
                                {
                                    mix.solo = if mix.solo == Some(index) {
                                        None
                                    } else {
                                        Some(index)
                                    };
                                }
                                ui.end_row();
                            }
                        }
                    });
                if levels.is_empty() {
                    ui.weak("Add an instrument in the channel rack to mix it here.");
                }
            });
        });
        self.mixer_open = open;
        let master_only = levels == self.project.channels()
            && mix
                == Mixer {
                    master: self.project.mixer.master,
                    ..mix
                };
        if mix.master != self.project.mixer.master && master_only {
            self.set_master_volume(mix.master);
        } else if mix != self.project.mixer || levels != self.project.channels() {
            self.edit("mixer", |project| {
                project.mixer = mix;
                for (index, channel) in levels.into_iter().enumerate() {
                    if let Some(state) = project.channel_mut(index) {
                        state.volume = channel.volume;
                        state.muted = channel.muted;
                    }
                }
            });
        }
        for action in picked {
            self.apply(action);
        }
    }
}

impl LineupApp {
    /// Sets the master volume. It is saved with the song, so exports sound
    /// the same, but one nudge after another is one step to undo.
    pub(super) fn set_master_volume(&mut self, master: f32) {
        const STEP: &str = "master volume";
        self.history.reopen(STEP);
        self.edit(STEP, |project| project.mixer.master = master);
    }
}

/// What a channel plays, for the Instrument column.
fn instrument(channel: &Channel) -> String {
    match channel.kind {
        ChannelKind::Synth => channel
            .program
            .map_or("Lineup Osc", general_midi::name)
            .to_string(),
        ChannelKind::MidiOut { channel: midi } => match channel.program {
            Some(program) => format!("{} · MIDI {}", general_midi::name(program), midi + 1),
            None => format!("MIDI Out {}", midi + 1),
        },
        ChannelKind::Sampler(SampleSource::Builtin(_)) => "Built-in drum".to_string(),
        ChannelKind::Sampler(SampleSource::File(_)) => "Sample".to_string(),
    }
}

fn percent_slider(ui: &mut egui::Ui, value: &mut f32) {
    ui.add(
        egui::Slider::new(value, 0.0..=1.0)
            .custom_formatter(|value, _| format!("{:.0}%", value * 100.0))
            .custom_parser(|text| {
                text.trim()
                    .trim_end_matches('%')
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .map(|value| value / 100.0)
            }),
    );
}
