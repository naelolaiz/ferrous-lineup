# The sequencer: timing to the sample

File: [`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sequencer.rs)

## The arithmetic

At a tempo of *T* beats per minute and a sample rate of *R* frames per
second, one beat lasts `R × 60 / T` frames and one step (a quarter of a
beat) lasts a quarter of that:

```text
frames per step = R × 60 / T / 4

120 BPM, 48 kHz:  48000 × 60 / 120 / 4 = 6000 frames      (exact)
130 BPM, 44.1 kHz: 44100 × 60 / 130 / 4 = 5088.46... frames (not whole!)
```

If we rounded 5088.46 to 5088, every step would be 0.46 frames short. That
is about 10 microseconds, nothing you would hear once, but it adds up:
after a few minutes the drums would be audibly ahead of anything else
running at 130 BPM. So the sequencer keeps the fraction.

## Ticking

The audio thread calls `tick` once for every frame it renders:

```rust
pub fn tick(&mut self) -> Option<usize> {
    let fired = if self.frames_until_next <= 0.0 {
        let step = self.next_step;
        self.next_step = (self.next_step + 1) % self.length;
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
frame each time, but never drifting. `%` is the remainder operator, which
wraps the step number back to 0 at the end of the pattern.

Because a step is triggered on the exact frame where it begins, not at
the start of the audio block that contains it, the timing does not depend
on the sound card's buffer size. This is what "sample-accurate" means.

## Why it lives in `lineup-core`

The sequencer has nothing to do with sound cards. It only counts. Putting
it in `lineup-core` means it is tested in milliseconds, without audio, and
step 4's playlist can reuse it.

## Testing with iterators

```rust
fn run(sequencer: &mut Sequencer, frames: usize) -> Vec<(usize, usize)> {
    (0..frames)
        .filter_map(|frame| sequencer.tick().map(|step| (frame, step)))
        .collect()
}
```

This little helper ticks the sequencer `frames` times and lists every
step that fired together with the frame it fired on:

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

and that one minute at 130 BPM gives exactly 520 steps (130 beats × 4).
Underscores in number literals like `6_000` are ignored by the compiler
and just make big numbers easier to read.

The same timing was also checked end to end: the app was run with its
sound output recorded to a file, and every hit started on the expected
frame.
