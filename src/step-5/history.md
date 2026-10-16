# Undo and redo: a history of copies

Files: [`crates/loom-core/src/history.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/history.rs)
and [`crates/loom-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/app/edit.rs)

## Two ways to undo

There are two classic ways to build undo.

The **command pattern** records every change as an instruction that knows
how to reverse itself: "add note C4 at step 3" is undone by "remove note
C4 at step 3". The history is a list of instructions. It uses very little
memory, but every kind of change needs its own reverse, written by hand,
and each one is a new place to make a mistake. Think of what undoing
"delete pattern" would need: put the pattern back at its old position,
with its name and length and the notes of every channel, put back every
clip that used it, and renumber the clips of the patterns after it again.
If one detail is forgotten, undo quietly gives back a slightly different
song.

**Snapshots** keep a whole copy of the project from before each change.
Undo puts the copy back. There is nothing to reverse, so there is nothing
to reverse wrongly: whatever the change did, the copy is exactly what was
there before. The cost is memory, and the time to make a copy.

Ferrous Loom uses snapshots. A `NoteEvent` takes 24 bytes, and the demo
song has 37 of them, so a copy of the whole demo project is a few
kilobytes. Even a big project with 10,000 notes holds about 240 KB of
notes, so 100 copies of it are about 24 MB, and making one copy takes far
less than a millisecond. That is a fair price for undo that is correct by
construction. (Exercise 6 shows how to make the copies much cheaper, if a
project ever gets big enough to need it.)

## `History<T>`

The history lives in `loom-core`, and it does not know about projects at
all:

```rust
struct Step<T> {
    label: &'static str,
    state: T,
}

pub struct History<T> {
    /// The oldest step is at the front, the newest at the back.
    undo: VecDeque<Step<T>>,
    redo: Vec<Step<T>>,
    limit: usize,
    /// Whether the newest undo step still takes in new changes.
    open: bool,
}

impl<T> History<T> {
    pub fn new(limit: usize) -> Self { ... }
    pub fn record(&mut self, label: &'static str, before: T) { ... }
    pub fn close(&mut self) { ... }
    pub fn undo(&mut self, current: &mut T) -> Option<&'static str> { ... }
    pub fn redo(&mut self, current: &mut T) -> Option<&'static str> { ... }
    pub fn undo_label(&self) -> Option<&'static str> { ... }
    pub fn redo_label(&self) -> Option<&'static str> { ... }
    pub fn clear(&mut self) { ... }
}
```

`History<T>` is **generic**: `T` is whatever is being remembered. The app
uses a `History<Project>`; the tests use a `History<i32>`, which makes
them short and easy to read:

```rust
#[test]
fn changes_join_the_open_step() {
    let mut history = History::default();
    let mut value = 0;
    // A drag: three changes before the mouse button is released.
    for new in [1, 2, 3] {
        history.record("drag", value);
        value = new;
    }
    history.close();
    history.undo(&mut value);
    assert_eq!(value, 0, "one undo goes back to before the drag");
}
```

Look at the `impl` line: `impl<T> History<T>`, with no **trait bound** on
`T`. Not even `Clone`. The history never copies a `T`; it only moves the
values it is given from one list to another. The caller makes the copy
(`self.project.clone()`) and hands it over. If a method of `History` ever
called `.clone()` on a state, the compiler would refuse with "no method
named `clone` found for type parameter `T`", and we would have to write
`impl<T: Clone>`. A generic type with no bounds is a strong promise: the
code cannot do anything with a `T` except store it, move it and drop it.

The compiler makes a separate copy of the code for each `T` it is used
with (this is **monomorphization**, from step 1), so a `History<Project>`
is as fast as if it had been written for `Project` by hand.

### `&'static str` labels

Each step has a label, such as `"move clip"`, for the Edit menu. Its type
is `&'static str`: a reference to text that lives for the whole program.
Every string literal written in the code has this type, because the text
is stored inside the program file itself. So a label costs nothing: no
`String` is built, nothing is allocated, and the history holds no borrow
that could end too soon. The price is that a label cannot be made at run
time ("rename pattern Beat"), since that text would not exist until the
program builds it. Exercise 7 asks how to allow both.

### `VecDeque`: a list with two cheap ends

When there are already 100 steps, recording one more forgets the oldest.
In a `Vec`, removing the first item (`remove(0)`) moves every other item
one place down. A **`VecDeque`** ("double-ended queue", said "vec deck")
is a ring buffer, like the queues between the threads in step 2: it keeps
track of where its front is, so `push_back` and `pop_front` are both
cheap.

```rust
pub fn record(&mut self, label: &'static str, before: T) {
    self.redo.clear();
    if self.open {
        return;
    }
    self.undo.push_back(Step { label, state: before });
    if self.undo.len() > self.limit {
        self.undo.pop_front();
    }
    self.open = true;
}
```

Every new change clears the redo list. After you undo three steps and
then change something, those three steps belong to a future that no
longer exists. (A few programs keep a whole tree of histories, so no
branch is ever lost; most simply drop the old future, as this one does.)

The redo list is a plain `Vec`, because steps are only added and taken at
its end.

### `mem::swap`: undo without copying

```rust
pub fn undo(&mut self, current: &mut T) -> Option<&'static str> {
    let mut step = self.undo.pop_back()?;
    // Now `current` holds the old state and `step` the newer one.
    std::mem::swap(current, &mut step.state);
    let label = step.label;
    self.redo.push(step);
    self.open = false;
    Some(label)
}
```

