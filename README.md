# Ferrous Loom

An open source, pattern-based music studio written in Rust, inspired by the
workflow of FL Studio. Build patterns in a step sequencer or piano roll,
arrange them into songs, and learn harmony along the way.

> Ferrous Loom is an independent project. It is not affiliated with
> Image-Line or FL Studio.

![The Ferrous Loom window](screenshots/step-1-window.png)

## Status

Early development, Linux first.

| Step | Contents | Status |
|------|----------|--------|
| 1 | Window, audio engine, test tone, CI | Done |
| 2 | Channel rack and step sequencer | Next |
| 3 | Piano roll with harmony helpers | Planned |
| 4 | Playlist, saving, macOS and Windows | Planned |

## Build and run (Linux)

```sh
# Debian/Ubuntu system libraries
sudo apt install build-essential pkg-config libasound2-dev \
    libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev

cargo run --release
```

Press **Space** to start and stop the test tone.

## Project layout

| Path | Contents |
|------|----------|
| `crates/loom-core` | Music types and theory: notes, tempo. No audio, no UI. |
| `crates/loom-audio` | Real-time audio engine on top of `cpal`. |
| `crates/loom-app` | The desktop app (`ferrous-loom`), built with `egui`. |

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
