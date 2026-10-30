# Rust glossary

Short definitions of the Rust terms used in this book, with the chapter
that explains each one.

| Term | Meaning | Explained in |
|------|---------|--------------|
| **Anonymous lifetime (`'_`)** | A lifetime written without a name, meaning "the lifetime of the borrowed argument". | [Tempo, meter, swing and feel](../sound/tempo.md#swing) |
| **`Arc<T>`** | A shared, reference-counted pointer that is safe to use across threads; the data is freed when the last `Arc` goes. | [Instruments](../sound/instruments.md) |
| **Array** | A fixed number of values stored inline, like `[u8; 12]`; the length is part of the type. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#every-form-from-one-function) |
| **Array `map`** | `[T; N]::map` calls a closure on each item of an array and gives back a new array of the same length, with no `Vec` and no `collect`. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#every-form-from-one-function) |
| **`array::from_fn`** | Builds an array by calling a closure once for each element. | [Instruments](../sound/instruments.md) |
| **`as` cast** | Converts between number types, or a fieldless enum to its number: `PitchClass::D as u8` is 2. | [Five crates, and what each one may know](../program/crates.md#numbers-as) |
| **Associated function** | A function in an `impl` block without `self`, called as `Type::name()`. `Note::new` is one. | [Five crates, and what each one may know](../program/crates.md#methods-impl-blocks) |
| **Associated type** | A type that each implementation of a trait chooses, like `type Error = FileError;` in an `impl TryFrom`. | [Saving](../program/saving.md) |
| **Atomic** | A value (like `AtomicUsize`) that threads can read and write without a lock. | [Two threads](../program/threads.md) |
| **`AtomicU64` for an `f64`** | There is no atomic float, but `f64::to_bits` turns a float into the `u64` with the same bits, and `f64::from_bits` turns it back, so an `AtomicU64` can share one. | [MIDI](../sound/midi.md) |
| **Backtracking** | Building an answer a piece at a time, and taking the last piece back when it leads nowhere; usually a function that calls itself. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#what-makes-a-row-interesting) |
| **Big-endian / `to_be_bytes`** | Writing a number's most significant byte first; `6u32.to_be_bytes()` is `[0, 0, 0, 6]`. MIDI files use it. | [MIDI](../sound/midi.md) |
| **Bit operations** | `&` (and), `\|` (or), `!` (not), `<<` and `>>` (shift) work on the bits of an integer. | [Pitch classes, notes and spelling](../theory/pitch.md#a-set-of-pitch-classes-in-twelve-bits) |
| **`bool::then` / `then_some`** | Turn a condition into an `Option`: `cond.then_some(x)` is `Some(x)` if `cond` is true and `None` if not; `cond.then(\|\| ...)` only runs the closure when it is true. | [Editing tools as plain functions](../theory/tools.md) |
| **Borrow** | Using a value through a reference (`&T` shared, `&mut T` mutable) without taking ownership. | [Two threads](../program/threads.md) |
| **Borrow checker** | The part of the compiler that enforces: one `&mut` *or* any number of `&` at a time, and no reference outliving its value. | [Telling the audio thread what changed](../program/sync.md#reading-edit) |
| **`Borrow` trait** | A promise that an owned type (like `PathBuf`) hashes and compares exactly like the type it holds (`Path`), so a `HashMap<PathBuf, _>` can be searched with a `&Path`. | [Telling the audio thread what changed](../program/sync.md) |
| **`Box<T>`** | An owned pointer to a value on the heap. | [Drawing the windows with egui](../program/egui.md) |
| **Byte string (`b"..."`)** | A string literal that is a `&[u8; N]` of ASCII codes instead of a `&str`, like `b"MThd"`. | [MIDI](../sound/midi.md) |
| **Capacity** | How many items a `Vec` can hold before it must allocate more memory. | [Two threads](../program/threads.md) |
| **Cargo** | Rust's build tool and package manager. | [Five crates, and what each one may know](../program/crates.md) |
| **`Cell<T>`** | A box whose value can be replaced through a shared reference, by moving whole values in and out with `get` and `set` (for `Copy` types); nothing can hold a reference to the inside, so nothing reads it while it changes. | [Roman numerals, the harmony keys and chord functions](../theory/numerals.md#a-setting-every-function-can-see-one-per-thread) |
| **`cfg` / `cfg_attr`** | Conditional compilation: `#[cfg(target_os = "windows")]` keeps the next item only when the condition is true, and `cfg_attr` applies an attribute only then. | [Three systems and continuous integration](../shipping/ci.md) |
| **`#[cfg(test)]` on an item** | Keeps one function (or `use`, or struct) only in test builds, so a helper that only the tests call does not warn as unused in a normal build. | [Micromodes](../theory/micromodes.md#checking-a-table-with-tests) |
| **`checked_add_signed`** | Adds a signed number to an unsigned one, like `usize + isize`, and gives `None` instead of going below zero or overflowing. | [Editing tools as plain functions](../theory/tools.md) |
| **`chunk_by`** | Splits a slice into runs of neighbours that belong together, without copying. | [The editors](../program/editors.md) |
| **`chunks` / `chunks_exact`** | Split a slice into pieces of n items; `chunks_exact` leaves out a last piece that is too short. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#what-makes-a-row-interesting) |
| **`chars` / `Chars::as_str`** | `text.chars()` walks a string one whole character at a time ("♭" is three bytes but one `char`); `as_str()` is the part not walked yet, borrowed from the same text. | [Pitch classes, notes and spelling](../theory/pitch.md#writing-and-reading-a-name) |
| **Closure** | An anonymous function, `\|args\| body`, that can capture variables from its surroundings. | [Two threads](../program/threads.md) |
| **Closure captures fields** | Since the 2021 edition, a closure that uses `self.plays` borrows only that field, not all of `self`, so other fields can be borrowed at the same time. | [The sequencer](../sound/sequencer.md#when-the-song-changes-under-a-playing-note) |
| **`cmp::Reverse`** | Wraps a value so that it compares the other way round: sorting by `Reverse(count)` puts the largest count first. | [Micromodes](../theory/micromodes.md#counting-groups-of-three) |
| **Collecting `Option`s** | Collecting an iterator of `Option<T>` into an `Option<Vec<T>>` gives `Some` of all the values, or `None` as soon as one item is `None`. `Result` works the same way. | [Editing tools as plain functions](../theory/tools.md) |
| **`const fn`** | A function simple enough to run while compiling, so it can be used to set a `const`. | [Saving](../program/saving.md) |
| **Constant promotion** | Writing `&[1, 2, 3]` of plain constant values gives a `&'static` borrow, because the compiler stores the values with the program. A function call inside the brackets, even a `const fn`, is not promoted. | [The library](../band/library.md#macro_rules-and-why-not-a-function) |
| **`Copy`** | A marker trait for small values that are copied instead of moved, like numbers and `Note`; `let held = list[i];` then copies the item out. | [Five crates, and what each one may know](../program/crates.md#traits-and-derive) |
| **`count_ones`** | How many bits of an integer are 1: the size of a set stored as bits. | [Pitch classes, notes and spelling](../theory/pitch.md#a-set-of-pitch-classes-in-twelve-bits) |
| **Crate** | One library or program; the unit Cargo builds and publishes. | [Five crates, and what each one may know](../program/crates.md) |
| **`cycle`** | Repeats an iterator forever. Safe because iterators are lazy: something else, like `zip`, decides when to stop. | [Editing tools as plain functions](../theory/tools.md) |
| **Declarative macro (`macro_rules!`)** | A macro defined by patterns: `macro_rules! hit { ($at:expr, ...) => { ... } }` matches what it is given and pastes it into a template, before the code is compiled. | [The library](../band/library.md#macro_rules-and-why-not-a-function) |
| **`Default`** | A trait for "the default value" of a type; on an enum, `#[derive(Default)]` needs `#[default]` on one variant. | [Five crates, and what each one may know](../program/crates.md#traits-and-derive) |
| **Deref coercion** | Rust turning `&Vec<T>` into `&[T]` (or `&String` into `&str`) automatically where needed. | [MIDI](../sound/midi.md#writing-one-two-or-three-bytes) |
| **Dereference (`*`)** | Follow a reference to the value it points at, to read or write it. | [The project](../program/project.md) |
| **Derive** | `#[derive(Trait)]` asks the compiler to implement common traits automatically. | [Five crates, and what each one may know](../program/crates.md#traits-and-derive) |
| **Derive macro** | Code that writes a trait implementation for a type; serde's `#[derive(Serialize)]` comes from a crate, not from the compiler. | [Saving](../program/saving.md) |
| **`Display`** | The trait behind `{}` in `format!` and behind `to_string()`. Implement its `fmt` method once, and every place writes the value the same way. | [Five crates, and what each one may know](../program/crates.md#traits-and-derive) |
| **Disjoint borrows** | Borrowing two different fields of a struct at once, even one mutably; allowed because they cannot overlap. | [Drawing the windows with egui](../program/egui.md#a-theme-is-one-function) |
| **`div_ceil`** | Integer division that rounds up: `16.div_ceil(6)` is 3, where `16 / 6` is 2. | [Micromodes](../theory/micromodes.md#the-walk-choosing-with-a-tuple) |
| **`drain(..)`** | Takes every item out of a `Vec`, in order, and keeps the `Vec`'s memory for later. | [MIDI](../sound/midi.md) |
| **`dyn Trait`** | A trait object: some type implementing `Trait`, chosen at run time. | [Saving](../program/saving.md#errors-as-an-enum) |
| **Edition** | A version of the language (2015, 2018, 2021, 2024) chosen per crate. | [Five crates, and what each one may know](../program/crates.md) |
| **Enum** | A type whose value is one of several variants, each of which can hold data. | [Two threads](../program/threads.md) |
| **Enum size** | An enum takes as much room as its biggest variant; putting a big variant's data in a `Box` keeps every value small. Clippy's `large_enum_variant` lint points it out. | [MIDI](../sound/midi.md#midi-out-an-instrument-with-an-outbox) |
| **`env!` and `concat!`** | Macros that run while the program is compiled: `env!("CARGO_PKG_VERSION")` becomes the version Cargo read from `Cargo.toml`, and `concat!` joins literals into one `&'static str`, so both can build a `const`. | [Drawing the windows with egui](../program/egui.md#help-the-manual-and-about) |
| **`expect`** | Like `unwrap`, but panics with the message given. It states a promise ("this is always `Some`"), best kept by a test. | [Pitch-class sets, and one harmony with four names](../theory/sets.md#224-classes-in-a-table) |
| **Feature** | An optional part of a crate, turned on or off in `Cargo.toml`; features can only add, never take away. | [Saving](../program/saving.md) |
| **`find_map`** | Calls a closure on each item and returns the first `Some` it gives. | [Chords](../theory/chords.md) |
| **`flat_map`** | Maps each item to an iterator and joins all of them into one iterator. | [The editors](../program/editors.md) |
| **Float to integer `as`** | `x as usize` drops the fraction and *saturates*: negative numbers and NaN become 0, numbers too big become the type's maximum. It never panics. | [MIDI](../sound/midi.md) |
| **`fn` pointer** | The type of a plain function, like `fn(usize) -> Change`. Enum variants that hold data are such functions too: `Change::RemovePattern` builds a `Change` from a `usize`. | [Telling the audio thread what changed](../program/sync.md) |
| **`Fn` / `FnMut` / `FnOnce`** | The traits closures implement: only reads, mutates its captures, or can be called once. | [Instruments](../sound/instruments.md#closures-that-remember-fnmut) |
| **`From` / `Into`** | Conversion traits: implementing `From<A> for B` also gives `a.into()`. The `?` operator uses `From` to convert errors. | [The project](../program/project.md#names-and-impl-intostring) |
| **`FromIterator`** | The trait behind `collect()`; implement it to collect into your own type. | [Pitch classes, notes and spelling](../theory/pitch.md) |
| **`FromStr` / `str::parse`** | The trait for "can be read from text": implement `from_str`, and `"Am7".parse::<ChordSymbol>()` works, as it does for numbers. | [Chords](../theory/chords.md#reading-a-whole-symbol) |
| **Generic** | Code written for a placeholder type `T`, filled in at compile time. | [Two threads](../program/threads.md#generics-one-function-many-sample-types) |
| **Glob import** | `use super::*;` brings in every name the other module can see, including a parent's private items when used from a child. Two glob imports that bring the same name make it ambiguous. | [How a demo is written](../songs/demos.md#one-file-per-genre) |
| **`HashMap<K, V>`** | A map from keys to values that finds a key in about the same time however many entries it holds. | [Telling the audio thread what changed](../program/sync.md) |
| **`if let` / `while let`** | Run a block if (or while) a value matches one pattern. | [Drawing the windows with egui](../program/egui.md) |
| **`ilog2`** | The integer base-2 logarithm: how many times a number can be halved before reaching 1. `8u8.ilog2()` is 3. It panics on 0. | [Tempo, meter, swing and feel](../sound/tempo.md#in-files-and-midi-files) |
| **`impl Trait`** | As an argument type: any type with this trait. As a return type: one type with this trait, left unnamed. | [The project](../program/project.md#names-and-impl-intostring) |
| **Inner attribute (`#![...]`)** | An attribute that applies to the thing it is written inside, such as the whole crate at the top of `main.rs`. | [Three systems and continuous integration](../shipping/ci.md) |
| **Invariant** | A rule that is always true about a value, usually protected by private fields. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#a-row-in-rust-tonerow) |
| **`is_some_and`** | `true` if an `Option` is `Some` and its value passes a test, `false` for `None`. | [Roman numerals, the harmony keys and chord functions](../theory/numerals.md#which-role-comes-first) |
| **Iterator** | A lazy sequence of values, transformed with methods like `map`, `filter`, `count`. | [How the code is tested](../shipping/testing.md) |
| **`JoinHandle<T>`** | What `thread::spawn` returns. `join()` waits for the thread to end and gives back what its closure returned, or an `Err` if it panicked. | [Exporting](../sound/export.md) |
| **Lazy** | Iterators do no work until something pulls items out of them. | [Editing tools as plain functions](../theory/tools.md#one-note-of-the-key-up-find_map) |
| **Leading underscore (`_name`)** | Tells the compiler that a variable or field is unused on purpose, so it does not warn about it. | [Exporting](../sound/export.md) |
| **`let ... else`** | Destructure a pattern, or run an `else` block that must leave the function. | [Telling the audio thread what changed](../program/sync.md#the-order-of-everything) |
| **Let chain** | `if cond && let Some(x) = value { ... }`: conditions and pattern matches in one `if` (Rust 1.88). | [Two threads](../program/threads.md) |
| **Literal in a pattern** | `Event::Key { repeat: false, .. }`: a value written in a pattern only matches when the field equals it. | [MIDI](../sound/midi.md#the-typing-keyboard) |
| **`match`** | Compare a value against patterns; must cover every case. | [Two threads](../program/threads.md) |
| **Match guard** | `pattern if condition =>`: the arm matches only if the condition is also true. | [Chords](../theory/chords.md#labels-match-guards-and-nested-patterns) |
| **`matches!`** | A macro that is `true` if a value matches a pattern. | [Pitch classes, notes and spelling](../theory/pitch.md) |
| **`mem::replace`** | Put a new value into a place and get the old one back. | [Two threads](../program/threads.md) |
| **`mem::swap`** | Exchanges the values behind two `&mut` references, without copying either. | [Undo and redo](../program/undo.md) |
| **`mem::take`** | Take a value out of a place, leaving the type's default (such as an empty `Vec`) behind. | [The sequencer](../sound/sequencer.md) |
| **Method** | A function in an `impl` block that takes `self`, `&self` or `&mut self`. | [Five crates, and what each one may know](../program/crates.md#methods-impl-blocks) |
| **`min_by_key`** | The item of an iterator with the smallest key, or `None` if there are no items. | [Pitch-class sets, and one harmony with four names](../theory/sets.md#set-theory-in-twelve-bits) |
| **Module** | A namespace inside a crate, usually one file, declared with `mod`. | [Five crates, and what each one may know](../program/crates.md) |
| **Monomorphization** | The compiler making a specialized copy of generic code for each type used. | [Two threads](../program/threads.md) |
| **Move** | Transferring ownership of a value; the old name becomes unusable. | [Two threads](../program/threads.md) |
| **`move` closure** | A closure that takes ownership of everything it uses instead of borrowing it; needed for a closure given to another thread. | [Exporting](../sound/export.md) |
| **`mpsc` channel** | A queue from the standard library with many senders (`Sender` can be cloned) and one receiver; `try_iter` takes what is waiting without waiting for more. | [MIDI](../sound/midi.md) |
| **`Mutex<T>` and poisoning** | A lock that lets one thread at a time reach the `T` inside. If a thread panics while holding it, the mutex is *poisoned*, and `lock` returns an error until you take the data anyway with `PoisonError::into_inner`. | [MIDI](../sound/midi.md) |
| **Newtype** | A struct with one field that wraps another type to give it its own meaning. | [Pitch classes, notes and spelling](../theory/pitch.md#a-note-a-midi-number-in-a-type-of-its-own) |
| **`?` operator** | Return early with the error if a `Result` is `Err` (or an `Option` is `None`). | [Two threads](../program/threads.md) |
| **`Option::and_then`** | Calls a closure that itself returns an `Option` on the value, if there is one; `None` stays `None`. | [The editors](../program/editors.md#moving-a-group) |
| **`Option::as_mut`** | Turns `&mut Option<T>` into `Option<&mut T>`, to use the value inside without moving it out. | [Two threads](../program/threads.md#audiothreadprocess) |
| **`Option::filter`** | Keeps a `Some` only if a closure says yes, and gives `None` otherwise. | [MIDI](../sound/midi.md) |
| **`Option<T>`** | `Some(value)` or `None`; Rust's replacement for null. | [Two threads](../program/threads.md) |
| **`Option::take`** | Moves the value out of an `Option` and leaves `None` in its place; handy to take ownership of something stored in a field. | [Exporting](../sound/export.md) |
| **Or-pattern** | `A \| B` in a pattern matches either `A` or `B`. | [Drawing the windows with egui](../program/egui.md#mainrs) |
| **Ownership** | Every value has exactly one owner, and is dropped when the owner goes away. | [Two threads](../program/threads.md) |
| **`partition`** | Splits an iterator in two by a closure, in one pass: the items it says yes to and the rest, each collected into its own collection. | [Micromodes](../theory/micromodes.md#textures) |
| **`partition_point`** | On a slice sorted so that a test is `true` for a first part and `false` for the rest, the index where the `false` part begins, found by binary search. | [Pitch-class sets, and one harmony with four names](../theory/sets.md#reading-only-when-something-changes) |
| **Panic** | An unrecoverable error that stops the current thread (`panic!`, failed `assert!`). | [Five crates, and what each one may know](../program/crates.md#panics) |
| **Property-based test** | A test that checks a rule which must hold for any input ("following the changes gives the new project"), rather than one answer for one input. | [Telling the audio thread what changed](../program/sync.md#testing-with-a-model) |
| **`pub(crate)`** | Visible everywhere inside the current crate, but not to other crates. | [The project](../program/project.md#handing-out-mut-pattern-safely) |
| **`pub(super)`** | Visible to the parent module and everything inside it. | [Saving](../program/saving.md) |
| **Pure function** | A function whose result depends only on its arguments and that changes nothing else; the easiest kind of code to test. | [Editing tools as plain functions](../theory/tools.md) |
| **`Rc<T>`** | A reference-counted pointer for one thread: cloning it adds to a count instead of copying the value, which is freed when the last `Rc` goes. `Rc<[T]>` shares a slice. | [Pitch-class sets, and one harmony with four names](../theory/sets.md#reading-only-when-something-changes) |
| **RAII / `Drop`** | Owning a value means owning its resource; dropping it releases the resource. | [Two threads](../program/threads.md) |
| **Range** | `a..b`, the numbers from `a` up to but not including `b`; a value with methods like `contains`. | [The sequencer](../sound/sequencer.md#testing-with-iterators) |
| **Recursion** | A function that calls itself on a smaller piece of the problem, until a piece is small enough to answer directly. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#what-makes-a-row-interesting) |
| **`rem_euclid`** | A remainder that is never negative: `(-1_i32).rem_euclid(12)` is 11, while `-1 % 12` is -1. | [Pitch classes, notes and spelling](../theory/pitch.md) |
| **`Result<T, E>`** | `Ok(value)` or `Err(error)`; Rust's way to return errors. | [Two threads](../program/threads.md) |
| **`retain`** | Keeps only the items of a `Vec` for which a closure returns `true`, in one pass. | [The project](../program/project.md#four-rules-one-gatekeeper) |
| **RON** | Rust Object Notation, a text format that looks like Rust's own struct syntax; Ferrous Lineup's project files use it. | [Saving](../program/saving.md) |
| **`rsplit_once` / `split_once`** | Splits a string at the last (or first) place a pattern occurs, into the parts before and after, or `None` if it does not occur. | [Chords](../theory/chords.md#slash-chords-and-the-six-nine-chord) |
| **`rposition`** | The index of the last item of a slice that matches, searching from the end; `position` searches from the start. | [Micromodes](../theory/micromodes.md#the-walk-choosing-with-a-tuple) |
| **`saturating_sub`** | Subtraction that stops at the smallest value (0 for `usize`) instead of overflowing. | [Editing tools as plain functions](../theory/tools.md#arpeggios-cycle-step_by-and-zip) |
| **`Send`** | A marker trait: values of this type can be moved to another thread. | [Two threads](../program/threads.md#the-audio-callback-is-a-closure) |
| **serde** | The usual crate for turning Rust values into text or bytes and back, through the `Serialize` and `Deserialize` traits. | [Saving](../program/saving.md) |
| **serde container attributes** | `#[serde(try_from = "String", into = "String")]` on a type saves it as the `String` its `From` gives, and reads it back through its `TryFrom<String>`, so a value read from a file is checked like any other. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#files-try_from-and-into) |
| **serde field attributes** | `#[serde(default)]` fills in a missing field with its `Default` when reading; `#[serde(skip_serializing_if = "path")]` leaves a field out when writing if that function says so. | [Saving](../program/saving.md#serde-describe-the-data-once) |
| **Slice** | A borrowed view of part of an array or `Vec`: `&[T]` or `&mut [T]`. | [Two threads](../program/threads.md#the-audio-callback-is-a-closure) |
| **Slice pattern** | `let [a, b, c, ..] = *bytes else { ... }`: matches a slice by its shape, naming the first items, so no indexing can panic. | [MIDI](../sound/midi.md) |
| **Slice pattern with `@`** | `[first @ .., last]` matches a slice of two or more items and names everything but the last as the slice `first`. | [Micromodes](../theory/micromodes.md#checking-a-table-with-tests) |
| **State machine** | Code that keeps a current state, often an enum, and moves between states as things happen. | [Instruments](../sound/instruments.md) |
| **`'static`** | A lifetime meaning "lives as long as needed"; for closures, "borrows nothing temporary". | [Two threads](../program/threads.md#the-audio-callback-is-a-closure) |
| **`static` item** | `static NAME: T = ...;` is one value at one fixed address for the whole program. A `const` is pasted in wherever it is used instead. | [The library](../band/library.md#styles-a-table-in-a-static) |
| **`step_by`** | Takes every n-th item of an iterator: `(0..8).step_by(2)` is 0, 2, 4, 6. | [Editing tools as plain functions](../theory/tools.md#arpeggios-cycle-step_by-and-zip) |
| **Struct update syntax** | `NoteEvent { length: 2, ..note }`: a new struct with some fields changed and the rest taken from `note`. | [Drawing the windows with egui](../program/egui.md#mainrs) |
| **Target** | The kind of system a program is compiled for, like `x86_64-pc-windows-gnu`; `rustup target add` installs Rust's library for one. | [Three systems and continuous integration](../shipping/ci.md) |
| **`thread::Builder`** | Starts a thread with options such as a name; its `spawn` returns a `Result` instead of panicking when the system refuses. | [MIDI](../sound/midi.md) |
| **`thread_local!`** | Declares a value of which each thread has its own copy, reached with `.with(...)`; the app keeps its settings for numerals, note names and language this way, so tests running side by side never see each other's. | [Roman numerals, the harmony keys and chord functions](../theory/numerals.md#a-setting-every-function-can-see-one-per-thread) |
| **Trait** | A set of methods a type promises to provide, like an interface. | [Five crates, and what each one may know](../program/crates.md#traits-and-derive) |
| **Trait bound** | A requirement on a generic type, such as `T: FromSample<f32>`. | [Two threads](../program/threads.md) |
| **`TryFrom` / `TryInto`** | Conversions that can fail, returning a `Result`. | [Saving](../program/saving.md) |
| **Tuple** | A fixed group of values, possibly of different types, like `(usize, Note)`; it can be matched and taken apart. | [Pitch classes, notes and spelling](../theory/pitch.md#matching-on-two-values-at-once) |
| **Tuple comparison** | Tuples compare item by item: the second item only decides when the first ones are equal. A tuple as a sort key gives tie-breaks in order. | [The mixer](../sound/mixer.md#sorting-and-grouping) |
| **Tuple struct** | A struct whose fields have no names, like `struct Note(u8)`; the fields are read as `.0`, `.1`... | [Five crates, and what each one may know](../program/crates.md#a-first-type-note) |
| **Turbofish (`::<>`)** | Writing a generic function's type out at the call, as in `collect::<Vec<Note>>()`, when Rust cannot work it out. | [Two threads](../program/threads.md#generics-one-function-many-sample-types) |
| **Type alias** | A second name for an existing type: `type Hit = (u8, i32);`. | [The library](../band/library.md#rhythms-one-line-per-drum) |
| **`VecDeque<T>`** | A double-ended queue: a ring buffer that is cheap to add to and take from at both ends. | [Undo and redo](../program/undo.md) |
| **`windows`** | Every run of n neighbours of a slice, overlapping: `[1, 2, 3].windows(2)` gives `[1, 2]` and `[2, 3]`. | [How the code is tested](../shipping/testing.md) |
| **`wrapping_add` / `wrapping_mul`** | Arithmetic that wraps around on overflow instead of stopping a debug build, for when wrapping is what you want, as in random numbers. | [Twelve-tone rows and their rules](../theory/twelve-tone.md#what-makes-a-row-interesting) |
| **Workspace** | A group of crates built together with one `Cargo.lock`. | [Five crates, and what each one may know](../program/crates.md) |
| **`zip`** | Pairs the items of two iterators, `(a, b)`, and stops when either runs out. | [Telling the audio thread what changed](../program/sync.md) |

## Where to learn more

- *The Rust Programming Language* ("the book"): installed with Rust, open
  it offline with `rustup doc --book`.
- *Rust by Example*: `rustup doc --rust-by-example`.
- The standard library docs: `rustup doc --std`.
