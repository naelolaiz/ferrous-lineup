# The channel rack

The **channel rack** (**F6**, or **View > Channel rack**) shows one
pattern, a row for each **channel**: each drum, and each instrument
that plays notes. It is where you write drums, step by step, as on a
drum machine.

![The channel rack on the Chords pattern while it plays in pattern mode: the light over the step that sounds lit orange and its column outlined down every row, the drum rows empty with the line "No drum steps in Chords: click a step to add one. Drums are in: Beat, Fill", and the Lineup Osc row showing the chords as a small piano roll](images/channel-rack.png)

## Reading it

The window's title names the pattern it shows, as in **Channel rack ·
Beat**. Each row has, from the left:

- a **light**, green while the channel is heard: click it to mute or
  unmute the channel;
- a **knob** for its volume: drag it up or down;
- a **padlock**, to keep the channel's notes and sound when you try
  styles (see [Locking a track, and playing your part](locks.md));
- its **name**;
- its **steps**: sixteen to a bar, one for each sixteenth note, shaded
  dark and light beat by beat so you can count them.

A channel that plays melodies or chords shows a small picture of its
notes instead of steps. Click it to open the piano roll.

Over the steps is a row of small lights. While the pattern plays, the
light over the step that sounds is lit orange and its column is
outlined, down every row.

## Writing drums

**Click a step** to light it, and the drum plays there; click it again,
or right-click it, to clear it. Press **Space** with the rack in front
to hear the pattern over and over (or press **L** for pattern mode and
**Space**), and change steps while it plays.

A good first beat in 4/4, in one bar of sixteen steps:

| Channel | Steps | What you hear |
|---|---|---|
| Kick | 1, 9 | the bass drum on beats 1 and 3 |
| Snare or Clap | 5, 13 | the backbeat, on beats 2 and 4 |
| Hat | 1, 3, 5, 7, 9, 11, 13, 15 | eighth notes |

That is the backbeat of rock and pop: the low drum on the strong beats,
the snare on the weak ones. Move a kick to step 11, the "and" of beat 3,
and the beat gets a push.

When a pattern has no drum steps but others do, the rack says so, with a
button for each pattern that has them, as in "No drum steps in Chords:
click a step to add one. Drums are in: Beat, Fill". Songs often keep
their drums in patterns of their own.

## Along the top

- **Edit style…** and **New style…** open the style editor (see [Your
  own styles](style-editor.md)).
- **▶ Play pattern** plays every channel of this pattern; while it
  plays, the button is **Pause pattern**, and **■ Stop pattern** stops
  it.
- While the song plays, **▶ Play over the song** plays the pattern on
  top of it instead, from the next bar and in time with it, until you
  click it again: a way to try a new pattern with the rest of the song.
- The list picks which pattern the rack shows, as the PATTERN list in
  the toolbar does.

## Channels

- **Click a name** to select the channel and hear it; click it again,
  or an empty part of the rack, or press **Escape**, to select none.
  The selected channel is the one a MIDI keyboard or the computer
  keyboard plays.
- **Double-click a name** to open the channel's piano roll.
- **Right-click a name** for its menu: **Piano roll**, its
  **Instrument** (see [Sounds](sounds.md)), sending it to an external
  synth over MIDI (see [MIDI keyboards and synths](midi.md)), **Lock**
  or **Unlock**, and **Remove channel**. **Delete** removes the selected
  channel too, and **Ctrl+Z** brings it back.
- **➕ Add channel**, under the rows, adds a drum (Kick, Snare, Clap,
  Hat, Open hat, Rim, Low tom, High tom, Ride, Crash, Cowbell, Shaker,
  Conga), a **Lineup Osc (synth)** to play notes, or a **MIDI Out
  (external synth)**, whose notes play on a synth outside the app.

### Your own sounds

Drag a **WAV** file from your file manager onto a channel's name, and
that channel plays it on every step you light, as a drum machine plays
its samples. Dropped anywhere else on the rack, it gets a channel of its
own. The song keeps where the file is, not the sound itself, so leave
the WAV where it is: moved or deleted, the channel has nothing to play.
A WAV longer than ten seconds is taken for a whole recording and
[imported](importing.md#recordings-experimental) as a new song
instead.

## Zoom

Over the rack, **Ctrl** with **+** and **−**, or with the mouse wheel,
makes the whole rack bigger or smaller, and **Ctrl+0** takes it back to
100%.
