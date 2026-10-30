# Exercises: sound

These exercises practise this part on the real code: the sequencer,
tempo and meter, the instruments, the mixer, MIDI, the exports and the
trace. Each one says what is **already there**, so you extend it rather
than build it again. Some are things to try in the app and listen to;
some are pieces of Rust with a clear end; the ones called "Think about
it" are questions, with no code to write.

Try the Rust ones on a branch of your own. Run `cargo test` after each
change, and `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` before you call
one finished: CI runs all three. Several exercises grow the project
file format; each needs the next free format number, so if you do two
of them, the second one takes the number after the first.

## Timing, tempo and meter

### 1. A meter map inside a pattern (lineup-core, hard)

The song already has a meter map: `Meter` keeps the starting
`TimeSignature` and a sorted list of `TimeMarker`s, and `bar_at`,
`time_at` and `click` read it (see
[Tempo, meter, swing and feel](tempo.md)). A `Pattern` still
has only one `time: Option<TimeSignature>`, so a part that changes
meter every bar is cut into a pattern per run of bars.

Let one pattern hold a meter map of its own, with changes at bar lines.
`Pattern::length` counts steps, and a bar is no longer one fixed number
of them, so:

- Which code that asks `TimeSignature::steps_per_bar` or `Meter::bar_at`
  for a pattern would have to ask the pattern's map instead: the note
  tools, `Project::pattern_clicks`, the piano roll's bar lines?
- What should the playlist and the piano roll ask when a clip starts
  partway through a pattern?
- How would `format_for` account for it, so a pattern in one meter is
  written exactly as now?

Start with `Meter` rather than inventing a second type. `bar_at` walks
the stretches between markers one by one. Why can it not simply use
`partition_point` on the markers, as other sorted lists in the program
do? What would it need to keep beside each marker to make that
possible?

*Practises:* reading an existing data structure before extending it,
bar arithmetic, keeping files compatible.

### 2. Swing by ear (the app, easy)

Open *Midnight Shuffle* and play it with the **Swing** control at
straight, 50 and 100. Which instrument shows the change most? Which
setting sounds like a march, which like a shuffle?

Then work it out. At 96 BPM, how many milliseconds after the beat does
the second eighth start at straight, 50 and 100? (A beat is 96 ticks,
and `swing::delay` says how many ticks late the second eighth is.) At
120 BPM the third note of a triplet comes 333 ms after the beat: why is
the 100 answer not 333 ms at 96 BPM? Check your answers with a
[trace](trace.md): the `late_ms` column.

*Practises:* turning ticks into time.

### 3. Swing in Rust (lineup-core, medium)

`swing.rs` delays the second eighth of every beat as the notes are
*played*: read `delay`, `swung`, `swings_in` and
`Project::played_notes`.

- Write a test that a note held from the second sixteenth of a beat
  (step 1) to the offbeat ends 16 ticks later at full swing on a piano
  channel, but not on a drum channel. Which argument of `swung` makes
  the difference, and why should a drum hit not get longer?
- Add sixteenth-note swing: within each eighth, delay the second
  sixteenth (the "e" and the "a" of "1 e & a") by a smaller amount,
  chosen by a second field or a `SwingUnit` enum. Which ticks move, and
  by how much at full swing? Write a test that pairs of sixteenths still
  take the same total time, and that the written notes are untouched.
  What must `swings_in` say about 6/8, where the sixteenths come in
  pairs too? What else does the new setting touch: the project file,
  the control, the MIDI export?

*Practises:* arithmetic with ticks, unit tests with exact numbers,
keeping playback and editing apart.

### 4. Odd meters by ear (the app, easy)

- Open **Ajde Jano** and, in the Time list, type `7` over `8` grouped
  as `2+2+3`, and click **Use 2+2+3/8**. The melody keeps its notes,
  but the band now leans on other eighths. Which beats of the tune now
  land between the bass notes?
- In the Library, play the **Kalamatianos** and the **Rachenitsa**
  rhythms one after the other. Clap along: where is the long beat in
  each?
- Pick `5/8` in the Time list of a song in 4/4, with **Use matching
  style** ticked. Which rhythm does it get, and how does the list write
  the meter?
- Open **Süsserl** and watch the ruler while it plays. Count the bars
  of two and the bars of three in the first strain: do they match
  2 2 3, 2 2 3, 2 2 3 3?

*Practises:* hearing grouped beats, reading `TimeSignature` groups.

### 5. Think about it: swing

