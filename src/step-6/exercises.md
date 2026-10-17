# Exercises

## 1. Humanize velocities (lineup-core, easy)

Notes that are all played exactly as hard sound mechanical. Add
`tools::humanize(notes, amount, seed) -> Vec<NoteEvent>`, which moves
each velocity up or down by a random amount of at most `amount`, kept
within 1..=127, and a "Humanize velocity" item in the Tools menu.

Ferrous Lineup has no random number crate, and does not need one: a tiny
generator such as *xorshift* is three lines of bit shifts on a `u32`
(look it up, and use `wrapping_mul` or plain shifts so nothing
overflows). Why does the function take a `seed` instead of making one
up? Think about how you would test it, and what the same seed should
give twice in a row. Where does the piano roll get a different seed each
time?

*Practices:* pure functions with an explicit seed, bit operations,
`clamp`.

## 2. Copy and paste (lineup-app, easy)

Make Ctrl+C copy the selected notes and Ctrl+V paste them at the last
step the pointer was over (`last_step`). Store the copied notes with
their starts counted from the earliest one, so they can be pasted
anywhere. The paste can be a `ReplaceNotes` with an empty `old`: what
does `replace_notes` do when a pasted note lands on an existing one, and
is "all or nothing" the right behaviour for a paste? Should the pasted
notes be selected afterwards, so they can be dragged into place?

Watch out for the order of shortcuts: is Ctrl+C a problem for anything
already in the list?

*Practices:* `KeyboardShortcut`, `Option<usize>`, reusing an action.

## 3. Name chords over a beat (lineup-core and lineup-app, medium)

After "Arpeggiate chords", the chord lane names each step on its own:
"A", "A+C", "A5"... You can see this in the arpeggio screenshot of this
step. Your ear hears A minor, because it remembers the notes of the
last beat. Add a choice to the chord lane, "per step" or "per beat",
and in "per beat" mode name each beat (4 steps) from every note that
*starts* in that beat (or is still held when it begins).

Write the gathering part in `lineup-core` as a function on `Pattern`, so
it can be tested: an arpeggio of C, E, G over one beat must be named C.
What should happen when a beat holds the end of one chord and the start
of the next?

*Practices:* ranges and `flat_map`, deciding what "at the same time"
means, tests on music you know.

## 4. Borrowed chords (lineup-core, medium)

Play C, then Bb major, in C major. Bb is not in the key, so
`next_chords` suggests nothing. But Bb major is a very common chord in C
major songs: it is borrowed from C *minor*, the parallel key, where it
is VII. Borrowed chords usually go back to the tonic (Bb to C), or on to
other borrowed chords (Bb to Ab, F to Fm).

Extend `next_chords`: when the root is not in the key, try the parallel
key (`Scale::new(scale.root, ScaleKind::NaturalMinor)` for a major key,
and the other way around), and give its suggestions with a reason that
says where the chord comes from. Write the tests first: Bb and Ab in C
major, and A minor in C minor (borrowed from C major, where it is vi).

The `degree` of a suggestion is used to light up a button. What should
the piano roll do with a suggestion whose chord is not one of the seven
buttons? Maybe the hint line is enough.

*Practices:* falling back with `or_else`, tests as a specification.

## 5. Draw in key (lineup-app, medium)

Step 3's exercise 5 asked for a "snap to scale" switch. Step 6 built
`tools::nearest_in_key` for the Tools menu; use it to add a "Draw in
key" checkbox next to Draw. While it is on, new notes, single notes
being dragged and selections being dragged stay in the key: each note
goes to the nearest note of the scale. The outline of what a click
places must show the snapped notes too, so the outline and the click still agree. How does a
selection drag snap: each note on its own, or the whole group by the same
amount? Try both and listen to a chord being dragged.

*Practices:* reusing `lineup-core` functions in the UI, keeping the
preview and the action in step.

## 6. Strumming (all three crates, harder)

The [step's introduction](README.md#what-is-not-here-strumming) explains
why strumming was left out: notes start on whole steps, and a strum
needs offsets of a few milliseconds. There are two ways to add it. Work
out both before writing any code.

- **In the notes.** Give `NoteEvent` an `offset` inside its step, in
  *ticks* (many studios use 96 or 960 ticks per beat). The sequencer
  already counts samples, so it can start a note `offset` ticks into its
  step. A "Strum" tool then gives each note of a chord a growing offset,
  low to high. The file needs the new field, with `#[serde(default)]`
  for old files (step 4's exercise 3 shows how), and `diff` must notice
  offset changes.
- **In the instrument.** Give each channel a "strum" setting in
  milliseconds. When several notes of a channel start in the same
  frame, the audio thread starts them that many milliseconds apart,
  lowest first. Nothing in the pattern changes.

Which one lets you strum one chord and not the next? Which one also
makes it possible to place notes off the grid ("swing", or a drum hit
slightly early)? Which one changes fewer files? Real studios have both.

*Practices:* designing across crates, evolving a file format,
sample-accurate scheduling.

## 7. Think about it

- The selection holds notes by value, which works because a channel
  never holds two equal notes. Imagine that rule did not exist (two
  identical notes on top of each other). What would go wrong with the
  selection, with `replace_notes`, and with the velocity lane? Would
  giving every note a permanent id (`NoteId(u32)`) fix it, and what
  would ids cost in the file and in `diff`?
- `tools::transpose` refuses to move a chord if one note would leave the
  MIDI range. Another design would move the notes that can move and
  leave the rest. When would that be better? Is there a case where
  refusing confuses the user, and how could the piano roll tell them why
  nothing happened?
- `next_chords` keeps its knowledge in tables of degrees. Most of the
  table could be computed instead from a few rules (roots falling a
  fifth, steps up to the dominant). Which entries would the rules miss?
  Is a table that says plainly what it means better than rules that are
  shorter?
- `contains` on a `Vec` looks through every item. The piano roll calls it
  for every note, every frame. At what size of pattern would a
  `HashSet<NoteEvent>` be worth it? How would you measure, instead of
  guessing? (Look up `std::time::Instant`.)
- egui matches Shift+↑ as ↑. Why might a library choose that on purpose?
  Think of keys that need Shift on some keyboard layouts, like `+` or `?`.
