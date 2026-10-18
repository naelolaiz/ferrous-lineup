//! The Library tab: one searchable, alphabetical list of accompaniment
//! presets, with optional chord progressions and shared copy controls.
//!
//! Like the other views, it only draws and reads the mouse. A click
//! returns a [`LibraryAction`], and the app writes the style or rhythm
//! into a pattern (see `app/library.rs`).

use eframe::egui::{self, RichText, Ui};
use lineup_core::library::apply::StyleParts;
use lineup_core::library::{
    FILLS, MICROMODE_STYLES, MicromodeStyle, ROW_STYLES, Rhythm, RowStyle, Style,
};
use lineup_core::{Band, Project, TimeSignature};

pub(crate) mod catalogue;
use catalogue::{Preset, alphabetical, matches};

/// Something picked in the library.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibraryAction {
    UserStyle(usize),
    /// Play this style's chords and bass line, in the project's key.
    Style(&'static Style),
    /// Apply this preset to the whole song even while stopped.
    StyleInSong(&'static Style),
    /// Play this rhythm (or fill) on the drums.
    Rhythm(&'static Rhythm),
    /// Play the whole song in this rhythm: its drums, and its chords and
    /// bass played the way the rhythm goes.
    RhythmInSong(&'static Rhythm),
    /// Play this way of playing a twelve-tone row. The key becomes the
    /// row, starting on the project's key note.
    RowStyle(&'static RowStyle),
    /// Play this micromode style. The key becomes its micromode, on the
    /// project's key note.
    MicromodeStyle(&'static MicromodeStyle),
}

/// The same browser serves auditions, the band's playing and written progressions.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LibraryPurpose {
    Play,
    Band,
    Progression,
}

/// What the selected pattern was made from, if the library made it, to
/// show it selected.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Tried {
    pub user_style: Option<u64>,
    pub style: Option<&'static Style>,
    pub rhythm: Option<&'static Rhythm>,
    pub row_style: Option<&'static RowStyle>,
    pub micromode_style: Option<&'static MicromodeStyle>,
}

impl Tried {
    /// Read the exact rhythm and harmony style from the project, including undo and load.
    pub fn from_band(band: &Band) -> Self {
        Self {
            user_style: band.custom_rhythm.as_ref().map(|style| style.id()),
            style: band.harmony_style.filter(|_| band.custom_rhythm.is_none()),
            rhythm: band.style.rhythm.filter(|_| band.custom_rhythm.is_none()),
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy)]
pub enum StyleEditorRequest {
    New,
    Builtin(&'static Style),
    Rhythm(&'static Rhythm),
    User(usize),
    Folder,
}

/// What the library tab remembers between frames.
pub struct LibraryBrowser {
    pub user_styles: Vec<lineup_core::library::UserStyle>,
    pub editor_request: Option<StyleEditorRequest>,
    filter: String,
    /// Which components of a choice replace the current song's settings.
    pub parts: StyleParts,
    /// The selection last drawn, to reveal it after applying, loading or undoing.
    shown: Tried,
    /// Until when (in `ui` time) to keep revealing a changed selection.
    reveal_until: f64,
    /// Whether that is the rhythm (only the rhythm changed) or the style.
    reveal_rhythm: bool,
    /// Whether it is a twelve-tone style.
    reveal_row: bool,
}

impl LibraryBrowser {
    pub fn new() -> Self {
        LibraryBrowser {
            user_styles: Vec::new(),
            editor_request: None,
            filter: String::new(),
            parts: StyleParts::default(),
            shown: Tried::default(),
            reveal_until: 0.0,
            reveal_rhythm: false,
            reveal_row: false,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, project: &Project, tried: Tried) -> Option<LibraryAction> {
        self.show_for(ui, project, tried, LibraryPurpose::Play)
    }

    pub fn show_for(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        tried: Tried,
        purpose: LibraryPurpose,
    ) -> Option<LibraryAction> {
        ui.horizontal(|ui| {
            if ui.small_button("New style…").clicked() {
                self.editor_request = Some(StyleEditorRequest::New);
            }
            if ui.small_button("Styles folder…").clicked() {
                self.editor_request = Some(StyleEditorRequest::Folder);
            }
        });
        ui.horizontal(|ui| {
            let width = (ui.available_width() - 28.0).max(80.0);
            ui.add(
                egui::TextEdit::singleline(&mut self.filter)
                    .hint_text("🔍 Search: bossa, 6/8, tango…")
                    .desired_width(width),
            );
            if ui
                .add_enabled(!self.filter.is_empty(), egui::Button::new("×"))
                .on_hover_text("Clear search")
                .clicked()
            {
                self.filter.clear();
            }
        });
        let height = ui
            .available_height()
            .min(ui.ctx().content_rect().bottom() - ui.next_widget_position().y - 16.0)
            .max(48.0);
        egui::ScrollArea::vertical()
            .id_salt("library-content")
            .max_height(height)
            .show(ui, |ui| self.results(ui, project, tried, purpose))
            .inner
    }

    fn results(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        tried: Tried,
        purpose: LibraryPurpose,
    ) -> Option<LibraryAction> {
        let mut picked = None;
        if purpose != LibraryPurpose::Progression {
            style_parts_ui(ui, &mut self.parts);
        }
        if purpose != LibraryPurpose::Progression
            && let Some(band) = project.band()
        {
            ui.weak(format!("Patterns: {}", band.name()));
            ui.weak(format!("Instruments: {}", band.instrument_name()));
            ui.weak("Chords: chord sheet");
            if band.accents_only {
                ui.weak("Plain accents with current instruments");
            }
        }
        ui.weak(match purpose {
            LibraryPurpose::Play => {
                "Click to play it in a pattern. Click another to swap. While the song plays, a \
             click plays the whole song that way instead, and the song carries on. Right-click \
             a preset to do that while stopped. Ctrl+Z takes it back."
            }
            LibraryPurpose::Band => "Choose a preset to apply the checked components to the band.",
            LibraryPurpose::Progression => {
                "Choose a preset’s chord progression to write from the selected bar."
            }
        });
        ui.add_space(2.0);

        let filter = self.filter.trim().to_lowercase();
        let searching = !filter.is_empty();
        let tonic = project.scale.root;
        let now = ui.input(|i| i.time);
        let tracking = purpose == LibraryPurpose::Play;
        let new_style = tracking
            && ((tried.style.is_some() && tried.style != self.shown.style)
                || (tried.user_style.is_some() && tried.user_style != self.shown.user_style));
        let new_rhythm = tracking && tried.rhythm.is_some() && tried.rhythm != self.shown.rhythm;
        let new_row = tracking
            && ((tried.row_style.is_some() && tried.row_style != self.shown.row_style)
                || (tried.micromode_style.is_some()
                    && tried.micromode_style != self.shown.micromode_style));
        if tracking {
            self.shown = tried;
        }
        if new_style || new_rhythm || new_row {
            self.reveal_until = now + 0.5;
            self.reveal_rhythm = !new_style && !new_row;
            self.reveal_row = new_row;
        }
        let revealing = tracking && now < self.reveal_until;
        if revealing {
            ui.ctx().request_repaint();
        }
        let scroll_row = revealing && self.reveal_row;
        let scroll_rhythm = revealing && self.reveal_rhythm;
        let presets = catalogue::presets(
            &filter,
            &self.user_styles,
            purpose == LibraryPurpose::Progression,
        );
        if searching && presets.is_empty() && !anything_else_matches(&filter, purpose) {
            ui.label(format!(
                "Nothing in the library matches \"{}\".",
                self.filter.trim()
            ));
            ui.weak("Songs with melodies are in File ▸ Demos ▸ Songs.");
        }
        ui.scope(|ui| {
            ui.label(format!("Presets ({})", presets.len()));
            if purpose != LibraryPurpose::Progression {
                ui.weak("+ chords: includes a progression. Other presets keep your chords.");
            }
            for preset in presets {
                let selected = match preset {
                    Preset::Style(style) => tried.style == Some(style),
                    Preset::Rhythm(rhythm) => {
                        tried.style.is_none()
                            && tried.row_style.is_none()
                            && tried.micromode_style.is_none()
                            && tried.rhythm == Some(rhythm)
                    }
                    Preset::User(_, style) => tried.user_style == Some(style.id()),
                };
                let hover = match preset {
                    Preset::Style(style) => describe_style(style, tonic),
                    Preset::Rhythm(rhythm) => format!(
                        "{}\n\n{} · {} BPM · {} bars\nNo chord progression; keeps your chords.",
                        rhythm.about,
                        rhythm.time,
                        rhythm.tempo,
                        rhythm.bars(),
                    ),
                    Preset::User(_, style) => format!(
                        "{}\n\n{} · {} BPM · {} bars\n{}",
                        style.about,
                        style.time,
                        style.tempo.bpm(),
                        style.bars,
                        style.chart,
                    ),
                };
                let response = ui
                    .horizontal_wrapped(|ui| {
                        let response =
                            ui.selectable_label(selected, label(preset.name(), preset.time()));
                        if purpose != LibraryPurpose::Progression {
                            ui.weak(if preset.has_progression() {
                                "+ chords"
                            } else {
                                "keeps chords"
                            });
                        }
                        response
                    })
                    .inner
                    .on_hover_text(hover);
                if selected && revealing && !self.reveal_row {
                    response.scroll_to_me(None);
                }
                if response.clicked() {
                    picked = Some(match preset {
                        Preset::Style(style) => LibraryAction::Style(style),
                        Preset::Rhythm(rhythm) => LibraryAction::Rhythm(rhythm),
                        Preset::User(index, _) => LibraryAction::UserStyle(index),
                    });
                }
                response.context_menu(|ui| {
                    let edit_label = if matches!(preset, Preset::User(_, _)) {
                        "Edit in channel rack…"
                    } else {
                        "Edit a copy in channel rack…"
                    };
                    if ui.button(edit_label).clicked() {
                        self.editor_request = Some(match preset {
                            Preset::Style(style) => StyleEditorRequest::Builtin(style),
                            Preset::Rhythm(rhythm) => StyleEditorRequest::Rhythm(rhythm),
                            Preset::User(index, _) => StyleEditorRequest::User(index),
                        });
                        ui.close();
                    }
                    if purpose == LibraryPurpose::Play
                        && ui.button("Play the whole song with this preset").clicked()
                    {
                        picked = Some(match preset {
                            Preset::Style(style) => LibraryAction::StyleInSong(style),
                            Preset::Rhythm(rhythm) => LibraryAction::RhythmInSong(rhythm),
                            Preset::User(index, _) => LibraryAction::UserStyle(index),
                        });
                        ui.close();
                    }
                });
            }
            if purpose == LibraryPurpose::Play {
                let open = searching || new_row;
                picked = picked.or(row_styles(ui, &filter, open, tried, scroll_row));
                picked = picked.or(micromode_styles(ui, &filter, open, tried, scroll_row));
            }
            if purpose == LibraryPurpose::Progression {
                return;
            }
            if purpose == LibraryPurpose::Play {
                let fills = alphabetical(
                    FILLS.iter().filter(|r| {
                        matches(&filter, r.name, r.genre, r.time, r.about)
                            || names_a_word(&filter, "fill fills")
                    }),
                    |rhythm| rhythm.name,
                );
                if !fills.is_empty() {
                    let header = egui::CollapsingHeader::new(format!("Fills ({})", fills.len()))
                        .id_salt("rhythms-fills");
                    let open = searching
                        || (new_rhythm && tried.rhythm.is_some_and(|r| FILLS.contains(r)));
                    let header = if open {
                        header.open(Some(true))
                    } else {
                        header
                    };
                    header.show(ui, |ui| {
                        for fill in fills {
                            picked = picked.or(rhythm_label(
                                ui,
                                fill,
                                tried,
                                scroll_rhythm,
                                purpose,
                                &mut self.editor_request,
                            ));
                        }
                    });
                }
            }
        });
        picked
    }
}

/// Shared by the Library and the chord sheet's band menu.
pub fn style_parts_ui(ui: &mut Ui, parts: &mut StyleParts) {
    let mut accents = !parts.matching_style;
    if ui.checkbox(&mut accents, "Plain accents, keep instruments").on_hover_text(
        "For any preset, play chord and bass accents on the meter’s beat groups, using your \
         current sounds. Drum patterns still come from the preset. Turn off to copy its full playing patterns.",
    ).changed() {
        parts.matching_style = !accents;
    }
    egui::CollapsingHeader::new("Copy from preset").show(ui, |ui| {
        ui.checkbox(&mut parts.rhythm, "Rhythm patterns").on_hover_text(
            "Replace drum, chord and bass playing patterns, keeping the harmony unless \
             Chord progression is checked. Plain accents replaces chord and bass playing with beat-group accents.",
        );
        ui.add_enabled_ui(parts.matching_style, |ui| {
            ui.checkbox(&mut parts.instruments, "Voices / instruments")
                .on_hover_text("Copy the preset’s sounds and band players. Plain accents keeps current instruments.");
        });
        ui.checkbox(&mut parts.time_signature, "Time signature")
            .on_hover_text(
                "Put each bar in the chosen meter. Turn off to fit the pattern into the current meter.",
            );
        ui.checkbox(&mut parts.progression, "Chord progression (when included)")
            .on_hover_text(
                "Copy the chart from presets marked + chords. Other presets keep the current chords. \
                 An empty audition uses example chords to demonstrate the preset.",
            );
        ui.checkbox(&mut parts.tempo, "Tempo").on_hover_text(
            "Use the usual tempo when the meter stays the same. Changing meter keeps each bar's duration.",
        );
        ui.checkbox(&mut parts.swing, "Swing")
            .on_hover_text("Use the preset’s swing amount.");
    });
}

/// The twelve-tone styles, under the harmony styles, opened when `open`
/// is true. Returns the one clicked, if any.
fn row_styles(
    ui: &mut Ui,
    filter: &str,
    open: bool,
    tried: Tried,
    scroll: bool,
) -> Option<LibraryAction> {
    let styles = alphabetical(
        ROW_STYLES
            .iter()
            .filter(|style| row_style_matches(filter, style)),
        |style| style.name,
    );
    if styles.is_empty() {
        return None;
    }
    let mut picked = None;
    let header = egui::CollapsingHeader::new(RichText::new("Twelve-tone rows").strong());
    let header = if open {
        header.open(Some(true))
    } else {
        header
    };
    header.show(ui, |ui| {
        for style in styles {
            let rhythm = match style.rhythm {
                "" => String::new(),
                rhythm => format!(" · drums: {rhythm}"),
            };
            let hover = format!(
                "{}\n\nRow: {}{rhythm} · {} · {} BPM\n\nThe key becomes the row, starting on the \
                 key note. While the song plays, a click plays the whole song this way.",
                style.about, style.row, style.time, style.tempo
            );
            let selected = tried.row_style == Some(style);
            let response = ui
                .selectable_label(selected, label(style.name, style.time))
                .on_hover_text(hover);
            if selected && scroll {
                response.scroll_to_me(None);
            }
            if response.clicked() {
                picked = Some(LibraryAction::RowStyle(style));
            }
        }
    });
    picked
}

/// The micromode styles, under the twelve-tone styles, opened when
/// `open` is true. Returns the one clicked, if any.
fn micromode_styles(
    ui: &mut Ui,
    filter: &str,
    open: bool,
    tried: Tried,
    scroll: bool,
) -> Option<LibraryAction> {
    let styles = alphabetical(
        MICROMODE_STYLES
            .iter()
            .filter(|style| micromode_style_matches(filter, style)),
        |style| style.name,
    );
    if styles.is_empty() {
        return None;
    }
    let mut picked = None;
    let header = egui::CollapsingHeader::new(RichText::new("Micromodes (Kröpfl)").strong());
    let header = if open {
        header.open(Some(true))
    } else {
        header
    };
    header.show(ui, |ui| {
        for style in styles {
            let micromode = style.micromode();
            let rhythm = match style.rhythm {
                "" => String::new(),
                rhythm => format!(" · drums: {rhythm}"),
            };
            let hover = format!(
                "{}\n\nMicromode: {} (set {}) · {}{rhythm} · {} · {} BPM\n\nThe key becomes the \
                 micromode, starting on the key note. While the song plays, a click plays the \
                 whole song this way.",
                style.about,
                micromode.full_name(),
                micromode.set_class().name(),
                style.walk.name(),
                style.time,
                style.tempo
            );
            let selected = tried.micromode_style == Some(style);
            let response = ui
                .selectable_label(selected, label(style.name, style.time))
                .on_hover_text(hover);
            if selected && scroll {
                response.scroll_to_me(None);
            }
            if response.clicked() {
                picked = Some(LibraryAction::MicromodeStyle(style));
            }
        }
    });
    picked
}

/// One rhythm in the list. Returns it if it was clicked, or picked for
/// the whole song from its menu. Fills have no menu: they are one bar.
/// With `scroll`, a selected rhythm scrolls into view.
fn rhythm_label(
    ui: &mut Ui,
    rhythm: &'static Rhythm,
    tried: Tried,
    scroll: bool,
    purpose: LibraryPurpose,
    editor: &mut Option<StyleEditorRequest>,
) -> Option<LibraryAction> {
    let selected = tried.rhythm == Some(rhythm);
    let is_fill = FILLS.contains(rhythm);
    let hover = format!(
        "{}\n\n{} · {} BPM · {} bar{}{}",
        rhythm.about,
        rhythm.time,
        rhythm.tempo,
        rhythm.bars(),
        if rhythm.bars() == 1 { "" } else { "s" },
        if purpose == LibraryPurpose::Band {
            "\n\nApply the checked rhythm components to the band."
        } else if is_fill {
            "\n\nWhile the song plays, a click puts this fill in the last bar of its drums."
        } else {
            "\n\nRight-click to play the whole song in this rhythm. While the song plays, \
             a click does that too."
        }
    );
    let response = ui
        .selectable_label(selected, label(rhythm.name, rhythm.time))
        .on_hover_text(hover);
    if selected && scroll {
        response.scroll_to_me(None);
    }
    let mut picked = response.clicked().then_some(LibraryAction::Rhythm(rhythm));
    response.context_menu(|ui| {
        if ui.button("Edit a copy in channel rack…").clicked() {
            *editor = Some(StyleEditorRequest::Rhythm(rhythm));
            ui.close();
        }
        if !is_fill
            && purpose == LibraryPurpose::Play
            && ui
                .button("Play the whole song in this rhythm")
                .on_hover_text(
                    "Every pattern with drums gets this rhythm, and the chords and bass \
                     are played the way it goes, keeping the same chords. A rhythm in \
                     another time signature puts the song in it, bar for bar.",
                )
                .clicked()
        {
            picked = Some(LibraryAction::RhythmInSong(rhythm));
            ui.close();
        }
    });
    picked
}

/// The name, with the time signature after it unless it is 4/4.
pub(crate) fn label(name: &str, time: TimeSignature) -> String {
    if time.is_common() {
        name.to_string()
    } else {
        format!("{name}  ({time})")
    }
}

/// Whether a twelve-tone style matches the search.
fn row_style_matches(filter: &str, style: &RowStyle) -> bool {
    filter.is_empty()
        || names_a_word(
            filter,
            "twelve-tone twelve tone rows serialism dodecaphony dodecafonismo",
        )
        || style.name.to_lowercase().contains(filter)
        || style.about.to_lowercase().contains(filter)
}

/// Whether a micromode style matches the search: by its name, what it
/// says, or its micromode's name in Spanish or English.
fn micromode_style_matches(filter: &str, style: &MicromodeStyle) -> bool {
    let micromode = style.micromode();
    filter.is_empty()
        || names_a_word(filter, "micromodes micromodos kröpfl kropfl trichords")
        || style.name.to_lowercase().contains(filter)
        || style.about.to_lowercase().contains(filter)
        || micromode.name.to_lowercase().contains(filter)
        || micromode.english.to_lowercase().contains(filter)
}

/// Whether anything at all in the library matches the search.
fn anything_else_matches(filter: &str, purpose: LibraryPurpose) -> bool {
    purpose == LibraryPurpose::Play
        && (names_a_word(filter, "fill fills")
            || FILLS
                .iter()
                .any(|r| matches(filter, r.name, r.genre, r.time, r.about))
            || ROW_STYLES
                .iter()
                .any(|style| row_style_matches(filter, style))
            || MICROMODE_STYLES
                .iter()
                .any(|style| micromode_style_matches(filter, style)))
}

/// Whether `filter` is the start of one of `words` (at least three
/// letters of it), so that "fil" finds the fills but "l" does not.
fn names_a_word(filter: &str, words: &str) -> bool {
    filter.chars().count() >= 3
        && words
            .split_whitespace()
            .any(|word| word.starts_with(filter))
}

/// The tooltip of a style: its chords in the project's key, and more.
fn describe_style(style: &Style, tonic: lineup_core::PitchClass) -> String {
    let scale = style.scale(tonic);
    let numerals = style.chart.split_whitespace().collect::<Vec<_>>().join(" ");
    let rhythm = if style.rhythm.is_empty() {
        String::new()
    } else {
        format!(" · drums: {}", style.rhythm)
    };
    format!(
        "{}\n\nIn {}: {}\n{}\n\n{} · {} BPM · {} bars{rhythm}",
        style.about,
        scale.name(),
        style.chord_names(tonic),
        numerals,
        style.time,
        style.tempo,
        style.bars(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::BandStyle;
    use lineup_core::library::STYLES;

    #[test]
    fn the_library_follows_repeated_band_changes() {
        let mut project = Project::with_band();
        for name in ["Pop beat", "Waltz", "Bossa nova", "Pop beat", "Waltz"] {
            let band = BandStyle::named(name).unwrap();
            project.set_band_style(Some(band));
            let tried = Tried::from_band(project.band().unwrap());
            assert_eq!(tried.rhythm, band.rhythm);
            assert!(tried.style.is_none());
        }
    }

    #[test]
    fn the_exact_style_stays_selected_across_repeated_changes_and_loading() {
        let mut project = Project::with_band();
        for style in STYLES.iter().cycle().take(150) {
            let Some(band) = BandStyle::of_style(style) else {
                continue;
            };
            project.apply_band_parts(
                band,
                Some(style),
                StyleParts {
                    time_signature: false,
                    tempo: false,
                    ..StyleParts::default()
                },
            );
            let tried = Tried::from_band(project.band().unwrap());
            assert_eq!(tried.style, Some(style));
            assert_eq!(tried.rhythm, band.rhythm);
            assert_eq!(project.band().unwrap().name(), style.name);
        }
        let text = lineup_core::file::to_text(&project).unwrap();
        let loaded = lineup_core::file::from_text(&text).unwrap();
        assert_eq!(
            Tried::from_band(loaded.band().unwrap()),
            Tried::from_band(project.band().unwrap())
        );
    }
}
