# South American folklore, and many more songs

The second batch of demos does two things. It adds a genre that was
missing, the folklore of South America, with its own drum patterns and
harmony styles, and it more than doubles the number of songs, from 62
to 130, every new one transcribed from a written source rather than from
memory. Along the way a demo learnt to show, in the Library tab, which
style and rhythm it is playing. This page is about what had to change to
make room for all of it, and what was learnt on the way.

## A new genre

`Genre` is an enum, and the whole library hangs off it: the Styles,
Rhythms and Songs menus each have a submenu per genre, in the order of
`Genre::ALL`. Adding `Folklore` between `Tango` and `Flamenco` touched
three places:

```rust
pub enum Genre {
    ...
    Tango,
    Folklore,
    Flamenco,
    ...
}

pub const ALL: [Genre; 14] = [ ... Genre::Tango, Genre::Folklore, Genre::Flamenco, ... ];

pub fn name(self) -> &'static str {
    match self {
        ...
        Genre::Folklore => "South American folklore",
        ...
    }
}
```

The array's length is part of its type, `[Genre; 14]`, so the compiler
refuses the old `13` the moment a variant is added, and `match` must
cover every variant, so `name` cannot be forgotten either. That is the
enum doing its job: there is no way to add a genre and leave the menus
half-built.

The tests then asked for the rest. `every_genre_has_songs_and_they_are_in_order`
failed until the new genre had at least two songs, and because `SONGS`
must be sorted in enum order, the folklore songs had to go between the
tangos and the flamenco. The position of a variant in the enum is its
position everywhere.

## Two against three, with no time markers

Nearly all of this music is in 6/8 and keeps crossing into 3/4. A
chacarera's guitar strums two dotted-quarter beats to the bar while the
bombo plays three quarters; a chamamé's accordion is in 6/8 over a bass
in 3/4; the joropo's maracas and harp disagree on purpose. This is the
*hemiola* (or *sesquiáltera*), and it is the whole flavour.

It looked at first like a job for [time markers](time-markers.md), but
it is not. Both 6/8 and 3/4 are twelve steps long:

```rust
pub fn steps_per_bar(self) -> usize {
    (usize::from(self.beats) * 16 / unit).max(1)
}
```

6 × 16 / 8 = 12, and 3 × 16 / 4 = 12. A twelve-step bar can be heard
as two groups of six or three groups of four, and the drum lines can do
both at once:

```rust
rhythm(
    "Chacarera",
    Folklore,
    132.0,
    &[
        (LowTom, "X..... x.x..."),
        (Rim, "....x. ..x.x."),
    ],
    "",
    "...",
)
.in_time(SIX_EIGHT),
```

The low tom is the bombo's head, on the two 6/8 beats; the rim is the
stick on its wooden hoop, on the second and third beats of 3/4 (steps
4 and 8) and the last eighth. Nothing in the engine knows that two
metres are being played; it just plays twelve steps. The space in the
middle of each line is for the reader, not the parser.

The bombo legüero has no sample of its own. `Drum` has thirteen
variants and adding one means a new sample, a new menu entry and a new
file format version, so the folklore patterns borrow: `LowTom` for the
head, `Rim` for the hoop, `Shaker` for the chajchas and maracas, `Clap`
for the cueca's palmas, `Conga` for the candombe's chico. The point of a
pattern is where the hits fall, and the ear fills in the rest.

## A comping that fits in twelve steps

The first draft of the folklore styles strummed, with `Comping::Strum`,
and a test in `apply.rs` refused it:

```
Chacarera is in three but plays Strum
```

`Strum` is written as a sixteen-step pattern, a bar of 4/4. In a
twelve-step bar it would be cut off a beat early, and the cut would
fall somewhere different in 3/4 and in 6/8. The test, which came with
[the rhythm re-comp](new-rhythm.md), only allows the compings that were
written for three:

```rust
assert!(
    !three || matches!(comping, Comping::Waltz | Comping::Sway | Comping::Arp3 | Comping::Held),
    "{} is in three but plays {comping:?}",
    rhythm.name
);
```

`matches!` is a macro that returns `true` when a value fits a pattern;
here it does in one line what a four-arm `match` would. So the styles
in 6/8 play `Sway` (one chord on beat 2, held), `Waltz` (chords on
beats 2 and 3 after the bass) or `Arp3` (up and down the chord in
eighths). Against the 6/8 drums, `Waltz` and `Sway` *are* the 3/4 side
of the hemiola, which is why the gato and the cueca use them.

## Letting a test choose

Every style has to pass `every_style_is_heard_as_its_chart`: played
in its own way, the chord hearing of [A new rhythm for a song](new-rhythm.md)
must get its chart back exactly. Four of the sixty styles written for
this batch failed, and so might any of the thirteen folklore ones. Rather
than guess, a temporary test tried every pairing of the 26 compings and
21 basses for each chart and printed the ones the hearing got wrong:

