# Pitch classes, notes and spelling

Files: [`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs)
and [`crates/lineup-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/pitch_set.rs)

This part of the book is about the music theory inside the program:
scales, chords, numerals, sets, rows and the rules of voice leading,
each one a few small Rust types with methods, tested like any other
code. The theory itself is taught in the user manual's [Learning
harmony](https://naelolaiz.github.io/ferrous-lineup/manual/notes-and-scales.html)
chapters; each chapter here says the musical idea in a few lines and
then shows how the code holds it.

Everything else is built on the types of this chapter: a note, the
twelve keys of an octave, the name a key is written with, and a set of
keys.

## The music, briefly

A piano has 88 keys but only twelve different notes, which repeat in
every octave. The distance from one key to the next, white or black, is
a **semitone**:

```text
 C  C♯ D  D♯ E  F  F♯ G  G♯ A  A♯ B  C
 0  1  2  3  4  5  6  7  8  9  10 11 12
```

A note's name without its octave is its **pitch class**: every C on the
piano has pitch class C. Harmony mostly works with pitch classes; a C
major chord is C, E and G, in any octave and any order. The octave is
written after the name: **C4** is middle C and **A4** the A an
orchestra tunes to, 440 Hz.

Each black key has two names: the key between C and D is C♯ and also
D♭. Which name is right depends on the music around it, and the rule is
that a scale uses each of the seven letters once. F major has a B♭,
never an A♯, because it already has an A. F♯ major's seventh note is
E♯, the key usually called F, because F♯ already has the letter F. Now
and then a note needs two signs: the seventh of G♯ harmonic minor is
F𝄪 (F double sharp, the key usually called G), and the seventh of a C
diminished seventh chord is B𝄫 (B double flat, which sounds as A). The
manual's [Notes, intervals and
scales](https://naelolaiz.github.io/ferrous-lineup/manual/notes-and-scales.html)
tells this more slowly.

So the code needs two different things: which **key** of the piano a
note is, and what it is **called**. They are different types.

## Twelve pitch classes: an enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PitchClass {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}
```

There are exactly twelve pitch classes, so an enum fits: a `PitchClass`
can never be 13 or -1. The variants carry no data (a **fieldless**
enum), and Rust numbers them from 0 in the order they are written. The
`as` keyword turns such an enum into its number, which is the number of
semitones above C:

```rust
pub fn semitone(self) -> u8 {
    self as u8
}
```

Going the other way needs care, because not every number is a pitch
class. `from_semitone` wraps first, then looks the answer up in the
`ALL` array, so it cannot fail:

```rust
pub fn from_semitone(semitone: u8) -> Self {
    Self::ALL[(semitone % 12) as usize]
}
```

`transpose` adds semitones and wraps around the octave. It takes an
`i32` so it can also go down. For a negative number `%` gives a
negative remainder (`-1 % 12` is `-1` in Rust), so it uses
`rem_euclid`, which always gives 0 to 11:

```rust
pub fn transpose(self, semitones: i32) -> Self {
    let wrapped = (self.semitone() as i32 + semitones).rem_euclid(12);
    Self::from_semitone(wrapped as u8)
}
```

`semitones_up_to` answers the opposite question, how far up one pitch
class is from another. From A up to C is 3 semitones (A♯, B, C), and
from C up to A is 9; adding 12 before subtracting keeps the `u8` from
going below zero:

```rust
pub fn semitones_up_to(self, other: PitchClass) -> u8 {
    (other.semitone() + 12 - self.semitone()) % 12
}
```

### Matching on two values at once

The everyday name of a pitch class depends on two things: which one it
is, and whether the black keys are written as sharps or as flats.
`match` can look at both at once when they are put in a **tuple**:

```rust
pub fn name(self, spelling: Spelling) -> &'static str {
    use PitchClass::*;
    use Spelling::*;
    match (self, spelling) {
        (C, _) => "C",
        (CSharp, Sharps) => "C#",
        (CSharp, Flats) => "Db",
        (D, _) => "D",
        // ...
        (B, _) => "B",
    }
}
```

- `use PitchClass::*;` inside the function lets it write `C` instead of
  `PitchClass::C`, without spreading those short names over the whole
  file.
- `_` matches anything: a white key has the same name either way.
- The compiler checks that the arms cover all 24 combinations. Leave out
  `(GSharp, Flats)` and it refuses to build, naming the missing case.
- `&'static str` is a borrowed string that lives as long as the
  program: string literals are stored in the program itself.

