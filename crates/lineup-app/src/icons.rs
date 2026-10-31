//! Small pictures drawn with lines and shapes, for the toolbar's toggles.
//!
//! They are painted rather than taken from a font, so they look the same
//! on every system: the emoji the toolbar used before came from the
//! system's fonts, and a system without them showed empty boxes.
//!
//! Each picture is drawn in a square `rect` (16 points in the toolbar)
//! in one colour, so a toggle that is on can draw it dark on its light.

use eframe::egui::{self, Color32, Painter, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, vec2};

use crate::theme;

/// A picture: drawn by a function of the painter, the square it fills
/// and the colour to draw it in.
pub type Picture = fn(&Painter, Rect, Color32);

/// The Id of the toggle named `name`, to find it again (in tests).
pub fn id(name: &str) -> egui::Id {
    egui::Id::unique(("icon-toggle", name))
}

/// A square button showing `picture`, lit as [`theme::light_button`] is
/// when `on`. `name` says what it is: it names the button for screen
/// readers and gives it a lasting Id (see [`id`]).
pub fn toggle(ui: &mut egui::Ui, name: &str, on: bool, picture: Picture) -> egui::Response {
    toggle_lit(ui, name, on, theme::ACCENT, picture)
}

/// A [`toggle`] lit in `light` when `on`, as Rec is lit red.
pub fn toggle_lit(
    ui: &mut egui::Ui,
    name: &str,
    on: bool,
    light: Color32,
    picture: Picture,
) -> egui::Response {
    let (_, rect) = ui.allocate_space(vec2(30.0, 26.0));
    let response = ui.interact(rect, id(name), Sense::click());
    response
        .widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, ui.is_enabled(), on, name));
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let fill = if on { light } else { visuals.weak_bg_fill };
        let painter = ui.painter();
        painter.rect(
            rect,
            visuals.corner_radius,
            fill,
            visuals.bg_stroke,
            StrokeKind::Inside,
        );
        let color = if on {
            theme::ON_ACCENT
        } else if response.hovered() {
            Color32::WHITE
        } else {
            theme::TEXT
        };
        picture(
            painter,
            Rect::from_center_size(rect.center(), vec2(16.0, 16.0)),
            color,
        );
    }
    response
}

/// The point `x`, `y` of the way across and down `rect`, each from 0 to 1.
fn at(rect: Rect, x: f32, y: f32) -> Pos2 {
    rect.min + vec2(x * rect.width(), y * rect.height())
}

fn line(painter: &Painter, rect: Rect, from: (f32, f32), to: (f32, f32), color: Color32) {
    painter.line_segment(
        [at(rect, from.0, from.1), at(rect, to.0, to.1)],
        Stroke::new(1.5, color),
    );
}

fn filled(painter: &Painter, rect: Rect, points: &[(f32, f32)], color: Color32) {
    let points = points.iter().map(|&(x, y)| at(rect, x, y)).collect();
    painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
}

/// The chord sheet: a page of chord names, two to a line.
pub fn chord_sheet(painter: &Painter, rect: Rect, color: Color32) {
    let page = Rect::from_min_max(at(rect, 0.12, 0.0), at(rect, 0.88, 1.0));
    painter.rect_stroke(page, 1.5, Stroke::new(1.5, color), StrokeKind::Inside);
    for y in [0.28, 0.5, 0.72] {
        line(painter, rect, (0.3, y), (0.44, y), color);
        line(painter, rect, (0.56, y), (0.7, y), color);
    }
}

/// The channel rack: rows of steps, some lit.
pub fn rack(painter: &Painter, rect: Rect, color: Color32) {
    const LIT: [[bool; 4]; 3] = [
        [true, false, true, false],
        [false, false, true, false],
        [true, true, false, true],
    ];
    let size = rect.width() / 4.0;
    for (row, steps) in LIT.iter().enumerate() {
        for (column, &lit) in steps.iter().enumerate() {
            let min = rect.min + vec2(column as f32 * size, 1.0 + row as f32 * (size + 1.3));
            let step = Rect::from_min_size(min, vec2(size - 1.3, size - 1.3));
            let fill = if lit {
                color
            } else {
                color.gamma_multiply(0.35)
            };
            painter.rect_filled(step, 0.8, fill);
        }
    }
}

/// The piano roll: a keyboard down the side and notes along it.
pub fn piano_roll(painter: &Painter, rect: Rect, color: Color32) {
    let keys = Rect::from_min_max(at(rect, 0.0, 0.0), at(rect, 0.22, 1.0));
    painter.rect_stroke(keys, 1.0, Stroke::new(1.5, color), StrokeKind::Inside);
    for (y, from, to) in [(0.2, 0.32, 0.62), (0.5, 0.55, 1.0), (0.8, 0.38, 0.78)] {
        let note = Rect::from_min_max(at(rect, from, y - 0.07), at(rect, to, y + 0.07));
        painter.rect_filled(note, 1.0, color);
    }
}

