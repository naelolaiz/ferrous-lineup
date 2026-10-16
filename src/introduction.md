# Introduction

**Ferrous Loom** is an open source music studio, written in Rust, inspired by
the workflow of pattern-based studios such as FL Studio. You build short
patterns in a step sequencer or a piano roll and arrange them into songs.
It also has a second goal: helping you **learn harmony** while you play, by
showing you scales, chords and their names as you write.

This book has a third goal: it is a **Rust course built around a real
project**. Every step of the project adds a chapter that explains:

- *what* was built and *why* it was designed that way, and
- *which Rust concepts* the code uses, pointing at the real files.

You do not need to know Rust before reading. If you know another
programming language, you will be able to follow along. Each step ends with
a few exercises you can try on your own copy of the code.

## How to read this book

You can read it three ways, all of them offline:

1. As plain Markdown files in the `docs/src` folder of the repository.
2. As a website on your computer: run `mdbook build docs` and open
   `docs/book/index.html` in a browser (see [Getting started](getting-started.md)).
3. As the `ferrous-loom-book` download attached to every CI run on GitHub.

## The plan

| Step | What gets built | Rust topics |
|------|-----------------|-------------|
| 1 | Window, audio engine, test tone, CI | Cargo, crates, modules, structs, enums, traits, closures, threads, errors |
| 2 | Channel rack and step sequencer | Arrays, `Option` combinators, `Arc`, atomics, closures as `FnMut`, custom widgets, sample-accurate timing |
| 3 | Piano roll with harmony helpers | Enums and pattern matching for music theory, bit sets, iterators, your own traits and generics, state machines |
| 4 | Playlist, saving projects, macOS and Windows builds | Serialization with serde, error handling, cross-platform builds |

The [Roadmap](roadmap.md) chapter keeps this table up to date.

## Name and license

The name keeps the "F L" initials as a friendly nod to the original, but
Ferrous Loom is an independent project with no connection to Image-Line or
FL Studio. "Ferrous" means "made of iron", and rust is iron oxide; a loom
weaves threads together, the way the playlist weaves patterns into a song.

Ferrous Loom is free software, released under the GNU General Public
License, version 3 or later.
