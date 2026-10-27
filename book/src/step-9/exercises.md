# Exercises

These are for after Step 9: the band plays, the windows are in place,
and the code is yours to change. Each one is a small piece of work with
a clear end. Some are mostly in the app, some mostly in Rust, and a few
touch both. Do them in any order.

Each exercise says what is **already there**, so you extend it rather
than build it again. Exercises 6 and 8 both grow the project format, to 29; if
you do both, give the second one the next free number, 30. Run
`cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` before you call
one finished; CI runs both.

## 1. A recovery copy that is damaged (lineup-app, easy)

**Goal.** Make crash recovery trustworthy when the copy on disk is not.
`app/recovery.rs` already writes a `.lineup` and a `.info` file for each
run to the `recovery` folder in the app's data folder, offers the copy
at startup, and removes a copy that will not load. The existing test
`a_recovery_copy_comes_back_with_its_file` covers the happy path. Add
the unhappy ones.

**Files.** `crates/lineup-app/src/app/recovery.rs`.

**Steps.**

1. See it by hand: open a demo, type a chord in the first bar, wait a
   moment, and force-quit the app. Do not use **File > Quit**. Start it
   again and choose **Recover it**. Close the recovered song without
   saving it.
2. Read `Files`, `load` and `write`, and the existing tests at the
   bottom of the file.
3. Add a test that takes `Files::of_run(&folder, "test")`, writes
   rubbish into its `project` file, puts a valid `info` file beside it,
   calls `load(files, None)`, and checks that it returns `None` **and**
   that both files are gone.
4. Add a second test that creates the folder, writes a valid project
   to `files.project` with `file::save`, but leaves out the info file.
   Check that `load` still returns the project, with no path and no
   name, so the dialog can call it "Untitled".
5. Clean up the temporary folder at the end of each test, as the
   existing one does.

**Run.** `cargo test -p lineup-app recovery`.

**Done when.** Both tests pass, and the first fails if the
`files.remove()` call in `load`'s error branch is deleted.

## 2. Trace a bar inserted and deleted (app, easy)

**Goal.** See with your own eyes that inserting a bar moves everything
after it, and deleting it moves everything back. The chord sheet and
the playlist already have **Insert a bar before**, **Insert a bar
after** and **Delete this bar**; `--trace` already writes every note the
player plays.

**Files.** `crates/lineup-core/src/bars.rs`,
`crates/lineup-core/src/project.rs`,
`crates/lineup-app/src/chord_sheet.rs`,
`crates/lineup-app/src/app/trace.rs`.

**Steps.**

1. Write a baseline trace of a demo with a steady tempo:

   ```sh
   cargo run --release -- --trace "El Choclo" --out before.tsv
   ```

2. Open the same demo, choose **File > Save as…**, and save it as
   `inserted.lineup`.
3. Right-click bar 3 on the chord sheet and choose **Insert a bar
   after**. Save. Trace the saved file:

   ```sh
   cargo run --release -- --trace inserted.lineup --out after.tsv
   ```

4. Compare the two tables, ignoring the `#` header lines. In the
   `after.tsv`, every note that used to start at or after bar 4 should
   now be one bar later; notes in bars 1 to 3 should be unchanged.
5. Delete the bar you inserted (it is bar 4), save, trace again, and
   compare with `before.tsv`.

**Run.**

```sh
diff <(grep -v '^#' before.tsv) <(grep -v '^#' after.tsv) | head -40
```

**Done when.** The inserted trace shows the one-bar shift in `start`,
`bar` and `step`, the deleted trace matches the baseline, and you can
point to the code in `bars.rs` that moves the clips, markers and
sections.

## 3. Build a style, save it, and find it in a song (app and Rust, medium)

**Goal.** Use the style editor that is already in the channel rack
([The style editor](style-editor.md)), then read the two files it
produces: the reusable `.lineup-style` file and the song that embeds
it.

**Files.** `crates/lineup-app/src/style_editor.rs`,
`crates/lineup-app/src/app/styles.rs`,
`crates/lineup-core/src/library/user_style.rs`,
`crates/lineup-core/src/band.rs`,
`crates/lineup-core/src/file.rs`.

**Steps.**

1. Open the channel rack (**F6**) and click **New style…**.
2. Give it a name, two bars, a tempo and a progression such as
   `I | vi | ii7 | V7`. Add a kick and a snare in the rack, then a bass
   note in the piano roll. Press **Space** to hear it.
3. Click **Save as…** and keep it in the styles folder the dialog opens
   in. That is the app's data folder under `styles/`; on Linux it is
   `~/.local/share/ferrouslineup/styles/`. A style saved outside that
   folder is not listed in the Library.
4. Open the saved file in a text editor. It is RON. Find the style's
   own `format: 1`, its `bars`, `chart`, and the `lanes` that hold each
   channel and its notes.
