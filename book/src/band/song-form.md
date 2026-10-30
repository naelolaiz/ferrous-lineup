# Song form, sections, variations and locks

Files: [`crates/lineup-core/src/band/form.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/form.rs),
[`crates/lineup-core/src/band/variation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/variation.rs),
[`crates/lineup-core/src/band/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/sections.rs),
[`crates/lineup-core/src/band/locks.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/locks.rs),
[`crates/lineup-core/src/lock.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lock.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/arrangement.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/arrangement.rs),
[`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs),
[`crates/lineup-app/src/chord_sheet/form.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/form.rs),
[`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/sections.rs)
and [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs)

A band does not play a tune once, the same way from the first bar to
the last, and stop dead. It plays a few bars to set the key and the
groove, then the tune, maybe several times, and then it ends on the
home chord. It plays the verse lightly and digs into the chorus. A
Latin jazz tune may play its verse as a bossa nova and its bridge in
swing. And if you play the piano part yourself, the band plays around
you.

This chapter is about those four tools: the **form** (an intro,
choruses and an ending), the two **variations** of every style, a
**style for each section**, and **locks**, which keep a track out of
the styles' way. The first three live on the chord sheet; the locks
live in the channel rack.

## The form: an intro, choruses and an ending

![The chord sheet with its form row: "Intro: 4 bars", Choruses 3, Ending lit. The sheet reads Intro (C, Am7, Dm7, G7), then "Chorus ×3" between repeat signs (C, Am, F, G7), then Ending (C, then C again with "held" and the final bar line)](../images/step-9-song-form-sheet.png)

### Trying it

Write a few bars of chords on the sheet (**F5**), or pick a
progression. Under the sheet's buttons there is a row named **Form**:

- **Intro**: No intro, 2, 4 or 8 bars. The band puts that many bars
  in front of the song, with chords that lead into its first chord.
  They are written on the sheet under the heading **Intro**, so you can
  read them, hear them (hold one down with the mouse) and change them.
  Picking a length again writes them anew; **No intro** takes them
  away. The rest of the song, melody and all, moves along to make room.
- **Choruses**: how many times the band plays the tune. The sheet shows
  it once, under **Chorus ×3**, between the repeat signs of printed
  music: ‖: at the start, :‖ at the end. While it plays, the form row
  shows which time round it is, "▶ 2 of 3".
- **Ending**: two bars after the last chorus. The band goes to the
  key's home chord, plays one bar of it in the style, and then strikes
  it once and holds it for the whole last bar, with a crash of the
  cymbal. The two bars are shown in grey under **Ending**, the last
  one marked "held", with the final double bar line after it.

The playlist shows the repeat too: along the bottom of the section
strip, the same repeat signs at the two ends of the chorus, and "×3".

