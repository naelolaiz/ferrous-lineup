# The chord sheet fills the window, with lines that fit the names

Files: [`crates/lineup-app/src/chord_sheet/layout.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/layout.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs),
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)
and [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

[The chord sheet](chord-sheet.md) put four bars on every line, in a
window over the playlist. Four bars is what a printed chart usually
has, but the sheet now writes much longer names than "C" and "G7":
polychords such as "Eb7|Fb", sets such as "3-11 T7I", micromodes such
as "Acorde menor-mayor T7I", each with a numeral or its notes under it.
With four of those in a bar and four bars to a line, the names didn't
fit, and the sheet made them smaller until they did. On a wide screen it
was the other way round: a short song in big empty boxes, and a window
that hid the playlist.

Two changes, both at the top of the sheet, next to **Show**:

- **Bars per line.** By default, **Fit the names**: as many bars as fit
  on a line with every name and every word under it at its full size,
  never shrunk. The menu says how many that is now, as in **Fit (4)**.
  Or pick a number from 1 to 16; then the line holds that many, and
  names that don't fit are written smaller.
- **⛶ Fill the window.** The sheet takes the playlist's place in the
  main window, beside the patterns and the Library. With the whole
  width, a line holds more bars. **Back to the playlist** puts it back
  in its window. The same switch is in the **View** menu as **Chord
  sheet in the main window**.

Both are remembered from one run of the app to the next. And **F5**
still opens and closes the sheet: closed and opened again, it comes back
where it was, in the main window or in its own.

![Embraceable You on the chord sheet, filling the main window beside the patterns: four bars to a line, where its window had room for two, and every name and numeral at its full size.](../images/step-9-chord-sheet-fills-window.png)

## Trying it

1. Open a song from **File > Demos**. Songs open on the chord sheet.
2. Make the sheet's window narrower and wider by its corner. With
   **Fit the names**, the lines hold 1, 2, 4, 8, 12 or 16 bars, and the
   menu's **Fit (N)** follows. The names keep their size.
3. Click **⛶ Fill the window**. The playlist makes room for the sheet,
   and the lines get longer. The channel rack, the piano roll and the
   mixer still open over it with F6, F7 and F9.
4. Pick **Bars per line > 16**. The names that don't fit in a
   sixteenth of the width get smaller, all the names of a bar at the
   same size.
5. Pick **Fit the names** again, and **Back to the playlist**.

![The Bars per line menu: Fit the names, or a number of bars from 1 to 16 (the list scrolls to 15 and 16).](../images/step-9-chord-sheet-bars-per-line.png)

## Why 1, 2, 4, 8, 12 or 16

Fitting could have put any number of bars on a line: five, seven,
eleven. A printed chart never does, and for a reason. Most songs are
made of **phrases of four bars**, and a chart puts a phrase's first bar
at the start of a line, so the eye finds bar 1, 5, 9 and 13 in the same
column. With five bars to a line, the second phrase would start in the
second column of the second line, the third in the third: the song's
shape would be lost.

So the counts it fits are the ones that keep phrases together: 1 and
2 go evenly into a phrase, and 4, 8, 12 and 16 are made of whole
phrases. Twelve is a twelve-bar blues on one line.

```rust
/// The counts of bars a line holds when they fit, from the most: each
/// divides a four-bar phrase or is made of them.
const FITS: [usize; 6] = [16, 12, 8, 4, 2, 1];

/// No bar is made narrower than this to fit more on a line, however
/// short its names.
const NARROWEST_BAR: f32 = 110.0;
```

`NARROWEST_BAR` is there for songs of short names. A sheet of "C" and
"G" would otherwise fit sixteen bars in a window that shows four
comfortably, and a bar too narrow to click or to type in.

## The choice, as a type

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Columns {
    /// As many as fit with every name at its full size.
    #[default]
    Fit,
    /// This many, from 1 to [`MOST_BARS`]: names that don't fit shrink.
    Bars(usize),
}
```

`#[default]` on a variant tells `#[derive(Default)]` which variant
`Columns::default()` is. It works on enums since Rust 1.62; before, the
`impl Default` had to be written by hand.

The count comes from one method:

```rust
impl Columns {
    /// How many bars a line holds when it is `width` wide and the widest
    /// bar needs `needed` to show its names in full.
    pub fn count(self, width: f32, needed: f32) -> usize {
        match self {
            Columns::Bars(bars) => bars.clamp(1, MOST_BARS),
            Columns::Fit => {
                let bar = needed.max(NARROWEST_BAR);
                FITS.into_iter()
                    .find(|&bars| width / bars as f32 >= bar)
                    .unwrap_or(1)
            }
        }
    }
}
```

`FITS` goes from the most bars to the fewest, so `find` stops at the
largest count whose bars are still wide enough. If even one bar is too
narrow (a very long name in a very narrow window), it is still one bar
to a line, and that one bar's names are written smaller.

`FITS.into_iter()` iterates over the array **by value**, so its items
are `usize`s. `find` hands each item to its closure by reference, as a
`&usize`, so it can test the item without taking it, and give it back
if it is the one. The pattern `&bars` in the closure's parameter takes
the `usize` back out of the reference.

`count` is a pure function of two numbers, so its test needs no window
at all:

```rust
// Short names: the narrowest bar decides.
assert_eq!(Columns::Fit.count(760.0, 60.0), 4);
assert_eq!(Columns::Fit.count(1300.0, 60.0), 8);
// Long names take more room, and whole phrases still start lines:
// room for three bars makes lines of two.
assert_eq!(Columns::Fit.count(760.0, 240.0), 2);
```

At 1300 pixels, twelve bars would be 108 pixels each, under the
narrowest bar; eight are 162. At 760 pixels with names that need 240,
three bars would fit (253 each), but three is not a count that keeps
phrases together, so it is two.

## How wide a bar must be

Before drawing, the sheet asks every bar of the song how wide it must
be, and takes the widest. A bar's names don't share its width equally:
**each name has the part of the bar from its beat to the next name's**,
because that is where the sheet draws it. In a 4/4 bar of "C A7", C
has beats 1 and 2, half the bar, and so does A7. In "C / / A7", C has
three quarters and A7 one.

So a name that is 30 pixels wide and has a quarter of the bar needs a
bar of 120 pixels:

```rust
let share = (end - starts[index]) as f32 / length;
let text = width(&item.text, NAME_SIZE);
let caption = item
    .caption
    .as_deref()
    .map_or(0.0, |caption| width(caption, CAPTION_SIZE));
(text.max(caption) + gap) / share
```

`width` measures the text with the painter's fonts (`layout_no_wrap`),
the same way it will be drawn. The **caption** is the line under the
name: the Roman numeral of a chord, or the notes of a set ("C E G"),
which is often the wider of the two. `as_deref` turns the
`Option<String>` into an `Option<&str>`, and `map_or` gives 0 when there
is no caption. A name followed by another keeps a gap of 4 pixels from
it, and the padding on both sides of the bar and any repeat signs are
added at the end.

The widest of them all is a **fold**:

```rust
.map(|bar| self.needed_width(ui.painter(), project, &meter, &keys, &bar, marks, &heard))
.fold(0.0, f32::max)
```

Why not `.max()`? Because `Iterator::max` needs `Ord`, a total order,
and `f32` doesn't have one: `NaN` is neither smaller nor larger than any
number. `f32::max` is a plain function that takes two `f32`s and returns
the larger (and ignores a `NaN`), so folding with it from `0.0` gives
the widest bar, or 0 for a song with no bars. Passing `f32::max` by name
works because a function is a value that can be called, just as a
closure `|a, b| a.max(b)` would be.

Measuring every bar costs a little on each frame: a song of 200 bars is
200 bars of text measured. egui keeps the galleys it has laid out
recently in a cache, so measuring the same name again is a lookup.

## Names that don't fit, when you pick the number

With a number picked, a bar can be too narrow for its names. They are
then written smaller, by `draw_names`, with three rules:

- **All the names of a bar at one size**, the size the most crowded one
  allows. A bar that mixes big and small names reads as if the small
  ones mattered less.
- A name going on **from an earlier bar**, drawn faintly at the start
  of the bar, only shrinks itself.
- A set or micromode whose name doesn't fit even at the smallest size
  is written by its **class** alone ("Menor 3" for "Menor 3 T8I"), and
  where not even that fits, "…": a name cut short would read as another
  name.

## Taking the playlist's place

The main window has a central panel, where the playlist is drawn. When
the sheet fills the window, that panel draws the sheet instead, still
next to the Patterns and Library list:

```rust
// The chord sheet can take the playlist's place, beside the
// pattern picker and the library.
self.chord_sheet.remember(&ctx);
let sheet_fills = self.chord_sheet.fills_window();
// …
let playlist = egui::CentralPanel::default_margins().show(ui, |ui| {
    if sheet_fills {
        let actions = self.playlist.browser(ui, &self.project, self.pattern, tried);
        let sheet = self.chord_sheet.main(ui, &self.project, playing, typing, /* … */);
        (actions, sheet)
    } else {
        let actions = self.playlist.show(ui, /* … */);
        (actions, Vec::new())
    }
});
```

Both branches return a pair of lists of actions: what the playlist (or
the browser beside the sheet) asks for, and what the sheet asks for.
The `if` is an expression, so both branches must have the same type;
the playlist's branch returns an empty `Vec` for the sheet's actions.
The app then applies both lists the same way, whichever was drawn.

`ChordSheet::main` draws the same contents as the sheet's window:
the same rows of buttons, the same bars, the same keys. Only the frame
around them changes. So everything the sheet does in its window it
does in the main window, without a second copy of the code.

`remember` is called **before** `fills_window`. It reads the choices
saved by the last run, the first time it is called. Called after, the
first frame would draw the playlist, the second the sheet: a flash of
the wrong view every time the app starts.

## Remembering the choices

The two choices are one small struct, saved with the app's state:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    columns: Columns,
    /// Whether the sheet takes the playlist's place in the main window.
    fills: bool,
}
```

`#[serde(default)]` on the struct says: a field missing from what was
saved takes its value from `Choices::default()`. If a later version
adds a third choice, the state saved by this one still loads, with the
third choice at its default, instead of failing and losing the first
two.

