# Quicker to work in: songs open on their chords, one row of buttons, every key in a list

Files: [`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs),
[`crates/lineup-app/src/icons.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/icons.rs),
[`crates/lineup-app/src/app/shortcuts.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/shortcuts.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs)
and [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

The [third bug hunt](bug-hunt-3.md) ended with a list of ideas for the
way the app is used, numbered so they could be picked one by one. This
chapter is about the small ones, each a change of its own on `main`.
None of them adds anything to the music. Each removes a step between
you and the music:

- a song opens on its **chord sheet**, and a beat on the **channel rack**;
- the **toolbar** fits on one row again, with small painted pictures for
  the windows;
- **Help > Keyboard shortcuts** (F1) lists every key the app answers to;
- every window opens **at the size of what it shows**, not at a size
  egui remembered from another day.

## Trying it

1. Open **File > Demos > El Choclo**. The chord sheet is in
   front, and the channel rack is closed. **F6** brings the rack back.
2. Open **File > New beat**. Now the rack is in front and the chord
   sheet is closed.
3. Hold the mouse over the five square buttons in the middle of the
   toolbar. Each says its name, what it shows and its key, from
   **F5** (Chord sheet) to **F9** (Mixer). A button is lit while its
   window is open.
4. Press **F1**. A window lists every key, in sections: the keys that
   work anywhere, the windows, File and Edit, zoom, and then each
   window's own keys.
5. Open the mixer (**F9**), drag its corner to make it much bigger,
   close it and open it again. It comes back as big as its channels.

![The toolbar on one row: the five window buttons and the Count-in and Metronome toggles are pictures.](../images/step-9-workflow-toolbar.png)

## A song opens on its chords

A song demo used to open with the channel rack over the playlist. The
rack shows the steps of one pattern, which is what you edit in a beat.
In a song, what you study is the bars and their chords. So now the app
decides what to show by what was opened:

```rust
/// What a song opens on: the chord sheet in front, since its bars and
/// chords are what you study and write in a song, and the rack out of
/// the way (F6 brings it back).
fn show_chord_sheet(&mut self) {
    self.chord_sheet.open = true;
    self.rack_open = false;
    self.bring_to_front = Some(chord_sheet::window_id());
}

/// What a beat opens on: the rack in front, and no chord sheet.
fn show_beat(&mut self) {
    self.chord_sheet.open = false;
    self.show_rack();
}
```

Song demos, **File > New** (with the chords you type) and an imported
chord sheet call `show_chord_sheet`; **File > New beat** calls
`show_beat`. Opening a saved file leaves the windows as they are, since
you chose them yourself. A demo is a song or a style, an `enum`, and only
the songs open on the sheet:

```rust
if let Demo::Song(_) = demo {
    self.show_chord_sheet();
}
```

`if let` is a `match` with one arm. It is the usual way to say "if it
is this variant", when nothing needs to be done for the others.

## The toolbar on one row

Over the steps of this book the toolbar kept growing: Follow playhead,
Count-in and the Metronome joined it. In a window 1280 points wide it
had started to wrap onto a second row, while a comment in the code still
said it fit on one.

The five window buttons were words, most with an emoji in front
("📝 Chords", "🎹 Piano roll", "🎼 Harmony"), and Follow playhead,
Count-in and Metronome were words too. All eight are now square buttons
30 points wide, each showing a picture. Its name, and for a window what
it shows and its key, appear when the mouse is over it. The window
buttons are in the order of their keys, F5 to F9.

### Pictures painted, not typed

An emoji is a character, drawn by a font. egui uses the fonts it was
given, and some systems have no font with those characters: there the
toolbar showed empty boxes. So the pictures in `icons.rs` are
**painted**, with lines, rectangles and polygons, the same way on every
system.

Each picture is a plain function with the same signature, and Rust can
name that signature as a type:

```rust
/// A picture: drawn by a function of the painter, the square it fills
/// and the colour to draw it in.
pub type Picture = fn(&Painter, Rect, Color32);
```

`fn(&Painter, Rect, Color32)` is a **function pointer** type. Any
function with that signature can be stored in a variable of type
`Picture`, put in an array, or passed to another function, without a
`Box` or a generic. A closure that captures nothing can be one too.

Each picture is drawn in a square, and its points are given as fractions
of the square, from 0 to 1, so the same picture can be drawn at any
size:

```rust
/// The point `x`, `y` of the way across and down `rect`, each from 0 to 1.
fn at(rect: Rect, x: f32, y: f32) -> Pos2 {
    rect.min + vec2(x * rect.width(), y * rect.height())
}

/// The mixer: three faders, each at its own level.
pub fn mixer(painter: &Painter, rect: Rect, color: Color32) {
    for (x, level) in [(0.2, 0.62), (0.5, 0.3), (0.8, 0.5)] {
        line(painter, rect, (x, 0.0), (x, 1.0), color.gamma_multiply(0.6));
        let knob = Rect::from_center_size(at(rect, x, level), vec2(6.0, 3.5));
        painter.rect_filled(knob, 1.0, color);
    }
}
```

The chord sheet is a page with chord names two to a line, the rack three
rows of steps with some lit, the piano roll a keyboard with three notes,
the harmony keys a keyboard with two black keys, the metronome its body
and arm.

### Five buttons from one table

The toolbar draws its five window buttons from an array. Each entry holds
the name, the `bool` that says whether that window is open, the picture
and the hint:

```rust
let windows: [(&str, &mut bool, icons::Picture, &str); 5] = [
    ("Chord sheet", &mut self.chord_sheet.open, icons::chord_sheet,
     "the song's harmony bar by bar, to read and write (F5)"),
    ("Channel rack", &mut self.rack_open, icons::rack,
     "the steps of the selected pattern (F6)"),
    // … the piano roll, the harmony keys, the mixer
];
for (name, open, picture, about) in windows {
    let button = icons::toggle(ui, name, *open, picture);
    if theme::hint(button, format!("{name}: {about}")).clicked() {
        *open = !*open;
    }
}
```

There are five `&mut` borrows of `self` in one array. The borrow checker
allows it because each one borrows a **different field**:
`self.chord_sheet.open`, `self.rack_open`, `self.piano_roll.open` and so
on. Rust tracks borrows field by field inside a function body, so
mutable borrows of different fields don't clash. Two `&mut` to the same
field would not compile.

### The tests

`the_toolbar_fits_on_one_row_at_1280_points` draws the whole app in a
1280 × 800 window and checks that PAT, SONG, TEMPO, the position, the
pattern box, VOL and all eight picture buttons are on the same row, and
that the last one ends before 1280.
`the_window_buttons_open_and_close_their_windows` clicks each of the five
and checks that its window opens, that the button lights, and that a
second click closes it. `every_picture_stays_in_its_square` paints each
picture into a 16-point square and checks that no shape goes outside it.

That last test failed on CI on every system, while it passed on the
machine where it was written. The difference was the build: CI runs the
tests in a **debug** build, and the test had been run in release.
`ctx.run_ui` returns everything egui would hand to the graphics card,
including the font's texture. egui checks, only in debug builds, that
such a texture is not dropped without being used, and panics if it is.
The fix, in a pull request of its own, clears it:

```rust
// The font texture is not drawn anywhere here.
output.textures_delta.clear();
```

The lesson is in the project's notes since then: run the tests in a
debug build too, as CI does. Checks written with `debug_assert!` are
compiled only into debug builds, and a library can have many of them.

## Every key in a list

The app's shortcuts were in tooltips, menu items and the hint line, but
nowhere all together. **Help > Keyboard shortcuts**, or **F1**, opens a
window with all of them, window by window.

![Help > Keyboard shortcuts: each section says when its keys work.](../images/step-9-workflow-shortcuts.png)

The list is data: a `Vec` of sections, each with a title, a note on when
its keys work and its lines. A line's keys are one of four kinds:

```rust
enum Keys {
    /// Any of these does it.
    Any(Vec<KeyboardShortcut>),
    /// The first does the first half of what the line says, the second the
    /// second: "Up / down".
    Pair(KeyboardShortcut, KeyboardShortcut),
    /// The mouse wheel, turned while holding these.
    Wheel(Modifiers),
    /// Keys that are not one shortcut, written out.
    Written(&'static str),
}
```

Wherever the app already had a constant for a shortcut, the list uses
that same constant: `files::SAVE`, `edit::UNDO`, `piano_roll::OCTAVE_UP`,
`view::MIXER`. So if a key changes, the list changes with it. Turning a
`Keys` into text uses egui's `format_shortcut`, which writes
"Ctrl+Shift+Z" on Linux and Windows and "⇧⌘Z" on a Mac, the way the
menus write them.

Because the list is data, it can also be tested as data.
`no_two_global_shortcuts_share_a_key` collects the keys of the sections
that work anywhere (Anywhere, Windows, File and Edit, Zoom) and fails if
any key is listed twice, since that key would do two things at once.

## Windows the size of what they show

egui remembers each window's size, and saves it with the rest of the
app's state when the app closes. That is right for a window you work
in, like the piano roll: you made it big because you wanted it big. It
is wrong for a window that is as big as what it shows. Drag the mixer
wide once, and every later time it opened across the whole playlist,
with its channels in one corner. The channel rack had the same trouble:
fitted to a long pattern once, it opened at that size over the 16 steps
of the next beat.

So there are now two kinds of window:

```rust
pub enum Opening {
    /// Each time it opens: a window as big as what it shows.
    EveryTime,
    /// The first time it opens after the app starts: a window to work
    /// in, which keeps the size you give it while the app runs.
    FirstTime,
}
```

The mixer, the shortcuts list, ear training, the twelve-tone row, the
pitch-class sets and the micromodes open **every time** at the size of
their contents, up to the height of the playlist, and scroll past that.
The piano roll, the chord sheet, the lead sheet and Settings open at
their own size the **first time** after the app starts, and keep a size
you give them while it runs.

`widgets::open_at` does it. It finds out whether the window is opening
now (it was not drawn in the last frame), and if so gives it a fixed
size for that one frame:

```rust
let first = ctx.data_mut(|data| {
    !std::mem::replace(data.get_temp_mut_or(id.with("has-opened"), false), true)
});
if opens(ctx, id) && (first || opening == Opening::EveryTime) {
    ctx.request_discard("a window opens at its own size");
    window.fixed_size(size)
} else {
    window
}
```

Three things in it are worth a look:

- `std::mem::replace(place, new)` puts `new` in `place` and returns what
  was there. Here it reads "had this window opened before?" and marks
  it as opened, in one step. The flag is stored with `get_temp_mut_or`,
  which egui does not save, so it is `false` again each time the app
  starts.
- egui lays a window out as it draws it, so on its first frame the
  window is drawn before anything knows how big its contents are.
  `request_discard` asks egui to throw that frame away and draw it
  again; the user only sees the second one, already at the right size.
- The "size" given is a narrow width and the greatest height. egui grows
  a window to fit wider contents but never makes it taller than they
  are, so the window ends up exactly as big as what it shows. For that
  to work, nothing in it may fill all the room it is given, such as a
  scroll area that does not shrink to its contents.

A window whose contents change while it is open, like the mixer when a
channel is added, is fitted again with `fit_when_changed`. It hashes a
summary of the contents (each channel's name and instrument) and fits
the window again when the hash changes.

The test `windows_open_at_their_own_size_not_one_remembered_from_before`
goes through seven windows. For each, it opens and closes it, writes a
size of 1300 × 900 into egui's memory, as dragging the window would,
starts the app again and checks that the window opens at its own size.
It keeps the seven windows in a table, with a closure that opens or
closes each one, stored as a function pointer:

```rust
type Open = fn(&mut LineupApp, bool);
let windows: [(&str, egui::Id, Open); 7] = [
    ("mixer", egui::Id::unique("mixer"), |app, open| {
        app.mixer_open = open;
    }),
    // … the shortcuts, ear training, the chord sheet, the lead sheet,
    // the piano roll, Settings
];
```

These closures capture nothing, so each one can become a plain `fn`
pointer, as the pictures above are. `the_mixer_is_as_big_as_its_channels`
checks that the mixer ends just past its Solo column and its last
channel, and `the_rack_fits_its_rows_again_when_the_app_starts` makes
the rack wide with a 64-step pattern, starts the app again on a short
one and checks that the rack fits the short one.

## If the app stops

These changes are about the way in, not about losing work. If the app
closes unexpectedly, it offers the recovery copy it wrote; **File >
Open recent** brings back the last ten projects; and **Settings > Audio
> Restart audio** starts the sound card again without reopening the app.
[Recovery, recent projects and restarting
audio](recovery.md) has the details.

## Exercises

1. Add a picture to `icons.rs` for **Follow playhead** in another
   style: a playhead with two arrows. Which test tells you when a line
   goes outside its square?
2. In `shortcuts.rs`, the "Harmony keys" section uses
   `Written("1 to 7")`. Why not seven `KeyboardShortcut`s in an
   `Any`? What would the list show then?
3. The lead sheet opens with `Opening::FirstTime`. What would change for
   you if it used `EveryTime`? Try it, and say which you prefer.
