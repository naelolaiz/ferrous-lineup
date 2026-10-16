# Chords, and naming what you hear

Files: [`crates/loom-core/src/harmony/chord.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/harmony/chord.rs)
and [`crates/loom-core/src/harmony/analysis.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/harmony/analysis.rs)

## The music

A **chord** is several notes played together. Most chords are built from
**thirds**: a major third is 4 semitones, a minor third is 3.

| Chord | Built from the root | Example on C | Sounds |
|-------|---------------------|--------------|--------|
| Major | major third + minor third (0, 4, 7) | C E G | bright, settled |
| Minor | minor third + major third (0, 3, 7) | C Eb G | darker |
| Diminished | two minor thirds (0, 3, 6) | C Eb Gb | tense |
| Augmented | two major thirds (0, 4, 8) | C E G# | unstable, dreamy |
| Dominant 7th | major triad + minor third (0, 4, 7, 10) | C E G Bb | wants to move on |
| Major 7th | major triad + major third (0, 4, 7, 11) | C E G B | soft, jazzy |
| Minor 7th | minor triad + minor third (0, 3, 7, 10) | C Eb G Bb | mellow |

The lowest note is the **root** when the chord is in **root position**.
Play C major as E G C (E at the bottom) and it is still C major, in its
first **inversion**. Chord symbols write inversions with a slash: C/E is
"C major with E in the bass".

A chord's **Roman numeral** says which degree of the key it is built on,
in upper case for major chords and lower case for minor ones. In C
major, Am is **vi** and G7 is **V7**. Numerals let you talk about
progressions in any key at once: I, vi, IV, V is C, Am, F, G in C major
and G, Em, C, D in G major.

## Chord qualities: a table in a `match`

```rust
pub enum ChordQuality {
    Major, Minor, Diminished, Augmented, Sus2, Sus4,
    Major6, Minor6, Dominant7, Major7, Minor7, MinorMajor7,
    HalfDiminished7, Diminished7, AugmentedMajor7, Dominant7Sus4,
    Add9, MinorAdd9, Dominant9, Major9, Minor9,
}

pub fn intervals(self) -> &'static [u8] {
    use ChordQuality::*;
    match self {
        Major => &[0, 4, 7],
        Minor => &[0, 3, 7],
        Dominant7 => &[0, 4, 7, 10],
        Dominant9 => &[0, 4, 7, 10, 14],
        // ...
    }
}
```

Each quality has three more `match` methods in the same style: `symbol`
("m7"), `name` ("minor seventh") and `numeral_suffix` (the "°" in
"vii°"). Writing the data as `match` arms instead of, say, a list of
structs has one big advantage: add a new variant and the compiler lists
every method that does not handle it yet.

`numeral_suffix` ends with a **binding pattern**:

```rust
match self {
    Major | Minor => "",
    Diminished => "°",
    // ...
    other => other.symbol(),
}
```

`other` matches anything left and gives it a name, like `_` but usable.

Some chords have intervals bigger than an octave: a ninth is 14
semitones. In a `PitchSet` that wraps around to 2, which is fine,
because pitch classes ignore octaves anyway. It does mean C add9 (C D E
G as pitch classes) can only be told apart from the other chords by its
set, which leads to this test:

```rust
#[test]
fn no_two_qualities_have_the_same_notes() {
    for (i, a) in ChordQuality::ALL.iter().enumerate() {
        for b in &ChordQuality::ALL[i + 1..] {
            assert_ne!(a.pitch_set(), b.pitch_set(), "{a:?} and {b:?}");
        }
    }
}
```

`&ChordQuality::ALL[i + 1..]` is a slice of the array from `i + 1` to
the end, so every pair is compared once.

Finding the quality of a set of notes (seen from the root, see the
previous chapter) is then one line. `find` returns the first item for
which the closure is true:

```rust
pub fn identify(relative: PitchSet) -> Option<ChordQuality> {
    Self::ALL
        .into_iter()
        .find(|quality| quality.pitch_set() == relative)
}
```

## What is playing? An enum for the answer

The piano roll needs to name whatever notes sound at a step. That might
be nothing, one note, two notes, a chord we know, or a cluster we have
no name for. Each answer needs different data, which is a job for an
enum whose variants carry data:

```rust
pub enum Harmony {
    Silence,
    Single(PitchClass),
    Interval { low: PitchClass, high: PitchClass, semitones: u8 },
    Chord { chord: Chord, bass: PitchClass },
    Unknown(PitchSet),
}
```

`analyze` takes any iterator of notes and returns a `Harmony`:

```rust
pub fn analyze(notes: impl IntoIterator<Item = Note>) -> Harmony {
    let mut pitch_set = PitchSet::EMPTY;
    let mut lowest: Option<Note> = None;
    for note in notes {
        pitch_set.insert(note.pitch_class());
        lowest = Some(match lowest {
            Some(low) => low.min(note),
            None => note,
        });
    }

    let Some(lowest) = lowest else {
        return Harmony::Silence;
    };
    let bass = lowest.pitch_class();

    match pitch_set.len() {
        1 => Harmony::Single(bass),
        2 => { /* an interval */ }
        _ => match find_chord(pitch_set, bass) {
            Some(chord) => Harmony::Chord { chord, bass },
            None => Harmony::Unknown(pitch_set),
        },
    }
}
```

`impl IntoIterator<Item = Note>` in an *argument* means "anything you
can loop over that gives notes": a `Vec<Note>`, an array, or an iterator
straight from the pattern, with nothing collected in between. One loop
builds the set and finds the lowest note, and `match` on a number
(`1`, `2`, `_` for everything else) picks the kind of answer.

