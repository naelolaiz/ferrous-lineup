# The style editor: writing your own band parts

Files: [`crates/lineup-app/src/style_editor.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/style_editor.rs),
[`crates/lineup-app/src/app/styles.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/styles.rs),
[`crates/lineup-core/src/library/user_style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/user_style.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)
and [`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs)

The [library](../step-8/library.md) is knowledge written as data: a
style is a chart, a comping pattern, a bass line and a rhythm, each a
few lines in a table. The **style editor** writes down the groove those
tables do not have, with the channel rack and piano roll you know.

## Where it is

- In the **channel rack**, **Edit style…** opens the style the song is
  playing; **New style…** starts an empty one.
- **View > Style editor…** opens the editor, or brings it to the front.
- In the **Library**, right-click a style or a rhythm. A built-in one
  says **Edit a copy in channel rack…**; a saved one says **Edit in
  channel rack…**.

**Edit style…** starts from the song's style. A saved style opens as
itself, so **Save style** writes back to its file. A built-in style or
rhythm opens as an editable copy, named "… copy". With no band, the
editor starts empty.

## What it edits

A user style is **one repeating pattern**, one to sixteen bars long. It
can hold drum lanes, a piano or other chordal lane, a bass lane, and
lanes for the guitar, strings, organ and soloist that a bigger band
uses. The top row has **Start from…**, **Save style**, **Save as…**,
**Undo**, **Redo**, **Preview ×4** and **Stop preview**. Under it are
**Name**, **BPM**, the time signature, **Bars**, **Swing**,
**Progression**, **Description** and **Part assignments**.

The rack is the same widget, holding the draft's rows: click steps to
write drums. Click a note preview to open the piano roll, where drawing,
moving, stretching and the Tools menu work as in a pattern. The editor's
roll has its own instance id, window and keys, and never edits the
song's roll.

Pitched parts are written **over C major**. When the style is applied,
each note moves to the chord of the moment: a C plays the root, an E the
third, a G the fifth, and the other notes follow the key or keep their
colour. The fields say: "Write pitched parts over C major. They follow
the song's chords when applied."

**Preview ×4** plays the pattern four times and stops. **Space**
starts or stops it while the editor has the keyboard. The preview is a
phrase on the audio thread; the song pauses. **Stop preview** ends it
early.

Its history is its own. **Ctrl+Z**, **Ctrl+Shift+Z** and **Ctrl+Y**
undo and redo the draft without touching the song's history. Nothing
reaches the song until you apply the saved style; then applying it is
one undoable edit.

## User styles

**Save style** writes to the file the style came from. **Save as…**
asks for a new one. Both use **`.lineup-style`**, a RON file with the
style's name, meter, tempo, swing, chart, bars and lanes, each lane
carrying its instrument and notes. If you type another extension, it is
replaced: `my groove.txt` becomes `my groove.lineup-style`.

Styles live in a **styles folder**, by default a `styles` subfolder of
the app's data folder. **Edit > Settings… > Library > Your styles >
Styles folder…** chooses another. A style saved outside it is not
listed; the notice says the Library lists the folder, not the file you
chose.

The Library reads every `.lineup-style` file at startup, and again
after a save or a folder change. It sorts them by name and reports a
damaged file. Saving writes a temporary file and renames it, so a crash
leaves the old style whole.

Styles saved in that folder join **Presets**, beside the built-in styles
and rhythms. They have no genre, so **Group by genre** puts them
under **User styles (N)**; the heading's count includes them. A style
with a chart says **+ chords**, one without says **keeps chords**.
**Start from…** shows the same list and search, with **Empty style**;
choosing one copies it into the draft.

Applying a saved style works like any other preset. **Playing
patterns** brings its lanes, **Voices / instruments** their sounds,
**Time signature**, **Tempo** and **Swing** those, and **Chord
progression** its chart. Leave the progression unchecked to keep the
song's chords. See [the Library chapter](../step-8/library.md#styles)
and [Quicker to work in](workflow.md).

### The style travels with the song

When **Playing patterns** is applied, the whole `UserStyle` is stored in
the band:

```rust
pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
pub custom_instruments: Option<Box<crate::library::UserStyle>>,
```

The first holds the patterns, the second the instruments and their
settings. The project needs no access to the `.lineup-style` file:
delete it, move it, or open the song on another computer, and the band
still plays. **Format 22** introduced this ([the project file
format](../appendix-project-format.md)); such a project is written as
22, or higher if it also uses a later feature. Older versions cannot
read such a project.

That is why a style can disappear from **Presets** without breaking a
song that uses it: the song keeps its own copy.

## How it works

The editor does not have a second rack or a second piano roll. It has a
throwaway one-pattern `Project`, built by
`UserStyle::pattern_project`, and keeps it beside the style and the
role of each row:

```rust
struct Draft {
    style: UserStyle,
    project: Project,
    roles: Vec<Player>,
}
```

It then calls the same widgets as the song windows. The rack returns
`RackAction`s, the roll returns `RollAction`s, and the editor applies
them to `draft.project` instead of `LineupApp::project`. Adding a drum,
moving a note or opening the roll go through the code that already does
it. Each change records the whole `Draft` in its own
`History<Draft>`.

`sync` copies the draft back into the style after each frame: meter,
tempo and swing, then one `StyleLane` per channel with that channel's
notes, clipped to the pattern length. `UserStyle::validate` checks the
result before it is written or applied: one to sixteen bars, at most
256 steps, one lane per pitched player, and every note inside the
pattern.

Applying uses the same `StyleParts` as a built-in preset
(`library/apply.rs`), so the Library's checkboxes mean the same for
both. `Band::apply_user_style` clones the project first, validates the
style, and only then changes the band. It stores the style in
`custom_rhythm` or `custom_instruments`, writes the band's patterns, and
lets `UserStyle::play_lane` repeat the bars and move each pitch to the
chord under it.

## Trying it

1. Run `cargo run --release` and open **File > New**. Press **F6** for
   the channel rack and click **Edit style…**. The editor opens with the
   current style as a draft, title **Style editor · channel rack**.
2. Open **Start from…**, type `bossa` in **Search presets**, and choose
   a style. Try **Preview ×4**.
3. Click a few kick and snare steps. Click a piano note preview to open
   the editor's piano roll; draw a chordal figure, then press **Space**.
4. Click **Save as…**, name it `My groove`, and save it in the styles
   folder. Close the editor, open the Library, and find **My groove**
   under **Presets** (or **User styles** when grouped).
5. Apply it with **Playing patterns** ticked and **Chord progression**
   unchecked. The song keeps its chords and the band plays your
   pattern. Save the project, close the app, then move the
   `.lineup-style` file away and open the project again: the band still
   plays it.