`LayoutView::remember` keeps a copy of what it last saved, and writes
only when the choices differ from it:

```rust
match self.saved {
    None => {
        self.choices = ctx
            .data_mut(|data| data.get_persisted(choices_id()))
            .unwrap_or_default();
        self.saved = Some(self.choices);
    }
    Some(saved) if saved != self.choices => {
        let choices = self.choices;
        ctx.data_mut(|data| data.insert_persisted(choices_id(), choices));
        self.saved = Some(choices);
    }
    Some(_) => {}
}
```

The `None` arm runs once, on the first frame. The second arm has a
**match guard**, `if saved != self.choices`: the arm matches only when
the pattern does and the condition holds. Everything else falls to
`Some(_) => {}`, nothing to do, which is almost every frame.

## The arrows still land in the right column

The first chord sheet chapter ended with an exercise: make the number of
bars per line a setting, and check that the up and down arrows still
land in the right column. They do, because they never counted bars.
The sheet keeps every bar it drew as a cell with its **line** and its
**column**, and the down arrow looks for the cell on the next line whose
column is closest to the selected one:

```rust
let in_line = |line: usize, column: usize| {
    cells
        .iter()
        .filter(|cell| cell.line == line)
        .min_by_key(|cell| cell.column.abs_diff(column))
        .map(|cell| cell.bar.number)
};
```

