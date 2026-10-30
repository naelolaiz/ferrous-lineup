# Chords: building, naming and inversions

Files: [`crates/lineup-core/src/harmony/chord.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord.rs),
[`crates/lineup-core/src/harmony/symbol.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/symbol.rs),
[`crates/lineup-core/src/harmony/analysis.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/analysis.rs)
and [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs)

Chords are everywhere in Ferrous Lineup: the piano roll places them
with one click and names whatever is sounding, the chord sheet reads
them as you type them, the harmony keys play them, and the band turns
them into parts. This chapter is about the types behind all of that: a
chord's quality, a chord, a chord symbol as a lead sheet writes it,
and the analysis that names a group of notes.

## The music, briefly

A **chord** is several notes played together. Most chords are built
from **thirds**: a major third is four semitones, a minor third three.

| Chord | Built from the root | On C | Sounds |
|-------|---------------------|------|--------|
| major | major third + minor third (0 4 7) | C E G | bright, settled |
| minor | minor third + major third (0 3 7) | C E♭ G | darker |
| diminished | two minor thirds (0 3 6) | C E♭ G♭ | tense |
| augmented | two major thirds (0 4 8) | C E G♯ | unstable, dreamy |
| dominant seventh | major triad + minor third (0 4 7 10) | C E G B♭ | wants to move on |
| major seventh | major triad + major third (0 4 7 11) | C E G B | soft, jazzy |
| minor seventh | minor triad + minor third (0 3 7 10) | C E♭ G B♭ | mellow |

Going on up in thirds past the octave gives the **extended** chords of
jazz, soul and funk. Their names count the notes of the scale from the
root: the ninth is the second an octave up (14 = 12 + 2 semitones), the
eleventh the fourth (17 = 12 + 5), the thirteenth the sixth (21 = 12 +
9). C13 is C E G B♭ D A.

The lowest note is the **root** when the chord is in **root position**.
Played as E G C, with E at the bottom, it is still C major, in its
first **inversion**. A **chord symbol**, as a lead sheet writes it,
puts the bass after a slash: C/E. The manual's [Chords and their
names](https://naelolaiz.github.io/ferrous-lineup/manual/chords.html)
has all of this, with what to listen for.

## Qualities: a table in a `match`

A chord without its root is a **quality**: "minor seventh", whatever
note it starts on. There are thirty, one enum variant each:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,
    Major6,
    Minor6,
    Dominant7,
    Major7,
    Minor7,
    // ...
    Dominant13,
    Major13,
}
```

Everything Ferrous Lineup knows about a quality is a `match` on it, one
method per fact. The notes, as semitones above the root:

```rust
pub fn intervals(self) -> &'static [u8] {
    use ChordQuality::*;
    match self {
        Major => &[0, 4, 7],
        Minor => &[0, 3, 7],
        Diminished => &[0, 3, 6],
        // ...
        Dominant9 => &[0, 4, 7, 10, 14],
        // ...
        // Players leave the eleventh out of thirteenth chords, because
        // it clashes with the third; so does Ferrous Lineup.
        Dominant13 => &[0, 4, 7, 10, 14, 21],
        Major13 => &[0, 4, 7, 11, 14, 21],
    }
}
```

The ninth is written as 14, not 2. As a pitch class it is the same, but
a chord placed in the piano roll is built from these numbers, and a
ninth a whole tone above the root would sound like a cluster, not a
ninth chord.

The other facts are methods of the same shape: `symbol` (the "m7" of
"Am7"), `name` ("minor seventh"), `tones` (which note of the chord each
interval is, below), `is_minor`, `is_dominant` and `numeral_suffix`
(the "°" of "vii°"). Writing knowledge as `match` arms has one big
advantage over, say, a list of structs: add a variant and the compiler
names every method that does not handle it yet.

`numeral_suffix` ends with a **binding pattern**:

```rust
match self {
    Major | Minor => "",
    Diminished => "°",
    // ...
    other => other.symbol(),
}
```

`other` matches anything left, like `_`, but gives it a name to use.
`family`, which sorts a quality into the three kinds of triad, adds a
guard to one:

```rust
pub fn family(self) -> ChordFamily {
    use ChordQuality::*;
    match self {
        Diminished | HalfDiminished7 | Diminished7 => ChordFamily::Diminished,
        quality if quality.is_minor() => ChordFamily::Minor,
        _ => ChordFamily::Major,
    }
}
```

Keys use the family to match a chord to their own chords: G7 is the
major chord on G, Bm7b5 the diminished one on B ([Roman numerals, the
harmony keys and chord functions](numerals.md)).

### Which quality is this?

A quality's notes on C are a [`PitchSet`](pitch.md#a-set-of-pitch-classes-in-twelve-bits),
and finding the quality of a group of notes seen from its root is one
search:

```rust
pub fn identify(relative: PitchSet) -> Option<ChordQuality> {
    Self::ALL
        .into_iter()
        .find(|quality| quality.pitch_set() == relative)
}
```

`find` returns the first quality that fits. So the order of
`ChordQuality::ALL` matters, and it lists the simpler, more common
chords first. For `identify` to find every quality, no two may have
the same notes, and a test makes sure of it:

```rust
#[test]
fn no_two_qualities_have_the_same_notes() {
    // Otherwise `identify` could never find the second one.
    for (i, a) in ChordQuality::ALL.iter().enumerate() {
        for b in &ChordQuality::ALL[i + 1..] {
            assert_ne!(a.pitch_set(), b.pitch_set(), "{a:?} and {b:?}");
        }
    }
}
```

`&ChordQuality::ALL[i + 1..]` is the slice of the array from `i + 1` to
the end, so every pair is compared once.

## A chord: a root and a quality

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord {
    pub root: PitchClass,
    pub quality: ChordQuality,
}
```

Its pitch classes are the quality's moved up to the root, and its notes
are stacked up from a given root note, leaving out any that would fall
off the top of MIDI:

```rust
pub fn pitch_set(self) -> PitchSet {
    self.quality.pitch_set().transpose(self.root.semitone())
}

pub fn notes_from(self, root_note: Note) -> Vec<Note> {
    self.quality
        .intervals()
        .iter()
        .filter_map(|&interval| root_note.transpose(interval as i32))
        .collect()
}
```

### The chords of a key: stacking thirds

To build the chord on a note of a key, take every other note of the
scale: the note, the one two places up, the one four places up (and six
places up for a seventh chord). This is **stacking thirds**. In C major,
starting on D gives D, F and A: D minor.

```rust
pub fn stack_thirds(self, note: Note, count: usize) -> Option<Vec<Note>> {
    if !self.is_heptatonic() {
        return None;
    }
    let intervals = self.kind.tonal_intervals();
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

Read it as a pipeline: the numbers `0..count` become places in the
scale, the places become semitones above `note`, and `filter_map` turns
the semitones into notes, dropping any that would leave the MIDI range.
The `?` after `degree_of` returns `None` early when the note is not in
the scale.

`Scale::triad(degree)` and `seventh(degree)` stack three or four notes
on a degree and name the result with `identify`. For all seven degrees
of C major, that is the table every songwriter learns:

| Degree | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|--------|---|---|---|---|---|---|---|
| Triad | C | Dm | Em | F | G | Am | Bdim |
| Numeral | I | ii | iii | IV | V | vi | vii° |
| Seventh | Cmaj7 | Dm7 | Em7 | Fmaj7 | G7 | Am7 | Bm7b5 |

Change the key and the table follows, because it is computed, not
written down. `ninth(degree)` stacks five notes, and returns `None` on
the degrees where the ninth is only a semitone above the root an octave
up (E G B D F in C major), which clashes, and no named chord fits.

### Chord stamps

What a click in the piano roll places is a **stamp**, an enum whose
variants carry what they need:

```rust
pub fn notes_at(self, note: Note, scale: Scale) -> Vec<Note> {
    match self {
        ChordStamp::Single => vec![note],
        ChordStamp::ScaleTriad => scale.stack_thirds(note, 3).unwrap_or_else(|| vec![note]),
        ChordStamp::ScaleSeventh => scale.stack_thirds(note, 4).unwrap_or_else(|| vec![note]),
        ChordStamp::Fixed(quality) => Chord::new(note.pitch_class(), quality).notes_from(note),
        ChordStamp::Set(set) => set
            .semitones()
            .into_iter()
            .filter_map(|up| note.transpose(i32::from(up)))
            .collect(),
    }
}
```

The "Chord in key" stamps are where the piano roll teaches harmony:
click any lit row and you get the chord the key gives you on that note,
without knowing its name first. The name then appears in the chord lane
above the grid. A `Set` stamp places any group of notes picked in the
set browser ([Pitch-class sets, and one harmony with four
names](sets.md)).

The **Draw** menu lists every stamp. `all()` chains two iterators: the
three special stamps, then one `Fixed` stamp per quality:

```rust
pub fn all() -> impl Iterator<Item = ChordStamp> {
    [
        ChordStamp::Single,
        ChordStamp::ScaleTriad,
        ChordStamp::ScaleSeventh,
    ]
    .into_iter()
    .chain(ChordQuality::ALL.into_iter().map(ChordStamp::Fixed))
}
```

`map(ChordStamp::Fixed)` passes the variant itself as a function: a
tuple variant like `Fixed` *is* a function from `ChordQuality` to
`ChordStamp`.

Next to the Draw menu, a second menu picks the **inversion** of a
stamped chord. Inverting moves the lowest note up an octave
(`tools::inversion`, in [Editing tools as plain
functions](tools.md)), which would move the chord away from
where you clicked, so the piano roll moves it back down an octave and
the note you clicked stays the root:

```rust
fn stamp_notes(&self, pitch: Note, scale: Scale) -> Vec<Note> {
    let notes = tools::inversion(&self.stamp.notes_at(pitch, scale), self.inversion);
    if notes.contains(&pitch) {
        return notes;
    }
    notes
        .iter()
        .map(|note| note.transpose(-12))
        .collect::<Option<Vec<Note>>>()
        .unwrap_or(notes)
}
```

Collecting an iterator of `Option`s into an `Option<Vec<_>>` gives
`None` as soon as one item is `None`. Here the type is written with a
**turbofish**, `collect::<...>()`, because `unwrap_or` is called on the
result straight away, and Rust must know the result's type to find that
method. If a note would fall off the bottom of MIDI, the chord stays as
it was. The outline that follows the pointer is drawn with the same
function, so what you see is what a click places.

## Spelling a chord

C♯7 and D♭7 are played on the same keys, but written differently. A
chord is spelled up from its root by letters: the third is two letters
up, the fifth four, the seventh six.

| Chord | Root | Third | Fifth | Seventh |
|-------|------|-------|-------|---------|
| C♯7 | C♯ | E♯ | G♯ | B |
| D♭7 | D♭ | F | A♭ | C♭ |

The seventh of D♭7 is C♭, not B: six letters above D is C. So each
quality also says which note of the chord each interval is, counted
the way musicians count:

```rust
pub fn tones(self) -> &'static [u8] {
    use ChordQuality::*;
    match self {
        Major | Minor | Diminished | Augmented => &[1, 3, 5],
        Sus2 => &[1, 2, 5],
        Sus4 => &[1, 4, 5],
        // ...
        Dominant13 | Major13 => &[1, 3, 5, 7, 9, 13],
    }
}
```

and `spell` walks the intervals and the tones together with `zip`,
asking [`NoteName::above`](pitch.md#intervals-count-letters-too)
for each note: so many semitones, `tone - 1` letters up:

```rust
pub fn spell(self, root: NoteName) -> Vec<NoteName> {
    self.quality
        .intervals()
        .iter()
        .zip(self.quality.tones())
        .map(|(&interval, &tone)| root.above(interval % 12, tone - 1))
        .collect()
}
```

That gives A major as A C♯ E, C diminished seventh as C E♭ G♭ B𝄫 (its
seventh is a seventh, even though it sounds like a sixth), and E7♯9 as
E G♯ B D F𝄪: the sharp ninth is a ninth, one letter above the root, so
it is a kind of F, not G.

### A name for the root

Spelling needs the root's name to start from. In a key, the key gives
it: `Scale::spell_root` writes a root in the scale as the scale writes
it, and a root from outside the scale from the tonic, as
[`Scale::spell`](scales.md#notes-from-outside-the-key) does
(E♭ major in C major, the chord borrowed from C minor). One more rule
keeps chord names readable: if that root would spell the chord with two
sharps or flats on one note, the other name of the root is used. In D♭
major, the chord a semitone above the tonic is written D major, not
E𝄫 major.

```rust
let doubled = |root: NoteName| chord.spell(root).iter().any(|n| n.accidental.abs() >= 2);
if !doubled(strict) {
    return strict;
}
let other = chord.root_name();
if doubled(other) {
    strict.simplified()
} else {
    other
}
```

Without a key, as for a twelve-tone row, `Chord::root_name` picks the
sharp or the flat name of the root, whichever spells the whole chord
with fewer signs. C♯ major is C♯ E♯ G♯, three sharps; D♭ major is D♭ F
A♭, two flats; so it is D♭.

```rust
let usual = NoteName::chromatic(self.root);
[Spelling::Sharps, Spelling::Flats]
    .into_iter()
    .map(|spelling| NoteName::plain(self.root, spelling))
    .fold(usual, |best, other| {
        if accidentals(other) < accidentals(best) {
            other
        } else {
            best
        }
    })
```

`fold` carries a value through the iterator: it starts with `usual`
and, for each item, the closure returns the value to carry on. Here it
keeps whichever name has fewer signs, and on a tie the one it already
has, the usual name of the black key.

`Scale::chord_name(chord)` and `Scale::chord_notes(chord)` put it
together: the chord on the third note of A major is "C♯m", and in D
minor A major's notes are A C♯ E.

## Chord symbols: reading a lead sheet

A song you want the band to play usually comes as a **lead sheet**, and
a lead sheet names its chords: "C | G/B | Am | Am/G". The chord sheet
reads exactly that ([The chord sheet: model, layout and
text](../band/chord-sheet.md)), through `ChordSymbol`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChordSymbol {
    /// The root, spelled as written: Db7 keeps its Db, it is not C#7.
    pub root: NoteName,
    pub quality: ChordQuality,
    /// The note under the chord, after the slash: the B of "G/B". `None`
    /// when the root is in the bass, as usual.
    pub bass: Option<NoteName>,
}
```

A symbol keeps its root as written, a `NoteName`, so a chart in D♭
stays in D♭. To play it, `chord()` turns it into a `Chord`, a pitch
class and a quality.

### Many ways to write one chord

Musicians write the same quality in several ways: a minor seventh is
Cm7, Cmin7 or C-7; a major seventh Cmaj7, CM7 or CΔ7; a half-diminished
chord Cm7b5 or Cø. All of them are read, from a table of tuples:

```rust
const SPELLINGS: [(ChordQuality, &[&str]); 30] = {
    use ChordQuality::*;
    [
        (Major, &["", "maj", "M"]),
        (Minor, &["m", "min", "mi", "-"]),
        (Diminished, &["dim", "°", "o"]),
        // ...
        (Major13, &["maj13", "M13", "Δ13"]),
    ]
};
```

Each entry pairs a quality with a slice of strings, `&[&str]`. Slices
can have different lengths, so one quality can have one spelling and
another six in the same array. The block `{ use ChordQuality::*; [...] }`
is an expression whose value is its last line, so the `use` reaches only
the table. Reading a quality cleans the text and searches:

```rust
fn read_quality(text: &str) -> Option<ChordQuality> {
    let plain: String = text
        .chars()
        .filter(|c| !matches!(c, '(' | ')' | ' '))
        .map(|c| match c {
            '♭' => 'b',
            '♯' => '#',
            other => other,
        })
        .collect();
    SPELLINGS
        .iter()
        .find(|(_, spellings)| spellings.contains(&plain.as_str()))
        .map(|(quality, _)| *quality)
}
```

Brackets and spaces are dropped, so "C7(b9)" reads like "C7b9", and ♭
and ♯ work like `b` and `#`. Upper and lower case matter in one place,
an important one: CM7 is a major seventh (C E G B) and Cm7 a minor
seventh (C E♭ G B♭). When Ferrous Lineup writes a chord back, it uses
one way for each, the one `symbol` gives: Cmaj7, Cm7, Cm7b5, Cdim7. A
test writes every quality on every root, with and without a bass, and
reads it back, so the table and `symbol` cannot drift apart.

### Slash chords, and the six-nine chord

A slash puts another note in the bass. There are two kinds:

- **Inversions**: the bass note is in the chord. C/E is C major with
  its third in the bass, C/G with its fifth, G7/F with the seventh of
  G7. `ChordSymbol::bass_in_chord` says which: 3, 5 or 7.
- **A chord over a bass note**: the bass is not in the chord. F/G is F
  major over a G, an open sound that pop and gospel players often use
  instead of G11. `bass_in_chord` says `None`.

One slash is not a bass note: the one in C6/9, the six-nine chord. So
only what follows the **last** slash, and only if it is a note name, is
a bass:

```rust
fn split_bass(rest: &str) -> (&str, Option<NoteName>) {
    match rest.rsplit_once('/') {
        Some((before, after)) => match NoteName::parse(after.trim()) {
            Some(bass) => (before, Some(bass)),
            None => (rest, None),
        },
        None => (rest, None),
    }
}
```

`rsplit_once('/')` splits the text at its last slash into the parts
before and after it, or gives `None` when there is none. "C6/9" is a
six-nine chord, and "C6/9/E" a six-nine chord over E.

### Reading a whole symbol

`ChordSymbol::parse` reads the root with
[`NoteName::read_either`](../program/languages.md#reading-a-name-either-way),
in letters or in solfège, keeping the reading whose rest is a chord, so
"Faug" is F augmented and "Lam7" is A minor seventh:

```rust
let read = NoteName::read_either(text, |rest| read_rest(rest).is_some());
let Some((root, rest)) = read else {
    return Err(match NoteName::read_either(text, |_| true) {
        None => trf(
            "\"{}\" does not start with a note: chords start with a letter from A \
             to G, like C, F#m or Bb7, or with a syllable, like Do, Fa#m or Sib7.",
            &[&text],
        ),
        Some((_, rest)) => trf(
            "\"{}\": \"{}\" is not a kind of chord Ferrous Lineup knows, \
             such as m, 7, maj7, m7b5, sus4 or 13.",
            &[&text, &split_bass(rest).0],
        ),
    });
};
```

The error says what went wrong, so the chord sheet can show it next to
the bar where it was typed: either the text does not start with a note,
or what follows the note is no chord Ferrous Lineup knows.

`ChordSymbol` also implements the standard trait `FromStr`, "can be
read from text", by calling `parse`. With it, `str::parse` works for
chords as it does for numbers:

```rust
impl FromStr for ChordSymbol {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ChordSymbol::parse(text)
    }
}
```

so `let chord: ChordSymbol = "Am7".parse()?;` reads a chord.

### Moving a chart to another key

To move a song to another key, a musician moves every chord by the
same distance, counted in letters as well as semitones. From E minor to
D minor every letter goes down one:

| E minor | F♯m7b5 | B7 | Em |
|---------|--------|----|----|
| D minor | Em7b5 | A7 | Dm |
| C minor | Dm7b5 | G7 | Cm |
| B♭ minor | Cm7b5 | F7 | B♭m |
| G♯ minor | A♯m7b5 | D♯7 | G♯m |

This is a **ii–V–i**, the most common way to arrive in a minor key: in
every key the ii is one letter above the tonic and the V four letters
above. Counting by letters keeps it that way: in G♯ minor the V is D♯7,
never E♭7. `transposed(from, to)` does it, moving the root and the bass
with `NoteName::above` and taking the simpler name for a root that
would need two signs, since chord symbols never have them ([Pitch
classes, notes and
spelling](pitch.md#intervals-count-letters-too)). A test
checks the ii–V–i in all twelve keys, written with sharps and with
flats.

## Naming what is playing

The piano roll names whatever sounds at each step: nothing, one note,
two notes, a chord it knows, or a group with no chord name. Each answer
needs different data, which is a job for an enum whose variants carry
data:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Harmony {
    /// No notes at all.
    Silence,
    /// One pitch class, maybe in several octaves.
    Single(PitchClass),
    /// Two pitch classes. `low` is the one in the bass, and `semitones`
    /// counts up from it to the other.
    Interval {
        low: PitchClass,
        high: PitchClass,
        semitones: u8,
    },
    /// A chord we know. When `bass` is not the chord's root, the chord is
    /// an inversion, written with a slash: C/E is C major with E in the bass.
    Chord { chord: Chord, bass: PitchClass },
    // ...
    /// Three or more pitch classes that are not a chord we know, named
    /// by their set class, such as "4-23" for a stack of fourths.
    Unknown(PitchSet),
}
```

One more variant, `Polychord`, is two chords at once, such as the E♭7
over F♭ major that opens the Augurs of Spring in The Rite of Spring.
Only the chord hearing finds those, from how the notes are grouped
([Hearing harmony: chords and keys from
notes](hearing.md)); the notes alone cannot say which belong
to which chord.

`analyze` takes any notes and returns a `Harmony`:

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
        2 => {
            let high = pitch_set.iter().find(|&p| p != bass).unwrap_or(bass);
            Harmony::Interval {
                low: bass,
                high,
                semitones: bass.semitones_up_to(high),
            }
        }
        _ => match find_chord(pitch_set, bass) {
            Some(chord) => Harmony::Chord { chord, bass },
            None => Harmony::Unknown(pitch_set),
        },
    }
}
```

`impl IntoIterator<Item = Note>` in an *argument* means "anything that
can be looped over and gives notes": a `Vec<Note>`, an array, or an
iterator straight from a pattern
([`sounding_at`](../program/editors.md#ghost-notes-hearing-the-other-channels)), with nothing collected in
between. One loop builds the set and finds the lowest note, and a
`match` on the number of pitch classes picks the kind of answer.

### Finding the chord

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
        ChordQuality::identify(pitch_set.relative_to(root)).map(|quality| Chord::new(root, quality))
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

- `roots` is a **closure that makes an iterator**. An iterator is used
  up as it goes, so to search twice the code calls `roots()` again for
  a fresh one.
- `find_map` is `find` and `map` in one: it calls the closure on each
  item and returns the first `Some`.
- `or_else` on an `Option` runs its closure only if the option is
  `None`. The second search, which allows the fifth to be missing, runs
  only when the exact search found nothing. Players often leave the
  fifth out of seventh chords, and C, E and B♭ is still heard as C7.
- `Option::filter` keeps a `Some` only if the closure agrees. It stops
  "C and E, plus a guessed G" from being named a whole C major chord:
  guessing is allowed only for chords of four notes or more.

Starting with the bass settles real ambiguities. C E G A is both C6 and
Am7: over a C it is C6, over an A it is Am7. C D G is both Csus2 and
Gsus4. Big chords overlap all the time: Am11 and Cmaj13 are the same
six pitch classes, A B C D E G, and the bass names them.

```rust
// C E G A is both C6 and Am7: the lowest note decides.
assert_eq!(label(&[60, 64, 67, 69]).unwrap(), "C6");
assert_eq!(label(&[57, 60, 64, 67]).unwrap(), "Am7");
```

### Labels: match guards and nested patterns

The chord lane above the piano roll shows a short label for each step:

```rust
pub fn label(&self, spelling: Spelling) -> Option<String> {
    match *self {
        Harmony::Silence => None,
        Harmony::Single(pitch_class) => Some(pitch_class.name(spelling).to_string()),
        // A root and its fifth is a "power chord", written C5.
        Harmony::Interval {
            low, semitones: 7, ..
        } => Some(format!("{}5", low.name(spelling))),
        // A dash, not a plus: "C+" would read as C augmented.
        Harmony::Interval { low, high, .. } => {
            Some(format!("{}–{}", low.name(spelling), high.name(spelling)))
        }
        Harmony::Chord { chord, bass } if bass != chord.root => {
            Some(format!("{}/{}", chord.name(spelling), bass.name(spelling)))
        }
        Harmony::Chord { chord, .. } => Some(chord.name(spelling)),
        // ...
        Harmony::Unknown(set) => Some(set.set_class().name()),
    }
}
```

- `semitones: 7` inside a pattern matches only that value. Other
  intervals fall through to the next arm.
- `..` means "and the other fields, whatever they are".
- `if bass != chord.root` is a **match guard**: inversions get a slash,
  root-position chords do not.
- Arms are tried from the top, so the more specific arm comes first.
  If the plain `Harmony::Chord { chord, .. }` came first, the slash arm
  could never match.
- `match *self` matches on the value behind the reference. That works
  because every field is `Copy`, so the patterns copy them out.

A group with no chord name gets the name of its **set class**, such as
3-1 for three neighbouring semitones ([Pitch-class sets, and one
harmony with four names](sets.md)).

`label` names black keys with plain sharps or flats. `label_in(scale)`
spells the same answer in a key, with `Scale::spell` and
`Scale::spell_root`: E♭ rather than D♯ in C major, A/C♯ in D minor,
E♯dim in F♯ major.

`describe(scale)` writes a sentence for the piano roll's line about
the step under the pointer, and for the status bar while you hold keys,
such as "Am: A minor, chord vi of C major." or "C/E: C major
with E in the bass, chord I⁶ of C major." It says whether a chord is
one of the key's, **borrowed** from the parallel key, or a **secondary
dominant**; those words are explained in [Roman numerals, the harmony
keys and chord functions](numerals.md).

### Naming an interval

Two notes are an **interval**, and an interval has two names. Its size
in semitones gives the everyday name, a table of twelve
(`interval_name`: 3 is a "minor third", 6 a "tritone"). But musicians
name it from its spelling: the letters give the number, the semitones
the quality. C to E is a third (C, D, E), and so are C to E♭ and C♯ to
E; a third of four semitones is major, of three minor. So C up to D♯ is
an augmented second, even though it sounds like the minor third C up
to E♭.

```rust
let letters = usize::from(low.letter.steps_up_to(high.letter));
let semitones = low.pitch_class().semitones_up_to(high.pitch_class()) as i8;
// How far it is from the major or perfect interval, from -6 to 5.
let off = (semitones - MAJOR[letters] + 18).rem_euclid(12) - 6;
let perfect = matches!(letters, 0 | 3 | 4);
let quality = match (perfect, off) {
    (true, 0) => "perfect",
    (false, 0) => "major",
    (false, -1) => "minor",
    (_, 1) => "augmented",
    (true, -1) | (false, -2) => "diminished",
    (_, 2) => "doubly augmented",
    _ => "doubly diminished",
};
```

Unisons, fourths and fifths are "perfect" and have no major or minor;
one semitone smaller than perfect is diminished, where for the others
diminished is one smaller than minor. Adding 18 and taking
`rem_euclid(12)` before subtracting 6 folds any difference into -6 to
5, so a seventh that wraps past the octave still compares correctly.

In a key, the two notes take the key's names, so in A harmonic minor F
up to G♯ really is an augmented second, the scale's famous gap. A note
from outside the key that would make an odd interval is named the
usual way for the interval's size instead: in C major, C and E♭ make a
minor third.

## Inversions

A chord symbol writes an inversion with a slash. A Roman numeral
writes it with the figures of **figured bass**, the shorthand of
Baroque keyboard players, which are the intervals above the bass:

| Chord | Lowest note | Chord symbol | Numeral in C major |
|-------|-------------|--------------|--------------------|
| C major | C, the root | C | I |
| C major | E, the third | C/E | I⁶ (first inversion) |
| C major | G, the fifth | C/G | I⁶₄ (second inversion) |
| G7 | G, the root | G7 | V7 |
| G7 | B, the third | G7/B | V⁶₅ (first inversion) |
| G7 | D, the fifth | G7/D | V⁴₃ (second inversion) |
| G7 | F, the seventh | G7/F | V⁴₂ (third inversion) |

C/E has C a sixth above E and G a third above it, "6/3", shortened to
⁶. A seventh chord has three intervals above its bass; the figures keep
the two that tell its inversions apart, and stand in for the 7: G7/B is
V⁶₅, not V7⁶₅.

`Chord::tone_of` finds which note of the chord a pitch is, walking the
intervals and their tone numbers together:

```rust
pub fn tone_of(self, pitch_class: PitchClass) -> Option<u8> {
    let semitones = self.root.semitones_up_to(pitch_class);
    self.quality
        .intervals()
        .iter()
        .zip(self.quality.tones())
        .find(|(interval, _)| **interval % 12 == semitones)
        .map(|(_, &tone)| tone)
}
```

and `inversion_figures` turns that into figures with one `match` on a
tuple:

```rust
match (self.tone_of(bass), triad, seventh) {
    (Some(3), true, _) => "⁶",
    (Some(5), true, _) => "⁶₄",
    (Some(3), _, true) => "⁶₅",
    (Some(5), _, true) => "⁴₃",
    (Some(7), _, true) => "⁴₂",
    _ => "",
}
```

Sus, sixth and ninth chords have no figures; they keep the slash name.
How the figures go onto a numeral, and what an inversion changes about
where a chord goes next (the cadential six-four), are in [Roman
numerals, the harmony keys and chord
functions](numerals.md).

### One track, or all of them

The piano roll's chord lane has two rows. The first names the chords of
the track being edited, step by step, with `analyze`. The band's piano
may play G7 as B D F G, so on its own it is G7/B. The second row,
**All**, names what the whole band plays there, beat by beat: if the
bass plays F under that G7, the band plays G7/F, V⁴₂. Naming the whole
band is harder than naming one track, because the melody and the bass
line have passing notes that are not in the chord, and an arpeggio
plays its chord one note at a time. That is the job of the chord
hearing, in [Hearing harmony: chords and keys from
notes](hearing.md).

## Testing it

The tests read like a harmony textbook. Every quality's notes are
different, every quality has a tone number for each interval, and the
chords are spelled by their letters:

```rust
assert_eq!(
    spelled(name(Letter::C, 0), ChordQuality::Diminished7),
    "C Eb Gb Bbb"
);
assert_eq!(
    spelled(name(Letter::E, 0), ChordQuality::Dominant7Sharp9),
    "E G# B D F##"
);
```

The analysis names real voicings, in any order and octave, and without
their fifth:

```rust
assert_eq!(label(&[48, 64, 67, 69, 74]).unwrap(), "C6/9");
assert_eq!(label(&[55, 59, 62, 65, 68]).unwrap(), "G7b9");
// E, G#, B, D and G: the "Hendrix chord".
assert_eq!(label(&[52, 56, 59, 62, 67]).unwrap(), "E7#9");
```

```rust
assert_eq!(label(&[48, 64, 70]).unwrap(), "C7");
```

The descriptions use the key, inversions included:

```rust
assert_eq!(describe(&[57, 60, 64]), "Am: A minor, chord vi of C major.");
```

And the chord symbols are read in every way of writing them, from "C"
and "Am7" to "C7(b9)", "Bø" and "Cm(maj7)", and moved to all twelve
keys by their letters.

Exercises on chords are in [Exercises: music theory in
code](exercises.md).
