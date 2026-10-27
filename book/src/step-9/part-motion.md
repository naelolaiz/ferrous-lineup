# How each part moves from chord to chord

Files: [`crates/lineup-core/src/harmony/voice_motion.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_motion.rs)
and [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

The [voice-leading check](voice-leading.md) marks what classical part
writing avoids. That is the teacher's red pen. It says nothing when the
parts move well, and good part writing is most of what there is to
learn: each part moving as little as it can, the outer parts going
opposite ways, the leading tone rising and the seventh falling. Now the
piano roll shows every move, good or not. Tick **Voice leading**, point
at a chord change, and each part is named with where it goes.

![The sixteen bars of the chord functions example in the piano roll, four parts on one Piano track. Each part's notes are joined by a line of its own colour, and the moves at bar 5, where G7 goes to C, are bold. The line above the chord names reads "Piano: top D5→C5 · 2nd F4→E4 · 3rd B3→G3 · lowest G2→C3; outer parts in contrary motion".](../images/step-9-part-motion.png)

The example is the sixteen bars of
[Borrowed chords, secondary dominants and pivot chords](chord-functions.md),
written as four parts on one Piano track: a bass in the left hand and
three parts in the right. The check finds nothing to mark: "Voice
leading, 4 parts: nothing classical part writing avoids."

## What you see

- **A line of colour for each part**, from each of its notes to the
  next, over the notes. The eye can follow one part through the whole
  pattern, as on a score. A held note has no line: the part stays
  where it is.
- **The moves at the change you point at**, drawn bolder. Pointing
  anywhere in a chord picks the change that led into it.
- **A line of moves** above the chord names. It names each part from
  the top down, with where it goes: "top D5→C5 · 2nd F4→E4 · 3rd
  B3→G3 · lowest G2→C3". On one track the parts are named by place.
  With several tracks they are named by track. Then it names the
  motion between the outer parts.
- **Its tooltip**, which says each move in words. Move the pointer from
  the notes straight up to the line: the line keeps the change you
  pointed at last while you read it.

![The tooltip of the line of moves at the change from G7 to C: "Piano (top voice): down a major second from D5 to C5, a step. Piano (voice 2 of 4): down a minor second from F4 to E4, a step. Piano (voice 3 of 4): down a major third from B3 to G3, a skip. Piano (lowest voice): up a perfect fourth from G2 to C3, a leap." Then the motion between each pair of parts next to each other, and between the outer parts: contrary motion.](../images/step-9-part-motion-tooltip.png)

Where the check finds something at the step you point at, the line says
that instead, in red, as on the [voice-leading](voice-leading.md) page.

## Steps, skips and leaps

How far a part moves has a name:

| Move | Interval | Semitones | Example |
|---|---|---|---|
| **Held** | the same note | 0 | G3 to G3 |
| **Step** | a second | 1 or 2 | F4 to E4 |
| **Skip** | a third | 3 or 4 | B3 to G3 |
| **Leap** | a fourth or more | 5 or more | G2 to C3 |

The inner parts of a chorale move mostly by step or hold their notes.
The bass leaps more, because it moves from root to root: G2 up a fourth
to C3 at a cadence is one of the commonest bass moves there is.

The tooltip spells the interval in the key. B3 to G3 is a major third,
not "four semitones", and F♯ to G is a minor second, not "one
semitone". A note struck again shows as held on the line. The tooltip
says "strikes C5 again", so you can tell it from a note held over.

## Motion between two parts

Two parts that both move go together in one of four ways:

| Motion | The two parts | Example |
|---|---|---|
| **Parallel** | move the same way and keep the same interval | thirds going to thirds |
| **Similar** | move the same way, to another interval | a sixth going to a fifth, both down |
| **Contrary** | move opposite ways | the bass up, the top down |
| **Oblique** | one moves, the other holds | a held note under a moving one |

Part writing likes **contrary** and **oblique** motion between the
outer parts, the top line and the bass, because the ear follows those
two most. Moving opposite ways keeps them two lines. Parallel thirds
and sixths are fine anywhere, and they sound sweet. Parallel fifths and
octaves are what the voice-leading check marks, because they make two
parts sound like one.

"The same interval" means the same interval **number** as the key
spells it, not the same number of semitones. In the picture's G7 to C,
the top part goes from D5 to C5 and the second from F4 to E4. D over F
is a major sixth and C over E is a minor sixth: a sixth both times, so
that is parallel motion, parallel sixths. A harmony teacher counts it
that way too.

## What the example shows

At the change from G7 to C (bar 5):

- The **seventh** of G7, F4, falls a step to E4.
- The **leading tone**, B3, is in an inner part, and it falls a third
  to G3 instead of rising to C. An inner leading tone may do this in V7
  to I: then the C chord gets its fifth, G, and has all three notes.
  The voice-leading check leaves an inner leading tone alone for this
  reason.
- The **bass** leaps up a fourth from root to root, and the **top**
  steps down: the outer parts are in contrary motion.

Point at other changes and compare. From Ab to G7 (bar 4), the alto's
E♭ goes up to F instead of down to D. Down, it would make parallel
fifths with the bass, A♭ to E♭ and then G to D. From Db/F to G (bar
12), the top part falls from D♭5 to B4, the diminished third that the
Neapolitan sixth is known for.

## In the code

### Following the parts through every change

The voice-leading check already splits each track into parts
(`Voice`s). `voice_motion::changes` walks through the moments where a
new note starts in any part. For each part it takes the note it had
just before and the note it has there:

```rust
fn before(voice: &Voice, tick: usize) -> Option<NoteEvent> {
    let index = voice
        .notes
        .partition_point(|note| note.start_tick() < tick)
        .checked_sub(1)?;
    let note = voice.notes[index];
    (tick < note.end_tick() + LINE_BREAK).then_some(note)
}
```

The notes of a part are sorted by start, so `partition_point` finds
with a binary search how many start before `tick`. The one before that
count is the part's last note. `LINE_BREAK` is a quarter note: after a
rest that long the line has ended, and the next note starts a new one.
`bool::then_some` turns the condition into an `Option`, `Some(note)`
when it holds and `None` when it does not.

### Naming the motion

```rust
pub fn motion(upper: &Move, lower: &Move, key: Scale) -> Motion {
    let (a, b) = (upper.semitones(), lower.semitones());
    match (a.signum(), b.signum()) {
        (0, 0) => Motion::Static,
        (0, _) | (_, 0) => Motion::Oblique,
        (x, y) if x != y => Motion::Contrary,
        _ => {
            let number = |upper: Note, lower: Note| {
                let (upper, lower) = (
                    key.spell(upper.pitch_class()),
                    key.spell(lower.pitch_class()),
                );
                lower.letter.steps_up_to(upper.letter)
            };
            if number(upper.from.pitch, lower.from.pitch) == number(upper.to.pitch, lower.to.pitch)
            {
                Motion::Parallel
            } else {
                Motion::Similar
            }
        }
    }
}
```

`signum` is −1, 0 or 1: down, held or up. Matching on the pair of signs
sorts out the easy cases first. Both held. One held (the `|` in a
pattern means "or"). Opposite signs. Only when both go the same way
does the interval matter. Then the closure `number` spells both notes
in the key and counts letters from the lower to the upper: D over F is
`F.steps_up_to(D)`, 5 letters up, a sixth. Counting letters, not
semitones, is what makes a major sixth going to a minor sixth parallel.

### Keeping the line while you read it

The line shows the change at the step under the pointer. But to read
its tooltip, the pointer has to leave the notes for the line, and then
no step is under it. So the piano roll remembers the last step pointed
at, and the line uses it while the pointer is on the line itself:

```rust
let line = Rect::from_min_size(
    ui.cursor().min,
    vec2(ui.available_width(), ui.text_style_height(&TextStyle::Body)),
);
let step = self
    .hovered_step
    .or(drawing.playhead)
    .or(self.last_step.filter(|_| ui.rect_contains_pointer(line)));
```

`ui.cursor()` is where egui will put the next widget, so `line` is the
rectangle the line is about to take, worked out before it is drawn.
`Option::filter` keeps `last_step` only when the closure says yes: when
the pointer is inside that rectangle.

## Exercises

1. Write C to F to G7 to C in four parts, with the bass C3 F2 G2 C3.
   Keep every inner part moving by step or holding. Check the line of
   moves at each change: are the outer parts ever in parallel or
   similar motion?
2. In the example, move the alto's F4 in bar 4 to D4, so that G7 is
   G2 B3 D4 D5. What does the line of moves say at bar 4, and what does
   the voice-leading check find?
3. `Move::size` calls one or two semitones a step. An augmented second,
   A♭ to B, is three semitones, but its name says "second". Which size
   does the code give it, and which would a harmony teacher give it?
   Change `size` to count letters, as `motion` does, and write a test.
