# Instruments: drums, sampler and Lineup Osc

Files: [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs),
[`crates/lineup-audio/src/drums.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/drums.rs),
[`crates/lineup-audio/src/dsp.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs),
[`crates/lineup-audio/src/sample.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/sample.rs)
and [`crates/lineup-audio/src/timbre.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/timbre.rs)

Every channel has an instrument: the part that turns notes into sound.
This chapter is about the two that Ferrous Lineup makes itself, the
**sampler**, which plays a recorded or synthesized sound, and the
**Lineup Osc**, a small synthesizer. The other two kinds play through
something else: a SoundFont ([SoundFonts and General
MIDI](soundfonts.md)) or an external synth over MIDI
([MIDI](midi.md)). How the sounds are chosen in the app is in
the manual's
[Sounds](https://naelolaiz.github.io/ferrous-lineup/manual/sounds.html).

## One enum, four instruments

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    SoundFont(SoundFontPlayer),
    MidiOut(MidiOut),
}
```

A common design for "many kinds of one thing" is a trait, `trait
Instrument`, with values of type `Box<dyn Instrument>`. That is open:
anyone could add a kind without touching this file. But every `Box::new`
allocates, and every call goes through a pointer to a table of functions.
With four kinds, all known, an enum is simpler. The audio thread
`match`es on it, making one never allocates, and when a fifth kind is
added the compiler lists every `match` that has to learn about it.

Every method of `Instrument` is a `match` that hands the call to the
kind inside:

```rust
pub fn release_all(&mut self) {
    match self {
        Instrument::Sampler(sampler) => sampler.release_all(),
        Instrument::Synth(synth) => synth.release_all(),
        Instrument::SoundFont(player) => player.release_all(),
        Instrument::MidiOut(midi_out) => midi_out.release_all(),
    }
}
```

What an instrument can be asked:

| Method | What it does |
|---|---|
| `note_on` | Starts a note: a pitch, a velocity and a **gate**, how many frames it is held |
| `note_on_from` | The same, for a note of the song, which remembers where it came from (`Origin`, see [The sequencer](sequencer.md)) |
| `note_off` | Lets go of a note held with the gate `HELD` |
| `release_all` | Lets go of every held note, when playback stops |
| `release_unless` | Lets go of the song's notes that the song no longer plays |
| `resume` | Starts a long sound partway through, where it would be had playback started earlier (the sampler only) |
| `tempo_changed` | Stretches held notes after a change of tempo ([Tempo, meter, swing and feel](tempo.md)) |
| `next_sample` | The next frame of sound: the sum of every voice |

A note of the song knows its length when it starts, so it gets a gate
in frames and ends by itself; the audio thread never has to remember to
stop it. A key pressed on a MIDI keyboard has no known length, so it
gets `HELD`, `u32::MAX`, which never runs out: only `note_off` ends it.

The instrument for a channel is built by `Instrument::for_channel`, on
the UI thread, from the project's `Channel`:

```rust
Ok(match &channel.kind {
    // A drum sent over MIDI is its note of a General MIDI drum set.
    ChannelKind::Sampler(SampleSource::Builtin(drum)) => match channel.sent_on {
        Some(number) => Instrument::MidiOut(
            MidiOut::new(number).playing_as(Some(general_midi_drum(*drum))),
        ),
        None => {
            Instrument::Sampler(Sampler::new(Arc::new(drums::render(*drum, sample_rate))))
        }
    },
    ChannelKind::Sampler(SampleSource::File(path)) => {
        Instrument::Sampler(Sampler::new(samples.get(path)?))
    }
    ChannelKind::Synth => Instrument::Synth(Synth::with_timbre(
        sample_rate,
        Timbre::for_program(channel.program),
    )),
    ChannelKind::MidiOut { channel: number } => {
        Instrument::MidiOut(MidiOut::new(*number).with_program(channel.program))
    }
})
```

Drums are synthesized here and files read here (through the
[`SampleCache`](../program/sync.md)), which is slow work that may
allocate: exactly what the audio thread must not do. The finished
instrument is sent over in a command.

## Drums from a few lines of math

Ferrous Lineup ships no sound files. Its thirteen drums are computed
when a drum channel is created, each in a few lines:

| Drum | Recipe |
|---|---|
| Kick | A sine whose pitch falls fast from 150 to 45 Hz, fading over half a second |
| Snare | A short 185 Hz tone (the shell) plus high-passed noise (the wires) |
| Clap | High-passed noise in three bursts 11 ms apart, like several hands, and a tail |
| Hi-hat | Very bright noise, high-passed at 7 kHz, gone in about a tenth of a second |
| Open hat, ride, crash | "Metal" (below) and noise, ringing longer and longer |
| Rim | A 1750 Hz click, a 480 Hz body and a snap of noise |
| Low and high tom | Like the kick, higher and rounder, settling at 98 and 165 Hz |
| Cowbell | Two square waves at 540 and 800 Hz, a little less than a fifth apart |
| Shaker | Bright noise that swells for 15 ms, then dies away quickly |
| Conga | A 320 Hz tone that drops a little, and the slap of a hand |

Most of them share one shape, the **exponential decay**:

```rust
fn decay(t: f32, tau: f32) -> f32 {
    (-t / tau).exp()
}
```

It is 1 at the start and falls to about 37% after `tau` seconds, 13%
after twice that, and so on. Struck things fade this way.

### Closures that remember: `FnMut`

All the drums are built with one helper:

```rust
fn generate(seconds: f32, sample_rate: u32, f: impl FnMut(f32) -> f32) -> Vec<f32> {
    let frames = (seconds * sample_rate as f32) as usize;
    (0..frames)
        .map(|i| i as f32 / sample_rate as f32)
        .map(f)
        .collect()
}
```

It takes a closure `f` from a time in seconds to a sample. The kick's
closure keeps a running `phase` between calls:

```rust
fn kick(sample_rate: u32) -> Vec<f32> {
    let mut phase = 0.0;
    generate(0.5, sample_rate, |t| {
        let frequency = 45.0 + 105.0 * decay(t, 0.04);
        phase += frequency / sample_rate as f32;
        (phase * TAU).sin() * decay(t, 0.15)
    })
}
```

The closure *changes* `phase`, a variable it captured from `kick`. A
closure that changes what it captures implements the **`FnMut`** trait;
`Fn` is for closures that only read, and `FnOnce` for those that can be
called only once. `impl FnMut(f32) -> f32` in the parameter list means
"any closure of that shape", and the compiler makes a copy of `generate`
for each drum's closure, as it does for generic types.

Why add up a phase instead of computing `sin(t × frequency × 2π)`?
Because the frequency changes over time. The phase is the sum of all
the small steps taken so far, so the pitch sweeps smoothly; the direct
formula would jump around as the frequency changed.

### Metal from six square waves

The cymbals use a trick from the analog drum machines of the early
1980s: six square waves at frequencies that are not multiples of each
other, added together.

```rust
const METAL: [f32; 6] = [205.3, 304.4, 369.6, 522.7, 540.0, 800.0];

