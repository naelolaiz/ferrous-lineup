# Five crates, and what each one may know

Files: [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml),
the `Cargo.toml` of each crate in
[`crates/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates),
and [`crates/lineup-core/src/lib.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lib.rs)

Ferrous Lineup is one program, `ferrous-lineup`, built from five Rust
crates. This chapter is about how they are split and why: what Cargo
is, what a crate, a workspace and a module are, a first small type and
the Rust it shows, and the few rules the whole program keeps, each explained in a chapter of its own later in
the book. The last section is a map of where each part of the app
lives.

## Cargo

**Cargo** is Rust's build tool and package manager in one. It reads
`Cargo.toml` files, downloads dependencies from [crates.io](https://crates.io),
compiles everything in the right order, runs tests and much more. You will
almost never call the compiler `rustc` directly.

## Crates

A **crate** is Rust's unit of compilation: one library or one program.
Ferrous Lineup is five crates, each with a job it can be tested on its
own:

| Crate | Kind | Job |
|-------|------|-----|
| `lineup-core` | library | The music and the song: notes, scales, chords and harmony, the chord sheet, patterns and the playlist, project files and diffs, undo history, MIDI files, lead sheets, the band, and the library of styles, rhythms and songs. No audio and no UI. |
| `lineup-audio` | library | The real-time engine and the offline renderer: drums and samplers, the Lineup Osc, SoundFont instruments, song and pattern playback, effects, WAV export, and the trace of every note played. |
| `lineup-listen` | library | Decodes a recording (MP3, WAV, FLAC or Ogg Vorbis) and listens for its beats, key, chords and meter. Experimental. |
| `lineup-video` | library | Paints the app's views in memory and encodes them as H.264 and MP3 in an MP4 file. |
| `lineup-app` | binary (program) | The window, built with `egui`, and MIDI through `midir`. It owns the state that joins the other four, and produces the `ferrous-lineup` executable. |

Why five crates instead of one?

- **Clear boundaries.** `lineup-core` *cannot* accidentally depend on the UI,
  because it does not list `eframe` as a dependency. The compiler enforces
  the architecture for us.
- **Faster tests.** `lineup-core` depends only on `serde` and `ron`, so
  `cargo test -p lineup-core` compiles a small crate, not the whole GUI
  toolkit, the audio drivers or the video encoders.
- **Heavy dependencies stay where they are needed.** Only
  `lineup-listen` decodes audio formats, and only `lineup-video`
  compiles the H.264 and MP3 encoders from their C sources.
- **Reuse.** A command-line renderer or a plugin could be built on top of
  `lineup-core` and `lineup-audio` without touching the window code.
  The playback trace already works this way: `ferrous-lineup --trace`
  plays a song through `lineup-audio` without opening a window (see
  [The playback trace](../sound/trace.md)).

The dependencies only point one way:

```text
lineup-app ──▶ lineup-audio ──▶ lineup-core
    │ ├──────▶ lineup-listen ──▶ lineup-core
    │ └──────▶ lineup-video
    └────────────────────────▶ lineup-core
```

`lineup-video` does not know about songs at all: it is given pictures
and sound and makes a file of them.

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
members = [
    "crates/lineup-core",
    "crates/lineup-audio",
    "crates/lineup-listen",
    "crates/lineup-video",
    "crates/lineup-app",
]

[workspace.package]
version = "0.4.0"
edition = "2024"
license = "GPL-3.0-or-later"
repository = "https://github.com/naelolaiz/ferrous-lineup"
rust-version = "1.95"

[workspace.dependencies]
lineup-core = { path = "crates/lineup-core" }
cpal = "0.18"
...
```

- `members` lists the crates.
- `[workspace.package]` holds settings shared by every crate. Each crate
  then writes `version.workspace = true` to inherit them, so the version
  number lives in one place. `rust-version` is the oldest Rust compiler
  that can build the code; an older one stops with a clear message
  instead of a confusing error.
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
// ... and about thirty more

pub use pitch::Note;   // re-export: users can write lineup_core::Note
pub use tempo::Tempo;
```

- `mod pitch;` tells the compiler to look for `pitch.rs` next to `lib.rs`.
- `pub` makes it visible outside the crate. Without `pub`, everything in
  Rust is **private** by default, visible only inside its own module.
- `use` brings a name into scope, like `import` in other languages.
  `pub use` also re-exports it.

A module can have modules of its own. `harmony.rs` declares
`pub mod chord;`, and the compiler finds it in `harmony/chord.rs`: the
folder has the name of the module it belongs to.

`lib.rs` is the **crate root** of a library; `main.rs` is the crate root of
a program. Note that the crate is called `lineup-core` in `Cargo.toml` but
`lineup_core` in code: Rust code cannot contain `-` in names, so Cargo
replaces it with `_`.

## Doc comments

Comments starting with `///` document the item below them, and `//!`
documents the enclosing module or crate. They are written in Markdown.
Run `cargo doc --open` to see them rendered as a browsable website: every
type and function in Ferrous Lineup shows up with its explanation.

## A first type: `Note`

Most of `lineup-core` is small types with functions attached. The
smallest, `Note` in `pitch.rs`, shows the pieces every later chapter
uses. A note is stored as its MIDI number, 0 to 127, middle C being 60:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub struct Note(u8);
```

`Note` is a **tuple struct**: a struct whose fields have no names, read
as `.0`, `.1` and so on. With one field that gives a number a type of its
own, it is a *newtype*, which [Pitch classes, notes and
spelling](../theory/pitch.md#a-note-a-midi-number-in-a-type-of-its-own)
explains with the rest of the note's music. The field has no `pub`, so
code outside `pitch.rs` cannot write `Note(200)`.

### Methods: `impl` blocks

A type's functions are written apart from its data, in an `impl` block:

```rust
impl Note {
    /// The lowest MIDI note.
    pub const MIN: Note = Note(0);
    /// The highest MIDI note.
    pub const MAX: Note = Note(127);
    /// Middle C.
    pub const C4: Note = Note(60);
    // ...

    /// Creates a note from a MIDI number, clamping it to 0..=127.
    pub fn new(midi: u8) -> Self {
        Note(midi.min(Self::MAX.0))
    }

    /// The MIDI note number.
    pub fn midi(self) -> u8 {
        self.0
    }
    // ...
}
```

- `pub const C4` is an **associated constant**, written `Note::C4`.
- `Self` is the type the `impl` is for, here `Note`.
- A function whose first parameter is `self` is a **method**, called as
  `note.midi()`. One without `self`, like `new`, is an **associated
  function**, called as `Note::new(60)`. Rust has no constructors;
  `new` is only the usual name.
- The last expression of a function, written without `;`, is what it
  returns.
- `midi` takes `self` by value, not `&self`: a `Note` is `Copy`, so
  passing one copies a single byte.

### Traits, and `derive`

A **trait** is a set of methods a type promises to have, like an
interface in other languages. The standard library's `fmt::Display`
says how a value is written by `{}` in `format!` and by `to_string()`:

```rust
impl fmt::Display for Note {
    /// Writes the note as a name and octave, such as `C#4`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class(), self.octave())
    }
}
```

So `Note::new(61).to_string()` is `"C#4"`, and every place that shows a
note writes it the same way.

Many traits are the same for every type: comparing field by field,
copying, printing for debugging. **`#[derive(...)]`** asks the compiler
to write those for you:

