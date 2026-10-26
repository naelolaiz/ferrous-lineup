# More chords, inversions, and what comes next

Files: [`crates/lineup-core/src/harmony/chord.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord.rs),
[`crates/lineup-core/src/harmony/progression.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/progression.rs)
and [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

This chapter is half music theory and half Rust. The theory is what the
piano roll now teaches; the Rust is how that knowledge is stored as
data.

## Seven new chords

Step 3's exercise 1 suggested adding a chord or two. Step 6 adds seven,
the extended chords of jazz, soul and funk, which made 28 in all (two
more qualities arrived in later steps; `ChordQuality::ALL` has 30 today):

| Name | Semitones above the root | In C | Where you hear it |
|------|--------------------------|------|-------------------|
| 6/9 | 0, 4, 7, 9, 14 | C E G A D | a soft, open major chord; jazz and bossa nova endings |
| 7b9 | 0, 4, 7, 10, 13 | C E G Bb Db | a tense dominant, very common as V in minor keys |
| 7#9 | 0, 4, 7, 10, 15 | C E G Bb D# | the "Hendrix chord" of funk and rock |
| 11 | 0, 4, 7, 10, 14, 17 | C E G Bb D F | soul and gospel (often played without its third) |
| m11 | 0, 3, 7, 10, 14, 17 | C Eb G Bb D F | a warm minor chord, everywhere in neo-soul |
| 13 | 0, 4, 7, 10, 14, 21 | C E G Bb D A | the full dominant of jazz and funk |
| maj13 | 0, 4, 7, 11, 14, 21 | C E G B D A | a lush major chord |

The names count the notes of the scale from the root, past the octave:
the ninth is the second an octave up (14 = 12 + 2 semitones), the
eleventh the fourth (17 = 12 + 5), the thirteenth the sixth (21 = 12 + 9).
Writing 14 instead of 2 matters for stamping, because a stamped chord's
notes are built from these numbers: a 9 placed a whole tone above the
root would sound like a cluster, not like a ninth chord.

A thirteenth chord "should" contain the eleventh as well, seven notes in
all, but players leave it out because it clashes with the third, a
semitone below it. Ferrous Lineup does the same, and says so in a comment
next to the numbers. The chord *naming* goes one step further, as it has
since step 3: if a chord of four notes or more is only missing its fifth,
it is still named. So C, E, Bb, D, A is called C13.

Adding a chord was as small as step 3 promised: a variant in the enum, a
line in `ALL`, and one arm in each `match`. The compiler pointed at
every `match` that had to grow, and the test
`no_two_qualities_have_the_same_notes` checked that no new chord has the
same notes as an old one. A new test names a few real voicings:

```rust
#[test]
fn extended_chords() {
    assert_eq!(label(&[48, 64, 67, 69, 74]).unwrap(), "C6/9");
    assert_eq!(label(&[55, 59, 62, 65, 68]).unwrap(), "G7b9");
    // E, G#, B, D and G: the "Hendrix chord".
    assert_eq!(label(&[52, 56, 59, 62, 67]).unwrap(), "E7#9");
    assert_eq!(label(&[55, 59, 62, 65, 69, 72]).unwrap(), "G11");
    assert_eq!(label(&[45, 48, 52, 55, 59, 62]).unwrap(), "Am11");
    assert_eq!(label(&[48, 64, 67, 71, 74, 81]).unwrap(), "Cmaj13");
    // Thirteenth chords are often played without their fifth too.
    assert_eq!(label(&[48, 64, 70, 74, 81]).unwrap(), "C13");
}
```

Look closely at Am11 and Cmaj13. Both are A, B, C, D, E and G: the same
six pitch classes. Big chords overlap like that all the time, which is
why the analysis tries the bass note as the root first. With A in the
bass you hear A minor, with C in the bass C major, and the name follows.

## Inversions when stamping

[The tools chapter](tools.md#inversions) showed `tools::inversion`. The
piano roll uses it for stamped chords too. When the Draw menu stamps a
chord, a second menu appears next to it:

```rust
// A chord of n notes has n - 1 inversions.
let chord_size = self.stamp.notes_at(Note::C4, scale).len();
if chord_size > 1 {
    self.inversion = self.inversion.min(chord_size - 1);
    egui::ComboBox::from_id_salt("inversion")
        .selected_text(INVERSIONS[self.inversion])
        .show_ui(ui, |ui| {
            for (times, name) in INVERSIONS.iter().enumerate().take(chord_size) {
                ui.selectable_value(&mut self.inversion, times, *name);
            }
        });
}
```

- The menu is only drawn when the stamp has more than one note. For
  single notes, there is nothing to invert.
- `INVERSIONS` lists six names, from "Root position" to "5th inversion",
  because the biggest chords have six notes. `.take(chord_size)` offers
  only as many as the chosen chord has: a triad gets three.
- `self.inversion.min(chord_size - 1)` keeps the choice valid when you
  switch from a big chord to a smaller one: a 4th inversion becomes the
  last one a triad has.

Inverting moves the lowest note up, so inverting the chord you point at
would move the whole chord up, away from the pointer. Stamping C major
on C5 in its second inversion would give G5, C6, E6, all of it above
where you clicked. So the piano roll moves the chord back down an
octave, and the note you clicked stays the root:

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

`notes.contains(&pitch)` is true in root position (nothing moved), and
also when an inversion could not move the root (at the top of the MIDI
range). Otherwise every note moves down an octave. This is the same
"collect `Option`s" pattern as the tools, written with a **turbofish**
(`collect::<Option<Vec<Note>>>()`) this time. In the tools, the return
type told `collect` what to build. Here a method, `unwrap_or`, is called
on the result straight away, and Rust must know the result's type before
it can look that method up, so the type is written out. If any note would fall off the bottom of the range,
`unwrap_or(notes)` keeps the chord as it was.

The outline that follows the pointer is drawn with the same
`stamp_notes`, so what you see is always what a click places.

## What comes next: functional harmony

Some chords sound like they want to go somewhere. Play G7 and stop: you
feel that the song is not over, and C major is where it wants to land.
Music theory calls this **functional harmony**: each chord of a key has a
job.

- The **tonic** (I, and in a way vi and iii) is home. A song usually
  starts and ends there.
- The **subdominant** (IV, and ii) moves away from home.
- The **dominant** (V, and vii°) is tension that wants to go home.
  Its third, B in G major, is one semitone below C: the **leading tone**,
  which pulls up to the root of the key.

Most chord progressions are made of a few strong moves between these
jobs:

| Move | Example in C | Why it works |
|------|--------------|--------------|
| V to I | G to C | the strongest ending; the leading tone rises to the root |
| IV to V | F to G | a step up, from moving away to wanting home |
| ii to V | Dm to G | the root falls a fifth; ii, V, I is everywhere, from jazz to pop |
| IV to I | F to C | the soft "amen" (plagal) cadence |
| V to vi | G to Am | the "deceptive" cadence: you expect C and get its relative minor |
| vi to IV | Am to F | as in I, V, vi, IV, the progression of countless songs |

A root that **falls a fifth** (G to C, D to G, A to D...) is the
strongest move there is, which is why so many suggestions are of that
kind.

### Knowledge as data

`next_chords(scale, current)` gives the chords that often follow
`current`, most usual first, each with the reason. The knowledge is not
in code; it is in a table. Each chord is found by its root (a number of
semitones above the tonic) and its triad family, and each entry lists the
moves that often follow:

```rust
use ChordFamily::{Diminished, Major, Minor};

struct Move {
    /// The root of the next chord, in semitones above the tonic.
    to: u8,
    short: &'static str,
    reason: &'static str,
}

struct Slot {
    root: u8,
    family: ChordFamily,
    moves: &'static [Move],
}

const fn go(to: u8, short: &'static str, reason: &'static str) -> Move {
    Move { to, short, reason }
}

const fn slot(root: u8, family: ChordFamily, moves: &'static [Move]) -> Slot {
    Slot { root, family, moves }
}

/// The chords of a major key, and the ones it borrows from minor.
const MAJOR: &[Slot] = &[
    slot(
        0,
        Major,
        &[
            go(5, "away from home",
               "I to IV: moving away from home, toward the dominant."),
            go(7, "tension, wants to go home",
               "I to V: tension that wants to come back to I."),
            go(9, "relative minor, darker",
               "I to vi: the relative minor, a darker colour."),
            go(2, "leads on to V",
               "I to ii: a minor chord that leads on to V."),
        ],
    ),
    // ...ii, iii, IV, V, vi, vii°, and the borrowed iv, ii°, bIII, bVI, bVII
];
```

Read a slot from the inside out: `go` builds a move with its reason in a
sentence and in a few words; `slot` builds a chord's entry; `MAJOR` is a
slice of slots, so it can hold a different number of entries for each
kind of key and for borrowed chords. All of it is `const`: it is built
into the program, and lives as long as the program does.

A suggestion carries both kinds of reason:

```rust
use std::borrow::Cow;

pub struct Suggestion {
    /// The degree of the key the chord is built on, from 0 (the tonic).
    pub degree: usize,
    pub chord: Chord,
    pub reason: Cow<'static, str>,
    pub short: Cow<'static, str>,
}
```

`short` is the reason in a few words ("root falls a fifth"), for a small
hint line; `reason` is the full sentence. They are `Cow<'static, str>`,
"clone on write": usually `Cow::Borrowed`, pointing straight at the
table's `&'static str`, with no allocation. But the reason can be
*rewritten* at run time: step 9 lets the chord sheet write minor-key
Roman numerals one way or another, and then a sentence such as "I to IV"
has to become owned text. `Cow` holds either case.

**Changed in step 8.** `Suggestion` also gained a `why(level)` method:
`HintLevel::Detailed` returns the sentence, `Short` the few words, and
`Minimal` nothing. The piano roll's hint setting chooses. The table and
the idea are the ones above.

Keeping knowledge as data has a nice consequence: changing what the
piano roll suggests means editing a table, not logic, and the tests
check the table, not the code. One of them goes through every degree of
three kinds of keys and checks that each has suggestions, and that no
chord suggests itself.

### Minor keys borrow a major V

In A natural minor, the chord on the fifth degree is E minor: E, G, B.
It has no leading tone (G is a whole tone below A), so it does not pull
home very hard. Centuries ago, musicians started raising its third to
G#, making it E major, and most music in minor keys still does. That
raised note is what the *harmonic minor* scale is made of.

So in all three minor scales, `next_chords` uses the chords of the
natural minor scale, with two harmonic-minor exceptions. The table of
roots and qualities makes both plain:

```rust
/// Natural minor, with a major V and the leading-tone vii°.
const MINOR_CHORDS: [(u8, ChordQuality); 8] = [
    (0, ChordQuality::Minor),        // i
    (2, ChordQuality::Diminished),   // ii°
    (3, ChordQuality::Major),        // bIII
    (5, ChordQuality::Minor),        // iv
    (7, ChordQuality::Major),        // V, not v
    (8, ChordQuality::Major),        // bVI
    (10, ChordQuality::Major),       // bVII
    (11, ChordQuality::Diminished),  // vii°
];
```

The roots are semitones above the tonic, so the lookup works in every
minor key without naming the notes. The fifth entry gives `V` major, with
the leading tone; the last gives the leading-tone chord. `MINOR` itself
also has an entry for a *minor* v (E minor in A minor), because a song
may use the natural-minor chord; its moves are the same "home or bVI"
moves as the major V's.

The "Chords in A minor" buttons show the key's own chords, so in A
natural minor the button for the fifth degree says "v · Em". When E major is suggested,
that button lights up, and its hover text says "played as E". This is
also why a suggestion carries its own `chord`: the degree is the key's,
but the quality can be the harmonic-minor one.

### Secondary dominants

What about a chord that is *not* in the key? Play C, then D major, in C
major. D major has an F#, which C major does not. It sounds like it is
going somewhere, and it is: D major is the V of G. For a moment, the
music behaves as if G were home, and D pulls to G the way G pulls to C.
This is called a **secondary dominant**, written V/V ("five of five"),
and it is one of the most common ways to add colour to a progression.

`next_chords` checks for it before looking at the table:

```rust
// A major or dominant chord from outside the key, a fifth above one
// of the key's chords, is most often its dominant: D major in C
// major leads to G.
let root = scale.semitones_up_to(current.root);
let dominant_kind =
    current.quality == ChordQuality::Major || current.quality.is_dominant();
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

The `if` is a **let chain**: plain conditions and `let` patterns joined
with `&&`, each one able to use what the ones before it bound. Read it
as the rule it is: the chord sounds like a dominant, it is not one of the
key's own chords, the note a fifth below it *is* in the key, that note is
not the tonic, and the chord built there is not diminished.

- "a fifth below" is `(root + 5) % 12`: a fifth down and a fourth up are
  the same pitch class, and `% 12` wraps around the octave.
- `is_chord_of_key` is more than "its notes are in the scale": it also
  accepts the minor key's major V, which is the key's own dominant even
  though the natural-minor scale has no G#.
- `target != 0`: a dominant of the tonic is just V. In A minor,
  E major is not the key's natural-minor chord, but it *is* the minor
  key's own V, so it goes to the table, which sends it home to A minor.
- `!= Diminished`: a diminished chord is too unstable to feel like home,
  even for a moment, so it is never "tonicized" this way.
- `suggest` returns an `Option<Suggestion>`; the `let Some` also refuses
  a target whose root is not a degree of the scale.

A test checks the classic cases in C major:

```rust
// D major is V of G; A7 is V of Dm; E is V of Am; C7 is V of F.
for (chord, target) in [
    (major(D, ChordQuality::Major), "G"),
    (major(A, ChordQuality::Dominant7), "Dm"),
    (major(E, ChordQuality::Major), "Am"),
    (major(C, ChordQuality::Dominant7), "F"),
] {
    let suggestions = next_chords(c_major, chord);
    assert_eq!(names(&suggestions), [target], "{chord:?}");
    assert_eq!(suggestions[0].reason, SECONDARY_DOMINANT);
}
```

C7 is a nice one: C is the tonic, but C7 has a Bb, so it is not the
tonic chord any more. It is the V of F, and sounds it.

The table already knows the common chords borrowed from the parallel
minor key: iv, ii°, bIII, bVI and bVII. Bb major in C major is bVII, and
it suggests C (the rock cadence) or F (the "double plagal"). What it does
not know is the **Neapolitan** bII, the major chord on the lowered second
degree (Db in C). Exercise 4 adds it.

### In the piano roll

The piano roll asks for suggestions about one chord, chosen by
`current_chord`:

```rust
fn current_chord(&self, drawing: Drawing) -> Option<(ChordAt, Chord, PitchClass)> {
    let held_chord = match analyze(drawing.held.iter().copied()) {
        Harmony::Chord { chord, bass } => Some((ChordAt::Keys, chord, bass)),
        _ => None,
    };
    held_chord.or_else(|| {
        let step = self.last_step.or(drawing.playhead)?;
        match analyze(sounding(drawing.pattern, drawing.heard, step)) {
            Harmony::Chord { chord, bass } => Some((ChordAt::Step(step), chord, bass)),
            _ => None,
        }
    })
}
```

A chord held on a keyboard comes first: it is what you are thinking
about right now. Otherwise the chord at the last step pointed at is used,
or the step being played when the pointer has not been over the grid.
`last_step` is remembered on purpose when the pointer leaves the grid;
otherwise the suggestions would vanish as soon as you moved the mouse up
to the buttons to read them. During playback the suggestions follow the
music.

The call is `next_chords_over(scale, chord, bass)`, not `next_chords`,
because the bass matters: a tonic chord with its fifth in the bass
(C/G in C major) is heard as a cadential six-four and leads to V. For
everything else it simply passes the work to `next_chords`.

Each suggested chord's button is drawn with `Button::selected(true)`,
the same highlighted look as a chosen tab, and its reason goes first in
its hover text. The grey line under the buttons lists all the
suggestions in order, so they can be read at a glance: "After C (I) at
step 5, try: F (IV), G (V), Am (vi), Dm (ii)."
