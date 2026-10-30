# Pitch-class sets, and one harmony with four names

Files: [`crates/lineup-core/src/harmony/set_class.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/set_class.rs),
[`crates/lineup-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/pitch_set.rs),
[`crates/lineup-core/src/chord_sheet/notation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/notation.rs),
[`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs)
and [`crates/lineup-app/src/chord_sheet/notation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/notation.rs)

Chord names work for the chords of tonal music. Much music of the 20th
century uses groups of notes that have no chord name, such as C, C♯ and
F♯. **Pitch-class set theory**, worked out by Milton Babbitt, Allen
Forte, John Rahn and others, gives every group of notes a name. In
Ferrous Lineup a set is one more way to *write* a harmony, next to
chord symbols, Roman numerals and micromodes: switching to sets never
moves a note. This chapter is about the theory, the few lines of bit
arithmetic that compute it, and the places the app uses it: the set
browser, a key shown as a set, and the chord sheet's four notations.

## The music, briefly

**Pitch classes as numbers.** C is 0, C♯ is 1, and so on up to B, 11.
Octaves and spellings do not count: every C is 0, and C♯ and D♭ are
both 1. A **set** is a choice of some of the twelve; there are
2¹² = 4,096 of them, from the empty set to all twelve.

**Moving a set.** *Transposing* (T*n*) moves every note up *n*
semitones: C E G up 3 is E♭ G B♭. *Inverting* (I) turns every interval
upside down: C E G (up 4, then up 3) becomes C A♭ F (down 4, then down
3), which is F minor. Upside down, a major triad is a minor triad.

**Set classes.** A set together with all its transpositions and
inversions is a **set class**. All 24 major and minor triads are one
class. The 4,096 sets fall into **224 classes**.

**Normal form** is the most compact way to write a set. Written around
a circle, a set can start on any of its notes; start on each in turn
and measure the span from the first note to the last. Take C major
seven, C E G B:

| starting on | notes | span | to the note before last |
|-------------|-------|------|----------------|
| C | C E G B | 11 | |
| E | E G B C | 8 | 7 |
| G | G B C E | 9 | |
| B | B C E G | 8 | 5 |

E and B tie with a span of 8, so compare the span to the note before
last: B to E is 5, E to B is 7. The normal form is **B C E G**. (This
tie-break, comparing from the end, is John Rahn's; Allen Forte compared
from the start, and the two disagree for six classes. Ferrous Lineup
uses Rahn's, as most textbooks now do.)

**Prime form** goes one step further: take the normal forms of the set
*and of its inversion*, move both to start on 0, and keep the more
compact one. For Cmaj7 it is **[0158]**: B C E G moved down to start on
0 is 0 1 5 8. Every major seventh chord, on any root, has the prime form
[0158].

**Forte numbers** list the classes by size, then in Forte's order:
[0158] is **4-20**, the twentieth class of four notes. The triads are
3-11, the diminished seventh chord 4-28, the major scale 7-35, the
whole-tone scale 6-35. Books on 20th-century music use these numbers
everywhere.

**The interval vector.** Every pair of notes in a set makes an
interval. Counting them by size gives six numbers: how many minor
seconds, major seconds, minor thirds, major thirds, fourths and
tritones. A fifth counts as a fourth (it is a fourth upside down) and a
major seventh as a minor second, so six sizes are enough. The vector is
a fair summary of what a set *sounds* like. For C E G B:

```text
C-E major third   C-G fourth   C-B minor second
E-G minor third   E-B fourth   G-B major third
<1 0 1 2 2 0>, written <101220>
```

Two curiosities follow from it:

- **Z-related** classes have the same interval vector without being the
  same class. 4-Z15 [0146] and 4-Z29 [0137] each hold one interval of
  every size, `<111111>` (they are the two **all-interval tetrachords**),
  yet neither can be moved onto the other. The **Z** in their names
  marks them.
- **Complements**: the notes a set leaves out make another set. The
  white keys (7-35) leave out the black keys, the pentatonic scale
  (5-35). A set of six notes and its complement always have the same
  interval vector.

## Set theory in twelve bits

A `PitchSet` is twelve bits of a `u16`, bit 0 for C up to bit 11 for B,
and transposing is rotating the bits (see [Pitch classes, notes and
spelling](pitch.md#a-set-of-pitch-classes-in-twelve-bits)).
The rest of set theory fits those bits just as well.

**The complement is `!`.** "Not" flips every bit, and `from_bits` keeps
the twelve that mean notes:

```rust
pub fn complement(self) -> PitchSet {
    PitchSet::from_bits(!self.bits())
}
```

**Inverting** turns each pitch class *p* into 12 − *p*, which reflects
the set around C:

```rust
pub fn invert(self) -> PitchSet {
    self.iter()
        .map(|pitch_class| PitchClass::from_semitone(12 - pitch_class.semitone()))
        .collect()
}
```

**The most compact form is the smallest number.** This is the neat
part. Move a set down so that its first note is C, bit 0. Its last note
is now its highest bit, so a smaller span means a smaller highest bit,
which means a smaller number. On a tie, the next bit down is the note
before last, which is exactly Rahn's tie-break. Comparing two numbers
*is* Rahn's rule:

| starting on | moved to C | bits | number |
|-------------|-----------|------|--------|
| C | 0 4 7 11 | `1000_1001_0001` | 2193 |
| E | 0 3 7 8 | `0001_1000_1001` | 393 |
| G | 0 4 5 9 | `0010_0011_0001` | 561 |
| B | 0 1 5 8 | `0001_0010_0011` | 291 |

So the normal form is "the start that gives the smallest number", and
the prime form "the smallest number of all 24 transpositions and
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
sets, so the iterator yields 24 sets, and the smallest is the prime
form. No sorting and no lists of intervals: a few integer comparisons.
`relative_to(start)` moves the set down until `start` is C.

How one set is moved onto another, as T*n* or T*n*I, is a search over
the same 24 moves:

```rust
pub fn moved_to(self, other: PitchSet) -> Option<(u8, bool)> {
    let inverted = self.invert();
    (0..12)
        .find(|&n| self.transpose(n) == other)
        .map(|n| (n, false))
        .or_else(|| {
            (0..12)
                .find(|&n| inverted.transpose(n) == other)
                .map(|n| (n, true))
        })
}
```

`or_else` runs its closure only when the first search found nothing,
so a transposition is always preferred to an inversion. A class is
**symmetric** when inverting its prime form gives a transposition of
it, as for the diminished triad: upside down, it is the same set moved.

The interval vector is the one place that counts pairs the plain way, a
loop inside a loop. With at most twelve notes that is at most 66 pairs:

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
together: 7, a fifth, becomes 5, a fourth.

## 224 classes in a table

The prime form can be computed, but the Forte *number* cannot: it is
only a place in Forte's list. So `set_class.rs` holds the list, a
constant array of 224 entries:

```rust
const SET_CLASSES: [SetClass; 224] = [
    SetClass::new(0, 1, false, 0),
    SetClass::new(1, 1, false, 0b000000000001),
    SetClass::new(2, 1, false, 0b000000000011),
    // ...
    SetClass::new(4, 20, false, 0b000100100011),
    // ...
    SetClass::new(12, 1, false, 0b111111111111),
];
```

`SetClass::new` and `PitchSet::from_bits` are `const fn`, so the whole
table is built while compiling and stored with the program; looking a
set up costs nothing at start-up. The `false` or `true` says whether
the class has a Z in its name. `0b` writes a number in binary, so the
prime form [0158] can be read off the bits.

A table of 224 lines of numbers is easy to get wrong, so the tests
check it against the theory rather than trusting it:

- every entry is its own prime form, and the entries are in order of
  size and number;
- there are 1, 1, 6, 12, 29, 38, 50, 38, 29, 12, 6, 1 and 1 classes of
  0 to 12 notes, the counts every textbook gives;
- **all 4,096 sets** are looked up, and each lands in a class of its
  own size;
- a class has a Z exactly when another class of its size has the same
  interval vector;
- for 5-20, where Forte and Rahn pack the notes differently, the prime
  form is Rahn's.

Because of the third test, the lookup can state what it expects instead
of handling a "not found" that cannot happen:

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
is a promise, and the test is what keeps the promise. The function
returns `&'static SetClass`: a borrow of an entry in the constant table,
which lives as long as the program, so a caller can keep it as long as
it likes.

The class writes its own names. `vector_text` writes the six counts as
digits, `<001110>`, the way the books do, until one of them reaches 10:
all twelve notes hold twelve of every interval but the tritone, and that
is written with spaces, `<12 12 12 12 12 6>`, so it cannot be misread.
Some classes also have a familiar name, from a short list of examples:
"major or minor triad", "whole-tone scale", "octatonic (diminished)
scale", "Petrushka chord".

## The set browser

**Sets…**, in the piano roll's toolbar, opens the **Pitch-class sets**
window: every class of the size picked (2 to 10 notes), each with its
Forte number, prime form, interval vector and familiar name, such as
"3-11 [037] <001110> major or minor triad". The search box finds a class
by number or by name ("4-Z15", "triad", "octatonic"), and with two or
more notes selected in the piano roll, **Show it** jumps to their class.
Picking a class shows its Z-partner if it has one, the class of its
complement, and whether it is symmetric. **Move it up** and **Inverted
(I)** choose one member of the class: T3 is the set three semitones up,
T3I its inversion moved up three. Then:

- **▶ Hear it** plays the set on the selected instrument;
- **Draw with it** makes the set the piano roll's chord stamp, so each
  click places the whole set above the note clicked
  (`ChordStamp::Set`, next to the chord stamps of [Chords: building,
  naming and inversions](chords.md#chord-stamps));
- **Use as the key** makes the set the song's key: the piano roll
  highlights its notes and the harmony keys play chords stacked from it.

A set of three notes is also one of Kröpfl's micromodes, and the window
links to it (see [Micromodes](micromodes.md)).

The piano roll names sets without the window, too. Under the toolbar,
next to the chord at the step under the pointer, it shows that step's
set when three or more notes sound there, and the set of the selected
notes when two or more are selected:

```text
Set: 3-11 · normal form G# B D# · prime form [037] · intervals <001110> · major or minor triad
```

## A key as a set: the same notes, another name

Picking **pitch-class set** in a Key menu could mean two things: play
in another scale, or name the same notes another way. Ferrous Lineup
takes the second meaning, because a set is a notation. C major already
*is* a pitch-class set, C D E F G A B, the class 7-35. So switching the
key from C major to "pitch-class set" keeps every note of the song and
only shows the same harmony in set notation:

| Harmony key | Notes | Set class |
|---|---|---|
| C | C E G | 3-11 |
| Dm | D F A | 3-11 |
| Em | E G B | 3-11 |
| F | F A C | 3-11 |
| G | G B D | 3-11 |
| Am | A C E | 3-11 |
| Bdim | B D F | 3-10 |

The table is a lesson on its own. Major and minor triads are the
**same** class, 3-11, because a minor triad is a major triad upside
down, and a set class counts a set and its inversion as one. The
diminished triad is another class, 3-10.

Switching between scales is different: from major to minor, E, A and B
become E♭, A♭ and B♭, and the song moves (see [Scales, modes and
keys](scales.md#changing-key-moving-the-song)). A set you
choose in the set browser with **Use as the key** may be any set at all.

`set_key`, in the app, picks the set the menu switches to:

```rust
fn set_key(ctx: &egui::Context, scale: Scale, root: PitchClass) -> Scale {
    let up = scale.root.semitones_up_to(root);
    if let Some(group) = scale
        .tone_row()
        .and_then(|row| row.segments(3).first().copied())
    {
        return Scale::set(root, group.transpose(up));
    }
    if scale.kind.is_tonal() && scale.kind != ScaleKind::Chromatic {
        let set = Scale::set(root, scale.pitch_set().transpose(up));
        // Gb major as a set still writes its notes Gb Ab Bb Cb...
        return if up == 0 {
            set.with_spelling(scale.spelling())
        } else {
            set
        };
    }
    let set = remembered_set(ctx);
    let start = set.normal_form().first().copied().unwrap_or(root);
    Scale::set(root, set.transpose(start.semitones_up_to(root)))
}
```

From a key it is the key's own scale; from a twelve-tone row, the row's
first three notes, which are what the harmony keys were playing; from
the chromatic scale, which holds every note and says nothing about the
harmony, the set used last. A set whose notes are exactly a scale on its
root is spelled as that scale, and moves on as that key when the key
changes again (`set_as_scale` and `as_key`, in [Scales, modes and
keys](scales.md)).

## One harmony, four names

The chord sheet applies the same rule to every harmony on it. Its top
row has a **Show** menu: **As written**, **Chords**, **Pitch-class
sets** or **Micromodes**. The notes stay the same, only their names
change, and nothing in the song is touched:

| Written | Chords | Pitch-class sets | Micromodes |
|---------|--------|------------------|------------|
| C | C | 3-11 T7I | Acorde menor-mayor T7I |
| Dm | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| G | G | 3-11 T2I | Acorde menor-mayor T2I |
| Am | Am | 3-11 T9 | Acorde menor-mayor T9 |
| Bdim | Bdim | 3-10 T11 | Acorde disminuido T11 |
| G7 | G7 | 4-27 T7I | 4-27 T7I |
| 3-11 T2 | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| Menor 3 | Menor 3 | 3-3 | Menor 3 |

How to read it:

- **T*n* and T*n*I.** Every class has a prime form that starts on C.
  For 3-11 it is [0, 3, 7], C E♭ G, a minor triad. **T9** means "the
  prime form moved up 9 semitones": A C E, A minor. **T*n*I** means
  "turned upside down first, then moved up *n*". Upside down, [0, 3, 7]
  is 0, −3 and −7: C, A and F. Moved up 7, that is G, E and C, C major.
  So every major chord has an I in its name, and no minor chord does.
  Dm (T2) and G (T2I) share the number 2 but not the notes. The prime
  form itself, T0, is written with no T at all: Menor 3 is C D♭ E, 3-3
  on C.
- **A micromode is always three notes.** Kröpfl's twelve micromodes are
  the twelve classes of three notes, 3-1 to 3-12, with the Spanish names
  he gave them. The micromode 3-11 is **Acorde menor-mayor**, the
  minor-major chord, since the major and minor triads are one class. G7
  has four notes, so it is no micromode, and the Micromodes view writes
  its set.
- **Chords** turns a set back into a chord **when its notes make
  one**: the set 3-11 T2 is D F A, so the view writes Dm, with the root
  spelled as the key spells it. A set that makes no chord the app knows,
  such as 3-1 or Menor 3, keeps its name. **N.C.** (no chord) stays
  N.C. in every view, and a row form (P0, RI7) stays a row form: a row is a line
  of twelve notes, not one group.

A polychord is two groups of notes, one over the other, and merging them
into one set would lose what makes it a polychord. So each chord is
named on its own, written upper first as the polychord is:

```text
Eb7|Fb   →   4-27 T3I|3-11 T11I
```

### In Rust: one `match` decides

`Notation` is an enum of four variants, `#[default]` on `Written` so the
sheet starts as it always was. Renaming a harmony is one `match` on a
**tuple** of the notation and the harmony. Rust tries the arms from the
top, so their order is the rule: first what never changes, then the
Chords view, then two chords at once, then everything else.

```rust
pub fn rename(self, harmony: HarmonyName, key: Scale) -> Vec<HarmonyName> {
    let group = |set: PitchSet| self.name_group(set, key);
    match (self, harmony) {
        (Notation::Written, _)
        | (_, HarmonyName::NoChord)
        | (_, HarmonyName::Row(_))
        | (Notation::Chords, HarmonyName::Chord(_) | HarmonyName::Polychord(..)) => {
            vec![harmony]
        }
        (Notation::Chords, HarmonyName::Set(set) | HarmonyName::Micromode(set)) => {
            match chord_of(set) {
                Some(chord) => vec![HarmonyName::Chord(ChordSymbol::new(
                    key.spell_root(chord),
                    chord.quality,
                ))],
                None => vec![harmony],
            }
        }
        (_, HarmonyName::Polychord(upper, lower)) => {
            // The lower chord with its bass, then the upper chord.
            let lower = notes_of(lower);
            vec![group(lower), group(upper.chord().pitch_set())]
        }
        (_, HarmonyName::Chord(symbol)) => vec![group(notes_of(symbol))],
        (_, HarmonyName::Set(set) | HarmonyName::Micromode(set)) => vec![group(set)],
    }
}
```

The first arm uses **or-patterns** at two levels: `A | B | C` between
whole tuples, and `Chord(_) | Polychord(..)` inside one. An or-pattern
that binds a name, as `Set(set) | Micromode(set)` does, must bind it in
every alternative, with the same type. The function returns a `Vec`
because a polychord becomes two names and everything else one.

`chord_of` finds the chord a set makes. It goes through the chord
qualities from the simplest, the major triad, to the most complex, and
for each one through the notes of the set, in normal form, as possible
roots:

```rust
pub fn chord_of(set: PitchSet) -> Option<Chord> {
    let roots = set.normal_form();
    ChordQuality::ALL.into_iter().find_map(|quality| {
        roots
            .iter()
            .find(|&&root| ChordQuality::identify(set.relative_to(root)) == Some(quality))
            .map(|&root| Chord::new(root, quality))
    })
}
```

`identify` says which quality the intervals above a root are, if any.
Because the qualities are the outer loop, the quality listed first wins
whatever its root: C E G A is both Am7 and C6, and is found as Am7,
since seventh chords come before sixth chords in the list. `&&root` in
the closure's pattern takes apart a reference to a reference: `iter`
gives `&PitchClass`, and `find` passes its closure a reference to that.

## Reading the notes themselves

Renaming the sheet's harmonies is half of what an analyst does with
sets. Music without chord symbols, a Webern concerto or a page of the
Rite of Spring, is cut into sets **from its notes**, in two ways:

- **vertically**, the notes that sound together, as a stacked chord or a
  cluster is heard;
- **horizontally**, the notes one after another in a line, as a
  twelve-tone row is cut into groups of three or six.

With **Pitch-class sets** or **Micromodes** shown, a second menu picks
which: **The sheet's harmonies** (renamed, as above), **Notes together
(vertical)** or **Notes in a line (serial)**. Reading notes, the sheet
shows the groups it finds, each where it starts, and leaves what is
written alone: typing in a bar still writes its harmony, and **As
written** shows it again. Two more choices say how many notes make a
group (2 to 12, or **All notes** sounding together; a micromode always
has three) and whose notes are read. **Auto** reads the tune for a line
(the track called Lead, or else the highest one, as the lead sheet picks
it), because a line through every instrument at once would mix the
melody with the chords; for notes together it reads every track, since a
chord is usually shared between instruments.

### Notes together

Wherever a note starts, `together` takes all the notes held at that
moment, from the lowest up, keeps one of each pitch class (an octave
doubling is the same note in a set) and cuts them into groups of the
size chosen. Two small rules keep the groups musical:

- a single note left over at the top joins the group below it: seven
  notes in threes are 3 + 4, not 3 + 3 + 1, since one note is no
  harmony;
- a moment where fewer than two different notes sound is skipped.

```rust
fn chunks(notes: Vec<Note>, size: Option<usize>) -> Vec<NoteGroup> {
    if notes.is_empty() {
        return Vec::new();
    }
    let Some(size) = size else {
        return vec![NoteGroup { notes }];
    };
    let mut groups: Vec<NoteGroup> = notes
        .chunks(size.max(1))
        .map(|chunk| NoteGroup {
            notes: chunk.to_vec(),
        })
        .collect();
    if groups.len() > 1 && groups.last().is_some_and(|last| last.notes.len() == 1) {
        let left = groups.pop().map(|last| last.notes).unwrap_or_default();
        if let Some(under) = groups.last_mut() {
            under.notes.extend(left);
        }
    }
    groups
}
```

`chunks` on a slice cuts it into pieces of `size`, the last one maybe
shorter. `pop` takes the last group out, and `last_mut` reaches the one
that is now last, to add the leftover note to it.

The Augurs chord shows why the first rule matters. From the bottom up
the strings play F♭ A♭ C♭, then G B♭ D♭, then E♭ on top. In threes that
is F♭ A♭ C♭ | G B♭ D♭ | E♭, and the lone E♭ joins the group below:
G B♭ D♭ E♭, which is E♭7. The sheet writes **4-27 T3I|3-11 T11I**, the
two chords Stravinsky put together, found from the notes alone. With
**All notes**, the seven notes are one set, **7-32 T7**: the class of the
harmonic minor scale, and the notes are exactly A♭ harmonic minor. Two
ways to look at one chord: two familiar chords stacked, or one scale
sounding at once.

The Petrushka chord works the same way. C E G under F♯ A♯ C♯, in threes,
is **3-11 T1I|3-11 T7I**, F♯ major over C major, and all six notes are
**6-30 T1I**, the class often called the Petrushka chord itself.

### Notes in a line

`in_line` takes the notes one after another, in the order they start
(notes struck together from the lowest up), and every so many notes make
a group. Two rules matter here:

- a note struck again straight after itself, at the same pitch, counts
  once: it is the same note played again;
- any other repeat counts: the same note in another octave, or coming
  back later, is a new note of the line.

The row of Webern's Concerto, Op. 24, is the classic example:

```text
B B♭ D | E♭ G F♯ | A♭ E F | C C♯ A
```

In threes, every group is **Menor 3** (3-3, [014]): Webern built the
whole row from one three-note cell, moved and turned upside down. In
sixes, both halves are **6-20**, the hexatonic set. All twelve in one
group are the aggregate, 12-1.

A group that is the one before it again is written once, where it
starts, as the sheet writes a chord only where it changes
(`merge_repeats`).

### Reading only when something changes

Cutting a whole song into groups and naming them takes milliseconds for
a long song: too slow to repeat sixty times a second while it plays. So the view keeps what it read, and what it read it from:

```rust
struct Read {
    from: (Notation, Grouping, Option<usize>, Option<usize>),
    notes: Vec<HeardNote>,
    keys: KeyMap,
    shown: Rc<[Shown]>,
}
```

Each frame it gathers the song's notes, which is quick, and compares
them, the choices and the keys with what it read last time. Only when
something differs, a new choice in a menu, a note edited in the piano
roll, a key change, does it cut and name the groups again.

The result is an **`Rc<[Shown]>`**, a reference-counted pointer to a
slice. `Rc` is the single-threaded cousin of `Arc` (see [Instruments:
drums, sampler and Lineup Osc](../sound/instruments.md#the-sampler)):
cloning it does not copy the names,
it only adds one to a counter. The chord sheet clones it every frame to
draw the bars while the cache keeps its own, and the names stay in one
place in memory. The whole user interface runs on one thread, so `Rc`
is enough; `Arc` would work too but would pay for thread safety that is
not needed.

The groups are sorted by the step they start on. To draw one bar, the
sheet needs the groups that start inside it, and `partition_point` finds
them by **binary search**:

```rust
let first = heard.partition_point(|shown| shown.start < bar.start);
let end = heard.partition_point(|shown| shown.start < bar.end());
let shown = heard[first..end].to_vec();
```

On a sorted slice, "starts before the bar" is `true` for a first part
and `false` for the rest, and `partition_point` returns where the
`false` part begins. It looks at about log₂(n) items instead of all of
them: about 11 for a song with 2,000 groups.

Micromode names are long, and "Acorde menor-mayor T9" does not fit in
a quarter of a bar. Where the whole name does not fit, a set or a
micromode is written by its class alone, "Acorde menor-mayor" or "3-11",
and past that as "…"; the hint under the pointer always gives the whole
name and its notes. How the sheet fits names into bars is in [The chord
sheet: model, layout and text](../band/chord-sheet.md).

The menus' choices are a small struct kept in egui's memory, which the
app saves to disk, so they are there the next time it starts:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    notation: Notation,
    grouping: Grouping,
    /// How many different notes a group of notes sounding together
    /// holds, or `None` for all of them.
    together: Option<usize>,
    /// How many notes a group of a line holds.
    line: usize,
}
```

`#[serde(default)]` on the struct means a field missing from what was
saved takes its value from `Default`. If a later version adds a field,
the choices saved by this one still load.

## Testing it

`set_class.rs` tests the theory itself: every set belongs to a class of
its size, the table holds prime forms in order with the textbook counts,
well-known classes have their well-known names (3-11, 4-28, 4-Z15, 7-35,
6-35, 5-35), Rahn and Forte differ for 5-20, Z-related classes share
their intervals, normal forms pack the notes tightly, and the
complement of the major scale is the pentatonic scale.

`chord_sheet/notation.rs` tests the views: the same notes in every
notation, and every renamed harmony with exactly the notes of the
written one; sets that make a chord written as the chord (3-11 T2 is
Dm, C E G A is Am7, the fourths 4-23 T2 are A7sus4); two chords at once
as two sets; the Petrushka and Augurs chords read vertically; Webern's
row read in a line; and every note of a line counting but a repeated
one:

```rust
#[test]
fn sets_that_make_a_chord_are_written_as_the_chord() {
    assert_eq!(renamed(Notation::Chords, "3-11 T2"), "Dm");
    assert_eq!(renamed(Notation::Chords, "{C E G}"), "C");
    assert_eq!(renamed(Notation::Chords, "Acorde disminuido T11"), "Bdim");
    // C E G A is both C6 and Am7: the simpler, more common chord.
    assert_eq!(renamed(Notation::Chords, "{C E G A}"), "Am7");
    // ...
}
```

A test of the window (`ux_tests.rs`) plays the Four chords style,
clicks through the Show menu, checks the names, then switches back and
checks that the project is exactly as it was.

Exercises on sets are in [Exercises: music theory in
code](exercises.md).
