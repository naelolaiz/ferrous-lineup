# Practising with the band

Files: [`crates/lineup-audio/src/metronome.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/metronome.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/history.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/history.rs),
[`crates/lineup-app/src/app/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/practice.rs)
and [`crates/lineup-app/src/chord_sheet/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/practice.rs)

A band that plays your chord sheet is most useful when you play along.
This change adds what a musician practising with a recording reaches
for: a **count-in**, a **metronome**, a slower **speed**, **tap tempo**,
a way to take one part away from the band and play it **yourself**, and
**looping** the bars you are working on.

![The chord sheet with its practice row: Count-in and Metronome lit, Speed 80%, Tap, "You play" with Bass lit, "Loop bars 2–3" lit and "✖ Stop looping"; bars 2 and 3 shaded, with a green line along their top, and bar 3 framed in white as it plays](../images/step-9-practice-sheet.png)

## Trying it

Open the chord sheet (**F5**). Under the Band and Progression buttons
there is now a second row:

- **Count-in**: when you press Play, one bar of clicks comes first, and
  the music starts on the next "one". Start in the middle of a bar and
  the count runs on to where the playhead is: from beat 3 of a 4/4 bar
  you hear "1 2 3 4 1 2", and the music comes in on 3, so the beat
  never jumps.
- **Metronome**: a click on every beat while the music plays. The first
  beat of each bar is higher (an A, 1760 Hz) than the others (the E a
  fifth below), so you always know where "one" is.
- **Speed**: how fast the band plays, as a percentage of the song's
  tempo, from 25% to 200%. The song keeps its own tempo: the TEMPO box
  still says 110, with "TEMPO 80%" over it, and a saved song or a WAV
  export is at full speed.
- **Tap**: click it on the beat a few times and the song's tempo
  becomes yours. Taps in a row are one undo step, "Undo tap tempo".
- **You play: Piano, Bass, Drums**: the band leaves that part out and
  you play it. For the piano and the bass, your keyboard (MIDI, or the
  typing keyboard) switches to that part's sound.
- **Loop bars**: click a bar, Shift+click another (or hold Shift and
  press an arrow), and **🔁 Loop bars 2–3** plays them over and over.
  **✖ Stop looping** plays the whole song again from where you are.

Count-in, Metronome and Speed are also in the new **Practice** menu, so
they work without the chord sheet, over any song or pattern. The
count-in and the metronome are remembered for next time; the speed
starts at 100% every time the app starts, so a song is never slow by
surprise.

![The Practice menu: Count-in and Metronome ticked, Speed (80%) with its list of percentages](../images/step-9-practice-menu.png)

## Where the clicks come from

The clicks are not a channel of the song. If they were, they would be
saved with it, exported into every WAV file, muted by the mixer, and
moved by the band when it rewrites its parts. They are made by the
audio thread itself, which already knows when every step starts.

What it needs to know is *what to click on each step*. That depends on
the time signature, and the time signature can change in the middle of
the song ([any time signature](../step-8/any-time-signature.md)), so
the UI thread works it out and sends the answer as a list, one entry
per step:

```rust
pub enum Click {
    None,
    Beat,
    Bar,
}

impl TimeSignature {
    pub fn click(self, step: usize) -> Click {
        if self.is_bar_start(step) {
            Click::Bar
        } else if self.is_beat_start(step) {
            Click::Beat
        } else {
            Click::None
        }
    }
}
```

"Beat" means the *felt* beat, the one you tap your foot to, which
`beat_starts` already knew from step 8:

| Time | Clicks in a bar | Why |
|------|-----------------|-----|
| 4/4 | 1 2 3 4 | four quarter notes |
| 3/4 | 1 2 3 | a waltz |
| 6/8 | 1 2 | two dotted quarters: six eighths felt as two groups of three |
| 2+2+3/8 | 1 2 3 | three beats, the last one longer |

So a metronome in 6/8 does not click six times a bar. A jig, or Silent
Night in 6/8, is felt in two, and clicking every eighth note would
hide that.

The song's list comes from `Project::played_clicks`, which counts bars
as the song is *played*: a section in 3/4 that repeats is in 3/4 both
times round, and a loop starts its count at the loop's first bar. The
selected pattern's list comes from `Project::pattern_clicks`. They are
sent as two new commands, `SetSongClicks` with the song and
`SetPatternClicks` whenever another pattern is selected or its length
or time signature changes.

> **Rust: the old list goes back.** When a new list arrives, the audio
> thread swaps it in with `std::mem::replace` and sends the old `Vec`
> back to the UI thread as `Garbage::Clicks`, to be freed there. Freeing
> memory can take a lock inside the allocator, and the audio thread must
> never wait ([The audio thread, grown up](../step-2/realtime.md)).

### The click itself

`ClickVoice` in `metronome.rs` plays a short sine wave that dies away:

```rust
const RING: f32 = 0.03;

decay: (-6.9 / (RING * sample_rate)).exp(),
```

Each frame the level is multiplied by `decay`. After `RING` seconds
(30 ms, 1440 frames at 48 kHz) it has been multiplied by `decay` 1440
times, which is e<sup>−6.9</sup> ≈ 0.001, a thousandth of where it
started: −60 dB, too quiet to hear. Long enough to hear a pitch, short
enough to be over well before the next beat even at 300 BPM.

## The count-in: a second clock

The audio thread has one `Sequencer`, the clock that says when each
step starts ([The sequencer: timing to the sample](../step-2/sequencer.md)). It
also drives the playhead, which the UI reads to draw where the song
is. During a count-in the song has not started, so the playhead must
not move. The count-in gets a clock of its own, `counter`, and a small
record of how far it has got:

```rust
struct CountIn {
    bar: usize,     // where the bar it counts starts, in the clicks
    length: usize,  // how many steps that bar has
    steps: usize,   // how many steps the whole count lasts
    counted: usize, // how many have gone by
}
```

Play starts it:

```rust
fn begin_play(&mut self) {
    if !self.count_in {
        self.sequencer.start_at(self.start);
        self.playing = true;
        return;
    }
    // …
    let (bar, length) = count_in_bar(self.loop_clicks(), self.start);
    let into_the_bar = self.start.saturating_sub(bar) % length;
    self.counter.set_tempo(self.tempo_of_step(self.start));
    self.counter.reset();
    self.counting = Some(CountIn {
        bar,
        length,
        steps: length + into_the_bar,
        counted: 0,
    });
}
```

`count_in_bar` looks back from the playhead to the last `Click::Bar`,
and on to the next, to find the bar the playhead is in. The count lasts
that whole bar *plus* the steps from its start to the playhead: from
step 6 of a 16-step bar, 16 + 6 = 22 steps, "1 2 3 4 1 2", and the
music starts on step 6, half a beat into beat 2. It counts at the tempo
where the music will start, so a song that begins slowly is counted in
slowly.

Then, on every frame:

```rust
if let Some(count) = self.counting {
    self.count_in_frame(count);
}
```

```rust
fn count_in_frame(&mut self, mut count: CountIn) {
    if self.counter.tick().is_none() {
        return;
    }
    if count.counted == count.steps {
        self.counting = None;
        self.sequencer.start_at(self.start);
        self.playing = true;
        return;
    }
    let step = count.bar + count.counted % count.length;
    self.click.strike(click_at(self.loop_clicks(), step));
    count.counted += 1;
    self.counting = Some(count);
}
```

When the count is over, the song's own sequencer starts *on that same
frame*: the frame loop handles the count-in before it asks the song's
sequencer for a step, so the first note lands exactly one bar after
the first click. The test `the_count_in_comes_before_the_music` checks
it to the frame: clicks on frames 0, 24000, 48000 and 72000 at 120 BPM,
the first note on 96000.

> **Rust: why `count` is passed by value.** `CountIn` derives `Copy`,
> so `if let Some(count) = self.counting` makes a copy, and
> `count_in_frame` gets the copy. If it took a reference into
> `self.counting` instead, that borrow of `self` would still be alive
> while the function calls `self.counter.tick()` and
> `self.click.strike(…)`, which need `&mut self`: the borrow checker
> would refuse. Working on a copy and writing it back at the end
> (`self.counting = Some(count)`) is the usual way out for small
> state like this.

Stop, during the count-in, ends it (`stop` sets `counting` to `None`).
Moving the playhead or switching between pattern and song mode during
the count starts it again, counting the bar playback will now start in.

Keys played during the count-in are not recorded: the song's position
is "none" until the music starts, and recording only keeps notes that
have a position.

## The speed

The practice speed never touches the project. `Project::tempo` and
the tempo markers stay as they are; only what the audio thread is told
changes:

```rust
pub(super) fn at_speed(tempo: Tempo, speed: f32) -> Tempo {
    if speed == 1.0 {
        tempo
    } else {
        Tempo::new(tempo.bpm() * speed)
    }
}
```

Every command that carries a tempo goes through it: `SetTempo` after
an edit, and the tempo of every step of the song, so a song that slows
down at the end still slows down, from 80% of its tempo. A WAV export
builds its commands at speed `1.0`, which is why it is never slowed.

## Tap tempo

Tap tempo is arithmetic on the times of the clicks. Taps at 10.0,
10.5, 11.0 and 11.5 seconds are three beats in 1.5 seconds:

> BPM = 60 × beats ÷ seconds = 60 × 3 ÷ 1.5 = 120

Using the first and the last tap of the run, rather than only the last
two, averages out the unevenness of a human hand. Only the last eight
taps count, so a tempo that settles as you tap is followed, and a pause
of more than two seconds starts a new run. The result is rounded to a
whole BPM.

Each tap is a click, and each click normally starts a new undo step
(see [Undo and redo: a history of copies](../step-5/history.md)). Undoing a tempo tap by tap
would be tiresome, so `History` gained one method:

```rust
pub fn reopen(&mut self, label: &'static str) -> bool {
    if self.redo.is_empty() && self.undo_label() == Some(label) {
        self.open = true;
    }
    self.open
}
```

From the third tap of a run on, the app reopens the "tap tempo" step
before changing the tempo, so the whole run is one step. It only does
so when the last step *is* "tap tempo" and nothing was undone since:
it never joins a tap to some other change.

## You play

Taking a part away from the band mutes that player's track in the
playlist:

```rust
pub fn set_you_play(&mut self, player: Player, you: bool) -> bool {
    let Some(track) = self.player_track(player) else {
        return false;
    };
    self.playlist.tracks[track].muted = you;
    true
}
```

Muting, not removing, matters. The band goes on writing the part when
you change a chord, so when you click the button again it plays what
the sheet says *now*. And because a part's fingerprint leaves out the
track its clips are on ([A part you edit stays
yours](band.md#a-part-you-edit-stays-yours)), muting does not count as
an edit: the part is not frozen.

For the piano and the bass, the app also selects that player's
channel, the one your keyboard plays: you take over the bass *with*
the bass's sound. The drums play on several channels (kick, snare,
hat…), so there is no one channel to select for them.

## Looping the bars you choose

The sheet remembers, besides the bar selected, an `anchor`: the other
end of a selection made with Shift.

```rust
fn select_to(&mut self, bar: usize, shift: bool) {
    self.anchor = match (shift, self.anchor, self.selected) {
        (false, ..) => None,
        (true, Some(anchor), _) => Some(anchor),
        (true, None, selected) => selected,
    };
    self.selected = Some(bar);
}
```

The first Shift+click keeps the bar selected before as the anchor;
later ones move only the other end, as in a text editor. Delete clears
every bar selected, in one undo step.

**Loop bars** turns the selection into the playlist's loop region,
which song mode already knew how to play
([Song structure: sections, repeats and loops](../step-8/song-structure.md)), switches to
song mode and moves to the loop's start. Bars past the end of the song
cannot be looped: there is nothing there to play. On the sheet, the
bars that loop get a green line along their top, the colour of the
loop on the playlist.

## Exercises

1. Turn on the metronome over Silent Night, a demo in 6/8. How many
   clicks are there in a bar, and where does the high one fall? Now
   Greensleeves, in 3/4: the same six eighth notes to a bar, counted
   differently.
2. With the count-in on, play a song and pause it (the ❚❚ button)
   halfway through a bar, then press Play again. Write down what you
   hear before the music. Why is it better than counting one whole bar
   and then starting halfway through the next?
3. Set the speed to 50% and export a WAV file. How long is it,
   compared with the song at full speed? Why?
4. Tap 4 times at an even pace, then 4 more times faster without
   stopping. Which tempo do you get, and why not the average of all 8?
5. Take over the bass, then change a chord. Give the bass back: is the
   new chord in its part?
6. (Rust) `ClickVoice::strike(Click::None)` returns at once. What would
   you hear if it set `level` to 0 instead?
7. (Rust) In `begin_play`, what goes wrong if `self.stop()` is not
   called when Play arrives while the song is already playing?
