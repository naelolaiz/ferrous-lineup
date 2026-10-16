# From steps to notes

Files: [`crates/loom-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/pattern.rs)
and [`crates/loom-core/src/project.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/project.rs)

In step 2 a pattern was a grid of on/off switches, one `[bool; 64]` row
per channel. A piano roll needs more: each note has a pitch, a start and
a length, and several notes can start on the same step (a chord). So
step 3 replaces the switches with a list of notes per channel, and the
step sequencer becomes a special way of looking at that list.

## A note in a pattern

```rust
pub const STEP_NOTE: Note = Note::C4;
pub const DEFAULT_VELOCITY: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteEvent {
    pub pitch: Note,
    pub start: usize,  // in steps
    pub length: usize, // in steps, at least 1
    pub velocity: u8,  // 1..=127, how hard it is played
}

impl NoteEvent {
    pub fn step(start: usize) -> Self {
        NoteEvent::new(STEP_NOTE, start, 1)
    }

    pub fn sounds_at(&self, step: usize) -> bool {
        (self.start..self.end()).contains(&step)
    }

    pub fn is_step(&self) -> bool {
        self.pitch == STEP_NOTE && self.length == 1
    }
}
```

The name is `NoteEvent` because `Note` was already taken by the pitch
type from step 1. In audio programming, something that happens at a
time (a note starting, a knob turning) is often called an *event*.

`self.start..self.end()` is a **range**, a value of its own type
(`Range<usize>`), and `contains` asks whether a number is inside it.
Ranges include their start and exclude their end, so a note from step 4
with length 2 sounds at steps 4 and 5, not 6.

Every field is `Copy`, so `NoteEvent` derives `Copy` too: notes are
passed around like numbers, which keeps the code free of borrowing
puzzles.

**Velocity** is how hard a key is pressed, from 1 to 127, as in MIDI.
The audio engine turns it into volume. The piano roll does not edit it
yet (an exercise at the end of this step does).

## The pattern

```rust
pub struct Pattern {
    length: usize,
    rows: Vec<Vec<NoteEvent>>,
}
```

A row is now a `Vec<NoteEvent>`, the channel's notes in the order they
were added. The old step functions keep their names and meaning, written
in terms of notes:

```rust
/// Whether a note starts on this step (a step is "on").
pub fn is_on(&self, channel: usize, step: usize) -> bool {
    self.notes(channel).iter().any(|note| note.start == step)
}

/// Switches a step on (adds a step note) or off (removes every note starting there).
pub fn set(&mut self, channel: usize, step: usize, on: bool) -> bool {
    if step >= MAX_STEPS {
        return false;
    }
    let Some(row) = self.rows.get_mut(channel) else {
        return false;
    };
    row.retain(|note| note.start != step);
    if on {
        row.push(NoteEvent::step(step));
    }
    true
}
```

`retain` keeps the items for which the closure returns `true` and drops
the rest, in place. Because the step functions kept their signatures, the
step-2 tests for patterns still pass with only small changes.

`notes` returns a slice, and a channel that does not exist simply has
none:

```rust
pub fn notes(&self, channel: usize) -> &[NoteEvent] {
    match self.rows.get(channel) {
        Some(row) => row,
        None => &[],
    }
}
```

`row` is a `&Vec<NoteEvent>`, and Rust turns it into `&[NoteEvent]`
automatically (a **deref coercion**: a `Vec` can always be viewed as a
slice of its items). `&[]` is an empty slice, which, like a string
literal, lives in the program and costs nothing.

## Keeping notes in the grid: struct update syntax

`add_note` and `replace_note` first make the note fit the grid. A note
must start before `MAX_STEPS`, be at least one step long, and is
shortened if it would run past the end:

```rust
fn fitted(self) -> Option<NoteEvent> {
    if self.start >= MAX_STEPS || self.length == 0 {
        return None;
    }
    Some(NoteEvent {
        length: self.length.min(MAX_STEPS - self.start),
        ..self
    })
}
```

