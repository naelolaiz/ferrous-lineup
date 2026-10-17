# MIDI messages and MIDI files

Files: [`crates/lineup-core/src/midi.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/midi.rs)
and [`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/midi_file.rs)

## Three bytes per key

A MIDI keyboard says almost everything in messages of three bytes. The
first one, the *status byte*, says what kind of message it is and on
which of 16 *channels*. The other two are *data bytes*. For a key going
down:

```text
  1001 0000   0011 1100   0110 0100
  │    │      │           │
  │    │      │           └ velocity 100: how hard the key was struck
  │    │      └ note 60: middle C (C4)
  │    └ channel 0 (people call it channel 1)
  └ 1001 = note-on
```

A status byte always has its top bit set (128 and up), and a data byte
never does (0 to 127). That is why a MIDI note number and a velocity are
never more than 127, and why `Note` in step 1 used the same numbers: C4
is 60 in Ferrous Lineup because it is 60 in MIDI.

There are many kinds of message: knobs and pedals ("control change"),
pitch bends, clock ticks for keeping machines in time. Ferrous Lineup only
needs two, and `MidiMessage` has only two variants:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MidiMessage {
    NoteOn { channel: u8, pitch: Note, velocity: u8 },
    NoteOff { channel: u8, pitch: Note },
}

/// The kinds of message, as the top four bits of the status byte.
const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
```

The status byte holds two things at once, so the code splits it with bit
operations (step 3 used the same ones for pitch sets): `status & 0xF0`
keeps the top four bits, the kind, and `status & 0x0F` keeps the bottom
four, the channel.

## Reading a message: slice patterns

```rust
pub fn parse(bytes: &[u8]) -> Option<MidiMessage> {
    let [status, pitch, velocity, ..] = *bytes else {
        return None;
    };
    // Data bytes have the top bit clear; anything else is malformed.
    if pitch > 127 || velocity > 127 {
        return None;
    }
    let channel = status & 0x0F;
    let pitch = Note::new(pitch);
    match status & 0xF0 {
        NOTE_ON if velocity > 0 => Some(MidiMessage::NoteOn { channel, pitch, velocity }),
        NOTE_ON | NOTE_OFF => Some(MidiMessage::NoteOff { channel, pitch }),
        _ => None,
    }
}
```

`let [status, pitch, velocity, ..] = *bytes else` is a **slice pattern**.
A slice can have any length, so a pattern for it describes a *shape*:
"at least three items; call the first three `status`, `pitch` and
`velocity`, and `..` for whatever follows". If the slice is shorter, the
pattern does not match and the `else` block returns. Without slice
patterns this would be a length check and three indexings, each of which
could panic if the check were wrong. The `*` matches the slice itself,
not the reference to it; it is allowed because the parts taken out are
`u8`s, which are `Copy`.

The `match` uses two things from earlier steps. `NOTE_ON` and `NOTE_OFF`
are constants, and a constant in a pattern means "equal to this value",
not "a new variable". And `NOTE_ON if velocity > 0` has a **match
guard**: the arm only matches when the condition holds too. A note-on
with velocity 0 falls through to the next arm and becomes a note-off.
Many keyboards really send that instead of a note-off: a stream of
messages of the same kind may leave out the repeated status byte
("running status"), which saved time on the slow cables of 1983.

Turning a message back into bytes is the same in reverse:

```rust
pub fn bytes(self) -> [u8; 3] {
    match self {
        MidiMessage::NoteOn { channel, pitch, velocity } => {
            [NOTE_ON | (channel & 0x0F), pitch.midi(), velocity.clamp(1, 127)]
        }
        // Velocity 64 is the usual "no particular speed" for a release.
        MidiMessage::NoteOff { channel, pitch } => [NOTE_OFF | (channel & 0x0F), pitch.midi(), 64],
    }
}
```

The `clamp(1, 127)` matters. A note-on with velocity 0 would be read as
a note-off by the synth, so a note played "as softly as possible" would
not be played at all.

One other message is written, never read: control change 123, "All Notes
Off". It tells a synth to stop every note on a channel. Ferrous Lineup
sends it on all 16 channels before it lets go of an output port, in
case a note-off got lost on the way:

```rust
pub fn all_notes_off(channel: u8) -> [u8; 3] {
    [CONTROL_CHANGE | (channel & 0x0F), 123, 0]
}
```

## A MIDI file

A Standard MIDI File (`.mid`) holds the same messages, with times. It
holds notes, not sound: "C4 starts here, ends there, on channel 1". The
program that opens it chooses the instruments. That makes it the common
language of music programs, and the way to take a Ferrous Lineup pattern
into another studio.

The file is a list of *chunks*. Each one starts with four letters and
its length:

```text
  MThd  length 6  format 1  3 tracks  96 ticks per beat     the header
  MTrk  length …  name "Ferrous Lineup", tempo, time signature, end
  MTrk  length …  name "Lineup Osc", the notes of the synth, end
  MTrk  length …  name "Kick", the notes of the kick, end
