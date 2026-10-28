# Rust glossary

Short definitions of the Rust terms used in this book, with the chapter
that explains each one.

| Term | Meaning | Explained in |
|------|---------|--------------|
| **Anonymous lifetime (`'_`)** | A lifetime written without a name, meaning "the lifetime of the borrowed argument". | [Steps to notes](step-3/notes.md) |
| **`Arc<T>`** | A shared, reference-counted pointer that is safe to use across threads; the data is freed when the last `Arc` goes. | [Instruments](step-2/instruments.md) |
| **Array** | A fixed number of values stored inline, like `[bool; 64]`; the length is part of the type. | [Patterns](step-2/project.md) |
| **Array `map`** | `[T; N]::map` calls a closure on each item of an array and gives back a new array of the same length, with no `Vec` and no `collect`. | [Twelve-tone rows](step-8/twelve-tone.md#every-form-from-one-function) |
| **`array::from_fn`** | Builds an array by calling a closure once for each element. | [Voices](step-3/voices.md) |
| **`as` cast** | Converts between number types, or a fieldless enum to its number: `PitchClass::D as u8` is 2. | [lineup-core](step-1/lineup-core.md) |
| **Associated function** | A function in an `impl` block without `self`, called as `Type::name()`. `Note::new` is one. | [lineup-core](step-1/lineup-core.md) |
| **Associated type** | A type that each implementation of a trait chooses, like `type Error = FileError;` in an `impl TryFrom`. | [Saving](step-4/saving.md) |
| **Atomic** | A value (like `AtomicUsize`) that threads can read and write without a lock. | [Audio thread](step-2/realtime.md) |
| **`AtomicU64` for an `f64`** | There is no atomic float, but `f64::to_bits` turns a float into the `u64` with the same bits, and `f64::from_bits` turns it back, so an `AtomicU64` can share one. | [Live keys](step-7/live.md) |
| **Backtracking** | Building an answer a piece at a time, and taking the last piece back when it leads nowhere; usually a function that calls itself. | [Twelve-tone rows](step-8/twelve-tone.md#what-makes-a-row-interesting) |
| **Big-endian / `to_be_bytes`** | Writing a number's most significant byte first; `6u32.to_be_bytes()` is `[0, 0, 0, 6]`. MIDI files use it. | [MIDI messages](step-7/messages.md) |
| **Bit operations** | `&` (and), `\|` (or), `!` (not), `<<` and `>>` (shift) work on the bits of an integer. | [Pitch classes and scales](step-3/theory.md) |
| **`bool::then` / `then_some`** | Turn a condition into an `Option`: `cond.then_some(x)` is `Some(x)` if `cond` is true and `None` if not; `cond.then(\|\| ...)` only runs the closure when it is true. | [Tools](step-6/tools.md) |
| **Borrow** | Using a value through a reference (`&T` shared, `&mut T` mutable) without taking ownership. | [lineup-audio](step-1/audio-engine.md) |
| **Borrow checker** | The part of the compiler that enforces: one `&mut` *or* any number of `&` at a time, and no reference outliving its value. | [lineup-audio](step-1/audio-engine.md) |
| **`Borrow` trait** | A promise that an owned type (like `PathBuf`) hashes and compares exactly like the type it holds (`Path`), so a `HashMap<PathBuf, _>` can be searched with a `&Path`. | [What changed](step-5/diff.md) |
| **`Box<T>`** | An owned pointer to a value on the heap. | [lineup-app](step-1/window.md) |
| **Byte string (`b"..."`)** | A string literal that is a `&[u8; N]` of ASCII codes instead of a `&str`, like `b"MThd"`. | [MIDI messages](step-7/messages.md) |
| **Capacity** | How many items a `Vec` can hold before it must allocate more memory. | [Audio thread](step-2/realtime.md) |
| **Cargo** | Rust's build tool and package manager. | [Workspace](step-1/workspace.md) |
| **`cfg` / `cfg_attr`** | Conditional compilation: `#[cfg(target_os = "windows")]` keeps the next item only when the condition is true, and `cfg_attr` applies an attribute only then. | [Linux, macOS and Windows](step-4/platforms.md) |
| **`#[cfg(test)]` on an item** | Keeps one function (or `use`, or struct) only in test builds, so a helper that only the tests call does not warn as unused in a normal build. | [Micromodes](step-8/micromodes.md#checking-a-table-with-tests) |
| **`checked_add_signed`** | Adds a signed number to an unsigned one, like `usize + isize`, and gives `None` instead of going below zero or overflowing. | [Tools](step-6/tools.md) |
| **`chunk_by`** | Splits a slice into runs of neighbours that belong together, without copying. | [Piano roll window](step-3/piano-roll-ui.md) |
| **`chunks` / `chunks_exact`** | Split a slice into pieces of n items; `chunks_exact` leaves out a last piece that is too short. | [Twelve-tone rows](step-8/twelve-tone.md#what-makes-a-row-interesting) |
| **`chars` / `Chars::as_str`** | `text.chars()` walks a string one whole character at a time ("♭" is three bytes but one `char`); `as_str()` is the part not walked yet, borrowed from the same text. | [Chord names](step-9/chord-names.md#reading-a-note-and-keeping-the-rest) |
| **Closure** | An anonymous function, `\|args\| body`, that can capture variables from its surroundings. | [lineup-audio](step-1/audio-engine.md) |
| **Closure captures fields** | Since the 2021 edition, a closure that uses `self.midi_out` borrows only that field, not all of `self`, so other fields can be borrowed at the same time. | [Live keys](step-7/live.md) |
| **`cmp::Reverse`** | Wraps a value so that it compares the other way round: sorting by `Reverse(count)` puts the largest count first. | [Micromodes](step-8/micromodes.md#counting-groups-of-three) |
| **Collecting `Option`s** | Collecting an iterator of `Option<T>` into an `Option<Vec<T>>` gives `Some` of all the values, or `None` as soon as one item is `None`. `Result` works the same way. | [Tools](step-6/tools.md) |
| **`const fn`** | A function simple enough to run while compiling, so it can be used to set a `const`. | [Saving](step-4/saving.md) |
| **Constant promotion** | Writing `&[1, 2, 3]` of plain constant values gives a `&'static` borrow, because the compiler stores the values with the program. A function call inside the brackets, even a `const fn`, is not promoted. | [The library](step-8/library.md#macro_rules-and-why-not-a-function) |
| **`Copy`** | A marker trait for small values that are copied instead of moved, like numbers and `Note`; `let held = list[i];` then copies the item out. | [lineup-core](step-1/lineup-core.md) |
| **`count_ones`** | How many bits of an integer are 1: the size of a set stored as bits. | [Twelve-tone rows](step-8/twelve-tone.md#sets-in-bits) |
| **Crate** | One library or program; the unit Cargo builds and publishes. | [Workspace](step-1/workspace.md) |
| **`cycle`** | Repeats an iterator forever. Safe because iterators are lazy: something else, like `zip`, decides when to stop. | [Tools](step-6/tools.md) |
| **Declarative macro (`macro_rules!`)** | A macro defined by patterns: `macro_rules! hit { ($at:expr, ...) => { ... } }` matches what it is given and pastes it into a template, before the code is compiled. | [The library](step-8/library.md#macro_rules-and-why-not-a-function) |
| **`Default`** | A trait for "the default value" of a type; on an enum, `#[derive(Default)]` needs `#[default]` on one variant. | [lineup-core](step-1/lineup-core.md) |
| **Deref coercion** | Rust turning `&Vec<T>` into `&[T]` (or `&String` into `&str`) automatically where needed. | [Steps to notes](step-3/notes.md) |
| **Dereference (`*`)** | Follow a reference to the value it points at, to read or write it. | [Patterns](step-2/project.md) |
| **Derive** | `#[derive(Trait)]` asks the compiler to implement common traits automatically. | [lineup-core](step-1/lineup-core.md) |
| **Derive macro** | Code that writes a trait implementation for a type; serde's `#[derive(Serialize)]` comes from a crate, not from the compiler. | [Saving](step-4/saving.md) |
| **`Display`** | The trait behind `{}` in `format!` and behind `to_string()`. Implement its `fmt` method once, and every place writes the value the same way. | [lineup-core](step-1/lineup-core.md) |
| **Disjoint borrows** | Borrowing two different fields of a struct at once, even one mutably; allowed because they cannot overlap. | [Voices](step-3/voices.md) |
| **`div_ceil`** | Integer division that rounds up: `16.div_ceil(6)` is 3, where `16 / 6` is 2. | [Micromodes](step-8/micromodes.md#the-walk-choosing-with-a-tuple) |
| **`drain(..)`** | Takes every item out of a `Vec`, in order, and keeps the `Vec`'s memory for later. | [Live keys](step-7/live.md) |
| **`dyn Trait`** | A trait object: some type implementing `Trait`, chosen at run time. | [lineup-app](step-1/window.md) |
| **Edition** | A version of the language (2015, 2018, 2021, 2024) chosen per crate. | [Workspace](step-1/workspace.md) |
| **Enum** | A type whose value is one of several variants, each of which can hold data. | [lineup-audio](step-1/audio-engine.md) |
| **Enum size** | An enum takes as much room as its biggest variant; putting a big variant's data in a `Box` keeps every value small. Clippy's `large_enum_variant` lint points it out. | [Live keys](step-7/live.md) |
| **`env!` and `concat!`** | Macros that run while the program is compiled: `env!("CARGO_PKG_VERSION")` becomes the version Cargo read from `Cargo.toml`, and `concat!` joins literals into one `&'static str`, so both can build a `const`. | [Quicker to work in](step-9/workflow.md#who-made-it-and-which-version) |
| **`expect`** | Like `unwrap`, but panics with the message given. It states a promise ("this is always `Some`"), best kept by a test. | [Twelve-tone rows](step-8/twelve-tone.md#the-table-of-224-classes-and-how-to-trust-it) |
| **Feature** | An optional part of a crate, turned on or off in `Cargo.toml`; features can only add, never take away. | [Saving](step-4/saving.md) |
| **`find_map`** | Calls a closure on each item and returns the first `Some` it gives. | [Chords](step-3/chords.md) |
| **`flat_map`** | Maps each item to an iterator and joins all of them into one iterator. | [The piano roll, grown up](step-6/piano-roll.md) |
| **Float to integer `as`** | `x as usize` drops the fraction and *saturates*: negative numbers and NaN become 0, numbers too big become the type's maximum. It never panics. | [Recording](step-7/recording.md) |
| **`fn` pointer** | The type of a plain function, like `fn(usize) -> Change`. Enum variants that hold data are such functions too: `Change::RemovePattern` builds a `Change` from a `usize`. | [What changed](step-5/diff.md) |
| **`Fn` / `FnMut` / `FnOnce`** | The traits closures implement: only reads, mutates its captures, or can be called once. | [Instruments](step-2/instruments.md) |
| **`From` / `Into`** | Conversion traits: implementing `From<A> for B` also gives `a.into()`. The `?` operator uses `From` to convert errors. | [Many patterns](step-4/patterns.md) |
| **`FromIterator`** | The trait behind `collect()`; implement it to collect into your own type. | [Pitch classes and scales](step-3/theory.md) |
| **`FromStr` / `str::parse`** | The trait for "can be read from text": implement `from_str`, and `"Am7".parse::<ChordSymbol>()` works, as it does for numbers. | [Chord names](step-9/chord-names.md#fromstr-am7parse) |
| **Generic** | Code written for a placeholder type `T`, filled in at compile time. | [lineup-audio](step-1/audio-engine.md) |
| **Glob import** | `use super::*;` brings in every name the other module can see, including a parent's private items when used from a child. Two glob imports that bring the same name make it ambiguous. | [More demos](step-8/more-demos.md#use-super-and-what-children-can-see) |
| **`HashMap<K, V>`** | A map from keys to values that finds a key in about the same time however many entries it holds. | [What changed](step-5/diff.md) |
| **`if let` / `while let`** | Run a block if (or while) a value matches one pattern. | [lineup-app](step-1/window.md) |
| **`ilog2`** | The integer base-2 logarithm: how many times a number can be halved before reaching 1. `8u8.ilog2()` is 3. It panics on 0. | [Time signatures](step-8/time-signatures.md#in-midi-files) |
| **`impl Trait`** | As an argument type: any type with this trait. As a return type: one type with this trait, left unnamed. | [Pitch classes and scales](step-3/theory.md) |
| **Inner attribute (`#![...]`)** | An attribute that applies to the thing it is written inside, such as the whole crate at the top of `main.rs`. | [Linux, macOS and Windows](step-4/platforms.md) |
| **Invariant** | A rule that is always true about a value, usually protected by private fields. | [Patterns](step-2/project.md) |
| **`is_multiple_of`** | `a.is_multiple_of(b)` is the clear way to write `a % b == 0`, and does not panic when `b` is 0. | [Time signatures](step-8/time-signatures.md) |
| **`is_some_and`** | `true` if an `Option` is `Some` and its value passes a test, `false` for `None`. | [Many patterns](step-4/patterns.md) |
| **Iterator** | A lazy sequence of values, transformed with methods like `map`, `filter`, `count`. | [Tests](step-1/ci.md) |
| **`Iterator::eq`** | Compares two iterators item by item, stopping at the first difference, without collecting them into lists. | [What changed](step-5/diff.md) |
| **`JoinHandle<T>`** | What `thread::spawn` returns. `join()` waits for the thread to end and gives back what its closure returned, or an `Err` if it panicked. | [WAV export](step-5/export.md) |
| **Lazy** | Iterators do no work until something pulls items out of them. | [Steps to notes](step-3/notes.md) |
| **Leading underscore (`_name`)** | Tells the compiler that a variable or field is unused on purpose, so it does not warn about it. | [WAV export](step-5/export.md) |
| **`let ... else`** | Destructure a pattern, or run an `else` block that must leave the function. | [Instruments](step-2/instruments.md) |
| **Let chain** | `if cond && let Some(x) = value { ... }`: conditions and pattern matches in one `if` (Rust 1.88). | [Audio thread](step-2/realtime.md) |
| **Literal in a pattern** | `Event::Key { repeat: false, .. }`: a value written in a pattern only matches when the field equals it. | [Live keys](step-7/live.md) |
| **`match`** | Compare a value against patterns; must cover every case. | [lineup-audio](step-1/audio-engine.md) |
| **Match guard** | `pattern if condition =>`: the arm matches only if the condition is also true. | [Chords](step-3/chords.md) |
| **`matches!`** | A macro that is `true` if a value matches a pattern. | [Pitch classes and scales](step-3/theory.md) |
| **`mem::replace`** | Put a new value into a place and get the old one back. | [Audio thread](step-2/realtime.md) |
| **`mem::swap`** | Exchanges the values behind two `&mut` references, without copying either. | [Undo and redo](step-5/history.md) |
| **`mem::take`** | Take a value out of a place, leaving the type's default (such as an empty `Vec`) behind. | [Song playback](step-4/song-playback.md) |
| **Method** | A function in an `impl` block that takes `self`, `&self` or `&mut self`. | [lineup-core](step-1/lineup-core.md) |
| **`min_by_key`** | The item of an iterator with the smallest key, or `None` if there are no items. | [Twelve-tone rows](step-8/twelve-tone.md#sets-in-bits) |
| **Module** | A namespace inside a crate, usually one file, declared with `mod`. | [Workspace](step-1/workspace.md) |
| **Monomorphization** | The compiler making a specialized copy of generic code for each type used. | [lineup-audio](step-1/audio-engine.md) |
| **Move** | Transferring ownership of a value; the old name becomes unusable. | [lineup-audio](step-1/audio-engine.md) |
| **`move` closure** | A closure that takes ownership of everything it uses instead of borrowing it; needed for a closure given to another thread. | [WAV export](step-5/export.md) |
| **`mpsc` channel** | A queue from the standard library with many senders (`Sender` can be cloned) and one receiver; `try_iter` takes what is waiting without waiting for more. | [Live keys](step-7/live.md) |
| **`Mutex<T>` and poisoning** | A lock that lets one thread at a time reach the `T` inside. If a thread panics while holding it, the mutex is *poisoned*, and `lock` returns an error until you take the data anyway with `PoisonError::into_inner`. | [Live keys](step-7/live.md) |
| **Newtype** | A struct with one field that wraps another type to give it its own meaning. | [lineup-core](step-1/lineup-core.md) |
| **`?` operator** | Return early with the error if a `Result` is `Err` (or an `Option` is `None`). | [lineup-audio](step-1/audio-engine.md) |
| **`Option::and_then`** | Calls a closure that itself returns an `Option` on the value, if there is one; `None` stays `None`. | [Patterns](step-2/project.md) |
| **`Option::as_mut`** | Turns `&mut Option<T>` into `Option<&mut T>`, to use the value inside without moving it out. | [Live keys](step-7/live.md) |
| **`Option::filter`** | Keeps a `Some` only if a closure says yes, and gives `None` otherwise. | [Recording](step-7/recording.md) |
| **`Option<T>`** | `Some(value)` or `None`; Rust's replacement for null. | [lineup-audio](step-1/audio-engine.md) |
| **`Option::take`** | Moves the value out of an `Option` and leaves `None` in its place; handy to take ownership of something stored in a field. | [WAV export](step-5/export.md) |
| **Or-pattern** | `A \| B` in a pattern matches either `A` or `B`. | [Pitch classes and scales](step-3/theory.md) |
| **Ownership** | Every value has exactly one owner, and is dropped when the owner goes away. | [lineup-audio](step-1/audio-engine.md) |
| **`partition`** | Splits an iterator in two by a closure, in one pass: the items it says yes to and the rest, each collected into its own collection. | [Micromodes](step-8/micromodes.md#textures) |
| **`partition_point`** | On a slice sorted so that a test is `true` for a first part and `false` for the rest, the index where the `false` part begins, found by binary search. | [One harmony, four names](step-9/harmony-notations.md#finding-a-bars-groups-with-partition_point) |
| **Panic** | An unrecoverable error that stops the current thread (`panic!`, failed `assert!`). | [lineup-core](step-1/lineup-core.md) |
| **Property-based test** | A test that checks a rule which must hold for any input ("following the changes gives the new project"), rather than one answer for one input. | [What changed](step-5/diff.md) |
| **`pub(crate)`** | Visible everywhere inside the current crate, but not to other crates. | [Patterns](step-2/project.md) |
| **`pub(super)`** | Visible to the parent module and everything inside it. | [Saving](step-4/saving.md) |
| **Pure function** | A function whose result depends only on its arguments and that changes nothing else; the easiest kind of code to test. | [Tools](step-6/tools.md) |
| **`Rc<T>`** | A reference-counted pointer for one thread: cloning it adds to a count instead of copying the value, which is freed when the last `Rc` goes. `Rc<[T]>` shares a slice. | [One harmony, four names](step-9/harmony-notations.md#reading-the-notes-only-when-they-change) |
| **RAII / `Drop`** | Owning a value means owning its resource; dropping it releases the resource. | [lineup-audio](step-1/audio-engine.md) |
| **Range** | `a..b`, the numbers from `a` up to but not including `b`; a value with methods like `contains`. | [Steps to notes](step-3/notes.md) |
| **Recursion** | A function that calls itself on a smaller piece of the problem, until a piece is small enough to answer directly. | [Twelve-tone rows](step-8/twelve-tone.md#what-makes-a-row-interesting) |
| **`rem_euclid`** | A remainder that is never negative: `(-1_i32).rem_euclid(12)` is 11, while `-1 % 12` is -1. | [Pitch classes and scales](step-3/theory.md) |
| **`Result<T, E>`** | `Ok(value)` or `Err(error)`; Rust's way to return errors. | [lineup-audio](step-1/audio-engine.md) |
| **`retain`** | Keeps only the items of a `Vec` for which a closure returns `true`, in one pass. | [Steps to notes](step-3/notes.md) |
| **RON** | Rust Object Notation, a text format that looks like Rust's own struct syntax; Ferrous Lineup's project files use it. | [Saving](step-4/saving.md) |
| **`rsplit_once` / `split_once`** | Splits a string at the last (or first) place a pattern occurs, into the parts before and after, or `None` if it does not occur. | [Chord names](step-9/chord-names.md#rsplit_once-and-the-six-nine-chord) |
| **`rposition`** | The index of the last item of a slice that matches, searching from the end; `position` searches from the start. | [Micromodes](step-8/micromodes.md#the-walk-choosing-with-a-tuple) |
| **`saturating_sub`** | Subtraction that stops at the smallest value (0 for `usize`) instead of overflowing. | [Voices](step-3/voices.md) |
| **`Send`** | A marker trait: values of this type can be moved to another thread. | [lineup-audio](step-1/audio-engine.md) |
| **serde** | The usual crate for turning Rust values into text or bytes and back, through the `Serialize` and `Deserialize` traits. | [Saving](step-4/saving.md) |
| **serde container attributes** | `#[serde(try_from = "String", into = "String")]` on a type saves it as the `String` its `From` gives, and reads it back through its `TryFrom<String>`, so a value read from a file is checked like any other. | [Twelve-tone rows](step-8/twelve-tone.md#files-try_from-and-into) |
| **serde field attributes** | `#[serde(default)]` fills in a missing field with its `Default` when reading; `#[serde(skip_serializing_if = "path")]` leaves a field out when writing if that function says so. | [Time signatures](step-8/time-signatures.md#in-files-format-3) |
| **Slice** | A borrowed view of part of an array or `Vec`: `&[T]` or `&mut [T]`. | [lineup-audio](step-1/audio-engine.md) |
| **Slice pattern** | `let [a, b, c, ..] = *bytes else { ... }`: matches a slice by its shape, naming the first items, so no indexing can panic. | [MIDI messages](step-7/messages.md) |
| **Slice pattern with `@`** | `[first @ .., last]` matches a slice of two or more items and names everything but the last as the slice `first`. | [Micromodes](step-8/micromodes.md#checking-a-table-with-tests) |
| **State machine** | Code that keeps a current state, often an enum, and moves between states as things happen. | [Voices](step-3/voices.md) |
| **`'static`** | A lifetime meaning "lives as long as needed"; for closures, "borrows nothing temporary". | [lineup-audio](step-1/audio-engine.md) |
| **`static` item** | `static NAME: T = ...;` is one value at one fixed address for the whole program. A `const` is pasted in wherever it is used instead. | [The library](step-8/library.md#styles) |
| **`step_by`** | Takes every n-th item of an iterator: `(0..8).step_by(2)` is 0, 2, 4, 6. | [Tools](step-6/tools.md) |
| **Struct update syntax** | `NoteEvent { length: 2, ..note }`: a new struct with some fields changed and the rest taken from `note`. | [Steps to notes](step-3/notes.md) |
| **Target** | The kind of system a program is compiled for, like `x86_64-pc-windows-gnu`; `rustup target add` installs Rust's library for one. | [Linux, macOS and Windows](step-4/platforms.md) |
| **`thread::Builder`** | Starts a thread with options such as a name; its `spawn` returns a `Result` instead of panicking when the system refuses. | [Live keys](step-7/live.md) |
| **Trait** | A set of methods a type promises to provide, like an interface. | [lineup-core](step-1/lineup-core.md) |
| **Trait bound** | A requirement on a generic type, such as `T: FromSample<f32>`. | [lineup-audio](step-1/audio-engine.md) |
| **`TryFrom` / `TryInto`** | Conversions that can fail, returning a `Result`. | [Saving](step-4/saving.md) |
| **Tuple** | A fixed group of values, possibly of different types, like `(usize, Note)`; it can be matched and taken apart. | [Pitch classes and scales](step-3/theory.md) |
| **Tuple comparison** | Tuples compare item by item: the second item only decides when the first ones are equal. A tuple as a sort key gives tie-breaks in order. | [Micromodes](step-8/micromodes.md#the-walk-choosing-with-a-tuple) |
| **Tuple struct** | A struct whose fields have no names, like `struct Note(u8)`; the fields are read as `.0`, `.1`... | [lineup-core](step-1/lineup-core.md) |
| **Turbofish (`::<>`)** | Writing a generic function's type out at the call, as in `collect::<Vec<Note>>()`, when Rust cannot work it out. | [lineup-audio](step-1/audio-engine.md) |
| **Type alias** | A second name for an existing type: `type StepRow = [bool; 64];`. | [Patterns](step-2/project.md) |
| **`VecDeque<T>`** | A double-ended queue: a ring buffer that is cheap to add to and take from at both ends. | [Undo and redo](step-5/history.md) |
| **`windows`** | Every run of n neighbours of a slice, overlapping: `[1, 2, 3].windows(2)` gives `[1, 2]` and `[2, 3]`. | [Tests](step-1/ci.md) |
| **`wrapping_add` / `wrapping_mul`** | Arithmetic that wraps around on overflow instead of stopping a debug build, for when wrapping is what you want, as in random numbers. | [Twelve-tone rows](step-8/twelve-tone.md#what-makes-a-row-interesting) |
| **Workspace** | A group of crates built together with one `Cargo.lock`. | [Workspace](step-1/workspace.md) |
| **`zip`** | Pairs the items of two iterators, `(a, b)`, and stops when either runs out. | [What changed](step-5/diff.md) |

## Where to learn more

- *The Rust Programming Language* ("the book"): installed with Rust, open
  it offline with `rustup doc --book`.
- *Rust by Example*: `rustup doc --rust-by-example`.
- The standard library docs: `rustup doc --std`.
