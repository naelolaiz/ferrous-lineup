# Recording: from a key to a note

Files: [`crates/lineup-core/src/record.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/record.rs),
[`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs)
and [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)

The [last chapter](live.md) brought each key to the UI thread with the
position it was played at. Recording is what the UI does with it: a key
went down at one position and came up at another, and between them is
a note.

## When keys are recorded

The **● Rec** button only *arms* recording. Keys are recorded while
three things are true at once:

```rust
pub(super) fn is_recording(&self) -> bool {
    self.live.recording && self.playing && self.mode == PlayMode::Pattern
}
```

Armed, playing, and in pattern mode, because a take goes into the
pattern that loops. Arming switches to pattern mode for you
(`toggle_recording`), so pressing Rec and then Space is enough.

## Remembering keys that are down

Every key that is down, from either keyboard, is kept in a list of
`Held`:

```rust
struct Held {
    pitch: Note,
    velocity: u8,
    channel: usize,
    recorded_from: Option<f64>,
}
```

When a key goes down, `key_played` adds one:

```rust
self.live.held.retain(|held| held.pitch != pitch);
let recorded_from = played.position.filter(|_| self.is_recording());
self.live.held.push(Held {
    pitch,
    velocity,
    channel: self.piano_roll.channel(),
    recorded_from,
});
```

The same list feeds the chord name in the status bar and the keys lit
up in the piano roll; recording is only one of its uses. `retain` first
removes the same pitch if it was already down (on the other keyboard,
say), so each pitch is in the list once.

`Option::filter` keeps a `Some` only if the closure says yes, and turns
it into `None` otherwise. So `recorded_from` is the position when the
key is being recorded, and `None` when it is just being played. The
closure ignores its argument (`|_|`): the question is not about the
position, but about whether recording is on.

The `channel` is remembered for the same reason as `live_keys` on the
audio thread: the note goes into the channel the key played, even if
another one is selected before the key comes up.

When the key comes up, its `Held` is taken out of the list, and if it
has a start position and there is an end position, it becomes a note:

```rust
let held = self.live.held.remove(index);
if let (Some(from), Some(until)) = (held.recorded_from, played.position) {
    self.record(held, from, until);
}
```

Matching a tuple of two `Option`s checks both at once: the block runs
only if both are `Some`.

## Quantizing: `recorded_note`

Nobody plays exactly on the step. A key that went down at 3.9 and came
up at 6.2 was meant as a note from step 4 to step 6. Moving the ends of
a note to the grid is called **quantizing**, and it is a plain function
in `lineup-core`, with no keyboard and no UI in sight:

```rust
pub fn recorded_note(pitch: Note, velocity: u8, from: f64, until: f64, length: usize) -> NoteEvent {
    let length = length.max(1);
    let until = if until < from {
        until + length as f64
    } else {
        until
    };
    let start = from.round();
    let end = until.round().max(start + 1.0);
    let start_step = start as usize % length;
    let steps = ((end - start) as usize).min(length - start_step);
    NoteEvent { pitch, start: start_step, length: steps, velocity }
}
```

Line by line:

1. `length.max(1)` guards the `%` below: a remainder by zero panics. A
   pattern is never empty, but a function should not depend on its
   caller for that.
2. The pattern loops. If the key came up *before* where it went down
   (down at 14.5, up at 1.5), the loop started again in between, and the
   end is really 17.5, one loop further on.
3. `round()` moves each end to the nearest step. `f64::round` rounds
   halves away from zero, so 2.5 becomes 3.
4. `.max(start + 1.0)`: a quick tap would round both ends to the same
   step, a note of length 0. A note is at least one step long.
5. A key pressed just before the loop starts again (at 15.7 in a
   16-step pattern) rounds to 16, which is step 0 of the next time
   round: `% length` folds it back to 0. That is where the player meant
   it.
6. A note cannot run past the end of the pattern, so it is cut there
   with `.min(length - start_step)`.

