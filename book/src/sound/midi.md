# MIDI: messages, devices, recording and files

Files: [`crates/lineup-core/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs),
[`crates/lineup-core/src/midi_file/read.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file/read.rs),
[`crates/lineup-core/src/midi_file/import.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file/import.rs),
[`crates/lineup-core/src/record.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/record.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs),
[`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs)
and [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs)

MIDI is how keyboards, synths and music programs talk to each other. It
has been the same since 1983: a few bytes that say "this key went down,
this hard" or "this key came up". Ferrous Lineup speaks it both ways:

- the keys of a **MIDI keyboard**, or of the computer's own keyboard
  laid out as a piano, play the *selected channel*, the one the piano
  roll shows, and the status bar names the chord they hold;
- **recording** turns the keys played while a pattern loops into notes
  of that pattern;
- **MIDI Out** sends channels to an external synth instead of the
  speakers: every channel at once, or the ones you choose;
- **MIDI files** (`.mid`) are written from a song or a pattern, and
  read back into a new project.

How to use all this is in the manual's
[MIDI keyboards and synths](https://naelolaiz.github.io/ferrous-lineup/manual/midi.html),
[Opening MIDI files and recordings](https://naelolaiz.github.io/ferrous-lineup/manual/importing.html)
and [Exporting](https://naelolaiz.github.io/ferrous-lineup/manual/exporting.html).
This chapter follows a key on its way: the bytes, the threads they
cross, the note they become, and the file that keeps them.

Four threads share the work. A key from a MIDI keyboard takes this
path:

```text
  MIDI keyboard
     │ 3 bytes
     ▼
  midir's thread: on_message ──▶ LiveInput (ring buffer) ──▶ audio thread ──▶ sound card
     │                                                          │
     │ a copy, with the playback position (mpsc channel)        │ MIDI Out channels' messages
     ▼                                                          ▼
  UI thread: keys held, chord name, recording              MidiOutput (ring buffer)
     │                                                          │
     │ typing keyboard: Command::Live ──▶ audio thread          ▼
                                                    sending thread ──▶ MIDI port ──▶ synth
```

Four ideas carry the design:

1. **Keys skip the UI.** The UI draws once per frame, and a key could
   wait a whole frame there before anything happens. So a key goes from
   the MIDI thread straight to the audio thread, and sounds within one
   audio block, a few milliseconds. The UI gets a copy, to show and
   record it.
2. **The audio thread still never waits.** Sending a MIDI message is a
   system call, which may take any time. So the audio thread leaves
   messages in a queue, and a thread of their own sends them.
3. **One clock for everything.** The audio thread publishes where it is
   in the pattern, to a fraction of a step. Each key is stamped with it
   as it arrives, and recording only has to round.
4. **A file is read as data that can be wrong.** A MIDI file from the
   internet may be cut short or damaged. Every read can fail without a
   panic, and the import keeps what it could read and says so.

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
never more than 127, and why `Note` uses the same numbers: C4 is 60 in
Ferrous Lineup because it is 60 in MIDI.

There are many kinds of message: knobs and pedals ("control change"),
pitch bends, clock ticks for keeping machines in time. The app needs
three:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MidiMessage {
    NoteOn {
        channel: u8,
        pitch: Note,
        /// How hard the key was struck, 1 to 127.
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        pitch: Note,
    },
    /// From now on, `channel` plays General MIDI program `program`, 0 to
    /// 127. Only sent, never read: a keyboard choosing its own sound does
    /// not change the channel it plays.
    ProgramChange {
        channel: u8,
        program: u8,
    },
}

/// The kinds of message, as the top four bits of the status byte.
const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
const CONTROL_CHANGE: u8 = 0xB0;
const PROGRAM_CHANGE: u8 = 0xC0;
```

The status byte holds two things at once, so the code splits it with
bit operations, the ones [pitch-class sets](../theory/sets.md) use
too: `status & 0xF0` keeps the top four bits, the kind, and
`status & 0x0F` keeps the bottom four, the channel.

### Reading a message: slice patterns

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
        NOTE_ON if velocity > 0 => Some(MidiMessage::NoteOn {
            channel,
            pitch,
            velocity,
        }),
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

The `match` uses two more things. `NOTE_ON` and `NOTE_OFF` are
constants, and a constant in a pattern means "equal to this value", not
"a new variable". And `NOTE_ON if velocity > 0` has a **match guard**:
the arm only matches when the condition holds too. A note-on with
velocity 0 falls through to the next arm and becomes a note-off. Many
keyboards really send that instead of a note-off: a stream of messages
of the same kind may leave out the repeated status byte ("running
status"), which saved time on the slow cables of 1983.

`parse` reads only notes. Program changes are only ever sent, and every
other message, from a sustain pedal to a clock tick, gives `None`.

### Writing one: two or three bytes

A note is three bytes, a program change two. `bytes` returns a small
value that holds three and says how many count:

```rust
/// The bytes of one message: two or three of them, as a slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiBytes {
    bytes: [u8; 3],
    len: usize,
}

impl std::ops::Deref for MidiBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
```

**`Deref`** is the trait behind the `*` operator. Implementing it says
"a `MidiBytes` can be used as a `[u8]`": `*bytes` gives the slice, and
so does passing `&bytes` where a `&[u8]` is wanted, because Rust follows
`Deref` by itself there (*deref coercion*). So the sending code can pass
`&message.bytes()` to a port, a test can compare `*off.bytes()` with
`[0x80, 60, 64]`, and nothing is allocated: the three bytes live in the
value, on the stack.

```rust
pub fn bytes(self) -> MidiBytes {
    let (bytes, len) = match self {
        MidiMessage::NoteOn {
            channel,
            pitch,
            velocity,
        } => (
            [
                NOTE_ON | (channel & 0x0F),
                pitch.midi(),
                velocity.clamp(1, 127),
            ],
            3,
        ),
        // Velocity 64 is the usual "no particular speed" for a release.
        MidiMessage::NoteOff { channel, pitch } => {
            ([NOTE_OFF | (channel & 0x0F), pitch.midi(), 64], 3)
        }
        MidiMessage::ProgramChange { channel, program } => {
            ([PROGRAM_CHANGE | (channel & 0x0F), program & 0x7F, 0], 2)
        }
    };
    MidiBytes { bytes, len }
}
```

The `clamp(1, 127)` matters. A note-on with velocity 0 would be read as
a note-off by the synth, so a note played "as softly as possible" would
not be played at all. `& 0x0F` and `& 0x7F` keep a wrong channel or
program number from spilling into the bits of another field.

One message is written and never read: control change 123, "All Notes
Off". It tells a synth to stop every note on a channel:

```rust
pub fn all_notes_off(channel: u8) -> [u8; 3] {
    [CONTROL_CHANGE | (channel & 0x0F), 123, 0]
}
```

The app sends it on all 16 channels whenever it lets go of a synth,
because a note-off lost on the way would leave a note sounding for
ever: when another output port is picked, and when the audio thread
that owed the note-offs goes away ([Two threads](../program/threads.md#when-the-sound-card-goes-away)).

## A key's way in

Most notes come from the sequencer, which lives on the audio thread and
knows each note's length before it starts. A key on a keyboard is
different in two ways: it comes from outside, on a thread Ferrous
Lineup does not own, and nobody knows how long it will be held.

### Why keys skip the UI

The obvious path for a key would be the one every other change takes:
the UI hears about it, and sends a `Command` to the audio thread. But
the UI only runs once per frame. At 60 frames a second that is up to
17 ms of waiting, and on a slow computer much more, before the audio
thread even hears of the key. A pianist feels that.

The audio thread runs far more often: once per *block*, every few
milliseconds. So a key from a MIDI keyboard goes straight there, and the
UI gets a copy afterwards, only to show it and record it:

```text
  midir's thread ──▶ LiveInput ──▶ audio thread        (plays it)
        └────────▶ mpsc channel ──▶ UI thread           (shows and records it)
```

The typing keyboard cannot do this: egui reads the keyboard, and egui
runs on the UI thread. So typed keys take the slow way, as a
`Command::Live(MidiMessage)`. On the audio thread both end up in the
same method, `play_live`.

### One producer per queue: `LiveInput`

The command queue is a ring buffer from the `rtrb` crate
([Two threads](../program/threads.md#sending-commands-with-a-ring-buffer)),
and its `Producer` belongs to the `AudioEngine`, on the UI thread. rtrb
is *single producer, single consumer*: one thread pushes, one thread
pops, and that is what lets it work without locks. midir's thread is a
second thread that wants to push, so it needs a ring buffer of its own:

```rust
pub struct LiveInput {
    notes: rtrb::Producer<MidiMessage>,
    position: Arc<AtomicU64>,
}

impl LiveInput {
    /// Plays (or lets go of) a key on the live channel. Returns `false`
    /// if the queue was full and the key was lost.
    pub fn send(&mut self, message: MidiMessage) -> bool {
        self.notes.push(message).is_ok()
    }

    /// Where playback is, in steps (see [`Sequencer::position`]), or
    /// `None` when stopped. Recording uses it to place the key.
    pub fn position(&self) -> Option<f64> {
        load_position(&self.position)
    }
}
```

Because there can be only one producer, this cannot be a method on
`AudioEngine` that anybody may call. It is a *value*, and whoever owns
it is the one producer. The engine creates it when it starts and keeps
it in an `Option` until someone asks for it:

```rust
pub fn take_live_input(&mut self) -> Option<LiveInput> {
    self.live_input.take()
}
```

`Option::take` moves the value out and leaves `None` behind, so the
second call gets nothing. The type system makes the rule "one producer"
impossible to break: `LiveInput` does not implement `Clone`, so it
cannot be copied, only moved. `MidiOutput`, the way out for MIDI Out
channels, works the same way from the other end: it holds the queue's
`Consumer`, and `take_midi_output` hands it over once.

On the audio thread, the consumer is an `Option` too, because the audio
thread that [exports a song](export.md) has no keyboard at
all:

```rust
while let Some(message) = self.live_input.as_mut().and_then(|keys| keys.pop().ok()) {
    self.play_live(message);
}
```

That line packs three steps. `self.live_input` is an
`Option<Consumer>`, and calling `pop` needs a `&mut Consumer`.
`as_mut()` turns `&mut Option<Consumer>` into `Option<&mut Consumer>`,
borrowing what is inside instead of moving it out. `and_then` runs the
closure only if there is a consumer, and the closure's own `Option`
(from `pop().ok()`: `Some(key)`, or `None` when the queue is empty)
becomes the result. So the loop goes on while there is a consumer *and*
it has a key.

### Where in the pattern: the position

Recording needs to know *when* each key came, as a place in the
pattern: "halfway through step 4". The sequencer can tell, since it
counts down the frames until the next step
([The sequencer](sequencer.md)):

```rust
pub fn position(&self) -> f64 {
    if self.frames_until_next <= 0.0 {
        return self.next_step as f64;
    }
    let current = (self.next_step + self.length - 1) % self.length;
    let passed = 1.0 - self.frames_until_next / self.frames_per_step;
    current as f64 + passed.clamp(0.0, 1.0)
}
```

`next_step` is the step about to fire, so the one playing is the step
before it. `next_step - 1` would go below zero at step 0, which a
`usize` cannot do (it would panic in a debug build). Adding `length`
first and taking the remainder goes back one step *around the loop*:
from step 0 to the last step. `passed` is how much of that step is
over, from 0 to 1.

The audio thread stores it after every block, as the bits of the `f64`
in an `AtomicU64`, since there is no atomic float
([Two threads](../program/threads.md#the-playhead-an-atomic-integer)).
"Stopped" is `NO_POSITION = u64::MAX`; as a float, those bits are a NaN
("not a number"), which no position ever is. `LiveInput` holds a clone
of the same `Arc`, so midir's thread can read the position at the very
moment a key arrives. It is up to one block old then, about a
hundredth of a second; a step at 120 BPM lasts an eighth of a second,
and recording rounds to the nearest step anyway.

### A key on the audio thread

A note of the song gets a gate, the number of frames it is held. A key
has no known length, so it gets `HELD`, a gate that never runs out:
only `note_off` ends it ([Instruments](instruments.md)).
`play_live` starts and ends them:

```rust
fn play_live(&mut self, message: MidiMessage) {
    match message {
        MidiMessage::NoteOn {
            pitch, velocity, ..
        } => {
            // A key that is already down (on both keyboards at once,
            // say) comes up first.
            self.release_live(pitch);
            if let Some(channel) = self.live_channel
                && let Some(state) = self.channels.get_mut(channel)
            {
                state
                    .instrument
                    .note_on(pitch, velocity, HELD, self.sample_rate);
                self.live_keys[usize::from(pitch.midi())] = Some(channel);
            }
        }
        MidiMessage::NoteOff { pitch, .. } => self.release_live(pitch),
        MidiMessage::ProgramChange { .. } => {}
    }
}
```

Keys play the *live channel*, which the UI sets with
`Command::SetLiveChannel` whenever the selected channel changes. But the
selection can change while a key is down. If the key then came up on
the new channel, the note on the old one would play for ever. So the
audio thread remembers, for each of the 128 notes, which channel its
key went down on:

```rust
live_keys: [Option<usize>; 128],

fn release_live(&mut self, pitch: Note) {
    if let Some(channel) = self.live_keys[usize::from(pitch.midi())].take()
        && let Some(state) = self.channels.get_mut(channel)
    {
        state.instrument.note_off(pitch);
    }
}
```

An array of 128 `Option`s is a map from note to channel that never
allocates, and `take()` reads an entry and empties it in one go. The
`if let ... && let ...` is a **let chain**: both patterns must match for
the block to run.

Channels are known by their position in the list, and inserting or
removing a channel moves the ones after it. A remembered "channel 2"
could then be a different channel, or none. So before the list changes,
`release_live_keys` lets go of every key that is down. The test
`removing_channels_lets_go_of_their_keys` removes channel 0 while a key
is down on channel 1, which then *becomes* channel 0.

Live notes play like a preview: on a muted channel too, and whether or
not the sequencer is playing. Muting stops the song's notes on a
channel, not the key you are holding
(`muting_a_channel_stops_its_notes_but_not_keys_held_down`). Stopping
playback lets go of everything, keys included.

## midir: ports and connections

[midir](https://docs.rs/midir) is the crate that talks to each system's
MIDI layer: ALSA on Linux, Core MIDI on macOS, Windows Multimedia on
Windows. It is built the way `cpal` is: list the ports, open a
connection to one, and the connection stays open for as long as the
value is kept. Ferrous Lineup wraps it in a `Midi` struct, in
`lineup-app/src/midi.rs`.

Whether there is MIDI at all is found out by trying: creating a
`MidiInput` fails on a system without a MIDI layer, and the error is
kept to show in the menu. On Linux that is a computer without the ALSA
*sequencer* (the `snd-seq` kernel module, which makes `/dev/snd/seq`),
and the menu's hint says so.

Ports are found again by an id that midir gives, a `String` that stays
the same while the device is plugged in, because the list of ports
changes as devices come and go. The MIDI menu looks for ports each time
it opens (`refresh_ports`), and leaves out Ferrous Lineup's own: on
Linux, the port the app sends from shows up as an *input* too, and
picking it would send every note round in a circle.

```rust
fn is_own(name: &str) -> bool {
    name.starts_with(&format!("{CLIENT_NAME}:"))
}
```

The colon matters: a synth called "Ferrous Lineupish Synth" is not the
app (the test `its_own_ports_are_left_out` checks that).

### Connecting consumes, closing gives back

```rust
let mut midi_in = MidiInput::new(CLIENT_NAME).map_err(|err| err.to_string())?;
// Clock ticks, "I am still here" messages and system exclusive
// data are not notes. Leaving them out saves waking up for them.
midi_in.ignore(Ignore::All);
let found = midi_in
    .find_port_by_id(&port.id)
    .ok_or_else(|| crate::i18n::trf("{} is not connected any more", &[&port.name]))?;
let link = InputLink {
    live: self.live.clone(),
    to_ui: self.to_ui.clone(),
    ctx: self.ctx.clone(),
    snap: Arc::clone(&self.snap),
    sounding: Vec::new(),
};
let connection = midi_in
    .connect(&found, "input", on_message, link)
    .map_err(|err| err.to_string())?;
```

`connect` takes `self`, not `&self`: the `MidiInput` is consumed and
lives inside the connection from then on. `close` gives it back, with
the data. This is ownership used as a state machine: a `MidiInput` that
is connected cannot be connected twice, because you no longer have it.

`on_message` is a plain function, and `link` is the data it gets each
time, as `&mut InputLink`:

```rust
fn on_message(_microseconds: u64, bytes: &[u8], link: &mut InputLink) {
    let Some(message) = MidiMessage::parse(bytes) else {
        return;
    };
    let message = link.snapped(message);
    let mut position = None;
    if let Some(live) = &link.live {
        let mut live = lock(live);
        live.send(message);
        position = live.position();
    }
    // `send` fails only if the UI is gone, and then nobody needs the copy.
    let _ = link.to_ui.send(Played { message, position });
    // egui only redraws when something happens. This is something, and
    // `request_repaint` is one of the few egui calls meant for other threads.
    link.ctx.request_repaint();
}
```

It runs on midir's thread, so `connect` asks for it and its data to be
`Send + 'static`, like the audio callback. The time midir passes in is
ignored: the position from the audio thread is the clock that matters.

### Why the `LiveInput` sits behind a `Mutex`

`InputLink` holds the `LiveInput` as `Arc<Mutex<LiveInput>>`, which
looks odd for a value only one thread uses at a time. The reason is
midir's errors. If `connect` fails, the error gives back the
`MidiInput`, but *not* the data: `link` would be gone, and with it the
one producer of the live queue, for good. No keyboard could ever be
connected again.

So the `Midi` struct keeps the `LiveInput` itself, behind an
`Arc<Mutex<>>`, and each link holds a clone of the `Arc`. A failed
connection loses only a clone. Only one connection exists at a time, so
the lock is never waited on, and midir's thread is not the audio
thread, so it may lock. The same `Arc` lets the app swap in a new
engine's `LiveInput` after the sound card was opened again
(`reconnect_audio`), without reconnecting the keyboard.

### Poisoned locks

```rust
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
```

If a thread panics while it holds a lock, Rust marks the `Mutex` as
**poisoned**, and every later `lock()` returns an `Err`: the data
inside may be half changed, and Rust will not hand it over without
asking. `PoisonError::into_inner` is the way to say "give it to me
anyway". Here it is safe, because a queue end, a connection or a key is
never left half changed. The helper is generic over `T`, so it works
for every mutex in the file.

### A copy for the UI: `mpsc`

The UI's copy of each key goes through a `std::sync::mpsc` channel
("multiple producer, single consumer"), from the standard library:

```rust
let (to_ui, from_keyboard) = mpsc::channel();
// ...
pub fn played(&self) -> Vec<Played> {
    self.from_keyboard.try_iter().collect()
}
```

Unlike rtrb, it allocates and may lock, which is fine here: neither
midir's thread nor the UI thread is real-time. And the `Sender` can be
cloned, so each new connection gets its own. `try_iter` gives the
messages that are waiting and stops when there are none, without
waiting for more: right for a UI that must not stop drawing.

egui only draws a frame when something happens: the mouse moves, a key
is typed. A key on a MIDI keyboard is not something egui sees, so
`on_message` calls `ctx.request_repaint()`. `egui::Context` is a handle
that can be cloned and sent to other threads, and `request_repaint` is
one of the few egui calls meant to be used from them.

### Snapping keys to the key

The piano roll's **Snap notes you play** moves every key you play to the
nearest note of the key the song is in at that point, so a wrong note
cannot happen ([Scales, modes and keys](../theory/scales.md)). For a
MIDI keyboard that has to happen on midir's thread, before the key goes
to the audio thread. The UI shares the key to snap to through one more
`Arc<Mutex<Option<Scale>>>`, set once per frame, and the link snaps
each key with `tools::nearest_in_key` ([Editing tools as plain
functions](../theory/tools.md)).

The key of the song can change while a key is held: playback crosses a
key marker. Then the key-up, snapped to the new key, could name another
note, and the note started by the key-down would never end. So the link
remembers, for each key that is down, the note it plays:

```rust
MidiMessage::NoteOff {
    channel,
    pitch: key,
} => {
    let at = self.sounding.iter().position(|&(k, _)| k == key);
    let pitch = at.map_or(key, |at| self.sounding.remove(at).1);
    MidiMessage::NoteOff { channel, pitch }
}
```

`map_or(key, ...)` reads "the note this key started, or the key itself
if it started nothing we know of". The typing keyboard keeps the same
pairs for its keys, in `typing_down`.

## The typing keyboard

No MIDI keyboard at hand? **MIDI > Typing keyboard to piano** turns the
letter keys into one, laid out the way a piano's black keys sit between
and above the white ones:

```text
  black keys:   S  D     G  H  J      2  3     5  6  7     9  0
  white keys:  Z  X  C  V  B  N  M   Q  W  E  R  T  Y  U  I  O  P
  notes:       C4 D4 E4 F4 G4 A4 B4  C5 D5 E5 F5 G5 A5 B5 C6 D6 E6
```

The layout is a table, `TYPING_KEYS: &[(Key, u8)]`, and `typing_note`
looks a key up in it with `find`. Reading the keys happens once per
frame. egui keeps the keyboard's events of this frame in a list, and the
code picks out the ones it wants:

```rust
egui::Event::Key {
    key,
    physical_key,
    pressed,
    repeat: false,
    modifiers,
} => {
    // The *physical* key, where it is on the keyboard,
    // so the piano keeps its shape on any layout:
    // on a French keyboard, W is where Z is in English.
    let key = physical_key.unwrap_or(*key);
    // Ctrl+Z is undo, not a note. Key releases always
    // count, or a note could stay on.
    let shortcut = modifiers.command || modifiers.alt;
    (!*pressed || !shortcut).then_some((key, *pressed))
}
```

Four details make it behave like a piano:

- **`repeat: false`** in the pattern. A key held down makes the system
  repeat it ("aaaaa" in a text field). A field written as a value in a
  pattern must equal that value, so repeats do not match, and a held
  key plays once.
- **`physical_key`**. `key` is the letter the keyboard layout prints on
  the key; `physical_key` is where the key *is*. Some systems do not
  report it, so `unwrap_or` falls back to the letter.
- **Shortcuts are not notes.** A key going down with Ctrl (Cmd on a
  Mac) or Alt held is skipped. A key coming *up* always counts, whatever
  is held, or a note started before Ctrl was pressed would never end.
  The app's one-key shortcuts (Space, L, Home, End, Delete...) are
  not on the piano's keys, so they still work while it is on.
- **Focus.** If the window loses the keyboard (you click another
  window), or a text field, a menu or a dialog takes it, the keys' "up"
  events go elsewhere. So all typing keys are let go at once when that
  happens.

The events are read inside `ctx.input(|input| ...)`, which holds a lock
on egui's input while the closure runs. The closure only collects a
`Vec` of `(key, pressed)`, and the loop that plays the notes runs after
it, with the lock released.

Letting go of all keys uses `std::mem::take`:

```rust
fn release_typing_keys(&mut self) {
    // `take` leaves an empty list behind, so the loop can call `&mut self` methods.
    for (_, pitch) in std::mem::take(&mut self.live.typing_down) {
        self.play_typed(MidiMessage::NoteOff { channel: 0, pitch });
    }
}
```

Looping over `&self.live.typing_down` would borrow `self` for the whole
loop, and `play_typed` needs `&mut self`. Taking the list out first
leaves an empty one in its place, and the loop owns the old one, so the
two never overlap.

`play_typed` sends the key to the audio thread as `Command::Live`, then
hands it to `key_played` with the position, exactly as a key from midir
arrives. The [harmony keys](../theory/numerals.md) play their chords
the same way. From there on, the app cannot tell the keyboards apart.

### The selected channel

Keys need a channel to play, and the piano roll already has one: the
channel it shows. Once per frame, `live_input` tells the audio thread if
that changed:

```rust
let channel = self
    .piano_roll
    .selected_channel()
    .filter(|&channel| channel < self.project.channels().len());
if self.live.channel_sent != Some(channel) {
    self.send(Command::SetLiveChannel(channel));
    self.live.channel_sent = Some(channel);
}
```

`channel_sent` is an `Option<Option<usize>>`, and both layers mean
something. The outer `None` is "nothing sent yet", so the first frame,
and the first frame after a new project, always sends. `Some(None)` is
"sent: no channel", which the audio thread takes as "let go of every key
and ignore new ones". Folding the two into one `Option` would make one
of them impossible to say.

Choosing another channel also ends a take that is being recorded, and
lets go of every key that is down, so a key never ends on a channel it
did not start on.

### Keys held

Every key that is down, from either keyboard, is kept in a list of
`Held`, in the order they went down:

```rust
struct Held {
    pitch: Note,
    velocity: u8,
    /// The channel it plays, and will be recorded into.
    channel: usize,
    /// Where playback was when it went down, if it is being recorded.
    recorded_from: Option<f64>,
}
```

This list is what the rest of the app means by "the keys you hold". The
status bar names their chord ("🎹 Am: A minor, chord vi of C major"),
the piano roll lights them up on its keyboard, the harmony keys and the
key finder listen to them ([Hearing harmony](../theory/hearing.md)),
and **Accompaniment follows chords** turns them into the band's chord
([How the band plays a chord sheet](../band/band.md)). Recording is
one more use.

## Recording: from a key to a note

The **⏺** button next to Play only *arms* recording. Keys are recorded
while four things are true at once:

```rust
pub(super) fn is_recording(&self) -> bool {
    self.live.recording
        && self.playing
        && self.mode == PlayMode::Pattern
        && self.piano_roll.selected_channel().is_some()
}
```

Armed, playing, in pattern mode (a take goes into the pattern that
loops), and with a channel to record into. Arming switches to pattern
mode for you, so ⏺ and then Space is enough.

### Remembering where a key went down

When a key goes down, `key_played` adds a `Held`:

```rust
self.live.held.retain(|held| held.pitch != pitch);
self.live.follow.key_down();
let recorded_from = played.position.filter(|_| self.is_recording());
self.live.held.push(Held {
    pitch,
    velocity,
    channel,
    recorded_from,
});
```

`retain` first removes the same pitch if it was already down (on the
other keyboard, say), so each pitch is in the list once.

`Option::filter` keeps a `Some` only if the closure says yes, and turns
it into `None` otherwise. So `recorded_from` is the position when the
key is being recorded, and `None` when it is just being played. The
closure ignores its argument (`|_|`): the question is not about the
position, but about whether recording is on.

When the key comes up, its `Held` is taken out of the list, and if it
has a start position and there is an end position, it becomes a note:

```rust
let held = self.live.held.remove(index);
if let (Some(from), Some(until)) = (held.recorded_from, played.position) {
    self.record(held, from, until);
}
```

Matching a tuple of two `Option`s checks both at once: the block runs
only if both are `Some`.

### Quantizing: `recorded_note`

Nobody plays exactly on the step. A key that went down at 3.9 and came
up at 6.2 was meant as a note from step 4 to step 6. Moving the ends of
a note to the grid is called **quantizing**, and it is a plain function
in `lineup-core`, with no keyboard and no UI in sight:

```rust
pub fn recorded_note(pitch: Note, velocity: u8, from: f64, until: f64, length: usize) -> NoteEvent {
    let length = length.max(1);
    let until = if until < from {
        until + length as f64
    } else {
        until
    };
    let start = from.round();
    let end = until.round().max(start + 1.0);
    // `as usize` turns a float into an integer, dropping the fraction
    // (there is none left after `round`). Negative numbers become 0.
    let start_step = start as usize % length;
    let steps = ((end - start) as usize).min(length - start_step);
    NoteEvent {
        velocity,
        ..NoteEvent::new(pitch, start_step, steps)
    }
}
```

Line by line:

1. `length.max(1)` guards the `%` below: a remainder by zero panics. A
   pattern is never empty, but a function should not depend on its
   caller for that.
2. The pattern loops. If the key came up *before* where it went down
   (down at 14.5, up at 1.5), the loop started again in between, and the
   end is really 17.5, one loop further on.
3. `round()` moves each end to the nearest step. `f64::round` rounds
   halves away from zero, so 2.5 becomes 3.
4. `.max(start + 1.0)`: a quick tap would round both ends to the same
   step, a note of length 0. A note is at least one step long.
5. A key pressed just before the loop starts again (at 15.7 in a
   16-step pattern) rounds to 16, which is step 0 of the next time
   round: `% length` folds it back to 0. That is where the player meant
   it.
6. A note cannot run past the end of the pattern, so it is cut there
   with `.min(length - start_step)`.

The last lines use **struct update syntax**: `..NoteEvent::new(...)`
means "every field not named, as in this value", so the note gets its
velocity and `nudge` and `trim` of 0 from `new`. A recorded note lands
on the grid.

The tests read like those rules:

```rust
#[test]
fn ends_go_to_the_nearest_step() {
    assert_eq!(recorded(2.0, 4.0), (2, 2));
    assert_eq!(recorded(1.8, 4.3), (2, 2), "a little early, a little late");
    assert_eq!(recorded(2.4, 4.6), (2, 3));
}

#[test]
fn just_before_the_loop_is_step_zero() {
    assert_eq!(recorded(15.7, 0.4), (0, 1));
    assert_eq!(recorded(15.6, 1.9), (0, 2));
}

#[test]
fn a_note_across_the_loop_end_is_cut_there() {
    assert_eq!(recorded(14.0, 2.0), (14, 2));
}
```

`recorded(from, until)` is a helper in the test module that records a
C4 in a 16-step pattern and returns `(start, length)`. Being able to
write `15.7` instead of having to play a key at exactly that moment is
why the function lives in `lineup-core` and takes plain numbers.

`start as usize` turns an `f64` into a `usize`. For floats, `as` drops
the fraction, and it **saturates**: a negative number becomes 0, a
number too big becomes `usize::MAX`, and NaN becomes 0. It never panics
and never wraps around. That is different from `as` between integers,
which keeps only the low bits: `300_i32 as u8` is 44.

The positions say *where in the loop* a key was, not *which time
round*. A key held for more than a whole loop looks the same as a short
one: down at 2, up at 3 one loop later reads as one step long. And a
note across the end of the loop is cut there, instead of going on at
step 0. Both are left as [exercises](exercises.md).

### One undo step per take

`record` adds the note to the selected pattern through `edit_pattern`,
like every other change, so it is undoable and the audio thread hears
about it ([Telling the audio thread what changed](../program/sync.md)):

```rust
fn record(&mut self, held: Held, from: f64, until: f64) {
    let Some(length) = self.project.pattern(self.pattern).map(Pattern::length) else {
        return;
    };
    let note = recorded_note(held.pitch, held.velocity, from, until, length);
    self.edit_pattern("record notes", |pattern| {
        pattern.add_note(held.channel, note);
    });
}
```

`.map(Pattern::length)` passes a method as the function to `map`:
`Pattern::length` is a function that takes a `&Pattern`, exactly what
`map` hands over. `add_note` skips a note with the same pitch and start
as one already there, so playing the same thing the next time round
does not stack notes on top of each other.

A take can be dozens of notes, and Ctrl+Z should undo all of them, not
one at a time. The undo history already keeps a step open while the
mouse is held, so a drag is one step
([Undo and redo](../program/undo.md#several-clicks-one-step)).
Recording joins that rule:

```rust
pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
    let holding = ctx.input(|input| input.pointer.any_down());
    if !holding && !ctx.text_edit_focused() && !self.is_recording() {
        self.close_step();
    } else if self.step_unsynced {
        // ...
    }
}
```

While recording, the step stays open, and every note joins it. The take
ends, and the step with it, when playback stops, recording is disarmed,
the mode changes or another channel is chosen.

### Ending a take: before Stop

When playback stops, some keys may still be down. They are recorded as
if they came up right then, by `finish_recording`:

```rust
pub(super) fn finish_recording(&mut self) {
    let until = self.position();
    for index in 0..self.live.held.len() {
        let held = self.live.held[index];
        if let (Some(from), Some(until)) = (held.recorded_from, until) {
            self.record(held, from, until);
        }
        self.live.held[index].recorded_from = None;
    }
}
```

The keys stay in the list (they are still down, and still sound), but
their `recorded_from` is cleared, so their key-up later records
nothing. The loop goes by index, not `for held in &self.live.held`,
because `record` needs `&mut self` while the list is borrowed. `Held`
is `Copy`, so `let held = self.live.held[index];` copies it out, and the
borrow ends on that line.

The order in `halt`, which every way of stopping goes through, matters:

```rust
fn halt(&mut self) {
    if self.playing {
        // Before the audio thread forgets where it was.
        self.finish_recording();
    }
    self.playing = false;
    self.send(Command::Stop);
}
```

`finish_recording` needs the position, and `position()` gives `None`
once `self.playing` is false, or once the audio thread has stopped and
stored `NO_POSITION`. So the take is finished first, while both still
say "playing", and only then is Stop sent.

### Latency: what you hear is in the past

When you press a key "on the beat", you press it on the beat you
*heard*. But sound takes time to reach you. The audio thread makes each
block ahead of time and hands it to the sound card, which plays it a
little later; a sound card usually holds one or two blocks, so with
blocks of 512 frames at 48 kHz a sound comes out 11 to 21 ms after the
audio thread made it. By the time you hear step 4 begin, the audio
thread is already that far into it, and a key pressed exactly then is
stamped that late.

At 120 BPM a step lasts 125 ms, so 21 ms is a sixth of a step, and
rounding to the nearest step hides it, unless you were already a
little late. A MIDI keyboard adds almost nothing on top: its keys are
stamped on midir's thread as they arrive. The typing keyboard adds
more: egui reads the keyboard once per frame, so a typed key can wait up
to a frame before it is stamped, 17 ms at 60 frames a second, and much
more on a computer that draws its frames slowly.

The fix is to subtract the latency from each key's position before
rounding. cpal can tell how long it is: each call of the audio callback
gets an `OutputCallbackInfo` with two timestamps, when the callback ran
and when its sound will be played. That is one of the
[exercises](exercises.md).

## MIDI Out: an instrument with an outbox

A MIDI Out channel plays no sound. Its instrument turns notes into MIDI
messages for another synth. It is one more kind of `Instrument`, next to
the sampler, the Lineup Osc and the SoundFont player, so everything else
(the sequencer, live keys, volume, mute) works for it without changes.

The audio thread must not send the messages itself. Sending goes
through the operating system's MIDI layer, a system call that may take
any time, and the
[golden rule](../program/threads.md#the-golden-rule-of-the-audio-thread)
forbids waiting. So `MidiOut` collects its messages in an *outbox*, a
`Vec` with room reserved when it is created on the UI thread:

```rust
fn post(&mut self, message: MidiMessage) {
    let outbox = &mut self.state.outbox;
    if outbox.len() < outbox.capacity() {
        outbox.push(message);
    }
}
```

Checking `len() < capacity()` before pushing means the `Vec` never
grows, so it never allocates. The room is `OUTBOX_SIZE`, 256 messages
for one block of a few milliseconds; even a sixteen-note chord starting
while sixteen others end is only 32. If it were ever full, a message
would be lost rather than break the rule.

```rust
pub struct MidiOut {
    /// The MIDI channel, 0 to 15.
    channel: u8,
    /// The note every note goes out as, for a drum.
    pitch: Option<Note>,
    /// The General MIDI program the synth should play, if any.
    program: Option<u8>,
    /// Whether the program change still has to be sent: before the first
    /// note, and again after every stop, in case the synth's sound was
    /// changed on the synth itself in between.
    program_due: bool,
    /// Frames played so far: the clock that says when a note-off is due.
    now: u64,
    /// Boxed so an `Instrument` stays small, as the voices of the others are.
    state: Box<MidiOutState>,
}
```

A synth only ends a note when it is told to, so `MidiOut` must send a
note-off when each note's gate runs out. It keeps, for each of the 128
notes, an enum that says whether the note is sounding and until when:

```rust
enum Sounding {
    Off,
    /// Until this frame, when its note-off is due.
    Until(u64),
    /// Until `note_off`.
    Held,
}
```

`next_sample` makes no sound, but it counts the frames in `now`, which
is the clock for `Until`. Starting a note posts a program change first
when one is due, then the note-on:

```rust
let pitch = self.pitch.unwrap_or(pitch);
let slot = usize::from(pitch.midi());
// MIDI has one of each note per channel. If this one is already
// sounding, end it first, or the synth would hear "on, on, off"
// and the new note would stop at the old one's note-off.
if self.state.notes[slot] != Sounding::Off {
    self.post(self.note_off_message(pitch));
}
```

`self.pitch.unwrap_or(pitch)` reads "the drum's note if there is one,
otherwise the note itself", and writing `let pitch = ...` again
*shadows* the parameter: from that line on, `pitch` means the new value,
and the old one cannot be used by mistake. `note_off` ends only notes
that are `Held`: if the pattern plays a long C4 and you tap C4 on the
keyboard, letting go of your key must not end the pattern's note.

Once per block, the audio thread asks every instrument to hand over its
messages:

```rust
pub fn flush(&mut self, mut send: impl FnMut(MidiMessage)) {
    for index in 0..128 {
        if let Sounding::Until(end) = self.state.notes[index]
            && end <= self.now
        {
            self.post(self.note_off_message(Note::new(index as u8)));
            self.state.notes[index] = Sounding::Off;
        }
    }
    // `drain(..)` empties the `Vec` but keeps its memory for next time.
    for message in self.state.outbox.drain(..) {
        send(message);
    }
}
```

Note-offs are checked once per block, not on every frame, so they can
be a few milliseconds late, which nobody hears at the end of a note.
`drain(..)` takes every item out of the `Vec` in order, and the `Vec`
keeps its memory for the next block. `send` is an `impl FnMut`: the
instrument does not know or care where the messages go. The audio
thread passes a closure that puts them in the `MidiOutput` queue (or,
when that is full, a backlog reserved in advance, because a lost
note-off leaves a note sounding for ever; see
[Two threads](../program/threads.md#when-the-sound-card-goes-away)).
The tests pass one that pushes them into a plain `Vec` to look at.

The state lives in a `Box`. 128 `Sounding`s and 128 `Option<Origin>`s
take a few kilobytes, and an enum is as big as its biggest variant:
without the box, every `Instrument`, and every `Command` that carries
one through the ring buffer, would be that big. An engine test keeps a
command at 80 bytes or less, and it would fail without the box.

A SoundFont instrument is a `MidiOut` too, whose messages go to a
synthesizer inside the app instead of a port
([SoundFonts and General MIDI](soundfonts.md#a-midi-out-that-plays-inside-the-app)).

### The sending thread

The messages that MIDI Out channels leave in the `MidiOutput` queue are
sent by a thread of their own, named `midi-out`:

```rust
let sender = move || {
    while !messages.is_abandoned() {
        thread::sleep(SEND_INTERVAL);
        let mut output = lock(&output);
        while let Some(message) = messages.pop() {
            // With no port connected, the messages are dropped. A send
            // that fails (a cable pulled out) loses only that message.
            if let Some(connection) = output.as_mut() {
                let _ = connection.send(&message.bytes());
            }
        }
    }
    // ...
};
// If the thread cannot start, MIDI Out stays silent; nothing else breaks.
thread::Builder::new()
    .name("midi-out".to_string())
    .spawn(sender)
    .ok()
```

It sleeps for a millisecond (`SEND_INTERVAL`), sends whatever is
waiting, and starts again. A thread that waits for work would usually
sleep until it is woken, with a `Condvar`. But waking a thread is a
system call, which the audio thread must not make. Looking a thousand
times a second costs almost nothing, and adds at most a millisecond.

The lock is taken after the sleep, not before, so the UI thread never
waits for a sleeping thread. The `output` guard is dropped at the end
of each turn of the loop, which unlocks it.

`is_abandoned()` is true once the other end of the queue is gone: the
audio engine was dropped, as when the app closes or the sound card is
opened again. Then the loop ends, the thread sends All Notes Off for the
notes the old engine still owed, and finishes. Nothing has to tell it
to stop.

`thread::Builder` is the long way to start a thread. It can give the
thread a name, which shows in debuggers and in tools like `top -H`. And
its `spawn` returns a `Result` where `thread::spawn` would panic if the
system refused. `.ok()` turns it into an `Option<JoinHandle>`: if the
thread cannot start, MIDI Out stays silent and everything else works,
which is better than a crash at start. Without audio there is no
sending thread at all, since the notes are made on the audio thread,
and the MIDI menu says that nothing can be sent.

### Changing ports without hanging notes

The connection to the output port sits behind a `Mutex` because two
threads use it: the sending thread sends, and the UI thread replaces it
when you pick another port:

```rust
// Swap the connections while holding the lock, so the sending
// thread never sends half to one and half to the other.
let old = std::mem::replace(&mut *lock(&self.output), new);
if let Some(mut old) = old {
    // Notes that were playing there would hang: tell them all to stop.
    for channel in 0..16 {
        let _ = old.send(&all_notes_off(channel));
    }
    // `close` gives back the `MidiOutput`, which is not needed.
    old.close();
}
```

`&mut *lock(&self.output)` locks the mutex, follows the guard to the
`Option` inside (`*`), and borrows it mutably, all in one expression;
the guard is dropped at the end of the statement, so the lock is held
just long enough to swap. The sending thread then sends either to the
old port or the new one, never half to each.

The input side has its own version of this: when you pick another MIDI
keyboard, every key held on the old one is let go
(`release_all_keys`), because their key-ups would never arrive.

## Sending channels over MIDI

A MIDI Out channel, added with **Add channel > MIDI Out (external
synth)**, is the classic way: a channel of its own that sends its notes
to the synth. It is also a trap. Plug in a synth, pick it under
**MIDI > Output**, open a demo, press Play... and if nothing were done
about it, the synth would say nothing, because no demo has a MIDI Out
channel. So there are two more ways: every channel at once, and one
channel from its instrument menu.

### Every channel at once: routes

**MIDI > Send every channel** is on from the start. While an output is
connected, every channel plays on the synth instead of the speakers, on
the same MIDI channels that **File > Export song as MIDI…** writes, so a
song sounds the same played live as from its MIDI file. A *route* is
where one channel's notes go out:

```rust
pub type Route = (u8, Option<Note>);

pub fn routes(channels: &[Channel]) -> Vec<Option<Route>> {
    channels
        .iter()
        .zip(midi_channels(channels))
        .map(|(channel, midi_channel)| match &channel.kind {
            _ if channel.sent_over_midi().is_some() => None,
            ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
                Some((midi_channel, Some(general_midi_drum(*drum))))
            }
            _ => Some((midi_channel, None)),
        })
        .collect()
}
```

A route is a MIDI channel, and the drum note if there is one. `type`
only gives a long type a short name; a `Route` is still a plain tuple.
`zip` walks two lists side by side: each channel with its MIDI channel,
from `midi_channels` (below). The first arm, `_ if ...`, is a wildcard
with a guard: it matches any kind, but only for a channel already sent
over MIDI, which sends its notes itself and needs no route.

While every channel is sent, each sampler and synth is simply
*replaced* by a `MidiOut`, and put back when it stops:

```rust
pub fn to_midi(self, channel: u8, pitch: Option<Note>, program: Option<u8>) -> Self {
    match self {
        Instrument::MidiOut(_) => self,
        _ => Instrument::MidiOut(
            MidiOut::new(channel)
                .playing_as(pitch)
                .with_program(program),
        ),
    }
}
```

`to_midi` takes `self` by value: the old instrument is *moved* into the
function and dropped there, and the caller gets the new one back. A MIDI
Out already sends its notes, so it comes back unchanged.

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
when they differ does a channel get a new instrument, a MIDI Out or its
own, through `Command::SetInstrument`, and only the channels whose route
changed.

`routes_sent` is an `Option<Vec<...>>`: `None` means "not known". When
channels are added, removed or reloaded, the audio thread gets plain
instruments for them, and the ones that moved keep whatever they had.
If any of them was a MIDI Out in place of its own instrument, the app
forgets what it sent, and the next frame sends every channel again.
That is rare (it takes a change to the rack), and it cannot go wrong.

### One channel from its instrument menu

Often you want something in between: the bass on a hardware synth and
the rest of the band in the app, or the drums on a drum machine. So
every instrument menu, in the mixer and in the channel rack, starts
with **Send over MIDI**: a MIDI channel from 1 to 16, and the output.

![The Keys' instrument menu in the mixer: Send over MIDI, with its sixteen MIDI channels and the output (this machine has none), above the General MIDI instruments.](../images/step-9-midi-out-menu.png)

A synth sent over MIDI **becomes** a MIDI Out channel, keeping its
instrument:

```rust
pub fn send_over_midi(&mut self, midi_channel: u8) {
    let midi_channel = midi_channel.min(15);
    match self.kind {
        ChannelKind::Sampler(SampleSource::Builtin(_)) => {
            self.sent_on = Some(midi_channel);
            return;
        }
        ChannelKind::Sampler(SampleSource::File(_)) => return,
        _ => {}
    }
    let plain = match self.kind {
        ChannelKind::Synth => self.name == Channel::synth().name,
        ChannelKind::MidiOut { channel } => self.name == Channel::midi_out(channel).name,
        _ => false,
    };
    if plain {
        self.name = Channel::midi_out(midi_channel).name;
    }
    self.kind = ChannelKind::MidiOut {
        channel: midi_channel,
    };
}
```

A name that only said what played it, "Lineup Osc" or "MIDI Out 2",
follows the change and becomes "MIDI Out 3". A name you gave it, such as
"Bass", stays. `play_here` does the opposite. A sample from a file has
no note a synth could play in its place, so it cannot be sent.

A **drum** does not change kind. It gets a field, `sent_on`, with the
MIDI channel, and stays a `Sampler` playing its built-in drum. That
matters to the band: styles find the kick, the snare and the hats by
what they are (`Sampler(Builtin(Drum::Kick))`), to write their beats on
them. A kick turned into a MIDI Out channel would no longer be found,
and the next style would add a second kick. For the same reason the
styles find a "Bass" or "Keys" sent over MIDI by its name, as the bass
and the keys ([The library](../band/library.md)). On the audio thread
`Instrument::for_channel` builds a `MidiOut` for a drum with `sent_on`,
playing its own note of a General MIDI drum set
([Instruments](instruments.md)), so the drum part, the band
and the styles stay exactly as they were; only the sound goes elsewhere.

`Channel::sent_over_midi` answers "is it sent, and on which channel?"
for both kinds at once, so the rest of the code does not have to know
the difference.

The mixer names the drum a channel sends, from the General MIDI drum
table in `general_midi.rs`:

```rust
/// The drum General MIDI plays for `note` on channel 10, such as "Bass
/// Drum 1" for 36, if the drum set has one there.
pub fn drum_name(note: u8) -> Option<&'static str> {
    DRUMS
        .get(usize::from(note.checked_sub(FIRST_DRUM)?))
        .copied()
}
```

Two kinds of "maybe" meet here. `checked_sub` returns `None` instead of
wrapping around when the note is below 35 (a `u8` can't go below 0), and
the `?` returns that `None` from the function at once. `get` returns
`None` past the end of the table, for notes above 81. `copied` turns the
`Option<&&str>` that `get` gives into an `Option<&str>`.

### The menu needs the list of outputs

The instrument menus are drawn by functions in `channel_rack.rs`, which
know a channel and nothing about the app's MIDI connections. Passing
those down through the rack, the mixer and the style editor would add a
parameter to many functions. Instead the app puts the list of outputs
into egui's memory once a frame, and the menu reads it there:

```rust
pub fn set_midi_outputs(ctx: &egui::Context, outputs: MidiOutputs) {
    ctx.data_mut(|data| data.insert_temp(midi_outputs_id(), outputs));
}
```

```rust
let outputs: MidiOutputs = ui
    .ctx()
    .data(|data| data.get_temp(midi_outputs_id()))
    .unwrap_or_default();
```

egui's memory can hold a value of any type that is `Clone`, `Send`,
`Sync` and `'static`, one per `Id` and type. Values stored with
`insert_temp` are not saved when the app closes, which is right for a
list of ports that may be different tomorrow. The menu does not change
the connection itself: it pushes a `RackAction::ConnectMidiOutput`, which
the app carries out, as every other click in the rack.

A test of this menu, `the_mixer_sends_a_channel_over_midi_and_back`,
shows why a test must ask the machine what it has. It opens **Send over
MIDI** and checks the outputs listed. Expecting "No outputs found"
passes on Linux and macOS test machines, which have no MIDI output, and
fails on Windows, which always has one: its built-in "Microsoft GS
Wavetable Synth". So the test reads the outputs the app found, and
expects that text only when there are none. It is the kind of
difference running the tests on three systems is there to catch
([Three systems and continuous integration](../shipping/ci.md)).

### Which MIDI channel each channel uses

A program reading a MIDI file, or a synth hearing the app, uses the MIDI
channel to pick the instrument. General MIDI, the standard most synths
follow, keeps **channel 10** (9, counted from 0) for drums, with a fixed
note for each drum: 36 is a kick, 38 a snare, 39 a clap, 42 a closed
hi-hat. One function decides the MIDI channel of every channel, for the
MIDI file and for the routes alike:

```rust
pub fn midi_channels(channels: &[Channel]) -> Vec<u8> {
    let taken = |number: u8| {
        channels
            .iter()
            .any(|channel| channel.sent_over_midi() == Some(number))
    };
    let free: Vec<u8> = (0..16u8)
        .filter(|&number| number != DRUM_CHANNEL && !taken(number))
        .collect();
    let mut melodic = if free.is_empty() {
        (0..16u8).filter(|&number| number != DRUM_CHANNEL).collect()
    } else {
        free
    }
    .into_iter()
    .cycle();
    channels
        .iter()
        .map(|channel| match channel.kind {
            ChannelKind::Sampler(SampleSource::Builtin(_)) => {
                channel.sent_on.unwrap_or(DRUM_CHANNEL)
            }
            ChannelKind::MidiOut { channel } => channel,
            _ => melodic.next().unwrap_or(0),
        })
        .collect()
}
```

The built-in drums share channel 10, unless one was sent elsewhere.
Channels sent over MIDI keep the channel they were given. Every other
channel takes the next *free* one: not 10, and not one a channel sent
over MIDI already plays on, or the two would share one sound on the
synth and cut each other's notes short. `cycle` repeats the free
channels for ever, and the iterator is created once, outside the
closure, which borrows it mutably: each `next()` moves it on, so the
channels get 1, 2, 3… in turn, and with more channels than free ones,
they start sharing.

## Writing a MIDI file

A Standard MIDI File holds the same messages, with times. It holds
notes, not sound: "C4 starts here, ends there, on channel 1". The program
that opens it chooses the instruments. That makes it the common language
of music programs, and the way to take a Ferrous Lineup song into
another studio. `song_to_midi` writes the song as the playlist plays it,
and `pattern_to_midi` one pattern, once.

The file is a list of *chunks*. Each one starts with four letters and
its length:

```text
  MThd  length 6  format 1  3 tracks  96 ticks per beat     the header
  MTrk  length …  name "Ferrous Lineup", tempo, time signature, key, end
  MTrk  length …  name "Lineup Osc", a program change, the notes, end
  MTrk  length …  name "Kick", the notes of the kick, end
```

This is **format 1**: several tracks played together. The first track
holds only the tempo, the time signature and the key, and every channel
of the project gets a track of its own, named after it, that starts with
a program change when the channel has a General MIDI instrument. A
channel with no notes, or one that is not heard (muted, or silenced by
another's solo), gets no track.

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

### Ticks and delta times

Time in a MIDI file is counted in *ticks*, and the header says how many
ticks make a beat. Ferrous Lineup writes 96, the same ticks its notes
are placed in: a step, a sixteenth note, is 24 ticks, and a note's
`nudge` and `trim` count the ticks it starts after or ends before its
steps ([The project](../program/project.md)). So a note between two
steps is written exactly where it plays.

```rust
pub const TICKS_PER_BEAT: u16 = 96;
pub const TICKS_PER_STEP: u32 = crate::pattern::TICKS_PER_STEP as u32;

// A check made while compiling: if someone changed one of the two
// numbers and not the other, the build would stop here.
const _: () = assert!(TICKS_PER_BEAT as u32 == TICKS_PER_STEP * STEPS_PER_BEAT);
```

`const _: () = assert!(...)` is a constant with no name (`_`) whose
value is worked out while compiling. `assert!` works in a `const` since
Rust 1.57, and a failed assertion there is a compile error, not a panic
at run time. It costs nothing in the program, and it turns "remember to
change both" into something the compiler checks.

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
fit in an array, and nothing is allocated. The test
`variable_length_numbers` feeds it the examples printed in the MIDI file
specification.

A `TrackWriter` keeps the tick of the last event, so the rest of the
code can think in absolute ticks:

```rust
#[derive(Default)]
struct TrackWriter {
    data: Vec<u8>,
    last_tick: u32,
}

impl TrackWriter {
    /// Writes the time since the last event. Events must come in order.
    fn delta(&mut self, tick: u32) {
        let delta = tick.saturating_sub(self.last_tick);
        write_variable_length(&mut self.data, delta);
        self.last_tick = self.last_tick.max(tick);
    }

    // ...

    /// Ends the track at `end` (or at its last event, if that is later)
    /// and wraps it in its chunk header.
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

### Meta events: names, tempo, time and key

Events that are not notes start with `0xFF`, a kind, and a length:

| Bytes | Means |
|-------|-------|
| `FF 03 len text` | the track's name |
| `FF 51 03 tt tt tt` | the tempo, as microseconds per beat |
| `FF 58 04 nn dd cc 08` | the time signature: 4/4 is `04 02 18 08` |
| `FF 59 02 sf mi` | the key signature: sharps (negative for flats), major or minor |
| `FF 7F len data` | data for one program only (sequencer-specific) |
| `FF 2F 00` | the end of the track |

The tempo is the odd one: not beats per minute, but how long one beat
lasts, in microseconds. A song whose tempo changes gets a tempo event on
every step where the played tempo changes, and the song's time
signatures go in at every change; both are in
[Tempo, meter, swing and feel](tempo.md#in-files-and-midi-files).

The key signature says how many sharps or flats, and whether the key is
minor. `Scale::key_signature` works it out by going round the circle of
fifths ([Scales, modes and keys](../theory/scales.md)). A key
signature only knows major and minor, so a mode takes the signature of
the major key it comes from: D Dorian has none, like C major, and E
Dorian the two sharps of D major. That loses something: a file that says
"two sharps, major" cannot say whether it means D major or E Dorian. So
next to each key signature the file gets the exact key, in a
**sequencer-specific** event, the one place the standard leaves for a
program's own data:

```rust
/// The start of our own event: 0x7D is the maker's number set aside for
/// programs that are not sold, then our name, so another program's
/// 0x7D event is never mistaken for ours.
const SCALE_EVENT_START: &[u8] = b"\x7DFerrous Lineup key ";

pub fn scale_event(scale: Scale) -> Vec<u8> {
    let mut data = SCALE_EVENT_START.to_vec();
    data.extend_from_slice(ron::to_string(&scale).unwrap_or_default().as_bytes());
    data
}

/// The key a sequencer-specific event holds, if [`scale_event`] wrote it.
pub fn scale_from_event(data: &[u8]) -> Option<Scale> {
    let key = data.strip_prefix(SCALE_EVENT_START)?;
    let text = std::str::from_utf8(key).ok()?;
    ron::from_str(text).ok()
}
```

The key is written the way a project file writes it, in RON, so the
event reads `7D "Ferrous Lineup key (root:E,kind:Dorian)"`, and the code
that saves projects reads it back ([Saving](../program/saving.md)).
`\x7D` in a byte string is the byte 0x7D. `strip_prefix` gives the rest
of the slice after the start, or `None` when the slice does not start
that way; the `?`s make every step that can fail end the function with
`None`. Other programs skip the event, and Ferrous Lineup reads back E
Dorian, C♯ harmonic minor or a twelve-tone row exactly. A row or a set
has no key signature, so for those this event is all there is.

### Notes, drums and order

Each note becomes two events, its note-on and its note-off. The notes
are the ones playback plays: `played_notes` gives them with the song's
swing, `song_steps_of` every song step where a piece of the song plays
that step (once per repeat), and a note running past the end of its
pattern, or of the piece being played, is cut there, exactly as the
audio thread plays it:

```rust
let room = (length - note.start).min(play.end() - song_step);
let step = song_step as u32 * TICKS_PER_STEP;
let on = step + u32::from(note.nudge);
let off = if note.length <= room {
    on + note.ticks() as u32
} else {
    step + room as u32 * TICKS_PER_STEP
};
let pitch = drum.map_or(note.pitch, general_midi_drum);
```

`map_or(note.pitch, general_midi_drum)` gives the note's own pitch for
a melodic channel, and for a built-in drum, its General MIDI note: a
drum's steps are all written on C4, which a General MIDI drum set plays
as a bongo. Passing the function `general_midi_drum` itself, by name,
works because its type, `fn(Drum) -> Note`, is what `map_or` asks for.

Then the events are sorted by time. At the same tick, note-offs must
come before note-ons: if a note ends exactly where the same note starts
again, "on, off" would end the new note at once, and only "off, on" is
heard twice.

```rust
events.sort_by_key(|&(tick, message)| (tick, matches!(message, MidiMessage::NoteOn { .. })));
```

The key is a tuple, `(tick, is_note_on)`, and tuples compare item by
item. For the same tick, `false < true`, so note-offs win. `sort_by_key`
is a *stable* sort: events with equal keys keep their order, so the
notes of a chord stay in the order they were written. The test
`a_repeated_note_ends_before_it_starts_again` checks the order at the
tick where a C4 ends and starts again.

In the app, **File > Export song as MIDI…** and **Export pattern as
MIDI…** only ask where to save, then write the bytes with
`std::fs::write`. A pattern's name could hold a `/`, which a file name
cannot, so the name offered for a pattern replaces it.

## Reading a MIDI file

**File > Import MIDI file…**, opening a `.mid` file from the command
line, or dropping one on the window turns a MIDI file into a new
project, ready to play. MIDI files are the easiest way to get real music
into the app: almost every piece ever written can be found as one,
often typed in from the score, sometimes played in on a keyboard.

Reading is writing walked backwards, with one difference that changes
how the code is written: **a file we read can be wrong.** It may be cut
short by a broken download, it may claim a chunk is longer than it is,
it may not be a MIDI file at all. Whatever the bytes are, the program
must not crash.

### A cursor that cannot fall off the end

The reader walks the bytes with a small *cursor*: a slice and a position
in it.

```rust
struct Bytes<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Bytes<'a> {
    // ...

    fn byte(&mut self) -> Option<u8> {
        let byte = *self.data.get(self.at)?;
        self.at += 1;
        Some(byte)
    }

    /// The next `count` bytes, or `None` if there are fewer left.
    fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(count)?;
        let slice = self.data.get(self.at..end)?;
        self.at = end;
        Some(slice)
    }
}
```

Two Rust ideas make this safe:

- **`'a` is a lifetime.** `Bytes<'a>` borrows the file's bytes; it does
  not copy them. The lifetime says "a `Bytes` cannot outlive the bytes
  it points into", and the compiler checks that. `take` returns
  `&'a [u8]`: a slice of the *original* bytes, which stays valid even
  after the cursor moves on.
- **`get` instead of indexing.** `data[at]` panics when `at` is past the
  end; `data.get(at)` returns `None`. The `?` after it returns that
  `None` from `byte` straight away. `checked_add` does the same for a
  length so large that adding it would overflow. So every read either
  gives a value or `None`, and the caller decides what a missing value
  means.

A delta time is read back by gluing the groups of seven bits together:

```rust
fn variable_length(&mut self) -> Option<u32> {
    let mut value = 0u32;
    for _ in 0..4 {
        let byte = self.byte()?;
        value = (value << 7) | u32::from(byte & 0x7F);
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}
```

`81 00` is `1 << 7 | 0` = 128. The standard allows four bytes at most;
a fifth would mean the file is damaged, so after four the loop gives
up.

### The header and the chunks

`read` checks for `MThd`, reads the format and the *division*, and then
reads every chunk to the end of the file:

```rust
while let Some(id) = file.take(4) {
    let Some(length) = file.u32() else {
        damaged = true;
        break;
    };
    let length = length as usize;
    // A chunk that claims to be longer than what is left of the file
    // was cut short: read what there is.
    let data = file.take(length.min(file.remaining()));
    let data = data.unwrap_or_default();
    if data.len() < length {
        damaged = true;
    }
    // Chunks of other kinds may be added by future versions of the
    // standard, or by one program for itself: they are skipped.
    if id == b"MTrk" {
        let (track, complete) = read_track(data);
        damaged |= !complete;
        tracks.push(track);
    }
}
```

The header also says how many tracks there are, but some programs write
that number wrong, so the chunks themselves are trusted instead. Real
files are often slightly broken, and every program that opens them is
forgiving about it. So is this one: a track cut short keeps the events
read so far, a chunk longer than the file is read as far as it goes,
unknown chunks are skipped. But the result says so: `MidiFile::damaged`
is set, and the notice after the import ends with "The file is damaged:
everything before the damage was read". `damaged |= !complete` is the
boolean "or" written as an assignment, like `+=`: once damaged, always
damaged.

Two more things a reader meets in the wild:

- **RMID files** (`.rmi`). Windows used to wrap MIDI files in RIFF, the
  container of WAV files: `RIFF`, a size, `RMID`, then chunks. The MIDI
  file is the chunk called `data`. Unlike MIDI, RIFF writes its numbers
  least significant byte first, so `unwrap_riff` uses
  `u32::from_le_bytes` where MIDI uses `u32::from_be_bytes`, and a chunk
  of odd length is followed by a byte of padding.
- **Ticks per second.** The division usually counts ticks per beat (480
  and 960 are common). With its top bit set it counts ticks per frame of
  film or video instead (24, 25, 29.97 or 30 frames a second), as
  `Timing::PerSecond`. Such a file has no beats at all; the import counts
  its time in beats of 120 BPM, so it plays at the right speed.

### Running status

A track of a piano piece is mostly note on, note off, note on... To save
space, a file may leave out the status byte when it is the same as the
one before. The reader can always tell, because a status byte has its
top bit set and a data byte does not:

```rust
let (status, first_data) = if first & 0x80 != 0 {
    (first, bytes.byte())
} else {
    (running.unwrap_or(0), Some(first))
};
```

So `90 3C 64 00 40 64` is two notes on: C4 and E4, both at velocity 100.
Program changes and channel pressure have one data byte, the rest of the
channel messages two. A meta event (`FF`) and a system exclusive message
(`F0` or `F7`, settings for one maker's synths, which are skipped) carry
their own length. Any other system message only travels down cables,
never in files, so meeting one means the track is damaged.

### Events as an enum

Each event the import needs becomes a variant of one enum, with its data
inside:

```rust
pub enum EventKind {
    NoteOn { channel: u8, key: u8, velocity: u8 },
    NoteOff { channel: u8, key: u8 },
    Controller { channel: u8, number: u8, value: u8 },
    Program { channel: u8, program: u8 },
    Tempo(u32),
    TimeSignature { beats: u8, unit: u8 },
    KeySignature { sharps: i8, minor: bool },
    ExactKey(Scale),
    TrackName(String),
    InstrumentName(String),
}
```

Everything else (pitch bend, aftertouch, lyrics) is read past and
dropped. A note-on with velocity 0 becomes a `NoteOff`, as on a cable.
`meta_event` uses slice patterns to check that a meta event's data has
the right shape before trusting it:

```rust
// Sharps (negative for flats), then 0 for major or 1 for minor.
(KEY_SIGNATURE, &[sharps, mode]) => {
    let sharps = sharps as i8;
    ((-7..=7).contains(&sharps) && mode <= 1).then_some(EventKind::KeySignature {
        sharps,
        minor: mode == 1,
    })
}
```

A key signature is exactly two bytes. `&[sharps, mode]` only matches a
slice of length two and names its bytes; a key signature of three bytes,
or one claiming 9 sharps, is not a key signature and is skipped.
`sharps as i8` reads the same eight bits as a signed number, so the byte
`0xFE` is -2: two flats. `bool::then_some` gives `Some(value)` when the
`bool` is true and `None` otherwise.

The names of tracks and instruments are text, and the standard says
nothing about how letters beyond plain English are written: newer
programs use UTF-8, older ones Latin-1, one byte per letter. `text`
tries UTF-8 first, since Latin-1 text is almost never valid UTF-8, and
otherwise turns each byte into the `char` with that number, which is
exactly Latin-1.

## Importing: from events to a project

`import(&file, title)` turns a `MidiFile` into a `Project` and a
`Report` of what it did. The report becomes the notice in the status
bar, such as *"Clementi: 2 channels, 812 notes, 38 bars, 4/4, 120 BPM,
C major"*.

In the app, reading the file, reading the MIDI and importing are three
steps that can each fail with a different error type. They are chained
into one `Result`:

```rust
let imported = std::fs::read(path)
    .map_err(|err| err.to_string())
    .and_then(|bytes| midi_file::read::read(&bytes).map_err(|err| err.to_string()))
    .and_then(|file| {
        midi_file::import::import(&file, &title).map_err(|err| err.to_string())
    });
```

`and_then` runs the next step only if the one before succeeded, and
passes its value on. `and_then` keeps the error type, so each step turns
its own error into a `String` with `map_err` first. One `match` at the
end shows the notice or the error. Reading takes a few thousandths of a
second, so unlike [importing a recording](../theory/listen.md), it is
done straight away on the UI thread.

### Ticks into our ticks

A file counts in its own ticks, often 480 or 960 to the beat; a project
counts 96. `Clock` turns one into the other, to the nearest tick:

```rust
Timing::PerBeat(per_beat) => {
    let per_beat = u128::from(per_beat);
    (u128::from(tick) * u128::from(TICKS_PER_BEAT) + per_beat / 2) / per_beat
}
```

Adding half the divisor before dividing rounds to the nearest integer
instead of down. The sum is done in `u128` because a damaged file can
claim a tick near `u64::MAX`, and multiplying that by 96 would overflow
a `u64`; the result is then capped, far past the end of any song.

### Notes between steps

A pattern counts time in steps. Rounding every note to the nearest step
would ruin a lot of music:

- **Triplets** split a beat in three. Eighth-note triplets start at
  0, ⅓ and ⅔ of a beat: none of those is a sixteenth.
- **Swing** plays the second eighth of each beat late.
- A file **played in** on a keyboard has every note a little early or
  late, and that is a large part of what makes it sound alive.

So a note keeps its exact ticks in `nudge` and `trim`. Why 24 to the
step? Because 24 divides by 2, 3, 4, 6, 8 and 12. An eighth-note triplet
is 32 ticks long, a sixteenth triplet 16: both whole numbers, so
triplets land *exactly*. `NoteEvent::exact` builds a note from its exact
ticks:

```rust
pub fn exact(pitch: Note, start: usize, ticks: usize) -> Self {
    let end = start + ticks.max(1);
    let first = start / TICKS_PER_STEP;
    let last = end.div_ceil(TICKS_PER_STEP);
    NoteEvent {
        nudge: (start % TICKS_PER_STEP) as u8,
        trim: (last * TICKS_PER_STEP - end) as u8,
        ..NoteEvent::new(pitch, first, last - first)
    }
}
```

The triplet on tick 32 is in step 1 (`32 / 24`), nudged by 8 ticks
(`32 % 24`). `div_ceil` divides rounding up, so `last` is the step after
the one the note ends in. The piano roll draws a nudged note where it
really starts, the audio thread plays it there
([The sequencer](sequencer.md#how-long-a-note-is-held)), and
**Tools > Quantize to the steps** moves notes onto the grid when you want
them there.

![A file whose notes are a little off the grid, as when a piece is played in on a keyboard: in the piano roll the notes start a little after or before the step lines, and the velocity bars below are all of different heights.](../images/step-8-midi-played-in.png)

### Pairing note on with note off

A file does not store notes. It stores *key down* and *key up*, and the
import has to pair them. While a track is read, each MIDI channel keeps
the notes still sounding on each key:

```rust
struct ChannelState {
    /// For each key, the notes sounding on it, oldest first. There is
    /// usually one at most, but a file can strike a key again before
    /// letting it go; each note off then ends the oldest note.
    open: Vec<Vec<Open>>,
    pedal: bool,
    notes: Vec<Played>,
}
```

A key can have two notes open: two voices of a fugue on the same note,
or a round where the parts overlap. Then each key up ends the **oldest**
note on that key, first in, first out. Ending the newest one instead
would cut notes short wherever parts overlap: the rounds among the demos
would not come back as they were written.

Two more things the import does like a piano would:

- **The sustain pedal** (controller 64). While it is down, a key let go
  keeps sounding (`held_by_pedal`); the note ends when the pedal comes
  up, or when the same key is struck again. Without this, a piano piece
  played in with the pedal sounds dry and choppy, because the player's
  fingers left the keys long before the sound was meant to stop.
- **A note let go on the tick it started** (a grace note, a drum hit)
  still lasts one tick, so it is heard.

Lifting the pedal splits each key's list in two:

```rust
let (held, sounding): (Vec<Open>, Vec<Open>) =
    open.drain(..).partition(|note| note.held_by_pedal);
*open = sounding;
for note in held {
    self.end(key, note, tick);
}
```

`partition` sends each item to one of two collections, depending on the
closure, and keeps their order. `drain(..)` empties the list as it goes,
so the notes are moved, not copied, and `*open = sounding` puts the ones
still sounding back. Keys still down when a track ends are let go
there.

A note struck twice on the same tick by mistake is one note: the notes
are sorted with `sort_by_key(|note| (note.start, note.pitch,
std::cmp::Reverse(note.end)))`, and `dedup_by_key` keeps the first of
each `(start, pitch)`. `Reverse` turns an order round, so the longer of
the two comes first and is the one kept.

### Tracks, channels and drums

Each MIDI channel of each track that plays notes becomes a channel of
the rack. Most files are **format 1**, with one track per instrument,
so each track becomes a channel, named after the track ("Violin I"), or
after its General MIDI instrument when the track has no name. A
**format 0** file puts everything in one track, with the instruments on
different MIDI channels; there, each MIDI channel becomes a rack
channel. A rack holds 32 channels; when a file has more parts than fit
next to its drums, its parts are joined by MIDI channel.

The **program change** that chose each part's instrument is kept as the
channel's `program` ([SoundFonts and General MIDI](soundfonts.md)),
and its **volume** (controller 7) sets the channel's volume, so a quiet
accompaniment stays quiet. Programs and volumes are sometimes set on
another track than the notes, so the first one set on each channel
anywhere in the file is the fallback:

```rust
EventKind::Program { channel, program } => {
    programs[usize::from(channel)].get_or_insert(program);
}
```

`Option::get_or_insert` sets the value only if there is none yet, so the
first program change wins without an `if`.

MIDI channel 10 is the drum channel: each key is a drum sound. The
import gives each built-in drum the file uses a channel of its own, as
in a new project, so the drums show up in the channel rack as a step
sequencer, each hit one step long. Sounds the built-in drums do not have
go to the closest one (`general_midi_drum_sound`: a triangle to the
ride, a tambourine to the shaker, bongos to the conga); a sound with
nothing close, such as a whistle, is left out, and the notice counts it.

A few files on the internet have millions of notes, written as a stunt,
and would make the piano roll too slow to use. The import keeps the
50,000 earliest (`MAX_NOTES`). `keep_first_notes` sorts the start ticks,
takes the start of the last note that fits as the cut, and uses
`partition_point` (a binary search for the first item that fails a test)
to count how many notes on the cut tick still fit.

### Tempo, time and key

Tempo changes, time signatures and key signatures apply to the whole
song, whichever track holds them. One generic function gathers events
of one kind from every track:

```rust
fn song_events<T>(tracks: &[Track], pick: impl Fn(&EventKind) -> Option<T>) -> Vec<(u64, T)> {
    let mut events: Vec<(u64, T)> = tracks
        .iter()
        .flat_map(|track| &track.events)
        .filter_map(|event| pick(&event.kind).map(|value| (event.tick, value)))
        .collect();
    // A stable sort keeps events on the same tick in the order they were
    // read, so the last one written wins.
    events.sort_by_key(|&(tick, _)| tick);
    events
}
```

`pick` says which events to keep and what to take from them, so the
same function serves tempos (`u32`), time signatures and keys. Each
kind then goes into a `BTreeMap` by step, where a later event on the
same step replaces an earlier one, as on playback. Every tempo change
becomes a tempo marker, every time signature a time marker
([Tempo, meter, swing and feel](tempo.md)), and `tidy` leaves
out the markers that change nothing: files often repeat the time
signature at the start of every section. A time signature the app
cannot count, with bars that are not a whole number of sixteenths, is
reduced when it can be: 6/32 becomes 3/16.

The key comes from the key signatures, as written: two flats and minor
is G minor, and the signature's sharps or flats decide the spelling, so
six flats are G♭ major and six sharps F♯ major. Every later signature
that names a different key becomes a key marker on the playlist. The
exact key from a file Ferrous Lineup wrote wins over the signature next
to it. Two cases are left to the notes:

- **A file with no key signature at all.**
- **A file that says C major but is not.** Many programs write "no
  sharps, major" into every file, whether or not anyone chose a key. So
  when every key signature in the file says C major, the import checks
  it against the notes, and if another key fits *clearly* better (by `CLEARLY_BETTER`,
  0.1 of correlation) it takes that one, and the notice says "worked out
  from the notes".

How notes decide a key, with key profiles and the Krumhansl-Schmuckler
method, is in [Hearing harmony](../theory/hearing.md). The import adds up how
long each of the twelve pitch classes sounds, and asks `key_fits` which
key fits best.

### Cutting the song into patterns

A pattern holds at most 256 steps (16 bars of 4/4), and a song may last
200 bars. So the import cuts the song into patterns at **bar lines**, at
most 16 bars each, and lays them one after another on the playlist.

Where to cut? A note that sounds across the cut has to be split in two,
which is harmless for the sound but ugly to edit. So among the bar
lines in the second half of each stretch, `chunks` picks the one the
fewest notes sound across, the latest of those. To count them for every
step at once, it uses a **difference array**:

```rust
// How many notes cross each step's start: a difference array, +1
// where a note starts crossing and -1 after it stops.
let mut change = vec![0i64; length + 2];
for note in notes {
    let first = note.start / TICKS_PER_STEP + 1;
    let last = note.end.div_ceil(TICKS_PER_STEP).saturating_sub(1);
    if first <= last && first <= length {
        change[first] += 1;
        change[(last + 1).min(length + 1)] -= 1;
    }
}
let mut crossing = Vec::with_capacity(length + 1);
let mut count = 0;
for &delta in change.iter().take(length + 1) {
    count += delta;
    crossing.push(count);
}
```

For each note, +1 on the first step boundary it crosses and -1 after the
last; adding the numbers up from the left gives, at every step, how many
notes are sounding across its start. That is one pass over the notes and
one over the steps, instead of checking every note at every step. The
choice is then `(earliest..=last).rev().min_by_key(...)`: `min_by_key`
returns the *first* of several equal minimums, and walking the bars
backwards makes the first one the latest bar.

`place_patterns` fills the patterns, splitting the notes that cross a
cut. A stretch that comes back **note for note** (a repeated verse, a
da capo) plays the same pattern again instead of a copy, as you would
build the song by hand. Each pattern is named after its bars, such as
"Bars 1–16".

![Mozart's Eine kleine Nachtmusik, imported from a MIDI file, in the piano roll. Above the notes, the Chords lane names each chord: G, D, G, D, G, C, A, D... The key box says G major. Below, the velocity lane shows how hard each note was played.](../images/step-8-midi-mozart.png)

Because the notes land in ordinary patterns, everything that already
understands notes works on them: the piano roll's **Chords** lane names
the chord at every step, and the harmony keys light up the chord
sounding now while the song plays.

## Testing it

- `midi.rs`: `parse` on note-ons, note-offs, a silent note-on, and
  messages it must ignore (a sustain pedal, a pitch bend, a clock tick,
  a message too short, a data byte out of range), and every message to
  bytes and back.
- `record.rs`: the quantizing rules, with made-up positions.
- `engine.rs`: `keys_play_the_live_channel_until_they_come_up`,
  `a_key_comes_up_on_the_channel_it_went_down_on`,
  `a_key_up_leaves_the_sequencers_notes_alone`,
  `keys_from_a_midi_keyboard_skip_the_command_queue`,
  `removing_channels_lets_go_of_their_keys`,
  `midi_out_channels_send_their_notes`,
  `removing_a_midi_out_ends_its_notes`, and the two tests that a MIDI
  message waits for room instead of being dropped, in order.
- `instrument.rs`: note-offs after the gate, a repeated note, the
  program change before the first note and after every stop, a synth or
  drum sent to MIDI that is silent and sends its notes.
- `midi_file.rs`: the variable-length numbers from the specification,
  then whole files, read back by a small reader written in the test
  module: tracks, program changes, tempo changes, time and key
  signatures, the exact key, drums on channel 10, clips and repeats, a
  repeated note, a section played once, and the MIDI channels of every
  kind of channel.
- `read.rs`: running status, meta events, system exclusive, an RMID
  file, ticks per second, and `a_file_cut_short_keeps_what_was_read`,
  which cuts a file at *every* length from 0 bytes to the whole file and
  reads each one without a panic.
- `import.rs`: small files built by hand for what a demo never has:
  triplets, the sustain pedal, a format 0 file, drums, keys with and
  without signatures, a long song cut at bar lines, a stretch that comes
  back, a file without notes.

The best test of an import is an export.
`every_demo_comes_back_note_for_note` exports each song demo as a MIDI
file, imports it, exports the result again, and compares the two files
event by event: every note on with its key and velocity, every note
off, every tempo change, time signature, key signature and program
change, each at the same tick. It also checks the imported project has
exactly the demo's key and key markers.

A reader written by the same person who wrote the writer can share its
mistakes, though. So it is worth opening exported files in a program
that knows nothing about Ferrous Lineup (Python's `mido` library reads
one in a few lines), and files from other programs in the app.

### Testing without a synth

A test can check the bytes an instrument would send, but not that they
reach a port. For that, Linux has the ALSA sequencer, and two kernel
modules that make pretend MIDI devices: `snd-seq-dummy` (a "Midi
Through" port that passes everything on) and `snd-virmidi` (ports that
look like a USB keyboard or synth). `aseqdump` prints every message that
reaches a port:

```sh
sudo modprobe snd-seq-dummy snd-virmidi
aseqdump -p "Midi Through"
```

Pick **Midi Through** under **MIDI > Output**, press Play, and each note
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

![The MIDI menu: the inputs and outputs of a machine with virtual MIDI ports, Send every channel, Typing keyboard to piano and Accompaniment follows chords.](../images/step-8-midi-out-menu.png)
