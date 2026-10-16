# Rust glossary

Short definitions of the Rust terms used in this book, with the chapter
where each one first appears in real code.

| Term | Meaning | First seen in |
|------|---------|---------------|
| **`Arc<T>`** | A shared, reference-counted pointer that is safe to use across threads; the data is freed when the last `Arc` goes. | [Instruments](step-2/instruments.md) |
| **Array** | A fixed number of values stored inline, like `[bool; 64]`; the length is part of the type. | [Patterns](step-2/project.md) |
| **Associated function** | A function in an `impl` block without `self`, called as `Type::name()`. `Note::new` is one. | [loom-core](step-1/loom-core.md) |
| **Atomic** | A value (like `AtomicUsize`) that threads can read and write without a lock. | [Audio thread](step-2/realtime.md) |
| **Borrow** | Using a value through a reference (`&T` shared, `&mut T` mutable) without taking ownership. | [loom-audio](step-1/audio-engine.md) |
| **Borrow checker** | The part of the compiler that enforces: one `&mut` *or* any number of `&` at a time, and no reference outliving its value. | [loom-audio](step-1/audio-engine.md) |
| **`Box<T>`** | An owned pointer to a value on the heap. | [loom-app](step-1/window.md) |
| **Capacity** | How many items a `Vec` can hold before it must allocate more memory. | [Audio thread](step-2/realtime.md) |
| **Cargo** | Rust's build tool and package manager. | [Workspace](step-1/workspace.md) |
| **Closure** | An anonymous function, `\|args\| body`, that can capture variables from its surroundings. | [loom-audio](step-1/audio-engine.md) |
| **Crate** | One library or program; the unit Cargo builds and publishes. | [Workspace](step-1/workspace.md) |
| **Dereference (`*`)** | Follow a reference to the value it points at, to read or write it. | [Patterns](step-2/project.md) |
| **Derive** | `#[derive(Trait)]` asks the compiler to implement common traits automatically. | [loom-core](step-1/loom-core.md) |
| **`dyn Trait`** | A trait object: some type implementing `Trait`, chosen at run time. | [loom-app](step-1/window.md) |
| **Edition** | A version of the language (2015, 2018, 2021, 2024) chosen per crate. | [Workspace](step-1/workspace.md) |
| **Enum** | A type whose value is one of several variants, each of which can hold data. | [loom-audio](step-1/audio-engine.md) |
| **`Fn` / `FnMut` / `FnOnce`** | The traits closures implement: only reads, mutates its captures, or can be called once. | [Instruments](step-2/instruments.md) |
| **Generic** | Code written for a placeholder type `T`, filled in at compile time. | [loom-audio](step-1/audio-engine.md) |
| **`if let` / `while let`** | Run a block if (or while) a value matches one pattern. | [loom-app](step-1/window.md) |
| **Invariant** | A rule that is always true about a value, usually protected by private fields. | [Patterns](step-2/project.md) |
| **Iterator** | A lazy sequence of values, transformed with methods like `map`, `filter`, `count`. | [Tests](step-1/ci.md) |
| **`let ... else`** | Destructure a pattern, or run an `else` block that must leave the function. | [Instruments](step-2/instruments.md) |
| **Let chain** | `if cond && let Some(x) = value { ... }`: conditions and pattern matches in one `if` (Rust 1.88). | [Audio thread](step-2/realtime.md) |
| **`match`** | Compare a value against patterns; must cover every case. | [loom-audio](step-1/audio-engine.md) |
| **`mem::replace`** | Put a new value into a place and get the old one back. | [Audio thread](step-2/realtime.md) |
| **Method** | A function in an `impl` block that takes `self`, `&self` or `&mut self`. | [loom-core](step-1/loom-core.md) |
| **Module** | A namespace inside a crate, usually one file, declared with `mod`. | [Workspace](step-1/workspace.md) |
| **Monomorphization** | The compiler making a specialized copy of generic code for each type used. | [loom-audio](step-1/audio-engine.md) |
| **Move** | Transferring ownership of a value; the old name becomes unusable. | [loom-audio](step-1/audio-engine.md) |
| **Newtype** | A struct with one field that wraps another type to give it its own meaning. | [loom-core](step-1/loom-core.md) |
| **`?` operator** | Return early with the error if a `Result` is `Err` (or an `Option` is `None`). | [loom-audio](step-1/audio-engine.md) |
| **`Option<T>`** | `Some(value)` or `None`; Rust's replacement for null. | [loom-audio](step-1/audio-engine.md) |
| **Ownership** | Every value has exactly one owner, and is dropped when the owner goes away. | [loom-audio](step-1/audio-engine.md) |
| **Panic** | An unrecoverable error that stops the current thread (`panic!`, failed `assert!`). | [loom-core](step-1/loom-core.md) |
| **`pub(crate)`** | Visible everywhere inside the current crate, but not to other crates. | [Patterns](step-2/project.md) |
| **RAII / `Drop`** | Owning a value means owning its resource; dropping it releases the resource. | [loom-audio](step-1/audio-engine.md) |
| **`Result<T, E>`** | `Ok(value)` or `Err(error)`; Rust's way to return errors. | [loom-audio](step-1/audio-engine.md) |
| **`Send`** | A marker trait: values of this type can be moved to another thread. | [loom-audio](step-1/audio-engine.md) |
| **Slice** | A borrowed view of part of an array or `Vec`: `&[T]` or `&mut [T]`. | [loom-audio](step-1/audio-engine.md) |
| **`'static`** | A lifetime meaning "lives as long as needed"; for closures, "borrows nothing temporary". | [loom-audio](step-1/audio-engine.md) |
| **Trait** | A set of methods a type promises to provide, like an interface. | [loom-core](step-1/loom-core.md) |
| **Trait bound** | A requirement on a generic type, such as `T: FromSample<f32>`. | [loom-audio](step-1/audio-engine.md) |
| **Type alias** | A second name for an existing type: `type StepRow = [bool; 64];`. | [Patterns](step-2/project.md) |
| **Workspace** | A group of crates built together with one `Cargo.lock`. | [Workspace](step-1/workspace.md) |

## Where to learn more

- *The Rust Programming Language* ("the book"): installed with Rust, open
  it offline with `rustup doc --book`.
- *Rust by Example*: `rustup doc --rust-by-example`.
- The standard library docs: `rustup doc --std`.
