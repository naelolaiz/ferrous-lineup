//! Application preferences, collected in Edit > Settings.

use eframe::egui::{self, Align, Layout, RichText, vec2};
use lineup_core::harmony::MinorNumerals;
use lineup_core::library::apply::StyleParts;

use super::{LineupApp, view};
use crate::library_browser::{
    StyleEditorRequest, grouped_presets, set_grouped_presets, style_mode_ui, style_parts_checkboxes,
};
use crate::theme;
use crate::widgets::{self, Opening};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Page {
    #[default]
    Appearance,
    Harmony,
    Library,
    Audio,
    Playback,
}

impl Page {
    const ALL: [Self; 5] = [
        Self::Appearance,
        Self::Harmony,
        Self::Library,
        Self::Audio,
        Self::Playback,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Appearance => "Appearance",
            Self::Harmony => "Harmony",
            Self::Library => "Library",
            Self::Audio => "Audio",
            Self::Playback => "Playback",
        }
    }
}

#[derive(Default)]
pub(super) struct SettingsView {
    page: Page,
    style_defaults: StyleParts,
}

fn minor_numerals_id() -> egui::Id {
    egui::Id::unique("minor-numerals")
}

/// How Roman numerals count the degrees of a minor key, as chosen in
/// Settings > Harmony.
pub(super) fn minor_numerals(ctx: &egui::Context) -> MinorNumerals {
    ctx.data_mut(|data| data.get_persisted(minor_numerals_id()).unwrap_or_default())
}

fn minor_numerals_ui(ui: &mut egui::Ui) {
    card(
        ui,
        "Roman numerals in minor keys",
        "Both ways are correct. They name the same chords and differ only in \
         the chords on the third, sixth and seventh degrees of a minor key, \
         as here in A minor.",
        |ui| {
            let mut numerals = minor_numerals(ui.ctx());
            for (value, label, example) in [
                (
                    MinorNumerals::FromMajor,
                    "bIII bVI bVII: counted from the major scale (pop and jazz)",
                    "C is bIII, F is bVI and G is bVII; the leading-tone chord G#° is vii°.",
                ),
                (
                    MinorNumerals::FromMinor,
                    "III VI VII: counted from the minor scale (classical harmony)",
                    "C is III, F is VI and G is VII; G#° is still vii°, and E major is V.",
                ),
            ] {
                ui.radio_value(&mut numerals, value, label);
                ui.indent(label, |ui| {
                    ui.label(RichText::new(example).color(theme::TEXT_DIM));
                });
            }
            ui.add_space(4.0);
            ui.label(
                RichText::new(
                    "Applies to every view that shows numerals, and to numerals you \
                     type or paste on the chord sheet. A style's Progression box is \
                     always written bIII bVI bVII, as the library writes it.",
                )
                .small()
                .color(theme::TEXT_DIM),
            );
            ui.ctx()
                .data_mut(|data| data.insert_persisted(minor_numerals_id(), numerals));
        },
    );
}

/// A titled group with enough space to distinguish preferences from their help.
pub(super) fn card(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::new()
        .fill(theme::PANEL)
        .stroke(egui::Stroke::new(1.0, theme::BUTTON))
        .corner_radius(6)
        .inner_margin(14)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.strong(title);
            if !description.is_empty() {
                ui.label(RichText::new(description).color(theme::TEXT_DIM));
            }
            ui.add_space(6.0);
            contents(ui);
        });
}

