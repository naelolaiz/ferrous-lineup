# Hearing chords: over time, two at once, and as sets

Files: [`crates/lineup-core/src/harmony/hearing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/hearing.rs),
[`crates/lineup-core/src/harmony/analysis.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/analysis.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-app/src/harmony_keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/harmony_keys.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)
and [`crates/lineup-core/tests/chord_reading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/chord_reading.rs)

[The concert hall](concert-hall.md) brought the Rite of Spring and
Petrushka into the app, but the chords it showed for them were mostly
wrong. Opening **The Augurs of Spring** wrote a chord sheet with 25
empty bars, and where it did write a chord it was something like Fsus2
or C♯dim. The Augurs chord, which the strings hammer for pages, had no
name at all. Out of 64 bars checked against the scores, none came out
right.

The cause was not a bug in one place. It was the way the app listened.
This page explains how it listens now, and what that taught about
harmony and about making Rust code fast.

## Why one moment is not enough

Until now every harmony view called `analyze`
([Chord names](chord-names.md)) on the notes **held at one moment**.
That is right for a chord held down on a keyboard. Music that moves
is harder to name that way:

- An **arpeggio** plays its chord one note at a time: C, then E, then
  G. At any one moment only one note sounds, and one note names no
  chord.
- A **tune** passes through notes outside the chord. Under a C major
  chord, a melody on D makes the moment C, D, E, G, which reads as
  Cadd9 even though the harmony is still plain C.
- A **walking bass** steps between the chords: C, B♭, A on the way
  from C to A7. The B♭ under C, E and G turns the chord into C7 for a
  moment, though no one in the band is playing C7.
- An orchestra can play **two chords at once**, one over the other.
  This is a *polychord*. In the Augurs of Spring the low strings play
  F♭ major (F♭, A♭, C♭) while E♭7 (E♭, G, B♭, D♭) sounds above it. In
  Petrushka the clarinets play C major over F♯ major. No single chord
  symbol holds either one.
- Some harmonies are **not stacks of thirds** at all. Petrushka's fair
  opens on the flute's D, E, G and A, which stack up in fourths: E, A,
  D, G. Calling that a chord forces a name onto it.

So the new module, `harmony/hearing.rs`, listens to a **stretch** (a
beat, half a bar or a bar) and weighs every note by how long it sounds
there.

## Weighing the notes

A note heard in a stretch is a `HeardNote`. Its times are counted in
steps from the start of the stretch. A note that began earlier has a
start below 0, and one that goes on past the end has an end beyond
the length:

```rust
pub struct HeardNote {
    pub pitch: Note,
    pub start: f32,
    pub end: f32,
    /// Which part plays it, such as a channel.
    pub part: usize,
}
```

Notes can lie wholly outside the stretch, too. Those weigh nothing,
but they tell what each part is doing around it, as the next section
shows.

Adding up how long each of the 12 pitch classes sounds gives a
**profile**, twelve numbers. A C major arpeggio over a bar is C, E and
G each sounding for a third of it. The profile is the same as a
held chord's, so the arpeggio is heard as its chord.

## What each part does

How much a note weighs also depends on what its part is doing. The
module tells four roles apart, looking at the bar before and the bar
after as well:

| Role | How it is told | What its notes weigh |
|------|----------------|----------------------|
| Chords | three different notes or more sound together in it | in full |
| Bass line | one note at a time, all below the chords | in full if the chords have the note; 0.15 if not (a passing note) |
| Tune | any other part, when some part plays chords | a quarter |
| Layer | a part that would be a tune, but whose notes all belong to one triad or seventh chord: three notes at least, two of them not in the chords | in full |

The **layer** row is there for the Rite of Spring. Its ostinato D♭,
B♭, E♭, B♭ is one line, but it spells E♭7 against the C major and E
minor arpeggios below it. It is a harmony of its own, not a tune.

One more case: a part that plays chords but moves them in parallel
with a tune, every voice the same way by a step or a third, as in the
last pages of Boléro. Those chords colour the tune. The harmony is
what goes on under them, so such a part counts as a tune. Telling
this case apart needs the bars around the stretch: two chords are not
enough to show parallel motion, but a whole phrase is.

When no part plays chords, as in an orchestra where every instrument
plays one line, every line counts in full.

## A score for every chord

Every chord, 12 roots times every quality the app knows, gets a score
for how well it explains the profile:

```rust
let explained = PitchSet::from_bits(chord.pitch_set().bits() | also.bits());
let inside: f32 = explained.iter().map(|p| profile.of(p)).sum();
let outside = profile.total - inside;
// ...
let fit = (inside - OUTSIDE * outside) / profile.total - missing - SIZE * extra;
```

- `inside` is how much of the sound the chord explains. `outside` is
  what it leaves out, which counts one and a half times as much
  (`OUTSIDE = 1.5`).
- `missing` is a small loss for each note of the chord that is hardly
  heard. A missing fifth costs less than a missing third.
- `extra` counts the notes beyond a triad. A thirteenth chord holds
  nearly every note, so without this cost it would explain anything.
  It wins only when its notes really sound.
- A bonus of 0.06 goes to a chord whose root is in the bass.

`PitchSet` stores a set of pitch classes as the bits of one `u16`: bit
0 for C, bit 1 for C♯, and so on. `|` on the bits is the union of two
sets, so `explained` is "the chord's notes and the notes of `also`" in
a single instruction.

### A walking bass, worked through

Rhythm changes starts with a bar of C that walks down to A7. The piano
plays C, E and G on beats 1 and 2, and the bass plays C, then B♭. Over
those two beats the profile is C 9, E 5, G 5, and the passing B♭
4 × 0.15 = 0.6.

- **C** explains everything but the B♭: (19 − 1.5 × 0.6) / 19.6 = 0.92,
  plus 0.06 for C in the bass: **0.98**.
- **C7** explains everything, but its B♭ is hardly heard, which costs
  0.12, and its fourth note costs 0.06. With the bass bonus it scores
  1 − 0.12 − 0.06 + 0.06 = **0.88**.

C wins, as the chart says. An earlier version weighed passing bass
notes at 0.35, and there C7 won by a nose. A test that writes the
chord sheet of Rhythm changes caught it.

## Two chords at once

If no single chord fits well (a score under 0.7), the module tries
two. It splits the notes into a lower and an upper layer, in every
way that makes sense:

- **by register**: everything below a pitch is the lower layer.
  A split is skipped when it would cut through a line, since a line
  that runs from A up to G is an arpeggio of A7, not the top of
  G major;
- **by instrument**: the instruments sorted from the lowest sounding
  to the highest, and cut between any two of them.

Each layer must make a triad or a seventh chord of its own, with three
of its notes heard and its root among them. Then the pair must pass
four checks:

1. The roots differ, and the upper chord adds at least two notes of
   its own. C over A minor is just Am7.
2. The two chords do not make **one key** between them. F major over
   G major uses only the notes of C major: that is G eleventh, a single
   harmony in C, not two keys at once.
3. Both chords **sound together** through at least 80% of the stretch.
   C major and then G7 is a progression, not a polychord. The ear holds
   the notes of a staccato chord struck again and again, so gaps of two
   steps or less count as sounding.
4. The pair, minus a cost of 0.2 for needing two chords, scores better
   than the best single chord.

```rust
fn in_one_key(notes: PitchSet) -> bool {
    let major = PitchSet::from_semitones(&[0, 2, 4, 5, 7, 9, 11]);
    (0..12).any(|tonic| notes.is_subset_of(major.transpose(tonic)))
}
```

`transpose` rotates the bits, so this tries all 12 major scales, and
their modes with them. Why only major scales? Because the Augurs chord
itself fits a **harmonic minor** scale. Its seven notes are F♭, G, A♭,
B♭, C♭, D♭ and E♭, exactly A♭ harmonic minor. Had the check counted
harmonic minor as "one key", the chord this page is about would have
been rejected.

The Petrushka chord passes the same way. C major and F♯ major lie a
tritone apart and share no note. Between them they hold both E and F♯,
and both G and F♯, which no single major scale has. Both triads come
from the **octatonic** scale, C, D♭, E♭, E, F♯, G, A, B♭, which
alternates half and whole steps.

## A set when no chord fits

Last, the notes are scored as a **pitch-class set**
([Twelve-tone rows and pitch-class sets](../step-8/twelve-tone.md)). A set
explains every note heard enough to count, so it always fits, but it
pays 0.35 for giving up on a chord name. A stack of fourths pays only
0.1, because since Debussy it has been a harmony of its own, as plain
to the ear as a triad. There are three such sets: 3-9 (C, F, B♭, the
same notes as a sus chord), 4-23 (C, F, B♭, E♭) and 5-35 (C, F, B♭, E♭,
A♭, the pentatonic scale).

The flute's D, E, G and A at the start of Petrushka's fair is 4-23,
so it is named as that set, in place of a forced chord such as
"D6sus2".

## The best reading, and the ones close behind

A `Hearing` keeps every reading that scores within 0.12 of the best,
three at most:

```rust
pub struct Hearing {
    readings: Vec<Reading>,
    notes: PitchSet,
    profile: [f32; 12],
}
```

`best()` is the first reading, `alternatives()` the rest. The app
shows them the way it shows other cues: the most likely one brightest,
the others fainter.

- On the **harmony keys**, each chord of a polychord lights its key,
  where the key has one, and a chord the music could also be heard as
  lights its key faintly. Next to the hint, a line says it in words:
  "Heard: Eb7|Fb, two chords at once", or "Heard: the notes 4-23".
- In the **piano roll**, the All row of the chord lane is now heard beat
  by beat with the whole band, and the line above the grid adds the
  other readings in brackets: "(or C6)".

## Cutting a bar into chords

**Write the chords the song plays** used to walk the song beat by beat
and write a chord wherever the name changed. Now `write_heard_chords`
takes the song a bar at a time and asks: where should this bar be cut?
A bar of 4/4 has beat lines at 0, 1, 2, 3 and 4, and any two of them
make a stretch: ten stretches, from single beats to the whole bar. The
best way to cut the bar is found by **dynamic programming**:

```rust
// best[i]: the best score for the beats before line i, where its
// last stretch starts, and how that stretch is heard.
for i in 1..lines.len() {
    let mut here = (f32::NEG_INFINITY, 0, None);
    for j in 0..i {
        let hearing = heard(lines[j], lines[i]);
        let cut = match lines[j] {
            line if line == bar.start => 0.0,
            line if line == half => CUT_HALF_BAR,
            _ => CUT_BEAT,
        };
        let total = best[j].0 + worth(&hearing) * (i - j) as f32 - cut;
        if total > here.0 {
            here = (total, j, Some(hearing));
        }
    }
    best.push(here);
}
```

The best way to cut the first `i` beats is the best way to cut the
first `j`, plus one stretch from `j` to `i`, for the best `j`. Each
stretch earns its score once per beat it covers, and each cut costs a
little: 0.15 halfway through the bar, 0.25 on any other beat. So a bar
is split only where two chords really fit better than one. Walking
back from the end through the `j`s gives the cuts.

A polychord **goes on** while its notes do. In Petrushka's Room the F♯
major half of the chord rests for a bar while C major goes on. Heard
alone, that bar is just C. But C major and F♯ major still explain
nearly all of it, so the sheet keeps "C|F♯" there instead of jumping to
C and back.

## Writing two chords down: Eb7|Fb

A lead sheet draws a polychord as a fraction, the upper chord over a
line over the lower one. The chord sheet writes it on one line as
`upper|lower`: `Eb7|Fb` or `C|F#`. A slash already means a bass note,
as in `G/B`, so it could not be used here.

```rust
if let Some((upper, lower)) = text.split_once('|') {
    let upper = ChordSymbol::parse(upper)?;
    let lower = ChordSymbol::parse(lower)?;
    return Ok(HarmonyName::Polychord(upper, lower));
}
```

`split_once` splits a string at the first `|` and returns both
halves, or `None` when there is no `|`. The `?` after each `parse`
returns the error at once if either half is not a chord. The new
variant `HarmonyName::Polychord` holds two chord symbols. Each one can
have a bass of its own, and the lower chord's bass is the bass of the
whole harmony.

A project whose sheet has a polychord is saved as **format 25**. An
older version of the app could not read `Eb7|Fb`, so it is told the
file is newer instead of losing the bar.

The band plays both chords. `polychord_voicing` puts the lower chord
from the G below middle C upward and stacks the upper chord right above
it, so the two layers never cross, the way an orchestra keeps them
apart.

## Making it fast

The first working version was correct and about **60 times slower**
than the old way. Writing the chord sheet for all 306 demos took
3.6 seconds instead of 0.06, and the test suite, which writes chord
sheets for every demo and style many times over, crawled. Four changes
brought it back down.

**1. Measure first.** `valgrind --tool=callgrind` runs a program on a
simulated CPU and counts every instruction, per function and per line.
Build with debug information in release mode, then annotate:

```sh
CARGO_PROFILE_RELEASE_DEBUG=true cargo test --release -p lineup-core --no-run
valgrind --tool=callgrind ./target/release/deps/<test binary> --ignored one
callgrind_annotate --inclusive=yes callgrind.out.<pid>
```

80% of the time went to one closure. For **every note**, it asked
again whether the note's part was a tune, and that question walked
**every note** of the stretch. That is n² work. Asking once per part
and looking the answer up for each note made it n.

**2. Do shared work once.** A bar is heard as ten stretches, but which
parts move in parallel depends only on the bar and the bars around it.
A small struct now remembers that:

```rust
pub struct Listening<'a> {
    notes: &'a [HeardNote],
    /// The parts that move their chords along with a tune.
    planing: Vec<usize>,
}

impl<'a> Listening<'a> {
    pub fn new(notes: &'a [HeardNote]) -> Self { /* works out `planing` */ }
    pub fn hear(&self, start: f32, end: f32) -> Hearing { /* one stretch */ }
}
```

The `'a` is a **lifetime**. `Listening` borrows the notes instead of
copying them, and `'a` tells the compiler that a `Listening` must not
outlive the slice it borrows. The piano roll uses the same idea: it
gathers each bar's notes once and hears its beats from them, where it
used to gather the notes again for every beat.

**3. Keep only what is used.** Only the three best chords become
readings, so there is no need to sort all of them. A small vector stays
sorted as chords arrive:

```rust
let at = found.partition_point(|&(_, other): &(Chord, f32)| other >= fit);
if at < MOST_READINGS {
    found.insert(at, (chord, fit));
    found.truncate(MOST_READINGS);
}
```

`partition_point` does a binary search for the first place where the
condition turns false: the first chord that fits worse than the new
one. A chord that fits exactly as well goes after the ones already
there, so the result is the same as a stable sort.

**4. Don't hear the same bar twice.** Songs repeat. A bar played again
exactly as before, between the same bars, is heard the same way, so
`write_heard_chords` keeps a `HashMap` from each bar's beat lines and
notes to how it was cut. The notes go into the key as whole numbers,
`(start, end, pitch, part)`, since `f32` cannot be a key: it has no
`Hash` and no `Eq`, because `NaN` is not equal to itself.

Writing every demo's sheet now takes about 0.85 seconds. The slowest
test that writes chord sheets takes as long as it did before the
change.

## How well it hears

The test file `tests/chord_reading.rs` checks bars whose harmony is
known from the scores: the Augurs chord in 52 bars, Petrushka's chord
in bars 47 to 53 of Petrushka's Room, and the fourths at the start of
the fair. It also has a report, which compares the sheet heard from the
notes with the chart of every demo that has one, beat by beat:

```sh
cargo test --release -p lineup-core --test chord_reading -- --ignored --nocapture
```

| | Before | Now |
|---|---|---|
| Exactly the chart's chord | 74.9% | 87.9% |
| A compatible chord (same notes, or same root with notes added or left out) | 77.7% | 94.0% |
| The right root | 77.2% | 94.5% |
| Bars checked against the Rite and Petrushka scores | 0 of 64 | 64 of 64 |

The report also lists every demo where fewer than 70% of the beats
match its chart: the place to look next.

## Try it

1. Open **File > Demos > Songs > Classical and film > The Rite of
   Spring, The Augurs of Spring**. A song without a chart opens with the
   chords it plays written on its sheet: open **📝 Chords** and read
   `Eb7|Fb` bar after bar. Hover over a bar to read what it is: "Eb
   dominant seventh over Fb major, two chords at once: a polychord.
   Upper: Eb G Bb Db. Lower: Fb Ab Cb. In Eb Mixolydian: I7 over
   bII."
2. Press Play with the harmony keys showing. The key of I lights for
   E♭7 (F♭ major is not a chord of E♭ Mixolydian, so it has no key),
   and the line next to the hint names both chords.
3. Open **Petrushka, Petrushka's Room** and find bar 47, where the
   clarinets enter with the Petrushka chord. The sheet writes it
   `C|F#/A#`: C major over F♯ major, with A♯ in the bass.
4. Open **Petrushka, The Shrovetide Fair** and look at the first bars:
   the sheet writes the set as `4-23 T2`, 4-23 transposed up two
   semitones, and hovering says "stacked fourths".
5. In a new song, type `C|F#` in a bar of the chord sheet and choose a
   band style. The band plays both chords, one above the other.

## Exercises

1. Play E, G♯ and B with the left hand and E♭, G, B♭ and D♭ with the
   right: the Augurs chord. Now play the A♭ harmonic minor scale, A♭,
   B♭, C♭, D♭, E♭, F♭, G. Check that every note of the chord is in it.
   Why did `in_one_key` have to leave harmonic minor out?
2. In the walking bass example, work out the scores of C and C7 again
   with the passing note weighing 0.35 instead of 0.15 (the B♭ is then
   1.4 and the total 20.4). Which chord wins?
3. The cut costs are 0.15 halfway through the bar and 0.25 on other
   beats. A bar of 4/4 plays C, then G7, and each of them, heard alone,
   scores 1.06 (a perfect fit with its root in the bass). Suppose the
   whole bar, heard as one stretch, scores 1.0 per beat. Does the sheet
   cut the bar when G7 comes on beat 4, for one beat? And when it comes
   on beat 3, for two?
4. Why does `HeardBars` use the notes of the bars around a bar in its
   key, and not only the bar's own notes?
