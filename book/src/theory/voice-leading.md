# Voice leading and part motion

Files: [`crates/lineup-core/src/harmony/voice_leading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_leading.rs),
[`crates/lineup-core/src/harmony/voice_motion.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_motion.rs)
and [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

The chord lane names each chord you write. That is half of harmony. The
other half is **voice leading**: how each part moves from its note in
one chord to its note in the next. A part is one line of notes, such as
a singer, an instrument, or one line in a piano's chords. Two pieces can
use the same chords and sound very different, smooth or clumsy, because
their parts move differently.

Tick **Voice leading** next to **Ghost notes** in the piano roll, and
it does two things a harmony teacher does. It marks where the parts
break the rules of classical part writing, the red pen; and it shows
how every part moves at every chord change, good or not, which is most
of what there is to learn. This chapter covers the theory, the rules
and their exceptions, and the code: dynamic programming to split a
track into parts, `Option` chains to find parallel fifths, and a cache
so the check runs only when something changed.

## The music, briefly

Classical harmony courses teach voice leading with four-part writing:
soprano, alto, tenor and bass, as in Bach's chorales. They give a few
rules that keep every part a line of its own. The check follows the
best known three: **no parallel fifths or octaves**, **the leading tone
rises**, **a seventh falls**. They are explained one by one in [The
rules](#the-rules) below. Two more ideas describe every move, good or
bad.

### Steps, skips and leaps

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

### Motion between two parts

Two parts go together in one of four ways:

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
octaves are what the check marks, because they make two parts sound
like one.

"The same interval" means the same interval **number** as the key
spells it, not the same number of semitones. From E4 over G3 to F4
over A3, the upper part goes up a minor second and the lower a major
second. E over G is a major sixth and F over A a minor sixth: a sixth
both times, so that is parallel motion, parallel sixths. A harmony
teacher counts it that way too.

## What you see

- **A Voices row** under the two rows of chord names. Each finding is a
  block over the chord the parts arrive on, named in a few letters:
  **5ths** and **8ves** for parallel fifths and octaves, **1sns** for
  unisons, **LT** for a leading tone that does not rise, **7th** for a
  seventh that does not fall.
- **Magenta lines on the grid** from the note a part leaves to the note
  it goes to, where a rule is broken, and **a dashed outline** where a
  leading tone or a seventh should have gone.
- **A line of colour for each part**, from each of its notes to the
  next, over the notes, so the eye can follow one part through the
  whole pattern, as on a score. A held note has no line: the part stays
  where it is. The moves at the change you point at are drawn bolder;
  pointing anywhere in a chord picks the change that led into it.
- **A line above the grid.** When you point at a change with nothing to
  mark, it names each part from the top down, with where it goes, then
  the motion between the outer parts: "Piano: top D5→C5 · 2nd F4→E4 ·
  3rd B3→G3 · lowest G2→C3; outer parts in contrary motion". On one
  track the parts are named by place; with several tracks, by track.
  Its tooltip says each move in words ("down a major second from D5 to
  C5, a step"), and the motion between each pair of parts next to each
  other. When you point at a step with a finding, the line says what
  happens there instead, in magenta, and its tooltip says why classical
  part writing avoids it. With nothing pointed at, it sums the check
  up: "Voice leading, 4 parts: parallel fifths (1), …".

The tooltip spells each interval in the key: B3 to G3 is a major third,
not "four semitones", and F♯ to G is a minor second. A note struck
again shows as held on the line, and the tooltip says "strikes C5
again", so you can tell it from a note held over.

## Which notes make a part

The check needs parts, but the piano roll has tracks. A track that plays
one note at a time is one part. A track that plays chords, like a piano,
holds several parts. The app splits it the way you would read a score:
the top note of each chord goes to the top part, the next note to the
next part, and each note joins the part whose last note was nearest.
The messages call them "Right hand (upper voice)" and "Right hand
(lower voice)", or "Piano (top voice)", "Piano (voice 2 of 4)" and
"Piano (lowest voice)".

With **Ghost notes** on, the other tracks are checked too. Drum tracks
are left out.

A rest of a quarter note or more ends a line, so the notes on either
side of it are not checked against each other. A pair of parts that
plays in octaves almost all the time, like double basses doubling the
cellos, is one line made louder, not two parts in parallel octaves: at
least four notes together, seven in eight of them an octave or a unison
apart. The check counts such a pair once, and the summary says "one
line doubled in octaves".

## The rules

### No parallel fifths or octaves

Take two parts a perfect fifth apart. If both move the same way into
another perfect fifth, those are **parallel fifths**. The same goes for
octaves and unisons. These intervals blend so well that, for that
moment, the two parts sound like one. The texture loses a line, so
classical part writing avoids them. A fifth plus an octave (a twelfth)
still counts as a fifth.

When the parts move in opposite directions, from a fifth to a twelfth,
they make **fifths by contrary motion**. Each part moves differently,
but the ear still hears two fifths in a row, so strict part writing
avoids those too. Octaves by contrary motion show up now and then
between the top part and the bass at a cadence, and the message says
so.

C major to D minor with every part rising a step is the textbook case:
the soprano G4 and the tenor C4 are a fifth apart, and go to A4 and D4,
a fifth apart again: "Parallel fifths: Soprano and Tenor go from G4
over C4 (a fifth) to A4 over D4 (a fifth), both up." The tenor and the
bass, C4 over C3 to D4 over D3, make parallel octaves.

A held or repeated note does not count. Both parts have to move.

### The leading tone rises

The **leading tone** is the note a semitone below the tonic: B in C
major, G♯ in A minor. When a dominant chord (V, V7, vii°, vii°7) goes to
the tonic, the leading tone pulls up to the tonic.

The rule is strict in the top part and the bass, where the ear follows
the line. An inner part is freer. In V7 to I, an inner leading tone
often drops to the fifth of the tonic chord, so that chord gets all
three notes. Textbooks call this a "frustrated" leading tone, and the
check leaves inner parts alone.

The check knows three places where the leading tone must rise:

- **V or vii° to I**, the cadence.
- **V to vi** (V to VI in minor), the **deceptive cadence**. The leading
  tone still rises, to the tonic, which is now the third of vi.
- **A secondary dominant** going to its chord: D7 to G in C major. D7
  borrows F♯ from outside the key, and F♯ is G's leading tone, so it
  rises to G. C7 going to F in C major is also a secondary dominant
  (V7/IV), but its leading tone E belongs to the key, and keeps the
  freedom it has in C.

If both chords have the leading tone, it may stay where it is. A leading
tone going down a scale (C, B, A) is a passing note, not one that has to
rise. And V followed by I⁶₄ is no arrival, because the parts move on
before the leading tone has anywhere to go.

When two parts both have the leading tone, the message also says that it
is doubled. Classical part writing never doubles the leading tone: both
copies cannot rise without making parallel octaves.

### A seventh falls

In a seventh chord such as G7 (G B D F), the seventh, F, is a
dissonance. It **resolves down by step** into the next chord. In G7 to
C, F falls to E. If the soprano's F5 rises to G5 instead, the check says
"Seventh not falling: Soprano has F5, the seventh of G7 (V7), and rises
to G5 in C (I) instead of falling a step to E5", and the dashed outline
marks E5.

The check reads a few chords the way harmony books do. F A C D going to
G is named F6 by its notes, but it works as Dm7 with F in the bass,
ii⁶₅ in C. Its seventh is C, which should fall to B. A diminished
seventh chord's root comes from where it goes: G♯ B D F going to A has
its root on G♯, and its seventh is F.

The textbook exceptions are not flagged:

- **The next chord has the note.** Then the seventh may stay where it
  is.
- **A suspension.** The seventh is held into the next chord and falls a
  little later.
- **Passing and neighbour notes.** A short note that comes and goes by
  step makes a seventh only in passing. So does a note added off the
  beat while the other parts hold theirs.
- **V⁴₃ to I⁶.** The seventh (F in G7 with D in the bass) may rise to G
  while the bass rises from D to E a tenth below it.
- **A passing chord.** In C, then Em7 over D, then C over E, the bass's
  D is a passing note between two forms of the same C chord.
- **Chromatic voice leading.** A seventh can rise a semitone into the
  next chord's own leading tone. In Am7 with G in the bass going to E7
  with G♯ in the bass, G rises to G♯. The line keeps rising, and the
  next chord resolves it.
- **The bass in another octave.** The bass may take its note an octave
  away: B2 to C2 is the leading tone going to the tonic.

### What the check does not judge

These are the rules of one style: the part writing of Bach through
Brahms, and of harmony exercises. Pop and jazz move whole chords in
parallel on purpose, and so did Debussy. Nobody expects a jazz seventh
to resolve. Read the check as a teacher of the classical style, not a
judge of all music. Where the key is a twelve-tone row, the check
follows the rules of twelve-tone writing instead (see [The rules of
twelve-tone
writing](twelve-tone.md#the-rules-of-twelve-tone-writing)),
and where it is a pitch-class set, nothing is checked:

```rust
pub fn of(key: Scale) -> Rules {
    match key.kind {
        ScaleKind::Row(_) => Rules::TwelveTone,
        ScaleKind::Set(_) => Rules::Neither,
        _ => Rules::Classical,
    }
}
```

Some rules from harmony books are not checked: direct (hidden) fifths
and octaves in the outer parts, spacing between the parts, parts
crossing, and which note of a chord to double.

## In the code

### Sharing out the notes of a chord: dynamic programming

`voices_of` turns a track's notes into parts. It walks through the
chords in time, cutting the notes where their start changes with
`chunk_by`. For each chord, `share_out` decides which part each note
goes to. Notes and parts are both sorted from highest to lowest, and a
note may only go to a part below the one the note above it took, so
the parts never cross. Among all the ways to do that, it picks the one
with the smallest total distance in semitones.

Trying every way would take too long for a big chord. **Dynamic
programming** solves it with a table instead. `cost[i][j]` is the
smallest cost of placing the first `i` notes using only the first `j`
parts:

```rust
for i in 1..=notes {
    for j in 1..=lines {
        let skip_part = cost[i][j - 1];
        let new_part = cost[i - 1][j] + NEW_PART;
        let take = cost[i - 1][j - 1] + distance(i - 1, j - 1);
        cost[i][j] = skip_part.min(new_part).min(take);
    }
}
```

Each cell has three choices. Skip part `j`, so it gets no note in this
chord. Start a new part for note `i`, at the large cost `NEW_PART`
(10,000, more than any distance), so it happens only when a chord has
more notes than there are parts. Or put note `i` on part `j`, at the
cost of the distance between them. Each cell is built from cells
already filled in, so the whole table takes `notes × parts` steps.
Walking back from the last cell recovers which choice made each cost,
and so where each note went. The [twelve-tone
check](twelve-tone.md#the-best-reading-dynamic-programming)
uses the same idea to find which forms of a row are played.

### Comparing two parts note against note

`together` lists every moment either part starts a note, with the note
each part is on at that moment:

```rust
onsets
    .into_iter()
    .map(|tick| a.current(tick).zip(b.current(tick)))
    .collect()
```

`Option::zip` turns two `Option`s into an `Option` of a pair. It is
`Some` only when both are `Some`, that is, when both parts are playing.
`consecutive` then looks at each pair of neighbouring moments, and
`kind_of_move` decides what the move makes:

```rust
fn kind_of_move(x: Hop, y: Hop) -> Option<SlipKind> {
    let moves = |(from, to): Hop| {
        let rested = to.start_tick() >= from.end_tick() + LINE_BREAK;
        (from.pitch != to.pitch && !rested).then(|| to.pitch > from.pitch)
    };
    let (up_x, up_y) = (moves(x)?, moves(y)?);
    let first = perfect(x.0.pitch.midi().abs_diff(y.0.pitch.midi()))?;
    let second = perfect(x.1.pitch.midi().abs_diff(y.1.pitch.midi()))?;
    let same_way = up_x == up_y;
    match (first, second) {
        (Perfect::Fifth, Perfect::Fifth) if same_way => Some(SlipKind::ParallelFifths),
        (Perfect::Fifth, Perfect::Fifth) => Some(SlipKind::ContraryFifths),
        (Perfect::Unison, Perfect::Unison) => Some(SlipKind::ParallelUnisons),
        (Perfect::Fifth, _) | (_, Perfect::Fifth) => None,
        _ if same_way => Some(SlipKind::ParallelOctaves),
        _ => Some(SlipKind::ContraryOctaves),
    }
}
```

`moves` gives `Some(true)` for a part that goes up, `Some(false)` for
one that goes down, and `None` for a held note or a rest of a quarter
note or more: `bool::then` runs its closure only when the condition
holds. Each `?` returns `None` from the function as soon as something
is missing, and `perfect` gives `None` for an interval that is not a
unison, fifth or octave. What is left after the `?`s is the interesting
case, two perfect intervals in a row, and a `match` on the pair names
it. The arms are tried in order, so the fifth-to-something-else arm
must come before the two octave arms that catch everything left.

### Naming a part with a closure

A finding knows its parts by number. Only the app knows the tracks'
names, so `describe` takes a function that names a part:

```rust
pub fn describe(&self, voice_name: impl Fn(usize) -> String) -> String {
```

The piano roll passes a closure:

```rust
let name = |voice: usize| voice_name(drawing.project, check, voice);
// ...
let why: Vec<String> = here.iter().map(|slip| slip.describe(name)).collect();
```

`name` is passed by value, once for each finding, and yet it can still
be used afterwards. That works because the closure only holds shared
references (`&Project`, `&VoiceLeading`). A closure whose captures are
all `Copy` is itself `Copy`, so each call gets its own copy. (Clippy
says so too: it flags `describe(&name)` as a needless borrow.)

### Checking only when something changed

Checking a pattern takes under a millisecond in a release build, but
the piano roll draws 60 frames a second while a song plays. So it keeps
the last check with what it was made from:

```rust
#[derive(Debug, Clone, PartialEq)]
struct VoiceInput {
    pattern: usize,
    /// The notes of each channel checked.
    notes: Vec<(usize, Vec<NoteEvent>)>,
    time: TimeSignature,
    /// The key on each beat.
    keys: Vec<Scale>,
}
```

Each frame builds a new `VoiceInput` and compares it with the stored
one. `#[derive(PartialEq)]` compares field by field, down to every
note. Comparing is much cheaper than checking, and when nothing changed
the old check is used:

```rust
if let Some((made_from, check)) = &self.voice_check
    && *made_from == input
{
    return Rc::clone(check);
}
```

`if let … && …` is a **let chain**, from the 2024 edition. It tests a
pattern and a condition in one `if`.

The check is kept in an `Rc<VoiceLeading>`, a shared pointer. Drawing
needs the check while it also changes the piano roll (`&mut self`).
Borrowing the check from `self` would lock `self` for the whole frame,
so the borrow checker would refuse it. `Rc::clone` only adds one to a
count, so the frame gets a handle of its own for the cost of an integer
increment, with no copy of the findings.

### Following the parts through every change

`voice_motion::changes` walks through the moments where a new note
starts in any part. For each part it takes the note it had just before
and the note it has there:

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
count is the part's last note; `checked_sub(1)?` gives up when there is
none. After a rest of `LINE_BREAK`, a quarter note, the line has ended,
and the next note starts a new one. `bool::then_some` turns the
condition into an `Option`: `Some(note)` when it holds and `None` when
it does not.

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
in the key and counts letters from the lower to the upper: E over G is
`G.steps_up_to(E)`, 5 letters up, a sixth. Counting letters, not
semitones, is what makes a major sixth going to a minor sixth parallel
(see [Intervals count letters
too](pitch.md#intervals-count-letters-too)).

### Keeping the line while you read it

The line above the grid shows the change at the step under the
pointer. But to read its tooltip, the pointer has to leave the notes for
the line, and then no step is under it. So the piano roll remembers the
last step pointed at, and the line uses it while the pointer is on the
line itself:

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
the pointer is inside that rectangle. While a song plays, the playhead
picks the step when the pointer is elsewhere.

## Testing it

You learn harmony from this check, so a wrong mark would teach you
something wrong. Two kinds of test guard it:

- **Textbook examples.** The tests in `voice_leading.rs` write small
  four-part examples, about twenty of them, each part as a line of note
  names. Each test writes a rule and its exceptions as notes and checks
  what is found: a textbook cadence with nothing to find, parallel
  fifths by a twelfth, a held note, a rest that ends a line, a doubled
  line, the leading tone in an outer and an inner part, the deceptive
  cadence, a secondary dominant, ii⁶₅, V⁴₃ to I⁶, a suspension, a
  passing chord, a note added off the beat, chromatic voice leading,
  the bass in another octave, a six-four chord, and a track of chords
  split into parts.
- **Bach's chorales.** The music21 library, used by music scholars, has
  its own check for parallel fifths and octaves. Both were run on 350
  Bach chorales, and they found the same 129 parallels, at the same
  places between the same parts. For the leading tone and the seventh,
  every one of the findings left in those chorales is a real departure
  from strict part writing, such as a soprano leaping away from the
  leading tone at a cadence. Bach broke the rules now and then, on
  purpose and with good reason.

```rust
#[test]
fn a_textbook_cadence_has_nothing_to_avoid() {
    // I IV V7 I in C major, as harmony books write it.
    let check = satb(["C5 C5 B4 C5", "G4 A4 G4 G4", "E4 F4 F4 E4", "C3 F3 G3 C3"]);
    assert_eq!(kinds(&check), vec![]);
    assert_eq!(
        check.summary(),
        "Voice leading, 4 parts: nothing classical part writing avoids."
    );
}
```

`voice_motion.rs` follows a short chorale on one track, C major (C3
G3 E4 C5) to F major (F2 A3 F4 C5) to G7 (G2 B3 F4 D5), and checks
the line of moves at the first change, "S C5 held · A E4→F4 · T G3→A3
· B C3→F2; outer parts in oblique motion"; the sizes held, step, step
and leap; the words ("B: down a perfect fifth from C3 to F2, a leap");
the alto and tenor in parallel motion, parallel sixths, and the tenor
and bass in contrary motion. At the second change the soprano and the
bass both rise a step, a fifth and two octaves apart: parallel motion,
and parallel fifths. Other tests check a melody over a held note
(oblique), and that a fifth to a twelfth, or a major sixth to a minor
sixth, is still parallel.

Exercises on voice leading are in [Exercises: music theory in
code](exercises.md).
