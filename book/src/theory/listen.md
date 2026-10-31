# Takes: hearing a sung tune

Files: [`crates/lineup-listen/src/voice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/voice.rs),
[`crates/lineup-audio/src/input.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/input.rs),
[`crates/lineup-core/src/take.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/take.rs)
and [`crates/lineup-app/src/app/voice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/voice.rs)

A **take** is a tune sung or whistled along with the song. The
microphone button next to Rec records one while the song plays;
**➕ Recording** in the playlist, or a file dropped on a track, lays a
recording made before on the song. Either way the app hears the notes in it, with their pitch,
their timing and how loud each was sung, and the **Your take** window
asks where they go: in place of what the Lead played there, on another
channel, on a new one, or nowhere. The recording itself stays on a
track of the playlist, drawn as its waveform, where it plays with the
song as a reference. How
to use it is in the manual's
[Takes](https://naelolaiz.github.io/ferrous-lineup/manual/takes.html).

![The Your take window over the playlist: the notes heard and the bars they were sung in, the first ones named, Notes go to Lead, the grid in sixteenth notes, the pitches as sung, and the recording kept on a track of its own](../images/take-review.png)

Three questions have to be answered for that, and each is answered in a
different crate:

1. **Which notes?** `lineup-listen` hears the pitch of the voice,
   moment by moment, and cuts it into notes, in seconds from the start
   of the recording. It knows nothing of songs.
2. **When were they sung?** `lineup-audio` knows when the sound card
   played each bit of the song and when the microphone heard each bit
   of the voice, and writes down, every few milliseconds, where the
   song was heard. It knows nothing of notes.
3. **Where do they go?** `lineup-core` turns those seconds into places
   in the playlist, rounds the notes to the grid and to the key, and
   writes them into the song.

The app ties them together, on threads of their own, so the window
never waits.

## Reading a recording

A recording added from a file is first turned into samples.

### Decoding with Symphonia

An audio file is a **container** (WAV, MP3, Ogg...) holding one or
more **tracks**, each compressed with a **codec**. The
[Symphonia](https://crates.io/crates/symphonia) crate, written in pure
Rust, so there is no C library to install on any system, separates the
two: you *probe* the container, pick its audio track, and give the
track's **packets** to a decoder for its codec. Each packet decodes to a
block of samples.

Symphonia reads from anything that implements its `MediaSource` trait:
a file, or a buffer in memory. It takes it as a `Box<dyn MediaSource>`:

```rust
let file = File::open(path).map_err(DecodeError::Io)?;
let stream = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());
```

`dyn MediaSource` is a **trait object**: "some type that implements
`MediaSource`", with the method to call looked up while the program
runs. Different types have different sizes, so a trait object always
lives behind a pointer, here a `Box`. `Box::new(file)` turns the `File`
into one without a word more.

The decoder loop shows how errors are *matched*, not just passed on
with `?`:

```rust
loop {
    let packet = match format.next_packet() {
        Ok(Some(packet)) => packet,
        Ok(None) => break,
        // A file cut short ends with an I/O error: keep what we have.
        Err(SymphoniaError::IoError(_)) => break,
        Err(err) => return Err(format_error(err)),
    };
    if packet.track_id != track_id {
        continue;
    }
    let block = match decoder.decode(&packet) {
        Ok(block) => block,
        // One damaged packet is a click, not a reason to give up.
        Err(SymphoniaError::DecodeError(_)) => continue,
        Err(err) => return Err(format_error(err)),
    };
    let channels = block.spec().channels().count().max(1);
    sample_rate = block.spec().rate();
    block.copy_to_vec_interleaved(&mut interleaved);
    samples.extend(
        interleaved
            .chunks(channels)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32),
    );
    if too_long(samples.len(), sample_rate) {
        return Err(DecodeError::TooLong);
    }
}
```

An I/O error at the end means a file cut short, so the loop keeps what
it has; a decode error means one damaged packet, so it skips it; any
other error ends the reading. `format_error` is a closure defined just
before, `|err: SymphoniaError| DecodeError::Format(err.to_string())`,
which turns Symphonia's error into ours.

Samples come **interleaved**: left, right, left, right. `chunks(channels)`
cuts the list into one slice per moment, and each slice's average is
one sample of a mono recording: a voice does not care about left and
right. A recording longer than `MAX_SECONDS`, ten minutes, is refused
while it decodes, since it is held in memory while its notes are heard.

Symphonia's codecs are Cargo **features**, turned on in the workspace
`Cargo.toml`, so that a build only carries the ones it uses:

```toml
symphonia = { version = "0.6", default-features = false, features = ["mp3", "wav", "pcm", "flac", "ogg", "vorbis"] }
```

## Hearing the pitch

`voice::pitch_track` gives the pitch of the take every 10 ms, as a
**fractional MIDI note number**: 69.0 is A4 (440 Hz), 69.5 a quarter
tone above it, and `None` where there is no voice:

```rust
pub struct PitchFrame {
    pub time: f64,
    pub pitch: Option<f32>,
    pub loudness: f32,
}
```

A MIDI number is the right unit because the ear hears pitch as ratios:
from 220 to 440 Hz is an octave, and so is from 440 to 880. On the MIDI
scale both are 12. The conversion is one line:

```rust
pub fn midi_of(hz: f32) -> f32 {
    69.0 + 12.0 * (hz / 440.0).log2()
}
```

### Getting the sound ready

Before looking for a pitch, `prepare` cleans the sound and thins it
out. A **high-pass filter** takes away what is below about 40 Hz: the
small offset from zero many microphones have, and the rumble of the
room. It is a *biquad*, a filter that remembers its last two inputs and
its last two outputs, with the coefficients of the classic recipe from
Robert Bristow-Johnson's audio EQ cookbook. The heart of it is one line
inside a `map`, and a tuple assignment that moves the four values it
remembers along:

```rust
let y = b0 * x + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
(x2, x1, y2, y1) = (x1, x, y1, y);
```

Then the sound is **decimated**: only every second or third sample is
kept, so 48 kHz becomes 16 kHz and 44.1 kHz becomes 22.05 kHz. The
highest pitch looked for is about E7 (2 600 Hz), far below half of 16
kHz, so nothing needed is lost, and the work is a third or a half.
Keeping every third sample of a sound with anything above the new half
rate in it would fold that down as false pitches (*aliasing*), so a
low-pass filter goes first: a **windowed sinc**, the ideal low-pass
shape cut to a few dozen samples and smoothed at its ends with a Hann
window.

### YIN: a sound compared with itself

A sung or whistled note is a sound that repeats: the same wave shape
comes back every *period*, 1/440 of a second for A4. The **YIN**
method (de Cheveigné and Kawahara, 2002) finds the period by comparing
a short stretch of the sound, 25 ms here, with itself shifted by every
lag τ, from the shortest period looked for to the longest (60 Hz, a
little below B1):

```text
d(τ) = Σ (x[j] − x[j + τ])²        over the 25 ms
```

Where the shifted copy lines up with the original, the difference is
close to zero, and that lag is the period. Short lags give small
differences too, since the sound changes little in a few samples, so
YIN divides each difference by the average of the ones before it, the
*cumulative mean normalized difference*:

```text
d′(τ) = d(τ) · τ / (d(1) + d(2) + … + d(τ))
```

It starts near 1 and dips towards 0 at the period, and again at twice
the period (the wave lines up there too, an octave lower), and at three
times.

Working out `d(τ)` for every lag one by one would take a sum over the
window for each lag. Squared out, it is

```text
d(τ) = energy of x[0..W] + energy of x[τ..τ+W] − 2 · r(τ)
```

where `r(τ)`, the sum of `x[j] · x[j + τ]`, is a *correlation*. The
energies come from a running sum of squares, and a **Fourier transform**
gives every correlation at once: transform the window and the whole
frame, multiply one spectrum by the other's mirror image (`conj`), and
transform back.

```rust
let mut product: Vec<Complex<f32>> = a.iter().zip(&b).map(|(a, b)| a.conj() * b).collect();
```

The `realfft` crate does the transforms. They are planned once per
sample rate and kept in the `Yin` struct as
`Arc<dyn RealToComplex<f32>>`: a trait object shared by reference
counting, which is how the planner hands its plans out.

### pYIN: every dip a candidate

Plain YIN takes the first dip under a threshold of about 0.1. That
fails in two ways: a breathy note may not dip that low, and the dip at
twice the period may be deeper than the right one. **Probabilistic YIN**
(pYIN, Mauch and Dixon, 2014) keeps every dip as a *candidate* pitch,
with a probability. It tries a hundred thresholds, from 0.01 to 1, and
for each one gives its weight to the first dip under it (the shortest
lag, the highest pitch). The weights follow a *beta distribution* with
its mean at 0.1, where YIN's authors found the best threshold:

```rust
fn threshold_weights() -> Vec<(f32, f32)> {
    let (alpha, beta) = (2.0_f32, 18.0_f32);
    let raw: Vec<(f32, f32)> = (1..=100)
        .map(|i| {
            let x = i as f32 / 100.0;
            (x, x.powf(alpha - 1.0) * (1.0 - x).max(0.0).powf(beta - 1.0))
        })
        .collect();
    let total: f32 = raw.iter().map(|&(_, weight)| weight).sum();
    raw.into_iter()
        .map(|(threshold, weight)| (threshold, weight / total))
        .collect()
}
```

A clear note gets one candidate with a probability near 1. An unclear
frame gets several, each with a little, and what is left of 1 is the
chance that the frame has no pitch at all. Each dip's lag is refined
between samples with a parabola through the three points around it, so
the pitch is not stuck on whole samples: at 16 kHz, a period of 36 or
37 samples is A4 or a third of a semitone lower.

### Viterbi: one path through the whole take

Choosing each frame's most likely candidate on its own would let one
frame jump an octave and back. Instead `viterbi` chooses one candidate
per frame, or "no pitch", for the **whole take at once**: the path that
costs least, where each frame costs how unlikely its choice is (`-ln`
of its probability), each move to the next frame costs how far the
pitch jumps (0.6 for each semitone past the first third of one), and
starting or stopping the voice costs a fixed price. A frame much
quieter than the loud part of the take costs more as a pitch: it is
more likely breath, the room, or the backing track leaking out of the
headphones.

This is the dynamic programming that also finds where a song's key
changes (see [Where the key changes](hearing.md#where-the-key-changes)):
for each frame and each of its states, the cheapest path that ends
there, and which state of the frame before it came from. Then, from the
cheapest end, back frame by frame:

```rust
let mut state = last
    .iter()
    .enumerate()
    .min_by(|a, b| a.1.total_cmp(b.1))
    .map_or(0, |(index, _)| index);
for t in (0..candidates.len()).rev() {
    path[t] = candidates[t].get(state).map(|c| c.pitch);
    state = from[t][state];
}
```

The last state of each frame is "no pitch", one past its candidates, so
`candidates[t].get(state)` is `None` there, which is exactly the
`Option` the pitch track holds. `total_cmp` orders floating-point
numbers completely, `NaN` included, where `<` alone cannot, so `min_by`
can use it.

## From a pitch line to notes

The pitch line of a voice is never flat: it slides into notes, wobbles
with vibrato, and drops out on consonants. `voice::notes` cuts it into
notes:

```rust
pub struct SungNote {
    pub start: f64,
    pub end: f64,
    pub pitch: f32,
    pub loudness: f32,
}
```

(`SungNote` lives in `lineup-core`, which places the notes in the song,
and `lineup-listen` re-exports it with `pub use`.)

1. **Runs.** The frames with a pitch are grouped into runs where the
   voice sounds without stopping. Gaps of a frame or two, where the
   pitch finder lost the voice for a moment, stay inside.
2. **Smoothing.** Each pitch is replaced by the middle value of the
   five around it, a *median filter*: one wrong frame goes, while a
   real change of note, which lasts, stays.
3. **Cuts.** A run is cut where the pitch moves more than three
   quarters of a semitone away from where the note settled and stays
   away for 70 ms, all in the same direction (vibrato comes back
   sooner); where the loudness dips by 5 dB and comes back, as for a
   note sung again on the same pitch ("la, la"); and where it jumps up
   by 6 dB within 30 ms, as when a whistle's tongue strikes a note
   again.
4. **Each piece's pitch** is the middle of where it settled: the median
   of its pitches, leaving out the first quarter (the slide into it)
   and the last tenth. Its loudness is its loudest moment.
5. **Tidying.** A *scoop*, a quick slide of less than 120 ms into the
   next note from a little below or above, belongs to that note, which
   starts where the scoop does; so does less than 120 ms an octave
   away, as the start of a note can be heard an octave off. Anything
   else under 60 ms goes into the note before it. Two notes in a row on
   the same pitch become one, unless the second was sung again.

The loudest moment of a stretch is a **fold**: start from the lowest
possible value and keep the larger of it and each loudness.

```rust
loudness: frames
    .iter()
    .map(|frame| frame.loudness)
    .fold(f32::MIN, f32::max),
```

`f32::max` is passed as the function itself, with no closure around
it: a function's name is a value in Rust.

### In tune: an average that goes round

A singer may be a little sharp all through, or a whistle a quarter tone
flat. `voice::tuning` measures how far the whole take is from the
piano's notes, so that each note can be rounded to the note meant
rather than the nearest one. The distances of the notes from their
nearest semitone lie between −0.5 and 0.5, but they go round in a
circle: +0.45 and −0.45 are close, 0.1 apart across the half-way
point, not 0.9. So each distance is turned into an angle, the angles
are averaged as arrows, longer notes counting more, and the average
arrow is turned back into a distance:

```rust
for note in notes {
    let angle = f64::from(note.pitch - note.pitch.round()) * std::f64::consts::TAU;
    let weight = note.seconds().min(1.0);
    x += angle.cos() * weight;
    y += angle.sin() * weight;
}
(y.atan2(x) / std::f64::consts::TAU) as f32
```

This is a *circular mean*, the same average used for wind directions or
the hours of the day.

## When was it sung: two clocks

A note heard 3.2 s into the recording was sung at some place in the
song. To know which, every moment of the recording has to be tied to a
moment of the song, and two delays are in the way. The sound the audio
thread makes now comes out of the speakers a little later (the output's
*latency*), so the singer hears bar 5 after the engine played it. And
the sound the microphone hears now reaches the program a little later
(the input's latency). Together they can be 20 to 100 ms, as long as a
sixteenth note at a fast tempo.

The sound card says both. With every block of output, cpal gives a
timestamp of when the callback runs and when the block will be played;
with every block of input, when it runs and when the sound was
captured. Both are turned into `Instant`s, moments on the computer's
own steady clock, and then they can be compared.

On the audio thread, while a take records, the engine writes down for
each block **when it will be heard** and where the song is then:

```rust
pub struct Heard {
    pub at: Instant,
    pub position: f64,
    pub steps_per_second: f64,
}
```

They go into a `SongClock`, a lock-free queue (`rtrb`, as for the other
messages of the audio thread; see [Two threads](../program/threads.md)),
which the engine hands out with `take_song_clock` and gets back with
`give_back_song_clock`. Nothing is written while no one listens: an
`Arc<AtomicBool>` says whether anyone does, so the audio thread does
not fill a queue no one empties. Nothing is written during a count-in
either, when the song does not move yet.

The microphone has a callback of its own, on a thread of its own, which
must not wait any more than the audio thread may. It only mixes the
channels into one, pushes the samples into a ring buffer, and writes
down for each block when it was heard:

```rust
let stamp = info.timestamp();
let delay = stamp.callback.duration_since(stamp.capture);
let at = Instant::now()
    .checked_sub(delay)
    .unwrap_or_else(Instant::now);
```

A `Recorder` thread empties both queues every 5 ms, keeps the samples,
and for each block of the microphone finds the last moment of the song
heard at or before it. The song moves on at a steady speed between two
moments, so the place of the block in the song follows:

```rust
let moment = heard.iter().rev().find(|moment| moment.at <= at)?;
let since = at.duration_since(moment.at);
if since > LONGEST_GAP {
    return None;
}
Some(Mark {
    frame,
    position: moment.position + since.as_secs_f64() * moment.steps_per_second,
    steps_per_second: moment.steps_per_second,
})
```

A **mark** says: at this sample of the recording, the song was heard at
this position, moving on at this speed. A block with no moment of the
song in the last 100 ms (`LONGEST_GAP`) was sung while the song was not
playing, and gets no mark. Some sound cards say their delays wrong, so
**Settings > Audio > Microphone** has a nudge, in milliseconds, which
moves every mark later or earlier.

When the take stops, the thread ends and **returns** what it made, the
recording and the song clock, through its `JoinHandle`. The clock goes
back to the engine for the next take. Ownership makes this tidy: the
clock was moved into the thread when it started, and comes back out
when it ends, so no two threads ever hold it at once, and no lock is
needed.

A recording added from a file has no sound card to ask. `Take::laid`
makes its marks as if it had been heard from the step it is laid on,
straight on at the song's tempo, through its tempo changes, a mark
every 10 ms. From there it is a take like any other.

## Where in the song: runs

The song as it plays is not the playlist as drawn: a chorus marked ×2
plays twice, and a loop region goes round and round (see [Sections,
repeats and loops](../program/project.md#sections-repeats-and-loops)).
So a mark's song position is turned back into a place in the playlist
with the arrangement's `pass_at`, as the playhead is (see [Back from the
song to the playlist](../program/project.md#back-from-the-song-to-the-playlist)),
and the take is cut where that place jumps, into **runs**.

The marks come every few milliseconds, and the sound card's clocks
wobble by a few milliseconds from block to block, so the song may seem
to go back a little, or on a little further than it should. `goes_on`
allows 20 ms of that before it calls it a jump. `runs` checks every
5 ms, and finds a jump to the sample by halving the gap between the
last sample that went on and the first that did not, a **binary
search**.

Where two runs cover the same bars, a chorus sung twice or a loop sung
round again, the **later one wins**: `kept` walks the runs from the last
to the first and takes away from each what a later one covers. Sing it
again until it is right. Each part kept becomes a **piece** of the
recording, starting on the first whole step inside it.

## Which notes: transcribing

`take::transcribe` puts it all together. For each note heard:

- its start and end become places in the playlist (or the pattern); a
  note heard less than 150 ms before the song starts starts with the
  song, since the pitch finder can hear a note's start a frame or two
  early;
- they are rounded to the grid asked for: sixteenths, eighths or
  quarter notes;
- its pitch, less the take's tuning, is rounded to a semitone; a pitch
  right between two (from 0.35 to 0.65 of the way) goes to the one in
  the key there; or it goes to a note of the key, or of the chord's
  scale, as the window asks (see [Pitches: as sung, in the key, or with
  the chords](#pitches-as-sung-in-the-key-or-with-the-chords));
- how loud it was sung sets its **velocity**: a note as loud as the
  middle one of the take plays at 100, louder ones harder, softer ones
  softer.

A voice's loud and soft lie further apart than an instrument's, so the
difference is squeezed:

```rust
pub fn velocity(loudness: f32, usual: f32) -> u8 {
    let gain = 10.0_f32.powf(0.35 * (loudness - usual) / 20.0);
    (100.0 * gain).round().clamp(20.0, 127.0) as u8
}
```

Decibels are logarithmic, so `10^(dB / 20)` turns a difference in dB
into a ratio of amplitudes, and the `0.35` squeezes it: 6 dB louder
plays at 127, 20 dB softer at 45, roughly from forte to piano.

Last, a take is one voice, so the notes are made one at a time: of two
on the same step the longer one sung stays, a note ends where the next
one starts, and a gap of a step or two (a breath, a consonant) is
closed rather than written as a rest.

### Pitches: as sung, in the key, or with the chords

A singer a little out of tune, or a whistle that slides, can land
nearer a wrong semitone than the right one. `take::transcribe_with`
takes a `Pitches` that says how strict to be:

```rust
pub enum Pitches {
    #[default]
    AsSung,
    InKey,
    WithChords,
}
```

**As sung** is the rounding above. **In the key** writes the nearest
note of the key there: in D major, a D♯ sung a quarter of a semitone
sharp is an E, since D♯ is not in the key. **Key and chords** writes the
nearest note of the scale a soloist plays over the chord on the chord
sheet there, in the key: the **chord scale** that `chord_scale` picks
for the [soloist](../band/soloist.md#choosing-a-chord-scale). Over D7 in
C major, the V7 of G, it is G major, C D E F♯ G A B, so a sung F is
written F♯, while G, the note D7 leads to, stays. Over Fm, borrowed from
C minor, it is F G A♭ B♭ C D E♭. Bars with no chord keep to the key; a
micromode on the chord sheet is the scale there.

`allowed_at` gives two [sets of pitch
classes](pitch.md#a-set-of-pitch-classes-in-twelve-bits) for a step:
the notes a pitch may be written as, and the ones preferred right
between two, the chord's own:

```rust
let allowed = match harmony {
    Some(HarmonyName::Chord(symbol)) => {
        let mut allowed = chord_scale(symbol.chord(), scale);
        allowed.insert(symbol.bass_pitch_class());
        allowed
    }
    // A micromode is the scale there.
    Some(HarmonyName::Micromode(set)) => set,
    // A set, or two chords at once: the key with their notes.
    _ => PitchSet::from_bits(key.bits() | chord.bits()),
};
```

The bass of a slash chord is added, since G/F♯ has an F♯ under it
that the G chord's scale may not hold. `PitchSet::from_bits(a.bits() |
b.bits())` is a union: each set is twelve bits, one per pitch class, so
the bitwise or of the two holds every note in either.

`nearest_in` then looks at every note within an octave of the pitch
sung, keeps the allowed ones, and takes the nearest; at the same
distance, one in the preferred set wins. The window transcribes the
take again each time the choice changes, so the notes it lists show
the difference at once.

## Writing it in

`Project::add_take` writes a transcription into the song, where the
`Target` says:

```rust
pub enum Target {
    Channel(usize),
    New { name: String, program: u8 },
    Nowhere,
}
```

In song mode the notes go into the **patterns that already hold that
channel's notes** where they were sung, in place of the notes there:
correct the tune once, and it is corrected wherever its pattern plays.
`Added::shared` lists those patterns that play more than once, so the
app can say so. Notes sung where no pattern holds the channel's notes,
and every note for a new channel, go into clips of their own, on a
track **right under the recording's**, named by `Keep::notes`, as
*Take 1 notes*: the recording and its notes stay side by side, and
each can be muted on its own. Without a recording kept, those clips go
on the track the window chose. In pattern mode the notes go into the
pattern.

The recording is kept as a WAV file per piece (`lineup_listen::wav`,
32-bit floating point, so nothing recorded is lost), in a folder next
to the song named after it, *song takes*. Each piece plays on a
sampler channel of its own, a `SampleSource::Take`, started by one note
on the piece's first step, in patterns that start on bar lines on the
take's track. A take's sampler **plays along** with the song like a
long recording, however short it is: from wherever playback starts,
halfway through if playback starts in its second bar, and it stops when
playback stops.

The song saves the full path of each piece. When a piece is not there
any more, `file::load` looks for it again: `find_moved_take` tries a
folder of the same name next to the song, the takes folder named after
the song, and the song's own folder. So a song moved or copied to
another computer together with its takes folder finds its takes.

A song with a take is saved in [format 32](../reference/project-format.md):
an older Ferrous Lineup does not know what a take is, and says it needs
a newer version rather than open the song without it.

## In the app

`app/voice.rs` keeps four `Option`s: the take recording, the take whose
notes are being heard, the file being read, and the take waiting in the
**Your take** window.

The microphone button opens the system's default input with
`Microphone::open`, takes the song clock, starts the `Recorder` and,
if the song is not playing, plays it. While it records, a small panel
under the button shows how loud the microphone hears you and the note
it hears, with how far off it is ("A4 +12¢"). The pitch comes from the
recorder's thread: every 40 ms it calls a **closure** the app gave it,
with the last 60 ms of sound, and the closure stores the pitch of one
frame (`voice::pitch_now`) in an `AtomicU32`, as the bits of an `f32`:

```rust
let view: lineup_audio::input::LiveView = Box::new(move |samples, rate| {
    let pitch = voice::pitch_now(samples, rate).unwrap_or(f32::NAN);
    live.store(pitch.to_bits(), Ordering::Relaxed);
});
```

There is no atomic `f32` in the standard library, but an `f32` is 32
bits, so `to_bits` and `f32::from_bits` carry it through an
`AtomicU32`. `lineup-audio` does not know what a pitch is: it only
calls a `Box<dyn FnMut(&[f32], u32) + Send>`, and the app decides what
to do with the sound.

Hearing the notes of a long take takes a moment, so it runs on a thread
of its own, as the WAV export does (see [Knowing when it is
done](../sound/export.md#knowing-when-it-is-done)). Every frame
`check_voice` asks the handle `is_finished()`, and when it is, `join`s
it and opens the window. A file added is read and heard the same way;
that thread first takes the window's language, so an error is written
in it (see [Settings that live on a
thread](../sound/export.md#settings-that-live-on-a-thread)).

The window changes nothing until **Write it in**. Choosing another grid
or another way of writing the pitches transcribes the take again, which
is quick, since the notes were already heard. **Write it in** transcribes once more, in case the song
changed while the window was open, saves the pieces, and writes the
take with `self.edit`, so it is **one step of undo**. **Throw it away**,
or closing the window, keeps nothing.

### Transcribing a take again

A take already written in can be heard again, to write its notes with
other choices: **Transcribe again…** on its clip's menu. The playlist
finds the recording a clip shows with `recording_in`: one whose note
starts in the clip or, for an empty clip (the rest of a long take), the
last one before it on its track. The app reads that WAV file and hears
it as it hears any recording added, on a thread, then lays it from the
step its note is on with `Take::laid`. A piece of a take starts on a
whole step and plays straight on at the song's tempo, so laying it there
gives the timing it was written with.

The review remembers what it is in an `Again`: the recording's track and
the take's name. **The recording** row only says it stays as it is, and
nothing is saved or added: `Keep::recordings` is empty. The notes go
where they went before: `Review::again` looks at the track right under
the recording, and if it is the take's notes track, picks the channel
whose notes are on it; else the tune channel, as for a new take. Then
`add_take` does the rest, as for any take: the notes go into the
patterns that hold that channel's notes where the take was sung, in
place of the ones there, which are the notes written the first time.

Notes with no home there go on a track of their own, right under the
recording, as the first time. `Keep::under` names the recording's track
for that, since no recording clips are made this time, and
`notes_track_under` uses the notes track already there when it has
room, or adds one.

On macOS an app has to say why it wants the microphone before the
system lets it listen, so `packaging/macos/Info.plist` has an
`NSMicrophoneUsageDescription`, which macOS shows the first time.

### Drawing a take: its waveform

A take's clips hold one note, on the piece's first step, so drawn as
notes they would show a dot. The playlist draws its **waveform**
instead: a vertical line for each column of pixels, as tall as the
loudest sample in the stretch of the recording under it.

Reading a long recording takes a moment, too long to hold up a frame,
so `Waves` reads each file once on a thread of its own and keeps the
`JoinHandle` in a `HashMap` keyed by the file's path. Until the thread
`is_finished()`, the clip is drawn empty and the playlist asks for
another frame in 100 ms; then it `join`s, and keeps the result behind
an `Arc`, so drawing can hold it while the map is borrowed again.
`Wave::read` decodes the file with `lineup_listen::decode_file` and
keeps the loudest sample of every 1/400 s. Then it divides them all by
the loudest of the whole take, with a floor of 0.01 so near silence
stays near silence: a take sung softly fills its clip as one sung
loudly does.

A long take is cut into several clips, each at most a pattern long,
and only the first holds the note that starts it. So `paint_clips`
first gathers where each recording starts, by track, and draws each
one under every clip of its track it plays under, not only under the
one that starts it.

The recording plays straight on in seconds while the playlist is laid
out in steps, and a step's length in seconds changes with the tempo.
`paint_wave` walks the steps from the piece's start, adding each one's
length, `seconds_per_beat() / 4.0` at the tempo there, so the waveform
stays under the bars it is heard with through every tempo change.

### Hearing a take alone

Every track has an **S** next to its mute light. It mutes every other
track, and a second click unmutes them all. One rule does both: a
track is alone when it is the only one playing, and the click flips
between that and everyone playing:

```rust
let alone = tracks
    .iter()
    .enumerate()
    .all(|(other, t)| (other == track) != t.muted);
for other in 0..tracks.len() {
    if let Some(t) = project.track_mut(other) {
        t.muted = !alone && other != track;
    }
}
```

`(other == track) != t.muted` is true when this track plays and every
other is muted: `!=` on two `bool`s is "exactly one of them". The S is
lit while its track is alone. It is one edit, so **Ctrl+Z** brings the
mutes back as they were, and the mutes are saved with the song, as the
mute light's always were.

### From the playlist, the command line or a drop

A recording reaches the app from **➕ Recording** in the playlist's
toolbar, from a track's **Add a recording…**, from the command line, or
dropped on the window. The button lays it on the song from the playhead
(from bar 1 in pattern mode, which switches to song mode); the track's
menu does too, on that track. A drop on the playlist starts it where
it was dropped, on the bar, beat or step **Snap** says, on the track
under the pointer; a drop anywhere else starts it at the playhead.

Dropped files reach the app before the playlist is drawn again, so the
playlist keeps a `DropGrid`, the grid as it was drawn last frame: where
it is on the screen, the layer it is on, and where each track starts.
`recording_drop` asks it which step and track the pointer is over, and
only if no window covers that place (`ctx.layer_id_at` names the
topmost layer there). While a file is dragged over the playlist, the
same grid lights the place where it would start. A file named on the command line,
or opened from a file manager, goes through `open_now`, which opens
projects too, and which asks `lineup_listen::is_audio_file` first: an
extension in `AUDIO_EXTENSIONS`, `["mp3", "wav", "flac", "ogg"]`, in
any case. An audio file opened that way starts a new, empty song with
the recording from its start.

A dropped file is harder to place, because a sound dropped on the
window can also be a sample for the channel rack (see [The
sampler](../sound/instruments.md#the-sampler)): a WAV of a drum hit is
a sound, a WAV of a whole song is a recording. `sample::is_recording`
decides by what the sampler can play and for how long:

```rust
pub fn is_recording(path: &Path) -> bool {
    let is_wav = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"));
    if !is_wav {
        return true;
    }
    hound::WavReader::open(path).is_ok_and(|reader| {
        let rate = u64::from(reader.spec().sample_rate);
        u64::from(reader.duration()) > rate * crate::instrument::LONG_SAMPLE_SECONDS
    })
}
```

The sampler plays only WAV, so an MP3, FLAC or Ogg file can only be a
recording. A WAV is one when it lasts longer than ten seconds
(`LONG_SAMPLE_SECONDS`), the same line past which a sampler voice fades
out when playback stops: past it, a sound is treated as a piece of
music rather than a hit. `reader.duration()` is the number of samples
per channel, read from the file's header without decoding it.
`handle_dropped_files` splits what was dropped in three with
`partition`, twice: project files, recordings, and sounds. Only one
recording is added at a time, since each opens its own **Your take**
window.

## How well it works

The pitch finder was measured on the
[vocadito](https://zenodo.org/records/5578807) dataset: 40 solo singers,
with the pitch and the notes written down by hand (CC BY 4.0).

- **Pitch**: 84.7% of the frames where someone sings are heard within a
  quarter tone of the written pitch.
- **Notes**: an F-measure of 0.708, counting a note as found when it
  starts within 50 ms of a written one, on the same pitch within a
  quarter tone. The F-measure combines how many written notes are found
  with how many of the notes found are written.

The timing was measured with the speakers recorded as the microphone (a
*loopback*), playing a demo's melody: every note came back on its step,
with a median error of about 2 ms.

## Rough edges

- Notes sung again on the same pitch with no gap and no new attack (a
  legato repeat) come out as one long note.
- One voice at a time: a chord, or a voice over a loud backing track,
  is heard as whichever pitch is strongest. Headphones help.
- A recording added from a file goes straight on at the song's tempo:
  a performance that speeds up or slows down drifts from the bars.

## Testing it

The tests make their own voices, so every case has a known answer: a
voice is a tone with softer and softer overtones and a little vibrato,
a whistle a pure tone. A held note is one note at its pitch, a legato
line is cut where the pitch moves, a note sung again is two notes,
silence and noise have none, a take sung sharp is heard as sharp.
`take.rs` tests the placing with marks made by hand: a chorus sung
twice keeps the second time, a clock that wobbles does not cut the
take, a note heard a moment before the song starts with it, a take
saved and opened again is still a take. `input.rs` tests the marks with
moments made by hand, and the window tests in `ux_tests.rs` sing a take
into the Lead, keep only the notes on a new channel, and throw one
away.

The vocadito measurement is an **ignored test**, since the dataset is
not in the repository:

```text
VOCADITO=/path/to/vocadito cargo test --release -p lineup-listen \
    --test sung_notes -- --ignored --nocapture
```

`#[ignore]` keeps a test out of a plain `cargo test`, and
`-- --ignored` runs only those. `--nocapture` lets it print its table.
