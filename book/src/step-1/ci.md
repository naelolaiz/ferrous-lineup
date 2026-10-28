# Tests and continuous integration

## Running the tests

```sh
cargo test                 # every crate
cargo test -p lineup-core    # one crate
cargo test frequency       # only tests whose name contains "frequency"
```

Step 1 had tests in four files:

| File | What it checks |
|------|----------------|
| `lineup-core/src/pitch.rs` | A4 is 440 Hz, middle C is 261.63 Hz, octaves double, note names |
| `lineup-core/src/tempo.rs` | Tempo clamps to its range, beat lengths are right |
| `lineup-audio/src/synth.rs` | The test tone, at this step: silent when off, fades in without a click, stays within its volume, plays the right frequency (today the synth's tests live in `instrument.rs`) |
| `lineup-audio/src/engine.rs` | Commands reach the audio thread; integer sample formats use their full range |

The test tone's frequency test is a nice example of testing audio without
listening to it. It renders one second of a 100 Hz tone and counts how
often the wave crosses zero going up. That should happen 100 times:

```rust
let rising_crossings = buffer
    .windows(2)
    .filter(|w| w[0] < 0.0 && w[1] >= 0.0)
    .count();
```

This is an **iterator chain**. `windows(2)` yields every pair of
neighboring samples, `filter` keeps the pairs that go from negative to
non-negative, and `count` counts them. Iterators are lazy: nothing happens
until `count` asks for values, and the compiler turns the whole chain into
a simple loop, as fast as one written by hand.

## Formatting and linting

Two more tools keep the code tidy:

- `cargo fmt` rewrites the code in the standard Rust style, so nobody
  argues about where braces go.
- `cargo clippy` is the official **linter**. It knows hundreds of patterns
  that compile fine but could be simpler, faster or more correct, and
  explains each suggestion.

## Continuous integration

File: [`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/ci.yml)

At this step, every push and pull request ran one Linux job, **check**:
it installed the system libraries, then ran `cargo fmt --check`,
`cargo clippy -- -D warnings` (warnings count as errors), `cargo test` and
a release build. The built program was attached to the run as the Linux
artifact, `ferrous-lineup-linux`.

> **Today:** `.github/workflows/ci.yml` still has one `check` job, now a
> matrix of Linux, macOS and Windows. Each runner uploads its own
> artifact (`ferrous-lineup-linux`, `ferrous-lineup-macos` and
> `ferrous-lineup-windows`), and the Linux runner still checks
> formatting. The book is not a job in that workflow: it has its own,
> [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/book.yml),
> where **build** makes the website and the PDF from `book/` and
> **deploy** publishes them on GitHub Pages (see [The book online and as
> a PDF](../step-9/book-online.md)).

A pull request shows a green check when the job passes. Step 4 adds macOS
and Windows to the matrix.
