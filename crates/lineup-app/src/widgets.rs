//! Custom widgets for the channel rack: step buttons, knobs, mute lights
//! and the small piano roll preview.
//!
//! An egui widget is just a function that reserves some space, reads the
//! mouse for that space and paints into it. It returns a [`Response`] so
//! the caller can ask whether it was clicked, dragged or changed.

use eframe::egui::{self, Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use lineup_core::NoteEvent;

pub fn scroll_menu<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let height = (ui.ctx().content_rect().height() * 0.6).max(80.0);
    egui::ScrollArea::vertical()
        .max_height(height)
        .show(ui, add)
        .inner
}

/// Editor shortcuts belong to the last clicked window, including its title
/// and toolbar. Popup controls and text fields keep their own keyboard input.
pub fn editor_keyboard(ctx: &egui::Context, layer: egui::LayerId) -> bool {
    editor_active(ctx, layer)
        && !ctx.text_edit_focused()
        && !egui::Popup::is_any_open(ctx)
        && ctx.memory(|memory| memory.is_above_modal_layer(layer))
}

pub fn modal_open(ctx: &egui::Context) -> bool {
    ctx.memory(|memory| memory.top_modal_layer().is_some())
}

/// Who has the editor keyboard: the layer last pressed, and where it was
/// pressed. The playlist shares the background layer with the toolbars and
/// the side panel, so for it the place matters too.
#[derive(Clone, Copy)]
struct KeyboardOwner {
    layer: egui::LayerId,
    /// `None` when the app gave the keyboard to a window it opened.
    pos: Option<egui::Pos2>,
}

/// The layer last clicked, or given the keyboard by the app, while it is
/// still shown: a window that was closed or hidden gives its keys up.
fn clicked_owner(ctx: &egui::Context) -> Option<KeyboardOwner> {
    let id = egui::Id::unique("keyboard-editor");
    if let Some(pos) = ctx.input(|input| press_pos(input, None))
        && let Some(clicked) = ctx.layer_id_at(pos)
        && clicked.order <= egui::Order::Middle
    {
        let owner = KeyboardOwner {
            layer: clicked,
            pos: Some(pos),
        };
        ctx.data_mut(|data| data.insert_temp(id, owner));
    }
    let owner = ctx.data(|data| data.get_temp::<KeyboardOwner>(id))?;
    shown(ctx, owner.layer).then_some(owner)
}

fn shown(ctx: &egui::Context, layer: egui::LayerId) -> bool {
    layer.order != egui::Order::Middle || ctx.memory(|memory| memory.areas().is_visible(&layer))
}

/// The clicked layer, or else the frontmost window on screen.
fn keyboard_owner(ctx: &egui::Context) -> Option<KeyboardOwner> {
    clicked_owner(ctx).or_else(|| {
        let front = ctx.memory(|memory| {
            memory
                .layer_ids()
                .filter(|layer| layer.order == egui::Order::Middle)
                .filter(|layer| memory.areas().parent_layer(*layer).is_none())
                .filter(|layer| memory.areas().is_visible(layer))
                .last()
        })?;
        Some(KeyboardOwner {
            layer: front,
            pos: None,
        })
    })
}

pub fn editor_active(ctx: &egui::Context, layer: egui::LayerId) -> bool {
    keyboard_owner(ctx).is_some_and(|owner| owner.layer == layer)
}

/// [`editor_keyboard`] for a view drawn on the background layer, such as
/// the playlist: a click on a toolbar or a panel beside it lands on the
/// same layer, so only a click inside `area` gives it the keyboard.
pub fn editor_keyboard_in(ctx: &egui::Context, layer: egui::LayerId, area: Rect) -> bool {
    editor_keyboard(ctx, layer)
        && keyboard_owner(ctx)
            .and_then(|owner| owner.pos)
            .is_none_or(|pos| area.contains(pos))
}

