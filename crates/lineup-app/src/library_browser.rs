//! The Library tab, next to the list of patterns: harmony styles and
//! rhythms, sorted by genre, to try with one click.
//!
//! Like the other views, it only draws and reads the mouse. A click
//! returns a [`LibraryAction`], and the app writes the style or rhythm
//! into a pattern (see `app/library.rs`).

use eframe::egui::{self, RichText, Ui};
use lineup_core::library::{
    FILLS, Genre, MICROMODE_STYLES, MicromodeStyle, RHYTHMS, ROW_STYLES, Rhythm, RowStyle, STYLES,
    Style,
};
use lineup_core::{Project, TimeSignature};

/// Something picked in the library.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibraryAction {
    /// Play this style's chords and bass line, in the project's key.
    Style(&'static Style),
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

/// What the selected pattern was made from, if the library made it, to
/// show it selected.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Tried {
    pub style: Option<&'static Style>,
    pub rhythm: Option<&'static Rhythm>,
    pub row_style: Option<&'static RowStyle>,
    pub micromode_style: Option<&'static MicromodeStyle>,
}

/// What the library tab remembers between frames.
pub struct LibraryBrowser {
    filter: String,
    /// Whether trying a style or a rhythm also sets its usual tempo.
    pub use_tempo: bool,
    /// The selection last drawn. When it changes (a demo was opened, or
    /// something was tried), the genre holding what changed opens and it
    /// scrolls into view.
    shown: Tried,
    /// Until when (in `ui` time) to keep scrolling the selection into
    /// view, while its genre opens.
    reveal_until: f64,
    /// Whether that is the rhythm (only the rhythm changed) or the style.
    reveal_rhythm: bool,
    /// Whether it is a twelve-tone style.
    reveal_row: bool,
}

impl LibraryBrowser {
    pub fn new() -> Self {
        LibraryBrowser {
            filter: String::new(),
            use_tempo: true,
            shown: Tried::default(),
            reveal_until: 0.0,
            reveal_rhythm: false,
            reveal_row: false,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, project: &Project, tried: Tried) -> Option<LibraryAction> {
        let mut picked = None;
        ui.add(
            egui::TextEdit::singleline(&mut self.filter)
                .hint_text("🔍 Search: bossa, 6/8, tango…")
                .desired_width(f32::INFINITY),
        );
        ui.checkbox(&mut self.use_tempo, "Use their tempo")
            .on_hover_text(
                "Trying a style or a rhythm also sets the tempo it is usually played at",
            );
        ui.weak(
            "Click to play it in a pattern. Click another to swap. While the song plays, a \
             click plays the whole song that way instead, and the song carries on. Right-click \
             a rhythm to do that while stopped. Ctrl+Z takes it back.",
        );
        ui.add_space(2.0);

        let filter = self.filter.trim().to_lowercase();
        let searching = !filter.is_empty();
        let tonic = project.scale.root;
        let now = ui.input(|i| i.time);
        let new_style = tried.style.is_some() && tried.style != self.shown.style;
        let new_rhythm = tried.rhythm.is_some() && tried.rhythm != self.shown.rhythm;
        let new_row = (tried.row_style.is_some() && tried.row_style != self.shown.row_style)
            || (tried.micromode_style.is_some()
                && tried.micromode_style != self.shown.micromode_style);
        self.shown = tried;
        if new_style || new_rhythm || new_row {
            self.reveal_until = now + 0.5;
            self.reveal_rhythm = !new_style && !new_row;
            self.reveal_row = new_row;
        }
        let revealing = now < self.reveal_until;
        if revealing {
            ui.ctx().request_repaint();
        }
        let scroll_row = revealing && self.reveal_row;
        let scroll_style = revealing && !self.reveal_rhythm && !self.reveal_row;
        let scroll_rhythm = revealing && self.reveal_rhythm;
        if searching && !anything_matches(&filter) {
            ui.label(format!(
                "Nothing in the library matches \"{}\".",
                self.filter.trim()
            ));
            ui.weak("Songs with melodies are in File ▸ Demos ▸ Songs.");
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            let open = searching || new_style;
            section(ui, "Harmony styles", open, |ui| {
                for genre in Genre::ALL {
                    let styles: Vec<&'static Style> = STYLES
                        .iter()
                        .filter(|style| style.genre == genre)
                        .filter(|style| {
                            matches(&filter, style.name, genre, style.time, style.about)
                        })
                        .collect();
                    let open =
                        searching || (new_style && tried.style.is_some_and(|s| s.genre == genre));
                    group(ui, "styles", genre, styles.len(), open, |ui| {
                        for style in styles {
                            let selected = tried.style == Some(style);
                            let response = ui
                                .selectable_label(selected, label(style.name, style.time))
                                .on_hover_text(describe_style(style, tonic));
                            if selected && scroll_style {
                                response.scroll_to_me(None);
                            }
                            if response.clicked() {
                                picked = Some(LibraryAction::Style(style));
                            }
                        }
                    });
                }
            });
            let open = searching || new_row;
            picked = picked.or(row_styles(ui, &filter, open, tried, scroll_row));
            picked = picked.or(micromode_styles(ui, &filter, open, tried, scroll_row));
            let open = searching || new_rhythm;
            section(ui, "Rhythms", open, |ui| {
                for genre in Genre::ALL {
                    let rhythms: Vec<&'static Rhythm> = RHYTHMS
                        .iter()
                        .filter(|rhythm| rhythm.genre == genre)
                        .filter(|r| matches(&filter, r.name, genre, r.time, r.about))
                        .collect();
                    let open = searching
                        || (new_rhythm
                            && tried
                                .rhythm
                                .is_some_and(|r| r.genre == genre && !FILLS.contains(r)));
                    group(ui, "rhythms", genre, rhythms.len(), open, |ui| {
                        for rhythm in rhythms {
                            picked = picked.or(rhythm_label(ui, rhythm, tried, scroll_rhythm));
                        }
                    });
                }
                let fills: Vec<&'static Rhythm> = FILLS
                    .iter()
                    .filter(|r| {
                        matches(&filter, r.name, r.genre, r.time, r.about)
                            || names_a_word(&filter, "fill fills")
                    })
                    .collect();
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
                            picked = picked.or(rhythm_label(ui, fill, tried, scroll_rhythm));
                        }
                    });
                }
            });
        });
        picked
    }
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
    let styles: Vec<&'static RowStyle> = ROW_STYLES
        .iter()
        .filter(|style| row_style_matches(filter, style))
        .collect();
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
    let styles: Vec<&'static MicromodeStyle> = MICROMODE_STYLES
        .iter()
        .filter(|style| micromode_style_matches(filter, style))
        .collect();
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

