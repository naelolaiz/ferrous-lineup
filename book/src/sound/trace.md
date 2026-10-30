# The playback trace

Files: [`crates/lineup-audio/src/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/trace.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/app/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/trace.rs)
and [`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)

The screen shows the project; it does not show what the player did with
it. A song put into another time signature can look right, every bar
redrawn in 3/4, and still play its bars at the old pace, because the
notes were squeezed and the tempo slowed down, and the two cancel out.
Only the sound tells the truth. But listening is slow, and it says that
something is wrong, not what.

A **trace** is a list of every note the player plays, written down by
the audio thread at the moment it plays it. It is not worked out again
from the project: it is what the speakers get, after swing, tempo
changes, nudges and loose timing, with the song unrolled the way the
playlist plays it and the muted tracks left out. So a trace can be read
instead of listened to, by you or by a test. Many of the program's tests
read traces, and so does anyone checking that a demo plays what its
score says.

## Trying it

From a terminal, with a song file or the name of a demo:

```sh
ferrous-lineup --trace song.lineup --out song.tsv
ferrous-lineup --trace "Twelve-bar blues"
ferrous-lineup --demos
```

`--trace` plays the song the way **File > Export WAV…** does, from start
to end, but without a window, a sound card or a sound file: you get the
notes of the export, frame for frame, in a second or a few. `--out`
writes them to a file; without it they go to the terminal. `--rate`
plays at another sample rate than the export's 44,100. `--demos` lists
the names `--trace` understands. Capitals do not matter, and part of a
name is enough when only one demo has it.

On Windows, use `--out`: the ready-built program is a windowed program
and has no terminal to print to.

In the app, **Edit > Settings… > Playback > Trace** has a box, **Write a
trace while playing**. While it is ticked, every playback, from Play to
Stop, adds a table to `playback-trace.tsv`, in the folder the app keeps
its settings in; the setting shows the full path. It starts unticked
each time the app opens, so a trace you forgot about never fills your
disk. The box is greyed out when no sound card is open, since then
nothing plays. The manual suggests attaching this file when reporting a
problem with playback.

## Reading the table

The lines starting with `#` say what the table holds, then list the tempo
and the time signature of every part of the song. These are the first
ones of Für Elise:

```text
# Ferrous Lineup playback trace: Für Elise
# Every note the player started, in the order they start. Times are in seconds from the start of playback;
# steps are sixteenth notes of the loop played (the song, or the pattern in pattern mode), counted from 0.
# sample rate: 44100 Hz
# tempo: 0.0000 s, step 0, bar 1: 144.00 BPM
# tempo: 123.7500 s, step 1188, bar 100: 216.00 BPM
# tempo: 130.0000 s, step 1278, bar 105: 144.00 BPM
# tempo: 156.2500 s, step 1530, bar 126: 142.80 BPM
# tempo: 156.3551 s, step 1531, bar 126: 141.60 BPM
...
# time: step 0, bar 1: 3/4
# time: step 1188, bar 100: 9/8
# time: step 1278, bar 105: 3/4
```

The tempo changes as written at bars 100 and 105, then glides down
from bar 126: a ritardando, which the player plays one step at a time,
each a little slower, so the trace has a line for each step of it.

Then comes one line for each note, in the order they start, with a tab
between columns. These are the first notes of the bass and the hi-hat
in the **Twelve-bar blues** demo, with some columns left out:

```text
start   length  bar beat into_beat step   steps bpm    name pitch note velocity late_ms end
0.0000  0.4000  1   1    0.000     0.000  2.667 100.00 Bass 36    C2   104      0.0     gate
0.0000  0.1500  1   1    0.000     0.000  1.000 100.00 Hat  60    C4   96       0.0     gate
0.4000  0.0500  1   1    2.667     2.667  0.333 100.00 Hat  60    C4   96       100.0   gate
0.4000  0.2000  1   1    2.667     2.667  1.333 100.00 Bass 40    E2   90       100.0   gate
0.6000  0.4000  1   2    0.000     4.000  2.667 100.00 Bass 43    G2   96       0.0     gate
0.6000  0.1500  1   2    0.000     4.000  1.000 100.00 Hat  60    C4   96       0.0     gate
```