The tests read like those rules:

```rust
#[test]
fn ends_go_to_the_nearest_step() {
    assert_eq!(recorded(2.0, 4.0), (2, 2));
    assert_eq!(recorded(1.8, 4.3), (2, 2), "a little early, a little late");
    assert_eq!(recorded(2.4, 4.6), (2, 3));
}

#[test]
fn just_before_the_loop_is_step_zero() {
    assert_eq!(recorded(15.7, 0.4), (0, 1));
    assert_eq!(recorded(15.6, 1.9), (0, 2));
}

#[test]
fn a_note_across_the_loop_end_is_cut_there() {
    assert_eq!(recorded(14.0, 2.0), (14, 2));
}
```

`recorded(from, until)` is a helper in the test module that records a
C4 in a 16-step pattern and returns `(start, length)`. Being able to
write `15.7` instead of having to play a key at exactly that moment is
why the function lives in `lineup-core` and takes plain numbers.

### Floats into integers: `as`

`start as usize` turns an `f64` into a `usize`. For floats, `as` drops
the fraction (there is none left after `round`), and it **saturates**:
a negative number becomes 0, a number too big becomes `usize::MAX`, and
NaN becomes 0. It never panics and never wraps around. That is
different from `as` between integers, which keeps only the low bits:
`300_i32 as u8` is 44. For floats into integers, saturating is what you
almost always want, and Rust has done it since version 1.45.

### What it cannot know

The positions say *where in the loop* a key was, not *which time
round*. A key held for more than a whole loop looks the same as a short
one: down at 2, up at 3 one loop later reads as one step long. And a
note across the end of the loop is cut there, instead of going on at
step 0. Both are left as [exercise 4](exercises.md).

## Adding the note: one undo step per take

`record` adds the note to the selected pattern through `edit_pattern`,
like every other change since step 5, so it is undoable and the audio
thread hears about it:

```rust
fn record(&mut self, held: Held, from: f64, until: f64) {
    let Some(length) = self.project.pattern(self.pattern).map(Pattern::length) else {
        return;
    };
    let note = recorded_note(held.pitch, held.velocity, from, until, length);
    self.edit_pattern("record notes", |pattern| {
        pattern.add_note(held.channel, note);
    });
}
```

`.map(Pattern::length)` passes a method as the function to `map`:
`Pattern::length` is a function that takes a `&Pattern`, exactly what
`map` hands over. `add_note` skips a note with the same pitch and start
as one already there, so playing the same thing the next time round
does not stack notes on top of each other.

A take can be dozens of notes, and Ctrl+Z should undo all of them, not
one at a time. [Step 5](../step-5/history.md) already keeps a step open
while the mouse is held, so a drag is one step. Recording joins that
rule:

```rust
pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
    let holding = ctx.input(|input| input.pointer.any_down());
    if !holding && !ctx.text_edit_focused() && !self.is_recording() {
        self.history.close();
    }
}
```

While recording, the step stays open, and every note joins it. The take
ends, and the step with it, when playback stops, Rec is disarmed, or
the mode changes. Anything else you change during the take, such as a
step clicked in the rack, joins the same step.

## Ending a take: before Stop

When playback stops, some keys may still be down. They are recorded as
if they came up right then, by `finish_recording`:

```rust
pub(super) fn finish_recording(&mut self) {
    let until = self.position();
    for index in 0..self.live.held.len() {
        let held = self.live.held[index];
        if let (Some(from), Some(until)) = (held.recorded_from, until) {
            self.record(held, from, until);
        }
        self.live.held[index].recorded_from = None;
    }
}
```

The keys stay in the list (they are still down, and still sound), but
their `recorded_from` is cleared, so their key-up later records
nothing. The loop goes by index, not `for held in &self.live.held`,
because `record` needs `&mut self` while the list is borrowed. `Held`
is `Copy`, so `let held = self.live.held[index];` copies it out, and the
borrow ends on that line.

