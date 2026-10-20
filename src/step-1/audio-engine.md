# lineup-audio: the real-time engine

This is the most important chapter of step 1. The way audio works shapes
the whole program, and it is also where Rust's ownership rules start to
earn their keep.

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
- no **memory allocation** (`Vec::push`, `String`, `Box::new`...),
- no **file, network or console I/O** (`println!` included),
- no waiting for other threads.

This rule decides how the UI talks to the audio engine.

## Sending commands with a ring buffer

File: [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

The UI never changes audio state directly. It sends small messages:

```rust
pub enum Command {
    SetToneOn(bool),
    SetFrequency(f32),
    SetVolume(f32),
}
```

This is a Rust **enum**. Unlike enums in C or Java, each variant can carry
its own data: `SetToneOn` carries a `bool`, `SetFrequency` an `f32`. An
enum value is exactly one of its variants.

The messages travel through a **ring buffer** from the
[`rtrb`](https://docs.rs/rtrb) crate: a fixed-size queue, allocated once,
with one writer (the `Producer`) and one reader (the `Consumer`). Pushing
and popping never lock and never allocate, so the audio thread can read
it safely.

```rust
let (producer, consumer) = rtrb::RingBuffer::new(COMMAND_QUEUE_SIZE);
```

`RingBuffer::new` returns a **tuple** of two values, which we take apart
with a `let (a, b) = ...` **pattern**. The producer stays in the
`AudioEngine` on the UI side, and the consumer is moved into the audio
callback.

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

## Opening the sound card

`AudioEngine::start` uses the [`cpal`](https://docs.rs/cpal) crate, which
works on Linux (ALSA), macOS (CoreAudio) and Windows (WASAPI). This is why
the Linux-first plan will not need a rewrite for the other systems.

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
the rest of the Rust ecosystem. The UI turns it into text for the status
bar with `err.to_string()`.

## Generics: one function, many sample types

Different sound cards want samples in different formats: 32-bit floats,
16-bit integers, and so on. We generate `f32` internally and convert at
the last moment. Instead of writing the stream code six times, we write it
once as a **generic function**:

```rust
fn build_stream<T>(device: &cpal::Device, config: StreamConfig,
                   commands: rtrb::Consumer<Command>)
    -> Result<cpal::Stream, EngineError>
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
    SampleFormat::F32 => build_stream::<f32>(&device, config, consumer),
    SampleFormat::I16 => build_stream::<i16>(&device, config, consumer),
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
- `consumer` is moved into whichever arm runs. The compiler knows only one
  arm runs, so this is allowed.

## The audio callback is a closure

Inside `build_stream`:

```rust
let mut audio_thread = AudioThread::new(config.sample_rate, config.channels as usize, commands);

let data_callback = move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
    audio_thread.process(output);
};

device.build_output_stream(config, data_callback, error_callback, None)
```

`|args| body` is a **closure**: an anonymous function that can use
variables from the place where it was written. The `move` keyword makes
the closure take **ownership** of `audio_thread` (and so of the command
consumer and the test tone inside it). That is necessary because the
closure will run on another thread, long after `build_stream` has
returned, so it cannot just borrow local variables.

cpal's signature requires the callback to be
`FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static`:

| Bound | Meaning |
|-------|---------|
| `FnMut` | a closure that may change the values it owns (our oscillator advances its phase) |
| `Send` | safe to move to another thread |
| `'static` | owns everything it uses, borrowing nothing that could disappear |

If we tried to share something unsafe, such as a plain reference into the
UI's data, the program would not compile. This is how Rust turns the
golden rule of audio into checks the compiler does for you.

`&mut [T]` is a **mutable slice**: a borrowed view into the sound card's
buffer, which we fill but do not own.

## `AudioThread::process`

```rust
fn process<T>(&mut self, output: &mut [T])
where
    T: SizedSample + FromSample<f32>,
{
    while let Ok(command) = self.commands.pop() {
        match command {
            Command::SetToneOn(on) => self.tone.set_on(on),
            Command::SetFrequency(hz) => self.tone.set_frequency(hz),
            Command::SetVolume(volume) => self.tone.set_volume(volume),
        }
    }
    for frame in output.chunks_mut(self.channels) {
        let sample = T::from_sample(self.tone.next_sample());
        frame.fill(sample);
    }
}
```

- `while let Ok(command) = ...` keeps looping as long as the pattern
  matches, so it drains every waiting command and stops when the queue is
  empty.
- The `match` **destructures** each variant: in `Command::SetFrequency(hz)`,
  `hz` is bound to the number inside.
- `chunks_mut(channels)` splits the buffer into frames, one sample per
  channel, and `fill` writes the same value to every channel.

Nothing here allocates, locks or prints. The function lives in its own
struct instead of inside the closure, so the tests at the bottom of
`engine.rs` can call it with a plain `Vec` instead of a sound card.

## Keeping the stream alive: `Drop`

```rust
pub struct AudioEngine {
    _stream: cpal::Stream,
    commands: rtrb::Producer<Command>,
    info: EngineInfo,
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

## The test tone

File: [`crates/lineup-audio/src/synth.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/synth.rs)

`TestTone` generates a sine wave with a **phase accumulator**: `phase`
goes from 0.0 to 1.0 once per cycle, and each sample advances it by
`frequency / sample_rate`. The sample is `sin(phase × 2π)`.

```rust
pub fn next_sample(&mut self) -> f32 {
    let target = if self.on { self.volume } else { 0.0 };
    self.current_gain += (target - self.current_gain) * self.smoothing;

    let sample = (self.phase * TAU).sin() * self.current_gain;

    self.phase += self.frequency / self.sample_rate;
    if self.phase >= 1.0 {
        self.phase -= 1.0;
    }
    sample
}
```

Switching a sound on or off instantly makes the speaker jump, which you
hear as a click. So the gain glides toward its target: every sample it
covers a small fraction (`smoothing`) of the remaining distance. This is a
*one-pole smoother*, and with the value we chose it settles in about 5 ms,
too fast to hear as a fade but enough to remove the click.

Notice `if` is an **expression** in Rust: `if self.on { a } else { b }`
produces a value, like the `?:` operator in other languages.

`&mut self` means the method **borrows** the tone mutably: it can change
the fields, and while it runs nobody else can read or write the tone.
Rust allows either one mutable borrow *or* any number of shared (`&`)
borrows at a time, never both. This rule, checked by the **borrow
checker**, is what prevents data races and many other bugs.
