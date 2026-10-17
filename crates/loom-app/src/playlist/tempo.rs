//! The tempo strip, between the sections and the bar numbers: a line
//! that shows how fast the song plays, bar by bar.
//!
//! - The line is flat while the tempo stays the same, jumps where it
//!   changes at once, and slopes where it changes gradually: down for a
//!   ritardando (slowing down), up for an accelerando (speeding up).
//! - Each change is labelled with its tempo: "♩ = 90" for a sudden one,
//!   "rit. → 60" or "accel. → 140" for a gradual one.
//! - Click or right-click the strip to change the tempo from that point
//!   on, or to take a change away.

use eframe::egui::{
    self, Align2, Color32, FontId, Painter, PopupCloseBehavior, Rect, Sense, Shape, Stroke, Ui,
    pos2,
};
use loom_core::{Meter, Project, Tempo, TempoMap, TempoMarker};

use super::{Layout, PlaylistAction, Snap};

const STRIP: Color32 = Color32::from_rgb(26, 30, 36);
const LINE: Color32 = Color32::from_rgb(110, 186, 236);
const TEXT: Color32 = Color32::from_rgb(170, 214, 244);
/// The height of the labels along the top of the strip.
const LABEL: f32 = 12.0;

/// How many bars a gradual change can take, in the menu.
const GLIDES: [(&str, usize, usize); 6] = [
    // (label, bars, of these many: ½ bar is 1 of 2)
    ("1 beat", 0, 1),
    ("½ bar", 1, 2),
    ("1 bar", 1, 1),
    ("2 bars", 2, 1),
    ("4 bars", 4, 1),
    ("8 bars", 8, 1),
];

/// What the tempo strip remembers between frames.
#[derive(Debug, Default)]
pub(super) struct TempoStrip {
    /// The step whose tempo menu is open.
    menu_at: usize,
}

impl TempoStrip {
    /// Draws the strip and reads the mouse over it. `steps` is how far
    /// the playlist reaches.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn show(
        &mut self,
        ui: &mut Ui,
        layout: Layout,
        strip: Rect,
        project: &Project,
        snap: Snap,
        steps: usize,
        actions: &mut Vec<PlaylistAction>,
    ) {
        let painter = ui.painter_at(strip);
        painter.rect_filled(strip, 0.0, STRIP);
        let map = project.tempo_map();
        let meter = project.meter();
        paint_line(&painter, layout, strip, &map, steps);

        let response = ui
            .interact(strip, ui.scope_id().with("tempo-strip"), Sense::click())
            .on_hover_text(
                "The tempo. Click to change it from here on: at once, or gradually, \
                 to slow down (a ritardando) or speed up (an accelerando)",
            );
        if (response.clicked() || response.secondary_clicked())
            && let Some(pos) = response.interact_pointer_pos()
        {
            self.menu_at = snap.floor(layout.step_at(pos.x), &meter);
        }
        let open = response.clicked() || response.secondary_clicked();
        let at = self.menu_at;
        egui::Popup::menu(&response)
            .open_memory(open.then_some(egui::SetOpenCommand::Bool(true)))
            .at_pointer_fixed()
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| tempo_menu(ui, project, &map, &meter, at, actions));
    }
}

/// The tempo's line, and a label at the start and at every change.
fn paint_line(painter: &Painter, layout: Layout, strip: Rect, map: &TempoMap, steps: usize) {
    let (low, high) = map.range(steps);
    // A little room above and below, so the line never touches the
    // edges, and a steady tempo sits in the middle.
    let (low, high) = (low - 4.0, high + 4.0);
    let y = |tempo: Tempo| {
        let t = (tempo.bpm() - low) / (high - low);
        // The labels go along the top; the line keeps below them.
        strip.bottom() - 3.0 - t * (strip.height() - LABEL - 4.0)
    };

    // The steps where the line bends: the start, every change, and the
    // end of every glide.
    let mut bends = vec![0, steps];
    for marker in map.markers() {
        bends.push(marker.start);
        bends.push(marker.start + marker.glide);
    }
    bends.retain(|&step| step <= steps);
    bends.sort_unstable();
    bends.dedup();
    let mut points = Vec::new();
    for pair in bends.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        points.push(pos2(layout.x(from), y(map.tempo_at(from))));
        points.push(pos2(layout.x(to), y(map.tempo_at(to.saturating_sub(1)))));
    }
    painter.add(Shape::line(points, Stroke::new(1.5, LINE)));

    let label = |step: usize, text: String| {
        painter.text(
            pos2(layout.x(step) + 3.0, strip.top() + 1.0),
            Align2::LEFT_TOP,
            text,
            FontId::proportional(10.0),
            TEXT,
        );
    };
    label(0, format!("♩ = {}", bpm_text(map.first())));
    for marker in map.markers().iter().filter(|m| m.start < steps) {
        painter.vline(
            layout.x(marker.start),
            strip.y_range(),
            Stroke::new(1.0, LINE.gamma_multiply(0.6)),
        );
        label(marker.start, describe(map, marker));
    }
}

