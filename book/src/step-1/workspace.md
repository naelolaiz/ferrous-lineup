# Cargo, workspaces and crates

## Cargo

**Cargo** is Rust's build tool and package manager in one. It reads
`Cargo.toml` files, downloads dependencies from [crates.io](https://crates.io),
compiles everything in the right order, runs tests and much more. You will
almost never call the compiler `rustc` directly.

## Crates

A **crate** is Rust's unit of compilation: one library or one program.
At this step, Ferrous Lineup was split into three crates:

| Crate | Kind | Job |
|-------|------|-----|
| `lineup-core` | library | Music concepts: notes, tempo, later patterns and chords. No audio, no UI. |
| `lineup-audio` | library | Talks to the sound card and generates sound. |
| `lineup-app` | binary (program) | The window. Produces the `ferrous-lineup` executable. |

> **At this step / today:** the workspace had these three crates and
> version 0.1.0. Today there are five: `lineup-listen` (decoding and
> analysing a recording for **File > Import recording**) and
> `lineup-video` (painting egui pictures into an MP4) joined them. The
> source links in this chapter go to `main`, so the files they open are
> the final ones, not the step-1 snapshot.

Why three crates instead of one?

- **Clear boundaries.** `lineup-core` *cannot* accidentally depend on the UI,
  because it does not list `eframe` as a dependency. The compiler enforces
  the architecture for us.
- **Faster tests.** `cargo test -p lineup-core` compiles only a tiny crate, not
  the whole GUI toolkit.
- **Reuse.** Later we could build a command-line renderer or a plugin on top
  of `lineup-core` and `lineup-audio` without touching the window code.

The dependencies only point one way:

```text
lineup-app ──▶ lineup-audio ──▶ lineup-core
    └─────────────────────────▲
```

### Package name and program name

The program is built from the package `lineup-app`, but the file Cargo
writes is called `ferrous-lineup`. The two names live in different
places of
[`crates/lineup-app/Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/Cargo.toml):

```toml
[package]
name = "lineup-app"

[[bin]]
name = "ferrous-lineup"
path = "src/main.rs"
```

`[package] name` is how Cargo and other crates refer to the package
(`cargo test -p lineup-app`). `[[bin]]` describes one program the
package builds; the double brackets are TOML for "one entry of a list",
because a package can build several programs. Without a `[[bin]]`
entry, Cargo would name the program after the package, `lineup-app`.
A crate called `lineup-core` is written `lineup_core` in `use` lines,
because a hyphen is a minus sign in Rust and cannot be part of a name;
Cargo turns the hyphens into underscores for you.

## The workspace

A **workspace** is a group of crates that are built together and share one
`Cargo.lock` file and one `target/` output folder. Look at the top-level
[`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml):

```toml
[workspace]
resolver = "3"
members = ["crates/lineup-core", "crates/lineup-audio", "crates/lineup-app"]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "GPL-3.0-or-later"
...

[workspace.dependencies]
lineup-core = { path = "crates/lineup-core" }
cpal = "0.18"
...
```

- `members` lists the crates.
- `[workspace.package]` holds settings shared by every crate. Each crate
  then writes `version.workspace = true` to inherit them, so the version
  number lives in one place.
- `[workspace.dependencies]` pins each dependency version once. A crate
  says `cpal.workspace = true` to use it. This keeps all crates on the
  same version of every library.

A version like `"0.18"` means "0.18.0 or any newer 0.18.x". The exact
versions Cargo picked are recorded in `Cargo.lock`, which is committed to
the repository so everyone builds exactly the same code.

### Editions

`edition = "2024"` picks the **edition** of the Rust language. Editions
let Rust make small breaking changes every few years without breaking old
code: each crate declares which edition it is written in, and crates of
different editions work together fine.

## Modules

Inside a crate, code is organized in **modules**. A module is usually one
file. In [`crates/lineup-core/src/lib.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lib.rs):

```rust
pub mod pitch;   // loads src/pitch.rs as the module `pitch`
pub mod tempo;   // loads src/tempo.rs as the module `tempo`

pub use pitch::Note;   // re-export: users can write lineup_core::Note
pub use tempo::Tempo;
```

- `mod pitch;` tells the compiler to look for `pitch.rs` next to `lib.rs`.
- `pub` makes it visible outside the crate. Without `pub`, everything in
  Rust is **private** by default, visible only inside its own module.
- `use` brings a name into scope, like `import` in other languages.
  `pub use` also re-exports it.

`lib.rs` is the **crate root** of a library; `main.rs` is the crate root of
a program. Note that the crate is called `lineup-core` in `Cargo.toml` but
`lineup_core` in code: Rust code cannot contain `-` in names, so Cargo
replaces it with `_`.

## Doc comments

Comments starting with `///` document the item below them, and `//!`
documents the enclosing module or crate. They are written in Markdown.
Run `cargo doc --open` to see them rendered as a browsable website: every
type and function in Ferrous Lineup shows up with its explanation.
