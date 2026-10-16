//! Trying styles and rhythms from the library, and opening the demos.
//!
//! A click in the library writes into a pattern and plays it. Clicking
//! another style or rhythm while that pattern is selected, and unchanged,
//! rewrites the same pattern, so you can flick through the library
//! without filling the project with patterns. Once you change the
//! pattern, it is yours: the next click starts a new one.
//!
//! The app remembers every pattern the library wrote, not just the last,
//! so that after Ctrl+Z brings back an earlier one, the next click still
//! rewrites it.

use eframe::egui;
use loom_audio::PlayMode;
use loom_core::library::demo::{song_demo, style_demo};
use loom_core::library::{Genre, Rhythm, SONGS, STYLES, Song, Style, apply};
use loom_core::{Pattern, Project, Tempo};

use super::LoomApp;
use super::files::Pending;
use crate::library_browser::{LibraryAction, Tried};

/// How many of the patterns the library wrote are remembered.
const REMEMBERED: usize = 64;

/// A pattern the library wrote, and what it wrote there.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Audition {
    pattern: usize,
    /// The pattern as the library left it. If it still looks like this,
    /// the next click may rewrite it.
    written: Pattern,
    style: Option<&'static Style>,
    rhythm: Option<&'static Rhythm>,
    /// Whether the rhythm was picked, or came with the style. A rhythm
    /// that came with a style changes with the next style.
    rhythm_picked: bool,
}

/// A demo to open from the File menu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Demo {
    /// The song Ferrous Loom starts with.
    Start,
    Song(&'static Song),
    Style(&'static Style),
}

impl Demo {
    pub(super) fn project(self) -> Project {
        match self {
            Demo::Start => Project::demo(),
            Demo::Song(song) => song_demo(song),
            Demo::Style(style) => style_demo(style),
        }
    }
}

impl LoomApp {
    /// What the library put in the selected pattern, if it is unchanged.
    pub(super) fn tried(&self) -> Tried {
        match self.unchanged_audition() {
            Some(audition) => Tried {
                style: audition.style,
                rhythm: audition.rhythm,
            },
            None => Tried::default(),
        }
    }

    /// What the library wrote in the selected pattern, if it is still
    /// exactly as the library left it.
    fn unchanged_audition(&self) -> Option<&Audition> {
        let pattern = self.project.pattern(self.pattern)?;
        self.auditions
            .iter()
            .rev()
            .find(|audition| audition.pattern == self.pattern && &audition.written == pattern)
    }

    /// Writes a style or a rhythm into a pattern, selects it and plays it.
    pub(super) fn try_from_library(&mut self, action: LibraryAction) {
        let reuse = self.unchanged_audition().cloned();
        let mut style = reuse.as_ref().and_then(|a| a.style);
        let mut rhythm = reuse.as_ref().and_then(|a| a.rhythm);
        let mut rhythm_picked = reuse.as_ref().is_some_and(|a| a.rhythm_picked);
        match action {
            LibraryAction::Style(picked) => {
                style = Some(picked);
                if !rhythm_picked {
                    rhythm = Rhythm::named(picked.rhythm);
                }
            }
            LibraryAction::Rhythm(picked) => {
                rhythm = Some(picked);
                rhythm_picked = true;
            }
        }

        let name = match (style, rhythm) {
            (Some(style), Some(rhythm)) if style.name != rhythm.name => {
                format!("{} · {}", style.name, rhythm.name)
            }
            (Some(style), _) => style.name.to_string(),
            (None, Some(rhythm)) => rhythm.name.to_string(),
            (None, None) => return,
        };
        let target = reuse.as_ref().map(|a| a.pattern);
        let selected = self.piano_roll.channel();
        let tonic = self.project.scale.root;
        let use_tempo = self.playlist.library_tempo();
        let written = self.edit("try from the library", |project| {
            let index = match target {
                Some(index) => index,
                None => project.add_pattern()?,
            };
            let pattern = project.pattern_mut(index)?;
            pattern.name = name;
            for channel in 0..pattern.rows() {
                pattern.set_notes(channel, []);
            }
            match style {
                // The style decides the bars: its time signature and length.
                Some(style) => {
                    apply::write_style(project, index, style, tonic, Some(selected));
                }
                // A rhythm alone brings its own.
                None => {
                    if let Some(rhythm) = rhythm {
                        project.time_signature = rhythm.time;
                        let length = rhythm.bars() * rhythm.time.steps_per_bar();
                        project.pattern_mut(index)?.set_length(length);
                    }
                }
            }
            if let Some(rhythm) = rhythm {
                apply::write_rhythm(project, index, rhythm);
            }
            if use_tempo {
                let bpm = style.map_or_else(|| rhythm.map(|r| r.tempo), |s| Some(s.tempo));
                if let Some(bpm) = bpm {
                    project.tempo = Tempo::new(bpm);
                }
            }
            Some((index, project.pattern(index)?.clone()))
        });
        let Some((pattern, written)) = written else {
            self.message = Some("There is no room for another pattern.".to_string());
            return;
        };
        self.select_pattern(pattern);
        if self.auditions.len() == REMEMBERED {
            self.auditions.remove(0);
        }
        self.auditions.push(Audition {
            pattern,
            written,
            style,
            rhythm,
            rhythm_picked,
        });
        self.set_mode(PlayMode::Pattern);
        if !self.playing {
            self.set_playing(true);
        }
    }

    /// The Demos submenu of the File menu.
    pub(super) fn demos_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let mut picked = None;
        ui.menu_button("Demos", |ui| {
            if ui
                .button("The starting song")
                .on_hover_text("The beat and chords Ferrous Loom opens with")
                .clicked()
            {
                picked = Some(Demo::Start);
            }
            ui.separator();
            ui.menu_button("Songs", |ui| {
                ui.weak("Well-known tunes, all in the public domain");
                for song in SONGS {
                    let hover = format!("{}\n\n{}", song.origin, song.about);
                    if ui
                        .button(format!("{} · {}", song.name, song.feel))
                        .on_hover_text(hover)
                        .clicked()
                    {
                        picked = Some(Demo::Song(song));
                    }
                }
            });
            ui.menu_button("Styles", |ui| {
                ui.weak("Each harmony style as a short song");
                for genre in Genre::ALL {
                    ui.menu_button(genre.name(), |ui| {
                        for style in STYLES.iter().filter(|style| style.genre == genre) {
                            if ui.button(style.name).on_hover_text(style.about).clicked() {
                                picked = Some(Demo::Style(style));
                            }
                        }
                    });
                }
            });
        });
        if let Some(demo) = picked {
            ui.close();
            self.request(&ctx, Pending::Demo(demo));
        }
    }

    /// Opens a demo in place of the project, and plays it.
    pub(super) fn open_demo(&mut self, demo: Demo) {
        self.replace_project(demo.project(), None);
        self.set_mode(PlayMode::Song);
        self.set_playing(true);
    }
}
