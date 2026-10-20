# Shots, holds, rests and pushes: the band breaks the groove

Files: [`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-core/src/band/stops.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/stops.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)
and [`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)

A style plays the same groove bar after bar, and that is what makes it
a style. But listen to a real band: at the end of a phrase they all hit
one chord together and stop dead, or hold a chord out under the singer,
or drop out for a bar so the drummer or the soloist is alone, or come
in with the next chord a little before the beat. Arrangers write these
moments into the chart, and they are what make a band sound arranged
rather than looped. Now the chord sheet has marks for them.

![The chord sheet with C, ^F, G7., Am.. in the first row and F G7..., ^^C, Dm7, G7 in the second. Under each harmony, its numeral and its mark: "IV · push", "V7 · shot", "vi · hold", "V7 · rest". Bar 3 is right-clicked, and the menu "Band on G7" is open: "As the style goes", "Shot (.)" (chosen), "Hold (..)", "Rest (...)", then "On the beat" (chosen), "Push (^)" and "Push a sixteenth (^^)"](../images/step-9-stops-menu.png)

## Trying it

Press **Ctrl+N** for a new chord sheet and type these bars, one per
bar:

```text
| C | ^F | G7. | Am.. | F G7... | ^^C | Dm7 | G7 |
```

Under each marked harmony, a word says what the band does there:
**push**, **shot**, **hold** or **rest**. Hover over one for the whole
sentence, as in "Hold: the band strikes it and holds it until the next
harmony". Press Space and listen:

- bar 2: the band plays F on the "and" of beat 4 of bar 1, an eighth
  note early, and lets it ring over the bar line, where it is written;
- bar 3: the whole band strikes G7 once, short, with a kick and a
  crash, and then nothing until bar 4;
- bar 4: Am struck and held for the whole bar;
- bar 5: F in the style for two beats, then silence for G7;
- bar 6: C a sixteenth note early;
- bars 7 and 8: the style again.

The playlist shows the same: the band's parts have gaps and long notes
where the marks are.

![The playlist with the Piano, Bass, Drums and Guitar tracks over eight bars. Each track is busy in bars 1, 2, 7 and 8. In bar 3 every track has one short hit at the start and then nothing; in bar 4 one long note or one hit; bar 5 is empty in its second half, and the notes of bar 6 start just before its bar line](../images/step-9-stops-playlist.png)

The marks can also be set without typing: right-click a bar and pick
**Band on G7** (one item for each harmony in the bar), then a stop or a
push. **As the style goes** and **On the beat** take the marks off.
Like every change on the sheet, the band plays again, undo takes it
back, and the marks are copied with the bars, as text too
([Chord sheets as text](text-chord-sheets.md)):

```text
| C. | ^F | G7.. Am... | ^^C |
```

A song with marks is saved as format 18; a song without any is saved
as before, so older versions keep opening it.

## The music: breaking the groove

### Shots

A **shot** (also called a hit, a stab or a kick) is a chord the whole
band plays at once, short and accented, followed by silence. Big band
charts are full of them: the brass hits a chord, and the drummer plays
it with the band on the crash cymbal and the bass drum, often with a
short fill before it to "set it up". A run of shots, one at the start
of each bar or two, is called **stop-time**: the band plays only the
hits and leaves the rest of the bar to the singer, the soloist or a tap
dancer. In the verses of Muddy Waters' "Hoochie Coochie Man" the band
plays only a short riff at the start of each bar and the voice fills
the silence; it is one of the best-known stop-time grooves in the
blues.

Here a shot is an eighth note. Every player strikes the chord its own
way (the piano its voicing, the bass the bass note, the guitar a
strum), a little harder than the style, and the drums play a kick and
a crash. Then the band is silent until the next harmony. That is how
the end of bar 3 above works: the silence is part of the music, and
the harmony is still G7 for whoever plays over it.

### Holds

A **hold** is a chord struck and held. Bands use it at the end of a
phrase, for a big chord under the last word of a line, or to let a
soloist play freely over one harmony. It is not a fermata: a fermata
stops the beat, and the band waits for the conductor, while a hold
lasts exactly until the next harmony, and the beat goes on underneath.
Each player strikes the chord with a kick and a crash, as for a shot,
and lets it ring. A row form, which the band plays as trichords one
after another, is held one trichord at a time: each is struck as it
comes and held until the next.

### Rests