/// Gives the keyboard to a window the app opened or brought to the front,
/// as a click on it would.
pub fn give_keyboard(ctx: &egui::Context, layer: egui::LayerId) {
    let owner = KeyboardOwner { layer, pos: None };
    ctx.data_mut(|data| data.insert_temp(egui::Id::unique("keyboard-editor"), owner));
}

/// The window explicitly clicked, without assigning a floating window
/// keyboard control merely because it opened over the song at startup.
pub fn clicked_editor(ctx: &egui::Context) -> Option<egui::LayerId> {
    clicked_owner(ctx).map(|owner| owner.layer)
}

pub fn follow_playback(ctx: &egui::Context) -> bool {
    ctx.data_mut(|data| {
        data.get_persisted::<bool>(egui::Id::unique("follow-playback"))
            .unwrap_or(true)
    })
}

pub fn follow_control(ui: &mut Ui) {
    let mut follow = follow_playback(ui.ctx());
    if ui
        .checkbox(&mut follow, "Follow playhead")
        .on_hover_text(
            "Keep the playing bar or step visible in the playlist, chord sheet and piano roll",
        )
        .changed()
    {
        ui.ctx()
            .data_mut(|data| data.insert_persisted(egui::Id::unique("follow-playback"), follow));
    }
}

/// A persistent transport toggle, sharing its value with the preferences.
pub fn follow_button(ui: &mut Ui) {
    let follow = follow_playback(ui.ctx());
    let button = crate::theme::light_button(ui, "Follow playhead", follow, None);
    if crate::theme::hint(
        button,
        "Keep the playing bar or step visible in the playlist, chord sheet and piano roll",
    )
    .clicked()
    {
        ui.ctx().data_mut(|data| {
            data.insert_persisted(egui::Id::unique("follow-playback"), !follow);
        });
    }
}

/// Reveal the playhead horizontally, leaving the user's vertical position
/// intact. Only a new step can scroll, so manual scrolling still works.
pub fn follow_x(ui: &Ui, step: Option<usize>, x: f32, view: Rect) {
    let id = ui.scope_id().with("followed-step");
    let step = step.filter(|_| follow_playback(ui.ctx()));
    let previous = ui.ctx().data_mut(|data| {
        let previous = data.get_temp::<Option<usize>>(id).flatten();
        data.insert_temp(id, step);
        previous
    });
    if step.is_some() && step != previous {
        let margin = (view.width() * 0.1).min(60.0);
        ui.scroll_to_rect(
            Rect::from_center_size(pos2(x, view.center().y), vec2(margin * 2.0, 1.0)),
            None,
        );
    }
}

/// Step colors. Steps are grouped by beats (four steps in 4/4, six in
/// 6/8), and the groups alternate colors so you can count beats at a
/// glance.
const STEP_OFF: [Color32; 2] = [Color32::from_rgb(70, 78, 84), Color32::from_rgb(98, 66, 70)];
const STEP_ON: Color32 = Color32::from_rgb(222, 228, 232);
/// The dark edge along the bottom of a step, so it looks like a raised pad.
const STEP_EDGE: Color32 = Color32::from_rgb(26, 30, 33);
const PLAYHEAD: Color32 = Color32::from_rgb(246, 150, 58);
pub const LED_ON: Color32 = Color32::from_rgb(96, 220, 120);
const LED_OFF: Color32 = Color32::from_rgb(70, 74, 80);
const MINI_ROLL_BACKGROUND: Color32 = Color32::from_rgb(40, 54, 48);
const MINI_ROLL_LINE: Color32 = Color32::from_rgb(64, 74, 68);
const MINI_ROLL_NOTE: Color32 = Color32::from_rgb(118, 214, 140);

pub const STEP_SIZE: egui::Vec2 = vec2(20.0, 28.0);

/// The size of a step button in `ui`, which grows with its window's zoom.
pub fn step_size(ui: &Ui) -> egui::Vec2 {
    STEP_SIZE * zoom(ui)
}