`..self` is **struct update syntax**: "every field I did not write takes
its value from `self`". It is the usual way to make a changed copy of a
struct. The piano roll uses it to move a note (`NoteEvent { pitch, start, ..note }`)
and to resize one (`NoteEvent { length, ..note }`).

`add_note` refuses a second note with the same pitch on the same start
step, and returns the new note's index, which the piano roll uses to
keep dragging a note it just created:

```rust
pub fn add_note(&mut self, channel: usize, note: NoteEvent) -> Option<usize> {
    let note = note.fitted()?;
    let row = self.rows.get_mut(channel)?;
    let duplicate = row
        .iter()
        .any(|other| other.pitch == note.pitch && other.start == note.start);
    if duplicate {
        return None;
    }
    row.push(note);
    Some(row.len() - 1)
}
```

## An iterator that borrows: `impl Iterator + '_`

To name the chord at a step, the piano roll needs "the pitches held at
step `s`". The pattern returns them as an iterator instead of a new `Vec`:

```rust
pub fn sounding_at(&self, channel: usize, step: usize) -> impl Iterator<Item = Note> + '_ {
    self.notes(channel)
        .iter()
        .filter(move |note| note.sounds_at(step))
        .map(|note| note.pitch)
}
```

Nothing is collected or allocated: the iterator walks the notes only when
someone pulls items out of it, and `analyze` (previous chapter) does
exactly that, reading the pitches straight from the pattern.

The iterator holds a reference into `self.rows`, so it must not outlive
the pattern. `+ '_` says so. `'_` is an **anonymous lifetime**: "the
lifetime of the borrowed argument", here `&self`. Since the 2024 edition
the compiler assumes this for every `impl Trait` return type, so the
code would build without it; it is written out so that a reader can see
the borrow. (In older editions it was required.) `move` gives the
`filter` closure its own copy of `step`, because the closure outlives
the function call that received `step`.

Iterators like this are **lazy**, and they compose. In the piano roll,
the whole chord lane is one expression:

```rust
let labels: Vec<Option<String>> = (0..length)
    .map(|step| analyze(project.pattern().sounding_at(channel, step)).label(spelling))
    .collect();
```

## Steps or a melody?

```rust
pub fn is_step_row(&self, channel: usize) -> bool {
    self.notes(channel).iter().all(NoteEvent::is_step)
}
```

`all` is true if the closure is true for every item (and also for an
empty list). Instead of a closure, it is given the method itself,
`NoteEvent::is_step`, a function that takes `&NoteEvent`, which is
exactly what `iter()` yields. The channel rack draws a row as step
buttons when this is true, and as a small piano roll picture when it is
not.

## The project: a key and a chord progression

`Project` gained a `scale: Scale` (C major by default): the key the
piano roll highlights and names chords in. `ChannelKind::Synth` lost its
`note` field, because the synth's pitch now comes from each note.

The demo plays C, Am, F, G, one of the most used progressions in pop
music (I, vi, IV, V). Each chord is a bass note and three chord notes:

```rust
let chords: [[u8; 4]; 4] = [
    [48, 60, 64, 67], // C:  C3, C4 E4 G4
    [45, 57, 60, 64], // Am: A2, A3 C4 E4
    [41, 57, 60, 65], // F:  F2, A3 C4 F4
    [43, 59, 62, 67], // G:  G2, B3 D4 G4
];
for (i, chord) in chords.iter().enumerate() {
    for &midi in chord {
        let note = NoteEvent::new(Note::new(midi), i * 8, 8);
        project.pattern.add_note(synth, note);
    }
}
```

Look at the upper three notes: C4 stays in the first three chords, E4
stays from C to Am, A3 stays from Am to F. Each chord changes as few
notes as possible, and those move by a step or two. This is called
**voice leading**, and it is why a progression sounds smooth instead of
jumping around. In the piano roll you can see it: long horizontal lines
of notes that carry on from one chord to the next.

A test plays the demo through `analyze` and checks the chord names, so
if any of these numbers is wrong, `cargo test` says which chord broke:

```rust
assert_eq!(chords, ["C", "Am", "F", "G"]);
```
