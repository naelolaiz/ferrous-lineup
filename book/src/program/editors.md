# The editors: channel rack, piano roll and playlist

Files: [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs),
[`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs)
and [`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)

Three windows edit the music itself. The **playlist** fills the main
window: patterns placed as clips on tracks, which together make the
song. The **channel rack** (F6) shows one pattern as rows of steps, one
row per channel. The **piano roll** (F7) shows one channel of one
pattern as notes on a grid, with high notes at the top. The rack and the
piano roll float over the playlist, as the [previous
chapter](egui.md#windows-over-the-playlist) describes.

All three are built the same way, and that is the first thing this
chapter is about.

## Drawing returns actions

None of the editors changes the project. Each one only looks at it and
returns what the user did, as a list of actions. The rack's are an
enum:

```rust
pub enum RackAction {
    PlayPattern,
    // ...
    SelectChannel(Option<usize>),
    ToggleStep { channel: usize, step: usize },
    ClearStep { channel: usize, step: usize },
    Preview(usize),
    SetVolume { channel: usize, volume: f32 },
    ToggleMute(usize),
    Remove(usize),
    OpenPianoRoll(usize),
    AddDrum(Drum),
    AddSynth,
    // ...sending over MIDI, instruments...
    LoadSample { channel: Option<usize>, path: PathBuf },
}

pub fn show(
    ui: &mut Ui,
    project: &Project,
    pattern: usize,
    selected: Option<usize>,
    playhead: Option<usize>,
    playback: Option<RackPlayback>,
    locks: bool,
) -> Vec<RackAction>
```

`project: &Project` is a **shared borrow**: the function may read the
project but cannot change it, and the compiler enforces that. The app
then applies the actions:

```rust
let rack_actions = rack
    .show(ctx, |ui| {
        // ...
        channel_rack::show(ui, &self.project, self.pattern, selected, playhead, /* ... */)
    })
    .and_then(|window| window.inner)
    .unwrap_or_default();
for action in rack_actions {
    self.apply(action);
}
```

`Window::show` returns `None` when the window is closed, and its `inner`
is `None` when the window is folded up, so the actions are wrapped
twice. `.and_then(|window| window.inner)` unwraps the outer `Option` and
returns the inner one; `.unwrap_or_default()` turns `None` into an empty
`Vec`.

This split also solves a borrowing puzzle. While the rack is drawing, it
holds a shared borrow of the project, so it could not also change it
(that would need a `&mut` at the same time, which the borrow checker
forbids). Collecting actions first and applying them after the drawing
is finished keeps the two borrows apart in time.

`apply` is one `match` with an arm per action. Each arm that changes
the song goes through `edit` or `edit_pattern`, which keeps the undo
history and tells the audio thread what changed (see [Telling the audio
thread what changed](sync.md)):

```rust
RackAction::ToggleStep { channel, step } => {
    self.edit_pattern("change steps", |pattern| {
        pattern.toggle(channel, step);
    });
}
```

The same action does the same thing wherever it comes from: a click, a
key, or a menu. The piano roll returns `RollAction`s and the playlist
`PlaylistAction`s, and the app applies them the same way.

## The selected pattern

The rack and the piano roll show **one pattern**, the selected one. The
app keeps it:

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
pattern does not count as an unsaved change. The editors take it as an
argument, so they stay functions of what they are given.

The playhead they get depends on the mode. In pattern mode the audio
thread's step *is* a step of the selected pattern. In song mode it is a
step of the song, and the app asks the project where the selected
pattern is at that moment:

```rust
fn pattern_playhead(&self) -> Option<usize> {
    let step = self.playhead()?;
    match self.mode {
        PlayMode::Pattern => Some(step),
        PlayMode::Song => {
            // ...a pattern played over the song gives its own step...
            let (step, _) = self.song_position()?;
            self.project.pattern_step_in_song(self.pattern, step)
        }
    }
}
```

`pattern_step_in_song` looks for a clip of the pattern, on a track that
is not muted, that covers the song's step, and returns `None` if there
is none. So while the song plays, the rack's playhead runs whenever a
clip of the selected pattern is playing, and disappears in the bars
where it is not.

## The channel rack

![The channel rack while the song plays: a row of small lights over the
steps, the eighth one lit, and the eighth column of every row tinted
orange.](../images/step-9-rack-lights-song.png)

### Writing your own widgets

egui has buttons and sliders, but not step buttons or knobs. Making one
takes three moves: reserve space, read the mouse, paint.

```rust
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
        // ...a dark edge along the bottom, the pad, a light strip on a
        // step that is on, an orange outline on the step playing...
    }
    response
}
```

- `allocate_exact_size` claims a rectangle in the layout and says which
  interactions we want (`Sense::click()`). It returns the rectangle and a
  `Response` describing what the mouse did there this frame.
  `step_size(ui)` is the step's size times the rack's zoom.
- `is_rect_visible` skips painting rows that are scrolled out of view.
- The `Painter` draws shapes. Colours are `Color32` constants defined at
  the top of the file, so the palette lives in one place.
- `lerp_to_gamma` mixes some orange into the step playing, a little less
  on a step that is on, so a lit step stays white.
- Returning the `Response` lets the caller use it like any built-in
  widget: `if response.clicked() { ... }`.

Beats alternate colours, so you can count them, and start again in
every bar so all bars look alike: in 3/4, dark, light, dark. The rack
asks the time signature which beat each step is in
(`time.beat_at(step).index`), so 6/8 groups its steps in threes.

The knob uses `Sense::click_and_drag()`, turns vertical mouse movement
into a value change, and calls `response.mark_changed()` so callers can
use `.changed()` as they would on a slider. Its pointer is drawn with a
little trigonometry: the angle sweeps 270 degrees, and `(sin, -cos)` of
the angle gives the direction (screen *y* grows downward, hence the
minus).

### Menus

```rust
name.context_menu(|ui| self.channel_menu(ui, index, channel, locked, actions));
```

`context_menu` opens a menu on a right-click; `menu_button` opens one on
a normal click (the "Add channel" button). Both take a closure that
draws the menu's contents, and `ui.close()` closes the menu after a
choice. `add_enabled_ui(false, ...)` greys out everything inside, which
is how "Add channel" is disabled when the rack is full:

```rust
let rack_full = project.channels().len() >= MAX_CHANNELS;
ui.add_enabled_ui(!rack_full, |ui| {
    ui.menu_button(tr("➕ Add channel"), |ui| { /* ... */ });
});
```

### Drag and drop

egui reports files being dragged over the window in
`input.raw.hovered_files`, and files dropped in
`input.raw.dropped_files`. While drawing, the rack keeps each row's
rectangle; afterwards it finds the row under the mouse:

```rust
let target_row = pointer.and_then(|p| row_rects.iter().position(|rect| rect.contains(p)));
```

`position` returns the index of the first item for which the closure is
true, or `None`. A WAV file dropped on a row replaces that channel's
sound; a file dropped anywhere else becomes a new channel named after
the file (`path.file_stem()` is the file name without its extension). A
project, MIDI or chord sheet file, or a recording, is left for the app,
which opens or imports it.

Loading happens in `LineupApp::load_sample`, on the window's thread,
because reading files is slow and must never happen on the audio thread.
It reads the file *before* changing anything, so a broken file leaves
the channel as it was and puts the reason in the status bar.

### A row of lights over the steps

A drum machine has a row of small lights, one over each step, and the
light over the step being played runs along with the music. The rack
has that row too. The lights must sit exactly over the steps, but the
steps start after the mute light, the knob, the lock and the channel
name, and their widths change with the rack's zoom. Rather than adding
those widths up (and getting it wrong the next time a control is added
to a row), the rack lets the rows tell it:

```rust
let (lights, _) = ui.allocate_exact_size(
    egui::vec2(1.0, widgets::step_lights_height(ui)),
    egui::Sense::hover(),
);
let mut steps_left = None;
for (index, channel) in project.channels().iter().enumerate() {
    let row = ui.horizontal(|ui| self.row(ui, index, channel, actions));
    steps_left.get_or_insert(row.inner);
    row_rects.push(row.response.rect);
}
if let Some(left) = steps_left {
    let marks = step_marks(self.time, self.pattern.length());
    widgets::step_lights(ui, lights, left, &marks, self.playhead);
}
```

`allocate_exact_size` reserves an empty strip at the top, so the rows go
under it. Each `row` returns where its steps start (`ui.cursor().left()`
just before the first step); `get_or_insert` keeps the first row's value
and ignores the others. Once the rows are done, `step_lights` paints
into the reserved strip. egui draws everything at the end of the frame,
so painting into space reserved earlier is fine.

`step_lights` takes a slice of `StepMark`, a small enum that says
whether each step starts a bar, starts a beat, or neither:

```rust
pub enum StepMark {
    Bar,
    Beat,
    Plain,
}
```

The light over the first step of each bar is a little brighter than the
others, and the light over the first step of each beat a little brighter
than the steps in between, so the row can be counted like the steps
under it. Each light is placed one step width (the button plus the gap
after it) after the one before, the same arithmetic the mini piano roll
uses, so lights, buttons and previews line up at any zoom.

### A mini piano roll for melodies

A channel whose notes are all plain steps shows step buttons. Any other
channel shows `widgets::mini_roll`, a tiny drawing of its notes, and
clicking it opens the piano roll. The widget has to line up with the
step buttons, which have a gap between them:

```rust
let gap = ui.spacing().item_spacing.x;
let step_size = step_size(ui);
let step_width = step_size.x + gap;
let size = vec2(step_width * length as f32 - gap, step_size.y);
```

The pitch range is squeezed into the row's height, but never less than
an octave, so three close notes do not fill the row from top to bottom:

```rust
let visible = || notes.iter().filter(|note| note.start < length);
let lowest = visible().map(|note| note.pitch.midi()).min().unwrap_or(0);
let highest = visible().map(|note| note.pitch.midi()).max().unwrap_or(0);
let span = (highest - lowest).max(12) as f32;
```

`visible` is a closure that makes a fresh iterator each time it is
called, because `min` and `max` each use one up. The column that is
playing is filled with a see-through orange (`gamma_multiply(0.25)`
makes the colour a quarter as strong), as the step buttons are tinted.

### Drums in another pattern, and a rack that fits

Songs often keep their drums in a pattern of their own, "A drums" next
to "A". The rack for "A" would show drum rows with nothing in them,
which looks broken. So when a pattern has no drum steps and others do,
`drums_elsewhere` says so above the rows, with a button for each of
those patterns.

The rack opens as big as its rows. `natural_size` adds them up (the
controls, the steps, the lights, the "Add channel" menu, room for a
scroll bar), and the window is given that size for one frame whenever
the rows change, as [windows the size of what they
show](egui.md#windows-the-size-of-what-they-show) do. It
can still be made smaller, and then it scrolls. When every row fits,
the plain mouse wheel scrolls the steps sideways. egui has a style
setting for exactly that, which works when the scroll area allows only
one direction:

```rust
ui.style_mut().always_scroll_the_only_direction = true;
let vertical = rows_height > ui.available_height();
let scroll = egui::ScrollArea::new([true, vertical])
```

## The piano roll

![The piano roll in F major: B♭ is highlighted, B is not, and the B
notes are red.](../images/step-3-f-major.png)

### A window with state of its own

The channel rack is a function: everything it shows comes from the
project. The piano roll needs to remember things the project does not
know about, from one frame to the next: which channel it shows, what a
click draws, the selected notes, a drag in progress. So it is a struct
with methods:

```rust
pub struct PianoRoll {
    pub open: bool,
    channel: usize,
    stamp: ChordStamp,
    /// The length of new notes, in steps. Resizing a note changes it, so
    /// the next note drawn is as long as the last one edited.
    note_length: usize,
    drag: Option<Drag>,
    group_drag: Option<GroupDrag>,
    selection: Vec<NoteEvent>,
    ghosts: bool,
    zoom: Vec2,
    // ...
}
```

It still never changes the project itself. `show` returns a list of
`RollAction`s, as the rack does:

```rust
pub enum RollAction {
    /// Add one note, or the notes of a chord.
    AddNotes { channel: usize, notes: Vec<NoteEvent> },
    RemoveNote { channel: usize, index: usize },
    /// Replace a note that was moved or resized.
    ReplaceNote { channel: usize, index: usize, note: NoteEvent },
    /// Replace some notes with others, all at once: a moved selection,
    /// a tool, new velocities. `label` names the undo step.
    ReplaceNotes { channel: usize, old: Vec<NoteEvent>, new: Vec<NoteEvent>, label: &'static str },
    /// Play some notes right now, without adding them.
    Preview { channel: usize, pitches: Vec<Note> },
    SetScale(Scale),
    /// Show another pattern: one where the channel has notes.
    SelectPattern(usize),
    /// Open the channel rack, where drum steps are edited.
    OpenRack,
    // ...
}
```

So the struct holds *view* state, which is never saved, and the project
holds *document* state (the notes), which is.

The window is built like this:

```rust
// `open` is a copy: the window needs `&mut open` while the closure
// below needs `&mut self`, and both cannot borrow `self` at once.
let mut open = self.open;
let window = egui::Window::new(roll_title)
    .id(id)
    .open(&mut open)
    .default_pos([150.0, 160.0])
    .constrain_to(area);
// ...the size it opens at...
window.show(ctx, |ui| {
    self.contents(ui, project, pattern_notes, playhead, held, &mut actions)
});
self.open = open;
```

- The title changes with the pattern and the channel, and egui normally
  identifies a window by its title. `.id(...)` gives it a fixed identity
  instead, so it keeps its position and size when you switch channels.
- `.open(&mut open)` adds the close button: egui sets `open` to `false`
  when it is clicked.
- If the window borrowed `&mut self.open` while the closure borrowed
  `&mut self` (to call `self.contents`), `self.open` would be borrowed
  mutably twice at once, which Rust forbids. Copying the `bool` out and
  back in avoids the overlap.

### Pixels and music: `Grid`

Most of the drawing code turns steps and pitches into screen positions,
or the pointer position back into a step and a pitch. That arithmetic
lives in one place:

```rust
#[derive(Debug, Clone, Copy)]
struct Grid {
    /// Where all the steps and rows are, even those scrolled out of view.
    rect: Rect,
    step_width: f32,
    /// How tall a row (one pitch) is: the vertical zoom.
    row_height: f32,
    length: usize,
    time: TimeSignature,
    /// The left edge of the keyboard. The keyboard stays at the left of
    /// the view while the grid scrolls sideways under it.
    keys_left: f32,
}

impl Grid {
    /// The x coordinate where `step` begins.
    fn x(self, step: usize) -> f32 {
        self.rect.left() + step as f32 * self.step_width
    }

    /// The y coordinate of the top of a pitch's row (high notes at the top).
    fn row_top(self, midi: u8) -> f32 {
        self.rect.top() + (HIGHEST - midi) as f32 * self.row_height
    }

    /// The step and pitch under a point. Points outside the grid give the
    /// nearest cell, so a drag can go past the edges.
    fn cell_at(self, pos: Pos2) -> (usize, Note) {
        let row = ((pos.y - self.rect.top()) / self.row_height)
            .floor()
            .clamp(0.0, (HIGHEST - LOWEST) as f32) as u8;
        (self.step_at(pos.x), Note::new(HIGHEST - row))
    }
    // ...keys, cells, step_at, note_rect, text_size
}
```

High notes are at the top, so the row counts down from `HIGHEST`.
`cell_at` clamps, so a drag that leaves the grid still gets the nearest
cell, and returns a **tuple**, `(usize, Note)`, which the caller takes
apart with `let (step, pitch) = grid.cell_at(pos);`. `note_rect` draws a
note from its exact start to its exact end, including how far it is
nudged off the step, so a triplet sits between two steps, as it is
heard.

Because `Grid` is `Copy` and its methods take `self` by value, it can be
passed to every drawing function without any `&` or lifetimes. For a
struct of a few numbers, copying is as cheap as passing a reference.

At 100% the whole pattern fits across the window, so the step width is
worked out on every frame from the window's width. Zooming in makes the
steps wider, and the grid scrolls sideways under the keyboard.

### A keyboard that stays, lanes that follow

The grid is inside a scroll area, but two things must not scroll away
with it. The piano keys are drawn at the left edge of what is *visible*
(`keys_left: visible.left()`), on top of the grid, so the notes slide
under them. The chord lane above the grid and the velocity lane below it
are drawn outside the scroll area, so they do not know how far the grid
is scrolled. A small struct carries what they need to line up:

```rust
/// What the chord and velocity lanes need to line up with the grid.
#[derive(Debug, Clone, Copy)]
struct Lanes {
    step_width: f32,
    /// How far the grid is scrolled sideways, in pixels.
    scroll_x: f32,
    length: usize,
}
```

Each lane builds its own `Grid` from it (`Lanes::grid`), moved by
`scroll_x`, and uses the same functions as the note grid to find where a
step is. One source of truth for "where is step 12?" is what keeps three
strips drawn separately in line.

### Mouse input as a small state machine

egui reports what the mouse is doing *this frame*. A drag lasts many
frames, so the piano roll remembers it in an enum:

```rust
enum Drag {
    /// Moving a note. `grab` is how many steps after the note's start the
    /// pointer took hold of it, so the note does not jump under the pointer.
    Move { index: usize, grab: usize },
    /// Changing a note's length by dragging its right end.
    Resize { index: usize },
}
```

`handle_input` is long, but each part answers one question:

1. **Was the left button just pressed on the grid?** If it hit a note,
   start a `Move`, or a `Resize` if the press was near the note's right
   end. If it hit an empty cell, add what a click draws (a note or a
   chord); a single new note also starts a `Move`, so you can place and
   drag in one gesture.
2. **Is the right button down over a note?** Delete it. Checking every
   frame means you can sweep across several notes to delete them all.
3. **Is the left button up?** Then any drag is over.
4. **Is a drag in progress?** Work out where the note would be now, and
   if that differs from where it is, send a `ReplaceNote`.

The fourth part uses a **let chain** with three conditions, and struct
update syntax to change one or two fields:

```rust
if let Some(drag) = self.drag
    && let Some(pos) = pointer
    && let Some(&note) = notes.get(drag.index())
{
    let (step, pitch) = grid.cell_at(pos);
    let pitch = self.snapped(pitch, scale);
    let changed = match drag {
        Drag::Move { grab, .. } => NoteEvent {
            pitch,
            start: step.saturating_sub(grab),
            ..note
        },
        Drag::Resize { .. } => {
            let length = (step + 1).saturating_sub(note.start).max(1);
            self.note_length = length;
            // ...
            NoteEvent { length, trim, ..note }
        }
    };
    if changed != note {
        // ...play the new pitch, keep the note selected...
        actions.push(RollAction::ReplaceNote { channel, index: drag.index(), note: changed });
    }
}
```

`Some(&note)` in a pattern copies the note out of the `&NoteEvent` that
`get` returns. Comparing `changed != note` (thanks to the derived
`PartialEq`) means nothing is sent while the pointer stays inside the
same cell, even though this code runs every frame. `snapped` moves the
pitch onto the key when **Snap to scale** is on.

Two details make clicks land where they were meant. A quick right-click
can press and release the button within a single frame, so "is the
button down?" is already false by the time the frame is drawn. The code
asks "was it pressed this frame, or is it down?" instead:

```rust
pointer.secondary_pressed() || pointer.secondary_down(),
```

And on the frame the left button goes down, the press is handled where
the button *went down*, which `widgets::press_pos` reads from the
frame's events (see [Clicks that do what you
meant](egui.md#clicks-that-do-what-you-meant)): a quick
hand can press and move on in the same frame.

When the pointer is over an empty cell, the piano roll draws outlines of
what a click would place, from the same `stamp_notes` the click itself
uses. With "Chord in key (seventh)" chosen, you see the whole chord
before you place it.

### The selection holds notes, not positions

The simplest way to remember which notes are selected would be their
positions in the channel's list: `selection: Vec<usize>`. The
single-note drag does exactly that (`Drag::Move { index, .. }`), and it
works there because nothing else changes the list during a drag.

A selection lives much longer than a drag. While notes are selected, a
tool takes them out of the list and puts new ones at the end, undo swaps
in a whole older project, and deleting a note moves every note after it
one place up. After any of those, position 3 may be a different note,
or no note at all. A selection of positions would quietly select the
wrong notes, and the next tool would change notes you never chose.

So the selection holds the notes themselves:

```rust
/// The selected notes, by value. A note that is no longer in the
/// pattern (after an undo, say) is simply not found, and so not
/// selected: the selection can never point at the wrong note.
selection: Vec<NoteEvent>,
```

This works because a channel never holds two notes in the same place:
the pattern's rule (same pitch, same start, same nudge) makes every
note's value unique, so a value names exactly one note. `NoteEvent` is
`Copy` and `PartialEq`, so keeping copies and comparing them is cheap
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
fn replace(
    &mut self,
    pattern: &Pattern,
    actions: &mut Vec<RollAction>,
    label: &'static str,
    old: Vec<NoteEvent>,
    new: Vec<NoteEvent>,
) -> Option<Vec<NoteEvent>> {
    let new = tidy_notes(new);
    if old == new || !pattern.can_replace_notes(self.channel, &old, &new) {
        return None;
    }
    let was_selected = old.iter().any(|note| self.selection.contains(note));
    self.selection.retain(|note| !old.contains(note));
    if was_selected {
        self.selection.extend(&new);
    }
    actions.push(RollAction::ReplaceNotes {
        channel: self.channel,
        old,
        new: new.clone(),
        label,
    });
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
is kept; the selection is rebuilt from scratch on every frame, from that
corner and the pointer, so making the box smaller again unselects
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

### Moving a group

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
    /// Whether the notes have moved at all.
    edited: bool,
}
```

On every frame of the drag, the new position is worked out from the
*originals*, with the whole distance from the start cell to the
pointer:

```rust
let steps = step as isize - drag.from_step as isize;
let semitones = i32::from(pitch.midi()) - i32::from(drag.from_pitch.midi());
drag.edited |= steps != 0 || semitones != 0;
let snap = self.snap_to_scale && drag.edited;
let new = tools::shift(&drag.originals, steps, grid.length)
    .and_then(|shifted| tools::transpose(&shifted, semitones))
    .map(|moved| match snap {
        true => tools::snap_to_scale(&moved, scale),
        false => moved,
    })
    .filter(|moved| fits_view(moved));
```

Moving the notes a little each frame (from `moved`) would be the obvious
way, but small errors would add up: a move refused at the edge of the
pattern would be lost, and the notes would end up somewhere other than
under the pointer. Starting from the originals every time means the
notes are always exactly where the pointer says, and dragging back to
the start puts them back exactly as they were.

The tools (see [Editing tools as plain functions](../theory/tools.md))
are joined with `Option::and_then`: shift, and *if that worked*,
transpose the result. `map` snaps the notes to the key when Snap to
scale is on, but only once they have moved, so dragging a selection up a
semitone and back down snaps it into the key. `filter` then refuses
notes outside the rows the piano roll shows. The replacement goes from
`moved` (what is in the pattern now) to `new`, and `moved` is updated to
what `replace` returned.

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
through `edit_pattern`. They still make a single undo step, because a
step stays open while a mouse button is held (see [Undo and
redo](undo.md#when-does-one-step-end)). And because the
step is already open, `edit` does not copy the whole project again on
every frame; it sends the audio thread what changed at most every 50 ms,
and the rest when the button is released. Nothing in the piano roll has
to know about either.

### Who gets the keys

The arrow keys and Delete are useful in the piano roll, but the rack
and the playlist use them too, and a key that changes notes in a window
you are not looking at would be a nasty surprise. So the editor keys go
to the window clicked last, and to no other:

```rust
/// Editor shortcuts belong to the last clicked window, including its title
/// and toolbar. Popup controls and text fields keep their own keyboard input.
pub fn editor_keyboard(ctx: &egui::Context, layer: egui::LayerId) -> bool {
    editor_active(ctx, layer)
        && !ctx.text_edit_focused()
        && !egui::Popup::is_any_open(ctx)
        && ctx.memory(|memory| memory.is_above_modal_layer(layer))
}
```

`editor_active` remembers the *layer* of the last press (each window is
a layer, see [What is under the
pointer?](egui.md#what-is-under-the-pointer)), in egui's
memory, and answers whether it is this one. A window the app opens for
you, as a double-click on a clip opens the piano roll, is given the
keyboard with `widgets::give_keyboard`, as if it had been clicked. The
playlist shares the background layer with the toolbars, so for it the
place of the click matters too (`editor_keyboard_in`). The keys are
ignored while a text field has the keyboard, so typing a pattern name
never moves notes, and while a menu is open.

Space, which plays and stops, has the opposite problem: it must work
everywhere, but egui lets a focused button be pressed with Space too.
Click ▶ Play, then press Space, and the shortcut would stop the music
while the button started it again. So the shortcuts are read before any
panel is drawn, and `consume_key` takes the press away once it is
used:

```rust
let typing = ctx.text_edit_focused() || egui::Popup::is_any_open(ctx);
// `consume_key` reports whether the key was pressed and removes the
// press, so a focused button does not also react to Space.
let pressed = |key| ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
if !typing && !controls_active && pressed(egui::Key::Space) {
    // ...play or stop the song, or the pattern in the rack or piano roll...
}
```

### The order of the shortcuts

Each key is a `KeyboardShortcut`: a key and its modifiers.

```rust
pub(crate) const KEY_UP: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowUp);
pub(crate) const OCTAVE_UP: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowUp);
pub(crate) const IN_KEY_UP: KeyboardShortcut = shortcut(Modifiers::ALT, Key::ArrowUp);
```

`shortcut` is a `const fn`, a function simple enough to run while
compiling, which is why its results can be `const`s. They are
`pub(crate)` so that the list in **Help > Keyboard shortcuts** uses the
same constants (see [Every key in a
list](egui.md#every-key-in-a-list)).

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
    Copy,
    Cut,
    Paste,
    Duplicate,
    Transpose(i32),
    InKey(i32),
    Shift(isize),
    Quantize,
    SnapToKey,
    Invert { up: bool },
    Arpeggiate,
    Delete,
    SelectAll,
    SelectNone,
}
```

