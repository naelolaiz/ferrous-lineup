# Your own styles: the style editor

Files: [`crates/lineup-app/src/style_editor.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/style_editor.rs),
[`crates/lineup-app/src/app/styles.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/styles.rs),
[`crates/lineup-core/src/library/user_style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/user_style.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)
and [`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs)

The [library](library.md) is knowledge written as data: a
style is a chart, a comping pattern, a bass line and a rhythm, each a
few lines in a table. The **style editor** lets you write a groove those
tables do not have, with the channel rack and piano roll you already
know, and save it as a style of your own. The band then plays it over
any chord sheet, moving every note to the chord of the moment.

## Trying it

1. Open **File > New**. Press **F6** for the channel rack and click
   **Edit style…**. The editor opens with a copy of the band the song
   plays, **Pop beat copy**, in a window titled **Style editor ·
   channel rack**.
2. Open **Start from…**, type `bossa` in **Search presets**, and choose
   a style. Click **Preview ×4**.
3. Click a few kick and snare steps. Click a piano note preview to open
   the editor's piano roll; draw a chordal figure, then press **Space**
   to hear it.
4. Click **Save as…**, name it `My groove`, and save it in the folder
   the dialog opens. Close the editor, open the **Library**, and find
   **My groove** among the presets (under **User styles** when they are
   grouped by genre).
5. Apply it with **Playing patterns** ticked and **Chord progression**
   unticked. The song keeps its chords and the band plays your pattern
   over them. Save the project, close the app, move the `.lineup-style`
   file away and open the project again: the band still plays it.

## Where it is

- In the **channel rack**, **Edit style…** opens the style the song is
  playing ("Edit a copy of the current style’s repeating parts", says
  its hint); **New style…** starts an empty one.
- **View > Style editor…** opens the editor, or brings it to the front.
- The **Library** has a **New style…** button over its list. Right-click
  a style or a rhythm there: a built-in one offers **Edit a copy in
  channel rack…**, one of yours **Edit in channel rack…**.

**Edit style…** starts from what the song plays. A style of yours opens
as itself, so **Save style** writes back to its file. A band opens as a
copy of all its players, two bars of what they play over a C chord,
named "… copy". A style or rhythm being tried in a pattern opens as a
copy of it, and with nothing to copy the editor starts empty: a kick, a
snare, a hi-hat and a piano, over the progression `I | V | vi | IV`.

## What it edits

A user style is **one repeating pattern**, one to sixteen bars long and
at most 256 steps (so in 12/8, with 24 steps a bar, ten bars at most).
It holds drum lanes, one for each drum sound, and at most one lane
for each pitched player of the band: piano, bass, guitar, strings,
organ, soloist and horns.

The top row has **Start from…**, **Save style**, **Save as…**,
**Undo**, **Redo**, **Preview ×4** and **Stop preview**. Under it are
**Name**, **BPM**, the time signature, **Bars** and **Swing**; then
**Progression**, the chords the style brings, in Roman numerals or
chord names with bars separated by `|`; **Description**; and **Part
assignments**, which says which player each row is, so a row named
"Rhodes" can be the piano.

The rack is the same widget as the song's, holding the draft's rows:
click steps to write drums. Click a note preview to open the piano
roll, where drawing, moving, stretching and the Tools menu work as in a
pattern. The editor's roll has its own window and keys, and never edits
the song's roll.

**Preview ×4** plays the pattern four times, as written over C, and
stops; **Space** starts or stops it while the editor has the keyboard,
and **Stop preview** ends it early. The song pauses while it plays. **Ctrl+S** saves and
**Ctrl+Shift+S** saves as.

The editor has a history of its own. **Ctrl+Z**, **Ctrl+Shift+Z** and
**Ctrl+Y** undo and redo the draft without touching the song's history.
Nothing reaches the song until you apply the saved style, and applying
it is then one step to undo.

## The music: one part over every chord

The hint under the fields says it: "Write pitched parts over C major.
They follow the song’s chords when applied." A player who learns a
comping figure learns it as a shape, root, third, fifth, seventh, and
plays the same shape over every chord. Writing the shape over C major
gives it fixed names: C is the root, E the third, G the fifth, B the
seventh.

### What each written note plays

