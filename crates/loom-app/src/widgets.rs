//! Custom widgets for the channel rack: step buttons, knobs, mute lights
//! and the small piano roll preview.
//!
//! An egui widget is just a function that reserves some space, reads the
//! mouse for that space and paints into it. It returns a [`Response`] so
//! the caller can ask whether it was clicked, dragged or changed.

use eframe::egui::{self, Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use loom_core::NoteEvent;

/// Step colors. Steps are grouped by beats (four steps in 4/4, six in
/// 6/8), and the groups alternate colors so you can count beats at a
/// glance.
const STEP_OFF: [Color32; 2] = [Color32::from_rgb(58, 62, 72), Color32::from_rgb(84, 62, 70)];
const STEP_ON: Color32 = Color32::from_rgb(226, 230, 238);
const PLAYHEAD: Color32 = Color32::from_rgb(255, 152, 48);
const LED_ON: Color32 = Color32::from_rgb(96, 220, 120);
const LED_OFF: Color32 = Color32::from_rgb(70, 74, 80);
const MINI_ROLL_BACKGROUND: Color32 = Color32::from_rgb(44, 52, 48);
const MINI_ROLL_LINE: Color32 = Color32::from_rgb(64, 74, 68);
const MINI_ROLL_NOTE: Color32 = Color32::from_rgb(118, 214, 140);

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

/// A small picture of a channel's notes, drawn in place of its steps when
/// the channel plays melodies or chords. Clicking it opens the piano roll.
pub fn mini_roll(
    ui: &mut Ui,
    notes: &[NoteEvent],
    length: usize,
    playhead: Option<usize>,
) -> Response {
    // Step buttons have a gap between them, so one step takes up the
    // button's width plus the gap. The preview must line up with them.
    let gap = ui.spacing().item_spacing.x;
    let step_width = STEP_SIZE.x + gap;
    let size = vec2(step_width * length as f32 - gap, STEP_SIZE.y);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let painter = ui.painter();
    let inner = rect.shrink(1.0);
    let mut background = MINI_ROLL_BACKGROUND;
    if response.hovered() {
        background = background.gamma_multiply(1.25);
    }
    painter.rect_filled(inner, 3.0, background);
    for beat in (4..length).step_by(4) {
        let x = rect.left() + beat as f32 * step_width - gap / 2.0;
        painter.vline(x, inner.y_range(), Stroke::new(1.0, MINI_ROLL_LINE));
    }

    // Fit the notes' pitch range into the height, at least an octave tall
    // so a few close notes do not fill it from top to bottom.
    let visible = || notes.iter().filter(|note| note.start < length);
    let lowest = visible().map(|note| note.pitch.midi()).min().unwrap_or(0);
    let highest = visible().map(|note| note.pitch.midi()).max().unwrap_or(0);
    let span = (highest - lowest).max(12) as f32;
    let centering = (span - (highest - lowest) as f32) / 2.0;
    for note in visible() {
        let position = (note.pitch.midi() - lowest) as f32 + centering;
        let y = inner.bottom() - 3.0 - position / span * (inner.height() - 6.0);
        let x_start = rect.left() + note.start as f32 * step_width + 1.0;
        let x_end = rect.left() + note.end().min(length) as f32 * step_width - gap - 1.0;
        painter.line_segment(
            [pos2(x_start, y), pos2(x_end, y)],
            Stroke::new(2.0, MINI_ROLL_NOTE),
        );
    }

    if let Some(step) = playhead.filter(|&step| step < length) {
        let x = rect.left() + step as f32 * step_width;
        let column =
            Rect::from_min_max(pos2(x, inner.top()), pos2(x + STEP_SIZE.x, inner.bottom()));
        painter.rect_stroke(column, 2.0, Stroke::new(2.0, PLAYHEAD), StrokeKind::Inside);
    }
    response
}
