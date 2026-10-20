# Chord names: reading a lead sheet

Files: [`crates/lineup-core/src/harmony/symbol.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/symbol.rs),
[`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs),
[`crates/lineup-core/src/library/numeral.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/numeral.rs),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs),
[`crates/lineup-core/src/library/voicing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/voicing.rs)
and [`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs)

Until now every chart in Ferrous Lineup was written in Roman numerals:
"I | V | vi | IV". That is how the library stores progressions, because
one chart then works in every key. But a song you want the band to play
comes on a **lead sheet**, and a lead sheet names its chords: "C | G/B |
Am | Am/G". Before there can be a chord sheet to type into (the next
change), Ferrous Lineup has to read chord names the way musicians write
them. That is all this change does; there is nothing new on the screen
yet.

## What a chord symbol says

A chord symbol has up to three parts:

| Part | Example | What it says |
|------|---------|--------------|
| Root | the **F#** of F#m7b5 | The note the chord is built on: a capital letter, maybe a sharp or a flat. |
| Quality | the **m7b5** of F#m7b5 | What kind of chord: minor, dominant seventh, half-diminished... |
| Bass | the **/B** of G/B | The lowest note, when it is not the root. |

Musicians write the same quality in several ways, and all of these are
read:

| Chord | Written as |
|-------|-----------|
| major | C, Cmaj, CM |
| minor | Cm, Cmin, Cmi, C- |
| minor seventh | Cm7, Cmin7, C-7 |
| major seventh | Cmaj7, CM7, CΔ, CΔ7 |
| half-diminished | Cm7b5, C-7b5, Cø, Cø7 |
| diminished seventh | Cdim7, C°7, Co7 |
| augmented | Caug, C+ |
| dominant seventh with a sharp fifth | C7#5, C+7, Caug7 |
| minor with a major seventh | Cm(maj7), CmM7, C-Δ7 |
| suspended fourth | Csus4, Csus |
| six-nine | C6/9, C69 |
| altered ninths | C7b9, C7(b9), C7#9 |

Brackets and spaces are ignored, and the flat and sharp signs ♭ and ♯
work as well as `b` and `#`. Upper and lower case matter in one place
only, but it is an important one: **CM7** is a major seventh (C E G B)
and **Cm7** a minor seventh (C Eb G Bb). When Ferrous Lineup writes a chord
back, it uses one way for each, the one the piano roll and the harmony
keys already use: Cmaj7, Cm7, Cm7b5, Cdim7.

A root is always a capital letter, because a lower-case "b" right after
it is a flat: "Bb7" is B flat seven. "H", which German books use for B,
is not read.

## Slash chords

A slash puts another note in the bass. There are two kinds:

- **Inversions**: the bass note is in the chord. C/E is C major with its
  **third** in the bass (the first inversion), C/G with its **fifth**
  (the second inversion), and G7/F has the **seventh** of G7 in the bass
  (the third inversion). `ChordSymbol::bass_in_chord` says which: 3, 5
  or 7.
- **A chord over a bass note**: the bass note is not in the chord. F/G is
  F major (F A C) over a G, a soft, open sound that pop and gospel
  players use in place of G11. `bass_in_chord` says `None`.

Most slash chords are written for the bass line they make. In "C | G/B |
Am | Am/G | F | C/E | Dm | G" each chord on its own is ordinary, but
the bass walks down a scale: C, B, A, G, F, E, D. That line is the
reason for the slashes, and it is what the band now plays (see
[below](#the-band-plays-the-slash)).

One slash is not a bass note: the one in **C6/9**, the six-nine chord.
Ferrous Lineup only takes what follows the last slash as a bass if it is a
note name, so "C6/9" is a six-nine chord and "C6/9/E" is a six-nine
chord over E.

## Spelling

Db7 and C#7 are played on the same keys of the piano, but they are not
written the same way. A chord is spelled up from its root by letters:
the third is two letters up, the fifth four, the seventh six.

| Chord | Root | Third | Fifth | Seventh |
|-------|------|-------|-------|---------|
| C#7 | C# | E# | G# | B |
| Db7 | Db | F | Ab | Cb |

The seventh of Db7 is Cb, not B: six letters above D is C, and the note
a minor seventh above Db, written with a C, is C flat. `ChordSymbol`
keeps the root exactly as it was typed, so `notes()` spells the chord
the way the lead sheet means it. For playing, both become the same
`Chord`, a pitch class and a quality.

## Transposing a chart

To move a song to another key, a musician moves every chord by the same
distance, counted in **letters** as well as semitones. From E minor to
D minor every letter goes one down:

| E minor | F#m7b5 | B7 | Em |
|---------|--------|----|----|
| D minor | Em7b5 | A7 | Dm |
| C minor | Dm7b5 | G7 | Cm |
| Bb minor | Cm7b5 | F7 | Bbm |
| G# minor | A#m7b5 | D#7 | G#m |

This is a **ii–V–i**, the most common way to arrive in a minor key: in
every key the ii is one letter above the tonic and the V four letters
above. Counting by letters keeps it that way: in G# minor the V is D#7,
never Eb7. A test checks all twelve keys, written with sharps and with
flats.

`transposed(from, to)` does it: the interval from the old tonic to the
new one, as semitones and letters, is added to the root and to the bass
with `NoteName::above`, the same function that spells chords.

One thing is done differently from a score. A chord symbol never has a
double sharp or a double flat on its root: D#°7 in E minor moved to G#
minor would strictly be F##°7, and Ferrous Lineup writes it G°7, the same
keys, as lead sheets do. In a score, where every note of the chord is
written out, F##°7 would be right.

## From names to numerals, and back

The library still thinks in numerals, so a chord by name becomes a
`RomanChord` once the key is known: in A minor, Dm7 is five semitones
above the tonic (iv7), and E7/G# is seven, with its bass eleven.
`RomanChord` gained a `bass` field for that. The way back,
`RomanChord::symbol_in(key)`, spells the chord in a key: the same numerals
are A7/C# in D minor, because the third of A7 is C#, not Db.

Charts can now name their chords. `parse_chart_in` reads a chart like
`parse_chart` does, but a chord starting with a capital letter from A to
G is read as a chord name, counted from the tonic it is given. The two
can be mixed, since a numeral never starts with a capital A to G:
"I | G7/B | vi".

## The band plays the slash

`accompany`, which turns a chart into chords and a bass line, now plays
a slash chord's bass note wherever the bass would play the root. The
chords above stay the chord's own: G/B is still G, B and D in the piano.
The other notes of a bass pattern (the third, the fifth, the seventh)
are found just above the bass note, and "the fifth below" just under it.

Then there is the question of octaves. Roots have always been played
between C2 and B2, which is fine for roots but breaks the line above:
C2 then B2 is a jump up a seventh, not a step down. So around slash
chords the bass goes the short way: the bass of a slash chord, and the
chord just after one, are played a step (one or two semitones) from the
note before when there is one, down to E1, the lowest string of a bass
guitar.

| Chord | C | G/B | Am | Am/G | F | C/E | Dm | G |
|-------|---|-----|----|------|---|-----|----|---|
| Bass note | C2 | B1 | A1 | G1 | F1 | E1 | D2 | G2 |

D1 would be below a bass guitar, so the line jumps up to D2 there. A
chart with no slash chords plays exactly as before: every root between
C2 and B2. A test checks that, and every one of the 145 styles is still
heard as its own chart.

You cannot hear it in the app yet; the chord sheet comes next. Until
then the tests show it working:

```sh
cargo test -p lineup-core symbol
cargo test -p lineup-core slash
```

## The code

### Reading a note and keeping the rest

`NoteName::read` reads the note at the start of a chord symbol and hands
back what is left, so the next step can go on from there:

```rust
pub fn read(text: &str) -> Option<(NoteName, &str)> {
    let mut chars = text.chars();
    let letter = Letter::from_char(chars.next()?)?;
    let rest = chars.as_str();
    let mut chars = rest.chars();
    let accidental = match chars.next() {
        Some('#' | '♯') => 1,
        Some('b' | '♭') => -1,
        _ => return Some((NoteName::new(letter, 0), rest)),
    };
    Some((NoteName::new(letter, accidental), chars.as_str()))
}
```

`text.chars()` walks the text one character at a time, and
`chars.as_str()` is the part not walked yet. That `&str` is not a copy:
it points into the same text, a little further along. The return type
says so: the `&str` in `Option<(NoteName, &str)>` borrows from `text`,
and Rust works out by itself that it may not outlive it.

Walking by `char` matters here. "♭" takes three bytes in UTF-8, so
cutting the text at "one byte after the letter" would cut it in the
middle of a character, and Rust would panic. `chars()` always moves by
whole characters.

### `rsplit_once` and the six-nine chord

```rust
let (quality_text, bass) = match rest.rsplit_once('/') {
    Some((before, after)) => match NoteName::parse(after.trim()) {
        Some(bass) => (before, Some(bass)),
        None => (rest, None),
    },
    None => (rest, None),
};
```

`rsplit_once('/')` splits the text at its **last** slash into the part
before and the part after, or gives `None` when there is no slash. Only
when the part after is a note name is it a bass; otherwise the slash
belongs to the quality, as in "6/9", and the whole rest is the quality.

### A table of spellings

The ways of writing each quality are a constant table:

```rust
const SPELLINGS: [(ChordQuality, &[&str]); 30] = { ... };
```

Each entry is a tuple of a quality and a slice of strings, `&[&str]`:
slices can have different lengths, so one quality can have one spelling
and another six, in the same array. Reading a quality is a search:

```rust
SPELLINGS
    .iter()
    .find(|(_, spellings)| spellings.contains(&plain.as_str()))
    .map(|(quality, _)| *quality)
```

`contains` wants a reference to an item of the slice, a `&&str`, which
is what `&plain.as_str()` is. A test writes every quality on every root,
with and without a bass, and reads it back, so the table and
`ChordQuality::symbol` cannot drift apart.

### `FromStr`: `"Am7".parse()`

```rust
impl FromStr for ChordSymbol {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ChordSymbol::parse(text)
    }
}
```

`FromStr` is the standard trait for "can be read from text". Once
`ChordSymbol` has it, `str::parse` works for it, as it does for numbers:
`let chord: ChordSymbol = "Am7".parse()?;`. The error is a `String` that
says what went wrong, ready to show next to the bar where it was typed.

### Passing the reader in

`parse_chart` and `parse_chart_in` share one function, `read_chart`,
which takes the way to read one chord as a closure:

```rust
fn read_chart(
    text: &str,
    bar: usize,
    read: impl Fn(&str) -> Option<RomanChord>,
) -> Option<Vec<ChordSpan>>
```

`parse_chart` passes `numeral::parse`, a plain function, and
`parse_chart_in` a closure that also looks at the tonic it was given. A
function and a closure are both `Fn`, so `read_chart` does not mind
which. Inside, each bar is read with
`cell.split_whitespace().map(&read).collect::<Option<Vec<_>>>()`: one
chord that cannot be read makes the whole bar `None`, and the `?` after
it makes the whole chart `None`.

## Exercises

1. Pop charts sometimes write **Cadd2**, which has the same notes as
   Cadd9 with the D played low. Add "add2" to `SPELLINGS`. What is lost
   by reading it as `Add9`?
2. **C5** is a "power chord": just C and G, no third. It is not in
   `ChordQuality`. What else in Ferrous Lineup would have to know about it
   (look at `ChordQuality::family`, the harmony keys and `is_minor`)?
3. The **line cliché** keeps one chord while a single voice moves down
   by semitones: "Am | Am/G# | Am/G | Am/F#". Read it with
   `parse_chart_in`, play it with `accompany` and `Bass::Held`, and check
   the bass notes. Is G# in A minor? What is the chord really, in the
   names of this page?
