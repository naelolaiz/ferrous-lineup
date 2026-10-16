# Cargo, workspaces and crates

## Cargo

**Cargo** is Rust's build tool and package manager in one. It reads
`Cargo.toml` files, downloads dependencies from [crates.io](https://crates.io),
compiles everything in the right order, runs tests and much more. You will
almost never call the compiler `rustc` directly.

## Crates

A **crate** is Rust's unit of compilation: one library or one program.
Ferrous Loom is split into three crates:

| Crate | Kind | Job |
|-------|------|-----|
| `loom-core` | library | Music concepts: notes, tempo, later patterns and chords. No audio, no UI. |
| `loom-audio` | library | Talks to the sound card and generates sound. |
| `loom-app` | binary (program) | The window. Produces the `ferrous-loom` executable. |

Why three crates instead of one?

- **Clear boundaries.** `loom-core` *cannot* accidentally depend on the UI,
  because it does not list `eframe` as a dependency. The compiler enforces
  the architecture for us.
- **Faster tests.** `cargo test -p loom-core` compiles only a tiny crate, not
  the whole GUI toolkit.
- **Reuse.** Later we could build a command-line renderer or a plugin on top
  of `loom-core` and `loom-audio` without touching the window code.

The dependencies only point one way:

```text
loom-app ──▶ loom-audio ──▶ loom-core
    └─────────────────────────▲
```

## The workspace

A **workspace** is a group of crates that are built together and share one
`Cargo.lock` file and one `target/` output folder. Look at the top-level
[`Cargo.toml`](https://github.com/naelolaiz/ferrous-loom/blob/main/Cargo.toml):

```toml
[workspace]
resolver = "3"
members = ["crates/loom-core", "crates/loom-audio", "crates/loom-app"]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "GPL-3.0-or-later"
...

[workspace.dependencies]
loom-core = { path = "crates/loom-core" }
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
file. In [`crates/loom-core/src/lib.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/lib.rs):

```rust
pub mod pitch;   // loads src/pitch.rs as the module `pitch`
pub mod tempo;   // loads src/tempo.rs as the module `tempo`

pub use pitch::Note;   // re-export: users can write loom_core::Note
pub use tempo::Tempo;
```

- `mod pitch;` tells the compiler to look for `pitch.rs` next to `lib.rs`.
- `pub` makes it visible outside the crate. Without `pub`, everything in
  Rust is **private** by default, visible only inside its own module.
- `use` brings a name into scope, like `import` in other languages.
  `pub use` also re-exports it.

`lib.rs` is the **crate root** of a library; `main.rs` is the crate root of
a program. Note that the crate is called `loom-core` in `Cargo.toml` but
`loom_core` in code: Rust code cannot contain `-` in names, so Cargo
replaces it with `_`.

## Doc comments

Comments starting with `///` document the item below them, and `//!`
documents the enclosing module or crate. They are written in Markdown.
Run `cargo doc --open` to see them rendered as a browsable website: every
type and function in Ferrous Loom shows up with its explanation.
