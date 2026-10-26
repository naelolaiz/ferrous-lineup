# A playback trace: every note the player plays, to read

Files: [`crates/lineup-audio/src/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/trace.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/app/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/trace.rs)
and [`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)

Until now, the only way to be sure how a song really plays was to export
it and listen, or to look for the notes in the sound itself. The re-meter
fix of step 8 showed why the screen is not enough: the bars looked right
in 3/4, but the song sounded exactly as before, because squeezing the
notes and slowing the tempo cancelled each other out. Only the sound
told the truth.

A **trace** is a list of every note the player plays, written down by
the audio thread at the moment it plays it. It is not worked out again
from the project: it is what the speakers get, after swing, tempo
changes, nudges and loose timing, with the song unrolled the way the
playlist plays it and the muted tracks left out. So a trace can be read
instead of listened to, by you or by a test.

## Trying it

From a terminal, with a song file or the name of a demo:

```sh
ferrous-lineup --trace song.lineup --out song.tsv
ferrous-lineup --trace "Twelve-bar blues"
ferrous-lineup --demos
```

`--trace` plays the song the way **File > Export WAV** does, from start
to end, but without a window, a sound card, or a sound file: you get the
notes of the export, frame for frame, in a second or two. `--out` writes
them to a file; without it they go to the terminal. `--demos` lists the
names `--trace` understands. Capitals do not matter, and part of a name
is enough when only one demo has it.

In the app, **Settings > Playback > Trace** has a box, **Write a trace
while playing**. While it is ticked, every playback, from Play to Stop,
adds a table to the file named under it, in the folder the app keeps its
settings in. It starts unticked each time the app opens, so a trace you
forgot about never fills your disk.

On Windows, use `--out`: the ready-built program is a windowed program
and has no terminal to print to.

## Reading the table

The lines starting with `#` say what the table holds, then list the tempo
and the time signature of every part of the song:

```text
# Ferrous Lineup playback trace: Für Elise
# sample rate: 44100 Hz
# tempo: 0.0000 s, step 0, bar 1: 144.00 BPM
# tempo: 123.7500 s, step 1188, bar 100: 216.00 BPM
# tempo: 130.0000 s, step 1278, bar 105: 144.00 BPM
# time: step 0, bar 1: 3/4
# time: step 1188, bar 100: 9/8
# time: step 1278, bar 105: 3/4
```

Then comes one line for each note, in the order they start, with a tab
between columns. These are the first notes of the bass and the hi-hat
in the **Twelve-bar blues** demo, with a few columns left out:

```text
start   length  bar beat into_beat step   steps bpm    name pitch note velocity late_ms end
0.0000  0.4000  1   1    0.000     0.000  2.667 100.00 Bass 36    C2   104      0.0     gate
0.0000  0.1500  1   1    0.000     0.000  1.000 100.00 Hat  60    C4   96       0.0     gate
0.4000  0.0500  1   1    2.667     2.667  0.333 100.00 Hat  60    C4   96       100.0   gate
0.4000  0.2000  1   1    2.667     2.667  1.333 100.00 Bass 40    E2   90       100.0   gate
0.6000  0.4000  1   2    0.000     4.000  2.667 100.00 Bass 43    G2   96       0.0     gate
```

The swing is right there. At 100 BPM a beat lasts 0.6 seconds and has
four steps. Written down, the second eighth of the beat is on step 2,
halfway. Played, it comes at 2.667 steps into the beat, 100 ms late: two
thirds of the way, the third note of a triplet, which is what full swing
means. And the bass note before it lasts 2.667 steps instead of 2, long
before short.

| Column | What it says |
|---|---|
| `start`, `length` | When the note starts and how long it is held, in seconds from the start of playback |
| `bar`, `beat` | Its bar and its beat, counted from 1, in the time signature in force |
| `into_beat` | How many steps after the start of its beat it starts |
| `step`, `steps` | The step it starts on (counted from 0, with the fraction it is late by) and how many steps it is held |
| `bpm`, `time` | The tempo of its step and the time signature of its bar |
| `channel`, `name`, `program`, `instrument` | Its row in the channel rack (from 1), the channel's name, its General MIDI program and what plays it |
| `pitch`, `note` | The MIDI note number and its name, spelled in the key of the moment |
| `velocity` | How hard it is played, 1 to 127 |
| `late_ms` | How long after its step it starts: its nudge, the swing and loose timing |
| `end` | Why it ended: `gate` when its length was over, `cut` when it was stopped early (Stop, a jump, its track muted, its clip taken away), `again` when the same note started again on a MIDI Out or a SoundFont, which play one of each note at a time |
| `pattern` | Its pattern and the step of the pattern it comes from |

Tables like this are made for `grep` and `awk`. The bass notes of bar
12, or every note louder than 110:

```sh
ferrous-lineup --trace song.lineup | awk -F'\t' '$11 == "Bass" && $3 == 12'
ferrous-lineup --trace song.lineup | awk -F'\t' '!/^#/ && $16+0 > 110'
```

A spreadsheet opens the file too: tell it the columns are separated by
tabs, and that lines starting with `#` are comments.

## What a trace does not tell

A trace follows the notes, not the sound. A drum sample plays to its end
whatever the note's length, a synth's release fades on after the note
ends, and when every voice of the Lineup Osc is busy the oldest one is
taken for the new note. None of that is in the trace. For a problem
inside an instrument, listening (or looking at the samples) is still the
way.

## Notes that must not get lost

A trace may lose events on purpose: if the window falls behind, the ring
buffer fills, and the table's header says how many events were lost,
because a trace is a report and not the music. The queues that carry the
music are treated differently. Commands wait for room (`send_wait`); a
MIDI message that finds the output queue full waits in a backlog for the
next block; and a note-off is never thrown away, so an external synth
cannot be left with a stuck note. A recording longer than ten minutes is
refused when it is imported, rather than filling memory.
[Recovery](recovery.md) covers what the app does when something does go
wrong.

