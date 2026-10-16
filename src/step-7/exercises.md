# Exercises

## 1. Octave keys (loom-app, easy)

The typing keyboard covers C4 to E6. Add two keys that move it down and
up by an octave, such as `-` and `=` (`Key::Minus` and `Key::Equals`),
which play no note yet. Keep the shift between C1 and C7, and show it
in the MIDI menu ("Typing keyboard: C3 to E5").

The tricky case: hold Z, press `=`, then let go of Z. Which note must
end? Look at `typing_down`, which keeps the note each key played, not
only the key. Why was it written that way, when the note could be
worked out again from the key?

*Practices:* `Note` arithmetic with `checked_add_signed` (step 6),
remembering what was done instead of recomputing it.

## 2. Softer and louder typing (loom-app, easy)

Typed keys always play at `DEFAULT_VELOCITY`, so recorded takes from
the typing keyboard all have the same velocity, while a MIDI keyboard's
keep how hard each key was struck. Add a "Typing velocity" slider to the
MIDI menu (`egui::Slider::new(&mut value, 1..=127)`), and make Shift
play at 127 while it is held.

Shift does not change `physical_key`, so the piano keeps working. But
look at the filter in `typing_keyboard`: does a key that went down with
Shift come up properly when Shift is let go first?

*Practices:* egui sliders, keyboard modifiers.

## 3. A metronome (loom-audio, medium)

Recording is much easier with a click on every beat. Add
`Command::SetMetronome(bool)` and a "Metronome" checkbox next to Rec.
The audio thread plays a short click on each step where
`step % STEPS_PER_BEAT == 0`, louder on the first step of each bar
(`STEPS_PER_BAR`). Make the click with a few lines of synthesis, the way
[`drums.rs`](../step-2/instruments.md) makes its sounds, so no sample
file is needed.

The click must not be in the WAV export, or in the MIDI export. Where
does each of those come from, and which one would get the click by
accident if it lived in a channel?

*Practices:* a new command end to end, real-time sound without
allocating.

## 4. Notes across the loop (loom-core, medium)

[`recorded_note`](recording.md#what-it-cannot-know) has two limits.
A note held across the end of the loop is cut there, and a key held for
more than a whole loop is recorded as a short note.

- Make a key held across the loop end record **two** notes: one to the
  end of the pattern, and one from step 0 for the rest. The function
  then returns a `Vec<NoteEvent>` (or an array and a count). Write the
  tests first: (14, 2) in a 16-step pattern gives (14, 2) and (0, 2).
- For long notes, the position alone is not enough. Have the audio
  thread also count how many times the loop has gone round, or publish
  the number of steps played since Play instead of the position in the
  loop. Which is simpler to share through one `AtomicU64`? Which one can
  `finish_recording` still use?

*Practices:* changing a function's return type and its callers, tests
as a specification.

## 5. Record in song mode (all three crates, harder)

Rec switches to pattern mode, because a take goes into the pattern that
loops. In song mode, the position is a step of the *song* (the
sequencer's length is the song's length then), so a key could go into
the clip playing at that moment, at `step - clip.start`.

Work out the questions before writing code. Two clips can play at once,
on different tracks: which one gets the note? What if no clip plays
there, should a new clip be made? A clip may be played several times in
the song: recording into one copy changes all of them, because they are
the same pattern. Is that what you would want? How do the other
studios you know handle it?

*Practices:* designing behaviour before code, the playlist from step 4.

## 6. Import MIDI files (loom-core, harder)

Ferrous Loom can write `.mid` files. Add `midi_file::read(bytes) ->
Result<Project, MidiFileError>`, and **File > Import MIDI…**. The tests
of `midi_file.rs` already have a small reader to start from, but files
made by other programs need more:

- the header's ticks per beat can be anything (480 and 960 are common),
  so ticks must be converted to steps by rounding,
- *running status* ([the first chapter](messages.md#reading-a-message-slice-patterns)):
  a message may leave out its status byte when it repeats the one before,
- format 0 files put every channel in one track, so notes are split by
  their MIDI channel instead,
- notes on channel 10 are drums: map General MIDI's 36, 38, 39 and 42
  back to the built-in drums,
- the tempo meta event (`FF 51`) sets the project's tempo.

Each note-on must be matched with the next note-off of the same pitch
and channel. A `HashMap<(u8, u8), u32>` of the ticks where notes
started does it. What should happen to a note that never ends?

*Practices:* parsing binary data with slice patterns, error types,
`HashMap`.

## 7. Latency compensation (loom-audio and loom-app, harder)

[Latency](recording.md#latency-what-you-hear-is-in-the-past) makes
every recorded key a little late. cpal knows by how much: the audio
callback gets a `&cpal::OutputCallbackInfo`, whose `timestamp()` has
two instants, `callback` (now) and `playback` (when this block will be
heard). The difference between them is the output latency.

The callback ignores that argument today (`_: &cpal::OutputCallbackInfo`
in `build_stream`). Pass the latency to the audio thread, publish it as
a number of steps in another atomic, and subtract it from each key's
position before `recorded_note` rounds it. Remember that a position
just after step 0 minus the latency must go round to the end of the
loop, not below zero.

Then think about the other direction. MIDI Out messages leave when
their block is made, but the built-in instruments' sound of that same
block is heard about 21 ms later. So an external synth plays a little
*before* the rest. How could the sending thread hold each message back
by the latency? What would each message need to carry?

*Practices:* reading the docs of a crate (`cargo doc --open -p cpal`),
time as data.

## 8. Think about it

- The live queue holds 256 messages, and `LiveInput::send` returns
  `false` when it is full. `on_message` ignores that. Which is worse to
  lose, a key-down or a key-up? Could a design make a lost key-up
  impossible, for example by sharing *which keys are down* (128 bits)
  instead of sending events?
- The sending thread wakes up a thousand times a second, even when no
  MIDI Out channel exists. Could it sleep longer when there is nothing
  to send? What would a longer sleep cost when there is?
- `release_live_keys` lets go of every key when a channel is inserted
  or removed. The other way would be to renumber `live_keys` to follow
  the channels. Which is easier to get right? Would a player ever
  notice the difference?
- midir gives `on_message` a timestamp in microseconds, which the code
  ignores in favour of the audio thread's position. When would midir's
  timestamp be more precise? Why can the two clocks not simply be
  compared?
- Without the format check, a format 1 version of Ferrous Loom opening a
  file with a MIDI Out channel would fail anyway: serde has never heard
  of `MidiOut`. So why raise the format at all? Compare the two error
  messages the user would see.
- A pattern played by a MIDI Out channel and exported as a `.mid` file
  sends the same notes in both cases. Could the export and the audio
  thread share more code? What would that code need to know about
  time?
