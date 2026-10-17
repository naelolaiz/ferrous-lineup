# Song form: an intro, choruses and an ending

Files: [`crates/lineup-core/src/band/form.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/lineup-core/src/band/form.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/arrangement.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/lineup-core/src/arrangement.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/chord_sheet/form.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/lineup-app/src/chord_sheet/form.rs),
[`crates/lineup-app/src/app/form.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/lineup-app/src/app/form.rs)
and [`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/lineup-app/src/playlist/sections.rs)

A band does not play a tune once and stop dead. It plays a few bars to
set the key and the groove (the **intro**), then the tune, then the
tune again, maybe several times, and then it **ends**: it lands on the
home chord and holds it. Until now the band in Ferrous Lineup played
the bars on the sheet once and stopped at the last one, on whatever
chord was there. This change gives it the shape of a real
performance.

![The chord sheet with its form row: "Intro: 4 bars", Choruses 3, Ending lit. The sheet reads Intro (C, Am7, Dm7, G7), then "Chorus ×3" between repeat signs (C, Am, F, G7), then Ending (C, then C again with "held" and the final bar line)](../images/step-9-song-form-sheet.png)

## Trying it

Write a few bars of chords on the sheet (**F5**), or pick a
progression. Under the Band and Progression buttons there is a new
row, **Form**:

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

## "Chorus" means one time through

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
repeat as they always did, inside each time round.

## The intro: a turnaround

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
on C7 gets C7, Am7, Dm7, G7.

A pitch-class set, a micromode or a row form has no dominant to lead
into it, so the intro plays it from the first bar: a **vamp**, the
band setting up the sound before the tune comes in. So does a
diminished chord, which no turnaround resolves to.

## The ending: home, and held

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
drums play one kick with the crash cymbal. That is the "Crash ending"
from the library's fills, which the band had never used. The chorus's
last bar already gets a drum fill (it leads into the next time round
or into the ending), so the ending's first bar is plain groove, and
the song does not end with two fills in a row.

The ending is not written on the sheet: its chords are the band's,
worked out each time from the key. Type a chord in an ending bar and
it becomes a bar of the tune: the chorus grows to take it in, and the
ending moves on after it.

## How it works

### `Form`: three numbers that travel with the band

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Form {
    pub intro: usize,
    pub choruses: usize,
    pub ending: bool,
}
```

`Form` lives inside the `Band`, because only a band plays one. It is
`Copy`: three plain numbers cost nothing to copy, so functions take a
`Form`, not a `&Form`. `#[serde(default)]` on the struct means any
field missing from a file takes its value from `Form::default()`, and
the band leaves the whole form out of the file when it is the plain
one:

```rust
#[serde(default, skip_serializing_if = "Form::is_plain")]
pub form: Form,
```

so a song without an intro, extra choruses or an ending is written
exactly as before, as format 13, and older versions of Ferrous Lineup
still open it. A song that uses the form is format 14.

Where are the intro and the chorus? Only the intro's length is stored,
in steps. The chorus is worked out when needed, from the end of the
intro to the end of the last bar written (`Project::chorus`), so typing
a chord after the last bar makes the chorus longer with nothing to keep
in step. The ending is worked out the same way (`Project::ending`): the
two bars after the chorus, and `ending_harmony`.

### The intro moves the song

Putting bars in front of a song means moving everything else later:
clips, sections, markers of time, tempo, key and chords, the loop
region. `Project::make_room` already did this for copying a section.
Taking the intro away needs the opposite, `Project::cut_start`, which
also has to keep what the song sounds like where the cut ends: the
song then *starts* in the time signature, tempo, key and harmony that
were sounding there.

```rust
let made = self.moving_the_band(|project| {
    project.cut_start(old);
    steps == 0 || project.make_room(0, steps)
});
```

`moving_the_band` runs the change and then tells the band that its
parts moved with everything else. Without it, the band would compare
its clips with the fingerprints it kept, find them in new places, and
think you had edited them: every part would be frozen.

### Playing the chorus again: the arrangement

The playlist is the song *as drawn*; the arrangement
([Song structure](../step-8/song-structure.md)) is the song *as
played*, with every repeat unrolled. It already played a section
marked ×2 twice. The chorus is one more layer around that:

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
ending once. Each pass remembers which time round it is in its new
field, `Pass::chorus`, which the form row and the playlist read for
"▶ 2 of 3".

Because everything that plays reads the arrangement (the audio thread,
the WAV and MIDI exports, the metronome's clicks, the tempo of every
step), none of them had to change to play three choruses. A loop
region inside the chorus does not take in all of it, so the chorus is
not repeated there: looping bars 3 to 4 plays bars 3 to 4.

### The last bar: one closure, two ways of playing

The piano normally plays every bar with the style's comping:
`style.comping.play_in_bars(&bars, &harmonies)`. With an ending, the
last bar must be played with `Comping::Held` instead. The bass is the
same story with `Bass::Held`. Rather than write the split twice, the
band hands the way of playing to a function as a **closure**:

```rust
fn played_with(
    bars: &[Bar],
    harmonies: &[Played],
    held: Option<Bar>,
    play: impl Fn(&[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent>
```

`impl Fn(&[Bar], &[Played]) -> Vec<NoteEvent>` means "anything that
can be called with bars and harmonies and gives back notes": the
piano's call passes `|bars, played| style.comping.play_in_bars(bars,
played)`, the bass's passes the bass line's. `played_with` plays every
bar but the held one; `held_bar` plays the held one. Both cut the
harmonies to their stretch with `cut`, which also stops a harmony cut
short from "leading" anywhere: a walking bass would otherwise walk
towards the next chord before it arrives.

The ending's harmony is not on the sheet, so the band adds it to the
list of markers it plays from, as if it were written there:

```rust
if let Some(ending) = ending {
    markers.push(ChordMarker::new(ending.start, ending.harmony));
}
```

It is then voiced from the chord before it, like any other, so the
piano moves to the last chord smoothly.

## Exercises

1. Write | Dm7 | G7 | Cmaj7 | Cmaj7 |, a ii–V–I in C, and give it a
   2-bar intro. Which chords does the intro write? They do not lead
   into C: what do they lead into, and why?
2. Set the key to A minor (the Key box at the bottom left), write a
   song that starts on Am, and turn the ending on. Which chord does it
   end on? Now change the first chord to F: which chord does it end on
   now, and why is that still right?
3. Export a song with a 4-bar intro, three choruses and an ending as
   a WAV file (File > Export WAV…). How many bars long is it? Check
   your answer against the file's length and the tempo.
4. Give a song an 8-bar intro, then change it to 2 bars. What happens
   to a chord you had changed in the intro? Undo it.
5. Type a chord in the first bar of the ending. Where is the ending
   now? Where is the end-repeat sign?
6. (Rust) `Form` is `Copy`; `Band` is not. Why could `Band` not be
   `Copy`? Look at its fields.
7. (Rust) In `played_pieces`, the chorus repeats only `if from <= start
   && end <= to`. Write down what a loop region from the middle of the
   intro to the middle of the chorus would play if that test were left
   out.
