# Undo, zoom, windows and settings

![The Settings window on its Appearance page: the pages Appearance, Harmony, Library, Audio and Playback down the left, and Language (English, Español), Interface size and Editor zoom](images/settings.png)

## Undo and redo

Every change to the song can be undone: notes, steps, chords, clips,
tracks, a style tried from the Library, a new key or tempo, a mixer
setting. **Ctrl+Z** undoes the last change and **Ctrl+Shift+Z** (or
**Ctrl+Y**) does it again. The **Edit** menu names what they would undo
and redo, as in **Undo add notes**. The app keeps the last hundred
changes.

The style editor has an undo of its own, apart from the song's (see
[Your own styles](style-editor.md)).

## Windows

The chord sheet, the channel rack, the piano roll and the mixer open
over the playlist, each at the size of what it shows, and each where you
left it last. Each has a key, and a button in the toolbar that is lit
while it is open:

| Window | Key |
|---|---|
| Chord sheet | **F5** |
| Channel rack | **F6** |
| Piano roll | **F7** |
| Harmony keys | **F8** |
| Patterns and Library panel | **Alt+F8** |
| Mixer | **F9** |

- **×** at a window's top right closes it; its key or its button opens
  it again.
- **▼** at its top left folds it up to its title, out of the way.
- **View** lists every window and panel, and also opens the **Lead
  sheet**, the **Style editor…** and **Ear training**.
- **View > Chord sheet in the main window** puts the chord sheet in the
  playlist's place, with more bars to a line.

The app remembers the size of its own window, which windows were open
and where they were, and starts with them the next time. The first time,
it starts with the chord sheet, the Library and the harmony keys.

## Zoom

The zoom keys zoom what is under the pointer:

| Over | Ctrl with +, − or the wheel | Alt with +, − or the wheel |
|---|---|---|
| the playlist | bars wider or narrower | tracks taller or shorter |
| the piano roll | steps wider or narrower | rows taller or shorter |
| the channel rack | the whole rack bigger or smaller | |
| the chord sheet in the main window | text, buttons and menus bigger or smaller, everywhere: smaller names fit more bars on a line | |
| anything else | text, buttons and menus bigger or smaller, everywhere | |

The step under the pointer stays under the pointer. **Ctrl+0** takes
what is under the pointer back to 100%. On a Mac, **⌘** takes the place
of Ctrl. The playlist's two zooms are also at the right of the status
bar.

## Settings

**Edit > Settings…** has five pages. What you change is kept at once,
also for the next time; **Done** closes the window.

### Appearance

- **Language**: **English** or **Español**. The whole app changes at
  once, menus, hints and chord explanations included.
- **Interface size**: text, buttons and menus bigger or smaller
  throughout the app.
- **Editor zoom**: the playlist's and the piano roll's zooms, across
  (**Horizontal**) and up and down (**Vertical**), and the channel
  rack's, which is one for both. **Reset all zoom levels to 100%**
  resets them all.
- **Zoom shortcuts**: the keys above, as a reminder.

### Harmony

- **Note naming**: **Letters (C, D, E…)** or **Solfège (Do, Re, Mi…)**,
  for notes, keys and chords everywhere in the app (see [Notes,
  intervals and scales](notes-and-scales.md#letters-or-solfège)). Until
  you choose, they follow the language: letters in English, solfège in
  Spanish.
- **Roman numerals in minor keys**: how the chords on the third, sixth
  and seventh notes of a minor key are numbered. **bIII bVI bVII** counts
  them from the major scale, as pop and jazz books do: in A minor, C is
  ♭III. **III VI VII** counts them from the minor scale, as classical
  harmony books do: C is III. Both name the same chords (see [What
  chords do](chord-functions.md)).

### Library

The style list, the choices each style's window starts with, and the
folder of your own styles: see [Styles and the
Library](styles.md#the-library-settings).

### Audio

- **Output**: the sound card in use, with its sample rate and channels.
  **Restart audio** opens it again, for instance after plugging in
  headphones.
- **Microphone**: takes are recorded from your system's default input.
  **Move takes later by** moves their notes later on the grid, or
  earlier with a negative number, in milliseconds, for a sound card
  whose delays are not what it says (see
  [Takes](takes.md#if-the-notes-land-late-or-early)).
- **Source separation**, in builds that enable it: **Off** (the default)
  or a compiled backend, **charon_audio**, **stem-splitter-core** or
  **demucs-rs**. Choose its local executable and, for Charon, its
  HTDemucs ONNX model. Blank paths use helpers installed beside the app,
  then executables on `PATH`. Each backend's paths and the selection are
  remembered. Changes apply to the next analysis, including **Transcribe
  again**. Audio stays local, although helpers may download model weights
  the first time. See [Takes](takes.md#optional-source-separation) for the
  layers used and cancellation.
- **Instrument playback**: **SoundFont instruments** and **SoundFont
  drums**, and **SoundFont library**: the SoundFont in use and **Choose
  SoundFont…**. See [Sounds](sounds.md).

### Playback

- **Follow playhead**: keeps the music that plays in sight in the
  playlist, the chord sheet and the piano roll.
- **Count-in**, **Metronome**, **Practice speed** and the **Speed
  trainer**: see [Practising](practising.md).
- **Write a trace while playing**: writes every note played to a text
  file, to check what the app plays by reading it. It is off each time
  the app starts, and is useful when reporting a problem with playback.
