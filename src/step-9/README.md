# Step 9: The band

Ferrous Loom began as a pattern studio in the style of FL Studio: build
loops in a step sequencer and a piano roll, then arrange them in a
playlist. The program the project really set out to follow is a
different one: **Band-in-a-Box**, by PG Music. There you type the chords
of a song bar by bar, choose a style, press Play, and a band (bass,
drums, piano, guitar, strings) plays the song in that style. Change the
style, the key or the tempo, and the band plays it again the new way.

Most of a band was already inside the app. The library turns a
chord chart into voiced chords and a bass line in 145 styles, adds drums
from 81 rhythms, fills and endings, and builds whole songs from charts
([The library: knowledge as data](../step-8/library.md)). What was
missing is the front of it: a place where you write the chords, which
the band then plays. Step 9 brings that to the front, one change at a
time:

1. **Chord names**: reading chords as lead sheets write them, "Am7",
   "F#m7b5", "G/B" ([Chord names: reading a lead sheet](chord-names.md)),
   and then a new name for the app ([A new name](new-name.md)),
2. **a chord sheet**: a window of bars where you type the chords, stored
   in the project. A bar can also hold a pitch-class set, a micromode or
   a row form, the other ways Ferrous Lineup names harmony
   ([The chord sheet: writing a song bar by bar](chord-sheet.md)),
3. **the band plays the chord sheet**: styles split into "band styles"
   (how to play) and "progressions" (what to play)
   ([The band plays the chord sheet](band.md)),
4. **a practice bar**: count-in, tempo as a percentage, a toggle to play
   a part yourself,
5. **song form**: choruses, an intro, an ending, and a second variation
   of the style for other sections,
6. **swing**, from straight to triplet,
7. **real instrument sounds** from a SoundFont, and the band through
   MIDI out to an external synth,
8. **a bigger band**: guitar, strings, a second keyboard,
9. **rests, shots, holds and pushes** typed on the chord sheet,
10. **chord sheets as plain text**, to paste and share,
11. **a lead sheet**: the melody in notation, the chords above it,
12. **a soloist** and 13. **ear training**, both experimental and off by
   default.

Nothing goes away. The playlist with its tracks, the channel rack, the
piano roll, the harmony keys, the twelve-tone rows, the pitch-class sets
and the micromodes all stay as they are; the chord sheet is one more
window beside them.

## Trying it

Each change lands on a branch of its own, `band-in-a-box`, so it can be
tried before it reaches `main`:

```sh
git fetch origin
git checkout band-in-a-box
cargo run --release
```

`git checkout main` goes back to the app as it was.

## A new name

With the band at the front, the app is now called **Ferrous Lineup**:
a lineup is the players of a band, and "ferrous" (made of iron, the
metal that rusts) keeps the nod to Rust. The rename came as a change of
its own, right after chord names; [A new name](new-name.md) shows what
it touched and what kept the old name.