The swing is right there. At 100 BPM a beat lasts 0.6 seconds and has
four steps. Written down, the second eighth of the beat is on step 2,
halfway. Played, it comes 2.667 steps into the beat, 100 ms late: two
thirds of the way, the third note of a triplet, which is what full swing
means. And the bass note before it lasts 2.667 steps instead of 2, long
before short. (How swing moves the notes is in
[Swing](tempo.md#swing).)

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
tabs, and that lines starting with `#` are comments. The table is
always in English and names notes with letters, whatever the window
shows, so a trace reads the same for everyone.

## What a trace does not tell

A trace follows the notes, not the sound. A drum sample plays to its end
whatever the note's length, a synth's release fades on after the note
ends, and when every voice of the Lineup Osc is busy the oldest one is
taken for the new note. None of that is in the trace. For a problem
inside an instrument, listening (or looking at the samples) is still the
way.

## Writing on the audio thread

The audio thread has [strict rules](../program/threads.md#the-golden-rule-of-the-audio-thread):
it must never wait, never allocate memory, never write to a file.
Writing a trace is exactly what it may not do. So it does what it
already does for MIDI Out: it hands small messages to a ring buffer, and
another thread, which may take its time, reads them.

The messages are a `Copy` enum, so pushing one copies a few bytes and
allocates nothing:

```rust
pub enum TraceEvent {
    /// Playback started on `step`: of the song, or of pattern `pattern`
    /// when `song` is false. After a count-in, when the count is over.
    Play {
        frame: u64,
        step: usize,
        song: bool,
        pattern: usize,
    },
    /// Playback jumped to `step` while playing.
    Jump { frame: u64, step: usize },
    /// Playback stopped.
    Stop { frame: u64 },
    /// A step started, at `tempo`.
    Step {
        frame: u64,
        step: usize,
        tempo: Tempo,
    },
    /// A note started on channel `channel` (its row in the channel rack),
    /// `late` frames after its step (nudge and loose timing), to be held
    /// for `gate` frames. `id` pairs it with its [`TraceEvent::NoteOff`].
    NoteOn {
        frame: u64,
        id: u32,
        channel: usize,
        origin: Origin,
        pitch: Note,
        velocity: u8,
        gate: u32,
        late: u32,
    },
    /// The note `id` ended, as `end` says.
    NoteOff { frame: u64, id: u32, end: End },
}
```

`frame` is the audio thread's own clock: the frames it has played since
it started. Every note gets an `id`, so its `NoteOff` can be paired with
its `NoteOn` later. The ids count up with `wrapping_add(1)`, which goes
back to 0 after `u32::MAX` instead of panicking in a debug build; four
billion notes later, the first ones are long finished.

The audio thread keeps the tracer as an `Option<Box<Tracer>>`. When the
trace is off it is `None`, and every place that traces is an
`if let Some(trace) = &mut self.trace`, which costs next to nothing. The
tracer is made on another thread, where its memory can be reserved:

```rust
/// A tracer for the audio thread and the reader of what it writes.
pub fn trace_queue() -> (Tracer, TraceReader) {
    let (events, consumer) = rtrb::RingBuffer::new(TRACE_QUEUE_SIZE);
    let lost = Arc::new(AtomicUsize::new(0));
    let tracer = Tracer {
        events,
        lost: Arc::clone(&lost),
        open: Vec::with_capacity(OPEN_NOTES),
        next_id: 0,
        next_end: u64::MAX,
    };
    let reader = TraceReader {
        events: consumer,
        lost,
    };
    (tracer, reader)
}
```

and travels to the audio thread inside a command, the same way an
instrument does: `Command::SetTrace(Some(Box::new(tracer)))`. The queue
holds 65,536 events (`1 << 16`), and the list of notes sounding is
reserved for 4,096, so nothing grows on the audio thread. An old tracer
goes back to the UI thread as garbage, to be freed there.

### Lost on purpose

If the ring buffer is full, because nobody read it for a long time, the
event is not pushed and a counter of lost events goes up:

```rust
fn push(&mut self, event: TraceEvent) {
    if self.events.push(event).is_err() {
        self.lost.fetch_add(1, Ordering::Relaxed);
    }
}
```

The table's header then says how many events were lost, rather than
silently missing notes. The queues that carry the *music* do the
opposite: a MIDI note-off that finds its queue full waits for room,
because a lost one would leave a note sounding for ever on a synth (see
[When the sound card goes away](../program/threads.md#when-the-sound-card-goes-away)).
A trace is a report, not the music. Making the audio thread wait for it
would put the report before the sound.

### Where the notes start

A note of the song starts in one of two places: in
[`start_pattern_notes`](sequencer.md), when its step comes,
or in `start_pending`, a few milliseconds later, for a note that is
nudged or loosely timed. Both tell the tracer on the frame the note
really starts, which is why a swung note shows its 100 ms:

```rust
if let Some(trace) = &mut self.trace {
    trace.note_on(
        self.now,
        note.channel,
        note.origin,
        note.pitch,
        note.velocity,
        note.gate,
        note.late,
        one_per_pitch(&channel.instrument),
    );
}
```

`one_per_pitch` says whether the instrument is a MIDI Out or a
SoundFont, which play one of each note at a time: on those, a note that
starts again ends the one sounding, and the trace writes `again` for it.

### How long a note lasts

When a note starts, its length is turned into a number of frames, the
*gate*. If the tempo changes while it sounds, every instrument
stretches the gates of its notes with the same function, `stretched`,
so the note still ends on its step (see
[Every step at its own tempo](tempo.md#every-step-at-its-own-tempo)).
The tracer keeps its own list of the notes sounding, `open`, and
stretches them with that very function:

```rust
pub(crate) fn tempo_changed(&mut self, now: u64, next_step: f64, stretch: f64) {
    for note in &mut self.open {
        if note.end != u64::MAX {
            let left = note.end.saturating_sub(now) as f64;
            note.end = now + stretched(left, next_step, stretch) as u64;
        }
    }
    self.find_next_end();
}
```

A note with the gate that never runs out, `HELD` (`u32::MAX`), has no
end the tracer could know, which it keeps as `u64::MAX`.

The tracer also hears everything that ends notes early: Stop, a jump, a
mute, a song sent while playing that no longer has a clip, a channel
taken away. The song-changed case shows a small Rust detail. The audio
thread checks each sounding note with a closure, `still_played`, that
borrows the song. The instruments and the tracer both need it:

```rust
for channel in &mut self.channels {
    channel.instrument.release_unless(still_played);
}
if let Some(trace) = &mut self.trace {
    trace.cut(self.now, None, still_played);
}
```

`still_played` is passed by value, once for each channel and then once
more. That works because a closure that only captures shared references
is `Copy`, like the references themselves: passing it hands over a copy
and the original stays usable.

### Only when a note ends

To find the notes whose length is over, the tracer would have to look
at every one of them on every frame, 44,100 times a second. Instead it
remembers the earliest end, `next_end`, and does nothing until that
frame comes:

```rust
/// Ends the notes whose length is over by frame `now`. Called once
/// a frame, before anything else happens on it.
pub(crate) fn frame(&mut self, now: u64) {
    if now < self.next_end {
        return;
    }
    // The fields are borrowed one by one: `open` to go through, the
    // queue and the counter to write to.
    let events = &mut self.events;
    let lost = &self.lost;
    self.open.retain(|note| {
        if note.end > now {
            return true;
        }
        let event = TraceEvent::NoteOff {
            frame: note.end,
            id: note.id,
            end: End::Gate,
        };
        if events.push(event).is_err() {
            lost.fetch_add(1, Ordering::Relaxed);
        }
        false
    });
    self.find_next_end();
}
```

`retain` keeps the items for which the closure returns `true` and drops
the others, in one pass and in place. The closure needs to push to the
queue while `retain` holds `self.open` mutably. Calling `self.push`
inside it would borrow all of `self` a second time, which the compiler
refuses. Borrowing the two other fields into local variables first
(`events`, `lost`) makes three borrows of three different fields, which
do not overlap.

## Turning events into notes

`Trace::notes` reads the events after the fact, on an ordinary thread.
It splits them into playbacks (*runs*) at each `Play`, and keeps the
`Step` events of each one in a list, in the order they were played. To
find where a frame is, it looks for the last step that started before
it, with `partition_point`:

```rust
fn position(&self, frame: u64, sample_rate: u32) -> Option<(f64, f64, Tempo)> {
    let index = self.steps.partition_point(|played| played.frame <= frame);
    let played = self.steps.get(index.checked_sub(1)?)?;
    let length = match self.steps.get(index) {
        Some(next) => (next.frame - played.frame) as f64,
        None => frames_per_step(played.tempo, sample_rate),
    };
    let fraction = (frame - played.frame) as f64 / length.max(1.0);
    Some((
        played.step as f64 + fraction,
        played.count as f64 + fraction,
        played.tempo,
    ))
}
```

`partition_point` is a binary search: given a sorted list and a test
that is true for a first part of it and false for the rest, it returns
where the false part starts. Here, that is the first step that started
after `frame`; the one before it is the step being played. The fraction
is how far `frame` is between that step and the next.
`index.checked_sub(1)?` returns `None` for a frame before the first
step, where `index - 1` would underflow.

Each step also has a `count` of the steps played before it, which goes
on growing when a pattern loops or the playhead jumps. A note's length
in steps is the difference of those counts: it is measured along the
steps really played, whatever numbers the loop gives them. A note held
across the end of a one-bar loop, from step 14 to step 2 of the next
time round, lasts 4 steps, not -12.

A `NoteOff` finds its `NoteOn` by id in a small list of the notes still
open in the run. `swap_remove` takes it out by moving the last item into
its place, which is quick and fine here, since the order of that list
does not matter.

## Writing the table

`write_table` writes the header and one line per note. The names come
from `Labels`: the song's title, the project (for the channels' and
patterns' names), the time signatures and the keys, all in the steps of
the loop that was played:

```rust
/// For a trace of song mode playing `arrangement`.
pub fn song(title: &'a str, project: &'a Project, arrangement: &Arrangement) -> Self {
    Labels {
        title,
        project,
        meter: project.played_meter(arrangement),
        keys: project.played_keys(arrangement),
    }
}
```

`played_meter` and `played_keys` unroll the time signatures and keys
the same way the arrangement unrolls the clips, so a section in 3/4
that plays twice is in 3/4 both times. Each note is spelled in the key
of its step: an F♯ in G major, a G♭ in D♭ major.

A channel or pattern name could hold a tab or a line break, which would
split the table. `cell` replaces them with spaces:

```rust
/// Text for a table cell: no tabs or line breaks, which would split it.
fn cell(text: &str) -> String {
    text.replace(['\t', '\n', '\r'], " ")
}
```

`str::replace` takes a *pattern*, and an array of characters is one:
it matches any of them.

A note still sounding when the trace ended has no length; the table
writes `?` for it, and the header says how many there are.

## A trace while you play

The setting's trace is a `LiveTrace`, kept by the app. Once a frame,
`check_trace` moves what the audio thread handed over into it. Each
playback is written as its own table once it is over, at Stop, or when
another starts (switching between song and pattern mode while playing
starts another). `Trace::take_finished` takes the finished playbacks
out and leaves the one still going.

A table needs the project's names, but you can rename a channel while a
song plays. So at each `Play`, `check_trace` keeps a copy of the project
as it was, and the table is written with the names the playback had:

```rust
for event in &live.trace.events[before..] {
    if let TraceEvent::Play { song, pattern, .. } = *event {
        live.playing.push_back(Playing {
            title: self
                .file
                .song_name()
                .unwrap_or_else(|| "Untitled".to_string()),
            project: self.project.clone(),
            song,
            pattern,
        });
    }
}
```

A `VecDeque` is a queue that can add at one end and take from the
other: copies go in at the back as playbacks start, and come out at the
front as their tables are written.

The window shows note names in letters or solfège and words in English
or Spanish, but the table must not change with them. So `write` sets
both for as long as it writes, with the `with` functions of the
[per-thread settings](../program/languages.md#two-settings-kept-like-the-numerals-setting):

```rust
let mut table = Vec::new();
lineup_core::i18n::Language::English.with(|| {
    lineup_core::NoteNaming::Letters.with(|| write_table(&mut table, trace, &labels))
})?;
```

Unticking the box, or closing the app, drops the `LiveTrace`. Its
`Drop` writes the playback still going, so the last notes are not lost.
If the sound card goes away and comes back, `resume_trace` writes what
the old audio thread left and hands a new tracer to the new one, and the
trace goes on in the same file.

## The command line

`main` looks at its first argument before it opens a window (see
[`main.rs`](../program/egui.md#mainrs)). After `--trace`, a small loop
reads the rest:

```rust
while let Some(arg) = args.next() {
    match arg.to_str() {
        Some("--out" | "-o") => out = args.next().map(PathBuf::from),
        Some("--rate") => match args.next().and_then(|hz| hz.to_str()?.parse().ok()) {
            Some(hz) if (8_000..=192_000).contains(&hz) => rate = hz,
            _ => {
                eprintln!("--rate takes a sample rate in Hz, such as 44100");
                return 2;
            }
        },
        _ if source.is_none() => source = Some(arg),
        _ => {
            eprintln!("{USAGE}");
            return 2;
        }
    }
}
```

`args` is an iterator, and the loop takes items from it two ways: one
per turn with `while let`, and one more with `args.next()` inside the
body, for an option's value. The arms show three kinds of pattern: `"--out"
| "-o"` matches either text, `Some(hz) if ...` adds a guard, and `_ if
source.is_none()` matches anything, but only while no song was given.
The function returns the exit code the program ends with: 0 when the
trace was written, 1 when it could not be, 2 for a mistake in the
arguments. Scripts check it.

`trace_table` builds the whole app without a window or a sound card,
the way the tests do (`Audio::Unavailable`), opens the file or demo, and
asks for the same song render the WAV export makes, with a trace
switched on:

```rust
pub(super) fn trace_song(&mut self, sample_rate: u32) -> Trace {
    let mut render = self.song_render_traced(sample_rate);
    render.renderer.skip(render.frames);
    render.renderer.take_trace().unwrap_or_default()
}
```

`skip` plays all the frames and throws the sound away. The trace is the
export's, because it is the same code path (see
[Exporting](export.md)).

A demo name is matched in three tries: exactly, then ignoring capitals,
then as part of a name. The last try can find none, one or several, and
a slice pattern says which:

```rust
match matching.as_slice() {
    [demo] => Ok(*demo),
    [] => Err(format!(
        "There is no file or demo called \"{name}\" (--demos lists the demos)."
    )),
    several => {
        let names: Vec<&str> = several.iter().map(|demo| demo.name()).collect();
        Err(format!("\"{name}\" could be any of: {}.", names.join(", ")))
    }
}
```

## Tests that read instead of listen

The tests use traces the way a person would. In `lineup-audio`, on a
small renderer:

- `notes_start_on_their_step_and_last_their_length`, and
  `a_nudged_note_starts_late_by_its_nudge`.
- `a_slower_tempo_stretches_a_held_note`: a note that sounds across a
  change to half the tempo lasts its steps, in more seconds.
- `stopping_and_muting_cut_the_notes_sounding`, and
  `a_midi_out_ends_a_note_that_starts_again`.
- `a_looping_pattern_counts_its_steps_on`: lengths are measured along
  the steps played, across the loop.

And in the app, on whole songs:

- `the_trace_plays_every_note_of_the_export`: every note the project
  holds, clip by clip, is in the trace exactly once, and all of them end
  when their length is over.
- `swing_plays_the_off_beats_late`: with swing, the only notes that
  start late are the second eighths of each beat.
- `a_tempo_marker_changes_the_seconds_not_the_steps`: a tempo marker at
  step 16 changes when the notes after it start, in seconds, but not
  their steps.
- `re_metering_keeps_each_bar_as_long_in_seconds`: a song put from 4/4
  into 3/4 starts every bar at the same second as before. That is the
  bug from the start of this chapter, the one only the sound could show,
  written as a test.
- `a_trace_while_playing_writes_a_table_for_each_playback`.

Other chapters' tests use traces too, such as the one that takes El
Choclo through seven styles and checks that every melody note still
starts and lasts as it did ([Tempo, meter, swing and
feel](tempo.md#testing-it)).

Exercises on traces are in [Exercises: sound](exercises.md).
