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
| 9. The band | Done | Chord names and slash chords, a chord sheet the band plays, a practice bar, SoundFont sounds, chord inversions, ear training, song form, variations, swing, chord sheets as text, a lead sheet, a bigger band, shots, holds, rests and pushes, and an experimental soloist; then release packages (Linux AppImage, `.deb` and `.tar.gz`; Windows Setup and `.zip`; macOS `.dmg` for Apple Silicon and Intel), odd meters, a mixer in score order, a mixer with pan, a reverb send and meters, video export, a playback trace, locked tracks, a style for each section, the style editor, many more songs, a library of 177 harmony styles and 112 rhythms, crash recovery, recent projects and restarting audio, chords from outside the key explained (borrowed chords, secondary dominants, pivot chords), how each part moves from chord to chord, a horn section for any style, EQ, compression and delay on each channel, and this book online and as a PDF with contents, an index and bookmarks; see [step 9](step-9/README.md) |

Ferrous Lineup is a backing band you can learn harmony with: type the
chords of a song, pick a style, and the band plays it, while the app
names every chord you hear. It runs on Linux, macOS and Windows, using
the same code on all three.

Steps 1 to 8 built what the band stands on. Steps 1 to 4 made the
sound, the patterns and the tracks of a song, and step 5 added undo and
WAV export. Step 6 gave the piano roll its harmony help. Step 7 came
from a question, "does it have MIDI, in and out?" Now it does, both
ways, and it names the chord you hold down. It was built on a computer
with no MIDI system, so real MIDI hardware has not been tried yet;
[step 7](step-7/README.md#how-it-was-tested) says what was tested. Step
8 added the harmony keys, one for each chord of the key, and a library
of styles, rhythms and songs: the knowledge the band plays from. In
step 8 the accompaniment could already follow the chords you play,
the way an arranger keyboard does. Step 9 put the band in front, with a
chord sheet to type the song into.

Some ideas of the early roadmap are done along the way: a mixer, MIDI
file import, notes that start between steps (swing, pushes, strummed
guitars, imported MIDI), time signature changes partway through a song,
and installable downloads for Linux, Windows and macOS, with an AppImage
and a `.deb` for Linux.

## Ideas for what comes next

None of these are planned yet. They are ideas, roughly from smaller to
bigger:

- **More of the band**: more styles, and more instruments it can
  bring to any of them, as it brings the horns now.
- **A soloist that is no longer an experiment.**
- **Automation**: knob movements recorded or drawn over time, on the
  tracks.
- **Audio clips** on the tracks, next to the patterns, to play along
  with a recording.
- **Recording in song mode** (exercise 5 of step 7).
