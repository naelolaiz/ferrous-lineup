# Step 4: The playlist, saving, and three operating systems

Step 4 turns Ferrous Lineup from a pattern loop into a small song editor
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
- **saving and loading** projects as `.lineup` files, which are plain text
  you can read, with a star in the window title when there are unsaved
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
| Drop a `.lineup` file on the window | open it |
| `ferrous-lineup song.lineup` | start with a file open |

![The pattern menu: rename, clone or delete](../images/step-4-pattern-menu.png)

## What changed in the architecture

```text
  lineup-core                                    lineup-audio
  ─────────                                    ──────────
  Project                                      AudioThread
    channels: Vec<Channel>                       channels[c].notes: ChannelNotes
    patterns: Vec<Pattern>  ──SetNotes{p,c}──▶     (one list per pattern)
    playlist: Playlist      ──SetSong─────────▶  clips, song length
      tracks, clips                              pattern_lengths
  playlist.rs (new): Track, Clip                 mode: Pattern | Song
  file.rs (new): ProjectFile, save, load

  lineup-app
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
crates/lineup-core/src/playlist.rs     Track, Clip, Playlist (new)
crates/lineup-core/src/project.rs      many patterns, the playlist, the demo song
crates/lineup-core/src/pattern.rs      patterns have names; no duplicate notes
crates/lineup-core/src/file.rs         project files: save, load, FileError (new)
crates/lineup-core/src/{pitch,tempo}.rs and harmony/scale.rs   serde derives
crates/lineup-audio/src/engine.rs      ChannelNotes, PlayMode, song playback
crates/lineup-app/src/playlist.rs      the playlist window (new)
crates/lineup-app/src/app.rs           patterns, modes, the new layout
crates/lineup-app/src/app/files.rs     New, Open, Save, unsaved changes (new)
crates/lineup-app/src/channel_rack.rs  shows the selected pattern, in a window
crates/lineup-app/src/piano_roll.rs    edits the selected pattern
crates/lineup-app/src/main.rs          a file to open, no console on Windows
.github/workflows/ci.yml             Linux, macOS and Windows jobs
```

The chapters follow the data: first patterns and the playlist in
`lineup-core`, then how the audio thread plays a song, then files, then the
windows on screen, and last the other operating systems.

> **Changed in step 5.** The code links in these chapters point to the
> latest code, which step 5 changed in a few places. The audio thread's
> `AddPattern` and `AddChannel` became `InsertPattern` and
> `InsertChannel`, which take a position (`ChannelNotes::push_empty`
> became `insert_empty`). The app no longer sends commands from each
> action: every change goes through `edit`, which compares the project
> before and after. And `Instrument::for_channel` takes a `SampleCache`.
> [Step 5](../step-5/README.md) explains why.
>
> **Changed later still.** Step 8 gave clips a `repeats` count, and
> `SetSong` now carries `plays`: the pieces of patterns with repeats and
> sections already unrolled. The chapters say where that changed.
