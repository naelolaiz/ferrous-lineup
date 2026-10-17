# Roadmap

| Step | Status | Contents |
|------|--------|----------|
| 1. Foundation | Done | Cargo workspace, `cpal` audio engine, `egui` window, test tone, tests, Linux CI, this book |
| 2. Step sequencer | Done | Channel rack, sample-accurate step sequencer, synthesized drum kit, WAV sampler with drag and drop, Lineup Osc synth |
| 3. Piano roll and harmony | Done | Piano roll editor, notes with pitch and length, scale highlighting, chord stamping, live chord names and Roman numerals, polyphonic synth, pitched samplers |
| 4. Playlist and saving | Done | Many patterns, the playlist with tracks and clips, pattern and song mode, `.lineup` project files, macOS and Windows builds in CI |
| 5. Undo and export | Done | Undo and redo for every change, one diff-based path to the audio thread, a sample cache, WAV export in the background |
| 6. Harmony tools | Done | Selecting and moving groups of notes, a Tools menu (transpose, move in key, snap to the key, invert, arpeggiate), stamped inversions, seven more chord types, next-chord suggestions with secondary dominants, ghost notes from the other channels, a velocity lane |
| 7. MIDI | Done | MIDI keyboards in, external synths out (MIDI Out channels), a typing keyboard, chords named as you play, recording takes, `.mid` export of songs and patterns, file format 2 |
| 8. Harmony keys and library | Done | The harmony keys (the seven chords of the key, coloured by function, with next-chord suggestions and the chord playing), time signatures, 78 harmony styles and 49 rhythms in 13 genres including tango, 16 public-domain songs, demos, nine new drums, patterns up to sixteen bars, file format 3 |
| 9. The band | In progress, on the `band-in-a-box` branch | Following Band-in-a-Box: chord names and slash chords (done), then a chord sheet the band plays, a practice bar, song form, swing, SoundFont sounds, a bigger band, a lead sheet; see [step 9](step-9/README.md) |

The four steps planned at the start are done: Ferrous Lineup runs on
Linux, macOS and Windows, using the same code on all three. Step 5 added
two features every studio needs, undo and WAV export. Step 6 brought the
piano roll's harmony help closer to what a full studio offers. One
planned tool, strumming, was left out, because it needs notes that start
between steps; [step 6](step-6/README.md#what-is-not-here-strumming)
explains why. Step 7 came from a question, "does it have MIDI, in and
out?" Now it does, both ways, and it names the chord you hold down. It
was built on a computer with no MIDI system, so real MIDI
hardware has not been tried yet; [step 7](step-7/README.md#how-it-was-tested)
says what was tested. Step 8 added what a pattern-based studio is
known for when you learn harmony: a row of chord keys, one for each
chord of the key. A library of styles, rhythms and songs came with it,
to hear how each genre uses those chords.

## Ideas for what comes next

None of these are planned yet. They are the features a
pattern-based studio usually has that Ferrous Lineup does not, roughly from
smaller to bigger:

- **A mixer.** Each channel routed to a mixer track with its own volume,
  pan and effects (a reverb and a delay to start with).
- **Automation**: knob movements recorded or drawn over time, in the
  playlist.
- **Audio clips** in the playlist, next to pattern clips.
- **MIDI file import**, and recording in song mode (exercises 5 and 6 of
  step 7).
- **Finer timing**: notes that start between steps, for strumming,
  swing and notes played slightly early or late.
- **Even more harmony help**: borrowed chords and modulations
  explained (exercise 4 of step 8 starts it), a "voice leading" view,
  chords named over a whole beat.
- **Time signature changes** in the middle of a song (exercise 6 of
  step 8).
- **Packaging**: icons, a macOS `.app` bundle, a Windows installer, and
  double-clicking `.lineup` files to open them.