- In a song with full swing, write a dotted eighth and a sixteenth in
  the piano roll, then two eighths. Which pair moves? Many jazz players
  read both the same way, as a triplet. What would Ferrous Lineup need
  to know to do that, and why could it go wrong on a march?
- Put a swinging song in 6/8 with the Time box. What happens to the
  Swing control, and to the sound? Write the same groove in 12/8 by
  hand: where do the long and short notes go?
- `played_notes` returns a `Cow`. What would change if it returned a
  `Vec<NoteEvent>` instead? Count how many notes `diff` would copy for a
  song of 20 patterns and 8 channels, on every edit.

## Instruments and SoundFonts

### 6. A low cowbell (lineup-audio and lineup-core, easy)

Read `drums::cowbell`: two square waves at 540 and 800 Hz (a little
less than a fifth apart: 800 / 540 is about 680 cents, a fifth 702), a
high-pass at 400 Hz, a low-pass at 3 kHz, and two decays.

Make the two frequencies parameters (`cowbell_at(sample_rate, low,
high)`) and add a `Drum::LowCowbell` an octave below. The compiler lists
every `match` to update in `drums::render`, `Drum::name` and
`general_midi_drum`; add the new drum to `Drum::ALL` too (and change its
length), or the menus and the drum tests will not see it. Which General
MIDI note should it be sent as? (`general_midi::drum_name` lists them.)

Adding a drum also changes which projects older versions can open. Read
`format_for` and `Drum::is_new_in_format_3`: is that check enough for a
drum added now, or does the file need the next format number?

*Practises:* enums, exhaustive `match`, a change that crosses crates.

### 7. A legato synth (lineup-audio, harder)

The Lineup Osc plays each note on a voice of its own. Give it a mono,
*legato* mode: only one voice plays, and a new note changes that
voice's pitch without restarting the envelope. Add a `glide` time so the
pitch slides from the old note to the new one (use `dsp::Smoothed` for
the frequency). How does `pick_voice` change? Which parts of the `Stage`
state machine are skipped? Where would the setting live, and how does it
reach the audio thread?

*Practises:* state machines, reusing small DSP pieces.

### 8. A click on the offbeats (lineup-audio, medium)

The metronome clicks the beats, higher on the first beat of a bar:
`Project::played_clicks` and `pattern_clicks` build a `Click` (`None`,
`Beat` or `Bar`) for every step, `metronome::click_at` chooses one, and
`ClickVoice` makes the sound. Add a **subdivision** setting (eighths,
triplets or sixteenths) that makes it click between the beats too,
quieter than the beat and with the bar accent kept. Decide whether the
subdivision belongs in `Click` or in a second setting sent to the audio
thread. Should the count-in stay on the beat?

The click must not be in the WAV export, or in the MIDI export. Where
does each of those come from, and which one would get the click by
accident if it lived in a channel?

*Practises:* a new setting end to end, real-time sound without
allocating.

### 9. Another SoundFont (the app, easy)

Download another General MIDI SoundFont in `.sf2` form (*FluidR3_GM*,
or the small *TimGM6mb*), pick it with **Choose SoundFont…**, and play
the Canon in D. Which strings do you prefer? Is one SoundFont much
louder than the other? Where would you put a volume setting for the
SoundFont as a whole: in `Sounds`, on the audio thread, or in each
`SoundFontPlayer`?

*Practises:* listening, finding where a setting belongs.

### 10. An instrument for the Lead (lineup-core, easy)

Twelve-tone rows and micromodes write their melody on a channel called
Lead, which `library::apply::synth_named` makes with no General MIDI
program, so it keeps the Lineup Osc while the Keys get a piano and the
Bass a bass. Pick an instrument for it in the rack and listen. Which
instrument would you give the Lead by default, and where in
`synth_named` would it go? Which tests would tell you that the songs,
which choose their own Lead, still keep theirs?

*Practises:* reading a `match` on names, defaults.

### 11. Think about it: instruments

- A SoundFont renders in stereo, but `SoundFontPlayer::next_sample`
  folds left and right into one sample before the channel's pan. What
  would it take to keep its left and right apart, and what should Pan
  then do: move both sides, or turn one down (a *balance*)?
