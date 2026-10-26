# A third bug hunt: a slow file, a jumping pattern and names cut short

Files: [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-core/src/diff.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)
and [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

After the [second bug hunt](bug-hunt.md) the app gained whole orchestral
pieces (Boléro, The Rite of Spring, Petrushka), a playback trace, track
locks and a style per section. Each new piece of work was tested on its
own; this hunt asked again what breaks when they are used **together**,
the way you use the app. Four bugs came out of it, one commit and one
test each. One of them made a saved Boléro take a minute and a half to
open; its fix is a single reordering of two lines, but finding *why* is
a lesson in how a program can be slow without doing anything wrong.

## How the hunt was done

The release build ran on a virtual screen (`Xvfb`), driven with
`xdotool`, as in the earlier hunts. This time the questions were about
the whole workflow rather than one part:

- open a song demo, study it on the chord sheet, try a Library style on
  it, save it, open it again;
- does each window still show what it showed before, after each step?
- is everything the app writes on screen readable at the usual zoom?

Every bug was **seen** first, in the running app or in a test on the
code as it was, and every fix came with a test that fails without it.

## Opening a long song took a minute and a half

Save Boléro as a `.lineup` file, open it again, and the window froze:
85 seconds before it answered. Petrushka's "Shrovetide Fair in the
Evening" took 29 seconds, The Rite's "Augurs of Spring" 4.5. Saving the
same songs took a hundredth of a second, so the work was not in the
size of the file as such.

### Measuring before guessing

The first thing to do with a slow program is to find out **where** the
time goes, not to guess. A profiler (on Linux, `perf`) stops the
program thousands of times a second and notes which function it is in.
Nearly every sample was in one function of the RON library, the one
that reads `.lineup` files, and inside it in `str::find`.

Then a second measurement: reading the same text straight into the
project's types took **0.02 seconds**. So RON was fast at reading the
file, and slow at something else the app asked of it.

### What the app asked

A project file starts with its format number:

```text
(format: 24, tempo: 110.0, channels: [ ... ], patterns: [ ... ], ...)
```

Before reading a file, the app read that number alone, so it could say
"this file was made by a newer version" instead of a confusing error. It
did that with a struct that has only that field:

```rust
#[derive(Deserialize)]
struct Header {
    format: u32,
}

let header: Header = ron::from_str(text)?;
```

serde, the library that turns text into Rust values, lets a struct
leave out fields: the reader **skips** every field the struct does not
name. That is how one struct with one field can read the start of any
project file, even a future one. But to skip a value, the reader has to
read it anyway, to know where it ends, and it has to do that without
knowing its type (`Header` does not say what `patterns` is). For a
number, RON then asks: is this a whole number or a decimal one? Here is
the question, from RON's source:

```rust
let raw_float_len = self.next_chars_while_from_len(skip, is_float_char);
// Trim at ".." to avoid treating range operators as float chars
let valid_float_len = self.src()[skip..]
    .find("..")
    .map(|i| i.min(raw_float_len))
    .map_or(raw_float_len, |i| i.min(raw_float_len));
```

`raw_float_len` is how many characters after this point could belong to
a decimal number: a few. The next line looks for `..` to stop before it,
but it searches `self.src()[skip..]`, **all the text from here to the
end of the file**, not just those few characters. A project file has no
`..` in it, so every search reads to the end and finds nothing.

### Why that grows so fast

A long song is a list of notes, and a note is mostly numbers: step,
length, pitch, velocity. Boléro's file has over two megabytes of text
and well over a hundred thousand numbers. For each number the search reads
the rest of the file: the first number reads two megabytes, one in the
middle reads one, the last reads almost nothing. On average, half the
file for every number.

If a file has *n* numbers, the work is about *n* × *n* / 2. That is
called **quadratic** time, and it is why the times above are so
uneven: a song twice as long takes **four** times as long to open, ten
times as long takes a hundred times. Small files hid it (Carioca, 2.2
seconds, felt like a slow disk), and only the longest pieces made it
obvious.

Notice that no single line is wrong. `find` is the right tool, it just
looks at more text than it needs, and the cost comes from calling it
once for every number. When a program is slow in proportion to the
*square* of its input, look for a loop that, inside, does something
proportional to the whole input.

### The fix: read the whole file first

When RON reads into the real project types, it knows that a `tempo` is
an `f32` and a `step` a `u32`, so it never has to guess, and never runs
that search. So `from_text` now reads the whole project first, and looks
at the format number afterwards:

```rust
pub fn from_text(text: &str) -> Result<Project, FileError> {
    let file: ProjectFile = match ron::from_str(text) {
        Ok(file) => file,
        Err(err) => {
            return Err(match format_number(text) {
                Some(format) if format > FORMAT_VERSION => FileError::NewerFormat(format),
                _ => err.into(),
            });
        }
    };
    if file.format > FORMAT_VERSION {
        return Err(FileError::NewerFormat(file.format));
    }
    Project::try_from(file)
}
```

There are two ways a file from a newer version can show up:

- it **does not read**, because it has a field or a name this version
  does not know. Then the error alone would be confusing ("unexpected
  field `sparkles`"), so the format number is read by hand, and if it
  is newer, that is the error the app shows;
- it **reads**, but its format number is newer, so it may mean
  something else. The `if` after the `match` catches that.

The hand-made reader looks only at the start of the text:

```rust
fn format_number(text: &str) -> Option<u32> {
    let rest = text.trim_start().strip_prefix('(')?.trim_start();
    let rest = rest.strip_prefix("format")?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let digits = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..digits].parse().ok()
}
```

`strip_prefix` returns the text after a prefix, or `None` if the text
does not start with it, and the `?` after each one returns that `None`
from the whole function at once: "if any of these is missing, this is
not a project file's start". Each `rest` is a slice of the same text,
so nothing is copied. Here `find` *is* bounded: it stops at the first
character that is not a digit, two or three characters in.

Saved Boléro now opens in under a second. A test reads it back and
fails if that takes ten seconds, and another reads the format number
from a few starts of files, good and bad. A side effect: the test that
saves and opens again every demo spent almost three minutes in that
header, and all of `lineup-core`'s tests now take 14 seconds in an
optimized build, so the checks on every pull request are faster too.

## Trying a style moved you to another pattern

Open El Choclo, select its "Pickup" pattern in the pattern box, and
apply a style from the Library in song mode (Tango nuevo, say). The
band plays the song, and the pattern box now says **"Ending"**. Every
time.

### How the app hears about changes

The app keeps two copies of the song: the one you edit, in the window's
thread, and the one that plays, in the audio thread. After each edit,
the app compares the project before and after the edit and sends the
audio thread only the differences (`diff.rs`): "insert a pattern at 3",
"remove channel 5", "this note changed".

To compare two lists of patterns, it lines them up with an
`Alignment`: the patterns that are the same at the start of both lists,
the same at the end, and in between what was removed and what was
inserted. Adding a pattern in the middle gives "same, same, *new*, same,
same".

When it sees an inserted pattern, `sync_audio` (in `app/edit.rs`)
selects it, because a pattern you just added, cloned or brought back
with Undo is the one you want to see:

```rust
Change::InsertPattern(index) => {
    // ...
    if bands {
        // A part the band wrote: the pattern you had stays selected.
        if index <= self.pattern {
            self.pattern += 1;
        }
    } else {
        self.pattern = index;
        self.rewind_for_new_pattern();
    }
```

### Why every pattern looked new

The band does not only add patterns; it adds **channels** (piano, bass,
drums...). A pattern holds a row of notes for every channel, so adding
three channels gives *every* pattern three new rows. The `Alignment`
asked "are these two patterns equal?", with `==`, and no pattern was
equal to itself any more. So the diff said: every pattern was removed,
and every pattern was inserted again. `sync_audio` selected each song
pattern as it was "inserted", and the last one, "Ending", won. The audio
thread was also sent the whole song again, which played right but did
much more work than needed.

### The fix: say what "the same pattern" means

Equal and *the same* are two different questions. A pattern whose notes
changed is still the same pattern, as a person whose hair changed is
still the same person. `Alignment::new` now takes the question as an
argument:

```rust
fn new<T>(before: &[T], after: &[T], same: impl Fn(&T, &T) -> bool) -> Self {
    // ...
    let start = before
        .iter()
        .zip(after)
        .take_while(|(old, new)| same(old, new))
        .count();
```

`impl Fn(&T, &T) -> bool` means "any function or closure that takes two
`&T` and answers yes or no". Before, the type had to be `T: PartialEq`
so that `==` could be used; now `T` needs nothing, and the caller
decides. Patterns are lined up by what makes them *that* pattern:

```rust
fn same_pattern(old: &Pattern, new: &Pattern) -> bool {
    old.name == new.name && old.color == new.color && old.player == new.player
}

let patterns = Alignment::new(before.patterns(), after.patterns(), same_pattern);
let channels = Alignment::new(before.channels(), after.channels(), |old, new| old == new);
```

Note the two kinds of argument. `same_pattern` is the **name of a
function**: in Rust a function is a value, and it can be passed where
an `impl Fn` is expected. `|old, new| old == new` is a **closure**, a
function written in place; Rust works out the types of `old` and `new`
from `Alignment::new`'s signature. Channels still use `==`: a channel
is its sound settings, and a channel whose settings changed is sent
again anyway.

A pattern lined up as "the same" but with other notes is not lost: the
rest of the diff compares the notes of every pattern it lined up and
sends the ones that changed. So this choice cannot make the song play
wrong; it only decides what counts as *new*.

Two tests check it: one in `diff.rs`, where a new channel and a new
pattern give exactly one "insert pattern", and one in the app's tests,
which applies Tango nuevo to El Choclo with "Pickup" selected and
checks that "Pickup" is still selected afterwards.

## Following the playhead hid bar 1

With **Follow playhead** on, the playlist scrolls to keep the playhead
in view. When a song plays to its end and starts again, or a loop goes
back to its start, the playhead jumps left and the view follows it. But
it stopped with bar 1 half hidden, under the track names, and the
playhead with it.

The track names on the left are **sticky**: they are drawn over the
bars, at the left edge of the view, and stay there while the bars
scroll under them. To the scroll area they are not there at all. When
the playhead moved, `follow_x` asked it to reveal a small rectangle
around the playhead:

```rust
let margin = (view.width() * 0.1).min(60.0);
ui.scroll_to_rect(
    Rect::from_center_size(pos2(x, view.center().y), vec2(margin * 2.0, 1.0)),
    None,
);
```

`scroll_to_rect` scrolls as little as it can so the rectangle is
inside the view. Going left, that leaves the rectangle's left edge at
the view's left edge: the playhead 60 points from it. The names are 116
points wide, so the playhead was under them. The Home key already
allowed for the names; following did not.

The fix tells `follow_x` how wide the fixed part is, and makes the
rectangle reach that much further left:

```rust
pub fn follow_x(ui: &Ui, step: Option<usize>, x: f32, view: Rect, covered: f32) {
    // ...
    let y = view.center().y;
    ui.scroll_to_rect(
        Rect::from_min_max(
            pos2(x - margin - covered, y - 0.5),
            pos2(x + margin, y + 0.5),
        ),
        None,
    );
```

The playlist passes `HEADER_WIDTH` (its names), the piano roll
`KEY_WIDTH` (its keyboard), which had the same problem. Going right,
nothing changes: the right edge of the rectangle is where it was. A test
runs a scroll area 800 points wide with 116 points of names, plays far to
the right, jumps back, and checks that the playhead ends up right of the
names.

## The piano roll printed pieces of names

Open El Choclo's "A" pattern in the piano roll. Under the notes, the
chord lane names the chord of each stretch of steps. At the usual zoom
it showed "n", "7/", "/A7"; the track's row, which names single notes,
showed "E♭E" and "G#B", two names run together.

A name was drawn centred in its block and **clipped** to it: whatever
fell outside the block was cut off. "E7/B" in a block too narrow
showed only its middle, "7/", which is not a name at all, and "/A7"
looks like another chord. For someone learning harmony from the screen,
a wrong name is worse than none.

### Measure, then draw

egui draws text in two steps. First it **lays it out**: it picks the
glyph of each character in the font, and works out where each goes and
how much room the whole takes. The result is a *galley* (the name comes
from printing: a tray of type set in lines, ready to print). Then it
paints the galley. `painter.text(...)` does both at once; to decide
*before* drawing, the two steps are taken apart:

```rust
let name = painter.layout_no_wrap(label, FontId::proportional(11.0), LANE_TEXT);
if name.size().x <= chord_rect.width() - 2.0 {
    // A long chord's name stays in view while it scrolls.
    let shown = chord_rect.intersect(cells);
    painter.with_clip_rect(chord_rect).galley(
        shown.center() - name.size() / 2.0,
        name,
        LANE_TEXT,
    );
}
```

`layout_no_wrap` lays the text out on one line (no wrapping into a
second line), and `name.size()` is how wide and tall it is. If it fits
in the block, with a point to spare on each side, the same galley is
painted; nothing is laid out twice. `galley` takes the top-left corner
of the text, so the centre is moved back by half the size. If it does
not fit, nothing is drawn; pointing at the block still names the chord
in full in the line under the toolbar ("Step 12: E7/B").

The test opens "A" in the piano roll, finds every piece of text painted
in the lane's colour, and checks that each is no wider than its block.
Before the fix it failed on "D#": 14.9 points wide, in a block of 6.2.

## What shipped after this hunt

The same hunt produced a list of ideas for the workflow and the screen:
inserting and deleting bars, Open recent, a recovery copy, a one-row
toolbar, a choice of numerals in minor keys, and more. The main ones
have since shipped. Bars can be
[inserted and deleted](insert-delete-bars.md) with the whole song moving
along; **File > Open recent** keeps the last ten projects; a crash leaves
a [recovery copy](recovery.md) that is offered at the next start; [the
workflow chapter](workflow.md) covers the one-row toolbar and the
shortcut list; and [numerals in minor keys](minor-numerals.md) are
written as your book writes them.

The RON search could also be fixed where it is, by looking for `..`
only among the characters that could be part of the number. That is a
change to someone else's library, so the app does not depend on it: it
simply no longer asks RON to skip fields.

## Exercises

1. Saved, "The Shrovetide Fair" is 780 KB of text with about 43,000
   numbers in it. If each number's search reads, on average, half the
   file, how many bytes are read in total? At about 2 GB a second, how
   long does that take? Compare with the 10 seconds measured.
2. Fix RON's look-ahead: rewrite the `find("..")` line so it searches
   only the first `raw_float_len` characters. (Hint: slice
   `self.src()[skip..skip + raw_float_len]`, and think about what
   happens if the slice ends in the middle of a character.)
3. `format_number` returns `None` for `(tempo: 120.0, format: 25)`. Why
   is it fine that it only reads the start of the file? (Look at the
   order of the fields in `ProjectFile`, and at `to_text`.)
4. Write `same_channel(old: &Channel, new: &Channel) -> bool` that lines
   channels up by name only, and pass it to `Alignment::new`. Then find
   a case where it gives the wrong answer: what happens with two
   channels called "Piano"?
5. The piano roll leaves out a chord name that does not fit. Make it
   try a shorter name first: for a slash chord, the chord without its
   bass ("E7" for "E7/B"), and then the root alone ("E"). Lay out each
   with `layout_no_wrap` and draw the first one that fits.
