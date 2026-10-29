# The piano roll

The **piano roll** shows the notes of one channel in one pattern: time
from left to right, pitch from bottom to top, with a piano keyboard down
the left side. Each note is a bar as long as the note lasts. It is where
melodies, chords and bass lines are written and changed, note by note,
and where the app tells you what the notes make.

![The piano roll on the starting song's Chords pattern: the toolbar with the key, Snap to scale, Row…, Sets…, Micromodes…, Draw, Length and Tools; the chord buttons of C major with ii, IV and V lit after Am; the line "Step 13: Am: A minor, chord vi of C major." and the chord names C, Am, F and G over the grid, and the velocity lane under it](images/piano-roll-window.png)

## Opening it

- Press **F7**, or click **Piano roll** in the toolbar: it opens on the
  selected pattern and the selected channel.
- **Double-click** a clip of melody or chords in the playlist, or a
  channel's name in the channel rack.
- Right-click a channel's name in the rack and pick **Piano roll**.

The window's title names the pattern and the channel, as in **Piano
roll · Chords · Lineup Osc**. When the channel plays nothing in that
pattern, the piano roll says so and offers a button for each pattern
where it does play; for drums it offers the channel rack, where drums
are written.

## Reading the grid

- The rows of the notes of the key are lighter than the others, and the
  rows of the key note are brown. On the keyboard, a blue dot marks
  each note of the key and an orange one the key note. **Click a key**
  to hear it.
- Bar lines are stronger than beat lines.
- Over the grid, two rows name the chords: the first the chord this
  channel plays, the second, **All**, the chord all the channels make
  together.
- Above them, a line says what is under the pointer: the chord at that
  step, its name and its number in the key ("Am: A minor, chord vi of
  C major"). Another names the **set** of the notes, for the set
  theory of [Beyond keys: sets, rows and
  micromodes](sets-rows-micromodes.md).
- While the music plays, the step that sounds is shaded, and the line
  above the grid names the chord that sounds there.

## Writing notes

| Do this | And |
|---|---|
| **Click** an empty place | a note goes there (hold the button down and drag to move it straight away) |
| **Drag** a note | it moves, in time and in pitch |
| **Drag a note's right end** | it gets longer or shorter |
| **Right-click** a note, or right-drag over notes | they are deleted |
| **Click** a note | it is selected |
| **Ctrl+click** a note | it is added to the selection, or taken out |
| **Ctrl+drag** on an empty place | the notes in the box are selected |
| **Drag** a selected note | all the selected notes move together |

**Draw**, in the toolbar, says what a click puts down:

- **Single note**;
- **Chord in key (triad)** or **Chord in key (seventh)**: the chord of
  the key on the note you click, with the notes the key gives it, so
  in C major a click on D draws Dm and on G draws G (or Dm7 and G7);
- one kind of chord, the same on any note: **Major**, **Minor**,
  **Dominant seventh**, **Major seventh**, **Half-diminished seventh**,
  **Six-nine** and the others, thirty in all;
- **A set from the set browser…**.

For a chord, the list next to it chooses its **inversion**: which of
its notes is lowest (see [Chords and their names](chords.md)). The note
you click stays the root. **Length** is how long a new note is: **1
step**, **2 steps**, **1 beat**, **2 beats** or **1 bar**.

**Snap to scale** makes the notes you draw or move land on the notes of
the key. Right-click it for **Snap notes you play** too, which turns
the keys played on a MIDI or computer keyboard into the nearest note of
the key, both when you hear them and when you record them.

## The chords of the key

Under the toolbar, a button for each chord of the key, as in **Chords
in C major: I · C, ii · Dm, iii · Em…**. Click one to hear it. Point at
a chord in the grid, or hold one on a keyboard, and the chords that
often come next light up, with a line that says so: "After Am (vi),
try: Dm (ii), F (IV), G (V)." [What chords do](chord-functions.md)
explains why those.

## Tools

The **Tools** menu changes the selected notes, or every note when none
is selected:

| Tool | Key |
|---|---|
| **Up a semitone**, **Down a semitone** | **↑**, **↓** |
| **Up an octave**, **Down an octave** | **Shift+↑**, **Shift+↓** |
| **Up a note in the key**, **Down a note in the key** | **Alt+↑**, **Alt+↓** |
| **A step earlier**, **A step later** | **←**, **→** |
| **A beat earlier**, **A beat later** | **Shift+←**, **Shift+→** |
| **Quantize to the steps**: notes played a little off the grid move onto it | |
| **Snap to the key**: notes outside the key move to the nearest note of the key | |
| **Invert chords up**, **Invert chords down**: the lowest note of each chord goes up an octave, or the highest down | |
| **Arpeggiate chords**: each chord played one note after another, **up**, **down** or **up and down**, at the speed chosen under **Each note** | |
| **Select all** | **Ctrl+A** |
| **Copy**, **Cut**, **Paste**, **Duplicate** | **Ctrl+C**, **Ctrl+X**, **Ctrl+V**, **Ctrl+D** |
| **Delete selected notes** | **Delete** |

**Up a note in the key** is the musician's way to move a tune: C E G
in C major becomes D F A, a minor chord, not D F♯ A, because it stays
in the key. **Up a semitone** moves every note by the same distance,
out of the key.

Pasted or duplicated notes make the pattern longer when they need the
room. Notes copied in the song can be pasted in the style editor, and
the other way round.

## Velocity

Under the grid, the **Velocity** lane shows how hard each note is
played, as a bar from soft to loud. Click or drag in it to set the
velocity of the notes on that step, or of the selected ones there.

## Seeing the other parts

- **Ghost notes** shows the notes of the other channels faintly behind
  this one's, so you can write a bass line that fits the chords, and
  names the chords they all make together.
- **Voice leading** draws how each voice moves from one chord to the
  next (see [Voice leading and how parts move](voice-leading.md)).

## The key and the windows of sets

The **Key** menus at the top left are the song's key: picking another
moves the song, as described in [Keys and modes](keys-and-modes.md).
Next to them, the notes of the key are spelled out. **Row…**, **Sets…**
and **Micromodes…** open the windows of twelve-tone rows, pitch-class
sets and Kröpfl's micromodes (see [Beyond keys: sets, rows and
micromodes](sets-rows-micromodes.md)).

## Zoom

Over the piano roll, **Ctrl** with the mouse wheel, **+** or **−** makes
the steps wider or narrower, and **Alt** makes the rows taller or
shorter, around the pointer. **Ctrl+0** takes it back to 100%.