- rustysynth has a reverb and a chorus of its own, and
  `enable_reverb_and_chorus` turns them off. Turn them on and listen.
  Each channel would then have its own room: what does that cost,
  compared with the [mixer's shared reverb](mixer.md), and
  why do mixing desks use a send instead?
- The export runs at 44,100 frames per second even if your sound card
  runs at 48,000. Look at how the drums are made (`drums::render` takes
  the sample rate) and how the sampler plays a sound recorded at
  another rate. Could an export ever sound different from what you
  hear?

## The mixer and effects

### 12. A constant-power pan law (lineup-core, easy)

`Channel::pan_gains` keeps the near side full and turns the far side
down in a straight line. Change it to a *constant-power* law: with
`angle = (pan + 1) × π / 4`, the gains are `[angle.cos(), angle.sin()]`.
Which tests fail, and why? Move a channel from L100 to R100 with each
law and listen: which one stays equally loud all the way across, and
which one is louder in the middle?

*Practises:* trigonometry on samples, tests that pin a design.

### 13. Meter arithmetic (lineup-app and lineup-audio, easy)

- A level of 0.1 is how many decibels? How far is the meter lit for it?
  Work it out, then check with `lit` and `FLOOR_DB` in `meters.rs`.
- `levels::raise` compares `peak > 0.0` rather than `peak >= 0.0`.
  What would `fetch_max` do with the bits of −0.0, or of a NaN? (Look at
  how `f32::to_bits` orders them as integers.)

*Practises:* decibels, floats as bits.

### 14. Delays and compressors by ear (the app, easy)

- Put a delay of a dotted eighth with 30% mix on the Lead of a demo, and
  play it at 60 BPM and at 140 BPM. How many milliseconds apart are the
  echoes each time? Check with `Delay::seconds`.
- Set a compressor at −30 dB and 10:1 on the drums. Then raise its
  **Gain** until the drums are as loud as before. What happened to the
  difference between the loud and the soft hits? Why do drums with a
  strong compressor sound "squashed"?

*Practises:* listening for what an effect does.

### 15. A wider or narrower bell (lineup-core, lineup-audio and lineup-app, medium)

The equalizer's middle band is a bell with a fixed width, a Q of 1
(`Biquad::set` in `lineup-audio/src/effects.rs`). Add a `width` to
`Equalizer` and a slider for it, and use it in `Biquad::set`. What does
the Cookbook say alpha is for a given Q? A song that sets the width
needs the next free format number: teach `format_for`, and leave the
field out of the file when it has its default.

Then write a test in the style of the existing ones: a sine at the
bell's centre is boosted by the gain, and one two octaves away is boosted
less with a narrow bell than with a wide one.

*Practises:* a filter formula, a new saved field end to end.

### 16. A shared delay send (lineup-core, lineup-audio and lineup-app, hard)

Each channel already has a delay of its own among its effects. Add a
second **send** beside the reverb's, into one delay that every channel
shares, so several channels echo together. This crosses the project
file, the audio thread and the mixer window.

1. In `Channel`, add `delay_send: f32`, default `0.0`, and include it in
   the range checks and in `has_plain_mix`. Give every constructor the
   default.
2. In `Mixer`, add the delay's own settings: time in steps (so it
   follows the tempo, as `Delay` does), feedback, and mix. Validate
   them in `Mixer::valid`.
3. Add `Command::SetSharedDelay` and `Command::SetDelaySend`, and a
   shared delay that allocates its buffer before it reaches the audio
   thread, never in `process`. The channel delay's `DelayLine` shows
   how. Follow the reverb's pattern for the send.
4. Add a **Delay** slider to each mixer line, next to **Reverb**, and a
   small delay section beside the reverb's controls.
5. Teach `format_for` that a non-zero delay send or non-default delay
   settings need the next free format number, and leave the fields out
   of the text when they are at their defaults.
6. Test the audio path (a send of 0 makes no echo, a send above 0 makes
   one) and the file (an old file opens with send 0; a song with a
   delay send is written in the new format and round-trips).

Done when the slider is audible in the app and in a WAV export, the
audio thread allocates nothing while it runs, and a song without the
delay is still written in the format it was.

*Practises:* everything in this part at once.

### 17. Score order (the app and lineup-core, easy)

- Open *In the Mood*, then *Sing, Sing, Sing*, with the mixer open. In
  which section is each song's "Horns" channel? Look at the Instrument
  column to see why.
- Open *Danny Boy*, right-click its Lead in the channel rack and pick
  **Instrument > Pipe > 74 Flute**. Where does it move in the mixer?
- In `score_order.rs`, the saxophones are between the clarinets and the
  bassoons, as most orchestral scores have them. In a big band score
  the saxophones come first, above the trumpets. What would you change
  to get the big band order, and which test would tell you that you
  broke the orchestral one?

