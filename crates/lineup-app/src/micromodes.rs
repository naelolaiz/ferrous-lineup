//! The micromode window: Kröpfl's twelve groups of three notes, to look
//! at, hear, draw with and use as a key.
//!
//! A micromode is a pitch-class set of three notes, so this window and
//! the set browser point at each other: each micromode names its Forte
//! class and opens it in the set browser, and the set browser lists the
//! micromodes inside any set and opens them here. The theory is in
//! `lineup_core::harmony` (`Micromode`, `micromodes_in`); this module only
//! draws and reads the mouse.

use std::f32::consts::TAU;

use eframe::egui::{self, Color32, Pos2, RichText, Sense, Stroke, Ui, Vec2};
use lineup_core::harmony::{Micromode, MicromodeFamily, SetClass, digits, micromodes_in};
use lineup_core::{ChordStamp, Note, NoteName, PitchClass, PitchSet, Scale};

use crate::theme;
use crate::twelve_tone::{Front, NoteNames};
use crate::widgets::{self, Opening};

/// The id of the window, to keep it in front.
pub(crate) fn window_id() -> egui::Id {
    egui::Id::unique("micromode-window")
}

/// Something done in the micromode window.
pub enum MicroAction {
    /// Make this group the key.
    UseAsKey(Scale),
    /// Play these notes now.
    Hear(Vec<Note>),
    /// Show this set in the set browser.
    ShowSet(PitchSet),
}

/// The notes of `form`, written by `names`: a chord micromode spelled as
/// the chord it is, from its root (C Eb Gb, not C Eb F#); the others in
/// their normal form.
pub fn form_names(form: PitchSet, names: impl NoteNames) -> Vec<NoteName> {
    match Micromode::of(form).and_then(|m| m.chord(form)) {
        Some(chord) => names.chord_names(chord),
        None => form
            .normal_form()
            .into_iter()
            .map(|p| names.note_name(p))
            .collect(),
    }
}

/// "Menor 3 ×2", for a micromode found in a set.
pub(crate) fn found_text(micromode: &Micromode, count: u16) -> String {
    match count {
        1 => micromode.name.to_string(),
        n => format!("{} ×{n}", micromode.name),
    }
}

/// The window.
pub struct MicromodeWindow {
    pub open: bool,
    micromode: &'static Micromode,
    transposition: u8,
    inverted: bool,
    /// Whether to come to the front: a micromode was shown from the set
    /// browser.
    raise: bool,
    front: Front,
}

impl MicromodeWindow {
    pub fn new() -> Self {
        MicromodeWindow {
            open: false,
            micromode: Micromode::named("Menor 3").unwrap_or(&Micromode::all()[0]),
            transposition: 0,
            inverted: false,
            raise: false,
            front: Front::default(),
        }
    }

    /// The form chosen: the prime form, inverted if asked, moved up.
    fn chosen(&self) -> PitchSet {
        let prime = self.micromode.prime;
        let form = if self.inverted { prime.invert() } else { prime };
        form.transpose(self.transposition)
    }

    /// Shows the micromode of `form` (three notes), moved as `form` is.
    pub fn pick(&mut self, form: PitchSet) {
        if let Some(micromode) = Micromode::of(form) {
            self.micromode = micromode;
            let (n, inverted) = micromode.prime.moved_to(form).unwrap_or((0, false));
            self.transposition = n;
            self.inverted = inverted;
            self.raise = true;
        }
    }

    fn choose(&mut self, micromode: &'static Micromode) {
        if self.micromode != micromode {
            self.micromode = micromode;
            self.transposition = 0;
            self.inverted = false;
        }
    }