The order in `set_playing` matters:

```rust
fn set_playing(&mut self, playing: bool) {
    if !playing {
        // Before the audio thread forgets where it was.
        self.finish_recording();
    }
    self.playing = playing;
    self.send(if playing { Command::Play } else { Command::Stop });
}
```

`finish_recording` needs the position, and `position()` gives `None`
once `self.playing` is false, or once the audio thread has stopped and
stored `NO_POSITION`. So the take is finished first, while both still
say "playing", and only then is Stop sent.

## Latency: what you hear is in the past

When you press a key "on the beat", you press it on the beat you
*heard*. But sound takes time to reach you. The audio thread makes each
block ahead of time and hands it to the sound card, which plays it a
little later. On the computer this step was built on, blocks were 512
frames, and a sound took about 21 ms from the audio thread to the sound
card's output. So by the time you hear step 4 begin, the audio thread
is already about 21 ms into it, and a key pressed exactly then is
stamped 21 ms late.

At 120 BPM a step lasts 125 ms, so 21 ms is a sixth of a step, and
rounding to the nearest step hides it, unless you were already a
little late. A MIDI keyboard adds almost nothing on top: its keys are
stamped on midir's thread as they arrive.

The typing keyboard adds more. egui reads the keyboard once per frame,
so a typed key can wait up to a frame before it is stamped: 17 ms at 60
frames a second. The virtual machine this step was tested on drew its
frames slowly, with no graphics card, and the typed takes there landed
about 110 to 130 ms late. At 90 BPM that is most of a step, which is
why the Am in the [take shown in the overview](README.md) starts one
step late. On a normal computer it is far less.

The fix is to subtract the latency from each key's position before
rounding. cpal can tell how long it takes: each call of the audio
callback gets an `OutputCallbackInfo` with two timestamps, when the
callback ran and when its sound will be played. That is [exercise
7](exercises.md).

## Format 2: only when needed (step 7)

A MIDI Out channel is a new kind of channel, and an older Ferrous Lineup
that reads `MidiOut(channel: 4)` in a project file would not know what
it is. Step 4 planned for this: every file starts with its format
number, and a version that sees a number higher than its own refuses
the file with a clear message, instead of misreading it. So this step
raises the format to 2.

But most projects have no MIDI Out channel, and they are written in
exactly the same text as before. Marking them as format 2 would make
older versions refuse them for no reason. So the file says the lowest
format that can hold the project:

```rust
pub fn format_for(project: &Project) -> u32 {
    let uses_midi_out = project
        .channels()
        .iter()
        .any(|channel| matches!(channel.kind, ChannelKind::MidiOut { .. }));
    if uses_midi_out { 2 } else { 1 }
}
```

`matches!` with `MidiOut { .. }` asks only "is it this variant?", and
`..` ignores the fields. `any` stops at the first match. The test
checks both sides, and that a format 2 file reads back as the same
project:

```rust
#[test]
fn midi_out_channels_need_format_two() {
    let mut project = Project::template();
    assert!(to_text(&project).unwrap().contains("format: 1,"));

    project.add_channel(Channel::midi_out(4)).unwrap();
    let text = to_text(&project).unwrap();
    assert!(text.contains("format: 2,"), "{text}");
    ...
    assert_eq!(from_text(&text).unwrap(), project);
}
```

In step 7, `FORMAT_VERSION` became 2, and that was what the reader
accepted: it read formats 1 and 2, and refused 3. The test for files from
the future moved from 2 to 3 for that reason.

**Today.** `FORMAT_VERSION` is 27, and `format_for` checks many more
features than a MIDI Out channel: from step 8's format 3 (nine new drums,
patterns longer than four bars, and time signatures) up to format 27
(sending a built-in drum over MIDI). The rule has not changed: write the
lowest format that can hold the project. A project whose only new feature
is a MIDI Out channel is still written as format 2.