/// One step of the sequencer. `beat_group` is 0 or 1 and picks the
/// background color; `is_playhead` outlines the step that is playing.
pub fn step_button(ui: &mut Ui, on: bool, beat_group: usize, is_playhead: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(step_size(ui), Sense::click());

    if ui.is_rect_visible(rect) {
        let mut fill = if on {
            STEP_ON
        } else {
            STEP_OFF[beat_group % 2]
        };
        if response.hovered() {
            fill = fill.gamma_multiply(1.25);
        }
        if is_playhead {
            // The whole column that is sounding lights up, down every row.
            fill = fill.lerp_to_gamma(PLAYHEAD, if on { 0.25 } else { 0.35 });
        }
        let painter = ui.painter();
        let pad = rect.shrink(1.0);
        painter.rect_filled(pad, 2.0, STEP_EDGE);
        painter.rect_filled(
            Rect::from_min_max(pad.min, pad.max - vec2(0.0, 2.0)),
            2.0,
            fill,
        );
        if on {
            // A light strip along the top of a step that is on.
            let shine = Rect::from_min_size(pad.min + vec2(2.0, 2.0), vec2(pad.width() - 4.0, 3.0));
            painter.rect_filled(shine, 1.0, Color32::WHITE);
        }
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

/// How a step's light in [`step_lights`] looks when it is not playing:
/// the first step of a bar and of a beat are a little brighter, so the
/// row can be counted like the steps under it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StepMark {
    Bar,
    Beat,
    Plain,
}

/// The height of the row of lights above the rack's steps, unzoomed.
pub const STEP_LIGHTS_HEIGHT: f32 = 10.0;

/// The height of the row of lights above the rack's steps in `ui`.
pub fn step_lights_height(ui: &Ui) -> f32 {
    STEP_LIGHTS_HEIGHT * zoom(ui)
}

/// A row of lights over a pattern's steps, like those of a drum machine:
/// the one over the step that is sounding is lit, the others are dim.
/// `left` is where the first step starts, and `marks` says which steps
/// begin a bar or a beat.
pub fn step_lights(ui: &Ui, row: Rect, left: f32, marks: &[StepMark], playhead: Option<usize>) {
    let gap = ui.spacing().item_spacing.x;
    let step_width = step_size(ui).x + gap;
    let zoom = zoom(ui);
    let painter = ui.painter();
    for (step, &mark) in marks.iter().enumerate() {
        let x = left + step as f32 * step_width;
        let light = Rect::from_min_size(
            pos2(x + 3.0 * zoom, row.center().y - 2.5 * zoom),
            vec2(step_size(ui).x - 6.0 * zoom, 5.0 * zoom),
        );
        if !ui.is_rect_visible(light) {
            continue;
        }
        if playhead == Some(step) {
            // A soft glow around the lit one, so it catches the eye.
            painter.rect_filled(light.expand(2.0 * zoom), 3.0, PLAYHEAD.gamma_multiply(0.35));
            painter.rect_filled(light, 2.0, PLAYHEAD);
        } else {
            let color = match mark {
                StepMark::Bar => Color32::from_rgb(112, 116, 124),
                StepMark::Beat => Color32::from_rgb(88, 92, 100),
                StepMark::Plain => LED_OFF,
            };
            painter.rect_filled(light, 2.0, color);
        }
    }
}

/// A round knob for a value from 0.0 to 1.0. Drag up or down to turn it;
/// double-click to reset it to `default`.
pub fn knob(ui: &mut Ui, value: &mut f32, default: f32) -> Response {
    let zoom = zoom(ui);
    let (rect, mut response) =
        ui.allocate_exact_size(vec2(24.0, 24.0) * zoom, Sense::click_and_drag());

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
        let radius = 9.0 * zoom;
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
    let zoom = zoom(ui);
    let (rect, response) = ui.allocate_exact_size(vec2(14.0, 24.0) * zoom, Sense::click());
    if ui.is_rect_visible(rect) {
        let color = if enabled { LED_ON } else { LED_OFF };
        ui.painter().circle_filled(rect.center(), 4.5 * zoom, color);
    }
    response
}

/// A small padlock: closed and lit while the channel is locked (see
/// [`lineup_core::lock`]), open and faint while it is not.
pub fn lock_light(ui: &mut Ui, locked: bool) -> Response {
    let zoom = zoom(ui);
    let (rect, response) = ui.allocate_exact_size(vec2(14.0, 24.0) * zoom, Sense::click());
    if ui.is_rect_visible(rect) {
        let color = match (locked, response.hovered()) {
            (true, _) => ui.visuals().warn_fg_color,
            (false, true) => ui.visuals().weak_text_color(),
            (false, false) => LED_OFF,
        };
        let body =
            Rect::from_center_size(rect.center() + vec2(0.0, 3.0) * zoom, vec2(9.0, 7.0) * zoom);
        // The shackle is an arch over the body, lifted out of it on one
        // side when the lock is open.
        let radius = 3.0 * zoom;
        let top = body.top() - if locked { 2.0 } else { 4.0 } * zoom;
        let arch = (0..=12).map(|step| {
            let angle = std::f32::consts::PI * step as f32 / 12.0;
            pos2(
                body.center().x - radius * angle.cos(),
                top - radius * angle.sin(),
            )
        });
        let left = pos2(
            body.center().x - radius,
            if locked { body.top() } else { top },
        );
        let right = pos2(body.center().x + radius, body.top());
        let shackle: Vec<_> = std::iter::once(left).chain(arch).chain([right]).collect();
        let painter = ui.painter();
        painter.add(egui::Shape::line(shackle, Stroke::new(1.5 * zoom, color)));
        if locked {
            painter.rect_filled(body, 1.5 * zoom, color);
        } else {
            painter.rect_stroke(
                body,
                1.5 * zoom,
                Stroke::new(1.2 * zoom, color),
                StrokeKind::Inside,
            );
        }
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
    let step_size = step_size(ui);
    let step_width = step_size.x + gap;
    let size = vec2(step_width * length as f32 - gap, step_size.y);
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
            Rect::from_min_max(pos2(x, inner.top()), pos2(x + step_size.x, inner.bottom()));
        painter.rect_filled(column, 2.0, PLAYHEAD.gamma_multiply(0.25));
        painter.rect_stroke(column, 2.0, Stroke::new(2.0, PLAYHEAD), StrokeKind::Inside);
    }
    response
}

/// Scales the text and spacing of a window's contents by `scale`, and
/// remembers it for [`zoom`]. Called first thing inside the window.
pub fn zoom_contents(ui: &mut egui::Ui, scale: f32) {
    ui.ctx()
        .data_mut(|data| data.insert_temp(egui::Id::unique(("scale", ui.layer_id())), scale));
    if scale == 1.0 {
        return;
    }
    let style = ui.style_mut();
    for font in style.text_styles.values_mut() {
        font.size *= scale;
    }
    let spacing = &mut style.spacing;
    spacing.item_spacing *= scale;
    spacing.button_padding *= scale;
    spacing.interact_size *= scale;
    spacing.indent *= scale;
    spacing.slider_width *= scale;
    spacing.combo_width *= scale;
    spacing.text_edit_width *= scale;
    spacing.icon_width *= scale;
    spacing.icon_width_inner *= scale;
    spacing.icon_spacing *= scale;
    spacing.combo_height *= scale;
}

/// How much bigger than usual the window `ui` is in should draw what it
/// draws itself: 1.0 unless [`zoom_contents`] said otherwise.
pub fn zoom(ui: &egui::Ui) -> f32 {
    ui.ctx()
        .data(|data| data.get_temp(egui::Id::unique(("scale", ui.layer_id()))))
        .unwrap_or(1.0)
}

/// Turns off "rubber-banding" for the scroll areas `ui` shows next:
/// scrolling past the end of the contents, which then springs back.
///
/// A zoom changes how big the contents are and jumps the view to keep
/// what is under the pointer in place. On that frame a scroll area still
/// knows only the old size, so it would take the jump for scrolling past
/// the end and spring part of the way back, moving the view after the
/// zoom.
pub fn no_rubber_band(ui: &mut egui::Ui) {
    ui.spacing_mut().scroll.kinetic.rubber_band = false;
}

/// Keeps track of whether the mouse button now down went down while a
/// menu or drop-down list was open. Called at the start of every frame.
///
/// egui closes a menu when you click outside it, but the click still
/// reaches whatever is under the pointer. On the grids that would draw a
/// note, place a clip or turn on a step just to close a menu; in most
/// programs that click only closes the menu. The
/// grids ask [`dismissing`] and leave such a click alone, from the frame
/// the button goes down to the frame it comes up.
pub fn track_dismissing(ctx: &egui::Context) {
    let id = dismissing_id();
    let (pressed, down, released) = ctx.input(|input| {
        (
            press_pos(input, None).is_some(),
            input.pointer.any_down(),
            input.pointer.any_released(),
        )
    });
    let open = egui::Popup::is_any_open(ctx);
    ctx.data_mut(|data| {
        let dismissing = data.get_temp_mut_or(id, false);
        if pressed {
            *dismissing = open;
        } else if !down && !released {
            *dismissing = false;
        }
    });
}

/// Whether the mouse button now down (or just released) went down to
/// close a menu. See [`track_dismissing`].
pub fn dismissing(ctx: &egui::Context) -> bool {
    ctx.data(|data| data.get_temp(dismissing_id()))
        .unwrap_or(false)
}

fn dismissing_id() -> egui::Id {
    egui::Id::unique("press-closes-a-menu")
}

/// Where a mouse button went down this frame, if one did: only the
/// primary (left) button, or `None` for any button.
///
/// egui's `press_origin` is forgotten when the button comes back up, and
/// a quick tap (on a touchpad, say) can press and release within one
/// frame. So the press is read from the frame's events instead, which
/// also gives the place it went down, not where the pointer is now.
pub fn press_pos(
    input: &egui::InputState,
    only: Option<egui::PointerButton>,
) -> Option<egui::Pos2> {
    input.events.iter().find_map(|event| match *event {
        egui::Event::PointerButton {
            pos,
            button,
            pressed: true,
            ..
        } if only.is_none_or(|only| only == button) => Some(pos),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_follow_scrolls_to_the_cursor_and_can_be_disabled() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut offset = 0.0;
        for (frame, follow) in [false, false, false, true, true, true]
            .into_iter()
            .enumerate()
        {
            ctx.data_mut(|data| data.insert_persisted(egui::Id::unique("follow-playback"), follow));
            let mut output = ctx.run_pass(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(800.0, 600.0))),
                    time: Some(frame as f64 * 0.1),
                    ..Default::default()
                },
                |ctx| {
                    egui::Window::new("Follow test")
                        .default_size([400.0, 250.0])
                        .show(ctx, |ui| {
                            let result =
                                egui::ScrollArea::horizontal()
                                    .animated(false)
                                    .show(ui, |ui| {
                                        let view = ui.clip_rect();
                                        let (rect, _) = ui.allocate_exact_size(
                                            vec2(2000.0, 40.0),
                                            Sense::hover(),
                                        );
                                        follow_x(ui, Some(100), rect.left() + 1500.0, view);
                                    });
                            offset = result.state.clamped_offset().x;
                        });
                },
            );
            output.textures_delta.clear();
            if !follow {
                assert_eq!(offset, 0.0);
            }
        }
        assert!(
            offset > 1000.0,
            "the playing cursor should be revealed: {offset}"
        );
    }
}
