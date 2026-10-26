# Exercises

## 1. A low cowbell (lineup-audio, easy)

`Drum::Cowbell` and `drums::cowbell` are already in the app: step 8 added
the cowbell with the rest of the rhythm kit. Read `cowbell`: two square
waves at 540 and 800 Hz, a high-pass at 400 Hz, a low-pass at 3 kHz, and
two decays.

Then extend it. Make the two frequencies parameters
(`cowbell_at(sample_rate, low, high)`) and add a `Drum::LowCowbell` an
octave below. The compiler lists every `match` to update in
`drums::render`, `Drum::name` and `general_midi_drum`; add the new drum to
`Drum::ALL` too (and its length), or the menus and the drum tests will not
see it. Adding a drum also changes which projects older versions can open:
read `format_for` and `is_new_in_format_3` before you keep it.

*Practices:* enums, exhaustive `match`, closures, a change that crosses
crates.

## 2. Clear a row (lineup-app, easy)

Right-clicking one step already clears it (`RackAction::ClearStep`). Add
"Clear steps" to a channel's name menu in `channel_menu`, with a new
`RackAction::ClearRow { channel }`. In `LineupApp::apply`, call
`edit_pattern` to set every step of that channel off, without sending the
audio thread a command by hand: the edit's diff sends the new notes. Why
is one whole-row edit better than a loop of single-step edits?

*Practices:* the action pattern, keeping project and audio in sync.

## 3. Paint steps by dragging (lineup-app, medium)

In the app you can click one step at a time. Give
`widgets::step_button` a `Sense::click_and_drag()` and make `Rows::steps`
paint every step the pointer passes over. `Rows` is rebuilt every frame
and only returns actions, so store the first step and whether it was
being switched on or off in `LineupApp` (or egui memory) and pass it in.

Add `RackAction::SetStep { channel, step, on }` rather than reusing
`ToggleStep`, so a step the pointer crosses twice does not flip back.
Check `response.hovered()` together with
`ui.input(|i| i.pointer.primary_down())`, and stop painting when the
pointer leaves the row.

*Practices:* immediate-mode state, egui input, the action pattern.

## 4. Swing, already built (lineup-core, medium)

Swing is in the app now: `lineup-core/src/swing.rs` delays the second
eighth of every beat as it *plays* the notes, and step 9 adds the control.
Read `delay`, `swung` and `Project::played_notes`.

Then extend it. Today it swings eighth notes. Add a sixteenth-note
variant: within each beat, delay the second sixteenth of every pair (the
"e" and the "a" of "1 e & a") by a smaller amount, chosen by a second
field or a `SwingUnit` enum. Write a test that checks pairs of steps still
take the same total time, and that the written notes are untouched. What
else would the new setting touch: the project file, the control, the MIDI
export?

*Practices:* arithmetic with ticks, unit tests with exact numbers,
keeping playback and editing apart.

## 5. Velocity, already built (lineup-core and lineup-app, harder)

Notes carry velocity now: `NoteEvent` has `velocity: u8`, and the piano
roll's `velocity_lane` edits it (`piano_roll.rs`). Read how the lane turns
height into `1..=127` and sends a `RollAction::ReplaceNotes`.

Then extend it with a velocity ramp: a tool that takes the selected notes
and spreads a crescendo or diminuendo between the first and last. Write
the interpolation in `lineup_core::tools`, next to `transpose` and
`snap_to_scale`, with tests for a rising, falling and single-note
selection. Which type carries the new operation from the UI: a new
`RollAction`, a `Tool`, or both? Where should the ramp run so it is one
undo step?

*Practices:* arithmetic on `u8`, following an action through the
workspace, keeping logic out of the UI.

## 6. Think about it

- Why would `Command::InsertChannel { instrument: Box<Instrument>, ... }`
  be worse than sending the instrument by value? (`Instrument` already
  boxes its voice arrays; what would a second box buy?)
- What would happen if the UI forgot to call `collect_garbage`?
- `AudioEngine::playhead` uses `Ordering::Relaxed`. Could the UI ever see
  a step number that was never played?
