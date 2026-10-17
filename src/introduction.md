# Introduction

**Ferrous Lineup** is an open source music studio, written in Rust, that
plays songs with a backing band. It began as a pattern studio in the
spirit of FL Studio, where you build short patterns in a step sequencer
or a piano roll and arrange them into songs, and from step 9 on it grows
toward a backing band: type the chords of a song, pick a style, and
the band plays it. It also has a second goal: helping you
**learn harmony** while you play, by showing you scales, chords and their
names as you write.

Until step 9 the app was called **Ferrous Loom** ([A new name](step-9/new-name.md)
tells how it was renamed).

This book has a third goal: it is a **Rust course built around a real
project**. Every step of the project adds a chapter that explains:

- *what* was built and *why* it was designed that way, and
- *which Rust concepts* the code uses, pointing at the real files.

You do not need to know Rust before reading. If you know another
programming language, you will be able to follow along. Each step ends with
a few exercises you can try on your own copy of the code.

## How to read this book

You can read it three ways, all of them offline:

1. As plain Markdown files, in the book's `src` folder.
2. As a website on your computer: run `mdbook build` in the book's
   folder and open `book/index.html` in a browser.
3. As that website in a zip file, ready to unzip and open.

## The plan

| Step | What gets built | Rust topics |
|------|-----------------|-------------|
| 1 | Window, audio engine, test tone, CI | Cargo, crates, modules, structs, enums, traits, closures, threads, errors |
| 2 | Channel rack and step sequencer | Arrays, `Option` combinators, `Arc`, atomics, closures as `FnMut`, custom widgets, sample-accurate timing |
| 3 | Piano roll with harmony helpers | Enums and pattern matching for music theory, bit sets, iterators, your own traits and generics, state machines |
| 4 | Playlist, saving projects, macOS and Windows builds | Keeping rules across lists, serde and derive macros, error enums, newtypes, trait objects, `cfg` and features, a CI matrix |
| 5 | Undo and redo, WAV export | Generics without bounds, `VecDeque`, `mem::swap`, `FnOnce`, `fn` pointers, testing against a model, `HashMap` and `Borrow`, threads, `move` closures, `JoinHandle` |
| 6 | Harmony tools: selections, transposing, arpeggios, inversions, suggestions, ghost notes, velocities | Pure functions, collecting `Option`s, `checked_add_signed`, `find_map`, `cycle`, `step_by` and `zip`, knowledge as `const` tables, `Option::take` against the borrow checker, `flat_map` and lifetimes on returned iterators |
| 7 | MIDI in and out: keyboards, external synths, recording, `.mid` files | Slice patterns, byte strings and `to_be_bytes`, an `f64` in an `AtomicU64`, `Option::as_mut`, `Mutex` and poisoning, `mpsc` channels, `thread::Builder`, closures that capture fields, `Box` for a large enum variant, float `as` casts |
| 8 | Harmony keys, time signatures, a library of styles, rhythms and songs, demos | Knowledge as data, `macro_rules!`, constant promotion, `static` tables, struct update syntax, `Display`, `is_multiple_of` and `ilog2`, serde field attributes, enum variants with data, `&'static` references in actions, comparing snapshots with `PartialEq` |
| 9 | The band: chord names, a chord sheet the band plays, song form, swing, instrument sounds | `FromStr` and `str::parse`, `rsplit_once`, closures as `impl Fn` parameters, `[[bin]]` targets, byte strings as file-format tags |

The [Roadmap](roadmap.md) chapter keeps this table up to date.

## Name and license

"Ferrous" means "made of iron", and rust is iron oxide; a lineup is the
players of a band. The first name, Ferrous Loom, kept the "F L" initials
of FL Studio, and a loom weaves threads together the way the playlist
weaves patterns into a song. Ferrous Lineup is an independent project
with no connection to Image-Line or FL Studio.

Ferrous Lineup is free software, released under the GNU General Public
License, version 3 or later.
