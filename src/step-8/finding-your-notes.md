# Finding your notes: empty patterns, double-clicks and a zoom for each window

Files: [`crates/loom-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/piano_roll.rs),
[`crates/loom-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/channel_rack.rs),
[`crates/loom-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/playlist.rs),
[`crates/loom-app/src/app/view.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/app/view.rs)
and [`crates/loom-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/widgets.rs)

The first thing a new user did with the finished step 8 was press F7 on
the demo song. The piano roll opened on "Beat · Loom Osc" and showed an
empty grid. The song plays chords, the playlist shows them, so where were
they?

They were in another pattern. The piano roll, like the channel rack,
shows **one pattern**: the selected one. The app starts with "Beat"
selected, and in "Beat" the synth plays nothing. The chords are in
"Chords". Nothing was wrong with the data; the screen just did not say
what it was showing. This page is about three fixes for that, and about
the zoom, which the same user wanted for each window on its own.

## Saying "nothing here", and where to go instead

When the channel has no notes in the pattern, the piano roll now says so,
above the grid and in the middle of it, and offers buttons:

![The piano roll on "Beat": a coloured line says Loom Osc has no notes
there, with a button for "Chords", where it does, and one for the
channel rack, where the drums of "Beat" are edited.](../images/step-8-no-notes.png)

Finding the patterns where the channel plays is a filter over all of
them, keeping the index (to select it) and the name (to show it):

```rust
let elsewhere: Vec<(usize, &str)> = project
    .patterns()
    .iter()
    .enumerate()
    .filter(|&(index, other)| index != self.pattern && !other.notes(self.channel).is_empty())
    .map(|(index, other)| (index, other.name.as_str()))
    .collect();
```

The `&str` in the vector borrows from `project`. That is fine because the
vector lives only while this frame is drawn, and `project` outlives it.

A button for another pattern cannot select it directly: the selected
pattern belongs to the app, not to the piano roll. So it returns an
action, the same way every other change leaves the piano roll:

```rust
pub enum RollAction {
    // ...
    /// Show another pattern: one where the channel has notes.
    SelectPattern(usize),
    /// Open the channel rack, where drum steps are edited.
    OpenRack,
}
```

## Following the notes

Better than a message is not needing one. When the piano roll opens, or
the pattern changes, and the channel shown has no notes there, it moves
to the channel with the most melody or chord notes:

```rust
pub fn busiest_channel(pattern: &Pattern) -> Option<usize> {
    (0..pattern.rows())
        .map(|channel| {
            let melodic = pattern.notes(channel).iter().filter(|note| !note.is_step());
            (channel, melodic.count())
        })
        .filter(|&(_, count)| count > 0)
        .max_by_key(|&(channel, count)| (count, std::cmp::Reverse(channel)))
        .map(|(channel, _)| channel)
}
```

`max_by_key` returns the *last* of several equal maximums. Comparing the
tuple `(count, Reverse(channel))` breaks ties towards the lower channel
number instead: `std::cmp::Reverse` wraps a value and flips its ordering.
Drum steps are left out on purpose. In "Beat" no channel plays a melody,
so the piano roll stays on Loom Osc and shows the message; it does not
jump to the kick drum, whose "notes" are all the same plain step.

To notice the window *opening*, the piano roll remembers whether it was
open last frame. `open` is a public field that the toolbar button, F7 and
the View menu flip, so this is the one place that sees every way in:

```rust
let opened = self.open && !self.was_open;
// ... at the end of `show`:
self.was_open = open;
```

## Double-click a clip to edit it

In FL Studio you double-click a clip in the playlist to edit it. The
playlist now does the same. egui counts the clicks for us:

```rust
let double_clicked = ui.input(|input| {
    input
        .pointer
        .button_double_clicked(egui::PointerButton::Primary)
});
if double_clicked
    && response.hovered()
    && let Some(pos) = pointer.filter(|pos| cells.contains(*pos))
    && let Some(index) = clip_at(project, layout, pos)
{
    actions.push(PlaylistAction::EditClip(clips[index].pattern));
}
```

The app picks the editor: the piano roll, on the busiest channel, when the
pattern has melodies or chords; the channel rack when it only has drums.
Either way the window comes to the front with `Context::move_to_top`.
A window's layer is named by the id the window was given, which is why
the rack and the piano roll now get theirs from small functions
(`rack_id()` and `PianoRoll::id()`) instead of writing
`Id::unique("channel-rack")` in two places.

![Double-clicking a "Beat" clip opens the channel rack on "Beat", sized
to fit its rows.](../images/step-8-rack-double-click.png)

The channel rack learned the same lesson as the piano roll. Songs from
the library keep their drums in a pattern of their own, "A drums" next to
"A", so the rack for "A" showed drum rows with nothing in them. When a
pattern has no drum steps, the rack now names the patterns that do, as
buttons.

It also became resizable. It opens as big as its rows (`natural_size`
adds them up), scrolls when made smaller, and when every row fits, the
plain mouse wheel scrolls the steps sideways. egui has a style setting
for exactly that, `always_scroll_the_only_direction`, which works when
the scroll area allows only one direction:

```rust
ui.style_mut().always_scroll_the_only_direction = true;
let vertical = size.y > ui.available_height();
egui::ScrollArea::new([true, vertical])
```

## A zoom for each window

[Fitting any window](fitting-the-window.md) used egui's zoom, which makes
*everything* bigger. Zooming the piano roll to see its notes then also
zoomed the playlist, and the user asked for each window on its own, the
way a browser zooms one tab.

![The main window at 110%, the channel rack still at 100%: its names and
steps keep their size.](../images/step-8-zoom-each.png)

There are three areas with a zoom of their own, so they are an enum:

```rust
pub(crate) enum Area {
    Main,
    Rack,
    PianoRoll,
}
```

Which area a point is in comes from egui's layers. Each window is a
layer, and `Context::layer_id_at(pos)` names the top one under a point;
anything that is not one of our two windows is the main window. Ctrl and
the wheel zoom the area under the pointer; Ctrl and + or −, which have no
pointer position, zoom the area clicked last.

The main window keeps egui's zoom. The windows cannot opt out of it, so
they undo it: a window at 120% inside a main window at 110% draws its
contents `1.2 / 1.1` times bigger than usual. Drawing contents bigger
means two things. Text and spacing come from the `Style`, and a `Ui` can
change its own copy without touching anyone else's:

```rust
pub fn zoom_contents(ui: &mut egui::Ui, scale: f32) {
    let style = ui.style_mut();
    for font in style.text_styles.values_mut() {
        font.size *= scale;
    }
    style.spacing.item_spacing *= scale;
    style.spacing.interact_size *= scale;
    // ... and the other sizes in `Spacing`
}
```

Everything the windows paint themselves, the piano roll's rows, the
keyboard, the step buttons and knobs, used fixed sizes such as
`ROW_HEIGHT`. Those are now multiplied by the zoom wherever they are
used, so the grid grows with the text. The text is still drawn at its
real size, so it stays sharp; scaling the finished picture instead (egui
can do that per layer) would have blurred it.

### A bug that the tests found

The first try at keeping the same notes in the middle while zooming
scrolled to the bottom of the keyboard instead. A test that draws the
piano roll without a window, zooms it three times and compares the
pitch in the middle found it, and the cause was one line:

```rust
let row = ((pos.y - self.rect.top()) / ROW_HEIGHT * self.zoom)  // wrong
let row = ((pos.y - self.rect.top()) / (ROW_HEIGHT * self.zoom)) // right
```

`/` and `*` have the same precedence in Rust and group from the left, so
the first line divides by `ROW_HEIGHT` and then *multiplies* by the zoom.
At 100% both lines give the same answer, which is why nothing looked
wrong until the zoom changed. The same function also tells which note a
click lands on, so clicks on a zoomed grid would have been wrong too.

Drawing egui without a window is how the piano roll's tests work: make a
`Context`, and run frames with `run_pass`, giving each a screen size and
a time. The time matters because scrolling can be animated, and an
animation only moves when time passes. One more detail: egui hands back
the font textures it wants uploaded to the graphics card, and complains
if they are dropped unread, so a test clears them:

```rust
let mut output = self.ctx.run_pass(input, |ctx| {
    roll.show(ctx, project, pattern, None, &[], screen, zoom);
});
// Nothing is drawn, so the font textures are never uploaded.
output.textures_delta.clear();
```