## The code

### Writing on the audio thread

The audio thread has strict rules, which step 2 explained: it must never
wait, never allocate memory, never write to a file. Writing a trace is
exactly what it may not do. So it does what it already does for MIDI
Out: it hands small messages to a ring buffer from the `rtrb` crate, and
another thread, which may take its time, reads them.

The messages are a `Copy` enum, so pushing one copies a few bytes and
allocates nothing:

```rust
pub enum TraceEvent {
    Play { frame: u64, step: usize, song: bool, pattern: usize },
    Jump { frame: u64, step: usize },
    Stop { frame: u64 },
    Step { frame: u64, step: usize, tempo: Tempo },
    NoteOn { frame: u64, id: u32, channel: usize, origin: Origin,
             pitch: Note, velocity: u8, gate: u32, late: u32 },
    NoteOff { frame: u64, id: u32, end: End },
}
```

`frame` is the audio thread's own clock: the frames it has played since
it started. Every note gets an `id`, so its `NoteOff` can be paired with
its `NoteOn` later.

The tracer lives in the audio thread as an `Option<Box<Tracer>>`. When
the trace is off it is `None`, and every place that traces is an
`if let Some(trace) = &mut self.trace`, which costs next to nothing. It
is made on the UI thread, where its memory is reserved, and travels to
the audio thread inside a command, the same way an instrument does:

```rust
let (tracer, reader) = trace_queue();
self.send(Command::SetTrace(Some(Box::new(tracer))));
```

An old tracer goes back to the UI thread as garbage, to be freed there.

If the ring buffer is full, because the window did not read it for a
long time, the event is not pushed and a counter of lost events goes up.
The trace says so in its header rather than silently missing notes.

### Where the notes start

A note of the song starts in one of two places: `start_pattern_notes`,
when its step comes, or `start_pending`, a few milliseconds later for a
note that is nudged or loosely timed. Both now tell the tracer, on the
frame the note really starts, which is why a swung note shows its 100 ms.

### How long a note lasts

When a note starts, its length is turned into a number of frames, the
gate. If the tempo changes while it sounds, every instrument stretches
the gates of its notes with the same function, `stretched`, so the note
still ends on its step. The tracer keeps its own list of the notes
sounding and stretches them with that very function. It also hears
everything that ends notes early: Stop, a seek, a mute, a song sent
while playing that no longer has a clip, a channel taken away.

The song-changed case shows a small Rust detail. The audio thread checks
each sounding note with a closure, `still_played`, that borrows the song.
The instruments and the tracer both need it:

```rust
for channel in &mut self.channels {
    channel.instrument.release_unless(still_played);
}
if let Some(trace) = &mut self.trace {
    trace.cut(self.now, None, still_played);
}
```

`still_played` is passed by value twice. That works because a closure
that only captures references is `Copy`, like the references themselves:
passing it hands over a copy and the original stays usable.

To find the notes whose length is over, the tracer would have to look at
every one of them on every frame, 44 100 times a second. Instead it
remembers the earliest end, `next_end`, and does nothing until that
frame comes.

### Turning events into notes

`Trace::notes` reads the events after the fact, on an ordinary thread.
It splits them into playbacks at each `Play`, and keeps the `Step`
events of each one in a list, in the order they were played. To find
where a frame is, it looks for the last step that started before it,
with `partition_point`:

```rust
let index = self.steps.partition_point(|played| played.frame <= frame);
```

`partition_point` is a binary search: given a sorted list and a test
that is true for a first part of it and false for the rest, it returns
where the false part starts. Here, the first step that started after
`frame`; the one before it is the step being played. The fraction is
how far `frame` is between that step and the next.

Each step also has a count of the steps played before it, which goes on
growing when a pattern loops or the playhead jumps. A note's length in
steps is the difference of those counts: it is measured along the steps
really played, whatever numbers the loop gives them.

### The command line

`main` used to take its first argument as a file to open. Now it looks at
it first:

```rust
match first.as_ref().and_then(|arg| arg.to_str()) {
    Some("--trace") => std::process::exit(trace(args)),
    Some("--demos") => { /* print the names */ return Ok(()); }
    _ => {}
}
```

`trace` builds the whole app without a window or a sound card, the way
the tests do (`Audio::Unavailable`), opens the file or demo, and asks for
the same song render the WAV export makes, with a trace switched on. It
then plays all the frames and throws the sound away. The trace is the
export's, because it is the same code path.

## Tests that read instead of listen

The tests use traces the way a person would. Among them:

- Every note the project holds, clip by clip, is in the trace exactly
  once, and all of them end when their length is over.
- With swing, the only notes that start late are the second eighths of
  each beat.
- A tempo marker at step 16 changes when the notes after it start, in
  seconds, but not their steps.
- A song re-metered from 4/4 to 3/4 starts every bar at the same second
  as before: the bug that only the sound could show is now a test.

## Exercises

1. Trace a demo with a ritardando (a tempo marker that glides) and read
   the `# tempo` lines: how many are there, and why one per step?
2. Turn **Loose timing** up in a song, trace it, and use `awk` to find the
   latest note. Is it ever later than the setting?
3. With the trace on in the app, play a pattern, mute its channel in the
   middle of a long note, and stop. Find the note in the file: what does
   its `end` column say?
4. The tracer looks for the notes whose length is over at the very top
   of each frame, before the tempo can change on that frame. What would
   go wrong with a note that ends on the same frame as a tempo change,
   if it looked afterwards?
