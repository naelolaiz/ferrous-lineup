# Tempo, meter, swing and feel

Files: [`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tempo.rs),
[`crates/lineup-core/src/remeter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/remeter.rs),
[`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/playlist/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/tempo.rs),
[`crates/lineup-app/src/time_picker.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/time_picker.rs)
and [`crates/lineup-app/src/app/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/swing.rs)

In Ferrous Lineup a step is always a sixteenth note. A waltz, a
ritardando, a shuffle and a Bulgarian 7/8 are all written on that same
grid. What changes is how the steps are *counted* and *played*:

| What | What it changes | Where |
|---|---|---|
| A time signature | Where bar and beat lines fall; never a note | `meter.rs` |
| A tempo change | How long each step lasts | `tempo.rs`, `engine.rs` |
| Putting the song in another meter | Every note, so each bar becomes a bar of the new meter | `remeter.rs` |
| Swing | When the second eighth of each beat is played, not where it is written | `swing.rs` |
| Loose timing | How late each note starts, as it plays | `engine.rs` |

The music behind all of this (beats and bars, two and three, odd meters,
what a ritardando and a swing are) is explained in the user manual:
[Rhythm and
meter](https://naelolaiz.github.io/ferrous-lineup/manual/rhythm-and-meter.html),
[Tempo, time signatures and key
changes](https://naelolaiz.github.io/ferrous-lineup/manual/tempo-meter-key.html),
[Swing and feel](https://naelolaiz.github.io/ferrous-lineup/manual/swing.html)
and [Odd meters and the
Balkans](https://naelolaiz.github.io/ferrous-lineup/manual/odd-meters-and-balkans.html).
This chapter is about the code.

## `TimeSignature`: two numbers and a grouping

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "SavedTime", into = "SavedTime")]
pub struct TimeSignature {
    /// How many notes of `unit` length make a bar: the top number.
    pub beats: u8,
    /// Which note counts as a beat: 4 for quarter notes, 8 for eighths.
    /// The bottom number.
    pub unit: u8,
    /// Where the felt beats start, when they are not the usual ones: bit
    /// `i` is set when a beat starts on the bar's `i`-th `unit` note.
    /// 0 means the usual beats (see [`TimeSignature::usual_groups`]).
    ///
    /// A `u64` keeps `TimeSignature` `Copy`, so it can be passed around
    /// as freely as a number; a `Vec` could not be.
    groups: u64,
}
```

The top number can be anything from 1 up to what fits in a pattern; the
bottom one is 1, 2, 4, 8 or 16 (`UNITS`). There is no 32: a bar of 7/32
would end in the middle of a sixteenth-note step.

Everything else is worked out from those numbers, and the arithmetic is
written so that a damaged file can't make it divide by zero:

```rust
pub fn unit_steps(self) -> usize {
    (16 / usize::from(self.unit.clamp(1, 16))).max(1)
}

pub fn steps_per_bar(self) -> usize {
    let unit = usize::from(self.unit.clamp(1, 16));
    (usize::from(self.beats) * 16 / unit).max(1)
}
```

`clamp` keeps the unit between 1 and 16, so the division always has
something to divide by, and `max(1)` keeps a bar at least one step long,
because "which bar is step 50 in?" divides by the bar's length. The
menus and `set_time_marker` only accept time signatures that pass
`is_valid`, but a hand-edited file can still say `beats: 0` or
`unit: 0`, and the test `nonsense_does_not_divide_by_zero` checks that
the arithmetic survives it.

`new` is a `const fn`, so time signatures can be constants:

```rust
pub const COMMON: TimeSignature = TimeSignature::new(4, 4);

pub const ALL: [TimeSignature; 10] = [
    TimeSignature::new(2, 4),
    TimeSignature::new(3, 4),
    // ...
];
```

### A grouping inside a `Copy` type

In 7/8 the seven eighths are felt as beats of two and three, and the
order matters: 2+2+3/8 is a Bulgarian rachenitsa, 3+2+2/8 a Greek
kalamatianos. The grouping is part of the time signature.

`TimeSignature` is passed by value everywhere, which needs it to be
`Copy`: copied like an integer, with no heap memory. A grouping would
naturally be a `Vec<u8>`, `[2, 2, 3]`, but a type that holds a `Vec`
can't be `Copy`. So the grouping is a **bit mask** instead: bit `i` is
set when a beat starts on the bar's `i`-th note. 2+2+3/8 starts its
beats on notes 0, 2 and 4, so its mask is `0b10101`. A `u64` has room
for 64 notes (`MAX_GROUPED_BEATS`), more than any real bar.

Songs and rhythms are `const` tables, so a grouping has to be buildable
at compile time too:

```rust
pub const fn of_groups(groups: &[u8], unit: u8) -> Self {
    let mut mask = 0_u64;
    let mut at = 0;
    let mut index = 0;
    // `for` loops are not allowed in a `const fn` yet: `while` is.
    while index < groups.len() {
        assert!(groups[index] > 0, "a group of no notes");
        assert!(at < Self::MAX_GROUPED_BEATS as usize, "too many notes");
        mask |= 1 << at;
        at += groups[index] as usize;
        index += 1;
    }
    assert!(at > 0 && at <= Self::MAX_GROUPED_BEATS as usize);
    let mut time = TimeSignature::new(at as u8, unit);
    // The usual grouping is written the plain way, so 3+3/8 and 6/8
    // are the same time signature.
    if mask != time.usual_mask() {
        time.groups = mask;
    }
    time
}
```

`1 << at` is a 1 moved `at` places to the left, and `|=` switches that
bit on without touching the others. An `assert!` that fails inside a
`const fn` evaluated for a constant stops the **build**, not the
program, so a mistyped grouping in a song table never reaches anyone.
For groupings typed by a person there is `grouped`, which checks the
same things and returns an `Option` instead.

The last few lines keep one rule: the usual grouping is stored as 0.
So `"3+3/8"` and `"6/8"` give the same value, and `==` says they are
equal. Without it the Time menu would show 6/8 as not selected after
someone typed 3+3.

The `groups` field is private, so code outside `meter.rs` can only make
a grouping through `grouped` or `of_groups`. `beats` and `unit` are
public, though, so someone could change `beats` and leave a mask that
no longer fits. The private `mask()` covers that: a mask that does not
fit the top number is ignored, and the usual beats are used.

### Beats of different lengths

In 2+2+3/8 the beats are 4, 4 and 6 steps long, so "which beat is this
step in?" can't be a division. Everything asks `beat_at` instead:

```rust
pub struct Beat {
    /// Counted from 0: the first beat of the bar is 0.
    pub index: usize,
    /// The step it starts on, counted from the start of the bar.
    pub start: usize,
    /// How many steps it lasts.
    pub length: usize,
}
```

From the test `irregular_meters_group_their_beats`, where `typed` is
short for `text.parse().unwrap()`:

```rust
let balkan = typed("2+2+3/8");
assert_eq!(balkan.beat_starts(), [0, 4, 8]);
assert_eq!(balkan.beat_at(9).length, 6);
assert_eq!(balkan.beat_at(9).index, 2);
```

`beat_at` finds the last beat start at or before the step with
`rposition`, which searches a slice from the end.

There is no "steps per beat" function at all, on purpose. Any code that
assumed equal beats would be wrong in an irregular meter, and with no
such function to call, nobody can make that mistake. (Removing a
function is also the quickest way to find every caller: the compiler
lists them.)

A bar can be asked about its beats in three ways, because three parts
of the program mean different things by "beat":

| Function | Answers | 4/4 | 6/8 | 7/8 | 2+2+3/8 | 5/4 |
|---|---|---|---|---|---|---|
| `groups` | the beats a metronome clicks, in units | 1 1 1 1 | 3 3 | 1×7 | 2 2 3 | 1×5 |
| `usual_groups` | the unit notes in a usual beat | 1 | 3 | 1 | 1 | 1 |
| `felt_groups` | the beats a player leans on | 2 2 | 3 3 | 2 2 3 | 2 2 3 | 2 3 |

`felt_groups` is what the band uses for its "oom-pa": a grouping if
there is one, the three-eighth beats of 6/8, and otherwise beats of two
with a three at the end. Bars of three notes or fewer, like 3/4, are
one felt beat. How the band plays them is in [The
band](../band/band.md).

### Reading and writing "2+2+3/8"

`TimeSignature` implements `Display`, so `format!("{time}")` writes
"3/4", or "2+2+3/8" with the grouping as the top number, everywhere in
the same way. The other direction is the standard `FromStr` trait, which
is what makes `"7/8".parse::<TimeSignature>()` work:

```rust
impl FromStr for TimeSignature {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (top, bottom) = text
            .split_once('/')
            .ok_or(tr("write it as two numbers, like 7/8"))?;
        // ... then the bottom number, the top number or numbers, and the bar's size
    }
}
```

`split_once('/')` gives `Some((before, after))` or `None`; `ok_or` turns
the `None` into an error, and `?` returns it at once. The error is a
`String` the Time menu shows as it is, in red, so the messages go
through `tr` and come out in the app's language ([Two
languages](../program/languages.md)).

