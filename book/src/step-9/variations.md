# Variations a and b: the band digs in

Files: [`crates/lineup-core/src/band/variation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/variation.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
and [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)

A band does not play every part of a song the same way. The verse is
lighter, so the words come through; the chorus is fuller, and the
drummer, the bassist and the pianist all lift at once. A jazz group
plays the first chorus of a ballad gently and then starts to swing
harder. Until now the band in Ferrous Lineup played the whole song with
one energy. Now every style has two variations: **a** is the style as
it was, and **b** is the same style played fuller.

![The chord sheet of an eight-bar pop ballad. Bar 5 has a boxed "b". Its menu is open, with Variation a and Variation b, b chosen, and the hint: "From bar 5 on, the band plays the style fuller: the drums on the ride cymbal, a busier bass or piano where the style has one, a little louder. The drums fill into it"](../images/step-9-variations-sheet.png)

## Trying it

Right-click a bar on the chord sheet (**F5**) and pick **Variation a**
or **Variation b**. The band plays that variation from that bar up to
the next change, or to the end of the song. To change only some bars,
select them first: click the first, Shift+click the last, then
right-click one of them. The selection stays, and the bars after it
keep what they played before. That is how you play only the bridge of
a song fuller, or only the last chorus.

A boxed letter marks each change, on the sheet after the bar number
and on the playlist's bar ruler, like the rehearsal letters printed
over the staff in band parts. Picking the variation that is already
playing puts no marker; changing a bar back takes the marker away.

The drums announce each change: the bar before it is a **fill**, and
the first beat of the new variation gets a **crash** cymbal. This is
the same signal real drummers give, and why a listener knows that
"something new starts here" even without hearing the words.

![The playlist of the same ballad: bars 1 to 4 and 5 to 8 are separate patterns for each player, and the ruler shows a boxed "b" at bar 5. Bass 2 has a note on every beat, and Drums 2 is busier](../images/step-9-variations-playlist.png)

## What "fuller" means

Variation b does not invent a new style. It opens the same one up, the
way musicians do it:

| Who | Variation a | Variation b |
|---|---|---|
| Drums | time on the closed hi-hat | time on the **ride** cymbal, unless the style keeps time on the ride already |
| Drums, in a pop or rock ballad | **cross-stick** on the snare's rim | the full snare |
| Bass, a jazz style played in two, in 4/4 | **in two**: root and fifth, a note every two beats | **walking**: a note on every beat |
| Bass, blues shuffle | quarter notes | the **boogie** line, up and down the chord |
| Bass, pop, rock, soul or folk in 4/4 | the root held | the root on every beat |
| Piano, pop or rock ballad | the chord, then its notes one by one | steady eighth notes |
| Everyone | | a little louder: each note's velocity a tenth higher |

A few words from that table:

- The **hi-hat** is two cymbals that the drummer closes with a pedal.
  Closed, it gives a short, dry "tick". The **ride** is one large
  cymbal that rings; eighth notes on it make a shimmering wash that
  fills more of the sound. Moving from one to the other is the most
  common way a drummer lifts a chorus.
- A **cross-stick** is the stick laid across the snare drum, its end
  clicking on the rim: a soft, woody sound. Hitting the snare's head
  instead is much louder. Ballads are full of cross-stick verses and
  full-snare choruses.
- A jazz bassist **in two** plays half notes, usually the root and the
  fifth, which leaves room around a quiet melody. **Walking** fills
  every beat with a line that leads to the next chord. Going from "two"
  to "four" is how a jazz rhythm section turns up the heat without
  playing louder.

Where a style has no well-known busier way to play (a tango, a bossa
nova, a chorale), variation b keeps its patterns and only changes the
drums and the loudness. The rules are deliberately few, so that b
always sounds like the same band, not like a different song.

## How it works

### Markers that only mark changes

```rust
pub struct VariationMarker {
    pub start: usize,
    pub variation: Variation,
}
```

The markers live in the `Band`, beside its `Form`, because only a band
plays variations. A marker that changes nothing (variation a at the
start of the song, or b right after another b) would only clutter the
sheet, so after every change `tidy` keeps only the real ones:

```rust
let mut playing = Variation::A;
markers.retain(|marker| {
    let changes = marker.variation != playing;
    playing = marker.variation;
    changes
});
```

The closure passed to `retain` changes `playing`, a variable outside
it. A closure that changes what it captures is an `FnMut`: it can be
called many times, but not from two places at once, because each call
needs the variable to itself. `retain` asks for exactly that.

Which variation plays at a step is the last marker at or before it:

```rust
markers
    .iter()
    .rev()
    .find(|marker| marker.start <= step)
    .map_or(Variation::A, |marker| marker.variation)
```

`rev()` walks the list backwards, so `find` stops at the first marker
that is not after the step.

### Playing in runs

The piano and the bass used to play the whole song in one call:
`style.comping.play_in_bars(&bars, &harmonies)`. Now the bars are
grouped into **runs**, stretches in one variation, and each run is
played with that variation's way of playing:

```rust
fn played_in_runs(
    runs: &[Run],
    harmonies: &[Played],
    play: impl Fn(Variation, &[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent>
```

The piano passes `|variation, bars, played|
style.comping_in(variation).play_in_bars(bars, played)`. Each run gets
only its own harmonies, cut at its edges with `cut`, the same function
the ending uses. A song with no markers is one run, played exactly as
before, so no song changed its sound with this feature.

`comping_in` and `bass_in` are small `match`es on the variation, the
style's genre and its way of playing:

```rust
match (variation, self.genre, self.bass) {
    (Variation::B, Genre::Jazz, Bass::RootFifth) if common => Bass::Walking,
    (Variation::B, Genre::Blues, Bass::Quarters) if common => Bass::Boogie,
    // ...
    (_, _, bass) => bass,
}
```

Matching on a tuple lets one line name a whole situation, and the last
arm keeps everything else as it was.

### Drums on other drums

The drums keep the rhythm's patterns and change which drum plays them:
`drum_in(Drum::HiHat, Variation::B)` is `Drum::Ride` when the rhythm
has no ride of its own. The band writes each pattern into the channel
of the drum that should sound, so the hi-hat's eighth notes land on the
ride's channel. A channel the song does not have yet (a ride, a crash)
is added the first time a variation needs it, and never otherwise:
songs without markers keep exactly the channels they had.

Each change of variation also starts a new **chunk**, the stretch one
pattern covers, so the playlist shows the a and b parts as separate
clips you can see, mute or edit.

### Markers move with the song

Everything that moves the song moves the markers too: an intro put in
front (`make_room`), an intro taken away (`cut_start`), a section
copied (`copy_section`, which copies the variations inside it), a new
time signature (`remeter`), and "End the song here", which drops the
markers after the end. A song with markers is saved as format 15.

## Exercises

1. Choose the **Jazz ballad** style, write | Cmaj7 | Am7 | Dm7 | G7 |
   twice, and set bars 5 to 8 to variation b. Open the bass's patterns
   in the piano roll. How many notes does a bar have in a, and in b?
   Which notes does the bass play in a bar of Dm7 in variation a?
2. In an eight-bar **Pop beat**, set bar 5 to variation b and bar 6
   back to a. Which bars get a drum fill, and where do the crashes
   fall? Why could that sound restless in a real song?
3. Choose **Swing** and play a few bars in each variation. What changes
   and what stays? Why does the hi-hat stay a hi-hat here?
4. (Rust) `tidy` first sorts the markers, then uses `dedup_by` and then
   `retain`. What would go wrong if it called `retain` before sorting?
5. (Rust) `played_in_runs` takes `play: impl Fn(...)`. Could it take an
   `FnMut` instead? What would change at the places that call it?
