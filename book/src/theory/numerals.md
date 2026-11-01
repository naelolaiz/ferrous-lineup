# Roman numerals, the harmony keys and chord functions

Files: [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/harmony/keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/keys.rs),
[`crates/lineup-core/src/harmony/progression.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/progression.rs),
[`crates/lineup-core/src/library/numeral.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/numeral.rs),
[`crates/lineup-core/src/chord_sheet/function.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/function.rs)
and [`crates/lineup-app/src/harmony_keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/harmony_keys.rs)

A chord's name says what it is; its **Roman numeral** says what it is
*in the key*. The numerals are how Ferrous Lineup teaches harmony: they
label the harmony keys, they sit under every chord of the chord sheet,
they explain the hints about what could come next, and the library
writes its progressions with them. This chapter is about writing and
reading numerals, the harmony keys, the tables of common moves, and how
the chord sheet works out what a chord from outside the key is doing
there.

## The music, briefly

A key has seven chords, one on each note of its scale, and musicians
number them with Roman numerals: upper case for a major chord, lower
case for a minor one, ° for diminished, + for augmented, ø for
half-diminished.

```text
  I    ii    iii   IV    V     vi    vii°
  C    Dm    Em    F     G     Am    Bdim
```

Numerals let you talk about a progression in every key at once: I vi
IV V is C Am F G in C major and G Em C D in G major.

Each chord does one of three jobs in its key, its **function**:

| Function | Chords | Feels like |
|----------|--------|------------|
| **Tonic** | I, iii, vi | home: the music can rest here |
| **Subdominant** | ii, IV | leaving home, on the way to the dominant |
| **Dominant** | V, vii° | tension that pulls back home to I |

The dominant's pull comes from its third, B in G major, a semitone
below C: the **leading tone**, which wants to rise to the tonic. In a
minor key the scale has no leading tone (in A minor, G is a whole tone
below A), so music in minor keys raises it to G♯, which is what the
harmonic minor scale is, and its V is major: E, not Em.

A few moves between the chords are the backbone of most music:

| Move | In C | Why it works |
|------|------|--------------|
| V to I | G to C | the strongest ending: the leading tone rises to the tonic |
| IV to V | F to G | a step up, from moving away to wanting home |
| ii to V | Dm to G | the root falls a fifth; ii V I is everywhere, from jazz to pop |
| IV to I | F to C | the soft "amen" (plagal) cadence |
| V to vi | G to Am | the deceptive cadence: you expect C and get its relative minor |

A root that **falls a fifth** (G to C, D to G, A to D) is the strongest
move there is, and many of the suggestions are of that kind.