The Time menu's "Any other" row keeps what is being typed in a small
`Draft` (the top number, the bottom number, and the grouping as text)
in egui's memory between frames, and builds the time signature with
the same `parse`. If the groups don't add up to the top number, it says
so: "the groups add up to 8, not 11".

### Saving it without changing old files

A time signature without a grouping must be saved exactly as before,
`(beats: 7, unit: 8)`, so older versions keep opening the file. The
attribute `#[serde(from = "SavedTime", into = "SavedTime")]` on
`TimeSignature` says: to save one, turn it into a `SavedTime` first; to
load one, read a `SavedTime` and turn it back:

```rust
#[derive(Serialize, Deserialize)]
struct SavedTime {
    beats: u8,
    unit: u8,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    groups: Vec<u8>,
}
```

`SavedTime` can hold a friendly `Vec` (`groups: [2, 2, 3]`) because it
only lives for a moment while saving or loading. The `From<SavedTime>`
conversion leaves out a grouping that does not add up to the top number,
so a hand-edited file can't make a bar whose beats don't fit.

## `Meter`: time signatures that change

A song can change time signature partway through: a hymn's last line
in 3/4, Mussorgsky's Promenade alternating 5/4 and 6/4. Each change is
a marker on the playlist:

```rust
pub struct TimeMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub time: TimeSignature,
}
```

A marker always starts a new bar. One placed in the middle of a bar cuts
that bar short, which is how a score writes a bar with a beat missing.

With markers, "which bar is step 50 in?" depends on everything before
step 50, so every question about bars goes to one type:

```rust
pub struct Meter {
    first: TimeSignature,
    /// Sorted by start, all after step 0, no two on the same step.
    markers: Vec<TimeMarker>,
}
```