fn metal(t: f32, ratio: f32) -> f32 {
    METAL.iter().map(|&f| square(t, f * ratio)).sum::<f32>() / METAL.len() as f32
}
```

Notes whose frequencies are simple multiples of each other blend into a
chord. These don't, so together they clash and shimmer like metal. A
high-pass filter keeps the bright part, and the envelope decides whether
it is a closed hat, an open hat, a ride or a crash.

Every drum is then scaled with `normalize` so that its loudest sample
is 0.9, and two tests go through `Drum::ALL`: every drum reaches that
peak and is longer than a thousand frames
(`every_drum_is_audible_and_in_range`), and every drum ends quietly
instead of being cut off (`every_drum_fades_out`).

## Noise and filters

`dsp.rs` holds the small building blocks. **White noise** comes from
xorshift, a tiny random number generator:

```rust
pub fn next_sample(&mut self) -> f32 {
    let mut x = self.state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    self.state = x;
    (x as f32 / u32::MAX as f32) * 2.0 - 1.0
}
```

Three shifts and three exclusive-ors. It is no good for security, but it
sounds just like noise, never allocates, needs no crate, and gives the
same "random" drums every time, which makes them testable. Its one trap
is zero: xorshift from 0 stays at 0 for ever, so `Noise::new` starts
from `seed.max(1)`.

The filters are **one-pole** filters, the simplest there are. The
low-pass moves its output a fraction of the way toward the input on
every sample, so fast changes (high frequencies) are smoothed away:

```rust
pub fn process(&mut self, input: f32) -> f32 {
    self.state += (input - self.state) * self.coefficient;
    self.state
}
```

That is the same arithmetic as `Smoothed`, the volume that glides
instead of jumping ([Two threads](../program/threads.md)). The
high-pass follows the *changes* in its input instead, so a steady signal
dies away and fast changes pass. The tests say exactly that: a constant
input goes through the low-pass and is removed by the high-pass.

## The sampler

```rust
pub struct Sampler {
    sample: Arc<SampleData>,
    /// Boxed so an `Instrument` stays small: commands carrying one are
    /// copied into the command queue, and a pointer is cheaper to copy
    /// than eight voices. The box is created here, on the UI thread.
    voices: Box<[SamplerVoice; SAMPLER_VOICES]>,
    notes_started: u64,
}
```

`Arc<T>` ("atomically reference-counted") is a pointer that several
owners share. Cloning one does not copy the data, it adds one to a
counter, and the data is freed when the last `Arc` goes. The counter is
updated atomically, so it is safe across threads, which is what lets a
sample be built on the UI thread and played on the audio thread. The
same sound file in three channels is in memory once.

Each of the eight voices plays one hit:

```rust
fn next_sample(&mut self, frames: &[f32]) -> f32 {
    let Some(position) = self.position else {
        return 0.0;
    };
    let index = position as usize;
    if index + 1 >= frames.len() {
        self.position = None;
        return frames.get(index).copied().unwrap_or(0.0) * self.gain;
    }
    // Linear interpolation between the two nearest frames.
    let fraction = (position - index as f64) as f32;
    let sample = frames[index] + (frames[index + 1] - frames[index]) * fraction;
    self.position = Some(position + self.increment);
    // ...
}
```

`let ... else` matches a pattern, or runs the `else` block, which must
leave the function. It keeps the main path unindented: "not playing,
so silence; otherwise carry on with `position`".

`position` is a fraction of a frame, because a sound rarely moves
through its frames at exactly one per output frame. A WAV recorded at
44.1 kHz played on a 48 kHz sound card must move a little slower, or it
would sound higher and shorter. And a sampler plays pitches by playing
faster or slower: an octave up is twice as fast, and each semitone
multiplies the speed by the twelfth root of two, the same rule
`Note::frequency` uses ([Pitch classes, notes and
spelling](../theory/pitch.md)):

```rust
// Every semitone up plays the sample 2^(1/12) times faster.
let semitones = pitch.midi() as f64 - STEP_NOTE.midi() as f64;
let pitch_ratio = 2.0_f64.powf(semitones / 12.0);
let rate_ratio = self.sample.sample_rate() as f64 / output_rate as f64;
```

The voice's `increment` is the two ratios multiplied. On `STEP_NOTE`, C4,
the note a step button plays, the pitch ratio is 1 and the sound plays
as recorded. Between two stored frames the voice draws a straight line,
**linear interpolation**: not the best a sampler can do, but cheap and
good enough for drums.

The velocity scales the volume, `velocity / 127`. A sampler ignores the
gate: like a drum machine, it plays its whole sound every time, and
because it has voices, a new hit does not cut off the one before.

That is right for drums and wrong for one thing: a long recording, such
as a whole song imported as audio, would play on after Stop. So
`release_all` fades out the voices of a sound longer than ten seconds,
in about 15 ms, and leaves short hits ringing:

```rust
pub fn release_all(&mut self) {
    let long = self.sample.frames().len() as u64
        > u64::from(self.sample.sample_rate()) * LONG_SAMPLE_SECONDS;
    if long {
        for voice in self.voices.iter_mut().filter(|voice| !voice.is_idle()) {
            voice.fading.get_or_insert(1.0);
        }
    }
}
```

`release_unless`, for a clip taken away or a track muted while it plays,
fades the song's sounds whatever their length.

### Starting partway

A note sounds when playback reaches the step it starts on. For a drum
hit that is all there is to it, but a recording is one note on the
first step of a song: started from bar 20, the song would play without
it. So when playback starts, jumps, or comes round to the start of the
loop, the engine's `resume_long_sounds` looks back over the song for
the notes that started before the playhead, and asks each instrument
to `resume` them:

```rust
fn resume(
    &mut self,
    pitch: Note,
    velocity: u8,
    output_rate: u32,
    origin: Origin,
    elapsed: f64,
) -> bool {
    let increment = self.increment(pitch, output_rate);
    let position = elapsed * increment;
    if !self.is_long() || position + 1.0 >= self.sample.frames().len() as f64 {
        return false;
    }
    self.start_at(position, increment, velocity, Some(origin));
    true
}
```

`elapsed` is how many output frames ago the note started, and a voice
moves `increment` frames of the sound for each output frame, so the
voice starts `elapsed * increment` frames in, as far as it would have
got. Only a long sound, over ten seconds, is started this way, and only
if it lasts that long: a drum hit from two bars ago has long finished,
and other instruments answer `false`. `resume_long_sounds` goes through
every piece of a clip that starts at or before the step, and every time
the piece played each note:

```rust
for note in channel.notes.get(play.pattern) {
    // Every time the piece played the note before
    // `step`, counted from the piece's start, which is
    // below zero for a time in the part of the clip the
    // piece skips.
    let first = play.start as isize - play.skip as isize + note.start as isize;
    let times = (first..play.end().min(step) as isize).step_by(length);
    for at in times.filter(|&at| at < step as isize) {
        let origin = Origin {
            pattern: play.pattern,
            pattern_step: note.start,
            song_step: at.max(0) as usize,
            over_song: false,
        };
        let elapsed = frames_between(timing, at, step);
        let started = channel.instrument.resume(
            origin,
            note.pitch,
            note.velocity,
            self.sample_rate,
            elapsed,
        );
        // ...
    }
}
```

`first` is where the note falls if the piece started at its own first
step; it is below zero when the clip leaves out its start (`skip`), and
`step_by(length)` gives each repeat of the pattern within the piece.
`frames_between` adds up the frames of the steps from there to the
playhead at the song's tempo on each step, so a recording under a
tempo change starts at the right place too.

### Loading a WAV file

`SampleData::load_wav` reads WAV files with the
[`hound`](https://docs.rs/hound) crate. A WAV stores samples as floats
or as integers of 8 to 32 bits, and both become floats between -1 and 1:

```rust
let interleaved: Vec<f32> = match spec.sample_format {
    hound::SampleFormat::Float => reader
        .samples::<f32>()
        .collect::<Result<_, _>>()
        .map_err(SampleError::Wav)?,
    hound::SampleFormat::Int => {
        let full_scale = (1_i64 << (spec.bits_per_sample - 1)) as f32;
        reader
            .samples::<i32>()
            .map(|sample| sample.map(|s| s as f32 / full_scale))
            .collect::<Result<_, _>>()
            .map_err(SampleError::Wav)?
    }
};
```

`samples()` gives a `Result` per sample, because a file can be cut off
halfway. Collecting an iterator of `Result`s into a `Result<Vec<_>, _>`
gives `Ok` with every value if all were `Ok`, or the first `Err`, and
stops there. The `_`s let the compiler fill in the types. Then
`interleaved.chunks(channels)` takes one frame at a time (left, right)
and averages it, so a stereo file becomes mono.

`SampleError` implements `Display` with translated messages, so "could
not read the WAV file" reaches the user in their language. The test
`loads_and_mixes_to_mono` writes a real 16-bit stereo WAV to the
system's temporary folder with `hound::WavWriter` and reads it back.

## Voices

A **voice** is one note's worth of sound-making: its own place in the
sound, its own oscillator, its own envelope. The Lineup Osc has 16
voices and a sampler 8. Each new note takes one:

```rust
trait Voice {
    /// Whether the voice is silent and free to play a new note.
    fn is_idle(&self) -> bool;
    /// When the voice's note started, as a count of notes started so far.
    fn started(&self) -> u64;
    /// Whether the voice's note has ended and it is only fading out.
    fn is_releasing(&self) -> bool;
}