and `use_tool` turns any `Tool` into a call to the right function in
`lineup_core::tools`, followed by `replace`. A tool changes the selected
notes, or every note when none is selected; Delete only ever deletes a
selection. Whether a tool came from the menu or a key, it takes exactly
the same path, so the two can never behave differently. The menu shows
each tool's key next to it, written the way the system writes keys, so
it doubles as the list of shortcuts.

### A menu that stays open

egui menus close on any click inside them, which is right for a list of
commands. But the Tools menu also holds choices: the arpeggio's order
and speed. Closing the menu when you pick "up and down" would make you
open it again to press "Arpeggiate chords". So this menu is told to
close only on a click *outside* it, and each tool closes it by hand:

```rust
let stay_open = egui::containers::menu::MenuConfig::new()
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
let menu = egui::containers::menu::MenuButton::new(tr("Tools")).config(stay_open);
menu.ui(ui, |ui| {
    // ...
    let mut item =
        |ui: &mut Ui, text: &str, tool: Tool, shortcut: Option<KeyboardShortcut>| {
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
on to it would block that. Menus with a text field in them, such as the
playlist's section menu, stay open the same way, so a click into the
field does not close them.

### Ghost notes: hearing the other channels

A chord is often shared between instruments: the pad plays C and G, the
bass plays E, and together they make C/E. Looking only at the channel
being edited, the piano roll would say "C5" (a power chord) where you
hear C/E.

So the piano roll can listen to the other channels as well. Not all of
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
one, and the others when "Ghost notes" is ticked. The line that names
the chord under the pointer, the next-chord hints and the note names
all take `heard`, and gather its notes with a small helper:

```rust
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
  them. `<'a>` names that borrow, and `+ 'a` on the return type says
  "this iterator lives no longer than `'a`". `analyze` takes any
  iterator of notes, so it reads the notes straight from the pattern,
  without copying them into a `Vec` first.
