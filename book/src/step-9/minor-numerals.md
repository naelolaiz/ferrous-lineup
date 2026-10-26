# Roman numerals in minor keys: bIII or III

Files: [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/library/numeral.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/numeral.rs),
[`crates/lineup-app/src/app/settings.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/settings.rs)
and [`crates/lineup-app/src/app/video.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/video.rs)

In A minor, the chord C major has two names. A jazz or pop book calls it
**bIII**; a classical harmony book calls it **III**. Both are right.
Until now Ferrous Lineup only wrote the first. If you study from a
classical book, every minor-key numeral the app showed you on the third,
sixth or seventh degree looked wrong, though it was not.

**Edit > Settings… > Harmony > Roman numerals in minor keys** now lets
you choose. The chords don't change, only how they are written.

![Settings > Harmony: two ways to count the degrees of a minor key, with A minor as the example.](../images/step-9-minor-numerals-setting.png)

## Two ways to count

A Roman numeral says which degree of the key a chord is built on: I on
the first note, V on the fifth. Its case says the chord's quality:
upper case for major (V), lower case for minor (ii), ° for diminished
(vii°), + for augmented (III+). The question is what the plain numeral
means: built on **which** note?

Take A minor. Its natural minor scale is A B C D E F G, and its chords
are Am, B°, C, Dm, Em, F and G.

**Counted from the major scale.** The numerals are measured against
the major scale on the same tonic, A major: A B C# D E F# G#. C is a
semitone below C#, so its chord is a **flat** III: bIII. In the same way
F is bVI and G is bVII:

```text
Am   B°    C     Dm   Em   F     G
i    ii°   bIII  iv   v    bVI   bVII
```

The flats say "lower than in major". The advantage is that one table of
progressions works in every key and mode: "bVI bVII i" is the same
progression in A minor, A Dorian or A Aeolian, and a song in C major
that borrows Ab and Bb from C minor writes them bVI and bVII too. This
is how most pop and jazz books, and the app's library, write numerals.

**Counted from the minor scale.** The numerals are measured against
the key's own scale, the one its key signature gives. C is simply the
third note of A minor, so its chord is III:

```text
Am   B°    C     Dm   Em   F     G
i    ii°   III   iv   v    VI    VII
```

This is how classical harmony books write them. III is the relative
major, the key a minor-key piece most often goes to.

Only three degrees are written differently, the third, the sixth and the
seventh. These are the notes that differ between A major and A minor.
The first, second, fourth and fifth are the same in both scales, so
their numerals are the same either way.

### The sixth and seventh have two forms

