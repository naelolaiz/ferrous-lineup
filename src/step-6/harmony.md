# More chords, inversions, and what comes next

Files: [`crates/lineup-core/src/harmony/chord.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord.rs),
[`crates/lineup-core/src/harmony/progression.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/progression.rs)
and [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

This chapter is half music theory and half Rust. The theory is what the
piano roll now teaches; the Rust is how that knowledge is stored as
data.

## Seven new chords

Step 3's exercise 1 suggested adding a chord or two. Step 6 adds seven,
the extended chords of jazz, soul and funk, for 28 in all:

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
in code; it is in a table, one entry per degree of the key:

```rust
/// For each degree of a major key, the degrees that often come next.
const MAJOR: [&[(usize, &str)]; 7] = [
    // I
    &[
        (3, "I to IV: moving away from home, toward the dominant."),
        (4, "I to V: tension that wants to come back to I."),
        (5, "I to vi: the relative minor, the same notes with a darker colour."),
        (1, "I to ii: a minor chord that leads on to V."),
    ],
    // ii
    &[
        (4, "ii to V: the root falls a fifth. ii, V, I is everywhere, from jazz to pop."),
        (6, "ii to vii°: another chord that leads home to I."),
    ],
    // ...
];
```

Read the type from the outside in: an array of 7 items (one per degree),
each a `&[(usize, &str)]`, a slice of pairs of "the degree that can come
next" and "why". An array needs all its items to have the same type, and
each degree has a different number of suggestions; a slice can have any
length, so every item is a slice. All of it is `const`: it is built into
the program, and lives as long as the program does. The `&str`s are
really `&'static str`, so a `Suggestion` can carry its reason without
copying it:

```rust
pub struct Suggestion {
    /// The degree of the key the chord is built on, from 0 (the tonic).
    pub degree: usize,
    pub chord: Chord,
    pub reason: &'static str,
}
```

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
natural minor scale, except on the fifth degree, where it uses a major
chord:

```rust
let chord_on = |degree: usize| -> Option<Chord> {
    if !minor {
        return scale.triad(degree);
    }
    if degree == 4 {
        let root = scale.pitch_classes().nth(4)?;
        return Some(Chord::new(root, ChordQuality::Major));
    }
    Scale::new(scale.root, ScaleKind::NaturalMinor).triad(degree)
};
```

This closure has its own `?`: it returns `Option<Chord>`, so `?` inside
it leaves the closure, not `next_chords`.

The "Chords in A minor" buttons show the key's own chords, so in A
natural minor the button for the fifth degree says "v · Em". When E major is suggested,
that button lights up, and its hover text says "played as E".

### Secondary dominants

What about a chord that is *not* in the key? Play C, then D major, in C
major. D major has an F#, which C major does not. It sounds like it is
going somewhere, and it is: D major is the V of G. For a moment, the
music behaves as if G were home, and D pulls to G the way G pulls to C.
This is called a **secondary dominant**, written V/V ("five of five"),
and it is one of the most common ways to add colour to a progression.

`next_chords` checks for it before looking at the table:

```rust
// A major chord on a degree where the key has none (D major in C) is
// most often the dominant of the chord a fifth below it (G).
let target = current.root.transpose(5);
let dominant_kind = matches!(
    current.quality,
    ChordQuality::Major
        | ChordQuality::Dominant7
        | ChordQuality::Dominant9
        | ChordQuality::Dominant7Flat9
        | ChordQuality::Dominant7Sharp9
        | ChordQuality::Dominant11
        | ChordQuality::Dominant13
);
if dominant_kind
    && !scale.is_diatonic(current)
    && let Some(target_degree) = scale.degree_of(target)
    && target_degree != 0
    && let Some(chord) = chord_on(target_degree)
    && chord.quality != ChordQuality::Diminished
{
    return vec![Suggestion { degree: target_degree, chord, reason: SECONDARY_DOMINANT }];
}
```

The `if` is a **let chain**: plain conditions and `let` patterns joined
with `&&`, each one able to use what the ones before it bound. Read it
as the rule it is: the chord sounds like a dominant, it is not in the
key, the note a fifth below it *is* in the key, that note is not the
tonic, and the chord built there is not diminished.

- "a fifth below" is `transpose(5)`: a fifth down and a fourth up are the
  same pitch class, and `transpose` on a `PitchClass` wraps around.
- `target_degree != 0`: a dominant of the tonic is just V. In A minor,
  E major is not in the natural minor key, but it is the minor key's own
  V, so it goes to the table, which sends it home to A minor.
- `!= Diminished`: a diminished chord is too unstable to feel like home,
  even for a moment, so it is never "tonicized" this way.

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

Chords whose root is not in the key at all, like Bb major in C major
(borrowed from C minor, and common in rock), get no suggestions yet.
Exercise 4 is about them.

### In the piano roll

The piano roll asks for suggestions about one chord, the one at the last
step the pointer was over:

```rust
let current = self.last_step.or(playhead).and_then(|step| {
    match analyze(sounding(pattern, &heard, step)) {
        Harmony::Chord { chord, .. } => Some((step, chord)),
        _ => None,
    }
});
```

`last_step` is remembered on purpose when the pointer leaves the grid.
Otherwise the suggestions would vanish as soon as you moved the mouse up
to the buttons to read them. When nothing has been pointed at yet, the
step being played is used, so during playback the suggestions follow the
music.

Each suggested chord's button is drawn with `Button::selected(true)`,
the same highlighted look as a chosen tab, and its reason goes first in
its hover text. The grey line under the buttons lists all the
suggestions in order, so they can be read at a glance: "After C (I) at
step 5, try: F (IV), G (V), Am (vi), Dm (ii)."
