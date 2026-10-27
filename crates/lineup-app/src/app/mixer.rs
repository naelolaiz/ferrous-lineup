//! The mixer window: every channel, in the order of a score's sections,
//! with its instrument (click it to choose another, or to send the
//! channel over MIDI), its volume, pan, reverb send, effects (an
//! equalizer, a compressor and a delay) and level meter; and the master
//! volume, reverb and meter.

use eframe::egui::{self, Rect};
use lineup_core::score_order::score_order;
use lineup_core::{
    Channel, ChannelKind, Compressor, Delay, Effects, Equalizer, Mixer, SampleSource, general_midi,
};

use super::{Audio, LineupApp};
use crate::meters::{self, Meters};
use crate::widgets::{self, Opening};
use crate::{channel_rack, theme};

/// How far channel names sit in from their section's heading.
const INDENT: f32 = 10.0;

/// The size of a channel's level meter.
const METER: egui::Vec2 = egui::vec2(64.0, 8.0);

/// How often the mixer looks at the meters while nothing else redraws
/// the window: for notes played on a MIDI keyboard while stopped.
const METER_REFRESH: std::time::Duration = std::time::Duration::from_millis(100);

const METER_HELP: &str = "How loud it plays: full scale at the right end. Amber is close to \
                          full; red is over it, where the sound is rounded off.";

impl LineupApp {
    pub(super) fn mixer_window(&mut self, ctx: &egui::Context, area: Rect) {
        if !self.mixer_open {
            return;
        }
        if let Audio::Running(engine) = &self.audio {
            let seconds = ctx.input(|input| input.stable_dt).min(0.25);
            self.meters
                .update(engine.levels(), self.project.channels().len(), seconds);
            if self.meters.moving() {
                ctx.request_repaint();
            } else {
                ctx.request_repaint_after(METER_REFRESH);
            }
        }
        let meters = &self.meters;
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
                master_meter(ui, meters);
                percent_slider(ui, &mut mix.master);
                if ui
                    .button("Reset mix")
                    .on_hover_text(
                        "Reset master, reverb, solo, levels, pans, reverb sends, effects and \
                         mutes (the band's mutes stay); Undo restores them",
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
                        ui.strong("Level");
                        ui.strong("Volume");
                        ui.strong("Pan");
                        ui.strong("Reverb");
                        ui.strong("Effects");
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
                                // the rack. A drum has none, but can be sent
                                // over MIDI; a sample can't be either.
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
                                } else if channel.can_go_over_midi() {
                                    ui.menu_button(instrument(channel), |ui| {
                                        channel_rack::midi_part(ui, index, channel, &mut picked);
                                    })
                                    .response
                                    .on_hover_text(
                                        "Send it over MIDI, to a synth or a drum machine, as \
                                         its drum of a General MIDI drum set",
                                    );
                                } else {
                                    ui.label(instrument(channel));
                                }
                                meters::meter(ui, meters.channel(index), METER)
                                    .on_hover_text(METER_HELP);
                                percent_slider(ui, &mut channel.volume);
                                pan_slider(ui, &mut channel.pan);
                                ui.scope(|ui| {
                                    ui.spacing_mut().slider_width = 60.0;
                                    percent_slider(ui, &mut channel.reverb_send);
                                })
                                .response
                                .on_hover_text(
                                    if mix.reverb.enabled {
                                        "How much of it goes into the reverb: none plays it dry"
                                    } else {
                                        "How much of it goes into the reverb, once the Reverb \
                                     above is on"
                                    },
                                );
                                effects_menu(ui, &mut channel.effects);
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
                        state.pan = channel.pan;
                        state.reverb_send = channel.reverb_send;
                        state.effects = channel.effects;
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
        ChannelKind::Sampler(SampleSource::Builtin(drum)) => match channel.sent_on {
            Some(midi) => {
                let note = lineup_core::general_midi_drum(drum).midi();
                let name = general_midi::drum_name(note).unwrap_or("Drum");
                format!("{name} · MIDI {}", midi + 1)
            }
            None => "Built-in drum".to_string(),
        },
        ChannelKind::Sampler(SampleSource::File(_)) => "Sample".to_string(),
    }
}

