# Playing a song on the audio thread

File: [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

The audio thread keeps its own copy of everything it plays, as in steps 2
and 3, and the rules have not changed: no allocating, no freeing, no
locks. What is new is that there is more to copy: notes for every
pattern, the length of every pattern, and the song.

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

The sequencer from step 2 does not change at all. It still counts steps
from 0 to some length and loops. Only the length changes: the selected
pattern's length in pattern mode, the song's length in song mode.

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

## Notes for every pattern: `ChannelNotes`

Each channel now needs one list of notes per pattern. A `Vec` of `Vec`s
does that, but there is a catch. When the user adds a pattern, the audio
thread must add an empty list to every channel:

```rust
channel.notes.push(Vec::new());
```

`Vec::new()` itself is free: an empty `Vec` has no memory of its own
yet (Rust only asks for memory when the first item arrives). But `push`
onto the *outer* `Vec` allocates if the outer `Vec` is full. To be safe,
the outer `Vec` must have room for every pattern there could ever be,
reserved before it reaches the audio thread.

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

    /// Adds an empty list for a new pattern, without allocating.
    fn push_empty(&mut self) {
        if self.0.len() < self.0.capacity() {
            self.0.push(Vec::new());
        }
    }
}
```

Because the field is private, code outside `engine.rs` cannot write
`ChannelNotes(some_vec)`. The only way to make one is `ChannelNotes::new`,
which always reserves room for `MAX_PATTERNS` lists. So every
`ChannelNotes` the audio thread receives has the room, and `push_empty`
can never allocate. The check `len < capacity` makes extra sure: if it
were ever full, it would rather skip the push than allocate.

`.0` is how you reach the field of a tuple struct (one whose fields have
no names). A test pins the promise down:

```rust
#[test]
fn channel_notes_have_room_for_every_pattern() {
    let notes = ChannelNotes::new([Vec::new()]);
    assert!(notes.0.capacity() >= MAX_PATTERNS);
}
```

On the UI thread, sending a channel builds one with an iterator, one list
per pattern:

```rust
let notes = ChannelNotes::new(
    self.project
        .patterns()
        .iter()
        .map(|pattern| pattern.notes(index).to_vec()),
);
```

`new` takes `impl IntoIterator<Item = Vec<NoteEvent>>`, so it accepts this
iterator, an array like `[steps(&[0]), steps(&[2])]` in the tests, or a
`Vec` of lists.

## The new commands

```rust
SetMode(PlayMode),
SelectPattern(usize),
AddPattern { length: usize },
RemovePattern(usize),
SetPatternLength { pattern: usize, length: usize },
SetSong { clips: Vec<Clip>, length: usize },
SetNotes { pattern: usize, channel: usize, notes: Vec<NoteEvent> },
Clear,
```

`RemovePattern` takes the pattern's list out of every channel. The lists
after it move down one place, exactly as `Project::remove_pattern`
renumbers the clips, so both sides keep agreeing about what "pattern 2"
means. Each removed list goes to the trash queue, to be freed on the UI
thread:

```rust
Command::RemovePattern(pattern) => {
    if pattern < self.pattern_lengths.len() {
        self.pattern_lengths.remove(pattern);
        for index in 0..self.channels.len() {
            if let Some(notes) = self.channels[index].notes.remove(pattern) {
                self.throw_away(Garbage::Notes(notes));
            }
        }
        self.update_loop_length();
    }
}
```

The loop counts with an index instead of `for channel in &mut
self.channels`, because the body calls `self.throw_away`, which needs
`&mut self`. A `for` loop over `&mut self.channels` would hold a mutable
borrow of part of `self` for the whole loop, and `self.throw_away` would
need another. With an index, each `self.channels[index]` borrow ends on
the same line.

`SetSong` brings the song's length along with its clips. The audio
thread could compute it, but `Project::song_length` already does, and two
copies of the same calculation can drift apart. The UI sends a new song
whenever a clip moves, a track is muted, or a pattern's length changes.

`Clear` empties everything before a new project is sent, when a file is
opened:

```rust
while let Some(channel) = self.channels.pop() {
    self.throw_away(Garbage::Channel(channel));
}
self.pattern_lengths.clear();
let old = std::mem::take(&mut self.clips);
self.throw_away(Garbage::Song(old));
```

- `pop` takes the last item out of a `Vec`, or returns `None` when it is
  empty, so `while let Some(..)` runs until nothing is left.
- `clear` on a `Vec<usize>` sets its length to zero but keeps its memory,
  so nothing is freed here, and later pushes do not allocate.
- `std::mem::take` takes the value out and leaves the type's default in
  its place, here an empty `Vec` (which, again, owns no memory).

Loading a project throws away up to 32 channels, each with its
instrument and notes, so the trash queue grew from 256 places to 1024.

## Starting notes in song mode

When a step fires, the audio thread starts the notes that begin on it.
In song mode, a step of the song can be inside several clips at once, one
per track, and each clip has its own idea of "which step":

```rust
PlayMode::Song => {
    // `self.clips` is read while `self.channels` is changed: two
    // different fields, so the borrows do not overlap.
    for clip in &self.clips {
        let length = self.pattern_lengths.get(clip.pattern).copied().unwrap_or(0);
        if (clip.start..clip.start + length).contains(&step) {
            let local_step = step - clip.start;
            start_pattern_notes(&mut self.channels, clip.pattern, local_step, length, timing);
        }
    }
}
```

A clip that starts on song step 32 plays its pattern's step 2 on song
step 34. If two clips overlap (the Chords clip and a Beat clip, on two
tracks), both start their notes.

`start_pattern_notes` is a plain function, not a method:

```rust
fn start_pattern_notes(
    channels: &mut [ChannelState],
    pattern: usize,
    step: usize,
    length: usize,
    timing: Timing,
) { ... }
```

If it were a method taking `&mut self`, calling it inside
`for clip in &self.clips` would not compile: the loop borrows `self.clips`
to read, and the call would borrow *all* of `self` to change. As a
function taking `&mut self.channels`, it borrows only that field, which
does not overlap with `self.clips`. This is the same "disjoint fields"
rule from step 3, used on purpose: shape the function's arguments so it
borrows only what it needs.

`Timing` bundles two numbers that always travel together:

```rust
#[derive(Debug, Clone, Copy)]
struct Timing {
    step_frames: f64,
    sample_rate: u32,
}
```

Without it, the function would take six arguments, five of them plain
numbers, and a call with five numbers in a row is easy to get in the
wrong order. (Clippy's `too_many_arguments` lint only complains from
eight, but readability suffers well before that.)

## Testing it

The tests run the audio thread without a sound card, as before. A small
helper finds where sounds start in a block of samples:

```rust
fn onsets(out: &[f32]) -> Vec<usize> {
    (0..out.len())
        .filter(|&i| out[i] != 0.0 && (i == 0 || out[i - 1] == 0.0))
        .collect()
}
```

The test click is 10 frames long, so every hit is a short burst after
silence. Then a song can be checked in one line:

```rust
#[test]
fn song_mode_plays_the_clips() {
    let mut h = two_patterns();
    // Pattern 1 at bar 1, pattern 0 at bar 2: a two-bar song.
    let clips = vec![Clip::new(1, 0, 0), Clip::new(0, 0, 16)];
    send(&mut h, Command::SetSong { clips, length: 32 });
    send(&mut h, Command::SetMode(PlayMode::Song));
    send(&mut h, Command::Play);
    let out = render(&mut h, 40 * STEP);
    // Step 2 of the first clip, step 0 of the second, then the song
    // loops back to the start: step 2 of the first clip again.
    assert_eq!(onsets(&out), vec![2 * STEP, 16 * STEP, 34 * STEP]);
    assert_eq!(h.playhead.load(Ordering::Relaxed), 7, "step 39 is step 7 of the second time round");
}
```

The first version of this test expected the playhead at 39 and failed:
after 40 steps of a 32-step song, the sequencer is on its second time
round. The test was wrong, not the code, and the fix was to say why in
the assertion's message. A failing test is a question, not always a bug.

## Checking it by ear, without ears

As in step 3, the real app's output was recorded through a "file" sound
device and measured bar by bar. Bars 1 and 2 had almost no energy above
6 kHz (only the synth), bars 3 to 7 had about four times as much (hats
and claps arrived), bar 8 had hits on steps 12 to 15 (the snare roll),
and bar 9 sounded like bar 1 again: the song looped.
