# Roadmap

| Step | Status | Contents |
|------|--------|----------|
| 1. Foundation | Done | Cargo workspace, `cpal` audio engine, `egui` window, test tone, tests, Linux CI, this book |
| 2. Step sequencer | Next | Channel rack, patterns of steps, tempo-driven playback, one synth, sample playback |
| 3. Piano roll and harmony | Planned | Piano roll editor, scale and chord highlighting, placing whole chords, live chord names |
| 4. Playlist and saving | Planned | Arranging patterns into a song, saving and loading projects, macOS and Windows builds |

The app runs on Linux first. Every library chosen so far (`cpal`, `egui`,
`eframe`, `rtrb`) also works on macOS and Windows, so step 4 adds those
builds without rewriting code.
