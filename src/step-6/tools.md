# Tools are plain functions

Files: [`crates/loom-core/src/tools.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/tools.rs)
and [`crates/loom-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/pattern.rs)

## Notes in, notes out

Every tool in the piano roll's Tools menu is a function in
`loom_core::tools` with the same shape: it borrows some notes and returns
new ones.

```rust
pub fn transpose(notes: &[NoteEvent], semitones: i32) -> Option<Vec<NoteEvent>>
pub fn shift(notes: &[NoteEvent], steps: isize, pattern_length: usize) -> Option<Vec<NoteEvent>>
pub fn transpose_in_key(notes: &[NoteEvent], scale: Scale, steps: i32) -> Option<Vec<NoteEvent>>
pub fn snap_to_scale(notes: &[NoteEvent], scale: Scale) -> Vec<NoteEvent>
pub fn invert(notes: &[NoteEvent], up: bool) -> Option<Vec<NoteEvent>>
pub fn arpeggiate(notes: &[NoteEvent], order: ArpeggioOrder, rate: usize) -> Vec<NoteEvent>
```

None of them changes anything. They do not see the pattern, the
project, the selection or the window; they cannot, because they are not
given any of them. This kind of function is called a **pure function**:
its result depends only on its arguments, and calling it has no other
effect. Pure functions are the easiest code there is to test, because a
test only has to call them and look at what comes back:

```rust
#[test]
fn moving_in_key_changes_the_chord() {
    // C major one step up in C major is D minor: D, F, A.
    let c_major = [note(60, 0, 4), note(64, 0, 4), note(67, 0, 4)];
    let up = transpose_in_key(&c_major, C_MAJOR, 1).unwrap();
    assert_eq!(pitches(&up), [62, 65, 69]);
}
```

All the decisions about *which* notes and *what then* stay in the piano
roll: it hands a tool the selected notes (or all of them), and turns the
result into a `RollAction`. The last section of this chapter shows how
that result reaches the pattern.

The arguments are slices, `&[NoteEvent]`, not `&Vec<NoteEvent>`. A slice
accepts a `Vec`, an array, or part of either (thanks to deref coercion),
so the tests can pass a plain array like `[note(60, 0, 4), ...]` while
the piano roll passes its `Vec`.

## Grouping notes into chords: `chords`

Inverting and arpeggiating work on chords, not on single notes, so the
first tool is a helper that finds the chords. In a piano roll, a chord
is simply the notes that start on the same step:

```rust
pub fn chords(notes: &[NoteEvent]) -> Vec<Vec<NoteEvent>> {
    let mut sorted = notes.to_vec();
    sorted.sort_by_key(|note| (note.start, note.pitch));
    // `chunk_by` cuts the sorted list wherever the start changes.
    sorted
        .chunk_by(|a, b| a.start == b.start)
        .map(<[NoteEvent]>::to_vec)
        .collect()
}
```

Three small things to notice:

- `sort_by_key` sorts by a **tuple**. Tuples compare item by item, like
  words in a dictionary: first by `start`, and only when two starts are
  equal, by `pitch`. So the notes end up in time order, and each chord
  lowest note first.