5. Open a demo, pick the style in the Library, and save the song. Open
   the song file and search for `custom_rhythm`: the whole style is
   inside the song, because the song can no longer depend on a file in
   your data folder. Its format is now at least 22, and higher if the
   song also uses a later feature such as a saved mixer (format 26).

**Run.** `cargo test -p lineup-core user_style`, then
`cargo test -p lineup-app styles`.

**Done when.** The app lists the style after a restart, the song plays
it, and the song file contains the style's lanes and a format of at
least 22.

## 4. Put a new song in its genre file (lineup-core, medium)

**Goal.** Add a public-domain tune to the demo library, in the file for
its genre, and make the menu and the library tests agree with it. The
songs are already data: `Song`, `Part`, `SONGS` and the per-genre
modules exist; you are adding an entry, not a feature.

**Files.** `crates/lineup-core/src/library/song/<genre>.rs` (for
example `blues.rs` or `folk.rs`), `crates/lineup-core/src/library/song.rs`.

**Steps.**

1. Read two songs in the genre file you chose. Copy the shape of one:
   `name`, `origin`, `feel`, `genre`, `tonic`, `mode`, `time`, `tempo`,
   `comping`, `bass`, `rhythm`, `band`, `style`, `parts`, `form`,
   `drums_from`, `about`.
2. Write the melody with `|` between bars and a chord chart under it.
   Use a part name such as "Verse" or "Tune". Keep the tune public
   domain.
3. Make the constant `pub(super) const`, and add it to `SONGS` in
   `song.rs` inside its genre's block. `SONGS` is kept in `Genre::ALL`
   order, and the test checks it.
4. Let the tests find the mistakes: a bar that does not add up, a chord
   that does not parse, a style in another meter.

**Run.**

```sh
cargo test -p lineup-core library
cargo run --release -- --demos
```

**Done when.** The tests pass, the demo appears under **File > Demos >
Songs >** its genre, and it opens on the chord sheet and plays.

## 5. A rhythm with a cowbell and a fill (lineup-core, easy)

**Goal.** Add one drum pattern to the library, with a cowbell and a
named fill. `RHYTHMS` and `FILLS` already exist, the nine extra drums
including `Cowbell` already exist, and the tests already check every
line's length and characters.

**Files.** `crates/lineup-core/src/library/rhythm.rs`.

**Steps.**

1. Add a `rhythm(...)` entry to `RHYTHMS` in the right genre block. Give
   it a name, a genre, a tempo, its lines, and a fill that exists, such
   as `"Tom fill"`.
2. Write one line for `Kick`, one for `Snare`, and one for `Cowbell`.
   Every line must be exactly sixteen characters in 4/4, with `X`, `x`,
   `o`, `h`, `l` and `.` as the only characters. Spaces are only for
   reading.
3. Put the cowbell on an offbeat or a clave, not on every step; the
   point is to hear it as a different colour.
4. Open the Library, find the rhythm in its genre, click it, and play
   it. Then make a section change so the fill is used.

**Run.** `cargo test -p lineup-core rhythm`.

**Done when.** `every_line_is_whole_bars_of_known_characters` and
`names_are_unique_and_fills_exist` pass, the rhythm is listed with its
tooltip, and the fill plays into the next section.

## 6. A second send: delay (lineup-core, lineup-audio and lineup-app, hard)

**Goal.** The mixer already has a per-channel **Reverb** send and a
master reverb. Add a **Delay** send beside it, so a channel can go into
an echo without losing its dry sound. Each channel already has a delay
of its own among its [effects](channel-effects.md); this one is shared,
like the reverb, so several channels echo together in one delay. This
is a capstone: it crosses the project file, the audio thread and the
mixer window.

**Files.** `crates/lineup-core/src/project.rs`,
`crates/lineup-core/src/mixer.rs`, `crates/lineup-core/src/file.rs`,
`crates/lineup-audio/src/engine.rs`,
`crates/lineup-audio/src/reverb.rs` (as the model for a new
`delay.rs`), `crates/lineup-app/src/app/mixer.rs`.

**Steps.**

1. In `Channel`, add `delay_send: f32`, default `0.0`, and include it
   in the range checks and in `has_plain_mix`. Give every constructor
   the default.
2. In `Mixer`, add the delay's own settings: time in milliseconds,
   feedback, and wet/dry. Validate them in `Mixer::valid`.
3. Add `Command::SetDelay` and `Command::SetDelaySend`, and a shared
   delay that allocates its buffer when the engine is built, never in
   `process`. The channel delay's ring buffer in
   `crates/lineup-audio/src/effects.rs` shows how to read the echo. Follow the reverb's pattern: take `amount` of the sample
   out of the dry mix as `send`, and add the delayed signal back.
4. Add a **Delay** slider to each mixer line, next to **Reverb**, and a
   small delay section beside the reverb controls. Send the command
   through the same path as `SetReverbSend`.
5. Bump `FORMAT_VERSION` to 29, teach `format_for` that a non-zero
   delay send or non-default delay settings need 29, and leave the
   fields out of the text when they are at their defaults.
