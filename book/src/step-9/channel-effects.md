# EQ, compression and delay on each channel

Files: [`crates/lineup-core/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/effects.rs),
[`crates/lineup-audio/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/effects.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)
and [`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs)

The [mixer](mixer.md) puts the whole band in one room with its reverb,
and places each channel between the speakers with its pan. But on a
real mixing desk every channel strip also has its own **effects**,
which shape one instrument before it joins the others. Now each
channel in the mixer has three, in the order most desks have them: an
**equalizer**, a **compressor** and a **delay**.

![The mixer of a Motown song with horns. The Horns line's Effects button reads "EQ, delay", and its menu is open: Equalizer with Low at -4.0 dB, Mid and High at 0 dB; Compressor with Threshold -18.0 dB, Ratio 1.0:1 and Gain 0.0 dB; Delay with Time dotted 1/8, Feedback 35% and Mix 30%; and a No effects button.](../images/step-9-effects.png)

## Trying it

1. Open a demo from **File > Demos**, press **F9** for the mixer and
   **Space** to play.
2. The **Effects** column says "none" on every line. Click it on the
   Piano's line: a menu opens with the three effects. It stays open
   while you change them, until you click somewhere else.
3. Drag the **Low** slider of the **Equalizer** down to −6 dB. The
   piano gets thinner, and the bass is easier to hear under it. The
   button now reads "EQ". **Flat** puts every band back to 0 dB, and
   a double-click on a slider puts that one back.
4. On the Bass line, set the **Compressor**'s **Ratio** to 4:1 and its
   **Threshold** to −24 dB. The loud notes come down. Raise its
   **Gain** a few decibels to bring the whole part back up.
5. On the Lead line, set the **Delay**'s **Mix** to 30% and pick
   **dotted 1/8**. Each note comes back three sixteenths later, and
   again, softer each time. Change the song's tempo: the echoes keep
   time with it.

**No effects** at the bottom of the menu turns them all off for that
channel, and **Reset mix** turns them off on every channel. Every
change is one step to undo. The effects play in an exported WAV too,
and the song saves them.

## The equalizer: turning frequencies up or down

Every sound is a mix of frequencies: a low note's fundamental, and its
overtones above it. An **equalizer** (EQ) turns ranges of them up or
down. It does not add notes. It changes the colour of the ones there.
The mixer's EQ has three bands:

| Band | Where | Shape | What lives there |
|---|---|---|---|
| **Low** | below 200 Hz | a shelf | the weight of a bass, a kick drum, a piano's left hand |
| **Mid** | around 1 kHz | a bell, about an octave and a half wide | the body of most instruments, and of voices |
| **High** | above 4 kHz | a shelf | the air and sparkle: cymbals, a breathy voice, a bow on a string |

A **shelf** turns everything past its frequency up or down by the same
amount, like a step. A **bell** turns its frequency up or down most,
and less and less on each side of it.

Each band goes from −12 dB to +12 dB. Decibels measure a ratio of
loudness: +6 dB makes a band's waves about twice as tall, −6 dB about
half, 0 dB leaves it as it is.

Turning things **down** is the commonest use. When two instruments play
in the same range, they mask each other: a piano's lows and the bass
turn into one muddy sound. Taking the lows out of the piano leaves room
for the bass, and the piano sounds no thinner once the bass plays.

## The compressor: evening out how loud a part is

Some parts come out unevenly: a bass line where some notes jump out and
others get lost. A **compressor** turns a sound down when it gets loud:

- Under its **threshold** it does nothing.
- Over it, it turns the sound down by an amount its **ratio** sets. At
  4:1, a sound 8 dB over the threshold comes out only 2 dB over it: 4
  decibels in for each decibel out. At 1:1 it turns nothing down, so
  the compressor is off. At 20:1 almost nothing gets over the threshold.
- Its **gain** (the "make-up gain") turns the whole part up afterwards,
  since the loud notes came down. The part is now as loud as it was,
  but steadier.

A compressor does not turn down each wave. It follows how loud the
sound has been over the last few milliseconds. It turns down fast once
a sound goes over the threshold (its **attack**, 10 ms) and back up
more slowly once the sound is under it again (its **release**, 150 ms).
If it followed every wave, it would bend the wave's shape and add a
buzz.

## The delay: echoes in time with the song

A **delay** plays the sound again a little later, and again, each echo
quieter than the one before. Its **time** is a note value of the song:
a sixteenth, an eighth, a dotted eighth, a quarter, a dotted quarter or
a half note. The echoes fall on the song's rhythm, and follow the tempo
when it changes. The **dotted eighth** is the favourite: its echoes
fall between the beats, three sixteenths after each note. A guitar or
a voice playing eighth notes sounds as if it played sixteenths.

- **Feedback** says how much of each echo comes back as the next one.
  At 50%, each echo is half as loud as the one before. At 0% there is
  one echo only. It stops at 90%, so the echoes always die away.
- **Mix** says how loud the echoes are next to the sound itself. 0%
  turns the delay off, 100% makes the first echo as loud as the note.

An echo comes at most 2 seconds after the sound. A half note at 40
BPM would be 3 seconds, so at such a slow tempo the longest times
stop at 2.

## In the code

### The settings

`lineup-core` keeps the settings, in `Channel::effects`:

```rust
pub struct Effects {
    #[serde(default, skip_serializing_if = "Equalizer::is_flat")]
    pub eq: Equalizer,
    #[serde(default, skip_serializing_if = "Compressor::is_default")]
    pub compressor: Compressor,
    #[serde(default, skip_serializing_if = "Delay::is_default")]
    pub delay: Delay,
}
```

`skip_serializing_if` leaves an effect out of the file while it is as on
a new channel, and `default` fills it back in when it is missing. A song
that uses no effects saves the same text as before, and an older file
opens with every effect off. A song where any channel uses one is saved
in format 28.

The compressor's sum is in the core too, so it can be tested without
sound:

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
and comes out 2 dB over. So does the delay's time: `Delay::seconds`
multiplies its steps by the length of a step at the tempo, and stops at
2 seconds. A dotted eighth (three steps) at 120 BPM is 3 × 0.125 =
0.375 s.

### Each sample through the three

On the audio thread, each channel has a `ChannelEffects`. The engine
sends every sample the instrument makes through it, before the
channel's volume, pan and reverb send:

```rust
let sample = channel.effects.process(channel.instrument.next_sample()) * volume;
```

`process` first checks whether any effect is on. If none is, it gives
the sample back as it came, so a channel without effects sounds exactly
as before and costs nothing.

### The equalizer: three biquads

Each band is a **biquad**, a second-order filter, the building block
of most equalizers. Each sample out is a mix of the last three samples
in and the last two out:

```rust
let output = b0 * input + b1 * self.inputs[0] + b2 * self.inputs[1]
    - a1 * self.outputs[0]
    - a2 * self.outputs[1];
```

The five numbers `b0`, `b1`, `b2`, `a1` and `a2` decide what the filter
does. `Biquad::set` works them out for a shelf or a bell, a frequency
and a gain, with the formulas of Robert Bristow-Johnson's "Audio EQ
Cookbook", which almost every audio program uses. The three filters run
one after the other: low, mid, high. They are only worked out again
when the settings change, never for each sample.

One more line in `process` drops any output smaller than 10⁻²⁰. A
filter's echo of a sound that has stopped gets smaller and smaller
without ever reaching zero, and numbers that tiny (**denormals**) can
make a processor many times slower.

### The compressor: a level that follows the peaks

`Follower` keeps how loud the channel has been. For each sample, the
level moves part of the way towards the sample's size: a big part when
the sample is louder (the attack), a small part when it is quieter (the
release):

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

`attack` and `release` are worked out once, from the times:
`1 − e^(−1 / (seconds × sample rate))`. With that number, the level gets
about two thirds of the way to a new loudness in the given time.
`20 × log10` turns the level into decibels, `reduction` says how much
to turn down, and `10^(dB / 20)` turns the result back into a number
to multiply the sample by.

### The delay: a ring buffer

The delay needs to remember the sound it plays again: up to 2 seconds
of it, 96,000 samples at 48 kHz. That memory is a `DelayLine`, used as
a **ring buffer**: a fixed array written round and round, where the
sample `delay_frames` back is always behind the one being written:

```rust
let read = (self.written + frames - self.delay_frames) % frames;
let echo = line.0[read];
line.0[self.written] = sample + echo * self.settings.delay.feedback;
self.written = (self.written + 1) % frames;
sample += echo * mix;
```

What goes into the line is the sound plus part of the echo. That part
comes back once more `delay_frames` later: that is the feedback. What
comes out is the sound plus the echo at the mix. Adding `frames` before
the `%` keeps the subtraction from going below zero, which a `usize`
cannot hold.

### Making the line on the other thread

The audio thread must never allocate memory (see
[The audio thread, grown up](../step-2/realtime.md)), and a 96,000
sample line is a big allocation. So the UI thread makes it, and sends
it with the settings:

```rust
SetEffects {
    channel: usize,
    effects: lineup_core::Effects,
    line: Option<DelayLine>,
},
```

The UI sends a line every time it sends settings with the delay on,
because only the audio thread knows whether that channel already has
one. `ChannelEffects::set` keeps the first line it gets and returns any
other. The engine puts that one in the trash queue, and the UI thread
frees it:

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
and on again, `set` fills the line with silence first: otherwise the
echoes of the last time would come back.

When the tempo changes, the engine calls `set_tempo` on every channel's
effects, so the echoes move with it.

## How we know it is right

- **DSP tests** in `lineup-audio/src/effects.rs` play sine waves through
  the effects and measure what comes out:
  - The EQ turns 50 Hz up by 6 dB with Low at +6, 1 kHz down by 6 dB
    with Mid at −6, and 15 kHz up by 9 dB with High at +9, and each
    leaves the far end of the range as it was.
  - A compressor at −20 dB and 4:1 turns a full-scale wave, 20 dB over,
    down by 15 dB, and lets a quiet one through.
  - A delay of an eighth note at 120 BPM with 50% feedback echoes a
    click after 0.25 s at full mix, then at half, then at a quarter.
  - A line is kept once, the next one is given back, and a delay turned
    off and on keeps its line.
  - A channel without effects gives every sample back as it came.
- **Settings tests** in `lineup-core/src/effects.rs`: the reduction
  sum, the delay's time at 120 BPM and its 2-second cap, settings out
  of range, and the button's summary ("EQ, comp").
- **A render test** plays a click with a delay and finds its echo an
  eighth note later, at half its level, in the middle like the channel.
- **App tests** click the mixer's Effects button, check the menu has
  the three effects, pick "dotted 1/4" and "No effects", and export a
  song with every band turned down to check it comes out quieter.
- **A file test**: a song with effects saves in format 28 and comes
  back the same, an effect that is off is left out of the text, and a
  file edited by hand to a feedback of 150%, echoes that would never
  die away, is refused.

## Exercises

1. Put a delay of a dotted eighth with 30% mix on the Lead of a demo,
   and play it at 60 BPM and at 140 BPM. How many milliseconds are the
   echoes apart each time? Check with `Delay::seconds`.
2. Set a compressor at −30 dB and 10:1 on the drums. Then raise its
   Gain until the drums are as loud as before. What happened to the
   difference between the loud and the soft hits? Why do drums with a
   strong compressor sound "squashed"?
3. A compressor that follows every wave would not need a `Follower`.
   Write a test that sends a 50 Hz sine through a compressor with an
   attack of 0.0001 s, and measure how far the output's shape is from
   a sine. Explain why the real compressor waits 10 ms.
4. The bell's width is fixed at a Q of 1. Add a `width` to `Equalizer`
   and a slider for it, and use it in `Biquad::set`. What does the
   Cookbook say alpha is for a Q? Remember the file: a song that sets
   it needs a new format, 29.
5. Most desks let a compressor listen to one channel while it turns
   another down: a bass that ducks each time the kick drum hits
   ("side-chain compression"). Sketch what `ChannelEffects` would need
   for that. Which channel's sample would `Follower::gain` read?
