# Exercises

## 1. More chords, one that is still missing (lineup-core, easy)

`Dominant13` and `Minor11` are already in `ChordQuality::ALL` now; step 6
added them and five more. Read `ChordQuality::intervals`, `symbol`,
`name` and `numeral_suffix`, then add a quality that is still absent, such
as `MinorMajor9` (0, 3, 7, 11, 14), and add it to `ChordQuality::ALL`
too. The compiler lists every `match` to update;
`no_two_qualities_have_the_same_notes` will tell you if the new pitch set
collides with an existing one. As pitch classes, why do big chords often
look like other chords built on another root?

*Practices:* exhaustive `match`, letting tests guard your data.

## 2. Show the scale degree on the keys (lineup-app, easy)

`paint_keys` in `piano_roll.rs` draws a dot on every note of the key. Draw
the degree number instead (1 for the root, 2, 3...), using
`Scale::degree_of`. A row already numbers its notes; keep that path as it
is. Musicians use these numbers all the time ("the 5 chord", "a flat 7").

*Practices:* `Option` in drawing code, `painter.text`.

## 3. Velocity from the keyboard (lineup-app, medium)

Velocity is already in the app: `NoteEvent` has a `velocity`, the piano
roll's `velocity_lane` edits it with the mouse, and
[step 6](../step-6/piano-roll.md#the-velocity-lane) explains the lane.
Read how `replace` sends a `RollAction::ReplaceNotes` and how the shortcut
list checks longer combinations first.

Then extend it: make Ctrl/Cmd+↑ and Ctrl/Cmd+↓ raise or lower the selected
notes by 8, clamped to 1..=127, as one undo step. Add the shortcuts beside
`KEY_UP`/`KEY_DOWN`, before them in the list so Ctrl+↑ is not eaten by "up
a semitone", and call `replace` with a label that names the change.

*Practices:* struct update syntax, reading egui input, `u8` arithmetic.

## 4. Inversions, and a drop-2 (lineup-core, medium)

`tools::inversion(pitches, times)` and `tools::invert(notes, up)` are
already in the app: the Draw menu's inversion list uses `inversion` for
stamped chords, and `Tool::Invert { up }` calls `invert`. Read `chords`,
`above`/`below` and `invert`.

Then extend it with a **drop 2** voicing: for each chord, lower the
second-highest note by an octave. Add it beside `invert` with a test like
`inverting_chords_up_and_down`, and return `None` when the note would fall
below `Note::MIN`. Does the chord lane still name C/E, or does the bass
change? Why?

*Practices:* `Vec` manipulation (`remove`, `push`, `sort`), tests that
check your own output.

## 5. Snap to scale, with a direction (lineup-core and lineup-app, medium)

`tools::nearest_in_key` and `tools::snap_to_scale` are already in the app,
and the piano roll's "Snap to scale" checkbox uses them. Read
`nearest_in_key`: it tries one semitone below, then above, then two, so
the lower note wins a tie.

Then extend it with `nearest_in_key_towards(note, scale, direction)`,
which breaks a tie in the direction the melody is moving. Call it from a
second checkbox or a Tools item, and write tests for notes exactly halfway
between two scale notes, both directions. Which existing test in
`tools.rs` pins the tie rule, and would the new function change its
answer?

*Practices:* searching with iterators (`min_by_key` or `find_map`),
keeping logic out of the UI.

## 6. A legato synth (lineup-audio, harder)

The Lineup Osc has no mono / legato mode yet. Give it one: only one voice
plays, and a new note changes that voice's pitch without restarting the
envelope. Add a `glide` time so the pitch slides from the old note to the
new one (use `dsp::Smoothed` for the frequency). How does `pick_voice`
change? Which parts of the `Stage` state machine are skipped?

*Practices:* state machines, reusing small DSP pieces.

## 7. Think about it

- `find_chord` tries the bass note first. Can you find a set of notes
  where this gives a surprising name? Is there a better rule?
- `SetNotes` sends the whole list for every change. At what size would
  that become a problem, and what would you send instead?
- The single-note drag still remembers a note by its index in the list
  (`Drag::Move { index, .. }`), while a selection remembers values. What
  could go wrong with the index if the list were kept sorted by start
  time, and a note was moved while being dragged?
- Why is `Garbage` an enum, rather than two separate trash queues?
  What would two queues cost, and what would they gain?
