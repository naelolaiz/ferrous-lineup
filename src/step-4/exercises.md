# Exercises

Several of these were built later in the book. Where that happened, the
exercise now starts by pointing at the code that does it, and then asks
for something that is still missing. That is good practice too: reading a
working implementation is often the fastest way to learn the shape of a
change.

## 1. Delete every empty track (lineup-core and lineup-app, easy)

Deleting one track is already implemented: read
`Project::remove_track` and the "Delete track" item in the track's
right-click menu. It removes the track and its clips, and moves the clips
of the tracks below it up by one, just as `remove_pattern` does for
patterns.

What is missing is a way to remove *all* the empty tracks at once.
`remove_unused_tracks` only drops empty tracks at the bottom of the
playlist. Add `remove_empty_tracks`, which removes a track wherever it
is if it has no clips, and a command in the track menu. Which indices
must you visit, and in which order, so that the clips keep pointing at
the right tracks? What should happen if that would leave no tracks?
Write a test with the demo song first, then the method.

*Practices:* `retain`, keeping rules that span two lists, testing the
order of index changes.

## 2. Alt-drag to copy a clip (lineup-app, easy)

In pattern-based studios, dragging a clip with a modifier held copies it
instead of moving it. `Ctrl+D` already duplicates the selected clips, but
dragging a copy is quicker when you want it somewhere else.

Shift is taken: Shift-click adds a clip to the selection. Use Alt for the
copy. In `PlaylistView::handle_input`, read the modifier keys with
`ui.input(|input| input.modifiers.alt)`. The obvious plan is to add a copy
on the press and drag the copy. But `add_clip` refuses an exact duplicate,
so a copy cannot be added on top of the original. Instead, remember in
`ClipDrag` that this drag copies, and add the copy only when the pointer
reaches another cell. Which index does the new clip get in
`Project::playlist().clips()`, and how does the drag learn it?

*Practices:* reading egui input, reusing actions you already have.

## 3. A pattern colour picker (lineup-app, medium)

Pattern colours are already saved. `Pattern` has a public `color` (an
index into the palette in `playlist.rs`), `PatternFile` writes it, and
files saved before it existed load with the pattern's place as its
colour. Read `PatternFile` and `pattern_color`.

What is missing is the picker. Add one to the ⋯ pattern menu, with a
little square for each palette colour, and a `PlaylistAction` that sets
the selected pattern's colour through `edit`. Then add "Reset colours",
which gives every pattern its place in the list again. Does changing a
colour need a new `FORMAT_VERSION`? What does an older Ferrous Lineup do
with a file that has one?

*Practices:* evolving a file format, `#[serde(default)]`, routing a UI
change through `edit`.

## 4. Forget a recent project (lineup-app, medium)

File > Open recent already remembers the last ten projects, in egui's
persisted data, and stays right when a file has been moved or deleted
(the open fails and the path is removed). Read `app/recent.rs`.

Extend it: a "Clear recent" item that forgets the whole list, and a small
✕ next to each entry that forgets one. The ✕ must not also open the file,
so watch the order in which you check the two clicks. What should happen
to the menu while an export is running? Why is the list stored as
`Vec<String>` rather than `Vec<PathBuf>`?

*Practices:* egui menus, persistence, thinking about paths that are not
UTF-8.

## 5. Seek without the mouse (lineup-app, medium)

Clicking the playlist's ruler already moves the playhead: it sends
`PlaylistAction::Seek`, the app turns it into `Command::Seek(step)`, and
the engine moves the sequencer there. Read `Command::Seek` and
`LineupApp::seek`.

Add the keys a transport usually has: Home goes to the start, End to the
end of the song, and Page Up / Page Down move by one bar. Then add a
small "bar" box next to the position display, where typing a bar number
and pressing Enter seeks there. Which window should own these keys, and
how do you keep them from firing while a text field has the keyboard?
What should Play do after a seek: start from the seek position, or from
the start?

*Practices:* keyboard shortcuts, converting bars to steps, deciding
behaviour before writing code.

## 6. Export the song as a WAV file (lineup-audio, harder)

*Step 5 builds this; try it yourself before reading
[its chapter](../step-5/export.md). If you are reading the finished
book, the code is already there: read `render.rs` and `app/export.rs`,
then compare your design with it.*

The engine's tests already play songs without a sound card: they create
an `AudioThread`, send it commands and call `process` on a buffer. Use
the same idea for File > Export WAV. Build a separate `AudioThread` on
the UI thread (it never goes near the real one), send it the whole
project as `send_whole_project` does, then `SetMode(Song)` and `Play`,
and call `process` block by block for `song_length` steps, writing the
samples with `hound::WavWriter`. Add a second or two at the end so the
last notes can ring out.

Some things to work out: how do you send commands if `AudioThread::new`
wants the consumer end of a ring buffer? Which parts of `engine.rs` must
become public, and can you keep them `pub(crate)` by putting the export
function in `lineup-audio`? How long does exporting a three-minute song
take, and should the window keep drawing while it runs?

*Practices:* reusing real-time code offline, API design across crates,
`hound`.

## 7. Think about it

- Clips refer to patterns by their *position* in the list, which is why
  `remove_pattern` has to renumber them. Imagine instead that every
  pattern has a permanent id (`struct PatternId(u32)`) that never
  changes. Which methods of `Project` become simpler, and which harder?
  What would change in the file, and in the audio thread's
  `ChannelNotes`? Would an undo feature be easier with ids or with
  positions?
- `has_unsaved_changes` compares the whole project with the saved copy,
  but only after an edit or an undo sets `check_dirty`, not on every
  frame. Roughly how many notes would a project need before one
  comparison takes a noticeable part of a frame (about 16 milliseconds)?
  Why does `project_edited` set `check_dirty` only when `dirty` is
  already false? What would a counter that every edit increases add to
  this design?
- The project file has its own types (`ProjectFile`, `PatternFile`)
  instead of deriving `Serialize` on `Project` itself. What would go wrong
  the day someone renames a field of `Project`, if the file used it
  directly?
- An older Ferrous Lineup refuses to open a file with a newer format number.
  Could it load what it understands and ignore the rest instead? When
  would that be helpful, and when would it quietly lose someone's work?
- The UI sends the song's length along with its pieces, instead of
  letting the audio thread work it out. What would it take to keep two
  copies of that calculation in agreement, and how would you notice if
  they drifted apart? Since step 8, the pieces are `Play`s made by
  `Project::playback_arrangement`; why is unrolling repeats on the UI
  side simpler than teaching the audio thread about sections?
