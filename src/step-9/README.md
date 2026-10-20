# Step 9: The band

Steps 1 to 8 built the parts: loops in a step sequencer and a piano
roll, arranged on the tracks of a playlist, with a library of styles
behind them. What the project was meant to be from the start is a
**backing band**. You type the chords of a song bar by bar, choose a
style, press Play, and a band (bass, drums, piano, guitar, strings)
plays the song in that style. Change the
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
4. **a practice bar**: count-in, metronome, tempo as a percentage, tap
   tempo, a toggle to play a part yourself, and looping the bars you
   choose ([Practising with the band](practice.md)),
5. **song form**: an intro, choruses and an ending
   ([Song form: an intro, choruses and an ending](song-form.md)), and a
   second variation of the style for the parts that need more energy
   ([Variations a and b: the band digs in](variations.md)),
6. **swing**, from straight to triplet
   ([Swing: eighth notes played long-short](swing.md)),
7. **real instrument sounds** from a SoundFont, and the band through
   MIDI out to an external synth. The SoundFont came early, before
   items 4 to 6 ([Real instrument sounds: a SoundFont](soundfont.md)),
8. **a bigger band**: a guitar, strings and an organ, each where the
   style has them ([A bigger band: guitar, strings and organ](bigger-band.md)),
9. **rests, shots, holds and pushes** typed on the chord sheet
   ([Shots, holds, rests and pushes](stops.md)),
10. **chord sheets as plain text**, to paste and share
   ([Chord sheets as text: to paste and share](text-chord-sheets.md)),
11. **a lead sheet**: the melody in notation, the chords above it,
   ready to print ([The lead sheet: the tune in notation](lead-sheet.md)),
12. **a soloist** and 13. **ear training**, both experimental and off by
   default ([The soloist: an improvised line over the chords](soloist.md)).
   Ear training came early, in a window of its own
   ([Ear training](ear-training.md)).

Nothing goes away. The playlist with its tracks, the channel rack, the
piano roll, the harmony keys, the twelve-tone rows, the pitch-class sets
and the micromodes all stay as they are; the chord sheet is one more
window beside them.

## After the plan

With those thirteen in place, playing songs with the band showed what
else was needed:

- downloads for every system ([Release builds](releases.md)), and a
  second and a third round of fixes ([A second bug hunt](bug-hunt.md),
  [a third](bug-hunt-3.md));
- many more songs: [swing and big band](big-band-songs.md),
  [Latin American](latin-american-songs.md), [odd meters](odd-meters.md),
  [a TV theme](big-bang-theory-theme.md), and
  [the concert hall](concert-hall.md), with a
  [search](demo-search.md) to find them;
- the band's sound and what it plays, to see and check:
  [the mixer in score order](mixer-score-order.md),
  [export video](video-export.md), [a playback trace](playback-trace.md)
  and [step lights in the channel rack](rack-lights.md);
- more say over the band: [locking a track](locks.md),
  [a style for each section](section-styles.md) and
  [an orchestra for music that changes meter](orchestra.md);
- and this book, [online and as a PDF](book-online.md).

## Trying it

The changes of this step were first tried on a branch of their own,
and are now all on `main`:

```sh
git checkout main
git pull
cargo run --release
```

## A new name

With the band at the front, the app is now called **Ferrous Lineup**:
a lineup is the players of a band, and "ferrous" (made of iron, the
metal that rusts) keeps the nod to Rust. The rename came as a change of
its own, right after chord names; [A new name](new-name.md) shows what
it touched and what kept the old name.
