# Live keys: threads in and out

Files: [`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/midi.rs),
[`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/app/live.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/engine.rs)
and [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/instrument.rs)

Until now, every note came from the sequencer, which lives on the audio
thread and knows each note's length before it starts. A key on a
keyboard is different in two ways: it comes from outside, on a thread
Ferrous Lineup does not own, and nobody knows how long it will be held.
This chapter follows a key from the cable to the speaker, and a MIDI Out
note from the pattern to the cable.

## Why keys skip the UI

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

## One producer per queue: `LiveInput`

The command queue from [step 1](../step-1/audio-engine.md#sending-commands-with-a-ring-buffer)
is a ring buffer from the `rtrb` crate, and its `Producer` belongs to
the `AudioEngine`, on the UI thread. rtrb is *single producer, single
consumer*: one thread pushes, one thread pops, and that is what lets it
work without locks. midir's thread is a second thread that wants to
push, so it needs a ring buffer of its own:

```rust
pub struct LiveInput {
    notes: rtrb::Producer<MidiMessage>,
    position: Arc<AtomicU64>,
}

impl LiveInput {
    pub fn send(&mut self, message: MidiMessage) -> bool {
        self.notes.push(message).is_ok()
    }
}
```

Because there can be only one producer, this cannot be a method on
`AudioEngine` that anybody may call. It is a *value*, and whoever owns
it is the one producer. The engine creates it in `start` and keeps it in
an `Option` until someone asks for it:

```rust
pub fn take_live_input(&mut self) -> Option<LiveInput> {
    self.live_input.take()
}
```

`Option::take` moves the value out and leaves `None` behind (step 5 used
it for the export thread), so the second call gets nothing. The type
system makes the rule "one producer" impossible to break: `LiveInput`
does not implement `Clone`, so it cannot be copied, only moved.
`MidiOutput`, the way out for MIDI Out channels, works the same way
from the other end: it holds the queue's `Consumer`.

On the audio thread, the consumer is an `Option` too, because the WAV
export from step 5 runs an audio thread with no keyboard at all:

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
(from `pop().ok()`: `Some(key)` or `None` when the queue is empty)
becomes the result. So the loop goes on while there is a consumer *and*
it has a key.

## Sharing a float: the position in an `AtomicU64`

Recording needs to know *when* each key came, as a place in the pattern:
"halfway through step 4". The playhead from step 2 is a whole step, an
`AtomicUsize`. The position is an `f64`, and Rust has no atomic float.

It does have `AtomicU64`, and an `f64` is 64 bits. `f64::to_bits` gives
those bits as a `u64` without changing them, and `f64::from_bits` turns
them back:

```rust
// audio thread, after each block while playing:
self.position.store(self.sequencer.position().to_bits(), Ordering::Relaxed);

// any other thread:
fn load_position(position: &AtomicU64) -> Option<f64> {
    match position.load(Ordering::Relaxed) {
        NO_POSITION => None,
        bits => Some(f64::from_bits(bits)),
    }
}
```

"Stopped" is `NO_POSITION = u64::MAX`, the way `NO_STEP` is for the
playhead. As a float, those bits are a NaN ("not a number"), which no
position ever is, so the two cannot be confused.

The position itself comes from the sequencer, which counts down the
frames until the next step:

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

The position is stored once per block, so it is up to one block old
when a key arrives: about a hundredth of a second. A step at 120 BPM
lasts an eighth of a second, and recording rounds to the nearest step
anyway, so this is far too small to change where a note lands.

## A gate that never runs out

Notes from the sequencer have a *gate*, the number of frames they are
held for, counted down by the voice. A key has no known length, so it
gets a gate that means "until told otherwise":

```rust
pub const HELD: u32 = u32::MAX;
```

The synth's voice skips the countdown for it:

```rust
if matches!(self.stage, Stage::Attack | Stage::Decay) && self.gate != HELD {
    ...
}
```

and the new `note_off` lets go of held voices only:

```rust
pub fn note_off(&mut self, pitch: Note) {
    for voice in self.voices.iter_mut() {
        if voice.pitch == pitch
            && voice.gate == HELD
            && matches!(voice.stage, Stage::Attack | Stage::Decay)
        {
            voice.stage = Stage::Release;
        }
    }
}
```

The `voice.gate == HELD` part matters when you play along with the
pattern. If the pattern plays a long C4 and you tap C4 on the keyboard,
letting go of your key must not end the pattern's note. The test
`a_key_up_leaves_the_sequencers_notes_alone` checks exactly that. Voices
now remember their `pitch`, which they did not need before. The sampler
ignores `note_off`: a drum hit always plays to its end.

## Which channel a key comes up on: `live_keys`

Keys play the *selected channel*, the one the piano roll shows. The UI
tells the audio thread with `Command::SetLiveChannel` whenever that
changes. But the selection can change while a key is down. If the key
then came up on the new channel, the note on the old one would play for
ever. So the audio thread remembers, for each of the 128 notes, which
channel its key went down on:

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
`if let ... && let ...` is a let chain, as in step 6.

Channels are known by their position in the list, and inserting or
removing a channel moves the ones after it. A remembered "channel 2"
could then be a different channel, or none. So before the list changes,
`release_live_keys` lets go of every key that is down. The test
`removing_channels_lets_go_of_their_keys` removes channel 0 while a key
is down on channel 1, which then *becomes* channel 0.

## MIDI Out: an instrument with an outbox

A MIDI Out channel plays no sound. Its instrument turns notes into MIDI
messages for another synth. It is a third kind of `Instrument`, next to
the sampler and the synth, so everything else (the sequencer, live
keys, volume, mute) works for it without changes:

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    MidiOut(MidiOut),
}
```

The audio thread must not send the messages itself. Sending goes
through the operating system's MIDI layer, a system call that may take
any time, and the [golden rule](../step-1/audio-engine.md#the-golden-rule-of-the-audio-thread)
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
grows, so it never allocates. If it were ever full, a message would be
lost rather than break the rule.

A synth only ends a note when it is told to, so `MidiOut` must send a
note-off when each note's gate runs out. It keeps, for each note, an
enum that says whether the note is sounding and until when:

```rust
enum Sounding {
    Off,
    Until(u64), // the frame when its note-off is due
    Held,       // until note_off, for live keys
}
```

`next_sample` makes no sound, but it counts the frames in `now`, which
is the clock for `Until`. Once per block, the audio thread asks every
instrument to hand over its messages:

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
    for message in self.state.outbox.drain(..) {
        send(message);
    }
}
```

