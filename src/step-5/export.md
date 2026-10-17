# Exporting the song as a WAV file

Files: [`crates/lineup-audio/src/render.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/render.rs)
and [`crates/lineup-app/src/app/export.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/app/export.rs)

## What is in a WAV file

A WAV file is a short header (44 bytes here) saying how the sound is
stored, followed by the samples themselves. Ferrous Lineup writes the most
common kind:

- **44,100 frames per second**, the rate of a CD, which every program
  understands,
- **2 channels** (stereo), stored **interleaved**: left, right, left,
  right..., exactly like the buffers cpal hands the audio thread since
  step 1,
- **16 bits per sample**: each sample is an `i16`, a whole number from
  -32,768 to 32,767.

That makes 44,100 × 2 × 2 = 176,400 bytes per second of sound, about
10.6 MB per minute. The demo song with its two-second tail is 739,523
frames, 16.77 seconds, and its file is 44 + 739,523 × 4 = 2,958,136
bytes.

Writing the header and the samples by hand would be easy, but the
`hound` crate already does it, and the app has used it since step 2 to
*read* WAV files for the sampler.

## The audio thread, without a sound card

Look at the audio thread from the outside, and it is a function that
fills a buffer:

```rust
pub(crate) fn process(&mut self, output: &mut [f32])
```

It does not know or care that cpal is the one asking. With a sound card,
cpal asks for a few hundred frames at a time, as fast as they are played.
To make a file, a loop asks for them as fast as the computer can make
them, and writes them out. It is the same code making the same sound, so
an export sounds exactly like playback: there is no second engine to keep
in agreement with the first. (The engine's tests have done this since
step 2: they never open a sound card either.)

`lineup-audio` wraps this up in a **`Renderer`**:

```rust
pub struct Renderer {
    audio: AudioThread,
    sample_rate: u32,
    channels: usize,
    /// The audio thread is built with a command queue, but the renderer
    /// hands it commands directly. Keeping this end of the queue alive
    /// keeps the queue valid.
    _commands: rtrb::Producer<Command>,
    trash: rtrb::Consumer<Garbage>,
}
```

