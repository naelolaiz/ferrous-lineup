# The channel rack UI

Files: [`channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs),
[`widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs) and
[`app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)
in `crates/lineup-app/src/`

## Writing your own widgets

egui has buttons and sliders, but not step buttons or knobs. Making one
takes three moves: reserve space, read the mouse, paint.

```rust
pub fn step_button(ui: &mut Ui, on: bool, beat_group: usize, is_playhead: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(STEP_SIZE, Sense::click());

    if ui.is_rect_visible(rect) {
        let mut fill = if on { STEP_ON } else { STEP_OFF[beat_group % 2] };
        if response.hovered() {
            fill = fill.gamma_multiply(1.25);
        }
        let painter = ui.painter();
        painter.rect_filled(rect.shrink(1.0), 3.0, fill);
        if is_playhead {
            painter.rect_stroke(rect.shrink(1.0), 3.0, Stroke::new(2.0, PLAYHEAD), StrokeKind::Inside);
        }
    }
    response
}
```

- `allocate_exact_size` claims a rectangle in the layout and says which
  interactions we want (`Sense::click()`). It returns the rectangle and a
  `Response` describing what the mouse did there this frame.
- `is_rect_visible` skips painting rows that are scrolled out of view.
- The `Painter` draws shapes. Colors are `Color32` constants defined at
  the top of the file, so the palette lives in one place.
- Returning the `Response` lets the caller use it like any built-in
  widget: `if response.clicked() { ... }`.

The knob uses `Sense::click_and_drag()`, turns vertical mouse movement
into a value change, and calls `response.mark_changed()` so callers can
use `.changed()` as they would on a slider. Its pointer is drawn with a
little trigonometry: the angle sweeps 270 degrees, and `(sin, -cos)` of
the angle gives the direction (screen *y* grows downward, hence the minus).

## Drawing returns actions

The rack's `show` function never changes the project. It only looks at
it and returns what the user did:

```rust
pub enum RackAction {
    ToggleStep { channel: usize, step: usize },
    ClearStep { channel: usize, step: usize },
    Preview(usize),
    SetVolume { channel: usize, volume: f32 },
    ToggleMute(usize),
    Remove(usize),
    AddDrum(Drum),
    AddSynth,
    LoadSample { channel: Option<usize>, path: PathBuf },
}

pub fn show(ui: &mut Ui, project: &Project, playhead: Option<usize>) -> Vec<RackAction>
```

`project: &Project` is a **shared borrow**: the function may read the
project but cannot change it, and the compiler enforces that. The app
then applies the actions:

```rust
let actions = egui::CentralPanel::default_margins()
    .show(ui, |ui| channel_rack::show(ui, &self.project, playhead))
    .inner;
for action in actions {
    self.apply(action);
}
```

This split also solves a borrowing puzzle. While the rack is drawing, it
holds a shared borrow of the project, so it could not also change it
(that would need a `&mut` at the same time, which the borrow checker
forbids). Collecting actions first and applying them after the drawing
is finished keeps the two borrows apart in time.

`apply` is one `match` with an arm per action. Each arm updates the
`Project` *and* sends the matching `Command`, so the UI's project and
the audio thread's copy can never disagree:

```rust
RackAction::ToggleStep { channel, step } => {
    if let Some(on) = self.project.toggle_step(channel, step) {
        self.send(Command::SetStep { channel, step, on });
    }
}
```

## Menus

```rust
name.context_menu(|ui| {
    if ui.button("Remove channel").clicked() {
        actions.push(RackAction::Remove(index));
        ui.close();
    }
});
```

`context_menu` opens a menu on right-click; `menu_button` opens one on a
normal click (the "Add channel" button). Both take a closure that draws
the menu's contents, and `ui.close()` closes the menu after a choice.
`add_enabled_ui(false, ...)` greys out everything inside, which is how
"Add channel" is disabled when the rack is full.

## Drag and drop

egui reports files being dragged over the window in
`input.raw.hovered_files`, and files dropped in `input.raw.dropped_files`.
While drawing, the rack stores each row's rectangle; afterwards it finds
the row under the mouse:

```rust
let target_row = pointer.and_then(|p| row_rects.iter().position(|rect| rect.contains(p)));
```

`position` returns the index of the first item for which the closure is
true, or `None`. A file dropped on a row replaces that channel's sound; a
file dropped anywhere else becomes a new channel named after the file
(`path.file_stem()` is the file name without its extension).

Loading happens in `LineupApp::load_sample`, on the UI thread, because
reading files is slow and must never happen on the audio thread. A
broken file leaves the channel as it was and puts the reason in the
status bar.

## Keeping the playhead moving

egui only redraws when something happens, such as a mouse move. While
the pattern plays, the playhead changes on its own, so the app asks for
continuous redraws:

```rust
if self.playing {
    ui.ctx().request_repaint();
}
```