    /// Draws the window. `selected` is the set of the selected notes (or
    /// of the notes held on a keyboard); `names` writes the notes: pass
    /// the key, so they are spelled as it spells them.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        area: egui::Rect,
        names: impl NoteNames,
        selected: PitchSet,
        drawing: &mut ChordStamp,
    ) -> Vec<MicroAction> {
        let mut actions = Vec::new();
        let mut open = self.open;
        self.front.keep(ctx, window_id());
        if std::mem::take(&mut self.raise) {
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, window_id()));
        }
        let window = egui::Window::new(crate::i18n::tr("Micromodes"))
            .id(window_id())
            .open(&mut open)
            .default_pos([360.0, 120.0])
            .constrain_to(area)
            .resizable([false, true]);
        // As tall as its contents, up to the height of the playlist; they
        // scroll past that.
        let size = Vec2::new(580.0, area.height());
        widgets::open_at(ctx, window, window_id(), size, Opening::EveryTime).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.contents(ui, names, selected, drawing, &mut actions)
            })
        });
        self.open = open;
        actions
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        names: impl NoteNames,
        selected: PitchSet,
        drawing: &mut ChordStamp,
        actions: &mut Vec<MicroAction>,
    ) {
        ui.weak(
            "Francisco Kröpfl's twelve groups of three notes. Any three notes, in any order, \
             octave or spelling (C# and Db are one note here), moved or upside down, are one \
             of them, as they are one of Forte's twelve trichords, 3-1 to 3-12. Larger sets \
             are made of them.",
        );
        self.selection(ui, names, selected, actions);
        ui.separator();
        for family in MicromodeFamily::ALL {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(family.name()).strong());
                ui.weak(family.rule());
            });
            ui.horizontal_wrapped(|ui| {
                for micromode in Micromode::all().iter().filter(|m| m.family == family) {
                    let on = micromode == self.micromode;
                    if chip(ui, micromode, on)
                        .on_hover_text(format!(
                            "{} · {} · 3-{} {}",
                            micromode.full_name(),
                            micromode.solfege,
                            micromode.number,
                            micromode.set_class().prime_text()
                        ))
                        .clicked()
                    {
                        self.choose(micromode);
                    }
                }
            });
        }
        ui.separator();
        self.details(ui, names, drawing, actions);
    }

    /// What the selected notes are, or hold.
    fn selection(
        &mut self,
        ui: &mut Ui,
        names: impl NoteNames,
        selected: PitchSet,
        actions: &mut Vec<MicroAction>,
    ) {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Selected notes:").strong());
            match selected.len() {
                0..=2 => {
                    ui.weak(
                        "select three notes or more in the piano roll to see their micromodes.",
                    );
                }
                3 => {
                    let notes: Vec<String> = form_names(selected, names)
                        .iter()
                        .map(|n| n.to_string())
                        .collect();
                    if let Some(micromode) = Micromode::of(selected) {
                        ui.label(format!(
                            "{} are {}.",
                            notes.join(" "),
                            micromode.full_name()
                        ));
                    }
                    if ui.button("Show it").clicked() {
                        self.pick(selected);
                    }
                }
                n => {
                    let class = selected.set_class();
                    ui.label(format!(
                        "{n} notes, set {}, hold {} groups of three:",
                        class.name(),
                        n * (n - 1) * (n - 2) / 6
                    ));
                    for (micromode, count) in micromodes_in(selected) {
                        let text = found_text(micromode, count);
                        if ui
                            .selectable_label(micromode == self.micromode, text)
                            .on_hover_text(micromode.full_name())
                            .clicked()
                        {
                            self.choose(micromode);
                        }
                    }
                    if ui
                        .small_button("In Sets →")
                        .on_hover_text("Show this set in the pitch-class set browser")
                        .clicked()
                    {
                        actions.push(MicroAction::ShowSet(selected));
                    }
                }
            }
        });
    }

    fn details(
        &mut self,
        ui: &mut Ui,
        names: impl NoteNames,
        drawing: &mut ChordStamp,
        actions: &mut Vec<MicroAction>,
    ) {
        let micromode = self.micromode;
        let form = self.chosen();
        let notes = form_names(form, names);
        ui.horizontal(|ui| {
            clock(ui, form, &notes, 170.0);
            ui.vertical(|ui| {
                ui.label(RichText::new(micromode.full_name()).strong().size(16.0));
                ui.label(format!("Prime form: {}", micromode.solfege));
                ui.label(format!("Intervals: {}.", micromode.intervals()));
                let class = micromode.set_class();
                ui.horizontal_wrapped(|ui| {
                    ui.label(format!(
                        "Pitch-class set {} {} {}",
                        class.name(),
                        class.prime_text(),
                        class.vector_text()
                    ));
                    if ui
                        .small_button(format!("{} in Sets →", class.name()))
                        .on_hover_text("Show this set class in the pitch-class set browser")
                        .clicked()
                    {
                        actions.push(MicroAction::ShowSet(form));
                    }
                });
                ui.weak(format!(
                    "{} forms: {}. Hour {} of Peter Schat's Tone Clock.",
                    micromode.members().len(),
                    forms_text(micromode),
                    micromode.hour()
                ));
                ui.add_space(4.0);
                ui.label(micromode.about);
            });
        });
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.label("Move it up");
            egui::ComboBox::from_id_salt("micromode-transposition")
                .width(48.0)
                .selected_text(format!("T{}", self.transposition))
                .show_ui(ui, |ui| {
                    for n in 0..12 {
                        ui.selectable_value(&mut self.transposition, n, format!("T{n}"));
                    }
                })
                .response
                .on_hover_text("Transpose: move every note up this many semitones");
            ui.checkbox(&mut self.inverted, "Inverted (I)")
                .on_hover_text("Turn every interval upside down first");
            let label = if self.inverted {
                format!("T{}I", self.transposition)
            } else {
                format!("T{}", self.transposition)
            };
            let names: Vec<String> = notes.iter().map(|n| n.to_string()).collect();
            ui.label(
                RichText::new(format!("{label}: {} ({})", names.join(" "), digits(form))).strong(),
            );
        });
        ui.horizontal_wrapped(|ui| {
            if ui
                .button("▶ Hear it")
                .on_hover_text("Play the three notes together, from middle C")
                .clicked()
            {
                actions.push(MicroAction::Hear(voiced(form, &notes)));
            }
            crate::twelve_tone::draw_set_control(ui, form, drawing);
            if ui
                .button("Use as the key")
                .on_hover_text("The piano roll highlights these three notes and snaps to them")
                .clicked()
                && let Some(root) = notes.first()
            {
                actions.push(MicroAction::UseAsKey(Scale::set(root.pitch_class(), form)));
            }
        });
        ui.separator();
        self.inside(ui, actions);
    }

    /// Well-known larger sets that hold the micromode, and how often.
    fn inside(&self, ui: &mut Ui, actions: &mut Vec<MicroAction>) {
        let index = usize::from(self.micromode.number - 1);
        let found: Vec<(&'static SetClass, u16)> = SetClass::all()
            .iter()
            .filter(|class| class.cardinality >= 4 && class.cardinality < 12)
            .filter(|class| class.nickname().is_some())
            .map(|class| {
                (
                    class,
                    lineup_core::harmony::micromode_content(class.prime)[index],
                )
            })
            .filter(|(_, count)| *count > 0)
            .collect();
        ui.label(RichText::new("Inside larger sets").strong());
        if found.is_empty() {
            ui.weak("None of the sets with a name holds it.");
            return;
        }
        ui.horizontal_wrapped(|ui| {
            for (class, count) in found {
                let nickname = class.nickname().unwrap_or_default();
                let text = format!("{nickname} ×{count}");
                if ui
                    .small_button(text)
                    .on_hover_text(format!(
                        "{}: {} of its groups of three are {}. Click to see it in Sets.",
                        class.describe(),
                        count,
                        self.micromode.name
                    ))
                    .clicked()
                {
                    actions.push(MicroAction::ShowSet(class.prime));
                }
            }
        });
    }
}

