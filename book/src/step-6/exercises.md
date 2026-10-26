# Exercises

Some of these tools were built later in the book. Where that happened,
the exercise points at the finished code first, then asks for an
extension that is still missing. Copy, cut, paste and duplicate, for
example, are already in the piano roll: read `Tool::Copy` and
`clipboard_tool` if you want to see how they reuse `ReplaceNotes`.

## 1. Humanize velocities (lineup-core, easy)

Notes that are all played exactly as hard sound mechanical. Add
`tools::humanize(notes, amount, seed) -> Vec<NoteEvent>`, which moves
each velocity up or down by a random amount of at most `amount`, kept
within 1..=127, and a "Humanize velocity" item in the Tools menu.

Ferrous Lineup has no random number crate, and does not need one: a tiny
generator such as *xorshift* is three lines of bit shifts on a `u32`
(look it up, and use `wrapping_mul` or plain shifts so nothing
overflows). `lineup_core::harmony::ToneRow::random` and the ear
training's questions use such generators. Why does the function take a
`seed` instead of making one up? Think about how you would test it, and
what the same seed should give twice in a row. Where does the piano roll
get a different seed each time?

*Practices:* pure functions with an explicit seed, bit operations,
`clamp`.

## 2. Retrograde a phrase (lineup-core and lineup-app, medium)

Playing a phrase backwards is a classic way to make a variation, and the
piano roll has no tool for it. Add
`tools::retrograde(notes: &[NoteEvent], span: usize) -> Vec<NoteEvent>`:
within `span` steps, each note keeps its pitch and length but its start
is mirrored, so a note at step 0 goes to the end of the span and a note at
step 3 goes near the beginning.

Work out what "mirrored" means for a note of length 2 in a 16-step span:
does its *start* mirror, its *end*, or both? Which choice keeps the phrase
inside the span and sounds most like the original played backwards? What
happens to `nudge` and `trim`? Add a "Retrograde" item to the Tools menu,
with a test on a four-note phrase you can sing. The result goes through
`replace` like every other tool.

*Practices:* writing a pure function, `NoteEvent` fields, deciding a
musical rule before coding it.

## 3. Name chords over a beat (lineup-core and lineup-app, medium)

After "Arpeggiate chords", the track's own chord row names each step on
its own: "A", "A+C", "A5"... You can see this in the arpeggio screenshot
of this step. Your ear hears A minor, because it remembers the notes of
the last beat.

The "All" row already does something like this: `heard_by_beat` gathers
every note that sounds during a beat and names the harmony once per beat.
What is missing is the same choice for the track's row. Add a small
"per step / per beat" switch, and in "per beat" mode name each beat from
every note that *starts* in that beat (or is still held when it begins).

Write the gathering part in `lineup-core` as a function on `Pattern`, so
it can be tested: an arpeggio of C, E, G over one beat must be named C.
What should happen when a beat holds the end of one chord and the start
of the next?

*Practices:* ranges and `flat_map`, deciding what "at the same time"
means, tests on music you know.

## 4. The Neapolitan chord (lineup-core, medium)

Chords borrowed from the parallel minor are already in the table: iv,
ii°, bIII, bVI and bVII all suggest their usual next chords. Play C, then
Bb major, in C major, and `next_chords` answers.

The **Neapolitan** is still missing: bII, the major chord on the lowered
second degree (Db in C). It usually goes to V, or directly home to I, and
it is common before a cadence. Add a `Slot` with root 1 and
`ChordFamily::Major`, with those two moves and reasons, to `MAJOR` and
to `MINOR` (both keys borrow it). Which other field of `Suggestion` has
to be right for the "Chords in C major" buttons to light up, and what
should the piano roll do with a suggestion whose chord is not one of the
seven buttons?

Write the tests first: Db to C and Db to G in C major; then the same
Neapolitan in a minor key, where bII is also borrowed. Does the note Db
belong to C minor's scale, or is it outside that key too?

*Practices:* extending a data table, tests as a specification, thinking
about whether the target is a degree of the key.

## 5. Note-length tools (lineup-core and lineup-app, medium)

The Draw menu has a "Length" setting for *new* notes, but there is no way
to change the length of notes already written. Add two tools:

- `tools::set_note_length(notes, steps)` makes every note `steps` steps
  long, keeping its start and pitch, and leaves room at the end of the
  pattern;
- `tools::legato(notes, pattern_length)` makes each note last until the
  next note starts, so a chord or melody line joins up.

`NoteEvent` stores its length in whole steps plus `trim`, how many ticks
before the end of its last step it stops. A note shortened to one step
must have its `trim` kept below a step; a longer note that is shortened
must clear any `trim` that no longer fits. Add both to the Tools menu,
and check what happens when two notes would then overlap in the same
channel. Why is `set_note_length` not simply `note.length = steps`?

*Practices:* using `tidy_notes` correctly, note lengths and ticks,
keeping a pattern's rules.

## 6. A Strum tool, or a channel strum (all three crates, harder)

The [step's introduction](README.md#what-is-not-here-a-strum-tool)
explains what is missing: not off-grid timing, which `NoteEvent::nudge`
already provides, but a tool and a channel setting for your own notes.
The band's guitar in `crates/lineup-core/src/band/guitar.rs` is a working
example: it nudges each string a tick later than the one below. Work out
both designs before writing any code.

- **In the notes.** A Strum tool gives each note of a chord a growing
  `nudge`, from the lowest note up, a few ticks apart. Nothing else in
  `NoteEvent` changes: the file already saves `nudge`, `diff` already
  compares the notes the audio thread plays, and Quantize can undo it.
  The tool needs the chord's notes and a direction (down or up), and it
  must keep the notes inside their steps (a nudge is at most 23 ticks).
- **In the instrument.** Give each channel a "strum" setting in
  milliseconds. When several notes of a channel start in the same frame,
  the audio thread starts them that many milliseconds apart, lowest
  first. Nothing in the pattern changes, but `Channel`, the
  `InsertChannel` command, `project_commands` and the file format all
  need the new field.

Which one lets you strum one chord and not the next? Which one also makes
it possible to place notes off the grid ("swing", or a drum hit slightly
early)? Which one changes fewer files? Real studios have both.

*Practices:* designing across crates, evolving a file format,
sample-accurate scheduling.

## 7. Think about it

- The selection holds notes by value, which works because a channel
  never holds two notes in the same place. `same_place` compares pitch,
  start and `nudge`. Imagine that rule did not exist (two notes in
  exactly the same place). What would go wrong with the selection, with
  `replace_notes`, and with the velocity lane? Would giving every note a
  permanent id (`NoteId(u32)`) fix it, and what would ids cost in the
  file and in `diff`?
- `tools::transpose` refuses to move a chord if one note would leave the
  MIDI range. Another design would move the notes that can move and
  leave the rest. When would that be better? Is there a case where
  refusing confuses the user, and how could the piano roll tell them why
  nothing happened?
- `next_chords` keeps its knowledge in tables of root, family and moves.
  Most of the table could be computed instead from a few rules (roots
  falling a fifth, steps up to the dominant). Which entries would the
  rules miss? Is a table that says plainly what it means better than
  rules that are shorter?
- `contains` on a `Vec` looks through every item. The piano roll calls it
  for every note, every frame. At what size of pattern would a
  `HashSet<NoteEvent>` be worth it? How would you measure, instead of
  guessing? (Look up `std::time::Instant`.)
- egui matches Shift+↑ as ↑. Why might a library choose that on purpose?
  Think of keys that need Shift on some keyboard layouts, like `+` or `?`.
