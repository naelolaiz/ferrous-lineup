# The Big Bang Theory theme: a TV theme from a transcription

Files: [`crates/lineup-core/src/library/song/pop.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/pop.rs)
and [`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)

The **File > Demos** menu gets the opening theme of the television series
*The Big Bang Theory*, written by Ed Robertson and played by Barenaked
Ladies in 2007. It is the television version, all of it: twelve bars,
about twenty seconds. The song is still under copyright; it was added at
the owner's request, and the comment above it in the code says so.

## Trying it

Open **File > Demos > Songs > Pop > The Big Bang Theory theme** (or type
"bang" in the search box at the top of the Demos menu) and press Play.

- The **tempo strip** shows two tempos: 130 beats a minute for the first
  four bars, then 159 from bar 5. The band does not speed up bit by bit:
  it jumps.
- The **chord sheet** shows four chords going round, two beats each:
  Cadd9, D, G, Em7.
- In the **last bar** the band stops dead on beat 1, strikes G once on
  beat 2, short, with a crash cymbal, and is silent again. The song
  ends there: no bar of the home chord held comes after it.

To read the tempos instead of listening for them, trace the demo:

```sh
ferrous-lineup --trace "Big Bang" --out bbt.tsv
```

The head of the file lists them:

```text
# tempo: 0.0000 s, step 0, bar 1: 130.00 BPM
# tempo: 7.3846 s, step 64, bar 5: 159.00 BPM
```

Four bars of 4/4 are 16 beats, and 16 beats at 130 a minute last
16 × 60 / 130 = 7.3846 seconds: the verse starts exactly there.

## The music

### Four chords, two beats each

In G major the four chords are **IV, V, I and vi**: C, D, G and E minor.
They are the same four chords as the "four chords" style in the Library
(I V vi IV), in another order. Here the order tells a small story every
two bars: C and D, the two chords that lead home, then G, home, then
E minor, the **relative minor**, which shares two notes with G (G and B)
and turns back to C to start again.

Two of the chords carry an extra note, as a guitarist plays them:

| Chord | Numeral | Notes | The extra note |
|---|---|---|---|
| Cadd9 | IVadd9 | C E G **D** | D, the ninth above C |
| D | V | D F# A | |
| G | I | G B D | |
| Em7 | vi7 | E G B **D** | D, the seventh above E |

So D is in all four chords: added to C and to E minor, and a note of
its own chord and of G. On a guitar in G, these four shapes keep D and G
on the top two strings, at the same frets, so the top of the strum
hardly moves while the bass climbs C, D, G and drops to E.

The transcription also gives the D chord an eleventh, G, on the
TheoryTab page. The app's numerals have no "add11" (only "sus4", which
drops the third, a different chord), so the demo plays a plain D.

### Patter

The tune puts a syllable on nearly every eighth note, mostly on G, A
and B, the bottom of the scale, like someone talking fast. Twice it
breaks the even eighths with a short note followed by a long one: G4 for
a sixteenth and A4 for a dotted eighth in bar 5, D5 and B4 the same way
in bar 8. Then the last lines jump up to the top: E5 and D5, then G5
with F sharp, the **leading note**, a semitone under it, again and again.

### Two tempos

A song has one tempo to start with, `tempo: 130.0`, and a part can
change it with tempo marks (see [`TempoMark`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)).
The verse starts at 159, at its first step:

```rust
Part::new(
    "Verse",
    "E4:2 G4 G4 A4 B4 A4 G4:1 A4:3 | B4:2 B4 A4 A4 G4 G4 G4 D4 | ...",
    "IVadd9 V | I vi7 | IVadd9 V | I vi7 | ...",
)
.with_tempo(&[tempo(0, 159)]),
```

`tempo(at, bpm)` changes the tempo at once, as a new section of a score
does; `glide` would change it bit by bit, as an accelerando does.

### The stop and the shot

