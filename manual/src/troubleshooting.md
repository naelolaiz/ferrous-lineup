# Troubleshooting

## The app does not start

See [Installing it](installing.md): the AppImage must be allowed to run
and may need the FUSE 2 library; Windows asks once before it runs an
unsigned program; macOS asks you to allow the app once in **System
Settings > Privacy & Security**.

## No sound

Look at the left of the status bar, at the bottom of the window:

- **Audio: Default Audio Device · 48000 Hz · 2 channels**, or the name
  of your sound card: the app has a sound card, so the silence is
  somewhere in the song. Check the toolbar's **VOL** slider and the
  mixer's **Main** level (**F9**), a **Mute** or a **Solo** in the
  mixer, a channel's light in the channel rack and a track's light in
  the playlist: a light that is not green is muted.
- **Audio unavailable**, and a reason: the app could not open a sound
  card. Plug in your headphones or speakers, close any program that may
  hold the sound card for itself, and press **Try again** in **Edit >
  Settings… > Audio**.

The app plays through your system's default output. If you plug in
headphones while it runs, or change the default output, press
**Restart audio** in **Edit > Settings… > Audio** to move the sound
there.

## The instruments sound like simple synthesizers

That is the Lineup Osc, the app's own synthesizer, which plays until a
SoundFont is chosen. If you chose **Not now** when the app offered
GeneralUser GS, or the download failed, open **Edit > Settings… >
Audio** and tick **SoundFont instruments**: the offer comes back.
**Choose SoundFont…** there picks a `.sf2` file you already have (see
[Sounds](sounds.md)). If the download fails, the window says why; you
can download `GeneralUser-GS.sf2` with a browser from [its
page](https://github.com/mrbumpy409/GeneralUser-GS) and choose it.

## The sound stutters

A slow or busy computer can fall behind while it plays a big
arrangement, such as an orchestral demo with every instrument of the
score. Close other programs, and close the windows you don't need while
it plays. **File > Export WAV…** is never affected: it makes the file
apart from what you hear, note by note, so the file never stutters (see
[Exporting](exporting.md#a-sound-file)).

## MIDI

- **The MIDI menu says MIDI unavailable.** The system gave the app no
  MIDI at all. On Linux, MIDI needs the ALSA sequencer: the kernel
  module `snd-seq`. Load it with `sudo modprobe snd-seq`, then start the
  app again.
- **My keyboard is not in the list.** The MIDI menu looks for keyboards
  each time it opens: plug the keyboard in, switch it on, then open the
  menu again. A keyboard that another program has taken for itself may
  not show.
- **I play, but nothing sounds.** What you play sounds on the selected
  channel: click a channel's name in the channel rack. Check that
  channel is not muted.
- See [MIDI keyboards and synths](midi.md).

## The keys don't work

- **Space doesn't play**, and the other keys do nothing: a text box has
  the keyboard, such as the Library's search or a bar of the chord sheet
  you are typing in. Click an empty place first.
- **The letter keys play notes** instead of doing what you expect:
  **MIDI > Typing keyboard to piano** is on.
- **On a Mac**, the shortcuts use **⌘** where this manual writes Ctrl.
  See [Keyboard shortcuts](shortcuts.md).

## Saving and files

- **Save asks for a new file every time.** You opened a demo, which is
  read-only: save it once under a name of your own, and **Save** writes
  to that file from then on (see [Saving, opening and
  recovering](saving.md#demos-are-read-only)).
- **The app closed without saving.** Start it again: it offers to
  recover the unsaved work (see [Recovering unsaved
  work](saving.md#recovering-unsaved-work)).
- **On a Mac, double-clicking a .lineup file doesn't open it.** Use
  **File > Open…** in the app.

## The band and the chords

- **The band plays something different from what I wrote.** When you
  change the chords or the style, the band writes its parts again. To
  keep a part as it is, lock its track (see [Locking a track, and
  playing your part](locks.md)).
- **The chords of an imported MIDI file are wrong.** The app guesses
  them from the notes, and the guess can be wrong where the music is
  thin or moves fast. Correct them on the chord sheet (see [Opening MIDI
  files and recordings](importing.md#getting-the-chords-right)).
- **The chord names are not what I expected**, such as Do instead of C,
  or the other way round: **Edit > Settings… > Harmony > Note naming**
  chooses letters or solfège.

## The window

- **Everything is too small, or too big.** Hold the pointer over the
  menus or the toolbar and press **Ctrl++** or **Ctrl+−**, or change
  **Interface size** in **Edit > Settings… > Appearance**. **Ctrl+0**
  over anything takes it back to 100% (see [Undo, zoom, windows and
  settings](settings.md#zoom)).
- **A window is gone.** Its key brings it back: **F5** to **F9**, or
  **View**. A window folded up to its title opens again with the **▼**
  at its top left.
- **Starting afresh.** The app keeps its settings, and where its
  windows were, in the file `app.ron` in its data folder (see [Saving,
  opening and recovering](saving.md#recovering-unsaved-work) for where
  that is). With the app closed, rename or delete that file, and the
  next start is like the first one: in English, with the default
  settings, the windows where they first open, and no recent songs.
  Your songs, your styles and a downloaded SoundFont are not touched;
  if you had chosen another folder for your styles, choose it again
  with **Styles folder…** in **Edit > Settings… > Library**.

## Reporting a problem

**Help > About Ferrous Lineup** has **Report an issue**, which opens
the project's list of known problems, where you can look for yours or
describe a new one. Say which version you run (the About window shows
it) and on which system, and what you did, step by step, so someone
else can see the same thing. For a problem with what the app plays,
turn on **Edit > Settings… > Playback > Write a trace while playing**,
play the passage, and attach the trace file: it lists every note the
app played, and the setting says where it writes it.
