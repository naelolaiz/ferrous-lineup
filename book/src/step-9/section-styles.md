# A style for each section

Files: [`crates/lineup-core/src/band/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/sections.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs)
and [`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)

Many songs change their feel partway through. A Latin jazz tune plays
its verse as a bossa nova and its bridge in swing; a pop song goes to
half time for the bridge; an arrangement starts as a ballad and ends
as a rock song. [Variations a and b](variations.md) make a part of a
song fuller, but in the same style. Now each part of a song can have a
style of its own.

![The chord sheet of an eight-bar song in Pop beat. Bar 5 has the word "Swing" in orange after its number: from there the band plays in swing.](../images/step-9-section-styles-sheet.png)

## Trying it

1. Open the chord sheet with **F5**, with a band playing (any demo,
   or a Library style).
2. Right-click a section's name on the sheet, for example **Chorus**.
   The section's bars are selected, and the menu has **Variation a**,
   **Variation b** and **Style: …**, which names the style playing
   there now.
3. Open **Style**, then a genre, and pick a style, say **Latin** ›
   **Bossa nova**. The bars of that section are now played as a bossa
   nova. The sheet writes "Bossa nova" after the number of the first
   bar, and the notice says which bars changed.
4. The same **Style** menu is in every bar's right-click menu. Without
   a selection it changes the style from that bar on; with bars
   selected (click the first, Shift+click the last), it changes only
   those bars, and the bars after them keep what they played.
5. **The song's style**, at the top of the menu, goes back to the
   band's own style.

![The menu of bar 3: Variation a, Variation b, and "Style: Pop beat" opened, with "The song's style: Pop beat" at the top, then one entry per genre (Pop, Rock, Blues, Jazz, Soul, funk and R&B, ... Classical and film). Classical and film is open: Epic toms, March, Chorale, Alberti bass, Arpeggios.](../images/step-9-section-styles-menu.png)

The menu only lists styles in the time signature of those bars. A
waltz in the middle of a song in 4/4 would need bars of three beats,
which means changing the song's bars, not only how the band plays
them. Changing the time signature is a separate step, so it stays
separate here.

## What changes and what stays

A section's style changes **how** the band plays:

- the piano's **comping**, the rhythm it strikes the chords in,
- the **bass line**,
- the **drums**,
- the patterns of the guitar, the strings and the organ, where the
  band has them,
- the **swing**: a swing section in a straight pop song swings its
  eighth notes, and a rock section in a jazz song plays them even.

It does not change **who** plays or the sounds they play with. The
band keeps its instruments, so the same piano and the same bass go
from the bossa nova verse to the swing bridge, as a real band would.
To change the instruments, use the band's **Instrument lineup**.

The drums tell the listener a change is coming, as they do for a
variation: the bar before the change plays the fill of the style that
is ending, and the first beat of the new style gets a crash cymbal.
Each player's comping pattern starts again from its beginning on the
first bar of the new style, so a two-bar bossa nova pattern does not
start halfway through.

## How it works

### Markers, the same way as variations

```rust
pub struct StyleMarker {
    pub start: usize,
    pub style: Option<&'static BandStyle>,
}
```

A marker with `style: None` goes back to the band's own style. The
markers live in the `Band`, next to the variation markers, and follow
the same rules: `tidy` keeps only markers that change something, and
everything that moves the song (an intro added, a cut, a section
copied, a new time signature, "End the song here") moves them too.

`&'static BandStyle` is a reference that lives for the whole program:
the band styles are built once (`band_styles()` keeps them in a
`OnceLock`) and never freed. That is why a marker can hold a
reference instead of a copy or a name. The file can't hold a
reference, so it saves the style's name and looks it up again when it
reads the song.

### Every style plays the whole song, then each keeps its own bars

The simplest way to write a part with several styles was not to teach
every piece of the band about sections. Instead, `write_band` asks
each style that plays somewhere for the whole song, and keeps each
note only where that style plays:

```rust
for style in sections::styles_played(&band.styles) {
    let playing = band.playing(style);
    let part = self.band_notes(&playing, player, &score);
    notes.extend(sections::where_it_plays(part, &band.styles, style));
}
```

`band.playing(style)` is a copy of the band with the other style's
patterns. `band_notes` is the code that already wrote a player's part,
moved into a method of its own so it can be called once per style.
`where_it_plays` keeps the notes that start where `style` plays, and
cuts a note that would ring into the next style:

```rust
notes
    .into_iter()
    .filter(move |(_, note)| style_in(markers, note.start) == style)
    .map(move |(channel, mut note)| {
        if let Some(change) = next_change(markers, note.start)
            && note.start + note.length > change
        {
            note.length = change - note.start;
        }
        (channel, note)
    })
```

`move` makes each closure own what it uses (`markers` and `style`, both
cheap to copy), so the iterator can be returned from the function and
used after it ends. A song without section styles has one style,
`None`, and keeps every note: it sounds as it did.

The fills come from the same idea. The drums already played a fill in
the bar before a change of variation; style changes are now changes
too. When the band plays the outgoing style through the whole song,
that style has a fill before the change, and the bar before the
change belongs to it, so its fill is kept. The incoming style has a
crash on the change, and the change is its first bar, so its crash is
kept.

### Swing per section

Swing is played, not written: notes stay on the grid, and playback
moves the second eighth note of each beat later
([Swing](swing.md)). The amount used to be one number for the whole
song. Now a band pattern in a section with its own style swings as
that style does:

```rust
pub fn pattern_swing(&self, pattern: usize) -> u8 {
    let swing = self.section_swing(pattern).unwrap_or(self.swing);
    // ...
}
```

`section_swing` looks at where the pattern's first clip is and returns
that section's swing, or `None` (the song's swing) for your own
patterns and for sections in the band's own style. The band starts a
new pattern where a section's style starts, and two stretches with the
same notes but different swing never share a pattern, so each pattern
swings only one way.

The playback trace ([A playback trace](playback-trace.md)) shows it:
in a straight Pop beat song whose second half is Swing, the ride
cymbal and the piano's offbeats in bar 5 start at 2.667 sixteenths
into the beat, the last third of a triplet, while in bars 1 to 4 they
are on the grid.

A song with section styles needs at least format 24 ([the project file
format](../appendix-project-format.md)).

## Exercises

1. Write | C | Am | F | G | twice in **Pop beat**, and make bars 5 to 8
   **Bossa nova**. Open the bass's patterns in the piano roll. Where
   does the bossa nova's bass line start, and how is it different from
   the pop one?
2. Make only bar 3 of the same song **Swing**. Which bars have a fill,
   and where do the crashes fall?
3. Play a section in **Chorale**, which has no drums. What do the drums
   play there, and what do they play in the bar before it?
4. (Rust) `where_it_plays` returns `impl Iterator`. What would change if
   it returned a `Vec` instead? Why does `notes.extend(...)` accept
   both?
5. (Rust) `Band::playing` clones the whole band. Which fields of `Band`
   make that clone costly, and why is it still fine here?
