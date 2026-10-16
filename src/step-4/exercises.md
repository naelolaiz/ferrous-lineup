# Exercises

## 1. Delete a track (loom-core, easy)

Tracks can be added and renamed, but not removed. Write
`Project::remove_track(index)`, which removes the track and its clips,
and moves the clips of the tracks below it up by one, just as
`remove_pattern` does for patterns. Should it refuse to remove the last
track? Write a test with the demo song first, then the method. Finally
add a "Delete track" item to a right-click menu on the track name
(`response.context_menu(|ui| ...)`) and a `PlaylistAction` for it.

*Practices:* `retain`, keeping rules that span two lists, the action
pattern.

## 2. Shift-drag to copy a clip (loom-app, easy)

In pattern-based studios, holding Shift while dragging a clip copies it
instead of moving it. In `PlaylistView::handle_input`, read the
modifier keys with `ui.input(|input| input.modifiers.shift)`. The
obvious plan is to add a copy on the press and drag the copy. But
`add_clip` refuses an exact duplicate, so a copy cannot be added on top of
the original. Instead, remember in `ClipDrag` that this drag copies, and
add the copy only when the pointer reaches another cell. Which index does
the new clip get in `Project::playlist().clips()`, and how does the drag
learn it?

*Practices:* reading egui input, reusing actions you already have.

## 3. Pattern colours that are saved (loom-core and loom-app, medium)

Pattern colours come from `pattern_color(index)` in `playlist.rs`, so a
pattern changes colour when one before it is deleted. Give `Pattern` a
`pub color: u8` (an index into the palette), add a colour picker to the
⋯ pattern menu, and save it in `PatternFile`.

Files saved before your change have no `color`, and loading them must
still work: put `#[serde(default)]` on the new field of `PatternFile`, and
add a test that loads a file without it. Does `FORMAT_VERSION` need to
change? Ask yourself what an *older* Ferrous Loom does with a file that
has the new field.

*Practices:* evolving a file format, `#[serde(default)]`, tests with
hand-written file text.

## 4. Open recent (loom-app, medium)

Add an "Open recent" submenu to the File menu with the last five files
opened or saved. The list must survive closing the app, so it has to be
stored somewhere, and each system has its own place for that (under
`~/.local/share` on Linux, `~/Library/Application Support` on macOS,
`AppData` on Windows).

eframe can do this for you. Turn on its `persistence` feature in
`Cargo.toml`, read the list in `LoomApp::new` with
`cc.storage.and_then(|storage| eframe::get_value(storage, "recent"))`,
and write it by implementing `eframe::App::save` with
`eframe::set_value`. `Vec<PathBuf>` already implements serde's traits.
What should happen when a recent file has been moved or deleted?

*Practices:* features, `Option` chains, letting a crate handle the
differences between systems.

## 5. Start from any bar (loom-core and loom-audio, medium)

Clicking the playlist's ruler should move the playhead there, playing or
not. Add `Sequencer::seek(step)` in `loom-core` with a test, a
`Command::Seek(usize)` in the engine, and a `PlaylistAction::Seek` sent
when the ruler is clicked. Where should the next step start: at once, or
on the next step boundary? What should happen to notes that are already
sounding when the playhead jumps? (Look at how `Command::Stop` ends
them.) And what should the Play button do after a seek: start from the
seek position, or from the start?

*Practices:* sample-accurate timing, deciding behaviour before writing
code.

## 6. Export the song as a WAV file (loom-audio, harder)

*Step 5 builds this; try it yourself before reading
[its chapter](../step-5/export.md).*

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
function in `loom-audio`? How long does exporting a three-minute song
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
  every frame. Roughly how many notes would a project need before that
  takes a noticeable part of a frame (about 16 milliseconds)? The saving
  chapter explains what a `dirty: bool` flag gets wrong. Could a counter
  that every edit increases, plus a full comparison only when the counter
  changed, have the best of both?
- The project file has its own types (`ProjectFile`, `PatternFile`)
  instead of deriving `Serialize` on `Project` itself. What would go wrong
  the day someone renames a field of `Project`, if the file used it
  directly?
- An older Ferrous Loom refuses to open a file with a newer format number.
  Could it load what it understands and ignore the rest instead? When
  would that be helpful, and when would it quietly lose someone's work?
- The UI sends the song's length along with its clips, instead of
  letting the audio thread work it out. What would it take to keep two
  copies of that calculation in agreement, and how would you notice if
  they drifted apart?
