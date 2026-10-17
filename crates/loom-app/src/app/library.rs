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
//!
//! A rhythm picked after a style changes how the style's chords and bass
//! are played, too, as in FL Studio. And a rhythm can be used for the
//! whole song: see [`apply::use_rhythm`]. That is what a click on a
//! rhythm does while the song plays, so the song carries on in the new
//! rhythm instead of stopping for a pattern of its own.

use eframe::egui;
use loom_audio::PlayMode;
use loom_core::library::demo::{row_demo, song_demo, style_demo};
use loom_core::library::{
    FILLS, Genre, ROW_STYLES, Rhythm, RowStyle, SONGS, STYLES, Song, Style, apply, rows,
};
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
    row_style: Option<&'static RowStyle>,
}

/// A demo to open from the File menu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Demo {
    /// The song Ferrous Loom starts with.
    Start,
    Song(&'static Song),
    Style(&'static Style),
    Row(&'static RowStyle),
}

impl Demo {
    /// The style and rhythm the demo is made from.
    fn library(self) -> Tried {
        match self {
            Demo::Start => Tried::default(),
            Demo::Song(song) => Tried {
                style: Style::named(song.style),
                rhythm: Rhythm::named(song.rhythm),
                row_style: None,
            },
            Demo::Style(style) => Tried {
                style: Some(style),
                rhythm: Rhythm::named(style.rhythm),
                row_style: None,
            },
            Demo::Row(style) => Tried {
                style: None,
                rhythm: style.rhythm(),
                row_style: Some(style),
            },
        }
    }

    pub(super) fn project(self) -> Project {
        match self {
            Demo::Start => Project::demo(),
            Demo::Song(song) => song_demo(song),
            Demo::Style(style) => style_demo(style),
            Demo::Row(style) => row_demo(style),
        }
    }
}

impl LoomApp {
    /// What the library shows selected: what it put in the selected
    /// pattern, if that is unchanged, or else the style and rhythm the
    /// whole song plays, so a demo opens with its own selected, as in FL
    /// Studio.
    pub(super) fn tried(&self) -> Tried {
        match self.unchanged_audition() {
            Some(audition) => Tried {
                style: audition.style,
                rhythm: audition.rhythm,
                row_style: audition.row_style,
            },
            None => self.song_library,
        }
    }

    /// Remembers the style and rhythm the song plays in this version of
    /// the project, for undo and redo.
    fn remember_song_version(&mut self, project: Project, tried: Tried) {
        self.song_versions
            .retain(|(version, _)| *version != project);
        if self.song_versions.len() == REMEMBERED {
            self.song_versions.remove(0);
        }
        self.song_versions.push((project, tried));
    }

    /// After undo or redo, shows the style and rhythm this version of the
    /// project plays, if the library changed it.
    pub(super) fn restore_song_library(&mut self) {
        if let Some((_, tried)) = self
            .song_versions
            .iter()
            .rev()
            .find(|(version, _)| *version == self.project)
        {
            self.song_library = *tried;
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
    /// While the song plays, it changes the song instead, and the song
    /// carries on.
    pub(super) fn try_from_library(&mut self, action: LibraryAction) {
        // Taking over with a pattern of its own would stop the song, so
        // while the song plays, every click goes to the song.
        if self.playing && self.mode == PlayMode::Song {
            self.use_in_song(action);
            return;
        }
        if let LibraryAction::RhythmInSong(_) = action {
            self.use_in_song(action);
            return;
        }
        if let LibraryAction::RowStyle(style) = action {
            self.try_row_style(style);
            return;
        }
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
            LibraryAction::RhythmInSong(_) | LibraryAction::RowStyle(_) => return,
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
        // A rhythm that was picked, not the style's own, decides how the
        // chords and bass are played.
        let feel = rhythm.filter(|_| rhythm_picked);
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
                    apply::write_style_played(project, index, style, tonic, Some(selected), feel);
                }
                // A rhythm alone brings its own, to the song too if the
                // song has nothing else in it.
                None => {
                    if let Some(rhythm) = rhythm {
                        let own = apply::song_has_music(project, index);
                        if !own {
                            project.time_signature = rhythm.time;
                        }
                        let length = rhythm.bars() * rhythm.time.steps_per_bar();
                        let pattern = project.pattern_mut(index)?;
                        pattern.time = own.then_some(rhythm.time);
                        pattern.set_length(length);
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
        // A song with music of its own kept its key and time signature,
        // which may not be the style's: say so.
        if apply::song_has_music(&self.project, pattern) {
            let (key, time) = match (style, rhythm) {
                (Some(style), _) => (Some(style.scale(tonic)), style.time),
                (None, Some(rhythm)) => (None, rhythm.time),
                (None, None) => (None, self.project.time_signature),
            };
            let song = self.project.scale;
            let mut differ = Vec::new();
            if let Some(key) = key.filter(|key| *key != song) {
                differ.push((key.name(), song.name()));
            }
            if time != self.project.time_signature {
                differ.push((time.to_string(), self.project.time_signature.to_string()));
            }
            if !differ.is_empty() {
                let (theirs, ours): (Vec<String>, Vec<String>) = differ.into_iter().unzip();
                let name = &self.project.patterns()[pattern].name;
                self.message = Some(format!(
                    "\"{name}\" is in {}; the song stays in {}.",
                    theirs.join(", "),
                    ours.join(", ")
                ));
            }
        }
        if self.auditions.len() == REMEMBERED {
            self.auditions.remove(0);
        }
        self.auditions.push(Audition {
            pattern,
            written,
            style,
            rhythm,
            rhythm_picked,
            row_style: None,
        });
        self.set_mode(PlayMode::Pattern);
        if !self.playing {
            // What the library wrote plays from its start, not from where
            // the last pattern was paused.
            self.rewind_for_new_pattern();
            self.set_playing(true);
        }
    }

    /// Plays the whole song the way a style, a rhythm or a fill goes, as
    /// one step to undo, and says what changed. The song keeps its
    /// chords, its melody and its place: see [`apply::use_style`],
    /// [`apply::use_rhythm`] and [`apply::use_fill`].
    fn use_in_song(&mut self, action: LibraryAction) {
        let use_tempo = self.playlist.library_tempo();
        let (name, tempo, label) = match action {
            LibraryAction::Style(style) => (style.name, Some(style.tempo), "change the style"),
            LibraryAction::RowStyle(style) => (style.name, Some(style.tempo), "change the style"),
            LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm)
                if FILLS.contains(rhythm) =>
            {
                // A fill is one bar: the song keeps its tempo.
                (rhythm.name, None, "change the fill")
            }
            LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
                (rhythm.name, Some(rhythm.tempo), "change the rhythm")
            }
        };
        let song_library = self.song_library;
        let before = self.project.clone();
        let change = self.edit(label, |project| {
            let change = match action {
                LibraryAction::Style(style) => apply::use_style(project, style),
                LibraryAction::RowStyle(style) => rows::use_row_style(project, style),
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm)
                    if FILLS.contains(rhythm) =>
                {
                    apply::use_fill(project, rhythm)
                }
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
                    apply::use_rhythm(project, rhythm)
                }
            };
            if use_tempo
                && change != apply::RhythmChange::default()
                && let Some(bpm) = tempo
            {
                project.tempo = Tempo::new(bpm);
            }
            change
        });
        let patterns = |count: usize| match count {
            1 => "1 pattern".to_string(),
            count => format!("{count} patterns"),
        };
        let mut parts = Vec::new();
        if change.drums > 0 {
            parts.push(format!("new drums in {}", patterns(change.drums)));
        }
        if change.harmony > 0 {
            let what = match action {
                LibraryAction::RowStyle(_) => "the row instead of the chords",
                _ => "chords and bass played again",
            };
            parts.push(format!("{what} in {}", patterns(change.harmony)));
        }
        let fill = matches!(
            action,
            LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) if FILLS.contains(rhythm)
        );
        // The song now plays what was picked, so the library shows it.
        if change != apply::RhythmChange::default() {
            self.song_library = match action {
                LibraryAction::Style(style) => Tried {
                    style: Some(style),
                    row_style: None,
                    ..song_library
                },
                LibraryAction::RowStyle(style) => Tried {
                    style: None,
                    rhythm: style.rhythm().or(song_library.rhythm),
                    row_style: Some(style),
                },
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) if !fill => {
                    Tried {
                        rhythm: Some(rhythm),
                        ..song_library
                    }
                }
                _ => song_library,
            };
            self.remember_song_version(before, song_library);
            self.remember_song_version(self.project.clone(), self.song_library);
        }
        self.notice = Some(if fill && change.drums > 0 {
            format!(
                "{name} now plays in the last bar of {}. Ctrl+Z takes it back.",
                patterns(change.drums)
            )
        } else if parts.is_empty() {
            format!(
                "{name} finds nothing to change in this song: it has no drums or chords for it."
            )
        } else {
            format!(
                "The song now plays {name}: {}. Ctrl+Z takes it back.",
                parts.join(", ")
            )
        });
    }

    /// Writes a twelve-tone style into a pattern and plays it, as
    /// [`Self::try_from_library`] does for a harmony style: the same
    /// pattern is rewritten while it is unchanged. The row starts on the
    /// project's key note.
    fn try_row_style(&mut self, style: &'static RowStyle) {
        let target = self.unchanged_audition().map(|audition| audition.pattern);
        let selected = self.piano_roll.channel();
        let start = self.project.scale.root;
        let use_tempo = self.playlist.library_tempo();
        let written = self.edit("try from the library", |project| {
            let index = match target {
                Some(index) => index,
                None => project.add_pattern()?,
            };
            let pattern = project.pattern_mut(index)?;
            pattern.name = style.name.to_string();
            for channel in 0..pattern.rows() {
                pattern.set_notes(channel, []);
            }
            rows::write_row_style(project, index, style, start, Some(selected));
            if let Some(rhythm) = style.rhythm() {
                apply::write_rhythm(project, index, rhythm);
            }
            if use_tempo {
                project.tempo = Tempo::new(style.tempo);
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
            style: None,
            rhythm: style.rhythm(),
            rhythm_picked: false,
            row_style: Some(style),
        });
        self.set_mode(PlayMode::Pattern);
        if !self.playing {
            self.rewind_for_new_pattern();
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
                ui.weak("Tunes with their melodies, free for everyone to use");
                // A genre with no songs gets no empty submenu.
                for genre in Genre::ALL
                    .into_iter()
                    .filter(|&genre| SONGS.iter().any(|song| song.genre == genre))
                {
                    ui.menu_button(genre.name(), |ui| {
                        for song in SONGS.iter().filter(|song| song.genre == genre) {
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
                }
            });
            ui.menu_button("Styles", |ui| {
                ui.weak("Each harmony style as a short song");
                for genre in Genre::ALL
                    .into_iter()
                    .filter(|&genre| STYLES.iter().any(|style| style.genre == genre))
                {
                    ui.menu_button(genre.name(), |ui| {
                        for style in STYLES.iter().filter(|style| style.genre == genre) {
                            if ui.button(style.name).on_hover_text(style.about).clicked() {
                                picked = Some(Demo::Style(style));
                            }
                        }
                    });
                }
            });
            ui.menu_button("Twelve-tone", |ui| {
                ui.weak("Each row style as a short piece: the row, backwards, upside down, both");
                for style in &ROW_STYLES {
                    if ui.button(style.name).on_hover_text(style.about).clicked() {
                        picked = Some(Demo::Row(style));
                    }
                }
            });
        });
        if let Some(demo) = picked {
            ui.close();
            self.request(&ctx, Pending::Demo(demo));
        }
    }

    /// Opens a demo in place of the project, and plays it, with its
    /// style and rhythm selected in the library.
    pub(super) fn open_demo(&mut self, demo: Demo) {
        self.replace_project(demo.project(), None);
        self.song_library = demo.library();
        self.set_mode(PlayMode::Song);
        self.set_playing(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_songs_select_their_style_and_rhythm() {
        for song in SONGS {
            let tried = Demo::Song(song).library();
            assert_eq!(
                tried.style.map(|s| s.name),
                (!song.style.is_empty()).then_some(song.style)
            );
            assert_eq!(
                tried.rhythm.map(|r| r.name),
                (!song.rhythm.is_empty()).then_some(song.rhythm),
                "{}",
                song.name
            );
        }
        let zamba = SONGS.iter().find(|s| s.name == "Zamba del telar").unwrap();
        let tried = Demo::Song(zamba).library();
        assert_eq!(tried.style.map(|s| s.name), Some("Zamba"));
        assert_eq!(tried.rhythm.map(|r| r.name), Some("Zamba"));
        assert_eq!(Demo::Start.library(), Tried::default());
    }
}
