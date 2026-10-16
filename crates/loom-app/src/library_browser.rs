//! The Library tab, next to the list of patterns: harmony styles and
//! rhythms, sorted by genre, to try with one click.
//!
//! Like the other views, it only draws and reads the mouse. A click
//! returns a [`LibraryAction`], and the app writes the style or rhythm
//! into a pattern (see `app/library.rs`).

use eframe::egui::{self, RichText, Ui};
use loom_core::library::{FILLS, Genre, RHYTHMS, Rhythm, STYLES, Style};
use loom_core::{Project, TimeSignature};

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
}

/// What the selected pattern was made from, if the library made it, to
/// show it selected.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Tried {
    pub style: Option<&'static Style>,
    pub rhythm: Option<&'static Rhythm>,
}

/// What the library tab remembers between frames.
pub struct LibraryBrowser {
    filter: String,
    /// Whether trying a style or a rhythm also sets its usual tempo.
    pub use_tempo: bool,
}

impl LibraryBrowser {
    pub fn new() -> Self {
        LibraryBrowser {
            filter: String::new(),
            use_tempo: true,
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
            "Click to play it in a pattern. Click another to swap. Right-click a rhythm to \
             play the whole song in it, as a click does while the song plays. Ctrl+Z takes \
             it back.",
        );
        ui.add_space(2.0);

        let filter = self.filter.trim().to_lowercase();
        let searching = !filter.is_empty();
        let tonic = project.scale.root;
        egui::ScrollArea::vertical().show(ui, |ui| {
            section(ui, "Harmony styles", searching, |ui| {
                for genre in Genre::ALL {
                    let styles: Vec<&'static Style> = STYLES
                        .iter()
                        .filter(|style| style.genre == genre)
                        .filter(|style| {
                            matches(&filter, style.name, genre, style.time, style.about)
                        })
                        .collect();
                    group(ui, "styles", genre, styles.len(), searching, |ui| {
                        for style in styles {
                            let selected = tried.style == Some(style);
                            let response = ui
                                .selectable_label(selected, label(style.name, style.time))
                                .on_hover_text(describe_style(style, tonic));
                            if response.clicked() {
                                picked = Some(LibraryAction::Style(style));
                            }
                        }
                    });
                }
            });
            section(ui, "Rhythms", searching, |ui| {
                for genre in Genre::ALL {
                    let rhythms: Vec<&'static Rhythm> = RHYTHMS
                        .iter()
                        .filter(|rhythm| rhythm.genre == genre)
                        .filter(|r| matches(&filter, r.name, genre, r.time, r.about))
                        .collect();
                    group(ui, "rhythms", genre, rhythms.len(), searching, |ui| {
                        for rhythm in rhythms {
                            picked = picked.or(rhythm_label(ui, rhythm, tried));
                        }
                    });
                }
                let fills: Vec<&'static Rhythm> = FILLS
                    .iter()
                    .filter(|r| {
                        matches(&filter, r.name, r.genre, r.time, r.about)
                            || "fills".contains(&filter)
                    })
                    .collect();
                if !fills.is_empty() {
                    let header = egui::CollapsingHeader::new(format!("Fills ({})", fills.len()))
                        .id_salt("rhythms-fills");
                    let header = if searching {
                        header.open(Some(true))
                    } else {
                        header
                    };
                    header.show(ui, |ui| {
                        for fill in fills {
                            picked = picked.or(rhythm_label(ui, fill, tried));
                        }
                    });
                }
            });
        });
        picked
    }
}

/// A top-level part of the library, open at the start.
fn section(ui: &mut Ui, title: &str, searching: bool, add: impl FnOnce(&mut Ui)) {
    let header = egui::CollapsingHeader::new(RichText::new(title).strong()).default_open(true);
    let header = if searching {
        header.open(Some(true))
    } else {
        header
    };
    header.show(ui, add);
}

/// The entries of one genre, under its name. Closed at the start, open
/// while searching, and left out if nothing in it matches.
fn group(
    ui: &mut Ui,
    kind: &str,
    genre: Genre,
    count: usize,
    searching: bool,
    add: impl FnOnce(&mut Ui),
) {
    if count == 0 {
        return;
    }
    let header = egui::CollapsingHeader::new(format!("{} ({count})", genre.name()))
        .id_salt(format!("{kind}-{genre:?}"));
    let header = if searching {
        header.open(Some(true))
    } else {
        header
    };
    header.show(ui, add);
}

/// One rhythm in the list. Returns it if it was clicked, or picked for
/// the whole song from its menu. Fills have no menu: they are one bar.
fn rhythm_label(ui: &mut Ui, rhythm: &'static Rhythm, tried: Tried) -> Option<LibraryAction> {
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
            ""
        } else {
            "\n\nRight-click to play the whole song in this rhythm. While the song plays, \
             a click does that too."
        }
    );
    let response = ui
        .selectable_label(selected, label(rhythm.name, rhythm.time))
        .on_hover_text(hover);
    let mut picked = response.clicked().then_some(LibraryAction::Rhythm(rhythm));
    if !is_fill {
        response.context_menu(|ui| {
            if ui
                .button("Play the whole song in this rhythm")
                .on_hover_text(
                    "Every pattern with drums gets this rhythm, and the chords and bass \
                     are played the way it goes, keeping the same chords",
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
fn describe_style(style: &Style, tonic: loom_core::PitchClass) -> String {
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