```
== Slow blues: 80 BAD: ArpUp/Pulse ... Arp3/Waltz ...
== Bachata: 40 BAD: ... ArpUpDown/Tresillo ...
== Cha-cha-chá: 94 BAD: Montuno/None Montuno/Held ... Montuno/Tumbao ...
== Joropo: 15 BAD: ...
== Vidala: 0 BAD:
```

The failures were all of one kind: arpeggios and montunos spread a
four-note chord so thinly over the bar that a stretch of it holds only
three notes, and the hearing names those three. So the slow blues plays
`Pulse` (triplet chords in 12/8), the bachata arpeggiates upwards only,
the cha-cha-chá plays stabs, and the son montuno keeps its montuno but
over a `Tresillo` bass instead of the tumbao, whose anticipation of the
next root was being heard as a suspended chord. The temporary test was
deleted afterwards. A test that runs once and tells you something is
still a test worth writing.

## Twelve-eight

The slow blues wanted a bar of four beats each split in three, and there
was no time signature for it. `TimeSignature::new(12, 8)` already
worked, 12 × 16 / 8 = 24 steps, so the only change was a constant in
`style.rs` and `rhythm.rs`, beside the others:

```rust
const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);
```

A melody in 12/8 writes an eighth as 2 steps and a bar as 24. The
piano roll and the playlist know nothing new: a bar is as long as the
time signature says.

## Transcribed, not remembered

