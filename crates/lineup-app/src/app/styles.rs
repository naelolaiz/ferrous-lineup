//! User style storage and the connection between style drafts and songs.

use super::{APP_NAME, LineupApp};
use crate::library_browser::{StyleEditorRequest, Tried};
use crate::style_editor::EditorEvent;
use eframe::egui;
use lineup_audio::phrase::PhraseNote;
use lineup_audio::{Command, Phrase};
use lineup_core::library::apply;
use lineup_core::library::{UserStyle, user_style::EXTENSION};
use lineup_core::{Project, Source, SourceKind};
use std::path::{Path, PathBuf};

pub(super) struct StyleFiles {
    pub(super) folder: PathBuf,
    paths: Vec<PathBuf>,
}

impl StyleFiles {
    pub fn new(ctx: &egui::Context) -> Self {
        let saved =
            ctx.data_mut(|data| data.get_persisted::<String>(egui::Id::unique("styles-folder")));
        let folder = saved.map(PathBuf::from).unwrap_or_else(|| {
            eframe::storage_dir(APP_NAME)
                .unwrap_or_else(|| PathBuf::from("."))
                .join("styles")
        });
        Self {
            folder,
            paths: Vec::new(),
        }
    }
}

#[derive(Default)]
struct LoadedStyles {
    styles: Vec<UserStyle>,
    paths: Vec<PathBuf>,
    errors: Vec<String>,
}

fn read_styles(folder: &Path) -> Result<LoadedStyles, String> {
    if !folder.exists() {
        return Ok(LoadedStyles::default());
    }
    let entries = std::fs::read_dir(folder).map_err(|error| error.to_string())?;
    let mut styles = Vec::new();
    let mut errors = Vec::new();
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path
            .extension()
            .is_none_or(|extension| extension != EXTENSION)
        {
            continue;
        }
        match std::fs::read_to_string(&path)
            .map_err(|error| error.to_string())
            .and_then(|text| UserStyle::from_text(&text))
        {
            Ok(style) => styles.push((style, path)),
            Err(error) => errors.push(format!("{}: {error}", path.display())),
        }
    }
    styles.sort_by_cached_key(|(style, _)| style.name.to_lowercase());
    let (styles, paths) = styles.into_iter().unzip();
    Ok(LoadedStyles {
        styles,
        paths,
        errors,
    })
}

