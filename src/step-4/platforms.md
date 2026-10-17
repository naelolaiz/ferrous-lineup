# Linux, macOS and Windows

Files: [`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/ci.yml),
[`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)
and [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)

The plan from the start was "Linux first, the others at the end, without
a rewrite". Here is the whole change to the program's code for macOS and
Windows: one line in `main.rs`. Everything else in this chapter is about
*why* that is enough, and how the CI proves it on every push.

## Who talks to the operating system

Ferrous Lineup never calls the operating system for sound, windows or
dialogs itself. Four crates do:

| Job | Crate | Linux | macOS | Windows |
|-----|-------|-------|-------|---------|
| Sound | `cpal` | ALSA (also through PipeWire or PulseAudio) | Core Audio | WASAPI |
| Window, keys and mouse | `winit`, inside `eframe` | X11 or Wayland | AppKit | Win32 |
| Drawing | `wgpu`, inside `eframe` | Vulkan or OpenGL | Metal | DirectX 12 or Vulkan |
| File dialogs | `rfd` | the desktop portal, or `zenity` | the Finder's open and save panels | the Explorer dialog |

Each of these crates contains the code for every system, and keeps only
the right one while compiling. The app calls `cpal::default_host()`, draws
with egui and opens an `rfd::FileDialog`; none of that names an operating
system. A search of the workspace for `target_os` finds nothing.

This was a choice made in step 1, when each library was picked: anything
that only worked on Linux would have had to be replaced now.

## How a crate keeps one system's code: `cfg`

Inside `cpal`, the default host is chosen like this (shortened):

```rust
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
condition is false, the item under it is removed before the compiler even
looks at it. On Linux only the first module exists; on a Mac only the
second. Conditions can ask about:

- the system you build for: `target_os = "windows"`, `target_os =
  "macos"`, `target_family = "unix"`, `target_vendor = "apple"`;
- the kind of build: `debug_assertions` is on in debug builds, `test`
  while running `cargo test` (the `#[cfg(test)] mod tests` blocks in every
  file use it);
- a crate's **features**: `feature = "wayland"`;
- and combinations, with `any(..)`, `all(..)` and `not(..)`.

Cargo can do the same for dependencies. `cpal`'s own `Cargo.toml` only
asks for the ALSA library on systems that have it:

```toml
[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
alsa = "0.11"
```

There is also a `cfg!(...)` macro, which gives `true` or `false` as a
normal value (`if cfg!(target_os = "macos") { ... }`). The difference
matters: with `cfg!`, both branches are still compiled, so both must be
valid on every system. With `#[cfg]`, code for other systems is never
compiled at all, which is why it can call functions that only exist
there.

That is also its danger. A typo in a `#[cfg(windows)]` function goes
unnoticed on Linux, because the compiler never reads it. Code you cannot
compile is code you cannot trust, which is why it is best kept inside a
few well-tested crates, and why the CI below builds on every system.

## Features: choosing the parts of a crate

The `rfd` line in the workspace's `Cargo.toml` turns some of the crate
off:

```toml
rfd = { version = "0.17", default-features = false, features = ["xdg-portal"] }
```

A **feature** is an optional part of a crate, named in its `Cargo.toml`.
Some are on unless you say otherwise: those are the `default` features.
`serde`'s `derive` feature, from the saving chapter, is one you turn on.

`rfd` normally turns on two: `xdg-portal`, which asks the Linux desktop to
show its file dialog (over D-Bus, the desktop's message system, so there
is no GTK to install), and `wayland`, which tells the portal which window
the dialog belongs to, if the program says so. Ferrous Lineup does not, so
`default-features = false` drops `wayland`, and `features = ["xdg-portal"]`
keeps the one that is needed. Fewer features means fewer crates to
compile. On macOS and Windows the Linux feature makes no difference: the
`#[cfg]`s inside `rfd` leave its code out.

Features can only *add*. If two crates in your program use `rfd`, one with
`wayland` and one without, Cargo builds it once, with `wayland`. So a
crate's features must never take something away: a crate with `wayland`
must still do everything it did without it.

## No console window on Windows: `cfg_attr`

The one line of new code for Windows is the first item in `main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

On Windows, every program is either a *console* program, which opens a
terminal window when started, or a *windows* program, which does not.
Rust makes console programs unless told otherwise, so double-clicking
`ferrous-lineup.exe` would open an empty black window next to the app.

- `windows_subsystem = "windows"` asks for the second kind. On other
  systems the attribute does nothing.
- `#![...]`, with `!`, applies to the thing it is inside, here the whole
  crate. `#[...]` without `!` applies to the next item.
- `cfg_attr(condition, attribute)` applies the attribute only when the
  condition is true. `not(debug_assertions)` is true for release builds.

