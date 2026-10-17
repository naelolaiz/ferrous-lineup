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
//!
//! A rhythm or style in another time signature puts the whole song in
//! it, bar for bar (see [`loom_core::remeter`]). Each squeeze or stretch
//! loses a little (notes that met on one tick may part on the next), so
//! the app remembers the song as it was before the library first changed
//! it, and every rhythm or style picked after that starts again from that
//! original: 4/4 to 3/4 to 6/8 and back to 4/4 gives back the 4/4 song.

use eframe::egui;
use loom_audio::PlayMode;
use loom_core::library::demo::{micromode_demo, row_demo, song_demo, style_demo};
use loom_core::library::{
    FILLS, Genre, MICROMODE_STYLES, MicromodeStyle, ROW_STYLES, Rhythm, RowStyle, SONGS, STYLES,
    Song, Style, apply, micromodes, rows,
};
use loom_core::remeter::remeter;
use loom_core::{Pattern, Project, Tempo, TimeSignature};

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
    micromode_style: Option<&'static MicromodeStyle>,
}

/// A demo to open from the File menu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Demo {
    /// The song Ferrous Lineup starts with.
    Start,
    Song(&'static Song),
    Style(&'static Style),
    Row(&'static RowStyle),
    Micromode(&'static MicromodeStyle),
}

impl Demo {
    /// The style and rhythm the demo is made from.
    fn library(self) -> Tried {
        match self {
            Demo::Start => Tried::default(),
            Demo::Song(song) => Tried {
                style: Style::named(song.style),
                rhythm: Rhythm::named(song.rhythm),
                ..Tried::default()
            },
            Demo::Style(style) => Tried {
                style: Some(style),
                rhythm: Rhythm::named(style.rhythm),
                ..Tried::default()
            },
            Demo::Row(style) => Tried {
                rhythm: style.rhythm(),
                row_style: Some(style),
                ..Tried::default()
            },
            Demo::Micromode(style) => Tried {
                rhythm: style.rhythm(),
                micromode_style: Some(style),
                ..Tried::default()
            },
        }
    }

    /// The name the title bar shows for the demo, such as "Für Elise".
    pub(super) fn name(self) -> &'static str {
        match self {
            Demo::Start => "The starting song",
            Demo::Song(song) => song.name,
            Demo::Style(style) => style.name,
            Demo::Row(style) => style.name,
            Demo::Micromode(style) => style.name,
        }
    }

    pub(super) fn project(self) -> Project {
        match self {
            Demo::Start => Project::demo(),
            Demo::Song(song) => song_demo(song),
            Demo::Style(style) => style_demo(style),
            Demo::Row(style) => row_demo(style),
            Demo::Micromode(style) => micromode_demo(style),
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
                micromode_style: audition.micromode_style,
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
        if let LibraryAction::MicromodeStyle(style) = action {
            self.try_micromode_style(style);
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
            LibraryAction::RhythmInSong(_)
            | LibraryAction::RowStyle(_)
            | LibraryAction::MicromodeStyle(_) => return,
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
                    project.scale_tempo(Tempo::new(bpm));
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
            micromode_style: None,
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
            LibraryAction::MicromodeStyle(style) => {
                (style.name, Some(style.tempo), "change the style")
            }
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
        let original = self.song_original();
        // A rhythm or a style starts again from the song as it was before
        // the library changed it. A fill or a twelve-tone or micromode
        // style goes on top of what plays now.
        let from_original = matches!(
            action,
            LibraryAction::Style(_) | LibraryAction::Rhythm(_) | LibraryAction::RhythmInSong(_)
        ) && !fill_picked(action);
        let change = self.edit(label, |project| {
            if from_original {
                *project = original.clone();
            }
            let change = match action {
                LibraryAction::Style(style) => apply::use_style(project, style),
                LibraryAction::RowStyle(style) => rows::use_row_style(project, style),
                LibraryAction::MicromodeStyle(style) => {
                    micromodes::use_micromode_style(project, style)
                }
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm)
                    if FILLS.contains(rhythm) =>
                {
                    apply::use_fill(project, rhythm)
                }
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
                    apply::use_rhythm(project, rhythm)
                }
            };
            // A song put in another time signature keeps its pace (see
            // `remeter`): the rhythm's own tempo would rush or drag it.
            if use_tempo
                && change != apply::RhythmChange::default()
                && change.time.is_none()
                && let Some(bpm) = tempo
            {
                project.scale_tempo(Tempo::new(bpm));
            }
            if change == apply::RhythmChange::default() {
                // Nothing to play differently: the song stays as it is.
                *project = before.clone();
            }
            change
        });
        if self.project != before {
            self.remember_original(original);
            self.keep_place_in_song(&before);
        }
        // The time signature the song changed to, back to its own
        // included.
        let new_time =
            (self.project.meter() != before.meter()).then_some(self.project.time_signature);
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
                LibraryAction::MicromodeStyle(_) => "the micromode instead of the chords",
                _ => "chords and bass played again",
            };
            parts.push(format!("{what} in {}", patterns(change.harmony)));
        }
        let fill = fill_picked(action);
        // The song now plays what was picked, so the library shows it.
        if change != apply::RhythmChange::default() {
            self.song_library = match action {
                LibraryAction::Style(style) => Tried {
                    style: Some(style),
                    row_style: None,
                    micromode_style: None,
                    ..song_library
                },
                LibraryAction::RowStyle(style) => Tried {
                    rhythm: style.rhythm().or(song_library.rhythm),
                    row_style: Some(style),
                    ..Tried::default()
                },
                LibraryAction::MicromodeStyle(style) => Tried {
                    rhythm: style.rhythm().or(song_library.rhythm),
                    micromode_style: Some(style),
                    ..Tried::default()
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
        } else if let Some(time) = new_time {
            let what = if parts.is_empty() {
                String::new()
            } else {
                format!(": {}", parts.join(", "))
            };
            format!(
                "The song now plays {name} in {time}, every bar squeezed or stretched into a \
                 {time} bar{what}. Ctrl+Z takes it back."
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

    /// The Time box, and the time signature of bar 1: puts the song in
    /// `time`. A song with notes is re-metered bar for bar, as a library
    /// rhythm in `time` would (see [`loom_core::remeter`]), so it really
    /// plays in the new time signature; an empty one just gets the new
    /// bars.
    pub(super) fn set_song_time(&mut self, time: TimeSignature) {
        let original = self.song_original();
        let before = self.project.clone();
        let remetered = self.edit("change time signature", |project| {
            if project.patterns().iter().all(Pattern::is_empty) {
                project.time_signature = time;
                false
            } else {
                remeter(project, time)
            }
        });
        if !remetered {
            return;
        }
        // The original stays the reference for the library's rhythms.
        self.remember_original(original);
        self.keep_place_in_song(&before);
        self.notice = Some(format!(
            "The song is now in {time} at {:.0} BPM: every bar squeezed or stretched, \
             lasting as long as before. Ctrl+Z takes it back.",
            self.project.tempo.bpm()
        ));
    }

    /// The song as it was before the library changed it, if the library
    /// made the project as it is now, or else the project itself.
    fn song_original(&self) -> Project {
        self.song_originals
            .iter()
            .rev()
            .find(|(version, _)| *version == self.project)
            .map_or_else(|| self.project.clone(), |(_, original)| original.clone())
    }

    /// Remembers that the project as it is now was made from `original`.
    fn remember_original(&mut self, original: Project) {
        let project = &self.project;
        self.song_originals
            .retain(|(version, _)| version != project);
        if self.song_originals.len() == REMEMBERED {
            self.song_originals.remove(0);
        }
        self.song_originals.push((self.project.clone(), original));
    }

    /// After the song changed time signature, keeps playing (or ready to
    /// play) the same bar, and the same place in it, as before.
    pub(super) fn keep_place_in_song(&mut self, before: &Project) {
        if self.mode != PlayMode::Song || before.meter() == self.project.meter() {
            return;
        }
        // Bar n, and the same part of it, in the song as it plays now.
        let old = before.played_meter(&before.playback_arrangement());
        let new = self
            .project
            .played_meter(&self.project.playback_arrangement());
        let place = |step: usize| {
            let bar = old.bar_at(step);
            let into = (step - bar.start) as f64 / bar.length.max(1) as f64;
            let target = new.bar(bar.number);
            target.start + (into * target.length as f64).round() as usize
        };
        self.cursor = place(self.cursor);
        self.marker = place(self.marker);
        if let Some(step) = self.playhead() {
            self.send(loom_audio::Command::Seek(place(step)));
        }
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
                project.scale_tempo(Tempo::new(style.tempo));
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
            micromode_style: None,
        });
        self.set_mode(PlayMode::Pattern);
        if !self.playing {
            self.rewind_for_new_pattern();
            self.set_playing(true);
        }
    }

    /// Writes a micromode style into a pattern and plays it, as
    /// [`Self::try_row_style`] does for a row style. The micromode starts
    /// on the project's key note.
    fn try_micromode_style(&mut self, style: &'static MicromodeStyle) {
        let target = self.unchanged_audition().map(|audition| audition.pattern);
        let selected = self.piano_roll.channel();
        let key = self.project.scale.root;
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
            let start = micromodes::DEMO_SECTIONS[0].0;
            micromodes::write_micromode_style(project, index, style, key, start, Some(selected));
            if let Some(rhythm) = style.rhythm() {
                apply::write_rhythm(project, index, rhythm);
            }
            if use_tempo {
                project.scale_tempo(Tempo::new(style.tempo));
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
            row_style: None,
            micromode_style: Some(style),
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
                .on_hover_text("The beat and chords Ferrous Lineup opens with")
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
            ui.menu_button("Micromodes", |ui| {
                ui.weak(
                    "Each micromode style as a short piece: from the micromode, upside down, \
                     a tritone away, upside down there, and back",
                );
                for style in &MICROMODE_STYLES {
                    let micromode = style.micromode();
                    let hover =
                        format!("{}\n\n{}: {}", style.about, micromode.name, micromode.about);
                    if ui
                        .button(format!("{} · {}", style.name, micromode.name))
                        .on_hover_text(hover)
                        .clicked()
                    {
                        picked = Some(Demo::Micromode(style));
                    }
                }
                ui.separator();
                ui.weak("A famous row made of one micromode");
                if let Some(style) = RowStyle::named("Webern's trichords")
                    && ui
                        .button(format!("{} · Menor 3", style.name))
                        .on_hover_text(style.about)
                        .clicked()
                {
                    picked = Some(Demo::Row(style));
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
        self.file.open_demo(demo);
        self.song_library = demo.library();
        self.set_mode(PlayMode::Song);
        self.set_playing(true);
    }
}

/// Whether `action` puts a fill in the song.
fn fill_picked(action: LibraryAction) -> bool {
    matches!(
        action,
        LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) if FILLS.contains(rhythm)
    )
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
