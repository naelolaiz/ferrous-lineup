//! The Library tab: searchable accompaniment presets, shown alphabetically
//! or grouped by family, with optional chord progressions and copy controls.
//!
//! Like the other views, it only draws and reads the mouse. A click
//! returns a [`LibraryAction`], and the app writes the style or rhythm
//! into a pattern (see `app/library.rs`).

use eframe::egui::{self, RichText, Ui};
use lineup_core::library::apply::StyleParts;
use lineup_core::library::{
    FILLS, Genre, MICROMODE_STYLES, MicromodeStyle, ROW_STYLES, Rhythm, RowStyle, Style,
};
use lineup_core::{Band, Project, SongPart, SourceKind, TimeSignature};

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

/// The same browser serves accompaniment presets and written progressions.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LibraryPurpose {
    Play,
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

/// Where the Library options are saved between runs.
const OPTIONS: &str = "library-options";

/// Where they were saved while the Tempo option started ticked.
const OPTIONS_TEMPO_ON: &str = "preset-copy-options";

/// The tick boxes under the search, which keep only the presets like
/// the song. Saved with the app's state.
#[derive(Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct Filters {
    /// In the song's time signature.
    time: bool,
    /// Made for about the song's speed.
    speed: bool,
    /// Bringing a chord progression.
    chords: bool,
}

/// How far a preset's own speed may be from the song's for "Near the
/// song's speed": 15% either way, so 110 to 150 BPM for a song at 130.
const NEAR_SPEED: f32 = 1.15;

/// The [`Filters`] ticked, with what they compare against.
#[derive(Clone, Copy)]
struct Narrowing {
    filters: Filters,
    /// The song's first time signature.
    time: TimeSignature,
    /// The song's speed, in beats a minute.
    tempo: f32,
}

impl Narrowing {
    fn of(filters: Filters, project: &Project) -> Self {
        Narrowing {
            filters,
            time: project.time_signature,
            tempo: project.tempo.bpm(),
        }
    }

    /// Whether a preset in `time`, made for `tempo`, is kept. One that
    /// plays every bar in its own meter (`any_meter`) fits any time
    /// signature; `chords` says whether it brings a progression.
    fn keeps(self, time: TimeSignature, tempo: f32, any_meter: bool, chords: bool) -> bool {
        let filters = self.filters;
        // 7/8 as 2+2+3 is still 7/8.
        let same_time = any_meter || time.ungrouped() == self.time.ungrouped();
        let ratio = tempo / self.tempo;
        let near = ratio.max(1.0 / ratio) <= NEAR_SPEED;
        (!filters.time || same_time) && (!filters.speed || near) && (!filters.chords || chords)
    }

    fn any(self) -> bool {
        self.filters != Filters::default()
    }
}

