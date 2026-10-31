# Undo and redo

Files: [`crates/lineup-core/src/history.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/history.rs)
and [`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs)

Every change to a song can be undone with Ctrl+Z and redone with
Ctrl+Shift+Z or Ctrl+Y, and the Edit menu names the step it would undo
("Undo move clip"). This chapter is about how the history is kept, and
when one step ends and the next begins.

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

**Snapshots** keep a whole copy of the project from before each undoable
step. Undo puts the copy back. There is nothing to reverse, so there is
nothing to reverse wrongly: whatever the change did, the copy is exactly
what was there before. The cost is memory, and the time to make a copy.

Ferrous Lineup uses snapshots. A `NoteEvent` takes 24 bytes, and a song
has a few hundred or a few thousand of them, so a copy of a whole
project is some kilobytes. Even a big project with 10,000 notes holds
about 240 KB of notes, and making one copy takes far less than a
millisecond. The app keeps at most 256 MiB of snapshots: a small song
gets the full 100 steps, while a very large one still gets at least 10
(which can go over the budget). That is a fair price for undo that is
correct by construction. (One of the [exercises](exercises.md)
shows how to make the copies much cheaper, if a project ever gets big
enough to need it.)

## `History<T>`

The history lives in `lineup-core`, and it does not know about projects at
all:

```rust
struct Step<T> {
    label: &'static str,
    state: T,
}

pub struct History<T> {
    /// The oldest step is at the front, the newest at the back. A
    /// `VecDeque` can drop the oldest step from the front cheaply.
    undo: VecDeque<Step<T>>,
    redo: Vec<Step<T>>,
    limit: usize,
    /// Whether the newest undo step still takes in new changes.
    open: bool,
}

impl<T> History<T> {
    pub fn new(limit: usize) -> Self { ... }
    pub fn record(&mut self, label: &'static str, before: T) { ... }
    pub fn is_open(&self) -> bool { ... }
    pub fn close(&mut self) { ... }
    pub fn reopen(&mut self, label: &'static str) -> bool { ... }
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
with (this is **monomorphization**), so a `History<Project>` is as fast
as if it had been written for `Project` by hand.

### `&'static str` labels

Each step has a label, such as `"move clip"`, for the Edit menu. Its type
is `&'static str`: a reference to text that lives for the whole program.
Every string literal written in the code has this type, because the text
is stored inside the program file itself. So a label costs nothing: no
`String` is built, nothing is allocated, and the history holds no borrow
that could end too soon. The price is that a label cannot be made at run
time ("rename pattern Beat"), since that text would not exist until the
program builds it.

The labels are written in English, and the Edit menu translates them
when it shows them, so the history does not need to know the app's
language (see [Two languages](languages.md)).

### `VecDeque`: a list with two cheap ends

When there are already `limit` steps, recording one more forgets the
oldest. In a `Vec`, removing the first item (`remove(0)`) moves every
other item one place down. A **`VecDeque`** ("double-ended queue", said
"vec deck") is a ring buffer, like the queues between the threads (see
[Two threads](threads.md)): it keeps track of where its
front is, so `push_back` and `pop_front` are both cheap.

The limit is not a constant in the app. `History::new` takes it, and
`undo_limit_for` works it out from the project's size:

```rust
const UNDO_BYTES: usize = 256 * 1024 * 1024;
const MIN_UNDO_STEPS: usize = 10;

pub(super) fn undo_limit_for(project: &Project) -> usize {
    limit_for_bytes(project.heap_bytes())
}

fn limit_for_bytes(bytes: usize) -> usize {
    (UNDO_BYTES / bytes.max(1)).clamp(MIN_UNDO_STEPS, DEFAULT_LIMIT)
}
```

`heap_bytes` counts the notes, clips, markers and names, the things
whose number grows with the song. A small project divides 256 MiB by a
few kilobytes and gets more than `DEFAULT_LIMIT`, so it is clamped to
100. A huge project gets fewer, but never fewer than 10, so undo does
not vanish just because the song is long. Opening a file builds a fresh
`History` with the limit for that project. `History::default()` means
100, which is what the little tests use.

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
        self.close_step();
    }
}

