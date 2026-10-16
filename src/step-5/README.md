# Step 5: Undo, redo, and exporting the song

Step 5 adds two things every studio needs, and that you miss as soon as
you have written something you like:

- **undo and redo** for every change to the project: steps, notes, chords,
  clips, tracks, channels, samples, patterns, the tempo, names and the key.
  The Edit menu says what will be undone ("Undo move clip"). A drag is one
  step, however many frames it lasts, and typing a name is one step too.
  The last 100 steps are kept,
- **Export WAV**: File > Export WAV… saves the song as a sound file that
  any music player, video editor or other studio can open: 16-bit stereo
  at 44,100 frames per second, like a CD. The export runs in the
  background with a progress bar in the status bar. You can keep playing
  and editing while it runs, and nothing you do changes the file being
  written.

![The Edit menu after clicking two steps: "Undo change steps"](../images/step-5-edit-menu.png)

![The status bar while a long song is exported](../images/step-5-exporting.png)

![The status bar when the export is done](../images/step-5-exported.png)

## How to use it

| Do this | To |
|---------|----|
| Ctrl+Z, or Edit > Undo | undo the last change |
| Ctrl+Shift+Z or Ctrl+Y, or Edit > Redo | redo what was undone |
| File > Export WAV… | save the song as a `.wav` file |

On a Mac, use ⌘ instead of Ctrl. While you type in a text field (a
pattern or track name), Ctrl+Z undoes your typing inside the field, as in
any other program; once you leave the field, it undoes whole changes
again.

Some things are not part of the project, so undo leaves them alone:
whether the music is playing, the master volume, which windows are open,
the snap setting, and which pattern is selected (except that a pattern
brought back by undo is selected, so you can see it). Opening a file or
starting a new one forgets the history.

The export plays the song once, from the first bar to the end of the last
clip, then two more seconds so the last notes can ring out. The file is
named after the project ("song.loom" suggests "song.wav"). Export WAV is
greyed out when the playlist has no clips.

## What changed in the architecture

```text
  loom-core
    history.rs (new): History<T>, copies of the project to undo and redo to
    diff.rs (new):    diff(before, after) -> Vec<Change>

  loom-app
    app/edit.rs (new):   edit(), undo(), redo(), sync_audio(), the Edit menu
    app/export.rs (new): File > Export WAV, on a thread of its own

  loom-audio
    render.rs (new): Renderer and export_wav: the audio thread, without a sound card
    engine.rs:       InsertPattern and InsertChannel, at any position
    sample.rs:       SampleCache, so sample files are read once
```

Every change to the project now takes the same path:

```text
  click on a step
    └▶ edit("change steps", |project| ...)
         before = project.clone()
         change the project
         diff(&before, &project)  ──▶  [Notes { pattern: 0, channel: 3 }]
                                            └▶ Command::SetNotes ──▶ audio thread
         history.record("change steps", before)

  Ctrl+Z
    └▶ undo()
         before = project.clone()
         history swaps the older project in
         diff(&before, &project)  ──▶  the same kind of commands, backwards
```

Three ideas carry this step:

1. **Keep copies, not instructions.** The history keeps whole copies of
   the project, not a list of "how to reverse what I did". A copy cannot
   be reversed wrongly, and the project is small enough to copy on every
   change.
2. **One way to tell the audio thread.** Before this step, each kind of
   edit sent its own commands to the audio thread. Now every edit, and
   every undo and redo, compares the project before and after and sends
   the differences. There is one piece of code to get right, and it is
   tested on its own.
3. **The same code, faster than real time.** The export does not have
   its own way of making sound. It runs the very same audio thread code
   the sound card uses, on a thread of its own, as fast as the computer
   can go, and writes the result to a file.

## Files in this step

```text
crates/loom-core/src/history.rs        History<T>: undo and redo (new)
crates/loom-core/src/diff.rs           Change and diff(): what changed between two projects (new)
crates/loom-audio/src/render.rs        Renderer and export_wav (new)
crates/loom-audio/src/engine.rs        InsertPattern, InsertChannel, settle_volumes
crates/loom-audio/src/sample.rs        SampleCache
crates/loom-audio/src/instrument.rs    Instrument::for_channel takes the cache
crates/loom-audio/src/dsp.rs           Smoothed::settle
crates/loom-app/src/app/edit.rs        edit, undo, redo, the Edit menu (new)
crates/loom-app/src/app/export.rs      File > Export WAV (new)
crates/loom-app/src/app.rs             every change goes through edit()
crates/loom-app/src/app/files.rs       the Export WAV menu item; opening a file clears the history
crates/loom-app/src/piano_roll.rs      follows its channel when one is inserted above it
```

The chapters go in the order a change travels: first the history that
remembers it, then how the audio thread learns about it, and last the
export, which plays the result into a file.
