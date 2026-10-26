# Step 2: The channel rack

> The code shown in step 2's chapters is the code as it was at the end of
> step 2. Step 3 changed some of it: patterns now hold notes instead of
> `[bool; 64]` rows, `Command::SetStep` became `Command::SetNotes`, and
> instruments got voices. Step 4 changed more: a project holds many
> patterns, so `SetNotes` also names the pattern, and the rack became a
> floating window that shows the selected pattern. This chapter is a
> historical snapshot, and the old `step-2-sequencer` checkpoint branch
> is no longer published.

Step 2 turns the test tone into a real, if small, drum machine:

- a **channel rack** with one row per instrument, each with a mute light,
  a volume knob, a name button and a row of **steps**,
- a **step sequencer** that plays the pattern in a loop at the project
  tempo, with sample-accurate timing and a moving playhead,
- a **built-in drum kit** (kick, snare, clap, hi-hat) synthesized from
  scratch, so no sound files need to be shipped,
- a **sampler** that plays WAV files you drag onto the rack,
- the **Lineup Osc**, a simple synth channel (a filtered sawtooth with a
  pluck envelope) that step 3 will give melodies to.

![The channel rack playing the demo beat](../images/step-2-rack.png)

## How to use it

| Do this | To |
|---------|----|
| Click a step | switch it on or off |
| Right-click a step | switch it off |
| Space or ▶ Play | start and stop the pattern |
| Click a channel name | hear that channel once |
| Right-click a channel name | remove the channel |
| Drag the knob up or down | change the channel volume (double-click resets it) |
| Click the green light | mute or unmute the channel |
| Drop a `.wav` file on a channel | replace its sound with the file |
| Drop a `.wav` file below the rows | add a new channel playing it |
| ➕ Add channel | add a drum or a Lineup Osc |

## What changed in the architecture

```text
  UI thread (lineup-app)                         Audio thread (lineup-audio)
  ─────────────────────                        ─────────────────────────
  Project (lineup-core)                          AudioThread
   ├─ tempo                 Command queue       ├─ Sequencer (lineup-core)
   ├─ channels       ───────────────────▶       ├─ channels: Vec<ChannelState>
   └─ pattern                                   │    └─ Instrument + steps
                                                │
  channel rack UI   ◀── playhead (atomic) ──────┤
                                                │
  collect_garbage() ◀── trash queue ────────────┘  old instruments to free
```

Three new ideas carry this step:

1. **The UI owns the project; the audio thread keeps its own copy** of
   just what it needs to make sound. Every change the user makes is
   applied to the project and also sent as a `Command`. The audio thread
   never reads the UI's data, so the two never need a lock.
2. **Information flows back without locks too.** The current step goes
   back through an *atomic* integer, and instruments the audio thread no
   longer needs go back through a second ring buffer so their memory is
   freed on the UI thread.
3. **Drawing is separated from changing.** The rack's drawing code
   returns a list of `RackAction`s, and the app applies them in one place.

## Files in this step

```text
crates/lineup-core/src/pattern.rs     Pattern: the grid of steps
crates/lineup-core/src/sequencer.rs   Sequencer: when does each step start?
crates/lineup-core/src/project.rs     Project, Channel, ChannelKind, Drum
crates/lineup-audio/src/dsp.rs        Smoothed, Noise, HighPass, LowPass
crates/lineup-audio/src/drums.rs      the synthesized drum kit
crates/lineup-audio/src/sample.rs     SampleData and WAV loading
crates/lineup-audio/src/instrument.rs Instrument: Sampler and Synth (Lineup Osc)
crates/lineup-audio/src/engine.rs     new Commands, the trash queue, the playhead
crates/lineup-app/src/channel_rack.rs the rack: rows, steps, drag and drop
crates/lineup-app/src/widgets.rs      step button, knob and mute light
crates/lineup-app/src/app.rs          applies rack actions to project and audio
```

The test tone from step 1 (`synth.rs`) is gone. Its click-free volume
trick lives on as `dsp::Smoothed`, and its oscillator grew into the Lineup Osc.
