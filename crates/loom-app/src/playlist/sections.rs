//! The song's structure, at the top of the playlist: the section strip
//! (Intro, Verse, Chorus...) and the loop region on the bar numbers.
//!
//! Both strips stay at the top while the tracks scroll under them, like
//! the bar numbers always did.
//!
//! - Drag along the empty section strip to mark a new section.
//! - Drag a section to move it, or its right edge to make it longer or
//!   shorter. A plain click (or a right-click) opens its menu: rename it,
//!   choose how many times it plays, loop it, copy it or delete it.
//! - Drag along the bar numbers to loop that part of the song. The
//!   "Looping bars…" button in the toolbar, or a section's menu, stops it.
//!   A plain click on the bar numbers moves the playhead there.
//! - Right-click the bar numbers to change the time signature from that
//!   bar on, or to take a change away.
//!
//! While a section or a loop is being dragged, only an outline of where
//! it will land is drawn. The project changes once, when the button comes
//! up, so a section dragged past another one cannot change places with
//! it halfway through.

use eframe::egui::{
    self, Align2, Color32, CursorIcon, FontId, Painter, PopupCloseBehavior, Rect, Response, Sense,
    Stroke, StrokeKind, Ui, pos2, vec2,
};
use loom_core::{
    KeyMarker, LoopRegion, MAX_REPEATS, Pass, PitchClass, Project, SECTION_NAMES, Scale, ScaleKind,
    Section, TimeMarker,
};

use super::{Layout, PlaylistAction, Snap};
use crate::time_picker::time_picker;

/// How close to a section's right edge the pointer must be to resize it.
const EDGE: f32 = 6.0;

const STRIP: Color32 = Color32::from_rgb(22, 24, 29);
const STRIP_HINT: Color32 = Color32::from_rgb(110, 116, 128);
const SECTION_TEXT: Color32 = Color32::from_rgb(18, 20, 24);
const GHOST: Color32 = Color32::from_rgb(245, 245, 250);
pub(super) const LOOP: Color32 = Color32::from_rgb(92, 196, 120);

/// The colours of sections. A name always gets the same colour, so every
/// chorus looks alike: the names in [`SECTION_NAMES`] each have their
/// own, and other names take one by their length.
const COLORS: [Color32; 9] = [
    Color32::from_rgb(140, 150, 170), // Intro
    Color32::from_rgb(110, 170, 230), // Verse
    Color32::from_rgb(240, 150, 90),  // Chorus
    Color32::from_rgb(190, 140, 220), // Bridge
    Color32::from_rgb(150, 160, 150), // Outro
    Color32::from_rgb(150, 200, 230), // Pre-Chorus
    Color32::from_rgb(230, 200, 100), // Solo
    Color32::from_rgb(120, 200, 170), // Break
    Color32::from_rgb(230, 110, 120), // Drop
];

/// The colour of a section called `name`. "Chorus 2", a copy made by
/// Duplicate, is coloured like "Chorus".
pub(super) fn section_color(name: &str) -> Color32 {
    let base = name.trim_end_matches(|c: char| c.is_ascii_digit() || c == ' ');
    SECTION_NAMES
        .iter()
        .position(|known| known.eq_ignore_ascii_case(base))
        .map_or(COLORS[base.len() % COLORS.len()], |index| COLORS[index])
}

/// Something being dragged in the strips at the top.
#[derive(Debug, Clone)]
enum Drag {
    /// A new section, from the step where the drag began.
    New { from: usize },
    /// Section `index`, held `grab` steps after its start.
    Move { index: usize, grab: usize },
    /// The right edge of section `index`.
    Resize { index: usize },
    /// A loop region, from the step where the drag began.
    Loop { from: usize },
}

/// What the strips remember between frames.
#[derive(Debug, Default)]
pub(super) struct Strips {
    drag: Option<Drag>,
    /// The bar whose time signature menu is open, by its first step.
    time_menu: usize,
}