/// A top-level part of the library, open at the start, and opened when
/// `open` is true (while searching, or to show the selection).
fn section(ui: &mut Ui, title: &str, open: bool, add: impl FnOnce(&mut Ui)) {
    let header = egui::CollapsingHeader::new(RichText::new(title).strong()).default_open(true);
    let header = if open {
        header.open(Some(true))
    } else {
        header
    };
    header.show(ui, add);
}

/// The entries of one genre, under its name. Closed at the start, opened
/// when `open` is true (while searching, or when it holds the selection),
/// and left out if nothing in it matches.
fn group(
    ui: &mut Ui,
    kind: &str,
    genre: Genre,
    count: usize,
    open: bool,
    add: impl FnOnce(&mut Ui),
) {
    if count == 0 {
        return;
    }
    let header = egui::CollapsingHeader::new(format!("{} ({count})", genre.name()))
        .id_salt(format!("{kind}-{genre:?}"));
    let header = if open {
        header.open(Some(true))
    } else {
        header
    };
    header.show(ui, add);
}

/// One rhythm in the list. Returns it if it was clicked, or picked for
/// the whole song from its menu. Fills have no menu: they are one bar.
/// With `scroll`, a selected rhythm scrolls into view.
fn rhythm_label(
    ui: &mut Ui,
    rhythm: &'static Rhythm,
    tried: Tried,
    scroll: bool,
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
        if is_fill {
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
    if !is_fill {
        response.context_menu(|ui| {
            if ui
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
    }
    picked
}

/// The name, with the time signature after it unless it is 4/4.
fn label(name: &str, time: TimeSignature) -> String {
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
fn anything_matches(filter: &str) -> bool {
    STYLES
        .iter()
        .any(|s| matches(filter, s.name, s.genre, s.time, s.about))
        || RHYTHMS
            .iter()
            .chain(FILLS.iter())
            .any(|r| matches(filter, r.name, r.genre, r.time, r.about))
        || names_a_word(filter, "fill fills")
        || ROW_STYLES
            .iter()
            .any(|style| row_style_matches(filter, style))
        || MICROMODE_STYLES
            .iter()
            .any(|style| micromode_style_matches(filter, style))
}

/// Whether `filter` is the start of one of `words` (at least three
/// letters of it), so that "fil" finds the fills but "l" does not.
fn names_a_word(filter: &str, words: &str) -> bool {
    filter.chars().count() >= 3
        && words
            .split_whitespace()
            .any(|word| word.starts_with(filter))
}

/// Whether an entry matches the search: its name, genre, time signature
/// or description contains the text.
fn matches(filter: &str, name: &str, genre: Genre, time: TimeSignature, about: &str) -> bool {
    filter.is_empty()
        || name.to_lowercase().contains(filter)
        || genre.name().to_lowercase().contains(filter)
        || time.to_string() == filter
        || about.to_lowercase().contains(filter)
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
        "{}\n\nIn {}: {}\n{}\n\n{} · {} BPM · {} bars{rhythm}\n\nWhile the song plays, a \
         click plays the whole song this way, keeping its chords.",
        style.about,
        scale.name(),
        style.chord_names(tonic),
        numerals,
        style.time,
        style.tempo,
        style.bars(),
    )
}
