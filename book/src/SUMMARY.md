# Summary

[Introduction](introduction.md)

[Building it from source](building.md)

# Part I: The shape of the program

- [Five crates, and what each one may know](program/crates.md)
- [Two threads: the window and the audio](program/threads.md)
- [The project: patterns, channels, tracks, clips and markers](program/project.md)
- [Telling the audio thread what changed](program/sync.md)
- [Undo and redo](program/undo.md)
- [Saving: the .lineup file, recovery and damaged files](program/saving.md)
- [Drawing the windows with egui: layout, zoom, themes, icons](program/egui.md)
- [The editors: channel rack, piano roll and playlist](program/editors.md)
- [Two languages: translations, and letters or solfège](program/languages.md)
- [Exercises: the shape of the program](program/exercises.md)

# Part II: Sound

- [The sequencer: timing to the sample](sound/sequencer.md)
- [Tempo, meter, swing and feel](sound/tempo.md)
- [Instruments: drums, sampler and Lineup Osc](sound/instruments.md)
- [SoundFonts and General MIDI](sound/soundfonts.md)
- [The mixer: levels, pan, reverb, effects and score order](sound/mixer.md)
- [MIDI: messages, devices, recording and files](sound/midi.md)
- [Exporting: WAV, MIDI and video](sound/export.md)
- [The playback trace](sound/trace.md)
- [Exercises: sound](sound/exercises.md)

# Part III: Music theory in code

- [Pitch classes, notes and spelling](theory/pitch.md)
- [Scales, modes and keys](theory/scales.md)
- [Chords: building, naming and inversions](theory/chords.md)
- [Roman numerals, the harmony keys and chord functions](theory/numerals.md)
- [Editing tools as plain functions](theory/tools.md)
- [Hearing harmony: chords and keys from notes](theory/hearing.md)
- [Pitch-class sets, and one harmony with four names](theory/sets.md)
- [Twelve-tone rows and their rules](theory/twelve-tone.md)
- [Micromodes](theory/micromodes.md)
- [Voice leading and part motion](theory/voice-leading.md)
- [Takes: hearing a sung tune](theory/listen.md)
- [Exercises: music theory in code](theory/exercises.md)

# Part IV: The band

- [The library: styles and rhythms as data](band/library.md)
- [The chord sheet: model, layout and text](band/chord-sheet.md)
- [How the band plays a chord sheet](band/band.md)
- [An orchestra for music that changes meter](band/orchestra.md)
- [Song form, sections, variations and locks](band/song-form.md)
- [Shots, holds, rests and pushes](band/stops.md)
- [The soloist](band/soloist.md)
- [Your own styles: the style editor](band/style-editor.md)
- [Practice tools: metronome, speed trainer, ear training](band/practice.md)
- [The lead sheet: from notes to notation](band/lead-sheet.md)
- [Exercises: the band](band/exercises.md)

# Part V: Songs as data

- [How a demo is written](songs/demos.md)
- [From a MIDI performance or a score to a demo](songs/from-midi-and-scores.md)
- [Exercises: songs as data](songs/exercises.md)

# Part VI: Building, testing and shipping

- [How the code is tested: unit tests, window tests, traces](shipping/testing.md)
- [Three systems and continuous integration](shipping/ci.md)
- [Release packages](shipping/releases.md)
- [How the books are built](shipping/books.md)

# Reference

- [Project file format](reference/project-format.md)
- [Rust glossary](reference/rust-glossary.md)
- [Music glossary](reference/music-glossary.md)
- [Ideas for what comes next](reference/ideas.md)
