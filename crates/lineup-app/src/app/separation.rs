//! Source-separation preferences, separate from the song being edited.

use std::collections::BTreeMap;
use std::path::PathBuf;

use eframe::egui;
use lineup_listen::separation::{Backend, Config};
use serde::{Deserialize, Serialize};

use super::preferences::{self, Preferences};
use crate::i18n::{tr, trf};

const KEY: &str = "take source separation";

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct Paths {
    program: String,
    model: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    selected: Option<String>,
    helpers: BTreeMap<String, Paths>,
}

fn bundled_folder() -> Option<PathBuf> {
    Some(std::env::current_exe().ok()?.parent()?.join("separators"))
}

fn default_program(backend: Backend) -> PathBuf {
    let name = format!("{}{}", backend.id(), std::env::consts::EXE_SUFFIX);
    if let Some(path) = bundled_folder().map(|folder| folder.join("bin").join(&name))
        && path.is_file()
    {
        return path;
    }
    PathBuf::from(name)
}

fn default_model() -> PathBuf {
    bundled_folder().unwrap_or_default().join("htdemucs.onnx")
}

impl Settings {
    fn configuration(&self) -> Result<Option<Config>, String> {
        let Some(id) = &self.selected else {
            return Ok(None);
        };
        let backend = Backend::ALL
            .into_iter()
            .find(|backend| backend.id() == id)
            .ok_or_else(|| trf("Unknown separator: {}", &[id]))?;
        let paths = self.helpers.get(id).cloned().unwrap_or_default();
        let config = Config {
            backend,
            program: if paths.program.is_empty() {
                default_program(backend)
            } else {
                PathBuf::from(paths.program)
            },
            model: (backend == Backend::Charon).then(|| {
                if paths.model.is_empty() {
                    default_model()
                } else {
                    PathBuf::from(paths.model)
                }
            }),
        };
        config.validate().map_err(|error| error.to_string())?;
        Ok(Some(config))
    }
}

pub(super) fn configuration(preferences: &Preferences) -> Result<Option<Config>, String> {
    preferences
        .get::<Settings>(KEY)
        .unwrap_or_default()
        .configuration()
}

pub(super) fn settings_ui(ui: &mut egui::Ui) {
    let mut settings: Settings = preferences::get(ui.ctx(), KEY).unwrap_or_default();
    let available: Vec<_> = Backend::available().collect();
    // A choice from another build remains visible so it can be turned
    // off rather than silently ignored or left impossible to change.
    if available.is_empty() && settings.selected.is_none() {
        return;
    }
    let before = settings.clone();
    super::settings::card(
        ui,
        "Source separation",
        "Separate recordings with a local helper before extracting notes.",
        |ui| {
            ui.radio_value(&mut settings.selected, None, tr("Off"));
            for backend in available {
                ui.radio_value(
                    &mut settings.selected,
                    Some(backend.id().to_string()),
                    backend.name(),
                );
            }
            if let Some(backend) = Backend::ALL.into_iter().find(|backend| {
                backend.compiled() && settings.selected.as_deref() == Some(backend.id())
            }) {
                let paths = settings
                    .helpers
                    .entry(backend.id().to_string())
                    .or_default();
                ui.push_id(backend.id(), |ui| {
                    ui.label(tr("Separator executable"));
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut paths.program)
                                .hint_text(default_program(backend).display().to_string())
                                .desired_width(280.0),
                        );
                        if ui.button(tr("Choose executable")).clicked()
                            && let Some(path) = rfd::FileDialog::new().pick_file()
                        {
                            paths.program = path.to_string_lossy().into_owned();
                        }
                    });
                    if backend == Backend::Charon {
                        ui.label(tr("HTDemucs model"));
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut paths.model)
                                    .hint_text(default_model().display().to_string())
                                    .desired_width(280.0),
                            );
                            if ui.button(tr("Choose model")).clicked()
                                && let Some(path) = rfd::FileDialog::new()
                                    .add_filter("ONNX", &["onnx"])
                                    .pick_file()
                            {
                                paths.model = path.to_string_lossy().into_owned();
                            }
                        });
                    }
                });
                ui.label(tr("Leave paths blank to use bundled helpers, then PATH."));
                ui.label(tr("Audio stays on this computer. Helpers may download model weights on first use."));
                ui.label(tr("Melody uses vocals; chords use other instruments. Turn off for instrumental melodies."));
                ui.label(tr(
                    "Applies to the next analysis, including Transcribe again.",
                ));
            }
            if let Err(error) = settings.configuration() {
                ui.colored_label(ui.visuals().error_fg_color, error);
            }
        },
    );
    if settings != before {
        preferences::set(ui.ctx(), KEY, &settings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separation_is_off_until_chosen() {
        assert!(Settings::default().configuration().unwrap().is_none());
    }

    #[test]
    fn separation_settings_show_only_compiled_helpers() {
        let ctx = egui::Context::default();
        let draw = || {
            let mut output = ctx.run_pass(egui::RawInput::default(), |ctx| {
                let mut ui = egui::Ui::new(
                    ctx.clone(),
                    egui::Id::unique("separation-test"),
                    egui::UiBuilder::new().max_rect(ctx.content_rect()),
                );
                settings_ui(&mut ui);
            });
            output.textures_delta.clear();
            output
        };
        draw();
        let output = draw();
        let texts: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| {
                if let egui::Shape::Text(text) = &shape.shape {
                    Some(text.galley.text())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(
            texts.contains(&"Source separation"),
            Backend::available().next().is_some()
        );
        for backend in Backend::ALL {
            assert_eq!(texts.contains(&backend.name()), backend.compiled());
        }
    }

    #[test]
    fn separator_choices_and_each_helpers_paths_survive_restarts() {
        let ctx = egui::Context::default();
        let settings = Settings {
            selected: Some("demucs".into()),
            helpers: BTreeMap::from([
                (
                    "demucs".into(),
                    Paths {
                        program: "/some/demucs cpu".into(),
                        model: String::new(),
                    },
                ),
                (
                    "charon".into(),
                    Paths {
                        program: "charon".into(),
                        model: "/models/htdemucs.onnx".into(),
                    },
                ),
            ]),
        };
        preferences::set(&ctx, KEY, &settings);
        let mut storage = preferences::Stored::default();
        Preferences::of(&ctx).save(&mut storage);
        let next = egui::Context::default();
        Preferences::of(&next).load(&storage);
        assert_eq!(preferences::get::<Settings>(&next, KEY), Some(settings));
    }

    #[test]
    fn a_saved_unavailable_backend_does_not_silently_analyze_the_mix() {
        for backend in Backend::ALL
            .into_iter()
            .filter(|backend| !backend.compiled())
        {
            let settings = Settings {
                selected: Some(backend.id().into()),
                ..Settings::default()
            };
            assert!(
                settings
                    .configuration()
                    .unwrap_err()
                    .contains(backend.name())
            );
        }
        let settings = Settings {
            selected: Some("unknown".into()),
            ..Settings::default()
        };
        assert!(settings.configuration().unwrap_err().contains("unknown"));
    }
}