impl LineupApp {
    pub(super) fn reload_user_styles(&mut self) {
        match read_styles(&self.style_files.folder) {
            Ok(loaded) => {
                self.playlist.library_browser_mut().user_styles = loaded.styles;
                self.style_files.paths = loaded.paths;
                if !loaded.errors.is_empty() {
                    self.message = Some(crate::i18n::trf(
                        "Some styles could not be loaded: {}",
                        &[&loaded.errors.join("; ")],
                    ));
                }
            }
            Err(error) => {
                self.playlist.library_browser_mut().user_styles.clear();
                self.style_files.paths.clear();
                self.message = Some(crate::i18n::trf("Could not load styles: {}", &[&error]));
            }
        }
    }
    pub(super) fn start_style_editor(&mut self, style: UserStyle, path: Option<PathBuf>) {
        self.style_editor.start(style, path);
        self.bring_to_front = Some(crate::style_editor::window_id());
    }
    pub(super) fn edit_current_style(&mut self) {
        if let Some(style) = self
            .project
            .band()
            .and_then(|band| band.custom_rhythm.as_deref())
            .cloned()
        {
            let path = self
                .playlist
                .library_browser_mut()
                .user_styles
                .iter()
                .position(|user| user.id() == style.id())
                .and_then(|index| self.style_files.paths.get(index).cloned());
            self.start_style_editor(style, path);
        } else if let Some(band) = self.project.band() {
            // The whole band the song plays, not only its drums.
            let style = UserStyle::from_band(band.style, band.harmony_style);
            self.start_style_editor(style, None);
        } else if let Some(style) = self.tried().style {
            self.start_style_editor(UserStyle::from_builtin(style), None);
        } else if let Some(rhythm) = self.tried().rhythm {
            self.start_style_editor(UserStyle::from_rhythm(rhythm), None);
        } else {
            self.start_style_editor(UserStyle::blank(), None);
        }
    }
    pub(super) fn style_editor_window(&mut self, ctx: &egui::Context, area: egui::Rect) {
        if let Some(request) = self.playlist.library_browser_mut().editor_request.take() {
            egui::Popup::close_all(ctx);
            match request {
                StyleEditorRequest::New => self.start_style_editor(UserStyle::blank(), None),
                StyleEditorRequest::Builtin(style) => {
                    self.start_style_editor(UserStyle::from_builtin(style), None)
                }
                StyleEditorRequest::Rhythm(rhythm) => {
                    self.start_style_editor(UserStyle::from_rhythm(rhythm), None)
                }
                StyleEditorRequest::User(index) => {
                    if let Some(style) = self
                        .playlist
                        .library_browser_mut()
                        .user_styles
                        .get(index)
                        .cloned()
                    {
                        self.start_style_editor(style, self.style_files.paths.get(index).cloned());
                    }
                }
                StyleEditorRequest::Folder => {
                    if let Some(folder) = rfd::FileDialog::new()
                        .set_directory(&self.style_files.folder)
                        .pick_folder()
                    {
                        ctx.data_mut(|data| {
                            data.insert_persisted(
                                egui::Id::unique("styles-folder"),
                                folder.to_string_lossy().into_owned(),
                            )
                        });
                        self.style_files.folder = folder;
                        self.reload_user_styles();
                    }
                }
            }
        }
        let events =
            self.style_editor
                .show(ctx, area, &self.playlist.library_browser_mut().user_styles);
        for event in events {
            match event {
                EditorEvent::Save { style, save_as } => self.save_user_style(style, save_as),
                EditorEvent::Preview(project) => self.preview_style(&project),
                EditorEvent::Stop => self.send(Command::StopPhrase),
                EditorEvent::StartFrom(style) => self.start_style_editor(style, None),
                EditorEvent::ConnectMidiOutput(port) => self.connect_midi_output(port.as_ref()),
                EditorEvent::Hear { channel, pitches } => {
                    let rate = self.sample_rate();
                    let instrument = self.build_instrument(&channel, rate);
                    let notes: Vec<_> = pitches
                        .into_iter()
                        .map(|pitch| PhraseNote {
                            instrument: 0,
                            start: 0.0,
                            length: 0.35,
                            pitch,
                            velocity: 96,
                        })
                        .collect();
                    self.send(Command::PlayPhrase(Box::new(Phrase::new(
                        vec![instrument],
                        &notes,
                        rate,
                    ))));
                }
            }
        }
        ctx.data_mut(|data| {
            data.insert_persisted(
                egui::Id::unique("styles-folder"),
                self.style_files.folder.to_string_lossy().into_owned(),
            )
        });
    }
    pub(super) fn save_user_style(&mut self, style: UserStyle, save_as: bool) {
        let text = match style.to_text() {
            Ok(text) => text,
            Err(error) => {
                self.style_editor.error = Some(error);
                return;
            }
        };
        let path = if !save_as {
            self.style_editor.path.clone()
        } else {
            None
        };
        let path = path.or_else(|| {
            let _ = std::fs::create_dir_all(&self.style_files.folder);
            let name = format!("{}.{}", style.name.replace(['/', '\\'], "-"), EXTENSION);
            rfd::FileDialog::new()
                .set_directory(&self.style_files.folder)
                .set_file_name(name)
                .add_filter(crate::i18n::tr("Lineup style"), &[EXTENSION])
                .save_file()
        });
        let Some(mut path) = path else {
            return;
        };
        if path
            .extension()
            .is_none_or(|extension| extension != EXTENSION)
        {
            path.set_extension(EXTENSION);
        }
        let temporary = path.with_extension(format!("{EXTENSION}.tmp"));
        if let Err(error) =
            std::fs::write(&temporary, text).and_then(|()| std::fs::rename(&temporary, &path))
        {
            self.style_editor.error = Some(crate::i18n::trf(
                "Could not save {}: {}",
                &[&path.display(), &error],
            ));
            return;
        }
        self.style_editor.saved(path.clone());
        // Saving one style elsewhere leaves the styles folder as it is,
        // with every other style in it; Settings changes the folder.
        self.reload_user_styles();
        self.notice = Some(
            if path.parent() == Some(self.style_files.folder.as_path()) {
                crate::i18n::trf("Saved style: {}", &[&path.display()])
            } else {
                crate::i18n::trf(
                    "Saved style: {}. The Library lists the styles in {}; Settings > Styles \
                 folder changes it.",
                    &[&path.display(), &self.style_files.folder.display()],
                )
            },
        );
    }
    fn preview_style(&mut self, project: &Project) {
        if self.playing {
            self.pause();
        }
        let rate = self.sample_rate();
        let instruments = project
            .channels()
            .iter()
            .map(|channel| self.build_instrument(channel, rate))
            .collect();
        let seconds = 60.0 / f64::from(project.tempo.bpm()) / 4.0;
        let period = project.pattern(0).unwrap().length() as f64 * seconds;
        let mut notes = Vec::new();
        for repeat in 0..4 {
            for (channel, state) in project
                .channels()
                .iter()
                .enumerate()
                .filter(|(_, state)| !state.muted)
            {
                for note in project.played_notes(0, channel).iter() {
                    notes.push(PhraseNote {
                        instrument: channel,
                        start: f64::from(repeat) * period
                            + note.start_tick() as f64 * seconds
                                / lineup_core::TICKS_PER_STEP as f64,
                        length: note.ticks() as f64 * seconds / lineup_core::TICKS_PER_STEP as f64,
                        pitch: note.pitch,
                        velocity: (f32::from(note.velocity) * state.volume)
                            .round()
                            .clamp(1.0, 127.0) as u8,
                    });
                }
            }
        }
        self.send(Command::PlayPhrase(Box::new(Phrase::new(
            instruments,
            &notes,
            rate,
        ))));
    }
    pub(super) fn apply_user_library(&mut self, index: usize, progression_from: Option<usize>) {
        let Some(style) = self
            .playlist
            .library_browser_mut()
            .user_styles
            .get(index)
            .cloned()
        else {
            return;
        };
        let mut parts = self.playlist.library_parts();
        // A saved default is a preference, not a chart supplied by this preset.
        parts.progression &= !style.chart.trim().is_empty();
        if progression_from.is_none()
            && !(parts.rhythm
                || parts.instruments
                || parts.time_signature
                || parts.progression
                || parts.tempo
                || parts.swing)
        {
            self.notice =
                Some(crate::i18n::tr("Choose a component to apply in the Library options.").into());
            return;
        }
        let before = self.project.clone();
        let previous = self.tried();
        let source = Source::new(SourceKind::SavedStyle, &style.name);
        // Writing its progression from a bar brings in its chords alone,
        // whichever playing style the Library options have.
        let taken = match progression_from {
            Some(_) => apply::StyleParts {
                progression: true,
                matching_style: true,
                ..parts
            },
            None => parts,
        };
        let result = self.edit_song(
            "apply user style",
            progression_from.is_none().then_some(style.time),
            taken,
            true,
            Some(super::library::Bringing::from(&source)),
            |project, parts| {
                if let Some(from) = progression_from {
                    project.write_user_progression(&style, from).map(|_| ())
                } else {
                    project.apply_user_style(&style, parts)
                }
            },
        );
        if let Err(error) = result {
            self.message = Some(error);
            return;
        }
        self.song_library = self
            .project
            .band()
            .map_or_else(Tried::default, Tried::from_band);
        self.keep_place_in_song(&before);
        self.remember_song_version(before, previous);
        self.remember_song_version(self.project.clone(), self.song_library);
        if progression_from.is_none() {
            self.set_mode(lineup_audio::PlayMode::Song);
        }
        if self.project.band().is_none() && parts.instruments && !parts.rhythm {
            self.notice = Some(crate::i18n::trf(
                "{}'s instruments play in a band: tick Playing patterns too.",
                &[&style.name],
            ));
        } else if self.project.playlist().chord_markers().is_empty() {
            self.notice = Some(crate::i18n::tr(if parts.matching_style {
                "Style applied. Write chords on the sheet or enable Chord progression when applying it."
            } else {
                "Style applied. Plain accents play the chords on the sheet: write some there."
            }).into());
        } else {
            self.notice = Some(crate::i18n::trf("Applied {}", &[&style.name]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn saved_styles_reload_alphabetically_with_their_instruments_and_patterns() {
        let directory =
            std::env::temp_dir().join(format!("lineup-styles-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let mut zeta = UserStyle::from_builtin(&lineup_core::library::STYLES[0]);
        zeta.name = "Zeta".into();
        let mut alpha = zeta.clone();
        alpha.name = "Alpha".into();
        std::fs::write(
            directory.join(format!("z.{EXTENSION}")),
            zeta.to_text().unwrap(),
        )
        .unwrap();
        std::fs::write(
            directory.join(format!("a.{EXTENSION}")),
            alpha.to_text().unwrap(),
        )
        .unwrap();
        std::fs::write(directory.join(format!("broken.{EXTENSION}")), "invalid").unwrap();
        let loaded = read_styles(&directory).unwrap();
        assert_eq!(loaded.styles, vec![alpha, zeta]);
        assert_eq!(loaded.paths.len(), 2);
        assert_eq!(loaded.errors.len(), 1);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