The cells come from what was drawn, so they are right with four bars to
a line, with sixteen, and on a short line at the end of a section.
`min_by_key` works here, unlike `max` above, because the key is a
`usize`, which is `Ord`.

## The tests

`lines_fit_as_many_whole_phrases_as_there_is_room_for`, in `layout.rs`,
checks `count` with the numbers above.

In the app, `lines_hold_as_many_bars_as_show_every_name_in_full` starts
a song whose first bar is "Dm7/C F/C G7/D C/E", four harmonies in one
bar. It checks that the sheet holds fewer than four bars to a line and
that every name is drawn at its full size, 20 points. Then it picks 8:
the menu no longer says "Fit (8)", and "Dm7/C" is smaller. Then **Fit
the names** again brings back the first count and the full size.

`the_sheet_fills_the_main_window_and_goes_back` clicks **⛶ Fill the
window** and checks that the word "Playlist" is gone, "Patterns" is
still there, the lines hold more bars than in the window and the names
are whole. It closes and opens the sheet with F5, restarts the app, and
finds the sheet in the main window each time. **Back to the playlist**
and the View menu item finish it.

## Exercises

1. A song in 3/4 is often charted eight bars to a line, a waltz even
   sixteen. Should `FITS` depend on the time signature? What would you
   pass to `count`?
2. `needed_width` makes the widest bar decide for the whole sheet. Make
   it decide per **section** instead, so a verse of short chords can
   have more bars to a line than a bridge of polychords. What happens
   to the up and down arrows between sections?
3. Write `widest(values: impl Iterator<Item = f32>) -> Option<f32>`
   that returns `None` for no values, with `reduce` instead of `fold`.
   Why is `reduce` a better fit when there is no natural starting value?