fn pick_voice<V: Voice>(voices: &mut [V]) -> &mut V {
    let oldest = |releasing: bool| {
        voices
            .iter()
            .enumerate()
            .filter(|(_, voice)| !releasing || voice.is_releasing())
            .min_by_key(|(_, voice)| voice.started())
            .map(|(index, _)| index)
    };
    let index = voices
        .iter()
        .position(Voice::is_idle)
        .or_else(|| oldest(true))
        .or_else(|| oldest(false))
        .unwrap_or(0);
    &mut voices[index]
}
```

The order is: a silent voice if there is one; otherwise the oldest voice
that is only fading out; otherwise the oldest of all. Taking a busy
voice is **voice stealing**, and every hardware synth does it when more
notes are held than it has voices. A fading voice goes first because
cutting a tail is far less noticeable than cutting a note still held,
such as a long bass note under quick chords (the test
`voices_that_are_fading_out_are_stolen_first` plays exactly that).

`Voice` is a private trait: `SamplerVoice` and `SynthVoice` are
different structs, both implement it, and one generic function serves
both. `V: Voice` is the trait bound; the function may only call what the
trait promises.

Why find an index first and only then return `&mut voices[index]`? The
searches borrow the slice to read it (`iter()`). A mutable reference
can't come out of a read-only search, so the code finds the position,
lets the read borrows end, and borrows mutably once at the end.

`started` comes from a counter that goes up with every note, so "oldest"
is "smallest". A `u64` won't overflow: at a thousand notes a second it
would take 584 million years.

### A fixed number of voices, in a box

```rust
voices: Box::new(std::array::from_fn(|_| {
    SynthVoice::new(sample_rate, timbre.cutoff)
})),
```

The voices are a fixed-size array, so the audio thread never grows it
and never allocates. `std::array::from_fn` builds an array by calling
the closure once per element; the closure gets the index, which is not
needed here, hence `|_|`. The sampler's voices implement `Default`, so
`Box::default()` builds them.

The array is in a `Box` because of the command queue to the audio
thread. Every slot of the queue is as big as the biggest `Command`, and
a command that inserts a channel carries an `Instrument`. Sixteen voices
are over a kilobyte; a `Box` is one 8-byte pointer. A test pins
the size down:

```rust
let size = std::mem::size_of::<Command>();
assert!(size <= 80, "a command takes {size} bytes");
```

Boxing means the voices live on the heap, and freeing them is freeing
memory. That is why an instrument the audio thread is done with always
goes back to the UI thread in the trash queue ([Two
threads](../program/threads.md)): the box is made and freed on the UI
thread, and the audio thread only moves the pointer.

## The Lineup Osc

The Lineup Osc plays one oscillator per voice, through a soft low-pass
filter, shaped by a volume **envelope**:

```rust
pub struct Synth {
    sample_rate: f32,
    voices: Box<[SynthVoice; SYNTH_VOICES]>,
    /// Boxed, like the voices, so an `Instrument` stays small.
    envelope: Box<EnvelopeRates>,
    notes_started: u64,
}
```

### The wave

An oscillator keeps a **phase**, from 0 to 1 through one cycle, and the
wave says what each phase sounds like:

```rust
pub fn at(self, phase: f32) -> f32 {
    match self {
        // Ramps from -1 to 1.
        Wave::Saw => phase * 2.0 - 1.0,
        // 1 for the first half of the cycle, -1 for the second.
        Wave::Square => {
            if phase < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        // Up from -1 to 1 and back down.
        Wave::Triangle => 1.0 - 4.0 * (phase - 0.5).abs(),
        Wave::Sine => (phase * std::f32::consts::TAU).sin(),
    }
}
```

Each frame, the phase moves on by `frequency / sample_rate` and wraps
round with `fract()`. A sawtooth has every harmonic, so it is bright and
buzzy; a square only the odd ones, which sounds hollow; a triangle has
weak ones and sounds soft; a sine has none and sounds pure.

A plain sawtooth made like this contains frequencies above what the
sample rate can hold, and they fold back as faint metallic tones, which
is called **aliasing**. The low-pass filter hides most of it. Removing it
properly (with "PolyBLEP" corrections, or wavetables) would be a good
project.

### The envelope: a state machine

A note that just switches on and off clicks, and sounds like nothing
real. An envelope shapes its volume over time. The classic one has four
parts, **ADSR**:

```text
 volume
   1 ┤  /\
     │ /  \___________          A: attack, rising to full volume
     │/    D    S      \        D: decay, falling to...
     │                  \       S: sustain, held while the note is held
   0 ┼───────────────────\___   R: release, fading out after the note ends
       A       gate ends ^
```

Each voice is in one **stage**, and every sample may move it to the
next. Code shaped like this is a **state machine**:

```rust
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
        let sustain = envelope.sustain;
        self.level = sustain + (self.level - sustain) * envelope.decay_factor;
    }
    Stage::Release => {
        self.level *= envelope.release_factor;
        if self.level < SILENT {
            self.stage = Stage::Idle;
            return 0.0;
        }
    }
}

