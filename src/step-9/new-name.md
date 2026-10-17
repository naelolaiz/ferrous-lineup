# A new name

Files: [`Cargo.toml`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/Cargo.toml),
[`crates/lineup-app/Cargo.toml`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/Cargo.toml),
[`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/main.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/midi.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/file.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/midi_file.rs)
and [`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/.github/workflows/ci.yml)

Ferrous Loom is now **Ferrous Lineup**. Renaming a program sounds like a
search and replace, and mostly it is. The interesting parts are the
places where a name is a promise to something that already exists: files
saved by the old version, and programs that read them.

## What changed

Everything a person sees or types, and the names of the files:

| Where | Before | After |
|-------|--------|-------|
| The window title | `song.loom · Ferrous Loom` | `song.lineup · Ferrous Lineup` |
| Project files | `song.loom` | `song.lineup` (and `song.loom` still opens) |
| The program's file name | `ferrous-loom` | `ferrous-lineup` |
| The crates (folders under `crates/`) | `loom-core`, `loom-audio`, `loom-listen`, `loom-app` | `lineup-core`, `lineup-audio`, `lineup-listen`, `lineup-app` |
| In the code | `use loom_core::Project;`, `LoomApp` | `use lineup_core::Project;`, `LineupApp` |
| The CI downloads | `ferrous-loom-linux`, … | `ferrous-lineup-linux`, … |
| The name other MIDI programs see | `Ferrous Loom` | `Ferrous Lineup` |
| The track name in exported `.mid` files | `Ferrous Loom` | `Ferrous Lineup` |
| The open and save dialogs | "Ferrous Loom project" | "Ferrous Lineup project" |
| A decoded recording beside the original | `song.ferrous-loom.wav` | `song.ferrous-lineup.wav` |
| Messages, the README, the comments, this book | Ferrous Loom | Ferrous Lineup |

## Renaming a crate

A crate's name lives in three places, and all three had to change
together.

1. **Its folder**, `crates/loom-core` became `crates/lineup-core`. Git
   records this as a rename (`git mv`), so `git log --follow` still
   finds a file's history from before.
2. **Its `[package] name`**, in its own `Cargo.toml`, and every place
   that names it: the workspace's list of `members` and its
   `[workspace.dependencies]` in the top
   [`Cargo.toml`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/Cargo.toml),
   and the crates that depend on it:

   ```toml
   [workspace]
   members = ["crates/lineup-core", "crates/lineup-audio", "crates/lineup-listen", "crates/lineup-app"]

   [workspace.dependencies]
   lineup-core = { path = "crates/lineup-core" }
   ```

   `Cargo.lock` lists the packages by name too; Cargo rewrites it on the
   next build.
3. **Its name in Rust code.** A crate called `lineup-core` is written
   `lineup_core` in `use` lines, because a hyphen is a minus sign in Rust
   and cannot be part of a name. Cargo turns the hyphens into
   underscores for you, so `use loom_core::Project;` became
   `use lineup_core::Project;` everywhere.

The app's own type, `LoomApp`, became `LineupApp` with them, so the code
you read says the same name as the window.

## Old project files keep opening

Every song saved so far ends in `.loom`. A program that only knew
`.lineup` would no longer see them in its Open dialog, and dropping one
on the window would do nothing. So
[`file.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/file.rs)
knows both:

```rust
pub const EXTENSION: &str = "lineup";

/// The extension projects had while the app was called Ferrous Loom.
/// Such files open as before, and saving one keeps its name.
pub const OLD_EXTENSION: &str = "loom";

pub fn is_project(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        [EXTENSION, OLD_EXTENSION]
            .iter()
            .any(|known| extension.eq_ignore_ascii_case(known))
    })
}
```

`path.extension()` is the part after the last dot, without the dot, or
`None` when there is none. It is an `OsStr`, the operating system's kind
of text, not a Rust `String`, because file names on Linux need not be
valid UTF-8. `eq_ignore_ascii_case` compares it with our text, so
`SONG.LOOM` counts too.

The Open dialog lists both kinds, the window opens both when they are
dropped on it, and the command line takes both. Save As always suggests
`.lineup`. Ctrl+S on a `.loom` file you opened writes that same file,
under its old name: saving to a new `song.lineup` beside it would leave
two copies of the song, and the next time you opened the old one, it
would not have your changes. The contents are the same either way; only
the name differs.

## What kept the old name

- **The Loom Osc** synth and the **Loom row** are names of things inside
  the app. They are saved in projects (a channel called "Loom Osc"), so a
  new name would make old projects look different from new ones.
- **The hidden key in MIDI files.** When Ferrous Lineup exports a
  `.mid` file it writes the exact key, such as E Dorian, in an event of
  its own that starts with the bytes `\x7DFerrous Loom key `
  ([MIDI file import](../step-8/midi-import.md) reads it back). That
  text is a tag, not a label: if the new version looked for
  `Ferrous Lineup key `, every file exported before the rename would lose
  its key on import. The tag stays, with a comment saying why.
- **The repository** is still `naelolaiz/ferrous-loom` until it is
  renamed on GitHub. GitHub sends the old address on to the new one, so
  links keep working after that.

## Package name and program name

The program is built from the package `lineup-app`, but the file Cargo
writes is called `ferrous-lineup`. The two names live in different
places of
[`crates/lineup-app/Cargo.toml`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/Cargo.toml):

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

## Where the window size is remembered

`main` hands the app's name to eframe:

```rust
eframe::run_native(
    "Ferrous Lineup",
    options,
    Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc, path)))),
)
```

eframe uses that name for two things: the window title when none is
given, and the folder where it keeps what it remembers between runs
(the window's size and place, and the zooms). On Linux that is
`~/.local/share/ferrouslineup`, on macOS
`~/Library/Application Support/Ferrous-Lineup` and on Windows
`%APPDATA%\Ferrous Lineup\data`. The folder changed with the name, so the
first start after the rename opens the window at its first size and the
zooms at 100%, once. Copying the old folder over would have kept them;
for one window size it was not worth the code.

## Tests pin names too

Some tests changed with the name. `FileState::title` is tested with
whole titles such as `"song.lineup* · Ferrous Lineup"`, `is_project`
with both extensions, and the MIDI menu's `is_own` with port names:

```rust
fn is_own(name: &str) -> bool {
    name.starts_with(&format!("{CLIENT_NAME}:"))
}

assert!(is_own("Ferrous Lineup:output 130:0"));
assert!(!is_own("Ferrous Lineupish Synth:in 20:0"));
```

On Linux the app's own ports show up in its port lists, and picking one
would send every note round in a loop, so the menu leaves them out. The
colon is why the second line holds: a synth whose name merely starts
with "Ferrous Lineup" is someone else's.

## Exercises

1. The app's name is written out in `main.rs` (twice), `files.rs`,
   `midi.rs`, `midi_file.rs` and a few messages. Put it in one
   `pub const APP_NAME: &str` in `lineup-core` and use it everywhere.
   Which places cannot use it, because they need the old name on
   purpose?
2. Make Ctrl+S on a `.loom` file save it as `.lineup` instead, and
   delete the old file only once the new one is safely written. What
   should happen to the title, and to the file that was open?
3. Make `ferrous-lineup --version` print the version and quit, with
   `env!("CARGO_PKG_VERSION")`, a macro that Cargo fills in at compile
   time from `Cargo.toml`.