impl LineupApp {
    pub(super) fn settings_window(&mut self, ctx: &egui::Context) {
        if !self.settings_open {
            return;
        }
        let mut open = true;
        let available = (ctx.content_rect().size() - vec2(32.0, 64.0)).max(vec2(280.0, 180.0));
        let id = egui::Id::unique("application-settings");
        let window = egui::Window::new("Settings")
            .id(id)
            .open(&mut open)
            .collapsible(false)
            .min_size(vec2(500.0, 350.0).min(available))
            .max_size(available);
        let size = vec2(800.0, 540.0).min(available);
        widgets::open_at(ctx, window, id, size, Opening::FirstTime).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(12.0, 12.0);
            egui::Panel::bottom("settings-footer")
                .exact_size(48.0)
                .resizable(false)
                .frame(egui::Frame::NONE)
                .show(ui, |ui| {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Preferences are saved automatically.")
                                .small()
                                .color(theme::TEXT_DIM),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add_sized(vec2(80.0, 28.0), egui::Button::new("Done"))
                                .clicked()
                            {
                                self.settings_open = false;
                            }
                        });
                    });
                });
            let narrow = ui.available_width() < 620.0;
            if narrow {
                ui.horizontal_wrapped(|ui| self.settings_navigation(ui, false));
                ui.separator();
            }
            let height = ui.available_height().max(80.0);
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), height),
                Layout::left_to_right(Align::Min),
                |ui| {
                    if !narrow {
                        egui::Frame::new()
                            .fill(theme::GROUP)
                            .corner_radius(6)
                            .inner_margin(8)
                            .show(ui, |ui| {
                                ui.set_width(136.0);
                                ui.set_min_height((height - 16.0).max(0.0));
                                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                                    self.settings_navigation(ui, true);
                                });
                            });
                    }
                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical()
                            .id_salt(("settings-page", self.settings.page))
                            .auto_shrink([false, false])
                            .max_height(height)
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 12.0;
                                match self.settings.page {
                                    Page::Appearance => {
                                        ui.heading("Appearance");
                                        view::zoom_settings_ui(ui);
                                    }
                                    Page::Harmony => {
                                        ui.heading("Harmony");
                                        minor_numerals_ui(ui);
                                    }
                                    Page::Library => self.library_settings_ui(ui),
                                    Page::Audio => {
                                        ui.heading("Audio");
                                        card(
                                            ui,
                                            "Output",
                                            "Uses your system’s default audio device.",
                                            |ui| {
                                                self.audio_status(ui);
                                                ui.add_space(6.0);
                                                let label = match &self.audio {
                                                    super::Audio::Running(engine)
                                                        if !engine.is_broken() =>
                                                    {
                                                        "Restart audio"
                                                    }
                                                    _ => "Try again",
                                                };
                                                if ui
                                                    .button(label)
                                                    .on_hover_text(
                                                        "Open the system’s default output \
                                                         device again",
                                                    )
                                                    .clicked()
                                                {
                                                    self.start_audio();
                                                }
                                            },
                                        );
                                        self.sound_settings_ui(ui);
                                    }
                                    Page::Playback => self.playback_settings_ui(ui),
                                }
                            })
                    });
                },
            );
        });
        self.settings_open &= open;
    }

    fn settings_navigation(&mut self, ui: &mut egui::Ui, sidebar: bool) {
        for page in Page::ALL {
            let button = egui::Button::new(page.name())
                .selected(self.settings.page == page)
                .min_size(vec2(if sidebar { 136.0 } else { 0.0 }, 34.0));
            if ui.add(button).clicked() {
                self.settings.page = page;
            }
        }
    }

    fn library_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Style library");
        card(
            ui,
            "Style list",
            "Presentation in the Library and Progression browsers.",
            |ui| {
                let mut grouped = grouped_presets(ui.ctx());
                ui.radio_value(&mut grouped, false, "Alphabetical list");
                ui.radio_value(&mut grouped, true, "Group by genre");
                set_grouped_presets(ui.ctx(), grouped);
            },
        );
        card(
            ui,
            "Style application defaults",
            "Start each style’s popup with your last confirmed options.",
            |ui| {
                let parts = self.playlist.library_parts_mut();
                style_mode_ui(ui, parts);
                ui.strong("Apply by default");
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                    for (label, value, visible) in [
                        (
                            if parts.matching_style {
                                "Patterns"
                            } else {
                                "Accents + drums"
                            },
                            &mut parts.rhythm,
                            true,
                        ),
                        ("Instruments", &mut parts.instruments, parts.matching_style),
                        ("Chords", &mut parts.progression, parts.matching_style),
                        ("Meter", &mut parts.time_signature, true),
                        ("Tempo", &mut parts.tempo, true),
                        ("Swing", &mut parts.swing, true),
                    ] {
                        if visible && theme::light_button(ui, label, *value, None).clicked() {
                            *value = !*value;
                        }
                    }
                });
                let edit = ui.button("Edit defaults…");
                if edit.clicked() {
                    self.settings.style_defaults = *parts;
                }
                egui::Popup::from_toggle_button_response(&edit)
                    .width(340.0)
                    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                    .show(|ui| {
                        ui.strong("Default style options");
                        egui::ScrollArea::vertical()
                            .max_height((ui.ctx().content_rect().height() - 160.0).max(80.0))
                            .show(ui, |ui| {
                                style_parts_checkboxes(ui, &mut self.settings.style_defaults)
                            });
                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button("Save defaults").clicked() {
                                *self.playlist.library_parts_mut() = self.settings.style_defaults;
                                ui.close();
                            }
                            if ui.button("Cancel").clicked() {
                                ui.close();
                            }
                        });
                    });
                ui.label(
                    RichText::new("Click a style to review. Ctrl-click to apply immediately.")
                        .small()
                        .color(theme::TEXT_DIM),
                );
            },
        );
        card(
            ui,
            "Your styles",
            "Choose where your saved styles are stored.",
            |ui| {
                if ui.button("Styles folder…").clicked() {
                    self.playlist.library_browser_mut().editor_request =
                        Some(StyleEditorRequest::Folder);
                }
            },
        );
    }

    fn playback_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Playback");
        card(
            ui,
            "Playhead",
            "Keep the music in view as it plays.",
            crate::widgets::follow_control,
        );
        card(
            ui,
            "Practice",
            "Clicks are for monitoring and stay out of saved songs and exports.",
            |ui| {
                let practice = self.practice.view();
                let mut count_in = practice.count_in;
                if ui.checkbox(&mut count_in, "Count-in").changed() {
                    self.set_count_in(count_in);
                }
                ui.label(
                    RichText::new("One bar of clicks before playback starts.")
                        .small()
                        .color(theme::TEXT_DIM),
                );
                let mut metronome = practice.metronome;
                if ui.checkbox(&mut metronome, "Metronome").changed() {
                    self.set_metronome(metronome);
                }
                ui.label(
                    RichText::new("A click on every beat during playback.")
                        .small()
                        .color(theme::TEXT_DIM),
                );
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    ui.label("Practice speed");
                    let mut speed = practice.speed;
                    if ui
                        .add(
                            egui::DragValue::new(&mut speed)
                                .range(crate::chord_sheet::SPEEDS)
                                .suffix("%"),
                        )
                        .changed()
                    {
                        self.set_speed(speed);
                    }
                    if ui
                        .add_enabled(speed != 100, egui::Button::new("Reset"))
                        .clicked()
                    {
                        self.set_speed(100);
                    }
                });
                ui.label(
                    RichText::new("Relative to the song tempo. Starts at 100% each session.")
                        .small()
                        .color(theme::TEXT_DIM),
                );
                ui.separator();
                let mut training = practice.training;
                if ui.checkbox(&mut training, "Speed trainer").changed() {
                    self.set_training(training);
                }
                ui.horizontal_wrapped(|ui| {
                    if let Some(trainer) =
                        crate::chord_sheet::trainer_controls(ui, practice.trainer)
                    {
                        self.set_trainer(trainer);
                    }
                });
                ui.label(
                    RichText::new(
                        "Each time the loop comes round, the band plays a little faster, up \
                         to the target. Stop starts again from the first speed.",
                    )
                    .small()
                    .color(theme::TEXT_DIM),
                );
            },
        );
        card(
            ui,
            "Trace",
            "Writes every note played to a text file, to check playback by reading it.",
            |ui| {
                let mut on = self.trace.is_some();
                let can = matches!(self.audio, super::Audio::Running(_));
                if ui
                    .add_enabled(
                        can,
                        egui::Checkbox::new(&mut on, "Write a trace while playing"),
                    )
                    .changed()
                {
                    self.set_tracing(on);
                }
                let note = match &self.trace {
                    Some(trace) => format!("Writing to {}", trace.path().display()),
                    None => "Off each time the app starts.".to_string(),
                };
                ui.label(RichText::new(note).small().color(theme::TEXT_DIM));
            },
        );
    }
}
