# Hearing harmony: chords and keys from notes

Files: [`crates/lineup-core/src/harmony/hearing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/hearing.rs),
[`crates/lineup-core/src/harmony/key_finding.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/key_finding.rs),
[`crates/lineup-core/src/library/listen.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/listen.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/app/key_finder.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/key_finder.rs)
and [`crates/lineup-core/tests/chord_reading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/chord_reading.rs)

A pattern stores notes, not chords, and an imported MIDI file or a
song you played in has no chords written anywhere. Yet many parts of
the app need the harmony: the chord sheet's **Write the chords the song
plays**, the **All** row of the piano roll's chord lane, the harmony key
that lights up as the song plays, the voice-leading check, a style that
plays a whole song again in another rhythm, and the key of a MIDI file
without key signatures. All of them listen to the notes the way a
musician does by ear. This chapter is about how: first chords, then
keys.

`analyze`, in [Chords: building, naming and
inversions](chords.md#naming-what-is-playing), names the
notes held at one moment, which is right for a chord held down on a
keyboard. Music that moves needs more.

## The music, briefly

Naming the notes of one moment goes wrong in five common ways:

- An **arpeggio** plays its chord one note at a time: C, then E, then
  G. At any moment only one note sounds, and one note names no chord.
- A **tune** passes through notes outside the chord. Under a C major
  chord, a melody on D makes the moment C, D, E, G, which reads as
  Cadd9 though the harmony is plain C.
- A **walking bass** steps between chords: C, B♭, A on the way from C
  to A7. The B♭ under C, E and G makes C7 for a moment, though no one
  plays C7.
- An orchestra can play **two chords at once**, one over the other: a
  **polychord**. In the Augurs of Spring, from Stravinsky's *Rite of
  Spring*, the low strings play F♭ major (F♭, A♭, C♭) while E♭7 (E♭, G,
  B♭, D♭) sounds above. In *Petrushka* the clarinets play C major over
  F♯ major. No single chord symbol holds either one.
- Some harmonies are **not stacks of thirds** at all. *Petrushka*'s fair
  opens on a flute figure on D, E, G and A, which stack up in fourths:
  E, A, D, G. Calling that a chord forces a name on it.

A **key** is heard the same way, over time. In C major, C, E and G come
back most, then the other white keys, and the black keys hardly at all.
A minor uses the same white keys, but A, C and E lead, and G♯ turns up
at every cadence. A piece **modulates** when it moves to a new key and
stays there; a single chord from another key, such as D7 leading to G
in a song in C major (a *secondary dominant*), only *tonicizes* G for a
moment and is no change of key.

## Listening to a stretch, not a moment

The chord hearing listens to a **stretch** of music, a beat, half a bar
or a bar, and weighs every note by how long it sounds there. A note
heard is a `HeardNote`, timed in steps from the start of the stretch:

```rust
pub struct HeardNote {
    pub pitch: Note,
    /// When it starts, in steps from the start of the stretch: below 0
    /// for a note that started before it.
    pub start: f32,
    /// When it ends, in steps from the start of the stretch: beyond its
    /// length for a note that goes on after it.
    pub end: f32,
    /// Which part plays it, such as a channel: an instrument's notes are
    /// a layer of their own when two chords sound at once.
    pub part: usize,
}
```

The times are `f32`, not whole steps, because a note can start between
steps (its nudge, see [The project: patterns, channels, tracks, clips
and markers](../program/project.md#notes-steps-and-patterns)). Notes
can also lie wholly outside the stretch. Those weigh nothing, but they
tell what each part is doing around it, as the next section shows.
`Project::heard_notes` gathers the notes of every channel that plays
notes (synths and MIDI Out channels that are not muted; drums and
samplers do not) between two steps of the song.

Adding up how long each of the twelve pitch classes sounds gives a
**profile**:

```rust
struct Profile {
    weight: [f32; 12],
    total: f32,
    /// How long the pitch class heard longest sounds.
    loudest: f32,
}
```

A C major arpeggio over a bar is C, E and G each sounding a third of
it: the same proportions as a held chord, so the arpeggio is heard as
its chord. That one idea fixes the first problem in the list. The
others need to know what each part is doing.

## What each part does

How much a note weighs depends on the part that plays it. `weights`
tells four roles apart:

| Role | How it is told | What its notes weigh |
|------|----------------|----------------------|
| Chords | three different notes or more sound together in it, or it plays chords one note at a time | in full |
| Bass line | one note at a time, all below the chords | in full if the chords have the note; 0.15 if not, a passing note |
| Tune | any other part, when some part plays chords | a quarter |
| Layer | a part that would be a tune, but whose notes are three or more of one triad or seventh chord, none of them in the chords | in full |

The end of `weights` applies the table, note by note:

```rust
let weight = if tune {
    w * TUNE
} else if lowest_chord.is_some()
    && below
    && !chord_notes.contains(note.pitch.pitch_class())
{
    w * PASSING_BASS
} else {
    w
};
```

`w` is how long the note sounds inside the stretch. When no part plays
chords, as in an orchestra where every instrument plays one line,
`lowest_chord` is `None`, nobody is a tune and every line counts in
full.

The **layer** row is there for the Rite of Spring. Its ostinato D♭, B♭,
E♭, B♭ is one line, but it spells E♭7 against the C major and E minor
arpeggios below it: a harmony of its own, not a tune.

Two more questions need the bars around the stretch, since a single
beat cannot answer them:

- Does a part **move its chords along with a tune**, every voice the
  same way by a step or a third, as in the last statements of Ravel's
  *Boléro*? Then its chords colour the tune and the harmony is what
  goes on under them, so the part counts as a tune. That is
  `moves_in_parallel`; two chords are not enough to show parallel
  motion, but a phrase is.
- Does a part **play chords one note at a time**, as an Alberti bass or
  the triplets of the *Moonlight* Sonata do? Short notes, nearly all
  leaps of a minor third or more, on three pitch classes at least:
  `breaks_chords`. Then the part plays chords as much as one striking
  them does.

## A score for every chord

Every chord, twelve roots times every quality the app knows, gets a
score for how well it explains the profile:

```rust
fn chord_fit_with(chord: Chord, profile: &Profile, also: PitchSet) -> (f32, bool) {
    if profile.total <= 0.0 {
        return (f32::NEG_INFINITY, false);
    }
    let explained = PitchSet::from_bits(chord.pitch_set().bits() | also.bits());
    let inside: f32 = explained.iter().map(|p| profile.of(p)).sum();
    let outside = profile.total - inside;
    let full = PRESENT * profile.loudest;
    let mut missing = 0.0;
    let mut core = true;
    for (&interval, &tone) in chord.quality.intervals().iter().zip(chord.quality.tones()) {
        let pitch_class = chord.root.transpose(i32::from(interval));
        let presence = (profile.of(pitch_class) / full).min(1.0);
        let loss = if tone == 5 && interval % 12 == 7 {
            MISSING_FIFTH
        } else {
            MISSING
        };
        missing += loss * (1.0 - presence);
        if matches!(tone, 1..=4) && presence < 0.5 {
            core = false;
        }
    }
    let extra = chord.quality.intervals().len().saturating_sub(3) as f32;
    let fit = (inside - OUTSIDE * outside) / profile.total - missing - SIZE * extra;
    (fit, core)
}
```

- `inside` is how much of the sound the chord explains, and `outside`
  what it leaves out, which counts one and a half times as much
  (`OUTSIDE = 1.5`): a wrong note is stronger evidence than a right one.
  Divided by the total, a chord that explains everything scores 1.
- `missing` is a small loss for each note of the chord that is hardly
  heard, 0.15 for a missing note and only 0.05 for a missing perfect
  fifth, which players leave out freely. A note heard at least 35% as
  long as the loudest (`PRESENT`) counts as fully there.
- `extra` counts the notes beyond a triad, 0.06 each. A thirteenth chord
  holds nearly every note of a scale, so without this cost it would
  explain anything; it wins only when its notes really sound.
- `core` says whether the root and the third (or the note that stands
  for it in a sus chord) are really heard. A chord without them is
  passed over.

`PitchSet` stores a set of pitch classes as the bits of one `u16` (see
[Pitch classes, notes and
spelling](pitch.md#a-set-of-pitch-classes-in-twelve-bits)).
`|` on the bits is the union of two sets, so `explained` is "the
chord's notes and those of `also`" in one instruction. `also` is empty
for a single chord; in a polychord it holds the other chord's notes.

The bass adds the last touch:

```rust
fn bass_fit(chord: Chord, bass: Option<PitchClass>) -> f32 {
    match bass.map(|bass| (bass, chord.tone_of(bass))) {
        Some((bass, _)) if bass == chord.root => BASS_ROOT,
        Some((_, Some(3 | 5 | 7))) | None => 0.0,
        Some(_) => -BASS_OUTSIDE,
    }
}
```

A root in the bass earns 0.06; a third, fifth or seventh is an
inversion and costs nothing; any other note costs 0.06. The middle arm
shows two pattern tools: `3 | 5 | 7` inside `Some(...)` matches any of
three values, and `| None` joins a second whole pattern to the arm.

### A walking bass, worked through

A piano holds C, E and G for two beats (eight steps) while the bass
walks C, then B♭, on its way down to A for the next chord. The bass
plays below the chords and B♭ is not one of their notes, so it weighs
4 × 0.15 = 0.6. The profile is C 12 (piano and bass), E 8, G 8 and
B♭ 0.6, 28.6 in all; `loudest` is capped at the stretch's length, 8,
so a note doubled in two parts does not make the others look faint.

- **C** explains everything but the B♭: (28 − 1.5 × 0.6) / 28.6 = 0.948,
  nothing missing, plus 0.06 for its root in the bass: **1.008**.
- **C7** explains everything, 1.0. But its B♭ is heard for 0.6 of the
  2.8 that counts as fully there, which costs 0.15 × (1 − 0.21) = 0.12,
  and its fourth note costs 0.06. With the bass bonus: **0.882**.

C wins, as a chart would say, and C7 is not even kept as a second
reading (that needs to be within 0.12 of the best). Had the passing
note weighed 0.35, C would still win, by 0.941 to 0.925, but C7 would
show as a close alternative.

### Keeping the best three

Only the three best chords become readings, so there is no need to
sort all of them. A short vector stays sorted as chords arrive:

```rust
let at = found.partition_point(|&(_, other): &(Chord, f32)| other >= fit);
if at < MOST_READINGS {
    found.insert(at, (chord, fit));
    found.truncate(MOST_READINGS);
}
```

`partition_point` does a binary search for the first place where the
condition turns false: the first chord that fits worse than the new
one. A chord that fits exactly as well goes after those already there,
so the result is the same as a stable sort, at a fraction of the work.

## Two chords at once

If no single chord fits well (a fit under 0.7, `ONE_CHORD`), the
hearing tries two. `best_polychord` splits the notes into a lower and an
upper layer in every way that makes sense:

- **by register**: everything below a pitch is the lower layer. A split
  that would cut through a line is skipped, since a line running from A
  up to G is an arpeggio of A7, not the top of G major;
- **by instrument**: the parts sorted from the lowest sounding to the
  highest, cut between any two of them.

Each layer must make a major or minor triad, or a major, minor or
dominant seventh chord, with three of its notes heard and its root among
them. Two bigger chords at once would explain any notes at all. Then the
pair must pass four checks:

1. The roots differ, and the upper chord adds at least two notes of its
   own. C over A minor is just Am7.
2. The two chords do not make **one key** between them. F major over G
   major uses only the notes of C major: that is G eleventh, one
   harmony in C, not two keys at once.
3. Both chords **sound together** through at least 80% of the time
   anything sounds in the stretch (`covered`). C major and then G7 is a progression, not a polychord.
   The ear holds the notes of an arpeggio, or of a staccato chord
   struck again and again, so a gap of two steps or less counts as
   sounding.
4. The pair, less 0.2 for needing two chords (`POLYCHORD`), scores
   better than the best single chord.

The second check is a few lines:

```rust
fn in_one_key(notes: PitchSet) -> bool {
    let major = PitchSet::from_semitones(&[0, 2, 4, 5, 7, 9, 11]);
    (0..12).any(|tonic| notes.is_subset_of(major.transpose(tonic)))
}
```

`transpose` rotates the bits, so this tries all twelve major scales, and
their modes with them. Why only major scales? Because the Augurs chord
itself fits a **harmonic minor** scale. Its seven notes, F♭, G, A♭, B♭,
C♭, D♭ and E♭, are exactly A♭ harmonic minor. Had the check counted
harmonic minor as one key, the chord would have been rejected.

The Petrushka chord passes the same way. C major and F♯ major lie a
tritone apart and share no note, and between them they hold six notes,
C, C♯, E, F♯, G and A♯, that no single major scale holds. Both triads
come from the **octatonic** scale, C, D♭, E♭, E, F♯, G, A, B♭, which
alternates half and whole steps.

## A set when no chord fits

Last, the notes are scored as a **pitch-class set** (see [Pitch-class
sets, and one harmony with four names](sets.md)). A set
explains every note heard enough to count, so it always fits, but it
pays 0.35 for giving up on a chord name. A stack of fourths pays only
0.1, because since Debussy it has been a harmony of its own, as plain to
the ear as a triad:

```rust
/// The sets that are stacks of fourths: C F Bb (3-9, the same notes as a
/// sus chord), C F Bb Eb (4-23) and C F Bb Eb Ab (5-35, the pentatonic
/// scale), as their prime forms.
const QUARTAL_SETS: [&[u8]; 3] = [&[0, 2, 7], &[0, 2, 5, 7], &[0, 2, 4, 7, 9]];
```

The flute's D, E, G and A at the start of *Petrushka*'s fair is 4-23,
so it is named as that set, instead of a forced chord such as D6sus2.

## The best reading, and the ones close behind

A `Hearing` keeps every reading within 0.12 of the best (`CLOSE`),
three at most:

```rust
pub struct Hearing {
    readings: Vec<Reading>,
    /// The pitch classes heard enough to be part of the harmony, without
    /// the passing notes.
    notes: PitchSet,
    /// How long each pitch class sounds, weighed.
    profile: [f32; 12],
}
```

`best()` is the first reading, or `Harmony::Silence` when there is none,
and `alternatives()` the rest:

```rust
pub fn alternatives(&self) -> &[Reading] {
    self.readings.get(1..).unwrap_or_default()
}
```

`get(1..)` returns `None` for an empty vector, where `&self.readings[1..]`
would panic, and `unwrap_or_default` turns that `None` into an empty
slice, the default of `&[Reading]`.

The fields are private, so a `Hearing` can only be made by hearing:
nothing outside the module can build one whose readings are out of
order. The app shows the readings the way it shows its other cues, the
most likely brightest and the others fainter:

- Next to the harmony keys' hint, a line says what is heard when it is
  not simply one chord: "Heard: Eb7|Fb, two chords at once", "Heard:
  the notes 4-23", or a chord followed by "or" and its close rivals.
  On the harmony keys, each chord of a polychord lights its key.
- The piano roll's status line, for the step under the pointer, adds
  the other readings in brackets: "(or C6)".

One last rule protects a tune alone. Für Elise opens with E, D♯, E,
D♯, E, B, D, C and nothing under them: a tune, not a chord, though
added up its notes would earn some chord name. So when a single line
plays alone and its best reading fits less than 0.85 (`ALONE`), the
hearing names no harmony at all, and the chord before it goes on.

## Cutting a bar into chords

**Write the chords the song plays** fills the chord sheet from the
notes. It also runs by itself when a band style arrives in a song whose
sheet is empty, since the band plays from the sheet. It takes the song
a bar at a time and asks: where should this bar be cut? A bar of 4/4
has beat lines at 0, 1, 2, 3 and 4, and any two of them make a stretch:
ten stretches, from single beats to the whole bar. The best way to cut
is found by **dynamic programming**:

```rust
// best[i]: the best score for the beats before line i, where its
// last stretch starts, and how that stretch is heard.
let mut best: Vec<(f32, usize, Option<Hearing>)> = vec![(0.0, 0, None)];
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

The idea: the best way to cut the first `i` beats ends with some last
stretch, from line `j` to line `i`. Whatever `j` is, the cuts before it
must be the best way to cut the first `j` beats, or there would be a
better answer. So `best[i]` is worked out from the `best[j]` before it,
which are already known, trying every `j`. A bar of four beats can be
cut in only 2³ = 8 ways, but the same stretch turns up in many of them,
and this way each of the ten stretches is heard once. The library's
listener, below, cuts a whole pattern the same way, where trying every
way would be hopeless: a pattern of 64 beats can be cut in 2⁶³ ways,
and the loop asks about 64 × 64 / 2 questions.

Each stretch earns its fit once per beat it covers (`worth`), and each
cut costs a little: 0.08 halfway through the bar, 0.12 on any other
beat. So a bar is split only where two chords really fit better than
one, and more easily in the middle than on a weak beat, as chords
change in real music. `f32::NEG_INFINITY` is smaller than every other
float, so the first real total always beats it, with no special case.
Walking back from the end through the `j`s gives the cuts, last first,
and `reverse` puts them in order.

`worth` gives silence a full score, and a single line a little less:

```rust
fn worth(hearing: &Hearing) -> f32 {
    match hearing.best() {
        Harmony::Silence => 1.0,
        Harmony::Single(_) | Harmony::Interval { .. } => 0.8,
        _ => hearing.fit(),
    }
}
```

A beat of rest, or of a melody alone, then joins the stretch around it
rather than being cut off, and writes nothing: the chord before goes on.

A polychord also **goes on** while its notes do. In *Petrushka*'s Room,
the F♯ major half of the chord rests for a bar while C major goes on.
Heard alone, that bar is C. But C major and F♯ major still explain
nearly all of it, so where nothing else fits well the sheet keeps C|F♯
instead of jumping to C and back (`Hearing::goes_on`).

### Hearing fast

Writing the sheet hears ten stretches of every bar of 4/4, and the
tests write the sheets of every demo, so speed matters. Three things keep it fast.

**Do shared work once.** Which parts move in parallel, and which play
broken chords, depends on the bar and the bars around it, not on the
stretch. A small struct works it out once and hears the stretches from
it:

```rust
pub struct Listening<'a> {
    notes: &'a [HeardNote],
    /// The parts that move their chords along with a tune.
    planing: Vec<usize>,
    /// The parts that play chords one note at a time.
    broken: Vec<usize>,
}
```

`'a` is a **lifetime**. `Listening` borrows the notes instead of copying
them, and `'a` tells the compiler that a `Listening` must not outlive
the slice it borrows. `hear(start, end)` then picks the notes of one
stretch and retimes them from its start. The piano roll does the same
for its All row: it gathers each bar's notes once and hears its beats
from them.

**Don't ask a question per note that could be asked per part.** Whether
a part is a tune depends on all of its notes. Asked again for every note
of the stretch, that is n² work; `weights` asks once per part, keeps the
answers in `roles`, and looks each note's part up.

**Don't hear the same bar twice.** Songs repeat. A bar played again
exactly as before, between the same bars, is heard the same way, so
`write_heard_chords` keeps a `HashMap` from each bar's beat lines and
notes to how it was cut:

```rust
type HeardBars = HashMap<(Vec<usize>, Vec<(i64, i64, u8, usize)>), Vec<(usize, Hearing)>>;
```

The notes go into the key as whole numbers, `(start, end, pitch,
part)`, since an `f32` cannot be a key: it has no `Hash` and no `Eq`,
because `NaN` is not equal to itself. `type` gives the long type a short
name; it is the same type, not a new one.

To find what is slow, measure. `valgrind --tool=callgrind` runs a
program on a simulated processor and counts every instruction, per
function and per line:

```sh
CARGO_PROFILE_RELEASE_DEBUG=true cargo test --release -p lineup-core --no-run
valgrind --tool=callgrind ./target/release/deps/<test binary> --ignored one
callgrind_annotate --inclusive=yes callgrind.out.<pid>
```

It points at the function, and the line, where the time goes; a
question asked per note that could be asked per part is the kind of
thing it finds.

### Writing it down

Each stretch becomes a chord marker where the harmony changes, and again
where a section starts, as a chart writes the chord again at the top of
each part. A chord is spelled in the key of the song where it plays,
with the band's lowest note after the slash: piano chords voiced C, E, A
over a bass playing A are Am, not Am/C. A polychord is written
`upper|lower`, `Eb7|Fb`, and a set by its name, `4-23 T2` (see [The
chord sheet: model, layout and text](../band/chord-sheet.md)). In a
key that is a set or a twelve-tone row, where chords have no function,
each beat is written as the set its notes make instead, or as a
micromode (see [Micromodes](micromodes.md)).

## A second ear, for the band's own patterns

The library hears chords too, in a simpler setting. To play a song in a
new rhythm, or to let the band follow the chords of a pattern, it needs
the chords of one **chord channel** over one **bass line**, in whole
steps: the patterns a style writes. `library/listen.rs` does that with
the same two ideas, a score for every chord and dynamic programming for
the cuts, tuned so that every style of the library, however it plays
its chords, is heard as its chart.

Its scores need "how many steps does each pitch class sound between
step `a` and step `b`", for thousands of stretches. So it adds up once,
from the start:

```rust
struct Heard {
    sums: Vec<[f32; 12]>,
    busy: Vec<f32>,
    quiet: Vec<f32>,
    /// The pitch class of the lowest bass note that starts at each step,
    /// if any.
    bass: Vec<Option<PitchClass>>,
}
```

`sums[s][pc]` is how many of the steps *before* `s` have pitch class
`pc` sounding. The time between `a` and `b` is then one subtraction,
however long the stretch:

```rust
fn between(&self, start: usize, end: usize) -> Stretch {
    let mut heard = [0.0; 12];
    for (pc, time) in heard.iter_mut().enumerate() {
        *time = self.sums[end][pc] - self.sums[start][pc];
    }
    // ...
}
```

This is a **prefix sum**, and it turns up wherever a program asks "how
much in this range" many times. Arrays of a fixed size are `Copy` when
their items are, so building the sums with
`let mut next = heard.sums[heard.sums.len() - 1];` copies the last row,
and adding to `next` leaves the row in the vector untouched.

After the cuts, neighbours are joined. Two stretches with the same chord
become one, and so do a C and a C7 a beat later, since an arpeggio
shows its notes one by one:

```rust
match joined.last_mut() {
    Some(last) if last.chord == stretch.chord => last.length += stretch.length,
    // An arpeggio shows its notes one by one: a C that turns into
    // C7 a beat later is one C7 still unfolding. A change on a
    // bar line or half way through the bar is taken as meant.
    Some(last) if !half_bar(stretch.start) && grows(last.chord, stretch.chord) => {
        last.chord = bigger(last.chord, stretch.chord);
        last.length += stretch.length;
    }
    _ => joined.push(stretch),
}
```

`last_mut()` gives an `Option<&mut HeardChord>`, a reference to change
the last stretch through, so it grows in place. The arms are tried in
order with their **match guards** (the `if` after a pattern); a false
guard moves on to the next arm, and `_` catches the rest. What the band
does with the chords it hears is in [The library: styles and rhythms as
data](../band/library.md) and [How the band plays a chord
sheet](../band/band.md).

## Finding the key

The second half of the chapter is the key. **🔍 Find the key**, under
the harmony keys, shows the key the music sounds like it is in, with
its closest rivals fainter beside it, and what the chord playing does
in each. It is an experiment, so it is off until you turn it on. The
work is in `harmony/key_finding.rs`.

### What a key sounds like: profiles

Carol Krumhansl and Edward Kessler (1982) played listeners a scale and
then a single note, and asked how well the note fitted. The answers make
a **key profile**: twelve numbers, how much each note above the tonic
belongs to the key. Later studies counted notes in real scores instead.
Ferrous Lineup uses the profiles of Albrecht and Shanahan (2013),
counted over 490 classical pieces, which named the keys of the demos
best of those tried:

| note above the tonic | 1 | ♭2 | 2 | ♭3 | 3 | 4 | ♯4 | 5 | ♭6 | 6 | ♭7 | 7 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| major | .24 | .01 | .11 | .01 | .14 | .09 | .02 | .21 | .01 | .08 | .01 | .08 |
| minor | .22 | .01 | .10 | .12 | .02 | .10 | .01 | .21 | .06 | .02 | .06 | .05 |

The tonic and the fifth stand out in both; the third tells major (3)
from minor (♭3). In minor, ♭7 and 7 are close: the natural seventh of
the scale and the raised one of harmonic minor both turn up.

### Comparing: the Krumhansl-Schmuckler method

To find the key of some music:

1. Add up how long each of the twelve pitch classes sounds, in any
   octave: the music's **chroma**, C first. (It is the same kind of
   twelve numbers as the chord hearing's profile.)
2. Turn each profile to start on each of the twelve tonics: 24 keys.
3. Compare the chroma with each of the 24 using their **correlation**:
   1 when they rise and fall together, 0 when unrelated, −1 when
   opposite.
4. The key with the highest correlation wins.

The correlation is two steps: move both lists to an average of 0 and a
length of 1, then take their dot product.

```rust
fn normalized(values: &Chroma) -> Option<Chroma> {
    let mean = values.iter().sum::<f32>() / 12.0;
    let centered = values.map(|v| v - mean);
    let length = centered.iter().map(|v| v * v).sum::<f32>().sqrt();
    (length > 1e-9).then(|| centered.map(|v| v / length))
}
```

`Chroma` is `[f32; 12]`, and arrays have a `map` of their own that
returns an array of the same size, so no `Vec` is made. `Option` handles
silence: when nothing sounds, all twelve numbers are equal, the length
is 0, and there is no key to find, so the function returns `None`
instead of dividing by zero. `then` is the lazy cousin of `then_some`:
it runs the closure only when the condition holds, so the division
never happens for silence.

Turning a profile to another tonic uses `std::array::from_fn`, which
builds an array by calling a closure for each index:

```rust
// The profile counts from the tonic; turn it to count from C.
let turned: Chroma = std::array::from_fn(|c| profile[(c + 12 - tonic) % 12]);
```

`key_fits` returns all 24 keys with their correlation, best first. The
MIDI import uses it to check a file's key signature against its notes
(see [MIDI: messages, devices, recording and
files](../sound/midi.md#tempo-time-and-key)).

### Sure or not sure

The correlations alone do not say how sure the answer is. C major and
A minor have the same notes, so a tune on the white keys can fit both
almost equally well. `ranked` turns the 24 correlations into **shares**
that add up to 1, a *softmax*:

```rust
let best = fits.iter().copied().fold(f32::MIN, f32::max);
let weights: Vec<f32> = fits
    .iter()
    .map(|&fit| ((fit - best) * SHARE_SHARPNESS).exp())
    .collect();
let total: f32 = weights.iter().sum();
```

With `SHARE_SHARPNESS` at 12, a fit better by 0.05 gets a share
e^(12 × 0.05) ≈ 1.8 times larger. Subtracting `best` first keeps every
exponent at 0 or below, so `exp` can never overflow. A key is **clear**
when its share is at least twice the second's:

```rust
fn clear(guesses: &[KeyGuess]) -> bool {
    match guesses {
        [] => false,
        [_] => true,
        [best, second, ..] => best.share >= CLEAR_RATIO * second.share,
    }
}
```

These are **slice patterns**: `[]` matches an empty slice, `[_]` one of
exactly one item, and `[best, second, ..]` names the first two of a
longer one, with `..` for the rest. When the key is not clear, the next
keys whose share is at least 40% of the best are its **alternatives**,
two at most, and the app shows them fainter.

E Dorian has the same notes as A minor and G major, so the profiles,
which only know major and minor, cannot pick its home note with
confidence, and the app says so: A minor, or E minor, or A major. The
real tonic is among the alternatives, which is the honest answer.

### Where the key changes

A song that modulates has no single chroma. So the music is cut into
bars, and each bar gets a score for each key. One bar is too little to
go on, though: a bar holding only D7 fits G major better than C major,
even in the middle of a C major song. So each bar is scored over the
bars around it, itself counted three times, its neighbours twice, the
bars two away once.

Then a key change **costs** a penalty, 1.5 in units of correlation, and
the **Viterbi algorithm** finds the sequence of keys with the best total
score. For each bar and each key it keeps only the best way of arriving
there: staying in that key, or changing from the best key so far and
paying the penalty.

```rust
for (i, here) in windowed.iter().enumerate().skip(1) {
    let (leader, leader_score) = best_of(&best);
    let mut next = vec![0.0; count];
    for k in 0..count {
        let (previous, total) = if best[k] >= leader_score - CHANGE_COST {
            (k, best[k])
        } else {
            (leader, leader_score - CHANGE_COST)
        };
        from[i][k] = previous;
        next[k] = total + here[k];
    }
    best = next;
}
```

`from[i][k]` remembers where the best path into key `k` at bar `i` came
from, and at the end the code walks back through it to read off the
path. It is dynamic programming again, over keys instead of cuts. A new
key wins only when it fits better for long enough to pay back the
penalty, which is exactly "a change only once the new key holds": one
borrowed chord is not a modulation.

Scoring over neighbouring bars blurs *where* a change happens by a bar
or two. So each change is then moved, within three bars, to the bar
where the bars' own notes switch from fitting the old key to fitting
the new one. In the Habanera from *Carmen* it lands on bar 10, where
Bizet's D minor turns to D major.

`Project::heard_keys` does this for the whole song: synths and MIDI Out
channels that are not muted count, drums and samplers (such as an
imported recording) do not, and each note counts for as long as it
sounds in each bar. It returns `KeySpan`s, each with a start, an end and
every key ranked. **Mark on the playlist**, next to the key heard, turns
them into key markers (see [Scales, modes and
keys](scales.md#key-changes-partway-through-key-markers)):
no note moves, and Undo takes them back. It is the way to give an
imported MIDI file without key signatures the key changes it really has.

### While you play

For chords played on a MIDI or typing keyboard there is no score to look
ahead in, only the bars played so far. `KeyListener` keeps a chroma of
the past that fades:

```rust
/// A bar ended: it joins the past, and the key may change.
pub fn end_bar(&mut self) {
    if normalized(&self.bar).is_none() {
        return;
    }
    let heard = self.heard();
    self.past = heard.map(|v| v * LISTENER_MEMORY);
    self.bar = [0.0; 12];
    // ...
}
```

Each bar, everything heard before counts 0.84 times as much, so a bar
from four bars ago counts about half (0.84⁴ ≈ 0.5). A new key takes over
only after it has led for two bars in a row. The listener does not
start from nothing either: `KeyListener::starting_in` fills the past
with music shaped like the key around it, the key heard in the song's
notes there or the project's key, so the first chords you play are
heard in its light. Play C, F and G7 over an E Dorian song and they are
first heard against A minor; a few bars later the listener settles on
C major, clear.

The app shows the key from the most immediate source it has (see
`key_finder.rs`): what you play, if you played in the last four bars;
otherwise the keys heard in an imported recording, or in the song's
notes at the playhead; in pattern mode, the selected pattern's notes.
The line says which: "Sounds in what you play:", "in the notes here",
"in this pattern". How a recording's keys are heard is in [Importing a
recording (experimental)](listen.md).

### What a chord is in each key

The same chord does a different job in each key it could belong to,
which is why the alternatives matter. `read_chord` works it out with
what the app already knows about keys: the numeral, the function, a
dominant seventh heard as the dominant of the chord it pulls to, and
chords borrowed from the parallel key (see [Roman numerals, the
harmony keys and chord
functions](numerals.md#what-a-chord-does-the-chord-sheets-functions)).
G7 reads:

| key | reading | what it does |
|---|---|---|
| C major | V7, dominant | pulls home to C |
| A minor | ♭VII7 = V7/♭III | the dominant of C, the relative major |
| G major | I7 = V7/IV | the home chord with a flat seventh, pulling to C, the IV |

Numerals count along the major scale of the tonic, as everywhere in
Ferrous Lineup, so G in A minor is ♭VII.

## How well it hears

The tests in `tests/chord_reading.rs` check bars whose harmony is known
from the scores: the Augurs chord, the Petrushka chord in *Petrushka*'s
Room, and the fourths at the start of the fair. A report, which is an
ignored test, compares the chords heard from the notes with the chart
of every demo that has one, beat by beat:

```sh
cargo test --release -p lineup-core --test chord_reading -- --ignored --nocapture
```

`tests/key_finding.rs` has a report of the same kind for keys, against
the keys and key markers the demos have from their scores:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

Measured on 30 October 2026:

| Chords, 330 charted demos, 60,322 beats | |
|---|---|
| Exactly the chart's chord | 84.0% |
| A compatible chord (same notes, or same root with notes added or left out) | 91.1% |
| The right root | 92.1% |

| Keys, from the notes | |
|---|---|
| Bars in the right key | 87.0% of 18,828 |
| Key changes in the scores found within a bar | 132 of 315 |
| Bars shown as clear that are right | 13,900 of 16,153 (86%) |

Each report also lists the demos it gets most wrong: the place to look
next. For keys, most misses are of three kinds:

- **Short visits.** A change that lasts four to eight bars, often to
  the dominant, is hard to tell from a passage that only tonicizes it,
  and many musicians would read those bars as tonicizations too.
- **Relative keys.** Same notes, another home: a song in B♭ major can
  come out as G minor.
- **Modes**, as with E Dorian above.

Some "changes that are not there" are real, too: the scores the demos
come from mark only the bigger key changes.

## Testing it

The unit tests in `hearing.rs` build a few notes by hand and check what
is heard. An arpeggio, one note per step, is its chord:

```rust
#[test]
fn an_arpeggio_is_its_chord() {
    // A minor broken up, one note per step: no two notes ever sound
    // together.
    let notes = [
        held(45, 0.0, 1.0),
        held(52, 1.0, 2.0),
        held(57, 2.0, 3.0),
        held(60, 3.0, 4.0),
    ];
    assert_eq!(chord_of(&hear(&notes, 4.0)).unwrap(), "Am");
}
```

`held` makes a note of part 0, and `chord_of` names the best reading.
`unwrap` is fine in a test: if there is no name, the test should fail
right there. The others check
that a held chord is itself, that a tune's passing notes leave the chord
alone, that the Augurs chord is E♭7 over F♭ and the Petrushka chord two
arpeggios a tritone apart, that a stack of fourths is a set, and that
nothing heard is silence. `key_finding.rs` checks that a cadence names
its key, that the shares add up to one, that the pentatonic scale alone
is not clear, that one borrowed bar is not a modulation and that the
listener changes key once the new one holds; `tests/key_finding.rs`
checks that most bars
of the demos are heard in their key and that clear modulations, such as
the Habanera's, are found on their bar.

Exercises on hearing are in [Exercises: music theory in
code](exercises.md).
