# Where the band ends up

Files: [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)
and [`README.md`](https://github.com/naelolaiz/ferrous-lineup/blob/main/README.md)

Ferrous Lineup began as a step sequencer with a test tone. It ends as a
backing band: a chord sheet, a library of styles and songs, a playlist,
a piano roll, a mixer, MIDI in and out, and a way to hear what the band
plays. The last chapters were less about new music than about making
that band trustworthy: recovery, recent projects, a sound card that can
come back, and commands that arrive.

## Five crates

The workspace is five crates, each with a job it can be tested on its
own:

- **`lineup-core`** holds the music and the song: notes, scales, chords
  and harmony, the chord sheet, patterns and the playlist, project files
  and diffs, undo history, MIDI files, lead sheets, and the library of
  styles, rhythms and songs. It has no audio and no UI, and depends only
  on `serde` and `ron`, so `cargo test -p lineup-core` is fast.
- **`lineup-audio`** is the real-time engine (`cpal`) and the offline
  renderer: drums and samplers, the Lineup Osc, SoundFont instruments,
  song and pattern playback, WAV export, and the trace of every note
  played.
- **`lineup-listen`** decodes a recording (MP3, WAV, FLAC or Ogg
  Vorbis) and listens for its beats, key, chords and meter. It is
  experimental: the results are approximate and meant as a starting
  point.
- **`lineup-video`** paints the app's views in memory and encodes them
  as H.264 and MP3 in an MP4 file. The encoders are built from their C
  sources, so no system codec is needed.
- **`lineup-app`** is the desktop program, `ferrous-lineup`, built with
  `egui`, with MIDI through `midir`. It owns the windows and the state
  that joins the other four crates.

## What it does now

A song can start on the chord sheet, in a MIDI file, in a chord chart
pasted as text, in a recording, or in one of the demos. Type chords bar
by bar, choose a style, and the band plays them in any key and tempo,
with an intro, choruses, an ending, variations, swing, shots and pushes.
Every part lands on a track of its own, where the piano roll can change
it by hand.

Around that are the tools the band grew: a style editor for your own
repeating parts, a mixer with pan, reverb and meters, a practice bar with
count-in, metronome and loops, and the harmony windows that name what
you hear and what might come next. MIDI keyboards play into it; MIDI Out
channels and a SoundFont play out. A song can leave as WAV, MIDI, a lead
sheet PDF, or an MP4 video. A project can survive a crash, and sound can
come back after the interface is unplugged.

## The numbers

The built-in library is written as data in `lineup-core`:

- **177** harmony styles,
- **112** rhythms, plus fills,
- **343** song demos.

That is enough to play *Ode to Joy* as a house track or *When the
Saints* as Dixieland. The [Library](../step-8/library.md) chapter
explains how the tables work; the [Demos](demo-search.md) chapter
explains how to search them.

## Running it and testing it

From the repository:

```sh
cargo run --release
```

Use `--release`: the audio and video code is far too slow in a debug
build. `cargo run --release -- --demos` lists every demo, and
`cargo run --release -- song.lineup` opens a project directly.

```sh
cargo test --workspace
```

The tests run without a display or a sound card. The UI tests draw the
app headless with `Audio::Unavailable`, so they work on CI. The project
also runs `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings`, then builds the
release binary, as [Tests and continuous integration](../step-1/ci.md)
describes.

## The book's appendices

The chapters are the course; the back of the book is for looking things
up. The [Rust glossary](../glossary.md) and the
[music glossary](../music-glossary.md) define the terms, with links to
the chapters. The
[project file format](../appendix-project-format.md),
[keyboard shortcuts](../appendix-shortcuts.md) and
[troubleshooting](../troubleshooting.md) are references. The
[Roadmap](../roadmap.md) is the status table and the ideas that have not
been built.

It is also online and as one PDF, made from the same Markdown by the
workflow in [The book online and as a PDF](book-online.md).

## What remains

The roadmap is careful to say that none of its ideas is planned yet.
Some are small; some would change the shape of the app:

- a soloist that is no longer an experiment;
- automation, drawn or recorded over the tracks;
- audio clips on the tracks, to play along with a recording;
- recording in song mode.

There is also the honest gap from step 7: MIDI was built on a computer
with no MIDI hardware, so real keyboards and synths were tested by other
means. The app has grown a great deal since; the gap is still worth
closing.

Ferrous Lineup is free software under the GNU General Public License,
version 3 or later. The band is yours to change: every style, rhythm and
song in the library is a line in a table, and every chapter points at
the code that makes it.
