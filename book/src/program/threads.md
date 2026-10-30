# Two threads: the window and the audio

Files: [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-audio/src/dsp.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs)
and [`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs)

The window and the sound live on different threads. The window thread
draws about sixty times a second and answers clicks and keys; the audio
thread fills the sound card's buffer every few milliseconds and must
never be late. How the two talk to each other is the most important
decision in the program, and it is where Rust's ownership rules start
to earn their keep.

```text
  UI thread (lineup-app)                         Audio thread (lineup-audio)
  ─────────────────────                        ─────────────────────────
  Project (lineup-core)       commands         AudioThread
   ├─ channels       ───────────────────▶       ├─ channels: Vec<ChannelState>
   ├─ patterns                                  │    └─ Instrument + notes
   └─ playlist                                  ├─ Sequencer
                                                │
  views             ◀── playhead, position ─────┤  (atomic numbers)
                                                │
  collect_garbage() ◀── trash queue ────────────┘  old values to free
```

## How digital audio works

Sound is a vibration. A sound card plays it by moving a speaker according
to a list of numbers called **samples**, typically 44,100 or 48,000 per
second (the **sample rate**). Each sample is a value between −1.0 and 1.0,
the speaker position at that instant. With two **channels** (stereo), the
samples are **interleaved**: left, right, left, right...

The sound card does not wait for us. It plays from a small buffer and,
every few milliseconds, asks the program to refill it by calling a
function we provide: the **audio callback**. It runs on a separate,
high-priority **audio thread**.

## The golden rule of the audio thread

If the callback is late, even once, the buffer runs empty and you hear a
click or a gap (an *underrun* or *xrun*). So code on the audio thread
must never do anything that can take an unpredictable amount of time:

- no **locks** (a `Mutex` held by the UI could block it),
- no **memory allocation or freeing** (`Vec::push` past its capacity,
  `String`, `Box::new`, or dropping the last owner of any of them),
- no **file, network or console I/O** (`println!` included),
- no waiting for other threads.

Everything else in this chapter follows from this rule.

## Sending commands with a ring buffer

The UI never changes audio state directly. It sends messages, values of
one enum with nearly forty variants. A few of them:

```rust
pub enum Command {
    Play,
    Seek(usize),
    Stop,
    SetTempo(Tempo),
    SetMasterVolume(f32),
    InsertChannel {
        index: usize,
        instrument: Instrument,
        notes: ChannelNotes,
        volume: f32,
        muted: bool,
    },
    RemoveChannel(usize),
    SetNotes {
        pattern: usize,
        channel: usize,
        notes: Vec<NoteEvent>,
    },
    // ...
}
```

This is a Rust **enum**. Unlike enums in C or Java, each variant can carry
its own data: `Seek` carries a step number, `SetNotes` a whole list of
notes. An enum value is exactly one of its variants.

The messages travel through a **ring buffer** from the
[`rtrb`](https://docs.rs/rtrb) crate: a fixed-size queue, allocated once,
with one writer (the `Producer`) and one reader (the `Consumer`). Pushing
and popping never lock and never allocate, so the audio thread can read
it safely.

```rust
let (command_producer, command_consumer) = rtrb::RingBuffer::new(COMMAND_QUEUE_SIZE);
let (trash_producer, trash_consumer) = rtrb::RingBuffer::new(TRASH_QUEUE_SIZE);
let (live_producer, live_consumer) = rtrb::RingBuffer::new(LIVE_QUEUE_SIZE);
let (midi_producer, midi_consumer) = rtrb::RingBuffer::new(MIDI_OUT_QUEUE_SIZE);
```

`RingBuffer::new` returns a **tuple** of two values, which we take apart
with a `let (a, b) = ...` **pattern**. There are four queues: commands
from the UI, values for the UI to free (the trash, below), keys from a
MIDI keyboard ([MIDI](../sound/midi.md)), and messages for MIDI Out
channels. For each, one end stays on one thread and the other end is
moved to the other.

### Ownership makes this safe

Here is where Rust shines. `Producer` can be used from one thread and
`Consumer` from another, but there is only one of each, and Rust's
**ownership** rules guarantee it:

- Every value has exactly one **owner**. When the owner goes away, the
  value is dropped (freed).
- Assigning or passing a value **moves** it to the new owner, and the old
  name can no longer be used.

Once `consumer` is moved into the audio callback, the UI code physically
cannot touch it anymore: the compiler would reject it. Data races are not
"unlikely"; they are compile errors.

### Commands that carry data

`Command` is not `Copy`, because an `Instrument` can hold an
`Arc<SampleData>` and `SetNotes` holds a `Vec`. Moving it through the
ring buffer is still fine: the value is moved into the queue's
preallocated slot and out again, with no allocation. The `Vec` inside
was allocated on the UI thread, before it was sent.

### When the queue is full

The queue holds 4096 commands, enough for a whole large project. If it
is ever full, the UI got ahead of the audio thread, which empties it at
the start of every block. `send_wait` does not drop the command: it
yields and tries again, up to a thousand times.

```rust
pub fn send_wait(&mut self, mut command: Command) -> bool {
    for _ in 0..SEND_WAIT_ROUNDS {
        match self.commands.push(command) {
            Ok(()) => return true,
            Err(rtrb::PushError::Full(returned)) => command = returned,
        }
        // A stream that has reported an error will not empty the
        // queue, so waiting for room would only freeze the window.
        if self.problems.broken.load(Ordering::Relaxed) {
            return false;
        }
        std::thread::yield_now();
    }
    false
}
```

`push` takes the command by value. When the queue is full it gives the
command back inside the error, `PushError::Full(returned)`, so the loop
can try again with it: nothing is copied and nothing is lost.

## Opening the sound card

`AudioEngine::start` uses the [`cpal`](https://docs.rs/cpal) crate, which
works on Linux (ALSA), macOS (CoreAudio) and Windows (WASAPI), so the
same code runs on all three.

```rust
pub fn start() -> Result<Self, EngineError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(EngineError::NoOutputDevice)?;
    let supported = device
        .default_output_config()
        .map_err(EngineError::Config)?;
    ...
}
```

### `Option`, `Result` and the `?` operator

Rust has no `null` and no exceptions. Instead:

- **`Option<T>`** is either `Some(value)` or `None`.
  `default_output_device()` returns `Option<Device>` because there may be
  no sound card.
- **`Result<T, E>`** is either `Ok(value)` or `Err(error)`.
  `default_output_config()` returns `Result<SupportedStreamConfig, cpal::Error>`.

Both are ordinary enums from the standard library. Because they are
types, you cannot forget to handle the missing or failed case: the
compiler will not let you use a `Device` you might not have.

- `.ok_or(EngineError::NoOutputDevice)` turns an `Option` into a
  `Result`, using our error for the `None` case.
- `.map_err(EngineError::Config)` wraps cpal's error in our own variant.
  Notice that `EngineError::Config` is used like a function here: enum
  variants with data can be called as constructors.
- **`?`** means "if this is an `Err`, return it from the function right
  now; otherwise give me the `Ok` value". It replaces long chains of
  `if error then return`.

### A custom error type

```rust
#[derive(Debug)]
pub enum EngineError {
    NoOutputDevice,
    Config(cpal::Error),
    UnsupportedFormat(SampleFormat),
    BuildStream(cpal::Error),
    Play(cpal::Error),
}