impl Strips {
    /// The section strip: draws the sections and reads the mouse over it.
    /// `pass` is what the song is playing now, to show "2 of 3" on a
    /// section that repeats.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn sections(
        &mut self,
        ui: &mut Ui,
        layout: Layout,
        strip: Rect,
        project: &Project,
        snap: Snap,
        pass: Option<Pass>,
        actions: &mut Vec<PlaylistAction>,
    ) {
        let painter = ui.painter_at(strip);
        painter.rect_filled(strip, 0.0, STRIP);
        let sections = project.playlist().sections();
        let meter = project.meter();

        // The empty strip, under the sections: dragging on it marks a new one.
        let background = ui.interact(strip, ui.scope_id().with("section-strip"), Sense::drag());
        if sections.is_empty() && self.drag.is_none() {
            painter.text(
                pos2(strip.left() + 8.0, strip.center().y),
                Align2::LEFT_CENTER,
                "Drag here to mark a section: Intro, Verse, Chorus…",
                FontId::proportional(11.0),
                STRIP_HINT,
            );
        }
        if background.drag_started()
            && let Some(pos) = press_origin(ui)
        {
            let from = snap.floor(layout.step_at(pos.x), &meter);
            self.drag = Some(Drag::New { from });
        }
        background.on_hover_text("Drag to mark a section of the song");

        for (index, section) in sections.iter().enumerate() {
            let rect = Rect::from_min_max(
                pos2(layout.x(section.start), strip.top() + 2.0),
                pos2(layout.x(section.end()), strip.bottom() - 2.0),
            );
            if !rect.intersects(strip) {
                continue;
            }
            let response = ui.interact(
                rect,
                ui.scope_id().with(("section", index)),
                Sense::click_and_drag(),
            );
            let near_edge = |x: f32| x > rect.right() - EDGE;
            if response.hover_pos().is_some_and(|pos| near_edge(pos.x)) {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
            }
            if response.drag_started()
                && let Some(pos) = press_origin(ui)
            {
                self.drag = Some(if near_edge(pos.x) {
                    Drag::Resize { index }
                } else {
                    let grab = layout.step_at(pos.x).saturating_sub(section.start);
                    Drag::Move { index, grab }
                });
            }

            let playing = pass.filter(|pass| pass.section == Some(index));
            paint_section(&painter, rect, section, playing);
            let response = response.on_hover_text(format!(
                "{}: click to rename, repeat, loop or copy it. \
                 Drag to move it, drag its right edge to resize it.",
                section.name
            ));
            section_menu(&response, index, section, project, actions);
        }

        // While dragging, an outline shows where the section will land,
        // and letting go puts it there.
        let pointer = ui.input(|input| input.pointer.interact_pos());
        let released = ui.input(|input| input.pointer.any_released());
        if let (Some(drag), Some(pos)) = (&self.drag, pointer) {
            let step = layout.step_at(pos.x);
            let planned = match *drag {
                Drag::New { from } => {
                    let to = snap.next(step, &meter);
                    let (start, end) = if to > from {
                        (from, to)
                    } else {
                        (snap.floor(step, &meter), snap.next(from, &meter))
                    };
                    let name = project.playlist().suggested_section_name();
                    Some((None, Section::new(name, start, end - start)))
                }
                Drag::Move { index, grab } => sections.get(index).map(|section| {
                    let start = snap.floor(step.saturating_sub(grab), &meter);
                    (
                        Some(index),
                        Section {
                            start,
                            ..section.clone()
                        },
                    )
                }),
                Drag::Resize { index } => sections.get(index).map(|section| {
                    let end = snap
                        .next(step, &meter)
                        .max(snap.next(section.start, &meter));
                    let length = end - section.start;
                    (
                        Some(index),
                        Section {
                            length,
                            ..section.clone()
                        },
                    )
                }),
                Drag::Loop { .. } => None,
            };
            if let Some((index, section)) = planned {
                let ghost = Rect::from_min_max(
                    pos2(layout.x(section.start), strip.top() + 1.0),
                    pos2(layout.x(section.end()), strip.bottom() - 1.0),
                );
                painter.rect_stroke(ghost, 3.0, Stroke::new(1.5, GHOST), StrokeKind::Inside);
                if released {
                    actions.push(match index {
                        Some(index) => PlaylistAction::SetSection { index, section },
                        None => PlaylistAction::AddSection(section),
                    });
                }
            }
        }
        if released && !matches!(self.drag, Some(Drag::Loop { .. })) {
            self.drag = None;
        }
    }

    /// The bar ruler: click to move the playhead there, drag to set the
    /// loop region. `ruler` is the strip with the bar numbers.
    pub(super) fn loop_ruler(
        &mut self,
        ui: &mut Ui,
        layout: Layout,
        ruler: Rect,
        project: &Project,
        snap: Snap,
        actions: &mut Vec<PlaylistAction>,
    ) {
        let meter = project.meter();
        let response = ui
            .interact(
                ruler,
                ui.scope_id().with("loop-ruler"),
                Sense::click_and_drag(),
            )
            .on_hover_text(
                "Click the bar numbers to move the playhead there. Drag along them to loop \
                 that part of the song. Right-click to change the time signature or the key \
                 from that bar on",
            );
        if response.secondary_clicked()
            && let Some(pos) = response.interact_pointer_pos()
        {
            self.time_menu = meter.floor_to_bar(layout.step_at(pos.x));
        }
        let at = self.time_menu;
        // The menu holds fields to type a time signature in, so it must
        // stay open while typing: only a click outside closes it.
        egui::Popup::context_menu(&response)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| time_menu(ui, project, at, actions));
        let region = project.playlist().loop_region();
        let painter = ui.painter_at(ruler);

        if response.clicked()
            && let Some(pos) = response.interact_pointer_pos()
        {
            actions.push(PlaylistAction::Seek(
                snap.floor(layout.step_at(pos.x), &meter),
            ));
        }

        if response.drag_started()
            && let Some(pos) = press_origin(ui)
        {
            self.drag = Some(Drag::Loop {
                from: snap.floor(layout.step_at(pos.x), &meter),
            });
        }

        let pointer = ui.input(|input| input.pointer.interact_pos());
        let released = ui.input(|input| input.pointer.any_released());
        if let (Some(Drag::Loop { from }), Some(pos)) = (&self.drag, pointer) {
            let step = layout.step_at(pos.x);
            let to = snap.next(step, &meter);
            let planned = if to > *from {
                LoopRegion::new(*from, to)
            } else {
                LoopRegion::new(snap.floor(step, &meter), snap.next(*from, &meter))
            };
            paint_loop(&painter, layout, ruler, planned, true);
            if released {
                actions.push(PlaylistAction::SetLoop(Some(planned)));
                self.drag = None;
            }
        } else if let Some(region) = region {
            paint_loop(&painter, layout, ruler, region, false);
        }
    }
}