`drain(..)` takes every item out of the `Vec` in order, and the `Vec`
keeps its memory for the next block. `send` is an `impl FnMut`: the
instrument does not know or care where the messages go. The audio
thread passes a closure that pushes them into the `MidiOutput` queue,
and the tests pass one that pushes them into a plain `Vec` to look at:

```rust
// audio thread
fn send_midi(&mut self) {
    for channel in &mut self.channels {
        channel.instrument.send_midi(|message| {
            if let Some(out) = &mut self.midi_out {
                let _ = out.push(message);
            }
        });
    }
}
```

The loop borrows `self.channels` mutably, and the closure borrows
`self.midi_out` mutably, both at once. That is allowed: since the 2021
edition, a closure captures only the fields it uses, not all of `self`,
and two different fields are [disjoint borrows](../step-3/voices.md).

MIDI has one of each note per channel. If a note starts again while it
is still sounding, the synth would hear "on, on, off", and the second
note would stop at the first one's note-off. So `note_on` sends a
note-off first in that case
(`midi_out_ends_a_repeated_note_before_starting_it_again`).

Two last details keep notes from hanging on the external synth. When a
MIDI Out channel is removed, or its instrument is replaced, or a new
project is loaded, `silence` sends its note-offs before the instrument
is thrown away:

```rust
fn silence(instrument: &mut Instrument, midi_out: &mut Option<rtrb::Producer<MidiMessage>>) {
    instrument.release_all();
    instrument.send_midi(|message| {
        if let Some(out) = midi_out {
            let _ = out.push(message);
        }
    });
}
```

And the state lives in a `Box`:

```rust
pub struct MidiOut {
    channel: u8,
    now: u64,
    state: Box<MidiOutState>, // 128 `Sounding`s and the outbox
}
```

128 `Sounding`s take 2 KB. An enum is as big as its biggest variant, so
without the box every `Instrument`, and every `Command` that carries one
through the ring buffer, would be 2 KB. The engine has a test that keeps
a command under 80 bytes, and it would fail without the box.

