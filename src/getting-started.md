# Getting started

## 1. Install Rust

Rust is installed with **rustup**, a small tool that downloads the compiler
(`rustc`), the build tool (`cargo`) and keeps them up to date:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal afterwards and check it worked:

```sh
cargo --version
rustc --version
```

Ferrous Lineup needs Rust 1.88 or newer. It uses the 2024 edition of the
language (more on editions in the next chapter) and `if let` chains, which
arrived in 1.88. If your Rust is older, `rustup update` fixes it.

## 2. Install the system's build tools

Rust downloads its own libraries (called *crates*) automatically, but it
needs a linker from your system, and on Linux a few crates talk to
libraries that come with the system itself: ALSA for sound and X11 or
Wayland for windows.

### Linux

On Debian or Ubuntu:

```sh
sudo apt install build-essential pkg-config libasound2-dev \
    libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev
```

On Fedora:

```sh
sudo dnf install gcc pkgconf-pkg-config alsa-lib-devel \
    libxkbcommon-devel libxkbcommon-x11 wayland-devel
```

File dialogs use your desktop's portal, which GNOME and KDE already
have. On a bare window manager, install `xdg-desktop-portal-gtk` or
`zenity` to get them.

MIDI goes through the ALSA *sequencer*. Desktop systems load it
already; if Ferrous Lineup's MIDI menu says **MIDI unavailable**, load it
with `sudo modprobe snd-seq`.

### macOS

Install Apple's command line tools, which bring the linker and the
system libraries:

```sh
xcode-select --install
```

### Windows

Download and run `rustup-init.exe` from <https://rustup.rs> instead of
the `curl` command above. When it asks, let it install the Visual Studio
Build Tools: Rust uses their linker. Then use the commands below in
PowerShell or the Command Prompt.

## 3. Build and run

From the repository folder:

```sh
cargo run --release
```

The first build takes a few minutes because Cargo compiles every
dependency. Later builds only recompile what changed and take seconds.
`--release` turns on optimizations, which matters for audio code. During
development you can drop it to build faster.

You should see this window:

![Ferrous Lineup as it opens: the demo song on the playlist, the channel rack over it with Kick, Clap, Hat, Snare and the Lineup Osc, and the harmony keys of C major along the bottom.](images/getting-started-window.png)

Press **Space** or click **▶ Play** to hear the demo song: two bars of
chords, then a beat comes in, and a drum fill ends it before it loops.
The playlist fills the window; **F6** shows and hides the channel rack,
**F7** the piano roll, and **L** switches between playing the song and
looping the selected pattern. **Ctrl+S** (⌘S on a Mac) saves the
project, **Ctrl+Z** undoes the last change and **Ctrl+Shift+Z** redoes
it, and **File > Export WAV…** saves the song as a sound file. In the
piano roll, **Ctrl+click** selects notes, the **arrow keys** move them,
and the **Tools** menu lists everything else it can do to them, with
its keys. The **MIDI** menu connects a MIDI keyboard or synth, and
turns the letter keys into a piano; **● Rec** records what you play. The
status bar at the bottom shows which sound card is in use, or why audio
could not start, and names the chord you hold down.

To play a song of your own with the band, pick **File > New**. It opens
an empty **chord sheet**: click a bar and type its chords (`C`, `Am7`,
`G7/B`), and a piano, a bass and drums play them, each on a track of its
own. **🎷 Band** chooses the players, and the **Library** the style they
play in. **F5**, or **📝 Chords** in the toolbar, opens the chord sheet of
any song, the demos too. [Step 9](step-9/README.md) tells how the band
works.

Above the status bar are the **harmony keys**: the key and the time
signature, and a big key for each chord of the key. Hold one down, or
press **1** to **7**, to play its chord. The **Library** tab next to the
pattern list holds the styles and rhythms: click one, then **Apply**, to
hear the song played that way. **File > Demos** opens whole songs: try *Für Elise* or
*Frère Jacques*, press nothing, and watch the harmony keys follow the
chords.

[A tour of the app](tour.md) goes through every window, menu and key.

To open a project straight away, give its file to the program:

```sh
cargo run --release -- my-song.lineup
```

The `--` tells Cargo that what follows is for the program, not for Cargo.

### Without building: the CI downloads

Every CI run on GitHub builds the program for all three systems. On the
run's page, under **Artifacts**, download `ferrous-lineup-linux`,
`ferrous-lineup-macos` or `ferrous-lineup-windows` and unzip it. On Linux and
macOS, make it executable first with `chmod +x ferrous-lineup`. These
programs are not signed, so macOS and Windows will warn you before the
first start; [Linux, macOS and Windows](step-4/platforms.md#what-is-not-there-yet)
explains how to get past the warning.

The **Release** workflow packs the same programs ready to use, each
with the README and the licence: a `.tar.gz` and an AppImage for Linux,
a `.zip` for Windows, and builds for Macs with Apple Silicon and with
Intel. They are under **Artifacts** of every Release run, and on the
repository's **Releases** page for each version
([Release builds: a download for every system](step-9/releases.md)).

## 4. Run the tests

```sh
cargo test
```

This runs every test in every crate of the workspace. The
[Tests and continuous integration](step-1/ci.md) chapter explains how they
are written.

## 5. Build this book

The book is written with [mdBook](https://rust-lang.github.io/mdBook/), a
Rust tool. It lives on a branch of its own, `book`, which shares no
history with the app's code. Open that branch in a folder next to the
app, install mdBook once, then build:

```sh
git worktree add ../ferrous-lineup-book book
cd ../ferrous-lineup-book
cargo install mdbook --locked
mdbook build
```

Open `book/index.html` in your browser. It works without internet.
While writing, `mdbook serve --open` rebuilds the pages every time you
save a file. The same book is online, and as one PDF
([The book online and as a PDF](step-9/book-online.md)).

## Useful commands

| Command | What it does |
|---------|--------------|
| `cargo build` | Compile everything (debug mode) |
| `cargo run` | Compile and start the app |
| `cargo test` | Run all tests |
| `cargo fmt` | Format all code in the standard Rust style |
| `cargo clippy` | Run the linter, which suggests better ways to write things |
| `cargo doc --open` | Build API docs from the `///` comments and open them |