`Spelling` is a two-variant enum, `Sharps` (the `#[default]`) and
`Flats`. Which one a key uses, and the finer choice of a letter for each
note, is in [Scales, modes and keys](scales.md).

`is_black_key` uses the `matches!` macro, a short way of asking "does
this value match this pattern?" as a `bool`. The `|` means "or":

```rust
pub fn is_black_key(self) -> bool {
    use PitchClass::*;
    matches!(self, CSharp | DSharp | FSharp | GSharp | ASharp)
}
```

`PitchClass` also has a `Display` implementation, so `{}` in `format!`
writes it with sharps, in letters or in solfège as the settings say
([Two languages: translations, and letters or
solfège](../program/languages.md#two-settings-kept-like-the-numerals-setting)).

## A note: a MIDI number in a type of its own

A note with its octave is stored as its MIDI note number, 0 to 127,
wrapped in a struct of its own:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub struct Note(u8);
```

A struct with one field used to give a value a type of its own is a
**newtype**. A `Note` is a `u8` in memory, but the compiler will not let
a note be passed where a velocity (also a `u8`) is expected, or the
other way round. The field is private, so the only ways to make a note
are the ones `Note` offers, and they keep it in range:

```rust
pub fn new(midi: u8) -> Self {
    Note(midi.min(Self::MAX.0))
}
```

The same goes for files. `#[serde(from = "u8", into = "u8")]` tells
serde to save a note as its number and to read it back through
`From<u8>`, which calls `Note::new`, so not even a hand-edited project
file can hold a note above 127:

```rust
impl From<u8> for Note {
    fn from(midi: u8) -> Self {
        Note::new(midi)
    }
}
```

Some notes have names in the code: `Note::C4` is middle C (60),
`Note::A4` the tuning A (69), `Note::MIN` and `Note::MAX` the ends of
the range.

### Octaves

The pitch class is the number modulo 12, and the octave is the number
divided by 12, minus one, because MIDI note 60 is middle C and middle C
is called C4:

```rust
pub fn octave(self) -> i8 {
    (self.0 / 12) as i8 - 1
}
```

So the lowest MIDI note is C-1 and the highest G9. The octave goes
with the letter, not with the key: it changes between B and C, so the
key a semitone above B3 is C4.

### Moving a note: `Option` for "it might not exist"

A pitch class always has a pitch class seven semitones above it; a note
does not. Seven semitones above G9 is past the top of MIDI. `transpose`
returns an `Option<Note>`, and `None` when the new note would be out of
range:

```rust
pub fn transpose(self, semitones: i32) -> Option<Note> {
    let midi = self.0 as i32 + semitones;
    (0..=Self::MAX.0 as i32)
        .contains(&midi)
        .then_some(Note(midi as u8))
}
```

`contains` asks whether a range holds a number, and `then_some` turns a
`bool` into an `Option`: `Some` of the value if true, `None` if false.
The callers decide what an impossible note means: a chord built near
the top of the keyboard leaves its highest notes out, and the
transpose tool refuses to move a selection at all if one of its notes
would fall off the keyboard, so no note is lost ([Editing tools as
plain functions](tools.md)).

### The frequency

Ferrous Lineup tunes the way pianos are tuned, in **twelve-tone equal
temperament**: every semitone multiplies the frequency by the same
number, the twelfth root of two (about 1.0595), so twelve semitones
exactly double it. A4 is 440 Hz:

```rust
pub fn frequency(self) -> f32 {
    let semitones_from_a4 = self.0 as f32 - Self::A4.0 as f32;
    440.0 * 2.0_f32.powf(semitones_from_a4 / 12.0)
}
```

Middle C is nine semitones below A4, so its frequency is 440 · 2^(-9/12),
about 261.63 Hz. The synth plays this frequency, and a sampler plays a
recording faster or slower by the same rule ([Instruments: drums,
sampler and Lineup Osc](../sound/instruments.md)).

## Letters and names

What a note is called is a letter and some sharps or flats. The letters
are an enum of their own, the seven white keys:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}
```

Letters go round in sevens as pitch classes go round in twelves, so
they have the same kind of methods: the white key's semitone above C,
read from a table, and moving up or down with `rem_euclid`:

```rust
pub fn semitone(self) -> u8 {
    [0, 2, 4, 5, 7, 9, 11][self as usize]
}

pub fn up(self, steps: i32) -> Letter {
    Self::ALL[(self as i32 + steps).rem_euclid(7) as usize]
}
```

Two letters above A is C (A, B, C), and `steps_up_to` counts the other
way: from A to C is 2.

A name is a letter and a count of signs, positive for sharps and
negative for flats:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteName {
    pub letter: Letter,
    /// Sharps if above zero, flats if below: 1 is "#", -2 is "bb".
    pub accidental: i8,
}
```

One `i8` holds both directions, so a name can never be "a sharp and a
flat" at once, and F𝄪 is just `NoteName::new(Letter::F, 2)`. The key of
the piano a name stands for is its letter's white key moved by the
signs:

```rust
pub fn pitch_class(self) -> PitchClass {
    PitchClass::from_semitone(self.letter.semitone()).transpose(i32::from(self.accidental))
}
```

### A name for a key, with a given letter

Most of the time the code knows the key and the letter it wants, and
needs the signs. B with the letter C is C♭ (one flat); F with the letter
E is E♯ (one sharp). `with_letter` counts how far up from the letter's
white key the pitch class is, and takes the shorter way round: up to
five semitones up is that many sharps, six or more is flats below:

```rust
pub fn with_letter(pitch_class: PitchClass, letter: Letter) -> Self {
    let up = (pitch_class.semitone() + 12 - letter.semitone()) % 12;
    // Up to five semitones up is sharps; six or more is flats below.
    let accidental = if up <= 5 { up as i8 } else { up as i8 - 12 };
    NoteName::new(letter, accidental)
}
```

G with the letter F is two up, F𝄪; A with the letter B is ten up, so two
below, B𝄫.

Two simpler names are built on it. `plain` takes the letter from the
everyday name, so it gives C♯ or D♭ for the black key and the white
keys' own letters. `chromatic` is the name used when there is no key to
follow, as in twelve-tone music: C, C♯, D, E♭, E, F, F♯, G, A♭, A, B♭, B,
with sharps for two black keys and flats for the other three:

```rust
pub fn chromatic(pitch_class: PitchClass) -> Self {
    let spelling = match pitch_class {
        PitchClass::CSharp | PitchClass::FSharp => Spelling::Sharps,
        _ => Spelling::Flats,
    };
    NoteName::plain(pitch_class, spelling)
}
```

Choosing the letter is the key's job: each note of a key takes the next
letter, and a note from outside the key takes the letter musicians most
often use for it. That is `Scale::spell`, in [Scales, modes and
keys](scales.md).

### Intervals count letters too

An interval has a size in semitones and a size in letters. A major
third is four semitones and two letters: above C it is E, above A it is
C♯ (not D♭, which is three letters up), and above E♭ it is G. `above`
moves the key by the semitones, the letter by the letters, and lets
`with_letter` find the signs:

```rust
pub fn above(self, semitones: u8, letters: u8) -> NoteName {
    let pitch_class = self.pitch_class().transpose(i32::from(semitones));
    NoteName::with_letter(pitch_class, self.letter.up(i32::from(letters)))
}
```

The diminished seventh is nine semitones and six letters: above B it is
A♭, and above C it is B𝄫. That is how every chord is spelled from its
root ([Chords: building, naming and
inversions](chords.md)), and how a chart is moved to
another key, letter by letter, the way a musician transposes it.

A transposed chord symbol sometimes lands on a name with two signs.
Moving a chart from E minor to G♯ minor moves everything up four
semitones and two letters, so the D♯°7 of E minor becomes F𝄪°7.
That is right on a score, but a chord symbol is read at a glance, so
symbols take the simpler name of the same key:

```rust
pub fn simplified(self) -> NoteName {
    if self.accidental.abs() < 2 {
        return self;
    }
    let spelling = if self.accidental > 0 {
        Spelling::Sharps
    } else {
        Spelling::Flats
    };
    NoteName::plain(self.pitch_class(), spelling)
}
```

E𝄫 becomes D, F𝄪 becomes G, and the D♯°7 above becomes G°7.

### The octave of a spelled note

A note's own name, `Note::name`, uses the everyday name, so middle C is
always "C4". But when a key spells middle C as B♯, it must be written
"B♯3": the octave number goes with the letter, and B♯3 is the B of the
octave below, raised a semitone. In the same way, the B below middle C
written as C♭ is "C♭4". `name_as` takes away the letter and the signs
to find the C at the bottom of the letter's octave, and numbers the
octave from there:

```rust
pub fn name_as(self, name: NoteName) -> String {
    let natural =
        i32::from(self.0) - i32::from(name.letter.semitone()) - i32::from(name.accidental);
    format!("{name}{}", natural.div_euclid(12) - 1)
}
```

For middle C written B♯, that is 60 - 11 - 1 = 48, the MIDI number of
C3, so the name is B♯3. The voice-leading check and the ear trainer
write notes this way.

### Writing and reading a name

Writing a name repeats the sign as many times as the count says:

```rust
pub fn format(self, naming: NoteNaming) -> String {
    let sign = if self.accidental > 0 { "#" } else { "b" };
    let accidentals = sign.repeat(self.accidental.unsigned_abs() as usize);
    match naming {
        NoteNaming::Letters => format!("{}{accidentals}", self.letter.as_char()),
        NoteNaming::Solfege => format!("{}{accidentals}", self.letter.solfege()),
    }
}
```

`unsigned_abs` turns -2 into 2 as an unsigned number, which can never
overflow (`abs` on an `i8` would for -128). With no signs, `repeat(0)`
gives an empty string.

Reading is the other way, and what the app writes it must be able to
read back. In G♯ harmonic minor the chord on the seventh note is
F𝄪°7, saved as `F##dim7`, so `read` takes up to two signs, both in the
same direction. It reads a name at the start of a text and returns the
rest, so `"F#m7"` gives F♯ and `"m7"`:

```rust
let sign = |c| match c {
    '#' | '♯' => 1,
    'b' | '♭' => -1,
    _ => 0,
};
let mut accidental = 0;
for _ in 0..2 {
    let mut chars = rest.chars();
    let step = chars.next().map_or(0, sign);
    if step == 0 || (accidental != 0 && step != accidental) {
        break;
    }
    accidental += step;
    rest = chars.as_str();
}
```

- `sign` is a closure kept in a variable. Its argument's type is not
  written: the compiler works it out from how the closure is used, on
  characters.
- `chars()` walks the text one character at a time, and `as_str()`
  gives back what has not been walked yet. That is the safe way to step
  over "♯", which takes three bytes in UTF-8, where cutting the string
  at a byte count could cut a character in half.
- `map_or(0, sign)` gives 0 when the text has ended, and the sign of the
  next character otherwise.
- The loop stops at a character that is not a sign, at a sign in the
  other direction, and after two signs. So `"Ebbb"` reads E𝄫 and leaves
  `"b"`, and `"F#b"` reads F♯ and leaves `"b"`.

The letter itself must be a capital. `Letter::from_char` refuses `'c'`,
because in a chord symbol a small "b" is a flat: "Bb" is B♭, not two
B's. The same reading in solfège, "Sib" for B♭, and how the app reads a
name typed either way, are in [Two languages: translations, and
letters or solfège](../program/languages.md#reading-a-name-either-way).

## A set of pitch classes in twelve bits

Chords and scales are sets of pitch classes. A set of at most twelve
things fits in the bits of one integer: bit 0 means "C is in the set",
bit 1 "C♯ is in the set", and so on up to bit 11 for B.

```rust
pub struct PitchSet(u16);
```

```rust
pub fn insert(&mut self, pitch_class: PitchClass) {
    self.0 |= 1 << pitch_class.semitone();
}

pub fn contains(self, pitch_class: PitchClass) -> bool {
    self.0 & (1 << pitch_class.semitone()) != 0
}
```

`1 << n` is the number with only bit `n` set (1 shifted left by `n`
places). `|=` sets that bit, and `&` keeps only the bits two numbers
share, so `x & (1 << n) != 0` asks whether bit `n` is set. A C major
chord, C, E and G, is bits 0, 4 and 7:

```text
bit:       11 10  9  8  7  6  5  4  3  2  1  0
           B  A♯ A  G♯ G  F♯ F  E  D♯ D  C♯ C
C major:   0  0  0  0  1  0  0  1  0  0  0  1   = 0b0000_1001_0001
```

Other set questions are one instruction each. `count_ones()` counts the
members, and `a & !b == 0` asks whether every member of `a` is also in
`b` (`!` flips every bit, so `a & !b` is what `a` has and `b` lacks):

```rust
pub fn is_subset_of(self, other: PitchSet) -> bool {
    self.0 & !other.0 == 0
}
```

That is how the code asks whether a chord belongs to a key: A minor's
A, C and E are all in the C major scale, E major's G♯ is not. And
because a set is one small number, comparing two chords is a single
`==`, and a set can be a key in a `HashMap`.

`iter` gives the members from C upwards by keeping the pitch classes
the set contains:

```rust
pub fn iter(self) -> impl Iterator<Item = PitchClass> {
    PitchClass::ALL
        .into_iter()
        .filter(move |&pitch_class| self.contains(pitch_class))
}
```

`move` gives the closure its own copy of `self` (a `PitchSet` is
`Copy`), because the iterator is returned and outlives the call.

### Seeing a chord from its root

The trick behind naming chords is moving a whole set up or down. In
bits, that is a **rotation** of the twelve bits: the ones that go past
B come back in at C.

```rust
pub fn transpose(self, semitones: u8) -> PitchSet {
    let shift = (semitones % 12) as u32;
    let bits = self.0 as u32;
    // Rotate the 12 bits left by `shift`: bits that go past B come
    // back in at C.
    let rotated = (bits << shift) | (bits >> (12 - shift));
    PitchSet(rotated as u16 & ALL_BITS)
}
```

`bits << shift` moves every member up; the ones that went past bit 11
are dropped by the mask `ALL_BITS` (twelve ones). `bits >> (12 - shift)`
brings exactly those back in at the bottom. C major moved up by 9 has
A (bit 9), C♯ (bit 13, back in as bit 1) and E (bit 16, back in as bit
4): A major.

`relative_to` moves a set down until a given root becomes C:

```rust
pub fn relative_to(self, root: PitchClass) -> PitchSet {
    self.transpose(12 - root.semitone())
}
```

E minor, E G B, seen from E is C D♯ G: semitones 0, 3 and 7, the shape
of every minor triad. Seen from its root, every chord of one kind is
the same number, whatever its root. [Chords: building, naming and
inversions](chords.md) names a chord by finding the
root from which its set matches a known shape.

### Making `collect` build a `PitchSet`

`collect()` can build any type that implements the `FromIterator`
trait; `Vec`, `String` and `HashMap` do. Implementing it for `PitchSet`
lets any code turn notes into a set in one line, such as
`notes.map(|note| note.pitch_class()).collect::<PitchSet>()`:

```rust
impl FromIterator<PitchClass> for PitchSet {
    fn from_iter<I: IntoIterator<Item = PitchClass>>(iter: I) -> Self {
        let mut set = PitchSet::EMPTY;
        for pitch_class in iter {
            set.insert(pitch_class);
        }
        set
    }
}
```

`from_semitones(&[0, 4, 7])` builds a set from numbers the same way,
by mapping each one to a pitch class and collecting.

### In a file

A project file stores a set as its semitones above C, such as
`"0 3 6 9"`, which is easier to read and to write by hand than the
number 585. The attribute `#[serde(try_from = "String", into =
"String")]` saves it through a `String` and reads it back through
`TryFrom<String>`, which refuses anything that is not a number from 0
to 11. The same pair of conversions, and why the reading one is
`try_from`, is in [Twelve-tone rows and their
rules](twelve-tone.md#files-try_from-and-into).

## Testing it

The tests in `pitch.rs` and `pitch_set.rs` read like the rules of this
chapter. Names and octaves, at both ends of the range:

```rust
assert_eq!(Note::new(61).to_string(), "C#4");
assert_eq!(Note::new(61).name(Spelling::Flats), "Db4");
assert_eq!(Note::MIN.to_string(), "C-1");
assert_eq!(Note::MAX.to_string(), "G9");
```

Names with any letter, and the chromatic names:

```rust
assert_eq!(name(F, Letter::E), "E#");
assert_eq!(name(B, Letter::C), "Cb");
assert_eq!(name(G, Letter::F), "F##");
assert_eq!(name(A, Letter::B), "Bbb");
```

```rust
assert_eq!(chromatic.join(" "), "C C# D Eb E F F# G Ab A Bb B");
```

Intervals counted in letters, and the octave of a spelled note:

```rust
// A major third above A is C#, not Db.
assert_eq!(a.above(4, 2).to_string(), "C#");
```

```rust
assert_eq!(c4.name_as(NoteName::new(Letter::B, 1)), "B#3");
assert_eq!(Note::new(59).name_as(NoteName::new(Letter::C, -1)), "Cb4");
```

And reading, including what must not be read:

```rust
assert_eq!(read("F##dim7"), Some(("F##".to_string(), "dim7")));
assert_eq!(read("Ebbb"), Some(("Ebb".to_string(), "b")), "two at most");
assert_eq!(read("F#b"), Some(("F#".to_string(), "b")), "not mixed");
assert_eq!(read("c"), None, "lower case is a flat, not a note");
```

The sets' tests check that transposing wraps around (C major up 9 is A
major, up 12 is itself) and that E minor seen from E is the set
`0 3 7`.

Exercises on these types are in [Exercises: music theory in
code](exercises.md).
