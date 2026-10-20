# Real instrument sounds: a SoundFont

Files: [`crates/lineup-audio/src/soundfont.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/soundfont.rs),
[`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs),
[`crates/lineup-app/src/app/sounds.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/sounds.rs)
and [`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)

Until now every note the app played came from the Lineup Osc, its own
synth: two oscillators, a filter and an envelope. Given an instrument
from the General MIDI list, it picks waves and a filter that come
*close* to it ([Every voice on its own channel, with a real
instrument](../step-8/voices-and-instruments.md)): a "piano" is a
sawtooth wave struck and filtered, a "violin" a sawtooth that swells in
slowly. You can tell the parts apart, but nobody would take them for
the real thing.

Backing-band programs play recordings of real instruments. This change
lets Ferrous Lineup do the same, with a **SoundFont**.

## What you see

There is a new menu, **Sounds**, between View and MIDI. Tick
**SoundFont instruments**. The first time, a window explains what a
SoundFont is and offers to download one:

![The "Real instrument sounds" window: GeneralUser GS, about 30 MB, kept in ~/.local/share/ferrouslineup/soundfonts, with Download, Choose a file… and Not now](../images/step-9-soundfont-offer.png)

**Download** fetches GeneralUser GS, a free General MIDI SoundFont made
by S. Christian Collins, from its GitHub page. The window and the
status bar show how far it is; **Stop** gives up. When it is done, the
status bar says "Instruments play from GeneralUser GS", and from then
on:

- every channel with an instrument plays a recording of it. That is the
  startup song's chords (now a piano), the Keys and Bass of every
  Library style (a piano and a bass guitar), and every voice of every
  demo: the Canon's violins and cello, the gospel songs' choir and
  organ, the tangos' bandoneón (an accordion, in General MIDI);
- with **SoundFont drums** ticked (the default), the built-in drums play
  the SoundFont's drum kit: a recorded kick, snare and hi-hat;