So `cargo run` (a debug build) keeps the console, where `println!` and
panic messages show up while you develop, and `cargo build --release`
makes the quiet program you give to other people. In a release build on
Windows, those messages go nowhere, which is one more reason the status
bar shows audio errors and the "Could not open" and "Could not save"
messages inside the window.

## Small differences the code already handles

- **Shortcuts:** `Modifiers::COMMAND` is Ctrl on Linux and Windows and ⌘
  on macOS (see [the saving chapter](saving.md#shortcuts)).
- **Paths:** `PathBuf` and `Path` handle `/` and `\`, drive letters and
  file names that are not valid text. `std::env::args_os` passes the file
  to open exactly as the system gave it.
- **Replacing a file:** `std::fs::rename` replaces an existing file on
  every system, which [safe saving](saving.md#saving-without-losing-the-old-file)
  depends on.
- **Floating-point sound:** cpal hands each system's audio callback the
  same `&mut [f32]`, so the audio thread does not know where it runs.

## Proving it: the CI matrix

Until this step, the CI's main job ran on one Linux machine. Now the same
job runs three times:

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

- A **matrix** turns one job into several. Each entry under `include`
  is one copy of the job, with its own values for `name`, `os`,
  `artifact` and `binary`.
- `${{ matrix.os }}` is replaced by the entry's value, so `runs-on`
  picks a Linux, Mac or Windows machine, and the job's name shows which
  one it is ("Build and test (macOS)").
- `fail-fast: false` lets the other jobs finish when one fails. Without
  it, a Windows failure would cancel the Mac job, and you would only find
  out about a second problem after fixing the first.

Some steps only make sense on one system:

```yaml
- name: Install system libraries
  if: runner.os == 'Linux'
  run: |
    sudo apt-get update
    sudo apt-get install -y libasound2-dev libxkbcommon-dev libwayland-dev

- name: Check formatting
  if: runner.os == 'Linux'
  run: cargo fmt --all --check
```

macOS and Windows come with their sound and window libraries, so only
Linux installs anything. Formatting is the same everywhere, so it is
checked once. Clippy, the tests and the release build run on all three:
clippy because, as the `cfg` section explained, each system compiles
slightly different code, and the tests because a test that passes on one
system can fail on another (a path with `/` in it, a file that cannot be
renamed while open).

Each job ends by uploading its program. On the page of a CI run, under
**Artifacts**, there are now four downloads: `ferrous-lineup-linux`,
`ferrous-lineup-macos`, `ferrous-lineup-windows` and the book. GitHub puts
each in a zip file, and zip files made this way forget that the program
was executable, so on Linux and macOS run `chmod +x ferrous-lineup` after
unzipping.

## Checking Windows code from Linux

Waiting for CI to find a Windows mistake is slow. Rust can type-check
your code for another system without leaving Linux:

```sh
rustup target add x86_64-pc-windows-gnu
cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu -- -D warnings
```

A **target** is the system you build *for*, as opposed to the one you
build *on*. `rustup target add` downloads Rust's standard library
compiled for Windows. `cargo clippy` (and `cargo check`) only run the
compiler's checks and never link a program, so they do not need Windows'
linker or its system libraries. This catches most mistakes: a
Windows-only `#[cfg]` that does not compile, or a crate that does not
support Windows at all. It cannot catch what only happens when the
program runs.

The same works for Macs with `aarch64-apple-darwin` (Apple's processors)
or `x86_64-apple-darwin` (older Intel Macs), as long as no dependency
needs Apple's own tools to build. Both checks passed for this step before
anything was pushed, so the first CI run on macOS and Windows was not a
leap in the dark.

## Building it yourself

The [Getting started](../getting-started.md) page has the steps for each
system. In short:

- **macOS:** install Apple's command line tools (`xcode-select
  --install`), which bring the linker and the system libraries, then
  rustup, then `cargo run --release`.
- **Windows:** install Rust with `rustup-init.exe` from rustup.rs. It
  offers to install the Visual Studio Build Tools, which Rust uses to
  link programs (the "Desktop development with C++" part). Then `cargo
  run --release` in a terminal.

## What is not there yet

Being honest about the limits:

- The CI machines have no sound card and no screen. The jobs prove that
  the program builds and that the music logic is right on each system,
  not that you hear anything. The first person to run it on a Mac or a
  Windows computer is the real test.
- The programs are not **signed**. macOS will refuse to open a
  downloaded one at first ("cannot be verified"); right-click it and
  choose Open, or run `xattr -d com.apple.quarantine ferrous-lineup`.
  Windows SmartScreen will warn too; choose "More info", then "Run
  anyway". Signing needs paid certificates from Apple and others.
- There is no installer, no icon, and no macOS `.app` bundle, so the Mac
  program is a file you start from the terminal, and double-clicking a
  `.lineup` file does not open it in Ferrous Lineup on any system. Each of
  these needs packaging work per system, which is a good later step.
