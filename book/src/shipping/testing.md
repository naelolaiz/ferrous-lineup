# How the code is tested: unit tests, window tests, traces

Files: the `mod tests` at the end of most files in `crates/*/src`,
[`crates/lineup-core/tests/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/tests),
[`crates/lineup-listen/tests/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-listen/tests)
and [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

Ferrous Lineup has more than 1,300 tests: about 900 in `lineup-core`,
150 in `lineup-audio`, 260 in `lineup-app` and a few dozen in the other
two crates. Every chapter of this book ends with the tests of what it
describes. This one is about the kinds of test, why each exists, and
how to run them.

## Running them

```sh
cargo test                       # every crate
cargo test -p lineup-core        # one crate
cargo test frequency             # only tests whose name contains "frequency"
cargo test -- --nocapture        # show what the tests print
```

A test is an ordinary function marked `#[test]`. It passes if it
returns, and fails if it panics, which is what `assert!`, `assert_eq!`
and `unwrap` on an error do. `cargo test` builds the crates in a test
mode, runs every test in parallel threads, and lists the ones that
failed with their messages.

The tests normally run in a **debug** build, and the continuous
integration runs them that way too. That matters: in a debug build an
`+` or a `*` that overflows panics, and every `debug_assert!` is
checked, while a release build wraps the number around and skips the
`debug_assert!`s. A bug of that kind passes in release and fails in
debug, so debug is where tests should run. The few crates that would
make a debug build too slow to play music, such as the SoundFont
synthesizer, are built optimized even then (see [Debug
builds](../sound/soundfonts.md#debug-builds)).
Long reports, below, are the exception: they run with `--release`.

## Unit tests, next to the code

Most tests sit at the end of the file they test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
```

`#[cfg(test)]` leaves the module out of every build except `cargo
test`, so the tests cost the program nothing. `use super::*` brings in
everything of the file, private items too, since a child module can see
its parent's private items. So a unit test can check a helper the rest
of the program never sees.

Sound can be tested without listening. A sine wave crosses zero twice
in every cycle, so counting the times the signal changes sign measures
its pitch:

```rust
fn sign_changes(samples: &[f32]) -> usize {
    samples
        .windows(2)
        .filter(|pair| (pair[0] < 0.0) != (pair[1] < 0.0))
        .count()
}
```

`windows(2)` gives every pair of neighbouring samples, `filter` keeps
the pairs on either side of zero, and `count` counts them. One second of
A4 must change sign about 880 times, twice 440:

```rust
// A4 is 440 Hz: about 880 sign changes in one second.
assert!((860..=900).contains(&changes), "got {changes}");
```

The range allows for the attack at the start of the note. Iterators are
lazy: nothing runs until `count` asks for values, and the compiler turns
the chain into a loop as fast as one written by hand.

## Tests over data

Much of Ferrous Lineup is data: 343 songs, 215 harmony styles, 139
rhythms, the twelve-tone and micromode styles. Tests read all of it, so
a mistake in a table fails before anyone hears it: a bar that does not
add up, a chord that cannot be read, a name used twice, a style whose
rhythm does not exist (see [Testing it](../band/library.md#testing-it)
for the library and [Testing it](../songs/demos.md#testing-it) for the
songs). Their messages name the entry, so a failure says which song to
open.

## Tests in `tests/`, from the outside

A file in a crate's `tests/` folder is an **integration test**: a
program of its own that uses the crate as any other crate would, so it
sees only what the crate makes public.

| File | What it checks |
|------|----------------|
| `lineup-core/tests/chord_reading.rs` | how well the chord sheet names what the hardest demos play: the Augurs chord, the Petrushka chord, a stack of fourths |
| `lineup-core/tests/key_finding.rs` | how many bars of the demos the key finder hears in their written key, and that it finds clear modulations on their bar |
| `lineup-core/tests/damaged_files.rs` | that a damaged project, MIDI file or chord sheet ends in an error or a project, never in a crash |
| `lineup-listen/tests/key_changes.rs` | how well key changes are found in demos rendered to audio |

### Damaged files

`damaged_files.rs` reads hundreds of damaged copies of good projects,
MIDI files and chord sheets, as a **fuzzer** does, and checks that each
ends in a project or an error, never in a crash. How it damages them,
and the overflow it found that only a debug build sees, are in
[Damaged files never crash the
app](../program/saving.md#damaged-files-never-crash-the-app).

### Reports

Some tests measure rather than check. `report` in `key_finding.rs` and
`chord_reading.rs` prints a table, song by song, of how well each demo
is heard. They are marked `#[ignore]`, so `cargo test` skips them, and
run on their own:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

The ordinary tests next to them keep the figure from falling:
`most_bars_of_the_demos_are_heard_in_their_key` fails if fewer than 87%
of the bars are heard in their key. The report says why a change moved
that figure; the test makes sure it never moves the wrong way unnoticed
(see [How well it hears](../theory/hearing.md#how-well-it-hears)).

## Window tests

egui draws the whole window again on every frame, from the app's state
(see [Immediate mode](../program/egui.md#immediate-mode)). That makes the window itself easy to
test: a test can run frames without a screen, feed them clicks and keys,
and read what was drawn. `ux_tests.rs` keeps a `Screen` for that:

```rust
fn new() -> Self {
    let ctx = egui::Context::default();
    theme::apply(&ctx);
    ctx.all_styles_mut(|style| style.animation_time = 0.0);
    let mut app = LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()));
```

The app starts as it does without a sound card (`Audio::Unavailable`),
with animations off so one frame shows the end of every animation, and
it never reads or writes a real recovery copy or downloads a SoundFont.
`draw` runs one frame with the events given; egui returns the shapes it
would paint, and `text_rect` finds a label among them:

```rust
fn text_rect(output: &FullOutput, label: &str) -> Option<Rect> {
    output
        .shapes
        .iter()
        .rev()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.text() == label => {
```

`click` draws a frame, finds the label, and sends a pointer press and
release at its centre. So a test reads like a person using the app:

```rust
fn f1_and_the_help_menu_show_the_keyboard_shortcuts() {
    let mut screen = Screen::new();
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Keyboard shortcuts").is_none());

    screen.key(Key::F1);
    let output = screen.draw(vec![]);
    assert!(screen.app.shortcuts_open);
```

126 tests in that file open demos, apply styles, click the mixer and
the rack, resize windows and restart the app. Smaller ones sit next to a
single widget, such as the time signature picker or the playlist, with a
`frame` helper that draws only that widget in a window of its own.

## Traces: what was played

A test of the sequencer can count notes, but a song's sound is what the
player sends to the instruments, step by step, at the tempo of the
moment. The [playback trace](../sound/trace.md) writes exactly that,
one line per note, playing headless as **File > Export WAV…** does. It is the
tool for a question a unit test cannot ask easily, such as "does this
demo still play the same as before?": trace it before and after a
change and compare the two tables. MIDI is tested the same way, without
a synthesizer: a song is exported to bytes, read back and compared (see
[Testing without a synth](../sound/midi.md#testing-without-a-synth)).

## Formatting and lints

Two more tools keep the code tidy, and the continuous integration runs
both:

- `cargo fmt` rewrites the code in the standard Rust style, so nobody
  argues about where braces go. `cargo fmt --all --check` only says
  whether anything would change.
- `cargo clippy` is Rust's **linter**. It knows hundreds of patterns that
  compile but could be simpler, faster or more correct, and explains each
  one. `-D warnings` makes every warning an error.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

`--all-targets` lints the tests and build scripts too. The continuous
integration installs the newest stable Rust on every run, and a new Rust
often brings new lints, so run `rustup update` before you lint: a
lint your older Clippy does not know will fail the run anyway.
