# The piano roll, grown up

File: [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

The tools are plain functions, but something has to decide which notes
they get, call them at the right moment, and show the result. That is
the piano roll's job, and this step roughly doubled its size. This
chapter goes through the new parts in the order you meet them when you
use them: selecting, moving, the keyboard and the menu, the other
channels, and velocity.

## The selection holds notes, not positions

The simplest way to remember which notes are selected would be their
positions in the channel's list: `selection: Vec<usize>`. The single-note
drag from step 3 does exactly that (`Drag::Move { index, .. }`), and it
works there because nothing else changes the list during a drag.

A selection lives much longer than a drag. While notes are selected,
a tool takes them out of the list and puts new ones at the end
(`replace_notes` does that), undo swaps in a whole older project, and
deleting a note moves every note after it one place up. After any of
those, position 3 may be a different note, or no note at all. A
selection of positions would quietly select the wrong notes, and the
next tool would change notes you never chose.

So the selection holds the notes themselves:

```rust
/// The selected notes, by value. A note that is no longer in the
/// pattern (after an undo, say) is simply not found, and so not
/// selected: the selection can never point at the wrong note.
selection: Vec<NoteEvent>,
```

This works because a channel never holds two notes in the same place: the
pattern's rule (`same_place`: same pitch, same start, same `nudge`) makes
every note's value unique, so a value names exactly one note. `NoteEvent`
is `Copy` and `PartialEq`, so keeping copies and comparing them is cheap
and easy.

At the start of every frame, notes that have gone are forgotten:

```rust
let notes = pattern.notes(self.channel);
self.selection.retain(|note| notes.contains(note));
```

And when the piano roll replaces some notes, it makes the selection
follow them, so that pressing ↑ three times moves the same chord three
times:

```rust
fn replace(&mut self, pattern: &Pattern, actions: &mut Vec<RollAction>,
           label: &'static str, old: Vec<NoteEvent>, new: Vec<NoteEvent>)
    -> Option<Vec<NoteEvent>>
{
    let new = tidy_notes(new);
    if old == new || !pattern.can_replace_notes(self.channel, &old, &new) {
        return None;
    }
    let was_selected = old.iter().any(|note| self.selection.contains(note));
    self.selection.retain(|note| !old.contains(note));
    if was_selected {
        self.selection.extend(&new);
    }
    actions.push(RollAction::ReplaceNotes { channel: self.channel, old, new: new.clone(), label });
    Some(new)
}
```

`replace` is the one door every group change goes through: the tools,
the group drag and the velocity lane. It tidies the new notes the same
way the pattern will, asks the pattern whether the change is allowed,
and only then sends the action. Tidying here too means the selection
holds exactly the notes that will be in the pattern, and checking first
means a refused move does nothing at all instead of half a thing.

The comparison costs a little: `contains` looks through the whole list,
so checking every note against the selection takes time proportional to
the number of notes times the size of the selection. For a piano roll
with a few hundred notes, that is far below a millisecond. If it ever
mattered, a `HashSet<NoteEvent>` would make each check constant time
(`NoteEvent` already derives `Hash` for that).

### Selecting with the mouse

Ctrl+click on a note adds it to the selection, or takes it out:

```rust
if let Some(at) = self.selection.iter().position(|&n| n == note) {
    self.selection.remove(at);
} else {
    self.selection.push(note);
}
```

`input.modifiers.command` is "Ctrl" on Linux and Windows and "⌘" on a
Mac, so the same code follows each system's habits.

Ctrl+drag on empty space draws a box. Only the corner where it started
is stored; the selection is rebuilt from scratch on every frame, from
that corner and the pointer, so making the box smaller again unselects
notes:

```rust
if let Some(origin) = self.selection_box
    && let Some(pos) = pointer
{
    let area = Rect::from_two_pos(origin, pos);
    self.selection = notes
        .iter()
        .filter(|note| note.start < grid.length && grid.note_rect(note).intersects(area))
        .copied()
        .collect();
}
```

`Rect::from_two_pos` makes a rectangle from any two corners, whichever
way the box was dragged, and `intersects` is true when two rectangles
overlap at all, so a note only needs to be touched by the box.

## Moving a group

Pressing a selected note (anywhere but its right end, which still
resizes) starts a group drag:

```rust
struct GroupDrag {
    /// The cell where the drag started.
    from_step: usize,
    from_pitch: Note,
    /// The notes as they were when the drag started.
    originals: Vec<NoteEvent>,
    /// The notes as they are now, after the moves so far.
    moved: Vec<NoteEvent>,
}
```

On every frame of the drag, the new position is computed from the
*originals*, with the whole distance from the start cell to the pointer:

```rust
let steps = step as isize - drag.from_step as isize;
let semitones = i32::from(pitch.midi()) - i32::from(drag.from_pitch.midi());
let new = tools::shift(&drag.originals, steps, grid.length)
    .and_then(|shifted| tools::transpose(&shifted, semitones))
    .filter(|moved| fits_view(moved));
```

Moving the notes a little each frame (from `moved`) would be the
obvious way, but small errors would add up: a move refused at the edge
of the pattern would be lost, and the notes would end up somewhere other
than under the pointer. Starting from the originals every time means
the notes are always exactly where the pointer says, and dragging back to
the start puts them back exactly as they were.

The two tools are joined with `Option::and_then`: shift, and *if that
worked*, transpose the result. `filter` then refuses notes outside the
rows the piano roll shows. The replacement goes from `moved` (what is in
the pattern now) to `new`, and `moved` is updated to what `replace`
returned.

The drag is taken out of `self` while it is worked on:

```rust
if let Some(mut drag) = self.group_drag.take()
    && let Some(pos) = pointer
{
    // ... self.replace(pattern, actions, "move notes", drag.moved.clone(), new) ...
    self.group_drag = Some(drag);
}
```

`self.replace` needs `&mut self`, the whole struct. If `drag` were a
reference into `self.group_drag`, the borrow checker would refuse: one
part of `self` would be borrowed while all of `self` is borrowed
mutably. `Option::take` moves the drag out and leaves `None`, so `drag`
is a separate value and `self` is free; the drag is put back at the end.

Every frame of the drag produces a `ReplaceNotes`, and each one goes
through `edit`. They still make a single undo step:
[step 5's history](../step-5/history.md) keeps a step open while a mouse
button is held. And because the step is already open, `edit` does not
copy the whole project again on every frame; it sends the audio thread
the difference from the last state it was given at most every 50 ms, and
the rest when the button is released. Nothing in the piano roll had to
know about either detail.

## The keyboard

### Who gets the keys

The arrow keys and Delete are useful in the piano roll, but Ferrous
Lineup has other windows, and a key that changes notes in a window you are
not looking at would be a nasty surprise. So the piano roll only takes
the keys after a click inside its window, until a click anywhere else:

```rust
let press = ctx.input(|input| {
    input
        .pointer
        .any_pressed()
        .then(|| input.pointer.press_origin())
        .flatten()
});
if let Some(pos) = press {
    self.has_keyboard = self.window_rect.is_some_and(|rect| rect.contains(pos));
}
```

- `bool::then(closure)` is the lazy cousin of `then_some`: `Some(closure())`
  if the condition is true, `None` otherwise, without calling the closure
  when it is false.
- `press_origin()` itself returns an `Option<Pos2>`, so `then` makes an
  `Option<Option<Pos2>>`, and `flatten` turns that into an
  `Option<Pos2>`.
- The window's rectangle is the one from the *previous* frame, saved
  after `Window::show`. During this frame, the window has not been drawn
  yet, so last frame's position is the best there is, and a position one
  frame old is close enough for a click.

The keys are also ignored while a text field has the keyboard
(`ctx.text_edit_focused()`), so typing a pattern name never moves notes.

### The order of the shortcuts

Each key is a `KeyboardShortcut`: a key and its modifiers.

```rust
const KEY_UP: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowUp);
const OCTAVE_UP: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowUp);
const IN_KEY_UP: KeyboardShortcut = shortcut(Modifiers::ALT, Key::ArrowUp);
```

`shortcut` is a `const fn`, a function simple enough to run while
compiling, which is why its results can be `const`s.

The piano roll asks egui whether each one was pressed with
`input.consume_shortcut(&shortcut)`. That returns `true` once, and
"consumes" the key press so nothing else reacts to it. One detail of
egui matters here: it matches shortcuts *logically*, which means extra
Shift or Alt keys do not stop a match. Shift+↑ counts as a press of ↑ as
well. Checked in the wrong order, Shift+↑ would be eaten by the "up a
semitone" shortcut before the "up an octave" one ever saw it. So the
list goes from the most specific to the least:

```rust
// egui counts Shift+↑ and Alt+↑ as ↑ too, so the longer shortcuts
// are checked first.
let keys = [
    (IN_KEY_UP, Tool::InKey(1)),
    (IN_KEY_DOWN, Tool::InKey(-1)),
    (OCTAVE_UP, Tool::Transpose(12)),
    (OCTAVE_DOWN, Tool::Transpose(-12)),
    (KEY_UP, Tool::Transpose(1)),
    // ...
];
keys.into_iter()
    .find(|&(shortcut, _)| pressed(shortcut))
    .map(|(_, tool)| tool)
```

`find` stops at the first shortcut pressed, so at most one tool runs per
frame, and the later, shorter shortcuts are not even asked.

### One enum for the menu and the keys

The Tools menu and the keys both produce a `Tool`:

```rust
enum Tool {
    Transpose(i32),
    InKey(i32),
    Shift(isize),
    SnapToKey,
    Invert { up: bool },
    Arpeggiate,
    Delete,
    SelectAll,
    SelectNone,
}
```

and `use_tool` turns any `Tool` into a call to the right function in
`lineup_core::tools`, followed by `replace`. Whether a tool came from the
menu or a key, it takes exactly the same path, so the two can never
behave differently. The menu shows each tool's key next to it, written
the way the system writes keys (`ctx.format_shortcut`), so it doubles as
the list of shortcuts.

**Changed later.** The enum has grown as more tools arrived: `Quantize`,
and `Copy`, `Cut`, `Paste` and `Duplicate` for the note clipboard. They
all still become a `Tool` and go through `use_tool`; a paste adds the
notes with `AddNotes`, and a cut replaces the selection with an empty
`new` through `ReplaceNotes`.

### A menu that stays open

egui menus close on any click inside them, which is right for a list of
commands. But the Tools menu also holds choices: the arpeggio's order
and speed. Closing the menu when you pick "up and down" would make you
open it again to press "Arpeggiate chords". So this menu is told to
close only on a click *outside* it, and each tool closes it by hand:

```rust
let stay_open = egui::containers::menu::MenuConfig::new()
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
let menu = egui::containers::menu::MenuButton::new("Tools").config(stay_open);
menu.ui(ui, |ui| {
    let mut item = |ui: &mut Ui, text: &str, tool: Tool, shortcut: Option<KeyboardShortcut>| {
        // ...
        if ui.add(button).clicked() {
            picked = Some(tool);
            ui.close();
        }
    };
    item(ui, "Up a semitone", Tool::Transpose(1), Some(KEY_UP));
    // ...
});
```

`item` is a closure stored in a variable and called many times. It
assigns to `picked`, a variable outside it, so it borrows `picked`
mutably for as long as the closure exists, and has to be declared `let
mut item` (calling it changes what it captured: it is an `FnMut`). The
`ui` is passed in as an argument on each call instead of being captured,
because the code between the calls uses `ui` too, and a closure holding
on to it would block that.

## Ghost notes: hearing the other channels

A chord is often shared between instruments: the pad plays C and G, the
bass plays E, and together they make C/E. Before this step, the chord
lane only looked at the channel being edited, so it would say "C5" (a
power chord) where you hear C/E.

Now the piano roll listens to the other channels as well. Not all of
them: drum channels hold plain steps, and a kick drum is not part of the
harmony. A channel counts if any of its notes is not a plain step:

```rust
let others: Vec<usize> = (0..pattern.rows())
    .filter(|&other| other != self.channel)
    .filter(|&other| pattern.notes(other).iter().any(|note| !note.is_step()))
    .collect();
let mut heard = vec![self.channel];
if self.ghosts {
    heard.extend(&others);
}
```

`heard` is the list of channels whose notes count for the harmony: this
one, and the others when "Ghost notes" is ticked. The chord lane, the
line that names the chord under the pointer and the suggestions all
take `heard`, and gather its notes with a small helper:

```rust
/// The pitches held during `step` on any of `channels`.
fn sounding<'a>(
    pattern: &'a Pattern,
    channels: &'a [usize],
    step: usize,
) -> impl Iterator<Item = Note> + 'a {
    channels
        .iter()
        .flat_map(move |&channel| pattern.sounding_at(channel, step))
}
```

- `flat_map` calls the closure for each channel, gets an iterator of
  notes back each time, and joins them all into one iterator, as if the
  channels' notes were one list.
- The result borrows `pattern` and `channels`, so it must not outlive
  them. `<'a>` names that borrow, and `+ 'a` on the return type says "this
  iterator lives no longer than `'a`". `analyze` takes any iterator of
  notes, so it reads the notes straight from the pattern, without
  copying them into a `Vec` first.
- `move` makes the closure take its own copies of what it uses: `step` (a
  number) and `pattern` (a reference, which is copied, not the pattern).
  Without `move`, the closure would borrow the variable `step`, which
  belongs to `sounding` and disappears when it returns, while the
  iterator lives on.

The ghost notes are painted before this channel's notes, so this
channel's notes are always on top, in grey with a thin border. They are
only drawn: clicks only look at this channel's notes, so ghost notes can
never be selected, moved or deleted by accident. (They are not the same
thing as the outline that shows what a click would place, which step 3
also called a ghost. In the code, that one is now called an outline.)

When no other channel plays melodies or chords, the checkbox is greyed
out, and its hover text says why:

```rust
ui.add_enabled_ui(!alone, |ui| {
    ui.checkbox(&mut self.ghosts, "Ghost notes")
        .on_hover_text("Show the notes of the other channels faintly, ...")
        .on_disabled_hover_text("No other channel plays melodies or chords yet.");
});
```

## The velocity lane

### Making room for it

The lane sits under the grid. The grid is a `ScrollArea` that fills the
height it is given, and the first version gave it all the height left in
the window. The lane was drawn below that, so the window's content was a
little taller than the window. egui windows grow to fit their content,
so on the next frame the window was taller, the grid filled the new
height, the lane went below it again... and the window grew a few pixels
on every frame, for as long as the app ran.

The fix is to give the grid exactly the height that is left once the
lane has its share:

```rust
// The velocity lane goes below the grid, so the grid leaves room
// for it: the lane, the gap, and egui's spacing between widgets.
// Taking even a pixel too much would make the window grow a
// little on every frame.
let below = VELOCITY_HEIGHT + 4.0 + 2.0 * ui.spacing().item_spacing.y;
let grid_height = (ui.available_height() - below).max(96.0);
egui::ScrollArea::vertical()
    .auto_shrink(false)
    .max_height(grid_height)
    .show(ui, |ui| { /* the grid */ });
ui.add_space(4.0);
self.velocity_lane(ui, pattern, step_width, actions);
```

The easy part to forget is `item_spacing`: egui puts a small gap after
every widget, here after the scroll area and after the space. Leaving
those two gaps out was enough to make the window creep. The way to check
was simple: take two screenshots a few seconds apart and compare them.

### Setting velocities

The lane is one widget that senses clicks and drags. While the button is
down on it, the pointer's height sets the velocity, and its x position
picks the step:

```rust
if response.is_pointer_button_down_on()
    && let Some(pos) = response.interact_pointer_pos()
{
    let step = lane.step_at(pos.x);
    let height = (lane.rect.bottom() - pos.y) / lane.rect.height();
    let velocity = (height * 127.0).round().clamp(1.0, 127.0) as u8;
    let selected_here = notes
        .iter()
        .any(|note| note.start == step && self.selection.contains(note));
    let old: Vec<NoteEvent> = notes
        .iter()
        .filter(|note| note.start == step && note.velocity != velocity)
        .filter(|note| !selected_here || self.selection.contains(note))
        .copied()
        .collect();
    let new = old.iter().map(|&note| NoteEvent { velocity, ..note }).collect();
    self.replace(pattern, actions, "change velocity", old, new);
}
```

- The height is a fraction from 0 (bottom) to 1 (top), scaled to MIDI's
  1 to 127. `clamp` keeps it inside even when the pointer leaves the lane
  during a drag. Casting a float with `as` never wraps (300.0 becomes 255
  and -5.0 becomes 0), but 0 would be a note you cannot hear, so the
  clamp makes sure the velocity is at least 1.
- A chord has several notes on the same step. All of them change,
  unless some of them are selected: then only the selected ones do. That
  is how you make the top note of a chord louder than the others, the
  way pianists bring out a melody.
- Notes that already have the velocity are left out of `old`, so holding
  the button still does not send the same change every frame.

Sweeping across the lane changes each step as the pointer passes over
it, and the whole sweep is one undo step, like any drag.

Velocity shows in the grid too. A note's colour is multiplied by a
factor between 0.55 and 1 depending on its velocity
(`fill.gamma_multiply(0.55 + 0.45 * f32::from(note.velocity) / 127.0)`),
so soft notes are darker, but never so dark that they disappear.

## Testing by hand

The tools are covered by unit tests, but the piano roll can only be
tested by using it. Running the app and trying every feature found three
problems that no unit test would have:

- the window that grew on every frame (above),
- the Tools menu closing when an arpeggio order was picked (fixed by
  `CloseOnClickOutside`),
- stamped inversions landing above the pointer instead of around it
  (`stamp_notes` moving the chord back down).

That is a good habit for any UI code: write the logic so it can be
tested without the window, and then still try the window.
