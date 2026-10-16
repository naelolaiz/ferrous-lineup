# Step 4: The playlist, saving, and three operating systems

Step 4 turns Ferrous Loom from a pattern loop into a small song editor
that you can keep working on tomorrow, on Linux, macOS or Windows:

- **many patterns**: make as many patterns as you like, name them, clone
  them, delete them. The channel rack and the piano roll always show the
  selected pattern,
- a **playlist** that fills the window: tracks of **clips**, each clip
  playing one pattern at one place in the song. Place clips with a click,
  drag them around, right-click to delete. The same pattern can be used
  many times; edit it once and every clip changes,
- **pattern and song mode**: PAT loops the selected pattern while you work
  on it, SONG plays the playlist from the start and loops at the end,
- **saving and loading** projects as `.loom` files, which are plain text
  you can read, with a dot in the window title when there are unsaved
  changes and a "Save changes?" question before anything is lost,
- **macOS and Windows** builds, made and tested by CI on every push next
  to the Linux one.

![The demo song in the playlist, with the channel rack showing the Beat pattern](../images/step-4-playlist.png)

The demo is now an eight-bar song made of three patterns: two bars of
chords alone, then the beat comes in, and a drum fill ends it before it
loops.

## How to use it

| Do this | To |
|---------|----|
| Space, or ▶ Play | play or stop |
| PAT / SONG, or L | loop the selected pattern, or play the song |
| Click a pattern in the list on the left, or pick it at the top | select it: the rack shows it, PAT plays it, new clips use it |
| ➕ next to the pattern menu | add an empty pattern |
| ⋯ next to the pattern menu | rename, clone or delete the selected pattern |
| Click an empty place in the playlist | place a clip of the selected pattern |
| Drag a clip | move it to another track or time (it snaps to bars, beats or steps; see Snap) |
| Right-click a clip, or right-drag across clips | delete them |
| Click a track name | rename the track |
| Click the light next to a track | mute or unmute the track |
| F6, or Channel rack | show or hide the channel rack |
| F7, or 🎹 Piano roll | show or hide the piano roll |
| Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+Shift+S | new, open, save, save as (⌘ instead of Ctrl on a Mac) |
| Drop a `.loom` file on the window | open it |
| `ferrous-loom song.loom` | start with a file open |

![The pattern menu: rename, clone or delete](../images/step-4-pattern-menu.png)

## What changed in the architecture

```text
  loom-core                                    loom-audio
  ─────────                                    ──────────
  Project                                      AudioThread
    channels: Vec<Channel>                       channels[c].notes: ChannelNotes
    patterns: Vec<Pattern>  ──SetNotes{p,c}──▶     (one list per pattern)
    playlist: Playlist      ──SetSong─────────▶  clips, song length
      tracks, clips                              pattern_lengths
  playlist.rs (new): Track, Clip                 mode: Pattern | Song
  file.rs (new): ProjectFile, save, load

  loom-app
    playlist.rs (new): the playlist and the pattern picker
    app.rs: the playlist fills the window; rack and piano roll float
    app/files.rs (new): New, Open, Save, "Save changes?", the title
```

Three ideas carry this step:

1. **Things that refer to each other by number must change together.** A
   clip says "pattern 2". Remove pattern 1 and that clip must now say
   "pattern 1". The project keeps these rules by owning all the lists
   and only letting them change through its methods.
2. **A file is a contract with the future.** The file format has its own
   types, separate from the ones the program works with, and a version
   number. Loading a file goes through the same checks as editing.
3. **Write portable code, then prove it.** Every library was chosen in
   step 1 to work on all three systems. This step adds the CI jobs that
   build and test on each one, so a mistake shows up on the day it is made.

## Files in this step

```text
crates/loom-core/src/playlist.rs     Track, Clip, Playlist (new)
crates/loom-core/src/project.rs      many patterns, the playlist, the demo song
crates/loom-core/src/pattern.rs      patterns have names; no duplicate notes
crates/loom-core/src/file.rs         project files: save, load, FileError (new)
crates/loom-core/examples/print_demo.rs   prints the demo as a project file (new)
crates/loom-core/src/{pitch,tempo}.rs and harmony/scale.rs   serde derives
crates/loom-audio/src/engine.rs      ChannelNotes, PlayMode, song playback
crates/loom-app/src/playlist.rs      the playlist window (new)
crates/loom-app/src/app.rs           patterns, modes, the new layout
crates/loom-app/src/app/files.rs     New, Open, Save, unsaved changes (new)
crates/loom-app/src/channel_rack.rs  shows the selected pattern, in a window
crates/loom-app/src/piano_roll.rs    edits the selected pattern
crates/loom-app/src/main.rs          a file to open, no console on Windows
.github/workflows/ci.yml             Linux, macOS and Windows jobs
```

The chapters follow the data: first patterns and the playlist in
`loom-core`, then how the audio thread plays a song, then files, then the
windows on screen, and last the other operating systems.
