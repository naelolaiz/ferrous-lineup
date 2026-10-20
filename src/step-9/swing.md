# Swing: eighth notes played long-short

Files: [`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs),
[`crates/lineup-core/src/diff.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
[`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs),
[`crates/lineup-app/src/app/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/swing.rs)
and [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)

Clap along to a jazz or blues record and say "da-da da-da" with the
eighth notes. It will not fit: the band plays "doo-ba doo-ba", a long
note and a short one, in every beat. That unevenness is **swing**, and
it is why a shuffle bounces and a pop song does not. Until now Ferrous
Lineup played every pair of eighth notes evenly, or, in the swing
beats of its library, as a dotted eighth and a sixteenth, which is
much too jerky. Now a song has one **Swing** amount, from even
("straight") to a full triplet swing.

![The chord sheet with the band playing Swing at 110 BPM. Next to the tempo, "Swing 100%", with its hint: "Swing: the second eighth note of every beat comes late, so pairs of eighths are played long-short, as in jazz, blues and shuffles. "straight" plays them even, 50% is a light swing and 100% a triplet: two thirds and one third. Each style remembers what you set."](../images/step-9-swing-sheet.png)

## Trying it

Open **File > Demos > Songs > Blues > Midnight Shuffle**. The playlist
says **Swing 100%** next to **Loose**. Drag the number down to
*straight* while it plays, then back up: the drums, the piano, the bass
and the harmonica all change their feel together, but not a single note
moves in the piano roll. With a band, the same control sits next to the
tempo on the chord sheet.

Each band style comes with its own swing: **Swing**, **Jazz ballad**,
**Jazz waltz** and **Shuffle** at 100%, **Rock and roll** at 50%, every
other style straight. Change it, and the style remembers: the next time
you pick it, in any song, it plays with your amount. The amount is part
of the song too, so it is saved with it, undo takes it back, and WAV
and MIDI exports play it.

## The music: how long is the long note?

Swing is about the two eighth notes of one beat. Played straight, each
lasts half the beat. Played swung, the first gets longer and the second
starts later and is shorter. How much longer is the whole question:

| Swing | The two eighths | Ratio | Second eighth at 120 BPM |
|---|---|---|---|
| straight | 1/2 + 1/2 of the beat | 1 : 1 | 250 ms after the beat |
| 50% | 7/12 + 5/12 | 7 : 5 | 292 ms |
| 100% | 2/3 + 1/3: a triplet | 2 : 1 | 333 ms |
| (a dotted eighth and a sixteenth) | 3/4 + 1/4 | 3 : 1 | 375 ms |

The 100% row is how the textbooks explain swing: split the beat into
three (a **triplet**) and play the first two parts as one note. That is
why jazz charts write plain eighth notes with "Swing" over the first
bar, or a small sign showing two eighths equal to a triplet's quarter
and eighth. Writing every swung pair as a triplet would fill the page
with brackets and 3s, and musicians know what "Swing" means.

Real players do not stick to one ratio. Measurements of jazz drummers
show them swinging harder at slow tempos and evening out at fast ones,
where there is no time left for a long-short pair. A ballad can lean
past 2:1, and a very fast bebop line is nearly straight. The control
lets you try the whole range.

The last row is what Ferrous Lineup used to play. A **dotted eighth and
a sixteenth** is the closest a grid of sixteenths can get to swing, but
it is harder than a triplet and sounds stiff, like a march. It is also
a real rhythm of its own: "Hap-py" in Happy Birthday is written that
way, and stays that way here, because swing only moves notes that sit
on the second eighth of a beat (or a few ticks from it, see below).

A **shuffle** is the blues' name for the same long-short feel, usually
a full triplet, with the drummer's hi-hat or the guitar's chords
chugging on both notes. Some blues are written in **12/8** instead
(*Trouble in Mind* in the demos): four beats of three eighths each,
the triplet feel written out. That is why the control is greyed out in
6/8, 9/8 or 12/8: the beats already split in three, so there are no
pairs to swing. The same goes for 5/8 or 7/8, whose beats are the
eighths themselves. It works in every time signature counted in quarter
notes or longer, such as 4/4, 3/4 or 2/2.

Hip hop, funk and "new jack swing" swing their **sixteenths** instead:
the pairs inside each eighth note. This control swings eighths only.

## How it works

### One number, and the notes as they are played

The project has one more field, next to `loose_timing`:

```rust
pub swing: u8, // 0 to MAX_SWING (100)
```

The notes in the patterns never change. Instead, everything that
*plays* the song asks for the notes as they are played:

```rust
pub fn played_notes(&self, pattern: usize, channel: usize) -> Cow<'_, [NoteEvent]> {
    let Some(written) = self.pattern(pattern).map(|p| p.notes(channel)) else {
        return Cow::Borrowed(&[]);
    };
    match self.pattern_swing(pattern) {
        0 => Cow::Borrowed(written),
        swing => Cow::Owned(swung(written, swing, self.plays_hits(channel))),
    }
}
```

`Cow` is "clone on write" from the standard library: a value that is
either **borrowed** or **owned**. Most songs do not swing, and for them
`played_notes` hands back the pattern's own list without copying a
single note. Only a swinging pattern gets a new `Vec`. Both kinds can
be read the same way, as a slice, and a caller that needs a `Vec` of
its own calls `.into_owned()`, which copies only if it was borrowed.

The `'_` in `Cow<'_, ...>` says the borrowed case borrows from `self`:
the result cannot outlive the project it came from.

The audio thread gets its notes from `played_notes` (when a project
is loaded, and whenever notes change), and so do the MIDI export and
the accompaniment that follows the chords you play. The piano roll,
the step buttons and the chord sheet read the written notes, so the
grid you edit stays a grid.

### Moving the second eighth

Ferrous Lineup counts time in ticks, 24 to a step and 96 to a quarter
note. The second eighth of a beat starts on tick 48; a triplet's third
note starts on tick 64. So full swing is 16 ticks late:

```rust
const BEAT: usize = 4 * TICKS_PER_STEP;              // 96
const OFFBEAT: usize = BEAT / 2;                       // 48
const FULL_DELAY: usize = BEAT * 2 / 3 - OFFBEAT;      // 16

pub fn delay(swing: u8) -> usize {
    let swing = usize::from(swing.min(MAX_SWING));
    (FULL_DELAY * swing + usize::from(MAX_SWING) / 2) / usize::from(MAX_SWING)
}
```

Adding half of 100 before dividing by 100 rounds to the nearest tick
instead of always down: 50% is 8 ticks exactly, 70% is 11.2 and becomes
11.

`swung` then moves every start and end that falls on an offbeat. The
first version matched it exactly:

```rust
let played = |tick: usize| match tick % BEAT {
    OFFBEAT => tick + late,
    _ => tick,
};
```

`OFFBEAT` in a `match` arm is a constant, so the arm compares with
its value, 48. Had it been a lowercase name like `offbeat`, Rust would
read it as a *new variable* that matches anything, and the `_` arm
would never run. (The compiler warns about exactly that, and the
naming convention, capitals for constants, keeps the two apart.)

The band's guitar later needed "on the offbeat" to mean "close to it":
a strum starts its strings a tick apart, 48, 49, 50..., and only the
first string would have moved
([A bigger band](bigger-band.md#strums-and-swing)). The arm is now a
**match guard**:

```rust
const NEAR: usize = BEAT / 16; // 6 ticks, a sixty-fourth note

let played = |tick: usize| match tick % BEAT {
    offset if offset.abs_diff(OFFBEAT) < NEAR => tick + late,
    _ => tick,
};
```

Here `offset` *is* a new variable that matches anything, on purpose,
and the `if` after it decides whether the arm applies. `abs_diff`
gives the distance between two numbers whichever is bigger:
`offset - OFFBEAT` on a `usize` below 48 would underflow and panic in a
debug build.

A note that ends on the offbeat ends later too, so the first eighth
really is long, legato into the second. A drum hit is different: a
sampled drum sounds to its end however long the note is held, so for
drum channels only the starts move. A note is then rebuilt from its
new ticks with `NoteEvent::exact`, the function the MIDI import uses
for notes between steps: the swung eighth on step 2 becomes step 2
with a `nudge` of 16 ticks, which the audio thread already knew how to
play late.

### No new command for the audio thread

The audio thread learns about every change from
[`diff`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
which compares the project before and after an edit. It now compares
the *played* notes:

```rust
let old_notes = match old_index {
    Some(old) => before.played_notes(old, old_channel),
    None => Cow::Borrowed(&[][..]),
};
if old_notes != after.played_notes(index, channel) {
    changes.push(Change::Notes { pattern: index, channel });
}
```

Changing the swing therefore needs no command of its own: the
patterns whose notes move are sent again, the others are left alone.
The same goes for a pattern moved into 6/8, which stops swinging. And
undo, which swaps a whole earlier project in, is just another diff.

### The library, rewritten in even eighths

The library's swing beats were written the only way a sixteenth grid
allowed, as dotted pairs:

```text
Swing ride   "x... x..x x... x..x"     (before)
Swing ride   "x... x.x. x... x.x."     (now, with .swung(100))
```

Now they are written the way sheet music writes them, in even eighths,
and a rhythm carries the amount it swings by, `Rhythm::swing`. The
band, a rhythm or style picked from the library, and a demo all give
the song the swing of the rhythm they play. This fixed a real clash
too: in the blues demos, the boogie bass played even eighths under the
drums' dotted pairs, two different feels at once. Now everything swings
together.

Two demos written for Ferrous Lineup, *Midnight Shuffle* and *Rolling
Boogie*, had their tunes in dotted pairs for the same reason; they are
now even eighths. Tunes taken from scores keep the rhythms their
scores print. Folk and classical songs keep even eighths whatever beat
is under them, so *Scarborough Fair*, played over the ride pattern of
a jazz waltz, does not turn into jazz.

### Remembered per style

The app keeps the amounts you set, by style name, in a `BTreeMap`:

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Remembered(BTreeMap<String, u8>);
```

egui stores settings between runs by an `Id` *and a type*, so the map
is wrapped in a small struct of its own, a **newtype**: a
`BTreeMap<String, u8>` stored by another part of the app could never be
mistaken for this one. A `BTreeMap` keeps its keys sorted, so the
settings file lists the styles in the same order every time.

When you pick a band style, a rhythm or a style in the library, the
song first takes the style's own swing (in `lineup-core`, which knows
nothing about settings), and then the app replaces it with the
remembered amount, if there is one, in the same undo step.

## Exercises

1. Open *Midnight Shuffle* and play it at *straight*, 50% and 100%.
   Which instrument shows the change most? Which setting sounds like
   a march, which like a shuffle?
2. At 96 BPM, how many milliseconds after the beat does the second
   eighth start at straight, 50% and 100%? Check your 100% answer
   against the table: why is it not 333 ms here?
3. In a song with 100% swing, write a dotted eighth and a sixteenth
   in the piano roll, then two eighths. Which pair moves? Many jazz
   players read both the same way, as a triplet. What would Ferrous
   Lineup need to know to do that, and why could it go wrong on a
   march?
4. Put a swinging song in 6/8 with the Time box. What happens to the
   Swing control, and to the sound? Write the same groove in 12/8 by
   hand: where do the long and short notes go?
5. (Rust) `played_notes` returns a `Cow`. What would change if it
   returned a `Vec<NoteEvent>` instead? Count how many notes `diff`
   would copy for a song of 20 patterns and 8 channels, on every edit.
6. (Rust) Write a test that a note held from the second sixteenth of a
   beat (step 1) to the offbeat ends 16 ticks later at full swing on a
   piano channel, but not on a drum channel.
7. (Rust, harder) Add sixteenth-note swing: which ticks of a beat move,
   by how much at full swing, and what must `swings_in` say about
   6/8, where the sixteenths come in pairs too?
