# Opening MIDI files and recordings

Ferrous Lineup can open music that was made somewhere else: a **MIDI
file** of a piece, a **chord sheet** saved as text, or, as an
experiment, a **recording**. Each becomes a new song, with its chords on
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

## Recordings (experimental)

**File > Import recording…** opens a sound file (MP3, WAV, FLAC or
Ogg) and tries to play along with it. You can also drop the file on
the window, or open it with Ferrous Lineup from your file manager. (A
WAV file of ten seconds or less dropped on the window is taken as a
sound for a channel instead: see [The channel
rack](channel-rack.md#your-own-sounds).) It listens for a few seconds
(*Listening to …* in the status bar), then makes a song with two
tracks:

- **Recording**: a channel that plays the whole recording;
- **Harmony**: a chord on every beat, on the chords it heard.

It also guesses the tempo, the time signature, the key and the key
changes, so the bar lines, the chord sheet and the harmony keys follow
the music. The song's key is the one the recording stays in longest. The
recording plays wherever you start: from the beginning, from a bar you
click, or after a jump. The status bar says what it found, as in *canon-in-d: 63.9 BPM, 4/4,
D major, chords on 43 of 43 beats*.

Hearing a full recording is much harder than reading a MIDI file, and
the result is often wrong: the tempo may come out at half or double
speed, and chords may be heard where a passing melody note sits. Treat
it as a starting point to correct, and trust your ears over it.

The recording is saved again as a WAV file next to the original, named
like *song.ferrous-lineup.wav* (or in the system's temporary folder if
that folder can't be written to), with a little silence in front so its
first downbeat falls on a bar line. The song plays that file and only
says where it is, so keep it with the song.
