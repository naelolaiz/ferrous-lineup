# Editing tools as plain functions

Files: [`crates/lineup-core/src/tools.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tools.rs)
and [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

The piano roll's **Tools** menu changes many notes at once: it moves
them up a semitone, an octave or a note of the key, earlier or later,
onto the grid or into the key, inverts chords and turns them into
arpeggios. Each tool is a function in `lineup_core::tools` that takes
some notes and returns new ones. None of them knows about windows,
selections or undo, which is what makes them easy to read and to test.
This chapter goes through them one by one; how the menu and the arrow
keys pick a tool is in [The editors: channel rack, piano roll and
playlist](../program/editors.md#one-enum-for-the-menu-and-the-keys).

## The music, briefly

**Transposing** moves music up or down without changing its shape:
every interval stays the same, so a melody moved up a tone is the same
melody, higher. C, E, G up a tone is D, F♯, A: still a major chord.

Moving notes **in the key** is different. Each note goes to the next
note of the *scale*, not up a fixed number of semitones, so the
intervals change to fit the key. In C major, C, E, G up one note of the
key is D, F, A: the chord on the next degree, D minor, not D major.
Composers have always used this: a short figure repeated a note higher
each time, staying in the key, is a **sequence**, and Baroque music is
full of them. Copy a phrase, paste it later and move it up a note in
the key, and you have written one.

**Quantizing** moves notes onto the grid. A person never plays exactly
on the beat, and a MIDI file of a performance keeps every small
difference; quantizing makes the notes as exact as a drum machine.

An **inversion** of a chord has the same notes with another one in the
bass, as [Chords: building, naming and
inversions](chords.md#inversions) explains. An **arpeggio** plays
a chord's notes one after another, the way a guitarist picks a chord or
a harp plays one, instead of all at once.

## Notes in, notes out

Every tool has the same shape. It borrows the notes as a slice,
`&[NoteEvent]`, and returns new ones; it changes nothing itself. The
module says so at the top:

```rust
//! Each tool is a plain function: it takes some notes and returns new
//! ones, and changes nothing itself. The piano roll hands the result to
//! [`Pattern::replace_notes`](crate::Pattern::replace_notes), through
//! the app's `edit`, so every tool can be undone like any other change.
```

The piano roll picks the function for each tool, with a name for the
undo history:

```rust
let (label, new) = match tool {
    // ...
    Tool::Transpose(semitones) => {
        ("transpose notes", tools::transpose(&targets, semitones))
    }
    Tool::InKey(steps) => (
        "move notes in key",
        tools::transpose_in_key(&targets, scale, steps),
    ),
    Tool::Shift(steps) => ("move notes", tools::shift(&targets, steps, length)),
    Tool::Quantize => ("quantize notes", Some(tools::quantize(&targets, length))),
    Tool::SnapToKey => (
        "snap notes to key",
        Some(tools::snap_to_scale(&targets, scale)),
    ),
    Tool::Invert { up } => ("invert chords", tools::invert(&targets, up)),
    // ...
};
```

`targets` is the selection, or every note of the channel when nothing
is selected. The new notes then replace the old ones in one
`ReplaceNotes` action, which is one step of undo; how that works, and
why the selection holds notes rather than positions, is in [The
editors: channel rack, piano roll and
playlist](../program/editors.md#the-selection-holds-notes-not-positions).
Keeping the tools out of the app also means the same functions serve
other callers: the twelve-tone tests group a written row into chords
with `tools::chords`, and the MIDI keyboard snaps keys with
`tools::nearest_in_key`.

Most tools build each new note from an old one with **struct update
syntax**:

```rust
Some(NoteEvent {
    pitch: note.pitch.transpose(semitones)?,
    ..*note
})
```

`..*note` means "every other field from `*note`": the start, the
length, the velocity, the nudge and the trim stay as they were, and only
the pitch is new. `NoteEvent` is `Copy`, so `*note` copies the old note
out of the slice without taking it away. If a field is added to
`NoteEvent` one day, every tool keeps it without a change.

## Grouping notes into chords: `chords`

Inverting and arpeggiating work on chords, so the first tool finds
them. Here a chord is simply the notes that start on the same step:

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

Sorting by the tuple `(note.start, note.pitch)` sorts by start, and by
pitch among notes that start together, so each chord comes out lowest
note first. `chunk_by` then cuts the list into runs, as the chord lane
does (see [The chord lane: `chunk_by`, then
measure](../program/editors.md#the-chord-lane-chunk_by-then-measure)).

Each run is a slice that borrows from `sorted`, and `sorted` is dropped
when the function returns. Returning the slices would leave them
pointing at freed memory, which the borrow checker refuses, so each one
is copied into a `Vec` of its own. `<[NoteEvent]>::to_vec` is the slice
method `to_vec` named as a function, so `map` can take it directly
instead of `|run| run.to_vec()`. The angle brackets are needed because
`[NoteEvent]` is a type, not a name: `[NoteEvent]::to_vec` would not
parse.

## When a tool cannot work: collecting `Option`s

Transposing a chord near the top of the keyboard can push a note past
MIDI note 127. Leaving that note where it is would break the chord;
dropping it would lose a note. So the tool does neither: it refuses the
whole move.

```rust
pub fn transpose(notes: &[NoteEvent], semitones: i32) -> Option<Vec<NoteEvent>> {
    // Collecting an iterator of `Option`s into `Option<Vec<_>>` gives
    // `None` as soon as one item is `None`, and the whole `Vec` otherwise.
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

`Note::transpose` returns `None` for a note off the keyboard (see
[Pitch classes, notes and
spelling](pitch.md#moving-a-note-option-for-it-might-not-exist)).
The `?` inside the closure returns `None` from the *closure*, not from
`transpose`, so the iterator yields an `Option<NoteEvent>` for each
note. `collect` can build many kinds of collection, and the return type
asks for `Option<Vec<NoteEvent>>`: that collection stops at the first
`None` and is `None` itself, or holds every note. `Result` works the
same way: collected into a `Result<Vec<_>, E>`, it stops at the first
error.

The piano roll does nothing when a tool returns `None`, so pressing ↑
on a chord that has a note in the top row simply does not move it:

```rust
let Some(new) = new.filter(|new| fits_view(new)) else {
    return;
};
```

`Option::filter` turns `Some` into `None` when the test fails; here the
test is that every note fits the rows the piano roll shows, which are
narrower than the 128 notes of MIDI.

### Moving in time: `checked_add_signed` and `then_some`

← and → move notes a step earlier or later, and with Shift four steps,
a beat of 4/4.
Starts are `usize`, which cannot be negative, while the move can go
either way, so it is an `isize`:

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

`usize::checked_add_signed` adds a signed number to an unsigned one and
returns `None` if the result would fall below zero, so a note at step 0
cannot move earlier. `then_some` turns the `bool` into an `Option`:
`Some` of the moved note if it still starts inside the pattern, `None`
if it would start at its end or after. Either way the whole selection
stays where it was, with the same `collect` as before.

### Back onto the grid: `quantize`

**Quantize to the steps** moves each note's start and end to the
nearest step. A note keeps its exact place as a step plus a `nudge` and
a `trim`, in ticks, 24 to a step (see [The project: patterns, channels,
tracks, clips and markers](../program/project.md#notes-steps-and-patterns)),
and the rounding is on the note itself:

```rust
/// The note moved to the nearest steps, both its start and its end:
/// what *quantizing* does. A note shorter than a step keeps one.
pub fn quantized(self) -> NoteEvent {
    let nearest = |tick: usize| (tick + TICKS_PER_STEP / 2) / TICKS_PER_STEP;
    let start = nearest(self.start_tick());
    let end = nearest(self.end_tick()).max(start + 1);
    NoteEvent {
        velocity: self.velocity,
        ..NoteEvent::new(self.pitch, start, end - start)
    }
}
```

Integer division rounds down, so adding half a step first rounds to the
nearest step. `NoteEvent::new` makes a note on the grid, with no nudge
and no trim, and the struct update takes everything from it but the
velocity. The tool adds one rule: a note that would now start at the end
of the pattern starts on its last step instead.

```rust
pub fn quantize(notes: &[NoteEvent], pattern_length: usize) -> Vec<NoteEvent> {
    let last = pattern_length.saturating_sub(1);
    notes
        .iter()
        .map(|note| {
            let quantized = note.quantized();
            let start = quantized.start.min(last);
            NoteEvent {
                start,
                length: quantized.end() - start,
                ..quantized
            }
        })
        .collect()
}
```

Quantizing is a choice, not something the app does on its own, because
it can change the music. Eighth-note triplets start every 32 ticks, a
step and a third apart, and no grid of sixteenths holds them: quantized,
they land on steps 0, 1 and 3 and are no longer even. That is why
notes imported from a MIDI file keep their nudge until you ask. (Notes
you record are put on the grid as they are recorded, by a function of
their own: see [MIDI: messages, devices, recording and
files](../sound/midi.md#quantizing-recorded_note).)

## Staying in the key

Two tools work with the key: one moves notes a note of the key up or
down, the other moves every note that is outside the key into it. Both
use the key the song is in where the pattern plays, after key markers
(see [Scales, modes and keys](scales.md#the-key-of-a-pattern)).

### One note of the key up: `find_map`

The next note of the key above a note is the first note, a semitone
higher, then two, and so on, that is in the scale:

```rust
pub fn step_in_key(note: Note, scale: Scale, up: bool) -> Option<Note> {
    let direction = if up { 1 } else { -1 };
    (1..=12).find_map(|distance| {
        note.transpose(direction * distance)
            .filter(|candidate| scale.contains(candidate.pitch_class()))
    })
}
```

`find_map` calls the closure for each distance and stops at the first
`Some`. The closure transposes, which can fail at the edge of the
keyboard, and `filter` keeps the note only if the scale contains it.
Twelve semitones reach every pitch class, so if the scale has any note
at all, one is found: at worst the same note an octave away, when the
scale holds nothing else. A range is lazy, and `find_map` stops at the
first hit, so a step in C major tries one or two distances, not
twelve.

Moving several notes of the key is that step, repeated:

```rust
pub fn transpose_in_key(notes: &[NoteEvent], scale: Scale, steps: i32) -> Option<Vec<NoteEvent>> {
    notes
        .iter()
        .map(|note| {
            let mut pitch = note.pitch;
            for _ in 0..steps.unsigned_abs() {
                pitch = step_in_key(pitch, scale, steps > 0)?;
            }
            Some(NoteEvent { pitch, ..*note })
        })
        .collect()
}
```

`unsigned_abs` gives the size of `steps` as an unsigned number, so the
loop runs as many times going down as going up. A note outside the key
lands on the nearest note of the key in the direction of the move: C♯
up one step in C major is D. **Alt+↑** and **Alt+↓** use this tool.

### The nearest note of the key: snapping

Snapping looks both ways at once:

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

For each distance, the inner `find_map` tries the note below before the
one above, so when two notes of the key are as near, the lower one wins:
in C major, C♯ snaps to C, F♯ to F and B♭ to A. A rule is needed, since
neither is more correct, and this one is easy to remember. Six semitones
up or down reach all twelve pitch classes, so any scale with a note in
it is found; only at the very ends of the keyboard can both tries fail,
and `unwrap_or(note)` then leaves the note where it is. Snapping never
refuses the way a move does; at worst a note stays put.

`snap_to_scale` maps it over many notes, for **Snap to the key** in the
Tools menu:

```rust
pub fn snap_to_scale(notes: &[NoteEvent], scale: Scale) -> Vec<NoteEvent> {
    notes
        .iter()
        .map(|note| NoteEvent {
            pitch: nearest_in_key(note.pitch, scale),
            ..*note
        })
        .collect()
}
```

The same two functions do the piano roll's **Snap to scale** button: a
note you draw lands on `nearest_in_key` of the row you clicked, and a
group you drag is snapped with `snap_to_scale` once it has moved (see
[Moving a group](../program/editors.md#moving-a-group)). A key
played on the typing keyboard or a MIDI keyboard is snapped the same way
(see [MIDI: messages, devices, recording and
files](../sound/midi.md#snapping-keys-to-the-key)).

## Inversions

**Invert chords up** moves each chord's lowest note above its highest;
**Invert chords down** moves the highest below the lowest. Both move a
note by octaves until it passes the other end of the chord:

```rust
/// `note` moved up by octaves until it is above `ceiling`.
fn above(note: Note, ceiling: Note) -> Option<Note> {
    let mut note = note;
    while note <= ceiling {
        note = note.transpose(12)?;
    }
    Some(note)
}
```

"Until it is above", not "one octave up": in C, E, G, C the low C must
go up two octaves to be above the high C. The `?` gives up if the note
would leave the keyboard.

```rust
pub fn invert(notes: &[NoteEvent], up: bool) -> Option<Vec<NoteEvent>> {
    let mut result = Vec::with_capacity(notes.len());
    for mut chord in chords(notes) {
        if let (Some(&lowest), Some(&highest)) = (chord.first(), chord.last())
            && chord.len() >= 2
        {
            if up {
                chord[0].pitch = above(lowest.pitch, highest.pitch)?;
            } else {
                let last = chord.len() - 1;
                chord[last].pitch = below(highest.pitch, lowest.pitch)?;
            }
            chord.sort_by_key(|note| note.pitch);
        }
        result.extend(chord);
    }
    Some(result)
}
```

`chords` returns each chord lowest note first, so `first` and `last` are
the bass and the top. They return `Option`s, because an empty `Vec` has
neither, and matching a tuple of both takes them in one pattern. The `&`
in `Some(&lowest)` copies the note out of the reference that `first`
returns, so `lowest` and `highest` are plain `NoteEvent`s rather than
borrows of `chord`. The `&&` makes a **let chain**: the body runs only if both notes exist *and*
the chord has two notes or more, so a single note is left alone.
`for mut chord` makes each chord a variable that can be changed, without
a `let mut` inside the loop.

`inversion`, a sibling for a list of pitches, gives a chord in any
inversion at once. The piano roll's chord stamp uses it for the
inversion menu next to **Draw** (see [Chords: building, naming and
inversions](chords.md#chord-stamps)):

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

A chord of three notes has three positions, so turning it three times
is the root position again: `times % notes.len()` skips the full turns.
`.max(1)` keeps an empty list from dividing by zero, which would panic.
The `let ... else` takes the two ends or leaves the loop. Unlike
`invert`, this one does not refuse when a note cannot go higher: a stamp
near the top of the keyboard still draws a chord, in the last inversion
that fits.

## Arpeggios: `cycle`, `step_by` and `zip`

**Arpeggiate chords** plays each chord's notes one after another, for as
long as the chord lasted. The Tools menu picks the order (**up**,
**down**, **up and down**) and how long each note is (**1 step**,
**2 steps**, **1 beat**). The order is an enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArpeggioOrder {
    #[default]
    Up,
    Down,
    UpDown,
}
```

`#[derive(Default)]` on an enum needs to be told which variant is the
default, with `#[default]`. Each order knows which note of the chord
comes next, counted from the lowest, forever:

```rust
fn indices(self, count: usize) -> impl Iterator<Item = usize> {
    let once: Vec<usize> = match self {
        ArpeggioOrder::Up => (0..count).collect(),
        ArpeggioOrder::Down => (0..count).rev().collect(),
        ArpeggioOrder::UpDown => (0..count)
            .chain((1..count.saturating_sub(1)).rev())
            .collect(),
    };
    // `cycle` repeats an iterator forever. It is lazy, so "forever"
    // costs nothing: the caller takes only as many as it needs.
    once.into_iter().cycle()
}
```

For three notes, up and down is 0, 1, 2, 1, and then again from 0: the
top and bottom notes are played once on each turn, never twice in a
row. `saturating_sub` stops at zero instead of going below it, so
`count - 1` cannot underflow the `usize` for an empty chord. The return type `impl Iterator<Item = usize>` says "some
iterator of `usize`" without naming its type, which here would be
`Cycle<std::vec::IntoIter<usize>>`; callers only need to know they can
iterate it.

```rust
pub fn arpeggiate(notes: &[NoteEvent], order: ArpeggioOrder, rate: usize) -> Vec<NoteEvent> {
    let rate = rate.max(1);
    let chords = chords(notes);
    let mut result = Vec::new();
    for (index, chord) in chords.iter().enumerate() {
        if chord.len() < 2 {
            result.extend(chord);
            continue;
        }
        let start = chord[0].start;
        let mut end = chord.iter().map(NoteEvent::end).max().unwrap_or(start + 1);
        if let Some(next) = chords.get(index + 1) {
            end = end.min(next[0].start);
        }
        // `step_by` takes every `rate`-th step; `zip` pairs each of those
        // steps with the next note of the order, and stops when the steps
        // run out (the order never does).
        for (step, pick) in (start..end).step_by(rate).zip(order.indices(chord.len())) {
            result.push(NoteEvent {
                start: step,
                length: rate.min(end - step),
                ..chord[pick]
            });
        }
    }
    result
}
```

The arpeggio lasts until the chord's longest note ends, but stops where
the next chord or single note starts, so it never plays over it.
`map(NoteEvent::end)` passes the method as a function, as `to_vec` was
above; `max` returns an `Option`, because an empty list has no largest
item. `zip` stops when either side runs out, and the endless order never
does, so the steps decide how many notes there are. Each note is built
`..chord[pick]`, from the note of the chord it plays, so it keeps that
note's velocity: a loud top note stays loud in the arpeggio. The last
note is cut short if the chord ends in the middle of it
(`rate.min(end - step)`).

`result.extend(chord)` adds a single note as it is. `chord` is a
`&Vec<NoteEvent>`, and a `Vec` of a `Copy` type can be extended from
references, copying each item.

## Hearing what moved

After a tool that changes pitches, the piano roll plays the first chord
of the result, so you hear where it went without starting the song:

```rust
if matches!(
    tool,
    Tool::Transpose(_) | Tool::InKey(_) | Tool::Invert { .. }
) && let Some(first) = tools::chords(&new).into_iter().next()
{
    actions.push(RollAction::Preview {
        channel: self.channel,
        pitches: first.iter().map(|note| note.pitch).collect(),
    });
}
```

`matches!` asks whether a value fits a pattern and gives a `bool`; `_`
and `{ .. }` accept any value inside. It is a let chain again: the
preview is sent only for those tools *and* if there is a first chord.
`Preview` is one more action the app handles after drawing.

## Testing it

Because the tools are plain functions, the tests are plain too: a few
notes in, the expected notes out, no window. A helper makes a note from
a MIDI number, a start and a length:

```rust
fn note(midi: u8, start: usize, length: usize) -> NoteEvent {
    NoteEvent::new(Note::new(midi), start, length)
}
```

and the tests read like the music:

```rust
#[test]
fn moving_in_key_changes_the_chord() {
    // C major one step up in C major is D minor: D, F, A.
    let c_major = [note(60, 0, 4), note(64, 0, 4), note(67, 0, 4)];
    let up = transpose_in_key(&c_major, C_MAJOR, 1).unwrap();
    assert_eq!(pitches(&up), [62, 65, 69]);
    // Two steps down from C, E, G: A, C, E (A minor).
    let down = transpose_in_key(&c_major, C_MAJOR, -2).unwrap();
    assert_eq!(pitches(&down), [57, 60, 64]);
    // ...
}
```

The others check that one note too high spoils the whole transposition,
that a shift stays inside the pattern, that eighth-note triplets
quantize to steps 0, 1 and 3, that snapping moves only the notes outside
the key (and the lower note wins a tie), that inverting three times is
the root position again, that an arpeggio stops at the next chord and
keeps each note's velocity. The tests of `change_key`, the function the
**Key** menus use, which lives in the same file, are in [Scales, modes
and keys](scales.md#testing-it).

Exercises on the tools are in [Exercises: music theory in
code](exercises.md).
