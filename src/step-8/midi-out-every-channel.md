# MIDI Out for every channel

Files: [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs)
and [`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs)

[Step 7](../step-7/live.md) built MIDI Out the way FL Studio does it: a
channel of a special kind, *MIDI Out*, plays no sound and sends its notes
to the port chosen in the MIDI menu. Every other channel stays inside
Ferrous Lineup.

That is right, and it is also a trap. Plug in a synth, pick it in
**MIDI > Output**, open a demo, press Play... and the synth says
nothing. None of the demos has a MIDI Out channel, so nothing is sent.
Nothing in the window says why, either. Someone reported exactly that:
"MIDI output does nothing".

## How to use it

1. Open the **MIDI** menu and pick your synth under **Output**.
2. Leave **Send every channel** ticked (it is on from the start).
3. Play. Every channel plays on the synth instead of the speakers, as
   a MIDI Out channel does. Pick **None** as the output, or untick the
   box, to hear Ferrous Lineup's own sounds again.

![The MIDI menu: an output picked, and Send every channel](../images/step-8-midi-out-menu.png)

The notes go out on the same MIDI channels that **File > Export MIDI**
writes, so a song sounds the same played live as from its MIDI file:

| Channel in the rack | MIDI channel | Notes |
|---------------------|--------------|-------|
| a built-in drum (Kick, Hat...) | 10 | its General MIDI drum note: kick 36, snare 38, clap 39, closed hat 42... |
| Lineup Osc, or a sample from a file | 1, 2, 3... (10 is skipped) | the notes as written |
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

- The port lists leave out Ferrous Lineup's own ports. On Linux, the port
  the app sends from shows up as an *input* too, and picking it would
  send every note round in a circle.
- If the audio engine could not start, the menu says that nothing can
  be sent. The notes are made on the audio thread, so without audio no
  note is made.

## A channel turned into a MIDI Out

`instrument.rs`

The audio thread already had everything needed: `MidiOut` turns notes
into messages and keeps them in an outbox, and once per block the audio
thread hands the outbox on to the sending thread. So while every channel
is sent, each sampler and synth is simply *replaced* by a `MidiOut`, and
put back when it stops:

```rust
pub fn to_midi(self, channel: u8, pitch: Option<Note>) -> Self {
    match self {
        Instrument::MidiOut(_) => self,
        _ => Instrument::MidiOut(MidiOut::new(channel).playing_as(pitch)),
    }
}
```

`to_midi` takes `self` by value: the old instrument is *moved* into the
function and dropped there, and the caller gets the new one back. A MIDI
Out channel already sends its notes, so it comes back unchanged.

The one new thing a `MidiOut` learnt is a fixed note, for drums:

```rust
pub fn note_on(&mut self, pitch: Note, velocity: u8, gate: u32) {
    let pitch = self.pitch.unwrap_or(pitch);
    ...
```

`unwrap_or(pitch)` reads "the drum's note if there is one, otherwise the
note itself". Writing `let pitch = ...` again *shadows* the parameter:
from that line on, `pitch` means the new value, and the old one cannot
be used by mistake. `note_off` does the same, so a held drum key lets go
of the note it started. The `MidiOut` ends each note when its gate runs
out, so a pattern's notes reach the synth with the right lengths.

A first version kept the sampler or synth playing and added a `MidiOut`
beside it, so the song sounded twice: through the speakers and on the
synth. That is not what a MIDI output is for, so it was taken out.

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
when they differ does a channel get a new instrument, a MIDI Out or its own,
through `Command::SetInstrument`, and only the channels whose route
changed.

`routes_sent` is an `Option<Vec<...>>`: `None` means "not known".
When channels are added, removed or reloaded, the audio thread gets
plain instruments for them, and the ones that moved keep whatever they
had. If any of them was a MIDI Out in place of its own instrument, the app forgets what it sent, and the
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
