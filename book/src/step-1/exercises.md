# Exercises

Try these on your own branch. Each one practices something from this step.
Run `cargo test` and `cargo clippy` after each change.

> These exercises target today's `main`, not the step-1 snapshot. The
> test tone and `synth.rs` were replaced in step 2; the code below uses
> the Lineup Osc in `instrument.rs` instead.

## 1. Flat names, the existing way (lineup-core, easy)

`Note` already knows both spellings: `Note::new(61).name(Spelling::Flats)`
is `Db4`, while `Display` still writes `C#4`. Read `Spelling` and the two
`name` methods in `pitch.rs`.

Write `fn flat_name(self) -> String` that uses that machinery, with a test
for a sharp black key, a flat one and a natural note. Then add a
`Spelling::Auto` variant that picks sharps for C# and F# and flats for the
other black keys (as `NoteName::chromatic` does), and make `name` handle
it. Which matches does the compiler ask you to update, and why can
`Spelling` stay `Copy`?

*Practices:* enums, `match`, `impl` blocks, tests.

## 2. Transpose, clamped (lineup-core, easy)

`Note::transpose(self, i32) -> Option<Note>` is already there; it returns
`None` when the move would leave 0..=127. Read it and its tests.

Add `fn transpose_clamped(self, semitones: i16) -> Note`: the same move,
but stop at `Note::MIN` or `Note::MAX` instead of returning `None`. Write
tests for both directions and the edges. Why is `Option` the better fit
for a piano roll, and when is clamping better? Look up `i16::clamp`.

*Practices:* casts between number types, `Option` versus saturation,
overflow.

## 3. A new wave for the Lineup Osc (lineup-audio, medium)

At this step the synth was `TestTone`; today it is `Synth` in
[`instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs).
Read `Wave::at` and `Wave::gain` in
[`timbre.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/timbre.rs),
then `SynthVoice::next_sample` in `instrument.rs`.

Add a `Wave::Pulse` variant that is `1.0` for the first quarter of the
cycle and `-1.0` for the rest. The compiler lists the matches to update
in `Wave::at` and `Wave::gain`; then give one group of General MIDI
programs a pulse in `Timbre::for_program` (the square-wave reeds are a
natural choice). Add a test in `timbre.rs` for the four quarter points of
the cycle. Why does `Eq` still work on a fieldless variant, and what
would break if `Pulse` carried an `f32` width?

*Practices:* enums, `match`, `derive`, following a General MIDI program
from the channel rack to the audio thread.

## 4. A reset button (lineup-app, easy)

The test tone's note and volume went with the test tone. The same idea
lives in the toolbar's master volume: add a "Reset" button next to it that
sets the master back to `lineup_core::Mixer::default().master` (0.8).

Call `LineupApp::set_master_volume` in `crates/lineup-app/src/app/mixer.rs`
rather than assigning the field, so the change is undoable and the audio
thread gets `Command::SetMasterVolume`. Run the app and check that the
slider moves.

*Practices:* immediate-mode UI, sending commands, routing a change
through `edit`.

## 5. Break it on purpose (any crate)

Learning to read compiler errors is half of learning Rust. Try each of
these, read the error, then undo it:

- In `engine.rs`, call
  `build_stream::<f32>(&device, config.clone(), audio_thread, Arc::clone(&problems))`
  and then
  `build_stream::<f32>(&device, config, audio_thread, Arc::clone(&problems))`,
  so the second call fails because `audio_thread` was moved.
- Remove `move` from the audio callback closure.
- Add a variant `SetName(String)` to `Command`. Why can `Command` not be
  `Copy`? Which variants already make that impossible, and what does the
  compiler say when you ask for `Clone`?
- Make `Note`'s field `pub` and write `Note(200)` in `app.rs`. Why does
  that compile, and why is it a bad idea?

*Practices:* ownership, `move` closures, `Copy` vs. non-`Copy` types,
privacy.