`AudioThread` stays `pub(crate)`: other crates cannot build one, call
`process` on it, or reach its fields. They get the `Renderer`, whose
methods keep the rules. Making the export a function *inside*
`lineup-audio` (step 4's exercise 6 asked about this) is what lets the
engine stay private.

`AudioThread::new` wants the receiving end of a command queue, because on
the real audio thread that is how commands arrive. The renderer has no
other thread to receive from, so it makes a queue with room for one
command, never uses it, and calls `apply` directly instead:

```rust
/// Applies a command at once: there is no other thread to hand it to.
/// Volume changes take effect at once too, without smoothing: the
/// commands come before the first sample is made.
pub fn send(&mut self, command: Command) {
    self.audio.apply(command);
    self.audio.settle_volumes();
    self.empty_trash();
}
```

The field holding the queue's other end is called `_commands`. A leading
underscore tells the compiler (and the reader) that the field is never
read on purpose, so it does not warn "field `commands` is never read".
The field still matters: it keeps the producer alive for as long as the
renderer lives.

### Two details that only show up offline

**Smoothing.** Since step 2, volume changes glide over a few milliseconds
(`Smoothed`), because a sudden jump makes an audible click. On a sound
card, that glide happens while the music plays. In an export, setting the
master volume right before the first sample would make the first
milliseconds of the file glide from the default volume to yours. The
first test of the renderer caught it: the first click came out too quiet.
So `send` calls `settle_volumes`, which puts every volume at its target
at once, using a new `Smoothed::settle`:

```rust
/// Jumps straight to the target, with no smoothing.
pub fn settle(&mut self) {
    self.current = self.target;
}
```

**The trash.** The real audio thread never frees memory: whatever it
throws away (an old list of notes, a removed channel) goes into the trash
queue, and the UI thread frees it once per frame. The renderer is not a
real-time thread, so it can free things straight away, and it empties
the queue after every command and every block. Nothing would break if it
forgot (the audio thread frees things itself, as a last resort, when the
queue is full), but this way the renderer behaves like the real thing.

## Writing the file

```rust
pub fn export_wav(
    renderer: &mut Renderer,
    path: &Path,
    frames: usize,
    mut progress: impl FnMut(usize),
) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: renderer.channels() as u16,
        sample_rate: renderer.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut temporary = PathBuf::from(path);
    temporary.as_mut_os_string().push(".tmp");

    let mut writer = hound::WavWriter::create(&temporary, spec)?;
    let mut block = vec![0.0_f32; BLOCK_FRAMES * renderer.channels()];
    let mut done = 0;
    while done < frames {
        let count = (frames - done).min(BLOCK_FRAMES);
        let samples = &mut block[..count * renderer.channels()];
        renderer.render(samples);
        for &sample in samples.iter() {
            writer.write_sample(to_i16(sample))?;
        }
        done += count;
        progress(done);
    }
    writer.finalize()?;
    fs::rename(&temporary, path)?;
    Ok(())
}
```

The sound is made in blocks of 4,096 frames, in one buffer that is reused
for every block. The last block is usually shorter, so `samples` is a
slice of just the part that is needed. After each block, `progress` is
called with the number of frames done. It is an `impl FnMut(usize)`: any
closure that takes a number, and may change what it captured (the app's
closure stores the number somewhere the window can read it).

Every `?` here returns a `hound::Error`. `fs::rename` returns a
`std::io::Error` instead, and `?` still works, because `hound::Error`
implements `From<std::io::Error>`: the `?` operator converts the error on
the way out, as step 4's `FileError` did.

### Write to a temporary file, then rename

The samples go to `song.wav.tmp` first, and only when everything is
written is it renamed to `song.wav`. Exporting over an older `song.wav`
means the old file stays whole until the new one is complete: if the disk
fills up or something fails halfway, you are left with your old export,
not half of a new one. Renaming a file within one folder replaces the old
one on Linux, macOS and Windows alike, and on Linux and macOS it does so
in one indivisible step.

`as_mut_os_string().push(".tmp")` adds to the name. `set_extension("tmp")`
would *replace* `.wav`, giving `song.tmp`, which could be the name of
someone else's file.

### From `f32` to `i16`

```rust
/// Turns a sample between -1.0 and 1.0 into a 16-bit integer sample.
fn to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16
}
```

`clamp` keeps the sample between -1.0 and 1.0, then it is scaled to
-32,767..32,767 and rounded to the nearest whole number. An `as` cast
from a float to an integer already stops at the integer's limits in Rust
(it *saturates*: `3e9 as i16` is 32,767), so the clamp is not strictly
needed for safety, but it says what is meant. `f32::from(i16::MAX)` is a
conversion that can never lose anything, so it is a `From`, not an `as`.

The audio thread already keeps the master output inside -1.0..1.0 with
its soft clipper (`tanh`, from step 2), so in practice the clamp never
cuts anything.

## How long is a song?

The song's length is known in steps; the file's length is in frames:

```rust
let step_frames = frames_per_step(self.project.tempo, EXPORT_SAMPLE_RATE);
let song_length = self.project.song_length();
let tail_frames = TAIL_SECONDS * f64::from(EXPORT_SAMPLE_RATE);
...
let frames = (song_length as f64 * step_frames + tail_frames) as usize;
```

At 130 BPM there are 4 steps per beat, so a step lasts 60 / 130 / 4 =
0.115 seconds, 5,088.46 frames at 44,100. The demo's 128 steps take
651,323 frames, plus a two-second tail of 88,200, makes 739,523.

The tail lets the last notes ring out. But there is a catch: in song mode
the audio thread *loops*. When the playhead reaches the end of the song,
it starts again from bar 1, so the tail would end with the first chord of
the song starting again. The fix is to tell this renderer, and only this
one, that the song is longer than it really is, by some silent steps:

```rust
// The song loops when it reaches its end, so silent steps are
// added after it: enough for the tail, and one more, so the
// export ends before the song would start again.
let silent_steps = (tail_frames / step_frames).ceil() as usize + 1;
renderer.send(Command::SetSong {
    clips: self.project.playlist().audible_clips().collect(),
    length: song_length + silent_steps,
});
```

For the demo that is 17.33, rounded up to 18, plus one: 19 silent steps.
Why the extra one? A step is not a whole number of frames, so the
sequencer starts some steps a frame earlier or later than the exact
fraction. The extra step means the end of the file is always a whole
step away from the loop, whatever the rounding.

## A thread of its own

Rendering takes a while. On the computer this was written on, the demo
exports in a fraction of a second, and a 15-minute song took about three
seconds (in a release build; a debug build is many times slower). The
window must not freeze meanwhile. So the export runs on a **thread** of
its own:

```rust
let done = Arc::new(AtomicUsize::new(0));
// The thread gets its own `Arc` pointing to the same counter, and
// its own copy of the path. `move` gives the closure ownership of
// them, and of the renderer: from now on they belong to the thread.
let worker = {
    let done = Arc::clone(&done);
    let path = path.clone();
    thread::spawn(move || {
        export_wav(&mut renderer, &path, frames, |frames_done| {
            done.store(frames_done, Ordering::Relaxed);
        })
        .map_err(|err| err.to_string())
    })
};
```

`std::thread::spawn` starts a new thread running a closure, and wants
two things of it:

- **`'static`**: the closure may not borrow anything from the code that
  started it. The new thread can outlive the function that spawned it
  (it certainly outlives this frame of the UI), so a borrowed reference
  could point at something that is gone. That is why the closure is
  marked **`move`**: it takes *ownership* of everything it uses (the
  renderer, the path, the counter) instead of borrowing it.
- **`Send`**: everything moved into it must be safe to hand to another
  thread. The renderer qualifies, because everything in it does: the
  instruments, the `Arc`s with the samples, the ring buffer ends. If it
  held an `Rc` (a reference count that is not thread-safe), this line
  would not compile.

The block `{ let done = Arc::clone(&done); ... }` is a common trick. Both
the UI and the thread need the counter, so each gets its own `Arc`
pointing at it. The inner `done` shadows the outer one inside the block,
and the `move` closure takes the inner one, leaving the outer `done` for
the UI. The same goes for `path`.

### Progress: an atomic counter

The thread writes how many frames are done into an **`AtomicUsize`**,
and the window reads it when it draws the progress bar. An atomic can be
read and written from two threads at once without a lock, as the
playhead has been since step 2.

`Ordering::Relaxed` is the weakest (and cheapest) promise about how this
write is seen by other threads: the number itself is never torn or
mixed up, but nothing else is guaranteed to be seen in order with it.
That is all a progress bar needs. Nothing else is read because of the
counter's value; the file is only trusted once the thread has finished,
and `join` (below) gives the full guarantee.

### Knowing when it is done

`spawn` returns a **`JoinHandle`**. The app keeps it, with the counter,
in an `Export`:

```rust
pub(super) struct Export {
    path: PathBuf,
    frames: usize,
    done: Arc<AtomicUsize>,
    worker: JoinHandle<Result<(), String>>,
}
```

`JoinHandle<Result<(), String>>` says what the thread's closure returns:
`Ok(())` if the file was written, or an error message. The error is
turned into a `String` on the export thread (`map_err(|err|
err.to_string())`), since a message for the status bar is all the window
needs.

Every frame, the app checks on it:

```rust
pub(super) fn check_export(&mut self, ctx: &egui::Context) {
    let Some(export) = &self.export else {
        return;
    };
    if !export.worker.is_finished() {
        ctx.request_repaint_after(Duration::from_millis(100));
        return;
    }
    // `take` moves the export out of the `Option` and leaves `None`,
    // so `join` can take ownership of the thread's handle.
    if let Some(export) = self.export.take() {
        let name = export.file_name();
        match export.worker.join() {
            Ok(Ok(())) => self.notice = Some(format!("Exported {name}")),
            Ok(Err(err)) => self.message = Some(format!("Could not export {name}: {err}")),
            // `join` fails only if the thread panicked.
            Err(_) => self.message = Some(format!("Exporting {name} failed")),
        }
    }
}
```

A few things to notice:

- egui only redraws when something happens (a mouse move, a key). While
  an export runs, nothing might, and the progress bar would stand still.
  `request_repaint_after(100 ms)` asks for another frame soon, ten times
  a second, until the export is done.
- `is_finished()` asks without waiting. `join()` *waits* for the thread
  to end, but it is only called once `is_finished()` is true, so it
  returns at once.
- `join` takes the handle by value (it consumes it: a thread can only be
  joined once). The handle is inside `self.export`, an `Option`, and you
  cannot move something out of a field you only borrow. **`Option::take`**
  solves this: it moves the value out and leaves `None` in its place, so
  the export is gone from `self` and owned by the local `export`.
- The `match` has two layers: `join` returns a `Result` that is `Err`
  only if the thread panicked, and inside its `Ok` is the closure's own
  `Result`. Patterns can be nested, so `Ok(Ok(()))` matches "the thread
  ended normally, and the file was written".

## Putting the song into the renderer

The export builds its renderer with the same commands the app sends at
startup and after opening a file, but with instruments made for 44,100
frames per second, whatever rate your sound card runs at:

```rust
let mut renderer = Renderer::new(EXPORT_SAMPLE_RATE, EXPORT_CHANNELS);
renderer.send(Command::SetMasterVolume(self.master_volume));
for command in self.project_commands(EXPORT_SAMPLE_RATE) {
    renderer.send(command);
}
// ...the longer song...
renderer.send(Command::SetMode(PlayMode::Song));
renderer.send(Command::Play);
```

`project_commands` is new in this step: it lists the tempo, every pattern
(`InsertPattern`), every channel (`InsertChannel`, with its notes in all
patterns) and the song. `send_whole_project`, which talks to the real
audio thread, uses it too, so both are built the same way. The built-in
drums are synthesized again at 44,100, the Lineup Osc is made for that
rate, and samplers get their sounds from the `SampleCache` of the
previous chapter, so no file is read.

The renderer owns its own instruments and its own copy of every note. It
is a snapshot, like the copies in the undo history: you can keep playing
and editing while it runs, and the export goes on with the song as it was
when you chose Export WAV.

The file name comes from the project's: `FileState::export_name` turns
`song.lineup` into `song.wav`, or suggests `Untitled.wav` for a project
that was never saved. If you type a name without an extension, `.wav` is
added.

## What the result was checked against

Besides the unit tests (a click rendered by the renderer is exactly the
click the audio thread makes; a WAV file has the right format, length and
first sample, and no `.tmp` file is left behind), the demo song was
exported from the running app and the file read back:

- exactly 739,523 frames, as calculated above,
- left and right are equal (every instrument is in the middle for now),
- bars 1 and 2 hold chords alone, bars 3 to 7 the beat, bar 8 the fill,
- the tail is quiet, with no chord from the song starting again.

You can do the same by opening the file in any audio editor: the shape of
the song is easy to see in the waveform.
