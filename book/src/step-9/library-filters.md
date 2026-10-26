# Library tick boxes: presets like the song, and one helper to try them

Files: [`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs),
[`crates/lineup-app/src/library_browser/catalogue.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser/catalogue.rs),
[`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)
and [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

The Library holds a few hundred presets: harmony styles, rhythms,
your own styles, fills, twelve-tone row styles and micromode styles.
The search box finds one by name, genre or time signature. But often
the question is not "where is the bossa nova?". It is "which of these
would fit **this** song?". A waltz won't, if the song is in 4/4; the
Drum and bass beat, made for 174 BPM, won't fit a ballad at 70.

Three tick boxes under the search now narrow the list to presets like
the song:

- **In 4/4**: only presets in the song's time signature, and those that
  play every bar in its own meter (like Orchestra). The label shows
  the song's time signature, whatever it is.
- **Near 112 BPM**: only presets made for about the song's speed, at
  most 1.15 times faster or slower. The label shows the song's tempo.
- **With chords**: only presets that bring a chord progression. It is
  hidden where every preset brings one, as in the chord sheet's
  Progression menu.

![The Library for El Choclo with In 4/4 and Near 112 BPM ticked: 72 presets are left.](../images/step-9-library-filters.png)

## Trying it

1. Open **File > Demos > El Choclo** (4/4, 112 BPM) and the **Library**
   tab. The heading says **Presets** and how many there are.
2. Tick **In 4/4**. The waltzes and the 6/8 styles go, and the count
   goes down.
3. Tick **Near 112 BPM**. What is left was made for 97 to 129 BPM.
4. Set the tempo to 20 and look again: nothing is made for that, so the
   Library says "Nothing in the library matches the ticked boxes" and
   "Untick a box to see more".

The ticks are saved with the app's state, so they are still there the
next time you start it. They narrow the fills, the row styles and the
micromode styles too.

## Narrowing, in one struct

The ticks are three `bool`s:

```rust
#[derive(Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct Filters {
    /// In the song's time signature.
    time: bool,
    /// Made for about the song's speed.
    speed: bool,
    /// Bringing a chord progression.
    chords: bool,
}
```

`Default` gives all three `false`, nothing ticked. `PartialEq` lets the
browser compare the ticks before and after drawing the boxes, and save
them only when they changed. `Serialize` and `Deserialize` let egui save
them with `insert_persisted`.

To decide, the ticks need what they compare against, the song's time
signature and tempo. A second struct holds the ticks and those two
together, and answers one question:

```rust
impl Narrowing {
    fn keeps(self, time: TimeSignature, tempo: f32, any_meter: bool, chords: bool) -> bool {
        let filters = self.filters;
        // 7/8 as 2+2+3 is still 7/8.
        let same_time = any_meter || time.ungrouped() == self.time.ungrouped();
        let ratio = tempo / self.tempo;
        let near = ratio.max(1.0 / ratio) <= NEAR_SPEED;
        (!filters.time || same_time) && (!filters.speed || near) && (!filters.chords || chords)
    }
}
```

Read the last line as three rules joined by "and": `!filters.time ||
same_time` says "either the box is not ticked, or the time is the
same". A box that is not ticked lets everything through.

Three details:

- **Grouped meters.** A time signature can carry its grouping, as
  "7/8 (2+2+3)" (see [Any time signature](../step-8/any-time-signature.md)).
  The grouping says where the accents fall, not how long the bar is, so
  `ungrouped` drops it before comparing.
- **Faster or slower, the same way.** `ratio.max(1.0 / ratio)` turns
  "half as fast" (0.5) and "twice as fast" (2.0) into the same number,
  2. So a preset 1.15 times slower and one 1.15 times faster are both
  just inside. At 112 BPM that is from 112 / 1.15 ≈ 97 to 112 × 1.15 ≈
  129. Comparing ratios rather than a difference in BPM treats a slow
  song and a fast one alike: ±15 BPM would be a lot at 60 and little
  at 180.
- **`self`, not `&self`.** `Narrowing` is `Copy`: three ticks, a time
  signature and an `f32`, a few bytes. Taking it by value costs no more
  than a reference, and calling `keeps` doesn't use it up: a `Copy`
  value is copied, so the next preset can be checked with it too.

The same `keeps` filters every list. The presets are filtered with
`Vec::retain`, which keeps the items a closure says yes to, in place:

```rust
presets.retain(|preset| {
    narrowing.keeps(
        preset.time(),
        preset.tempo(),
        preset.follows_the_song(),
        preset.has_progression(),
    )
});
```

The fills, the row styles and the micromode styles are filtered as they
are listed, with `Iterator::filter`. When a search and the ticks together
leave nothing, the browser says which one did it: "Nothing in the
library matches" the search, or "the ticked boxes", with "Untick a box
to see more".

## One helper to try them: a small refactoring

Clicking a preset in pattern mode **tries** it: the Library writes it
into a pattern of its own and plays it there, so the song is not
touched. There are three kinds of preset this can happen with, harmony
styles, twelve-tone row styles and micromode styles, and each had its
own copy of the same five steps:

1. select the pattern just written,
2. remember what was written there, so the next click rewrites the same
   pattern while you haven't changed it (keeping only the last 64),
3. switch to pattern mode,
4. rewind,
5. and play, if not playing already.

Three copies of the same steps are three places to fix the next time
one changes, and sooner or later one is forgotten. The steps are now
one method:

```rust
/// Selects `pattern`, which the library has just written, and plays
/// it on its own. `audition` remembers what was written there, so
/// that the next click rewrites the same pattern while it is
/// unchanged; only the last [`REMEMBERED`] are kept.
fn play_written_pattern(&mut self, pattern: usize, audition: Option<Audition>) {
    self.select_pattern(pattern);
    if let Some(audition) = audition {
        if self.auditions.len() == REMEMBERED {
            self.auditions.remove(0);
        }
        self.auditions.push(audition);
    }
    self.set_mode(PlayMode::Pattern);
    if !self.playing {
        self.rewind_for_new_pattern();
        self.set_playing(true);
    }
}
```

The row styles and the micromode styles shared more: the whole write.
Pick the pattern (the one written last if it is unchanged, or a new
one), clear it, write the notes, the drums and the tempo, and say so
when there is no room for another pattern. Only "write the notes" is
different between the two: a row style writes the forms of a
twelve-tone row, a micromode style writes micromodes. So that one step
became a parameter, a **closure**:

```rust
fn write_pattern_style(
    &mut self,
    name: &'static str,
    tempo: f32,
    rhythm: Option<&'static Rhythm>,
    write: impl FnOnce(&mut Project, usize),
) -> Option<(usize, Pattern)> {
    // … pick the pattern, clear it …
    write(project, index);
    // … the drums and the tempo …
}
```

and the two callers pass what is theirs:

```rust
self.write_pattern_style(style.name, style.tempo, style.rhythm(), |project, index| {
    // write the micromodes of `style` into pattern `index`
});
```

`impl FnOnce(&mut Project, usize)` accepts any closure that can be
called once with those two arguments. The compiler makes a separate copy
of `write_pattern_style` for each closure passed to it, so the call
costs no more than if the code were written out. `FnOnce` is the least
demanding of the three closure traits (`Fn`, `FnMut`, `FnOnce`): it
only promises to be called once, so the closure may move values out of
what it captured. Any closure is at least `FnOnce`, so asking for it
accepts every closure.

Nothing the user sees changed, which is the point of a refactoring. But
something had to show that nothing changed. The harmony styles' path
had tests; the row and micromode path had none. So the change came with
`trying_row_and_micromode_styles_rewrites_one_pattern_while_it_is_unchanged`.
It tries two row styles and then a micromode style, checks that they
all land in the same new pattern while it is as the Library wrote it,
that it plays in pattern mode, and that each try is one step to undo.

## The tests

`the_library_tick_boxes_keep_only_presets_like_the_song` sets the song to
4/4 at 120 BPM and counts the presets the Library lists: all of them,
then those in 4/4 (a waltz goes, Orchestra stays), then those in 4/4
made for 104 to 138 BPM. It starts the app again at 10 BPM, checks the
boxes are still ticked, that nothing is listed and that the Library
says why, and that unticking the speed brings the 4/4 presets back.

## Exercises

1. Tick **Near … BPM** on a song at 60 BPM. What range does it keep?
   Then at 180. Why would ±15 BPM have been worse?
2. Add a fourth box, **Same genre as the song's style**, that keeps
   presets of the genre of the style the band plays. What would
   `Narrowing` need to know, and where would you get it from?
3. `write_pattern_style` takes `impl FnOnce`. Would `impl Fn` work for
   the two closures passed to it today? What could a later closure do
   that `Fn` would refuse?