`pop_back()` gives an `Option`, and the `?` operator works on `Option` in
a function that returns an `Option`: if there is nothing to undo, `undo`
returns `None` at once and changes nothing.

**`std::mem::swap`** exchanges the values behind two `&mut` references.
After it, `current` (the app's project) holds the older version, and the
step holds the version we just left, ready for redo. Nothing is copied or
allocated: a `Project` is mostly `Vec`s, and a `Vec` is a pointer, a
length and a capacity, so swapping two projects swaps a few of those
small values, whatever the size of the song. The borrow checker is what
makes this simple: `swap` needs two `&mut` references, and two `&mut`
references can never point at the same value, so `swap` can exchange the
two values' bytes without checking anything.

`redo` is the mirror image, popping from `redo` and pushing onto `undo`.

## When does one step end?

Dragging a clip across the playlist moves it a little on many frames, and
each move is an edit. If each one were its own step, undoing the drag
would take as many presses of Ctrl+Z as there were frames. So the
history keeps the newest step **open**, and `record` does nothing while
it is: the step already holds the project from before the drag started,
which is exactly where undo should go back to.

The app decides when a step ends, because only it knows what the mouse
is doing. It calls the history at the start and at the end of every
frame:

```rust
/// Called at the start of each frame. A press of a mouse button
/// always begins a new undo step.
pub(super) fn start_frame_for_undo(&mut self, ctx: &egui::Context) {
    if ctx.input(|input| input.pointer.any_pressed()) {
        self.history.close();
    }
}

/// Called at the end of each frame. While a mouse button is held (a
/// drag) or a text field has the keyboard (typing a name), changes
/// join one undo step; once both are let go, the step ends.
pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
    let holding = ctx.input(|input| input.pointer.any_down());
    if !holding && !ctx.text_edit_focused() {
        self.history.close();
    }
}
```

Follow a few gestures through these two functions:

- **A click on a step.** egui reports a click on the frame the button is
  let go. That frame's edit opens a step, and the end of the same frame
  closes it, since no button is down any more: one change, one step.
- **A drag.** The press closes whatever was open. Every frame of the drag
  edits the project while the button is down, so the edits join one step.
  The frame where the button is let go closes it.
- **Typing a pattern name.** While the text field has the keyboard
  focus, every letter joins the same step. Clicking elsewhere takes the
  focus away and closes it, so undo puts back the whole old name at once.
- **Typing a tempo.** Clicking the BPM box turns it into a text field,
  so the same rule applies: everything typed there is one step, which
  ends when you leave the box.

`start_frame_for_undo` matters for one case: a click that starts while a
text field still has the focus. The press closes the typing step, so the
click's change does not join it.

### Everything goes through `edit`

The pattern name field shows how strict the rule "every change goes
through `edit`" is. egui's text field wants a `&mut String` to change.
Giving it the project's own name would change the project directly,
behind the history's back: no undo step, and (as the next chapter shows)
no message to the audio thread either. So the field edits a copy, and the
copy goes through `edit` when it changed:

```rust
// The field edits a copy, so the change can go through `edit`.
let mut name = self.project.patterns()[self.pattern].name.clone();
if ui.text_edit_singleline(&mut name).changed() {
    self.edit_pattern("rename pattern", |pattern| pattern.name = name);
}
```

The `Project` type helps here: its lists are private, and only its
methods can change them. Anything that changes a project needs a
`&mut Project`, and in the app the only place that hands one out is
`edit`.

## Keyboard shortcuts

```rust
const UNDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Z);
const REDO: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::Z);
/// The other common shortcut for redo, used on Windows.
const REDO_Y: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Y);

pub(super) fn edit_shortcuts(&mut self, ctx: &egui::Context) {
    let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
    // Ctrl+Z would also match Ctrl+Shift+Z, so redo is checked first.
    if pressed(REDO) || pressed(REDO_Y) {
        self.redo();
    } else if pressed(UNDO) {
        self.undo();
    }
}
```

`Modifiers::COMMAND` is Ctrl on Linux and Windows and ⌘ on a Mac, as in
step 4. egui's `consume_shortcut` is forgiving about extra keys: a
shortcut without Shift also matches when Shift is held. That is usually
what you want (on many keyboards `+` needs Shift, and Ctrl++ should still
work), but here it means Ctrl+Shift+Z would count as Ctrl+Z too. Checking redo
first, and using `else if`, settles it.

`||` stops at the first `true`, so if Ctrl+Shift+Z was pressed, Ctrl+Y is
not even looked at. "Consume" means the key press is taken out of egui's
input, so nothing else in the same frame reacts to it.

These shortcuts are skipped while a text field has the keyboard. egui's
text fields have their own undo for typing, and Ctrl+Z there should undo
the last letters, not the last change to the song.

The Edit menu shows the same shortcuts, written the way the system writes
them, with `ctx.format_shortcut(&UNDO)` ("Ctrl+Z" on Linux, "⌘Z" on a
Mac), and puts the label in the item: `format!("Undo {label}")`.

## Undo and the unsaved changes dot

Step 4 shows a dot in the window title when the project differs from the
saved file, by comparing the two projects. That choice pays off now:
make a change, the dot appears; undo it, and the dot goes away by itself,
because the project is equal to the saved one again. A `dirty: bool` flag
set on every edit would have kept the dot after the undo.

Opening a file, or File > New, replaces the whole project and clears the
history. Undoing into a different song would be surprising, and the
"Save changes?" question already protects what you leave behind.