Songs also use chords from outside the key, and harmony names what
those are doing. A **borrowed** chord comes from the minor key on the
same tonic, as Fm does in C major. A **secondary dominant** is the
dominant of another chord of the key: D7 is the V of G, written V7/V,
and for a moment it makes G sound like home. The **Neapolitan** is a
major chord on the lowered second degree, D♭ in C. A **pivot chord**
belongs to two keys, and the music changes key through it. The manual's
[What chords do: numerals and
functions](https://naelolaiz.github.io/ferrous-lineup/manual/chord-functions.html)
and [Chords from outside the
key](https://naelolaiz.github.io/ferrous-lineup/manual/outside-the-key.html)
teach all of this with examples to play.

## Writing a numeral

A numeral has three parts: a sharp or flat in front, the numeral, and
what follows it, the "7" of V7 or the "°" of vii°. All three come from
one function:

```rust
fn numeral_on(self, letters: u8, chord: Chord) -> Option<String> {
    const NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
    if !self.is_heptatonic() {
        return None;
    }
    let letters = usize::from(letters);
    let unaltered =
        self.unaltered_degree(letters, chord.quality, MinorNumerals::current()) as i8;
    let semitones = self.root.semitones_up_to(chord.root) as i8;
    // How far the root is from the note the numeral names without an
    // accidental, from -6 to 5.
    let shift = (semitones - unaltered + 18).rem_euclid(12) - 6;
    let accidentals = if shift < 0 { "b" } else { "#" }.repeat(shift.unsigned_abs() as usize);
    let numeral = if chord.quality.is_minor() {
        NUMERALS[letters].to_lowercase()
    } else {
        NUMERALS[letters].to_string()
    };
    Some(format!(
        "{accidentals}{numeral}{}",
        chord.quality.numeral_suffix()
    ))
}
```

- `letters` is the degree, how many letters above the tonic the root
  is. It picks the numeral: 4 letters up is V.
- `unaltered` is the note the plain numeral names, in semitones above
  the tonic. In a major key, the note of the major scale: V names the
  note seven semitones up. (In a minor key it can be another, as the
  next section shows.)
- `shift` is how far the chord's root is from that note. The
  difference of two pitch classes is only known up to octaves, so
  adding 18, taking the remainder by 12 and taking 6 off gives the
  nearer way round, from -6 to 5: a root one semitone below gives -1, a
  flat, not +11.
- The case comes from the quality, and the end from `numeral_suffix`
  ([Chords: building, naming and inversions](chords.md)).

So in C major, B♭ major is ♭VII: the plain VII names B, eleven
semitones above C; B♭ is ten, one below, so one flat.

Which degree a chord stands on can be counted in two ways, and both are
used. `roman_numeral(chord)` counts from the sound: the root's letter in
the key's own spelling ([Scales, modes and
keys](scales.md#notes-from-outside-the-key)).
`spelled_numeral(root, chord)` counts from the root as it was
**written**, so a chord typed on the chord sheet keeps its letter. In C
major, C♯°7, a raised tonic leading up to Dm, is ♯i°7, and D♭7 is ♭II7.
From the sound alone both would stand on the second degree.

```rust
pub fn spelled_numeral(self, root: NoteName, chord: Chord) -> Option<String> {
    self.numeral_on(self.tonic_name().letter.steps_up_to(root.letter), chord)
}
```

### Figures

An inversion's figures ([Chords: building, naming and
inversions](chords.md#inversions)) go after the numeral, in
place of a seventh chord's 7: V7 with its third in the bass is V⁶₅. A
secondary dominant keeps its target after the figures: V⁶₅/vi.

```rust
pub fn with_figures(numeral: &str, figures: &str) -> String {
    if figures.is_empty() {
        return numeral.to_string();
    }
    // A secondary dominant's figures go on its own numeral, before the
    // slash: V⁶₅/vi.
    let (own, target) = match numeral.split_once('/') {
        Some((own, target)) => (own, format!("/{target}")),
        None => (numeral, String::new()),
    };
    let own = own.strip_suffix('7').unwrap_or(own);
    format!("{own}{figures}{target}")
}
```

`split_once('/')` splits at the first slash; `strip_suffix('7')` gives
the text without a final 7, or `None` if it has none.

### Reading a numeral back

The library stores its progressions as numerals, and the chord sheet
accepts them typed. `numeral::read` takes a numeral apart:

```rust
let mut shift: i32 = 0;
let mut rest = text;
loop {
    if let Some(after) = rest.strip_prefix('b') {
        shift -= 1;
        rest = after;
    } else if let Some(after) = rest.strip_prefix('#') {
        shift += 1;
        rest = after;
    } else {
        break;
    }
}
// The numeral, in either case. `get(..len)` gives `None` instead of
// panicking when `rest` is shorter than the numeral.
let (numeral, degree) = NUMERALS.iter().find(|(numeral, _)| {
    rest.get(..numeral.len())
        .is_some_and(|start| start.eq_ignore_ascii_case(numeral))
})?;
```

`strip_prefix` gives the rest of the text when it starts with the
prefix. Slicing a string with `&rest[..n]` panics when `n` is past the
end; `get(..n)` gives `None` instead, so a short text is simply not
that numeral. The table lists III before II and I, and VII before VI
and V, so a longer numeral is tried before a shorter one it starts
with. A numeral in mixed case, such as "Iv",
is refused, since its case is what says major or minor.

`parse_in(text, key, numerals)` then turns degree and shift back into
semitones above the tonic, with the same `unaltered_degree` that
writing uses, so writing a numeral and reading it back can never
disagree. A test checks exactly that for 10,368 numerals: both ways of
counting, four kinds of key, twelve tonics, a chord on each of the
twelve notes and nine qualities.

## Minor keys: bIII or III

In A minor, the chord C major has two names. Pop and jazz books call it
**♭III**; classical harmony books call it **III**. Both are right, and
**Edit > Settings… > Harmony > Roman numerals in minor keys** lets you
choose.

**Counted from the major scale**, the numerals are measured against
the major scale on the same tonic, A major, whose third is C♯. C is a
semitone below, so its chord is ♭III; F is ♭VI and G ♭VII. One table of
progressions then works in every key and mode: ♭VI ♭VII i is the same
progression in A minor and A Dorian, and a song in C major that
borrows A♭ and B♭ from C minor writes them ♭VI and ♭VII too. The
library always writes its progressions this way.

**Counted from the minor scale**, the numerals are measured against the
key's own scale. C is simply its third note, so its chord is III.

Only the third, sixth and seventh degrees differ, the notes that differ
between A major and A minor. But the sixth and seventh notes of a minor
key also move: harmonic minor raises G to G♯, melodic minor raises F to
F♯ as well. So a plain numeral on those degrees must name one of two
notes, and the rule classical books use is: **a plain numeral names the
form whose chord belongs to the key**. VII is G major, from natural
minor; vii° is G♯°, the leading-tone chord of harmonic minor, since G°
(G B♭ D♭) is no chord of A minor.

```text
                     from the major scale           from the minor scale
natural minor   i ii° bIII  iv v  bVI bVII     i ii° III  iv v  VI  VII
harmonic minor  i ii° bIII+ iv V  bVI vii°     i ii° III+ iv V  VI  vii°
melodic minor   i ii  bIII+ IV V  vi° vii°     i ii  III+ IV V  vi° vii°
```

That rule is `unaltered_degree`:

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

For the first five degrees the natural minor note is the answer. For
the sixth and seventh, the function asks the rule's question: is the
chord on the natural note a chord of natural minor? If not, and the
chord a semitone higher is in harmonic or melodic minor, the raised
note is the one meant. `of_key` builds the chord on C, because only its
distance from the tonic matters.

The setting changes how numerals are **written** and **read**, never a
chord. A song stores chords, not numerals, so it is the same whichever
way you choose. Major keys and the modes are written the same both
ways.

### A setting every function can see, one per thread

Numerals are written in many places, the harmony keys, the chord
sheet, the piano roll, the hints, a video export, and all of them end
up in `numeral_on`. Passing the setting down as a parameter would
change the signature of dozens of functions on the way, for a choice
that changes once in a while. So the setting is kept where `numeral_on`
can read it:

```rust
thread_local! {
    /// How the numerals written on this thread count a minor key's
    /// degrees. The window's thread sets it from the settings each frame;
    /// each thread has its own, so tests running side by side never see
    /// each other's.
    static MINOR_NUMERALS: Cell<MinorNumerals> = const { Cell::new(MinorNumerals::FromMajor) };
}

impl MinorNumerals {
    /// How numerals are written on this thread.
    pub fn current() -> MinorNumerals {
        MINOR_NUMERALS.with(Cell::get)
    }

    /// Writes numerals this way on this thread from now on.
    pub fn set(self) {
        MINOR_NUMERALS.with(|numerals| numerals.set(self));
    }
}
```

The window's thread sets it before drawing each frame:

```rust
// Every view writes numerals as the settings say.
settings::minor_numerals(ctx).set();
```

Two pieces of Rust make this safe:

- **`thread_local!`** gives each thread its own copy of the value.
  `cargo test` runs tests on many threads at once, and a test that
  writes numerals from the minor scale must not change what a test
  running beside it sees. With an ordinary `static` it would, and the
  other test would fail now and then, depending on timing.
- **`Cell`** lets a value be changed through a shared reference. A
  thread-local value is only ever reached through `&`, so a plain value
  could never be set. `Cell` works by moving whole values in and out
  (`get`, `set`), never handing out a reference to what is inside, so
  nothing can be read while it changes.

Because the value belongs to a thread, a new thread starts with the
default. A video export draws its views on a thread of its own, so it
takes the setting along, with the language and the note naming
([Exporting: WAV, MIDI and
video](../sound/export.md#settings-that-live-on-a-thread)):

```rust
let numerals = MinorNumerals::current();
```

and calls `numerals.set()` first thing on the new thread.

#### Putting it back, even after a panic

Tests write numerals one way for a while, then go back:

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

`Restore` holds the old value, and its `Drop` puts it back. Rust drops a
local variable when it goes out of scope, and also while a **panic**
unwinds through the function. A failing `assert!` inside `f` is a
panic, so even then the old setting comes back, and the next test on
that thread starts clean. Cleaning up in a `Drop` like this is called a
**guard**; it is how the standard library's `MutexGuard` unlocks a
mutex.

The variable is called `_restore`, not `_`. A plain `_` does not bind
at all, so the guard would be dropped at once, on the same line, before
`self.set()`, and nothing would put the old value back after `f`.

The hints are sentences written once, with numerals as the library
writes them: "I to IV: moving away from home". When numerals are
counted from the minor scale, `numeral::written_in` rewrites the
numerals inside such a sentence. Most of the time nothing changes, so
it returns a `Cow<str>`: the original text borrowed, or a new `String`
only when it had to rewrite something.

## The harmony keys

The harmony keys along the bottom of the window are one big key for
each chord of the key, coloured by function, played by holding them
down. The model is pure theory, so it lives in `lineup-core`, with
tests, and knows nothing of egui:

```rust
pub fn chord_keys(scale: Scale, size: ChordSize) -> Vec<ChordKey> {
    let scale = chord_scale(scale);
    let leading_tone = Scale::new(scale.root, ScaleKind::HarmonicMinor);
    (0..7)
        .filter_map(|degree| {
            let from = if scale.kind == ScaleKind::NaturalMinor && degree == 4 {
                leading_tone
            } else {
                scale
            };
            let chord = chord_on(from, degree, size)?;
            Some(ChordKey {
                degree,
                chord,
                numeral: scale.roman_numeral(chord)?,
                function: has_functions(scale.kind).then(|| Function::of(degree)),
            })
        })
        .collect()
}
```

- `filter_map` runs the closure on every degree and keeps the `Some`s,
  and inside the closure `?` turns a `None` into "leave this one out".
- In a natural minor key, the chord on V comes from harmonic minor:
  major, with the leading tone.
- `chord_scale` gives a pentatonic or blues scale, which has no seven
  chords of its own, the major or minor scale on the same root.
- `bool::then` turns `true` into `Some` of the closure's value and
  `false` into `None`. The modes get no function: they have no dominant
  that pulls home the way V does. In C Locrian the chord on the fifth
  note is G♭ major, which does not pull to C at all.

`Function::of` is the table of jobs by degree:

```rust
pub fn of(degree: usize) -> Function {
    match degree % 7 {
        1 | 3 => Function::Subdominant,
        4 | 6 => Function::Dominant,
        _ => Function::Tonic,
    }
}
```

`ChordSize` picks triads, sevenths or ninths. On two degrees of a major
key, iii and vii, the ninth lands a semitone above the root, an octave
up; that clashes, and players leave it out, so `Scale::ninth` gives
`None` there and the key falls back to the seventh chord:

```rust
ChordSize::Ninths => scale.ninth(degree).or_else(|| scale.seventh(degree)),
```

`Option::or_else` tries the closure only when the first value is
`None`: "if that did not work, try this".

### Playing a key

Pressing C and then F could play C E G and then F A C, every note
jumping up. A pianist would keep the C and move the other two fingers a
step each, to C F A. `keys::play` voices each chord close to the one
before (the voicing itself is the band's, in [How the band plays a
chord sheet](../band/band.md)) and adds the root below, as a left
hand would:

```rust
pub fn play(chord: Chord, previous: Option<&[Note]>) -> (Vec<Note>, Vec<Note>) {
    let voicing = voice(chord, previous);
    let mut notes = voicing.clone();
    if let Some(&lowest) = voicing.first() {
        // The root below the lowest note: an octave below if the lowest
        // note is the root itself.
        let down = match chord.root.semitones_up_to(lowest.pitch_class()) {
            0 => 12,
            gap => gap,
        };
        if let Some(root) = lowest.transpose(-i32::from(down)) {
            notes.insert(0, root);
        }
    }
    (voicing, notes)
}
```

It returns a tuple of two vectors: the voicing, to remember for next
time, and the notes to play. The caller takes it apart with `let
(voicing, notes) = ...`.

A harmony key is a key, not a button: its chord starts when the mouse
goes down and stops when it comes up, and the number keys 1 to 7 play
them too. Which one holds a key down is an enum whose variant carries
the number key:

```rust
enum Holder {
    Mouse,
    /// A number key on the computer keyboard, 1 to 7.
    Digit(Key),
}

/// The harmony key that is down, and the notes it started.
#[derive(Debug, Clone, PartialEq)]
struct Down {
    key: usize,
    holder: Holder,
    notes: Vec<Note>,
}
```

Only the holder that started a chord can stop it, and `Down` keeps the
notes it started, so the right notes stop even if the key or the chord
size changed while it was held. The notes go the same way as the typing
keyboard's ([MIDI: messages, devices, recording and
files](../sound/midi.md)), so they sound on the selected channel, are
named in the status bar and are recorded, without that code knowing the
harmony keys exist.

### Which key lights up

While the music plays, the key of the chord you hear is lit and framed
in white, the most visible mark on the bar. Keys held down win; else
the chord is the one heard over the beat at the playhead, from the
chord hearing ([Hearing harmony: chords and keys from
notes](hearing.md)), so an arpeggio lights its chord and
two chords at once light two keys. A chord lights a key when it has
the same root and the same kind of triad as the key's chord there:

```rust
pub fn key_for(keys: &[ChordKey], chord: Chord) -> Option<usize> {
    keys.iter().position(|key| {
        key.chord.root == chord.root && key.chord.quality.family() == chord.quality.family()
    })
}
```

G7 lights V in C major. D7 lights nothing: the key's chord on D is
minor, and D7 comes from outside the key.

## What comes next: knowledge as data

While a chord sounds, held down or playing in the song, the keys that
often come next get a thin yellow outline, the most usual one a little
brighter, and a hint line says why. Let go, or stop the song, and the
outline goes: the keys show only what is playing, so the chord the
outline follows is the one `sounding` finds in the keys' code, a key
held down first, then a chord held on a keyboard, then the music's. `next_chords(scale, current)` gives those chords, most usual first.
The knowledge is not in code but in tables: each chord is found by its
root (semitones above the tonic) and its kind of triad, and lists the
moves that often follow it.

```rust
struct Move {
    to: u8,
    short: &'static str,
    reason: &'static str,
}

const fn go(to: u8, short: &'static str, reason: &'static str) -> Move {
    Move { to, short, reason }
}

struct Slot {
    root: u8,
    family: ChordFamily,
    moves: &'static [Move],
}
```

```rust
const MAJOR: &[Slot] = &[
    slot(
        0,
        Major,
        &[
            go(
                5,
                "away from home",
                "I to IV: moving away from home, toward the dominant.",
            ),
            go(
                7,
                "tension, wants to go home",
                "I to V: tension that wants to come back to I.",
            ),
            // ...
        ],
    ),
    // ...
];
```

A constant can only be built by code the compiler can run while
compiling, and a `const fn` is such code, so the tiny `go` and `slot`
keep each move on a line or two. `MAJOR` is a slice of slots, so it
holds as many entries as a key needs, the borrowed iv, ♭III, ♭VI and
♭VII included. All of it is built into the program. Changing what the
app suggests means editing a table, and the tests check the table:
every degree of three kinds of key has suggestions, and no chord
suggests itself.

Each move carries its reason twice, in a sentence and in a few words.
The hint line's level, **Detailed**, **Short** or **Minimal**, picks
one:

```rust
pub fn why(&self, level: HintLevel) -> Option<&str> {
    match level {
        HintLevel::Detailed => Some(&self.reason),
        HintLevel::Short => Some(&self.short),
        HintLevel::Minimal => None,
    }
}
```

A `Suggestion` keeps its reasons as `Cow<'static, str>`, "clone on
write": usually a borrowed `&'static str` from the table, with no
allocation, and an owned `String` only when `written_in` had to rewrite
its numerals.

Minor keys find their chords in another table, natural minor with the
major V and the leading-tone vii° of harmonic minor:

```rust
const MINOR_CHORDS: [(u8, ChordQuality); 8] = [
    (0, ChordQuality::Minor),
    (2, ChordQuality::Diminished),
    (3, ChordQuality::Major),
    (5, ChordQuality::Minor),
    (7, ChordQuality::Major),
    (8, ChordQuality::Major),
    (10, ChordQuality::Major),
    (11, ChordQuality::Diminished),
];
```

### Secondary dominants first

A chord from outside the key that sounds like a dominant is most often
the dominant of the chord a fifth below it, so `next_chords` checks for
that before looking at the tables:

```rust
let dominant_kind = current.quality == ChordQuality::Major || current.quality.is_dominant();
let target = (root + 5) % 12;
if dominant_kind
    && !scale.is_chord_of_key(current)
    && target != 0
    && let Some(suggestion) = suggest(target, SECONDARY_DOMINANT_SHORT, SECONDARY_DOMINANT)
    && suggestion.chord.quality != ChordQuality::Diminished
{
    return vec![suggestion];
}
```

Read the let chain as the rule it is: the chord sounds like a dominant,
it is not one of the key's own chords, the chord a fifth below is not
the tonic (in a minor key, a major chord on V is the key's own
dominant), there is a chord of the key there, and it is not
diminished, which is too unstable to sound like home even for a moment.
A fifth down and a fourth up are the same pitch class, so the target is
`(root + 5) % 12`. In C major D major suggests G, A7 Dm, E Am, and C7,
whose B♭ makes it no longer the tonic chord, F.

### The bass matters: the cadential six-four

C/G, the tonic chord with its fifth in the bass, sounds unsettled: over
G, the C and the E want to fall a step to B and D, which makes G. So in
a cadence C/G goes on to V, and is called the **cadential six-four**
(I⁶₄). `next_chords_over` takes the bass too, and handles that case
before handing everything else to `next_chords`:

```rust
let six_four = current == Chord::new(scale.root, tonic) && current.tone_of(bass) == Some(5);
if !six_four {
    return next_chords(scale, current);
}
```

## What a chord does: the chord sheet's functions

The chord sheet writes a numeral under every chord, and for a chord
from outside the key, what it is doing there. That depends on more
than the chord and the key: a secondary chord is named by where it
goes, and a pivot by the chords around a key change. So
`Functions::read` reads the whole sheet in one pass, and the sheet asks
it about each chord. What a chord does is a `Role`, an enum whose
variants carry what they need:

```rust
pub enum Role {
    /// One of the key's own chords, or a chord outside it that none of
    /// the others names.
    Plain,
    /// Taken from the parallel key, `from`.
    Borrowed { from: Scale },
    /// The dominant of `of`, one of the key's chords.
    SecondaryDominant { of: Chord },
    /// The chord on the leading tone of `of`, one of the key's chords.
    SecondaryLeadingTone { of: Chord },
    /// A major chord on the lowered second degree.
    Neapolitan,
}
```

A `match` gets the data out where it is used, so `of` only exists in
the arms where it means something:

```rust
match self.role {
    Role::SecondaryDominant { of } => Some(with_figures(
        &format!(
            "V{}/{}",
            chord.quality.numeral_suffix(),
            self.target_numeral(of)?
        ),
        figures,
    )),
    // ...
    Role::Neapolitan => Some(with_figures("N", figures)),
    Role::Plain | Role::Borrowed { .. } => figured(self.key, self.symbol),
}
```

`Role::Borrowed { .. }` matches a borrowed chord whatever key it comes
from.

### The key's helpers

The questions behind the roles are methods of `Scale`:

- `parallel()`: the key with the same tonic and the other mode, C minor
  for C major.
- `is_chord_of_key(chord)`: in the scale, or in a minor key the V or
  vii° with the raised seventh of harmonic minor. E7 and G♯dim7 belong
  to A minor.
- `borrowed_from(chord)`: the parallel key, if the chord is not the
  key's own but is that key's. A major key borrows from harmonic minor
  too: B♭ and Bdim7 in C major both come from C minor.
- `secondary_dominant_target(chord)`: for a major or dominant seventh
  chord from outside the key, the key's major or minor chord (not the
  tonic) a fifth below it: E7 in C major is the V7 of Am.
