//! The look of the app: its colours, spacing, and a few small pieces that
//! the toolbar and the bars share.
//!
//! Pattern-based studios have a recognisable look: dark blue-grey panels,
//! raised grey buttons, an orange light for whatever is selected or
//! switched on, and black "LCD" displays for numbers such as the tempo.
//! [`apply`] sets egui's style to match once, at startup; every window
//! and widget then picks it up without being told.
//!
//! The hint bar is another idea borrowed from those studios: a line at the
//! top of the window that says what the thing under the mouse does. Call
//! [`hint`] on a widget's response to give it a hint, and [`HintBar`]
//! shows it.

use eframe::egui::{
    self, Color32, CornerRadius, FontFamily, FontId, Margin, Response, RichText, Shadow, Stroke,
    TextStyle, Ui, vec2,
};

/// The darkest background: the menu bar, the status bar and text fields.
pub const BACKGROUND: Color32 = Color32::from_rgb(33, 38, 42);
/// The toolbar and the bars along the bottom.
pub const PANEL: Color32 = Color32::from_rgb(52, 60, 66);
/// Windows such as the channel rack and the piano roll.
pub const WINDOW: Color32 = Color32::from_rgb(44, 51, 56);
/// The frame around a group of controls in the toolbar.
pub const GROUP: Color32 = Color32::from_rgb(41, 47, 52);
/// A raised button, at rest, when the mouse is over it, and pressed.
pub const BUTTON: Color32 = Color32::from_rgb(72, 82, 89);
pub const BUTTON_HOVER: Color32 = Color32::from_rgb(88, 100, 108);
pub const BUTTON_PRESSED: Color32 = Color32::from_rgb(104, 117, 126);
/// The lines around buttons and between panels.
pub const OUTLINE: Color32 = Color32::from_rgb(24, 28, 31);
pub const TEXT: Color32 = Color32::from_rgb(216, 223, 228);
pub const TEXT_DIM: Color32 = Color32::from_rgb(142, 154, 162);
/// Whatever is selected or switched on.
pub const ACCENT: Color32 = Color32::from_rgb(246, 150, 58);
/// Text drawn on top of [`ACCENT`].
pub const ON_ACCENT: Color32 = Color32::from_rgb(34, 24, 14);
/// The play button while playing, and song mode.
pub const GREEN: Color32 = Color32::from_rgb(118, 214, 108);
/// The record button while recording.
pub const RED: Color32 = Color32::from_rgb(226, 74, 66);
/// The part of the song that loops, on the playlist and the chord sheet.
pub const LOOP: Color32 = Color32::from_rgb(92, 196, 120);
/// The displays for the tempo and the song position.
pub const LCD: Color32 = Color32::from_rgb(16, 20, 22);
pub const LCD_TEXT: Color32 = Color32::from_rgb(150, 230, 210);
pub const LCD_LABEL: Color32 = Color32::from_rgb(88, 128, 120);