![The playlist: the band's Piano, Bass and Drums tracks over bars 1 to 10, with repeat signs on the section strip around bars 5 to 8 and "×3"](../images/step-9-song-form-playlist.png)

### "Chorus" means one time through

In a pop song the chorus is the part that comes back after each verse
(in "Let It Be", "When I find myself in times of trouble" starts a
verse and "Let it be, let it be" is the chorus). Jazz musicians use
the word differently: a **chorus** is one time through the whole tune,
from the top of the chart to its last bar. A soloist "takes two
choruses"; the band plays "the head" (the melody) for one chorus, then
solos for several, then the head again. The form row uses the word
that way: one chorus is every bar on the sheet after the intro. A pop
song's verses and choruses are all inside one chorus here. If your
song has sections of its own (Verse ×2, Chorus) on the playlist, they
repeat as they always do, inside each time round.

### The intro: a turnaround

A **turnaround** is the handful of chords at the end of a tune that
"turn it around" and lead back to the top. It is the most natural
intro there is: it leads into the first chord, so when the tune starts
it sounds like an arrival, not a beginning out of nowhere.

The engine of a turnaround is the **dominant seventh**. In C major, G7
(G B D F) holds two notes that want to move: B, a semitone under C,
pulls up to C, and F, a semitone over E, pulls down to E. Played before
C, G7 makes C sound like home. And the chord a fifth above G, D minor,
leads to G the same way G leads to C: roots that fall by fifths
(A → D → G → C) are the strongest way chords can move.

| Intro | Into C major | Into A minor |
|---|---|---|
| 2 bars | Dm7, G7 (ii7 V7) | Bm7b5, E7 (iiø7 V7) |
| 4 bars | C, Am7, Dm7, G7 (I vi7 ii7 V7) | Am, Fmaj7, Bm7b5, E7 (i bVImaj7 iiø7 V7) |
| 8 bars | the four chords, two bars each | the same, two bars each |

Into a minor chord the ii is **half-diminished** (B D F A over Am):
that is the chord that is really on the second note of A minor. The V
is still a dominant seventh, E7, with G sharp: the **leading note** a
semitone under A, borrowed from the harmonic minor scale, because a
plain E minor would not pull to A nearly as hard.

The numerals are counted from **the chord the intro leads to**, as if
it were the tonic, not from the key of the song. Most songs start on
their tonic, and then the two are the same. A song in C that starts on
F (IV) gets Gm7, C7 for a two-bar intro: the ii and V *of F*, so the
intro still leads straight into the first chord. A blues that starts
on C7 gets C7, Am7, Dm7, G7. The chords are spelled from the letter
the first chord is written with: into Ebm the four-bar intro is Ebm,
Cbmaj7, Fm7b5, Bb7, not the same sounds written from D sharp.

A pitch-class set, a micromode or a row form has no dominant to lead
into it, so the intro plays it from the first bar: a **vamp**, the
band setting up the sound before the tune comes in. So does a
diminished chord, which no turnaround resolves to.

### The ending: home, and held

The ending goes to the **tonic**, the chord the whole key leans
towards. Which tonic chord? If the tune starts on the tonic, the
ending plays that very chord: a blues in C that starts on C7 ends on
C7, a ballad that starts on Cmaj7 ends on Cmaj7. Otherwise it plays
the key's own triad, C in C major and Am in A minor. The key is the
one the last chorus ends in, so a song that modulates ends in its new
key. A song in a set or a row has no tonic; it ends where it started,
on its first harmony.

The last bar is played differently from all the others: the piano
strikes the chord once and holds it, the bass holds the root, and the
drums play one kick with the crash cymbal, the library's "Crash
ending". The chorus's last bar already gets a drum fill (it leads into
the next time round or into the ending), so the ending's first bar is
plain groove, and the song does not end with two fills in a row.

The ending is not written on the sheet: its chords are the band's,
worked out each time from the key. Type a chord in an ending bar and
it becomes a bar of the tune: the chorus grows to take it in, and the
ending moves on after it.

### `Form`: three numbers that travel with the band

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Form {
    /// How many steps of intro come before the chorus: 0 for none.
    pub intro: usize,
    /// How many times the chorus plays: 1 to [`MAX_REPEATS`].
    pub choruses: usize,
    /// Whether the band plays an ending after the last chorus.
    pub ending: bool,
}
```

`Form` lives inside the `Band`, because only a band plays one. It is
`Copy`: three plain numbers cost nothing to copy, so functions take a
`Form`, not a `&Form`. `#[serde(default)]` on the struct means any
field missing from a file takes its value from `Form::default()`.

Where are the intro and the chorus? Only the intro's length is stored,
in steps. The chorus is worked out when needed, from the end of the
intro to the end of the last bar written:

```rust
pub fn chorus(&self) -> Option<(usize, usize)> {
    let band = self.band.as_ref()?;
    let (start, end) = (band.form.intro, band.end);
    (start < end).then_some((start, end))
}
```

