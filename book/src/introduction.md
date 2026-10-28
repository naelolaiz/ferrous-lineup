# Introduction

**Ferrous Lineup** is an open source program, written in Rust, for
playing songs with a **backing band**. Type the chords of a song bar by
bar, pick a style, and a band (bass, drums and piano, with a guitar,
strings or an organ when the style has them) plays it in any key and
tempo. Change the style, the key or the tempo, and the band plays the
song again the new way. Every part the band plays lands on a track of
its own, where you can open it in the piano roll, change it by hand, or
leave it out and play it yourself.

It also has a second goal: helping you **learn harmony** while you play.
It names every chord you hear, with its numeral in the key and what it
does there (home, away or tension), and it knows the other ways music
names harmony too: pitch-class sets, twelve-tone rows and Kröpfl's
micromodes.

Steps 1 to 8 build what the band stands on: the sound, patterns and
tracks, a piano roll with harmony help, MIDI, and a library of styles,
rhythms and songs. Step 9 puts the band in front: a chord sheet to type
the song into, and a band that plays it.

This book has a third goal: it is a **Rust course built around a real
project**. Every step of the project adds a chapter that explains:

- *what* was built and *why* it was designed that way, and
- *which Rust concepts* the code uses, pointing at the real files.

You do not need to know Rust before reading. If you know another
programming language, you will be able to follow along. Each step ends with
a few exercises you can try on your own copy of the code.

## How to read this book

You can read it in four ways:

1. As plain Markdown files, in the book's `src` folder.
2. As a website on your computer: run `mdbook build` in the book's
   folder and open `book/index.html` in a browser.
3. Online, at <https://naelolaiz.github.io/ferrous-lineup/>.
4. As one PDF with every chapter in order, to read offline or print:
   [ferrous-lineup-book.pdf](https://naelolaiz.github.io/ferrous-lineup/ferrous-lineup-book.pdf).
   It has page numbers, the contents at the front, an index of words at
   the back, and bookmarks for every chapter in your PDF reader's side
   panel.

[The book online and as a PDF](step-9/book-online.md) tells how the last
two are made.

Source links in the chapters point at the current `main` branch, which is
the finished program. The book is a history: each step says when a later
step changed the code it shows, so a link is a place to read further, not
a promise that the file still looks exactly like the snippet.

## Where to start

- **To play and learn music**: read [Getting started](getting-started.md),
  then [A tour of the app](tour.md), then [Step 9](step-9/README.md).
  The earlier steps explain how it was built; you do not need them to use
  the app.
- **To learn Rust**: read Steps 1 to 9 in order. Keep the
  [Rust glossary](glossary.md) open, and try the exercises at the end of
  each step.
- **To contribute or look something up**: the [Roadmap](roadmap.md) says
  where the project is going; the [project file format](appendix-project-format.md),
  [keyboard shortcuts](appendix-shortcuts.md), [troubleshooting](troubleshooting.md)
  and [music glossary](music-glossary.md) are the reference pages.

## The plan

| Step | What gets built | Rust topics |
|------|-----------------|-------------|
| 1 | Window, audio engine, test tone, CI | Cargo, crates, `[[bin]]` targets, modules, structs, enums, traits, closures, threads, errors |
| 2 | Channel rack and step sequencer | Arrays, `Option` combinators, `Arc`, atomics, closures as `FnMut`, custom widgets, sample-accurate timing |
| 3 | Piano roll with harmony helpers | Enums and pattern matching for music theory, bit sets, iterators, your own traits and generics, state machines |
| 4 | Playlist, saving projects, macOS and Windows builds | Keeping rules across lists, serde and derive macros, error enums, newtypes, trait objects, `FnOnce`, `cfg` and features, a CI matrix |
| 5 | Undo and redo, WAV export | Generics without bounds, `VecDeque`, `mem::swap`, `fn` pointers, testing against a model, `HashMap` and `Borrow`, threads, `Send` and `'static`, `move` closures, `JoinHandle` |
| 6 | Harmony tools: selections, transposing, arpeggios, inversions, suggestions, ghost notes, velocities | Pure functions, collecting `Option`s, `checked_add_signed`, `find_map`, `cycle`, `step_by` and `zip`, knowledge as `const` tables, `Option::take` against the borrow checker, `flat_map` and lifetimes on returned iterators |
| 7 | MIDI in and out: keyboards, external synths, recording, `.mid` files | Slice patterns, byte strings and `to_be_bytes`, an `f64` in an `AtomicU64`, `Option::as_mut`, `Mutex` and poisoning, `mpsc` channels, `thread::Builder`, closures that capture fields, `Box` for a large enum variant, float `as` casts |
| 8 | Harmony keys, time signatures, a library of styles, rhythms and songs, demos | Knowledge as data, `macro_rules!`, constant promotion, `static` tables, struct update syntax, `Display`, `is_multiple_of` and `ilog2`, serde field attributes, enum variants with data, `&'static` references in actions, comparing snapshots with `PartialEq` |
| 9 | The band: chord names, a chord sheet, song form, swing, instrument sounds, then the repertoire, mixer, video, and reliability | `FromStr` and `str::parse`, `rsplit_once`, closures as `impl Fn` parameters, byte strings as file-format tags, H.264/MP3 encoding, PDF drawing, recovery state, command and MIDI queues |

The [Roadmap](roadmap.md) chapter keeps this table up to date.

## Name and license

"Ferrous" means "made of iron", and rust is iron oxide; a lineup is the
players of a band, the band that plays your songs.

Ferrous Lineup is free software, released under the GNU General Public
License, version 3 or later.