/// How a change is written above the line: "♩ = 90" for a sudden one,
/// "rit. → 60" for a gradual slowing down, "accel. → 140" for a gradual
/// speeding up.
fn describe(map: &TempoMap, marker: &TempoMarker) -> String {
    let to = bpm_text(marker.tempo);
    let before = map.tempo_before(marker.start);
    if marker.glide == 0 || marker.tempo == before {
        format!("♩ = {to}")
    } else if marker.tempo < before {
        format!("rit. → {to}")
    } else {
        format!("accel. → {to}")
    }
}

/// A tempo without ".0" when it is a whole number.
fn bpm_text(tempo: Tempo) -> String {
    let bpm = tempo.bpm();
    if bpm.fract() == 0.0 {
        format!("{bpm:.0}")
    } else {
        format!("{bpm:.1}")
    }
}

/// The menu on the tempo strip: the tempo from step `at` on, and how it
/// gets there.
fn tempo_menu(
    ui: &mut Ui,
    project: &Project,
    map: &TempoMap,
    meter: &Meter,
    at: usize,
    actions: &mut Vec<PlaylistAction>,
) {
    ui.set_min_width(250.0);
    let bar = meter.bar_at(at);
    let beat = bar.beat_at(at).index + 1;
    let marker = map
        .markers()
        .iter()
        .find(|marker| marker.start == at)
        .copied();
    let current = match (at, marker) {
        (0, _) => TempoMarker::new(0, project.tempo.bpm()),
        (_, Some(marker)) => marker,
        (_, None) => TempoMarker::new(at, map.tempo_at(at).bpm()),
    };
    if beat == 1 {
        ui.label(format!("Tempo from bar {}", bar.number + 1));
    } else {
        ui.label(format!("Tempo from bar {}, beat {beat}", bar.number + 1));
    }

    let mut bpm = current.tempo.bpm();
    ui.horizontal(|ui| {
        ui.label("♩ =");
        let value = ui.add(
            egui::DragValue::new(&mut bpm)
                .range(Tempo::MIN..=Tempo::MAX)
                .speed(0.5)
                .fixed_decimals(1)
                .suffix(" BPM"),
        );
        if value.changed() {
            actions.push(PlaylistAction::SetTempoMarker(TempoMarker {
                tempo: Tempo::new(bpm),
                ..current
            }));
        }
    });
    if at == 0 {
        ui.weak("Bar 1 sets the tempo the song starts at.");
        return;
    }

    if marker.is_none() {
        ui.weak("No change here yet: a new tempo adds one.");
    }
    ui.separator();
    ui.label("Get there");
    ui.horizontal_wrapped(|ui| {
        ui.set_max_width(250.0);
        if ui
            .selectable_label(marker.is_some() && current.glide == 0, "At once")
            .on_hover_text("A sudden change, as after a pause or a new section")
            .clicked()
        {
            actions.push(PlaylistAction::SetTempoMarker(TempoMarker {
                glide: 0,
                ..current
            }));
        }
        for (label, bars, of) in GLIDES {
            let glide = if bars == 0 {
                bar.beat_at(at).length
            } else {
                bars * bar.time.steps_per_bar() / of
            };
            if ui
                .selectable_label(
                    marker.is_some() && current.glide == glide,
                    format!("over {label}"),
                )
                .clicked()
            {
                actions.push(PlaylistAction::SetTempoMarker(TempoMarker {
                    glide,
                    ..current
                }));
            }
        }
    });
    let before = map.tempo_before(at);
    if current.glide > 0 && current.tempo != before {
        let what = if current.tempo < before {
            "Slowing down bit by bit is a ritardando (rit.)."
        } else {
            "Speeding up bit by bit is an accelerando (accel.)."
        };
        ui.weak(format!(
            "From {} to {} BPM. {what}",
            bpm_text(before),
            bpm_text(current.tempo)
        ));
    }

    if marker.is_some() {
        ui.separator();
        if ui
            .button(format!(
                "Remove this change (back to {} BPM)",
                bpm_text(before)
            ))
            .clicked()
        {
            actions.push(PlaylistAction::RemoveTempoMarker(at));
            ui.close();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changes_are_named_as_a_score_names_them() {
        let map = TempoMap::new(
            Tempo::new(120.0),
            [
                TempoMarker::new(16, 90.0),
                TempoMarker::gliding(32, 60.0, 16),
                TempoMarker::gliding(64, 140.0, 16),
            ],
        );
        let names: Vec<String> = map.markers().iter().map(|m| describe(&map, m)).collect();
        assert_eq!(names, ["♩ = 90", "rit. → 60", "accel. → 140"]);
        assert_eq!(bpm_text(Tempo::new(92.5)), "92.5");
    }
}
