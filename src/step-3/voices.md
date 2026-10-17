# Playing notes: voices and envelopes

Files: [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/instrument.rs)
and [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/engine.rs)

In step 2, an instrument could only be *triggered*: start the sound
again, always at the same pitch. To play chords and melodies it needs to
play several notes at once, each at its own pitch, and hold each one for
its own length.

## Sending notes to the audio thread

The audio thread keeps its own copy of each channel's notes, as it kept
a copy of the steps before. `Command::SetStep` is gone; instead, the UI
sends the whole list again whenever anything in it changes:

```rust
pub enum Command {
    // ...
    AddChannel { instrument: Instrument, notes: Vec<NoteEvent>, volume: f32, muted: bool },
    SetNotes { channel: usize, notes: Vec<NoteEvent> },
    PreviewNote { channel: usize, pitch: Note },
}
```

Sending a whole list for one changed note sounds wasteful, but a list of
a few hundred notes is a few kilobytes, copied on the UI thread. It
keeps the audio side trivial: swap the old list for the new one.

The new list was allocated by the UI, and the old one must not be freed
on the audio thread (freeing memory is as forbidden as allocating it).
So the trash queue from step 2 now carries two kinds of garbage, and an
enum says which:

```rust
enum Garbage {
    Instrument(Instrument),
    Notes(Vec<NoteEvent>),
}

Command::SetNotes { channel, notes } => match self.channels.get_mut(channel) {
    Some(state) => {
        let old = std::mem::replace(&mut state.notes, notes);
        self.throw_away(Garbage::Notes(old));
    }
    None => self.throw_away(Garbage::Notes(notes)),
},
```

`Garbage` is private to `engine.rs`. `AudioEngine` is public, but its
fields are private, so a private type can sit inside one of them
(`trash: rtrb::Consumer<Garbage>`) without ever being visible to other
crates. On the UI thread, `collect_garbage` empties the queue and drops
each item, which is when the memory is actually freed.

## Starting notes on time

When the sequencer fires a step, every note that starts there begins
playing. Its **gate**, how long it is held, is its length in steps turned
into frames:

```rust
fn start_notes(&mut self, step: usize) {
    let step_frames = self.sequencer.step_frames();
    let length = self.sequencer.length();
    for channel in &mut self.channels {
        if channel.muted {
            continue;
        }
        for note in channel.notes.iter().filter(|note| note.start == step) {
            // A note that runs past the end of the pattern is cut there.
            let steps_held = note.length.min(length.saturating_sub(step));
            let gate = (steps_held as f64 * step_frames) as u32;
            channel
                .instrument
                .note_on(note.pitch, note.velocity, gate, self.sample_rate);
        }
    }
}
```

The inner loop reads `channel.notes` while it changes `channel.instrument`.
A shared borrow and a mutable borrow at the same time would normally be
refused, but these are borrows of **two different fields** of the same
struct, and the compiler can see they do not overlap. (This only works
when the fields are named directly. If `notes` were reached through a
method like `channel.notes()`, the method would borrow the whole
`channel` and the code would not compile.)

`saturating_sub` subtracts but stops at zero instead of overflowing: a
`usize` cannot be negative, and `0 - 1` would panic in a debug build.

Each note decides its own end time when it starts, so the audio thread
never has to remember to stop it. That also means editing or deleting a
note while it plays can never leave it stuck on: it still ends when its
gate runs out.

## Voices

A **voice** is one note's worth of sound-making: its own oscillator, its
own position in the envelope. The Lineup Osc has 16 voices and a sampler
has 8. Each `note_on` takes a voice:

```rust
trait Voice {
    fn is_idle(&self) -> bool;
    fn started(&self) -> u64;
}

fn pick_voice<V: Voice>(voices: &mut [V]) -> &mut V {
    let index = voices
        .iter()
        .position(Voice::is_idle)
        .or_else(|| {
            voices
                .iter()
                .enumerate()
                .min_by_key(|(_, voice)| voice.started())
                .map(|(index, _)| index)
        })
        .unwrap_or(0);
    &mut voices[index]
}
```

The search order is: a silent voice if there is one, otherwise the voice
whose note started longest ago. Taking over a busy voice is called
**voice stealing**; every hardware synth does it when you hold more notes
than it has voices.

This is the first **trait** Ferrous Lineup defines itself. `SamplerVoice`
and `SynthVoice` are different structs, but both implement `Voice`, so
one generic function serves both. `V: Voice` is the trait bound: the
function may only call what the trait promises. The trait is private
(no `pub`), because it is a detail of this file.

Why look up an index first and then return `&mut voices[index]`, instead
of returning the voice the search found? The searches use `iter()`, which
borrows the slice to read. Returning a mutable reference out of a
read-only search is not possible, so the code finds the position and
then borrows mutably once, at the end.