/// The mixer: three faders, each at its own level.
pub fn mixer(painter: &Painter, rect: Rect, color: Color32) {
    for (x, level) in [(0.2, 0.62), (0.5, 0.3), (0.8, 0.5)] {
        line(painter, rect, (x, 0.0), (x, 1.0), color.gamma_multiply(0.6));
        let knob = Rect::from_center_size(at(rect, x, level), vec2(6.0, 3.5));
        painter.rect_filled(knob, 1.0, color);
    }
}

/// The harmony keys: a keyboard.
pub fn harmony_keys(painter: &Painter, rect: Rect, color: Color32) {
    let keys = Rect::from_min_max(at(rect, 0.0, 0.12), at(rect, 1.0, 0.88));
    painter.rect_stroke(keys, 1.5, Stroke::new(1.5, color), StrokeKind::Inside);
    for x in [0.25, 0.5, 0.75] {
        line(painter, rect, (x, 0.5), (x, 0.88), color);
    }
    for x in [0.25, 0.75] {
        let key = Rect::from_min_max(at(rect, x - 0.09, 0.12), at(rect, x + 0.09, 0.56));
        painter.rect_filled(key, 0.5, color);
    }
}

/// Follow playhead: the playhead, moving on.
pub fn follow(painter: &Painter, rect: Rect, color: Color32) {
    line(painter, rect, (0.3, 0.25), (0.3, 1.0), color);
    filled(painter, rect, &[(0.1, 0.0), (0.5, 0.0), (0.3, 0.3)], color);
    line(painter, rect, (0.48, 0.62), (0.95, 0.62), color);
    line(painter, rect, (0.78, 0.45), (0.95, 0.62), color);
    line(painter, rect, (0.78, 0.79), (0.95, 0.62), color);
}

/// Count-in: clicks, then play.
pub fn count_in(painter: &Painter, rect: Rect, color: Color32) {
    for x in [0.05, 0.25, 0.45] {
        line(painter, rect, (x, 0.3), (x, 0.7), color);
    }
    filled(
        painter,
        rect,
        &[(0.62, 0.15), (1.0, 0.5), (0.62, 0.85)],
        color,
    );
}

/// The metronome: its body and its swinging arm.
pub fn metronome(painter: &Painter, rect: Rect, color: Color32) {
    let body = [(0.38, 0.05), (0.62, 0.05), (0.88, 0.95), (0.12, 0.95)];
    let points = body.iter().map(|&(x, y)| at(rect, x, y)).collect();
    painter.add(Shape::closed_line(points, Stroke::new(1.5, color)));
    line(painter, rect, (0.5, 0.8), (0.82, 0.12), color);
    painter.circle_filled(at(rect, 0.66, 0.46), 2.0, color);
}

/// The microphone: its head, its stand and its foot.
pub fn microphone(painter: &Painter, rect: Rect, color: Color32) {
    let head = Rect::from_min_max(at(rect, 0.34, 0.0), at(rect, 0.66, 0.62));
    painter.rect_filled(head, 0.16 * rect.width(), color);
    let cradle = [
        (0.2, 0.4),
        (0.2, 0.55),
        (0.32, 0.74),
        (0.5, 0.78),
        (0.68, 0.74),
        (0.8, 0.55),
        (0.8, 0.4),
    ];
    let points = cradle.iter().map(|&(x, y)| at(rect, x, y)).collect();
    painter.add(Shape::line(points, Stroke::new(1.5, color)));
    line(painter, rect, (0.5, 0.78), (0.5, 0.95), color);
    line(painter, rect, (0.3, 0.95), (0.7, 0.95), color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_picture_stays_in_its_square() {
        let ctx = egui::Context::default();
        let square = Rect::from_min_size(Pos2::new(7.0, 5.0), vec2(16.0, 16.0));
        let pictures: [Picture; 9] = [
            chord_sheet,
            rack,
            piano_roll,
            mixer,
            harmony_keys,
            follow,
            count_in,
            metronome,
            microphone,
        ];
        for picture in pictures {
            let mut output = ctx.run_ui(Default::default(), |ui| {
                picture(ui.painter(), square, Color32::WHITE);
            });
            // The font texture is not drawn anywhere here.
            output.textures_delta.clear();
            let mut shapes = 0;
            for shape in output.shapes {
                let bounds = shape.shape.visual_bounding_rect();
                assert!(square.expand(1.0).contains_rect(bounds), "{bounds:?}");
                shapes += 1;
            }
            assert!(shapes > 0);
        }
    }
}