The last bar's chart is `I... I. I... I...`: four chords sharing the bar,
a beat each. Three dots are a **rest**, the band silent; one dot is a
**shot**, the whole band striking the chord once, short, with a kick and
a crash (see [Shots, holds, rests and pushes](stops.md)). So: silence on
beat 1, G struck on beat 2, silence to the end. The first beat is written
`I...` and not `V...` because the tune sings G there: the harmony is
already home, even with nobody playing it.

### Ending on the stop

Most song demos end with one bar the score does not have: the home
chord held, with a crash, so a chart that just runs out still sounds
finished. The genres that end slowly (tangos, folk songs, soul,
classical pieces) also broaden their last bar, a small ritardando. A song whose
chart already ends on a break (a shot, a hold or a rest) gets neither:
the score says how the band stops, and that stop is in time. A tango's
last "chan-chan" is the same case as this shot.

`song_demo`, in
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
looks at the last chord of the last part's chart and skips the extra
bar when that chord has a stop:

```rust
let stopped = song.form.last().is_some_and(|name| {
    patterns
        .iter()
        .rev()
        .find(|piece| piece.part == *name)
        .and_then(|piece| piece.spans.last())
        .is_some_and(|span| span.stop.is_some())
});
```

`is_some_and` asks two things at once: is there a value, and does it
pass the test? `song.form.last()` is `None` for a song with no form, and
then the answer is simply `false`. The chain inside reads from the end
of the list of patterns (`.rev()`), takes the last one made from that
part, its last chord span, and checks whether the span carries a stop.
Eleven demos end this way; the trace of each shows the last note where
the score has it.

## Where the notes come from

No score of the theme could be read from where the demo was written,
but a **transcription** could. On Hooktheory's *TheoryTab* site, people
write songs down the way a music theory class reads them: every note as
a **scale degree** in the key (1 is the tonic, 5 the fifth...), every
chord as a Roman numeral. A research project, Sheet Sage (Chris Donahue
and others, 2022), packed about 26,000 of these transcriptions into one
file on GitHub, and Hooktheory's own site gives the same sections one at
a time.

The theme has two sections there, "Intro" and "Verse". A note is
written like this:

```json
{"sd": "6", "octave": 0, "beat": 1, "duration": 1}
```

Scale degree 6, in the octave that starts on the tonic, on beat 1, a
beat long. In G major degree 6 is E, and with the tonic as G4, the
octave from G4 up holds E5: a quarter note `E5:4`. Degree 6 in octave
-1 is E4, and degree 1 in octave 1 is G5. With G4 as the tonic the tune
runs from D4 to G5, the range a lead sheet writes it in; a man sings
it an octave lower.

| Written there | In G | Here |
|---|---|---|
| sd 6, octave 0, 1 beat | E, above the tonic | `E5:4` |
| sd 5, octave 0, ½ beat | D | `D5:2` |
| sd 1, octave 0, ¼ beat | G, the tonic | `G4:1` |
| sd 5, octave -1, ½ beat | D, below the tonic | `D4:2` |

The verse is also **aligned** to the recording: its beat 1 is at 7.6
seconds into it, and beat 33 at 19.66 seconds. 32 beats in 12.06
seconds is 32 × 60 / 12.06 = 159.2 beats a minute. That is where the
159 comes from.

The intro's transcription was revised. The version of 2020 has it at
124 beats a minute, over IV | I | IV | I, a bar each; the page now gives
130 and the verse's chords, IVadd9 V I vi7. The demo takes the newer
reading, two beats a chord as in the verse, for a reason the notes give:
the intro's third bar is the same eight notes as the verse's first and
third bars (E G G A B A G A), and there the transcription has them over
C and D.

## Exercises

1. Trace the demo and find the first note of bar 5 and of bar 6. How
   long is a bar at 159? (4 × 60 / 159 = 1.509 seconds.) And at 130?
   (1.846 seconds.)
2. Em7 without its seventh is Em. Which note is gone, and which of the
   four chords would then have no D?
3. Name the same four chords in A major. (D add9, E, A, F sharp minor
   seven.)
4. In bar 10 the tune alternates G5 and F#5 over G, then over Em7. Over
   G, F sharp is the chord's seventh; what is it over E minor?
   (The ninth.)
