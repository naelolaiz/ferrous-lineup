# The audio thread, grown up

File: [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

Step 1's golden rule still holds: the audio thread must never wait, lock,
allocate or free memory. Step 2 makes that rule harder to keep, because
commands now carry whole instruments, channels come and go, and the UI
needs to know where the playhead is.

## Commands that carry data

```rust
pub enum Command {
    Play,
    Stop,
    SetTempo(Tempo),
    AddChannel { instrument: Instrument, steps: StepRow, volume: f32, muted: bool },
    RemoveChannel(usize),
    SetInstrument { channel: usize, instrument: Instrument },
    SetStep { channel: usize, step: usize, on: bool },
    ...
}
```

The command is no longer `Copy`, because `Instrument` can hold an
`Arc<SampleData>`. Moving it through the ring buffer is still fine: the
value is moved into the queue's preallocated slot and out again, with no
allocation. Note that it is sent by value, not as `Box<Instrument>`: a
`Box` would be freed by the audio thread when it is done with it.

## Room reserved in advance

```rust
channels: Vec::with_capacity(MAX_CHANNELS),
```

A `Vec` has a **length** (how many items it holds) and a **capacity**
(how many fit in the memory it has). `push` allocates only when the
length would exceed the capacity. Reserving room for `MAX_CHANNELS` at
startup, on the UI thread, means `push` on the audio thread never
allocates. The UI refuses to add more channels than that, so the limit is
never hit.

`Vec::remove(index)` shifts the following items down, which never
allocates either. That is how `RemoveChannel` keeps the audio thread's
channel numbers matching the UI's list.

## Freeing memory somewhere else: the trash queue

When a channel is removed or its instrument replaced, the old instrument
may hold the last `Arc` to a sample. Dropping it would free that memory,
and freeing can take a lock inside the memory allocator. So the audio
thread does not drop it. It sends it back:

```rust
Command::SetInstrument { channel, instrument } => match self.channels.get_mut(channel) {
    Some(state) => {
        let old = std::mem::replace(&mut state.instrument, instrument);
        self.throw_away(old);
    }
    None => self.throw_away(instrument),
},
```

`std::mem::replace(&mut place, new)` puts `new` into `place` and returns
what was there. It is how you take a value out of a struct field you
only have a `&mut` to, since Rust never lets a field be left empty.

`throw_away` pushes the instrument into a second ring buffer, going the
other way. The UI thread empties it once per frame:

```rust
pub fn collect_garbage(&mut self) {
    while let Ok(old_instrument) = self.trash.pop() {
        drop(old_instrument);
    }
}
```

`drop(value)` is a plain function from the prelude that takes ownership
and lets the value go out of scope. Writing it explicitly documents the
intent: this is where the memory is freed.

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

## Rendering a block

```rust
for frame in output.chunks_mut(self.output_channels) {
    if self.playing
        && let Some(step) = self.sequencer.tick()
    {
        self.playhead.store(step, Ordering::Relaxed);
        for channel in &mut self.channels {
            if channel.steps[step % MAX_STEPS] && !channel.muted {
                channel.instrument.trigger(self.sample_rate);
            }
        }
    }

    let mut mix = 0.0;
    for channel in &mut self.channels {
        let volume = channel.volume.next_value();
        mix += channel.instrument.next_sample() * volume;
    }
    let sample = (mix * self.master_volume.next_value()).tanh();
    frame.fill(T::from_sample(sample));
}
```

- `if a && let Some(x) = b { ... }` is a **let chain** (Rust 1.88): a
  condition and a pattern match combined in one `if`.
- `for channel in &mut self.channels` iterates over mutable references,
  so each instrument can advance its own state.
- Mixing is just adding: sounds in the air add up, and so do samples.
- Several loud drums at once can add up past 1.0, which a sound card
  would clip harshly. `tanh` (the hyperbolic tangent) passes small values
  almost unchanged and bends large ones smoothly toward ±1.0, a gentle
  **soft clipper**.
- The UI never sets a pattern longer than 64 steps, but `% MAX_STEPS`
  guarantees the index is in range even if it did, so this line can never
  panic on the audio thread.

## Testing without a sound card

The tests at the bottom of `engine.rs` build an `AudioThread` with ring
buffers, push commands, render into a `Vec<f32>` and check the result:
a hit at frame 0 and at frame 6000 at 120 BPM, silence when muted, edits
taking effect while playing, and old instruments arriving in the trash.
A small `Harness` struct holds the pieces so each test stays short.