/// The menu on the bar numbers: the time signature from the bar that
/// starts on step `at` on.
fn time_menu(ui: &mut Ui, project: &Project, at: usize, actions: &mut Vec<PlaylistAction>) {
    let meter = project.meter();
    let bar = meter.bar_at(at);
    let marker = project
        .playlist()
        .time_markers()
        .iter()
        .find(|marker| marker.start == at);
    ui.label(format!("Time signature from bar {}", bar.number + 1));
    if let Some(time) = time_picker(ui, egui::Id::unique(("time-from", at)), bar.time) {
        actions.push(PlaylistAction::SetTimeMarker(TimeMarker::new(at, time)));
    }
    if at == 0 {
        ui.weak("Bar 1 sets the time signature the song starts in.");
    }
    if marker.is_some() {
        ui.separator();
        let back = meter.time_at(at.saturating_sub(1));
        if ui
            .button(format!("Remove this change (back to {back})"))
            .clicked()
        {
            actions.push(PlaylistAction::RemoveTimeMarker(at));
            ui.close();
        }
    }
    ui.separator();
    key_menu(ui, project, at, bar.number, actions);
}

/// The key half of the bar numbers' menu: a key change from the bar
/// that starts on step `at` (number `bar`, counted from 0) on. It only
/// changes what the key is called and shown as, never the notes, as a
/// new key signature in a score does.
fn key_menu(
    ui: &mut Ui,
    project: &Project,
    at: usize,
    bar: usize,
    actions: &mut Vec<PlaylistAction>,
) {
    let keys = project.key_map();
    let here = keys.key_at(at);
    ui.label(format!("Key from bar {}: {}", bar + 1, here.name()));
    if at == 0 {
        ui.weak(
            "Bar 1 is the key the song starts in: change it in the harmony keys or the \
             piano roll, which also move the notes.",
        );
        return;
    }
    let marker = keys.markers().iter().find(|marker| marker.start == at);
    // Twelve-tone rows and sets have no place in a key signature.
    let kind = if here.kind.is_tonal() {
        here.kind
    } else {
        ScaleKind::Major
    };
    ui.horizontal_wrapped(|ui| {
        ui.set_max_width(260.0);
        for root in PitchClass::ALL {
            let key = Scale::new(root, kind);
            if ui
                .selectable_label(here == key, key.spell(root).to_string())
                .clicked()
            {
                actions.push(PlaylistAction::SetKeyMarker(KeyMarker::new(at, key)));
                ui.close();
            }
        }
    });
    ui.horizontal_wrapped(|ui| {
        ui.set_max_width(260.0);
        for kind in ScaleKind::ALL {
            let key = Scale::new(here.root, kind);
            if ui.selectable_label(here == key, kind.name()).clicked() {
                actions.push(PlaylistAction::SetKeyMarker(KeyMarker::new(at, key)));
                ui.close();
            }
        }
    });
    ui.weak("The notes stay as they are: this names the key they are in from here.");
    if marker.is_some() {
        let back = keys.key_at(at - 1);
        if ui
            .button(format!("Remove this key change (back to {})", back.name()))
            .clicked()
        {
            actions.push(PlaylistAction::RemoveKeyMarker(at));
            ui.close();
        }
    }
}

