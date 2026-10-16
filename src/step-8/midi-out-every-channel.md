# MIDI Out for every channel

Files: [`crates/loom-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-audio/src/instrument.rs),
[`crates/loom-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/midi.rs)
and [`crates/loom-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/app/live.rs)

[Step 7](../step-7/live.md) built MIDI Out the way FL Studio does it: a
channel of a special kind, *MIDI Out*, plays no sound and sends its notes
to the port chosen in the MIDI menu. Every other channel stays inside
Ferrous Loom.

That is right, and it is also a trap. Plug in a synth, pick it in
**MIDI > Output**, open a demo, press Play... and the synth says
nothing. None of the demos has a MIDI Out channel, so nothing is sent.
Nothing in the window says why, either. Someone reported exactly that:
"MIDI output does nothing".

## How to use it

1. Open the **MIDI** menu and pick your synth under **Output**.
2. Leave **Send every channel** ticked (it is on from the start).
3. Play. Every channel plays on the synth as well as on the speakers.
   Turn the volume (VOL) down to hear only the synth.

![The MIDI menu: an output picked, and Send every channel](../images/step-8-midi-out-menu.png)

The notes go out on the same MIDI channels that **File > Export MIDI**
writes, so a song sounds the same played live as from its MIDI file:

| Channel in the rack | MIDI channel | Notes |
|---------------------|--------------|-------|
| a built-in drum (Kick, Hat...) | 10 | its General MIDI drum note: kick 36, snare 38, clap 39, closed hat 42... |
| Loom Osc, or a sample from a file | 1, 2, 3... (10 is skipped) | the notes as written |
| a MIDI Out channel | the one you gave it | as before: it sends its notes itself |

Channel 10 is where every General MIDI synth keeps its drum kit, and
on it each *note* is a different drum. A drum's steps are all written
on C4 (note 60, which a drum kit plays as a bongo), so a drum sends its
own note instead of the one in the pattern.

Keys you play go out too: a MIDI keyboard, the typing keyboard, the
harmony keys, and clicking a channel's name to hear it. Untick **Send
every channel** to get FL Studio's behaviour back, where only MIDI Out
channels reach the port.

Two smaller things changed in the menu:

- The port lists leave out Ferrous Loom's own ports. On Linux, the port
  the app sends from shows up as an *input* too, and picking it would
  send every note round in a circle.
- If the audio engine could not start, the menu says that nothing can
  be sent. The notes are made on the audio thread, so without audio no
  note is made.

## An instrument wrapped in another

`instrument.rs`

The audio thread already had everything needed: `MidiOut` turns notes
into messages and keeps them in an outbox, and once per block the audio
thread hands the outbox on to the sending thread. What was missing was
a way for a sampler or a synth to fill an outbox *as well as* making
its sound.

Instead of teaching the sampler and the synth about MIDI, a new kind of
instrument holds one of them together with a `MidiOut`:

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    MidiOut(MidiOut),
    Echoed(Box<Echoed>),
}

pub struct Echoed {
    inner: Instrument,
    midi: MidiOut,
    /// The note every note goes out as, for a drum.
    pitch: Option<Note>,
}
```

Every method passes the call to both halves. Starting a note, for
example:

```rust
Instrument::Echoed(echoed) => {
    echoed.inner.note_on(pitch, velocity, gate, output_rate);
    let sent = echoed.pitch.unwrap_or(pitch);
    echoed.midi.note_on(sent, velocity, gate);
}
```

`unwrap_or(pitch)` reads "the drum's note if there is one, otherwise the
note itself". The `MidiOut` half also ends each note when its gate runs
out, exactly as a MIDI Out channel does, so a pattern's notes reach the
synth with the right lengths.

### Why the `Box`

An `Echoed` contains an `Instrument`, and an `Instrument` can be an
`Echoed`. Written without the `Box`, Rust refuses:

```text
error[E0072]: recursive type `Instrument` has infinite size
```

Rust must know how many bytes every value of a type takes. To fit an
`Echoed`, an `Instrument` would need room for another `Instrument`
inside, which needs room for another, with no end. A `Box` is a pointer
to memory elsewhere: it is always the same size (8 bytes on a 64-bit
computer), whatever it points to, so the loop is broken. The cost is one
allocation, made on the UI thread when the instrument is built, and the
old box goes back to the UI thread to be freed, like every other
instrument the audio thread throws away.

## Routes: who goes where

`midi.rs`

A *route* is where one channel's notes go out:

```rust
pub type Route = (u8, Option<Note>);
```

That is a MIDI channel, and the drum note if there is one. `type` only
gives a long type a short name; a `Route` is still a plain tuple.
`routes` works out one for each channel of the rack, with
`midi_channels` and `general_midi_drum`, the two functions the MIDI file
export already used (they are now public, so the app can call them):

```rust
pub fn routes(channels: &[Channel]) -> Vec<Option<Route>> {
    channels
        .iter()
        .zip(midi_channels(channels))
        .map(|(channel, midi_channel)| match &channel.kind {
            ChannelKind::MidiOut { .. } => None,
            ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
                Some((midi_channel, Some(general_midi_drum(*drum))))
            }
            _ => Some((midi_channel, None)),
        })
        .collect()
}
```

`zip` walks two lists side by side: each channel with its MIDI channel.

## Sending only what changed

`app/live.rs`

The routes depend on the menu (an output picked, the box ticked) and on
the rack: adding a synth above another one moves the lower one to the
next MIDI channel. Rather than chase every place that can change them,
the app works them out again every frame and compares:

```rust
let wanted = if self.live.send_all && self.live.midi.output().is_some() {
    midi::routes(channels)
} else {
    vec![None; channels.len()]
};
if self.live.routes_sent.as_ref() == Some(&wanted) {
    return;
}
```

Working out 32 routes at most is far cheaper than drawing the window,
and comparing two `Vec`s with `==` compares them element by element,
because tuples, `Option` and `Note` all implement `PartialEq`. Only
when they differ does a channel get a new instrument, wrapped or plain,
through `Command::SetInstrument`, and only the channels whose route
changed.

`routes_sent` is an `Option<Vec<...>>`: `None` means "not known".
When channels are added, removed or reloaded, the audio thread gets
plain instruments for them, and the ones that moved keep whatever they
had. If any of them was wrapped, the app forgets what it sent, and the
next frame sends every channel again. That is rare (it takes a change
to the rack), and it cannot go wrong.

## Testing MIDI without a synth

A test can check the bytes an instrument would send (the new tests in
`instrument.rs` and `midi.rs` do), but not that they reach a port. For
that, Linux has the ALSA sequencer, and two kernel modules that make
pretend MIDI devices: `snd-seq-dummy` (a "Midi Through" port that passes
everything on) and `snd-virmidi` (ports that look like a USB keyboard or
synth). `aseqdump` prints every message that reaches a port:

```sh
sudo modprobe snd-seq-dummy snd-virmidi
aseqdump -p "Midi Through"
```

Pick **Midi Through** in **MIDI > Output**, press Play, and each note
appears as it is sent:

```text
129:0   Note on                 9, note 36, velocity 100
129:0   Note on                 0, note 60, velocity 100
129:0   Note off                9, note 36, velocity 64
```

`aseqdump` counts channels from 0, so its 9 is MIDI channel 10, the
drums. Writing bytes to a virtual port's raw device plays a key on a
pretend keyboard: pick **Virtual Raw MIDI 0-0** as the input, then

```sh
printf '\x90\x45\x50' > /dev/snd/midiC0D0   # A4 down, velocity 80
printf '\x80\x45\x00' > /dev/snd/midiC0D0   # and up
```

This change was checked that way, in a virtual machine with those
modules and an emulated sound card. With the old code, the demo song
sent nothing. With the new one it sent 137 notes on and 137 off, drums
on channel 10 and chords on channel 1, and the keyboard's A came out on
channel 1 with its velocity.
