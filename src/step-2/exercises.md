# Exercises

## 1. A cowbell (lineup-audio, easy)

Add `Drum::Cowbell` to `lineup-core` and a `cowbell` function to
`drums.rs`. A classic cowbell is two square waves at about 540 Hz and
800 Hz added together, band-passed, with a fast decay (tau around
0.06 s). The compiler will tell you every `match` you need to update;
follow its errors until it builds.

*Practices:* enums, exhaustive `match`, closures.

## 2. Clear a row (lineup-app, easy)

Add "Clear steps" to a channel's right-click menu. You need a new
`RackAction` variant, a new arm in `LineupApp::apply`, and one
`Command::SetStep` per step that was on.

*Practices:* the action pattern, keeping project and audio in sync.

## 3. Paint steps by dragging (lineup-app, medium)

In the original studio you can hold the mouse button and sweep across
steps to switch many on at once. Give `step_button` a
`Sense::click_and_drag()` and look at `response.hovered()` together with
`ui.input(|i| i.pointer.primary_down())` to paint every step the mouse
passes over. Remember whether the first step was being switched on or
off, and apply the same to the others.

*Practices:* immediate-mode state, egui input.

## 4. Swing (lineup-core, medium)

Swing delays every second step a little, which gives a pattern a
shuffling groove. Add a `swing: f32` (0.0 to 0.5) to the `Sequencer`:
after an even step, the next step comes `frames_per_step × (1 + swing)`
frames later, and after an odd step `frames_per_step × (1 - swing)`
frames later, so pairs of steps still take the same total time. Write a
test that checks the frames, then add a knob to the transport bar.

*Practices:* arithmetic with `f64`, unit tests with exact numbers.

## 5. Velocity (lineup-core and lineup-audio, harder)

Steps are only on or off. Change `StepRow` so each step stores a
velocity, `0` for off and `1..=127` for loudness, and make instruments
play quieter for lower velocities. Which types have to change? What does
the compiler tell you about `[u8; MAX_STEPS]` versus `[bool; MAX_STEPS]`?

*Practices:* changing a core type and following the compiler through a
whole workspace.

## 6. Think about it

- Why would `Command::AddChannel { instrument: Box<Instrument>, ... }` be
  worse than sending the instrument by value?
- What would happen if the UI forgot to call `collect_garbage`?
- `AudioEngine::playhead` uses `Ordering::Relaxed`. Could the UI ever see
  a step number that was never played?
