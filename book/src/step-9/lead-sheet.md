# The lead sheet: the tune in notation

Files: [`crates/lineup-core/src/lead_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet.rs)
and the modules in [`crates/lineup-core/src/lead_sheet/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/lead_sheet)
(`melody.rs`, `spelling.rs`, `rhythm.rs`, `score.rs`, `layout.rs`,
`glyphs.rs`), and in the app
[`crates/lineup-app/src/lead_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet.rs)
with [`fonts.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet/fonts.rs)
and [`pdf.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet/pdf.rs)

A **lead sheet** is what a band reads from. It has one staff with the
tune (the "lead"), the harmony written above it as chord symbols, and
the song's key, time signature, sections and repeats. It leaves out
everything else: the bass line, the drums and how the piano voices its
chords are for the players to make up, in the style of the evening.
Jazz, pop and folk musicians carry whole books of them.

Ferrous Lineup already holds everything a lead sheet needs: the tune
is the notes of one channel, and the chords are the chord sheet's
([The chord sheet](chord-sheet.md)). This is item 11 of the plan: a
window that writes them out as a lead sheet, on pages you can print.

The window only reads the song. It changes nothing in it or in the
file, so the file format stays as it was.

## What you see

**View > Lead sheet** opens the window. The sheet is laid out on pages,
as it will print, and the bar that plays lights up while the song plays.
When the playing bar is out of sight, the window scrolls to it.

![The Lead sheet window on I Got Rhythm in B flat major while it plays: the melody on a treble staff with two flats, chord symbols above (Bb, Gm7, Cm7, F7), four bars to a line, and bar 2 lit in yellow](../images/step-9-lead-sheet-playing.png)

Along the top:

- **Melody** picks the channel whose notes are the tune. **Auto** takes
  the channel called Lead, which every song demo has, or else the
  channel whose notes are highest on average, which is where a tune
  usually is. Any channel with notes can be picked instead, the bass
  for a bass player's part. **None: chords only** writes no tune at
  all: each bar gets a slash for every beat, under the chord symbols,
  as a rhythm section reads its part.
- **Clef**: **Auto** writes a low tune in the bass clef and the others
  in the treble clef; **Treble** and **Bass** choose one.
- **Paper**: A4 (used almost everywhere) or Letter (in the USA and
  Canada).
- **−** and **+** zoom the pages from 50% to 200%.
- **Export PDF…** saves the sheet as a PDF file.
- **Print…** writes the PDF to the computer's temporary folder and opens
  it in your PDF viewer, to print from there. Printing that way works
  the same on Linux, macOS and Windows, and the viewer shows the
  printer's own settings.

![The same song with None: chords only: four slashes in every bar under the chord symbols](../images/step-9-lead-sheet-chords.png)

The paper and the zoom are remembered for next time.

## Reading a lead sheet

Here is the first page of La Cumparsita as the PDF prints it:

![The first page of La Cumparsita as a PDF: the title, a tempo mark "quarter = 112", a treble staff in G minor with two flats, chord symbols D7, Gm, Cm, Em7b5, Am7b5, and bar numbers 4, 7, 11 at the start of each line](../images/step-9-lead-sheet-pdf.png)

From left to right on the first line:

- The **clef**. The treble clef curls around the G line (the second
  line from the bottom); the bass clef's two dots sit around the F line.
- The **key signature**: the sharps or flats of the key, so they need
  not be written on every note. Two flats (B♭ and E♭) are B♭ major or
  G minor. La Cumparsita is in G minor, and its F♯ (the leading tone of
  the harmonic minor) gets a sharp each time, because the signature
  doesn't have it.
- The **time signature**, only on the first line and where it changes.
- The **tempo**: "♩ = 112", 112 quarter notes a minute. In 6/8 it is a
  dotted quarter, the beat you tap.

Above the staff are the **chord symbols**, each over the place where its
chord starts. A chord is written once, when it changes; it goes on
sounding until the next one. The small number at the start of each line
is the **bar number** of its first bar, so a band can say "from bar 21".

Further on, a lead sheet can hold:

- **rehearsal marks**: the name of a section ("A", "Verse", "Chorus") in
  a box, where the section starts. Each section starts a new line, so
  the form of the song shows at a glance;
- **repeat signs**: a thick and a thin bar line with two dots, around a
  section played twice; "×3" over the end when it is played three times
  or more;
- a **double bar line** (two thin lines) before a new section and
  before a change of key, where the new key signature is written, with
  naturals for the sharps or flats the old key had and the new one
  doesn't;
- the **final bar line** (thin and thick) at the end.

## From notes to notation

Notation is not a picture of the piano roll. The piano roll says "this
key, from this step, for this many steps"; notation has to choose a
letter for every note (A♯ or B♭?), write every length as note values
that can be counted, and show where the beats are. Musicians read
fluently because these choices follow rules, and the rules are what
this item is about. Each step is a module of `lineup-core`, and none of
them knows about windows or PDF files.

### The tune, one note at a time (`melody.rs`)

The notes of the melody channel are read along the playlist, clip by
clip, as they play. A lead sheet has one note at a time, so where
several notes start together the highest is kept (the "skyline" rule:
a tune harmonized in thirds is read as its top voice), and a note that
is still sounding when the next one starts is cut short there.

Notes played a little early or late (recorded from a keyboard, or
imported from a MIDI file) are moved to the nearest sixteenth. A lead
sheet shows what is meant, not every nuance of a performance.

### Which letter (`spelling.rs`)

The same key of the piano can be written two ways: the black key
between G and A is G♯ or A♭. Which one is right depends on what
the note *does*, and the rules are tried in this order:

```rust
pub fn spell(key: Scale, pitch: Note, chord: Option<ChordSymbol>, next: Option<Note>) -> NoteName {
    let pitch_class = pitch.pitch_class();
    if key.has_letter_for(pitch_class) {
        return key.spell(pitch_class);
    }
    if let Some(symbol) = chord {
        let names = symbol.chord().spell(symbol.root);
        // ... the chord's own name for the note, if it has the note
    }
    // ... a chromatic neighbour, named by where the line goes
    // ... otherwise the key's usual name for notes outside it
}
```

1. **A note of the key** is written as the key writes it: in A major,
   the black key above C is C♯, never D♭.
2. **A note of the chord** that sounds with it is written as the chord
   spells it. Over E7 in C major the black key above G is G♯, the major
   third of E (E G♯ B D). Over Fm in C major the same key is A♭, the
   minor third of F (F A♭ C). The letters follow the chord's thirds.
3. **A chromatic neighbour**, a note that moves a semitone to the next
   one, takes its letter from the way the line goes: up, it is the
   raised letter below (F♯ going to G); down, the lowered letter above
   (B♭ going to A). The note "leans" towards where it goes.
4. **Anything else** gets the name the key gives notes outside it.

One more rule keeps outside notes plain: outside the key, a white key
of the piano is written as one. In a key with many flats, a note B
between C and B♭ is written C – B – B♭, not C – C♭ – B♭, which reads
worse even though C♭ is the "same" note.

Once each note has its letter, the bar decides which **accidentals** to
print. An accidental lasts until the bar line: in G minor, the first F♯
of a bar gets a sharp, the next F♯ in that bar doesn't, and an F
after it gets a natural. The next bar starts again from the key
signature.

Rows and pitch-class sets have no key signature, so a song in a row or
a set is written without one, every black key with its own sharp or
flat.

### How long (`rhythm.rs`)

A note four sixteenths long is a quarter note, wherever it is. But the
reader counts beats, and notation must show them. So every place in a
bar has a **strength**:

```rust
/// - 4 for the bar line;
/// - 3 for the middle of a bar of four even beats (beat 3 of 4/4, the
///   third dotted quarter of 12/8);
/// - 2 for any other beat;
/// - 1 for the halves of a beat (the "and" of 4/4), or the thirds of a
///   dotted beat (each eighth of 6/8);
/// - 0 for anything smaller.
pub fn strength(time: TimeSignature, offset: usize) -> u8 {
```

The rules, as engravers use them (they are written down in Elaine
Gould's *Behind Bars*, the reference book of music engraving), say
which notes may cross which places:

- A note may hold over a beat only if it starts on a beat. A quarter
  note that starts on the "and" of beat 2 in 4/4 hides beat 3, so it is
  written as two eighths **tied** together: the second note isn't played
  again, the tie (a thin curve) holds the first one on.
- Nothing hides the middle of a bar of four beats, unless it starts at
  the bar line. A note from beat 2 to the end of the bar is a quarter
  tied to a half.
- One pattern is let through because every reader knows it: the
  syncopation "♪ ♩ ♪", a quarter from the middle of one beat to the
  middle of the next, inside one half of the bar.
- In 6/8 and in grouped meters like 2+2+3/8 notes hold only from beat to
  beat, so the groups stay visible.
- Rests are stricter still: a rest covers several beats only from the
  bar line or the middle of the bar, as a half rest on beat 1 or 3 of
  4/4.

When a note doesn't fit in one value, it is cut at the strongest place
inside it first, and the pieces are tied. Here are four bars of I Got
Rhythm: the tune's syncopations come out as quarters and eighths tied
over beat 3, the middle of the bar, so the reader always sees where it
is.

![Bars 9 to 12 of I Got Rhythm: a quarter rest, F tied over the middle of the bar to an eighth, G tied to a quarter; B flat dotted quarter, C eighth tied to a half note](../images/step-9-lead-sheet-ties.png)

**Beams** join eighths and sixteenths beat by beat (by the whole bar
when the beats are themselves eighths, as in 3/8), so the beats show
here too. A beam slants with the notes, but never by more than one
staff space, and a sixteenth beside an eighth gets a short second beam
of its own, a "stub".

### What a bar holds (`score.rs`)

`score.rs` puts these together bar by bar. A bar holds its notes and
rests (each with its value, letter, accidental and tie), the chords
that start in it, and whatever is written at its edges: a new key or
time signature, the start of a section, repeat signs. Before the first
note of the tune and after the last one, a bar where the band plays a
chord gets slashes instead of rests, so an introduction or an ending
the band plays alone reads as "play these chords".

### Where everything goes (`layout.rs`)

Then everything is placed on the page, in points (1/72 of an inch). The
staff is 21.2 points high (7.5 mm), four **staff spaces** of 5.3 points,
the usual size for a lead sheet. Every size in the layout is counted in
staff spaces.

Notes get room by how long they last, but not in proportion: a half
note doesn't get twice the room of a quarter note, or a bar of
sixteenths would be crushed next to a bar of whole notes. Engravers
space by something close to the square root of the length:

```rust
/// How much room a note or rest gets for lasting `steps` sixteenths,
/// before the line is stretched: about 2.6 spaces for a sixteenth, 4.2
/// for a quarter, 7.4 for a whole note.
fn spring(steps: usize) -> f32 {
    sp(1.0 + 1.6 * (steps as f32).sqrt())
}
```

A chord symbol is wider than a note, so a quick chord change ("Gm G7"
over two quarter notes) pushes its notes apart until the symbols don't
touch.

A line holds up to four bars, fewer if they don't fit, and a new
section starts a new line. Each full line is then **stretched** until it
reaches the right margin exactly. How much to stretch is found by
trying: draw the line with a stretch of 0.25 and of 6, then halve the
gap 24 times, keeping the half where the width lands. This is a
**binary search**, the same idea as finding a word in a dictionary by
opening it in the middle:

```rust
fn fit(&self, bars: Range<usize>, width: f32) -> f32 {
    let wide = |stretch: f32| self.draw_system(bars.clone(), stretch).width;
    let (mut low, mut high) = (0.25_f32, 6.0_f32);
    // ...
    for _ in 0..24 {
        let middle = (low + high) / 2.0;
        if wide(middle) > width {
            high = middle;
        } else {
            low = middle;
        }
    }
    low
}
```

`wide` is a **closure**, a small function written inline that borrows
`self` and `bars` from around it. The line is drawn up to 26 times to
place it once; it is quick, because drawing a line only computes
positions.

The last line of a section or of the song is often short. Stretched to
the margin, its few notes would sit far apart, so it is spaced like the
full lines and left short, as engravers do. Lines then go on pages,
spread out so each page is filled from top to bottom, except the last.

### One list of marks

The layout's result is a list of **marks** per page: plain values that
say what to draw where.

```rust
pub enum Mark {
    Music { glyph: glyphs::Glyph, at: Point, size: f32 },
    Text { text: String, font: TextFont, at: Point, size: f32 },
    Line { from: Point, to: Point, width: f32 },
    Beam { from: Point, to: Point, thickness: f32 },
    Tie { from: Point, to: Point, up: bool, thickness: f32 },
}
```

The window paints these marks on screen and `pdf.rs` writes the same
marks to the PDF, so what is printed is exactly what was shown. This is
the same split the app uses everywhere: `lineup-core` decides, the app
only draws. It also makes the layout testable without a window: a test
can lay out a song and look for a notehead per note, a box per bar, and
nothing outside the page.

## The fonts

Music symbols come from a **music font**. Fonts that follow
**SMuFL**, the Standard Music Font Layout, give every symbol a fixed code
point (the treble clef is U+E050 in all of them) and are drawn so that
one em, the font's size, is the height of the staff. The app uses
**Bravura**, the font SMuFL was designed with, by Steinberg. The words
and chord symbols are in **Inter**, a font made for screens that prints
cleanly too. Both are free, under the SIL Open Font License, and are
built into the program with `include_bytes!`, so the sheet looks the
same on every computer:

```rust
const BRAVURA: &[u8] = include_bytes!("../../assets/fonts/Bravura.otf");
```

`include_bytes!` is a macro that reads the file when the program is
compiled and puts its bytes in the program, as a `&'static [u8]`: a
slice that lives as long as the program does.

The layout needs to know how wide each symbol is and how far it reaches
up and down. Those numbers come from Bravura's own metadata, copied into
`glyphs.rs`, and a test reads the font file to check every one of them.

Text is measured with the very font it is drawn with. `lineup-core` has
no fonts, so it asks through a **trait**:

```rust
pub trait TextWidth {
    fn text_width(&self, text: &str, font: TextFont, size: f32) -> f32;
}
```

The app implements it by adding up the letters' widths read from the
font file with the `ttf-parser` crate; the tests implement it with a
simple rule (every letter half as wide as the font size). The layout
takes a `&dyn TextWidth` and works with either.

Chord symbols follow the custom of lead sheets: the root letter large,
the rest a little smaller ("Am7", "G7"), and real ♯ and ♭ signs from the
music font, raised, instead of "#" and "b".

## The PDF

A PDF page is a list of drawing operations, much like the marks: move
to a point, draw a line to another, fill a shape. The
[`pdf-writer`](https://crates.io/crates/pdf-writer) crate writes them
in the PDF's format; it runs on every platform and needs nothing from
the system.

Letters and symbols are not embedded as fonts. Each one is copied in as
its **outline**, the curves that draw it, read from the font file with
`ttf-parser`, and filled. A symbol used many times, like a black
notehead, is written once as a "form XObject" and then placed wherever
it appears, scaled to its size:

```rust
content
    .save_state()
    .transform([scale, 0.0, 0.0, scale, x, height - at.y])
    .x_object(Name(name.as_bytes()))
    .restore_state();
```

The `transform` is a matrix that scales the shape from the font's units
to points and moves it to its place. Notice `height - at.y`: PDF counts
y upwards from the bottom of the page, the layout counts it downwards
from the top, so every y is turned round.

A two-page lead sheet is about 130 KB, and it looks the same in every
PDF viewer, with nothing to install.

## How it was tested

`lineup-core` has 41 tests for the lead sheet. Among them:

- **spelling**: the key's letters; a chord's letters (G♯ over E7 and A♭
  over Fm in C major); chromatic neighbours by direction; white keys
  outside the key stay plain (B in F major, but E♯ in F♯ major, where
  it is a note of the key); accidentals that last to the bar line;
  key signatures in both clefs; the naturals of a key change;
- **rhythm**: note and rest values in 4/4, 3/4, 6/8, 5/4 and 2+2+3/8;
  ties over the middle of the bar; the "♪ ♩ ♪" syncopation left untied
  in simple time and tied in 6/8; beam groups;
- **melody and score**: the tune read from repeated clips, notes
  rounded to the grid, the skyline rule, notes tied over the bar line,
  the clef chosen from the tune, slashes before the tune starts, key
  changes, sections and repeats;
- **layout**: a notehead for every note and a box for every bar,
  everything inside the page, a section starting a new line, chord
  symbols that never run into each other, beams and ties.

The app has three: every symbol of the music font is in Bravura at the
size the layout thinks it is, text is measured with the font, and El
Choclo becomes a PDF with a page object per page and its shapes written
once each.

Then the sheets were read, as a musician reads them, for Ode to Joy,
Für Elise, Minuet in G, Careless Love, Korobeiniki, Funiculì,
funiculà, The Entertainer, the Moonlight Sonata, El Choclo,
Greensleeves, La Cumparsita, When the Saints Go Marching In and I Got
Rhythm. That reading found the rules above that the tests didn't have
yet: the C – B – B♭ spelling, the "♪ ♩ ♪" syncopation, more room after
a bar line, and a double bar line before a change of key.

## Exercises

1. **Courtesy accidentals.** When a bar has an F♯ and the next bar an F
   (natural, by the key signature), many lead sheets print a natural in
   brackets as a reminder. Where would `BarAccidentals` need to know
   about the bar before?
2. **For B♭ instruments.** A trumpet or a tenor saxophone reads music a
   major second higher than it sounds. Add a choice that writes
   everything, key signature and chord symbols too, a major second up.
   Which module changes, and which don't?
3. **Endings.** When the last bar of a repeated section differs the
   second time, lead sheets use first and second endings: a bracket
   "1." over one bar, "2." over the other. What would the playlist need
   to tell `score.rs`?
4. **Bars of rest.** A tune that is silent for eight bars could show
   one bar with a thick line and "8" over it. Where in `systems` would
   it go, and how does it change the bar numbers of the next line?
5. **Half-diminished.** Some lead sheets write Em7♭5 as Eø7. Add a
   choice for it. Is it a choice of `layout.rs` or of the chord's name?
