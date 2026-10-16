# Roadmap

| Step | Status | Contents |
|------|--------|----------|
| 1. Foundation | Done | Cargo workspace, `cpal` audio engine, `egui` window, test tone, tests, Linux CI, this book |
| 2. Step sequencer | Done | Channel rack, sample-accurate step sequencer, synthesized drum kit, WAV sampler with drag and drop, Loom Osc synth |
| 3. Piano roll and harmony | Done | Piano roll editor, notes with pitch and length, scale highlighting, chord stamping, live chord names and Roman numerals, polyphonic synth, pitched samplers |
| 4. Playlist and saving | Next | Arranging patterns into a song, saving and loading projects, macOS and Windows builds |

The app runs on Linux first. Every library chosen so far (`cpal`, `egui`,
`eframe`, `rtrb`, `hound`) also works on macOS and Windows, so step 4 adds those
builds without rewriting code.