/// The master's meters, left above right.
fn master_meter(ui: &mut egui::Ui, meters: &Meters) {
    let [left, right] = meters.master();
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 2.0;
        let size = egui::vec2(METER.x, 4.0);
        meters::meter(ui, left, size).on_hover_text(METER_HELP);
        meters::meter(ui, right, size).on_hover_text(METER_HELP);
    });
}

/// Where a channel sits between the speakers: "L50" is halfway to the
/// left, "C" the middle. A double-click puts it back in the middle.
fn pan_slider(ui: &mut egui::Ui, pan: &mut f32) {
    let response = ui
        .scope(|ui| {
            ui.spacing_mut().slider_width = 60.0;
            // Room for "L100", so the columns after it stay put.
            ui.spacing_mut().interact_size.x = 48.0;
            ui.add(
                egui::Slider::new(pan, -1.0..=1.0)
                    // Not lit from the left: the middle is where it starts.
                    .trailing_fill(false)
                    .custom_formatter(|value, _| pan_text(value as f32))
                    .custom_parser(pan_value),
            )
        })
        .inner
        .on_hover_text("Where it sits between the speakers. Double-click for the middle.");
    // A slider senses drags, not clicks, so the double-click is read from
    // the pointer.
    let double_click = ui.input(|input| {
        input
            .pointer
            .button_double_clicked(egui::PointerButton::Primary)
    });
    if double_click && response.contains_pointer() {
        *pan = 0.0;
    }
}

fn pan_text(pan: f32) -> String {
    let percent = (pan * 100.0).round();
    if percent < 0.0 {
        format!("L{:.0}", -percent)
    } else if percent > 0.0 {
        format!("R{percent:.0}")
    } else {
        "C".to_string()
    }
}

/// What a typed pan means: "L30", "R100", "C", or a number from -100
/// (left) to 100 (right).
fn pan_value(text: &str) -> Option<f64> {
    let text = text.trim().to_uppercase();
    let percent = if text == "C" {
        0.0
    } else if let Some(left) = text.strip_prefix('L') {
        -left.trim().parse::<f64>().ok()?
    } else if let Some(right) = text.strip_prefix('R') {
        right.trim().parse::<f64>().ok()?
    } else {
        text.parse::<f64>().ok()?
    };
    Some(percent / 100.0)
}

/// A channel's effects: a button that says which are on, and opens their
/// settings.
fn effects_menu(ui: &mut egui::Ui, effects: &mut Effects) {
    let summary = effects.summary();
    let label = if summary.is_empty() {
        egui::RichText::new("none").weak()
    } else {
        egui::RichText::new(summary)
    };
    ui.menu_button(label, |ui| {
        ui.set_min_width(300.0);
        effects_settings(ui, effects);
    })
    .response
    .on_hover_text(
        "Its own equalizer, compressor and delay, which shape its sound before it joins \
         the others",
    );
}

const EQ_HELP: &str = "Turns bands of frequencies up or down. The lows are the weight of a bass \
                       or a kick drum, the middle the body of most instruments, the highs the \
                       air and sparkle of cymbals. Turn a piano's lows down to leave the bass \
                       room.";
const COMPRESSOR_HELP: &str = "Evens out how loud it plays: above the threshold it turns the \
                               sound down, the more the higher the ratio (at 4:1, 8 dB over \
                               comes out 2 dB over). The gain brings the whole part back up. \
                               A ratio of 1:1 turns nothing down.";
const DELAY_HELP: &str = "Plays it again a little later, in time with the song, and again, \
                          each echo quieter: the feedback says how much of each echo comes \
                          back. A mix of 0% turns it off.";

