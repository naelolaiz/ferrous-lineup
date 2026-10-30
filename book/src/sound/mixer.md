# The mixer: levels, pan, reverb, effects and score order

Files: [`crates/lineup-core/src/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/mixer.rs),
[`crates/lineup-core/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/effects.rs),
[`crates/lineup-core/src/score_order.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/score_order.rs),
[`crates/lineup-audio/src/reverb.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/reverb.rs),
[`crates/lineup-audio/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/effects.rs),
[`crates/lineup-audio/src/levels.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/levels.rs),
[`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs)
and [`crates/lineup-app/src/meters.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/meters.rs)

The mixer (**F9**) is where the band is balanced. Each channel has a
line: its instrument, a level meter, its volume, where it sits between
the speakers (**Pan**), how much of it goes into the room (**Reverb**),
its own **Effects** (an equalizer, a compressor and a delay), **Mute**
and **Solo**. Above them are the master volume with a meter for each
speaker, and the reverb that every channel shares. The lines are listed
the way a conductor's score lists instruments, in sections.

![The mixer playing El Choclo with the reverb on: the Keys panned to L30, the Lead (an accordion) to R30, and the drums sending 40% to the reverb.](../images/step-9-mixer.png)

The work is split as everywhere else in the program. `lineup-core`
holds the settings, which are saved with the song; `lineup-audio` turns
them into sound on the audio thread; `lineup-app` draws the window. How
to use the mixer is in the manual's
[The mixer](https://naelolaiz.github.io/ferrous-lineup/manual/mixer.html).

## What is saved

The project has one `Mixer`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mixer {
    pub master: f32,
    pub reverb: ReverbSettings,
    pub solo: Option<usize>,
}
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ReverbSettings {
    pub enabled: bool,
    /// Wet fraction, from 0 to 1.
    pub mix: f32,
    /// Approximate time to decay by 60 dB, in seconds.
    pub decay: f32,
    /// High frequency absorption, from 0 to 1.
    pub damping: f32,
    pub pre_delay_ms: f32,
}
```

and each `Channel` has its own mix: `volume` and `muted`, and three more
fields:

```rust
#[serde(default, skip_serializing_if = "is_centred")]
pub pan: f32,
#[serde(default = "full_send", skip_serializing_if = "is_full_send")]
pub reverb_send: f32,
#[serde(default, skip_serializing_if = "Effects::is_plain")]
pub effects: Effects,
```

`skip_serializing_if` leaves a field out of the file while it has the
value of a new channel (in the middle, everything sent to the reverb, no
effects), and `default` fills it back in when it is missing. A song
whose channels are all plain is written exactly as it would be by a
Ferrous Lineup with no pans or effects, so an older version still opens
it. Only a song that uses them needs format 26 (pan and send) or 28
(effects) ([The version
number](../program/saving.md#the-version-number)). Opening checks the
ranges: a file edited by hand with a pan of −3, or a delay whose echoes
would never die away, is refused with a message instead of playing a
channel louder than full.

**Solo** is one channel number, not a flag on each channel. The project
answers "is this channel heard?" in one place, so everything that plays
(the audio thread, an export, a MIDI file) agrees:

```rust
pub fn channel_muted(&self, channel: usize) -> bool {
    self.channels.get(channel).is_none_or(|state| state.muted)
        || self.mixer.solo.is_some_and(|solo| solo != channel)
}
```

That is why soloing a channel reaches the audio thread as a mute change
on every other channel ([`diff`: the list of
changes](../program/sync.md#diff-the-list-of-changes)).

**Reset mix** puts the master, the reverb, the solo and every channel's
volume, pan, send and effects back as they are on a new song. The band's
players get their usual volumes back, and channels the band keeps quiet
stay muted (`Project::reset_mix`, in `band.rs`).

## The path of one sample

For each frame, the audio thread takes every channel through the same
steps. This is the heart of the mix, in `engine.rs`:

```rust
let wet = self.reverb.wet();
let mut dry = [0.0; 2];
let mut send = 0.0;
for channel in &mut self.channels {
    let volume = channel.volume.next_value();
    let sample = channel.effects.process(channel.instrument.next_sample()) * volume;
    channel.peak = channel.peak.max(sample.abs());
    let amount = channel.reverb_send.next_value();
    let gains = lineup_core::Channel::pan_gains(channel.pan.next_value());
    let kept = sample * (1.0 - wet * amount);
    dry[0] += kept * gains[0];
    dry[1] += kept * gains[1];
    send += sample * amount;
}
// ...
let click = self.click.next_sample();
let stereo = self
    .reverb
    .next(dry, send, wet)
    .map(|sample| sample + click);
let volume = self.master_volume.next_value();
let stereo = stereo.map(|sample| sample * volume);
master_peak = [0, 1].map(|side| master_peak[side].max(stereo[side].abs()));
// `tanh` gently rounds off peaks above 1.0 instead of clipping them hard.
let stereo = stereo.map(f32::tanh);
```

In order:

1. The instrument makes a sample, mono.
2. The channel's **effects** shape it: equalizer, compressor, delay.
3. Its **volume** scales it, and its **peak** is kept for the meter.
4. Its **pan** splits it between the two speakers, and its **send** puts
   part of it into the reverb.
5. The **reverb** adds the room to the sum of all channels.
6. The metronome's click is added after the reverb: the room is the
   music's, and a click with a tail is harder to follow.
7. The **master volume** scales the whole, its peaks go to the master
   meters, and `tanh` rounds off anything above full scale
   ([Mixing a frame](../program/threads.md#mixing-a-frame)).

Volume, pan, send and the reverb's wet level are all `Smoothed`: moved
in the window, they glide to the new value over a few milliseconds
instead of jumping, since a jump in the middle of a wave is heard as a
click. The effects come before the volume, as on a mixing desk's channel
strip, so a compressor hears the instrument as it plays, whatever the
fader says.

## Pan: two speakers, one channel

Pan is a number from −1 (all left) to 1 (all right), with 0 in the
middle. It becomes a gain for each speaker:

```rust
/// How loud it is in the left and the right speaker for its pan: both
/// full in the middle, and the far side fading out as it moves away.
pub fn pan_gains(pan: f32) -> [f32; 2] {
    [(1.0 - pan).min(1.0), (1.0 + pan).min(1.0)]
}
```

```text
pan     −1.0  −0.5   0.0   0.5   1.0
left     1.0   1.0   1.0   0.5   0.0
right    0.0   0.5   1.0   1.0   1.0
```

In the middle both speakers play the channel at full volume, the way a
mono sound plays on a stereo system. Moving a channel to one side turns
the **other** speaker down, like the balance knob of a stereo.

This is one of several **pan laws**. Mixing desks often use a
*constant-power* law instead, where the middle plays each speaker at
about 0.71 (−3 dB), so a channel sounds equally loud wherever it is
placed. With the law used here, a channel moved hard left sounds a
little quieter than in the middle, because only one speaker plays it.
The reason for keeping it is that a song where nothing is panned plays
every channel at exactly its volume on both sides.

The slider writes its value as `L30`, `C` or `R100`, and reads what you
type the same way: `pan_value` accepts `L30`, `R100`, `C`, a plain
number from −100 to 100, and the Spanish `I30` (*izquierda*) and `D30`
(*derecha*), which is what the Spanish interface shows. A double-click
puts it back in the middle. A slider only senses drags, so the
double-click is read from the pointer and checked against the slider's
rectangle.

## The reverb: a room made of delays

A reverb imitates a room. A sound bounces off the walls, the floor and
the ceiling and comes back, later and softer, thousands of times, until
the echoes melt into a tail that dies away over a second or two. The
mixer's reverb is a **Schroeder reverb**, the oldest digital design
(Manfred Schroeder, 1962), built from two kinds of delay.

A **comb filter** is a delay whose output is fed back into its input,
a little softer each time: a sound comes back every 31 ms, say, quieter
on each lap. One comb sounds like a metallic flutter, but four in
parallel, with lengths that never line up, give a dense pattern of
echoes. An **all-pass filter** is a delay that smears each echo into a
cluster without changing the overall tone; three in a row turn the
combs' echoes into a smooth wash.

```rust
fn new(rate: u32, spread: usize) -> Self {
    let length =
        |samples| ((samples + spread) as f32 * rate as f32 / 48_000.0).round() as usize;
    Self {
        combs: [1493, 1601, 1747, 1867].map(|samples| Comb {
            delay: Delay::new(length(samples)),
            feedback: 0.0,
            filtered: 0.0,
        }),
        diffusers: [241, 503, 719].map(|samples| Delay::new(length(samples))),
    }
}
```

The lengths are counted in samples at 48 kHz (1493 samples is 31 ms)
and scaled to the actual rate. They are all prime numbers, so no two
share a factor and their echoes rarely fall on the same instant, which
would sound like a ringing note. Each side of the stereo room is a
`Side` with its own combs and all-passes; the right one is made with
`spread` 37, so its delays are 37 samples longer and the two sides
differ, which the ear hears as width.

`[...].map(...)` on an array gives an array of the same length, so
`combs` is a `[Comb; 4]` built in one expression, with no `Vec`.

### How long the room rings

**Decay** is the time the tail takes to fall by 60 dB, to a thousandth
of its level: the usual measure of a room, called RT60. Each comb's
feedback is worked out from it:

```rust
fn configure(&mut self, rate: u32, decay: f32) {
    for comb in &mut self.combs {
        comb.feedback =
            10.0_f32.powf(-3.0 * comb.delay.samples.len() as f32 / (rate as f32 * decay));
    }
}
```

In `decay` seconds the sound goes round a comb `rate × decay / length`
times, and is multiplied by `feedback` each time. For the total to be
10⁻³ (−60 dB), each lap multiplies by 10^(−3 × length / (rate × decay)).
Longer combs go round fewer times, so they get a stronger feedback, and
all four die away together.

**Damping** imitates the air and the soft walls of a real room, which
swallow high frequencies sooner than low ones. Inside each comb, the
echo goes through a one-pole low-pass filter on its way back in:

```rust
fn next(&mut self, input: f32, damping: f32) -> f32 {
    let out = self.delay.read();
    self.filtered = flushed(out * (1.0 - damping) + self.filtered * damping);
    self.delay.write(input + self.filtered * self.feedback);
    out
}
```

`filtered` moves only part of the way towards each new sample, which
smooths out fast wiggles: the highs. Each lap smooths them a little
more, so the tail gets darker as it fades, as in a hall.

**Pre-delay** is the gap before the room answers, up to 200 ms: in a big
hall the first reflection arrives later than in a small room. It is one
more ring buffer in front of the combs, a fifth of a second long.

### Never too small, never in the way

`flushed` turns any value under 10⁻²⁰ into exactly 0. A fading tail
gets smaller and smaller without ever reaching zero, and numbers that
tiny (**denormals**) make many processors dozens of times slower. A
reverb left to ring out in silence could then overrun the audio
thread's time.

All delay memory is reserved in `Reverb::new`, before the stream
starts; `configure` only changes numbers, so moving a slider never
allocates on the audio thread. A setting out of range is ignored rather
than applied. Turned off, the wet level fades to zero and then
`Reverb::next` returns the dry sound at once, doing no work. Turned on
again, it starts from silence (`clear`), not from the tail it had when
it was switched off.

### The send: how much of each channel is in the room

Each channel has a **send**, from 0 to 1: how much of it goes into the
room. The sum of all sends is the reverb's single input, so twenty
channels cost one reverb, not twenty, as with a send on a real desk.
The reverb returns only the room, which `next` adds to the dry sound:

```rust
[
    dry[0] + left.next(delayed, damping) * wet,
    dry[1] + right.next(delayed, damping) * wet,
]
```

The line to look at in the mix is `kept = sample * (1.0 - wet *
amount)`: what goes into the room is taken out of the dry sound, in
proportion to the **Wet / dry** setting. Check the two ends:

- **Send 1** (all of it, as on every new channel): `kept = sample × (1 −
  wet)`, and the whole sample goes to the room. That is a classic wet /
  dry mix of the whole song.
- **Send 0**: `kept = sample`, and nothing goes to the room. The channel
  plays dry and close, as with the reverb off, while the rest of the
  band stays in the room.

The room is mono going in and stereo coming out, so a channel panned
hard left still has its reverb on both sides, as a guitarist on the
left of a stage fills the whole hall.

## Each channel's effects

A real mixing desk has, on every channel strip, a few effects that shape
one instrument before it joins the others. Each mixer line has three,
in the order most desks have them: an **equalizer**, a **compressor**
and a **delay**.

![The mixer of a Motown song with horns. The Horns line's Effects button reads "EQ, delay", and its menu is open: Equalizer with Low at -4.0 dB, Mid and High at 0 dB; Compressor with Threshold -18.0 dB, Ratio 1.0:1 and Gain 0.0 dB; Delay with Time dotted 1/8, Feedback 35% and Mix 30%; and a No effects button.](../images/step-9-effects.png)

### The equalizer: turning frequencies up or down

Every sound is a mix of frequencies: a note's fundamental, and its
overtones above it. An **equalizer** (EQ) turns ranges of them up or
down. It adds no notes; it changes the colour of the ones there. The
mixer's EQ has three bands, each from −12 dB to +12 dB:

| Band | Where | Shape | What lives there |
|---|---|---|---|
| **Low** | below 200 Hz | a shelf | the weight of a bass, a kick drum, a piano's left hand |
| **Mid** | around 1 kHz | a bell, about an octave and a half wide | the body of most instruments, and of voices |
| **High** | above 4 kHz | a shelf | the air and sparkle: cymbals, a breathy voice, a bow on a string |

A **shelf** turns everything past its frequency up or down by the same
amount, like a step. A **bell** turns its frequency up or down most,
and less and less on each side. Decibels measure a ratio: +6 dB makes a
band's waves about twice as tall, −6 dB about half, 0 dB leaves it as it
is.

Turning things **down** is the commonest use. When two instruments play
in the same range, they mask each other: a piano's lows and the bass
turn into one muddy sound. Taking the lows out of the piano leaves room
for the bass, and once the bass plays, the piano sounds no thinner.

On the audio thread each band is a **biquad**, a second-order filter,
the building block of most equalizers. Each sample out is a mix of the
last three samples in and the last two out:

```rust
let output = b0 * input + b1 * self.inputs[0] + b2 * self.inputs[1]
    - a1 * self.outputs[0]
    - a2 * self.outputs[1];
```

The five numbers `b0`, `b1`, `b2`, `a1` and `a2` decide what the filter
does. `Biquad::set` works them out for a shelf or a bell, a frequency
and a gain, with the formulas of Robert Bristow-Johnson's *Audio EQ
Cookbook*, which almost every audio program uses: a slope of 1 for the
shelves, a Q of 1 for the bell. The three filters run one after the
other, low, mid, high, and their numbers are only worked out again when
the settings change, never for each sample. `set` keeps the samples the
filter remembers, so moving a band while it plays does not click. The
same 10⁻²⁰ floor as in the reverb keeps denormals out.

### The compressor: evening out how loud a part is

Some parts come out unevenly: a bass line where some notes jump out and
others get lost. A **compressor** turns a sound down when it gets loud:

- Under its **threshold** it does nothing.
- Over it, it turns the sound down by an amount its **ratio** sets. At
  4:1, a sound 8 dB over the threshold comes out only 2 dB over: four
  decibels in for each one out. At 1:1 it turns nothing down, so the
  compressor is off. At 20:1 almost nothing gets over the threshold.
- Its **gain** (the "make-up gain") turns the whole part up afterwards,
  since the loud notes came down. The part is as loud as it was, but
  steadier.

That rule is plain arithmetic, so it lives in `lineup-core` where a test
can check it without any sound:

```rust
pub fn reduction(&self, level: f32) -> f32 {
    let over = level - self.threshold;
    if over <= 0.0 || self.ratio <= 1.0 {
        0.0
    } else {
        over * (1.0 - 1.0 / self.ratio)
    }
}
```

At 4:1, 8 dB over the threshold is turned down by 8 × (1 − ¼) = 6 dB,
and comes out 2 dB over.

A compressor does not turn down each wave. It follows how loud the sound
has been over the last few milliseconds: it turns down fast once a sound
goes over the threshold (its **attack**, 10 ms) and back up more slowly
once it is under again (its **release**, 150 ms). If it followed every
wave, it would bend the wave's shape and add a buzz. The `Follower`
does this:

```rust
let peak = sample.abs();
let speed = if peak > self.level {
    self.attack
} else {
    self.release
};
self.level += (peak - self.level) * speed;
let level = (20.0 * self.level.log10()).max(FLOOR);
let decibels = compressor.gain - compressor.reduction(level);
10f32.powf(decibels / 20.0)
```

`attack` and `release` are worked out once from their times, as
`1 − e^(−1 / (seconds × sample rate))`: with that number, the level gets
about two thirds of the way to a new loudness in the given time.
`20 × log10` turns the level into decibels, `reduction` says how much to
turn down, and `10^(dB / 20)` turns the answer back into a number to
multiply the sample by. `.max(FLOOR)` keeps silence, whose logarithm is
minus infinity, at −100 dB.

### The delay: echoes in time with the song

A **delay** plays the sound again a little later, and again, each echo
quieter than the one before. Its **time** is a note value of the song,
counted in steps (sixteenths):

```rust
pub const TIMES: [(u8, &'static str); 6] = [
    (1, "1/16"),
    (2, "1/8"),
    (3, "dotted 1/8"),
    (4, "1/4"),
    (6, "dotted 1/4"),
    (8, "1/2"),
];
```

So the echoes fall on the song's rhythm, and follow the tempo when it
changes. The **dotted eighth** is the favourite: its echoes fall three
sixteenths after each note, between the beats, and a guitar or a voice
playing eighth notes sounds as if it played sixteenths.

```rust
pub fn seconds(&self, tempo: Tempo) -> f32 {
    let step = tempo.seconds_per_beat() / crate::STEPS_PER_BEAT as f32;
    (f32::from(self.steps) * step).min(Delay::MOST_SECONDS)
}
```

A dotted eighth at 120 BPM is 3 × 0.125 = 0.375 s. An echo comes at most
2 seconds after the sound: a half note at 40 BPM would be 3 seconds, so
at such slow tempos the longest times stop at 2.

- **Feedback** says how much of each echo comes back as the next one.
  At 50%, each echo is half as loud as the one before; at 0% there is
  only one. It stops at 90%, so the echoes always die away.
- **Mix** says how loud the echoes are next to the sound itself. 0%
  turns the delay off; 100% makes the first echo as loud as the note.

The delay remembers the sound it plays again in a **ring buffer**: a
fixed array written round and round, where the sample `delay_frames`
back is always behind the one being written:

```rust
let frames = line.len();
let read = (self.written + frames - self.delay_frames) % frames;
let echo = line.0[read];
line.0[self.written] = sample + echo * self.settings.delay.feedback;
self.written = (self.written + 1) % frames;
sample += echo * mix;
```

What goes into the line is the sound plus part of the echo; that part
comes back once more `delay_frames` later, which is the feedback. What
comes out is the sound plus the echo at the mix. Adding `frames` before
the `%` keeps the subtraction from going below zero, which a `usize`
cannot hold.

### Memory made on the other side

Two seconds of sound is 96,000 samples at 48 kHz, a big allocation, and
the audio thread never allocates. So the window's thread makes the
`DelayLine` and sends it with the settings:

```rust
SetEffects {
    channel: usize,
    effects: lineup_core::Effects,
    line: Option<DelayLine>,
},
```

It sends a line every time it sends settings with the delay on, because
only the audio thread knows whether that channel already has one.
`ChannelEffects::set` keeps the first line it gets and returns any
other, which goes back on the trash queue to be freed on the window's
side:

```rust
let spare = match self.channels.get_mut(channel) {
    Some(state) => state.effects.set(effects, line, tempo),
    None => line,
};
if let Some(spare) = spare {
    self.throw_away(Garbage::DelayLine(spare));
}
```

A channel keeps its line once it has one. When its delay is turned off
and on again, `set` fills the line with silence first, or the echoes of
the last time would come back. When the tempo changes, the engine calls
`set_tempo` on every channel's effects, so the echoes move with it.

`process` first checks whether any effect is on. If none is, it returns
the sample untouched: a channel without effects sounds exactly as it
would without the code, and costs nothing.

## Level meters: from the audio thread to the screen

A meter shows how loud a channel is **now**. The audio thread knows,
since it makes the sound; the window's thread draws the meter. They run
at different speeds (the audio thread fills a block of a few hundred
samples many times a second, the window draws about 60 frames a
second), and the audio thread must never wait for the window, so a
`Mutex` is out.

### Peaks in atomics

The audio thread keeps the loudest sample of each channel over a block
(`channel.peak` above). At the end of the block it raises a shared peak
to it, if it is louder. Once a frame, the meter takes the shared peak and
sets it back to silence. So no peak is missed, however blocks and frames
line up, and neither side waits:

```rust
/// The peaks since the UI last took them, as the bits of `f32`s.
///
/// For numbers that are not negative, comparing their bits compares the
/// numbers, so `fetch_max` on the bits keeps the louder peak.
pub struct Levels {
    channels: [AtomicU32; MAX_CHANNELS],
    master: [AtomicU32; 2],
}
```

```rust
fn raise(slot: &AtomicU32, peak: f32) {
    // `peak > 0.0` is false for NaN too, which must not get in.
    if peak > 0.0 {
        slot.fetch_max(peak.to_bits(), Ordering::Relaxed);
    }
}

fn take(slot: &AtomicU32) -> f32 {
    f32::from_bits(slot.swap(0, Ordering::Relaxed))
}
```

Rust's standard library has no `AtomicF32`. But an `f32` is 32 bits,
and `to_bits` and `from_bits` turn it into a `u32` and back without
changing a bit. Better still, for numbers that are **not negative**, the
order of the bits as unsigned integers is the order of the numbers: the
exponent sits in the high bits and the mantissa below it. So
`fetch_max` on the bits, an atomic "keep the larger", keeps the louder
peak. Peaks are absolute values, never negative, and the `if` keeps NaN
out (its bits would compare as larger than any number).

`swap(0, …)` reads the value and puts 0 (the bits of 0.0) in its place
in one step, so a peak raised between a read and a reset can't be lost.
`Ordering::Relaxed` is enough because each peak stands alone: nothing
else in memory has to be seen in a certain order along with it.

`std::array::from_fn(|_| AtomicU32::new(0))` builds the arrays, since an
`AtomicU32` is not `Copy` and `[AtomicU32::new(0); N]` would not
compile.

### Decibels, and a meter that falls slowly

The ear hears loudness by ratios, not by differences, so the meter is
drawn in **decibels**, a scale of ratios:

```text
decibels = 20 × log10(level)        1.0 → 0 dB (full scale)
                                    0.5 → −6 dB
                                   0.25 → −12 dB
                                  0.004 → −48 dB (the meter's floor)
```

Each 6 dB down halves the level, wherever on the scale. A meter is lit
from −48 dB at the left to 0 dB at the right: green, amber from −6 dB,
red at full scale and above. The master's meters measure after the
master volume and before `tanh`, so red on the master means the
rounding-off is at work. Hovering a meter says its level in decibels.

A meter that followed every peak exactly would flicker too fast to
read. So it **jumps up** to a new peak at once and **falls** 24 dB a
second:

```rust
pub fn update(&mut self, levels: &Levels, channels: usize, seconds: f32) {
    let fall = gain(-FALL_DB_PER_SECOND * seconds);
    self.channels.resize(channels, 0.0);
    for (index, shown) in self.channels.iter_mut().enumerate() {
        *shown = levels.take_channel(index).max(*shown * fall);
    }
    for (shown, peak) in self.master.iter_mut().zip(levels.take_master()) {
        *shown = peak.max(*shown * fall);
    }
}
```

`seconds` is the time since the last frame. Falling 24 dB a second means
multiplying by 10^(−24 × seconds / 20) each frame, whatever the frame
rate. Half a second after a peak, the meter shows it 12 dB down, at
about a quarter.

egui only draws a frame when something happens. While a meter is still
falling, the mixer asks for the next frame at once (`request_repaint`);
when every meter is dark, it looks again every tenth of a second
(`request_repaint_after`), which is enough to catch the notes of a MIDI
keyboard played while the song is stopped.

## Score order: the drums together

The channel rack keeps channels in the order they were added. A style
that brings in a rim shot, a bass, a cowbell and a conga adds them at
the end, so a list in rack order scatters the drums around the piano and
the bass. A conductor's score never does that: every instrument has its
place, the same in every score, so a reader finds the flutes at the top
and the double basses at the bottom without searching. The mixer lists
channels in that order, in sections with a heading each:

| Section | Instruments, top to bottom |
|---|---|
| Woodwinds | piccolo, flutes, oboes, English horn, clarinets, saxophones, bassoons |
| Brass | horns, trumpets, trombones, tuba |
| Percussion | timpani; glockenspiel, xylophone, vibraphone, marimba, tubular bells; the rest |
| Drums | hand percussion, then the drum kit |
| Harp and guitars | harp, guitars, banjo, koto, bass guitar |
| Keyboards | celesta, piano, harpsichord, organ, accordion and bandoneón |
| Synths | synth leads, pads and effects; a Lineup Osc with no instrument |
| Voices | a solo voice, then the choir |
| Strings | string sections, violins, violas, cellos, double basses |

The horns come above the trumpets although they sound lower, because
they blend with the woodwinds and were long written next to them. The
strings come last, so the double basses, the lowest part of the
orchestra, end the page; a choir goes just above them, as in a requiem
or an oratorio.

The drums follow the **drum staff**, where each drum has its own line or
space: the cymbals on and above the top line (the crash highest, then
the hi-hat, then the ride), the high tom in the top space, the snare in
the third space, the floor tom below it and the kick in the bottom
space. So the **Drums** section reads Crash, Hat, Open hat, Ride, High
tom, Snare, Rim (a cross-stick, written where the snare is), Low tom,
Kick. Hand percussion (cowbell, shaker, clap, conga) usually has staves
of its own above the kit, so it comes first.

Only the mixer changes its order. The channel rack, the piano roll and
the saved file keep the channels as they were added, and nothing sounds
different.

### Where a channel goes

`score_order.rs` is in `lineup-core`, with no UI code, so it is tested
with plain `cargo test`. A `Section` is an enum in score order:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Section {
    Woodwinds,
    Brass,
    Percussion,
    Drums,
    HarpAndGuitars,
    Keyboards,
    Synths,
    Voices,
    Strings,
}
```

Deriving `PartialOrd` and `Ord` on an enum orders its variants as they
are written: `Section::Woodwinds < Section::Brass`. The order of the
score is the order of the source code.

The mixer does not look at a channel's name to place it, but at its
instrument, the General MIDI program ([SoundFonts and General
MIDI](soundfonts.md)). A channel called "Lead" that plays a
flute goes with the woodwinds. The program numbers mostly come in
families already, but not always the way a score wants: General MIDI
puts the timpani with the strings and the celesta with the mallets, and
the score puts the timpani first in the percussion and the celesta with
the keyboards. So each section lists its programs, top of the score
first, and `place` finds a channel's section and its rank in it:

```rust
pub fn place(channel: &Channel) -> (Section, usize) {
    match &channel.kind {
        ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
            let rank = DRUMS.iter().position(|d| d == drum).unwrap_or(DRUMS.len());
            (Section::Drums, rank)
        }
        // A sound file usually takes a drum's row in the rack.
        ChannelKind::Sampler(SampleSource::File(_)) => (Section::Drums, DRUMS.len() + 1),
        ChannelKind::MidiOut {
            channel: DRUM_CHANNEL,
        } => (Section::Drums, DRUMS.len()),
        ChannelKind::Synth | ChannelKind::MidiOut { .. } => match channel.program {
            Some(program) => program_place(program),
            // The Lineup Osc, or an outside synth with no instrument named.
            None => (Section::Synths, usize::MAX),
        },
    }
}
```

`ChannelKind::MidiOut { channel: DRUM_CHANNEL }` matches only MIDI Out
channels whose field equals the constant `DRUM_CHANNEL` (9, shown as
10): a `const` can stand in a pattern like a literal number. The next
arm, `MidiOut { .. }`, catches every other MIDI channel; `match` tries
the arms from the top, so the drum channel never gets that far.

```rust
fn program_place(program: u8) -> (Section, usize) {
    Section::ALL
        .into_iter()
        .find_map(|section| {
            let rank = section.programs().iter().position(|&p| p == program)?;
            Some((section, rank))
        })
        .unwrap_or((Section::Synths, usize::MAX))
}
```

`find_map` runs the closure on each section and stops at the first one
that returns `Some`. Inside, the `?` after `position` returns `None` from
the closure when the program is not in that section, which tells
`find_map` to try the next. A test checks that each of the 128 programs
is in exactly one list, and another that each built-in drum has one
place in the kit.

### Sorting and grouping

```rust
pub fn score_order(channels: &[Channel]) -> Vec<(Section, Vec<usize>)> {
    let mut order: Vec<usize> = (0..channels.len()).collect();
    order.sort_by_key(|&index| place(&channels[index]));
    let mut sections: Vec<(Section, Vec<usize>)> = Vec::new();
    for index in order {
        let section = place(&channels[index]).0;
        match sections.last_mut() {
            Some((last, indices)) if *last == section => indices.push(index),
            _ => sections.push((section, vec![index])),
        }
    }
    sections
}
```

It sorts the channels' **numbers**, not the channels: each row of the
mixer must know which channel it is, to change that channel's volume and
to solo the right one. The key is the pair `(Section, usize)`, and
tuples compare field by field, so the section decides first and the rank
breaks the tie. `sort_by_key` is a *stable* sort: channels with the same
key, such as "Lead", "Voice 2" and "Voice 3" all playing violins, keep
the rack's order.

The loop then walks the sorted list and starts a new group whenever the
section changes. `last_mut` gives a mutable reference to the last group,
if there is one; the `if` after the pattern (a *match guard*) only takes
the first arm when that group is the same section.

## The window

`mixer_window` draws from copies: `mix` is a copy of the project's
`Mixer`, and `levels` a copy of its channels. The sliders change the
copies, and when the window is done the app compares them with the
project and makes one `edit` for whatever changed, so a drag is one step
to undo ([Undo and redo](../program/undo.md)). The master volume goes
through `set_master_volume` instead, which reopens the last step if it
was also the master volume, so nudging it several times is one step
([Several clicks, one
step](../program/undo.md#several-clicks-one-step)).

```rust
for (section, indices) in score_order(&levels) {
    ui.label(egui::RichText::new(tr(section.name())).color(theme::ACCENT));
    ui.end_row();
    for index in indices {
        let channel = &mut levels[index];
        // name, instrument, meter, volume, pan, send, effects, mute ...
        if ui
            .selectable_label(mix.solo == Some(index), tr("S"))
            // ...
            .clicked()
        {
            mix.solo = if mix.solo == Some(index) {
                None
            } else {
                Some(index)
            };
        }
        ui.end_row();
    }
}
```

The **Instrument** column is a menu for a synth or MIDI Out channel,
with the same choices as the rack's: the rack and the mixer both call
`channel_rack::instrument_choices`. A built-in drum has no instrument to
choose, but its menu can send it over MIDI ([MIDI](midi.md));
a sample file's cell is plain text. The menu is drawn inside closures
that already borrow parts of the app, so it cannot change the project
while it is drawn. It pushes what you picked into a list, and the app
applies it when the window is done:

```rust
for action in picked {
    self.apply(action);
}
```

It is the pattern the channel rack and the chord sheet use too: the UI
code says **what** you did, as a value (`RackAction::SetProgram {
channel, program }`), and one place in the app does it.

The **Effects** button says which effects are on ("EQ, delay"), or
"none", and opens a menu with all three. That menu is made with
`PopupCloseBehavior::CloseOnClickOutside`, so it stays open while you
drag its sliders, until you click elsewhere. The equalizer's sliders go
back to 0 dB on a double-click, as the pan goes back to the middle.

The window fits its columns, and is fitted again when the channels or
their instruments change, since a shorter instrument name can make a
column narrower ([Windows the size of what they
show](../program/egui.md#windows-the-size-of-what-they-show)).

## Testing it

- `reverb.rs`: `bypass_preserves_samples_exactly` (off, the samples
  come out bit for bit); `impulse_has_a_stereo_tail_that_decays_and_can_be_cleared`
  at 22.05, 48 and 96 kHz (silence during the pre-delay, a tail that
  differs left and right, less than a thousandth of the first half
  second's energy after one second, and silence after `clear`);
  `a_tail_ends_in_zeros_not_denormals`.
- `effects.rs` in `lineup-audio` plays sine waves through the effects
  and measures what comes out: Low at +6 dB turns 50 Hz up by 6 dB and
  leaves 5 kHz alone; a compressor at −20 dB and 4:1 turns a full-scale
  wave, 20 dB over, down by 15 dB, and lets a quiet one through; an
  eighth-note delay at 120 BPM with 50% feedback echoes a click after
  0.25 s at full level, then half, then a quarter; a line is kept once
  and a second one given back.
- `effects.rs` in `lineup-core`: the reduction arithmetic, the delay's
  time at 120 BPM and its 2-second cap, settings out of range, the
  button's summary ("EQ, comp"), and that plain effects save as `()`.
- `render.rs` plays a click through the whole engine:
  `a_panned_channel_plays_more_on_its_side` at five pans against the
  table above; `a_channel_sent_nowhere_plays_dry_in_a_room` (a send of 0
  is as loud as with the reverb off and leaves no tail);
  `a_channel_with_a_delay_echoes_in_time_with_the_song`;
  `the_meters_see_each_channel_and_the_master` (a click at volume 0.5,
  panned left, master at 0.8: the channel's meter reads 0.25, the
  master's left 0.2 and its right 0).
- `levels.rs` and `meters.rs`: the loudest peak is kept until taken and
  NaN never gets in; a meter jumps up, falls 12 dB in half a second, and
  goes dark below the floor.
- `score_order.rs`: every program and every drum has exactly one place;
  drums added after the band join the other drums; an orchestra reads
  from the flutes down to the basses; the MIDI drum channel and sound
  files count as drums; channels in the same place keep the rack's
  order.
- In the app: `the_mixer_pans_a_channel_and_sets_its_reverb_send`,
  `the_mixer_sets_a_channels_effects` (opens the menu, picks "dotted
  1/4", then "No effects"), `a_click_on_an_instrument_in_the_mixer_changes_it`
  (and one Undo brings the piano back),
  `the_mixer_keeps_the_drums_together_and_each_one_still_solos_alone`,
  and two exports: `an_exported_song_keeps_each_channel_where_the_mixer_put_it`
  and `an_exported_song_plays_each_channels_effects`.
- In `file.rs`: `pan_and_reverb_sends_need_format_twenty_six` and
  `effects_need_format_twenty_eight`, each with a hand-edited file that
  must be refused.
