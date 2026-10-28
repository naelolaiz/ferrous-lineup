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

The files are in a `recovery` folder inside the app's data folder, the
one `eframe::storage_dir(APP_NAME)` gives it. On Linux that is usually
`~/.local/share/ferrouslineup/recovery`. Each run of the app names its
files after itself, such as `4211-1793047912612-0.lineup` and
`4211-1793047912612-0.info`: the process number, the time it started in
milliseconds, and a count. Why each run needs its own is in [Two windows
at once](#two-windows-at-once) below. The copy is **not** written
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
Version 0.2.0 kept one copy, `recovery.lineup`, straight in the data
folder; a copy left there is still offered after an update.

### Two windows at once

Nothing stops you from starting Ferrous Lineup twice. When every run
wrote the same `recovery.lineup`, the second window found the first
one's copy at startup and offered it as if the first had crashed, while
it was still open. Each window's autosave also replaced the other's
copy, and the first to save removed it.

So each run writes its own copy, and next to it a third file,
`<run>.lock`, which it **locks** before the first copy is written and
holds while it runs. A file lock is a promise kept by the operating
system: only one open file can hold it, and when the program that holds
it ends, the system lets go of it, even in a crash or a power cut. That
makes it the right question to ask at startup: "is the run that wrote
this copy still going?" If its lock can be taken, it is not, and the
copy is offered; if not, the copy belongs to a window that is still open
and is left alone.

The standard library has file locks since Rust 1.89 (`File::lock`,
`File::try_lock` and friends). `try_lock` does not wait: it says at once
whether it got the lock, and its error tells "someone else holds it"
apart from "this system cannot lock files":

```rust
fn take_lock(path: &Path) -> Lock {
    // A copy whose lock file is gone was left by a run that closed with
    // work unsaved: the file is made again, to be taken.
    let Ok(file) = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
    else {
        return Lock::Unknown;
    };
    match file.try_lock() {
        Ok(()) => Lock::Taken(file),
        Err(TryLockError::WouldBlock) => Lock::Held,
        Err(TryLockError::Error(_)) => Lock::Unknown,
    }
}
```

The lock lasts as long as the `File` value: dropping it lets go. So a
window that finds a copy keeps the `File` in the `Recovered` value until
you answer the question; a third window opened meanwhile cannot take the
lock and does not offer the same copy twice. When the app closes, the
`Drop` of `Recovery` removes its lock file; a copy of unsaved work stays,
without its lock, to be offered next time. A system that cannot lock
files (`Lock::Unknown`) offers the copy anyway: offering it twice loses
nothing, never offering it would.

If two runs crashed, there are two copies. The newest is offered first,
and the next waits until the work recovered before it is saved or let
go, so recovering one never replaces another.

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
  sheet that opens in the wrong importer.
- Opening a project, a demo, a MIDI file or a chord sheet stops a
  **File > Import recording…** that is still listening. The import was
  started for the project being left behind; without this, its result
  could arrive later and replace the new project. The notice says
  "Stopped importing the recording."
- **File > Quit** during a WAV or video export asks **An export is still
  running**, with **Stop it and quit** and **Keep exporting**. Stopping
  asks the export to stop, waits until it has, removes the unfinished
  file, and then closes; any unsaved-changes question comes first. A
  half-written WAV or video is never left behind looking finished, and
  the project is not closed underneath an export that is still writing.

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
closes. A drag held still, or a pause in typing, has no next change to
carry the last one, so the end of each frame sends it once its 50
milliseconds are up, and asks egui for a frame at that moment even if
nothing else moves:

```rust
const SYNC_INTERVAL: Duration = Duration::from_millis(50);
```

A change to the number of patterns or channels is structural and is
sent at once, because the audio thread's rows have to move with it. The
unsaved star is no longer worked out by comparing the whole project
every frame. A change made in one go, such as a click, an undo or a
redo, compares the project with the saved one again, so a step clicked
on and then off clears the star. The frames of a drag do not compare;
the comparison runs once when the drag's undo step closes, so a note
dragged away and back clears the star when you let go. Undo still keeps
up to
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

Notes the old engine left sounding on an external synth stop: before the
old MIDI Out thread finishes, it sends All Notes Off on all sixteen
channels, and the new one waits for it. If the playback trace is on, it
goes on in the same file. [Before the next
release](before-the-release.md#restart-audio-notes-that-stop-and-a-trace-that-goes-on)
shows how.

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