/// How the forms of a micromode come about, in words.
fn forms_text(micromode: &Micromode) -> &'static str {
    match micromode.members().len() {
        24 => "on each of the twelve notes, the right way up and upside down",
        12 => "upside down it is the same, so one on each of the twelve notes",
        _ => "moved up a major third it is the same, so there are only four",
    }
}

/// The notes of `form` as keys of the piano, from middle C: the first
/// written note at or above C4, the others stacked above it.
pub(crate) fn voiced(form: PitchSet, notes: &[NoteName]) -> Vec<Note> {
    let mut keys: Vec<Note> = Vec::new();
    for name in notes {
        let pitch_class = name.pitch_class();
        let note = match keys.last() {
            None => Note::new(60 + PitchClass::C.semitones_up_to(pitch_class)),
            Some(&below) => {
                let up = match below.pitch_class().semitones_up_to(pitch_class) {
                    0 => 12,
                    gap => gap,
                };
                below.transpose(i32::from(up)).unwrap_or(below)
            }
        };
        keys.push(note);
    }
    debug_assert_eq!(
        keys.iter().map(|n| n.pitch_class()).collect::<PitchSet>(),
        form
    );
    keys
}

/// The point for pitch class `n` on a clock face: C at the top, going
/// round clockwise a semitone at a time.
fn on_clock(center: Pos2, radius: f32, n: u8) -> Pos2 {
    let angle = f32::from(n) / 12.0 * TAU;
    center + Vec2::new(angle.sin(), -angle.cos()) * radius
}

