# Rust glossary

Short definitions of the Rust terms used in this book, with the chapter
where each one first appears in real code.

| Term | Meaning | First seen in |
|------|---------|---------------|
| **Anonymous lifetime (`'_`)** | A lifetime written without a name, meaning "the lifetime of the borrowed argument". | [Steps to notes](step-3/notes.md) |
| **`Arc<T>`** | A shared, reference-counted pointer that is safe to use across threads; the data is freed when the last `Arc` goes. | [Instruments](step-2/instruments.md) |
| **Array** | A fixed number of values stored inline, like `[bool; 64]`; the length is part of the type. | [Patterns](step-2/project.md) |
| **`array::from_fn`** | Builds an array by calling a closure once for each element. | [Voices](step-3/voices.md) |
| **`as` cast** | Converts between number types, or a fieldless enum to its number: `PitchClass::D as u8` is 2. | [Pitch classes and scales](step-3/theory.md) |
| **Associated function** | A function in an `impl` block without `self`, called as `Type::name()`. `Note::new` is one. | [loom-core](step-1/loom-core.md) |
| **Associated type** | A type that each implementation of a trait chooses, like `type Error = FileError;` in an `impl TryFrom`. | [Saving](step-4/saving.md) |
| **Atomic** | A value (like `AtomicUsize`) that threads can read and write without a lock. | [Audio thread](step-2/realtime.md) |
| **Bit operations** | `&` (and), `\|` (or), `!` (not), `<<` and `>>` (shift) work on the bits of an integer. | [Pitch classes and scales](step-3/theory.md) |
| **Borrow** | Using a value through a reference (`&T` shared, `&mut T` mutable) without taking ownership. | [loom-audio](step-1/audio-engine.md) |
| **Borrow checker** | The part of the compiler that enforces: one `&mut` *or* any number of `&` at a time, and no reference outliving its value. | [loom-audio](step-1/audio-engine.md) |
| **`Borrow` trait** | A promise that an owned type (like `PathBuf`) hashes and compares exactly like the type it holds (`Path`), so a `HashMap<PathBuf, _>` can be searched with a `&Path`. | [What changed](step-5/diff.md) |
| **`Box<T>`** | An owned pointer to a value on the heap. | [loom-app](step-1/window.md) |
| **Capacity** | How many items a `Vec` can hold before it must allocate more memory. | [Audio thread](step-2/realtime.md) |
| **Cargo** | Rust's build tool and package manager. | [Workspace](step-1/workspace.md) |
| **`cfg` / `cfg_attr`** | Conditional compilation: `#[cfg(target_os = "windows")]` keeps the next item only when the condition is true, and `cfg_attr` applies an attribute only then. | [Linux, macOS and Windows](step-4/platforms.md) |
| **`chunk_by`** | Splits a slice into runs of neighbours that belong together, without copying. | [Piano roll window](step-3/piano-roll-ui.md) |
| **Closure** | An anonymous function, `\|args\| body`, that can capture variables from its surroundings. | [loom-audio](step-1/audio-engine.md) |
| **`const fn`** | A function simple enough to run while compiling, so it can be used to set a `const`. | [Saving](step-4/saving.md) |
| **Crate** | One library or program; the unit Cargo builds and publishes. | [Workspace](step-1/workspace.md) |
| **`Default`** | A trait for "the default value" of a type; on an enum, `#[derive(Default)]` needs `#[default]` on one variant. | [Pitch classes and scales](step-3/theory.md) |
| **Deref coercion** | Rust turning `&Vec<T>` into `&[T]` (or `&String` into `&str`) automatically where needed. | [Steps to notes](step-3/notes.md) |
| **Dereference (`*`)** | Follow a reference to the value it points at, to read or write it. | [Patterns](step-2/project.md) |
| **Derive** | `#[derive(Trait)]` asks the compiler to implement common traits automatically. | [loom-core](step-1/loom-core.md) |
| **Derive macro** | Code that writes a trait implementation for a type; serde's `#[derive(Serialize)]` comes from a crate, not from the compiler. | [Saving](step-4/saving.md) |
| **Disjoint borrows** | Borrowing two different fields of a struct at once, even one mutably; allowed because they cannot overlap. | [Voices](step-3/voices.md) |
| **`dyn Trait`** | A trait object: some type implementing `Trait`, chosen at run time. | [loom-app](step-1/window.md) |
| **Edition** | A version of the language (2015, 2018, 2021, 2024) chosen per crate. | [Workspace](step-1/workspace.md) |
| **Enum** | A type whose value is one of several variants, each of which can hold data. | [loom-audio](step-1/audio-engine.md) |
| **Feature** | An optional part of a crate, turned on or off in `Cargo.toml`; features can only add, never take away. | [Saving](step-4/saving.md) |
| **`fn` pointer** | The type of a plain function, like `fn(usize) -> Change`. Enum variants that hold data are such functions too: `Change::RemovePattern` builds a `Change` from a `usize`. | [What changed](step-5/diff.md) |
| **`Fn` / `FnMut` / `FnOnce`** | The traits closures implement: only reads, mutates its captures, or can be called once. | [Instruments](step-2/instruments.md) |
| **`From` / `Into`** | Conversion traits: implementing `From<A> for B` also gives `a.into()`. The `?` operator uses `From` to convert errors. | [Many patterns](step-4/patterns.md) |
| **`FromIterator`** | The trait behind `collect()`; implement it to collect into your own type. | [Pitch classes and scales](step-3/theory.md) |
| **Generic** | Code written for a placeholder type `T`, filled in at compile time. | [loom-audio](step-1/audio-engine.md) |
| **`HashMap<K, V>`** | A map from keys to values that finds a key in about the same time however many entries it holds. | [What changed](step-5/diff.md) |
| **`if let` / `while let`** | Run a block if (or while) a value matches one pattern. | [loom-app](step-1/window.md) |
| **`impl Trait`** | As an argument type: any type with this trait. As a return type: one type with this trait, left unnamed. | [Pitch classes and scales](step-3/theory.md) |
| **Inner attribute (`#![...]`)** | An attribute that applies to the thing it is written inside, such as the whole crate at the top of `main.rs`. | [Linux, macOS and Windows](step-4/platforms.md) |
| **Invariant** | A rule that is always true about a value, usually protected by private fields. | [Patterns](step-2/project.md) |
| **`is_some_and`** | `true` if an `Option` is `Some` and its value passes a test, `false` for `None`. | [Many patterns](step-4/patterns.md) |
| **Iterator** | A lazy sequence of values, transformed with methods like `map`, `filter`, `count`. | [Tests](step-1/ci.md) |
| **`Iterator::eq`** | Compares two iterators item by item, stopping at the first difference, without collecting them into lists. | [What changed](step-5/diff.md) |
| **`JoinHandle<T>`** | What `thread::spawn` returns. `join()` waits for the thread to end and gives back what its closure returned, or an `Err` if it panicked. | [WAV export](step-5/export.md) |
| **Lazy** | Iterators do no work until something pulls items out of them. | [Steps to notes](step-3/notes.md) |
| **Leading underscore (`_name`)** | Tells the compiler that a variable or field is unused on purpose, so it does not warn about it. | [WAV export](step-5/export.md) |
| **`let ... else`** | Destructure a pattern, or run an `else` block that must leave the function. | [Instruments](step-2/instruments.md) |
| **Let chain** | `if cond && let Some(x) = value { ... }`: conditions and pattern matches in one `if` (Rust 1.88). | [Audio thread](step-2/realtime.md) |
| **`match`** | Compare a value against patterns; must cover every case. | [loom-audio](step-1/audio-engine.md) |
| **Match guard** | `pattern if condition =>`: the arm matches only if the condition is also true. | [Chords](step-3/chords.md) |
| **`matches!`** | A macro that is `true` if a value matches a pattern. | [Pitch classes and scales](step-3/theory.md) |
| **`mem::replace`** | Put a new value into a place and get the old one back. | [Audio thread](step-2/realtime.md) |
| **`mem::swap`** | Exchanges the values behind two `&mut` references, without copying either. | [Undo and redo](step-5/history.md) |
| **`mem::take`** | Take a value out of a place, leaving the type's default (such as an empty `Vec`) behind. | [Song playback](step-4/song-playback.md) |
| **Method** | A function in an `impl` block that takes `self`, `&self` or `&mut self`. | [loom-core](step-1/loom-core.md) |
| **Module** | A namespace inside a crate, usually one file, declared with `mod`. | [Workspace](step-1/workspace.md) |
| **Monomorphization** | The compiler making a specialized copy of generic code for each type used. | [loom-audio](step-1/audio-engine.md) |
| **Move** | Transferring ownership of a value; the old name becomes unusable. | [loom-audio](step-1/audio-engine.md) |
| **`move` closure** | A closure that takes ownership of everything it uses instead of borrowing it; needed for a closure given to another thread. | [WAV export](step-5/export.md) |
| **Newtype** | A struct with one field that wraps another type to give it its own meaning. | [loom-core](step-1/loom-core.md) |
| **`?` operator** | Return early with the error if a `Result` is `Err` (or an `Option` is `None`). | [loom-audio](step-1/audio-engine.md) |
| **`Option<T>`** | `Some(value)` or `None`; Rust's replacement for null. | [loom-audio](step-1/audio-engine.md) |
| **`Option::take`** | Moves the value out of an `Option` and leaves `None` in its place; handy to take ownership of something stored in a field. | [WAV export](step-5/export.md) |
| **Or-pattern** | `A \| B` in a pattern matches either `A` or `B`. | [Pitch classes and scales](step-3/theory.md) |
| **Ownership** | Every value has exactly one owner, and is dropped when the owner goes away. | [loom-audio](step-1/audio-engine.md) |
| **Panic** | An unrecoverable error that stops the current thread (`panic!`, failed `assert!`). | [loom-core](step-1/loom-core.md) |
| **Property-based test** | A test that checks a rule which must hold for any input ("following the changes gives the new project"), rather than one answer for one input. | [What changed](step-5/diff.md) |
| **`pub(crate)`** | Visible everywhere inside the current crate, but not to other crates. | [Patterns](step-2/project.md) |
| **`pub(super)`** | Visible to the parent module and everything inside it. | [Saving](step-4/saving.md) |
| **RAII / `Drop`** | Owning a value means owning its resource; dropping it releases the resource. | [loom-audio](step-1/audio-engine.md) |
| **Range** | `a..b`, the numbers from `a` up to but not including `b`; a value with methods like `contains`. | [Steps to notes](step-3/notes.md) |
| **`rem_euclid`** | A remainder that is never negative: `(-1_i32).rem_euclid(12)` is 11, while `-1 % 12` is -1. | [Pitch classes and scales](step-3/theory.md) |
| **`Result<T, E>`** | `Ok(value)` or `Err(error)`; Rust's way to return errors. | [loom-audio](step-1/audio-engine.md) |
| **`retain`** | Keeps only the items of a `Vec` for which a closure returns `true`, in one pass. | [Steps to notes](step-3/notes.md) |
| **RON** | Rust Object Notation, a text format that looks like Rust's own struct syntax; Ferrous Loom's project files use it. | [Saving](step-4/saving.md) |
| **`saturating_sub`** | Subtraction that stops at the smallest value (0 for `usize`) instead of overflowing. | [Voices](step-3/voices.md) |
| **`Send`** | A marker trait: values of this type can be moved to another thread. | [loom-audio](step-1/audio-engine.md) |
| **serde** | The usual crate for turning Rust values into text or bytes and back, through the `Serialize` and `Deserialize` traits. | [Saving](step-4/saving.md) |
| **Slice** | A borrowed view of part of an array or `Vec`: `&[T]` or `&mut [T]`. | [loom-audio](step-1/audio-engine.md) |
| **State machine** | Code that keeps a current state, often an enum, and moves between states as things happen. | [Voices](step-3/voices.md) |
| **`'static`** | A lifetime meaning "lives as long as needed"; for closures, "borrows nothing temporary". | [loom-audio](step-1/audio-engine.md) |
| **Struct update syntax** | `NoteEvent { length: 2, ..note }`: a new struct with some fields changed and the rest taken from `note`. | [Steps to notes](step-3/notes.md) |
| **Target** | The kind of system a program is compiled for, like `x86_64-pc-windows-gnu`; `rustup target add` installs Rust's library for one. | [Linux, macOS and Windows](step-4/platforms.md) |
| **Trait** | A set of methods a type promises to provide, like an interface. | [loom-core](step-1/loom-core.md) |
| **Trait bound** | A requirement on a generic type, such as `T: FromSample<f32>`. | [loom-audio](step-1/audio-engine.md) |
| **`TryFrom` / `TryInto`** | Conversions that can fail, returning a `Result`. | [Saving](step-4/saving.md) |
| **Tuple** | A fixed group of values, possibly of different types, like `(usize, Note)`; it can be matched and taken apart. | [Pitch classes and scales](step-3/theory.md) |
| **Tuple struct** | A struct whose fields have no names, like `struct Note(u8)`; the fields are read as `.0`, `.1`... | [loom-core](step-1/loom-core.md) |
| **Type alias** | A second name for an existing type: `type StepRow = [bool; 64];`. | [Patterns](step-2/project.md) |
| **`VecDeque<T>`** | A double-ended queue: a ring buffer that is cheap to add to and take from at both ends. | [Undo and redo](step-5/history.md) |
| **Workspace** | A group of crates built together with one `Cargo.lock`. | [Workspace](step-1/workspace.md) |

## Where to learn more

- *The Rust Programming Language* ("the book"): installed with Rust, open
  it offline with `rustup doc --book`.
- *Rust by Example*: `rustup doc --rust-by-example`.
- The standard library docs: `rustup doc --std`.