impl fmt::Display for EngineError { ... }
impl std::error::Error for EngineError {}
```

One enum lists every way starting the engine can fail. Implementing
`Display` gives each a human-readable message, and the empty
`impl std::error::Error` marks it as a proper error type that works with
the rest of the Rust ecosystem. The app turns it into text with
`err.to_string()` and keeps running without sound: every window works,
and **Edit > Settings… > Audio** offers to try again.

## Generics: one function, many sample types

Different sound cards want samples in different formats: 32-bit floats,
16-bit integers, and so on. We generate `f32` internally and convert at
the last moment. Instead of writing the stream code six times, we write it
once as a **generic function**:

```rust
fn build_stream<T>(
    device: &cpal::Device,
    config: StreamConfig,
    mut audio_thread: AudioThread,
    problems: Arc<StreamProblems>,
) -> Result<cpal::Stream, EngineError>
where
    T: SizedSample + FromSample<f32>,
{ ... }
```

`T` is a placeholder type, and the `where` clause lists **trait bounds**:
`T` can be any type that is a cpal sample (`SizedSample`) and that can be
created from an `f32` (`FromSample<f32>`). Inside, `T::from_sample(x)`
does the conversion.

Then `start` picks the right version with a `match`:

```rust
let stream = match sample_format {
    SampleFormat::F32 => {
        build_stream::<f32>(&device, config, audio_thread, Arc::clone(&problems))
    }
    SampleFormat::I16 => {
        build_stream::<i16>(&device, config, audio_thread, Arc::clone(&problems))
    }
    ...
    other => return Err(EngineError::UnsupportedFormat(other)),
}?;
```

- `match` compares a value against **patterns** and runs the first arm
  that fits. The compiler checks that every possible value is covered.
  `other` is a catch-all pattern that also gives a name to the value.
- `build_stream::<f32>` (the "turbofish" syntax `::<>`) chooses `T = f32`.
  The compiler generates a separate, fully optimized copy of the function
  for each type used. This is called **monomorphization**, and it means
  generics cost nothing at run time.
- `audio_thread` is moved into whichever arm runs. The compiler knows
  only one arm runs, so this is allowed.

## The audio callback is a closure

Inside `build_stream`:

```rust
let data_callback = move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
    audio_thread.process(output);
};

