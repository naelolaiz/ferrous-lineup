# Saving, opening and recovering

A song is saved as a **.lineup** file: its patterns and notes, the chord
sheet, the band and its style, the song form, the tempo, the mixer, and
the instrument of every channel. One file holds the whole song.

## The File menu

| Item | Key | What it does |
|---|---|---|
| **New** | **Ctrl+N** | an empty chord sheet with a band to play it |
| **New beat** | | an empty pattern and a drum kit, to build a song from patterns |
| **Open…** | **Ctrl+O** | opens a .lineup file |
| **Open recent** | | the last ten songs opened or saved |
| **Demos** | | the demo songs (see [Finding a demo](finding-a-demo.md)) |
| **Save** | **Ctrl+S** | saves the song in its file |
| **Save as…** | **Ctrl+Shift+S** | saves it in a new file, and goes on working there |
| **Quit** | **Ctrl+Q** | closes Ferrous Lineup |

The **Import** and **Export** items are in [Opening MIDI files and
chord sheets](importing.md) and [Exporting](exporting.md). A recording
goes on the song from the playlist: see
[Takes](takes.md#adding-a-recording-made-before).

The window's title names the song, with a **\*** after it while it has
changes that are not saved. **Ctrl+Z** back to where it was saved takes
the star away again. A new song is **Untitled** until you save it, and
the first **Save** asks where, as **Save as…** does. The file always
gets the **.lineup** ending, even when you leave it out.

A song that no longer opens, because it was moved or deleted, drops out
of **Open recent** when you click it.

## Demos are read-only

A demo says **(demo, read-only)** in the title. Change it as much as
you like, but **Save** asks for a new file: the demo itself stays as it
was, and your version is a song of your own from then on.

## Before your changes are lost

When an action would close a song with unsaved changes (**New**,
**Open…**, a demo, or closing the window), Ferrous Lineup asks **Save
changes?** first:

- **Save** saves the song and goes on;
- **Don't save** throws the changes away and goes on;
- **Cancel** goes back to the song as it is.

The [style editor](style-editor.md) asks the same about a style, with
**Save style**, **Don't save style** and **Cancel**. Closing the window
while an [export](exporting.md) is running asks whether to **Stop it and
quit** or **Keep exporting**.

## Opening files in other ways

- **Drop a file on the window** from your file manager. A .lineup file
  opens; a MIDI file or a text chord sheet is imported; a recording
  (MP3, FLAC, Ogg, or a WAV longer than ten seconds) is [added to the
  song as a take](takes.md#adding-a-recording-made-before), on the
  track and from the bar where you drop it. Drop one at
  a time: with several, none is opened. A shorter WAV file is a sound: it
  becomes a channel that plays it, or, dropped on a channel of the
  rack, that channel's sound (see [The channel rack](channel-rack.md)).
- **Open a file with Ferrous Lineup** from your file manager, or give
  it to the program on the command line, as in `ferrous-lineup
  song.lineup`. A .lineup file opens, a MIDI file is imported, a text
  file is imported as a chord sheet, and a sound file starts a new song
  with it as a take.

## Recovering unsaved work

While a song has unsaved changes, Ferrous Lineup keeps a copy of it on
the side, brought up to date within about half a minute of each change,
even if you then leave the app alone. The copy is not your file, which only **Save** changes, and it is removed
once the song is saved or the app closes normally.

If the computer crashes or the app is closed without the chance to ask,
the next start offers the copy back:

![The question at the start, over the starting song: "Recover unsaved work?", "my-song.lineup" was not saved when Ferrous Lineup last closed, with the buttons Recover it and Discard it](images/recover.png)

**Recover it** opens the song as it was, still unsaved: save it to
keep it. **Discard it** throws the copy away. Each window keeps its own
copy, so when two were open, both are offered.

The copies live in Ferrous Lineup's data folder, in **recovery**:

| System | Data folder |
|---|---|
| Linux | `~/.local/share/ferrouslineup/` |
| macOS | `~/Library/Application Support/Ferrous-Lineup/` |
| Windows | `%APPDATA%\Ferrous Lineup\data\` |

The SoundFont downloaded in [Sounds](sounds.md) lives in the same
folder, in **soundfonts**.

## Sharing a song

A .lineup file opens on Linux, macOS and Windows alike. What it doesn't
hold are the sound files it uses: a WAV played by a channel stays where
it was, and the song only says where. To give someone a song that uses
them, send the sound files too, and put them back on the channels on
the other computer. A song's [takes](takes.md) are in a folder next to
it: send that folder with the song, and put it next to the song again,
and the takes are found.

A song made only with things an older version of Ferrous Lineup knows
is saved so that the older version opens it too. A song that uses
something newer tells the older version that it was made by a newer
one, instead of opening with parts missing.