so typing a chord after the last bar makes the chorus longer with
nothing to keep in step. The ending is worked out the same way
(`Project::ending`): the two bars after the chorus, and the harmony
`ending_harmony` picks.

### The intro moves the song

Putting bars in front of a song means moving everything else later:
clips, sections, markers of time, tempo, key and chords, the loop
region. `Project::make_room` does this, the same function that makes
room when you copy a section. Taking the intro away needs the
opposite, `Project::cut_start`, which also has to keep what the song
sounds like where the cut ends: the song then *starts* in the time
signature, tempo, key and harmony that were sounding there.

```rust
let made = self.moving_the_band(|project| {
    project.cut_start(old);
    steps == 0 || project.make_room(0, steps)
});
```

`moving_the_band` runs the change and then tells the band that its
parts moved with everything else (see [Moving the whole song is not an
edit](band.md#moving-the-whole-song-is-not-an-edit)).
Without it, the band would compare its clips with the fingerprints it
kept, find them in new places, and think you had edited them: every
part would be frozen.

### Playing the chorus again: the arrangement

The playlist is the song *as drawn*; the arrangement (see [Sections,
repeats and loops](../program/project.md#sections-repeats-and-loops))
is the song *as played*, with every repeat unrolled. It plays a
section marked ×2 twice. The chorus is one more layer around that:

```rust
let mut played: Vec<_> = once(from, start).collect();
for chorus in 1..=choruses {
    played.extend(
        pieces(project, start, end)
            .into_iter()
            .map(|piece| (piece, Some((chorus, choruses)))),
    );
}
played.extend(once(end, to));
```

The intro's pieces once, the chorus's pieces `choruses` times, then the
ending once. Each pass remembers which time round it is in
`Pass::chorus`, which the form row and the playlist read for
"▶ 2 of 3".

Because everything that plays reads the arrangement (the audio thread,
the WAV and MIDI exports, the metronome's clicks, the tempo of every
step), none of them needs to know about choruses. A loop region
inside the chorus does not take in all of it, so the chorus is not
repeated there: looping bars 3 to 4 plays bars 3 to 4.

### The last bar: one closure, two ways of playing

The piano plays every bar with the style's comping, and the last bar
of an ending with `Comping::Held` instead; the bass the same with
`Bass::Held`. So the band plays its bars in **runs** that leave the
held bar out (more on runs under [Playing in
runs](#playing-in-runs)), and then plays the held bar on its own. The
way of playing is handed to the function as a **closure**:

```rust
fn held_bar(
    held: Option<Bar>,
    harmonies: &[Played],
    play: impl Fn(&[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent> {
    held.map_or_else(Vec::new, |bar| {
        play(&[bar], &cut(harmonies, bar.start, bar.end()))
    })
}
```

`impl Fn(&[Bar], &[Played]) -> Vec<NoteEvent>` means "anything that
can be called with bars and harmonies and gives back notes". The
piano passes `|bars, played| Comping::Held.play_in_bars(bars, played)`,
the bass `Bass::Held`'s. `cut` cuts the harmonies to the bar, and also
stops a harmony cut short from "leading" anywhere: a walking bass would
otherwise walk towards the next chord before it arrives.

The ending's harmony is not on the sheet, so the band adds it to the
list of markers it plays from, as if it were written there:

```rust
if let Some(ending) = ending {
    markers.push(ChordMarker::new(ending.start, ending.harmony));
}
```

It is then voiced from the chord before it, like any other, so the
piano moves to the last chord smoothly.

## Variations a and b: the band digs in

A band does not play every part of a song the same way. The verse is
lighter, so the words come through; the chorus is fuller, and the
drummer, the bassist and the pianist all lift at once. A jazz group
plays the first chorus of a ballad gently and then starts to swing
harder. So every style has two variations: **a** is the style as it
is, and **b** is the same style played fuller.

![The chord sheet of an eight-bar pop ballad. Bar 5 has a boxed "b". Its menu is open, with Variation a and Variation b, b chosen, and the hint: "From bar 5 on, the band plays the style fuller: the drums on the ride cymbal, a busier bass or piano where the style has one, a little louder. The drums fill into it"](../images/step-9-variations-sheet.png)

### Trying it

Right-click a bar on the chord sheet (**F5**) and pick **Variation a**
or **Variation b**. The band plays that variation from that bar up to
the next change, or to the end of the song. To change only some bars,
select them first: click the first, Shift+click the last, then
right-click one of them. The bars after the selection keep what they
played before. That is how you play only the bridge of a song fuller,
or only the last chorus. Right-clicking a section's name on the sheet
selects its bars and opens the same menu.

A boxed letter marks each change, on the sheet after the bar number
and on the playlist's bar ruler, like the rehearsal letters printed
over the staff in band parts. Picking the variation that is already
playing puts no marker; changing a bar back takes the marker away.

The drums announce each change: the bar before it is a **fill**, and
the first beat of the new variation gets a **crash** cymbal. This is
the same signal real drummers give, and why a listener knows that
"something new starts here" even without hearing the words.

![The playlist of the same ballad: bars 1 to 4 and 5 to 8 are separate patterns for each player, and the ruler shows a boxed "b" at bar 5. Bass 2 has a note on every beat, and Drums 2 is busier](../images/step-9-variations-playlist.png)

### What "fuller" means

Variation b does not invent a new style. It opens the same one up, the
way musicians do it:

| Who | Variation a | Variation b |
|---|---|---|
| Drums | time on the closed hi-hat | time on the **ride** cymbal, unless the style keeps time on the ride already |
| Drums, in a pop or rock ballad | **cross-stick** on the snare's rim | the full snare |
| Bass, a jazz style played in two, in 4/4 | **in two**: root and fifth, one every two beats | **walking**: a note on every beat |
| Bass, blues shuffle | quarter notes | the **boogie** line, up and down the chord |
| Bass, pop, rock, soul or folk in 4/4 | the root held | the root on every beat |
| Piano, pop or rock ballad | the chord, then its notes one by one | steady eighth notes |
| Guitar | chords left to ring, or picked; held power chords | strummed; power chords chugged in eighths |
| Strings or organ, in some styles | silent | they join in |
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
- A jazz bassist **in two** plays on beats 1 and 3, usually the root
  and the fifth, which leaves room around a quiet melody. **Walking**
  fills every beat with a line that leads to the next chord: over Dm7
  going to G7, the Jazz ballad's bass plays D, F, A and F♯, the F♯ a
  semitone under the G it is heading for. Going from "two" to "four"
  is how a jazz rhythm section turns up the heat without playing
  louder.

Where a style has no well-known busier way to play (a tango, a bossa
nova, a chorale), variation b keeps its patterns and only changes the
drums and the loudness. The rules are deliberately few, so that b
always sounds like the same band, not like a different song. The
guitar's and the pads' parts in b are described with the bigger band
(see [Enums that carry data, and a function as a
parameter](band.md#enums-that-carry-data-and-a-function-as-a-parameter)).

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
pub(crate) fn tidy(markers: &mut Vec<VariationMarker>) {
    markers.sort_by_key(|marker| marker.start);
    markers.dedup_by(|later, earlier| {
        later.start == earlier.start && {
            *earlier = *later;
            true
        }
    });
    let mut playing = Variation::A;
    markers.retain(|marker| {
        let changes = marker.variation != playing;
        playing = marker.variation;
        changes
    });
}
```

First in order of their start; then, of two markers on the same step,
the later one wins (`dedup_by` hands the closure the later element and
the one kept before it, and the closure copies the later one over);
then only the changes stay. The closure passed to `retain` changes
`playing`, a variable outside it. A closure that changes what it
captures is an `FnMut`: it can be called many times, but not from two
places at once, because each call needs the variable to itself.
`retain` asks for exactly that.

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

The bars are grouped into **runs**, stretches in one variation, and
each run is played with that variation's way of playing:

```rust
fn played_in_runs(
    runs: &[Run],
    harmonies: &[Played],
    play: impl Fn(Variation, &[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent> {
```

The piano's part is the two calls together, the runs and then the
held bar of an ending:

```rust
Player::Piano => {
    let mut notes =
        played_in_runs(&score.runs, &score.harmonies, |variation, bars, played| {
            band.comping_in(variation).play_in_bars(bars, played)
        });
    notes.extend(held_bar(score.held, &score.harmonies, |bars, played| {
        Comping::Held.play_in_bars(bars, played)
    }));
```

Each run gets only its own harmonies, cut at its edges with `cut`. A
song with no markers is one run.

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
arm keeps everything else as it is.

### Drums on other drums

The drums keep the rhythm's patterns and change which drum plays them:
`drum_in(Drum::HiHat, Variation::B)` is `Drum::Ride` when the rhythm
has no ride of its own. The band writes each pattern into the channel
of the drum that should sound, so the hi-hat's eighth notes land on the
ride's channel. A channel the song does not have yet (a ride, a crash)
is added the first time a variation needs it, and never otherwise:
a song without markers keeps exactly the channels it has.

Each change of variation also starts a new **chunk**, the stretch one
pattern covers, so the playlist shows the a and b parts as separate
clips you can see, mute or edit.

### Markers move with the song

Everything that moves the song moves the markers too: an intro put in
front (`make_room`), an intro taken away (`cut_start`), a section
copied (`copy_section`, which copies the variations inside it), a new
time signature (`remeter`), and **End the song here**, which drops the
markers after the end.

## A style for each section

Variations make a part of a song fuller, but in the same style. Many
songs change their feel partway through: a Latin jazz tune plays its
verse as a bossa nova and its bridge in swing; a pop song goes to half
time for the bridge; an arrangement starts as a ballad and ends as a
rock song. So each part of a song can have a style of its own.

![The chord sheet of an eight-bar song in Pop beat. Bar 5 has the word "Swing" in orange after its number: from there the band plays in swing.](../images/step-9-section-styles-sheet.png)

### Trying it

1. Open the chord sheet with **F5**, with a band playing (any demo,
   or a Library style).
2. Right-click a section's name on the sheet. The section's bars are
   selected, and the menu has **Variation a**, **Variation b** and
   **Style: …**, which names the style playing there now.
3. Open **Style**, then a genre, and pick a style, say **Latin** ›
   **Bossa nova**. The bars of that section are now played as a bossa
   nova. The sheet writes "Bossa nova" after the number of the first
   bar, and the notice says which bars changed.
4. The same **Style** menu is in every bar's right-click menu. Without
   a selection it changes the style from that bar on; with bars
   selected (click the first, Shift+click the last), it changes only
   those bars, and the bars after them keep what they played.
5. **The song's style**, at the top of the menu, goes back to the
   band's own style.

![The menu of bar 3: Variation a, Variation b, and "Style: Pop beat" opened, with "The song's style: Pop beat" at the top, then one entry per genre (Pop, Rock, Blues, Jazz, Soul, funk and R&B, ... Classical and film). Classical and film is open: Epic toms, March, Chorale, Alberti bass, Arpeggios.](../images/step-9-section-styles-menu.png)

The menu only lists styles in the time signature of those bars, and
the [Orchestra](orchestra.md), which plays in any. A waltz in
the middle of a song in 4/4 would need bars of three beats, which
means changing the song's bars, not only how the band plays them.
Changing the time signature is a separate step, so it stays separate
here.

### What changes and what stays

A section's style changes **how** the band plays:

- the piano's **comping**, the rhythm it strikes the chords in,
- the **bass line**,
- the **drums**,
- the patterns of the guitar, the strings and the organ, where the
  band has them,
- the **swing**: a swing section in a straight pop song swings its
  eighth notes, and a rock section in a jazz song plays them even.

It does not change **who** plays or the sounds they play with. The
band keeps its instruments, so the same piano and the same bass go
from the bossa nova verse to the swing bridge, as a real band would.
To change the instruments, use the band's **Instrument lineup**.

The drums tell the listener a change is coming, as they do for a
variation: the bar before the change plays the fill of the style that
is ending, and the first beat of the new style gets a crash cymbal.
Each player's comping pattern starts again from its beginning on the
first bar of the new style, so a two-bar bossa nova pattern does not
start halfway through.

### Markers, the same way as variations

```rust
pub struct StyleMarker {
    pub start: usize,
    pub style: Option<&'static BandStyle>,
}
```

A marker with `style: None` goes back to the band's own style. The
markers live in the `Band`, next to the variation markers, and follow
the same rules: a `tidy` of their own keeps only markers that change
something, and everything that moves the song moves them too.

`&'static BandStyle` is a reference that lives for the whole program:
the band styles are built once (`band_styles()` keeps them in a
`OnceLock`) and never freed. That is why a marker can hold a
reference instead of a copy or a name. A file can't hold a reference,
so `StyleMarkerFile` saves the style's name and looks it up again when
it reads the song; a name this version does not know is an error that
says so.

### Every style plays the whole song, then each keeps its own bars

The simplest way to write a part with several styles is not to teach
every piece of the band about sections. Instead, `write_band` asks
each style that plays somewhere for the whole song, and keeps each
note only where that style plays:

```rust
for style in sections::styles_played(&band.styles) {
    let playing = band.playing(style);
    let mut part = self.band_notes(&playing, player, &score);
    if let Some(following) = &following
        && playing.style.follows_the_song()
    {
        part = self.played_along(following, &playing, player, part, &score);
    }
    notes.extend(sections::where_it_plays(part, &band.styles, style));
}
```

`band.playing(style)` is a copy of the band with the other style's
patterns. `band_notes` writes one player's part for one style (the
`if let` in the middle is the [orchestra](orchestra.md)
playing along with a score). `where_it_plays` keeps the notes that
start where `style` plays, and cuts a note that would ring into the
next style:

```rust
notes
    .into_iter()
    .filter(move |(_, note)| style_in(markers, note.start) == style)
    .map(move |(channel, mut note)| {
        if let Some(change) = next_change(markers, note.start)
            && note.start + note.length > change
        {
            note.length = change - note.start;
            note.trim = 0;
        }
        (channel, note)
    })
```

`move` makes each closure own what it uses (`markers` and `style`, both
cheap to copy), so the iterator can be returned from the function and
used after it ends. A song without section styles has one style,
`None`, and keeps every note.

The fills come from the same idea. The drums play a fill in the bar
before a change of variation, and a change of style is a change too.
When the band plays the outgoing style through the whole song, that
style has a fill before the change, and the bar before the change
belongs to it, so its fill is kept. The incoming style has a crash on
the change, and the change is its first bar, so its crash is kept.

### Swing per section

Swing is played, not written: notes stay on the grid, and playback
moves the second eighth note of each beat later (see
[Swing](../sound/tempo.md#swing)). A band pattern in a section with
its own style swings as that style does:

```rust
pub fn pattern_swing(&self, pattern: usize) -> u8 {
    let swing = self.section_swing(pattern).unwrap_or(self.swing);
    if swing > 0 && swings_in(self.pattern_time(pattern)) {
        swing
    } else {
        0
    }
}
```

`section_swing` looks at where the pattern's first clip is and returns
that section's swing, or `None` (the song's swing) for your own
patterns and for sections in the band's own style. The band starts a
new pattern where a section's style starts, and two stretches with the
same notes but different swing never share a pattern, so each pattern
swings only one way.

The [playback trace](../sound/trace.md) shows it: in a straight Pop
beat song whose bars 5 to 8 are Swing, the ride cymbal's and the
piano's offbeats in bar 5 start 2.667 sixteenths into the beat, the
last third of a triplet, while in bars 1 to 4 they are on the grid.

## Locking a track: styles play around it

Say you played a piano part yourself, or picked a nylon guitar for the
chords, and now want to hear the song as a bossa nova. Picking the
style would rewrite every track it plays: your piano part would be
replaced by the style's comping, and your guitar would go back to the
style's instrument. So you can **lock** a track first. A locked track
keeps its notes and its instrument, and the style plays the rest of
the song around it.

![The channel rack: a small padlock between each channel's knob and its name. Kick's padlock is lit and closed; Clap, Hat, Snare and Lineup Osc have faint, open ones. The hint reads "Lock: styles keep its notes and sound".](../images/step-9-lock-rack.png)

### Trying it

1. Open a demo, for example **Ode to Joy**, and show the rack with
   **F6**.
2. Click the small padlock between **Keys**'s knob and its name. It
   closes and turns orange. Right-clicking the name has **Lock** and
   **Unlock** too, and **Ctrl+Z** undoes it.
3. In the **Library**, apply another style, say **Bossa nova**. The
   band comes in on its own tracks, but leaves the piano to your Keys:
   the notice ends with **🔒 Kept: Keys.**, the Library's list of where
   each part came from has a **Locked** row, and the chord sheet's band
   says "🔒 Keys is locked: the band leaves the piano to it."
4. Try styles one after another in pattern mode. Each trial copies the
   locked rows into the trial pattern, so the new bass follows the
   locked chords.
5. Unlock Keys again. The band takes the piano part back, and the
   notice says your Keys is muted so the two do not play over each
   other.

What moves the *whole* song still moves a locked track with it: a new
time signature puts its notes in the new bars, and a key change moves
them to the new key. A lock protects a part from styles, not from the
song changing shape. You can always edit a locked track by hand.

### One bit per channel

The rack has at most 32 channels, so the set of locked ones fits in
one `u32`, one bit for each channel:

```rust
pub struct Locked(u32);

// One bit for each channel there can be.
const _: () = assert!(MAX_CHANNELS <= u32::BITS as usize);

impl Locked {
    pub fn contains(self, channel: usize) -> bool {
        channel < MAX_CHANNELS && self.0 & (1 << channel) != 0
    }
}
```

`Locked(u32)` is a **tuple struct**: a new type wrapped around a `u32`,
reached with `.0`. Wrapping it means nobody can add two sets by
mistake or pass a channel count where a set is meant. The `const _`
line is a check the compiler runs: if someone raised `MAX_CHANNELS`
above 32, the build would fail right there instead of a lock silently
landing on the wrong channel.

Removing a channel renumbers the ones after it, so the set must shift
too. `without` keeps the bits below the removed channel and moves the
bits above it down by one:

```rust
let below = self.0 & ((1 << index) - 1);
let above = self.0.checked_shr(index as u32 + 1).unwrap_or(0) << index;
Locked(below | above)
```

`checked_shr` returns `None` instead of overflowing when the shift is
32 or more, which happens when the last channel is removed.

### One door for every style

Styles write into patterns from many places: a whole style, a rhythm,
a saved style, a row or micromode style, a fill, the final chord. If
each of them had to remember to skip locked channels, one would
forget. So the library never gets a `&mut Pattern` at all. It gets
a `StylePattern`, which only writes rows that are not locked:

```rust
pub(crate) struct StylePattern<'a> {
    pattern: &'a mut Pattern,
    locked: Locked,
}

impl StylePattern<'_> {
    pub(crate) fn set_notes(
        &mut self,
        channel: usize,
        notes: impl IntoIterator<Item = NoteEvent>,
    ) -> bool {
        !self.locked.contains(channel) && self.pattern.set_notes(channel, notes)
    }
}

impl Deref for StylePattern<'_> {
    type Target = Pattern;

    fn deref(&self) -> &Pattern {
        self.pattern
    }
}
```

`Deref` lets a `StylePattern` be read exactly like a `Pattern`
(`pattern.length()`, `pattern.notes(channel)`), while writing only
goes through its own methods. This is a common Rust shape: a
**wrapper** that hands out reading for free and guards writing. The
instrument has the same kind of door, `Project::set_sound`, which
refuses a locked channel. A test applies every style, rhythm, saved
style, row and micromode style with each channel locked in turn, and
checks that the locked one never changes.

### The band plays around a lock

The band writes its own patterns, a part per player, so it has its own
rules (in `band/locks.rs`):

- a player whose channel is locked is skipped: its part stays as it is;
- locked drum channels keep their notes while the other drums change;
- if your own Keys or Bass is locked and plays, the band's piano or
  bass stays silent, so the song's part is not doubled;
- when the band leaves (**No band**), a locked band part stays in the
  song as your own.

The band keeps a fingerprint of what it plays from, to know when to
write again (see [A part you edit stays
yours](band.md#a-part-you-edit-stays-yours)). The lock
counts in that fingerprint only when something is locked, so a song
with nothing locked keeps the fingerprint it had.

## In the file

All four are saved with the band, and each is left out of the file
when it is not used, so a song that does not use them is written as
it would be without them:

```rust
#[serde(default, skip_serializing_if = "Form::is_plain")]
pub form: Form,
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub variations: Vec<VariationMarker>,
```

`Form::is_plain` is `true` for the form a song has without the form
row: no intro, one chorus, no ending. A song that uses the form needs
at least format 14, variations format 15, a locked channel format 23,
and section styles format 24 (see [The project file
format](../reference/project-format.md)).

## Testing it

Each module tests its own part:

- `form.rs`: `choruses_play_the_sheet_again_and_again` checks the
  playlist shows the chorus once and the arrangement plays it three
  times (192 steps for a 64-step chorus), with "2 of 3" at step 70;
  `an_intro_leads_into_the_chorus` and `the_intro_moves_the_melody_along`
  put intros in and take them out, and check the melody moves and the
  band's parts are not frozen; `an_intro_is_a_turnaround_into_the_first_chord`
  checks turnarounds into major and minor chords, the spellings into
  Ebm and Gb, and a blues; `the_ending_holds_the_tonic` checks the last bar is
  one chord held 16 steps, with one crash; `the_form_is_saved` checks
  the file reads back the same.
- `variation.rs`: `markers_keep_only_changes`,
  `the_drummer_moves_to_the_ride`, `the_drums_fill_into_a_change`,
  `variation_b_is_louder`, `players_switch_to_their_busier_way`,
  `a_jazz_ballad_walks_in_variation_b`, `markers_move_with_the_song`
  and `a_copied_section_plays_the_same_variations`.
- `sections.rs`: `a_section_plays_as_its_style_does`,
  `a_section_plays_its_drums_and_starts_its_pattern_on_its_first_bar`,
  `a_section_without_drums_rests_them`,
  `a_swing_section_swings_in_a_straight_song` and
  `markers_move_with_the_song_and_are_saved`.
- `lock.rs`: `no_library_choice_writes_a_locked_channel` is the test
  that tries every style with each channel locked;
  `removing_a_channel_renumbers_the_ones_after_it` checks `without`.
  `band/locks.rs` checks each rule of the band, and that
  `no_band_leaves_the_locked_parts_to_the_song`.

In the app, `ux_tests.rs` clicks the rack's lock and undoes it
(`the_rack_locks_a_channel_and_undo_unlocks_it`), tries Doo-wop and
then Bossa nova in a pattern around a locked Keys, and checks the
notices and the chord sheet say what is kept
(`the_library_and_the_band_say_what_is_kept`).
