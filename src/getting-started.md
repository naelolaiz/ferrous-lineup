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

Ferrous Loom needs Rust 1.88 or newer. It uses the 2024 edition of the
language (more on editions in the next chapter) and `if let` chains, which
arrived in 1.88. If your Rust is older, `rustup update` fixes it.

## 2. Install the Linux system libraries

Rust downloads its own libraries (called *crates*) automatically, but a few
crates talk to libraries that come with Linux itself: ALSA for sound and
X11 or Wayland for windows. On Debian or Ubuntu:

```sh
sudo apt install build-essential pkg-config libasound2-dev \
    libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev
```

On Fedora:

```sh
sudo dnf install gcc pkgconf-pkg-config alsa-lib-devel \
    libxkbcommon-devel libxkbcommon-x11 wayland-devel
```

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

![The Ferrous Loom channel rack](images/step-2-rack.png)

Press **Space** or click **▶ Play** to hear the demo beat. The status bar at
the bottom shows which sound card is in use, or why audio could not start.

## 4. Run the tests

```sh
cargo test
```

This runs every test in every crate of the workspace. The
[Tests and continuous integration](step-1/ci.md) chapter explains how they
are written.

## 5. Build this book

The book is written with [mdBook](https://rust-lang.github.io/mdBook/), a
Rust tool. Install it once, then build:

```sh
cargo install mdbook
mdbook build docs
```

Open `docs/book/index.html` in your browser. It works without internet.
While writing, `mdbook serve docs --open` rebuilds the pages every time you
save a file.

## Useful commands

| Command | What it does |
|---------|--------------|
| `cargo build` | Compile everything (debug mode) |
| `cargo run` | Compile and start the app |
| `cargo test` | Run all tests |
| `cargo fmt` | Format all code in the standard Rust style |
| `cargo clippy` | Run the linter, which suggests better ways to write things |
| `cargo doc --open` | Build API docs from the `///` comments and open them |
