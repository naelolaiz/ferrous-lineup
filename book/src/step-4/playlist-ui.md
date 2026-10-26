# The playlist window

Files: [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)
and [`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)

## A new layout

Pattern-based studios put the song at the centre: the playlist fills the
main window, and the channel rack and piano roll float on top of it,
coming and going with F6 and F7. Ferrous Lineup now does the same. The
central panel shows the playlist, and the rack moved into an
`egui::Window`, like the piano roll in step 3:

```rust
let rack_actions = egui::Window::new(format!("Channel rack · {pattern_name}"))
    .id(egui::Id::unique("channel-rack"))
    .open(&mut rack_open)
    .default_pos([170.0, 380.0])
    .resizable(false)
    .show(&ctx, |ui| {
        channel_rack::show(ui, &self.project, self.pattern, playhead)
    })
    .and_then(|window| window.inner)
    .unwrap_or_default();
```

`Window::show` returns `None` when the window is closed, and its `inner`
is `None` when the window is folded up, so the actions are wrapped twice.
`.and_then(|window| window.inner)` unwraps the outer `Option` and returns
the inner one; `.unwrap_or_default()` turns `None` into an empty `Vec`.

A window that is not resizable shrinks to fit its contents, but a scroll
area inside it does not know how wide its contents want to be until it
has drawn them. The first screenshot showed a rack five steps wide. The
fix tells the scroll area the width of one row up front:

```rust
let row_width = controls + length as f32 * (widgets::STEP_SIZE.x + gap);
egui::ScrollArea::both()
    .min_scrolled_width(row_width.min(room.x))
```

## The selected pattern

The app remembers which pattern is selected:

```rust
pub struct LineupApp {
    // ...
    /// The selected pattern: the one the channel rack and the piano roll
    /// show, pattern mode plays, and new playlist clips use.
    pattern: usize,
    mode: PlayMode,
    // ...
}
```

It is a field of the app, not of the project: it is *where you are
looking*, not part of the song, so it is not saved and selecting a
pattern does not count as an unsaved change. The rack and the piano roll
take it as an argument (`channel_rack::show(ui, &self.project,
self.pattern, playhead)`), so they stay functions of what they are given.

The playhead they get depends on the mode. In pattern mode the audio
thread's step *is* a step of the selected pattern. In song mode it is a
step of the song, and the app asks the project where the selected
pattern is at that moment:

```rust
fn pattern_playhead(&self) -> Option<usize> {
    let step = self.playhead()?;
    match self.mode {
        PlayMode::Pattern => Some(step),
        PlayMode::Song => self.project.pattern_step_in_song(self.pattern, step),
    }
}
```

So while the song plays, the rack's orange playhead runs whenever a clip
of the selected pattern is playing, and disappears in the bars where it
is not, as in the screenshot at the start of this step.

## Drawing a big grid: `show_viewport`

The playlist can be far bigger than the window: 16 tracks of 36 pixels,
and at least 32 bars of 72 pixels, over 2300 pixels wide. It goes in a
scroll area. But two parts must *not* scroll away: the track names on
the left, and the bar numbers at the top.

`ScrollArea::show_viewport` makes this possible. Its closure receives the
visible part of the content, as a rectangle measured from the content's
top-left corner:

```rust
egui::ScrollArea::both()
    .auto_shrink(false)
    .show_viewport(ui, |ui, viewport| {
        self.grid(ui, viewport, project, current, playhead, &mut actions)
    });
```

`grid` allocates the full size of the content, then works out where the
visible part is on screen:

```rust
let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());
let visible = viewport.translate(rect.min.to_vec2());
```

`rect` is the whole content in screen coordinates (its top-left corner
moves up and left as you scroll). Adding that corner to `viewport` gives
`visible`, the part of the screen where the content shows. Then:

- the tracks and clips are drawn at their place in the content, so they
  scroll;
- the track names are drawn at `visible.left()`, and the ruler at
  `visible.top()`, so they stay put while the clips slide underneath.

They are drawn *after* the clips, so they cover them. The corner where
they meet is drawn last of all.

The arithmetic for steps, tracks and pixels lives in a small `Copy`
struct, as in the piano roll:

```rust
#[derive(Debug, Clone, Copy)]
struct Layout {
    /// The top-left corner of step 0 of track 0.
    origin: Pos2,
    tracks: usize,
}

impl Layout {
    fn x(self, step: usize) -> f32 {
        self.origin.x + step as f32 * STEP_WIDTH
    }

    fn track_top(self, track: usize) -> f32 {
        self.origin.y + track as f32 * TRACK_HEIGHT
    }
    // ... clip_rect, cell_at
}
```

## Real widgets in a painted area: `ui.put`

Everything in the grid is painted by hand, but the track names should be
editable text fields and the mute lights should be the same light as in
the rack. `ui.put` places any widget in a rectangle you choose:

```rust
let light_response = ui
    .put(light, |ui: &mut Ui| widgets::mute_light(ui, enabled))
    .on_hover_text("Click to mute or unmute the track");

let mut name = track.name.clone();
let field = egui::TextEdit::singleline(&mut name)
    .frame(egui::Frame::NONE)
    .text_color(HEADER_TEXT);