A **rest** is the band dropping out. Arrangers call it a **break**: in
jazz, the band stops for a bar or two and the soloist plays alone (the
famous "break" at the end of a tune's first chorus), and in funk and
hip hop the bars where only the drummer plays gave the "breakbeat" its
name. Here the whole band rests, drums too; type the drums' break
yourself in the piano roll, or play it with **You play**.

### Pushes

A **push** (or **anticipation**) is a chord played before the beat it
belongs to and held over it. In C | ^F, F is written on beat 1 of bar
2 but played on the "and" of beat 4 of bar 1, and the band does not
strike again on beat 1. A chart writes it as an eighth note tied over
the bar line:

```text
 bar 1               bar 2
 1   2   3   4 &   | 1   2   3   4
 C   C   C   C F---|-F   F   F   F
               ^ the F is struck here, and held over beat 1
```

Without the push, the downbeat is the strongest moment of the bar, and
everybody expects it. With it, the chord comes early and beat 1 is
silent: the music leans forward, as if it could not wait. That is
**syncopation**, an accent off the beat, and pushes are everywhere in
pop, rock, funk and Latin music. Salsa bass players anticipate nearly
every chord: in the **tumbao**, the bass plays the next chord's root on
beat 4, held over the bar line. A push of a sixteenth note (`^^C`) is
subtler, the pull of funk and fast Latin music.

A push can be combined with a stop: `^G7.` is a shot an eighth early.

### Why dots and carets

Dots after a harmony and carets before it are a short way to write the
marks without leaving the keyboard: one dot for the shortest (a shot),
three for the longest silence (a rest). The caret `^` looks like the
**marcato**, the accent sign musicians write over a note to play it
strongly, and a push is played with an accent. `N.C.` (no chord) keeps
its own dots: they are part of its name.

## How it works

### Two optional marks on a marker

A harmony on the sheet is a `ChordMarker`: where it starts and what it
is. It gets two more fields, each an `Option`, `None` when it is not
marked:

```rust
pub struct ChordMarker {
    pub start: usize,
    pub harmony: HarmonyName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
}

pub enum Stop { Shot, Hold, Rest }
pub enum Push { Eighth, Sixteenth }
```

The two `serde` attributes, which you met in
[A bigger band](bigger-band.md), keep the file as it was when there are
no marks: `skip_serializing_if` leaves the field out, and `default`
reads a file without it as `None`. The file's format only becomes 18
when some marker has a mark, checked with `any`:

```rust
let has_breaks = project.playlist.chord_markers
    .iter()
    .any(|marker| marker.push.is_some() || marker.stop.is_some());
```

Making a marked marker reads like a sentence, thanks to two small
methods that take the marker by value and give it back changed:

```rust
pub fn pushed(self, push: Option<Push>) -> Self {
    ChordMarker { push, ..self }
}

ChordMarker::new(16, f).pushed(Some(Push::Eighth))
```

`..self` is the struct update syntax again: every field not named comes
from `self`. Because `pushed` takes `self` and not `&self`, it uses the
old marker up and returns a new one, so the calls can be chained:
`.pushed(...).stopped(...)`.

### Reading the marks

Typing `^^Am7/G..` must give a push, the harmony `Am7/G` and a stop.
`read_marks` peels the marks off both ends and counts them:

```rust
let name = text.trim_start_matches('^');
let push = match text.len() - name.len() {
    0 => None,
    1 => Some(Push::Eighth),
    2 => Some(Push::Sixteenth),
    _ => return Err("a push is written ^ (an eighth early) or ^^ (a sixteenth).".to_string()),
};
let bare = name.trim_end_matches('.');
let stop = match name.len() - bare.len() { /* 0 to 3 dots */ };
Ok((push, bare, stop))
```

`trim_start_matches` returns a **slice** of the same string without the
carets, not a copy, so the difference of the two lengths is how many it
took off. A caret and a dot are one byte each in UTF-8, so counting
bytes counts marks. The rest, `bare`, goes to the parser that already
read chords, numerals, sets and rows. Before all this, `N.C.` is
recognised and left whole, or its last dot would become a shot.

Writing a marker back is the same in reverse, `format!` with the marks
on both sides:

```rust
pub fn text(&self) -> String {
    let carets = self.push.map_or("", Push::marks);
    let dots = self.stop.map_or("", Stop::marks);
    format!("{carets}{}{dots}", self.harmony)
}
```

`map_or("", Push::marks)` gives `""` for `None` and calls `Push::marks`
on the value inside otherwise. A method can be passed by name like
this, as a function that takes the value as its first argument.

### After the style: rewriting each part

The players do not need to know about marks. Each style writes its
part exactly as before, and then a new step, `Breaks::apply` in
`band/stops.rs`, rewrites it. First `Breaks::of` turns the markers into
stretches of the song:

- a **stop** lasts from its harmony (earlier if it is pushed too) to
  where the next harmony comes in (earlier if that one is pushed);
- a **push** lasts from where it is played to the next harmony. A push
  never reaches back to the harmony before it: if F is written one step
  after C, it cannot come in earlier than C.

Then, for each player:

```rust
for stop in &self.stops {
    clear(&mut notes, stop.start, stop.end);
    let Some(harmony) = harmony_at(stop.written) else { continue };
    let end = match stop.stop {
        Stop::Shot => (stop.start + SHOT_STEPS).min(stop.end),
        Stop::Hold => stop.end,
        Stop::Rest => continue,
    };
    notes.extend(accented(strike(&Played { start: stop.start, end, ..harmony.clone() })));
}
```

`clear` takes out the notes that start in the stretch, and shortens the
ones that started before it and would still be ringing, so that a
shot is followed by real silence. A `continue` inside a `match` arm
skips to the next stop: a rest strikes nothing.

A push clears the beat it is written on and strikes the harmony early.
How long it is held depends on the style: up to where that player's
part goes on. A piano playing on every beat holds the pushed chord to
beat 2; a bass playing half notes holds it to beat 3.

### Each player strikes its own way

What does the piano strike, and what does the bass? `apply` does not
know; it takes a closure that does:

```rust
pub(super) fn apply(
    &self,
    notes: Vec<NoteEvent>,
    harmonies: &[Played],
    strike: impl Fn(&Played) -> Vec<NoteEvent>,
) -> Vec<NoteEvent>
```

The piano passes its held chords, the bass a function that plays one
note, and the guitar and strings check first whether they are playing
in this variation at all (strings that only come in for variation b
should not suddenly hit a shot in variation a):

```rust
breaks.apply(notes, &harmonies, |played| {
    match extras.guitar_in(variation_at(played.start)) {
        Some(_) => guitar::play(GuitarPart::Strum(Comping::Held), Comping::Held,
                                &bars, std::slice::from_ref(played)),
        None => Vec::new(),
    }
})
```

In [A bigger band](bigger-band.md), `strummed` took a **function
pointer**, `fn(&Played) -> Vec<Note>`. Here that would not compile:
this closure uses `extras`, `variation_at` and `bars`, variables of
`write_band` around it. A closure that **captures** its surroundings
carries them with it, so it is not a plain function, and its type has
no name you could write. `impl Fn(&Played) -> Vec<NoteEvent>` says
"any type that can be called like this", and the compiler writes a
version of `apply` for each closure passed to it.

`std::slice::from_ref(played)` turns one `&Played` into a slice of one,
`&[Played]`, without copying it, because `guitar::play` takes a slice.

Why does the bass not use its own held style, `Bass::Held`? Because
that one strikes again on every bar line, and a hold over two bars
would be struck twice. The bass gets a function of its own,
`bass_struck`, that plays one note for the whole stretch.

### The drums

The drums are different: their notes come in pairs, `(channel, note)`,
one channel per drum. `apply_to_drums` takes the groove out of each
stretch and adds the hits, on the kick's and the crash's channels:

```rust
notes.retain(|(_, note)| !(stop.start..stop.end).contains(&note.start));
```

`(a..b).contains(&x)` reads better than `a <= x && x < b`, and for a
push, `(start..=written)` with `..=` includes the step it is written
on. The kick and the crash must have channels before they can play: a
style without a crash gets one, only when the sheet has a mark,
through

```rust
.chain((!breaks.is_empty()).then_some([Drum::Kick, Drum::Crash]).into_iter().flatten())
```

`then_some` turns a `bool` into an `Option`: `Some([...])` when true,
`None` when false. `into_iter().flatten()` then gives the two drums or
nothing, so the list of drums the style needs grows only when needed.

### The band plays again when a mark changes

Nothing had to be written for that. The band remembers a fingerprint
of everything it played from, including the chord markers, made from
their `Debug` text. A marker's `Debug` text now includes its marks, so
changing a shot into a hold changes the fingerprint, and the band plays
again. Parts you edited by hand stay frozen until **Play it again**, as
always.

## Exercises

1. **Stop-time.** Type a twelve-bar blues in F with shots on the first
   four bars:
   `| F7. | F7. | F7. | F7. | Bb7 | Bb7 | F7 | F7 | C7 | Bb7 | F7 | C7 |`
   and pick the **Shuffle** style. Play along in the gaps with the
   F blues scale (F A♭ B♭ B C E♭). What changes in the feel when the
   band comes back in bar 5?
2. **Pushes.** Type `| C | Am | F | G |` in **Pop beat** and listen.
   Then push every chord after the first: `| C | ^Am | ^F | ^G |`.
   Clap the piano's rhythm both ways. Which beat disappears?
3. In the piano roll, compare the piano and the bass in exercise 2.
   How long is each pushed chord in the piano, and each pushed note in
   the bass? Why can they differ?
4. Type `C..` in bar 1, nothing in bar 2 and `G7` in bar 3, so the
   hold lasts two bars. Look at the bass in the piano roll: one note.
   Now replace `bass_struck` in `write_band` with a closure that calls
   `Bass::Held.play_in_bars`. What does the bass play now, and which
   test catches it?
5. (Rust) Make a shot a quarter note long. Which constant do you
   change, and which tests fail? Fix them.
6. (Rust) Add `^^^C`, a push of a quarter note. Add a variant to
   `Push` and let the compiler show you every `match` that must learn
   it. Which other places need it (hint: `read_marks` and the menu)?
7. (Rust, harder) In variation b the drums play a fill in the bar
   before the change. Should a shot or a rest in that bar keep the
   fill? Find where `drum_part` adds the fill and where
   `apply_to_drums` takes the groove out, decide, and write a test for
   what you chose.
