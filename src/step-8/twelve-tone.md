# Twelve-tone rows and pitch-class sets

Files: [`crates/lineup-core/src/harmony/row.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/row.rs),
[`crates/lineup-core/src/harmony/set_class.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/set_class.rs),
[`crates/lineup-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/pitch_set.rs),
[`crates/lineup-core/src/serial.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/serial.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/library/rows.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rows.rs)
and [`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs)

Everything in Ferrous Lineup so far assumed music in a key: a home note,
chords that lean towards it, harmony keys numbered I to vii°. A large
part of the music of the 20th century has no key at all. This chapter
adds the two main tools musicians use to think about it:

- **twelve-tone rows** (*dodecafonismo* in Spanish, also called
  *series*): all twelve notes in an order that the whole piece is made
  of. The key menus now offer **twelve-tone row** after the scales, and
  **Row…** in the piano roll opens a window to type, draw or pick a
  row, see its **matrix** with all 48 forms, and write any form into
  the pattern, as a melody or as chords, with the durations, dynamics
  and octaves following the row too if you like (**serialism**);
- **pitch-class sets**: a name for every group of notes, chord or not.
  **Sets…** lists all 224 of them by **Forte number**, to draw with or
  to use as a scale, and the piano roll now names the set of the notes
  you select and of the notes under the pointer.

The Library gets a group of **Twelve-tone rows** styles, and **File >
Demos > Twelve-tone** plays each of them as a short piece.

![The Row window beside the piano roll. The row is C E G D F# A A# C# F B D# G#, four triads in a row; the matrix shows its 48 forms, with P0 selected. The piano roll numbers its notes 1 to 12 on the keys, and the harmony keys play its four trichords.](../images/step-8-twelve-tone-row.png)

![The Sets window, searching for "octatonic": set 8-28, its prime form, its interval vector, and what its complement is. Below the piano roll's toolbar, the notes at the step under the pointer are named as set 3-11.](../images/step-8-pitch-class-sets.png)

## What FL Studio does, and what we do

FL Studio's piano roll knows scales and chords, but not rows or sets.
Its **Flip** tool turns a selection upside down or back to front, which
is an inversion or a retrograde done by hand, and that is as close as
it gets. So this chapter is not a copy of anything: it is the kind of
help a student of twelve-tone music would want next to a piano roll,
built on the same pieces as the rest of the app (scales, the harmony
keys, chord stamps, the library).

## Music with no home note

In C major, C is home. G pulls towards it, B pulls hard, and a piece
that ends on C sounds finished. Around 1921 Arnold Schoenberg looked
for a way to write music in which no note is home. His answer was to
use all twelve notes equally, in a fixed order that he chose for each
piece: the **row**. No note can come back until the other eleven have
sounded, so none of them gets to be the centre.

This is the row of Schoenberg's Suite for Piano, Op. 25, from 1925, one
of the first pieces built entirely from one:

```text
E  F  G  C#  F#  D#  G#  D  B  C  A  A#
```

A row would be dull if it could only be played one way, so it comes in
four:

| form | how | Schoenberg's row on E |
|------|-----|------------------------|
| **P**, prime | as written | E F G C# F# D# G# D B C A A# |
| **I**, inversion | every interval upside down: where P goes up a semitone, I goes down one | E D# C# G D F C F# A G# B A# |
| **R**, retrograde | P backwards | A# A C B D G# D# F# C# G F E |
| **RI**, retrograde inversion | I backwards | A# B G# A F# C F D G C# D# E |

Each of the four can start on any of the twelve notes, which gives
**48 forms**. They are named by a letter and a number: the number is
the note the form starts on, with C as 0, C# as 1, up to B as 11. So
the row above is **P4** (it starts on E), and **I0** is its inversion
starting on C. A retrograde gets the number of the form it reverses:
**R4** is P4 backwards, so it *ends* on E.

Some books number the forms differently, from the row's own first
note, so that the row as written is always P0. Then Schoenberg's row is
P0 and what Ferrous Lineup calls I11 is I7 there. The forms are the same;
only their numbers move. Ferrous Lineup uses the first way (the note a
form starts on), because then a label tells you a note you can find
on the keyboard.

### The matrix

All 48 forms fit in one 12×12 square. The row goes along the top, its
inversion from the same note down the left side, and each line *i* is
the row moved to start on the note at the left of that line:

```text
      I4  I5  I7  I1  I6  I3  I8  I2  I11 I0  I9  I10
P4    E   F   G   C#  F#  D#  G#  D   B   C   A   A#    R4
P3    D#  E   F#  C   F   D   G   C#  A#  B   G#  A     R3
P1    C#  D   E   A#  D#  C   F   B   G#  A   F#  G     R1
P7    G   G#  A#  E   A   F#  B   F   D   D#  C   C#    R7
...
      RI4 RI5 RI7 RI1 ...
```

Read a line left to right and it is a P form; right to left, an R form.
Read a column top to bottom and it is an I form; bottom to top, an RI
form. The labels around the square say which: P on the left, R on the
right, I along the top, RI along the bottom. In the Row window, click
any label to choose that form, and its line or column lights up.

The diagonal from top left to bottom right is the same note all the
way down (E, here). That is a quick check that a matrix was filled in
correctly.

## A row in Rust: `ToneRow`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ToneRow([u8; 12]);
```

A row is twelve numbers, the semitones above C of its notes. That is a
**newtype** around an array: the array is private, so the only ways to
make a `ToneRow` are functions that check it:

```rust
pub fn new(semitones: [u8; 12]) -> Option<Self> {
    let set: PitchSet = PitchSet::from_semitones(&semitones);
    (set.len() == 12 && semitones.iter().all(|&n| n < 12)).then_some(ToneRow(semitones))
}
```

Collect the twelve numbers into a set of pitch classes (below), and if
the set has twelve members, no note was repeated. Every `ToneRow` in
the program is therefore a real row: the **invariant** is checked once,
here, and nothing else has to worry about it.

### Files: `try_from` and `into`

The two serde attributes on the struct say: "to save a `ToneRow`, turn
it into a `String`; to read one, read a `String` and try to turn it
into a `ToneRow`". The conversions are ordinary trait implementations:

```rust
impl TryFrom<String> for ToneRow {
    type Error = String;

    fn try_from(text: String) -> Result<Self, String> {
        ToneRow::parse(&text)
    }
}

impl From<ToneRow> for String {
    fn from(row: ToneRow) -> Self {
        let numbers: Vec<String> = row.0.iter().map(u8::to_string).collect();
        numbers.join(" ")
    }
}
```

So a project file holds `"4 5 7 1 6 3 8 2 11 0 9 10"`, which a person
can read, instead of a list of twelve numbers in brackets. And because
reading goes through `parse`, a file with a note written twice is
refused with a message, rather than producing a broken row. `PitchSet`
gets the same pair, so a set is saved as `"0 3 6 9"`.

`parse` is also what reads the box at the top of the Row window. It
takes note names (`E F G Db Gb Eb Ab D B C A Bb`), numbers (`4 5 7 1 …`)
or the compact form twelve-tone books use, one character a note with
**T** and **E** for ten and eleven (`45716382E09T`). When a note is
missing it says which, and which one came twice instead.

**From the notes** reads a row from the piano roll instead: the first
twelve different pitch classes among the selected notes (or all of
them), in the order they start. `ToneRow::from_notes` skips a note
whose pitch class is already in the row, so you can draw freely and
repeat notes, as long as all twelve turn up.

### Every form from one function

```rust
pub fn form(self, label: RowLabel) -> ToneRow {
    let first = self.0[0];
    let n = label.number;
    let mut notes = self.0.map(|note| {
        let up = (note + 12 - first) % 12;
        if label.form.is_inverted() {
            (n + 12 - up) % 12
        } else {
            (n + up) % 12
        }
    });
    if label.form.is_backwards() {
        notes.reverse();
    }
    ToneRow(notes)
}
```

Each note is first measured as `up`, its distance above the row's first
note (0 to 11). A P form puts it the same distance above the new start
`n`; an I form puts it the same distance *below*. That is all an
inversion is. The `+ 12` before each `%` keeps the numbers from going
below zero: they are `u8`, and `4 - 7` would not wrap around the octave
but stop the program in a debug build.

`self.0.map(...)` is the `map` of an **array**, not of an iterator: it
calls the closure on each of the twelve items and gives back a new
`[u8; 12]`, with no `Vec` and no `collect`. The length is part of the
type, so the result is still exactly twelve notes.

With `form`, the matrix is two lines:

```rust
pub fn matrix(self) -> [[u8; 12]; 12] {
    let inversion = self.form(RowLabel::new(RowForm::Inversion, self.0[0]));
    inversion
        .0
        .map(|start| self.starting_on(PitchClass::from_semitone(start)).0)
}
```

The inversion from the row's first note is the left column; each of
its notes starts one line.

And going the other way, "which form of this row is that?", tries all
48 until one matches:

```rust
pub fn label_of(self, other: ToneRow) -> Option<RowLabel> {
    RowForm::ALL.into_iter().find_map(|form| {
        (0..12)
            .map(|n| RowLabel::new(form, n))
            .find(|&label| self.form(label) == other)
    })
}
```

## What makes a row interesting

The Row window lists what is special about the row you are looking at.
Each line is a short function.

**The steps.** `intervals()` gives the eleven steps from note to note,
in semitones going up. For Schoenberg's row they are
`1 2 6 5 9 5 6 9 1 9 1`. It is built with `std::array::from_fn`, which
makes an array by calling a closure with each index:

```rust
pub fn intervals(self) -> [u8; 11] {
    std::array::from_fn(|i| (self.0[i + 1] + 12 - self.0[i]) % 12)
}
```

**All-interval rows.** When the eleven steps are all different, every
interval from a semitone to a major seventh appears exactly once. Alban
Berg's Lyric Suite (1927) starts with such a row: F E C A G D G# C# D#
F# A# B, with the tritone right in the middle. They are rare: of the
39,916,800 rows that start on C, only 3,856 are all-interval. So the
**All-interval** button cannot just shuffle notes until it finds one;
it builds the row a note at a time:

```rust
fn all_interval_from(notes: &mut Vec<u8>, random: &mut Random) -> Option<Vec<u8>> {
    if notes.len() == 12 {
        return Some(notes.clone());
    }
    let used: Vec<u8> = notes.windows(2).map(|w| (w[1] + 12 - w[0]) % 12).collect();
    let mut intervals: Vec<u8> = (1..12).filter(|i| !used.contains(i)).collect();
    // ... shuffle `intervals` ...
    let last = *notes.last()?;
    for interval in intervals {
        let next = (last + interval) % 12;
        if notes.contains(&next) {
            continue;
        }
        notes.push(next);
        if let Some(row) = all_interval_from(notes, random) {
            return Some(row);
        }
        notes.pop();
    }
    None
}
```

This is **backtracking**: try each step not used yet, in a random
order; if the note it lands on is free, add it and carry on from there
(the function calls itself); if that leads nowhere, take the note back
(`pop`) and try the next step. When no step works, return `None` so the
caller takes back *its* note. `windows(2)` hands out every pair of
neighbours, `[a, b]`, which is exactly what a step is.

**Random rows.** The **Random** button shuffles the twelve notes with
the Fisher-Yates shuffle: walk from the last place to the first, and
swap each with a place chosen at random among those not done yet.
Every order is then equally likely. The random numbers come from a
tiny generator, *SplitMix64*, written out in fifteen lines instead of
adding a library:

```rust
fn next(&mut self) -> u64 {
    self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = self.0;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
```

`wrapping_add` and `wrapping_mul` say "overflowing is fine here, just
wrap around": a plain `+` that overflows would stop a debug build.
The same seed always gives the same row, which makes the tests
repeatable.

**Groups of notes.** Composers often play a row in groups: four
trichords (three notes each), three tetrachords, two hexachords.
`segments(size)` cuts the row with `chunks_exact`, and the window names
each group's set class (see the second half of this chapter). The
groups of Schoenberg's row are three different tetrachords, 4-12, 4-Z15
and 4-1; the Four triads row is four groups of 3-11, the major and
minor triads.

**Combinatoriality.** Take the first six notes of a row. Some other
form of the same row may start with exactly the *other* six. Play the
two forms at the same time, and each half of the music holds all twelve
notes, without one repeating early. Schoenberg built his later pieces
on this, and it is called **hexachordal combinatoriality**:

```rust
pub fn combinatorial_forms(self) -> Vec<RowLabel> {
    let missing = PitchSet::from_semitones(&self.0[..6]).complement();
    [RowForm::Prime, RowForm::Inversion, RowForm::RetrogradeInversion]
        .into_iter()
        .flat_map(|form| (0..12).map(move |n| RowLabel::new(form, n)))
        .filter(|&label| PitchSet::from_semitones(&self.form(label).0[..6]) == missing)
        .collect()
}
```

The retrograde is left out because it always works (its first half is
the row's second half), so it says nothing about the row. Schoenberg's
Op. 25 row is combinatorial with I11; the Lineup row, written for this
chapter, with I3. The **Hexachord pads** style plays exactly that: one
half of the row as a chord, and the other six notes from I3 as a
melody over it.

The `move` in `move |n|` is needed because the inner closure is handed
out of the outer one by `flat_map`, and it uses `form`: `move` makes it
take its own copy of `form` instead of borrowing a variable that is
about to go away.

### The rows to start from

The **Famous and example rows** menu holds seven rows. Three come from
famous pieces, and four were written for Ferrous Lineup to show what a
row can do: a combinatorial one, a row of four triads, a palindrome and
an all-interval row.

| row | from | what it shows |
|-----|------|---------------|
| E F G C# F# D# G# D B C A A# | Schoenberg, Suite Op. 25 (1925) | one of the first twelve-tone pieces; B C A A# spells BACH in German note names, scrambled |
| F E C A G D G# C# D# F# A# B | Berg, Lyric Suite (1927) | an all-interval row |
| A F# G G# E F B A# D C# C D# | Webern, Symphony Op. 21 (1929) | a palindrome: backwards, it is the same row a tritone higher (R9 is P3) |

All three were published before 1930, so they are in the public domain
in the United States, and their composers died in 1951, 1935 and 1945,
so they are in the public domain where copyright lasts 70 years after
the author's death too. And only the rows are used: twelve notes each.

Webern's row shows that a row does not always have 48 *different*
forms. Since its R9 is its P3, every R form is also a P form, and every
RI form an I form: only 24 are different. `label_of` tries the P forms
first, so such a form is named as the P form it also is.

## From a row to notes: serialism

A row says which note comes next, and nothing else. To make music of
it, someone still has to decide how long each note is, how loud, in
which octave, and whether some notes sound together. `RowWriting` holds
those choices, and the controls under the matrix set them:

```rust
pub struct RowWriting {
    pub label: RowLabel,       // the form to start with, such as P0
    pub chain: Chain,          // then: once, repeat, or P, R, I, RI in turn
    pub together: usize,       // 1 for a melody, 3 for trichords, 6 for hexachords
    pub step: usize,           // how long each note or chord lasts, in steps
    pub register: Register,    // close, smooth or wide
    pub low: Note,             // the lowest note
    pub durations: bool,       // serialism: durations follow the row
    pub dynamics: bool,        // dynamics follow the inversion
    pub octaves: bool,         // octaves follow the retrograde
}
```

The three **registers** decide the octaves when the row does not:

- **Close** puts every note in the octave above the lowest note;
- **Smooth** puts each note as near as it can be to the one before,
  which makes a singable line (a tritone, exactly halfway, goes up);
- **Wide** sends the notes low, high, middle, low, high, middle over
  three octaves, the leaping lines of Anton Webern.

### Integral serialism

Around 1950, Olivier Messiaen, Pierre Boulez and Milton Babbitt took
the idea further: if the pitches follow a series, why not the
durations, the dynamics and the octaves too? This is **integral** (or
total) serialism. Real pieces map series to durations and dynamics in
many ways; Ferrous Lineup uses three simple ones, each reading a
different form of the row so that the four series do not move in step:

- **Durations**: the row's numbers (each note's distance above the
  first note, 0 to 11) become lengths. The *k*-th note lasts as many
  steps as the *k*-th number, plus one, so from 1 to 12 sixteenth notes.
- **Dynamics**: the numbers of the inversion become twelve velocities,
  from very soft to very loud: `28 + 9 * level`.
- **Octaves**: the numbers of the retrograde, counted in threes
  (`% 3`), pick one of three octaves.

Inside `write_row`, each note asks those questions in turn:

```rust
let lasts = if writing.durations {
    usize::from(numbers[place % 12]) + 1
} else {
    step
};
// ...
let octave = if writing.octaves {
    // The retrograde's numbers, in threes.
    Some(numbers[11 - k % 12] % 3)
} else if writing.register == Register::Wide {
    // Low, high, middle, and round again, against the row.
    Some([0, 2, 1][k % 3])
} else {
    None
};
// ...
let velocity = if writing.dynamics {
    // The inversion's numbers.
    dynamic((12 - numbers[k % 12]) % 12)
} else {
    crate::DEFAULT_VELOCITY
};
```

`place` counts notes across forms, so with **P, R, I, RI** the series
of durations carries on from one form into the next instead of starting
again.

`write_row` is a pure function: a row, the choices and a pattern length
in; a list of `NoteEvent`s out. The window calls it every frame to show
a warning when the pattern is too short for the whole row ("The pattern
only has room for 9 of the 12 notes"), and the **Write** button sends
the same notes to the piano roll, which replaces the channel's notes
through the usual edit, so **Ctrl+Z** brings the old ones back.

## A row as the key

The key menus now list two more kinds of key under the scales:
**twelve-tone row** and **pitch-class set**. In the code, they are two
new variants of `ScaleKind`, and unlike the others they carry data:

```rust
pub enum ScaleKind {
    Major,
    NaturalMinor,
    // ... the other scales ...
    Chromatic,
    /// A twelve-tone row, stored starting on C. The scale's root is the
    /// note it starts on.
    Row(ToneRow),
    /// A pitch-class set used as a scale: its notes as semitones above
    /// the root, the root among them.
    Set(PitchSet),
}
```

The row is stored starting on C, and the `Scale`'s root says where it
really starts, so "row P4" is `Scale { root: E, kind: Row(...) }` and
the root menu moves it like any other key. Both `ToneRow` (12 bytes)
and `PitchSet` (2 bytes) are `Copy`, so `ScaleKind` and `Scale` stay
`Copy` too, and none of the code that passes a `Scale` around had to
change.

Three things did change, and each says something about Rust:

- **`ScaleKind::ALL` no longer lists every kind.** A list of all rows
  would have 39,916,800 entries. `ALL` now lists the thirteen scales of
  tonal music, and the menus add the row and the set after a separator,
  with the row or set used last (kept in egui's memory, as in
  [Fitting any window](fitting-the-window.md)).
- **`intervals()` returns a `Vec<u8>` instead of a `&'static [u8]`.**
  The scales' notes were constant arrays stored with the program, so a
  `'static` borrow of them was free. A row's notes are only known while
  the program runs, inside a value that will go away; there is nothing
  `'static` to borrow, so the function has to hand out its own copy.
  The constant arrays moved to a private `tonal_intervals()`.
- **`is_tonal()`** answers "does this key have a home note?" with
  `matches!`. `is_heptatonic()`, which the chords of the key and the
  Roman numerals already asked before doing anything, now asks it too:
  a set of seven notes is not a major scale, even though it has seven
  notes.

### What the key changes

With a row as the key, the piano roll numbers its notes **1 to 12** on
the keys, in the row's order, so you can draw the row by following the
numbers. `degree_of` already gave a note's place in a scale; for a row,
the "scale" is the row in its own order, so the place in the row comes
for free.

The harmony keys play the row's groups instead of chords: four keys
for the trichords (**1–3**, **4–6**, **7–9**, **10–12**), or three for
tetrachords, or two for hexachords. The **3 notes / 4 notes / 6 notes**
choice takes the place of Triads, 7ths and 9ths. For a set, each key
stacks every other note of the set from one of its notes, the way the
chords of a key stack every other note of the scale.

### Changing key between kinds

[Changing key](changing-key.md) moves every note into the new key. From
C major to a row, though, where would E go? There is no sensible
answer, so `change_key` now asks first whether the two kinds have
anything in common:

```rust
pub fn notes_follow(self, other: ScaleKind) -> bool {
    match (self, other) {
        (ScaleKind::Row(a), ScaleKind::Row(b)) => a == b,
        (ScaleKind::Set(_), ScaleKind::Set(_)) => true,
        (a, b) => a.is_tonal() && b.is_tonal(),
    }
}
```

Matching on a **tuple** of two values handles every pair of kinds in
three lines: the same row on another note moves the notes (that is
transposing), two sets do too, two tonal keys do as before, and
anything else leaves the notes where they are and only changes what
the piano roll highlights.

### Format 6

A project whose key is a row or a set is saved as **format 6**. Older
versions of Ferrous Lineup do not know these kinds, so they could not
read the key; with the new number they say the file is from a newer
version instead of failing in a confusing way. As always, `format_for`
writes the lowest format a project needs, so projects in an ordinary
key are still readable by older versions. In the file, the key looks
like this:

```text
scale: (root: C, kind: Set("0 3 6 9")),
```

## Pitch-class sets

Chord names work for the chords of tonal music. Much music of the 20th
century uses groups of notes that have no chord name, like C, C# and
F#. **Pitch-class set theory**, worked out by Milton Babbitt, Allen
Forte, John Rahn and others, gives every group of notes a name by
asking what stays the same when you move it around.

**Pitch classes as numbers.** C is 0, C# is 1, up to B, 11. Octaves
and spellings do not count: every C is 0, and C# and Db are both 1. A
**set** is a choice of some of the twelve; there are 2¹² = 4,096 of
them, from the empty set to all twelve.

**Moving a set.** *Transposing* (T*n*) moves every note up *n*
semitones: C E G up 3 is Eb G Bb. *Inverting* (I) turns every interval
upside down: C E G (up 4, then up 3) becomes C Ab F (down 4, then down
3), which is F minor. Upside down, a major triad is a minor triad.

**Set classes.** A set together with all its transpositions and
inversions is a **set class**. All 24 major and minor triads are one
class. The 4,096 sets fall into **224 classes**.

### Normal form, prime form and Forte numbers

To name a class, every set in it must come out with the same name. Two
steps get there. Take C major seven, E G B C, as the example.

**Normal form** is the most compact way to write the set. Written
around a circle, a set can start on any of its notes. Start on each in
turn, and measure the span from the first note to the last:

| starting on | notes | span | second-to-last |
|-------------|-------|------|----------------|
| C | C E G B | 11 | |
| E | E G B C | 8 | 7 |
| G | G B C E | 9 | |
| B | B C E G | 8 | 5 |

E and B tie with a span of 8, so compare the span to the note before
last: B to E is 5, E to B is 7. The normal form is **B C E G**. (This
tie-break, comparing from the end, is John Rahn's; Allen Forte compared
from the start. They disagree for six classes. Ferrous Lineup uses Rahn's,
as most textbooks now do.)

**Prime form** goes one step further: take the normal forms of the set
*and of its inversion*, move both to start on 0, and keep the more
compact one. For Cmaj7, it is **[0158]**: B C E G moved down to start
on 0 is 0 1 5 8. Every major seventh chord, on any root, and its
inversion, has the prime form [0158].

**Forte numbers** list the classes by size, then in Forte's order:
[0158] is **4-20**, the twentieth class of four notes. The triads are
3-11, the diminished seventh 4-28, the major scale 7-35, the whole-tone
scale 6-35. Books on 20th-century music use these numbers all the time,
so seeing them in the piano roll makes those books easier to follow.

### The interval vector

Every pair of notes in a set makes an interval. Counting them by size
gives six numbers: how many minor seconds, major seconds, minor thirds,
major thirds, fourths and tritones. A fifth counts as a fourth (it is a
fourth upside down) and a major seventh as a minor second, so six sizes
are enough. This is the **interval vector**, and it is a fair summary
of what a set *sounds* like. For C E G B:

```text
C-E major third   C-G fourth   C-B minor second
E-G minor third   E-B fourth   G-B major third
<1 0 1 2 2 0>, written <101220>
```

Two curiosities follow from it:

- **Z-related sets** have the same interval vector without being in the
  same class. 4-Z15 [0146] and 4-Z29 [0137] both contain one interval
  of every size, `<111111>` (they are the two **all-interval
  tetrachords**), yet neither can be moved onto the other. The **Z** in
  their names marks them.
- **Complements**: the notes a set leaves out make another set. The
  white keys (7-35) leave out the black keys, the pentatonic scale
  (5-35). A hexachord and its complement always have the same interval
  vector, which is why hexachords matter so much for combinatoriality.

### Sets in bits

Since [step 3](../step-3/theory.md), a `PitchSet` has been twelve bits
of a `u16`: bit 0 for C, bit 1 for C#, up to bit 11 for B. Set theory
turns out to fit those bits remarkably well.

**Transposing is rotating the bits.** Move every note up `shift`
semitones: shift the bits left, and the ones that fall off the top (past
B) come back in at the bottom (C):

```rust
pub fn transpose(self, semitones: u8) -> PitchSet {
    let shift = (semitones % 12) as u32;
    let bits = self.0 as u32;
    let rotated = (bits << shift) | (bits >> (12 - shift));
    PitchSet(rotated as u16 & ALL_BITS)
}
```

**The complement is `!`.** "Not" flips every bit, and the mask keeps
the twelve that mean notes:

```rust
pub fn complement(self) -> PitchSet {
    PitchSet::from_bits(!self.bits())
}
```

**The most compact form is the smallest number.** This is the neat
part. Move a set down so that its first note is C (bit 0). Its last
note is now its highest bit, so a smaller span means a smaller highest
bit, which means a smaller number. On a tie, the next bit down is the
note before last, which is exactly Rahn's tie-break. Comparing two
numbers *is* Rahn's rule:

| starting on | moved to C | bits | number |
|-------------|-----------|------|--------|
| C | 0 4 7 11 | `1000_1001_0001` | 2193 |
| E | 0 3 7 8 | `0001_1000_1001` | 393 |
| G | 0 4 5 9 | `0010_0011_0001` | 561 |
| B | 0 1 5 8 | `0001_0010_0011` | 291 |

So normal form is "the start that gives the smallest number", and
prime form is "the smallest number of all 24 transpositions and
inversions":

```rust
pub fn normal_form(self) -> Vec<PitchClass> {
    let Some(start) = self
        .iter()
        .min_by_key(|start| self.relative_to(*start).bits())
    else {
        return Vec::new();
    };
    self.relative_to(start)
        .iter()
        .map(|pitch_class| pitch_class.transpose(start.semitone() as i32))
        .collect()
}

pub fn prime_form(self) -> PitchSet {
    let inverted = self.invert();
    (0..12)
        .flat_map(|n| [self.transpose(n), inverted.transpose(n)])
        .min_by_key(|set| set.bits())
        .unwrap_or(self)
}
```

`min_by_key` walks an iterator and keeps the item with the smallest
key. In `prime_form`, `flat_map` turns each `n` into an array of two
sets, so the iterator yields 24 sets, and the smallest one is the prime
form. No sorting, no lists of intervals: a few integer comparisons.

The interval vector is the one place that counts pairs the slow way, a
loop inside a loop, and with at most twelve notes that is at most 66
pairs:

```rust
pub fn interval_vector(self) -> [u8; 6] {
    let notes = self.semitones();
    let mut vector = [0; 6];
    for (i, low) in notes.iter().enumerate() {
        for high in &notes[i + 1..] {
            let interval = high - low;
            let class = interval.min(12 - interval);
            vector[class as usize - 1] += 1;
        }
    }
    vector
}
```

`interval.min(12 - interval)` folds an interval and its inversion
together: 7 (a fifth) becomes 5 (a fourth).

### The table of 224 classes, and how to trust it

The prime form can be computed, but the Forte *number* cannot: it is
just the place in Forte's list. So `set_class.rs` holds the list, as a
constant array of 224 entries:

```rust
const SET_CLASSES: [SetClass; 224] = [
    SetClass::new(0, 1, false, 0),
    SetClass::new(1, 1, false, 0b000000000001),
    SetClass::new(2, 1, false, 0b000000000011),
    // ...
    SetClass::new(4, 20, false, 0b000100100011),  // [0158]
    // ...
    SetClass::new(12, 1, false, 0b111111111111),
];
```

`SetClass::new` and `PitchSet::from_bits` are `const fn`, so the whole
table is built while compiling and stored with the program; looking a
set up costs nothing at start-up. The `false`/`true` says whether the
class has a Z in its name.

The table was generated from the one in music21, a music library for
Python (BSD licence), and 224 lines of numbers are easy to get wrong.
So the tests check it against the theory rather than trusting it:

- every entry is its own prime form, and the entries are in order of
  size and number;
- there are 1, 1, 6, 12, 29, 38, 50, 38, 29, 12, 6, 1 and 1 classes of
  0 to 12 notes, the counts every textbook gives;
- **all 4,096 sets** are looked up, and each belongs to exactly one
  class of the right size;
- a class has a Z exactly when another class of its size has the same
  interval vector.

Because of the third test, the lookup can say what it expects without
handling a "not found" that cannot happen:

```rust
pub fn of(set: PitchSet) -> &'static SetClass {
    let prime = set.prime_form();
    SET_CLASSES
        .iter()
        .find(|class| class.prime == prime)
        .expect("every prime form is in the table")
}
```

`expect` panics with that message if the `Option` is `None`. Using it
is a promise, and the test is what keeps the promise.

The function returns `&'static SetClass`: a borrow of an entry in the
constant table, which lives as long as the program, so callers can keep
it as long as they like.

## The set browser, and naming what you play

**Sets…** in the piano roll opens a list of every class of the size
you pick (2 to 10 notes), each with its Forte number, prime form,
interval vector and, for the famous ones, a name: "3-11 [037] <001110>
major or minor triad", "6-35 [02468T] <060603> whole-tone scale". The
search box finds a class by its Forte number or its name ("triad",
"octatonic", "4-Z15"), and **Show it** finds the class of the notes
selected in the piano roll. Picking one shows its complement, its
Z-partner if it has one, and whether it is **symmetric** (upside down
it is just a transposition, as for the diminished seventh). **Move it
up** and **Inverted** choose which member of the class to use: T3 is
the set three semitones up, T3I its inversion moved up three. Then you
can:

- **Draw with it**: the set becomes the piano roll's chord stamp, so
  each click places the whole set above the note clicked. It is a new
  variant, `ChordStamp::Set(PitchSet)`, next to the chord stamps of
  [step 6](../step-6/tools.md);
- **Use as the key**: the set becomes the scale, so the piano roll
  highlights its notes and the harmony keys play chords stacked from it.

And the piano roll now names any group of notes. Under the toolbar,
next to the chord at the step under the pointer, it shows that step's
set when three or more notes sound there, and the set of the selected
notes when two or more are selected:

```text
Set: 3-11 · normal form G# B D# · prime form [037] · intervals <001110> · major or minor triad
```

`vector_text` writes the six counts as digits, `<001110>`, the way the
books do, until one of them reaches 10: all twelve notes have twelve of
each interval but tritones, and that is written with spaces,
`<12 12 12 12 12 6>`, so it cannot be misread.

## Twelve-tone styles and demos

The Library has a new group, **Twelve-tone rows**, with six styles:

| style | what it does |
|-------|--------------|
| Row melody | the Lineup row as a tune in eighth notes, through P, R, I and RI |
| Triad chorale | the Four triads row as chords, over its retrograde in the bass: twelve-tone music that sounds almost tonal |
| Hexachord pads | half the row as a chord, the other six notes from I3 as a melody: hexachordal combinatoriality |
| Pointillism | single notes leaping over three octaves, durations and dynamics from the row, as in Webern |
| Total serialism | pitches, durations, dynamics and octaves all from the all-interval row |
| Twelve-tone lo-fi | the row's trichords over a lo-fi beat: a row can groove too |

A harmony style was a chord progression and a way of playing it. A row
style has no progression, so it is a row and a list of **voices**, each
saying how one instrument plays the row. As with the rest of the
library, it is data:

```rust
RowStyle {
    name: "Triad chorale",
    row: "Four triads",
    tempo: 72.0,
    time: TimeSignature::COMMON,
    bars: 4,
    voices: &[
        voice(KEYS, RowForm::Prime)
            .chain(Chain::FourForms)
            .chords(3, 4)
            .from(Register::Nearest, 55),
        voice(BASS, RowForm::Retrograde)
            .step(8)
            .from(Register::Close, 36),
    ],
    rhythm: "",
    // ...
},
```

`voice` makes a plain voice (a melody in eighth notes from middle C),
and each method changes one thing and returns the new voice:

```rust
impl RowVoice {
    const fn chain(self, chain: Chain) -> Self {
        RowVoice { chain, ..self }
    }
    const fn chords(self, together: usize, step: usize) -> Self {
        RowVoice { together, step, ..self }
    }
    // ...
}
```

Taking `self` by value and giving back a changed copy with **struct
update syntax** (`..self`) lets the calls chain, like a builder. And
because they are `const fn`, the whole table can be a `static`: the
styles are built while compiling, with no code running at start-up.

### Form arithmetic

The demos play each style four times, as the sections of a song:
**P: the row**, **R: backwards**, **I: upside down** and **RI: both**,
then P once more. But a style's voices already use different forms:
the Triad chorale's bass plays R. What does the bass play in the R
section? R of R, which is the row forwards again: P.

The four forms behave like two switches, "upside down" and
"backwards", and combining two forms flips each switch once for each
form that has it on:

```rust
pub fn combine(section: RowForm, voice: RowForm) -> RowForm {
    let flags = |form| match form {
        RowForm::Prime => (false, false),
        RowForm::Inversion => (true, false),
        RowForm::Retrograde => (false, true),
        RowForm::RetrogradeInversion => (true, true),
    };
    let (a, b) = (flags(section), flags(voice));
    match (a.0 != b.0, a.1 != b.1) {
        (false, false) => RowForm::Prime,
        (true, false) => RowForm::Inversion,
        (false, true) => RowForm::Retrograde,
        (true, true) => RowForm::RetrogradeInversion,
    }
}
```

`a.0 != b.0` on two `bool`s is an *exclusive or*: true when exactly one
is true. Mathematicians call this little system the *Klein four-group*:
every form undoes itself (I of I is P), and the order does not matter.
One more detail: the Hexachord pads melody plays I3, three semitones
*above* the row's first note. Upside down, "three above" becomes "three
below", so in the I and RI sections that offset becomes 12 − 3 = 9.

### Playing a whole song as a row

As with the other styles, a row style can be tried two ways: stopped,
it is written into a new pattern of its own; while the song plays, it
takes over the whole song. `use_row_style` goes through every pattern
that has chords, a bass line or a melody, and writes the style there,
reading the patterns in turn as P, R, I and RI so the song does not
repeat itself.

Testing that in the running app found two problems. **Row melody**
changed nothing at all: the first version wrote a style's chords and
bass into the song and kept the song's own melody, so a style that is
only a melody had nothing to write. And the **Triad chorale** wrote its
chords on a Keys channel of their own while the song's chords kept
playing on theirs: C major triads under a row only clash. The fix follows one rule:
the style takes over the harmony. Chords and bass channels the style
does not use go quiet in that pattern, and the style's chords go on
the channel that was playing the song's chords:

```rust
if chords.is_some() || plays(project, bass) || melody {
    let before = project.pattern(pattern).cloned();
    let quiet: Vec<usize> = [(KEYS, chords), (BASS, bass)]
        .into_iter()
        .filter(|(name, _)| !uses(name))
        .filter_map(|(_, channel)| channel)
        .collect();
    // ... empty the `quiet` channels, write the voices ...
    if project.pattern(pattern) != before.as_ref() {
        written += 1;
        change.harmony += 1;
    }
}
```

To know whether a pattern really changed, it keeps a copy from before
(`cloned()` turns the `Option<&Pattern>` into an `Option<Pattern>`)
and compares at the end. `as_ref()` turns the copy back into an
`Option<&Pattern>`, so the two sides of `!=` have the same type.

## Keeping a new window in front

One bug was not about music at all. Clicking **Row…** opened the Row
window *behind* the piano roll, where it could not be seen.

egui keeps its windows in layers and draws the one that "wants to be on
top" last. Two windows wanted that in the same frame: the new Row
window, which egui raises when it first appears, and the piano roll,
because its **Row…** button had just been clicked, and a click raises
the window it lands in. egui sorts the windows that want the top with
a *stable* sort, which keeps their order when they tie, and the piano
roll came first. So the fix keeps the new window in front for its first
few frames:

```rust
#[derive(Default)]
struct Front {
    /// The frame the window was last drawn in.
    drawn_in: u64,
    /// The frame it opened in.
    opened_in: u64,
}

impl Front {
    fn keep(&mut self, ctx: &egui::Context, id: egui::Id) {
        let frame = ctx.cumulative_pass_nr();
        if self.drawn_in + 1 < frame {
            self.opened_in = frame;
        }
        self.drawn_in = frame;
        if frame - self.opened_in < 3 {
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, id));
            ctx.request_repaint();
        }
    }
}
```

`cumulative_pass_nr` counts the frames egui has drawn. A window that
was not drawn last frame has just opened, so it remembers that frame;
for its first three frames it moves its own layer to the top and asks
for another frame, so the three come at once instead of waiting for
the mouse to move. To find its layer, the window needs to know its id, so
both windows now set one with `.id(...)`, made with `egui::Id::unique`.
(egui 0.37 calls it *unique* to make clear it names one thing in the
whole program, as a window id must.)

The Row window also got taller than some screens. It now scrolls
(`.vscroll(true)`), can be made taller or shorter but not wider
(`.resizable([false, true])`), and stays inside the main window
(`.constrain_to(area)`).

## How it was tested

- **Rows**: Schoenberg's four forms are checked note by note against
  the published ones; every one of the 48 forms of the Lineup row is
  named again by `label_of`; every line of Berg's matrix is a P form
  and every column an I form; Webern's row is a palindrome (R9 is P3);
  Berg's row, the All the intervals row and fifty random all-interval
  rows are all-interval; the Lineup row is combinatorial with I3 only;
  rows are read from names, numbers and the compact form, and wrong ones
  give the right message; random rows are rows, and the same seed gives
  the same row; a row with a note written twice cannot be read from a
  file.
- **Sets**: the table tests above, plus well-known classes by name,
  normal forms that wrap around the octave, Z-partners, complements,
  symmetric classes, and Rahn's prime form for 5-20 where Forte's book
  differs.
- **Serialism**: a melody in eighth notes, the four forms in turn
  filling the pattern, trichords as chords, the three series, and notes
  that never run past the end of the pattern.
- **Styles and demos**: every style uses a row from the presets and
  writes its voices in any key; combining forms; Hexachord pads has all
  twelve notes in every bar; a song takes a row style, and the Triad
  chorale replaces the song's chords on their own channel instead of
  playing over them; every style makes a demo with five sections that
  saves as format 6.
- **In the running app**: the key menus, the numbers on the keys, the
  harmony keys of a row, the matrix and writing each form, the
  Schoenberg row's properties, sets found by name, used as a key and
  drawn with, the selection's set, the Library styles both stopped and
  with the song playing, the demos, and the windows opening in front,
  at 1280×800 too.

## Trying it

1. Open **File > Demos > Twelve-tone > Triad chorale** and press Space.
   Watch the section names in the playlist: the same music, then
   backwards, upside down, and both.
2. Open the piano roll (**F7**) and click **Row…**. Pick **Famous and
   example rows > Schoenberg's Suite**. The window says its tetrachords
   are 4-12, 4-Z15 and 4-1, and that it is combinatorial with I11.
3. Click **I11** above the matrix, set **Together** to 6, and click
   **Write I11 into the pattern**: two hexachords as chords. Press
   **Ctrl+Z** to take it back.
4. Tick the three **Serialism** boxes, set **Octaves** to **Wide**, and
   write P4 again: Webern-like points of sound.
5. Click **Sets…**, type `octatonic`, and click **Use as the key**.
   The harmony keys now play chords stacked from the octatonic scale.
6. Select a few notes in the piano roll and read their set under the
   toolbar.

## Exercises

1. **Distinct forms (lineup-core, small).** Write a test that counts how
   many of the 48 forms of a row are different (collect them into a
   `HashSet<ToneRow>`). Check that Webern's row has 24 and Schoenberg's
   48. Can any row have fewer than 24? (Hint: could a row be one of its
   own I forms? Write out what that means for each note, and you will
   find that at most two notes could fit.)
2. **Derived rows (lineup-core, medium).** Webern's Concerto, Op. 24,
   uses a row whose four trichords all belong to the same class, 3-3
   [014]. Add `ToneRow::is_derived(size)`, true when every segment of
   that size has the same set class, and a `random_derived` that builds
   one by backtracking, like `random_all_interval`.
3. **Forte's way (lineup-core, small).** For six classes, Forte's prime
   form differs from Rahn's. Write `forte_prime()` that breaks ties
   from the start of the set instead of from the end, and show it in
   the set browser's details when it differs. Why can the trick of
   "the smallest number" not do this one directly? (Think about which
   bits a comparison looks at first.)
4. **Multiplying by 5 (lineup-core, medium).** Multiplying every pitch
   class by 5 (mod 12) turns the chromatic scale into the circle of
   fourths: 0 5 10 3 8 1 … This **M5** operation is a fifth way to
   transform a row or a set, used by some composers after Webern. Add
   `PitchSet::multiply` and a `ToneRow::multiply`, and find which set
   classes M5 turns into a different class. (Hint: the Z-related pairs
   are worth a look.)
5. **A series of its own (lineup-core and lineup-app, medium).** In
   `write_row`, the durations always read the row's own numbers. Let
   the user type a second row, used only for the durations, and show
   both series in the window. `RowWriting` derives `Copy`; why does it
   still compile once it holds a second `ToneRow`, and why would it not
   if the second series were a `Vec<u8>`?