- a channel on **None (the Lineup Osc's own sound)** keeps the Lineup
  Osc. Right-click a channel's name and pick an instrument under
  **Instrument** to move it to the SoundFont.

![The Sounds menu, with the hover text of SoundFont instruments](../images/step-9-sounds-menu.png)

The SoundFont is downloaded once. The app keeps it in its data folder
(`~/.local/share/ferrouslineup/soundfonts` on Linux,
`~/Library/Application Support/Ferrous-Lineup/soundfonts` on macOS,
`AppData\Roaming\Ferrous Lineup\data\soundfonts` on Windows) and
remembers the menu's ticks: next time it starts with the sounds on.
**Choose SoundFont…** plays any other `.sf2` file instead; **Use
GeneralUser GS** goes back.

Two more things use the SoundFont without being told: **File > Export
WAV** writes the song as you hear it, SoundFont included, and the
typing keyboard and a MIDI keyboard play it too. **MIDI > Send every
channel** still sends the notes to the external synth, as before.

If the download fails (no network, a firewall), the window says why.
You can download `GeneralUser-GS.sf2` from
[its page](https://github.com/mrbumpy409/GeneralUser-GS) yourself and
pick it with **Choose a file…**.

![Downloading: a progress bar, "2.5 of 6.0 MB", and the same in the status bar](../images/step-9-soundfont-download.png)

(The screenshot was taken with a small test SoundFont of 6 MB;
GeneralUser GS is about 30 MB.)

## What a SoundFont is

A sampler plays one recording at different speeds
([Instruments: drums, sampler and synth](../step-2/instruments.md)):
twice as fast sounds an octave higher. That works for a few notes, but a piano recorded on middle C and
played two octaves up sounds like a toy: the timbre of a real
instrument changes across its range, and its attack gets too short.

A SoundFont (a `.sf2` file, a format from the sound cards of the 1990s)
is a sampler's whole library in one file:

- **samples**: short recordings, a piano's C, E, G♯ and so on every few
  notes, each with the note it was recorded at. Sounds that last, like
  an organ or strings, have a **loop**: once the attack has played, the
  middle of the recording repeats for as long as the key is held;
- **instruments**: which sample to play for which keys and velocities
  (a soft note can play a recording of a soft note), and how: the
  volume envelope, a filter, vibrato;
- **presets**: what a MIDI program change chooses, by **bank** and
  **program**.

A *General MIDI* SoundFont puts the 128 General MIDI instruments in bank
0, in their numbered order, and a drum kit in bank 128. So program 0 is
always a grand piano and program 33 a fingered bass guitar, and on
**MIDI channel 10** (9, counted from 0) every key is a drum: 36 the
kick, 38 the snare, 42 the closed hi-hat.

Playing a note means finding the sample whose region holds that key
and velocity, and playing it faster or slower by the ratio between the
two notes. Every semitone is a factor of 2^(1/12), about 1.0595, so a
sample of A4 (440 Hz) played for B4, two semitones up, runs
2^(2/12) = 1.122 times as fast: 494 Hz.

## Who does the work: rustysynth

Reading an `.sf2` file and playing it is a lot of code: parsing the
file, the regions, the envelopes, the filters, the loops, mixing up to
hundreds of voices. The `rustysynth` crate does all of it, in plain
Rust with no C libraries, under the MIT licence (which a GPL program
may use). Its interface is small:

```rust
let sound_font = Arc::new(SoundFont::new(&mut file)?);
let mut synthesizer = Synthesizer::new(&sound_font, &settings)?;
synthesizer.process_midi_message(0, 0xC0, 33, 0); // channel 1: program 33
synthesizer.note_on(0, 40, 100);                  // E2, velocity 100
synthesizer.render(&mut left, &mut right);        // the next block of sound
```

It speaks MIDI. And the app already has a channel that speaks MIDI: the
**MIDI Out** ([Step 7](../step-7/README.md)). A MIDI Out keeps track of
which notes are sounding and when each one must end, and collects
note-on, note-off and program-change messages for an external synth.
So the new instrument is a MIDI Out whose messages go to a synthesizer
inside the app:

```rust
pub struct SoundFontPlayer {
    /// Which notes sound and until when, as for an external synth; its
    /// messages go to the synthesizer once per block.
    notes: MidiOut,
    /// Boxed so an `Instrument` stays small.
    block: Box<Block>,
}

/// The synthesizer and the block of sound it rendered last.
struct Block {
    synthesizer: Synthesizer,
    sound_font: Arc<SoundFont>,
    gain: f32,
    left: [f32; BLOCK],
    right: [f32; BLOCK],
    read: usize,
}
```

The audio thread asks every instrument for one sample at a time. The
synthesizer works in blocks of 64 frames, so the player hands out the
block it has, and when the block runs out it passes on the messages of
the last 64 frames and asks for the next one:

```rust
pub fn next_sample(&mut self) -> f32 {
    self.notes.next_sample(); // counts frames, for note lengths
    let block = &mut *self.block;
    if block.read == BLOCK {
        let synthesizer = &mut block.synthesizer;
        self.notes.flush(|message| play(synthesizer, message));
        synthesizer.render(&mut block.left, &mut block.right);
        block.read = 0;
    }
    let sample = (block.left[block.read] + block.right[block.read]) * 0.5 * block.gain;
    block.read += 1;
    sample
}
```

A note that starts in the middle of a block starts at the next one: at
most 64 frames late, 1.3 ms at 48 kHz, far below what anyone hears. The
SoundFont's sounds are in stereo; every channel of the app is mono, so
the two sides are added and halved.

`flush` is the same method that hands a MIDI Out's messages to the MIDI
port, and `play` turns each `MidiMessage` into the synthesizer's call:

```rust
fn play(synthesizer: &mut Synthesizer, message: MidiMessage) {
    match message {
        MidiMessage::NoteOn { channel, pitch, velocity } => synthesizer.note_on(
            i32::from(channel), i32::from(pitch.midi()), i32::from(velocity)),
        MidiMessage::NoteOff { channel, pitch } => {
            synthesizer.note_off(i32::from(channel), i32::from(pitch.midi()))
        }
        MidiMessage::ProgramChange { channel, program } => {
            synthesizer.process_midi_message(i32::from(channel), 0xC0, i32::from(program), 0)
        }
    }
}
```

So everything a MIDI Out already does right comes for free: notes that
end after their length, held keys, notes that stop when their track is
muted, note lengths that stretch with a tempo change.

A drum is a MIDI Out on channel 10 that plays one note whatever note
it is given (`playing_as`), exactly as **Send every channel** already
sends the built-in drums ([MIDI Out for every
channel](../step-8/midi-out-every-channel.md)):

```rust
pub fn drum(sound_font: &Arc<SoundFont>, sample_rate: u32, drum: Note) -> Option<Self> {
    let notes = MidiOut::new(DRUM_CHANNEL).playing_as(Some(drum));
    Self::on(sound_font, sample_rate, notes, DRUM_GAIN)
}
```

## One synthesizer per channel, one SoundFont for all

A synthesizer can play sixteen channels at once, so one could play the
whole band. Each channel of the app gets its own instead: that way its
volume knob, its mute light and its meter work exactly as on any other
channel, because the engine mixes channels, not MIDI channels.

The SoundFont itself is tens of megabytes, and is not copied: it is
read once into an `Arc<SoundFont>`, an *atomically reference-counted*
pointer. `Arc::clone` copies the pointer and adds one to a counter; the
memory is freed when the last clone goes. Every synthesizer holds one
clone, and so does the app, so the SoundFont is never freed on the
audio thread (freeing memory there is as forbidden as allocating it).

`Instrument` got a fourth kind:

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    SoundFont(SoundFontPlayer),
    MidiOut(MidiOut),
}
```

and a constructor that says which channels the SoundFont plays:

```rust
pub fn from_sound_font(
    channel: &Channel,
    sample_rate: u32,
    sound_font: &Arc<SoundFont>,
    drums: bool,
) -> Option<Self> {
    let player = match &channel.kind {
        ChannelKind::Synth => SoundFontPlayer::new(sound_font, sample_rate, channel.program?),
        ChannelKind::Sampler(SampleSource::Builtin(drum)) if drums => {
            SoundFontPlayer::drum(sound_font, sample_rate, general_midi_drum(*drum))
        }
        _ => None,
    };
    player.map(Instrument::SoundFont)
}
```

The `?` after `channel.program` returns `None` from the whole function
when the channel has no instrument: that is what keeps a Lineup Osc
channel on the Lineup Osc. A sample file and a MIDI Out are not the
SoundFont's to play either. The app's `build_instrument` asks this
first and falls back on `for_channel` when the answer is `None`.

Every command the UI thread sends to the audio thread travels in a slot
of the command queue, and every slot is as big as the biggest command.
Clippy and the test `commands_stay_small` both caught that a
synthesizer's two blocks of 64 samples had made `Instrument` grow from
32 bytes to 568; the boxed `Block` brings it back.

### Nothing allocated on the audio thread

The rule from [The audio thread, grown up](../step-2/realtime.md) still
holds: the audio thread never asks for memory. A synthesizer reserves everything it needs
when it is made: its voices (128 per channel here), their buffers, the
table of presets. `Synthesizer::new` runs on the UI thread, inside
`build_instrument`, and the finished instrument is sent across. Before
choosing rustysynth, its `note_on` and `render` were read to check they
only reuse those voices and buffers.

Reverb and chorus are turned off: they would cost as much as the voices
themselves, once per channel, and the recordings already have some room
in them.

## Loud enough

The synthesizer plays quietly, at half volume, because it expects a
whole band to share it. The player turns it up by `GAIN` (5) and the
drums by `DRUM_GAIN` (7). Those numbers come from a measurement, not
from taste: the same notes were rendered on the Lineup Osc and on the
SoundFont, and their peaks and average levels compared. A real piano
dies away while the Lineup Osc holds its notes, so the two never match
exactly, and another SoundFont may be louder or softer; each channel's
volume knob does the rest.

## Downloading in the background

`sounds.rs` in the app does the rest: the menu, the window, and the
download. A download of 30 MB takes from a second to a minute, so, like
the WAV export ([Exporting the song as a WAV file](../step-5/export.md)), it runs
on a thread of its own, and the UI asks it each frame whether it is
done.

The thread and the UI share a `Progress`:

```rust
#[derive(Default)]
struct Progress {
    done: AtomicU64,
    total: AtomicU64,
    cancel: AtomicBool,
}
```

*Atomics* are numbers two threads can read and write at the same time
without a lock: the download stores how many bytes it has, the window
reads it to draw the bar, and **Stop** stores `true` in `cancel`, which
the download checks before every piece it reads.

The HTTP request is made with `ureq`, a small HTTP client, using the
certificates the operating system trusts (so it works behind a company
proxy that has its own) and the proxy in `HTTPS_PROXY`, if there is
one. The file is written as `GeneralUser-GS.sf2.part`, then read as a
SoundFont, which checks it is one, and only then renamed to
`GeneralUser-GS.sf2`:

```rust
fn download_to(url: &str, path: &Path, progress: &Progress) -> Result<Arc<SoundFont>, String> {
    let part = path.with_extension("sf2.part");
    let result = download_part(url, &part, progress).and_then(|()| {
        let sound_font = soundfont::load(&part).map_err(|err| err.to_string())?;
        fs::rename(&part, path).map_err(|err| err.to_string())?;
        Ok(sound_font)
    });
    if result.is_err() {
        let _ = fs::remove_file(&part);
    }
    result
}
```

A download cut off halfway, or a web page that came back instead of
the file, never leaves a broken `GeneralUser-GS.sf2` that would fail
every time the app starts.

When the thread is done, the app has an `Arc<SoundFont>`, and every
channel needs a new instrument made from it. The app already knows how
to do that: when **Send every channel** changes, it rebuilds the
instruments whose route changed. Forgetting all the routes makes it
rebuild every one on the next frame:

```rust
pub(super) fn instruments_changed(&mut self) {
    self.routes_sent = None;
}
```

Turning the sounds off does the same, and every channel gets its Lineup
Osc or its built-in drum back.

The ticks of the menu are kept the way the zoom is: in egui's memory,
which eframe saves with the window's size. A `Settings` struct with
`#[derive(Serialize, Deserialize)]` is all it takes:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    on: bool,
    drums: bool,
    file: Option<PathBuf>,
}
```

`#[serde(default)]` lets a future version add a field: settings saved
before it get the field's default instead of failing to load.