*Practises:* data that decides a layout.

### 18. Think about it: dynamics

- The compressor's `Follower` rises in 10 ms (`ATTACK_SECONDS`) instead
  of following every wave. Write a test that sends a 50 Hz sine through
  a compressor whose follower is much faster, and measure how far the
  output's shape is from a sine. What does that tell you about the
  10 ms?
- Many desks let a compressor listen to one channel while it turns
  another down: a bass that ducks each time the kick drum hits
  ("side-chain compression"). Sketch what the channel effects would
  need for that. Which channel's sample would the follower read, and in
  which order would the audio thread have to mix the channels?

## MIDI

### 19. Octave keys (lineup-app, easy)

The [typing keyboard](midi.md#the-typing-keyboard) covers C4
to E6. Add two keys that move it down and up by an octave, such as `-`
and `=` (`Key::Minus` and `Key::Equals`), which play no note yet. Keep
the shift between C1 and C7, and show it in the MIDI menu, next to
**Typing keyboard to piano**.

The tricky case: hold Z, press `=`, then let go of Z. Which note must
end? Look at `typing_down`, which keeps the note each key played, not
only the key. Why was it written that way, when the note could be
worked out again from the key?

*Practises:* `Note` arithmetic, remembering what was done instead of
working it out again.

### 20. Softer and louder typing (lineup-app, easy)

Typed keys play at `DEFAULT_VELOCITY`, so takes recorded from the typing
keyboard all have the same velocity, while a MIDI keyboard's keep how
hard each key was struck. Add a "Typing velocity" slider to the MIDI
menu (`egui::Slider::new(&mut value, 1..=127)`), and make Shift play at
127 while it is held.

Shift does not change `physical_key`, so the piano keeps working. But
look at the filter in `typing_keyboard`: does a key that went down with
Shift come up properly when Shift is let go first?

*Practises:* egui sliders, keyboard modifiers.

### 21. Notes across the loop (lineup-core, medium)

[`recorded_note`](midi.md#quantizing-recorded_note) has two
limits. A note held across the end of the loop is cut there, and a key
held for more than a whole loop is recorded as a short note.

- Make a key held across the loop end record **two** notes: one to the
  end of the pattern, and one from step 0 for the rest. The function
  then returns a `Vec<NoteEvent>` (or an array and a count). Write the
  tests first: (14, 2) in a 16-step pattern gives (14, 2) and (0, 2).
- For long notes, the position alone is not enough. Have the audio
  thread also count how many times the loop has gone round, or publish
  the number of steps played since Play instead of the position in the
  loop. Which is simpler to share through one `AtomicU64`? Which one
  can `finish_recording` still use?

*Practises:* changing a function's return type and its callers, tests
as a specification.

### 22. Record in song mode (all three crates, harder)

Rec switches to pattern mode, because a take goes into the pattern that
loops. In song mode, the position is a step of the *song*, so a key
could go into the clip playing at that moment, at `step - clip.start`.

Work out the questions before writing code. Two clips can play at once,
on different tracks: which one gets the note? What if no clip plays
there: should a new clip be made? A clip may be played several times in
the song, and recording into one of them changes all of them, because
they are the same pattern. Is that what you would want? How do other
studios you know handle it?

*Practises:* designing behaviour before code.

### 23. The other pieces of a format 2 file (lineup-core, harder)

`midi_file::read::read` turns a file's bytes into events, and
`midi_file::import::import` makes a project of them. Two corners are
still missing.

A **format 2** file is not one song but several independent pieces, one
per track. `import` keeps only the first track. Add a way to choose a
piece: list them in the notice, ask in a dialog, or import each as its
own project. What should the tests say about a format 2 file with three
pieces?

The other is the **SMPTE offset** meta event (`FF 54`): five bytes that
say where a track starts on a timecode, for music written against film.
The header's SMPTE timing (`Timing::PerSecond`) already works; this
event is read past and dropped by `meta_event`. Read it and decide what
a project with no video should do with it. Test both cases with small
files built in the test module, the way the existing tests do.

*Practises:* parsing binary data with slice patterns, designing a
choice.

### 24. Latency compensation (lineup-audio and lineup-app, harder)

[Latency](midi.md#latency-what-you-hear-is-in-the-past)
makes every recorded key a little late. cpal knows by how much: the
audio callback gets a `&cpal::OutputCallbackInfo`, whose `timestamp()`
has two instants, `callback` (now) and `playback` (when this block will
be heard). The difference between them is the output latency.

The callback ignores that argument (`_: &cpal::OutputCallbackInfo` in
`build_stream`). Pass the latency to the audio thread, publish it as a
number of steps in another atomic, and subtract it from each key's
position before `recorded_note` rounds it. A position just after step 0
minus the latency must go round to the end of the loop, not below zero.

Then think about the other direction. MIDI Out messages leave when their
block is made, but the built-in instruments' sound of that same block
is heard a little later. So an external synth plays slightly *before*
the rest. How could the sending thread hold each message back by the
latency? What would each message need to carry?

*Practises:* reading a crate's documentation (`cargo doc --open -p
cpal`), time as data.

### 25. A MIDI note-off test (lineup-audio, medium)

Prove that **Stop** sends note-offs for notes the sequencer started on a
MIDI Out channel. Read `midi_out_channels_send_their_notes` and
`stopping_releases_held_notes` in the engine's test module, which has
the helpers `add_midi_out`, `received` and `note_off`. Add
`stopping_releases_midi_out_notes`: a long note on MIDI channel 3,
**Play**, render one step and check the NoteOn; send **Stop**, render a
few frames, and check the NoteOff. Make the note long enough that it
cannot have ended by its own gate.

Run the test once with its `Command::Stop` line removed, to see it
fail, then put the line back. A test you have never seen fail may not be
testing anything.

*Practises:* the engine's test harness, regression tests.

### 26. MIDI Out from the menus (the app, easy)

- Send the Bass of a band demo over MIDI on channel 2, then apply
  another style from the Library. How many Bass channels are there
  afterwards? Read `find_synth` in `library/apply.rs` to see why.
- Which General MIDI drum is note 60? A drum written on C4 and sent as
  written, rather than as its own note, would play it. Find it with
  `general_midi::drum_name`.
- `Channel::send_over_midi` clamps the channel with `.min(15)` instead
  of returning an error. Where else does a number from a menu or a file
  get clamped like this, and where is it refused instead? Which is
  better for a number read from a file?

*Practises:* reading code to explain behaviour.

### 27. Think about it: MIDI

- The live queue holds 256 messages, and `LiveInput::send` returns
  `false` when it is full. `on_message` ignores that. Which is worse to
  lose, a key-down or a key-up? Could a design make a lost key-up
  impossible, for example by sharing *which keys are down* (128 bits)
  instead of sending events?
- The sending thread wakes up a thousand times a second, even when no
  channel is sent over MIDI. Could it sleep longer when there is nothing
  to send? What would a longer sleep cost when there is?
- `release_live_keys` lets go of every key when a channel is inserted or
  removed. The other way would be to renumber `live_keys` to follow the
  channels. Which is easier to get right? Would a player ever notice the
  difference?
- midir gives `on_message` a timestamp in microseconds, which the code
  ignores in favour of the audio thread's position. When would midir's
  timestamp be more precise? Why can the two clocks not simply be
  compared?
- A pattern played by a MIDI Out channel and exported as a `.mid` file
  sends the same notes in both cases. Could the export and the audio
  thread share more code? What would that code need to know about time?

## Exporting

### 28. Export one pattern as audio (lineup-app, easy)

**File > Export pattern as MIDI…** writes the selected pattern as a MIDI
file. Add the same as *audio*: an **Export pattern as WAV…** item,
saving the selected pattern, played four times, as a WAV file.

The obvious plan is pattern mode: `SetMode(PlayMode::Pattern)`,
`SelectPattern`, and render four pattern lengths plus the tail. But
pattern mode loops the pattern for ever, so the tail would hold the
pattern starting a fifth time: the problem [the export](export.md#silent-steps-after-the-end)
solves with silent steps, and pattern mode has no song length to
stretch. Instead, make a *copy* of the project, replace its playlist
with four clips of the pattern on one track, and export that copy with
the code you already have. What has to change in `song_render_with` so
it can render a project other than `self.project`?

*Practises:* working on a copy, making a function more general.

### 29. Export the loop region (lineup-app, medium)

Songs can have a loop region, and song mode plays only that part. Add
**File > Export loop region…**, which writes just the region to a WAV
file. `Project::playback_arrangement()` already returns exactly the part
song mode plays, with repeats unrolled. Make `song_render_with` take an
`Arrangement` instead of always calling `project.arrangement()`, and
work out the frame count from that arrangement's own length and tempos.
What should the file name be? What should the menu item do when there
is no loop region?

*Practises:* reusing a calculation, thinking about an empty input.

### 30. Stems: one file per channel (lineup-app, medium)

Musicians often want *stems*: the song exported once per channel, each
file holding only that channel, to mix elsewhere. Add **File > Export
stems…**, which asks for a folder and writes `Kick.wav`, `Clap.wav` and
so on.

Each stem is an export with every other channel muted
(`Command::SetMuted`). Rendering them one after the other is simplest;
rendering them on several threads at once is faster on a computer with
several cores. The video export's canvas already paints bands of rows
with `std::thread::scope`: could the stems do the same, or do they need
`thread::spawn`, given that the window must not wait for them? How
should one progress bar show several exports? And what about two
channels with the same name, or a name with a `/`?

*Practises:* threads, file names from user text.

### 31. Two views in a video, and a sync check (the app, medium)

1. Open **El Choclo** and choose **File > Export WAV…**; save
   `el-choclo.wav`.
2. Choose **File > Export video…**, tick **Chord sheet** and **Piano
   roll**, put them side by side on the preview, pick **1280 × 720**,
   and export `el-choclo.mp4`.
3. If `ffprobe` is installed, compare the durations:

   ```sh
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.wav
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.mp4
   ```

4. Watch a bar where the chord changes. The framed bar on the chord
   sheet, the playhead in the piano roll and the sound should meet
   there together. Check a second point, such as the first bar after a
   repeat.

Done when the durations differ by no more than a picture or two, the
two views show the same bar at the same moment, and you can say why
`start_video` draws each picture **before** it renders the sound that
plays under it.

*Practises:* checking a design with outside tools.

### 32. Think about it: exporting

- The progress counter uses `Ordering::Relaxed`. Suppose the window
  opened the file the moment the counter reached the total, without
  waiting for `join`. What could it find? (Look at what
  `export_wav_cancellable` does after its last `progress` call.) Would a
  stronger `Ordering` fix that, or is the problem somewhere else?
- A video's sound is MP3 at 256 kbit/s, while the WAV file is the exact
  samples. Could the video carry the samples unchanged instead? Which
  MP4 box would change, and which players would no longer play it?
- `repeat_picture` makes the last picture last longer when nothing
  changed. What would go wrong if the scene compared *pixels* instead of
  triangles to decide that? What would it cost?

## The playback trace

### 33. Trace a bar inserted and deleted (the app and a terminal, easy)

The chord sheet and the playlist have **Insert a bar before**, **Insert
a bar after** and **Delete this bar**; `--trace` writes every note the
player plays.

1. Write a trace of a demo:

   ```sh
   cargo run --release -- --trace "El Choclo" --out before.tsv
   ```

2. Open the same demo, save it as `inserted.lineup`, right-click bar 3
   on the chord sheet and choose **Insert a bar after**. Save, and
   trace the saved file:

   ```sh
   cargo run --release -- --trace inserted.lineup --out after.tsv
   ```

3. Compare the two tables, ignoring the `#` lines:

   ```sh
   diff <(grep -v '^#' before.tsv) <(grep -v '^#' after.tsv) | head -40
   ```

   Every note that started at or after bar 4 should now be one bar
   later; notes in bars 1 to 3 should be unchanged.
4. Delete the bar you inserted (it is bar 4), save, trace again, and
   compare with `before.tsv`.

Done when the inserted trace shows the one-bar shift in `start`, `bar`
and `step`, the deleted trace matches the first one, and you can point
to the code in `bars.rs` that moves the clips, markers and sections.

*Practises:* checking an edit by what it plays.

### 34. Reading traces (a terminal, easy)

- Trace a demo with a ritardando (a tempo marker that glides) and read
  the `# tempo` lines. How many are there, and why one per step?
- Turn **Loose** up in a song, trace it, and use `awk` to find the
  latest note (the `late_ms` column). Is it ever later than the
  setting?
- With **Write a trace while playing** on in the app, play a pattern,
  mute its channel in the middle of a long note, and stop. Find the note
  in the file: what does its `end` column say?

*Practises:* `awk` on tables, reading a report.

### 35. Think about it: the tracer

- The tracer looks for the notes whose length is over at the very top
  of each frame, before the tempo can change on that frame. What would
  go wrong with a note that ends on the same frame as a tempo change, if
  it looked afterwards?
- A trace loses events when its queue is full, and says how many. Could
  it instead keep a few events that matter more (the `NoteOff`s, say)
  and drop others first? What would the reader then have to cope with?
