# Inserting and deleting bars

Files: [`crates/lineup-core/src/bars.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/bars.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs),
[`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/sections.rs)
and [`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs)

A chart you are writing is rarely right the first time. The bridge needs
one more bar before the turnaround; a transcription has a bar too many
where two lines of the score were copied twice. Until now the only way
to fix that was to move every clip, marker and section after the place
by hand, and to hope the band's parts and the chords still lined up.

Now a bar can be inserted or deleted in one click, and the whole song
after it moves along:

```text
before:  | C    | F    | G7   | C    |
insert a bar after bar 2:
after:   | C    | F    | %    | G7   | C    |
```

Right-click a bar on the **chord sheet**, or a bar number on the
**playlist's ruler**, and the menu has:

- **Insert a bar before**
- **Insert a bar after**
- **Delete this bar**, or **Delete bars 5–8** when several bars are
  selected on the sheet.

Each one is a single step to undo ("Undo insert bar").

![The menu on bar 3 of Embraceable You: type in it, clear it, move the playhead there, insert a bar before or after it, or delete it.](../images/step-9-insert-bar-menu.png)

## Trying it

1. Open **File > New**, which gives an empty chord sheet with a band,
   and type `C`, `F`, `G7` and `C` in the first four bars.
2. Right-click the F and pick **Insert a bar after**. A new bar 3
   appears, with the F written faintly in it: the F goes on through the
   empty bar, as `%` does in a printed chart. The G7 is now in bar 4.
3. Type `D7` in the new bar. The band plays C, F, D7, G7, C.
4. Right-click the D7 and pick **Delete this bar**. The song is as it
   was.
5. On the playlist, right-click the bar number **2** on the ruler and
   pick **Insert a bar before**. The clips, sections and markers after
   bar 1 all move a bar later.

![After Insert a bar after on bar 3 of Embraceable You: the new bar 4 is empty, so the Am7 goes on through it, written faintly, and the D7 has moved to bar 5. The A section's pattern is played elsewhere in the song too, so its clip was cut in two, A (1) and A (2), and its drums with it.](../images/step-9-insert-bar-after.png)

## What an inserted bar is

A new bar is **empty**, and empty means "nothing changes here": the
harmony going on before it goes on through it, and so do the key, the
tempo and the band's variation. That is what an empty bar in a chart
means too, and it is why the menu doesn't ask what to put in it.

It is in the **time signature** of the bar it is inserted next to, and
in that bar's **section**. A bar inserted before the first bar of a
waltz section is a waltz bar, even though the bar before it is in 4/4:
you were looking at the waltz bar when you asked for one more like it.

**Everything after it moves a bar later**: clips, sections, the loop,
the time, tempo, key and chord markers, and the band's variations and
section styles.

## What a deleted bar takes with it

A deleted bar takes its notes and its chords, and everything after it
moves a bar earlier. The question is what happens to **what was going
on at its end**. Take C, F, G7, C with the key changing to G major on
bar 3 and the tempo to 90 on bar 4, and delete bar 3. The markers in
bar 3 go with it, but the key of G major was meant to go on afterwards,
and still should. So before anything moves, `close_bars` reads what was
going on at the end of the deleted bars:

```rust
// What goes on at the end of the bars goes on from their start.
let time = meter.time_at(end);
let tempo = self.tempo_map().tempo_at(end);
let key = self.key_at(end);
```

and afterwards sets each of them again where the bars were. Markers that
say the same as the one before them are then tidied away, so a delete in
the middle of a long stretch in one key doesn't leave a key marker
behind.

The harmony is the subtle one. If a G7 starts in a deleted bar and is
held into the next one (the next bar has no chord of its own), the G7
should go on there. But if the bar before the deleted one is a G7
already, writing another G7 would be a second marker saying the same
thing. The code checks both:

```rust
if let Some(marker) = going_on
    && self.playlist.chord_markers.iter().all(|m| m.start != start)
    && start
        .checked_sub(1)
        .and_then(|step| self.chord_at(step))
        .is_none_or(|before| before.harmony != marker.harmony)
{
    self.playlist
        .chord_markers
        .push(ChordMarker::new(start, marker.harmony));
}
```

This is a **let chain**: `if let` and plain conditions joined with
`&&`, stable since Rust 1.88 (with edition 2024). Each part can use what
an earlier `let` bound: `marker` is used in the last line of the
condition. Before let chains, this needed an `if let` with a nested `if`
inside it.

`start.checked_sub(1)` is `None` for the first bar, where there is no
"before"; `and_then` passes `None` through and otherwise looks up the
chord one step earlier; and `is_none_or` says yes when there is no
chord before, or when the one there differs.

## Moving every step: one closure

Inserting uses `Project::make_room(at, steps)`, which the song already
had for copying a section and for putting an intro in front: every clip, section and marker at or
after `at` moves `steps` later. Deleting needs the opposite, and one
more case: what was **inside** the deleted steps. One closure says where
any step of the song goes:

```rust
// Where a step of the song goes: the deleted bars close up.
let place = |step: usize| match step {
    step if step <= start => step,
    step if step >= end => step - steps,
    _ => start,
};
```

Steps before the deleted bars stay; steps after them move `steps`
earlier; steps inside them land where the bars were. Then it is applied
to everything that has a place in the song. A section's end goes
through `place` too, so a section that contained the bars gets shorter,
and one made only of them gets a length of 0 and is removed. The loop
is kept only if something is left of it:

```rust
playlist.loop_region = playlist.loop_region.and_then(|region| {
    let (from, to) = (place(region.start), place(region.end));
    (from < to).then_some(crate::LoopRegion::new(from, to))
});
```

`bool::then_some` turns `true` into `Some(value)` and `false` into
`None`: here "the loop if it still has a length".

## The notes: open the pattern, or cut the clip

Clips are the tricky part. A clip plays a **pattern**, and the notes are
in the pattern, not in the clip. Inserting a bar in the middle of a clip
has to make room in the notes it plays. Two cases:

- **The pattern is played by this clip only, once.** Then the pattern
  itself can change: it gets the empty steps (`opened`) or loses the
  deleted ones (`closed`), and the clip simply plays the longer or
  shorter pattern.
- **The pattern is played elsewhere too**, by another clip or by this
  one repeated. Then the pattern must not change, or the other places
  would change with it. The clip is **cut in two** where the bar goes in
  or comes out, and each half plays a pattern of its own: "Riff (1)"
  and "Riff (2)".

![The playlist after the insert: A (1) plays bars 1 to 3, bar 4 is empty, and A (2) plays the rest of the old A from bar 5, with the drums cut the same way.](../images/step-9-insert-bar-playlist.png)

The check is `only_clip`:

```rust
/// Whether clip `index` plays its pattern once and no other clip
/// plays it, so the pattern can change with it.
fn only_clip(&self, index: usize) -> bool {
    let clip = self.playlist.clips[index];
    clip.repeats == 1
        && self
            .playlist
            .clips
            .iter()
            .enumerate()
            .all(|(other, playing)| other == index || playing.pattern != clip.pattern)
}
```

`enumerate` pairs each clip with its index, so `all` can skip the clip
itself (`other == index`) and ask about every other one.

Cutting a clip that is **repeated** is gentler. A cut that falls between
two repeats needs no new pattern at all: a clip playing a one-bar groove
four times, with its second bar deleted, becomes a clip playing it once
and a clip playing it twice. Only a cut in the middle of one repeat
needs that repeat's two halves as patterns.

Notes held across the place are **cut short** there, and the notes after
it move:

```rust
/// `notes` with `steps` empty steps put in at step `at`: the notes from
/// there on move later, and a note held across it ends there.
fn opened(notes: &[NoteEvent], at: usize, steps: usize) -> Vec<NoteEvent> {
    let at_tick = at * TICKS_PER_STEP;
    notes
        .iter()
        .map(|note| match note.start_tick() < at_tick {
            true => cut_short(note, at_tick),
            false => NoteEvent {
                start: note.start + steps,
                ..*note
            },
        })
        .collect()
}
```

A note held over the new bar is cut where the bar starts, rather than
held through it, because the new bar is meant to be empty: you will
write something there, and a note from before sounding under it would
be a surprise. `NoteEvent { start: …, ..*note }` is **struct update
syntax**: a copy of `*note` with `start` changed. It works with `*note`
because `NoteEvent` is `Copy`.

The halves of a cut pattern keep its **colour**, so the clips look as
they did on the playlist, and a half that is only cut off to be deleted
is removed again at the end, so a delete leaves no empty patterns
behind.

## All or nothing

Two things can make an insert or a delete impossible: the song would
grow past its longest length, or there is no room for the patterns a
cut needs (a song holds at most 999). Either is found out halfway
through, after some clips have already changed. So both start by
keeping a copy of the project:

```rust
let before = self.clone();
if !self.open_clips(at, steps) || !self.make_room(at, steps) {
    *self = before;
    return false;
}
```

and put it back if anything fails. `*self = before` replaces the whole
project, moving `before` into it. The app then says "There is no room
for another bar in the song" or "There is no room for the patterns the
cut needs", and nothing has changed.

`||` stops at the first `true`, so `make_room` runs only if
`open_clips` succeeded. `make_room` checks before it moves anything,
but `open_clips` can fail on the third clip after cutting the first
two; the copy covers that.

## The band follows

The band's parts (piano, bass, drums) are written from the chord sheet.
Moving everything by a bar moves their notes too, and the app must not
mistake that for an edit of yours: a part you edit is **frozen**, kept
as you left it, and no longer follows the sheet. So both methods run
inside `moving_the_band`, which the tools that transpose or re-meter
the whole song use too:

```rust
pub fn insert_bars(&mut self, bar: usize, after: bool, count: usize) -> bool {
    self.moving_the_band(|project| project.open_bars(bar, after, count))
}
```

`moving_the_band` takes the closure as `impl FnOnce(&mut Project) -> R`
and runs it on the project, then takes the moved parts as the band's
own. In the app, the edit goes through the same `edit` method as every
other change: it keeps the project as it was for undo, runs the change,
lets the band play the new sheet, and tells the audio thread what
changed.

## The menu, in two places

Both menus are the same function, so they always offer the same items
with the same hints:

```rust
/// The menu items that insert a bar next to bar `bar` or delete it, or
/// delete the bars of `selection` when it holds more than one.
pub fn bar_edits(ui: &mut Ui, bar: usize, selection: Option<(usize, usize)>) -> Option<BarEdit> {
```

It draws the buttons and returns what was clicked, if anything, as a
value:

```rust
pub enum BarEdit {
    /// An empty bar next to bar `bar`, after it or before it.
    Insert { bar: usize, after: bool },
    /// Bars `first` to `last` taken out.
    Delete { first: usize, last: usize },
}
```

The chord sheet wraps it in its own action, `SheetAction::Bars`, and the
playlist in `PlaylistAction::Bars`. Neither changes the project while
drawing; the app applies the action afterwards with `edit_bars`, which
also selects the new bar on the sheet so you can type in it at once.
Most views in the app work this way: draw, collect actions, apply
them after.

## The tests

The tests in `bars.rs` work on a small song: one pattern of four bars
playing C, F, G7, C, a verse over all of it, a loop over bars 2 and 3,
the key of G from bar 3 and 90 BPM from bar 4.

- `an_inserted_bar_moves_everything_after_it` inserts after bar 2 and
  checks every note, chord, key, tempo, the verse and the loop: the
  note held into the new bar now ends where it starts.
- `a_bar_inserted_before_a_change_of_time_takes_the_new_time` makes bar
  3 a waltz bar and inserts before it: the new bar is twelve steps long.
- `a_clip_of_a_pattern_played_elsewhere_too_is_cut_in_two` checks
  "Riff (1)" and "Riff (2)", and that the other clip's "Riff" is
  untouched.
- `a_deleted_bar_closes_up`, `a_deleted_bar_leaves_no_pattern_behind`,
  `a_harmony_from_a_deleted_bar_goes_on_where_it_was` and
  `a_repeated_clip_loses_the_repeats_in_the_deleted_bars` do the same
  for deleting.
- `the_band_follows_the_bars` inserts and deletes in a band song and
  checks that no part was frozen.

In the app, `a_bars_menu_inserts_a_bar_and_deletes_it` right-clicks the
G of C, G, Am, F, inserts a bar after it, types D7 into it, and deletes
it again; `the_bar_numbers_insert_and_delete_bars_too` does it from the
playlist's ruler and undoes it with Ctrl+Z.

The tests check the song's data. To hear that nothing else moved, the
[playback trace](playback-trace.md) of a demo before and after an
insert shows every note before the bar at the same time, and every note
after it exactly one bar later.

## Exercises

1. `insert_bars` takes a `count`, but the menu always inserts one bar.
   Add **Insert bars…** with a number. Which part of `open_bars` already
   handles more than one?
2. Write a test that inserts a bar in the middle of the second repeat
   of a clip repeated three times. Before running it, write down the
   clips you expect, with their starts, repeats and patterns.
3. `close_bars` reads the time, tempo and key at the end of the deleted
   bars **before** moving anything. What would go wrong if it read them
   afterwards?