- `move` makes the closure take its own copies of what it uses: `step`
  (a number) and `pattern` (a reference, which is copied, not the
  pattern). Without `move`, the closure would borrow the variable
  `step`, which belongs to `sounding` and disappears when it returns,
  while the iterator lives on.

The ghost notes are painted before this channel's notes, so this
channel's notes are always on top, in grey with a thin border. They are
only drawn: clicks only look at this channel's notes, so ghost notes can
never be selected, moved or deleted by accident. When no other channel
plays melodies or chords, the checkbox is greyed out, and its hover text
says why.

### The chord lane: `chunk_by`, then measure

The lane above the grid names the harmony. Its first row names what this
channel plays at every step; its second row, **All**, names what the
whole song sounds like there, beat by beat, as [Hearing
harmony](../theory/hearing.md) works it out. Each row joins
neighbouring steps with the same name into one block.

What "the same" means depends on the lane's **Show** menu. As chords, a
harmony holds its bass, so C and C/E are two blocks; as pitch-class
sets they are one, the same notes (see [In the piano
roll](../theory/sets.md#in-the-piano-roll)). So each step first gets
what the lane will call it, and the blocks are made from that:

```rust
#[derive(PartialEq)]
enum LaneName {
    Heard(Harmony),
    Named(Vec<HarmonyName>),
}

let steps: Vec<LaneName> = harmonies
    .iter()
    .map(|&(harmony, notes)| match self.notation.name_heard(&harmony, notes, scale) {
        Some(names) => LaneName::Named(names),
        None => LaneName::Heard(harmony),
    })
    .collect();
let mut start = 0;
for run in steps.chunk_by(|a, b| a == b) {
    let end = start + run.len();
    // ...a block from step `start` to step `end`, named by `run[0]`...
    start = end;
}
```

`chunk_by` splits a slice into runs wherever the closure says two
neighbours do not belong together. Each `run` is a slice of the original
vector, so nothing is copied. The `==` it uses is the one
`#[derive(PartialEq)]` writes: two `LaneName`s are equal when they are
the same variant with equal contents. Comparing whole harmonies, not
their labels, matters: every group of notes without a chord name is
labelled "?", and joining by label would turn several different unnamed
chords in a row into one long "?" block, as if one chord were held.

Running `analyze` for every step on every frame sounds like a lot, but it
is at most a few hundred small loops over a few notes. Measure before
optimizing: here it does not show up at all.

A name must fit its block. Cut off at the edges, "E7/B" in a narrow
block would show "7/", which is not a name at all, and for someone
learning harmony from the screen a wrong name is worse than none. egui
draws text in two steps. First it **lays it out**: it picks the glyph of
each character in the font, and works out where each goes and how much
room the whole takes. The result is a *galley* (the name comes from
printing: a tray of type set in lines, ready to print). Then it paints
the galley. `painter.text(...)` does both at once; to decide *before*
drawing, the two steps are taken apart:

```rust
let name = painter.layout_no_wrap(label, FontId::proportional(11.0), LANE_TEXT);
if name.size().x <= chord_rect.width() - 2.0 {
    // A long chord's name stays in view while it scrolls.
    let shown = chord_rect.intersect(cells);
    painter.with_clip_rect(chord_rect).galley(
        shown.center() - name.size() / 2.0,
        name,
        LANE_TEXT,
    );
}
```

`layout_no_wrap` lays the text out on one line, and `name.size()` is how
wide and tall it is. If it fits in the block, with a point to spare on
each side, the same galley is painted; nothing is laid out twice.
`galley` takes the top-left corner of the text, so the centre is moved
back by half the size. A set or a micromode has a shorter name to try
first, its class alone, "3-11" for "3-11 T7I"; if even that does not
fit, nothing is drawn. Pointing at the block still names it in full in
the line under the toolbar.

### Scale highlighting

`paint_rows` colours each row by asking the scale about its pitch class,
with a `match` on a tuple of two `bool`s:

```rust
let fill = match (is_root, in_scale) {
    (true, _) => ROOT_ROW,
    (false, true) => SCALE_ROW,
    (false, false) => OTHER_ROW,
};
```

The keys on the left get a dot for notes in the key (orange for the
root), and notes outside the key are drawn red. The key is the song's
key where the pattern is played (`project.pattern_key`), so a pattern
used after a key change is shown in the new key. Change the key and
every colour follows on the next frame, because nothing is cached:
immediate mode draws from the current state every time.

### The velocity lane

#### Making room for it

The lane sits under the grid. The grid is a `ScrollArea` that fills the
height it is given. Given all the height left in the window, it would
push the lane below the window's edge; egui windows grow to fit their
contents, so the window would grow, the grid would fill the new height,
the lane would be pushed out again, and the window would creep a few
pixels taller on every frame. So the grid is given exactly the height
that is left once the lane has its share:

```rust
// The velocity lane goes below the grid, so the grid leaves room
// for it: the lane, the gap, and egui's spacing between widgets.
// Taking even a pixel too much would make the window grow a
// little on every frame.
let below = VELOCITY_HEIGHT + 4.0 + 2.0 * ui.spacing().item_spacing.y;
let grid_height = (ui.available_height() - below).max(96.0);
```

The easy part to forget is `item_spacing`: egui puts a small gap after
every widget, here after the scroll area and after the space. A simple
way to catch a window that creeps is to take two screenshots a few
seconds apart and compare them.

#### Setting velocities

The lane is one widget that senses clicks and drags. While the button is
down on it, the pointer's height sets the velocity, and its x position
picks the step:

```rust
if response.is_pointer_button_down_on()
    && !crate::widgets::dismissing(ui.ctx())
    && let Some(pos) = response.interact_pointer_pos()
    && (cells.contains(pos) || response.dragged())
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
  1 to 127. `clamp` keeps it inside even when the pointer leaves the
  lane during a drag. Casting a float with `as` never wraps (300.0
  becomes 255 and -5.0 becomes 0), but 0 would be a note you cannot
  hear, so the clamp makes sure the velocity is at least 1.
- A chord has several notes on the same step. All of them change,
  unless some of them are selected: then only the selected ones do.
  That is how you make the top note of a chord louder than the others,
  the way pianists bring out a melody.
- Notes that already have the velocity are left out of `old`, so
  holding the button still does not send the same change every frame.

Sweeping across the lane changes each step as the pointer passes over
it, and the whole sweep is one undo step, like any drag.

Velocity shows in the grid too. A note's colour is multiplied by a
factor between 0.55 and 1 depending on its velocity
(`fill.gamma_multiply(0.55 + 0.45 * f32::from(note.velocity) / 127.0)`),
so soft notes are darker, but never so dark that they disappear.

### Finding your notes

The piano roll shows one channel of one pattern, and that pair can be
empty while the song plays plenty: the chords of a song may be in
"Chords", not in "Beat". An empty grid with no word of explanation looks
like lost notes. So the piano roll does two things.

First, it follows the notes. When it opens, or the pattern changes, and
the channel shown has no notes there, it moves to the channel with the
most melody or chord notes:

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
number instead: `std::cmp::Reverse` wraps a value and flips its
ordering. Drum steps are left out on purpose: in a pattern of drums
alone, the piano roll does not jump to the kick drum, whose "notes" are
all the same plain step. A channel you picked yourself, in the rack or
the editor, is kept.

To notice the window *opening*, the piano roll remembers whether it was
open last frame. `open` is a public field that the toolbar button, F7
and the View menu flip, so this is the one place that sees every way
in:

```rust
let opened = self.open && !self.was_open;
// ... at the end of `show`:
self.was_open = open;
```

On opening, it also scrolls to the middle of the notes. The middle is
not worked out by halving pitches: with integers, `/` throws away the
remainder, and `41 / 2 + 67 / 2` is 53, a row off the true middle, 54.
Instead it asks egui to centre the whole block of rows, from the top of
the highest note to the bottom of the lowest:

```rust
let top = grid.row_top(highest.clamp(LOWEST, HIGHEST));
let bottom = grid.row_top(lowest.clamp(LOWEST, HIGHEST)) + row_height;
let target = Rect::from_min_max(pos2(rect.left(), top), pos2(rect.left() + 1.0, bottom));
ui.scroll_to_rect(target, Some(Align::Center));
```

Second, when the channel really has no notes in the pattern, it says so
over the grid, with buttons to go where the notes are:

![The piano roll on "Beat": a coloured line says Lineup Osc has no notes
there, with a button for "Chords", where it does, and one for the
channel rack, where the drums of "Beat" are
edited.](../images/step-8-no-notes.png)

Finding the patterns where the channel plays is a filter over all of
them, keeping the index (to select it) and the name (to show it):

```rust
let elsewhere: Vec<(usize, &str)> = project
    .patterns()
    .iter()
    .enumerate()
    .filter(|&(index, other)| {
        index != self.pattern && !other.notes(self.channel).is_empty()
    })
    .map(|(index, other)| (index, other.name.as_str()))
    .collect();
```

The `&str` in the vector borrows from `project`. That is fine because the
vector lives only while this frame is drawn, and `project` outlives it.
A button for another pattern cannot select it directly: the selected
pattern belongs to the app. So it returns `RollAction::SelectPattern`,
the same way every other change leaves the piano roll.

The message lies *over* the grid instead of taking a line above it, so
the grid does not jump up when the first note is drawn and the message
goes away. It is drawn after the grid, in a child `Ui` placed over it,
and the grid remembers where it was drawn last frame (`self.banner`) and
leaves clicks there alone: a click on one of its buttons must not also
draw a note underneath.

### Wiring it into the app

The app owns a `PianoRoll` and, while drawing the windows, calls:

```rust
let roll_actions = self.piano_roll.show(
    ctx,
    &self.project,
    self.pattern,
    playhead,
    &held,
    area,
    roll_zoom,
);
for action in roll_actions {
    self.apply_roll(action);
}
```

`self.piano_roll.show(..., &self.project, ...)` borrows one field
mutably and another one immutably at the same time, which the borrow
checker allows because they are different fields. `apply_roll` then
changes the pattern through `edit_pattern`, as the rack does:

```rust
RollAction::ReplaceNotes { channel, old, new, label } => {
    self.edit_pattern(label, |pattern| {
        pattern.replace_notes(channel, &old, &new);
    });
}
```

When a channel is removed, `PianoRoll::channel_removed` either lets go
of its channel (it is gone) or moves its channel number down by one, so
it keeps showing the same channel; `channel_inserted` does the opposite
when undo brings a channel back.

In a short window, under 420 points, the grid comes first: the hint
lines under the toolbar move into the tooltip of the line that names the
chord, so a small screen still shows a dozen rows of notes.

## The playlist

### A big grid: `show_viewport`

The playlist can be far bigger than the window: dozens of bars, and as
many tracks as the song uses. It goes in a scroll area. But some parts
must *not* scroll away: the track names on the left, and the bar
numbers and the strips above them at the top.

`ScrollArea::show_viewport` makes this possible. Its closure receives
the visible part of the content, as a rectangle measured from the
content's top-left corner:

```rust
let output = area.show_viewport(ui, |ui, viewport| {
    self.grid(ui, viewport, project, current, playhead, &mut actions)
});
```

`grid` allocates the full size of the content, then works out where the
visible part is on screen:

```rust
let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

// `viewport` is the visible part, measured from the top-left of the
// content. Moving it by the content's position on screen gives the
// visible part in screen coordinates, where everything is drawn.
let visible = viewport.translate(rect.min.to_vec2());
```

`rect` is the whole content in screen coordinates (its top-left corner
moves up and left as you scroll). Adding that corner to `viewport` gives
`visible`, the part of the screen where the content shows. Then:

- the tracks and clips are drawn at their place in the content, so they
  scroll;
- the track names are drawn at `visible.left()`, and the ruler and the
  strips at `visible.top()`, so they stay put while the clips slide
  underneath.

They are drawn *after* the clips, so they cover them.

The arithmetic for steps, tracks and pixels lives in a small `Copy`
struct, as in the piano roll:

```rust
#[derive(Debug, Clone, Copy)]
struct Layout<'a> {
    /// The top-left corner of step 0 of track 0.
    origin: Pos2,
    /// How wide a step is and how tall a track is: the zoom.
    step_width: f32,
    track_tops: &'a [f32],
    tracks: usize,
    /// Whether there is an empty row below the last track, where a clip
    /// starts a new track. There is unless the playlist is full.
    spare: bool,
}
```

Each track can have its own height, so `Layout` keeps where each track
starts (`track_tops`), worked out once per frame. To find the track under
the pointer, `cell_at` uses `partition_point`, a binary search on that
sorted list: the number of tops above the pointer, minus one, is the
track.

### Real widgets in a painted area: `ui.put`

Everything in the grid is painted by hand, but the track names should be
editable text fields and the mute lights should be the same light as in
the rack. `ui.put` places any widget in a rectangle you choose:

```rust
let light_response = ui
    .put(light, |ui: &mut Ui| widgets::mute_light(ui, enabled))
    .on_hover_text(tr("Click to mute or unmute the track"));

let shown = tr_name(&track.name).into_owned();
let mut name = shown.clone();
let field = egui::TextEdit::singleline(&mut name)
    .frame(egui::Frame::NONE)
    .text_color(HEADER_TEXT);
let name_response = ui.put(name_rect, field);
if name_response.changed() && name != shown {
    actions.push(PlaylistAction::RenameTrack { track: index, name });
}
```

- A closure `|ui: &mut Ui| -> Response` counts as a widget: egui
  implements its `Widget` trait for any such closure. That is how a
  function-style widget like `mute_light` can be passed to `put`.
- The text field edits a *copy* of the name. If it changed, the copy
  goes into a `RenameTrack` action, and the app changes the project. The
  playlist keeps the rule that it never changes the project itself.
- `Frame::NONE` removes the field's border, so it looks like a label
  until you click it.

### Placing, moving and stretching clips

Placing, moving and deleting clips follows the piano roll's pattern: a
press decides what the drag is, and the drag follows the pointer until
the button is released.

```rust
struct ClipDrag {
    index: usize,
    grab: usize,
    stretch: bool,
}
```

A press on empty space places a clip of the selected pattern and starts
moving it; a press on a clip starts moving that clip and selects its
pattern; a press on a clip's right edge starts stretching it, which
repeats its pattern as many times as fit up to the pointer. The press
is handled where the button went down:

```rust
// A press is handled where the button went down. A quick hand can
// press and move in the same frame, and the clip or empty cell
// under the pointer *now* may not be the one that was clicked.
// A click that closes a menu does nothing else.
let dismissing = widgets::dismissing(ui.ctx());
let pressed_at = press_origin.filter(|pos| {
    primary_pressed && !dismissing && response.hovered() && cells.contains(*pos)
});
if let Some(pos) = pressed_at {
    let (track, step) = layout.cell_at(pos);
    match clip_at(project, layout, pos) {
        Some(index) => { /* start moving or stretching that clip */ }
        None => { /* add a clip of the selected pattern, and start moving it */ }
    }
}
```

`Option::filter` keeps the value only if the closure returns `true`, so
`pressed_at` is `Some` only on the frame of a press, over the grid.

Positions snap to the grid you choose in the toolbar: a bar, a beat or
a step. Where bars and beats fall depends on the time signature at that
point of the song, so the snap asks the song's `Meter`:

```rust
/// The snap point at or before `step`. Where the bars and beats are
/// depends on the time signature there: a bar of 3/4 is 12 steps, a
/// beat of 6/8 is 6.
fn floor(self, step: usize, meter: &Meter) -> usize {
    match self {
        Snap::Bar => meter.floor_to_bar(step),
        Snap::Beat => meter.floor_to_beat(step),
        Snap::Step => step,
    }
}
```

Ctrl or Shift and a click adds a clip to the selection, Ctrl and a drag
on empty space draws a box around clips, and dragging one of several
selected clips moves them all, kept inside the playlist.

### Double-click a clip to edit it

In most music programs you double-click a clip to edit it. egui counts
the clicks:

```rust
let double_clicked = ui.input(|input| {
    input
        .pointer
        .button_double_clicked(egui::PointerButton::Primary)
});
if double_clicked
    && !dismissing
    && response.hovered()
    && let Some(pos) = pointer.filter(|pos| cells.contains(*pos))
    && let Some(index) = clip_at(project, layout, pos)
{
    actions.push(PlaylistAction::EditClip(clips[index].pattern));
}
```

The app picks the editor: the piano roll, on the busiest channel, when
the pattern has melodies or chords; the channel rack when it only has
drums. Either way the window comes to the front, and takes the keyboard
with it:

```rust
fn bring_window_to_front(&mut self, ctx: &egui::Context) {
    if let Some(id) = self.bring_to_front.take() {
        let layer = egui::LayerId::new(egui::Order::Middle, id);
        ctx.move_to_top(layer);
        // Its keys go with it: Delete after double-clicking a clip
        // deletes notes in the piano roll, not clips in the playlist.
        widgets::give_keyboard(ctx, layer);
    }
}
```

A window's layer is named by the id the window was given, which is why
the rack and the piano roll get theirs from small functions (`rack_id()`
and `PianoRoll::id()`) instead of writing the same `Id::unique(...)` in
several places.

![Double-clicking a "Beat" clip opens the channel rack on "Beat", sized
to fit its rows.](../images/step-8-rack-double-click.png)

### Drawing a clip's notes: `Box<dyn Fn>`

Each clip shows a sketch of its pattern. Drum patterns look best as a
tiny step sequencer (one line per channel), anything with melody as a
tiny piano roll (higher notes higher). The two need different ways to
turn a note into a height, chosen once per clip:

```rust
// Turns a channel and a note into a height from 0.0 (bottom) to 1.0 (top).
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
    // ...the lowest and highest note, as in the rack's mini piano roll...
    Box::new(move |_, midi| ((midi - lowest) as f32 + centering) / span)
};
```

Every closure has its own, unnamed type, so the two branches of the `if`
produce values of different types, and a variable can only have one
type. `Box<dyn Fn(usize, u8) -> f32>` is the common type: "a closure on
the heap that takes a channel and a pitch and returns a height". `dyn
Fn` is a trait object, like `dyn App` in `main.rs`. The `move` keyword
makes each closure take ownership of what it uses (`used`, `lowest`...),
which must outlive the `if` that created them.

This runs on the window's thread, once per clip per frame, so the small
allocation for the `Box` does not matter. On the audio thread it would.

A clip's colour is its pattern's. Each pattern keeps a `color`, a place
in the playlist's palette, saved in the project file. It does not come
from the pattern's number, because then deleting the first pattern would
recolour all the others.

### Only the tracks you use

A project has only the tracks down to its last clip: File > New has one
track, a demo has the ones it uses. Below the last track there is a
fainter **spare row**: place a clip on it (or drag one onto it) and it
becomes a track. Its header has a **➕ New track** button too.

![The start demo after placing a clip on the spare row and pressing
"New track".](../images/step-8-spare-row.png)

`Project::clip_fits` decides whether a clip may go somewhere. One more
row than the tracks is allowed, the one just below the last track, as
long as the playlist is not full:

```rust
fn clip_fits(&self, clip: Clip) -> bool {
    let tracks = self.playlist.tracks.len();
    clip.pattern < self.patterns.len()
        && (clip.track < tracks || (clip.track == tracks && tracks < MAX_TRACKS))
        && clip.start < MAX_SONG_STEPS
        && (1..=MAX_REPEATS).contains(&clip.repeats)
}
```

`add_clip` and `move_clip` then call `open_track`, which adds a track
when the clip is on that new row. Because the track and the clip are added by the same edit, one
Ctrl+Z takes both away. The playlist view only has to draw one more row:
`Layout` has its `spare` flag and a `rows()` method, and `cell_at` clamps
to `rows()`.

Demos are built on a playlist of sixteen tracks, because naming track 0
and track 1 is easier when they exist; at the end, `remove_unused_tracks`
drops the empty tracks at the bottom. Files saved before tracks were
trimmed need almost the same thing, with one difference: a track someone
renamed or muted was kept on purpose, so loading drops only tracks that
are exactly as a new track starts. Two rules, one loop, which takes the
rule as a closure:

```rust
pub fn remove_unused_tracks(&mut self) {
    self.trim_tracks(|_, _| true);
}

