# lineup-core: notes and tempo

`lineup-core` is the music brain of the project. In step 1 it holds two
small types, `Note` and `Tempo`. They are simple, which makes them a good
place to meet Rust's basic building blocks: structs, `impl` blocks,
associated constants, traits and tests.

## `Note`: a newtype around a number

File: [`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs)

### The music

Music software usually names notes with **MIDI note numbers**, from 0 to
127. Middle C is 60, and each step of 1 is a **semitone** (one piano key,
white or black). Twelve semitones make an **octave**, and the note an
octave higher has exactly twice the frequency. A4, MIDI number 69, is the
tuning note at 440 Hz. Together that gives the formula for
*equal temperament*:

```text
frequency = 440 × 2^((midi − 69) / 12)
```

### The code

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(u8);
```

This is a **tuple struct** with one field, a `u8` (an unsigned 8-bit
integer, 0 to 255). Wrapping a plain number in its own type is called the
**newtype pattern**. It costs nothing at run time, since a `Note` is just a
`u8` in memory, but now the compiler knows a `Note` is not a velocity or a
channel number, and will refuse to mix them up.

The field has no `pub`, so it is private: other crates cannot write
`Note(200)`. They must use `Note::new`, which clamps the value to 127. This
is a common Rust idea: **make invalid values impossible to build**.

`#[derive(...)]` asks the compiler to write some trait implementations for
us automatically:

| Trait | Gives you |
|-------|-----------|
| `Debug` | `{:?}` printing, for debugging |
| `Clone`, `Copy` | the value is copied when assigned, like a number |
| `PartialEq`, `Eq` | `==` and `!=` |
| `PartialOrd`, `Ord` | `<`, `>`, sorting |
| `Hash` | can be used as a key in a `HashMap` |

### `impl` blocks and associated constants

Methods are defined in an `impl` block, separately from the data:

```rust
impl Note {
    pub const C4: Note = Note(60);
    pub const A4: Note = Note(69);

    pub fn new(midi: u8) -> Self { Note(midi.min(Self::MAX.0)) }
    pub fn midi(self) -> u8 { self.0 }
    pub fn frequency(self) -> f32 { ... }
}
```

- `pub const C4` is an **associated constant**, used as `Note::C4`.
- `Self` is a shorthand for "the type this `impl` is for", here `Note`.
- `self.0` reads the first (and only) field of the tuple struct.
- A function whose first parameter is `self` is a **method**, called as
  `note.frequency()`. A function without `self`, like `new`, is an
  **associated function**, called as `Note::new(60)`. Rust has no special
  constructors; `new` is just a naming convention.
- The last expression of a function, written without `;`, is its return
  value. No `return` keyword needed.

The methods take `self` **by value** rather than `&self` (by reference).
That is fine because `Note` is `Copy`: passing it around copies one byte.

### Implementing a trait: `Display`

A **trait** is a set of methods a type can promise to provide, similar to
an interface in other languages. The standard library's `fmt::Display`
trait controls how a value is printed with `{}`:

```rust
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = NOTE_NAMES[self.pitch_class() as usize];
        write!(f, "{}{}", name, self.octave())
    }
}
```

Once `Display` is implemented, `Note::new(61).to_string()` gives `"C#4"`
and `format!("{note}")` works too. The UI uses this to label the test tone
slider.

`as usize` and `as i8` are **casts**. Rust never converts between number
types silently, so you have to say so. Array indexes must be `usize`, the
type Rust uses for sizes and positions in memory.

## `Tempo`: keeping a value valid

File: [`crates/lineup-core/src/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tempo.rs)

```rust
pub struct Tempo { bpm: f32 }

impl Tempo {
    pub const MIN: f32 = 10.0;
    pub const MAX: f32 = 522.0;

    pub fn new(bpm: f32) -> Self {
        if bpm.is_nan() { return Self::default(); }
        Tempo { bpm: bpm.clamp(Self::MIN, Self::MAX) }
    }
}
```

Same idea as `Note`: the `bpm` field is private, so the only way to get a
`Tempo` is through `new`, which clamps it into range. The range 10 to 522
BPM matches the original studio's.

`f32` is a 32-bit floating-point number. Floats have a special
"not a number" value, NaN, produced by things like `0.0 / 0.0`. NaN
compares false with everything, so `clamp` cannot fix it, and we replace it
with the default tempo instead.

### The `Default` trait

```rust
impl Default for Tempo {
    fn default() -> Self { Tempo { bpm: 130.0 } }
}
```

`Default` is the standard trait for "a sensible starting value". Many
library functions and `#[derive(Default)]` rely on it.

## Unit tests

Both files end with a test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4_is_440_hz() {
        assert_eq!(Note::A4.frequency(), 440.0);
    }
}
```

- `#[cfg(test)]` means "only compile this when running tests", so tests
  add nothing to the real program.
- `use super::*;` imports everything from the parent module, including
  private items. Unit tests can therefore test private functions too.
- Each `#[test]` function is one test. It passes if it returns normally
  and fails if it **panics**, which is what `assert!` and `assert_eq!` do
  when their condition is false.

Comparing floats with `==` is risky because of rounding, so most tests
check that the difference is tiny instead:

```rust
assert!((freq - 261.626).abs() < 0.01, "got {freq}");
```

The string after the condition is printed if the test fails.
