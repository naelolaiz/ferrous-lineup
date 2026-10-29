# The mixer

The **mixer** (**F9**, or **View > Mixer**) is where the parts of a
song are balanced: how loud each one is, where it sits between the two
speakers, how much of it is heard in a room, and how its sound is
shaped. Every change in it can be undone, and is saved with the song.

![The mixer of the El Choclo demo while it plays: the meters lit on every line, the reverb on, the Keys panned to L30 with EQ in its Effects column, the Lead panned to R30, the Bass with no reverb (0%), and the channels grouped as Drums, Harp and guitars, Keyboards and Strings](images/mixer.png)

## The lines

Each channel has a line, and the lines are grouped as a score groups
its instruments, from the woodwinds at the top down to the strings:
**Woodwinds**, **Brass**, **Percussion**, **Drums**, **Harp and
guitars**, **Keyboards**, **Synths**, **Voices** and **Strings**. The
order is only how the mixer shows them; the channel rack keeps its own.

| Column | What it does |
|---|---|
| **Instrument** | what the channel plays. Click it to choose another, from the same list as the channel rack (see [Sounds](sounds.md)). A drum says **Built-in drum**, a sound of your own **Sample**. |
| **Level** | a meter of how loud the channel is playing now. It turns amber in the loudest part, and red when it reaches the most a channel can play. |
| **Volume** | how loud the channel is, from 0 to 100%. |
| **Pan** | where it sits between the speakers: **C** in the middle, **L100** all left, **R100** all right. Drag it, or double-click the box and type, as in `L30`. Double-click the slider to put it back in the middle. |
| **Reverb** | how much of it goes into the room of the reverb (see below). At 0% it plays dry, close to you. |
| **Effects** | its own equalizer, compressor and delay (see below). It says **none**, or which are on: **EQ**, **comp**, **delay**. |
| **Mute** | silences it. |
| **Solo** | **S** plays this channel alone; click it again to hear them all. |

At the top, **Master** is the volume of the whole song, the same as the
**VOL** slider in the toolbar, with a meter for each speaker. **Reset
mix** goes back to the usual mix: the usual volumes, every pan in the
middle, the reverb off, no effects, no solo, and nothing muted but what
the band itself leaves out. **Ctrl+Z** brings your mix back.

## Placing the band

A mix is easier to hear when each part has a place of its own. Some
habits that work:

- **Keep the bass, the kick drum and the singer in the middle.** They
  carry the song, and low notes sound the same from anywhere.
- **Move the chords and the other parts a little to the sides**, as the
  players would stand on a stage: a piano at L30 and a guitar or an
  accordion at R30 leave the middle to the tune.
- **Turn down before turning up.** When a part is lost, lowering the
  others usually works better than raising it.
- **Accompaniment softer than the tune.** The band is there to support
  the melody: it should be heard, not cover it.

## The reverb

A **reverb** imitates a room: the sound comes back from its walls,
later and softer, for a second or more. Without it, music made by a
computer can sound as if it were played in a cupboard.

Tick **Reverb** to turn it on. It is one room for the whole song:

- **Wet / dry**: how much room is heard next to the sound itself. At
  20% the room stays in the background; at 50% it is as loud as the
  music, as in a church.
- **Decay**: how long the room keeps sounding, in seconds.
- **Damping**: how soon the high frequencies die away. Soft walls and
  curtains damp a room; stone does not.
- **Pre-delay**: how long before the room answers, in milliseconds. A
  little pre-delay keeps the notes clear in a big room.

Each channel's **Reverb** column says how much of it goes into the
room. Drums and strings sound natural with a lot; a bass usually sounds
better close and dry, with little or none.

## Effects

Click a channel's **Effects** to open its effects. They shape its sound
before it joins the others:

- **Equalizer**: turns three bands of frequencies up or down, up to 12
  dB: **Low** (below 200 Hz, the weight of a bass or a kick drum),
  **Mid** (around 1 kHz, the body of most instruments) and **High**
  (above 4 kHz, the air and sparkle of cymbals). Turning a piano's lows
  down a few decibels leaves room for the bass. **Flat** puts every
  band back to 0 dB, and a double-click puts one back.
- **Compressor**: evens out how loud the part plays. Above the
  **Threshold** it turns the sound down, the more the higher the
  **Ratio** (at 4:1, a peak 8 dB over the threshold comes out 2 dB
  over). **Gain** brings the whole part back up. At a ratio of 1:1 it
  does nothing.
- **Delay**: plays the part again a little later, in time with the
  song, as an echo. **Time** is how long after, in notes of the song
  (from a sixteenth to a half note, dotted eighths and quarters
  included), so the echo follows the tempo. **Feedback** says how loud
  each echo is next to the one before, and **Mix** how loud the echoes
  are; at 0% the delay is off.

**No effects** turns them all off.

## What the mixer changes

The mixer changes how the song sounds, not what it plays: the notes
stay as they are. What you hear, and a WAV or a video you export, sound
as the mixer says. A MIDI file you export leaves out the channels that
are muted, or silenced by a solo, but carries no volumes, pans or
effects: the program that opens it does its own mix.
