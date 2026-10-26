# Losing nothing: autosave, recovery, recent projects, and audio that comes back

Files: [`crates/lineup-app/src/app/recovery.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/recovery.rs),
[`crates/lineup-app/src/app/recent.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/recent.rs),
[`crates/lineup-app/src/app/settings.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/settings.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs)
and [`crates/lineup-listen/src/decode.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/decode.rs)

A long song is a lot of work, and most of it is invisible: a chord
typed on the sheet, a note dragged in the piano roll, a mix changed in
the mixer. October 2026 was spent on the ways that work could still be
lost, and on the sound card that could stop. The fixes are not musical.
They are the difference between a program you trust and one you save
twice.

## A copy of unsaved work

The app writes a recovery copy of the project while it has unsaved
changes. The first change writes one at once, so the first edit of a
session is not the one a crash loses; after that it writes every thirty
seconds:

```rust
const INTERVAL: Duration = Duration::from_secs(30);
```

The files are `recovery.lineup` and `recovery.info` in the app's data
folder, the one `eframe::storage_dir(APP_NAME)` gives it. On Linux that
is usually `~/.local/share/ferrouslineup`. The copy is **not** written
next to the project because a project may be on a read-only medium, and
an autosave file in a folder of songs would surprise anyone looking at
it. The small `.info` file carries the project's path and name, so a
recovered song comes back as itself, not as "Untitled".

The copy is written by `file::save`, which writes a temporary file and
renames it. A crash in the middle leaves the previous copy whole; the
info file is written after the project, so a crash between them loses
the name rather than giving the project the wrong one.

At startup, if a copy is there, a modal asks **Recover unsaved work?**
and names the file:

- **Recover it** puts the project back, with its path and name, and
  removes the copy. It also loads the file on disk, if it still exists,
  as the "saved" version, so the recovered work correctly shows the
  unsaved star; undoing back to the file clears it. If the file is gone,
  the recovered project is simply unsaved.
- **Discard it** removes the copy and leaves the project the app would
  otherwise have opened.

A clean exit removes the copy, because there is nothing to recover. The
copy that remains is the sign of a crash, a kill, or a power cut. A copy
whose project will not load is removed at startup rather than offered.

## File > Open recent

**File > Open recent** lists the last ten projects opened or saved,
most recent first. The list is kept in egui's stored data, so it
survives a restart. Each item shows the file's name, with the full path
in its tooltip. Opening a file that no longer loads removes it from the
list instead of leaving a dead entry. The menu is disabled while the
list is empty.

The list holds paths, not copies of projects. If a project is moved or
renamed outside the app, the old path stays until an attempt to open it
fails.

## Save, import and quit

Three smaller fixes keep the project you meant to keep:

- **Save as…** now gives the file a project extension even if you type
  another one. `song.txt` is saved as `song.lineup`, not as a chord
  sheet that opens in the wrong importer. A `.loom` file keeps its name.
- Opening a project, a demo, a MIDI file or a chord sheet stops a
  **File > Import recording…** that is still listening. The import was
  started for the project being left behind; without this, its result
  could arrive later and replace the new project. The notice says
  "Stopped importing the recording."
- **File > Quit** during a WAV or video export asks **An export is still
  running**, with **Stop it and quit** and **Keep exporting**. Stopping
  waits for the export to finish, then closes; any unsaved-changes
  question comes first. The export is not cut off mid-file, and the
  project is not closed underneath it.

The style editor has its own unsaved state. Quitting with an edited
draft asks **Save style changes?**, offering **Save style**, **Don't
save style** and **Cancel**, so a song save does not silently discard
the style.

## Dragging without copying the song

Every edit used to clone the whole project to compare it with the
previous version. For a click that is cheap. For a drag it happened on
every frame: moving one note in a large song copied every pattern, every
channel and every note, sixty times a second.

Now the first change of a drag opens one undo step and keeps a copy from
before it. The following frames change the project in place. The audio
thread is sent what it is missing from a shadow copy of the last state
it received, at most every 50 milliseconds, and once more when the step
closes:

```rust
const SYNC_INTERVAL: Duration = Duration::from_millis(50);
```

A change to the number of patterns or channels is structural and is
sent at once, because the audio thread's rows have to move with it. The
unsaved star is no longer worked out by comparing the whole project
every frame: a project already known to differ stays that way, and the
comparison runs only when it might have changed. Undo still keeps up to
256 MB of snapshots, with at least ten steps even for a very large song.

## Audio that comes back

**Edit > Settings… > Audio** has a button under **Output**. It says
**Restart audio** while the stream is running, and **Try again** when
audio is unavailable or the stream has reported an error. It opens the
system's default output device again. The project stays; the MIDI input
and output ports stay selected and are reconnected to the new engine;
the whole project is sent to the fresh audio thread. If the song was
playing, it seeks back to the same position and plays again, and the
notice says "Audio is playing again."

The old stream is dropped before the new one starts, because a device
being reopened cannot still be held by the dead stream. If opening
fails, the status bar says why, and the button remains **Try again**.
An unplugged interface, a changed default device, or a sound server that
restarted can all be fixed without closing the app.

## Commands that must arrive

The audio thread may not wait, so the UI talks to it through a ring
buffer. The queue holds 4096 commands, a whole large project. When it is
full, `send_wait` does not drop the command at the first full queue: it
yields and tries again, up to a thousand times. Only a stream that has
already reported an error gives up at once, because waiting for room
that will never come would freeze the window.

MIDI Out has the same rule for a stricter reason. A dropped note-off
leaves a note sounding for ever on an external synth. Messages that find
the output queue full wait in a reserved backlog and go out at the start
of the next block. Removing a channel or replacing an instrument sends
its note-offs through the same path. Disconnecting a MIDI port sends an
All Notes Off message on all sixteen channels before the old connection
is closed. The backlog is finite, but it takes hundreds of stuck messages
to fill it, which means the port itself is gone.

**File > Import recording…** refuses a recording longer than ten
minutes. Decoding checks the length as it reads, and stops with
"the recording is longer than 10 minutes". Analysis holds the recording,
and a copy of it, in memory; a longer one would need more than a small
machine can spare. The import shows the reason instead of failing
silently or exhausting memory.

The playback trace is allowed to drop. It records every note for a
report, not for the sound. If its reader falls behind and its 65536
events fill, the audio thread counts the lost events and carries on; the
report says how many were lost. Waiting there would put the trace before
the music, which is the wrong order.

## Trying it

1. Run `cargo run --release`, open **File > New**, make a change, and
   save it as `recovery-test.lineup`. Make another change. Wait half a
   minute, then kill the app from the terminal (`Ctrl+C`) rather than
   closing it. Start it again. **Recover unsaved work?** names the file;
   click **Recover it**. The change is there, with the unsaved star.
2. Save it, open another project, then use **File > Open recent**: the
   test song is in the list. Quit and start again; it is still there.
3. Start a song playing and open **Edit > Settings… > Audio**. Click
   **Restart audio** while it plays. The sound returns, the notice says
   "Audio is playing again", and the song continues from where it was.
   If the device is unavailable, the same button says **Try again**.
4. With a MIDI synth connected, play a long note and remove the channel
   or change its instrument while it sounds. The note stops: its
   note-off was kept and sent rather than dropped. (This needs real MIDI
   hardware; the tests run without it.)