// Count down the gate while the note is held, then let go. A
// `HELD` gate is not counted: only `note_off` ends it.
if self.is_held() && self.gate != HELD {
    match self.gate.checked_sub(1) {
        Some(left) => self.gate = left,
        None => self.stage = Stage::Release,
    }
}
```

- The attack rises in a straight line; decay and release are
  **exponential**: each frame keeps a fixed fraction of the distance
  left, the way strings and bells fade. The fractions are worked out once
  per instrument in `EnvelopeRates::new`, not on every sample:
  `decay_factor` is `e^(-1 / (seconds × sample_rate))`.
- `checked_sub` returns `None` instead of going below zero, so it reads
  as "if there is no gate left, release".
- There is no `Sustain` stage. Decay approaches the sustain level and
  stays there, so a stage of its own would do nothing.
- `SILENT` is 0.0001, about -80 dB. An exponential fade never reaches
  zero, so the voice is called silent below it and freed.

`note_off` releases the voices of a pitch only if they were started
with the `HELD` gate. A key let go on a MIDI keyboard must not end the
same note that the song is playing on its own gate. `release_all` moves
every held voice to `Release`, which is how Stop lets long notes fade
instead of playing on.

A stolen voice is not reset: `start` sets the new pitch, gain and gate,
and leaves the level, the phase and the filter as they were.

```rust
voice.stage = Stage::Attack;
voice.started = self.notes_started;
voice.origin = origin;
// The level, phase and filter carry on from where they were, so a
// stolen voice does not jump (and click).
```

The attack then rises from wherever the level was. Each voice is also
turned down to `SYNTH_VOICE_GAIN`, 0.35, so a four-note chord does not
come out four times too loud.

### Timbres: sounding like a General MIDI instrument

The Lineup Osc can never sound like a real violin, but it can sound more
like a violin than like a piano. A channel with a General MIDI program
and no SoundFont plays a **timbre** chosen by the program's family:

```rust
pub struct Timbre {
    pub wave: Wave,
    /// Seconds to reach full volume.
    pub attack: f32,
    /// How quickly the volume falls to the sustain level (time constant,
    /// in seconds).
    pub decay: f32,
    /// The volume a held note settles at, 0 to 1. 0 for a sound that dies
    /// away by itself, like a piano or a plucked string.
    pub sustain: f32,
    /// How quickly a note fades after it is let go (time constant).
    pub release: f32,
    /// The low-pass filter's cutoff, in Hz: lower is darker.
    pub cutoff: f32,
    /// How loud, compared with the Lineup Osc's own sound.
    pub gain: f32,
}
```

Two small `const fn`s name the two ways instruments behave:

```rust
/// Struck or plucked: starts at once and dies away over `decay`.
const fn struck(wave: Wave, decay: f32, release: f32) -> Timbre {
    Timbre::new(wave, 0.003, decay, 0.0, release)
}

