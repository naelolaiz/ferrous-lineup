# Keyboard shortcuts

The app's own list is **Help > Keyboard shortcuts**, or **F1**. It is
built from the same key constants the windows read, in
[`crates/lineup-app/src/app/shortcuts.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/shortcuts.rs),
so it always names the keys the app is actually listening for. This
page is the same list in a form you can print ([Quicker to work
in](step-9/workflow.md#every-key-in-a-list) describes the window).

The tables write **Ctrl**, as on Linux and Windows. On macOS, read
**⌘** for Ctrl and **Option** for Alt. The F1 window draws whichever
your system uses.

## Anywhere

These work wherever the keyboard is, except while you are typing in a
text field.

| Key | What it does |
|---|---|
| **Space** | Play or stop. In the channel rack or the piano roll it plays or stops the pattern. |
| **Home** | Go to the start. |
| **End** | Go to the end. |
| **PageUp** / **PageDown** | Back / forward one bar. |
| **L** | Switch between pattern mode and song mode. |
| **F1** | Open or close the keyboard-shortcut list. |

## Windows

Show or hide each window or panel.

| Key | What it shows |
|---|---|
| **F5** | Chord sheet |
| **F6** | Channel rack |
| **F7** | Piano roll |
| **F8** | Harmony keys |
| **Alt+F8** | Patterns and library panel |
| **F9** | Mixer |

## File and Edit

**File**

| Key | What it does |
|---|---|
| **Ctrl+N** | New |
| **Ctrl+O** | Open |
| **Ctrl+S** | Save |
| **Ctrl+Shift+S** | Save as |
| **Ctrl+Q** | Quit |

**Edit**

| Key | What it does |
|---|---|
| **Ctrl+Z** | Undo |
| **Ctrl+Shift+Z** or **Ctrl+Y** | Redo |

## Zoom

The keys zoom what is under the pointer: the playlist, the piano roll,
the channel rack, or the whole interface anywhere else. The step under
the pointer stays under the pointer.

| Key | What it does |
|---|---|
| **Ctrl++** / **Ctrl+−** | Wider / narrower |
| **Alt++** / **Alt+−** | Taller / shorter |
| **Ctrl+0** | Back to 100% |
| **Ctrl+wheel** | Wider or narrower |
| **Alt+wheel** | Taller or shorter |

On most keyboards `=` is the unshifted `+` key, and **Ctrl+=** and
**Alt+=** zoom the same way. The F1 list names the `+` forms.

## Chord sheet

These work after you click a bar.

| Key | What it does |
|---|---|
| **Arrows**, **Tab**, **Shift+Tab** | Move between bars. With Shift, select the bars you pass. |
| **Enter** or **F2** | Type in the bar. Starting to type does the same. |
| **Delete** or **Backspace** | Clear the selected bars. |
| **Ctrl+A** | Select every bar. |
| **Ctrl+C** | Copy the bars as text. |
| **Ctrl+X** | Cut the bars. |
| **Ctrl+V** | Paste bars, or a whole chord chart. |
| **Escape** | Select nothing. |

## Piano roll

These act on the selected notes, or on all of the channel's notes when
none is selected.

| Key | What it does |
|---|---|
| **↑** / **↓** | Up / down a semitone |
| **Shift+↑** / **Shift+↓** | Up / down an octave |
| **Alt+↑** / **Alt+↓** | Up / down a note in the key |
| **←** / **→** | A step earlier / later |
| **Shift+←** / **Shift+→** | A beat earlier / later |
| **Ctrl+C** | Copy |
| **Ctrl+X** | Cut |
| **Ctrl+V** | Paste |
| **Ctrl+D** | Duplicate |
| **Delete** or **Backspace** | Delete |
| **Ctrl+A** | Select every note |
| **Escape** | Select nothing |

## Playlist

These act on the selected clips.

| Key | What it does |
|---|---|
| **Ctrl+C** | Copy |
| **Ctrl+X** | Cut |
| **Ctrl+V** | Paste |
| **Ctrl+D** | Duplicate |
| **Delete** or **Backspace** | Delete |
| **Ctrl+A** | Select every clip |
| **Escape** | Select nothing |

## Channel rack

These work after you click a channel's name.

| Key | What it does |
|---|---|
| **Delete** or **Backspace** | Delete the channel. |
| **Escape** | Select no channel. |

## Harmony keys

| Key | What it does |
|---|---|
| **1** to **7** | Play the chord on each degree of the key. |

## Typing keyboard

These play notes when **MIDI > Typing keyboard to piano** is on.

| Key | What it plays |
|---|---|
| **Z X C V B N M** | White keys from C4 |
| **S D G H J** | Black keys from C#4 |
| **Q W E R T Y U I O P** | White keys from C5 |
| **2 3 5 6 7 9 0** | Black keys from C#5 |

## Style editor

| Key | What it does |
|---|---|
| **Space** | Play the style. |
| **Ctrl+S** | Save the style. |
| **Ctrl+Shift+S** | Save the style as. |
| **Ctrl+Z** | Undo. |
| **Ctrl+Shift+Z** or **Ctrl+Y** | Redo. |
| **Escape** | Select no channel. |

The style editor has its own history, separate from the song's; see
[The style editor](step-9/style-editor.md).