let error_callback = move |err: cpal::Error| {
    problems.broken.store(true, Ordering::Relaxed);
    if let Ok(mut slot) = problems.error.lock() {
        *slot = Some(err.to_string());
    }
    eprintln!("audio stream error: {err}");
};

device
    .build_output_stream(config, data_callback, error_callback, None)
    .map_err(EngineError::BuildStream)
```

`|args| body` is a **closure**: an anonymous function that can use
variables from the place where it was written. The `move` keyword makes
the closure take **ownership** of `audio_thread`, and with it of the
command consumer and everything else the audio thread works with. That
is necessary because the closure will run on another thread, long after
`build_stream` has returned, so it cannot just borrow local variables.

cpal's signature requires the callback to be
`FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static`:

| Bound | Meaning |
|-------|---------|
| `FnMut` | a closure that may change the values it owns (the instruments advance their state) |
| `Send` | safe to move to another thread |
| `'static` | owns everything it uses, borrowing nothing that could disappear |

If we tried to share something unsafe, such as a plain reference into the
UI's data, the program would not compile. This is how Rust turns the
golden rule of audio into checks the compiler does for you.

`&mut [T]` is a **mutable slice**: a borrowed view into the sound card's
buffer, which we fill but do not own.

The error callback is the one place that may lock and print: cpal calls
it outside the audio callback, when something goes wrong while the
stream runs, such as an unplugged interface. It keeps the newest
problem for the window to show.

## `AudioThread::process`

The audio callback hands each buffer to one method. Its outline:

```rust
pub(crate) fn process<T>(&mut self, output: &mut [T])
where
    T: SizedSample + FromSample<f32>,
{
    self.flush_trash();
    self.flush_midi_backlog();
    while let Ok(command) = self.commands.pop() {
        self.apply(command);
    }
    while let Some(message) = self.live_input.as_mut().and_then(|keys| keys.pop().ok()) {
        self.play_live(message);
    }

    for frame in output.chunks_mut(self.output_channels) {
        if self.playing
            && let Some(step) = self.sequencer.tick()
        {
            self.playhead.store(step, Ordering::Relaxed);
            self.start_notes(step);
        }
        // ... mix the channels, the reverb and the metronome ...
    }
    self.send_midi();
}
```

- `while let Ok(command) = ...` keeps looping as long as the pattern
  matches, so it drains every waiting command and stops when the queue is
  empty.
- `as_mut` turns `&mut Option<Consumer>` into `Option<&mut Consumer>`,
  and `and_then` asks it for a key only if there is a consumer.
- `chunks_mut(channels)` splits the buffer into **frames**, one sample
  per output channel.
- `if a && let Some(x) = b { ... }` is a **let chain**: a condition and a
  pattern match combined in one `if`.

The audio thread's state lives in a struct, `AudioThread`, instead of
loose variables inside the closure. That lets the tests run it with a
plain `Vec` instead of a sound card, and lets the
[renderer](../sound/export.md) play a song into a file with the same
code.

## Mixing a frame

For each frame, every channel's instrument makes one sample. The sample
goes through the channel's effects and volume, and then to the left and
right sides as its pan says, and to the reverb as much as its send says
([The mixer](../sound/mixer.md)):

```rust
for channel in &mut self.channels {
    let volume = channel.volume.next_value();
    let sample = channel.effects.process(channel.instrument.next_sample()) * volume;
    ...
    dry[0] += kept * gains[0];
    dry[1] += kept * gains[1];
    send += sample * amount;
}
...
// `tanh` gently rounds off peaks above 1.0 instead of clipping them hard.
let stereo = stereo.map(f32::tanh);
```

