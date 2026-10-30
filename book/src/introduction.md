# Introduction

**Ferrous Lineup** is an open source program, written in Rust, for
playing songs with a **backing band**. Type the chords of a song bar by
bar, pick a style, and a band (bass, drums and piano, with a guitar,
strings, horns or an organ when the style has them) plays it in any key
and tempo. Every part the band plays lands on a track of its own, where
you can open it in the piano roll, change it by hand, or leave it out
and play it yourself.

It is also a way to **learn harmony** while you play. It names every
chord you hear, with its numeral in the key and what it does there, and
it knows the other ways music names harmony too: pitch-class sets,
twelve-tone rows and Kröpfl's micromodes.

This book is about the inside: **how the code is organized, how it is
designed and why**, and how the music theory the app knows is written
in Rust. It explains the Rust each part uses as it goes, pointing at the
real files, so it is also a Rust course built around a real program. You
do not need to know Rust before reading; if you know another
programming language, you can follow along. To *use* the app, read the
[user manual](https://naelolaiz.github.io/ferrous-lineup/manual/)
instead: it has no code at all.

## How the book is organized

The chapters follow the code, from the program's shape to how it ships.
Each says what a part of the app does, with a link to the manual page
on using it; how it is designed and why; the Rust it uses; the files it
lives in; and the tests that guard it. Exercises close each part.

| Part | About | Rust along the way |
|------|-------|--------------------|
| I. The shape of the program | the five crates, the two threads, the project's data, undo, saving, the windows, two languages | workspaces and crates, modules, enums with data, rules kept by one gatekeeper, `Arc` and a lock-free ring buffer, `History<T>`, serde, error enums, `Drop`, immediate-mode drawing, thread-local settings |
| II. Sound | the sequencer, tempo and meter, the instruments, SoundFonts, the mixer, MIDI, exporting, the playback trace | timing to the sample, atomics, `FnMut` closures, a `Mutex` and `mpsc` queues, byte strings and slice patterns, the one `unsafe` block, `Copy` types with rules |
| III. Music theory in code | pitch, scales and keys, chords, numerals and functions, editing tools, hearing harmony, sets, rows, micromodes, voice leading, importing a recording | `match` tables, bit sets, iterators (`find_map`, `cycle`, `chunk_by`), collecting `Option`s, `FromStr` and `Display`, `try_from`, dynamic programming |
| IV. The band | the library, the chord sheet, how the band plays, the orchestra, song form, stops, the soloist, your own styles, practice tools, the lead sheet | `static` tables and `const fn`, `macro_rules!`, hashing and seeded generators, parsing text, drawing a PDF |
| V. Songs as data | how a demo is written, and how a MIDI performance or a score becomes one | `const` data in child modules, struct update syntax, `LazyLock`, sorting with `Ordering` |
| VI. Building, testing and shipping | the tests, the three systems and their CI, release packages, how these books are built | `#[cfg(test)]`, integration tests, `catch_unwind`, `cfg` and features, build scripts |

At the back are the [project file format](reference/project-format.md),
a [Rust glossary](reference/rust-glossary.md), a [music
glossary](reference/music-glossary.md) and [ideas for what comes
next](reference/ideas.md).

## Where to start

- **To learn Rust:** read the parts in order, with the [Rust
  glossary](reference/rust-glossary.md) open, and try the exercises
  at the end of each part. [Building it from
  source](building.md) gets the code running first.
- **To learn how the music theory is programmed:** start with Part III,
  and keep the [music glossary](reference/music-glossary.md) close.
  The theory itself, without code, is taught in the manual.
- **To change something:** find the part of the app in the contents, read
  its chapter, and run its tests before and after your change.

## How to read it

1. Online, at <https://naelolaiz.github.io/ferrous-lineup/>.
2. As one PDF, to read offline or print:
   [ferrous-lineup-book.pdf](https://naelolaiz.github.io/ferrous-lineup/ferrous-lineup-book.pdf).
   It has page numbers, the contents at the front, an index of words at
   the back, and bookmarks for every chapter.
3. As Markdown files, in the repository's `book/src` folder, or as a
   website on your computer with `mdbook serve --open` in `book/`.

The book is also in Spanish, at
<https://naelolaiz.github.io/ferrous-lineup/es/>. [How the books are
built](shipping/books.md) tells how the websites and PDFs are made.

Code links point at the `main` branch, and the snippets in the chapters
are copied from it. Where a snippet leaves lines out, it says so with
`// ...`.

## Name and license

"Ferrous" means "made of iron", and rust is iron oxide; a lineup is the
players of a band, the band that plays your songs.

Ferrous Lineup is free software, released under the GNU General Public
License, version 3 or later.