## The band gets instruments

The Library made its **Keys** and **Bass** channels with no instrument,
so they kept the Lineup Osc's own sound, and sent to a MIDI synth they
played whatever it had (often a piano for both). Now `synth_named`
gives them one:

```rust
let program = match name {
    KEYS => Some(general_midi::ACOUSTIC_GRAND_PIANO),
    BASS => Some(general_midi::FINGER_BASS),
    _ => None,
};
```

The startup song's chords are a piano too. With the SoundFont off, the
Lineup Osc imitates them as it imitates the demos' instruments.

A project with an instrument on any channel is saved in file format 9
([Every voice on its own channel](../step-8/voices-and-instruments.md)),
so a song started from a Library style now needs format 9. The tests
that check which features need which format start from the startup song
with its piano taken away (`plain_demo`), so each still sees only the
feature it tests.

## Testing without a SoundFont file

The tests should not need a 30 MB download, and the repository should
not hold one. So the test module writes a SoundFont byte by byte: one
sample, a sine wave at 440 Hz that loops, two instruments (one plays
the sample as recorded, one an octave higher) and three presets
(program 0, program 33, and the drum kit in bank 128).

An `.sf2` file is a RIFF file, the same container as a WAV file: a tree
of *chunks*, each a four-letter name, a size and its bytes. Two small
functions build them:

