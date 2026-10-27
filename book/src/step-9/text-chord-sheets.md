# Chord sheets as text: to paste and share

Files: [`crates/lineup-core/src/chord_sheet/text.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/text.rs),
and in the app
[`crates/lineup-app/src/app/chord_text.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/chord_text.rs)
(File menu) and
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
(copy and paste)

Musicians send each other chord charts as plain text all the time: in an
email before a rehearsal, in a forum post about a song, in a notes app on
the phone. Nobody agreed on a format, but most charts look about the
same, because they copy the way a chord chart is printed: bar lines
between the bars, the chords of each bar between them, and the name of
each section over its bars.

```text
Title: Autumn Song
Key: C major
Time: 4/4
Tempo: 120
Style: Bossa nova

[Verse ×2]
| C      | Am7 | Dm7 G7 | C  |
| F      | %   | G7     | C  |

[Chorus]
| F      | G   | Em7 A7 | Dm |
```

This is item 10 of the plan. Ferrous Lineup reads such a chart into a
song with a band, writes the chord sheet of any song the same way, and
copies and pastes bars of the chord sheet as text. Nothing new is stored
in the song, so the file format stays as it was.

## What you see

![The File menu with Import chord sheet… under Import MIDI file… and Export chord sheet… under Export pattern as MIDI…](../images/step-9-text-file-menu.png)

**File > Import chord sheet…** opens a `.txt` file (or drop one on the
window) and makes a new song from it: the band plays the chords in the
chart's key, time signature, tempo and style, the sections are on the
playlist, and the chord sheet opens so you can see what was read. If
the song open has unsaved changes, you are asked first, as for **Open**.
The new song is called by the chart's title (or the file's name) until
you save it, and **Save as…** suggests that name. A song imported from
a MIDI file now keeps its file's name the same way.

