# Ferrous Loom

An open source, pattern-based music studio written in Rust, inspired by the
workflow of FL Studio. Build patterns in a step sequencer or piano roll,
arrange them into songs, and learn harmony along the way.

> Ferrous Loom is an independent project. It is not affiliated with
> Image-Line or FL Studio.

![Ferrous Loom with the playlist and the channel rack](screenshots/step-4-playlist.png)

## Status

Early development. Runs on Linux, macOS and Windows.

| Step | Contents | Status |
|------|----------|--------|
| 1 | Window, audio engine, test tone, CI | Done |
| 2 | Channel rack, step sequencer, drum kit, sampler, synth | Done |
| 3 | Piano roll with harmony helpers | Done |
| 4 | Playlist, saving, macOS and Windows | Done |

## Build and run

Install Rust with [rustup](https://rustup.rs), then the system's build
tools:

- **Linux** (Debian/Ubuntu):
  `sudo apt install build-essential pkg-config libasound2-dev libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev`
- **macOS**: `xcode-select --install`
- **Windows**: the Visual Studio Build Tools, which `rustup-init.exe`
  offers to install

```sh
cargo run --release
```

Press **Space** to play the demo song: two bars of a C, Am, F, G chord
progression, then a beat comes in, and a drum fill ends it. The playlist
arranges patterns into a song: click to place a clip of the selected
pattern, drag clips around, right-click to delete. **F6** shows or hides
the channel rack, where you click steps and drop WAV files to use your
own sounds. **F7** shows the piano roll: pick a key to highlight its notes,
draw notes or whole chords, and watch the chord names above the grid.
**Ctrl+S** saves the project as a `.loom` file (⌘S on a Mac).

Ready-built programs for all three systems are attached to every CI run
as `ferrous-loom-linux`, `ferrous-loom-macos` and `ferrous-loom-windows`.
They are not signed yet, so macOS and Windows warn before the first start.

## Project layout

| Path | Contents |
|------|----------|
| `crates/loom-core` | Music types and theory: notes, scales, chords and chord naming, tempo, patterns, the playlist, project files, the sequencer clock. No audio, no UI. |
| `crates/loom-audio` | Real-time audio engine on top of `cpal`: pattern and song playback, drum synthesis, a pitched sampler, a polyphonic synth. |
| `crates/loom-app` | The desktop app (`ferrous-loom`), built with `egui`. |

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