## Finding the chord

Any note of a chord could be its root. `find_chord` tries each one,
starting with the bass, because chords are most often played with the
root lowest:

```rust
fn find_chord(pitch_set: PitchSet, bass: PitchClass) -> Option<Chord> {
    let roots = || {
        (0..12)
            .map(move |semitones| bass.transpose(semitones))
            .filter(move |&root| pitch_set.contains(root))
    };

    let exact = roots().find_map(|root| {
        ChordQuality::identify(pitch_set.relative_to(root))
            .map(|quality| Chord::new(root, quality))
    });

    exact.or_else(|| {
        roots().find_map(|root| {
            let mut with_fifth = pitch_set;
            with_fifth.insert(root.transpose(7));
            if with_fifth == pitch_set {
                return None; // the fifth was already there
            }
            ChordQuality::identify(with_fifth.relative_to(root))
                .filter(|quality| quality.intervals().len() >= 4)
                .map(|quality| Chord::new(root, quality))
        })
    })
}
```

- `roots` is a **closure that makes an iterator**. Iterators are used up
  as you go through them, so to search twice, the code calls `roots()`
  again to get a fresh one.
- `find_map` is `find` and `map` in one: it calls the closure on each
  item and returns the first `Some` it gets.
- `or_else` on an `Option` runs its closure only if the option is
  `None`. The second search, which allows a missing fifth, only runs when
  the exact search found nothing. Musicians often leave the fifth out of
  seventh chords, and C, E, Bb is still heard as C7.
- `Option::filter` keeps a `Some` only if the closure says yes. Here it
  stops "C and E, plus a guessed G" from being named as a whole C major
  chord; guessing is only allowed for chords of four notes or more.

Starting with the bass settles real ambiguities. C E G A is both C6 and
Am7; played over a C it is C6, over an A it is Am7. Tests pin these
cases down:

```rust
assert_eq!(label(&[60, 64, 67, 69]).unwrap(), "C6");
assert_eq!(label(&[57, 60, 64, 67]).unwrap(), "Am7");
assert_eq!(label(&[64, 67, 72]).unwrap(), "C/E");
```

## Labels: match guards and nested patterns

The chord lane above the piano roll shows a short label for each step:

```rust
pub fn label(&self, spelling: Spelling) -> Option<String> {
    match *self {
        Harmony::Silence => None,
        Harmony::Single(pitch_class) => Some(pitch_class.name(spelling).to_string()),
        Harmony::Interval { low, semitones: 7, .. } => Some(format!("{}5", low.name(spelling))),
        Harmony::Interval { low, high, .. } => {
            Some(format!("{}+{}", low.name(spelling), high.name(spelling)))
        }
        Harmony::Chord { chord, bass } if bass != chord.root => {
            Some(format!("{}/{}", chord.name(spelling), bass.name(spelling)))
        }
        Harmony::Chord { chord, .. } => Some(chord.name(spelling)),
        Harmony::Unknown(_) => Some("?".to_string()),
    }
}
```

Three pattern features in one function:

- `semitones: 7` inside a pattern matches only that value. A root plus
  its fifth is a "power chord", written C5. Other intervals fall through
  to the next arm.
- `..` means "and the other fields, whatever they are".
- `if bass != chord.root` after a pattern is a **match guard**: the arm
  only matches if the condition is true too. Inversions get a slash,
  root-position chords do not.

Arms are tried from top to bottom, so the more specific arm must come
first. If the plain `Harmony::Chord { chord, .. }` came first, the slash
arm could never match.

`match *self` matches on the value behind the reference. That works
because every field is `Copy`; the patterns copy the fields out.

`describe` uses the key to write a whole sentence, such as "Am: A minor,
chord vi of C major" or "E: E major, III borrowed from outside C major".
A chord whose root is in the key but whose notes are not all in it (E
major has a G#, which C major does not) is **borrowed** from another key.
Borrowed chords are common and colourful; the message just names them.

## Chord stamps

What a click in the piano roll places is another enum, with one variant
that carries a `ChordQuality`:

```rust
pub enum ChordStamp {
    Single,
    ScaleTriad,
    ScaleSeventh,
    Fixed(ChordQuality),
}

pub fn notes_at(self, note: Note, scale: Scale) -> Vec<Note> {
    match self {
        ChordStamp::Single => vec![note],
        ChordStamp::ScaleTriad => scale.stack_thirds(note, 3).unwrap_or_else(|| vec![note]),
        ChordStamp::ScaleSeventh => scale.stack_thirds(note, 4).unwrap_or_else(|| vec![note]),
        ChordStamp::Fixed(quality) => Chord::new(note.pitch_class(), quality).notes_from(note),
    }
}
```

"Chord in key" stamps are where the piano roll teaches harmony: click
any highlighted row and you get the chord the key gives you on that
note, without having to know its name first. The name then appears in
the chord lane.

The Draw menu lists every stamp. `all()` builds that list by chaining
two iterators, the three special stamps and then one `Fixed` per quality:

```rust
pub fn all() -> impl Iterator<Item = ChordStamp> {
    [ChordStamp::Single, ChordStamp::ScaleTriad, ChordStamp::ScaleSeventh]
        .into_iter()
        .chain(ChordQuality::ALL.into_iter().map(ChordStamp::Fixed))
}
```

`map(ChordStamp::Fixed)` passes the variant itself as a function: a
tuple variant like `Fixed` *is* a function from `ChordQuality` to
`ChordStamp`.