- `chunk_by`, which [step 3's chord lane](../step-3/piano-roll-ui.md)
  already used, cuts a slice into runs of neighbours for which the
  closure says "these belong together". Each run is a `&[NoteEvent]`, a
  borrowed piece of `sorted`.
- `.map(<[NoteEvent]>::to_vec)` passes a method as if it were a function.
  `<[NoteEvent]>::to_vec` is the `to_vec` method of slices of notes,
  written as a path. It is the same as `.map(|chunk| chunk.to_vec())`,
  without inventing a name for the chunk. The angle brackets are needed
  because `[NoteEvent]::to_vec` would not parse.

## When a tool cannot work: collecting `Option`s

Transposing a chord up an octave can fail: the MIDI range ends at note
127, and `Note::transpose` returns `None` for a note that would leave it.
What should happen when one note of a chord cannot move? Moving the other
notes would quietly change the chord. The tools do nothing instead: they
return `None` for the whole group.

```rust
pub fn transpose(notes: &[NoteEvent], semitones: i32) -> Option<Vec<NoteEvent>> {
    notes
        .iter()
        .map(|note| {
            Some(NoteEvent {
                pitch: note.pitch.transpose(semitones)?,
                ..*note
            })
        })
        .collect()
}
```

The closure returns an `Option<NoteEvent>` for each note. The `?` inside
it returns `None` *from the closure* (not from `transpose`) when
`Note::transpose` fails. So `map` makes an iterator of `Option`s, and
`collect` gathers them.

The trick is the type it collects into. `Option<Vec<NoteEvent>>`
implements `FromIterator<Option<NoteEvent>>`, which means: collecting
`Option`s into an `Option<Vec>` gives `Some(vec)` if every item was
`Some`, and `None` as soon as one item is `None`. It even stops at the
first `None`, without looking at the rest. Rust knows which
`collect` to use from the function's return type, so the code does not
have to say it. (`Result` works the same way: collecting `Result`s into
a `Result<Vec<_>, E>` stops at the first error.)

`..*note` is struct update syntax: "every other field as in `*note`".
`note` is a `&NoteEvent` here, because `iter()` lends the items, so it is
dereferenced with `*`. That only works because `NoteEvent` is `Copy`:
the fields are copied out, and the original stays where it was.

### Moving in time: `checked_add_signed` and `then_some`

`shift` moves notes earlier or later. A start is a `usize` (never
negative), but the move can be negative, so it is an `isize`:

```rust
pub fn shift(notes: &[NoteEvent], steps: isize, pattern_length: usize) -> Option<Vec<NoteEvent>> {
    notes
        .iter()
        .map(|note| {
            let start = note.start.checked_add_signed(steps)?;
            (start < pattern_length).then_some(NoteEvent { start, ..*note })
        })
        .collect()
}
```

`usize::checked_add_signed` adds a signed number to an unsigned one, and
returns `None` if the result would go below zero (or past the largest
`usize`). Writing `(note.start as isize + steps) as usize` instead would
turn -1 into a huge number without a word.

`bool::then_some(value)` turns a condition into an `Option`: `Some(value)`
if the condition is true, `None` if not. The two lines read as the rule
itself: the new start must exist, and must be inside the pattern.

## Staying in the key

### One note of the key up: `find_map`

"Up a note in the key" moves each note to the next note above it that
belongs to the key. In C major, E goes to F (one semitone up), but F goes
to G (two):

```rust
pub fn step_in_key(note: Note, scale: Scale, up: bool) -> Option<Note> {
    let direction = if up { 1 } else { -1 };
    (1..=12).find_map(|distance| {
        note.transpose(direction * distance)
            .filter(|candidate| scale.contains(candidate.pitch_class()))
    })
}
```

`find_map` tries each distance in turn, and returns the first `Some` the
closure gives. `Option::filter` keeps a `Some` only if its value passes
the test, so the closure says "the note this far away, if it exists and
is in the key". Every scale has a note within 12 semitones (the octave
of the note itself, if nothing nearer), so `None` only comes back at the
edges of the MIDI range.

`transpose_in_key` calls `step_in_key` once for every step of the move,
for every note, and collects the `Option`s just as `transpose` does. Its
effect on chords is the interesting part: every note of C, E, G moves to
the next note of the key, so the chord becomes D, F, A, which is D
*minor*. Moving a chord inside the key keeps it in the key, so its kind
changes from degree to degree, exactly like the seven chords of the key
in the "Chords in C major" row.

### The nearest note of the key: snapping

Snapping moves a note that is outside the key to the nearest note inside
it. In C major, C# is one semitone from C and one from D: which one wins?
There is no right answer, so the code makes a choice and says so: the
lower note.