pub(crate) fn remove_blank_tracks(&mut self) {
    self.trim_tracks(|index, track| *track == Track::numbered(index + 1));
}

fn trim_tracks(&mut self, drop: impl Fn(usize, &Track) -> bool) {
    // ...from the bottom, pop tracks with no clips while `drop` says so...
}
```

`impl Fn(usize, &Track) -> bool` means "any function or closure that
takes a row number and a track and answers yes or no". The compiler
makes a copy of `trim_tracks` for each closure, so this costs nothing at
run time. Comparing with `Track::numbered(index + 1)` works because
`Track` derives `PartialEq`: a blank track is equal to a brand new one
for its row.

Trimming only ever removes tracks **below** the last clip. An empty
track in the middle stays, because removing it would move every clip
under it up a row.

### Inserting, moving and deleting tracks

A clip names its track by **number**. Insert a track at row 2, and every
clip on row 2 or below must move down one, or the drums would suddenly
sit on the wrong track:

```rust
pub fn insert_track(&mut self, index: usize) -> Option<usize> {
    let tracks = &mut self.playlist.tracks;
    if tracks.len() >= MAX_TRACKS {
        return None;
    }
    let index = index.min(tracks.len());
    tracks.insert(index, Track::numbered(tracks.len() + 1));
    for clip in &mut self.playlist.clips {
        if clip.track >= index {
            clip.track += 1;
        }
    }
    Some(index)
}
```

`remove_track` is the mirror image: drop the track's clips with
`retain`, then move the ones below up one. `swap_tracks` swaps two rows
and the clips on them. Whoever changes the list fixes the numbers that
point into it, which is why the fields are private and only `Project`'s
methods change them (see [The project](project.md)). Each
of these is one call inside `edit`, so undo brings a deleted track back
with its clips.

![Right-click a track's name.](../images/step-8-track-menu.png)

The menu is egui's `context_menu` on the track's name field. A text
field still opens a menu on a right-click, because typing only needs the
left button. A track's lower edge can be dragged to make it taller or
shorter, and a double-click on it gives it back its usual height.

### Following the playhead past the names

With **Follow playhead** on, the playlist scrolls to keep the playhead in
view. When a song plays to its end and starts again, or a loop goes back
to its start, the playhead jumps left and the view follows it. But the
track names on the left are drawn *over* the bars, and to the scroll
area they are not there at all: revealing the playhead could leave it
under them. So `follow_x` is told how wide the fixed part is, and makes
the rectangle it reveals reach that much further left:

```rust
pub fn follow_x(ui: &Ui, step: Option<usize>, x: f32, view: Rect, covered: f32) {
    // ...only when the step changed, and Follow playhead is on...
        let margin = (view.width() * 0.1).min(60.0);
        let y = view.center().y;
        ui.scroll_to_rect(
            Rect::from_min_max(
                pos2(x - margin - covered, y - 0.5),
                pos2(x + margin, y + 0.5),
            ),
            None,
        );
}
```

`scroll_to_rect` scrolls as little as it can so the rectangle is inside
the view. Going left, that leaves the rectangle's left edge at the
view's left edge, `covered` points left of the playhead, so the playhead
ends up right of the names. The playlist passes `HEADER_WIDTH` (its
names), the piano roll `KEY_WIDTH` (its keyboard). Going right, nothing
changes. Only a new step scrolls, so you can still scroll by hand while
the music plays.

## Trying the windows

The tools and the project are covered by unit tests, and many of the
editors' behaviours by tests that draw the window without a screen (see
[Testing a window without a
screen](egui.md#testing-a-window-without-a-screen)). But
some problems only show when a person uses the window: a window that
creeps taller on every frame, a menu that closes when you pick a choice
in it, a chord that lands above the pointer instead of around it. That
is a good habit for any UI code: write the logic so it can be tested
without the window, and then still try the window.
