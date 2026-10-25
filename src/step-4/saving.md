# Saving and loading projects

Files: [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)
and [`crates/lineup-app/src/app/files.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/files.rs)

## What a project file looks like

A `.lineup` file is plain text. To see one, start the app, choose a demo
under File > Demos, then use File > Save As… and open the saved file in a
text editor:

```sh
cargo run --release
# File > Demos > The starting song, then File > Save As… song.lineup
head -n 30 song.lineup
```

A new project (File > New) has no notes yet, so its file is short. This is
the real start of the file that `Project::template()` writes:

```text
(
    format: 1,
    tempo: 130.0,
    scale: (
        root: C,
        kind: Major,
    ),
    channels: [
        (
            name: "Kick",
            kind: Sampler(Builtin(Kick)),
            volume: 0.8,
            muted: false,
        ),
        (
            name: "Clap",
            kind: Sampler(Builtin(Clap)),
            volume: 0.8,
            muted: false,
        ),
        ...
        (
            name: "Lineup Osc",
            kind: Synth,
            volume: 0.6,
            muted: false,
        ),
    ],
    patterns: [
        (
            name: "Pattern 1",
            length: 16,
            notes: [
                [],
                ...
```

The starting song is a little different: its synth channel plays a
General MIDI piano, so it carries `program: Some(0)` and its file says
`format: 9`. Its chords read as notes, one per line:

```text
                    (pitch: 48, start: 0, length: 8, velocity: 100),
                    (pitch: 60, start: 0, length: 8, velocity: 100),
                    (pitch: 64, start: 0, length: 8, velocity: 100),
                    (pitch: 67, start: 0, length: 8, velocity: 100),
```

The format is **RON**, "Rusty Object Notation". It looks like Rust: a
struct is `( field: value, ... )`, a list is `[ ... ]`, and enum variants
are written by name, `Sampler(Builtin(Kick))`, just as in the code. JSON
would work too, but JSON has no enums, so the same channel would read
`{"Sampler": {"Builtin": "Kick"}}`. RON is easier to read and to fix by
hand, which matters for a learning project: you can open a song in a text
editor and see your own data structures.

## serde: describe the data once

Rust's standard way to turn values into text (or bytes) and back is the
**serde** crate, short for *ser*ialize and *de*serialize. serde does not
know any format itself. It defines two traits, `Serialize` and
`Deserialize`, and format crates such as `ron` or `serde_json` use them.

You almost never write those traits by hand. A **derive macro** writes
them for you:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub kind: ChannelKind,
    pub volume: f32,
    pub muted: bool,
}
```

The macro reads the struct definition at compile time and generates code
that writes each field by name, and reads them back. For this to work,
every field's type must be `Serialize` too, so the derive was added to
`ChannelKind`, `SampleSource`, `Drum`, `NoteEvent`, `Scale`, `ScaleKind`,
`PitchClass`, `Track` and `Clip`. `String`, numbers, `bool`, `Vec` and
`PathBuf` come with serde.

The derive is turned on by asking for serde's `derive` feature in
`Cargo.toml`. **Features** are optional parts of a crate that you switch
on when you need them:

```toml
serde = { version = "1", features = ["derive"] }
```

### Keeping the rules while reading: `from` and `into`

Since step 1, a `Tempo` has been impossible to create outside 10 to 522
BPM, because its field is private and `Tempo::new` clamps it. A derived
`Deserialize` would set the field directly from the file, so a file
saying `tempo: 9000.0` would create a tempo the rest of the program
promises can never exist.

The fix is to tell serde to go through a type we control:

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(from = "f32", into = "f32")]
pub struct Tempo {
    bpm: f32,
}

impl From<f32> for Tempo {
    fn from(bpm: f32) -> Self {
        Tempo::new(bpm)
    }
}

impl From<Tempo> for f32 {
    fn from(tempo: Tempo) -> Self {
        tempo.bpm
    }
}
```

`#[serde(from = "f32")]` means "read an `f32`, then convert it with
`From<f32>`". That conversion is `Tempo::new`, which clamps. The test
`values_out_of_range_are_clamped` edits a saved file to say 9000 BPM and
checks that it loads as 522. `into = "f32"` does the reverse when saving,
which also makes the file say `tempo: 130.0` instead of
`tempo: (bpm: 130.0)`. `Note` does the same with its MIDI number.

## A separate type for the file

`Project` itself does not derive serde. Its fields are private because
they must agree with each other (one row per channel, clips that point at
real patterns), and a derived `Deserialize` would fill them straight from
the file, agreeing or not. So the file has its own struct:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct ProjectFile {
    format: u32,
    #[serde(default, skip_serializing_if = "crate::Mixer::is_default")]
    mixer: crate::Mixer,
    tempo: Tempo,
    scale: Scale,
    #[serde(default, skip_serializing_if = "TimeSignature::is_common")]
    time_signature: TimeSignature,
    channels: Vec<Channel>,
    patterns: Vec<PatternFile>,
    tracks: Vec<Track>,
    clips: Vec<Clip>,
    // ...sections, a loop region, time, tempo, key and chord markers,
    // the band, and where each part came from. Later chapters add them.
}

#[derive(Debug, Serialize, Deserialize)]
struct PatternFile {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time: Option<TimeSignature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    player: Option<Player>,
    length: usize,
    /// One list of notes per channel, in the order of `channels`.
    notes: Vec<Vec<NoteEvent>>,
}
```

The `#[serde(default, skip_serializing_if = ...)]` lines are the file
format's quiet way of growing: a field that is at its default is not
written at all, and a field missing from an older file is filled in with
the type's default. That is why a project with nothing special in it is
still written as the same short format-1 text as before.

A `ProjectFile` accepts anything shaped right. Turning it into a
`Project` is where the checking happens, and it is written as a
**`TryFrom`** implementation, the standard trait for "a conversion that
can fail":

```rust
impl TryFrom<ProjectFile> for Project {
    type Error = FileError;

    fn try_from(file: ProjectFile) -> Result<Self, Self::Error> {
        let mut project = Project::empty();
        project.mixer = file.mixer;
        project.tempo = file.tempo;
        project.scale = file.scale;
        // ...checks on the mixer, the time signature and the channel mix...
        for channel in file.channels {
            project.add_channel(channel);
        }
        for (index, saved) in file.patterns.into_iter().enumerate() {
            if index > 0 {
                project.add_pattern();
            }
            let pattern = project.pattern_mut(index).unwrap();
            pattern.set_length(saved.length);
            for (channel, notes) in saved.notes.into_iter().enumerate() {
                for note in notes {
                    if pattern.add_note(channel, note).is_none() {
                        return invalid(format!(/* where, and what is wrong */));
                    }
                }
            }
        }
        // ...tracks, clips, sections, markers and the band...
        Ok(project)
    }
}
```

The project is rebuilt **through its front door**: `add_channel`,
`add_pattern`, `add_note`, `add_clip`, the same methods the app calls
when you click. Those methods already keep every rule, so a file can never
create a project that the UI could not have created. When a method
refuses (a note that does not fit, a clip that plays a pattern that is
not there), the error says where, so a person editing the file by hand
can fix it.

`type Error = FileError;` is an **associated type**: part of the trait
that each implementation chooses. With `TryFrom` in place, the standard
library also provides the matching `try_into`, so
`let project: Project = file.try_into()?;` would work too.

Saving goes the other way with `From<&Project> for ProjectFile`, which
copies the data out with `to_vec()` and `map`. It cannot fail, so it is a
`From`, not a `TryFrom`.

## The version number

```rust
pub const FORMAT_VERSION: u32 = 27;
```

`FORMAT_VERSION` is the newest layout this program can read. A file does
not always use it: `format_for` writes the **lowest** format that can hold
the project. A project that uses nothing new stays format 1, so an older
Ferrous Lineup can still open it. The starting song needs format 9,
because its synth channel plays a General MIDI program; a song with a
chord sheet needs at least format 12; and so on. The current list goes up to 27,
for a drum sent over MIDI. Each entry in the long comment above
`FORMAT_VERSION` says what that version added and what an older program
would get wrong without it.

Refusing a newer file is harder than it sounds. The obvious way is to
read a tiny struct first:

```rust
#[derive(Deserialize)]
struct Header {
    format: u32,
}
```

By default, serde skips fields that a struct does not list, so a `Header`
can be read from the whole file. But to skip the rest of the file RON has
to look ahead through all of it. On a long song such as Boléro, that took
85 seconds, even in a release build. The whole file now opens in a
fraction of a second, because `format_number` reads only the first few
bytes:

```rust
pub fn from_text(text: &str) -> Result<Project, FileError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let file: ProjectFile = match ron::from_str(text) {
        Ok(file) => file,
        Err(err) => {
            return Err(match format_number(text) {
                Some(format) if format > FORMAT_VERSION => FileError::NewerFormat(format),
                _ => err.into(),
            });
        }
    };
    if file.format > FORMAT_VERSION {
        return Err(FileError::NewerFormat(file.format));
    }
    Project::try_from(file)
}

/// Reads `(format: 25, ...` without the RON reader.
fn format_number(text: &str) -> Option<u32> {
    let rest = text.trim_start().strip_prefix('(')?.trim_start();
    let rest = rest.strip_prefix("format")?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let digits = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..digits].parse().ok()
}
```

`to_text` always writes `format` first, so this looks only where the
number must be. If the file is so broken that even the number cannot be
read, the original RON error is the one reported. And if the file parses
but still says a newer format, the number is checked once more. A
byte-order mark at the start (some text editors add one) is stripped
first, so it cannot hide the `(`.

That `#[serde(default)]` from `ProjectFile` is also how a future version
can add a field without breaking old files: files that do not have it get
the type's default value. The test `files_from_the_future_are_refused`
reads `(format: 28, sparkles: true)` and expects `NewerFormat(28)`.

## Errors as an enum

```rust
#[derive(Debug)]
pub enum FileError {
    Io(io::Error),
    Parse(ron::error::SpannedError),
    Write(ron::Error),
    NewerFormat(u32),
    Invalid(String),
}
```

The same pattern as `EngineError` in step 1 and `SampleError` in step 2:
one variant per kind of problem, a `Display` implementation with a
message for people, and `std::error::Error` so it works with any Rust
error handling. Two things are new here.

**`From` for `?`.** Inside a function that returns
`Result<_, FileError>`, the `?` operator can only pass an error on if it
knows how to turn it into a `FileError`. These implementations teach it:

```rust
impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::Io(err)
    }
}
```

With one for `io::Error`, one for `ron`'s parse error and one for its
write error, a function can mix calls from both libraries and use `?`
on all of them:

```rust
pub fn load(path: &Path) -> Result<Project, FileError> {
    let text = fs::read_to_string(path)?; // io::Error -> FileError::Io
    from_text(&text)
}
```

**`source`.** The `Error` trait has an optional method that returns the
lower-level error a value wraps. Error-reporting tools follow it to print
a chain like "could not load: no such file". `FileError::Io` returns its
`io::Error`; `Invalid` has nothing underneath and returns `None`.

```rust
fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
        FileError::Io(err) => Some(err),
        // ...
        FileError::NewerFormat(_) | FileError::Invalid(_) => None,
    }
}
```

`&(dyn Error + 'static)` is a **trait object**: a reference to *some*
value that implements `Error`, without saying which type. That lets one
method return an `io::Error` in one case and a `ron` error in another.

## Saving without losing the old file

```rust
pub fn save(project: &Project, path: &Path) -> Result<(), FileError> {
    let text = to_text(project)?;
    let temporary = path.with_extension(format!("{EXTENSION}.tmp"));
    if let Err(err) = fs::write(&temporary, text) {
        let _ = fs::remove_file(&temporary);
        return Err(err.into());
    }
    if let Err(err) = fs::rename(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(err.into());
    }
    Ok(())
}
```

Writing straight over `song.lineup` has a nasty failure: if the program
crashes or the disk fills up halfway, the file is half old, half new, and
neither version can be loaded. Writing to `song.lineup.tmp` first and then
renaming it over the original avoids that, because the operating system
replaces a file by renaming in a single step. Either the rename happened
and the new file is complete, or it did not and the old file is
untouched. On Windows, Rust's `fs::rename` also replaces an existing file,
so the same code works everywhere.

The tests save to `std::env::temp_dir()`, the system's folder for
temporary files (`/tmp` on Linux, something under your user folder on
Windows), with the process id in the name so two test runs at once do not
collide, and delete the file afterwards.

## In the app: File menu, shortcuts and the title bar

The app side is in `crates/lineup-app/src/app/files.rs`. Its first lines
explain an unusual layout:

```rust
//! This is a child module of `app` (its file is `app/files.rs`), so it can
//! use the private fields of [`LineupApp`]: in Rust, private means "visible
//! in this module and the modules inside it".
```

`app.rs` declares `mod files;`, and Rust looks for that module in
`app/files.rs`. The file holds another `impl LineupApp` block. A type can
have any number of `impl` blocks, in any modules of its crate, so a big
type can be split by topic. Being a child of `app`, `files` sees the
private fields `self.project`, `self.audio` and so on, which a sibling
module like `playlist` could not.

### Choosing a file

File dialogs come from the **rfd** crate ("Rusty File Dialog"), which
shows the system's own dialog: GTK or KDE on Linux (through the desktop
portal, or the `zenity` program), the Finder's panel on macOS, the
Explorer dialog on Windows.

```rust
fn project_dialog() -> rfd::FileDialog {
    let extensions = [file::EXTENSION, file::OLD_EXTENSION];
    rfd::FileDialog::new().add_filter("Ferrous Lineup project", &extensions)
}

if let Some(path) = project_dialog().pick_file() {
    self.open_now(path);
}
```

`pick_file` waits until the user picks a file or cancels, and returns an
`Option<PathBuf>`. The window does not redraw while the dialog is open,
but the music keeps playing, because it runs on the audio thread. On a
Linux system with neither a portal nor `zenity`, `pick_file` returns
`None` straight away; the dialog simply does not appear.

### "Save changes?"

How does the app know there are unsaved changes? It keeps a copy of the
project as it was last opened or saved, and compares. But `Project`
derives `PartialEq`, so `!=` walks every channel, note and clip. That is
cheap once, but a drag edits the project on every frame, and comparing the
whole song every frame is wasted work. `FileState` therefore caches the
answer:

```rust
/// Whether the project differs from `saved`, as last worked out.
dirty: bool,
/// Set when the project may differ from `saved`.
check_dirty: bool,
```

An edit sets `check_dirty`, but only if the project was not already dirty.
While it is dirty, more edits do not ask for the whole comparison again;
undo, redo and saving will ask when they need it:

```rust
pub(super) fn project_edited(&mut self) {
    if !self.dirty {
        self.check_dirty = true;
    }
}
```

Undo and redo, on the other hand, can make the project match the saved
copy again, so they always ask for a new comparison
(`project_reverted`). The cached answer is read every frame for the
title, but the comparison itself runs only when an edit or undo set the
flag:

```rust
pub(super) fn has_unsaved_changes(&mut self) -> bool {
    if self.file.check_dirty {
        self.file.dirty = match &self.file.saved {
            Some(saved) => self.project != *saved,
            None => true, // recovered work has never been saved
        };
        self.file.check_dirty = false;
    }
    self.file.dirty
}
```

This gets something right that a plain dirty flag would not: undo sets
`check_dirty` even when the project is already dirty, so undoing back to
the saved copy makes the comparison find them equal again and clears the
star. The price is that ordinary edits do not rewalk the project once it
is dirty; changing a step back by hand may leave the star until the next
undo, save or reopen. That is a fair trade for not comparing a whole song
on every frame of a drag.

Anything that would throw the project away waits for an answer first:

```rust
pub(super) enum Pending {
    New,
    Open,
    OpenPath(PathBuf),
    Quit,
}

fn request(&mut self, ctx: &egui::Context, action: Pending) {
    if self.has_unsaved_changes() {
        self.file.pending = Some(action);
    } else {
        self.perform(ctx, action);
    }
}
```

While `pending` holds something, `unsaved_changes_dialog` shows an
`egui::Modal`, a window that dims everything behind it and takes all the
clicks:

![Asking before unsaved changes are lost](../images/step-4-save-changes.png)

Save saves and then carries out the waiting action, unless saving failed
or the user cancelled the Save As dialog; Don't save carries it out
straight away; Cancel, Escape or a click outside forgets it. The
`pending` value is the whole state machine: `None` means "nothing
waiting", `Some(Pending::Open)` means "the user wants to open a file,
once they decide about the changes".

`pub(super)` makes the enum visible to the parent module (`app`), which
needs it for `FileState`, but no further.

### Closing the window

Clicking the window's × asks the operating system to close it. egui
reports the request, and the app can say no:

```rust
let close_requested = ctx.input(|input| input.viewport().close_requested());
if close_requested && !self.file.quitting && self.has_unsaved_changes() {
    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
    self.file.pending = Some(Pending::Quit);
}
```

When the user then picks Save or Don't save, `perform(Pending::Quit)`
sets `quitting` and sends `ViewportCommand::Close`, which is let through
this time.

### Shortcuts

```rust
const SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
const SAVE_AS: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::S);

let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
if pressed(SAVE_AS) {
    self.save_as();
} else if pressed(SAVE) {
    self.save();
}
```

- `Modifiers::COMMAND` is Ctrl on Linux and Windows and ⌘ on macOS, so
  each system gets the shortcut its users expect. The File menu shows
  the right one, too: `ctx.format_shortcut(&SAVE)` gives "Ctrl+S" or
  "⌘S".
- `consume_shortcut` checks the keys and removes them from the input, so
  a text field does not also see them.
- Ctrl+S would also match Ctrl+Shift+S (the extra Shift is ignored), so
  the longer shortcut is checked first.

These `const` values call `KeyboardShortcut::new` while compiling. That
is allowed because egui marked it as a `const fn`, a function simple
enough to run at compile time.

The window title shows the file's name and a star for unsaved changes
("song.lineup* · Ferrous Lineup"). The app sends a
`ViewportCommand::Title` only when the text changes, not on every frame.
(Earlier versions used a dot before the name; a demo now shows its own
name and says it is read-only: see
[A demo keeps its name](../step-8/demo-names.md).)

### Opening a file from outside

```rust
let path = std::env::args_os().nth(1).map(PathBuf::from);
```

`ferrous-lineup song.lineup` starts with the song open, which is also what a
file manager does when you double-click a file associated with the app.
`args_os` returns the arguments as `OsString`s, exactly as the system
gave them. `std::env::args` would panic on a file name that is not valid
UTF-8, which Linux allows. `nth(1)` skips the first argument, which is
the program's own name.

A `.lineup` file dropped on the window is opened too, after the same
"Save changes?" question. Dropped WAV files still go to the channel rack.