```rust
pub fn nearest_in_key(note: Note, scale: Scale) -> Note {
    if scale.contains(note.pitch_class()) {
        return note;
    }
    (1..=6)
        .find_map(|distance| {
            [-distance, distance].into_iter().find_map(|semitones| {
                note.transpose(semitones)
                    .filter(|candidate| scale.contains(candidate.pitch_class()))
            })
        })
        .unwrap_or(note)
}
```

Two `find_map`s, one inside the other: the outer one tries distances 1,
2, 3..., and for each distance the inner one tries below, then above.
Because `-distance` comes first in the array, the lower note wins a tie.
The rule is in the *order of the array*, which is easy to miss, so the
doc comment and a test both say it out loud:

```rust
// C stays. C#, F# and Bb are each one semitone from two notes of
// the key, so the lower one wins: C, F and A.
assert_eq!(pitches(&snap_to_scale(&notes, C_MAJOR)), [60, 60, 65, 69]);
```

`snap_to_scale` never fails, so it returns a plain `Vec`: a note that
cannot move (no note of the scale nearby inside the MIDI range) stays
where it is.

## Inversions

A chord is **inverted** by moving its lowest note up an octave, above
the others. C, E, G becomes E, G, C (the first inversion), and doing it
again gives G, C, E (the second). The notes, and so the chord, stay the
same; what changes is the note in the bass, and that changes the colour
of the chord a lot. The chord lane writes it with a slash: E, G, C is
"C/E", C major over E.

```rust
pub fn inversion(pitches: &[Note], times: usize) -> Vec<Note> {
    let mut notes = pitches.to_vec();
    notes.sort();
    for _ in 0..times % notes.len().max(1) {
        let (Some(&lowest), Some(&highest)) = (notes.first(), notes.last()) else {
            break;
        };
        let Some(moved) = above(lowest, highest) else {
            break; // no room above: leave the chord as it is
        };
        notes.remove(0);
        notes.push(moved);
    }
    notes
}
```

- `times % notes.len()`: a chord of three notes inverted three times is
  back where it started, an octave higher. Rather than climb octaves, the
  function inverts `times` modulo the number of notes. The `.max(1)` is
  there because `% 0` panics in Rust, and an empty chord must not crash
  anything: `0..0` simply does no loops.
- `let (Some(&lowest), Some(&highest)) = (...) else { break; }` puts two
  `Option`s in a tuple and matches both at once: "if there is a first and
  a last note". The `&` in the pattern copies each `Note` out of the
  reference that `first()` returns.
- `above(lowest, highest)` moves a note up by octaves until it is higher
  than `highest`. For C, E, G, the C goes up one octave, above the G. For
  C, E, G, C' (with the octave on top), it has to go up two octaves to
  get above C'.

`invert`, the tool in the menu, uses `chords` to find every chord in
the notes it is given and inverts each one once, up or down. Down is the
mirror image: the highest note goes below the lowest. After moving a
note it sorts the chord again by pitch, because the moved note is now at
the wrong end of the list: the first test of this function caught
exactly that, with the moved note still first.

## Arpeggios: `cycle`, `step_by` and `zip`

An **arpeggio** plays the notes of a chord one after the other instead
of together. "Up" plays C, E, G, C, E, G..., "down" G, E, C..., and "up
and down" C, E, G, E, C, E, G, E...: the top and bottom notes are not
played twice in a row.

The order is an enum, with a method that says which note comes next,
counted from the lowest:

```rust
fn indices(self, count: usize) -> impl Iterator<Item = usize> {
    let once: Vec<usize> = match self {
        ArpeggioOrder::Up => (0..count).collect(),
        ArpeggioOrder::Down => (0..count).rev().collect(),
        ArpeggioOrder::UpDown => (0..count)
            .chain((1..count.saturating_sub(1)).rev())
            .collect(),
    };
    once.into_iter().cycle()
}
```

For three notes, "up and down" is `0, 1, 2` chained with `1` (the range
`1..2` reversed): `0, 1, 2, 1`. Then `cycle` repeats that **forever**:
`0, 1, 2, 1, 0, 1, 2, 1, 0...`. An endless iterator sounds dangerous, but
iterators are lazy: nothing is computed until someone asks for the next
item, so "forever" costs nothing as long as the caller stops asking.

