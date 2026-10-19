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
//! Library choices apply their checked components to the current song.
//! Meter changes keep each bar's duration (see [`lineup_core::remeter`])
//! and start from a source in the original meter, so rounding and chord
//! snapping do not accumulate. Preset components selected along with a
//! meter change also update that source; standalone edits start a new reference.

use lineup_audio::PlayMode;
use lineup_core::library::demo::{micromode_demo, row_demo, song_demo, style_demo};
use lineup_core::library::{
    FILLS, MicromodeStyle, Rhythm, RowStyle, Song, Style, apply, micromodes, rows,
};
use lineup_core::{BandStyle, Pattern, Project, Tempo, TimeSignature};

use super::LineupApp;
use crate::library_browser::{LibraryAction, Tried};

/// How many of the patterns the library wrote are remembered.
const REMEMBERED: usize = 64;

#[derive(Clone)]
pub(super) struct SongSource {
    original: Project,
    library: Tried,
    /// Latest preset components, applied without changing the source meter.
    reference: Project,
}

/// A pattern the library wrote, and what it wrote there.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Audition {
    pattern: usize,
    /// The pattern as the library left it. If it still looks like this,
    /// the next click may rewrite it.
    written: Pattern,
    style: Option<&'static Style>,
    rhythm: Option<&'static Rhythm>,
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

impl LineupApp {
    fn song_source(&self) -> SongSource {
        self.song_sources
            .iter()
            .rev()
            .find(|(version, _)| *version == self.project)
            .map(|(_, source)| source.clone())
            .unwrap_or_else(|| SongSource {
                original: self.project.clone(),
                library: self
                    .project
                    .band()
                    .map_or(self.song_library, Tried::from_band),
                reference: self.project.clone(),
            })
    }

    /// Apply meter changes directly to the source, and keep unchecked
    /// components from earlier preset choices in that same source meter.
    /// Version matching lets undo reuse its reference and makes edits
    /// establish a new one instead of losing the user's work.
    pub(super) fn edit_song<R>(
        &mut self,
        label: &'static str,
        time: Option<TimeSignature>,
        parts: apply::StyleParts,
        tempo_on_meter_change: bool,
        change: impl Fn(&mut Project, apply::StyleParts) -> R,
    ) -> R {
        let Some(time) = time else {
            return self.edit(label, |project| change(project, parts));
        };
        // A standalone component edit is an explicit project change. Its
        // result will not match a cached conversion, establishing a new source.
        if !parts.time_signature {
            return self.edit(label, |project| change(project, parts));
        }
        let before = self.project.clone();
        let mut source = self.song_source();
        let changes_meter = !lineup_core::remeter::is_in(&before, time);
        let effective = apply::StyleParts {
            // Built-in meter changes preserve bar duration; saved styles
            // can explicitly supply their tempo along with their meter.
            tempo: parts.tempo && (!changes_meter || tempo_on_meter_change),
            ..parts
        };
        let result = self.edit(label, |project| {
            *project = source.reference.clone();
            lineup_core::remeter::remeter(project, time);
            change(
                project,
                apply::StyleParts {
                    time_signature: false,
                    ..effective
                },
            )
        });
        if self.project != before {
            change(
                &mut source.reference,
                apply::StyleParts {
                    time_signature: false,
                    tempo: false,
                    ..parts
                },
            );
            if effective.tempo {
                let ratio = source.reference.time_signature.steps_per_bar() as f32
                    / self.project.time_signature.steps_per_bar() as f32;
                source
                    .reference
                    .scale_tempo(Tempo::new(self.project.tempo.bpm() * ratio));
            }
            source.reference.follow_chord_sheet();
            self.song_sources
                .retain(|(version, _)| *version != self.project);
            if self.song_sources.len() == REMEMBERED {
                self.song_sources.remove(0);
            }
            self.song_sources.push((self.project.clone(), source));
        }
        result
    }

