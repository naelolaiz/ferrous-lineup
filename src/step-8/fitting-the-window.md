# Fitting any window: wrapping, zoom and remembering

Files: [`crates/loom-app/src/app/view.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/app/view.rs),
[`crates/loom-app/src/app.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/app.rs),
[`crates/loom-app/src/harmony_keys.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/harmony_keys.rs)
and [`Cargo.toml`](https://github.com/naelolaiz/ferrous-loom/blob/main/Cargo.toml)

By the end of step 8 the toolbar along the top held three menus, the
transport, the tempo, the pattern selector, three window buttons and the
master volume. On a 1280-pixel-wide window at 100% it just fitted. On a
smaller window, or a screen that draws everything one and a half times
bigger, the end of it ran off the right edge, and there was no way to
reach the master volume. This page is about the small fix for that, and
about zoom, which egui had all along but nobody could find.

![At 150% zoom, the toolbar takes two rows, the little piano moves under
the key settings, and the zoom control sits at the right end of the
status bar. The channel rack stays over the playlist.](../images/step-8-zoom.png)

## Rows that wrap

egui lays widgets out one after another. `ui.horizontal` puts them in a
row and keeps going past the edge if they do not fit: whatever is past
the edge is still there, just not drawn. `ui.horizontal_wrapped` is the
same row, except that a widget that would not fit starts a new row:

```rust
fn transport_bar(&mut self, ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        self.file_menu(ui);
        self.edit_menu(ui);
        self.view_menu(ui);
        self.midi_menu(ui);
        // ... the rest as before
    });
}
```

The top panel grows to fit the second row, and everything under it moves
down. The harmony keys bar got the same change, so on a narrow window
the little piano moves to a row of its own.

One widget needed more care. The master volume was a `Slider` with
`.text("Master")`, which draws the slider and its label together. The
wrapping row decides where to put the *slider* and then the label is
squeezed in after it, so the label could still be cut off. Now the label
is a widget of its own, placed before the slider, and the two wrap
together.

## Text that gets shorter

Wrapping suits buttons. For a long line of text, like the playlist's
help line or a status message, it is often better to cut it short. Every
text in egui has a *wrap mode*: `Wrap` breaks it onto more lines,
`Truncate` cuts it with "…", and `Extend` lets it run on. A single label
can ask for one:

```rust
ui.add(egui::Label::new(egui::RichText::new(&help).weak()).truncate());
```

and a whole area can set it for all its text through its style:

```rust
ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
```

A truncated label shows the whole text when you point at it, so nothing
is lost.

## Right to left

The status bar now has two parts: messages on the left, and the zoom
control on the right. If the messages were drawn first, a long one ("Audio
unavailable: could not read the device settings...") would take all the
room and push the zoom control out. So the bar is laid out *right to
left*: the zoom control goes in first, from the right edge, and the
messages get whatever is left:

```rust
ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
    view::zoom_control(ui);
    ui.separator();
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
        self.status_messages(ui);
    });
});
```

In a right-to-left layout the first widget is the rightmost one, which is
why `zoom_control` adds its **+** button before the percentage and the
**−** button.

## Zoom

egui draws everything in *points*, not pixels. How many pixels a point
takes is `pixels_per_point`: the screen's own scale (1.0, 1.5 or 2.0 on
most screens) times egui's *zoom factor*, which starts at 1.0. Change the
zoom factor and everything is drawn bigger or smaller, text, buttons and
the piano roll alike.

egui already changes it with **Ctrl+Plus**, **Ctrl+Minus** and **Ctrl+0**,
like a web browser. The functions behind those keys are public, in the
`egui::gui_zoom` module, so the View menu and the zoom control use the
very same ones:

```rust
if ui.add(small("+")).clicked() {
    gui_zoom::zoom_in(&ctx);
}
```

`gui_zoom::zoom_menu_buttons(ui)` even draws the three menu entries with
their shortcuts written beside them, which is all the View menu needs.
The zoom control's list of sizes calls `ctx.set_zoom_factor(1.25)` and
so on.

![The View menu: the zoom, and every window and panel with its
key](../images/step-8-view-menu.png)

## Windows that stay out of the way

The channel rack and the piano roll are `egui::Window`s, which float
over everything. When the toolbar took two rows, or the harmony bar grew
taller, a window could cover them. `Window::constrain_to(rect)` keeps a
window inside a rectangle, so the app takes the rectangle of the central
panel, where the playlist is, and gives it to both windows:

```rust
let playlist = egui::CentralPanel::default_margins().show(ui, |ui| { /* ... */ });
let area = playlist.response.rect;
// ...
egui::Window::new("Channel rack").constrain_to(area) // ...
```

Every `show` in egui returns an `InnerResponse`: `.inner` is what the
closure returned (here the playlist's actions) and `.response` tells
where it was drawn.

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
(on Linux, `~/.local/share/ferrousloom/`), every 30 seconds and when the
window closes.

Only egui's own memory is saved. The app's state, such as whether the
channel rack is open or which hint level is chosen, is not: that needs
`App::save`, which is exercise 7.

## Cargo features

A *feature* is a switch a crate offers to turn on optional code. Inside
eframe the code that saves memory sits behind `#[cfg(feature =
"persistence")]`, so without the feature it is not even compiled. Turning
on a feature in one crate can turn on features in others: eframe's
`persistence` turns on egui's, which adds `serde` derives to the types
egui keeps in its memory. You can see what a crate offers in the
`[features]` table of its `Cargo.toml`, or on its page on docs.rs.

## The View menu and the panel

The View menu also lists the channel rack (F6), the piano roll (F7), the
harmony keys (F8) and the Patterns and Library panel (**Alt+F8**, the key
other pattern-based studios use for their browser). Hiding the panel
gives the playlist more room. The panel is shown with
`Panel::show_collapsible`, which takes a `&mut bool` and slides the panel
in and out.

The shortcuts are now `KeyboardShortcut` constants in `view.rs`, used both
to check the keys and to write them in the menu, so the two cannot
disagree. One catch: `consume_shortcut` lets *extra* Alt or Shift keys
through (it matches "logically", so that Ctrl+Plus works on keyboards
where + needs Shift). F8 would therefore also match Alt+F8, so
`view_shortcuts` checks Alt+F8 first and only looks for F8 when it was
not pressed.

## When the groups stopped wrapping

The new look (see [A clearer look, a real transport, and editing
tracks](look-and-transport.md)) put the toolbar's buttons in groups, each
inside a dark frame. After that, a zoomed-in window cut off the window
buttons and the volume again, instead of moving them to a second row.

The reason is how `horizontal_wrapped` decides to wrap. Before it places
a widget, it asks how big the widget wants to be; if that is more than
what is left of the row, it starts a new row. A frame cannot answer that
question. `Frame::show` draws its contents into whatever room is left,
and only knows its size afterwards. So the layout never got the chance
to move it down.

The fix, `wrapping` in
[`crates/loom-app/src/theme.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/theme.rs),
uses the frame before. egui redraws the whole window many times a
second, so last time's width is a very good guess:

