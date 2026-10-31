# Exercises: the shape of the program

These exercises practise the Rust of this part on the real code. Each
one says what is **already there**, so you extend it rather than build
it again. Some are small pieces of work with a clear end; the ones
called "Think about it" are questions, with no code to write.

Try them on a branch of your own. Run `cargo test` after each change,
and `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` before you call
one finished: CI runs all three.

## Crates and threads

### 1. Break it on purpose (any crate)

Learning to read compiler errors is half of learning Rust. Try each of
these, read the error, then undo it:

- In `AudioEngine::new` (`engine.rs`), call
  `build_stream::<f32>(&device, config.clone(), audio_thread, Arc::clone(&problems))`
  twice in a row, so the second call fails because `audio_thread` was
  moved into the first.
- Remove `move` from the audio callback closure in `build_stream`.
- Add a variant `SetName(String)` to `Command`. Why can `Command` not be
  `Copy`? Which variants already make that impossible, and what does the
  compiler say when you ask for `Clone`?
- Make `Note`'s field `pub` (in `lineup-core/src/pitch.rs`) and write
  `Note(200)` in `app.rs`. Why does that compile, and why is it a bad
  idea?

*Practises:* ownership, `move` closures, `Copy` and non-`Copy` types,
privacy.

### 2. Think about it: the audio thread

- Why would `Command::InsertChannel { instrument: Box<Instrument>, ... }`
  be worse than sending the instrument by value? (`Instrument` already
  boxes its voice arrays; what would a second box buy?)
- What would happen if the window forgot to call `collect_garbage`?
- `Garbage` is one enum with a variant for each kind of thing thrown
  away. What would one trash queue per kind cost, and what would it
  gain?
- `AudioEngine::playhead` uses `Ordering::Relaxed`. Could the window
  ever see a step number that was never played?

## The project

### 3. Delete every empty track (lineup-core and lineup-app, easy)

Deleting one track is already there: read `Project::remove_track` and
the "Delete track" item in the track's right-click menu. It removes the
track and its clips, and moves the clips of the tracks below it up by
one, just as `remove_pattern` does for patterns.

What is missing is a way to remove *all* the empty tracks at once.
`remove_unused_tracks` only drops empty tracks at the bottom of the
playlist (it calls `trim_tracks`). Add `remove_empty_tracks`, which
removes a track wherever it is if it has no clips, and an item for it in
the track menu. Which indices must you visit, and in which order, so
that the clips keep pointing at the right tracks? What should happen if
that would leave no tracks? Write a test with the demo song first, then
the method.

*Practises:* `retain`, keeping rules that span two lists, testing the
order of index changes.

### 4. Think about it: positions or ids

Clips refer to patterns by their *position* in the list, which is why
`remove_pattern` has to renumber them. Imagine instead that every
pattern has a permanent id (`struct PatternId(u32)`) that never changes.

- Which methods of `Project` become simpler, and which harder?
- What would change in the file, and in the audio thread's
  `ChannelNotes`?
- Would undo be easier with ids or with positions?

## Telling the audio thread what changed

### 5. Random edits for the diff (lineup-core, medium)

The diff tests check hand-picked cases. Write one that checks a thousand
random ones: start from the demo, make a random edit (toggle a step, add
or remove a pattern or a channel, change a volume, move a clip...), call
`check_both_ways(&before, &after)`, and repeat from the new project.

For randomness without any crate, a tiny "xorshift" generator is enough
for tests:

```rust
struct Random(u64);

impl Random {
    fn next(&mut self, below: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % below as u64) as usize
    }
}
```

Start it from a fixed number (not 0, which stays 0 forever), so a
failure happens again every time you run the test. If you want to go
further, the `proptest` crate does this properly, and when it finds a
failing case it *shrinks* it to the smallest list of edits that still
fails.

Also try several random edits between two checks, as one undo step
might hold. Does `diff` ever produce a long list where a short one would
do? Is that a problem?

*Practises:* property-based testing, reproducible randomness.

### 6. Channels lined up by name (lineup-core, medium)