6. Test the audio path: a send of 0 produces no echo, a send above 0
   produces one, and the dry sample is reduced by the wet amount. Test the
   file: an old file opens with send 0; a song with a delay send is
   written as format 29 and round-trips.

**Run.**

```sh
cargo test -p lineup-core mixer
cargo test -p lineup-audio delay
cargo test -p lineup-app mixer
```

**Done when.** The slider is audible in the app and in a WAV export,
the audio thread allocates nothing while it runs, and a song without
delay is still written in the format it was.

## 7. F10 for the metronome, and its F1 line (lineup-app, easy)

**Goal.** The metronome is already a toggle in the toolbar and in
**Edit > Settings… > Playback**. Give it a key and add the key to the F1 list.
F10 is free; do not use plain **M**, which plays a note when the typing
keyboard is on.

**Files.** `crates/lineup-app/src/app/practice.rs`,
`crates/lineup-app/src/app.rs`,
`crates/lineup-app/src/app/shortcuts.rs`,
`crates/lineup-app/src/app/ux_tests.rs`.

**Steps.**

1. In `practice.rs`, add a `KeyboardShortcut` constant for F10.
2. In `LineupApp::keyboard_shortcuts`, read it in the same guarded
   place as Space: not while a text field has the keyboard, and not
   while a modal is open. Call `set_metronome(!metronome)`.
3. Add a line to the **Anywhere** section in `shortcuts.rs`:
   "Metronome on or off". Import the constant rather than writing the
   key twice.
4. Add a headless test in `ux_tests.rs`: press F10, assert
   `practice.view().metronome`; press it again, assert it is off.

**Run.**

```sh
cargo test -p lineup-app shortcuts
cargo test -p lineup-app metronome
```

**Done when.** F10 toggles the toolbar button, F1 lists it under
"Anywhere", and the existing test `no_two_global_shortcuts_share_a_key`
still passes.

## 8. A new project field and its format test (lineup-core, medium)

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
3. Raise `FORMAT_VERSION` to 29. In `format_for`, return 29 first when
   `!project.notes.is_empty()`; leave everything else as it is.
4. Write three tests in `file.rs`: an empty project stays at its old
   format and has no `notes:` line; a project with notes is format 29
   and round-trips; and a file written before the field existed reads
   back with empty notes.

**Run.** `cargo test -p lineup-core file`.

**Done when.** Old project texts still open, the new field survives a
round trip, and `format_for` names 29 only for a song that needs it.

## 9. Two views in a video, and a sync check (app, medium)

**Goal.** Export a video with two views and check that the picture and
the sound agree. Video export already exists, with chord sheet, lead
sheet, playlist, piano roll and harmony keys; you are testing the sync,
not building a view.

**Files.** `crates/lineup-app/src/app/video.rs`,
`crates/lineup-video/src/` for reference.

**Steps.**

1. Open a demo with a meter or key change, such as **El Choclo**.
2. Choose **File > Export WAV…** and save `el-choclo.wav`.
3. Choose **File > Export video…**, tick **Chord sheet** and **Piano
   roll**, put them side by side on the preview, choose **1280×720**,
   and export `el-choclo.mp4`.
4. If `ffprobe` is installed, compare the durations. The picture is 30
   frames a second, and the sound is MP3 inside the MP4, so allow a
   tenth of a second:

   ```sh
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.wav
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.mp4
   ```

5. Watch the bar where the key or meter changes. The framed bar on the
   chord sheet, the playhead in the piano roll and the sound should meet
   there together. Check a second point, such as the first bar after a
   repeat.

**Done when.** The durations differ by no more than a frame or two plus
the audio tail, the two views show the same bar at the same moment, and
you can say which code in `video.rs` draws the picture **before** the
sound for each frame.

## 10. A MIDI note-off regression test (lineup-audio, medium)

**Goal.** Prove that **Stop** sends note-offs for notes the sequencer
started on a MIDI Out channel. The engine already keeps note-offs when
the MIDI queue is full, and already sends All Notes Off before closing
an output; this test pins the Stop path.

**Files.** `crates/lineup-audio/src/engine.rs` (the `#[cfg(test)]`
module).

**Steps.**

1. Read `midi_out_channels_send_their_notes` and
   `stopping_releases_held_notes`. Use `add_midi_out`, `received` and
   `note_off` from the test module.
2. Add `stopping_releases_midi_out_notes`: a long note on MIDI channel
   3, **Play**, render one step and assert the NoteOn; send **Stop**,
   render a few frames, and assert the NoteOff.
3. Make the note long enough that it cannot have ended by its own gate.
4. Run the test once with its `send(&mut h, Command::Stop)` line removed
   to see it fail, then put the line back.

**Run.**
`cargo test -p lineup-audio stopping_releases_midi_out_notes`.

**Done when.** The test fails if `stop` stops calling `release_all`, and
passes on the current code.
