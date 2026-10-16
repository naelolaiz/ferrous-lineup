# Exercises

## 1. A style of your own (loom-core, easy)

Add a style to `STYLES` in `library/style.rs`: a progression from a
song you like, written in numerals. For example the "Pachelbel"
progression, `I | V | vi | iii | IV | I | IV | V`, with
`Comping::ArpUpDown` and `Bass::Held`. Run `cargo test -p loom-core`
before you open the app: if a numeral is misspelled, or a bar cannot be
shared by its chords, a test names the style.

Then try it in another key from the Library tab, and look at its
tooltip. Which chords change their names, and which numerals stay the
same?

*Practices:* `const fn` tables, struct literals, reading test failures.

## 2. A chacarera (loom-core, easy)

The chacarera, a folk dance from the north of Argentina, is in 6/8 with
a bass drum (the *bombo legüero*) that plays across the beats. Add it
as a rhythm in the Folk genre, with `.in_time(SIX_EIGHT)`. A bar of 6/8
is twelve steps, so each line has twelve characters:

```text
Kick  "X.....  ..x.x."
Rim   "..x.x.  x....."
```

Write a style that uses it, and check in the rack that the beat colours
come in two groups of six.

*Practices:* the rhythm notation, `const` time signatures.

## 3. Latch (loom-app, easy)

To record a long chord, you have to hold the harmony key for the whole
chord. Add a **Latch** toggle next to Triads, 7ths and 9ths: while it
is on, a harmony key keeps sounding after you let go, until another key
is pressed or Latch is turned off.

Look at `Down` and `Holder`. Could a latched chord be a third kind of
`Holder`? What must happen to it when the bar is closed with F8?

*Practices:* enums with data, state machines.

## 4. Borrowed chords (loom-core and loom-app, medium)

Songs in major keys often borrow chords from the minor key with the
same tonic: iv (F minor in C), bVI (A♭), bVII (B♭) and bIII (E♭). This
is called *modal interchange*, and it is behind a lot of the bittersweet
moments in pop.

Add a second, smaller row of keys under the harmony keys with these
four chords, built from numerals with `library::numeral::parse` and
`RomanChord::in_key`. Give them their own colour, and a tooltip that
says which minor-key chord each one is.

*Practices:* reusing the library's numerals, egui layout.

## 5. A song of your own (loom-core, medium)

Add a song in the public domain to `SONGS`. Some that are not in the
library yet: *Brahms' Lullaby* (3/4), *Yankee Doodle*, *Scarborough
Fair* (the traditional tune, in Dorian), *La Paloma* (a habanera from
1863, so it can use `Comping::Habanera`).

Write the melody bar by bar, with `|` between bars, and let the tests
tell you which bars do not add up. Then open it from **File > Demos**.
Does `analyze` find the chords you wrote while it plays? If not, which
notes confuse it?

*Practices:* the melody notation, tests as a safety net for data.

## 6. Changing time in the middle (loom-core, hard)

A project has one time signature. Real songs sometimes change: a bar of
2/4 at the end of a phrase, a chorus that goes from 4/4 to 3/4. Design
a *meter map*: a list of `(bar, TimeSignature)` changes, sorted by bar.

- Write `fn bar_start(&self, bar: usize) -> usize` and
  `fn bar_at(&self, step: usize) -> usize`, with tests first. Which one
  is a search, and could it use `partition_point`?
- Which parts of the app would have to ask the map instead of
  `project.time_signature`? Make a list before you write any code.
- How would you save it, so that projects with one time signature are
  written exactly as now?

*Practices:* designing a data structure, binary search on sorted data,
keeping files compatible.

## 7. Remember the hint level (loom-app, medium)

The hint level goes back to Short every time the app starts. eframe
already saves egui's own memory between runs (the zoom and the window
positions, see [Fitting any window](fitting-the-window.md)), because its
`persistence` feature is on. The app's own settings need a little more:
give `HintLevel` serde's `Serialize` and `Deserialize`, store it in
`App::save` with `eframe::set_value`, and read it back with
`eframe::get_value` from `CreationContext::storage` when the app starts.
Which other settings would you remember the same way? Whether the
channel rack is open is one.

*Practices:* Cargo features, serde derives, the `eframe::App` trait.