    /// What the library shows selected: what it put in the selected
    /// pattern, if that is unchanged, or else the style and rhythm the
    /// whole song plays, so a demo opens with its own selected, as in FL
    /// Studio.
    pub(super) fn tried(&self) -> Tried {
        if let Some(band) = self.project.band() {
            return Tried::from_band(band);
        }
        if self.mode == PlayMode::Song {
            return self.song_library;
        }
        match self.unchanged_audition() {
            Some(audition) => Tried {
                user_style: None,
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
    pub(super) fn remember_song_version(&mut self, project: Project, tried: Tried) {
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
        self.song_library = self
            .song_versions
            .iter()
            .rev()
            .find(|(version, _)| *version == self.project)
            .map_or_else(Tried::default, |(_, tried)| *tried);
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
        if let LibraryAction::UserStyle(index) = action {
            self.apply_user_library(index, None);
            return;
        }
        let parts = match action {
            LibraryAction::Rhythm(_) | LibraryAction::RhythmInSong(_) => {
                self.playlist.library_parts().for_rhythm()
            }
            _ => self.playlist.library_parts(),
        };
        if !(parts.rhythm
            || parts.instruments
            || parts.time_signature
            || parts.progression
            || parts.tempo
            || parts.swing)
        {
            self.notice = Some("Choose a component to apply in the Library options.".to_string());
            return;
        }
        // With a band, a style or a rhythm is the band's new way of
        // playing the chord sheet.
        if (self.project.band().is_some() || self.mode == PlayMode::Song) && self.band_plays(action)
        {
            return;
        }
        // Taking over with a pattern of its own would stop the song, so
        // while the song plays, every click goes to the song.
        if self.playing && self.mode == PlayMode::Song {
            self.use_in_song(action);
            return;
        }
        if matches!(
            action,
            LibraryAction::RhythmInSong(_) | LibraryAction::StyleInSong(_)
        ) {
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
        let previous = reuse.as_ref().map_or(self.song_library, |a| Tried {
            user_style: None,
            style: a.style,
            rhythm: a.rhythm,
            row_style: a.row_style,
            micromode_style: a.micromode_style,
        });
        let parts = match action {
            LibraryAction::Rhythm(_) => self.playlist.library_parts().for_rhythm(),
            _ => self.playlist.library_parts(),
        };
        let mut tried = previous;
        let (name, tempo) = match action {
            LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
                if parts.rhythm || parts.progression {
                    tried.style = Some(style);
                    tried.row_style = None;
                    tried.micromode_style = None;
                }
                if parts.rhythm {
                    tried.rhythm = Rhythm::named(style.rhythm);
                }
                (style.name, style.tempo)
            }
            LibraryAction::Rhythm(rhythm) => {
                if parts.rhythm {
                    tried.rhythm = Some(rhythm);
                    tried.style = None;
                }
                (rhythm.name, rhythm.tempo)
            }
            _ => return,
        };
        // Without patterns or chords to write, only the sounds, the tempo
        // or the swing of your own pattern change: it stays yours, keeps
        // its name, and the next click does not write over it.
        let own_pattern = reuse.is_none() && !parts.rhythm && !parts.progression;
        let target = reuse
            .as_ref()
            .map(|a| a.pattern)
            .or_else(|| own_pattern.then_some(self.pattern));
        let selected = self.piano_roll.selected_channel();
        let swing = tried.rhythm.and_then(|rhythm| self.swings.of(rhythm.name));
        let before = self.project.clone();
        let written = self.edit("try from the library", |project| {
            let index = target.or_else(|| project.add_pattern())?;
            match action {
                LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
                    apply::write_style_parts(project, index, style, selected, parts)
                }
                LibraryAction::Rhythm(rhythm) => {
                    apply::write_rhythm_parts(project, index, rhythm, parts)
                }
                _ => return None,
            }
            if parts.swing
                && let Some(swing) = swing
            {
                project.swing = swing;
            }
            if parts.tempo {
                project.scale_tempo(Tempo::new(tempo));
            }
            if !own_pattern {
                project.pattern_mut(index)?.name = name.to_string();
            }
            Some((index, project.pattern(index)?.clone()))
        });
        let Some((pattern, written)) = written else {
            self.message = Some("There is no room for another pattern.".to_string());
            return;
        };
        if own_pattern && self.project == before {
            self.notice = Some(if parts.instruments {
                format!(
                    "{name} changed nothing: its instruments go to the channels named \
                     Keys and Bass, or to the channel chosen in the rack."
                )
            } else {
                format!("{name} changed nothing: the song already goes that way.")
            });
            return;
        }
        self.select_pattern(pattern);
        if !own_pattern {
            if self.auditions.len() == REMEMBERED {
                self.auditions.remove(0);
            }
            self.auditions.push(Audition {
                pattern,
                written,
                style: tried.style,
                rhythm: tried.rhythm,
                row_style: tried.row_style,
                micromode_style: tried.micromode_style,
            });
        }
        self.set_mode(PlayMode::Pattern);
        if !self.playing {
            self.rewind_for_new_pattern();
            self.set_playing(true);
        }
    }

    /// Plays the whole song the way a style, a rhythm or a fill goes, as
    /// one step to undo, and says what changed. The song keeps its
    /// chords, its melody and its place: see [`apply::use_style`],
    /// [`apply::use_rhythm`] and [`apply::use_fill`].
    fn use_in_song(&mut self, action: LibraryAction) {
        let parts = match action {
            LibraryAction::Rhythm(_) | LibraryAction::RhythmInSong(_) => {
                self.playlist.library_parts().for_rhythm()
            }
            _ => self.playlist.library_parts(),
        };
        self.use_in_song_parts(action, parts);
    }

    fn use_in_song_parts(&mut self, action: LibraryAction, parts: apply::StyleParts) {
        if let LibraryAction::UserStyle(index) = action {
            self.apply_user_library(index, None);
            return;
        }
        if self.band_plays_parts(action, parts) {
            return;
        }
        let (name, tempo, label) = match action {
            LibraryAction::UserStyle(_) => unreachable!("handled above"),
            LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
                (style.name, Some(style.tempo), "change the style")
            }
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
        // The swing the rhythm was last played with, if it was set.
        let rhythm = match action {
            _ if fill_picked(action) => None,
            LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
                Rhythm::named(style.rhythm)
            }
            LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => Some(rhythm),
            LibraryAction::UserStyle(_)
            | LibraryAction::RowStyle(_)
            | LibraryAction::MicromodeStyle(_) => None,
        };
        let swing = rhythm.and_then(|rhythm| self.swings.of(rhythm.name));
        let time = match action {
            LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => Some(style.time),
            LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm)
                if !fill_picked(action) =>
            {
                Some(rhythm.time)
            }
            _ => None,
        };
        let change = self.edit_song(label, time, parts, false, |project, parts| {
            let change = match action {
                LibraryAction::UserStyle(_) => unreachable!("handled above"),
                LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
                    apply::use_style_parts(project, style, parts)
                }
                LibraryAction::RowStyle(style) => rows::use_row_style(project, style),
                LibraryAction::MicromodeStyle(style) => {
                    micromodes::use_micromode_style(project, style)
                }
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm)
                    if FILLS.contains(rhythm) =>
                {
                    if parts.rhythm {
                        apply::use_fill(project, rhythm)
                    } else {
                        apply::RhythmChange::default()
                    }
                }
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
                    apply::use_rhythm_parts(project, rhythm, parts)
                }
            };
            if parts.swing
                && let Some(swing) = swing
            {
                project.swing = swing;
            }
            // A song put in another time signature keeps its pace (see
            // `remeter`): the rhythm's own tempo would rush or drag it.
            if parts.tempo
                && change.time.is_none()
                && let Some(bpm) = tempo
            {
                project.scale_tempo(Tempo::new(bpm));
            }
            change
        });
        if self.project != before {
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
        let mut descriptions = Vec::new();
        if change.drums > 0 {
            descriptions.push(format!("new drums in {}", patterns(change.drums)));
        }
        if change.harmony > 0 {
            let what = match action {
                LibraryAction::RowStyle(_) => "the row instead of the chords",
                LibraryAction::MicromodeStyle(_) => "the micromode instead of the chords",
                _ => "chords and bass played again",
            };
            descriptions.push(format!("{what} in {}", patterns(change.harmony)));
        }
        let fill = fill_picked(action);
        // The song now plays what was picked, so the library shows it.
        if self.project != before || change != apply::RhythmChange::default() {
            self.song_library = match action {
                LibraryAction::Style(style) | LibraryAction::StyleInSong(style)
                    if parts.rhythm || parts.progression =>
                {
                    Tried {
                        user_style: None,
                        style: Some(style),
                        rhythm: if parts.rhythm {
                            Rhythm::named(style.rhythm)
                        } else {
                            song_library.rhythm
                        },
                        row_style: None,
                        micromode_style: None,
                    }
                }
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
                LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm)
                    if !fill && parts.rhythm =>
                {
                    Tried {
                        user_style: None,
                        style: None,
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
            let what = if descriptions.is_empty() {
                String::new()
            } else {
                format!(": {}", descriptions.join(", "))
            };
            format!(
                "The song now plays {name} in {time}, every bar squeezed or stretched into a \
                 {time} bar{what}. Ctrl+Z takes it back."
            )
        } else if descriptions.is_empty() {
            format!("Applied selected parts of {name}.")
        } else {
            format!(
                "The song now plays {name}: {}. Ctrl+Z takes it back.",
                descriptions.join(", ")
            )
        });
    }

    /// The Time box, and the time signature of bar 1: puts the song in
    /// `time`, so that it sounds in `time`.
    ///
    /// Squeezing the notes into the new bars is not enough: the tempo
    /// keeps each bar as long as it was (see [`lineup_core::remeter`]),
    /// so every note would still sound at the same moment. What makes 4/4
    /// sound like 3/4 is the drums and the way the chords are played, so
    /// the song plays a rhythm in `time`, as if it were picked in the
    /// library: one of the kind the song plays if there is one (see
    /// [`Rhythm::for_time`]). With matching style off, a plain beat and
    /// accents use the requested meter and current instruments. Returning
    /// to the source meter restores the original performance exactly.
    pub(super) fn set_song_time(&mut self, time: TimeSignature) {
        if self.project.patterns().iter().all(Pattern::is_empty) {
            self.edit("change time signature", |project| {
                project.time_signature = time;
            });
            return;
        }
        let source = self.song_source();
        if lineup_core::remeter::is_in(&source.original, time) {
            let before = self.project.clone();
            let previous = self.tried();
            self.edit("change time signature", |project| {
                *project = source.original.clone()
            });
            self.song_library = source.library;
            self.keep_place_in_song(&before);
            self.remember_song_version(before, previous);
            self.remember_song_version(self.project.clone(), self.song_library);
            self.notice = Some(format!(
                "The song is back in {time}, as it was. Ctrl+Z takes it back."
            ));
            return;
        }
        let playing = source.library.rhythm.or_else(|| {
            source
                .library
                .style
                .and_then(|style| Rhythm::named(style.rhythm))
        });
        let parts = apply::StyleParts {
            time_signature: true,
            ..self.playlist.library_parts().for_rhythm()
        };
        let rhythm = if parts.matching_style {
            Rhythm::for_time(time, playing)
        } else {
            Rhythm::basic(time)
        };
        self.use_in_song_parts(LibraryAction::RhythmInSong(rhythm), parts);
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
            self.send(lineup_audio::Command::Seek(place(step)));
        }
    }

    /// Writes a twelve-tone style into a pattern and plays it, as
    /// [`Self::try_from_library`] does for a harmony style: the same
    /// pattern is rewritten while it is unchanged. The row starts on the
    /// project's key note.
    fn try_row_style(&mut self, style: &'static RowStyle) {
        let target = self.unchanged_audition().map(|audition| audition.pattern);
        let selected = self.piano_roll.selected_channel();
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
            rows::write_row_style(project, index, style, start, selected);
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
        let selected = self.piano_roll.selected_channel();
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
            micromodes::write_micromode_style(project, index, style, key, start, selected);
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
            row_style: None,
            micromode_style: Some(style),
        });
        self.set_mode(PlayMode::Pattern);
        if !self.playing {
            self.rewind_for_new_pattern();
            self.set_playing(true);
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

impl LineupApp {
    /// Has the band play the way a library style or rhythm goes. Returns
    /// `false` for what a band does not play: fills, rows and micromode
    /// styles.
    fn band_plays(&mut self, action: LibraryAction) -> bool {
        let parts = match action {
            LibraryAction::Style(_) | LibraryAction::StyleInSong(_) => {
                self.playlist.library_parts()
            }
            _ => self.playlist.library_parts().for_rhythm(),
        };
        self.band_plays_parts(action, parts)
    }

    fn band_plays_parts(&mut self, action: LibraryAction, parts: apply::StyleParts) -> bool {
        // A demo has a written accompaniment but no live band yet. Copying
        // both patterns and voices must bring in the full preset lineup,
        // just as changing an existing band does. Copying a chart is optional.
        if self.project.band().is_none()
            && !(parts.rhythm
                && parts.for_preset().instruments
                && !self.project.playlist().chord_markers().is_empty())
        {
            return false;
        }
        let style = match action {
            _ if fill_picked(action) => return false,
            LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
                BandStyle::of_style(style)
            }
            LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
                BandStyle::named(rhythm.name)
            }
            LibraryAction::UserStyle(_)
            | LibraryAction::RowStyle(_)
            | LibraryAction::MicromodeStyle(_) => return false,
        };
        let Some(style) = style else {
            // Generated beats have no named BandStyle. Embed their patterns
            // just like a saved style, so the live band plays and saves them.
            if let LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) = action {
                let mut style = lineup_core::library::UserStyle::from_rhythm(rhythm);
                style.name = rhythm.name.into();
                style.chart.clear();
                let parts = apply::StyleParts {
                    matching_style: false,
                    ..parts
                }
                .for_preset();
                let before = self.project.clone();
                let previous = self.tried();
                self.edit_song(
                    "change the band",
                    Some(rhythm.time),
                    parts,
                    false,
                    |project, parts| {
                        project
                            .apply_user_style(&style, parts)
                            .expect("generated rhythm is valid");
                    },
                );
                self.song_library = Tried::from_band(self.project.band().unwrap());
                self.keep_place_in_song(&before);
                self.remember_song_version(before, previous);
                self.remember_song_version(self.project.clone(), self.song_library);
                self.notice = Some(format!("Applied selected parts of {}", rhythm.name));
                return true;
            }
            return false;
        };
        let harmony = match action {
            LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => Some(style),
            _ => None,
        };
        let swing = self.swings.of(style.name);
        let before = self.project.clone();
        let previous = self.tried();
        self.edit_song(
            "change the band",
            Some(harmony.map_or(style.time, |style| style.time)),
            parts,
            false,
            |project, parts| {
                project.apply_band_parts(style, harmony, parts);
                if parts.swing
                    && let Some(swing) = swing
                {
                    project.swing = swing;
                }
            },
        );
        self.song_library = Tried::from_band(self.project.band().unwrap());
        self.keep_place_in_song(&before);
        self.remember_song_version(before, previous);
        self.remember_song_version(self.project.clone(), self.song_library);
        self.notice = Some(format!(
            "Applied selected parts of {}",
            harmony.map_or(style.name, |s| s.name)
        ));
        true
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
    use lineup_core::library::SONGS;

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