/// How long a section is: in bars if it starts and ends on bar lines,
/// counted in whatever time signatures it crosses, or else in steps.
fn describe_length(project: &Project, section: &Section) -> String {
    let meter = project.meter();
    if !meter.is_bar_start(section.start) || !meter.is_bar_start(section.end()) {
        return format!("{} steps", section.length);
    }
    match meter.bar_at(section.end()).number - meter.bar_at(section.start).number {
        1 => "1 bar".to_string(),
        bars => format!("{bars} bars"),
    }
}

/// Where the button that is down went down.
fn press_origin(ui: &Ui) -> Option<egui::Pos2> {
    ui.input(|input| input.pointer.press_origin())
}

/// The loop region on the ruler: a green bar along its bottom with a
/// bracket at each end. `planned` draws the outline of one being dragged.
fn paint_loop(painter: &Painter, layout: Layout, ruler: Rect, region: LoopRegion, planned: bool) {
    let rect = Rect::from_min_max(
        pos2(layout.x(region.start), ruler.top() + 1.0),
        pos2(layout.x(region.end), ruler.bottom() - 1.0),
    );
    if planned {
        painter.rect_stroke(rect, 2.0, Stroke::new(1.5, LOOP), StrokeKind::Inside);
    } else {
        painter.rect_filled(rect, 2.0, LOOP.gamma_multiply(0.22));
        let bar = Rect::from_min_max(pos2(rect.left(), rect.bottom() - 3.0), rect.max);
        painter.rect_filled(bar, 0.0, LOOP);
    }
    for x in [rect.left(), rect.right()] {
        painter.vline(x, rect.y_range(), Stroke::new(2.0, LOOP));
    }
}

