# More demos: songs for every genre

Files: [`crates/loom-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/library/song.rs),
the genre files in [`crates/loom-core/src/library/song/`](https://github.com/naelolaiz/ferrous-loom/tree/main/crates/loom-core/src/library/song)
and [`crates/loom-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/app/library.rs)

The first demos were 16 songs. They were good to hear, but most of the
library's genres had no song at all: there was a tango *style*, but no
tango with a melody to learn from. Now **File > Demos > Songs** has 62
songs, with a submenu for each genre and at least two songs in every
one.

## Where the songs come from

Most are tunes in the **public domain**, chosen so each genre gets
something to listen to:

| Genre | Songs |
|---|---|
| Pop | Auld Lang Syne, London Bridge, Old MacDonald, Deck the Halls |
| Rock | Drunken Sailor, Bella Ciao, Beethoven's Fifth, Yankee Doodle |
| Blues | Midnight Shuffle\*, Rolling Boogie\*, Minor Mood Blues\* |
| Jazz | Jingle Bells, When the Saints, The Entertainer, Happy Birthday, Two-Five Walk\* |
| Soul, funk and R&B | Amazing Grace, Joy to the World, Dorian Strut\*, Sunday Seventh\* |
| Hip hop and lo-fi | Twinkle Twinkle, Gymnopédie, Prelude in C, Sakura Sakura, Dies Irae |
| Electronic | Ode to Joy, Korobeiniki, Symphony No. 40, Eine kleine Nachtmusik, Toccata in D minor, Turkish March |
| Latin | La Cucaracha, Rio Afternoon\*, Carnival Morning\* |
| Tango | Habanera, Tango del telar\*, Milonga del río\*, Vals del puerto\* |
| Flamenco | Rumba del sur\*, Noche frigia\* |
| Reggae and ska | Mary Had a Little Lamb, This Old Man, Valley Dub\* |
| Country and folk | Frère Jacques, Greensleeves, Oh! Susanna, Scarborough Fair, Clementine, Camptown Races, Row Row Row Your Boat, Silent Night, We Wish You a Merry Christmas |
| Classical and film | Canon in D, Für Elise, Mountain King, Minuet in G, The Blue Danube, Brahms' Lullaby, Morning Mood, New World Largo, Shchedryk, Funeral March |

The ones marked \* say "Written for Ferrous Loom" in their tooltip.
Some genres have very few famous tunes old enough to be free: the great
tangos, bossa novas and blues songs mostly date from the 1910s on, and
their composers died too recently. Rather than leave those genres
empty, they got new tunes, written the way those genres write them:

- **Over the style's own chords.** *Tango del telar* uses the "Tango"
  style's chart, `i | V7 | V7 | i | iv | i | V7 | i`, *Dorian Strut*
  the "Funk vamp", *Noche frigia* the "Phrygian flamenco". Open the
  song and the style side by side: the harmony is the same, the song
  adds a tune.
- **Chord notes on the strong beats.** A tune sounds "right" when the
  notes that land on the beat belong to the chord under them. The notes
  between are free to pass through: in *Midnight Shuffle* the blues
  scale's G and C clash with the A7 chord on purpose, which is the sound
  of the blues.
- **The genre's rhythm in the tune.** *Milonga del río* plays the
  milonga's 3 + 3 + 2 in its melody too: `E5:3 E5 G5:2 C6:8`, two
  dotted eighths, an eighth, then a long note.

## One file per genre

62 songs would make `song.rs` thousands of lines long. So the songs
moved to a folder, one file per genre, next to `song.rs`:

```text
library/
  song.rs          the Song and Part types, SONGS, the tests
  song/
    blues.rs
    classical.rs
    ...
    tango.rs
```

A file `song.rs` and a folder `song/` with the same name work
together: `mod tango;` inside `song.rs` makes Rust look for
`song/tango.rs`. Each genre file is a **child module** of `song`.

### Each song is a `const`

In a genre file, every song is a constant:

```rust
pub(super) const HABANERA: Song = Song {
    name: "Habanera",
    origin: "Georges Bizet, Carmen, 1875",
    feel: "Habanera",
    genre: Genre::Tango,
    tonic: D,
    mode: NaturalMinor,
    ...
};
```

`pub(super)` makes it visible to the parent module, `song`, and no
further: nothing outside the library needs to name `HABANERA`, only to
find it in `SONGS`. Then `song.rs` lists them all:

```rust
pub static SONGS: &[Song] = &[
    pop::AULD_LANG_SYNE,
    pop::LONDON_BRIDGE,
    ...
    classical::FUNERAL_MARCH,
];
```

Why not have each file make its own list and join them? Because
`SONGS` is a `static`, built when the program is compiled, and Rust has
no way to glue slices together at compile time. Listing each `const`
by name is the plain way to do it. A `const` is copied in wherever it is
used, so writing `tango::HABANERA` in the list puts the song itself
into `SONGS`; the `static` then has one fixed place in memory, which is
why the menu can hand out `&'static Song` references to it.

### `use super::*` and what children can see

Each genre file starts with one line:

```rust
use super::*;
```

`super` is the parent module, `song`, and `*` is a **glob import**: it
brings in every name the parent can see. That includes `Song`, `Part`,
`Comping`, `Bass` and `Genre`, the time signatures, and also the
parent's *private* items, such as `const COMMON` and its own `use`
lines. A private item in Rust is private to its module **and that
module's children**, so a child can always use what its parent keeps
to itself.

The parent's `use` lines matter here:

```rust
use PitchClass::*;
use ScaleKind::*;
```

They let a song write `tonic: A` and `mode: Dorian` instead of
`PitchClass::A` and `ScaleKind::Dorian`. `Genre` is not imported that
way, on purpose: `Genre::Blues` and `ScaleKind::Blues` would both be
called `Blues`, and two glob imports with the same name are an error
as soon as you use it. So genres are always written in full,
`genre: Genre::Blues`, and `mode: Blues` means the scale.

## The menu, by genre

Each song now carries its `genre`, so the Songs menu does what the
Styles menu already did: a submenu per genre, with the songs of that
genre inside.

```rust
for genre in Genre::ALL {
    ui.menu_button(genre.name(), |ui| {
        for song in SONGS.iter().filter(|song| song.genre == genre) {
            ...
        }
    });
}
```

`filter` walks all 62 songs once per genre, 13 times in all. That is
806 comparisons of two small enums per frame while the menu is open,
far too few to matter, and it keeps one list as the only truth.

## Tests for the new data

The tests from [the library](library.md#is-it-right) already check
every bar of every melody and that melodies stay mostly in key, so all
46 new songs were checked the moment they were written. Three new
checks came with them:

- **every genre has at least two songs**, so no submenu is ever empty,
- **`SONGS` is sorted by genre**, so its order matches the menu:

  ```rust
  SONGS.windows(2).all(|pair| pair[0].genre <= pair[1].genre)
  ```

  `windows(2)` gives every pair of neighbours, `[a, b]`, `[b, c]` and
  so on. `<=` works on genres because `Genre` derives `PartialOrd`,
  which orders an enum's variants in the order they are written,
- **each song's drums are in its time signature**: a waltz song asking
  for a 4/4 rhythm would play a bar of drums that does not line up
  with its bars, which no other test would notice.
