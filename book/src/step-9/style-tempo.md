# Styles keep the song's tempo

Files: [`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs),
[`crates/lineup-core/src/remeter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/remeter.rs),
[`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs),
[`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)
and [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

Open El Choclo (112 BPM), apply **Tango de salón** from the Library,
then **Tango nuevo**. Both are tangos in 4/4, and nothing about the
song's time signature changes, yet the tempo box went from 112 to 132.
Every note of the song got shorter: the melody you know rushed, only
because you wanted to hear another accompaniment.

That is not how a backing band behaves. A band asked to "play it as a
tango nuevo" keeps the singer's tempo. Now Ferrous Lineup does too: **a
style keeps the song's tempo**, and the song's own notes last as long
as before, in seconds, whatever style you try.

![Tango nuevo's options: Tempo is not ticked, and the tempo still comes from El Choclo (demo).](../images/step-9-style-tempo-popup.png)

## Trying it

1. Open **File > Demos > El Choclo**. The tempo box says **112.0**.
2. In the **Library**, click **Tango de salón**, then **Apply**. Click
   **Tango nuevo**, then **Apply**. The tempo stays at 112.
3. Click **Tango vals (3/4)** and **Apply**. The song is now in 3/4
   and the tempo box says **84.0** (see below why that is the same
   pace).
4. Apply **Tango nuevo** again: 4/4, and **112.0**, exactly where you
   started.

If you *want* a style's own tempo, tick **Tempo** in its options before
**Apply**. It is used where the time signature stays the same.

## Where the jumps came from

Every Library choice has a list of parts it can bring: the playing
patterns, the instruments, the time signature, the chords, the tempo
and the swing. They are the fields of `StyleParts` in `apply.rs`, and
the tick boxes in each style's options. `tempo` started **ticked**, so
every style brought its "usual tempo": 112 for Tango de salón, 132 for
Tango nuevo, 180 for Tango vals.

The ticks are also remembered between runs, so the ticked Tempo came
back every time the app started. And with time signature changes in
between, the jumps added up. A song changing meter keeps each bar as
long as it was (more below), and the next style in the new meter took
its own tempo from there. In one run, El Choclo went through a few
styles and came back to 4/4 at **240** BPM: more than twice its speed.

## A meter change keeps the pace

The tempo counts quarter notes. A bar of 4/4 has four, a bar of 3/4
three. When a style in 3/4 puts the song in 3/4 (bar n of the song
becomes bar n of the waltz, see
[re-metering](../step-8/re-metering.md)), the tempo goes down in the
same proportion, so that each bar lasts exactly as long as before:

```text
4/4 at 112:  4 beats × 60/112 s = 2.14 s a bar
3/4 at  84:  3 beats × 60/84  s = 2.14 s a bar
```

So 84 in 3/4 is not a slower song: it is the same song, counted in
three. Each note keeps its place in its bar, so it starts at the same
moment and lasts as long. The [playback trace](playback-trace.md)
shows it. These are the first melody notes of El Choclo, start + length
in seconds:

```text
4/4 at 112:  1.74+0.13  1.88+0.13  2.01+0.13  2.14+0.13 …   song 98.9 s
3/4 at  84:  1.74+0.13  1.88+0.13  2.01+0.13  2.14+0.13 …   song 98.9 s
```

What changes is how the notes are written, not how they sound: a 3/4
bar has twelve steps instead of sixteen, so a note covers fewer steps on
the piano roll, and each step lasts longer. The drums and the
accompaniment are what make it sound like a waltz.

![El Choclo after Tango vals: 3/4, tempo 84.](../images/step-9-style-tempo-waltz.png)

## What changed in the code

### The default

```rust
impl Default for StyleParts {
    fn default() -> Self {
        Self {
            rhythm: true,
            instruments: true,
            time_signature: true,
            progression: false,
            tempo: false, // was true
            swing: true,
            matching_style: true,
        }
    }
}
```

The struct also has `#[serde(default)]`. When it is read back from the
saved settings, a field missing there takes its value from
`Default::default()`. So a new field can be added without breaking
settings saved by an older version.

### Settings saved with Tempo ticked

Changing the default is not enough: your settings file still says
`tempo: true`, saved because it was ticked for everyone. The app saves
its preferences with egui, under a key (an `egui::Id`). The fix moves
them to a new key, `"library-options"`. When that key is empty, the app
reads the old one, `"preset-copy-options"`, once, and turns Tempo off:

```rust
self.parts = ctx
    .data_mut(|data| {
        data.get_persisted(egui::Id::unique(OPTIONS)).or_else(|| {
            data.get_persisted(egui::Id::unique(OPTIONS_TEMPO_ON))
                .map(|parts| StyleParts {
                    tempo: false,
                    ..parts
                })
        })
    })
    .unwrap_or_default();
```

Two pieces of Rust do the work here. `Option::or_else` runs its closure
only when the first lookup found nothing. `StyleParts { tempo: false,
..parts }` is *struct update syntax*: a copy of `parts` with one field
changed. Every other choice you made (plain accents, the chords, the
swing) is kept. If you tick Tempo again, it is saved under the new key
and stays ticked.

### Pattern mode follows the same rule

The Tempo box says: "Use the preset's usual tempo when the meter stays
the same." In song mode that was already true. In pattern mode, a
style tried in a trial pattern took its tempo even when the pattern
went into another meter. Now the code compares the pattern's time
signature before and after writing it, and takes the tempo only if it
is the same:

```rust
let time = project.pattern_time(index);
// … the style is written into the pattern …
if parts.tempo && project.pattern_time(index) == time {
    project.scale_tempo(Tempo::new(tempo));
}
```

### 124, not 124.00001

A tempo is an `f32`, a floating-point number with about seven
significant digits. Re-metering used to multiply the tempo by the ratio
of the bar lengths, `16.0 / 14.0` for 7/8 to 4/4. That ratio has no
exact `f32` value (it is 1.142857…, forever), so 108.5 BPM in 7/8 came
back to 4/4 as 124.00001 instead of 124. Nobody hears the difference,
but the song is no longer exactly where it started. The fix multiplies
first and divides last, in `f64`:

```rust
let scaled = |tempo: Tempo, step: usize| {
    Tempo::new((f64::from(tempo.bpm()) * bar as f64 / old_bar(step) as f64) as f32)
};
```

`108.5 × 16 = 1736` is exact, and `1736 / 14 = 124` is exact too. A
song put into 7/8, 3/4 or 13/16 and back now comes back at exactly its
tempo. A test checks this for several tempos and meters.

## The tests

`styles_tried_one_after_another_keep_the_songs_tempo` takes El Choclo
through seven changes: Four chords, Emotional pop, Tango vals, Ranchera,
Six-eight ballad, the Time box (7/8) and Four chords again. It does this
three times: in pattern mode, in song mode while stopped, and in song
mode while playing. After each click, the BPM must be unchanged if the
meter is, and the song's own BPM whenever it is back in 4/4. Then the
test plays the start of the song with the trace on and checks that
every melody note starts and lasts as it did, to within 2 ms.

Playing a whole song for a trace takes a few seconds, and much more in
a debug build, the way CI runs the tests. So the test plays only the
first twelve seconds, at 8000 samples a second instead of 48000. That
is plenty to time notes, and the test runs in about six seconds in a
debug build.

One thing still moves the song's own notes on purpose: **swing**. A
swing style plays the second eighth of every beat late, in the melody
too (see [Swing](swing.md)). Whether swing should only apply to the
band is a separate question.