## midir: ports and connections

[midir](https://docs.rs/midir) is the crate that talks to each system's
MIDI layer: ALSA on Linux, Core MIDI on macOS, Windows Multimedia on
Windows. It is built the way `cpal` is: list the ports, open a
connection to one, and the connection stays open for as long as the
value is kept. Ferrous Lineup wraps it in a `Midi` struct, in
`lineup-app/src/midi.rs`.

Ports are found again by an id that midir gives, a `String`, because
the list of ports changes as devices are plugged in and out. The MIDI
menu looks for ports each time it opens.

### Connecting consumes, closing gives back

```rust
let mut midi_in = MidiInput::new(CLIENT_NAME).map_err(|err| err.to_string())?;
midi_in.ignore(Ignore::All);
let found = midi_in
    .find_port_by_id(&port.id)
    .ok_or_else(|| format!("{} is not connected any more", port.name))?;
let connection = midi_in
    .connect(&found, "input", on_message, link)
    .map_err(|err| err.to_string())?;
```

`connect` takes `self`, not `&self`: the `MidiInput` is consumed and
lives inside the connection from then on. `close` gives it back, with
the data. This is ownership used as a state machine: a `MidiInput` that
is connected cannot be connected twice, because you no longer have it.

`Ignore::All` leaves out the messages that are not notes (clock ticks,
system exclusive data, and "active sensing", a keyboard saying "I am
still here" three times a second), so midir's thread does not wake up
for them.

`on_message` is a plain function, and `link` is the data it gets each
time, as `&mut InputLink`:

```rust
fn on_message(_microseconds: u64, bytes: &[u8], link: &mut InputLink) {
    let Some(message) = MidiMessage::parse(bytes) else {
        return;
    };
    let mut position = None;
    if let Some(live) = &link.live {
        let mut live = lock(live);
        live.send(message);
        position = live.position();
    }
    let _ = link.to_ui.send(Played { message, position });
    link.ctx.request_repaint();
}
```

It runs on midir's thread, so `connect` asks for it to be `Send +
'static`, like the audio callback in step 1. The time midir passes in
is ignored: the position from the audio thread is the clock that
matters (see [the next chapter](recording.md)).

### Why the `LiveInput` sits behind a `Mutex`

`InputLink` holds the `LiveInput` as `Arc<Mutex<LiveInput>>`, which
looks odd for a value only one thread uses at a time. The reason is
midir's errors. If `connect` fails, the error gives back the
`MidiInput` (`ConnectError::into_inner`), but *not* the data: `link`
would be gone, and with it the one producer of the live queue, for
good. No keyboard could ever be connected again.

So the `Midi` struct keeps the `LiveInput` itself, behind an
`Arc<Mutex<>>`, and each link holds a clone of the `Arc`. A failed
connection loses only a clone. Only one connection exists at a time, so
the lock is never waited on, and midir's thread is not the audio
thread, so it may lock.

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
anyway". Here it is safe, because a queue end and a connection are
never left half changed. The helper is generic over `T`, so it works for
both mutexes in this file.

### A copy for the UI: `mpsc`

The UI's copy of each key goes through a `std::sync::mpsc` channel
("multiple producer, single consumer"), from the standard library:

```rust
let (to_ui, from_keyboard) = mpsc::channel();
...
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

## The sending thread

The messages that MIDI Out channels leave in the `MidiOutput` queue are
sent by a thread of their own, named `midi-out`:

```rust
fn start_sending(mut messages: MidiOutput, output: Arc<Mutex<Option<MidiOutputConnection>>>) {
    let sender = move || {
        while !messages.is_abandoned() {
            thread::sleep(SEND_INTERVAL);
            let mut output = lock(&output);
            while let Some(message) = messages.pop() {
                if let Some(connection) = output.as_mut() {
                    let _ = connection.send(&message.bytes());
                }
            }
        }
    };
    let _ = thread::Builder::new()
        .name("midi-out".to_string())
        .spawn(sender);
}
```

It sleeps for a millisecond, sends whatever is waiting, and starts
again. A thread that waits for work would usually sleep until it is
woken, with a `Condvar`. But waking a thread is a system call, which
the audio thread must not make. Looking a thousand times a second costs
almost nothing, and adds at most a millisecond.

The lock is taken after the sleep, not before, so the UI thread never
waits for a sleeping thread. The `output` guard is dropped at the end
of each turn of the loop, which unlocks it.

`is_abandoned()` is true once the other end of the queue is gone: the
audio engine was dropped, as when the app closes. Then the loop ends,
and so does the thread. Nothing has to tell it to stop.

`thread::Builder` is the long way to start a thread. It can give the
thread a name, which shows in debuggers and in tools like `top -H`. And
its `spawn` returns a `Result` where `thread::spawn` would panic if the
system refused. If that ever happens, MIDI Out stays silent and
everything else works, which is better than a crash at start.

### Changing ports without hanging notes

The connection to the output port sits behind a `Mutex` because two
threads use it: the sending thread sends, and the UI thread replaces it
when you pick another port:

```rust
let old = std::mem::replace(&mut *lock(&self.output), new);
if let Some(mut old) = old {
    for channel in 0..16 {
        let _ = old.send(&all_notes_off(channel));
    }
    old.close();
}
```

`&mut *lock(&self.output)` locks the mutex, follows the guard to the
`Option` inside (`*`), and borrows it mutably, all in one expression;
the guard is dropped at the end of the statement, so the lock is held
just long enough to swap. The sending thread then sends either to the
old port or the new one, never half to each. Before the old connection
is closed, it gets All Notes Off on all 16 channels, so a synth that
was playing a note does not keep playing it for ever.

The input side has its own version of this: when you pick another MIDI
keyboard, every key held on the old one is let go
(`release_all_keys`), because their key-ups would never arrive.

## The typing keyboard

Reading the letter keys happens once per frame, in `typing_keyboard`.
egui keeps the keyboard's events of this frame in a list, and the code
picks out the ones it wants:

```rust
egui::Event::Key {
    key,
    physical_key,
    pressed,
    repeat: false,
    modifiers,
} => {
    let key = physical_key.unwrap_or(*key);
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
  the key; `physical_key` is where the key *is*. On a French keyboard,
  the key where an English keyboard has Z says W. Using where it is
  keeps the piano's shape on every layout. Some systems do not report
  it, so `unwrap_or` falls back to the letter.
- **Shortcuts are not notes.** Ctrl+Z is undo. A key going down with
  Ctrl (Cmd on a Mac) or Alt held is skipped. A key coming *up* always
  counts, whatever is held, or a note started before Ctrl was pressed
  would never end.
- **Focus.** If the window loses the keyboard (you click another
  window), or a text field takes it (renaming a pattern), the keys'
  "up" events go elsewhere. So all typing keys are let go at once when
  that happens.

The events are read inside `ctx.input(|input| ...)`, which holds a lock
on egui's input while the closure runs. The closure only collects a
`Vec` of `(key, pressed)`, and the loop that plays the notes runs after
it, with the lock released.

Letting go of all keys uses `std::mem::take` again:

```rust
fn release_typing_keys(&mut self) {
    for (_, pitch) in std::mem::take(&mut self.live.typing_down) {
        self.play_typed(MidiMessage::NoteOff { channel: 0, pitch });
    }
}
```

Looping over `&self.live.typing_down` would borrow `self` for the whole
loop, and `play_typed` needs `&mut self`. Taking the list out first
leaves an empty one in its place, and the loop owns the old one, so the
two never overlap.

## The selected channel

Keys need a channel to play, and the piano roll already has one: the
channel it shows. So that is the *selected channel*. Clicking a
channel's name in the rack, which already played a preview, now selects
it too, and the rack highlights it. Once per frame, `live_input` sends
`SetLiveChannel` if the selection changed since the last time.

## A boxed engine

One small change in `app.rs` came from Clippy:

```rust
enum Audio {
    Running(Box<AudioEngine>),
    Unavailable(String),
}
```

The engine grew two new fields in this step, and Clippy's
`large_enum_variant` lint noticed that one variant had become much
bigger than the other. An enum is as big as its biggest variant, so a
`String`'s worth of error message took the room of a whole engine.
`Box` moves the engine to the heap, and the variant holds only a
pointer. The same reason as `MidiOut`'s boxed state, found by a tool
this time.