fn filters_id() -> egui::Id {
    egui::Id::unique("library-filters")
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

    pub fn restore_preferences(&mut self, ctx: &egui::Context) {
        self.parts = ctx
            .data_mut(|data| {
                data.get_persisted(egui::Id::unique(OPTIONS)).or_else(|| {
                    // Options saved while Tempo started ticked: it was
                    // ticked for everyone, so it starts off now like a new
                    // one, and the other choices stay.
                    data.get_persisted(egui::Id::unique(OPTIONS_TEMPO_ON))
                        .map(|parts| StyleParts {
                            tempo: false,
                            ..parts
                        })
                })
            })
            .unwrap_or_default();
    }

    pub fn store_preferences(&self, ctx: &egui::Context) {
        ctx.data_mut(|data| {
            data.insert_persisted(egui::Id::unique(OPTIONS), self.parts);
        });
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
        if ui.small_button("New style…").clicked() {
            self.editor_request = Some(StyleEditorRequest::New);
        }
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
        let saved: Filters = ui
            .ctx()
            .data_mut(|data| data.get_persisted(filters_id()))
            .unwrap_or_default();
        let mut filters = saved;
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut filters.time, format!("In {}", project.time_signature))
                .on_hover_text(
                    "Only presets in the song's time signature, and those that play every bar \
                     in its own meter",
                );
            ui.checkbox(
                &mut filters.speed,
                format!("Near {:.0} BPM", project.tempo.bpm()),
            )
            .on_hover_text(
                "Only presets made for about the song's speed: 15% faster or slower at most",
            );
            if purpose != LibraryPurpose::Progression {
                ui.checkbox(&mut filters.chords, "With chords")
                    .on_hover_text("Only presets that bring a chord progression");
            }
        });
        if filters != saved {
            ui.ctx()
                .data_mut(|data| data.insert_persisted(filters_id(), filters));
        }
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
        let sources = SongPart::ALL.map(|part| part_source_text(project, part));
        let locked = project.locked_names().join(", ");
        if purpose != LibraryPurpose::Progression {
            part_sources(ui, project, &sources, &locked);
            if project.band().is_some_and(|band| band.accents_only) {
                ui.weak("Plain accents with current instruments");
            }
        }
        ui.weak(match purpose {
            LibraryPurpose::Play => {
                "Click a preset to choose what to copy, then Apply. Ctrl-click applies the \
                 previous options immediately. While the song plays, it carries on with the \
                 new preset. Right-click to apply to the whole song while stopped. Ctrl+Z undoes it."
            }
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
        let narrowing = Narrowing::of(
            ui.ctx()
                .data_mut(|data| data.get_persisted(filters_id()))
                .unwrap_or_default(),
            project,
        );
        let mut presets = catalogue::presets(
            &filter,
            &self.user_styles,
            purpose == LibraryPurpose::Progression,
        );
        presets.retain(|preset| {
            narrowing.keeps(
                preset.time(),
                preset.tempo(),
                preset.follows_the_song(),
                preset.has_progression(),
            )
        });
        if (searching || narrowing.any())
            && presets.is_empty()
            && !anything_else_matches(&filter, narrowing, purpose)
        {
            if searching {
                ui.label(format!(
                    "Nothing in the library matches \"{}\".",
                    self.filter.trim()
                ));
            } else {
                ui.label("Nothing in the library matches the ticked boxes.");
            }
            if narrowing.any() {
                ui.weak("Untick a box to see more.");
            }
            ui.weak("Songs with melodies are in File ▸ Demos ▸ Songs.");
        }
        ui.scope(|ui| {
            ui.label(format!("Presets ({})", presets.len()));
            if purpose != LibraryPurpose::Progression {
                ui.weak("+ chords: includes a progression. Other presets keep your chords.");
            }
            let mut draw = |ui: &mut Ui, preset| {
                if let Some(action) = preset_label(
                    ui,
                    preset,
                    PresetView {
                        tried,
                        tonic,
                        reveal: revealing && !self.reveal_row,
                        purpose,
                        now: &sources,
                        locked: &locked,
                    },
                    &mut self.parts,
                    &mut self.editor_request,
                ) {
                    picked = Some(action);
                }
            };
            if grouped_presets(ui.ctx()) {
                grouped_results(
                    ui,
                    &presets,
                    tried,
                    searching,
                    revealing || !tracking,
                    &mut draw,
                );
            } else {
                for preset in presets {
                    draw(ui, preset);
                }
            }
            if purpose == LibraryPurpose::Play {
                let open = searching || new_row;
                picked = picked.or(row_styles(ui, &filter, narrowing, open, tried, scroll_row));
                picked = picked.or(micromode_styles(
                    ui, &filter, narrowing, open, tried, scroll_row,
                ));
            }
            if purpose == LibraryPurpose::Progression {
                return;
            }
            if purpose == LibraryPurpose::Play {
                let fills = alphabetical(
                    FILLS.iter().filter(|r| {
                        (matches(&filter, r.name, r.genre, r.time, r.about)
                            || names_a_word(&filter, "fill fills"))
                            && narrowing.keeps(r.time, r.tempo, r.follows_the_meter, false)
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

/// The presentation preference is shared by every preset browser.
pub fn grouped_presets(ctx: &egui::Context) -> bool {
    ctx.data_mut(|data| {
        data.get_persisted(egui::Id::unique("group-presets"))
            .unwrap_or(false)
    })
}

pub fn set_grouped_presets(ctx: &egui::Context, grouped: bool) {
    ctx.data_mut(|data| data.insert_persisted(egui::Id::unique("group-presets"), grouped));
}

fn preset_selected(preset: Preset<'_>, tried: Tried) -> bool {
    match preset {
        Preset::Style(style) => {
            tried.style == Some(style)
                || (tried.style.is_none()
                    && tried.user_style.is_none()
                    && tried.row_style.is_none()
                    && tried.micromode_style.is_none()
                    && tried.rhythm.and_then(catalogue::equivalent_style) == Some(style))
        }
        Preset::Rhythm(rhythm) => {
            tried.user_style.is_none()
                && tried.style.is_none()
                && tried.row_style.is_none()
                && tried.micromode_style.is_none()
                && tried.rhythm == Some(rhythm)
        }
        Preset::User(_, style) => tried.user_style == Some(style.id()),
    }
}

fn grouped_results<'a>(
    ui: &mut Ui,
    presets: &[Preset<'a>],
    tried: Tried,
    searching: bool,
    reveal: bool,
    draw: &mut impl FnMut(&mut Ui, Preset<'a>),
) {
    let users: Vec<_> = presets
        .iter()
        .copied()
        .filter(|preset| preset.genre().is_none())
        .collect();
    if !users.is_empty() {
        let open =
            searching || (reveal && users.iter().any(|&preset| preset_selected(preset, tried)));
        let header = egui::CollapsingHeader::new(format!("User styles ({})", users.len()))
            .id_salt("preset-users");
        let header = if open {
            header.open(Some(true))
        } else {
            header
        };
        header.show(ui, |ui| {
            for preset in users {
                draw(ui, preset);
            }
        });
    }
    let mut genres = Genre::ALL;
    genres.sort_by_key(|genre| genre.name());
    for genre in genres {
        let group: Vec<_> = presets
            .iter()
            .copied()
            .filter(|preset| preset.genre() == Some(genre))
            .collect();
        if group.is_empty() {
            continue;
        }
        let open =
            searching || (reveal && group.iter().any(|&preset| preset_selected(preset, tried)));
        let header = egui::CollapsingHeader::new(format!("{} ({})", genre.name(), group.len()))
            .id_salt(("preset-genre", genre));
        let header = if open {
            header.open(Some(true))
        } else {
            header
        };
        header.show(ui, |ui| {
            for preset in group {
                draw(ui, preset);
            }
        });
    }
}

#[derive(Clone, Copy)]
struct PresetView<'a> {
    tried: Tried,
    tonic: lineup_core::PitchClass,
    reveal: bool,
    purpose: LibraryPurpose,
    /// Where each part of the song comes from now, in [`SongPart::ALL`]'s
    /// order (see [`part_source_text`]).
    now: &'a [String; 6],
    /// The names of the locked channels, "Bass, Keys", or nothing.
    locked: &'a str,
}

/// Where a part of the song comes from, in words: "Tango marcato
/// (style)", "edited, was El Choclo", or "yours" for a part that never
/// came from a demo or a style.
pub(crate) fn part_source_text(project: &Project, part: SongPart) -> String {
    match project.part_source(part) {
        Some(origin) if origin.edited => format!("edited, was {}", origin.source.name),
        Some(origin) => {
            let kind = match origin.source.kind {
                SourceKind::Demo => "demo",
                SourceKind::Style => "style",
                SourceKind::Rhythm => "rhythm",
                SourceKind::SavedStyle => "saved style",
            };
            format!("{} ({kind})", origin.source.name)
        }
        None => "yours".to_string(),
    }
}

/// What a part of the song is now, where a few words say it: "3/4",
/// "120 BPM", "even".
fn part_value(project: &Project, part: SongPart) -> Option<String> {
    match part {
        SongPart::TimeSignature => {
            let changes = !project.playlist().time_markers().is_empty();
            Some(format!(
                "{}{}",
                project.time_signature,
                if changes { " and others" } else { "" }
            ))
        }
        SongPart::Tempo => Some(format!("{:.0} BPM", project.tempo.bpm())),
        SongPart::Swing => Some(match project.swing {
            0 => "even".to_string(),
            swing => format!("{swing}%"),
        }),
        SongPart::Patterns | SongPart::Instruments | SongPart::Chords => None,
    }
}

/// Where each part of the song comes from: a demo, a style, your edits.
/// And the channels styles leave alone, if any are `locked`.
fn part_sources(ui: &mut Ui, project: &Project, now: &[String; 6], locked: &str) {
    egui::Grid::new("part-sources")
        .num_columns(2)
        .spacing([10.0, 2.0])
        .show(ui, |ui| {
            for (part, source) in SongPart::ALL.into_iter().zip(now) {
                ui.weak(part.name());
                let text = match part_value(project, part) {
                    Some(value) => format!("{value} · {source}"),
                    None => source.clone(),
                };
                let hover = match project.part_source(part) {
                    Some(origin) if origin.edited => {
                        format!("You changed it since it came from {}.", origin.source.name)
                    }
                    Some(origin) => format!("As it came from {}.", origin.source.name),
                    None => "Your own: it did not come from a demo or a style.".to_string(),
                };
                ui.label(text).on_hover_text(hover);
                ui.end_row();
            }
            let meter = project.meter();
            let sections: Vec<String> = project
                .section_styles()
                .iter()
                .map(|marker| {
                    format!(
                        "bar {}: {}",
                        meter.bar_at(marker.start).number + 1,
                        project.style_name_at(marker.start)
                    )
                })
                .collect();
            if !sections.is_empty() {
                ui.weak("Sections");
                ui.label(sections.join(", ")).on_hover_text(
                    "Where the band plays another style. Right-click a bar or a section on \
                     the chord sheet to change it.",
                );
                ui.end_row();
            }
            if !locked.is_empty() {
                ui.weak("Locked");
                ui.label(format!("🔒 {locked}")).on_hover_text(
                    "Styles and the band keep their notes and sound. Unlock them in the \
                     channel rack.",
                );
                ui.end_row();
            }
        });
}

fn preset_label(
    ui: &mut Ui,
    preset: Preset<'_>,
    view: PresetView<'_>,
    parts: &mut StyleParts,
    editor: &mut Option<StyleEditorRequest>,
) -> Option<LibraryAction> {
    let selected = preset_selected(preset, view.tried);
    let hover = match preset {
        Preset::Style(style) => describe_style(style, view.tonic),
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
    // Harmony and rhythm presets can share both a name and a genre.
    let identity = (
        "preset",
        std::mem::discriminant(&preset),
        preset.name(),
        preset.genre(),
        match preset {
            Preset::User(_, style) => Some(style.id()),
            _ => None,
        },
    );
    ui.push_id(identity, |ui| {
        let response = ui
            .horizontal_wrapped(|ui| {
                let name = if preset.follows_the_song() {
                    format!("{}  (any meter)", preset.name())
                } else {
                    label(preset.name(), preset.time())
                };
                let response = ui.selectable_label(selected, name);
                if view.purpose != LibraryPurpose::Progression {
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
        if selected && view.reveal {
            response.scroll_to_me(None);
        }
        let action = match preset {
            Preset::Style(style) => LibraryAction::Style(style),
            Preset::Rhythm(rhythm) => LibraryAction::Rhythm(rhythm),
            Preset::User(index, _) => LibraryAction::UserStyle(index),
        };
        // This uses a different ID from the right-click editor menu.
        let popup_id = response.id.with("copy-options");
        let mut picked = None;
        let clicked = response.clicked();
        let immediate = ui.input(|input| input.modifiers.ctrl || input.modifiers.command)
            || view.purpose == LibraryPurpose::Progression;
        if clicked && immediate {
            egui::Popup::close_id(ui.ctx(), popup_id);
            picked = Some(action);
        }
        let opening = clicked && !immediate;
        let mut draft = ui.ctx().data_mut(|data| {
            if opening {
                data.insert_temp(popup_id, *parts);
            }
            data.get_temp::<StyleParts>(popup_id).unwrap_or(*parts)
        });
        let popup = egui::Popup::from_response(&response)
            .id(popup_id)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .width(300.0);
        let popup = popup.open_memory(opening.then_some(egui::SetOpenCommand::Bool(true)));
        popup.show(|ui| {
            ui.strong(preset.name());
            let height = (ui.ctx().content_rect().height() - 110.0).max(80.0);
            egui::ScrollArea::vertical()
                .max_height(height)
                .show(ui, |ui| {
                    style_parts_editor(
                        ui,
                        &mut draft,
                        preset.has_progression(),
                        preset.follows_the_song(),
                        Some(view.now),
                        view.locked,
                    );
                });
            ui.separator();
            ui.horizontal(|ui| {
                let effective = draft.for_preset();
                let can_apply = effective.rhythm
                    || effective.instruments
                    || effective.time_signature
                    || effective.tempo
                    || effective.swing
                    || (effective.progression && preset.has_progression());
                if ui
                    .add_enabled(can_apply, egui::Button::new("Apply"))
                    .clicked()
                {
                    *parts = draft;
                    picked = Some(action);
                    ui.close();
                }
                if ui.button("Cancel").clicked() {
                    ui.close();
                }
            });
        });
        ui.ctx().data_mut(|data| data.insert_temp(popup_id, draft));
        response.context_menu(|ui| {
            let edit_label = if matches!(preset, Preset::User(_, _)) {
                "Edit in channel rack…"
            } else {
                "Edit a copy in channel rack…"
            };
            if ui.button(edit_label).clicked() {
                *editor = Some(match preset {
                    Preset::Style(style) => StyleEditorRequest::Builtin(style),
                    Preset::Rhythm(rhythm) => StyleEditorRequest::Rhythm(rhythm),
                    Preset::User(index, _) => StyleEditorRequest::User(index),
                });
                ui.close();
            }
            if view.purpose == LibraryPurpose::Play
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
        picked
    })
    .inner
}

/// Shared option editor for applying a preset and editing the saved defaults.
pub fn style_parts_checkboxes(ui: &mut Ui, parts: &mut StyleParts) {
    style_parts_editor(ui, parts, true, false, None, "");
}

/// The options for applying a preset. With `now`, each says where that
/// part of the song comes from now, which it would replace. The
/// `locked` channels, if any, are named as kept. A preset that `follows`
/// the song keeps its time signatures and tempo.
fn style_parts_editor(
    ui: &mut Ui,
    parts: &mut StyleParts,
    has_progression: bool,
    follows: bool,
    now: Option<&[String; 6]>,
    locked: &str,
) {
    ui.spacing_mut().item_spacing.y = 8.0;
    ui.add_space(4.0);
    ui.weak("Playing style");
    style_mode_ui(ui, parts);
    ui.separator();
    ui.strong("Accompaniment");
    if !locked.is_empty() {
        ui.weak(format!("🔒 Kept: {locked}"));
    }
    copy_parts_ui(ui, parts, has_progression, follows, now);
}

/// A part's option, and beside it where that part comes from now, if
/// known.
fn with_source(
    ui: &mut Ui,
    now: Option<&[String; 6]>,
    part: SongPart,
    option: impl FnOnce(&mut Ui),
) {
    let Some(now) = now else {
        option(ui);
        return;
    };
    ui.horizontal(|ui| {
        option(ui);
        let text = RichText::new(format!("now: {}", now[part as usize]))
            .small()
            .weak();
        ui.add(egui::Label::new(text).truncate());
    });
}

pub(crate) fn style_mode_ui(ui: &mut Ui, parts: &mut StyleParts) {
    ui.horizontal_wrapped(|ui| {
        ui.selectable_value(&mut parts.matching_style, true, "Preset patterns");
        ui.selectable_value(&mut parts.matching_style, false, "Plain accents");
    });
    ui.weak(if parts.matching_style {
        "Use the preset’s playing patterns and choose whether to copy its instruments."
    } else {
        "Simple chord and bass accents on the meter’s beat groups. Drum patterns still come from the preset. Your current instruments and chords are kept."
    });
}

fn copy_parts_ui(
    ui: &mut Ui,
    parts: &mut StyleParts,
    has_progression: bool,
    follows: bool,
    now: Option<&[String; 6]>,
) {
    let (label, hint) = if parts.matching_style {
        (
            "Playing patterns",
            "Copy the preset’s drum, chord, bass and other accompaniment patterns. Existing chords are kept unless Chord progression is checked.",
        )
    } else {
        (
            "Drums and plain accents",
            "Use the preset’s drums with simple chord and bass accents on the meter’s beat groups, over the song’s own chords and instruments.",
        )
    };
    with_source(ui, now, SongPart::Patterns, |ui| {
        ui.checkbox(&mut parts.rhythm, label).on_hover_text(hint);
    });
    if parts.matching_style {
        with_source(ui, now, SongPart::Instruments, |ui| {
            instrument_option(ui, parts, "Voices / instruments");
        });
    }
    ui.add_space(2.0);
    ui.strong("Harmony");
    if parts.matching_style {
        let mut progression = has_progression && parts.progression;
        with_source(ui, now, SongPart::Chords, |ui| {
            if ui
                .add_enabled(
                    has_progression,
                    egui::Checkbox::new(&mut progression, "Chord progression"),
                )
                .on_hover_text("Replace the song’s chords with this preset’s chart. The melody is kept. Leave this off to keep the song’s harmony while changing its playing style or instruments: each style you try then plays the chords already there, and in a new trial pattern, those of the first style you tried.")
                .on_disabled_hover_text(
                    "This preset has no chord progression. Your current chords are kept.",
                )
                .changed()
            {
                parts.progression = progression;
            }
        });
        if has_progression && parts.progression {
            ui.weak("Replaces existing chords. The melody is kept.");
        }
    } else {
        // Plain accents play the song's own chords. Whether to copy a
        // progression is kept for when preset patterns are picked again.
        with_source(ui, now, SongPart::Chords, |ui| {
            ui.weak("Keeps the song’s chords");
        });
    }
    ui.add_space(2.0);
    ui.strong("Timing");
    if follows {
        // The ticks stay as they are for the other presets.
        ui.weak(
            "Keeps the song’s time signatures and tempo: this preset plays every bar in \
             that bar’s meter.",
        );
    } else {
        with_source(ui, now, SongPart::TimeSignature, |ui| {
            ui.checkbox(&mut parts.time_signature, "Time signature")
                .on_hover_text(
                    "Put each bar in the chosen meter. Turn off to fit the pattern into the current meter.",
                );
        });
        with_source(ui, now, SongPart::Tempo, |ui| {
            ui.checkbox(&mut parts.tempo, "Tempo").on_hover_text(
                "Use the preset’s usual tempo when the meter stays the same. Off, the song keeps its tempo. Changing meter keeps each bar's duration.",
            );
        });
    }
    with_source(ui, now, SongPart::Swing, |ui| {
        ui.checkbox(&mut parts.swing, "Swing")
            .on_hover_text("Use the preset’s swing amount.");
    });
}

/// Display the effective option while preserving the preference for preset patterns.
pub(crate) fn instrument_option(ui: &mut Ui, parts: &mut StyleParts, label: &str) {
    if parts.matching_style {
        ui.checkbox(&mut parts.instruments, label)
            .on_hover_text("Copy the preset’s sounds and band players.");
    }
}

/// The twelve-tone styles, under the harmony styles, opened when `open`
/// is true. Returns the one clicked, if any.
fn row_styles(
    ui: &mut Ui,
    filter: &str,
    narrowing: Narrowing,
    open: bool,
    tried: Tried,
    scroll: bool,
) -> Option<LibraryAction> {
    let styles = alphabetical(
        ROW_STYLES
            .iter()
            .filter(|style| row_style_matches(filter, style))
            .filter(|style| narrowing.keeps(style.time, style.tempo, false, false)),
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
    narrowing: Narrowing,
    open: bool,
    tried: Tried,
    scroll: bool,
) -> Option<LibraryAction> {
    let styles = alphabetical(
        MICROMODE_STYLES
            .iter()
            .filter(|style| micromode_style_matches(filter, style))
            .filter(|style| narrowing.keeps(style.time, style.tempo, false, false)),
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
fn anything_else_matches(filter: &str, narrowing: Narrowing, purpose: LibraryPurpose) -> bool {
    purpose == LibraryPurpose::Play
        && (FILLS.iter().any(|r| {
            (names_a_word(filter, "fill fills")
                || matches(filter, r.name, r.genre, r.time, r.about))
                && narrowing.keeps(r.time, r.tempo, r.follows_the_meter, false)
        }) || ROW_STYLES.iter().any(|style| {
            row_style_matches(filter, style)
                && narrowing.keeps(style.time, style.tempo, false, false)
        }) || MICROMODE_STYLES.iter().any(|style| {
            micromode_style_matches(filter, style)
                && narrowing.keeps(style.time, style.tempo, false, false)
        }))
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
    let chart = style.chart.split_whitespace().collect::<Vec<_>>().join(" ");
    let numerals = lineup_core::library::numeral::written_in(&chart, scale);
    let rhythm = if style.rhythm.is_empty() {
        String::new()
    } else {
        format!(" · drums: {}", style.rhythm)
    };
    let timing = if style.follows_the_song() {
        format!(
            "Any meter, at the song's tempo (on its own: {}, {} BPM)",
            style.time, style.tempo
        )
    } else {
        format!("{} · {} BPM", style.time, style.tempo)
    };
    format!(
        "{}\n\nIn {}: {}\n{}\n\n{timing} · {} bars{rhythm}",
        style.about,
        scale.name(),
        style.chord_names(tonic),
        numerals,
        style.bars(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::BandStyle;
    use lineup_core::library::STYLES;

    #[test]
    fn browser_preferences_survive_recreation_and_serialization() {
        let ctx = egui::Context::default();
        let mut browser = LibraryBrowser::new();
        browser.parts.tempo = false;
        browser.parts.progression = true;
        browser.parts.matching_style = false;
        browser.store_preferences(&ctx);
        set_grouped_presets(&ctx, true);

        // eframe writes the context's persisted data to disk between runs.
        let data = ctx.data(|data| ron::to_string(data).unwrap());
        let restored = egui::Context::default();
        restored.data_mut(|memory| *memory = ron::from_str(&data).unwrap());
        let mut next = LibraryBrowser::new();
        next.restore_preferences(&restored);
        assert_eq!(next.parts, browser.parts);
        assert!(grouped_presets(&restored));
        assert!(!grouped_presets(&egui::Context::default()));
    }

    #[test]
    fn options_saved_while_tempo_started_ticked_come_back_with_tempo_off() {
        let ctx = egui::Context::default();
        let saved = StyleParts {
            tempo: true,
            progression: true,
            matching_style: false,
            ..StyleParts::default()
        };
        ctx.data_mut(|data| data.insert_persisted(egui::Id::unique(OPTIONS_TEMPO_ON), saved));
        let mut browser = LibraryBrowser::new();
        browser.restore_preferences(&ctx);
        assert_eq!(
            browser.parts,
            StyleParts {
                tempo: false,
                ..saved
            }
        );
        // Ticked again since, it stays ticked.
        browser.parts.tempo = true;
        browser.store_preferences(&ctx);
        let mut next = LibraryBrowser::new();
        next.restore_preferences(&ctx);
        assert!(next.parts.tempo);
        assert!(!LibraryBrowser::new().parts.tempo);
    }

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