- `secondary_leading_tone_target(chord)`: for a diminished chord, the
  chord a semitone above it: F♯°7 in C major leads to G.

### Which role comes first

Some chords could be two things. In A minor, A major is the dominant
of Dm and also the major tonic chord borrowed from A major; D major is
the IV of melodic minor and the dominant of G. `role` decides by where
the chord goes:

```rust
let goes_to = |of: Chord| next.is_some_and(|next| arrives(next, of));
let dominant = key.secondary_dominant_target(chord);
let leading = key.secondary_leading_tone_target(chord);
let secondary = match (dominant, leading) {
    (Some(of), _) => Some(Role::SecondaryDominant { of }),
    // A leading-tone chord is named after where it goes: F#ø7 going
    // on to B7 in C major is the ii of E minor, not the vii of G.
    (_, Some(of)) if goes_to(of) => Some(Role::SecondaryLeadingTone { of }),
    _ => None,
};
// A chord that is both, such as A major in A minor (the major tonic,
// or the V of Dm), is a secondary dominant where it goes to the
// chord it leads to, and a borrowed chord anywhere else.
if let Some(of) = dominant.or(leading)
    && goes_to(of)
{
    return secondary.unwrap_or(Role::Plain);
}
// So is a chord of melodic minor, such as D major in A minor: the V
// of G going to G, and the key's own IV anywhere else.
if in_minor(key, chord, &[MelodicMinor]) {
    return Role::Plain;
}
if let Some(from) = borrowed_from(key, symbol) {
    return Role::Borrowed { from };
}
if is_neapolitan(key, symbol) {
    return Role::Neapolitan;
}
secondary.unwrap_or(Role::Plain)
```