| Written over C | Plays over the chord |
|---|---|
| C | its root |
| E | its third, or the note a suspended chord has instead (F in Csus4) |
| G | its fifth, diminished or augmented if the chord's is |
| B | its own seventh (the sixth of a 6 chord); with none, the key's seventh |
| D, F, A | the key's note one, three or five steps above the root: the 9th, 11th or 13th |
| C♯, E♭, F♯, A♭, B♭ | the same distance from the root: a colour kept |

The written arpeggio C–E–G–B, in a song in C major:

| Chord | Plays |
|---|---|
| Dm7 | D F A C |
| G7 | G B D F |
| Cmaj7 | C E G B |
| Am | A C E G |
| F | F A C E |
| Csus4 | C F G B |
| Am6 | A C E F♯ |

Over Am and F, which have no seventh, the B takes the key's own
seventh above the root: Am plays as Am7 and F as Fmaj7, as a pianist
comping in C major would colour them. If you want plain triads, leave
the B out.

### A scale becomes the mode of each chord

Write the whole scale, C D E F G A B, and the band plays it from each
chord's root with the notes of the key:

| Chord | Plays | Which is |
|---|---|---|
| Dm7 | D E F G A B C | D Dorian |
| G7 | G A B C D E F | G Mixolydian |
| Am | A B C D E F G | A Aeolian |
| Bdim | B C D E F G A | B Locrian |
| F | F G A B C D E | F Lydian |