- `for channel in &mut self.channels` iterates over mutable references,
  so each instrument can advance its own state.
- Mixing is just adding: sounds in the air add up, and so do samples.
- Several loud drums at once can add up past 1.0, which a sound card
  would clip harshly. `tanh` (the hyperbolic tangent) passes small values
  almost unchanged and bends large ones smoothly toward ±1.0, a gentle
  **soft clipper**.
- `volume.next_value()` is a
  [`Smoothed`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs)
  value. A volume that jumps makes the speaker jump, which you hear as a
  click, so it glides toward its target instead: every sample it covers
  a small fraction of the remaining distance. This is a *one-pole
  smoother*, settling in a few milliseconds, too fast to hear as a fade
  but enough to remove the click.

## Room reserved in advance

```rust
channels: Vec::with_capacity(MAX_CHANNELS),
```

A `Vec` has a **length** (how many items it holds) and a **capacity**
(how many fit in the memory it has). `push` allocates only when the
length would exceed the capacity. Reserving room for `MAX_CHANNELS` at
startup, on the UI thread, means `push` on the audio thread never
allocates. The UI refuses to add more channels than that, so the limit is
never hit. The list of notes waiting to start late, the pattern lengths
and the trash backlog are reserved the same way.

`Vec::remove(index)` shifts the following items down, which never
allocates either. That is how `RemoveChannel` keeps the audio thread's
channel numbers matching the UI's list.

## Freeing memory somewhere else: the trash queue

When a channel is removed or its notes replaced, the old value may hold
the last `Arc` to a sample, or a `Vec` of notes. Dropping it would free
that memory, and freeing can take a lock inside the memory allocator.
So the audio thread does not drop it. It sends it back:

```rust
fn set_instrument(&mut self, channel: usize, instrument: Instrument) {
    match self.channels.get_mut(channel) {
        Some(state) => {
            let mut old = std::mem::replace(&mut state.instrument, instrument);
            silence(&mut old, &mut self.midi_out, &mut self.midi_backlog);
            ...
            self.throw_away(Garbage::Instrument(old));
        }
        None => self.throw_away(Garbage::Instrument(instrument)),
    }
}
```

`std::mem::replace(&mut place, new)` puts `new` into `place` and returns
what was there. It is how you take a value out of a struct field you
only have a `&mut` to, since Rust never lets a field be left empty.
(`silence` sends the note-offs the old instrument still owes a MIDI Out
synth.)

`throw_away` pushes the value into a second ring buffer, going the other
way. `Garbage` is an enum with a variant for each kind of thing the
audio thread gives back: instruments, note lists, channels, songs,
tempo lists, phrases, delay lines. The UI thread empties the queue once
per frame:

```rust
pub fn collect_garbage(&mut self) {
    while let Ok(garbage) = self.trash.pop() {
        // Dropping a value frees its memory. Here, on the UI thread,
        // that is allowed to take as long as it needs.
        match garbage {
            Garbage::Instrument(instrument) => drop(instrument),
            Garbage::Notes(notes) => drop(notes),
            ...
        }
    }
}
```

`drop(value)` is a plain function from the prelude that takes ownership
and lets the value go out of scope. Writing it explicitly documents the
intent: this is where the memory is freed.

A burst of changes between two frames could fill the trash queue. Then
`throw_away` keeps the value in a backlog reserved in advance, and
`flush_trash` moves it into the queue at the start of the next block.
Only if the backlog fills too, which means the UI has not collected for
a long time, is the value freed on the audio thread, as a last resort.

## The playhead: an atomic integer

The UI wants to highlight the step being played. The audio thread knows
it. Sharing it through a `Mutex` would mean the audio thread could wait
for the lock, so we use an **atomic**:

```rust
let playhead = Arc::new(AtomicUsize::new(NO_STEP));
// audio thread:
self.playhead.store(step, Ordering::Relaxed);
// UI thread:
match self.playhead.load(Ordering::Relaxed) { NO_STEP => None, step => Some(step) }
```

An `AtomicUsize` is a number that the processor can read and write in one
indivisible operation, so two threads can use it without a lock and
without ever seeing a half-written value. Both threads hold an `Arc` to
the same one (`Arc::clone(&playhead)` makes the second handle).