Before this, `role` has already sent away the chords that need no name
beyond their numeral: every chord in a mode (E major in E Phrygian is
the tonic chord of flamenco, not the dominant of A minor), every chord
of the key, and in a minor key every chord of its natural or harmonic
minor. A chord "goes to" its target when the next chord is on that
root, or is a six-four chord over it, which holds the target back for
a moment. `is_some_and` answers `false` for `None`, so the last chord,
with nothing after it, goes nowhere.

A borrowed chord must be **spelled** as the other key writes it, not
only sound like it. G augmented (G B D♯) is played on the same keys as
E♭ augmented (E♭ G B), a chord of C harmonic minor. But its D♯ rises to
E, where an E♭ would fall: it is no note of C minor. So `borrowed_from`
checks that every note of the chord is written on the letter the key's
scales give it, counting up from the tonic. In the same way the
Neapolitan must be spelled on the second letter: D♭ major in C is one,
C♯ major, the same keys, is a chromatic chord on the tonic.

### Finding the pivot

A **modulation** is a move to a new key that the music stays in. In C
major Am is vi; in G major it is ii. In C, Am, D7, G the ear hears Am as
vi in C; then D7 comes, which C major does not have, and the ear hears
that Am was really ii in G. Am is the **pivot chord**, and the chord
sheet writes both its numerals, old key first: `vi = ii`.