```rust
let id = ui.next_auto_id().with("wrapping");
let last_width = ui.ctx().data(|data| data.get_temp::<f32>(id));
let row_started = ui.cursor().min.x > ui.max_rect().min.x;
if let Some(width) = last_width
    && ui.layout().main_wrap
    && row_started
    && width > ui.available_size_before_wrap().x
{
    ui.end_row();
}
let response = add(ui);
ui.ctx().data_mut(|data| data.insert_temp(id, response.response.rect.width()));
```

A few things to notice:

- **`ui.next_auto_id()`** is the id egui would give the next widget. It
  depends on how many widgets came before in this `Ui`, so it is the same
  every frame as long as the toolbar is drawn in the same order. That
  makes it a key for remembering something about one group.
- **`data` and `data_mut`** are egui's memory: a map from ids to values of
  any type. `get_temp::<f32>` asks for an `f32`; the `::<f32>` (the
  "turbofish") tells Rust which type to look for, because nothing else in
  the line says so.
- **The `if let ... && ...` chain** (a "let chain", stable since Rust
  1.88) only runs `end_row` when there is a width from last time, the
  layout wraps, the row already has something in it, and the group does
  not fit. The very first frame has no width yet, so the group may stick
  out for one frame, which nobody sees.
- **The function is generic over `R`**, whatever the group's contents
  return, and takes the drawing as a closure, so both `group` and
  `lcd_frame` wrap their frames in it without changing what they return.

## Every zoom stops at the same sizes

Ctrl and + or −, and Ctrl with the mouse wheel, zoom in steps of 10%
between 50% and 300%. The View menu and the + and − at the end of the
status bar used to call egui's own `gui_zoom::zoom_in` and `zoom_out`
instead, which go from 20% to 500%. So the buttons went further than the
keys, and the keys then jumped back into range.

Now there is one function every way of zooming calls:

```rust
fn step_zoom(self, ctx: &egui::Context, bigger: bool) {
    self.set_zoom(ctx, zoom_step(self.zoom(ctx), bigger));
}
```

and `set_zoom` clamps to `ZOOM_RANGE` (`0.5..=3.0`). The View menu draws
its own Zoom In, Zoom Out and Reset Zoom buttons, greyed out at the ends
of the range, with their shortcuts beside them. egui's ready-made menu
buttons hid the shortcuts, because the app turns off egui's own keyboard
zoom so that it can zoom each window separately. A test checks that the
sizes in the status bar's list are all inside the range too.