/// The twelve notes in a circle with the form's notes joined: the shape
/// of a micromode, which stays the same when it is moved (it turns) and
/// is mirrored when it is turned upside down.
fn clock(ui: &mut Ui, form: PitchSet, names: &[NoteName], size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let painter = ui.painter_at(rect);
    let center = rect.center();
    let radius = size * 0.36;
    painter.circle_stroke(center, radius, Stroke::new(1.0, theme::BUTTON));
    let points: Vec<Pos2> = form
        .iter()
        .map(|p| on_clock(center, radius, p.semitone()))
        .collect();
    painter.add(egui::Shape::convex_polygon(
        points.clone(),
        theme::ACCENT.gamma_multiply(0.25),
        Stroke::new(2.0, theme::ACCENT),
    ));
    for n in 0..12u8 {
        let pitch_class = PitchClass::from_semitone(n);
        let inside = form.contains(pitch_class);
        let dot = on_clock(center, radius, n);
        let (fill, r) = if inside {
            (theme::ACCENT, 5.0)
        } else {
            (theme::BUTTON_HOVER, 3.0)
        };
        painter.circle_filled(dot, r, fill);
        let name = names
            .iter()
            .find(|name| name.pitch_class() == pitch_class)
            .copied()
            .unwrap_or(NoteName::chromatic(pitch_class));
        let (color, font) = if inside {
            (theme::TEXT, egui::FontId::proportional(14.0))
        } else {
            (theme::TEXT_DIM, egui::FontId::proportional(11.0))
        };
        painter.text(
            on_clock(center, radius + 16.0, n),
            egui::Align2::CENTER_CENTER,
            name.to_string(),
            font,
            color,
        );
    }
}

/// A small clock face of the prime form, with the micromode's name: one
/// button of the list.
fn chip(ui: &mut Ui, micromode: &Micromode, on: bool) -> egui::Response {
    let text_color = if on { theme::ON_ACCENT } else { theme::TEXT };
    let galley = ui.painter().layout_no_wrap(
        micromode.name.to_string(),
        egui::FontId::proportional(13.0),
        text_color,
    );
    let icon = 22.0;
    let size = Vec2::new(icon + 10.0 + galley.size().x + 8.0, icon + 8.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let fill = if on {
        theme::ACCENT
    } else if response.hovered() {
        theme::BUTTON_HOVER
    } else {
        theme::BUTTON
    };
    let painter = ui.painter();
    painter.rect_filled(rect, 4.0, fill);
    let center = Pos2::new(rect.left() + 4.0 + icon / 2.0, rect.center().y);
    let radius = icon / 2.0 - 2.0;
    let line = if on { theme::ON_ACCENT } else { theme::ACCENT };
    painter.circle_stroke(center, radius, Stroke::new(1.0, line.gamma_multiply(0.5)));
    let points: Vec<Pos2> = micromode
        .prime
        .iter()
        .map(|p| on_clock(center, radius, p.semitone()))
        .collect();
    painter.add(egui::Shape::closed_line(points, Stroke::new(1.5, line)));
    painter.galley(
        Pos2::new(
            rect.left() + icon + 10.0,
            rect.center().y - galley.size().y / 2.0,
        ),
        galley,
        Color32::PLACEHOLDER,
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::{ScaleKind, Spelling};

    fn set(semitones: &[u8]) -> PitchSet {
        PitchSet::from_semitones(semitones)
    }

    fn text(names: Vec<NoteName>) -> String {
        names
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn chord_micromodes_are_spelled_as_chords() {
        let c_major = Scale::new(PitchClass::C, ScaleKind::Major);
        // The diminished triad on C is C Eb Gb, not C Eb F#.
        assert_eq!(text(form_names(set(&[0, 3, 6]), c_major)), "C Eb Gb");
        // The augmented triad on C, and F major written from F.
        assert_eq!(text(form_names(set(&[0, 4, 8]), c_major)), "C E G#");
        assert_eq!(text(form_names(set(&[0, 5, 9]), c_major)), "F A C");
        // Others in their normal form, as the key writes them.
        assert_eq!(text(form_names(set(&[0, 1, 4]), c_major)), "C Db E");
        assert_eq!(
            text(form_names(set(&[0, 1, 4]), Spelling::Sharps)),
            "C C# E"
        );
    }

    #[test]
    fn hearing_a_form_plays_its_three_notes_from_middle_c() {
        let form = set(&[9, 0, 1]);
        let notes = form_names(form, Spelling::Sharps);
        let keys = voiced(form, &notes);
        assert_eq!(
            keys.iter().map(|n| n.midi()).collect::<Vec<_>>(),
            [69, 72, 73]
        );
    }

    #[test]
    fn picking_a_form_shows_how_it_was_moved() {
        let mut window = MicromodeWindow::new();
        // A C C# is Menor 3 upside down, moved up 1: T1I of C C# E.
        window.pick(set(&[9, 0, 1]));
        assert_eq!(window.micromode.name, "Menor 3");
        assert!(window.inverted);
        assert_eq!(window.chosen(), set(&[9, 0, 1]));
    }
}
