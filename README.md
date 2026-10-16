# Ferrous Loom

An open source, pattern-based music studio written in Rust, inspired by the
workflow of FL Studio. Build patterns in a step sequencer or piano roll,
arrange them into songs, and learn harmony along the way.

> Ferrous Loom is an independent project. It is not affiliated with
> Image-Line or FL Studio.

![The Ferrous Loom channel rack](screenshots/step-2-rack.png)

## Status

Early development, Linux first.

| Step | Contents | Status |
|------|----------|--------|
| 1 | Window, audio engine, test tone, CI | Done |
| 2 | Channel rack, step sequencer, drum kit, sampler, synth | Done |
| 3 | Piano roll with harmony helpers | Next |
| 4 | Playlist, saving, macOS and Windows | Planned |

## Build and run (Linux)

```sh
# Debian/Ubuntu system libraries
sudo apt install build-essential pkg-config libasound2-dev \
    libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev

cargo run --release
```

Press **Space** to play the demo beat. Click steps to change it, and drop
WAV files on the channel rack to use your own sounds.

## Project layout

| Path | Contents |
|------|----------|
| `crates/loom-core` | Music types and theory: notes, tempo, patterns, the sequencer clock. No audio, no UI. |
| `crates/loom-audio` | Real-time audio engine on top of `cpal`: drum synthesis, sampler, synth. |
| `crates/loom-app` | The desktop app (`ferrous-loom`), built with `egui`. |

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