/// Bowed, blown or sung: swells in over `attack` and keeps going.
const fn held(wave: Wave, attack: f32, release: f32) -> Timbre {
    Timbre::new(wave, attack, 0.3, 0.85, release)
}
```

and `for_program` is one `match` on the program number, in General
MIDI's own order, where each family is a range of eight numbers:

```rust
let timbre = match program.min(127) {
    // Pianos.
    0..=3 => Timbre::struck(Wave::Saw, 0.9, 0.25).cutoff(1_800.0),
    // ...
    // Brass.
    56..=63 => Timbre::held(Wave::Saw, 0.04, 0.1).cutoff(2_000.0),
    // Reeds: saxophones, oboe, bassoon, clarinet.
    64..=71 => Timbre::held(Wave::Square, 0.03, 0.08).cutoff(1_800.0),
    // Pipes: flutes, recorder, ocarina.
    72..=79 => Timbre::held(Wave::Triangle, 0.05, 0.1).cutoff(3_000.0),
    // ...
    // Sound effects have no tune to play; the Lineup Osc stands in.
    _ => Timbre::LINEUP_OSC,
};
```

| Family | Wave | Envelope | Heard as |
|---|---|---|---|
| Piano, guitar, harp, pizzicato | sawtooth or triangle | starts at once, dies away even while held | struck or plucked |
| Strings, brass | sawtooth | swells in, keeps going | bowed or blown |
| Organ, accordion, reeds | square | full while held | hollow, reedy |
| Flutes, voices | triangle | soft attack, keeps going | soft and round |
| Bells, vibraphone, marimba | sine | dies away | pure |

Range patterns (`0..=3`), alternatives (`24..=28 | 31`) and a final `_`
make the whole table one expression, and the compiler checks that every
number from 0 to 255 lands somewhere. At the end, each timbre's gain is
multiplied by the wave's, so a square, the loudest wave, doesn't jump
out of the mix. `Timbre::for_program(None)` is `LINEUP_OSC`, the synth's
own bright sawtooth, so a channel without a program sounds as it always
did. Which program each channel gets is in [SoundFonts and General
MIDI](soundfonts.md).

## Testing it

The tests play an instrument into a buffer and look at the numbers,
since nobody can listen in a test:

- `synth_plays_the_right_pitch` plays A4 for a second and counts how
  often the output changes sign: about 880 times, two per cycle of
  440 Hz.
- `synth_stops_after_the_gate_and_release` holds a note for a tenth of a
  second and checks the output is exactly zero ten release times later,
  with no voice playing.
- `synth_plays_chords`, `voices_are_stolen_when_all_are_busy` and
  `voices_that_are_fading_out_are_stolen_first` check the voices.
- `sampler_resamples_to_the_output_rate` plays one second recorded at
  24 kHz on a 48 kHz output and expects 48 000 frames of sound, give or
  take ten; `sampler_plays_higher_notes_faster` expects half as many an
  octave up; `sampler_hits_can_overlap` plays two hits at once.
- `held_notes_last_until_note_off` checks the `HELD` gate.
- In `timbre.rs`, `plucked_instruments_die_away_and_bowed_ones_keep_going`
  checks the families, and `every_wave_stays_between_minus_one_and_one`
  the waves.