`Ordering::Relaxed` says we only care about this one value, not about
the order of other memory writes around it. It is the cheapest ordering
and enough for a display. Stricter orderings (`Acquire`, `Release`,
`SeqCst`) are for when one value guards access to others.

Rust has no "null" step, so `usize::MAX` stands for "stopped", and
`playhead()` turns it into an honest `Option<usize>` right away so no
other code ever sees the magic number.

The position to a fraction of a step, which recording and the views
need, is an `f64`, and there is no atomic float. So it is kept as the
bits of the `f64` in an `AtomicU64`: `position.to_bits()` on the audio
thread, `f64::from_bits(bits)` on the other side. Two numbers that must
always be read together, such as the pattern played over the song and
its step, share one `AtomicU64`: one in the high 32 bits, the other in
the low ones.

## Keeping the stream alive: `Drop`

```rust
pub struct AudioEngine {
    // The stream is never read, but it must be kept: dropping it stops audio.
    _stream: cpal::Stream,
    commands: rtrb::Producer<Command>,
    trash: rtrb::Consumer<Garbage>,
    playhead: Arc<AtomicUsize>,
    position: Arc<AtomicU64>,
    ...
}
```

The `cpal::Stream` is never used after it starts, but it must be kept:
when a value is dropped, Rust runs its cleanup code (its `Drop`
implementation), and dropping a `Stream` closes the sound card. Storing
it in the engine ties the sound's lifetime to the engine's. The leading
underscore tells the compiler "I know this field is never read".

This pattern, where owning a value means owning a resource and dropping it
releases the resource, is called **RAII**. Rust uses it everywhere: files
close, locks unlock and memory frees itself when its owner goes away.

## When the sound card goes away

An unplugged interface, a changed default device or a sound server that
restarted makes the stream report an error. **Edit > Settings… > Audio**
then offers **Try again** (and **Restart audio** while the stream runs),
which opens the system's default output device again. The project
stays: the whole of it is sent to the fresh audio thread, the MIDI
ports are reconnected to the new engine, and a song that was playing
seeks back to the same position and plays on.

The old engine is dropped before the new one starts, because a device
being reopened cannot still be held by the dead stream.

Two things need care when an audio thread goes away:

**Notes sounding on an external synth.** A note sent over MIDI Out gets
its note-off from the audio thread that played it; when that thread is
gone, the note-offs it still owed go with it. The messages go out
through a small thread, `midi-out`, that empties the audio thread's
queue every millisecond. When its queue is abandoned, its loop ends,
and before it finishes it sends **All Notes Off** (controller 123) on
all sixteen channels:

```rust
let sender = move || {
    while !messages.is_abandoned() {
        // ... send what the audio thread queued ...
    }
    // The engine is gone, and with it the note offs of notes still
    // sounding on the synth.
    if let Some(connection) = lock(&output).as_mut() {
        for channel in 0..16 {
            let _ = connection.send(&all_notes_off(channel));
        }
    }
};
```

The new engine's sending thread must not start before the old one has
sent its All Notes Off, or that message could cut off the new engine's
first notes. So `reconnect_audio` waits for the old thread, polling
`JoinHandle::is_finished` for at most 100 ms (in practice about a
millisecond, since the old engine is already gone). It does not call
`join()`, which could wait for ever on a MIDI port that hangs: the
window must never freeze over a port.

**Messages that must arrive.** A dropped note-off leaves a note sounding
for ever on an external synth, so MIDI Out never drops one. Messages
that find the output queue full wait in a reserved backlog and go out
at the start of the next block. Removing a channel or replacing an
instrument sends its note-offs the same way, and disconnecting a port
sends All Notes Off on all sixteen channels before the connection
closes.

The [playback trace](../sound/trace.md) is the opposite case: it
records every note for a report, not for the sound, so if its queue
fills, the audio thread counts the lost events and carries on. Waiting
there would put the trace before the music. When the audio restarts,
the trace goes on in the same file: what the old thread left is written,
and a new queue is sent to the new thread.

## Testing without a sound card

The tests at the bottom of `engine.rs` build an `AudioThread` with ring
buffers, push commands, render into a `Vec<f32>` and check the result:
a hit at frame 0 and another 6000 frames later at 120 BPM, silence when
muted, edits taking effect while playing, and old instruments arriving
in the trash. A small `Harness` struct holds the pieces so each test
stays short.