In a minor key the sixth and seventh notes move. Harmonic minor raises
the seventh (G becomes G#), so that the chord on the fifth degree is
E major, with a leading tone, rather than E minor. Melodic minor, going
up, raises the sixth too (F becomes F#).

So in A minor there are two G's and two F's, and a plain numeral has to
name one of them. The rule the app follows is the one classical books
use: **a plain numeral names the form whose chord belongs to the key.**

- **VII** is G major (G B D), from natural minor. **vii°** is G#°
  (G# B D), the leading-tone chord of harmonic minor. G° (G B♭ D♭) is
  no chord of A minor, so vii° can't mean that.
- **VI** is F major, from natural minor. **vi°** is F#° (F# A C), from
  melodic minor.

The same chords, counted from the major scale, are bVII, vii°, bVI and
vi°, since G# and F# are the notes of A major. So the leading-tone
chord is vii° in both ways, and E major is V in both. Here are the
triads of the three minor scales, written both ways:

```text
                     from the major scale           from the minor scale
natural minor   i ii° bIII  iv v  bVI bVII     i ii° III  iv v  VI  VII
harmonic minor  i ii° bIII+ iv V  bVI vii°     i ii° III+ iv V  VI  vii°
melodic minor   i ii  bIII+ IV V  vi° vii°     i ii  III+ IV V  vi° vii°
```

A chord outside the key gets an accidental against the form a plain
numeral names. Bb major in A minor, the Neapolitan, is **bII** both
ways. Counted from the minor scale, G° is **bvii°** (G#° lowered) and
G# major is **#VII** (raised above both forms of G).

The choice applies to major keys and to the modes not at all: C major
is I ii iii IV V vi vii° either way, and E Phrygian keeps its bII.

## What the setting changes, and what it doesn't

The setting changes how numerals are **written** and **read**: on the
harmony keys, under the chords of the chord sheet, in the piano roll,
in the hints, in ear training, in the style tooltips and in an exported
video. Numerals you type or paste on the chord sheet are read the same
way, so with "from the minor scale" chosen, typing `VII` in A minor
gives G major.

It does not change any chord. What a song stores is the chord itself (G
major), never its numeral, so a song reads the same whichever way you
choose, and switching back and forth changes nothing in it. The
library's progressions are always read counted from the major scale,
because that is how they are written: a style's Progression box shows
`i | bVII | bVI | V`, for example, and it plays the same chords for
everyone.

![El Choclo, in A harmonic minor, counted from the minor scale: G7 and C are VII7 and III on the chord sheet, and the harmony keys say III+, VI and vii°.](../images/step-9-minor-numerals-keys.png)

## In the code

### The choice is an enum

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MinorNumerals {
    /// From the major scale on the same tonic: i, ii°, bIII, iv, v, bVI,
    /// bVII. …
    #[default]
    FromMajor,
    /// From the minor scale itself: i, ii°, III, iv, v, VI, VII, as
    /// classical harmony books write them. …
    FromMinor,
}
```

`#[default]` on a variant tells `#[derive(Default)]` which variant
`MinorNumerals::default()` is. `Serialize` and `Deserialize` let the
Settings window save it with egui's `insert_persisted`, as the Library's
options are saved.

### Which note a plain numeral names

Writing a numeral used to measure the chord's root against the major
scale. Now it measures it against `unaltered_degree`, the note the plain
numeral names:

```rust
pub(crate) fn unaltered_degree(
    self,
    letters: usize,
    quality: ChordQuality,
    numerals: MinorNumerals,
) -> u8 {
    use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
    if numerals == MinorNumerals::FromMajor || !self.is_minor_key() {
        return ScaleKind::Major.tonal_intervals()[letters];
    }
    let natural = NaturalMinor.tonal_intervals()[letters];
    if letters < 5 {
        return natural;
    }
    // Whether the chord on `semitones` is in one of these scales,
    // counted from C, since only the distance from the tonic counts.
    let of_key = |semitones: u8, kinds: &[ScaleKind]| {
        let chord = Chord::new(PitchClass::C.transpose(i32::from(semitones)), quality);
        kinds
            .iter()
            .any(|&kind| Scale::new(PitchClass::C, kind).is_diatonic(chord))
    };
    if !of_key(natural, &[NaturalMinor]) && of_key(natural + 1, &[HarmonicMinor, MelodicMinor])
    {
        natural + 1
    } else {
        natural
    }
}
```

`letters` is the degree, from 0 for the tonic. For degrees 0 to 4 the
natural minor note is the answer. For the sixth and seventh (5 and 6),
the function asks the question of the rule above: is the chord on the
natural note in natural minor? If not, and the chord a semitone higher
is in harmonic or melodic minor, the raised note is the one meant.
`of_key` builds the chord from C, because only its distance from the
tonic matters, not which key it is.

With that note, the accidentals are counted as before:

```rust
let unaltered =
    self.unaltered_degree(letters, chord.quality, MinorNumerals::current()) as i8;
let semitones = self.root.semitones_up_to(chord.root) as i8;
// How far the root is from the note the numeral names without an
// accidental, from -6 to 5.
let shift = (semitones - unaltered + 18).rem_euclid(12) - 6;
let accidentals = if shift < 0 { "b" } else { "#" }.repeat(shift.unsigned_abs() as usize);
```

The difference of two notes in semitones is only known up to octaves.
Adding 6, taking the remainder by 12 and taking 6 off again gives the
nearest way round, from −6 to 5: a root one semitone below gives −1, a
"b", not +11. (`rem_euclid` would give a remainder from 0 to 11 even for
a negative number; the extra 12 in `+ 18` keeps the sum positive anyway.)

Reading a typed numeral (`numeral::parse_in`) uses the same
`unaltered_degree`, so writing and reading can't disagree.

### A setting every function can see, one per thread

Numerals are written in many places: the harmony keys, the chord sheet,
the piano roll, the hints, a video export. They all end up in
`Scale::roman_numeral`. Passing the setting down as a parameter would
have changed the signature of every function on the way, dozens of
them, for a choice that changes once in a while. So the setting is kept
where `roman_numeral` can read it directly:

```rust
thread_local! {
    static MINOR_NUMERALS: Cell<MinorNumerals> = const { Cell::new(MinorNumerals::FromMajor) };
}

impl MinorNumerals {
    pub fn current() -> MinorNumerals {
        MINOR_NUMERALS.with(Cell::get)
    }

    pub fn set(self) {
        MINOR_NUMERALS.with(|numerals| numerals.set(self));
    }
}
```

The window's thread sets it once a frame, from the settings:

```rust
settings::minor_numerals(&ctx).set();
```

Two pieces of Rust make this safe:

- **`thread_local!`** gives each thread its own copy of the value.
  `cargo test` runs tests on many threads at once; a test that writes
  numerals counted from the minor scale must not change what a test
  running beside it sees. With an ordinary `static`, it would, and the
  other test would fail now and then, depending on timing.
- **`Cell`** lets the value be changed through a shared reference.
  Thread-local values are only ever reached by `&`, so a plain value
  could never be set. `Cell` works for `Copy` types by moving whole
  values in and out (`get`, `set`), never handing out a reference to
  what is inside, so nothing can be read while it changes.

Because the value belongs to a thread, a new thread starts with the
default. A video export draws its views on a thread of its own, so it
takes the setting along:

```rust
let numerals = MinorNumerals::current();
let worker = thread::spawn(move || {
    // The views are made here, on the export's own thread,
    // and write numerals as the app does.
    numerals.set();
    // …
});
```

### Putting it back, even after a panic

Tests need to write numerals one way for a while, then go back:

```rust
pub fn with<T>(self, f: impl FnOnce() -> T) -> T {
    struct Restore(MinorNumerals);
    impl Drop for Restore {
        fn drop(&mut self) {
            self.0.set();
        }
    }
    let _restore = Restore(MinorNumerals::current());
    self.set();
    f()
}
```

`Restore` holds the old value, and its `Drop` puts it back. Rust drops
a local variable when it goes out of scope, and also while a **panic**
unwinds through the function. A failing `assert!` inside `f` is a
panic, so even then the old setting comes back, and the next test on
that thread starts clean. This pattern, cleaning up in a `Drop`, is
called a *guard*, and it is how Rust's `MutexGuard` unlocks a mutex.

The variable is named `_restore`, not `_`. With a plain `_`, the guard
would be dropped at once, on the same line, before `self.set()`; nothing
would put the old value back after `f`.

## The tests

`minor_keys_can_be_numbered_from_their_own_scale` writes the triads and
sevenths of natural, harmonic and melodic minor counted from the minor
scale (the table above), checks that C major and E Phrygian are
unchanged, and that the old way comes back after `with`.

`numerals_from_the_minor_scale_read_back_as_the_same_chords` checks
that every numeral the app writes is read back as the same chord: both
ways of counting, four kinds of key, twelve tonics, a chord on each of
the twelve notes and nine qualities. That is 10,368 numerals, written
and read.

`minor_key_numerals_follow_the_harmony_setting` shows the harmony keys
of A minor (bIII, bVI, bVII), clicks **Harmony** and the second choice
in the Settings window, and checks that the keys now say III, VI and
VII, and that the choice was saved.

## Exercises

1. In A harmonic minor, what does each way write for C augmented
   (C E G#)? And for E minor? Check in the app with the harmony keys.
2. Dorian is a minor mode too, but the app counts it from the major
   scale whatever you choose (`is_minor_key` is false for it). D Dorian
   is D E F G A B C. Which of its numerals would change if it were
   counted from its own scale, and why might that confuse more than
   help?
3. Remove the `_` from `_restore` (name it `restore`) and run
   `cargo test -p lineup-core`. Then change it to a bare `_` and run the
   tests again. What happens, and why?