/// Called at the end of each frame. While a mouse button is held (a
/// drag), a text field has the keyboard (typing a name) or a take is
/// being recorded, changes join one undo step; once all of them are
/// over, the step ends.
pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
    let holding = ctx.input(|input| input.pointer.any_down());
    if !holding && !ctx.text_edit_focused() && !self.is_recording() {
        self.close_step();
    } else if self.step_unsynced {
        // The step goes on, but its last change may still be waiting
        // for its turn: a drag held still, or the last key typed, has
        // no next edit to carry it to the audio thread. Send it when
        // it is due, and make sure a frame comes then.
        self.sync_step_if_due();
        if self.step_unsynced {
            ctx.request_repaint_after(SYNC_INTERVAL.saturating_sub(self.last_sync.elapsed()));
        }
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
- **Recording from a keyboard.** Every note played while recording joins
  one step, so one undo takes the whole recording away. A sung take
  written in from the **Your take** window is one step too (see
  [Takes](../theory/listen.md#in-the-app)).

`start_frame_for_undo` matters for one case: a click that starts while a
text field still has the focus. The press closes the typing step, so the
click's change does not join it.

The `else if` branch is about sound, not undo. While a step is open,
changes reach the audio thread at most every 50 ms, carried by the next
edit (see [Telling the audio thread what
changed](sync.md#a-drag-is-one-step-not-one-per-frame)).
A drag held still makes no next edit, so the frame sends what is still
waiting once it is due, and `request_repaint_after` asks egui for a
frame at that moment even if nothing moves.

Closing a step is `close_step`:

```rust
fn close_step(&mut self) {
    self.finish_step_sync();
    if self.history.is_open() {
        self.file.project_reverted();
    }
    self.history.close();
}
```

It sends whatever the audio thread is still missing, and works out the
unsaved-changes star again (a note dragged away and back leaves the song
as it was saved). Then the history stops taking changes into the step.

### Several clicks, one step

Some changes come one click at a time but belong together. Tapping a
tempo is a click per beat; nudging the master volume or the number of
choruses is a click or a drag per nudge. Undo should take all of them
back at once. `reopen` lets the app open the newest step again:

```rust
/// Opens the newest step again, if it is called `label` and nothing
/// was undone since, so the next change joins it: taps of a tempo,
/// each a click of its own, make one step. Returns whether it did.
pub fn reopen(&mut self, label: &'static str) -> bool {
    if self.redo.is_empty() && self.undo_label() == Some(label) {
        self.open = true;
    }
    self.open
}
```

and the app calls it just before the edit:

```rust
pub(super) fn set_master_volume(&mut self, master: f32) {
    const STEP: &str = "master volume";
    self.history.reopen(STEP);
    self.edit(STEP, |project| project.mixer.master = master);
}
```

If the last step was another change, or something was undone since,
the label does not match and a new step starts as usual.

### Everything goes through `edit`

The pattern name field shows how strict the rule "every change goes
through `edit`" is. egui's text field wants a `&mut String` to change.
Giving it the project's own name would change the project directly,
behind the history's back: no undo step, and no message to the audio
thread either. So the field edits a copy, and the copy goes through
`edit` when it changed:

```rust
// The field edits a copy, so the change can go through `edit`.
let mut name = self.project.patterns()[self.pattern].name.clone();
if ui.text_edit_singleline(&mut name).changed() {
    self.edit_pattern("rename pattern", |pattern| pattern.name = name);
}
```

The `Project` type helps here: its lists are private, and only its
methods can change them (see [The project](project.md)).
Anything that changes a project needs a `&mut Project`, and in the app
the only place that hands one out is `edit`.

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

`Modifiers::COMMAND` is Ctrl on Linux and Windows and ⌘ on a Mac. egui's
`consume_shortcut` is forgiving about extra keys: a shortcut without
Shift also matches when Shift is held. That is usually what you want (on
many keyboards `+` needs Shift, and Ctrl++ should still work), but here
it means Ctrl+Shift+Z would count as Ctrl+Z too. Checking redo first,
and using `else if`, settles it.

`||` stops at the first `true`, so if Ctrl+Shift+Z was pressed, Ctrl+Y is
not even looked at. "Consume" means the key press is taken out of egui's
input, so nothing else in the same frame reacts to it.

These shortcuts are skipped while a text field has the keyboard. egui's
text fields have their own undo for typing, and Ctrl+Z there should undo
the last letters, not the last change to the song.

The Edit menu shows the same shortcuts, written the way the system
writes them ("Ctrl+Z" on Linux, "⌘Z" on a Mac), and puts the translated
label in the item, "Undo move clip".

## Undo and the unsaved-changes star

The window title shows a star when the project differs from the saved
file. It does not compare the two projects on every frame: an edit sets
a flag, and the comparison happens when the title is next worked out.
Undo and redo set the flag too, so they can make the star go away when
the project is equal to the saved one again. A plain `dirty: bool` set
on every edit would have kept the star after the undo. [Saving](saving.md)
shows the rest of this.

Opening a file, or File > New, replaces the whole project and clears the
history. Undoing into a different song would be surprising, and the
"Save changes?" question already protects what you leave behind.