`started` comes from a counter that goes up with every note, so "started
longest ago" is "smallest counter". A `u64` will not overflow: at a
thousand notes per second it would take 584 million years.

### A fixed number of voices, in a box

```rust
pub struct Synth {
    sample_rate: f32,
    voices: Box<[SynthVoice; SYNTH_VOICES]>,
    envelope: EnvelopeRates,
    notes_started: u64,
}

voices: Box::new(std::array::from_fn(|_| SynthVoice::new(sample_rate))),
```

The voices are a fixed-size array: the audio thread never grows or
shrinks it, so it never allocates. `std::array::from_fn` builds an array
by calling a closure once per element (the closure gets the index, which
is not needed here, hence `|_|`).

The array is put in a `Box`, on the heap, because of the command queue.
Each slot of the queue is as big as the biggest `Command`, and
`AddChannel` carries an `Instrument`. Sixteen voices are about 640 bytes;
a `Box` is one 8-byte pointer. A test pins the size down:

```rust
#[test]
fn commands_stay_small() {
    let size = std::mem::size_of::<Command>();
    assert!(size <= 64, "a command takes {size} bytes");
}
```

Step 2 warned that sending a `Box<Instrument>` would make the audio
thread free memory when it is done with it. That is still true, which is
why instruments the audio thread is done with always go to the trash
queue, boxed voices and all. The box is created and freed on the UI
thread; the audio thread only moves the pointer.

## The envelope: a state machine

A held note should not just switch on and off: that clicks, and sounds
nothing like an instrument. An **envelope** shapes the volume over time.
The classic one has four parts, **ADSR**:

```text
 volume
   1 ┤  /\
     │ /  \___________          A: attack, rising to full volume
 0.45┤/    D    S      \        D: decay, falling to...
     │                  \       S: sustain, held while the note is held
   0 ┼───────────────────\___   R: release, fading out after the note ends
       A       gate ends ^
```

Each voice is in one **stage**, an enum, and every sample may move it
to the next stage. Code shaped like this is called a **state machine**:

```rust
enum Stage { Idle, Attack, Decay, Release }

match self.stage {
    Stage::Idle => return 0.0,
    Stage::Attack => {
        self.level += envelope.attack_step;
        if self.level >= 1.0 {
            self.level = 1.0;
            self.stage = Stage::Decay;
        }
    }
    Stage::Decay => {
        self.level = SYNTH_SUSTAIN + (self.level - SYNTH_SUSTAIN) * envelope.decay_factor;
    }
    Stage::Release => {
        self.level *= envelope.release_factor;
        if self.level < SILENT {
            self.stage = Stage::Idle;
            return 0.0;
        }
    }
}

// Count down the gate while the note is held, then let go.
if matches!(self.stage, Stage::Attack | Stage::Decay) {
    match self.gate.checked_sub(1) {
        Some(left) => self.gate = left,
        None => self.stage = Stage::Release,
    }
}
```

- Decay and release are **exponential**: each frame keeps a fixed
  fraction of the distance left, so the sound fades the way real strings
  and bells do. `decay_factor` is `e^(-1 / (seconds × sample_rate))`,
  computed once per instrument in `EnvelopeRates::new`, not per sample.
- `checked_sub` returns `None` instead of going below zero: "if there is
  no gate left, release".
- There is no `Sustain` variant. Decay approaches the sustain level and
  stays there, so a separate stage would do nothing.

`Instrument::release_all` moves every held voice to `Release`. The
engine calls it on Stop, so long notes fade out instead of playing on
after the music stopped.

## Samplers play pitches too

A sampler plays a note by playing its sample faster or slower. One
octave up is twice as fast, and each semitone multiplies the speed by
the twelfth root of two, the same rule `Note::frequency` uses:

```rust
let semitones = pitch.midi() as f64 - STEP_NOTE.midi() as f64;
let pitch_ratio = 2.0_f64.powf(semitones / 12.0);
let rate_ratio = self.sample.sample_rate() as f64 / output_rate as f64;
// ...
increment: rate_ratio * pitch_ratio,
```

At `STEP_NOTE` (C4), the ratio is 1, so steps sound exactly as before.
Open the piano roll on the Kick and draw a melody to hear a pitched drum.
Samplers ignore the gate: like a drum machine, they play the whole sound
every time. Because they have voices now, a new hit no longer cuts off
the one before; the two overlap.

## Checking it by ear, without ears

The container that builds Ferrous Lineup has no speakers, so this step
was also checked by recording the app's output to a file and looking at
its spectrum. During the first chord the strongest frequencies were
131, 261, 329 and 392 Hz: C3, C4, E4 and G4, the notes of the demo's C
major chord. The unit tests do a simpler version of the same thing:
`synth_plays_the_right_pitch` counts how often the output crosses zero
while playing A4, and expects about 880 crossings per second (two per
cycle of 440 Hz).