`modulation` looks at the chords around each key change, with two
closures that say what a chord is to the two keys:

```rust
let new_only = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && !heard_in(from, chord)
};
let common = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && heard_in(from, chord)
};
```

`heard_in` counts the old key's borrowed chords as its own, which is
why Fm, borrowed in C major, can be the pivot into A♭ major, where it
is vi. The **arrival** is the first chord that belongs to the new key
alone (D7 above). It is looked for just before the key marker first,
since musicians mark a new key where they feel it has arrived, which
can be a bar or two late, and then after it. The pivot is the chord
before the arrival, if it belongs to both:

```rust
change.pivot = arrival
    .checked_sub(1)
    .map(|before| range[before])
    .filter(|before| common(before))
    .map(|before| (before.start, before.symbol));
```

`checked_sub(1)` gives `None` when the arrival is the first chord, so
no index ever goes below zero, and a change that starts on its arrival
has no pivot. Not every key change has one: a **direct modulation**
goes straight to a chord of the new key, as a song does when it moves
up a semitone for its last chorus, and a **change of mode**, C major to
C minor, keeps the tonic and needs no chord to lead there.

## Testing it

The tests are small chord sheets in the style of a harmony textbook,
each checking the numerals and what the tooltips say:

- C Am A♭ Fm G7 C: ♭VI and iv are borrowed from C minor.
- C E7 Am D7/F♯ G7 C A7 Dm: V7/vi, V⁶₅/V and V7/ii, each going to its
  chord. E7 going to F is still V7/vi, and says it goes elsewhere.
- F♯dim7, G♯dim and C♯m7♭5: vii°7/V, vii°/vi and viiø7/ii. In Autumn
  Leaves read in C major, F♯m7♭5 going to B7 keeps ♯ivø7.
- D♭/F in C minor is N⁶; C♯ in C major is not a Neapolitan.
- In A minor, B7 is V7/V, or V7/v going to Em; A7 going to Dm is V7/iv;
  A major at the end is a Picardy third.
- G augmented in C major is not borrowed; Bdim7, whose A♭ is C
  minor's, is.
- C, Am, D7, G into G major, with the marker on the Am, the D7 or the
  G: Am is the pivot each time.

In `scale.rs`, the minor-key numerals are checked both ways for natural,
harmonic and melodic minor, and the 10,368 numerals are written and
read back. In `keys.rs` and `progression.rs`, the harmony keys of every
kind of key and the suggestions after every degree; for instance, in C
major:

```rust
// D major is V of G; A7 is V of Dm; E is V of Am; C7 is V of F.
```

Exercises on numerals and functions are in [Exercises: music theory in
code](exercises.md).
