# Twelve-tone rows and their rules

Files: [`crates/lineup-core/src/harmony/row.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/row.rs),
[`crates/lineup-core/src/serial.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/serial.rs),
[`crates/lineup-core/src/library/rows.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rows.rs),
[`crates/lineup-core/src/harmony/voice_leading/serial.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_leading/serial.rs)
and [`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs)

Most of Ferrous Lineup assumes music in a key: a home note, chords that
lean towards it, harmony keys numbered I to vii°. A large part of the
music of the 20th century has no key at all, and much of it was written
with **twelve-tone rows** (*dodecafonismo* in Spanish, also called
*series*): all twelve notes in an order that the whole piece is made
of. In the app, **twelve-tone row** is one of the kinds of key,
**Row…** in the piano roll opens a window to type, draw or pick a row,
see its **matrix** of 48 forms and write any form into the pattern,
the Library has a group of row styles, and the **Voice leading** check
knows the rules twelve-tone composers wrote by. This chapter covers
all four, and the Rust behind them: a newtype that is always a real
row, arithmetic on twelve numbers, backtracking, a builder that runs
while compiling, bit sets and dynamic programming.

## The music, briefly

In C major, C is home. G pulls towards it, B pulls hard, and a piece
that ends on C sounds finished. Around 1921 Arnold Schoenberg looked
for a way to write music in which no note is home. His answer was to
use all twelve notes equally, in a fixed order that he chose for each
piece: the **row**. No note comes back until the other eleven have
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

Some books number the forms from the row's own first note instead, so
that the row as written is always P0. Then Schoenberg's row is P0, and
what Ferrous Lineup calls I11 is I7 there. The forms are the same; only
their numbers move. Ferrous Lineup numbers a form by the note it starts
on, because then a label names a note you can find on the keyboard.

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
way down (E, here): a quick check that a matrix was filled in
correctly.

Groups of notes taken from a row, such as its four groups of three,
are **pitch-class sets**, and the theory that names them has a chapter
of its own: [Pitch-class sets, and one harmony with four
names](sets.md).

## A row in Rust: `ToneRow`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ToneRow([u8; 12]);
```

A row is twelve numbers, the semitones above C of its notes. That is a
**newtype** around an array: the array is private to the module, so
the only ways to make a `ToneRow` are functions that check it:

```rust
pub fn new(semitones: [u8; 12]) -> Option<Self> {
    let set: PitchSet = PitchSet::from_semitones(&semitones);
    (set.len() == 12 && semitones.iter().all(|&n| n < 12)).then_some(ToneRow(semitones))
}
```

Collect the twelve numbers into a set of pitch classes, and if the set
has twelve members, no note was repeated. Every `ToneRow` in the
program is therefore a real row: the **invariant** is checked once,
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
can read, instead of a list of twelve numbers in brackets. Saving
cannot fail, so it is a plain `From`; reading can, so it is a `TryFrom`
with an error. Because reading goes through `parse`, a file with a note
written twice is refused with a message, rather than producing a
broken row. `PitchSet` has the same pair, so a set is saved as
`"0 3 6 9"`.

`parse` also reads the box at the top of the Row window. It takes note
names (`E F G Db Gb Eb Ab D B C A Bb`), numbers (`4 5 7 1 …`) or the
compact form twelve-tone books use, one character a note with **T**
and **E** for ten and eleven (`45716382E09T`). When a note comes twice
it says which, and which note is missing instead.

**From the notes** reads a row from the piano roll: the first twelve
different pitch classes among the selected notes (or all of them), in
the order they start. `ToneRow::from_notes` skips a note whose pitch
class is already in the row, so you can draw freely and repeat notes,
as long as all twelve turn up. Both end in a third conversion, from a
`Vec<u8>`:

```rust
impl TryFrom<Vec<u8>> for ToneRow {
    type Error = String;

    fn try_from(semitones: Vec<u8>) -> Result<Self, String> {
        let array: [u8; 12] = semitones
            .try_into()
            .map_err(|v: Vec<u8>| trf("a row has 12 notes, not {}", &[&v.len()]))?;
        ToneRow::new(array).ok_or_else(|| tr("a row has each of the 12 notes once").to_string())
    }
}
```

A `Vec` can hold any number of notes and an array exactly twelve, so
the standard library's `try_into` from one to the other can fail. When
it does, it hands the vector back unchanged as the error, and
`map_err` turns that into a message that says how many notes there
were. `ok_or_else` then turns `new`'s `None` into an error too. The
closures run only when something is wrong, so a good row never builds
a message.

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

Going the other way, "which form of this row is that?", tries all 48
until one matches:

```rust
pub fn label_of(self, other: ToneRow) -> Option<RowLabel> {
    RowForm::ALL.into_iter().find_map(|form| {
        (0..12)
            .map(|n| RowLabel::new(form, n))
            .find(|&label| self.form(label) == other)
    })
}
```

`find_map` stops at the first form that gives `Some`, and `find` at
the first number that matches, so the search ends as soon as the form
is found.

## What makes a row interesting

Under the matrix, the Row window lists what is special about the row.
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

The return type says eleven, so `from_fn` calls the closure eleven
times, and `i + 1` never runs past the twelfth note.

**All-interval rows.** When the eleven steps are all different, every
interval from a semitone to a major seventh appears exactly once. Alban
Berg's Lyric Suite (1927) starts with such a row: F E C A G D G# C# D#
F# A# B, with the tritone right in the middle. They are rare: of the
39,916,800 rows that start on C, only 3,856 are all-interval. So the
**🎲 All-interval** button cannot just shuffle notes until it finds
one; it builds the row a note at a time:

```rust
fn all_interval_from(notes: &mut Vec<u8>, random: &mut Random) -> Option<Vec<u8>> {
    if notes.len() == 12 {
        return Some(notes.clone());
    }
    let used: Vec<u8> = notes.windows(2).map(|w| (w[1] + 12 - w[0]) % 12).collect();
    let mut intervals: Vec<u8> = (1..12).filter(|i| !used.contains(i)).collect();
    for i in (1..intervals.len()).rev() {
        intervals.swap(i, random.below(i + 1));
    }
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
neighbours, `[a, b]`, which is exactly what a step is. All the calls
share one vector through `&mut`, so trying a note costs a `push` and a
`pop`, not a copy.

**Random rows.** The **🎲 Random** button shuffles the twelve notes
with the **Fisher-Yates shuffle**, the same loop that shuffles the
intervals above: walk from the last place to the first, and swap each
with a place chosen at random among those not done yet. Every order is
then equally likely. The random numbers come from a tiny generator,
*SplitMix64*, written out in a few lines instead of adding a library:

```rust
pub(crate) fn next(&mut self) -> u64 {
    self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = self.0;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
```

`wrapping_add` and `wrapping_mul` say "overflowing is fine here, just
wrap around": a plain `+` that overflows would stop a debug build. The
same seed always gives the same row, which makes the tests repeatable.
`pub(crate)` lets other modules of `lineup-core` use the generator (ear
training picks its questions with it) while keeping it out of the
crate's public API.

**Groups of notes.** Composers often play a row in groups: four
trichords (three notes each), three tetrachords, two hexachords.
`segments(size)` cuts the row with `chunks_exact`, which hands out
slices of exactly `size` notes and leaves out a shorter last one, and
the window names each group's set class (see [Pitch-class
sets](sets.md)). The tetrachords of Schoenberg's row are
three different classes, 4-12, 4-Z15 and 4-1; the trichords of the Four
triads row are all 3-11, the major and minor triads.

**Combinatoriality.** Take the first six notes of a row. Some other
form of the same row may start with exactly the *other* six. Play the
two forms at the same time, and each half of the music holds all twelve
notes, without one coming back early. Schoenberg built his later works
on this, and it is called **hexachordal combinatoriality**:

```rust
pub fn combinatorial_forms(self) -> Vec<RowLabel> {
    let missing = PitchSet::from_semitones(&self.0[..6]).complement();
    [
        RowForm::Prime,
        RowForm::Inversion,
        RowForm::RetrogradeInversion,
    ]
    .into_iter()
    .flat_map(|form| (0..12).map(move |n| RowLabel::new(form, n)))
    .filter(|&label| PitchSet::from_semitones(&self.form(label).0[..6]) == missing)
    .collect()
}
```

The retrograde is left out because it always works (its first half is
the row's second half), so it says nothing about the row. Schoenberg's
Op. 25 row is combinatorial with I11: its first six notes, E F G C# F#
D#, and the first six of I11, B A# G# D A C, make all twelve. The
Lineup row, written for Ferrous Lineup, is combinatorial with I3.

The `move` in `move |n|` is needed because the inner closure is handed
out of the outer one by `flat_map`, and it uses `form`: `move` makes it
take its own copy of `form` instead of borrowing a variable that is
about to go away.

### The rows to start from

The **Famous and example rows** menu holds eight rows. Four come from
famous works, and four were written for Ferrous Lineup to show what a
row can do. `ROW_PRESETS` is a `const` array of them, each with a name,
where it comes from and a sentence about it.

| row | from | what it shows |
|-----|------|---------------|
| E F G C# F# D# G# D B C A A# | Schoenberg, Suite for Piano, Op. 25 (1925) | one of the first twelve-tone pieces; its last four notes, B C A A#, spell BACH backwards in German note names, where B is B♭ and H is B |
| F E C A G D G# C# D# F# A# B | Berg, Lyric Suite (1927) | an all-interval row, with the tritone in the middle |
| A F# G G# E F B A# D C# C D# | Webern, Symphony, Op. 21 (1929) | a palindrome: backwards, it is the same row a tritone higher (R9 is P3) |
| B A# D D# G F# G# E F C C# A | Webern, Concerto, Op. 24 (1934) | four groups of three notes, all the same micromode, Menor 3 |
| C D F# G# F E A C# B D# G A# | Lineup row | hexachordally combinatorial with I3 |
| C E G D F# A A# C# F B D# G# | Four triads | four triads that still use every note once |
| C E D# G G# B F D C# A A# F# | Mirror row | a palindrome, like Webern's |
| C D F E B A D# G G# C# A# F# | All the intervals | an all-interval row |

The works from 1925 to 1929 are in the public domain in the United
States (95 years after publication), and their composers died in 1951,
1935 and 1945, so they are in the public domain where copyright lasts
70 years after the author's death too. Webern's Concerto was published
in 1948, so it is in the public domain where copyright lasts 70 years
after the author's death. Only the rows are used: twelve notes each.

Some rows do not have 48 *different* forms. Since Webern's R9 is its
P3, every R form of that row is also a P form, and every RI form an I
form: only 24 are different. Berg's row and the Mirror row are the
same: read backwards, each is itself a tritone higher. `label_of` tries
the P forms first, so such a form is named as the P form it also is.

## From a row to notes: serialism

A row says which note comes next, and nothing else. To make music of
it, someone still has to decide how long each note is, how loud, in
which octave, and whether some notes sound together. `RowWriting`
holds those choices, and the controls under the matrix set them:

```rust
pub struct RowWriting {
    /// The form to start with, such as `P0`.
    pub label: RowLabel,
    pub chain: Chain,
    /// How many notes of the row sound together: 1 for a melody, 3 to
    /// play the row as four three-note chords (trichords), 6 for two
    /// hexachords.
    pub together: usize,
    /// How long each note (or chord) lasts, in steps, when the durations
    /// do not follow the row.
    pub step: usize,
    pub register: Register,
    /// The lowest note the row is written from.
    pub low: Note,
    /// Whether the durations follow the row (see the module documentation).
    pub durations: bool,
    /// Whether the velocities follow the row's inversion.
    pub dynamics: bool,
    /// Whether the octaves follow the row's retrograde.
    pub octaves: bool,
}
```

**Then** (`chain`) says what comes after the form until the pattern is
full: nothing (**Once**), the same form again (**Repeat**), or the four
forms in turn with the same number (**P, R, I, RI**). **Together** is
`together`, and **Each** is `step`. The three **registers** (the
**Octaves** menu) decide the octaves when the row does not:

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
different form of the row so that the series do not move in step:

- **Durations**: the row's numbers (each note's distance above the
  first note, 0 to 11) become lengths. The *k*-th note lasts as many
  steps as the *k*-th number, plus one, so from 1 to 12 sixteenth notes.
- **Dynamics**: the numbers of the inversion become twelve velocities,
  from very soft to very loud: `28 + 9 * level`, from 28 to 127.
- **Octaves**: the numbers of the retrograde, counted in threes
  (`% 3`), pick one of three octaves.

For Schoenberg's row the numbers are 0 1 3 9 2 11 4 10 7 8 5 6, so with
**Durations** ticked its notes last 1, 2, 4, 10, 3, 12… steps. Inside
`write_row`, each note asks the three questions in turn:

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
again. `k` is `place` plus the note's place in its chord, so each note
of a chord gets its own octave and loudness.

`write_row` is a pure function: a row, the choices and a pattern length
in; a list of `NoteEvent`s out. The window calls it every frame to warn
when the pattern is too short for the whole row ("The pattern only has
room for 9 of the 12 notes"), and **Write P0 into current pattern**
sends the same notes to the piano roll, which replaces the channel's
notes through the usual edit, so **Ctrl+Z** brings the old ones back.
**Save to new pattern** puts them in a new pattern instead.

## A row as the key

**twelve-tone row** in the key menus, or **Use as the key** in the Row
window for the form you chose, makes the row the key. In the code it is
one more kind of key, `ScaleKind::Row(ToneRow)`, which carries its row
with it ([Scales, modes and keys](scales.md) shows how the
rest of the code treats it). The row is stored starting on C, and the
key's root says where it really starts, so P4 of Schoenberg's row is
saved like this:

```text
scale: (root: E, kind: Row("0 1 3 9 2 11 4 10 7 8 5 6")),
```

With a row as the key, the piano roll numbers its notes **1 to 12** on
the keys, in the row's order, so you can draw the row by following the
numbers. The harmony keys play the row's groups instead of chords, the
way twelve-tone composers build their chords: **3 notes** gives four
keys for the trichords, **1–3**, **4–6**, **7–9** and **10–12**; **4
notes** gives three keys, and **6 notes** two. They come from cutting
the row in order:

```rust
ScaleKind::Row(_) => {
    let group = row_group(size);
    Some(
        notes
            .chunks(group)
            .enumerate()
            .map(|(i, chunk)| {
                let label = format!("{}–{}", i * group + 1, i * group + chunk.len());
                GroupKey::new(scale, Some(label), chunk.to_vec())
            })
            .collect(),
    )
}
```

Changing the key from C major to a row moves no notes, since there is
no sensible place for E to go in a row; changing to the same row on
another note moves them all, which is transposing. Switching from a row
to **pitch-class set** keeps the row's first three notes as the set,
which are what the harmony keys were playing (see [A key as a set: the
same notes, another
name](sets.md#a-key-as-a-set-the-same-notes-another-name)).

## Row styles and demos

The Library's **Twelve-tone rows** group has seven styles:

| style | what it does |
|-------|--------------|
| Row melody | the Lineup row as a tune in eighth notes, through P, R, I and RI |
| Triad chorale | the Four triads row as chords, over its retrograde in the bass: twelve-tone music that sounds almost tonal |
| Hexachord pads | half the row as a chord, the other six notes from I3 as a melody: every bar has all twelve notes |
| Pointillism | single notes leaping over three octaves, durations and dynamics from the row, as in Webern |
| Total serialism | pitches, durations, dynamics and octaves all from the all-interval row, over its retrograde inversion in two-note chords |
| Twelve-tone lo-fi | the row's trichords over a lo-fi beat, its prime form in the bass: a row can groove too |
| Webern's trichords | the row of Webern's Concerto, its melody leaping over three octaves over its groups of three as chords |

A harmony style is a chord progression and a way of playing it. A row
style has no progression, so it is a row and a list of **voices**, each
saying how one instrument plays the row. As with the rest of the
Library, it is data:

```rust
RowStyle {
    name: "Triad chorale",
    // ...
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
},
```

`voice` makes a plain voice (a melody in eighth notes from middle C),
and each method changes one thing and returns the new voice:

```rust
impl RowVoice {
    const fn offset(self, offset: u8) -> Self {
        RowVoice { offset, ..self }
    }
    const fn chain(self, chain: Chain) -> Self {
        RowVoice { chain, ..self }
    }
    // ...
}
```

Taking `self` by value and giving back a changed copy with **struct
update syntax** (`..self`) lets the calls chain, like a builder. And
because they are `const fn`, the whole table can be a `static`:
`pub static ROW_STYLES: [RowStyle; 7]` is built while compiling, with
no code running at start-up.

### Form arithmetic

The demos in **File > Demos > Twelve-tone** play each style four times,
as the sections of a song: **P: the row**, **R: backwards**, **I:
upside down** and **RI: both**, then P once more to end. With a rhythm,
the drums come in after the first section, with a fill before the last.
But a style's voices already use different forms: the Triad chorale's
bass plays R. What does the bass play in the R section? R of R, which
is the row forwards again: P.

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
is true. Mathematicians call this little system the **Klein
four-group**: every form undoes itself (I of I is P), and the order
does not matter. One more detail: the Hexachord pads melody plays I3,
three semitones *above* the row's first note. Upside down, "three
above" becomes "three below", so in the I and RI sections that offset
becomes 12 − 3 = 9:

```rust
let offset = if inverted {
    (12 - self.offset % 12) % 12
} else {
    self.offset % 12
};
```

### Playing a whole song as a row

As with the other styles, a row style can be tried two ways: stopped,
it is written into a pattern of its own; while the song plays, it takes
over the whole song. `use_row_style` goes through every pattern that
has chords, a bass line or a melody, and writes the style there,
reading the patterns in turn as P, R, I and RI so the song does not
repeat itself. The key becomes the row, starting on the song's key
note, and endings stay as they are.

One rule decides what happens to the song's own parts: **the style
takes over the harmony**. Tonal chords or a tonal bass under a row only
clash, so the chords and bass channels the style does not use go quiet
in that pattern, and the style's chords go on the channel that was
playing the song's chords. The melody stays, unless the style has one
of its own:

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
`Option<&Pattern>`, so the two sides of `!=` have the same type. This
function, `take_over_harmony`, takes the writing as a closure, so the
[micromode styles](micromodes.md) use it too.

## A window that opens in front

egui puts a window that opens on top of the others. But the window
whose **Row…** button opened it was clicked in the same frame, and a
click raises the window it lands in, so both want the top, and the new
one can stay behind. `Front` keeps a new window in front for its first
few frames:

```rust
#[derive(Default)]
pub(crate) struct Front {
    /// The frame the window was last drawn in.
    drawn_in: u64,
    /// The frame it opened in.
    opened_in: u64,
}

impl Front {
    /// Called each frame the window `id` is drawn, before it is.
    pub(crate) fn keep(&mut self, ctx: &egui::Context, id: egui::Id) {
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
the mouse to move. To find its layer, the window needs its id, so the
Row, Sets and Micromodes windows each set one with `.id(...)`, made with
`egui::Id::unique` (*unique* because it names one thing in the whole
program, as a window id must). The playlist does the same when a
double-click opens a clip in the piano roll ([Double-click a clip to
edit it](../program/editors.md#double-click-a-clip-to-edit-it)).

The Row window is as wide as the matrix and scrolls when the screen is
short, so its buttons at the bottom stay in reach:
`.resizable([false, true])` lets it get taller or shorter but not
wider, and `.constrain_to(area)` keeps it inside the main window.

## The rules of twelve-tone writing

The [voice-leading check](voice-leading.md) marks what
classical part writing avoids: parallel fifths, a leading tone that
does not rise, a seventh that does not fall. Every one of those rules
is about a key. In music written with a row there is no tonic, so they
mean nothing there. Twelve-tone music has rules of its own. Schoenberg
wrote by them, and so did his students Berg and Webern; composition
courses still teach them. So when the key is a **twelve-tone row**, the
**Voice leading** tick box checks those rules instead.

### What you see

- **A Row line** under the chord names. It names each form of the row
  that the check finds in the notes, such as **P4** or **P10**, over
  the notes that play it. Point at it to read the form's notes: "P4:
  the row starting on E: E F G C# F# Eb Ab D B C A Bb."
- **The Voices line** under it marks what twelve-tone writing avoids:
  **order** for a note out of the row's order, **rep** for a note
  repeated too early, **gap** for a form left unfinished, **8ve** for a
  note doubled in octaves, **tonal** for a chord that sounds like tonal
  harmony.
- **Outlines** around the notes concerned. Two doubled notes are joined
  by a line, and so are the notes of a tonal chord. A dashed outline
  shows the note the row wanted instead.
- **The summary line** above the chord rows sums it all up:
  "Twelve-tone rules, P4, P10, through all the tracks together: out of
  row order (2), octave doubling (1), sounds tonal (1)." Point at a
  step with a finding and the line says what happens there; hover over
  the line to read why.

With a **pitch-class set** as the key, nothing is checked. A set is a
collection of notes with no order and no rules of its own, so there is
nothing to check against, and the line under the chords says so.

### The five rules

**The row sets the order.** A piece moves through forms of the row,
each played note after note, in its order, so that everything you hear
comes from the row. Notes next to each other in the row may sound
together as a chord, in any order from the bottom up, and a note may be
repeated straight away, or trilled with the note before it: that is
still the same note of the row. Two mistakes are counted with care. If
you **swap** two notes, or **leave one out**, that is one note out of
place, not every note after it: with B played before D in P4, the check
marks the B ("Melody plays B4 where P4 wants D, its note 8") and finds
the rest of the row in order again.

**No note comes back early.** Before a note comes back, the other
eleven sound. The row was made for that: no note is heard more often
than the others, so none of them becomes a tonic. A note that comes
back after other notes, before the form has sounded all twelve, is
marked **rep**.

**A form is played whole.** When a form stops short and the next one
starts, the notes it left out are missing from the twelve. The check
marks the last note of such a form with **gap** and names what never
sounded: "Row left unfinished: P4 stops after 8 of its 12 notes, with
Melody's D5; B, C, A and Bb never sound before the next form starts."

**No octave doublings.** Doubling a note in octaves gives it weight. As
Schoenberg explained, a note with weight can be heard as a root, or
even as a tonic. The check marks two parts that play the same note
together, and also a part that plays a note while another part still
holds it. A part that doubles another all the way through, a line in
octaves, is told once, where it starts.

**No chords of tonal harmony.** A major or minor triad, or a dominant
seventh, reminds the ear of tonal music, and the ear then expects the
music to go on as tonal music would. Schoenberg and Webern avoided
them. The check marks a moment where everything sounding together
makes one of these chords, and names it. **Berg** used them on purpose:
the row of his Violin Concerto (1935) is built from triads, G minor, D
major, A minor and E major. So read **tonal** as "this sounds like a
key", not "this is wrong".

### Reading the row as an analyst would

Nothing in the notes says which form of the row is playing, or where
one form ends and the next begins. A student analysing a score has to
find that out, and so does the check. It cuts the notes into
**groups**, notes that start together, then tries every way of sharing
the groups out among forms of the row, and keeps the way with the
fewest problems: each note out of order, each note repeated early and
each note a form leaves out counts one. A few things make the reading
closer to what composers actually do:

- **Forms can overlap at the join.** A form may begin on the note, or
  the chord, that the form before ended on, so the two share it: an
  **elision**. If P4 ends with the chord A B♭ C and P10 starts on B♭,
  P10 takes the B♭ from that chord and goes on with B.
- **A chord may share notes with the chord before.** Those notes count
  for the new form if it needs them, or as repeated notes if it does
  not.
- **One row or one per track.** All the tracks can share one form, as
  a tune and the chords under it often do, or each track can play
  forms of its own, as in a canon. The check reads the row both ways,
  keeps the reading with fewer problems, and the summary line says
  which one it kept.
- **Patterns join up.** Each pattern is checked on its own, but a row
  may run on from one pattern into the next. So the first form may
  have begun in the pattern before, as long as its notes here are in
  order: the Row line then says "…P4", and the summary "P4 from note
  3". The last form may go on in the next pattern, so it is never
  marked unfinished.
- **Ties.** When two readings have equally few problems, the check
  prefers forms that start on their first note. If two forms still
  tie, it names the first in the order P, I, R, RI. Chords can make two
  forms impossible to tell apart: the row of Webern's Concerto is four
  groups of three notes, and its forms R0 and I1 have the same three
  notes in each group, in another order. In the Webern's trichords
  style the melody and the chords both play R0 after P0. The melody's
  notes come one at a time, so they can only be R0; the chords sound
  each group at once, so they fit R0 and I1 equally, and the check
  names them I1, which comes first. Either name is right.

The check follows the classic rules of Schoenberg's method. It does
not judge **free atonal music**, written with no row at all (choose a
pitch-class set as the key for that); **serialism beyond pitch**,
whether durations, dynamics or registers follow a series too; or
**combinatoriality**: in the Hexachord pads style it reads each track's
own form, which is right, but it does not say that the two halves make
all twelve notes together.

## How the check reads the row

### A form's places in sixteen bits

`Walk` goes through one form of the row, group by group, and remembers
which of the twelve places have sounded. That is twelve yes-or-no
answers, so it keeps them as the bits of a `u16`, one bit per place:

```rust
#[derive(Debug, Clone, Copy)]
struct Walk {
    /// One bit for each place in the form, from the first.
    used: u16,
    /// The place after the furthest one sounded.
    next: usize,
    errors: usize,
}

impl Walk {
    fn from(from: usize) -> Walk {
        Walk {
            used: (1 << from) - 1,
            next: from,
            errors: 0,
        }
    }

    fn heard(self) -> usize {
        self.used.count_ones() as usize
    }

    fn has(self, place: usize) -> bool {
        self.used & (1 << place) != 0
    }
    // ...
}
```

`Walk::from(3)` starts a walk whose first three places have sounded
already, in the pattern before or as the shared notes of an elision:
`(1 << 3) - 1` is `0b111`. `Walk` is `Copy`, two machine words and a
bit set, so trying a form costs nothing to set up. `count_ones` counts
the places heard in one processor instruction on most machines. The
[pitch-class sets](pitch.md#a-set-of-pitch-classes-in-twelve-bits)
use the same idea, with twelve bits for the twelve notes.

### When are new notes in order?

A group of *n* new notes is in order when they fill the next *n* places
of the form, in any order among themselves. "The next places" can mean
two things, and `window` tries both:

```rust
fn open(self, places: std::ops::Range<usize>, count: usize) -> u16 {
    places
        .filter(|&place| !self.has(place))
        .take(count)
        .fold(0, |bits, place| bits | 1 << place)
}

fn window(self, taken: u16, count: usize) -> u16 {
    let first_open = self.open(0..12, count);
    let ahead = self.open(self.next..12, count);
    match (taken & ahead).count_ones() > (taken & first_open).count_ones() {
        true => ahead,
        false => first_open,
    }
}
```

`open` builds a set of places with a `fold` that ORs in one bit per
place. `first_open` is the first places still open, which catches up a
note skipped just before. `ahead` is the places after the furthest note
heard, which goes on after a skip. Whichever holds more of the group's
notes wins.

Take P4 with D and B swapped: E F G C# F# Eb Ab **B D** C A Bb. At B,
the form wants D, so B is out of place. At D, the first open place is
D's own, so D is in order, and so is everything after it.

Now take P4 with D left out: E F G C# F# Eb Ab **B** C A Bb. B is out
of place again. At C, the first open place is still D's, which never
comes. But the place after the furthest note heard, B, is C's own, so C
is in order, and so are A and Bb. Only B is marked. With `first_open`
alone, every note after the missing D would be out of place.

### One walk, two jobs

`take` takes in one group and finds what is out of place:

```rust
fn take(&mut self, form: &Form, group: &Group, mut tell: impl FnMut(Misplaced)) {
    // ...
    for &played in &group.notes {
        let place = place_of(&played);
        let kind = if repeat(&played) {
            continue;
        } else if self.has(place) {
            SlipKind::EarlyRepeat
        } else if window & (1 << place) == 0 {
            SlipKind::RowOrder
        } else {
            continue;
        };
        self.errors += 1;
        tell(Misplaced {
            kind,
            played,
            place,
            wanted,
            heard: self.heard(),
        });
    }
    self.used |= fresh;
    if fresh != 0 {
        self.next = self.next.max(16 - fresh.leading_zeros() as usize);
    }
}
```

`let kind = if ... { continue; } else ...` works because `continue`
never gives a value: its type is `!`, "never", which fits in place of
any type, so the other branches decide that `kind` is a `SlipKind`. A
note that only repeats the note just played is skipped this way, and so
is a note in its place. At the end, `16 - fresh.leading_zeros()` is the
place after the highest bit in `fresh`, the furthest place this group
reached.

The same `take` counts the problems while the readings are compared,
and reports them for the reading that wins. It does both through the
closure `tell`, which it calls for each note out of place. While
comparing, the closure does nothing: `walk.take(form, &groups[end - 1],
|_| {})`. When reporting, it turns each note into a finding:
`walk.take(form, group, |misplaced| slips.push(misplaced.slip(form,
key)))`. That second closure changes `slips`, a vector it borrows, so
it is an `FnMut`, not just an `Fn`. Because `take` is generic over the
closure (`impl FnMut`), the compiler builds a copy of `take` for each
one, and the empty one costs nothing at all.

### The best reading: dynamic programming

`read` has to choose where each form starts and ends, and which form it
is. `best[end]` is the fewest problems in the first `end` groups, and
`came_from[end]` is the last stretch on the way there:

```rust
for start in 0..n {
    if best[start] == usize::MAX {
        continue;
    }
    // ...
    for from in 0..12 {
        for (index, form) in forms.iter().enumerate() {
            // ... may this form start at its place `from` here? ...
            let mut walk = Walk::from(from);
            for end in start + 1..=reach {
                walk.take(form, &groups[end - 1], |_| {});
                // ...
                let missing = if end == n { 0 } else { 12 - walk.heard() };
                let cost = best[start] + walk.errors + missing;
                if cost < best[end] {
                    best[end] = cost;
                    came_from[end] = Stretch {
                        start,
                        end,
                        form: index,
                        from,
                    };
                }
            }
        }
    }
}
```

Each stretch is tried once for each form. The walk carries on from one
`end` to the next instead of starting again, so a form tried on twenty
groups costs twenty steps, not two hundred. A form may start at a
later place (`from` above 0) only at the very start, carried over from
the pattern before, or where the notes before it are the ones the
group before shared (an elision). Forms from their first note are tried
first, and only a strictly lower cost replaces a reading, which is how
the ties go to them. Walking `came_from` back from the last group gives
the forms in order.

`forms_of` lists the forms to try, skipping a form that is the same
notes as one already listed, so Webern's palindrome is read with 24
forms, not 48. And `read_rows` runs `read` twice, once on all the
notes and once per track, and keeps the cheaper:

```rust
if apart.iter().map(|(_, reading)| reading.cost).sum::<usize>() < together.cost {
```

The same idea, keeping the best way to reach each point, shares out a
chord's notes among the parts in the classical check ([Voice leading
and part motion](voice-leading.md)).

### Notes that start together: `chunk_by`

The groups come from the notes sorted by start, cut wherever the start
changes:

```rust
for chunk in played.chunk_by(|a, b| a.note.start_tick() == b.note.start_tick()) {
```

`chunk_by` gives slices of neighbours for which the closure says "same
group", without copying anything.

## Testing it

`row.rs` checks the theory against the books: Schoenberg's four forms
note by note against the published ones; every one of the 48 forms of
the Lineup row named again by `label_of`; every line of Berg's matrix a
P form and every column an I form, with the row's first note all down
the diagonal; Webern's row a palindrome (R9 is P3); Berg's row, the All
the intervals row and fifty random all-interval rows all-interval; the
Lineup row combinatorial with I3 only; rows read from names, numbers
and the compact form, and wrong ones refused with the right message;
the same seed giving the same random row; and a row with a note written
twice refused when read from a file:

```rust
#[test]
fn webern_is_a_palindrome() {
    let webern = RowPreset::named("Webern's Symphony").unwrap().row;
    let r9 = webern.form(RowLabel::new(RowForm::Retrograde, 9));
    assert_eq!(webern.label_of(r9).unwrap().to_string(), "P3");
    let mirror = RowPreset::named("Mirror row").unwrap().row;
    let backwards = mirror.form(RowLabel::new(RowForm::Retrograde, 0));
    assert_eq!(mirror.label_of(backwards).unwrap().to_string(), "P6");
}
```

`serial.rs` writes a melody in eighth notes, the four forms in turn
filling the pattern, trichords as chords, the three series, and checks
that no note runs past the end of the pattern. `library/rows.rs`
checks that every style uses a row from the presets and writes its
voices in any key; that Hexachord pads has all twelve notes in every
bar; that a song takes a row style, and that the style's chords replace
the song's on their own channel instead of playing over them. One test
joins the two halves of this chapter: it writes every style, on C and
on F♯, and runs the voice-leading check on it, which must read each one
as forms of its row with no note out of order, so the styles and the
check follow the same rules.

The tests of the rules in `voice_leading/serial.rs` write each rule and
its exceptions as notes on Schoenberg's row: a row in order, chords
from neighbouring notes, two notes swapped, a note left out, repeats
and trills, an elision, a form left unfinished, a tune and chords
sharing a form, a canon, octave doublings, tonal chords, and the
classical rules switched off in rows and sets. Their names read as the
rules: `two_notes_swapped_are_one_note_out_of_order`,
`a_note_may_be_repeated_or_trilled_but_not_brought_back_later`,
`one_form_may_end_on_the_note_the_next_begins_with`.

Exercises on rows are in [Exercises: music theory in
code](exercises.md).
