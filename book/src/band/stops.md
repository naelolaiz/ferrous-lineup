# Shots, holds, rests and pushes

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
rather than looped. The chord sheet has marks for them, and the demo
songs' charts use the same marks.

![The chord sheet with C, ^F, G7., Am.. in the first row and F G7..., ^^C, Dm7, G7 in the second. Under each harmony, its numeral and its mark: "IV · push", "V7 · shot", "vi · hold", "V7 · rest". Bar 3 is right-clicked, and the menu "Band on G7" is open: "As the style goes", "Shot (.)" (chosen), "Hold (..)", "Rest (...)", then "On the beat" (chosen), "Push (^)" and "Push a sixteenth (^^)"](../images/step-9-stops-menu.png)

## Trying it

Press **Ctrl+N** for a new song with a band, and type these bars on
the chord sheet, one per bar:

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
back, and the marks are copied with the bars, as text too (see [Chord
sheets as text](chord-sheet.md#chord-sheets-as-text)):

```text
| C. | ^F | G7.. Am... | ^^C |
```

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
bar 3 above works: the silence is part of the music, and the harmony
is still G7 for whoever plays over it.

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
name. Here the whole band rests, drums too; write the drums' break
yourself in the piano roll, or play it with **You play** (see
[Practice tools](practice.md)).

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

A harmony on the sheet is a `ChordMarker`, with two optional fields,
`push` and `stop`, `None` when it is not marked (see [How the sheet is
stored](chord-sheet.md#how-the-sheet-is-stored)). Their types
are two small enums:

```rust
pub enum Stop {
    /// "C.": the whole band strikes the chord once, short, with a kick
    /// and a crash cymbal, and then is silent.
    Shot,
    /// "C..": the band strikes the chord and holds it.
    Hold,
    /// "C...": the band is silent. The harmony goes on, for whoever
    /// plays over it.
    Rest,
}
```

```rust
pub enum Push {
    /// "^C": an eighth note early.
    Eighth,
    /// "^^C": a sixteenth note early.
    Sixteenth,
}
```

Each knows how to write itself (`marks`: `"."`, `".."`, `"..."`,
`"^"`, `"^^"`), how many steps early a push is (`steps`: 2 or 1), and a
sentence for the hints (`describe`). The two fields are left out of the
file when they are `None`, so a song without marks is written as it
would be without them. The file's format only becomes 18 when some
marker has a mark, checked with `any`:

```rust
let has_breaks = playlist
    .chord_markers()
    .iter()
    .any(|marker| marker.push.is_some() || marker.stop.is_some());
```

`ChordMarker::pushed` and `ChordMarker::stopped` take a marker by value
and give it back changed, so a marked marker reads like a sentence:
`ChordMarker::new(16, f).pushed(Some(Push::Eighth))`.

### Reading the marks

Typing `^^Am7/G..` must give a push, the harmony `Am7/G` and a stop.
`read_marks` peels the marks off both ends and counts them:

```rust
let name = text.trim_start_matches('^');
let push = match text.len() - name.len() {
    0 => None,
    1 => Some(Push::Eighth),
    2 => Some(Push::Sixteenth),
    _ => {
        return Err(
            tr("a push is written ^ (an eighth early) or ^^ (a sixteenth).").to_string(),
        );
    }
};
if is_no_chord(name) {
    return Ok((push, name, None));
}
let bare = name.trim_end_matches('.');
let stop = match name.len() - bare.len() {
    0 => None,
    1 => Some(Stop::Shot),
    2 => Some(Stop::Hold),
    3 => Some(Stop::Rest),
    _ => {
        return Err(tr(
            "after a harmony, . is a shot, .. a hold and ... a rest: no more dots.",
        )
        .to_string());
    }
};
Ok((push, bare, stop))
```

`trim_start_matches` returns a **slice** of the same string without the
carets, not a copy, so the difference of the two lengths is how many it
took off. A caret and a dot are one byte each in UTF-8, so counting
bytes counts marks. A no-chord (`N.C.`, `NC`) is left whole, or its
last dot would become a shot. The rest, `bare`, goes to the parser that
reads chords, numerals, sets and rows.

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
part as it always does, and then a last step, `Breaks::apply` in
`band/stops.rs`, rewrites it. First `Breaks::of` turns the markers into
stretches of the song:

- a **stop** lasts from its harmony (earlier if it is pushed too) to
  where the next harmony comes in (earlier if that one is pushed);
- a **push** lasts from where it is played to the next harmony. A push
  never reaches back to the harmony before it: if F is written one step
  after C, it cannot come in earlier than C.

Then, for each player, the stops:

```rust
for stop in &self.stops {
    clear(&mut notes, stop.start, stop.end);
    let Some(harmony) = harmony_at(stop.written) else {
        continue;
    };
    match stop.stop {
        Stop::Shot => notes.extend(accented(strike(&Played {
            start: stop.start,
            end: (stop.start + SHOT_STEPS).min(stop.end),
            ..harmony.clone()
        }))),
        // A row form is played as trichords one after another:
        // each is struck as it comes, and held.
        Stop::Hold => {
            // ...
        }
        Stop::Rest => {}
    }
}
```

`clear` takes out the notes that start in the stretch, and shortens the
ones that started before it and would still be ringing, so that a
shot is followed by real silence. `accented` raises every note to at
least velocity 100, a little harder than the style. `SHOT_STEPS` is 2,
an eighth note. A rest strikes nothing: its arm is empty.

A push clears the beat it is written on and strikes the harmony early.
How long it is held depends on the player: up to where that player's
part goes on. In the song above, the Pop beat piano plays eighth notes,
so the pushed F is held from the "and" of beat 4 to the "and" of
beat 1, a quarter note; a bass playing half notes would hold it to
beat 3.

### Each player strikes its own way

What does the piano strike, and what does the bass? `apply` does not
know; it takes a closure that does:

```rust
pub(crate) fn apply(
    &self,
    mut notes: Vec<NoteEvent>,
    harmonies: &[Played],
    strike: impl Fn(&Played) -> Vec<NoteEvent>,
) -> Vec<NoteEvent> {
```

The piano passes its held chords, the bass a function that plays one
note, and the guitar and strings check first whether they are playing
in this variation at all (strings that only come in for variation b
should not suddenly hit a shot in variation a):

```rust
score.breaks.apply(notes, &score.harmonies, |played| {
    match extras.guitar_in(variation_at(played.start)) {
        Some(_) => guitar::play(
            GuitarPart::Strum(Comping::Held),
            Comping::Held,
            &score.bars,
            std::slice::from_ref(played),
        ),
        None => Vec::new(),
    }
})
```

This closure uses `extras`, `variation_at` and `score`, variables of
the function around it. A closure that **captures** its surroundings
carries them with it, so it is not a plain function pointer like the
`fn(&Played) -> Vec<Note>` that `strummed` takes (see [Enums that carry
data, and a function as a
parameter](band.md#enums-that-carry-data-and-a-function-as-a-parameter)),
and its type has no name you could write. `impl Fn(&Played) ->
Vec<NoteEvent>` says "any type that can be called like this", and the
compiler writes a version of `apply` for each closure passed to it.
A plain function fits too: the bass passes `bass_struck` by name.

`std::slice::from_ref(played)` turns one `&Played` into a slice of one,
`&[Played]`, without copying it, because `guitar::play` takes a slice.

Why does the bass not use its own held style, `Bass::Held`? Because
that one strikes again on every bar line, and a hold over two bars
would be struck twice. `bass_struck` plays one note for the whole
stretch.

### The drums

The drums are different: their notes come in pairs, `(channel, note)`,
one channel per drum. `apply_to_drums` takes the groove out of each
stretch and adds the hits, on the kick's and the crash's channels:

```rust
notes.retain(|(_, note)| !(stop.start..stop.end).contains(&note.start));
```

`(a..b).contains(&x)` reads better than `a <= x && x < b`, and for a
push, `(push.start..=push.written)` with `..=` includes the step it is
written on. The kick and the crash must have channels before they can
play: a style without a crash gets one, only when the sheet has a mark,
through

```rust
.chain(
    (!breaks.is_empty())
        .then_some([Drum::Kick, Drum::Crash])
        .into_iter()
        .flatten(),
)
```

`then_some` turns a `bool` into an `Option`: `Some([...])` when true,
`None` when false. `into_iter().flatten()` then gives the two drums or
nothing, so the list of drums the style needs grows only when needed.

### The band plays again when a mark changes

No code is needed for that. The band remembers a fingerprint of
everything it plays from, including the chord markers, made from their
`Debug` text (see [A part you edit stays
yours](band.md#a-part-you-edit-stays-yours)). A marker's
`Debug` text includes its marks, so changing a shot into a hold
changes the fingerprint, and the band plays again. Parts you edited by
hand stay frozen until **Play it again**, as always.

## Testing it

The tests in `band/stops.rs` work on `Breaks` alone, with a few
`Played` harmonies made by hand:
`marks_become_stretches_up_to_the_next_harmony`,
`a_push_never_reaches_back_past_the_harmony_before`,
`a_shot_strikes_once_and_the_band_stops`,
`a_hold_lasts_until_the_next_harmony_and_a_rest_is_silent`,
`a_hold_over_a_row_form_holds_each_trichord_as_it_comes`,
`a_push_comes_in_early_and_is_held_over_the_beat` and
`the_drums_stop_and_hit_with_the_band`. In `chord_sheet.rs`,
`marks_tell_the_band_where_to_stop_and_push` reads every mark, on
chords, numerals and micromodes, `N.C.` with and without a push, and
the mistakes (`C....`, `^^^C`, a lone `^`);
`marks_are_written_back_as_typed` writes them back.
`the_band_stops_and_pushes_where_the_sheet_says` in `band.rs` plays a
song with marks and checks every player, and
`shots_holds_rests_and_pushes_need_format_eighteen` in `file.rs` checks
that only the marked harmonies get `push:` and `stop:` in the file.

To hear it without listening, save the song above and read its
[playback trace](../sound/trace.md): the piano's F starts in bar 1
on beat 4, two steps into the beat; in bar 3 the piano strikes G7 once,
an eighth long; in bar 4 its Am lasts the whole bar; and the pushed C
starts in bar 5 on beat 4, three steps in.
