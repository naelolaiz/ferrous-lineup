# Exercises

## 1. An undo history window (loom-app, easy)

Add Edit > History…, a small window listing every step that can be
undone or redone, newest at the top, with the current position marked.
Clicking a step jumps there.

You need two methods on `History`: one giving the undo labels and one the
redo labels, each as an iterator (`self.undo.iter().map(|step|
step.label)`). Jumping back five steps is five calls to `history.undo`.
Should `sync_audio` run after each one, or once at the end, comparing
with the project from before the first? Which is cheaper, and does the
audio thread end up the same either way? (The diff tests from the second
chapter answer the last question.)

*Practices:* iterators over private fields, `rev()`, reusing `diff`.

## 2. Export one pattern (loom-app, easy)

Add "Export pattern…" to the ⋯ pattern menu: it saves the selected
pattern, played four times, as a WAV file.

The obvious plan is to use pattern mode: `SetMode(Pattern)`,
`SelectPattern`, and render four pattern lengths plus the tail. But
pattern mode loops the pattern forever, so the tail would hold the
pattern starting a fifth time: the same problem the export solved with
silent steps, and pattern mode has no song length to stretch. Instead,
make a *copy* of the project, replace its playlist with four clips of the
pattern on one track, and export that copy with the code you already
have. What has to change in `export_song` so it can export a project
other than `self.project`?

*Practices:* working on a copy, making a function more general.

## 3. Cancel an export (loom-audio and loom-app, medium)

Put a ✖ button next to the progress bar that stops the export, and
deletes the unfinished `.tmp` file.

The export thread has to check, between blocks, whether it should stop.
Share an `Arc<AtomicBool>` with it, as the progress counter is shared.
`export_wav` could take a second closure, or its progress closure could
return whether to go on (`impl FnMut(usize) -> bool`, or the standard
library's `ControlFlow`). What should `export_wav` return when it is
cancelled: `Ok`, an error, or a new enum? What should the status bar say?

While you are there: what happens today if you quit Ferrous Loom in the
middle of an export? Which file is left behind, and how could quitting
wait for the export, or cancel it, first?

*Practices:* atomics as signals between threads, cleaning up after
yourself, designing return types.

## 4. Stems: one file per channel (loom-app, medium)

Musicians often want "stems": the song exported once per channel, each
file holding only that channel, to mix elsewhere. Add File > Export
stems…, which asks for a folder and writes `Kick.wav`, `Clap.wav` and so
on.

Each stem is an export with every other channel muted (`SetMuted`).
Rendering them one after the other is simplest; rendering them on
several threads at once is faster on a computer with several cores.
Look up `std::thread::scope`: it starts threads that may *borrow* from
the function that started them, because the scope waits for all of them
before it ends. Why does that make the `'static` rule from the export
chapter unnecessary? How should one progress bar show several exports?
And what about two channels with the same name?

*Practices:* `thread::scope`, file names from user text.

## 5. Random edits for the diff (loom-core, medium)

The diff tests check hand-picked cases. Write one that checks a thousand
random ones: start from the demo, make a random edit (toggle a step, add
or remove a pattern or channel, change a volume, move a clip...), call
`check_both_ways(&before, &after)`, and repeat from the new project.

For randomness without any crate, a tiny "xorshift" generator is enough
for tests:

```rust
struct Random(u64);

impl Random {
    fn next(&mut self, below: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % below as u64) as usize
    }
}
```

Start it from a fixed number (not 0, which stays 0 forever), so a
failure happens again every time you run the test. If you want to go further, the `proptest` crate does this
properly, and when it finds a failing case it *shrinks* it to the
smallest edit list that still fails.

Also try several random edits between two checks, as one undo step
might hold. Does `diff` ever produce a long list where a short one would
do? Is that a problem?

*Practices:* property-based testing, reproducible randomness.

## 6. Cheaper snapshots (loom-core, harder)

Every edit copies the whole project, every pattern included, even though
a click on a step changes one pattern. Make the copies cheap by sharing
what did not change: store the patterns as `Vec<Arc<Pattern>>`.

Cloning a `Vec<Arc<Pattern>>` copies only the pointers and adds one to
each count; the patterns themselves are shared between the current
project and every copy in the history. To change one, use
**`Arc::make_mut`**, which gives a `&mut Pattern` and first makes a
private copy *only if* the pattern is shared ("clone on write"). So
`Project::pattern_mut` becomes `self.patterns.get_mut(index).map(Arc::make_mut)`.

Then `diff` can skip unchanged patterns quickly: if `Arc::ptr_eq` says
two patterns are the same memory, they are certainly equal. Make a test
project with 99 patterns full of notes, and measure the time of an edit
before and after with `std::time::Instant`. Does `PartialEq` for the
project still work unchanged? Does the file format need to change?

*Practices:* `Arc` for sharing, clone on write, measuring before
optimizing.

## 7. Think about it

- Step labels are `&'static str`, so "rename pattern" cannot say which
  pattern. A `String` label would allow "rename pattern Beat", but most
  labels are fixed text, and a `String` would allocate for each of them.
  Look up `std::borrow::Cow<'static, str>`: how does it let a label be
  either, and allocate only when it must?
- Undo while the song plays. When undo removes a channel, its ringing
  notes stop at once, because the channel goes to the trash. In the
  "three identical Loom Osc" case from the diff chapter, the audio thread
  removes the *last* channel and gives its notes to another one. Which
  channel's ringing note keeps sounding, and could you ever hear the
  difference?
- The master volume, the selected pattern and the open windows are not
  part of the project, so undo leaves them alone. Which of them would you
  expect undo to change? What would have to move into `Project` for that,
  and what would it mean for the saved file?
- The export runs at 44,100 frames per second even if your sound card
  runs at 48,000. Look at how the drums are made (`drums::render` takes
  the sample rate) and how the sampler plays a sound recorded at another
  rate. Could an export ever sound different from what you hear?
- The progress counter uses `Ordering::Relaxed`. Suppose the window
  opened the file the moment the counter reached the total, without
  waiting for `join`. What could it find? (Look at what `export_wav` does
  after its last `progress` call.) Would a stronger `Ordering` fix that,
  or is the problem somewhere else?
