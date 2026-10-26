# Exercises

## 1. Octave keys (lineup-app, easy)

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

## 2. Softer and louder typing (lineup-app, easy)

Typed keys always play at `DEFAULT_VELOCITY`, so recorded takes from
the typing keyboard all have the same velocity, while a MIDI keyboard's
keep how hard each key was struck. Add a "Typing velocity" slider to the
MIDI menu (`egui::Slider::new(&mut value, 1..=127)`), and make Shift
play at 127 while it is held.

Shift does not change `physical_key`, so the piano keeps working. But
look at the filter in `typing_keyboard`: does a key that went down with
Shift come up properly when Shift is let go first?

*Practices:* egui sliders, keyboard modifiers.

## 3. A click on the offbeats (lineup-audio, medium)

The basic metronome this exercise once asked for now exists: the
practice row has `Command::SetMetronome`, and the count-in uses the same
clicks. It clicks only the beats. Add a **subdivision** setting
(eighths, triplets or sixteenths) that makes the metronome click between
the beats too, quieter than the beat and with the bar accent kept.
`Project::played_clicks` and `pattern_clicks` build a `Click` for every
step, `metronome::click_at` chooses it, and `ClickVoice` makes the
sound; decide whether the subdivision belongs in `Click` or in a second
setting sent to the audio thread. Should the count-in stay on the beat?

The click must not be in the WAV export, or in the MIDI export. Where
does each of those come from, and which one would get the click by
accident if it lived in a channel?

*Practices:* a new setting end to end, real-time sound without
allocating.

## 4. Notes across the loop (lineup-core, medium)

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

## 6. The other pieces of a format 2 file (lineup-core, harder)

Opening MIDI files, which this exercise once asked for, now exists:
`midi_file::read::read` turns the bytes into events, and
`midi_file::import::import` makes the project behind **File > Import
MIDI file…**. Two corners are still missing.

A **format 2** file is not one song but several independent pieces, one
per track. `import` keeps only the first track and says nothing about
the rest. Add a way to choose a piece: list them in the notice, ask in
the dialog, or import each as its own project. What should the tests say
about a format 2 file with three pieces?

The other is the **SMPTE offset** meta event (`FF 54`): five bytes that
say where a track starts on a timecode, for music written against film.
The header's SMPTE timing (`Timing::PerSecond`) already works; this
event is read past and dropped. Read it and decide what a project with
no video should do with it. Test both cases with small files built in
the test module, the way the existing tests do.

*Practices:* parsing binary data with slice patterns, designing a
choice, error types.

## 7. Latency compensation (lineup-audio and lineup-app, harder)

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
- Without the format check, a format 1 version of Ferrous Lineup opening a
  file with a MIDI Out channel would fail anyway: serde has never heard
  of `MidiOut`. So why raise the format at all? Compare the two error
  messages the user would see.
- A pattern played by a MIDI Out channel and exported as a `.mid` file
  sends the same notes in both cases. Could the export and the audio
  thread share more code? What would that code need to know about
  time?
