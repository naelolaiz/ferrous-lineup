# Drawing the windows with egui: layout, zoom, themes, icons

Files: [`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs),
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs),
[`crates/lineup-app/src/theme.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/theme.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs),
[`crates/lineup-app/src/icons.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/icons.rs),
[`crates/lineup-app/src/app/view.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/view.rs),
[`crates/lineup-app/src/app/shortcuts.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/shortcuts.rs)
and [`crates/lineup-app/src/app/about.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/about.rs)

The window is drawn with [egui](https://github.com/emilk/egui), a GUI
library written in Rust, and [eframe](https://docs.rs/eframe), egui's
framework for running it as a native app on Linux, macOS and Windows.
This chapter is about how the main window is put together each frame,
the look, how rows of controls fit any window size, the zooms, the
windows that float over the playlist, and the Help menu. The editors
themselves, the channel rack, the piano roll and the playlist, have the
[next chapter](editors.md).

## Immediate mode

Most GUI toolkits are **retained mode**: you create button objects once,
keep them in a tree and attach callbacks to them. egui is
**immediate mode**: on every frame, you call a function that describes
the *whole* UI from your own data, and the library draws it and tells you
what was clicked.

```rust
if ui.button("▶ Play").clicked() {
    self.set_playing(true);
}
```

That line both draws the button and checks whether it was clicked. There
are no button objects to keep in sync with your state. The state is just
the fields of your struct, and the UI is always a picture of it. This fits
Rust nicely, because there are no callbacks holding references to your
data.

The price is that a frame cannot go back. Something drawn early in the
frame cannot know what is drawn later in it. Several pieces of this
chapter (the hint bar, wrapping groups, zoom) solve that the same way:
they use what the *previous* frame found, a sixtieth of a second old.

## `main.rs`

```rust
fn main() -> eframe::Result {
    // The first command-line argument, if there is one, is a file to open,
    // or what to do instead of opening the window.
    let mut args = std::env::args_os().skip(1);
    let first = args.next();
    match first.as_ref().and_then(|arg| arg.to_str()) {
        Some("--trace") => std::process::exit(trace(args)),
        Some("--demos") => { /* print the demos' names */ return Ok(()); }
        Some("--help" | "-h") => { /* print how to use it */ return Ok(()); }
        _ => {}
    }
    let path = first.map(PathBuf::from);

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_title(app::APP_NAME)
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([800.0, 520.0])
        .with_icon(window_icon());
    // ...inside a Flatpak, the window takes the sandbox's app id...

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        app::APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc, path)))),
    )
}
```

- `main` returns a `Result`. If it returns `Err`, Rust prints the error
  and exits with a failure code.
- `ViewportBuilder::default().with_title(...)` is the **builder pattern**:
  each method takes the builder and returns it modified, so calls chain.
- `..Default::default()` is **struct update syntax**: "every field I did
  not list takes its default value".
- `Box::new(|cc| ...)` passes a closure that eframe calls once to create
  our app. `Box` puts a value on the **heap**. It is needed here because
  eframe accepts *any* type implementing its `App` trait, and to store a
  value whose exact type it does not know, it needs a pointer to it:
  `Box<dyn App>`. The `dyn` keyword means "some type implementing this
  trait, decided at run time" (a **trait object**).
- `"--help" | "-h"` is an **or-pattern**: the arm matches either string.

## `LineupApp`: the state

`LineupApp` holds everything the window needs: the project, the undo
history, the open windows and their own state, the file, the settings,
and the connection to the audio thread. That connection is an enum:

```rust
enum Audio {
    /// Boxed: the engine is much bigger than a `String`, and every
    /// `Audio` would otherwise take as much room as the biggest variant.
    Running(Box<AudioEngine>),
    Unavailable(String),
}
```

It captures a real situation: either there is a running engine, or there
is the reason there is not. You cannot have both, and you cannot have
neither, so the type cannot represent an impossible state. Compare that
with the usual alternative, an `Option<AudioEngine>` plus an
`Option<String>`, which allows four combinations of which two make no
sense.

An enum is as big as its biggest variant, plus room to say which one it
is. `Box` keeps the big engine on the heap, so `Audio` is the size of a
pointer or a `String`, whichever is bigger. Clippy points this out with
its `large_enum_variant` lint.

Using it:

```rust
fn send(&mut self, command: Command) {
    if let Audio::Running(engine) = &mut self.audio {
        engine.send(command);
    }
}
```

`if let` is a one-arm `match`: if `self.audio` is `Running`, bind the
engine inside to `engine` and run the block. Matching on `&mut self.audio`
borrows the engine mutably instead of moving it out of the struct.

If the computer has no working sound card, the app still opens and the
status bar says why in orange: "Audio unavailable: …". A failure becomes
data we display, rather than a crash.

## One frame

eframe calls `ui` every time the window needs repainting:

```rust
impl eframe::App for LineupApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Audio::Running(engine) = &mut self.audio {
            engine.collect_garbage();
        }
        self.check_trace();
        let ctx = ui.ctx().clone();
        self.before_drawing(&ctx);
        self.panels(ui, &ctx);
        let playing = self.song_position().filter(|_| self.playing);
        let area = self.main_area(ui, &ctx, playing);

        // The channel rack and the piano roll float on top of the playlist.
        let playhead = self.pattern_playhead();
        self.bring_window_to_front(&ctx);
        self.rack_window(&ctx, area, playhead);
        self.piano_roll_window(&ctx, area, playhead);
        // The chord sheet is drawn last, so it opens over the others.
        self.chord_sheet_window(&ctx, area, playing);
        // ...ear training, the lead sheet, the mixer, the shortcuts,
        // About, the style editor, Settings and the dialogs...
        self.unsaved_changes_dialog(&ctx);
        self.update_title(&ctx);
        self.end_frame_for_undo(&ctx);
    }

    fn on_exit(&mut self) {
        self.forget_recovery();
    }
}
```

The order is the design:

1. **`collect_garbage`** frees what the audio thread sent back (see [The
   trash queue](threads.md#freeing-memory-somewhere-else-the-trash-queue)).
2. **`before_drawing`** does everything that must happen before anything
   is drawn: it notes whether a click is closing a menu, sets the
   language and the note names for the frame, closes an undo step on a
   mouse press, writes the recovery copy when due, reads the keyboard
   shortcuts and the zoom keys, reads MIDI input, and checks on exports
   and imports running in the background. While playing it asks egui
   for another frame at once (`ctx.request_repaint()`), so the playhead
   moves; otherwise egui draws only when something happens.
3. **`panels`** draws the bars along the edges: the menus with the hint
   bar, the toolbar, the status bar, and the harmony keys. Panels must
   come before the central area, which takes whatever space is left.
4. **`main_area`** draws the playlist (or the chord sheet, when it fills
   the main window) and returns the rectangle it took.
5. **The windows** float over that rectangle. Windows drawn later are
   on top, so the chord sheet comes last of the three editors.
6. **`end_frame_for_undo`** decides whether the undo step ends (see
   [Undo and redo](undo.md#when-does-one-step-end)).

`on_exit` runs once when the app closes cleanly: there is no unsaved
work to recover, so this run's recovery copy goes.

The many `self.something(ui)` methods keep `ui` short. Each closure
passed to a panel can borrow `self` mutably, because each one runs and
finishes before the next starts, which the borrow checker can see.

## A theme is one function

egui draws every widget from a `Style`: colours in `style.visuals`,
sizes and gaps in `style.spacing`. Nothing is painted with a hard-coded
colour unless the code asks for one. So the whole look is one function,
`theme::apply`, called once when the app starts:

```rust
pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        let visuals = &mut style.visuals;
        visuals.panel_fill = PANEL;
        visuals.window_fill = WINDOW;
        visuals.selection.bg_fill = ACCENT; // selected and switched-on things
        visuals.selection.stroke = Stroke::new(1.0, ON_ACCENT);
        // ...shadows, corners, warning and error colours...
        let widgets = &mut visuals.widgets;
        for (state, fill) in [
            (&mut widgets.inactive, BUTTON),
            (&mut widgets.hovered, BUTTON_HOVER),
            (&mut widgets.active, BUTTON_PRESSED),
            (&mut widgets.open, BUTTON_HOVER),
        ] {
            state.bg_fill = fill;
            state.weak_bg_fill = fill;
            state.bg_stroke = Stroke::new(1.0, OUTLINE);
            // ...
        }
        // ...spacing...
    });
}
```

The look borrows from pattern-based studios: dark blue-grey panels,
raised grey buttons, an orange light for whatever is selected or
switched on, and black "LCD" displays for numbers such as the tempo.
Three things to notice in the code:

- `style_mut_of(Theme::Dark, ...)` changes the *dark* style only. egui
  keeps one style for dark and one for light, and the app always uses
  dark.
- The `for` loop walks an **array of pairs of mutable references**. Each
  `&mut widgets.inactive` borrows a *different field*, so Rust allows
  several of them at once: the borrow checker tracks fields of a struct
  separately.
- The colours are `const` items: `Color32::from_rgb` is a `const fn`, so
  it runs at compile time and costs nothing when the app runs.

Views still have their own constants for the things only they draw (the
playlist's rows, the piano roll's keys, the step buttons), with values
that match the theme.

### Changing the style for a few widgets

The tempo display is a `DragValue`, egui's number you drag or type, but
drawn black with green-blue digits. Its style is changed only inside its
frame:

```rust
pub fn lcd_frame(
    ui: &mut Ui,
    label: &str,
    width: f32,
    add: impl FnOnce(&mut Ui) -> Response,
) -> Response {
    wrapping(ui, |ui| {
        egui::Frame::new()
            .fill(LCD)
            // ...
            .show(ui, |ui| {
                // ...the label above, written smaller if it is long...
                ui.vertical(|ui| {
                    ui.label(label);
                    let visuals = &mut ui.visuals_mut().widgets;
                    for state in [&mut visuals.inactive, &mut visuals.hovered, &mut visuals.active] {
                        state.bg_fill = LCD;
                        state.fg_stroke.color = LCD_TEXT;
                        // ...
                    }
                    ui.style_mut().override_font_id = Some(FontId::monospace(16.0));
                    add(ui)
                })
                .inner
            })
    })
    .inner
}
```

Every `Ui` has its own copy of the style (an `Arc<Style>`; `style_mut`
makes a private copy the first time it is changed). The frame's
contents get a child `Ui`, so the change stays inside the frame and the
rest of the toolbar is untouched. `add` is a closure that *returns* the
widget's `Response`, so the caller can still ask `changed()` outside:

```rust
let tempo = theme::lcd_frame(ui, tr("TEMPO"), 62.0, |ui| {
    ui.add(egui::DragValue::new(&mut bpm).range(Tempo::MIN..=Tempo::MAX))
});
```

`Tempo`'s field is private, so the value is copied into a local `bpm`,
the widget edits that, and a new `Tempo` is built if it changed. The
`Tempo` stays valid whatever is typed. `Tempo::MIN..=Tempo::MAX` is an
**inclusive range**: `a..b` leaves `b` out, `a..=b` keeps it.

A label longer than the display, as some are in Spanish, is written a
little smaller to fit, so the toolbar keeps its width in every
language.

## The hint bar: a message from the future

Next to the menus, a line says what the control under the mouse does.
That line is drawn *first*, at the top of the window, but the widgets
that know what the mouse is on are drawn *later* in the same frame. In
immediate mode there is no going back to repaint the bar.

So the bar shows what was found on the **previous** frame. Widgets leave
their hint in egui's memory, `ctx.data`, which is a map from an `Id` to
a value of any type:

```rust
pub fn hint(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    if response.hovered() {
        response
            .ctx
            .data_mut(|data| data.insert_temp(hint_id(), text.clone()));
    }
    response.on_hover_text(text)
}
```

At the start of the next frame, the bar takes it out, so it does not
linger once the mouse moves away:

```rust
pub fn start_frame(&mut self, ctx: &egui::Context) {
    let text = ctx.data_mut(|data| data.remove_temp::<String>(hint_id()));
    if text != self.text {
        // The mouse moved onto or off a widget: draw once more, so the
        // bar does not lag a frame behind until the next event.
        ctx.request_repaint();
    }
    self.text = text;
}
```

`remove_temp::<String>` uses the **turbofish** `::<String>` to say which
type to take out: the map can hold values of many types under the same
`Id`. The `request_repaint` is the subtle part. egui only draws a frame
when something happens. When the mouse moves onto a button, that frame
records the hint, but the bar still shows the old one. Asking for one
more frame shows the new hint straight away, instead of whenever the
mouse next moves.

## Rows that fit any window

The toolbar holds the play mode, the transport, the tempo and position
displays, the pattern selector, the five window buttons, the count-in,
the metronome and the master volume. In a window 1280 points wide it
fits on one row; a test, `the_toolbar_fits_on_one_row_at_1280_points`,
checks that it stays so. A narrower or zoomed window must still reach
every control.

![The toolbar on one row: the five window buttons and the Count-in and
Metronome toggles are pictures.](../images/step-9-workflow-toolbar.png)

egui lays widgets out one after another. `ui.horizontal` puts them in a
row and keeps going past the edge if they do not fit: whatever is past
the edge is still there, just not drawn. `ui.horizontal_wrapped` is the
same row, except that a widget that would not fit starts a new row. The
toolbar and the harmony keys bar are wrapping rows, so on a narrow
window the second part of them moves down.

### Groups that wrap: last frame's width

The toolbar's buttons are in groups, each inside a dark frame. A
wrapping row decides to wrap by asking a widget how big it wants to be
before placing it. A frame cannot answer that: `Frame::show` draws its
contents into whatever room is left, and only knows its size afterwards.
So `theme::wrapping` uses the frame before. egui redraws the window many
times a second, so last time's width is a very good guess:

```rust
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
```

- **`ui.next_auto_id()`** is the id egui would give the next widget. It
  depends on how many widgets came before in this `Ui`, so it is the same
  every frame as long as the toolbar is drawn in the same order. That
  makes it a key for remembering something about one group.
- **`get_temp::<f32>`** asks egui's memory for an `f32`; the turbofish
  tells Rust which type to look for, because nothing else in the line
  says so.
- **The `if let ... && ...` chain** (a "let chain") only runs `end_row`
  when there is a width from last time, the layout wraps, the row
  already has something in it, and the group does not fit. The very
  first frame has no width yet, so the group may stick out for one
  frame, which nobody sees.

`theme::group` and `lcd_frame` wrap their frames in it, and
`keep_together` offers the same to any group of widgets, with a `guess`
for the very first frame. That first frame matters inside a window:
egui windows grow to fit their contents, and **a single frame of
contents that is too wide is enough to widen a window for good**. The
piano roll's toolbar keeps its groups together this way, so a long
scale name (F# major's "F# G# A# B C# D# E#") moves its group to a
second row instead of widening the window.

### Text that gets shorter, and right to left

Wrapping suits buttons. For a long line of text, like a status message,
it is often better to cut it short. Every text in egui has a *wrap
mode*: `Wrap` breaks it onto more lines, `Truncate` cuts it with "…",
and `Extend` lets it run on. A whole area can set it for all its text
through its style:

```rust
ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
```

A truncated label shows the whole text when you point at it, so nothing
is lost.

The status bar has two parts: messages on the left, and the playlist's
zoom controls on the right. If the messages were drawn first, a long one
("Audio unavailable: could not read the device settings…") would take
all the room and push the zoom out. So the bar is laid out *right to
left*: the zoom goes in first, from the right edge, and the messages get
whatever is left:

```rust
fn status_bar(&self, ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        view::zoom_control(ui);
        ui.separator();
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
            self.status_messages(ui);
        });
    });
}
```

In a right-to-left layout the first widget is the rightmost one, which
is why `zoom_control` adds its **+** button before the percentage and
the **−** button.

## Zoom

There are two kinds of zoom.

The **interface size** makes everything bigger or smaller: text,
buttons, menus. egui draws in *points*, not pixels. How many pixels a
point takes is `pixels_per_point`: the screen's own scale (1.0, 1.5 or
2.0 on most screens) times egui's *zoom factor*. `ctx.set_zoom_factor`
changes the second.

The **areas** zoom what they draw without changing the text. As in most
music programs, the playlist and the piano roll zoom their two
directions separately: horizontal zoom makes bars and steps wider (less
time on screen, in more detail), vertical zoom makes tracks and piano
keys taller. The channel rack's steps are square buttons, so it has one
zoom for both.

| Pointer over | Ctrl and + / − (or wheel) | Alt and + / − (or wheel) | Ctrl+0 |
|---|---|---|---|
| Playlist | bars wider / narrower | tracks taller / shorter | both to 100% |
| Piano roll | steps wider / narrower | keys taller / shorter | both to 100% |
| Channel rack | bigger / smaller | the same | 100% |
| Anything else, the chord sheet too | interface bigger / smaller | the same | 100% |

### What is under the pointer?

The keys and the wheel zoom what the pointer is over. In Rust, "one of
these kinds" is an `enum`, and a variant can carry a value:

```rust
pub(crate) enum Target {
    Area(Area), // Area::Main (the playlist), Area::Rack, Area::PianoRoll
    Interface,
}
```

`Target::at` finds it. egui keeps a stack of *layers*: the background
where the panels are drawn, then each window above it, then menus and
tooltips. `ctx.layer_id_at(pos)` says which layer is on top at a point:

```rust
fn at(ctx: &egui::Context, pos: egui::Pos2, playlist: egui::Rect) -> Target {
    match ctx.layer_id_at(pos) {
        Some(layer) if layer.id == super::rack_id() => Target::Area(Area::Rack),
        Some(layer) if layer.id == PianoRoll::id() => Target::Area(Area::PianoRoll),
        Some(layer) if layer.order == egui::Order::Background && playlist.contains(pos) => {
            Target::Area(Area::Main)
        }
        _ => Target::Interface,
    }
}
```

Each arm has a **match guard**, the `if` after the pattern. The pattern
`Some(layer)` matches any layer, and the arm is only taken if the guard
is true too; otherwise `match` tries the next arm. The last arm, `_`,
catches everything left.

The toolbars, the harmony keys and the playlist are all drawn on the
same background layer, so the layer alone cannot tell them apart. The
playlist therefore remembers where it was drawn, and the guard checks
`playlist.contains(pos)`. The zoom keys are read at the start of a
frame, before anything is drawn, so this is the rectangle from the
previous frame. If the pointer is outside the window, the keys zoom
what was clicked last.

The chord sheet is text, so it follows the interface size, in its own
window or in the playlist's place. In the playlist's place it covers the
playlist, and the playlist's last rectangle would be a lie, so
`zoom_shortcuts` passes an empty one instead:

```rust
let playlist = match self.chord_sheet.fills_window() {
    true => egui::Rect::NOTHING,
    false => self.playlist.area(),
};
```

`Rect::NOTHING` contains no point, so the guard is never true, and the
keys fall through to `Target::Interface`.

Every way of zooming (keys, wheel, touchpad pinch) calls the same few
methods on `Target`, and only they look at which kind it is:

```rust
fn step(self, ctx: &egui::Context, axis: Axis, bigger: bool) {
    match self {
        Target::Area(area) => area.step_zoom(ctx, axis, bigger),
        Target::Interface => {
            ctx.set_zoom_factor(interface_step(ctx.zoom_factor(), bigger));
        }
    }
}
```

`Target::Area(area)` both checks the variant and takes the `Area` out of
it into the variable `area`. The interface has a single size, so `axis`
is not used there: Ctrl and Alt do the same.

### A zoom per area and per axis

Each area keeps one number per direction, in egui's memory, under a key
made of both:

```rust
fn key(self, axis: Axis) -> egui::Id {
    let axis = if self.two_way() { axis } else { Axis::Horizontal };
    egui::Id::unique(("zoom-2", self, axis))
}
```

The rack is not `two_way`, so both its axes share the horizontal key:
zooming it "vertically" zooms it both ways. The values are stored with
`insert_persisted`, so egui saves them with the window positions and
they are the same the next time the app starts. The `"zoom-2"` name is
there because an older meaning of "zoom" (a scale for everything in the
window, text included) was saved under `"zoom"`: a new name means old
values are simply not found, instead of being read the wrong way.

Every way of zooming in steps goes through `step_zoom`, which walks a
ladder of sizes:

```rust
const STEPS: [f32; 19] = [
    0.25, 0.33, 0.4, 0.5, 0.6, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0,
    8.0,
];
```

Each step is roughly a fifth bigger than the one before, so it takes as
many steps to go from 100% to 200% as from 400% to 800%. That is how
zoom feels natural: our eyes compare sizes by ratio, not by difference.
Each area and axis has its own range within the ladder (tracks and keys
can only get so short before their names no longer fit), and the
buttons are greyed out at the ends. The interface sizes are a shorter
list of their own, 50% to 250%, the same ones Settings > Appearance
offers.

### Reading the wheel ourselves

egui gives a single `zoom_delta()` for Ctrl+wheel, with no idea of
direction. So the app reads the frame's raw wheel events and sorts them
by the modifier keys held:

```rust
for event in &input.raw.events {
    if let egui::Event::MouseWheel { unit, delta, modifiers, .. } = event {
        // Up or right zooms in. Shift turns a wheel sideways.
        let amount = if delta.y != 0.0 { delta.y } else { delta.x };
        if modifiers.alt {
            tall.add(*unit, amount);
        } else if modifiers.command {
            wide.add(*unit, amount);
        }
    }
}
```

`if let` with a pattern is how Rust picks one variant out of an enum and
names its fields at the same time; `..` ignores the fields we do not
need.

On Linux (X11), one notch of the wheel sometimes arrives as two events,
and zooming two steps per notch feels jumpy. So the notches are added
up and zoom at most one step per frame:

```rust
fn apply(self, target: Target, ctx: &egui::Context, axis: Axis) {
    if self.notches != 0.0 {
        target.step(ctx, axis, self.notches > 0.0);
    }
    if self.smooth != 1.0 {
        target.scale(ctx, axis, self.smooth);
    }
}
```

A frame lasts a few milliseconds, so turning the wheel quickly still
zooms step by step. A touchpad sends many tiny movements instead of
notches (`MouseWheelUnit::Point`); those multiply the zoom smoothly.

### Zooming into what you point at

When the zoom changes, the step and the note under the pointer must stay
under the pointer. The grid is inside a scroll area, so this means
choosing a new scroll offset. The thing under the pointer is at
`from + anchor` in the contents, where `from` is how far the view is
scrolled and `anchor` is where the pointer is in the view. Zooming by
`ratio` moves it to `(from + anchor) * ratio`. For it to stay under the
pointer, the new scroll must be that minus `anchor`. In the piano roll:

```rust
let x = KEY_WIDTH + (from.x + anchor.x - KEY_WIDTH) * ratio.x - anchor.x;
let y = (from.y + anchor.y) * ratio.y - anchor.y;
self.scroll_to = Some(vec2(x.max(0.0), y.max(0.0)));
```

The `KEY_WIDTH` terms are there because the piano keyboard at the left
does not stretch: only what is right of it grows.

One more thing had to be turned off. egui's scroll areas have a
**rubber band**: scroll past the end, and the view springs back. On the
frame of a zoom, the scroll area still knows the *old* size of the
contents, so the new offset looks like scrolling past the end, and the
view would slide back after each zoom. `widgets::no_rubber_band` turns
it off for the grids:

```rust
pub fn no_rubber_band(ui: &mut egui::Ui) {
    ui.spacing_mut().scroll.kinetic.rubber_band = false;
}
```

![The piano roll zoomed wider and taller, then scrolled sideways. The
keyboard stays at the left, and the chord and velocity lanes stay lined
up with the grid.](../images/step-8-two-way-zoom.png)

The channel rack is drawn with ordinary egui widgets, which have no zoom
of their own. `widgets::zoom_contents` makes them bigger by multiplying
the text sizes and the spacing of the window's style, as `lcd_frame`
changes its colours: only inside the rack.

## Windows over the playlist

The channel rack, the piano roll, the chord sheet, the mixer and the
others are `egui::Window`s, which float over everything. They are kept
inside the playlist's rectangle with `Window::constrain_to(area)`, so a
window never hides the toolbar, the harmony keys or the status bar:

```rust
let playlist = egui::CentralPanel::default_margins().show(ui, |ui| { /* ... */ });
let area = playlist.response.rect;
```

Every `show` in egui returns an `InnerResponse`: `.inner` is what the
closure returned (here the playlist's actions) and `.response` tells
where it was drawn.

### Which windows a song opens on

What you study in a song is its bars and chords; what you edit in a
beat is the steps of a pattern. So the app decides what to show by what
was opened:

```rust
/// What a song opens on: the chord sheet in front, since its bars and
/// chords are what you study and write in a song, and the rack out of
/// the way (F6 brings it back).
fn show_chord_sheet(&mut self) {
    self.chord_sheet.open = true;
    self.rack_open = false;
    self.bring_to_front = Some(chord_sheet::window_id());
}

/// What a beat opens on: the rack in front, and no chord sheet.
fn show_beat(&mut self) {
    self.chord_sheet.open = false;
    self.show_rack();
}
```

Song demos, File > New and an imported chord sheet call
`show_chord_sheet`; File > New beat calls `show_beat`. Opening a saved
file leaves the windows as they are, since you chose them yourself. A
demo is a song or a style, an `enum`, and only the songs open on the
sheet:

```rust
if let Demo::Song(_) = demo {
    self.show_chord_sheet();
}
```

The app itself opens on the windows that were open when it was last
closed. `app/windows.rs` keeps them as one small struct of `bool`s, one
for each window of the View menu:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct OpenWindows {
    chord_sheet: bool,
    lead_sheet: bool,
    rack: bool,
    piano_roll: bool,
    harmony_keys: bool,
    /// The Patterns and Library panel.
    browser: bool,
    mixer: bool,
    ear_training: bool,
}
```

Its `Default` is the first time: the chord sheet, the harmony keys and
the Library over the playlist, with no rack. `#[serde(default)]` on the
struct fills a window added to the list later with its value in
`Default`, so an older save still reads. The style editor and the
dialogs are left out: each opens for something, a style or a question,
that the next run does not have.

`App::save`, which eframe calls every 30 seconds and when the window
closes, writes the struct among the preferences kept by name (see
[Kept by name, not by type](#kept-by-name-not-by-type)), and
`LineupApp::new` reads it back:

```rust
fn save(&mut self, storage: &mut dyn eframe::Storage) {
    self.preferences.put(windows::NAME, &self.open_windows());
    self.preferences.save(storage);
}
```

```rust
let windows: OpenWindows = preferences::get(ctx, NAME).unwrap_or_default();
```

Writing them only on save, rather than at every click on a window
button, keeps the many places that open and close windows free of that
chore. `the_app_first_opens_on_the_chord_sheet_then_with_the_windows_left_open`
checks both runs: the first opens on the sheet, and after closing it
with F5 and opening the mixer with F9, the next start finds the mixer
open and the sheet closed.

### Windows the size of what they show

egui remembers each window's size, and saves it with the rest of its
memory when the app closes. That is right for a window you work in,
like the piano roll: you made it big because you wanted it big. It is
wrong for a window that is as big as what it shows: drag the mixer wide
once, and it would open across the whole playlist ever after, with its
channels in one corner. So there are two kinds of window:

```rust
pub enum Opening {
    /// Each time it opens: a window as big as what it shows.
    EveryTime,
    /// The first time it opens after the app starts: a window to work
    /// in, which keeps the size you give it while the app runs.
    FirstTime,
}
```

The mixer, the shortcuts list, ear training, the twelve-tone row, the
pitch-class sets and the micromodes open **every time** at the size of
their contents, up to the height of the playlist, and scroll past that.
The piano roll, the chord sheet, the lead sheet and Settings open at
their own size the **first time** after the app starts, and keep a size
you give them while it runs.

`widgets::open_at` does it. It finds out whether the window is opening
now (it was not drawn in the last frame), and if so gives it a fixed
size for that one frame:

```rust
let first = ctx.data_mut(|data| {
    !std::mem::replace(data.get_temp_mut_or(id.with("has-opened"), false), true)
});
if opens(ctx, id) && (first || opening == Opening::EveryTime) {
    ctx.request_discard("a window opens at its own size");
    window.fixed_size(size)
} else {
    window
}
```

Three things in it are worth a look:

- `std::mem::replace(place, new)` puts `new` in `place` and returns what
  was there. Here it reads "had this window opened before?" and marks
  it as opened, in one step. The flag is stored with `get_temp_mut_or`,
  which egui does not save, so it is `false` again each time the app
  starts.
- egui lays a window out as it draws it, so on its first frame the
  window is drawn before anything knows how big its contents are.
  `request_discard` asks egui to throw that frame away and draw it
  again; you only see the second one, already at the right size.
- The "size" given is a narrow width and the greatest height. egui grows
  a window to fit wider contents but never makes it taller than they
  are, so the window ends up exactly as big as what it shows. For that
  to work, nothing in it may fill all the room it is given, such as a
  scroll area that does not shrink to its contents.

A window whose contents change while it is open, like the mixer when a
channel is added or the rack when another pattern has fewer channels, is
fitted again with `fit_when_changed`. It hashes a summary of the
contents (each channel's name and instrument, say) and fits the window
again when the hash changes.

## Clicks that do what you meant

Two helpers in `widgets.rs` make the grids react to the click you meant.

**A quick tap.** On a touchpad, a tap can press and release the button
within one frame. egui's `press_origin()` is forgotten as soon as the
button is up, so on that frame it is `None`. `press_pos` reads the press
from the frame's list of events instead, which also says *where* the
button went down, not where the pointer is at the end of the frame:

```rust
pub fn press_pos(
    input: &egui::InputState,
    only: Option<egui::PointerButton>,
) -> Option<egui::Pos2> {
    input.events.iter().find_map(|event| match *event {
        egui::Event::PointerButton { pos, button, pressed: true, .. }
            if only.is_none_or(|only| only == button) => Some(pos),
        _ => None,
    })
}
```

`find_map` walks the events until the closure returns `Some`, and
returns that. `Option::is_none_or` reads well here: "no button was asked
for, or it is this one".

**A click that closes a menu.** Open a drop-down list, then click on the
grid to close it: egui closes the menu, but the click also reaches the
grid, which would draw a note, place a clip or turn on a step. In most
programs that click only closes the menu. So at the start of each frame,
before anything is drawn and while a menu that will close is still open,
the app remembers whether the button now down went down while a menu was
open:

```rust
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
```

The grids ask `dismissing(ctx)` and ignore the press, the drag and the
release that belong to it. The value lives in egui's temporary memory,
a handy place for a little state shared between widgets that do not
know each other.

## Remembering between runs

egui keeps some state of its own between frames in its *memory*: the zoom
factor, where each window is, how wide a resizable panel is. eframe can
save that memory when the app closes and load it when it starts. It does
so only with its `persistence` feature, which is off by default because
it brings in `serde` and `ron`. Turning it on is one line in the
workspace's `Cargo.toml`:

```toml
eframe = { version = "0.37", features = ["persistence"] }
```

With it, eframe also remembers the size and position of the main window.
It writes a file called `app.ron` in the place each system keeps app data
(on Linux, `~/.local/share/ferrouslineup/`), every 30 seconds and when the
window closes.

The app keeps most of its own lasting choices in the same memory, with
`insert_persisted` instead of `insert_temp`: the area zooms, the
settings, the recent files, the library's filters. Each module stores
its own under an `Id` of its own, and eframe saves them all together.
Anything kept with `insert_temp` is forgotten when the app closes.

### Kept by name, not by type

egui finds a value in its memory by two things: the `Id` it was stored
under and its Rust type, through the type's `TypeId`. A `TypeId` is a
number the compiler works out from the type's path, its fields and the
crate it is in, and the crate's part changes with the crate's version
and with the version of the compiler. So after an update of the app, or
a build with a newer Rust, or a field added to the type, `get_persisted`
looks under a new number and finds nothing: the setting is back to its
default, and the old value sits unread in `app.ron`. The SoundFont
showed it: with the same `app.ron`, version 0.4.0 played from
GeneralUser GS and a build numbered 0.4.1 had it off.

eframe has a storage of its own, the same `app.ron`, which finds values
by a name: a `&str` that stays the same whatever the version. eframe
hands it to the app only twice: as `cc.storage` when the app starts, and
to `App::save`, every 30 seconds and when the window closes. The module
`app/preferences.rs` bridges the two. It keeps every preference by name,
written as RON text, in one map:

```rust
/// Each preference by name, written as RON. egui's context holds one
/// handle to the map and the app another, to save it.
#[derive(Clone, Default)]
pub(super) struct Preferences(Arc<Mutex<BTreeMap<String, String>>>);
```

A handle to the map lives in egui's temporary memory, so any code with
the `Context` reaches it, as with any other value kept there:

```rust
/// The preference kept as `name`, if there is one and it can be read.
pub(super) fn get<T: DeserializeOwned>(ctx: &egui::Context, name: &str) -> Option<T> {
    let text = Preferences::of(ctx).map().get(name).cloned()?;
    ron::from_str(&text).ok()
}

/// Keeps `value` as the preference `name`.
pub(super) fn set<T: Serialize>(ctx: &egui::Context, name: &str, value: &T) {
    if let Ok(text) = ron::to_string(value) {
        Preferences::of(ctx).map().insert(name.to_string(), text);
    }
}
```

The app reads the map from eframe's storage before anything else is
made,

```rust
if let Some(storage) = cc.storage {
    preferences::Preferences::of(&cc.egui_ctx).load(storage);
}
```

keeps another handle to it, and gives it back to eframe to write:

```rust
fn save(&mut self, storage: &mut dyn eframe::Storage) {
    self.preferences.save(storage);
}
```

The `Arc<Mutex<…>>` lets both handles share one map. The lock is held
only while a line is read or written, and only the window's thread
takes it. Because the values are RON text until they are asked for, a
type can change its fields and still read what an older version kept,
with `#[serde(default)]` for fields that were not there yet.

The sound settings ([SoundFonts](../sound/soundfonts.md)) are kept this
way, as `"sound-font"`, and so are the open windows, as
`"open-windows"`. The other choices above are still in egui's
memory, so an update can forget them; moving one over is a matter of
calling `preferences::get` and `preferences::set` with a name instead
of `get_persisted` and `insert_persisted` with an `Id`.

### Cargo features

A *feature* is a switch a crate offers to turn on optional code. Inside
eframe the code that saves memory sits behind `#[cfg(feature =
"persistence")]`, so without the feature it is not even compiled. Turning
on a feature in one crate can turn on features in others: eframe's
`persistence` turns on egui's, which adds `serde` derives to the types
egui keeps in its memory. You can see what a crate offers in the
`[features]` table of its `Cargo.toml`, or on its page on docs.rs.

## Pictures painted, not typed

The toolbar's five window buttons and the count-in and metronome
toggles are square buttons with a picture; the name, what the window
shows and its key appear when the mouse is over one. An emoji would be
the easy way to draw such a picture, but an emoji is a character, drawn
by a font, and some systems have no font with those characters: there
the buttons would show empty boxes. So the pictures in `icons.rs` are
**painted**, with lines, rectangles and polygons, the same way on every
system.

Each picture is a plain function with the same signature, and Rust can
name that signature as a type:

```rust
/// A picture: drawn by a function of the painter, the square it fills
/// and the colour to draw it in.
pub type Picture = fn(&Painter, Rect, Color32);
```

`fn(&Painter, Rect, Color32)` is a **function pointer** type. Any
function with that signature can be stored in a variable of type
`Picture`, put in an array, or passed to another function, without a
`Box` or a generic.

Each picture is drawn in a square, and its points are given as fractions
of the square, from 0 to 1, so the same picture can be drawn at any
size:

```rust
/// The point `x`, `y` of the way across and down `rect`, each from 0 to 1.
fn at(rect: Rect, x: f32, y: f32) -> Pos2 {
    rect.min + vec2(x * rect.width(), y * rect.height())
}

/// The mixer: three faders, each at its own level.
pub fn mixer(painter: &Painter, rect: Rect, color: Color32) {
    for (x, level) in [(0.2, 0.62), (0.5, 0.3), (0.8, 0.5)] {
        line(painter, rect, (x, 0.0), (x, 1.0), color.gamma_multiply(0.6));
        let knob = Rect::from_center_size(at(rect, x, level), vec2(6.0, 3.5));
        painter.rect_filled(knob, 1.0, color);
    }
}
```

The chord sheet is a page with chord names two to a line, the rack three
rows of steps with some lit, the piano roll a keyboard with three notes,
the harmony keys a keyboard with two black keys, the metronome its body
and arm.

The toolbar draws its five window buttons from an array. Each entry holds
the name, the `bool` that says whether that window is open, the picture
and the hint:

```rust
let windows: [(&str, &mut bool, icons::Picture, &str); 5] = [
    ("Chord sheet", &mut self.chord_sheet.open, icons::chord_sheet,
     "the song's harmony bar by bar, to read and write (F5)"),
    ("Channel rack", &mut self.rack_open, icons::rack,
     "the steps of the selected pattern (F6)"),
    // … the piano roll, the harmony keys, the mixer
];
for (name, open, picture, about) in windows {
    let button = icons::toggle(ui, name, *open, picture);
    // ...the hint, and a click opens or closes the window...
}
```

There are five `&mut` borrows of `self` in one array. The borrow checker
allows it because each one borrows a **different field**:
`self.chord_sheet.open`, `self.rack_open`, `self.piano_roll.open` and so
on. Two `&mut` to the same field would not compile.

The test `every_picture_stays_in_its_square` paints each picture into a
16-point square and checks that no shape goes outside it, and
`the_window_buttons_open_and_close_their_windows` clicks each of the
five and checks that its window opens, that the button lights, and that
a second click closes it.

## Every key in a list

**Help > Keyboard shortcuts**, or **F1**, opens a window with every key
the app answers to, section by section.

![Help > Keyboard shortcuts: each section says when its keys
work.](../images/step-9-workflow-shortcuts.png)

The list is data: a `Vec` of sections, each with a title, a note on when
its keys work and its lines. A line's keys are one of four kinds:

```rust
enum Keys {
    /// Any of these does it.
    Any(Vec<KeyboardShortcut>),
    /// The first does the first half of what the line says, the second the
    /// second: "Up / down".
    Pair(KeyboardShortcut, KeyboardShortcut),
    /// The mouse wheel, turned while holding these.
    Wheel(Modifiers),
    /// Keys that are not one shortcut, written out.
    Written(&'static str),
}
```

Wherever the app has a constant for a shortcut, the list uses that same
constant: `files::SAVE`, `edit::UNDO`, `piano_roll::OCTAVE_UP`,
`view::MIXER`. So if a key changes, the list changes with it. Turning a
`Keys` into text uses egui's `format_shortcut`, which writes
"Ctrl+Shift+Z" on Linux and Windows and "⇧⌘Z" on a Mac, the way the
menus write them.

Because the list is data, it can also be tested as data.
`no_two_global_shortcuts_share_a_key` collects the keys of the sections
that work anywhere and fails if any key is listed twice, since that key
would do two things at once.

## Help: the manual and About

**Help > User manual** opens the user manual's web pages in the app's
language. The address lives in a function of `about.rs`, so the Help
menu and the About window both ask it:

```rust
pub(super) fn manual() -> &'static str {
    match i18n::current_language() {
        Language::English => MANUAL,
        Language::Spanish => MANUAL_ES,
    }
}
```

The `match` has no `_` arm on purpose. Add a third language, and the
compiler stops here until you decide which manual its readers get.
`pub(super)` makes `manual` visible to `app.rs` and to the modules
inside it, such as `shortcuts.rs`, where the Help menu is, and to
nothing else.

In the menu, the button asks egui to open that address in a new tab of
the web browser:

```rust
if ui.button(crate::i18n::tr("User manual")).clicked() {
    ui.ctx()
        .open_url(egui::OpenUrl::new_tab(super::about::manual()));
    ui.close();
}
```

**Help > About Ferrous Lineup** says which version you are running,
under which license, and where to find the user manual, this book (each
also as a PDF), the code and the list of known problems. It is the
first thing to look at before reporting one.

![Help > About: the version, the license and the links.](../images/step-9-about.png)

Everything it says about the program comes from `Cargo.toml`, read
while the program is compiled:

```rust
const ISSUES: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues");

ui.label(format!("{} {}", tr("Version"), env!("CARGO_PKG_VERSION")));
ui.weak(tr(env!("CARGO_PKG_DESCRIPTION")));
```

- `env!("NAME")` is a macro that the compiler replaces with the value
  of the environment variable `NAME` *while it compiles*, as a
  `&'static str`. Cargo sets `CARGO_PKG_VERSION`,
  `CARGO_PKG_DESCRIPTION`, `CARGO_PKG_REPOSITORY` and a few more from
  the `[package]` section of `Cargo.toml` (the app takes its version,
  license and repository from the workspace's `[workspace.package]`).
  A variable that is not set is a compile error, not a surprise for the
  user.
- `concat!` joins literals into one at compile time, so `ISSUES` is a
  constant too. `format!` could not be used there: it builds a `String`
  while the program runs, and a `const` must be known before it runs.

So when a release changes the version in `Cargo.toml`, the window
changes with it, and nobody has to remember a second place. The
[release builds](../shipping/releases.md) compare the tag with this
same number.

The picture is the one the desktop shows for the app's window:
`WINDOW_ICON`, 256 × 256 pixels of red, green, blue and alpha bytes,
built into the program with `include_bytes!`. egui draws pictures from
**textures**, images kept on the graphics card, so the bytes are made
into a texture the first time the window opens and kept in egui's
memory:

```rust
fn icon(ctx: &egui::Context) -> egui::TextureHandle {
    let id = egui::Id::unique("about-icon");
    if let Some(texture) = ctx.data(|data| data.get_temp(id)) {
        return texture;
    }
    let size = crate::WINDOW_ICON_SIZE as usize;
    let image = egui::ColorImage::from_rgba_unmultiplied([size, size], crate::WINDOW_ICON);
    let texture = ctx.load_texture("about-icon", image, egui::TextureOptions::LINEAR);
    ctx.data_mut(|data| data.insert_temp(id, texture.clone()));
    texture
}
```

A `TextureHandle` works like an `Arc`: a clone is cheap, and the
texture stays on the graphics card while any clone of it is alive. The
one kept in egui's memory means the picture is sent to the card once,
not on every frame. `WINDOW_ICON` is a constant of `main.rs`, not
marked `pub`; `about.rs` can still read it as `crate::WINDOW_ICON`,
because a private item is visible to its own module and to every module
inside it.

`ui.hyperlink_to(text, url)` draws a link. Clicking it does not start
the browser: egui adds an `OutputCommand::OpenUrl` to what the frame
gives back, and eframe asks the system to open the address. That makes
the click easy to test.
`help_about_shows_the_version_the_license_and_where_the_book_is` opens
**Help > About**, checks that the version line and every link are
drawn, clicks **The book**, and checks that the frame asked for the
book's address, without any browser opening.
`help_user_manual_opens_the_manual_in_the_apps_language` does the same
for **Help > User manual**, in English and in Spanish.

## Testing a window without a screen

All the tests named in this chapter draw the app without a screen.
`egui::Context::run_ui` (or `run_pass`) takes the input of one frame,
the mouse position, the keys and the clicks, and returns everything egui
would hand to the graphics card, without drawing it. A test can then
find a widget by its `Id` or its text, check where it was placed, click
it in the next frame, and read the app's state. `app/ux_tests.rs` holds
most of them; [How the code is tested](../shipping/testing.md) has the
rest of the story.

One detail of these tests is worth knowing. What `run_ui` returns
includes the font's texture. egui checks, only in debug builds, that
such a texture is not dropped without being used, and panics if it is.
So tests that keep the output clear it first:

```rust
// The font texture is not drawn anywhere here.
output.textures_delta.clear();
```

A test that passes in a release build and fails in a debug build is
usually one of these checks: `debug_assert!` is compiled only into
debug builds, and a library can have many of them. That is why CI runs
the tests in a debug build.
