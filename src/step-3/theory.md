# Music theory as types: pitch classes and scales

Files: [`crates/loom-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/pitch.rs),
[`crates/loom-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/harmony/pitch_set.rs)
and [`crates/loom-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/harmony/scale.rs)

## The music

A piano has 88 keys, but only **twelve different note names** that
repeat every octave: C, C#, D, D#, E, F, F#, G, G#, A, A#, B, and then C
again, one octave higher. A note's name without its octave is its
**pitch class**. Harmony mostly works with pitch classes: a C major chord
is "C, E and G", in any octave and in any order.

The distance between two neighbouring keys (white or black) is a
**semitone**. Everything in this chapter is counting semitones:

```text
 C  C# D  D# E  F  F# G  G# A  A# B  C
 0  1  2  3  4  5  6  7  8  9  10 11 12
```

A **scale** is a choice of some of the twelve, starting from a **root**.
The major scale is the white keys from C: C D E F G A B. As semitones
above the root, that is `0, 2, 4, 5, 7, 9, 11`. The same pattern started
on another note gives the major scale of that key: from D, it is
D E F# G A B C#.

The black keys have two names. The key between A and B is A# ("A sharp",
a semitone above A) and also Bb ("B flat", a semitone below B). Which
name is right depends on the key: F major's notes are F G A **Bb** C D E,
because each letter is used once. Ferrous Loom follows that rule in a
simple way: keys whose signature has flats (F, Bb, Eb, Ab, Db and their
minor relatives) are written with flats, all others with sharps.

## An enum for the twelve pitch classes

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PitchClass {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}
```

There are exactly twelve pitch classes, so an enum fits perfectly: a
`PitchClass` can never be "13" or "-1". The variants have no data
(a *fieldless* enum), and Rust numbers them from 0 in the order they are
written. The `as` keyword turns a fieldless enum into that number:

```rust
pub fn semitone(self) -> u8 {
    self as u8 // C is 0, CSharp is 1, ..., B is 11
}
```

Going the other way needs care, because not every number is a pitch
class. `from_semitone` wraps first, then looks the answer up in the
`ALL` array, so it can never fail:

```rust
pub const ALL: [PitchClass; 12] = [PitchClass::C, PitchClass::CSharp, /* ... */ PitchClass::B];

pub fn from_semitone(semitone: u8) -> Self {
    Self::ALL[(semitone % 12) as usize]
}
```

`transpose` adds semitones and wraps around the octave. It takes an
`i32` so it can also go down. For negative numbers `%` would give a
negative remainder (`-1 % 12` is `-1` in Rust), so it uses
`rem_euclid`, which always gives `0..12`:

```rust
pub fn transpose(self, semitones: i32) -> Self {
    let wrapped = (self.semitone() as i32 + semitones).rem_euclid(12);
    Self::from_semitone(wrapped as u8)
}
```

## Matching on two values at once

The name of a pitch class depends on two things: which one it is, and
whether the key uses sharps or flats. `match` can look at both at once
if you put them in a **tuple**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Spelling {
    #[default]
    Sharps,
    Flats,
}

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

Things to notice:

- `use PitchClass::*;` inside the function brings the variants into
  scope, so you can write `C` instead of `PitchClass::C`. Doing it inside
  the function keeps those short names from leaking into the whole file.
- `_` matches anything: white keys have the same name either way.
- The compiler checks that the arms cover all 24 combinations. Forget
  `(GSharp, Flats)` and it refuses to build, naming the missing case.
- `#[derive(Default)]` on an enum needs `#[default]` on one variant, the
  one `Spelling::default()` returns.
- `&'static str` is a borrowed string that lives for the whole program:
  string literals are stored in the program itself.

`is_black_key` uses the `matches!` macro, a short way to write "does
this value match this pattern?" as a `bool`. The `|` means "or":

```rust
pub fn is_black_key(self) -> bool {
    use PitchClass::*;
    matches!(self, CSharp | DSharp | FSharp | GSharp | ASharp)
}
```

`Note` (the MIDI note number from step 1) now returns a `PitchClass`
from `pitch_class()`, and gained `transpose` (which returns `None` if
you would leave the MIDI range) and `name(spelling)`, which gives
names like "Bb3".

## A set of pitch classes in twelve bits

Chords and scales are *sets* of pitch classes. A set of at most twelve
things fits in the bits of one integer: bit 0 means "C is in the set",
bit 1 means "C# is in the set", and so on.

```rust
pub struct PitchSet(u16);

pub fn insert(&mut self, pitch_class: PitchClass) {
    self.0 |= 1 << pitch_class.semitone();
}

pub fn contains(self, pitch_class: PitchClass) -> bool {
    self.0 & (1 << pitch_class.semitone()) != 0
}
```

`1 << n` is the number with only bit `n` set (shift 1 left by `n`
places). `|=` sets that bit, and `&` keeps only the bits two numbers
share, so `x & (1 << n) != 0` asks "is bit `n` set?". C major (C, E, G)
is bits 0, 4 and 7:

```text
bit:       11 10  9  8  7  6  5  4  3  2  1  0
           B  A# A  G# G  F# F  E  D# D  C# C
C major:   0  0  0  0  1  0  0  1  0  0  0  1   = 0b0000_1001_0001
```

Other set operations are one instruction each: `count_ones()` counts
the members, and `a & !b == 0` asks "is every member of `a` also in
`b`?" (`!` flips every bit).

### Seeing a chord from its root

The key trick for naming chords is **transposing a set**: moving every
pitch class up or down by the same amount. In bits, that is a *rotation*
of the twelve bits, where bits that fall off one end come back in at the
other:

```rust
pub fn transpose(self, semitones: u8) -> PitchSet {
    let shift = (semitones % 12) as u32;
    let bits = self.0 as u32;
    let rotated = (bits << shift) | (bits >> (12 - shift));
    PitchSet(rotated as u16 & ALL_BITS) // keep only the 12 low bits
}

pub fn relative_to(self, root: PitchClass) -> PitchSet {
    self.transpose(12 - root.semitone())
}
```

`relative_to(root)` moves the set down until `root` becomes C. E, G, B
seen from E becomes C, D#, G, which is `0, 3, 7`: exactly the minor
triad's shape. Comparing a set seen from its root with each chord shape
is how the next chapter names chords.

### Making `collect` build a `PitchSet`

`collect()` can build any type that implements the `FromIterator` trait.
`Vec`, `String` and `HashMap` do. Implementing it for `PitchSet` lets
the rest of the code write `notes.map(|note| note.pitch_class()).collect()`
and get a `PitchSet`:

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

## Scales

```rust
pub enum ScaleKind {
    Major, NaturalMinor, HarmonicMinor, MelodicMinor,
    Dorian, Phrygian, Lydian, Mixolydian, Locrian,
    MajorPentatonic, MinorPentatonic, Blues, Chromatic,
}

pub fn intervals(self) -> &'static [u8] {
    match self {
        ScaleKind::Major => &[0, 2, 4, 5, 7, 9, 11],
        ScaleKind::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
        ScaleKind::MajorPentatonic => &[0, 2, 4, 7, 9],
        // ...
    }
}
```

Scales have different numbers of notes (five for pentatonic, seven for
major, twelve for chromatic), so `intervals` returns a **slice**,
`&[u8]`, which can have any length. `&[0, 2, 4, 5, 7, 9, 11]` is a
borrowed array that the compiler stores in the program, so it is
`'static`: no allocation, ever.

The modes (Dorian, Phrygian and so on) are the major scale started on
another of its notes: D Dorian is D E F G A B C, the white keys from D.
A test checks exactly that.

A `Scale` is a root plus a kind, and most of its methods are short
iterator chains:

```rust
pub struct Scale {
    pub root: PitchClass,
    pub kind: ScaleKind,
}

pub fn pitch_classes(self) -> impl Iterator<Item = PitchClass> {
    self.kind
        .intervals()
        .iter()
        .map(move |&interval| self.root.transpose(interval as i32))
}

pub fn degree_of(self, pitch_class: PitchClass) -> Option<usize> {
    self.pitch_classes().position(|p| p == pitch_class)
}
```

- `impl Iterator<Item = PitchClass>` in the return type means "some
  iterator of pitch classes; the caller does not need to know which".
  The real type is a long name involving `Map<Iter<u8>, {closure}>`, and
  closures have types you cannot even write. `impl Trait` hides it.
- `move` makes the closure take its own copy of `self` (a `Scale` is
  `Copy`). Without it, the closure would borrow `self`, a local variable
  of `pitch_classes`, and could not be returned from it.
- `|&interval|` is a pattern in the closure's parameter: the iterator
  gives `&u8`, and `&interval` takes the `u8` out of the reference.
- `position` returns the index of the first item that matches, as an
  `Option`. The **degree** of a note is its position in the scale, so
  in A minor, E is degree 4 (counting from 0; musicians say "the fifth").

### Which spelling does a key use?

```rust
pub fn spelling(self) -> Spelling {
    use PitchClass::*;
    match self.root.transpose(self.kind.relative_major() as i32) {
        F | ASharp | DSharp | GSharp | CSharp => Spelling::Flats,
        _ => Spelling::Sharps,
    }
}
```

Every minor key and mode shares its key signature with a major key: A
minor uses the same notes as C major, D Dorian too. `relative_major`
says how far up that major key is (3 semitones for minor, 10 for Dorian),
and the `match` decides by that major key. Or-patterns (`F | ASharp`)
work in `match` arms as well as in `matches!`.

### The chords of a key

To build the chord on a degree, take every other note of the scale:
degree, degree + 2, degree + 4 (and + 6 for a seventh chord). This is
called **stacking thirds**. In C major, starting on D gives D, F, A:
D minor.

```rust
pub fn stack_thirds(self, note: Note, count: usize) -> Option<Vec<Note>> {
    if !self.is_heptatonic() {
        return None;
    }
    let intervals = self.kind.intervals();
    let degree = self.degree_of(note.pitch_class())?;
    let notes = (0..count)
        .map(|i| {
            let index = degree + 2 * i;
            // Going past the seventh note wraps into the next octave.
            let octave = (index / 7) as i32;
            let above_root = intervals[index % 7] as i32 + 12 * octave;
            above_root - intervals[degree] as i32
        })
        .filter_map(|semitones| note.transpose(semitones))
        .collect();
    Some(notes)
}
```

Read it as a pipeline: the numbers `0..count` become scale positions,
the positions become semitones above `note`, and `filter_map` turns the
semitones into notes, dropping any that would fall off the top of the
MIDI range (`transpose` returns `None` for those). `?` after `degree_of`
returns `None` early if the note is not in the scale.

Doing this for all seven degrees of C major gives the chords every
songwriter learns:

| Degree | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|--------|---|---|---|---|---|---|---|
| Triad | C | Dm | Em | F | G | Am | Bdim |
| Numeral | I | ii | iii | IV | V | vi | vii° |
| Seventh | Cmaj7 | Dm7 | Em7 | Fmaj7 | G7 | Am7 | Bm7b5 |

The piano roll's "Chords in C major" buttons are this table. Change the
key and the buttons follow, because they are computed, not written down.