/// The settings of the equalizer, the compressor and the delay.
fn effects_settings(ui: &mut egui::Ui, effects: &mut Effects) {
    ui.horizontal(|ui| {
        ui.strong("Equalizer").on_hover_text(EQ_HELP);
        if ui
            .add_enabled(!effects.eq.is_flat(), egui::Button::new("Flat"))
            .on_hover_text("Every band back to 0 dB")
            .clicked()
        {
            effects.eq = Equalizer::default();
        }
    });
    egui::Grid::new("eq").num_columns(2).show(ui, |ui| {
        let bands = [
            ("Low", "below 200 Hz", &mut effects.eq.low),
            ("Mid", "around 1 kHz", &mut effects.eq.mid),
            ("High", "above 4 kHz", &mut effects.eq.high),
        ];
        for (name, band, gain) in bands {
            ui.label(name).on_hover_text(band);
            decibel_slider(ui, gain, -Equalizer::RANGE..=Equalizer::RANGE)
                .on_hover_text(format!("{name}: {band}. Double-click for 0 dB."));
            ui.end_row();
        }
    });
    ui.separator();
    ui.strong("Compressor").on_hover_text(COMPRESSOR_HELP);
    let compressor = &mut effects.compressor;
    egui::Grid::new("compressor").num_columns(2).show(ui, |ui| {
        ui.label("Threshold");
        ui.add(
            egui::Slider::new(
                &mut compressor.threshold,
                Compressor::LOWEST_THRESHOLD..=0.0,
            )
            .suffix(" dB"),
        )
        .on_hover_text("The level it starts turning down at");
        ui.end_row();
        ui.label("Ratio");
        ui.add(
            egui::Slider::new(&mut compressor.ratio, 1.0..=Compressor::MOST_RATIO)
                .logarithmic(true)
                .custom_formatter(|ratio, _| format!("{ratio:.1}:1"))
                .custom_parser(|text| text.trim().trim_end_matches(":1").parse().ok()),
        )
        .on_hover_text("How much it turns down over the threshold: 1:1 is off");
        ui.end_row();
        ui.label("Gain");
        ui.add(egui::Slider::new(&mut compressor.gain, 0.0..=Compressor::MOST_GAIN).suffix(" dB"))
            .on_hover_text("How much it turns the whole part up afterwards");
        ui.end_row();
    });
    ui.separator();
    ui.strong("Delay").on_hover_text(DELAY_HELP);
    let delay = &mut effects.delay;
    egui::Grid::new("delay").num_columns(2).show(ui, |ui| {
        ui.label("Time");
        ui.horizontal_wrapped(|ui| {
            for (steps, name) in Delay::TIMES {
                ui.selectable_value(&mut delay.steps, steps, name)
                    .on_hover_text(
                        "How long after the sound each echo comes, in notes of the song",
                    );
            }
        });
        ui.end_row();
        ui.label("Feedback");
        ui.add(
            egui::Slider::new(&mut delay.feedback, 0.0..=Delay::MOST_FEEDBACK)
                .custom_formatter(|value, _| format!("{:.0}%", value * 100.0)),
        )
        .on_hover_text("How loud each echo is next to the one before");
        ui.end_row();
        ui.label("Mix");
        percent_slider(ui, &mut delay.mix);
        ui.end_row();
    });
    ui.separator();
    if ui
        .add_enabled(!effects.is_plain(), egui::Button::new("No effects"))
        .on_hover_text("Turn every effect of this channel off")
        .clicked()
    {
        *effects = Effects::default();
    }
}

/// A gain in decibels, that a double-click puts back at 0.
fn decibel_slider(
    ui: &mut egui::Ui,
    gain: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) -> egui::Response {
    let response = ui.add(
        egui::Slider::new(gain, range)
            .trailing_fill(false)
            .step_by(0.5)
            .custom_formatter(|value, _| format!("{value:+.1} dB"))
            .custom_parser(|text| text.trim().trim_end_matches("dB").trim().parse().ok()),
    );
    let double_click = ui.input(|input| {
        input
            .pointer
            .button_double_clicked(egui::PointerButton::Primary)
    });
    if double_click && response.contains_pointer() {
        *gain = 0.0;
    }
    response
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pans_read_as_they_are_written() {
        for (pan, text) in [(-1.0, "L100"), (-0.25, "L25"), (0.0, "C"), (0.5, "R50")] {
            assert_eq!(pan_text(pan), text);
            assert_eq!(pan_value(text), Some(f64::from(pan)));
        }
        assert_eq!(pan_text(-0.001), "C");
        assert_eq!(pan_value("r 30"), Some(0.3));
        assert_eq!(pan_value("-40"), Some(-0.4));
        assert_eq!(pan_value("left"), None);
    }
}