The first batch taught a hard lesson, written up on
[More demos](more-demos.md#true-to-the-score): tunes written
from memory land a beat early, lose notes and get chords wrong. For this
batch every song came from a written source, an ABC, LilyPond or
MusicXML transcription or an engraved score, and the source is noted in
a comment above each song. Two rules decided what could go in:

- **Public domain.** A song is in if it is traditional, or if every
  composer died before 1956 and it was first published before 1929.
  That lets in the guardia vieja of the tango (El Choclo, La
  Cumparsita, Gallo ciego), the spirituals and gospel hymns, Schubert,
  Verdi, Grieg, Mussorgsky, Debussy, Rachmaninoff and Holst, and El
  cóndor pasa and Alma llanera, and keeps out most of what was recorded
  after 1930.
- **Checked before it was heard.** A small script, outside the
  repository, reads a file of `Song` consts and checks what the unit
  tests will check: every bar adds up to its time signature, the melody
  has as many bars as its chart, every form names a real part, and at
  least 80% of the notes sit in the part's key. The notes themselves
  are checked against the source by whoever transcribes them. The
  script finds the slips of the pen; the source finds the slips of
  memory.

A rule that strict leaves holes. Zamba de Vargas, the classic
chacareras and most of the cuecas are traditional, but no written
transcription of them could be reached to check against, and writing
them down from memory is exactly the mistake the first batch made. So
each dance without a checked source got a new tune instead, written for
Ferrous Lineup and marked as such in its `origin`: Zamba del telar,
Chacarera del patio, Gato del arroyo and eight more. They are short and
plain on purpose, so that the rhythm and the harmony of the dance are
what you hear.

## The songs

The new demos, by genre in the Songs menu:

- **South American folklore**: El cóndor pasa and Dos palomitas
  (huayno), Alma llanera (joropo), and the eleven new dance tunes.
- **Tango**: El Choclo, La Cumparsita, La Paloma (a habanera) and Gallo
  ciego.
- **Soul, funk and R&B**: eighteen spirituals and gospel hymns, from
  Swing Low, Sweet Chariot and Go Down, Moses to O Happy Day.
- **Country and folk**: The Wild Rover, Shenandoah, Danny Boy, Loch
  Lomond, Waltzing Matilda, Soldier's Joy, Cotton-Eyed Joe and more.
- **Rock and pop**: a few old tunes played the modern way: Whiskey in
  the Jar, When Johnny Comes Marching Home, Bizet's Toreador Song as a
  rock march.
- **Classical and film**: La donna è mobile, Ave Maria, the Hallelujah
  Chorus, the Wedding March, The Great Gate of Kiev, Anitra's Dance,
  Rachmaninoff's Prelude in C-sharp minor, Holst's Jupiter hymn and
  Mozart's Dies irae; Debussy's Rêverie turns up as lo-fi hip hop and
  his Golliwogg's Cakewalk as jazz.

Each song's `about` says what to listen for: where the modulation is,
which chord is borrowed, where the hemiola falls.

## A demo opens with its style and rhythm selected

In a good browser, whatever you load stays highlighted, so you can see
where it came from and try the one next to it. Here, opening a
demo used to leave the Library tab with nothing selected. The library
only highlighted what it had itself written into the selected pattern,
and a demo is built by `song_demo`, not by a click in the library.

The fix starts with data. A `Song` already named its rhythm; now it also
names the harmony style it is closest to:

```rust
pub struct Song {
    ...
    pub rhythm: &'static str,
    /// The style in [`super::STYLES`] whose harmony and playing the song
    /// is closest to, or "" for none. The library shows it, and the
    /// rhythm, selected while the demo is open.
    pub style: &'static str,
    ...
}
```

A name in a string can be misspelt, so a test looks every one up, and
checks the time signature while it is there, since a 6/8 song should
not point at a 4/4 style:

```rust
#[test]
fn every_song_names_a_real_style() {
    for song in SONGS {
        if song.style.is_empty() {
            continue;
        }
        let style = crate::library::Style::named(song.style)
            .unwrap_or_else(|| panic!("{}: no style called {}", song.name, song.style));
        assert_eq!(style.time, song.time, "{}: {} is in another time", song.name, song.style);
    }
}
```

On the app side, `Tried` is the pair the library highlights: an
`Option<&'static Style>` and an `Option<&'static Rhythm>`. The app keeps
one more of them, `song_library`, for the song as a whole, and `Demo`
says what to put there:

```rust
fn library(self) -> Tried {
    match self {
        Demo::Start => Tried::default(),
        Demo::Song(song) => Tried {
            style: Style::named(song.style),
            rhythm: Rhythm::named(song.rhythm),
        },
        Demo::Style(style) => Tried {
            style: Some(style),
            rhythm: Rhythm::named(style.rhythm),
        },
    }
}
```

`Style::named` returns an `Option`, so an empty name simply gives
`None`, and nothing is selected. Opening a demo sets
`self.song_library = demo.library()`, and opening any other file sets it
back to `Tried::default()`, two `None`s.

What the library shows is then a `match` with a fallback:

```rust
pub(super) fn tried(&self) -> Tried {
    match self.unchanged_audition() {
        Some(audition) => Tried {
            style: audition.style,
            rhythm: audition.rhythm,
        },
        None => self.song_library,
    }
}
```

When a library click changes the whole song while it plays, the song's
selection follows. Only the half that changed is replaced, with
*struct update syntax*: `..song_library` copies every field not named
before it.

```rust
self.song_library = match action {
    LibraryAction::Style(style) => Tried {
        style: Some(style),
        ..song_library
    },
    LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) if !fill => {
        Tried {
            rhythm: Some(rhythm),
            ..song_library
        }
    }
    _ => song_library,
};
```

`Tried` is `Copy` (two shared references are cheap to copy), so
`song_library` can be used again after `..song_library` without being
moved out.

### Showing it

A highlight inside a closed genre is no highlight at all, and every
genre starts closed. So the library browser remembers the selection it
drew last, `shown`, and when it changes it opens the genre that holds
the new style or rhythm and scrolls it into view:

```rust
let new_style = tried.style.is_some() && tried.style != self.shown.style;
let new_rhythm = tried.rhythm.is_some() && tried.rhythm != self.shown.rhythm;
self.shown = tried;
```

egui's `CollapsingHeader::open(Some(true))` opens a header on this frame
only; after that the user can close it again, which is what we want.
`Response::scroll_to_me(None)` asks the surrounding `ScrollArea` to
scroll just enough to show a widget, and does nothing if it is already
visible. A header opens with a short animation, though, and during it
the list is not yet tall enough to scroll to the bottom of. So the
browser keeps asking for half a second:

```rust
if new_style || new_rhythm {
    self.reveal_until = now + 0.5;
    self.reveal_rhythm = !new_style;
}
let revealing = now < self.reveal_until;
```

`reveal_rhythm` decides where to scroll when both changed: to the
style, which comes first in the list. When only the rhythm changed, as
after a click on a rhythm, the list scrolls to the rhythm, so the view
does not jump away from where the user just clicked.

### Undo

Ctrl+Z takes a library click back, but the undo history holds only
`Project`s, and `song_library` is not part of the project. After undo
the old drums were back but the new rhythm was still highlighted. The
fix keeps, beside the history, a short list of the project versions the
library made and the selection that goes with each:

```rust
song_versions: Vec<(Project, Tried)>,
```

A library click remembers the project before it with the old selection,
and the project after it with the new one. After undo or redo, the app
looks for the version it is now on:

```rust
pub(super) fn restore_song_library(&mut self) {
    if let Some((_, tried)) = self
        .song_versions
        .iter()
        .rev()
        .find(|(version, _)| *version == self.project)
    {
        self.song_library = *tried;
    }
}
```

`Project` derives `PartialEq`, so a whole project can be compared with
`==`, the same comparison `edit` uses to decide whether anything
changed. `iter().rev()` searches from the newest end, and the pattern
`|(version, _)|` takes the tuple apart right in the closure's
arguments. If the version is not in the list (an edit made by hand
since), the selection stays as it is, which is right: hand edits do not
change the style.
