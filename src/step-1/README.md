# Step 1: The foundation

Step 1 builds the skeleton that every later feature hangs on:

- a **window** with a transport bar (play button, tempo, test tone controls)
  and a status bar,
- a **real-time audio engine** that opens the sound card and plays a sine
  wave test tone,
- a **message queue** that lets the window control the audio without ever
  making the audio wait,
- **tests** for the music and audio code, and **continuous integration**
  that builds and tests every change on Linux.

It does not make music yet. That begins in step 2, with the step sequencer.
But the hardest architectural decision, how the UI and the audio thread
talk to each other, is made here, and everything else builds on it.

## The big picture

```text
  UI thread (loom-app)                    Audio thread (loom-audio)
  draws the window ~60 times a second     asked for samples every few ms
  reacts to clicks and keys               must never wait or allocate

  LoomApp ──Command──▶ ring buffer (rtrb) ──▶ AudioThread ──▶ sound card
                                                 └── TestTone

  Both sides use loom-core (Note, Tempo): pure music logic, no I/O.
```

## Files added in this step

```text
Cargo.toml                    the workspace: lists the three crates
crates/
  loom-core/src/lib.rs        crate root, re-exports Note and Tempo
  loom-core/src/pitch.rs      Note: MIDI numbers, names, frequencies
  loom-core/src/tempo.rs      Tempo: beats per minute, beat lengths
  loom-audio/src/lib.rs       crate root
  loom-audio/src/synth.rs     TestTone: a click-free sine generator
  loom-audio/src/engine.rs    AudioEngine: sound card, audio thread, commands
  loom-app/src/main.rs        program entry point, opens the window
  loom-app/src/app.rs         LoomApp: everything drawn in the window
docs/                         this book
.github/workflows/ci.yml      automatic build and test on GitHub
```

The next chapters walk through these files in order, from the outside in.