/// Sets egui's style to the app's look. Called once, at startup.
pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        let visuals = &mut style.visuals;
        visuals.dark_mode = true;
        visuals.panel_fill = PANEL;
        visuals.window_fill = WINDOW;
        visuals.window_stroke = Stroke::new(1.0, OUTLINE);
        visuals.window_corner_radius = CornerRadius::same(4);
        visuals.window_shadow = Shadow {
            offset: [0, 6],
            blur: 18,
            spread: 0,
            color: Color32::from_black_alpha(110),
        };
        visuals.popup_shadow = Shadow {
            offset: [0, 4],
            blur: 10,
            spread: 0,
            color: Color32::from_black_alpha(90),
        };
        visuals.menu_corner_radius = CornerRadius::same(3);
        visuals.extreme_bg_color = BACKGROUND;
        visuals.faint_bg_color = Color32::from_rgb(47, 54, 59);
        visuals.code_bg_color = LCD;
        visuals.hyperlink_color = ACCENT;
        visuals.warn_fg_color = Color32::from_rgb(250, 196, 92);
        visuals.error_fg_color = Color32::from_rgb(255, 118, 104);
        visuals.selection.bg_fill = ACCENT;
        visuals.selection.stroke = Stroke::new(1.0, ON_ACCENT);
        visuals.slider_trailing_fill = true;

        let radius = CornerRadius::same(3);
        let widgets = &mut visuals.widgets;
        widgets.noninteractive.bg_fill = WINDOW;
        widgets.noninteractive.weak_bg_fill = WINDOW;
        widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(64, 73, 80));
        widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);
        widgets.noninteractive.corner_radius = radius;
        for (state, fill) in [
            (&mut widgets.inactive, BUTTON),
            (&mut widgets.hovered, BUTTON_HOVER),
            (&mut widgets.active, BUTTON_PRESSED),
            (&mut widgets.open, BUTTON_HOVER),
        ] {
            state.bg_fill = fill;
            state.weak_bg_fill = fill;
            state.bg_stroke = Stroke::new(1.0, OUTLINE);
            state.corner_radius = radius;
            state.expansion = 0.0;
        }
        widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
        widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE);
        widgets.active.fg_stroke = Stroke::new(2.0, Color32::WHITE);
        widgets.open.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(130, 142, 150));

        let spacing = &mut style.spacing;
        spacing.item_spacing = vec2(6.0, 4.0);
        spacing.button_padding = vec2(7.0, 3.0);
        spacing.window_margin = Margin::same(6);
        spacing.menu_margin = Margin::same(5);
        spacing.interact_size.y = 22.0;
        spacing.scroll = egui::style::ScrollStyle::solid();
        spacing.scroll.bar_width = 10.0;
        spacing.scroll.dormant_handle_opacity = 0.8;
        spacing.scroll.active_handle_opacity = 1.0;
        spacing.scroll.dormant_background_opacity = 0.3;

        style.interaction.tooltip_delay = 0.6;
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(17.0, FontFamily::Proportional),
        );
    });
}

/// A group of controls in the toolbar, inside a dark rounded frame.
pub fn group<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    wrapping(ui, |ui| {
        egui::Frame::new()
            .fill(GROUP)
            .stroke(Stroke::new(1.0, OUTLINE))
            .corner_radius(4)
            .inner_margin(Margin::symmetric(4, 3))
            .show(ui, |ui| ui.horizontal(add).inner)
    })
    .inner
}

/// Lays out `add` left to right as one piece of a wrapping row, so that
/// a label and its list move to the next row together (see
/// [`wrapping`]). Until the piece has been drawn once, `guess` stands in
/// for its width.
///
/// In a window this matters even more than in the toolbar: a row that
/// runs past the edge for a single frame makes egui widen the window for
/// good.
pub fn keep_together<R>(ui: &mut Ui, guess: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    wrapping_from(ui, Some(guess), |ui| ui.horizontal(add)).inner
}

/// Draws a frame as one piece in a wrapping row: when it was too wide for
/// what is left of the row last time, it starts a new row first.
///
/// A wrapping layout only wraps single widgets. A frame is drawn into the
/// rest of the row and only measured afterwards, so without this, a group
/// that does not fit runs off the edge instead of moving down. Its width
/// from the frame before is kept in egui's memory, under an id from its
/// place in the toolbar.
fn wrapping<R>(
    ui: &mut Ui,
    add: impl FnOnce(&mut Ui) -> egui::InnerResponse<R>,
) -> egui::InnerResponse<R> {
    wrapping_from(ui, None, add)
}

/// [`wrapping`], with a width to assume before the frame is first drawn.
fn wrapping_from<R>(
    ui: &mut Ui,
    guess: Option<f32>,
    add: impl FnOnce(&mut Ui) -> egui::InnerResponse<R>,
) -> egui::InnerResponse<R> {
    let id = ui.next_auto_id().with("wrapping");
    let last_width = ui.ctx().data(|data| data.get_temp::<f32>(id)).or(guess);
    let row_started = ui.cursor().min.x > ui.max_rect().min.x;
    if let Some(width) = last_width
        && ui.layout().main_wrap
        && row_started
        && width > ui.available_size_before_wrap().x
    {
        ui.end_row();
    }
    let response = add(ui);
    let width = response.response.rect.width();
    ui.ctx().data_mut(|data| data.insert_temp(id, width));
    response
}

/// A small black display, like the ones on hardware, showing `value` with
/// a dim `label` above it.
pub fn lcd(ui: &mut Ui, label: &str, value: &str, width: f32) -> Response {
    lcd_frame(ui, label, width, |ui| {
        ui.label(
            RichText::new(value)
                .font(FontId::monospace(16.0))
                .color(LCD_TEXT),
        )
    })
}

