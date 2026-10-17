# Instruments: drums, sampler and synth

Files: [`drums.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/drums.rs),
[`dsp.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/dsp.rs),
[`sample.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/sample.rs) and
[`instrument.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/instrument.rs)
in `crates/lineup-audio/src/`

## Synthesizing a drum kit

Ferrous Lineup ships no sound files: its drums are computed when a drum
channel is created. Each drum is a few lines of math.

| Drum | Recipe |
|------|--------|
| Kick | A sine wave whose pitch drops fast from 150 Hz to 45 Hz, fading out over half a second |
| Snare | A short 185 Hz tone (the drum shell) plus high-passed noise (the snare wires) |
| Clap | High-passed noise in three quick bursts 11 ms apart, like several hands, plus a short tail |
| Hi-hat | Very bright noise (high-passed at 7 kHz) that dies away in about 0.1 s |

Most of them share one shape, the **exponential decay**:

```rust
fn decay(t: f32, tau: f32) -> f32 {
    (-t / tau).exp()
}
```

It is 1.0 at the start and falls to about 37% after `tau` seconds, 13%
after `2 × tau`, and so on. Natural sounds fade this way.

### Closures that remember: `FnMut`

All the drums are built with one helper:

```rust
fn generate(seconds: f32, sample_rate: u32, f: impl FnMut(f32) -> f32) -> Vec<f32> {
    let frames = (seconds * sample_rate as f32) as usize;
    (0..frames)
        .map(|i| i as f32 / sample_rate as f32)
        .map(f)
        .collect()
}
```

It takes a closure `f` that maps a time (in seconds) to a sample. The
kick's closure keeps a running `phase` between calls:

```rust
fn kick(sample_rate: u32) -> Vec<f32> {
    let mut phase = 0.0;
    generate(0.5, sample_rate, |t| {
        let frequency = 45.0 + 105.0 * decay(t, 0.04);
        phase += frequency / sample_rate as f32;
        (phase * TAU).sin() * decay(t, 0.15)
    })
}
```

The closure *changes* `phase`, a variable it captured from `kick`. A
closure that mutates what it captures implements the **`FnMut`** trait
(`Fn` is for closures that only read, `FnOnce` for ones that can be called
only once). `impl FnMut(f32) -> f32` in the parameter list means "any
closure of that shape". The compiler generates a specialized `generate`
for each drum's closure, just as it does for generic types.

Why accumulate a phase instead of computing `sin(t × frequency × 2π)`
directly? Because the frequency changes over time. The phase is the sum
of all the small steps taken so far, which gives a smooth pitch sweep;
the direct formula would jump around.

### White noise without a crate

```rust
pub fn next_sample(&mut self) -> f32 {
    let mut x = self.state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    self.state = x;
    (x as f32 / u32::MAX as f32) * 2.0 - 1.0
}
```

This is **xorshift**, a tiny random number generator: three shifts (`<<`,
`>>`) and exclusive-ors (`^`). It is not secure, but it sounds just like
noise, never allocates, and gives the same "random" drums every time,
which makes them testable.

### Filters

`dsp.rs` has two **one-pole filters**, the simplest filters there are.
A **low-pass** keeps low frequencies by moving its output a fraction of
the way toward the input each sample (exactly the same math as the
`Smoothed` volume). A **high-pass** keeps high frequencies by following
the *changes* in the input. Their tests show the basic behavior: a
constant input passes the low-pass and is removed by the high-pass.

## The sampler

```rust
pub struct Sampler {
    sample: Arc<SampleData>,
    position: Option<f64>,
    increment: f64,
}
```

### `Arc`: shared ownership

`Arc<T>` (*atomically reference-counted*) is a pointer that several
owners can share. Cloning an `Arc` does not copy the data; it adds one to
a counter. When the last `Arc` is dropped, the data is freed. The
"atomic" part means the counter is safe to update from several threads,
which is what lets a sample cross from the UI thread to the audio thread.

### Playing at a different sample rate

A WAV file recorded at 44.1 kHz played on a 48 kHz sound card must
advance through the file a bit slower than one frame per frame, or it
would sound higher and shorter. `increment` is the ratio of the two
rates, and `position` is fractional. Between two stored frames, the
sampler draws a straight line (**linear interpolation**):

```rust
let fraction = (position - index as f64) as f32;
let sample = frames[index] + (frames[index + 1] - frames[index]) * fraction;
```

### `let ... else`

```rust
let Some(position) = self.position else {
    return 0.0;
};
```

`let`-`else` destructures a pattern or, if it does not match, runs the
`else` block, which must leave the function (with `return`, `break` and so
on). It keeps the main path of the function unindented: "if we are not
playing, output silence; otherwise carry on with `position`".

## Loading WAV files

`SampleData::load_wav` uses the [`hound`](https://docs.rs/hound) crate.
WAV files store samples either as floats or as integers of 8, 16, 24 or
32 bits; both become floats between -1.0 and 1.0, and stereo files are
mixed down to mono by averaging each frame's channels.

```rust
reader
    .samples::<i32>()
    .map(|sample| sample.map(|s| s as f32 / full_scale))
    .collect::<Result<_, _>>()
    .map_err(SampleError::Wav)?
```

`samples()` yields a `Result` per sample, because the file could be cut
off halfway. Collecting an iterator of `Result`s into `Result<Vec<_>, _>`
gives `Ok(vec)` if every item was `Ok`, or the first `Err`. The `_`
placeholders let the compiler fill in the types.

The test writes a real WAV file to the system's temporary folder with
`hound::WavWriter`, loads it back and checks the mono mix.

## The Lineup Osc

The synth channel generates a **sawtooth** wave, a ramp from -1 to 1 once
per cycle, which is bright and buzzy because it is rich in harmonics. A
low-pass filter at 2.5 kHz softens it, and an **envelope** shapes its
volume like a pluck: a 3 ms fade-in (the *attack*, to avoid a click)
followed by an exponential fade (the *decay*).

It plays C4 on every step for now. In step 3 the piano roll will give
each step its own note.

> A plain sawtooth computed like this contains frequencies above what the
> sample rate can represent, which fold back as faint metallic tones
> (*aliasing*). Fixing that properly (with "PolyBLEP" or wavetables) is a
> good later exercise.

## An enum instead of a trait object

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
}
```

Another common design is a trait `Instrument` with `Box<dyn Instrument>`
values. That is more open-ended, since anyone could add a new kind, but
every `Box::new` allocates, and calls go through a pointer to a function
table. With only two kinds, an enum is simpler: the audio thread
`match`es on it, creating one never allocates, and the compiler checks
that every kind is handled. If Ferrous Lineup ever loads plugins, the trait
design will return.
