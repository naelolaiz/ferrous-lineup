# The sequencer: timing to the sample

Files: [`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sequencer.rs)
and [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

The audio thread plays a pattern or the song by counting audio frames.
The counting is done by a small type in `lineup-core`, the
**sequencer**, which knows nothing about sound: it only says on which
frame each step starts. Everything that sounds on a step, the notes of
every channel in every clip, is the audio thread's side, in
`engine.rs`. This chapter follows a step from the frame count to the
notes it starts. How the audio thread itself runs is in [Two
threads](../program/threads.md); how tempo changes, meters and swing
bend the steps is in [Tempo, meter, swing and feel](tempo.md).

## The arithmetic

A step is a sixteenth note, and a beat a quarter note, so a beat has
four steps (`STEPS_PER_BEAT`). At a tempo of *T* beats per minute and a
sample rate of *R* frames per second, one beat lasts `R × 60 / T` frames
and one step a quarter of that:

```text
frames per step = R × 60 / T / 4

120 BPM, 48 kHz:  48000 × 60 / 120 / 4 = 6000 frames      (exact)
130 BPM, 44.1 kHz: 44100 × 60 / 130 / 4 = 5088.46... frames (not whole!)
```

```rust
pub fn frames_per_step(tempo: Tempo, sample_rate: u32) -> f64 {
    tempo.samples_per_beat(sample_rate) / STEPS_PER_BEAT as f64
}
```

If 5088.46 were rounded to 5088, every step would be 0.46 frames short.
That is about 10 microseconds, nothing you would hear once, but it adds
up: after a few minutes the drums would be audibly ahead of anything
else running at 130 BPM. So the sequencer keeps the fraction, in an
`f64`.

## Ticking

The audio thread calls `tick` once for every frame it renders:

```rust
pub fn tick(&mut self) -> Option<usize> {
    let fired = if self.frames_until_next <= 0.0 {
        let step = self.next_step;
        if self.coming_round {
            self.laps += 1;
        }
        self.next_step = (self.next_step + 1) % self.length;
        self.coming_round = self.next_step == 0;
        self.frames_until_next += self.frames_per_step;
        Some(step)
    } else {
        None
    };
    self.frames_until_next -= 1.0;
    fired
}
```

`frames_until_next` counts down by one each frame. When it reaches zero,
the next step fires and a whole step's length is *added* to the counter,
not assigned. Whatever fraction was left over stays in the counter, so
step boundaries land at 0, 5088.46, 10176.92... rounded to the nearest
frame each time, but never drifting. `%` is the remainder operator: it
wraps the step number back to 0 at the end of the loop.

Because a step starts on the exact frame where it begins, not at the
start of the audio block that contains it, the timing does not depend on
the sound card's buffer size. This is what **sample-accurate** means.

### What else the sequencer can tell

Other parts of the program ask the sequencer more than "which step now":

- **`position`** gives the step with the fraction that has passed: 4.5
  is halfway through step 4. Recording uses it to put a key played
  between two steps on the nearer one ([MIDI](midi.md)).
- **`laps`** counts how many times the loop came round by playing on
  past its last step. A jump back to the start does not count. The
  speed trainer goes faster on each lap ([Practice
  tools](../band/practice.md)).
- **`due`** says which step starts on the *next* tick, without moving.
  The audio thread asks it before `tick`, so a song whose tempo changes
  can set the new tempo before that step starts, and the whole step
  lasts as long as it should.
- **`start_at`** jumps, so the given step fires on the next tick; a
  step past the end wraps round, as playback would.

`set_tempo` changes the length of the steps to come; the step already
playing keeps the frames it has left. `set_length` wraps to step 0 if
the next step is past the new end.

## Why it lives in `lineup-core`

The sequencer has nothing to do with sound cards. It only counts.
Keeping it in `lineup-core` means it is tested in milliseconds, without
audio, and the count-in uses a second one of its own.

## Testing with iterators

```rust
fn run(sequencer: &mut Sequencer, frames: usize) -> Vec<(usize, usize)> {
    (0..frames)
        .filter_map(|frame| sequencer.tick().map(|step| (frame, step)))
        .collect()
}
```

This helper ticks the sequencer `frames` times and lists every step that
fired together with the frame it fired on:

- `0..frames` is a **range**, which is an iterator over numbers.
- `filter_map` calls the closure for each frame and keeps only the
  `Some` results, unwrapped.
- `Option::map` turns `Some(step)` into `Some((frame, step))`, a tuple.
- `collect()` gathers the results into a `Vec`. Rust works out which
  collection to build from the return type.

The tests then check exact frame numbers:

```rust
assert_eq!(
    fired,
    vec![(0, 0), (6_000, 1), (12_000, 2), (18_000, 3), (24_000, 0)]
);
```

and that one minute at 130 BPM gives exactly 520 steps (130 beats × 4),
the last within one frame of where 519 exact steps put it. Underscores
in number literals like `6_000` are ignored by the compiler and just
make big numbers easier to read.

## Pattern mode and song mode

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayMode {
    /// Loop the selected pattern, to work on it.
    #[default]
    Pattern,
    /// Play the playlist from the start, and loop at the end of the song.
    Song,
}
```

The sequencer is the same in both modes. It counts steps from 0 to some
length and loops. Only the length changes: the selected pattern's length
in pattern mode, the song's length in song mode.

```rust
fn update_loop_length(&mut self) {
    let length = match self.mode {
        PlayMode::Pattern => self.pattern_length(self.pattern),
        PlayMode::Song => self.song_length,
    };
    let length = if length == 0 { STEPS_PER_BAR } else { length };
    self.sequencer.set_length(length);
}
```

An empty song would have length 0, and a loop of zero steps makes no
sense, so it becomes one silent bar.

The song arrives as `Command::SetSong { plays, length }`: the pieces of
patterns to play, with every repeat already unrolled, and the length.
The audio thread could work the length out, but `Project` already does,
and two copies of one calculation can drift apart. What a `Play` is, and
how the project builds them, is in [Sections, repeats and
loops](../program/project.md#sections-repeats-and-loops).

## Notes for every pattern: `ChannelNotes`

Each channel needs one list of notes per pattern. A `Vec` of `Vec`s does
that, but there is a catch. When a pattern is added, the audio thread
must add an empty list to every channel. `Vec::new()` itself is free: an
empty `Vec` has no memory of its own yet. But adding it to the *outer*
`Vec` allocates if the outer `Vec` is full. To be safe, the outer `Vec`
must have room for every pattern there could ever be, reserved before it
reaches the audio thread.

That is a rule about how the value was made, and Rust can enforce such
rules with a **newtype**, a struct with one private field:

```rust
pub struct ChannelNotes(Vec<Vec<NoteEvent>>);

impl ChannelNotes {
    /// One list per pattern, in pattern order.
    pub fn new(lists: impl IntoIterator<Item = Vec<NoteEvent>>) -> Self {
        let mut patterns = Vec::with_capacity(MAX_PATTERNS);
        patterns.extend(lists.into_iter().take(MAX_PATTERNS));
        ChannelNotes(patterns)
    }

    /// Adds an empty list for a new pattern at position `pattern`, without
    /// allocating. The lists from there on move up one place.
    fn insert_empty(&mut self, pattern: usize) {
        if self.0.len() < self.0.capacity() {
            self.0.insert(pattern.min(self.0.len()), Vec::new());
        }
    }
}
```

Because the field is private, code outside `engine.rs` cannot write
`ChannelNotes(some_vec)`. The only way to make one is
`ChannelNotes::new`, which always reserves room for `MAX_PATTERNS`
lists. So every `ChannelNotes` the audio thread receives has the room,
and `insert_empty` can never allocate. The check `len < capacity` makes
extra sure: if it were ever full, it would rather skip the insert than
allocate. `.0` is how you reach the field of a tuple struct (one whose
fields have no names). The test `channel_notes_have_room_for_every_pattern`
pins the promise down.

On the window's side, sending a channel builds one with an iterator,
one list per pattern:

```rust
// The notes as they are played, with the song's swing.
let notes = ChannelNotes::new(
    (0..self.project.patterns().len())
        .map(|pattern| self.project.played_notes(pattern, index).into_owned()),
);
```

`new` takes `impl IntoIterator<Item = Vec<NoteEvent>>`, so it accepts
this iterator, an array like `[steps(&[0]), steps(&[2])]` in the tests,
or a `Vec` of lists. The notes are `played_notes`, not the written ones:
with swing, the second eighth of each beat comes late ([Tempo, meter,
swing and feel](tempo.md#swing)).

## Patterns come and go

`RemovePattern` takes the pattern's list out of every channel. The lists
after it move down one place, exactly as `Project::remove_pattern`
renumbers the clips, so both sides keep agreeing about what "pattern 2"
means. Each removed list goes to the trash queue, to be freed on the
window's thread:

```rust
fn remove_pattern(&mut self, pattern: usize) {
    if pattern < self.pattern_lengths.len() {
        self.pattern_lengths.remove(pattern);
        for index in 0..self.channels.len() {
            if let Some(notes) = self.channels[index].notes.remove(pattern) {
                self.throw_away(Garbage::Notes(notes));
            }
        }
        self.update_loop_length();
        ...
    }
}
```

The loop counts with an index instead of `for channel in &mut
self.channels`, because the body calls `self.throw_away`, which needs
`&mut self`. A `for` loop over `&mut self.channels` would hold a mutable
borrow of part of `self` for the whole loop, and `self.throw_away` would
need another. With an index, each `self.channels[index]` borrow ends on
the same line.

`Clear` empties everything before a new project is sent, when a file is
opened:

```rust
while let Some(mut channel) = self.channels.pop() {
    silence(&mut channel.instrument, &mut self.midi_out, &mut self.midi_backlog);
    self.throw_away(Garbage::Channel(channel));
}
// `clear` empties the list but keeps its memory: nothing is freed.
self.pattern_lengths.clear();
let old = std::mem::take(&mut self.plays);
self.throw_away(Garbage::Song(old));
```

- `pop` takes the last item out of a `Vec`, or returns `None` when it is
  empty, so `while let Some(..)` runs until nothing is left.
- `silence` tells an external synth to let go of the notes a MIDI Out
  channel is holding, since a synth only stops a note when it is told
  to.
- `clear` on a `Vec<usize>` sets its length to zero but keeps its
  memory, so nothing is freed here, and later pushes do not allocate.
- `std::mem::take` takes the value out and leaves the type's default in
  its place, here an empty `Vec` (which owns no memory).

Opening a project throws away up to `MAX_CHANNELS` (32) channels, each
with its instrument and notes, and the song, its tempos and its
metronome clicks, which is why the trash queue has 1024 places.

## Starting the notes of a step

When a step fires, the audio thread starts the notes that begin on it.
In pattern mode that is one pattern. In song mode, a step of the song
can be inside several clips at once, one per track, and each clip has
its own idea of "which step":

```rust
PlayMode::Song => {
    // `self.plays` is read while `self.channels` is changed: two
    // different fields, so the borrows do not overlap.
    for play in &self.plays {
        let length = self.pattern_lengths.get(play.pattern).copied().unwrap_or(0);
        if let Some(local_step) = play.pattern_step(step, length) {
            // Notes stop at the end of the pattern, or of the
            // piece, whichever comes first.
            let room = (length - local_step).min(play.end() - step);
            start_pattern_notes(
                &mut self.channels,
                Origin {
                    pattern: play.pattern,
                    pattern_step: local_step,
                    song_step: step,
                    over_song: false,
                },
                room,
                timing,
                &mut self.pending,
                self.trace.as_deref_mut(),
            );
        }
    }
    ...
}
```

A clip that starts on song step 32 plays its pattern's step 2 on song
step 34. If two clips overlap (a Chords clip and a Beat clip, on two
tracks), both start their notes. A clip that repeats its pattern is one
long `Play`, and `pattern_step` takes the song step modulo the pattern
length, so the pattern comes round again and again inside it.

### Borrowing only what you need

`start_pattern_notes` is a plain function, not a method:

```rust
fn start_pattern_notes(
    channels: &mut [ChannelState],
    origin: Origin,
    room: usize,
    timing: Timing,
    pending: &mut Vec<Pending>,
    mut trace: Option<&mut Tracer>,
) { ... }
```

If it were a method taking `&mut self`, calling it inside
`for play in &self.plays` would not compile: the loop borrows
`self.plays` to read, and the call would borrow *all* of `self` to
change. As a function taking `&mut self.channels`, it borrows only that
field, which does not overlap with `self.plays`. Shaping a function's
arguments so it borrows only what it needs is a common way to work with
the borrow checker rather than against it.

`Origin` says where the notes come from: the pattern, the step of the
pattern, the step of the song, and whether they come from a pattern
played over the song (below). Every voice keeps the origin of the note
it plays, which is how the audio thread later knows whether the song
still plays it.

`Timing` bundles the numbers that always travel together:

```rust
/// How long a step is, the sample rate the instruments run at, how
/// loose the timing is, and the frame the step starts on.
#[derive(Debug, Clone, Copy)]
struct Timing {
    step_frames: f64,
    sample_rate: u32,
    loose_frames: f64,
    now: u64,
}
```

Without it, the function would take nine arguments, several of them
plain numbers, and a call with four numbers in a row is easy to get in
the wrong order. (Clippy's `too_many_arguments` lint complains from
eight, and readability suffers before that.)

### How long a note is held

A note's length is in steps; the instrument wants frames. The audio
thread works it out when the note starts, and calls it the **gate**:

```rust
let tick = timing.step_frames / TICKS_PER_STEP as f64;
let held = if note.length <= room {
    note.length as f64 * timing.step_frames - f64::from(note.nudge + note.trim) * tick
} else {
    room as f64 * timing.step_frames - f64::from(note.nudge) * tick
};
let gate = held as u32;
```

A note that runs past the end of its pattern, or of the piece of the
song that plays it, is cut there (`room`), so a pattern's last chord
does not hang over the next clip. `nudge` and `trim` are fractions of a
step, in ticks: a note from a MIDI performance can start a little after
its step and end a little before its last one. With loose timing, each
note also starts a little late on purpose. A note that starts late waits
in `pending`, a list reserved in advance, until its frame comes. Both
are in [Tempo, meter, swing and feel](tempo.md).

## When the song changes under a playing note

The song can change while it plays: a clip is moved or deleted, a track
is muted, the band writes new patterns. A note that already started
would ring on to the end of its gate, even though the song no longer
plays it. So when a new song arrives while playing, the audio thread
lets go of the notes it no longer plays:

```rust
fn release_notes_left_out(&mut self) {
    // The fields are borrowed one by one, so the closure can read the
    // song while the channels are changed.
    let plays = &self.plays;
    let lengths = &self.pattern_lengths;
    let still_played = |origin: Origin| {
        if origin.over_song {
            return true;
        }
        let length = lengths.get(origin.pattern).copied().unwrap_or(0);
        plays.iter().any(|play| {
            play.pattern == origin.pattern
                && play.pattern_step(origin.song_step, length) == Some(origin.pattern_step)
        })
    };
    for channel in &mut self.channels {
        channel.instrument.release_unless(still_played);
    }
    ...
}
```

A note is still played if some piece of the new song plays the same
step of the same pattern on the same song step. `release_unless` takes
the test as a closure (`impl Fn(Origin) -> bool`), so each kind of
instrument can let go in its own way: a synth voice goes to its release,
a sampler fades its sound out quickly, a MIDI Out sends a note-off.
Keys held on a keyboard and previews have no origin, so they are never
cut. `notes_stop_when_the_song_no_longer_plays_them` checks it.

The closure borrows `self.plays` and `self.pattern_lengths`, and the
loop borrows `self.channels` to change it. That is allowed because they
are different fields: since the 2021 edition, a closure that uses
`self.plays` captures only that field, not all of `self`, so writing
`self.plays` inside the closure would work as well. The two local
references only make plain which parts of `self` it reads.

## Seeking, and switching mode

`Command::Seek(step)` moves the start position. While stopped, that is
where the next `Play` starts. While playing, playback jumps there at
once:

```rust
fn seek(&mut self, step: usize) {
    self.start = step;
    self.pending.clear();
    ...
    if self.playing {
        // Notes that were playing would otherwise hang on
        // until the end they will now never reach.
        for channel in &mut self.channels {
            channel.instrument.release_all();
        }
        ...
        self.sequencer.start_at(step);
    }
}
```

Notes waiting in `pending` belong to the old position, so they are
dropped. Switching between pattern and song mode starts the new loop
from its top, and releases held notes for the same reason. Stopping
releases every note, keys held down included: each one goes into its
release, so the sound rings out a moment and does not hang on.

## A pattern over the song

The channel rack's **▶ Play over the song** plays the selected pattern
over the song as it plays, to try it out with the rest of the band
(`Command::SetOverlay`). It starts on the song's next bar line, so it
comes in in time, and loops for as long as it plays:

```rust
fn overlay_step(&mut self, step: usize) -> Option<(usize, usize)> {
    let bar_line = click_at(&self.song_clicks, step) == Click::Bar;
    let overlay = self.overlay.as_mut()?;
    // After a jump it waits for a bar line, to start again from its
    // top in time with the bars.
    if overlay.last.is_some_and(|last| step != last + 1)
        && let Some((pattern, _)) = overlay.playing.take()
    {
        overlay.coming.get_or_insert(pattern);
    }
    overlay.last = Some(step);
    if bar_line && let Some(pattern) = overlay.coming.take() {
        overlay.playing = Some((pattern, 0));
    }
    let (pattern, at) = overlay.playing?;
    let length = self.pattern_lengths.get(pattern).copied().unwrap_or(0);
    if length == 0 {
        return None;
    }
    let at = at % length;
    overlay.playing = Some((pattern, at + 1));
    Some((pattern, at))
}
```

The bar lines come from the song's metronome clicks, which already say
where each bar starts in every meter. A pattern chosen while another
plays over the song waits as `coming` until the next bar line. A jump
(a seek, or the song coming back round) is noticed because the step is
not the one after the last, and the pattern starts again from its top
on the next bar line.

The window draws a second playhead in the rack for the pattern played
over the song. Two numbers, the pattern and its step, must be read
together, or the window could see a new pattern with an old step. So
they share one `AtomicU64`, the pattern in the high 32 bits and the step
in the low 32:

```rust
fn store_overlay(overlay: &AtomicU64, playing: Option<(usize, usize)>) {
    let bits = playing.map_or(NO_POSITION, |(pattern, step)| {
        ((pattern as u64) << 32) | step as u64
    });
    overlay.store(bits, Ordering::Relaxed);
}
```

`<<` shifts the pattern's bits up 32 places, and `|` (bitwise or) puts
the step in the empty low half. Reading undoes it with `>> 32` and a
mask, `bits & u64::from(u32::MAX)`. One load, one store: an atomic
integer cannot be read half old and half new. A special value,
`NO_POSITION`, means "nothing plays over the song".

## Testing it

The tests run the audio thread without a sound card (see [Testing
without a sound card](../program/threads.md#testing-without-a-sound-card)).
A small helper finds where sounds start in a block of samples:

```rust
fn onsets(out: &[f32]) -> Vec<usize> {
    (0..out.len())
        .filter(|&i| out[i] != 0.0 && (i == 0 || out[i - 1] == 0.0))
        .collect()
}
```

The test click is a short burst after silence, so every hit is an
onset. Then a song can be checked in one line:

```rust
#[test]
fn song_mode_plays_the_clips() {
    let mut h = two_patterns();
    // Pattern 1 at bar 1, pattern 0 at bar 2: a two-bar song.
    let plays = vec![whole(1, 0), whole(0, 16)];
    send(&mut h, Command::SetSong { plays, length: 32 });
    send(&mut h, Command::SetMode(PlayMode::Song));
    send(&mut h, Command::Play);
    let out = render(&mut h, 40 * STEP);
    // Step 2 of the first clip, step 0 of the second, then the song
    // loops back to the start: step 2 of the first clip again.
    assert_eq!(onsets(&out), vec![2 * STEP, 16 * STEP, 34 * STEP]);
    assert_eq!(
        h.playhead.load(Ordering::Relaxed),
        7,
        "step 39 is step 7 of the second time round"
    );
}
```

The last assertion carries a message saying *why* 7 is right: after 40
steps of a 32-step song, the sequencer is on its second time round. A
reader who expects 39 sees the reason as soon as the test fails.

Other tests cover the rest of this chapter: clips that overlap, pieces
that repeat their pattern, patterns inserted and removed under the
others, `Clear` sending everything to the trash, `Play` starting where
a seek put it, and seven tests on a pattern played over the song.