/// An LCD frame around anything: used for the tempo, whose value you can
/// drag. Widgets inside it are drawn black with LCD-coloured text.
pub fn lcd_frame(
    ui: &mut Ui,
    label: &str,
    width: f32,
    add: impl FnOnce(&mut Ui) -> Response,
) -> Response {
    wrapping(ui, |ui| {
        egui::Frame::new()
            .fill(LCD)
            .stroke(Stroke::new(1.0, OUTLINE))
            .corner_radius(3)
            .inner_margin(Margin::symmetric(6, 1))
            .show(ui, |ui| {
                ui.set_width(width);
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(label)
                            .font(FontId::proportional(9.0))
                            .color(LCD_LABEL),
                    );
                    let visuals = &mut ui.visuals_mut().widgets;
                    for state in [
                        &mut visuals.inactive,
                        &mut visuals.hovered,
                        &mut visuals.active,
                    ] {
                        state.bg_fill = LCD;
                        state.weak_bg_fill = LCD;
                        state.bg_stroke = Stroke::NONE;
                        state.fg_stroke.color = LCD_TEXT;
                    }
                    ui.style_mut().override_font_id = Some(FontId::monospace(16.0));
                    ui.spacing_mut().button_padding = vec2(0.0, 0.0);
                    add(ui)
                })
                .inner
            })
    })
    .inner
}

/// A toolbar button that stays lit while `on`, in the accent colour or in
/// `color` if given. Returns the response, so the caller can check
/// `clicked()`.
pub fn light_button(ui: &mut Ui, text: &str, on: bool, color: Option<Color32>) -> Response {
    let light = color.unwrap_or(ACCENT);
    // A symbol with no letters (play, stop, record) is drawn bigger, on
    // a square button.
    let icon = !text.chars().any(char::is_alphanumeric);
    let mut text = RichText::new(text);
    if icon {
        text = text.size(16.0);
    }
    let button = if on {
        egui::Button::new(text.color(ON_ACCENT).strong()).fill(light)
    } else {
        egui::Button::new(text.color(TEXT))
    };
    let width = if icon { 30.0 } else { 0.0 };
    ui.add(button.min_size(vec2(width, 26.0)))
}

/// Gives a widget a hint for the hint bar, and the same text as a tooltip
/// for those who wait.
pub fn hint(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    if response.hovered() {
        response
            .ctx
            .data_mut(|data| data.insert_temp(hint_id(), text.clone()));
    }
    response.on_hover_text(text)
}

fn hint_id() -> egui::Id {
    egui::Id::unique("hint-bar")
}

/// What the hint bar shows when the mouse is over nothing with a hint.
const DEFAULT_HINT: &str = "Space plays and stops · L switches pattern and song · \
                            F5 chord sheet · F6 channel rack · F7 piano roll · F8 harmony keys";

/// The line at the top of the window that says what the thing under the
/// mouse does.
///
/// Widgets record their hint while they are drawn, but the bar is drawn
/// first, at the top. So the bar shows the hint recorded on the frame
/// before; [`HintBar::start_frame`] picks it up and clears it for this
/// frame.
#[derive(Default)]
pub struct HintBar {
    text: Option<String>,
}

impl HintBar {
    /// Takes the hint left by the last frame. Call before drawing anything.
    pub fn start_frame(&mut self, ctx: &egui::Context) {
        let text = ctx.data_mut(|data| data.remove_temp::<String>(hint_id()));
        if text != self.text {
            // The mouse moved onto or off a widget: draw once more, so the
            // bar does not lag a frame behind until the next event.
            ctx.request_repaint();
        }
        self.text = text;
    }

    pub fn show(&self, ui: &mut Ui) {
        let text = self.text.as_deref().unwrap_or(DEFAULT_HINT);
        // Only the first line: the bar is a single row.
        let line = text.lines().next().unwrap_or_default();
        let color = if self.text.is_some() { TEXT } else { TEXT_DIM };
        ui.add(
            egui::Label::new(RichText::new(line).color(color).italics())
                .truncate()
                .selectable(false),
        );
    }
}
