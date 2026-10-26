# Exercises

## 1. A style of your own (lineup-core, easy)

Add a style to `STYLES` in `library/style.rs`: a progression from a
song you like, written in numerals. For example the "Pachelbel"
progression, `I | V | vi | iii | IV | I | IV | V`, with
`Comping::ArpUpDown` and `Bass::Held`. Run `cargo test -p lineup-core`
before you open the app: if a numeral is misspelled, or a bar cannot be
shared by its chords, a test names the style.

Then try it in another key from the Library tab, and look at its
tooltip. Which chords change their names, and which numerals stay the
same?

*Practices:* `const fn` tables, struct literals, reading test failures.

## 2. A second chacarera (lineup-core, easy)

The library already has `Chacarera` and `Chacarera trunca` in `STYLES`
and `RHYTHMS`, and *Chacarera del patio* in `SONGS`. Read the two
rhythms first: `Chacarera` in 6/8 sets the bombo's head against its rim,
and `Chacarera trunca` in 3/4 cuts its phrases short. Then add a third
rhythm for the same dance with a different bombo pattern, and a style
that uses it. Keep 6/8 if you write a chacarera: a bar is twelve steps,
so each line has twelve characters:

```text
LowTom  "X.....  ..x.x."
Rim     "..x.x.  x....."
```

Check in the rack that the beat colours come in two groups of six. Why
does the existing `Chacarera` style fit a 6/8 bar while the trunca needs
3/4?

*Practices:* reading existing data before adding to it, the rhythm
notation, `const` time signatures.

## 3. Latch (lineup-app, easy)

To record a long chord, you have to hold the harmony key for the whole
chord. Add a **Latch** toggle next to Triads, 7ths and 9ths: while it
is on, a harmony key keeps sounding after you let go, until another key
is pressed or Latch is turned off.

Look at `Down` and `Holder`. Could a latched chord be a third kind of
`Holder`? What must happen to it when the bar is closed with F8?

*Practices:* enums with data, state machines.

## 4. Borrowed chords (lineup-core and lineup-app, medium)

Songs in major keys often borrow chords from the minor key with the
same tonic: iv (F minor in C), bVI (A♭), bVII (B♭) and bIII (E♭). This
is called *modal interchange*, and it is behind a lot of the bittersweet
moments in pop.

Add a second, smaller row of keys under the harmony keys with these
four chords, built from numerals with `library::numeral::parse` and
`RomanChord::in_key`. Give them their own colour, and a tooltip that
says which minor-key chord each one is.

*Practices:* reusing the library's numerals, egui layout.

## 5. Extend a song that is already there (lineup-core, medium)

*La Paloma*, *Shenandoah* and *Danny Boy* are in `SONGS` now, and each
shows something different: *La Paloma* has verse, chorus and coda;
*Shenandoah* is a river song that starts on a pickup and reaches high;
*Danny Boy* keeps its verse low and leaps up in the chorus. Pick one and
add a part the demo does not play yet: an introduction, a second verse,
or a coda. Write the melody bar by bar with `|` between bars, give it a
chord chart, and add it to the song's `form`.

Let the tests tell you which bars do not add up. Then open it from
**File > Demos**. Does `analyze` find the chords you wrote while it
plays? If not, which notes confuse it?

*Practices:* the melody notation, reading existing data, tests as a
safety net for data.

## 6. A meter map inside a pattern (lineup-core, hard)

Step 8 already has a song-level meter map: `Meter` keeps the starting
`TimeSignature` and a sorted list of `TimeMarker`s, and `bar_at`,
`time_at` and `click` read it. A `Pattern` still has only one
`time: Option<TimeSignature>`, so a part that changes meter every bar
is cut into a pattern per run of bars.

Take the next step: let one pattern hold a meter map of its own, with
changes at bar lines. `Pattern::length` counts steps, and a bar is no
longer one fixed number of them, so:

- Which code that asks `TimeSignature::steps_per_bar` or `Meter::bar_at`
  for a pattern would need to ask the pattern's map instead: the note
  tools, `Project::pattern_clicks`, the piano roll's bar lines?
- What should the playlist and the piano roll ask when a clip starts
  partway through a pattern?
- How would `format_for` account for it, so a pattern in one meter is
  written exactly as now?

Start with `Meter` rather than inventing a second type. Why does keeping
the changes in a sorted list make `bar_at` a search, and why does the
existing `Meter` not simply use `partition_point`?

*Practices:* reading an existing data structure before extending it,
bar arithmetic, keeping files compatible.

## 7. Remember the hint level (lineup-app, medium)

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

## 8. Zoom the popups too (lineup-app, small)

Zoom the channel rack to 150% and right-click a channel's name: the menu is
drawn at the normal size. A popup is a layer of its own, so it never
sees the style the window's contents changed (see [Finding your
notes](finding-your-notes.md)). Find where egui lets you change the
style of a popup or a combo box, and give it the window's scale. (The
playlist and the piano roll no longer scale their text when they zoom:
see [A bug hunt](bug-hunt.md).)

*Practices:* reading a library's source, egui's layers and styles.

## 9. Hints everywhere (lineup-app, small)

Only the toolbar and the harmony keys give the hint bar something to
say (see [A clearer look](look-and-transport.md)). The channel rack,
the playlist and the piano roll still use plain `on_hover_text`. Change
them to `theme::hint`, and make the hint bar show the current zoom or
the pattern's name when nothing is under the mouse. Why does
`theme::hint` take the `Response` by value and give it back, instead of
taking `&Response`?

*Practices:* ownership through a function, egui's `Response`, reading
someone else's module before changing it.

## 10. Listen better (lineup-core, medium)

Hearing chords gets about 91% of the library right when every style is
played in every way (see [A new rhythm for a song](new-rhythm.md)).
Write that check as a test of your own: loop over `STYLES`, play each
chart with every `Comping` and a few `Bass` lines using
`chart::accompany`, call `hear_chords`, and count how often the chords
come back as written. Print the combinations that fail most. Then try
to raise the number: change one constant at the top of `listen.rs`, or
listen to more of the bass line (a bass note held for a whole beat is
probably a chord note). Keep `every_style_is_heard_as_its_chart`
passing. Run it with `cargo test --release`: in release mode the same
test runs many times faster.

*Practices:* measuring before changing, nested loops over enums, tuning
by tests.

## 11. A tempo change on a pattern (lineup-core, small)

`TempoMark` puts a demo's markings on the playlist (see [Tempo changes,
and playing like a person](tempo-and-feel.md)). The Toccata in D minor
has six fermatas in the bars its demo plays, left out because the demo
already writes those notes long. Shorten the notes back to what Bach
wrote, add the fermatas with `.with_tempo`, and listen to both. Count
each fermata's step from the start of the part. Does
`every_demo_keeps_a_sensible_tempo` still pass? What would it take to
make it fail?

*Practices:* `const` slices of enums, counting steps in a bar.

## 12. Lean on the highest note (lineup-core, medium)

The melody shaping in `library/interpret.rs` gives a note up to six
velocity steps for its height in the phrase. Singers often lean on the
single *highest* note of a phrase more than on the notes near it. Give
that one note two more steps, and write the test first: in "Twinkle,
twinkle", which note is it, and which phrase is it in?

*Practices:* `Iterator::max_by_key`, writing a test before the code.
