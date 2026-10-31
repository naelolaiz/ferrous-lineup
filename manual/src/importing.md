# Opening MIDI files and chord sheets

Ferrous Lineup can open music that was made somewhere else: a **MIDI
file** of a piece, or a **chord sheet** saved as text. Each becomes a
new song, with its chords on
the chord sheet and its key on the harmony keys, so you can study it as
you would a demo. When the song open before has unsaved changes, you
are asked first.

## MIDI files

A MIDI file holds the notes of a piece, not its sound: which note
starts when, how long it lasts, how hard it is played, and which
instrument should play it. Many pieces can be found as MIDI files, and
most music programs can write one.

**File > Import MIDI file…** opens one (.mid, .midi, .kar or .rmi).
You can also drop it on the window, or open it with Ferrous Lineup
from your file manager. The song that comes out:

- has a **channel for each part** of the file, named after its track or
  its instrument, with the instrument the file asks for, and its
  volume. The drums become one channel per drum, as in the channel
  rack of a new song;
- keeps **every note** as it was played: how hard, and exactly when,
  even between two steps of the grid;
- is cut at bar lines into **patterns**, laid one after another in the
  playlist, the drums on a track of their own; a passage that comes
  back note for note plays the same pattern again;
- takes the file's **tempo changes**, **time signatures** and **key
  signatures**. A file with no key, or one that says C major when its
  notes are plainly in another key (many programs write C major when
  nobody chose), gets the key its notes fit best;
- gets on its **chord sheet** the chords the app hears in it;
- has its **parts** heard in its notes: the band plays by them, and they
  show faint over the playlist until you keep them (see [Parts the band
  hears](song-form.md#parts-the-band-hears)).

The status bar sums it up, as in *canon-in-d: 3 channels, 77 notes, 9
bars, 4/4, 72 BPM, D major*:

![Pachelbel's Canon opened from a MIDI file: the chord sheet in front with the chords heard in it, D A Bm F#m G D G A, with their numerals I V vi iii IV I IV V, the key D major and 72 in the tempo display, the pattern Bars 1–9 in the toolbar, and the status bar saying "canon-in-d: 3 channels, 77 notes, 9 bars, 4/4, 72 BPM, D major"](images/import-midi.png)

From there it is a song like any other: play it, slow it down, look at
each part in the piano roll, change its sounds in the mixer, try a
style from the Library on it, and save it as a .lineup file.

### Getting the chords right

The app hears chords as a musician would hear them over a stretch of
music, not note by note, and it gets most of them right. Where a piece
moves fast, or its harmony is unusual, check what it heard: click a bar
and type the right chord (see [The chord sheet](chord-sheet.md)).

## Chord sheets as text

**File > Import chord sheet…** opens a chord chart saved as text, as in
`| C | Am7 | Dm7 G7 |`, and the band plays it. Such a text is what
**File > Export chord sheet…** writes, and what you find in songbooks
and on websites. The format is in [The chord
sheet](chord-sheet.md#chord-sheets-as-text).

## Recordings

A sound file (MP3, WAV, FLAC or Ogg) is not opened as a song of its
own: **File > Add a recording…** lays it on the song you have open, and
writes the notes sung or played in it, as a take (see
[Takes](takes.md#adding-a-recording-made-before)).