`diff` lines up the channels of two projects with
`|old, new| old == new`, and the patterns with `same_pattern`. Write
`same_channel(old: &Channel, new: &Channel) -> bool`, which says two
channels are the same when they have the same name, and pass it to
`Alignment::new` instead. Run the diff tests. Then find a case where it
gives the wrong answer: what happens with two channels called "Piano"?

*Practises:* functions as arguments, what "the same" means.

### 7. Think about it: what to send

- `SetNotes` sends a channel's whole list of notes for every change. At
  what size would that become a problem, and what would you send
  instead?
- The window sends the song's length along with its pieces, instead of
  letting the audio thread work it out. What would it take to keep two
  copies of that calculation in agreement, and how would you notice if
  they drifted apart? The pieces are `Play`s made by
  `Project::playback_arrangement`. Why is unrolling repeats on the
  window's side simpler than teaching the audio thread about sections?
- Undo while the song plays. When undo removes a channel, its ringing
  notes stop at once, because the channel goes to the trash. In the case
  of [three identical Lineup Osc channels](sync.md#when-the-story-is-ambiguous),
  the audio thread removes the *last* channel and gives its notes to
  another one. Which channel's ringing note keeps sounding, and could
  you ever hear the difference?

## Undo and redo

### 8. An undo history window (lineup-app, easy)

Add Edit > History…, a small window listing every step that can be
undone or redone, newest at the top, with the current position marked.
Clicking a step jumps there.

You need two methods on `History`: one giving the undo labels and one
the redo labels, each as an iterator (`self.undo.iter().map(|step|
step.label)`). Jumping back five steps is five calls to `history.undo`.
Should `sync_audio` run after each one, or once at the end, comparing
with the project from before the first? Which is cheaper, and does the
audio thread end up the same either way? (Exercise 5 answers the last
question.)

*Practises:* iterators over private fields, `rev()`, reusing `diff`.

### 9. Cheaper snapshots (lineup-core, harder)

Every undo step copies the whole project, every pattern included, even
though a click on a step changes one pattern. (A drag copies once, when
its step opens, not on every frame, but the copy is still the whole
project.) Make the copies cheap by sharing what did not change: store the
patterns as `Vec<Arc<Pattern>>`.

Cloning a `Vec<Arc<Pattern>>` copies only the pointers and adds one to
each count; the patterns themselves are shared between the current
project and every copy in the history. To change one, use
**`Arc::make_mut`**, which gives a `&mut Pattern` and first makes a
private copy *only if* the pattern is shared ("clone on write"). So
`Project::pattern_mut` becomes
`self.patterns.get_mut(index).map(Arc::make_mut)`.

Then `diff` can skip unchanged patterns quickly: if `Arc::ptr_eq` says
two patterns are the same memory, they are certainly equal. Make a test
project with 999 patterns full of notes, and measure the time of an edit
before and after with `std::time::Instant`. Does `PartialEq` for the
project still work unchanged? Does the file format need to change?

*Practises:* `Arc` for sharing, clone on write, measuring before
optimizing.

### 10. Think about it: labels and what undo covers

- Step labels are `&'static str`, so "rename pattern" cannot say which
  pattern. A `String` label would allow "rename pattern Beat", but most
  labels are fixed text, and a `String` would allocate for each of them.
  Look up `std::borrow::Cow<'static, str>`: how does it let a label be
  either, and allocate only when it must?
- The master volume is part of the project's mixer, so undo changes it
  back, and it is saved with the song. The selected pattern, the zoom
  and the open windows are not. Would you expect undo to change any of
  them? What would have to move into `Project` for that, and what would
  it mean for the saved file?

## Saving

### 11. A new project field and its format test (lineup-core, medium)

**Goal.** Add one harmless field to a project, save it without breaking
old files, and prove that a song which uses it is written as the next
format. `ProjectFile`, its serde defaults and the file tests are
already there; this is the smallest complete exercise in growing the
format.

**Files.** `crates/lineup-core/src/project.rs`,
`crates/lineup-core/src/file.rs`.

**Steps.**

1. Add `pub notes: String` to `Project`, initialized to `String::new()`
   in `Project::empty`.
2. Add `notes: String` to `ProjectFile`, with
   `#[serde(default, skip_serializing_if = "String::is_empty")]`, and
   copy it in both directions.
3. Raise `FORMAT_VERSION` by one, to 30 (if another exercise already
   took 30, use the next free number). In `format_for`, return the new
   number first when `!project.notes.is_empty()`; leave everything else
   as it is.
4. Write three tests in `file.rs`: an empty project stays at its old
   format and has no `notes:` line; a project with notes is the new
   format and round-trips; and a file written before the field existed
   reads back with empty notes.

**Run.** `cargo test -p lineup-core file`.

**Done when.** Old project texts still open, the new field survives a
round trip, and `format_for` names the new number only for a song that
needs it. `files_from_the_future_are_refused` builds its file from
`FORMAT_VERSION + 1`, so it keeps passing.

### 12. A recovery copy that is damaged (lineup-app, easy)

**Goal.** Make crash recovery trustworthy when the copy on disk is not.
`app/recovery.rs` already writes a project copy, an `.info` file and a
lock file for each window to the `recovery` folder in the app's data
folder, offers the copy at the next start, and removes a copy that will
not load. The test `a_recovery_copy_comes_back_with_its_file` covers the
happy path. Add the unhappy ones.

**Files.** `crates/lineup-app/src/app/recovery.rs`.

**Steps.**

1. See it by hand: open a demo, type a chord in the first bar, wait a
   moment, and force-quit the app. Do not use **File > Quit**. Start it
   again and choose **Recover it**. Close the recovered song without
   saving it.
2. Read `Files`, `load` and `write`, and the tests at the bottom of the
   file.
3. Add a test that takes `Files::of_run(&folder, "test")`, creates the
   folder, writes rubbish into its `project` file, puts a valid `info`
   file beside it, calls `load(files.clone(), None)`, and checks that it
   returns `None` **and** that both files are gone.
4. Add a second test that writes a valid project to `files.project`
   with `file::save`, but leaves out the info file. Check that `load`
   still returns the project, with no path and no name, so the dialog
   can call it "Untitled".
5. Remove the temporary folder at the end of each test, as the existing
   ones do.

**Run.** `cargo test -p lineup-app recovery`.

**Done when.** Both tests pass, and the first fails if the
`files.remove()` call in `load`'s error branch is deleted.

### 13. Forget a recent project (lineup-app, medium)

File > Open recent remembers the last ten projects in egui's persisted
data, and stays right when a file has been moved or deleted (the open
fails and the path is removed). Read `app/recent.rs`.

Extend it: a "Clear recent" item that forgets the whole list, and a
small ✕ next to each entry that forgets one. The ✕ must not also open
the file, so watch the order in which you check the two clicks. Why is
the list stored as `Vec<String>` rather than `Vec<PathBuf>`, and what
happens to a path that is not UTF-8?

*Practises:* egui menus, persistence, paths that are not UTF-8.

### 14. Reading numbers slowly (lineup-core and RON, medium)

[The version number](saving.md#the-version-number) explains
why the app no longer reads a file's format through a struct with one
field: skipping the rest of the file made RON look ahead, number after
number, to the end of the text.

1. Say a saved song is 2 MB of text with 120,000 numbers in it. If
   skipping each number reads, on average, half the file, how many bytes
   are read in total? At about 2 GB a second, how long does that take?
   How long for a song twice as long?
2. Find the look-ahead in RON's source: in the version `Cargo.lock`
   names, `src/parse.rs` searches `self.src()[skip..]` for `".."`.
   Rewrite that line so it searches only the first `raw_float_len`
   characters. (Hint: slice `self.src()[skip..][..raw_float_len]`, and
   think about what happens if the slice ended in the middle of a
   character.) Then compare with how RON 0.12.3 does it, and read its
   comment on why a `..` cannot straddle the end.
3. `format_number` returns `None` for `(tempo: 120.0, format: 25)`. Why
   is it fine that it only reads the start of the file? (Look at the
   order of the fields in `ProjectFile`, and at `to_text`.)

*Practises:* thinking in growth rates, reading a library's source,
string slices.

### 15. Think about it: files

- `has_unsaved_changes` compares the whole project with the saved copy,
  but only after an edit or an undo sets `check_dirty`, not on every
  frame. Roughly how many notes would a project need before one
  comparison takes a noticeable part of a frame (about 16
  milliseconds)? Why does `project_edited` set `check_dirty` only when
  `dirty` is already false? What would a counter that every edit
  increases add to this design?
- The project file has its own types (`ProjectFile`, `PatternFile`)
  instead of deriving `Serialize` on `Project` itself. What would go
  wrong the day someone renames a field of `Project`, if the file used
  it directly?
- An older Ferrous Lineup refuses to open a file with a newer format
  number. Could it load what it understands and ignore the rest
  instead? When would that be helpful, and when would it quietly lose
  someone's work?
- The damaged-file test writes `NaN` in place of numbers. What happens
  when a project file says `tempo: NaN`? And `master: NaN`? Find the
  code that decides each, and say why they differ.

## Drawing the windows

### 16. A double-click resets the volume (lineup-app, easy)

Make a double-click on the toolbar's VOL slider set the master volume
back to `lineup_core::Mixer::default().master` (0.8). A Reset button
would not fit: the toolbar must stay on one row in a window 1280 points
wide, in Spanish too, and `the_toolbar_fits_on_one_row_at_1280_points`
checks it.

Call `LineupApp::set_master_volume` (in `app/mixer.rs`) rather than
assigning the field, so the change is a step to undo and the diff sends
the audio thread `Command::SetMasterVolume`. Run the app and check that
the slider moves.

*Practises:* immediate-mode input, `Response`, routing a change through
`edit`.

### 17. Remember the hint level (lineup-app and lineup-core, medium)

The harmony keys' hint level goes back to Short every time the app
starts. Other choices are remembered:
[Remembering between runs](egui.md#remembering-between-runs)
shows how, and `settings.rs` keeps the minor-numerals choice with
`get_persisted` and `insert_persisted` under an `Id` of its own. Do the
same for `HintLevel`: give it serde's `Serialize` and `Deserialize` in
`lineup-core` (which already depends on serde), and read and write it
where `HarmonyKeys` keeps `hints`. Which other choices would you
remember the same way?

*Practises:* serde derives across crates, egui's persisted data.

### 18. Hints everywhere (lineup-app, small)

`theme::hint` gives a widget a line in the hint bar and the same text as
a tooltip. The toolbar, the harmony keys and some playlist buttons use
it; the channel rack and the piano roll still use plain
`on_hover_text`. Change them to `theme::hint`, and make the hint bar
show the pattern's name when nothing is under the mouse. Why does
`theme::hint` take the `Response` by value and give it back, instead of
taking `&Response`?

*Practises:* ownership through a function, egui's `Response`, reading
someone else's module before changing it.

### 19. F10 for the metronome, and its F1 line (lineup-app, easy)

**Goal.** The metronome is a toggle in the toolbar and in
**Edit > Settings… > Playback**. Give it a key and add the key to the
F1 list. F10 is free; do not use plain **M**, which plays a note when
the typing keyboard is on.

**Files.** `crates/lineup-app/src/app/practice.rs`,
`crates/lineup-app/src/app.rs`,
`crates/lineup-app/src/app/shortcuts.rs`,
`crates/lineup-app/src/app/ux_tests.rs`.

**Steps.**

1. In `practice.rs`, add a `KeyboardShortcut` constant for F10, the way
   `view.rs` keeps F5 to F8.
2. In `LineupApp::keyboard_shortcuts`, read it in the same guarded
   place as Home and End: not while a text field has the keyboard, and
   not while a control is in use. Call `set_metronome` with the
   opposite of `practice.view().metronome`.
3. Add a line to the **Anywhere** section in `shortcuts.rs`:
   "Metronome on or off". Use the constant rather than writing the key
   twice.
4. Add a headless test in `ux_tests.rs`: press F10, assert
   `practice.view().metronome`; press it again, assert it is off.

**Run.**

```sh
cargo test -p lineup-app shortcuts
cargo test -p lineup-app metronome
```

**Done when.** F10 toggles the toolbar button, F1 lists it under
"Anywhere", and `no_two_global_shortcuts_share_a_key` still passes. A
new line in the F1 list is English text: the test
`every_window_writes_in_spanish` tells you it needs a Spanish one too
(see [Two languages](languages.md)).

### 20. A picture and a version (lineup-app, easy)

- Draw another picture for **Follow playhead** in `icons.rs`: a
  playhead with two arrows. Which test tells you when a line goes
  outside its square?
- Add a **Copy version** button to the About window that puts
  "Ferrous Lineup" and the real version on the clipboard with
  `ctx.copy_text`. Can the text be a `const`, built with `concat!`
  and `env!`?

*Practises:* painting with egui, compile-time strings.

### 21. Think about it: lists and windows

- In `shortcuts.rs`, the "Harmony keys" section uses
  `Written("1 to 7")`. Why not seven `KeyboardShortcut`s in an `Any`?
  What would the list show then?
- The lead sheet opens with `Opening::FirstTime`. What would change for
  you if it used `EveryTime`? Try it, and say which you prefer.

## The editors

### 22. Clear a row (lineup-app, easy)

Right-clicking one step already clears it (`RackAction::ClearStep`).
Add "Clear steps" to a channel's name menu in `channel_menu`, with a new
`RackAction::ClearRow { channel }`. In `LineupApp::apply`, call
`edit_pattern` to switch every step of that channel off, without sending
the audio thread a command by hand: the edit's diff sends the new notes.
Why is one whole-row edit better than a loop of single-step edits?

*Practises:* the action pattern, keeping project and audio in step.

### 23. Paint steps by dragging (lineup-app, medium)

In the rack you click one step at a time. Give `widgets::step_button` a
`Sense::click_and_drag()` and make `Rows::steps` paint every step the
pointer passes over. `Rows` is rebuilt every frame and only returns
actions, so keep the first step, and whether it was being switched on
or off, in `LineupApp` (or in egui's memory) and pass it in.

Add `RackAction::SetStep { channel, step, on }` rather than reusing
`ToggleStep`, so a step the pointer crosses twice does not flip back.
Check `response.hovered()` together with
`ui.input(|i| i.pointer.primary_down())`, and stop painting when the
pointer leaves the row. Should the whole drag be one step to undo? See
[When does one step end?](undo.md#when-does-one-step-end)

*Practises:* immediate-mode state, egui input, the action pattern.

### 24. Show the scale degree on the keys (lineup-app, easy)

`paint_keys` in `piano_roll.rs` draws a dot on every key of the scale.
Draw the degree number instead (1 for the root, 2, 3...), using
`Scale::degree_of`. A twelve-tone row already numbers its notes; keep
that path as it is. Musicians use these numbers all the time ("the 5
chord", "a flat 7").

*Practises:* `Option` in drawing code, `painter.text`.

### 25. The rack's lights (lineup-app, easy)

1. Make the lit light green instead of orange. Where is the colour
   defined, and what else uses it?
2. Give the lights on the first step of each bar a colour of their own,
   instead of only a brighter one.
3. While the song plays a different pattern than the rack shows, the
   lights stay dark. Add a faint note in the rack, such as "Not playing
   now", for that case. Which values in `channel_rack::show` tell you
   the song is playing?

*Practises:* reading drawing code, egui colours.

### 26. Shorter chord names (lineup-app, medium)

The piano roll's chord lane leaves out a name that does not fit its
block. Make it try a shorter name first: for a slash chord, the chord
without its bass ("E7" for "E7/B"), and then the root alone ("E"). Lay
out each with `layout_no_wrap` and draw the first one that fits. Is a
chord without its bass still the right name for what plays?

*Practises:* measuring text before drawing it, iterators with
`find`.

### 27. Alt-drag to copy a clip (lineup-app, easy)

**Ctrl+D** duplicates the selected clips, but dragging a copy is quicker
when you want it somewhere else. Shift is taken (Shift-click adds a clip
to the selection), so use Alt.

In `PlaylistView::handle_input`, read the modifier keys with
`ui.input(|input| input.modifiers.alt)`. The obvious plan is to add a
copy on the press and drag the copy. But `add_clip` refuses a clip of
the same pattern that already starts at the same place, so a copy cannot
be added on top of the original. Instead, remember in `ClipDrag` that
this drag copies, and add the copy only when the pointer reaches
another cell. Which index does the new clip get in
`Project::playlist().clips()`, and how does the drag learn it?

*Practises:* reading egui input, reusing actions you already have.

### 28. A pattern colour picker (lineup-app, medium)

Pattern colours are already saved. `Pattern` has a public `color` (an
index into `PALETTE` in `playlist.rs`), `PatternFile` writes it when it
is not the pattern's place in the list, and files without it load with
the pattern's place as its colour. Read `PatternFile` and
`pattern_color`.

What is missing is the picker. Add one to the ⋯ pattern menu in the
toolbar, with a little square for each palette colour, that sets the
pattern's colour through `edit_pattern`. Then add "Reset colours", which
gives every pattern its place in the list again. Does changing a colour
need a new `FORMAT_VERSION`? What does an older Ferrous Lineup do with a
file that has one?

*Practises:* evolving a file format, `#[serde(default)]`, routing a
change through `edit`.

### 29. Go to a bar (lineup-app, medium)

Home, End, Page Up and Page Down already move the playhead: read the
transport keys in `LineupApp::keyboard_shortcuts`, `seek` and
`skip_bars`. Add a small "bar" box next to the position display, where
typing a bar number and pressing Enter goes there. A bar's first step
depends on every meter change before it: which function already turns
a step into a bar, and can it go the other way? While you type in the
box, Home and End must move the text cursor, not the playhead: what
keeps them from doing both?

*Practises:* keyboard focus, converting bars to steps.

### 30. Think about it: what a drag remembers

The single-note drag in the piano roll remembers a note by its index in
the list (`Drag::Move { index, .. }`), while a selection remembers the
notes themselves. What could go wrong with the index if the list were
kept sorted by start time, and a note was moved while being dragged?

A dragged clip edge sends a new trimmed clip on every frame the pointer
moves, each worked out from the clip as the last frame left it. Drag a
16-step clip's left edge 4 steps out past its start and back in again,
on paper, with `Clip::trimmed_from` (see [Trimming a
clip](project.md#trimming-a-clip)): what are `start`, `skip` and
`repeats` after each frame? What does `trimmed_from` keep the same from
one frame to the next, and why does that make the notes stay where they
were however long you drag?

## Two languages

### 31. Letters and solfège (lineup-core and the app)

1. With solfège chosen, type `Do` in an empty bar of the chord sheet.
   Then choose letters and type `Do` in the next bar. What does each bar
   hold, and why are they different?
2. Write the seven chords of Si♭ major in solfège, then check them on
   the harmony keys. Which chord is diminished?
3. A project is saved with letters more than once over: `file::to_text`
   does its work inside `NoteNaming::Letters.with`, and so does
   `From<HarmonyName> for String`. Remove every
   `NoteNaming::Letters.with` from both, keeping what is inside, and run
   `cargo test -p lineup-core solfege`. Which check fails, and what
   would a friend using letters find in your file? Then put back only
   the one in `From<HarmonyName>`. The tests pass again: why, and why
   keep the others too?
4. `with_notes` leaves `{0,1,4}` alone. Find the line in `with_notes`
   that decides this, and say what it would do with `{Do}`.
5. `note_naming` could store the language's naming the first time it
   reads none, with `insert_persisted`, so the next read finds it. It
   would look tidier. What would then go wrong for someone who starts
   the app in English and switches it to Spanish a week later?

*Practises:* thread-local settings, string slices, reading tests.