The comment on `markers` is an *invariant*, a rule that is always true.
The fields are private and `Meter::new` makes it true (it drops markers
on step 0, sorts the rest and removes duplicates with `dedup_by_key`),
so no method has to check it again.

`bar_at` walks the song in **stretches**, one per time signature,
counting the bars it passes:

```text
4/4 from 0, 3/4 from 32:
stretch 1: steps 0..32,  16 a bar -> bars 0 and 1
stretch 2: steps 32..,   12 a bar -> bars 2, 3, 4, ...
```

```rust
fn stretches(&self) -> impl Iterator<Item = (usize, usize, TimeSignature)> + '_ {
    let starts =
        std::iter::once(TimeMarker::new(0, self.first)).chain(self.markers.iter().copied());
    let ends = self
        .markers
        .iter()
        .map(|marker| marker.start)
        .chain(std::iter::once(usize::MAX));
    starts
        .zip(ends)
        .map(|(from, end)| (from.start, end, from.time))
}

pub fn bar_at(&self, step: usize) -> Bar {
    let mut bars_before = 0;
    for (start, end, time) in self.stretches() {
        let bar = time.steps_per_bar();
        if step < end {
            let index = (step - start) / bar;
            let bar_start = start + index * bar;
            return Bar {
                number: bars_before + index,
                start: bar_start,
                length: bar.min(end - bar_start),
                time,
            };
        }
        bars_before += (end - start).div_ceil(bar);
    }
    unreachable!("the last stretch never ends")
}
```

`stretches` builds an iterator out of others: the starts (the first time
signature, then each marker) are `zip`ped with the ends (each marker,
then `usize::MAX` for "never"). `div_ceil` divides rounding up, so a
short bar before a marker still counts as a bar, and `bar.min(end -
bar_start)` gives that short bar its real length. `unreachable!` tells
the reader, and the compiler, that the loop always returns: the last
stretch ends at `usize::MAX`.

`bars()` gives every bar without end:

```rust
pub fn bars(&self) -> impl Iterator<Item = Bar> + '_ {
    let mut next = 0;
    std::iter::from_fn(move || {
        let bar = self.bar_at(next);
        next = bar.end();
        Some(bar)
    })
}
```

`std::iter::from_fn` turns a closure into an iterator: each call to
`next` runs the closure. It never returns `None`, so the iterator never
stops on its own; the playlist draws its grid with
`meter.bars().take_while(|bar| bar.start <= steps)` and decides itself
how much of it to use.

The rest of `Meter` is the questions the app asks: `time_at`,
`bar(number)`, `is_bar_start`, `floor_to_bar`, `floor_to_beat`,
`beat_end`, `round_up_to_bar`, and `position(step)` for the BAR:BEAT:STEP
display. `Meter::steady(time)` is a song with no markers, and a test
checks, step by step, that it answers exactly like the `TimeSignature`
alone. `tidy()` drops the markers that change nothing: one that keeps
the time signature before it and falls on one of its bar lines.

### Who asks the meter

- **The playlist**: its grid, bar numbers, sections and snapping.
  `Snap::floor` and `Snap::next` give the start and the end of the bar or
  beat under the pointer, which is what dragging a clip, a section or
  the loop needs.
- **The channel rack and the piano roll** show one pattern, not the
  song. They ask `Project::pattern_time(pattern)`: the time signature
  where the pattern's first clip starts, or, for a pattern not in the
  song, its own `Pattern::time` (a waltz tried from the Library is in
  3/4 even in a 4/4 song), or else the project's.
- **The length of the song**, rounded up to the next bar line.
- **The pattern length menu**, which offers whole bars
  (`pattern_lengths`: 1, 2, 3, 4, 6, 8, 12 or 16 of them, as long as they
  fit) and writes "4 bars" instead of "64 steps" (`describe_length`).

### The song as it plays