/// One section: a coloured band with its name, "×2" if it repeats, and
/// which time round it is while the song plays it.
fn paint_section(painter: &Painter, rect: Rect, section: &Section, playing: Option<Pass>) {
    let color = section_color(&section.name);
    let fill = if playing.is_some() {
        color
    } else {
        color.gamma_multiply(0.8)
    };
    painter.rect_filled(rect, 3.0, fill);
    painter.vline(rect.left(), rect.y_range(), Stroke::new(2.0, color));

    let mut label = section.name.clone();
    if section.repeats > 1 {
        label.push_str(&format!("  ×{}", section.repeats));
    }
    if let Some(pass) = playing
        && pass.rounds > 1
    {
        label.push_str(&format!("   ▶ {} of {}", pass.round, pass.rounds));
    }
    painter.with_clip_rect(rect.shrink2(vec2(2.0, 0.0))).text(
        rect.left_center() + vec2(5.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        SECTION_TEXT,
    );
}

/// The menu of a section, opened by clicking or right-clicking it.
fn section_menu(
    response: &Response,
    index: usize,
    section: &Section,
    project: &Project,
    actions: &mut Vec<PlaylistAction>,
) {
    let toggle = response.clicked() || response.secondary_clicked();
    let set = |section: Section| PlaylistAction::SetSection { index, section };
    // The menu holds a text field, so it must stay open while typing:
    // only a click outside closes it.
    egui::Popup::menu(response)
        .open_memory(toggle.then_some(egui::SetOpenCommand::Toggle))
        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.set_min_width(220.0);
            let mut name = section.name.clone();
            ui.horizontal(|ui| {
                ui.label("Name");
                if ui.text_edit_singleline(&mut name).changed() {
                    actions.push(set(Section {
                        name,
                        ..section.clone()
                    }));
                }
            });
            ui.horizontal_wrapped(|ui| {
                for preset in SECTION_NAMES {
                    let chip = egui::Button::new(egui::RichText::new(preset).small())
                        .fill(section_color(preset).gamma_multiply(0.5));
                    if ui.add(chip).clicked() {
                        actions.push(set(Section {
                            name: preset.to_string(),
                            ..section.clone()
                        }));
                    }
                }
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Play it");
                let mut repeats = section.repeats;
                if ui
                    .add_enabled(repeats > 1, egui::Button::new(" − "))
                    .clicked()
                {
                    repeats -= 1;
                }
                ui.add(
                    egui::DragValue::new(&mut repeats)
                        .range(1..=MAX_REPEATS)
                        .suffix(" ×"),
                );
                if ui
                    .add_enabled(repeats < MAX_REPEATS, egui::Button::new(" + "))
                    .clicked()
                {
                    repeats += 1;
                }
                if repeats != section.repeats {
                    actions.push(set(Section {
                        repeats,
                        ..section.clone()
                    }));
                }
                ui.weak(format!("({} each time)", describe_length(project, section)));
            })
            .response
            .on_hover_text(
                "How many times the song plays this section before it moves on. \
                 It is drawn once, and played that many times.",
            );
            ui.separator();

            if ui.button("🔁 Loop this section").clicked() {
                let region = LoopRegion::new(section.start, section.end());
                actions.push(PlaylistAction::SetLoop(Some(region)));
                ui.close();
            }
            if ui
                .button("⧉ Duplicate")
                .on_hover_text("Insert a copy of the section and its clips right after it")
                .clicked()
            {
                actions.push(PlaylistAction::DuplicateSection(index));
                ui.close();
            }
            // A clip that starts before the section or ends after it
            // cannot be cut at the section's edges, so the copies would
            // not sound like the repeats.
            let cut = project.section_cuts_a_clip(index);
            let unroll = ui
                .add_enabled(
                    section.repeats > 1 && !cut,
                    egui::Button::new("⇶ Write out the repeats"),
                )
                .on_hover_text(
                    "Turn \"×3\" into three copies one after the other, \
                     so each time round can be changed on its own",
                )
                .on_disabled_hover_text(if cut {
                    "A clip starts before this section or ends after it, so its repeats \
                     can't be written out as copies. Move or shorten the clip to fit the \
                     section first."
                } else {
                    "The section plays only once: there are no repeats to write out."
                });
            if unroll.clicked() {
                actions.push(PlaylistAction::UnrollSection(index));
                ui.close();
            }
            if ui
                .button("🗑 Delete section")
                .on_hover_text("Remove the marker. The clips under it stay.")
                .clicked()
            {
                actions.push(PlaylistAction::RemoveSection(index));
                ui.close();
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_are_coloured_like_the_original() {
        assert_eq!(section_color("Chorus 2"), section_color("Chorus"));
        assert_eq!(section_color("chorus"), section_color("Chorus"));
        assert_ne!(section_color("Verse"), section_color("Chorus"));
    }
}