![Autumn Leaves imported from text: the chord sheet shows the band's style (Swing), the key G major, 132 BPM, the section A ×2 over bars 1 to 8 and the section B from bar 9, every chord with its numeral; the status bar says "Imported autumn.txt: 24 bars"](../images/step-9-text-chart-imported.png)

If something in the chart can't be read, nothing is imported, and the
status bar says where:

> Could not import bad.txt: line 4: bar 3: "Qx7" is not a chord (Am7,
> G/B), a set (3-11 T2, {0,4,7}), a micromode (Menor 3), a row form (P0,
> RI7) or N.C.

**File > Export chord sheet…** writes the song's chord sheet to a text
file, in the format above, with the bars lined up in columns so the
text reads like the window.

In the chord sheet window itself:

- **Ctrl+C** copies the bars selected (click one, Shift+click another to
  select several) as text, "| Am7 | D7 | Gmaj7 | Cmaj7 |". With every
  bar selected, or none, it copies the whole sheet with its settings
  and sections, as Export writes it.
- **Ctrl+A** selects every bar.
- **Ctrl+V** writes the text on the clipboard from the selected bar on,
  and selects the bars it wrote. One line without any `|` is read as
  one bar, as if you had typed it there, so "Dm7 G7" copied from a message
  goes straight into a bar.

A paste is one change of the sheet, so **Ctrl+Z** takes it back in one
go. The message above the bars says what was copied or pasted, and
goes away when you click another bar.

![Twenty-four bars pasted into a new song in C major: the same chord names, now with their numerals in C (Am7 vi7, D7 V7/V, F#m7b5 #ivø7, B7 V7/iii). [Secondary dominants](chord-functions.md#secondary-dominants-v-of-v) such as V7/V are named after the chord they lead to](../images/step-9-text-pasted.png)

Pasting takes only the chords. The song keeps its own key, time
signature, tempo, band and sections; the pasted Autumn Leaves above is
in G major, but the song it went into is in C major, so its numerals are
read in C. To bring a whole chart with its settings, import it instead.

## The format

Everything is optional except the bars.

| Line | Means |
|------|-------|
| `Title: Autumn Leaves` | The song's name. A first line that is nothing else is the title too, as at the top of a printed chart. |
| `Key: G major` | The key. `G`, `Em`, `E minor`, `A Dorian`, `C harmonic minor` and the other scales of the key menu (not rows or sets, which are chosen in the app). A key with two names keeps the one written: `Gb major` stays G♭ major, `F# major` stays F# major. |
| `Time: 3/4` | The time signature, any the app takes: `6/8`, `5/4`, `2+2+3/8`. |
| `Tempo: 132` | Quarter notes per minute; `132 BPM` reads too. |
| `Style: Bossa nova` | The band style, by its name in the Library. |
| `[Verse]` | A section starts with the next bar. `[Chorus ×2]`, `[Chorus x2]` or `[Chorus] x2` plays it twice. |
| `\| C \| Am7 \| Dm7 G7 \|` | Bars. A line can hold any number of them. |
| `# Ask Ana about the bridge` | A note for people, skipped. |

Inside a bar goes exactly what you type in a bar of the chord sheet
([Writing in a bar](chord-sheet.md#writing-in-a-bar)): chords, Roman
numerals, sets, micromodes, row forms or `N.C.`, spread over the beats
by the same rule, and `/` for a beat that holds the harmony before it.
Two more signs come from printed charts:

- **`%`** is the **simile sign**, printed as a slash between two dots
  (𝄎). It means "play the bar before again". An empty bar, `| |`, means
  the same.
- **`||`**, **`|:`** and **`:|`** are a double bar line and repeat signs.
  They are read as plain bar lines; repeats are written with `×2` on a
  section, which is how the playlist stores them.

When there is no `Style:` line, the band plays a style in the chart's
time signature, so a chart in 3/4 gets the jazz waltz, and with no time
signature either, the style a new song starts with. When no style is in
the chart's time signature (5/4, say), the band plays the style a new
song starts with, in the chart's time, and the status bar says so:
"The band plays Pop beat, a style in 4/4, in 5/4". A `Key:` or `Time:`
line between bars changes the key or the time signature from the next
bar on, as a key marker or a time-signature marker on the playlist
does.

Roman numerals are read in the chart's key. This chart

```text
Key: F major
| I | vi | ii7 V7 | I |
```

becomes F, Dm, Gm7 C7, F, the most common turnaround there is, in F.
The same text with `Key: D major` would give D, Bm, Em7 A7, D: one
progression in any key, which is why musicians who play by ear like
numerals.

## Reading the text

`text::read` turns the text into a `TextSheet`, and only checks the
layout. It can't read the bars yet: how many beats a bar has depends on
the time signature at that point, which only the song knows once the
`Time:` lines before it are in place.

```rust
pub struct TextSheet {
    pub title: Option<String>,
    pub key: Option<Scale>,
    pub time: Option<TimeSignature>,
    pub tempo: Option<f32>,
    pub style: Option<(String, usize)>,
    pub items: Vec<Item>,
}

pub enum Item {
    Bar { text: String, line: usize },
    Section { name: String, repeats: usize, line: usize },
    Key(Scale),
    Time(TimeSignature),
}
```

`Item` is an enum whose variants hold different things: two of them
have named fields, like a struct, and two hold one value, like a tuple.
A `match` on an `Item` has to say what it does with each of the four,
so when a fifth kind of line is added, the compiler lists every place
that must learn about it.

Bars and sections keep the number of their line, so a problem found
later, when the bar is read, can still name it. The
lines are numbered with `enumerate`, which counts from 0, so the
number is `index + 1`:

```rust
for (index, line) in text.lines().enumerate() {
    let number = index + 1;
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        continue;
    }
    ...
}
```

The error is a small struct with the line and the message, and
`impl fmt::Display` says how it prints, "line 4: …". Every function
that can fail returns `Result<_, TextError>`, and `?` passes the error
up at once. `map_err` turns another error into this one on the way:

```rust
let key = read_key(value).map_err(|message| error(number, message))?;
```

### Settings, sections and bars

A line is a setting when it has a colon and the word before it is one
of the five settings, whatever its case. `split_once(':')` splits
"Key: G major" at the first colon into `("Key", " G major")`, and
returns `None` when there is no colon:

```rust
fn setting(line: &str) -> Option<(String, &str)> {
    let (name, value) = line.split_once(':')?;
    let name = name.trim().to_lowercase();
    ["title", "key", "time", "tempo", "style"]
        .contains(&name.as_str())
        .then_some((name, value))
}
```

`bool::then_some(value)` is `Some(value)` when the bool is true and
`None` otherwise, a short way to end a function that keeps or drops
its answer.

A section heading can put its count inside the brackets or after them.
`read_times` reads "×2", "x2" or "X 2":

```rust
fn read_times(text: &str) -> Option<Option<usize>> {
    let number = text
        .strip_prefix(['×', 'x', 'X'])?
        .trim()
        .parse::<usize>()
        .ok()?;
    Some((1..=MAX_REPEATS).contains(&number).then_some(number))
}
```

`strip_prefix` takes an array of characters and strips whichever one
the text starts with. The answer is an option inside an option, because
there are three cases to tell apart: `None` is "this is not a count at
all" (the heading goes on with other words), `Some(None)` is "a count,
but too many times" (an error), and `Some(Some(2))` is twice. Where the
heading is read, two `let` conditions are chained with `&&`, which Rust
allows since its 2024 edition:

```rust
if repeats.is_none()
    && let Some((first, last)) = name.rsplit_once(char::is_whitespace)
    && let Some(times) = read_times(last)
{
    repeats = Some(times);
    name = first.trim().to_string();
}
```

The block runs only if every condition holds, and the names bound
along the way (`first`, `last`, `times`) can be used in the later
conditions and in the block. Without the chain this would be three
nested `if`s.

Bar lines are made plain first, then the line is split at each `|`.
The pieces before the first sign and after the last are empty when the
line starts and ends with a bar line, so they are dropped; when they are
not empty, they are bars, so "C | G" is two bars.

## Making the song

`text::import` starts from `Project::with_band()`, the song **File >
New** makes, and sets what the chart says, in the order the band needs
it: first the style (which can re-meter the empty song to its own time
signature), then the chart's time signature, key and tempo on top.
Then it walks the items once, keeping the step where the next bar
starts:

```rust
Item::Bar { text, line } => {
    bars.push((step, text, *line));
    step += time.steps_per_bar();
}
Item::Time(new) => {
    time = *new;
    project.set_time_marker(TimeMarker::new(step, time));
}
Item::Key(key) => project.set_key_marker(KeyMarker::new(step, *key)),
Item::Section { name, repeats, .. } => sections.push((name.clone(), *repeats, step)),
```

A section ends where the next one starts, or at the end of the song.
Once every marker is in place, each bar is read with `read_bar`, which
gets the bar (to know its beats) and the key at its start (to read
numerals). All the chords go into the song in one `set_chords`, and
`follow_chord_sheet` tells the band to write its parts.

`text::paste` does the same reading into an existing song, from a bar
on, and returns the markers instead of writing them. The window turns
them into one `SheetAction::SetChords`, the same action as typing in a
bar, so undo, the band and the playlist need nothing new.

## Writing the text

`text::write` goes through the bars of the sheet with `write_bar` (the
function that fills a bar's text field when you click it) and collects
lines: words ("[Verse ×2]", "Key: D minor") or a row of up to four bars.
A section, a change of key or of time signature starts a new row, and
an empty bar is written `%`.

Then `layout` lines the bars up. First it finds the widest bar in each
of the four columns, then pads every bar to its column's width:

```rust
for (width, bar) in widths.iter_mut().zip(bars) {
    *width = (*width).max(bar.chars().count());
}
...
.map(|(bar, width)| format!("{bar:<width$}"))
```

`{bar:<width$}` writes `bar` left-aligned (`<`) in a field as wide as
the variable `width` (the `$` says the width comes from a variable).
The widths are counted with `chars().count()`, not `len()`: `len()`
counts the bytes of the UTF-8 text, and a letter outside plain ASCII
(ø, ×, ♭) takes two or three bytes but one place on the screen.

A key that text can't say, a twelve-tone row or a pitch-class set, is
written as a note for people, "# Key: … (chosen in the app)", so the
chart still reads and nobody is misled.

## Copy and paste in the window

egui turns Ctrl+C and Ctrl+V into events of their own,
`egui::Event::Copy` and `egui::Event::Paste(text)`, with the text
already taken from the system clipboard. That is the same on Linux,
macOS (where the keys are Cmd+C and Cmd+V) and Windows. The sheet looks
for them among the frame's events while it has the keyboard and no bar
is being typed in; while a bar is being typed in, they go to the text
field as usual.

```rust
for event in &input.events {
    match event {
        egui::Event::Copy => copy = true,
        egui::Event::Paste(text) => paste = Some(text.clone()),
        _ => {}
    }
}
```

To copy, `ctx.copy_text(text)` puts the text on the clipboard at the
end of the frame.

## Two old mistakes, found by a round trip

The best test of a pair of functions like `write` and `read` is a
**round trip**: write something, read it back, and check you got the
same thing. The test `every_song_demo_comes_back_from_its_text` does
that for every song demo: each one's chord sheet is written as text,
imported, and its chords, keys and time signatures are compared with
the demo's.

It found two mistakes that were already in the chord sheet window
([The chord sheet](chord-sheet.md)), both in rare bars:

1. **"C G7 C /"**, three chords on the first three beats and the last
   beat holding the third. `write_bar` dropped slashes at the end of a
   bar, since "C / G7 /" says the same as "C / G7". But "C G7 C" with
   no slash at all means three chords spread over the bar, on beats 1,
   3 and 4, so clicking such a bar and leaving it moved two chords. A
   trailing slash is now kept when it is the last one in the bar.
2. **"A E / B"**, A, then E held for two beats, then B. Looking for
   names of several words ([Names of several
   words](chord-sheet.md#names-of-several-words)), `read_bar` tried
   "E / B" and found the slash chord E/B. A `/` standing alone is a
   beat, so a name now never runs across one; "E/B" written together
   is still the slash chord.

A few demos have a chord between two beats (Happy Birthday, for
instance), which a chord sheet can't write, so the test compares
the chords placed on the beats, and checks that fewer than one demo in
ten has such a chord.

## Exercises

1. **Chords on the off-beat.** Some text charts write a push, a chord
   that comes half a beat early, with a sign such as `>` or `^`. Which
   function would read it, and what would `ChordMarker::start` need to
   hold?
2. **The title on export.** A song saved as `leaves.lineup` is exported
   with "Title: leaves", the file's name. Store the title read from a
   chart in the song itself, so it comes back on export. Why does that
   need a new file format number, and what would an old file read as?
3. **The key on paste.** When the text pasted has a `Key:` line that
   differs from the song's key, the chords keep their names. Offer to
   transpose them into the song's key instead. Which function in
   `lineup_core` already moves chords from one key to another?
4. **More columns.** Charts on a phone read better with two bars to a
   line. Make the number of bars on a line a parameter of `write`. What
   in `layout` assumes four?
5. **Other formats.** ChordPro writes chords inside the lyrics, as in
   `[C]Autumn [Am7]leaves`. Sketch a reader for its `{title: …}` and
   `{key: …}` lines. Could it return a `TextSheet`?
