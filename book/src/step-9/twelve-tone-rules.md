# Twelve-tone rules: checking music written with a row

Files: [`crates/lineup-core/src/harmony/voice_leading/serial.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_leading/serial.rs),
[`crates/lineup-core/src/harmony/voice_leading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_leading.rs)
and [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

The [voice-leading check](voice-leading.md) marks what classical part
writing avoids: parallel fifths, a leading tone that doesn't rise, a
seventh that doesn't fall. Every one of those rules is about a key.
A leading tone leads to the tonic, and a seventh falls into the next
chord of the key. In music written with a
[twelve-tone row](../step-8/twelve-tone.md) there is no tonic, so these
rules mean nothing there.

Twelve-tone music has rules of its own. Schoenberg wrote by them,
and so did his students Berg and Webern. Composition courses still teach
them. So when the key is a **twelve-tone row**, the **Voice leading**
tick box now checks those rules instead.

![Schoenberg's row from his Suite, Op. 25, as the key (P4, on E). A melody and a keyboard part share two forms of the row, P4 and P10, named on the Row line. The Voices line marks a note out of the row's order, a chord out of order that is also an A minor triad, and a D doubled in octaves. The notes are outlined, and dashed outlines show the notes the row wanted.](../images/step-9-twelve-tone-rules.png)

## What you see

- **A Row line** under the chord names. It names each form of the row
  that the check finds in the notes, such as **P4** or **P10**, over the
  notes that play it. Point at it to read the form's notes: "P4: the
  row starting on E: E F G C# F# Eb Ab D B C A Bb."
- **The Voices line** under it marks what twelve-tone writing avoids:
  **order** for a note out of the row's order, **rep** for a note
  repeated too early, **gap** for a form left unfinished, **8ve** for a
  note doubled in octaves, **tonal** for a chord that sounds like tonal
  harmony.
- **Outlines** around the notes concerned. Two doubled notes are joined
  by a line, and so are the notes of a tonal chord. A dashed outline
  shows the note the row wanted instead.
- **The summary line**, just above the chord rows, sums it all up: "Twelve-tone rules,
  P4, P10, through all the tracks together: out of row order (2), octave
  doubling (1), sounds tonal (1)." Point at a step with a finding and it
  says what happens there. Hover over it to read why.

![Pointing at bar 3, beat 4: the keyboard plays E4 where P10 wants D, and C4 E4 A4 make an A minor triad.](../images/step-9-twelve-tone-rules-tooltip.png)

With a **pitch-class set** as the key, nothing is checked. A set is a
collection of notes with no order and no rules of its own, so there is
nothing to check against, and the line under the chords says so.

## The rules

### The row sets the order

The row takes the place of the key. A piece moves through forms of the
row: P, I, R and RI, each on any of the twelve notes. Each form is
played note after note, in its order, so that everything you hear comes
from the row.

Notes next to each other in the row may sound together as a chord,
in any order from the bottom up. In the picture, the keyboard's first
chord is notes 4 to 6 of P4 (C#, F# and Eb), in the middle of the
melody's E F G and Ab. A note may also be repeated straight away, or
trilled with the note before it. That is still the same note of the row.

Two mistakes look alike but are counted differently. If you **swap** two
notes, or **leave one out**, that is one note out of place, not every
note after it. In the picture, the melody plays B before D. The check
marks the B ("Melody plays B4 where P4 wants D, its note 8") and finds
the rest of the row in order again.

### No note comes back early

Before a note comes back, the other eleven sound. The row was made for
that: no note is heard more often than the others, so none of them
becomes a tonic. A note played again straight away is fine. A note that
comes back after other notes, before the form has sounded all twelve, is
marked **rep**.

### A form is played whole

When a form stops short and the next one starts, the notes it left out
are missing from the twelve. The check marks the last note of such a
form with **gap** and names what never sounded: "Row left unfinished: P4
stops after 8 of its 12 notes, with Melody's D5; B, C, A and Bb never
sound before the next form starts."

### No octave doublings

Doubling a note in octaves gives it weight. As Schoenberg explained, a
note with weight can be heard as a root, or even as a tonic. The check
marks two parts that play the same note together, and also a part that
plays a note while another part still holds it. A part that doubles
another all the way through, a line in octaves, is told once, where it
starts.

### No chords of tonal harmony

A major or minor triad, or a dominant seventh, reminds the ear of tonal
music, and the ear then expects the music to go on as tonal music would.
Schoenberg and Webern avoided them. The check marks a moment where
everything sounding together makes one of these chords, and names it,
spelled as that chord. **Berg** used them on purpose. The row of his
Violin Concerto (1935) is built from triads: G minor, D major, A minor,
E major. So read **tonal** as "this sounds like a key", not "this is
wrong".

## Reading the row as an analyst would

Nothing in the notes says which form of the row is playing, or where one
form ends and the next begins. A student analysing a score has to find
that out, and so does the check.

It first cuts the notes into **groups**: notes that start together. Then
it tries every way of sharing the groups out among forms of the row, and
keeps the way with the fewest problems. It counts each note out of
order, each note repeated early, and each note a form leaves out.
That sounds like too many ways to try, but dynamic programming makes it
quick (see [In the code](#in-the-code)).

A few things make the reading closer to what composers actually do:

- **Forms can overlap at the join.** A form may begin on the note, or
  the chord, that the form before ended on, so the two share it. This
  is called an **elision**. In the picture, P4 ends with the chord A Bb
  C, and P10 starts on Bb. P10 takes the Bb from that chord and goes on
  with B.
- **A chord may share notes with the chord before.** Those notes count
  for the new form if it needs them, or as repeated notes if it doesn't.
- **One row or one per track.** All the tracks can share one form, as
  a tune and the chords under it do in the picture. Or each track can
  play forms of its own, as in a canon. The check reads the row both
  ways and keeps the reading with fewer problems. The summary line says
  which one it kept.
- **Patterns join up.** Each pattern is checked on its own, but a row
  may run on from one pattern into the next. So the first form may have
  begun in the pattern before, as long as its notes here are in order.
  The Row line then says "…P4", and its tooltip says at which note the
  pattern starts. The last form may go on in the next pattern, so it is
  never marked unfinished.
- **Ties.** When two readings have equally few problems, the check
  prefers forms that start on their first note. If two forms still tie,
  it names the first in the order P, I, R, RI. Chords can make two forms
  impossible to tell apart. Webern's row for his Concerto, Op. 24, is
  built from four groups of three notes, all related to each other, and
  its forms R0 and I1 have the same three notes in each group. In the
  Library's Webern's trichords style, the check reads the melody's
  second form as R0, and the chords under it as I1. Either name is
  right.

## What the check doesn't do

The check follows the classic rules of Schoenberg's twelve-tone method.
It doesn't judge:

- **free atonal music**, written with no row at all (choose a
  pitch-class set as the key for that, and nothing is checked);
- **serialism beyond pitch**: whether durations, dynamics or registers
  follow a series too, as in the music of Boulez and Babbitt;
- **combinatoriality** and other ways of pairing forms. In the
  [Hexachord pads](../step-8/twelve-tone.md#twelve-tone-styles-and-demos)
  style, the chords hold half of P0 while the melody plays the other six
  notes from I3. The check reads each track's own form there, which is
  right, but it doesn't tell you that the two halves make all twelve
  notes together.

## How we know it is right

- **Textbook cases.** The tests in `serial.rs` write each rule and its
  exceptions as notes, using Schoenberg's row from the Suite. They cover
  a row in order, chords from neighbouring notes, two notes swapped, a
  note left out, repeats and trills, an elision, a form left unfinished,
  a form carried over from the pattern before, a tune and chords sharing
  a form, a canon, octave doublings, tonal chords, and the classical
  rules switched off in rows and sets.
- **The Library's own row styles.** The Library writes seven
  [twelve-tone styles](../step-8/twelve-tone.md#twelve-tone-styles-and-demos)
  from their rows, so the check must read each one as the forms it was
  built from. A test writes all seven, on C and on F#, and asserts that
  no note is out of order. The other findings are real. Triad
  chorale's triads sound tonal, as its description promises. In
  Twelve-tone lo-fi, the bass doubles the chords' first note at the
  start. In Webern's trichords, the melody doubles the chords under it
  all the way through. That is the Library's arrangement: the style says
  its rhythm is Ferrous Lineup's, not Webern's.

## In the code

### A form's places in sixteen bits

`Walk` goes through one form of the row, group by group, and remembers
which of the twelve places have sounded. That is twelve yes-or-no
answers, so it keeps them as the bits of a `u16`, one bit per place:

```rust
struct Walk {
    /// One bit for each place in the form, from the first.
    used: u16,
    /// The place after the furthest one sounded.
    next: usize,
    errors: usize,
}

fn has(self, place: usize) -> bool {
    self.used & (1 << place) != 0
}
```

`Walk` is `Copy`, two machine words and a bit set, so trying a form
costs nothing to set up. Building a set of places is a `fold` that ORs
in one bit per place, and the integer methods answer the questions the
check asks:

```rust
let fresh: u16 = group
    .notes
    .iter()
    .map(place_of)
    .filter(|&place| !self.has(place))
    .fold(0, |bits, place| bits | 1 << place);
// How many places have sounded:
self.used.count_ones() as usize
// The place after the highest one in `fresh`:
16 - fresh.leading_zeros() as usize
```

`count_ones` and `leading_zeros` are single processor instructions on
most machines. The [pitch-class sets](../step-8/twelve-tone.md#sets-in-bits)
use the same idea, with twelve bits for the twelve notes.

### When are new notes in order?

A group of *n* new notes is in order when they fill the next *n* places
of the form, in any order among themselves. "The next places" can mean
two things, and `window` tries both:

```rust
fn window(self, taken: u16, count: usize) -> u16 {
    let first_open = self.open(0..12, count);
    let ahead = self.open(self.next..12, count);
    match (taken & ahead).count_ones() > (taken & first_open).count_ones() {
        true => ahead,
        false => first_open,
    }
}
```

`first_open` is the first places still open, which catches up a note
skipped just before. `ahead` is the places after the furthest note
heard, which goes on after a skip.

Take P4 with D and B swapped: E F G C# F# Eb Ab **B D** C A Bb. At B,
the form wants D, so B is out of place. At D, the first open place is
D's own, so D is in order, and so is everything after it.

Now take P4 with D left out: E F G C# F# Eb Ab **B** C A Bb. B is out of
place again. At C, the first open place is still D's, which never comes.
But the place after the furthest note heard, B, is C's own, so C is in
order, and so are A and Bb. Only B is marked. With `first_open` alone,
every note after the missing D would be out of place.

### One walk, two jobs

The same `take` counts the problems while the readings are compared, and
reports them for the reading that wins. It does both by taking a
closure, `tell`, which it calls for each note out of place:

```rust
fn take(&mut self, form: &Form, group: &Group, mut tell: impl FnMut(Misplaced))
```

While comparing, the closure does nothing: `walk.take(form, group,
|_| {})`. When reporting, it turns each note into a finding:
`walk.take(form, group, |misplaced| slips.push(misplaced.slip(form,
key)))`. That second closure changes `slips`, a vector it borrows, so it
is an `FnMut`, not just an `Fn`. Because `take` is generic over the
closure (`impl FnMut`), the compiler builds a copy of `take` for each
one. The empty one costs nothing at all.

### The best reading: dynamic programming

`read` has to choose where each form starts and ends, and which form it
is. `best[end]` is the fewest problems in the first `end` groups, and
`came_from[end]` is the last stretch on the way there:

```rust
for start in 0..n {
    for (index, form) in forms.iter().enumerate() {
        let mut walk = Walk::from(0);
        for end in start + 1..=reach {
            walk.take(form, &groups[end - 1], |_| {});
            let missing = if end == n { 0 } else { 12 - walk.heard() };
            let cost = best[start] + walk.errors + missing;
            if cost < best[end] {
                best[end] = cost;
                came_from[end] = Stretch { start, end, form: index, from: 0 };
            }
        }
    }
}
```

Each stretch is tried once for each form. The walk carries on from one
`end` to the next instead of starting again, so a form tried on twenty
groups costs twenty steps, not two hundred. Walking `came_from` back from
the last group gives the forms in order. (The real loop also tries
elisions and forms carried over from the pattern before, through
`Walk::from`.) The same idea shares out a chord's notes among the parts
in the [classical check](voice-leading.md#sharing-out-the-notes-of-a-chord-dynamic-programming).

### Notes that start together: `chunk_by`

The groups come from the notes sorted by start, cut wherever the start
changes:

```rust
for chunk in played.chunk_by(|a, b| a.note.start_tick() == b.note.start_tick()) {
```

`chunk_by` gives slices of neighbours for which the closure says "same
group", without copying anything.

## Exercises

1. Choose **twelve-tone row** as the key, pick **Schoenberg's Suite** in
   **Row…** with E as the key's note, and write P4 (E F G C# F# Eb Ab D
   B C A Bb) as a melody. Tick Voice leading. Then swap two
   notes, then leave one out. How many notes does the check mark each
   time, and where does the dashed outline go?
2. Write the first three notes of P4 as a chord, then the rest as a
   melody. Then write notes 1, 2 and 4 as the chord. What changes?
3. Write P4 in one track and P4 an octave lower in another, in the same
   rhythm. What does the check say, and why does it say it only once?
4. Write a chord of C, E and G in a row key. Then try G, B, D and F.
   Which rule do they break, and why did Berg break it on purpose?
5. In `Walk::take`, a note that only repeats a note of the group before
   is never out of place. Find the line that makes it so. Then write a
   test in `serial.rs` where a chord shares a note with the chord before
   it, and check that nothing is marked.