if ui.put(name_rect, field).changed() {
    actions.push(PlaylistAction::RenameTrack { track: index, name });
}
```

- A closure `|ui: &mut Ui| -> Response` counts as a widget: egui
  implements its `Widget` trait for any such closure. That is how a
  function-style widget like `mute_light` can be passed to `put`.
- The text field edits a *copy* of the name. If it changed, the copy goes
  into a `RenameTrack` action, and the app changes the project. The
  playlist keeps the rule that it never changes the project itself.
- `Frame::NONE` removes the field's border, so it looks like a label until
  you click it.

## Mouse input, and a bug found by a robot

Placing, moving and deleting clips follows the piano roll's pattern: a
press decides what the drag is, and the drag follows the pointer until
the button is released.

```rust
struct ClipDrag {
    index: usize,
    grab: usize,
}
```

Testing with `xdotool` (a program that moves the mouse for you) found a
bug. A script pressed on a clip and moved down one track within a few
milliseconds, and the playlist *added a new clip* below instead of moving
the old one. egui reports the pointer's position at the time the frame is
drawn. When the press and the movement arrive in the same frame, "where
the pointer is now" is already one track lower, over empty space.

The fix is to handle a press where the button went down, which egui
remembers as the **press origin**:

```rust
// A press is handled where the button went down. A quick hand can
// press and move in the same frame, and the clip or empty cell
// under the pointer *now* may not be the one that was clicked.
let pressed_at = press_origin
    .filter(|pos| primary_pressed && response.hovered() && cells.contains(*pos));
if let Some(pos) = pressed_at {
    let (track, step) = layout.cell_at(pos);
    match clip_at(project, layout, pos) {
        Some(index) => { /* start moving that clip, and select its pattern */ }
        None => { /* add a clip of the selected pattern, and start moving it */ }
    }
}
```

`Option::filter` keeps the value only if the closure returns `true`, so
`pressed_at` is `Some` only on the frame of a press, over the grid. The
piano roll had the same bug and got the same fix. A person rarely moves
that fast, but "rarely" is not "never", and on a slow computer a frame
lasts longer.

Positions snap to the grid you choose in the toolbar:

```rust
fn floor(self, step: usize) -> usize {
    step / self.steps() * self.steps()
}
```

Integer division throws the remainder away, so `step / 16 * 16` is "the
bar line at or before this step". With Snap set to Beat it uses 4, and
with Step it uses 1 (no snapping).

## Drawing a clip's notes: `Box<dyn Fn>`

Each clip shows a sketch of its pattern. Drum patterns look best as a
tiny step sequencer (one line per channel), anything with melody as a
tiny piano roll (higher notes higher). The two need different ways to
turn a note into a height, chosen once per clip:

```rust
let height: Box<dyn Fn(usize, u8) -> f32> = if steps_only {
    let used: Vec<usize> = (0..channels)
        .filter(|&channel| !pattern.notes(channel).is_empty())
        .collect();
    let count = used.len().max(1) as f32;
    Box::new(move |channel, _| {
        let row = used.iter().position(|&c| c == channel).unwrap_or(0) as f32;
        1.0 - (row + 0.5) / count
    })
} else {
    // ... lowest and highest note, as in the rack's mini piano roll
    Box::new(move |_, midi| ((midi - lowest) as f32 + centering) / span)
};
```

Every closure has its own, unnamed type, so the two branches of the `if`
produce values of different types, and a variable can only have one type.
`Box<dyn Fn(usize, u8) -> f32>` is the common type: "a closure on the
heap that takes a channel and a pitch and returns a height". `dyn Fn` is
a trait object, like `dyn Error` in the saving chapter. The `move`
keyword makes each closure take ownership of what it uses (`used`,
`lowest`...), which must outlive the `if` that created them.

This runs on the UI thread, once per clip per frame, so the small
allocation for the `Box` does not matter. On the audio thread it would.

## Keys that reach the wrong widget

Space started and stopped the music in step 3 by checking whether any
widget had keyboard focus:

```rust
let typing = ctx.memory(|memory| memory.focused().is_some());
```

With buttons everywhere, that broke. Clicking ▶ Play gives the button
focus, and egui lets a focused button be pressed with Space. So Space did
two things at once: the shortcut toggled playback, and the button toggled
it back. The new version asks a narrower question, and takes the key
away from the widgets once it has used it:

```rust
let typing = ctx.text_edit_focused();
let pressed =
    |key| ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
if !typing && pressed(egui::Key::Space) {
    self.set_playing(!self.playing);
}
```

The shortcuts run before any panel is drawn, so once `consume_key` has
taken the Space press, the focused button never sees it.

One more surprise: the ⋯ pattern menu has a text field for the pattern's
name, and egui menus close on any click, including a click *inside* the
menu on that field. The menu now uses a different close behaviour:

```rust
let stay_open = egui::containers::menu::MenuConfig::new()
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
let menu = egui::containers::menu::MenuButton::new("⋯").config(stay_open);
```

Both bugs were found the same way: by driving the real program with
scripted clicks and keys, and looking at screenshots. Unit tests check
the music; for the window, nothing replaces using it.
