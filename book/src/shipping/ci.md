# Three systems and continuous integration

Files: [`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/ci.yml),
[`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs),
[`crates/lineup-app/build.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/build.rs)
and [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)

Ferrous Lineup runs on Linux, macOS and Windows from one code base.
Almost none of its code knows which system it is on, and three
workflows on GitHub check every change on all three. This chapter is
about how a Rust program stays portable, and how the checks run.

## Who talks to the operating system

Ferrous Lineup never calls the operating system for sound, windows or
dialogs itself. Four crates do:

| Job | Crate | Linux | macOS | Windows |
|-----|-------|-------|-------|---------|
| Sound | `cpal` | ALSA (also through PipeWire or PulseAudio) | Core Audio | WASAPI |
| Window, keys and mouse | `winit`, inside `eframe` | X11 or Wayland | AppKit | Win32 |
| Drawing | `wgpu`, inside `eframe` | Vulkan or OpenGL | Metal | DirectX 12 or Vulkan |
| File dialogs | `rfd` | the desktop portal | the Finder's open and save panels | the Explorer dialog |

Each of these crates holds the code for every system and keeps only the
right one while compiling. The app calls `cpal::default_host()`, draws
with egui and opens an `rfd::FileDialog`; none of that names a system.
Each library was chosen for that: anything that worked on only one
system would have to be replaced to reach the others.

The workspace's own code names a system in four places:

- `main.rs` hides the console window on Windows (below);
- `build.rs` puts the icon and the version into the Windows `.exe`;
- the lead sheet chooses `cmd`, `open` or `xdg-open` to show a PDF in the
  system's viewer;
- the sounds window shows a folder inside the home folder as `~/...`
  only on Linux and macOS, where people write paths that way.

## How a crate keeps one system's code: `cfg`

Inside `cpal`, the default host is chosen like this (shortened):

```rust,ignore
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod platform_impl {
    pub fn default_host() -> Host {
        AlsaHost::new().expect("the default host should always be available").into()
    }
}

#[cfg(target_vendor = "apple")]
mod platform_impl {
    pub fn default_host() -> Host {
        CoreAudioHost::new().expect("the default host should always be available").into()
    }
}
```

Two modules with the same name would normally be an error. But
`#[cfg(...)]` is a **conditional compilation** attribute: when its
condition is false, the item under it is removed before the compiler
even looks at it. On Linux only the first module exists; on a Mac only
the second. Conditions can ask about:

- the system you build for: `target_os = "windows"`, `target_os =
  "macos"`, `target_family = "unix"` (or just `unix`), `target_vendor =
  "apple"`;
- the kind of build: `debug_assertions` is on in debug builds, `test`
  while running `cargo test` (every `#[cfg(test)] mod tests` uses it);
- a crate's **features**: `feature = "wayland"`;
- and combinations, with `any(..)`, `all(..)` and `not(..)`.

Cargo can do the same for dependencies. `cpal`'s own `Cargo.toml` asks
for the ALSA library only on systems that have it:

```toml
[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
alsa = "0.11"
```

There is also a `cfg!(...)` macro, which gives `true` or `false` as an
ordinary value. The lead sheet uses it:

```rust
let mut command = if cfg!(target_os = "windows") {
```

The difference matters. With `cfg!`, every branch is still compiled, so
every branch must be valid on every system; with `#[cfg]`, code for other
systems is never compiled at all, which is why it can call functions
that only exist there. That is also its danger: a typo in a
`#[cfg(windows)]` function goes unnoticed on Linux, because the compiler
never reads it. Code you cannot compile is code you cannot trust, which
is why it is best kept inside a few well-tested crates, and why the
continuous integration below builds on every system.

## Features: choosing the parts of a crate

The `rfd` line in the workspace's `Cargo.toml` turns part of the crate
off:

```toml
rfd = { version = "0.17", default-features = false, features = ["xdg-portal"] }
```

A **feature** is an optional part of a crate, named in its `Cargo.toml`.
Some are on unless you say otherwise: the `default` features. `rfd`
normally turns on two: `xdg-portal`, which asks the Linux desktop to
show its own file dialog (over D-Bus, the desktop's message system, so
there is no GTK to install), and `wayland`, which tells the portal which
window the dialog belongs to. Ferrous Lineup doesn't tell it, so
`default-features = false` drops `wayland`, and `features =
["xdg-portal"]` keeps the one that is needed. Fewer features means fewer
crates to compile. On macOS and Windows the Linux feature changes
nothing: the `#[cfg]`s inside `rfd` leave its code out.

Features can only *add*. If two crates in a program use `rfd`, one with
`wayland` and one without, Cargo builds it once, with `wayland`. So a
feature must never take something away: a crate with it must still do
everything it did without it.

## No console window on Windows: `cfg_attr`

On Windows, every program is either a *console* program, which opens a
terminal window when started, or a *windows* program, which does not.
Rust makes console programs unless told otherwise, so double-clicking
`ferrous-lineup.exe` would open an empty black window next to the app.
One line at the top of `main.rs` says otherwise:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

- `windows_subsystem = "windows"` asks for the second kind. On other
  systems the attribute does nothing.
- `#![...]`, with `!`, applies to the thing it is inside, here the whole
  crate. `#[...]` without `!` applies to the next item.
- `cfg_attr(condition, attribute)` applies the attribute only when the
  condition is true. `not(debug_assertions)` is true in release builds.

So `cargo run`, a debug build, keeps the console, where `println!` and
panic messages show while you work, and `cargo build --release` makes
the quiet program you give to other people. Those messages then go
nowhere on Windows, which is one more reason the app shows audio errors
and the "Could not open" and "Could not save" messages inside its own
window.

## A build script for the `.exe`

`crates/lineup-app/build.rs` is a **build script**: Cargo compiles and
runs it before compiling the crate. On Windows it puts the icon, the
version and the copyright into the `.exe`, so Explorer and the taskbar
show the app's icon and the file's Properties page its name:

```rust
#[cfg(windows)]
{
    let version = env!("CARGO_PKG_VERSION");
```

Here `#[cfg(windows)]` describes the computer doing the build, because
a build script runs there: the resource compiler it calls is
Microsoft's, which only Windows has. The release workflow builds the
Windows program on Windows, so the icon is always there. macOS and Linux
take their icons from the bundles and desktop files in `packaging/` (see
[Release packages](releases.md)).

## Small differences the code already handles

- **Shortcuts:** `Modifiers::COMMAND` is Ctrl on Linux and Windows and ⌘
  on macOS.
- **Paths:** `PathBuf` and `Path` handle `/` and `\`, drive letters and
  file names that are not valid text. `std::env::args_os` passes the file
  to open exactly as the system gave it.
- **Replacing a file:** `std::fs::rename` replaces an existing file on
  every system, which [safe
  saving](../program/saving.md#saving-without-losing-the-old-file)
  depends on.
- **Sound:** cpal hands each system's audio callback the same
  `&mut [f32]`, so the audio thread never knows where it runs.

## Three workflows

A **workflow** is a file in `.github/workflows/` that tells GitHub what
to run, on which machines, and when. Ferrous Lineup has three:

| Workflow | Runs on | Does |
|---|---|---|
| **CI** (`ci.yml`) | every pull request and every push to main | formats, lints, tests and builds on Linux, macOS and Windows |
| **Release** (`release.yml`) | every pull request, push to main and `v` tag | builds the downloads for every system, and publishes them for a tag (see [Release packages](releases.md)) |
| **Book** (`book.yml`) | changes to the books and manuals | builds the websites and PDFs, and publishes them from main (see [How the books are built](books.md)) |

A pull request shows a check for each job, green when it passed and
red when it failed, with a link to its log.

### One job, three machines: the matrix

The CI workflow has one job, `check`, run three times:

```yaml
check:
  name: Build and test (${{ matrix.name }})
  strategy:
    fail-fast: false
    matrix:
      include:
        - name: Linux
          os: ubuntu-latest
          artifact: ferrous-lineup-linux
          binary: target/release/ferrous-lineup
        - name: macOS
          os: macos-latest
          artifact: ferrous-lineup-macos
          binary: target/release/ferrous-lineup
        - name: Windows
          os: windows-latest
          artifact: ferrous-lineup-windows
          binary: target/release/ferrous-lineup.exe
  runs-on: ${{ matrix.os }}
```

- A **matrix** turns one job into several. Each entry under `include` is
  one copy of the job, with its own values for `name`, `os`, `artifact`
  and `binary`.
- `${{ matrix.os }}` is replaced by the entry's value, so `runs-on` picks
  a Linux, Mac or Windows machine, and the job's name says which ("Build
  and test (macOS)").
- `fail-fast: false` lets the other copies finish when one fails.
  Without it, a Windows failure would cancel the Mac job, and a second
  problem would show only after the first was fixed.

Each copy then runs the same steps:

1. **Install system libraries**, on Linux only: the development files
   for ALSA, xkbcommon and Wayland. macOS and Windows come with theirs.
2. **Rust**, the newest stable, with `rustfmt` and `clippy`.
3. **A cache** of Cargo's downloads and compiled dependencies
   (`Swatinem/rust-cache`), so a run compiles only what changed.
4. **Check formatting**, on Linux only: formatting is the same
   everywhere.
5. **Lint**: `cargo clippy --workspace --all-targets -- -D warnings`.
6. **Test**: `cargo test --workspace`, in a debug build (see [Running
   them](testing.md#running-them)).
7. **Build release**, and upload the program as the run's artifact.

The steps that only make sense on one system say so:

```yaml
- name: Install system libraries
  if: runner.os == 'Linux'
```

Clippy and the tests run on all three: clippy because each system
compiles slightly different code, and the tests because a test that
passes on one system can fail on another (a path written with `/`, a
file that cannot be renamed while it is open).

On the page of a run, under **Artifacts**, are the three programs.
GitHub zips each, and the zip forgets that the program was executable,
so on Linux and macOS it needs `chmod +x ferrous-lineup` after
unzipping. The proper downloads, with installers, come from the Release
workflow.

## Checking Windows code from Linux

Waiting for CI to find a Windows mistake is slow. Rust can check code
for another system without leaving Linux:

```sh
rustup target add x86_64-pc-windows-gnu
cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu -- -D warnings
```

A **target** is the system you build *for*, as opposed to the one you
build *on*. `rustup target add` downloads Rust's standard library
compiled for Windows. `cargo clippy` and `cargo check` only run the
compiler's checks and never link a program, so they need neither
Windows' linker nor its system libraries. That catches most mistakes, a
Windows-only `#[cfg]` that does not compile or a crate that does not
support Windows at all, though not what only happens when the program
runs. The same works for Macs with `aarch64-apple-darwin` (Apple's
processors) or `x86_64-apple-darwin` (Intel Macs), as long as no
dependency needs Apple's own tools to build.

## What the checks cannot prove

The CI machines have no sound card and no screen. The jobs prove that
the program builds and that the music logic is right on each system,
not that you hear anything: the window tests draw into memory (see
[Window tests](testing.md#window-tests)), and the audio
tests render into a buffer. A run on a real Mac or Windows computer is
still the real test of sound and drawing.
