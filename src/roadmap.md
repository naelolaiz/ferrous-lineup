# Roadmap

| Step | Status | Contents |
|------|--------|----------|
| 1. Foundation | Done | Cargo workspace, `cpal` audio engine, `egui` window, test tone, tests, Linux CI, this book |
| 2. Step sequencer | Done | Channel rack, sample-accurate step sequencer, synthesized drum kit, WAV sampler with drag and drop, Loom Osc synth |
| 3. Piano roll and harmony | Done | Piano roll editor, notes with pitch and length, scale highlighting, chord stamping, live chord names and Roman numerals, polyphonic synth, pitched samplers |
| 4. Playlist and saving | Done | Many patterns, the playlist with tracks and clips, pattern and song mode, `.loom` project files, macOS and Windows builds in CI |

The four planned steps are done: Ferrous Loom runs on Linux, macOS and
Windows, using the same code on all three.

## Ideas for what comes next

None of these are planned yet. They are the features a pattern-based
studio usually has that Ferrous Loom does not, roughly from smaller to
bigger:

- **Undo and redo.** Every edit already goes through an action and a
  `Project` method, which is a good place to record what changed.
- **A mixer.** Each channel routed to a mixer track with its own volume,
  pan and effects (a reverb and a delay to start with).
- **Exporting the song** as a WAV file, by running the audio engine
  without a sound card, as the tests do (step 4's exercise 6).
- **Automation**: knob movements recorded or drawn over time, in the
  playlist.
- **Audio clips** in the playlist, next to pattern clips.
- **MIDI input**, to play the Loom Osc from a keyboard, and MIDI file
  export of patterns.
- **More harmony help**: suggestions for the next chord, borrowed chords
  and modulations explained, a "voice leading" view.
- **Packaging**: icons, a macOS `.app` bundle, a Windows installer, and
  double-clicking `.loom` files to open them.
