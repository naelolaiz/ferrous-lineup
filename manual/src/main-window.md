# The main window

![The main window when the app starts for the first time: the menus and the toolbar at the top, the Library on the left, the chord sheet of the starting song over the playlist, and the harmony keys along the bottom](images/main-window-start.png)

From top to bottom, the window has:

1. **The menus**: **File**, **Edit**, **View**, **MIDI** and **Help**.
   Next to them, a line of hints says what the thing under the mouse
   does.
2. **The toolbar**: playing, the tempo, the position, the pattern, the
   windows and the volume.
3. **The Patterns and Library panel** on the left, and **the playlist**
   in the middle, where the song is laid out in time. Windows such as
   the channel rack, the chord sheet and the piano roll open over it.
4. **The harmony keys** along the bottom: the key, the time signature
   and a big key for each chord of the key.
5. **The status bar**: messages, the sound output, and the playlist's
   zoom.

## The menus

| Menu | What is in it |
|---|---|
| **File** | **New** (an empty chord sheet with a band), **New beat** (an empty pattern and a drum kit), **Open…**, **Open recent**, **Demos**, **Add a recording…** (a recording made before, laid on the song as a [take](takes.md#adding-a-recording-made-before)), **Import MIDI file…**, **Import chord sheet…**, **Save**, **Save as…**, the exports (**Export WAV…**, **Export video…**, the song or the pattern as MIDI, the chord sheet as text) and **Quit** |
| **Edit** | **Undo** and **Redo**, which name what they undo, and **Settings…** |
| **View** | each window and the panel, shown or hidden; **Chord sheet in the main window**, the **Lead sheet**, the **Style editor…** and **Ear training** |
| **MIDI** | **Typing keyboard to piano**, **Accompaniment follows chords**, a keyboard to play with (**Input**) and a synth to play on (**Output**) |
| **Help** | **Keyboard shortcuts**, **User manual** and **About Ferrous Lineup** |

## The toolbar

From left to right:

| Control | What it does |
|---|---|
| **PAT** / **SONG** | **Pattern mode** loops the selected pattern; **song mode** plays the playlist. **L** switches between them. |
| ⏮ | goes to the start (**Home**) |
| ⏪ ⏩ | go back or forward a bar (**Page Up**, **Page Down**) |
| ▶ | plays from the playhead (**Space**). While playing, the same button is ❚❚ **Pause**: it stops there, and ▶ carries on from there. |
| ⏹ | stops, and goes back to where you last put the playhead (**Space** while playing) |
| ● | records what you play on a MIDI keyboard or the computer keyboard into the pattern (see [MIDI keyboards and synths](midi.md#recording)) |
| microphone | records a **take**: sing or whistle along with the song, and your notes are written into a part (see [Takes](takes.md)) |
| **Follow playhead** | keeps the bar playing in sight in the playlist, the chord sheet and the piano roll. Off, you can scroll freely while the song plays. |
| **Count-in** | one bar of clicks before the music starts |
| **Metronome** | a click on every beat |
| **TEMPO** | the tempo in beats per minute: drag up or down to change it, double-click to type it |
| **BAR : BEAT : STEP** | where the playhead is: in the pattern (PAT) or in the song (SONG) |
| **PATTERN** | the selected pattern, which the channel rack and the piano roll show; **+** adds a new one, **⋯** renames, clones or deletes it, and the last menu sets its length |
| window buttons | the **chord sheet** (**F5**), the **channel rack** (**F6**), the **piano roll** (**F7**), the **harmony keys** (**F8**) and the **mixer** (**F9**). A button is lit while its window is open. |
| **VOL** | the master volume |

Point at any button to see its name and its key.

## The Patterns and Library panel

The panel on the left has two tabs:

- **Library**, in front when the app starts: the styles, rhythms and
  fills the band can play, and your own styles. Click one to try it in
  your song. See [Styles and the Library](styles.md).
- **Patterns**: the song's patterns, the blocks the playlist is made of.
  Click one to select it. See [Patterns and the
  playlist](patterns-and-playlist.md).

**Alt+F8**, or **View > Patterns and library**, hides the panel to give
the playlist more room.

## The playlist

The playlist is the whole song laid out in time: bars from left to
right, and **tracks** one under another, each holding **clips** of
patterns. Above the tracks are three strips:

- **Sections**: drag along it to mark a section, such as Intro, Verse or
  Chorus.
- **Tempo**: the song's tempo as a line; click it to change the tempo
  from a bar on.
- **Bars**: the bar numbers, with the time signature and the key where
  they change. Click a number to move the playhead there; drag along
  them to loop part of the song; right-click one for its menu.

[Patterns and the playlist](patterns-and-playlist.md) tells how to work
with it.

## The windows

Each window opens over the playlist, at the size of what it shows, and
remembers where you left it. Close one with its **×**, its window
button, or its key.

| Window | Key | What it is for |
|---|---|---|
| [Chord sheet](chord-sheet.md) | **F5** | the song's chords, bar by bar, for the band |
| [Channel rack](channel-rack.md) | **F6** | the steps of a pattern: drums and samples |
| [Piano roll](piano-roll.md) | **F7** | melodies and chords, note by note |
| [Harmony keys](harmony-keys.md) | **F8** | the chords of the key (shown along the bottom) |
| [Mixer](mixer.md) | **F9** | levels, pan, reverb, effects and instruments |

**View** also opens the **Lead sheet**, the **Style editor…** and
**Ear training**.

The first time, the app starts with the chord sheet, the Library and
the harmony keys open. After that, it starts with the windows that were
open when you closed it. What comes to the front then depends on what
you open: a song opens with the chord sheet in front, a beat (**File >
New beat**) with the channel rack.

## The status bar

The bottom line says what just happened ("Saved", "Instruments play
from GeneralUser GS", what went wrong and why) and which sound output
is in use. At its right are the playlist's two zooms: how wide the bars
are (↔) and how tall the tracks are (↕).

## Keys that work everywhere

| Key | What it does |
|---|---|
| **Space** | play and stop |
| **L** | pattern mode or song mode |
| **Ctrl+Z**, **Ctrl+Shift+Z** | undo, redo |
| **Ctrl+S** | save |
| **F1** | the list of every keyboard shortcut |
| **F5** to **F9** | the windows |

**F1**, or **Help > Keyboard shortcuts**, lists every key the app
answers to, window by window; the same list is in [Keyboard
shortcuts](shortcuts.md).

**Help > User manual** opens this manual in your web browser, in the
language the app speaks.

**Help > About Ferrous Lineup** shows the version you are running, the
license, and links to this manual (on the web and as one PDF file), the
book, the source code and the list of known problems.
