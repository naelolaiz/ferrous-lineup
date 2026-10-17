# Step 9: The band

Ferrous Loom began as a pattern studio in the style of FL Studio: build
loops in a step sequencer and a piano roll, then arrange them in a
playlist. What the project really set out to be is something else: a
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
   ([Song form: an intro, choruses and an ending](song-form.md)), then a
   second variation of the style for other sections,
6. **swing**, from straight to triplet,
7. **real instrument sounds** from a SoundFont, and the band through
   MIDI out to an external synth. The SoundFont came early, before
   items 4 to 6 ([Real instrument sounds: a SoundFont](soundfont.md)),
8. **a bigger band**: guitar, strings, a second keyboard,
9. **rests, shots, holds and pushes** typed on the chord sheet,
10. **chord sheets as plain text**, to paste and share,
11. **a lead sheet**: the melody in notation, the chords above it,
12. **a soloist** and 13. **ear training**, both experimental and off by
   default. Ear training came early, in a window of its own
   ([Ear training](ear-training.md)).

Nothing goes away. The playlist with its tracks, the channel rack, the
piano roll, the harmony keys, the twelve-tone rows, the pitch-class sets
and the micromodes all stay as they are; the chord sheet is one more
window beside them.

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