| Trait | Gives |
|-------|-------|
| `Debug` | `{:?}` printing, for debugging and test failures |
| `Clone`, `Copy` | a copy of the value; with `Copy`, assigning copies instead of moving |
| `PartialEq`, `Eq` | `==` and `!=` |
| `PartialOrd`, `Ord` | `<`, `>` and sorting |
| `Hash` | use as a key in a `HashMap` |

`Serialize` and `Deserialize` come from serde, not from the compiler
([Saving](saving.md#serde-describe-the-data-once)).
`Default` is the trait for "a sensible starting value"; `Tempo`
writes its own, 130 BPM:

```rust
impl Default for Tempo {
    fn default() -> Self {
        Tempo { bpm: 130.0 }
    }
}
```

### Numbers: `as`

Rust never changes a number's type by itself. `as` says so, as in
`octave`:

```rust
pub fn octave(self) -> i8 {
    (self.0 / 12) as i8 - 1
}
```

`self.0 / 12` is a `u8`, which cannot be negative, and the octave of
the lowest notes is -1, so the division's result becomes an `i8` before
1 is taken away. Indexes into a list must be `usize`, so `as usize`
shows up wherever a number picks an item.

### Panics

When a program reaches something that must never happen, such as an
index past the end of a list, Rust **panics**: it stops the thread and
prints where and why. `assert!`, `assert_eq!`, `unwrap` and `expect`
panic on purpose when their condition fails, which is how a test fails
([How the code is tested](../shipping/testing.md)). A panic means a
mistake in the program, never bad input: a damaged project file is an
error the app shows, and tests make sure no file can make the reader
panic ([Saving](saving.md#damaged-files-never-crash-the-app)).

## Rules the whole program keeps

A handful of decisions shape every part of the code. Each has a chapter
that shows it at work:

1. **The audio thread never waits.** The sound card asks for samples
   every few milliseconds, and a late answer is a click. The audio
   thread never takes a lock, never allocates memory and never makes a
   system call; it reads commands from a queue and leaves its messages
   in others. See [Two threads](threads.md).
2. **The window owns the project; the audio thread keeps its own copy**
   of just what it needs to make sound. The two never share data, so
   they never need a lock.
3. **One way to tell the audio thread.** Every edit, and every undo and
   redo, compares the project before and after and sends the
   differences. There is one piece of code to get right, and it is
   tested on its own. See [Telling the audio thread what
   changed](sync.md).
4. **Keep copies, not instructions.** Undo keeps whole copies of the
   project, not a list of how to reverse each edit. A copy cannot be
   reversed wrongly. See [Undo and redo](undo.md).
5. **Things that refer to each other by number change together.** A
   clip says "pattern 2"; remove pattern 1, and the clip must now say
   "pattern 1". The project owns all its lists and only lets them change
   through its methods. See [The project](project.md).
6. **A file is a contract with the future.** The file format has its
   own types, separate from the ones the program works with, and a
   version number that only moves when it must. See
   [Saving](saving.md).
7. **Drawing is separated from changing.** A window's drawing code
   returns a list of actions, and the app applies them in one place,
   through the same edit function. See [The
   editors](editors.md).
8. **Music theory as types.** Pitch classes, scales, chords and "what is
   playing" are small Rust types with methods, tested like any other
   code. See [Pitch classes, notes and
   spelling](../theory/pitch.md).
9. **Tools are plain functions.** Every editing tool takes notes and
   returns new ones, knowing nothing about windows or mice. See
   [Editing tools as plain functions](../theory/tools.md).
10. **Knowledge as data.** Styles, rhythms and songs are written as short
    strings in tables that a person can read and check, in Roman
    numerals and steps of a bar, so the same table plays in any key and
    meter. See [The library](../band/library.md).
11. **The band only writes patterns.** Trying a style or playing a
    chord sheet is an ordinary edit, so undo, saving and the audio
    thread work without knowing the band exists. See [How the band plays
    a chord sheet](../band/band.md).
12. **The same code, faster than real time.** Exporting a WAV file, a
    video or a trace runs the very same code the sound card uses, on a
    thread of its own, as fast as the computer can go. See
    [Exporting](../sound/export.md).

## Where each part lives

Every source file starts with a `//!` comment that says what it is for;
this map is made of their first lines. `lineup-core`:

| Module | What it holds |
|---|---|
| `pitch.rs`, `tempo.rs`, `meter.rs` | notes and their names, tempo, time signatures |
| `pattern.rs`, `project.rs`, `playlist.rs`, `arrangement.rs` | patterns, channels, tracks and clips, and the song as it plays |
| `sequencer.rs`, `swing.rs` | where playback is, step by step |
| `diff.rs`, `history.rs`, `file.rs` | what changed, undo, saving |
| `harmony/` | scales, chords, symbols, keys, sets, rows, micromodes, hearing harmony, voice leading |
| `tools.rs` | the piano roll's tools |
| `chord_sheet/`, `band/`, `bars.rs` | the chord sheet and the band that plays it |
| `library/` | styles, rhythms, songs and demos, written as data |
| `lead_sheet/` | the tune in notation |
| `midi.rs`, `midi_file.rs`, `record.rs` | MIDI messages, MIDI files, recording |
| `mixer.rs`, `effects.rs`, `score_order.rs` | the mixer's settings |
| `i18n/` | the Spanish for every text the app writes |

`lineup-audio`: `engine.rs` (the sound card and the audio thread),
`instrument.rs`, `drums.rs`, `timbre.rs`, `sample.rs` and
`soundfont.rs` (the instruments), `effects.rs` and `reverb.rs`,
`levels.rs`, `metronome.rs`, `render.rs` (playing without a sound card)
and `trace.rs`.

`lineup-app`: `app.rs` and the modules in `app/` (menus, files, editing,
export, MIDI, settings), one module per window (`channel_rack.rs`,
`piano_roll.rs`, `playlist.rs`, `chord_sheet.rs`, `harmony_keys.rs`,
`library_browser.rs`, `lead_sheet.rs`, `twelve_tone.rs`,
`micromodes.rs`, `style_editor.rs`, `ear_training.rs`), and the shared
pieces: `theme.rs`, `widgets.rs`, `icons.rs` and `i18n.rs`.