Sections can repeat ([Sections, repeats and
loops](../program/project.md#sections-repeats-and-loops)), so the
song as heard is longer than the playlist as drawn. If a 3/4 section
plays twice, the song as heard has two 3/4 stretches. The arrangement's
*passes* say which part of the timeline plays when, and
`Project::played_meter` builds a second `Meter`, in song steps:

```rust
pub fn played_meter(&self, arrangement: &Arrangement) -> Meter {
    let meter = self.meter();
    let first = arrangement
        .passes
        .first()
        .map_or(self.time_signature, |pass| meter.time_at(pass.from));
    let mut markers = Vec::new();
    for pass in &arrangement.passes {
        markers.push(TimeMarker::new(pass.start, meter.time_at(pass.from)));
        for marker in meter.markers() {
            if marker.start > pass.from && marker.start < pass.from + pass.length {
                markers.push(TimeMarker::new(
                    pass.start + marker.start - pass.from,
                    marker.time,
                ));
            }
        }
    }
    Meter::new(first, markers).tidy()
}
```

Every pass gets a marker for the time signature it starts in, plus the
markers inside it, and `tidy` then removes the ones that change
nothing, such as a 4/4 pass after another 4/4 pass on a bar line. The
transport uses this meter for the position display and for Page
Up/Page Down, so "the next bar" is the next bar you hear.

The metronome uses it too. `played_clicks` turns it into one `Click`
per song step (`Bar`, `Beat` or `None`), and the app sends that list to
the audio thread with `Command::SetSongClicks`, so the metronome clicks
the beats of each bar: the two dotted quarters of 6/8, the three groups
of 2+2+3/8, a higher click on every "one". In pattern mode it is
`pattern_clicks`, in the pattern's own time. The count-in and the
metronome's sound are in [Practice tools](../band/practice.md).

## Tempo changes

### `Tempo`, always valid

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(from = "f32", into = "f32")]
pub struct Tempo {
    bpm: f32,
}
```

The field is private, so the only way to make a `Tempo` is `Tempo::new`,
which clamps the value between `MIN` (10) and `MAX` (522) BPM and turns
a NaN into the default tempo, 130. Every `Tempo` in the program is
therefore valid. Files go through the same door: a tempo is saved as a
plain number, and `From<f32>` reads it back with `Tempo::new`, so a
file that says `tempo: 9000.0` opens at 522 ([Saving](../program/saving.md)).

### Markers, and a map

A tempo change is a marker on the playlist, like a time signature
change:

```rust
pub struct TempoMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub tempo: Tempo,
    /// How many steps the change takes; 0 for at once. Project files
    /// leave it out when it is 0.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub glide: usize,
}
```

With `glide` at 0 the tempo changes at once, as after a pause. With a
glide it moves a little on every step until it arrives: down for a
ritardando, up for an accelerando. A step is a sixteenth note, so a
glide over two bars of 4/4 is 32 small changes, close enough to a curve
that nobody hears the stairs. The tempo never changes in the middle of a
step.

```rust
fn bpm_on(&self, step: usize, from: f32) -> f32 {
    let into = step - self.start + 1;
    if into >= self.glide {
        return self.tempo.bpm();
    }
    let done = into as f32 / self.glide as f32;
    from + (self.tempo.bpm() - from) * done
}
```

A glide of 4 steps from 100 to 60 BPM plays 90, 80, 70 and 60: the last
step of the glide is already at the new tempo.

`TempoMap` is to tempo what `Meter` is to bars: the first tempo and the
markers, with the same invariant, and one main question:

```rust
pub fn tempo_at(&self, step: usize) -> Tempo {
    // The marker in effect so far, and the tempo its glide started from.
    let mut current: Option<(&TempoMarker, f32)> = None;
    let bpm_on = |current: Option<(&TempoMarker, f32)>, step: usize| match current {
        Some((marker, from)) => marker.bpm_on(step, from),
        None => self.first.bpm(),
    };
    for marker in self.markers.iter().take_while(|m| m.start <= step) {
        let from = bpm_on(current, marker.start - 1);
        current = Some((marker, from));
    }
    Tempo::new(bpm_on(current, step))
}
```

A glide starts from whatever tempo was playing on the step before its
marker, even halfway through another glide, so the loop carries `from`
along: each marker asks the one before it "where were you, one step
before me?". `marker.start - 1` can't underflow, because no marker is on
step 0.

### The tempo strip

The strip above the playlist's bar numbers draws the map as a line. It
only needs the steps where the line bends (the start, each marker and
the end of each glide), joined pairwise with `windows(2)`:

```rust
let mut bends = vec![0, steps];
for marker in map.markers() {
    bends.push(marker.start);
    bends.push(marker.start + marker.glide);
}
bends.retain(|&step| step <= steps);
bends.sort_unstable();
bends.dedup();
let mut points = Vec::new();
for pair in bends.windows(2) {
    let (from, to) = (pair[0], pair[1]);
    points.push(pos2(layout.x(from), y(map.tempo_at(from))));
    points.push(pos2(layout.x(to), y(map.tempo_at(to.saturating_sub(1)))));
}
```

Each segment goes from the tempo on its first step to the tempo on its
last, so a sudden change becomes a vertical jump and a glide a slope.
The labels are written the way a score writes them: `describe`
compares the marker's tempo with `tempo_before` its step, and writes
"♩ = 90" for a sudden change, "rit. → 60" for a glide down and
"accel. → 140" for a glide up.

The toolbar's TEMPO box sets the tempo the song *starts* at, and
`scale_tempo` moves every marker by the same proportion:

```rust
pub fn scale_tempo(&mut self, tempo: Tempo) {
    let ratio = tempo.bpm() / self.tempo.bpm();
    self.tempo = tempo;
    for marker in &mut self.playlist.tempo_markers {
        marker.tempo = Tempo::new(marker.tempo.bpm() * ratio);
    }
}
```

Practising a piece more slowly keeps its ritardando, only slower too.
While a song whose tempo changes is playing, the box shows TEMPO NOW
instead: `tempo_at` the playhead's place on the timeline.

### Every step at its own tempo

`Project::played_tempos` gives the tempo of every step of the song *as
it plays*, so a section that slows down at its end slows down every time
round:

```rust
pub fn played_tempos(&self, arrangement: &Arrangement) -> Vec<Tempo> {
    let map = self.tempo_map();
    if map.is_steady() {
        return Vec::new();
    }
    (0..arrangement.length)
        .map(|step| {
            let timeline = arrangement.timeline_step(step).unwrap_or(step);
            map.tempo_at(timeline)
        })
        .collect()
}
```

The list is built on the UI thread and sent with
`Command::SetSongTempos`, next to the song. The audio thread may not
allocate, and when a new list arrives the old one goes back to the UI
thread to be freed, like old songs and old notes ([Two
threads](../program/threads.md)). An empty list, the usual case, means
"play the TEMPO box's tempo", and costs nothing.

A step must last as long as *its own* tempo says, so the tempo has to be
set before the step starts. The sequencer's `due` says which step starts
on the next tick ([The sequencer](sequencer.md)), and the
audio thread asks it first, on every frame:

```rust
if let Some(step) = self.sequencer.due() {
    // ...
    let tempo = self.tempo_of_step(step);
    if tempo != self.tempo_now {
        self.set_tempo(tempo);
    }
}
if let Some(step) = self.sequencer.tick() {
```

`tempo_of_step` is the song's tempo for that step in song mode, and the
TEMPO box's in pattern mode, where you work on one pattern and want a
steady loop. (The part left out above, `lap_tempos`, is the speed
trainer's next tempo, which takes over when the loop comes round:
[Practice tools](../band/practice.md).)

Notes already sounding have a length in frames, worked out at the old
tempo. `set_tempo` stretches them so they still end on the step they
should:

```rust
fn set_tempo(&mut self, tempo: Tempo) {
    self.tempo_now = tempo;
    let before = self.sequencer.step_frames();
    let next_step = self.sequencer.frames_to_next_step();
    self.sequencer.set_tempo(tempo);
    let stretch = self.sequencer.step_frames() / before;
    for channel in &mut self.channels {
        channel.instrument.tempo_changed(next_step, stretch);
        channel.effects.set_tempo(tempo);
    }
    // ...
}
```

and each instrument works out what is left of every held note:

```rust
pub(crate) fn stretched(left: f64, next_step: f64, stretch: f64) -> f64 {
    if left <= next_step {
        left
    } else {
        (next_step + (left - next_step) * stretch).round()
    }
}
```

The frames up to the next step belong to the step already playing,
which keeps its length; only the rest is stretched. A note with 10 000
frames left, 2 000 of them in this step, at half the tempo (a stretch of
2) has 2 000 + 8 000 × 2 = 18 000 left. Effects that follow the beat,
such as a delay set in note values, get the new tempo too
([The mixer](mixer.md)).

An export plays the song through the same audio thread, so it gets the
same list. Its length is no longer "steps times the length of a step":
`played_seconds` adds up every step at its own tempo, with
`tempo::seconds`:

```rust
pub fn seconds(steps: usize, tempo: impl Fn(usize) -> Tempo) -> f64 {
    (0..steps)
        .map(|step| f64::from(tempo(step).seconds_per_beat()) / 4.0)
        .sum()
}
```

`impl Fn(usize) -> Tempo` takes any closure from a step to a tempo, so
the same function serves a `TempoMap` in a test and the played list in
`played_seconds`.

## Putting a song in another time signature

A time signature only moves bar lines. But pick a waltz in the Library
while a 4/4 song plays, and its twelve-step bars laid into sixteen-step
ones play "one two three" and then a beat of silence, every bar. To
sound like a waltz, every bar of the song has to become a 3/4 bar.
`remeter` does that, with one rule: **bar n of the old song becomes bar
n of the new one, and every note keeps its place in its bar**.

```text
4/4: | C . . . E . . . G . . . E . . . |   16 steps
3/4: | C . . E . . G . . E . . |           12 steps
```

It is a mechanical stretch, the way a tape machine would do it, not the
way a composer would rewrite a tune for a new meter.

### Squeezing a bar, to the tick

Squeezing sixteen steps into twelve puts many notes between steps:
eighth notes two steps apart become 1.5 steps apart. Rounding them to
the grid would make them uneven. So the squeeze is done in **ticks**,
24 to a step (`TICKS_PER_STEP`, the unit MIDI import uses for notes off
the grid, see [MIDI](midi.md)):

```rust
pub struct Remap {
    old: Meter,
    /// Steps in one new bar.
    bar: usize,
}

impl Remap {
    pub fn tick(&self, tick: usize) -> usize {
        let bar = self.old.bar_at(tick / TICKS_PER_STEP);
        let start = bar.start * TICKS_PER_STEP;
        let length = bar.length * TICKS_PER_STEP;
        let new_bar = self.bar * TICKS_PER_STEP;
        // Rounded to the nearest tick.
        bar.number * new_bar + ((tick - start) * new_bar + length / 2) / length
    }
}
```

Two habits of integer arithmetic are in the last line. **Multiply before
dividing**: `(tick - start) / length * new_bar` would be 0 for every
tick inside the bar. **Add half before dividing** to round to the
nearest whole number instead of always down. The second eighth of a 4/4
bar, tick 48 of 384, lands on (48 × 288 + 192) / 384 = 36 of a 288-tick
3/4 bar: 1.5 steps, exactly.

A `Remap` holds the song's old `Meter`, so it works for songs that
already change meter: every old bar, whatever its time signature (a
short one too), becomes one whole new bar. `Remap::step` is the same,
rounded to the nearest step, for things that live on steps.

### Every bar keeps its duration

The tempo counts quarter notes. A 3/4 bar has three, a 4/4 bar four, so
at the same tempo the 3/4 song would be over in three quarters of the
time. So `remeter` scales the tempo by the same ratio as the bar: 4/4 at
112 becomes 3/4 at 84, and a bar lasts 2.14 seconds in both. Tempo
markers are scaled the same way, and a meter change in the middle of the
song that changed the bar length gets a tempo marker of its own, so each
stretch keeps its pace.

The scaling has to come back exactly. 108.5 BPM in 7/8, put back into
4/4, must be 108.5 × 16 / 14 = 124, not 124.00001. An `f32` has about
seven significant digits, and the ratio 16/14 has no exact value in it.
So the code multiplies first and divides last, in `f64`:

```rust
let scaled = |tempo: Tempo, step: usize| {
    Tempo::new((f64::from(tempo.bpm()) * bar as f64 / old_bar(step) as f64) as f32)
};
```

108.5 × 16 = 1736 is exact, and 1736 / 14 = 124 is exact too. A test
takes several tempos through 7/8, 3/4, 13/16, 6/8 and 12/8 and back to
4/4, and checks they come back unchanged.

So after `remeter` every note still starts at the same moment, in
seconds, and lasts as long. The music sounds the same; only how it is
written and counted changes. What makes a song *sound* like a waltz is
the drums and the way the chords are struck, which is why the Library
and the Time box re-meter the song **and** give it a rhythm in the new
meter. That choice of rhythm is in [The
Library](../band/library.md).

### Patterns that grow too long

A pattern holds at most 256 steps (`MAX_STEPS`). Sixteen bars of 4/4 are
exactly that, but sixteen bars of 12/8 are 384. Rather than drop notes,
`remeter` cuts such a pattern into pieces of whole bars that fit:

```rust
let chunk = MAX_STEPS / bar * bar;
```

256 / 24 × 24 is 240 steps: ten bars of 12/8, then six. The pieces are
named "Theme", "Theme (2)" and so on, and each clip of the pattern
becomes clips of the pieces, one after the other. The song sounds the
same; the playlist shows one clip more.

### Everything else moves with its bar

Clips, sections, the loop, key markers and the band's markers all go
through `song.step`. A tempo marker's glide is mapped by its two ends,
so a ritardando over two bars of 4/4 lasts two bars of 3/4. Chords
change on a beat, so they go to the nearest beat of the new bar
(`snap_to_beat`). The time markers go: the whole song is in one time
signature now.

Two markers one step apart can be squeezed onto the same step.
`Vec::dedup_by` removes neighbours its closure calls equal; the closure
gets the later element first and the earlier second, and may change the
earlier one, which is how "the later marker wins" is written:

```rust
playlist_mut.tempo_markers.dedup_by(|later, earlier| {
    later.start == earlier.start && {
        *earlier = *later;
        true
    }
});
```

`a && { ... }` runs the block only when `a` is true: the copy happens
only for markers on the same step.

`remeter_song` also works out every new pattern from the untouched
project first, keeps a clone of the old playlist, and only then writes.
Reading the project while writing it would need two borrows of the same
`Project` at once, one shared and one mutable, which the borrow checker
rightly refuses. And it runs inside `project.moving_the_band(...)`: the
band's notes are moved along with everything else, and that is not an
edit of yours, so the band keeps following the chord sheet ([The
band](../band/band.md)).

### Keeping your place

While the song plays you are somewhere, bar 13 beat 2, say. After the
squeeze, step 196 is no longer in bar 13. So after any change of meter
the app finds the bar the playhead was in, and how far into it, and
moves to the same bar and the same fraction of it:

```rust
let place = |step: usize| {
    let bar = old.bar_at(step);
    let into = (step - bar.start) as f64 / bar.length.max(1) as f64;
    let target = new.bar(bar.number);
    target.start + (into * target.length as f64).round() as usize
};
```

`old` and `new` are the `played_meter`s before and after, so repeats
count. The same closure moves where Play starts and where Stop returns
to. It works after undo and redo too, which can swap in a song with time
markers of its own, where a `Remap` would not fit.

## Swing

Swing plays the two eighth notes of each beat long-short. The project
has one amount, from 0 ("straight") to `MAX_SWING`, 100 (a triplet's two
thirds and one third):

```rust
pub swing: u8,
```

The notes in the patterns never move. Instead, everything that *plays*
the song asks for the notes as they are played:

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

`Cow` ("clone on write", from `std::borrow`) is either **borrowed** or
**owned**. Most songs don't swing, and for them `played_notes` hands back
the pattern's own notes without copying one. Only a swinging pattern
gets a new `Vec`. Both are read the same way, as a slice, and a caller
that needs a `Vec` of its own calls `.into_owned()`, which copies only
if it was borrowed. The `'_` says the borrowed case borrows from `self`:
the result can't outlive the project.

The audio thread, the MIDI export and the accompaniment that follows
your playing take `played_notes`. The piano roll, the step buttons and
the chord sheet read the written notes, so the grid you edit stays a
grid.

### Moving the second eighth

A beat is 96 ticks. The second eighth starts on tick 48; the third note
of a triplet on tick 64. So full swing is 16 ticks late:

```rust
const BEAT: usize = 4 * TICKS_PER_STEP;
const OFFBEAT: usize = BEAT / 2;
const FULL_DELAY: usize = BEAT * 2 / 3 - OFFBEAT;
const NEAR: usize = BEAT / 16;

pub fn delay(swing: u8) -> usize {
    let swing = usize::from(swing.min(MAX_SWING));
    (FULL_DELAY * swing + usize::from(MAX_SWING) / 2) / usize::from(MAX_SWING)
}
```

Adding half of 100 before dividing by 100 rounds to the nearest tick
again: 50 is 8 ticks, a 7:5 swing; 70 is 11.2 and becomes 11.

`swung` moves every start and end that falls on or near an offbeat:

```rust
pub fn swung(notes: &[NoteEvent], swing: u8, hits: bool) -> Vec<NoteEvent> {
    let late = delay(swing);
    let played = |tick: usize| match tick % BEAT {
        offset if offset.abs_diff(OFFBEAT) < NEAR => tick + late,
        _ => tick,
    };
    notes
        .iter()
        .map(|note| {
            let start = played(note.start_tick());
            let end = match hits {
                true => note.end_tick(),
                false => played(note.end_tick()),
            };
            NoteEvent {
                velocity: note.velocity,
                ..NoteEvent::exact(note.pitch, start, end.saturating_sub(start))
            }
        })
        .collect()
}
```

The first arm is a **match guard**: `offset` is a new variable that
matches anything, and the `if` after it decides whether the arm applies.
"Near" is less than six ticks either way, because the strings of a
strummed chord start a tick apart, 48, 49, 50, and all of them must
move together. `abs_diff` is the distance between two numbers whichever
is bigger; `offset - OFFBEAT` on a `usize` below 48 would underflow and
panic in a debug build.

A pattern of a constant is different from a binding. Had the arm been
`OFFBEAT => ...`, it would compare with the constant's value, 48. Had it
been a lowercase `offbeat => ...`, Rust would read it as a new variable
matching *everything*, and the `_` arm would never run. The compiler
warns about that, and the convention of capitals for constants keeps
the two apart.

A note that ends on the offbeat ends later too, so the first eighth
really is long, legato into the second. Drum hits are different: a
sampled drum sounds to its end however long the note is, so only their
starts move (`hits`). The new note is built with `NoteEvent::exact`, the
same function MIDI import uses for notes off the grid: a swung eighth on
step 2 becomes step 2 with a `nudge` of 16 ticks, which the audio thread
already plays late ([The sequencer](sequencer.md)). Notes
that are not on an offbeat (sixteenths, a dotted eighth and a
sixteenth) stay as they are.

### Where swing applies

```rust
pub fn swings_in(time: TimeSignature) -> bool {
    time.unit <= 4
}
```

Only time signatures counted in quarter notes or longer have pairs of
eighths in a beat. In 6/8 or 12/8 a beat is already three eighths, and
in 5/8 or 7/8 the eighth is the beat. `pattern_swing` returns 0 for a
pattern in such a meter, and the Swing control is greyed out when no
part of the song can swing, with a hint that says why. A band section
played in another style can bring that style's swing
(`section_swing`).

### No new command for the audio thread

The audio thread learns about every edit from `diff`, which compares
the project before and after, and it compares the *played* notes
([Telling the audio thread what changed](../program/sync.md)). So
changing the swing needs no command of its own: the patterns whose
played notes moved are sent again, and the others are left alone. A
pattern moved into 6/8, which stops swinging, is caught the same way,
and so is undo, which swaps a whole earlier project in. When nothing
swings, both sides of the comparison are borrowed `Cow`s, and comparing
them compares the slices without copying anything.

### Remembered per style

Each rhythm in the Library carries its own swing (`Rhythm::swing`), and
a style takes its rhythm's: Swing, Jazz ballad, Jazz waltz, Shuffle and
Five-four swing at 100, Rock and roll at 50, the rest straight. Their
drum patterns are written in even eighths, as sheet music writes them,
and the swing does the rest. When you change the amount, the app
remembers it for that style, between runs, in `app/swing.rs`:

```rust
#[derive(Debug, Default)]
pub(super) struct SwingMemory {
    amounts: BTreeMap<String, u8>,
    /// Whether an amount changed since they were last stored.
    changed: bool,
}

/// What is stored: a newtype, so it has a type of its own in egui's
/// store.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Remembered(BTreeMap<String, u8>);
```

egui keeps persisted values by an `Id` *and* a type. Wrapping the map in
a struct of its own, a **newtype**, gives it a type nobody else uses, so
a `BTreeMap<String, u8>` stored by another part of the app could never
be read as this one. A `BTreeMap` keeps its keys sorted, so the settings
file lists the styles in the same order every time. `store_swings` runs
once a frame and writes only when `changed` is set.

Picking a style first gives the song the style's own swing, in
`lineup-core`, which knows nothing about settings; then the app puts the
remembered amount in its place, in the same undo step.

## Loose timing

A good player is never exactly on the grid. **Loose**, in the playlist's
toolbar, sets how far off it notes may land, from *off* to 40 ms
(`MAX_LOOSE_TIMING`). It is saved in the project as `loose_timing`, in
milliseconds, and the audio thread turns it into frames once, when
`Command::SetLooseTiming` arrives.

Each note's delay is worked out when its step starts:

```rust
fn delay(loose_frames: f64, channel: usize, song_step: usize, pitch: Note) -> f64 {
    let onset = ((song_step as u64) << 8) | channel as u64;
    let together = scatter(onset);
    let apart = scatter(onset ^ (u64::from(pitch.midi()) << 40));
    loose_frames * (0.8 * together + 0.2 * apart)
}
```

A pianist is late or early by about the same amount for all the notes of
a chord, so four fifths of the delay are shared by every note a channel
starts on a step, and one fifth differs from note to note. The numbers
come from `scatter`, which mixes the bits of a seed with the SplitMix64
function:

```rust
fn scatter(seed: u64) -> f64 {
    let mut x = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    // The top 53 bits, as many as an `f64` holds exactly.
    (x >> 11) as f64 / (1u64 << 53) as f64
}
```

It looks random, but the same seed always gives the same number, so a
song plays the same way every time and a WAV export sounds like
playback. `wrapping_mul` lets the multiplication wrap past `u64::MAX`
instead of panicking in a debug build: here wrapping is the point.
`scatter` also needs no state, which suits the audio thread: no random
generator to keep, lock or seed.

A delayed note waits in a list until its frame:

```rust
pub(crate) struct Pending {
    channel: usize,
    origin: Origin,
    pitch: Note,
    velocity: u8,
    gate: u32,
    /// The frame it starts on, counted on [`AudioThread::now`]'s clock.
    at: u64,
    /// How many frames after its step that is, for a trace.
    late: u32,
}
```

The list is made with room for 512 notes (`PENDING_NOTES`) on the UI
thread, and the audio thread never lets it grow:

```rust
// A full list never grows (that would allocate): the note
// then starts on time.
if late > 0 && pending.len() < pending.capacity() {
```

Every frame, the notes whose `at` has come are started, and taken out
with `swap_remove`, which moves the last note into the gap instead of
shifting the whole list down. The order of waiting notes does not
matter, so the cheaper removal is the right one.

Why only late, never early? To start a note early the audio thread
would have to look into the next step before it starts. Late by 0 to 20
ms sounds the same as early or late by 10 ms around a grid that is
itself 10 ms late, and nobody hears a whole song 10 ms late.

## In files and MIDI files

Each of these features is saved only when it is used, so a project that
doesn't use it can still be opened by older versions ([Saving](../program/saving.md)). `format_for` writes the oldest format
that can hold the project:

| Format | Needed for |
|---|---|
| 3 | A time signature other than 4/4, or a pattern with its own |
| 5 | Time markers |
| 7 | Tempo markers, or loose timing |
| 10 | Notes off the grid, such as a re-metered song's |
| 11 | A grouped time signature, such as 2+2+3/8 |
| 16 | Swing |

When a file is read, every time and tempo marker goes through
`set_time_marker` or `set_tempo_marker`, which refuse one that starts
too late or whose bars would not fit in a pattern, as if it had been
added by hand.

A Standard MIDI File keeps tempo and meter as *meta events* on its first
track. A tempo is written as microseconds per beat, in three bytes, and
a change gets an event on the step where the played tempo changes, so a
glide becomes one event per step, which is how MIDI files write a
ritardando:

```rust
let tempo_event = |tempo: Tempo| {
    let micros = (60_000_000.0 / f64::from(tempo.bpm())).round() as u32;
    micros.min(0xFF_FFFF).to_be_bytes()[1..].to_vec()
};
// ...
for (step, pair) in tempos.windows(2).enumerate() {
    if pair[1] != pair[0] {
        let tick = (step as u32 + 1) * TICKS_PER_STEP;
        events.push((tick, TEMPO, tempo_event(pair[1])));
    }
}
```

`to_be_bytes` gives the four bytes of the `u32`, most significant
first ("big-endian", the order MIDI files use), and `[1..]` drops the
first, which is always 0 below `0xFF_FFFF`.

A time signature event holds the top number, the bottom number as a
power of two (`unit.ilog2()`: 2 for a quarter, 3 for an eighth), how
many MIDI clocks a metronome click lasts (24 to a quarter note, so 6 per
step), and a constant 8. There is one at the start and one at every
change of the played meter. MIDI has no room for a grouping, so a
grouped bar clicks every note (`ungrouped().first_beat()`). How a whole
MIDI file is written is in [MIDI](midi.md#writing-a-midi-file).

## Testing it

Each type has tests next to it that read like the rules they check:

- `meter.rs`: `a_steady_meter_counts_like_its_time_signature`,
  `a_marker_off_the_bar_line_leaves_a_short_bar`,
  `the_usual_grouping_is_written_plainly` (3+3/8 is 6/8),
  `a_grouping_is_saved_only_when_there_is_one`,
  `nonsense_does_not_divide_by_zero`.
- `tempo.rs`: `a_glide_moves_a_little_every_step` (the 90, 80, 70, 60
  above), `a_glide_starts_from_where_the_last_one_got_to` (a ritardando
  cut short by an *a tempo*), `seconds_add_up_every_step`.
- `remeter.rs`: `every_note_keeps_its_place_in_the_bar`,
  `notes_between_steps_keep_their_exact_place`,
  `every_bar_lasts_as_long_as_before` (the song's length in seconds,
  before and after, within 0.2%),
  `a_tempo_put_in_another_meter_and_back_is_the_same_tempo`,
  `a_pattern_too_long_for_the_new_bars_is_cut`.
- `swing.rs`: `full_swing_plays_the_eighths_as_triplets`,
  `half_swing_is_seven_to_five`, `a_strum_on_the_offbeat_moves_as_a_whole`,
  `notes_off_the_eighths_stay`.
- `engine.rs` plays a few notes and checks where the sound starts,
  frame by frame: `a_song_plays_each_step_at_its_own_tempo` (two steps
  at 120 BPM, then two at 60, which last twice as long) and
  `loose_timing_starts_notes_a_little_late` (every note within 960
  frames, 20 ms at 48 kHz, after its step; not all by the same amount;
  and the same every time).

One test goes through the whole app:
`styles_tried_one_after_another_keep_the_songs_tempo` opens El Choclo
and takes it through seven styles and meters (among them a tango vals in
3/4 and the Time box at 7/8), in pattern mode, in song mode stopped,
and in song mode playing. After each one, the tempo must be unchanged
when the meter is, and the song's own whenever it is back in 4/4. Then
it plays the start of the song with the [playback
trace](trace.md) and checks that every melody note starts and
lasts as it did, within 2 ms. To keep it quick in a debug build, it
plays only twelve seconds, at 8000 samples a second: plenty to time
notes.
