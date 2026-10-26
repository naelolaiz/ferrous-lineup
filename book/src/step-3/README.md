# Step 3: The piano roll and harmony

> The code shown in step 3's chapters is the code as it was at the end of
> step 3. Step 4 changed some of it: a project now holds many patterns,
> so the note methods of `Project` (`add_note`, `replace_note`...) gave
> way to `project.pattern_mut(index)`, `Command::SetNotes` also names the
> pattern, and the piano roll edits the selected pattern. This chapter is
> a historical snapshot, and the old `step-3-piano-roll` checkpoint
> branch is no longer published.

Step 3 gives Ferrous Lineup melodies and chords, and starts teaching harmony:

- a **piano roll** window where you draw notes on a grid of pitches and
  steps, move them, stretch them and delete them,
- **scale highlighting**: pick a key (say, A minor) and the rows of its
  notes light up, the root is tinted, and notes outside the key turn red,
- **chord stamping**: one click can place a whole chord, either a fixed
  shape (major, minor seventh...) or "the chord of the key" built on the
  note you click,
- **live chord names**: a lane above the grid names the chord at every
  step (C, Am, F/A, G7...), and a line of text explains it in the key:
  "Am: A minor, chord vi of C major",
- **chords in key**: seven buttons with the chords of the current key and
  their Roman numerals (I, ii, iii, IV, V, vi, vii°). Click one to hear it,
- a **polyphonic** Lineup Osc that plays chords, with notes held for their
  length, and samplers that play higher or lower with the note.

![The piano roll playing the demo progression](../images/step-3-piano-roll.png)

The demo now plays a classic chord progression, C, Am, F, G (I, vi, IV,
V), over two bars of the beat from step 2.

## How to use it

| Do this | To |
|---------|----|
| F7, or the 🎹 Piano roll button | open or close the piano roll |
| Right-click a channel name, then Piano roll | open the piano roll on that channel |
| Click the small note preview in the rack | open the piano roll on that channel |
| Click an empty cell | add a note, or a chord (see Draw) |
| Drag a note | move it in time and pitch |
| Drag the right end of a note | make it longer or shorter |
| Right-click a note, or right-drag across notes | delete them |
| Click a piano key | hear that note |
| Key and scale menus | choose the key to highlight and name chords in |
| Draw menu | choose what one click places |
| Length menu | choose how long new notes are |
| Point at a step | see the chord there explained under the toolbar |

A channel whose notes are all plain steps still shows step buttons in
the rack. As soon as it has a melody or chords, the rack shows a small
picture of its notes instead.

## What changed in the architecture

```text
  lineup-core                                lineup-audio
  ─────────                                ──────────
  harmony/  (new)                          Instrument
    PitchSet, Scale, Chord,                  ├─ Sampler: 8 voices, pitched
    analyze(), ChordStamp                    └─ Synth:  16 voices, ADSR envelope
  pattern.rs
    Pattern: Vec<NoteEvent> per channel   ──SetNotes──▶  ChannelState.notes
    (steps are 1-step notes)                             (old list → trash queue)

  lineup-app
    piano_roll.rs (new): the window, its own state, RollActions
    channel_rack.rs: step buttons or a mini piano roll per row
```

Two ideas carry this step:

1. **One kind of data, two views.** The channel rack and the piano roll
   both edit the same notes. A step that is "on" is just a note one step
   long at the default pitch. This is how pattern-based studios work, and
   it means the audio thread has a single kind of thing to play.
2. **Music theory as types.** Pitch classes, scales, chords and "what is
   playing" are all small Rust types, mostly enums, with methods. The
   compiler checks the theory code like any other code, and more than
   half of this step's tests are music theory tests.

## Files in this step

```text
crates/lineup-core/src/pitch.rs              + PitchClass and Spelling
crates/lineup-core/src/harmony.rs            the harmony module (new)
crates/lineup-core/src/harmony/pitch_set.rs  PitchSet: 12 bits, one per pitch class
crates/lineup-core/src/harmony/scale.rs      ScaleKind, Scale, chords of a key
crates/lineup-core/src/harmony/chord.rs      ChordQuality, Chord, ChordStamp
crates/lineup-core/src/harmony/analysis.rs   analyze() and Harmony: naming notes
crates/lineup-core/src/pattern.rs            NoteEvent; patterns hold notes now
crates/lineup-core/src/project.rs            the key, note editing, the new demo
crates/lineup-audio/src/instrument.rs        voices, pitch, envelopes
crates/lineup-audio/src/engine.rs            SetNotes, PreviewNote, the Garbage enum
crates/lineup-app/src/piano_roll.rs          the piano roll window (new)
crates/lineup-app/src/channel_rack.rs        mini piano roll in the rack
crates/lineup-app/src/widgets.rs             + mini_roll widget
crates/lineup-app/src/app.rs                 applies piano roll actions
```

The chapters go from the inside out: first the music theory, then notes
in patterns, then how the audio thread plays them, and last the window.