```

This is **format 1**: several tracks played together. The first track
holds only the tempo and the time signature, and every channel of the
project gets a track of its own, named after it. A channel with no
notes, or a muted one, gets no track.

Numbers in a MIDI file are big-endian: the most significant byte first.
Rust's integers have a method for that, which gives an array of bytes:

```rust
let mut file = Vec::new();
file.extend_from_slice(b"MThd");
file.extend_from_slice(&6u32.to_be_bytes()); // the header's length
file.extend_from_slice(&1u16.to_be_bytes()); // format 1
file.extend_from_slice(&(tracks.len() as u16).to_be_bytes());
file.extend_from_slice(&TICKS_PER_BEAT.to_be_bytes());
```

`b"MThd"` is a **byte string**: a `&[u8; 4]` with the ASCII codes of the
letters, instead of a `&str`. `6u32` is the number 6 as a `u32` (the
type written after the number), so `to_be_bytes` gives four bytes:
`[0, 0, 0, 6]`. A `u16` gives two.

## Ticks and delta times

Time in a MIDI file is counted in *ticks*, and the header says how many
ticks make a beat. Ferrous Lineup writes 96, which divides by 2, 3, 4, 8,
16 and 32, so triplets and thirty-second notes would fit too. A step is
a sixteenth note, a quarter of a beat, so it is 24 ticks:

```rust
pub const TICKS_PER_BEAT: u16 = 96;
pub const TICKS_PER_STEP: u32 = TICKS_PER_BEAT as u32 / STEPS_PER_BEAT;
```

Inside a track, each event is written after a *delta time*: the number
of ticks since the event before it, not since the start. Most deltas are
small (0 for the notes of a chord, 24 for the next step), so they are
written in a format where small numbers take one byte, the
**variable-length quantity**: seven bits of the number per byte, the
most significant first, with the top bit set on every byte except the
last.

```text
  200  = 1 1001000            ->  1000 0001  0100 1000   (0x81 0x48)
         │ └ last 7 bits                │         └ top bit clear: the last byte
         └ the rest: 1                  └ top bit set: more to come
  384  = 11 0000000           ->  0x83 0x00
```

```rust
fn write_variable_length(out: &mut Vec<u8>, value: u32) {
    let mut groups = [0u8; 5];
    let mut count = 0;
    let mut rest = value;
    loop {
        groups[count] = (rest & 0x7F) as u8;
        count += 1;
        rest >>= 7;
        if rest == 0 {
            break;
        }
    }
    for index in (0..count).rev() {
        let more = if index > 0 { 0x80 } else { 0 };
        out.push(groups[index] | more);
    }
}
```

The loop cuts the number into groups of seven bits from the low end,
because that is the end `& 0x7F` and `>>= 7` can reach. The file wants
them from the high end, so the second loop walks the groups backwards
with `(0..count).rev()`. Five groups are enough for any `u32`, so they
fit in an array, and nothing is allocated. The test feeds it the
examples printed in the MIDI file specification.

A `TrackWriter` keeps the tick of the last event, so the rest of the
code can think in absolute ticks:

```rust
#[derive(Default)]
struct TrackWriter {
    data: Vec<u8>,
    last_tick: u32,
}

impl TrackWriter {
    fn delta(&mut self, tick: u32) {
        let delta = tick.saturating_sub(self.last_tick);
        write_variable_length(&mut self.data, delta);
        self.last_tick = self.last_tick.max(tick);
    }
    ...
    fn finish(mut self, end: u32) -> Vec<u8> {
        self.meta(end.max(self.last_tick), END_OF_TRACK, &[]);
        let mut chunk = Vec::with_capacity(8 + self.data.len());
        chunk.extend_from_slice(b"MTrk");
        chunk.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        chunk.extend(self.data);
        chunk
    }
}
```

`#[derive(Default)]` gives `TrackWriter::default()`, an empty `Vec` and
tick 0. `finish` takes `mut self`, not `&mut self`: it consumes the
writer, and its data moves into the chunk without a copy. A finished
writer cannot be written to by mistake, because it no longer exists.

