//! Custom widgets for the channel rack: step buttons, knobs and mute lights.
//!
//! An egui widget is just a function that reserves some space, reads the
//! mouse for that space and paints into it. It returns a [`Response`] so
//! the caller can ask whether it was clicked, dragged or changed.

use eframe::egui::{self, Color32, Response, Sense, Stroke, StrokeKind, Ui, vec2};

/// Step colors. Steps are grouped in fours (one beat each), and the
/// groups alternate colors so you can count beats at a glance.
const STEP_OFF: [Color32; 2] = [Color32::from_rgb(58, 62, 72), Color32::from_rgb(84, 62, 70)];
const STEP_ON: Color32 = Color32::from_rgb(226, 230, 238);
const PLAYHEAD: Color32 = Color32::from_rgb(255, 152, 48);
const LED_ON: Color32 = Color32::from_rgb(96, 220, 120);
const LED_OFF: Color32 = Color32::from_rgb(70, 74, 80);

pub const STEP_SIZE: egui::Vec2 = vec2(20.0, 28.0);

/// One step of the sequencer. `beat_group` is 0 or 1 and picks the
/// background color; `is_playhead` outlines the step that is playing.
pub fn step_button(ui: &mut Ui, on: bool, beat_group: usize, is_playhead: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(STEP_SIZE, Sense::click());

    if ui.is_rect_visible(rect) {
        let mut fill = if on {
            STEP_ON
        } else {
            STEP_OFF[beat_group % 2]
        };
        if response.hovered() {
            fill = fill.gamma_multiply(1.25);
        }
        let painter = ui.painter();
        painter.rect_filled(rect.shrink(1.0), 3.0, fill);
        if is_playhead {
            painter.rect_stroke(
                rect.shrink(1.0),
                3.0,
                Stroke::new(2.0, PLAYHEAD),
                StrokeKind::Inside,
            );
        }
    }
    response
}

/// A round knob for a value from 0.0 to 1.0. Drag up or down to turn it;
/// double-click to reset it to `default`.
pub fn knob(ui: &mut Ui, value: &mut f32, default: f32) -> Response {
    let (rect, mut response) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::click_and_drag());

    if response.dragged() {
        let delta = -response.drag_delta().y * 0.005;
        *value = (*value + delta).clamp(0.0, 1.0);
        response.mark_changed();
    }
    if response.double_clicked() {
        *value = default;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let center = rect.center();
        let radius = 9.0;
        let visuals = ui.style().interact(&response);
        painter.circle_filled(center, radius, visuals.bg_fill);
        painter.circle_stroke(center, radius, visuals.bg_stroke);

        // The pointer sweeps 270 degrees, from bottom-left (0.0) to bottom-right (1.0).
        let angle = (-135.0 + 270.0 * *value).to_radians();
        let direction = vec2(angle.sin(), -angle.cos());
        painter.line_segment(
            [center, center + direction * (radius - 1.5)],
            Stroke::new(2.0, visuals.fg_stroke.color),
        );
    }
    response
}

/// A small light that is green while the channel is enabled.
pub fn mute_light(ui: &mut Ui, enabled: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(vec2(14.0, 24.0), Sense::click());
    if ui.is_rect_visible(rect) {
        let color = if enabled { LED_ON } else { LED_OFF };
        ui.painter().circle_filled(rect.center(), 4.5, color);
    }
    response
}