These are the modes of C major (see [Scales, modes and
keys](../theory/scales.md#the-music-briefly)): the notes of the
key, started from each chord. So the D you write over C, a ninth, is
the key's E over Dm7 but F over Em, and the A, a sixth, is C over Em,
not C♯: the line never leaves the key while the chords stay in it.

### Chords from outside the key

A chord with a note outside the key, as a borrowed Fm or the E7 that
leads to Am, cannot take the key's notes for its ninth or sixth. It
keeps the distances written over C, the notes of its own major scale,
except that a minor chord takes the flat seventh:

| Chord | C D E F G A B plays | C–E–G–B plays |
|---|---|---|
| Fm | F G A♭ B♭ C D E♭ | F A♭ C E♭ |
| E7 | E F♯ G♯ A B C♯ D | E G♯ B D |
| B♭ | B♭ C D E♭ F G A | B♭ D F A |

This is simpler than the [soloist's chord
scales](soloist.md#chord-scales), which would give the E7
that leads to Am the F and C of A minor. A style is a pattern that
should sound the same over every chord; the soloist chooses notes one
by one.

The chromatic notes keep their distance from the root. A written E♭,
the minor third over C, plays B♭ over G7, the "blue" third against the
chord's B; a written B♭ plays C over Dm7 and E♭ over F.

### The bass, the register and held notes

The bass follows the same rules, with one more: its Cs play the bass
note of a slash chord. A line written C–G–C–E (root, fifth, octave,
third) plays D–A–D–F over Dm7, and E–G–E–E over C/E: the Cs become E,
the G stays the chord's fifth and the E its third.

A part moves at most a tritone from where you wrote it. Over a chord on
C to F♯ it moves up; over G to B it moves down, so a figure written
around middle C stays around it: over G7 the arpeggio is G3 B3 D4 F4,
below middle C, not a fifth above.

A note held across a chord change is cut there and goes on in the new
chord. Drums are copied as written. A style of two bars or more counts
its bars from the start of the song: a two-bar style plays its first
bar in bars 1, 3, 5 and so on, and in a bar shorter than the pattern's
the notes that do not fit are left out.

### Who plays what

A style with only drums, such as a copy of a rhythm, changes only the
drums: the band's piano, bass and the others go on as they were. Once a
style has a pitched lane, it takes over the band's harmony: the piano,
bass, guitar, strings and organ play only the lanes you wrote, and one
with no lane is silent. The soloist and the horns play their own parts
unless the style has a lane for them. Your parts also stop for the
[shots, holds, rests and pushes](stops.md) of the sheet, and
play louder in the [b variation](song-form.md#variations-a-and-b-the-band-digs-in).

## Your styles are files

**Save style** writes to the file the style came from. **Save as…**
asks for a new one. Both write **`.lineup-style`** files: if you type
another extension, it is replaced, so `my groove.txt` becomes
`my groove.lineup-style`.

The file is the style itself, written as RON (Rusty Object Notation,
the format of the project files) by `serde`:

```rust
pub struct StyleLane {
    pub player: Player,
    pub channel: Channel,
    pub notes: Vec<NoteEvent>,
}
```

```rust
pub struct UserStyle {
    pub format: u32,
    pub name: String,
    pub about: String,
    pub time: TimeSignature,
    pub tempo: Tempo,
    pub swing: u8,
    pub mode: ScaleKind,
    pub chart: String,
    pub bars: usize,
    pub lanes: Vec<StyleLane>,
}
```

Each lane carries its whole `Channel`: the instrument, its General
MIDI program, its volume and its other settings. Names are
written in letters even when the app shows solfège, so a style saved in
one language opens in the other.

Styles live in a **styles folder**, by default a `styles` folder in
the app's data folder. **Edit > Settings… > Library > Your styles >
Styles folder…** chooses another. The Library lists only that folder:
a style saved elsewhere is not listed, and the notice after saving says
so: "Saved style: … The Library lists the styles in …; Settings >
Styles folder changes it."

The Library reads every `.lineup-style` file in the folder at startup,
and again after a save or a change of folder, sorts them by name, and
reports any it cannot read: "Some styles could not be loaded: …".
They join the presets beside the built-in styles and rhythms. They have
no genre, so **Group by genre** puts them under **User styles (N)**. A
style with a progression says **+ chords**; one without says **keeps
chords**. **Start from…** in the editor shows the same list and search,
with **Empty style** first.

Applying one of your styles works as for any preset (see [Which parts
to take](library.md#which-parts-to-take)): **Playing
patterns** brings its lanes, **Voices / instruments** their sounds, and
**Time signature**, **Tempo**, **Swing** and **Chord progression**
those. Leave the progression unticked to keep the song's chords.

### The style travels with the song

When **Playing patterns** is applied, the whole `UserStyle` is stored
in the band:

```rust
pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
pub custom_instruments: Option<Box<crate::library::UserStyle>>,
```

The first holds the patterns, the second the instruments and their
settings. The project needs no access to the `.lineup-style` file:
delete it, move it, or open the song on another computer, and the band
still plays. A project that carries a style of yours needs at least
format 22 (see [the project file
format](../reference/project-format.md)). That is also why a style
can disappear from the Library without breaking a song that uses it:
the song keeps its own copy.

## How it works

### A pattern project of its own

The editor has no second rack or second piano roll. It keeps a
throwaway one-pattern `Project`, built by
`UserStyle::pattern_project`, beside the style and the player of each
row:

```rust
struct Draft {
    style: UserStyle,
    project: Project,
    roles: Vec<Player>,
}
```

It then calls the same widgets as the song windows (see [Drawing
returns actions](../program/editors.md#drawing-returns-actions)).
The rack returns `RackAction`s, the roll returns `RollAction`s, and
the editor applies them to `draft.project` instead of the song. Adding
a drum, moving a note or opening the roll go through the code that
already does it. The one action it refuses is a new pattern: "Styles
contain a single pattern. Save new patterns from the song piano roll."

Its undo is the app's own `History<T>` (see
[`History<T>`](../program/undo.md#historyt)), with the whole draft as
`T`:

```rust
history: History<Draft>,
```

### From the rows back to the style

After each frame, `sync` copies the draft back into the style: meter,
tempo and swing, then one `StyleLane` per channel with that channel's
player and notes, cut at the end of the pattern:

```rust
draft.style.time = draft.project.time_signature;
draft.style.tempo = draft.project.tempo;
draft.style.swing = draft.project.swing;
```

`UserStyle::validate` checks the result before it is saved or applied,
one rule at a time, each with a message for the user:

```rust
if !(1..=16).contains(&self.bars) || self.length() > MAX_STEPS {
    return Err(tr("The pattern must fit in 1–16 bars and 256 steps").to_string());
}
```

```rust
if lane.player != Player::Drums && !players.insert(lane.player) {
    return Err(trf(
        "Assign only one lane to {}",
        &[&tr(lane.player.name())],
    ));
}
```

The others check the format, the name, the progression (every bar must
read as chords), the number of lanes, the swing, each lane's settings,
and that every note is inside the repeating pattern. Reading a file
calls `validate` too, so a damaged or hand-edited style is refused
with the reason instead of playing nonsense.

### Applying: on a copy first

`Project::apply_user_style` changes the song in several steps: the
band, the meter, the tempo, the chords. Reading the progression into
the song's bars can fail halfway, in a meter the chart does not fit.
So it works on a clone, and only a finished result replaces the song:

```rust
style.validate()?;
// The chart is read into the song's bars, which can fail halfway
// in another meter, after the band and the tempo changed: so it
// is applied to a copy, and an error changes nothing.
let mut project = self.clone();
project.apply_valid_user_style(style, parts.for_preset())?;
*self = project;
Ok(())
```

`?` returns the error at once, and the song is left as it was. If the
song has no band, one is made for the style's lanes to play in; the
players the style has no lane for play as the band usually does in that
time signature.

### Following the chords

When the band writes its parts, `UserStyle::play_lane` repeats the
style's bars over the song's and gives each pitched note to
`adapt_pitch`, with the harmony and the key at that point. Its comment
is the table above in words:

```rust
/// The note a template note written over C major plays over `harmony`,
/// in `key`. C, E and G are the chord's root, third and fifth, and B is
/// its seventh, or sixth, if it has one. Over a chord of the key, B, D, F
/// and A take the note of the key as many steps above the root: an A over
/// Em in C major is C, not C#, and a B over G is F. Over a chord from
/// outside the key they keep their distance from the root, as in its own
/// major, but a minor chord takes the flat seventh. The other notes are
/// chromatic colours and keep their distance from the root.
```

The chord tones are one `match` on the written note's pitch class (0
is C, 4 E, 7 G, 11 B) and the chord's own seventh, if any:

```rust
let own_seventh = [10, 11, 9].into_iter().find(|&seventh| has(seventh));
let pc = source.midi() % 12;
let target = match (pc, own_seventh) {
    (0, _) => 0,
    (4, _) => intervals.get(1).copied().unwrap_or(4),
    (7, _) => intervals.get(2).copied().unwrap_or(7),
    (11, Some(seventh)) => seventh,
    (11, None) if has(3) => 10,
    _ => pc,
};
```

`intervals` are the chord's notes in semitones above its root, so
`intervals.get(1)` is its second note, the third or a suspended note,
and `intervals.get(2)` its fifth. `own_seventh` looks for a minor
seventh (10), a major seventh (11) or a sixth (9), the note a 6 chord or
a diminished seventh has in that place. Then D, F, A and a B with no
seventh to take are counted in steps instead:

```rust
let steps = match (pc, own_seventh) {
    (2, _) => 1,
    (5, _) => 3,
    (9, _) => 5,
    (11, None) => 6,
    _ => return Some(note),
};
```

If the key has seven notes and holds every note of the chord, the note
moves to the key's note that many steps above the root. Otherwise it
stays as the first `match` placed it.

### Saving without losing the old file

Saving writes a temporary file next to the real one and renames it:

```rust
let temporary = path.with_extension(format!("{EXTENSION}.tmp"));
```

A rename replaces a file in one step, so a crash or a full disk while
writing leaves the old style whole, never half a file.

## Testing it

In `library/user_style.rs`, `template_notes_stay_in_the_key` checks the
table above note by note: a B over Am is G and over D7 C, an A over Em
is C, a B over Fm is E♭ and over B♭ A. `every_builtin_can_be_copied_edited_and_saved`
and `every_band_style_can_be_copied_with_all_its_players` copy every
built-in style and band, validate the copy and read it back from its
text. `a_copy_is_named_and_described_in_the_apps_language` checks
"Bossa nova copy" and "Bossa nova (copia)", and
`user_parts_follow_chords_and_survive_a_project_round_trip` applies a
two-note piano part over Dm and G7, checks the thirds it plays, saves
the project (format 22), opens it again and changes the chords.

In the app, `style_window_previews_and_saves_the_editable_pattern`,
`discarding_a_draft_leaves_nothing_of_it_to_undo` and
`style_confirmations_block_draft_shortcuts_and_escape_keeps_edits`
drive the editor's window;
`saved_styles_reload_alphabetically_with_their_instruments_and_patterns`
reads a styles folder back; `edit_style_copies_the_whole_band_of_a_new_song`
checks that **Edit style…** on a new song brings piano, bass, drums and
guitar; and `saving_a_style_elsewhere_keeps_the_styles_folder` the
notice.