## Meta events: names, tempo, time signature

Events that are not notes start with `0xFF`, a kind, and a length:

| Bytes | Means |
|-------|-------|
| `FF 03 len text` | the track's name |
| `FF 51 03 tt tt tt` | the tempo, as microseconds per beat |
| `FF 58 04 4 2 24 8` | 4/4 time (4 beats, a beat is 2² = a quarter note) |
| `FF 2F 00` | the end of the track |

The tempo is the odd one: not beats per minute, but how long one beat
lasts, in microseconds, in three bytes. At 120 BPM a beat lasts half a
second, 500 000 µs, which is `07 A1 20`. The code takes the last three
of the four bytes `to_be_bytes` gives:

```rust
let micros = (60_000_000.0 / f64::from(project.tempo.bpm())).round() as u32;
let micros = micros.min(0xFF_FFFF).to_be_bytes();
track.meta(0, TEMPO, &micros[1..]);
```

`&micros[1..]` is a slice of the array from index 1 to the end: the
three low bytes.

## Notes, drums and channels

Each note becomes two events, its note-on and its note-off. The notes of
a pattern placed at a step of the song are moved by that many steps, and
a note running past the end of its pattern is cut there, exactly as the
audio thread plays it:

```rust
let held = note.length.min(length - note.start);
let on = (first_step + note.start) as u32 * TICKS_PER_STEP;
let off = on + held as u32 * TICKS_PER_STEP;
```

Then the events are sorted by time. At the same tick, note-offs must
come before note-ons: if a note ends exactly where the same note starts
again, "on, off" would end the new note at once, and only "off, on" is
heard twice.

```rust
events.sort_by_key(|&(tick, message)| (tick, matches!(message, MidiMessage::NoteOn { .. })));
```

The key is a tuple, `(tick, is_note_on)`, and tuples compare item by item
([step 6](../step-6/tools.md#grouping-notes-into-chords-chords) sorted
chords the same way). For the same tick, `false < true`, so note-offs
win. `sort_by_key` is a *stable* sort: events with equal keys keep their
order, so the notes of a chord stay in the order they were written.

Which MIDI channel does each track use? A program reading the file uses
the channel to pick the instrument, and General MIDI, the standard most
synths follow, keeps **channel 10** (9, counted from 0) for drums, with a
fixed note for each drum: 36 is a kick, 38 a snare, 39 a clap, 42 a
closed hi-hat. So:

```rust
fn midi_channels(channels: &[Channel]) -> Vec<u8> {
    let mut melodic = (0..16u8).filter(|&channel| channel != DRUM_CHANNEL).cycle();
    channels
        .iter()
        .map(|channel| match channel.kind {
            ChannelKind::Sampler(SampleSource::Builtin(_)) => DRUM_CHANNEL,
            ChannelKind::MidiOut { channel } => channel,
            _ => melodic.next().unwrap_or(0),
        })
        .collect()
}
```

The built-in drums share channel 10, and their notes are replaced by the
General MIDI ones (`general_midi_drum`). A MIDI Out channel keeps the
channel it was given. Every other channel takes the next one from
`melodic`: the numbers 0 to 15 without 9, repeated for ever by `cycle`
(which [step 6](../step-6/tools.md) used for arpeggios). The iterator is
created once, outside the closure, and the closure borrows it mutably:
each `next()` moves it on, so the channels get 0, 1, 2… in turn. With
more than 15 melodic channels, they start sharing.

## Testing a file format

The tests read every file back with a small reader, `read`, written in
the test module, the way any MIDI program would. Then they check what
came back:

```rust
#[test]
fn drums_use_channel_ten_and_general_midi_notes() {
    let tracks = read(&pattern_to_midi(&project(), 0));
    let kick = &tracks[2];
    assert!(kick.contains(&(0, vec![0x99, 36, DEFAULT_VELOCITY])));
    assert!(kick.contains(&(192, vec![0x99, 36, DEFAULT_VELOCITY])));
}
```

`0x99` is note-on (`0x90`) on channel 10 (`0x09`), and tick 192 is step 8.
A reader written by the same person who wrote the writer can share its
mistakes, though. So the exported files were also opened with a program
that knows nothing about Ferrous Lineup, Python's `mido` library, which
read the demo song as six tracks lasting 14.769 seconds: 128 steps at
130 BPM, as it should.
