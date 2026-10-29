# Your own styles

The Library's styles are a good start, but the groove you hear in your
head may not be there. The **style editor** lets you write your own band
parts, with the channel rack and the piano roll you already know, and
save them as a style that sits in the Library next to the others.

![The style editor on a copy of Bossa nova: Start from…, Save style, Save as…, Undo, Redo, Preview ×4 and Stop preview along the top, then Name, BPM, the time signature, Bars, Swing, Progression, Description and Part assignments, and the style's lanes: Piano, Bass, Kick, Hat, Rim and Guitar](images/style-editor.png)

## Opening it

- In the **channel rack**, **Edit style…** opens a copy of the style the
  song plays, with every part its band plays, and **New style…** starts an empty one. **New style…** is
  also at the top of the Library.
- **View > Style editor…** opens the editor, or brings it to the front.
- In the **Library**, right-click a style: a built-in one says **Edit a
  copy in channel rack…**, one of yours **Edit in channel rack…**.

A built-in style opens as a copy, named "… copy", so the original stays
as it is. One of your own opens as itself, and **Save style** writes it
back to its file.

## What a style holds

A style of your own is **one repeating pattern**, one to sixteen bars
long, with a lane for each part: drums, a piano or other chord part, a
bass, and the guitar, strings, organ or soloist of a bigger band.

Along the top: **Start from…**, **Save style**, **Save as…**, **Undo**,
**Redo**, **Preview ×4** and **Stop preview**. Under them:

| Field | What it is |
|---|---|
| **Name** | the style's name in the Library |
| **BPM** | its usual tempo |
| time signature | its meter |
| **Bars** | how long the pattern is, 1 to 16 bars |
| **Swing** | its swing (see [Swing and feel](swing.md)) |
| **Progression** | a chord progression it brings, as Roman numerals or chord names, bars separated by `|`: `I | vi | ii7 | V7`. Leave it empty for a style that plays your chords |
| **Description** | a sentence, shown when you point at the style in the Library |
| **Part assignments** | which player each lane is: drums, piano, bass, guitar and so on |

Below is a channel rack with the style's lanes. Click steps to write
drums, as in any pattern. Click a lane's note preview to open its piano
roll, where drawing, moving, stretching notes and the **Tools** menu work
as they do in the song. **+ Add channel** adds a lane.

**Write the pitched parts over C major.** When the style is applied, each
note moves to the chord of the moment: a C plays the chord's root, an E
its third, a G its fifth, and the other notes follow the key or keep
their colour. So a bass line written as C, G, C, E in the editor plays
the root, fifth, root and third of whatever chord the song has.

**Start from…** copies an existing style into the editor, from the same
list as the Library, with its search; **Empty style** starts from
nothing.

**Preview ×4** plays the pattern four times and stops;
**Space** starts or stops it while the editor has the keyboard. The song
pauses while it plays.

The editor has its own **Undo** and **Redo** (**Ctrl+Z**,
**Ctrl+Shift+Z**), separate from the song's. Nothing reaches the song
until you apply the style; applying it is then one step to undo, like
any other style.

## Saving and using your styles

**Save style** writes to the file the style came from; **Save as…** asks
for a new one. Styles are saved as `.lineup-style` files, in a **styles
folder**: a `styles` folder in the app's data folder unless you choose
another in **Edit > Settings… > Library > Your styles > Styles folder…**.
The Library lists the styles in that folder; one saved elsewhere is not
listed, and the app says so.

Your saved styles join the Library's list, next to the built-in ones,
with **+ chords** if they bring a progression and **keeps chords** if
not. With **Group by genre** they have a group of their own, **User
styles**. Applying one works as for any style: **Playing patterns**
brings its lanes, **Voices / instruments** their sounds, and so on (see
[Styles and the Library](styles.md)).

A song keeps its own copy of the style it plays, inside its `.lineup`
file. Delete or move the style file, or open the song on another
computer, and the band still plays it.