```rust
fn chunk(id: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut bytes = id.to_vec();
    bytes.extend((body.len() as u32).to_le_bytes());
    bytes.extend(body);
    if body.len() % 2 == 1 {
        bytes.push(0);
    }
    bytes
}

fn list(kind: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut body = kind.to_vec();
    for each in chunks {
        body.extend(each);
    }
    chunk(b"LIST", &body)
}
```

The file is a list of three lists: `INFO` (a version and a name), `sdta`
(the samples, as 16-bit numbers) and `pdta` (the presets, instruments
and sample headers, in nine fixed-size tables). The tests then play it
and listen the way an oscilloscope would: a sine wave crosses zero twice
per period, so counting the crossings in a quarter of a second gives its
frequency.

```rust
#[test]
fn the_program_picks_the_instrument() {
    let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 33).unwrap();
    player.start(Note::new(A4), 100, HELD, None);
    let sound = render(&mut player, RATE as usize / 2);
    assert!((frequency(&sound) - 880.0).abs() <= 8.0, "{}", frequency(&sound));
}
```

Program 33 is the instrument an octave up, so A4 comes out at 880 Hz.
Other tests check that a note starts within one block, ends after its
length, gets louder with velocity, that a drum plays its own key
whatever note it is written on, and which channels the SoundFont takes.

## Debug builds

`cargo run` builds without optimizations, and a synthesizer works out
every sample of every voice: unoptimized, a band could be too much for
the audio thread. Cargo can optimize one dependency while the rest
stays quick to build:

```toml
[profile.dev.package.rustysynth]
opt-level = 3
```

## Exercises

1. **Another SoundFont.** Download a different General MIDI SoundFont
   in `.sf2` form (*FluidR3_GM*, or the small *TimGM6mb*), pick it with
   **Choose SoundFont…**, and play the Canon in D. (`.sf3` files, which
   compress their samples, are not supported.) Which strings do you prefer?
   Is one of them much louder? Where would you put a volume setting for
   the SoundFont as a whole?
2. **The Lead.** Twelve-tone rows and micromodes write their melody on
   a channel called Lead, which has no instrument and keeps the Lineup
   Osc. Pick an instrument for it in the rack. Which General MIDI
   instrument would you give the Lead by default, and where in
   `synth_named` would it go?
3. **Stereo.** Every channel is mono, so the SoundFont's stereo piano
   is folded into one side. What would it take to give channels a pan
   knob, and to keep a SoundFont's left and right apart? (Look at how
   the engine mixes `next_sample` into the output frames.)
4. **Reverb.** Turn `enable_reverb_and_chorus` on and listen. It costs a
   reverb per channel; how could all channels share one reverb instead,
   as a mixing desk's send does?