The return type `impl Iterator<Item = usize>` means "some iterator of
`usize`s; which one is my business". The real type is
`Cycle<vec::IntoIter<usize>>`, which the caller does not need to know.

The arpeggio itself fills the time the chord lasted:

```rust
let start = chord[0].start;
let mut end = chord.iter().map(NoteEvent::end).max().unwrap_or(start + 1);
if let Some(next) = chords.get(index + 1) {
    end = end.min(next[0].start);
}
for (step, pick) in (start..end).step_by(rate).zip(order.indices(chord.len())) {
    result.push(NoteEvent {
        start: step,
        length: rate.min(end - step),
        ..chord[pick]
    });
}
```

- The arpeggio lasts as long as the longest note of the chord, but stops
  where the next chord (or note) starts, so it never runs into it.
- `(start..end).step_by(rate)` gives every `rate`-th step: with a chord
  from 0 to 8 and a rate of 2, the steps 0, 2, 4 and 6.
- `zip` pairs those steps with the endless note order: `(0, C)`,
  `(2, E)`, `(4, G)`, `(6, C)`. A zip stops as soon as *either* side runs
  out, and the steps always run out first. That is what makes the
  endless `cycle` safe.
- `rate.min(end - step)` shortens the last note if the chord ends in the
  middle of it.
- `..chord[pick]` copies everything else from the chosen note, so each
  note of the arpeggio keeps the velocity it had in the chord.

The result is a plain list of notes. Single notes in the input are passed
through untouched, so arpeggiating a whole channel only changes its
chords.

## From new notes to the pattern: `replace_notes`

A tool's result has to replace the notes it was given. A pattern keeps
one rule about its notes, since step 3: no two notes of a channel start
on the same step with the same pitch (they would sound as one, louder,
and could never be told apart). Replacing a group must keep that rule,
and do it all at once, so `Pattern` has a method for exactly that:

```rust
pub fn replace_notes(&mut self, channel: usize, old: &[NoteEvent], new: &[NoteEvent]) -> bool
pub fn can_replace_notes(&self, channel: usize, old: &[NoteEvent], new: &[NoteEvent]) -> bool
```

Both are built on one private method that works out the result without
changing anything:

```rust
fn replaced(&self, channel: usize, old: &[NoteEvent], new: &[NoteEvent]) -> Option<Vec<NoteEvent>> {
    let row = self.rows.get(channel)?;
    if !old.iter().all(|note| row.contains(note)) {
        return None;
    }
    let mut notes: Vec<NoteEvent> = row
        .iter()
        .filter(|note| !old.contains(note))
        .copied()
        .collect();
    let new = tidy_notes(new.iter().copied());
    let lands_on_another = new.iter().any(|note| {
        notes
            .iter()
            .any(|other| other.pitch == note.pitch && other.start == note.start)
    });
    if lands_on_another {
        return None;
    }
    notes.extend(new);
    Some(notes)
}
```

It refuses (returns `None`) in two cases: when one of the `old` notes is
not in the channel any more, and when a new note would land on a note
that stays. `replace_notes` then either stores the whole result or
changes nothing; there is no half-done replacement. `can_replace_notes`
asks the same question without changing anything, so the piano roll can
check first, and simply not move a selection onto other notes.

`tidy_notes` cleans up the new notes before they go in: each note is
fitted to the grid (shortened if it runs past the end of the longest
pattern, 64 steps, and left out if it starts beyond it), and of several new
notes with the same pitch and start, only the first is kept. That second
case is easy to reach: snap C and C# played together to C major, and
both become C. Rather than make every tool careful, the pattern tidies
whatever it is given.

Why does `old` hold notes and not positions in the list? Because the
piano roll's selection holds notes too, for reasons the
[piano roll chapter](piano-roll.md) explains. Finding a note by value in
a list of a few hundred is fast enough to do many times per frame.
