# Exercises

## 1. More chords (loom-core, easy)

*Step 6 adds both, and five more; try it yourself before reading
[its chapter](../step-6/harmony.md#seven-new-chords).*

Add `ChordQuality::Dominant13` (0, 4, 7, 10, 14, 21) or `Minor11`
(0, 3, 7, 10, 14, 17). The compiler lists every `match` to update. Then
run the tests: `no_two_qualities_have_the_same_notes` will tell you if
your new chord collides with an existing one. As pitch classes, why do
big chords often look like other chords built on another root?

*Practices:* exhaustive `match`, letting tests guard your data.

## 2. Show the scale degree on the keys (loom-app, easy)

The keyboard marks notes of the key with a dot. Draw the degree number
instead (1 for the root, 2, 3...), using `Scale::degree_of`. Musicians
use these numbers all the time ("the 5 chord", "a flat 7").

*Practices:* `Option` in drawing code, `painter.text`.

## 3. Velocity with the arrow keys (loom-app, medium)

*Step 6 edits velocity in a lane under the grid instead, and uses the
arrow keys to move notes; [its chapter](../step-6/piano-roll.md#the-velocity-lane)
shows how. The exercise is still a good one on its own.*

`NoteEvent` has a `velocity`, but nothing edits it yet. When the pointer
is over a note, make the up and down arrow keys change its velocity
(`ui.input(|i| i.key_pressed(egui::Key::ArrowUp))`), send a
`ReplaceNote`, and draw quieter notes with a paler colour
(`Color32::gamma_multiply`). Keep the velocity within 1..=127: what does
`u8::saturating_add` do, and is it enough?

*Practices:* struct update syntax, reading egui input.

## 4. Inversions (loom-core, medium)

*Step 6 builds this; try it yourself before reading
[its chapter](../step-6/tools.md#inversions).*

Add an "Inversion" menu next to Draw (0, 1 or 2): inversion 1 moves the
lowest note of a stamped chord up an octave, inversion 2 does it twice.
Write it as a function `invert(notes: Vec<Note>, times: u8) -> Vec<Note>`
in `chord.rs`, with tests. Then check the chord lane: does it show C/E
for the first inversion of C?

*Practices:* `Vec` manipulation (`remove`, `push`, `sort`), tests that
use `analyze` to check your own output.

## 5. Snap to scale (loom-app, medium)

*Step 6 builds the "nearest note in the scale" function, for a Tools
menu item rather than a checkbox; see [its chapter](../step-6/tools.md#the-nearest-note-of-the-key-snapping).
Step 6's exercise 5 adds the checkbox.*

Add a "Snap to scale" checkbox. When it is on, a note dropped on a row
outside the key moves to the nearest note of the key. Write the "nearest
note in the scale" function in `loom-core`, where it can be tested
without a window. What should happen exactly halfway between two scale
notes?

*Practices:* searching with iterators (`min_by_key`), keeping logic out
of the UI.

## 6. A legato synth (loom-audio, harder)

Give the Loom Osc a "mono / legato" mode: only one voice plays, and a new
note changes that voice's pitch without restarting the envelope. Add a
`glide` time so the pitch slides from the old note to the new one (use
`dsp::Smoothed` for the frequency). How does `pick_voice` change? Which
parts of the `Stage` state machine are skipped?

*Practices:* state machines, reusing small DSP pieces.

## 7. Think about it

- `find_chord` tries the bass note first. Can you find a set of notes
  where this gives a surprising name? Is there a better rule?
- `SetNotes` sends the whole list for every change. At what size would
  that become a problem, and what would you send instead?
- The piano roll refers to notes by their index in the list. What could
  go wrong if the list were kept sorted by start time, and a note was
  moved while being dragged?
- Why is `Garbage` an enum, rather than two separate trash queues?
  What would two queues cost, and what would they gain?
