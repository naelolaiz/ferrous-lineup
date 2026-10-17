# Exercises

Try these on your own branch. Each one practices something from this step.
Run `cargo test` and `cargo clippy` after each change.

## 1. Flat names (lineup-core, easy)

`Note`'s `Display` always uses sharps: `C#4`. Add a method
`fn name_with_flats(self) -> String` that returns `Db4` instead, and a test
for it.

*Practices:* arrays, `impl` blocks, `format!`, unit tests.

## 2. Transpose (lineup-core, easy)

Add `fn transpose(self, semitones: i8) -> Note` that moves a note up or
down, staying within 0..=127. Write tests for both directions and for the
edges (transposing `Note::MAX` up should stay at `Note::MAX`).

*Hint:* convert to `i16` before adding, then clamp, then convert back.
Look up `i16::clamp` and the `as` keyword.

*Practices:* casts between number types, overflow.

## 3. Another waveform (lineup-audio, medium)

Add a `Waveform` enum with `Sine` and `Square` variants to `synth.rs`, a
`waveform` field to `TestTone`, and a `match` in `next_sample` that picks
the formula. A square wave is `1.0` when `phase < 0.5` and `-1.0` otherwise.

Then add a `Command::SetWaveform(Waveform)` and a combo box in the UI
(`egui::ComboBox`). Remember the enum must be `Copy` to travel through the
ring buffer easily.

*Practices:* enums, `match`, `derive`, following data from UI to audio.

## 4. A reset button (lineup-app, easy)

Add a "Reset" button next to the volume slider that sets the note back to
A4 and the volume back to 0.3. Make sure the audio thread hears about it.

*Practices:* immediate-mode UI, sending commands.

## 5. Break it on purpose (any crate)

Learning to read compiler errors is half of learning Rust. Try each of
these, read the error, then undo it:

- In `engine.rs`, call `build_stream::<f32>(&device, config, consumer)`
  twice in a row, so `consumer` is used after it was moved.
- Remove `move` from the audio callback closure.
- Add a variant `SetName(String)` to `Command`. Why does `#[derive(Copy)]`
  stop working?
- Make `Note`'s field `pub` and write `Note(200)` in `app.rs`. Why does
  that compile, and why is it a bad idea?

*Practices:* ownership, `move` closures, `Copy` vs. non-`Copy` types,
privacy.
