# Roadmap

| Step | Status | Contents |
|------|--------|----------|
| 1. Foundation | Done | Cargo workspace, `cpal` audio engine, `egui` window, test tone, tests, Linux CI, this book |
| 2. Step sequencer | Done | Channel rack, sample-accurate step sequencer, synthesized drum kit, WAV sampler with drag and drop, Loom Osc synth |
| 3. Piano roll and harmony | Done | Piano roll editor, notes with pitch and length, scale highlighting, chord stamping, live chord names and Roman numerals, polyphonic synth, pitched samplers |
| 4. Playlist and saving | Done | Many patterns, the playlist with tracks and clips, pattern and song mode, `.loom` project files, macOS and Windows builds in CI |
| 5. Undo and export | Done | Undo and redo for every change, one diff-based path to the audio thread, a sample cache, WAV export in the background |
| 6. Harmony tools | Next | Ghost notes, snapping notes to the scale, chord inversions, strummed and arpeggiated chords, more chord types, a velocity lane, suggestions for the next chord |

The four steps planned at the start are done: Ferrous Loom runs on
Linux, macOS and Windows, using the same code on all three. Step 5 added
two features every studio needs, undo and WAV export. Step 6 will bring
the piano roll's harmony help closer to what a full studio offers.

## Ideas for what comes next

Beyond step 6, none of these are planned yet. They are the features a
pattern-based studio usually has that Ferrous Loom does not, roughly from
smaller to bigger:

- **A mixer.** Each channel routed to a mixer track with its own volume,
  pan and effects (a reverb and a delay to start with).
- **Automation**: knob movements recorded or drawn over time, in the
  playlist.
- **Audio clips** in the playlist, next to pattern clips.
- **MIDI input**, to play the Loom Osc from a keyboard, and MIDI file
  export of patterns.
- **Even more harmony help**: borrowed chords and modulations
  explained, a "voice leading" view.
- **Packaging**: icons, a macOS `.app` bundle, a Windows installer, and
  double-clicking `.loom` files to open them.
